use pretty_assertions::StrComparison;
use std::fs;
use std::path::{Path, PathBuf};

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
