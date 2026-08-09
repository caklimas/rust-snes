use std::fmt;

use crate::{
    memory::addresses::{
        BG1HOFS, BG1SC, BG1VOFS, BG2HOFS, BG2SC, BG2VOFS, BG3HOFS, BG3SC, BG3VOFS, BG4HOFS, BG4SC,
        BG4VOFS, BG12NBA, BG34NBA, BGMODE, CGADD, CGADSUB, CGDATA, CGDATAREAD, CGWSEL, COLDATA,
        INIDISP, MOSAIC, OAMADD_HI, OAMADD_LO, OAMDATA, OAMDATAREAD, OBSEL, SETINI, TM, TMW, TS,
        TSW, VMADDH, VMADDL, VMAIN, VMDATAH, VMDATAL, W12SEL, W34SEL, WBGLOG, WH0, WH1, WH2, WH3,
        WOBJLOG, WOBJSEL,
    },
    ppu::{
        bg_horizontal_offset::BgHorizontalOffset,
        bg_mode::BgMode,
        bg_sample::BgSample,
        bg_sample_params::{BgLayerConfig, BgSampleParams},
        bg_tilemap::BgTilemap,
        bg_vertical_offset::BgVerticalOffset,
        bpp_settings::BppSettings,
        cgadsub::Cgadsub,
        cgram::Cgram,
        cgwsel::Cgwsel,
        coldata::Coldata,
        display::Display,
        frame_buffer::FrameBuffer,
        mosaic::Mosaic,
        mosaic_config::MosaicConfig,
        oam::Oam,
        obj_sample::ObjSample,
        obsel::Obsel,
        palette_base::PaletteBase,
        priority_resolver::PriorityResolver,
        rgb::Rgb,
        screen_designation::ScreenDesignation,
        screen_setting::ScreenSetting,
        tile_graphic_base_address::TileGraphicBaseAddress,
        tilemap_entry::TilemapEntry,
        vram::Vram,
        wbglog::Wbglog,
        window_bounds::WindowBounds,
        window_condition::WindowCondition,
        window_layer_disable::WindowLayerDisable,
        window_mask_settings::WindowMaskSettings,
        winning_layer::Layer,
        wobjlog::Wobjlog,
    },
};

pub mod bg_horizontal_offset;
pub mod bg_mode;
pub mod bg_sample;
pub mod bg_sample_params;
pub mod bg_tilemap;
pub mod bg_vertical_offset;
pub mod bpp_settings;
pub mod cgadsub;
pub mod cgram;
pub mod cgwsel;
pub mod coldata;
pub mod display;
pub mod frame_buffer;
pub mod high_table_sprite;
pub mod low_table_sprite;
pub mod mosaic;
pub mod mosaic_config;
pub mod oam;
pub mod obj_sample;
pub mod obsel;
pub mod packed_attributes;
pub mod palette_base;
pub mod priority_resolver;
pub mod rgb;
pub mod screen_designation;
pub mod screen_setting;
pub mod tile_graphic_base_address;
pub mod tilemap_entry;
pub mod vmain;
pub mod vram;
pub mod wbglog;
pub mod window_bounds;
pub mod window_condition;
pub mod window_layer_disable;
pub mod window_mask_settings;
pub mod winning_layer;
pub mod wobjlog;

pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 224;

