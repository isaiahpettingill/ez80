# Zilog eZ80 Opcodes

Generated from the emulator disassembler for `CpuMode::EZ80`.
The tables list every opcode form decoded by this mode. `NONINOP` is the emulator's non-instruction NOP behavior for unsupported ED-prefixed Z80-family opcodes.

## Base opcodes

| Bytes | Disassembly |
| --- | --- |
| `00` | `NOP` |
| `01 34 12 56` | `LD BC, $561234` |
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
| `11 34 12 56` | `LD DE, $561234` |
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
| `21 34 12 56` | `LD HL, $561234` |
| `22 34 12 56` | `LD ($561234), HL` |
| `23` | `INC HL` |
| `24` | `INC H` |
| `25` | `DEC H` |
| `26 34` | `LD H, $34` |
| `27` | `DAA` |
| `28 34` | `JR Z, $36` |
| `29` | `ADD HL, HL` |
| `2a 34 12 56` | `LD HL, ($561234)` |
| `2b` | `DEC HL` |
| `2c` | `INC L` |
| `2d` | `DEC L` |
| `2e 34` | `LD L, $34` |
| `2f` | `CPL` |
| `30 34` | `JR NC, $36` |
| `31 34 12 56` | `LD SP, $561234` |
| `32 34 12 56` | `LD ($561234), A` |
| `33` | `INC SP` |
| `34` | `INC (HL)` |
| `35` | `DEC (HL)` |
| `36 34` | `LD (HL), $34` |
| `37` | `SCF` |
| `38 34` | `JR C, $36` |
| `39` | `ADD HL, SP` |
| `3a 34 12 56` | `LD A, ($561234)` |
| `3b` | `DEC SP` |
| `3c` | `INC A` |
| `3d` | `DEC A` |
| `3e 34` | `LD A, $34` |
| `3f` | `CCF` |
| `40 34` | `INC.SIS (HL)` |
| `41` | `LD B, C` |
| `42` | `LD B, D` |
| `43` | `LD B, E` |
| `44` | `LD B, H` |
| `45` | `LD B, L` |
| `46` | `LD B, (HL)` |
| `47` | `LD B, A` |
| `48` | `LD C, B` |
| `49 34` | `INC.LIS (HL)` |
| `4a` | `LD C, D` |
| `4b` | `LD C, E` |
| `4c` | `LD C, H` |
| `4d` | `LD C, L` |
| `4e` | `LD C, (HL)` |
| `4f` | `LD C, A` |
| `50` | `LD D, B` |
| `51` | `LD D, C` |
| `52 34` | `INC.SIL (HL)` |
| `53` | `LD D, E` |
| `54` | `LD D, H` |
| `55` | `LD D, L` |
| `56` | `LD D, (HL)` |
| `57` | `LD D, A` |
| `58` | `LD E, B` |
| `59` | `LD E, C` |
| `5a` | `LD E, D` |
| `5b 34` | `INC.LIL (HL)` |
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
| `c2 34 12 56` | `JP NZ, $561234` |
| `c3 34 12 56` | `JP $561234` |
| `c4 34 12 56` | `CALL NZ, $561234` |
| `c5` | `PUSH BC` |
| `c6 34` | `ADD A, $34` |
| `c7` | `RST 00h` |
| `c8` | `RET Z` |
| `c9` | `RET` |
| `ca 34 12 56` | `JP Z, $561234` |
| `cb 34` | `SLL H` |
| `cc 34 12 56` | `CALL Z, $561234` |
| `cd 34 12 56` | `CALL $561234` |
| `ce 34` | `ADC A, $34` |
| `cf` | `RST 08h` |
| `d0` | `RET NC` |
| `d1` | `POP DE` |
| `d2 34 12 56` | `JP NC, $561234` |
| `d3 34` | `OUT ($34), A` |
| `d4 34 12 56` | `CALL NC, $561234` |
| `d5` | `PUSH DE` |
| `d6 34` | `SUB A, $34` |
| `d7` | `RST 10h` |
| `d8` | `RET C` |
| `d9` | `EXX` |
| `da 34 12 56` | `JP C, $561234` |
| `db 34` | `IN A, ($34)` |
| `dc 34 12 56` | `CALL C, $561234` |
| `dd 34 12` | `INC (IX+18)` |
| `de 34` | `SBC A, $34` |
| `df` | `RST 18h` |
| `e0` | `RET PO` |
| `e1` | `POP HL` |
| `e2 34 12 56` | `JP PO, $561234` |
| `e3` | `EX (SP), HL` |
| `e4 34 12 56` | `CALL PO, $561234` |
| `e5` | `PUSH HL` |
| `e6 34` | `AND A, $34` |
| `e7` | `RST 20h` |
| `e8` | `RET PE` |
| `e9` | `JP (HL)` |
| `ea 34 12 56` | `JP PE, $561234` |
| `eb` | `EX DE, HL` |
| `ec 34 12 56` | `CALL PE, $561234` |
| `ed 34` | `TST A, (HL)` |
| `ee 34` | `XOR A, $34` |
| `ef` | `RST 28h` |
| `f0` | `RET P` |
| `f1` | `POP AF` |
| `f2 34 12 56` | `JP P, $561234` |
| `f3` | `DI` |
| `f4 34 12 56` | `CALL P, $561234` |
| `f5` | `PUSH AF` |
| `f6 34` | `OR A, $34` |
| `f7` | `RST 30h` |
| `f8` | `RET M` |
| `f9` | `LD SP, HL` |
| `fa 34 12 56` | `JP M, $561234` |
| `fb` | `EI` |
| `fc 34 12 56` | `CALL M, $561234` |
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
| `ed 00 34` | `IN0 B, ($34)` |
| `ed 01 34` | `OUT0 ($34), B` |
| `ed 02 34` | `LEA BC, IX+$34` |
| `ed 03 34` | `LEA BC, IY+$34` |
| `ed 04` | `TST A, B` |
| `ed 05` | `NONINOP` |
| `ed 06` | `NONINOP` |
| `ed 07` | `LD BC, (HL)` |
| `ed 08 34` | `IN0 C, ($34)` |
| `ed 09 34` | `OUT0 ($34), C` |
| `ed 0a 34` | `LEA BC, IX+$34` |
| `ed 0b 34` | `LEA BC, IY+$34` |
| `ed 0c` | `TST A, C` |
| `ed 0d` | `NONINOP` |
| `ed 0e` | `NONINOP` |
| `ed 0f` | `LD (HL), BC` |
| `ed 10 34` | `IN0 D, ($34)` |
| `ed 11 34` | `OUT0 ($34), D` |
| `ed 12 34` | `LEA DE, IX+$34` |
| `ed 13 34` | `LEA DE, IY+$34` |
| `ed 14` | `TST A, D` |
| `ed 15` | `NONINOP` |
| `ed 16` | `NONINOP` |
| `ed 17` | `LD DE, (HL)` |
| `ed 18 34` | `IN0 E, ($34)` |
| `ed 19 34` | `OUT0 ($34), E` |
| `ed 1a 34` | `LEA DE, IX+$34` |
| `ed 1b 34` | `LEA DE, IY+$34` |
| `ed 1c` | `TST A, E` |
| `ed 1d` | `NONINOP` |
| `ed 1e` | `NONINOP` |
| `ed 1f` | `LD (HL), DE` |
| `ed 20 34` | `IN0 H, ($34)` |
| `ed 21 34` | `OUT0 ($34), H` |
| `ed 22 34` | `LEA HL, IX+$34` |
| `ed 23 34` | `LEA HL, IY+$34` |
| `ed 24` | `TST A, H` |
| `ed 25` | `NONINOP` |
| `ed 26` | `NONINOP` |
| `ed 27` | `LD HL, (HL)` |
| `ed 28 34` | `IN0 L, ($34)` |
| `ed 29 34` | `OUT0 ($34), L` |
| `ed 2a 34` | `LEA HL, IX+$34` |
| `ed 2b 34` | `LEA HL, IY+$34` |
| `ed 2c` | `TST A, L` |
| `ed 2d` | `NONINOP` |
| `ed 2e` | `NONINOP` |
| `ed 2f` | `LD (HL), HL` |
| `ed 30` | `NONINOP` |
| `ed 31` | `LD IY, (HL)` |
| `ed 32 34` | `LEA IX, IX+$34` |
| `ed 33 34` | `LEA IY, IY+$34` |
| `ed 34` | `TST A, (HL)` |
| `ed 35` | `NONINOP` |
| `ed 36` | `NONINOP` |
| `ed 37` | `LD IX, (HL)` |
| `ed 38 34` | `IN0 A, ($34)` |
| `ed 39 34` | `OUT0 ($34), A` |
| `ed 3a 34` | `LEA IX, IX+$34` |
| `ed 3b 34` | `LEA IY, IY+$34` |
| `ed 3c` | `TST A, A` |
| `ed 3d` | `NONINOP` |
| `ed 3e` | `LD (HL), IY` |
| `ed 3f` | `LD (HL), IX` |
| `ed 40` | `IN B, (C)` |
| `ed 41` | `OUT (C), B` |
| `ed 42` | `SBC HL, BC` |
| `ed 43 34 12 56` | `LD ($561234), BC` |
| `ed 44` | `NEG` |
| `ed 45` | `RETN` |
| `ed 46` | `IM 0` |
| `ed 47` | `LD I, A` |
| `ed 48` | `IN C, (C)` |
| `ed 49` | `OUT (C), C` |
| `ed 4a` | `ADC HL, BC` |
| `ed 4b 34 12 56` | `LD BC, ($561234)` |
| `ed 4c` | `MLT BC` |
| `ed 4d` | `RETI` |
| `ed 4e` | `IM 0` |
| `ed 4f` | `LD R, A` |
| `ed 50` | `IN D, (C)` |
| `ed 51` | `OUT (C), D` |
| `ed 52` | `SBC HL, DE` |
| `ed 53 34 12 56` | `LD ($561234), DE` |
| `ed 54 34` | `LEA IX, IY+$34` |
| `ed 55 34` | `LEA IY, IX+$34` |
| `ed 56` | `IM 1` |
| `ed 57` | `LD A, I` |
| `ed 58` | `IN E, (C)` |
| `ed 59` | `OUT (C), E` |
| `ed 5a` | `ADC HL, DE` |
| `ed 5b 34 12 56` | `LD DE, ($561234)` |
| `ed 5c` | `MLT DE` |
| `ed 5d` | `RETN` |
| `ed 5e` | `IM 2` |
| `ed 5f` | `LD A, R` |
| `ed 60` | `IN H, (C)` |
| `ed 61` | `OUT (C), H` |
| `ed 62` | `SBC HL, HL` |
| `ed 63 34 12 56` | `LD ($561234), HL` |
| `ed 64 34` | `TST A, $34` |
| `ed 65 34` | `PEA IX+$34` |
| `ed 66 34` | `PEA IY+$34` |
| `ed 67` | `RRD` |
| `ed 68` | `IN L, (C)` |
| `ed 69` | `OUT (C), L` |
| `ed 6a` | `ADC HL, HL` |
| `ed 6b 34 12 56` | `LD HL, ($561234)` |
| `ed 6c` | `MLT HL` |
| `ed 6d` | `LD MB, A` |
| `ed 6e` | `LD A, MB` |
| `ed 6f` | `RLD` |
| `ed 70` | `IN (C)` |
| `ed 71` | `OUT (C), 0` |
| `ed 72` | `SBC HL, SP` |
| `ed 73 34 12 56` | `LD ($561234), SP` |
| `ed 74 34` | `TSTIO $34` |
| `ed 75` | `RETN` |
| `ed 76` | `SLP` |
| `ed 77` | `NOP` |
| `ed 78` | `IN A, (C)` |
| `ed 79` | `OUT (C), A` |
| `ed 7a` | `ADC HL, SP` |
| `ed 7b 34 12 56` | `LD SP, ($561234)` |
| `ed 7c` | `MLT SP` |
| `ed 7d` | `STMIX` |
| `ed 7e` | `RSMIX` |
| `ed 7f` | `NOP` |
| `ed 80` | `NONINOP` |
| `ed 81` | `NONINOP` |
| `ed 82` | `NONINOP` |
| `ed 83` | `OTIM` |
| `ed 84` | `INI2` |
| `ed 85` | `NONINOP` |
| `ed 86` | `NONINOP` |
| `ed 87` | `NONINOP` |
| `ed 88` | `NONINOP` |
| `ed 89` | `NONINOP` |
| `ed 8a` | `NONINOP` |
| `ed 8b` | `OTDM` |
| `ed 8c` | `IND2` |
| `ed 8d` | `NONINOP` |
| `ed 8e` | `NONINOP` |
| `ed 8f` | `NONINOP` |
| `ed 90` | `NONINOP` |
| `ed 91` | `NONINOP` |
| `ed 92` | `NONINOP` |
| `ed 93` | `OTIMR` |
| `ed 94` | `INI2R` |
| `ed 95` | `NONINOP` |
| `ed 96` | `NONINOP` |
| `ed 97` | `NONINOP` |
| `ed 98` | `NONINOP` |
| `ed 99` | `NONINOP` |
| `ed 9a` | `NONINOP` |
| `ed 9b` | `OTDMR` |
| `ed 9c` | `IND2R` |
| `ed 9d` | `NONINOP` |
| `ed 9e` | `NONINOP` |
| `ed 9f` | `NONINOP` |
| `ed a0` | `LDI` |
| `ed a1` | `CPI` |
| `ed a2` | `INI` |
| `ed a3` | `OUTI` |
| `ed a4` | `OUTI2` |
| `ed a5` | `NONINOP` |
| `ed a6` | `NONINOP` |
| `ed a7` | `NONINOP` |
| `ed a8` | `LDD` |
| `ed a9` | `CPD` |
| `ed aa` | `IND` |
| `ed ab` | `OUTD` |
| `ed ac` | `OUTD2` |
| `ed ad` | `NONINOP` |
| `ed ae` | `NONINOP` |
| `ed af` | `NONINOP` |
| `ed b0` | `LDIR` |
| `ed b1` | `CPIR` |
| `ed b2` | `INIR` |
| `ed b3` | `OTIR` |
| `ed b4` | `OTI2R` |
| `ed b5` | `NONINOP` |
| `ed b6` | `NONINOP` |
| `ed b7` | `NONINOP` |
| `ed b8` | `LDDR` |
| `ed b9` | `CPDR` |
| `ed ba` | `INDR` |
| `ed bb` | `OTDR` |
| `ed bc` | `OTD2R` |
| `ed bd` | `NONINOP` |
| `ed be` | `NONINOP` |
| `ed bf` | `NONINOP` |
| `ed c0` | `NONINOP` |
| `ed c1` | `NONINOP` |
| `ed c2` | `INIRX` |
| `ed c3` | `OTIRX` |
| `ed c4` | `NONINOP` |
| `ed c5` | `NONINOP` |
| `ed c6` | `NONINOP` |
| `ed c7` | `LD I, HL` |
| `ed c8` | `NONINOP` |
| `ed c9` | `NONINOP` |
| `ed ca` | `INDRX` |
| `ed cb` | `OTDRX` |
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
| `ed d7` | `LD HL, I` |
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
| `ed fe` | `ILLEGAL` |
| `ed ff` | `NONINOP` |

## DD-prefixed IX opcodes

