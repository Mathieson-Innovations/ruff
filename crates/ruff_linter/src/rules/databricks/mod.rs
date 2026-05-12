pub mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::test::test_path;
    use crate::{assert_diagnostics, settings};

    #[test_case(Rule::DbutilsFsCp, Path::new("DBX003.py"))]
    #[test_case(Rule::DbutilsFsHead, Path::new("DBX004.py"))]
    #[test_case(Rule::DbutilsFsLs, Path::new("DBX005.py"))]
    #[test_case(Rule::DbutilsFsMount, Path::new("DBX006.py"))]
    #[test_case(Rule::DbutilsCredentials, Path::new("DBX007.py"))]
    #[test_case(Rule::DbutilsNotebookRun, Path::new("DBX008.py"))]
    #[test_case(Rule::HardcodedDatabricksToken, Path::new("DBX009.py"))]
    #[test_case(Rule::DatabricksInternalApi, Path::new("DBX010.py"))]
    #[test_case(Rule::DatabricksLegacyCli, Path::new("DBX011.py"))]
    #[test_case(Rule::UnityCatalogIncompatible, Path::new("DBX012.py"))]
    #[test_case(Rule::TooManyNotebookCells, Path::new("DBX013.py"))]
    #[test_case(Rule::NotebookPercentRun, Path::new("DBX014.py"))]
    #[test_case(Rule::SparkOutsideFunction, Path::new("DBX015.py"))]
    #[test_case(Rule::UnpassedSparkReference, Path::new("DBX016.py"))]
    #[test_case(Rule::SparkDataFrameShow, Path::new("DBX017.py"))]
    #[test_case(Rule::ImplicitMockDependency, Path::new("DBX018.py"))]
    #[test_case(Rule::MagicMockUsage, Path::new("DBX019.py"))]
    #[test_case(Rule::UnassignedMock, Path::new("DBX021.py"))]
    #[test_case(Rule::UnusedMock, Path::new("DBX022.py"))]
    #[test_case(Rule::MultilineListComprehension, Path::new("DBX023.py"))]
    #[test_case(Rule::NotebookPercentPip, Path::new("DBX024.py"))]
    #[test_case(Rule::MissingDataSecurityMode, Path::new("DBX001.py"))]
    #[test_case(Rule::UnsupportedRuntime, Path::new("DBX002.py"))]
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