#[derive(Default)]
pub struct Ppu {
    pub display: Display,
    pub oam: Oam,
    pub vram: Vram,
    bg1: BgTilemap,
    bg2: BgTilemap,
    bg3: BgTilemap,
    bg4: BgTilemap,
    bg_horizontal_offset: BgHorizontalOffset,
    bg_vertical_offset: BgVerticalOffset,
    bg_mode: BgMode,
    bg_old: u8,
    cgadsub: Cgadsub,
    cgram: Cgram,
    cgwsel: Cgwsel,
    coldata: Coldata,
    fixed_color: Rgb,
    frame_buffer: FrameBuffer,
    main_screen_designation: ScreenDesignation,
    mosaic: Mosaic,
    obsel: Obsel,
    screen_setting: ScreenSetting,
    sub_screen_designation: ScreenDesignation,
    tile_graphic12: TileGraphicBaseAddress,
    tile_graphic34: TileGraphicBaseAddress,
    tmw: WindowLayerDisable,
    tsw: WindowLayerDisable,
    w12sel: WindowMaskSettings,
    w34sel: WindowMaskSettings,
    wbglog: Wbglog,
    window_bounds_1: WindowBounds,
    window_bounds_2: WindowBounds,
    wobjlog: Wobjlog,
    wobjsel: WindowMaskSettings,
    pub current_scanline: u16,
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ppu")
            .field("mode", &self.bg_mode.bg_mode())
            .field("forced_blank", &self.display.forced_blank())
            .field("brightness", &self.display.master_brightness())
            .field("scanline", &self.current_scanline)
            .field(
                "bg_hofs",
                &[
                    self.bg_horizontal_offset.bg1_offset,
                    self.bg_horizontal_offset.bg2_offset,
                    self.bg_horizontal_offset.bg3_offset,
                    self.bg_horizontal_offset.bg4_offset,
                ],
            )
            .field(
                "bg_vofs",
                &[
                    self.bg_vertical_offset.bg1_offset,
                    self.bg_vertical_offset.bg2_offset,
                    self.bg_vertical_offset.bg3_offset,
                    self.bg_vertical_offset.bg4_offset,
                ],
            )
            .field(
                "tm",
                &format_args!(
                    "BG1={} BG2={} BG3={} BG4={} OBJ={}",
                    self.main_screen_designation.bg1_enable(),
                    self.main_screen_designation.bg2_enable(),
                    self.main_screen_designation.bg3_enable(),
                    self.main_screen_designation.bg4_enable(),
                    self.main_screen_designation.obj_enable(),
                ),
            )
            .field(
                "ts",
                &format_args!(
                    "BG1={} BG2={} BG3={} BG4={} OBJ={}",
                    self.sub_screen_designation.bg1_enable(),
                    self.sub_screen_designation.bg2_enable(),
                    self.sub_screen_designation.bg3_enable(),
                    self.sub_screen_designation.bg4_enable(),
                    self.sub_screen_designation.obj_enable(),
                ),
            )
            .field(
                "window_1",
                &format_args!(
                    "left={} right={}",
                    self.window_bounds_1.left, self.window_bounds_1.right
                ),
            )
            .field(
                "window_2",
                &format_args!(
                    "left={} right={}",
                    self.window_bounds_2.left, self.window_bounds_2.right
                ),
            )
            .field(
                "w12sel",
                &format_args!("0x{:02X}", self.w12sel.0),
            )
            .field(
                "w34sel",
                &format_args!("0x{:02X}", self.w34sel.0),
            )
            .field(
                "wobjsel",
                &format_args!("0x{:02X}", self.wobjsel.0),
            )
            .field(
                "tmw",
                &format_args!(
                    "BG1={} BG2={} BG3={} BG4={} OBJ={}",
                    self.tmw.bg1_disable(),
                    self.tmw.bg2_disable(),
                    self.tmw.bg3_disable(),
                    self.tmw.bg4_disable(),
                    self.tmw.obj_disable(),
                ),
            )
            .field(
                "tsw",
                &format_args!(
                    "BG1={} BG2={} BG3={} BG4={} OBJ={}",
                    self.tsw.bg1_disable(),
                    self.tsw.bg2_disable(),
                    self.tsw.bg3_disable(),
                    self.tsw.bg4_disable(),
                    self.tsw.obj_disable(),
                ),
            )
            .finish()
    }
}

impl Ppu {
    pub fn frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn bg_vertical_offset(&self) -> &BgVerticalOffset {
        &self.bg_vertical_offset
    }

    pub fn bg_horizontal_offset(&self) -> &BgHorizontalOffset {
        &self.bg_horizontal_offset
    }

    pub fn bg_old(&self) -> u8 {
        self.bg_old
    }

    pub fn main_screen_designation(&self) -> &ScreenDesignation {
        &self.main_screen_designation
    }

    pub fn bg_mode_value(&self) -> u8 {
        self.bg_mode.bg_mode()
    }

    pub fn bg1_tilemap_base(&self) -> u16 {
        self.bg1.get_vram_word_address()
    }

    pub fn bg3_tilemap_base(&self) -> u16 {
        self.bg3.get_vram_word_address()
    }