| Bytes | Disassembly |
| --- | --- |
| `dd 00` | `NOP` |
| `dd 01 34 12 56` | `LD BC, $561234` |
| `dd 02` | `LD (BC), A` |
| `dd 03` | `INC BC` |
| `dd 04` | `INC B` |
| `dd 05` | `DEC B` |
| `dd 06 34` | `LD B, $34` |
| `dd 07 34` | `LD BC, (IX+$34)` |
| `dd 08` | `EX AF, AF'` |
| `dd 09` | `ADD IX, BC` |
| `dd 0a` | `LD A, (BC)` |
| `dd 0b` | `DEC BC` |
| `dd 0c` | `INC C` |
| `dd 0d` | `DEC C` |
| `dd 0e 34` | `LD C, $34` |
| `dd 0f 34` | `LD (IX+$34), BC` |
| `dd 10 34` | `DJNZ $37` |
| `dd 11 34 12 56` | `LD DE, $561234` |
| `dd 12` | `LD (DE), A` |
| `dd 13` | `INC DE` |
| `dd 14` | `INC D` |
| `dd 15` | `DEC D` |
| `dd 16 34` | `LD D, $34` |
| `dd 17 34` | `LD DE, (IX+$34)` |
| `dd 18 34` | `JR $37` |
| `dd 19` | `ADD IX, DE` |
| `dd 1a` | `LD A, (DE)` |
| `dd 1b` | `DEC DE` |
| `dd 1c` | `INC E` |
| `dd 1d` | `DEC E` |
| `dd 1e 34` | `LD E, $34` |
| `dd 1f 34` | `LD (IX+$34), DE` |
| `dd 20 34` | `JR NZ, $37` |
| `dd 21 34 12 56` | `LD IX, $561234` |
| `dd 22 34 12 56` | `LD ($561234), IX` |
| `dd 23` | `INC IX` |
| `dd 24` | `INC H` |
| `dd 25` | `DEC H` |
| `dd 26 34` | `LD H, $34` |
| `dd 27 34` | `LD HL, (IX+$34)` |
| `dd 28 34` | `JR Z, $37` |
| `dd 29` | `ADD IX, IX` |
| `dd 2a 34 12 56` | `LD IX, ($561234)` |
| `dd 2b` | `DEC IX` |
| `dd 2c` | `INC L` |
| `dd 2d` | `DEC L` |
| `dd 2e 34` | `LD L, $34` |
| `dd 2f 34` | `LD (IX+$34), HL` |
| `dd 30 34` | `JR NC, $37` |
| `dd 31 34` | `LD IY, (IX+$34)` |
| `dd 32 34 12 56` | `LD ($561234), A` |
| `dd 33` | `INC SP` |
| `dd 34 34` | `INC (IX+52)` |
| `dd 35 34` | `DEC (IX+52)` |
| `dd 36 34 12` | `LD (IX+52), $12` |
| `dd 37 34` | `LD IX, (IX+$34)` |
| `dd 38 34` | `JR C, $37` |
| `dd 39` | `ADD IX, SP` |
| `dd 3a 34 12 56` | `LD A, ($561234)` |
| `dd 3b` | `DEC SP` |
| `dd 3c` | `INC A` |
| `dd 3d` | `DEC A` |
| `dd 3e 34` | `LD (IX+$34), IY` |
| `dd 3f 34` | `LD (IX+$34), IX` |
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
| `dd 86 34` | `ADD A, (IX+$34)` |
| `dd 87` | `ADD A, A` |
| `dd 88` | `ADC A, B` |
| `dd 89` | `ADC A, C` |
| `dd 8a` | `ADC A, D` |
| `dd 8b` | `ADC A, E` |
| `dd 8c` | `ADC A, H` |
| `dd 8d` | `ADC A, L` |
| `dd 8e 34` | `ADC A, (IX+$34)` |
| `dd 8f` | `ADC A, A` |
| `dd 90` | `SUB A, B` |
| `dd 91` | `SUB A, C` |
| `dd 92` | `SUB A, D` |
| `dd 93` | `SUB A, E` |
| `dd 94` | `SUB A, H` |
| `dd 95` | `SUB A, L` |
| `dd 96 34` | `SUB A, (IX+$34)` |
| `dd 97` | `SUB A, A` |
| `dd 98` | `SBC A, B` |
| `dd 99` | `SBC A, C` |
| `dd 9a` | `SBC A, D` |
| `dd 9b` | `SBC A, E` |
| `dd 9c` | `SBC A, H` |
| `dd 9d` | `SBC A, L` |
| `dd 9e 34` | `SBC A, (IX+$34)` |
| `dd 9f` | `SBC A, A` |
| `dd a0` | `AND A, B` |
| `dd a1` | `AND A, C` |
| `dd a2` | `AND A, D` |
| `dd a3` | `AND A, E` |
| `dd a4` | `AND A, H` |
| `dd a5` | `AND A, L` |
| `dd a6 34` | `AND A, (IX+$34)` |
| `dd a7` | `AND A, A` |
| `dd a8` | `XOR A, B` |
| `dd a9` | `XOR A, C` |
| `dd aa` | `XOR A, D` |
| `dd ab` | `XOR A, E` |
| `dd ac` | `XOR A, H` |
| `dd ad` | `XOR A, L` |
| `dd ae 34` | `XOR A, (IX+$34)` |
| `dd af` | `XOR A, A` |
| `dd b0` | `OR A, B` |
| `dd b1` | `OR A, C` |
| `dd b2` | `OR A, D` |
| `dd b3` | `OR A, E` |
| `dd b4` | `OR A, H` |
| `dd b5` | `OR A, L` |
| `dd b6 34` | `OR A, (IX+$34)` |
| `dd b7` | `OR A, A` |
| `dd b8` | `CP A, B` |
| `dd b9` | `CP A, C` |
| `dd ba` | `CP A, D` |
| `dd bb` | `CP A, E` |
| `dd bc` | `CP A, H` |
| `dd bd` | `CP A, L` |
| `dd be 34` | `CP A, (IX+$34)` |
| `dd bf` | `CP A, A` |
| `dd c0` | `RET NZ` |
| `dd c1` | `POP BC` |
| `dd c2 34 12 56` | `JP NZ, $561234` |
| `dd c3 34 12 56` | `JP $561234` |
| `dd c4 34 12 56` | `CALL NZ, $561234` |
| `dd c5` | `PUSH BC` |
| `dd c6 34` | `ADD A, $34` |
| `dd c7` | `RST 00h` |
| `dd c8` | `RET Z` |
| `dd c9` | `RET` |
| `dd ca 34 12 56` | `JP Z, $561234` |
| `dd cb 34 12` | `RL D, (IX+52)` |
| `dd cc 34 12 56` | `CALL Z, $561234` |
| `dd cd 34 12 56` | `CALL $561234` |
| `dd ce 34` | `ADC A, $34` |
| `dd cf` | `RST 08h` |
| `dd d0` | `RET NC` |
| `dd d1` | `POP DE` |
| `dd d2 34 12 56` | `JP NC, $561234` |
| `dd d3 34` | `OUT ($34), A` |
| `dd d4 34 12 56` | `CALL NC, $561234` |
| `dd d5` | `PUSH DE` |
| `dd d6 34` | `SUB A, $34` |
| `dd d7` | `RST 10h` |
| `dd d8` | `RET C` |
| `dd d9` | `EXX` |
| `dd da 34 12 56` | `JP C, $561234` |
| `dd db 34` | `IN A, ($34)` |
| `dd dc 34 12 56` | `CALL C, $561234` |
| `dd dd 34 12` | `INC (IX+18)` |
| `dd de 34` | `SBC A, $34` |
| `dd df` | `RST 18h` |
| `dd e0` | `RET PO` |
| `dd e1` | `POP IX` |
| `dd e2 34 12 56` | `JP PO, $561234` |
| `dd e3` | `EX (SP), IX` |
| `dd e4 34 12 56` | `CALL PO, $561234` |
| `dd e5` | `PUSH IX` |
| `dd e6 34` | `AND A, $34` |
| `dd e7` | `RST 20h` |
| `dd e8` | `RET PE` |
| `dd e9` | `JP (IX)` |
| `dd ea 34 12 56` | `JP PE, $561234` |
| `dd eb` | `EX DE, IX` |
| `dd ec 34 12 56` | `CALL PE, $561234` |
| `dd ed 34` | `TST A, (HL)` |
| `dd ee 34` | `XOR A, $34` |
| `dd ef` | `RST 28h` |
| `dd f0` | `RET P` |
| `dd f1` | `POP AF` |
| `dd f2 34 12 56` | `JP P, $561234` |
| `dd f3` | `DI` |
| `dd f4 34 12 56` | `CALL P, $561234` |
| `dd f5` | `PUSH AF` |
| `dd f6 34` | `OR A, $34` |
| `dd f7` | `RST 30h` |
| `dd f8` | `RET M` |
| `dd f9` | `LD SP, IX` |
| `dd fa 34 12 56` | `JP M, $561234` |
| `dd fb` | `EI` |
| `dd fc 34 12 56` | `CALL M, $561234` |
| `dd fd 34 12` | `INC (IY+18)` |
| `dd fe 34` | `CP A, $34` |
| `dd ff` | `RST 38h` |

## FD-prefixed IY opcodes

| Bytes | Disassembly |
| --- | --- |
| `fd 00` | `NOP` |
| `fd 01 34 12 56` | `LD BC, $561234` |
| `fd 02` | `LD (BC), A` |
| `fd 03` | `INC BC` |
| `fd 04` | `INC B` |
| `fd 05` | `DEC B` |
| `fd 06 34` | `LD B, $34` |
| `fd 07 34` | `LD BC, (IY+$34)` |
| `fd 08` | `EX AF, AF'` |
| `fd 09` | `ADD IY, BC` |
| `fd 0a` | `LD A, (BC)` |
| `fd 0b` | `DEC BC` |
| `fd 0c` | `INC C` |
| `fd 0d` | `DEC C` |
| `fd 0e 34` | `LD C, $34` |
| `fd 0f 34` | `LD (IY+$34), BC` |
| `fd 10 34` | `DJNZ $37` |
| `fd 11 34 12 56` | `LD DE, $561234` |
| `fd 12` | `LD (DE), A` |
| `fd 13` | `INC DE` |
| `fd 14` | `INC D` |
| `fd 15` | `DEC D` |
| `fd 16 34` | `LD D, $34` |
| `fd 17 34` | `LD DE, (IY+$34)` |
| `fd 18 34` | `JR $37` |
| `fd 19` | `ADD IY, DE` |
| `fd 1a` | `LD A, (DE)` |
| `fd 1b` | `DEC DE` |
| `fd 1c` | `INC E` |
| `fd 1d` | `DEC E` |
| `fd 1e 34` | `LD E, $34` |
| `fd 1f 34` | `LD (IY+$34), DE` |
| `fd 20 34` | `JR NZ, $37` |
| `fd 21 34 12 56` | `LD IY, $561234` |
| `fd 22 34 12 56` | `LD ($561234), IY` |
| `fd 23` | `INC IY` |
| `fd 24` | `INC H` |
| `fd 25` | `DEC H` |
| `fd 26 34` | `LD H, $34` |
| `fd 27 34` | `LD HL, (IY+$34)` |
| `fd 28 34` | `JR Z, $37` |
| `fd 29` | `ADD IY, IY` |
| `fd 2a 34 12 56` | `LD IY, ($561234)` |
| `fd 2b` | `DEC IY` |
| `fd 2c` | `INC L` |
| `fd 2d` | `DEC L` |
| `fd 2e 34` | `LD L, $34` |
| `fd 2f 34` | `LD (IY+$34), HL` |
| `fd 30 34` | `JR NC, $37` |
| `fd 31 34` | `LD IX, (IY+$34)` |
| `fd 32 34 12 56` | `LD ($561234), A` |
| `fd 33` | `INC SP` |
| `fd 34 34` | `INC (IY+52)` |
| `fd 35 34` | `DEC (IY+52)` |
| `fd 36 34 12` | `LD (IY+52), $12` |
| `fd 37 34` | `LD IY, (IY+$34)` |
| `fd 38 34` | `JR C, $37` |
| `fd 39` | `ADD IY, SP` |
| `fd 3a 34 12 56` | `LD A, ($561234)` |
| `fd 3b` | `DEC SP` |
| `fd 3c` | `INC A` |
| `fd 3d` | `DEC A` |
| `fd 3e 34` | `LD (IY+$34), IX` |
| `fd 3f 34` | `LD (IY+$34), IY` |
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
| `fd 86 34` | `ADD A, (IY+$34)` |
| `fd 87` | `ADD A, A` |
| `fd 88` | `ADC A, B` |
| `fd 89` | `ADC A, C` |
| `fd 8a` | `ADC A, D` |
| `fd 8b` | `ADC A, E` |
| `fd 8c` | `ADC A, H` |
| `fd 8d` | `ADC A, L` |
| `fd 8e 34` | `ADC A, (IY+$34)` |
| `fd 8f` | `ADC A, A` |
| `fd 90` | `SUB A, B` |
| `fd 91` | `SUB A, C` |
| `fd 92` | `SUB A, D` |
| `fd 93` | `SUB A, E` |
| `fd 94` | `SUB A, H` |
| `fd 95` | `SUB A, L` |
| `fd 96 34` | `SUB A, (IY+$34)` |
| `fd 97` | `SUB A, A` |
| `fd 98` | `SBC A, B` |
| `fd 99` | `SBC A, C` |
| `fd 9a` | `SBC A, D` |
| `fd 9b` | `SBC A, E` |
| `fd 9c` | `SBC A, H` |
| `fd 9d` | `SBC A, L` |
| `fd 9e 34` | `SBC A, (IY+$34)` |
| `fd 9f` | `SBC A, A` |
| `fd a0` | `AND A, B` |
| `fd a1` | `AND A, C` |
| `fd a2` | `AND A, D` |
| `fd a3` | `AND A, E` |
| `fd a4` | `AND A, H` |
| `fd a5` | `AND A, L` |
| `fd a6 34` | `ADD A, (IY+$34)` |
| `fd a7` | `AND A, A` |
| `fd a8` | `XOR A, B` |
| `fd a9` | `XOR A, C` |
| `fd aa` | `XOR A, D` |
| `fd ab` | `XOR A, E` |
| `fd ac` | `XOR A, H` |
| `fd ad` | `XOR A, L` |
| `fd ae 34` | `XOR A, (IY+$34)` |
| `fd af` | `XOR A, A` |
| `fd b0` | `OR A, B` |
| `fd b1` | `OR A, C` |
| `fd b2` | `OR A, D` |
| `fd b3` | `OR A, E` |
| `fd b4` | `OR A, H` |
| `fd b5` | `OR A, L` |
| `fd b6 34` | `OR A, (IY+$34)` |
| `fd b7` | `OR A, A` |
| `fd b8` | `CP A, B` |
| `fd b9` | `CP A, C` |
| `fd ba` | `CP A, D` |
| `fd bb` | `CP A, E` |
| `fd bc` | `CP A, H` |
| `fd bd` | `CP A, L` |
| `fd be 34` | `CP A, (IY+$34)` |
| `fd bf` | `CP A, A` |
| `fd c0` | `RET NZ` |
| `fd c1` | `POP BC` |
| `fd c2 34 12 56` | `JP NZ, $561234` |
| `fd c3 34 12 56` | `JP $561234` |
| `fd c4 34 12 56` | `CALL NZ, $561234` |
| `fd c5` | `PUSH BC` |
| `fd c6 34` | `ADD A, $34` |
| `fd c7` | `RST 00h` |
| `fd c8` | `RET Z` |
| `fd c9` | `RET` |
| `fd ca 34 12 56` | `JP Z, $561234` |
| `fd cb 34 12` | `RL D, (IY+52)` |
| `fd cc 34 12 56` | `CALL Z, $561234` |
| `fd cd 34 12 56` | `CALL $561234` |
| `fd ce 34` | `ADC A, $34` |
| `fd cf` | `RST 08h` |
| `fd d0` | `RET NC` |
| `fd d1` | `POP DE` |
| `fd d2 34 12 56` | `JP NC, $561234` |
| `fd d3 34` | `OUT ($34), A` |
| `fd d4 34 12 56` | `CALL NC, $561234` |
| `fd d5` | `PUSH DE` |
| `fd d6 34` | `SUB A, $34` |
| `fd d7` | `RST 10h` |
| `fd d8` | `RET C` |
| `fd d9` | `EXX` |
| `fd da 34 12 56` | `JP C, $561234` |
| `fd db 34` | `IN A, ($34)` |
| `fd dc 34 12 56` | `CALL C, $561234` |
| `fd dd 34 12` | `INC (IX+18)` |
| `fd de 34` | `SBC A, $34` |
| `fd df` | `RST 18h` |
| `fd e0` | `RET PO` |
| `fd e1` | `POP IY` |
| `fd e2 34 12 56` | `JP PO, $561234` |
| `fd e3` | `EX (SP), IY` |
| `fd e4 34 12 56` | `CALL PO, $561234` |
| `fd e5` | `PUSH IY` |
| `fd e6 34` | `AND A, $34` |
| `fd e7` | `RST 20h` |
| `fd e8` | `RET PE` |
| `fd e9` | `JP (IY)` |
| `fd ea 34 12 56` | `JP PE, $561234` |
| `fd eb` | `EX DE, IY` |
| `fd ec 34 12 56` | `CALL PE, $561234` |
| `fd ed 34` | `TST A, (HL)` |
| `fd ee 34` | `XOR A, $34` |
| `fd ef` | `RST 28h` |
| `fd f0` | `RET P` |
| `fd f1` | `POP AF` |
| `fd f2 34 12 56` | `JP P, $561234` |
| `fd f3` | `DI` |
| `fd f4 34 12 56` | `CALL P, $561234` |
| `fd f5` | `PUSH AF` |
| `fd f6 34` | `OR A, $34` |
| `fd f7` | `RST 30h` |
| `fd f8` | `RET M` |
| `fd f9` | `LD SP, IY` |
| `fd fa 34 12 56` | `JP M, $561234` |
| `fd fb` | `EI` |
| `fd fc 34 12 56` | `CALL M, $561234` |
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

## eZ80 SIS base opcodes

