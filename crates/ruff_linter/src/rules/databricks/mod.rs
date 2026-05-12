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
    #[test_case(Rule::PatTokenLeaked, Path::new("DBX.py"))]
    #[test_case(Rule::InternalApi, Path::new("DBX.py"))]
    #[test_case(Rule::LegacyCli, Path::new("DBX.py"))]
    #[test_case(Rule::IncompatibleWithUc, Path::new("DBX.py"))]
    #[test_case(Rule::NotebooksPercentRun, Path::new("DBX.py"))]
    #[test_case(Rule::NotebooksPercentPip, Path::new("DBX.py"))]
    #[test_case(Rule::NotebooksTooManyCells, Path::new("DBX.py"))]
    #[test_case(Rule::SparkOutsideFunction, Path::new("DBX_spark.py"))]
    #[test_case(Rule::NoSparkArgumentInFunction, Path::new("DBX_spark.py"))]
    #[test_case(Rule::UseDisplayInsteadOfShow, Path::new("DBX_spark.py"))]
    #[test_case(Rule::ExplicitDependencyRequired, Path::new("DBX_mocking.py"))]
    #[test_case(Rule::ObscureMock, Path::new("DBX_mocking.py"))]
    #[test_case(Rule::MockNoAssign, Path::new("DBX_mocking.py"))]
    #[test_case(Rule::RewriteAsForLoop, Path::new("DBX_readability.py"))]
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
