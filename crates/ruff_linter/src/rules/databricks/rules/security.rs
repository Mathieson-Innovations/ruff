use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct PatTokenLeaked;

impl Violation for PatTokenLeaked {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not hardcode secrets in code, use Databricks SDK instead".to_string()
    }
}

/// DBX009
pub(crate) fn pat_token_leaked(checker: &Checker, string_literal: &ast::ExprStringLiteral) {
    let value = string_literal.value.to_str();
    if value.starts_with("dapi") || value.starts_with("dkea") || value.starts_with("dosa") {
        checker.report_diagnostic(PatTokenLeaked, string_literal.range());
    }
}