| Bytes | Disassembly |
| --- | --- |
| `40 00` | `NOP.SIS` |
| `40 01 34 12` | `LD.SIS BC, $1234` |
| `40 02` | `LD.SIS (BC), A` |
| `40 03` | `INC.SIS BC` |
| `40 04` | `INC.SIS B` |
| `40 05` | `DEC.SIS B` |
| `40 06 34` | `LD.SIS B, $34` |
| `40 07` | `RLCA.SIS` |
| `40 08` | `EX.SIS AF, AF'` |
| `40 09` | `ADD.SIS HL, BC` |
| `40 0a` | `LD.SIS A, (BC)` |
| `40 0b` | `DEC.SIS BC` |
| `40 0c` | `INC.SIS C` |
| `40 0d` | `DEC.SIS C` |
| `40 0e 34` | `LD.SIS C, $34` |
| `40 0f` | `RRCA.SIS` |
| `40 10 34` | `DJNZ.SIS $37` |
| `40 11 34 12` | `LD.SIS DE, $1234` |
| `40 12` | `LD.SIS (DE), A` |
| `40 13` | `INC.SIS DE` |
| `40 14` | `INC.SIS D` |
| `40 15` | `DEC.SIS D` |
| `40 16 34` | `LD.SIS D, $34` |
| `40 17` | `RLA.SIS` |
| `40 18 34` | `JR.SIS $37` |
| `40 19` | `ADD.SIS HL, DE` |
| `40 1a` | `LD.SIS A, (DE)` |
| `40 1b` | `DEC.SIS DE` |
| `40 1c` | `INC.SIS E` |
| `40 1d` | `DEC.SIS E` |
| `40 1e 34` | `LD.SIS E, $34` |
| `40 1f` | `RRA.SIS` |
| `40 20 34` | `JR.SIS NZ, $37` |
| `40 21 34 12` | `LD.SIS HL, $1234` |
| `40 22 34 12` | `LD.SIS ($1234), HL` |
| `40 23` | `INC.SIS HL` |
| `40 24` | `INC.SIS H` |
| `40 25` | `DEC.SIS H` |
| `40 26 34` | `LD.SIS H, $34` |
| `40 27` | `DAA.SIS` |
| `40 28 34` | `JR.SIS Z, $37` |
| `40 29` | `ADD.SIS HL, HL` |
| `40 2a 34 12` | `LD.SIS HL, ($1234)` |
| `40 2b` | `DEC.SIS HL` |
| `40 2c` | `INC.SIS L` |
| `40 2d` | `DEC.SIS L` |
| `40 2e 34` | `LD.SIS L, $34` |
| `40 2f` | `CPL.SIS` |
| `40 30 34` | `JR.SIS NC, $37` |
| `40 31 34 12` | `LD.SIS SP, $1234` |
| `40 32 34 12` | `LD.SIS ($1234), A` |
| `40 33` | `INC.SIS SP` |
| `40 34` | `INC.SIS (HL)` |
| `40 35` | `DEC.SIS (HL)` |
| `40 36 34` | `LD.SIS (HL), $34` |
| `40 37` | `SCF.SIS` |
| `40 38 34` | `JR.SIS C, $37` |
| `40 39` | `ADD.SIS HL, SP` |
| `40 3a 34 12` | `LD.SIS A, ($1234)` |
| `40 3b` | `DEC.SIS SP` |
| `40 3c` | `INC.SIS A` |
| `40 3d` | `DEC.SIS A` |
| `40 3e 34` | `LD.SIS A, $34` |
| `40 3f` | `CCF.SIS` |
| `40 40 34` | `INC.SIS (HL)` |
| `40 41` | `LD.SIS B, C` |
| `40 42` | `LD.SIS B, D` |
| `40 43` | `LD.SIS B, E` |
| `40 44` | `LD.SIS B, H` |
| `40 45` | `LD.SIS B, L` |
| `40 46` | `LD.SIS B, (HL)` |
| `40 47` | `LD.SIS B, A` |
| `40 48` | `LD.SIS C, B` |
| `40 49 34` | `INC.LIS (HL)` |
| `40 4a` | `LD.SIS C, D` |
| `40 4b` | `LD.SIS C, E` |
| `40 4c` | `LD.SIS C, H` |
| `40 4d` | `LD.SIS C, L` |
| `40 4e` | `LD.SIS C, (HL)` |
| `40 4f` | `LD.SIS C, A` |
| `40 50` | `LD.SIS D, B` |
| `40 51` | `LD.SIS D, C` |
| `40 52 34` | `INC.SIL (HL)` |
| `40 53` | `LD.SIS D, E` |
| `40 54` | `LD.SIS D, H` |
| `40 55` | `LD.SIS D, L` |
| `40 56` | `LD.SIS D, (HL)` |
| `40 57` | `LD.SIS D, A` |
| `40 58` | `LD.SIS E, B` |
| `40 59` | `LD.SIS E, C` |
| `40 5a` | `LD.SIS E, D` |
| `40 5b 34` | `INC.LIL (HL)` |
| `40 5c` | `LD.SIS E, H` |
| `40 5d` | `LD.SIS E, L` |
| `40 5e` | `LD.SIS E, (HL)` |
| `40 5f` | `LD.SIS E, A` |
| `40 60` | `LD.SIS H, B` |
| `40 61` | `LD.SIS H, C` |
| `40 62` | `LD.SIS H, D` |
| `40 63` | `LD.SIS H, E` |
| `40 64` | `LD.SIS H, H` |
| `40 65` | `LD.SIS H, L` |
| `40 66` | `LD.SIS H, (HL)` |
| `40 67` | `LD.SIS H, A` |
| `40 68` | `LD.SIS L, B` |
| `40 69` | `LD.SIS L, C` |
| `40 6a` | `LD.SIS L, D` |
| `40 6b` | `LD.SIS L, E` |
| `40 6c` | `LD.SIS L, H` |
| `40 6d` | `LD.SIS L, L` |
| `40 6e` | `LD.SIS L, (HL)` |
| `40 6f` | `LD.SIS L, A` |
| `40 70` | `LD.SIS (HL), B` |
| `40 71` | `LD.SIS (HL), C` |
| `40 72` | `LD.SIS (HL), D` |
| `40 73` | `LD.SIS (HL), E` |
| `40 74` | `LD.SIS (HL), H` |
| `40 75` | `LD.SIS (HL), L` |
| `40 76` | `HALT.SIS` |
| `40 77` | `LD.SIS (HL), A` |
| `40 78` | `LD.SIS A, B` |
| `40 79` | `LD.SIS A, C` |
| `40 7a` | `LD.SIS A, D` |
| `40 7b` | `LD.SIS A, E` |
| `40 7c` | `LD.SIS A, H` |
| `40 7d` | `LD.SIS A, L` |
| `40 7e` | `LD.SIS A, (HL)` |
| `40 7f` | `LD.SIS A, A` |
| `40 80` | `ADD.SIS A, B` |
| `40 81` | `ADD.SIS A, C` |
| `40 82` | `ADD.SIS A, D` |
| `40 83` | `ADD.SIS A, E` |
| `40 84` | `ADD.SIS A, H` |
| `40 85` | `ADD.SIS A, L` |
| `40 86` | `ADD.SIS A, (HL)` |
| `40 87` | `ADD.SIS A, A` |
| `40 88` | `ADC.SIS A, B` |
| `40 89` | `ADC.SIS A, C` |
| `40 8a` | `ADC.SIS A, D` |
| `40 8b` | `ADC.SIS A, E` |
| `40 8c` | `ADC.SIS A, H` |
| `40 8d` | `ADC.SIS A, L` |
| `40 8e` | `ADC.SIS A, (HL)` |
| `40 8f` | `ADC.SIS A, A` |
| `40 90` | `SUB.SIS A, B` |
| `40 91` | `SUB.SIS A, C` |
| `40 92` | `SUB.SIS A, D` |
| `40 93` | `SUB.SIS A, E` |
| `40 94` | `SUB.SIS A, H` |
| `40 95` | `SUB.SIS A, L` |
| `40 96` | `SUB.SIS A, (HL)` |
| `40 97` | `SUB.SIS A, A` |
| `40 98` | `SBC.SIS A, B` |
| `40 99` | `SBC.SIS A, C` |
| `40 9a` | `SBC.SIS A, D` |
| `40 9b` | `SBC.SIS A, E` |
| `40 9c` | `SBC.SIS A, H` |
| `40 9d` | `SBC.SIS A, L` |
| `40 9e` | `SBC.SIS A, (HL)` |
| `40 9f` | `SBC.SIS A, A` |
| `40 a0` | `AND.SIS A, B` |
| `40 a1` | `AND.SIS A, C` |
| `40 a2` | `AND.SIS A, D` |
| `40 a3` | `AND.SIS A, E` |
| `40 a4` | `AND.SIS A, H` |
| `40 a5` | `AND.SIS A, L` |
| `40 a6` | `AND.SIS A, (HL)` |
| `40 a7` | `AND.SIS A, A` |
| `40 a8` | `XOR.SIS A, B` |
| `40 a9` | `XOR.SIS A, C` |
| `40 aa` | `XOR.SIS A, D` |
| `40 ab` | `XOR.SIS A, E` |
| `40 ac` | `XOR.SIS A, H` |
| `40 ad` | `XOR.SIS A, L` |
| `40 ae` | `XOR.SIS A, (HL)` |
| `40 af` | `XOR.SIS A, A` |
| `40 b0` | `OR.SIS A, B` |
| `40 b1` | `OR.SIS A, C` |
| `40 b2` | `OR.SIS A, D` |
| `40 b3` | `OR.SIS A, E` |
| `40 b4` | `OR.SIS A, H` |
| `40 b5` | `OR.SIS A, L` |
| `40 b6` | `OR.SIS A, (HL)` |
| `40 b7` | `OR.SIS A, A` |
| `40 b8` | `CP.SIS A, B` |
| `40 b9` | `CP.SIS A, C` |
| `40 ba` | `CP.SIS A, D` |
| `40 bb` | `CP.SIS A, E` |
| `40 bc` | `CP.SIS A, H` |
| `40 bd` | `CP.SIS A, L` |
| `40 be` | `CP.SIS A, (HL)` |
| `40 bf` | `CP.SIS A, A` |
| `40 c0` | `RET.SIS NZ` |
| `40 c1` | `POP.SIS BC` |
| `40 c2 34 12` | `JP.SIS NZ, $1234` |
| `40 c3 34 12` | `JP.SIS $1234` |
| `40 c4 34 12` | `CALL.SIS NZ, $1234` |
| `40 c5` | `PUSH.SIS BC` |
| `40 c6 34` | `ADD.SIS A, $34` |
| `40 c7` | `RST.SIS 00h` |
| `40 c8` | `RET.SIS Z` |
| `40 c9` | `RET.SIS` |
| `40 ca 34 12` | `JP.SIS Z, $1234` |
| `40 cb 34` | `SLL.SIS H` |
| `40 cc 34 12` | `CALL.SIS Z, $1234` |
| `40 cd 34 12` | `CALL.SIS $1234` |
| `40 ce 34` | `ADC.SIS A, $34` |
| `40 cf` | `RST.SIS 08h` |
| `40 d0` | `RET.SIS NC` |
| `40 d1` | `POP.SIS DE` |
| `40 d2 34 12` | `JP.SIS NC, $1234` |
| `40 d3 34` | `OUT.SIS ($34), A` |
| `40 d4 34 12` | `CALL.SIS NC, $1234` |
| `40 d5` | `PUSH.SIS DE` |
| `40 d6 34` | `SUB.SIS A, $34` |
| `40 d7` | `RST.SIS 10h` |
| `40 d8` | `RET.SIS C` |
| `40 d9` | `EXX.SIS` |
| `40 da 34 12` | `JP.SIS C, $1234` |
| `40 db 34` | `IN.SIS A, ($34)` |
| `40 dc 34 12` | `CALL.SIS C, $1234` |
| `40 dd 34 12` | `INC.SIS (IX+18)` |
| `40 de 34` | `SBC.SIS A, $34` |
| `40 df` | `RST.SIS 18h` |
| `40 e0` | `RET.SIS PO` |
| `40 e1` | `POP.SIS HL` |
| `40 e2 34 12` | `JP.SIS PO, $1234` |
| `40 e3` | `EX.SIS (SP), HL` |
| `40 e4 34 12` | `CALL.SIS PO, $1234` |
| `40 e5` | `PUSH.SIS HL` |
| `40 e6 34` | `AND.SIS A, $34` |
| `40 e7` | `RST.SIS 20h` |
| `40 e8` | `RET.SIS PE` |
| `40 e9` | `JP.SIS (HL)` |
| `40 ea 34 12` | `JP.SIS PE, $1234` |
| `40 eb` | `EX.SIS DE, HL` |
| `40 ec 34 12` | `CALL.SIS PE, $1234` |
| `40 ed 34` | `TST.SIS A, (HL)` |
| `40 ee 34` | `XOR.SIS A, $34` |
| `40 ef` | `RST.SIS 28h` |
| `40 f0` | `RET.SIS P` |
| `40 f1` | `POP.SIS AF` |
| `40 f2 34 12` | `JP.SIS P, $1234` |
| `40 f3` | `DI.SIS` |
| `40 f4 34 12` | `CALL.SIS P, $1234` |
| `40 f5` | `PUSH.SIS AF` |
| `40 f6 34` | `OR.SIS A, $34` |
| `40 f7` | `RST.SIS 30h` |
| `40 f8` | `RET.SIS M` |
| `40 f9` | `LD.SIS SP, HL` |
| `40 fa 34 12` | `JP.SIS M, $1234` |
| `40 fb` | `EI.SIS` |
| `40 fc 34 12` | `CALL.SIS M, $1234` |
| `40 fd 34 12` | `INC.SIS (IY+18)` |
| `40 fe 34` | `CP.SIS A, $34` |
| `40 ff` | `RST.SIS 38h` |

## eZ80 SIS ED-prefixed opcodes

| Bytes | Disassembly |
| --- | --- |
| `40 ed 00 34` | `IN0.SIS B, ($34)` |
| `40 ed 01 34` | `OUT0.SIS ($34), B` |
| `40 ed 02 34` | `LEA.SIS BC, IX+$34` |
| `40 ed 03 34` | `LEA.SIS BC, IY+$34` |
| `40 ed 04` | `TST.SIS A, B` |
| `40 ed 05` | `NONINOP.SIS` |
| `40 ed 06` | `NONINOP.SIS` |
| `40 ed 07` | `LD.SIS BC, (HL)` |
| `40 ed 08 34` | `IN0.SIS C, ($34)` |
| `40 ed 09 34` | `OUT0.SIS ($34), C` |
| `40 ed 0a 34` | `LEA.SIS BC, IX+$34` |
| `40 ed 0b 34` | `LEA.SIS BC, IY+$34` |
| `40 ed 0c` | `TST.SIS A, C` |
| `40 ed 0d` | `NONINOP.SIS` |
| `40 ed 0e` | `NONINOP.SIS` |
| `40 ed 0f` | `LD.SIS (HL), BC` |
| `40 ed 10 34` | `IN0.SIS D, ($34)` |
| `40 ed 11 34` | `OUT0.SIS ($34), D` |
| `40 ed 12 34` | `LEA.SIS DE, IX+$34` |
| `40 ed 13 34` | `LEA.SIS DE, IY+$34` |
| `40 ed 14` | `TST.SIS A, D` |
| `40 ed 15` | `NONINOP.SIS` |
| `40 ed 16` | `NONINOP.SIS` |
| `40 ed 17` | `LD.SIS DE, (HL)` |
| `40 ed 18 34` | `IN0.SIS E, ($34)` |
| `40 ed 19 34` | `OUT0.SIS ($34), E` |
| `40 ed 1a 34` | `LEA.SIS DE, IX+$34` |
| `40 ed 1b 34` | `LEA.SIS DE, IY+$34` |
| `40 ed 1c` | `TST.SIS A, E` |
| `40 ed 1d` | `NONINOP.SIS` |
| `40 ed 1e` | `NONINOP.SIS` |
| `40 ed 1f` | `LD.SIS (HL), DE` |
| `40 ed 20 34` | `IN0.SIS H, ($34)` |
| `40 ed 21 34` | `OUT0.SIS ($34), H` |
| `40 ed 22 34` | `LEA.SIS HL, IX+$34` |
| `40 ed 23 34` | `LEA.SIS HL, IY+$34` |
| `40 ed 24` | `TST.SIS A, H` |
| `40 ed 25` | `NONINOP.SIS` |
| `40 ed 26` | `NONINOP.SIS` |
| `40 ed 27` | `LD.SIS HL, (HL)` |
| `40 ed 28 34` | `IN0.SIS L, ($34)` |
| `40 ed 29 34` | `OUT0.SIS ($34), L` |
| `40 ed 2a 34` | `LEA.SIS HL, IX+$34` |
| `40 ed 2b 34` | `LEA.SIS HL, IY+$34` |
| `40 ed 2c` | `TST.SIS A, L` |
| `40 ed 2d` | `NONINOP.SIS` |
| `40 ed 2e` | `NONINOP.SIS` |
| `40 ed 2f` | `LD.SIS (HL), HL` |
| `40 ed 30` | `NONINOP.SIS` |
| `40 ed 31` | `LD.SIS IY, (HL)` |
| `40 ed 32 34` | `LEA.SIS IX, IX+$34` |
| `40 ed 33 34` | `LEA.SIS IY, IY+$34` |
| `40 ed 34` | `TST.SIS A, (HL)` |
| `40 ed 35` | `NONINOP.SIS` |
| `40 ed 36` | `NONINOP.SIS` |
| `40 ed 37` | `LD.SIS IX, (HL)` |
| `40 ed 38 34` | `IN0.SIS A, ($34)` |
| `40 ed 39 34` | `OUT0.SIS ($34), A` |
| `40 ed 3a 34` | `LEA.SIS IX, IX+$34` |
| `40 ed 3b 34` | `LEA.SIS IY, IY+$34` |
| `40 ed 3c` | `TST.SIS A, A` |
| `40 ed 3d` | `NONINOP.SIS` |
| `40 ed 3e` | `LD.SIS (HL), IY` |
| `40 ed 3f` | `LD.SIS (HL), IX` |
| `40 ed 40` | `IN.SIS B, (C)` |
| `40 ed 41` | `OUT.SIS (C), B` |
| `40 ed 42` | `SBC.SIS HL, BC` |
| `40 ed 43 34 12` | `LD.SIS ($1234), BC` |
| `40 ed 44` | `NEG.SIS` |
| `40 ed 45` | `RETN.SIS` |
| `40 ed 46` | `IM.SIS 0` |
| `40 ed 47` | `LD.SIS I, A` |
| `40 ed 48` | `IN.SIS C, (C)` |
| `40 ed 49` | `OUT.SIS (C), C` |
| `40 ed 4a` | `ADC.SIS HL, BC` |
| `40 ed 4b 34 12` | `LD.SIS BC, ($1234)` |
| `40 ed 4c` | `MLT.SIS BC` |
| `40 ed 4d` | `RETI.SIS` |
| `40 ed 4e` | `IM.SIS 0` |
| `40 ed 4f` | `LD.SIS R, A` |
| `40 ed 50` | `IN.SIS D, (C)` |
| `40 ed 51` | `OUT.SIS (C), D` |
| `40 ed 52` | `SBC.SIS HL, DE` |
| `40 ed 53 34 12` | `LD.SIS ($1234), DE` |
| `40 ed 54 34` | `LEA.SIS IX, IY+$34` |
| `40 ed 55 34` | `LEA.SIS IY, IX+$34` |
| `40 ed 56` | `IM.SIS 1` |
| `40 ed 57` | `LD.SIS A, I` |
| `40 ed 58` | `IN.SIS E, (C)` |
| `40 ed 59` | `OUT.SIS (C), E` |
| `40 ed 5a` | `ADC.SIS HL, DE` |
| `40 ed 5b 34 12` | `LD.SIS DE, ($1234)` |
| `40 ed 5c` | `MLT.SIS DE` |
| `40 ed 5d` | `RETN.SIS` |
| `40 ed 5e` | `IM.SIS 2` |
| `40 ed 5f` | `LD.SIS A, R` |
| `40 ed 60` | `IN.SIS H, (C)` |
| `40 ed 61` | `OUT.SIS (C), H` |
| `40 ed 62` | `SBC.SIS HL, HL` |
| `40 ed 63 34 12` | `LD.SIS ($1234), HL` |
| `40 ed 64 34` | `TST.SIS A, $34` |
| `40 ed 65 34` | `PEA.SIS IX+$34` |
| `40 ed 66 34` | `PEA.SIS IY+$34` |
| `40 ed 67` | `RRD.SIS` |
| `40 ed 68` | `IN.SIS L, (C)` |
| `40 ed 69` | `OUT.SIS (C), L` |
| `40 ed 6a` | `ADC.SIS HL, HL` |
| `40 ed 6b 34 12` | `LD.SIS HL, ($1234)` |
| `40 ed 6c` | `MLT.SIS HL` |
| `40 ed 6d` | `LD.SIS MB, A` |
| `40 ed 6e` | `LD.SIS A, MB` |
| `40 ed 6f` | `RLD.SIS` |
| `40 ed 70` | `IN.SIS (C)` |
| `40 ed 71` | `OUT.SIS (C), 0` |
| `40 ed 72` | `SBC.SIS HL, SP` |
| `40 ed 73 34 12` | `LD.SIS ($1234), SP` |
| `40 ed 74 34` | `TSTIO.SIS $34` |
| `40 ed 75` | `RETN.SIS` |
| `40 ed 76` | `SLP.SIS` |
| `40 ed 77` | `NOP.SIS` |
| `40 ed 78` | `IN.SIS A, (C)` |
| `40 ed 79` | `OUT.SIS (C), A` |
| `40 ed 7a` | `ADC.SIS HL, SP` |
| `40 ed 7b 34 12` | `LD.SIS SP, ($1234)` |
| `40 ed 7c` | `MLT.SIS SP` |
| `40 ed 7d` | `STMIX.SIS` |
| `40 ed 7e` | `RSMIX.SIS` |
| `40 ed 7f` | `NOP.SIS` |
| `40 ed 80` | `NONINOP.SIS` |
| `40 ed 81` | `NONINOP.SIS` |
| `40 ed 82` | `NONINOP.SIS` |
| `40 ed 83` | `OTIM.SIS` |
| `40 ed 84` | `INI2.SIS` |
| `40 ed 85` | `NONINOP.SIS` |
| `40 ed 86` | `NONINOP.SIS` |
| `40 ed 87` | `NONINOP.SIS` |
| `40 ed 88` | `NONINOP.SIS` |
| `40 ed 89` | `NONINOP.SIS` |
| `40 ed 8a` | `NONINOP.SIS` |
| `40 ed 8b` | `OTDM.SIS` |
| `40 ed 8c` | `IND2.SIS` |
| `40 ed 8d` | `NONINOP.SIS` |
| `40 ed 8e` | `NONINOP.SIS` |
| `40 ed 8f` | `NONINOP.SIS` |
| `40 ed 90` | `NONINOP.SIS` |
| `40 ed 91` | `NONINOP.SIS` |
| `40 ed 92` | `NONINOP.SIS` |
| `40 ed 93` | `OTIMR.SIS` |
| `40 ed 94` | `INI2R.SIS` |
| `40 ed 95` | `NONINOP.SIS` |
| `40 ed 96` | `NONINOP.SIS` |
| `40 ed 97` | `NONINOP.SIS` |
| `40 ed 98` | `NONINOP.SIS` |
| `40 ed 99` | `NONINOP.SIS` |
| `40 ed 9a` | `NONINOP.SIS` |
| `40 ed 9b` | `OTDMR.SIS` |
| `40 ed 9c` | `IND2R.SIS` |
| `40 ed 9d` | `NONINOP.SIS` |
| `40 ed 9e` | `NONINOP.SIS` |
| `40 ed 9f` | `NONINOP.SIS` |
| `40 ed a0` | `LDI.SIS` |
| `40 ed a1` | `CPI.SIS` |
| `40 ed a2` | `INI.SIS` |
| `40 ed a3` | `OUTI.SIS` |
| `40 ed a4` | `OUTI2.SIS` |
| `40 ed a5` | `NONINOP.SIS` |
| `40 ed a6` | `NONINOP.SIS` |
| `40 ed a7` | `NONINOP.SIS` |
| `40 ed a8` | `LDD.SIS` |
| `40 ed a9` | `CPD.SIS` |
| `40 ed aa` | `IND.SIS` |
| `40 ed ab` | `OUTD.SIS` |
| `40 ed ac` | `OUTD2.SIS` |
| `40 ed ad` | `NONINOP.SIS` |
| `40 ed ae` | `NONINOP.SIS` |
| `40 ed af` | `NONINOP.SIS` |
| `40 ed b0` | `LDIR.SIS` |
| `40 ed b1` | `CPIR.SIS` |
| `40 ed b2` | `INIR.SIS` |
| `40 ed b3` | `OTIR.SIS` |
| `40 ed b4` | `OTI2R.SIS` |
| `40 ed b5` | `NONINOP.SIS` |
| `40 ed b6` | `NONINOP.SIS` |
| `40 ed b7` | `NONINOP.SIS` |
| `40 ed b8` | `LDDR.SIS` |
| `40 ed b9` | `CPDR.SIS` |
| `40 ed ba` | `INDR.SIS` |
| `40 ed bb` | `OTDR.SIS` |
| `40 ed bc` | `OTD2R.SIS` |
| `40 ed bd` | `NONINOP.SIS` |
| `40 ed be` | `NONINOP.SIS` |
| `40 ed bf` | `NONINOP.SIS` |
| `40 ed c0` | `NONINOP.SIS` |
| `40 ed c1` | `NONINOP.SIS` |
| `40 ed c2` | `INIRX.SIS` |
| `40 ed c3` | `OTIRX.SIS` |
| `40 ed c4` | `NONINOP.SIS` |
| `40 ed c5` | `NONINOP.SIS` |
| `40 ed c6` | `NONINOP.SIS` |
| `40 ed c7` | `LD.SIS I, HL` |
| `40 ed c8` | `NONINOP.SIS` |
| `40 ed c9` | `NONINOP.SIS` |
| `40 ed ca` | `INDRX.SIS` |
| `40 ed cb` | `OTDRX.SIS` |
| `40 ed cc` | `NONINOP.SIS` |
| `40 ed cd` | `NONINOP.SIS` |
| `40 ed ce` | `NONINOP.SIS` |
| `40 ed cf` | `NONINOP.SIS` |
| `40 ed d0` | `NONINOP.SIS` |
| `40 ed d1` | `NONINOP.SIS` |
| `40 ed d2` | `NONINOP.SIS` |
| `40 ed d3` | `NONINOP.SIS` |
| `40 ed d4` | `NONINOP.SIS` |
| `40 ed d5` | `NONINOP.SIS` |
| `40 ed d6` | `NONINOP.SIS` |
| `40 ed d7` | `LD.SIS HL, I` |
| `40 ed d8` | `NONINOP.SIS` |
| `40 ed d9` | `NONINOP.SIS` |
| `40 ed da` | `NONINOP.SIS` |
| `40 ed db` | `NONINOP.SIS` |
| `40 ed dc` | `NONINOP.SIS` |
| `40 ed dd` | `NONINOP.SIS` |
| `40 ed de` | `NONINOP.SIS` |
| `40 ed df` | `NONINOP.SIS` |
| `40 ed e0` | `NONINOP.SIS` |
| `40 ed e1` | `NONINOP.SIS` |
| `40 ed e2` | `NONINOP.SIS` |
| `40 ed e3` | `NONINOP.SIS` |
| `40 ed e4` | `NONINOP.SIS` |
| `40 ed e5` | `NONINOP.SIS` |
| `40 ed e6` | `NONINOP.SIS` |
| `40 ed e7` | `NONINOP.SIS` |
| `40 ed e8` | `NONINOP.SIS` |
| `40 ed e9` | `NONINOP.SIS` |
| `40 ed ea` | `NONINOP.SIS` |
| `40 ed eb` | `NONINOP.SIS` |
| `40 ed ec` | `NONINOP.SIS` |
| `40 ed ed` | `NONINOP.SIS` |
| `40 ed ee` | `NONINOP.SIS` |
| `40 ed ef` | `NONINOP.SIS` |
| `40 ed f0` | `NONINOP.SIS` |
| `40 ed f1` | `NONINOP.SIS` |
| `40 ed f2` | `NONINOP.SIS` |
| `40 ed f3` | `NONINOP.SIS` |
| `40 ed f4` | `NONINOP.SIS` |
| `40 ed f5` | `NONINOP.SIS` |
| `40 ed f6` | `NONINOP.SIS` |
| `40 ed f7` | `NONINOP.SIS` |
| `40 ed f8` | `NONINOP.SIS` |
| `40 ed f9` | `NONINOP.SIS` |
| `40 ed fa` | `NONINOP.SIS` |
| `40 ed fb` | `NONINOP.SIS` |
| `40 ed fc` | `NONINOP.SIS` |
| `40 ed fd` | `NONINOP.SIS` |
| `40 ed fe` | `ILLEGAL.SIS` |
| `40 ed ff` | `NONINOP.SIS` |

