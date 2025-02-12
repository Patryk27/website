use std::io;
use std::io::{Cursor, Read, Write};

pub struct Env<'a> {
    pub stdin: &'a mut dyn Read,
    pub stdout: &'a mut dyn Write,
    pub stderr: &'a mut dyn Write,
}

impl Env<'_> {
    pub fn with<T>(run: impl FnOnce(&mut Env) -> T) -> T {
        let mut stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        let mut stderr = io::stderr().lock();

        let mut this = Env {
            stdin: &mut stdin,
            stdout: &mut stdout,
            stderr: &mut stderr,
        };

        run(&mut this)
    }

    pub fn mock<T>(
        stdin: String,
        run: impl FnOnce(&mut Env) -> T,
    ) -> (T, String, String) {
        let mut stdin = Cursor::new(stdin);
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let result = run(&mut Env {
            stdin: &mut stdin,
            stdout: &mut stdout,
            stderr: &mut stderr,
        });

        let stdout = String::from_utf8(stdout).unwrap();
        let stderr = String::from_utf8(stderr).unwrap();

        (result, stdout, stderr)
    }
}
