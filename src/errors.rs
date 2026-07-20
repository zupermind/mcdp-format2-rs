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
/// Codes are stable dotted hierarchy paths under the repo prefix `MF2R`, so
/// downstream consumers can match on identity across Rust renames. The first
/// segment is the repo prefix, the second groups the failing subsystem, and the
/// leaf segment names the specific failure.
#[derive(zuper_errors2::ZErrorEnum)]
pub enum Mf2rError {
    /// A gzip-compressed payload could not be decompressed.
    #[zerror(code = "MF2R.parse.decompress-failed", locus = Caller, stability = Persistent)]
    Decompress,

    /// Input bytes for a text-based format (YAML or JSON) were not valid UTF-8.
    #[zerror(code = "MF2R.parse.invalid-utf8", locus = Caller, stability = Persistent)]
    Utf8,

    /// A YAML document could not be parsed.
    #[zerror(code = "MF2R.parse.yaml-failed", locus = Caller, stability = Persistent)]
    Yaml,

    /// A CBOR byte stream could not be decoded into a value.
    #[zerror(code = "MF2R.parse.cbor-failed", locus = Caller, stability = Persistent)]
    Cbor,

    /// A JSON document could not be parsed.
    #[zerror(code = "MF2R.parse.json-failed", locus = Caller, stability = Persistent)]
    Json,

    /// The file name did not correspond to a known data format.
    #[zerror(code = "MF2R.parse.unknown-format", locus = Caller, stability = Persistent)]
    #[error("unknown data format: {format}")]
    UnknownFormat { format: String },

    /// A JSON or YAML number had a type with no CBOR representation.
    ///
    /// Defensive: every valid `serde_json` / `serde_yaml` number satisfies
    /// `as_i64()` or `as_f64()`, so the else branch is unreachable via the
    /// public API.
    #[zerror(code = "MF2R.convert.unsupported-number", locus = Caller, stability = Persistent)]
    UnsupportedNumber,

    /// A `ciborium::value::Value` could not be re-encoded to CBOR bytes.
    ///
    /// Defensive: ciborium serialization of the values this crate constructs
    /// does not fail in practice, so this branch has no known trigger via the
    /// public API.
    #[zerror(code = "MF2R.encode.cbor-failed", locus = Implementation, stability = Persistent)]
    Encode,

    /// CBOR bytes could not be deserialized into the requested target type.
    ///
    /// Raised when the on-disk value does not match the shape of `T` in
    /// [`crate::parsing::read`]; a caller supplying a file whose contents do
    /// not fit the requested Root type reaches this kind.
    #[zerror(code = "MF2R.convert.decode-typed", locus = Caller, stability = Persistent)]
    DecodeTyped,

    /// A file could not be opened or read.
    ///
    /// The single kind collapses `File::open` and `read_to_end` failures, which
    /// span caller mistakes (missing path, bad path — persistent), environment
    /// conditions (permissions — persistent), and transient device/interrupt
    /// errors. No single locus or stability is true for the whole kind, so both
    /// axes are left `Unknown` per the manual's rule (chapter 51) to classify
    /// only when the kind itself determines the axis.
    #[zerror(code = "MF2R.fs.read-failed", locus = Unknown, stability = Unknown)]
    ReadFile,

    /// A directory tree could not be traversed.
    ///
    /// Wraps a `walkdir::Error` raised while walking a directory with
    /// `WalkDir`. Such failures span symlink loops (under `follow_links(true)`),
    /// permission denials on a subtree, and transient underlying I/O errors. As
    /// with [`Mf2rError::ReadFile`], no single locus or stability is true for
    /// the whole kind, so both axes are left `Unknown` per the manual's rule
    /// (chapter 51): classify only when the kind itself determines the axis.
    /// The concrete `walkdir::Error` is retained as a cause so a caller can
    /// recover the failed path and underlying reason.
    #[zerror(code = "MF2R.fs.walk-failed", locus = Unknown, stability = Unknown)]
    WalkFailed,

    /// The supplied glob pattern was not valid.
    ///
    /// A binary (CLI) concern: `parse_args` builds the `glob::Pattern` from the
    /// user-supplied string before the library ever sees it.
    #[zerror(code = "MF2R.cli.invalid-pattern", locus = Caller, stability = Persistent)]
    InvalidPattern,
}