    pub fn render_scanline(&mut self, y: u16) {
        self.current_scanline = y;
        let bpp_settings = BppSettings::new(&self.bg_mode);
        let palette_base = PaletteBase::new(&self.bg_mode);
        let brightness_factor = self.display.master_brightness() as u16 + 1;

        let bg1_layer = BgLayerConfig {
            bg_tilemap: &self.bg1,
            horizontal_offset: self.bg_horizontal_offset.bg1_offset,
            vertical_offset: self.bg_vertical_offset.bg1_offset,
            char_base: self.tile_graphic12.first_vram_word_address(),
            bpp_opt: bpp_settings.bg1,
            palette_base: palette_base.bg1,
            tile_size_16: self.bg_mode.tile_size_1(),
        };
        let bg2_layer = BgLayerConfig {
            bg_tilemap: &self.bg2,
            horizontal_offset: self.bg_horizontal_offset.bg2_offset,
            vertical_offset: self.bg_vertical_offset.bg2_offset,
            char_base: self.tile_graphic12.second_vram_word_address(),
            bpp_opt: bpp_settings.bg2,
            palette_base: palette_base.bg2,
            tile_size_16: self.bg_mode.tile_size_2(),
        };
        let bg3_layer = BgLayerConfig {
            bg_tilemap: &self.bg3,
            horizontal_offset: self.bg_horizontal_offset.bg3_offset,
            vertical_offset: self.bg_vertical_offset.bg3_offset,
            char_base: self.tile_graphic34.first_vram_word_address(),
            bpp_opt: bpp_settings.bg3,
            palette_base: palette_base.bg3,
            tile_size_16: self.bg_mode.tile_size_3(),
        };
        let bg4_layer = BgLayerConfig {
            bg_tilemap: &self.bg4,
            horizontal_offset: self.bg_horizontal_offset.bg4_offset,
            vertical_offset: self.bg_vertical_offset.bg4_offset,
            char_base: self.tile_graphic34.second_vram_word_address(),
            bpp_opt: bpp_settings.bg4,
            palette_base: palette_base.bg4,
            tile_size_16: self.bg_mode.tile_size_4(),
        };

        for x in 0u16..SCREEN_WIDTH {
            let index = (((y - 1) * SCREEN_WIDTH) + x) as usize;
            if self.display.forced_blank() {
                self.frame_buffer.0[index] = 0;
                continue;
            }

            let mosaic_size = self.mosaic.mosaic_size() as u16 + 1;
            let bg1_mosaic = MosaicConfig {
                enabled: self.mosaic.bg1_enable(),
                size: mosaic_size,
            };
            let bg2_mosaic = MosaicConfig {
                enabled: self.mosaic.bg2_enable(),
                size: mosaic_size,
            };
            let bg3_mosaic = MosaicConfig {
                enabled: self.mosaic.bg3_enable(),
                size: mosaic_size,
            };
            let bg4_mosaic = MosaicConfig {
                enabled: self.mosaic.bg4_enable(),
                size: mosaic_size,
            };

            let bg_1_params = BgSampleParams::new(
                self.is_enabled(
                    self.main_screen_designation.bg1_enable(),
                    x as u8,
                    self.w12sel.instance_1_window_1(),
                    self.w12sel.instance_1_window_2(),
                    self.wbglog.bg1_combine_logic(),
                    self.tmw.bg1_disable(),
                ),
                self.is_enabled(
                    self.sub_screen_designation.bg1_enable(),
                    x as u8,
                    self.w12sel.instance_1_window_1(),
                    self.w12sel.instance_1_window_2(),
                    self.wbglog.bg1_combine_logic(),
                    self.tsw.bg1_disable(),
                ),
                x,
                y,
                &bg1_layer,
                &bg1_mosaic,
            );

            let bg_2_params = BgSampleParams::new(
                self.is_enabled(
                    self.main_screen_designation.bg2_enable(),
                    x as u8,
                    self.w12sel.instance_2_window_1(),
                    self.w12sel.instance_2_window_2(),
                    self.wbglog.bg2_combine_logic(),
                    self.tmw.bg2_disable(),
                ),
                self.is_enabled(
                    self.sub_screen_designation.bg2_enable(),
                    x as u8,
                    self.w12sel.instance_2_window_1(),
                    self.w12sel.instance_2_window_2(),
                    self.wbglog.bg2_combine_logic(),
                    self.tsw.bg2_disable(),
                ),
                x,
                y,
                &bg2_layer,
                &bg2_mosaic,
            );

            let bg_3_params = BgSampleParams::new(
                self.is_enabled(
                    self.main_screen_designation.bg3_enable(),
                    x as u8,
                    self.w34sel.instance_1_window_1(),
                    self.w34sel.instance_1_window_2(),
                    self.wbglog.bg3_combine_logic(),
                    self.tmw.bg3_disable(),
                ),
                self.is_enabled(
                    self.sub_screen_designation.bg3_enable(),
                    x as u8,
                    self.w34sel.instance_1_window_1(),
                    self.w34sel.instance_1_window_2(),
                    self.wbglog.bg3_combine_logic(),
                    self.tsw.bg3_disable(),
                ),
                x,
                y,
                &bg3_layer,
                &bg3_mosaic,
            );

            let bg_4_params = BgSampleParams::new(
                self.is_enabled(
                    self.main_screen_designation.bg4_enable(),
                    x as u8,
                    self.w34sel.instance_2_window_1(),
                    self.w34sel.instance_2_window_2(),
                    self.wbglog.bg4_combine_logic(),
                    self.tmw.bg4_disable(),
                ),
                self.is_enabled(
                    self.sub_screen_designation.bg4_enable(),
                    x as u8,
                    self.w34sel.instance_2_window_1(),
                    self.w34sel.instance_2_window_2(),
                    self.wbglog.bg4_combine_logic(),
                    self.tsw.bg4_disable(),
                ),
                x,
                y,
                &bg4_layer,
                &bg4_mosaic,
            );

            let bg1_sample_main = self.bg_sample(&bg_1_params, true);
            let bg2_sample_main = self.bg_sample(&bg_2_params, true);
            let bg3_sample_main = self.bg_sample(&bg_3_params, true);
            let bg4_sample_main = self.bg_sample(&bg_4_params, true);
            let obj_sample_main = self.obj_sample(
                x,
                y,
                self.is_enabled(
                    self.main_screen_designation.obj_enable(),
                    x as u8,
                    self.wobjsel.instance_1_window_1(),
                    self.wobjsel.instance_1_window_2(),
                    self.wobjlog.obj_combine_logic(),
                    self.tmw.obj_disable(),
                ),
            );

            let priority_resolver_main = PriorityResolver::new(
                bg1_sample_main,
                bg2_sample_main,
                bg3_sample_main,
                bg4_sample_main,
                obj_sample_main,
            );
            let sample_main = priority_resolver_main.get_sample(self.bg_mode);

            let bg1_sample_sub = self.bg_sample(&bg_1_params, false);
            let bg2_sample_sub = self.bg_sample(&bg_2_params, false);
            let bg3_sample_sub = self.bg_sample(&bg_3_params, false);
            let bg4_sample_sub = self.bg_sample(&bg_4_params, false);
            let obj_sample_sub = self.obj_sample(
                x,
                y,
                self.is_enabled(
                    self.sub_screen_designation.obj_enable(),
                    x as u8,
                    self.wobjsel.instance_1_window_1(),
                    self.wobjsel.instance_1_window_2(),
                    self.wobjlog.obj_combine_logic(),
                    self.tsw.obj_disable(),
                ),
            );

            let priority_resolver_sub = PriorityResolver::new(
                bg1_sample_sub,
                bg2_sample_sub,
                bg3_sample_sub,
                bg4_sample_sub,
                obj_sample_sub,
            );
            let sample_sub = priority_resolver_sub.get_sample(self.bg_mode);

            let mut color = Rgb(match sample_main {
                Some(wl) => self.cgram.read_color(wl.cgram_index as u16),
                None => self.cgram.read_color(0),
            });

            let sub_color = match (self.cgwsel.sub_screen_enable(), sample_sub) {
                (false, _) => self.fixed_color,
                (true, Some(wl)) => Rgb(self.cgram.read_color(wl.cgram_index as u16)),
                (true, None) => self.fixed_color,
            };

            let suppress_div2 = self.cgwsel.sub_screen_enable() && sample_sub.is_none();

            let math_enabled = match &sample_main {
                Some(wl) => match wl.layer {
                    Layer::Bg1 => self.cgadsub.bg1(),
                    Layer::Bg2 => self.cgadsub.bg2(),
                    Layer::Bg3 => self.cgadsub.bg3(),
                    Layer::Bg4 => self.cgadsub.bg4(),
                    Layer::Obj => self.cgadsub.obj(),
                },
                None => self.cgadsub.backdrop(),
            };

            let math_window_active = self.is_layer_active(
                x as u8,
                self.wobjsel.instance_2_window_1(),
                self.wobjsel.instance_2_window_2(),
                self.wobjlog.math_combine_logic(),
            );

            match (
                self.cgwsel.get_color_math_enable(),
                math_window_active,
                math_enabled,
            ) {
                (WindowCondition::Always, _, true) => {
                    self.apply_color_math(&mut color, sub_color, suppress_div2)
                }
                (WindowCondition::MathWindow, true, true) => {
                    self.apply_color_math(&mut color, sub_color, suppress_div2)
                }
                (WindowCondition::NotMathWin, false, true) => {
                    self.apply_color_math(&mut color, sub_color, suppress_div2)
                }
                _ => {}
            }

            color.set_red((color.red() * brightness_factor) / 16);
            color.set_green((color.green() * brightness_factor) / 16);
            color.set_blue((color.blue() * brightness_factor) / 16);

            self.frame_buffer.0[index] = color.0;
        }
    }

    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            OAMADD_LO => 0,
            OAMADD_HI => 0,
            OAMDATA => 0,
            BGMODE => self.bg_mode.0,
            BG1SC => self.bg1.0,
            BG2SC => self.bg2.0,
            BG3SC => self.bg3.0,
            BG4SC => self.bg4.0,
            BG12NBA => self.tile_graphic12.0,
            BG34NBA => self.tile_graphic34.0,
            TM => self.main_screen_designation.0,
            TS => self.sub_screen_designation.0,
            OAMDATAREAD => self.oam.read_oamdata(),
            CGADD => 0,
            CGDATA => 0,
            CGDATAREAD => self.cgram.read_cgdata(),
            _ => {
                eprintln!("Unhandled PPU read: {:#06X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            INIDISP => self.display.0 = value,
            OBSEL => self.obsel.0 = value,
            OAMADD_LO => self.oam.set_oamadd(value, true),
            OAMADD_HI => self.oam.set_oamadd(value, false),
            OAMDATA => self.oam.write_oamdata(value),
            BGMODE => self.bg_mode.0 = value,
            MOSAIC => self.mosaic.0 = value,
            BG1SC => self.bg1.0 = value,
            BG2SC => self.bg2.0 = value,
            BG3SC => self.bg3.0 = value,
            BG4SC => self.bg4.0 = value,
            BG12NBA => self.tile_graphic12.0 = value,
            BG34NBA => self.tile_graphic34.0 = value,
            BG1HOFS => self.set_horizontal_offset(1, value),
            BG1VOFS => self.set_vertical_offset(1, value),
            BG2HOFS => self.set_horizontal_offset(2, value),
            BG2VOFS => self.set_vertical_offset(2, value),
            BG3HOFS => self.set_horizontal_offset(3, value),
            BG3VOFS => self.set_vertical_offset(3, value),
            BG4HOFS => self.set_horizontal_offset(4, value),
            BG4VOFS => self.set_vertical_offset(4, value),
            W12SEL => self.w12sel.0 = value,
            W34SEL => self.w34sel.0 = value,
            WOBJSEL => self.wobjsel.0 = value,
            WBGLOG => self.wbglog.0 = value,
            WOBJLOG => self.wobjlog.0 = value,
            WH0 => self.window_bounds_1.left = value,
            WH1 => self.window_bounds_1.right = value,
            WH2 => self.window_bounds_2.left = value,
            WH3 => self.window_bounds_2.right = value,
            TM => self.main_screen_designation.0 = value,
            TS => self.sub_screen_designation.0 = value,
            TMW => self.tmw.0 = value,
            TSW => self.tsw.0 = value,
            CGWSEL => self.cgwsel.0 = value,
            CGADSUB => self.cgadsub.0 = value,
            COLDATA => {
                self.coldata.0 = value;

                let intensity = self.coldata.intensity() as u16;
                if self.coldata.apply_to_red() {
                    self.fixed_color.set_red(intensity);
                }

                if self.coldata.apply_to_green() {
                    self.fixed_color.set_green(intensity);
                }

                if self.coldata.apply_to_blue() {
                    self.fixed_color.set_blue(intensity);
                }
            }
            SETINI => self.screen_setting.0 = value,
            OAMDATAREAD => {}
            VMAIN => self.vram.vmain.0 = value,
            VMADDL => self.vram.set_address_lo(value),
            VMADDH => self.vram.set_address_hi(value),
            VMDATAL => self.vram.write_data_lo(
                value,
                !self.vram.rendering_active || self.display.forced_blank(),
            ),
            VMDATAH => self.vram.write_data_hi(
                value,
                !self.vram.rendering_active || self.display.forced_blank(),
            ),
            CGADD => self.cgram.write_cgadd(value),
            CGDATA => self.cgram.write_cgdata(value),
            CGDATAREAD => {}
            _ => eprintln!("Unhandled PPU write: {:#06X} = {:#04X}", address, value),
        }
    }

