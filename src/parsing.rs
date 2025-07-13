use anyhow::Context;
use console::style;
use console::Emoji;
use glob::Pattern;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub use anyhow::Result;
use flate2::read::GzDecoder;
use std::io::Read;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataFormat {
    JSON,
    JSON_GZ,
    YAML,
    YAML_GZ,
    CBOR,
    CBOR_GZ,
    Unknown(String),
}

fn detect_format(path: &Path) -> DataFormat {
    let just_basename = path.file_name().and_then(OsStr::to_str).unwrap_or("");

    if just_basename.ends_with(".json.gz") {
        return DataFormat::JSON_GZ;
    }
    if just_basename.ends_with(".yaml.gz") || just_basename.ends_with(".yml.gz") {
        return DataFormat::YAML_GZ;
    }
    if just_basename.ends_with(".cbor.gz") {
        return DataFormat::CBOR_GZ;
    }
    if just_basename.ends_with(".json") {
        return DataFormat::JSON;
    }
    if just_basename.ends_with(".yaml") || just_basename.ends_with(".yml") {
        return DataFormat::YAML;
    }
    if just_basename.ends_with(".cbor") {
        return DataFormat::CBOR;
    }

    DataFormat::Unknown(just_basename.to_string())
}

fn interpret_cvalue(contents: &[u8], format: &DataFormat) -> Result<ciborium::value::Value> {
    let mut contents: Vec<u8> = contents.to_vec();
    match format {
        DataFormat::YAML_GZ | DataFormat::CBOR_GZ | DataFormat::JSON_GZ => {
            contents = decompress_gz(&contents)?;
        }
        _ => {}
    };

    match format {
        DataFormat::YAML | DataFormat::YAML_GZ => {
            let decoded = match std::str::from_utf8(&contents) {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!("UTF decoding error: {e:?}"));
                }
            };

            let r = TODO(decoded);
            match r {
                Ok(x) => {
                    if x.len() != 1 {
                        return anyhow::anyhow!("Expected one document, got {}", x.len());
                    }
                    Ok(x.first().unwrap().clone())
                }
                Err(e) => anyhow::anyhow!("YAML parsing error:\n{e}"),
            }
        }
        DataFormat::CBOR | DataFormat::CBOR_GZ => {
            let t0 = std::time::Instant::now();
            let r = TODO(&contents)?;
            let _time_mine = t0.elapsed();

            Ok(r)
        }
        DataFormat::JSON | DataFormat::JSON_GZ => {
            anyhow::anyhow!("JSON parsing not implemented")
        }

        DataFormat::Unknown(_) => {
            anyhow::anyhow!("Unknown format: {:?}", format)
        }
    }
}

fn decompress_gz(compressed: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

pub fn read_file(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(path).context("Failed to open file")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .context("Failed to read file")?;
    Ok(contents)
}

static CHECK_MARK: Emoji<'_, '_> = Emoji("✓", "√");
static CROSS_MARK: Emoji<'_, '_> = Emoji("✗", "×");

pub struct ProcessingResults {
    pub failed_files: Vec<(PathBuf, anyhow::Error)>,
    pub success_count: usize,
}

impl ProcessingResults {
    pub fn new() -> Self {
        Self {
            failed_files: Vec::new(),
            success_count: 0,
        }
    }

    pub fn add_success(&mut self) {
        self.success_count += 1;
    }

    pub fn add_failure(&mut self, path: PathBuf, error: anyhow::Error) {
        self.failed_files.push((path, error));
    }

    pub fn print_summary(&self, delete_failed: bool) {
        let total = self.success_count + self.failed_files.len();

        if !self.failed_files.is_empty() {
            println!(
                "\n{} Failed files{}:",
                style(self.failed_files.len()).red(),
                if delete_failed { " (deleted)" } else { "" }
            );
            for (path, error) in &self.failed_files {
                println!("{} {}: {}", style(CROSS_MARK).red(), path.display(), error);
            }
        }
        println!("\nProcessing Summary:");
        println!(
            "Successfully processed: {} of {} files",
            style(self.success_count).green(),
            total
        );
    }
}

pub struct Config {
    pub pattern: Pattern,
    pub paths: Vec<PathBuf>,
    pub verbose: bool,
    pub yaml: bool,
    pub delete_failed: bool,
}

pub fn process_path(path: &Path, config: &Config, results: &mut ProcessingResults) {
    if path.is_dir() {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
                    if config.pattern.matches(file_name) {
                        process_file(
                            path,
                            config.verbose,
                            config.yaml,
                            config.delete_failed,
                            results,
                        );
                    }
                }
            }
        }
    } else {
        process_file(
            path,
            config.verbose,
            config.yaml,
            config.delete_failed,
            results,
        );
    }
}

pub fn process_file(
    path: &Path,
    verbose: bool,
    yaml: bool,
    delete_failed: bool,
    results: &mut ProcessingResults,
) {
    print!("Processing file: {} ... ", path.display());

    match (|| -> anyhow::Result<()> {
        let format = detect_format(path);
        let contents = read_file(path)?;
        let data = parse_data(&contents, format)?;

        if verbose || yaml {
            println!();
            if yaml {
                let yaml_string =
                    serde_yaml::to_string(&data).context("Failed to convert to YAML")?;
                println!("{}", yaml_string);

                let json_string =
                    serde_json::to_string(&data).context("Failed to convert to JSON")?;
                println!("{}", json_string);
            }
            if verbose {
                println!("{:#?}", data);
            }
        }
        Ok(())
    })() {
        Ok(_) => {
            if !verbose && !yaml {
                println!("{} {}", style(CHECK_MARK).green(), style("Success").green());
            }
            results.add_success();
        }
        Err(e) => {
            if !verbose && !yaml {
                println!("{} {}", style(CROSS_MARK).red(), style("Failed").red());
            }
            if delete_failed {
                if let Err(delete_err) = std::fs::remove_file(path) {
                    println!(
                        "{} Failed to delete file: {}",
                        style(CROSS_MARK).red(),
                        delete_err
                    );
                }
            }
            results.add_failure(path.to_path_buf(), e);
        }
    }
}
