use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use glob::Pattern;
use mcdp_format2_rs::parsing::process_path;
use mcdp_format2_rs::parsing::Config;
use mcdp_format2_rs::parsing::ProcessingResults;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "MCDP file parser")]
struct Cli {
    /// Files or directories to process
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// File pattern to match (e.g. "*.yaml")
    #[arg(short, long, default_value = "*.yaml")]
    pattern: String,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Print as YAML
    #[arg(short = 'y', long)]
    yaml: bool,

    /// Delete files that fail to parse
    #[arg(long)]
    delete_failed: bool,
}
fn parse_args() -> Result<Config> {
    let cli = Cli::parse();

    let pattern = Pattern::new(&cli.pattern).context("Invalid pattern")?;

    Ok(Config {
        pattern,
        paths: cli.paths,
        verbose: cli.verbose,
        yaml: cli.yaml,
        delete_failed: cli.delete_failed,
    })
}

fn main() -> Result<()> {
    let config = parse_args()?;
    let mut results = ProcessingResults::new();

    if config.verbose {
        println!("Using pattern: {}", config.pattern.as_str());
    }

    for path in &config.paths {
        process_path(path, &config, &mut results);
    }

    results.print_summary(config.delete_failed);

    // Exit with error if any files failed
    if !results.failed_files.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dummy() {}
}
