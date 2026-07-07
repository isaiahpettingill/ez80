# ZX Spectrum Next Z80N Opcodes

Generated from the emulator disassembler for `CpuMode::Z80N`.
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
| `08` | `EX AF, AF'` |
| `09` | `ADD HL, BC` |
| `0a` | `LD A, (BC)` |
| `0b` | `DEC BC` |
| `0c` | `INC C` |
| `0d` | `DEC C` |
| `0e 34` | `LD C, $34` |
| `0f` | `RRCA` |
| `10 34` | `DJNZ $36` |
| `11 34 12` | `LD DE, $1234` |
| `12` | `LD (DE), A` |
| `13` | `INC DE` |
| `14` | `INC D` |
| `15` | `DEC D` |
| `16 34` | `LD D, $34` |
| `17` | `RLA` |
| `18 34` | `JR $36` |
| `19` | `ADD HL, DE` |
| `1a` | `LD A, (DE)` |
| `1b` | `DEC DE` |
| `1c` | `INC E` |
| `1d` | `DEC E` |
| `1e 34` | `LD E, $34` |
| `1f` | `RRA` |
| `20 34` | `JR NZ, $36` |
| `21 34 12` | `LD HL, $1234` |
| `22 34 12` | `LD ($1234), HL` |
| `23` | `INC HL` |
| `24` | `INC H` |
| `25` | `DEC H` |
| `26 34` | `LD H, $34` |
| `27` | `DAA` |
| `28 34` | `JR Z, $36` |
| `29` | `ADD HL, HL` |
| `2a 34 12` | `LD HL, ($1234)` |
| `2b` | `DEC HL` |
| `2c` | `INC L` |
| `2d` | `DEC L` |
| `2e 34` | `LD L, $34` |
| `2f` | `CPL` |
| `30 34` | `JR NC, $36` |
| `31 34 12` | `LD SP, $1234` |
| `32 34 12` | `LD ($1234), A` |
| `33` | `INC SP` |
| `34` | `INC (HL)` |
| `35` | `DEC (HL)` |
| `36 34` | `LD (HL), $34` |
| `37` | `SCF` |
| `38 34` | `JR C, $36` |
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
| `cb 34` | `SLL H` |
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
| `d9` | `EXX` |
| `da 34 12` | `JP C, $1234` |
| `db 34` | `IN A, ($34)` |
| `dc 34 12` | `CALL C, $1234` |
| `dd 34 12` | `INC (IX+18)` |
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
| `ed 34 12 56` | `ADD HL, $5612` |
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
| `fd 34 12` | `INC (IY+18)` |
| `fe 34` | `CP A, $34` |
| `ff` | `RST 38h` |

## CB-prefixed opcodes

| Bytes | Disassembly |
| --- | --- |
| `cb 00` | `RLC B` |
| `cb 01` | `RLC C` |
| `cb 02` | `RLC D` |
| `cb 03` | `RLC E` |
| `cb 04` | `RLC H` |
| `cb 05` | `RLC L` |
| `cb 06` | `RLC (HL)` |
| `cb 07` | `RLC A` |
| `cb 08` | `RRC B` |
| `cb 09` | `RRC C` |
| `cb 0a` | `RRC D` |
| `cb 0b` | `RRC E` |
| `cb 0c` | `RRC H` |
| `cb 0d` | `RRC L` |
| `cb 0e` | `RRC (HL)` |
| `cb 0f` | `RRC A` |
| `cb 10` | `RL B` |
| `cb 11` | `RL C` |
| `cb 12` | `RL D` |
| `cb 13` | `RL E` |
| `cb 14` | `RL H` |
| `cb 15` | `RL L` |
| `cb 16` | `RL (HL)` |
| `cb 17` | `RL A` |
| `cb 18` | `RR B` |
| `cb 19` | `RR C` |
| `cb 1a` | `RR D` |
| `cb 1b` | `RR E` |
| `cb 1c` | `RR H` |
| `cb 1d` | `RR L` |
| `cb 1e` | `RR (HL)` |
| `cb 1f` | `RR A` |
| `cb 20` | `SLA B` |
| `cb 21` | `SLA C` |
| `cb 22` | `SLA D` |
| `cb 23` | `SLA E` |
| `cb 24` | `SLA H` |
| `cb 25` | `SLA L` |
| `cb 26` | `SLA (HL)` |
| `cb 27` | `SLA A` |
| `cb 28` | `SRA B` |
| `cb 29` | `SRA C` |
| `cb 2a` | `SRA D` |
| `cb 2b` | `SRA E` |
| `cb 2c` | `SRA H` |
| `cb 2d` | `SRA L` |
| `cb 2e` | `SRA (HL)` |
| `cb 2f` | `SRA A` |
| `cb 30` | `SLL B` |
| `cb 31` | `SLL C` |
| `cb 32` | `SLL D` |
| `cb 33` | `SLL E` |
| `cb 34` | `SLL H` |
| `cb 35` | `SLL L` |
| `cb 36` | `SLL (HL)` |
| `cb 37` | `SLL A` |
| `cb 38` | `SRL B` |
| `cb 39` | `SRL C` |
| `cb 3a` | `SRL D` |
| `cb 3b` | `SRL E` |
| `cb 3c` | `SRL H` |
| `cb 3d` | `SRL L` |
| `cb 3e` | `SRL (HL)` |
| `cb 3f` | `SRL A` |
| `cb 40` | `BIT 0, B` |
| `cb 41` | `BIT 0, C` |
| `cb 42` | `BIT 0, D` |
| `cb 43` | `BIT 0, E` |
| `cb 44` | `BIT 0, H` |
| `cb 45` | `BIT 0, L` |
| `cb 46` | `BIT 0, (HL)` |
| `cb 47` | `BIT 0, A` |
| `cb 48` | `BIT 1, B` |
| `cb 49` | `BIT 1, C` |
| `cb 4a` | `BIT 1, D` |
| `cb 4b` | `BIT 1, E` |
| `cb 4c` | `BIT 1, H` |
| `cb 4d` | `BIT 1, L` |
| `cb 4e` | `BIT 1, (HL)` |
| `cb 4f` | `BIT 1, A` |
| `cb 50` | `BIT 2, B` |
| `cb 51` | `BIT 2, C` |
| `cb 52` | `BIT 2, D` |
| `cb 53` | `BIT 2, E` |
| `cb 54` | `BIT 2, H` |
| `cb 55` | `BIT 2, L` |
| `cb 56` | `BIT 2, (HL)` |
| `cb 57` | `BIT 2, A` |
| `cb 58` | `BIT 3, B` |
| `cb 59` | `BIT 3, C` |
| `cb 5a` | `BIT 3, D` |
| `cb 5b` | `BIT 3, E` |
| `cb 5c` | `BIT 3, H` |
| `cb 5d` | `BIT 3, L` |
| `cb 5e` | `BIT 3, (HL)` |
| `cb 5f` | `BIT 3, A` |
| `cb 60` | `BIT 4, B` |
| `cb 61` | `BIT 4, C` |
| `cb 62` | `BIT 4, D` |
| `cb 63` | `BIT 4, E` |
| `cb 64` | `BIT 4, H` |
| `cb 65` | `BIT 4, L` |
| `cb 66` | `BIT 4, (HL)` |
| `cb 67` | `BIT 4, A` |
| `cb 68` | `BIT 5, B` |
| `cb 69` | `BIT 5, C` |
| `cb 6a` | `BIT 5, D` |
| `cb 6b` | `BIT 5, E` |
| `cb 6c` | `BIT 5, H` |
| `cb 6d` | `BIT 5, L` |
| `cb 6e` | `BIT 5, (HL)` |
| `cb 6f` | `BIT 5, A` |
| `cb 70` | `BIT 6, B` |
| `cb 71` | `BIT 6, C` |
| `cb 72` | `BIT 6, D` |
| `cb 73` | `BIT 6, E` |
| `cb 74` | `BIT 6, H` |
| `cb 75` | `BIT 6, L` |
| `cb 76` | `BIT 6, (HL)` |
| `cb 77` | `BIT 6, A` |
| `cb 78` | `BIT 7, B` |
| `cb 79` | `BIT 7, C` |
| `cb 7a` | `BIT 7, D` |
| `cb 7b` | `BIT 7, E` |
| `cb 7c` | `BIT 7, H` |
| `cb 7d` | `BIT 7, L` |
| `cb 7e` | `BIT 7, (HL)` |
| `cb 7f` | `BIT 7, A` |
| `cb 80` | `RES 0, B` |
| `cb 81` | `RES 0, C` |
| `cb 82` | `RES 0, D` |
| `cb 83` | `RES 0, E` |
| `cb 84` | `RES 0, H` |
| `cb 85` | `RES 0, L` |
| `cb 86` | `RES 0, (HL)` |
| `cb 87` | `RES 0, A` |
| `cb 88` | `RES 1, B` |
| `cb 89` | `RES 1, C` |
| `cb 8a` | `RES 1, D` |
| `cb 8b` | `RES 1, E` |
| `cb 8c` | `RES 1, H` |
| `cb 8d` | `RES 1, L` |
| `cb 8e` | `RES 1, (HL)` |
| `cb 8f` | `RES 1, A` |
| `cb 90` | `RES 2, B` |
| `cb 91` | `RES 2, C` |
| `cb 92` | `RES 2, D` |
| `cb 93` | `RES 2, E` |
| `cb 94` | `RES 2, H` |
| `cb 95` | `RES 2, L` |
| `cb 96` | `RES 2, (HL)` |
| `cb 97` | `RES 2, A` |
| `cb 98` | `RES 3, B` |
| `cb 99` | `RES 3, C` |
| `cb 9a` | `RES 3, D` |
| `cb 9b` | `RES 3, E` |
| `cb 9c` | `RES 3, H` |
| `cb 9d` | `RES 3, L` |
| `cb 9e` | `RES 3, (HL)` |
| `cb 9f` | `RES 3, A` |
| `cb a0` | `RES 4, B` |
| `cb a1` | `RES 4, C` |
| `cb a2` | `RES 4, D` |
| `cb a3` | `RES 4, E` |
| `cb a4` | `RES 4, H` |
| `cb a5` | `RES 4, L` |
| `cb a6` | `RES 4, (HL)` |
| `cb a7` | `RES 4, A` |
| `cb a8` | `RES 5, B` |
| `cb a9` | `RES 5, C` |
| `cb aa` | `RES 5, D` |
| `cb ab` | `RES 5, E` |
| `cb ac` | `RES 5, H` |
| `cb ad` | `RES 5, L` |
| `cb ae` | `RES 5, (HL)` |
| `cb af` | `RES 5, A` |
| `cb b0` | `RES 6, B` |
| `cb b1` | `RES 6, C` |
| `cb b2` | `RES 6, D` |
| `cb b3` | `RES 6, E` |
| `cb b4` | `RES 6, H` |
| `cb b5` | `RES 6, L` |
| `cb b6` | `RES 6, (HL)` |
| `cb b7` | `RES 6, A` |
| `cb b8` | `RES 7, B` |
| `cb b9` | `RES 7, C` |
| `cb ba` | `RES 7, D` |
| `cb bb` | `RES 7, E` |
| `cb bc` | `RES 7, H` |
| `cb bd` | `RES 7, L` |
| `cb be` | `RES 7, (HL)` |
| `cb bf` | `RES 7, A` |
| `cb c0` | `SET 0, B` |
| `cb c1` | `SET 0, C` |
| `cb c2` | `SET 0, D` |
| `cb c3` | `SET 0, E` |
| `cb c4` | `SET 0, H` |
| `cb c5` | `SET 0, L` |
| `cb c6` | `SET 0, (HL)` |
| `cb c7` | `SET 0, A` |
| `cb c8` | `SET 1, B` |
| `cb c9` | `SET 1, C` |
| `cb ca` | `SET 1, D` |
| `cb cb` | `SET 1, E` |
| `cb cc` | `SET 1, H` |
| `cb cd` | `SET 1, L` |
| `cb ce` | `SET 1, (HL)` |
| `cb cf` | `SET 1, A` |
| `cb d0` | `SET 2, B` |
| `cb d1` | `SET 2, C` |
| `cb d2` | `SET 2, D` |
| `cb d3` | `SET 2, E` |
| `cb d4` | `SET 2, H` |
| `cb d5` | `SET 2, L` |
| `cb d6` | `SET 2, (HL)` |
| `cb d7` | `SET 2, A` |
| `cb d8` | `SET 3, B` |
| `cb d9` | `SET 3, C` |
| `cb da` | `SET 3, D` |
| `cb db` | `SET 3, E` |
| `cb dc` | `SET 3, H` |
| `cb dd` | `SET 3, L` |
| `cb de` | `SET 3, (HL)` |
| `cb df` | `SET 3, A` |
| `cb e0` | `SET 4, B` |
| `cb e1` | `SET 4, C` |
| `cb e2` | `SET 4, D` |
| `cb e3` | `SET 4, E` |
| `cb e4` | `SET 4, H` |
| `cb e5` | `SET 4, L` |
| `cb e6` | `SET 4, (HL)` |
| `cb e7` | `SET 4, A` |
| `cb e8` | `SET 5, B` |
| `cb e9` | `SET 5, C` |
| `cb ea` | `SET 5, D` |
| `cb eb` | `SET 5, E` |
| `cb ec` | `SET 5, H` |
| `cb ed` | `SET 5, L` |
| `cb ee` | `SET 5, (HL)` |
| `cb ef` | `SET 5, A` |
| `cb f0` | `SET 6, B` |
| `cb f1` | `SET 6, C` |
| `cb f2` | `SET 6, D` |
| `cb f3` | `SET 6, E` |
| `cb f4` | `SET 6, H` |
| `cb f5` | `SET 6, L` |
| `cb f6` | `SET 6, (HL)` |
| `cb f7` | `SET 6, A` |
| `cb f8` | `SET 7, B` |
| `cb f9` | `SET 7, C` |
| `cb fa` | `SET 7, D` |
| `cb fb` | `SET 7, E` |
| `cb fc` | `SET 7, H` |
| `cb fd` | `SET 7, L` |
| `cb fe` | `SET 7, (HL)` |
| `cb ff` | `SET 7, A` |

