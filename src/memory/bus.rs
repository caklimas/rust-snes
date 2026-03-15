use std::ops::RangeInclusive;

use crate::{
    apu::Apu,
    input_output::InputOutput,
    memory::{
        addresses::{
            APU_REGISTERS_RANGE, CPU_IO_RANGE, DMA_REGISTERS_RANGE, DMA_REGISTERS_START, HDMAEN,
            HVBJOY, MDMAEN, MEMSEL, NMI_STATUS_REGISTER, NMITIMEN, PPU_REGISTERS_RANGE,
            PPU_REGISTERS_START, UNUSED_IO_GAP_RANGE, UNUSED_UPPER_GAP_RANGE, WMADDH, WMADDL,
            WMADDM, WMDATA, WRAM_MIRROR_OFFSET_END, WRAM_MIRROR_OFFSET_START, WRAM_RANGE,
            WRAM_START,
        },
        cartridge::Cartridge,
        dma_channel::DmaChannel,
        hvbjoy::Hvbjoy,
        interrupt_enable::InterruptEnable,
        memory_bus::MemoryBus,
        memory_region::MemoryRegion,
        memory_select::MemorySelect,
        nmi_status::NmiStatus,
        wram_access_address::WramAccessAddress,
    },
    ppu::Ppu,
};

const SYSTEM_MIRROR_BANK_RANGE: RangeInclusive<u8> = 0x80..=0xBF;
const SYSTEM_MIRROR_MASK: u32 = 0x7FFFFF;
const WRAM_ACCESS_MASK: u32 = 0x1FFFF;

pub struct Bus {
    pub hvbjoy: Hvbjoy,
    pub input_output: InputOutput,
    pub interrupt_enable: InterruptEnable,
    pub memory_select: MemorySelect,
    pub nmi_status: NmiStatus,
    pub ppu: Ppu,

    apu: Apu,
    cartridge: Cartridge,
    dma_channels: [DmaChannel; 8],
    hdmaen: u8,
    wram: MemoryRegion,
    wram_access_address: WramAccessAddress,
}

