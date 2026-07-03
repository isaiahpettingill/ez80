# ez80

This is an extension of [iz80](https://github.com/ivanizag/iz80) to support the eZ80 CPU.

It passes all the tests of the ZEXALL suite, but the eZ80 mode is more experimental. It
is used by the [Agon Light Emulator](https://github.com/astralaster/agon-light-emulator),
via the [Agon CPU Emulator](https://github.com/tomm/agon-cpu-emulator), which emulates
the CPU and CPU-connected peripherals of the Agon Light.

To run the ZEXALL test suite for Zilog Z80:

```shell
cargo test --release -- --nocapture --ignored --test zexall
```

To run the EX8080 test suite for Intel 8080:

```shell
cargo test --release -- --nocapture --ignored --test ex8080
```

To run the raxoft Z80 test binaries on both the standard and static fast paths:

```shell
cargo test --test z80test -- --ignored --nocapture
```

To run the CPU fast-path benchmarks:

```shell
cargo bench --bench cpu_fast_path
```

To build the Dhrystone Z80 benchmark when SDCC is installed:

```shell
scripts/bench_dhrystone.sh
```


To run Tiny Basic (from [cpuville](http://cpuville.com/Kits/Z80-kits-home.html)):

```shell
cargo run --bin cpuville
```

## Usage

See [cpuville.rs](src/bin/cpuville.rs) or the CP/M 2.2 emulator [iz-cpm](https://github.com/ivanizag/iz-cpm) for more usage examples.

To run this example, execute: `cargo run --bin simplest`

```rust
use iz80::*;

fn main() {
    // Prepare the device
    let mut machine = PlainMachine::new();
    let mut cpu = Cpu::new(); // Or Cpu::new_8080()
    cpu.set_trace(true);

    // Load program inline or from a file with:
    //      let code = include_bytes!("XXXX.rom");
    let code = [0x3c, 0xc3, 0x00, 0x00]; // INC A, JP $0000
    let size = code.len();
    for i in 0..size {
        machine.poke(0x0000 + i as u16, code[i]);
    }

    // Run emulation
    cpu.registers().set_pc(0x0000);
    loop {
        cpu.execute_instruction(&mut machine);

        // Examine machine state to update the hosting device as needed.
        if cpu.registers().a() == 0x10 {
            // Let's stop
            break;
        }
    }
}
```

## Links

- The ZEXALL test suite for Z80 was taken from https://github.com/anotherlin/z80emu
- The EX8080 test suite for Intel 8080 was taken from https://github.com/begoon/i8080-core
- The raxoft Z80 tests were taken from https://github.com/raxoft/z80test

## Test results:

### Diagnostics II by Supersoft Associates for Intel 8080

```
DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS 8080/8085
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK
```
### Diagnostics II by Supersoft Associates for Z80

```
DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS Z80
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK
```

### Z80 instruction exerciser ZEXALL

```
Z80 instruction exerciser
<adc,sbc> hl,<bc,de,hl,sp>....  OK
add hl,<bc,de,hl,sp>..........  OK
add ix,<bc,de,ix,sp>..........  OK
add iy,<bc,de,iy,sp>..........  OK
aluop a,nn....................  OK
aluop a,<b,c,d,e,h,l,(hl),a>..  OK
aluop a,<ixh,ixl,iyh,iyl>.....  OK
aluop a,(<ix,iy>+1)...........  OK
bit n,(<ix,iy>+1).............  OK
bit n,<b,c,d,e,h,l,(hl),a>....  OK
cpd<r>........................  OK
cpi<r>........................  OK
<daa,cpl,scf,ccf>.............  OK
<inc,dec> a...................  OK
<inc,dec> b...................  OK
<inc,dec> bc..................  OK
<inc,dec> c...................  OK
<inc,dec> d...................  OK
<inc,dec> de..................  OK
<inc,dec> e...................  OK
<inc,dec> h...................  OK
<inc,dec> hl..................  OK
<inc,dec> ix..................  OK
<inc,dec> iy..................  OK
<inc,dec> l...................  OK
<inc,dec> (hl)................  OK
<inc,dec> sp..................  OK
<inc,dec> (<ix,iy>+1).........  OK
<inc,dec> ixh.................  OK
<inc,dec> ixl.................  OK
<inc,dec> iyh.................  OK
<inc,dec> iyl.................  OK
ld <bc,de>,(nnnn).............  OK
ld hl,(nnnn)..................  OK
ld sp,(nnnn)..................  OK
ld <ix,iy>,(nnnn).............  OK
ld (nnnn),<bc,de>.............  OK
ld (nnnn),hl..................  OK
ld (nnnn),sp..................  OK
ld (nnnn),<ix,iy>.............  OK
ld <bc,de,hl,sp>,nnnn.........  OK
ld <ix,iy>,nnnn...............  OK
ld a,<(bc),(de)>..............  OK
ld <b,c,d,e,h,l,(hl),a>,nn....  OK
ld (<ix,iy>+1),nn.............  OK
ld <b,c,d,e>,(<ix,iy>+1)......  OK
ld <h,l>,(<ix,iy>+1)..........  OK
ld a,(<ix,iy>+1)..............  OK
ld <ixh,ixl,iyh,iyl>,nn.......  OK
ld <bcdehla>,<bcdehla>........  OK
ld <bcdexya>,<bcdexya>........  OK
ld a,(nnnn) / ld (nnnn),a.....  OK
ldd<r> (1)....................  OK
ldd<r> (2)....................  OK
ldi<r> (1)....................  OK
ldi<r> (2)....................  OK
neg...........................  OK
<rrd,rld>.....................  OK
<rlca,rrca,rla,rra>...........  OK
shf/rot (<ix,iy>+1)...........  OK
shf/rot <b,c,d,e,h,l,(hl),a>..  OK
<set,res> n,<bcdehl(hl)a>.....  OK
<set,res> n,(<ix,iy>+1).......  OK
ld (<ix,iy>+1),<b,c,d,e>......  OK
ld (<ix,iy>+1),<h,l>..........  OK
ld (<ix,iy>+1),a..............  OK
ld (<bc,de>),a................  OK
Tests complete
```

### 8080 instruction exerciser

```
8080 instruction exerciser (KR580VM80A CPU)
dad <b,d,h,sp>................  OK
aluop nn......................  OK
aluop <b,c,d,e,h,l,m,a>.......  OK
<daa,cma,stc,cmc>.............  OK
<inr,dcr> a...................  OK
<inr,dcr> b...................  OK
<inx,dcx> b...................  OK
<inr,dcr> c...................  OK
<inr,dcr> d...................  OK
<inx,dcx> d...................  OK
<inr,dcr> e...................  OK
<inr,dcr> h...................  OK
<inx,dcx> h...................  OK
<inr,dcr> l...................  OK
<inr,dcr> m...................  OK
<inx,dcx> sp..................  OK
lhld nnnn.....................  OK
shld nnnn.....................  OK
lxi <b,d,h,sp>,nnnn...........  OK
ldax <b,d>....................  OK
mvi <b,c,d,e,h,l,m,a>,nn......  OK
mov <bcdehla>,<bcdehla>.......  OK
sta nnnn / lda nnnn...........  OK
<rlc,rrc,ral,rar>.............  OK
stax <b,d>....................  OK
Tests complete
```

### Pre-commit tests

Note that some of the Zexall tests takes very long and is disabled for continuouos integrations. To run it, execute:

```
cargo test --release -- --nocapture --ignored --test zexall

```

### Current local verification

Run on 2026-07-02:

```text
cargo test
result: ok

cargo test --test z80test -- --ignored --nocapture
result: ok, 12 passed
standard path: z80doc, z80docflags, z80flags, z80full, z80ccf, z80memptr
fast path: z80doc_fast, z80docflags_fast, z80flags_fast, z80full_fast, z80ccf_fast, z80memptr_fast
```

### Fast-path benchmarks

Run on 2026-07-03 with `cargo bench --bench cpu_fast_path`:

Benchmark context:

- CPU: Intel(R) Core(TM) Ultra 9 285, 24 cores / 24 logical processors, 2500 MHz reported max clock
- OS: Microsoft Windows 11 Pro 10.0.26200, 64-bit
- Rust: rustc 1.95.0 (59807616e 2026-04-14), host x86_64-pc-windows-msvc, LLVM 22.1.2
- SDCC: 4.6.0 #16555, target `-mz80`

```text
benchmark                     ref ips     fast ips      ref cps     fast cps   speedup
-----------------------------------------------------------------------------------------
NOP loop                    201775626    273493053    201775626    273493053     1.36x
register ALU loop           118317386    130967226    118317386    130967226     1.11x
memory read/write           150848199    183266219    301691794    366526846     1.21x
branch loop                 130589219    154750851    391767656    464252553     1.19x
ADL load/store               97263283    161657622    486316414    808288111     1.66x
IX/IY indexed                94362851    121733073    377436285    486912789     1.29x
Eratosthenes sieve          113277660    144157429    451512367    574595749     1.27x
Dhrystone 2.1 Z80: 5000 runs, 62789 runs/s, 1399 instructions/run, 3550 cycles/run, 222872908 cycles/s, halt pc=$0208
save/load state                     -            -            -            -     5.4ns
run_cycles loop                     -    256173788            -    256173788        -
```

### Dhrystone

`scripts/bench_dhrystone.sh` builds Dhrystone 2.1 for Z80 with SDCC and writes artifacts under `target/dhrystone-z80` by default. A flat binary built with SDCC 4.6.0 is committed at `benches/res/dhrystone-z80.bin` so consumers can run it without a C compiler. The benchmark harness runs this binary on the Z80 fast path until the SDCC CRT halts at `$0208`.

```text
scripts/bench_dhrystone.sh
Built target/dhrystone-z80/dhrystone-z80.ihx and target/dhrystone-z80/dhrystone-z80.bin
```

