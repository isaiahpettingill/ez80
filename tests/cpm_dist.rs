use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[test]
#[ignore]
fn cpm_dist_hello_runs() {
    let root = cpm_dist_root();
    let output = run_cpm(&root, "M/hello.com", "M", "", None);
    assert!(output.contains("Hello world!"), "{}", output);
}

#[test]
#[ignore]
fn cpm_dist_help_reads_host_file() {
    let root = cpm_dist_root();
    let output = run_cpm(&root, "H/HELP.COM", "H", "PIP", Some(b"A"));
    assert!(output.contains("HELP V1.1"), "{}", output);
    assert!(output.contains("PIP Command"), "{}", output);
}

#[test]
#[ignore]
fn cpm_dist_stat_queries_disk_state() {
    let root = cpm_dist_root();
    let output = run_cpm(&root, "A/STAT.COM", "A", "", None);
    assert!(output.contains("SPACE"), "{}", output);
}

fn cpm_dist_root() -> PathBuf {
    std::env::var_os("CPM_DIST_DIR")
        .map(PathBuf::from)
        .expect("set CPM_DIST_DIR to a skx/cpm-dist checkout")
}

fn run_cpm(
    root: &Path,
    program: &str,
    host_drive: &str,
    tail: &str,
    stdin: Option<&[u8]>,
) -> String {
    let mut command = Command::new("cargo");
    command
        .args(["run", "--quiet", "--example", "cpm", "--", "--program"])
        .arg(root.join(program))
        .arg("--host-dir")
        .arg(root.join(host_drive))
        .args(["--cpu", "z80", "--max-steps", "10000000"]);

    if !tail.is_empty() {
        command.args(["--tail", tail]);
    }
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn cpm example");

    if let Some(input) = stdin {
        child
            .stdin
            .as_mut()
            .expect("stdin was not piped")
            .write_all(input)
            .expect("failed to write child stdin");
    }

    let output = child.wait_with_output().expect("failed to wait for cpm");
    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "{}", combined);
    combined
}