    fn set_horizontal_offset(&mut self, number: u8, value: u8) {
        let offset = ((value as u16) << 8) | ((self.bg_old as u16) & !7);
        match number {
            1 => {
                self.bg_horizontal_offset.bg1_offset =
                    offset | ((self.bg_horizontal_offset.bg1_offset >> 8) & 7)
            }
            2 => {
                self.bg_horizontal_offset.bg2_offset =
                    offset | ((self.bg_horizontal_offset.bg2_offset >> 8) & 7)
            }
            3 => {
                self.bg_horizontal_offset.bg3_offset =
                    offset | ((self.bg_horizontal_offset.bg3_offset >> 8) & 7)
            }
            4 => {
                self.bg_horizontal_offset.bg4_offset =
                    offset | ((self.bg_horizontal_offset.bg4_offset >> 8) & 7)
            }
            _ => unimplemented!(),
        }

        self.bg_old = value;
    }

    fn set_vertical_offset(&mut self, number: u8, value: u8) {
        let offset = ((value as u16) << 8) | (self.bg_old as u16);
        match number {
            1 => self.bg_vertical_offset.bg1_offset = offset,
            2 => self.bg_vertical_offset.bg2_offset = offset,
            3 => self.bg_vertical_offset.bg3_offset = offset,
            4 => self.bg_vertical_offset.bg4_offset = offset,
            _ => unimplemented!(),
        }

        self.bg_old = value;
    }

