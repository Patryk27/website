#![feature(extract_if)]
#![feature(try_blocks)]

mod dom;
mod msg;
mod printer;
mod scanner;

use self::dom::*;
use self::msg::*;
use self::printer::*;
use self::scanner::*;
use anyhow::{anyhow, Context, Result};
use common::Env;

fn main() -> Result<()> {
    Env::with(|env| main_ex(env, None))
}

fn main_ex(env: &mut Env, mut nodes: Option<&mut Vec<Node>>) -> Result<()> {
    let mut input = String::new();
    let mut output = String::new();

    env.stdin
        .read_to_string(&mut input)
        .context("couldn't read from stdin")?;

    let mut printer = Printer::new(&mut output);

    let result: Result<()> = try {
        for node in Scanner::new(&input) {
            let node = node?;

            if let Some(nodes) = &mut nodes {
                nodes.push(node.clone());
            }

            printer.process(node)?;
        }

        printer.finish()?;
    };

    if let Err(err) = result {
        return match err.downcast() {
            Ok(msg) => {
                MessageReporter::new(&mut env.stderr, &input).report(msg)?;

                env.stderr.write_all(b"\n")?;

                Err(anyhow!("compilation failed"))
            }

            Err(err) => Err(err),
        };
    }

    env.stdout.write_all(output.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Asserter;
    use itertools::Itertools;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;
    use test_case::test_case;

    #[test_case("err-listing-with-two-fragments")]
    #[test_case("err-mismatched-closing-tag")]
    #[test_case("err-overwritten-ref")]
    #[test_case("err-unclosed-tag")]
    #[test_case("err-unused-ref-1")]
    #[test_case("err-unused-ref-2")]
    #[test_case("ok-a")]
    #[test_case("ok-a-with-implied-href")]
    #[test_case("ok-header")]
    #[test_case("ok-img")]
    #[test_case("ok-listing")]
    #[test_case("ok-note")]
    #[test_case("ok-ref")]
    #[test_case("ok-smoke")]
    fn test(case: &str) {
        let dir = Path::new("src").join("tests").join(case);
        let stdin = fs::read_to_string(dir.join("given.stdin")).unwrap();
        let mut nodes = Vec::new();

        let (result, stdout, mut stderr) =
            Env::mock(stdin, |env| main_ex(env, Some(&mut nodes)));

        let nodes = nodes.iter().map(|node| format!("{:#?}", node)).join("\n");

        if let Err(err) = result {
            _ = writeln!(&mut stderr, "{:?}", err);
        }

        Asserter::new(dir)
            .assert("expected.stdout", stdout)
            .assert("expected.stderr", stderr)
            .assert("expected.nodes", nodes)
            .finish();
    }
}
