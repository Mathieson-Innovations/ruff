use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_python_ast::Expr;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct MissingDataSecurityMode;
impl Violation for MissingDataSecurityMode {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Cluster missing `data_security_mode` required for Unity Catalog compatibility".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct UnsupportedRuntime;
impl Violation for UnsupportedRuntime {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Cluster has unsupported runtime".to_string()
    }
}

/// DBX001, DBX002
pub(crate) fn airflow_call(checker: &Checker, call: &ast::ExprCall) {
    let func_name = match call.func.as_ref() {
        Expr::Name(ast::ExprName { id, .. }) => id.as_str(),
        Expr::Attribute(ast::ExprAttribute { attr, .. }) => attr.as_str(),
        _ => return,
    };

    if !matches!(
        func_name,
        "DatabricksCreateJobsOperator" | "DatabricksSubmitRunOperator"
    ) {
        return;
    }

    for keyword in &call.arguments.keywords {
        let arg = keyword
            .arg
            .as_ref()
            .map(ruff_python_ast::Identifier::as_str);
        if matches!(arg, Some("new_cluster" | "job_clusters" | "tasks")) {
            check_cluster_config(checker, &keyword.value);
        }
    }
}

fn check_cluster_config(checker: &Checker, expr: &Expr) {
    match expr {
        Expr::Dict(ast::ExprDict { items, .. }) => {
            let mut has_data_security_mode = false;
            for item in items {
                if let Some(key) = &item.key {
                    if let Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) = key {
                        let key_str = value.to_str();
                        if key_str == "data_security_mode" {
                            has_data_security_mode = true;
                        } else if key_str == "spark_version" {
                            if let Expr::StringLiteral(ast::ExprStringLiteral {
                                value: spark_version,
                                ..
                            }) = &item.value
                            {
                                if !is_supported_runtime(spark_version.to_str()) {
                                    if checker
                                        .is_rule_enabled(crate::registry::Rule::UnsupportedRuntime)
                                    {
                                        checker.report_diagnostic(
                                            UnsupportedRuntime,
                                            item.value.range(),
                                        );
                                    }
                                }
                            }
                        } else if key_str == "new_cluster" || key_str == "job_cluster" {
                            check_cluster_config(checker, &item.value);
                        }
                    }
                }
            }
            if !has_data_security_mode
                && checker.is_rule_enabled(crate::registry::Rule::MissingDataSecurityMode)
            {
                checker.report_diagnostic(MissingDataSecurityMode, expr.range());
            }
        }
        Expr::List(ast::ExprList { elts, .. }) => {
            for elt in elts {
                check_cluster_config(checker, elt);
            }
        }
        _ => {}
    }
}

fn is_supported_runtime(spark_version: &str) -> bool {
    let split: Vec<&str> = spark_version.split('-').collect();
    if split.is_empty() {
        return false;
    }
    let digits: Vec<&str> = split[0].split('.').collect();
    if digits.len() < 2 {
        return false;
    }
    let major = digits[0].parse::<i32>().unwrap_or(0);
    let minor = digits[1].parse::<i32>().unwrap_or(0);
    (major, minor) >= (11, 3)
}
