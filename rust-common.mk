.PHONY: check cargo-check test coverage lint docs tag upload upload-no-verify

check: cargo-check

cargo-check:
	cargo check --workspace --keep-going --all-targets

test:
	cargo nextest run --release --workspace --all-targets

coverage:
	cargo llvm-cov nextest --release --workspace --all-targets --no-report --test-threads 4
	cargo llvm-cov report --release --html --output-dir tmp/coverage
	cargo llvm-cov report --release --summary-only

lint:
	zuper-rs-lint run --manifest-path Cargo.toml

docs:
	$(MAKE) -C docs

tag:
	zuper-figaro-cargo tag --extra-tag-suffix /$(SCOPE) --allow-dirty

upload:
	zuper-figaro-cargo upload --which last-tag --allow-dirty

upload-no-verify:
	zuper-figaro-cargo upload --which last-tag --allow-dirty --no-verify

# sigil 3c5b3b5d94da7b021a496fec2d0b0242
# template-meta template-version=2.6
# template-meta zuper-templating-version=16.15.2610201932
