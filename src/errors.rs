//! Domain error kind for `mcdp-format2-rs`.
//!
//! Every fallible operation in this crate reports a [`Mf2rError`] leaf kind
//! carried by a `zuper_errors2::ZError`. Foreign errors (gzip decompression,
//! UTF-8 decoding, `serde_yaml` / `serde_json` / `ciborium` decode, and
//! `ciborium` encode) enter the typed tree through the `zerror_from!` /
//! `zerror_from_kv!` boundary macros, which retain the concrete source error
//! and the capture location as a cause of the classifying domain kind.
//!
//! The enum is crate-wide: both the library (`parsing`) and the binary
//! (`main`) report through it. The library's public functions propagate the
//! same kind that the binary returns, so a single enum keeps every code
//! absolute (repo prefix first) and lets `?` widen library failures into the
//! binary without a redundant wrapper segment (DEC-CODES: flatten trivial
//! composition, reuse the code identity).

/// Failure modes for MCDP Format 2 loading, format detection, value
/// conversion, and serialization.
///
/// Codes are derived by `ZErrorEnum` from the enum and variant identifiers, so
/// `Mf2rError::CannotOpenFile` has code `Mf2rError.CannotOpenFile`. No variant
/// declares an explicit `#[zerror(code = "...")]`: routine code overrides are a
/// checklist violation (`ZERRORS2-NO-ROUTINE-CODE-OVERRIDES`), and a downstream
/// consumer matching on identity should follow a variant rename rather than
/// have the rename hidden behind a frozen string.
#[derive(zuper_errors2::ZErrorEnum)]
pub enum Mf2rError {
    /// A gzip-compressed payload could not be decompressed.
    #[zerror(locus = Caller, stability = Persistent)]
    Decompress,

    /// Input bytes for a text-based format (YAML or JSON) were not valid UTF-8.
    #[zerror(locus = Caller, stability = Persistent)]
    Utf8,

    /// A YAML document could not be parsed.
    #[zerror(locus = Caller, stability = Persistent)]
    CannotParseYamlDocument,

    /// A CBOR byte stream could not be decoded into a value.
    #[zerror(locus = Caller, stability = Persistent)]
    Cbor,

    /// A JSON document could not be parsed.
    #[zerror(locus = Caller, stability = Persistent)]
    CannotParseJsonDocument,

    /// The file name did not correspond to a known data format.
    #[zerror(locus = Caller, stability = Persistent)]
    #[error("unknown data format: {format}")]
    UnknownFormat {
        /// The unrecognized file-name suffix as supplied by the caller, not a
        /// rendered form of the surrounding `DataFormat` value.
        format: String,
    },

    /// A JSON or YAML number had a type with no CBOR representation.
    ///
    /// Defensive: every valid `serde_json` / `serde_yaml` number satisfies
    /// `as_i64()` or `as_f64()`, so the else branch is unreachable via the
    /// public API.
    #[zerror(locus = Caller, stability = Persistent)]
    UnsupportedNumber,

    /// A `ciborium::value::Value` could not be re-encoded to CBOR bytes.
    ///
    /// Defensive: ciborium serialization of the values this crate constructs
    /// does not fail in practice, so this branch has no known trigger via the
    /// public API.
    #[zerror(locus = Implementation, stability = Persistent)]
    Encode,

    /// CBOR bytes could not be deserialized into the requested target type.
    ///
    /// Raised when the on-disk value does not match the shape of `T` in
    /// [`crate::parsing::read`]; a caller supplying a file whose contents do
    /// not fit the requested Root type reaches this kind.
    #[zerror(locus = Caller, stability = Persistent)]
    DecodeTyped,

    /// A file could not be opened.
    ///
    /// Raised by `File::open`. The failure spans caller mistakes (missing path,
    /// bad path) and environment conditions (permission denial), so neither
    /// axis is determined by the kind and both are left `Unknown` per the
    /// manual's rule (chapter 51): classify only when the kind itself
    /// determines the axis. The concrete `std::io::Error` is retained as a
    /// cause, so a caller can recover `ErrorKind::NotFound` from
    /// `PermissionDenied`.
    CannotOpenFile,

    /// An opened file's contents could not be read to the end.
    ///
    /// Raised by `read_to_end` after `File::open` already succeeded, so it is
    /// never a missing-path fault: it covers device errors, interrupted reads,
    /// and a file truncated or removed underneath the open handle. Both axes
    /// stay `Unknown` for the same reason as [`Mf2rError::CannotOpenFile`].
    CannotReadFileContents,

    /// A directory tree contains a symlink loop.
    ///
    /// Under `follow_links(true)`, `WalkDir` reports an ancestor loop rather
    /// than recursing forever. The tree handed in is malformed for a following
    /// traversal and will stay so until it is changed, which fixes both axes:
    /// the fault is in the caller's input and it is persistent. The concrete
    /// `walkdir::Error` is retained as a cause, so `loop_ancestor()` remains
    /// recoverable.
    #[zerror(locus = Caller, stability = Persistent)]
    DirectorySymlinkLoop,

    /// A directory entry could not be read while traversing a tree.
    ///
    /// The non-loop half of a `WalkDir` failure: permission denial on a
    /// subtree, a vanished entry, or a transient underlying I/O error. No
    /// single locus or stability is true for the whole kind, so both axes stay
    /// `Unknown` per the manual's rule (chapter 51). The concrete
    /// `walkdir::Error` is retained as a cause so a caller can recover the
    /// failed path and underlying reason.
    CannotReadDirectoryEntry,

    /// The supplied glob pattern was not valid.
    ///
    /// A binary (CLI) concern: `parse_args` builds the `glob::Pattern` from the
    /// user-supplied string before the library ever sees it.
    #[zerror(locus = Caller, stability = Persistent)]
    InvalidPattern,

    /// Fewer documents matched the invocation than it declared it needed.
    ///
    /// A binary (CLI) concern, and the kind that keeps a run from passing
    /// vacuously. Walking a tree that holds no matching document is not an
    /// error to the traversal itself — `list_paths` legitimately returns an
    /// empty list — so without this kind the processing loop's body never runs
    /// and `main` returns `Ok(())`: a corpus path that no longer exists, an
    /// unfetched Git-LFS payload, or a `--pattern` that no longer matches the
    /// corpus naming all report success having read nothing. `--min-files`
    /// turns the expected corpus size into a checked precondition, and this
    /// kind is how the check fails.
    ///
    /// The shortfall is the caller's: the roots, the pattern, or the minimum
    /// were wrong, and re-running the same invocation against the same tree
    /// keeps producing the same count. The matched count, the required
    /// minimum, the pattern, and the search roots ride along as annotation
    /// attributes so the report says which corpus came up short and by how
    /// much.
    #[zerror(locus = Caller, stability = Persistent)]
    TooFewFiles,
}
