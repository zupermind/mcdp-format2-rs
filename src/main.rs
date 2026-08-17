use glob::Pattern;
use mcdp_format2_rs::Mf2rError;
use mcdp_format2_rs::Root;
use mcdp_format2_rs::parsing::list_paths;
use mcdp_format2_rs::parsing::read_mcdp_root;
use std::path::PathBuf;
use zuper_errors2::ZResult;
use zuper_errors2::zerror_err;
use zuper_errors2::zerror_from_kv;

struct Config {
    pub pattern: Pattern,
    pub paths: Vec<PathBuf>,
    pub verbose: bool,
    pub min_files: usize,
}

/// Build the runtime [`Config`] from parsed CLI arguments.
///
/// Split out so the glob-pattern validation path (which mints
/// [`Mf2rError::InvalidPattern`]) is reachable from a unit test.
fn build_config(paths: Vec<PathBuf>, pattern: &str, verbose: bool, min_files: usize) -> ZResult<Config, Mf2rError> {
    let parsed_pattern =
        zerror_from_kv!(Pattern::new(&pattern), Mf2rError::InvalidPattern, "invalid pattern", pattern = &pattern,)?;

    Ok(Config {
        pattern: parsed_pattern,
        paths,
        verbose,
        min_files,
    })
}

/// Fail unless the matched corpus is at least `config.min_files` documents.
///
/// Kept separate from [`main`] so the shortfall path is reachable from a test
/// without staging a directory tree, and so the check cannot be skipped by
/// editing a Makefile recipe: the requirement is enforced where the files are
/// counted, not where the command line is written.
fn check_min_files(matched: &[PathBuf], config: &Config) -> ZResult<(), Mf2rError> {
    if matched.len() >= config.min_files {
        return Ok(());
    }

    let roots = config
        .paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    zerror_err!(
        Mf2rError::TooFewFiles,
        "fewer files matched than the invocation requires",
        matched = matched.len(),
        min_files = config.min_files,
        pattern = config.pattern.as_str(),
        roots = roots,
    )
}

fn load(paths: Vec<PathBuf>, pattern: &str, verbose: bool, min_files: usize) -> ZResult<(), Mf2rError> {
    let config = build_config(paths, pattern, verbose, min_files)?;

    if config.verbose {
        println!("Using pattern: {}", config.pattern.as_str());
    }

    let mut all_paths: Vec<PathBuf> = Vec::new();

    for path in &config.paths {
        let paths = list_paths(path, config.pattern.clone())?;
        all_paths.extend(paths);
    }
    check_min_files(&all_paths, &config)?;

    let n = all_paths.len();
    for (i, p) in all_paths.iter().enumerate() {
        println!("{}/{}: {}", i, n, p.display());
        let root: Root = read_mcdp_root(p)?;

        if config.verbose {
            println!("Parsed:\n{:#?}", root);
        }
    }

    Ok(())
}

async fn load_command(
    _invocation: &dyn zuper_cli::InvocationTrait,
    paths: Vec<PathBuf>,
    pattern: String,
    verbose: bool,
    min_files: std::num::NonZeroUsize,
) -> zuper_cli::CliResult<zuper_cli::RunOutcome> {
    zuper_errors2::zerror_because!(
        load(paths, &pattern, verbose, min_files.get()),
        zuper_cli::CliError::DomainError,
        "could not load the selected MCDP files",
    )?;
    Ok(zuper_cli::RunOutcome::completed_success())
}

include!(concat!(env!("OUT_DIR"), "/zx_component.rs"));
include!(concat!(env!("OUT_DIR"), "/zx_main.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use zuper_errors2::ErrorLocus;
    use zuper_errors2::ErrorStability;
    use zuper_errors2::ZTestResult;
    use zuper_errors2::ztest_bail;
    use zuper_errors2::ztest_ensure;

    /// An invalid glob pattern supplied on the command line produces
    /// `Mf2rError::InvalidPattern` and retains the concrete `glob::PatternError`.
    #[test]
    fn invalid_glob_pattern_is_cli_invalid_pattern() -> ZTestResult<()> {
        // `a**b` embeds a recursive wildcard that does not form its own path
        // component, which `glob::Pattern::new` rejects.
        let Err(err) = build_config(vec![PathBuf::from("some-path")], "a**b", false, 1) else {
            ztest_bail!("expected an invalid glob pattern to fail")
        };
        err.report().assert_primary_code(&Mf2rError::InvalidPattern.code())?;
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&zuper_errors2::error_code_for_external_type::<glob::PatternError>()),
            "expected the concrete glob::PatternError to remain recoverable",
        );
        Ok(())
    }

    fn config_with_min(min_files: usize) -> ZTestResult<Config> {
        match build_config(vec![PathBuf::from("some-path")], "*.mcdp2.*", false, min_files) {
            Ok(config) => Ok(config),
            Err(_) => ztest_bail!("building a config with minimum {min_files} should succeed"),
        }
    }

    /// Matching fewer files than `--min-files` demands is
    /// `Mf2rError::TooFewFiles`, which is what stops a run over a corpus that
    /// is not there from passing having read nothing.
    #[test]
    fn matching_under_the_minimum_is_too_few_files() -> ZTestResult<()> {
        let config = config_with_min(100)?;
        let matched = vec![PathBuf::from("a.mcdp2.yaml.gz"), PathBuf::from("b.mcdp2.yaml.gz")];
        let err = match check_min_files(&matched, &config) {
            Ok(()) => ztest_bail!("expected 2 matched files to fall short of a minimum of 100"),
            Err(err) => err,
        };
        err.report().assert_primary_code(&Mf2rError::TooFewFiles.code())?;
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        Ok(())
    }

    /// The default minimum is 1, so an empty match fails even when the caller
    /// asked for no particular corpus size — the hole this check closes.
    #[test]
    fn the_default_minimum_rejects_an_empty_match() -> ZTestResult<()> {
        let config = config_with_min(1)?;
        ztest_ensure!(config.min_files == 1, "unexpected default minimum: {}", config.min_files);
        let matched: Vec<PathBuf> = Vec::new();
        let err = match check_min_files(&matched, &config) {
            Ok(()) => ztest_bail!("expected an empty match to fail under the default minimum"),
            Err(err) => err,
        };
        err.report().assert_primary_code(&Mf2rError::TooFewFiles.code())?;
        Ok(())
    }

    /// A minimum of 0 is not expressible on the command line: it would request
    /// exactly the vacuous pass the option exists to prevent.
    #[test]
    fn a_minimum_of_zero_is_rejected_by_the_parser() -> ZTestResult<()> {
        ztest_ensure!(std::num::NonZeroUsize::new(0).is_none(), "the generated CLI input type must reject zero");
        Ok(())
    }

    /// The boundary is inclusive: matching exactly the minimum passes, so the
    /// check rejects a shortfall rather than demanding a surplus.
    #[test]
    fn matching_exactly_the_minimum_is_accepted() -> ZTestResult<()> {
        let config = config_with_min(2)?;
        let matched = vec![PathBuf::from("a.mcdp2.yaml.gz"), PathBuf::from("b.mcdp2.yaml.gz")];
        ztest_ensure!(check_min_files(&matched, &config).is_ok(), "2 matched files should satisfy a minimum of 2");
        let surplus = vec![
            PathBuf::from("a.mcdp2.yaml.gz"),
            PathBuf::from("b.mcdp2.yaml.gz"),
            PathBuf::from("c.mcdp2.yaml.gz"),
        ];
        ztest_ensure!(check_min_files(&surplus, &config).is_ok(), "3 matched files should satisfy a minimum of 2");
        Ok(())
    }
}