## eZ80 LIS base opcodes

| Bytes | Disassembly |
| --- | --- |
| `49 00` | `NOP.LIS` |
| `49 01 34 12` | `LD.LIS BC, $1234` |
| `49 02` | `LD.LIS (BC), A` |
| `49 03` | `INC.LIS BC` |
| `49 04` | `INC.LIS B` |
| `49 05` | `DEC.LIS B` |
| `49 06 34` | `LD.LIS B, $34` |
| `49 07` | `RLCA.LIS` |
| `49 08` | `EX.LIS AF, AF'` |
| `49 09` | `ADD.LIS HL, BC` |
| `49 0a` | `LD.LIS A, (BC)` |
| `49 0b` | `DEC.LIS BC` |
| `49 0c` | `INC.LIS C` |
| `49 0d` | `DEC.LIS C` |
| `49 0e 34` | `LD.LIS C, $34` |
| `49 0f` | `RRCA.LIS` |
| `49 10 34` | `DJNZ.LIS $37` |
| `49 11 34 12` | `LD.LIS DE, $1234` |
| `49 12` | `LD.LIS (DE), A` |
| `49 13` | `INC.LIS DE` |
| `49 14` | `INC.LIS D` |
| `49 15` | `DEC.LIS D` |
| `49 16 34` | `LD.LIS D, $34` |
| `49 17` | `RLA.LIS` |
| `49 18 34` | `JR.LIS $37` |
| `49 19` | `ADD.LIS HL, DE` |
| `49 1a` | `LD.LIS A, (DE)` |
| `49 1b` | `DEC.LIS DE` |
| `49 1c` | `INC.LIS E` |
| `49 1d` | `DEC.LIS E` |
| `49 1e 34` | `LD.LIS E, $34` |
| `49 1f` | `RRA.LIS` |
| `49 20 34` | `JR.LIS NZ, $37` |
| `49 21 34 12` | `LD.LIS HL, $1234` |
| `49 22 34 12` | `LD.LIS ($1234), HL` |
| `49 23` | `INC.LIS HL` |
| `49 24` | `INC.LIS H` |
| `49 25` | `DEC.LIS H` |
| `49 26 34` | `LD.LIS H, $34` |
| `49 27` | `DAA.LIS` |
| `49 28 34` | `JR.LIS Z, $37` |
| `49 29` | `ADD.LIS HL, HL` |
| `49 2a 34 12` | `LD.LIS HL, ($1234)` |
| `49 2b` | `DEC.LIS HL` |
| `49 2c` | `INC.LIS L` |
| `49 2d` | `DEC.LIS L` |
| `49 2e 34` | `LD.LIS L, $34` |
| `49 2f` | `CPL.LIS` |
| `49 30 34` | `JR.LIS NC, $37` |
| `49 31 34 12` | `LD.LIS SP, $1234` |
| `49 32 34 12` | `LD.LIS ($1234), A` |
| `49 33` | `INC.LIS SP` |
| `49 34` | `INC.LIS (HL)` |
| `49 35` | `DEC.LIS (HL)` |
| `49 36 34` | `LD.LIS (HL), $34` |
| `49 37` | `SCF.LIS` |
| `49 38 34` | `JR.LIS C, $37` |
| `49 39` | `ADD.LIS HL, SP` |
| `49 3a 34 12` | `LD.LIS A, ($1234)` |
| `49 3b` | `DEC.LIS SP` |
| `49 3c` | `INC.LIS A` |
| `49 3d` | `DEC.LIS A` |
| `49 3e 34` | `LD.LIS A, $34` |
| `49 3f` | `CCF.LIS` |
| `49 40 34` | `INC.SIS (HL)` |
| `49 41` | `LD.LIS B, C` |
| `49 42` | `LD.LIS B, D` |
| `49 43` | `LD.LIS B, E` |
| `49 44` | `LD.LIS B, H` |
| `49 45` | `LD.LIS B, L` |
| `49 46` | `LD.LIS B, (HL)` |
| `49 47` | `LD.LIS B, A` |
| `49 48` | `LD.LIS C, B` |
| `49 49 34` | `INC.LIS (HL)` |
| `49 4a` | `LD.LIS C, D` |
| `49 4b` | `LD.LIS C, E` |
| `49 4c` | `LD.LIS C, H` |
| `49 4d` | `LD.LIS C, L` |
| `49 4e` | `LD.LIS C, (HL)` |
| `49 4f` | `LD.LIS C, A` |
| `49 50` | `LD.LIS D, B` |
| `49 51` | `LD.LIS D, C` |
| `49 52 34` | `INC.SIL (HL)` |
| `49 53` | `LD.LIS D, E` |
| `49 54` | `LD.LIS D, H` |
| `49 55` | `LD.LIS D, L` |
| `49 56` | `LD.LIS D, (HL)` |
| `49 57` | `LD.LIS D, A` |
| `49 58` | `LD.LIS E, B` |
| `49 59` | `LD.LIS E, C` |
| `49 5a` | `LD.LIS E, D` |
| `49 5b 34` | `INC.LIL (HL)` |
| `49 5c` | `LD.LIS E, H` |
| `49 5d` | `LD.LIS E, L` |
| `49 5e` | `LD.LIS E, (HL)` |
| `49 5f` | `LD.LIS E, A` |
| `49 60` | `LD.LIS H, B` |
| `49 61` | `LD.LIS H, C` |
| `49 62` | `LD.LIS H, D` |
| `49 63` | `LD.LIS H, E` |
| `49 64` | `LD.LIS H, H` |
| `49 65` | `LD.LIS H, L` |
| `49 66` | `LD.LIS H, (HL)` |
| `49 67` | `LD.LIS H, A` |
| `49 68` | `LD.LIS L, B` |
| `49 69` | `LD.LIS L, C` |
| `49 6a` | `LD.LIS L, D` |
| `49 6b` | `LD.LIS L, E` |
| `49 6c` | `LD.LIS L, H` |
| `49 6d` | `LD.LIS L, L` |
| `49 6e` | `LD.LIS L, (HL)` |
| `49 6f` | `LD.LIS L, A` |
| `49 70` | `LD.LIS (HL), B` |
| `49 71` | `LD.LIS (HL), C` |
| `49 72` | `LD.LIS (HL), D` |
| `49 73` | `LD.LIS (HL), E` |
| `49 74` | `LD.LIS (HL), H` |
| `49 75` | `LD.LIS (HL), L` |
| `49 76` | `HALT.LIS` |
| `49 77` | `LD.LIS (HL), A` |
| `49 78` | `LD.LIS A, B` |
| `49 79` | `LD.LIS A, C` |
| `49 7a` | `LD.LIS A, D` |
| `49 7b` | `LD.LIS A, E` |
| `49 7c` | `LD.LIS A, H` |
| `49 7d` | `LD.LIS A, L` |
| `49 7e` | `LD.LIS A, (HL)` |
| `49 7f` | `LD.LIS A, A` |
| `49 80` | `ADD.LIS A, B` |
| `49 81` | `ADD.LIS A, C` |
| `49 82` | `ADD.LIS A, D` |
| `49 83` | `ADD.LIS A, E` |
| `49 84` | `ADD.LIS A, H` |
| `49 85` | `ADD.LIS A, L` |
| `49 86` | `ADD.LIS A, (HL)` |
| `49 87` | `ADD.LIS A, A` |
| `49 88` | `ADC.LIS A, B` |
| `49 89` | `ADC.LIS A, C` |
| `49 8a` | `ADC.LIS A, D` |
| `49 8b` | `ADC.LIS A, E` |
| `49 8c` | `ADC.LIS A, H` |
| `49 8d` | `ADC.LIS A, L` |
| `49 8e` | `ADC.LIS A, (HL)` |
| `49 8f` | `ADC.LIS A, A` |
| `49 90` | `SUB.LIS A, B` |
| `49 91` | `SUB.LIS A, C` |
| `49 92` | `SUB.LIS A, D` |
| `49 93` | `SUB.LIS A, E` |
| `49 94` | `SUB.LIS A, H` |
| `49 95` | `SUB.LIS A, L` |
| `49 96` | `SUB.LIS A, (HL)` |
| `49 97` | `SUB.LIS A, A` |
| `49 98` | `SBC.LIS A, B` |
| `49 99` | `SBC.LIS A, C` |
| `49 9a` | `SBC.LIS A, D` |
| `49 9b` | `SBC.LIS A, E` |
| `49 9c` | `SBC.LIS A, H` |
| `49 9d` | `SBC.LIS A, L` |
| `49 9e` | `SBC.LIS A, (HL)` |
| `49 9f` | `SBC.LIS A, A` |
| `49 a0` | `AND.LIS A, B` |
| `49 a1` | `AND.LIS A, C` |
| `49 a2` | `AND.LIS A, D` |
| `49 a3` | `AND.LIS A, E` |
| `49 a4` | `AND.LIS A, H` |
| `49 a5` | `AND.LIS A, L` |
| `49 a6` | `AND.LIS A, (HL)` |
| `49 a7` | `AND.LIS A, A` |
| `49 a8` | `XOR.LIS A, B` |
| `49 a9` | `XOR.LIS A, C` |
| `49 aa` | `XOR.LIS A, D` |
| `49 ab` | `XOR.LIS A, E` |
| `49 ac` | `XOR.LIS A, H` |
| `49 ad` | `XOR.LIS A, L` |
| `49 ae` | `XOR.LIS A, (HL)` |
| `49 af` | `XOR.LIS A, A` |
| `49 b0` | `OR.LIS A, B` |
| `49 b1` | `OR.LIS A, C` |
| `49 b2` | `OR.LIS A, D` |
| `49 b3` | `OR.LIS A, E` |
| `49 b4` | `OR.LIS A, H` |
| `49 b5` | `OR.LIS A, L` |
| `49 b6` | `OR.LIS A, (HL)` |
| `49 b7` | `OR.LIS A, A` |
| `49 b8` | `CP.LIS A, B` |
| `49 b9` | `CP.LIS A, C` |
| `49 ba` | `CP.LIS A, D` |
| `49 bb` | `CP.LIS A, E` |
| `49 bc` | `CP.LIS A, H` |
| `49 bd` | `CP.LIS A, L` |
| `49 be` | `CP.LIS A, (HL)` |
| `49 bf` | `CP.LIS A, A` |
| `49 c0` | `RET.LIS NZ` |
| `49 c1` | `POP.LIS BC` |
| `49 c2 34 12` | `JP.LIS NZ, $1234` |
| `49 c3 34 12` | `JP.LIS $1234` |
| `49 c4 34 12` | `CALL.LIS NZ, $1234` |
| `49 c5` | `PUSH.LIS BC` |
| `49 c6 34` | `ADD.LIS A, $34` |
| `49 c7` | `RST.LIS 00h` |
| `49 c8` | `RET.LIS Z` |
| `49 c9` | `RET.LIS` |
| `49 ca 34 12` | `JP.LIS Z, $1234` |
| `49 cb 34` | `SLL.LIS H` |
| `49 cc 34 12` | `CALL.LIS Z, $1234` |
| `49 cd 34 12` | `CALL.LIS $1234` |
| `49 ce 34` | `ADC.LIS A, $34` |
| `49 cf` | `RST.LIS 08h` |
| `49 d0` | `RET.LIS NC` |
| `49 d1` | `POP.LIS DE` |
| `49 d2 34 12` | `JP.LIS NC, $1234` |
| `49 d3 34` | `OUT.LIS ($34), A` |
| `49 d4 34 12` | `CALL.LIS NC, $1234` |
| `49 d5` | `PUSH.LIS DE` |
| `49 d6 34` | `SUB.LIS A, $34` |
| `49 d7` | `RST.LIS 10h` |
| `49 d8` | `RET.LIS C` |
| `49 d9` | `EXX.LIS` |
| `49 da 34 12` | `JP.LIS C, $1234` |
| `49 db 34` | `IN.LIS A, ($34)` |
| `49 dc 34 12` | `CALL.LIS C, $1234` |
| `49 dd 34 12` | `INC.LIS (IX+18)` |
| `49 de 34` | `SBC.LIS A, $34` |
| `49 df` | `RST.LIS 18h` |
| `49 e0` | `RET.LIS PO` |
| `49 e1` | `POP.LIS HL` |
| `49 e2 34 12` | `JP.LIS PO, $1234` |
| `49 e3` | `EX.LIS (SP), HL` |
| `49 e4 34 12` | `CALL.LIS PO, $1234` |
| `49 e5` | `PUSH.LIS HL` |
| `49 e6 34` | `AND.LIS A, $34` |
| `49 e7` | `RST.LIS 20h` |
| `49 e8` | `RET.LIS PE` |
| `49 e9` | `JP.LIS (HL)` |
| `49 ea 34 12` | `JP.LIS PE, $1234` |
| `49 eb` | `EX.LIS DE, HL` |
| `49 ec 34 12` | `CALL.LIS PE, $1234` |
| `49 ed 34` | `TST.LIS A, (HL)` |
| `49 ee 34` | `XOR.LIS A, $34` |
| `49 ef` | `RST.LIS 28h` |
| `49 f0` | `RET.LIS P` |
| `49 f1` | `POP.LIS AF` |
| `49 f2 34 12` | `JP.LIS P, $1234` |
| `49 f3` | `DI.LIS` |
| `49 f4 34 12` | `CALL.LIS P, $1234` |
| `49 f5` | `PUSH.LIS AF` |
| `49 f6 34` | `OR.LIS A, $34` |
| `49 f7` | `RST.LIS 30h` |
| `49 f8` | `RET.LIS M` |
| `49 f9` | `LD.LIS SP, HL` |
| `49 fa 34 12` | `JP.LIS M, $1234` |
| `49 fb` | `EI.LIS` |
| `49 fc 34 12` | `CALL.LIS M, $1234` |
| `49 fd 34 12` | `INC.LIS (IY+18)` |
| `49 fe 34` | `CP.LIS A, $34` |
| `49 ff` | `RST.LIS 38h` |