    fn bg_sample(&self, params: &BgSampleParams, main_screen: bool) -> Option<BgSample> {
        let enabled = if main_screen {
            params.main_enabled
        } else {
            params.sub_enabled
        };
        if !enabled {
            return None;
        }

        let bpp = params.bpp_opt?;
        let x_offset = params.bg_horizontal_offset.wrapping_add(params.x);
        let y_offset = params.bg_vertical_offset.wrapping_add(params.y);

        let denominator = if params.tile_size_16 { 16 } else { 8 };
        let tilemap_width = params.bg_tilemap.get_tilemap_width();
        let tilemap_height = params.bg_tilemap.get_tilemap_height();
        let tile_x = (x_offset / denominator) % tilemap_width;
        let tile_y = (y_offset / denominator) % tilemap_height;

        let screen_offset = self.get_screen_offset(params.bg_tilemap, tile_x, tile_y);
        let local_x = tile_x % 32;
        let local_y = tile_y % 32;
        let entry_address =
            params.bg_tilemap.get_vram_word_address() + screen_offset + (local_y * 32) + local_x;

        let tilemap_entry = TilemapEntry(self.vram.read_word(entry_address));
        let tile_number = if params.tile_size_16 {
            let pixel_in_tile_x = x_offset % 16;
            let pixel_in_tile_y = y_offset % 16;

            let mut sub_tile_col = pixel_in_tile_x / 8;
            let mut sub_tile_row = pixel_in_tile_y / 8;

            if tilemap_entry.x_flip() {
                sub_tile_col = 1 - sub_tile_col
            }

            if tilemap_entry.y_flip() {
                sub_tile_row = 1 - sub_tile_row
            }

            (tilemap_entry.tile_number() + sub_tile_col + (sub_tile_row * 16)) & 0x3FF
        } else {
            tilemap_entry.tile_number()
        };

        let tile_base_multiplier = if bpp == 4 {
            16
        } else if bpp == 8 {
            32
        } else {
            8
        };
        let tile_base = params.char_base + tile_number * tile_base_multiplier;

        let pixel_x_within_tile = x_offset % 8;
        let mut pixel_y_within_tile = y_offset % 8;

        if tilemap_entry.y_flip() {
            pixel_y_within_tile = 7 - pixel_y_within_tile;
        }

        let bit = if tilemap_entry.x_flip() {
            pixel_x_within_tile
        } else {
            7 - pixel_x_within_tile
        };
        let (plane_0, plane_1) = self.get_planes(tile_base, pixel_y_within_tile, bit, 0);
        let (plane_2, plane_3) = self.get_planes(tile_base, pixel_y_within_tile, bit, 8);

        let character_data = if bpp == 4 {
            plane_0 | (plane_1 << 1) | (plane_2 << 2) | (plane_3 << 3)
        } else if bpp == 8 {
            let (plane_4, plane_5) = self.get_planes(tile_base, pixel_y_within_tile, bit, 16);
            let (plane_6, plane_7) = self.get_planes(tile_base, pixel_y_within_tile, bit, 24);

            plane_0
                | (plane_1 << 1)
                | (plane_2 << 2)
                | (plane_3 << 3)
                | (plane_4 << 4)
                | (plane_5 << 5)
                | (plane_6 << 6)
                | (plane_7 << 7)
        } else {
            plane_0 | (plane_1 << 1)
        };

        if character_data == 0 {
            None
        } else {
            let cg_ram_index = if bpp == 4 {
                ((params.palette_base as u16)
                    + tilemap_entry.palette_number() * 16
                    + character_data) as u8
            } else if bpp == 8 {
                character_data as u8
            } else {
                ((params.palette_base as u16) + tilemap_entry.palette_number() * 4 + character_data)
                    as u8
            };

            Some(BgSample {
                cg_ram_index,
                priority: tilemap_entry.tile_priority(),
            })
        }
    }

