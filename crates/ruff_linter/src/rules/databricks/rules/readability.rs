use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct MultilineListComprehension;
impl Violation for MultilineListComprehension {
    #[derive_message_formats]
    fn message(&self) -> String {
        "List comprehension spans multiple lines, rewrite as for loop".to_string()
    }
}

/// DBX023
pub(crate) fn rewrite_as_for_loop(checker: &Checker, list_comp: &ast::ExprListComp) {
    let locator = checker.locator();
    let range = list_comp.range();
    let contents = locator.slice(range);
    if contents.contains('\n') || contents.contains('\r') {
        if checker.is_rule_enabled(crate::registry::Rule::MultilineListComprehension) {
            checker.report_diagnostic(MultilineListComprehension, range);
        }
    }
}