## eZ80 LIS ED-prefixed opcodes

| Bytes | Disassembly |
| --- | --- |
| `49 ed 00 34` | `IN0.LIS B, ($34)` |
| `49 ed 01 34` | `OUT0.LIS ($34), B` |
| `49 ed 02 34` | `LEA.LIS BC, IX+$34` |
| `49 ed 03 34` | `LEA.LIS BC, IY+$34` |
| `49 ed 04` | `TST.LIS A, B` |
| `49 ed 05` | `NONINOP.LIS` |
| `49 ed 06` | `NONINOP.LIS` |
| `49 ed 07` | `LD.LIS BC, (HL)` |
| `49 ed 08 34` | `IN0.LIS C, ($34)` |
| `49 ed 09 34` | `OUT0.LIS ($34), C` |
| `49 ed 0a 34` | `LEA.LIS BC, IX+$34` |
| `49 ed 0b 34` | `LEA.LIS BC, IY+$34` |
| `49 ed 0c` | `TST.LIS A, C` |
| `49 ed 0d` | `NONINOP.LIS` |
| `49 ed 0e` | `NONINOP.LIS` |
| `49 ed 0f` | `LD.LIS (HL), BC` |
| `49 ed 10 34` | `IN0.LIS D, ($34)` |
| `49 ed 11 34` | `OUT0.LIS ($34), D` |
| `49 ed 12 34` | `LEA.LIS DE, IX+$34` |
| `49 ed 13 34` | `LEA.LIS DE, IY+$34` |
| `49 ed 14` | `TST.LIS A, D` |
| `49 ed 15` | `NONINOP.LIS` |
| `49 ed 16` | `NONINOP.LIS` |
| `49 ed 17` | `LD.LIS DE, (HL)` |
| `49 ed 18 34` | `IN0.LIS E, ($34)` |
| `49 ed 19 34` | `OUT0.LIS ($34), E` |
| `49 ed 1a 34` | `LEA.LIS DE, IX+$34` |
| `49 ed 1b 34` | `LEA.LIS DE, IY+$34` |
| `49 ed 1c` | `TST.LIS A, E` |
| `49 ed 1d` | `NONINOP.LIS` |
| `49 ed 1e` | `NONINOP.LIS` |
| `49 ed 1f` | `LD.LIS (HL), DE` |
| `49 ed 20 34` | `IN0.LIS H, ($34)` |
| `49 ed 21 34` | `OUT0.LIS ($34), H` |
| `49 ed 22 34` | `LEA.LIS HL, IX+$34` |
| `49 ed 23 34` | `LEA.LIS HL, IY+$34` |
| `49 ed 24` | `TST.LIS A, H` |
| `49 ed 25` | `NONINOP.LIS` |
| `49 ed 26` | `NONINOP.LIS` |
| `49 ed 27` | `LD.LIS HL, (HL)` |
| `49 ed 28 34` | `IN0.LIS L, ($34)` |
| `49 ed 29 34` | `OUT0.LIS ($34), L` |
| `49 ed 2a 34` | `LEA.LIS HL, IX+$34` |
| `49 ed 2b 34` | `LEA.LIS HL, IY+$34` |
| `49 ed 2c` | `TST.LIS A, L` |
| `49 ed 2d` | `NONINOP.LIS` |
| `49 ed 2e` | `NONINOP.LIS` |
| `49 ed 2f` | `LD.LIS (HL), HL` |
| `49 ed 30` | `NONINOP.LIS` |
| `49 ed 31` | `LD.LIS IY, (HL)` |
| `49 ed 32 34` | `LEA.LIS IX, IX+$34` |
| `49 ed 33 34` | `LEA.LIS IY, IY+$34` |
| `49 ed 34` | `TST.LIS A, (HL)` |
| `49 ed 35` | `NONINOP.LIS` |
| `49 ed 36` | `NONINOP.LIS` |
| `49 ed 37` | `LD.LIS IX, (HL)` |
| `49 ed 38 34` | `IN0.LIS A, ($34)` |
| `49 ed 39 34` | `OUT0.LIS ($34), A` |
| `49 ed 3a 34` | `LEA.LIS IX, IX+$34` |
| `49 ed 3b 34` | `LEA.LIS IY, IY+$34` |
| `49 ed 3c` | `TST.LIS A, A` |
| `49 ed 3d` | `NONINOP.LIS` |
| `49 ed 3e` | `LD.LIS (HL), IY` |
| `49 ed 3f` | `LD.LIS (HL), IX` |
| `49 ed 40` | `IN.LIS B, (C)` |
| `49 ed 41` | `OUT.LIS (C), B` |
| `49 ed 42` | `SBC.LIS HL, BC` |
| `49 ed 43 34 12` | `LD.LIS ($1234), BC` |
| `49 ed 44` | `NEG.LIS` |
| `49 ed 45` | `RETN.LIS` |
| `49 ed 46` | `IM.LIS 0` |
| `49 ed 47` | `LD.LIS I, A` |
| `49 ed 48` | `IN.LIS C, (C)` |
| `49 ed 49` | `OUT.LIS (C), C` |
| `49 ed 4a` | `ADC.LIS HL, BC` |
| `49 ed 4b 34 12` | `LD.LIS BC, ($1234)` |
| `49 ed 4c` | `MLT.LIS BC` |
| `49 ed 4d` | `RETI.LIS` |
| `49 ed 4e` | `IM.LIS 0` |
| `49 ed 4f` | `LD.LIS R, A` |
| `49 ed 50` | `IN.LIS D, (C)` |
| `49 ed 51` | `OUT.LIS (C), D` |
| `49 ed 52` | `SBC.LIS HL, DE` |
| `49 ed 53 34 12` | `LD.LIS ($1234), DE` |
| `49 ed 54 34` | `LEA.LIS IX, IY+$34` |
| `49 ed 55 34` | `LEA.LIS IY, IX+$34` |
| `49 ed 56` | `IM.LIS 1` |
| `49 ed 57` | `LD.LIS A, I` |
| `49 ed 58` | `IN.LIS E, (C)` |
| `49 ed 59` | `OUT.LIS (C), E` |
| `49 ed 5a` | `ADC.LIS HL, DE` |
| `49 ed 5b 34 12` | `LD.LIS DE, ($1234)` |
| `49 ed 5c` | `MLT.LIS DE` |
| `49 ed 5d` | `RETN.LIS` |
| `49 ed 5e` | `IM.LIS 2` |
| `49 ed 5f` | `LD.LIS A, R` |
| `49 ed 60` | `IN.LIS H, (C)` |
| `49 ed 61` | `OUT.LIS (C), H` |
| `49 ed 62` | `SBC.LIS HL, HL` |
| `49 ed 63 34 12` | `LD.LIS ($1234), HL` |
| `49 ed 64 34` | `TST.LIS A, $34` |
| `49 ed 65 34` | `PEA.LIS IX+$34` |
| `49 ed 66 34` | `PEA.LIS IY+$34` |
| `49 ed 67` | `RRD.LIS` |
| `49 ed 68` | `IN.LIS L, (C)` |
| `49 ed 69` | `OUT.LIS (C), L` |
| `49 ed 6a` | `ADC.LIS HL, HL` |
| `49 ed 6b 34 12` | `LD.LIS HL, ($1234)` |
| `49 ed 6c` | `MLT.LIS HL` |
| `49 ed 6d` | `LD.LIS MB, A` |
| `49 ed 6e` | `LD.LIS A, MB` |
| `49 ed 6f` | `RLD.LIS` |
| `49 ed 70` | `IN.LIS (C)` |
| `49 ed 71` | `OUT.LIS (C), 0` |
| `49 ed 72` | `SBC.LIS HL, SP` |
| `49 ed 73 34 12` | `LD.LIS ($1234), SP` |
| `49 ed 74 34` | `TSTIO.LIS $34` |
| `49 ed 75` | `RETN.LIS` |
| `49 ed 76` | `SLP.LIS` |
| `49 ed 77` | `NOP.LIS` |
| `49 ed 78` | `IN.LIS A, (C)` |
| `49 ed 79` | `OUT.LIS (C), A` |
| `49 ed 7a` | `ADC.LIS HL, SP` |
| `49 ed 7b 34 12` | `LD.LIS SP, ($1234)` |
| `49 ed 7c` | `MLT.LIS SP` |
| `49 ed 7d` | `STMIX.LIS` |
| `49 ed 7e` | `RSMIX.LIS` |
| `49 ed 7f` | `NOP.LIS` |
| `49 ed 80` | `NONINOP.LIS` |
| `49 ed 81` | `NONINOP.LIS` |
| `49 ed 82` | `NONINOP.LIS` |
| `49 ed 83` | `OTIM.LIS` |
| `49 ed 84` | `INI2.LIS` |
| `49 ed 85` | `NONINOP.LIS` |
| `49 ed 86` | `NONINOP.LIS` |
| `49 ed 87` | `NONINOP.LIS` |
| `49 ed 88` | `NONINOP.LIS` |
| `49 ed 89` | `NONINOP.LIS` |
| `49 ed 8a` | `NONINOP.LIS` |
| `49 ed 8b` | `OTDM.LIS` |
| `49 ed 8c` | `IND2.LIS` |
| `49 ed 8d` | `NONINOP.LIS` |
| `49 ed 8e` | `NONINOP.LIS` |
| `49 ed 8f` | `NONINOP.LIS` |
| `49 ed 90` | `NONINOP.LIS` |
| `49 ed 91` | `NONINOP.LIS` |
| `49 ed 92` | `NONINOP.LIS` |
| `49 ed 93` | `OTIMR.LIS` |
| `49 ed 94` | `INI2R.LIS` |
| `49 ed 95` | `NONINOP.LIS` |
| `49 ed 96` | `NONINOP.LIS` |
| `49 ed 97` | `NONINOP.LIS` |
| `49 ed 98` | `NONINOP.LIS` |
| `49 ed 99` | `NONINOP.LIS` |
| `49 ed 9a` | `NONINOP.LIS` |
| `49 ed 9b` | `OTDMR.LIS` |
| `49 ed 9c` | `IND2R.LIS` |
| `49 ed 9d` | `NONINOP.LIS` |
| `49 ed 9e` | `NONINOP.LIS` |
| `49 ed 9f` | `NONINOP.LIS` |
| `49 ed a0` | `LDI.LIS` |
| `49 ed a1` | `CPI.LIS` |
| `49 ed a2` | `INI.LIS` |
| `49 ed a3` | `OUTI.LIS` |
| `49 ed a4` | `OUTI2.LIS` |
| `49 ed a5` | `NONINOP.LIS` |
| `49 ed a6` | `NONINOP.LIS` |
| `49 ed a7` | `NONINOP.LIS` |
| `49 ed a8` | `LDD.LIS` |
| `49 ed a9` | `CPD.LIS` |
| `49 ed aa` | `IND.LIS` |
| `49 ed ab` | `OUTD.LIS` |
| `49 ed ac` | `OUTD2.LIS` |
| `49 ed ad` | `NONINOP.LIS` |
| `49 ed ae` | `NONINOP.LIS` |
| `49 ed af` | `NONINOP.LIS` |
| `49 ed b0` | `LDIR.LIS` |
| `49 ed b1` | `CPIR.LIS` |
| `49 ed b2` | `INIR.LIS` |
| `49 ed b3` | `OTIR.LIS` |
| `49 ed b4` | `OTI2R.LIS` |
| `49 ed b5` | `NONINOP.LIS` |
| `49 ed b6` | `NONINOP.LIS` |
| `49 ed b7` | `NONINOP.LIS` |
| `49 ed b8` | `LDDR.LIS` |
| `49 ed b9` | `CPDR.LIS` |
| `49 ed ba` | `INDR.LIS` |
| `49 ed bb` | `OTDR.LIS` |
| `49 ed bc` | `OTD2R.LIS` |
| `49 ed bd` | `NONINOP.LIS` |
| `49 ed be` | `NONINOP.LIS` |
| `49 ed bf` | `NONINOP.LIS` |
| `49 ed c0` | `NONINOP.LIS` |
| `49 ed c1` | `NONINOP.LIS` |
| `49 ed c2` | `INIRX.LIS` |
| `49 ed c3` | `OTIRX.LIS` |
| `49 ed c4` | `NONINOP.LIS` |
| `49 ed c5` | `NONINOP.LIS` |
| `49 ed c6` | `NONINOP.LIS` |
| `49 ed c7` | `LD.LIS I, HL` |
| `49 ed c8` | `NONINOP.LIS` |
| `49 ed c9` | `NONINOP.LIS` |
| `49 ed ca` | `INDRX.LIS` |
| `49 ed cb` | `OTDRX.LIS` |
| `49 ed cc` | `NONINOP.LIS` |
| `49 ed cd` | `NONINOP.LIS` |
| `49 ed ce` | `NONINOP.LIS` |
| `49 ed cf` | `NONINOP.LIS` |
| `49 ed d0` | `NONINOP.LIS` |
| `49 ed d1` | `NONINOP.LIS` |
| `49 ed d2` | `NONINOP.LIS` |
| `49 ed d3` | `NONINOP.LIS` |
| `49 ed d4` | `NONINOP.LIS` |
| `49 ed d5` | `NONINOP.LIS` |
| `49 ed d6` | `NONINOP.LIS` |
| `49 ed d7` | `LD.LIS HL, I` |
| `49 ed d8` | `NONINOP.LIS` |
| `49 ed d9` | `NONINOP.LIS` |
| `49 ed da` | `NONINOP.LIS` |
| `49 ed db` | `NONINOP.LIS` |
| `49 ed dc` | `NONINOP.LIS` |
| `49 ed dd` | `NONINOP.LIS` |
| `49 ed de` | `NONINOP.LIS` |
| `49 ed df` | `NONINOP.LIS` |
| `49 ed e0` | `NONINOP.LIS` |
| `49 ed e1` | `NONINOP.LIS` |
| `49 ed e2` | `NONINOP.LIS` |
| `49 ed e3` | `NONINOP.LIS` |
| `49 ed e4` | `NONINOP.LIS` |
| `49 ed e5` | `NONINOP.LIS` |
| `49 ed e6` | `NONINOP.LIS` |
| `49 ed e7` | `NONINOP.LIS` |
| `49 ed e8` | `NONINOP.LIS` |
| `49 ed e9` | `NONINOP.LIS` |
| `49 ed ea` | `NONINOP.LIS` |
| `49 ed eb` | `NONINOP.LIS` |
| `49 ed ec` | `NONINOP.LIS` |
| `49 ed ed` | `NONINOP.LIS` |
| `49 ed ee` | `NONINOP.LIS` |
| `49 ed ef` | `NONINOP.LIS` |
| `49 ed f0` | `NONINOP.LIS` |
| `49 ed f1` | `NONINOP.LIS` |
| `49 ed f2` | `NONINOP.LIS` |
| `49 ed f3` | `NONINOP.LIS` |
| `49 ed f4` | `NONINOP.LIS` |
| `49 ed f5` | `NONINOP.LIS` |
| `49 ed f6` | `NONINOP.LIS` |
| `49 ed f7` | `NONINOP.LIS` |
| `49 ed f8` | `NONINOP.LIS` |
| `49 ed f9` | `NONINOP.LIS` |
| `49 ed fa` | `NONINOP.LIS` |
| `49 ed fb` | `NONINOP.LIS` |
| `49 ed fc` | `NONINOP.LIS` |
| `49 ed fd` | `NONINOP.LIS` |
| `49 ed fe` | `ILLEGAL.LIS` |
| `49 ed ff` | `NONINOP.LIS` |

## eZ80 SIL base opcodes

