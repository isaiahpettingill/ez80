use std::fs;
use std::path::{Path, PathBuf};

use ez80::*;

const STARTUP_INSTRUCTIONS: usize = 10_000;

/// Decodes and executes representative Mooneye CPU test-ROM startup paths.
///
/// Download and unpack v7.0 of c-sp/game-boy-test-roms, then run with:
/// `GAME_BOY_TEST_ROMS_DIR=/path/to/game-boy-test-roms-v7.0 \
///  cargo test --test gameboy_test_roms -- --ignored`
///
/// Full Mooneye pass/fail validation additionally needs a Game Boy `Machine`
/// implementation that provides timer, interrupt-controller, and PPU behavior.
#[test]
#[ignore]
fn mooneye_cpu_roms_execute_without_illegal_opcodes() {
    let root = game_boy_test_roms_root();
    for rom in [
        "mooneye-test-suite/acceptance/add_sp_e_timing.gb",
        "mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb",
        "mooneye-test-suite/acceptance/pop_timing.gb",
        "mooneye-test-suite/acceptance/push_timing.gb",
        "mooneye-test-suite/acceptance/jp_timing.gb",
        "mooneye-test-suite/acceptance/call_timing.gb",
        "mooneye-test-suite/acceptance/call_timing2.gb",
        "mooneye-test-suite/acceptance/jp_cc_timing.gb",
    ] {
        run_rom_startup(&root.join(rom));
    }
}

fn game_boy_test_roms_root() -> PathBuf {
    std::env::var_os("GAME_BOY_TEST_ROMS_DIR")
        .map(PathBuf::from)
        .expect("set GAME_BOY_TEST_ROMS_DIR to an unpacked c-sp/game-boy-test-roms release")
}

fn run_rom_startup(path: &Path) {
    let rom =
        fs::read(path).unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    assert!(
        rom.len() <= 0x1_0000,
        "ROM exceeds the plain 64 KiB test bus"
    );

    let mut machine = PlainMachine::new();
    for (address, byte) in rom.into_iter().enumerate() {
        machine.poke(address as u32, byte);
    }

    let mut cpu = Cpu::new_gameboy();
    cpu.state.set_pc(0x0100);
    for _ in 0..STARTUP_INSTRUCTIONS {
        assert!(
            !cpu.state.illegal_instruction,
            "illegal instruction at ${:04x} while running {}",
            cpu.state.pc(),
            path.display()
        );
        cpu.execute_instruction(&mut machine);
    }
    assert!(
        !cpu.state.illegal_instruction,
        "illegal instruction at ${:04x} while running {}",
        cpu.state.pc(),
        path.display()
    );
}
