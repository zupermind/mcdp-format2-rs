use anyhow::Context;
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

pub fn detect_format(path: &Path) -> DataFormat {
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

            let yaml_value: serde_yaml::Value =
                serde_yaml::from_str(decoded).context("Failed to parse YAML")?;

            // Convert serde_yaml::Value to ciborium::value::Value
            let cbor_value = yaml_to_cbor_value(yaml_value)?;
            Ok(cbor_value)
        }
        DataFormat::CBOR | DataFormat::CBOR_GZ => {
            let cbor_value: ciborium::value::Value =
                ciborium::de::from_reader(&contents[..]).context("Failed to parse CBOR")?;
            Ok(cbor_value)
        }
        DataFormat::JSON | DataFormat::JSON_GZ => {
            let decoded = match std::str::from_utf8(&contents) {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!("UTF decoding error: {e:?}"));
                }
            };

            let json_value: serde_json::Value =
                serde_json::from_str(decoded).context("Failed to parse JSON")?;

            // Convert serde_json::Value to ciborium::value::Value
            let cbor_value = json_to_cbor_value(json_value)?;
            Ok(cbor_value)
        }

        DataFormat::Unknown(_) => Err(anyhow::anyhow!("Unknown format: {:?}", format)),
    }
}

fn decompress_gz(compressed: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

pub fn read_file(path: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(path).context("Failed to open file")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .context("Failed to read file")?;
    Ok(contents)
}

static CHECK_MARK: Emoji<'_, '_> = Emoji("✓", "√");
static CROSS_MARK: Emoji<'_, '_> = Emoji("✗", "×");

pub struct Config {
    pub pattern: Pattern,
    pub paths: Vec<PathBuf>,
    pub verbose: bool,
    pub yaml: bool,
}

pub fn list_paths(path: &Path, pattern: Pattern) -> anyhow::Result<Vec<PathBuf>> {
    let mut res = Vec::new();
    list_paths_recursive(path, pattern, &mut res)?;

    Ok(res)
}

pub fn list_paths_recursive(
    path: &Path,
    pattern: Pattern,
    results: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    if path.is_dir() {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
                    if pattern.matches(file_name) {
                        list_paths_recursive(path, pattern.clone(), results)?;
                    }
                }
            }
        }
    } else {
        let filename = path.file_name().and_then(OsStr::to_str).unwrap_or("");

        if pattern.matches(filename) {
            results.push(path.to_path_buf());
        }
    }

    Ok(())
}

use ciborium;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::de::DeserializeOwned;

fn from_cbor_value<T: DeserializeOwned>(val: &ciborium::value::Value) -> Result<T> {
    let mut buf = Vec::new();
    into_writer(val, &mut buf)?; // Serialize the Value to CBOR bytes
    let t = from_reader(buf.as_slice())?; // Deserialize CBOR bytes into T
    Ok(t)
}

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let format = detect_format(path);
    let contents = read_file(path)?;
    let data: ciborium::Value = parse_data(&contents, format)?;

    let root: T = from_cbor_value(&data)?;

    Ok(root)
}

/// Convert serde_yaml::Value to ciborium::value::Value
fn yaml_to_cbor_value(yaml_value: serde_yaml::Value) -> Result<ciborium::value::Value> {
    use ciborium::value::Value;

    let result = match yaml_value {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i.into())
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                return Err(anyhow::anyhow!("Invalid number format"));
            }
        }
        serde_yaml::Value::String(s) => Value::Text(s),
        serde_yaml::Value::Sequence(seq) => {
            let mut vec = Vec::new();
            for item in seq {
                vec.push(yaml_to_cbor_value(item)?);
            }
            Value::Array(vec)
        }
        serde_yaml::Value::Mapping(map) => {
            let mut cbor_map = Vec::new();
            for (k, v) in map {
                let key = yaml_to_cbor_value(k)?;
                let value = yaml_to_cbor_value(v)?;
                cbor_map.push((key, value));
            }
            Value::Map(cbor_map)
        }
        serde_yaml::Value::Tagged(tagged) => {
            // For tagged values, we'll just use the inner value for now
            yaml_to_cbor_value(tagged.value)?
        }
    };

    Ok(result)
}

/// Convert serde_json::Value to ciborium::value::Value
fn json_to_cbor_value(json_value: serde_json::Value) -> Result<ciborium::value::Value> {
    use ciborium::value::Value;

    let result = match json_value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i.into())
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                return Err(anyhow::anyhow!("Invalid number format"));
            }
        }
        serde_json::Value::String(s) => Value::Text(s),
        serde_json::Value::Array(arr) => {
            let mut vec = Vec::new();
            for item in arr {
                vec.push(json_to_cbor_value(item)?);
            }
            Value::Array(vec)
        }
        serde_json::Value::Object(obj) => {
            let mut cbor_map = Vec::new();
            for (k, v) in obj {
                let key = Value::Text(k); // JSON object keys are always strings
                let value = json_to_cbor_value(v)?;
                cbor_map.push((key, value));
            }
            Value::Map(cbor_map)
        }
    };

    Ok(result)
}

/// Parse data from bytes using the specified format
pub fn parse_data(contents: &[u8], format: DataFormat) -> Result<ciborium::value::Value> {
    interpret_cvalue(contents, &format)
}
