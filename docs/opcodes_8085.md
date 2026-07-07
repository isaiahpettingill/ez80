# Intel 8085 Opcodes

Generated from the emulator disassembler for `CpuMode::I8085`.
The tables list every opcode form decoded by this mode. `NONINOP` is the emulator's non-instruction NOP behavior for unsupported ED-prefixed Z80-family opcodes.

## Base opcodes

| Bytes | Disassembly |
| --- | --- |
| `00` | `NOP` |
| `01 34 12` | `LD BC, $1234` |
| `02` | `LD (BC), A` |
| `03` | `INC BC` |
| `04` | `INC B` |
| `05` | `DEC B` |
| `06 34` | `LD B, $34` |
| `07` | `RLCA` |
| `08` | `NOP` |
| `09` | `ADD HL, BC` |
| `0a` | `LD A, (BC)` |
| `0b` | `DEC BC` |
| `0c` | `INC C` |
| `0d` | `DEC C` |
| `0e 34` | `LD C, $34` |
| `0f` | `RRCA` |
| `10` | `NOP` |
| `11 34 12` | `LD DE, $1234` |
| `12` | `LD (DE), A` |
| `13` | `INC DE` |
| `14` | `INC D` |
| `15` | `DEC D` |
| `16 34` | `LD D, $34` |
| `17` | `RLA` |
| `18` | `NOP` |
| `19` | `ADD HL, DE` |
| `1a` | `LD A, (DE)` |
| `1b` | `DEC DE` |
| `1c` | `INC E` |
| `1d` | `DEC E` |
| `1e 34` | `LD E, $34` |
| `1f` | `RRA` |
| `20` | `RIM` |
| `21 34 12` | `LD HL, $1234` |
| `22 34 12` | `LD ($1234), HL` |
| `23` | `INC HL` |
| `24` | `INC H` |
| `25` | `DEC H` |
| `26 34` | `LD H, $34` |
| `27` | `DAA` |
| `28` | `NOP` |
| `29` | `ADD HL, HL` |
| `2a 34 12` | `LD HL, ($1234)` |
| `2b` | `DEC HL` |
| `2c` | `INC L` |
| `2d` | `DEC L` |
| `2e 34` | `LD L, $34` |
| `2f` | `CPL` |
| `30` | `SIM` |
| `31 34 12` | `LD SP, $1234` |
| `32 34 12` | `LD ($1234), A` |
| `33` | `INC SP` |
| `34` | `INC (HL)` |
| `35` | `DEC (HL)` |
| `36 34` | `LD (HL), $34` |
| `37` | `SCF` |
| `38` | `NOP` |
| `39` | `ADD HL, SP` |
| `3a 34 12` | `LD A, ($1234)` |
| `3b` | `DEC SP` |
| `3c` | `INC A` |
| `3d` | `DEC A` |
| `3e 34` | `LD A, $34` |
| `3f` | `CCF` |
| `40` | `LD B, B` |
| `41` | `LD B, C` |
| `42` | `LD B, D` |
| `43` | `LD B, E` |
| `44` | `LD B, H` |
| `45` | `LD B, L` |
| `46` | `LD B, (HL)` |
| `47` | `LD B, A` |
| `48` | `LD C, B` |
| `49` | `LD C, C` |
| `4a` | `LD C, D` |
| `4b` | `LD C, E` |
| `4c` | `LD C, H` |
| `4d` | `LD C, L` |
| `4e` | `LD C, (HL)` |
| `4f` | `LD C, A` |
| `50` | `LD D, B` |
| `51` | `LD D, C` |
| `52` | `LD D, D` |
| `53` | `LD D, E` |
| `54` | `LD D, H` |
| `55` | `LD D, L` |
| `56` | `LD D, (HL)` |
| `57` | `LD D, A` |
| `58` | `LD E, B` |
| `59` | `LD E, C` |
| `5a` | `LD E, D` |
| `5b` | `LD E, E` |
| `5c` | `LD E, H` |
| `5d` | `LD E, L` |
| `5e` | `LD E, (HL)` |
| `5f` | `LD E, A` |
| `60` | `LD H, B` |
| `61` | `LD H, C` |
| `62` | `LD H, D` |
| `63` | `LD H, E` |
| `64` | `LD H, H` |
| `65` | `LD H, L` |
| `66` | `LD H, (HL)` |
| `67` | `LD H, A` |
| `68` | `LD L, B` |
| `69` | `LD L, C` |
| `6a` | `LD L, D` |
| `6b` | `LD L, E` |
| `6c` | `LD L, H` |
| `6d` | `LD L, L` |
| `6e` | `LD L, (HL)` |
| `6f` | `LD L, A` |
| `70` | `LD (HL), B` |
| `71` | `LD (HL), C` |
| `72` | `LD (HL), D` |
| `73` | `LD (HL), E` |
| `74` | `LD (HL), H` |
| `75` | `LD (HL), L` |
| `76` | `HALT` |
| `77` | `LD (HL), A` |
| `78` | `LD A, B` |
| `79` | `LD A, C` |
| `7a` | `LD A, D` |
| `7b` | `LD A, E` |
| `7c` | `LD A, H` |
| `7d` | `LD A, L` |
| `7e` | `LD A, (HL)` |
| `7f` | `LD A, A` |
| `80` | `ADD A, B` |
| `81` | `ADD A, C` |
| `82` | `ADD A, D` |
| `83` | `ADD A, E` |
| `84` | `ADD A, H` |
| `85` | `ADD A, L` |
| `86` | `ADD A, (HL)` |
| `87` | `ADD A, A` |
| `88` | `ADC A, B` |
| `89` | `ADC A, C` |
| `8a` | `ADC A, D` |
| `8b` | `ADC A, E` |
| `8c` | `ADC A, H` |
| `8d` | `ADC A, L` |
| `8e` | `ADC A, (HL)` |
| `8f` | `ADC A, A` |
| `90` | `SUB A, B` |
| `91` | `SUB A, C` |
| `92` | `SUB A, D` |
| `93` | `SUB A, E` |
| `94` | `SUB A, H` |
| `95` | `SUB A, L` |
| `96` | `SUB A, (HL)` |
| `97` | `SUB A, A` |
| `98` | `SBC A, B` |
| `99` | `SBC A, C` |
| `9a` | `SBC A, D` |
| `9b` | `SBC A, E` |
| `9c` | `SBC A, H` |
| `9d` | `SBC A, L` |
| `9e` | `SBC A, (HL)` |
| `9f` | `SBC A, A` |
| `a0` | `AND A, B` |
| `a1` | `AND A, C` |
| `a2` | `AND A, D` |
| `a3` | `AND A, E` |
| `a4` | `AND A, H` |
| `a5` | `AND A, L` |
| `a6` | `AND A, (HL)` |
| `a7` | `AND A, A` |
| `a8` | `XOR A, B` |
| `a9` | `XOR A, C` |
| `aa` | `XOR A, D` |
| `ab` | `XOR A, E` |
| `ac` | `XOR A, H` |
| `ad` | `XOR A, L` |
| `ae` | `XOR A, (HL)` |
| `af` | `XOR A, A` |
| `b0` | `OR A, B` |
| `b1` | `OR A, C` |
| `b2` | `OR A, D` |
| `b3` | `OR A, E` |
| `b4` | `OR A, H` |
| `b5` | `OR A, L` |
| `b6` | `OR A, (HL)` |
| `b7` | `OR A, A` |
| `b8` | `CP A, B` |
| `b9` | `CP A, C` |
| `ba` | `CP A, D` |
| `bb` | `CP A, E` |
| `bc` | `CP A, H` |
| `bd` | `CP A, L` |
| `be` | `CP A, (HL)` |
| `bf` | `CP A, A` |
| `c0` | `RET NZ` |
| `c1` | `POP BC` |
| `c2 34 12` | `JP NZ, $1234` |
| `c3 34 12` | `JP $1234` |
| `c4 34 12` | `CALL NZ, $1234` |
| `c5` | `PUSH BC` |
| `c6 34` | `ADD A, $34` |
| `c7` | `RST 00h` |
| `c8` | `RET Z` |
| `c9` | `RET` |
| `ca 34 12` | `JP Z, $1234` |
| `cb 34 12` | `JP $1234` |
| `cc 34 12` | `CALL Z, $1234` |
| `cd 34 12` | `CALL $1234` |
| `ce 34` | `ADC A, $34` |
| `cf` | `RST 08h` |
| `d0` | `RET NC` |
| `d1` | `POP DE` |
| `d2 34 12` | `JP NC, $1234` |
| `d3 34` | `OUT ($34), A` |
| `d4 34 12` | `CALL NC, $1234` |
| `d5` | `PUSH DE` |
| `d6 34` | `SUB A, $34` |
| `d7` | `RST 10h` |
| `d8` | `RET C` |
| `d9` | `RET` |
| `da 34 12` | `JP C, $1234` |
| `db 34` | `IN A, ($34)` |
| `dc 34 12` | `CALL C, $1234` |
| `dd 34 12` | `CALL $1234` |
| `de 34` | `SBC A, $34` |
| `df` | `RST 18h` |
| `e0` | `RET PO` |
| `e1` | `POP HL` |
| `e2 34 12` | `JP PO, $1234` |
| `e3` | `EX (SP), HL` |
| `e4 34 12` | `CALL PO, $1234` |
| `e5` | `PUSH HL` |
| `e6 34` | `AND A, $34` |
| `e7` | `RST 20h` |
| `e8` | `RET PE` |
| `e9` | `JP (HL)` |
| `ea 34 12` | `JP PE, $1234` |
| `eb` | `EX DE, HL` |
| `ec 34 12` | `CALL PE, $1234` |
| `ed 34 12` | `CALL $1234` |
| `ee 34` | `XOR A, $34` |
| `ef` | `RST 28h` |
| `f0` | `RET P` |
| `f1` | `POP AF` |
| `f2 34 12` | `JP P, $1234` |
| `f3` | `DI` |
| `f4 34 12` | `CALL P, $1234` |
| `f5` | `PUSH AF` |
| `f6 34` | `OR A, $34` |
| `f7` | `RST 30h` |
| `f8` | `RET N` |
| `f9` | `LD SP, HL` |
| `fa 34 12` | `JP N, $1234` |
| `fb` | `EI` |
| `fc 34 12` | `CALL N, $1234` |
| `fd 34 12` | `CALL $1234` |
| `fe 34` | `CP A, $34` |
| `ff` | `RST 38h` |