## ED-prefixed opcodes

| Bytes | Disassembly |
| --- | --- |
| `ed 00` | `NONINOP` |
| `ed 01` | `NONINOP` |
| `ed 02` | `NONINOP` |
| `ed 03` | `NONINOP` |
| `ed 04` | `NONINOP` |
| `ed 05` | `NONINOP` |
| `ed 06` | `NONINOP` |
| `ed 07` | `NONINOP` |
| `ed 08` | `NONINOP` |
| `ed 09` | `NONINOP` |
| `ed 0a` | `NONINOP` |
| `ed 0b` | `NONINOP` |
| `ed 0c` | `NONINOP` |
| `ed 0d` | `NONINOP` |
| `ed 0e` | `NONINOP` |
| `ed 0f` | `NONINOP` |
| `ed 10` | `NONINOP` |
| `ed 11` | `NONINOP` |
| `ed 12` | `NONINOP` |
| `ed 13` | `NONINOP` |
| `ed 14` | `NONINOP` |
| `ed 15` | `NONINOP` |
| `ed 16` | `NONINOP` |
| `ed 17` | `NONINOP` |
| `ed 18` | `NONINOP` |
| `ed 19` | `NONINOP` |
| `ed 1a` | `NONINOP` |
| `ed 1b` | `NONINOP` |
| `ed 1c` | `NONINOP` |
| `ed 1d` | `NONINOP` |
| `ed 1e` | `NONINOP` |
| `ed 1f` | `NONINOP` |
| `ed 20` | `NONINOP` |
| `ed 21` | `NONINOP` |
| `ed 22` | `NONINOP` |
| `ed 23` | `SWAPNIB` |
| `ed 24` | `MIRROR A` |
| `ed 25` | `NONINOP` |
| `ed 26` | `NONINOP` |
| `ed 27 34` | `TST A, $34` |
| `ed 28` | `NONINOP` |
| `ed 29` | `NONINOP` |
| `ed 2a` | `NONINOP` |
| `ed 2b` | `NONINOP` |
| `ed 2c` | `NONINOP` |
| `ed 2d` | `NONINOP` |
| `ed 2e` | `NONINOP` |
| `ed 2f` | `NONINOP` |
| `ed 30` | `MUL D, E` |
| `ed 31` | `ADD HL, A` |
| `ed 32` | `ADD DE, A` |
| `ed 33` | `ADD BC, A` |
| `ed 34 34 12` | `ADD HL, $1234` |
| `ed 35 34 12` | `ADD DE, $1234` |
| `ed 36 34 12` | `ADD BC, $1234` |
| `ed 37` | `NONINOP` |
| `ed 38` | `NONINOP` |
| `ed 39` | `NONINOP` |
| `ed 3a` | `NONINOP` |
| `ed 3b` | `NONINOP` |
| `ed 3c` | `NONINOP` |
| `ed 3d` | `NONINOP` |
| `ed 3e` | `NONINOP` |
| `ed 3f` | `NONINOP` |
| `ed 40` | `IN B, (C)` |
| `ed 41` | `OUT (C), B` |
| `ed 42` | `SBC HL, BC` |
| `ed 43 34 12` | `LD ($1234), BC` |
| `ed 44` | `NEG` |
| `ed 45` | `RETN` |
| `ed 46` | `IM 0` |
| `ed 47` | `LD I, A` |
| `ed 48` | `IN C, (C)` |
| `ed 49` | `OUT (C), C` |
| `ed 4a` | `ADC HL, BC` |
| `ed 4b 34 12` | `LD BC, ($1234)` |
| `ed 4c` | `NEG` |
| `ed 4d` | `RETI` |
| `ed 4e` | `IM 0` |
| `ed 4f` | `LD R, A` |
| `ed 50` | `IN D, (C)` |
| `ed 51` | `OUT (C), D` |
| `ed 52` | `SBC HL, DE` |
| `ed 53 34 12` | `LD ($1234), DE` |
| `ed 54` | `NEG` |
| `ed 55` | `RETN` |
| `ed 56` | `IM 1` |
| `ed 57` | `LD A, I` |
| `ed 58` | `IN E, (C)` |
| `ed 59` | `OUT (C), E` |
| `ed 5a` | `ADC HL, DE` |
| `ed 5b 34 12` | `LD DE, ($1234)` |
| `ed 5c` | `NEG` |
| `ed 5d` | `RETN` |
| `ed 5e` | `IM 2` |
| `ed 5f` | `LD A, R` |
| `ed 60` | `IN H, (C)` |
| `ed 61` | `OUT (C), H` |
| `ed 62` | `SBC HL, HL` |
| `ed 63 34 12` | `LD ($1234), HL` |
| `ed 64` | `NEG` |
| `ed 65` | `RETN` |
| `ed 66` | `IM 0` |
| `ed 67` | `RRD` |
| `ed 68` | `IN L, (C)` |
| `ed 69` | `OUT (C), L` |
| `ed 6a` | `ADC HL, HL` |
| `ed 6b 34 12` | `LD HL, ($1234)` |
| `ed 6c` | `NEG` |
| `ed 6d` | `RETN` |
| `ed 6e` | `IM 0` |
| `ed 6f` | `RLD` |
| `ed 70` | `IN (C)` |
| `ed 71` | `OUT (C), 0` |
| `ed 72` | `SBC HL, SP` |
| `ed 73 34 12` | `LD ($1234), SP` |
| `ed 74` | `NEG` |
| `ed 75` | `RETN` |
| `ed 76` | `IM 1` |
| `ed 77` | `NOP` |
| `ed 78` | `IN A, (C)` |
| `ed 79` | `OUT (C), A` |
| `ed 7a` | `ADC HL, SP` |
| `ed 7b 34 12` | `LD SP, ($1234)` |
| `ed 7c` | `NEG` |
| `ed 7d` | `RETN` |
| `ed 7e` | `IM 2` |
| `ed 7f` | `NOP` |
| `ed 80` | `NONINOP` |
| `ed 81` | `NONINOP` |
| `ed 82` | `NONINOP` |
| `ed 83` | `NONINOP` |
| `ed 84` | `NONINOP` |
| `ed 85` | `NONINOP` |
| `ed 86` | `NONINOP` |
| `ed 87` | `NONINOP` |
| `ed 88` | `NONINOP` |
| `ed 89` | `NONINOP` |
| `ed 8a` | `NONINOP` |
| `ed 8b` | `NONINOP` |
| `ed 8c` | `NONINOP` |
| `ed 8d` | `NONINOP` |
| `ed 8e` | `NONINOP` |
| `ed 8f` | `NONINOP` |
| `ed 90` | `NONINOP` |
| `ed 91 34` | `NEXTREG $34, A` |
| `ed 92 34` | `NEXTREG $34, $34` |
| `ed 93` | `NONINOP` |
| `ed 94` | `NONINOP` |
| `ed 95` | `NONINOP` |
| `ed 96` | `NONINOP` |
| `ed 97` | `NONINOP` |
| `ed 98` | `NONINOP` |
| `ed 99` | `NONINOP` |
| `ed 9a` | `NONINOP` |
| `ed 9b` | `NONINOP` |
| `ed 9c` | `NONINOP` |
| `ed 9d` | `NONINOP` |
| `ed 9e` | `NONINOP` |
| `ed 9f` | `NONINOP` |
| `ed a0` | `LDI` |
| `ed a1` | `CPI` |
| `ed a2` | `INI` |
| `ed a3` | `OUTI` |
| `ed a4` | `NONINOP` |
| `ed a5` | `NONINOP` |
| `ed a6` | `NONINOP` |
| `ed a7` | `NONINOP` |
| `ed a8` | `LDD` |
| `ed a9` | `CPD` |
| `ed aa` | `IND` |
| `ed ab` | `OUTD` |
| `ed ac` | `NONINOP` |
| `ed ad` | `NONINOP` |
| `ed ae` | `NONINOP` |
| `ed af` | `NONINOP` |
| `ed b0` | `LDIR` |
| `ed b1` | `CPIR` |
| `ed b2` | `INIR` |
| `ed b3` | `OTIR` |
| `ed b4` | `NONINOP` |
| `ed b5` | `NONINOP` |
| `ed b6` | `NONINOP` |
| `ed b7` | `NONINOP` |
| `ed b8` | `LDDR` |
| `ed b9` | `CPDR` |
| `ed ba` | `INDR` |
| `ed bb` | `OTDR` |
| `ed bc` | `NONINOP` |
| `ed bd` | `NONINOP` |
| `ed be` | `NONINOP` |
| `ed bf` | `NONINOP` |
| `ed c0` | `NONINOP` |
| `ed c1` | `NONINOP` |
| `ed c2` | `NONINOP` |
| `ed c3` | `NONINOP` |
| `ed c4` | `NONINOP` |
| `ed c5` | `NONINOP` |
| `ed c6` | `NONINOP` |
| `ed c7` | `NONINOP` |
| `ed c8` | `NONINOP` |
| `ed c9` | `NONINOP` |
| `ed ca` | `NONINOP` |
| `ed cb` | `NONINOP` |
| `ed cc` | `NONINOP` |
| `ed cd` | `NONINOP` |
| `ed ce` | `NONINOP` |
| `ed cf` | `NONINOP` |
| `ed d0` | `NONINOP` |
| `ed d1` | `NONINOP` |
| `ed d2` | `NONINOP` |
| `ed d3` | `NONINOP` |
| `ed d4` | `NONINOP` |
| `ed d5` | `NONINOP` |
| `ed d6` | `NONINOP` |
| `ed d7` | `NONINOP` |
| `ed d8` | `NONINOP` |
| `ed d9` | `NONINOP` |
| `ed da` | `NONINOP` |
| `ed db` | `NONINOP` |
| `ed dc` | `NONINOP` |
| `ed dd` | `NONINOP` |
| `ed de` | `NONINOP` |
| `ed df` | `NONINOP` |
| `ed e0` | `NONINOP` |
| `ed e1` | `NONINOP` |
| `ed e2` | `NONINOP` |
| `ed e3` | `NONINOP` |
| `ed e4` | `NONINOP` |
| `ed e5` | `NONINOP` |
| `ed e6` | `NONINOP` |
| `ed e7` | `NONINOP` |
| `ed e8` | `NONINOP` |
| `ed e9` | `NONINOP` |
| `ed ea` | `NONINOP` |
| `ed eb` | `NONINOP` |
| `ed ec` | `NONINOP` |
| `ed ed` | `NONINOP` |
| `ed ee` | `NONINOP` |
| `ed ef` | `NONINOP` |
| `ed f0` | `NONINOP` |
| `ed f1` | `NONINOP` |
| `ed f2` | `NONINOP` |
| `ed f3` | `NONINOP` |
| `ed f4` | `NONINOP` |
| `ed f5` | `NONINOP` |
| `ed f6` | `NONINOP` |
| `ed f7` | `NONINOP` |
| `ed f8` | `NONINOP` |
| `ed f9` | `NONINOP` |
| `ed fa` | `NONINOP` |
| `ed fb` | `NONINOP` |
| `ed fc` | `NONINOP` |
| `ed fd` | `NONINOP` |
| `ed fe` | `NONINOP` |
| `ed ff` | `NONINOP` |

