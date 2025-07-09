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
use crate::utils::Env;
use anyhow::{Context, Result as AResult, anyhow};
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Cmd;

impl Cmd {
    pub fn run(self, env: &mut Env) -> AResult<()> {
        Self::run_ex(env, None)
    }

    fn run_ex(env: &mut Env, out_nodes: Option<&mut Vec<Node>>) -> AResult<()> {
        let mut input = String::new();
        let mut output = String::new();

        env.stdin
            .read_to_string(&mut input)
            .context("couldn't read from stdin")?;

        let mut printer = Printer::new(&mut output);

        let result: Result<()> = try {
            let nodes: Vec<_> = Scanner::new(&input).collect::<Result<_>>()?;

            if let Some(out_nodes) = out_nodes {
                *out_nodes = nodes.clone();
            }

            for node in &nodes {
                printer.scan(node);
            }
            for node in nodes {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Asserter;
    use itertools::Itertools;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;
    use test_case::test_case;

    #[test_case("err-code-with-two-fragments")]
    #[test_case("err-mismatched-closing-tag")]
    #[test_case("err-overwritten-ref")]
    #[test_case("err-unclosed-tag")]
    #[test_case("err-unused-ref-1")]
    #[test_case("err-unused-ref-2")]
    #[test_case("ok-a")]
    #[test_case("ok-a-with-implied-href")]
    #[test_case("ok-code")]
    #[test_case("ok-hdr")]
    #[test_case("ok-img")]
    #[test_case("ok-note")]
    #[test_case("ok-ref")]
    #[test_case("ok-shdr")]
    #[test_case("ok-smoke")]
    #[test_case("ok-toc")]
    #[test_case("ok-video")]
    fn test(case: &str) {
        let dir = Path::new("src")
            .join("render_post")
            .join("tests")
            .join(case);

        let stdin = fs::read_to_string(dir.join("given.stdin")).unwrap();
        let mut nodes = Vec::new();

        let (result, stdout, mut stderr) =
            Env::mock(stdin, |env| Cmd::run_ex(env, Some(&mut nodes)));

        let nodes = nodes.iter().map(|node| format!("{node:#?}")).join("\n");

        if let Err(err) = result {
            _ = writeln!(&mut stderr, "{err:?}");
        }

        Asserter::new(dir)
            .assert("expected.nodes", nodes)
            .assert("expected.stdout", stdout)
            .assert("expected.stderr", stderr)
            .finish();
    }
}
