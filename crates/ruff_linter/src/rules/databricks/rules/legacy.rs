use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_python_ast::{Expr, Stmt};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct InternalApi;
impl Violation for InternalApi {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not use internal APIs, rewrite using Databricks SDK".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct LegacyCli;
impl Violation for LegacyCli {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Don't use databricks_cli, use databricks.sdk instead".to_string()
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.1.0")]
pub(crate) struct IncompatibleWithUc;
impl Violation for IncompatibleWithUc {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Incompatible with Unity Catalog".to_string()
    }
}

const UC_INCOMPATIBLE_BRUTE_FORCE: &[&str] = &[
    "s3fs",
    "boto3",
    "graphframes",
    "pyspark.ml",
    "dbfs:",
    "hive_metastore.",
    "kafka.sasl.client.callback.handler.class",
    "kafka.sasl.login.callback.handler.class",
    "kafka.sasl.login.class",
    "kafka.partition.assignment.strategy",
    "kafka.ssl.truststore.location",
    "kafka.ssl.keystore.location",
    "spark.catalog.",
    "spark._jsparkSession.catalog",
    "spark._jspark",
    "spark._jvm",
    "._jdf",
    "._jcol",
    "spark.udf.registerJavaFunction",
    "applyInPandas",
    "mapInPandas",
    "_jvm",
    "SQLContext",
    "emptyRDD",
    "pickleFile",
    "textFile",
    "newAPIHadoopFile",
    "newAPIHadoopRDD",
    "hadoopFile",
    "hadoopRDD",
    "saveAsHadoopFile",
    "saveAsHadoopDataset",
    "saveAsNewAPIHadoopFile",
    "saveAsNewAPIHadoopDataset",
    "setJobGroup",
    "setLocalProperty",
    "applyInPandasWithState",
];

/// DBX011, DBX012, DBX010
pub(crate) fn import(checker: &Checker, stmt: &Stmt) {
    let names = match stmt {
        Stmt::Import(ast::StmtImport { names, .. }) => names,
        Stmt::ImportFrom(ast::StmtImportFrom { module, names, .. }) => {
            if let Some(module) = module {
                let module_str = module.as_str();
                if module_str.starts_with("databricks_cli") {
                    if checker.is_rule_enabled(crate::registry::Rule::LegacyCli) {
                        checker.report_diagnostic(LegacyCli, stmt.range());
                    }
                }
                if module_str.starts_with("dbruntime") {
                    if checker.is_rule_enabled(crate::registry::Rule::InternalApi) {
                        checker.report_diagnostic(InternalApi, stmt.range());
                    }
                }
                for needle in UC_INCOMPATIBLE_BRUTE_FORCE {
                    if module_str.contains(needle) {
                        if checker.is_rule_enabled(crate::registry::Rule::IncompatibleWithUc) {
                            checker.report_diagnostic(IncompatibleWithUc, stmt.range());
                        }
                    }
                }
            }
            names
        }
        _ => return,
    };

    for alias in names {
        let name_str = alias.name.as_str();
        if name_str.starts_with("databricks_cli") {
            if checker.is_rule_enabled(crate::registry::Rule::LegacyCli) {
                checker.report_diagnostic(LegacyCli, alias.range());
            }
        }
        if name_str.starts_with("dbruntime") {
            if checker.is_rule_enabled(crate::registry::Rule::InternalApi) {
                checker.report_diagnostic(InternalApi, alias.range());
            }
        }
        for needle in UC_INCOMPATIBLE_BRUTE_FORCE {
            if name_str.contains(needle) {
                if checker.is_rule_enabled(crate::registry::Rule::IncompatibleWithUc) {
                    checker.report_diagnostic(IncompatibleWithUc, alias.range());
                }
            }
        }
    }
}

/// DBX010, DBX012
pub(crate) fn expr(checker: &Checker, expr: &Expr) {
    match expr {
        Expr::Attribute(ast::ExprAttribute { attr, .. }) => {
            let attr_name = attr.as_str();
            if matches!(attr_name, "getDbutils" | "apiToken")
                || attr_name.contains(".notebook().getContext()")
                || attr_name.contains(".notebook.entry_point")
            {
                if checker.is_rule_enabled(crate::registry::Rule::InternalApi) {
                    checker.report_diagnostic(InternalApi, expr.range());
                }
            }
        }
        Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) => {
            let val = value.to_str();
            for needle in UC_INCOMPATIBLE_BRUTE_FORCE {
                if val.contains(needle) {
                    if checker.is_rule_enabled(crate::registry::Rule::IncompatibleWithUc) {
                        checker.report_diagnostic(IncompatibleWithUc, expr.range());
                    }
                }
            }
        }
        _ => {}
    }
}
