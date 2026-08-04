//! Guards on the fixture corpus itself, rather than on any one document.
//!
//! `datasets.rs` runs one test case per file under `examples/`, which means an
//! empty or unfetched `examples/` produces *zero* test cases and a green run:
//! the corpus is Git-LFS-tracked (`.gitattributes` has `*.mcdp2.* filter=lfs`),
//! so a checkout without `git lfs checkout` — CI does it in `pre-circle-tests` —
//! leaves pointer files or nothing at all behind, and a harness with no cases to
//! run has nothing to fail. That is the same vacuous pass the binary's
//! `--min-files` closes for `make test`, in the entry point CI actually runs, so
//! it is closed here the same way: by asserting the corpus is populated before
//! trusting that traversing it proved anything.

use std::path::Path;
use std::path::PathBuf;

/// The floor the corpus must clear.
///
/// `examples/` holds 889 documents. The bound is deliberately far below that:
/// its job is to catch an absent or unfetched corpus, not to freeze the exact
/// count and demand an edit here every time a sample is added or retired.
const MIN_EXAMPLE_FILES: usize = 100;

/// Every document `datasets.rs` will traverse.
fn example_documents() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut found = Vec::new();
    let mut pending = vec![root];
    while let Some(dir) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.to_string_lossy().ends_with(".mcdp2.yaml.gz") {
                found.push(path);
            }
        }
    }
    found
}

/// The corpus `datasets.rs` drives is present and populated.
#[test]
fn the_example_corpus_is_populated() {
    let found = example_documents();
    assert!(
        found.len() >= MIN_EXAMPLE_FILES,
        "found {} documents under examples/, expected at least {MIN_EXAMPLE_FILES}; \
         an empty corpus makes the datasets.rs harness pass without reading anything \
         (run `make pre-circle-tests` to fetch the Git-LFS payloads)",
        found.len(),
    );
}

/// Each document is a real gzip payload rather than a Git-LFS pointer.
///
/// A pointer file is short ASCII beginning `version https://git-lfs…`, so it
/// would reach the parser and fail there as a malformed document. Checking the
/// magic separates "the corpus was never fetched" from "the parser is broken",
/// which are different faults with different fixes.
#[test]
fn every_example_document_is_really_gzipped() {
    let found = example_documents();
    assert!(!found.is_empty(), "no documents under examples/");

    let mut not_gzip = Vec::new();
    for path in &found {
        match std::fs::read(path) {
            Ok(bytes) => {
                if bytes.len() < 2 || bytes[0] != 0x1f || bytes[1] != 0x8b {
                    not_gzip.push(path.clone());
                }
            }
            Err(e) => panic!("could not read {}: {e}", path.display()),
        }
    }

    assert!(
        not_gzip.is_empty(),
        "{} of {} documents under examples/ are not gzip payloads (likely unfetched \
         Git-LFS pointers); first few: {:?}",
        not_gzip.len(),
        found.len(),
        not_gzip.iter().take(3).collect::<Vec<_>>(),
    );
}
