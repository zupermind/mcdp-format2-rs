.PHONY: check cargo-check cargo-check-wasm test coverage lint docs tag upload upload-no-verify

check: cargo-check cargo-check-wasm

cargo-check:
	cargo check --workspace --keep-going --all-targets

cargo-check-wasm:
	cargo check --workspace --keep-going --lib --target wasm32-unknown-unknown

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

# sigil f62ef07a4de0063523ac9716d47ec130
# template-meta template-version=2.1
# template-meta zuper-templating-version=8.33.2901010000
