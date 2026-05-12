use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_python_ast::Expr;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct ImplicitMockDependency;
impl Violation for ImplicitMockDependency {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Obscure implicit test dependency with mock.patch. Rewrite to inject dependencies through constructor.".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct MagicMockUsage;
impl Violation for MagicMockUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Obscure implicit test dependency with MagicMock(). Rewrite with create_autospec(ConcreteType).".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct UnassignedMock;
impl Violation for UnassignedMock {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Mock not assigned to a variable".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(preview_since = "NEXT_RUFF_VERSION")]
pub(crate) struct UnusedMock;
impl Violation for UnusedMock {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing usage of mock".to_string()
    }
}

/// DBX018, DBX019, DBX021, DBX022
pub(crate) fn mocking(checker: &Checker, call: &ast::ExprCall) {
    let func_name = match call.func.as_ref() {
        Expr::Name(ast::ExprName { id, .. }) => id.as_str(),
        Expr::Attribute(ast::ExprAttribute { attr, .. }) => attr.as_str(),
        _ => return,
    };

    if func_name == "MagicMock"
        && call.arguments.args.is_empty()
        && call.arguments.keywords.is_empty()
    {
        if checker.is_rule_enabled(crate::registry::Rule::MagicMockUsage) {
            checker.report_diagnostic(MagicMockUsage, call.func.range());
        }
    }

    if func_name == "patch" && !call.arguments.args.is_empty() {
        if let Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) = &call.arguments.args[0] {
            if value.to_str().starts_with("databricks") {
                if checker.is_rule_enabled(crate::registry::Rule::ImplicitMockDependency) {
                    checker.report_diagnostic(ImplicitMockDependency, call.func.range());
                }
            }
        }
    }

    if matches!(func_name, "MagicMock" | "create_autospec" | "patch") {
        let parent = checker.semantic().current_statement();
        if !matches!(parent, ast::Stmt::Assign(_) | ast::Stmt::AnnAssign(_)) {
            if checker.is_rule_enabled(crate::registry::Rule::UnassignedMock) {
                checker.report_diagnostic(UnassignedMock, call.range());
            }
        }
    }
}
