use clap::Parser;
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

#[derive(Parser)]
#[command(author, version, about = "MCDP file parser")]
struct Cli {
    /// Files or directories to process
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// File pattern to match (e.g. "*.yaml")
    #[arg(short, long, default_value = "*.mcdp2.*")]
    pattern: String,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}
/// Build the runtime [`Config`] from parsed CLI arguments.
///
/// Split out from [`parse_args`] so the glob-pattern validation path (which
/// mints [`Mf2rError::InvalidPattern`]) is reachable from a test without
/// touching the real process argv.
fn build_config(cli: Cli) -> ZResult<Config, Mf2rError> {
    let pattern = zerror_from_kv!(
        Pattern::new(&cli.pattern),
        Mf2rError::InvalidPattern,
        "invalid pattern",
        pattern = &cli.pattern,
    )?;

    Ok(Config {
        pattern,
        paths: cli.paths,
        verbose: cli.verbose,
    })
}

fn parse_args() -> ZResult<Config, Mf2rError> {
    build_config(Cli::parse())
}

#[zuper_errors2::zerror_main]
fn main() -> ZResult<(), Mf2rError> {
    let config = parse_args()?;

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
        // component, which `glob::Pattern::new` rejects. clap accepts it as a
        // plain string argument.
        let cli = match Cli::try_parse_from(["mcdp-format2-rs-load", "some-path", "--pattern", "a**b"]) {
            Ok(cli) => cli,
            Err(e) => ztest_bail!("clap should accept the raw args; got {e}"),
        };
        let err = match build_config(cli) {
            Ok(_) => ztest_bail!("expected an invalid glob pattern to fail"),
            Err(err) => err,
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