    fn obj_sample(&self, x: u16, y: u16, enabled: bool) -> Option<ObjSample> {
        if !enabled {
            return None;
        }

        for i in 0..128 {
            let (low, high) = self.oam.get_sprite(i);
            let x_full = low.x as i16 | ((high.x_position_bit_8() as i16) << 8);
            let x_signed = if x_full >= 256 { x_full - 512 } else { x_full };
            let sprite_size = self.obsel.get_object_size(high.size());
            let tile_row = ((y - 1) as u8).wrapping_sub(low.y);
            let bounds_check = (x as i16) >= x_signed
                && (x as i16) < x_signed + sprite_size as i16
                && (tile_row as i16) < sprite_size as i16;

            if !bounds_check {
                continue;
            }
            let tile_col = (x as i16) - x_signed;

            let tiles_per_axis = (sprite_size / 8) as u8;
            let mut sub_tile_col = (tile_col / 8) as u8;
            let mut sub_tile_row = tile_row / 8;

            if low.packed_attributes.x_flip() {
                sub_tile_col = (tiles_per_axis - 1) - sub_tile_col;
            }

            if low.packed_attributes.y_flip() {
                sub_tile_row = (tiles_per_axis - 1) - sub_tile_row;
            }

            let lower_nibble = ((low.tile_number & 0x0F) + sub_tile_col) & 0x0F;
            let upper_nibble = (low.tile_number & 0xF0) + (sub_tile_row * 0x10);

            let tile_number = upper_nibble | lower_nibble;
            let base_word_address = if low.packed_attributes.name_table() {
                ((self.obsel.name_base() as u16).wrapping_mul(0x2000))
                    .wrapping_add((self.obsel.name_select() as u16 + 1).wrapping_mul(0x1000))
            } else {
                (self.obsel.name_base() as u16).wrapping_mul(0x2000)
            };

            let tile_word_address =
                base_word_address.wrapping_add((tile_number as u16).wrapping_mul(16));
            let pixel_x_within_tile = (tile_col % 8) as u16;
            let mut pixel_y_within_tile = (tile_row % 8) as u16;
            if low.packed_attributes.y_flip() {
                pixel_y_within_tile = 7 - pixel_y_within_tile;
            }

            let bitplane_01_address = tile_word_address + pixel_y_within_tile;
            let bitplane_01 = self.vram.read_word(bitplane_01_address);

            let bit = if low.packed_attributes.x_flip() {
                pixel_x_within_tile
            } else {
                7 - pixel_x_within_tile
            };
            let plane_0 = ((bitplane_01 & 0xFF) >> bit) & 0b1;
            let plane_1 = (((bitplane_01 & 0xFF00) >> 8) >> bit) & 0b1;

            let bitplane_23_address = tile_word_address + 8 + pixel_y_within_tile;
            let bitplane_23 = self.vram.read_word(bitplane_23_address);
            let plane_2 = ((bitplane_23 & 0xFF) >> bit) & 0b1;
            let plane_3 = (((bitplane_23 & 0xFF00) >> 8) >> bit) & 0b1;

            let character_data = plane_0 | (plane_1 << 1) | (plane_2 << 2) | (plane_3 << 3);

            if character_data == 0 {
                continue;
            } else {
                return Some(ObjSample {
                    cg_ram_index: (128
                        + low.packed_attributes.palette() as u16 * 16
                        + character_data) as u8,
                    priority: low.packed_attributes.priority(),
                });
            }
        }

        None
    }

