use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_source_file::Line;

use crate::Violation;
use crate::checkers::ast::LintContext;

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct TooManyNotebookCells;
impl Violation for TooManyNotebookCells {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Notebooks should not have more than 75 cells".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct NotebookPercentRun;
impl Violation for NotebookPercentRun {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using %run is not allowed".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct NotebookPercentPip;
impl Violation for NotebookPercentPip {
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
            if context.is_rule_enabled(crate::registry::Rule::TooManyNotebookCells) {
                context.report_diagnostic(TooManyNotebookCells, line.range());
            }
        }
    }
    if text.contains("%run") {
        if context.is_rule_enabled(crate::registry::Rule::NotebookPercentRun) {
            context.report_diagnostic(NotebookPercentRun, line.range());
        }
    }
    if text.contains("%pip") && !text.contains("%uv pip") {
        if context.is_rule_enabled(crate::registry::Rule::NotebookPercentPip) {
            context.report_diagnostic(NotebookPercentPip, line.range());
        }
    }
}