## DD-prefixed IX opcodes

| Bytes | Disassembly |
| --- | --- |
| `dd 00` | `NOP` |
| `dd 01 34 12` | `LD BC, $1234` |
| `dd 02` | `LD (BC), A` |
| `dd 03` | `INC BC` |
| `dd 04` | `INC B` |
| `dd 05` | `DEC B` |
| `dd 06 34` | `LD B, $34` |
| `dd 07` | `RLCA` |
| `dd 08` | `EX AF, AF'` |
| `dd 09` | `ADD IX, BC` |
| `dd 0a` | `LD A, (BC)` |
| `dd 0b` | `DEC BC` |
| `dd 0c` | `INC C` |
| `dd 0d` | `DEC C` |
| `dd 0e 34` | `LD C, $34` |
| `dd 0f` | `RRCA` |
| `dd 10 34` | `DJNZ $37` |
| `dd 11 34 12` | `LD DE, $1234` |
| `dd 12` | `LD (DE), A` |
| `dd 13` | `INC DE` |
| `dd 14` | `INC D` |
| `dd 15` | `DEC D` |
| `dd 16 34` | `LD D, $34` |
| `dd 17` | `RLA` |
| `dd 18 34` | `JR $37` |
| `dd 19` | `ADD IX, DE` |
| `dd 1a` | `LD A, (DE)` |
| `dd 1b` | `DEC DE` |
| `dd 1c` | `INC E` |
| `dd 1d` | `DEC E` |
| `dd 1e 34` | `LD E, $34` |
| `dd 1f` | `RRA` |
| `dd 20 34` | `JR NZ, $37` |
| `dd 21 34 12` | `LD IX, $1234` |
| `dd 22 34 12` | `LD ($1234), IX` |
| `dd 23` | `INC IX` |
| `dd 24` | `INC H` |
| `dd 25` | `DEC H` |
| `dd 26 34` | `LD H, $34` |
| `dd 27` | `DAA` |
| `dd 28 34` | `JR Z, $37` |
| `dd 29` | `ADD IX, IX` |
| `dd 2a 34 12` | `LD IX, ($1234)` |
| `dd 2b` | `DEC IX` |
| `dd 2c` | `INC L` |
| `dd 2d` | `DEC L` |
| `dd 2e 34` | `LD L, $34` |
| `dd 2f` | `CPL` |
| `dd 30 34` | `JR NC, $37` |
| `dd 31 34 12` | `LD SP, $1234` |
| `dd 32 34 12` | `LD ($1234), A` |
| `dd 33` | `INC SP` |
| `dd 34 34` | `INC (IX+52)` |
| `dd 35 34` | `DEC (IX+52)` |
| `dd 36 34 12` | `LD (IX+52), $12` |
| `dd 37` | `SCF` |
| `dd 38 34` | `JR C, $37` |
| `dd 39` | `ADD IX, SP` |
| `dd 3a 34 12` | `LD A, ($1234)` |
| `dd 3b` | `DEC SP` |
| `dd 3c` | `INC A` |
| `dd 3d` | `DEC A` |
| `dd 3e 34` | `LD A, $34` |
| `dd 3f` | `CCF` |
| `dd 40` | `LD B, B` |
| `dd 41` | `LD B, C` |
| `dd 42` | `LD B, D` |
| `dd 43` | `LD B, E` |
| `dd 44` | `LD B, H` |
| `dd 45` | `LD B, L` |
| `dd 46 34` | `LD B, (IX+52)` |
| `dd 47` | `LD B, A` |
| `dd 48` | `LD C, B` |
| `dd 49` | `LD C, C` |
| `dd 4a` | `LD C, D` |
| `dd 4b` | `LD C, E` |
| `dd 4c` | `LD C, H` |
| `dd 4d` | `LD C, L` |
| `dd 4e 34` | `LD C, (IX+52)` |
| `dd 4f` | `LD C, A` |
| `dd 50` | `LD D, B` |
| `dd 51` | `LD D, C` |
| `dd 52` | `LD D, D` |
| `dd 53` | `LD D, E` |
| `dd 54` | `LD D, H` |
| `dd 55` | `LD D, L` |
| `dd 56 34` | `LD D, (IX+52)` |
| `dd 57` | `LD D, A` |
| `dd 58` | `LD E, B` |
| `dd 59` | `LD E, C` |
| `dd 5a` | `LD E, D` |
| `dd 5b` | `LD E, E` |
| `dd 5c` | `LD E, H` |
| `dd 5d` | `LD E, L` |
| `dd 5e 34` | `LD E, (IX+52)` |
| `dd 5f` | `LD E, A` |
| `dd 60` | `LD H, B` |
| `dd 61` | `LD H, C` |
| `dd 62` | `LD H, D` |
| `dd 63` | `LD H, E` |
| `dd 64` | `LD H, H` |
| `dd 65` | `LD H, L` |
| `dd 66 34` | `LD H, (IX+52)` |
| `dd 67` | `LD H, A` |
| `dd 68` | `LD L, B` |
| `dd 69` | `LD L, C` |
| `dd 6a` | `LD L, D` |
| `dd 6b` | `LD L, E` |
| `dd 6c` | `LD L, H` |
| `dd 6d` | `LD L, L` |
| `dd 6e 34` | `LD L, (IX+52)` |
| `dd 6f` | `LD L, A` |
| `dd 70 34` | `LD (IX+52), B` |
| `dd 71 34` | `LD (IX+52), C` |
| `dd 72 34` | `LD (IX+52), D` |
| `dd 73 34` | `LD (IX+52), E` |
| `dd 74 34` | `LD (IX+52), H` |
| `dd 75 34` | `LD (IX+52), L` |
| `dd 76` | `HALT` |
| `dd 77 34` | `LD (IX+52), A` |
| `dd 78` | `LD A, B` |
| `dd 79` | `LD A, C` |
| `dd 7a` | `LD A, D` |
| `dd 7b` | `LD A, E` |
| `dd 7c` | `LD A, H` |
| `dd 7d` | `LD A, L` |
| `dd 7e 34` | `LD A, (IX+52)` |
| `dd 7f` | `LD A, A` |
| `dd 80` | `ADD A, B` |
| `dd 81` | `ADD A, C` |
| `dd 82` | `ADD A, D` |
| `dd 83` | `ADD A, E` |
| `dd 84` | `ADD A, H` |
| `dd 85` | `ADD A, L` |
| `dd 86 34` | `ADD A, (IX+52)` |
| `dd 87` | `ADD A, A` |
| `dd 88` | `ADC A, B` |
| `dd 89` | `ADC A, C` |
| `dd 8a` | `ADC A, D` |
| `dd 8b` | `ADC A, E` |
| `dd 8c` | `ADC A, H` |
| `dd 8d` | `ADC A, L` |
| `dd 8e 34` | `ADC A, (IX+52)` |
| `dd 8f` | `ADC A, A` |
| `dd 90` | `SUB A, B` |
| `dd 91` | `SUB A, C` |
| `dd 92` | `SUB A, D` |
| `dd 93` | `SUB A, E` |
| `dd 94` | `SUB A, H` |
| `dd 95` | `SUB A, L` |
| `dd 96 34` | `SUB A, (IX+52)` |
| `dd 97` | `SUB A, A` |
| `dd 98` | `SBC A, B` |
| `dd 99` | `SBC A, C` |
| `dd 9a` | `SBC A, D` |
| `dd 9b` | `SBC A, E` |
| `dd 9c` | `SBC A, H` |
| `dd 9d` | `SBC A, L` |
| `dd 9e 34` | `SBC A, (IX+52)` |
| `dd 9f` | `SBC A, A` |
| `dd a0` | `AND A, B` |
| `dd a1` | `AND A, C` |
| `dd a2` | `AND A, D` |
| `dd a3` | `AND A, E` |
| `dd a4` | `AND A, H` |
| `dd a5` | `AND A, L` |
| `dd a6 34` | `AND A, (IX+52)` |
| `dd a7` | `AND A, A` |
| `dd a8` | `XOR A, B` |
| `dd a9` | `XOR A, C` |
| `dd aa` | `XOR A, D` |
| `dd ab` | `XOR A, E` |
| `dd ac` | `XOR A, H` |
| `dd ad` | `XOR A, L` |
| `dd ae 34` | `XOR A, (IX+52)` |
| `dd af` | `XOR A, A` |
| `dd b0` | `OR A, B` |
| `dd b1` | `OR A, C` |
| `dd b2` | `OR A, D` |
| `dd b3` | `OR A, E` |
| `dd b4` | `OR A, H` |
| `dd b5` | `OR A, L` |
| `dd b6 34` | `OR A, (IX+52)` |
| `dd b7` | `OR A, A` |
| `dd b8` | `CP A, B` |
| `dd b9` | `CP A, C` |
| `dd ba` | `CP A, D` |
| `dd bb` | `CP A, E` |
| `dd bc` | `CP A, H` |
| `dd bd` | `CP A, L` |
| `dd be 34` | `CP A, (IX+52)` |
| `dd bf` | `CP A, A` |
| `dd c0` | `RET NZ` |
| `dd c1` | `POP BC` |
| `dd c2 34 12` | `JP NZ, $1234` |
| `dd c3 34 12` | `JP $1234` |
| `dd c4 34 12` | `CALL NZ, $1234` |
| `dd c5` | `PUSH BC` |
| `dd c6 34` | `ADD A, $34` |
| `dd c7` | `RST 00h` |
| `dd c8` | `RET Z` |
| `dd c9` | `RET` |
| `dd ca 34 12` | `JP Z, $1234` |
| `dd cb 34 12` | `RL D, (IX+52)` |
| `dd cc 34 12` | `CALL Z, $1234` |
| `dd cd 34 12` | `CALL $1234` |
| `dd ce 34` | `ADC A, $34` |
| `dd cf` | `RST 08h` |
| `dd d0` | `RET NC` |
| `dd d1` | `POP DE` |
| `dd d2 34 12` | `JP NC, $1234` |
| `dd d3 34` | `OUT ($34), A` |
| `dd d4 34 12` | `CALL NC, $1234` |
| `dd d5` | `PUSH DE` |
| `dd d6 34` | `SUB A, $34` |
| `dd d7` | `RST 10h` |
| `dd d8` | `RET C` |
| `dd d9` | `EXX` |
| `dd da 34 12` | `JP C, $1234` |
| `dd db 34` | `IN A, ($34)` |
| `dd dc 34 12` | `CALL C, $1234` |
| `dd dd 34 12` | `INC (IX+18)` |
| `dd de 34` | `SBC A, $34` |
| `dd df` | `RST 18h` |
| `dd e0` | `RET PO` |
| `dd e1` | `POP IX` |
| `dd e2 34 12` | `JP PO, $1234` |
| `dd e3` | `EX (SP), IX` |
| `dd e4 34 12` | `CALL PO, $1234` |
| `dd e5` | `PUSH IX` |
| `dd e6 34` | `AND A, $34` |
| `dd e7` | `RST 20h` |
| `dd e8` | `RET PE` |
| `dd e9` | `JP (IX)` |
| `dd ea 34 12` | `JP PE, $1234` |
| `dd eb` | `EX DE, IX` |
| `dd ec 34 12` | `CALL PE, $1234` |
| `dd ed 34 12 56` | `ADD HL, $5612` |
| `dd ee 34` | `XOR A, $34` |
| `dd ef` | `RST 28h` |
| `dd f0` | `RET P` |
| `dd f1` | `POP AF` |
| `dd f2 34 12` | `JP P, $1234` |
| `dd f3` | `DI` |
| `dd f4 34 12` | `CALL P, $1234` |
| `dd f5` | `PUSH AF` |
| `dd f6 34` | `OR A, $34` |
| `dd f7` | `RST 30h` |
| `dd f8` | `RET N` |
| `dd f9` | `LD SP, IX` |
| `dd fa 34 12` | `JP N, $1234` |
| `dd fb` | `EI` |
| `dd fc 34 12` | `CALL N, $1234` |
| `dd fd 34 12` | `INC (IY+18)` |
| `dd fe 34` | `CP A, $34` |
| `dd ff` | `RST 38h` |

