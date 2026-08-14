use crate::Root;
use crate::errors::Mf2rError;
use ciborium;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use flate2::read::GzDecoder;
use glob::Pattern;
use serde::de::DeserializeOwned;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
use zuper_errors2::ZResult;
use zuper_errors2::zerror;
use zuper_errors2::zerror_from;
use zuper_errors2::zerror_from_kv;

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

fn interpret_cvalue(contents: &[u8], format: &DataFormat) -> ZResult<ciborium::value::Value, Mf2rError> {
    let mut contents: Vec<u8> = contents.to_vec();
    match format {
        DataFormat::YAML_GZ | DataFormat::CBOR_GZ | DataFormat::JSON_GZ => {
            contents =
                zerror_from!(decompress_gz(&contents), Mf2rError::Decompress, "could not decompress gzip payload",)?;
        }
        _ => {}
    };

    match format {
        DataFormat::YAML | DataFormat::YAML_GZ => {
            let decoded = zerror_from!(core::str::from_utf8(&contents), Mf2rError::Utf8, "input is not valid UTF-8",)?;

            let yaml_value: serde_yaml::Value = zerror_from!(
                serde_yaml::from_str(decoded),
                Mf2rError::CannotParseYamlDocument,
                "could not parse YAML",
            )?;

            // Convert serde_yaml::Value to ciborium::value::Value
            let cbor_value = yaml_to_cbor_value(yaml_value)?;
            Ok(cbor_value)
        }
        DataFormat::CBOR | DataFormat::CBOR_GZ => {
            let cbor_value: ciborium::value::Value =
                zerror_from!(from_reader(&contents[..]), Mf2rError::Cbor, "could not parse CBOR",)?;
            Ok(cbor_value)
        }
        DataFormat::JSON | DataFormat::JSON_GZ => {
            let decoded = zerror_from!(core::str::from_utf8(&contents), Mf2rError::Utf8, "input is not valid UTF-8",)?;

            let json_value: serde_json::Value = zerror_from!(
                serde_json::from_str(decoded),
                Mf2rError::CannotParseJsonDocument,
                "could not parse JSON",
            )?;

            // Convert serde_json::Value to ciborium::value::Value
            let cbor_value = json_to_cbor_value(json_value)?;
            Ok(cbor_value)
        }

        DataFormat::Unknown(suffix) => {
            Err(zerror!(Mf2rError::UnknownFormat { format: suffix.clone() }, "unknown data format",))
        }
    }
}

