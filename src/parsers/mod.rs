#![allow(dead_code)]

pub mod test_parser;
pub mod lint_parser;

pub use test_parser::{TestResult, TestStatus};
pub use lint_parser::LintResult;
