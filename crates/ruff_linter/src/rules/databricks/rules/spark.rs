use ruff_diagnostics::{Edit, Fix};
use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use crate::{FixAvailability, Violation};

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
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Rewrite to display in a notebook: display(...)".to_string()
    }
    fn fix_title(&self) -> Option<String> {
        Some("Replace with `display(...)`".to_string())
    }
}

/// DBX015, DBX016
pub(crate) fn spark_name(checker: &Checker, name: &ast::ExprName) {
    if name.id.as_str() != "spark" {
        return;
    }
    if !name.ctx.is_load() {
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
        if let ast::Expr::Name(ast::ExprName { id, .. }) = attribute.value.as_ref() {
            let id_str = id.as_str();
            if id_str == "df"
                || id_str == "dataframe"
                || id_str == "dataset"
                || id_str.ends_with("_df")
            {
                if checker.is_rule_enabled(crate::registry::Rule::SparkDataFrameShow) {
                    let mut diagnostic =
                        checker.report_diagnostic(SparkDataFrameShow, attribute.range());
                    let parent = checker.semantic().current_expression_parent();
                    if let Some(ast::Expr::Call(call)) = parent {
                        if call.arguments.args.is_empty() && call.arguments.keywords.is_empty() {
                            let replacement = format!("display({})", id_str);
                            diagnostic.set_fix(Fix::safe_edit(Edit::range_replacement(
                                replacement,
                                call.range(),
                            )));
                        }
                    }
                }
            }
        }
    }
}
