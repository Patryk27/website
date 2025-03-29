#![feature(try_blocks)]

mod node;
mod printer;
mod result;
mod scanner;
mod span;

use self::node::*;
use self::printer::*;
use self::result::*;
use self::scanner::*;
use self::span::*;
use anyhow::{Context, Result as AResult, anyhow};
use common::Env;

fn main() -> AResult<()> {
    Env::with(|env| main_ex(env, None))
}

fn main_ex(env: &mut Env, mut nodes: Option<&mut Vec<Node>>) -> AResult<()> {
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

            printer.add(node)?;
        }

        printer.finish()?;
    };

    if let Err(err) = result {
        ErrorReporter::new(&mut env.stderr, &input).report(err)?;

        env.stderr.write_all(b"\n")?;

        return Err(anyhow!("compilation failed"));
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
    #[test_case("ok-h2")]
    #[test_case("ok-img")]
    #[test_case("ok-listing")]
    #[test_case("ok-note")]
    #[test_case("ok-ref")]
    #[test_case("ok-smoke")]
    #[test_case("ok-video")]
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
