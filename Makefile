.PHONY: check cargo-check install

check: cargo-check

cargo-check:
	cargo check --workspace --all-targets --keep-going

install:
	cargo install --path . --locked --force

all:

# The corpus this binary is pointed at lives in the `mcdp-formats-samples` repo
# at the monorepo root (mr entry `mcdp-formats-samples`), three levels up from
# here: its documents are gzipped YAML kept in Git LFS, hence the pattern. The
# targets below used to pass `../../samples`, which resolves to no directory in
# this tree — neither at this repo's current location nor at the pre-`80-mcdp/`
# one. `list_paths` walking a non-existent root is not an error, so the run
# matched nothing, the processing loop never executed, and `main` returned
# `Ok(())`: the monorepo test sweep recorded this repo as a PASS having read
# zero documents. Hence `--min-files`, which is enforced in the binary (see
# `check_min_files`) so that a recipe edit cannot restore the vacuous pass.
# Override to widen the corpus, e.g.
#   make test SAMPLES=../../../mcdp-formats-samples/data MIN_FILES=100000
#
# `SAMPLES=examples` runs the 889-document snapshot vendored in this repo via
# Git LFS instead. That is the only setting that works from a checkout of this
# repo alone — a CI checkout has no sibling `mcdp-formats-samples` — and it is
# the corpus `tests/datasets.rs` already drives through `datatest_stable`. The
# default points at the samples repo because that snapshot is the exporter's
# current output while `examples/` is an older generation of the same suite
# under an earlier naming scheme, so the two are complementary rather than
# duplicates.
SAMPLES ?= ../../../mcdp-formats-samples/data/repos/unittests
SAMPLES_PATTERN ?= *.mcdp2.yaml.gz
MIN_FILES ?= 100

test:
	cargo run --  --pattern '$(SAMPLES_PATTERN)' --min-files $(MIN_FILES) $(SAMPLES)

test-release:
	cargo run -r --  --pattern '$(SAMPLES_PATTERN)' --min-files $(MIN_FILES) $(SAMPLES)

test-verbose:
	cargo run --  --verbose --pattern '$(SAMPLES_PATTERN)' --min-files $(MIN_FILES) $(SAMPLES)

pre-circle-tests:
	  git lfs install
	  git lfs fetch
	  git lfs checkout

the_schema=../../mcdp-formats/out/schema-no-concrete.yaml

generate:
	$(MAKE) -C ../../mcdp-formats preprocess
	cargo run -p zuper-rs-schemas --bin zuper-rs-schemas -- rust  \
		--only-concrete \
		--schema $(the_schema) \
		--target src/types.rs

	cp ${the_schema} mcdp2-openapi-schema.yaml

	echo "You need to update the version in Cargo.toml"
include rust-common.mk
