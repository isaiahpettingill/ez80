/// Fast-path bus API for CPU-only execution.
///
/// Implementations can track writes in `write8` for rollback/dirty-page logic.
/// The CPU calls `add_cycles` with the deterministic net cycles consumed by an
/// instruction; memory and I/O accesses are still visible through the typed
/// read/write/input/output methods.
pub trait FastBus {
    fn read8(&mut self, addr: u32) -> u8;
    fn write8(&mut self, addr: u32, value: u8);
    fn input8(&mut self, port: u16) -> u8;
    fn output8(&mut self, port: u16, value: u8);
    fn add_cycles(&mut self, cycles: u32);
}
