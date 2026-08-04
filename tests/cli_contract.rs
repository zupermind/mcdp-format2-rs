use core::error::Error;
use std::path::PathBuf;
use std::process::Command;

type TestResult = Result<(), Box<dyn Error + Send + Sync>>;

fn _run(arguments: &[&str]) -> Result<std::process::Output, Box<dyn Error + Send + Sync>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_mcdp-format2-rs-load"))
        .args(arguments)
        .output()?)
}

fn _utf8(bytes: Vec<u8>) -> Result<String, Box<dyn Error + Send + Sync>> { Ok(String::from_utf8(bytes)?) }

#[test]
fn generated_cli_preserves_the_loader_contract() -> TestResult {
    let help = _run(&["--help"])?;
    if !help.status.success() {
        return Err(format!("--help failed: {}", _utf8(help.stderr)?).into());
    }
    let help_stdout = _utf8(help.stdout)?;
    for expected in ["MCDP file parser", "Usage:", "--pattern", "--verbose", "--min-files", "<paths>..."] {
        if !help_stdout.contains(expected) {
            return Err(format!("help does not contain {expected:?}:\n{help_stdout}").into());
        }
    }

    let manifest = _run(&["--zx-describe-v3"])?;
    if !manifest.status.success() {
        return Err(format!("--zx-describe-v3 failed: {}", _utf8(manifest.stderr)?).into());
    }
    let manifest: serde_json::Value = serde_json::from_slice(&manifest.stdout)?;
    let root = manifest["commands"]
        .as_array()
        .and_then(|commands| commands.iter().find(|command| command["path"] == serde_json::json!([])))
        .ok_or_else(|| format!("manifest lacks the root command: {manifest}"))?;
    if root["positionals"][0]["id"] != "paths"
        || root["positionals"][0]["arity"] != "many"
        || root["positionals"][0]["required"] != true
    {
        return Err(format!("manifest has the wrong paths contract: {manifest}").into());
    }

    let missing = _run(&[])?;
    if missing.status.code() != Some(2) {
        return Err(format!("missing paths should exit 2, got {}", missing.status).into());
    }

    let zero_minimum = _run(&["some-path", "--min-files", "0"])?;
    if zero_minimum.status.code() != Some(2) {
        return Err(format!("a zero minimum should exit 2, got {}", zero_minimum.status).into());
    }

    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/basic_scalar_ops/models/ceil.dp.mcdp2.yaml.gz");
    let fixture = fixture
        .to_str()
        .ok_or_else(|| format!("fixture path is not UTF-8: {}", fixture.display()))?;
    let loaded = _run(&[fixture])?;
    if !loaded.status.success() {
        return Err(format!("fixture load failed: {}", _utf8(loaded.stderr)?).into());
    }
    let loaded_stdout = _utf8(loaded.stdout)?;
    if !loaded_stdout.contains("0/1:") || !loaded_stdout.contains("ceil.dp.mcdp2.yaml.gz") {
        return Err(format!("fixture load output changed:\n{loaded_stdout}").into());
    }

    let invalid = _run(&[fixture, "--pattern", "a**b"])?;
    if invalid.status.success() {
        return Err("invalid glob pattern unexpectedly succeeded".into());
    }
    let invalid_stderr = _utf8(invalid.stderr)?;
    if !invalid_stderr.contains("InvalidPattern") || !invalid_stderr.contains("a**b") {
        return Err(format!("invalid glob diagnostic changed:\n{invalid_stderr}").into());
    }
    Ok(())
}
