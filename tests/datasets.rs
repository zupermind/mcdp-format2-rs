use mcdp_format2_rs::Mf2rError;
use mcdp_format2_rs::Root;
use mcdp_format2_rs::parsing::read;
use std::path::Path;
use zuper_errors2::ZError;

/// Convert a structured load failure for datatest's legacy error boundary.
fn datatest_error(error: ZError<Mf2rError>) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(zuper_errors2::human_only_report(&error)))
}

fn run_test_file(path: &Path) -> datatest_stable::Result<()> {
    let _root: Root = read(path).map_err(datatest_error)?;

    Ok(())
}
datatest_stable::harness! {
    { test = run_test_file, root ="examples", pattern = r"^.*\.yaml\.gz$" },
}
