#![allow(dead_code)]
#[macro_use] pub mod kat;

pub mod exe {
    use std::fs::File;
    use std::process::{Command, Output};

    fn exe_cmd() -> Command {
        Command::new("./target/debug/dent")
    }

    pub fn run_with_stdin(f: File, args: &[&str]) -> Output {
        let mut cmd = exe_cmd();

        cmd.args(args)
            .stdin(f)
            .output()
            .expect("Unable to run command in test")
    }

    pub fn run(args: &[&str]) -> Output {
        let mut cmd = exe_cmd();

        cmd.args(args)
            .output()
            .expect("Unable to run command in test")
    }
}

pub mod fs {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};

    pub fn read_data(path: &str) -> Vec<f64> {
        let f = File::open(path).unwrap();
        let r = BufReader::new(f);

        let data: Vec<f64> = r
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect();

        data
    }

    pub fn read_string(path: &str) -> String {
        let mut f = File::open(path).unwrap();

        let mut s = String::new();

        // Fails if file contents are not valid UTF-8.
        f.read_to_string(&mut s).unwrap();

        s
    }
}

pub mod fixture {
    use std::fs::File;

    pub fn path(name: &str) -> String {
        format!("tests/support/fixture/{}", name)
    }

    pub fn file(name: &str) -> File {
        File::open(path(name)).unwrap()
    }

    pub fn read(name: &str) -> String {
        super::fs::read_string(&path(name))
    }
}

pub mod assert {
    use std::process::Output;
    use super::fixture;

    pub fn exit_ok(output: &Output) {
        assert!(output.status.success(), "Expected exit code 0");
    }

    pub fn exit_fail(output: &Output) {
        assert!(!output.status.success(), "Expected nonzero exit code");
    }

    pub fn stdout_eq_file(output: &Output, path: &str) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout, fixture::read(path),
                   "Expected stdout to contain text at {:?}", path);
    }

    pub fn stdout_is_empty(output: &Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.is_empty());
    }

    pub fn stdout_includes(output: &Output, s: &str) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(s), "Expected stdout to contain {:?}", s);
    }

    pub fn stderr_eq_file(output: &Output, path: &str) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(stderr, fixture::read(path),
                   "Expected stderr to contain text at {:?}", path);
    }

    pub fn stderr_is_empty(output: &Output) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.is_empty());
    }

    pub fn stderr_includes(output: &Output, s: &str) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(s), "Expected stderr to contain {:?}", s);
    }
}