| Bytes | Disassembly |
| --- | --- |
| `52 00` | `NOP.SIL` |
| `52 01 34 12 56` | `LD.SIL BC, $561234` |
| `52 02` | `LD.SIL (BC), A` |
| `52 03` | `INC.SIL BC` |
| `52 04` | `INC.SIL B` |
| `52 05` | `DEC.SIL B` |
| `52 06 34` | `LD.SIL B, $34` |
| `52 07` | `RLCA.SIL` |
| `52 08` | `EX.SIL AF, AF'` |
| `52 09` | `ADD.SIL HL, BC` |
| `52 0a` | `LD.SIL A, (BC)` |
| `52 0b` | `DEC.SIL BC` |
| `52 0c` | `INC.SIL C` |
| `52 0d` | `DEC.SIL C` |
| `52 0e 34` | `LD.SIL C, $34` |
| `52 0f` | `RRCA.SIL` |
| `52 10 34` | `DJNZ.SIL $37` |
| `52 11 34 12 56` | `LD.SIL DE, $561234` |
| `52 12` | `LD.SIL (DE), A` |
| `52 13` | `INC.SIL DE` |
| `52 14` | `INC.SIL D` |
| `52 15` | `DEC.SIL D` |
| `52 16 34` | `LD.SIL D, $34` |
| `52 17` | `RLA.SIL` |
| `52 18 34` | `JR.SIL $37` |
| `52 19` | `ADD.SIL HL, DE` |
| `52 1a` | `LD.SIL A, (DE)` |
| `52 1b` | `DEC.SIL DE` |
| `52 1c` | `INC.SIL E` |
| `52 1d` | `DEC.SIL E` |
| `52 1e 34` | `LD.SIL E, $34` |
| `52 1f` | `RRA.SIL` |
| `52 20 34` | `JR.SIL NZ, $37` |
| `52 21 34 12 56` | `LD.SIL HL, $561234` |
| `52 22 34 12 56` | `LD.SIL ($561234), HL` |
| `52 23` | `INC.SIL HL` |
| `52 24` | `INC.SIL H` |
| `52 25` | `DEC.SIL H` |
| `52 26 34` | `LD.SIL H, $34` |
| `52 27` | `DAA.SIL` |
| `52 28 34` | `JR.SIL Z, $37` |
| `52 29` | `ADD.SIL HL, HL` |
| `52 2a 34 12 56` | `LD.SIL HL, ($561234)` |
| `52 2b` | `DEC.SIL HL` |
| `52 2c` | `INC.SIL L` |
| `52 2d` | `DEC.SIL L` |
| `52 2e 34` | `LD.SIL L, $34` |
| `52 2f` | `CPL.SIL` |
| `52 30 34` | `JR.SIL NC, $37` |
| `52 31 34 12 56` | `LD.SIL SP, $561234` |
| `52 32 34 12 56` | `LD.SIL ($561234), A` |
| `52 33` | `INC.SIL SP` |
| `52 34` | `INC.SIL (HL)` |
| `52 35` | `DEC.SIL (HL)` |
| `52 36 34` | `LD.SIL (HL), $34` |
| `52 37` | `SCF.SIL` |
| `52 38 34` | `JR.SIL C, $37` |
| `52 39` | `ADD.SIL HL, SP` |
| `52 3a 34 12 56` | `LD.SIL A, ($561234)` |
| `52 3b` | `DEC.SIL SP` |
| `52 3c` | `INC.SIL A` |
| `52 3d` | `DEC.SIL A` |
| `52 3e 34` | `LD.SIL A, $34` |
| `52 3f` | `CCF.SIL` |
| `52 40 34` | `INC.SIS (HL)` |
| `52 41` | `LD.SIL B, C` |
| `52 42` | `LD.SIL B, D` |
| `52 43` | `LD.SIL B, E` |
| `52 44` | `LD.SIL B, H` |
| `52 45` | `LD.SIL B, L` |
| `52 46` | `LD.SIL B, (HL)` |
| `52 47` | `LD.SIL B, A` |
| `52 48` | `LD.SIL C, B` |
| `52 49 34` | `INC.LIS (HL)` |
| `52 4a` | `LD.SIL C, D` |
| `52 4b` | `LD.SIL C, E` |
| `52 4c` | `LD.SIL C, H` |
| `52 4d` | `LD.SIL C, L` |
| `52 4e` | `LD.SIL C, (HL)` |
| `52 4f` | `LD.SIL C, A` |
| `52 50` | `LD.SIL D, B` |
| `52 51` | `LD.SIL D, C` |
| `52 52 34` | `INC.SIL (HL)` |
| `52 53` | `LD.SIL D, E` |
| `52 54` | `LD.SIL D, H` |
| `52 55` | `LD.SIL D, L` |
| `52 56` | `LD.SIL D, (HL)` |
| `52 57` | `LD.SIL D, A` |
| `52 58` | `LD.SIL E, B` |
| `52 59` | `LD.SIL E, C` |
| `52 5a` | `LD.SIL E, D` |
| `52 5b 34` | `INC.LIL (HL)` |
| `52 5c` | `LD.SIL E, H` |
| `52 5d` | `LD.SIL E, L` |
| `52 5e` | `LD.SIL E, (HL)` |
| `52 5f` | `LD.SIL E, A` |
| `52 60` | `LD.SIL H, B` |
| `52 61` | `LD.SIL H, C` |
| `52 62` | `LD.SIL H, D` |
| `52 63` | `LD.SIL H, E` |
| `52 64` | `LD.SIL H, H` |
| `52 65` | `LD.SIL H, L` |
| `52 66` | `LD.SIL H, (HL)` |
| `52 67` | `LD.SIL H, A` |
| `52 68` | `LD.SIL L, B` |
| `52 69` | `LD.SIL L, C` |
| `52 6a` | `LD.SIL L, D` |
| `52 6b` | `LD.SIL L, E` |
| `52 6c` | `LD.SIL L, H` |
| `52 6d` | `LD.SIL L, L` |
| `52 6e` | `LD.SIL L, (HL)` |
| `52 6f` | `LD.SIL L, A` |
| `52 70` | `LD.SIL (HL), B` |
| `52 71` | `LD.SIL (HL), C` |
| `52 72` | `LD.SIL (HL), D` |
| `52 73` | `LD.SIL (HL), E` |
| `52 74` | `LD.SIL (HL), H` |
| `52 75` | `LD.SIL (HL), L` |
| `52 76` | `HALT.SIL` |
| `52 77` | `LD.SIL (HL), A` |
| `52 78` | `LD.SIL A, B` |
| `52 79` | `LD.SIL A, C` |
| `52 7a` | `LD.SIL A, D` |
| `52 7b` | `LD.SIL A, E` |
| `52 7c` | `LD.SIL A, H` |
| `52 7d` | `LD.SIL A, L` |
| `52 7e` | `LD.SIL A, (HL)` |
| `52 7f` | `LD.SIL A, A` |
| `52 80` | `ADD.SIL A, B` |
| `52 81` | `ADD.SIL A, C` |
| `52 82` | `ADD.SIL A, D` |
| `52 83` | `ADD.SIL A, E` |
| `52 84` | `ADD.SIL A, H` |
| `52 85` | `ADD.SIL A, L` |
| `52 86` | `ADD.SIL A, (HL)` |
| `52 87` | `ADD.SIL A, A` |
| `52 88` | `ADC.SIL A, B` |
| `52 89` | `ADC.SIL A, C` |
| `52 8a` | `ADC.SIL A, D` |
| `52 8b` | `ADC.SIL A, E` |
| `52 8c` | `ADC.SIL A, H` |
| `52 8d` | `ADC.SIL A, L` |
| `52 8e` | `ADC.SIL A, (HL)` |
| `52 8f` | `ADC.SIL A, A` |
| `52 90` | `SUB.SIL A, B` |
| `52 91` | `SUB.SIL A, C` |
| `52 92` | `SUB.SIL A, D` |
| `52 93` | `SUB.SIL A, E` |
| `52 94` | `SUB.SIL A, H` |
| `52 95` | `SUB.SIL A, L` |
| `52 96` | `SUB.SIL A, (HL)` |
| `52 97` | `SUB.SIL A, A` |
| `52 98` | `SBC.SIL A, B` |
| `52 99` | `SBC.SIL A, C` |
| `52 9a` | `SBC.SIL A, D` |
| `52 9b` | `SBC.SIL A, E` |
| `52 9c` | `SBC.SIL A, H` |
| `52 9d` | `SBC.SIL A, L` |
| `52 9e` | `SBC.SIL A, (HL)` |
| `52 9f` | `SBC.SIL A, A` |
| `52 a0` | `AND.SIL A, B` |
| `52 a1` | `AND.SIL A, C` |
| `52 a2` | `AND.SIL A, D` |
| `52 a3` | `AND.SIL A, E` |
| `52 a4` | `AND.SIL A, H` |
| `52 a5` | `AND.SIL A, L` |
| `52 a6` | `AND.SIL A, (HL)` |
| `52 a7` | `AND.SIL A, A` |
| `52 a8` | `XOR.SIL A, B` |
| `52 a9` | `XOR.SIL A, C` |
| `52 aa` | `XOR.SIL A, D` |
| `52 ab` | `XOR.SIL A, E` |
| `52 ac` | `XOR.SIL A, H` |
| `52 ad` | `XOR.SIL A, L` |
| `52 ae` | `XOR.SIL A, (HL)` |
| `52 af` | `XOR.SIL A, A` |
| `52 b0` | `OR.SIL A, B` |
| `52 b1` | `OR.SIL A, C` |
| `52 b2` | `OR.SIL A, D` |
| `52 b3` | `OR.SIL A, E` |
| `52 b4` | `OR.SIL A, H` |
| `52 b5` | `OR.SIL A, L` |
| `52 b6` | `OR.SIL A, (HL)` |
| `52 b7` | `OR.SIL A, A` |
| `52 b8` | `CP.SIL A, B` |
| `52 b9` | `CP.SIL A, C` |
| `52 ba` | `CP.SIL A, D` |
| `52 bb` | `CP.SIL A, E` |
| `52 bc` | `CP.SIL A, H` |
| `52 bd` | `CP.SIL A, L` |
| `52 be` | `CP.SIL A, (HL)` |
| `52 bf` | `CP.SIL A, A` |
| `52 c0` | `RET.SIL NZ` |
| `52 c1` | `POP.SIL BC` |
| `52 c2 34 12 56` | `JP.SIL NZ, $561234` |
| `52 c3 34 12 56` | `JP.SIL $561234` |
| `52 c4 34 12 56` | `CALL.SIL NZ, $561234` |
| `52 c5` | `PUSH.SIL BC` |
| `52 c6 34` | `ADD.SIL A, $34` |
| `52 c7` | `RST.SIL 00h` |
| `52 c8` | `RET.SIL Z` |
| `52 c9` | `RET.SIL` |
| `52 ca 34 12 56` | `JP.SIL Z, $561234` |
| `52 cb 34` | `SLL.SIL H` |
| `52 cc 34 12 56` | `CALL.SIL Z, $561234` |
| `52 cd 34 12 56` | `CALL.SIL $561234` |
| `52 ce 34` | `ADC.SIL A, $34` |
| `52 cf` | `RST.SIL 08h` |
| `52 d0` | `RET.SIL NC` |
| `52 d1` | `POP.SIL DE` |
| `52 d2 34 12 56` | `JP.SIL NC, $561234` |
| `52 d3 34` | `OUT.SIL ($34), A` |
| `52 d4 34 12 56` | `CALL.SIL NC, $561234` |
| `52 d5` | `PUSH.SIL DE` |
| `52 d6 34` | `SUB.SIL A, $34` |
| `52 d7` | `RST.SIL 10h` |
| `52 d8` | `RET.SIL C` |
| `52 d9` | `EXX.SIL` |
| `52 da 34 12 56` | `JP.SIL C, $561234` |
| `52 db 34` | `IN.SIL A, ($34)` |
| `52 dc 34 12 56` | `CALL.SIL C, $561234` |
| `52 dd 34 12` | `INC.SIL (IX+18)` |
| `52 de 34` | `SBC.SIL A, $34` |
| `52 df` | `RST.SIL 18h` |
| `52 e0` | `RET.SIL PO` |
| `52 e1` | `POP.SIL HL` |
| `52 e2 34 12 56` | `JP.SIL PO, $561234` |
| `52 e3` | `EX.SIL (SP), HL` |
| `52 e4 34 12 56` | `CALL.SIL PO, $561234` |
| `52 e5` | `PUSH.SIL HL` |
| `52 e6 34` | `AND.SIL A, $34` |
| `52 e7` | `RST.SIL 20h` |
| `52 e8` | `RET.SIL PE` |
| `52 e9` | `JP.SIL (HL)` |
| `52 ea 34 12 56` | `JP.SIL PE, $561234` |
| `52 eb` | `EX.SIL DE, HL` |
| `52 ec 34 12 56` | `CALL.SIL PE, $561234` |
| `52 ed 34` | `TST.SIL A, (HL)` |
| `52 ee 34` | `XOR.SIL A, $34` |
| `52 ef` | `RST.SIL 28h` |
| `52 f0` | `RET.SIL P` |
| `52 f1` | `POP.SIL AF` |
| `52 f2 34 12 56` | `JP.SIL P, $561234` |
| `52 f3` | `DI.SIL` |
| `52 f4 34 12 56` | `CALL.SIL P, $561234` |
| `52 f5` | `PUSH.SIL AF` |
| `52 f6 34` | `OR.SIL A, $34` |
| `52 f7` | `RST.SIL 30h` |
| `52 f8` | `RET.SIL M` |
| `52 f9` | `LD.SIL SP, HL` |
| `52 fa 34 12 56` | `JP.SIL M, $561234` |
| `52 fb` | `EI.SIL` |
| `52 fc 34 12 56` | `CALL.SIL M, $561234` |
| `52 fd 34 12` | `INC.SIL (IY+18)` |
| `52 fe 34` | `CP.SIL A, $34` |
| `52 ff` | `RST.SIL 38h` |

## eZ80 SIL ED-prefixed opcodes

