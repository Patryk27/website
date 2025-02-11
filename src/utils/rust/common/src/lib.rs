use pretty_assertions::StrComparison;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

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

pub struct Asserter {
    dir: PathBuf,
    failed: bool,
}

impl Asserter {
    pub fn new(dir: impl AsRef<Path>) -> Self {
        Self {
            dir: dir.as_ref().to_owned(),
            failed: false,
        }
    }

    pub fn assert(
        &mut self,
        file: impl AsRef<str>,
        actual: impl AsRef<str>,
    ) -> &mut Self {
        let file = file.as_ref();
        let actual = actual.as_ref();

        let expected_path = self.dir.join(file);
        let expected_new_path = self.dir.join(format!("{file}.new"));

        let expected = fs::read_to_string(&expected_path).unwrap_or_default();

        if *actual == expected {
            _ = fs::remove_file(&expected_new_path);

            if expected.is_empty() {
                _ = fs::remove_file(expected_path);
            }
        } else {
            self.failed = true;

            _ = fs::write(&expected_new_path, actual);

            eprintln!("{}", StrComparison::new(&expected, actual));
        }

        self
    }

    pub fn finish(&self) {
        if self.failed {
            panic!("some assertions failed");
        }
    }
}