impl Bus {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            apu: Default::default(),
            cartridge: Cartridge::new(data),
            dma_channels: [Default::default(); 8],
            hdmaen: 0,
            hvbjoy: Default::default(),
            input_output: Default::default(),
            interrupt_enable: Default::default(),
            memory_select: Default::default(),
            nmi_status: Default::default(),
            ppu: Default::default(),
            wram: MemoryRegion::new(vec![0; 131072], WRAM_START),
            wram_access_address: WramAccessAddress::default(),
        }
    }

    pub fn frame_buffer(&self) -> &[u16] {
        self.ppu.frame_buffer()
    }

    pub fn read(&mut self, address: u32) -> u8 {
        let normalized_address = Self::normalize_address(address);
        match normalized_address {
            addr if UNUSED_IO_GAP_RANGE.contains(&addr) => 0,
            WMDATA => {
                let value = self.wram.read(&self.get_wram_access_address());
                self.increment_wram_access_address();
                value
            }
            addr if UNUSED_UPPER_GAP_RANGE.contains(&addr) => 0,
            NMI_STATUS_REGISTER => {
                let value = self.nmi_status.0;
                self.nmi_status.set_nmi_flag(false);
                value
            }
            addr if WRAM_RANGE.contains(&addr) => self.wram.read(&addr),
            addr if Self::is_wram_mirror(addr) => {
                let wram_addr = WRAM_START + (addr & 0xFFFF);
                self.wram.read(&wram_addr)
            }
            addr if PPU_REGISTERS_RANGE.contains(&addr) => self.ppu.read(addr),
            addr if APU_REGISTERS_RANGE.contains(&addr) => self.apu.read(addr),
            NMITIMEN => 0,
            HVBJOY => (self.hvbjoy.vblank() as u8) << 7,
            addr if CPU_IO_RANGE.contains(&addr) => self.input_output.read(addr),
            _ => self.cartridge.read(address),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        let normalized_address = Self::normalize_address(address);
        match normalized_address {
            addr if UNUSED_IO_GAP_RANGE.contains(&addr) => {}
            WMDATA => {
                self.wram.write(&self.get_wram_access_address(), value);
                self.increment_wram_access_address();
            }
            WMADDL => self.wram_access_address.set_wmaddl(value as u32),
            WMADDM => self.wram_access_address.set_wmaddm(value as u32),
            WMADDH => self.wram_access_address.set_wmaddh(value & 0b1 == 1),
            addr if UNUSED_UPPER_GAP_RANGE.contains(&addr) => {}
            MDMAEN => {
                for i in 0u8..=7 {
                    let channel = (value >> i) & 0x1 == 1;
                    let mut incremented = false;
                    if channel {
                        let channel = self.dma_channels[i as usize];
                        let source = (channel.a1b as u32) << 16 | (channel.a1t as u32);
                        let destination = PPU_REGISTERS_START | (channel.bbad as u32);
                        let dmap_mode = channel.dmap.0 & 0x07;
                        let transfer_direction = channel.dmap.0 >> 7;
                        let das = if channel.das == 0 {
                            65536
                        } else {
                            channel.das as u32
                        };

                        for i in 0u32..das {
                            let destination = if dmap_mode == 1 && incremented {
                                destination + 1
                            } else {
                                destination
                            };

                            incremented = !incremented;

                            if transfer_direction == 0 {
                                let source_address = if channel.dmap.fixed_transfer() {
                                    source
                                } else {
                                    source + i
                                };
                                let value = self.read(source_address);
                                self.write(destination, value);
                            } else {
                                let value = self.read(destination);
                                self.write(source + i, value);
                            }
                        }
                    }
                }
            }
            HDMAEN => self.hdmaen = value,
            addr if DMA_REGISTERS_RANGE.contains(&addr) => {
                let offset = addr - DMA_REGISTERS_START;
                let upper_nibble = offset >> 4;
                self.dma_channels[upper_nibble as usize].set_register((offset & 0xF) as u8, value);
            }
            addr if WRAM_RANGE.contains(&addr) => self.wram.write(&addr, value),
            addr if Self::is_wram_mirror(addr) => {
                let wram_addr = WRAM_START + (addr & 0xFFFF);
                self.wram.write(&wram_addr, value)
            }
            addr if PPU_REGISTERS_RANGE.contains(&addr) => self.ppu.write(addr, value),
            addr if APU_REGISTERS_RANGE.contains(&addr) => self.apu.write(addr, value),
            NMITIMEN => self.interrupt_enable.0 = value,
            MEMSEL => self.memory_select.0 = value,
            addr if CPU_IO_RANGE.contains(&addr) => self.input_output.write(addr, value),
            _ => self.cartridge.write(address, value),
        }
    }

    pub fn init_hdma(&mut self) {
        for i in 0u8..8 {
            let channel_enabled = (self.hdmaen >> i) & 1 == 1;
            if channel_enabled {
                let a1b = self.dma_channels[i as usize].a1b;
                let a1t = self.dma_channels[i as usize].a1t;
                let address = ((a1b as u32) << 16) | (a1t as u32);
                let line_counter = self.read(address);
                let channel = &mut self.dma_channels[i as usize];

                channel.hdma_table_ptr = a1t + 1;
                channel.hdma_line_counter = line_counter;
                channel.hdma_do_transfer = true;
            }
        }
    }

    pub fn run_hdma_scanline(&mut self) {
        for i in 0u8..8 {
            let channel_enabled = (self.hdmaen >> i) & 1 == 1;
            if channel_enabled {
                let a1b = self.dma_channels[i as usize].a1b;
                let hdma_table_ptr = self.dma_channels[i as usize].hdma_table_ptr;
                let bbad = self.dma_channels[i as usize].bbad;
                let dmap = self.dma_channels[i as usize].dmap;

                if self.dma_channels[i as usize].hdma_do_transfer {
                    let mut bytes_consumed = 0;
                    let address = ((a1b as u32) << 16) | (hdma_table_ptr as u32);
                    let data = self.read(address);

                    match dmap.transfer_mode() {
                        0 => {
                            self.write(PPU_REGISTERS_START | (bbad as u32), data);
                            bytes_consumed = 1;
                        }
                        1 => {
                            self.write(PPU_REGISTERS_START | (bbad as u32), data);

                            let data = self.read(address + 1);
                            self.write(PPU_REGISTERS_START | ((bbad + 1) as u32), data);
                            bytes_consumed = 2;
                        }
                        _ => {}
                    }

                    self.dma_channels[i as usize].hdma_table_ptr = self.dma_channels[i as usize]
                        .hdma_table_ptr
                        .wrapping_add(bytes_consumed);
                }

                self.dma_channels[i as usize].hdma_line_counter = self.dma_channels[i as usize]
                    .hdma_line_counter
                    .wrapping_sub(1);

                if self.dma_channels[i as usize].hdma_line_counter & 0x7F == 0 {
                    let value = self.read(
                        ((a1b as u32) << 16)
                            | (self.dma_channels[i as usize].hdma_table_ptr as u32),
                    );

                    self.dma_channels[i as usize].hdma_table_ptr =
                        self.dma_channels[i as usize].hdma_table_ptr.wrapping_add(1);

                    if value == 0 {
                        self.hdmaen &= !(1 << i);
                    } else {
                        self.dma_channels[i as usize].hdma_line_counter = value;
                        self.dma_channels[i as usize].hdma_do_transfer = true;
                    }
                }
            }
        }
    }

    /// Returns the number of master clocks per CPU cycle for the given address.
    /// 6 = fast (3.58 MHz), 8 = slow (2.68 MHz), 12 = extra slow (1.78 MHz)
    pub fn master_clocks_for_address(&self, address: u32) -> u32 {
        let bank = (address >> 16) as u8;
        let offset = (address & 0xFFFF) as u16;

        match bank {
            // Banks $00-$3F and mirrors $80-$BF
            0x00..=0x3F | 0x80..=0xBF => match offset {
                0x0000..=0x1FFF => 8,  // WRAM mirror
                0x2100..=0x21FF => 6,  // PPU/APU registers (B-Bus)
                0x4000..=0x41FF => 12, // Joypad serial
                0x4200..=0x5FFF => 6,  // CPU I/O registers
                0x8000..=0xFFFF => {
                    // ROM area: banks $80-$BF are WS2 (fast if MEMSEL set)
                    if bank >= 0x80 && self.memory_select.access_speed() {
                        6
                    } else {
                        8
                    }
                }
                _ => 8, // Everything else (expansion, SRAM, etc.)
            },
            // Banks $40-$7D: HiROM / expansion — always slow
            0x40..=0x7D => 8,
            // Banks $7E-$7F: WRAM — always slow
            0x7E..=0x7F => 8,
            // Banks $C0-$FF: WS2 HiROM
            0xC0..=0xFF => {
                if self.memory_select.access_speed() {
                    6
                } else {
                    8
                }
            }
        }
    }

    fn is_wram_mirror(address: u32) -> bool {
        let bank = (address >> 16) as u8;
        let offset = (address & 0xFFFF) as u16;
        matches!(bank, 0x00..=0x3F)
            && (WRAM_MIRROR_OFFSET_START..=WRAM_MIRROR_OFFSET_END).contains(&offset)
    }

    fn normalize_address(address: u32) -> u32 {
        if Self::is_mirror_bank(address) {
            address & SYSTEM_MIRROR_MASK
        } else {
            address
        }
    }

    fn is_mirror_bank(address: u32) -> bool {
        let bank_byte = (address >> 16) as u8;
        SYSTEM_MIRROR_BANK_RANGE.contains(&bank_byte)
    }

    fn get_wram_access_address(&self) -> u32 {
        self.wram_access_address.0 + WRAM_START
    }

    fn increment_wram_access_address(&mut self) {
        self.wram_access_address.0 = (self.wram_access_address.0 + 1) & WRAM_ACCESS_MASK
    }
}

impl MemoryBus for Bus {
    fn read(&mut self, address: u32) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u32, value: u8) {
        self.write(address, value)
    }
}
