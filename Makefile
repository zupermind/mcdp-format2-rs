all:

test:
	cargo run --  ../../samples

test-release:
	cargo run -r --  ../../samples

test-verbose:
	cargo run --  --verbose ../../samples



generate:
	$(MAKE) -C ../../mcdp-formats preprocess
	cargo run -p zuper-rs-schemas --bin zuper-rs-schemas -- rust  \
		--only-concrete \
		--schema ../../mcdp-formats/out/schema-no-concrete.yaml \
		--target src/types.rs

