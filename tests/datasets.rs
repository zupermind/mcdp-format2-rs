use mcdp_format2_rs::parsing::read;
use mcdp_format2_rs::Root;
use std::path::Path;

fn run_test_file(path: &Path) -> datatest_stable::Result<()> {
    let _root: Root = read(path)?;

    Ok(())
}
datatest_stable::harness! {
    { test = run_test_file, root ="examples", pattern = r"^.*\.yaml\.gz$" },
}
