.PHONY: check cargo-check install

check: cargo-check

cargo-check:
	cargo check --workspace --all-targets --keep-going

install:
	cargo install --path . --locked --force

all:

test:
	cargo run --  ../../samples

test-release:
	cargo run -r --  ../../samples

test-verbose:
	cargo run --  --verbose ../../samples

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