## FD-prefixed IY opcodes

| Bytes | Disassembly |
| --- | --- |
| `fd 00` | `NOP` |
| `fd 01 34 12` | `LD BC, $1234` |
| `fd 02` | `LD (BC), A` |
| `fd 03` | `INC BC` |
| `fd 04` | `INC B` |
| `fd 05` | `DEC B` |
| `fd 06 34` | `LD B, $34` |
| `fd 07` | `RLCA` |
| `fd 08` | `EX AF, AF'` |
| `fd 09` | `ADD IY, BC` |
| `fd 0a` | `LD A, (BC)` |
| `fd 0b` | `DEC BC` |
| `fd 0c` | `INC C` |
| `fd 0d` | `DEC C` |
| `fd 0e 34` | `LD C, $34` |
| `fd 0f` | `RRCA` |
| `fd 10 34` | `DJNZ $37` |
| `fd 11 34 12` | `LD DE, $1234` |
| `fd 12` | `LD (DE), A` |
| `fd 13` | `INC DE` |
| `fd 14` | `INC D` |
| `fd 15` | `DEC D` |
| `fd 16 34` | `LD D, $34` |
| `fd 17` | `RLA` |
| `fd 18 34` | `JR $37` |
| `fd 19` | `ADD IY, DE` |
| `fd 1a` | `LD A, (DE)` |
| `fd 1b` | `DEC DE` |
| `fd 1c` | `INC E` |
| `fd 1d` | `DEC E` |
| `fd 1e 34` | `LD E, $34` |
| `fd 1f` | `RRA` |
| `fd 20 34` | `JR NZ, $37` |
| `fd 21 34 12` | `LD IY, $1234` |
| `fd 22 34 12` | `LD ($1234), IY` |
| `fd 23` | `INC IY` |
| `fd 24` | `INC H` |
| `fd 25` | `DEC H` |
| `fd 26 34` | `LD H, $34` |
| `fd 27` | `DAA` |
| `fd 28 34` | `JR Z, $37` |
| `fd 29` | `ADD IY, IY` |
| `fd 2a 34 12` | `LD IY, ($1234)` |
| `fd 2b` | `DEC IY` |
| `fd 2c` | `INC L` |
| `fd 2d` | `DEC L` |
| `fd 2e 34` | `LD L, $34` |
| `fd 2f` | `CPL` |
| `fd 30 34` | `JR NC, $37` |
| `fd 31 34 12` | `LD SP, $1234` |
| `fd 32 34 12` | `LD ($1234), A` |
| `fd 33` | `INC SP` |
| `fd 34 34` | `INC (IY+52)` |
| `fd 35 34` | `DEC (IY+52)` |
| `fd 36 34 12` | `LD (IY+52), $12` |
| `fd 37` | `SCF` |
| `fd 38 34` | `JR C, $37` |
| `fd 39` | `ADD IY, SP` |
| `fd 3a 34 12` | `LD A, ($1234)` |
| `fd 3b` | `DEC SP` |
| `fd 3c` | `INC A` |
| `fd 3d` | `DEC A` |
| `fd 3e 34` | `LD A, $34` |
| `fd 3f` | `CCF` |
| `fd 40` | `LD B, B` |
| `fd 41` | `LD B, C` |
| `fd 42` | `LD B, D` |
| `fd 43` | `LD B, E` |
| `fd 44` | `LD B, H` |
| `fd 45` | `LD B, L` |
| `fd 46 34` | `LD B, (IY+52)` |
| `fd 47` | `LD B, A` |
| `fd 48` | `LD C, B` |
| `fd 49` | `LD C, C` |
| `fd 4a` | `LD C, D` |
| `fd 4b` | `LD C, E` |
| `fd 4c` | `LD C, H` |
| `fd 4d` | `LD C, L` |
| `fd 4e 34` | `LD C, (IY+52)` |
| `fd 4f` | `LD C, A` |
| `fd 50` | `LD D, B` |
| `fd 51` | `LD D, C` |
| `fd 52` | `LD D, D` |
| `fd 53` | `LD D, E` |
| `fd 54` | `LD D, H` |
| `fd 55` | `LD D, L` |
| `fd 56 34` | `LD D, (IY+52)` |
| `fd 57` | `LD D, A` |
| `fd 58` | `LD E, B` |
| `fd 59` | `LD E, C` |
| `fd 5a` | `LD E, D` |
| `fd 5b` | `LD E, E` |
| `fd 5c` | `LD E, H` |
| `fd 5d` | `LD E, L` |
| `fd 5e 34` | `LD E, (IY+52)` |
| `fd 5f` | `LD E, A` |
| `fd 60` | `LD H, B` |
| `fd 61` | `LD H, C` |
| `fd 62` | `LD H, D` |
| `fd 63` | `LD H, E` |
| `fd 64` | `LD H, H` |
| `fd 65` | `LD H, L` |
| `fd 66 34` | `LD H, (IY+52)` |
| `fd 67` | `LD H, A` |
| `fd 68` | `LD L, B` |
| `fd 69` | `LD L, C` |
| `fd 6a` | `LD L, D` |
| `fd 6b` | `LD L, E` |
| `fd 6c` | `LD L, H` |
| `fd 6d` | `LD L, L` |
| `fd 6e 34` | `LD L, (IY+52)` |
| `fd 6f` | `LD L, A` |
| `fd 70 34` | `LD (IY+52), B` |
| `fd 71 34` | `LD (IY+52), C` |
| `fd 72 34` | `LD (IY+52), D` |
| `fd 73 34` | `LD (IY+52), E` |
| `fd 74 34` | `LD (IY+52), H` |
| `fd 75 34` | `LD (IY+52), L` |
| `fd 76` | `HALT` |
| `fd 77 34` | `LD (IY+52), A` |
| `fd 78` | `LD A, B` |
| `fd 79` | `LD A, C` |
| `fd 7a` | `LD A, D` |
| `fd 7b` | `LD A, E` |
| `fd 7c` | `LD A, H` |
| `fd 7d` | `LD A, L` |
| `fd 7e 34` | `LD A, (IY+52)` |
| `fd 7f` | `LD A, A` |
| `fd 80` | `ADD A, B` |
| `fd 81` | `ADD A, C` |
| `fd 82` | `ADD A, D` |
| `fd 83` | `ADD A, E` |
| `fd 84` | `ADD A, H` |
| `fd 85` | `ADD A, L` |
| `fd 86 34` | `ADD A, (IY+52)` |
| `fd 87` | `ADD A, A` |
| `fd 88` | `ADC A, B` |
| `fd 89` | `ADC A, C` |
| `fd 8a` | `ADC A, D` |
| `fd 8b` | `ADC A, E` |
| `fd 8c` | `ADC A, H` |
| `fd 8d` | `ADC A, L` |
| `fd 8e 34` | `ADC A, (IY+52)` |
| `fd 8f` | `ADC A, A` |
| `fd 90` | `SUB A, B` |
| `fd 91` | `SUB A, C` |
| `fd 92` | `SUB A, D` |
| `fd 93` | `SUB A, E` |
| `fd 94` | `SUB A, H` |
| `fd 95` | `SUB A, L` |
| `fd 96 34` | `SUB A, (IY+52)` |
| `fd 97` | `SUB A, A` |
| `fd 98` | `SBC A, B` |
| `fd 99` | `SBC A, C` |
| `fd 9a` | `SBC A, D` |
| `fd 9b` | `SBC A, E` |
| `fd 9c` | `SBC A, H` |
| `fd 9d` | `SBC A, L` |
| `fd 9e 34` | `SBC A, (IY+52)` |
| `fd 9f` | `SBC A, A` |
| `fd a0` | `AND A, B` |
| `fd a1` | `AND A, C` |
| `fd a2` | `AND A, D` |
| `fd a3` | `AND A, E` |
| `fd a4` | `AND A, H` |
| `fd a5` | `AND A, L` |
| `fd a6 34` | `AND A, (IY+52)` |
| `fd a7` | `AND A, A` |
| `fd a8` | `XOR A, B` |
| `fd a9` | `XOR A, C` |
| `fd aa` | `XOR A, D` |
| `fd ab` | `XOR A, E` |
| `fd ac` | `XOR A, H` |
| `fd ad` | `XOR A, L` |
| `fd ae 34` | `XOR A, (IY+52)` |
| `fd af` | `XOR A, A` |
| `fd b0` | `OR A, B` |
| `fd b1` | `OR A, C` |
| `fd b2` | `OR A, D` |
| `fd b3` | `OR A, E` |
| `fd b4` | `OR A, H` |
| `fd b5` | `OR A, L` |
| `fd b6 34` | `OR A, (IY+52)` |
| `fd b7` | `OR A, A` |
| `fd b8` | `CP A, B` |
| `fd b9` | `CP A, C` |
| `fd ba` | `CP A, D` |
| `fd bb` | `CP A, E` |
| `fd bc` | `CP A, H` |
| `fd bd` | `CP A, L` |
| `fd be 34` | `CP A, (IY+52)` |
| `fd bf` | `CP A, A` |
| `fd c0` | `RET NZ` |
| `fd c1` | `POP BC` |
| `fd c2 34 12` | `JP NZ, $1234` |
| `fd c3 34 12` | `JP $1234` |
| `fd c4 34 12` | `CALL NZ, $1234` |
| `fd c5` | `PUSH BC` |
| `fd c6 34` | `ADD A, $34` |
| `fd c7` | `RST 00h` |
| `fd c8` | `RET Z` |
| `fd c9` | `RET` |
| `fd ca 34 12` | `JP Z, $1234` |
| `fd cb 34 12` | `RL D, (IY+52)` |
| `fd cc 34 12` | `CALL Z, $1234` |
| `fd cd 34 12` | `CALL $1234` |
| `fd ce 34` | `ADC A, $34` |
| `fd cf` | `RST 08h` |
| `fd d0` | `RET NC` |
| `fd d1` | `POP DE` |
| `fd d2 34 12` | `JP NC, $1234` |
| `fd d3 34` | `OUT ($34), A` |
| `fd d4 34 12` | `CALL NC, $1234` |
| `fd d5` | `PUSH DE` |
| `fd d6 34` | `SUB A, $34` |
| `fd d7` | `RST 10h` |
| `fd d8` | `RET C` |
| `fd d9` | `EXX` |
| `fd da 34 12` | `JP C, $1234` |
| `fd db 34` | `IN A, ($34)` |
| `fd dc 34 12` | `CALL C, $1234` |
| `fd dd 34 12` | `INC (IX+18)` |
| `fd de 34` | `SBC A, $34` |
| `fd df` | `RST 18h` |
| `fd e0` | `RET PO` |
| `fd e1` | `POP IY` |
| `fd e2 34 12` | `JP PO, $1234` |
| `fd e3` | `EX (SP), IY` |
| `fd e4 34 12` | `CALL PO, $1234` |
| `fd e5` | `PUSH IY` |
| `fd e6 34` | `AND A, $34` |
| `fd e7` | `RST 20h` |
| `fd e8` | `RET PE` |
| `fd e9` | `JP (IY)` |
| `fd ea 34 12` | `JP PE, $1234` |
| `fd eb` | `EX DE, IY` |
| `fd ec 34 12` | `CALL PE, $1234` |
| `fd ed 34 12 56` | `ADD HL, $5612` |
| `fd ee 34` | `XOR A, $34` |
| `fd ef` | `RST 28h` |
| `fd f0` | `RET P` |
| `fd f1` | `POP AF` |
| `fd f2 34 12` | `JP P, $1234` |
| `fd f3` | `DI` |
| `fd f4 34 12` | `CALL P, $1234` |
| `fd f5` | `PUSH AF` |
| `fd f6 34` | `OR A, $34` |
| `fd f7` | `RST 30h` |
| `fd f8` | `RET N` |
| `fd f9` | `LD SP, IY` |
| `fd fa 34 12` | `JP N, $1234` |
| `fd fb` | `EI` |
| `fd fc 34 12` | `CALL N, $1234` |
| `fd fd 34 12` | `INC (IY+18)` |
| `fd fe 34` | `CP A, $34` |
| `fd ff` | `RST 38h` |

