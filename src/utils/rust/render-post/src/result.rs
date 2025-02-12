use crate::Span;
use ariadne::{Color, Label, Report, ReportKind, Source};
use std::fmt;
use std::io::Write;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    body: String,
    span: Span,
    labels: Vec<(String, Span)>,
}

impl Error {
    pub fn new(body: impl ToString, span: Span) -> Self {
        Self {
            body: body.to_string(),
            span,
            labels: Default::default(),
        }
    }

    pub fn with_label(mut self, body: impl ToString, span: Span) -> Self {
        self.labels.push((body.to_string(), span));
        self
    }
}

impl std::error::Error for Error {
    //
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

pub struct ErrorReporter<'a, 'b> {
    out: &'a mut dyn Write,
    src: &'b str,
}

impl<'a, 'b> ErrorReporter<'a, 'b> {
    pub fn new(out: &'a mut dyn Write, src: &'b str) -> Self {
        Self { out, src }
    }

    pub fn report(&mut self, msg: Error) -> anyhow::Result<()> {
        let mut report = Report::build(
            ReportKind::Custom("error", Color::Red),
            "input",
            msg.span.beg,
        )
        .with_message(&msg.body)
        .with_label(
            Label::new(("input", msg.span.beg..(msg.span.end + 1)))
                .with_message(&msg.body),
        );

        for (label_body, label_span) in msg.labels {
            report = report.with_label(
                Label::new(("input", label_span.beg..(label_span.end + 1)))
                    .with_message(label_body),
            );
        }

        #[cfg(test)]
        {
            report = report
                .with_config(ariadne::Config::default().with_color(false));
        }

        report
            .finish()
            .write(("input", Source::from(self.src)), &mut self.out)?;

        Ok(())
    }
}