    fn get_planes(
        &self,
        tile_base: u16,
        pixel_y_within_tile: u16,
        bit: u16,
        addend: u16,
    ) -> (u16, u16) {
        let bitplane_address = tile_base + addend + pixel_y_within_tile;
        let bitplane = self.vram.read_word(bitplane_address);
        let plane_lo = ((bitplane & 0xFF) >> bit) & 0b1;
        let plane_hi = (((bitplane & 0xFF00) >> 8) >> bit) & 0b1;

        (plane_lo, plane_hi)
    }

    fn get_screen_offset(&self, bg_tilemap: &BgTilemap, tile_x: u16, tile_y: u16) -> u16 {
        match bg_tilemap.mirror_size() {
            0 => 0,
            1 => (tile_x / 32) * 0x400,
            2 => (tile_y / 32) * 0x400,
            3 => (tile_x / 32) * 0x400 + (tile_y / 32) * 0x800,
            _ => unimplemented!(),
        }
    }

    fn is_enabled(
        &self,
        enabled: bool,
        x: u8,
        mode_1: u8,
        mode_2: u8,
        logic: u8,
        disabled: bool,
    ) -> bool {
        enabled && !(self.is_layer_active(x, mode_1, mode_2, logic) && disabled)
    }

    fn is_layer_active(&self, x: u8, mode_1: u8, mode_2: u8, logic: u8) -> bool {
        let window1_active = self.window_bounds_1.is_active(x, mode_1);
        let window2_active = self.window_bounds_2.is_active(x, mode_2);

        self.combine_windows(window1_active, window2_active, logic)
    }

