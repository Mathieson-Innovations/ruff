use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_source_file::Line;

use crate::Violation;
use crate::checkers::ast::LintContext;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct NotebooksTooManyCells;
impl Violation for NotebooksTooManyCells {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Notebooks should not have more than 75 cells".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct NotebooksPercentRun;
impl Violation for NotebooksPercentRun {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using %run is not allowed".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct NotebooksPercentPip;
impl Violation for NotebooksPercentPip {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `%uv pip` instead of `%pip` for faster dependency installation on Databricks"
            .to_string()
    }
}

pub(crate) fn notebooks(line: &Line, context: &LintContext, cell_count: &mut usize) {
    let text = line.as_str();
    if text.contains("# Databricks notebook source") {
        // This is a Databricks notebook
    }
    if text.contains("# COMMAND ----------") {
        *cell_count += 1;
        if *cell_count > 75 {
            if context.is_rule_enabled(crate::registry::Rule::NotebooksTooManyCells) {
                context.report_diagnostic(NotebooksTooManyCells, line.range());
            }
        }
    }
    if text.contains("%run") {
        if context.is_rule_enabled(crate::registry::Rule::NotebooksPercentRun) {
            context.report_diagnostic(NotebooksPercentRun, line.range());
        }
    }
    if text.contains("%pip") && !text.contains("%uv pip") {
        if context.is_rule_enabled(crate::registry::Rule::NotebooksPercentPip) {
            context.report_diagnostic(NotebooksPercentPip, line.range());
        }
    }
}
