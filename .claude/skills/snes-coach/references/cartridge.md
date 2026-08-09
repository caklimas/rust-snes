# SNES Cartridges

**General Cartridge Info**

**Basic Mapping Schemes**

**Cartridges with External Programmable CPUs**

**Cartridges with External Pre-Programmed CPUs**

**Cartridges with other custom chips**

**Cartridges with Real-Time Clocks**

Moreover, the Satellaview has a time function (allowing to receive the time via
satellite dish). And, there are S-3520 (Seiko RTC's) in the Super Famicom Box
and in the Nintendo Super System.

**Special Non-game Cartridges/Expansions**

### SNES Cartridge ROM Header

The Cartridge header is mapped to 00FFxxh in SNES memory (near the exception
vectors). In ROM-images it is found at offset 007Fxxh (LoROM), 00FFxxh (HiROM),
or 40FFxxh (ExHiROM); add +200h to that offsets if "(imagesize AND 3FFh)=200h",
ie. if there's an extra header from SWC/UFO/etc. copiers.

**Cartridge Header (Area FFC0h..FFCFh)**

```
  FFC0h  Cartridge title (21 bytes, uppercase ascii, padded with spaces)
  FFC0h  First byte of title (or 5Ch far-jump-opcode in Pirate X-in-1 Carts)
  FFD4h  Last byte of title (or 00h indicating Early Extended Header)
  FFD5h  Rom Makeup / ROM Speed and Map Mode (see below)
  FFD6h  Chipset (ROM/RAM information on cart) (see below)
  FFD7h  ROM size (1 SHL n) Kbytes (usually 8=256KByte .. 0Ch=4MByte)
          Values are rounded-up for carts with 10,12,20,24 Mbits
  FFD8h  RAM size (1 SHL n) Kbytes (usually 1=2Kbyte .. 5=32Kbyte) (0=None)
  FFD9h  Country (also implies PAL/NTSC) (see below)
  FFDAh  Developer ID code  (00h=None/Homebrew, 01h=Nintendo, etc.) (33h=New)
  FFDBh  ROM Version number (00h=First)
  FFDCh  Checksum complement (same as below, XORed with FFFFh)
  FFDEh  Checksum (all bytes in ROM added together; assume [FFDC-F]=FF,FF,0,0)
```

**Extended Header (Area FFB0h..FFBFh) (newer carts only)**

Early Extended Header (1993) (when [FFD4h]=00h; Last byte of Title=00h):

```
  FFB0h  Reserved   (15 zero bytes)
```

Later Extended Header (1994) (when [FFDAh]=33h; Old Maker Code=33h):

```
  FFB0h  Maker Code (2-letter ASCII, eg. "01"=Nintendo)
  FFB2h  Game Code  (4-letter ASCII) (or old 2-letter padded with 20h,20h)
  FFB6h  Reserved   (6 zero bytes)
  FFBCh  Expansion FLASH Size (1 SHL n) Kbytes (used in JRA PAT)
  FFBDh  Expansion RAM Size (1 SHL n) Kbytes (in GSUn games) (without battery?)
  FFBEh  Special Version      (usually zero) (eg. promotional version)
```

Both Early and Later Extended Headers:

```
  FFBFh  Chipset Sub-type (usually zero) (used when [FFD6h]=Fxh)
```

Note: The early-extension is used only with ST010/11 games.

If the first letter of the 4-letter Game Code is "Z", then the cartridge does
have Satellaview-like Data Pack/FLASH cartridge slot (this applies ONLY to
"Zxxx" 4-letter codes, not to old "Zx  " space padded 2-letter codes).

**Cartridge Header Variants**

The BS-X Satellaview FLASH Card Files, and Sufami Turbo Mini-Cartridges are
using similar looking (but not fully identical) headers (and usually the same
.SMC file extension) than normal ROM cartridges. Detecting the content of .SMC
files can be done by examining ID Strings (Sufami Turbo), or differently
calculated checksum values (Satellaview). For details, see:

Homebrew games (and copiers & cheat devices) are usually having several
errors in the cartridge header (usually no checksum, zero-padded title, etc),
they should (hopefully) contain valid entryoints in range 8000h..FFFEh. Many
Copiers are using 8Kbyte ROM bank(s) - in that special case the exception
vectors are located at offset 1Fxxh within the ROM-image.

**CPU Exception Vectors (Area FFE0h..FFFFh)**

```
  FFE0h  Zerofilled (or ID "XBOO" for WRAM-Boot compatible files)
  FFE4h  COP vector     (65C816 mode) (COP opcode)
  FFE6h  BRK vector     (65C816 mode) (BRK opcode)
  FFE8h  ABORT vector   (65C816 mode) (not used in SNES)
  FFEAh  NMI vector     (65C816 mode) (SNES V-Blank Interrupt)
  FFECh  ...
  FFEEh  IRQ vector     (65C816 mode) (SNES H/V-Timer or External Interrupt)
  FFF0h  ...
  FFF4h  COP vector     (6502 mode)
  FFF6h  ...
  FFF8h  ABORT vector   (6502 mode) (not used in SNES)
  FFFAh  NMI vector     (6502 mode)
  FFFCh  RESET vector   (6502 mode) (CPU is always in 6502 mode on RESET)
  FFFEh  IRQ/BRK vector (6502 mode)
```

Note: Exception Vectors are variable in SA-1 and CX4, and fixed in GSU.

**Text Fields**

The ASCII fields can use chr(20h..7Eh), actually they are JIS (with Yen instead
backslash).

**ROM Size / Checksum Notes**

The ROM size is specified as "(1 SHL n) Kbytes", however, some cartridges
contain "odd" sizes:

```
  * Game uses 2-3 ROM chips (eg. one 8MBit plus one 2MBit chip)
  * Game originally designed for 2 ROMs, but later manufactured as 1 ROM (?)
  * Game uses a single 24MBit chip (23C2401)
```

In all three cases the ROM Size entry in [FFD7h] is rounded-up. In memory, the
"bigger half" is mapped to address 0, followed by the "smaller half", then
followed by mirror(s) of the smaller half. Eg. a 10MBit game would be rounded
to 16MBit, and mapped (and checksummed) as "8Mbit + 4x2Mbit". In practice:

```
  Title                       Hardware          Size   Checksum
  Dai Kaiju Monogatari 2 (J)  ExHiROM+S-RTC     5MB    4MB + 4 x Last 1MB
  Tales of Phantasia (J)      ExHiROM           6MB    <???>
  Star Ocean (J)              LoROM+S-DD1       6MB    4MB + 2 x Last 2MB
  Far East of Eden Zero (J)   HiROM+SPC7110+RTC 5MB    5MB
  Momotaro Dentetsu Happy (J) HiROM+SPC7110     3MB    2 x 3MB
  Sufami Turbo BIOS           LoROM in Minicart xx     without checksum
  Sufami Turbo Games          LoROM in Minicart xx     without checksum
  Dragon Ball Z - Hyper Dimension    LoROM+SA-1 3MB       Overdump 4MB
  SD Gundam GNext (J)                LoROM+SA-1 1.5MB     Overdump 2MB
  Megaman X2                         LoROM+CX4  1.5MB     Overdump 2MB
  BS Super Mahjong Taikai (J)        BS           Overdump/Mirr+Empty
  Demon's Crest... reportedly 12MBit ? but, that's bullshit ?
```

```
  SPC7110 Title               ROM Size (Header value)    Checksum
  Super Power League 4        2MB      (rounded to 2MB)  1x(All 2MB)
  Momotaro Dentetsu Happy (J) 3MB      (rounded to 4MB)  2x(All 3MB)
  Far East of Eden Zero (J)   5MB      (rounded to 8MB)  1x(All 5MB)
```

On-chip ROM contained in external CPUs (DSPn,ST01n,CX4) is NOT counted in the
ROM size entry, and not included in the checksum.

Homebrew files often contain 0000h,0000h or FFFFh,0000h as checksum value.

**ROM Speed and Map Mode (FFD5h)**

```
  Bit7-6 Always 0
  Bit5   Always 1 (maybe meant to be MSB of bit4, for "2" and "3" MHz)
  Bit4   Speed (0=Slow, 1=Fast)              (Slow 200ns, Fast 120ns)
  Bit3-0 Map Mode
```

Map Mode can be:

```
  0=LoROM/32K Banks             Mode 20 (LoROM)
  1=HiROM/64K Banks             Mode 21 (HiROM)
  2=LoROM/32K Banks + S-DD1     Mode 22 (mappable) "Super MMC"
  3=LoROM/32K Banks + SA-1      Mode 23 (mappable) "Emulates Super MMC"
  5=HiROM/64K Banks             Mode 25 (ExHiROM)
  A=HiROM/64K Banks + SPC7110   Mode 25? (mappable)
```

Note: ExHiROM is used only by "Dai Kaiju Monogatari 2 (JP)" and "Tales of
Phantasia (JP)".

**Chipset (ROM/RAM information on cart) (FFD6h) (and some subclassed via FFBFh)**

```
  00h     ROM
  01h     ROM+RAM
  02h     ROM+RAM+Battery
  x3h     ROM+Co-processor
  x4h     ROM+Co-processor+RAM
  x5h     ROM+Co-processor+RAM+Battery
  x6h     ROM+Co-processor+Battery
  x9h     ROM+Co-processor+RAM+Battery+RTC-4513
  xAh     ROM+Co-processor+RAM+Battery+overclocked GSU1 ? (Stunt Race)
  x2h     Same as x5h, used in "F1 Grand Prix Sample (J)" (?)
  0xh     Co-processor is DSP    (DSP1,DSP1A,DSP1B,DSP2,DSP3,DSP4)
  1xh     Co-processor is GSU    (MarioChip1,GSU1,GSU2,GSU2-SP1)
  2xh     Co-processor is OBC1
  3xh     Co-processor is SA-1
  4xh     Co-processor is S-DD1
  5xh     Co-processor is S-RTC
  Exh     Co-processor is Other  (Super Gameboy/Satellaview)
  Fxh.xxh Co-processor is Custom (subclassed via [FFBFh]=xxh)
  Fxh.00h Co-processor is Custom (SPC7110)
  Fxh.01h Co-processor is Custom (ST010/ST011)
  Fxh.02h Co-processor is Custom (ST018)
  Fxh.10h Co-processor is Custom (CX4)
```

In practice, following values are used:

```
  00h     ROM             ;if gamecode="042J" --> ROM+SGB2
  01h     ROM+RAM (if any such produced?)
  02h     ROM+RAM+Battery ;if gamecode="XBND" --> ROM+RAM+Batt+XBandModem
                          ;if gamecode="MENU" --> ROM+RAM+Batt+Nintendo Power
  03h     ROM+DSP
  04h     ROM+DSP+RAM (no such produced)
  05h     ROM+DSP+RAM+Battery
  13h     ROM+MarioChip1/ExpansionRAM (and "hacked version of OBC1")
  14h     ROM+GSU+RAM                    ;\ROM size up to 1MByte -> GSU1
  15h     ROM+GSU+RAM+Battery            ;/ROM size above 1MByte -> GSU2
  1Ah     ROM+GSU1+RAM+Battery+Fast Mode? (Stunt Race)
  25h     ROM+OBC1+RAM+Battery
  32h     ROM+SA1+RAM+Battery (?) "F1 Grand Prix Sample (J)"
  34h     ROM+SA1+RAM (?) "Dragon Ball Z - Hyper Dimension"
  35h     ROM+SA1+RAM+Battery
  43h     ROM+S-DD1
  45h     ROM+S-DD1+RAM+Battery
  55h     ROM+S-RTC+RAM+Battery
  E3h     ROM+Super Gameboy      (SGB)
  E5h     ROM+Satellaview BIOS   (BS-X)
  F5h.00h ROM+Custom+RAM+Battery     (SPC7110)
  F9h.00h ROM+Custom+RAM+Battery+RTC (SPC7110+RTC)
  F6h.01h ROM+Custom+Battery         (ST010/ST011)
  F5h.02h ROM+Custom+RAM+Battery     (ST018)
  F3h.10h ROM+Custom                 (CX4)
```

**Country (also implies PAL/NTSC) (FFD9h)**

```
  00h -  International (eg. SGB)  (any)
  00h J  Japan                    (NTSC)
  01h E  USA and Canada           (NTSC)
  02h P  Europe, Oceania, Asia    (PAL)
  03h W  Sweden/Scandinavia       (PAL)
  04h -  Finland                  (PAL)
  05h -  Denmark                  (PAL)
  06h F  France                   (SECAM, PAL-like 50Hz)
  07h H  Holland                  (PAL)
  08h S  Spain                    (PAL)
  09h D  Germany, Austria, Switz  (PAL)
  0Ah I  Italy                    (PAL)
  0Bh C  China, Hong Kong         (PAL)
  0Ch -  Indonesia                (PAL)
  0Dh K  South Korea              (NTSC) (North Korea would be PAL)
  0Eh A  Common (?)               (?)
  0Fh N  Canada                   (NTSC)
  10h B  Brazil                   (PAL-M, NTSC-like 60Hz)
  11h U  Australia                (PAL)
  12h X  Other variation          (?)
  13h Y  Other variation          (?)
  14h Z  Other variation          (?)
```

Above shows the [FFD9h] value, and the last letter of 4-character game codes.

**Game Codes (FFB2h, exists only when [FFDAh]=33h)**

```
  "xxxx"  Normal 4-letter code (usually "Axxx") (or "Bxxx" for newer codes)
  "xx  "  Old 2-letter code (space padded)
  "042J"  Super Gameboy 2
  "MENU"  Nintendo Power FLASH Cartridge Menu
  "Txxx"  NTT JRA-PAT and SPAT4 (SFC Modem BIOSes)
  "XBND"  X-Band Modem BIOS
  "Zxxx"  Special Cartridge with satellaview-like Data Pack Slot
```

The last letter indicates the region (see Country/FFD9h description) (except in
2-letter codes and "MENU"/"XBND" codes).

### SNES Cartridge PCBs

**Cartridge PCB Naming (eg. SHVC-XXXX-NN)**

Prefix

```
  SHVC  Normal cartridge (japan, usa, europe)
  SNSP  Special PAL version (for SA1 and S-DD1 with built-in CIC)
  BSC   BIOS (or game cartridge) with external Satellaview FLASH cartridge slot
  MAXI  Majesco Sales Inc cartridge (Assembled in Mexico)
  MJSC  Majesco Sales Inc cartridge (Assembled in Mexico)
  WEI   Whatever? (Assembled in Mexico)
  EA    Electronics Arts cartridge
```

First Character

```
  1   One ROM chip (usually 36pin, sometimes 32pin)
  Y   Two 4Mbit ROM chips     (controlled by 74LS00)
  2   Two 8Mbit ROM chips     (controlled by 74LS00,MAD-1,etc.)
  B   Two 16Mbit ROM chips    (controlled by 74LS00 or MAD-1)
  L   Two 32Mbit ROM chips    (controlled by SPC7110F,S-DD1 or MAD-1)
  3   Three 8Mbit ROM chips   (controlled by 74LS139) (decoder/demultiplexer)
  4   Four ROM chips (used only for 4PVnn/4QW EPROM prototype boards)
  8   Eight ROM chips (used only for 8PVnn/8Xnn EPROM prototype boards)
```

Second Character(s)

```
  A   LoRom (A15 / Pin40 not connected to ROM)   (uh, 1A3B-20 ?)
  B   LoRom plus DSP-N chip
  C   LoRom plus Mario Chip 1               62pin  (and 36pin ROM) (no X1)
  CA  LoRom plus GSU-1                      62pin  (and 32pin ROM)
  CB  LoRom plus GSU-2 or GSU-2-SP1         62pin  (and 40pin ROM)
  DC  LoRom plus CX4                        62pin  (and 32pin ROMs)
  DH  HiRom plus SPC7110F                   62pin  (and 32pin+44pin ROMs)
  DE  LoRom plus ST018                      62pin  (and 32..40pin ROM possible)
  DS  LoRom plus ST010/ST011                62pin  (and 32..36pin ROM possible)
  E   LoRom plus OBC1 chip                  62pin  (and 32pin ROMs)
  J   HiRom (A15 / Pin40 is connected to ROM)
  K   HiRom plus DSP-N chip
  L   LoRom plus SA1 chip       62pin  (and 44pin ROM) (16bit data?)
  N   LoRom plus S-DD1 chip     62pin  (and 44pin ROM)
  P   LoRom with 2 prototype EPROMs (=unlike ROM A16..Ahi,/CS)
  PV  WhateverRom with 4 prototype EPROMs (=unlike ROM A16..Ahi,/CS)
  Q   WhateverRom prototype (see book2.pdf)
  QW  WhateverRom prototype (see book2.pdf)
  RA  LoRom plus GSU1A with prototype EPROMs (=unlike ROM A16..Ahi,/CS)  62pin
  X   WhateverRom prototype (see book2.pdf)
```

Third Character

```
  0   No SRAM
  1   2Kx8 SRAM (usually narrow 24pin DIP, sometimes wide 24pin DIP)
  2   prototype variable size SRAM
  3   8Kx8 SRAM (usually wide 28pin DIP)
  5   32Kx8 SRAM (usually wide 28pin DIP)
  6   64Kx8 SRAM (32pin SMD, found on boards with GSU)
  8   64Kx8 SRAM (in one SA1 cart) (seems to be a 64Kx8 chip, not 256Kx8)
```

Forth Character

```
  N   No battery
  B   Battery (with Transistor+Diodes or MM1026/MM1134 chip)
  M   Battery (with MAD-1 chip; or with rare MAD-R chip)
  X   Battery (with MAD-2 chip; maybe amplifies X1 oscillator for DSP1B chips)
  C   Battery and RTC-4513
  R   Battery and S-RTC
  F   FLASH Memory (instead of SRAM) (used by JRA-PAT and SPAT4)
```

Fifth Characters (only if cart contains RAM that is NOT battery-backed)

```
  5S  32Kx8 SRAM (for use by GSU, not battery backed)
  6S  64Kx8 SRAM (for use by GSU, not battery backed)
  7S  64Kx8 or 128Kx8 SRAM (for use by GSU, not battery backed)
  9P  512Kx8 PSRAM (32pin 658512LFP-85) (for satellaview)
  (the black-blob Star Fox PCB also contains RAM, but lacks the ending "nS")
```

Suffix

```
  -NN revision number (unknown if this indicates any relevant changes)
```

**Other Cartridge PCBs (that don't follow the above naming system)**

```
  CPU2 SGB-R-10  Super Gameboy (1994)
  SHVC-MMS-X1    Nintendo Power FLASH Cartridge (1997) (older version)
  SHVC-MMS-02    Nintendo Power FLASH Cartridge (1997) (newer version)
  SHVC-MMSA-1    Nintendo Power FLASH Cartridge (19xx) ???
  SHVC-SGB2-01   Super Gameboy 2 (1998)
  SHVC-1C0N      Star Fox (black blob version) (PCB name lacks ending nS-NN)
  SHVC TURBO     Sufami Turbo BASE CASSETTE (Bandai)
  <unknown?>     Sufami Turbo game cartridges
  123-0002-16    X-Band Modem (1995 by Catapult / licensed by Nintendo)
  BSMC-AF-01     Satellaview Mini FLASH Cartridge (plugged into BIOS cartridge)
  BSMC-CR-01     Satellaview Mini FLASH Cartridge (???) (not rewriteable ?)
  GPC-RAMC-4M    SRAM Cartridge (without ROM)?
  GPC-RAMC-S1    SRAM Cartridge (without ROM)?
  GS 0871-102    Super Famicom Box multi-game cartridge
  NSS-01-ROM-A   Nintendo Super System (NSS) cartridge
  NSS-01-ROM-B   Nintendo Super System (NSS) cartridge
  NSS-01-ROM-C   Nintendo Super System (NSS) cartridge
  NSS-X1-ROM-C   Rebadged NSS-01-ROM-C board (plus battery/sram installed)
  RB-01, K-PE1-945-01   SNES CD Super Disc BIOS Cartridge (prototype)
```

**ROM Chips used in SNES cartridges**

```
  2Mbit 256Kbyte        LH532 TC532 N-2001 (2nd chip/2A0N) (+SGB) (+Sufami)
  4Mbit 512Kbyte 23C401 LH534 TC534 HN623n4 HN623x5 23C4001 LH5S4 CAT534 CXK384
  8Mbit  1Mbyte  23C801 LH538 TC538 HN623n8 23C8001 TC23C8003 CAT548
  16Mbit 2Mbyte  23C1601 LH537 LHMN7 TC5316 M5316
  24Mbit 3Mbyte  23C2401 (seen on SHVC-1J3M board)
  32Mbit 4Mbyte  23C3201 LH535 LHMN5 M5332 23C3202/40pin/SA1 N-32000/44pin/DD1
```

**DIP vs SMD vs Blobs**

Most SNES carts are using DIP chips. SMD chips are used only in carts with
coprocessors (except S-RTC, DSP-n, ST01n). Black blobs are found in several
pirate carts (and in Star Fox, which contains Nintendo's Mario Chip 1, so it's
apparently not a pirate cart).

### SNES Cartridge ROM-Image Headers and File Extensions

Below file headers are dated back to back-up units, which allowed to load
ROM-images from 1.44MB floppy disks into RAM, larger images have been split
into "multi files".

Many of these files do have 512-byte headers. The headers don't contain any
useful information. So, if they are present: Just ignore them. Best way to
detect them is: "IF (filesize AND 3FFh)=200h THEN HeaderPresent=True"
(Headerless cartridges are always sized N*1024 bytes, Carts with header are
N*1024+512 bytes).

**.SMC - Super MagiCom (by Front Far East)**

This extension is often used for ANY type of SNES ROM-images, including for SWC
files.

**.SWC - Super Wild Card (SWC) Header (by Front Far East)**

```
  000h-001h  ROM Size (in 8Kbyte units)
  002h       Program execution mode
              Bit   Expl.
              7     Entrypoint (0=Normal/Reset Vector, 1=JMP 8000h)
              6     Multi File (0=Normal/Last file, 1=Further file(s) follow)
              5     SRAM mapping    (0=mode20, 1=mode21)
              4     Program mapping (0=mode20, 1=mode21)
              3-2   SRAM Size  (0=32Kbytes, 1=8Kbytes, 2=2Kbytes, 3=None)
              1     Reserved   (zero)
              0     Unknown    (seems to be randomly set to 0 or 1)
  003h       Reserved (zero) (but, set to 01h in homebrew "Pacman" and "Nuke")
  004h-007h  Reserved (zero)
  008h-009h  SWC File ID (AAh, BBh)
  00Ah       File Type (04h=Program ROM, 05h=Battery SRAM, 08h=real-time save)
  00Bh-1FFh  Reserved (zero)
```

**.FIG - Pro Fighter (FIG) header format (by China Coach Ltd)**

```
  000h-001h  ROM Size (in 8Kbyte units)
  002h       Multi File (00h=Normal/Last file, 40h=Further file(s) follow)
                        (02h=Whatever, used in homebrew Miracle,Eagle,Cen-Dem)
  003h       ROM Mode   (00h=LoROM, 80h=HiROM)
  004h-005h  DSP1/SRAM Mode (8377h=ROM, 8347h=ROM+DSP1, 82FDh=ROM+DSP1+SRAM)
  006h-1FFh  Reserved (zero) (or garbage at 01FCh in homebrew Darkness Demo)
```

**.BIN - Raw Binary**

Contains a raw ROM-image without separate file header.

**.078 - Game Doctor file name format (by Bung)**

Contains a raw ROM-image without separate file header.

Information about multi files is encoded in the "SFxxyyyz.078" filenames.

```
  SF   Abbreviation for Super Famicom
  xx   Image size in Mbit (2,4,8,16,32) (1-2 chars, WITHOUT leading zero)
  yyy  Game catalogue number (or random number if unknown)
  z    Indicates multi file (A=first, B=second, etc.)
  078  File extension (should be usually 078)
```

**.MGD - Multi Game Doctor ? (by Bung)**

Format Unknown?

**Another Game Doctor version...**

```
  000h-00Fh  ID "GAME DOCTOR SF 3"
  010h       Unknown (80h)      ;-SRAM size limit
  011h       Unknown (20h)      ;\
  012h       Unknown (21h)      ; DRAM mapping related
  013h-018h  Unknown (6x60h)    ;
  019h       Unknown (20h)      ;
  01Ah       Unknown (21h)      ;
  01Bh-028h  Unknown (14x60h)   ;/
  029h-02Ah  Zero               ;-SRAM mapping related
   011h-020h  512Kbyte DRAM chunk, mapped to upper 32Kbyte of Bank 0xh-Fxh
   021h-024h  512Kbyte DRAM chunk, mapped to lower 32Kbyte of Bank 4xh-7xh
   025h-028h  512Kbyte DRAM chunk, mapped to lower 32Kbyte of Bank Cxh-Fxh
   029h-02Ah  SRAM Flags (bit0-15 = Enable SRAM at 6000-7000 in banks 0xh-Fxh)
  02Bh-1FFh  Zero (Reserved)
```

**Superufo**

```
  000h       Unknown (20h or 40h)   ;maybe ROM size in 8K units   ?
  001h-007h  Zero
  008h-00Fh  ID "SUPERUFO"
  010h       Unknown (01h)
  011h       Unknown (02h or 04h)   ;maybe rom speed              ?
  012h       Unknown (E1h or F1h)   ;MSB=chipset (Exh or Fxh)     ?
  013h       Unknown (00h)
  014h       Unknown (01h)
  015h       Unknown (03h)
  016h       Unknown (00h)
  017h       Unknown (03h)
  018h-1FFh  Zero
```

**.SFC - Nintendo Developer File (Nintendo)**

Contains a raw ROM-image without separate file header.

Information about multi files is encoded in the "NnnnVv-N.SFC" filenames.

```
  Nnnn  Game code (4 letters)
  Vv    ROM Version
  N     Disk Number (0=First)
  SFC   Fixed extension (Super FamiCom)
```

This is how Nintendo wanted developers to name their files.

**NSRT Header (can be generated by Nach's NSRT tool)**

This format stores some additional information in a formerly unused 32-byte
area near the end of the 512-byte copier headers.

```
  1D0h  Unknown/unspecified (LSB=01h..03h, MSB=00h..0Dh) ;maybe ROM mapping
  1D1h  Unknown/unspecified   ;maybe title and/or NSRT version
  1E8h  ID1 "NSRT"
  1ECh  ID2 16h (22 decimal)
  1EDh  Controllers (MSB=Port1, LSB=Port2)
  1EEh  Checksum (sum of bytes at [1D0h..1EDh]+FFh)
  1EFh  Checksum Complement (Checksum XOR FFh)
```

Controller Values:

```
  00h Gamepad
  01h Mouse
  02h Mouse or Gamepad
  03h Super Scope
  04h Super Scope or Gamepad
  05h Justifier
  06h Multitap
  07h Mouse, Super Scope, or Gamepad
  08h Mouse or Multitap
  09h Lasabirdie
  0Ah Barcode Battler
  0Bh..0Fh Reserved
```

Most copiers also include a parallel PC port interface, allowing

your PC to control the unit and store images on your hard drive.

Copier's contain DRAM from 1 Megabyte to 16 Megabytes, 8MegaBits to

128MegaBits respectively. This is the reason why they are so expensive.

### SNES Cartridge ROM-Image Interleave

Some ROM images are "interleaved", meaning that their content is ordered
differently as how it appears in SNES memory. The interleaved format dates back
to old copiers, most modern tools use/prefer normal ROM-images without
interleave, but old interleaved files may still show up here and there.

**Interleave used by Game Doctor & UFO Copiers**

These copiers use interleave so that the ROM Header is always stored at file
offset 007Fxxh. For HiROM files, interleave is applied as so: store upper 32K
of all 64K banks, followed by lower 32K of all 64K banks (which moves the
header from 00FFxxh to 007Fxxh). For example, with a 320Kbyte ROM, the ten 32K
banks would be ordered as so:

```
  0,1,2,3,4,5,6,7,8,9 - Original
  1,3,5,7,9,0,2,4,6,8 - Interleaved
```

For LoROM files, there's no interleave applied (since the header is already at
007Fxxh).

Detecting an interleaved file could be done as so:

```
  Header must be located at file offset 007Fxxh (ie. in LoROM fashion)
  Header must not be a Sufami Turbo header (=title "ADD-ON BASE CASSETE")
  Header must not be a Satellaview header (=different chksum algorithm)
  Header should not contain corrupted entries
  The "Map Mode" byte at "[007FD5h] ANDed with 0Fh" is 01h,05h,0Ah (=HiROM)
```

If so, the file is interleaved (or, possibly, it's having a corrupted header
with wrong map mode setting).

**Interleave used by Human Stupidity**

There are interleaving & deinterleaving tools, intended to convert normal
ROM-images to/from the format used by above copiers. Using that tools on files
that are already in the desired format will result in messed-up garbage. For
example, interleaving a 320Kbyte file that was already interleaved:

```
  1,3,5,7,9,0,2,4,6,8 - Interleaved
  3,7,0,4,8,1,5,9,2,6 - Double-Interleaved
```

Or, trying to deinterleave a 320Kbyte file that wasn't interleaved:

```
  0,1,2,3,4,5,6,7,8,9 - Original
  5,0,6,1,7,2,8,3,9,4 - Mis-de-interleaved
```

One can eventually repair such files by doing the opposite (de-)interleaving
action. Or, in worst case, the user may repeat the wrong action, ending up with
a Triple-Interleaved, or Double-mis-de-interleaved file.

Another case of stupidity would be applying interleave to a LoROM file (which
would move the header <away from> 007Fxxh towards the middle of the file,
ie. the opposite of the intended interleaving effect of moving it <to>
007Fxxh).

**ExHiROM Files**

ExHiROM Files are also having the data ordered differently as in SNES memory.
However, in this special case, the data SHOULD be ordered as so. The ordering
is: Fast HiROM (4Mbytes, banks C0h..FFh), followed by Slow HiROM banks (usually
1-2MByte, banks 40h..4Fh/5Fh) (of which, the header and exception vectors are
in upper 32K of the first Slow HiROM bank, ie. at file offset 40FFxxh). There
are only 2 games using the ExHiROM format:

```
  Dai Kaiju Monogatari 2 (JP) (5Mbytes) PCB: SHVC-LJ3R-01
  Tales of Phantasia (JP) (6Mbytes)     PCB: SHVC-LJ3M-01
```

The ExHiROM ordering is somewhat "official" as it was defined in Nintendo's
developer manuals. Concerning software, the ordering does match-up with the
checksum calculation algorithm (8MB chksum across 4MB plus mirror(s) of
remaining 1-2MB). Concerning hardware, the ordering may have been 'required' in
case Nintendo did (or planned to) use "odd" sized 5Mbyte/6Mbyte-chips (they DID
produce cartridges with 3MByte/24Mbit chips).

### SNES Cartridge CIC Lockout Chip

SNES cartridges are required to contain a CIC chip (security chip aka lockout
chip). The CIC is a small 4bit CPU with built-in ROM. An identical CIC is
located in the SNES console. The same 4bit CPU (but with slightly different
code in ROM) is also used in NES consoles/cartridges.

The CIC in the console is acting as "lock", and that in the cartridge is acting
as "key". The two chips are sending random-like bitstreams to each other, if
the data (or transmission timing) doesn't match the expected values, then the
"lock" issues a RESET signal to the console. Thereby rejecting cartridges
without CIC chip (or such with CICs for wrong regions).

**CIC Details**

**CIC Disable**

### SNES Cartridge CIC Pseudo Code

**CicMain**

```
  CicInitFirst, CicInitTiming, CicRandomSeed, CicInitStreams
  time=data_start, a=1, noswap=1, if snes then noswap=0
 mainloop:
  for x=a to 0Fh
    if nes then Wait(time-5), else if snes then (time-7)     ;\verify idle
    if (nes_6113=0) and (P0.0=1 or P0.1=1) then Shutdown     ;/
    Wait(time+0)                                             ;\
    if (console xor snes) then a=[00h+x].0, else a=[10h+x].0 ; output data
    if noswap then P0.0=a, else P0.1=a                       ;/
    Wait(time+2-data_rx_error)                               ;\
    if (console xor snes) then a=[10h+x].0, else a=[00h+x].0 ; verify input
    if noswap then a=(a xor P0.1), else a=(a xor P0.0)       ;
    if a=1 then Shutdown                                     ;/
    Wait(time+3)                                             ;\output idle
    if noswap then P0.0=0, else P0.1=0                       ;/
    if snes then time=time+92, else if nes then time=time+79
  next x
  CicMangle(00h), CicMangle(10h)                        ;\mangle
  if snes then CicMangle(00h), CicMangle(10h)           ; (thrice on SNES)
  if snes then CicMangle(00h), CicMangle(10h)           ;/
  if snes then noswap=[17h].0   ;eventually swap input/output pins (SNES only)
  a=[17h]
  if a=0 then a=1, time=time+2
  if snes then time=time+44, else if nes then time=time+29
  goto mainloop
```

**CicMangle(buf)**

```
  for i=[buf+0Fh]+1 downto 1
    a=[buf+2]+[buf+3h]+1
    if a<10h then x=[buf+3], [buf+3]=a, a=x, x=1, else x=0
    [buf+3+x]=[buf+3+x]+a
    for a=x+6 to 0Fh, [buf+a]=[buf+a]+[buf+a-1]+1, next a
    a=[buf+4+x]+8, if a<10h then [buf+5+x]=[buf+5+x]+a, else [buf+5+x]=a
    [buf+4+x]=[buf+4+x]+[buf+3+x]
    [buf+1]=[buf+1]+i
    [buf+2]=NOT([buf+2]+[buf+1]+1)
    time=time+84-(x*6)
  next i
```

Note: All values in [buf] are 4bit wide (aka ANDed with 0Fh).

**CicInitFirst**

```
  timer=0                       ;reset timer (since reset released)
  P0=00h
  console=P0.3                  ;get console/cartridge flag
  if console
    while P0.2=1, r=r+1         ;get 4bit random seed (capacitor charge time)
    P1.1=1, P1.1=0              ;issue reset to CIC in cartridge
    timer=0                     ;reset timer (since reset released)
  if nes_6113 and (console=1)
    Wait(3), nes_6113_in_console=1, P0.0=1      ;request special 6113 mode
  if nes_6113 and (console=0)
    Wait(6), nes_6113_in_console=P0.1           ;check if 6113 mode requested
```

**CicRandomSeed**

```
  time=seed_start
  for i=0 to 3                  ;send/receive 4bit random seed (r)
    bit=((i+3) and 3)           ;bit order is 3,0,1,2 (!)
    if console=1 Wait(time+0+i*15), P0.0=r.bit, Wait(time+3+i*15), P0.0=0 ;send
    if console=0 Wait(time+2+i*15), r.bit=P0.1                            ;recv
  next i
```

**CicInitStreams**

```
  if snes
    if ntsc then x=9, else if pal then x=6
    [01h..0Fh]=B,1,4,F,4,B,5,7,F,D,6,1,E,9,8   ;init stream from cartridge (!)
    [11h..1Fh]=r,x,A,1,8,5,F,1,1,E,1,0,D,E,C   ;init stream from console   (!)
  if nes_usa                ;3193A
    [01h..0Fh]=1,9,5,2,F,8,2,7,1,9,8,1,1,1,5   ;init stream from console
    [11h..1Fh]=r,9,5,2,1,2,1,7,1,9,8,5,7,1,5   ;init stream from cartridge
    if nes_6113_in_console then overwrite [01h]=5 or so ???   ;special-case
  if nes_europe             ;3195A
    [01h..0Fh]=F,7,B,E,F,8,2,7,D,7,8,E,E,1,5   ;init stream from console
    [11h..1Fh]=r,7,B,D,1,2,1,7,E,6,7,A,7,1,5   ;init stream from cartridge
  if nes_hongkong_asia      ;3196A
    [01h..0Fh]=E,6,A,D,F,8,2,7,E,6,7,E,E,E,A   ;init stream from console
    [11h..1Fh]=r,6,A,D,E,D,E,8,E,6,7,A,7,1,5   ;init stream from cartridge
  if nes_uk_italy_australia ;3197A
    [01h..0Fh]=3,5,8,9,3,7,2,8,8,6,8,5,E,E,B   ;init stream from console
    [11h..1Fh]=r,7,9,A,A,1,6,8,5,8,9,1,5,1,7   ;init stream from cartridge
  if_nes_famicombox         ;3198A
    (unknown)
```

Note: In most cases, the PAL region changes are simply inverted or negated NTSC
values (not/neg), except, one NES-EUR value, and most of the NES-UK values are
somehow different. The rev-engineered NES-UK values may not match the exact
original NES-UK values (but they should be working anyways).

**CicInitTiming**

```
  if snes_d411           -> seed_start=630, data_start=817   ;snes/ntsc
  if snes_d413           -> (unknown?) (same as d411?)       ;snes/pal
  if nes_3193            -> (seems to be same as nes_3195?)  ;nes/usa (v1)
  if nes_3195            -> seed_start=32, data_start=200    ;nes/europe
  if nes_3196            -> (unknown?)                       ;nes/asia
  if nes_3197            -> (unknown?) ("burns five")        ;nes/uk
  if nes_6113            -> seed_start=32, data_start=201    ;nes/usa (v2)
  if nes_6113_in_console -> seed_start=33, data_start=216    ;nes/special
  if nes_tengen          -> seed_start=32, data_start=201    ;nes/cic-clone
  ;now timing errors...
  data_rx_error=0  ;default
  if console=0 and nes_3193a -> randomly add 0 or 0.25 to seed_start/data_start
  if console=0 and snes_d413 -> always add 1.33 to seed_start/data_start (bug)
  if console=0 and nes_6113  -> data_rx_error=1 (and maybe +1.25 on seed/data?)
  if other_chips & chip_revisions -> (unknown?)
```

Note: 3197 reportedly "burns five extra cycles before initialization", but
unknown if that is relative to 3193 <or> 3195 timings, and unknown if it
applies to <both> seed_start and data_start, and unknown if it means 1MHz
<or> 4MHz cycles.

Note: The "data_rx_error" looks totally wrong, but it is somewhat done
intentionally, so there might be a purpose (maybe some rounding, in case 6113
and 3193 are off-sync by a half clock cycle, or maybe an improper bugfix in
case they are off-sync by 1 or more cycles).

**Wait(time)**

Wait until "timer=time", whereas "timer" runs at 1MHz (NES) or 1.024MHz (SNES).
The "time" values are showing the <completion> of the I/O opcodes (ie.
the I/O opcodes <begin> at "time-1").

**Shutdown (should never happen, unless cartridge is missing or wrong region)**

```
  a=0, if nes then time=830142, else if snes then time=1037682
 endless_loop:          ;timings here aren't 100.000% accurate
  if nes_3195 then time=xlat[P1/4]*174785  ;whereas, xlat[0..3]=(3,2,4,5)
  if (console=0) and (snes or nes_6113) then P0=03h, P1=01h
  if (console=1) then P1=a, Wait(timer+time), a=a xor 4  ;toggle reset on/off
  goto endless_loop
```

### SNES Cartridge CIC Instruction Set

**CIC Registers**

```
  A  4bit Accumulator
  X  4bit General Purpose Register
  L  4bit Pointer Register (lower 4bit of 6bit HL)
  H  2bit Pointer Register (upper 2bit of 6bit HL)
  C  1bit Carry Flag (changed ONLY by "set/clr c", not by "add/adc" or so)
  PC 10bit Program Counter (3bit bank, plus 7bit polynomial counter)
```

**CIC Memory**

```
  ROM   512x8bit (program ROM) (NES/EUR=768x8) (max 1024x8 addressable)
  RAM   32x4bit  (data RAM) (max 64x4 addressable)
  STACK 4x10bit  (stack for call/ret opcodes)
  PORTS 4x4bit   (external I/O ports & internal RAM-like ports) (max 16x4)
```

**Newer CIC Opcodes (6113, D411) (and probably F411,D413,F413)**

```
  00      nop             no operation (aka "addsk A,0" opcode)
  00+n    addsk  A,n      add, A=A+n, skip if result>0Fh
  10+n    cmpsk  A,n      compare, skip if A=n
  20+n    mov    L,n      set L=n
  30+n    mov    A,n      set A=n
  40      mov    A,[HL]   set A=RAM[HL]
  41      xchg   A,[HL]   exchange A <--> RAM[HL]
  42      xchgsk A,[HL+]  exchange A <--> RAM[HL], L=L+1, skip if result>0Fh
  43      xchgsk A,[HL-]  exchange A <--> RAM[HL], L=L-1, skip if result<00h
  44      neg    A        negate, A=0-A                 ;(used by 6113 mode)
  45      ?
  46      out    [L],A    output, PORT[L]=A
  47      out    [L],0    output, PORT[L]=0
  48      set    C        set carry, C=1
  49      clr    C        reset carry, C=0
  4A      mov    [HL],A   set RAM[HL]=A
  4B      ?
  4C      ret             return, pop PC from stack
  4D      retsk           return, pop PC from stack, skip
  4E+n    ?
  52      movsk  A,[HL+]  set A=RAM[HL], L=L+1, skip if result>0Fh
  53      ?                    (guess: movsk  A,[HL-])
  54      not    A        complement, A=A XOR 0Fh
  55      in     A,[L]    input, A=PORT[L]
  56      ?
  57      xchg   A,L      exchange A <--> L
  58+n    ?
  5C      mov    X,A      set X=A
  5D      xchg   X,A      exchange X <--> A
  5E      ???             "SPECIAL MYSTERY INSTRUCTION" ;(used by 6113 mode)
  5F      ?
  60+n    testsk [HL].n   skip if RAM[HL].Bit(n)=1
  64+n    testsk A.n      skip if A.Bit(n)=1
  68+n    clr    [HL].n   set RAM[HL].Bit(n)=0
  6C+n    set    [HL].n   set RAM[HL].Bit(n)=1
  70      add    A,[HL]   add, A=A+RAM[HL]
  71      ?                    (guess: addsk  A,[HL])
  72      adc    A,[HL]   add with carry, A=A+RAM[HL]+C
  73      adcsk  A,[HL]   add with carry, A=A+RAM[HL]+C, skip if result>0Fh
  74+n    mov    H,n      set H=n  ;2bit range, n=0..3 only (used: 0..1 only)
  78+n mm jmp    nmm      long jump, PC=nmm
  7C+n mm call   nmm      long call, push PC+2, PC=nmm
  80+nn   jmp    nn       short jump, PC=(PC AND 380h)+nn
  -       reset           PC=000h
```

Note: "skip" means "do not execute next instruction"

**Older CIC Opcodes (3195) (and probably 3193,3196,3197,etc.)**

```
  Exchanged opcodes 48 <--> 49 (set/clr C)
  Exchanged opcodes 44 <--> 54 (neg/not A)
  ROM Size is 768x8 (although only 512x8 are actually used)
```

**Note**

The CIC is a 4bit Sharp CPU (maybe a Sharp SM4, but no datasheet exists) (the
instruction seems to be an older version of that in the Sharp SM5K1..SM5K7
datasheets).

### SNES Cartridge CIC Notes

**Program Counter (PC)**

The 10bit PC register consists of a 3bit bank (which gets changed only by
call/jmp/ret opcodes), and a 7bit polynomial counter (ie. not a linear
counter). After fetching opcode bytes, PC is "incremented" as so:

```
  PC = (PC AND 380h) + (PC.Bit0 XOR PC.Bit1)*40h + (PC AND 7Eh)/2
```

Ie. the lower 7bit will "increment" through 127 different values (and wrap to
00h thereafter). Address 7Fh is unused (unless one issues a JMP 7Fh opcode,
which would cause the CPU to hang on that address).

```
  Format     <------------- Valid Address Area ---------->      <--Stuck-->
  Linear     00 01 02 03 04 05 06 07 08 09 0A ... 7C 7D 7E  or  7F 7F 7F 7F
  Polynomial 00 40 60 70 78 7C 7E 3F 5F 6F 77 ... 05 02 01  or  7F 7F 7F 7F
```

To simplify things, programming tools like assemblers/disassemblers may use
"normal" linear addresses (and translate linear/polynomial addressses when
needed - the polynomial addresses are relevant only for encoding bits in
jmp/call opcodes, and for how the data is physically arranged in the chip ROMs
and in ROM-images).

**ROM-Images**

The existing ROM-images are .txt files, containing "0" and "1" BITS in ASCII
format, arranged as a 64x64 (or 96x64) matrix (as seen in decapped chips).

```
  Line 1..32   --->   Address X+9Fh..80h            ;\Lines (Y)
  Line 33..64  --->   Address X+1Fh..00h            ;/
  Column  1+(n*W) --> Data Bit(n) of Address 000h+Y ;\  ;\
  Column  2+(n*W) --> Data Bit(n) of Address 020h+Y ;   ; Columns (X)
  Column  3+(n*W) --> Data Bit(n) of Address 040h+Y ;   ;
  Column  4+(n*W) --> Data Bit(n) of Address 060h+Y ;   ; chips with 200h-byte
  Column  5+(n*W) --> Data Bit(n) of Address 100h+Y ;   ; (W=8) (64x64 bits)
  Column  6+(n*W) --> Data Bit(n) of Address 120h+Y ;   ;
  Column  7+(n*W) --> Data Bit(n) of Address 140h+Y ;   ;
  Column  8+(n*W) --> Data Bit(n) of Address 160h+Y ;   ;/
  Column  9+(n*W) --> Data Bit(n) of Address 200h+Y ;
  Column 10+(n*W) --> Data Bit(n) of Address 220h+Y ;  chips with 300h-byte
  Column 11+(n*W) --> Data Bit(n) of Address 240h+Y ;  (W=12) (96x64 bits)
  Column 12+(n*W) --> Data Bit(n) of Address 260h+Y ;/
```

Cautions: The bits are inverted (0=1, 1=0) in some (not all) dumps. Mind that
the bytes are arranged in non-linear polynomial fashion (see PC register).
Recommended format for binary ROM-images would be to undo the inversion (if
present), and to maintain the polynomial byte-order.

Note: Known decapped/dumped CICs are D411 and 3195A, and... somebody
decapped/dumped a CIC without writing down its part number (probably=6113).

**CIC Timings**

The NES CICs are driven by a 4.000MHz CIC oscillator (located in the console,
and divided by 4 in the NES CIC). The SNES CICs are driven by the 24.576MHz APU
oscillator (located and divided by 8 in the console's audio circuit, and
further divided by 3 in the SNES CIC) (exception are older SNES mainboards,
which are having a separate 4.00MHz resonator, like the NES).

Ie. internally, the CICs are clocked at 1.000MHz (NES) or 1.024MHz (SNES). All
opcodes are executed within 1 clock cycles, except for the 2-byte long jumps
(opcodes 78h-7Fh) which take 2 clock cycles. The "skip" opcodes are forcing the
following opcode to be executed as a "nop" (ie. the skipped opcode still takes
1 clock cycle; or possibly 2 cycles when skipping long jump opcodes, in case
the CPU supports skipping 2-byte opcodes at all).

After Reset gets released, the CICs execute the first opcode after a short
delay (3195A: randomly 1.0 or 1.25 cycles, D413A: constantly 1.33 cycles)
(whereas, portions of that delay may rely on a poorly falling edge of the
incoming Reset signal).

**CIC Ports**

```
  Name  Pin  Dir  Expl
  P0.0  1    Out  Data Out    ;\SNES version occassionally swaps these
  P0.1  2    In   Data In     ;/pins by software (ie. Pin1=In, Pin2=Out)
  P0.2  3    In   Random Seed (0=Charged/Ready, 1=Charging/Busy)
  P0.3  4    In   Lock/Key    (0=Cartridge/Key, 1=Console/Lock)
  P1.0  9    Out  Reset SNES  (0=Reset Console, 1=No)
  P1.1  10   Out  Reset Key   (0=No, 1=Reset Key)
  P1.2  11   In   Unused, or Reset Speed A (in 3195A) ;\blink speed of reset
  P1.3  12   In   Unused, or Reset Speed B (in 3195A) ;/signal (and Power LED)
  P2.0  13   -    Unused
  P2.1  14   -    Unused
  P2.2  15   -    Unused
  P2.3  -    -    Unused
  P3.0  -    RAM  Unused, or used as "noswap" flag (in SNES CIC)
  P3.1  -    -    Unused
  P3.2  -    -    Unused
  P3.3  -    -    Unused
```

P0.0-P2.2 are 11 external I/O lines (probably all bidirectional, above
directions just indicates how they are normally used). P2.3-P3.3 are 5 internal
bits (which seem to be useable as "RAM"). Pin numbers are for 16pin NES/SNES
DIP chips (Pin numbers on 18pin SNES SMD chips are slightly rearranged).
P4,P5,P6,P7,P8,P9,PA,PB,PC,PD,PF are unknown/unused (maybe 12x4 further bits,
or mirrors of P0..P3).

**CIC Stream Seeds**

There are different seeds used for different regions. And, confusingly, there
is a NES-CIC clone made Tengen, which uses different seeds than the real CIC
(some of the differences automatically compensated when summing up values, eg.
8+8 gives same 4bit result as 0+0, other differences are manually adjusted by
Tengen's program code).

Many of the reverse-engineered NES seeds found in the internet are based on the
Tengen design (the USA-seeds extracted from the decapped Tengen chip, the
EUR/ASIA/UK-seeds based on sampled Nintendo-CIC data-streams, and then
converted to a Tengen-compatible seed format). To convert them to real CIC
seeds:

```
  Nintendo[1..F] = Tengen[1..F] - (2,0,0,0,0,0,8,8,8,8,8,8,8,8,2)
```

There are other (working) variations possible, for example:

```
  Nintendo[1..F] = Tengen[1..F] - (2,0,0,0,0,A,E,8,8,8,8,8,8,8,2)
  (That, for Tengen-USA seeds. The Tengen-style-EUR/ASIA/UK seeds may differ)
```

Whereas, the random seed in TengenKEY[1] is meant to be "r+2" (so subtracting 2
restores "r").

**CIC Stream Logs**

There are some stream logs with filename "XXXX-N.b" where XXXX is the chip
name, and N is the random seed, and bytes in the file are as so:

```
  Byte 000h, bit0-7 = 1st-8th bit on Pin 1 (DTA.OUT on NES)(DTA.OUT/IN on SNES)
  Byte 001h, bit0-7 = 1st-8th bit on Pin 2 (DTA.IN on NES) (DTA.IN/OUT on SNES)
  Byte 002h, bit0-7 = 9th-16th bit on Pin 1
  Byte 003h, bit0-7 = 9th-16th bit on Pin 2
  etc.
```

Caution: The "N" in the filename is taken as if the seed were transferred in
order Bit 3,2,1,0 (actually it is Bit 3,0,1,2). Ie. file "3195-1.b" would refer
to a NES-EUR-CIC with seed r=4. The signals in the files are sampled at 1MHz
(ie. only each fourth 4MHz cycle).

**The 6113 Chip**

The 6113 chip was invented in 1987, and it replaced the 3193 chip in
US/Canadian cartridges (while US/Canadian consoles kept using 3193 chips). When
used in cartridges, the 6113 does usually "emulate" a 3193 chip. But, for
whatever reason, it can do more:

```
  Console  Cartridge  Notes
  3193     3193       Works (the "old" way)        ;\used combinations
  3193     6113       Works (the "new" way)        ;/
  6113     6113       Works (special seed/timing)  ;\
  6113     3193       Doesn't work                 ; not used as far as known
  6113     ??         Might work (??=unknown chip) ;/
```

When used in consoles, the 6113 uses slightly different timings and seed values
(and does request cartridges with 6113 chips to use the 6113-mode, too, rather
than emulating the 3193).

One guess: Maybe Nintendo originally used different CICs for NTSC regions (like
3193/3194 for USA/Canada/SouthKorea), and later combined them to one region (if
so, all NES consoles in Canada or SouthKorea should contain 3194/6113 chips,
unlike US consoles which have 3193 chips).

**3195A Signals (NES, Europe)**

The I/O ports are HIGH (for Output "1"), or LOW-Z (for Output "0" or Input).
Raising edges take circa 0.5us, falling edges take circa 3us.

```
  4MHz Clock Units     ...............................
  1MHz Clock Units       .   .   .   .   .   .   .   .
                          ___________                  ;\Console+Cartridge
  Data Should-be       __|           |________________ ;/should be 3us High
                           __________                  ;\actually 2.5us High
  Data From Console    __.'          ''----.......____ ;/and 3us falling
                           __________                  ;\
  Data From Cartridge  __.'          ''----.......____ ; either same as console
    or, delayed:            __________                 ; or 0.25us later
  Data From Cartridge  ___.'          ''----.......___ ;/
```

After Power-up, the Cartridge CIC does randomly start with good timing, or with
all signals delayed by 0.25us. In other words, the Cartridge CIC executes the
first opcode 1.0us or 1.25us (four or five 4MHz cycles) after Reset gets
released. However, for some reason, pushing the Reset Button doesn't alter the
timing, the random-decision occurs only on Power-up.

**D413A Signals (SNES, Europe)**

The D413A signals are looking strange. First, the software switches signals
High for 3us, but the actual signals are 3.33us High. Second, the signals on
one pin are constantly jumping back'n'forth by 1.33us (in relation to the other
pin).

```
  3.072MHz Clock Units ...............................
  1.024MHz Clock Units   .  .  .  .  .  .  .  .  .  .
                                ________               ;\Console+Cartridge
  Data Should-be       ________|        |_____________ ;/should be 3us High
                                _________              ;\actually 3.33us high
  Data From/To Console ________|         '--..._______ ;/and 2us falling
                            _________                  ;\
  Data From/To Cart    ____|         '--...___________ ; 1.33us earlier
   or, delayed                      _________          ; or 1.33us later
  Data From/To Cart    ____________|         '--...___ ;/
```

The earlier/later effect occurs because the SNES CICs are occassionally
reversing the data-direction of the pins. Ie. in practice, Data from Cartridge
is constantly 1.33us LATER than from Console.

Software-wise, the D411 (and probably D413A) is programmed as if the Cartridge
CIC would start "immediately", but in practice, it starts 1.33us (four 3.072MHz
cycles) after releasing Reset (that offset seems to be constant, unlike as on
the 3195A where it randomly changes between 1.0us and 1.25us).

### SNES Cartridge CIC Versions

**NES CIC Versions**

```
  3193,3193A        NES NTSC Cartridges and Consoles       ;\USA,Canada
  6113,6113A,6113B1 NES NTSC Cartridges (not consoles)     ;/(and Korea?)
  3194              Unknown/doesn't exist?
  3195,3193A        NES PAL Cartridges and Consoles "PAL-B";-Europe
  3196(A?)          NES PAL Cartridges and Consoles        ;-Hong Kong,Asia
  3197(A?)          NES PAL Cartridges and Consoles "PAL-A";-UK,Italy,Australia
  3198(A?)          FamicomBox CIC Cartridges and Consoles ;\
  3199(A?)          FamicomBox Coin Timer (not a CIC)      ; Japan
  N/A               Famicom Cartridges and Consoles        ;/
  RFC-CPU10 (?)     NES R.O.B. robot (no CIC, but maybe a 4bit Sharp CPU, too?)
```

**SNES CIC Versions**

```
  F411,F411A,F411B   SNES NTSC Cartridges-with-SMD-Chipset and Consoles
  D411,D411A,D411B   SNES NTSC Cartridges-with-DIP-Chipset
  F413,F413A,F413B   SNES PAL Cartridges-with-SMD-Chipset and Consoles
  D413,D413A,D413B   SNES PAL Cartridges-with-DIP-Chipset
  SA-1,S-DD1,MCC-BSC SNES Cartridges (coprocessors/mappers with on-chip CIC)
```

**NES CIC Clones**

```
  23C1033
  337002   ;Tengen's 16pin "Rabbit" CIC clone
  337006   ;Tengen's 40pin "RAMBO-1" mapper with built-in CIC clone
  4051
  7660
  KC5373B
  MX8018
  NINA
  Ciclone  ;homebrew multi-region CIC clone (based on Tengen design)
```

Aside from using cloned CICs, many unlicensed NES cartridges used a different
approach: injecting "wrong" voltages to the console, and "stunning" its CIC.

**SNES CIC Clones**

```
  10198    - CIC clone
  noname   - CIC clone (black chip without any part number)
  ST10198S - NTSC CIC clone
  ST10198P - PAL CIC clone
  265111   - maybe also a CIC clone (used in Bung Game Doctor SF6)
  D1       - maybe also a CIC clone (used in Super UFO Pro8)
  74LS112  - reportedly also a CIC clone (with fake part number) (UFO Pro6)
  CIVIC 74LS13   16pin - CIC/D411 clone (used in a 8-in-1 pirate cart)
  CIVIC CT6911   16pin - CIC      clone (used in a 7-in-1 pirate cart)
  93C26          16pin - CIC      clone (used in a 8-in-1 pirate cart)
  D1             16pin - CIC? (used in Super VG pirate)
  STS9311A 52583 16pin - CIC clone (used in Donkey King Country 3 pirate)
  black blob     16pin - CIC/D411 clone (used in Sonic the Hedgehog pirate)
```

**CIC Chip Year/Week Date Codes**

```
  Name   YYWW-YYWW
  3193   8539-8642
  3193A  8547-8733 (in cartridges) (but should be in consoles for more years)
  3195   8627-8638
  3195A  8647-9512
  3197A  8647-9227
  6113   8734-8823
  6113A  8823-8933
  6113B1 8847-9344
```

### SNES Cart LoROM Mapping (ROM divided into 32K banks) (around 1500 games)

**Plain LoROM**

```
  Board Type               ROM Area               ROM Mirrors
  SHVC-1A0N-01,02,10,20,30 00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
  SHVC-2A0N-01,10,11,20    00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
  SHVC-BA0N-01,10          00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
  SHVC-YA0N-01             00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
```

**LoROM with SRAM**

```
  Board Type               ROM Area               SRAM Area
  SHVC-1A1B-04,05,06       00-1F,80-9F:8000-FFFF  70-7D,F0-FF:0000-FFFF
  SHVC-1A3B-11,12,13       00-1F,80-9F:8000-FFFF  70-7D,F0-FF:0000-FFFF
  SHVC-1A5B-02,04          00-1F,80-9F:8000-FFFF  70-7D,F0-FF:0000-FFFF
  SHVC-2A3B-01             00-3F,80-BF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A3M-01 with MAD-R  00-3F,80-BF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A3M-01,11,20       00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A3B-20             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A1M-01,11,20       00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A1M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-BA1M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A3M-10,20,21,30    00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-BA3M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A5M-01,11,20       00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A5M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A7M-01             ?                      ?
```

Note that 2A3M-01 exists with/without MAD-R (and have different mappings).

Note that 1A3B-20 differs from earlier 1A3B-xx versions.

The older boards map SRAM to the whole 64K areas at banks 70h-7Dh/F0-FFh.

The newer boards map SRAM to the lower 32K areas at banks 70h-7Dh/F0-FFh (this
allows "BigLoROM" games to use the upper 32K of that banks as additional LoROM
banks, which is required for games with more than 3MB LoROM).

Most of the existing boards contain 0K, 2K, 8K, or 32K SRAM. A few games
contail 64K or 128K SRAM, which is divided into 32K chunks, mapped to bank 70h,
71h, etc.)

Some LoROM games are bigger than 2Mbytes (eg. Super Metroid, Gunple, Wizardry
6, Derby Stallion 3), these have bank 0-3Fh mapped in the 32K LoROM banks as
usually, and bank 40h and up each mapped twice in the 64K hirom banks.

Note: There's also a different "SpecialLoROM" mapping scheme for 3MByte ROMs
(used by Derby Stallion 96 and Sound Novel Tsukuru; aside from the special ROM
mapping, these cartridges have an additional Data Pack Slot).

### SNES Cart HiROM Mapping (ROM divided into 64K banks) (around 500 games)

**Plain HiROM**

```
  Board               ROM Area      ROM Mirrors   SRAM Area
  Type                at 0000-FFFF  at 8000-FFFF  (none such)
  SHVC-BJ0N-01,20     40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-YJ0N-01        40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-1J0N-01,10,20  40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-2J0N-01,10,11  40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-3J0N-01        40-6f,c0-ef   00-2f,80-af   N/A
```

The SHVC-3J0N-01 board contains 3 ROM chips (memory is divided into chunks of
16 banks, with one ROM per chunk, and with each 4th chunk being left empty, ie.
bank 30-3F,70-7D,B0-BF,F0-FF are open-bus).

**HiROM with SRAM**

```
  Board               ROM Area      ROM Mirrors   SRAM Area
  Type                at 0000-FFFF  at 8000-FFFF  at 6000-7FFF
  SHVC-1J3B-01        40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-1J1M-11,20     40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-1J3M-01,11,20  40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-BJ3M-10        40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-1J5M-11,20     40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-2J3M-01,11,20  40-7d,c0-ff   00-3f,80-bf   10-1f,30-3f,90-9f,b0-bf
  SHVC-2J5M-01        40-7d,c0-ff   00-3f,80-bf   10-1f,90-9f,30-3f,b0-bf
  SHVC-LJ3M-01        40-7d,c0-ff   00-3f,80-bf   80-bf
```

The SHVC-LJ3M-01 board uses ExHiROM mapping (meaning that bank 00h-7Dh contain
different ROM banks than 80h-FFh).

### SNES Cart SA-1 (programmable 65C816 CPU) (aka Super Accelerator) (35 games)

**Memory Map (SNES Side)**

```
  00h-3Fh/80h-BFh:2200h-23FFh  I/O Ports
  00h-3Fh/80h-BFh:3000h-37FFh  I-RAM (2Kbytes, on-chip, 10MHz fast RAM)
  00h-3Fh/80h-BFh:6000h-7FFFh  One mappable 8Kbyte BW-RAM block
  00h-3Fh/80h-BFh:8000h-FFFFh  Four mappable 1MByte LoROM blocks (max 8Mbyte)
  40h-4Fh:0000h-FFFFh          Entire 256Kbyte BW-RAM (mirrors in 44h-4Fh)
  C0h-FFh:0000h-FFFFh          Four mappable 1MByte HiROM blocks (max 8Mbyte)
```

The SA-1 supports both LoROM and HiROM mappings (eg. LoROM banks 00h-01h mirror
to HiROM bank 40h). Default exception vectors (and cartridge header) are always
in LoROM bank 00h (ie. at ROM offset 7Fxxh).

**Memory Map (SA-1 Side)**

Same as on SNES Side (of course without access to SNES internal WRAM and I/O
ports), plus following additional areas:

```
  00h-3Fh/80h-BFh:0000h-07FFh  I-RAM (at both 0000h-07FFh and 3000h-37FFh)
  60h-6Fh:0000h-FFFFh          BW-RAM mapped as 2bit or 4bit pixel buffer
```

Some other differences to SNES Side are: I/O Ports are different, on SA-1 side,
the mappable BW-RAM area (at 6000h-7FFFh) can be also assigned as 2bit/4bit
pixel buffer (on SNES Side it's always normal 8bit memory).

**Misc**

65C816 CPU at 10.74MHz

```
  2Kbytes internal I-RAM (work ram/stack) (optionally battery backed)
  Optional external backup/work BW-RAM up to 2MByte (or rather only 2Mbit?)
  Addressable ROM up to 8MByte (64MBits)
```

The SA-1 CPU can access memory at 10.74MHz rate (or less, if the SNES does
simultaneouly access cartridge memory).

The SNES CPU can access memory at 2.68MHz rate (or 3.5MHz, but that mode may
not be used in combination with the SA-1).

When interrupts are disabled (in CIE/SIE), then it sounds as if the interrupt
flags still do get set?

"BW-RAM cannot be used during character conversion DMA."

IRQ/NMI/Reset vectors can be mapped. Other vectors (BRK/COP etc) are always
taken from ROM (for BOTH CPUs).

```
    XXX pg 62..66 timings
 ok XXX pg 67..78 char/bitmap
 ok XXX pg 79..81 arit
    XXX pg 82..86 var-len
 ok XXX pg 87..90 dma
```

**SA-1 Pinouts**

```
  1-126  Unknown
  127    PAL/NTSC (for CIC mode and/or HV-timer?)
  128    Unknown
```

**SA-1 PCBs**

```
  BSC-1L3B-01    NTSC SRAM Battery FLASH-Slot (Itoi Shig. no Bass Tsuri No.1)
  SHVC-1L0N3S-20 NTSC SRAM NoBattery (Dragon Ball Z Hyper Dimension)
  SHVC-1L3B-11   NTSC SRAM Battery
  SHVC-1L5B-10   NTSC SRAM Battery
  SHVC-1L5B-11   NTSC SRAM Battery
  SHVC-1L8B-10   NTSC SRAM Battery
  SNSP-1L0N3S-01 PAL  SRAM NoBattery (Dragon Ball Z Hyper Dimension)
  SNSP-1L3B-20   PAL  SRAM Battery
```

The battery can be wired to I-RAM (on-chip SA-1 memory) or BW-RAM (aka SRAM) or
both; unknown how it is wired in practice (probably to BW-RAM?).

**Chipset/Components**

```
  U1  44pin  ROM (probably with full 16bit databus connected)
  U2  28pin  SRAM (LH52A64N-YL or LH52256ANZ or 32pin LH52A512NF)
  U3  128pin SA1 (SA1 RF5A123)
  U4  8pin   Battery controller MM1026AF  ;\only if PCB does include a battery
  BATT 2pin  CR2032                       ;/
  CN1 62pin  SNES cartridge edge-connector
  CN2 62pin  Satellaview FLASH cartridge slot  ;-only on BSC-boards
```

### SNES Cart SA-1 Games

SA1     - 128pin - Super Accelerator (book2) (10.74MHz 65C816 CPU)

Used by 35 games:

```
 #Asahi Shinbun Rensai Kato Ichi-Ni-San Kudan Shogi Shingiru (1995) Varie (JP)
  Daisenryaku Expert WWII: War in Europe (1996) SystemSoftAlpha/ASCII Corp (JP)
  Derby Jockey 2 (1995) Muse Soft/Asmik (JP)
  Dragon Ball Z: Hyper Dimension (1996) TOSE/Bandai (JP) (EU)
 #Habu Meijin no Omoshiro Syouhi -Unverified (19xx) Hiroshi/etc. (JP)
  Itoi Shigesato no Bass Tsuri No. 1 (1997) HAL Laboratory/Nintendo (JP)
  J. League '96 Dream Stadium (1996) Hudson Soft (JP)
  Jikkyou Oshaberi Parodius (1995) Konami (JP)
  Jumpin' Derby (1996) Naxat Soft (JP)
 #Kakinoki Shogi (1995) ASCII Corporation (JP)
  Kirby Super Star (1996) HAL Laboratory/Nintendo (NA) (JP) (EU)
  Kirby's Dream Land 3 (1997) HAL Laboratory/Nintendo (NA) (JP)
  Marvelous: Mouhitotsu no Takarajima (1996) Nintendo/R&D2 (JP)
  Masoukishin: Super Robot Wars Gaiden: Lord of Elemental (19xx) Banpresto (JP)
  Masters New: Haruka Naru Augusta 3 (1995) T&E Soft (JP)
  Mini Yonku/4WD Shining Scorpion - Let's & Go!! (1996) KID/ASCII Corp (JP)
  Pachi Slot Monogatari PAL Kogyo Special -Unverified (1995) PAL/KSS (JP)
  Pebble Beach no Hotou: New Tournament Edition (1996) T&E Soft (JP)
  PGA European Tour (1996) Halestorm/THQ/Black Pearl Software (NA)
  PGA Tour '96 (1995) Black Pearl Software/Electronic Arts (NA)
  Power Rangers Zeo: Battle Racers (1996) Natsume/Bandai (NA)
 #Pro Kishi Simulation Kishi No Hanamichi (1996) Atlus (JP)
 xRin Kaihou 9 Dan No Igo Taidou -Unverified (1996) .. (JP)
  SD F-1 Grand Prix (and "Sample" version) (1995) Video System (JP)
  SD Gundam G NEXT (1995) BEC/Bandai (JP)
 #Shin Shogi/Syogi Club (1995) Hect/Natsu (JP)
 #Shogi Saikyou (1995) Magical Company (JP) (unverified?)
 #Shogi Saikyou 2 (1996) Magical Company (JP)
 #Shougi Mahjong (1995) Varie Corp (JP)
  Super Bomberman Panic Bomber World (1995) Hudson Soft (JP)
  Super Mario RPG: Legend of the Seven Stars (1996) Square/Nintendo (NA) (JP)
  Super Robot T.G.: The Lord Of Elemental (?) (1996) Winkysoft/Banpresto (JP)
 #Super Shogi 3 Kitaihei -Unverified (1995) I'Max (JP)
 xTaikyoku-Igo Idaten -Unverified (1995) BPS (JP)
 xTakemiya Masaki Kudan No Igo Taisyou -Unverified (1995) KSS (JP)
```

The nine Shogi/Shougi/Syouhi/Kishi/Syogi titles are japanese Chess games, the
three Igo titles are Go games; that 12 titles are mainly using the SA-1 CPU for
calculating moves, without doing any impressive things with the SA-1 I/O ports.

### SNES Cart SA-1 I/O Map

**SA-1 I/O Map (Write Only Registers)**

```
  Port  Side  Name  Reset Expl.
  2200h SNES  CCNT  20h   SA-1 CPU Control (W)
  2201h SNES  SIE   00h   SNES CPU Int Enable (W)
  2202h SNES  SIC   00h   SNES CPU Int Clear  (W)
  2203h SNES  CRV   -     SA-1 CPU Reset Vector Lsb (W)
  2204h SNES  CRV   -     SA-1 CPU Reset Vector Msb (W)
  2205h SNES  CNV   -     SA-1 CPU NMI Vector Lsb (W)
  2206h SNES  CNV   -     SA-1 CPU NMI Vector Msb (W)
  2207h SNES  CIV   -     SA-1 CPU IRQ Vector Lsb (W)
  2208h SNES  CIV   -     SA-1 CPU IRQ Vector Msb (W)
  2209h SA-1  SCNT  00h   SNES CPU Control (W)
  220Ah SA-1  CIE   00h   SA-1 CPU Int Enable (W)
  220Bh SA-1  CIC   00h   SA-1 CPU Int Clear  (W)
  220Ch SA-1  SNV   -     SNES CPU NMI Vector Lsb (W)
  220Dh SA-1  SNV   -     SNES CPU NMI Vector Msb (W)
  220Eh SA-1  SIV   -     SNES CPU IRQ Vector Lsb (W)
  220Fh SA-1  SIV   -     SNES CPU IRQ Vector Msb (W)
  2210h SA-1  TMC   00h   H/V Timer Control (W)
  2211h SA-1  CTR   -     SA-1 CPU Timer Restart (W)
  2212h SA-1  HCNT  -     Set H-Count Lsb (W)
  2213h SA-1  HCNT  -     Set H-Count Msb (W)
  2214h SA-1  VCNT  -     Set V-Count Lsb (W)
  2215h SA-1  VCNT  -     Set V-Count Msb (W)
  2216h -     -     -     -
  2220h SNES  CXB   00h   MMC Bank C - Hirom C0h-CFh / LoRom 00h-1Fh (W)
  2221h SNES  DXB   01h   MMC Bank D - Hirom D0h-DFh / LoRom 20h-3Fh (W)
  2222h SNES  EXB   02h   MMC Bank E - Hirom E0h-EFh / LoRom 80h-9Fh (W)
  2223h SNES  FXB   03h   MMC Bank F - Hirom F0h-FFh / LoRom A0h-BFh (W)
  2224h SNES  BMAPS 00h   SNES CPU BW-RAM Mapping to 6000h-7FFFh (W)
  2225h SA-1  BMAP  00h   SA-1 CPU BW-RAM Mapping to 6000h-7FFFh (W)
  2226h SNES  SBWE  00h   SNES CPU BW-RAM Write Enable (W)
  2227h SA-1  CBWE  00h   SA-1 CPU BW-RAM Write Enable (W)
  2228h SNES  BWPA  FFh   BW-RAM Write-Protected Area (W)
  2229h SNES  SIWP  00h   SNES I-RAM Write-Protection (W)
  222Ah SA-1  CIWP  00h   SA-1 I-RAM Write-Protection (W)
  222Bh -     -     -     -
  2230h SA-1  DCNT  00h   DMA Control (W)
  2231h Both  CDMA  00h   Character Conversion DMA Parameters (W)
  2232h Both  SDA   -     DMA Source Device Start Address Lsb (W)
  2233h Both  SDA   -     DMA Source Device Start Address Mid (W)
  2234h Both  SDA   -     DMA Source Device Start Address Msb (W)
  2235h Both  DDA   -     DMA Dest Device Start Address Lsb (W)
  2236h Both  DDA   -     DMA Dest Device Start Address Mid (Start/I-RAM) (W)
  2237h Both  DDA   -     DMA Dest Device Start Address Msb (Start/BW-RAM)(W)
  2238h SA-1  DTC   -     DMA Terminal Counter Lsb (W)
  2239h SA-1  DTC   -     DMA Terminal Counter Msb (W)
  223Ah -     -     -     -
  223Fh SA-1  BBF   00h   BW-RAM Bit Map Format for 600000h-6FFFFFh (W)
  224xh SA-1  BRF   -     Bit Map Register File (2240h..224Fh) (W)
  2250h SA-1  MCNT  00h   Arithmetic Control (W)
  2251h SA-1  MA    -     Arithmetic Param A Lsb (Multiplicand/Dividend) (W)
  2252h SA-1  MA    -     Arithmetic Param A Msb (Multiplicand/Dividend) (W)
  2253h SA-1  MB    -     Arithmetic Param B Lsb (Multiplier/Divisor) (W)
  2254h SA-1  MB    -     Arithmetic Param B Msb (Multiplier/Divisor)/Start (W)
  2255h -     -     -     -
  2258h SA-1  VBD   -     Variable-Length Bit Processing (W)
  2259h SA-1  VDA   -     Var-Length Bit Game Pak ROM Start Address Lsb (W)
  225Ah SA-1  VDA   -     Var-Length Bit Game Pak ROM Start Address Mid (W)
  225Bh SA-1  VDA   -     Var-Length Bit Game Pak ROM Start Address Msb & Kick
  225Ch -     -     -     -
  2261h -     -     -     Unknown/Undocumented (Jumpin Derby writes 00h)
  2262h -     -     -     Unknown/Undocumented (Super Bomberman writes 00h)
```

**SA-1 I/O Map (Read Only Registers)**

```
  Port  Side  Name  Reset Expl.
  2300h SNES  SFR   SNES CPU Flag Read (R)
  2301h SA-1  CFR   SA-1 CPU Flag Read (R)
  2302h SA-1  HCR   H-Count Read Lsb / Do Latching (R)
  2303h SA-1  HCR   H-Count Read Msb (R)
  2304h SA-1  VCR   V-Count Read Lsb (R)
  2305h SA-1  VCR   V-Count Read Msb (R)
  2306h SA-1  MR    Arithmetic Result, bit0-7   (Sum/Product/Quotient) (R)
  2307h SA-1  MR    Arithmetic Result, bit8-15  (Sum/Product/Quotient) (R)
  2308h SA-1  MR    Arithmetic Result, bit16-23 (Sum/Product/Remainder) (R)
  2309h SA-1  MR    Arithmetic Result, bit24-31 (Sum/Product/Remainder) (R)
  230Ah SA-1  MR    Arithmetic Result, bit32-39 (Sum) (R)
  230Bh SA-1  OF    Arithmetic Overflow Flag (R)
  230Ch SA-1  VDP   Variable-Length Data Read Port Lsb (R)
  230Dh SA-1  VDP   Variable-Length Data Read Port Msb (R)
  230Eh SNES  VC    Version Code Register (R)
```

**Reset**

Port 2200h = 20h. Port 2228h = FFh. Ports 2220h-2223h = 00h,01h,02h,03h. Ports
2201h-2202h, 2209h-220Bh, 2210h, 2224h-2227h, 2229h-222Ah, 2230h-2231h, 223Fh,
2250h = 00h. Ports 2203h-2208h, 220Ch-220Fh, 2211h-2215h, 2232h-2239h,
2240h-224Fh, 2251h-2254h, 2258h-225Bh = N/A.

### SNES Cart SA-1 Interrupt/Control on SNES Side

**2200h SNES CCNT - SA-1 CPU Control (W)**

```
  0-3 Message from SNES to SA-1 (4bit value)
  4   NMI from SNES to SA-1   (0=No Change?, 1=Interrupt)
  5   Reset from SNES to SA-1 (0=No Reset, 1=Reset)
  6   Wait from SNES to SA-1  (0=No Wait, 1=Wait)
  7   IRQ from SNES to SA-1   (0=No Change?, 1=Interrupt)
```

Unknown if Wait freezes the whole SA1 (CPU, plus Timer and DMA?).

Unknown if Reset resets any I/O Ports (such like DMA or interrupts) or if it
does only reset the CPU?

**2201h SNES SIE - SNES CPU Int Enable (W)**

```
  0-4 Not used (should be 0)
  5   IRQ Enable (Character conversion DMA) (0=Disable, 1=Enable)
  6   Not used (should be 0)
  7   IRQ Enable (from SA-1) (0=Disable, 1=Enable)
```

**2202h SNES SIC - SNES CPU Int Clear (W)**

```
  0-4 Not used (should be 0)
  5   IRQ Acknowledge (Character conversion DMA) (0=No change, 1=Clear)
  6   Not used (should be 0)
  7   IRQ Acknowledge (from SA-1) (0=No change, 1=Clear)
```

**2203h SNES CRV - SA-1 CPU Reset Vector Lsb (W)**

**2204h SNES CRV - SA-1 CPU Reset Vector Msb (W)**

**2205h SNES CNV - SA-1 CPU NMI Vector Lsb (W)**

**2206h SNES CNV - SA-1 CPU NMI Vector Msb (W)**

**2207h SNES CIV - SA-1 CPU IRQ Vector Lsb (W)**

**2208h SNES CIV - SA-1 CPU IRQ Vector Msb (W)**

Exception Vectors on SA-1 side (these are ALWAYS replacing the normal vectors
in ROM).

**2300h SNES SFR - SNES CPU Flag Read (R)**

```
  0-3 Message from SA-1 to SNES (4bit value)          (same as 2209h.Bit0-3)
  4   NMI Vector for SNES (0=ROM FFExh, 1=Port 220Ch) (same as 2209h.Bit4)
  5   IRQ from Character Conversion DMA (0=None, 1=Interrupt) (ready-to-do-DMA)
  6   IRQ Vector for SNES (0=ROM FFExh, 1=Port 220Eh) (same as 2209h.Bit6)
  7   IRQ from SA-1 to SNES   (0=None, 1=Interrupt) (triggered by 2209h.Bit7)
```

Bit0-3,4,6 are same as in Port 2209h. Bit5 is set via ..DMA..? Bit7 is set via
Port 2209h. Bit5,7 can be cleared via Port 2202h.

**230Eh SNES VC - Version Code Register (R)**

```
  0-7  SA-1 Chip Version
```

Existing value(s) are unknown. There seems to be only one chip version (labeled
SA-1 RF5A123, used for both PAL and NTSC). The "VC" register isn't read by any
games (except, accidently, by a bugged memcopy function at 059E92h in Derby
Jockey 2).

### SNES Cart SA-1 Interrupt/Control on SA-1 Side

**2209h SA-1 SCNT - SNES CPU Control (W)**

```
  0-3 Message from SA-1 to SNES (4bit value)
  4   NMI Vector for SNES (0=ROM FFEAh, 1=Port 220Ch)
  5   Not used (should be 0)
  6   IRQ Vector for SNES (0=ROM FFEEh, 1=Port 220Eh)
  7   IRQ from SA-1 to SNES   (0=No Change?, 1=Interrupt)
```

**220Ah SA-1 CIE - SA-1 CPU Int Enable (W)**

```
  0-3 Not used (should be 0)
  4   NMI Enable (from SNES)  (0=Disable, 1=Enable)
  5   IRQ Enable (from DMA)   (0=Disable, 1=Enable)
  6   IRQ Enable (from Timer) (0=Disable, 1=Enable)
  7   IRQ Enable (from SNES)  (0=Disable, 1=Enable)
```

**220Bh SA-1 CIC - SA-1 CPU Int Clear (W)**

```
  0-3 Not used (should be 0)
  4   NMI Acknowledge (from SNES)  (0=No change, 1=Clear)
  5   IRQ Acknowledge (from DMA)   (0=No change, 1=Clear)
  6   IRQ Acknowledge (from Timer) (0=No change, 1=Clear)
  7   IRQ Acknowledge (from SNES)  (0=No change, 1=Clear)
```

**220Ch SA-1 SNV - SNES CPU NMI Vector Lsb (W)**

**220Dh SA-1 SNV - SNES CPU NMI Vector Msb (W)**

**220Eh SA-1 SIV - SNES CPU IRQ Vector Lsb (W)**

**220Fh SA-1 SIV - SNES CPU IRQ Vector Msb (W)**

Exception Vectors on SNES side (these are optionally replacing the normal
vectors in ROM; depending on bits in Port 2209h; the "I/O" vectors are used
only by Jumpin Derby, all other games are using the normal ROM vectors).

**2301h SA-1 CFR - SA-1 CPU Flag Read (R)**

```
  0-3 Message from SNES to SA-1 (4bit value)       (same as 2200h.Bit0-3)
  4   NMI from SNES to SA-1   (0=No, 1=Interrupt)  (triggered by 2200h.Bit4)
  5   IRQ from DMA to SA-1    (0=No, 1=Interrupt)  (triggered by DMA-finished)
  6   IRQ from Timer to SA-1  (0=No, 1=Interrupt)  (triggered by Timer)
  7   IRQ from SNES to SA-1   (0=No, 1=Interrupt)  (triggered by 2200h.Bit7)
```

### SNES Cart SA-1 Timer

**2210h SA-1 TMC - H/V Timer Control (W)**

```
  0   HEN             ;\Enables Interrupt or so ?
  1   VEN             ;/
  2-6 Not used (should be 0)
  7   Timer Mode (0=HV Timer, 1=Linear Timer)
```

**2211h SA-1 CTR - SA-1 CPU Timer Restart (W)**

```
  0-7 Don't care (writing any value restarts the timer at 0)
```

**2212h SA-1 HCNT - Set H-Count Lsb (W)**

**2213h SA-1 HCNT - Set H-Count Msb (W)**

```
  0-8  H-Counter (9bit)
  9-15 Not used (should be 0)
```

Ranges from 0-340 (in HV mode), or 0-511 (in Linear mode).

**2214h SA-1 VCNT - Set V-Count Lsb (W)**

**2215h SA-1 VCNT - Set V-Count Msb (W)**

```
  0-8  V-Counter (9bit)
  9-15 Not used (should be 0)
```

Ranges from 0-261 (in HV/NTSC mode), 0-311 (in HV/PAL mode), or 0-511 (in
Linear mode). The PAL/NTSC selection is probably done by a soldering point on
the PCB (which is probably also used for switching the built-in CIC to PAL/NTSC
mode).

**2302h SA-1 HCR - H-Count Read Lsb / Do Latching (R)**

**2303h SA-1 HCR - H-Count Read Msb (R)**

**2304h SA-1 VCR - V-Count Read Lsb (R)**

**2305h SA-1 VCR - V-Count Read Msb (R)**

Reading from 2302h automatically latches the other HV-Counter bits to
2303h-2305h.

**Notes**

In HV-mode, the timer clock is obviously equivalent to the dotclock (four 21MHz
master cycles per dot). The time clock in linear mode is unknown (probably same
as in HV-mode).

H-counter has 341 dots (one more as in SNES, but without long dots). Unknown if
the short-scanline (in each 2nd NTSC non-interlaced frame) is reproduced (if it
isn't, then one must periodically reset the timer in order to keep it in sync
with the PPU). There is no provision for interlaced video timings.

The meaning of Port 2212h-2215h is totally unknown (according to existing specs
it <sounds> as if they do set the <current> counter value - though
alltogether it'd be more likely that they do contain <compare> values).

Unknown what happens when setting both HEN and VEN (probably IRQ triggers only
if <both> H+V do match, ie. similar as for the normal SNES timers).

### SNES Cart SA-1 Memory Control

**2220h SNES CXB - Set Super MMC Bank C - Hirom C0h-CFh / LoRom 00h-1Fh (W)**

**2221h SNES DXB - Set Super MMC Bank D - Hirom D0h-DFh / LoRom 20h-3Fh (W)**

**2222h SNES EXB - Set Super MMC Bank E - Hirom E0h-EFh / LoRom 80h-9Fh (W)**

**2223h SNES FXB - Set Super MMC Bank F - Hirom F0h-FFh / LoRom A0h-BFh (W)**

```
  0-2  Select 1Mbyte ROM-Bank (0..7)
  3-6  Not used (should be 0)
  7    Map 1Mbyte ROM-Bank (0=To HiRom, 1=To LoRom and HiRom)
```

If LoRom mapping is disabled (bit7=0), then first 2 MByte of ROM are mapped to
00h-3Fh, and next 2 MByte to 80h-BFh. The registers do affect both SNES and
SA-1 mapping.

**2224h SNES BMAPS - SNES CPU BW-RAM Mapping to 6000h-7FFFh (W)**

```
  0-4  Select 8Kbyte BW-RAM Block for mapping to 6000h-7FFFh (0..31)
  5-7  Not used (should be 0)
```

BW-RAM is always mapped to bank 40h-43h (max 256 Kbytes).

This register allows to map an 8Kbyte chunk to offset 6000h-7FFFh in bank 0-3Fh
and 80h-BFh.

**2225h SA-1 BMAP - SA-1 CPU BW-RAM Mapping to 6000h-7FFFh (W)**

```
  0-6  Select 8Kbyte BW-RAM Block for mapping to 6000h-7FFFh (0..31 or 0..127)
  7    Select source (0=Normal/Bank 40h..43h, 1=Bitmap/Bank 60h..6Fh)
```

**223Fh SA-1 BBF - BW-RAM Bit Map Format for 600000h-6FFFFFh (W)**

```
  0-6 Not used (should be "..") (whatever ".." means, maybe "0"?)
  7   Format (0=4bit, 1=2bit)
```

"BW-RAM bitmap logical space format setting from perspective of the SA-1 CPU"

```
  600000h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit0-1 or 400000h.Bit0-3
  600001h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit2-3 or 400000h.Bit4-7
  600002h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit4-5 or 400001h.Bit0-3
  600003h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit6-7 or 400001h.Bit4-7
  etc.
```

Note that the LSBs in the packed-area contain the left-most pixel (not the
right-most one). The MSBs in the unpacked area are "ignored" (this is obvious
in case of writing; for reading it's unknown what it means - are reads
supported at all, and if so, do they return zero's or garbage in MSBs?)

**2226h SNES SBWE - SNES CPU BW-RAM Write Enable (W)**

**2227h SA-1 SBWE - SA-1 CPU BW-RAM Write Enable (W)**

```
  0-6  Not used (should be 0)
  7    Write Enable BW-RAM (0=Protect, 1=Write Enable)
```

**2228h SNES BWPA - BW-RAM Write-Protected Area (W)**

```
  0-3  Select size of Write-Protected Area ("256 SHL N" bytes)
  4-7  Not used (should be 0)
```

Selects how many bytes (originated at 400000h) shall be write protected.

It isn't possible to set the size to "none" (min is 256 bytes), though, one can
probably completely disable the protection via ports 2226h/2227h?

**2229h SNES SIWP - SNES I-RAM Write-Protection (W)**

**222Ah SA-1 CIWP - SA-1 I-RAM Write-Protection (W)**

```
  0-7  Write enable flags for eight 256-byte chunks (0=Protect, 1=Write Enable)
```

Bit0 for I-RAM 3000h..30FFh, bit1 for 3100h..31FFh, etc. bit7 for 3700h..37FFh.

### SNES Cart SA-1 DMA Transfers

**2230h SA-1 DCNT - DMA Control (W)**

```
  0-1 DMA Source Device      (0=ROM, 1=BW-RAM, 2=I-RAM, 3=Reserved);\for
  2   DMA Destination Device (0=I-RAM, 1=BW-RAM)                   ;/Normal DMA
  3   Not used (should be 0)
  4   DMA Char Conversion Type (0=Type 2/Semi-Automatic, 1=Type 1/Automatic)
  5   DMA Char Conversion Enable (0=Normal DMA, 1=Character Conversion DMA)
  6   DMA Priority (0=SA-1 CPU Priority, 1=DMA Priority) ;<-- for Normal DMA
  7   DMA Enable (0=Disable, 1=Enable... and Clear Parameters?)
```

Bit6 is only valid for Normal DMA between BW-RAM and I-RAM. Source and
Destination may not be the same devices (ie. no I-RAM to I-RAM, or BW-RAM to
BW-RAM).

**2231h Both CDMA - Character Conversion DMA Parameters (W)**

```
  0-1 Color Depth (0=8bit, 1=4bit, 2=2bit, 3=Reserved)
  2-4 Virtual VRAM Width (0..5 = 1,2,4,8,16,32 characters) (6..7=Reserved)
  5-6 Not used (should be 0)
  7   Terminate Character Conversion 1 (0=No change, 1=Terminate DMA)
```

**2232h Both SDA - DMA Source Device Start Address Lsb (W)**

**2233h Both SDA - DMA Source Device Start Address Mid (W)**

**2234h Both SDA - DMA Source Device Start Address Msb (W)**

```
  0-23  24bit Memory Address (translated to 23bit ROM Offset via 2220h..2223h)
  0-17  18bit BW-RAM Offset
  0-10  11bit I-RAM Offset
```

Used bits are 24bit/18bit/11bit for ROM/BW-RAM/I-RAM.

**2235h Both DDA - DMA Destination Device Start Address Lsb (W)**

**2236h Both DDA - DMA Destination Device Start Address Mid (Start/I-RAM) (W)**

**2237h Both DDA - DMA Destination Device Start Address Msb (Start/BW-RAM)(W)**

```
  0-17  BW-RAM Offset (transfer starts after writing 2237h)
  0-10  I-RAM Offset  (transfer starts after writing 2236h) (2237h is unused)
```

**2238h SA-1 DTC - DMA Terminal Counter Lsb (W)**

**2239h SA-1 DTC - DMA Terminal Counter Msb (W)**

```
  0-15  DMA Transfer Length in bytes (1..65535) (0=Reserved/unknown)
```

DTC is used only for Normal DMA (whilst Character Conversion DMA lasts endless;
for Type 1: as long as SNES reads "BW-RAM" / until it sets 2231h.Bit7, for Type
2: as long as SA-1 writes BRF / until it clears 2230h.Bit0).

**224xh SA-1 BRF - Bit Map Register File (2240h..224Fh) (W)**

These 16 registers can hold two 8 pixel rows (with 2bit/4bit/8bit per pixel).

```
  0-1  2bit pixel (bit 2-7=unused)
  0-3  4bit pixel (bit 4-7=unused)
  0-7  8bit pixel
```

Used only for (semi-automatic) Character Conversion Type 2, where the "DMA"
source data is to be written pixel-by-pixel to these registers; writing to one
8 pixel row can be done while transferring the other row to the SNES.

**Normal DMA (memory transfer within cartridge memory)**

```
  ROM    --> I-RAM     10.74MHz
  ROM    --> BW-RAM    5.37MHz
  BW-RAM --> I-RAM     5.37MHz
  I-RAM  --> BW-RAM    5.37MHz
```

For normal DMA:

```
  Set DCNT (select source/dest/prio/enable)
  Set SDA (set source offset)
  Set DTC (set transfer length)
  Set DDA (set destination offset, and start transfer)
  If desired, wait for CFR.Bit5 (DMA completion interrupt)
```

Normal DMA is used by J. League '96, Jumpin Derby, Marvelous. For ROM, SDA
should be usually C00000h and up (HiROM mapping); Jumpin Derby is
unconventionally using SDA at 2x8xxxh and up (LoROM mapping).

**Character Conversion DMA**

Used to convert bitmaps or pixels to bit-planed tiles. For details, see

**SNES DMA (via Port 43xxh)**

Can be used to transfer "normal" data from ROM/BW-RAM/I-RAM to SNES memory,
also used for forwarding temporary Character Conversion data from I-RAM to
SNES.

**Unknown details**

Unknown if SDA/DDA are increased and if DTC is decreased (or if that operations
appear only on internal registers) (MSBs of DDA are apparently NOT increased on
char conversion DMAs).

### SNES Cart SA-1 Character Conversion

**Character Conversion Types**

```
  Conversion  DMA-Transfer     Source / Pixel-Format
  Type 1      Automatic        BW-RAM, Packed Pixels, Bitmap Pixel Array
  Type 2      Semi-Automatic   CPU, Unpacked Pixels, 8x8 Pixel Tiles
```

Both Conversion types are writing data to a temporary buffer in I-RAM:

```
  I-RAM buffer 32/64/128 bytes (two 8x8 tiles at 2bit/4bit/8bit color depth)
```

From that buffer, data is forwarded to SNES (via a simultanously executed SNES
DMA, ie. via ports 43xxh).

**Character Conversion 1 - Automatically Convert Packed BW-RAM Pixels**

Can be used only if the cartridge DOES contain BW-RAM (most or all do so).

First, do this on SA-1 side:

```
  Set DCNT (Port 2230h) set to Char Conversion Type 1   (...and no DMA-enable?)
```

Then do following on SNES side:

```
  Set SDA (Port 2232h-2234h)=BW-RAM offset, align by (bytes/char)*(chars/line)
  Set CDMA (Port 2231h) = store bits/pixel and chars/line
  Set DDA (Port 2235h-2236h)=I-RAM offset, align (bytes/char)*2 (2237h=unused)
  Wait for SFR.Bit5 (Port 2300h) Char_DMA_IRQ (=first character available)
  Launch SNES-DMA via Port 43xxh from "Virtual BW-RAM?" to PPU-VRAM
    (this can transfer the WHOLE bitmap in one pass)
```

Finally, after the SNES-DMA has finished, do this on SA-1 side:

```
  Set CDMA.Bit7=1 (Port 2231h) - terminate SA-1 DMA
    (that stops writing to I-RAM on SA-1 side)
    (and stops tile-data to be mapped to 400000h-43FFFFh on SNES-side)
```

During conversion, the SA-1 can execute other program code (but waits may occur
on BW-RAM and I-RAM accesses). The SNES CPU is paused (by the DMA) for most of
the time, except for the time slots shortly before/after the DMA; in that time
slots, the SNES may access I-RAM, but may not access BW-RAM.

Conversion 1 is used by Haruka Naru Augusta 3 and Pebble Beach no Hotou.

**Character Conversion 2 - Semi-Automatic Convert Unpacked CPU Pixels**

First, do this on SA-1 side:

```
  Set DCNT (Port 2230h) set to Char Conversion Type 2 and set DMA-enable
  Set CDMA (Port 2231h) = store bits/pixel (chars/line is not used)
  Set DDA (Port 2235h-2236h)=I-RAM offset, align (bytes/char)*2 (2237h=unused)
```

Then repeat for each character:

```
  for y=0 to 7, for x=0 to 7, [2240h+x+(y and 1)]=pixel(x,y), next x,y
  On SNES side: Transfer DMA from 1st/2nd I-RAM buffer half to VRAM or WRAM
```

Finally,

```
  Set DCNT.Bit7=0 (Port 2230h) - disable DMA
```

Conversion 2 is used by Haruka Naru Augusta 3 and SD Gundam G NEXT.

### SNES Cart SA-1 Arithmetic Maths

**2250h SA-1 MCNT - Arithmetic Control (W)**

```
  0-1 Arithmetic Mode (0=Multiply, 1=Divide, 2=MultiplySum, 3=Reserved)
  2-7 Not used (should be "..") (whatever ".." means, maybe "0"?)
```

Note: Writing Bit1=1 does reset the Sum (aka "Cumulative Sum" aka "Accumulative
Sum") to zero.

**2251h SA-1 MA - Arithmetic Parameter A Lsb (Multiplicand/Dividend) (W)**

**2252h SA-1 MA - Arithmetic Parameter A Msb (Multiplicand/Dividend) (W)**

```
  0-15  SIGNED multiplicand or dividend (that is, both are signed)
```

The value in this register is kept intact after multiplaction, but gets
destroyed after division.

**2253h SA-1 MB - Arithmetic Parameter B Lsb (Multiplier/Divisor) (W)**

**2254h SA-1 MB - Arithmetic Parameter B Msb (Multiplier/Divisor)/Start (W)**

```
  0-15  SIGNED multiply parameter, or UNSIGNED divisor
```

The value in this register gets destroyed after both multiplaction and
division. Writing to 2254h starts the operation. Execution time is 5 cycles (in
10.74MHz units) for both Multiply and Divide, and 6 cycles for Multiply/Sum.

**2306h SA-1 MR - Arithmetic Result, bit0-7   (Sum/Product/Quotient) (R)**

**2307h SA-1 MR - Arithmetic Result, bit8-15  (Sum/Product/Quotient) (R)**

**2308h SA-1 MR - Arithmetic Result, bit16-23 (Sum/Product/Remainder) (R)**

**2309h SA-1 MR - Arithmetic Result, bit24-31 (Sum/Product/Remainder) (R)**

**230Ah SA-1 MR - Arithmetic Result, bit32-39 (Sum) (R)**

```
  32bit Multiply Result    (SIGNED)
  40bit Multiply/Sum       (SIGNED)
  16bit Division Result    (SIGNED)
  16bit Division Remainder (UNSIGNED !!!)
```

**230Bh SA-1 OF - Arithmetic Overflow Flag (R)**

This bit is reportedly set on 40bit multiply/addition overflows (rather than on
more useful 32bit overflows), thereby overflow can't occur unless one is doing
at least 512 continous multiply/additions.

```
  0-6 Not used (reportedly "..") (whatever ".." means, maybe 0 or open bus?)
  7   Arithmetic Sum Overflow Flag (0=No overflow, 1=Overflow)
```

Unknown when this bit gets cleared (all operations, or mode changes)?

Division by zero returns result=0000h and remainder=0000h (other info claims
other values?) (but, as far as known, doesn't set set overflow flag).

### SNES Cart SA-1 Variable-Length Bit Processing

**2258h SA-1 VBD - Variable-Length Bit Processing (W)**

```
  0-3  Data Length (1..15=1..15 bits, or 0=16 bits)
  4-6  Not used (should be "..") (whatever ".." means, maybe "0"?)
  7    Data Read Mode (0=Fixed Mode, 1=Auto-increment)
```

Manual/Fixed Mode is used by Jumpin Derby. Auto-increment isn't used by any
known games.

**2259h SA-1 VDA - Variable-Length Bit Game Pak ROM Start Address Lsb (W)**

**225Ah SA-1 VDA - Variable-Length Bit Game Pak ROM Start Address Mid (W)**

**225Bh SA-1 VDA - Variable-Length Bit Game Pak ROM Start Address Msb & Kick**

```
  0-23  Game Pak ROM Address
```

Reading starts on writing to 225Bh.

The ROM address is probably originated at 000000h (rather than using
LoROM/HiROM like CPU addresses)?

**230Ch SA-1 VDP - Variable-Length Data Read Port Lsb (R)**

**230Dh SA-1 VDP - Variable-Length Data Read Port Msb (R)**

```
  0-15  Data
```

Unknown what happens on data length less than 16bits:

```
  Are the selected bits located in MSBs or LSBs?
  Are the other bits set to zero? To next/prev values? Sign-expanded??
```

There is an "auto-increment" feature, which may trigger on reading 230Ch? or on
reading or 230Dh?

;*******PRELOAD:

;Preload occurs after writing VDA

;        bitpos = [2259h]*8

;        [230Ch] = WORD[bitpos/8]

;*******INCREMENT:

;Increment occurs AFTER reading VDP (when auto-increment enabled),

;and after writing VDB (reportedly always, but SHOULD be ONLY when inc=off)?

;        bitpos=bitpos+(([2258h]-1) AND 0Fh)+1

;        [230Ch] = dword[bitpos/16*2] shr (bitpos and 15) AND FFFFh

### SNES Cart GSU-n (programmable RISC CPU) (aka Super FX/Mario Chip) (10 games)

Graphic Support Unit (GSU) (10.74MHz RISC-like CPU)

**GSU Opcodes**

**Misc**

**GSU Caches**

**Pinouts**

### SNES Cart GSU-n List of Games, Chips, and PCB versions

**GSU1/Mario Chip1 is used by six games:**

```
  Dirt Racer (1994) MotiveTime/Elite Systems (EU)
  Dirt Trax FX (1995) Sculptured Software/Acclaim Entertainment (NA)
  Powerslide (cancelled, but unfinished prototype leaked) Elite Systems (EU)
  Star Fox / Starwing (1993) Argonaut/Nintendo EAD (NA) (JP) (EU)
  Star Fox / Starwing: Competition Edition (demo version) (1993) (NA) (EU)
  Stunt Race FX / Wild Trax (1994) Argonaut/Nintendo EAD (NA) (JP) (EU)
  Vortex (1994) Argonaut Games/Electro Brain (NA), Pack-In-Video (JP)
```

**GSU2/GSU2-SP1 is used by four games:**

```
  Doom (1996) Sculptured Software/Williams (NA), Imagineer (JP), Ocean (EU)
  Super Mario World 2: Yoshi's Island (1995) Nintendo EAD (NA) (JP) (EU)
  Winter Gold / FX Skiing (1997) Funcom/Nintendo (NA) (EU)
  Star Fox 2 (cancelled, but near-finished Beta version leaked into internet)
```

Reportedly, there have been another three GSU2 games planned:

```
  FX Fighter (Beta) (cancelled) Argonaut Games/GTE Entertainment (NA) (EU)
  Comanche (cancelled) Nova Logic (NA)
  Super Mario FX (cancelled) Nintendo EAD
```

**GSU Chip Versions**

```
  MC1      - 100pin - A/N Inc. Nintendo Mario Chip 1 (reportedly "FX-chip 1")
  GSU1     - 100pin - A/N Inc. Nintendo Super FX 1 (10.74MHz RISC-like CPU)
  GSU1A    - 100pin - A/N Inc. Nintendo Super FX 1
  GSU2     - 112pin - A/N Inc. Nintendo Super FX 2 (as above, but 21MHz)
  GSU2-SP1 - 112pin - A/N Inc. Nintendo Super FX 2 (as above, but 21MHz)
```

XXX according to MotZilla, GSU1 supports 21MHz, too? (but with less memory)

**GSU PCB Versions**

```
  SHVC-1C0N         Mario Chip 1      Star Fox (Blob)
  SHVC-1C0N5S-01    Mario Chip 1      Star Fox (SMD)
  SHVC-1CA0N5S-01   GSU-1             Dirt Racer & Vortex
  SHVC-1CA0N6S-01   GSU-1             Dirt Trax FX
  SHVC-1CA6B-01     GSU-1 Battery     Stunt Race FX
  SHVC-1CB0N7S-01   GSU-2             Doom
  SHVC-1CB5B-01     GSU-2 Battery     Super Mario World 2: Yoshi's Island
  SHVC-1CB5B-20     GSU-2-SP1 Battery Super Mario World 2: Yoshi's Island
  SHVC-1RA2B6S-01   GSU1A Batt+Eprom  Powerslide (prototype board)
  GS 0871-102       Mario Chip 1      Super Famicom Box PSS61 multi-game-cart
```

Note: Doom's "1CB0N7S" board has only 64K RAM installed (not 128K).

### SNES Cart GSU-n Memory Map

**MC1 Memory Map (at SNES Side)**

```
  00-3F/80-BF:3000-347F  GSU I/O Ports
  00-1F/80-9F:8000-FFFF  Game Pak ROM in LoRom mapping (1Mbyte max)
  60-7D/E0-FF:0000-FFFF  Game Pak RAM with mirrors (64Kbyte max?, usually 32K)
  Other Addresses        Open Bus
```

**GSU1 Memory Map (at SNES Side)**

```
  00-3F/80-BF:3000-34FF? GSU I/O Ports
  00-3F/80-BF:6000-7FFF  Mirror of 70:0000-1FFF (ie. FIRST 8K of Game Pak RAM)
  00-3F/80-BF:8000-FFFF  Game Pak ROM in LoRom mapping (1Mbyte max?)
  40-5F/C0-DF:0000-FFFF  Game Pak ROM in HiRom mapping (mirror of above)
  70-71/F0-F1:0000-FFFF  Game Pak RAM with mirrors (64Kbyte max?, usually 32K)
  78-7x/F8-Fx:0000-FFFF  Unknown (maybe Additional "Backup" RAM like GSU2)
  Other Addresses        Open Bus
```

**GSU2 Memory Map (at SNES Side)**

```
  00-3F/80-BF:3000-34FF  GSU I/O Ports
  00-3F/80-BF:6000-7FFF  Mirror of 70:0000-1FFF (ie. FIRST 8K of Game Pak RAM)
  00-3F:8000-FFFF        Game Pak ROM in LoRom mapping (2Mbyte max)
  40-5F:0000-FFFF        Game Pak ROM in HiRom mapping (mirror of above)
  70-71:0000-FFFF        Game Pak RAM       (128Kbyte max, usually 32K or 64K)
  78-79:0000-FFFF        Additional "Backup" RAM  (128Kbyte max, usually none)
  80-BF:8000-FFFF        Additional "CPU" ROM LoROM (2Mbyte max, usually none)
  C0-FF:0000-FFFF        Additional "CPU" ROM HiROM (4Mbyte max, usually none)
  Other Addresses        Open Bus
```

For HiROM mapping the address bits are shifted, so both LoROM and HiROM are
linear (eg. Bank 40h contains mirrors of Bank 00h and 01h).

Although both LoROM and HiROM are supported, the header & exception vectors
are located at ROM Offset 7Fxxh (in LoROM fashion), accordingly the cartridge
header declares the cartridge as LoROM.

The additional ROM/RAM regions would be mapped to SNES CPU only (not to GSU),
they aren't installed in existing cartridges, that implies that the "Fast" ROM
banks (80h-FFh) are unused, so GSU games are restricted to "Slow" ROM.

**GSU2 Memory Map (at GSU Side)**

```
  00-3F:0000-7FFF  Mirror of LoROM at 00-3F:8000-FFFF (for "GETB R15" vectors)
  00-3F:8000-FFFF  Game Pak ROM in LoRom mapping (2Mbyte max)
  40-5F:0000-FFFF  Game Pak ROM in HiRom mapping (mirror of above 2Mbyte)
  70-71:0000-FFFF  Game Pak RAM       (128Kbyte max, usually 32K or 64K)
  PBR:0000-01FF    Code-Cache (when having manually stored opcodes in it)
```

PBR can be set to both ROM/RAM regions (or cache region), ROMBR only to ROM
region (00h-5Fh), RAMBR only to RAM region (70h-71h).

**GSU Interrupt Vectors**

The SNES Exception Vectors (at FFE4h-FFFFh) are normally located in Game Pak
ROM. When the GSU is running (with GO=1 and RON=1), ROM isn't mapped to SNES
memory, instead, fixed values are appearing as ROM (depending of the lower 4bit
of the address):

```
  Any Address     Exception Vectors
  [xxx0h]=0100h   -
  [xxx2h]=0100h   -
  [xxx4h]=0104h   [FFE4h]=0104h  COP Vector in 65C816 mode (COP opcode)
  [xxx6h]=0100h   [FFE6h]=0100h  BRK Vector in 65C816 mode (BRK opcode)
  [xxx8h]=0100h   [FFE8h]=0100h  ABT Vector in 65C816 mode (Not used in SNES)
  [xxxAh]=0108h   [FFEAh]=0108h  NMI Vector in 65C816 mode (Vblank)
  [xxxCh]=0100h   -
  [xxxEh]=010Ch   [FFEEh]=010Ch  IRQ Vector in 65C816 mode (H/V-IRQ & GSU-STOP)
```

It'd be best to set the Game Pak ROM vectors to the same addresses, otherwise
the vectors would change when the GSU is running (or possibly, the fixed-LSBs
may be mixed-up with ROM-MSBs).

**GSU Cartridge Header (always at ROM Offset 7Fxxh, in LoROM fashion)**

```
  [FFD5h]=20h        Set to "Slow/LoROM" (although both LoROM/HiROM works)
  [FFD6h]=13h..1Ah   Chipset = GSUn (plus battery present/absent info)
  [FFD8h]=00h        Normal SRAM Size (None) (always use the Expansion entry)
  [FFBDh]=05h..06h   Expansion RAM Size (32Kbyte and 64Kbyte exist)
  Caution: Starfox/Star Wing, Powerslide, and Starfox 2 do not have extended
  headers (and thereby no [FFBDh] entry). RAM Size for Starfox/Starwing is
  32Kbytes, RAM Size for Powerslide and Starfox 2 is unknown.
```

There is no info in the header (nor extended header) whether the game uses a
GSU1 or GSU2. Games with 2MByte ROM are typically using GSU2 (though that rule
doesn't always match: Star Fox 2 is only 1MByte).

**GSU Busses**

The GSU seems to have 4 address/data busses (three external ones, and one
internal cache bus):

```
  SNES bus (for forwarding ROM/RAM access to SNES)
  ROM bus (for GSU opcode fetches, GETxx reads, and SNES reads)
  RAM bus (for GSU opcode fetches, LOAD/STORE/PLOT/RPIX, and SNES access)
  Code cache bus (for GSU opcode fetches only) (and SNES I/O via 3100h..32FFh)
```

To some level, this allows to do multiple things simultaneously: Reading a GSU
opcode from cache at the same time while prefetching ROM data and forwarding
the RAM or Pixel cache to RAM.

### SNES Cart GSU-n I/O Map

**GSU I/O Map (in banks 00h-3Fh and 80h-BFh)**

During GSU operation, only SFR, SCMR, and VCR may be accessed.

```
  3000h-3001h R0  Default source/destination register (Sreg/Dreg) (R/W)
  3002h-3003h R1  PLOT opcode: X coordinate (0000h on reset) (R/W)
  3004h-3005h R2  PLOT opcode: Y coordinate (0000h on reset) (R/W)
  3006h-3007h R3  General purpose (R/W)
  3008h-3009h R4  LMULT opcode: lower 16bits of result (R/W)
  300Ah-300Bh R5  General purpose (R/W)
  300Ch-300Dh R6  LMULT and FMULT opcodes: multiplier (R/W)
  300Eh-300Fh R7  MERGE opcode (R/W)
  3010h-3011h R8  MERGE opcode (R/W)
  3012h-3013h R9  General purpose (R/W)
  3014h-3015h R10 General purpose (conventionally stack pointer) (R/W)
  3016h-3017h R11 LINK opcode: destination (R/W)
  3018h-3019h R12 LOOP opcode: counter (R/W)
  301Ah-301Bh R13 LOOP opcode: address (R/W)
  301Ch-301Dh R14 GETxx opcodes: Game Pak ROM Address Pointer (R/W)
  301Eh-301Fh R15 Program Counter, writing MSB starts GSU operation (R/W)
  3020h-302Fh -
  3030h-3031h SFR Status/Flag Register (R) (Bit1-5: R/W)
  3032h       -
  3033h       BRAMR Back-up RAM Register (W)
  3034h       PBR   Program Bank Register (8bit, bank 00h..FFh) (R/W)
  3035h       -
  3036h       ROMBR Game Pak ROM Bank Register (8bit, bank 00h..FFh) (R)
  3037h       CFGR  Config Register (W)
  3038h       SCBR  Screen Base Register (8bit, in 1Kbyte units) (W)
  3039h       CLSR  Clock Select Register (W)
  303Ah       SCMR  Screen Mode Register (W)
  303Bh       VCR   Version Code Register (R)
  303Ch       RAMBR Game Pak RAM Bank Register (1bit, bank 70h/71h) (R)
  303Dh       -
  303Eh-303Fh CBR   Cache Base Register (in upper 12bit; lower 4bit=unused) (R)
  N/A         COLR  Color Register (COLOR,GETC,PLOT opcodes)
  N/A         POR   Plot Option Register (CMODE opcode)
  N/A         Sreg/Dreg    Memorized TO/FROM Prefix Selections
  N/A         ROM Read Buffer (1 byte) (prefetched from [ROMBR:R14])
  N/A         RAM Write Buffer (1 byte/word)
  N/A         RAM Address (1 word, or word+rambr?) (for SBK opcode)
  N/A         Pixel Write Buffer (two buffers for one 8-pixel row each)
  3100h-32FFh Cache RAM
```

**Full I/O Map with Mirrors for Black Blob (VCR=01h)**

```
  3000h..301Fh  20h  R0-R15
  3020h..302Fh  10h  open bus
  3030h..3031h  2    status reg
  3032h..303Fh  0Eh  mirrors of status reg (except 303Bh=01h=VCR)
  3040h..305Fh  20h  mirror of R0-R15
  3060h..307Fh  20h  mirrors of status reg (except 307Bh=01h=VCR)
  3080h..30FFh  80h  open bus
  3100h..32FFh  200h cache
  3300h..332Fh  30h  open bus
  3330h..333Fh  10h  mirrors of status reg (except 333Bh=01h=VCR)
  3340h..335Fh  20h  mirror of R0-R15
  3360h..337Fh  20h  mirrors of status reg (except 337Bh=01h=VCR)
  3380h..33FFh  80h  open bus
  3400h..342Fh  30h  open bus
  3430h..343Fh  10h  mirrors of status reg (except 343Bh=01h=VCR)
  3440h..345Fh  20h  mirror of R0-R15
  3460h..347Fh  20h  mirrors of status reg (except 347Bh=01h=VCR)
  3480h..3FFFh  B80h open bus
```

**Full I/O Map with Mirrors for GSU2 (VCR=04h)**

```
  3000h..301Fh  20h   R0-R15
  3020h..302Fh  10h   mirror of 3030h..303Fh
  3030h..303Fh  10h   status regs (unused or write-only ones return 00h)
  3040h..30FFh  C0h   mirrors of 3000h..303Fh
  3100h..32FFh  200h  cache
  3300h..34FFh  200h  mirrors of 3000h..303Fh
  3500h..3FFFh  B00h  open-bus
```

### SNES Cart GSU-n General I/O Ports

**3000h-301Fh - R0-R15 - CPU Registers (R/W)**

16bit CPU registers (see GSU I/O map for additional details on each register).

Writes to 3000h-301Eh (even addresses) do set LATCH=data.

Writes to 3001h-301Fh (odd addresses) do apply LSB=LATCH and MSB=data.

Writes to 301Fh (R15.MSB) do also set GO=1 (and start GSU code execution).

**3030h/3031h - SFR - Status/Flag Register (R) (Bit1-5: R/W)**

```
  0  -    Always 0                                                        (R)
  1  Z    Zero Flag     (0=NotZero/NotEqual, 1=Zero/Equal)                (R/W)
  2  CY   Carry Flag    (0=Borrow/NoCarry, 1=Carry/NoBorrow)              (R/W)
  3  S    Sign Flag     (0=Positive, 1=Negative)                          (R/W)
  4  OV   Overflow Flag (0=NoOverflow, 1=Overflow)                        (R/W)
  5  GO   GSU is running (cleared on STOP) (can be forcefully=0 via 3030h)(R/W)
  6  R    ROM[R14] Read (0=No, 1=Reading ROM via R14 address)             (R)
  7  -    Always 0                                                        (R)
  8  ALT1 Prefix Flag           ;\for ALT1,ALT2,ALT3 prefixes             (R)
  9  ALT2 Prefix Flag           ;/                                        (R)
  10 IL   Immediate lower 8bit flag ;\Unknown, probably set/reset internally
  11 IH   Immediate upper 8bit flag ;/when processing opcodes with imm operands
  12 B    Prefix Flag           ;-for WITH prefix (used by MOVE/MOVES opcodes)
  13 -    Always 0                                                        (R)
  14 -    Always 0                                                        (R)
  15 IRQ  Interrupt Flag (reset on read, set on STOP) (also set if IRQ masked?)
```

This register is read/write-able even when the GSU is running; reading mainly
makes sense for checking GO and IRQ bits, writing allows to clear the GO flag
(thereby aborting the GSU program; the write does most likely also destroy the
other SFR bits, so one cannot pause/resume).

**3034h - PBR - Program Bank Register (8bit, bank 00h..5Fh,70h..71h) (R/W)**

**3036h - ROMBR - Game Pak ROM Bank Register (8bit, bank 00h..5Fh) (R)**

**303Ch - RAMBR - Game Pak RAM Bank Register (1bit, bank 70h..71h) (R)**

Memory banks for GSU opcode/data accesses. PBR can be set to both ROM and RAM
regions, ROMBR/RAMBR only to ROM or RAM regions respectively. Existing
cartridges have only 32Kbyte or 64Kbyte RAM, so RAMBR should be always zero.

According to book2 (page 258), the screen base is also affected by RAMBR
(unknown if that is true, theoretically, SCBR is large enough to address more
than 64Kbytes without RAMBR).

**303Eh/303Fh - CBR - Cache Base Register (upper 12bit; lower 4bit=unused) (R)**

Code-Cache Base for Game Pak ROM/RAM. The register is read-only, so the SNES
cannot directly write to it, however, the SNES can set CBR=0000h by writing
GO=0 (in SFR register).

**3033h - BRAMR - Back-up RAM Register (W)**

```
  0   BRAM Flag (0=Disable/Protect, 1=Enable)
  1-7 Not used (should be zero)
```

This register would be used only if the PCB does have a separate "Backup" RAM
chip mapped to 780000h-79FFFFh (additionally to the Game Pak RAM chip). None of
the existing PCBs is having that extra RAM chip, so the register is having no
function. (Note: However, some PCBs do include a battery wired to Game Pak RAM
chip, anyways, that type of "backup" isn't affected by this register).

**303Bh - VCR - Version Code Register (R)**

```
  0-7 GSU Chip Version (01h..0xh ?)
```

Known versions: 1=MC1/Blob, ?=MC1/SMD, ?=GSU1, ?=GSU1A, 4=GSU2, ?=GSU2-SP1.

**3037h - CFGR - Config Register (W)**

```
  0-4 -   Not used (should be zero)
  5   MS0 Multiplier Speed Select (0=Standard, 1=High Speed Mode)
  6   -   Not used (should be zero)
  7   IRQ Interrupt Mask (0=Trigger IRQ on STOP opcode, 1=Disable IRQ)
```

MS0 <must> be zero in 21MHz mode (ie. only CFGR.Bit5 or CLSR.Bit0 may be
set).

MS0 is implemented in GSU2 (maybe also other chips), it is not implemented on
Black Blob MC1 (which is always using slow multiply mode).

**3039h - CLSR - Clock Select Register (W)**

```
  0   CLS Clock Select (0=10.7MHz, 1=21.4MHz)
  1-7 -   Not used (should be zero)
```

CLS exists on all GSU variants (including Black Blob MC1) (however, there are
rumours that the fast mode was "bugged" on older MC1, unknown if that's true).

**N/A - ROM Buffer - Prefetched Byte(s?) at [ROMBR:R14]**

**N/A - Sreg/Dreg - Memorized TO/FROM Prefix Selections**

**3100h..32FFh - Cache RAM**

### SNES Cart GSU-n Bitmap I/O Ports

**3038h - SCBR - Screen Base Register (8bit, in 1Kbyte units) (W)**

```
  0-7  Screen Base in 1K-byte Units (Base = 700000h+N*400h)
```

**303Ah - SCMR - Screen Mode Register (W)**

```
  0-1 MD0-1 Color Gradient (0=4-Color, 1=16-Color, 2=Reserved, 3=256-Color)
  2   HT0   Screen Height  (0=128-Pixel, 1=160-Pixel, 2=192-Pixel, 3=OBJ-Mode)
  3   RAN   Game Pak RAM bus access (0=SNES, 1=GSU)
  4   RON   Game Pak ROM bus access (0=SNES, 1=GSU)
  5   HT1   Screen Height  (MSB of HT0 bit)
  6-7 -     Not used (should be zero)
```

RON/RAN can be temporarily cleared during GSU operation, this causes the GSU to
enter WAIT status (if it accesses ROM or RAM), and continues when RON/RAN are
changed back to 1.

Note that "OBJ Mode" can be also selected by POR.Bit4 (if so, HT0/HT1 bits are
ignored).

```
  256x128 pixels   256x160 pixels   256x192 pixels   OBJ Mode 256x256 pixel
  000 010 .. 1F0 | 000 014 .. 26C | 000 018 .. 1E8 | 000 .. 00F 100 .. 10F
  001 011 .. 1F1 | 001 015 .. 26D | 001 019 .. 1E9 | ..  .. ..  ..  .. ..
  ..  ..  .. ..  | ..  ..  .. ..  | ..  ..  .. ..  | 0F0 .. 0FF 1F0 .. 1FF
  ..  ..  .. ..  | ..  ..  .. ..  | ..  ..  .. ..  | 200 .. 20F 300 .. 30F
  00E 01E .. 1FE | 012 026 .. 27E | 016 02E .. 2FE | ..  .. ..  ..  .. ..
  00F 01F .. 1FF | 013 027 .. 27F | 017 02F .. 2FF | 2F0 .. 2FF 3F0 .. 3FF
```

In the first three cases, BG Map is simply filled with columns containing
increasing tile numbers. The fourth case is matched to the SNES two-dimensional
OBJ mapping; it can be used for BG Map (with entries 0..3FF as shown above), or
for OBJ tiles (whereas, mind that the SNES supports only 0..1FF OBJs, not
200..3FF).

The Tile Number is calculated as:

```
  Height 128 --> (X/8)*10h + (Y/8)
  Height 160 --> (X/8)*14h + (Y/8)
  Height 192 --> (X/8)*18h + (Y/8)
  OBJ Mode --> (Y/80h)*200h + (X/80h)*100h + (Y/8 AND 0Fh)*10h + (X/8 AND 0Fh)
```

The Tile-Row Address is:

```
  4 Color Mode    TileNo*10h + SCBR*400h + (Y AND 7)*2
  16 Color Mode   TileNo*20h + SCBR*400h + (Y AND 7)*2
  256 Color Mode  TileNo*40h + SCBR*400h + (Y AND 7)*2
```

With Plane0,1 stored at Addr+0, Plane 2,3 at Addr+10h, Plane 4,5 at Addr+20h,
Plane 6,7 at Addr+30h.

**N/A - COLR - Color Register**

```
  0-7 CD0-7 Color Data
```

**N/A - POR - Plot Option Register (CMODE)**

```
  0   PLOT Transparent       (0=Do Not Plot Color 0, 1=Plot Color 0)
  1   PLOT Dither            (0=Normal, 1=Dither; 4/16-color mode only)
  2   COLOR/GETC High-Nibble (0=Normal, 1=Replace incoming LSB by incoming MSB)
  3   COLOR/GETC Freeze-High (0=Normal, 1=Write-protect COLOR.MSB)
  4   OBJ Mode               (0=Normal, 1=Force OBJ mode; ignore SCMR.HT0/HT1)
  5-7 Not used (should be zero)
```

Can be changed by CMODE opcode, used for COLOR/GETC/PLOT opcodes.

Dither can mix transparent & non-transparent pixels.

Bit0=0 (Transparent) causes PLOT to skip color 0 (so PLOT does only increment
R1 (X-coordinate), but doesn't draw a pixel). Depending on color depth, the
color 0 check tests the lower 2/4/8 bits of the drawing color (if POR.Bit3
(Freeze-High) is set, then it checks only the lower 2/4 bits, and ignores upper
4bit even when in 256-color mode).

Bit1=1 (Dither) causes PLOT to use dithering, that is, if "(r1.bit0 XOR
r2.bit0)=1" then COLOR/10h is used as drawing color; using Color 0 as one of
the two colors can produce a semi-transparency effect. Dither is ignored in
256-color mode.

Bit2=1 (High-Nibble) causes COLOR/GETC to replace the LSB of the incoming data
by the MSB of the incoming data; this allows two 4bit bitmaps being stored at
the same memory area (one in the LSBs, the other in MSBs).

Bit3=1 (Freeze-High) causes COLOR/GETC to change only the LSB of the color
register; this allows the MSB to be used as fixed palette-like value in
256-color mode, it might be also useful for fixed dither-colors in 4/16 color
mode.

Bit3=1 forces OBJ Mode (same as when setting SCMR.HT0/HT1 to OBJ Mode).

```
  <------- COLOR/GETC TO COLOR ------->    <------- PLOT COLOR TO RAM -------->
               ______              __________                    ______
  Bit7-4 --+--|Freeze|- - - - - ->|          |---+------------->|      |
           |  |POR.3 |            |  COLOR   |   |              |Transp|
           |  |______|  ______    | _ _ _ _  |   |    ______    |POR.0 |--> RAM
           '---------->|Nibble|   |          |   '-->|Dither|   |      |
                       |POR.2 |-->| Register |       |POR.1 |-->|      |
  Bit3-0 ------------->|______|   |__________|------>|______|   |______|
```

### SNES Cart GSU-n CPU MOV Opcodes

**GSU MOV Opcodes (Register/Immediate)**

```
  Opcode     Clks Flags   Native       Nocash
  2s 1d         2 000---- MOVE Rd,Rs   mov Rd,Rs   ;Rd=Rs
  2d Bs         2 000vs-z MOVES Rd,Rs  movs Rd,Rs  ;Rd=Rs (with flags, OV=bit7)
  An pp         2 000---- IBT Rn,#pp   mov Rn,pp   ;Rn=SignExpanded(pp)
  Fn xx yy      3 000---- IWT Rn,#yyxx mov Rn,yyxx ;Rn=yyxx
```

**GSU MOV Opcodes (Load BYTE from ROM)**

```
  EF          1-6 000---- GETB         movb Rd,[romb:r14]    ;hi=zero-expanded
  3D EF       2-6 000---- GETBH        movb Rd.hi,[romb:r14] ;lo=unchanged
  3E EF       2-6 000---- GETBL        movb Rd.lo,[romb:r14] ;hi=unchanged
  3F EF       2-6 000---- GETBS        movbs Rd,[romb:r14]   ;hi=sign-expanded
```

**GSU MOV Opcodes (Load/Store Byte/Word to/from RAM)**

```
  3D 4n         6 000---- LDB (Rn)     movb Rd,[ramb:Rn]  ;Rd=Byte[..] ;n=0..11
  4n            7 000---- LDW (Rn)     mov Rd,[ramb:Rn]   ;Rd=Word[..] ;n=0..11
  3D Fn lo hi  11 000---- LM Rn,(hilo) mov Rn,[ramb:hilo] ;Rn=Word[..]
  3D An kk     10 000---- LMS Rn,(yy)  mov Rn,[ramb:kk*2] ;Rn=Word[..]
  3D 3n       2-5 000---- STB (Rn)     movb [ramb:Rn],Rs  ;Byte[..]=Rs ;n=0..11
  3n          1-6 000---- STW (Rn)     mov [ramb:Rn],Rs   ;Word[..]=Rs ;n=0..11
  3E Fn lo hi 4-9 000---- SM (hilo),Rn mov [ramb:hilo],Rn ;Word[..]=Rn
  3E An kk    3-8 000---- SMS (yy),Rn  mov [ramb:kk*2],Rn ;Word[..]=Rn
  90          1-6 000---- SBK          mov [ram:bk],Rs    ;Word[LastRamAddr]=Rs
```

Words at odd addresses are accessing [addr AND NOT 1], with data LSB/MSB
swapped. LDB does zero-expand result (Rd.hi=00h). STB does store Rs.lo (ignores
Rs.hi). SBK does "writeback" to most recently used RAM address (eg. can be used
after LM) (unknown if whole 17bit, including ramb, are saved).

**GSU ROM/RAM Banks**

```
  3E DF         2 000---- RAMB         movb ramb,Rs ;RAMBR=Rs & 01h ;RAM Bank
  3F DF         2 000---- ROMB         movb romb,Rs ;ROMBR=Rs & FFh ;ROM Bank
```

**GSU Bitmap Opcodes**

```
  3D 4E         2 000---- CMODE        movb por,Rs           ;=Rs&1Fh
  4E            1 000---- COLOR        movb color,Rs         ;=Rs&FFh
  DF          1-6 000---- GETC         movb color,[romb:r14] ;=[membyte]
  4C         1-48 000---- PLOT         plot [r1,r2],color ;Pixel=COLR, R1=R1+1
  3D 4C     20-74 000-s-z RPIX         rpix Rd,[r1,r2] ;Rd=Pixel? FlushPixCache
```

Unknown if RPIX always sets SF=0, theoretically 2bit/4bit/8bit pixel-colors
cannot be negative, unless it uses bit7 as sign, or so?

### SNES Cart GSU-n CPU ALU Opcodes

**GSU ALU Opcodes**

```
  Opcode     Clks Flags   Native       Nocash
  5n            1 000vscz ADD Rn       add Rd,Rs,Rn ;Rd=Rs+Rn
  3E 5n         2 000vscz ADD #n       add Rd,Rs,n  ;Rd=Rs+n
  3D 5n         2 000vscz ADC Rn       adc Rd,Rs,Rn ;Rd=Rs+Rn+Cy
  3F 5n         2 000vscz ADC #n       adc Rd,Rs,n  ;Rd=Rs+n+Cy
  6n            1 000vscz SUB Rn       sub Rd,Rs,Rn ;Rd=Rs-Rn
  3E 6n         2 000vscz SUB #n       sub Rd,Rs,n  ;Rd=Rs-n
  3D 6n         2 000vscz SBC Rn       sbc Rd,Rs,Rn ;Rd=Rs-Rn-(Cy XOR 1)
  3F 6n         2 000vscz CMP Rn       cmp Rs,Rn    ;Rs-Rn
  7n            1 000-s-z AND Rn       and Rd,Rs,Rn ;Rd=Rs AND Rn     ;n=1..15!
  3E 7n         2 000-s-z AND #n       and Rd,Rs,n  ;Rd=Rs AND n      ;n=1..15!
  3D 7n         2 000-s-z BIC Rn       bic Rd,Rs,Rn ;Rd=Rs AND NOT Rn ;n=1..15!
  3F 7n         2 000-s-z BIC #n       bic Rd,Rs,n  ;Rd=Rs AND NOT n  ;n=1..15!
  Cn            1 000-s-z OR  Rn       or  Rd,Rs,Rn ;Rd=Rs OR Rn      ;n=1..15!
  3E Cn         2 000-s-z OR  #n       or  Rd,Rs,n  ;Rd=Rs OR n       ;n=1..15!
  3D Cn (?)     2 000-s-z XOR Rn       xor Rd,Rs,Rn ;Rd=Rs XOR Rn     ;n=1..15?
  3F Cn (?)     2 000-s-z XOR #n       xor Rd,Rs,n  ;Rd=Rs XOR n      ;n=1..15?
  4F            1 000-s-z NOT          not Rd,Rs    ;Rd=Rs XOR FFFFh
```

**GSU Rotate/Shift/Inc/Dec Opcodes**

```
  03            1 000-0cz LSR          shr Rd,Rs,1  ;Rd=Rs SHR 1
  96            1 000-scz ASR          sar Rd,Rs,1  ;Rd=Rs SAR 1
  04            1 000-scz ROL          rcl Rd,Rs,1  ;Rd=Rs RCL 1 ;\through
  97            1 000-scz ROR          rcr Rd,Rs,1  ;Rd=Rs RCR 1 ;/carry
  3D 96         2 000-scz DIV2         div2 Rd,Rs   ;Rd=Rs SAR 1, Rd=0 if Rs=-1
  Dn            1 000-s-z INC Rn       inc Rn       ;Rn=Rn+1          ;n=0..14!
  En            1 000-s-z DEC Rn       dec Rn       ;Rn=Rn-1          ;n=0..14!
```

**GSU Byte Operations**

```
  4D            1 000-s-z SWAP         ror Rd,Rs,8    ;Rd=Rs ROR 8
  95            1 000-s-z SEX          movbs Rd,Rs    ;Rd=SignExpanded(Rs&FFh)
  9E            1 000-s-z LOB          and Rd,Rs,0FFh ;Rd=Rs AND FFh  ;SF=Bit7
  C0            1 000-s-z HIB          shr Rd,Rs,8    ;Rd=Rs SHR 8    ;SF=Bit7
  70            1 000xxxx MERGE        merge Rd,r7,r8 ;Rd=R7&FF00 + R8/100h
```

Flags for MERGE are:

```
  S = set if (result AND 8080h) is nonzero
  V = set if (result AND C0C0h) is nonzero
  C = set if (result AND E0E0h) is nonzero
  Z = set if (result AND F0F0h) is nonzero (not set when zero!)
```

**GSU Multiply Opcodes**

```
  9F          4,8 000-scz FMULT     smulw Rd:nul,Rs,r6 ;Rd=signed(Rs*R6/10000h)
  3D 9F       5,9 000-scz LMULT     smulw Rd:R4,Rs,R6  ;Rd:R4=signed(Rs*R6)
  8n          1,2 000-s-z MULT Rn      smulb Rd,Rs,Rn ;Rd=signed(RsLsb*RnLsb)
  3E 8n       2,3 000-s-z MULT #n      smulb Rd,Rs,n  ;Rd=signed(RsLsb*0..15)
  3D 8n       2,3 000-s-z UMULT Rn     umulb Rd,Rs,Rn ;Rd=unsigned(RsLsb*RnLsb)
  3F 8n (?)   2,3 000-s-z UMULT #n     umulb Rd,Rs,n  ;Rd=unsigned(RsLsb*0..15)
```

The multiply speed can be selected via CFGR register. Do not use FMULT with
Dreg=R4 (this will reportedly leave R4 unchanged). When using LMULT with
Dreg=R4 then the result will be R4=MSB (and LSB is lost). Ie. if that is true
then, strangely, LMULT R4 <does> work as how FMULT R4 <should>
work.

### SNES Cart GSU-n CPU JMP and Prefix Opcodes

**GSU Special Opcodes**

```
  Opcode     Clks Flags   Native    Nocash
  00            1 000---- STOP      stop  ;SFR.GO=0, SFR.IRQ=1, R15=$+2
  01            1 000---- NOP       nop   ;NOP (often used as dummy after jump)
  02           1* 000---- CACHE     cache ;IF CBR<>PC&FFF0 then CBR=PC&FFF0
```

STOP at $+0 does prefetch another opcode byte at $+1 (but without executing
it), and does then stop with R15=$+2, SFR.GO=0, SFR.IRQ=1 (that, even if IRQ is
disabled in CFGR.IRQ).

BUG: On MC1 (maybe also GSU1), STOP hangs when executed after a RAM write
(there must be at least 2 cycles after write, eg. insert two NOPs before STOP;
the required delay might vary depending on CPU speed or code cache? the bug
doesn't occur on GSU2).

**GSU Jump Opcodes**

```
  Opcode     Clks Flags   Native    Nocash
  05 nn         2 ------- BRA addr  jr  addr   ;Always, R15=R15+signed(nn)
  06 nn         2 ------- BGE addr  jge addr   ;If (S XOR V)=0 then ..
  07 nn         2 ------- BLT addr  jl  addr   ;If (S XOR V)=1 then ..
  08 nn         2 ------- BNE addr  jne addr   ;If ZF=0 then R15=R15+signed(nn)
  09 nn         2 ------- BEQ addr  je  addr   ;If ZF=1 then R15=R15+signed(nn)
  0A nn         2 ------- BPL addr  jns addr   ;If SF=0 then R15=R15+signed(nn)
  0B nn         2 ------- BMI addr  js  addr   ;If SF=1 then R15=R15+signed(nn)
  0C nn         2 ------- BCC addr  jnc addr   ;If CY=0 then R15=R15+signed(nn)
  0D nn         2 ------- BCS addr  jc  addr   ;If CY=1 then R15=R15+signed(nn)
  0E nn         2 ------- BVC addr  jno addr   ;If OV=0 then R15=R15+signed(nn)
  0F nn         2 ------- BVS addr  jo  addr   ;If OV=1 then R15=R15+signed(nn)
  9n            1 000---- JMP Rn    jmp Rn     ;R15=Rn                ;n=8..13!
  3D 9n         2 000---- LJMP Rn   jmp Rn:Rs  ;R15=Rs, PBR=Rn, CBR=? ;n=8..13!
  3C            1 000-s-z LOOP    loop r12,r13 ;r12=r12-1, if Zf=0 then R15=R13
  9n            1 000---- LINK #n link r11,addr;R11=R15+n             ;n=1..4
```

Jumps can be also implemented by using R15 (PC) as destination register (eg. in
MOV/ALU commands).

Observe that the NEXT BYTE after any jump/branch opcodes is fetched before
continuing at the jump destination address. The fetched byte is executed after
the jump, but before executing following opcodes at the destination (in case of
multi-byte opcodes, this results in a 1-byte-fragment being located after the
jump-origin, and the remaining byte(s) at the destination).

**GSU Prefix Opcodes**

ALT1/ALT2/ALT3 prefixes do change the operation of an opcode, these are usually
implied in the opcode description (for example, "3F 6n" is "CMP R0,Rn").

TO/WITH/FROM prefixes allow to select source/destination registers (otherwise
R0 is used as default register) (for example, "Bs 3F 6n" is "CMP Rs,Rn").

The prefixes are normally reset after execution of any opcode, the only
exception are the Bxx (branch) opcodes, these leave prefixes unchanged
(allowing to "split" opcodes, for example placing ALT1/TO/etc. before Bxx, and
the next opcode byte after Bxx).

Aside from setting Sreg+Dreg, WITH does additionally set the B-flag, this
causes any following 1nh/Bnh bytes to act as MOVE/MOVES opcodes (rather than as
TO/FROM prefixes).

```
  Opcode Clks Flags   Name    Bflg ALT1 ALT2 Rs Rd
  3D        1 -1----- ALT1     -    1    -   -  -  ;prefix for 3D xx opcodes
  3E        1 --1---- ALT2     -    -    1   -  -  ;prefix for 3E xx opcodes
  3F        1 -11---- ALT3     -    1    1   -  -  ;prefix for 3F xx opcodes
  1n        1 ------- TO Rn    -    -    -   -  Rn ;select Rn as Rd
  2n        1 1------ WITH Rn  1    -    -   Rn Rn ;select Rn as Rd & Rs
  Bn        1 ------- FROM Rn  -    -    -   Rn -  ;select Rn as Rs
  05..0F nn 2 ------- Bxx addr -    -    -   -  -  ;branch opcodes (no change)
  other    .. 000---- other    0    0    0   R0 R0 ;other opcodes (reset all)
```

Other opcodes do reset B=0, ALT1=0, ALT2=0, Sreg=R0, Dreg=R0; that does really
apply to ALL other opcodes, namely including JMP/LOOP (unlike Bxx branches),
NOP (ie. NOP isn't exactly <no> operation), MOVE/MOVES (where 1n/Bn are
treated as 'real' opcodes rather than as TO/FROM prefixes).

**Ignored Prefixes**

ALT1/ALT2 prefixes are ignored if the opcode doesn't exist (eg. if "3D xx"
doesn't exist, then the CPU does instead execute "xx") (normally, doing that
wouldn't make any sense, however, "Doom" is using ALT1/ALT2 alongside with
conditional jumps, resulting in situations where the prefix is used/ignored
depending on the jump condition).

ALT3 does reportedly mirror to ALT1 (eg. if "3F xx" doesn't exist, then it acts
as "3D xx", and, if that doesn't either, as "xx").

TO/WITH/FROM are ignored if the following opcode doesn't use Dreg/Sreg.

**Program Counter (R15) Notes**

R15 can be used as source operand in MOV/ALU opcodes (and is also implied as
such in Bxx,LINK,CACHE opcodes); in all cases R15 contains the address of the
next opcode.

### SNES Cart GSU-n CPU Pseudo Opcodes

**Official GSU Pseudo/Macro Opcodes**

```
  --            3 000---- LEA Rn,yyxx    ;Alias for IWT, without "#"
  --            - 000---- MOVE Rn,#hilo  ;Alias for IBT/IWT (depending on size)
  --            - 000---- MOVE Rn,(xx)   ;Alias for LM/LMS (depending on size)
  --            - 000---- MOVE (xx),Rn   ;Alias for SM/SMS (depending on size)
  --            - 000---- MOVEB Rn,(Rm)  ;Alias for LDB/TO+LDB (depending Rn)
  --            - 000---- MOVEB (Rm),Rn  ;Alias for STB/FROM+STB
  --            - 000---- MOVEW Rn,(Rm)  ;Alias for LDW/TO+LDW (depending Rn)
  --            - 000---- MOVEW (Rm),Rn  ;Alias for STW/FROM+STW
```

Above are official pseudo opcodes for native syntax (the nocash syntax "MOV"
opcode is doing that things by default).

**Nocash GSU Pseudo Opcodes**

```
   jmp  nnnn      alias for "mov r15,nnnn"
   jz/jnz/jae/jb  alias for "je/jne/jc/jnc"
```

Further possible pseudo opcodes (not yet supported in a22i):

```
   push rs        mov [r10],rs, 2xinc_r10     ;\INCREASING on PUSH? or MEMFILL?
   pop  rd        2xdec_r10, mov rd,[r10]     ;/ (see Star Fox 1:ACA4)
   cmp  rn,0      alias for "sub rn,rn,0"
   call           alias for link+jmp
   ret            alias for jmp r11
   alu  rd,op     short for "alu rd,rs,op"
   and  rd,rs,n   alias for "bic rd,rs,not n"
```

### SNES Cart GSU-n CPU Misc

**Uncached ROM/RAM-Read-Timings**

```
  ROM Read:   5 cycles per byte at 21MHz, or 3 cycles per byte at 10MHz
  RAM Write: 10 cycles per word at 21MHz, or unknown at 10MHz?
  RAM Write: unknown number of cycles per byte?
  ROM/RAM Opcode-byte-read: 3 cycles at both 21MHz and 10MHz?
```

The uncached timings aren't well documented. Possibly ROM/RAM-byte read/write
are all having the same timing (3/5 clks at 10/21MHz) (and RAM-word 6/10)?

**Jump Notes**

Jumps can be implemented by JMP/Bxx opcodes, or by using R15 as destination
register. In all cases, the next BYTE after the jump opcode is fetched as
opcode byte, and is executed before continuing at the jump-target address.
Possible situations are:

```
  1) jump + NOP                 ;very simple
  2) jump + ONE-BYTE-OPCODE     ;still quite simple
  3) jump + MULTI-BYTE-OPCODE   ;rather strange
  4) Prefix + jump + ONE-BYTE-SUFFIX
  5) Prefix + jump + MULTI-BYTE-SUFFIX
```

In case 3, the first opcode-byte is picked from the address after jump, the
following byte(s) from the jump-destination.

In case 4/5, the prefix is located before the jump, the next byte after the
jump (this works only with Bxx jumps) (whilst JMP/LJMP or MOV/ALU R15,dest do
reset the prefix), and any further bytes at the jump-destination.

**Mistakes in book2.pdf**

BGE/BLT are exchanged with each other. MOVES src/dst operands are exchanged.
LJMP bank/offs operands are exchanged.

**GSU Undoc opcodes**

UMULT #n, WITH, XOR Rn, XOR #n are sorts of undocumented; they should be
described (on page 280), but the alphabetical list ends abruptly after UMULT
Rn. However, they are listed in the summary (page 101) and in the index (page
409). The WITH opcode is also mentioned in various other places.

page 121: R15 after STOP (strange, is that true?) (yes, it is)

page 122: cache/cbr after ABORT

MOV R13,R15  sets R13 to addr of next opcode after MOV (eg. for LOOP start)

LINK n       sets R11 to addr+n of next opcode (eg. for "CALLs" via jmp)

**GSU Power Consumption**

The GSU does (when it is running) increase the power consumption, this can
overload the SNES power supply if additional peripherals are connected. GSU
software should detect which controllers are connected, and refuse to start the
GSU if a controller with high power consumption (or with unknown power
consumption) is connected. The standard joypads are okay. A Multiplayer 5
adaptor isn't okay (at least, when multiple controllers are connected to it).

**After STOP**

Restarting (somewhere(?) after STOP) is possible by setting GO-flag (done by
Dirt Trax FX).

### SNES Cart GSU-n Code-Cache

**ROM/RAM-Code-Cache (512-byte cache)**

This cache is used only for Opcode fetches from ROM or RAM (not for
reading/writing Data to/from ROM nor RAM) (however, it does slightly increase
data access speed in so far that data can be read/written via Gamepak bus,
simultaneously while fetching opcodes from the cache).

32 lines of 16-bytes.

CACHE

LJMP

STOP

ABORT

after STOP, one "must" clear GO by software to clear the cache

**Cache Area**

"SNES_Addr = (CBR AND 1FFh)+3100h". For example, a CACHE opcode at C3A5h will
set CBR to C3A0h, and the (initially empty) cached region will be C3A0h..C59Fh,
when code gets loaded into the cache, GSU:C3A0h..C3FFh shows up at
SNES:32A0h..32FFh, and GSU:C400h..C59Fh at SNES:3100h..329Fh.

**Writing to Code-Cache (by SNES CPU)**

First of, set GO=0 (ie. write SFR=0000h), this forces CBR=0000h, and marks all
cache lines as empty. Then write opcodes 16-byte lines at 3100h..32FFh, writing
the last byte of a line at [3xxFh] will mark the line as not-empty.

Thereafter, the cached code can be excuted (by setting R15 to 0000h..01Fxh), in
this case, the GSU can be operated without RON/RAN flags being set - unless R15
leaves the cached area (this occurs also when a STOP is located in last byte of
last cache line; the hardware tries to prefetch one byte after STOP), or unless
ROM-DATA is accessed (via GETxx opcodes) or unless RAM-DATA is accessed (via
LOAD/STORE or PLOT/RPIX opcodes). Ie. usually one would have RAN set (unless
all incoming/outgoing parameters can be passed though R0..R14 registers).

**Code-Cache Loading Notes**

The 16-byte cache-lines are loaded alongside while executing opcodes (rather
than first loading the whole 16-byte-line, and then executing the opcodes
within it; which would be slightly slower). There are two special cases related
to jumps: If current cache-line isn't fully loaded then hardware keeps loading
the remaining bytes (from jump-origin to end-of-line). If the jump-target isn't
aligned by 16 (and isn't yet cached), then the hardware loads the leading bytes
(from start-of-line to jump-target). After that two steps, normal execution
continues at the jump-target address.

The leading-stuff also occurs on CACHE instruction.

```
  CACHE sets CBR to "R15 AND FFF0h" (whereas R15=address after CACHE opcode)
  LJMP sets CBR to "R15 AND FFF0h" (whereas R15=jump target address)
  SNES write to SFR register with GO=0 sets CBR=0000h
  (all of the above three cases do also mark all cache lines as empty)
```

All Code-Cache lines are marked as empty when executing CACHE or LJMP opcodes,
or when the SNES clears the GO flag (by writing to SFR). The STOP opcode
however (which also clears GO), doesn't empty the cache, so one may eventually
re-use the cached values when restarting the GSU (however, if PBR or code in
GamePak RAM has changed, then one must clear the cache by writing GO=0).

According to cache description (page 132), Cache-Code is 6 times faster than
ROM/RAM. However, according to opcode descriptions (page 160 and up), cache is
only 3 times faster than ROM/RAM. Whereas, maybe 6 times refers to 21MHz mode,
and 3 times to 10MHz mode?

The CACHE opcode is typically executed prior to loops and/or at the begin of
often-used sub-functions (or ideally, the loop and any used subfunctions should
both fit into the 512-byte cache region).

### SNES Cart GSU-n Pixel-Cache

**RAM-Pixel-Write-Cache (two 8-pixel rows)**

pixel cache is flushed when:

```
  1) cache full
  2) doing rpix <--- this does also WAIT until it is flushed
  3) changing r1 or r2 (really?)
```

**Pixel Cache**

Primary Pixel Cache (written to by PLOT)

Secondary Pixel Cache (data copied from Primary Cache, this WAITs if Secondary
cache wasn't yet forwarded to RAM) (if less than 8 flags are set, data is
merged with old RAM data).

Each cache contains 8 pixels (with 2bit/4bit/8bit depth), plus 8 flags
(indicating if (nontransparent) pixels were plotted).

Pixel X/Y coordinates are 8bit wide (using LSBs of R1/R2 registers).

(X and F8h) and (Y and FFh) are memorized, when plotting to different values,
Primary cache is forwarded to Secondary Cache, this happens also when all 8
cache flags are set.

Do not change SCREEN MODE (how to do that at all while GSU is running? SCMR is
writeable for changing RAN/RON, but changing the other SCMR bits during GSU
execution would be rather unpredictable) when data is in pixel caches (use RPIX
to force the caches to be flushed). Before STOP opcode, do also use RPIX to
force the caches to be flushed.

RPIX isn't cached, it does always read data from RAM, not from cache. Moreover,
before reading RAM, RPIX does force both pixel caches (unless they are empty)
to be forwarded to RAM. This is making RPIX very slow (trying to read/modify
pixels via RPIX+PLOT would work very slow). So far, RPIX is mainly useful for
forcing the pixel caches to be forwarded to RAM (and to WAIT until that
forwarding has completed).

### SNES Cart GSU-n Other Caches

**ROM-Read-Data Cache (1-byte read-ahead)**

The cache is used for GETB/GETBS/GETBL/GETBH/GETC opcodes (which do read from
[ROMBR:R14]). Loading the cache is invoked by any opcodes that do change R14
(such like ADD,MOVE,etc.), allowing following GETxx opcodes to be executed
without Waitstates.

In some situations WAITs can occur: When the cache-load hasn't yet completed
(ie. GETxx executed shortly after changing R14), when an opcode is fetched from
ROM (rather than from RAM or Code-Cache), when ROMBR is changed (caution: in
this special case following GETxx will receive [OldROMBR:R14] rather than
[NewROMBR:R14]).

Caution: Do not execute the CACHE opcode shortly (7 cycles in 21MHz mode, or 4
cycles in 10MHz mode) after changing R14 (when doing that, the read from R14
will fail somehow, and following GETxx will return garbage).

Unknown if SNES writes to R14 (via Port 301Ch) do also prefetch [R14] data?

**RAM-Write-Data Cache (1-byte/1-word write queue)**

This cache is used for STB/STW/SM/SMS/SBK opcodes. After any such store
opcodes, the written byte/word is memorized in the cache, and further opcodes
can be fetched (from ROM or from Code-Cache) immediately without Waitstates,
simultaneously with the cached value being forwarded to RAM.

In some situations WAITs can occur: When cache already contained data (ie. when
executing two store opcodes shortly after each other), when an opcode is
fetched from RAM (rather than from ROM or Code-Cache), when the RAMBR register
is changed (this works as expected, it finishes the write to [OldRAMBR:nnnn]).

Results on doing Data-RAM-Reads while the Data-RAM-write is still busy are
unknown (possibly, this will WAIT, too) (or it may return garbage)?

WAITs should also occur when the pixel-cache gets emptied?

**RAM-Address-Cache (1 word) (Bulk Processing for read-modify-write)**

This very simple cache memorizes the most recently used RAM address (from
LM/LMS opcodes, and probably also from LDB/LDW/STB/STW/SM/SMS opcodes; though
some games insert STW to push data on stack, as if they were intended not to
change the memorized address?), the SBK opcode can be used to write a word to
the memorized address (ie. one can avoid repeating immediate operands in SM/SMS
opcodes).

### SNES Cart Capcom CX4 (programmable RISC CPU) (Mega Man X 2-3) (2 games)

**Capcom CX4 - 80pin chip**

Used only by two games:

```
  Mega Man X2 (1994) Capcom (NA) (JP) (EU)   ;aka Rockman X2
  Mega Man X3 (1995) Capcom (NA) (JP)
```

The CX4 chip is actually a Hitachi HG51B169 as confirmed by decapping.

Note: The CX4 is occassionally referred to as C4 (the real chip name is CX4,
the C4 variant is some kind of scene slang).

**CX4 Memory Map**

```
  I/O  00-3F,80-BF:6000-7FFF
  ROM  00-3F,80-BF:8000-FFFF
  SRAM 70-77:0000-7FFF (not installed; reads return 00h)
```

**MISC MISC MISC**

Commands are executed on the CX4 by writing the command to 0x7F4F while bit 6
of 0x7F5E is clear. Bit 6 of 0x7F5E will stay set until the command has
completed, at which time output data will be available.

[Registers]

```
  $7f49-b = ROM Offset
  $7f4d-e = Page Select
  $7f4f = Instruction Pointer
  Start Address = ((Page_Select * 256) + Instruction Pointer) * 2) + ROM_Offset
```

[Memory layout]

```
 Program ROM is obviously 256x16-bit pages at a time. (taken from the SNES ROM)
 Program RAM is 2x256x16-bit. (two banks)    ;<-- uh, that means cache?
 Data ROM is 1024x24-bit. (only ROM internal to the Cx4)
 Data RAM is 4x384x16-bit.                   ;<-- uh, but it HAS 8bit data bus?
 Call stack is 8-levels deep, at least 16-bits wide.
```

**CX4ROM (3Kbytes) (1024 values of 24bit each)**

```
  Index      Name  ;Entry     = Table Contents   = Formula
  -------------------------------------------------------------------------
  000..0FFh  Div   ;N[0..FFh] = FFFFFFh..008080h = 800000h/(00h..FFh)
  100..1FFh  Sqrt  ;N[0..FFh] = 000000h..FF7FDFh = 100000h*Sqrt(00h..FFh)
  200..27Fh  Sin   ;N[0..7Fh] = 000000h..FFFB10h = 1000000h*Sin(0..89')
  280..2FFh  Asin  ;N[0..7Fh] = 000000h..75CEB4h = 800000h/90'*Asin(0..0.99)
  300..37Fh  Tan   ;N[0..7Fh] = 000000h..517BB5h = 10000h*Tan(0..89')
  380..3FFh  Cos   ;N[0..7Fh] = FFFFFFh..03243Ah = 1000000h*Cos(0..89')
```

Sin/Asin/Tan/Cos are spanning only 90' out of 360' degress (aka 80h out of 200h
degrees). Overflows on Div(0) and Cos(0) are truncated to FFFFFFh. All values
are unsigned, and all (except Asin/Tan) are using full 24bits (use SHR opcode
to convert these to signed values with 1bit sign + 23bit integer; for Div one
can omit the SHR if divider>01h).

**CX4 Component List (Megaman X2)**

```
  PCB "SHVC-2DC0N-01, (C)1994 Nintendo"
  U1 32pin P0 8M MASK ROM  (LH538LN4 = 8Mbit)
  U2 32pin P1 4/8 MASK ROM (LH534BN2 or LH5348N2 or so = 4Mbit)
  U3 80pin CX4 (CAPCOM CX4 DL-2427, BS169FB)
  U4 18pin CIC (F411A)
  X1  2pin 20MHz
  J  62pin Cart Edge connector (unknown if any special pins are actually used)
```

**CX4 Component List (Megaman X3)**

```
  PCB "SHVC-1DC0N-01, (C)1994 Nintendo"
  U1 40pin MASK ROM  (TC5316003CF = 16Mbit)
  U2 80pin CX4 (CAPCOM CX4 DL-2427, BS169FB)
  U3 18pin CIC (F411A)
  X1  2pin 20MHz
  J  62pin Cart Edge connector (unknown if any special pins are actually used)
```

**CX4 Cartridge Header (as found in Mega Man X2/X3 games)**

```
  [FFBD]=00h ;expansion RAM size (none) (there is 3KB cx4ram though)
  [FFBF]=10h ;CustomChip=CX4
  [FFD5]=20h ;Slow LoROM (but CX4 opcodes are probably using a faster cache)
  [FFD6]=F3h ;ROM+CustomChip (no battery, no sram)
  [FFD7]=0Bh ;rom size (X2: 1.5MB, rounded-up to 2MB) (X3: real 2MB)
  [FFD8]=00h ;sram size (none) (there is 3KB cx4ram though)
  [FFDA]=33h ;Extended Header (with FFB0h-FFBFh)
```

**ROM Enable**

On SHVC-2DC0N-01 PCBs (ie. PCBs with two ROM chips), the 2nd ROM chip is
reportedly initially disabled, and can be reportedly enabled by setting
[7F48h]=01h (that info doesn't match up with how 7F48h is used by the existing
games; unknown if that info is correct/complete).

**CX4 CPU Misc**

All values are little-endian (opcodes, I/O Ports, cx4rom-ROM-Image, etc).

Call Stack is reportedly 16 levels deep, at least 16bits per level.

Carry Flag is CLEARED on borrow (ie. opposite as on 80x86 CPUs).

**CX4 Timings (Unknown)**

All opcode & DMA timings are 100% unknown. The CX4 is said to be clocked at
20.000MHz, but this might be internally divided, possibly with different
waitstates for different memory regions or different opcodes.

The ROM speed is 2.68Mhz (according to the cartridge header), and 16bit opcodes
are passed through 8bit databus (though one may assume that the CX4 contains an
opcode cache) (cache might be divided into 200h-byte pages, so, far-jumps to
other pages might be slow, maybe/guessed).

The "skip" opcodes are "jumping" to the location after the next opcode (this
probably faster than the actual "jmp" opcodes).

After Multiply opcodes one should insert one "nop" (or another instruction that
doesn't access the MH or ML result registers).

Reading data bytes from SNES ROM requires some complex timing/handling:

```
  612Eh   movb   ext_dta,[ext_ptr]          ;\these 3 opcodes are used to
  4000h   inc    ext_ptr                    ; read one byte from [ext_ptr],
  1C00h   finish ext_dta                    ;/and to increment ext_ptr by 1
```

The exact meaning of the above opcodes is unknown (which one does what part?).

It is also allowed to use the middle opcode WITHOUT the "prepare/wait" part:

```
  4000h   inc    ext_ptr                    ;-increment ext_ptr by 1
```

In that case, "ext_ptr" is incremented, but "ext_dta" should not be used (might
be unchanged, or contain garbage, or receive data after some cycles?).

### SNES Cart Capcom CX4 - I/O Ports

**CX4 I/O Map**

```
  6000h..6BFFh R/W  CX4RAM (3Kbytes)
  6C00h..7F3Fh ?    Unknown/unused
  7F40h..7F42h ?/W  DMA source, 24bit SNES LoROM address
  7F43h..7F44h ?/W  DMA length, 16bit, in bytes (eg. 0800h = 2Kbytes)
  7F45h..7F46h ?/W  DMA destination, 16bit in CX4RAM (6000h = 1st byte)
  7F47h        ?/W  DMA start (write 00h to transfer direction SNES-to-CX4)
  7F48h        ?/W  Unknown "toggle" (set to 00h/01h, maybe cache load/on/off?)
  7F49h..7F4Bh R/W  Program ROM Base, 24bit LoROM addr (028000h in Mega Man)
  7F4Ch        ?/W  Unknown (set to 00h or 01h) soft_reset? maybe flush_cache?
  7F4Dh..7F4Eh ?/W  Program ROM Instruction Page (PC/200h)
  7F4Fh        ?/W  Program ROM Instruction Pointer (PC/2), starts execution
  7F50h..7F51h R/W  Unknown, set to 0144h (maybe config flags or waitstates?)
  7F52h        R/W  Unknown (set to 00h) hard_reset? maybe force stop?
  7F53h..7F5Dh ?    Unknown/unused
  7F5Eh        R/?  Status (bit6=busy, set upon [7F47],[7F48],[7F4F] writes)
  7F5Fh        ?    Unknown/unused
  7F60h..7F69h ?    Unknown/unused (maybe [FFE0..FFE9])
  7F6Ah..7F6Bh R/W  SNES NMI Vector       [FFEA..FFEB]
  7F6Ch..7F6Dh ?    Unknown/unused (maybe [FFEC..FFED])
  7F6Eh..7F6Fh R/W  SNES IRQ Vector       [FFEE..FFEF]
  7F70h..7F7Fh ?    Unknown/unused (maybe [FFF0..FFFF])
  7F80h..7FAFh R/W  Sixteen 24bit CX4 registers (R0..R15, at 7F80h+N*3)
  7FB0h..7FFFh ?    Unknown/unused
  8000h..FFFFh R    ROM (32Kbyte LoROM Banks) (disabled when CX4 is busy)
  FFExh..FFxxh R/?  Exception Vectors (from above I/O Ports, when CX4 is busy)
```

**Exception Vectors**

Unknown if these can be manually enabled, or if they are automatically enabled
when the CX4 is "busy". In the latter case, they would be REQUIRED to be same
as the ROM vectors (else LSB/MSB might be accidently fetched from different
locations when busy-flag changes at same time).

### SNES Cart Capcom CX4 - Opcodes

**CX4 Opcodes (all are 16bit wide)**

```
  Opcode         Clks NZC Syntax
  0000h            ?? ??? nop     ;nop is used as delay after "mul" opcodes
  0400h            ?? ??? -
  0800h+p0aaaaaaaa ?? ??? jmp   addr/prg_page:addr
  0C00h+p0aaaaaaaa ?? ??? jz    addr/prg_page:addr  ;Z=1 (equal)
  1000h+p0aaaaaaaa ?? ??? jc    addr/prg_page:addr  ;C=1 (above/equal)
  1400h+p0aaaaaaaa ?? ??? js    addr/prg_page:addr  ;N=1 (negative)
  1800h            ?? ??? -
  1C00h            ?? ??? finish ext_dta
  2000h            ?? ??? -
  2400h+nn0000000n ?? ??? skip<?/?/nc/c/nz/z/ns/s>  ;skip next opcode
  2800h+p0aaaaaaaa ?? ??? call  addr/prg_page:addr
  2C00h+p0aaaaaaaa ?? ??? callz addr/prg_page:addr  ;Z=1 (equal)
  3000h+p0aaaaaaaa ?? ??? callc addr/prg_page:addr  ;C=1 (above/equal)
  3400h+p0aaaaaaaa ?? ??? calls addr/prg_page:addr  ;N=1 (negative)
  3800h            ?? ??? -
  3C00h            ?? ??? ret
  4000h            ?? ??? inc   ext_ptr
  4400h            ?? ??? -
  4800h+ssoooooooo ?? ??? cmp   <op>,A/A*2/A*100h/A*10000h     ;\
  4C00h+ssoooooooo ?? ??? cmp   <imm>,A/A*2/A*100h/A*10000h    ; compare
  5000h+ssoooooooo ?? NZC cmp   A/A*2/A*100h/A*10000h,<op>     ;
  5400h+ssoooooooo ?? NZC cmp   A/A*2/A*100h/A*10000h,<imm>    ;/
  5800h+ss00000000 ?? ??? mov   A,A.?/lsb/lsw/?                ;-sign-expand
  5C00h            ?? ??? -
  6000h+nnoooooooo ?? ??? mov   A/ext_dta/?/prg_page,<op>
  6400h+nnoooooooo ?? ??? mov   A/?/?/prg_page,<imm>
  6800h+nnoooooooo ?? ??? movb  ram_dta.lsb/mid/msb/?,cx4ram[<op>]
  6C00h+nnoooooooo ?? ??? movb  ram_dta.lsb/mid/msb/?,cx4ram[ram_ptr+<imm>]
  7000h+00oooooooo ?? ??? mov   rom_dta,cx4rom[<op>*3]
  7400h            ?? ??? -
  7800h+0noooooooo ?? ??? mov   prg_page.lsb/msb,<op>
  7C00h+0noooooooo ?? ??? mov   prg_page.lsb/msb,<imm>
  8000h+ssoooooooo ?? ??C add   A,A/A*2/A*100h/A*10000h,<op>   ;\
  8400h+ssoooooooo ?? ?Z? add   A,A/A*2/A*100h/A*10000h,<imm>  ;
  8800h+ssoooooooo ?? ??? sub   A,<op>,A/A*2/A*100h/A*10000h   ; add/subtract
  8C00h+ssoooooooo ?? ??C sub   A,<imm>,A/A*2/A*100h/A*10000h  ;
  9000h+ssoooooooo ?? NZC sub   A,A/A*2/A*100h/A*10000h,<op>   ;
  9400h+ssoooooooo ?? NZC sub   A,A/A*2/A*100h/A*10000h,<imm>  ;/
  9800h+00oooooooo ?? ??? smul  MH:ML,A,<op>    ;\use NOP or other opcode,
  9C00h+00oooooooo ?? ??? smul  MH:ML,A,<imm>   ;/result is signed 48bit
  A000h            ?? ??? -
  A400h            ?? ??? -
  A800h+ssoooooooo ?? ??? xor   A,A/A*2/A*100h/A*10000h,<op>   ;\
  AC00h+ssoooooooo ?? ??? xor   A,A/A*2/A*100h/A*10000h,<imm>  ;
  B000h+ssoooooooo ?? ?Z? and   A,A/A*2/A*100h/A*10000h,<op>   ; logic
  B400h+ssoooooooo ?? ?Z? and   A,A/A*2/A*100h/A*10000h,<imm>  ;
  B800h+ssoooooooo ?? ??? or    A,A/A*2/A*100h/A*10000h,<op>   ;
  BC00h+ssoooooooo ?? ??? or    A,A/A*2/A*100h/A*10000h,<imm>  ;/
  C000h+00oooooooo ?? ??? shr   A,<op>                         ;\
  C400h+00oooooooo ?? NZ? shr   A,<imm>                        ;
  C800h+00oooooooo ?? ??? sar   A,<op>                         ;
  CC00h+00oooooooo ?? N?? sar   A,<imm>                        ; shift/rotate
  D000h+00oooooooo ?? ??? ror   A,<op>                         ;
  D400h+00oooooooo ?? ??? ror   A,<imm>                        ;
  D800h+00oooooooo ?? ??? shl   A,<op>                         ;
  DC00h+00oooooooo ?? N?? shl   A,<imm>                        ;/
  E000h+00oooooooo ?? ??? mov   <op>,A
  E400h            ?? ??? -
  E800h+nnoooooooo ?? ??? movb  cx4ram[<op>],ram_dta.lsb/mid/msb/?
  EC00h+nnoooooooo ?? ??? movb  cx4ram[ram_ptr+<imm>],ram_dta.lsb/mid/msb/?
  F000h+00oooooooo ?? ??? xchg  <op>,A
  F400h            ?? ??? -
  F800h            ?? ??? -
  FC00h            ?? ??? stop          ;stop, and clear Port [FF5E].bit6
```

**Opcode "Middle" 2bits (Bit9-8)**

Selects different parameters for some opcodes (eg. lsb/mid/msb, as shown in
above descriptions).

**Opcode Lower 8bits (Bit7-0)**

Lower Bits <op>:

```
  00h Register A
  01h Register MH       ;multiply.result.upper.24bit (MSBs are sign-expanded)
  02h Register ML       ;multiply.result.lower.24bit (same for signed/unsigned)
  03h Register ext_dta
  08h Register rom_dta
  0Ch Register ram_dta
  13h Register ext_ptr  ;24bit SNES memory address
  1Ch Register ram_ptr
  2Eh Special  snesrom[ext_ptr] (?)  ;for use by opcode 612Eh only (?)
  50h Constant 000000h
  51h Constant FFFFFFh
  52h Constant 00FF00h
  53h Constant FF0000h
  54h Constant 00FFFFh
  55h Constant FFFF00h
  56h Constant 800000h
  57h Constant 7FFFFFh
  58h Constant 008000h
  59h Constant 007FFFh
  5Ah Constant FF7FFFh
  5Bh Constant FFFF7Fh
  5Ch Constant 010000h
  5Dh Constant FEFFFFh
  5Eh Constant 000100h
  5Fh Constant 00FEFFh
  6xh Register R0..R15, aka Port [7F80h+x*3] ;(x=0h..Fh)
```

Lower Bits <imm>:

```
  nnh Immediate 000000h..0000FFh (unsigned)
```

Lower Bits jump/call<addr>:

```
  nnh Program Counter LSBs (within 256-word page) (absolute, non-relative)
```

Lower Bits skip<cond>:

```
  00h Skip next opcode if selected flag is zero (conditions ?/nc/nz/ns)
  01h Skip next opcode if selected flag is set  (conditions ?/c/z/s)
```

Lower Bits for opcodes that don't use them (uuuuuuuu):

```
  00h Unused, should be zero
```

### SNES Cart Capcom CX4 - Functions

**CX4 Functions (as contained in Mega Man X2/X3 ROMs)**

The CX4 functions are located at SNES address 02:8000-02:9FFF (aka CX4
addresses at PAGE:PC=0000:00..000F:FF with BASE=028000):

```
  PAGE:PC__Function_____________________________
  0000:00  build_oam
  0001:00  scale_tiles  ;<-- (seems to be unused by Mega Man games)
  0002:00  hires_sqrt   ;<-- (seems to be unused by Mega Man games)
  0002:03  sqrt         ;<-- (seems to be unused by Mega Man games)
  0002:05  propulsion
  0002:07  get_sin      ;<-- (seems to be unused by Mega Man games)
  0002:0A  get_cos      ;<-- (seems to be unused by Mega Man games)
  0002:0D  set_vector_length
  0002:10  triangle1
  0002:13  triangle2
  0002:15  pythagorean
  0002:1F  arc_tan
  0002:22  trapeziod
  0002:25  multiply
  0002:2D  transform_coordinates
  0003:00  scale_rotate1
  0005:00  transform_lines
  0007:00  scale_rotate2
  0008:00  draw_wireframe_without_clearing_buffer
  0008:01  draw_wireframe_with_clearing_buffer
  000B:00  disintergrate
  000C:00  wave
  000E:00  test_set_r0_to_00h ;\sixteen 4-word functions,
  ...      ...                ; located at 000E:00+4*(0..15)
  000E:3C  test_set_r0_to_0Fh ;/setting R0 to 00h..0Fh
  000E:40  test_2K_ram_chksum
  000E:54  test_square           ;R1:R2 = R0*R0
  000E:5C  test_immediate_register  ;copy 16 cpu constants to 30h-bytes RAM
  000E:89  test_3K_rom_chksum  ;"immediate_rom"
```

Both Mega Man X2 and X3 are containing 1:1 the same CX4 code (the only two
differences are different ROM bank numbers for "Wireframe" vertices):

```
  Mega Man X2:  [0008:3B]="or a,a,28h"  [000A:C4]="mov a,28h"  ;ROM bank 28h
  Mega Man X3:  [0008:3B]="or a,a,08h"  [000A:C4]="mov a,08h"  ;ROM bank 08h
```

That differences apply to the US/Canada versions. There <might> be
further differences in Japanese and/or European versions(?)

### SNES Cart DSP-n/ST010/ST011 (pre-programmed NEC uPD77C25 CPU) (23 games)

**Nintendo DSP-n Chips**

The DSP-n chips are 28pin NEC uPD77C25 CPUs with internal ROM/RAM. There are
six versions:

```
  DSP-1, DSP-1A, DSP-1B, DSP-2, DSP-3, DSP-4
```

DSP-1 and DSP-1A contain exactly the same Program/Data ROM. DSP-1B contains a
bug-fixed DSP1/1A version. DSP2/3/4 contain custom ROMs.

**Seta ST010/ST011 Chips**

These are 64pin chips, containing a slightly extended NEC uPD77C25 with more
ROM and RAM, faster CPU clock.

```
  64pin  SETA ST010 D96050CW-012 (PCB SHVC-1DS0B-01)
  64pin  SETA ST011 D96050CW-013 (PCB SHVC-1DS0B-10; with extra transistor)
```

The onchip RAM is battery-backed and is accessible directly via SNES address
bus.

**NEC uPD77C25 Specs**

**Game specific info**

**DSPn/ST010/ST011 Cartridge Header**

For DSPn Cartridges:

```
  [FFD6h]=03h..05h   Chipset = DSPn (plus battery present/absent info)
```

For ST010/ST011 Cartridges:

```
  [FFD6h]=F6h   Chipset = Custom (plus battery; for the on-chip RAM)
  [FFD4h]=00h   Last byte of Title=00h (indicate early extended header)
  [FFBFh]=01h   Chipset Sub Type = ST010/ST011
```

Note: The uPD77C25's ROM/RAM aren't counted in the ROM Size, ROM Checksum, SRAM
Size (nor Expansion RAM Size) entries. The header (nor extended header)
includes no info whether a DSPn game uses a DSP1, DSP2, DSP3, or DSP4, and no
info if a ST010/ST011 game uses ST010 or ST011. Ideally, the uPD77C25 ROM-Image
should be appended at the end of the SNES ROM-Image. In practice, it's often
not there, so there's no way to detect if the game uses this or that uPD77C25
ROM (except for using a list of known Titles or Checksums).

### SNES Cart DSP-n/ST010/ST011 - NEC uPD77C25 - Registers & Flags & Overview

**DSP Mapping**

```
  LoROM Mapping:
  DSP       PCB           Mode  ROM RAM Bank       Data (DR)     Status (SR)
  DSP1/DSP4 SHVC-1B0N-01  LoROM 1M  -   30h-3Fh    8000h-BFFFh   C000h-FFFFh
  DSP2      SHVC-1B5B-01  LoROM 1M  32K 20h-3Fh    8000h-BFFFh   C000h-FFFFh
  DSP3      SHVC-1B3B-01  LoROM 1M  8K  20h-3Fh    8000h-BFFFh   C000h-FFFFh
  DSP1      SHVC-2B3B-01  LoROM 2M  8K  60h-6Fh    0000h-3FFFh   4000h-7FFFh
  ST010     SHVC-1DS0B-01 LoROM 1M  -   60h-6xh    0000h         0001h
  ST011     SHVC-1DS0B-10 LoROM 512K-   60h-6xh    0000h         0001h
  HiROM Mapping:
  DSP   PCB          Mode  ROM RAM Bank            Data (DR)     Status (SR)
  DSP1  SHVC-1K0N-01 HiROM  4M   - 00h-1Fh         6000h-6FFFh   7000h-7FFFh
  DSP1  SHVC-1K1B-01 HiROM  4M  2K 00h-1Fh         6000h-6FFFh   7000h-7FFFh
  DSP1B SHVC-1K1X-01 HiROM  4M  2K 00h-0Fh,20h-2Fh 6000h-6FFFh   7000h-7FFFh
  DSP1B SHVC-2K1X-01 HiROM  2M  2K 00h-0Fh,20h-2Fh 6000h-6FFFh   7000h-7FFFh
  DSP1B SHVC-2K3X-01 HiROM  2M  8K 00h-0Fh,20h-2Fh 6000h-6FFFh   7000h-7FFFh
  SFC-Box:
  DSP   PCB          Mode  ROM RAM Bank            Data (DR)     Status (SR)
  DSP1? GS 0871-102  <Might have variable LoROM/HiROM mapping supported?>
```

Some of the above PCB names seem to be nonsense (eg. DSP with 2MbyteLoROM
hasn't ever been produced, except as prototype board).

**SNES I/O Ports**

```
  Type                 DR               SR                SRAM
  DSPn+LoROM (1MB)     30-3F:8000-BFFF  30-3F:C000-FFFF   None
  DSPn+LoROM (1MB+RAM) 20-3F:8000-BFFF  20-3F:C000-FFFF   70-7D:0000-7FFF
  DSPn+LoROM (2MB+RAM) 60-6F:0000-3FFF  60-6F:4000-7FFF   70-7D:0000-7FFF
  DSPn+HiROM           00-1F:6000-6FFF  00-1F:7000-7FFF   20-3F:6000-7FFF ?
  DSPn+HiROM (MAD-2)   00-0F:6000-6FFF  00-0F:7000-7FFF   30-3F:6000-7FFF ?
  ST010/ST011+LoROM    60-6x:0000       60-6x:0001        68-6F:0000-0FFF
```

All banks in range 00-7F are also mirrored to 80-FF. The "LoROM (2MB)" type
wasn't actually produced (but is defined in Nintendo's specs, see book1.pdf
page 52).

For ST010/ST011, the RAM is contained in the ST01n chip, and is sized 2Kx16bit,
whereas the SNES accesses it as 4Kx8bit (even addresses accessing the LSB, odd
ones the MSB of the 16bit words).

**Registers**

```
  DP          8-bit Data RAM Pointer               (ST010/11: 11-bit)
  RP          10-bit Data ROM Pointer              (ST010/11: 11-bit)
  PC          11-bit Program ROM Counter           (ST010/11: 14-bit)
  STACK       11-bit x 4-levels (for call/ret/irq) (ST010/11: 14-bit x 8-level)
  K,L         two 16bit registers (multiplier input)
  AccA,AccB   two 16bit registers (ALU accumulators) (aka A and B)
  FlagA,FlagB two 6bit registers with S1,S0,C,Z,OV1,OV0 flags for AccA/AccB
  TR,TRB      two 16bit registers (temporary storage)
  SR          16bit status I/O register
  DR          parallel I/O data (selectable 8bit/16bit via SR's DRC bit)
  SI,SO       serial I/O data (selectable 8bit/16bit via SR's SOC,SIC bits)
```

**FlagA/FlagB**

```
  S0  Sign Flag     (set if result.bit15)
  Z   Zero Flag     (set if result=0000h)
  C   Carry Flag    (set if carry or borrow)
  OV0 Overflow Flag (set if result>+7FFFh or result<-8000h)
  S1  Direction of Last Overflow (if OV0 then S1=S0, else S1=unchanged)
  OV1 Number of Overflows (0=even, 1=odd) (inverted when OV0 gets set)
```

S0,Z,C,OV0 are "normal" flags as used by various CPUs. S1,OV1 are specials for
use with JSA1/JSB1/JNSA1/JNSB1 and JOVA1/JOVB1/JNOVA1/JNOVB1 conditional jump
opcodes, or with SGN operand (which equals 8000h-SA1). Examples:

```
  or  a,a      ;SA1=A.Bit15 (undocumented)     ;\officially     ;\No Addition
  mov l,sgn    ;L=8000h-SA1 (but used by DSP1) ;/SA1=Undefined  ;/
  mov a,val1                                                    ;\
  add a,val2   ;affect OVA0 (and, if OVA0 set, also SA1)        ; Adding
  jnova0 skip0 ;test OVA0                                       ; Two Values
  mov a,sgn    ;A=8000h-SA1 (saturate max=+7FFFh, min=-8000h)   ;
 skip0:                                                         ;/
  ;below works with up to three 16bit values,                   ;\
  ;would also work with hundreds of small 8bit values,          ;
  ;ie. works if multiple overflows occur in opposite directions ;
  ;but doesn't work if two overflows occur in same direction)   ; Adding
  xor a,a      ;clear OVA1                                      ; More Values
  add a,val1   ;no overflow OVA1 yet                            ;
  add a,val2   ;this may set OVA1                               ;
  add a,val3   ;this may set/reset OVA1                         ;
  jnova1 skip1 ;test OVA1 (skip if 0 or 2 overflows occurred)   ;
  mov a,sgn    ;A=8000h-SA1 (done if 1 overflow occurred)       ;
 skip1:                                                         ;/
```

Note: The JSA1/JSB1/JNSA1/JNSB1 and JOVA1/JOVB1/JNOVA1/JNOVB1 opcodes aren't
used by any of the DSP1-DSP4 or ST010-ST011 games. The SGN operand is used only
by DSP1/DSP1A/DSP1B (once in conjuntion with JNOVA0, which seems to be ALWAYS
skipping the SGN-part, and once in conjunction with an OR opcode, which loads
an undocumented value to SA1).

**Status Register (SR)**

```
  15    RQM (R)  Request for Master (0=Busy internally, 1=Request external I/O)
  14-13 USF1-0   User's Flags (general purpose)        (0=Low, 1=High)
  12    DRS (R)  DR Status (for 16bit DR mode; 2x8bit) (0=Ready, 1=Busy)
  11    DMA      Direct Memory Access Mode             (0=Non-DMA, 1=DMA)
  10    DRC      DR Control, parallel data length      (0=16bit, 1=8bit)
  9     SOC      SO Control, serial data output length (0=16bit, 1=8bit)
  8     SIC      SI Control, serial data input length  (0=16bit, 1=8bit)
  7     EI       Interrupt Enable                      (0=Disable, 1=Enable)
  6-2   N/A (R?) Unused/Reserved  (should be zero) (read=always zero?)
  1-0   P1-0     Output to P0,P1 pins (0=Low, 1=High)
```

SR.Bit15-8 are output to D7-0 pins (when /CS=LOW, /RD=LOW, /WR=HIGH, A0=HIGH).

SR.Bit1-0 are always output to P1-0 pins.

**RQM**

RQM gets set when the uPD77C25 does read/write its DR register, and gets
cleared when the remote CPU does complete reading/writing DR (complete: when
DRC=8bit, or when DRC=16bit and DRS=Ready). DRS gets toggled in 16bit mode
after each 8bit fragment being read/written by remote CPU (the fragments are
transferred LSB first, then MSB).

**DMA**

The DRQ-pin isn't connected in SNES cartridges (since the DMA protocol isn't
compatible with the SNES), so there's no real DMA support. However, the
uPD77C25 (or at least the ST011) is fast enough to handle SNES-DMA transfers by
software. Software for DSP2 uses the 65C816's block-transfer command (which is
a bit slower than DMA, but, like DMA, doesn't use handshaking).

**Memory**

```
  2048 x 24bit Instruction ROM/PROM Opcodes
  2048 x 1bit  Instruction PROM Protection Flags (0=Lock, 1=Allow Dumping)
  1024 x 16bit Data ROM/PROM
  256 x 16bit  Data RAM
```

**ROM-Images**

DSPn/ST010/ST011 ROM-images consist of the Program ROM followed by the Data
ROM. Caution: There are several differently formatted dumps of these ROMs:

```
  Oldest Files  10K (DSPn)               --> Big-Endian, 24bit-to-32bit padding
  Old Files     8K (DSPn) or 52K (ST01n) --> Big-Endian, raw 24bit opcodes
  Newer Files   8K (DSPn) or 52K (ST01n) --> Little-Endian, raw 24bit opcodes
```

Preferred would be the "Newer" format. To detect the endianness: All existing
ROMs contain "JRQM $" within first four opcodes (97C00xh, with x=0/4/8/C
depending on whether it is 1st/2nd/3rd/4th opcode). Ie. possible cases are:

```
  Oldest Files  97h,C0h,0xh,FFh  ;big-endian 24bit, plus FFh-padding byte
  Old Files     97h,C0h,0xh      ;big-endian 24bit, without padding
  Newer Files   0xh,C0h,97h      ;little-endian 24bit, without padding
```

If the "JRQM $" opcode doesn't exist, best default to "Newer" format (that
might happen only with uncommon homebrewn DSP ROMs, not with the original
ROMs).

Ideally, the ROM-Image should be attached at the end of the SNES Cartridge
ROM-Image (when doing that, best remove any 200h-byte header, since the
existing headers don't define if/how to adjust their size-entries for
DSPn/ST01n ROMs).

**Chips**

```
  uPD77C25   Mask ROM
  uPD77P25   Programmable PROM/UVEPROM
```

**uPD77C25 Opocde Encoding**

All opcodes are 24bit wide. All opcodes are executed in one clock cycle (at
max=8.192MHz clock).

```
   23 22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
  +--+--+-----+-----------+--+-----+-----------+--+-----------+-----------+
  |0 |RT|  P  | ALU opcode|A | DPL |    DPH    |RP|    SRC    |    DST    | ALU
  +--+--+-----+-----------+--+-----+-----------+--+-----------+-----+-----+
  |1 |0 |    BRCH (jump opcode)    |    NA (11bit Next Address)     |  -  | JP
  +--+--+--------------------------+--------------------+-----+-----+-----+
  |1 |1 |            ID (16bit Immediate Data)          |  -  |    DST    | LD
  +--+--+-----------------------------------------------+-----+-----------+
```

**uPD77C20 Opocde Encoding (older pre-77C25 version) (not used in SNES)**

All opcodes are 23bit wide. All opcodes are executed in one clock cycle (at
max=4.xxxMHz clock).

```
   22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
  +--+--+-----+-----------+--+-----+--------+--+-----------+-----------+
  |0 |RT|  P  | ALU opcode|A | DPL |  DPH   |RP|    SRC    |    DST    | ALU
  +--+--+-----+-----------+--+-+---+--------+--+-----------+-----+-----+
  |1 |0 |  BRCH (jump opcode)  |  NA (9bit Next Address)   |     -     | JP
  +--+--+----------------------+--------------------+---+--+-----+-----+
  |1 |1 |            ID (16bit Immediate Data)          |- |    DST    | LD
  +--+--+-----------------------------------------------+--+-----------+
```

DPH is only 3bit (M0..M7), BRCH is only 8bit (without JDPLN0, JDPLNF opcodes).
Data ROM entries are only 13bit wide (sign-expanded to 16bit... or left-shifted
to 16bit?). NA is only 9bit. Internal clock is only 4MHz. TRB register is not
supported.

### SNES Cart DSP-n/ST010/ST011 - NEC uPD77C25 - ALU and LD Instructions

**ALU Instructions (Artithmetic/Logical Unit)**

```
  23    0   Must be 0 for ALU opcodes
  22    RT  Return after ALU (0=No/Normal, 1=Yes/Return from Call/Interrupt)
  21-20 P   ALU input P (0=RAM[DP], 1=IDB(SRC), 2=K*L*2/10000h, 3=K*L*2)
  19-16 ALU ALU opcode
  15    A   ALU input/output Q (0=AccA, 1=AccB)
  14-13 DPL Data RAM Pointer DP.3-0 adjust (0=DPNOP, 1=DPINC, 2=DPDEC, 3=DPCLR)
  12-9  DPH Data RAM Pointer DP.7-4 adjust (0..0Fh=M0..MF) (XOR by that value)
  8     RP  Data ROM Pointer RP.9-0 adjust (0=RPNOP, 1=RPDEC)
  7-4   SRC Source (copied to DST, and, for ALU, to "IDB" internal data bus)
  3-0   DST Destination (copied from SRC)
```

Allows to combine an ALU operation with memory load & store, DP/RP pointer
adjustment, optional RET from CALL, along with the K*L*2 multiplication.

**LD Instructions (Load)**

```
  23    1   Must be 1 for LD opcodes
  22    1   Must be 1 for LD opcodes
  21-6  ID  16bit Immediate
  5-4   -   Reserved (should be zero)
  3-0   DST Destination (copied from ID)
```

Load is mainly for initialization purposes & other special cases (normally
it's faster load immediates from Data ROM, via SRC=RO in ALU opcodes).

**ALU Opcode (Bit19-16)**

```
  Hex Name      ;Expl.                      S1  S0  Cy  Zf OV1 OV0
  00h NOP       ;No operation               -   -   -   -  -   -
  01h OR        ;Acc = Acc OR P             sf  sf  0   zf 0   0
  02h AND       ;Acc = Acc AND P            sf  sf  0   zf 0   0
  03h XOR       ;Acc = Acc XOR P            sf  sf  0   zf 0   0
  04h SUB       ;Acc = Acc - P              *   sf  cy  zf *   ov
  05h ADD       ;Acc = Acc + P              *   sf  cy  zf *   ov
  06h SBB       ;Acc = Acc - P - OtherCy    *   sf  cy  zf *   ov
  07h ADC       ;Acc = Acc + P + OtherCy    *   sf  cy  zf *   ov
  08h DEC       ;Acc = Acc - 1              *   sf  cy  zf *   ov
  09h INC       ;Acc = Acc + 1              *   sf  cy  zf *   ov
  0Ah NOT       ;Acc = Acc XOR FFFFh        sf  sf  0   zf 0   0
  0Bh SAR1      ;Acc = Acc/2    ;signed     sf  sf  cy  zf 0   0
  0Ch RCL1      ;Acc = Acc*2 + OtherCy      sf  sf  cy  zf 0   0
  0Dh SLL2      ;Acc = Acc*4 + 3            sf  sf  0   zf 0   0
  0Eh SLL4      ;Acc = Acc*16 + 15          sf  sf  0   zf 0   0
  0Fh XCHG      ;Acc = Acc ROL 8            sf  sf  0   zf 0   0
```

ADD/ADC/SUB/SBB/INC/DEC set "S1=sf" and "OV1=OV1 XOR 1" upon overflow (and
leave S1 and OV1 both unchanged if no overflow).

OtherCy is the incoming carry flag from other accumulator (ie. Cy from FlagA
when using AccB).

Note: "NOT" is called "CMP"=Complement in official syntax; it isn't Compare.
"SAR/RCL/SLL" are called "SHR/SHL/SHL" in official syntax; though they aren't
"normal" logical shifts. OR/AND/XOR/NOT/SAR/RCL/SLL/XCHG do offially set
S1=Undefined (but actually it seems to be S1=sf; required for loopings in
"Super Air Diver 2"; which uses OR opcode followed by SGN operand).

**Multiplier**

After each instruction (namely after any ALU and LD instructions that have
changed K or L registers), the hardware computes K*L*2 (signed
16bit*16bit->32bit). Result on overflows (-8000h*-8000h*2) is unknown.

**SRC Field (Bit7-4) and DST Field (Bit3-0)**

```
  Hex   SRC (Source, Bit7-4)           DST (Destination, Bit3-0)
  00h   TRB  (Temporary B)             @NON (none)
  01h   A    (AccA)                    @A   (AccA)
  02h   B    (AccB)                    @B   (AccB)
  03h   TR   (Temporary A)             @TR  (Temporary A)
  04h   DP   (Data RAM Pointer)        @DP  (Data RAM Pointer)
  05h   RP   (Data ROM Pointer)        @RP  (Data ROM Pointer)
  06h   RO   (ROM[RP])                 @DR  (parallel I/O port)
  07h   SGN  (saturation = 8000h-SA1)  @SR  (status register)
  08h   DR   (parallel I/O port)       @SOL (SO serial LSB first)
  09h   DRNF (DR without RQM/DRQ)      @SOM (SO serial MSB first)
  0Ah   SR   (status register)         @K   (Multiply Factor A)
  0Bh   SIM  (SI serial MSB first)     @KLR (K=SRC and L=ROM[RP])
  0Ch   SIL  (SI serial LSB first)     @KLM (L=SRC and K=RAM[DP OR 40h])
  0Dh   K    (Multiply Factor A)       @L   (Multiply Factor B)
  0Eh   L    (Multiply Factor B)       @TRB (Temporary B)
  0Fh   MEM  (RAM[DP])                 @MEM (RAM[DP])
```

When not using SRC: specify "NON" in source code, and 00h (a dummy TRB fetch)
in binary code.

Following combinations are prohibited in ALU instructions:

```
  DST field = @KLR or @KLM combined with SRC field = K or L register
  DST field and SRC field specify the same register
  P-SELECT field = RAM, DST field = @MEM (for ALU operation)
```

Everything else should be allowed (included ALU with SRC=Acc, eg ADD AccA,AccA)

**Variants**

The older uPD77C20 doesn't support TRB: SRC=00h is NON (=zero/undefined?),
DST=0Eh (and also DST=00h) is @NON. Opcodes are only 23bit wide (strip
ALU.Bit11/MSB of DPH, and LD.Bit5/Reserved).

### SNES Cart DSP-n/ST010/ST011 - NEC uPD77C25 - JP Instructions

**JP Instructions (Jump/Call)**

```
  23    1   Must be 1 for JP opcodes
  22    0   Must be 0 for JP opcodes
  21-13 BRC Jump/Call opcode
  12-2  NA  11bit Next Address Bit0-10 (000h..7FFh, in 24-bit word steps)
  1-0   -   Reserved (should be zero) (ST010/ST011: Bit12-11 of NA)
```

**BRCH Opcode (Bit21-13)**

```
  Binary    Hex  Op      Expl.
  000000000 000h JMPSO * Unconditional jump to SO register
  100000000 100h JMP     Unconditional jump 0000h..1FFFh
  100000001 101h JMP   * Unconditional jump 2000h..3FFFh
  101000000 140h CALL    Unconditional call 0000h..1FFFh  ;\return via RT-bit
  101000001 141h CALL  * Unconditional call 2000h..3FFFh  ;/in ALU opcodes
  010000000 080h JNCA    CA = 0         ;\
  010000010 082h JCA     CA = 1         ; carry flag of AccA/AccB
  010000100 084h JNCB    CB = 0         ;
  010000110 086h JCB     CB = 1         ;/
  010001000 088h JNZA    ZA = 0         ;\
  010001010 08Ah JZA     ZA = 1         ; zero flag of AccA/AccB
  010001100 08Ch JNZB    ZB = 0         ;
  010001110 08Eh JZB     ZB = 1         ;/
  010010000 090h JNOVA0  OVA0 = 0       ;\
  010010010 092h JOVA0   OVA0 = 1       ; overflow flag for last operation
  010010100 094h JNOVB0  OVB0 = 0       ;
  010010110 096h JOVB0   OVB0 = 1       ;/
  010011000 098h JNOVA1  OVA1 = 0       ;\
  010011010 09Ah JOVA1   OVA1 = 1       ; overflow flag for last 3 operations
  010011100 09Ch JNOVB1  OVB1 = 0       ; (set if 1 or 3 overflows occurred)
  010011110 09Eh JOVB1   OVB1 = 1       ;/
  010100000 0A0h JNSA0   SA0 = 0        ;\
  010100010 0A2h JSA0    SA0 = 1        ; sign bit (ie. Bit15) of AccA/AccB
  010100100 0A4h JNSB0   SB0 = 0        ;
  010100110 0A6h JSB0    SB0 = 1        ;/
  010101000 0A8h JNSA1   SA1 = 0        ;\
  010101010 0AAh JSA1    SA1 = 1        ; extra sign bit ("Bit16")
  010101100 0ACh JNSB1   SB1 = 0        ; indicating direction of overflows
  010101110 0AEh JSB1    SB1 = 1        ;/
  010110000 0B0h JDPL0   DPL = 00h      ;\
  010110001 0B1h JDPLN0  DPL <> 00h     ; lower 4bit of DP (Data RAM Pointer)
  010110010 0B2h JDPLF   DPL = 0Fh      ;
  010110011 0B3h JDPLNF  DPL <> 0Fh     ;/
  010110100 0B4h JNSIAK  SI ACK = 0     ;\
  010110110 0B6h JSIAK   SI ACK = 1     ; serial I/O port (SI/SO serial in/out)
  010111000 0B8h JNSOAK  SO ACK = 0     ;
  010111010 0BAh JSOAK   SO ACK = 1     ;/
  010111100 0BCh JNRQM   RQM = 0        ;\parallel I/O port (DR data register)
  010111110 0BEh JRQM    RQM = 1        ;/
```

Jump addresses should be specified 24bit word units (not in byte units).

(*) Opcodes 000h,101h,141h supported on ST010/ST011 only. On that CPU, PC.bit13
can be manipulated by unconditional jump/call/ret (whilst conditional jumps
affect only PC.bit12-0).

**Reset (Vector 000h)**

Reset is triggered when RST pin is high, and does set PC=000h, FlagA=00h,
FlagB=00h, SR=0000h, DRQ=0, SORQ=0, SI.ACK=0, SO.ACK=0, and RP=3FFh. Other
registers and Data RAM are left unchanged.

**Interrupts (Vector 100h)**

Interrupts are triggered on raising edge of INT pin. If interrupts are enabled
(in SR register), the CPU jumps to address 100h, and pushes PC on stack, the
interrupts are NOT automatically disabled.

**Variants**

The older uPD77C20 doesn't support JDPLN0 and JDPLNF. Opcodes are only 23bit
wide (strip JP.Bit13/LSB of BRCH). And PC/NA is only 9bit wide (replace
JP.Bit3-2 by Reserved).

### SNES Cart DSP-n/ST010/ST011 - List of Games using that chips

**Games using DSPn/ST01n chips**

The DSP-1/1A/1B is used by around 16..19 games:

```
  Ace Wo Nerae! 3D Tennis (DSP-1A) (1993) Telenet Japan (JP)
  Armored Trooper Votoms: The Battling Road (1993) Takara (JP)
  Ballz 3D, and 3 Jigen Kakutou Ballz (DSP-1B) (1994) PF Magic/Accolade (NA)
  Battle Racers (1995) Banpresto (JP)
  Bike Daisuki! Hashiriya Kon - Rider's Spirits (1994) Genki/NCS (JP)
  Final Stretch (1993) Genki/LOZC (JP)
  Korean League (aka Hanguk Pro Yagu) (1993) Jaleco (KO)
  Lock-On / Super Air Diver (1993) Vic Tokai
  Michael Andretti's Indy Car Challenge (1994) Genki/Bullet-Proof (NA) (JP)
  Pilotwings (1991) Nintendo EAD (NA) (JP) (EU) (DSP-1) (visible DSP1 glitch)
  Shutokou Battle'94: K.T. Drift King (1994) Genki/Bullet-Proof (JP)
  Shutokou Battle 2: Drift King K.T. & M.B. (1995) Genki/Bullet-Proof (JP)
  Super 3D Baseball (?) (is that same as Super Bases Loaded 2 ?)
  Super Air Diver 2 (1995) Asmik (JP)
  Super Bases Loaded 2 (1994) Jaleco (NA) (JP)
  Super F1 Circus Gaiden (1995) Nichibutsu (JP)
  Super Mario Kart (DSP-1/DSP-1B) (1992) Nintendo EAD (NA) (JP) (EU)
  Suzuka 8 Hours (1993) Namco (NA) (JP)
  Touge Densetsu: Saisoku Battle (1996) Genki/Bullet-Proof Software (JP) ?
```

The other five versions are used by only one game each:

```
  DSP-2: Dungeon Master (DSP-2) (1992) FTL Games/JVC Victor (JP)
  DSP-3: SD Gundam GX (DSP-3) (1994) BEC/Bandai (JP)
  DSP-4: Top Gear 3000 (DSP-4) (1995) Gremlin Interactive/Kemco (NA) (JP) (EU)
  ST010: F1 Race of Champions / Exhaust Heat II (1993) SETA Corp. (NA) (JP)
  ST011: Hayazashi Nidan Morita Shogi (1993) Random House/SETA Corp. (JP)
```

### SNES Cart DSP-n/ST010/ST011 - BIOS Functions

**DSP1 Commands**

When requesting data from an external device the DSP is oblivious to the type
of operation that occurs to the Data Register. Writing to the Data register
will update the contents of the register and allow the DSP to continue
execution. Reading from the Data Register will also allow the DSP to continue
execution. On completion of a valid command the Data Register should contain
the value 0x80. This is to prevent a valid command from executing should a
device read past the end of output.

```
  00h  16-bit Multiplication
  10h  Inverse Calculation
  20h  16-bit Multiplication
  01h  Set Attitude A
  11h  Set Attitude B
  21h  Set Attitude C
  02h  Projection Parameter Setting
  03h  Convert from Object to Global Coordinate A
  13h  Convert from Object to Global Coordinate B
  23h  Convert from Object to Global Coordinate C
  04h  Trigonometric Calculation
  14h  3D Angle Rotation
  06h  Object Projection Calculation
  08h  Vector Size Calculation
  18h  Vector Size Comparison
  28h  Vector Absolute Value Calculation (bugged) (fixed in DSP1B)
  38h  Vector Size Comparison
  0Ah  Raster Data Calculation
  0Bh  Calculation of Inner Product with the Forward Attitude A and a Vector
  1Bh  Calculation of Inner Product with the Forward Attitude B and a Vector
  2Bh  Calculation of Inner Product with the Forward Attitude C and a Vector
  0Ch  2D Coordinate Rotation
  1Ch  3D Coordinate Rotation
  0Dh  Convert from Global to Object Coordinate A
  1Dh  Convert from Global to Object Coordinate B
  2Dh  Convert from Global to Object Coordinate C
  0Eh  Coordinate Calculation of a selected point on the Screen
  0Fh  Test Memory Test
  1Fh  Test Transfer DATA ROM
  2Fh  Test ROM Version (0100h=DSP1/DSP1A, 0101h=DSP1B)
```

Command 28h is bugged in DSP1/DSP1A (fixed in DSP1B) bug is evident in
Pilotwings (Plane Demo).

**DSP2 Commands (Dungeon Master)**

This chip does - amazingly - assist 3D labyrinth drawing operations that are
normally implemented on ZX81 computers.

```
  01h  Convert Bitmap to Bitplane Tile
  03h  Set Transparent Color
  05h  Replace Bitmap using Transparent Color
  06h  Reverse Bitmap
  07h  Add
  08h  Subtract
  09h  Multiply (bugged) (used in Dungeon Master japanese/v1.0)
  0Dh  Scale Bitmap
  0Fh  Process Command (dummy NOP command for re-synchronisation)
  10h..FFh Mirrors of 00h..0Fh
```

**DSP3 Commands (SD Gundam GX)**

The DSP functions inherently similiar to the DSP1 with respect to command
parsing and execution. On completion of a valid command the Data Register
should contain the value 0x80.

```
  02h  Unknown
  03h  Calculate Cell Offset
  06h  Set Board Dimensions
  07h  Calculate Adjacent Cell
  18h  Convert Bitmap to Bitplane
  38h  Decode Shannon-Fano Bitstream (USF1 bit in SR register = direction)
  1Eh  Calculate Path of Least Travel
  3Eh  Set Start Cell
  0Fh  Test Memory Test
  1Fh  Test Transfer DATA ROM
  2Fh  Test ROM Version (0300h=DSP3)
```

**DSP4 Commands (Top Gear 3000)**

On completion of a valid command the Data Register should contain the value
0xffff. This is to prevent a valid command from executing should an external
device read past the end of output. Unlike previous DSP programs, all data
transfers are 16-bit.

```
  xxh      Unknown
  13h      Test Transfer DATA ROM
  14h      Test ROM Version (0400h=DSP4)
  15h..1Fh Unused (no function)
  20h..FFh Mirrors of 10h..1Fh
```

**ST010 Commands**

Commands are executed on the ST-0010 by writing the command to 0x0020 and
setting bit7 of 0x0021. Bit7 of 0x0021 will stay set until the Command has
completed, at which time output data will be available. See individual commands
for input and output parameter addresses.

```
  00h      Set RAM[0010h]=0000h
  01h      Unknown Command
  02h      Sort Driver Placements
  03h      2D Coordinate Scale
  04h      Unknown Command
  05h      Simulated Driver Coordinate Calculation
  06h      Multiply
  07h      Raster Data Calculation
  08h      2D Coordinate Rotation
  09h..0Fh Mirrors of 01h..07h
  10h..FFh Mirrors of 00h..0Fh
```

The ST010 BIOS functions are more or less useless and don't increase the
performance or quality of the game (the only feature that is <really>
used is the battery-backed on-chip RAM, aside from that, the powerful chip is a
waste of resources). Note: The ST010 is also used in "Twin Eagle II" (arcade
game, not a SNES game).

**ST011 Commands (japanese chess engine)**

```
  00h      Unused (no function)
  01h      ?
  02h      ?
  03h      ?
  04h      ?
  05h      ?
  06h      ?
  07h      ?
  08h      Unused (no function)
  09h      ?
  0Ah      Unused (no function)
  0Bh      ?
  0Ch      ?
  0Dh      Unused (no function)
  0Eh      ?
  0Fh      ?
  10h..F0h Unused (no function)
  F1h      Selftest1 ?
  F2h      Selftest2 ?
  F3h      Dump Data ROM (bugged, doesn't work due to wrong loop address)
  F4h..FFh Unused (no function)
```

### SNES Cart Seta ST018 (pre-programmed ARM CPU) (1 game)

**Seta ST018 - 160pin SETA D6984 ST018 chip (PCB SHVC-1DE3B-01)**

The chip is used by a single game only:

```
  Hayazashi Nidan Morita Shogi 2 (ST018) (1995) Random House/SETA Corp. (JP)
```

**ARM CPU Reference**

**ST018 Memory Map (ARM Side)**

```
  00000000h ROM 128K  -- with 32bit databus
  20000000h
  40000000h I/O ports
  60000000h probably (absent) external ROM/EPROM ;can redirect exceptions here?
  80000000h
  A0000000h ROM 32K ? -- with 8bit databus
  C0000000h
  E0000000h RAM 16K
```

**ST018 I/O Map (ARM Side)**

```
  40000010h.R  Data from SNES (reset STAT.3 and get latched data-to-arm)
  40000020h.R  Status         (get STAT)
  40000000h.W  Data to SNES   (set STAT.0 and latch data-to-snes)
  40000010h.W  Flag to SNES   (set STAT.2 on writing any value) (IRQ?)
  40000020h.W  Config 1
  40000024h.W  Config 2
  40000028h.W  Config 3
  4000002Ch.W  Config 4
```

**ST018 I/O Map (SNES Side)**

```
  3800h.R      Data to SNES   (reset STAT.0 and get latched data-from-arm)
  3802h.R      Ack Flag       (reset STAT.2 and get dummy data?)
  3804h.R      Status         (get STAT)
  3802h.W      Data from SNES (set STAT.3 and latch data-from-snes)
  3804h.W      Reset ARM      (00h=Normal, 01h=HardReset, FFh=SoftReset?)
```

**ST018 Status Register**

There are two status registers, ARM:40000020h.R and SNES:3804h.R. Bit0 of that
two registers appears to be same for ARM and SNES, the other are used only by
either CPU (as shown below), although they might be actually existing on both
CPUs, too.

```
  0 SNES ARM  ARM-to-SNES Data Present     (0=No, 1=Yes)
  1 -    -    Unknown/Unused               (unknown)
  2 SNES -    ARM-to-SNES IRQ Flag?        (0=No, 1=Yes)
  3 -    ARM  SNES-to-ARM Data Present     (0=No, 1=Yes)
  4 SNES -    Fatal Problem                (0=Okay, 1=SNES skips all transfers)
  5 -    ARM  Redirect ARM to 600000xxh    (0=No, 1=Yes)
  6 SNES -    Unused (unless [FF41h]<>00h) (0=Busy, 1=Ready)
  7 SNES -    ARM Reset Ready              (0=Busy, 1=Ready)
```

STAT.2 might be IRQ signal (ST018.pin12 connects to SNES./IRQ pin), but the
Shogi game contains only bugged IRQ handler (without ACK); instead it's just
polling STAT.2 by software.

**ST018 Component List**

```
  PCB "SHVC-1DE3B-01, (C) 1995 Nintendo"
  U1  32pin LH534BN6 LoROM 512Kx8 (alternately 40pin) (PCB: "4/8/16/32M")
  U2  28pin LH52A64N SRAM 8Kx8                        (PCB: "64K")
  U3 160pin Seta ST018, (C)1994-5 SETA                (PCB: "ST0018/ST0019")
  U4  16pin 74LS139A (demultiplexer)                  (PCB: "LS139")
  U5   8pin /\\ 532 26A (battery controller)          (PCB: "MM1026")
  U6  18pin FA11B CIC                                 (PCB: "CIC")
  BATT 2pin Maxell CR2032T (3V battery for U2)
  X1   3pin [M]21440C 21.44MHz (plastic oscillator)   (PCB: "21.44MHz")
  P1  62pin SNES Cart Edge connector (plus shield)
```

Note: U5 is located on PCB back side for some weird reason. The chip name is
"ST018", although the PCB text layer calls it "ST0018" (with double zero).

**ST018 ARM Timings (mostly unknown)**

The ARM CPU is clocked by a 21.44MHz oscillator, but unknown if there is some
internal clock multiplier/divider for the actual CPU clock (if so, then it
might even be controlled via I/O ports for low power mode purposes).

Unknown if there is any code/data cache, and unknown if there are any memory
waitstates (if so, timings might differ for 8bit/32bit access, for
sequential/nonsequential access, and for different memory regions).

**ST018 ARM Memory (mostly unknown)**

Unknown if there any memory mirrors, or unused regions (possibly filled with
00h, or FFh, or with garbage), or regions that do trap memory exceptions.
Unknown if there any unused extra I/O ports or memory regions.

The 128K ROM, I/O area, and 16K RAM seem to support both 8bit and 32bit access.
The 32K ROM is used only with 8bit access; unknown what happens on 32bit access
to that region.

Effects on misaligned 32bit RAM writes are probably ignoring the lower address
bits, and writing to "ADDR AND (NOT 3)" (at least it like so on ARMv4/ARMv5)
(the case is important because there's a ST018 BUG that does "str r14,[r2,2]",
which should be 8bit STRB, not mis-aligned 32bit STR).

**ST018 ARM Other Stuff (mostly unknown)**

Unknown if there's any coprocessor or SMULL/UMULL extension (the BIOS doesn't
use such stuff, but CP14/CP15 are more or less common to be present).

The CPU seems to use ARMv3 instruction set (since the BIOS is using ARMv3
features: 32bit program counter and CPSR register; but isn't using any ARMv4
features such like BX, LDRH, Sys mode, or THUMB code) (also possible that ARMv4
processors haven't even been available at time when the ST018 was developed in
1994/1995).

**ST018 Commands**

```
  00h..9Fh Unused
  A0       Debug: Reboot
  A1       Debug: Get Version 4 ;\maybe major/minor version (or vice-versa)
  A2       Debug: Get Version 5 ;/
  A3       Debug: Dump 80h bytes from address NNNNNNNNh
  A4       Debug: Dump NNh bytes from address NNNNNNNNh
  A5       Debug: Write NNh bytes to address NNNNNNNNh
  A8        do_high_level_func_0_1_with_reply_flag
  A9        do_high_level_func_1_1_with_reply_flag
  AA       UploadBoardAndSomethingElse (send 9x9 plus 16 bytes to 0E0000400h)
  AB       Write_1_byte_to_0E0000468h (usually value=02h)
  AC       Read ARM "R12" register value
  AD       Read 1 byte from 0E0000464h (LEN)
  AE        do_high_level_func_2_with_reply_flag
  AF       Read 1 byte from 0E0000464h (LEN+1)*2
  B0       Read (LEN+1)*2 bytes from 0E000046Ch  ;LEN as from cmd ADh/AFh
  B1        do_high_level_func_0_X_Y_with_reply_flag (send 2 bytes: X,Y)
  B2        do_high_level_func_1_X_Y_with_reply_flag (send 2 bytes: X,Y)
  B3        do_high_level_func_4_with_1_reply_byte   (recv 1 byte)
  B4        do_high_level_func_5_with_1_reply_byte   (recv 1 byte)
  B5        do_high_level_func_6_with_1_reply_byte   (recv 1 byte)
  B6        do_high_level_func_7_with_1_reply_byte   (recv 1 byte)
  B7        do_high_level_func_3_with_reply_flag
  B8h..F0h Unused
  F1       Selftest 1  ;if response.bit2=1, receive 2 error bytes
  F2       Selftest 2  ;if response<>00h, receive 2 error bytes
  F3       Debug: Dump 128Kbyte ROM from 00000000h ;\for HEX-DUMP display
  F4       Debug: Dump 32Kbyte ROM from A0000000h  ;/
  F5       Debug: Get Chksum for 128K ROM at 00000000h
  F6       Debug: Get Chksum for 32K ROM at A0000000h
  F7h..FFh Unused
```

Note: Command A5h allows to write code to RAM, and also to manipulate return
addresses on stack, thus allowing to execute custom ARM code.

### ARM CPU Reference

The ARM CPU is a 32bit RISC (Reduced Instruction Set Computer) processor,
designed by ARM (Advanced RISC Machines).

**General ARM Information**

**The ARM Instruction Set**

**Further Information**

### ARM Register Set

**Overview**

The following table shows the ARM7TDMI register set which is available in each
mode. There's a total of 37 registers (32bit each), 31 general registers (Rxx)
and 6 status registers (xPSR).

Note that only some registers are 'banked', for example, each mode has it's own
R14 register: called R14, R14_fiq, R14_svc, etc. for each mode respectively.

However, other registers are not banked, for example, each mode is using the
same R0 register, so writing to R0 will always affect the content of R0 in
other modes also.

```
  System/User FIQ       Supervisor Abort     IRQ       Undefined
  --------------------------------------------------------------
  R0          R0        R0         R0        R0        R0
  R1          R1        R1         R1        R1        R1
  R2          R2        R2         R2        R2        R2
  R3          R3        R3         R3        R3        R3
  R4          R4        R4         R4        R4        R4
  R5          R5        R5         R5        R5        R5
  R6          R6        R6         R6        R6        R6
  R7          R7        R7         R7        R7        R7
  --------------------------------------------------------------
  R8          R8_fiq    R8         R8        R8        R8
  R9          R9_fiq    R9         R9        R9        R9
  R10         R10_fiq   R10        R10       R10       R10
  R11         R11_fiq   R11        R11       R11       R11
  R12         R12_fiq   R12        R12       R12       R12
  R13 (SP)    R13_fiq   R13_svc    R13_abt   R13_irq   R13_und
  R14 (LR)    R14_fiq   R14_svc    R14_abt   R14_irq   R14_und
  R15 (PC)    R15       R15        R15       R15       R15
  --------------------------------------------------------------
  CPSR        CPSR      CPSR       CPSR      CPSR      CPSR
  --          SPSR_fiq  SPSR_svc   SPSR_abt  SPSR_irq  SPSR_und
  --------------------------------------------------------------
```

**R0-R12 Registers (General Purpose Registers)**

These thirteen registers may be used for whatever general purposes. Basically,
each is having same functionality and performance, ie. there is no 'fast
accumulator' for arithmetic operations, and no 'special pointer register' for
memory addressing.

**R13 Register (SP)**

This register is used as Stack Pointer (SP) in THUMB state. While in ARM state
the user may decided to use R13 and/or other register(s) as stack pointer(s),
or as general purpose register.

As shown in the table above, there's a separate R13 register in each mode, and
(when used as SP) each exception handler may (and MUST!) use its own stack.

**R14 Register (LR)**

This register is used as Link Register (LR). That is, when calling to a
sub-routine by a Branch with Link (BL) instruction, then the return address
(ie. old value of PC) is saved in this register.

Storing the return address in the LR register is obviously faster than pushing
it into memory, however, as there's only one LR register for each mode, the
user must manually push its content before issuing 'nested' subroutines.

Same happens when an exception is called, PC is saved in LR of new mode.

Note: In ARM mode, R14 may be used as general purpose register also, provided
that above usage as LR register isn't required.

**R15 Register (PC)**

R15 is always used as program counter (PC). Note that when reading R15, this
will usually return a value of PC+nn because of read-ahead (pipelining),
whereas 'nn' depends on the instruction.

**CPSR and SPSR (Program Status Registers) (ARMv3 and up)**

The current condition codes (flags) and CPU control bits are stored in the CPSR
register. When an exception arises, the old CPSR is saved in the SPSR of the
respective exception-mode (much like PC is saved in LR).

For details refer to chapter about CPU Flags.

### ARM Flags & Condition Field (cond)

**ARM Condition Field {cond}**

All ARM instructions can be conditionally executed depending on the state of
the CPSR flags (C,N,Z,V). The respective suffixes {cond} must be appended to
the mnemonics. For example: BEQ = Branch if Equal, MOVMI = Move if Signed.

```
  Code Suffix Flags         Meaning
  0:   EQ     Z=1           equal (zero) (same)
  1:   NE     Z=0           not equal (nonzero) (not same)
  2:   CS/HS  C=1           unsigned higher or same (carry set)
  3:   CC/LO  C=0           unsigned lower (carry cleared)
  4:   MI     N=1           negative (minus)
  5:   PL     N=0           positive or zero (plus)
  6:   VS     V=1           overflow (V set)
  7:   VC     V=0           no overflow (V cleared)
  8:   HI     C=1 and Z=0   unsigned higher
  9:   LS     C=0 or Z=1    unsigned lower or same
  A:   GE     N=V           greater or equal
  B:   LT     N<>V          less than
  C:   GT     Z=0 and N=V   greater than
  D:   LE     Z=1 or N<>V   less or equal
  E:   AL     -             always (the "AL" suffix can be omitted)
  F:   NV     -             never (ARMv1,v2 only) (Reserved on ARMv3 and up)
```

Execution Time: If condition=false: 1S cycle. Otherwise: as specified for the
respective opcode.

**Current Program Status Register (CPSR)**

```
  Bit   Expl.
  31    N - Sign Flag       (0=Not Signed, 1=Signed)               ;\
  30    Z - Zero Flag       (0=Not Zero, 1=Zero)                   ; Condition
  29    C - Carry Flag      (0=Borrow/No Carry, 1=Carry/No Borrow) ; Code Flags
  28    V - Overflow Flag   (0=No Overflow, 1=Overflow)            ;/
  27    Q - Reserved        (used as Sticky Overflow in ARMv5TE and up)
  26-8  - - Reserved        (For future use) - Do not change manually!
  7     I - IRQ disable     (0=Enable, 1=Disable)                  ;\
  6     F - FIQ disable     (0=Enable, 1=Disable)                  ; Control
  5     T - Reserved        (used as THUMB flag in ARMv4T and up)  ; Bits
  4-0   M4-M0 - Mode Bits   (See below)                            ;/
```

**CPSR Bit 27-8,5: Reserved Bits**

These bits are reserved for possible future implementations. For best forwards
compatibility, the user should never change the state of these bits, and should
not expect these bits to be set to a specific value.

**CPSR Bit 7-0: Control Bits (I,F,T,M4-M0)**

These bits may change when an exception occurs. In privileged modes (non-user
modes) they may be also changed manually.

The interrupt bits I and F are used to disable IRQ and FIQ interrupts
respectively (a setting of "1" means disabled).

The Mode Bits M4-M0 contain the current operating mode.

```
  Binary Hex Dec  Expl.
  0xx00b 00h 0  - Old User       ;\26bit Backward Compatibility modes
  0xx01b 01h 1  - Old FIQ        ; (supported only on ARMv3, except ARMv3G,
  0xx10b 02h 2  - Old IRQ        ; and on some non-T variants of ARMv4)
  0xx11b 03h 3  - Old Supervisor ;/
  10000b 10h 16 - User (non-privileged)
  10001b 11h 17 - FIQ
  10010b 12h 18 - IRQ
  10011b 13h 19 - Supervisor (SWI)
  10111b 17h 23 - Abort
  11011b 1Bh 27 - Undefined
  11111b 1Fh 31 - Reserved (used as System mode in ARMv4 and up)
```

Writing any other values into the Mode bits is not allowed.

**Saved Program Status Registers (SPSR_<mode>)**

Additionally to above CPSR, five Saved Program Status Registers exist:

SPSR_fiq, SPSR_svc, SPSR_abt, SPSR_irq, SPSR_und

Whenever the CPU enters an exception, the current status register (CPSR) is
copied to the respective SPSR_<mode> register. Note that there is only
one SPSR for each mode, so nested exceptions inside of the same mode are
allowed only if the exception handler saves the content of SPSR in memory.

For example, for an IRQ exception: IRQ-mode is entered, and CPSR is copied to
SPSR_irq. If the interrupt handler wants to enable nested IRQs, then it must
first push SPSR_irq before doing so.

### ARM 26bit Memory Interface

The 26bit Memory Interface was used by ARMv1 and ARMv2. The 32bit interface is
used by ARMv3 and newer, however, 26bit backward compatibility was included in
all ARMv3 (except ARMv3G), and optionally in some non-T variants of ARMv4.

**Format of R15 in 26bit Mode (Program Counter Register)**

```
  Bit   Name     Expl.
  31-28 N,Z,C,V  Flags (Sign, Zero, Carry, Overflow)
  27-26 I,F      Interrupt Disable bits (IRQ, FIQ) (1=Disable)
  25-2  PC       Program Counter, 24bit, Step 4 (64M range)
  1-0   M1,M0    Mode (0=User, 1=FIQ, 2=IRQ, 3=Supervisor)
```

Branches with +/-32M range wrap the PC register, and can reach all 64M memory.

**Reading from R15**

If R15 is specified in bit16-19 of an opcode, then NZCVIF and M0,1 are masked
(zero), otherwise the full 32bits are used.

**Writing to R15**

ALU opcodes with S=1, and LDM opcodes with PSR=1 can write to all 32bits in R15
(in 26bit mode, that is allowed even in user mode, though it does then affect
only NZCF, not the write protected IFMM bits ???), other opcodes which write to
R15 will modify only the program counter bits. Also, special CMP/CMN/TST/TEQ{P}
opcodes can be used to write to the PSR bits in R15 without modifying the PC
bits.

**Exceptions**

SWIs, Reset, Data/Prefetch Aborts and Undefined instructions enter Supervisor
mode. Interrupts enter IRQ and FIQ mode. Additionally, a special 26bit Address
Exception exists, which enters Supervisor mode on accesses to memory
addresses>=64M as follows:

```
  R14_svc = PC ($+8, including old PSR bits)
  M1,M0 = 11b = supervisor mode, F=same, I=1, PC=14h,
  to continue at the fault location, return by SUBS PC,LR,8.
```

32bit CPUs with 26bit compatibility mode can be configured to switch into 32bit
mode when encountering exceptions.

### ARM Exceptions

**Exception Vectors**

The following are the exception vectors in memory. That is, when an exception
arises, CPU is switched into ARM state, and the program counter (PC) is loaded
by the respective address.

```
  Address  Prio  Exception                  Mode on Entry      Interrupt Flags
  BASE+00h 1     Reset                      Supervisor (_svc)  I=1, F=1
  BASE+04h 7     Undefined Instruction      Undefined  (_und)  I=1, F=unchanged
  BASE+08h 6     Software Interrupt (SWI)   Supervisor (_svc)  I=1, F=unchanged
  BASE+0Ch 5     Prefetch Abort             Abort      (_abt)  I=1, F=unchanged
  BASE+10h 2     Data Abort                 Abort      (_abt)  I=1, F=unchanged
  BASE+14h ??    Address Exceeds 26bit      Supervisor (_svc)  I=1, F=unchanged
  BASE+18h 4     Normal Interrupt (IRQ)     IRQ        (_irq)  I=1, F=unchanged
  BASE+1Ch 3     Fast Interrupt (FIQ)       FIQ        (_fiq)  I=1, F=1
```

BASE is normally 00000000h, but may be optionally FFFF0000h in some ARM CPUs.
Priority for simultaneously occuring exceptions ranges from Prio=1=Highest to
Prio=7=Lowest.

As there's only space for one ARM opcode at each of the above addresses, it'd
be usually recommended to deposit a Branch opcode into each vector, which'd
then redirect to the actual exception handler address.

**Actions performed by CPU when entering an exception**

```
  - R14_<new mode>=PC+nn   ;save old PC, ie. return address
  - SPSR_<new mode>=CPSR   ;save old flags
  - CPSR new T,M bits      ;set to T=0 (ARM state), and M4-0=new mode
  - CPSR new I bit         ;IRQs disabled (I=1), done by ALL exceptions
  - CPSR new F bit         ;FIQs disabled (F=1), done by Reset and FIQ only
  - PC=exception_vector    ;see table above
```

Above "PC+nn" depends on the type of exception (due to pipelining).

**Required user-handler actions when returning from an exception**

Restore any general registers (R0-R14) which might have been modified by the
exception handler. Use return-instruction as listed in the respective
descriptions below, this will both restore PC and CPSR - that automatically
involves that the old CPU state (THUMB or ARM) as well as old state of FIQ and
IRQ disable flags are restored.

As mentioned above (see action on entering...), the return address is always
saved in ARM-style format, so that exception handler may use the same
return-instruction, regardless of whether the exception has been generated from
inside of ARM or THUMB state.

**FIQ (Fast Interrupt Request)**

This interrupt is generated by a LOW level on the nFIQ input. It is supposed to
process timing critical interrupts at a high priority, as fast as possible.

Additionally to the common banked registers (R13_fiq,R14_fiq), five extra
banked registers (R8_fiq-R12_fiq) are available in FIQ mode. The exception
handler may freely access these registers without modifying the main programs
R8-R12 registers (and without having to save that registers on stack).

In privileged (non-user) modes, FIQs may be also manually disabled by setting
the F Bit in CPSR.

**IRQ (Normal Interrupt Request)**

This interrupt is generated by a LOW level on the nIRQ input. Unlike FIQ, the
IRQ mode is not having its own banked R8-R12 registers.

IRQ is having lower priority than FIQ, and IRQs are automatically disabled when
a FIQ exception becomes executed. In privileged (non-user) modes, IRQs may be
also manually disabled by setting the I Bit in CPSR.

To return from IRQ Mode (continuing at following opcode):

```
  SUBS PC,R14,4   ;both PC=R14_irq-4, and CPSR=SPSR_irq
```

**Software Interrupt**

Generated by a software interrupt instruction (SWI). Recommended to request a
supervisor (operating system) function. The SWI instruction may also contain a
parameter in the 'comment field' of the lower 24bit of the 32bit opcode opcode
at [R14_svc-4].

To return from Supervisor Mode (continuing at following opcode):

```
  MOVS PC,R14   ;both PC=R14_svc, and CPSR=SPSR_svc
```

**Undefined Instruction Exception (supported by ARMv3 and up)**

This exception is generated when the CPU comes across an instruction which it
cannot handle. Most likely signalizing that the program has locked up, and that
an errormessage should be displayed.

However, it might be also used to emulate custom functions, ie. as an
additional 'SWI' instruction (which'd use R14_und and SPSR_und though, and it'd
thus allow to execute the Undefined Instruction handler from inside of
Supervisor mode without having to save R14_svc and SPSR_svc).

To return from Undefined Mode (continuing at following opcode):

```
  MOVS PC,R14   ;both PC=R14_und, and CPSR=SPSR_und
```

Note that not all unused opcodes are necessarily producing an exception, for
example, an ARM state Multiply instruction with Bit6=1 would be blindly
accepted as 'legal' opcode.

**Abort (supported by ARMv3 and up)**

Aborts (page faults) are mostly supposed for virtual memory systems (ie. not
used in GBA, as far as I know), otherwise they might be used just to display an
error message. Two types of aborts exists:

- Prefetch Abort (occurs during an instruction prefetch)

- Data Abort (occurs during a data access)

A virtual memory systems abort handler would then most likely determine the
fault address: For prefetch abort that's just "R14_abt-4". For Data abort, the
THUMB or ARM instruction at "R14_abt-8" needs to be 'disassembled' in order to
determine the addressed data in memory.

The handler would then fix the error by loading the respective memory page into
physical memory, and then retry to execute the SAME instruction again, by
returning as follows:

```
  prefetch abort: SUBS PC,R14,#4   ;PC=R14_abt-4, and CPSR=SPSR_abt
  data abort:     SUBS PC,R14,#8   ;PC=R14_abt-8, and CPSR=SPSR_abt
```

Separate exception vectors for prefetch/data abort exists, each should use the
respective return instruction as shown above.

**Address Exceeds 26bit**

This exception can occur only on old ARM CPUs with 26bit address scheme (or in
26bit backwards compatibility mode).

**Reset**

Forces PC=VVVV0000h, and forces control bits of CPSR to T=0 (ARM state), F=1
and I=1 (disable FIQ and IRQ), and M4-0=10011b (Supervisor mode).

### ARM Instruction Summary

Modification of CPSR flags is optional for all {S} instructions.

**Logical ALU Operations**

```
  Instruction                      Cycles    Flags Expl.
  MOV{cond}{S} Rd,Op2              1S+x+y     NZc- Rd = Op2
  MVN{cond}{S} Rd,Op2              1S+x+y     NZc- Rd = NOT Op2
  ORR{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn OR Op2
  EOR{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn XOR Op2
  AND{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn AND Op2
  BIC{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn AND NOT Op2
  TST{cond}{P}    Rn,Op2           1S+x       NZc- Void = Rn AND Op2
  TEQ{cond}{P}    Rn,Op2           1S+x       NZc- Void = Rn XOR Op2
```

Add x=1I cycles if Op2 shifted-by-register. Add y=1S+1N cycles if Rd=R15.

Carry flag affected only if Op2 contains a non-zero shift amount.

**Arithmetic ALU Operations**

```
  Instruction                      Cycles    Flags Expl.
  ADD{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn+Op2
  ADC{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn+Op2+Cy
  SUB{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn-Op2
  SBC{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn-Op2+Cy-1
  RSB{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Op2-Rn
  RSC{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Op2-Rn+Cy-1
  CMP{cond}{P}    Rn,Op2           1S+x       NZCV Void = Rn-Op2
  CMN{cond}{P}    Rn,Op2           1S+x       NZCV Void = Rn+Op2
```

Add x=1I cycles if Op2 shifted-by-register. Add y=1S+1N cycles if Rd=R15.

**Multiply**

```
  Instruction                      Cycles    Flags Expl.
  MUL{cond}{S} Rd,Rm,Rs            1S+mI      NZx- Rd = Rm*Rs
  MLA{cond}{S} Rd,Rm,Rs,Rn         1S+mI+1I   NZx- Rd = Rm*Rs+Rn
  UMULL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+1I   NZx- RdHiLo = Rm*Rs
  UMLAL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+2I   NZx- RdHiLo = Rm*Rs+RdHiLo
  SMULL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+1I   NZx- RdHiLo = Rm*Rs
  SMLAL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+2I   NZx- RdHiLo = Rm*Rs+RdHiLo
```

**Memory Load/Store**

```
  Instruction                      Cycles    Flags Expl.
  LDR{cond}{B}{T} Rd,<Address>     1S+1N+1I+y ---- Rd=[Rn+/-<offset>]
  LDM{cond}{amod} Rn{!},<Rlist>{^} nS+1N+1I+y ---- Load Multiple
  STR{cond}{B}{T} Rd,<Address>     2N         ---- [Rn+/-<offset>]=Rd
  STM{cond}{amod} Rn{!},<Rlist>{^} (n-1)S+2N  ---- Store Multiple
  SWP{cond}{B}    Rd,Rm,[Rn]       1S+2N+1I   ---- Rd=[Rn], [Rn]=Rm
```

For LDR/LDM, add y=1S+1N if Rd=R15, or if R15 in Rlist.

**Jumps, Calls, CPSR Mode, and others**

```
  Instruction                      Cycles    Flags Expl.
  B{cond}   label                  2S+1N      ---- PC=$+8+/-32M
  BL{cond}  label                  2S+1N      ---- PC=$+8+/-32M, LR=$+4
  MRS{cond} Rd,Psr                 1S         ---- Rd=Psr
  MSR{cond} Psr{_field},Op         1S        (psr) Psr[field]=Op
  SWI{cond} Imm24bit               2S+1N      ---- PC=8, ARM Svc mode, LR=$+4
  The Undefined Instruction        2S+1I+1N   ---- PC=4, ARM Und mode, LR=$+4
  condition=false                  1S         ---- Opcodes with {cond}=false
  NOP                              1S         ---- R0=R0
```

**Coprocessor Functions (if any)**

```
  Instruction                         Cycles  Flags Expl.
  CDP{cond} Pn,<cpopc>,Cd,Cn,Cm{,<cp>} 1S+bI   ----  Coprocessor specific
  STC{cond}{L} Pn,Cd,<Address>         (n-1)S+2N+bI  [address] = CRd
  LDC{cond}{L} Pn,Cd,<Address>         (n-1)S+2N+bI  CRd = [address]
  MCR{cond} Pn,<cpopc>,Rd,Cn,Cm{,<cp>} 1S+bI+1C      CRn = Rn {<op> CRm}
  MRC{cond} Pn,<cpopc>,Rd,Cn,Cm{,<cp>} 1S+(b+1)I+1C  Rn = CRn {<op> CRm}
```

**ARM Binary Opcode Format**

```
  |..3 ..................2 ..................1 ..................0|
  |1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0|
  |_Cond__|0_0_0|___Op__|S|__Rn___|__Rd___|__Shift__|Typ|0|__Rm___| DataProc
  |_Cond__|0_0_0|___Op__|S|__Rn___|__Rd___|__Rs___|0|Typ|1|__Rm___| DataProc
  |_Cond__|0_0_1|___Op__|S|__Rn___|__Rd___|_Shift_|___Immediate___| DataProc
  |_Cond__|0_0_1_1_0|P|1|0|_Field_|__Rd___|_Shift_|___Immediate___| PSR Imm
  |_Cond__|0_0_0_1_0|P|L|0|_Field_|__Rd___|0_0_0_0|0_0_0_0|__Rm___| PSR Reg
  |_Cond__|0_0_0_0_0_0|A|S|__Rd___|__Rn___|__Rs___|1_0_0_1|__Rm___| Multiply
  |_Cond__|0_0_0_0_1|U|A|S|_RdHi__|_RdLo__|__Rs___|1_0_0_1|__Rm___| MulLong
  |_Cond__|0_0_0_1_0|B|0_0|__Rn___|__Rd___|0_0_0_0|1_0_0_1|__Rm___| TransSwap
  |_Cond__|0_1_0|P|U|B|W|L|__Rn___|__Rd___|_________Offset________| TransImm
  |_Cond__|0_1_1|P|U|B|W|L|__Rn___|__Rd___|__Shift__|Typ|0|__Rm___| TransReg
  |_Cond__|0_1_1|________________xxx____________________|1|__xxx__| Undefined
  |_Cond__|1_0_0|P|U|S|W|L|__Rn___|__________Register_List________| TransBlock
  |_Cond__|1_0_1|L|___________________Offset______________________| B,BL
  |_Cond__|1_1_0|P|U|N|W|L|__Rn___|__CRd__|__CP#__|____Offset_____| CoDataTrans
  |_Cond__|1_1_1_0|_CPopc_|__CRn__|__CRd__|__CP#__|_CP__|0|__CRm__| CoDataOp
  |_Cond__|1_1_1_0|CPopc|L|__CRn__|__Rd___|__CP#__|_CP__|1|__CRm__| CoRegTrans
  |_Cond__|1_1_1_1|_____________Ignored_by_Processor______________| SWI
```

### ARM Opcodes: Branch and Branch with Link (B, BL, SWI)

**Branch and Branch with Link (B, BL)**

Branch (B) is supposed to jump to a subroutine. Branch with Link is meant to be
used to call to a subroutine, return address is then saved in R14/LR (and can
be restored via MOV PC,LR aka MOV R15,R14) (for nested subroutines, use PUSH LR
and POP PC).

```
  Bit    Expl.
  31-28  Condition
  27-25  Must be "101" for this instruction
  24     Opcode (0-1)
          0: B{cond} label    ;branch      (jump)    PC=PC+8+nn*4
          1: BL{cond} label   ;branch/link (call)    PC=PC+8+nn*4, LR=PC+4
  23-0   nn - Signed Offset, step 4      (-32M..+32M in steps of 4)
```

Execution Time: 2S + 1N

Return: No flags affected.

**Branch via ALU, LDR, LDM**

Most ALU, LDR, LDM opcodes can also change PC/R15.

**Mis-aligned PC/R15 (MOV/ALU/LDR with Rd=R15)**

For ARM code, the low bits of the target address should be usually zero,
otherwise, R15 is forcibly aligned by clearing the lower two bits.

In short, R15 will be always forcibly aligned, so mis-aligned branches won't
have effect on subsequent opcodes that use R15, or [R15+disp] as operand.

**Software Interrupt (SWI) (svc exception)**

SWI supposed for calls to the operating system - Enter Supervisor mode (SVC).

```
  Bit    Expl.
  31-28  Condition
  27-24  Opcode
          1111b: SWI{cond} nn   ;software interrupt
  23-0   nn - Comment Field, ignored by processor (24bit value)
```

Execution Time: 2S+1N

The exception handler may interprete the Comment Field by examining the lower
24bit of the 32bit opcode opcode at [R14_svc-4].

For Returning from SWI use "MOVS PC,R14", that instruction does restore both PC
and CPSR, ie. PC=R14_svc, and CPSR=SPSR_svc.

Nesting SWIs: SPSR_svc and R14_svc should be saved on stack before either
invoking nested SWIs, or (if the IRQ handler uses SWIs) before enabling IRQs.

**Undefined Instruction (und exception)**

```
  Bit    Expl.
  31-28  Condition
  27-25  Must be 011b for this instruction
  24-5   Reserved for future use
  4      Must be 1b for this instruction
  3-0    Reserved for future use
```

No assembler mnemonic exists, following bitstreams are (not) reserved.

```
  cond011xxxxxxxxxxxxxxxxxxxx1xxxx - reserved for future use (except below).
  cond01111111xxxxxxxxxxxx1111xxxx - free for user.
```

Execution time: 2S+1I+1N.

### ARM Opcodes: Data Processing (ALU)

**Data Processing (ALU)**

```
  Bit    Expl.
  31-28  Condition
  27-26  Must be 00b for this instruction
  25     I - Immediate 2nd Operand Flag (0=Register, 1=Immediate)
  24-21  Opcode (0-Fh)               ;*=Arithmetic, otherwise Logical
           0: AND{cond}{S} Rd,Rn,Op2    ;AND logical       Rd = Rn AND Op2
           1: EOR{cond}{S} Rd,Rn,Op2    ;XOR logical       Rd = Rn XOR Op2
           2: SUB{cond}{S} Rd,Rn,Op2 ;* ;subtract          Rd = Rn-Op2
           3: RSB{cond}{S} Rd,Rn,Op2 ;* ;subtract reversed Rd = Op2-Rn
           4: ADD{cond}{S} Rd,Rn,Op2 ;* ;add               Rd = Rn+Op2
           5: ADC{cond}{S} Rd,Rn,Op2 ;* ;add with carry    Rd = Rn+Op2+Cy
           6: SBC{cond}{S} Rd,Rn,Op2 ;* ;sub with carry    Rd = Rn-Op2+Cy-1
           7: RSC{cond}{S} Rd,Rn,Op2 ;* ;sub cy. reversed  Rd = Op2-Rn+Cy-1
           8: TST{cond}{P}    Rn,Op2    ;test            Void = Rn AND Op2
           9: TEQ{cond}{P}    Rn,Op2    ;test exclusive  Void = Rn XOR Op2
           A: CMP{cond}{P}    Rn,Op2 ;* ;compare         Void = Rn-Op2
           B: CMN{cond}{P}    Rn,Op2 ;* ;compare neg.    Void = Rn+Op2
           C: ORR{cond}{S} Rd,Rn,Op2    ;OR logical        Rd = Rn OR Op2
           D: MOV{cond}{S} Rd,Op2       ;move              Rd = Op2
           E: BIC{cond}{S} Rd,Rn,Op2    ;bit clear         Rd = Rn AND NOT Op2
           F: MVN{cond}{S} Rd,Op2       ;not               Rd = NOT Op2
  20     S - Set Condition Codes (0=No, 1=Yes) (Must be 1 for opcode 8-B)
  19-16  Rn - 1st Operand Register (R0..R15) (including PC=R15)
              Must be 0000b for MOV/MVN.
  15-12  Rd - Destination Register (R0..R15) (including PC=R15)
              Must be 0000b (or 1111b) for CMP/CMN/TST/TEQ{P}.
  When above Bit 25 I=0 (Register as 2nd Operand)
    When below Bit 4 R=0 - Shift by Immediate
      11-7   Is - Shift amount   (1-31, 0=Special/See below)
    When below Bit 4 R=1 - Shift by Register
      11-8   Rs - Shift register (R0-R14) - only lower 8bit 0-255 used
      7      Reserved, must be zero  (otherwise multiply or undefined opcode)
    6-5    Shift Type (0=LSL, 1=LSR, 2=ASR, 3=ROR)
    4      R - Shift by Register Flag (0=Immediate, 1=Register)
    3-0    Rm - 2nd Operand Register (R0..R15) (including PC=R15)
  When above Bit 25 I=1 (Immediate as 2nd Operand)
    11-8   Is - ROR-Shift applied to nn (0-30, in steps of 2)
    7-0    nn - 2nd Operand Unsigned 8bit Immediate
```

**Second Operand (Op2)**

This may be a shifted register, or a shifted immediate. See Bit 25 and 11-0.

Unshifted Register: Specify Op2 as "Rm", assembler converts to "Rm,LSL#0".

Shifted Register: Specify as "Rm,SSS#Is" or "Rm,SSS Rs" (SSS=LSL/LSR/ASR/ROR).

Immediate: Specify as 32bit value, for example: "#000NN000h", assembler should
automatically convert into "#0NNh,ROR#0ssh" as far as possible (ie. as far as a
section of not more than 8bits of the immediate is non-zero).

**Zero Shift Amount (Shift Register by Immediate, with Immediate=0)**

```
  LSL#0: No shift performed, ie. directly Op2=Rm, the C flag is NOT affected.
  LSR#0: Interpreted as LSR#32, ie. Op2 becomes zero, C becomes Bit 31 of Rm.
  ASR#0: Interpreted as ASR#32, ie. Op2 and C are filled by Bit 31 of Rm.
  ROR#0: Interpreted as RRX#1 (RCR), like ROR#1, but Op2 Bit 31 set to old C.
```

In source code, LSR#32, ASR#32, and RRX#1 should be specified as such -
attempts to specify LSR#0, ASR#0, or ROR#0 will be internally converted to
LSL#0 by the assembler.

**Using R15 (PC)**

When using R15 as Destination (Rd), note below CPSR description and Execution
time description.

When using R15 as operand (Rm or Rn), the returned value depends on the
instruction: PC+12 if I=0,R=1 (shift by register), otherwise PC+8 (shift by
immediate).

**Returned CPSR Flags**

If S=1, Rd<>R15, logical operations (AND,EOR,TST,TEQ,ORR,MOV,BIC,MVN):

```
  V=not affected
  C=carryflag of shift operation (not affected if LSL#0 or Rs=00h)
  Z=zeroflag of result
  N=signflag of result (result bit 31)
```

If S=1, Rd<>R15, arithmetic operations (SUB,RSB,ADD,ADC,SBC,RSC,CMP,CMN):

```
  V=overflowflag of result
  C=carryflag of result
  Z=zeroflag of result
  N=signflag of result (result bit 31)
```

IF S=1, with unused Rd bits=1111b, {P} opcodes (CMPP/CMNP/TSTP/TEQP):

```
  R15=result  ;modify PSR bits in R15, ARMv2 and below only.
  In user mode only N,Z,C,V bits of R15 can be changed.
  In other modes additionally I,F,M1,M0 can be changed.
  The PC bits in R15 are left unchanged in all modes.
```

If S=1, Rd=R15; should not be used in user mode:

```
  CPSR = SPSR_<current mode>
  PC = result
  For example: MOVS PC,R14  ;return from SWI (PC=R14_svc, CPSR=SPSR_svc).
```

If S=0: Flags are not affected (not allowed for CMP,CMN,TEQ,TST).

The instruction "MOV R0,R0" is used as "NOP" opcode in 32bit ARM state.

Execution Time: (1+p)S+rI+pN. Whereas r=1 if I=0 and R=1 (ie. shift by
register); otherwise r=0. And p=1 if Rd=R15; otherwise p=0.

### ARM Opcodes: PSR Transfer (MRS, MSR)

**Opcode Format**

These instructions occupy an unused area (TEQ,TST,CMP,CMN with S=0) of ALU
opcodes.

```
  Bit    Expl.
  31-28  Condition
  27-26  Must be 00b for this instruction
  25     I - Immediate Operand Flag  (0=Register, 1=Immediate) (Zero for MRS)
  24-23  Must be 10b for this instruction
  22     Psr - Source/Destination PSR  (0=CPSR, 1=SPSR_<current mode>)
  21     Opcode
           0: MRS{cond} Rd,Psr          ;Rd = Psr
           1: MSR{cond} Psr{_field},Op  ;Psr[field] = Op
  20     Must be 0b for this instruction (otherwise TST,TEQ,CMP,CMN)
  For MRS:
    19-16   Must be 1111b for this instruction (otherwise SWP)
    15-12   Rd - Destination Register  (R0-R14)
    11-0    Not used, must be zero.
  For MSR:
    19      f  write to flags field     Bit 31-24 (aka _flg)
    18      s  write to status field    Bit 23-16 (reserved, don't change)
    17      x  write to extension field Bit 15-8  (reserved, don't change)
    16      c  write to control field   Bit 7-0   (aka _ctl)
    15-12   Not used, must be 1111b.
  For MSR Psr,Rm (I=0)
    11-4    Not used, must be zero.
    3-0     Rm - Source Register <op>  (R0-R14)
  For MSR Psr,Imm (I=1)
    11-8    Shift applied to Imm   (ROR in steps of two 0-30)
    7-0     Imm - Unsigned 8bit Immediate
    In source code, a 32bit immediate should be specified as operand.
    The assembler should then convert that into a shifted 8bit value.
```

MSR/MRS and CPSR/SPSR supported by ARMv3 and up.

ARMv2 and below contained PSR flags in R15, accessed by CMP/CMN/TST/TEQ{P}.

The field mask bits specify which bits of the destination Psr are write-able
(or write-protected), one or more of these bits should be set, for example,
CPSR_fsxc (aka CPSR aka CPSR_all) unlocks all bits (see below user mode
restriction though).

Restrictions:

In non-privileged mode (user mode): only condition code bits of CPSR can be
changed, control bits can't.

Only the SPSR of the current mode can be accessed; In User and System modes no
SPSR exists.

Unused Bits in CPSR are reserved for future use and should never be changed
(except for unused bits in the flags field).

Execution Time: 1S.

Note: The A22i assembler recognizes MOV as alias for both MSR and MRS because
it is practically not possible to remember whether MSR or MRS was the load or
store opcode, and/or whether it does load to or from the Psr register.

### ARM Opcodes: Multiply and Multiply-Accumulate (MUL, MLA)

**Opcode Format**

```
  Bit    Expl.
  31-28  Condition
  27-25  Must be 000b for this instruction
  24-21  Opcode
          0000b: MUL{cond}{S}   Rd,Rm,Rs        ;multiply   Rd = Rm*Rs
          0001b: MLA{cond}{S}   Rd,Rm,Rs,Rn     ;mul.& accumulate Rd = Rm*Rs+Rn
          0100b: UMULL{cond}{S} RdLo,RdHi,Rm,Rs ;multiply   RdHiLo=Rm*Rs
          0101b: UMLAL{cond}{S} RdLo,RdHi,Rm,Rs ;mul.& acc. RdHiLo=Rm*Rs+RdHiLo
          0110b: SMULL{cond}{S} RdLo,RdHi,Rm,Rs ;sign.mul.  RdHiLo=Rm*Rs
          0111b: SMLAL{cond}{S} RdLo,RdHi,Rm,Rs ;sign.m&a.  RdHiLo=Rm*Rs+RdHiLo
  20     S - Set Condition Codes (0=No, 1=Yes) (Must be 0 for Halfword mul)
  19-16  Rd (or RdHi) - Destination Register (R0-R14)
  15-12  Rn (or RdLo) - Accumulate Register  (R0-R14) (Set to 0000b if unused)
  11-8   Rs - Operand Register               (R0-R14)
  7-4    Must be 1001b for these instructions
  3-0    Rm - Operand Register               (R0-R14)
```

**Multiply and Multiply-Accumulate (MUL, MLA)**

Restrictions: Rd may not be same as Rm. Rd,Rn,Rs,Rm may not be R15.

Note: Only the lower 32bit of the internal 64bit result are stored in Rd, thus
no sign/zero extension is required and MUL and MLA can be used for both signed
and unsigned calculations!

Execution Time: 1S+mI for MUL, and 1S+(m+1)I for MLA. Whereas 'm' depends on
whether/how many most significant bits of Rs are all zero or all one. That is
m=1 for Bit 31-8, m=2 for Bit 31-16, m=3 for Bit 31-24, and m=4 otherwise.

Flags (if S=1): Z=zeroflag, N=signflag, C=destroyed (ARMv4 and below) or C=not
affected (ARMv5 and up), V=not affected. MUL/MLA supported by ARMv2 and up.

**Multiply Long and Multiply-Accumulate Long (MULL, MLAL)**

Optionally supported, INCLUDED in ARMv3M, EXCLUDED in ARMv4xM/ARMv5xM.

Restrictions: RdHi,RdLo,Rm must be different registers. R15 may not be used.

Execution Time: 1S+(m+1)I for MULL, and 1S+(m+2)I for MLAL. Whereas 'm' depends
on whether/how many most significant bits of Rs are "all zero" (UMULL/UMLAL) or
"all zero or all one" (SMULL,SMLAL). That is m=1 for Bit 31-8, m=2 for Bit
31-16, m=3 for Bit 31-24, and m=4 otherwise.

Flags (if S=1): Z=zeroflag, N=signflag, C=destroyed (ARMv4 and below) or C=not
affected (ARMv5 and up), V=destroyed??? (ARMv4 and below???) or V=not affected
(ARMv5 and up).

### ARM Opcodes: Memory: Block Data Transfer (LDM, STM)

**Opcode Format**

```
  Bit    Expl.
  31-28  Condition
  27-25  Must be 100b for this instruction
  24     P - Pre/Post (0=post; add offset after transfer, 1=pre; before trans.)
  23     U - Up/Down Bit (0=down; subtract offset from base, 1=up; add to base)
  22     S - PSR & force user bit (0=No, 1=load PSR or force user mode)
  21     W - Write-back bit (0=no write-back, 1=write address into base)
  20     L - Load/Store bit (0=Store to memory, 1=Load from memory)
          0: STM{cond}{amod} Rn{!},<Rlist>{^}  ;Store (Push)
          1: LDM{cond}{amod} Rn{!},<Rlist>{^}  ;Load  (Pop)
          Whereas, {!}=Write-Back (W), and {^}=PSR/User Mode (S)
  19-16  Rn - Base register                (R0-R14) (not including R15)
  15-0   Rlist - Register List
  (Above 'offset' is meant to be the number of words specified in Rlist.)
```

Return: No Flags affected.

Execution Time: For normal LDM, nS+1N+1I. For LDM PC, (n+1)S+2N+1I. For STM
(n-1)S+2N. Where n is the number of words transferred.

**Addressing Modes {amod}**

The IB,IA,DB,DA suffixes directly specify the desired U and P bits:

```
  IB  increment before          ;P=1, U=1
  IA  increment after           ;P=0, U=1
  DB  decrement before          ;P=1, U=0
  DA  decrement after           ;P=0, U=0
```

Alternately, FD,ED,FA,EA could be used, mostly to simplify mnemonics for stack
transfers.

```
  ED  empty stack, descending   ;LDM: P=1, U=1  ;STM: P=0, U=0
  FD  full stack,  descending   ;     P=0, U=1  ;     P=1, U=0
  EA  empty stack, ascending    ;     P=1, U=0  ;     P=0, U=1
  FA  full stack,  ascending    ;     P=0, U=0  ;     P=1, U=1
```

Stack operations are conventionally using Rn=R13/SP as stack pointer in Full
Descending mode (meaning that free memory starts at SP-1 and below, and used
memory at SP+0 and up; that model is also used by other CPUs like 80x86 and
Z80). The following expressions are aliases for each other:

```
  STMFD=STMDB=PUSH   STMED=STMDA   STMFA=STMIB   STMEA=STMIA
  LDMFD=LDMIA=POP    LDMED=LDMIB   LDMFA=LDMDA   LDMEA=LDMDB
```

**When S Bit is set (S=1)**

If instruction is LDM and R15 is in the list: (Mode Changes)

```
  While R15 loaded, additionally: CPSR=SPSR_<current mode>
```

Otherwise: (User bank transfer)

```
  Rlist is referring to User Bank Registers R0-R15 (rather than to registers
  of the current mode; such like R14_svc etc.)
  Base write-back should not be used for User bank transfer.
  Caution - When instruction is LDM:
  If the following instruction reads from a banked register (eg. R14_svc),
  then CPU might still read R14 instead; if necessary insert a dummy NOP.
```

**Transfer Order**

The lowest Register in Rlist (R0 if its in the list) will be loaded/stored
to/from the lowest memory address.

Internally, the rlist registers are always processed with sequentially
INCREASING addresses (ie. for DECREASING addressing modes, the CPU does first
calculate the lowest address, and does then process rlist with increasing
addresses; this detail can be important when accessing memory mapped I/O
ports).

**Mis-aligned STM,LDM,PUSH,POP (forced align)**

The base address should be usually word-aligned. Otherwise, mis-aligned low
bit(s) are ignored, the memory access goes to a forcibly aligned (rounded-down)
memory address "addr AND (NOT 3)".

**Strange Effects on Invalid Rlist's**

Empty Rlist: R15 loaded/stored (ARMv4 only), and Rb=Rb+/-40h (ARMv4-v5).

Writeback with Rb included in Rlist: Store OLD base if Rb is FIRST entry in
Rlist, otherwise store NEW base (STM/ARMv4), always store OLD base (STM/ARMv5),
no writeback (LDM/ARMv4), writeback if Rb is "the ONLY register, or NOT the
LAST register" in Rlist (LDM/ARMv5).

### ARM Opcodes: Memory: Single Data Transfer (LDR, STR)

**Opcode Format**

```
  Bit    Expl.
  31-28  Condition
  27-26  Must be 01b for this instruction
  25     I - Immediate Offset Flag (0=Immediate, 1=Shifted Register)
  24     P - Pre/Post (0=post; add offset after transfer, 1=pre; before trans.)
  23     U - Up/Down Bit (0=down; subtract offset from base, 1=up; add to base)
  22     B - Byte/Word bit (0=transfer 32bit/word, 1=transfer 8bit/byte)
  When above Bit 24 P=0 (Post-indexing, write-back is ALWAYS enabled):
    21     T - Memory Management (0=Normal, 1=Force non-privileged access)
  When above Bit 24 P=1 (Pre-indexing, write-back is optional):
    21     W - Write-back bit (0=no write-back, 1=write address into base)
  20     L - Load/Store bit (0=Store to memory, 1=Load from memory)
          0: STR{cond}{B}{T} Rd,<Address>   ;[Rn+/-<offset>]=Rd
          1: LDR{cond}{B}{T} Rd,<Address>   ;Rd=[Rn+/-<offset>]
          Whereas, B=Byte, T=Force User Mode (only for POST-Indexing)
  19-16  Rn - Base register               (R0..R15) (including R15=PC+8)
  15-12  Rd - Source/Destination Register (R0..R15) (including R15=PC+12)
  When above I=0 (Immediate as Offset)
    11-0   Unsigned 12bit Immediate Offset (0-4095, steps of 1)
  When above I=1 (Register shifted by Immediate as Offset)
    11-7   Is - Shift amount      (1-31, 0=Special/See below)
    6-5    Shift Type             (0=LSL, 1=LSR, 2=ASR, 3=ROR)
    4      Must be 0 (Reserved, see The Undefined Instruction)
    3-0    Rm - Offset Register   (R0..R14) (not including PC=R15)
```

**Instruction Formats for <Address>**

An expression which generates an address:

```
  <expression>                  ;an immediate used as address
  ;*** restriction: must be located in range PC+/-4095+8, if so,
  ;*** assembler will calculate offset and use PC (R15) as base.
```

Pre-indexed addressing specification:

```
  [Rn]                          ;offset = zero
  [Rn, <#{+/-}expression>]{!}   ;offset = immediate
  [Rn, {+/-}Rm{,<shift>} ]{!}   ;offset = register shifted by immediate
```

Post-indexed addressing specification:

```
  [Rn], <#{+/-}expression>      ;offset = immediate
  [Rn], {+/-}Rm{,<shift>}       ;offset = register shifted by immediate
```

Whereas...

```
  <shift>  immediate shift such like LSL#4, ROR#2, etc. (see ALU opcodes).
  {!}      exclamation mark ("!") indicates write-back (Rn will be updated).
```

**Notes**

Shift amount 0 has special meaning, as described for ALU opcodes.

When writing a word (32bit) to memory, the address should be word-aligned.

When reading a byte from memory, upper 24 bits of Rd are zero-extended.

When reading a word from a halfword-aligned address (which is located in the
middle between two word-aligned addresses), the lower 16bit of Rd will contain
[address] ie. the addressed halfword, and the upper 16bit of Rd will contain
[Rd-2] ie. more or less unwanted garbage. However, by isolating lower bits this
may be used to read a halfword from memory. (Above applies to little endian
mode, as used in GBA.)

In a virtual memory based environment (ie. not in the GBA), aborts (ie. page
faults) may take place during execution, if so, Rm and Rn should not specify
the same register when post-indexing is used, as the abort-handler might have
problems to reconstruct the original value of the register.

Return: CPSR flags are not affected.

Execution Time: For normal LDR: 1S+1N+1I. For LDR PC: 2S+2N+1I. For STR: 2N.

**Mis-aligned 32bit STR (forced align)**

The mis-aligned low bit(s) are ignored, the memory access goes to a forcibly
aligned (rounded-down) memory address "addr AND (NOT 3)".

**Mis-aligned 32bit LDR (rotated read)**

Reads from forcibly aligned address "addr AND (NOT 3)", and does then rotate
the data as "ROR (addr AND 3)*8".

### ARM Opcodes: Memory: Single Data Swap (SWP)

**Opcode Format**

```
  Bit    Expl.
  31-28  Condition
  27-23  Must be 00010b for this instruction
         Opcode (fixed)
           SWP{cond}{B} Rd,Rm,[Rn]      ;Rd=[Rn], [Rn]=Rm
  22     B - Byte/Word bit (0=swap 32bit/word, 1=swap 8bit/byte)
  21-20  Must be 00b for this instruction
  19-16  Rn - Base register                     (R0-R14)
  15-12  Rd - Destination Register              (R0-R14)
  11-4   Must be 00001001b for this instruction
  3-0    Rm - Source Register                   (R0-R14)
```

SWP/SWPB supported by ARMv2a and up.

Swap works properly including if Rm and Rn specify the same register.

R15 may not be used for either Rn,Rd,Rm. (Rn=R15 would be MRS opcode).

Upper bits of Rd are zero-expanded when using Byte quantity. For info about
byte and word data memory addressing, read LDR and STR opcode description.

Execution Time: 1S+2N+1I. That is, 2N data cycles, 1S code cycle, plus 1I.

**Mis-aligned 32bit SWP (rotated read)**

The SWP opcode works like a combination of LDR and STR, that means, it does
read-rotated, but does write-unrotated.

### ARM Opcodes: Coprocessor Instructions (MRC/MCR, LDC/STC, CDP)

**Coprocessor Register Transfers (MRC, MCR) (with ARM Register read/write)**

```
  Bit    Expl.
  31-28  Condition
  27-24  Must be 1110b for this instruction
  23-21  CP Opc - Coprocessor operation code         (0-7)
  20     ARM-Opcode (0-1)
          0: MCR{cond} Pn,<cpopc>,Rd,Cn,Cm{,<cp>}   ;move from ARM to CoPro
          1: MRC{cond} Pn,<cpopc>,Rd,Cn,Cm{,<cp>}   ;move from CoPro to ARM
  19-16  Cn     - Coprocessor source/dest. Register  (C0-C15)
  15-12  Rd     - ARM source/destination Register    (R0-R15)
  11-8   Pn     - Coprocessor number                 (P0-P15)
  7-5    CP     - Coprocessor information            (0-7)
  4      Reserved, must be one (1) (otherwise CDP opcode)
  3-0    Cm     - Coprocessor operand Register       (C0-C15)
```

MCR/MRC supported by ARMv2 and up.

A22i syntax allows to use MOV with Rd specified as first (dest), or last
(source) operand. Native MCR/MRC syntax uses Rd as middle operand, <cp>
can be ommited if <cp> is zero.

When using MCR with R15: Coprocessor will receive a data value of PC+12.

When using MRC with R15: Bit 31-28 of data are copied to Bit 31-28 of CPSR (ie.
N,Z,C,V flags), other data bits are ignored, CPSR Bit 27-0 are not affected,
R15 (PC) is not affected.

Execution time: 1S+bI+1C for MCR, 1S+(b+1)I+1C for MRC.

Return: For MRC only: Either R0-R14 modified, or flags affected (see above).

For details refer to original ARM docs. The opcodes irrelevant for GBA/NDS7
because no coprocessor exists (except for a dummy CP14 unit). However, NDS9
includes a working CP15 unit.

**Coprocessor Data Transfers (LDC, STC) (with Memory read/write)**

```
  Bit    Expl.
  31-28  Condition
  27-25  Must be 110b for this instruction
  24     P - Pre/Post (0=post; add offset after transfer, 1=pre; before trans.)
  23     U - Up/Down Bit (0=down; subtract offset from base, 1=up; add to base)
  22     N - Transfer length (0-1, interpretation depends on co-processor)
  21     W - Write-back bit (0=no write-back, 1=write address into base)
  20     Opcode (0-1)
          0: STC{cond}{L} Pn,Cd,<Address>  ;Store to memory (from coprocessor)
          1: LDC{cond}{L} Pn,Cd,<Address>  ;Read from memory (to coprocessor)
          whereas {L} indicates long transfer (Bit 22: N=1)
  19-16  Rn     - ARM Base Register              (R0-R15)     (R15=PC+8)
  15-12  Cd     - Coprocessor src/dest Register  (C0-C15)
  11-8   Pn     - Coprocessor number             (P0-P15)
  7-0    Offset - Unsigned Immediate, step 4     (0-1020, in steps of 4)
```

LDC/STC supported by ARMv2 and up.

Execution time: (n-1)S+2N+bI, n=number of words transferred.

For details refer to original ARM docs, irrelevant in GBA because no
coprocessor exists.

**Coprocessor Data Operations (CDP) (without Memory or ARM Register operand)**

```
  Bit    Expl.
  31-28  Condition
  27-24  Must be 1110b for this instruction
         ARM-Opcode (fixed)
           CDP{cond} Pn,<cpopc>,Cd,Cn,Cm{,<cp>}
  23-20  CP Opc - Coprocessor operation code       (0-15)
  19-16  Cn     - Coprocessor operand Register     (C0-C15)
  15-12  Cd     - Coprocessor destination Register (C0-C15)
  11-8   Pn     - Coprocessor number               (P0-P15)
  7-5    CP     - Coprocessor information          (0-7)
  4      Reserved, must be zero (otherwise MCR/MRC opcode)
  3-0    Cm     - Coprocessor operand Register     (C0-C15)
```

CDP supported by ARMv2 and up.

Execution time: 1S+bI, b=number of cycles in coprocessor busy-wait loop.

Return: No flags affected, no ARM-registers used/modified.

For details refer to original ARM docs, irrelevant in GBA because no
coprocessor exists.

### ARM Pseudo Instructions and Directives

**ARM Pseudo Instructions**

```
  nop              mov r0,r0
  ldr Rd,=Imm      ldr Rd,[r15,disp] ;use .pool as parameter field
  add Rd,=addr     add/sub Rd,r15,disp
  adr Rd,addr      add/sub Rd,r15,disp
  adrl Rd,addr     two add/sub opcodes with disp=xx00h+00yyh
  mov Rd,Imm       mvn Rd,NOT Imm    ;or vice-versa
  and Rd,Rn,Imm    bic Rd,Rn,NOT Imm ;or vice-versa
  cmp Rd,Rn,Imm    cmn Rd,Rn,-Imm    ;or vice-versa
  add Rd,Rn,Imm    sub Rd,Rn,-Imm    ;or vice-versa
```

All above opcodes may be made conditional by specifying a {cond} field.

**A22i Directives**

```
  org  adr     assume following code from this address on
  .gba         indicate GBA program
  .nds         indicate NDS program
  .fix         fix GBA/NDS header checksum
  .norewrite   do not delete existing output file (keep following data in file)
  .data?       following defines RAM data structure (assembled to nowhere)
  .code        following is normal ROM code/data (assembled to ROM image)
  .include     includes specified source code file (no nesting/error handling)
  .import      imports specified binary file (optional parameters: ,begin,len)
  .radix nn    changes default numeric format (nn=2,8,10,16 = bin/oct/dec/hex)
  .errif expr  generates an error message if expression is nonzero
  .if expr     assembles following code only if expression is nonzero
  .else        invert previous .if condition
  .endif       terminate .if/.ifdef/.ifndef
  .ifdef sym   assemble following only if symbol is defined
  .ifndef sym  assemble following only if symbol is not defined
  .align nn    aligns to an address divisible-by-nn, inserts 00's
  l equ n      l=n
  l:   [cmd]   l=$   (global label)
  @@l: [cmd]   @@l=$ (local label, all locals are reset at next global label)
  end          end of source code
  db ...       define 8bit data (bytes)
  dw ...       define 16bit data (halfwords)
  dd ...       define 32bit data (words)
  defs nn      define nn bytes space (zero-filled)
  ;...         defines a comment (ignored by the assembler)
  //           alias for CRLF, eg. allows <db 'Text',0 // dw addr> in one line
```

**A22i Alias Directives (for compatibility with other assemblers)**

```
  align        .align 4          code16    .thumb
  align nn     .align nn         .code 16  .thumb
  % nn         defs nn           code32    .arm
  .space nn    defs nn           .code 32  .arm
  ..ds nn      defs nn           ltorg     .pool
  x=n          x equ n           .ltorg    .pool
  .equ x,n     x equ n           ..ltorg   .pool
  .define x n  x equ n           dcb       db (8bit data)
  incbin       .import           defb      db (8bit data)
  @@@...       ;comment          .byte     db (8bit data)
  @ ...        ;comment          .ascii    db (8bit string)
  @*...        ;comment          dcw       dw (16bit data)
  @...         ;comment          defw      dw (16bit data)
  .text        .code             .hword    dw (16bit data)
  .bss         .data?            dcd       dd (32bit data)
  .global      (ignored)         defd      dd (32bit data)
  .extern      (ignored)         .long     dd (32bit data)
  .thumb_func  (ignored)         .word     dw/dd, don't use
  #directive   .directive        .end      end
  .fill nn,1,0 defs nn
```

**Alias Conditions, Opcodes, Operands**

```
  hs   cs   ;condition higher or same = carry set
  lo   cc   ;condition lower = carry cleared
  asl  lsl  ;arithmetic shift left = logical shift left
```

**A22i Numeric Formats & Dialects**

```
  Type          Normal       Alias
  Decimal       85           #85  &d85
  Hexadecimal   55h          #55h  0x55  #0x55  $55  &h55
  Octal         125o         0o125  &o125
  Ascii         'U'          "U"
  Binary        01010101b    %01010101  0b01010101  &b01010101
  Roman         &rLXXXV      (very useful for arrays of kings and chapters)
```

Note: The default numeric format can be changed by the .radix directive
(usually 10=decimal). For example, with radix 16, values like "85" and "0101b"
are treated as hexadecimal numbers (in that case, decimal and binary numbers
can be still defined with prefixes &d and &b).

**A22i Numeric Operators Priority**

```
  Prio  Operator           Aliases
  8     (,) brackets
  7     +,- sign
  6     *,/,MOD,SHL,SHR    MUL,DIV,<<,>>
  5     +,- operation
  4     EQ,GE,GT,LE,LT,NE  =,>=,>,<=,<,<>,==,!=
  3     NOT
  2     AND
  1     OR,XOR             EOR
```

Operators of same priority are processed from left to right.

Boolean operators (priority 4) return 1=TRUE, 0=FALSE.

**A22i Nocash Syntax**

Even though A22i does recognize the official ARM syntax, it's also allowing to
use friendly code:

```
  mov   r0,0ffh         ;no C64-style "#", and no C-style "0x" required
  stmia [r7]!,r0,r4-r5  ;square [base] brackets, no fancy {rlist} brackets
  mov   r0,cpsr         ;no confusing MSR and MRS (whatever which is which)
  mov   r0,p0,0,c0,c0,0 ;no confusing MCR and MRC (whatever which is which)
  ldr   r0,[score]      ;allows to use clean brackets for relative addresses
  push  rlist           ;alias for stmfd [r13]!,rlist (and same for pop/ldmfd)
  label:                ;label definitions recommended to use ":" colons
```

[A22i is the no$gba debug version's built-in source code assembler.]

### ARM Instruction Cycle Times

Instruction Cycle Summary

```
  Instruction      Cycles      Additional
  ---------------------------------------------------------------------
  ALU              1S          +1S+1N if R15 loaded, +1I if SHIFT(Rs)
  MSR,MRS          1S
  LDR              1S+1N+1I    +1S+1N if R15 loaded
  STR              2N
  LDM              nS+1N+1I    +1S+1N if R15 loaded
  STM              (n-1)S+2N
  SWP              1S+2N+1I
  B,BL             2S+1N
  SWI,trap         2S+1N
  MUL              1S+ml
  MLA              1S+(m+1)I
  MULL             1S+(m+1)I
  MLAL             1S+(m+2)I
  CDP              1S+bI
  LDC,STC          (n-1)S+2N+bI
  MCR              1N+bI+1C
  MRC              1S+(b+1)I+1C
  {cond} false     1S
```

Whereas,

```
  n = number of words transferred
  b = number of cycles spent in coprocessor busy-wait loop
  m = depends on most significant byte(s) of multiplier operand
```

Above 'trap' is meant to be the execution time for exceptions. And '{cond}
false' is meant to be the execution time for conditional instructions which
haven't been actually executed because the condition has been false.

The separate meaning of the N,S,I,C cycles is:

**N - Non-sequential cycle**

Requests a transfer to/from an address which is NOT related to the address used
in the previous cycle. (Called 1st Access in GBA language).

The execution time for 1N is 1 clock cycle (plus non-sequential access
waitstates).

**S - Sequential cycle**

Requests a transfer to/from an address which is located directly after the
address used in the previous cycle. Ie. for 16bit or 32bit accesses at
incrementing addresses, the first access is Non-sequential, the following
accesses are sequential. (Called 2nd Access in GBA language).

The execution time for 1S is 1 clock cycle (plus sequential access waitstates).

**I - Internal Cycle**

CPU is just too busy, not even requesting a memory transfer for now.

The execution time for 1I is 1 clock cycle (without any waitstates).

**C - Coprocessor Cycle**

The CPU uses the data bus to communicate with the coprocessor (if any), but no
memory transfers are requested.

**Memory Waitstates**

Ideally, memory may be accessed free of waitstates (1N and 1S are then equal to
1 clock cycle each). However, a memory system may generate waitstates for
several reasons: The memory may be just too slow. Memory is currently accessed
by DMA, eg. sound, video, memory transfers, etc. Or when data is squeezed
through a 16bit data bus (in that special case, 32bit access may have more
waitstates than 8bit and 16bit accesses). Also, the memory system may separate
between S and N cycles (if so, S cycles would be typically faster than N
cycles).

**Memory Waitstates for Different Memory Areas**

Different memory areas (eg. ROM and RAM) may have different waitstates. When
executing code in one area which accesses data in another area, then the S+N
cycles must be split into code and data accesses: 1N is used for data access,
plus (n-1)S for LDM/STM, the remaining S+N are code access. If an instruction
jumps to a different memory area, then all code cycles for that opcode are
having waitstate characteristics of the NEW memory area.

### ARM Versions

**Version Numbers**

ARM CPUs are distributed by name ARM#, and are described as ARMv# in
specifications, whereas "#" is NOT the same than "v#", for example, ARM7TDMI is
ARMv4TM. That is so confusing, that ARM didn't even attempt to clarify the
relationship between the various "#" and "v#" values.

**Version Variants**

Suffixes like "M" (long multiply), "T" (THUMB support), "E" (Enhanced DSP)
indicate presence of special features, additionally to the standard instruction
set of a given version, or, when preceded by an "x", indicate the absence of
that features.

**ARMv1 aka ARM1**

Some sort of a beta version, according to ARM never been used in any commercial
products.

**ARMv2 and up**

MUL,MLA

CDP,LDC,MCR,MRC,STC

SWP/SWPB (ARMv2a and up only)

Two new FIQ registers

**ARMv3 and up**

MRS,MSR opcodes (instead CMP/CMN/TST/TEQ{P} opcodes)

CPSR,SPSR registers (instead PSR bits in R15)

Removed never condition, cond=NV no longer valid

32bit addressing (instead 26bit addressing in older versions)

26bit addressing backwards comptibility mode (except v3G)

Abt and Und modes (instead handling aborts/undefined in Svc mode)

SMLAL,SMULL,UMLAL,UMULL (optionally, INCLUDED in v3M, EXCLUDED in v4xM/v5xM)

**ARMv4 aka ARM7 and up**

LDRH,LDRSB,LDRSH,STRH

Sys mode (privileged user mode)

BX (only ARMv4T, and any ARMv5 or ARMv5T and up)

THUMB code (only T variants, ie. ARMv4T, ARMv5T)

**ARMv5 aka ARM9 and up**

BKPT,BLX,CLZ (BKPT,BLX also in THUMB mode)

LDM/LDR/POP PC with mode switch (POP PC also in THUMB mode)

CDP2,LDC2,MCR2,MRC2,STC2 (new coprocessor opcodes)

C-flag unchanged by MUL (instead undefined flag value)

changed instruction cycle timings / interlock ??? or not ???

QADD,QDADD,QDSUB,QSUB opcodes, CPSR.Q flag (v5TE and V5TExP only)

SMLAxy,SMLALxy,SMLAWy,SMULxy,SMULWy (v5TE and V5TExP only)

LDRD,STRD,PLD,MCRR,MRRC (v5TE only, not v5, not v5TExP)

**ARMv6**

No public specifications available.

**A Milestone in Computer History**

Original ARMv2 has been used in the relative rare and expensive Archimedes
deluxe home computers in the late eighties, the Archimedes has caught a lot of
attention, particularly for being the first home computer that used a BIOS
being programmed in BASIC language - which has been a absolutely revolutionary
decadency at that time.

Inspired, programmers all over the world have successfully developed even
slower and much more inefficient programming languages, which are nowadays
consequently used by nearly all ARM programmers, and by most non-ARM
programmers as well.

### SNES Cart OBC1 (OBJ Controller) (1 game)

The OBC1 is a 80pin OBJ Controller chip from Nintendo, used by only one game:

```
  Metal Combat: Falcon's Revenge (1993) Intelligent Systems/Nintendo
  (Note: the game also requires a Super Scope lightgun)
```

**OBC1 I/O Ports**

```
  7FF0h OAM Xloc = [Base+Index*4+0]  (R/W)
  7FF1h OAM Yloc = [Base+Index*4+1]  (R/W)
  7FF2h OAM Tile = [Base+Index*4+2]  (R/W)
  7FF3h OAM Attr = [Base+Index*4+3]  (R/W)
  7FF4h OAM Bits = [Base+Index/4+200h].Bit((Index AND 3)*2+0..1) (R?/W)
  7FF5h Base for 220h-byte region (bit0: 0=7C00h, 1=7800h)
  7FF6h Index (OBJ Number) (0..127)
  7FF7h Unknown (set to 00h or 0Ah) (maybe SRAM vs I/O mode select)
```

Other bytes at 6000h..7FFFh contain 8Kbyte battery-backed SRAM (of which,
7800h..7A1Fh and 7C00h..7E1Fh can be used as OBJ workspace).

**Notes**

Port 7FF0h-7FF3h/7FF5h are totally useless. Port 7FF4h/7FF6h are eventually
making it slightly easier to combine the 2bit OAM fragments, though putting a
huge 80pin chip into the cartridge for merging 2bit fragments is definetly
overcomplicated.

As far as known, the Index isn't automatically incremented. Port 7FF4h does
read-modify-write operations which may involve timing restrictions (?), or,
modify-write (when prefetching data on 7FF6h writes) which may come up with
out-dated-prefetch effects.

Reading from 7FF4h does reportedly return the desired BYTE, but WITHOUT
isolating & shifting the desired BITS into place?

Setting Index bits7+5 does reportedly enable SRAM mapping at 6000h..77FFh?

ROM is reportedly mapped to bank 00h..3Fh, and also to bank 70h..71h? Maybe
that info just refers to SRAM not being mapped to that region (as it'd be in
some other LoROM cartridges).

**PCB "SHVC-2E3M-01"**

Contains six chips and a battery. The chips are: Two 1MB ROMs, MAD-1, OBC1,
CIC, 8K SRAM. All chips (except MAD-1) are SMD chips.

### SNES Cart S-DD1 (Data Decompressor) (2 games)

The S-DD1 is a 100pin Data Decompression chip, used by only two games:

```
  Star Ocean (6MB ROM, 8KB RAM) (1996) tri-Ace/Enix (JP)
  Street Fighter Alpha 2 (4MB ROM, no RAM) (1996) Capcom (NA) (JP) (EU)
```

**S-DD1 Decompression Algorithm**

**S-DD1 I/O Ports**

```
  4800h  DMA Enable 1 (bit0..7 = DMA 0..7) (unchanged after DMA)
  4801h  DMA Enable 2 (bit0..7 = DMA 0..7) (automatically cleared after DMA)
  4802h  Unknown   ;\set to 0000h by Star Ocean (maybe SRAM related)
  4803h  Unknown   ;/unused by Street Fighter Alpha 2
  4804h  ROM Bank for C00000h-CFFFFFh (in 1MByte units)
  4805h  ROM Bank for D00000h-DFFFFFh (in 1MByte units)
  4806h  ROM Bank for E00000h-EFFFFFh (in 1MByte units)
  4807h  ROM Bank for F00000h-FFFFFFh (in 1MByte units)
  <DMA>  DMA from ROM returns Decompressed Data (originated at DMA start addr)
```

**S-DD1 Memory Map**

```
  ???-???          SRAM (if any)
  008000h-00FFFFh  Exception Handlers, mapped in LoROM-fashion (ROM 0..7FFFh)
  C00000h-CFFFFFh  ROM (mapped via Port 4804h) (in HiROM fashion)
  D00000h-DFFFFFh  ROM (mapped via Port 4805h) (in HiROM fashion)
  E00000h-EFFFFFh  ROM (mapped via Port 4806h) (in HiROM fashion)
  F00000h-FFFFFFh  ROM (mapped via Port 4807h) (in HiROM fashion)
```

**S-DD1 PCBs**

```
  SHVC-1NON-01  CartSlotPin59 not connected (no C12 capacitor on PA1 pin)
  SHVC-1NON-10  Strange revision (capacitor C12 between PA1 and GND)
  SNSP-1NON-10  PAL version (S-DD1.Pin82 wired to ... VCC?) (also with C12)
  SHVC-LN3B-01  Version with additional SRAM for Star Ocean
```

The 1NON board contains only two chips (100pin D-DD1 and 44pin ROM), the CIC
function is included in the S-DD1, whereas Pin82 does probably select
"PAL/NTSC" CIC mode.

The LN3B-board contains five chips (two 44pin ROMs, S-DD1, 8Kx8bit SRAM, and a
MM1026AF battery controller).

**S-DD1 Pinouts**

```
  1-81   Unknown
  82     PAL/NTSC (for CIC mode)
  83-100 Unknown
```

### SNES Cart S-DD1 Decompression Algorithm

**decompress_init(src)**

```
  input=[src], src=src+1
  if (input AND C0h)=00h then num_planes = 2
  if (input AND C0h)=40h then num_planes = 8
  if (input AND C0h)=80h then num_planes = 4
  if (input AND C0h)=C0h then num_planes = 0
  if (input AND 30h)=00h then high_context_bits=01c0h, low_context_bits=0001h
  if (input AND 30h)=10h then high_context_bits=0180h, low_context_bits=0001h
  if (input AND 30h)=20h then high_context_bits=00c0h, low_context_bits=0001h
  if (input AND 30h)=30h then high_context_bits=0180h, low_context_bits=0003h
  input=(input SHL 11) OR ([src+1] SHL 3), src=src+1, valid_bits=5
  for i=0 to 7 do bit_ctr[i]=00h, prev_bits[i]=0000h
  for i=0 to 31 do context_states[i]=00h, context_MPS[i]=00h
  plane=0, yloc=0, raw=0
```

**decompress_byte(src,dst)**

```
  if num_planes=0
    for plane=0 to 7 do GetBit(plane)
    [dst]=raw, dst=dst+1
  else if (plane AND 1)=0
    for i=0 to 7 do GetBit(plane+0), GetBit(plane+1)
    [dst]=prev_bits[plane] AND FFh, dst=dst+1, plane=plane+1
  else
    [dst]=prev_bits[plane] AND FFh, dst=dst+1, plane=plane-1
    yloc=yloc+1, if yloc=8 then yloc=0, plane = (plane+2) AND (num_planes-1)
```

**GetBit(plane)**

```
  context = (plane AND 1) SHL 4
  context = context OR ((prev_bits[plane] AND high_context_bits) SHR 5)
  context = context OR (prev_bits[plane] AND low_context_bits)
  pbit=ProbGetBit(context)
  prev_bits[plane] = (prev_bits[plane] SHL 1) + pbit
  if num_planes=0 then raw = (raw SHR 1)+(pbit SHL 7)
```

**ProbGetBit(context)**

```
  state=context_states[context]
  code_size=EvolutionCodeSize[state]
  if (bit_ctr[code_size] AND 7Fh)=0 then
    bit_ctr[code_size]=GetCodeword(code_size)
  pbit=context_MPS[context]
  bit_ctr[code_size] = bit_ctr[code_size]-1
  if bit_ctr[code_size]=00h    ;"GolombGetBit"
    context_states[context]=EvolutionLpsNext[state]
    pbit=pbit XOR 1
    if state<2 then context_MPS[context]=pbit
  else if bit_ctr[code_size]=80h
    context_states[context]=EvolutionMpsNext[state]
  return pbit
```

**GetCodeword(code_size)**

```
  if valid_bits=0 then input=input OR [src], src=src+1, valid_bits=8
  input=input SHL 1, valid_bits=valid_bits-1
  if (input AND 8000h)=0 return 80h+(1 SHL code_size)
  tmp=((input SHR 8) AND 7Fh) OR (7Fh SHR code_size)
  input=input SHL code_size, valid_bits=valid_bits-code_size
  if valid_bits<0 then
    input=input OR (([src] SHL (-valid_bits))
    src=src+1, valid_bits=valid_bits+8
  return RunTable[tmp]
```

**EvolutionCodeSize[0..32]**

```
  0 , 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3
  4 , 4, 5, 5, 6, 6, 7, 7, 0, 1, 2, 3, 4, 5, 6, 7
```

**EvolutionMpsNext[0..32]**

```
  25, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17
  18,19,20,21,22,23,24,24,26,27,28,29,30,31,32,24
```

**EvolutionLpsNext[0..32]**

```
  25, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15
  16,17,18,19,20,21,22,23, 1, 2, 4, 8,12,16,18,22
```

**RunTable[0..127]**

```
  128, 64, 96, 32, 112, 48, 80, 16, 120, 56, 88, 24, 104, 40, 72, 8
  124, 60, 92, 28, 108, 44, 76, 12, 116, 52, 84, 20, 100, 36, 68, 4
  126, 62, 94, 30, 110, 46, 78, 14, 118, 54, 86, 22, 102, 38, 70, 6
  122, 58, 90, 26, 106, 42, 74, 10, 114, 50, 82, 18,  98, 34, 66, 2
  127, 63, 95, 31, 111, 47, 79, 15, 119, 55, 87, 23, 103, 39, 71, 7
  123, 59, 91, 27, 107, 43, 75, 11, 115, 51, 83, 19,  99, 35, 67, 3
  125, 61, 93, 29, 109, 45, 77, 13, 117, 53, 85, 21, 101, 37, 69, 5
  121, 57, 89, 25, 105, 41, 73,  9, 113, 49, 81, 17,  97, 33, 65, 1
```

### SNES Cart SPC7110 (Data Decompressor) (3 games)

The SPC7110 (full name "SPC7110F0A" or "SPC7110Foa") is a 100pin Data
Decompression chip from Epson/Seiko, used only by three games from Hudson soft:

```
  Far East of Eden Zero (with RTC-4513) (1995) Red Company/Hudson Soft (JP)
  Momotaro Dentetsu Happy (1996) Hudson Soft (JP)
  Super Power League 4 (1996) Hudson Soft (JP)
```

XXX add info from byuu's "spc7110-mcu.txt" file.

**Pinouts**

### SNES Cart SPC7110 Memory and I/O Map

**Memory Map**

```
  4800h..4842h      SPC7110 I/O Ports
  6000h..7FFFh      Battery-backed SRAM (8K bytes, in all 3 games)
  8000h..FFFFh      Exception Handlers (Program ROM offset 8000h..FFFFh)
  C00000h..CFFFFFh  Program ROM (1MByte) (HiROM)
  D00000h..DFFFFFh  Data ROM (1MByte-fragment mapped via Port 4831h)
  E00000h..EFFFFFh  Data ROM (1MByte-fragment mapped via Port 4832h)
  F00000h..FFFFFFh  Data ROM (1MByte-fragment mapped via Port 4833h)
```

I/O Ports and SRAM are probably mirrored to banks 00h-3Fh and 80h-BFh.

Program/Data ROM is probably mirrored to 400000h-7FFFFFh, the upper 32K
fragments of each 64K bank probably also to banks 00h-3Fh and 80h-BFh.

**Reportedly (probably nonsense?)**

"data decompressed from data rom by spc7110 mapped to $50:0000-$50:FFFF".

That info would imply that "decompressed data" Port 4800h is mirrored to
500000h-5FFFFFh (though more likely, the "un-decompressed data" is mirrored
from D00000h-DFFFFFh).

**ROM-Image Format**

The existing SPC7110 games are 2MB, 3MB, 5MB in size. Stored like so:

```
  000000h..0FFFFFh  Program ROM (1MByte) (HiROM)
  100000h..xFFFFFh  Data ROM (1MByte, 2MByte, or 4MByte max)
```

Observe that the SPC7110 ROM checksums at [FFDCh..FFDFh] are calculated
unconventionally: 3MB/5MB aren't "rounded-up" to 4MB/8MB. Instead, 3MB is
checksummed twice (rounded to 6MB). 2MB/5MB are checksummed as 2MB/5MB (without
rounding).

**Data ROM Decompression Ports**

```
  4800h --  Decompressed Data Read
  4801h 00  Compressed Data ROM Directory Base, bit0-7
  4802h 00  Compressed Data ROM Directory Base, bit8-15
  4803h 00  Compressed Data ROM Directory Base, bit16-23
  4804h 00  Compressed Data ROM Directory Index
  4805h 00  Decompressed Data RAM Target Offset, bit0-7    OFFSET IN BANK $50
  4806h 00  Decompressed Data RAM Target Offset, bit8-15   OFFSET IN BANK $50
  4807h 00  Unknown ("DMA Channel for Decompression")
  4808h 00  Unknown ("C r/w option, unknown")
  4809h 00  Decompressed Data Length Counter, bit0-7
  480Ah 00  Decompressed Data Length Counter, bit8-15
  480Bh 00  Unknown ("Decompression Mode")
  480Ch 00  Decompression Status (bit7: 0=Busy/Inactive, 1=Ready/DataAvailable)
```

**Direct Data ROM Access**

```
  4810h 00  Data ROM Read from [Base] or [Base+Offs], and increase Base or Offs
  4811h 00  Data ROM Base, bit0-7   (R/W)
  4812h 00  Data ROM Base, bit8-15  (R/W)
  4813h 00  Data ROM Base, bit16-23 (R/W)
  4814h 00  Data ROM Offset, bit0-7   ;\optionally Base=Base+Offs
  4815h 00  Data ROM Offset, bit8-15  ;/on writes to both of these registers
  4816h 00  Data ROM Step, bit0-7
  4817h 00  Data ROM Step, bit8-15
  4818h 00  Data ROM Mode
  481Ah 00  Data ROM Read from [Base+Offset], and optionally set Base=Base+Offs
```

**Unsigned Multiply/Divide Unit**

```
  4820h 00  Dividend, Bit0-7 / Multiplicand, Bit0-7
  4821h 00  Dividend, Bit8-15 / Multiplicand, Bit8-15
  4822h 00  Dividend, Bit16-23
  4823h 00  Dividend, Bit24-31
  4824h 00  Multiplier, Bit0-7
  4825h 00  Multiplier, Bit8-15, Start Multiply on write to this register
  4826h 00  Divisor, Bit0-7
  4827h 00  Divisor, Bit8-15, Start Division on write to this register
  4828h 00  Multiply/Divide Result, Bit0-7
  4829h 00  Multiply/Divide Result, Bit8-15
  482Ah 00  Multiply/Divide Result, Bit16-23
  482Bh 00  Multiply/Divide Result, Bit24-31
  482Ch 00  Divide Remainder, Bit0-7
  482Dh 00  Divide Remainder, Bit8-15
  482Eh 00  Multiply/Divide Reset  (write = reset 4820h..482Dh) (write 00h)
  482Fh 00  Multiply/Divide Status (bit7: 0=Ready, 1=Busy)
```

**Memory Mapping**

```
  4830h 00  SRAM Chip Enable/Disable (bit7: 0=Disable, 1=Enable)
  4831h 00  Data ROM Bank for D00000h-DFFFFFh (1MByte, using HiROM mapping)
  4832h 01  Data ROM Bank for E00000h-EFFFFFh (1MByte, using HiROM mapping)
  4833h 02  Data ROM Bank for F00000h-FFFFFFh (1MByte, using HiROM mapping)
  4834h 00  SRAM Bank Mapping?, workings unknown
```

**Real-Time Clock Ports (for external RTC-4513)**

```
  4840h 00  RTC Chip Enable/Disable (bit0: 0=Disable, 1=Enable)
  4841h --  RTC Command/Index/Data Port
  4842h --  RTC Ready Status
```

### SNES Cart SPC7110 Decompression I/O Ports

**4800h - Decompressed Data Read**

Reading from this register returns one decompressed byte, and does also
decrease the 16bit length counter [4809h] by one.

**4801h - Compressed Data ROM Directory Base, bit0-7**

**4802h - Compressed Data ROM Directory Base, bit8-15**

**4803h - Compressed Data ROM Directory Base, bit16-23**

**4804h - Compressed Data ROM Directory Index**

Selects a directory entry in Data ROM at [Base+Index*4]. Each entry is 4-bytes
in size:

```
  Byte0  Decompression Mode (00h,01h,02h)
  Byte1  Compressed Data ROM Source Pointer, bit16-23  ;\ordered as so
  Byte2  Compressed Data ROM Source Pointer, bit8-15   ; (ie. big-endian)
  Byte3  Compressed Data ROM Source Pointer, bit0-7    ;/
```

**4805h - Decompressed Data RAM Target Offset, bit0-7    OFFSET IN BANK $50**

**4806h - Decompressed Data RAM Target Offset, bit8-15   OFFSET IN BANK $50**

Reportedly: Destination address in bank 50h, this would imply that the SPC7110
chip contains around 64Kbytes on-chip RAM, which is probably utmost nonsense.

Or, reportedly, too: Causes the first "N" decompressed bytes to be skipped,
before data shows up at 4800h. That sounds more or less reasonable. If so,
unknown if the hardware does decrement the offset value?

**4807h - DMA Channel for Decompression**

Unknown. Reportedly "DMA CHANNEL FOR DECOMPRESSION, set to match snes dma
channel used for compressed data". That info seems to be nonsense; the
registers seems to be always set to 00h, no matter if/which DMA channel is
used.

**4808h - C r/w option, unknown**

Unknown. Reportedly "C r/w option, unknown".

**4809h - Decompressed Data Length Counter, bit0-7**

**480Ah - Decompressed Data Length Counter, bit8-15**

This counter is decremented on reads from [4800h]. One can initialize the
counter before decompression & check its value during decompression.
However, this doesn't seem to be required hardware-wise; the decompression
seems to be working endless (as long as software reads [4800h]), and doesn't
seem to "stop" when the length counter becomes zero.

**480Bh - Decompression Mode**

Reportedly:

```
  00 - manual decompression, $4800 is used to read directly from the data rom
```

```
  02 - hardware decompression, decompressed data is mapped to $50:0000,
       $4800 can be used to read sequentially from bank $50
```

**480Ch - Decompression Status (bit7: 0=Busy/Inactive, 1=Ready/DataAvailable)**

Reportedly:

```
  DECOMPRESSION FINISHED STATUS:
  high bit set = done, high bit clear = processing,
  cleared after successful read,
  high bit is cleared after writing to $4806,
  $4809/A is set to compressed data length
  ---
  decompression mode is activated after writing to $4806
  and finishes after reading the high bit of $480C
```

### SNES Cart SPC7110 Direct Data ROM Access

**4810h Data ROM Read from [Base] or [Base+Offs], and increase Base or Offs**

**481Ah Data ROM Read from [Base+Offset], and optionally set Base=Base+Offs**

Reportedly,

Testing leads to believe that the direct ROM read section starts out as
inactive.

One of the ways to activate direct reads is to write a non-zero value to $4813.

No other action need be taken. You can write a non-zero value and immediately

write a zero to it and that's OK.  The order of writes to $4811/2/3 don't

seem to matter so long as $4813 has been written to once with a non-zero

value.  There may be a way to deactivate the direct reads again (maybe a

decompression cycle?).

There appears to be another way to activate direct reads that is more complex.

**4811h Data ROM Base, bit0-7   (R/W)**

**4812h Data ROM Base, bit8-15  (R/W)**

**4813h Data ROM Base, bit16-23 (R/W)**

**4814h Data ROM Offset, bit0-7   ;\optionally Base=Base+Offs**

**4815h Data ROM Offset, bit8-15  ;/on writes to both of these registers**

**4816h Data ROM Step, bit0-7**

**4817h Data ROM Step, bit8-15**

**4818h Data ROM Mode**

```
  0   Select Step   (for 4810h) (0=Increase by 1, 1=Increase by "Step" Value)
  1   Enable Offset (for 4810h) (0=Disable/Read Ptr, 1=Enable/Read Ptr+Offset)
  2   Expand Step from 16bit to 24bit           (0=Zero-expand, 1=Sign-expand)
  3   Expand Offset from 8bit?/16bit to 24bit   (0=Zero-expand, 1=Sign-expand)
  4   Apply Step (after 4810h read)    (0=On 24bit Pointer, 1=On 16bit Offset)
  5-6 Special Actions (see below)
  7   Unused (should be zero)
```

Special Actions:

```
  0=No special actions
  1=After Writing $4814/5 --> 8 bit offset addition using $4814
  2=After Writing $4814/5 --> 16 bit offset addition using $4814/5
  3=After Reading $481A   --> 16 bit offset addition using $4814/5
```

Reportedly,

```
  4818 write: set command mode,
  4818 read: performs action instead of returning value, unknown purpose
  command mode is loaded to $4818 but only set after writing to both $4814
  and $4815 in any order
  $4811/2/3 may increment on a $4810 read depending on mode byte)
  $4814/$4815 is sometimes incremented on $4810 reads (depending on mode byte)
```

Note: the data rom command mode is activated only after registers $4814 and
$4815 have been written to, regardless of the order they were written to

**4831h Data ROM Bank for D00000h-DFFFFFh (1MByte, using HiROM mapping)**

**4832h Data ROM Bank for E00000h-EFFFFFh (1MByte, using HiROM mapping)**

**4833h Data ROM Bank for F00000h-FFFFFFh (1MByte, using HiROM mapping)**

**4830h SRAM Chip Enable/Disable (bit7: 0=Disable, 1=Enable)**

**4834h SRAM Bank Mapping?, workings unknown**

### SNES Cart SPC7110 Multiply/Divide Unit

**Unsigned Multiply/Divide Unit**

```
  4820h Dividend, Bit0-7 / Multiplicand, Bit0-7
  4821h Dividend, Bit8-15 / Multiplicand, Bit8-15
  4822h Dividend, Bit16-23
  4823h Dividend, Bit24-31
  4824h Multiplier, Bit0-7
  4825h Multiplier, Bit8-15, Start Multiply on write to this register
  4826h Divisor, Bit0-7
  4827h Divisor, Bit8-15, Start Division on write to this register
  4828h Multiply/Divide Result, Bit0-7
  4829h Multiply/Divide Result, Bit8-15
  482Ah Multiply/Divide Result, Bit16-23
  482Bh Multiply/Divide Result, Bit24-31
  482Ch Divide Remainder, Bit0-7
  482Dh Divide Remainder, Bit8-15
  482Eh Multiply/Divide Reset  (write = reset 4820h..482Dh) (write 00h)
  482Fh Multiply/Divide Status (bit7: 0=Ready, 1=Busy)
```

**Unknown Stuff**

Multiply/Divide execution time is unknown. Is it constant/faster for small
values? Behaviour on Divide by 0 is unknown?

Purpose of 482Eh is unknown, does it really "reset" 4820h..482Dh? Meaning that
those registers are set to zero? Is that required/optional?

Are there other modes, like support for signed-numbers, or a fast 8bit*8bit
multiply mode or such?

**Reportedly**

```
  482Eh.bit0  (0=unsigned, 1=signed)
  (un)signed div0 returns --> result=00000000h, remainder=dividend AND FFFFh
  -80000000h/-1 returns <unknown> ?
```

### SNES Cart SPC7110 with RTC-4513 Real Time Clock (1 game)

RTC from Epson/Seiko. Used by one game from Hudson Soft:

```
  Far East of Eden Zero (with RTC-4513) (1995) Red Company/Hudson Soft (JP)
```

**SPC7110 I/O Ports for RTC-4513 Access**

```
  4840h RTC Chip Select (bit0: 0=Deselect: CE=LOW, 1=Select: CE=HIGH)
  4841h RTC Data Port   (bit0-3: Command/Index/Data)
  4842h RTC Status      (bit7: 1=Ready, 0=Busy) (for 4bit transfers)
```

**Usage**

Switch CE from LOW to HIGH, send Command (03h=Write, 0Ch=Read), send starting
Index (00h..0Fh), then read or write one or more 4bit Data units (index will
automatically increment after each access, and wraps from 0Fh to 00h at end of
data stream). Finally, switch CE back LOW.

**Epson RTC-4513 Commands**

```
  03h    Write-Mode
  0Ch    Read-Mode
```

**Epson RTC-4513 Register Table**

```
  Index  Bit3   Bit2   Bit1    Bit0   Expl.
  0      Sec3   Sec2   Sec1    Sec0   Seconds, Low
  1      LOST   Sec6   Sec5    Sec4   Seconds, High
  2      Min3   Min2   Min1    Min0   Minutes, Low
  3      WRAP   Min6   Min5    Min4   Minutes, High
  4      Hour3  Hour2  Hour1   Hour0  Hours, Low
  5      WRAP   PM/AM  Hour5   Hour4  Hours, High
  6      Day3   Day2   Day0    Day0   Day, Low     ;\
  7      WRAP   RAM    Day5    Day4   Day, High    ;
  8      Mon3   Mon2   Mon1    Mon0   Month, Low   ; or optionally,
  9      WRAP   RAM    RAM     Mon4   Month, High  ; 6x4bit User RAM
  A      Year3  Year2  Year1   Year0  Year, Low    ;
  B      Year7  Year6  Year5   Year4  Year, High   ;/
  C      WRAP   Week2  Week1   Week0  Day of Week
  D      30ADJ  IRQ-F  CAL/HW  HOLD   Control Register D
  E      RATE1  RATE0  DUTY    MASK   Control Register E
  F      TEST   24/12  STOP    RESET  Control Register F
```

Whereas, the meaning of the various bits is:

```
  Sec    Seconds (BCD, 00h..59h)
  Min    Minutes (BCD, 00h..59h)
  Hour   Hours   (BCD, 00h..23h or 01h..12h)
  Day    Day     (BCD, 01h..31h)
  Month  Month   (BCD, 01h..12h)
  Year   Year    (BCD, 00h..99h)
  Week   Day of Week (0..6) (Epson suggests 0=Monday as an example)
  PM/AM  Set for PM, cleared for AM (is that also in 24-hour mode?)
  WRAP   Time changed during access (reset on CE=LOW, set on seconds increase)
  HOLD   Pause clock when set (upon clearing increase seconds by 1 if needed)
  LOST   Time lost (eg. battery failure) (can be reset by writing 0)
  IRQ-F  Interrupt Flag (Read-only, set when: See Rate, cleared when: See Duty)
  RATE   Interrupt Rate (0=Per 1/64s, 1=Per Second, 2=Per Minute, 3=Per Hour)
  DUTY   Interrupt Duty (0=7.8ms, 1=Until acknowledge, ie. until IRQ-F read)
  MASK   Interrupt Disable (when set: IRQ-F always 0, STD.P always High-Z)
  TEST   Reserved for Epson's use (should be 0) (auto-cleared on CE=LOW)
  RAM    General purpose RAM (usually 3bits) (24bits when Calendar=off)
  CAL/HW Calendar Enable (1=Yes/Normal, 0=Use Day/Mon/Year as 24bit user RAM)
  24/12  24-Hour Mode (0=12, 1=24) (Time/Date may get corrupted when changed!)
  30ADJ  Set seconds to zero, and, if seconds was>=30, increase minutes
  STOP   Stop clock while set (0=Stop, 1=Normal)
  RESET  Stop clock and reset seconds to 00h (auto-cleared when CE=LOW)
```

If WRAP=1 then one must deselect the chip, and read time/date again.

Serial data is transferred LSB first.

On-chip 32.768kHz quartz crystal.

**Pin-Outs**

### SNES Cart SPC7110 Decompression Algorithm

**decompress_mode0(src,dst,len)**

```
  initialize
  while len>0
    decoded=0
    con=0, decompression_core
    con=1+decoded, decompression_core
    con=3+decoded, decompression_core
    con=7+decoded, decompression_core
    out = (out SHL 4) XOR (((out SHR 12) XOR decoded) AND Fh)
    decoded=0
    con=15, decompression_core
    con=15+1+decoded, decompression_core
    con=15+3+decoded, decompression_core
    con=15+7+decoded, decompression_core
    out = (out SHL 4) XOR (((out SHR 12) XOR decoded) AND Fh)
    [dst]=(out AND FFh), dst=dst+1, len=len-1
```

**decompress_mode1(src,dst,len)**

```
  initialize
  while len>0
   if (buf_index AND 01h)=0
    for pixel=0 to 7
      a = (out SHR 2)  AND 03h
      b = (out SHR 14) AND 03h
      decoded=0
      con = get_con(a,b,c)
      decompression_core
      con = con*2+5+decoded
      decompression_core
      do_pixel_order(a,b,c,2,decoded)
    plane0.bits(7..0) = out.bits(15,13,11,9,7,5,3,1)
    plane1.bits(7..0) = out.bits(14,12,10,8,6,4,2,0)
    [dst]=plane0
   else
    [dst]=plane1
   buf_index=buf_index+1, dst=dst+1, len=len-1
```

**decompress_mode2(src,dst,len)**

```
  initialize
  while len>0
   if (buf_index AND 11h)=0
    for pixel=0 to 7
      a = (out SHR 0)  AND 0Fh
      b = (out SHR 28) AND 0Fh
      decoded=0
      con=0
      decompression_core
      con=decoded+1
      decompression_core
      if con=2 then con=decoded+11 else con = get_con(a,b,c)+3+decoded*5
      decompression_core
      con=Mode2ContextTable[con]+(decoded AND 1)
      decompression_core
      do_pixel_order(a,b,c,4,decoded)
    plane0.bits(7..0) = out.bits(31,27,23,19,15,11,7,3)
    plane1.bits(7..0) = out.bits(30,26,22,18,14,10,6,2)
    plane2.bits(7..0) = out.bits(29,25,21,17,13, 9,5,1)
    plane3.bits(7..0) = out.bits(28,24,20,16,12, 8,4,0)
    bitplanebuffer[buf_index+0] = plane2
    bitplanebuffer[buf_index+1] = plane3
    [dst]=plane0
   else if (buf_index AND 10h)=0
    [dst]=plane1
   else
    [dst]=bitplanebuffer[buf_index AND 0Fh]
   buf_index=buf_index+1, dst=dst+1, len=len-1
```

**initialize**

```
  src=directory_base+(directory_index*4)
  mode=[src+0]
  src=[src+3]+[src+2]*100h+[src+1]*10000h  ;big-endian (!)
  buf_index=0
  out=00000000h
  c=0
  top=255
  val.msb=[src], val.lsb=00h, src=src+1, in_count=0
  for i=0 to 15 do pixelorder[i]=i
  for i=0 to 31 do ContextIndex[i]=0, ContextInvert[i]=0
```

**decompression_core**

```
  decoded=(decoded SHL 1) xor ContextInvert[con]
  evl=ContextIndex[con]
  top = top - EvolutionProb[evl]
  if val.msb > top
    val.msb = val.msb-(top-1)
    top = EvolutionProb[evl]-1
    if top>79 then ContextInvert[con] = ContextInvert[con] XOR 1
    decoded = decoded xor 1
    ContextIndex[con] = EvolutionNextLps[evl]
  else
    if top<=126 then ContextIndex[con] = EvolutionNextMps[evl]
  while(top<=126)
    if in_count=0 then val.lsb=[src], src=src+1, in_count=8
    top = (top SHL 1)+1
    val = (val SHL 1), in_count=in_count-1    ;16bit val.msb/lsb
```

**do_pixel_order(a,b,c,shift,decoded)**

```
  m=0, x=a, repeat, exchange(x,pixelorder[m]), m=m+1, until x=a
  for m=0 to (1 shl shift)-1 do realorder[m]=pixelorder[m]
  m=0, x=c, repeat, exchange(x,realorder[m]), m=m+1, until x=c
  m=0, x=b, repeat, exchange(x,realorder[m]), m=m+1, until x=b
  m=0, x=a, repeat, exchange(x,realorder[m]), m=m+1, until x=a
  out = (out SHL shift) + realorder[decoded]
  c = b
```

**get_con(a,b,c)**

```
  if (a=b AND b=c) then return=0
  else if (a=b) then return=1
  else if (b=c) then return=2
  else if (a=c) then return=3
  else return=4
```

**EvolutionProb[0..52]**

```
  90,37,17, 8, 3, 1,90,63,44,32,23,17,12, 9, 7, 5, 4, 3, 2
  90,72,58,46,38,31,25,21,17,14,11, 9, 8, 7, 5, 4, 4, 3, 2
  2 ,88,77,67,59,52,46,41,37,86,79,71,65,60,55
```

**EvolutionNextLps[0..52]**

```
  1 , 6, 8,10,12,15, 7,19,21,22,23,25,26,28,29,31,32,34,35
  20,39,40,42,44,45,46,25,26,26,27,28,29,30,31,33,33,34,35
  36,39,47,48,49,50,51,44,45,47,47,48,49,50,51
```

**EvolutionNextMps[0..52]**

```
  1 , 2, 3, 4, 5, 5, 7, 8, 9,10,11,12,13,14,15,16,17,18, 5
  20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38
  5 ,40,41,42,43,44,45,46,24,48,49,50,51,52,43
```

**Mode2ContextTable[0..14]  ;only entries 3..14 used (entries 0..2 = dummies)**

```
  0 ,0 ,0 ,15,17,19,21,23,25,25,25,25,25,27,29
```

### SNES Cart SPC7110 Notes

**Compression/Decompression Example**

Uncompressed Data (64-byte ASCII string):

```
  Test123.ABCDABCDAAAAAAAAaaaabbbbccccdddd7654321076543210.Test123
```

Compressed in Mode0:

```
  68 91 36 15 F8 BF 42 35 2F 67 3D B7 AA 05 B4 F7 70 7A 26 20 EA 58 2C 09 61 00
  C5 00 8C 6F FF D1 42 9D EE 7F 72 87 DF D6 5F 92 65 00 00
```

Compressed in Mode1:

```
  4B F6 80 1E 3A 4C 42 6C DA 16 0F C6 44 ED 64 10 77 AF 50 00 05 C0 01 27 22 B0
  83 51 05 32 4A 1E 74 93 08 76 07 E5 32 12 B4 99 9E 55 A3 F8 00
```

Compressed in Mode2:

```
  13 B3 27 A6 F4 5C D8 ED 6C 6D F8 76 80 A7 87 20 39 4B 37 1A CC 3F E4 3D BE 65
  2D 89 7E 0B 0A D3 46 D5 0C 1F D3 81 F3 AD DD E8 5C C0 BD 62 AA CB F8 B5 38 00
```

**Selftest Program**

All three SPC7110 games include a selftest function (which executes on initial
power-up, ie. when the battery-backed SRAM is still uninitialized). Press
Button A/B to start 1st/2nd test, and push Reset Button after each test.

**PCBs**

```
  SHVC-BDH3B-01 (without RTC)
  SHVC-LDH3C-01 (with RTC)
```

### SNES Cart Unlicensed Variants

**Gamars Puzzle (Kaiser)**

A LoROM game with SRAM at 316000h (unlike normal LoROM games that have SRAM at
70xxxxh). Cartridge Header Maker entry is [FFDAh]=00h, and SRAM size entry is
[FFD8h]=20h (4096 gigabytes), the actual size of the SRAM is unknown.

**Bootlegs**

The "bootleg" games are semi-illegal pirate productions, typically consisting
of a custom (and not-so-professional) game engine, bundled with graphics and
sounds ripped from commercial games. Some of these cartridges are containing
some small copy-protection hardware (see below).

**Copy-Protected Bootlegs (Standard "bitswap" variant)**

This type is used by several games:

```
  A Bug's Life                          2MB, CRC32=014F0FCFh
  Aladdin 2000                          2MB, CRC32=752A25D3h
  Bananas de Pijamas                    1MB, CRC32=52B0D84Bh
  Digimon Adventure                     2MB, CRC32=4F660972h
  King of Fighters 2000 (aka KOF2000)   3MB, CRC32=A7813943h
  Pocket Monster (aka Picachu)          2MB, CRC32=892C6765h
  Pokemon Gold Silver                   2MB, CRC32=7C0B798Dh
  Pokemon Stadium                       2MB, CRC32=F863C642h
  Soul Edge Vs Samurai                  2MB, CRC32=5E4ADA04h
  Street Fighter EX Plus Alpha          2MB, CRC32=DAD59B9Fh
  X-Men vs. Street Fighter              2MB, CRC32=40242231h
```

The protection hardware is mapped to:

```
  80-xx:8000-FFFF  Read 8bit Latch (bits re-ordered as: 0,6,7,1,2,3,4,5)
  88-xx:8000-FFFF  Write 8bit Latch (bits ordered as:   7,6,5,4,3,2,1,0)
```

**Copy-Protected Bootlegs (Soulblade "constant" variant)**

This type is used by only one game:

```
  Soul Blade                            3MB, CRC32=C97D1D7Bh
```

The protection hardware consists of a read-only pattern, mapped to:

```
  80-BF:8000-FFFF  Filled with a constant 4-byte pattern (55h,0Fh,AAh,F0h)
  C0-FF:0000-FFFF  Open bus (not used)
```

**Copy-Protected Bootlegs (Tekken2 "alu/flipflop" variant)**

This type is used by only one game:

```
  Tekken 2                              2MB, CRC32=066687CAh
```

The protection hardware is mapped to:

```
  [80-BF:80xx]=0Fh,00h Clear all 6 bits
  [80-BF:81xx]=xxh     Probably "No Change" (unused, except for Reading)
  [80-BF:82xx]=FFh,00h Set Data bit0
  [80-BF:83xx]=FFh,00h Set Data bit1
  [80-BF:84xx]=FFh,00h Set Data bit2
  [80-BF:85xx]=FFh,00h Set Data bit3
  [80-BF:86xx]=FFh,00h Set ALU Direction bit (0=Up/Left, 1=Down/Right)
  [80-BF:87xx]=FFh,00h Set ALU Function bit  (0=Count, 1=Shift)
  X=[80-BF:81xx]       Return "4bitData plus/minus/shl/shr 1"
  ;the above specs are based on 12 known/guessed results (as guessed by d4s),
  ;the remaining 52 combinations are probably following same rules (not tested
  ;on real hardware). theoretically some ports might do things like "set bitX
  ;and clear bitY", in that case, there would be more than 64 combinations.
```

The hardware is often missing I/O accesses, unless one is repeating them some
dozens of times; the existing game is issuing 240 words (480 bytes) to
write-ports, and reads 256 words (512 bytes) from the read-port. The reads
contain the result in lower 4bit (probably in both low-byte and high-byte of
the words) (and unknown/unused stuff in the other bits).

The set/clear ports are said to react on both reads and writes (which would
imply that the written data is don't care).

### SNES Cart S-RTC (Realtime Clock) (1 game)

PCB "SHVC-LJ3R-01" with 24pin "Sharp S-RTC" chip. Used only by one japanese
game:

```
  Dai Kaiju Monogatari 2 (1996) Birthday/Hudson Soft (JP)
```

**S-RTC I/O Ports**

```
  002800h S-RTC Read  (R)
  002801h S-RTC Write (W)
```

Both registers are 4bits wide. When writing: Upper 4bit should be zero. When
reading: Upper 4bit should be masked-off (they do possibly contain garbage, eg.
open-bus).

**S-RTC Communication**

The sequence for setting, and then reading the time is:

```
  Send <0Eh,04h,0Dh,0Eh,00h,Timestamp(12 digits),0Dh> to [002801h]
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  etc.
```

The exact meaning of the bytes is unknown. 0Eh/0Dh seems to invoke/terminate
commands, 04h might be some configuration stuff (like setting 24-hour mode).
00h is apparently the set-time command. There might be further commands (such
like setting interrupts, alarm, 12-hour mode, reading battery low & error
flags, etc.). When reading, 0Fh seems to indicate sth like "time available".

The 12/13-digit "SSMMHHDDMYYY(D)" Timestamps are having the following format:

```
  Seconds.lo  (BCD, 0..9)
  Seconds.hi  (BCD, 0..5)
  Minutes.lo  (BCD, 0..9)
  Minutes.hi  (BCD, 0..5)
  Hours.lo    (BCD, 0..9)
  Hours.hi    (BCD, 0..2)
  Day.lo      (BCD, 0..9)
  Day.hi      (BCD, 0..3)
  Month       (HEX, 01h..0Ch)
  Year.lo     (BCD, 0..9)
  Year.hi     (BCD, 0..9)
  Century     (HEX, 09h..0Ah for 19xx..20xx)
```

When READING the time, there is one final extra digit (the existing software
doesn't transmit that extra digit on WRITING, though maybe it's possible to do
writing, too):

```
  Day of Week? (0..6) (unknown if RTC assigns sth like 0=Sunday or 0=Monday)
```

**Pinouts**

**Note**

There's another game that uses a different RTC chip: A 4bit serial bus RTC-4513
(as made by Epson) connected to a SPC7110 chip.

### SNES Cart Super Gameboy

The Super Gameboy (SGB) is some kind of an adaptor for monochrome handheld
Gameboy games. The SGB cartridge contains a fully featured gameboy (with CPU,
Video & Audio controllers), but without LCD screen and without joypad
buttons.

The 4-grayshade 160x144 pixel video signal is forwarded to SNES VRAM and shown
on TV Set, and in the other direction, the SNES joypad data is forwarded to SGB
CPU.

Some gameboy games include additional SGB features, allowing to display a
256x224 pixel border that surrounds the 160x144 pixel screen, there are also
some (rather limited) functions for colorizing the monochrome screen, plus some
special Sound, OBJ, Joypad functions. Finally, the gameboy game can upload
program code to the SNES and execute it.

**Chipset**

```
  SGB CPU - 80pin - Super Gameboy CPU/Video/Audio Chip
  ICD2-R (or ICD2-N) - 44pin - Super Gameboy SGB-to-SNES Interface Chip
```

Plus VRAM/WRAM for SGB CPU, plus SNES SGB BIOS, plus CIC chip.

**SGB I/O Map (ICD2-R)**

```
  6000       R  LCD Character Row and Buffer Write-Row
  6001       W  Character Buffer Read Row Select
  6002       R  16-Byte Packet Available Flag
  6003       W  Reset/Multiplayer/Speed Control
  6004-6007  W  Controller Data for Player 1-4
  6008-600E  -  Unused (Open Bus, or mirror of 600Fh on some chips)
  600F       R  Chip Version (21h or 61h)
  6800-680F  -  Unused (Open Bus)
  7000-700F  R  16-byte command packet (addr 7000..700F)
  7800       R  Character Buffer Data (320 bytes of currently selected row)
  7801-780F  R  Unused (Mirrors of 7800h, not Open Bus)
```

The ICD2 chips decodes only A0-A3,A11-A15,A22 (so above is mirrored to various
addresses at xx6xxN/xx7xxN). Reading the Unused registers (and write-only ones)
returns garbage. On chips with [600Fh]=61h, that garbage is:

```
  CPU Open Bus values (though, for some reason, usually with bit3=1).
```

On chips with [600Fh]=21h, that garbage is:

```
  6001h.R, 6004h-6005h.R --> mirror of 6000h.R
  6003h.R, 6006h-6007h.R --> mirror of 6002h.R
  6008h-600Eh.R          --> mirror of 600Fh.R
```

On ICD2-N chips and/or such with [600Fh]=other, that garbage is: Unknown.

**SGB Port 6000h - LCD Character Row and Buffer Write-Row (R)**

```
  7-3  Current Character Row on Gameboy LCD (0..11h) (11h=Last Row, or Vblank)
  2    Seems to be always zero
  1-0  Current Character Row WRITE Buffer Number (0..3)
```

**SGB Port 6001h - Character Buffer Read Row Select (W)**

```
  7-2  Unknown/unused      (should be zero)
  1-0  Select Character Row READ Buffer Number (0..3)
```

Selects one of the four buffer rows (for reading via Port 7800h). Only the
three "old" buffers should be selected, ie. not the currently written row
(which is indicated in 6000h.Bit1-0).

**SGB Port 6002h - 16-Byte Packet Available Flag (R)**

```
  7-1  Seems to be always zero
  0    New 16-byte Packet Available (0=None, 1=Yes)
```

When set, a 16-byte SGB command packet can be read from 7000h-700Fh; of which,
reading 7000h does reset the flag in 6002h.

**SGB Port 6003h - Reset/Multiplayer/Speed Control (W)**

```
  7    Reset Gameboy CPU   (0=Reset, 1=Normal)
  6    Unknown/unused      (should be zero)
  5-4  num_controllers     (0,1,3=One,Two,Four)  (default 0=One Player)
  3-2  Unknown/unused      (should be zero)
  1-0  SGB CPU Speed       (0..3 = 5MHz,4MHz,3MHz,2.3MHz) (default 1=4MHz)
```

The LSBs select the SGB CPU Speed (the SNES 21MHz master clock divided by
4,5,7,9). Unknown if/how/when the SGB BIOS does use this. For the SGB, the
exact master clock depends on the console (PAL or NTSC). For the SGB2 it's
derived from a separate 20.9MHz oscillator.

**SGB Port 6004h-6007h - Controller Data for Player 1-4 (W)**

```
  7    Start     (0=Pressed, 1=Released)
  6    Select    (0=Pressed, 1=Released)
  5    Button B  (0=Pressed, 1=Released)
  4    Button A  (0=Pressed, 1=Released)
  3    Down      (0=Pressed, 1=Released)
  2    Up        (0=Pressed, 1=Released)
  1    Left      (0=Pressed, 1=Released)
  0    Right     (0=Pressed, 1=Released)
```

Used to forward SNES controller data to the gameboy Joypad inputs. Ports
6005h-6007h are used only in 2-4 player mode (which can be activated via 6003h;
in practice: this can be requested by SGB games via MLT_REQ (command 11h), see
SGB section in Pan Docs for details).

**SGB Port 600Fh - Chip Version (R)**

```
  7-0  ICD2 Chip Version
```

Seems to indicate the ICD2 Chip Version. Known values/versions are:

```
  21h = ICD2-R (without company logo on chip package)
  61h = ICD2-R (with company logo on chip package)
  ??  = ICD2-N (this one is used in SGB2)
```

The versions differ on reading unused/write-only ports (see notes in SGB I/O
map).

**SGB Port 7000h-700Fh - 16-byte Command Packet (R)**

```
  7-0  Data
```

Reading from 7000h (but not from 7001h-700Fh) does reset the flag in 6002h

Aside from regular SGB commands, the SGB BIOS (that in the SGB CPU chip) does
transfer six special packets upon Reset; these do contain gameboy cartridge
header bytes 104h..14Fh (ie. Nintendo Logo, Title, ROM/RAM Size, SGB-Enable
bytes, etc).

**SGB Port 7800h - Character Buffer Data (R)**

```
  7-0  Data (320 bytes; from Buffer Row number selected in Port 6001h)
```

This port should be used as fixed DMA source address for transferring 320 bytes
(one 160x8 pixel character row) to WRAM (and, once when the SNES is in Vblank,
the whole 160x144 pixels can be DMAed from WRAM to VRAM).

The ICD2 chip does automatically re-arrange the pixel color signals (LD0/LD1)
back to 8x8 pixel tiles with two bit-planes (ie. to the same format as used in
Gameboy and SNES VRAM).

The buffer index (0..511) is reset to 0 upon writing to Port 6001h, and is
automatically incremented on reading 7800h. When reading more than 320 bytes,
indices 320..511 return FFh bytes (black pixels), and, after 512 bytes, it
wraps to index 0 within the same buffer row.

**Gameboy Audio**

The stereo Gameboy Audio Output is fed to the External Audio Input on SNES
cartridge port, so sound is automatically forwarded to the TV Set, ie. software
doesn't need to process sound data (however, mind that the /MUTE signal of the
SNES APU must be released).

**SGB Commands**

Above describes only the SNES side of the Super Gameboy. For the Gameboy side
(ie. for info on sending SGB packets, etc), see SGB section in Pan Docs:

```
  http://problemkaputt.de/pandocs.htm
  http://problemkaputt.de/pandocs.txt
```

Some details that aren't described in (current) Pan Docs:

```
 * JUMP does always destroy the NMI vector (even if it's 000000h)
 * (The SGB BIOS doesn't seem to use NMIs, so destroying it doesn't harm)
 * JUMP can return via 16bit retadr (but needs to force program bank 00h)
 * After JUMP, all RAM can be used, except [0000BBh..0000BDh] (=NMI vector)
 * The IRQ/COP/BRK vectors/handlers are in ROM, ie. only NMIs can be hooked
 * APU Boot-ROM can be executed via MOV [2140h],FEh (but Echo-Write is kept on)
 * The TEST_EN command points to a RET opcode (ie. it isn't implemented)
 * Upon RESET, six packets with gameboy cart header are sent by gameboy bios
 * command 19h does allow to change an undoc flag (maybe palette related?)
 * command 1Ah..1Fh point to RET (no function) (except 1Eh = boot info)
 * sgb cpu speed can be changed (unknown if/how supported by sgb bios)
```

**Note**

There is a special controller, the SGB Commander (from Hori), which does
reportedly have special buttons for changing the CPU speed - unknown how it is
doing that (ie. unknown what data and/or ID bits it is transferring to the SNES
controller port).

Probably done by sending button sequences (works also with normal joypad):

```
 Codes for Super GameBoy Hardware
 Enter these codes very quickly for the desired effect.
  After choosing a border from 4 - 10, press L + R to exit.
   Press L, L, L, L, R, L, L, L, L, R. - Screen Savers
  At the Super Game Boy,
   press L, L, L, R, R, R, L, L, L, R, R, R, R, R, R, R - Super Gameboy Credits
  Hold UP as you turn on the SNES and then press L, R, R, L, L, R - Toggle Speed
  During a game, press L, R, R, L, L, R - Toggle Speed
  During a game, press R, L, L, R, R, L - Toggle Sound
  --
```

Screen Savers --> Choose a border from 4 to 10 and press L + R to exit.
Press L(4), R, L(4), R.

Super Gameboy Credits --> When you see the Super Game Boy screen appear,
press L, L, L, R, R, R, L, L, L, R, R, R, R, R, R, R

Toggle Speed (Fast, Normal, Slow, Very Slow)    Hold Up when powering up the
SNES, then press L, R, R, L, L, R very fast.

Toggle Speed (Normal, Slow, Very Slow)  During Gameplay, press L, R, R, L, L, R
very fast.

Un/Mute Sound --> During Gameplay, press R, L, L, R, R, L quite fast.

### SNES Cart Satellaview (satellite receiver & mini flashcard)

**Satellaview I/O Ports**

**Satellaview Transmission Format**

**Satellaview Memory**

**Other Satellaview Info**

### SNES Cart Satellaview I/O Map

**Receiver I/O Map (DCD-BSA chip)**

```
  2188h Stream 1 Hardware Channel Number, Lsb (R/W)
  2189h Stream 1 Hardware Channel Number, Msb (R/W)
  218Ah Stream 1 Queue Size (number of received 1+22 byte Units) (R)
  218Bh Stream 1 Queue 1-byte Status Units (Read=Data, Write=Reset)
  218Ch Stream 1 Queue 22-byte Data Units  (Read=Data, Write=Reset/Ack)
  218Dh Stream 1 Status Summary (R)
  218Eh Stream 2 Hardware Channel Number, Lsb (R/W)
  218Fh Stream 2 Hardware Channel Number, Msb (R/W)
  2190h Stream 2 Queue Size (number of received 1+22 byte Units) (R)
  2191h Stream 2 Queue 1-byte Status Unit(s?) (Read=Data, Write=Reset)
  2192h Stream 2 Queue 22-byte? Data Unit(s?) (Read=Data, Write=Reset/Ack)
  2193h Stream 2 Status Summary (R)
  2194h POWER (bit0) and ACCESS (bit2-3) LED Control? (R/W)
  2195h Unknown/Unused, maybe for EXT Expansion Port (?)
  2196h Status (only bit1 is tested) (R)
  2197h Control (only bit7 is modified) (R/W)
  2198h Serial I/O Port 1 (R/W)
  2199h Serial I/O Port 2 (R/W)
```

**Flash Card I/O Map (when mapped to bank C0h and up)**

```
  C00000h  Type 1-4   Detection Command             (W)
  C00002h  Type 1-4   Detection Status              (R)
  C0FFxxh  Type 1-4   Detection Response            (R)
  C00000h  Type 1,3,4 Command for Type 1,3,4        (W)
  C00000h  Type 1,3,4 Status (normal commands)      (R)
  C00004h  Type 1,3   Status (erase-entire command) (R)
  C02AAAh  Type 2     Command/Key for Type2         (W)
  C05555h  Type 2     Command/Status for Type2      (R/W)
  xx0000h  Type 1-4   Erase 64K Sector Address      (W)
  xxxxxxh  Type 1-4   Write Data Address            (W)
```

**BIOS Cartridge MCC-BSC Chip Ports**

```
  005000h Unknown/Unused
  015000h Bank 00h-3Fh and 80h-FFh (0=FLASH, 1=PSRAM) (?)
  025000h Mapping for PSRAM/FLASH (0=32K/LoROM, 1=64K/HiROM)
  035000h Bank 60h-6Fh (0=FLASH, 1=PSRAM) (?)
  045000h Unknown (set when mapping PSRAM as Executable or Streaming Buffer)
  055000h Bank 40h-4Fh (0=PSRAM, 1=FLASH) ;\probably also affects Banks 00h-3Fh
  065000h Bank 50h-5Fh (0=PSRAM, 1=FLASH) ;/and maybe 80h-BFh when BIOS is off?
  075000h Bank 00h-1Fh (0=PSRAM/FLASH, 1=BIOS)
  085000h Bank 80h-9Fh (0=PSRAM/FLASH, 1=BIOS)
  095000h Unknown/Unused (except: used by BS Dragon Quest, set to 00h)
  0A5000h Unknown/Unused (except: used by BS Dragon Quest, set to 80h)
  0B5000h Unknown/Unused (except: used by BS Dragon Quest, set to 80h)
  0C5000h Bank C0h-FFh FLASH Reads? (0=Disable, 1=Enable)
  0D5000h Bank C0h-FFh FLASH Writes (0=Disable, 1=Enable)
  0E5000h Apply Changes to Other MCC Registers (0=Unused/Reserved, 1=Apply)
  0F5000h Unknown/Unused
```

Bits C and D are R/W (the other ones maybe, too).

### SNES Cart Satellaview I/O Ports of MCC Memory Controller

**MCC I/O Ports**

The MCC chip is a simple 16bit register, with the bits scattered across various
memory banks (probably because the MCC chip doesn't have enough pins to decode
lower address bits).

To change a bit: [bit_number*10000h+5000h]=bit_value*80h

```
  005000h Unknown/Unused
  015000h Bank 00h-3Fh and 80h-FFh (0=FLASH, 1=PSRAM) (?)
  025000h Mapping for PSRAM/FLASH (0=32K/LoROM, 1=64K/HiROM)
  035000h Bank 60h-6Fh (0=FLASH, 1=PSRAM) (?)
  045000h Unknown (set when mapping PSRAM as Executable or Streaming Buffer)
  055000h Bank 40h-4Fh (0=PSRAM, 1=FLASH) ;\probably also affects Banks 00h-3Fh
  065000h Bank 50h-5Fh (0=PSRAM, 1=FLASH) ;/and maybe 80h-BFh when BIOS is off?
  075000h Bank 00h-1Fh (0=PSRAM/FLASH, 1=BIOS)
  085000h Bank 80h-9Fh (0=PSRAM/FLASH, 1=BIOS)
  095000h Unknown/Unused
  0A5000h Unknown/Unused
  0B5000h Unknown/Unused
  0C5000h Bank C0h-FFh FLASH Reads? (0=Disable, 1=Enable)
  0D5000h Bank C0h-FFh FLASH Writes (0=Disable, 1=Enable)
  0E5000h Apply Changes to Other MCC Registers (0=Unused/Reserved, 1=Apply)
  0F5000h Unknown/Unused
```

Bits C and D are R/W (the other ones maybe, too, probably except bit E)

Bit 5,6 might also enable FLASH reads,writes in bank 40h-7Dh ?

**Satellaview BIOS Cartridge Memory Map**

```
  00-0F:5000       MCC I/O Ports (Memory Control, BIOS/PSRAM/FLASH Enable)
  10-1F:5000-5FFF  SRAM             (32Kbyte SRAM in 4K-banks)
  xx-3F:6000-7FFF  PSRAM        (Mirror of 8K at PSRAM offset 06000h..07FFFh)
  00-3F:8000-FFFF  PSRAM/FLASH/BIOS in 32K-banks (Slow LoROM mapping)
  40-4F:0000-FFFF  PSRAM/FLASH  (for Executables with Slow HiROM mapping)
  50-5F:0000-FFFF  PSRAM/FLASH  (for Executables with Slow HiROM mapping)
  60-6F:0000-FFFF  FLASH/PSRAM  (for use as Work RAM or Data Files)
  70-77:0000-FFFF  PSRAM
  80-BF:8000-FFFF  PSRAM/FLASH/BIOS  in 32K-banks (Fast LoROM mapping)
  C0-FF:0000-FFFF  PSRAM/FLASH       (FLASH with R/W Access)
```

**Memory**

```
  BIOS ROM  1MByte (LoROM mapping, 20h banks of 32Kbytes each)
  FLASH     1Mbyte   (can be mapped as LoROM, HiROM, or Work Storage)
  PSRAM     512Kbyte (can be mapped as LoROM, HiROM, or Work RAM)
  SRAM      32Kbyte (mapped in eight 4K banks)
```

Note: FLASH is on an external cartridge, size is usually 1MByte (as shown
above).

### SNES Cart Satellaview I/O Receiver Data Streams

The receiver can be programmed to watch (and receive) two different Hardware
Channels simultaneously. In practice, Stream 1 is used only by the BIOS, and
Stream 2 is used only by a few BS FLASH games (Dragon Quest 1, Satella2 1, BS
Fire Emblem Akaneia Senki 1, and maybe some others) (which do use it for
receiving Time Channel Packets).

**2188h/2189h Stream 1 Hardware Channel Number, Lsb/Msb (R/W)**

**218Eh/218Fh Stream 2 Hardware Channel Number, Lsb/Msb (R/W)**

```
  0-15  Hardware Channel Number (16bit)
             XXX reportedly only 14bit !?
```

Values written to these registers should be taken from the Channel Map packet
(or for receiving the Channel Map itself, use fixed value 0124h). Be sure to
reset the Queues after changing the channel number (so you won't receive old
data from old channel).

**218Ah Stream 1 Queue Size (number of received 1+22 byte Units) (R)**

**2190h Stream 2 Queue Size (number of received 1+22 byte Units) (R)**

```
  0-6  Number of received Units contained in the Queue (0..127)
  7    Overrun Error Flag (set when received more than 127 units)
```

Indicates how many frames are in the queues. One doesn't need to process all
frames at once; when reading only a few frames, the Queue Size is decremented
accordingly, and the remaining frames stay in the Queue so they can be
processed at a later time. The decrement occurs either after reading 1 byte
from the Status Queue, or after reading 22 bytes from the Data Queue (anyways,
to keep the queues in sync, one should always read the same amount of 1/22-byte
Units from both Queues, so it doesn't matter when the decrement occurs).

**218Bh Stream 1 Queue 1-byte Status Units (Read=Data, Write=Reset)**

**2191h Stream 2 Queue 1-byte Status Unit(s?) (Read=Data, Write=Reset)**

Contains Header/Data Start/End flags for the received Data Frames, the format
seems to be same as for Port 218Dh/2193h (see there for details) (if it's
really same, then the two Error bits should be also contained in the Status
Queue, though the BIOS doesn't use them in that place).

**218Ch Stream 1 Queue 22-byte Data Units (Read=Data, Write=Reset/Ack)**

**2192h Stream 2 Queue 22-byte? Data Unit(s?) (Read=Data, Write=Reset/Ack)**

Contains the received Data Frames, or in case of Header Frames: The 5/10-byte
Frame Header, followed by the Packet/Fragment Header, followed by the actual
Data.

**218Dh Stream 1 Status Summary (R)**

**2193h Stream 2 Status Summary (R)**

These registers seem to contain a summary of the Status bytes being most
recently removed from the Queue. Ie. status bits are probably getting set by
ORing all values being read from Port 218Ah/2190h. The bits are probably
cleared after reading 218Dh/2193h.

```
  0-1  Unknown/unused
  2-3  Error Flags (probably set on checksum errors or lost data/timeouts)
  4    Packet Start Flag (0=Normal, 1=First Frame of Packet) (with Header)
  5-6  Unknown/unused
  7    Packet End Flag   (0=Normal, 1=Last Frame of Packet)
```

Bit 2-3 are more or less self-explaining: Don't use the queued data, and
discard any already (but still incompletely) received packet fragments. Bit 4,7
are a bit more complicated. See Notes in next chapter for details.

### SNES Cart Satellaview I/O Receiver Data Streams (Notes)

**Resetting the Queues**

Clearing the Status & Data Queues is needed on power-up, after Overrun, or
after changing the Hardware Channel number. The procedure is:

```
  MOV A,01h     ;\
  MOV [218Bh],A ; must be executed in FAST memory (at 3.58MHz) (otherwise the
  NOP           ; the Status Queue may be not in sync with the Data Queue)
  NOP           ; (for Stream 2 do the same with Port 2192h/2193h accordingly,
  NOP           ; though the existing games that do use Stream 2 are including
  NOP           ; several near-excessive timing bugs in that section)
  MOV [218Ch],A ;/
```

Thereafter, Status & Data queue are empty, and the Queue Size register is
00h (both 7bit counter, and Overrun flag cleared).

**Reading the Queues**

```
  N=[218Ah]                                      ;-get queue size
  if N=0 then exit                               ;-exit if no data in queues
  if N.Bit7=1 then reset_queue/abort_packet/exit ;-handle overrun error
  N=max(20,N)                                    ;-limit to max 20 (if desired)
  for i=0 to (N-1), stat[i]=[219Bh], next        ;-read status units
  stat_summary=[219Dh]                           ;-get status summary
  for i=0 to (N*22-1), data[i]=[219Ch], next     ;-read data units
```

**Channel Disable**

After receiving a full packet, the BIOS issues a "MOV [218Ch],00h", this might
acknowledge something, or (more probably) disable the Channel so that no new
data is added to the Queue. The mechanism for re-enabling the channel is
unknown (prossibly resetting the Queue, or writing the Channel register). For
Stream 2, "MOV [2192h],00h" should do the same thing.

**Overrun Notes**

Overrun means that one hasn't processed the queues fast enough. If so, one
should Reset the queues and discard any already-received incomplete packet
fragments. There seems to be no problem if an overrun occurs WHILE reading from
the queue (ie. overrun seems to stop adding data to the queue, rather than
overwriting the old queued data) (of course AFTER reading the queue, one will
need to handle the overrun, ie. discard all newer data).

Note: Stream 1 can queue 127 frames (presumably plus 1 incomplete frame, being
currently received). As far as known, Stream 2 is used only for Time Channels
(with single-frame packets), so it's unknown if Stream 2 is having the same
queue size.

**Packet Start/End Flags**

The status queue values (with start/end bits isolated) would be:

```
  90h               ;packet is 1 frame  (10-byte header + 12-byte data)
  10h,80h           ;packet is 2 frames (10-byte header + 34-byte data)
  10h,00h,80h       ;packet is 3 frames (10-byte header + 56-byte data)
  10h,00h,00h,80h   ;packet is 4 frames (10-byte header + 78-byte data)
```

and so on. For Channel Map, header is only 5-byte, and data is 5 bigger.

Caution: After having received the header (ie. at time when receiving the
data), the BIOS treats the Header-Start Flag in the Status Summary register (!)
as Error-Flag, to some point that makes sense in the Data-phase, but it will
cause an error if a new header frame is received shortly AFTER the Data-phase.
As a workaround, the transmitter should not send new Packet Fragments (on the
same Hardware Channel) for at least 1/60 seconds (preferably longer, say 1/20
seconds) after the end of the last Data Frame.

**Transfer Rate**

The transfer rate is unknown. Aside from the actual transmission speed, the
effective download rate will also depend on how often data is transmitted on a
specific channel (there are probably pauses between packet fragments, and maybe
also between 22-byte frames), this may vary depening on how many other packets
are transmitted, and how much priority is given to individual packets.

The download rate is slow enough for allowing the BIOS to write incoming data
directly to FLASH memory. Moreover, the BIOS Vblank NMI Handler processes only
max twenty 22-byte frames per 60Hz PPU frame. Knowing that, the download rate
must be definetly below 26400 bytes/second (20*22*60).

As far as known, the Satellaview broadcasts replaced former St.GIGA radio
broadcasts, assuming that the radio used uncompressed CD quality (2x16bit at
44.1kHz), and assuming that Satellaview used the same amount of data, the
transfer rate may have been 176400 bytes/second (which could have been divided
to transfer 8 different packets at 22050 bytes/second, for example).

### SNES Cart Satellaview I/O Receiver Control

**2194h POWER (bit0) and ACCESS (bit2-3) LED Control? (R/W)**

```
  0   Usually set  <-- is ZERO by Itoi (maybe POWER LED) (see? 2196h.Bit0)
  1   Usually zero <-- is SET by Itoi                    (see? 2196h.Bit0)
  2-3 Usually both set or both cleared (maybe ACCESS LED) (Bit2 is Access LED)
  4-7 Usually zero
```

Bit2/3 are toggled by software when writing to FLASH memory. Bit0 is usually
set. Might control the POWER and ACCESS LEDs on the Satellaview's Front Panel
(assuming that the LEDs are software controlled). Using other values than
listed above might change the LED color (assuming they are two-color LEDs).

**2195h Unknown/Unused, maybe for EXT Expansion Port (?)**

This register isn't used by the BIOS, nor by any games. Maybe it does allow to
input/output data to the Satellaview's EXT Port.

**2196h Status (only bit1 is tested) (R)**

```
  0    Unknown (reportedly toggles at fast speed when 2194h.Bit0-or-1? is set)
  1    Status (0=Okay, 1=Malfunction)
  2-7  Unknown/unused
```

The BIOS is using only Bit1, that bit is tested shortly after the overall
hardware detection, and also during NMI handling. Probably indicates some kind
of fundamental problem (like low supply voltage, missing EXPAND-Pin connection
in cartridge, or no Satellite Tuner connected).

**2197h Control (only bit7 is modified) (R/W)**

```
  0-6  Unknown/unused (should be left unchanged)
  7    Power Down Mode? (0=Power Down, 1=Operate/Normal) (Soundlink enable?)
```

Bit7 is set by various BIOS functions, and, notably: When [7FD9h/FFD9h].Bit4
(in Satellaview FLASH File Header) is set. Also notably: Bit7 is set/cleared
depending on Town Status Entry[07h].Bit6-7.

**2198h Serial I/O Port 1 (R/W)**

**2199h Serial I/O Port 2 (R/W)**

These ports are basically 3-bit parallel ports, which can be used as three-wire
serial ports (with clock, data.in, data.out lines) (by doing the "serial"
transfer by software). Outgoing data must be written before toggling clock,
incoming data can be read thereafter.

```
  0    Clock (must be manually toggled per data bit)
  1-5  Unknown/unused (should be 0)
  6    Chip Select - For Port 1: 1=Select / For Port 2: 0=Select
  7    Data (Write=Data.Out, Read=Data.in) (data-in is directly poll-able)
```

Bits are transferred MSB first.

Unknown which chips these ports are connected to. One port does most probably
connect to the 64pin MN88821 chip (which should do have a serial port; assuming
that it is a MN88831 variant). The other port <might> connect to the
small 8pin SPR-BSA chip?

Possible purposes might be configuration/calibration, Audio volume control, and
Audio channel selection (assuming that the hardware can decode audio data and
inject it to SNES Expansion Port sound inputs).

**Serial Port 1 (2198h)**

The BIOS contains several functions for sending multi-byte data to, and
receiving 16bit-units from this port. Though the functions seem to be left
unused? (at least, they aren't used in the low-level portion in first 32K of
the BIOS).

Port 1 specific notes: When reading (without sending), the outgoing dummy-bits
should be set to all zero. Chip is selected when Bit6=1. Aside from receiving
data from bit7, that bit is also polled in some cases for sensing if the chip
is ready (0=Busy, 1=Ready).

**Serial Port 2 (2199h)**

Data written to this port consists of simple 2-byte pairs (index-byte,
data-byte), apparently to configure some 8bit registers. Used values are:

```
  Reg[0] = 88h (or 00h when Power-Down?) (soundlink on/off?)
  Reg[1] = 80h
  Reg[2] = 04h
  Reg[3] = 00h
  Reg[4] = 08h
  Reg[5] = 00h
  Reg[6] = 70h
  Reg[7] = Not used
  Reg[8] = 00h
  Reg[9..FF] = Not used
```

There are also BIOS functions for reading 1-byte or 3-bytes from this Port, but
they seem to be left unused (but, BS Dragon Quest, and Itoi are doing 24bit
reads via direct I/O, whereas Itoi wants the 1st bit to be 0=ready/okay).

Port 2 specific notes: When reading (without sending), the outgoing dummy-bits
should be set to all ones. Chip(-writing) is selected when Bit6=0.

### SNES Cart Satellaview I/O FLASH Detection (Type 1,2,3,4)

The Satellaview FLASH cartridges contain slightly customized "standard" FLASH
chips; with a custom Nintendo-specific Chip Detection sequence:

**Detection Sequence**

```
  [C00000h]=38h, [C00000h]=D0h                  ;request chip info part 1
  delay (push/pop A, three times each)          ;delay
  [C00000h]=71h                                 ;enter status mode
  repeat, X=[C00002h], until (X.bit7=1)         ;wait until ready
  [C00000h]=72h, [C00000h]=75h                  ;request chip info part 2
  FOR i=0 to 9, info[i]=BYTE[C0FF00h+i*2], NEXT ;read chip info (10 bytes)
  [C00000h]=FFh   ;somewhat bugged, see below   ;terminate status mode
```

Note: Nintendo Power flashcarts are also using very similar nonstandard FLASH
commands as above (there, for reading hidden mapping info, instead of for chip
detection).

BUG: For Type 2 chips, one <should> use "[C05555h]=AAh, [C02AAAh]=55h,
[C05555h]=F0h" instead of "[C00000h]=FFh" (the BIOS is actually <trying>
to do that, but it's doing it before having deciphered the Type bits).

**Detection Values**

```
  info[0] - ID1 (Must be "M" aka 4Dh)
  info[1] - ID2 (Must be "P" aka 50h)
  info[2] - Flags (Must be bit7=0 and bit0=0) (other bits unknown)
  info[3] - Device Info (upper 4bit=Type, lower 4bit=Size)
  info[4..9] - Unknown/Unused (BIOS copies them to RAM, but doesn't use them)
```

Type must be 01h..04h for Type 1-4 accordingly. Size must be 07h..0Ch for
128Kbyte, 256Kbyte, 512Kbyte, 1Mbyte, 2Mbyte, 4Mbyte accordingly (ie. 1 SHL N
Kbytes).

**Rejected Values**

Wrong ID1/ID2 or wrong Flag Bit7/Bit0 are rejected. Type 00h or 05h..0Fh are
rejected. Size 00h..05h is rejected. Size 06h would be a half 128Kbyte block,
which is rounded-down to 0 blocks by the BIOS. Size 0Dh would exceed the 32bit
block allocation flags in header entry 7FD0h/FFD0h. Size 0Fh would additionally
exceed the 8bit size number.

**Special Cases**

If no FLASH cartridge is inserted, then the detection does probably rely on
open bus values, ie. "MOV A,[C00002h]" probably needs to return C0h (the last
opcode byte) which has bit7=1, otherwise the detection wait-loop would hang
forever.

There are reportedly some "write-protected" cartridges. Unknown what that
means, ROM-cartridges, or FLASH-cartridges with some (or ALL) sectors being
write-protected. And unknown what detection values they do return.

**FLASH Base Address**

The Satellaview BIOS always uses C00000h as Base Address when writing commands
to FLASH, the MCC chip could be programmed to mirror FLASH to other locations
(although unknown if they are write-able, if so, commands could be also written
to that mirrors).

Game Cartridges with built-in FLASH cartridge slot may map FLASH to other
locations than C00000h, details on that games are unknown. The game carts don't
include MCC chips, but other mapping hardware: In at least some of them the
mapping seems to be controlled by a SA-1 chip (an external 10.74MHz 65C816 CPU
with on-chip I/O ports, including memory-mapping facilities), the SA-1 doesn't
seem to have FLASH-specific mapping registers, so the FLASH might be mapped as
secondary ROM-chip. There may be also other games without SA-1, using different
FLASH mapping mechanism(s)? For some details, see:

**General Notes**

FLASH erase sets all bytes to FFh. FLASH writes can only change bits from 1 to
0. Thus, one must normally erase before writing (exceptions are, for example,
clearing the "Limited-Start" bits in Satellaview file header). Type 2 can write
128 bytes at once (when writing less bytes, the other bytes in that area are
left unchanged). The status/detection values may be mirrored to various
addresses; the normal FLASH memory may be unavailable during
write/erase/detection (ie. don't try to access FLASH memory, or even to execute
program code in it, during that operations).

### SNES Cart Satellaview I/O FLASH Access (Type 1,3,4)

The Type 1,3,4 protocol is somewhat compatible to Sharp LH28F032SU/LH28F320SK
chips (which has also a same/similar 52pin package). Concerning the commands
used by the BIOS, Type 1,3,4 seems to be exactly the same - except that Type 3
doesn't support the Erase-Entire chip command.

**Erase Entire Chip (Type 1 and 4 only) (not supported by Type 3)**

```
  [C00000h]=50h                                 ;clear status register
  [C00000h]=71h                                 ;enter status mode
  repeat, X=[C00004h], until (X.bit3=0)         ;wait until VPP voltage okay
  [C00000h]=A7h  ;"erase all unlocked pages"?   ;select erase entire-chip mode
  [C00000h]=D0h                                 ;start erase
  [C00000h]=71h                                 ;enter status mode
  repeat, X=[C00004h], until (X.bit7=1)         ;wait until ready
  if (X.bit5=1) then set erase error flag       ;check if erase error
  [C00000h]=FFh                                 ;terminate status mode
```

**Unknown Command**

Same as Erase Entire (see above), but using 97h instead of A7h, and implemented
ONLY for Type 1 and 4, that is: NOT supported (nor simulated by other commands)
for neither Type 2 nor Type 3. Maybe A7h erases only unlocked pages, and 97h
tries to erase all pages (and fails if some are locked?).

**Erase 64KByte Sector**

```
  [C00000h]=50h                                 ;clear status register
  [C00000h]=20h                                 ;select erase sector mode
  [nn0000h]=D0h                                 ;start erase 64K bank nn
  [C00000h]=70h                                 ;enter status mode
  repeat, X=[C00000h], until (X.bit7=1)         ;wait until ready
 ;; if (X.bit5=1) then set erase error flag       ;check if erase error
  [C00000h]=FFh                                 ;terminate status mode
```

**Write Data**

```
  FOR i=first to last
    [C00000h]=10h                               ;write byte command
    [nnnnnnh+i]=data[i]                         ;write one data byte
    [C00000h]=70h                               ;enter status mode
    repeat, X=[C00000h], until (X.bit7=1)       ;wait until ready
  NEXT i
  [C00000h]=70h                                 ;enter status mode
  repeat, X=[C00000h], until (X.bit7=1)         ;hmmm, wait again
  if (X.bit4=1) then set write error flag       ;check if write error
  [C00000h]=FFh                                 ;terminate status mode
```

**Enter Erase-Status Mode**

```
  [C00000h]=71h                                 ;enter status mode
  X=[C00004h]                                   ;read status byte
  IF (X.bit7=0) THEN busy
  IF (X.bit3=1) THEN not-yet-ready-to-erase (VPP voltage low)
  IF (X.bit7=1) AND (X.bit5=0) THEN ready/okay
  IF (X.bit7=1) AND (X.bit5=1) THEN erase error ?
```

**Enter Other-Status Mode**

```
  [C00000h]=70h                                 ;enter status mode
```

**Terminate Command**

```
  [C00000h]=FFh                                 ;terminate
```

Used to leave status or chip-detection mode.

BUGs: On Type 3 chips, the BIOS tries to simulate the "Erase-Entire" command by
issuing multiple "Erase-Sector" commands, the bug there is that it tests bit4
of the flash_size in 128Kbyte block units (rather than bit4 of the
flash_status) as erase-error flag (see 80BED2h); in practice, that means that
the erase-entire will always fail on 2MByte chips (and always pass okay on all
other chips); whereas, erase-entire is used when downloading files (except for
small files that can be downloaded or relocated to PSRAM).

### SNES Cart Satellaview I/O FLASH Access (Type 2)

Type 2 protocol is completely different as for Type 1,3,4 (aside from the Chip
Detection sequence, which is same for all types).

**Erase Entire Chip**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h   ;unlock erase
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=10h   ;do erase entire chip
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=70h   ;enter status mode
  repeat, X=[C05555h], until (X.bit7=1)         ;wait until ready
  if (X.bit5=1) then set erase error flag       ;check if erase error
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate status mode
```

**Erase 64KByte Sector**

```
  [C00000h]=50h                                 ;huh? (maybe a BIOS bug)
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h   ;unlock erase
  [C05555h]=AAh, [C02AAAh]=55h, [nn0000h]=30h   ;do erase bank nn
  repeat, X=[C05555h], until (X.bit7=1)         ;wait until ready
  if (X.bit5=1) then set erase error flag       ;check if erase error
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate status mode
```

**Write 1..128 Bytes (within a 128-byte boundary)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=A0h   ;enter write mode
  FOR i=first to last, [nnnnnn+i]=data[i]       ;write 1..128 byte(s)
  [nnnnnn+last]=DATA[last]   ;write LAST AGAIN  ;start write operation
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=70h   ;enter status mode
  repeat, X=[C05555h], until (X.bit7=1)         ;wait until ready
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate status mode
```

**Enter Status Mode**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=70h   ;enter status mode
  X=[C05555h]                                   ;read status byte
  IF (X.bit7=0) THEN busy
  IF (X.bit7=1) AND (X.bit5=0) THEN ready/okay
  IF (X.bit7=1) AND (X.bit5=1) THEN ready/erase error ?
```

**Terminate Command**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate
```

Used to leave status or chip-detection mode.

**Bugged Commands**

The are some cases (BIOS addresses 80BF88h, 80DBA4h, 80E0D5h) where Type 2
programming is mixed up with some non-Type-2 commands (setting [C00000h]=50h
and [C00000h]=FFh), these commands are probably ignored by chip (or switching
it back default mode).

### SNES Cart Satellaview Packet Headers and Frames

**Packet Fragment Format (10-byte header)**

```
  00h 1    Transmission ID (in upper 4bit) (must stay same for all fragments)
  01h 1    Current Fragment Number (in lower 7bit)
  02h 3    Fragment Size (N) (big-endian) (excluding first 5 bytes at [00..04])
  05h 1    Fixed, Must be 01h
  06h 1    Total Number of Fragments (00h=Infinite Streaming?)
  07h 3    Target Offset (big-endian) (location of fragment within whole file)
  0Ah N-5  Data Body (first 12 bytes located directly in header frame)
  ... ...  Unused/Padding (until begin of next 22-byte Frame)
```

This is the normal format used by all packets (except Channel Map packet).

**Channel Map Packet Format (5-byte header)**

```
  00h 1    Unknown/unused (would be 4bit Transmission ID for normal packets)
  01h 1    Unknown/unused (would be 7bit Fragment Number for normal packets)
  02h 3    Packet Size (N) (big-endian) (excluding first 5 bytes at [00..04])
  05h N    Data Body (first 17 bytes located directly in header frame)
  ... ...  Unused/Padding (until begin of next 22-byte Frame)
```

**Frames (22-bytes)**

Packets are divided into one or more 22-byte frames.

For each frame, a 1-byte status info can be read from Port 218Bh/2191h, the
status contains error flags (indicating bad checksums or lost-frames or so),
and header flags (indicating begin/end of header/data or so).

The 22-byte frame data can be read from Port 218Ch/2192h. If it is header
frame, then its first 10 (or 5) bytes contain the header (as described above),
and the remaining 12 (or 17) bytes are data. If it is a data frame, then all 22
bytes are plain data.

**Fragmented Packets**

A packet can consist of 1..128 fragments. The packet transmission is repeated
several times (say, for one hour). If it is consisting of several fragments,
one can start downloading anywhere, eg. with the middle fragment, and one can
keep downloading even if some fragments had transmission errors (in both cases,
one must download the missing fragments in the next pass(es) when the
transmission is repeated).

Software should maintain a list of fragments that are already received (if a
fragment is already received: just remove it from the queue without writing to
target area; both for saving CPU load, and for avoiding to destroy previously
received packets in case of transmission errors).

At some time (say, after an hour), transmission of the packet will end, and a
different packet may be transferred on the same hardware channel. Verify the
4bit Transmission ID to avoid mixing up fragments from the old (desired) file
with the new file. Ideally, that ID <should> be stored in the Channel Map
or Directory (this may actually be so), however, the Satellaview BIOS simply
takes the ID from the first received Fragment, and then compares it against
following Fragments (if the ID changed shortly after checking the Directory,
and before receiving the first fragment, then the BIOS will download a "wrong"
file).

**Fragment Size Bug**

The Satellaview BIOS supports only 16bit fragment sizes, it does try to handle
24bit sizes, but if the fragment size exceeds 65535 then it does receive only
the first few bytes, and then treats the whole fragment as "fully" received.

**Text Strings**

Text strings (file/folder names and descriptions) can contain ASCII (and
presumably JIS and SHIFT-JIS), and following specials:

```
  00h        End of Line (or return from a "\s" Sub-String)
  0Dh        Carriage Return Line Feed (in descriptions)
  20h..7Eh   ASCII 6pix characters (unlike SHIFT-JIS 12pix ones)
  80h..9Fh   Prefixes for double-byte characters (SHIFT-JIS)
  A0h..DFh   Japanese single-byte characters (JIS or so)
  E0h..EAh   Prefixes for double-byte characters (SHIFT-JIS)
  F0h        Prefix for Symbols (40h..51h:Music-Note,Heart,Dots,Faces,MaKenji)
  "\\"           Yen symbol (unlike ASCII, not a backslash)
  "\b0".."\b3"   Insert Username/Money/Gender/NumItems (12pix SHIFT-JIS)
  "\c0".."\c5"   Changes color or palette or so
  "\d#",p24bit   Insert 16bit Decimal at [p24bit] using 6pix-font
  "\D#",p24bit   Insert 16bit Decimal at [p24bit] using 12pix-font
  "\du#",v24bit  Insert 16bit Decimal Interpreter-Variable using 6pix-font
  "\Du#",v24bit  Insert 16bit Decimal Interpreter-Variable using 12pix-font
                    # = 00     Variable width (no leading spaces/zeroes)
                    # = 1..6   Width 1..6 chars (with leading spaces)
                    # = 01..06 Width 1..6 chars (with leading zeroes)
  "\s",ptr24bit  Insert Sub-string (don't nest with further "\s,\d,\D")
  "\g",ptr24bit  Insert Custom Graphics/Symbol (ptr to xsiz,ysiz,bitmap)
  "\i"           Carriage Return (set x=0, keep y=unchanged) (not so useful)
  "\m0".."\m3"   Flags (bit0=ForceHorizontal16pixGrid, bit1=DonNotUpdateBg3Yet)
  "\n"           Carriage Return Line Feed (same as 0Dh)
  "\p00".."\p07" Palette
  "\w00".."\w99" Character Delay in Frames (00=None)
  "\x00".."\xNN" Set Xloc
  "\y00".."\yNN" Set Yloc
```

Note: "\m","\p","\w","\x","\y" are slightly bugged (causing stack overflows
when using them too often within a single string; the text output thread quits
at the string-end, which somewhat 'fixes' the stack-problem).

CRLF can be used in Item Activation Messages, Folder or Download-File
Descriptions.

Multi-line messages are wrapped to the next line when reaching the end of a
line (the wrapping can occur anywhere within words, to avoid that effect one
must manually insert CRLF's (0Dh) at suitable locations). Some message boxes
are clipped to the visible number of lines, other messages boxes prompt the
user to push Button-A to read further lines.

Caution: CRLF will hang in Item-Descriptions (they do work in item shops, but
will hang in the Inventory menu; the only way to implement longer descriptions
here is to space-pad them so that the wrapping occurs at a suitable location).

### SNES Cart Satellaview Channels and Channel Map

**Channels**

Transmission is organized in "channels". Each can Channel transmits a single
Packet (which contains a File, or other special information like in Directory
and Time packets). There can be (theoretically) up to 4 billion logical
"Software Channels", but only 65536 physical "Hardware Channels". The latter
ones are those being currently transmitted, and which can be received by
programming the channel number into Port 2188h/218Eh.

**Channel Map Packet (Hardware Channel 0124h)**

Unlike normal packets (with 10-byte headers), this packet is preceeded by a
5-byte header.

Loaded to 7E9BECh. Unspecified size (size should be max 1485 bytes or less,
otherwise it'd overlap the Welcome Message at 7EA1B9h) (with that size limit,
the map can contain max 113 channels) (but, caution: Itoi works only with max
1007 bytes (1024-byte buffer, cropped to N*22-bytes, minus 5-byte packet
header)).

```
  00h 2    ID 53h,46h ("SF")
  02h 4    Unknown/unused
  06h 1    Number of entries (must be at least 1)
  07h 1    Checksum (above 7 bytes at [00..06] added together)
  08h ..   Entries (each one is 3+N*13 bytes)
```

Each entry is: (Packet Groups)

```
  00h 2    Software Channel Number (first 2 bytes, of total 4 bytes)
  02h 1    Number of sub-entries (N) (must be at least 1)
  03h N*13 Sub-entries (each one is 13 bytes)
```

Each sub-entry is: (Separate Packets)

```
  00h 1    Unknown/unused
  01h 2    Software Channel Number (last 2 bytes, of total 4 bytes)
  03h 5    Unknown/unused
  08h 2    Fragment Interval (in seconds) (big-endian) (for use as timeout)
  0Ah 1    Type/Target (lower 4bit indicate transfer method or so)
            Bit0-1: Autostart after Download (0=No, 1=Optional, 2=Yes, 3=Crash)
            Bit2-3: Target (0=WRAM, 1=PSRAM, 2=EntireFLASH, 3=FreeFLASH)
            Bit4-7: Unknown/Unused
  0Bh 2    Hardware Channel Number (2 bytes) (for Port 2188h/218Eh)
```

The transmission timeout for the Channel Map itself is 7 seconds.

**Hardware Channels (2-byte / 16bit) (XXX or only 14bit?!)**

```
  0121h     Used for hardware-connection test (received data is ignored)
  0124h     Channel Map
  AAEEh     Dummy number (often used to indicate an absent Time Channel)
  NNNNh     Other Hardware Channels (as listed in Channel Map)
  [7FFFF7h] Incoming Time Channel value for some games (from separate loader?)
```

**Software Channels (4-byte pairs / 32bit)**

```
  1.1.0.4    Welcome Message (100 bytes)
  1.1.0.5    Town Status (256 bytes)
  1.1.0.6    Directory (16Kbytes)
  1.1.0.7    SNES Patch (16Kbytes)
  1.1.0.8    Time Channel (used by BS Satella2 1, BS Fire Emblem, and Itoi)
  1.2.0.48   Time Channel (used by Dragon Quest 1, BS Zelda no Densetsu Remix)
  ?.?.?.?    Time Channel (for BS Zelda - Kodai no Sekiban Dai 3 Hanashi)
  1.2.129.0  Special Channel used by Derby Stallion 96 <-- on roof of building
  1.2.129.16 Special Channel used by Derby Stallion 96 <-- 6th main menu option
  1.2.130.N  Special Channel(s) used by Itoi Shigesato no Bass Tsuri No. 1
  N.N.N.N    Other Software Channels (as listed in Directory)
  N.N.0.0    None (for directory entries that have no File or Include File)
```

**Endianess of Numbers in Satellite Packets**

In the satellite packets, all 16bit/24bit values (such like length or address
offsets) are in big-endian format (opposite of the SNES CPUs byte-order). For
the Hardware/Software Channel Numbers it's hard to say if they are meant to be
big-endian, little-endian, or if they are meant to be simple byte-strings
without endianess (see below for their ordering in practice).

**Endianess of Hardware Channels**

The endiness of the Hardware Channel numbers in Channel Map is same as in Port
2188h/218Eh. The endianess of the fixed values (0121h and 0124h) is also same
as Port 2188h/218Eh. Ie. one can use that values without needing to swap LSBs
and MSBs.

**Endianess of Software Channels**

The fixed 4-byte pairs are using same byte-order as how they are ordered in
Channel Map (for example, 1.2.0.48 means that 48 (30h) is at highest address).
So far it's simple. The (slightly) confusing part is that SNES software usually
encodes them as 2-word pairs (since the SNES uses a little-endian CPU, the
above example values would be 0201h.3000h).

### SNES Cart Satellaview Town Status Packet

**Town Status (Software Channel 1.1.0.5)**

Loaded to 7EA31Dh, then copied to 7EA21Dh. Size (max) 256 bytes.

Uses a normal 10-byte fragment header, but with "4bit Transmission ID" and
"7bit Fragment Number" ignored, but still does use "Target Offset" for
fragment(s)?

```
  00h   1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h   1   Town Status ID (packet must be processed only if this ID changes)
  02h   1   Directory ID   (compared to Directory ID in Directory packet)
  03h   4   Unknown/unused
  07h   1   APU Sound Effects/Music & BSX Receiver Power-Down
             Bit0-3 Unknown/unused
             Bit4-5 APU (0=Mute, 1=Effects, 2=Effects/MusicA, 3=Effects/MusicB)
             Bit6   BSX (0=Normal, 1=Power-down with Port 2199h Reg[0]=88h)
             Bit7   BSX (0=Normal, 1=Power-down with Port 2199h Reg[0]=00h)
             (Or, maybe, the "Power-down" stuff enables satellite radio,
             being injected to audio-inputs on expansion port...?)
  08h   1   Unknown/unused
  09h   8   People Present Flags (Bit0-63) (max 5)        (LITTLE-ENDIAN)
  11h   2   Fountain Replacement & Season Flags (Bit0-15) (LITTLE-ENDIAN)
  13h   4   Unknown/unused
  17h   1   Number of File IDs (X) (may be 00h=none) (max=E8h)
  18h   X   File IDs (one byte each) (compared against File ID in Directory)
```

This packet should be (re-)downloaded frequently. The File IDs indicate which
Directory entries are valid (whereas, it seems to be possible to share the same
ID for ALL files), the Directory ID indicates if the Directory itself is still
valid.

**Fountain/Replacement and Season**

The animated Fountain (near beach stairs) has only decorative purposes (unlike
buildings/people that can contain folders). Optionally, the fountain can be
replaced by other (non-animated) decorative elements via Fountain Replacement
Flags in the Town Status packet: Bit0-11 are selecting element 1-12 (if more
than one bit is set, then the lowest bit is used) (if no bits are set, then the
fountain is used).

```
  None  00h Default Fountain (default when no bits set) (animated)
  Bit0  01h Jan     Altar with Apple or so
  Bit1  02h Feb     Red Roses arranged as a Heart
  Bit2  03h Mar     Mohican-Samurai & Batman-Joker
  Bit3  04h Apr     Pink Tree
  Bit4  05h May     Origami with Fish-flag
  Bit5  06h Jun     Decorative Mushrooms
  Bit6  07h Jul     Palmtree Christmas with Plastic-Blowjob-Ghost?
  Bit7  08h Aug     Melons & Sunshade
  Bit8  09h Sep     White Rabbit with Joss Sticks & Billiard Balls
  Bit9  0Ah Oct     National Boule-Basketball
  Bit10 0Bh Nov     Red Hemp Leaf (cannabis/autum)
  Bit11 0Ch Dec     Christmas Tree (with special Gimmick upon accessing it)
```

As shown above, the 12 replacements are eventually associated with months
(Christmas in December makes sense... and Fish in May dunno why).

Bit12-15 of the above flags are selecting Season:

```
  None  00h Default (when no bits set)
  Bit12 01h Spring  (pale green grass) (seems to be same as default)
  Bit13 02h Summery (poppy colors with high contrast)
  Bit14 03h Autumn  (yellow/brown grass)
  Bit15 04h Winter  (snow covered)
```

As for the Fountain, default is used when no bits set, and lowest bit is used
if more than one bit is set.

### SNES Cart Satellaview Directory Packet

**Directory (Software Channel 1.1.0.6)**

Loaded to 7FC000h and then copied to 7EC000h. Size (max) 16Kbytes.

```
  00h   1   Directory ID (compared to Directory ID in Town Status packet)
  01h   1   Number of Folders (Buildings, People, or hidden folders)
  02h   3   Unknown/unused
  05h   ..  Folders (and File) entries (see below)       ;\together
  ..    1   Number of Expansion Data Entries (00h=none)  ; max 3FFBh bytes
  ..    ..  Expansion Data Entries (see next chapter)    ;/
```

**Folder Entry Format**

```
  00h   1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h   1   Number of File Entries (if zero: buildings are usually closed)
  02h   15h Folder Name (max 20 chars, terminated by 00h) (shown in building)
  17h   1   Length of Folder Message (X) (in bytes, including ending 00h)
  18h   X   Folder Message/Description (terminated by 00h)
  18h+X 1   More Flags (Folder Type)
             Bit0   : Folder Content (0=Download/Files, 1=Shop/Items)
             Bit1-3 : Folder Purpose (0=Building, 1=Person, 2=Include-Files)
               000b = Indoors (Building ID at [19h+X])     (Bit3-1=000b)   (0)
               x01b = Outdoors (Person ID at [19h+X])      (Bit2-1=01b)  (1,5)
               x1xb = Folder contains hidden Include Files (Bit2=1b) (2,3,6,7)
               100b = Unknown/unused (may be useable for "Files at Home"?) (4)
             Bit4-7 : Unknown/unused
  19h+X 1   Folder 6bit ID (eg. for Building:01h=News, for People:01h=Hiroshi)
  1Ah+X 1   Unknown/Unused
  1Bh+X 1   Unknown/Unused
  1Ch+X 1   Clerk/Avatar (00h..10h) (eg. 0Eh=Robot, 10h=BS-X) (11h..FFh=Crash)
  1Dh+X 1   Unknown/Unused
  1Eh+X 1   Unknown/Unused
  1Fh+X 1   Unknown/Unused
  20h+X ..  File/Item Entries (each one is 32h+X bytes; for items: fixed X=79h)
```

**File/Item Entry Format**

For Both Files and Items:

```
  00h   1   File ID (compared to File IDs in Town Status Packet)
  01h   1   Flag (bit0=used by "Town Status" check (1=Not Available))
  02h   15h File Name (max 20 chars, terminated by 00h) (shown in building)
```

For Files: (in File Folders)

```
  17h   1   Length of File Message (X) (in bytes, including ending 00h)
  18h   X   File Message/Description (terminated by 00h)
```

For Items: (in Item Folders)

```
  17h   1   Length of Item Description+Activation+Price+Flag (X) (fixed=79h)
  18h   25h Item Description (max 36 chars, plus ending 00h)
  3Dh   47h Item Activation Message (min 1, max 70 chars, plus ending 00h)
  84h   12  Item Price (12-Digit ASCII String, eg. "000000001200" for 1200G)
  90h   1   Item Drop/Keep Flag (00h=Drop after Activation, 01h=Keep Item)
```

For Both Files and Items:

```
  18h+X 4   Software Channel Number of Current File (for Items: N.N.0.0=None)
  1Ch+X 3   Big-Endian Filesize
  1Fh+X 3   Unknown/unused (except, first 2 bytes used by Derby Stallion 96)
  22h+X 1   Flags
              Bit0: used by "Town Status" check (1=Not Available)
              Bit1: Unknown/unused
              Bit2: Building Only (0=Also available at Home, 1=Building only)
              Bit3: Low Download Accuracy / Streaming or so
               0=High-Download-Accuracy (for programs or other important data)
               1=Low-Download-Accuracy (for audio/video data streaming)
              Bit4: Unused (except, must be 0 for Derby Stallion 96 files)
              Bit5-7: Unknown/unused
  23h+X 1   Unknown/unused
  24h+X 1   Flags/Target (seems to be same/similar as in Channel Map)
             Bit2-3:
              0=Download to WRAM (not really implemented, will crash badly)
              1=Download to PSRAM (without saving in FLASH)
              2=Download to Continous FLASH banks (erases entire chip!)
              3=Download to FREE-FLASH banks (relocate to PSRAM upon execution)
  25h+X 2   Unknown/unused
  27h+X 1   Date (Bit7-4=Month, Bit3-0=0)             ;\copied to Satellaview
  28h+X 1   Date (Bit7-3=Day, Bit2=0, Bit1-0=Unknown) ;/FLASH File Header FFD6h
  29h+X 1   Timeslot (Bit7-3=StartHours.Bit4-0, Bit2-0=Start Minutes.Bit5-3)
  2Ah+X 1   Timeslot (Bit4-0=EndHours.Bit4-0,   Bit7-5=Start Minutes.Bit2-0)
  2Bh+X 1   Timeslot (Bit7-2=EndMinutes.Bit5-0, Bit1-0=Unused)
  2Ch+X 4   Software Channel Number for additional Include File (N.N.0.0=None)
  30h+X 2   Unknown/unused
```

The directory may contain files that aren't currently transmitted; check the
Town Status for list of currently available File IDs. Also check the Directory
ID in the Town Status, if it doesn't match up with the ID of the Directory
packet, then one must download the Directory again (otherwise one needs to
download it only once after power-up).

**Include Files**

Each File in the directory has an Include File entry. Before downloading a
file, the BIOS checks if the Include File's Software Channel Number is listed
in the Directory (in any folder(s) that are marked as containing Include
Files). If it isn't listed (or if it's N.N.0.0), then there is no include file.
If it is listed, then the BIOS does first download the include file, and
thereafter download the original file. Whereas, the include file itself may
also have an include file, and so on (the download order starts with the LAST
include file, and ends with the original file).

There are some incomplete dumps of some games, which have 1Mbyte FLASH dumped,
but do require additional data in WRAM/PSRAM:

```
  BS Dragon Quest (copy of channel_map in PSRAM, episode_number in WRAM)
  BS Zelda no Densetsu Kodai no Sekiban Dai 3 Hanashi (hw_channel in WRAM)
```

The missing data was probably transferred in form of Include files.

**Streaming Files**

There seems to be some streaming support:

Files flagged as FILE[22h+X].Bit3=1 are repeatedly downloaded-and-executed
again and again (possibly intended to produce movies/slideshows) (details: max
256K are loaded to upper half of PSRAM, and, if successfully received: copied
to lower 256K of PSRAM and executed in that place; whereas, the execution
starts once when receiving the next streaming block, that is: with a different
4bit transmission ID in the 10-byte packet header).

Moreover, Packets marked as having 00h fragments, are treated as having
infinite fragments (this would allow to overwrite received data by newer data;
though none of the higher-level BIOS functions seems to be using that feature).

**Files Available at Home**

Some files can be downloaded at the "Home" building (via the 3rd of the 4
options in that building), these "Home" files may be located in any folders,
namely, including following folders:

```
  FOLDER[00h].Bit0=Don't care (folder may be marked as hidden)
  FOLDER[18h+X]=Don't care    (folder may be also used as building/person/etc.)
```

For downloading them at "Home", the files must be defined as so:

```
  FILE[1Ah+X]<>0000h   (software channel isn't N.N.0.0)
  FILE[22h+X].Bit2=0   (flagged as available at home)
  FILE[18h]=Don't care (file description isn't shown at home)
```

BUG: If there are more than 32 of such "Home" files, then the BIOS tries to
skip the additional files, but destroys the stack alongside that attempt.

Note: Unlike other downloads, Home ones are always having a wooden-plank
background, and are done without transmission Interval timeouts. And, Include
Files are ignored. And, Autostart is disabled (ie. downloads to PSRAM are
totally useless, downloads to FLASH can/must be manually started).

### SNES Cart Satellaview Expansion Data (at end of Directory Packets)

**Directory (Software Channel 1.1.0.6)**

Loaded to 7FC000h and then copied to 7EC000h. Size (max) 16Kbytes.

```
  00h   1   Directory ID (compared to Directory ID in Town Status packet)
  01h   1   Number of Folders (Buildings, People, or hidden folders)
  02h   3   Unknown/unused
  05h   ..  Folders (and File) entries (see previous chapter) ;\together
  ..    1   Number of Expansion Data Entries (00h=none)       ; max 3FFBh
  ..    ..  Expansion Data Entries                            ;/bytes
```

**Expansion Data Entry Format (after folder/file area)**

```
  00h   1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h   1   Unknown/unused
  02h   2   Length (N) (16bit) (this one is BIG-ENDIAN)
  04h   N   Expansion Chunk(s) (all values in chunks are LITTLE-ENDIAN)
```

For for some reason, there may be more than one of these entries, but the BIOS
does use only the first entry with [00h].Bit0=0=Valid (any further entries are
simply ignored).

**Chunk 00h (End)**

```
  00h        1   Chunk ID (00h)
```

Ends (any further following chunks are ignored). Chunk 00h should be probably
always attached at the end of the chunk list (although it can be omitted in
some cases, eg. after Chunk 02h).

**Chunk 01h (Custom Building)**

All values in this chunk are LITTLE-ENDIAN.

```
  00h        1   Chunk ID (01h)
  01h        2   Chunk Length (73h+L1..L5) (LITTLE-ENDIAN)
  03h        11h Message Box Headline (max 16 chars, terminated by 00h)      ;\
  14h        20h BG Palette 5 (copied to WRAM:7E20A0h/CGRAM:50h)  (16 words) ;
  34h        38h BG Data (copied to WRAM:7E4C8Ch/BG1 Map[06h,0Dh])(4x7 words);/
  6Ch        2   Length of BG Animation Data (L1)                            ;\
  6Eh        L1  BG Animation Data (for Custom Building) (see below)         ;/
  6Eh+L1     2   Tile Length (L2) (MUST be nonzero) (zero would be 64Kbytes) ;\
                 BUG: Below Data may not start at WRAM addr 7Exx00h/7Exx01h  ;
                 (if so, data is accidently read from address-100h)          ;
  70h+L1     L2  Tile Data (DMAed to VRAM Word Addr 4900h) (byte addr 9200h) ;/
  70h+L1+L2  2   Length (L3) (should be even, data is copied in 16bit units) ;\
  72h+L1+L2  L3  Cell to Tile Xlat (copied to 7E4080h, BUG:copies L3+2 bytes);/
  72h+L1..L3 2   Length (L4) (MUST be even, MUST be nonzero, max 30h)        ;\
  74h+L1..L3 L4  Cell Solid/Priority List (copied to 7E45D0h,copies L4 bytes);/
  74h+L1..L4 2   Unknown/unused, probably Length (L5)                        ;\
  76h+L1..L4 L5  Door Location(s) (byte-pairs: xloc,yloc, terminated FFh,FFh);/
                 Door Locations work only if BG1 cells also have bit15 set!
                 Bit15 must be set IN FRONT of the door, this is effectively
                 reducing the building size from 4x7 to 4x6 cells.
                 Note: Animated BG cells are FORCEFULLY having bit15 cleared!
```

This chunk may be followed by Chunk 02h. If so, it processes Chunk 02h (and
ends thereafter). Otherwise it ends immediately (ie. when the following Chunk
has ID=00h) (or also on any "garbage" ID other than 02h).

**Chunk 02h (Custom Persons)**

```
  00h        1   Chunk ID (02h)
  01h        2   Chunk Length (N) (LITTLE-ENDIAN) (N may be even,odd,zero)
  03h        N   Data (copied to 7F0000h) (N bytes) (max 0A00h bytes or so)
                  00h 4  7F0000h  Person 2Ch - Token Interpreter Entrypoint
                  04h 4  7F0004h  Person 2Dh - Token Interpreter Entrypoint
                  08h .. 7F0008h  General Purpose (Further Tokens and Data)
```

Copies the Data, and ends (any further following chunks are ignored). If Person
2Ch/2Dh are enabled in the Town Status packet, then the person thread(s) are
created with above entrypoint(s). The threads may then install whatever OBJs
(for examples, see the table at 99DAECh, which contains initial X/Y-coordinates
and Entrypoints for Person 00h..3Fh).

**Chunk 03h..FFh (Ignored/Reserved for future)**

```
  00h        1   Chunk ID (03h..FFh)
  01h        2   Chunk Length (N) (LITTLE-ENDIAN)
  03h        N   Data (ignored/skipped)
```

Skips the data, and then goes on processing the following chunk.

**BG Animation Data (within Chunk 01h) (for Custom Building)**

```
  00h     2   Base.xloc (0..47) (FFFFh=No Animation Data)
  02h     2   Base.yloc (0..47)
  04h     X*4 Group(s) of 2 words (offset_to_frame_data,duration_in_60hz_units)
  04h+X*4 2   End (FFFEh=Loop/Repeat animation, FFFFh=Bugged/One-shot animat.)
  06h+X*4 ..  BG Animation Frame Data Block(s) (see below)
  ..      0-2 Padding (to avoid 100h-byte boundary BUG in L2)
```

Xloc/Yloc should be usually X=0006h,Y=000Dh (the location of the Custom
Building, at 7E4C8Ch) (although one can mis-use other xloc/yloc values to
animate completely different map locations; this works only if the Custom
Building's folder contains files/items).

BUG: The one-shot animation is applied ONLY to map cells that are INITIALLY
visible (ie. according to BG scroll offsets at time when entering the town).

**BG Animation Frame Data Block(s)**

```
  00h     Y*8 Group(s) of 4 words (xloc, yloc, bg1_cell, bg2_cell)
  00h+Y*8 2   End of Frame List (8000h) (ie. xloc=8000h=end)
```

NOTE: offset_to_frame_data is based at [94] (whereas, [94] points to the
location after Chunk 01h ID/Length, ie. to the base address of Chunk 01h plus
3).

If the animation goes forwards/backwards, one may use the same offsets for both
passes.

The Custom Bulding has 4x7 cells in non-animated form (so, usually one should
use xloc=0..3, yloc=0..6). Cells can be FFFFh=Don't change (usually one would
change only BG1 foreground cells, and set BG2 background cells to FFFFh). For
animated BG1 cells, Bit10-15 are stripped for some stupid reason (in result,
animated BG1 cells cannot be flagged as Doors via Bit15).

**BG Cell to Tile Translation**

Each 16x16 pixel Cell consists of four 8x8 Tiles. The Translation table
contains tiles arranged as Upper-Left, Lower-Left, Upper-Right, Lower-Right.
The 16bit table entries are 10bit BG tile numbers, plus BG attributes (eg.
xflip).

**Building/Map Notes**

The PPU runs in BG Mode 1 (BG1/BG2=16-color, BG3=4-color). VRAM used as so:

```
  BG1 64x32 map at VRAM:0000h-07FFh, 8x8 tiles at VRAM:1000h-4FFFh (foreground)
  BG2 64x32 map at VRAM:0800h-0FFFh, 8x8 tiles at VRAM:1000h-4FFFh (background)
  BG3 32x32 map at VRAM:5000h-53FFh, 8x8 tiles at VRAM:5000h-6FFFh (menu text)
  OBJ 8x8 and 16x16 tiles at VRAM:6000h-7FFFh (without gap)        (people)
  Custom BG1/BG2 Tiles are at VRAM:4900h-xxxxh (BG.Tile No 390h-xxxh)
  Custom OBJ Tiles at VRAM:7C00h-xxxxh (OBJ.Tile No 1C0h..xxxh)
```

The PPU BG Palettes are:

```
  BG.PAL0 Four 4-Color Palettes (for Y-Button BG3 Menu)
  BG.PAL1 Four 4-Color Palettes (for Y-Button BG3 Menu)
  BG.PAL2 Buildings
  BG.PAL3 Buildings
  BG.PAL4 Buildings
  BG.PAL5 Custom Palette
  BG.PAL6 Landscape (Trees, Phone Booth)         (colors changing per season)
  BG.PAL7 Landscape (Lawn, Streets, Water, Sky)  (colors changing per season)
```

The town map is 48x48 cells of 16x16 pix each (whole map=768x768pix).

```
  Custom BG1/BG2 Cells are at WRAM:7E4080h-7E41FFh (Cell No 3D0h-3FFh)
  Custom Cell Solid/Priority...
```

### SNES Cart Satellaview Other Packets

**SNES Patch Packet (by 105BBC) (Software Channel 1.1.0.7)**

Loaded to 7FC000h, data portions then copied to specified addresses. Size (max)
16Kbytes.

```
  00h  1   Number of entries (01h..FFh) (MUST be min 01h)
  01h  ..  Entries (max 3FFFh bytes)
```

Each entry is:

```
  00h  1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h  1   Unknown/unused
  02h  2   Length (N) (16bit, big-endian) (MUST be min 0001h)
  04h  3   SNES-specific Memory Address (24bit, big-endian)
  07h  N   Data (N bytes)
```

The data portions are copied directly to the specified SNES addresses (the BIOS
doesn't add any base-offset to the addresses; ie. if the data is to be copied
to WRAM, then the satellite would transmit 7E0000h..7FFFFFh as address; there
is no address checking, ie. the packet can overwrite stack or I/O ports).

**Welcome Message Packet (Software Channel 1.1.0.4)**

Loaded to 7EA1B9. Size max 64h bytes (100 decimal). Displayed in text window
with 37x4 ASCII cells.

```
  00h  100 Custom Message (max 99 characters, plus ending 00h)
```

If this packet is included in the Channel Map (and if it's successfully
received), then the Custom Message is displayed right before entering the town.
The japanese Default Message is still displayed, too (so one gets two messages
- which is causing some annoying additional slowdown).

**Time Channel Packet (Software channel 1.1.0.8) (BS Fire Emblem)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Satella2 1)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Parlor Parlor 2)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Shin Onigashima 1)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Tantei Club)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Kodomo Tyosadan..)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Super Mario USA 3)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Super Mario Collection 3)**

**Time Channel Packet (Software channel 1.1.0.8) (BS Excitebike - Mario .. 4)**

**Time Channel Packet (Software channel 1.1.0.8) (Itoi Shigesato no Bass Tsuri)**

**Time Channel Packet (Software channel 1.2.0.48) (BS Dragon Quest 1)**

**Time Channel Packet (Software channel 1.2.0.48) (BS Zelda .. Remix)**

**Time Channel Packet (HW channel [7FFFF7h]) (BS Zelda - Kodai .. Dai 3 ..)**

**Time Channel Packet (HW channel [7FFFF7h]) (BS Marvelous Camp Arnold 1)**

**Time Channel Packet (HW channel [7FFFF7h]) (BS Marvelous Time Athletic 4)**

Preceeded by a 10-byte packet header. Of which, first 5 bytes are ignored (Body
is 8-bytes, so packet size should be probably 5+8). Fixed 01h must be 01h.
Number of Fragments must be 01h. Target Offset must be 000000h. The 8-byte Data
Body is then:

```
  00h  1  Unknown/unused (probably NOT seconds) ;(un-)used by Itoi only
  01h  1  Minutes     (0..3Bh for 0..59)
  02h  1  Hours       (0..17h for 0..23) (or maybe 24..26 after midnight)
  03h  1  Day of Week (01h..07h) (rather not 00h..06h) (?=Monday)
  04h  1  Day         (01h..1Fh for 1..31)
  05h  1  Month       (01h..0Ch for 1..12)
  06h  1  Unknown/unused (maybe year)    ;\could be 2x8bit (00:00 to 99:99)
  07h  1  Unknown/unused (maybe century) ;/or maybe 16bit (0..65535) or so
```

Caution: The BS satellite program specified hours from 11..26 (ie. it didn't
wrap from 23 to 0), the Time Channel(s) might have been following that
notation; if so, then Date values might have also been stuck on the previous
day. Unknown if the Time Channels have been online 24 hours a day, or if their
broadcast ended at some time late-night.

Time Channel are often used by Soundlink games, in so far, it's also quite
possible that the "hours:minutes" are referring to the time within the
broadcast (eg. from 0:00 to 0:59 for a 1-hour broadcast duration), rather than
to a real-time-clock-style time-of-the-day.

Differences between 1.1.0.8 and 1.2.0.48 are unknown. Some games require
incoming Hardware channel number at [7FFFF7h] (from a separate loader or so).

One would expect TIME[0] to contain seconds - however, there aren't any games
using that entry as seconds (instead, they wait for minutes to change, and
reset seconds to zero; due to that wait-for-next-minute mechanism, many BSX
games seem to "hang" for up to 60 seconds after booting them).

TIME[4..7] aren't really used as "date" by any games (a few games seem to be
using TIME[4] or TIME[5] to determine how often the user has joined the game).
Itoi uses (and displays) TIME[4..5] as date.

Some games are treating TIME[6..7] as a 16bit little-endian value, others are
using only either TIME[6] or TIME[7] (ie. half of the games that use the "year"
values are apparently bugged).

**File Packet (Software Channel N.N.N.N as taken from Directory)**

```
  00h  N   Data, N bytes (N=Filesize as specified in Directory)
```

The file must contain a Satellaview Transmit Header (at file offset 7Fxxh or
FFxxh), that header is similar to the FLASH File Header. For details &
differences, see:

The filesize should be usually a multiple of 128Kbytes (because the checksum in
File Header is computed accross that size). Transmitting smaller files is
possible with some trickery: For FLASH download, assume FLASH to be erased (ie.
expand checksum to FFh-filled 128Kbyte boundary). For PSRAM download, the
checksum isn't verified (so no problem there). Moreover, filesize should be
usually at least 32Kbytes (for LoROM header at 7Fxxh), however, one could
manipulate the fragment-offset in packet header (eg. allowing a 4Kbyte file to
be loaded to offset 7000h..7FFFh).

**Special Channel(s) used by Itoi Shigesato no Bass Tsuri No. 1 (1.2.130.N)**

Itoi supports four special channel numbers to unlock special contests. The game
doesn't try to receive any data on that channels (it only checks if one of them
is listed in the Channel Map).

```
  1.2.130.0      ;\Special Contests 1..4 (or so)
  1.2.130.16     ; The 4 contests are looking more or less the same, possibly
  1.2.130.32     ; with different parameters, different japanese descriptions,
  1.2.130.48     ;/contest 2-3 have "No fishing" regions in some lake-areas.
  1.2.130.other  ;-Invalid (don't use; shows error with TV-style "test screen")
```

For Itoi, the Channel Map may be max 1019 bytes (or even LESS?!) (unlike
usually, where it could be 1485 bytes). There should be only one channel with
value 1.2.130.N in the Channel Map. The game additionally requires the 1.1.0.8
Time Channel. And, uses the "APU" byte in the Town Status packet.

**1.2.129.0  Special Channel used by Derby Stallion 96 (Dish on Building/Roof)**

**1.2.129.16 Special Channel used by Derby Stallion 96 (6th Main Menu Option)**

Differences between the two channels are unknown; both are processed by the
same callback function (and thus seem to have the same data-format). The
packet(s) are both loaded to 7F0000h, size could be max 8000h (although, actual
size might be 7E00h, as indicated by the checksum calculation). The overall
format is:

```
  0000h 3    Unknown/unused (3 bytes)
  0003h 8    ID "SHVCZDBJ" (compared against [B289D6])
  000Bh 2    Number of Chunks at 0010h and up (16bit) (little-endian) (min 1)
  000Dh 1    Unknown/unused? (8bit)
  000Eh 1    Checksum (bytes at [0000h..7DFFh] added together) (8bit)
  000Fh 1    Checksum complement (8bit)
  0010h DF0h Chunks
  7E00h 200h Begin of non-checksummed area (IF ANY) (if it DOES exist,
             then it MIGHT contain a file-style header at 7FB0h..7FFFh ?)
```

Note: The Chunks are processed via function at B28A10h. Despite of the
hardcoded channel numbers, the packets must be also listed (with the same
channel numbers) in the Directory Packet (as hidden Include File entries or
so).

**Hardware Channel 0121h - Test Channel**

Used for hardware-connection test (received data is ignored). Used by BIOS
function 105B6Ch; which is used only when pressing X+L+R buttons while the
Welcome Message is displayed. Transmission Timeout for the Test Channel is 10
seconds.

### SNES Cart Satellaview Buildings

**Home Building (Starting Point)**

This building can be entered at any time, the four japanese options are:

```
  1) Load File from FLASH Card
  2) Delete File from FLASH Card
  3) Download File (only files that are "Available at Home") (max 32 files)
  4) Delete Settings in SRAM
```

**Buildings**

The buildings are numbered roughly anti-clockwise from 00h..1Fh, starting at
lower-left of the town map:

```
  00h Robot Skyscraper (lower-left)
  01h News Center
  02h Parabol Antenna
  03h Junkfood
  04h Police
  05h Maths +-x/
  06h Beach Shop (Shop for Predefined ROM Items)
  07h Turtle-Arena
  08h C-Skyscaper (Shop for Predefined ROM Items)
  09h Red-Heart Church
  0Ah Red \\\ Factory (upper-right corner)
  0Bh Dracula Gift-Shop
  0Ch Cow-Skull Church
  0Dh Spintop/Abacus (near Maths +-X/)
  0Eh Blank Skyscraper (near Parabol Antenna)
  0Fh Sign (near Red Factory) (works only ONCE)     (or custom building)
  10h Greek Buddah Temple (upper-end)
  11h Bigger Neighbor's Building
  12h Smaller Neighbor's Building (unknown how to get in there)
  13h Phone Booth (can be entered only with Telephone Card item)
  14h Sewerage (near Spintop) (Shop for Predefined ROM Items)
  15h Unused
  16h Unused ;\these Building-Folders MUST not exist (else BIOS randomly
  17h Unused ;/crashes, accidently trying to animate Building number "44h/3")
  18h Special Location without folder: Player's Home
  19h Special Location without folder: Hydrant (near police)
  1Ah Special Location without folder: Talking Tree (near C-Skyscraper)
  1Bh Special Location without folder: Fountain (or Fountain Replacement)
  1Ch Special Location without folder: Beach Toilets (Railway Station)
  1Dh Special Location without folder: Ocean's Shore
  1Eh Special Location without folder: Unused
  1Fh Special Location without folder: Unused
  20h-3Fh Building-Folders with these IDs do destroy memory (don't use!)
```

Buildings can be entered only if there is a corresponding folder (in the
directory), and only if the folder contains at least one file or item (for the
three pre-defined Shops it works also if the folder is empty).

### SNES Cart Satellaview People

**People**

People are showing up only if they are flagged in the 64bit People Present
Flags in the Town Status packet. If more than 5 people are flagged, then only
the first 5 are shown (regardless of that limit, the 4 frogs and the ship can
additionally be there).

```
  00h Red Ball (on beach) (disappears after access)
  01h Spring-boot (aka Dr.Hiroshi's Shop) (near news center) (sells items)
  02h General Pee (showing up here and there pissing against buildings)
  03h Brown Barbarian on Cocaine (near temple)
  04h Blue Depressive Barbarian (near temple)
  05h Ghost Waver (near phone booth)
  06h Boy in Neighborhood
  07h Older Elvis (near churches)
  08h Purple Helmet (on beach)
  09h Surfer (near beach shop)
  0Ah Grayhaired (northwest lawn)
  0Bh Alien Man (near phone booth)
  0Ch Uncle Bicycle (near parabol antenna)
  0Dh Circus Man (near temple/lake)
  0Eh Speedy Blind Man (near parabol antenna/spintop)
  0Fh Blonde Boy (near factory)
  10h Girl with Pink Dress (near Bigger Neighbor's Home)
  11h Brunetty Guy (near Bigger Neighbor's Home)
  12h Brunette (near junkfood)
  13h Darkhaired (near junkfood)
  14h Blue Longhair (near junkfood)
  15h Brunette Longhair (near junkfood)
  16h Brunette Longhair (near red-heart church)
  17h Green Longhair (near red-heart church)
  18h Bicycle Girl (near C-Skyscraper)
  19h Brunette Office Woman (near C-Skyscraper)
  1Ah Blue Longhair (near parabol antenna)
  1Bh Turquoise Longhair (near home)
  1Ch Blue Longhair (near maths/spintop)
  1Dh Brunette Longhair (near news center/beach stairs)
  1Eh Black Longhair (near police)
  1Fh Red Longhair (southeast beach)
  20h Blackhaired Girl (near police)
  21h Greenhaired Girl (on bench between temple and lake)
  22h Graybluehaired older Woman (east of C-skyscraper)
  23h Darkhaired Housewife (near home)
  24h Traditional Woman (west of Robot-Skyscraper)
  25h Greenhaired Girl (near Turtle-Arena)
  26h Pinkhaired Girl (near Cow-Skull Church)
  27h Brown Dog (northeast lawn)
  28h White Dog (near home)
  29h Gray Duck (near temple/lake)
  2Ah Portable TV-Headed Guy (near Robot-Skyscraper)
  2Bh Satellite Wide-screen TV-Headed Guy (near Robot-Skyscraper)
  2Ch Custom Person 2Ch  ;\may be enabled only if defined in Expansion Area
  2Dh Custom Person 2Dh  ;/of Directory Packet (otherwise crashes)
```

Below 2Eh-37h are specials which cannot have a Folder assigned to them:

```
  2Eh Dead Dentist (on bench near lake) (gives Money when owning Fishing Pole)
  2Fh Gimmick: Allows to use Bus/Taxi/Ferrari Tickets at Fountain
  30h Gimmick: Allows to use Express/Museum-Train-Tickets at Railways Station
  31h Gimmick: Special Event when accessing the Hydrant
  32h Frog 32h (west of Robot-Skyscraper)       ;Change Identity Item
  33h Frog 33h (west of Robot-Skyscraper, too)  ;Change GUI Border Scheme
               (or on street near turtle arena?)
  34h Frog 34h (northwest lawn)                 ;Change GUI Color Scheme
  35h Frog 35h (near Cow-Skull Church)          ;Change GUI Cursor Shape
  36h Gimmick: Allows to use Whale/Dolphin/Fish Food at Oceans Shore
  37h Ship (cannot be accessed?) (near factory)
  38h Mr.Money (near police) (donates a 500G coin)  ;\only one can be present
  39h Mr.Money (near police) (donates a 1000G coin) ; at once, after the coin,
  3Ah Mr.Money (near police) (donates a 5000G coin) ;/all do act as Folder 38h
  3Bh-3Fh Unused?
```

Like Buildings, People can have a Folder associated to them (using above values
as Folder ID, at least, that works for People 00h..2Bh).

If there is no corresponding folder transmitted, then People aren't doing
anything useful (aside from producing whatever japanese sentences). One
exception: If there's no People-File-Folder with ID=01h, then the Spring-boot
guy acts as "Dr Hiroshi's Shop" where one can buy question-marked items for
3000G. The Frogs can be picked up (adding an Item to the inventory). Frog
positions seem to be random (west of Robot-Skyscraper, street near Turtle
Arena, northwest lawn, or near Cow-Skull Church).

**Avatars**

These are assigned for folders (either for people in buildings, or people on
the streets).

```
  00h Geisha (faecher)
  01h Snorty (nose bubble)
  02h Gold hat
  03h Naked guy
  04h Soldier (lanze)
  05h Whore (lipstick/eye blinking)
  06h Wise man1 (huge white eyebrows)
  07h Wise man2 (huge stirn, huge ears)
  08h DJ Proppy (headphones, sunglasses, muscles)
  09h Casino manager (fat guy with red slip-knot)
  0Ah Student girl (manga, karierte bluse)
  0Bh School girl (satchel ranzen/smiley)
  0Ch Kinky gay (green hair, sunglasses, gold-ohrring)
  0Dh Yankee (glistening teeth/dauerwelle)
  0Eh Robot (blech-mann)
  0Fh Blonde chick (blonde, lipstick)
  10h BS-X logo (not a person, just the "BS-X" letters)
  11h-FFh Unknown/Unused/Crashes
  30h None (seems to be an un-intended effect, probably unstable, don't use)
```

### SNES Cart Satellaview Items

**Predefined Items (and their 24bit memory pointers)**

Items sold in C-Skyscraper:

```
 00 88C229h Transfer Device (allows to teleport to any building) (unlimited)
 01 88C2B8h Telephone Card (5) (allows to enter phone booth) ;\
 02 88C347h Telephone Card (4) (allows to enter phone booth) ; decreases
 03 88C3D6h Telephone Card (3) (allows to enter phone booth) ; after usage
 04 88C465h Telephone Card (2) (allows to enter phone booth) ;
 05 88C4F4h Telephone Card (1) (allows to enter phone booth) ;/
 06 88C583h Fishing Pole (allows to get Money from Dead Dentist, Person 2Eh)
 07 88C612h Express Train Ticket                  ;\these are treated special
 08 88C6A1h Museum Train Ticket                   ;/by code at 88936Ch
 09 88C630h Bus Ticket (at Fountain)    ;\these all have same description
 0A 88C7BFh Taxi Ticket                 ;
 0B 88C84Eh Ferrari Blowjob Ticket      ;/
```

Items sold by Dr.Hiroshi (spring-boot guy near News Center)

```
 0C 88C8DDh Doping Item (walk/run faster when pushing B Button)
 0D 88C96Ch Unknown (disappears after usage)
```

Items sold in Beach Shop:

```
 0E 88C9FBh Whale Food (can be used at Oceans Shore)
 0F 88CA8Ah Dolphin Food (can be used at Oceans Shore)
 10 88CB19h Fish Food (can be used at Oceans Shore)
```

Items sold in Sewerage:

```
 11 88CBA8h Boy/Girl Gender Changer (can be used only once)
 12 88CC37h Transform Boy/Girl into Purple Helmet guy (Person 08h)(temporarily)
 13 88CCC6h Transform Boy/Girl into Brunette chick    (Person 1Dh)(temporarily)
 14 88CD55h Smaller Neighbor's Home Door Key (allows to enter that building)
```

Items obtained when picking-up Frogs:

```
 15 88CDE4h Change Identity (edit user name) (from Frog 32h) (works only once)
 16 88CE73h Change GUI Border Scheme         (from Frog 33h) (works only once)
 17 88CF02h Change GUI Color Scheme          (from Frog 34h) (works only once)
 18 88CF91h Change GUI Cursor Shape          (from Frog 35h) (works only once)
```

**Item Format**

As shown above, 25 items are defined in ROM at 99C229h-88D020h with 8Fh bytes
per item. Custom Items (defined in Directory packet's "File" entries) can be
stored at 10506Ah. The item format is:

```
  00h 15h Item Name (max 20 chars, plus ending 00h) (First 2 bytes 00h = Free)
  15h 1   Length of following (Description, Pointer, Whatever) (always 79h?)
  16h 25h Item Description (max 36 chars, plus ending 00h)
  3Bh 47h Item Activation Message (max 70 chars, plus ending 00h)
   If Activation Message = empty (single 00h byte), then Item Function follows:
   3Ch 3   Pointer to Interpreter Tokens (eg. 99974Dh for Transfer Device)
   3Fh 43h Unknown/Unused/Padding (should be zero)
   (there is no SRAM allocated for custom item functions,
   so this part may be used only for predefined ROM items)
  82h 12  Item Price (12-Digit ASCII String, eg. "000000001200" for 1200G)
  8Eh 1   Item Drop/Keep Flag (00h=Drop after Activation, 01h=Keep Item)
```

In case of Custom Items, above ITEM[00h..8Eh] is copied from FILE[02h..90h]
(ie. a fragment of "File" Entries in the Directory Packet).

Entry [15h] seems to be always 79h, giving a total length of 8Fh per item.

The Item Message is used for items that cannot be activated (eg. "You can't use
telephone card outside of the phone booth.",00h). If the message is empty
(00h), then the next 24bit are a pointer to the item handler (eg. the Teleport
function for the Transfer Device).

Note: Items can be listed, activated, and dropped via Y-Button. The teleport
device can be also activated via X-button.

**Shops**

There are four pre-defined shops: Dr.Hiroshi's appears when Person 01h exists,
WITHOUT folder assigned, or WITH an item-folder. The Beach Shop, C-Skyscraper
and Sewerage Shops appear if they HAVE an folder assigned, the folder must be
flagged as Item/Shop. In all cases, the folder may contain additional items
which are added to the Shop's predefined item list. Custom Shops can be created
by assigning Item-Folders to other People/Buildings (in that case, the Folder
MUST contain at least one item, otherwise the BIOS shows garbage). Shops may
contain max 0Ah items (due to 7E865Eh array size).

### SNES Cart Satellaview SRAM (Battery-backed)

The Satellaview BIOS cartridge contains 32Kbyte battery-backed SRAM, mapped to
eight 4Kbyte chunks at 5000h..5FFFh in Bank 10h..17h.

**SRAM Map**

```
  0000h 2    ID "SG" (aka 53h,47h)
  0002h 2    Checksum Complement (same as below checksum XORed with FFFFh)
  0004h 2    Checksum (bytes 0..2FFFh added together) (assume [2..5]=0,0,FF,FF)
  0006h 20   User's Name (Shift-JIS)
  001Ch 2    User's Gender (0000h=Boy, 0001h=Girl)
  001Eh 6    Money (max 00E8D4A50FFFh; aka 999,999,999,999 decimal)
  0024h 2    Number of Items (0..10h) (or temporarily up to 11h)
  0026h 44h  Item Entries (4-bytes each: Type=00/01=ROM/RAM, and 24bit pointer)
  006Ah 8F0h Custom RAM Items (8Fh bytes each) (First 2 bytes 00h = free entry)
  095Ah 2    Remaining Time on Doping Item (decreases when entering buildings)
  095Ch 2    Number of Doping Items (walk/run faster when pushing B Button)
  095Eh 2    Remaining Calls on first Telephone Card Item minus 1
  0960h 2    Number of Telephone Card Items (unlocks Phone Booth)
  0962h 2    Number of Transfer Devices (enables Teleport via menu or X-Button)
  0964h 2    Number of Fishing Poles (allows to get Money from Dead Dentist)
  0966h 2    Number of Smaller Neighbor's Home Keys (0000h=Lock, other=Unlock)
  0968h 2    GUI Cursor Shape 16bit selector (0000h..0005h) (other=crash)
  096Ah 3    GUI Border Scheme 24bit pointer (def=9498D9h) (MUST be 94xxxxh)
  096Dh 3    GUI Color Scheme 24bit pointer (initially 94A431h)
  0970h 1    Player got 500 coin from Person 38h      ;\(00h=No, 01h=Yes,
  0971h 1    Player got 1000 coin from Person 39h     ; flags stay set until
  0972h 1    Player got 5000 coin from Person 3Ah     ; that Person leaves
  0973h 1    Player picked-up Red Ball aka Person 00h ;/the town)
  0974h 18h  BIOS Boot/NMI/IRQ Hook Vectors (retf's) (mapped to 105974h and up)
  098Ch 2B0h BIOS Function Hook Vectors (jmp far's) (mapped to 10598Ch and up)
  0C3Ch 64h  BIOS Reset Function (and some zero-filled bytes)
  0CA0h 100h BIOS Interpreter Token Handlers (16bit addresses in bank 81h)
  0DA0h 100h Garbage Filled (reserved for unused Tokens number 80h..FFh)
  0EA0h 2    Garbage Filled (for impossible 8bit token number 100h)
  0EA2h 215Eh Reserved (but, mis-used for game positions by some games)
  3000h 3000h Backup Copy of 0..2FFFh
  6000h 2000h General Purpose (used for game positions by various games)
```

**Game Positions in SRAM**

```
  0000h-0EA1h  BX-X BIOS (see above)
  1400h-14FFh  BS Super Mario USA 3 (256 bytes)
  1500h-15FFh  BS Super Mario Collection 3 (256 bytes)
  1500h-15FFh  BS Kodomo Tyosadan Mighty Pockets 3 (256 bytes)
  1600h-1626h  BS Satella Walker 2 (27h bytes)
  1600h-1626h  BS Satella2 1 (27h bytes)
  1700h-17FFh  BS Excitebike Bun Bun Mario Battle Stadium 4 (256 bytes)
  2000h-27FFh  BS Marvelous Camp Arnold Course 1 (2Kbytes)
  2800h-2F89h  BS Dragon Quest 1 (1.9Kbytes) (probably 1K, plus 1K backup copy)
  2006h-2FF5h  BS Zelda no Densetsu Remix (3.9Kbytes)
  2000h-2EFFh  BS Zelda no Densetsu Kodai no Sekiban Dai 3 Hanashi (3.75K)
  2000h-2FFFh  BS Super Famicom Wars (V1.2) (first 4K of 8Kbytes)
  3000h-5FFFh  Backup Copy of 0..2FFFh (not useable for other purposes)
  6000h-63FFh  BS Treasure Conflix (1Kbyte)
  6020h-62F9h  BS Sutte Hakkun 98 Winter Event Version (0.7Kbytes)
  6000h-7FFFh  BS Chrono Trigger - Jet Bike Special (8Kbytes)
  6800h-6FFFh  BS Super Famicom Wars (V1.2) (middle 2K of 8Kbytes)
  7500h-7529h  BS Cu-On-Pa (2Ah bytes)
  7800h-7FFFh  BS Super Famicom Wars (V1.2) (last 2K of 8Kbytes)
  7826h-7827h  BS Dr. Mario (only 2 bytes used?)
 (7C00h-7FFFh) BS Radical Dreamers (default, if free, at 7C00h) (1Kbyte)
```

There is no filesystem with filenames nor SRAM allocation. Most games are using
hardcoded SRAM addresses, and do overwrite any other data that uses the same
addresses.

One exception is BS Radical Dreamers: The game searches for a free (zerofilled)
1K block (defaults to using 7C00h-7FFFh), if there isn't any free block, then
it prompts the user to select a 1K memory block (at 6000h-7FFFh) to be
overwritten.

Some games are saving data in PSRAM (eg. Zelda no Densetsu: Kamigami no
Triforce, in bank 70h) rather than SRAM - that kind of saving survives Reset,
but gets lost on power-off.

There aren't any known BS games that save data in FLASH memory. Some BS games
are using passwords instead of saving.

**BIOS Hooks**

These are 4-byte fields, usually containing a "JMP 80xxxxh" opcode (or a "RETF"
opcode in a few cases), changing them to "JMP 1x5xxxh" allows to replace the
normal BIOS functions by updated functions that are installed in SRAM. Unknown
if any such BIOS updates do exist, and at which SRAM locations they are
intended/allowed to be stored.

**SRAM Speed**

The Satellaview SRAM is mapped to a FAST memory area with 3.58MHz access time
(unlike ALL other SNES RAM chips like internal WRAM or external SRAM in other
cartridges).

The bad news is that this wonderful FAST memory isn't usable: It's located in
Bank 10h-17h, so it isn't usable as CPU Stack or CPU Direct Page (both S and D
registers can access Bank 00h only). And, the first 24Kbytes are used as
reserved (and checksummed) area.

### SNES Cart Satellaview FLASH File Header

**Satellaview FLASH File Header**

Located at offset 7Fxxh or FFxxh in file, mapped to FFxxh in bank 00h.

```
  FFB0h 2  Maker Code (2-letter ASCII)                   ;\garbage when
  FFB2h 4  Program Type (00000100h=Tokens, Other=65C816) ; [FFDBh]=01h
  FFB6h 10 Reserved (zero)                               ;/
  FFC0h 16 Title (7bit ASCII, 8bit JIS (?), and 2x8bit SHIFT-JIS supported)
  FFD0h 4  Block Allocation Flags (for 32 blocks of 128Kbytes each) (1=used)
              Retail (demo) games usually have ffff here -- Uh ???
              (exception BS Camp Arnold Marvelous)       -- Uh ???
  FFD4h 2  Limited Starts (bit15=0=Infinite, otherwise bit14-0=Remaining Flags)
  FFD6h 1  Date (Bit7-4=Month, Bit3-0=0)             ;\copied to from
  FFD7h 1  Date (Bit7-3=Day, Bit2=0, Bit1-0=Unknown) ;/Directory Packet
  FFD8h 1  Map Mode (20h=LoROM, 21h=HiROM) (or sometimes 30h/31h)
  FFD9h 1  File/Execution Type
            Bit0-3  Unknown/unused (usually/always 0)
            Bit4    Receiver Power Down (0=No/Sound Link, 1=Power-Down 2197h)
            Bit5-6  Execution Area (0=FLASH, 1=Reloc FLASH-to-PSRAM, 2/3=Fail)
            Bit7    Skip the "J033-BS-TDM1 St.GIGA" Intro (0=Normal, 1=Skip)
  FFDAh 1  Fixed (33h)
  FFDBh 1  Unknown (usually 02h, sometimes 01h, or rarely 00h) (see FFBxh)
  FFDCh 2  Checksum complement (same as below, XORed with FFFFh)
  FFDEh 2  Checksum (all bytes added together; assume [FFB0-DF]=00h-filled)
  FFE0h 32 Exception Vectors (IRQ,NMI,Entrypoint,etc.) (for 65C816 code)
```

Entrypoint is at 800000h+[FFFCh] for 65C816 Machine Code, or at 400000h for
Interpreter Tokens (ie. when [FFB2h]=00000100h, as used by few magazines like
BS Goods Press 6 Gatsu Gou).

Caution: Machine Code programs are started WITHOUT even the most basic
initialization (one of the more bizarre pieces: the download "screensaver" may
leave HDMAs enabled when auto-starting a downloaded file).

**Satellaview PSRAM File Header (download to PSRAM without saving in FLASH)**

Basically same as FLASH headers. FFD9h.Bit7 (skip intro) seems to be ignored
(accidently using FFD9h from FLASH cartridge instead from PSRAM?). The checksum
isn't verified (FFDEh must match with FFDCh, but doesn't need to match the
actual file content).

**Satellaview Transmit Header (located at 7Fxxh or FFxxh in File)**

Basically same as normal Satellaview FLASH File Header. However, after
downloading a file (and AFTER storing it in FLASH memory), the BIOS does
overwrite some Header entries:

```
  FFD0h  4-byte  Block Allocation field (set to whichever used FLASH Blocks)
  FFD6h  2-byte  Date field (set to Date from Satellite Directory Entry)
  FFDAh  1-byte  Fixed Value (set to 33h)
```

Since FLASH cannot change bits from 0 to 1 (without erasing), the above values
must be FFh-filled in the transmitted file (of course, for the fixed value, 33h
or FFh would work) (and of course, the FFh-requirement applies only to FLASH
downloads, not PSRAM downloads).

**Notes**

The title can be 16 bytes (padded with 20h when shorter), either in 7bit ASCII,
8bit JIS (or so?), or 2x8bit SHIFT-JIS, or a mixup thereof. A few files are
somewhat corrupted: Title longer than 16 bytes (and thereby overlapping the
Block Allocation flags).

Limited Starts consists of 15 flags (bit14-0), if the limit is enabled (bit15),
then one flag is changed from 1-to-0 each time when starting the file (the
1-to-0 change can be done without erasing the FLASH sector).

Note that the checksum excludes bytes at FFB0h-FFDFh, this is different as in
normal cartridges (and makes it relative easy to detect if a file contains a
SNES ROM-image or a Satellaview FLASH-file/image.

Files are treated as "deleted" if their Fixed Value isn't 33h, if Limited
Starts is 8000h (limit enabled, and all other bits cleared), or if Checksum
Complement entry isn't equal to Checksum entry XOR FFFFh. Some FLASH dumps in
the internet do have experired Limited Starts entries (so they can be used only
when changing [FFD4h] to a value other than 8000h).

The FLASH card PCBs can be fitted with 1/2/4 MByte chips (8/16/32 Mbit). As far
as known, all existing FLASH cards contain 1MByte chips (so Block Allocation
bit8-31 should be usually always 0). Most files are occupying the whole 1MByte
(so bit0-7 should be usually all set). There are also some 256Kbyte and
512Kbyte files (where only 2 or 4 bits would be set). Minimum file size would
be 128Kbyte. Odd sizes like 768Kbytes would be also possible.

Unlike the Satellite Packet Headers, the FLASH/Transmit-File header contains
"normal" little-endian numbers.

### SNES Cart Satellaview BIOS Function Summary

BIOS functions must be called with BIOS ROM enabled in Bank 80h-9Fh (in some
cases it may be required also in Bank 00h-1Fh), and with DB=80h.
Incoming/outgoing Parameters are passed in whatever CPU registers and/or
whatever WRAM locations. WRAM is somewhat reserved for the BIOS (if a FLASH
file changes WRAM, then it should preserve a backup-copy in PSRAM, and restore
WRAM before calling BIOS functions) (WRAM locations that MAY be destroyed are
7E00C0h..7E00FFh and 7E1500h..7E15FFh, and, to some level: 7F0000h..7FFFFFh,
which is used only as temporary storage).

**Hooks (usually containing RETF opcodes)**

```
  105974 boot_hook (changed by nocash fast-boot patch)
  105978 nmi_hook
  10597C irq_vector
  105980 download_start_hook --> see 9B8000
  105984 file_start_hook --> see 958000
  105988 whatever_hook --> see 99xxxx
```

**SRAM Vectors**

```
  10598C detect_receiver
  105990 port_2194_clr_bit0
  105994 port_2196_test_bit1
```

**Copy Data Queue to RAM Buffer**

```
  105998 set_port_218B_and_218C_to_01h
  10599C set_port_218C_to_00h
  1059A0 read_data_queue
```

**Port 2199h (serial port 2) (maybe satellite audio related)**

```
  1059A4 init_port_2199_registers
  1059A8 send_array_to_port_2199  ;BUGGED?
  1059AC recv_3x8bit_from_port_2199
  1059B0 send_16bit_to_port_2199
  1059B4 recv_8bit_from_port_2199
```

**Port 2198h (serial port 1) (unused/expansion or so)**

```
  1059B8 port_2198_send_cmd_recv_multiple_words
  1059BC port_2198_send_cmd_recv_single_word
  1059C0 port_2198_send_cmd_send_verify_multiple_words
  1059C4 port_2198_send_cmd_send_verify_single_word
  1059C8 port_2198_send_cmd_send_single_word
  1059CC port_2198_send_10h_send_verify_single_word
  1059D0 port_2198_send_cmd_verify_FFFFh
  1059D4 port_2198_send_20h_verify_FFFFh
  1059D8 recv_2198_skip_x BUGGED!
  1059DC recv_2198_want_x
  1059E0 send_30h_to_port_2198
  1059E4 send_00h_to_port_2198
  1059E8 send_8bit_to_port_2198
  1059EC wait_port_2198_bit7
```

**Forward Data Queue from RAM to Target**

```
  1059F0 forward_data_queue_to_target
  1059F4 forward_queue_to_wram
  1059F8 forward_queue_to_psram
  1059FC forward_queue_to_entire_flash
  105A00 forward_queue_to_entire_flash_type1
  105A04 forward_queue_to_entire_flash_type2
  105A08 forward_queue_to_entire_flash_type3
  105A0C forward_queue_to_entire_flash_type4
  105A10 forward_queue_to_flash_sectors
  105A14 forward_queue_to_flash_sectors_type1
  105A18 forward_queue_to_flash_sectors_type2
  105A1C forward_queue_to_flash_sectors_type3
  105A20 forward_queue_to_flash_sectors_type4
  105A24 forward_queue_to_channel_map  ;with 5-byte frame-header
  105A28 forward_queue_to_town_status
```

**FLASH Files**

```
  105A2C scan_flash_directory
  105A30 allocate_flash_blocks
  105A34 .. prepare exec / map file or so
  105A38 verify_file_checksum
  105A3C get_flash_file_header_a
  105A40 delete_flash_file_a
  105A44 get_flash_file_header_5A
  105A48 copy_file_header
  105A4C search_test_file_header, out:[57]
  105A50 test_gamecode_field
  105A54 copy_file_to_psram
  105A58 get_file_size
  105A5C decrease_limited_starts
```

**Memory Mapping**

```
  105A60 map_flash_as_data_file  (for non-executable data-files?)
  105A64 map_psram_as_data_file  (for non-executable data-files?)
  105A68 .. mapping and copy 512Kbytes ?
  105A6C map_flash_for_rw_access
  105A70 map_flash_for_no_rw_access
  105A74 map_flash_for_reloc_to_psram
  105A78 .. mapping (unused?)
  105A7C map_flash_as_lorom_or_hirom
  105A80 execute_game_code
  105A84 .. map_psram_for_streaming ???
  105A88 map_psram_as_lorom_or_hirom
  105A8C .. copy 256Kbytes...
```

**FLASH Memory**

```
  105A90 flash_abort
  105A94 flash_abort_type1
  105A98 flash_abort_type2
  105A9C flash_abort_type3
  105AA0 flash_abort_type4
  105AA4 flash_erase_entire
  105AA8 flash_erase_entire_type1
  105AAC flash_erase_entire_type2
  105AB0 flash_erase_entire_type4 ;4!
  105AB4 flash_erase_entire_type3
  105AB8 flash_test_status   ERASE-PROGRESS
  105ABC flash_test_status_type1
  105AC0 flash_test_status_type2
  105AC4 flash_test_status_type4 ;4!
  105AC8 flash_test_status_type3
  105ACC flash_erase_first_sector
  105AD0 flash_erase_first_sector_type1
  105AD4 flash_erase_first_sector_type2
  105AD8 flash_erase_first_sector_type3
  105ADC flash_erase_first_sector_type4
  105AE0 flash_erase_next_sector
  105AE4 flash_erase_next_sector_type1
  105AE8 flash_erase_next_sector_type2
  105AEC flash_erase_next_sector_type3
  105AF0 flash_erase_next_sector_type4
  105AF4 flash_write_byte
  105AF8 flash_write_byte_type1
  105AFC flash_write_byte_type2
  105B00 flash_write_byte_type3
  105B04 flash_write_byte_type4
  105B08 flash_get_free_memory_size
  105B0C flash_get_and_interprete_id
  105B10 flash_get_id
  105B14 flash_init_chip
  105B18 flash_init_chip_type1
  105B1C flash_init_chip_type2
  105B20 flash_init_chip_type3
  105B24 flash_init_chip_type4
```

**Satellite Directory**

```
  105B28 apply_satellite_directory
  105B2C directory_find_8bit_folder_id
  105B30 directory_find_32bit_file_channel
  105B34 test_if_file_available
  105B38 download_file_and_include_files
  105B3C directory_find_32bit_bugged
```

**Misc...**

```
  105B40 .. initialize stuff on reset
  105B44 download_nmi_handling (with download_callback etc.)
  105B48 download_nmi_do_timeout_counting
  105B4C nmi_do_led_blinking
  105B50 mark_flash_busy
  105B54 mark_flash_ready
  105B58 set_port_2197_bit7
  105B5C clr_port_2197_bit7
  105B60 detect_receiver_and_port_2196_test_bit1
  105B64 init_flash_chip_with_err_29h
  105B68 init_flash_chip_with_err_2Ah
  105B6C detect_receiver_and_do_downloads
  105B70 do_download_function
  105B74 retry_previous_download
  105B78 set_target_id_and_search_channel_map
  105B7C apply_target_for_download
  105B80 clear_queue_and_set_13D1_13D2
  105B84 flush_old_download   ;[218C]=0, clear some bytes
```

**Invoke Download Main Functions**

```
  105B88 download_to_whatever (BUGGED)
  105B8C download_channel_map
  105B90 download_welcome_message
  105B94 download_snes_patch
  105B98 download_town_status
  105B9C download_town_directory
  105BA0 download_to_memory
```

**Download sub functions**

```
  105BA4 add_download_array
  105BA8 wait_if_too_many_downloads
  105BAC do_download_callback
  105BB0 dload_channel_map_callback_1
  105BB4 dload_channel_map_callback_2
  105BB8 dload_welcome_message_callback
  105BBC dload_snes_patch_callback
  105BC0 dload_town_status_callback_1
  105BC4 dload_town_status_callback_2
  105BC8 dload_town_directory_callback_1
  105BCC dload_town_directory_callback_2
  105BD0 .. flash status
  105BD4 dload_to_mem_wram_callback1          ;\
  105BD8 dload_to_mem_wram_callback2          ;
  105BDC dload_to_mem_psram_callback1         ;
  105BE0 dload_to_mem_psram_callback2         ; dload_to_memory_callbacks
  105BE4 dload_to_mem_entire_flash_callback1  ;
  105BE8 dload_to_mem_entire_flash_callback2  ;
  105BEC dload_to_mem_free_flash_callback1    ;
  105BF0 dload_to_mem_free_flash_callback2    ;/
  105BF4 dload_to_mem_entire_flash_callback_final
  105BF8 dload_to_mem_free_flash_callback_final
  105BFC reset_interpreter_and_run_thread_958000h
  105C00 verify_channel_map_header
  105C04 raise_error_count_check_retry_limit
  105C08 search_channel_map
  105C0C post_download_error_handling
  105C10 .. erase satellite info ?
```

**APU Functions**

```
  105C14 apu_flush_and_clear_queues
  105C18 apu_flush_raw
  105C1C apu_message
  105C20 apu_nmi_handling
  105C24 apu_upload_extra_thread
  105C28 apu_upload_curr_thread
  105C2C apu_enable_effects_music_b
  105C30 apu_enable_effects_music_a
  105C34 apu_mute_effects_and_music
  105C38 apu_enable_effects_only
```

**Reset**

```
  105C3C reboot_bios (this one works even when BIOS=disabled or WRAM=destroyed)
```

**Further Stuff**

```
  105C96 Unused 7 bytes (used for nocash fast-boot patch)
  105C9D Unused 3 bytes (zero)
  105CA0 Token Vectors (16bit offsets in bank 81h)
```

**BIOS Tables**

```
  105xxx Tables in SRAM (see above)
  808000 Unsorted ptrs to BIOS Functions, Token-Extensions, and OBJ-Tile-Data
  9FFFF0 Pointers to source data for APU uploads
```

**Additional BIOS Functions (without SRAM-Table vectors)**

These are some hardcoded BIOS addresses (used by some FLASH programs).

```
  808C2A Invoke_dma_via_ax_ptr
  8091B6 Create_machine_code_thread
  809238 Pause_machine_code_thread
  80938F Do nothing (retf) (used as dummy callback address)
  80ABC8 ...whatever
  80AC01 ...whatever
  80B381 Upload_gui_border_shape_to_vram
  80B51B Clear_text_window_content
  80B91E Fill_400h_words_at_7E76000_by_0080h   ;clear whole BG3 map in WRAM
  80EB99 Injump_to_APU_Town_Status_handling (requires incoming pushed stuff)
  81C210 Reset_interpreter
  81C29A Set_interpreter_enable_flag
  81C2B0 Create_interpreter_thread
  81C80E Deallocate_all_obj_tiles_and_obj_palettes
```

Note: Some of the above functions are also listed in the table at 808000h.

**Returning to BIOS**

If an executable file wants to return control to BIOS, it must first reset the
APU (if it has uploaded code to it), and then it can do one the following:

Perform a warmboot (the BIOS intro is skipped, but the Welcome message is
re-displayed, and the player is moved back to the Home building):

```
  jmp 105C3Ch   ;srv_reboot_bios (simple, but quite annoying)
```

Or return to the BIOS NMI handler (from within which the executable was
started) this is done by many games (player returns to the most recently
entered building, this is more elegant from the user's view, though requires a
messy hack from the programmer's view):

```
  call restore_wram             ;-restore WRAM (as how it was owned by BIOS)
  jmp  far (($+4) AND 00FFFFh)  ;-PB=00h (so below can map BIOS to bank 80h)
  mov  a,80h   ;\                               ;\
  push a ;=80h ;                                ; set DB=80h, and
  pop  db      ;/                               ; enable BIOS in bank 80h-9Fh
  mov  [085000h],a ;map BIOS to bank 80h-9Fh    ; (though not yet in 00h-1Fh)
  mov  [0E5000h],a ;apply                       ;/
  call far 99D732h  ;super-slow ;out: M=0       ;-upload [9FFFF0h] to APU
 .assume p=10h  ;(above set M=0, and keeps X=unchanged)
  call far 81C210h                              ;-Reset Token Interpreter
  call far 81C29Ah                              ;-Enable/Unpause Interpreter
  call far 80937Fh ;set NMI callback to RETF prevent FILE to be executed AGAIN)
  mov  x,[13B2h]       ;BIOS online flag (8bit)  ;\skip below if offline
  jz   @@skip                                    ;/
  push pb       ;\retadr for below               ;\
  push @@back-1 ;/                               ;
  push db       ;-incoming pushed DB for below   ;
  push 7E00h    ;\                               ; init apu effects/music
  pop  db       ; incoming current DB for below  ; (according to APU bits
  pop  db ;=7Eh ;/                               ; in town status packet)
  jmp  far 80EB99h ;--> injump to 105BC0h        ;
 @@back:                                         ;
  .assume p=20h  ;(above set's it so)            ;
  ;(if executed, ie. not when @@skip'ed)         ;/
 @@skip:
  clr  p,30h // .assume p=00h  ;below call 81C2B0h requires M=0, X=0
  mov  [0CDEh],0000h                            ;-mark fade-in/out non-busy
  mov  a,0099h     ;\                           ;\
  mov  [0BEh],a    ; 99D69A ;BIOS - enter town  ; create_interpreter_thread
  mov  a,0D69Ah    ;/                           ; (99D69Ah = enter town)
  call far 81C2B0h                              ;/
  set  p,20h // .assume p=20h
  mov  a,81h       ;\enable NMI and joypad (unstable: BIOS isn't yet mapped!)
  mov  [4200h],a   ;/caution: ensure that no NMI occurs in next few clk cycles
  mov  a,80h                                    ;\enable BIOS also in bank 0,
  mov  [075000h],a ;map BIOS to bank 00h-1Fh    ; and return to BIOS NMI handler
  jmp  far 80BC27h ;apply [0E5000h]=a, and retf ;/
```

### SNES Cart Satellaview Interpreter Token Summary

**Interpreter Tokens**

```
  00h  ControlSubThread(pEntrypoint)  ;special actions upon xx0000h..xx0005h
  01h  SetXYsignViewDirectionToSignsOfIncomingValues(vX,vY) ;not if both zero
  02h  SleepWithFixedObjShape(wSleep,pObjShape)
  03h  SleepWithXYstepAs9wayObjShape(wSleep,pObjShape1,..,pObjShape9)
  04h  SleepWithXYsignAs9wayObjShape(wSleep,pObjShape1,..,pObjShape9)
  05h  ClearForcedBlankAndFadeIn(wSleep,wSpeedRange?)
  06h  MasterBrightnessFadeOut(wSleep,wSpeedRange?) ;OptionalForcedBlank?
  07h  SetMosaicAndSleep(wSleep,wBgFlags,wMosaicSize)
  08h  N/A (hangs)
  09h  SleepAndBlendFromCurrentToNewPalette(wSleep,vPalIndex,pNewPalette)
  0Ah  HdmaEffectsOnBg3(wSleep,wEffectType,vScrollOffset,vExtraOffset)
  0Bh  SleepWithAngleAs9wayObjShape(wSleep,pObjShape1,..,pObjShape9) ;[18A8+X]
  0Ch  DisableObjsOfAllThreads()
  0Dh  ReEnableObjsOfAllThreads()
  0Eh  SleepWithXYsignAs9wayPlayerGenderObjShape(wSleep,pObjShape1,..,Shape9)
  0Fh  N/A (hangs)
  10h  SleepAndSetXYpos(wSleep,vX,vY)
  11h  SleepAndMoveTowardsTargetXYpos(wSleep,vX,vY)
  12h  SleepAndMoveByIncomingXYstep(wSleep,vX,vY)
  13h  SleepAndMoveAndAdjustXYstep(wSleep,vRotationAngleToOldXYstepOrSo?)
  14h  SleepAndMoveWithinBoundary(wSleep,vX1,vX2,vY1,vY2,wFactor?)
  15h  SleepAndMoveChangeBothXYstepsIfCollideOtherThread(wSleep,wBounceSpeed?)
  16h  SleepAndMoveAndIncrementXYstep(wSleep,vXincr,vYincr,qXlimit,qYlimit)
  17h  SleepAndMoveByIncomingYstepAndWavingXstep(wSleep,wY)
  18h  SleepAndMoveAndAccelerateTowardsTarget(wSleep,vX,vY,vSpeed)
  19h  SleepAndMoveAndSomethingComplicated?(wSleep,vX,vY)  ;out: X,Y=modified
  1Ah  AdjustXYstep(wNewSpeedOrSo?) ;in: [18A8+X]=angle
  1Bh  MoveByOldXYstepWithoutSleep()
  1Ch  SleepAndMoveChangeXYstepIfCollideOtherThread(wSleep,vMask,vX?,vY?)
  1Dh  N/A (hangs)
  1Dh  N/A (hangs)
  1Fh  N/A (hangs)
  20h  Goto(pTarget)
  21h  Gosub(pTarget)   ;max nesting=8 (or less when also using Loops)
  22h  Return()         ;return from Gosub
  23h  QuitThread()     ;terminate thread completely
  24h  LoopStart(wRepeatCount)  ;see token 62h (LoopNext)
  25h  Sleep(wSleep)
  26h  MathsLet(vA,vB)       ;A=B
  27h  MathsAdd(vA,vB)       ;A=A+B      ;1998 if unsigned carry
  28h  MathsSub(vA,vB)       ;A=A-B      ;1998 if signed overflow
  29h  MathsAnd(vA,vB)       ;A=A AND B  ;1998 if nonzero
  2Ah  MathsOr(vA,vB)        ;A=A OR B   ;1998 if nonzero
  2Bh  MathsXor(vA,vB)       ;A=A XOR B  ;1998 if nonzero
  2Ch  MathsNot(vA)          ;A=NOT A    ;1998 if nonzero
  2Dh  MathsMulSigned(vA,vB) ;A=A*B/100h ;1998 never (tries to be overflow)
  2Eh  MathsDivSigned(vA,vB) ;A=A/B*100h ;1998 if division by 0
  2Fh  SignedCompareWithConditionalGoto(vA,wOperator,vB,pTarget)
  30h  GotoIf_1998_IsNonzero(pTarget)
  31h  GotoIf_1998_IsZero(pTarget)
  32h  GotoArray(vArrayIndex,pPointerToArrayWithTargets)
  33h  ReadJoypad(bJoypadNumber,wX,wY)
  34h  CreateAnotherInterpreterThreadWithLimit(vThreadCount,bLimit,pEntry)
  35h  CheckIfXYposCollidesWithFlaggedThreads(vFlagMask) ;out: 1998=ID
  36h  GetUnsignedRandomValue(vA,wB) ;A=Random MOD B, special on B>7FFFh
  37h  SetObjWidthDepthFlagmask(vWidth,vDepth,vMask) ;for collide checks
  38h  CreateAnotherInterpreterThreadWithIncomingXYpos(vX,vY,pEntrypoint)
  39h  N/A (hangs)
  3Ah  SoundApuMessage00h_nnh(vParameter8bit)
  3Bh  SoundApuMessage01h_nnnh(vLower6bit,bMiddle2bit,bUpper2bit)
  3Ch  SoundApuMessage02h_nnnnh(vLower6bit,bMiddle2bit,bUpper2bit)
  3Dh  SoundApuUpload(bMode,pPtrToPtrToData)
  3Eh  SetPpuBgModeKillAllOtherThreadsAndResetVariousStuff(bBgMode)
  3Fh  SetTemporaryTableForBanksF1hAndUp(vTableNumber,pTableBase)
  40h  KillAllFlaggedThreads(vMask)  ;ignores flags, and kills ALL when Mask=0
  41h  SetBUGGEDTimerHotspot(wHotspot) ;BUG: accidently ORed with AE09h
  42h  Ppu_Bg1_Bg2_SetScrollPosition(vX,vY)
  43h  Ppu_Bg1_Bg2_ApplyScrollOffsetAndSleep(wSleep,vX,vY)
  44h  NopWithDummyParameters(wUnused,wUnused)
  45h  NopWithoutParameters()
  46h  AllocateAndInitObjTilesOrUseExistingTiles(wLen,pSrc)
  47h  AllocateAndInitObjPaletteOrUseExistingPalette(pSrc)
  48h  DmaObjTilesToVram(wObjVramAddr,wOBjVramEnd,pSrc)
  49h  SetObjPalette(wObjPalIndex,wObjPalEnd,pSrc)
  4Ah  SramAddSubOrSetMoney(bAction,vLower16bit,vMiddle16bit,vUpper16bit)
  4Bh  SramUpdateChksumAndBackupCopy()
  4Ch  N/A (hangs)
  4Dh  N/A (hangs)
  4Eh  N/A (hangs)
  4Fh  N/A (hangs)
  50h  TestAndGotoIfNonzero(vA,vB,pTarget)  ;Goto if (A AND B)<>0
  51h  TestAndGotoIfZero(vA,vB,pTarget)     ;Goto if (A AND B)==0
  52h  InitNineGeneralPurposePrivateVariables(wA,wB,wC,wD,wE,wF,wG,wH,wI)
  53h  MultipleCreateThreadBySelectedTableEntries(vFlags,vLimit,pPtrToTable)
  54h  PrepareMultipleGosub()  ;required prior to token 6Ah
  55h  StrangeXYposMultiplyThenDivide(wA,wB) ;Pos=Pos*((B-A)/2)/((B-A)/2)
  56h  BuggedForceXYposIntoScreenArea() ;messes up xpos and/or hangs endless
  57h  Maths32bitAdd16bitMul100h(vA(Msw),vB) ;A(Msw:Lsw)=A(Msw:Lsw)+B*100h
  58h  Maths32bitSub16bitMul100h(vA(Msw),vB) ;A(Msw:Lsw)=A(Msw:Lsw)-B*100h
  59h  SoundApuUploadWithTimeout(wTimeout,pPtrToPtrToData)
  5Ah  N/A (hangs)
  5Bh  N/A (hangs)
  5Ch  N/A (hangs)
  5Dh  N/A (hangs)
  5Eh  N/A (hangs)
  5Fh  N/A (hangs)
  60h  CallMachineCodeFunction(pTarget)
  61h  SetTemporaryOffsetFor0AxxxxhVariables(vOffset)
  62h  LoopNext()  ;see token 24h (LoopStart)
  63h  SetForcedBlankAndSleepOnce()
  64h  ClearForcedBlankAndSleepOnce()
  65h  AllocateAndInitObjPaletteAndObjTilesOrUseExistingOnes(pSrc) ;fragile
  66h  WriteBgTiles(wBgNumber,pPtrTo16bitLenAnd24bitSrcPtr)
  67h  WritePalette(pPtrTo16bitLenAnd24bitSrcPtr)  ;to backdrop/color0 and up
  68h  WriteBgMap(wBgNumber,pPtrTo16bitLenAnd24bitSrcPtr)
  69h  KillAllOtherThreads()
  6Ah  MultipleGosubToSelectedTableEntries(vFlags,pPtrToTable) ;see token 54h
  6Bh  AllocateAndInitBgPaletteTilesAndMap2(vX1,vY1,pPtrToThreePtrs,vBgMapSize)
  6Ch  DeallocateAllObjTilesAndObjPalettes()
  6Dh  BuggedSetBgParameters(bBgNumber,pPtr,wXsiz,wYsiz,wUnused,wUnused)
  6Eh  BuggedSetUnusedParameters(bSomeNumber,pPtr,wX,wY)
  6Fh  BuggedChangeBgScrolling(wX,wY)
  70h  PauseAllOtherThreads()
  71h  UnPauseAllOtherThreads()
  72h  GosubIfAccessedByPlayer(pGosubTargetOrPeopleFolderID)
  73h  Dma16kbyteObjTilesToTempBufferAt7F4000h()   ;Backup OBJ Tiles
  74h  Dma16kbyteObjTilesFromTempBufferAt7F4000h() ;Restore OBJ Tiles
  75h  SetFixedPlayerGenderObjShape(pSrc,wLen1,wLen2)
  76h  InstallPeopleIfSatelliteIsOnline() ;create all people-threads
  77h  KillAllOtherThreadsAndGotoCrash()  ;Goto to FFh-filled ROM at 829B5Eh
  78h  ZerofillBgBufferInWram(vBgNumber)
  79h  ChangePtrToObjPriority(vVariableToBePointedTo)  ;default is <Ypos>
  7Ah  ChangeObjVsBgPriority(vPriorityBits)     ;should be (0..3 * 1000h)
  7Bh  SetXYposRelativeToParentThread(vX,vY)
  7Ch  TransferObjTilesAndObjPaletteToVram(pPtrToPtrsToPaletteAndTileInfo)
  7Dh  AllocateAndInitBgPaletteTilesAndMap1(vX1,vY1,pPtrToThreePtrs,vBgMapSize)
  7Eh  DrawMessageBoxAllAtOnce(vWindowNumber,vDelay,vX,vY,pPtrToString)
  7Fh  DrawMessageBoxCharByCharBUGGED(..)  ;works only via CALL, not token 7Fh
  80h..FFh  Reserved/Crashes (jumps to garbage function addresses)
```

**Legend for Token Parameters**

```
  v   16bit Global or Private Variable or Immediate (encoded as 3 token bytes)
  p   24bit Pointer (3 token bytes) (banks F0h..FFh translated, in most cases)
  b   8bit  Immediate (encoded directly as 1 token byte)
  w   16bit Immediate (encoded directly as 2 token bytes)
  q   16bit Immediate (accidently encoded as 3 token bytes, last byte unused)
```

**3-byte Variable Encoding (v)**

```
  +/-00nnnnh  -->  +/-nnnnh          R     ;immediate
  +/-01nnnnh  -->  +/-[nnnnh+X]      R/W   ;private variable (X=thread_id*2)
  +/-02nnnnh  -->  +/-[nnnnh]        R/W   ;global variable
  +  03nnnnh  -->  +[nnnnh+[19A4h]]  W     ;special (write-only permission)
  +  09nnnnh  -->  +[nnnnh+[19A4h]]  R/W   ;special (read/write permission)
  +  0Annnnh  -->  +[nnnnh+[19A4h]]  R     ;special (read-only permission)
  Examples: 000001h or FF0001h (aka -00FFFFh) are both meaning "+0001h".
  021111h means "+[1111h]", FDEEEF (aka -021111h) means "-[1111h]".
```

**3-byte Pointer Encoding (p)**

```
  00nnnnh..EFnnnnh     --> 00nnnnh..EFnnnnh      (unchanged)
  F0nnnnh              --> TokenProgramPtr+nnnn  (relative)
  F1nnnnh (or F2nnnnh) --> [[AFh+0]+nnnn*3]      (indexed by immediate)
  F3nnnnh              --> [[AFh+0]+[nnnn+X]*3]  (indexed by thread-variable)
  F4nnnnh              --> [[AFh+0]+[nnnn]*3]    (indexed by global-variable)
  F5nnnnh (or F6nnnnh) --> [[AFh+3]+nnnn*3]      (indexed by immediate)
  F7nnnnh              --> [[AFh+3]+[nnnn+X]*3]  (indexed by thread-variable)
  F8nnnnh              --> [[AFh+3]+[nnnn]*3]    (indexed by global-variable)
  F9nnnnh (or FAnnnnh) --> [[AFh+6]+nnnn*3]      (indexed by immediate)
  FBnnnnh              --> [[AFh+6]+[nnnn+X]*3]  (indexed by thread-variable)
  FCnnnnh              --> [[AFh+6]+[nnnn]*3]    (indexed by global-variable)
  FDnnnnh..FFnnnnh     --> crashes               (undefined/reserved)
```

**2-byte Operators for Signed Compare (Token 2Fh)**

```
  0000h Goto_if_less              ;A<B
  0001h Goto_if_less_or_equal     ;A<=B
  0002h Goto_if_equal             ;A=B
  0003h Goto_if_not_equal         ;A<>B
  0004h Goto_if_greater           ;A>B
  0005h Goto_if_greater_or_equal  ;A>=B
```

**ControlSubThread(pEntrypoint) values**

```
  xx0000h Pause
  xx0001h UnpauseSubThreadAndReenableObj
  xx0002h PauseAfterNextFrame
  xx0003h PauseAndDisableObj
  xx0004h ResetAndRestartSubThread
  xx0005h KillSubThread
  NNNNNNh Entrypoint (with automatic reset; only if other than old entrypoint)
```

Maximum Stack Nesting is 4 Levels (Stack is used by Gosub and Loop tokens).

**Token Extensions (some predefined functions with Token-style parameters)**

These are invoked via CALL Token (60h), call address, followed by params.

```
  809225h CallKillAllMachineCodeThreads()
  80B47Dh CallGetTextLayerVramBase()
  80B91Eh CallClearBg3TextLayer()
  818EF9h CallSetApuRelatedPtr()
  818F06h CallDrawMessageBoxCharByChar(vWindowNumber,vDelay,vX,vY,pPtrToString)
  818FF0h CallDrawBlackCircleInLowerRightOfWindow()
  81903Dh CallDisplayButton_A_ObjInLowerRightOfWindow()
  81A508h CallSetGuiBorderScheme(pAddr1,pAddr2)
  81A551h CallSetTextWindowBoundaries(wWindowNumber,bXpos,bYpos,bXsiz,bYsiz)
  81A56Eh CallHideTextWindow(wWindowNumber)
  81A57Bh CallSelectWindowBorder(wWindowNumber,wBorder) ;0..3, or FFh=NoBorder
  81A59Ah CallSelectTextColor(wWindowNumber,bColor,bTileBank,bPalette)
  81A5C3h CallClearTextWindowDrawBorder(wWindowNumber)
  81A5D2h CallZoomInTextWindow(wWindowNumber,wZoomType)  ;\1,2,3=Zoom HV,V,H
  81A603h CallZoomOutTextWindow(wWindowNumber,wZoomType) ;/0=None/BuggyWinDiv2
  81A634h CallSetGuiColorScheme(pAddr)
  81A65Dh CallChangePaletteOfTextRow(vX,vY,vWidth,vPalette)
  81A693h CallPeekMemory16bit(vDest,pSource)
  81A6B4h CallPokeMemory16bit(vSource,pDest)
  81C7D0h CallInitializeAndDeallocateAllObjTilesAndObjPalettes()
  81C871h CallDeallocateAllObjs()
  81CDF9h CallBackupObjPalette()
  81CE09h CallRestoreObjPalette()
  829699h CallUploadPaletteVram(pSource,wVramAddr,bPaletteIndex)
  88932Fh CallTestIfFolderExists()  ;in: 0780, out: 1998,077C,077E
  88D076h CallTestIfDoor()
  99D9A4h CallSelectPlayerAsSecondaryThread  ;[19A4]=PlayerThreadId*2
```

Note: Some of these Call addresses are also listed in a 24bit-pointer table at
address 808000h (though the (BIOS-)code uses direct 8xxxxxh values instead of
indirect [808xxxh] values).

**Token Functions (some predefined token-functions)**

These can be invoked with GOSUB token (or GOTO or used as thread entrypoint):

```
  99D69A EnterTown (use via goto, or use as entrypoint)
  828230 DeallocMostBgPalettesAndBgTiles ;except tile 000h and color 00h-1Fh
  88C1C6 SetCursorShape0
  88C1D0 SetCursorShape1
  88C1E0 SetCursorShape2
  88C1EA SetCursorShape3
  88C1F4 SetCursorShape4
  88C1FE SetCursorShape5
  99D8AB PauseSubThreadIfXYstepIsZero
  99D8CD MoveWithinX1andX2boundaries
  99D903 MoveWithinY1andY2boundaries
```

Note: Some of the above functions are also listed in the table at 808000h.

**Compressed Data**

Some of the Functions can (optionally) use compressed Tile/Map/Palette data.

Note: The actual compressed data is usually preceeded-by or bundled-with
compression flags, length entry, and/or (in-)direct src/dest pointers (that
"header" varies from function to function).

### SNES Cart Satellaview Chipsets

**BSC-1A5B9P-01 (1995) (BIOS cartridge PCB)**

```
  U1  44pin  MCC-BSC LR39197 Nintendo
  U2  36pin  ROM (36pin/40pin possible)
  U3  32pin  658512LFP-85 (4Mbit PSRAM)
  U4  28pin  LH52B256NB-10PLL (256Kbit SRAM)
  U5  8pin   MM1134 (battery controller for SRAM)
  BT1 2pin   Battery
  CN1 62pin  SNES Cartridge Edge (pin 2,33 used)
  CN2 62pin  Flash Cartridge Connector (male?)
```

There's no CIC chip (either it's contained in the MCC-chip... or in the flash
card, but in that case the thing won't work without flash card?)

**MAIN-BSA-01 (1995) (receiver unit/expansion port PCB)**

```
  U1 20pin  74LS541 8-bit 3-state buffer/line driver
  U2 20pin  74LS541 8-bit 3-state buffer/line driver
  U3 20pin  74LS245 8-bit 3-state bus transceiver
  U4 8pin   SPR-BSA (unknown, might be controlled via port 2198h or 2199h?)
  U5 100pin DCD-BSA (custom Nintendo chip)
  U6 64pin  MN88821 (maybe a MN88831 variant: Satellite Audio Decoder)
  U7 18pin  AN3915S Clock Regenerator (for amplifying/stabilizing Y1 crystal)
  U8 4pin   PQ05RH1L (5V regulator with ON/OFF control)
  U9 14pin  LM324 Quad Amplifier
  Y1 2pin   18.432MHz crystal
  T1 4pin   ZJYS5102-2PT Transformator
  T2 4pin   ZJYS5102-2PT Transformator
  CN1 28pin SNES Expansion Port
  CN2 38pin Expansion Port (EXT) (believed to be for modem)
  CN3 3pin  To POWER and ACCESS LEDs on Front Panel
  CN4 7pin  Rear connector (satellite and power supply?)
```

**BSMC-AF-01 (Memory Card PCB) (to be plugged into BIOS cartridge)**

```
  U1  56pin Sharp LH28F800SUT-ZI (or -Z1?) (1Mbyte FLASH)
  CN1 62pin Flash Cartridge Connector (female?)
```

There are no other chips on this PCB (only capacitors and resistors).

**BSMC-CR-01 (Memory Card PCB) (to be plugged into GAME cartridges)**

```
  U1  ?pin  unknown (reportedly read-only... mask ROM?)
  CN1 62pin Flash Cartridge Connector (female?)
```

**BSC-1A5M-01 (1995) (GAME cartridge with onboard FLASH cartridge slot)**

```
  U1  36pin  ROM
  U2  28pin  SRAM (32Kbytes)
  U3  16pin  MAD-1A
  U4  16pin  CIC D411B
  BT1 2pin   Battery CR2032
  CN1 62pin  SNES Cartridge Edge (pin 2,33 used)
  CN2 62pin  Flash Cartridge Connector (male 2x31 pins)
```

Used by "Derby Stallion 96" (and maybe other games, too).

**BSC-1L3B-01 (1996) (GAME cartridge with SA1 and onboard FLASH cartridge slot)**

```
  U1  44pin  ROM
  U2  28pin  SRAM (8Kbytes)
  U3  128pin SA1
  U4  8pin   MM1026AF (battery controller for SRAM)
  BT1 2pin   Battery
  CN1 62pin  SNES Cartridge Edge (pin 2,33 used)
  CN2 62pin  Flash Cartridge Connector (male?)
```

Used by "Itoi Shigesato no Bass Tsuri No. 1" (and maybe other games, too).

**Nintendo Power flashcarts**

Theoretically, Nintendo Power flashcarts are also compatible with the BSX
expansion hardware (in terms of connecting EXPAND to SYSCK via 100 ohms),
unknown if any Nintendo Power titles did actually use that feature.

### SNES Cart Data Pack Slots (satellaview-like mini-cartridge slot)

**Data Packs**

Data Packs are Satellaview 8M Memory Packs which have data meant to be used as
expansion for a Data Pack-compatible game. Data Pack-compatible game cartridges
have a resemblence to the BS-X Cartridge itself.

**Usage**

For most of these games, Data was distributed via St.GIGA's Satellaview
services. Same Game and SD Gundam G-Next had some Data Packs sold as retail in
stores. RPG Tsukuru 2, Sound Novel Tsukuru and Ongaku Tsukuru Kanaderu could
save user-created data to 8M Memory Packs.

**Cartridges with Data Pack Slot**

```
  Derby Stallion 96                  (SpecialLoROM, 3MB ROM, 32K RAM)
  Itoi Shigesato no Bass Tsuri No. 1 (SA-1, map-able 4MB ROM, 8K RAM)
  Joushou Mahjong Tenpai             (HiROM, 1MB)
  Ongaku Tukool/Tsukuru Kanaderu     (HiROM, 1MB)
  RPG Tukool/Tsukuru 2               (LoROM, 2MB)
  Same Game Tsume Game               (HiROM, 1MB)
  Satellaview BS-X BIOS              (MCC, 1MB ROM) (FLASH at C00000h)
  SD Gundam G-NEXT                   (SA-1, map-able 1.5MB ROM, 32K RAM)
  Sound Novel Tukool/Tsukuru         (SpecialLoROM, 3MB ROM, 64K RAM)
```

Aside from the BS-X BIOS, two of the above games are also accessing BS-X
hardware via I/O Ports 2188h/2194h/etc (Derby Stallion 96, Itoi Shigesato no
Bass Tsuri No. 1). For doing that, the cartridges do probably require the
EXPAND pin to be wired via 100 ohm to SYSCK.

**Cartridge Header of cartridges with Data Pack Slot**

The presence of Data Pack Slots is indicated by a "Z" as first letter of Game
Code:

```
  [FFB2h]="Z"   ;first letter of game code
  [FFB5h]<>20h  ;game code must be 4-letters (not space padded 2-letters)
  [FFDAh]=33h   ;game code must exist (ie. extended header must be present)
```

**Data Pack Mapping**

```
  MCC (BSX-BIOS)        FLASH at C00000h (continous) (mappable via MCC chip)
  SA-1                  FLASH at <unknown address> (probably mappable via SA1)
  HiROM                 FLASH at E00000h (probably continous)
  LoROM/SpecialLoROM    FLASH at C00000h (looks like 32K chunks)
```

**LoROM/SpecialLoROM Mapping Notes**

The FLASH memory seems to be divided into 32K chunks (mirrored to Cn0000h and
Cn8000h) (of which, Derby Stallion 96 uses Cn8000h, RPG Tukool uses Cn0000h,
and Ongaku Tukool uses both Cn0000h and Cn8000h).

The two 3MB SpecialLoROM games also have the ROM mapped in an unconventional
fashion:

```
  1st 1MB of ROM mapped to banks 00-1F
  2nd 1MB of ROM mapped to banks 20-3F and A0-BF
  3rd 1MB of ROM mapped to banks 80-9F
  1MB of Data Pack FLASH mapped to banks C0-DF
  32K..64K SRAM mapped to banks 70-71
```

Despite of memory-mirroring of 2nd MB, the checksum-mirroring goes on 3rd MB?

Note: Above mapping differs from "normal" 3MB LoROM games like Wizardry 6
(which have 3rd 1MB in banks 40h-5Fh).

### SNES Cart Nintendo Power (flashcard)

Nintendo Power cartridges are official FLASH cartridges from Nintendo (released
only in Japan). Unlike the older Satellaview FLASH cartridges, they do connect
directly to the SNES cartridge slot. The capacity is 4MByte FLASH and 32KByte
battery-backed SRAM.

**FLASH (512Kbyte blocks)**

The FLASH is divided into eight 512Kbyte blocks. The first block does usually
contain a Game Selection Menu, the other blocks can contain up to seven
512KByte games, or other combinations like one 3MByte game and one 512KByte
game. Alternately, the cartridge can contain a single 4MByte game (in that
case, without the Menu).

**SRAM (2Kbyte blocks) (battery-backed)**

The SRAM is divided into sixteen 2Kbyte blocks for storing game positions.
Games can use one or more (or all) of these blocks (the menu doesn't use any of
that memory).

**Nintendo Power Games**

Games have been available at kiosks with FLASH Programming Stations. There are
around 150 Nintendo Power games: around 21 games exclusively released only for
Nintendo Power users, and around 130 games which have been previously released
as normal ROM cartridges.

**Nintendo Power PCB "SHVC-MMS-X1" or "SHVC-MMS-02" (1997) Chipset (SNES)**

```
  U1  18pin CIC       ("F411B Nintendo")
  U2 100pin MX15001   ("Mega Chips MX15001TFC")
  U3  44pin 16M FLASH ("MX 29F1601MC-11C3") (2Mbyte FLASH, plus hidden sector)
  U4  44pin 16M FLASH ("MX 29F1601MC-11C3") (2Mbyte FLASH, plus hidden sector)
  U5  44pin 16M FLASH (N/A, not installed)
  U6  28pin SRAM      ("SEC KM62256CLG-7L") (32Kbyte SRAM)
  U7   8pin MM1134    ("M 707 134B") (battery controller)
  BAT1 2pin Battery   ("Panasonic CR2032 +3V")
```

**Nintendo Power PCB "DMG-A20-01" (199x) Chipset (Gameboy version)**

```
  U1  80pin G-MMC1    ("MegaChips MX15002UCA"
  U2  40pin 8M FLASH  ("MX29F008ATC-14") (plus hidden sector)
  U3  32pin 1M SRAM   ("UT621024SC-70LL")
  X1   3pin N/A       (oscillator? not installed)
  BAT1 2pin Battery   ("Panasonic CR2025")
```

**Nintendo Power Menu SNES Cartridge Header**

```
  Gamecode:        "MENU" (this somewhat indicates the "MX15001" chip)
  ROM Size:        512K (the menu size, not including the other FLASH blocks)
  SRAM Size:       0K (though there is 32Kbyte SRAM for use by the games)
  Battery Present: Yes
  Checksum:        Across 512Kbyte menu, with Directory assumed to be
                   FFh-filled (except for the "MULTICASSETTE 32" part)
```

The PCB doesn't contain a ROM (the Menu is stored in FLASH, too).

**Nintendo Power Menu Content**

```
  ROM Offset  SNES Address Size   Content
  000000h     808000h      4xxxh  Menu Code (around 16K, depending on version)
  004xxxh     80xxxxh      3xxxh  Unused (FFh-filled)
  007FB0h     80FFB0h      50h    Cartridge Header
  008000h     818000h      40000h Unused (FFh-filled)
  048000h     898000h      372Bh  Something (APU code/data or so)
  04B72Bh     8xxxxxh      47D5h  Unused (FFh-filled)
  050000h     8A8000h      8665h  Something (VRAM data or so)
  058665h     8Bxxxxh      798Bh  Unused (FFh-filled)
  060000h     8C8000h      10000h Directory (File 0..7) (2000h bytes/entry)
  070000h     8E8000h      10000h Unused (FFh-filled)
```

**Note**

Nintendo has used the name "Nintendo Power" for various different things:

```
  Super Famicom Flashcards (in Japan)
  Gameboy Color Flashcards (in Japan)
  Super Famicom Magazine (online via Satellaview BS-X) (in Japan)
  Official SNES Magazine (printout) (in USA)
```

### SNES Cart Nintendo Power - New Stuff

**Operation during /RESET=LOW**

```
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=F0h   ;FLASH read/reset command
  [00000h]=38h, [00000h]=D0h, [00000h]=71h   ;FLASH request chip info part 1
  dummy=[00004h]                             ;Read Ready-status (bit7=1=ready)
  [00000h]=72h, [00000h]=75h                 ;FLASH request chip info part 2
  Port[2404h..2407h]=[0FF00h+(n*8)+0,2,4,6]  ;Read mapping info for File(n)
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=F0h   ;FLASH read/reset command
```

**Detailed**

```
  [00000h]=38h   ;copy hidden sector to page buffer?
  [00000h]=D0h   ;  ...confirm above
  [00000h]=71h   ;read extended status register
  dummy=[00004h] ;  ...do read it (bit7=1=ready)
  [00000h]=72h   ;swap page buffer (map above buffer to cpu side?)
  [00000h]=75h   ;read page buffer to cpu
  xx=[0FFxxh]    ;  ...do read it
```

other interesting commands:

```
  [00000h]=74h   ;write page buffer single byte from cpu
  [0xxxxh]=xx    ;  ...do write it
```

or sequential:

```
  [00000h]=E0h   ;sequential load from cpu to page buffer
  [00000h]=num.L ;  ...byte count lsb (minus 1 ?) (0=one byte, what=100h bytes)
  [00000h]=num.H ;  ...byte cound msb (zero)
  [?]=data       ;  ...data?
  [00000h]=0Ch   ;forward page buffer to flash
  [00000h]=num.L ;  ...byte count lsb (minus 1 ?) (0=one byte, what=100h bytes)
  [addr]=num.H   ;  ...byte cound msb (zero)
  [...]?         ;  ...do something to wait until ready
```

**Hidden Mapping Info Example (chip 1 at C0FFxxh, chip 2 at E0FFxxh)**

```
  C0FF00      03 11 AA 74 AA 97 00 12  ;Menu              (512K Lorom, no SRAM)
  C0FF08      00 08 29 15 4A 12 10 01  ;Super Mario World (512K Lorom, 2K SRAM)
  C0FF10      0B FF AA FF AA FF 21 FF  ;Doraemon 4        (1.5M Lorom, no SRAM)
  C0FF18      49 FF 61 FF A5 FF 51 FF  ;Dragon Slayer II  (1.5M Hirom, 8K SRAM)
  C0FF20..FF  FF-filled (byte at C0FF7Fh is 00h in some carts)  ;-unused
  E0FF00..8F  FF-filled (other values at E0FF8xh in some carts) ;\garbage, from
  E0FF90      FF FF 55 00 FF FF FF FF FF FF FF FF FF FF FF FF   ; chip-testing
  E0FFA0      FF FF FF FF FF FF 55 00 FF FF FF FF FF FF FF FF   ; or so
  E0FFB0      FF FF FF FF FF FF 55 00 FF FF FF FF FF FF FF FF   ;/
  E0FFC0..FF  FF-filled                                         ;-unused
```

There are always 8 bytes at odd addresses at C0FF01..0F, interleaved with the
mapping entries 0 and 1 (though no matter if the cart uses 1, 2, or 3 mapping
entries). The 'odd' bytes are some serial number, apart from the first two
bytes, it seems to be just a BCD date/time stamp, ie. formatted as
11-xx-YY-MM-DD-HH-MM-SS.

New findings are that the "xx" in the "11-xx-YY-MM-DD-HH-MM-SS" can be non-BCD
(spotted in the Super Puyo Puyo cart).

Some carts have extra 'garbage' at C0FF7F and E0FF80..BF.

**Nintendo Power Commands**

```
  if [002400h]<>7Dh then skip unlocking   ;else locking would be re-enabled
  [002400h]=09h       ;\
  dummy=[002400h]     ;
  [002401h]=28h       ; wakeup sequence (needed before sending other commands,
  [002401h]=84h       ; and also enables reading from port 2400h..2407h)
  [002400h]=06h       ;
  [002400h]=39h       ;/
```

After wakeup, single-byte commands can be written to [002400h]:

```
  [002400h]=00h   RESET and map GAME14 ? (issues /RESET pulse)
  [002400h]=01h    causes always 8x7D
  [002400h]=02h   Set STATUS.bit2=1 (/WP=HIGH, release Write protect)
  [002400h]=03h   Set STATUS.bit2=0 (/WP=LOW, force Write protect)
  [002400h]=04h   HIROM:ALL  (map whole FLASH in HiROM mode)
  [002400h]=05h   HIROM:MENU (map MENU in HiROM mode instead normal LoROM mode)
  [002400h]=06h    causes always 8x7D (aka, undoes toggle?)
  [002400h]=07h    causes always 8x7D
  [002400h]=08h    causes always 8x7D
  [002400h]=09h    no effect  ;\
  [002400h]=0ah    no effect  ;/
  [002400h]=0bh    causes always 8x7D
  [002400h]=0ch    causes always 8x7D
  [002400h]=0dh    causes always 8x7D
  [002400h]=0eh    causes always 8x7D
  [002400h]=0fh    causes always 8x7D
  [002400h]=10h    causes always 8x7D
  [002400h]=14h    causes always 8x7D
  [002400h]=20h    Set STATUS.bit3=0 (discovered by skaman) (default)
  [002400h]=21h    Set STATUS.bit3=1 (discovered by skaman) (disable ROM read?)
  [002400h]=24h    causes always 8x7D
  [002400h]=44h    no effect (once caused crash with green rectangle)
  [002400h]=80h..8Fh  ;-Issue /RESET to SNES and map GAME 0..15
  [002400h]=C5h    causes always 8x7D
  [002400h]=FFh    sometimes maps GAME14 or GAME15? (unreliable)
```

### SNES Cart Nintendo Power - I/O Ports

**Nintendo Power I/O Map**

```
 Write registers:
  2400h        - Command
  2401h        - Extra parameter key (used only for wakeup command)
  2402h..2407h - Unknown/unused
 Read registers (before wakeup):
  2400h..2407h - Fixed 7Dh
 Read registers (after wakeup):
  2400h        - Fixed 2Ah
  2401h        - Status
  2402h..2403h - Fixed 2Ah
  2404h        - Mapping Info: ROM/RAM Size         ;\these four bytes are
  2405h..2406h - Mapping Info: SRAM Mapping related ; initialized from the
  2407h        - Mapping Info: ROM/RAM Base         ;/hidden flash sector
```

**Port 2401h = Status (R)**

```
  0-1 zero
  2   release /WP state    (set by CMD_02h, cleared by CMD_03h)
  3   disable ROM reading? (set by CMD_21h, cleared by CMD_20h)
  4-7 Selected Slot (0=Menu/File0, 1..15=File1..15) (via CMD_8xh)
```

**Port 2404h = Size (R)**

```
  0-1 SRAM Size (0=2K, 1=8K, 2=32K, 3=None) ;ie. 2K SHL (N*2)
  2-4 ROM Size (0=512K, 2=1.5M, 5=3M, 7=4M) ;ie. 512K*(N+1)
  5   Maybe ROM Size MSB for carts with three FLASH chips (set for HIROM:ALL)
  6-7 Mode (0=Lorom, 1=Hirom, 2=Forced HIROM:MENU, 3=Forced HIROM:ALL)
```

**Port 2407h = Base (R)**

```
  0-3 SRAM Base in 2K units
  4-7 ROM Base in 512K units (bit7 set for HIROM:MENU on skaman's blank cart)
```

**Port 2405h,2406h = SRAM Mapping Related (R)**

The values for port 2405h/2406h are always one of these three sets, apparently
related to SRAM mapping:

```
  29,4A for Lorom with SRAM
  61,A5 for Hirom with SRAM
  AA,AA for Lorom/Hirom without SRAM
  61,A5 (when forcing HIROM:ALL)
  D5,7F (when forcing HIROM:MENU)
  8A,8A (when forcing HIROM:MENU on skaman's blank cart)
```

Probably selecting which bank(s) SRAM is mapped/mirrored in the SNES memory
space.

**Nintendo Power I/O Ports**

The I/O ports at 002400h-002401h are used for mapping a selected game. Done as
follows:

```
  mov  [002400h],09h
  cmp  [002400h],7Dh
  jne  $  ;lockup if invalid
  mov  [002401h],28h
  mov  [002401h],84h
  mov  [002400h],06h
  mov  [002400h],39h
  mov  [002400h],80h+(Directory[n*2000h+0] AND 0Fh)
  jmp  $  ;lockup (until reset applies)
```

After the last write, the MX15001 chip maps the desired file, and does then
inject a /RESET pulse to the SNES console, which resets the CPU, APU (both SPC
and DSP), WRAM (address register), and any Expansion Port hardware (like
Satellaview), or piggyback cartridges (like Xband modem). The two PPU chips and
the CIC chip aren't affected by the /RESET signal. The overall effect is that
it boots the selected file via its Reset vector at [FFFCh].

### SNES Cart Nintendo Power - FLASH Commands

Before sending write/erase commands, one must initialize the MX15001 chip via
port 240xh (particulary: release the /WP pin), selecting the HIROM_ALL mapping
mode may be also recommended (for getting the whole 4Mbyte FLASH memory mapped
as continous memory block at address C00000h-FFFFFFh).

Observe that the cart contains two FLASH chips. In HIROM_ALL mode, one chip is
at C00000h-DFFFFFh, the other one at E00000h-FFFFFFh (ie. commands must be
either written to C0AAAAh/C05554h or E0AAAAh/E05554h, depending on which chip
is meant to be accessed; when programming large files that occupy both chips,
it would be fastest to program both chips simultaneously).

**FLASH Command Summary**

The FLASH chips are using more or less using standard FLASH commands, invoked
by writing to low-bytes at word-addresses 05555h and 02AAAh (aka writing bytes
to byte-addresses 0AAAAh and 05554h).

```
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=F0h, data=[addr..] ;Read/Reset
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=90h, ID=[00000h]   ;Get Maker ID
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=90h, ID=[00002h]   ;Get Device ID
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=90h, WP=[x0004h]   ;Get Sector Protect
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=70h, SRD=[00000h]  ;Read Status Reg
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=50h                ;Clear Status Reg
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=A0h, [addr..]=data ;Page/Byte Program
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=80h                ;Prepare Erase...
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=10h                ;...do Chip Erase
  [0AAAAh]=AAh, [05554h]=55h, [x0000h]=30h                ;...do Sector Erase
  [0xxxxh]=B0h                                            ;...Erase suspend
  [0xxxxh]=D0h                                            ;...Erase resume
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=60h                ;Prepare Protect...
  [0AAAAh]=AAh, [05554h]=55h, [addr]=20h                  ;...do Sector Protect
  [0AAAAh]=AAh, [05554h]=55h, [addr]=40h                  ;...do Sector Unprot.
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=C0h                ;Sleep
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=E0h                ;Abort
```

Undocumented commands for hidden sector:

```
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=77h                ;Prepare Hidden...
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=99h, [addr..]=data ;...do Hidden Write
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=E0h                ;...do Hidden Erase
  [00000h]=38h, [00000h]=D0h, [00000h]=71h, dummy=[00004] ;Prepare Hidden Rd...
  [00000h]=72h, [00000h]=75h, data=[addr...]              ;...do Hidden Read
```

**FLASH Read/Reset Command (F0h)**

Resets the chip to normal Read Data mode; this is required after most commands
(in order to resume normal operation; for leaving the Get Status, Get ID, or
Sleep states).

**FLASH Get Status (70h) and Clear Status (50h)**

Clear Status resets the error flags in bit4,5 (required because those bits
would otherwise stay set forever). Get Status switches to read-status mode
(this is usually not required because the erase/program/protect/sleep commands
are automatically entering read-status mode). The separate status bits are:

```
  7   Write/Erase State     (0=Busy, 1=Ready)
  6   Erase Suspend         (0=Normal, 1=Sector Erase Suspended)
  5   Erase Failure         (0=Okay, 1=Fail in erase)
  4   Program Failure       (0=Okay, 1=Fail in program)
  3   Reserved (zero)                           (MX29F1610A/B)
  3   Sector-Protect Status (0=?, 1=?)          (MX29F1611 only)
  2   Sleep Mode            (0=Normal, 1=Sleep) (MX29F1611 only)
  1-0 Reserved (zero)
```

**FLASH Get Maker/Device ID and Sector Protect Bytes (90h)**

Allows to read Maker/Device ID and/or Sector Protect Byte(s) from following
address(es):

```
 [00000h]=Manufacturer ID:
  C2h = Macronix
 [00002h]=Device ID:
  FAh = MX29F1610A  ;\with sector_protect, suspend/resume, without sleep/abort
  FBh = MX29F1610B  ;/
  F7h = MX29F1611   ;-with sector_protect, suspend/resume, sleep/abort
  6Bh = MX29F1615   ;-without sector_protect, suspend/resume, sleep/abort
  F3h = MX29F1601MC ;<-- undocumented, used in SNES nintendo power carts
 [x0004h]=Sector Protect State:
  00h = normal unprotected 128Kbyte sector (can occur on all sectors)
  C2h = write-protected 128Kbyte sector (can occur on first & last sector only)
```

**FLASH Erase: Prepare (80h), and Chip Erase (10h) or Sector Erase (30h)**

Allows to erase the whole 2Mbyte chip (ie. half of the Nintendo Power cart), or
a specific 128Kbyte sector.

Some MX29F16xx chips are also allowing to suspend (B0h) or resume (D0h) sector
erase (allowing to access other sectors during erase, if that should be
desired).

**FLASH Page/Byte Program (A0h)**

Allows to write one or more bytes (max 80h bytes) to a 128-byte page.

The Page/Byte Program command doesn't auto-erase the written page, so the
sector(s) should be manually erased prior to programming (otherwise the new
bytes will be ANDed with old data).

Caution: The chips in Nintendo Power carts require the LAST BYTE written TWICE
in order to start programming (unlike as in offical MX29F16xx specs, which
claim programmig to start automatically after not sending further bytes for
about 30..100us).

**FLASH Protect: Prepare (60h), and Protect (20h) or Unprotect (40h)**

Allows to write-protect or unprotect separate 128Kbyte sectors (this works only
for the first and last sector of each chip) (/WP=HIGH overrides the
protection).

**FLASH Sleep (C0h)**

Switches the chip to sleep state; can be resumed only via Read/Reset command
(F0h). Sleep mode is supported on MX29F1611 only.

**FLASH Abort (E0h)**

Aborts something. Supported on MX29F1611 only.

**Basic MX29F16xx specs**

JEDEC-standard EEPROM commands

Endurance: 100,000 cycles

Fast access time: 70/90/120ns

Sector erase architecture

- 16 equal sectors of 128k bytes each

- Sector erase time: 1.3s typical

Page program operation

- Internal address and data latches for 128 bytes/64 words per page

- Page programming time: 0.9ms typical

- Byte programming time: 7us in average

### SNES Cart Nintendo Power - Directory

**Directory Area**

```
  ROM Offset  SNES Address Size   Content
  060000h     8C8000h      2000h  File 0 (Menu)
  062000h     8CA000h      2000h  File 1
  064000h     8CC000h      2000h  File 2
  066000h     8CE000h      2000h  File 3
  068000h     8D8000h      2000h  File 4
  06A000h     8DA000h      2000h  File 5
  06C000h     8DC000h      2000h  File 6
  06E000h     8DE000h      2000h  File 7
  070000h     8E8000h      10000h Unused (FFh-filled)
```

The last 64Kbyte are probably usable as further file entries in cartridges
bigger than 4Mbyte (the Menu software in the existing cartridges is hardcoded
to process only files 1..7) (whilst Port 2400h seems to accept 4bit file
numbers).

**Directory Entry Format**

```
  0000h 1    Directory index (00h..07h for Entry 0..7) (or FFh=Unused Entry)
  0001h 1    First 512K-FLASH block (00h..07h for block 0..7)
  0002h 1    First 2K-SRAM block    (00h..0Fh for block 0..15)
  0003h 2    Number of 512K-FLASH blocks (mul 4) (=0004h..001Ch for 1..7 blks)
  0005h 2    Number of 2K-SRAM blocks (mul 16)   (=0000h..0100h for 0..16 blks)
  0007h 12   Gamecode (eg. "SHVC-MENU-  ", "SHVC-AGPJ-  ", or "SHVC-CS  -  ")
  0013h 44   Title in Shift-JIS format (padded with 00h's) (not used by Menu)
  003Fh 384  Title Bitmap (192x12 pixels, in 30h*8 bytes, ie. 180h bytes)
  01BFh 10   Date "MM/DD/YYYY" (or "YYYY/MM/DD" on "NINnnnnn" carts)
  01C9h 8    Time "HH:MM:SS"
  01D1h 8    Law  "LAWnnnnn" or "NINnnnnn" (eg. "LAW01712", or "NIN11001")
  01D9h 7703 Unused (1E17h bytes, FFh-filled)
  1FF0h 16   For File0: "MULTICASSETTE 32" / For Files 1-7: Unused (FFh-filled)
```

**Directory Index**

Directory Index indicates if the Entry is used (FFh=unused). If it is used,
then it must be equal to the current directory entry number (ie. a rather
redundant thing, where the index is indexing itself). The lower 4bit of the
index value is used for game selection via Port 2400h.

**First FLASH/SRAM Block**

The First FLASH block number is stored in lower some bits of [0001h].

The First SRAM block number is stored in lower some bits of [0002h].

The directory doesn't contain any flag that indicates HiROM or LoROM mapping.

There is no support for fragmented FLASH/SRAM files (ie. the programming
station must erase & rewrite the entire cartridge, with the old used/unused
blocks re-ordered so that they do form continous memory blocks).

**Number of FLASH/SRAM Blocks (displayed in Menu)**

These entries are used to display the amount of used/unused blocks. Free FLASH
blocks are shown as blue "F" symbols, free SRAM blocks as red "B" symbols, used
blocks as gray "F" and "B" symbols. Pressing the X-button in the menu indicates
which blocks are being used by the currently selected game.

**Title Bitmap (displayed in Menu)**

The 192x12 pixel title bitmap is divided into eight 24x12 pixel sections, using
a most bizarre encoding: Each section is 30h bytes in size (enough for 32 pixel
width, but of these 32 pixels, the "middle" 4 pixels are overlapping each
other, and the "right-most" 4 pixels are unused. The byte/pixel order for 12
rows (y=0..11) is:

```
  Left 8 pixels   = (Byte[00h+y*2])
  Middle 8 pixels = (Byte[01h+y*2]) OR (Byte[18h+y*2] SHR 4)
  Right 8 pixels  = (Byte[18h+y*2] SHL 4) OR (Byte[19h+y*2] SHR 4)
```

The result is displayed as a normal 192-pixel bitmap (without any spacing
between the 24-pixel sections). The bits in the separate bytes are bit7=left,
bit0=right. Color depth is 1bit (0=dark/background, 1=bright/text). The bitmap
does usually contain japanese text without "special" features, though it could
also be used for small icons, symbols, bold text, greek text, etc.

**Text Fields (not used by Menu)**

The Shift-JIS Title, and ASCII Game Code, Date, Time, Law & Multicassette
strings aren't used by the Menu. The 5-digit Law number is usually (but not
always) same for all files on the cartridge, supposedly indicating the station
that has programmed the file.

```
  LAW = games installed on kiosks in Lawson Convenience Store chain
  NIN = games pre-installed by nintendo (eg. derby 98)
```

The Multicassette number does probably indicate the FLASH size in MBits (it's
always 32 for the existing 32Mbit/4MByte cartridges).

### SNES Cart Sufami Turbo (Mini Cartridge Adaptor)

The Sufami Turbo from Bandai is an adaptor for low-cost mini-cartridges. Aside
from cost-reduction, one special feature is that one can connect two cartridges
at once (so two games could share ROM or SRAM data). The BIOS in the adaptor
provides a huge character set, which may allow to reduce ROM size of the games.

### SNES Cart Sufami Turbo General Notes

**Sufami Turbo Hardware**

The "adaptor" connects to the SNES cartridge socket, it contains the BIOS ROM,
and two slots for "mini-carts". Slot A for the game being played, Slot B can
contain another game (some games include features that allow to access game
position data from other games, some may also access ROM data from other
games).

**Sufami Turbo Memory Map**

```
  00-1F:8000h-FFFFh  BIOS ROM (always 256Kbytes)             (max 1MByte)
  20-3F:8000h-FFFFh  Cartridge A ROM (usually 512Kbytes)     (max 1MByte)
  40-5F:8000h-FFFFh  Cartridge B ROM (usually 512Kbytes)     (max 1MByte)
  60-63:8000h-FFFFh  Cartridge A SRAM (usually 0/2/8 Kbytes) (max 128Kbyte)
  70-73:8000h-FFFFh  Cartridge B SRAM (usually 0/2/8 Kbytes) (max 128Kbyte)
  80-FF:8000h-FFFFh  Mirror of above banks
```

**Memory Notes**

The BIOS detects max 128Kbyte (64 pages) SRAM per slot, some games are (maybe
accidently) exceeding that limit (eg. Poi Poi Ninja zerofills 256 pages). Some
games (eg. SD Gundam Part 1) access SRAM slot B at 700000h rather than 708000h.

Some games (eg. SD Ultra Battle) may fail if the SRAM in slot B is
uninitialized (ie. before linking games in Slot A and B, first launch them
separately in Slot A).

When not using BIOS functions, one can safely destroy all WRAM locations,
except for WRAM[00000h] (which MUST be nonzero to enable the Game NMI handler
& disable the BIOS NMI handler).

**Sufami Turbo ROM Images**

The games are typically 512Kbyte or 1MByte in size. Existing ROM-Images are
often 1.5Mbytes or 2MBytes - those files do include the 256KByte BIOS-ROM
(banks 00h-07h), plus three mirrors of the BIOS (banks 08h-1Fh), followed by
the actual 512Kbyte or 1MByte Game ROM (bank 20h-2Fh or 20h-3Fh).

There are also a few 3MByte ROM-images, with additional mirrors of the game
(bank 30h-3Fh), followed by a second game (bank 40h-4Fh), followed by mirrors
of the second game (bank 50h-5Fh).

That formats are simple (but very bloated) solutions to load the BIOS &
Game(s) as a "normal" LoROM file.

**Sufami Turbo Games**

There have been only 13 games released:

```
  Crayon Shin Chan
  Gegege No Kitarou
  Gekisou Sentai Car Ranger
  Poi Poi Ninja                    ;-link-able with itself (2-player sram)
  Sailor Moon Stars Panic 2
  SD Gundam Generations: part 1    ;\
  SD Gundam Generations: part 2    ;
  SD Gundam Generations: part 3    ; link-able with each other
  SD Gundam Generations: part 4    ;
  SD Gundam Generations: part 5    ;
  SD Gundam Generations: part 6    ;/
  SD Ultra Battle: Seven Legend    ;\link-able with each other
  SD Ultra Battle: Ultraman Legend ;/
```

All of them available only in Japan, released between June & September
1996. Thereafter, the games may have been kept available for a while, but
altogether, it doesn't seem to have been a too successful product.

**Component List for Sufami Turbo Adaptor**

PCB "SHVC TURBO, BASE CASSETTE, BANDAI, PT-923"

```
  IC1   18pin  unknown (CIC)
  IC2   16pin  "74AC139" or so
  IC3   40pin  SUFAMI TURBO "LH5326NJ" or so (BIOS ROM) (256Kbyte)
  IC4   8pin   unknown
  CP1   unknown (flashlight? oscillator? strange capacitor?)
  CN1   62pin  SNES cartridge edge (male)
  CN2   40pin  Sufami Cartridge Slot A (Game to be played)
  CN3   40pin  Sufami Cartridge Slot B (Other game to be "linked")
  C1..4  2pin  capacitors for IC1..4
  R1..4  2pin  resistors for unknown purpose
```

Note: Of the 62pin cartridge edge, only 43 pins are actually connected (the
middle 46 pins, excluding Pin 40,48,57, aka A15/A23/SYSCK).

**Component Lists for Sufami Turbo Game Carts**

All unknown. Probably contains only ROM, and (optionally) SRAM and Battery.
Physical SRAM size(s) are unknown (ie. unknown if there is enough memory for
more than one file). Cartridge slot pin-outs are unknown.

### SNES Cart Sufami Turbo ROM/RAM Headers

**Sufami Turbo BIOS ROM Header**

The BIOS has a rather incomplete Nintendo-like header at ROM Offset 07FB0h
(mapped to 00FFB0h):

```
  FFB0h Maker Code "B2"                        ;\extended header, present
  FFB2h Game Code "A9PJ"                       ; even though [FFDAh]<>33h
  FFB6h Reserved (10x00h)                      ;/
  FFC0h Title "ADD-ON BASE CASSETE  " (really mis-spelled, with only one "T")
  FFD4h Mapmode (always 30h = Fast LoROM)
  FFD5h Reserved (6x00h) (no ROM/RAM size entries, no ext.header-flag, etc.)
  FFDCh Dummy "checksum" value (always FFh,FFh,00h,00h)
  FFE0h Exception Vectors (IRQ,NMI,Entrypoint,etc.)
```

And, there is a header-like data field at ROM-Offset 00000h (mapped to 808000h)
(this part isn't really a header, but rather contains ID strings that are used
by the BIOS, for comparing them with Game ROM/SRAM):

```
  8000h 16 "BANDAI SFC-ADX",0,0   ;Game ROM ID
  8010h 16 "SFC-ADX BACKUP",0,0   ;Game SRAM ID
```

**Sufami Turbo Game ROM Header (40h bytes)**

Located at ROM Offset 00000h (mapped to 208000h/408000h for Slot A/B):

```
  00h 14 ID "BANDAI SFC-ADX" (required, compared against 14-byte ID in BIOS)
  0Eh 2  Zero-filled
  10h 14 Title, padded with spaces (can be 7bit ASCII and 8bit Japanese)
  1Eh 2  Zero-filled
  20h 2  Entrypoint (in bank 20h) ;game starts here (if it is in Slot A)
  22h 2  NMI Vector (in bank 20h) ;if RAM[000000h]=00h: use BIOS NMI handler
  24h 2  IRQ Vector (in bank 20h)
  26h 2  COP Vector (in bank 20h)
  28h 2  BRK Vector (in bank 20h)
  2Ah 2  ABT Vector (in bank 20h)
  2Ch 4  Zero-filled
  30h 3  Unique 24bit ID of a Game (or series of games) (usually 0xh,00h,0yh)
  33h 1  Index within a series (01h and up) (eg. 01h..06h for Gundam 1-6)
  34h 1  ROM Speed (00h=Slow/2.68Mhz, 01h=Fast=3.58MHz)
  35h 1  Chipset/Features (00h=Simple, 01h=SRAM or Linkable?, 03h=Special?)
  36h 1  ROM Size in 128Kbyte Units (04h=512K, 08h=1024K)
  37h 1  SRAM Size in 2Kbyte Units (00h=None, 01h=2K, 04h=8K)
  38h 8  Zero-filled
```

Some games have additional 64 header-like bytes at ROM Offset 40h..7Fh

```
  40h 1  Program code/data in some carts, 00h or 01h in other carts
  41h 63 Program code/data in some carts, 00h-filled in other carts
```

The game cartridges don't use/need a Nintendo-like header at 7Fxxh/FFxxh, but
some games like SDBATTLE SEVEN do have one.

**Sufami Turbo SRAM File Header (30h bytes)**

```
  0000h 15 ID "SFC-ADX BACKUP",0   ;Other = begin of free memory
  000Fh 1  Zero
  0010h 14 Title (same as 0010h..001Dh in ROM Header)
  001Eh 1  Zero
  001Fh 1  Zero (except, 01h in Poi Poi Ninja)
  0020h 4  Unique ID and Index in Series (same as 0030h..0033h in ROM Header)
  0024h 1  Filesize (in 2Kbyte units)    (same as 0037h in ROM Header)
  0025h 11 Zero-filled
```

The BIOS file-functions are only reading entry 0000h (ID) and 0024h (Filesize),
the BIOS doesn't write anything, all IDs and values must be filled-in by the
game.

SRAM is organized so that used 2Kbyte pages are at lower addresses, free pages
at higher addresses (deleting a file in the middle will relocate any pages at
higher addresses). Accordingly, files are always consisting of unfragmented
continous page numbers (leaving apart that there are 32Kbyte gaps in the memory
map).

### SNES Cart Sufami Turbo BIOS Functions & Charset

**Sufami Turbo BIOS Function Summary**

BIOS Function vectors (jmp 80xxxxh opcodes) are located at 80FF00h..80FF3Bh,

(the first 12 (of the 15) functions are also duplicated at 80FF80h..80FFAFh).

```
  80FF00  FillSramPages  ;in: AL=num, AH=slot, XL=first, [09h]=fillword
  80FF04  CopySramToSram ;in: AL=num, AH=direction, X/Y=first (slot A/B)
  80FF08  CopySramToWram ;in: AL=num, AH=direction, X=first, Y=slot, [09h]=addr
  80FF0C  GetChar2bpp    ;in: A=char(0000h..0FFFh), [06h]=dest_addr (64 bytes)
  80FF10  GetChar4bpp    ;in: A=char(0000h..0FFFh), [06h]=dest_addr (128 bytes)
  80FF14  GetCartType    ;out: AL/AH=Types for Slot A/B, b0=ROM, b1=SRAM, b2=?
  80FF18  GetSramSize    ;out: AL/AH=Sizes for Slot A/B, 0-4=0,2,8,32,128Kbyte
  80FF1C  FindFreeSram   ;in: AL=slot, out: AL=first_free_page, FFh=none
  80FF20  GetSramAddrTo6 ;in: AL=slot, XL=page, out: [06h]=addr
  80FF24  GetSramAddrTo9 ;in: AL=slot, XL=page, out: [09h]=addr
  80FF28  ShowHelpSwap   ;display instructions how to exchange cartridges
  80FF2C  ShowHelpNoSwap ;display instructions how to remove cartridges
  80FF30  DeleteFile     ;in: AL=first, AH=slot
  80FF34  TestSramId     ;in: AL=page, AH=slot, out: CY: 0=Used, 1=Free
  80FF38  SramToSramCopy ;in: AL=num, X=src, Y=dst; XH/YH=slot, XL/YL=first
```

Whereas,

```
  num = number of 2Kbyte pages
  slot = slot (0 or 1 for slot A or B)
  first = first 2Kbyte page number (of a file/area) (within selected slot)
  page = single 2Kbyte page number (within selected slot)
  addr = 24bit SNES memory address
  AL/AH, XL/XH, YL/YH = LSB/MSB of A,X,Y registers
```

The BIOS functions use first 16 bytes in WRAM [0000h..000Fh] for parameters,
return values, and internal workspace; when using BIOS functions, don't use
that memory for other purposes. [0000h] is NMI mode, don't change that even
when NOT using BIOS functions.

**File/SRAM Functions**

These functions may be (not very) helpful for managing SRAM, they are extremly
incomplete, there are no functions for creating files, or for searching
specific files. See the "Header" chapter for details on SRAM headers (again,
the BIOS doesn't create any headers or IDs, the game must fill-in all IDs,
Titles, and other values on its own).

**Character Set**

The BIOS ROM contains 4096 characters (each 16x16 pixel, aka 2x2 tiles). The
characters are stored at 1bit color depth in banks 04h..07h, offset 8000h-FFFFh
(20h bytes/character). The GetChar2bpp and GetChat4bpp functions can be used to
copy a selected character to WRAM, with bits in plane0, and the other plane(s)
zerofilled.

**Help Functions**

The two help functions are showing some endless repeated japanese instructions
about how to use, insert, remove, and exchange cartridges (similar to the
instructions shown when booting the BIOS without Game cartridges inserted). If
you have uploaded code to the APU, be sure to return control to the APU
boot-rom, otherwise the help functions will hang.

### SNES Cart X-Band (2400 baud Modem)

The X-Band is a 2400 baud modem from Catapult Entertainment Inc., licensed by
Nintendo, originally released 1994 in USA, and 199x? in Japan. Aside from the
SNES version, there have been also Genesis and Saturn versions.

**Note**

There's also another modem (which connects to controller port):

### SNES Cart X-Band Misc

**Info...**

It was used for networked gaming via phone lines.

The Xband worked by sending controller instructions, by intercepting code from
the game, and patching it with its own instructions, much like the Game Genie
works. (that are, probably, two separate features messed into one sentence?)

The system worked by dialing up the main server, which was located in
Cupertino, California (USA), and somewhere else (Japan). The server then sent
the Xband newsletters (called Bandwidth and Xband News). It also sent any
patches that were needed. You could then search for opponents.

**Unknown Features**

There seems to be no CIC chip, so the BIOS does likewise work only with another
SNES cart connected.

There is switch, for whatever on/off/mode selection. There are three LEDs for
whatever purpose. And, there is some kind of a credit-card (or so) reader.

**Memory Map**

```
  D00000h-DFFFFFh  1MB ROM (executed here, not at C00000h-CFFFFFh)
  E00000h-E0FFFFh  64K SRAM (in two 32Kx8 chips) (unknown if BOTH have battery)
  FBC000h-FBC17Fh  I/O Ports (unknown functions?)
  FBC180h-FBC1BFh  I/O Ports (Rockwell Modem Chip)
  FBFC02h          I/O Port  (unknown functions?)
  FBFE00h          I/O Port  (unknown functions?)
  FFC000h          I/O Port  (unknown functions?)
  004F02h          I/O Port  (unknown functions?)
  00F000h          Dummy/strobe read?
  00FFE0h          Dummy/strobe read?
```

I/O Ports seem to be 8bit-wide / word-aligned (ie. one can use 8bit or 16bit
writes, with the MSB ignored in the latter case). Normally ONLY the even
addresses are used (some exceptions are: 8bit write 00h to FBC153h, 16bit write
0000h to FBC160h).

Some of the I/O ports outside of the FBCxxxh region might belong to other
hardware? (eg. the X-Band might automatically disable any Game Genie BIOS in
order to access the Game ROM).

**Unknown 100pin Chip**

Unknown. Probably controls the cart reader, the cheat/patching feature, and
maybe also memory & I/O mapping of the other chips.

**Games supported by the X-Band modem**

```
  Doom                           +
  Ken Griffey Jr. Baseball       ? (not listed in stats)
  Killer Instinct                +
  Madden NFL '95                 +
  Madden NFL '96                 +
  Mortal Kombat II               +
  Mortal Kombat 3                +
  NBA Jam TE                     +
  NHL '95                        ? (not listed in stats)
  NHL '96                        ? (not listed in stats)
  Super Mario Kart               +
  Weaponlord                     + (listed in sf2dxb stats only)
```

and,

```
  Kirby's Avalanche              +
  Super Street Fighter II        +
  The Legend of Zelda: A Link to the Past (secret maze game)   +
  Super Mario World (chat function)
```

"First of all, the Legend of Zelda wasn't the only cartridge that would
activate the hidden maze game -- basically, any unsupported SNES cart would do
it. I usually used Super Mario World."

CZroe: "Zelda triggered the XBAND's built-in maze game (someone reported that
their copy didn't work... Zelda 1.1?!). Mario World triggered the Chat
function."

CZroe: "This is how I identified that there was a second version of Killer
Instinct long before it debuted on this site (all US Killer Instinct bundle

SNES consoles would not work with the XBAND)."

gainesvillefrank: "I remember XBAND tried this experimental use of Mario World
after a while. If you dialed in to XBAND with Mario World in your SNES then it
would treat the cartridge as a chat room."

"The black switch on the side needs to be in the down position. Otherwise it
passes through."

Most of the above games, don't include any built-in Xband support, instead,
Catapult reverse-engineered how they work, and patched them to work with the
modem. Exceptions are Weaponlord (and Doom?), which were released with "modem
support" (unknown what that means exactly... do they control modem I/O ports...
interact with the modem BIOS... or are they patched the same way as other
games, and the only difference is that the developers created the patches
before releasing the game?)

Note: The japanese BIOS does read the Game cartridge header several times
(unlike the US version which reads it only once), basically there is no good
reason for those multiple reads, but it might indicate the japanese version
includes multiple patches built-in in ROM?)

**CODES/SECRETS (still working, even when offline)**

Maze mini-game

Press Down(2), Left(2), Right, B at the main menu.

Blockade mini-game (tron clone)

Press Up(2), Left, Right, Left(2), Right, L at the main menu.

Fish Pong mini-game

Genesis only?

Change Font

To change the text font, enter these codes at the Player Select screen.

Green and yellow font - Up, Up, Right, Right, Down, Down, Left

Rainbow font - Left, Left, Up, Up, Right, Right, Down

Searchlight font - Down, Down, Left, Left, Up, Up, Right

Alternate screen

Press Up, Up, Left, Right on the title screen.

Screen Saver

Press Left, Right, Down, Down, R at the "X-Mail" and "Newsletters" screens.

**SNES X-Band SRAM Dumps**

```
  benner  3.26.97 (main character with most stats is lower-right)
  sf2dxb  4.30.97
  luke2   3.1.97
```

contains stats (for played game titles; separately for each of the 4 player
accounts), and the most recent bandwidth/newletter magazines, and x-mails.

**PCB "123-0002-16, Cyclone Rev 9, Catapult (C) 1995" Component List**

```
  U1   28pin Winbond W24257S-70L                    (32Kx8 SRAM)
  U2   36pin X X, X BAND, X X, SNES US ROM 1.0.1    (BIOS ROM)
  U3  100pin FredIIH, H3A4D1049, 9511 Korea (with Hyundai logo)
  U4   68pin RC2324DPL, R6642-14, Rockwell 91, 9439 A49172-2, Mexico
  U5    6pin LITEON 4N25 (optocoupler) (near TN0) (back side)
  U6   28pin Winbond W24257S-70L                    (32Kx8 SRAM)
  U7    6pin AT&T LF1504 (solid state relay) (near TN0) (back side)
  BT0   2pin Battery (not installed) (component side)
  BT200 2pin Battery (3V Lithium Penata CR2430) (back side)
  SW1   3pin Two-position switch (purpose unknown... battery off ??)
  J0   10pin Card-reader (for credit cards or so?) 8 contacts, plus 2pin switch
  J1   62pin SNES Cartridge Edge (to be plugged into the SNES console)
  J2   62pin SNES Cartridge Slot (for game-cart plugged on top of the modem)
  J3  4/6pin RJ socket (to phone line)
  Y1    2pin Oscillator (R24AKBB4, =24MHz or so?) (back side)
  TN0   4pin Transformator (671-8001 MIDCOM C439)
  LEDs       Three red LEDs (purpose/usage unknown?)
```

**PCB "123-0002-17, Catapult (C) 1995" Component List**

```
  MODEM is  "RC2424DPL, R6642-25, Rockwell 91, 9507 A61877.2, Hong Kong"
```

**PCB "123-0003-04, Tornado, Catapult (C) 1995" (Japan)**

```
  SRAMs are "SEC KOREA, 550A, KM62256CLG-7L"
  BIOS  is  "X X 9549, X BAND, X X, SUPER FAMICOM, ROM1.0"
  FRED  is  "Catapult, FRED5S, 549D" (100pin)
  MODEM is  "RC2424DPL, R6642-25, Rockwell 91, 9609 A62975-2, Mexico"
  Y1    is  "A24.000"
  BT201 is  "C?2032" (installed instead of bigger BT200)
```

### SNES Cart X-Band I/O Map

Below I/O Map is based on source code of the Sega Genesis X-Band version (files
i\harddef.a and i\feq.a). The I/O Map of the SNES version might differ in some
places.

default base addresses

```
  kDefaultInternal:   equ     ($1de000*2)     ;;=3BC000h   ;aka SNES: FBC000h
  kDefaultControl:    equ     ($1dff00*2)     ;;=3BFE00h   ;aka SNES: FBFE00h
```

**X-Band I/O Map**

```
  Addr  $nn*2 i\harddef.a      i\feq.a     ;Comment
  ----------------------------------------------------------------------------
  C000h $00*2 kPatch_0_Byte0   -   (lo?)   ;Translation (Patch Addr) regs ...
  C002h $01*2 kPatch_0_Byte1   -   (mid?)  ;(aka "Vectors 0..10"?)
  C004h $02*2 kPatch_0_Byte2   -   (hi?)
  C006h       N/A              -
  C008h $04*2 kPatch_1_Byte0   -
  C00Ah $05*2 kPatch_1_Byte1   -
  C00Ch $06*2 kPatch_1_Byte2   -
  C00Eh       N/A              -
  C010h $08*2 kPatch_2_Byte0   -
  C012h $09*2 kPatch_2_Byte1   -
  C014h $0A*2 kPatch_2_Byte2   -
  C016h       N/A              -
  C018h $0C*2 kPatch_3_Byte0   -
  C01Ah $0D*2 kPatch_3_Byte1   -
  C01Ch $0E*2 kPatch_3_Byte2   -
  C01Eh       N/A              -
  C020h $10*2 kPatch_4_Byte0   -
  C022h $11*2 kPatch_4_Byte1   -
  C024h $12*2 kPatch_4_Byte2   -
  C026h       N/A              -
  C028h $14*2 kPatch_5_Byte0   -
  C02Ah $15*2 kPatch_5_Byte1   -
  C02Ch $16*2 kPatch_5_Byte2   -
  C02Eh       N/A              -
  C030h $18*2 kPatch_6_Byte0   -
  C032h $19*2 kPatch_6_Byte1   -
  C034h $1A*2 kPatch_6_Byte2   -
  C036h       N/A              -
  C038h $1C*2 kPatch_7_Byte0   -
  C03Ah $1D*2 kPatch_7_Byte1   -
  C03Ch $1E*2 kPatch_7_Byte2   -
  C03Eh       N/A              -
  C040h $20*2 kPatch_8_Byte0   -
  C042h $21*2 kPatch_8_Byte1   -
  C044h $22*2 kPatch_8_Byte2   -
  C046h       N/A              -
  C048h $24*2 kPatch_9_Byte0   -
  C04Ah $25*2 kPatch_9_Byte1   -
  C04Ch $26*2 kPatch_9_Byte2   -
  C04Eh       N/A              -
  C050h $28*2 kPatch_10_Byte0  -
  C052h $29*2 kPatch_10_Byte1  -
  C054h $2A*2 kPatch_10_Byte2  -
  C056h       N/A              -
  C058h $2C*2 kRange0Start     -
  C05Ah         ""-mid?        -
  C05Ch         ""-hi?         -
  C05Eh       N/A              -
  C060h $30*2 kRange1Start     -
  C062h         ""-mid?        -
  C064h         ""-hi?         -
  C066h       N/A              -
  C068h       N/A              -
  C06Ah       N/A              -
  C06Ch       N/A              -
  C06Eh       N/A              -
  C070h $38*2 kMagicAddrByte0  kmagicl
  C072h $39*2 kMagicAddrByte1  kmagicm
  C074h $3A*2 kMagicAddrByte2  kmagich
  C076h       N/A              -
  C078h       N/A              -
  C07Ah       N/A              -
  C07Ch       N/A              -
  C07Eh       N/A              -
  C080h $40*2 kRange0End       krangel
  C082h         ""-mid?        krangem
  C084h         ""-hi?         krangeh
  C086h       N/A              -
  C088h $44*2 kRange1End       -
  C08Ah         ""-mid?        -
  C08Ch         ""-hi?         -
  C08Eh       N/A              -
  C090h       N/A              -
  C092h       N/A              -
  C094h       N/A              -
  C096h       N/A              -
  C098h       N/A              -
  C09Ah       N/A              -
  C09Ch       N/A              -
  C09Eh       N/A              -
  C0A0h $50*2 kRange0Dest      ktrbl
  C0A2h         ""-hi?         ktrbh
  C0A4h $52*2 kRange0Mask      ktrm
  C0A6h       N/A              -
  C0A8h $54*2 kRange1Dest      -
  C0AAh         ""-hi?         -
  C0ACh $56*2 kRange1Mask      -
  C0AEh       N/A              -
  C0B0h       N/A              -
  C0B2h       N/A              -
  C0B4h       N/A              -
  C0B6h       N/A              -
  C0B8h       N/A              -
  C0BAh       N/A              -
  C0BCh       N/A              -
  C0BEh       N/A              -
  C0C0h $60*2 kRAMBaseByte0    ksaferambasel
  C0C2h $61*2 kRAMBaseByte1    ksaferambaseh
  C0C4h       N/A              -
  C0C6h       N/A              -
  C0C8h $64*2 kRAMBoundByte0   ksaferambndl
  C0CAh $65*2 kRAMBoundByte1   ksaferambndh
  C0CCh       N/A              -
  C0CEh       N/A              -
  C0D0h $68*2 kVTableBaseByte0 kvtablel ;\vector table base address?
  C0D2h $69*2 kVTableBaseByte1 kvtableh ;/ (in 32-byte, or 32-word steps maybe?)
  C0D4h       N/A              -
  C0D6h       N/A              -
  C0D8h $6c*2 kEnableByte0     kenbll
  C0DAh $6d*2 kEnableByte1     kenblh
  C0DCh       N/A              -
  C0DEh       N/A              -
  C0E0h $70*2 kROMBound        ksaferombnd
  C0E2h       N/A              -
  C0E4h       N/A              -
  C0E6h       N/A              -
  C0E8h $74*2 kROMBase         ksaferombase
  C0EAh       N/A              -
  C0ECh       N/A              -
  C0EEh       N/A              -
  C0F0h       N/A              -              ;<-- but this is used on SNES !?!
  C0F2h       N/A              -              ;<-- but this is used on SNES !?!
  C0F4h       N/A              -
  C0F6h       N/A              -
  C0F8h $7c*2 kAddrStatus      kaddrstatusl
  C0FAh         ""-hi?         kaddrstatush
  C0FCh       N/A              -
  C0FEh       N/A              -
  C100h $80*2 kSControl        ksctl                  ;smart card control
  C102h       N/A              -
  C104h       N/A              -
  C106h       N/A              -
  C108h $84*2 kSStatus         ksstatus               ;smart card status
  C10Ah       N/A              -
  C10Ch       N/A              -
  C10Eh       N/A              -
  C110h $88*2 kReadMVSyncLow   kreadmvsync    ;<--Low? ;\Range of 0 to $61.
  C112h $89*2 kReadMVSyncHigh  kreadmvsynclow ;<--low? ;/Equal to
  C114h       N/A              -                       ; ReadSerialVCnt/2.
  C116h       N/A              -                       ; Value is $5c at start
  C118h $8c*2 kMStatus1        kmstatus1               ; of VBlank.
  C11Ah       N/A              -
  C11Ch       N/A              -
  C11Eh       N/A              -
  C120h $90*2 kTxBuff          ktxbuff            ; modem (and serial) bits ...
  C122h       N/A              -
  C124h       N/A              -
  C126h       N/A              -
  C128h $94*2 kRxBuff          krxbuff
  C12Ah       N/A              -
  C12Ch       N/A              -
  C12Eh       N/A              -
  C130h $98*2 kReadMStatus2    kreadmstatus2
  C132h       N/A              -
  C134h       N/A              -
  C136h       N/A              -
  C138h $9c*2 kReadSerialVCnt  kreadserialvcnt
  C13Ah       N/A              -
  C13Ch       N/A              -
  C13Eh       N/A              -
  C140h $a0*2 kReadMStatus1    kreadmstatus1
  C142h       N/A              -
  C144h       N/A              -
  C146h       N/A              -
  C148h $a4*2 kGuard           kguard
  C14Ah       N/A              -
  C14Ch       N/A              -
  C14Eh       N/A              -
  C150h $a8*2 kBCnt            kbcnt
  C152h       N/A              -
  C154h       N/A              -
  C156h       N/A              -
  C158h $ac*2 kMStatus2        kmstatus2
  C15Ah       N/A              -
  C15Ch       N/A              -
  C15Eh       N/A              -
  C160h $b0*2 kVSyncWrite      kvsyncwrite
  C162h       N/A              -
  C164h       N/A              -
  C166h       N/A              -
  C168h $b4*2 kLEDData         kleddata
  C16Ah $b5*2 kLEDEnable       kledenable
  C16Ch       N/A              -
  C16Eh       N/A              -
  C170h       N/A              -
  C172h       N/A              -
  C174h       N/A              -
  C176h       N/A              -
  C178h       N/A              -
  C17Ah       N/A              -
  C17Ch       N/A              -
  C17Eh       N/A              -
  C180h $c0*2 kModem           - ;<-- base for rockwell registers (C180h-C1BEh)
```

```
  FC02h       N/A              -      ;<-- unknown, but this is used by SNES
  FE00h $00*2 kKillReg         kkillhere ;same as killheresoft...trans register
  FE02h $01*2 kControlReg      kreghere
  FF80h $c0*2 kKillHereSoft    kkillheresoft  ;\maybe some sort of mirrors of
  FF82h $c1*2 kCtlRegSoft      kctlregsoft    ;/FE00h and FE02h ?
```

```
  617000h ? weirdness kSNESKillHereSoft       ;\maybe some sort of mirrors of
  617001h ? weirdness kSNESCtlRegSoft         ;/FE00h and FE02h ?
```

```
  FFC000h          I/O Port  (unknown functions?) ;-bank FFh ;\
  004F02h          I/O Port  (unknown functions?) ;\         ; whatever, used
  00F000h          Dummy/strobe read?             ; bank 00h ; by SNES version
  00FFE0h          Dummy/strobe read?             ;/         ;/
```

### SNES Cart X-Band I/O - Memory Patch/Mapping

**FE00h - KillReg (aka killhere) ;same as killheresoft...trans register**

;kill register bits  ;aka "kKillReg" and/or "kKillHereSoft"?

```
  0   HereAssert:      equ     $01 ; "Here" = cannot see cart
  1   Unknown/unused
  2   DecExcept:       equ     $04 ;
  3   Force:           equ     $08 ;
  4-7 Unknown/unused
```

**FE02h - ControlReg (aka reghere)**

;control bits for control register ;aka "kControlReg"? and/or "kCtlRegSoft"?

```
  0   EnTwoRam:        equ     $01 ;<-- maybe disable one of the two SRAMs?
  1   EnSafeRom:       equ     $02 ;<-- maybe SRAM read-only? or FlashROM?
  2   RomHi:           equ     $04 ;
  3   EnInternal:      equ     $08 ;<-- maybe disable ports C000h..C1FFh?
  4   EnFixedInternal: equ     $10 ;<-- maybe whatever related to above?
  5   EnSNESExcept:    equ     $20 ;
  6-7 Unknown/unused
```

**FF80h - KillHereSoft (aka kkillheresoft)**

**FF82h - CtlRegSoft (aka kctlregsoft)**

Unknown, maybe some sort of mirrors of FE00h and FE02h ?

**617000h ? weirdness kSNESKillHereSoft**

**617001h ? weirdness kSNESCtlRegSoft**

Unknown, maybe some sort of mirrors of FE00h and FE02h ?

Maybe non-Sega, SNES only stuff? Or maybe weird/ancient prototype stuff?

**C000h/C002h/C004h - Patch 0, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C008h/C00Ah/C00Ch - Patch 1, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C010h/C012h/C014h - Patch 2, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C018h/C01Ah/C01Ch - Patch 3, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C020h/C022h/C024h - Patch 4, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C028h/C02Ah/C02Ch - Patch 5, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C030h/C032h/C034h - Patch 6, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C038h/C03Ah/C03Ch - Patch 7, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C040h/C042h/C044h - Patch 8, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C048h/C04Ah/C04Ch - Patch 9, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

**C050h/C052h/C054h - Patch 10, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)**

aka "Vectors 0..10"?

**C070h/C072h/C074h - MagicAddr Byte0/1/2 (Lo/mid/hi) (aka magicl/m/h)**

```
  0-23 Unknown (also referred to as "transition address"?)
```

**C058h/C05Ah/C05Ch - Range0Start (Lo/mid/hi?)**

**C060h/C062h/C064h - Range1Start (Lo/mid/hi?)**

**C080h/C082h/C084h - Range0End (Lo/mid/hi) (aka rangel/m/h)**

**C088h/C08Ah/C08Ch - Range1End (Lo/mid/hi?)**

```
  0-23 Unknown (maybe ROM start/end addresses for BIGGER patch regions?)
```

**C0A0h/C0A2h - Range0Dest (Lo/hi) (aka trbl/h)**

**C0A8h/C0AAh - Range1Dest (Lo/hi?)**

```
  0-15 Unknown (maybe SRAM mapping target for above ROM ranges?)
```

**C0A4h - Range0Mask (aka trm)**

**C0ACh - Range1Mask**

```
  0-7  Unknown
```

**C0D0h/C0D2h - VTableBase Byte0/1 (Lo/hi) (aka kvtablel/h)**

```
  0-15 Unknown (maybe SRAM mapping target for ROM patch vectors?)
```

vector table base address? (in 32-byte, or 32-word steps maybe?)

**C0D8h/C0DAh - Enable Byte0/1 (Lo/hi) (aka enbll/h)**

```
  0-10 Vector 0-10 Enable (aka enable "kPatch_0..10"?) (?=off, ?=on)
  11   range0ena
  12   range1ena
  13   unknown/unused
  14   transAddrEnable aka magicena   ;enable transition address
  15   zeroPageEnable                 ;enable zero page  <-- game cart access?
```

**C0C0h/C0C2h - RAMBase, Byte0/1 (Lo/hi) (aka saferambasel/h)**

```
  0-15 Unknown
```

**C0C8h/C0CAh - RAMBound Byte0/1 (Lo/hi) (aka saferambndl/h)**

```
  0-15 Unknown
```

**C0E0h - ROMBound (aka saferombnd)**

```
  0-7  Unknown
```

**C0E8h - ROMBase (aka saferombase)**

```
  0-7  Unknown
```

**C0F8h/C0FAh - AddrStatus (Lo/hi?) (aka addrstatusl/h)**

```
  0-15 Unknown
```

### SNES Cart X-Band I/O - Smart Card Reader

The X-Band contains a built-in Smart Card reader (credit card shaped chip cards
with 8 gold contacts). The X-Band BIOS contains messages that refer to "XBand
Cards" and "XBand Rental Cards". There aren't any photos (or other info) of
these cards in the internet, maybe X-Band requested customers to return the
cards, or the cards got lost for another reason.

**Purpose**

Not much known. Reportedly the card reader was used for Prepaid Cards (for
users whom didn't want Xband to charge their credit cards automatically), if
that is correct, then only those users would have received cards, and other
users didn't need to use the card reader? Note: The Options/Account Info screen
show entries "Account" and "Card".

**Smart Card I/O Ports**

The BIOS seems to be accessing the cards via these I/O ports:

```
  FBC100h Card Data/Control/Whatever (out)
  FBC108h.Bit0 (In) Card Switch (1=card inserted, 0=card missing)
  FBC108h.Bit1 (In) Card Data (input)
```

Related BIOS functions are function 0380h..0386h (on SNES/US).

**C100h - SControl (aka sctl) ;smart card control**

```
  0   outputClk:   equ     $01
  1   enOutputData:equ     $02  ;aka data direction?
  2   outputData:  equ     $04
  3   outputReset: equ     $08
  4   outputVcc:   equ     $10
  5-7 unknown/unused
```

**C108h - SStatus (aka sstatus) ;smart card status**

```
  0   detect:      equ     $01 Card Switch (1=card inserted, 0=card missing)
  1   dataIn:      equ     $02 Card Data (input)
  2   outputClk:   equ     $04   ;\
  3   enOutputData:equ     $08   ; Current state of Port C100h.Bit0-4 ?
  4   outputData:  equ     $10   ;
  5   outputReset: equ     $20   ;
  6   outputVcc:   equ     $40   ;/
  7   outputVpp:   equ     $80   ;-Current state of Port ????.Bit0 ?
```

**???? - Smart Card control ii**

parameters for control ii  ;<-- from i\feq.a (uh, "control ii" is what?)

```
  0   ksoutputvpp: equ     $01
  1-7 unknown/unused
```

```
               _______ _______
       VCC C1 |       |       | C5 GND          common smart card pinout
              |____   |   ____|                 (unknown if xband is actually
       RST C2 |    \__|  /    | C6 VPP          using that same pinout)
              |____/     \____|
       CLK C3 |    \_____/    | C7 I/O
              |____/  |  \____|
       NC? C4 |       |       | C8 NC?
              |_______|_______|
```

### SNES Cart X-Band I/O - LED and Debug

**C168h - LEDData (aka leddata)**

```
  0-7  probably controls the LEDs (can be also used for other stuff)
```

Note: Sega version has 7 LEDs, SNES version has only 3 LEDs. Unknown which of
the 8 bits are controlling which LEDs.

**C16Ah - LEDEnable (aka ledenable)**

```
  0-7  seems to select data-direction for LED pins (0=input, 1=output)
```

**Debug Connection via LED ports**

People at Catapult have reportedly used modified X-Band PCBs during debugging:
The seven genesis LEDs replaced by a DB25 connector with 8 wires (7 debug
signals, plus GND, probably connected to a PC parallel/printer port). That
hardware mod also used special software (some custom X-Band BIOS on FLASH/ROM,
plus whatever software on PC side).

**Unknown 64bit Number via LED ports**

The SNES X-Band BIOS is reading a 64bit number via serial bus (which might
connect to exteral debug hardware, or to 'unused' smart card pins, or to
whatever), done via two I/O Ports:

```
  FBC168h Data        ;bit2 (data, in/out)       ;\there is maybe also a reset
  FBC16Ah Direction   ;bit2 (0=input, 1=output)  ;/flag, eventually in bit5 ?
```

The sequence for reading the 64bits is somewhat like so:

```
  Data=Output(0), Delay (LOOPx01F4h)
  Data=Output(1), Delay (LOOPx01F4h)
  Data=Input
  wait until Data=1 or fail if timeout
  wait until Data=0 or fail if timeout
  wait until Data=1 or fail if timeout
  Delay (LOOPx02BCh)
  for i=1 to 4
    Data=Output(0), Delay (NOPx8)
    Data=Output(1), Delay (NOPx8)
    Data=Input, Delay (LOOPx0050h)
  for i=1 to 4
    Data=Output(0), Delay (NOPx8)
    Data=Output(1), Delay (LOOPx003Ch)
    Data=Input, Delay (LOOPx001Eh)
  Data=Input, Delay (LOOPx0064h)
  for i=0 to 63
    Data=Output(1), Delay (NOPx8)
    Data=Input, Delay (LOOPx000Ah)
    key.bit(i)=Data, Delay (LOOPx004Bh)
```

For the exact timings (Delays and other software overload), see the BIOS
function (at D7BE78h). Before doing the above stuff, the BIOS initializes
[FBC168h]=40h, [FBC16Ah]=FFh (this may be also required).

The 64bit number is received LSB first, and stored in SRAM at 3FD8h-3FDFh.
Whereas the last byte is a checksum across the first 7 bytes, calculated as so:

```
  sum=00h
  for i=0 to 55
    if (sum.bit(0) xor key.bit(i))=1 then sum=sum/2 xor 8Ch else sum=sum/2
```

For example, if the 7 bytes are "testkey", then the 8th byte must be 2Fh. Or,
another simplier example would be setting all 8 bytes to 00h.

### SNES Cart X-Band I/O - Whatever Stuff (External FIFO for Modem?)

Below is some additional modem stuff (additionally to the normal Rockwell Modem
registers at C180h-C1BEh). The original source code refers to that extra stuff
as "modem (and serial) bits". Purpose is unknown...

Maybe the Rockwell Modem chip lacks internal FIFOs, so the VBlank handler could
transfer max 60 bytes/second. As a workaround, the Fred chip might contain some
sort of external FIFOs, allowing to send around 4 bytes per Vblank (which would
gain 240 bytes/second, ie. gaining the full bandwidth of the 2400 baud modem).

If so, then the Fred chip should be wired either to the Rockwell databus, or to
the Rockwell serial bus. Despite of the possible FIFO feature, directly
accessing the Rockwell RX/TX registers seems to be also supported.

**C118h - MStatus1 (aka mstatus1)**

```
  0  enModem
  1  resetModem
  2  bit_8
  3  enstop
  4  onestop
  5  enparity
  6  oddparity
  7  break
```

**C120h - TxBuff (aka txbuff)**

**C128h - RxBuff (aka rxbuff)**

Some TX/RX FIFOs?

**C130h - ReadMStatus2 (aka readmstatus2)**

```
  0   kRMrxready:    rxready:         equ $01  ;1 = have rx data, 0 = no data
  1   kRMframeerr:   ltchedframeerr:  equ $02
  2   kRMparityerr:  ltchedparityerr: equ $04
  3-4 kRMframecount: sfcnt:           equ $18
  6-7 unknown/unused
```

Bit3-4 is a 2bit framecounter to tell whether a byte arrived this frame or a
prev frame. It's a little wacky to use because unlike VCnt, there is no
separate place to read it on Fred other than right here, sharing it with the
FIFO. So, you must do the following:

If there is data in the FIFO, framecount reflects the frame number of the
oldest byte in the FIFO. If the FIFO is empty, however, it reflects the current
frame number. Used carefullly (i.e. make sure rxready is 0 if you are using it
for the current framecount), it should allow you to determine if a byte arrived
in the current frame or up to 3 previous frames ago.

**C140h - ReadMStatus1 (aka readmstatus1)**

```
  0   txfull:       equ $01      ; 1 = full, 0 = not full
  1   txempty:      equ $02
  2   rxbreak:      equ $04
  3   overrun:      equ $08
  4-5 smartrxretry: equ $30   smartrxnumretry:      equ     $30
  6-7 smarttxretry: equ $c0   smarttxnumretry:      equ     $c0
```

**C148h - Guard (aka guard)**

```
  0-7 unknown
```

**C150h - BCnt (aka bcnt)**

```
  0-7 unknown (whatever... B control... or B counter?)
```

**C158h - MStatus2 (aka mstatus2)**

```
  0   ensmartrxretry: equ     $1 ;
  1   ensmarttxretry: equ     $2 ;
  2   smart:          equ     $4 ;
  3   sync:           equ     $8 ;
  4-7 unknown/unused
```

**C160h - VSyncWrite (aka vsyncwrite)**

```
  0-7 unknown
```

Maybe the vsync/vblank handler must write here by software in order to reset to
"V" counters?

**C110h - ReadMVSyncLow (aka readmvsync)     ;<--Low?**

**C112h - ReadMVSyncHigh (aka readmvsynclow) ;<--low?**

```
  Range of 0 to $61. Equal to ReadSerialVCnt/2.
  Value is $5c at start of VBlank.
```

**C138h - ReadSerialVCnt (aka readserialvcnt)**

```
  0-7 some incrementing counter...
             ;i\feq.a: kreadserialvcnt
             ; top 8 bits of 20 bit counter tied to
             ; input clock, or it increments 1 each 4096 clks
             ; resets to zero at vblank
             ; at 24 mhz, each 170.667 usec
             ; in 1/60 sec, counts up to 97 ($61), so
             ; range is 0 to $61 (verified by observation)
```

```
                     ;i\harddef.a: kReadSerialVCnt
                     ; Top 8 bits of 19 bit counter tied to
  kFirstVCnt equ $5c ; input clock, i.e. it increments 1 each 2048 clks.
  kLastVCnt  equ $5b ; At 24 MHz, each 85.333 usec
  kMaxVCnt   equ $61 ; in 1/60 sec, counts up to 195 ($C3), so
  kMinVCnt   equ $00 ; range is 0 to $C3 (not yet verified by testing)
                     ; Value is about $B8 at start of vblank, counts up to $C3,
                     ; wraps to 0.  Note that ReadMVSyncHigh VCnt is equal
                     ; ReadSerialVCnt/2. Note also that if there is no data
                     ; in the read fifo, it appears that ReadSerialVCnt has
                     ; the value of ReadMVSyncHigh (i.e. 1/2 the resolution)
```

```
  kVCntsPerModemBit: equ     $5 ; 1 modem bit time is 1/2400 sec, or 417 usec
                                ; 417/85.333 (1 VCnt) = 4.89, rounded up gives
                                ; 5 VCnts per modem bit. Not that this refers
                                ; to ReadSerialVCnt.
```

```
  kLinesPerModemBit: equ     $7 ; 417/64 (1 horiz line time) = 6.51, rounded up
                                ; gives 7 Lines per modem bit
```

```
  ; for rx:
  ; 1. read status until rxready
  ; 2. read serialVcnt               <-- uhm, what/why?
  ; 3. read Rxbuff (reading rxbuff clears the full fifo entry)
```

### SNES Cart X-Band I/O - Rockwell Modem Ports

Below are the I/O Ports of the Rockwell chip. In the SNES, Rockwell registers
00h-1Fh are mapped to EVEN memory addresses at FBC180h-FBC1BEh. The chip used
in the SNES supports data/voice modem functions (but not fax modem functions).

**FBC180h/FBC182h - 00h/01h - Receive Data Buffer**

```
  0-7  RBUFFER Received Data Buffer. Contains received byte of data
  8    RXP     Received Parity bit (or ninth data bit)
  9-15 N/A     Unused
```

**FBC184h/FBC186h - 02h/03h - Control**

```
  0-8  N/A     Unused
  9    GTE     TX 1800Hz Guard Tone Enable (CCITT configuration only)
  10   SDIS    TX Scrambler Disable
  11   ARC     Automatic on-line Rate Change sequence Enable
  12   N/A     Unused
  13   SPLIT   Extended Overspeed TX/RX Split. Limit TX to basic overspeed rate
  14   HDLC    High Level HDLC Protocol Enable (in parallel data mode)
  15   NRZIE   Unknown (listed in datasheet without further description)
```

**FBC188h/FBC18Ah - 04h/05h - Control**

```
  0     CRFZ   Carrier Recovery Freeze. Disable update of receiver's carrier
               recovery phase lock loop
  1     AGCFZ  AGC Freeze. Inhibit updating of receiver AGC
  2     IFIX   Eye Fix. Force EYEX and EYEY serial data to be rotated
               equalizer output
  3     EQFZ   Equalizer Freeze. Inhibit update of receiver's adaptive
               equalizer taps
  4-5   N/A    Unused
  6     SWRES  Software Reset. Reinitialize modem to its power turn-on state
  7     EQRES  Equalizer Reset. Reset receiver adaptive equalizer taps to zero
  8     N/A    Unused
  9     TXVOC  Transmit Voice. Enable sending of voice samples
  10    RCEQ   Receiver Compromise Equalizer Enable. Control insertion of
               receive passband digital compromise equalizer into receive path
  11    CEQ(E) Compromise Equalizer Enable. Enable transmit passband digital
               compromise equalizer
  12    TXSQ   Transmitter Squelch. Disable transmission of energy
  13-15 N/A    Unused
```

**FBC18Ch/FBC18Eh - 06h/07h - Control**

```
  0,1 WDSZ   Data Word Size, in asynchronous mode (5, 6, 7, or 8 bits)
  2   STB    Stop Bit Number (number of stop bits in async mode)
  3   PEN    Parity Enable (generate/check parity in async parallel data mode)
  4,5 PARSL  Parity Select (stuff/space/even/odd in async parallel data mode)
  6   EXOS   Extended Overspeed. Selects extended overspeed mode in async mode
  7   BRKS   Break Sequence. Send of continuous space in parallel async mode
  8   ABORT  HDLC Abort. Controls sending of continuous mark in HDLC mode
  9   RA     Relay A Activate. Activate RADRV output
  10  RB     Relay B Activate. Activate RBDVR output
  11  L3ACT  Loop 3 (Local Analog Loopback) Activate. Select connection of
             transmitter's analog output Internally to receiver's analog input
  12  N/A    Unused
  13  L2ACT  Loop 2 (Local Digital Loopback) Activate. Select connection of
             receiver's digital output Internally to transmitter's digital
             input (locally activated digital loopback)
  14  RDL    Remote Digital Loopback Request. Initiate a request for remote
             modem to go into digital loop-back
  15  RDLE   Remote Digital Loopback Response Enable. Enable modem to respond
             to remote modem's digital loopback request
```

**FBC190h/FBC192h - 08h/09h - Control**

```
  0   RTS    Request to Send. Request transmitter to send data
  1   RTRN   Retrain. Send retrain-request or auto-rate-change to remote modem
  2   N/A    Unused
  3   TRFZ   Timing Recovery Freeze. Inhibit update of receiver's timing
             recovery algorithm
  4   DDIS   Descrambler Disable. Disable receiver's descrambler circuit
  5   N/A    Unused
  6   TPDM   Transmitter Parallel Data Mode. Select parallel/serial TX mode
  7   ASYNC  Asynchronous/Synchronous. Select sync/async data mode
  8   SLEEP  Sleep Mode. Enter SLEEP mode (wakeup upon pulse on RESET pin)
  9   N/A    Unused
  10  DATA   Data Mode. Select idle or data mode
  11  LL     Leased Line. Select leased line data mode or handshake mode
  12  ORG    Originate. Select originate or answer mode (see TONEC)
  13  DTMF   DTMF Dial Select. Select DTMF or Pulse dialing in dial mode
  14  CC     Controlled Carrier. Select controlled or constant carrier mode
  15  NV25   Disable V.25 Answer Sequence (Data Modes), Disable Echo Suppressor
             Tone (Fax Modes). Disable transmitting of 2100Hz CCITT answer tone
             when a handshake sequence is initiated in a data mode or disables
             sending of echo suppressor tone in a fax mode
```

**FBC194h/FBC196h - 0Ah/0Bh - Status**

```
  0   CRCS   CRC Sending. Sending status of 2-byte CRC in HDLC mode
  1-7 N/A    Unused
  8   BEL1O3 Bell 103 Mark Frequency Detected. Status of 1270Hz Bell 103 mark
  9   DTDET  DTMF Digit Detected. Valid DTFM digit has been detected
  10  PNSUC  PN Success. Receiver has detected PN portion of training sequence
  11  ATBELL Bell Answer Tone Detected. Detection status of 2225Hz answer tone
  12  ATV25  V25 Answer Tone Detected. Detection status of 2100Hz answer tone
  13  TONEC  Tone Filter C Energy Detected. Status of 1650Hz or 980Hz (selected
             by ORG bit) FSK tone energy detection by Tone C bandpass filter in
             Tone Detector configuration
  14  TONEB  Tone Filter B Energy Detected. Status of 390Hz FSK tone energy
             detection by Tone B bandpass filter in Tone Detector configuration
  15  TONEA  Tone Filter A Energy Detected. Status of energy above threshold
             detection by Call Progress Monitor filter in Dial Configuration or
             1300 Hz FSK tone energy detection by Tone A bandpass filter in
             Tone Detector configuration
```

**FBC198h/FBC19Ah - 0Ch/0Dh - Status**

```
  0-3 DTDIG  Detected DTMF Digit. Hexadecimal code of detected DTMF digit
  4-6 N/A    Unused
  7   EDET   Early DTMF Detect. High group frequency of DTMF tone pair detected
  8-9 N/A    Unused
  10  SADET  Scrambled Alternating Ones Sequence Detected
  11  U1DET  Unscrambled Ones Sequence Detected
  12  SCR1   Scrambled Ones Sequence Detected
  13  S1DET  S1 Sequence Detected
  14  PNDET  Unknown (listed in datasheet without further description)
  15  N/A    Unused
```

**FBC19Ch/FBC19Eh - 0Eh/0Fh - Status**

```
  0-2 SPEED  Speed Indication. Data rate at completion of a connection
  3   OE     Overrun Error. Overrun status of Receiver Data Buffer (RBUFFER)
  4   FE     Framing Error. Framing error or detection of an ABORT sequence
  5   PE     Parity Error. Parity error status or bad CRC
  6   BRKD   Break Detected. Receipt status of continuous space
  7   RTDET  Retrain Detected. Detection status of a retrain request sequence
  8   FLAGS  Flag Sequence. Transmission status of Flag sequence in HDLC mode,
             or transmission of a constant mark in parallel asynchronous mode
  9   SYNCD  Unknown (listed in datasheet without further description)
  10  TM     Test Mode. Active status of selected test mode
  11  RI     Ring Indicator. Detection status of a valid ringing signal
  12  DSR    Data Set Ready. Data transfer state
  13  CTS    Clear to Send. Training sequence has been completed (see TPDM)
  14  FED    Fast Energy Detected. Energy above turn-on threshold is detected
  15  RLSD   Received Line Signal Detector (carrier and receipt of valid data)
```

**FBC1A0h/FBC1A2h - 10h/11h - Transmit Data Buffer**

```
  0-7   TBUFFER Transmitter Data Buffer. Byte to be sent in parallel mode
  8     TXP     Transmit Parity Bit (or 9th Data Bit)
  9-15  N/A     Unused
```

**FBC1A4h/FBC1A6h - 12h/13h - Control**

```
  0-7   CONF   Modem Configuration Select. Modem operating mode (see below)
  8-9   TXCLK  Transmit Clock Select (internal, disable, slave, or external)
  10-11 VOL    Volume Control. Speaker volume (off, low, medium, high)
  12-15 TLVL   Transmit Level Attenuation Select. Select transmitter analog
               output level attenuation in 1 dB steps. The host can fine tune
               transmit level to a value lying within a 1 dB step in DSP RAM
```

**FBC1A8h/FBC1AAh - 14h/15h - Unused**

```
  0-15  N/A    Unused
```

**FBC1ACh/FBC1AEh - 16h/17h - Y-RAM Data (16bit)**

**FBC1B0h/FBC1B2h - 18h/19h - X-RAM Data (16bit)**

```
  0-15  DATA  RAM data word (R/W)
```

**FBC1B4h/FBC1B6h - 1Ah/1Bh - Y-RAM Addresss/Control**

**FBC1B8h/FBC1BAh - 1Ch/1Dh - X-RAM Addresss/Control**

```
  0-8   ADDR  RAM Address
  9     WT    RAM Write (controls read/write direction for RAM Data registers)
  10    CRD   RAM Continuous Read. Enables read of RAM every sample from
              location addressed by ADDR Independent of ACC and WT bits
  11    IOX   X-RAM only: I/O Register Select. Specifies that X RAM ADDRESS
              bit0-7 (Port 1Ch) is an internal I/O register address
  11    N/A   Y-RAM only: Unused
  12-14 N/A   Unused
  15    ACC   RAM Access Enable. Controls DSP access of RAM associated with
              address ADDR bits. WT determines if a read or write is performed
```

**FBC1BCh/FBC1BEh - 1Eh/1Fh - Interrupt Handling**

```
  0   RDBF    Receiver Data Buffer Full (RBUFFER Full)
  1   N/A     Unused
  2   RDBIE   Receiver Data Buffer Full Interrupt Enable
  3   TDBE    Transmitter Data Buffer Empty (TBUFFER Empty)
  4   N/A     Unused
  5   TDBIE   Transmitter Data Buffer Empty Interrupt
  6   RDBIA   Receiver Data Buffer Full Interrupt Active (IRQ Flag)
  7   TDBIA   Transmitter Data Buffer Empty Interrupt Active (IRQ Flag)
  8   NEWC    New Configuration. Initiates new configuration (cleared by modem
  9   N/A     Unused                   upon completion of configuration change)
  10  NCIE    New Configuration Interrupt Enable
  11  NEWS    New Status. Detection of a change in selected status bits
  12  NSIE    New Status Interrupt Enable
  13  N/A     Unused
  14  NCIA    New Configuration Interrupt Active (IRQ Flag)
  15  NSIA    New Status Interrupt Active (IRQ Flag)
```

**CONF Values**

Below are CONF values taken from RC96DT/RC144DT datasheet (the
RC96V24DP/RC2324DPL datasheet doesn't describe CONF values). Anyways, the
values hopefully same for both chip versions (except that, the higher baudrates
obviously won't work on older chips).

```
  CONF  Bits/sec Mode Name
  01h   2400     V.27 ter
  02h   4800     V.27 ter
  11h   4800     V.29
  12h   7200     V.29
  14h   9600     V.29
  52h   1200     V.22
  51h   600      V.22
  60h   0-300    Bell 103
  62h   1200     Bell 212A
  70h   -        V.32 bis/V.23 clear down    ;\
  71h   4800     V.32                        ;
  72h   12000    V.32 bis TCM                ; RC96DT/RC144DT only
  74h   9600     V.32 TCM                    ; (not RC96V24DP/RC2324DPL)
  75h   9600     V.32                        ;
  76h   14400    V.32 bis TCM                ;
  78h   7200     V.32 bis TCM                ;/
  80h   -        Transmit Single Tone
  81h   -        Dialing                  ;used by SNES X-Band (dial mode)
  82h   1200     V.22 bis
  83h   -        Transmit Dual Tone
  84h   2400     V.22 bis                 ;used by SNES X-Band (normal mode)
  86h   -        DTMF Receiver
  A0h   0-300    V.21
  A1h   75/1200  V.23 (TX/RX)
  A4h   1200/75  V.23 (TX/RX)
  A8h   300      V.21 channel 2
  B1h   14400    V.17 TCM                    ;\
  B2h   12000    V.17 TCM                    ; RC96DT/RC144DT only
  B4h   9600     V.17 TCM                    ; (not RC96V24DP/RC2324DPL)
  B8h   7200     V.17 TCM                    ;/
```

**XBand X/Y RAM Rockwell**

Below are X/Y RAM addresses that can be accessed via Ports 16h-1Dh.

Addresses 000h-0FFh are "Data RAM", 100h-1FFh are "Coefficient RAM".

X-RAM is "Real RAM", Y-RAM is "Imaginary RAM" (whatever that means).

```
  XRAM     YRAM     Parameter
  032      -        Turn-on Threshold
  03C      -        Lower Part of Phase Error (this, in X RAM ?)
  -        03C      Upper Part of Phase Error (this, in Y RAM ?)
  -        03D      Rotation Angle for Carrier Recovery
  03F      -        Max AGC Gain Word
  049      049      Rotated Error, Real/Imaginary
  059      059      Rotated Equalizer Output, Real/Imaginary
  05E      05E      Real/Imaginary Part of Error
  06C      -        Tone 1 Angle Increment Per Sample (TXDPHI1)
  06D      -        Tone 2 Angle Increment Per Sample (TXDPHI2)
  06E      -        Tone 1 Amplitude (TXAMP1)
  06F      -        Tone 2 Amplitude (TXAMP2)
  070      -        Transmit Level Output Attenuation
  071      -        Pulse Dial Interdigit Time
  072      -        Pulse Dial Relay Make Time
  073      -        Max Samples Per Ring Frequency Period (RDMAXP)
  074      -        Min Samples Per Ring Frequency Period (RDMINP)
  07C      -        Tone Dial Interdigit Time
  07D      -        Pulse Dial Relay Break Time
  07E      -        DTMF Duration
  110-11E  100-11E  Adaptive Equalizer Coefficients, Real/Imag.
  110      100      First coefficient, Real/Imag. (1) (Data/Fax)
  110      110      Last Coefficient, Real/Imag. (17) (Data)
  11E      11E      Last Coefficient, Real/Imag. (31) (Fax)
  -        121      RLSD Turn-off Time
  12D      -        Phase Error
  12E      -        Average Power
  12F      -        Tone Power (TONEA)
  130      -        Tone Power (TONEB,ATBELL,BEL103)
  131      -        Tone Power (TONEC,ATV25)
  136      -        Tone Detect Threshold for TONEA               (THDA)
  137      -        Tone Detect Threshold for TONEB,ATBELL,BEL103 (THDB)
  138      -        Tone Detect Threshold for TONEC,ATV25         (THDC)
  13E      -        Lower Part of AGC Gain Word
  13F      -        Upper Part of AGC Gain Word
  152      -        Eye Quality Monitor (EQM)
  -        162-166  Biquad 5 Coefficients a0,a1,a2,b1,b2
  -        167-16B  Biquad 6 Coefficients a0,a1,a2,b1,b2
  -        16C-170  Biquad 1 Coefficients a0,a1,a2,b1,b2
  -        171-175  Biquad 2 Coefficients a0,a1,a2,b1,b2
  -        176-17A  Biquad 3 Coefficients a0,a1,a2,b1,b2
  179      -        Turn-off Threshold
  -        17B-17F  Biquad 4 Coefficients a0,a1,a2,b1,b2
```

### SNES Cart X-Band Rockwell Notes

**Rockwell Configuration Changes**

Various changes (to ASYNC, WDSZ, etc.) seem to be not immediately applied.
Instead, one must apply them by setting NEWC=1 by software (and then wait until
hardware sets NEWC=0).

**Rockwell Dialing**

Dialing is done by setting CONF=81h, and then writing the telephone number
digits (range 00h..09h) to TBUFFER; before each digit wait for TDBE=1 (TX
buffer empty), the BIOS also checks for TONEA=1 before dialing.

The telephone number for the X-Band server is stored as ASCII string in the
BIOS ROM:

```
  "18002071194"  at D819A0h in US-BIOS (leading "800" = Toll-free?)
  "03-55703001"  at CE0AB2h in Japanese BIOS (leading "3" = Tokyo?)
```

Notes: Before dialing the above 'ASCII' numbers, the US-BIOS first dials
0Ah,07h,00h, and the japanese one first dials 01h. The "-" dash in the japanese
string isn't dialed.

**Rockwell Offline**

There seems to be no explicit offline mode (in CONF register). Instead, one
must probably change the Relay A/B bits (RA/RB) to go online/offline.

**X-Band Fred Chip Pin-Outs**

```
  1-100 unknown
```

**X-Band Rockwell Pin-Outs**

```
  Pin Number Signal Name I/O Type
  1 RS2 IA
  2 RS1 IA
  3 RS0 IA
  4 /TEST1
  5 /SLEEP OA
  6 RING
  7 EYEY OB
  8 EYEX OB
  9 EYESYNC OB
  10 RESET ID
  11 XTLI IE
  12 XTLO OB
  13 +5VD
  14 GP18 OA
  15 GP16 OA
  16 XTCLK IA
  17 DGND1
  18 TXD IA
  19 TDCLK OA
  20 TRSTO MI
  21 TSTBO MI
  22 TDACO MI
  23 RADCI MI
  24 RAGCO MI
  25 MODEO MI
  26 RSTBO MI
  27 RRSTO MI
  28 /RDCLK OA
  29 RXD OA
  30 TXA2 O(DD)
  31 TXA1 O(DD)
  32 RXA I(DA)
  33 RFILO MI
  34 AGCIN MI
  35 VC
  36 NC
  37 NC
  38 NC
  39 /RBDVR OD
  40 AGND
  41 /RADRV OD
  42 /SLEEP1 IA
  43 RAGCI MI
  44 NC
  45 RSTBI MI
  46 RRSTI MI
  47 RADCO MI
  48 TDACI MI
  49 TRSTI MI
  50 TSTBI MI
  51 MODE1 MI
  52 +5VA
  53 SPKR O(OF)
  54 DGND2
  55 D7 IA/OB
  56 D6 IA/OB
  57 D5 IA/OB
  58 D4 IA/OB
  59 D3 IA/OB
  60 D2 IA/OB
  61 D1 IA/OB
  62 D0 IA/OB
  63 /IRQ OC
  64 /WRITE IA
  65 /CS IA
  66 /READ IA
  67 RS4 IA
  68 RS3 IA
```

Notes:

(1) MI = Modem Interconnection

(2) NC = No connection (may have internal connection; leave pin disconnected
(open).

(3) I/O types are described in Table 2-3 (digital signals) and Table 2-4
(analog signals).

### SNES Cart X-Band BIOS Functions

**X-Band BIOS Functions (CALL E00040h)**

Invoked via CALL E00040h, with X=function_number (0001h..054xh on SNES/US),
with parameters pushed on stack, and with return value in A register (16bit) or
X:A register pair (32bit), and with zeroflag matched to the A return value.

The function table isn't initialized by the compiler/linker, instead, the BIOS
boot code is starting the separate components (such like "controls.c"), which
are then installing their function set via calls to "SetDispatchedFunction".

The Sega function numbers are based on the string list in file
"SegaServer\Server\Server_OSNumbers.h" (which is part of the SERVER sources,
but it does hopefully contain up to date info on the retail BIOS functions).

```
  Sega SNES SNES Function
  Gen. US   JP
```

**Sourceless - Misc**

```
  000h           RestoreSegaOS
  001h           AskForReplay
  002h           ThankYouScreen            ;thankyou shown at next coldboot?
  003h           InstallDispatchedManager
  004h           CallManagerControl
  005h           SoftInitOS
  006h           GetDispatchedFunction
  007h 007h 007h SetDispatchedFunction     ;change/install BIOS function vector
  008h           SetDispatchedGroup
  009h           GetManagerGlobals
  00Ah           SetManagerGlobals
  00Bh           AllocateGlobalSpace
  00Ch           FreeGlobalSpace
  00Dh           DisposePatch
  00Eh           CompactOSCodeHeap
  00Fh           GetPatchVersion
  010h           SetPatchVersion
```

**Sourceless - Memory**

```
  011h           InitHeap
  012h           NewMemory
  013h           NewMemoryHigh
  014h           NewMemoryClear
  015h           DisposeMemory
  016h 01Ah 01Ah GetMemorySize          ;get size of an item
  017h           MaxFreeMemory
  018h           TotalFreeMemory
  019h           SwitchPermHeap
  01Ah           SwtichTempHeap  ;uh, Swtich?
  01Bh           CreateTempHeap
  01Ch           CreateHeapFromPtr
  01Dh           CreateTempSubHeap
  01Eh           AllocPermHeapZone
  01Fh           DisposePermHeapZone
  020h           CompactHeap
  021h           MoveHeap
  022h           PrepareHeapForMove
  023h           ComputeHeapPtrDelta
  024h           ResizeHeap
  025h           BlockMove
  026h           WhichMemory
  027h           GetHeapSize
  028h           VerifySegaHeap
  029h           PurgePermHeaps
  02Ah           ByteCopy
  02Bh           UnpackBytes
  02Ch           FillMemory
  02Dh           GetCurrentHeap
  02Eh           FindLastAllocatedBlock
  02Fh           SetOSUnstable
  030h           SetDBUnstable
  031h           SetAddressUnstable
  032h           InstallReliableAddress
  033h           CheckOSReliable
```

**GameLib\controls.c - Keyboard/Joypad Controls**

```
  034h ?         InitControllers
  035h 033h      ReadHardwareController      ;get joypad data
  036h 034h      ControllerVBL               ;do joypad and keyboard scanning
  037h ?         ReadAllControllers
  038h 036h      FlushHardwareKeyboardBuffer ;flush char_queue
  039h 037h      GetNextHardwareKeyboardChar ;read char_queue
  03Ah 038h      GetHardwareKeyboardFlags
  03Bh 039h      SetHardwareKeyboardFlags
  03Ch 03Ah      GetNextESKeyboardRawcode ;read scancode_queue  ;ES=Eric Smith
  03Dh ?         GetNextESKeyboardStatus
  03Eh 03Ch      GetNextESKeyboardChar    ;read scancode_queue, xlat to char
  03Fh ?         SendCmdToESKeyboard
  -    03Eh 03Fh keyb_io_read_scancodes
  -    03Fh      keyb_blah_do_nothing
  -    040h      keyb_io_read_verify_id_code
  -    041h 043h keyb_forward_scancode_queue_to_char_queue
```

**Sourceless - Misc**

```
  040h           GetGlobal
  041h           SetGlobal
```

**Database\PatchDB.c - Game/Patch (SNES: installed at D6:4F93 ?)**

```
  042h 042h 044h AddGamePatch
  043h           LoadGamePatch
  044h           DisposeGamePatch
  045h           GetGamePatchVersion
  046h           GetGamePatchFlags
  047h 04Ah 04Eh FindGamePatch
  048h 054h 058h CreateGameDispatcher
  049h           InitGamePatch
  04Ah           StartGame
  04Bh           GameOver
  04Ch           ResumeGame
  04Dh           GameDoDialog
  04Eh           UpdateGameResultsAfterError
  04Fh           HandleGameError
  050h           PlayCurrentGame
  051h 053h 057h InstallGameFunction
  052h 055h 059h DisposeOldestGamePatch
  053h           MarkGamePatchUsed
```

**Sourceless - Messages**

```
  054h           InitMessages
  055h           ProcessServerData
  056h           ProcessPeerData
  057h           SendMessage
  058h           GetSendMessageHandler
  059h           GetPeerMessageHandler
  05Ah           GetSerialOpCode
  05Bh           GetServerMessageHandler
  05Ch           InstallPeerHandler
  05Dh           InstallReceiveServerHandler
  05Eh           InstallSendMessageHandler
  05Fh           ReceivePeerMessageDispatch
  060h           ReceiveServerMessageDispatch
  061h           GobbleMessage
  062h           SetClearLoginMisc
  063h           GetLoginMisc
```

**Graphics\Sprites.c**

```
  064h           CreateSprite
  065h           CreateSpriteInFront
  066h           CreateSpriteHigh
  067h           DisposeSprite
  068h           MoveSprite
  069h           DrawSprite
  06Ah           IncrementSpriteFrame
  06Bh           SetSpriteFrame
  06Ch           GetSpriteFrame
  06Dh           FlipSprite
  06Eh           CreateSpriteData
  06Fh           CreateTextSprite
  070h           CreateTextSpriteFromBitmap
  071h           ExplodeSprite
  072h           SetSpriteGrayFlag
  073h           SetSpriteTilePosition
  074h           SetSpriteImage
  075h           SetSpritePalette
  076h           WriteSpriteToVDP
  077h           FigureTileSize
  078h           AllocateSprite
  079h           FreeSprite
  07Ah           GetSpriteLastTile
  07Bh           GetSpriteFirstTile
  07Ch           NewSpark
  07Dh           DisposeSpark
  07Eh           GetSparkSprite
  07Fh           StartSpark
  080h           StopSpark
  081h           DrawXBandLogo
  082h           DisposeXBandLogoRef
  083h           DisposeXBandLogoSparks
  084h           SyncOTron
```

**Graphics\Decompress.c**

```
  085h           InitDecompression
  086h           CreateDecompressor
  087h           DisposeDecompressor
  088h           SetDstPattern
  089h           SetImageTiling
  08Ah           SetImageOrigin
  08Bh           GetImageClut
  08Ch           DisposeImagePatterns
  08Dh           DecompressFrame
  08Eh           SetDecompressorOptionsSelector
  08Fh           SetDecompressorPixelMappingSelector
  090h           SetDecompressorPaletteSelector
  091h           GetDictionaryCache
  092h           ReleaseDictionaryCache
  093h           SetDecompressorImage
  094h           ExpandPatternDictionary
  095h           GetDecompressorCache
  096h           ReleaseDecompressorCache
  097h           JoshDecompress
```

**Sourceless - Time...**

```
  098h           AddTimeRequest
  099h           RemoveTimeRequest
  09Ah           TimeIdle
  09Bh           IncCurrentTime
  09Ch           DelayMS
  09Dh           DelayTicks
  09Eh           SetOSIdle
  09Fh           SegaOSIdle
  0A0h           GetJesusTime
  0A1h           SetJesusTime
  0A2h           GetJesusDate
  0A3h           SetJesusDate
```

**Graphics\animation.c - Animations**

```
  0A4h           InitAnimateProcs
  0A5h           SpawnAnimation
  0A6h           SpawnDBAnimation
  0A7h           CreateAnimation
  0A8h           DisposeAnimation
  0A9h           DrawAnimationFrame
  0AAh           StartAnimation
  0ABh           StopAnimation
  0ACh           SuspendAnimations
  0ADh           SetAnimationPriority
  0AEh           SetAnimationGrayFlag
  0AFh           GetAnimationSuspendLevel
```

**Graphics\paths.c - Paths (and maybe also LinePath.c?)**

```
  0B0h           InitPathManager
  0B1h           CreatePath
  0B2h           DisposePath
  0B3h           SetPathPoints
  0B4h           SetPathFrames
  0B5h           SetPathVelocity
  0B6h           GetPathPoint
  0B7h           DistBetweenPoints
```

**Graphics\Pattern.c**

```
  0B8h           InitPatternManager
  0B9h           NewPatternBlock
  0BAh           NewPatternBlockHigh
  0BBh           FreePatternBlock
  0BCh           DeallocateTopPatternBlock
  0BDh           NewFirstPatternBlock
  0BEh           SetRange
  0BFh           ClearRange
  0C0h           RangeIsFree
  0C1h           FindFreeRange
  0C2h           GetLeftOnesTable
  0C3h           GetRightOnesTable
```

**Graphics\Cursor.c**

```
  0C4h           CreateSegaCursor
  0C5h           DisposeSegaCursor
  0C6h           MoveSegaCursor
  0C7h           HideSegaCursor
  0C8h           ShowSegaCursor
  0C9h           GetSegaCursorPos
  0CAh           SetSegaCursorImage
  0CBh           LoadCursorFromVRAM
  0CCh           DrawSegaCursor
  0CDh           LoadCursorPattern
```

**Graphics\SegaText.c (1)**

```
  0CEh           InitSegaFonts
  0CFh           SetCurFont
  0D0h           GetCurFont
  0D1h           GetCurFontHeight
  0D2h           GetCurFontLineHeight
  0D3h           SetFontColors
  0D4h           GetFontColors
  0D5h           SetupTextGDevice
  0D6h           GetTextPatternAddress
  0D7h           GetTextGDeviceOrigin
  0D8h           DrawSegaString
  0D9h           RenderSegaString
  0DAh           MeasureSegaText
  0DBh           CenterSegaText
  0DCh           DrawClippedSegaText
  0DDh           DrawCenteredClippedSegaText
  0DEh           DrawPaddedClippedSegaText
  0DFh           GetCharWidth
  0E0h           SegaNumToString
  0E1h           SegaNumToDate
  0E2h           SegaAppendText
  0E3h           CompareDates
  0E4h           CompareStrings
  0E5h           SetupTextSpriteGDevice
  0E6h           EraseTextGDevice
  0E7h           GetStringLength
```

**Graphics\SegaText.c (2) and Database\StringDB.c**

```
  0E8h           DrawDBXYString              ;Database\StringDB.c
  0E9h           GetDBXYString               ;Database\StringDB.c
  0EAh           GetSegaString               ;Database\StringDB.c
  0EBh           GetWriteableString          ;Database\StringDB.c
  0ECh           SetWriteableString          ;Database\StringDB.c
  0EDh           DeleteWriteableString       ;Database\StringDB.c
  0EEh           GetUniqueWriteableStringID  ;Database\StringDB.c
  -              AddDBXYString               ;Database\StringDB.c (simulator)
```

**Graphics\SegaText.c (3)**

```
  0EFh           CopyCString
  0F0h           SetTextPatternStart
  0F1h           EqualCStrings
  0F2h           GetTextStateReference
  0F3h           SaveTextState
  0F4h           RestoreTextState
  0F5h           DisposeTextStateReference
  0F6h           VDPCopyBlitDirect
  0F7h           VDPCopyBlitDirectBGColor
  0F8h           VDPCopyBlitTiled
  0F9h           VDPCopyBlitTiledBGColor
  0FAh           OrBlit2to4
  0FBh           OrBlit1to4
```

**Sourceless - Modem? (parts related to GameLib\CommManager.c?)**

```
  0FCh           PInit
  0FDh           POpen
  0FEh           PListen
  0FFh           POpenAsync
  100h           PListenAsync
  101h           PClose
  102h           PNetIdle
  103h           PCheckError
  104h           PWritePacketSync
  105h           PWritePacketASync
  106h           PGetError
  107h           PUOpenPort
  108h           PUClosePort
  109h           PUProcessIdle
  10Ah           PUProcessSTIdle
  10Bh           PUReadSerialByte
  10Ch           PUWriteSerialByte
  10Dh           PUTransmitBufferFree
  10Eh           PUReceiveBufferAvail
  10Fh           PUTestForConnection
  110h           PUReadTimeCallback
  111h           PUWriteTimeCallback
  112h           PUSetupServerTalk
  113h           PUTearDownServerTalk
  114h           PUSetError
  115h           PUIsNumberBusy
  116h           PUOriginateAsync
  117h           PUAnstondet
  118h           PUWaitForRLSD
  119h           PUInitCallProgress
  11Ah           PUCallProgress
  11Bh           PUDialNumber
  11Ch           PUWaitDialTone
  11Dh           PUAnswerAsync
  11Eh           PUCheckAnswer
  11Fh           PUCheckRing
  120h           PUResetModem
  121h           PUSetTimerTicks
  122h           PUSetTimerSecs
  123h           PUTimerExpired
  124h           PUHangUp
  125h           PUPickUp
  126h           PUWriteXRAM
  127h           PUWriteYRAM
  128h 13Dh      PUReadXRAM
  129h 13Eh      PUReadYRAM
  12Ah           PUIdleMode
  12Bh           PUDataMode
  12Ch           PUDialMode
  12Dh           PUToneMode
  12Eh           PUCheckLine
  12Fh           PUCheckCarrier
  130h           PUDetectLineNoise
  131h           PUListenToLine
  132h           PUDisableCallWaiting
  133h           PUAsyncReadDispatch
  134h           PUDoSelectorLogin
  135h           PUMatchString
  136h           PGetDebugChatScript
```

**Sourceless - Transport?**

```
  137h           TInit
  138h           TOpen
  139h           TListen
  13Ah           TOpenAsync
  13Bh           TListenAsync
  13Ch           TClose
  13Dh           TCloseAsync
  13Eh           TUnthread
  13Fh           TNetIdle
  140h           TUCheckTimers
  141h           TReadDataSync
  142h           TReadDataASync
  143h           TWriteDataSync
  144h           TWriteDataASync
  145h           TAsyncWriteFifoData
  146h           TReadData
  147h           TWriteData
  148h           TReadAByte
  149h           TWriteAByte
  14Ah           TQueueAByte
  14Bh           TReadBytesReady
  14Ch           TDataReady
  14Dh           TDataReadySess
  14Eh           TIndication
  14Fh           TForwardReset
  150h           TNetError
  151h           TCheckError
  152h           TUInitSessRec
  153h           TUSendCtl
  154h           TUDoSendCtl
  155h           TUDoSendOpenCtl
  156h           TUUpdateSessionInfo
  157h           TUSendOpen
  158h           TUSendOpenAck
  159h           TUSendCloseAdv
  15Ah           TUSendFwdReset
  15Bh           TUSendFwdResetAck
  15Ch           TUSendFwdResetPacket
  15Dh           TUSendRetransAdv
  15Eh           TUOpenDialogPacket
  15Fh           TUFwdResetPacket
  160h           TUCloseConnPacket
  161h           TURetransAdvPacket
  162h           TUAllowConnection
  163h           TUDenyConnection
  164h           TUSetError
  165h           TUGetError
  166h           TGetUserRef
  167h           TSetUserRef
  168h           TGetTransportHold
  169h           TGetTransportHoldSession
  16Ah           TSetTransportHold
  16Bh           TSetTransportHoldSession
```

**Database\DB.c - Database**

```
  16Ch           InitPermDatabase
  16Dh           CompactPermDatabase
  16Eh 185h      DBGetItem
  16Fh           DBAddItem
  170h 188h 19Bh DBDeleteItem
  171h 189h 19Ch DBGetUniqueID
  172h           DBGetUniqueIDInRange
  173h           DBGetItemSize
  174h           DBCountItems
  175h 18Dh      DBGetFirstItemID
  176h 18Eh      DBGetNextItemID
  177h           DBNewItemType
  178h           DBGetTypeFlags
  179h           DBSetTypeFlags
  17Ah           DBDeleteItemType
  17Bh           DBPurge
  17Ch           DBTypeChanged
  17Dh           ComputeTypeCheckSum
  17Eh           DBVerifyDatabase
  17Fh           DBROMSwitch
  180h           DBAddItemPtrSize
  181h 199h 1ACh DBAddItemHighPtrSize
  182h 19Ah 1ADh DBPreflight    ;check if enough free mem for new item
  183h           GetItemSize
  184h           DBGetTypeNode
  185h           DBGetPrevTypeNode
  186h           DBTNGetItem
  187h           DBTNGetPrevItem
  188h           DBTNDisposeList
  189h           DeleteItem
  18Ah           AddItemToDB
  18Bh           AllowDBItemPurge
```

**Graphics\SegaScrn.c - Video/Screen**

```
  18Ch           LinearizeScreenArea
  18Dh           GetSegaScreenBaseAddr
  18Eh           InitSegaGDevices
  18Fh           SetCurrentDevice
  190h           GetCurrentDevice
  191h           RequestClut
  192h           ReleaseClut
  193h           IncrementClutReferences
  194h           SetupClutDB
  195h           GetSegaScreenOrigin
  196h           GetSegaGDevice
  197h           EraseGDevice
  198h           SetupVDP
  199h           BlankClut
  19Ah           FadeInClut
  19Bh           FadeInScreen
  19Ch           GenerateGrayMap
  19Dh           WaitVBlank
  19Eh           SetBackgroundColor
  19Fh           GetBackgroundColor
  1A0h           RequestUniqueClut
  1A1h           RequestSpecificClut
  1A2h           SetupClut
  1A3h           GetClut
  1A4h           GetColorLuminance
  1A5h           FillNameTable
```

**Sourceless - VRAM...**

```
  1A6h           DMAToVRAM
  1A7h           CopyToVRAM
  1A8h           CopyToCRAM
  1A9h           CopyToVSRAM
  1AAh           CopyToVMap
  1ABh           FillVRAM
  1ACh           FillCRAM
  1ADh           FillVSRAM
```

**Database\Opponent.c - Opponent**

```
  1AEh           GetOpponentPhoneNumber
  1AFh           SetOpponentPhoneNumber
  1B0h           GetCurOpponentIdentification
  1B1h           SetCurOpponentIdentification
  1B2h           GetCurOpponentTaunt
  1B3h           GetCurOpponentInfo
  1B4h           ClearOldOpponent
  1B5h           GetOpponentVerificationTag
  1B6h           SetOpponentVerificationTag
```

**Database\UsrConfg.c - User/Password**

```
  1B7h           GetCurrentLocalUser
  1B8h           FillInUserIdentification
  1B9h           GetLocalUserTaunt
  1BAh           SetLocalUserTaunt
  1BBh           GetLocalUserInfo
  1BCh           SetLocalUserInfo
  1BDh           IsUserValidated
  1BEh           SetCurUserID
  1BFh           GetCurUserID
  1C0h           VerifyPlayerPassword
  1C1h           IsEmptyPassword
  1C2h           ComparePassword
  1C3h           GetPlayerPassword
```

**UserInterface\DitlMgr.c - DITL (also related to Database\DITLItemSetup.c?)**

```
  1C4h           NewDITL
  1C5h           GiveDITLTime
  1C6h           DisposeDITL
  1C7h           GetDITLItem
  1C8h           InitDITLMgr
  1C9h           ClearDITLDone
  1CAh           ProcessDITLScreen
  1CBh           SetupDITLItemList
  1CCh           SetupDITLObjectData
  1CDh           DisposeDITLItemList
  1CEh           SetupControlTable
  1CFh           DisposeControlTable
  1D0h           GetDITLObjectData
```

**UserInterface\Events.c**

```
  1D1h           InitUserEvents
  1D2h           FlushUserEvents
  1D3h           WaitForUserButtonPress
  1D4h           CheckUserButtonPress
  1D5h           GetNextControllerEvent
  1D6h           GetNextCommand
  1D7h           QueueGet
  1D8h           QueueInsert
```

**Sourceless - Sound**

```
  1D9h           SetBGMDisable
  1DAh           GetBGMDisable
  1DBh           InitSoundMgr
  1DCh           ShutDownSoundMgr
  1DDh           StartDBBGM
  1DEh           StopBGM
  1DFh           PlayDBFX
  1E0h           FX1NoteOff
  1E1h           FX2NoteOff
  1E2h           ShutUpFXVoice1
  1E3h           ShutUpFXVoice2
```

**Sourceless - Misc**

```
  1E4h           GetDataSync
  1E5h           GetDataBytesReady
  1E6h           GetDataError
```

**Database\Challnge.c - Challenge**

```
  1E7h           GetChallengePhoneNumber
  1E8h           SetChallengePhoneNumber
  1E9h           GetChallengeIdentification
  1EAh           SetChallengeIdentification
```

**Database\GameID.c - Game ID**

```
  1EBh 210h 224h GetGameID     ;out:A=SnesCartStandardChksum, X=SnesHeaderCCITT
  -    211h 225h   ... related to GameID ?
```

**Sourceless - Misc**

```
  1ECh           IsRemoteModemTryingToConnect
  1EDh           SetRemoteModemTryingToConnectState
  1EEh           InitScreen
  1EFh           PreflightScreen
  1F0h           SetupScreen
  1F1h           SendCommandToScreen
  1F2h           KillScreen
  1F3h           GetNewScreenIdentifier
  1F4h           GetCurScreenIdentifier
  1F5h           GetScreenStateTable
  1F6h           ResetCurrentScreen
  1F7h           GetScreenLayoutRectangleCount
  1F8h           GetScreenLayoutRect
  1F9h           GetScreenLayoutCharRect
  1FAh           GetScreenLayoutPointCount
  1FBh           GetScreenLayoutPoint
  1FCh           GetScreenLayoutStringCount
  1FDh           GetScreenLayoutString
  1FEh           DrawScreenLayoutString
  1FFh           BoxScreenLayoutString
  200h           GetScreensEnteredCount
```

**Graphics\Backdrops.c**

```
  201h           SetBackdropID
  202h           SetBackdropBitmap
  203h           ClearBackdrop
  204h           HideBackdrop
  205h           SetAuxBackgroundGraphic
  206h           ShowBackdrop
  207h           GetBlinkySprite
```

**Database\BoxSer.c (1)**

```
  208h           GetBoxSerialNumber
  209h           SetBoxSerialNumber
  20Ah           GetHiddenBoxSerialNumbers
  20Bh           GetBoxHometown
  20Ch           SetBoxHometown
  20Dh           SetBoxState
  20Eh           ResetBoxState
  20Fh           GetBoxState
  210h           SetLastBoxState
  211h           ResetLastBoxState
  212h           GetLastBoxState
  213h           GetGameWinsLosses
  214h           SetCompetitionResults
  215h           GetCompetitionResults
  216h           SetGameErrorResults
  217h           GetGameErrorResults
  218h           UpdateGameResults
  219h           ClearGameResults
  21Ah           ClearNetErrors
  21Bh           GetLocalGameValue
  21Ch           SetLocalGameValue
  21Dh           GetOppGameValue
  21Eh           SetOppGameValue
  21Fh           IsBoxMaster
  220h           SetBoxMaster
  221h 24Bh 25Fh SetCurGameID            ;SNES/US: [3631,3633]
  222h 24Ch      GetCurGameID
  223h           CheckBoxIDGlobals
  224h           InitBoxIDGlobals
  225h           ChangedBoxIDGlobals
  226h           DBAddConstant
  227h           DBGetConstant
  228h           DBSetConstants
  229h           SetDialNetworkAgainFlag
  22Ah           CheckDialNetworkAgainFlag
  22Bh           SetBoxXBandCard
  22Ch           GetBoxXBandCard
  22Dh           GetBoxLastCard
  22Eh           SetBoxMagicToken
  22Fh           SetBoxProblemToken
  230h           GetBoxProblemToken
  231h           UseBoxProblemToken
  232h           SetBoxValidationToken
  233h           GetBoxValidationToken
  234h           SetIMovedOption
  235h           SetQwertyKeyboardOption
  236h           SetCallWaitingOption
  237h           SetAcceptChallengesOption
  238h           GetAcceptChallengesOption
  239h           GetIMovedOption
  23Ah           GetQwertyKeyboardOption
  23Bh           GetCallWaitingOption
  23Ch           GetNetErrors
```

**Database\BoxSer.c (2), and also Database\PhoneNumbers.c ?**

```
  23Dh           GetBoxPhoneNumber
  23Eh           SetBoxPhoneNumber
  23Fh           GetLocalAccessPhoneNumber
  240h           SetLocalAccessPhoneNumber
  241h           Get800PhoneNumber
```

**Database\BoxSer.c (3)**

```
  242h           GetLocalUserName
  243h           SetLocalUserName
  244h           GetLocalUserROMIconID
  245h           SetLocalUserROMIconID
  246h           GetLocalUserCustomROMClutID
  247h           SetLocalUserCustomROMClutID
  248h           GetLocalUserPassword
  249h           SetLocalUserPassword
  24Ah           ValidateUserPersonification
  24Bh           InvalidateUserPersonification
```

**Database\PlayerDB.c**

```
  24Ch           GetAddressBookTypeForCurrentUser
  24Dh           GetAddressBookIDFromIndex
  24Eh           CountAddressBookEntries
  24Fh           RemoveAddressBookEntry
  250h           GetIndexAddressBookEntry
  251h           AddAddressBookEntry
  252h           GetUserAddressBookIndex
  253h           DeleteAddressBookEntry
  254h           SendNewAddressesToServer
  255h           MarkAddressBookUnchanged
  256h           AddressBookHasChanged
  257h           CorrelateAddressBookEntry
  -              PreflightNewAddressEntry
```

**UserInterface\NewAddressMgr.c**

```
  258h           AddPlayerToAddressBook
  259h           UpdateAddressBookStuff
  25Ah           AddOnDeckAddressBookEntry
  25Bh           MinimizeUserHandle
```

**Database\GraphicsDB.c**

```
  25Ch           GetDBGraphics
  25Dh           DrawDBGraphic
  25Eh           DrawDBGraphicAt
  25Fh           DrawGraphic
  260h           DisposeGraphicReference
  261h           GetGraphicReferenceClut
  262h           DrawPlayerIcon
  263h           NukePlayerRAMIcon
  264h           GetPlayerRAMIconBitMap
  265h           GetPlayerIconBitMap
  266h           GetIconBitMap
  267h           PlayerRAMIconExists
  268h           DisposeIconReference
  269h           GetDBButtonFrame
  26Ah           DrawGraphicGray
  26Bh           HueShift
```

**Graphics\TextUtls.c - Text Edit**

```
  26Ch           FindLineBreak
  26Dh           SegaBoxText
  26Eh           DrawSegaStringLength
  26Fh           MeasureSegaTextLength
  270h           InitTextEdit
  271h           SetTextEditLineHeight
  272h           TextEditAppend
  273h           TextEditDelete
  274h           DisposeTextEdit
  275h           TextEditActivate
  276h           TextEditDeactivate
  277h           TextEditPreflightAppend
  278h           TextEditGetLineLength
  279h           DrawTextBox
  27Ah           SetJizzleBehavior
  27Bh           GetJizzleBehavior
  27Ch           StartTextBoxAnimation
  27Dh           StopTextBoxAnimation
  27Eh           DisposeTextBoxReference
  27Fh           DrawSegaTextPlusSpaces
  280h           UpdateTECaret
  281h           EraseTextEditLine
  282h           GetCompressedJizzlers
```

**Database\News.c (and NewsUtils.c) - News**

```
  283h           FindNextNewsString
  284h           AddPageToNewsBox
  285h           GetPageFromNewsBox
  286h           GetNewsForm
  287h           GetNumNewsPages
  288h           EmptyNewsBox
  289h           DrawNewsPage
  28Ah           ValidateNews
  28Bh           InvalidateNews
  28Ch           SetupNewsForServerConnect
  28Dh           ServerConnectNewsDone
  28Eh           DoNewsControlIdle
  28Fh           KillCurNewsPage
  290h           GetNewsGraphicsID
  291h           ShowLeftRightPageControls
  292h           DrawNewsReturnIcon
  293h           SetNewsCountdownTimeConst
  294h           DrawXBandNews
  295h           DisposeXBandNews
```

**Database\GameDB.c - Network Game Database (NGP)**

```
  296h           GetNGPListGamePatchInfo
  297h           GetNGPListGamePatchVersion
  298h           GetNGPVersion
  299h           UpdateNGPList
  29Ah           UpdateNameList
  29Bh           GetGameName
```

**Database\Personification.c**

```
  29Ch           ChangeUserPersonificationPart
  29Dh           InstallOpponentPersonification
  29Eh           GetPersonificationPart
  29Fh           PutPersonificationOnWire
  2A0h           GetPersonificationFromWire
  2A1h           DisposePersonificationSetup
  2A2h           ReceivePersonficationBundle
  2A3h           ParsePersonificationBundle
  2A4h           CreatePersonificationBundle
```

**Database\Mail.c - MailCntl**

```
  2A5h           CountInBoxEntries
  2A6h           CountOutBoxEntries
  2A7h           AddMailToOutBox
  2A8h           AddMailToInBox
  2A9h           RemoveMailFromInBox
  2AAh           GetIndexInBoxMail
  2ABh           GetIndexOutBoxMail
  2ACh           GetInBoxGraphicID
  2ADh           MarkMailItemRead
  2AEh           DeleteAllOutBoxMail
  2AFh           GetInBoxTypeForCurrentUser
  2B0h           GetOutBoxTypeForCurrentUser
  2B1h           GetOutBoxIDFromIndex
  2B2h           GetInBoxIDFromIndex
  2B3h           GetBoxIDFromIndex
```

**Database\SendQ.c - Send Queue or so?**

```
  2B4h           AddItemToSendQ
  2B5h           AddItemSizeToSendQ
  2B6h           DeleteSendQ
  2B7h           KillSendQItem
  2B8h           GetFirstSendQElementID
  2B9h           GetNextSendQElementID
  2BAh           CountSendQElements
  2BBh           GetSendQElement
  2BCh           RemoveItemFromSendQ
```

**UserInterface\DialogMgr.c**

```
  2BDh           SetDialogColors
  2BEh           DoDialog
  2BFh           DialogParameterText
  2C0h           DoDialogItem
  2C1h           DoDialogParam
  2C2h           DoPlayAgainDialog
  2C3h           CopyString
  2C4h           DoAnyResponse
  2C5h           DoDataDrivenDismissal
  2C6h           DoPassword
  2C7h           DrawDialogFrame
  2C8h           FillTextRectangle
  2C9h           HorizontalLine
  2CAh           KillProgressTimer
  2CBh           ReplaceParameters
  2CCh           SetupProgressTimer
  2CDh           VerticalLine
  2CEh           CreateShiners
  2CFh           DisposeShiners
```

**Sourceless - Fred Chip Hardware**

```
  2D0h           SetVector
  2D1h           SetVectorTblAddr
  2D2h           SetSafeRamSrc
  2D3h           SetSafeRomSrc
  2D4h 323h 33Dh SetLEDs
  2D5h           SetLEDScreenAnimation
```

**Sourceless - Joggler**

```
  2D6h           InitJoggler
  2D7h           DisplayJoggler
  2D8h           StopJoggler
```

**Database\DeferredDialogMgr.c**

```
  2D9h           QDefDialog
  2DAh           ShowDefDialogs
  2DBh           CountDefDialogs
  2DCh           DisableDefDialogs
  2DDh           EnableDefDialogs
```

**Sourceless - Misc**

```
  2DEh           CheckNetRegister
  2DFh           NetRegister
  2E0h           NetRegisterDone
  2E1h           SetNetTimeoutValue
  2E2h           GetNetTimeoutValue
  2E3h           GetNetWaitSoFar
  2E4h           NetRegisterTimeOutTimeProc
  2E5h           IsBoxNetRegistered
  2E6h           GetNetRegisterCase
```

**Database\Capture.c - Session Capture (not actually implemented?)**

```
  -              BeginSession
  -              DeleteSession
  -              EndSession
  -              BeginStreamCapture
  -              AddDataToStream
```

**Database\Playback.c - Session Playback (not actually implemented?)**

```
  -              BeginSessionPlayback
  -              SessionExists
  -              PlaybackNextStream
  -              PlaybackCurrentStream
  -              PlaybackPreviousStream
  -              DoesNextSessionStreamExist
  -              DoesPreviousSessionStreamExist
```

**GameLib\Synch.c - Synch (not actually implemented?)**

```
  -              SynchModems
  -              SynchVbls
```

**Sourceless - Game Talk Session?**

```
  2E7h           GTSInit
  2E8h           GTSShutdown
  2E9h           GTSFlushInput
  2EAh           GTSessionPrefillFifo
  2EBh           GTSessionEstablishSynch
  2ECh           GTSessionExchangeCommands
  2EDh           GTSessionValidateControl
  2EEh           GTSErrorRecover
  2EFh           GTSCloseSessionSynch
  2F0h           GTSDoCommand
  2F1h           GTSDoResend
  2F2h           GTSResendFromFrame
  2F3h           GTSSetPacketFormat
  2F4h           GTSSetRamRomOffset
  2F5h           GTSessionSetLatency
  2F6h           GTSessionSendController8
  2F7h           GTSessionReadController8
  2F8h           GTSessionSendController12
  2F9h           GTSessionReadController12
  2FAh           GTSessionSendController16
  2FBh           GTSessionReadController16
  2FCh           GTSessionSendController18
  2FDh           GTSessionReadController18
  2FEh           GTSessionSendController24
  2FFh           GTSessionReadController24
  300h           GTSessionSendController27
  301h           GTSessionReadController27
```

**Sourceless - Game Talk Modem?**

```
  302h           GTModemInit
  303h           GTModemGetModemError
  304h           GTModemClearFifo
  305h           GTModemClockInByte
  306h           GTModemClockOutByte
  307h           GTModemAbleToSend
  308h           GTModemSendBytes
  309h           GTModemCheckLine
  30Ah           GTModemReadModem
  30Bh           GTSendReceiveBytes
  30Ch           GTCloseSessionSafe
  30Dh           GTCreateLooseSession
  30Eh           GTLooseSessionIdle
  30Fh           GTCloseLooseSession
  310h           GTSyncotron
  311h           GTMasterCalculateLatency
  312h           GTSlaveCalculateLatency
  313h           GTSyncoReadModemVBL
  314h           GTSyncronizeVBLs
  315h           GTSyncronizeMasterLeave
  316h           GTSyncronizeSlaveLeave
  317h           GTSyncoTronVBLHandler
  318h           GTUnused1
  319h           GTUnused2
  31Ah           GTUnused3
  31Bh           GTUnused4
  31Ch           GTUnused5
  31Dh           GTUnused6
```

**UserInterface\Keyboard.c - Keyboard**

```
  31Eh           SetupKeyboardEntryLayout
  31Fh           DisposeKeyboardEntryLayout
  320h           DoKeyboardEntry
  321h           InitKeyboardEntry
  322h           SendCommandToKeyboard
  323h           FinishKeyboardEntry
  324h           RefreshKeyboard
  325h           StuffCurrentKeyboardField
  326h           SelectKeyboardField
  327h           SendCommandToChatKeyboard
  328h           GetKeyLayoutFieldCount
  329h           GetKeyLayoutFieldSize
  32Ah           SetKeyboardEntryMeasureProc
  32Bh           SetFocusField
  32Ch           DrawKeyboard
  32Dh           ComputeCursorLineNumber
  32Eh           CacheKeyboardGraphics
  32Fh           ReleaseKeyboardGraphicsCache
```

**Sourceless - Smart Card**

```
  330h           GetCardType
  331h           CardInstalled
  332h 381h      ReadCardBytes       ;read smart card byte(s)
  333h           WriteCardBit
  334h           GotoCardAddress
  335h           IncrementCardAddress
  336h 385h      ReadCardBit         ;read smart card bit
  337h           ResetCard
  338h           PresentSecretCode
  339h           GetRemainingCredits
  33Ah           FindFirstOne
  33Bh           CountCardBits
  33Ch           DebitCardForConnect
  33Dh           DebitSmartCard
  33Eh           CheckValidDebitCard
  33Fh           IsGPM896
  340h           IsGPM103
  341h           IsGPM256
  342h           Debit896Card
  343h           Debit103Card
  344h           Get896Credits
  345h           Get103Credits
  346h           CheckWipeCard
  347h           UserWantsToDebitCard
```

**Sourceless - Sort**

```
  348h           QSort
```

**UserInterface\Secrets.c**

```
  349h           TrySecretCommand
  34Ah           TestThisSequence
  34Bh           ExecCommands
  34Ch           GetSecretList
  34Dh           GetSecretSequence
  34Eh           ResetSecretCommand
  34Fh           TestSequence
  350h           PlayMaze
  351h           EndPlayMaze
```

**Sourceless - Maths**

```
  352h           LongDivide
  353h           LongMultiply
  354h           Sqrt
  355h           RandomShort
  356h           Sine
  357h           Cosine
```

**Database\RankingMgr.c - Ranking**

```
  358h           GetFirstRanking
  359h           GetNextRanking
  35Ah           GetPrevRanking
  35Bh           GetHiddenStat
  35Ch           NextRankingExists
  35Dh           PrevRankingExists
  35Eh           CountRankings
  35Fh           GetFirstRankingID
  360h           GetNextRankingID
  361h           GetUniqueRankingID
  362h           GetRankingSize
  363h           DeleteRanking
  364h           AddRanking
  365h           GetRanking
```

**Graphics\Progress.c - Progress Bar Manager**

```
  366h           InitProgressProcs
  367h           SpawnProgressProc
  368h           DisposeProgressProc
  369h           SetProgressPosition
  36Ah           ProgressIdle
```

**UserInterface\RadioButtons.c**

```
  36Bh           SetupRadioButton
  36Ch           DrawRadioButton
  36Dh           ActivateRadioButton
  36Eh           DeactivateRadioButton
  36Fh           RadioButtonSelectNext
  370h           RadioButtonSelectPrevious
  371h           RadioButtonGetSelection
  372h           RadioButtonSetSelection
  373h           RadioButtonIdle
  374h           DisposeRadioButtonRef
  375h           DrawRadioSelection
```

**Sourceless - Misc**

```
  376h           NetIdleFunc
  377h           CheckError
  378h 3D9h 3F8h ccitt_updcrc
```

**UserInterface\PeerConnect.c**

```
  379h           DoPeerConnection
  37Ah           ConnectToPeer
  37Bh           DisplayPeerInfo
  37Ch           DoSlavePeerConnect
  37Dh           DoMasterPeerConnect
  37Eh           PeerConnectionDropped
  37Fh           DoPeerRestoreOS
  380h           DoExchangePeerData
  381h           DoPeerDialog
  382h           Chat
  383h           PeerStartVBL
  384h           PeerStopVBL
  385h           PeerVBLHandler
```

**Sourceless - Fifo**

```
  386h           FifoInit
  387h           FifoActive
  388h           FifoWrite
  389h           FifoRead
  38Ah           FifoPeek
  38Bh           FifoPeekEnd
  38Ch           FifoAvailable
  38Dh           FifoRemaining
  38Eh           FifoSkip
  38Fh           FifoCopy
  390h           FifoChkSum
  391h           GetFifoIn
  392h           FifoLastCharIn
  393h           FifoUnwrite
  394h           FifoSize
  395h           FifoFlush
  396h           FifoUnread
  397h           FifoResetConsumption
  398h           FifoAdjustConsumption
```

**Database\Results.c - Result FIFO (not implemented?)**

```
  -              AddToResultFIFO
  -              ReplaceTopEntryOfResultFIFO
  -              GetTopEntryOfResultFIFO
  -              GetIndexEntryInResultFIFO
  -              CountEntriesInResultFIFO
```

**Database\FourWayMailView.c**

```
  -              FourWayMail stuff
```

**Sourceless - Misc**

```
  399h           AddVBLRequest
  39Ah           RemoveVBLRequest
  39Bh           VBLIdle
  39Ch           PatchRangeStart                              <--- ??
  39Dh           PatchRangeEnd = kPatchRangeStart + 50        <--- ???
  -    54xh      SNES table end
```

**X-Band GAME Functions (CALL E000CCh)**

The GAME functions are just aliases for the normal BIOS functions. The idea
seems to have been that the BIOS function numbering might change in later BIOS
revisions, which would cause compatibility issues for older game patches. As a
workaround, there's a separate GAME function table which contains copies of
some important BIOS function vectors (and which is probably intendend to
maintain fixed function numbers even in later BIOS revisions).

The GAME functions are invoked via CALL E000CCh, with X=function_number
(0000h..004Dh on SNES/US).

The Game Function numbers for Sega are enumerated (among others) in
"Database\GamePatch.h". The Game Function table is initialized by
"CreateGameDispatcher" (which is using a lot of "InstallGameFunction" calls to
transfer the separate function vectors from BIOS table to GAME table).

```
  Sega SNES SNES Function
  Gen. US   JP
```

**general game stuff**

```
  00h  00h?      kOSHandleGameError
  01h  01h?      kOSGameOver
```

**basic os stuff**

```
  02h            kOSNewMemory
  03h            kOSDisposeMemory
  04h            kOSDelayTicks
```

**hardware stuff**

```
  05h            kOSSetSafeRomSrc
  06h            kOSSetSafeRamSrc
  07h            kOSSetVectorTableAddr
  08h            kOSSetVector
  09h  12h       kOSSetLEDs
```

**PModem**

```
  0Ah            kOSReadSerialByte
  0Bh            kOSWriteSerialByte
  0Ch            kOSReceiveBufferAvail
  0Dh            kOSTransmitBufferFree
  0Eh            kOSCheckLine
  0Fh            kOSDetectLineNoise
  10h            kOSCheckCarrier
  11h            kOSListenToLine
  12h            kOSSetTimerTicks
  13h            kOSTimerExpired
  14h            kOSToneMode
  15h  20h       kOSReadXRAM
  16h  21h       kOSReadYRAM
  17h            kOSWriteXRAM
  18h            kOSWriteYRAM
```

**gametalk**

```
  19h            kOSGTSSetPacketFormat
  1Ah            kOSGTSSetRamRomOffset
  1Bh            kOSGTSessionSetLatency
  1Ch            kOSGTSessionPrefillFifo
  1Dh            kOSGTSessionEstablishSynch
  1Eh            kOSGTSErrorRecover
  1Fh            kOSGTSCloseSessionSynch
  10h            kOSGTSFlushInput
  11h            kOSGTSessionValidateControl
  12h            kOSGTSessionExchangeCommands
  13h            kOSGTSDoCommand
  14h            kOSGTSDoResend
  15h            kOSGTSResendFromFrame
  16h            kOSGTModemInit
  17h            kOSGTModemGetModemError
  18h            kOSGTModemClearFifo
  19h            kOSGTModemClockInByte
  1Ah            kOSGTModemClockOutByte
  1Bh            kOSGTModemAbleToSend
  1Ch            kOSGTModemSendBytes
  1Dh            kOSGTModemCheckLine
```

**controller should probably be in "hardware stuff"**

```
  1Eh            kOSInitControllers
  1Fh            kOSReadControllers
```

**stinkotron**

```
  20h            kOSGTSyncotron
  21h            kOSGTMasterCalculateLatency
  22h            kOSGTSlaveCalculateLatency
  23h            kOSGTSyncoReadModemVBL
  24h            kOSGTSyncronizeVBLs
  25h            kOSGTSyncronizeMasterLeave
  26h            kOSGTSyncronizeSlaveLeave
  27h            kOSGTSyncoTronVBLHandler
```

**keep this one**

```
  28h  4Eh       kOSLastFunction
```

### SNES Cart FLASH Backup

Most SNES games are using battery-backed SRAM for storing data, the only
exception - which do use FLASH memory - are the JRA PAT BIOS cartridges for the
SFC Modem:

There are two JRA PAT versions, the older one (1997) supports only AMD FLASH,
the newer one (1999) supports AMD/Atmel/Sharp FLASH chips.

```
  ID=2001h - AM29F010 AMD (128Kbyte)      ;supported by BOTH bios versions
  ID=D51Fh - AT29C010A Atmel (128Kbyte)   ;supported only by newer bios version
  ID=32B0h - LH28F020SUT Sharp (256Kbyte?);supported only by newer bios version
```

The FLASH Size size defined in entry [FFBCh] of the Cartridge Header (this is
set to 07h in JRA PAT, ie. "(1K SHL 7)=128Kbytes").

There don't seem to be any data sheets for the Sharp LH28F020SUT-N80 chip (ID
B0h,32h) (so not 100% sure if it's really 256Kbytes), anyways, it does somehow
resemble LH28F020SU-N (5V, ID B0h,30h) and LH28F020SU-L (5V/3.3V, ID B0h,31h).

**JRA PAT Memory Map**

```
  80h-9Fh:8000h-FFFFh  ;1Mbyte LoROM (broken into 32 chunks of 32Kbytes)
  C0h-C3h:0000h-7FFFh  ;128Kbyte FLASH (broken into 4 chunks of 32Kbytes)
```

AMD FLASH

**Get Device ID (Type 1 - AMD)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=90h           ;enter ID mode
  manufacturer=01h=[C00000h], device_type=20h=[C00001h] ;read ID (AM29F010)
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h           ;terminate command
```

**Erase Entire Chip (Type 1 - AMD)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h           ;prepare erase
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=10h           ;erase entire chip
  repeat, stat=[C00000h], until stat.bit7=1=okay, or stat.bit5=1=timeout
```

**Erase 16Kbyte Sector (Type 1 - AMD)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h           ;prepare erase
  [C05555h]=AAh, [C02AAAh]=55h, [Cxx000h]=30h           ;erase 16kbyte sector
  repeat, stat=[Cxx000h], until stat.bit7=1=okay, or stat.bit5=1=timeout
```

**Write Single Data Byte (Type 1 - AMD)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=A0h           ;write 1 byte command
  [Cxxxxxh]=dta                                         ;write the data byte
  repeat, stat=[Cxxxxxh], until stat.bit7=dta.bit7=okay, or stat.bit5=1=timeout
```

**Notes**

After AMD timeout errors, one should issue one dummy/status read from [C00000h]
to switch the device back into normal data mode (at least, JRA PAT is doing it
like so, not too sure if that is really required/correct).

ATMEL FLASH

**Get Device ID (Type 2 - Atmel)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=90h           ;enter ID mode
  manufacturer=1Fh=[C00000h], device_type=D5h=[C00001h] ;read ID (AT29C010A)
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h           ;terminate command
```

**Erase Entire Chip (Type 2 - Atmel)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h           ;prepare erase
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=10h           ;erase entire chip
  wait two frames (or check if bit6 toggles on each read from [C00000h])
```

**Erase 16Kbyte Sector (Type 2 - Atmel)**

```
  No such command (one can write data without erasing)
  (to simulate a 16K-erase: write 128 all FFh-filled 128-byte blocks)
```

**Write 1..128 Data Bytes (within 128-byte boundary) (Type 2 - Atmel)**

```
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=A0h           ;write 1..128 byte(s)
  [Cxxxxxh+0..n]=dta[0..n]                              ;write the data byte(s)
  repeat, stat=[Cxxxxxh+n], until stat=dta[n]           ;wait last written byte
```

**Notes**

The JRA PAD functions do include a number of wait-vblank delays between various
ATMEL commands, that delays aren't shown in above flowcharts.

SHARP FLASH

**Get Device ID (Type 3 - Sharp)**

```
  [C00000h]=90h                                         ;enter ID mode
  manufacturer=B0h=[C00000h], device_type=32h=[C00001h] ;read ID (LH28F020SUT)
  [C00000h]=FFh                                         ;terminate command
```

**Set/Reset Protection (Type 3 - Sharp)**

```
  [C00000h]=57h/47h, [C000FFh]=D0h    ;<-- C000FFh (!)  ;set/reset protection
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
```

**Erase Entire Chip (Type 3 - Sharp)**

```
  [C00000h]=A7h, [C00000h]=D0h                          ;erase entire chip
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
```

**Erase 16Kbyte Sector (Type 3 - Sharp)**

```
  [C00000h]=20h, [Cxx000h]=D0h                          ;erase 16kbyte sector
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
  if failed, issue "Reset Protection", and retry
```

**Write Single Data Byte (Type 3 - Sharp)**

```
  [C00000h]=40h                                         ;write 1 byte command
  [Cxxxxxh]=dta                                         ;write the data byte
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  ;below error-check & terminate are needed only after writing LAST byte
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
```

PCB VERSIONS

**Older PCB "SHVC-1A9F-01" (1996) (DIP) (for JRA-PAT and SPAT4)**

```
  U1 32pin ROM
  U2 32pin AMD AM29F010-90PC (FLASH)
  U3 16pin SN74LS139AN
  U4 16pin D411B (CIC)
```

**Newer PCB "SHVC-1A8F-01" (1999) (SMD) (for JRA-PAT-Wide)**

```
  U1 32pin ROM
  U2 32pin Sharp LH28F020SUT-N80 (FLASH)
  U3 16pin 74AC139
  U4 18pin F411B (CIC)
  U5 14pin 74AC08
```

**See Also**

Another approach for using FLASH backup is used in carts with Data Pack slots:

### SNES Cart Cheat Devices

**Code Format Summary**

```
  Pro Action Replay         AAAAAADD        raw 8-digits         WRAM
  Pro Action Replay Mk2/Mk3 AAAAAADD        raw 8-digits         WRAM/ROM/SRAM
  X-Terminator/Game Wizard  AAAAAADD        raw 8-digits         WRAM
  Game Genie/Game Mage      DDAA-AAAA       encrypted 4-4 digits ROM/SRAM
  Gold Finger               AAAAADDDDDDCCW  raw 14-digits        DRAM/SRAM
  Front Far East            NNAAAAAADD..    raw 10..80 digits    DRAM offset
```

**Code Format Details**

**Hardware Details**

**Cheat Devices & Number of Hardware/Software patches & Built-in codes**

```
  Name                            Hardware/ROM  Software/WRAM  Built-in
  Pro Action Replay               None  (of 4)  4              None
  Pro Action Replay Mk2a/b        0/2/4 (of 4)  100            None
  Pro Action Replay Mk3           1/5   (of 7)  100            ? games
  Game Genie (Codemasters/Galoob) 5     (of 6)  None           None
  Game Mage (Top Game & Company)  8?            None?          250 codes?
  X-Terminator (Fire)             None  (of 0)  4              None
  X-Terminator 2 (noname)         None  (of 0)  64             307 games
  Game Wizard (Innovation)        None?         ?              ?
  Game Saver (Nakitek) allows to save WRAM/VRAM snapshots in non-battery DRAM
  Game Saver+ (Nakitek) allows to save WRAM/VRAM snapshots in battery DRAM
  Super UFO (copier, supports Gold Finger and X-Terminator codes)
  Super Wild Card/Magicom (copiers, support Gold Finger and Front Far East)
  Parame ROM Cassette Vol 1-5 (by Game Tech) (expansions for X-Terminator 2)
```

Note: The Game Mage's stylished "GAME|~AGE)" logo is often misread as
"Gametaged".

**Links**

http://www.gamegenie.com/cheats/gamegenie/snes/index.html

http://www.world-of-nintendo.com/pro_action_replay/super_nes.shtml

http://www.gamefaqs.com/snes/562623-harvest-moon/faqs/10690

http://www.gamefaqs.com/snes/588741-super-metroid/faqs/5667

### SNES Cart Cheat Devices - Code Formats

**PAR AAAAAADD - Normal Pro Action Replay Codes (Datel)**

The Pro Action Replay is a cheat device for the SNES produced by Datel. The
original PAR only support 3 codes, but the PAR2 supports 255 and has a built-in
trainer for code searcher. There is also a PAR3, but the added features are
unknown.

```
  AAAAAADD  ;-address (AAAAAA) and data (DD)
```

Address can be a ROM, SRAM, or WRAM location. Patching cartridge memory (both
ROM and SRAM) is implemented by hardware (supported by PAR2-PAR3 only, not by
PAR1 or X-Terminator 1-2). Patching WRAM is done by software (rewriting the
values on each Vblank NMI). WRAM addresses must be specified as 7E0000h-7FFFFFh
(mirrors at nn0000h-nn1FFFh aren't recognized by the BIOSes).

**PAR 7E000000 - Do nothing**

This is the most important PAR code (required as padding value, since the GUI
doesn't allow to remove items from the code list):

**PAR FE0000xx..FFFFFFxx - Pre-boot WRAM patch (PAR1 only)**

Writes xx to the corresponding WRAM address at 7E0000h..7FFFFFh, this is done
only once, and it's done BEFORE starting the game (purpose unknown - if any).

**PAR 00600000 - Disable Game's NMI handler (PAR3 only)**

Disables the game's NMI handler (executes only the NMI handler of the PAR
BIOS).

**PAR DEADC0DE - Special Multi-Byte Code Prefix (PAR2a/b and PAR3 only)**

Allows to hook program code, this feature is rarely used.

```
  DEADC0DE  ;-prefix (often misspelled as "DEADCODE", with "O" instead "0")
  AAAAAANN  ;-address (AAAAAA) and number of following 4-byte groups (NN)
  DDEEFFGG  ;-first 4-byte group    (DD=1st byte, .. GG=4th byte)
  HHIIJJKK  ;-second 4-byte group   (HH=5th byte, .. KK=8th byte) (if any)
  ...       ;-further 4-byte groups (etc.)                        (if any)
```

The data portion (DD,EE,FF..) (max 62h*4 = 188h bytes) is relocated to SRAM (in
the PAR cartridge), and the ROM address AAAAAA is patched by a 4-byte "JMP
nnnnnn" opcode (doing a far jump to the address of the relocated SRAM code;
this would be at 006A80h in PAR3, at 006700h in PAR2a/b, and isn't supported in
PAR1). There seems to be no special action required when returning control to
the game (such like disabling the SRAM - if the hardware does support that at
all?) (or such actions are required only for HiROM games that have their own
SRAM at 6000h?) (or games are typically accessing SRAM at 306xxxh, so there is
no conflict with PAR memory at 006xxxh?).

One can use only one DEADC0DE at a time, and, when using it, there are some
more restrictions: On PAR2a/b one cannot use ANY other hardware/software
patches. On PAR3 one can keep using ONE hardware patch (and any number of
software patches).

**PAR C0DEnn00 - Whatever (X-Terminator 2 only - not an official PAR code)**

Somehow changes the NMI (and IRQ) handling of the X-Terminator 2, "nn" can be
00..06.

**Game Genie Codes (Codemasters/Galoob)**

```
  DDAA-AAAA  ;-encrypted data (DD) and encrypted/shuffled address (AA-AAAA)
```

Address can be a ROM, or SRAM location (internal WRAM isn't supported). To
decrypt the code, first replace the Genie Hex digits by normal Hex Digits:

```
  Genie Hex:    D  F  4  7  0  9  1  5  6  B  C  8  A  2  3  E
  Normal Hex:   0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
```

Thereafter, DD is okay, but AAAAAA still needs to be deshuffled:

```
  ijklqrst opabcduv wxefghmn   ;Genie Address (i=Bit23 ... n=Bit0)
  abcdefgh ijklmnop qrstuvwx   ;SNES Address (a=Bit23 ... x=Bit0)
```

Aside from being generally annoying, the encryption makes it impossible to make
codes like "Start in Level NN" (instead, one would need to make separate codes
for each level).

Game Genie codes can be reportedly also used with the Game Mage. And, the PAR3
includes a "CONV" button for converting Game Genie codes. When manually
decrypting them, Game Genie codes would also work on PAR2 (although the PAR2
won't allow to use five ROM patches at once).

**Gold Finger / Goldfinger Codes (unknown who created this format)**

These codes are rarely used, and there isn't much known about them. Reportedly,
they have been supported by "certain copiers" (unknown which ones... Super UFO,
and also copiers from Front Far East?).

```
  AAAAADDEEFFCCW  ;-Address (AAAAA), Data (DD,EE,FF), Checksum (CC), Area (W)
```

The Address is a ROM address, not a CPU address. Data can be 1-3 bytes, when
using less than 3 bytes, pad the EE,FF fields with "XX" (and treat them as zero
in the checksum calculation). Checksum is calculated as so:

```
  CC = A0h + AAAAA/10000h + AAAAA/100h + AAAAA + DD (+ EE (+ FF))
```

W tells the copier whether to replace the byte in the DRAM (ROM image) or the
SRAM (Saved game static RAM) of the copier:

```
  W=0  DRAM (ROM image) (reportedly also for W=2,8,A,C,F)
  W=1  SRAM (Saved game image)
```

The 5-digit address allows to access max 1Mbyte. The address is an offset
within the ROM-image (excluding any 200h-byte header). The first LoROM byte
would be at address 00000h. Some copiers are using interleaved HiROM images -
unknown if any such interleave is used on code addresses - if so, first HiROM
byte would be at "ROMsize/2" (in middle of ROM-image), otherwise it'd be at
00000h (at begin of ROM-image).

Note: It doesn't seem to be possible to enter "X" digits in all copiers. Double
Pro Fighter Q allows to enter "X".

**Front Far East Codes (Front Far East)**

Supported by Front Far East copiers (Super Magicom and/or Super Wild Card?).
This format is even less popular than the Gold Finger format.

```
  NNAAAAAADD..    Number of bytes (NN), Address (AAAAAA), Data (DD..)
```

Allows to change 1..36 bytes; resulting in a code length of 10..80 digits (to
avoid confusion with 14-digit Gold Finger codes, Front Far East wants Gold
Finger codes to be prefixed by a "G").

AAAAAA is a 24bit offset within the ROM-image (excluding any 200h-byte header).
As far as known, Front Far East didn't use interleaved ROM-images, so the
offset should be straight.

### SNES Cart Cheat Devices - Game Genie

**Game Genie BIOS Versions**

There are at least three BIOS versions, named "GENSRC", "K7", and "ed":

```
  "GENSRC"  32Kbytes LoROM, straight I/O addresses  CRC32=AC94F94Ah
  "K7"      32Kbytes LoROM, messy I/O addresses     CRC32=F8D4C303h
  "ed"      64Kbytes LoROM, messy I/O addresses     CRC32=58CBF2FEh
```

The names are stored at ROM-offset 7FE0h (aka SNES address 00FFE0h). Most of
the cartridge header contains garbage (no checksum, wrong ROM size, etc.), only
the 21-byte title field is (more or less) correct:

```
  "Game Genie      ",0,0,0,0,0   ;32K versions (GENSRC & K7)
  "Game Genie         Jo"        ;64K version (ed)
```

Note: All three versions support only 5 codes to be entered.

**Game Genie I/O Ports**

The "GENSRC" version uses quite straight I/O addresses, the "K7" and "ed"
versions have the addresses messed up (unused gaps between the codes, several
mirrors, of which, mirrors with address bit8 and bits16-23 all zero are having
address bit0 inverted).

```
  Version   "GENSRC"          "K7" & "ed"
  Control   W:008000h         W:xx8100h, W:008001h  ;ed: xx=00, K7: xx=FF
  CodeFlags R/W:008001h       R:FF8001h, W:008000h  ;bit 0-4 = enable code 1-5
  CodeMsb   R/W:008003h+N*4   R:FF8005h+N*6, W:008004h+N*6  ;\
  CodeMid   R/W:008004h+N*4   R:FF8006h+N*6, W:008007h+N*6  ; N=0-4 for
  CodeLsb   R/W:008005h+N*4   R:FF8007h+N*6, W:008006h+N*6  ; code 1-5
  CodeData  R/W:008006h+N*4   R:FF8008h+N*6, W:008009h+N*6  ;/
```

Other (accidently) used I/O addresses are: W:004017h (bugged joypad access),
and W:00FFEAh/00FFEBh (used in an attempt to install a bugged NMI handler with
RET instead of RETI opcode; the hardware is hopefully ignoring that attempt).

**Control Register**

Allows to select what is mapped to memory. Used values are:

```
  00h Select Game Genie BIOS
  02h Select Game Genie I/O Ports
  06h Select Game Cartridge
  07h Select Game Cartridge and keep it selected
```

This register is set to 00h on /RESET (warmboot & coldboot).

**Code Flags Register**

Used to enable the 5 codes:

```
  Bit0-4  Enable Code 1..5  (0=Disable, 1=Enable)
  Bit5-7  Should be zero
```

This register is set to 00h on initial power-up (coldboot), but kept intact on
/RESET (warmboot), allowing to restore the previously enabled codes.

**CodeAddress/CodeData Registers (5*4 bytes)**

Contains the 24bit address and 8bit data values (in decrypted form). The
registers are kept intact on /RESET (warmboot), allowing to restore the
previously entered codes - however, when restoring the codes, the "K7" and "ed"
BIOS versions are ORing the LSB of of the 24bit address value with 01h, thereby
destroying all codes with even addresses (unknown why that's been done).

**Chipset**

The exact chipset is unknown, there should be the ROM and some logic (and no
SRAM). There is also a LED and a 2-position (?) switch (unknown function).

**XXX**

For more in-depth info see "genie.txt" from Charles MacDonald.

```
  http://cgfm2.emuviews.com/txt/genie.txt
```

**Game Genie 2 (prototype)**

There is also an unreleased Game Genie 2 prototype, the thing includes a small
LCD screen, five push buttons, four LEDs, battery backed SRAM, a 8255 PIO, a
huge ACTEL chip, and some expansion connectors.

### SNES Cart Cheat Devices - Pro Action Replay I/O Ports

**PAR1 I/O Ports (W)**

```
  008000h                  ;Code 0-3 (MID) (shared for code 0..3)
  010000h,010001h,010002h  ;Code 0 (DTA,LSB,MSB) (not used by BIOS)
  010003h                  ;Control (set to FFh)
  010004h,010005h,010006h  ;Code 1 (DTA,LSB,MSB) (not used by BIOS)
  010007h,010008h,010009h  ;Code 2 (DTA,LSB,MSB) (used as NMI vector.LSB)
  01000Ah,01000Bh,01000Ch  ;Code 3 (DTA,LSB,MSB) (used as NMI vector.MSB)
```

Most I/O ports are overlapping WRAM in bank 01h (unlike PAR2-PAR3 which use
bank 10h). The four code registers would allow to apply 4 hardware patches, the
PAR1 BIOS actually has provisions for doing that, but, before applying those
patches it erases code 0-3 (and does then use code 2-3 for patching the NMI
vector at 00FFEAh for applying the codes as WRAM software patches).

The PAR1 supports only LoROM addresses (address bit15 removed, and bit23-16
shifted down). The PAR1 does not (maybe cannot) disable unused codes; instead,
it directs them to an usually unused ROM location at 00FFF6h (aka 007FF6h after
removing bit15). Applying the codes is done in order DTA,MID,LSB,MSB, whereof,
the "shared" MID value is probably applied on the following LSB port write.

The control register is set to FFh before starting the game, purpose is unknown
(maybe enable the codes, or write-protect them, or disable the BIOS ROM; in
case that can be done by software).

**PAR2 I/O Ports (W)**

```
  100000h,100001h,100002h,100003h  ;Code 0 (DTA,LSB,MID,MSB) (code 0)
  100004h,100005h,100006h,100007h  ;Code 1 (DTA,LSB,MID,MSB) (code 1)
  100008h,100009h,10000Ah,10000Bh  ;Code 2 (DTA,LSB,MID,MSB) (code 2/NMI.LSB)
  10000Ch,10000Dh,10000Eh,10000Fh  ;Code 3 (DTA,LSB,MID,MSB) (code 3/NMI.MSB)
  100010h      ;Control A (set to 00h or FFh)
  C0A00nh      ;Control B (address LSBs n=0..7) (written data=don't care)
```

The registers overlapping the WRAM area are similar as for PAR1. The PAR2 BIOS
allows to use the code registers for ROM patches (and/or hooking the NMI
handler for WRAM patches).

Similar as in PAR1, address bit23-16 are shifted down, but with bit15 being
moved to bit23 (still a bit messy, but HiROM is now supported). Unused codes
are redirected to 00FFF6h (aka 807FF6h after moving bit15).

The control register is set to FFh before starting the game, purpose is unknown
(maybe enable the codes, or write-protect them, or disable the BIOS ROM; in
case that can be done by software).

The lower address bits of the newly added C0A00nh Register are:

```
  Address bit0 - set if one or more codes use bank 7Fh..FEh
  Address bit1 - set/cleared for PAL/NTSC selection (or vice-versa NTSC/PAL?)
  Address bit2 - set to... maybe, forcing the selection in bit1 (?)
```

The exact purpose of that three bits is unknown (and their implemention in
PAR2a/b BIOSes looks bugged). Doing anything special on bank 7Fh-FEh doesn't
make any sense, maybe the programmer wanted to use banks 80h-FFh, but that
wouldn't make much more sense either; it might be something for
enabling/disabling memory mirrors or so. The default PAL/NTSC flag is
auto-detected by reading 213Fh.Bit4 (the BIOS is doing that detection twice,
with opposite results on each detection, which seems to be a bug), the flag can
be also manually changed in the BIOS menu; purpose of the PAL/NTSC thing is
unknown... maybe directing a transistor to shortcut D4 to GND/VCC when games
are reading 213Fh.

**PAR3 I/O Ports**

```
  100000h,100001h,100002h,100003h  ;Code 0 (DTA,LSB,MID,MSB) (code 0)
  100004h,100005h,100006h,100007h  ;Code 1 (DTA,LSB,MID,MSB) (code 1)
  100008h,100009h,10000Ah,10000Bh  ;Code 2 (DTA,LSB,MID,MSB) (code 2)
  10000Ch,10000Dh,10000Eh,10000Fh  ;Code 3 (DTA,LSB,MID,MSB) (code 3)
  100010h,100011h,100012h,100013h  ;Code 4 (DTA,LSB,MID,MSB) (code 4)
  100014h,100015h,100016h,100017h  ;Code 5 (DTA,LSB,MID,MSB) (always NMI.LSB)
  100018h,100019h,10001Ah,10001Bh  ;Code 6 (DTA,LSB,MID,MSB) (always NMI.MSB)
  10001Ch         ;Control A    (bit4,6,7)
  10001Dh-10001Fh ;Set to zero  (maybe accidently, trying to init "code 7")
  10003Ch         ;Control B    (set to 01h upon game start)
  086000h         ;Control LEDs (bit0,1)
  206000h         ;Control C    (bit0)
  008000h         ;Control D    (set to 00h upon PAR-NMI entry)
```

Control A (10001Ch):

```
  Bit0-3 Should be 0
  Bit4   ROM Mapping (0=Normal, 1=Temporarily disable BIOS & enable GAME ROM)
  Bit5   Should be 0
  Bit6-7 Select/force Video Type (0=Normal, 1=NTSC, 2=PAL, 3=Reserved)
```

Control LEDs (086000h) (LEDs are in sticker area on front of cartridge):

```
  Bit0   Control left or right LED? (0=on or off?, 1=off or on?)
  Bit1   Control other LED          ("")
  Bit2-7 Should be 0
```

Control C (206000h):

```
  Bit0   Whatever (0=BIOS or PAR-NMI Execution, 1=GAME Execution)
  Bit1-7 Should be 0
```

Code0-6:

Unused codes are set to 00000000h (unlike PAR1/PAR2), and, codes use linear
24bit addresses (without moving/removing bit15). Of the seven codes, code 5-6
are always used for hooking the NMI handler (even when not using any WRAM
software patches), so one can use max 5 hardware ROM patches.

### SNES Cart Cheat Devices - Pro Action Replay Memory

**PAR1-PAR3 SRAM**

All PAR versions contain 32Kbytes SRAM, divided into 8K chunks, which are
unconventionally mapped to EVEN bank numbers.

```
  00/02/04/06:6000h..7FFFh      ;-32Kbyte SRAM (four 8K banks)
```

The SRAM is used as internal workspace (stack & variables, code list, NMI
handler, deadcode handler, and list of possible-matches for the code finder).

Unknown if the SRAM is battery backed (the way how it is used by the BIOS
suggests that it is NOT battery backed).

Note: Many HiROM games have their own SRAM mapped to 6000h-7FFFh, unknown
if/how/when the PAR can disable its SRAM for compatibility with such games
(PAR1 seems to be designed for LoROM games only, but newer PAR2-PAR3
<should> have HiROM support - if so, then the hardware must somehow
switch the SRAM on/off depending on whether it executes game code, or PAR code
like NMI & deadcode handlers).

**PAR1-PAR3 Switch**

All PAR versions do have a 3-position switch (on the right edge of the
cartridge). The way how Datel wants it to be used seems to be:

```
  1) Boot game with switch in MIDDLE position (maybe needed only for testing)
  2) Set LOWER position & push RESET button (to enter the BIOS menu)
  3) After selecting codes/cheat finder, start game with MIDDLE position
  4) Finally, UPPER position enables codes (best in-game, AFTER intro/menu)
```

Technically, the switch seems to work like so:

```
  UPPER Position   "Codes on"    Enable GAME and enable codes
  MIDDLE Position  "Codes off"   Enable GAME and disable codes
  LOWER Position   "Trainer On"  Enable BIOS and (maybe) enable codes
```

The "Codes off" setting may be required for booting some games (which may use
WRAM for different purposes during intro & game phases). The purpose of the
"Trainer" setting is unclear, GAME/BIOS mapping could be as well done via I/O
ports (and at least PAR3 does actually have such a feature for reading the GAME
header during BIOS execution).

There seems to be no I/O ports for sensing the switch setting, however, the
"Codes off" setting can be sensed by testing if the patches (namely the patched
NMI vector) are applied to memory or not.

**PAR BIOS Versions**

```
  Pro Action Replay Mk1 v2.1  1992        32K CRC32=81A67556h
  Pro Action Replay Mk2 v1.0  1992,93     32K CRC32=83B1D39Eh
  Pro Action Replay Mk2 v1.1  1992,93,94  32K CRC32=70D6B036h
  Pro Action Replay Mk3 v1.0U 1995       128K CRC32=0D7F770Ah
```

The two Mk2 versions are 99.9% same (v1.1 is only 10 bytes bigger than v1.0,
major change seems to be the copyright message).

Aside from v1.0/v1.1, there are reportedly further PAR2 BIOS versions (named
v2.P, v2.T, v2.H). Moreover, there's reportedly at least one localized BIOS (a
german PAR3 with unknown version number).

**PAR Component List**

The exact component list is all unknown. Some known components are:

```
  PAR1-3  3-position switch (on right edge of the cartridge)
  PAR1-3  32Kbytes SRAM (probably not battery-backed)
  PAR1-2  32Kbytes BIOS
  PAR3    128Kbytes BIOS (with modernized GUI and built-in "PRESET" codes)
  PAR1    46pin cartridge slot (incompatible with coprocessors that use 62pins)
  PAR2-3  62pin cartridge slot
  PAR2-3? second Npin cartridge slot at rear side (for CIC from other region)
  PAR3    two LEDs (within sticker-area on front of cartridge)
  PAR1-3  whatever logic chip(s)
```

### SNES Cart Cheat Devices - X-Terminator & Game Wizard

**Pro Action Replay (PAR1) clone**

Wide parts of the X-Terminator BIOS are copied 1:1 from a disassembled PAR1
BIOS. The similarities begin at the entrypoint (with some entirely useless
writes to 2140h-2143h), and go as far as using the ASCII characters "W H B ."
as default dummy data values for the 4 codes (the initials of the PAR1
programmer W.H.BECKETT). There are some differences to the original PAR1: The
hardware and I/O ports are a custom design, the GUI does resemble the PAR2
rather than the PAR1, and english words like "relation" are translated to odd
expressions like "differentship". Nonetheless, the thing was called back (in
some countries at least), presumably due to all too obvious copyright
violations.

**I/O Ports & Memory Map**

```
  X-Terminator 1         X-Terminator 2
  00FFE8h.W              00FFEAh.W            ;map BIOS (by writing any value)
  00FFE9h.W              00FFEBh.W            ;map GAME (by writing any value)
  00FFEAh.R (NMI read)   00FFEAh.R (NMI read) ;map BIOS/GAME (switch-selection)
  008000h-00FFFFh        008000h-00FFFFh      ;BIOS (32Kbytes)
  N/A                    028000h-02FFFFh      ;Expansion ROM 32Kbytes
  00,02,04,06:6000-7FFF  00-1F:02C00-2FFF     ;SRAM (32Kbytes)
```

Note: Both BIOS versions are confusingly using 16bit writes to the I/O ports in
some cases; the LSB-write to [addr+0] has no effect (or lasts only for 1 cpu
cycle), the MSB-write to [addr+1] is the relevant part.

The uncommon SRAM mapping in EVEN banks at 6000h-7FFFh was cloned from PAR. The
later mapping to 2C00h-2FFFh was probably invented for compatibility with HiROM
games that use 6000h-7FFFh for their own SRAM (or possibly just to look less
like a PAR clone).

Aside from NMI, the X-Terminator 2 is also using IRQ vectors (though unknown if
they are used only during BIOS execution or also during GAME execution, in
latter case reads from FFEEh would probably also trigger memory mapping).

**Game Wizard (by Innovation)**

The Game Wizard seems to be a rebadged X-Terminator. Unknown if the I/O
addresses are same as for X-Terminator 1 or 2.

**BIOS Versions**

There are at least two versions:

```
  X-Terminator    1993 (english)  (CRC32=243C4A53h) (no built-in codes)
  X-Terminator 2  19xx (japanese) (CRC32=5F75CE9Eh) (codes for 307 games)
```

There should be probably also a separate version for Game Wizard. And,
considering that the BIOS is stored on ERPOM, there might be many further
versions & revisions.

Cartridge header is FFh-filled (except for exception vectors), BIOS is 32Kbytes
LoROM.

**X-Terminator Expansion ROMs**

There have been at least 5 expansion cartridges released:

```
  Parame ROM Cassette Vol 1-5 (by Game Tech)
```

The cartridges contain 256Kbytes LoROM, and they can be used in two ways:

As normal executable (via normal ROM header at ROM-offset 7FC0h aka SNES
address 00FFC0h), or as cheat-code database extension for the X-Terminator 2
(via a special ROM header at ROM-offset 10000h aka SNES address 028000h).

```
  028000h - ID "FU O9149" (aka "UFO 1994" with each 2 bytes swapped)
  028008h - Boot callback (usually containing a RETF opcode)
  028010h - List of 16bit pointers (80xxh-FFFFh), terminated by 0000h
```

The 16bit pointers do address following structures (in ROM bank 02h):

```
  2   checksum (MSB,LSB) taken from GAME cartridge ROM header [FFDCh]
  1   number of following 5-byte codes (N)
  5*N codes (MID,MSB,DTA,LSB,TYPE)  ;TYPE=predefined description (00h..23h)
```

Unknown how the cartridges are intended to be connected (between X-Terminator
and Game cartridge... or maybe to a separate expansion slot).

**Super UFO Copier**

The Super UFO copiers are somehow closely related to X-Terminator (probably
both made by the same company). X-Terminator codes are supported by various
Super UFO versions. Later Super UFO versions also include/support Parame
expansion ROMs:

```
  Super UFO Pro-8 V8.8c BIOS
```

This versions seems to detect "FU O9149" IDs (ie. Parame carts), moreover, it
seems to include it's own "FU O9149" ID (but, strangely, at 048000h instead of
028000h, so the X-Terminator won't find it?).

**X-Terminator Chipset (whatever X-Terminator version)**

```
  Goldstar GM76C256ALL-70 (32Kbytes SRAM, not battery-backed)
  D27256 (32Kbytes UV-Eraseable EPROM)
  two logic chips & two PALs or so (part numbers not legible on existing photo)
  3-position switch (on PCB solder-side) (SCAN/NORMAL/ACTION)
  two cartridge slots (for PAL and NTSC cartridges or so)
```

### SNES Cart Cheat Devices - Game Saver

The Game Saver from Nakitek allows to load/save snapshots of (most of) the SNES
memory and I/O ports.

**Game Saver Controls (works with joypad in port 1 only)**

```
  L+R       upon boot --> test screen / version number
  L+R+START upon boot --> toggle slow DRAM checksumming on/off
  SELECT    in title  --> enter revival codes
  R+SELECT  in game   --> save state
  L+SELECT  in game   --> load state
  R+START   in game   --> toggle slow motion on/off  ;\one of these keeps
  L+START   in game   --> toggle slow motion on/off  ;/HDMA enabled (or so)
```

**Missing Save Data**

The SNES cannot directly access the APU, so APU RAM, DSP I/O Ports, and SPC700
registers aren't saved. The WRAM address (2181h-2183h) isn't saved. The
VRAM/OAM/CGRAM addresses are saved (but may have wrong values since the
autoincrement isn't handled). Any coprocessor I/O ports or cartridge SRAM
aren't saved.

**Hardware Versions (Game Saver and Game Saver+)**

The original Game Saver didn't have any power supply, which made (and still
makes) it the most controverse SNES add-on: Some people just like it, other
people are crying tears because they don't understand why the DRAM isn't
battery-backed.

This has led to the creation of the Game Saver Plus - a surreal product that
<does> use battery-backed DRAM (according to the booklet, where it is
called "portability" feature, six new AA batteries last 8-10 hours). Aside from
batteries, the Game Saver Plus is powered via the 9V DC supply of NTSC-SNES
consoles (even when the console itself is switched off). That's allowing to
switch off the SNES during supper in order to "save" energy (though after some
weeks, the permanently powered DRAM may negate that energy "saving" effect).

**BIOS Versions**

There seem to be several BIOS versions (DDMMYY formatted date and version
number are shown in the test screen). Known versions are:

```
  Game Saver v1.3 (19xx)
  Game Saver v1.7 (31 Jul 1995)
```

Unknown if Game Saver & Game Saver Plus use different BIOSes. Unknown if
any new/changed I/O ports were invented alongside with the BIOS versions.

**Game Saver Memory and I/O Map**

```
  002100h-0021xxh PPU ports (logged at 2081xxh) (or at 2080xxh on 2nd write)
  004200h-0042xxh CPU ports (logged at 2082xxh)
  008000h-00FFFFh BIOS ROM 32Kbytes
  0080xFh         Switch to GAME mapping (upon opcodes that end at 80xFh)
  00FFEAh         Switch to BIOS mapping (upon NMI execution; when enabled)
  108000h-108001h I/O - First/second write flags for write-twice PPU ports
  108002h-108003h I/O - Exception Mode/Status (bit0-1=BRK, bit2=NMI)
  208000h-2087FFh SRAM 2Kbytes (includes auto-logged writes to PPU/CPU ports)
  400000h-73FFFFh DRAM 256Kbytes (for saving WRAM/VRAM/OAM/CGRAM, CPU/DMA regs)
  808000h-80FFFFh GAME ROM (even while BIOS mapping is enabled)
```

The Game Saver can trap BRK or NMI exceptions. Of which, BIOS v1.7 seems to use
only BRKs (which are probably generated by outputting a 00h opcode in response
to joypad access, ie. [4218h] reads and [4017h]=01h writes).

**Game Saver Revival Codes**

The 5-digit "Revival Codes" are used to improve compatibility with different
games. Most commonly used are 2xxxx codes, which cause a byte in WRAM to be
left unchanged when loading data (probably in order to keep the Main CPU aware
of the state of the APU). The Code Format is:

```
  00000-0FFFF Blank (no action) (shown as "XXXXX" in GUI)
  10000-1FFFF Exception Mode (value for [108002]) (not used for any games)
  20000-3FFFF Preserve WRAM byte at 7E0000-7FFFFF (used for most games)
  40000-4FFFF PPU write-twice related     (used only for Starfox/Star Wing)
  50000-5FFFF PPU write-twice related     (not used for any games)
  60000-6FFFF Reserved (no action)        (not used for any games)
  70000-7FFFF Select special BRK handler  (used only for Aero the Acro Bat)
  80000-9FFFF Preserve WRAM byte at 7E0000-7FFFFF and pass it to 2140h ;\not
  A0000-BFFFF Preserve WRAM byte at 7E0000-7FFFFF and pass it to 2141h ; used
  C0000-DFFFF Preserve WRAM byte at 7E0000-7FFFFF and pass it to 2142h ; by any
  E0000-FFFFF Preserve WRAM byte at 7E0000-7FFFFF and pass it to 2143h ;/games
```

Codes (and updates) have been available as print-outs from Nakitek (the list
from 1995 contains codes for around 200 games; to be entered when pressing
SELECT in title screen). Moreover some (or all) BIOSes contain automatically
applied built-in codes (via checksumming portions of the game ROM header). The
v1.7 BIOS contains 284 codes (however, the code list does (maybe accidently)
contain an entry with NULL checksum, which causes the last 108 codes to be
ignored).

**Component List (Game Saver Plus)**

```
  24pin SRAM (2Kbytes) (probably used only because DRAM is too slow for I/O)
  28pin ROM/EPROM (32Kbytes)
  62pin cartridge slot (on rear side of device)
  14pin eight DRAM chips (256Kbytes in total)
  Xpin huge chip (whatever logic)
  3pin 7805 or so (for turning much of the 9 volts into heat)
  2pin oscillator (20.000MHz) (for DRAM refresh generator when power-off)
  socket/cable/plug for NTSC-SNES 9V DC supply (not PAL-SNES 9V AC supply)
  battery box for six 1.5V AA batteries, battery LED, and battery switch
  resistors, capacitors, and maybe diodes, transistors
```

**Note**

Some Copiers include a similar feature, allowing to load/save "real time saves"
on floppy disks and/or temporarily in unused portions of their built-in DRAM.

### SNES Cart Cheat Devices - Theory

**ROM Patches**

There are two possible ways for patching ROMs or ROM-images:

```
  1) rewrite ROM-image in RAM once before game starts        (GF/FFE/emulators)
  2) patch on ROM reading (by watching address bus)          (GG and PAR2-3)
```

Both are basically having same results, there may be some variations concerning
memory mirrors (depending on how the ROM-image in RAM is mirrored, or on how
the GG/PAR2-3 do decode the ROM address).

**WRAM Patches**

Implemented by rewriting WRAM upon NMI, variations would involve mirrors:

```
  1) allow WRAM addresses 7E0000-7FFFFF                      (PAR1-3, XT1-2)
  2) allow WRAM addresses 7E0000-7FFFFF and nn0000-nn1FFF    (N/A)
```

**SRAM Patches**

There are three possible ways for patching battery-backed SRAM:

```
  1) rewrite once before game starts                         (GF)
  2) patch on SRAM reading (like hardware based ROM patches) (GG and PAR2-3)
  3) rewrite repeatedly on NMI execution (like WRAM patches) (N/A)
```

SRAM is usually checksummed, so SRAM patches need to be usually combined with
ROM patches which do disable the checksum verification. Some devices (like Game
Genie) rely on the /ROMSEL signal, and thus probably can only patch SRAM in the
ROM area at 70xxxxh (but not in the Expansion area at 306xxxh).

**Slow Motion Feature**

Implemented by inserting delays in Vblank NMI handler. The feature can be
usually configured in the BIOS menu, and/or controlled via joypad button
combinations from within NMI handler.

**Cheat Finders (for WRAM Patches) (eventually also for SRAM patches)**

Implemented by searching selected values from within Vblank NMI handler, or
more simple: from within BIOS RESET handler. The search can be enabled/disabled
mechanically via switch, or in some cases, via joypad button combinations. The
searched value can be configured on RESET, or in some cases, via joypad button
combinations.

**Game Saver (Nakitek)**

Allows to save a copy of WRAM/VRAM and I/O ports (but not APU memory) in DRAM,
done upon joypad button-combinations sensed within BRK/NMI exception handlers.

### SNES Cart Tri-Star (aka Super 8) (allows to play NES games on the SNES)

The Tri-Star is an adaptor for playing NES games on the SNES (similar to the
Super Gameboy which allows to play Gameboy games on SNES). The thing have three
cartridge slots (two for western/japanese NES/Famicom cartridges, and one for
SNES cartridges).

NES or SNES mode can be selected in BIOS boot menu. SNES mode does simply
disable the BIOS and jump to game entrypoint at [FFFCh]. NES mode executes the
games via a NOAC (NES-on-a-Chip, a black blob, which is also used in various
other NES clones), in this mode, the SNES video signal is disabled, and, aside
from the BIOS passing joypad data to the NES, the SNES does merely serve as
power-supply for the NES.

**Memory and I/O Map**

```
  00E000h-00FFFFh.R - BIOS ROM (8Kbytes)
  00FFF0h.W - NES Joypad 1 (8bit data, transferred MSB first, 1=released)
  00FFF1h.W - NES Joypad 2 (bit4-5: might be NES reset and/or whatever?)
  00FFF2h.W - Enter NES Mode (switch to NES video signal or so)
  00FFF3h.W - Disable BIOS and map SNES cartridge
```

**Joypad I/O**

In NES mode, the BIOS is reading SNES joypads once per frame (via automatic
reading), and forwards the first 8bit of the SNES joypad data to the NES
(accordingly, it will work only with normal joypads, not with special hardware
like multitaps or lightguns). Like on japanese Famicoms, there are no
Start/Select buttons transferred to joypad 2. Instead, FFF1h.Bit5/4 are set to
Bit5=0/Bit4=1 in SNES mode, and to Bit5=1/Bit4=0 in NES mode (purpose is
unknown, one of the bits might control NES reset signal, the other might select
NES/SNES video signal, unless that part is controlled via FFF3h).

**Mode Selection I/O**

When starting a NES/SNES game, ports FFF2h or FFF3h are triggered by writing
twice to them (probably writing any value will work, and possibly writing only
once might work, too).

**BIOS Versions (and chksum, shown when pressing A+X on power-up)**

```
  Tri-Star (C) 1993                        ;ROM CHKSUM: 187C
  Tri-Star Super 8 by Innovation (C) 1995  ;ROM CHKSUM: F61E
```

Both BIOSes are 8Kbytes in size (although ROM-images are often overdumped). The
versions seem to differ only by the changed copyright message. The GUI does
resemble that of the X-Terminator and Super UFO (which were probably made by
the same anonymous company).

A third version would have been the (unreleased) Superdeck (a similar device
that has been announced by Innovation and some other companies).

**Component List (Board: SFFTP_C/SFFTP_S; component/solder side)**

```
  82pin NOAC chip (black blob on 82pin daughterboard) (on PCB bottom side)
  28pin EPROM 27C64 (8Kx8) (socketed)
  16pin SNES-CIC clone (NTSC: ST10198S) (PAL: probably ST10198P) (socketed)
  20pin sanded-chip (probably 8bit latch for joypad 1)
  20pin sanded-chip (probably 8bit latch for joypad 2)
  16pin sanded-chip (probably 8bit parallel-in shift-register for joypad 1)
  16pin sanded-chip (probably 8bit parallel-in shift-register for joypad 2)
  16pin sanded-chip (probably analog switch for SNES/NES audio or video)
  20pin sanded-chip (probably PAL for address decoding or so) (socketed)
  2pin  oscillator (? MHz) (for NES cpu-clock and/or NES color-clock or so)
  62pin cartridge edge (SNES) (on PCB bottom side)
  12pin cartridge edge (A/V MultiOut) (to TV set) (on PCB rear side)
  62pin cartridge slot (SNES)
  60pin cartridge slot (Famicom) (japanense NES)
  72pin cartridge slot (NES) (non-japanense NES)
  6pin  socket for three shielded wires (Composite & Stereo Audio in from SNES)
  TV Modulator (not installed on all boards)
  four transistors, plus some resistors & capacitors
```

### SNES Cart Pirate X-in-1 Multicarts (1)

There are several X-in-1 Multicarts, all containing the same type of text based
GUI, and thus probably all made by the same company.

**Cartridge Header**

The first 4 bytes of the title string at FFC0h do usually (or always) contain
values 5C,xx,xx,80 (a "JMP FAR 80xxxxh" opcode, which jumps to the GAME
entrypoint). The next 4 title bytes are sometimes containing another JMP FAR
opcode, the rest of the header is unmodified header of the first game; except
that [FFFCh] contains the MENU entrypoint.

**ROM-images**

ROM-images found in the internet are usually incomplete dumps, containing only
the first 4MBytes (clipped to the maximum size for normal unmapped LoROM
games), or only the first game (clipped to the ROM size entry of the 1st game
header). Whilst, the actual multicarts are usually 8MBytes in size (there's one
4Mbyte cartridge, which is actually fully dumped).

**ROM Size**

Most cartridges seem to contain 8Mbyte ROMs. There is one 4MByte cartridge. And
there's one cartridge that contains an 8Kbyte EPROM (plus unknown amount of
ROM).

**LoROM/HiROM**

Most games seem to be LoROM. Eventually "Donkey Kong Land 3" is HiROM? Unknown
if HiROM banks can be also accessed (dumped) in LoROM mode, and if so, unknown
how they are ordered; with/without 32Kbyte interleave...?

**SRAM**

According to photos, most or all X-in-1 carts do not contain any SRAM. Though
some might do so?

**DSPn**

According to photos, most or all X-in-1 carts do not contain any DSP chips.
Though the "Super 11 in 1" cartridge with "Top Gear 3000" seems to require a
DSP4 clone?

**Port FFFFxxh**

```
  A0-A3 Bank Number bit0-3 (base offset in 256Kbyte units)
  A4    Bank Number bit4 (or always one in "1997 New 7 in 1")
  A5    Always 0         (or Bank bit4 in "1997 New 7 in 1")
  A6    Varies (always 0, or always 1, or HiROM-flag in "Super 7 in 1")
  A7    Always 1 (maybe locks further access to the I/O port)
```

The bank number is somehow merged with the SNES address. As for somehow: This
may be ORed, XORed, or even ADDed - in most cases OR/XOR/ADD should give the
same result; in case of "1997 New 7 in 1" it looks as if it's XORed(?)

The special meaning of A4-A5 can be detected by sensing MOV [FFFFnn],A opcodes
(rather than normal MOV [FFFF00+x],A opcodes).

The special meaning of A6 can be detected by checking if the selected bank
contains a HiROM-header.

**Port 6FFFxxh**

Unknown. Some games write to both FFFFxxh and 6FFFxxh (using same data &
address LSBs for both ports). Maybe the ROM bank address changed to 6FFFxxh on
newer boards, and FFFFxxh was kept in there for backwards compatibility. Or
maybe 6FFFxxh controls SRAM mapping instead ROM mapping?

**X-in-1 Cartridges**

```
  Title               FFFFxx      6FFFxx    Size/Notes
  8 in 1 and 10 in 1  C0-DF       N/A       8MB (8 big games + 10 mini games?)
  1997 New 7 in 1     D0-DF,F0-FF N/A       ? MB
  Super 5 in 1        80-9F       80-9F     8MB
  Super 6 in 1        80-8F       N/A       4MB
  Super 7 in 1        80-8F,D0    80-8F,D0  8MB? (mario all stars + 3 games)
  Super 11 in 1       80-9F       N/A       8MB+DSP4 ?
```

**Chipset 7-in-1 (Board: SSF-07, REV.1)**

```
  U1 16pin CIVIC CT6911 (CIC clone)
  U2 16pin 74LS13x or so (not legible on photo)
  U3 16pin whatever      (not legible on photo)
  U4 14pin 74LS02 or so  (not legible on photo)
  U5 black blob
  U6 black blob
```

**Chipset 8-in-1 (Board: MM32-2)**

```
  U  20pin iCT PEEL18CV8P-25
  U  16pin 93C26 A60841.1 9312 (CIC clone)
  U  42pin 56C001 12533A-A 89315
  U  42pin 56C005-4X 12534A-A 89317
```

**Chipset 8-in-1 (Board: NES40M, 20045)**

```
  U  16pin CIVIC 74LS13 (CIC clone)
  U  16pin not installed
  U  28pin 27C64Q EPROM (8Kx8)
  U  20pin iCT PEEL18CV8P-25
  U  42pin JM62301
  U  42pin JM62305
```

### SNES Cart Pirate X-in-1 Multicarts (2)

There's at least one korean multicart (called "C20H" or "super20hab" or so),
with 20 small games stored on a relative small 1Mbyte ROM. The games are NES
games ported to work on SNES, some with typical pirate mods (like removing
copyright strings, or renaming the game to bizarre names).

**I/O Ports**

```
  20xxh NES PPU left-overs (written to, but ignored by the SNES)
  40xxh NES APU left-overs (written to, but ignored by the SNES)
  8000h ROM Bank Size/Base
```

Port 8000h works around as so:

```
  0-4  ROM Base Offset (in 32Kbyte units)
  5    Unknown/unused (always zero)
  6-7  ROM Bank Size (0=Used/unknown, 1=Unused/Unknown, 2=1x32K, 3=2x32K)
```

The ROM is mapped in LoROM fashion (with 1 or 2 banks of 32Kbyte).

SRAM might also exist (the photo shows some unidentified 24pin chip).

**Component List**

```
  PCB Name: Unknown (it has one, but isn't legible on lousy photo)
  32pin C20H (1Mbyte ROM)
  24pin Unknown (maybe SRAM) (there is no battery visible on PCB front side)
  20pin Unknown (looks like a sanded chip; presumably memory mapper)
  16pin CIVIC CTxxxx? (CIC clone)
  46pin Cartridge Edge Connector
```

### SNES Cart Copiers

**Copiers**

**Misc**

**Floppy Disc Controllers**

**BIOSes**

**See also**

### SNES Cart Copiers - Front Fareast (Super Magicom & Super Wild Card)

**Front/CCL/Clones**

The Front Fareast I/O addresses are used by Front's own models, by early CCL
models, and by some third-party clones:

```
  Super Magicom (Front/CCL)
  Super Wild Card (Front)
  Supercom Pro (CCL) (later CCL models use other I/O ports)
  Super Drive Pro-3 UFO (noname) (later UFO models use other I/O ports)
```

**I/O Ports (in banks 00h..7Dh and 80h..FFh)**

```
  C000.R    FDC Flags (Bit7: MCS3201 IRQ Signal, Bit6: Drive 'Index' Signal)
              Note: Index signal is (mis-)used for Disk Insert Check
  C002.W    FDC MCS3201 Drive Control Register (motor on, etc.)
  C004.R    FDC MCS3201 Main Status Register
  C005.RW   FDC MCS3201 Command/Data Register
  C007.R    FDC MCS3201 Diagnostics Register (bit7=disk change; MCS-chip only)
  C007.W    FDC MCS3201 Density Select Register (bit0-1=Transfer rate)
  C008.R    Parallel Data Input (Reading this register reverses busy flag)
  C008.W    Parallel Data Output (bit0-3) and DRAM/SRAM mapping (bit0-1)
              Bit 0: 0=LoROM/Mode 20, 1=HiROM/Mode 21 (DRAM Mapping)
              Bit 1: 0=LoROM/Mode  1, 1=HiROM/Mode  2 (SRAM Mapping)
  C009.R    Parallel Port Busy Flag, Bit 7 (older EP1810 Version) (Altera chip)
  C000.R    Parallel Port Busy Flag, Bit 5 (newer FC9203 Version) (FRONT chip)
  C00A-C00F Unused (mirrors of C008h-C009h)
  C010-DFFF Unused (mirrors of C000h-C00Fh)
```

Below E000h-E00Dh are triggered by writing any value

```
  E000.W    Memory Page 0  ;\Select an 8Kbyte page, CART/DRAM/SRAM address is:
  E001.W    Memory Page 1  ;    SNES address AND 1FFFh      ;lower bits
  E002.W    Memory Page 2  ;    +Selected Page * 2000h      ;upper bits
  E003.W    Memory Page 3  ;/   +SNES address AND FF0000h   ;bank number
  E004.W    Set System Mode 0 (BIOS Mode)             (with all I/O enabled)
  E005.W    Set System Mode 1 (Play Cartridge)        (with all I/O disabled)
  E006.W    Set System Mode 2 (Cartridge Emulation 1) (with E004-E007 kept on)
  E007.W    Set System Mode 3 (Cartridge Emulation 2) (with all I/O disabled)
  E008.W    Select 44256 DRAM Type  (for 2,4,6,8 Mega DRAM Card)
  E009.W    Select 441000 DRAM Type (for 8,16,24,32 Mega DRAM Card)
  E00C.W    BIOS Mode:CART at A000-BFFF, DRAM Mode:DRAM in bank 20-5F/A0-DF
  E00D.W    BIOS Mode:SRAM at A000-BFFF, DRAM Mode:CART in bank 20-5F/A0-DF
```

Later Wild Card DX models have various extra ports, eg. E0FDh, F083h, C108h.

Ports C00xh seem to be used by models up to DX and DX96.

In DX2, Ports C00xh seem to be moved to CF8xh/DF8xh.

**System Mode 0 (BIOS Mode) (selected via E004h)**

```
  bb2000-bb3FFF RW: SRAM or CART (E00C/E00D)  bb-40-7D,C0-FF ;\8K page via
  bb8000-bb9FFF RW: DRAM                      bb-00-7D,80-FF ; E000-E003
  bbA000-bbBFFF RW: SRAM or CART (E00C/E00D)  bb=00-7D,80-FF ;/
  bbC000-bbC00x RW: I/O Ports                 bb=00-7D,80-FF
  bbE000-bbE00x W : I/O Ports                 bb=00-7D,80-FF
  bbE000-bbFFFF R : BIOS ROM (8/16/256Kbytes) bb=00-1F
```

**System Mode 1 (CART Mode) (selected via E005h)**

```
  bb0000-bbFFFF RW: CART
```

**System Mode 2/3 (DRAM Modes) (selected via E006h/E007h)**

```
  bb0000-bb7FFF R : DRAM Mapping, bb=40-6F, C0-DF. (HiROM/Mode 21)
  bb8000-bbFFFF R : DRAM Mapping, bb=00-6F, 80-DF. (AnyROM/Mode 20,21)
  708000-70FFFF RW: SRAM Mode 1 Mapping.         ;<-- typically for LoROM
  306000-307FFF RW: SRAM Mode 2 Mapping, Page 0. ;<-- typically for HiROM
  316000-317FFF RW: SRAM Mode 2 Mapping, Page 1. ;\extra banks for HiROM
  326000-327FFF RW: SRAM Mode 2 Mapping, Page 2. ; (do any 'real' cartridges
  336000-337FFF RW: SRAM Mode 2 Mapping, Page 3. ;/do actually have that?)
```

DRAM mapping (LoROM/HiROM), and corresponding SRAM mapping are selected via
(sharing) Bit0-1 of the Parallel Data Output (Port C008h.W)

HiROM/Mode 21:

```
  Even DRAM Bank is mapped to bb0000-bb7FFF.
  Odd DRAM Bank is mapped to  bb8000-bbFFFF.
```

Optionally, banks 20-5F and A0-DF can be mapped to CART instead of DRAM (via
E00Dh), probably intended to allow ROM-images in DRAM to access DSP chips in
CART.

**BIOS Notes**

Observe that the BIOS is divided into 8Kbyte banks (so, the exception vectors
are at offset 1Fxxh in the ROM-image) (however, there are some overdumped
ROM-images that contain 24K padding prior to each 8K ROM-bank, ie. with LoROM
mapping style exception vectors at offset 7Fxxh). Aside from the exception
vectors, there isn't any title, nor other valid cartridge header entries.

Note: Unlike most SNES programs, Magicom & Wild Card (until v1.8) BIOSes
are running in 6502 emulation mode (with E=1), rather than 65C816 mode (with
E=0).

**Parallel Port Protocol (on SNES side)**

Data is received via 8bit data register, and sent via 4bit "status" register
(which is seen as status on PC side). Strobe/busy aren't clearly documented in
official Front specs; probably, Busy gets set automatically when sensing Strobe
(from PC side), and gets cleared automatically when reading Data from Port
C008h (on SNES side).

**Parallel Port Protocol (on PC side)**

Byte Output Procedure:

```
  Wait Busy Bit = 1           ;Status  PC Port 379h/279h/3BDh.Bit7
  Write One Byte              ;Data    PC Port 378h/278h/3BCh.Bit0-7
  Reverse Strobe Bit          ;Control PC Port 37Ah/27Ah/3BEh.Bit0
```

Byte Input Procedure:

```
  Wait Busy Bit = 0           ;Status  PC Port 379h/279h/3BDh.Bit7
  Read Low 4 Bits of Byte     ;Status  PC Port 379h/279h/3BDh.Bit3-6
  Reverse Strobe Bit          ;Control PC Port 37Ah/27Ah/3BEh.Bit0
  Wait Busy Bit = 0           ;Status  PC Port 379h/279h/3BDh.Bit7
  Read High 4 Bits of Byte    ;Status  PC Port 379h/279h/3BDh.Bit3-6
  Reverse Strobe Bit          ;Control PC Port 37Ah/27Ah/3BEh.Bit0
```

Receiving 4bit units via status line is done for compatibility with old
one-directional PC parallel (printer) ports.

Unknown if "Wait Busy Bit = X" means to wait while-or-until Bit=X?

**Parallel Port Command Format**

Commands are 9-bytes in length, sent from PC side.

```
  00h 3   ID (D5h,AAh,96h)
  03h 1   Command Code (00h-01h, or 04h-06h)
  04h 2   Address (LSB,MSB)
  06h 2   Length (LSB,MSB)
  08h 1   Checksum (81h XORed by Bytes 03h..07h)
  Followed by <Length> bytes of data (upload/download commands only)
```

Commands can be:

```
  Command 00h : Download Data (using page:address,length)   ;to-or-from PC?
  Command 01h : Upload Data (using page:address,length)     ;from-or-to PC?
  Command 04h : Force SFC Program to JMP (to address... plus page/bank?)
  Command 05h : Select 8Kbyte Memory Page Number (using address)
  Command 06h : Sub Function (address: 0=InitialDevice: 1=ExecDRAM, 2=ExecCART)
```

InitialDevice does probably reset the BIOS? ExecDRAM allows to run the uploaded
ROM-image, but unknow how to select LoROM/HiROM mode, or is it automatically
done by examining the uploaded cartridge header. The usage of the 16bit address
isn't quite clear: The lower 13bit are somehow combined with the 8Kbyte page
number, the upper 3bit might be used to select DRAM/SRAM?

**Super Magicom V3H - BIOS upgrade**

This ROM-image is a Magicom BIOS upgrade, and it's a pain in the ass:

The upgrade works ONLY as ROM-image (ie. must be loaded to DRAM), and does NOT
work as real ROM (ie. cannot be burned to EPROM), the reason is that it doesn't
include a character set (and uses that from the original Magicom BIOS at
E000h).

The upgrade isn't compatible with the parallel port (the original BIOS
relocates parallel port code from ROM to WRAM, and the upgrade relocates itself
from DRAM to WRAM - but still expects the parallel port code in the same WRAM
location and crashes when Busy-bit gets set).

The upgrade exists as 32Kbyte or 32.5Kbyte ROM-image (an 8K upgrade, 24K
garbage with entrypoint at end of garbage, plus 0.5K extra garbage), emulators
and other tools will be typically interprete the 32.5K file as 32Kbyte ROM with
512-byte header (which is NOT correct in that special case).

The upgrade exists in two variants: One using the standard Front Fareast I/O
addresses, one using the I/O addresses at C000h-C00Fh re-ordered as so:

```
  8000h-FFFFh RW DRAM-mode: DRAM (containing the Magicom V3H upgrade)
  E000h-FFFFh R  BIOS-mode: BIOS (containing the Character set)
  C000h       W  DRAM bank mapped to 8000h? (set to 00,20,40 upon DRAM detect)
  C001h       W  Memory Control?
  C002h       -  Unused
  C003h       R  Parallel Port Busy (bit7) (when set: crashes the V3H upgrade)
  C004h-C008h -  Unused
  C009h       R  Status (bit7=ready?,bit5=busy/timeout?)
  C00Ah       -  FDC Unused
  C00Bh       W  FDC Motor Control (set to 00h,29h,2Dh)
  C00Ch       RW FDC Command/Data
  C00Dh       R  FDC Main Status
  C00Eh       W  FDC Transfer Rate? (set to 00h,01h,02h or so)
  C00Fh       -  FDC Unused
  E004h       W  Map BIOS ROM (instead V3H upgrade) ;\
  E006h-E007h W  Something on/off                   ; seems to be same/similar
  E008h-E009h W  Something on/off                   ; as Front-like I/O ports
  E00Ch-E00Dh W  Something on/off                   ;/
```

Unknown which hardware uses that re-ordered addresses. Note: The V3H version
with re-ordered I/O addresses does also contain different 24K garbage (sorts of
as if it were created from original source code, rather than just patched?).

**Component List - Super Magicom Plus**

```
  U1  24pin  DRAM (onboard)
  U2  20pin  SN74LS245N (8-bit 3-state transceiver)
  U3  24pin  DRAM (onboard)
  U4  24pin  DRAM (onboard)
  U5  24pin  DRAM (onboard)
  U6  28pin  27C128-25 EPROM (16Kx8)
  U7  68pin  MCCS3201FN (=MCS3201FN without double-C) (disc controller)
  U8  100pin ?
  U9  28pin  HM62256 (SRAM 32Kx8)
  U10 20pin  ?
  U11 14pin  ?  (does not exist in later versions?)
  U12 20pin  AMI 16CVB8PC-25
  U13 20pin  AMI 16CVB8PC-25
  U14 16pin  ST10198S (newer version only) (mounted on top of U10 in old ver)
  BT1 2pin   ?
  J1  25pin  DB-25 parallel port
  J2? 25pin  DB-25 external floppy (not installed)
  J3  40pin  DRAM expansion board
  J4  34pin  internal floppy (flat cable)
  J5  26pin  (not installed)
  J6  4pin   floppy power supply
  J7  62pin  cartridge edge
  J8  62pin  cartridge slot
  J9  12pin  jumpers (I:I:I: or :I:I:I) (enable internal or external CIC)
  Y1  2pin   24.000 MHz
```

**Component List - Super Wild Card DX (AH-558001-02 Made in Japan 94.8.23)**

```
  U1  100pin CPU      FRONT FC9203 HG62E22926F9 (or so) (SMD)
  U2   28pin S-RAM    NEC D43256AC-10L (uPD43256AC) (SRAM, 32Kx8)
  U3   20pin          SN74LS245 (to parallel port) (8-bit 3-state transceiver)
  U4    ?pin PAL-2    L GAL20V84 25LP
  U5   20pin          SN74LS...? (or so)
  U6   32pin BIOS-ROM BIOS
  U7   16pin U7       SN74LS139AN (decoder/demultiplexer)
  U8   14pin U8       SN74LS125AN (quad 3-state buffer)
  U9   44pin          GoldStar GM82C765B (SMD) (floppy disc controller)
  U10  16pin DECODER  SNC4011 (or so)
  U11  20pin PAL-3    iCT PEEL17CV8P CTN24053
  U12?  3pin 7805H    voltage regulator
  U13  20pin PAL-1    AMI
  X1    2pin 16MHZ    16.000 MHz
  CN?   2pin AC/DC-IN power supply input
  CN?   4pin ..POW    power supply to internal disc drive (only 2pin connected)
  CN2  62pin          female cartridge slot
  CN3? 46pin RAM-SLOT to DRAM daughterboard (only 40pin used on remote side)
  CN5  25pin PC-I/F   DB-25 parallel port
  CN6  34pin FDD-I/F  cable to internal disc drive
  CN01 34pin          goes to one 1st of male 62pin cartridge edge
  CN02 34pin          goes to one 2nd of male 62pin cartridge edge
  SW1   3pin RESET-SW reset switch/button or so, for whatever purpose
  DB1   4pin          AC-DC converter
  BT1   2pin          3V battery
  J1   12pin          jumpers (near cartridge slot)
  J2   20pin          jumpers (near cartridge slot)
  J3    2pin          jumper (near power-input)
  J4    2pin          jumper (near power-input)
  J5    2pin          jumper (near power-input)
  DRAM Daughterboard:
  U1,U2,U7,U8   16pin  ST T74LS139B (decoder/demultiplexer) (four pieces)
  U3-U6,U9-U10  28pin  NEC D424900G5 (or so) (six pieces) (SMD)
  U11-U12       28pin  M5M44800ATP           (two pieces) (SMD)
```

**Component List - Supercom Pro (SP3200) (dated around 1992)**

(probably uses Front-like I/O)

```
  U1  20pin  SN74LS245N (to parallel port) (8-bit 3-state transceiver)
  U2  16pin  HD74LS174 (to parallel port?)
  U4  68pin  MCCS3201FN (=MCS3201FN without double-C) (floppy disc controller)
  U?  68pin  Altera EP1810LC-45 D9219
  U?  28pin  EPROM
  U?  28pin  SRAM Winbond W24256-10L 9149
  U7  20pin  SN74LS245N (8-bit 3-state transceiver)
  U8  16pin  not installed
  U?  20pin  modded (?) chip (soldered on cart-edge connector at bottom side)
  J1  25pin  DB-25 parallel port
  J2  25pin  DB-25 external floppy disc connector
  J3  40pin  to DRAM daughterboard
  J4  34pin  not installed (internal floppy disc connector)
  J5  62pin  cartridge edge
  J6  62pin  cartridge slot
  Y1  2pin   24.000 MHz
  BT1 2pin   VARTA Ni/Cd, 3.6V 60mA, 14h 6mA (recharge-able & acid-leaking)
```

**Component List - SMD800 Super Magic Drive (requires SNES-to-Genesis adaptor)**

```
  U1  20pin  SN74LS245N (to parallel port?) (8-bit 3-state transceiver)
  U2  16pin  SN74LS174 (to parallel port?)
  U3  20pin  SN74HC245P (8-bit 3-state transceiver)
  U4  20pin  SN74HC245P (8-bit 3-state transceiver)
  U5? 68pin  MCS3201FN (floppy disk controller)
  U6  20pin  SN74HC245P (8-bit 3-state transceiver)
  U   28pin  27C64A-15 (EPROM, 8Kx8) (with Genesis Z80 code, non-SNES code)
  U   28pin  HY62256ALP-10 (SRAM, 32Kx8)
  U     pin  Altera EP1810LC-45
  U10 16pin  MC74HC157 (decoder/demultiplexer)
  U11 16pin  MC74HC157 (decoder/demultiplexer)
  U12 14pin  xxxx
  J   25pin  DB-25 parallel port
  J2  25pin  DB-25 external floppy
  J3  40pin  internal floppy (not installed) (likely only 34pins of 40pin used)
  J   64pin  cartridge edge (genesis)
  J   64pin  cartridge slot (genesis)
  J   40pin  to DRAM daughterboard
  Y   2pin   oscillator
  BT  2pin   VARTA Ni/Cd, 3.6V 60mA, 14h 6mA (recharge-able & acid-leaking)
 DRAM Daughterboard:
  U1  20pin  HY514400J-70 (DRAM)
  U2  20pin  HY514400J-70 (DRAM)
  U3  20pin  HY514400J-70 (DRAM)
  U4  20pin  HY514400J-70 (DRAM)
  U5  14pin  74LS08 (quad 2-input AND gates)
  U6  16pin  HD74LS157P (decoder/demultiplexer)
  U7  14pin  74LS08 (quad 2-input AND gates)
  CN1 40pin  connector to mainboard
 Super Magicom-Drive (SNES-to-Genesis adaptor for above):
  xxx        components unknown
```

### SNES Cart Copiers - CCL (Supercom & Pro Fighter)

Below is for Supercom Partner & Pro Fighter models from CCL (China Coach
Limited). See the Front Fareast chapter for their earlier Super Magicom models
(which were produced by Front & CCL), and also for Supercom Pro 2 (which
was made by CCL alone, but still used the Front-like I/O ports).

**Pro Fighter 1993 by H.K. / Supercom Partner A**

Ports are somewhat based on the Front design (BIOS is expanded from 8K at
E000h-FFFFh to 16K at C000h-FFFFh, accordingly FDC Ports C000h-C007h are moved
to 2800h-2807h, and Parallel Port Ports C008h-C009h are simply removed. Ports
at E00xh are somehow changed, but might be still similar to the Front design
(?)

```
  2800.R   FDC MCS General Purpose Input (bit7,bit6 used)
  2802.W   FDC MCS Motor Control (set to 00h,29h,2Dh)
  2804.R   FDC MCS Main Status
  2805.RW  FDC MCS Command/Data Status
  2807.W   FDC MCS Transfer Rate/Density (set to 0..3)
  Below 2808-2810 only in newer "Pro Fighter Q"
   2808.R   Parallel Port Data (bit0-7)
   2809.W   Parallel Port Data (4bit or 8bit?)
   2810.R   Parallel Port Busy (bit5)
    (there seem to be 4bit & 8bit parallel port modes supported, one of them
    also WRITING to 2808h, and in some cases reading "FDC" register 2800 looks
    also parallel port DATA and/or BUSY related)
  Again changed for Double Pro Fighter
   2803.R   Parallel Port Busy (bit7)
   2808.R   Parallel Port Data (bit0-7)
   2809.W   Parallel Port Data (4bit or 8bit?)
   2804 =FDC DATA   ;\swapped ! (unlike older "non-double" models)
   2805 =FDC STAT   ;/
   280x =other ports in this region may be changed, too ?
   004800    ROM (from offset 8800-9FFF) (contains program code)
   014800    ROM (from offset A800-BFFF) (contains character set)
   E00x
   E800+x
   Note: Having BIOS portions mapped to the fast 3.58MHz region at 4800h-5FFFh
         was probably done unintentionally; this would require 120ns EPROMs,
         whilst some Double Pro Fighter boards are fitted with 200ns EPROMs
         (which are stable at 2.68MHz only, and may cause crashes, or charset
         glitches in this case)
   Double Pro Fighter BIOS is 64Kbytes:
     0000-3FFF  Genesis/Z80 BIOS
     4000-7FFF  Same content as 0000-3FFF
     8000-87FF  Unused (zerofilled)
     8800-9FFF  SNES BIOS (6K mapped to 004800-005FFF)
     A000-A7FF  Unused (zerofilled)
     A800-BFFF  SNES BIOS (6K mapped to 014800-015FFF)
     C000-FFFF  SNES BIOS (16K mapped to 00C000-00FFFF)
  7000.R
  A000.RW               ;7000-related
  C000-FFFF.R BIOS ROM (16Kbytes)
  E002.W   set to 00h   ;7000-related
  E003.W   set to BFh ;then compares BFFD with BFFC,BFFA,BFFB,BFEA,BFEB
  E00C.W   set to 00h   ;7000-related
  E00E.W   set to E0h
  008000.RW   DRAM detection?
  208000.RW   DRAM detection?
  408000.RW   DRAM detection?
  608000.RW   DRAM detection?
```

### SNES Cart Copiers - Bung (Game Doctor)

Game Doctor SF7

**Memory Map (in BIOS mode)**

```
  00:8000-807F     I/O Ports
  00:8080-FFFF     BIOS ROM (1st 32kBytes)
  01:8000-FFFF     BIOS ROM (2nd 32kBytes) (if any)
  02:8000-FFFF     unused
  03:8000-FFFF     unused
  04:8000-FFFF     SRAM for game positions (32Kbyte)
  05:8000-FFFF     SRAM for real time save data (4kByte)
  06:8000-FFFF     SRAM for copier settings (4kByte)
  07:8000-FFFF     DRAM for ROM-image (32Kbyte page, selected via Port 8030h)
  08-7D:8000-FFFF  Mirror of above banks 00-07
  80-FF:8000-FFFF  Mirror of above banks 00-07 or Cartridge banks 00-7F/80-FF
```

```
  FFBFh compared to FFh ?
```

**I/O Ports (in BIOS mode) in bank 00h**

```
  8000h-800Fh RW 512Kbyte DRAM chunk, mapped to upper 32Kbyte of Bank 0xh-Fxh
  8010h-8013h RW 512Kbyte DRAM chunk, mapped to lower 32Kbyte of Bank 4xh-7xh
  8014h-8017h RW 512Kbyte DRAM chunk, mapped to lower 32Kbyte of Bank Cxh-Fxh
  8018h-8019h W  SRAM Flags (bit0-15=Enable SRAM at 6000-7000 in banks 0xh-Fxh)
  8018h       R  bit1 = realtime.$4016.bit0, read bit7 = ? , bit = ?
  8018h.R  Flags (bit7/6 FDC IRQ?, and more)
  8019h       R  bit1 = ?
  801Ah       R  realtime.word, latch settings for double write word registers
  801Ah       W  write ?
  801Bh       W  write ?
  801Dh       W  BIOS mode mapping: changes what is mapped into banks $80-$FF
                   only bit0-bit1 seem to matter
                   0 = use cartridge banks $00-$7F
                   1 = use cartridge banks $80-$FF
                   2 = mirror banks $00-$7F (BIOS regs and all?)
                   3 = mirror banks $00-$7F (BIOS regs and all?)
  801Eh write ?
  __Floppy Disc__
  8020h       R  FDC Main Status
  8021h       RW FDC Command/Data
  8022h       W  FDC Transfer Rate/Density (?) (set to 00h,01h)
  8023h       -  FDC Unused
  8024h       W  FDC Motor Control (set to 00h,08h,0Ch,1Ch,2Dh)
  8025h-8027h -  FDC Unused
  8028h       W  set to same value (ANY VALUE?) as 8022/8029)
  8029h       W  set to same value (ANY VALUE?) as 8029)
  802Ah       W  set to 01-then-00 (once) (thereafter do sth to 8022)
  802Bh       W  set to 01h during FDC COMMAND-BYTEs (else to 00h) (maybe LED?)
  __Parallel Port__
  802Ch       RW Parallel Port Data Lines
  802Dh       RW Parallel Port Status Lines
  802Eh       RW Parallel Port Control Lines
  802Fh       W  Parallel Port? Unknown (set to 00h,01h) (data direction?)
  802Fh       R  Parallel Port? Unused (reads same as $00802D)
  __Memory__
  8030h-8031h W  Select 32Kbyte-DRAM-Page (0000h..01FFh) mapped to 078000h
  8030h-803Dh R  this is a 7 word table?? (gotten from code at 80/AE80)
  8040h-805Fh R  read same as 802Dh (uh, but, some are used for sth else?)
  8040h       R  used, parallel port related (or other mainboard version?)
  8043h       W  used, parallel port related (or other mainboard version?)
  8060h-807Fh R  read = FFh
  80xFh          any access to 0080xFh (x=8..F) switches to cartridge mode
```

**802Dh - Parallel Port status (not direct pin reading?)**

```
 read
  bit0 = /C1 (direct pin14, /AutoLF) (/Ctrl.Bit1 on PC side)
  bit1 = C2  (direct pin16, /INIT)   (Ctrl.Bit2 on PC side)
  bit2 = /C3 (direct pin17, /Select) (/Ctrl.Bit3 on PC side)
  bit3 = "write bit3"
  bit4 = "write bit4"
  bit5 = "write bit4" (uh, not bit5 here?)
  bit6 = "write bit4" (uh, not bit6 here?)
  bit7 = /S7 (direct pin11) = "write bit7" AND not "write bit0"
 write
  bit0 Enable/Disable Busy bit (0=Enable, 1=Disable) (?)
  bit3 => S3 (direct pin15, /ERR) (Stat.bit3 on PC side)
  bit4 => S4 (direct pin13, SLCT) (Stat.bit4 on PC side)
  bit5 => S5 (direct pin12, PE)   (Stat.bit5 on PC side)
  bit6 => S6 (direct pin10, /ACK) (Stat.bit6 on PC side)
  bit7 ...   (direct pin11, BUSY  (/Stat.bit7 on PC side) (ANDed with /bit0?)
```

**802Eh - Parallel Port control (not direct control reg values)**

```
 often write 12h-then-10h
 read/write?
  bit0 = /C1 (direct pin14, /AutoLF) (/Ctrl.Bit1 on PC side)   W
  bit1 = C2  (direct pin16, /INIT)   (Ctrl.Bit2 on PC side)    W
  bit2 = /C3 (direct pin17, /Select) (/Ctrl.Bit3 on PC side)   W
  bit3-bit6, read = bit3-bit6 of $00802D                       W
  bit7 = /C0 (direct pin1,  /STB) (/Ctrl.bit0 on PC side)      R
```

**Component List - Bung Game Doctor SF6 (Board CT401)**

```
  U    3pin 7805 or so
  U   40pin GoldStar xxx (=probably GM82C765B) (floppy disc controller)
  U  ???pin huge chip (200 pins or so)
  U   18pin 265111
  U   20pin 74LS744 or so (not installed)
  U   28pin SRAM or so
  U   28pin SRAM or so
  U   28pin EPROM (GDSF_6.0)
  U   14pin whatever/modded chip (wired top-down near EPROM)
  P   40pin to DRAM daughterboard 1 (2x10 male pins, 2x10 female pins)
  P   40pin to DRAM daughterboard 2 (2x10 male pins, 2x10 female pins)
  P   62pin cartridge port
  P   62pin cartridge port (on PCB back side)
  P   25pin DB-25 parallel port (on PCB back side)
  P    2pin power supply (on PCB back side)
  P    2pin floppy supply (on PCB back side)
  P   34pin floppy data
  X    2pin oscillator
```

### SNES Cart Copiers - Super UFO

UFO Super Drive Pro / Super UFO

**UFO3**

The UFO-3 is a Front Fareast clone. BIOS is 8Kbytes mapped to E000h-FFFFh, FDC
Registers are at C000h-C007h, Parallel Port at C008h-C009h, Memory Control at
E000h-E00Dh. For details see Front Fareast chapter.

**UFO6**

I/O ports for this version are unknown.

**UFO7/UFO8**

```
  2184.W   ... set to 00h/0Ch/0Fh
  2185.W   ... set to 00h/0Fh
  2186.W   ... set to 00h/0Fh
  2187.W   ... set to 08h/00h/0Bh
  2188.W   ... set to 00h..0Fh or so
  2189.W   ... set to 0Fh/0Eh
  218A.W   ... set to 00h
  218B.W   ... set to 0Ah/0Fh
  218C.R   FDC Main Status Register
  218D.RW  FDC Command/Data Register (emit 03h,DFh,03h = spd/dma)(then 07h,01h)
  218E.W   FDC Motor Control (set to 00h, 29h-then-2Dh on disc access)
  218F.W   FDC Transfer Rate
  218F.R   FDC Flags (bit7=irq?,bit6=index?) (UFO8: bit5=?)
  003F68.R         warmboot flag? if A581 --> JMP 3D00
  003FD0..3FFF   cartridge header? (or copy of it?)
  003C00..003FFF SRAM 1Kbyte (BIOS settings, I/O logging?, last 32-byte OAM)
  013C00..013FFF SRAM 1Kbyte (512-byte Palette and 1st 512-byte OAM)
  008000h and up BIOS 64Kbytes (UFO7) or more 128K..256K (UFO8)
  708000h and up SRAM 32Kbytes (for game positions)
  808000h and up DRAM (variable size detected) (via calls to 9025)
```

ufo7 rom chksum calculated at 9505 (32K ROM at 8000-FFFF must sum up to 00h)

ufo8 (and maybe ufo7 too) should have 8K SRAM (ie. MORE than above 2x1K...?)

**UFO6 Component List - UFO Super Drive Pro (with Pro-6 BIOS)**

```
  U    3pin 7805 or so
  U    ?pin xxxx (near 7805)
  U   40pin GoldStar GM82C765B
  U   14pin xxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin AMI xxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin LS245 (not installed, near DB-25) (8-bit 3-state transceiver)
  U   14pin HC74  (not installed, near DB-25) (dual flip-flop)
  U   16pin xxxx  (installed, near DB-25)
  U   28pin EPROM
  U   28pin Winbond W24256-10L (SRAM 32Kx8)
  U   20pin Philips PC74xxxx
  U   16pin 74LS112 (reportedly a cloned/mislabelled CIC chip)
  X    2pin oscillator (near 7805)
  BT   2pin 3V or so
  P   25pin DB-25 parallel port or so
  P    2pin power supply
  P    2pin floppy supply
  P   34pin floppy data
  P   40pin to DRAM daughterboard
  P   46pin cartridge slot (only 46 soldering points)
  P   62pin cartridge edge (has 62 soldering points, but only 46 connected?)
```

**UFO8 Component List Super UFO Super Drive PRO8 (REV 7.8 2)**

```
  1x 84pin Altera EPMxxxxxxx84-15
  1x 40pin GoldStar GM82C765B (DIL) (floppy disc controller)
  1x 32pin BIOS ROM/EPROM (located on PCB solder side)
  1x 28pin UM62256D-70L (SRAM 32Kx8)
  1x 28pin UT6264PC-70LL (SRAM 8Kx8)
  1x 28pin DSP chip (not installed)
  2x 24pin NN5117405BJ-60 (DRAM, two pieces, located on daughterboard)
  1x 16pin D1
  1x 14pin FT4066
  1x 14pin DSP_74HC74 (not installed) (dual flip-flop)
  1x 14pin 74LS00 or so
  1x 14pin whatever (near oscillator)
  1x 14pin 74LSxxx whatever (near PAL/NTSC jumpers)
  2x 16pin SN74HC157N (decoder/demultiplexer)
  1x 3pin  7805 or so
  1x 34pin connector/cable to internal disc drive
  2x 62pin cartridge connectors (one male, one female)
  1x 2pin  wire (supply to internal disc drive)
  1x 2pin  connector (external power supply input)
  no battery, no parallel port
```

### SNES Cart Copiers - Sane Ting (Super Disk Interceptor)

The Super Disk Interceptor is a SNES copier from KL818 B.C./Sane Ting Co. Ltd.,
the company also made a copier for Mega Drive, called Mega Disk Interceptor.

**I/O & Memory**

```
  8000-9FFF memory (SRAM/DRAM or so)
  A000.W  set to 00,03-then-01, or 40,80
  A001.W  set to 00,04 (as MSB of A000) or to 04,24,08
  A001.R  tests bit4,bit5
  A002.W  FDC Transfer Rate/Density (set to ([0B] XOR 1)*2)
  A003.W  FDC Motor Control (set to 08,0C,1C)
  A004    FDC Unused
  A005.RW FDC Command/Data
  A006.R  FDC Main Status
  A007    FDC Unused
  A008.W  set to [1802] (bit3,bit4 used)
  B000-B01F ...    I/O or RAM or RegisterFile workspace?
   B000.W  set to 00
   B000.R  checked if 00h
   B001.W  set to 00
   B002.W  set to xx OR 80h
   B002.R  read and ORed with 06h
   B003.W  set to xx OR 80h OR 03h
   B003.R  bit5 isolated, ORed with 04h, then written to A001h
   B004.W  set to 00 or 00..03h
   B004.R  whatever, if (N+1)=00..03 --> written to B004 and B005
   B005.W  set to 00
   B006.RW set to [4219h] = MSB of joypad1 (?)
   B00F.W  set to [FFDC]=00h or ([FFDC] XOR 1)=01h
   B00F.R  checked if 00h (if nonzero --> WRITE PROTECT)
   B01x    ...
  C000.R  dummy read within waitvblank
  C001.R  dummy read within waitvblank
  E000.W
  E001.W
  E002.W
  E000-FFFF BIOS (32Kbytes, in 8Kbyte units, in banks 00h-03h)
  704000
  708000
```

**Component List - Super Disk Interceptor (version dated around 1992)**

```
  U1  40pin  GoldStar GM82C765B PL (DIP) (floppy disk controller)
  U2  84pin  MD1812 9211 (with socket)
  U3  28pin  2xxx4A-25 (PROM, presumably 8Kx8, non-eraseable)
  U4  20pin  not installed (DIP) (BANK2.3)
  U4A 20pin  not installed (DIP) (BANK2.3)
  U5  20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U5A 20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U6  28pin  Hyundai HY62256ALP-10 (SRAM, 32Kx8)
  U7  20pin  not installed (DIP) (BANK2.3)
  U7A 20pin  not installed (DIP) (BANK2.3)
  U8  20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U8A 20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U9  16pin  74HC157N (decoder/demultiplexer)
  U10 16pin  74HC157N (decoder/demultiplexer)
  U11 20pin  HY-xxxxxx-30
  U12 20pin  HY-xxxxxx-30
  XTAL 2pin  16.000MHz
  J   46pin  Cartridge edge (snes)
  J   46pin  Cartridge slot (snes)
  J   34pin  Floppy data
  J    2pin  Floppy supply
  BT  2pin   3.6V Battery
```

**Component List - Super Disk Interceptor (version dated around 1993)**

```
  U   44pin  GoldStar GM82C765B PL (SMD) (floppy disk controller)
  U   28pin  27C64SDM (PROM, 8Kx8, non-eraseable)
  U   28pin  GoldStar GM76C256ALLFW70 (SRAM, 32Kx8)
  U   20pin  HD74HC373P (8-bit 3-state transparent latch)
  U   14pin  xxxx
  U   80pin  SD1812 349 (SMD, without socket)
  U   14pin  not installed
  U   28pin  KM48C2100J-7 (DRAM, 2Mx8)  ;\
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ; all installed,
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ; together = 4Mx8
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ;
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ;/
  X    2pin  16.000MHz
  X    2pin  16.257MHz
  J   46pin  Cartridge edge (snes)
  J   46pin  Cartridge slot (snes)
  J   34pin  not installed (alternate floppy connector?)
  J   34pin  Floppy data
  J    2pin  Floppy supply
  BT  2pin   3.6V Battery
```

### SNES Cart Copiers - Gamars Copier

Known as:

```
  ALMA Super Disk F-16
  Gamars Super Disk FC-301
  FR-402 Super Disk (bundled with "FR-402 Super 16bit" SNES clone)
```

```
  2K SRAM at 005000 with REQUIRED mirror at 005800
            3F5Fxx.W  set to FFh,FFh,FFh...
            3F5FC0.R  FDC stat  (bit7,bit5)
            3F5FD2.W  FDC motor? (set to 0Ch,1Ch,08h,0Ch)
            3F5FE4.R  FDC Main Status
            3F5FED.RW FDC Command/Data (emit 03,DF,03)
```

**Gamars Puzzle**

Aside from the Gamars BIOSes, there's a mis-named ROM-image in the internet:
"Gamars (Copier BIOS)", this file is made by the same company, but it's a
Puzzle game, not a copier BIOS.

### SNES Cart Copiers - Venus (Multi Game Hunter)

MGH (Multi Game Hunter) from Venus.

The 32Kbyte BIOS contains both SNES/65C816 code (entrypoint at [FFFCh]) and
Genesis/Z80 code (entrypoint at 0000h).

```
  006000..007FFF -- RAM or so
  035800..035807 -- I/O Ports
  ---
  006400.R          id "SFCJ"
  007D00..007EFF.R  checksummed
  035800.W    set to C0h
  035801.W    set to A0h
  035802.W    set to 0000h or 06h
  035803.W    set to 04h
  035804.R    disk status?  (bit7,bit6)
  035805.W    disk command? (set to 0Bh) (not a uPD765 command?)
  035806.W    set to 00h or ([AAh] ROR 1)
  035807.W    set to 00h or [ABh]
```

**FDC Accress via 80C51 CPU**

Like many other copiers, the MGH does use a "normal" MCS3201FN controller, but,
it does indirectly access it through a 80C51 CPU. For example,

```
    05       cmd (write sec)                                 ;\
    [0B6B]   track          ;\less parameters as than        ; write sector
    [0BA4]   head           ; directly accessing a uPD765    ; command
    [0BA2]   sector         ;/                               ; (at PC=CBAFh)
    [[18]+y] data... (200h bytes)                            ;/
```

### SNES Cart Copiers - Others

**Component List - Board "GP-003 REV. B" (used in Special Partner)**

```
  U   14pin  GD74HC04 (hex inverters)
  U   16pin  LR74HC158 (decoder/demultiplexer)
  U   16pin  LR74HC158 (decoder/demultiplexer)
  U   16pin  GD74HC138 (decoder/demultiplexer)
  U   16pin  GD74HC138 (decoder/demultiplexer)
  U   20pin  PALCE16V8H-25
  U   28pin  K-105                                               ;DSP clone?
  U   28pin  EPROM (28pin 27C512 64Kx8 installed, optionally 32pin possible)
  U   28pin  NEC D43256BGU-70L (uPD43256BGU) (SRAM 32Kx8)
  U   28pin  NEC 4364C-20L (SRAM 8Kx8)
  U   44pin  GoldStar GM82C765B PL (SMD)
  U   44pin  Lattice ispLSI 1016-60LJ B501B06 (SMD)
  U    3pin  7805 or so
  J2  62pin  to cartridge edge (snes)
  J   62pin  cartridge slot (snes game cartridge)
  J   62pin  cartridge slot (an expansion slot, not for any game carts)
  J   40pin  to DRAM daughterboard
  J   34pin  to internal floppy drive
  J2   2pin  floppy power supply
  J    2pin  external power supply
  J6   2pin  jumper (near 34pin floppy cable)
  J8   3pin  jumper (near EPROM; maybe ROM size select?)
  X    2pin  oscillator (160)
  X    2pin  oscillator (?)                                      ;DSP clock?
  BT   2pin  NiCd 3.6V
```

The board doesn't contain a CIC-clone (unless it's 'hidden' in one of the
chips).

**Components - Supercom, 24m DSP, CD-ROM, FX-32, High Density, Real Time Save**

```
  U1  40pin GoldStar GM82C765B (floppy disc controller)
  U2  20pin 16V8 (not installed)
  U3? 20pin PALCE16V8H-25PC/4
  U4  24pin PALCE20V8H-25PC/4
  U5  28pin not installed (probably for DSP clone)
  U6? 14pin xxx (below U5)
  U7? 20pin LS245 (not installed) (8-bit 3-state transceiver)
  U8? 24pin PALCE20V8H-25PC/4
  U9  20pin 74HC273 (8bit latch with reset)
  U10 28pin 27C256G-20 (EPROM 32Kx8) (boots as "FX-32 CD-ROM & DSP, 1994 H.K.")
  U11 28pin ST MK4864 (SRAM 8Kx8)
  U12 28pin xxx (SRAM ?Kx8)
  U13 20pin xxx
  U14 20pin xxx
  U15 20pin xxx
  U16 20pin xxx
  U17 20pin xxx
  U18 20pin xxx
  U19  ?pin Toshiba xxx (16pin chip, mounted in a 20pin socket)
  U20 16pin ST101xxx
  Q1   3pin 7805
  Y1   2pin oscillator
  Y2   2pin oscillator (not installed, probably for DSP chip)
  J?  34pin floppy data
  J?   2pin floppy power
  J?   2pin power supply
  J3? 25pin DB-25 (parallel port and/or external CD-ROM drive?)
  J4  62pin cartridge edge
  J5  62pin cartridge slot
  J6  40pin to DRAM daughterboard
```

**Component List - Double Pro Fighter (CCL) (1994)**

```
  U1  28pin  Hyundai HY62256ALP-10 (SRAM 32Kx8)
  U2  28pin  AM27C512-205DC (EPROM 64Kx8)
  U3  N/A    N/A
  U4  N/A    N/A
  U5  20pin  HD74LS245P (8-bit 3-state transceiver)
  U6  20pin  HD74LS245P (8-bit 3-state transceiver)
  U7  20pin  HD74LS245P (8-bit 3-state transceiver)
  U8  24pin  GoldStar GM76C28A-10 (SRAM 2Kx8)
  U9  16pin  noname-chip-without-part-number (or, marked 10198 on other boards)
  U10  3pin  AN7805 (voltage regulator)
  U11 14pin  HD74HC00P (quad 2-input NAND gates)
  U12 40pin  Goldstar GM82C765B (floppy disc controller)
  U13 68pin  Altera EP1810LC-45 D9407
  U14 16pin  74HC139 (decoder/demultiplexer)
  U15 24pin  PALCE20V8H
  U16 20pin  GAL16V8xxx
  U17 20pin  PALCxxx
  U18 16pin  74HC139 (decoder/demultiplexer)
  Y1   2pin  16.00 TDX (16 MHz oscillator)
  J1   2pin  power supply input
  J2   2pin  power supply connector (alternately to J1 or so, not installed)
  P4  50pin  ro dram daughterboard ?
  SL1  64pin connector for remove-able snes-or-sega? cartridge edge
  SL2  64pin connector for remove-able sega-or-snes? cartridge edge
  SL3  62pin cartridge slot (snes)
  SL4  64pin cartridge slot (sega genesis)
  ?     2pin connector for disc drive (supply)
  ?    34pin connector for disc drive (data)
  DRAM Daughterboard
  -    40pin connector (to 40pins of the 50pin socket on Double Pro Fighter)
  -    20pin NEC 424400-80 (EIGHT pieces)
  Optional Parallel Port (plugged into SL3-socket, ie. into SNES slot):
  U1   20pin PALCE16V8H-25PC/4
  U2   20pin HD74HC245P (8-bit 3-state transceiver) (no latch here ???)
  P1   25pin DB-25 parallel port connector
  -    62pin cartridge edge (to be plugged into SL3 of Double Pro Fighter)
```

**Component List - Super Smart Disc (same as Pro Fighter X?)**

```
  U   16pin 10198
  U   28pin GRAPHIC DSP1-1  (or is it "DCP1-1" or so?)
  U   28pin STxxxx (SRAM, ?x8)
  U   28pin xxxxxx (SRAM, ?x8)
  U   28pin EPROM (28pin chip mounted in 32pin socket)
  U   14pin xxxx
  U   40pin ICT PA7140T CTM42027JC
  U   40pin ICT PA7140T CTM42027JC
  U   40pin GoldStar GM82C765B
  U   24pin xxxxx (PAL or so)
  U    3pin 7805 or so
  X    2pin oscillator
  P1  64pin cartridge edge (via remove-able adaptor) (snes)
  P   62pin cartridge slot (snes)
  P   32pin cartridge slot (gameboy)
  P   34pin floppy data
  P    2pin power supply
  P    2pin floppy supply
  P   50pin to DRAM daughterboard
```

### SNES Cart Copiers - Misc

**Parallel Ports (DB-25)**

Parallel Ports are used to upload/download data from PCs. Later copiers seem to
be additionally using the Parallel Port for connecting CD-ROM drives.

Some copiers have fully working parallel ports installed (eg. Front Fareast),
some have them incompletely installed (eg. some Supercom seem to require
additional 74LS245 (8-bit 3-state transceiver), and a specially programmed
PAL16V8 chip?).

Other copiers don't have any provisions for parallel ports onboard - but can be
eventually upgraded externally by plugging a parallel port cartridge into the
SNES cartridge slot: There are at least two such upgrade cartridges (one
contains pure logic, the other one additionally contains a BIOS upgrade).

**DSP Chips**

Some copiers include DSP-clones onboard (or do at least have sockets or
soldering points for mounting DSP chips), other copiers can be upgraded
externally: By plugging a DSP cartridge into the SNES cartridge slot (either a
regular game cartridge with DSP chip, or a plain DSP-clone-cartridge without
any game in it).

Of course, this will work only with the correct DSP chip (DSP1 for most games;
unless there are any DSP clones that support more than one DSP chip at one?).
Another problem may be I/O addresses (different games expect DSP chips at
different addresses).

**Batteries**

Some boards contain batteries for the internal SRAM. Either 3V Lithium cells
(coin-shaped), or rechargeable 3.6V NiCd batteries (usually with blue coating,
which tend to leak acid, and to destroy wires on the PCB). Other boards don't
have any batteries at all (they are said to use capacitors instead of
batteries, which might be nonsense, or might last only a few minutes?) (there
seems to be no way to switch-off the external power-supply, so batteries aren't
needed to power SRAM) (eventually some boards might even power the DRAM in
standby mode (?), which would require a DRAM refresh generator). And, aside
from battery-backup, most or all copiers are allowing to save SRAM to floppy.

### SNES Cart Copiers - Floppy Disc Controllers

**FDC Chips**

Most (or all) SNES copiers are using one of the following FDCs:

```
  40pin  GM82C765B (DIP)  (Supercom, Ufo, Pro Fighter, Smart Disc, Bung?)
  44pin  GM82C765B (SMD)  (Wild Card, GP-003)
  68pin  MCS3201FN (SMD)  (used by OLD copiers: Super Magic Drive)
  68pin  MCCS3201FN (SMD) (used by OLD copiers: Supercom & Super Magicom)
```

**FDC Address Decoding**

The 68pin MCS3201FN chips include a 10bit address bus (for decoding address
3F0h-3F7h on IBM PCs; whereas, SNES copiers are using only the lower some
address bits), and an 8bit General Purpose Input. The 40pin/44pin GM82C765B
chips include a 1bit address bus bundled with 3 select lines, and have no
General Purpose Input register.

```
  GM82C765B   MCS3201FN  Dir  Register
  N/A         A0-A2=0    R    General Purpose Input (pins I0..I7)
  /LDOR       A0-A2=2    W    Motor Control (bit0-7)
  /CS+A0=0    A0-A2=4    R    Main Status  (NEC uPD765 compatible)
  /CS+A0=1    A0-A2=5    RW   Command/Data (NEC uPD765 compatible)
  /LDCR       A0-A2=7    W    Transfer Rate (Density) (bit0-1)
  N/A         A0-A2=7    R    Bit7=DiskChange, Bit6-0=Zero
```

Accordingly, MCS3201FN ports are always ordered as shown above, whilst
GM82C765B ports can be arranged differently (in case of the Super Wild Card,
Front Fareast kept them arranged the same way as on their older Super Magicom).

**FDC Command/Data and Main Status**

**FDC Motor Control**

```
  GM Bit0-7:  DSEL ,X    ,/RES,DMAEN,MOTOR1,MOTOR2,X     ,MSEL
  MCS Bit0-7: DSEL0,DSEL1,/RES,DMAEN,MOTOR1,MOTOR2,MOTOR3,MOTOR4
```

Note that, for whatever reason, most SNES Copiers are using the SECOND drive
(ie. DSEL=1 instead of DSEL=0, and MOTOR2=1 instead of MOTOR1=1).

**Transfer Rate (Density)**

```
  Val  Usage                 MCS3201FN       GM82C765B
  00h  HD (high density)     500K if /RWC=1  MFM:500K or FM:250K
  01h  DD 5.25" (double den) 300K if /RWC=0  MFM:300K if DRV=1, 250K if DRV=0
  02h  DD 3.5"(double den)   250K if /RWC=0  MFM:250K or FM:125K
  03h  N/A                   Reserved        125K
```

**Disk Change (MCS3201FN only) (not GM82C765B)**

```
  7   Disk Change Flag
  6-0 Unused (zero)
```

Possibly useful, but purpose/usage is unclear. According to the datasheet it is
for "diagnostics" purposes. Unknown when the flag gets reset, and unknown for
which drive(s) it does apply.

### SNES Cart Copiers - Floppy Disc NEC uPD765 Commands

**Accessing the FDC 765**

The Data Register is used to write Commands and Parameters, to read/write data
bytes, and to receive result bytes. These three operations are called Command-,
Execution-, and Result-Phase. The Main Status Register signalizes when the FDC
is ready to send/receive the next byte through the Data Register.

**Command Phase**

A command consists of a command byte (eventually including the MF, MK, SK
bits), and up to eight parameter bytes.

**Execution Phase**

During this phase, the actual data is transferred (if any). Usually that are
the data bytes for the read/written sector(s), except for the Format Track
Command, in that case four bytes for each sector are transferred.

**Result Phase**

Returns up to seven result bytes (depending on the command) that are containing
status information. The Recalibrate and Seek Track commands do not return
result bytes directly, instead the program must wait until the Main Status
Register signalizes that the command has been completed, and then it must (!)
send a Sense Interrupt State command to 'terminate' the Seek/Recalibrate
command.

**FDC Command Table**

```
 Command     Parameters              Exm Result               Description
 02+MF+SK    HU TR HD ?? SZ NM GP SL <R> S0 S1 S2 TR HD NM SZ read track
 03          XX YY                    -                       specify spd/dma
 04          HU                       -  S3                   sense drive state
 05+MT+MF    HU TR HD SC SZ LS GP SL <W> S0 S1 S2 TR HD LS SZ write sector(s)
 06+MT+MF+SK HU TR HD SC SZ LS GP SL <R> S0 S1 S2 TR HD LS SZ read sector(s)
 07          HU                       -                       recalib.seek TP=0
 08          -                        -  S0 TP                sense int.state
 09+MT+MF    HU TR HD SC SZ LS GP SL <W> S0 S1 S2 TR HD LS SZ wr deleted sec(s)
 0A+MF       HU                       -  S0 S1 S2 TR HD LS SZ read ID
 0C+MT+MF+SK HU TR HD SC SZ LS GP SL <R> S0 S1 S2 TR HD LS SZ rd deleted sec(s)
 0D+MF       HU SZ NM GP FB          <W> S0 S1 S2 TR HD LS SZ format track
 0F          HU TP                    -                       seek track n
 11+MT+MF+SK HU TR HD SC SZ LS GP SL <W> S0 S1 S2 TR HD LS SZ scan equal
 19+MT+MF+SK HU TR HD SC SZ LS GP SL <W> S0 S1 S2 TR HD LS SZ scan low or equal
 1D+MT+MF+SK HU TR HD SC SZ LS GP SL <W> S0 S1 S2 TR HD LS SZ scan high or eq.
```

Parameter bits that can be specified in some Command Bytes are:

```
  MT  Bit7  Multi Track (continue multi-sector-function on other head)
  MF  Bit6  MFM-Mode-Bit (Default 1=Double Density)
  SK  Bit5  Skip-Bit (set if secs with deleted DAM shall be skipped)
```

Parameter/Result bytes are:

```
  HU  b0,1=Unit/Drive Number, b2=Physical Head Number, other bits zero
  TP  Physical Track Number
  TR  Track-ID (usually same value as TP)
  HD  Head-ID
  SC  First Sector-ID (sector you want to read)
  SZ  Sector Size (80h shl n) (default=02h for 200h bytes)
  LS  Last Sector-ID (should be same as SC when reading a single sector)
  GP  Gap (default=2Ah except command 0D: default=52h)
  SL  Sectorlen if SZ=0 (default=FFh)
  Sn  Status Register 0..3
  FB  Fillbyte (for the sector data areas) (default=E5h)
  NM  Number of Sectors (default=09h)
  XX  b0..3=headunload n*32ms (8" only), b4..7=steprate (16-n)*2ms
  YY  b0=DMA_disable, b1-7=headload n*4ms (8" only)
```

Format Track: output TR,HD,SC,SZ for each sector during execution phase

Read Track: reads NM sectors (starting with first sec past index hole)

Read ID: read ID bytes for current sec, repeated/undelayed read lists all IDs

Recalib: walks up to 77 tracks, 80tr-drives may need second recalib if failed

Seek/Recalib: All read/write commands will be disabled until succesful senseint

Senseint: Set's IC if unsuccesful (no int has occured) (until IC=0)

**FDC Status Registers**

The Main Status register can be always read through an I/O Port. The other four
Status Registers cannot be read directly, instead they are returned through the
data register as result bytes in response to specific commands.

**Main Status Register (I/O Port)**

```
  b0..3  DB  FDD0..3 Busy (seek/recalib active, until succesful sense intstat)
  b4     CB  FDC Busy (still in command-, execution- or result-phase)
  b5     EXM Execution Mode (still in execution-phase, non_DMA_only)
  b6     DIO Data Input/Output (0=CPU->FDC, 1=FDC->CPU) (see b7)
  b7     RQM Request For Master (1=ready for next byte) (see b6 for direction)
```

**Status Register 0**

```
  b0,1   US  Unit Select (driveno during interrupt)
  b2     HD  Head Address (head during interrupt)
  b3     NR  Not Ready (drive not ready or non-existing 2nd head selected)
  b4     EC  Equipment Check (drive failure or recalibrate failed (retry))
  b5     SE  Seek End (Set if seek-command completed)
  b6,7   IC  Interrupt Code (0=OK, 1=aborted:readfail/OK if EN, 2=unknown cmd
             or senseint with no int occured, 3=aborted:disc removed etc.)
```

**Status Register 1**

```
  b0     MA  Missing Address Mark (Sector_ID or DAM not found)
  b1     NW  Not Writeable (tried to write/format disc with wprot_tab=on)
  b2     ND  No Data (Sector_ID not found, CRC fail in ID_field)
  b3,6   0   Not used
  b4     OR  Over Run (CPU too slow in execution-phase (ca. 26us/Byte))
  b5     DE  Data Error (CRC-fail in ID- or Data-Field)
  b7     EN  End of Track (set past most read/write commands) (see IC)
```

**Status Register 2**

```
  b0     MD  Missing Address Mark in Data Field (DAM not found)
  b1     BC  Bad Cylinder (read/programmed track-ID different and read-ID = FF)
  b2     SN  Scan Not Satisfied (no fitting sector found)
  b3     SH  Scan Equal Hit (equal)
  b4     WC  Wrong Cylinder (read/programmed track-ID different) (see b1)
  b5     DD  Data Error in Data Field (CRC-fail in data-field)
  b6     CM  Control Mark (read/scan command found sector with deleted DAM)
  b7     0   Not Used
```

**Status Register 3**

```
  b0,1   US  Unit Select (pin 28,29 of FDC)
  b2     HD  Head Address (pin 27 of FDC)
  b3     TS  Two Side (0=yes, 1=no (!))   GM82C765: Also WP (same as bit6)?
  b4     T0  Track 0 (on track 0 we are)
  b5     RY  Ready (drive ready signal)   GM82C765: Always 1=Ready
  b6     WP  Write Protected (write protected)
  b7     FT  Fault (if supported: 1=Drive failure) GM82C765: Always 0=Okay
```

**Notes:**

Before accessing a disk you should first Recalibrate the drive, that'll move
the head backwards until it reaches Track 0 (that's required to initialize the
FDCs track counter). On a 80 track drive you may need to repeat that in case
that the first recalibration attempt wasn't successful (that's because the FDC
stops searching after 77 steps) (at least older uPD765 chips did so, maybe the
MCS3201FN/GM82C765B chips don't).

Now if you want to format, read or write a sector on a specific track you must
first Seek that track (command 0Fh). That'll move the read/write head to the
physical track number. If you don't do that, then the FDC will attempt to
read/write data to/from the current physical track, independendly of the
specified logical Track-ID.

The Track-, Sector-, and Head-IDs are logical IDs only. These logical IDs are
defined when formatting the disk, and aren't required to be identical to the
physical Track, Sector, or Head numbers. However, when reading or writing a
sector you must specify the same IDs that have been used during formatting.

Despite of the confusing name, a sector with a "Deleted Data Address Mark"
(DAM) is not deleted. The DAM-flag is just another ID-bit, and (if that ID-bit
is specified correctly in the command) it can be read/written like normal data
sectors.

**DMA/IRQ**

Most (or all) SNES copiers don't support DMA or IRQs (some are allowing to poll
the IRQ flag by software I/O).

**Terminal Count (TC)**

*** Below info applies to Amstrad CPC with uPD765 chip.

*** Unknown if anything similar applies to SNES with MCS3201FN/GM82C765B chips.

At the end of a successful read/write command, the program should send a
Terminal Count (TC) signal to the FDC. However, in the CPC the TC pin isn't
connected to the I/O bus, making it impossible for the program to confirm a
correct operation. For that reason, the FDC will assume that the command has
failed, and it'll return both Bit 6 in Status Register 0 and Bit 7 in Status
Register 1 set. The program should ignore this errormessage.

### SNES Cart Copiers - Floppy Disc FAT12 Format

The SNES Copier floppy format is compatible to that used under DOS on PCs.

Typical formats are 3.5", Double Density, 80 Tracks/9 Sectors, Double Sided
(720KB). The Sectors are logically numbered 01h..09h, and each sized 200h
bytes.

XXX HD-disks have more sectors

XXX snes copiers are usually HD (or maybe some are DD?)

XXX snes copiers support 1.44MB and 1.6MB (FDFORMAT-like)

**Boot-Record**

The first sector is always used as bootsector, giving information about the
usage of the following sectors, and including the boot procedure (for loading
MSDOS etc).

```
  00-02       80x86 boot procedure (jmp opcode) (not used for SNES)
  03-0A       ascii disk name
  0B-0C       bytes / sector
  0D          sectors / cluster
  0E-0F       sectors / boot-record
  10          number of FAT-copys
  11-12       entrys / root-directory
  13-14       sectors / disk
  15          ID: F8=hdd, F9=3.5", FC=SS/9sec, FD=DS9, FE=SS8,FF=DS8
  16-17       sectors / FAT
  18-19       sectors / track
  1A-1B       heads / disk
  1C-1D       number of reserved sectors
  1E-1FF      MSX boot procedure (Z80 code) (not used for SNES)
```

**FAT and FAT copy(s)**

The following sectors are occupied by the File Allocation Table (FAT), which
contains 12- or 16-bit entries for each cluster:

```
  (0)000      unused, free
  (0)001      ???
  (0)002...   pointer to next cluster in chain (0)002..(F)FEF
  (F)FF0-6    reserved (no part of chain, not free)
  (F)FF7      defect cluster, don't use
  (F)FF8-F    last cluster of chain
```

Number and size of FATs can be calculated by the information in the boot
sector.

**Root directory**

The following sectors are the Root directory, again, size depends on the info
in bootsector. Each entry consists of 32 bytes:

```
  00-07       Filename (first byte: 00=free entry,2E=dir, E5=deleted entry)
  08-0A       Filename extension
  0B          Fileattribute
  0C-15       reserved
  16-17       Timestamp: HHHHHMMM, MMMSSSSS
  18-19       Datestamp: YYYYYYYM, MMMDDDDD
  1A-1B       Pointer to first cluster of file
  1C-1F       Filesize in bytes
```

The 'cluster' entry points to the first used cluster of the file. The FAT entry
for that cluster points to the next used cluster (if any), the FAT entry for
that cluster points to the next cluster, and so on.

**Reserved Sectors (if any)**

Usually the number of reserved sectors is zero. If it is non-zero, then the
following sector(s) are reserved (and could be used by the boot procedure for
whatever purposes).

**Data Clusters 0002..nnnn**

Finally all following sectors are data clusters. The first cluster is called
cluster number (0)002, followed by number (0)003, (0)004, and so on.

**Special Features**

Unknown if any copiers support sub-directories.

Unknown if any copiers support long file names.

Unknown if any copiers support compressed files (ZIP or such).

### SNES Cart Copiers - BIOSes

**Copier BIOSes**

```
  Name                                    I/O   BIOS Size
  Double Pro Fighter (1994)               2800  64K(6+6+16)
  Gamars Puzzle (not a Copier BIOS)       -     1M (32x32K) GAMARS~5 1,048,576
  Gamars Super Disk FC-301 V6.0  Kaiser94 5Fxx  64K (1x64K) GAMARS~4    65,536
  Gamars Super Disk FC-301 V7.13 Kaiser94 5Fxx 256K (4x64K) GAMARS~3   262,144
  Gamars Super Disk FC-301 V7.16 Kaiser94 5Fxx 256K (4x64K) GAMARS~2   262,144
  Game Doctor SF 3 V3.3C                  8000  32K (1x32K) GAMEDO~3    32,768
  Game Doctor SF 6 V6.2  (Professor SF)   8000  64K (2x32K) GAMEDO~4    65,536
  Game Doctor SF 6 V6.21 (Professor SF)   8000  64K (2x32K) GAMEDO~6    65,536
  Game Doctor SF 7 V7.11 (Professor SF 2) 8000  64K (2x32K) GAMEDO~2    65,536
  Multi Game Hunter V1.2 (Venus)          5800  32K (1x32K) MULTIG~2    32,768
  Multi Game Hunter V1.3 (Venus)          5800  32K (1x32K) MULTIG~3    32,768
  Multi Game Hunter V1.4 (Venus)          5800  32K (1x32K) MULTIG~3    32,768
  Pro Fighter Q (H.K.)           xx-xx-93 2800  16K (1x16K) SUPERP~1    16,xxx
  Supercom Partner A   [o1]               2800  16K (1x16K) SUPERC~1 3,145,728
  Supercom Pro 2 (CCL) ports=FFE 06-21-92 FFE    8K (1x8K)  SUPERC~2    32,768
  Super Disk Interceptor v5.2 (Sane Ting) A000  32K (4x8K)  SUPERD~1    32,768
  Super Magicom V1H (Front/CCL)  12-23-91 FFE    8K (1x8K)  SUPERM~2    32,768
  Super Magicom V31 (Front/CCL)  xx-xx-92 FFE    8K (1x8K)  SUPERM~4    32,768
  Super Magicom V3H SoftUpgrade  xx-xx-9x FFE   32K (DRAM)  SUPERM~8    32,768
  Super Pro Fighter (H.K.)       xx-xx-93 2800  16K (1x16K) SUPERP~1    16,xxx
  Super Pro Fighter (H.K.) [a1]  xx-xx-93 2800  16K (1x16K) SUPERP~1    16,xxx
  Super Wild Card V1.6   (Front) 93-01-26 FFE   16K (2x8K)  SUPERW~6    16,384
  Super Wild Card V1.8   (Front) 93-02-19 FFE   16K (2x8K)  SUPERW~7    16,384
  Super Wild Card V2.0XL (Front) 93-04-12 FFE   16K (2x8K)  SUPERW~9    16,384
  Super Wild Card V2.1B  (Front) 93-04-28 FFE   16K (2x8K)  SUPER~10    16,384
  Super Wild Card V2.1C  (Front) 93-04-28 FFE   16K (2x8K)  SUPER~11    16,384
  Super Wild Card V2.2CC (Front) 93-05-03 FFE   16K (2x8K)  SUPER~12    16,384
  Super Wild Card V2.6CC (Front) 93-07-17 FFE   16K (2x8K)  SUPER~15    16,384
  Super Wild Card V2.6F  (Front) 93-07-17 FFE   16K (2x8K)  SUPER~16    16,384
  Super Wild Card V2.6FX (Front) 93-07-17 FFE   16K (2x8K)  SUPER~17    16,384
  Super Wild Card V2.7CC (Front) 93-12-07 FFE   16K (2x8K)  SUPER~18    16,384
  Super Wild Card V2.8CC (Front) 06-08-94 FFE   16K (2x8K)  SUPER~19    16,384
  Super Wild Card V2.8CC   [o1]  06-28-94 FFE   16K (2x8K)  SUPER~22    65,536
  Super Wild Card DX             10-14-94 FFE  256K (32x8K) SUPERW~2   262,144
  Super Wild Card DX             11-03-94 FFE  256K (32x8K) SUPERW~3   262,144
  Super Wild Card DX96           01-04-96 FFE  256K (32x8K) SUPERW~1   262,144
  Super Wild Card DX2            06-08-96 FFE  256K (32x8K) SUPERW~4   262,144
  UFO - Super Drive PRO 3  [o1 as 4x8K]   FFE    8K (1x8K)  UFOSUP~1    32,768
  UFO - Pro 6                             ?       ? (?)
  UFO - Super UFO Pro-7 V7.3     1994     2184  64K (2x32K) SUPERU~1    65,536
  UFO - Super UFO Pro-8 V8.1     1995     2184 128K (4x32K) SUPERU~2   131,072
  UFO - Super UFO Pro-8 V8.8c    1995     2184 256K (8x32K) SUPERU~3   262,144
```

### SNES Cart CDROM Drive

**SNES Sony CDROM (unreleased)**

Nintendo and Sony originally planned a partnership where Sony would produce a
CDROM drive add-on for the SNES, additionally Sony would have produced a SNES
compatible console with the CDROM drive built-in. The project progressed far
enough to produce some prototype, and publish some press releases.

However, the deal failed, and Sony finally produced their own console (the Sony
Playstation). Anyways, the unreleased SNES CD Prototype worked as so:

For general info about CDROM discs, see the documentation for my PSX debugger:

```
  http://problemkaputt.de/psx.htm - no$psx homepage
  http://problemkaputt.de/psx-spx.htm - psx specifications
```

That docs are covering about everything about sector headers, sector encoding,
subchannels, tocs, tracks, sectors, frames, sessions, volume descriptors,
filesystem, xa-adpcm, cd-da audio, plus specs for different cdrom-image file
formats.

**SNES Philips CDROM (unreleased)**

After the deal with Sony had failed, Nintendo tried a new deal with Philips -
which failed, too.

**SNES Copier CDROM (released)**

Whilst Nintendo failed on producing an official CDROM drive, some SNES Copiers
are allowing to load ROM-images from CDROMs.

### SNES Cart CDROM - Memory and I/O Map

**I/O Ports**

```
  21D0h.W   - BIOS Cartridge Battery RAM Lock (write 00h)
  21E0h.W   - BIOS Cartridge Battery RAM Unlock Step 2 (write 0Fh downto 01h)
  21E1h.R/W - CDROM Unit Mechacon CPU (probably the NEC chip on daughterboard)
  21E2h.R/W - CDROM Unit Decoder/FIFO Index (CXD1800Q chip)
  21E3h.R/W - CDROM Unit Decoder/FIFO Data  (CXD1800Q chip)
  21E4h.W   - CDROM Unit (?) Whatever Control/Enable or so
  21E5h.W   - BIOS Cartridge Battery RAM Unlock Step 1 (write FFh)
  ???.R/W   - NEXT connector? (maybe some kind of UART, like PSX serial port?)
  ???.R/W   - BIOS Cartridge S-WRAM chip(s) (seem be wired to /PARD and /PAWR)
  IRQ       - used for Decoder and Mechacon
```

**APU I/O Ports**

The SNES CD prototype has APU chips with uncommon part numbers, which might
work slightly different than standard SNES APUs. However, adding that chips
wouldn't be possible with SNES CD expansions (for existing SNES consoles).
Either old SNES consoles would need to stick with old APUs, or, theoretically,
the SNES CD expansions could contain an extra APU unit (but, mapped elsewhere
than 2140h-2143h).

**Memory**

```
  00h-03h:8000h-FFFFh  BIOS Cart ROM (128Kbyte LoROM)
  80h-87h:8000h-FFFFh  BIOS Cart Work RAM (256Kbyte DRAM) (two S-WRAM chips)
  90h    :8000h-9FFFh  BIOS Cart Battery RAM (8Kbyte SRAM)
```

Special Memory regions/addresses:

```
  00h:1Fxxh  Work RAM reserved for BIOS functions
  00h:1FF8h  Work RAM containing NMI vector (should be 4-byte "JMP far" opcode)
  00h:1FFCh  Work RAM containing IRQ vector (should be 4-byte "JMP far" opcode)
  00h:0000h  Work RAM containing IRQ/BRK/COP vectors (if used)
  00h:1000h  Load address for 800h-byte boot sector
  00h:1080h  Entrypoint for 800h-byte boot sector
  00h:E000h  CD BIOS Functions in BIOS ROM
  83h:C000h  Work RAM reserved for loading cdrom data in "VRAM mode" (16Kbyte)
```

Caution: Initial/empty SRAM may NOT be zerofilled (else the BIOS treats the
checksum to be okay, with 0 files installed - but with 0000h bytes free space,
which is making it impossible to create/delete any files).

Caution: RAM at 1Fxxh is reserved for BIOS functions (and NMI/IRQ vectors, even
when not using any other BIOS functions), so stacktop should be 1EFFh (not
1FFFh, where it'd be usually located).

Unknown if the memory is mirrored anywhere; particulary mirroring the S-WRAMs
to C0h-C3h:0000h-FFFFh would be useful for HiROM-style games.

Unknown if the two S-WRAM chips are also mapped to B-bus (the B-bus would be
useful only for DMA from ROM carts, ie. not useful for CDROM games).

**21E4h.W - Whatever Control/Enable or so**

```
  7-4   Unknown/Unused (always set to 0)
  3     Enable Mechacon?      (0=Off, 1=On)
  2     Enable Decoder?       (0=Off, 1=On)
  1     Maybe Reset?          (0=Normal, 1=What?)
  0     Unknown/Unused (always set to 0)
```

Set to 0Eh,00h,04h,08h,0Ch.

**Decoder/FIFO Registers (CXD1800Q) (accessed via 21E2h/21E3h)**

```
 Decoder Write Registers
  00h     -           Reserved
  01h     DRVIF       DRIVE Interface (W)
  02h     CHPCTL      Chip Control (W)
  03h     DECCTL      Decoder Control (W)
  04h     INTMSK      Interrupt Mask (0=Disable, 1=Enable) (W)      ;\interrupt
  05h     INTCLR      Interrupt Clear/Ack (0=No change, 1=Clear/ack);/
  06h     CI          ADPCM Coding Information (to be used when AUTOCI=0)
  07h     DMAADRC_L   SRAM-to-CPU Xfer Address, Low (W)               ;\
  08h     DMAADRC_H   SRAM-to-CPU Xfer Address, High (W)              ;
  09h     DMAXFRC_L   SRAM-to-CPU Xfer Length, Low (W)                ;
  0Ah     DMAXFRC_H   SRAM-to-CPU Xfer Length, High & DMA Control (W) ;/
  0Bh     DRVADRC_L   Disc-to-SRAM Xfer Address, Low (W)              ;\
  0Ch     DRVADRC_H   Disc-to-SRAM Xfer Address, High (W)             ;/
  0Dh-0Fh -           Unspecified
  0Dh     "PLBA"      <-- shown as so in SNES CD's "CXD1800" test screen
  10h-1Ch -           Mirrors of 00h-0Ch
  1Dh     -           Reserved (TEST2)
  1Eh     -           Reserved (TEST1)
  1Fh     -           Reserved (TEST0)
 Decoder Read Registers
  00h     DMADATA     SRAM-to-CPU Xfer Data (R)             ;-Sector Data
  01h     INTSTS      Interrupt Status (0=No IRQ, 1=IRQ) (R);-Interrupt
  02h     STS         Status (R)                            ;\
  03h     HDRFLG      Header Flags (R)                      ;
  04h     HDR_MIN     Header "MM" Minute (R)                ; important info on
  05h     HDR_SEC     Header "SS" Second (R)                ; current sector
  06h     HDR_BLOCK   Header "FF" Frame (R)                 ; (to be handled
  07h     HDR_MODE    Header Mode (R)                       ; upon "DECINT"
  08h     SHDR_FILE   Sub-Header File (R)                   ; interrupt)
  09h     SHDR_CH     Sub-Header Channel (R)                ;
  0Ah     SHDR_S-MODE Sub-Header SubMode (R)                ;
  0Bh     SHDR_CI     Sub-Header Coding Info (R)            ;
  0Ch     CMADR_L     Current Minute Address, Low (R)       ;
  0Dh     CMADR_H     Current Minute Address, High (R)      ;/
  0Eh     MDFM        MODE/FORM (R)                         ;\extra details on
  0Fh     ADPCI       ADPCM Coding Information (R)          ;/current sector
  10h-to-2            Reserved (TEST 0 to 2) (R)
  13h     -           Unspecified
  14h-17h -           Mirrors of 04h-07h (HDR_xxx)
  18h.R   DMAXFRC_L - SRAM-to-CPU Xfer Length, Low (R)      ;\allows to read
  19h.R   DMAXFRC_H - SRAM-to-CPU Xfer Length, High (R)     ; address/remain
  1Ah.R   DMAADRC_L - SRAM-to-CPU Xfer Address, Low (R)     ; values
  1Bh.R   DMAADRC_H - SRAM-to-CPU Xfer Address, High (R)    ; (needed only for
  1Ch.R   DRVADRC_L - Disc-to-SRAM Xfer Address, Low (R)    ; diagnostics)
  1Dh.R   DRVADRC_H - Disc-to-SRAM Xfer Address, High (R)   ;/
  1Eh-1Fh -           Mirrors of 0Eh-0Fh (MDFM and ADPCI)
```

### SNES Cart CDROM - CDROM Bootsector and Volume Descriptor

SNES CD can be in MODE1 or MODE2/FORM1 format. The disc requires an 28h-byte ID
in sector 16, and a 800h-byte bootsector in sector 0, which may then loaded
further data via BIOS functions, or via direct access to the cdrom I/O ports.

The BIOS doesn't contain any filesystem support, however, the games may
implement a standard ISO filesystem (or some custom format), if desired.

Aside from data sectors, the drive controller does also support CD-DA audio
tracks and playing compressed ADPCM audio sectors.

**SNES CD Bootsector (sector 0)**

Located on Sector 0 (address 00:02:00), loaded to 00:1000h..17FFh, and then
started by jumping to 00:1080h.

**Primary Volume Descriptor (sector 16)**

Located on Sector 16 (address 00:02:16), the first 28h bytes must have
following values for boot-able SNES CDs.

```
  000h 1    Volume Descriptor Type        (01h=Primary Volume Descriptor)
  001h 5    Standard Identifier           ("CD001")
  006h 1    Volume Descriptor Version     (01h=Standard)
  007h 1    Reserved                      (00h)
  008h 32   System Identifier             (a-characters) ("SUPERDISC")
  028h ...  (further ISO primary volume descriptor entries may follow here)
```

**Note**

Aside from booting executable software, the CD BIOS does also contain code for
some "ELECTRONIC BOOK" format, but the volume descriptor detection lacks
support for detecting that disc type.

### SNES Cart CDROM - BIOS Cartridge

Contains extra DRAM, some small battery-backed SRAM, and the BIOS ROM. The DRAM
and SRAM are rather small, and there's no coprocessor. However, this is only
prototype, and Nintendo could have easly expanded the BIOS cartridge (without
needing to modify the actual CDROM hardware).

For example, there have been rumours about a 32bit CPU being planned, and SRAM
might have been intended to be replaced by a bigger memory chip (or possibly by
an external FLASH cart as used in Satellaview BIOS carts).

**BIOS User Interface**

```
  START  --> Load CDROM (if any)
  SELECT --> SRAM Manager (in there: Up/Down=Select, B=Delete, Y=Exit)
  A+X    --> Test Screen (in there: Up/Down/B --> Menu Selection)
```

Self Check tests:

```
  Page1: VRAM, CGRAM, OAM, WRAM, DMA, TIMER, SOUND (sound test works only once)
  Page2: BIOS_DRAM, BIOS_SRAM, CDROM DECODER, CD-PLAYER I/F
  The DECODER test seems to try to count sectors/second on STOPPED drive,
  that might fail on real HW, or it might work with the NOSYNC bit triggered?
```

ADPCM Test:

```
  Use Up/Down and L/R Buttons to select File/Channel and MM:SS:FF
  Press B to play ADPCM audio (eg. from PSX disc with ADPCM at selected values)
  Press Y to toggle Normal/Double speed, press Select to go back to menu
  Observe that APU is muting sound output (unless previously running Selfcheck)
```

Communication (Mechacon) Test:

```
  Use L/R Buttons to select a command, use B to issue the command, Select=Exit
  Use Up/Down and L/R Buttons to change variable parameters
```

CXD-1800 (Decoder) Test:

```
  Use Up/Down and L/R Buttons to change Write values
  Use Y to toggle Read/Write, X to toggle IRQ, Select=Exit
```

**00h-03h:8000h-FFFFh - BIOS Cart ROM (128Kbyte LoROM) (Sticker 0.95 SX)**

The BIOS has CRC32=3B64A370h and the ROM/EPROM is badged "0.95 SX", there are
some ASCII strings in the file:

```
  "Super Disc boot ROM ver.0.95 Jul. 14, 1992 by Tomomi Abe at SONY "
  "Super Disc BIOS program ver.0.93 by Tomomi Abe. May. 26 1992 at SONY. "
  01h,"CD001",01h,00h,"SUPERDISC",23x00h  ;28h-byte ISO volume descriptor
```

The cart header at 7FC0h-7FDFh is just FFh-filled and IRQ/NMI vectors point to
RAM:

```
  7FC0  FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF
  7FD0  FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF
  7FE0  00,00,00,00,00,00,00,00,00,00,F8,1F,00,00,FC,1F
  7FF0  00,00,00,00,00,00,00,00,00,00,00,00,00,80,00,00
```

That uncommon combination of FFh's and IRQ/NMI vectors can be used to detect if
a ROM image is having Super Disc support.

**80h-87h:8000h-FFFFh - BIOS Cart Work RAM (256Kbyte DRAM) (two S-WRAM chips)**

This expands the SNES's internal 128KBytes to a total of 384Kbytes Work RAM.
Allowing to load code and data from CDROM to CPU memory space.

**90h:8000h-9FFFh - BIOS Cart Battery RAM (8Kbyte SRAM)**

**21D0h.W - BIOS Cartridge Battery RAM Lock (write 00h)**

**21E0h.W - BIOS Cartridge Battery RAM Unlock Step 2 (write 0Fh downto 01h)**

**21E5h.W - BIOS Cartridge Battery RAM Unlock Step 1 (write FFh)**

These ports seem to be used to write-protect the battery backed SRAM (the BIOS
functions are automatically locking/unlocking the SRAM when saving/deleting
game position files).

### SNES Cart CDROM - BIOS Functions

**SNES CD BIOS Function Summary (jump opcodes at 00:E0xxh)**

```
  00h:E000h  cdrom_InitDetect          ;00h       ;\
  00h:E003h  cdrom_LoadFromDisc        ;01h       ;
  00h:E006h  cdrom_SendMechaconCommand ;02h       ; Main Functions
  00h:E009h  cdrom_WramToVramDMA       ;03h       ;
  00h:E00Ch  cdrom_PollMechacon        ;04h       ;/
  00h:E00Fh  no_function               ;05h..0Fh
  00h:E030h  cdrom_SramTest            ;10h       ;\
  00h:E033h  cdrom_SramGetDirectory    ;11h       ;
  00h:E036h  cdrom_SramSaveFile        ;12h       ; SRAM Functions
  00h:E039h  cdrom_SramLoadFile        ;13h       ;
  00h:E03Ch  cdrom_SramDeleteFile      ;14h       ;/
  00h:E03Fh  no_function               ;15h..1Fh
  00h:E060h  cdrom_DecoderDataMode     ;20h       ;\
  00h:E063h  cdrom_DecoderAudioMode    ;21h       ; Misc Functions
  00h:E066h  cdrom_DecoderTestDecint   ;22h       ;/
  00h:E069h  no_function               ;23h
  00h:Exxxh  crash                     ;24h and up
```

```
 ______________________________ Main Functions ______________________________
```

**00h:E000h - cdrom_InitDetect**

Initializes variables and NMI/IRQ handlers at [1Fxxh], and tries to flush any
old mechacon IRQs, and to issue a mechacon get status command.

```
  out: cy=error (0=okay, 1=no cdrom hardware)
```

**00h:E003h - cdrom_LoadFromDisc**

```
  in: [1F00h]=source address (24bit LBA, or 3-byte MM,SS,FF address)
  in: [1F03h]=read mode (flag byte) (usually 40h for LBA with normal data)
  in: [1F04h]=destination address (24bit wram address) (or 16bit vram address)
  in: [1F07h]=transfer length (max 7FFFh bytes, or maybe max 87FFh works, too)
  in: [1F09h]=max number of sub-q mismatches or so (usually 0Fh)
  in: [1F33h]=file and channel bytes (for ADPCM mode only)
  out: cy=error (0=okay, 1=bad)
```

Flag byte format:

```
  7    VRAM Mode (0=Load to WRAM, 1=Forward sectors from WRAM to VRAM)
  6    Source Address format (0=MM:SS:FF in non-BCD, 1=24bit LBA)
  5    ADPCM Mode (0=No, 1=Play ADPCM file/channel until EOR/EOF)
  4    Prevent loading (0=No, 1=Skip everything except ADPCM, if enabled)
  3-0  Unused (should be 0)
```

The cdrom_LoadFromDisc function uses DMA7 to transfer data from Disc to WRAM,
the "VRAM Mode" additionally uses DMA6 for forwarding incoming data from a WRAM
buffer (at 83h:C000h-FFFFh) to VRAM.

**00h:E006h - cdrom_SendMechaconCommand**

Allows to send mechacon commands (normally not required, the LoadFromDisc
functions does automatically issue seek+play+pause commands).

```
  in: a=command (8bit)
  in: [1Fxxh]=optional parameters (for command 00h and 01h)
  out: cy=error (0=okay, 1=bad)
  out: [1F2E]=last response digit (unknown purpose, checked after seek_mmssff)
```

Command numbers are:

```
  00h  seek_tr_indx   CxxxxF --> FFFFFx       ;in: [1F0Fh..1F12h]=four nibbles
  01h  seek_mmssff    BxxxxxxF --> FFFFFFFx   ;in: [1F13h..1F18h]=six nibbles
  02h  stop           D01F --> FFFx
  03h  play           D02F --> FFFx
  04h  pause          D03F --> FFFx
  05h  open_close     D04F --> FFFx
  06h  fast_forward   D10F --> FFFx
  07h  fast_reverse   D11F --> FFFx
  08h  forward        D12F --> FFFx
  09h  reverse        D13F --> FFFx
  0Ah  key_direct     D40F --> FFFx
  0Bh  key_ignore     D41F --> FFFx
  0Ch  continous      D42F --> FFFx
  0Dh  track_pause    D43F --> FFFx
  0Eh  index_pause    D44F --> FFFx
  0Fh  req_sub_q      D50F_0000000000000000F  ;out:[1F1Eh..1F2Dh]=16 nibbles
  10h  req_status     D51F_01234F             ;out:[1F19h..1F1Dh]=5 nibbles
  11h  normal_speed   D45F --> FFFx
  12h  double_speed   D46F --> FFFx
  13h  flush          F --> a
  N/A  ?              D14F --> FFFx
  N/A  ?              D15F --> FFFx
```

**00h:E009h - cdrom_WramToVramDMA (custom NMI handler callback)**

Usually done automatically by the default BIOS NMI handler: If CDROM loading is
done in "VRAM mode", then this functions forwards the incoming CDROM data from
WRAM to VRAM.

**00h:E00Ch - cdrom_PollMechacon (custom IRQ handler callback)**

Usually done automatically by the default BIOS IRQ handler: If a mechacon
command is being transmitted, then this function handles incoming mechacon
response nibbles, and sends further mechacon parameter nibbles (until
completion of the command sequence).

```
 ______________________________ SRAM Functions ______________________________
```

**00h:E030h - cdrom_SramTest**

Tests the SRAM checksum, does range checks on free memory size and number of
files, automatically reformats/erases the SRAM in case of errors.

```
  out: cy=error (0=okay, 1=bad, reformatted sram)
```

**00h:E033h - cdrom_SramGetDirectory**

Returns the whole SRAM directory with max 32 files, each 16-byte entry consists
of 14-byte filename, folled by 16bit filesize value.

```
  in: DB:Y = destination address (200h byte buffer)
  out: cy=error (0=okay, 1=bad)
  out: a=number of files actually used                      ;\returned only
  out: [DB:Y+0..1FF]=directory (unused entries 00h-filled)  ;/when cy=0=okay
```

**00h:E036h - cdrom_SramSaveFile**

```
  in: DB:Y = source address (14-byte name, 16bit length, filebody[length])
  out: cy=error (0=okay, 1=bad, directory or memory full)
```

**00h:E039h - cdrom_SramLoadFile**

```
  in: DB:Y = source address (14-byte name, 16bit length, filebody[length])
  out: filebody[length] is overwritten by loaded file
       (zeropadded if the specified length exceeded the specified filesize)
  out: cy=error (0=okay, 1=bad, file not found)
```

**00h:E03Ch - cdrom_SramDeleteFile**

```
  in: DB:Y = source address (14-byte name)
  out: cy=error (0=okay, 1=bad, file not found)
```

**Character Set for SRAM Filenames (shown when pressing SELECT in BIOS)**

```
  00h..09h  "0..9"
  0Ah..23h  "A..Z"
  24h..27h  Space, Slash, Dash, Dot
  28h..7Fh  Japanese symbols
  80h..FFh  Cause directory sort-order corruption when creating/deleting files
```

```
 ______________________________ Misc Functions ______________________________
```

**00h:E060h - cdrom_DecoderDataMode**

**00h:E063h - cdrom_DecoderAudioMode (CD-DA)**

These functions are just setting the decoder to data/audio mode, there are no
parameters or return values.

**00h:E066h - cdrom_DecoderTestDecint**

Runs a test on measuring the number of DECINT's per second (aka sectors per
second), passes okay when measuring 75+/-5 or 150+/-10 DECINTs (ie. both single
& double speed mode should pass). Execution time of the test is 1 second.

```
  out: cy=error (0=okay, 1=bad)
```

```
 ______________________________ Bugs & Glitches _____________________________
```

Instead of using unsigned maths, the BIOS used a lot of signed comparisions
without overflow checking.

This is restricting the CDROM filesize to max 7FFFh (or possibly 87FFh might
work when subtracting the first sector unit).

SRAM filename characters are also using that signed maths for the filename sort
order (using characters 80h..FFh can have unpredictable results when
adding/removing SRAM files; which may cause new comparision overflows to
occur/disappear).

SRAM is intended to hold max 32 files, however, that limit is checked when
overwriting old files (not when creating new files): Results are that one
cannot overwrite any files if the cart contains 32 files or more, whilst, on
the other hand, one could create even more then 32 files.

Booting the BIOS seems to be instantly STOPPING the drive motor (after the BIOS
intro/delay), apparently preventing the drive to spin-up, and to read the TOC,
or even to load data from the disc - until going through the "PRESS START" nag
screen.

### SNES Cart CDROM - Mechacon

The Mechacon handles all the drive mechanics (motor start/stop, seeking,
tracking, gain, balance). Essentinally it's covering only the "Audio" part
(streaming bits and watching the SubQ-channel's position info) without being
aware of "Digital" data in CDROM Headers & Data Blocks.

However, the same mechanics are also used for "Playing" CDROM data discs (ie.
seek the desired sector in MM:SS:FF notation, then issue Play command to start
reading).

Observe that seeking may inaccuratly settle "nearby" of the desired target
address (ie. one must check the Data header's MM:SS:FF bytes from the Decoder
chip, and ignore any sectors with smaller sector numbers, or eventually retry
seeking if the sector number is higher as planned).

**21E1h.R/W - CDROM Unit Mechacon CPU (probably the NEC chip on daughterboard)**

```
  7     Transfer Ready IRQ      (R)
  6-4   -
  3-0   Data                    (R/W)
```

**Mechacon Commands**

```
  Access MM/SS/FF     BmmssffF                --> FFFFFFFx
  Access Track/Index  CttiiF                  --> FFFFFx
  Stop                D01F                    --> FFFx
  Play                D02F                    --> FFFx
  Pause               D03F                    --> FFFx
  Open/Close          D04F                    --> FFFx
  Fast Forward        D10F                    --> FFFx
  Fast Reverse        D11F                    --> FFFx
  Forward             D12F                    --> FFFx
  Reverse             D13F                    --> FFFx
  Key Direct          D40F                    --> FFFx
  Key Ignore          D41F                    --> FFFx
  Continous Play      D42F                    --> FFFx
  Auto Track Pause    D43F                    --> FFFx
  Auto Index Pause    D44F                    --> FFFx
  Normal Speed        D45F                    --> FFFx
  Double Speed        D46F                    --> FFFx
  Q-Data Request      D50F 0000000000000000F  --> FFFx ................x
  Status Request      D51F 01234F             --> FFFx .....x
  Nop/Flush ?         F                       --> x
```

**Q-Data Request Digits**

These 16 digits are probably 8 bytes straight from 12-byte SubQ Position data
in BCD format (probably Track, Index, MM:SS:FF, AMM:ASS:AFF) (ie. probably
excluding the ADR/Control byte, Reserved byte, and the two CRC bytes).

**Status Request Digits**

```
 Digit(0) - Disc Type
  Bit0: Disc Type (or maybe Track Type) (0=Audio, 1=Data)
  Bit1-3: Unknown/unused
 Digit(1)
  Unknown/unused
 Digit(2) - Drive state
  00h  No Disc
  01h  Stop
  02h  Play
  03h  Pause
  04h  Fast Reverse
  05h  Fast Forward
  06h  Slow Reverse
  07h  Slow Forward
  08h  ?
  09h  ?
  0Ah  Access, Seek
  0Bh  Access, Read TOC
  0Ch  Tray Open
  0Dh  ?
  0Eh  ?
  0Fh  ?
 Digit(3)
  Unknown/unused
 Digit(4)
  Unknown/unused
```

Unknown bits & digits might include double-speed flag, LCD pad buttons, or
such stuff.

### SNES Cart CDROM - Decoder/FIFO

CXD1800Q chip (equivalent to CXD1196AR datasheet).

IRQs can be sensed via CXD1800 Register(01h.R).

**21E2h.R/W - CDROM Unit CXD1800 Index (REGADR) (R/W)**

```
  7-5  -      Reserved (should be 0)
  4-0  RA4-0  Register Index
```

This register is used for selection of the internal registers.

```
 --> When the low order 4 bits of REGADR are not 0 (hex), and a register write
     or read is made by setting A0=1 and /CS=0, the low order 4 bits of
     REGADR are incremented
 --> REGADR is cleared to 00h by rising edge of DMAEN (in DMA Control register)
```

**21E3h.R/W - CDROM Unit CXD1800 Data (R/W)**

```
  7-0  Data for register selected via REGADR
```

```
 _________________________ Configuration _________________________
```

**X1h.W - DRVIF - DRIVE Interface (W)**

```
  7   XSLOW    DMA/SRAM Speed (0=Slow/12 clks/320ns, 1=Fast/4 clks/120ns)
  6   C2PL1ST  DATA input C2PO-byte-order (0=Upper first, 1=Lower first)
  5   LCHLOW   Audio LRCK Polarity for Left channel (0=High, 1=Low)
  4   BCKRED   Audio BCLK Edge for strobing DATA (0=Falling, 1=Rising)
  3-2 BCKMD1-0 Audio BCLKs per WCLK cycle (0=16, 1=24, 2/3=32)
  1   LSB1ST   Audio DATA (bit?-)ordering (0=MSB First, 1=LSB first)
  0   CLKLOW   CLK Pin Output (0=8.4672MHz, 1=Fixed Low)
```

Configures how the drive is wired up. The SNES CD doesn't touch this register
and leaves it at it's power-up default. The Decoder should be disabled before
changing the register.

**X2h.W - CHPCTL - Chip Control (W)**

```
  7-5 -        Reserved (should be 0)
  4   CHPRST   Chip Reset (takes 500ns)   (0=No change, 1=Reset the chip)
  3   CD-DA    CD-Digital Audio Mode      (0=Data/CDROM, 1=Audio/CD-DA)
  2   SWOPN    Sync Detection Window      (0=Only if Sync expected, 1=Anytime)
  1   RPSTART  Repeat Correction Start  (0=No change, 1=Repeat if repeat mode)
  0   ADPEN    ADPCM Decode (to be set max 11.5ms after DECINT) (0=No, 1=Yes)
```

**X3h.W - DECCTL - Decoder Control (W)**

```
  7   AUTOCI    ADPCM Coding Information (0=Use CI Register, 1=Disc Subheader)
  6   -         Reserved (should be 0)
  5   MODESEL   Mode Select (when AUTODIST=0)               (0=MODE1, 1=MODE2)
  4   FORMSEL   Form Select (when AUTODIST=0 and MODESEL=1) (0=FORM1, 1=FORM2)
  3   AUTODIST  Auto Distinction        (0=Use MODESEL/FORMSEL, 1=Disc Header)
  2-0 DECMD2-0  Decoder Mode            (00h-07h, see below)
```

Decoder Mode values:

```
  00h/01h = Decoder disable (to be used for CD-DA Audio mode & during config)
  02h/03h = Monitor only    (read Header/Subheader, but don't write SRAM?)
  04h     = Write only mode (write sectors to SRAM without error correction?)
  05h     = Real time correction (abort correction if it takes too long?)
  06h     = Repeat correction (allow resume via RPSTART for important sectors?)
  07h     = Inhibit (reserved)
```

**X6h.W - CI - ADPCM Coding Information (to be used when AUTOCI=0) (W)**

```
  7   -        Reserved (should be 0)
  6   EMPHASIS ADPCM Emphasis           (0=Normal/Off, 1=Emphasis)
  5   -        Reserved (should be 0)
  4   BITL4H8  ADPCM Bit Length         (0=Normal/4bit, 1=8bit)
  3   -        Reserved (should be 0)
  2   FSL3H1   ADPCM Sampling Frequency (0=37800Hz, 1=18900Hz)
  1   -        Reserved (should be 0)
  0   MONOSTE  ADPCM Mono/Stereo        (0=Mono, 1=Stereo)
```

This register is used only when AUTOCI=0, allowing to use the correct ADPCM
format even in case of read errors on the CI byte in sector sub header (if
AUTOCI=1, such errors would trigger CIERR interrupt and omit playback of the
ADPCM sector with bad CI byte).

**0Dh.W - "PLBA" - Unknown  <-- shown as so in SNES CD's "CXD1800" test screen**

```
  7-0  PLBA?    ;Maybe PLBA means "PLayBAck" or even "PLayBAckwards" or so?
```

```
 _________________________ Interrupt / Status _________________________
```

**01h.R - INTSTS - Interrupt Status (0=No IRQ, 1=IRQ) (R)**

**X4h.W - INTMSK - Interrupt Mask (0=Disable, 1=Enable) (W)**

**X5h.W - INTCLR - Interrupt Clear/Ack (0=No change, 1=Clear/ack) (W)**

```
  7   ADPEND  ADPCM sector decode completed, and ADPCM disabled for next sector
  6   DECTOUT Decoder Time Out (no Sync within 3 sectors)
                Can occurs (only?) after the DECODER has been set to
                monitor only mode, or real time correction mode.
  5   DMACMP  DMA Complete (by DMAXFRC=0)                       (0=No, 1=Yes)
  4   DECINT  Decoder Interrupt (new "current sector" arrived)  (0=No, 1=Yes)
                If a SYNC mark is detected or internally inserted during
                execution of the write only, monitor only and real time
                correction modes by the DECODER, the DECINT status is created.
                  When the SYNC mark detected window is open, however, if the
                SYNC mark spacing is less than 2352 bytes, the DECINT status
                is not created.
                  During execution of the repeat correction mode by the DECODER,
                the DECINT status is created each time a correction ends.
  3   CIERR   Coding Info Error  (0=Okay, 1=Bad CI in ADPCM sector & AUTOCI=1)
  2-0 -       Reserved (should be 0)
```

**DECINT Handling (new "current sector" successfully/unsuccessfully received)**

First check the error flags in STS and HDRFLG registers (if desired, also check
MDFM and ADPCI to see how the decoder interpreted the sector).

Then check the MM:SS:FF values in HDR_xxx registers and ignore the sector if
the values aren't matching up with the desired values (that may happen if the
mechacon settled on sector number slightly lower than the requested seek
address, it might also happen during seek-busy phase, and it might happen if a
sector was skipped for some reason, which would require to issue a new seek
command and to retry reading the skipped sector).

When using ADPCM playback, also check SHDR_xxx registers to see if the sector
contains ADPCM data, and if it's having the desired file/channel numbers, if
so, set the ADPEN bit in CHPCTL.

Otherwise, if the sector is desired to be loaded to SNES memory: Handle the
CMADR either immediately, or if that isn't possible, memorize it in a queue,
and handle it as soon as possible, ie. after processing older queue entries,
but before the Sector Buffer location gets overwritten by newer sectors; the
32K SRAM can probably hold at least 8 sectors (8 x 924h bytes, plus some unused
padding areas, possibly plus some ADPCM area; as so on PSX).

As for handling CMADR: Usually one would only read the 800h-byte data portion
(without Header and Subheader), done by writing CMDADR+4 (for MODE1) or
CMDADR+0Ch (for MODE2) to DMAADRC, then writing 8800h to DMAXFRC, and then
reading 800h bytes from port 21E2h (usually via a SNES DMA channel).

**02h.R - STS - Status (R)**

```
  7   DRQ     Data Request (DRQ Pin)                            (0=?, 1=?)
  6   ADPBSY  ADPCM Playback Busy                               (0=No, 1=Busy)
  5   ERINBLK Erasure in Block; C2 flg anywhere except Syncmark (0=Okay, 1=Bad)
  4   CORINH  Correction Inhibit; MODE/FORM error & AUTODIST=1  (0=Okay, 1=Bad)
  3   EDCOK   EDC Error Detect Checksum (optional for FORM2)    (0=Bad, 1=Okay)
  2   ECCOK   ECC Error Correction Codes (not for FORM2)        (0=Bad, 1=Okay)
  1   SHRTSCT Sync Mark too early, no ECC/EDC done              (0=Okay, 1=Bad)
  0   NOSYNC  Sync Mark too late/missing, unreal SYNC inserted  (0=Okay, 1=Bad)
```

**03h.R - HDRFLG - Header C2-Error Flags (R)**

```
  7  MIN     Header MM   (0=Okay, 1=Error) ;\
  6  SEC     Header SS   (0=Okay, 1=Error) ; Header from MODE1/MODE2 data
  5  BLOCK   Header FF   (0=Okay, 1=Error) ; sector (ie. not for audio)
  4  MODE    Header MODE (0=Okay, 1=Error) ;/
  3  FILE    Sub-Header  (0=Okay, 1=Error) ;\Subheader exists for MODE2 only
  2  CHANNEL Sub-Header  (0=Okay, 1=Error) ; (the SNES CD BIOS wants these
  1  SUBMODE Sub-Header  (0=Okay, 1=Error) ; bits to be zero for MODE1, too)
  0  CI      Sub-Header  (0=Okay, 1=Error) ;/
```

**X4h.R - HDR_MIN - Header "MM" Minute (R)**

**X5h.R - HDR_SEC - Header "SS" Second (R)**

**X6h.R - HDR_BLOCK - Header "FF" Frame (R)**

**X7h.R - HDR_MODE - Header Mode (R)**

**08h.R - SHDR_FILE - Sub-Header File (R)**

**09h.R - SHDR_CH - Sub-Header Channel (R)**

**0Ah.R - SHDR_S-MODE - Sub-Header SubMode (R)**

**0Bh.R - SHDR_CI - Sub-Header Coding Info (R)**

Contains current sector's 4-byte Header (and 4-byte Subheader for MODE2 discs).

**0Ch/0Dh.R - CMADR_L/H - Current Minute Address, Low/High (R)**

```
  15    Unused
  14-0  Pointer to 1st byte of current sector (ie. to MM:SS:FF:MODE header)
```

Note: "Minute" is meaning the "1st byte of the sector". Named so because the
1st byte the "MM" value from the "MM:SS:FF:MODE" header. The sector stored in
SRAM is 924h bytes in size (ie. the whole 930h-byte sector, excluding the 12
Sync bytes).

**XEh.R - MDFM - MODE/FORM (R)**

```
  7-5 X        Unused
  4   RMODE2   Raw MODE byte, Bit2-7 ("logic sum") (aka all six bits ORed?)
                  Indicates the logic sum of the value of the high-order 6 bits
                  of the raw MODE byte AND THE POINTER (whut pointer?).
  3   RMODE1   Raw MODE byte, Bit1
  2   RMODE0   Raw MODE byte, Bit0
  1   CMODE    Correction Mode (0=MODE1, 1=MODE2)
  0   CFORM    Correction Form (0=FORM1, 1=FORM2) (for MODE2 only)
```

These bits indicate which of the MODEs and FORMs this IC determined that the
current sector was associated with when it corrected errors.

**XFh.R - ADPCI - ADPCM Coding Information (R)**

```
  7   MUTE     DA data is muted on      (0=No, 1=Muted)      <--- from where?
  6   EMPHASIS ADPCM Emphasis           (0=Normal/Off, 1=Emphasis)
  5   EOR      End of Record                         <--- (from SubMode.Bit0)
  4   BITLNGTH ADPCM Bit Length         (0=Normal/4bit, 1=8bit)
  3   X        Unused
  2   FS       ADPCM Sampling Frequency (0=37800Hz, 1=18900Hz)
  1   X        Unused
  0   M/S      ADPCM Mono/Stereo        (0=Mono, 1=Stereo)
```

Bit5 gets 1 when the SubMode.bit0=1 and there is no error in the SubMode byte.

```
 _________________________ DMA / Sector Buffer _________________________
```

**00h.R - DMADATA - SRAM-to-CPU Xfer Data (R)**

```
  7-0    Data from Sector buffer at [DMAADRC]
```

Reading increments DMAADRC and decrements DMAXFRC. However, for this special
case, REGADR is NOT incremented (allowing to read DMADATA continously without
needing to reset REGADR).

**X7h/X8h.W - DMAADRC_L/H - SRAM-to-CPU Xfer Address, Low/High (W)**

**1Ah/1Bh.R - DMAADRC_L/H - SRAM-to-CPU Xfer Address, Low/High (R)**

```
  15     Unused
  14-0   Current Read address for SRAM-to-CPU transfer (incrementing)
```

**X9h/XAh.W - DMAXFRC_L/H - SRAM-to-CPU Xfer Length & DMA Control, Low/High (W)**

**18h/19h.R - DMAXFRC_L/H - SRAM-to-CPU Xfer Length, Low/High (R)**

For writing X9h/XAh (with DMAEN bit inserted between other bits):

```
  15-12 DMAXFRC11-8 Transfer Length Remain Counter DMAXFRC, bit11-8
  11    DMAEN       CPU DMA Enable (0=Inhibit, 1=Enable)
  10-8  -           Reserved (should be 0)
  7-0   DMAXFRC7-0  Transfer Length Remain Counter DMAXFRC, bit7-0
```

For reading 18h/19h (without DMAEN bit, but instead with 15bit counter range):

```
  15    Unused      Unused
  14-0  DMAXFRC14-0 Transfer Length Remain Counter DMAXFRC, bit14-0
```

Setting DMAEN=1 does automatically set REGADR=00h (ie. select the DMADATA
register). DMAEN=1 should be used whenever starting a transfer (not matter if
the data is transferred via DMA, or if it's manually polled from DMADATA
register).

The DMACMP IRQ will occur when DMAXFRX reaches zero (to avoid that effect, one
may write DMAXFRC=0800h (DMAEN=1 and counter=000h); that will reportedly
prevent the IRQ; either because the counter doesn't decrease beyond zero, or
maybe it wraps to 7FFFh and thus won't expire anytime soon).

**XBh/XCh.W - DRVADRC_L/H - Disc-to-SRAM Xfer Address, Low/High (W)**

**1Ch/1Dh.R - DRVADRC_L/H - Disc-to-SRAM Xfer Address, Low/High (R)**

```
  15     Unused
  14-0   Disc-to-SRAM Xfer Address (incrementing)
```

This register is automatically advanced when storing incoming disc data in
Sector Buffer. The SNES CD BIOS doesn't touch this register at all.

Note: The datasheet has some obscure notes about needing to write the register
before "write only mode and real time correction mode" (unknown how/why/when to
do that).

### SNES Cart CDROM - Component List

"based on the on photos that have been posted, the main board has the same
parts as a Super Famicom, but with 7 additional chips:

```
  1) CXD2500 CD-DSP
  2) CXD1800 CD-ROM decoder/interface
  3) 32K SRAM (presuambly the CD-ROM sector buffer)
  4) some 20 pin SOP device that looks like a bus buffer
  5) a QFP with no markings (mechacon MCU?)
  6) A Sanyo 16 bit stereo DAC
  7) an 8 pin SOP - probably a dual opamp (it's next to the DAC outputs,
     so probably a buffer)
  The top board has a 4-bit MCU and a liquid crystal display.
```

There are also 5 visible ICs on the back of the CD-ROM control board - one of
them is a Rohm BTL driver another looks like a Sony CXA1272 (old CD drive focus
/ tracking servo) - the other chips are small SOP devices with numbers I can't
read. No sign of an RF amp chip, but on a lot of those older drives it was
built into the optical pickup. Basically, it has all the chips you would expect
for a basic data/audio CD drive of that vintage and nothing else."

**Sony Playstation SFX-100 Console Component List**

```
 Mainboard (MA-115, 0-396-987-04)
  IC101 100pin Nintendo S-CPU, 5A22-01  (65816 CPU with joypad I/O ports)
  IC102 100pin Nintendo S-PPU1, 5C77-01 (Video Chip 1)
  IC103 100pin Nintendo S-PPU2, 5C78-01 (Video Chip 2)
  IC104 28pin  NEC uPD43256A6U-10L? (32Kx8 SRAM, Video RAM 1)
  IC105 28pin  NEC uPD43256A6U-10L? (32Kx8 SRAM, Video RAM 2)
  IC106        ... whatever, maybe S-ENC or similar (Video RGB to composite)
  IC107 64pin  Nintendo S-WRAM (128Kx8 DRAM with B-bus)
  IC108 28pin  65256BLFP-12T   (32Kx8 SRAM, Sound RAM 1)
  IC109 18pin  Nintendo F411   (NTSC CIC)
  IC110 28pin  65256BLFP-12T   (32Kx8 SRAM, Sound RAM 2)
  IC111 64pin  SONY CXP1100Q-1 (APU, some newer S-SMP revision, SPC700 CPU)
  IC112 80pin  SONY CXD1222Q-1 (APU, some newer S-DSP revision, Sound Chip)
  IC113 20pin  LC78815M        (Two-channel 16bit D/A converter 1)
  IC201 80pin  SONY CXD2500AQ  (CDROM Signal Processor)
  IC202 20pin  LC78815M        (Two-channel 16bit D/A converter 2)
  IC203 48pin  Noname   ...  maybe Servo Amplifier (like CXA1782BR on PSX?)
  IC204 80pin  SONY CXD1800Q    (CDROM Decoder/FIFO, equivalent to CXD1196AR)
  IC205 18pin  74xxxx? (PCB has 20pin solderpoints, but chip is only 18pin)
  IC206 28pin  SONY CXK58257AM-70L (32Kx8 SRAM, CDROM Sector Buffer)
  IC301 8pin   Texas Instruments RC4558, "R4558 TI 25" (Dual Op-Amp 1)
  IC302 ...    ... whatever, maybe one of the 8pin IC???'s
  IC303 8pin   Texas Instruments RC4558, "R4558 TI 25" (Dual Op-Amp 2)
  ICxxx ...    if any... ?
  IC??? 8pin   whatever (front board edge, near headphone socket)
  IC??? 8pin   whatever (front board edge, near headphone socket)
  IC??? 24pin  whatever (front/mid board edge) (probably S-ENC or so)
  IC??? 3pin   voltage regulator (7805 or similar)
  IC??? ??     address decoder for I/O ports, 21E4h latch, NEXT port...?
               (maybe IC203 is doing that? but then where's Servo Amplifier?)
  CN201 29pin  To LCD Board
  CN..  ..     To CDROM Drive
  CN..  ..     To Controller Ports
  CN..  62pin  SNES Cartridge Slot
  CN..  ..     Rear panel
 Daughterboard with LCD
  IC701  80pin NEC uPD75P308GF  (CDROM Mechacon?)
  IC7xx ...    if any... ?
  X701         oscillator
  CN701  28pin LCD and six Buttons    (28pin, or maybe 2x28 pins?)
  CN702   4pin to somewhere  (2 LEDs ?, left of drive tray)
  CN703  29pin to Mainboard           (29pin, or maybe 2x29 pins?)
  CN704   3pin to front panel (disc eject button?, right of drive tray)
  ICxxx ...    if any... ?
  N/A?   28pin Something like BA6297AFP,BA6398FP,BA6397FP,AN8732SB,etc ?
 Daughterboard with controller ports
  ???   ...    whatever
 Daughterboard with Eject button
  ???   ...    whatever, a button, and maybe more stuff for the 3pin wire
 Daughterboard with LEDs
  ???   ...    whatever, two LEDs, and maybe more stuff for the 4pin wire
 Components in actual CD Drive unit
  ???   ...    whatever
 External Connectors
  1x snes cartridge slot (top)
  2x controller ports (front)
  1x 3.5mm headphone socket with "voltage level" regulator (front)
  1x "NEXT" port (serial link like PSX maybe?)
  1x Audio R        (red) (apparently with mono-switch when not connected)
  1x Audio L (MONO) (white)
  1x Video          (yellow)
  1x S VIDEO
  1x RF DC OUT
  1x MULTI OUT
  1x DC IN 7.6V
 Note: Some other/similar model has three RCA jacks instead headphone on front
```

**BIOS Cartridge - Case Sticker "'92.10.6." (plus some japanese symbols)**

```
  PCB "RB-01, K-PE1-945-01"
  IC1  64pin  Nintendo S-WRAM (128Kx8 DRAM)
  IC2  64pin  Nintendo S-WRAM (128Kx8 DRAM)
  IC3  32pin  HN2xxxxxx? (Sticker 0.95 SX) (ROM/EPROM)
  IC4  28pin  SONY CXK5864BM-12LL (8Kx8 SRAM)
  IC5  16pin  Noname?
  IC6  14pin  74F32 (Quad 2-input OR gates)
  IC7  16pin  74F138? (1-of-8 inverting decoder/demultiplexer?)
  IC8  14pin  Noname?
  IC9  14pin  Noname?
  IC10 16pin  Noname?
  IC11 16pin  Nintendo D411 (NTSC CIC)
  ?    ?      white space (in upper left)
  ?    2pin   something with 2 pins is apparently on PCB back side (battery?)
```

**LCD/Button Panel**

```
           PlayStation
                       SFX-100
  .---------------------------.
  |    TRACK  STEP/MIN SEC    |
  |  .---------------------.  |
  |  |        (LCD)        |  |
  |  '---------------------'  |
  |   PLAY MODE     REMAIN    |
  |  =========== ===========  |
  |      |<<         >>|      |
  |  =========== ===========  |
  |     |> ||         []      |
  |  =========== ===========  |
  '---------------------------'
```

Note: The date codes on the three S-WRAM's, D411, and uPD75P308GF seem to be
from 1991. Sticker on case of BIOS cart seems to be from 1992.
