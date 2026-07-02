use ez80::*;

// Test binaries from https://github.com/raxoft/z80test.
// These are exhaustive and slow, so they are ignored by default.

const START: u16 = 0x8000;

struct Z80TestCase {
    name: &'static str,
    code: &'static [u8],
    expected_failures: usize,
    expected_total: usize,
}

static Z80DOC: &[u8] = include_bytes!("res/z80doc.out");
static Z80DOCFLAGS: &[u8] = include_bytes!("res/z80docflags.out");
static Z80FLAGS: &[u8] = include_bytes!("res/z80flags.out");
static Z80FULL: &[u8] = include_bytes!("res/z80full.out");
static Z80CCF: &[u8] = include_bytes!("res/z80ccf.out");
static Z80MEMPTR: &[u8] = include_bytes!("res/z80memptr.out");

fn run_z80test(case: Z80TestCase) {
    let mut cpu = Cpu::new_z80();
    let mut machine = PlainMachine::new();

    for (i, byte) in case.code.iter().enumerate() {
        machine.poke(START as u32 + i as u32, *byte);
    }

    machine.poke(0x1601, 0xc9); // RET
    machine.poke(0x0010, 0xc9); // RET
    for high in 0..=0xff {
        machine.port_out((high << 8) | 0x00fe, 0xbf);
    }

    cpu.state.set_pc(START as u32);
    let mut msg = String::new();
    loop {
        cpu.execute_instruction(&mut machine);

        if cpu.state.pc() == 0x0000 {
            break;
        }

        if cpu.state.pc() == 0x0010 {
            let mut ch = cpu.registers().get8(Reg8::A) as char;
            if ch == '\r' {
                ch = '\n'
            } else if ch as u8 == 23 || ch as u8 == 26 {
                ch = ' '
            }
            msg.push(ch);
        }
    }

    let (failures, total) = parse_result(&msg);
    assert_eq!(
        (case.expected_failures, case.expected_total),
        (failures, total),
        "{}\n{}",
        case.name,
        msg
    );
}

fn parse_result(msg: &str) -> (usize, usize) {
    if msg.contains("CPU TESTS OK") {
        return (0, 0);
    }

    let result = msg
        .lines()
        .find(|line| line.contains("Result:"))
        .expect("z80test output did not contain a Result line");
    let mut numbers = result
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<usize>().expect("numeric result field"));
    let failures = numbers.next().expect("failure count");
    let total = numbers.next().expect("total count");
    (failures, total)
}

#[test]
#[ignore]
fn z80doc() {
    run_z80test(Z80TestCase {
        name: "z80doc",
        code: Z80DOC,
        expected_failures: 0,
        expected_total: 0,
    });
}

#[test]
#[ignore]
fn z80docflags() {
    run_z80test(Z80TestCase {
        name: "z80docflags",
        code: Z80DOCFLAGS,
        expected_failures: 0,
        expected_total: 0,
    });
}

#[test]
#[ignore]
fn z80flags() {
    run_z80test(Z80TestCase {
        name: "z80flags",
        code: Z80FLAGS,
        expected_failures: 0,
        expected_total: 0,
    });
}

#[test]
#[ignore]
fn z80full() {
    run_z80test(Z80TestCase {
        name: "z80full",
        code: Z80FULL,
        expected_failures: 15,
        expected_total: 152,
    });
}

#[test]
#[ignore]
fn z80ccf() {
    run_z80test(Z80TestCase {
        name: "z80ccf",
        code: Z80CCF,
        expected_failures: 0,
        expected_total: 0,
    });
}

#[test]
#[ignore]
fn z80memptr() {
    run_z80test(Z80TestCase {
        name: "z80memptr",
        code: Z80MEMPTR,
        expected_failures: 0,
        expected_total: 0,
    });
}
