use mcdp_format2_rs::process_file;
use mcdp_format2_rs::ProcessingResults;
use std::path::Path;

fn run_test_file(path: &Path) -> datatest_stable::Result<()> {
    let mut results = ProcessingResults::new();
    let verbose = false;
    let yaml = false;

    process_file(path, verbose, yaml, &mut results);

    if results.failed_files.is_empty() {
        Ok(())
    } else {
        Err("Some files failed to process".into())
    }
}
datatest_stable::harness! {
    { test = run_test_file, root ="examples", pattern = r"^.*\.yaml\.gz$" },
}