fn decompress_gz(compressed: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

fn read_file(path: &Path) -> ZResult<Vec<u8>, Mf2rError> {
    let mut file =
        zerror_from_kv!(File::open(path), Mf2rError::CannotOpenFile, "could not open file", path = path.display(),)?;
    let mut contents = Vec::new();
    zerror_from_kv!(
        file.read_to_end(&mut contents),
        Mf2rError::CannotReadFileContents,
        "could not read file",
        path = path.display(),
    )?;
    Ok(contents)
}

/// List all paths in a directory that match a given glob pattern.
pub fn list_paths(path: &Path, pattern: Pattern) -> ZResult<Vec<PathBuf>, Mf2rError> {
    let mut res = Vec::new();
    list_paths_recursive(path, pattern, &mut res)?;

    Ok(res)
}

fn list_paths_recursive(path: &Path, pattern: Pattern, results: &mut Vec<PathBuf>) -> ZResult<(), Mf2rError> {
    if path.is_dir() {
        for entry in WalkDir::new(path).follow_links(true) {
            // A traversal failure must fail the call rather than silently
            // truncate the list. An ancestor loop is a malformed input tree and
            // is classified as such; every other traversal failure (permission
            // denial, vanished entry, transient I/O) stays unclassified.
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) if error.loop_ancestor().is_some() => zerror_from_kv!(
                    Err(error),
                    Mf2rError::DirectorySymlinkLoop,
                    "directory tree contains a symlink loop",
                    path = path.display(),
                )?,
                Err(error) => zerror_from_kv!(
                    Err(error),
                    Mf2rError::CannotReadDirectoryEntry,
                    "could not read a directory entry while traversing",
                    path = path.display(),
                )?,
            };
            let path = entry.path();
            if path.is_file()
                && let Some(file_name) = path.file_name().and_then(OsStr::to_str)
                && pattern.matches(file_name)
            {
                list_paths_recursive(path, pattern.clone(), results)?;
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

fn from_cbor_value<T: DeserializeOwned>(val: &ciborium::value::Value) -> ZResult<T, Mf2rError> {
    let mut buf = Vec::new();
    // Serialize the Value to CBOR bytes
    zerror_from!(into_writer(val, &mut buf), Mf2rError::Encode, "could not encode value to CBOR",)?;
    // Deserialize CBOR bytes into T
    let t =
        zerror_from!(from_reader(buf.as_slice()), Mf2rError::DecodeTyped, "could not decode CBOR into target type",)?;
    Ok(t)
}

/// Read a file and deserialize its contents into a type T.
pub fn read<T: DeserializeOwned>(path: &Path) -> ZResult<T, Mf2rError> {
    let format = detect_format(path);
    let contents = read_file(path)?;
    let data: ciborium::Value = parse_data(&contents, format)?;

    let root: T = from_cbor_value(&data)?;

    Ok(root)
}

/// Read a file and deserialize its contents into one of the MCDP Format 2 Root types.
pub fn read_mcdp_root(path: &Path) -> ZResult<Root, Mf2rError> {
    let root: Root = read(path)?;
    Ok(root)
}

/// Convert serde_yaml::Value to ciborium::value::Value
fn yaml_to_cbor_value(yaml_value: serde_yaml::Value) -> ZResult<ciborium::value::Value, Mf2rError> {
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
                return Err(zerror!(Mf2rError::UnsupportedNumber, "number has no CBOR representation",));
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
fn json_to_cbor_value(json_value: serde_json::Value) -> ZResult<ciborium::value::Value, Mf2rError> {
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
                return Err(zerror!(Mf2rError::UnsupportedNumber, "number has no CBOR representation",));
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
pub fn parse_data(contents: &[u8], format: DataFormat) -> ZResult<ciborium::value::Value, Mf2rError> {
    interpret_cvalue(contents, &format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Root;
    use zuper_errors2::ErrorLocus;
    use zuper_errors2::ErrorStability;
    use zuper_errors2::ZTestResult;
    use zuper_errors2::error_code_for_external_type;
    use zuper_errors2::ztest_bail;
    use zuper_errors2::ztest_ensure;

    /// An unrecognized format produces a caller-persistent `Mf2rError::UnknownFormat`.
    #[test]
    fn unknown_format_is_caller_persistent() -> ZTestResult<()> {
        let err = match parse_data(b"whatever", DataFormat::Unknown("x.bin".to_owned())) {
            Ok(_) => ztest_bail!("expected unknown-format to fail"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code()
                == Mf2rError::UnknownFormat {
                    format: "x.bin".to_owned(),
                }
                .code(),
            "expected primary code Mf2rError::UnknownFormat",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        Ok(())
    }

    /// Non-UTF-8 bytes for a text format produce `Mf2rError::Utf8` and retain
    /// the concrete `std::str::Utf8Error`.
    #[test]
    fn invalid_utf8_yaml_retains_utf8_error() -> ZTestResult<()> {
        let err = match parse_data(&[0xff, 0xfe, 0xfd], DataFormat::YAML) {
            Ok(_) => ztest_bail!("expected invalid UTF-8 to fail"),
            Err(err) => err,
        };
        ztest_ensure!(err.primary_code() == Mf2rError::Utf8.code(), "expected primary code Mf2rError::Utf8",);
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<std::str::Utf8Error>()),
            "expected the concrete std::str::Utf8Error to remain recoverable",
        );
        Ok(())
    }

    /// A malformed YAML document produces `Mf2rError::CannotParseYamlDocument`
    /// and retains the concrete `serde_yaml::Error`.
    #[test]
    fn bad_yaml_retains_serde_yaml_error() -> ZTestResult<()> {
        // Unterminated flow sequence: valid UTF-8, invalid YAML.
        let err = match parse_data(b"key: [1, 2", DataFormat::YAML) {
            Ok(_) => ztest_bail!("expected malformed YAML to fail"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::CannotParseYamlDocument.code(),
            "expected primary code Mf2rError::CannotParseYamlDocument",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<serde_yaml::Error>()),
            "expected the concrete serde_yaml::Error to remain recoverable",
        );
        Ok(())
    }

    /// A malformed JSON document produces `Mf2rError::CannotParseJsonDocument`
    /// and retains the concrete `serde_json::Error`.
    #[test]
    fn bad_json_retains_serde_json_error() -> ZTestResult<()> {
        let err = match parse_data(b"{ not valid json", DataFormat::JSON) {
            Ok(_) => ztest_bail!("expected malformed JSON to fail"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::CannotParseJsonDocument.code(),
            "expected primary code Mf2rError::CannotParseJsonDocument",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<serde_json::Error>()),
            "expected the concrete serde_json::Error to remain recoverable",
        );
        Ok(())
    }

    /// Malformed CBOR bytes produce `Mf2rError::Cbor` and retain the concrete
    /// `ciborium::de::Error<std::io::Error>` decode error.
    ///
    /// `from_reader` is driven over a `&[u8]` reader, which — with ciborium's
    /// `std` feature — resolves `R::Error` to `std::io::Error`, so the retained
    /// external type is precisely `ciborium::de::Error<std::io::Error>`.
    #[test]
    fn bad_cbor_retains_ciborium_error() -> ZTestResult<()> {
        // 0x9f begins an indefinite-length array with no items and no break.
        let err = match parse_data(&[0x9f], DataFormat::CBOR) {
            Ok(_) => ztest_bail!("expected malformed CBOR to fail"),
            Err(err) => err,
        };
        ztest_ensure!(err.primary_code() == Mf2rError::Cbor.code(), "expected primary code Mf2rError::Cbor",);
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<ciborium::de::Error<std::io::Error>>(),),
            "expected the concrete ciborium::de::Error<std::io::Error> to remain recoverable",
        );
        Ok(())
    }

    /// Corrupt gzip bytes for a `*_GZ` format produce `Mf2rError::Decompress`
    /// and retain the concrete `std::io::Error` raised by the inflate reader.
    #[test]
    fn corrupt_gzip_is_decompress_failed_and_retains_io_error() -> ZTestResult<()> {
        // Valid gzip magic (0x1f 0x8b) + deflate method (0x08) + a well-formed
        // 10-byte header, then a garbage deflate body that fails to inflate.
        let corrupt: &[u8] = &[
            0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xde, 0xad, 0xbe, 0xef,
        ];
        let err = match parse_data(corrupt, DataFormat::JSON_GZ) {
            Ok(_) => ztest_bail!("expected corrupt gzip to fail"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::Decompress.code(),
            "expected primary code Mf2rError::Decompress",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<std::io::Error>()),
            "expected the concrete std::io::Error to remain recoverable",
        );
        Ok(())
    }

    /// A value whose shape does not fit the requested `T` produces
    /// `Mf2rError::DecodeTyped` and retains the concrete ciborium decode error
    /// captured while deserializing the re-encoded CBOR bytes.
    #[test]
    fn typed_decode_mismatch_is_decode_typed_and_retains_external() -> ZTestResult<()> {
        // A text value cannot deserialize into u32; `from_cbor_value` mints
        // DecodeTyped and retains the ciborium decode error.
        let value = ciborium::value::Value::Text("not a number".to_owned());
        let err = match from_cbor_value::<u32>(&value) {
            Ok(_) => ztest_bail!("expected typed decode of a text value into u32 to fail"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::DecodeTyped.code(),
            "expected primary code Mf2rError::DecodeTyped",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<ciborium::de::Error<std::io::Error>>(),),
            "expected the concrete ciborium::de::Error<std::io::Error> to remain recoverable",
        );
        Ok(())
    }

    /// A symlink-loop traversal failure produces the caller-persistent
    /// `Mf2rError::DirectorySymlinkLoop` and retains the concrete
    /// `walkdir::Error`.
    ///
    /// The fixture is a self-referential symlink: under `follow_links(true)`,
    /// `WalkDir` detects the ancestor loop and yields a `walkdir::Error`, which
    /// must fail the whole `list_paths` call rather than truncate the result.
    ///
    /// Platform caveat: the fixture creates a Unix symlink, so the test is
    /// gated to `#[cfg(unix)]`. The traversal-error propagation it exercises is
    /// platform-independent; only the deterministic loop fixture is Unix-only.
    #[cfg(unix)]
    #[test]
    fn walk_error_fails_the_call_and_retains_walkdir_error() -> ZTestResult<()> {
        use std::os::unix::fs::symlink;

        let base = std::env::temp_dir().join(format!("mf2r-walk-test-{}", std::process::id()));
        // Start from a clean slate in case a previous run left the fixture.
        let _ = std::fs::remove_dir_all(&base);
        if let Err(e) = std::fs::create_dir_all(&base) {
            ztest_bail!("could not create fixture dir: {e}");
        }
        // A symlink pointing back at its own parent directory is an ancestor
        // loop for follow_links(true).
        if let Err(e) = symlink(&base, base.join("loop")) {
            let _ = std::fs::remove_dir_all(&base);
            ztest_bail!("could not create symlink loop: {e}");
        }

        let pattern = match Pattern::new("*") {
            Ok(pattern) => pattern,
            Err(e) => {
                let _ = std::fs::remove_dir_all(&base);
                ztest_bail!("could not build match-all pattern: {e}");
            }
        };

        let result = list_paths(&base, pattern);
        // Clean up the fixture before asserting so an early return still tidies.
        let _ = std::fs::remove_dir_all(&base);

        let err = match result {
            Ok(_) => ztest_bail!("expected a symlink-loop traversal to fail"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::DirectorySymlinkLoop.code(),
            "expected primary code Mf2rError::DirectorySymlinkLoop",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Caller);
        ztest_ensure!(err.primary_stability() == ErrorStability::Persistent);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<walkdir::Error>()),
            "expected the concrete walkdir::Error to remain recoverable",
        );
        Ok(())
    }

    /// Reading a nonexistent path fails at `File::open`, so it produces
    /// `Mf2rError::CannotOpenFile` with both axes honestly `Unknown`, and
    /// retains the concrete `std::io::Error`.
    #[test]
    fn read_missing_path_is_read_failed_unknown_and_retains_io_error() -> ZTestResult<()> {
        let err = match read::<Root>(Path::new("/nonexistent/mcdp-format2-rs/definitely-does-not-exist.yaml.gz")) {
            Ok(_) => ztest_bail!("expected read to fail for a nonexistent path"),
            Err(err) => err,
        };
        ztest_ensure!(
            err.primary_code() == Mf2rError::CannotOpenFile.code(),
            "expected primary code Mf2rError::CannotOpenFile",
        );
        ztest_ensure!(err.primary_locus() == ErrorLocus::Unknown);
        ztest_ensure!(err.primary_stability() == ErrorStability::Unknown);
        ztest_ensure!(err.contains_source_error(), "expected the std::io::Error to be retained as a cause",);
        ztest_ensure!(
            err.contains_code(&error_code_for_external_type::<std::io::Error>()),
            "expected the concrete std::io::Error to remain recoverable",
        );
        Ok(())
    }
}