## DD CB indexed bit opcodes

| Bytes | Disassembly |
| --- | --- |
| `dd cb 01 00` | `RLC B, (IX+1)` |
| `dd cb 01 01` | `RLC C, (IX+1)` |
| `dd cb 01 02` | `RLC D, (IX+1)` |
| `dd cb 01 03` | `RLC E, (IX+1)` |
| `dd cb 01 04` | `RLC H, (IX+1)` |
| `dd cb 01 05` | `RLC L, (IX+1)` |
| `dd cb 01 06` | `RLC (IX+1), (IX+1)` |
| `dd cb 01 07` | `RLC A, (IX+1)` |
| `dd cb 01 08` | `RRC B, (IX+1)` |
| `dd cb 01 09` | `RRC C, (IX+1)` |
| `dd cb 01 0a` | `RRC D, (IX+1)` |
| `dd cb 01 0b` | `RRC E, (IX+1)` |
| `dd cb 01 0c` | `RRC H, (IX+1)` |
| `dd cb 01 0d` | `RRC L, (IX+1)` |
| `dd cb 01 0e` | `RRC (IX+1), (IX+1)` |
| `dd cb 01 0f` | `RRC A, (IX+1)` |
| `dd cb 01 10` | `RL B, (IX+1)` |
| `dd cb 01 11` | `RL C, (IX+1)` |
| `dd cb 01 12` | `RL D, (IX+1)` |
| `dd cb 01 13` | `RL E, (IX+1)` |
| `dd cb 01 14` | `RL H, (IX+1)` |
| `dd cb 01 15` | `RL L, (IX+1)` |
| `dd cb 01 16` | `RL (IX+1), (IX+1)` |
| `dd cb 01 17` | `RL A, (IX+1)` |
| `dd cb 01 18` | `RR B, (IX+1)` |
| `dd cb 01 19` | `RR C, (IX+1)` |
| `dd cb 01 1a` | `RR D, (IX+1)` |
| `dd cb 01 1b` | `RR E, (IX+1)` |
| `dd cb 01 1c` | `RR H, (IX+1)` |
| `dd cb 01 1d` | `RR L, (IX+1)` |
| `dd cb 01 1e` | `RR (IX+1), (IX+1)` |
| `dd cb 01 1f` | `RR A, (IX+1)` |
| `dd cb 01 20` | `SLA B, (IX+1)` |
| `dd cb 01 21` | `SLA C, (IX+1)` |
| `dd cb 01 22` | `SLA D, (IX+1)` |
| `dd cb 01 23` | `SLA E, (IX+1)` |
| `dd cb 01 24` | `SLA H, (IX+1)` |
| `dd cb 01 25` | `SLA L, (IX+1)` |
| `dd cb 01 26` | `SLA (IX+1), (IX+1)` |
| `dd cb 01 27` | `SLA A, (IX+1)` |
| `dd cb 01 28` | `SRA B, (IX+1)` |
| `dd cb 01 29` | `SRA C, (IX+1)` |
| `dd cb 01 2a` | `SRA D, (IX+1)` |
| `dd cb 01 2b` | `SRA E, (IX+1)` |
| `dd cb 01 2c` | `SRA H, (IX+1)` |
| `dd cb 01 2d` | `SRA L, (IX+1)` |
| `dd cb 01 2e` | `SRA (IX+1), (IX+1)` |
| `dd cb 01 2f` | `SRA A, (IX+1)` |
| `dd cb 01 30` | `SLL B, (IX+1)` |
| `dd cb 01 31` | `SLL C, (IX+1)` |
| `dd cb 01 32` | `SLL D, (IX+1)` |
| `dd cb 01 33` | `SLL E, (IX+1)` |
| `dd cb 01 34` | `SLL H, (IX+1)` |
| `dd cb 01 35` | `SLL L, (IX+1)` |
| `dd cb 01 36` | `SLL (IX+1), (IX+1)` |
| `dd cb 01 37` | `SLL A, (IX+1)` |
| `dd cb 01 38` | `SRL B, (IX+1)` |
| `dd cb 01 39` | `SRL C, (IX+1)` |
| `dd cb 01 3a` | `SRL D, (IX+1)` |
| `dd cb 01 3b` | `SRL E, (IX+1)` |
| `dd cb 01 3c` | `SRL H, (IX+1)` |
| `dd cb 01 3d` | `SRL L, (IX+1)` |
| `dd cb 01 3e` | `SRL (IX+1), (IX+1)` |
| `dd cb 01 3f` | `SRL A, (IX+1)` |
| `dd cb 01 40` | `BIT 0, (IX+1)` |
| `dd cb 01 41` | `BIT 0, (IX+1)` |
| `dd cb 01 42` | `BIT 0, (IX+1)` |
| `dd cb 01 43` | `BIT 0, (IX+1)` |
| `dd cb 01 44` | `BIT 0, (IX+1)` |
| `dd cb 01 45` | `BIT 0, (IX+1)` |
| `dd cb 01 46` | `BIT 0, (IX+1)` |
| `dd cb 01 47` | `BIT 0, (IX+1)` |
| `dd cb 01 48` | `BIT 1, (IX+1)` |
| `dd cb 01 49` | `BIT 1, (IX+1)` |
| `dd cb 01 4a` | `BIT 1, (IX+1)` |
| `dd cb 01 4b` | `BIT 1, (IX+1)` |
| `dd cb 01 4c` | `BIT 1, (IX+1)` |
| `dd cb 01 4d` | `BIT 1, (IX+1)` |
| `dd cb 01 4e` | `BIT 1, (IX+1)` |
| `dd cb 01 4f` | `BIT 1, (IX+1)` |
| `dd cb 01 50` | `BIT 2, (IX+1)` |
| `dd cb 01 51` | `BIT 2, (IX+1)` |
| `dd cb 01 52` | `BIT 2, (IX+1)` |
| `dd cb 01 53` | `BIT 2, (IX+1)` |
| `dd cb 01 54` | `BIT 2, (IX+1)` |
| `dd cb 01 55` | `BIT 2, (IX+1)` |
| `dd cb 01 56` | `BIT 2, (IX+1)` |
| `dd cb 01 57` | `BIT 2, (IX+1)` |
| `dd cb 01 58` | `BIT 3, (IX+1)` |
| `dd cb 01 59` | `BIT 3, (IX+1)` |
| `dd cb 01 5a` | `BIT 3, (IX+1)` |
| `dd cb 01 5b` | `BIT 3, (IX+1)` |
| `dd cb 01 5c` | `BIT 3, (IX+1)` |
| `dd cb 01 5d` | `BIT 3, (IX+1)` |
| `dd cb 01 5e` | `BIT 3, (IX+1)` |
| `dd cb 01 5f` | `BIT 3, (IX+1)` |
| `dd cb 01 60` | `BIT 4, (IX+1)` |
| `dd cb 01 61` | `BIT 4, (IX+1)` |
| `dd cb 01 62` | `BIT 4, (IX+1)` |
| `dd cb 01 63` | `BIT 4, (IX+1)` |
| `dd cb 01 64` | `BIT 4, (IX+1)` |
| `dd cb 01 65` | `BIT 4, (IX+1)` |
| `dd cb 01 66` | `BIT 4, (IX+1)` |
| `dd cb 01 67` | `BIT 4, (IX+1)` |
| `dd cb 01 68` | `BIT 5, (IX+1)` |
| `dd cb 01 69` | `BIT 5, (IX+1)` |
| `dd cb 01 6a` | `BIT 5, (IX+1)` |
| `dd cb 01 6b` | `BIT 5, (IX+1)` |
| `dd cb 01 6c` | `BIT 5, (IX+1)` |
| `dd cb 01 6d` | `BIT 5, (IX+1)` |
| `dd cb 01 6e` | `BIT 5, (IX+1)` |
| `dd cb 01 6f` | `BIT 5, (IX+1)` |
| `dd cb 01 70` | `BIT 6, (IX+1)` |
| `dd cb 01 71` | `BIT 6, (IX+1)` |
| `dd cb 01 72` | `BIT 6, (IX+1)` |
| `dd cb 01 73` | `BIT 6, (IX+1)` |
| `dd cb 01 74` | `BIT 6, (IX+1)` |
| `dd cb 01 75` | `BIT 6, (IX+1)` |
| `dd cb 01 76` | `BIT 6, (IX+1)` |
| `dd cb 01 77` | `BIT 6, (IX+1)` |
| `dd cb 01 78` | `BIT 7, (IX+1)` |
| `dd cb 01 79` | `BIT 7, (IX+1)` |
| `dd cb 01 7a` | `BIT 7, (IX+1)` |
| `dd cb 01 7b` | `BIT 7, (IX+1)` |
| `dd cb 01 7c` | `BIT 7, (IX+1)` |
| `dd cb 01 7d` | `BIT 7, (IX+1)` |
| `dd cb 01 7e` | `BIT 7, (IX+1)` |
| `dd cb 01 7f` | `BIT 7, (IX+1)` |
| `dd cb 01 80` | `LD B, RES 0, (IX+1)` |
| `dd cb 01 81` | `LD C, RES 0, (IX+1)` |
| `dd cb 01 82` | `LD D, RES 0, (IX+1)` |
| `dd cb 01 83` | `LD E, RES 0, (IX+1)` |
| `dd cb 01 84` | `LD H, RES 0, (IX+1)` |
| `dd cb 01 85` | `LD L, RES 0, (IX+1)` |
| `dd cb 01 86` | `LD (IX+1), RES 0, (IX+1)` |
| `dd cb 01 87` | `LD A, RES 0, (IX+1)` |
| `dd cb 01 88` | `LD B, RES 1, (IX+1)` |
| `dd cb 01 89` | `LD C, RES 1, (IX+1)` |
| `dd cb 01 8a` | `LD D, RES 1, (IX+1)` |
| `dd cb 01 8b` | `LD E, RES 1, (IX+1)` |
| `dd cb 01 8c` | `LD H, RES 1, (IX+1)` |
| `dd cb 01 8d` | `LD L, RES 1, (IX+1)` |
| `dd cb 01 8e` | `LD (IX+1), RES 1, (IX+1)` |
| `dd cb 01 8f` | `LD A, RES 1, (IX+1)` |
| `dd cb 01 90` | `LD B, RES 2, (IX+1)` |
| `dd cb 01 91` | `LD C, RES 2, (IX+1)` |
| `dd cb 01 92` | `LD D, RES 2, (IX+1)` |
| `dd cb 01 93` | `LD E, RES 2, (IX+1)` |
| `dd cb 01 94` | `LD H, RES 2, (IX+1)` |
| `dd cb 01 95` | `LD L, RES 2, (IX+1)` |
| `dd cb 01 96` | `LD (IX+1), RES 2, (IX+1)` |
| `dd cb 01 97` | `LD A, RES 2, (IX+1)` |
| `dd cb 01 98` | `LD B, RES 3, (IX+1)` |
| `dd cb 01 99` | `LD C, RES 3, (IX+1)` |
| `dd cb 01 9a` | `LD D, RES 3, (IX+1)` |
| `dd cb 01 9b` | `LD E, RES 3, (IX+1)` |
| `dd cb 01 9c` | `LD H, RES 3, (IX+1)` |
| `dd cb 01 9d` | `LD L, RES 3, (IX+1)` |
| `dd cb 01 9e` | `LD (IX+1), RES 3, (IX+1)` |
| `dd cb 01 9f` | `LD A, RES 3, (IX+1)` |
| `dd cb 01 a0` | `LD B, RES 4, (IX+1)` |
| `dd cb 01 a1` | `LD C, RES 4, (IX+1)` |
| `dd cb 01 a2` | `LD D, RES 4, (IX+1)` |
| `dd cb 01 a3` | `LD E, RES 4, (IX+1)` |
| `dd cb 01 a4` | `LD H, RES 4, (IX+1)` |
| `dd cb 01 a5` | `LD L, RES 4, (IX+1)` |
| `dd cb 01 a6` | `LD (IX+1), RES 4, (IX+1)` |
| `dd cb 01 a7` | `LD A, RES 4, (IX+1)` |
| `dd cb 01 a8` | `LD B, RES 5, (IX+1)` |
| `dd cb 01 a9` | `LD C, RES 5, (IX+1)` |
| `dd cb 01 aa` | `LD D, RES 5, (IX+1)` |
| `dd cb 01 ab` | `LD E, RES 5, (IX+1)` |
| `dd cb 01 ac` | `LD H, RES 5, (IX+1)` |
| `dd cb 01 ad` | `LD L, RES 5, (IX+1)` |
| `dd cb 01 ae` | `LD (IX+1), RES 5, (IX+1)` |
| `dd cb 01 af` | `LD A, RES 5, (IX+1)` |
| `dd cb 01 b0` | `LD B, RES 6, (IX+1)` |
| `dd cb 01 b1` | `LD C, RES 6, (IX+1)` |
| `dd cb 01 b2` | `LD D, RES 6, (IX+1)` |
| `dd cb 01 b3` | `LD E, RES 6, (IX+1)` |
| `dd cb 01 b4` | `LD H, RES 6, (IX+1)` |
| `dd cb 01 b5` | `LD L, RES 6, (IX+1)` |
| `dd cb 01 b6` | `LD (IX+1), RES 6, (IX+1)` |
| `dd cb 01 b7` | `LD A, RES 6, (IX+1)` |
| `dd cb 01 b8` | `LD B, RES 7, (IX+1)` |
| `dd cb 01 b9` | `LD C, RES 7, (IX+1)` |
| `dd cb 01 ba` | `LD D, RES 7, (IX+1)` |
| `dd cb 01 bb` | `LD E, RES 7, (IX+1)` |
| `dd cb 01 bc` | `LD H, RES 7, (IX+1)` |
| `dd cb 01 bd` | `LD L, RES 7, (IX+1)` |
| `dd cb 01 be` | `LD (IX+1), RES 7, (IX+1)` |
| `dd cb 01 bf` | `LD A, RES 7, (IX+1)` |
| `dd cb 01 c0` | `LD B, SET 0, (IX+1)` |
| `dd cb 01 c1` | `LD C, SET 0, (IX+1)` |
| `dd cb 01 c2` | `LD D, SET 0, (IX+1)` |
| `dd cb 01 c3` | `LD E, SET 0, (IX+1)` |
| `dd cb 01 c4` | `LD H, SET 0, (IX+1)` |
| `dd cb 01 c5` | `LD L, SET 0, (IX+1)` |
| `dd cb 01 c6` | `LD (IX+1), SET 0, (IX+1)` |
| `dd cb 01 c7` | `LD A, SET 0, (IX+1)` |
| `dd cb 01 c8` | `LD B, SET 1, (IX+1)` |
| `dd cb 01 c9` | `LD C, SET 1, (IX+1)` |
| `dd cb 01 ca` | `LD D, SET 1, (IX+1)` |
| `dd cb 01 cb` | `LD E, SET 1, (IX+1)` |
| `dd cb 01 cc` | `LD H, SET 1, (IX+1)` |
| `dd cb 01 cd` | `LD L, SET 1, (IX+1)` |
| `dd cb 01 ce` | `LD (IX+1), SET 1, (IX+1)` |
| `dd cb 01 cf` | `LD A, SET 1, (IX+1)` |
| `dd cb 01 d0` | `LD B, SET 2, (IX+1)` |
| `dd cb 01 d1` | `LD C, SET 2, (IX+1)` |
| `dd cb 01 d2` | `LD D, SET 2, (IX+1)` |
| `dd cb 01 d3` | `LD E, SET 2, (IX+1)` |
| `dd cb 01 d4` | `LD H, SET 2, (IX+1)` |
| `dd cb 01 d5` | `LD L, SET 2, (IX+1)` |
| `dd cb 01 d6` | `LD (IX+1), SET 2, (IX+1)` |
| `dd cb 01 d7` | `LD A, SET 2, (IX+1)` |
| `dd cb 01 d8` | `LD B, SET 3, (IX+1)` |
| `dd cb 01 d9` | `LD C, SET 3, (IX+1)` |
| `dd cb 01 da` | `LD D, SET 3, (IX+1)` |
| `dd cb 01 db` | `LD E, SET 3, (IX+1)` |
| `dd cb 01 dc` | `LD H, SET 3, (IX+1)` |
| `dd cb 01 dd` | `LD L, SET 3, (IX+1)` |
| `dd cb 01 de` | `LD (IX+1), SET 3, (IX+1)` |
| `dd cb 01 df` | `LD A, SET 3, (IX+1)` |
| `dd cb 01 e0` | `LD B, SET 4, (IX+1)` |
| `dd cb 01 e1` | `LD C, SET 4, (IX+1)` |
| `dd cb 01 e2` | `LD D, SET 4, (IX+1)` |
| `dd cb 01 e3` | `LD E, SET 4, (IX+1)` |
| `dd cb 01 e4` | `LD H, SET 4, (IX+1)` |
| `dd cb 01 e5` | `LD L, SET 4, (IX+1)` |
| `dd cb 01 e6` | `LD (IX+1), SET 4, (IX+1)` |
| `dd cb 01 e7` | `LD A, SET 4, (IX+1)` |
| `dd cb 01 e8` | `LD B, SET 5, (IX+1)` |
| `dd cb 01 e9` | `LD C, SET 5, (IX+1)` |
| `dd cb 01 ea` | `LD D, SET 5, (IX+1)` |
| `dd cb 01 eb` | `LD E, SET 5, (IX+1)` |
| `dd cb 01 ec` | `LD H, SET 5, (IX+1)` |
| `dd cb 01 ed` | `LD L, SET 5, (IX+1)` |
| `dd cb 01 ee` | `LD (IX+1), SET 5, (IX+1)` |
| `dd cb 01 ef` | `LD A, SET 5, (IX+1)` |
| `dd cb 01 f0` | `LD B, SET 6, (IX+1)` |
| `dd cb 01 f1` | `LD C, SET 6, (IX+1)` |
| `dd cb 01 f2` | `LD D, SET 6, (IX+1)` |
| `dd cb 01 f3` | `LD E, SET 6, (IX+1)` |
| `dd cb 01 f4` | `LD H, SET 6, (IX+1)` |
| `dd cb 01 f5` | `LD L, SET 6, (IX+1)` |
| `dd cb 01 f6` | `LD (IX+1), SET 6, (IX+1)` |
| `dd cb 01 f7` | `LD A, SET 6, (IX+1)` |
| `dd cb 01 f8` | `LD B, SET 7, (IX+1)` |
| `dd cb 01 f9` | `LD C, SET 7, (IX+1)` |
| `dd cb 01 fa` | `LD D, SET 7, (IX+1)` |
| `dd cb 01 fb` | `LD E, SET 7, (IX+1)` |
| `dd cb 01 fc` | `LD H, SET 7, (IX+1)` |
| `dd cb 01 fd` | `LD L, SET 7, (IX+1)` |
| `dd cb 01 fe` | `LD (IX+1), SET 7, (IX+1)` |
| `dd cb 01 ff` | `LD A, SET 7, (IX+1)` |