| Bytes | Disassembly |
| --- | --- |
| `52 ed 00 34` | `IN0.SIL B, ($34)` |
| `52 ed 01 34` | `OUT0.SIL ($34), B` |
| `52 ed 02 34` | `LEA.SIL BC, IX+$34` |
| `52 ed 03 34` | `LEA.SIL BC, IY+$34` |
| `52 ed 04` | `TST.SIL A, B` |
| `52 ed 05` | `NONINOP.SIL` |
| `52 ed 06` | `NONINOP.SIL` |
| `52 ed 07` | `LD.SIL BC, (HL)` |
| `52 ed 08 34` | `IN0.SIL C, ($34)` |
| `52 ed 09 34` | `OUT0.SIL ($34), C` |
| `52 ed 0a 34` | `LEA.SIL BC, IX+$34` |
| `52 ed 0b 34` | `LEA.SIL BC, IY+$34` |
| `52 ed 0c` | `TST.SIL A, C` |
| `52 ed 0d` | `NONINOP.SIL` |
| `52 ed 0e` | `NONINOP.SIL` |
| `52 ed 0f` | `LD.SIL (HL), BC` |
| `52 ed 10 34` | `IN0.SIL D, ($34)` |
| `52 ed 11 34` | `OUT0.SIL ($34), D` |
| `52 ed 12 34` | `LEA.SIL DE, IX+$34` |
| `52 ed 13 34` | `LEA.SIL DE, IY+$34` |
| `52 ed 14` | `TST.SIL A, D` |
| `52 ed 15` | `NONINOP.SIL` |
| `52 ed 16` | `NONINOP.SIL` |
| `52 ed 17` | `LD.SIL DE, (HL)` |
| `52 ed 18 34` | `IN0.SIL E, ($34)` |
| `52 ed 19 34` | `OUT0.SIL ($34), E` |
| `52 ed 1a 34` | `LEA.SIL DE, IX+$34` |
| `52 ed 1b 34` | `LEA.SIL DE, IY+$34` |
| `52 ed 1c` | `TST.SIL A, E` |
| `52 ed 1d` | `NONINOP.SIL` |
| `52 ed 1e` | `NONINOP.SIL` |
| `52 ed 1f` | `LD.SIL (HL), DE` |
| `52 ed 20 34` | `IN0.SIL H, ($34)` |
| `52 ed 21 34` | `OUT0.SIL ($34), H` |
| `52 ed 22 34` | `LEA.SIL HL, IX+$34` |
| `52 ed 23 34` | `LEA.SIL HL, IY+$34` |
| `52 ed 24` | `TST.SIL A, H` |
| `52 ed 25` | `NONINOP.SIL` |
| `52 ed 26` | `NONINOP.SIL` |
| `52 ed 27` | `LD.SIL HL, (HL)` |
| `52 ed 28 34` | `IN0.SIL L, ($34)` |
| `52 ed 29 34` | `OUT0.SIL ($34), L` |
| `52 ed 2a 34` | `LEA.SIL HL, IX+$34` |
| `52 ed 2b 34` | `LEA.SIL HL, IY+$34` |
| `52 ed 2c` | `TST.SIL A, L` |
| `52 ed 2d` | `NONINOP.SIL` |
| `52 ed 2e` | `NONINOP.SIL` |
| `52 ed 2f` | `LD.SIL (HL), HL` |
| `52 ed 30` | `NONINOP.SIL` |
| `52 ed 31` | `LD.SIL IY, (HL)` |
| `52 ed 32 34` | `LEA.SIL IX, IX+$34` |
| `52 ed 33 34` | `LEA.SIL IY, IY+$34` |
| `52 ed 34` | `TST.SIL A, (HL)` |
| `52 ed 35` | `NONINOP.SIL` |
| `52 ed 36` | `NONINOP.SIL` |
| `52 ed 37` | `LD.SIL IX, (HL)` |
| `52 ed 38 34` | `IN0.SIL A, ($34)` |
| `52 ed 39 34` | `OUT0.SIL ($34), A` |
| `52 ed 3a 34` | `LEA.SIL IX, IX+$34` |
| `52 ed 3b 34` | `LEA.SIL IY, IY+$34` |
| `52 ed 3c` | `TST.SIL A, A` |
| `52 ed 3d` | `NONINOP.SIL` |
| `52 ed 3e` | `LD.SIL (HL), IY` |
| `52 ed 3f` | `LD.SIL (HL), IX` |
| `52 ed 40` | `IN.SIL B, (C)` |
| `52 ed 41` | `OUT.SIL (C), B` |
| `52 ed 42` | `SBC.SIL HL, BC` |
| `52 ed 43 34 12 56` | `LD.SIL ($561234), BC` |
| `52 ed 44` | `NEG.SIL` |
| `52 ed 45` | `RETN.SIL` |
| `52 ed 46` | `IM.SIL 0` |
| `52 ed 47` | `LD.SIL I, A` |
| `52 ed 48` | `IN.SIL C, (C)` |
| `52 ed 49` | `OUT.SIL (C), C` |
| `52 ed 4a` | `ADC.SIL HL, BC` |
| `52 ed 4b 34 12 56` | `LD.SIL BC, ($561234)` |
| `52 ed 4c` | `MLT.SIL BC` |
| `52 ed 4d` | `RETI.SIL` |
| `52 ed 4e` | `IM.SIL 0` |
| `52 ed 4f` | `LD.SIL R, A` |
| `52 ed 50` | `IN.SIL D, (C)` |
| `52 ed 51` | `OUT.SIL (C), D` |
| `52 ed 52` | `SBC.SIL HL, DE` |
| `52 ed 53 34 12 56` | `LD.SIL ($561234), DE` |
| `52 ed 54 34` | `LEA.SIL IX, IY+$34` |
| `52 ed 55 34` | `LEA.SIL IY, IX+$34` |
| `52 ed 56` | `IM.SIL 1` |
| `52 ed 57` | `LD.SIL A, I` |
| `52 ed 58` | `IN.SIL E, (C)` |
| `52 ed 59` | `OUT.SIL (C), E` |
| `52 ed 5a` | `ADC.SIL HL, DE` |
| `52 ed 5b 34 12 56` | `LD.SIL DE, ($561234)` |
| `52 ed 5c` | `MLT.SIL DE` |
| `52 ed 5d` | `RETN.SIL` |
| `52 ed 5e` | `IM.SIL 2` |
| `52 ed 5f` | `LD.SIL A, R` |
| `52 ed 60` | `IN.SIL H, (C)` |
| `52 ed 61` | `OUT.SIL (C), H` |
| `52 ed 62` | `SBC.SIL HL, HL` |
| `52 ed 63 34 12 56` | `LD.SIL ($561234), HL` |
| `52 ed 64 34` | `TST.SIL A, $34` |
| `52 ed 65 34` | `PEA.SIL IX+$34` |
| `52 ed 66 34` | `PEA.SIL IY+$34` |
| `52 ed 67` | `RRD.SIL` |
| `52 ed 68` | `IN.SIL L, (C)` |
| `52 ed 69` | `OUT.SIL (C), L` |
| `52 ed 6a` | `ADC.SIL HL, HL` |
| `52 ed 6b 34 12 56` | `LD.SIL HL, ($561234)` |
| `52 ed 6c` | `MLT.SIL HL` |
| `52 ed 6d` | `LD.SIL MB, A` |
| `52 ed 6e` | `LD.SIL A, MB` |
| `52 ed 6f` | `RLD.SIL` |
| `52 ed 70` | `IN.SIL (C)` |
| `52 ed 71` | `OUT.SIL (C), 0` |
| `52 ed 72` | `SBC.SIL HL, SP` |
| `52 ed 73 34 12 56` | `LD.SIL ($561234), SP` |
| `52 ed 74 34` | `TSTIO.SIL $34` |
| `52 ed 75` | `RETN.SIL` |
| `52 ed 76` | `SLP.SIL` |
| `52 ed 77` | `NOP.SIL` |
| `52 ed 78` | `IN.SIL A, (C)` |
| `52 ed 79` | `OUT.SIL (C), A` |
| `52 ed 7a` | `ADC.SIL HL, SP` |
| `52 ed 7b 34 12 56` | `LD.SIL SP, ($561234)` |
| `52 ed 7c` | `MLT.SIL SP` |
| `52 ed 7d` | `STMIX.SIL` |
| `52 ed 7e` | `RSMIX.SIL` |
| `52 ed 7f` | `NOP.SIL` |
| `52 ed 80` | `NONINOP.SIL` |
| `52 ed 81` | `NONINOP.SIL` |
| `52 ed 82` | `NONINOP.SIL` |
| `52 ed 83` | `OTIM.SIL` |
| `52 ed 84` | `INI2.SIL` |
| `52 ed 85` | `NONINOP.SIL` |
| `52 ed 86` | `NONINOP.SIL` |
| `52 ed 87` | `NONINOP.SIL` |
| `52 ed 88` | `NONINOP.SIL` |
| `52 ed 89` | `NONINOP.SIL` |
| `52 ed 8a` | `NONINOP.SIL` |
| `52 ed 8b` | `OTDM.SIL` |
| `52 ed 8c` | `IND2.SIL` |
| `52 ed 8d` | `NONINOP.SIL` |
| `52 ed 8e` | `NONINOP.SIL` |
| `52 ed 8f` | `NONINOP.SIL` |
| `52 ed 90` | `NONINOP.SIL` |
| `52 ed 91` | `NONINOP.SIL` |
| `52 ed 92` | `NONINOP.SIL` |
| `52 ed 93` | `OTIMR.SIL` |
| `52 ed 94` | `INI2R.SIL` |
| `52 ed 95` | `NONINOP.SIL` |
| `52 ed 96` | `NONINOP.SIL` |
| `52 ed 97` | `NONINOP.SIL` |
| `52 ed 98` | `NONINOP.SIL` |
| `52 ed 99` | `NONINOP.SIL` |
| `52 ed 9a` | `NONINOP.SIL` |
| `52 ed 9b` | `OTDMR.SIL` |
| `52 ed 9c` | `IND2R.SIL` |
| `52 ed 9d` | `NONINOP.SIL` |
| `52 ed 9e` | `NONINOP.SIL` |
| `52 ed 9f` | `NONINOP.SIL` |
| `52 ed a0` | `LDI.SIL` |
| `52 ed a1` | `CPI.SIL` |
| `52 ed a2` | `INI.SIL` |
| `52 ed a3` | `OUTI.SIL` |
| `52 ed a4` | `OUTI2.SIL` |
| `52 ed a5` | `NONINOP.SIL` |
| `52 ed a6` | `NONINOP.SIL` |
| `52 ed a7` | `NONINOP.SIL` |
| `52 ed a8` | `LDD.SIL` |
| `52 ed a9` | `CPD.SIL` |
| `52 ed aa` | `IND.SIL` |
| `52 ed ab` | `OUTD.SIL` |
| `52 ed ac` | `OUTD2.SIL` |
| `52 ed ad` | `NONINOP.SIL` |
| `52 ed ae` | `NONINOP.SIL` |
| `52 ed af` | `NONINOP.SIL` |
| `52 ed b0` | `LDIR.SIL` |
| `52 ed b1` | `CPIR.SIL` |
| `52 ed b2` | `INIR.SIL` |
| `52 ed b3` | `OTIR.SIL` |
| `52 ed b4` | `OTI2R.SIL` |
| `52 ed b5` | `NONINOP.SIL` |
| `52 ed b6` | `NONINOP.SIL` |
| `52 ed b7` | `NONINOP.SIL` |
| `52 ed b8` | `LDDR.SIL` |
| `52 ed b9` | `CPDR.SIL` |
| `52 ed ba` | `INDR.SIL` |
| `52 ed bb` | `OTDR.SIL` |
| `52 ed bc` | `OTD2R.SIL` |
| `52 ed bd` | `NONINOP.SIL` |
| `52 ed be` | `NONINOP.SIL` |
| `52 ed bf` | `NONINOP.SIL` |
| `52 ed c0` | `NONINOP.SIL` |
| `52 ed c1` | `NONINOP.SIL` |
| `52 ed c2` | `INIRX.SIL` |
| `52 ed c3` | `OTIRX.SIL` |
| `52 ed c4` | `NONINOP.SIL` |
| `52 ed c5` | `NONINOP.SIL` |
| `52 ed c6` | `NONINOP.SIL` |
| `52 ed c7` | `LD.SIL I, HL` |
| `52 ed c8` | `NONINOP.SIL` |
| `52 ed c9` | `NONINOP.SIL` |
| `52 ed ca` | `INDRX.SIL` |
| `52 ed cb` | `OTDRX.SIL` |
| `52 ed cc` | `NONINOP.SIL` |
| `52 ed cd` | `NONINOP.SIL` |
| `52 ed ce` | `NONINOP.SIL` |
| `52 ed cf` | `NONINOP.SIL` |
| `52 ed d0` | `NONINOP.SIL` |
| `52 ed d1` | `NONINOP.SIL` |
| `52 ed d2` | `NONINOP.SIL` |
| `52 ed d3` | `NONINOP.SIL` |
| `52 ed d4` | `NONINOP.SIL` |
| `52 ed d5` | `NONINOP.SIL` |
| `52 ed d6` | `NONINOP.SIL` |
| `52 ed d7` | `LD.SIL HL, I` |
| `52 ed d8` | `NONINOP.SIL` |
| `52 ed d9` | `NONINOP.SIL` |
| `52 ed da` | `NONINOP.SIL` |
| `52 ed db` | `NONINOP.SIL` |
| `52 ed dc` | `NONINOP.SIL` |
| `52 ed dd` | `NONINOP.SIL` |
| `52 ed de` | `NONINOP.SIL` |
| `52 ed df` | `NONINOP.SIL` |
| `52 ed e0` | `NONINOP.SIL` |
| `52 ed e1` | `NONINOP.SIL` |
| `52 ed e2` | `NONINOP.SIL` |
| `52 ed e3` | `NONINOP.SIL` |
| `52 ed e4` | `NONINOP.SIL` |
| `52 ed e5` | `NONINOP.SIL` |
| `52 ed e6` | `NONINOP.SIL` |
| `52 ed e7` | `NONINOP.SIL` |
| `52 ed e8` | `NONINOP.SIL` |
| `52 ed e9` | `NONINOP.SIL` |
| `52 ed ea` | `NONINOP.SIL` |
| `52 ed eb` | `NONINOP.SIL` |
| `52 ed ec` | `NONINOP.SIL` |
| `52 ed ed` | `NONINOP.SIL` |
| `52 ed ee` | `NONINOP.SIL` |
| `52 ed ef` | `NONINOP.SIL` |
| `52 ed f0` | `NONINOP.SIL` |
| `52 ed f1` | `NONINOP.SIL` |
| `52 ed f2` | `NONINOP.SIL` |
| `52 ed f3` | `NONINOP.SIL` |
| `52 ed f4` | `NONINOP.SIL` |
| `52 ed f5` | `NONINOP.SIL` |
| `52 ed f6` | `NONINOP.SIL` |
| `52 ed f7` | `NONINOP.SIL` |
| `52 ed f8` | `NONINOP.SIL` |
| `52 ed f9` | `NONINOP.SIL` |
| `52 ed fa` | `NONINOP.SIL` |
| `52 ed fb` | `NONINOP.SIL` |
| `52 ed fc` | `NONINOP.SIL` |
| `52 ed fd` | `NONINOP.SIL` |
| `52 ed fe` | `ILLEGAL.SIL` |
| `52 ed ff` | `NONINOP.SIL` |

## eZ80 LIL base opcodes

| Bytes | Disassembly |
| --- | --- |
| `5b 00` | `NOP.LIL` |
| `5b 01 34 12 56` | `LD.LIL BC, $561234` |
| `5b 02` | `LD.LIL (BC), A` |
| `5b 03` | `INC.LIL BC` |
| `5b 04` | `INC.LIL B` |
| `5b 05` | `DEC.LIL B` |
| `5b 06 34` | `LD.LIL B, $34` |
| `5b 07` | `RLCA.LIL` |
| `5b 08` | `EX.LIL AF, AF'` |
| `5b 09` | `ADD.LIL HL, BC` |
| `5b 0a` | `LD.LIL A, (BC)` |
| `5b 0b` | `DEC.LIL BC` |
| `5b 0c` | `INC.LIL C` |
| `5b 0d` | `DEC.LIL C` |
| `5b 0e 34` | `LD.LIL C, $34` |
| `5b 0f` | `RRCA.LIL` |
| `5b 10 34` | `DJNZ.LIL $37` |
| `5b 11 34 12 56` | `LD.LIL DE, $561234` |
| `5b 12` | `LD.LIL (DE), A` |
| `5b 13` | `INC.LIL DE` |
| `5b 14` | `INC.LIL D` |
| `5b 15` | `DEC.LIL D` |
| `5b 16 34` | `LD.LIL D, $34` |
| `5b 17` | `RLA.LIL` |
| `5b 18 34` | `JR.LIL $37` |
| `5b 19` | `ADD.LIL HL, DE` |
| `5b 1a` | `LD.LIL A, (DE)` |
| `5b 1b` | `DEC.LIL DE` |
| `5b 1c` | `INC.LIL E` |
| `5b 1d` | `DEC.LIL E` |
| `5b 1e 34` | `LD.LIL E, $34` |
| `5b 1f` | `RRA.LIL` |
| `5b 20 34` | `JR.LIL NZ, $37` |
| `5b 21 34 12 56` | `LD.LIL HL, $561234` |
| `5b 22 34 12 56` | `LD.LIL ($561234), HL` |
| `5b 23` | `INC.LIL HL` |
| `5b 24` | `INC.LIL H` |
| `5b 25` | `DEC.LIL H` |
| `5b 26 34` | `LD.LIL H, $34` |
| `5b 27` | `DAA.LIL` |
| `5b 28 34` | `JR.LIL Z, $37` |
| `5b 29` | `ADD.LIL HL, HL` |
| `5b 2a 34 12 56` | `LD.LIL HL, ($561234)` |
| `5b 2b` | `DEC.LIL HL` |
| `5b 2c` | `INC.LIL L` |
| `5b 2d` | `DEC.LIL L` |
| `5b 2e 34` | `LD.LIL L, $34` |
| `5b 2f` | `CPL.LIL` |
| `5b 30 34` | `JR.LIL NC, $37` |
| `5b 31 34 12 56` | `LD.LIL SP, $561234` |
| `5b 32 34 12 56` | `LD.LIL ($561234), A` |
| `5b 33` | `INC.LIL SP` |
| `5b 34` | `INC.LIL (HL)` |
| `5b 35` | `DEC.LIL (HL)` |
| `5b 36 34` | `LD.LIL (HL), $34` |
| `5b 37` | `SCF.LIL` |
| `5b 38 34` | `JR.LIL C, $37` |
| `5b 39` | `ADD.LIL HL, SP` |
| `5b 3a 34 12 56` | `LD.LIL A, ($561234)` |
| `5b 3b` | `DEC.LIL SP` |
| `5b 3c` | `INC.LIL A` |
| `5b 3d` | `DEC.LIL A` |
| `5b 3e 34` | `LD.LIL A, $34` |
| `5b 3f` | `CCF.LIL` |
| `5b 40 34` | `INC.SIS (HL)` |
| `5b 41` | `LD.LIL B, C` |
| `5b 42` | `LD.LIL B, D` |
| `5b 43` | `LD.LIL B, E` |
| `5b 44` | `LD.LIL B, H` |
| `5b 45` | `LD.LIL B, L` |
| `5b 46` | `LD.LIL B, (HL)` |
| `5b 47` | `LD.LIL B, A` |
| `5b 48` | `LD.LIL C, B` |
| `5b 49 34` | `INC.LIS (HL)` |
| `5b 4a` | `LD.LIL C, D` |
| `5b 4b` | `LD.LIL C, E` |
| `5b 4c` | `LD.LIL C, H` |
| `5b 4d` | `LD.LIL C, L` |
| `5b 4e` | `LD.LIL C, (HL)` |
| `5b 4f` | `LD.LIL C, A` |
| `5b 50` | `LD.LIL D, B` |
| `5b 51` | `LD.LIL D, C` |
| `5b 52 34` | `INC.SIL (HL)` |
| `5b 53` | `LD.LIL D, E` |
| `5b 54` | `LD.LIL D, H` |
| `5b 55` | `LD.LIL D, L` |
| `5b 56` | `LD.LIL D, (HL)` |
| `5b 57` | `LD.LIL D, A` |
| `5b 58` | `LD.LIL E, B` |
| `5b 59` | `LD.LIL E, C` |
| `5b 5a` | `LD.LIL E, D` |
| `5b 5b 34` | `INC.LIL (HL)` |
| `5b 5c` | `LD.LIL E, H` |
| `5b 5d` | `LD.LIL E, L` |
| `5b 5e` | `LD.LIL E, (HL)` |
| `5b 5f` | `LD.LIL E, A` |
| `5b 60` | `LD.LIL H, B` |
| `5b 61` | `LD.LIL H, C` |
| `5b 62` | `LD.LIL H, D` |
| `5b 63` | `LD.LIL H, E` |
| `5b 64` | `LD.LIL H, H` |
| `5b 65` | `LD.LIL H, L` |
| `5b 66` | `LD.LIL H, (HL)` |
| `5b 67` | `LD.LIL H, A` |
| `5b 68` | `LD.LIL L, B` |
| `5b 69` | `LD.LIL L, C` |
| `5b 6a` | `LD.LIL L, D` |
| `5b 6b` | `LD.LIL L, E` |
| `5b 6c` | `LD.LIL L, H` |
| `5b 6d` | `LD.LIL L, L` |
| `5b 6e` | `LD.LIL L, (HL)` |
| `5b 6f` | `LD.LIL L, A` |
| `5b 70` | `LD.LIL (HL), B` |
| `5b 71` | `LD.LIL (HL), C` |
| `5b 72` | `LD.LIL (HL), D` |
| `5b 73` | `LD.LIL (HL), E` |
| `5b 74` | `LD.LIL (HL), H` |
| `5b 75` | `LD.LIL (HL), L` |
| `5b 76` | `HALT.LIL` |
| `5b 77` | `LD.LIL (HL), A` |
| `5b 78` | `LD.LIL A, B` |
| `5b 79` | `LD.LIL A, C` |
| `5b 7a` | `LD.LIL A, D` |
| `5b 7b` | `LD.LIL A, E` |
| `5b 7c` | `LD.LIL A, H` |
| `5b 7d` | `LD.LIL A, L` |
| `5b 7e` | `LD.LIL A, (HL)` |
| `5b 7f` | `LD.LIL A, A` |
| `5b 80` | `ADD.LIL A, B` |
| `5b 81` | `ADD.LIL A, C` |
| `5b 82` | `ADD.LIL A, D` |
| `5b 83` | `ADD.LIL A, E` |
| `5b 84` | `ADD.LIL A, H` |
| `5b 85` | `ADD.LIL A, L` |
| `5b 86` | `ADD.LIL A, (HL)` |
| `5b 87` | `ADD.LIL A, A` |
| `5b 88` | `ADC.LIL A, B` |
| `5b 89` | `ADC.LIL A, C` |
| `5b 8a` | `ADC.LIL A, D` |
| `5b 8b` | `ADC.LIL A, E` |
| `5b 8c` | `ADC.LIL A, H` |
| `5b 8d` | `ADC.LIL A, L` |
| `5b 8e` | `ADC.LIL A, (HL)` |
| `5b 8f` | `ADC.LIL A, A` |
| `5b 90` | `SUB.LIL A, B` |
| `5b 91` | `SUB.LIL A, C` |
| `5b 92` | `SUB.LIL A, D` |
| `5b 93` | `SUB.LIL A, E` |
| `5b 94` | `SUB.LIL A, H` |
| `5b 95` | `SUB.LIL A, L` |
| `5b 96` | `SUB.LIL A, (HL)` |
| `5b 97` | `SUB.LIL A, A` |
| `5b 98` | `SBC.LIL A, B` |
| `5b 99` | `SBC.LIL A, C` |
| `5b 9a` | `SBC.LIL A, D` |
| `5b 9b` | `SBC.LIL A, E` |
| `5b 9c` | `SBC.LIL A, H` |
| `5b 9d` | `SBC.LIL A, L` |
| `5b 9e` | `SBC.LIL A, (HL)` |
| `5b 9f` | `SBC.LIL A, A` |
| `5b a0` | `AND.LIL A, B` |
| `5b a1` | `AND.LIL A, C` |
| `5b a2` | `AND.LIL A, D` |
| `5b a3` | `AND.LIL A, E` |
| `5b a4` | `AND.LIL A, H` |
| `5b a5` | `AND.LIL A, L` |
| `5b a6` | `AND.LIL A, (HL)` |
| `5b a7` | `AND.LIL A, A` |
| `5b a8` | `XOR.LIL A, B` |
| `5b a9` | `XOR.LIL A, C` |
| `5b aa` | `XOR.LIL A, D` |
| `5b ab` | `XOR.LIL A, E` |
| `5b ac` | `XOR.LIL A, H` |
| `5b ad` | `XOR.LIL A, L` |
| `5b ae` | `XOR.LIL A, (HL)` |
| `5b af` | `XOR.LIL A, A` |
| `5b b0` | `OR.LIL A, B` |
| `5b b1` | `OR.LIL A, C` |
| `5b b2` | `OR.LIL A, D` |
| `5b b3` | `OR.LIL A, E` |
| `5b b4` | `OR.LIL A, H` |
| `5b b5` | `OR.LIL A, L` |
| `5b b6` | `OR.LIL A, (HL)` |
| `5b b7` | `OR.LIL A, A` |
| `5b b8` | `CP.LIL A, B` |
| `5b b9` | `CP.LIL A, C` |
| `5b ba` | `CP.LIL A, D` |
| `5b bb` | `CP.LIL A, E` |
| `5b bc` | `CP.LIL A, H` |
| `5b bd` | `CP.LIL A, L` |
| `5b be` | `CP.LIL A, (HL)` |
| `5b bf` | `CP.LIL A, A` |
| `5b c0` | `RET.LIL NZ` |
| `5b c1` | `POP.LIL BC` |
| `5b c2 34 12 56` | `JP.LIL NZ, $561234` |
| `5b c3 34 12 56` | `JP.LIL $561234` |
| `5b c4 34 12 56` | `CALL.LIL NZ, $561234` |
| `5b c5` | `PUSH.LIL BC` |
| `5b c6 34` | `ADD.LIL A, $34` |
| `5b c7` | `RST.LIL 00h` |
| `5b c8` | `RET.LIL Z` |
| `5b c9` | `RET.LIL` |
| `5b ca 34 12 56` | `JP.LIL Z, $561234` |
| `5b cb 34` | `SLL.LIL H` |
| `5b cc 34 12 56` | `CALL.LIL Z, $561234` |
| `5b cd 34 12 56` | `CALL.LIL $561234` |
| `5b ce 34` | `ADC.LIL A, $34` |
| `5b cf` | `RST.LIL 08h` |
| `5b d0` | `RET.LIL NC` |
| `5b d1` | `POP.LIL DE` |
| `5b d2 34 12 56` | `JP.LIL NC, $561234` |
| `5b d3 34` | `OUT.LIL ($34), A` |
| `5b d4 34 12 56` | `CALL.LIL NC, $561234` |
| `5b d5` | `PUSH.LIL DE` |
| `5b d6 34` | `SUB.LIL A, $34` |
| `5b d7` | `RST.LIL 10h` |
| `5b d8` | `RET.LIL C` |
| `5b d9` | `EXX.LIL` |
| `5b da 34 12 56` | `JP.LIL C, $561234` |
| `5b db 34` | `IN.LIL A, ($34)` |
| `5b dc 34 12 56` | `CALL.LIL C, $561234` |
| `5b dd 34 12` | `INC.LIL (IX+18)` |
| `5b de 34` | `SBC.LIL A, $34` |
| `5b df` | `RST.LIL 18h` |
| `5b e0` | `RET.LIL PO` |
| `5b e1` | `POP.LIL HL` |
| `5b e2 34 12 56` | `JP.LIL PO, $561234` |
| `5b e3` | `EX.LIL (SP), HL` |
| `5b e4 34 12 56` | `CALL.LIL PO, $561234` |
| `5b e5` | `PUSH.LIL HL` |
| `5b e6 34` | `AND.LIL A, $34` |
| `5b e7` | `RST.LIL 20h` |
| `5b e8` | `RET.LIL PE` |
| `5b e9` | `JP.LIL (HL)` |
| `5b ea 34 12 56` | `JP.LIL PE, $561234` |
| `5b eb` | `EX.LIL DE, HL` |
| `5b ec 34 12 56` | `CALL.LIL PE, $561234` |
| `5b ed 34` | `TST.LIL A, (HL)` |
| `5b ee 34` | `XOR.LIL A, $34` |
| `5b ef` | `RST.LIL 28h` |
| `5b f0` | `RET.LIL P` |
| `5b f1` | `POP.LIL AF` |
| `5b f2 34 12 56` | `JP.LIL P, $561234` |
| `5b f3` | `DI.LIL` |
| `5b f4 34 12 56` | `CALL.LIL P, $561234` |
| `5b f5` | `PUSH.LIL AF` |
| `5b f6 34` | `OR.LIL A, $34` |
| `5b f7` | `RST.LIL 30h` |
| `5b f8` | `RET.LIL M` |
| `5b f9` | `LD.LIL SP, HL` |
| `5b fa 34 12 56` | `JP.LIL M, $561234` |
| `5b fb` | `EI.LIL` |
| `5b fc 34 12 56` | `CALL.LIL M, $561234` |
| `5b fd 34 12` | `INC.LIL (IY+18)` |
| `5b fe 34` | `CP.LIL A, $34` |
| `5b ff` | `RST.LIL 38h` |

