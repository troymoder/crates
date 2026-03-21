use std::path::{Path, PathBuf};

use thiserror::Error;

/// Zero-based line and column range from a protobuf [`Location::span`](https://protobuf.dev/reference/cpp/api-docs/google.protobuf.source_code_info/#sourcecodeinfo-location-span).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProtoSpan {
    /// Zero-based start line.
    pub start_line: u32,
    /// Zero-based start column.
    pub start_column: u32,
    /// Zero-based end line.
    pub end_line: u32,
    /// Zero-based end column.
    pub end_column: u32,
}

/// Invalid `Location.span` slice.
#[derive(Debug, Error, Eq, PartialEq)]
pub(crate) enum ProtoSpanParseError {
    /// `span` must have length 3 or 4.
    #[error("protobuf Location.span must have 3 or 4 elements, got {0}")]
    BadLength(usize),
    /// Line or column values must be non-negative.
    #[error("protobuf Location.span contains a negative component")]
    NegativeComponent,
}

/// Decodes `Location.span`: three elements are `(start_line, start_column, end_column)` with
/// `end_line` equal to `start_line`; four elements add an explicit end line.
pub(crate) fn parse_proto_location_span(span: &[i32]) -> Result<ProtoSpan, ProtoSpanParseError> {
    let neg = |v: i32| v < 0;
    match span.len() {
        3 => {
            let (sl, sc, ec) = (span[0], span[1], span[2]);
            if neg(sl) || neg(sc) || neg(ec) {
                return Err(ProtoSpanParseError::NegativeComponent);
            }
            let line = sl as u32;
            Ok(ProtoSpan {
                start_line: line,
                start_column: sc as u32,
                end_line: line,
                end_column: ec as u32,
            })
        }
        4 => {
            let (sl, sc, el, ec) = (span[0], span[1], span[2], span[3]);
            if neg(sl) || neg(sc) || neg(el) || neg(ec) {
                return Err(ProtoSpanParseError::NegativeComponent);
            }
            Ok(ProtoSpan {
                start_line: sl as u32,
                start_column: sc as u32,
                end_line: el as u32,
                end_column: ec as u32,
            })
        }
        n => Err(ProtoSpanParseError::BadLength(n)),
    }
}

/// Resolves `FileDescriptorProto.name` (for example `foo/bar.proto`) to a readable path by trying
/// `root.join(proto_name)` for each `include_root`, in order, like `protoc -I`.
pub(crate) fn resolve_proto_file_path(
    proto_name: &str,
    include_roots: &[impl AsRef<Path>],
) -> Option<PathBuf> {
    include_roots
        .iter()
        .map(AsRef::as_ref)
        .map(|root| root.join(proto_name))
        .find(|p| p.is_file())
}

/// Result of mapping a [`prost_reflect::FileDescriptor`] and [`Location`](prost_reflect::prost_types::source_code_info::Location) to a logical name, span, and optional disk path.
#[cfg(feature = "prost")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedProtoLocation {
    /// Value of [`FileDescriptorProto.name`](prost_reflect::prost_types::FileDescriptorProto::name) for the owning file.
    pub proto_name: String,
    /// Parsed [`Location::span`](prost_reflect::prost_types::source_code_info::Location::span).
    pub span: ProtoSpan,
    /// First existing file found via [`resolve_proto_file_path`], when `include_roots` is non-empty.
    pub source_path: Option<PathBuf>,
}

/// Maps a source location from `file`’s embedded [`SourceCodeInfo`](prost_reflect::prost_types::SourceCodeInfo) to the logical `.proto` name, a parsed span, and optionally an on-disk path.
///
/// The span always refers to text in the file named by [`FileDescriptor::name`](prost_reflect::FileDescriptor::name); protobuf records one `SourceCodeInfo` per parsed file.
#[cfg(feature = "prost")]
pub(crate) fn resolve_proto_source_location(
    file: &prost_reflect::FileDescriptor,
    location: &prost_reflect::prost_types::source_code_info::Location,
    include_roots: &[impl AsRef<Path>],
) -> Result<ResolvedProtoLocation, ProtoSpanParseError> {
    let span = parse_proto_location_span(&location.span)?;
    let proto_name = file.name().to_string();
    let source_path = if include_roots.is_empty() {
        None
    } else {
        resolve_proto_file_path(&proto_name, include_roots)
    };
    Ok(ResolvedProtoLocation {
        proto_name,
        span,
        source_path,
    })
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn parse_three_element_span() {
        assert_eq!(
            parse_proto_location_span(&[2, 5, 18]).unwrap(),
            ProtoSpan {
                start_line: 2,
                start_column: 5,
                end_line: 2,
                end_column: 18,
            }
        );
    }

    #[test]
    fn parse_four_element_span() {
        assert_eq!(
            parse_proto_location_span(&[1, 0, 3, 40]).unwrap(),
            ProtoSpan {
                start_line: 1,
                start_column: 0,
                end_line: 3,
                end_column: 40,
            }
        );
    }

    #[test]
    fn parse_rejects_bad_lengths() {
        assert_eq!(
            parse_proto_location_span(&[1, 2]).unwrap_err(),
            ProtoSpanParseError::BadLength(2)
        );
        assert_eq!(
            parse_proto_location_span(&[1, 2, 3, 4, 5]).unwrap_err(),
            ProtoSpanParseError::BadLength(5)
        );
    }

    #[test]
    fn parse_rejects_negative() {
        assert_eq!(
            parse_proto_location_span(&[-1, 0, 1]).unwrap_err(),
            ProtoSpanParseError::NegativeComponent
        );
    }

    #[test]
    fn resolve_finds_first_include_match() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("pkg");
        std::fs::create_dir_all(&nested).unwrap();
        let proto_path = nested.join("a.proto");
        std::fs::write(&proto_path, "syntax = \"proto3\";").unwrap();

        let wrong = tempfile::tempdir().unwrap();
        assert_eq!(
            resolve_proto_file_path("pkg/a.proto", &[wrong.path(), dir.path()]),
            Some(proto_path)
        );
    }
}