## FD CB indexed bit opcodes

| Bytes | Disassembly |
| --- | --- |
| `fd cb 01 00` | `RLC B, (IY+1)` |
| `fd cb 01 01` | `RLC C, (IY+1)` |
| `fd cb 01 02` | `RLC D, (IY+1)` |
| `fd cb 01 03` | `RLC E, (IY+1)` |
| `fd cb 01 04` | `RLC H, (IY+1)` |
| `fd cb 01 05` | `RLC L, (IY+1)` |
| `fd cb 01 06` | `RLC (IY+1), (IY+1)` |
| `fd cb 01 07` | `RLC A, (IY+1)` |
| `fd cb 01 08` | `RRC B, (IY+1)` |
| `fd cb 01 09` | `RRC C, (IY+1)` |
| `fd cb 01 0a` | `RRC D, (IY+1)` |
| `fd cb 01 0b` | `RRC E, (IY+1)` |
| `fd cb 01 0c` | `RRC H, (IY+1)` |
| `fd cb 01 0d` | `RRC L, (IY+1)` |
| `fd cb 01 0e` | `RRC (IY+1), (IY+1)` |
| `fd cb 01 0f` | `RRC A, (IY+1)` |
| `fd cb 01 10` | `RL B, (IY+1)` |
| `fd cb 01 11` | `RL C, (IY+1)` |
| `fd cb 01 12` | `RL D, (IY+1)` |
| `fd cb 01 13` | `RL E, (IY+1)` |
| `fd cb 01 14` | `RL H, (IY+1)` |
| `fd cb 01 15` | `RL L, (IY+1)` |
| `fd cb 01 16` | `RL (IY+1), (IY+1)` |
| `fd cb 01 17` | `RL A, (IY+1)` |
| `fd cb 01 18` | `RR B, (IY+1)` |
| `fd cb 01 19` | `RR C, (IY+1)` |
| `fd cb 01 1a` | `RR D, (IY+1)` |
| `fd cb 01 1b` | `RR E, (IY+1)` |
| `fd cb 01 1c` | `RR H, (IY+1)` |
| `fd cb 01 1d` | `RR L, (IY+1)` |
| `fd cb 01 1e` | `RR (IY+1), (IY+1)` |
| `fd cb 01 1f` | `RR A, (IY+1)` |
| `fd cb 01 20` | `SLA B, (IY+1)` |
| `fd cb 01 21` | `SLA C, (IY+1)` |
| `fd cb 01 22` | `SLA D, (IY+1)` |
| `fd cb 01 23` | `SLA E, (IY+1)` |
| `fd cb 01 24` | `SLA H, (IY+1)` |
| `fd cb 01 25` | `SLA L, (IY+1)` |
| `fd cb 01 26` | `SLA (IY+1), (IY+1)` |
| `fd cb 01 27` | `SLA A, (IY+1)` |
| `fd cb 01 28` | `SRA B, (IY+1)` |
| `fd cb 01 29` | `SRA C, (IY+1)` |
| `fd cb 01 2a` | `SRA D, (IY+1)` |
| `fd cb 01 2b` | `SRA E, (IY+1)` |
| `fd cb 01 2c` | `SRA H, (IY+1)` |
| `fd cb 01 2d` | `SRA L, (IY+1)` |
| `fd cb 01 2e` | `SRA (IY+1), (IY+1)` |
| `fd cb 01 2f` | `SRA A, (IY+1)` |
| `fd cb 01 30` | `SLL B, (IY+1)` |
| `fd cb 01 31` | `SLL C, (IY+1)` |
| `fd cb 01 32` | `SLL D, (IY+1)` |
| `fd cb 01 33` | `SLL E, (IY+1)` |
| `fd cb 01 34` | `SLL H, (IY+1)` |
| `fd cb 01 35` | `SLL L, (IY+1)` |
| `fd cb 01 36` | `SLL (IY+1), (IY+1)` |
| `fd cb 01 37` | `SLL A, (IY+1)` |
| `fd cb 01 38` | `SRL B, (IY+1)` |
| `fd cb 01 39` | `SRL C, (IY+1)` |
| `fd cb 01 3a` | `SRL D, (IY+1)` |
| `fd cb 01 3b` | `SRL E, (IY+1)` |
| `fd cb 01 3c` | `SRL H, (IY+1)` |
| `fd cb 01 3d` | `SRL L, (IY+1)` |
| `fd cb 01 3e` | `SRL (IY+1), (IY+1)` |
| `fd cb 01 3f` | `SRL A, (IY+1)` |
| `fd cb 01 40` | `BIT 0, (IY+1)` |
| `fd cb 01 41` | `BIT 0, (IY+1)` |
| `fd cb 01 42` | `BIT 0, (IY+1)` |
| `fd cb 01 43` | `BIT 0, (IY+1)` |
| `fd cb 01 44` | `BIT 0, (IY+1)` |
| `fd cb 01 45` | `BIT 0, (IY+1)` |
| `fd cb 01 46` | `BIT 0, (IY+1)` |
| `fd cb 01 47` | `BIT 0, (IY+1)` |
| `fd cb 01 48` | `BIT 1, (IY+1)` |
| `fd cb 01 49` | `BIT 1, (IY+1)` |
| `fd cb 01 4a` | `BIT 1, (IY+1)` |
| `fd cb 01 4b` | `BIT 1, (IY+1)` |
| `fd cb 01 4c` | `BIT 1, (IY+1)` |
| `fd cb 01 4d` | `BIT 1, (IY+1)` |
| `fd cb 01 4e` | `BIT 1, (IY+1)` |
| `fd cb 01 4f` | `BIT 1, (IY+1)` |
| `fd cb 01 50` | `BIT 2, (IY+1)` |
| `fd cb 01 51` | `BIT 2, (IY+1)` |
| `fd cb 01 52` | `BIT 2, (IY+1)` |
| `fd cb 01 53` | `BIT 2, (IY+1)` |
| `fd cb 01 54` | `BIT 2, (IY+1)` |
| `fd cb 01 55` | `BIT 2, (IY+1)` |
| `fd cb 01 56` | `BIT 2, (IY+1)` |
| `fd cb 01 57` | `BIT 2, (IY+1)` |
| `fd cb 01 58` | `BIT 3, (IY+1)` |
| `fd cb 01 59` | `BIT 3, (IY+1)` |
| `fd cb 01 5a` | `BIT 3, (IY+1)` |
| `fd cb 01 5b` | `BIT 3, (IY+1)` |
| `fd cb 01 5c` | `BIT 3, (IY+1)` |
| `fd cb 01 5d` | `BIT 3, (IY+1)` |
| `fd cb 01 5e` | `BIT 3, (IY+1)` |
| `fd cb 01 5f` | `BIT 3, (IY+1)` |
| `fd cb 01 60` | `BIT 4, (IY+1)` |
| `fd cb 01 61` | `BIT 4, (IY+1)` |
| `fd cb 01 62` | `BIT 4, (IY+1)` |
| `fd cb 01 63` | `BIT 4, (IY+1)` |
| `fd cb 01 64` | `BIT 4, (IY+1)` |
| `fd cb 01 65` | `BIT 4, (IY+1)` |
| `fd cb 01 66` | `BIT 4, (IY+1)` |
| `fd cb 01 67` | `BIT 4, (IY+1)` |
| `fd cb 01 68` | `BIT 5, (IY+1)` |
| `fd cb 01 69` | `BIT 5, (IY+1)` |
| `fd cb 01 6a` | `BIT 5, (IY+1)` |
| `fd cb 01 6b` | `BIT 5, (IY+1)` |
| `fd cb 01 6c` | `BIT 5, (IY+1)` |
| `fd cb 01 6d` | `BIT 5, (IY+1)` |
| `fd cb 01 6e` | `BIT 5, (IY+1)` |
| `fd cb 01 6f` | `BIT 5, (IY+1)` |
| `fd cb 01 70` | `BIT 6, (IY+1)` |
| `fd cb 01 71` | `BIT 6, (IY+1)` |
| `fd cb 01 72` | `BIT 6, (IY+1)` |
| `fd cb 01 73` | `BIT 6, (IY+1)` |
| `fd cb 01 74` | `BIT 6, (IY+1)` |
| `fd cb 01 75` | `BIT 6, (IY+1)` |
| `fd cb 01 76` | `BIT 6, (IY+1)` |
| `fd cb 01 77` | `BIT 6, (IY+1)` |
| `fd cb 01 78` | `BIT 7, (IY+1)` |
| `fd cb 01 79` | `BIT 7, (IY+1)` |
| `fd cb 01 7a` | `BIT 7, (IY+1)` |
| `fd cb 01 7b` | `BIT 7, (IY+1)` |
| `fd cb 01 7c` | `BIT 7, (IY+1)` |
| `fd cb 01 7d` | `BIT 7, (IY+1)` |
| `fd cb 01 7e` | `BIT 7, (IY+1)` |
| `fd cb 01 7f` | `BIT 7, (IY+1)` |
| `fd cb 01 80` | `LD B, RES 0, (IY+1)` |
| `fd cb 01 81` | `LD C, RES 0, (IY+1)` |
| `fd cb 01 82` | `LD D, RES 0, (IY+1)` |
| `fd cb 01 83` | `LD E, RES 0, (IY+1)` |
| `fd cb 01 84` | `LD H, RES 0, (IY+1)` |
| `fd cb 01 85` | `LD L, RES 0, (IY+1)` |
| `fd cb 01 86` | `LD (IY+1), RES 0, (IY+1)` |
| `fd cb 01 87` | `LD A, RES 0, (IY+1)` |
| `fd cb 01 88` | `LD B, RES 1, (IY+1)` |
| `fd cb 01 89` | `LD C, RES 1, (IY+1)` |
| `fd cb 01 8a` | `LD D, RES 1, (IY+1)` |
| `fd cb 01 8b` | `LD E, RES 1, (IY+1)` |
| `fd cb 01 8c` | `LD H, RES 1, (IY+1)` |
| `fd cb 01 8d` | `LD L, RES 1, (IY+1)` |
| `fd cb 01 8e` | `LD (IY+1), RES 1, (IY+1)` |
| `fd cb 01 8f` | `LD A, RES 1, (IY+1)` |
| `fd cb 01 90` | `LD B, RES 2, (IY+1)` |
| `fd cb 01 91` | `LD C, RES 2, (IY+1)` |
| `fd cb 01 92` | `LD D, RES 2, (IY+1)` |
| `fd cb 01 93` | `LD E, RES 2, (IY+1)` |
| `fd cb 01 94` | `LD H, RES 2, (IY+1)` |
| `fd cb 01 95` | `LD L, RES 2, (IY+1)` |
| `fd cb 01 96` | `LD (IY+1), RES 2, (IY+1)` |
| `fd cb 01 97` | `LD A, RES 2, (IY+1)` |
| `fd cb 01 98` | `LD B, RES 3, (IY+1)` |
| `fd cb 01 99` | `LD C, RES 3, (IY+1)` |
| `fd cb 01 9a` | `LD D, RES 3, (IY+1)` |
| `fd cb 01 9b` | `LD E, RES 3, (IY+1)` |
| `fd cb 01 9c` | `LD H, RES 3, (IY+1)` |
| `fd cb 01 9d` | `LD L, RES 3, (IY+1)` |
| `fd cb 01 9e` | `LD (IY+1), RES 3, (IY+1)` |
| `fd cb 01 9f` | `LD A, RES 3, (IY+1)` |
| `fd cb 01 a0` | `LD B, RES 4, (IY+1)` |
| `fd cb 01 a1` | `LD C, RES 4, (IY+1)` |
| `fd cb 01 a2` | `LD D, RES 4, (IY+1)` |
| `fd cb 01 a3` | `LD E, RES 4, (IY+1)` |
| `fd cb 01 a4` | `LD H, RES 4, (IY+1)` |
| `fd cb 01 a5` | `LD L, RES 4, (IY+1)` |
| `fd cb 01 a6` | `LD (IY+1), RES 4, (IY+1)` |
| `fd cb 01 a7` | `LD A, RES 4, (IY+1)` |
| `fd cb 01 a8` | `LD B, RES 5, (IY+1)` |
| `fd cb 01 a9` | `LD C, RES 5, (IY+1)` |
| `fd cb 01 aa` | `LD D, RES 5, (IY+1)` |
| `fd cb 01 ab` | `LD E, RES 5, (IY+1)` |
| `fd cb 01 ac` | `LD H, RES 5, (IY+1)` |
| `fd cb 01 ad` | `LD L, RES 5, (IY+1)` |
| `fd cb 01 ae` | `LD (IY+1), RES 5, (IY+1)` |
| `fd cb 01 af` | `LD A, RES 5, (IY+1)` |
| `fd cb 01 b0` | `LD B, RES 6, (IY+1)` |
| `fd cb 01 b1` | `LD C, RES 6, (IY+1)` |
| `fd cb 01 b2` | `LD D, RES 6, (IY+1)` |
| `fd cb 01 b3` | `LD E, RES 6, (IY+1)` |
| `fd cb 01 b4` | `LD H, RES 6, (IY+1)` |
| `fd cb 01 b5` | `LD L, RES 6, (IY+1)` |
| `fd cb 01 b6` | `LD (IY+1), RES 6, (IY+1)` |
| `fd cb 01 b7` | `LD A, RES 6, (IY+1)` |
| `fd cb 01 b8` | `LD B, RES 7, (IY+1)` |
| `fd cb 01 b9` | `LD C, RES 7, (IY+1)` |
| `fd cb 01 ba` | `LD D, RES 7, (IY+1)` |
| `fd cb 01 bb` | `LD E, RES 7, (IY+1)` |
| `fd cb 01 bc` | `LD H, RES 7, (IY+1)` |
| `fd cb 01 bd` | `LD L, RES 7, (IY+1)` |
| `fd cb 01 be` | `LD (IY+1), RES 7, (IY+1)` |
| `fd cb 01 bf` | `LD A, RES 7, (IY+1)` |
| `fd cb 01 c0` | `LD B, SET 0, (IY+1)` |
| `fd cb 01 c1` | `LD C, SET 0, (IY+1)` |
| `fd cb 01 c2` | `LD D, SET 0, (IY+1)` |
| `fd cb 01 c3` | `LD E, SET 0, (IY+1)` |
| `fd cb 01 c4` | `LD H, SET 0, (IY+1)` |
| `fd cb 01 c5` | `LD L, SET 0, (IY+1)` |
| `fd cb 01 c6` | `LD (IY+1), SET 0, (IY+1)` |
| `fd cb 01 c7` | `LD A, SET 0, (IY+1)` |
| `fd cb 01 c8` | `LD B, SET 1, (IY+1)` |
| `fd cb 01 c9` | `LD C, SET 1, (IY+1)` |
| `fd cb 01 ca` | `LD D, SET 1, (IY+1)` |
| `fd cb 01 cb` | `LD E, SET 1, (IY+1)` |
| `fd cb 01 cc` | `LD H, SET 1, (IY+1)` |
| `fd cb 01 cd` | `LD L, SET 1, (IY+1)` |
| `fd cb 01 ce` | `LD (IY+1), SET 1, (IY+1)` |
| `fd cb 01 cf` | `LD A, SET 1, (IY+1)` |
| `fd cb 01 d0` | `LD B, SET 2, (IY+1)` |
| `fd cb 01 d1` | `LD C, SET 2, (IY+1)` |
| `fd cb 01 d2` | `LD D, SET 2, (IY+1)` |
| `fd cb 01 d3` | `LD E, SET 2, (IY+1)` |
| `fd cb 01 d4` | `LD H, SET 2, (IY+1)` |
| `fd cb 01 d5` | `LD L, SET 2, (IY+1)` |
| `fd cb 01 d6` | `LD (IY+1), SET 2, (IY+1)` |
| `fd cb 01 d7` | `LD A, SET 2, (IY+1)` |
| `fd cb 01 d8` | `LD B, SET 3, (IY+1)` |
| `fd cb 01 d9` | `LD C, SET 3, (IY+1)` |
| `fd cb 01 da` | `LD D, SET 3, (IY+1)` |
| `fd cb 01 db` | `LD E, SET 3, (IY+1)` |
| `fd cb 01 dc` | `LD H, SET 3, (IY+1)` |
| `fd cb 01 dd` | `LD L, SET 3, (IY+1)` |
| `fd cb 01 de` | `LD (IY+1), SET 3, (IY+1)` |
| `fd cb 01 df` | `LD A, SET 3, (IY+1)` |
| `fd cb 01 e0` | `LD B, SET 4, (IY+1)` |
| `fd cb 01 e1` | `LD C, SET 4, (IY+1)` |
| `fd cb 01 e2` | `LD D, SET 4, (IY+1)` |
| `fd cb 01 e3` | `LD E, SET 4, (IY+1)` |
| `fd cb 01 e4` | `LD H, SET 4, (IY+1)` |
| `fd cb 01 e5` | `LD L, SET 4, (IY+1)` |
| `fd cb 01 e6` | `LD (IY+1), SET 4, (IY+1)` |
| `fd cb 01 e7` | `LD A, SET 4, (IY+1)` |
| `fd cb 01 e8` | `LD B, SET 5, (IY+1)` |
| `fd cb 01 e9` | `LD C, SET 5, (IY+1)` |
| `fd cb 01 ea` | `LD D, SET 5, (IY+1)` |
| `fd cb 01 eb` | `LD E, SET 5, (IY+1)` |
| `fd cb 01 ec` | `LD H, SET 5, (IY+1)` |
| `fd cb 01 ed` | `LD L, SET 5, (IY+1)` |
| `fd cb 01 ee` | `LD (IY+1), SET 5, (IY+1)` |
| `fd cb 01 ef` | `LD A, SET 5, (IY+1)` |
| `fd cb 01 f0` | `LD B, SET 6, (IY+1)` |
| `fd cb 01 f1` | `LD C, SET 6, (IY+1)` |
| `fd cb 01 f2` | `LD D, SET 6, (IY+1)` |
| `fd cb 01 f3` | `LD E, SET 6, (IY+1)` |
| `fd cb 01 f4` | `LD H, SET 6, (IY+1)` |
| `fd cb 01 f5` | `LD L, SET 6, (IY+1)` |
| `fd cb 01 f6` | `LD (IY+1), SET 6, (IY+1)` |
| `fd cb 01 f7` | `LD A, SET 6, (IY+1)` |
| `fd cb 01 f8` | `LD B, SET 7, (IY+1)` |
| `fd cb 01 f9` | `LD C, SET 7, (IY+1)` |
| `fd cb 01 fa` | `LD D, SET 7, (IY+1)` |
| `fd cb 01 fb` | `LD E, SET 7, (IY+1)` |
| `fd cb 01 fc` | `LD H, SET 7, (IY+1)` |
| `fd cb 01 fd` | `LD L, SET 7, (IY+1)` |
| `fd cb 01 fe` | `LD (IY+1), SET 7, (IY+1)` |
| `fd cb 01 ff` | `LD A, SET 7, (IY+1)` |

