use ez80::*;

const MOS_3_0_2: &[u8] = include_bytes!("res/agon/mos-3.0.2.bin");
const VDP_2_16_0_BOOTLOADER: &[u8] = include_bytes!("res/agon/vdp-2.16.0-bootloader.bin");
const VDP_2_16_0_FIRMWARE: &[u8] = include_bytes!("res/agon/vdp-2.16.0-firmware.bin");
const VDP_2_16_0_PARTITIONS: &[u8] = include_bytes!("res/agon/vdp-2.16.0-partitions.bin");

#[test]
fn agon_release_binaries_are_vendored() {
    assert_eq!(108_490, MOS_3_0_2.len());
    assert_eq!(18_992, VDP_2_16_0_BOOTLOADER.len());
    assert_eq!(1_077_840, VDP_2_16_0_FIRMWARE.len());
    assert_eq!(3_072, VDP_2_16_0_PARTITIONS.len());

    assert_eq!(&[0xf3, 0xed, 0x7d, 0x5b], &MOS_3_0_2[..4]);
    assert_eq!(&[0xe9, 0x03, 0x02, 0x2f], &VDP_2_16_0_BOOTLOADER[..4]);
    assert_eq!(&[0xe9, 0x06, 0x02, 0x2f], &VDP_2_16_0_FIRMWARE[..4]);
    assert_eq!(&[0xaa, 0x50, 0x01, 0x02], &VDP_2_16_0_PARTITIONS[..4]);
}

#[test]
fn agon_mos_reset_prologue_runs_on_ez80() {
    let mut machine = PlainMachine::new();
    for (address, byte) in MOS_3_0_2.iter().enumerate() {
        machine.poke(address as u32, *byte);
    }

    let mut cpu = Cpu::new_ez80();
    cpu.set_adl(true);

    cpu.execute_instruction(&mut machine);
    assert_eq!(1, cpu.state.pc());

    cpu.execute_instruction(&mut machine);
    assert_eq!(3, cpu.state.pc());

    cpu.execute_instruction(&mut machine);
    assert_eq!(0x0017cf, cpu.state.pc());
}
