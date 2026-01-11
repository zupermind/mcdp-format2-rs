use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use glob::Pattern;
use mcdp_format2_rs::Root;
use mcdp_format2_rs::parsing::list_paths;
use mcdp_format2_rs::parsing::read_mcdp_root;
use std::path::PathBuf;

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
fn parse_args() -> Result<Config> {
    let cli = Cli::parse();

    let pattern = Pattern::new(&cli.pattern).context("Invalid pattern")?;

    Ok(Config {
        pattern,
        paths: cli.paths,
        verbose: cli.verbose,
    })
}

fn main() -> Result<()> {
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
