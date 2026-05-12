pub mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::test::test_path;
    use crate::{assert_diagnostics, settings};

    #[test_case(Rule::DbutilsFsCp, Path::new("DBX.py"))]
    #[test_case(Rule::DbutilsFsHead, Path::new("DBX.py"))]
    #[test_case(Rule::DbutilsFsLs, Path::new("DBX.py"))]
    #[test_case(Rule::DbutilsFsMount, Path::new("DBX.py"))]
    #[test_case(Rule::DbutilsCredentials, Path::new("DBX.py"))]
    #[test_case(Rule::DbutilsNotebookRun, Path::new("DBX.py"))]
    #[test_case(Rule::HardcodedDatabricksToken, Path::new("DBX.py"))]
    #[test_case(Rule::DatabricksInternalApi, Path::new("DBX.py"))]
    #[test_case(Rule::DatabricksLegacyCli, Path::new("DBX.py"))]
    #[test_case(Rule::UnityCatalogIncompatible, Path::new("DBX.py"))]
    #[test_case(Rule::NotebookPercentRun, Path::new("DBX.py"))]
    #[test_case(Rule::NotebookPercentPip, Path::new("DBX.py"))]
    #[test_case(Rule::TooManyNotebookCells, Path::new("DBX.py"))]
    #[test_case(Rule::SparkOutsideFunction, Path::new("DBX_spark.py"))]
    #[test_case(Rule::UnpassedSparkReference, Path::new("DBX_spark.py"))]
    #[test_case(Rule::SparkDataFrameShow, Path::new("DBX_spark.py"))]
    #[test_case(Rule::ImplicitMockDependency, Path::new("DBX_mocking.py"))]
    #[test_case(Rule::MagicMockUsage, Path::new("DBX_mocking.py"))]
    #[test_case(Rule::UnassignedMock, Path::new("DBX_mocking.py"))]
    #[test_case(Rule::MultilineListComprehension, Path::new("DBX_readability.py"))]
    #[test_case(Rule::MissingDataSecurityMode, Path::new("DBX_airflow.py"))]
    #[test_case(Rule::UnsupportedRuntime, Path::new("DBX_airflow.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("databricks").join(path),
            &settings::LinterSettings::for_rule(rule_code),
        )?;
        assert_diagnostics!(snapshot, diagnostics);
        Ok(())
    }
}
