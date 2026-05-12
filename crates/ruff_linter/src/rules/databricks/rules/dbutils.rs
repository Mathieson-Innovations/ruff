use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_python_ast::Expr;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct DbutilsFsCp;
impl Violation for DbutilsFsCp {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use Databricks SDK instead: w.dbfs.copy(...)".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct DbutilsFsHead;
impl Violation for DbutilsFsHead {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use Databricks SDK instead: with w.dbfs.download(...) as f: f.read()".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct DbutilsFsLs;
impl Violation for DbutilsFsLs {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use Databricks SDK instead: w.dbfs.list(...)".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct DbutilsFsMount;
impl Violation for DbutilsFsMount {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Mounts are not supported with Unity Catalog, switch to using Unity Catalog Volumes instead"
            .to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct DbutilsCredentials;
impl Violation for DbutilsCredentials {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Credentials utility is not supported with Unity Catalog".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct DbutilsNotebookRun;
impl Violation for DbutilsNotebookRun {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use Databricks SDK instead: w.jobs.submit(...)".to_string()
    }
}

/// DBX003, DBX004, DBX005, DBX006, DBX007, DBX008
pub(crate) fn dbutils_call(checker: &Checker, call: &ast::ExprCall) {
    let Expr::Attribute(ast::ExprAttribute { value, attr, .. }) = call.func.as_ref() else {
        return;
    };

    // Check for dbutils.credentials.*
    if let Expr::Attribute(ast::ExprAttribute {
        value: inner_value,
        attr: inner_attr,
        ..
    }) = value.as_ref()
    {
        if inner_attr.as_str() == "credentials" {
            if let Expr::Name(ast::ExprName { id, .. }) = inner_value.as_ref() {
                if id.as_str() == "dbutils" {
                    if checker.is_rule_enabled(crate::registry::Rule::DbutilsCredentials) {
                        checker.report_diagnostic(DbutilsCredentials, call.func.range());
                    }
                    return;
                }
            }
        }
    }

    // Check for dbutils.fs.* and dbutils.notebook.*
    let Expr::Attribute(ast::ExprAttribute {
        value: inner_value,
        attr: inner_attr,
        ..
    }) = value.as_ref()
    else {
        return;
    };
    let Expr::Name(ast::ExprName { id, .. }) = inner_value.as_ref() else {
        return;
    };
    if id.as_str() != "dbutils" {
        return;
    }

    match inner_attr.as_str() {
        "fs" => match attr.as_str() {
            "cp" => {
                if checker.is_rule_enabled(crate::registry::Rule::DbutilsFsCp) {
                    checker.report_diagnostic(DbutilsFsCp, call.func.range());
                }
            }
            "head" => {
                if checker.is_rule_enabled(crate::registry::Rule::DbutilsFsHead) {
                    checker.report_diagnostic(DbutilsFsHead, call.func.range());
                }
            }
            "ls" => {
                if checker.is_rule_enabled(crate::registry::Rule::DbutilsFsLs) {
                    checker.report_diagnostic(DbutilsFsLs, call.func.range());
                }
            }
            "mount" | "mounts" | "unmount" | "updateMount" | "refreshMounts" => {
                if checker.is_rule_enabled(crate::registry::Rule::DbutilsFsMount) {
                    checker.report_diagnostic(DbutilsFsMount, call.func.range());
                }
            }
            _ => {}
        },
        "notebook" => {
            if attr.as_str() == "run" {
                if checker.is_rule_enabled(crate::registry::Rule::DbutilsNotebookRun) {
                    checker.report_diagnostic(DbutilsNotebookRun, call.func.range());
                }
            }
        }
        _ => {}
    }
}