    fn combine_windows(
        &self,
        window1_active: Option<bool>,
        window2_active: Option<bool>,
        logic: u8,
    ) -> bool {
        match (window1_active, window2_active) {
            (None, None) => false,
            (Some(w1), None) => w1,
            (None, Some(w2)) => w2,
            (Some(w1), Some(w2)) => match logic {
                0 => w1 || w2,
                1 => w1 && w2,
                2 => w1 ^ w2,
                3 => !(w1 ^ w2),
                _ => unimplemented!(),
            },
        }
    }

    fn apply_color_math(&self, color: &mut Rgb, sub_color: Rgb, suppress_div2: bool) {
        let mut r;
        let mut g;
        let mut b;

        if self.cgadsub.add_or_subtract() {
            r = (color.red() as i16) - (sub_color.red() as i16);
            g = (color.green() as i16) - (sub_color.green() as i16);
            b = (color.blue() as i16) - (sub_color.blue() as i16);
        } else {
            r = (color.red() as i16) + (sub_color.red() as i16);
            g = (color.green() as i16) + (sub_color.green() as i16);
            b = (color.blue() as i16) + (sub_color.blue() as i16);
        }

        if self.cgadsub.half_math() && !suppress_div2 {
            r /= 2;
            g /= 2;
            b /= 2;
        }

        color.set_red(r.clamp(0, 31) as u16);
        color.set_green(g.clamp(0, 31) as u16);
        color.set_blue(b.clamp(0, 31) as u16);
    }
}
