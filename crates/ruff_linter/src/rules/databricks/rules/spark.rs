use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct SparkOutsideFunction;
impl Violation for SparkOutsideFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using spark outside the function is leading to untestable code".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct UnpassedSparkReference;
impl Violation for UnpassedSparkReference {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Function refers to a global spark variable, which may not always be available".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct SparkDataFrameShow;
impl Violation for SparkDataFrameShow {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Rewrite to display in a notebook: display(...)".to_string()
    }
}

/// DBX015, DBX016
pub(crate) fn spark_name(checker: &Checker, name: &ast::ExprName) {
    if name.id.as_str() != "spark" {
        return;
    }

    let scope = checker.semantic().current_scope();
    if scope.kind.is_module() {
        if checker.is_rule_enabled(crate::registry::Rule::SparkOutsideFunction) {
            checker.report_diagnostic(SparkOutsideFunction, name.range());
        }
    } else if scope.kind.is_function() {
        // Check if 'spark' is an argument
        let has_spark_arg = checker
            .semantic()
            .current_scope()
            .get("spark")
            .is_some_and(|id| checker.semantic().binding(id).kind.is_argument());
        if !has_spark_arg {
            if checker.is_rule_enabled(crate::registry::Rule::UnpassedSparkReference) {
                checker.report_diagnostic(UnpassedSparkReference, name.range());
            }
        }
    }
}

/// DBX017
pub(crate) fn spark_show(checker: &Checker, attribute: &ast::ExprAttribute) {
    if attribute.attr.as_str() == "show" {
        if checker.is_rule_enabled(crate::registry::Rule::SparkDataFrameShow) {
            checker.report_diagnostic(SparkDataFrameShow, attribute.range());
        }
    }
}
