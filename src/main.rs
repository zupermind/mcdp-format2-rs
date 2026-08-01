use glob::Pattern;
use mcdp_format2_rs::Mf2rError;
use mcdp_format2_rs::Root;
use mcdp_format2_rs::parsing::list_paths;
use mcdp_format2_rs::parsing::read_mcdp_root;
use std::path::PathBuf;
use zuper_errors2::ZResult;
use zuper_errors2::zerror_from_kv;

struct Config {
    pub pattern: Pattern,
    pub paths: Vec<PathBuf>,
    pub verbose: bool,
}

/// Build the runtime [`Config`] from parsed CLI arguments.
///
/// Split out so the glob-pattern validation path (which mints
/// [`Mf2rError::InvalidPattern`]) is reachable from a unit test.
fn build_config(paths: Vec<PathBuf>, pattern: &str, verbose: bool) -> ZResult<Config, Mf2rError> {
    let parsed_pattern = zerror_from_kv!(
        Pattern::new(&pattern),
        Mf2rError::InvalidPattern,
        "invalid pattern",
        pattern = &pattern,
    )?;

    Ok(Config {
        pattern: parsed_pattern,
        paths,
        verbose,
    })
}

fn load(paths: Vec<PathBuf>, pattern: &str, verbose: bool) -> ZResult<(), Mf2rError> {
    let config = build_config(paths, pattern, verbose)?;

    if config.verbose {
        println!("Using pattern: {}", config.pattern.as_str());
    }

    let mut all_paths: Vec<PathBuf> = Vec::new();

    for path in &config.paths {
        let paths = list_paths(path, config.pattern.clone())?;
        all_paths.extend(paths);
    }
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
    _invocation: &dyn zuper_cli::InvocationTrait, paths: Vec<PathBuf>, pattern: String, verbose: bool,
) -> zuper_cli::CliResult<zuper_cli::RunOutcome> {
    zuper_errors2::zerror_because!(
        load(paths, &pattern, verbose),
        zuper_cli::CliError::DomainError,
        "could not load the selected MCDP files",
    )?;
    Ok(zuper_cli::RunOutcome::success(""))
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
        let Err(err) = build_config(vec![PathBuf::from("some-path")], "a**b", false) else {
            ztest_bail!("expected an invalid glob pattern to fail")
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::InvalidPattern.code(),
            "unexpected primary code: {:?}",
            err.primary_code(),
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&zuper_errors2::error_code_for_external_type::<glob::PatternError>()),
            "expected the concrete glob::PatternError to remain recoverable",
        );
        Ok(())
    }
}
