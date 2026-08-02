#[zuper_errors2::zerror_main]
fn main() -> zuper_errors2::ZResult<(), zuper_cli::BuildSupportError> {
    zuper_cli::compile_setup_yaml("zx.setup.yaml")
}
