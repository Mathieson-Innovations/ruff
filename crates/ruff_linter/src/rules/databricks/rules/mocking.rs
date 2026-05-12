use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_python_ast::Expr;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct ExplicitDependencyRequired;
impl Violation for ExplicitDependencyRequired {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Obscure implicit test dependency with mock.patch. Rewrite to inject dependencies through constructor.".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct ObscureMock;
impl Violation for ObscureMock {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Obscure implicit test dependency with MagicMock(). Rewrite with create_autospec(ConcreteType).".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct MockNoAssign;
impl Violation for MockNoAssign {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Mock not assigned to a variable".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct MockNoUsage;
impl Violation for MockNoUsage {
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
        if checker.is_rule_enabled(crate::registry::Rule::ObscureMock) {
            checker.report_diagnostic(ObscureMock, call.func.range());
        }
    }

    if func_name == "patch" && !call.arguments.args.is_empty() {
        if let Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) = &call.arguments.args[0] {
            if value.to_str().starts_with("databricks") {
                if checker.is_rule_enabled(crate::registry::Rule::ExplicitDependencyRequired) {
                    checker.report_diagnostic(ExplicitDependencyRequired, call.func.range());
                }
            }
        }
    }

    if matches!(func_name, "MagicMock" | "create_autospec" | "patch") {
        let parent = checker.semantic().current_statement();
        if !matches!(parent, ast::Stmt::Assign(_) | ast::Stmt::AnnAssign(_)) {
            if checker.is_rule_enabled(crate::registry::Rule::MockNoAssign) {
                checker.report_diagnostic(MockNoAssign, call.range());
            }
        }
    }
}