## eZ80 LIL ED-prefixed opcodes

| Bytes | Disassembly |
| --- | --- |
| `5b ed 00 34` | `IN0.LIL B, ($34)` |
| `5b ed 01 34` | `OUT0.LIL ($34), B` |
| `5b ed 02 34` | `LEA.LIL BC, IX+$34` |
| `5b ed 03 34` | `LEA.LIL BC, IY+$34` |
| `5b ed 04` | `TST.LIL A, B` |
| `5b ed 05` | `NONINOP.LIL` |
| `5b ed 06` | `NONINOP.LIL` |
| `5b ed 07` | `LD.LIL BC, (HL)` |
| `5b ed 08 34` | `IN0.LIL C, ($34)` |
| `5b ed 09 34` | `OUT0.LIL ($34), C` |
| `5b ed 0a 34` | `LEA.LIL BC, IX+$34` |
| `5b ed 0b 34` | `LEA.LIL BC, IY+$34` |
| `5b ed 0c` | `TST.LIL A, C` |
| `5b ed 0d` | `NONINOP.LIL` |
| `5b ed 0e` | `NONINOP.LIL` |
| `5b ed 0f` | `LD.LIL (HL), BC` |
| `5b ed 10 34` | `IN0.LIL D, ($34)` |
| `5b ed 11 34` | `OUT0.LIL ($34), D` |
| `5b ed 12 34` | `LEA.LIL DE, IX+$34` |
| `5b ed 13 34` | `LEA.LIL DE, IY+$34` |
| `5b ed 14` | `TST.LIL A, D` |
| `5b ed 15` | `NONINOP.LIL` |
| `5b ed 16` | `NONINOP.LIL` |
| `5b ed 17` | `LD.LIL DE, (HL)` |
| `5b ed 18 34` | `IN0.LIL E, ($34)` |
| `5b ed 19 34` | `OUT0.LIL ($34), E` |
| `5b ed 1a 34` | `LEA.LIL DE, IX+$34` |
| `5b ed 1b 34` | `LEA.LIL DE, IY+$34` |
| `5b ed 1c` | `TST.LIL A, E` |
| `5b ed 1d` | `NONINOP.LIL` |
| `5b ed 1e` | `NONINOP.LIL` |
| `5b ed 1f` | `LD.LIL (HL), DE` |
| `5b ed 20 34` | `IN0.LIL H, ($34)` |
| `5b ed 21 34` | `OUT0.LIL ($34), H` |
| `5b ed 22 34` | `LEA.LIL HL, IX+$34` |
| `5b ed 23 34` | `LEA.LIL HL, IY+$34` |
| `5b ed 24` | `TST.LIL A, H` |
| `5b ed 25` | `NONINOP.LIL` |
| `5b ed 26` | `NONINOP.LIL` |
| `5b ed 27` | `LD.LIL HL, (HL)` |
| `5b ed 28 34` | `IN0.LIL L, ($34)` |
| `5b ed 29 34` | `OUT0.LIL ($34), L` |
| `5b ed 2a 34` | `LEA.LIL HL, IX+$34` |
| `5b ed 2b 34` | `LEA.LIL HL, IY+$34` |
| `5b ed 2c` | `TST.LIL A, L` |
| `5b ed 2d` | `NONINOP.LIL` |
| `5b ed 2e` | `NONINOP.LIL` |
| `5b ed 2f` | `LD.LIL (HL), HL` |
| `5b ed 30` | `NONINOP.LIL` |
| `5b ed 31` | `LD.LIL IY, (HL)` |
| `5b ed 32 34` | `LEA.LIL IX, IX+$34` |
| `5b ed 33 34` | `LEA.LIL IY, IY+$34` |
| `5b ed 34` | `TST.LIL A, (HL)` |
| `5b ed 35` | `NONINOP.LIL` |
| `5b ed 36` | `NONINOP.LIL` |
| `5b ed 37` | `LD.LIL IX, (HL)` |
| `5b ed 38 34` | `IN0.LIL A, ($34)` |
| `5b ed 39 34` | `OUT0.LIL ($34), A` |
| `5b ed 3a 34` | `LEA.LIL IX, IX+$34` |
| `5b ed 3b 34` | `LEA.LIL IY, IY+$34` |
| `5b ed 3c` | `TST.LIL A, A` |
| `5b ed 3d` | `NONINOP.LIL` |
| `5b ed 3e` | `LD.LIL (HL), IY` |
| `5b ed 3f` | `LD.LIL (HL), IX` |
| `5b ed 40` | `IN.LIL B, (C)` |
| `5b ed 41` | `OUT.LIL (C), B` |
| `5b ed 42` | `SBC.LIL HL, BC` |
| `5b ed 43 34 12 56` | `LD.LIL ($561234), BC` |
| `5b ed 44` | `NEG.LIL` |
| `5b ed 45` | `RETN.LIL` |
| `5b ed 46` | `IM.LIL 0` |
| `5b ed 47` | `LD.LIL I, A` |
| `5b ed 48` | `IN.LIL C, (C)` |
| `5b ed 49` | `OUT.LIL (C), C` |
| `5b ed 4a` | `ADC.LIL HL, BC` |
| `5b ed 4b 34 12 56` | `LD.LIL BC, ($561234)` |
| `5b ed 4c` | `MLT.LIL BC` |
| `5b ed 4d` | `RETI.LIL` |
| `5b ed 4e` | `IM.LIL 0` |
| `5b ed 4f` | `LD.LIL R, A` |
| `5b ed 50` | `IN.LIL D, (C)` |
| `5b ed 51` | `OUT.LIL (C), D` |
| `5b ed 52` | `SBC.LIL HL, DE` |
| `5b ed 53 34 12 56` | `LD.LIL ($561234), DE` |
| `5b ed 54 34` | `LEA.LIL IX, IY+$34` |
| `5b ed 55 34` | `LEA.LIL IY, IX+$34` |
| `5b ed 56` | `IM.LIL 1` |
| `5b ed 57` | `LD.LIL A, I` |
| `5b ed 58` | `IN.LIL E, (C)` |
| `5b ed 59` | `OUT.LIL (C), E` |
| `5b ed 5a` | `ADC.LIL HL, DE` |
| `5b ed 5b 34 12 56` | `LD.LIL DE, ($561234)` |
| `5b ed 5c` | `MLT.LIL DE` |
| `5b ed 5d` | `RETN.LIL` |
| `5b ed 5e` | `IM.LIL 2` |
| `5b ed 5f` | `LD.LIL A, R` |
| `5b ed 60` | `IN.LIL H, (C)` |
| `5b ed 61` | `OUT.LIL (C), H` |
| `5b ed 62` | `SBC.LIL HL, HL` |
| `5b ed 63 34 12 56` | `LD.LIL ($561234), HL` |
| `5b ed 64 34` | `TST.LIL A, $34` |
| `5b ed 65 34` | `PEA.LIL IX+$34` |
| `5b ed 66 34` | `PEA.LIL IY+$34` |
| `5b ed 67` | `RRD.LIL` |
| `5b ed 68` | `IN.LIL L, (C)` |
| `5b ed 69` | `OUT.LIL (C), L` |
| `5b ed 6a` | `ADC.LIL HL, HL` |
| `5b ed 6b 34 12 56` | `LD.LIL HL, ($561234)` |
| `5b ed 6c` | `MLT.LIL HL` |
| `5b ed 6d` | `LD.LIL MB, A` |
| `5b ed 6e` | `LD.LIL A, MB` |
| `5b ed 6f` | `RLD.LIL` |
| `5b ed 70` | `IN.LIL (C)` |
| `5b ed 71` | `OUT.LIL (C), 0` |
| `5b ed 72` | `SBC.LIL HL, SP` |
| `5b ed 73 34 12 56` | `LD.LIL ($561234), SP` |
| `5b ed 74 34` | `TSTIO.LIL $34` |
| `5b ed 75` | `RETN.LIL` |
| `5b ed 76` | `SLP.LIL` |
| `5b ed 77` | `NOP.LIL` |
| `5b ed 78` | `IN.LIL A, (C)` |
| `5b ed 79` | `OUT.LIL (C), A` |
| `5b ed 7a` | `ADC.LIL HL, SP` |
| `5b ed 7b 34 12 56` | `LD.LIL SP, ($561234)` |
| `5b ed 7c` | `MLT.LIL SP` |
| `5b ed 7d` | `STMIX.LIL` |
| `5b ed 7e` | `RSMIX.LIL` |
| `5b ed 7f` | `NOP.LIL` |
| `5b ed 80` | `NONINOP.LIL` |
| `5b ed 81` | `NONINOP.LIL` |
| `5b ed 82` | `NONINOP.LIL` |
| `5b ed 83` | `OTIM.LIL` |
| `5b ed 84` | `INI2.LIL` |
| `5b ed 85` | `NONINOP.LIL` |
| `5b ed 86` | `NONINOP.LIL` |
| `5b ed 87` | `NONINOP.LIL` |
| `5b ed 88` | `NONINOP.LIL` |
| `5b ed 89` | `NONINOP.LIL` |
| `5b ed 8a` | `NONINOP.LIL` |
| `5b ed 8b` | `OTDM.LIL` |
| `5b ed 8c` | `IND2.LIL` |
| `5b ed 8d` | `NONINOP.LIL` |
| `5b ed 8e` | `NONINOP.LIL` |
| `5b ed 8f` | `NONINOP.LIL` |
| `5b ed 90` | `NONINOP.LIL` |
| `5b ed 91` | `NONINOP.LIL` |
| `5b ed 92` | `NONINOP.LIL` |
| `5b ed 93` | `OTIMR.LIL` |
| `5b ed 94` | `INI2R.LIL` |
| `5b ed 95` | `NONINOP.LIL` |
| `5b ed 96` | `NONINOP.LIL` |
| `5b ed 97` | `NONINOP.LIL` |
| `5b ed 98` | `NONINOP.LIL` |
| `5b ed 99` | `NONINOP.LIL` |
| `5b ed 9a` | `NONINOP.LIL` |
| `5b ed 9b` | `OTDMR.LIL` |
| `5b ed 9c` | `IND2R.LIL` |
| `5b ed 9d` | `NONINOP.LIL` |
| `5b ed 9e` | `NONINOP.LIL` |
| `5b ed 9f` | `NONINOP.LIL` |
| `5b ed a0` | `LDI.LIL` |
| `5b ed a1` | `CPI.LIL` |
| `5b ed a2` | `INI.LIL` |
| `5b ed a3` | `OUTI.LIL` |
| `5b ed a4` | `OUTI2.LIL` |
| `5b ed a5` | `NONINOP.LIL` |
| `5b ed a6` | `NONINOP.LIL` |
| `5b ed a7` | `NONINOP.LIL` |
| `5b ed a8` | `LDD.LIL` |
| `5b ed a9` | `CPD.LIL` |
| `5b ed aa` | `IND.LIL` |
| `5b ed ab` | `OUTD.LIL` |
| `5b ed ac` | `OUTD2.LIL` |
| `5b ed ad` | `NONINOP.LIL` |
| `5b ed ae` | `NONINOP.LIL` |
| `5b ed af` | `NONINOP.LIL` |
| `5b ed b0` | `LDIR.LIL` |
| `5b ed b1` | `CPIR.LIL` |
| `5b ed b2` | `INIR.LIL` |
| `5b ed b3` | `OTIR.LIL` |
| `5b ed b4` | `OTI2R.LIL` |
| `5b ed b5` | `NONINOP.LIL` |
| `5b ed b6` | `NONINOP.LIL` |
| `5b ed b7` | `NONINOP.LIL` |
| `5b ed b8` | `LDDR.LIL` |
| `5b ed b9` | `CPDR.LIL` |
| `5b ed ba` | `INDR.LIL` |
| `5b ed bb` | `OTDR.LIL` |
| `5b ed bc` | `OTD2R.LIL` |
| `5b ed bd` | `NONINOP.LIL` |
| `5b ed be` | `NONINOP.LIL` |
| `5b ed bf` | `NONINOP.LIL` |
| `5b ed c0` | `NONINOP.LIL` |
| `5b ed c1` | `NONINOP.LIL` |
| `5b ed c2` | `INIRX.LIL` |
| `5b ed c3` | `OTIRX.LIL` |
| `5b ed c4` | `NONINOP.LIL` |
| `5b ed c5` | `NONINOP.LIL` |
| `5b ed c6` | `NONINOP.LIL` |
| `5b ed c7` | `LD.LIL I, HL` |
| `5b ed c8` | `NONINOP.LIL` |
| `5b ed c9` | `NONINOP.LIL` |
| `5b ed ca` | `INDRX.LIL` |
| `5b ed cb` | `OTDRX.LIL` |
| `5b ed cc` | `NONINOP.LIL` |
| `5b ed cd` | `NONINOP.LIL` |
| `5b ed ce` | `NONINOP.LIL` |
| `5b ed cf` | `NONINOP.LIL` |
| `5b ed d0` | `NONINOP.LIL` |
| `5b ed d1` | `NONINOP.LIL` |
| `5b ed d2` | `NONINOP.LIL` |
| `5b ed d3` | `NONINOP.LIL` |
| `5b ed d4` | `NONINOP.LIL` |
| `5b ed d5` | `NONINOP.LIL` |
| `5b ed d6` | `NONINOP.LIL` |
| `5b ed d7` | `LD.LIL HL, I` |
| `5b ed d8` | `NONINOP.LIL` |
| `5b ed d9` | `NONINOP.LIL` |
| `5b ed da` | `NONINOP.LIL` |
| `5b ed db` | `NONINOP.LIL` |
| `5b ed dc` | `NONINOP.LIL` |
| `5b ed dd` | `NONINOP.LIL` |
| `5b ed de` | `NONINOP.LIL` |
| `5b ed df` | `NONINOP.LIL` |
| `5b ed e0` | `NONINOP.LIL` |
| `5b ed e1` | `NONINOP.LIL` |
| `5b ed e2` | `NONINOP.LIL` |
| `5b ed e3` | `NONINOP.LIL` |
| `5b ed e4` | `NONINOP.LIL` |
| `5b ed e5` | `NONINOP.LIL` |
| `5b ed e6` | `NONINOP.LIL` |
| `5b ed e7` | `NONINOP.LIL` |
| `5b ed e8` | `NONINOP.LIL` |
| `5b ed e9` | `NONINOP.LIL` |
| `5b ed ea` | `NONINOP.LIL` |
| `5b ed eb` | `NONINOP.LIL` |
| `5b ed ec` | `NONINOP.LIL` |
| `5b ed ed` | `NONINOP.LIL` |
| `5b ed ee` | `NONINOP.LIL` |
| `5b ed ef` | `NONINOP.LIL` |
| `5b ed f0` | `NONINOP.LIL` |
| `5b ed f1` | `NONINOP.LIL` |
| `5b ed f2` | `NONINOP.LIL` |
| `5b ed f3` | `NONINOP.LIL` |
| `5b ed f4` | `NONINOP.LIL` |
| `5b ed f5` | `NONINOP.LIL` |
| `5b ed f6` | `NONINOP.LIL` |
| `5b ed f7` | `NONINOP.LIL` |
| `5b ed f8` | `NONINOP.LIL` |
| `5b ed f9` | `NONINOP.LIL` |
| `5b ed fa` | `NONINOP.LIL` |
| `5b ed fb` | `NONINOP.LIL` |
| `5b ed fc` | `NONINOP.LIL` |
| `5b ed fd` | `NONINOP.LIL` |
| `5b ed fe` | `ILLEGAL.LIL` |
| `5b ed ff` | `NONINOP.LIL` |

