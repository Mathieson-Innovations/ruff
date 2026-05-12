use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct RewriteAsForLoop;
impl Violation for RewriteAsForLoop {
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
        if checker.is_rule_enabled(crate::registry::Rule::RewriteAsForLoop) {
            checker.report_diagnostic(RewriteAsForLoop, range);
        }
    }
}
