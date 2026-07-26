//! Deterministic filesystem-authority errors for T033.
//!
//! Error messages contain no path values, PHI, document content, or dynamic OS
//! text. `IoFailure` exposes only `std::io::ErrorKind`.

use std::error::Error;
use std::fmt;
use std::io;

/// Fail-closed error for bounded project open/create mediation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FilesystemAuthorityError {
    /// Project path request was empty.
    EmptyProjectPath,
    /// Project path request was not a nonempty relative path.
    RelativeProjectPathRequired,
    /// A path component was not a normal relative component.
    InvalidProjectPathComponent,
    /// A Windows reserved device name appeared in a component.
    WindowsReservedName,
    /// Alternate data stream syntax (`:`) appeared in a component.
    WindowsAlternateDataStream,
    /// A component ended with a trailing dot or trailing space.
    WindowsTrailingDotOrSpace,
    /// Permitted root was not an absolute path.
    PermittedRootMustBeAbsolute,
    /// Permitted root used an unsupported prefix (UNC/device/non-disk).
    UnsupportedRootPrefix,
    /// Permitted root path did not exist.
    PermittedRootNotFound,
    /// Permitted root path existed but was not a directory.
    PermittedRootNotDirectory,
    /// A symlink, junction, or reparse-point component was denied.
    ReparsePointDenied,
    /// Requested project directory did not exist.
    ProjectNotFound,
    /// Requested project path existed but was not a directory.
    ProjectNotDirectory,
    /// Requested project directory already existed.
    ProjectAlreadyExists,
    /// Parent of the requested project create target did not exist.
    ProjectParentNotFound,
    /// Parent of the requested project create target was not a directory.
    ProjectParentNotDirectory,
    /// Canonical target resolved outside the permitted root.
    PathEscapeDenied,
    /// An I/O failure occurred; only the error kind is retained.
    IoFailure(io::ErrorKind),
}

impl fmt::Display for FilesystemAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProjectPath => write!(f, "project path must not be empty"),
            Self::RelativeProjectPathRequired => {
                write!(f, "project path must be a nonempty relative path")
            }
            Self::InvalidProjectPathComponent => {
                write!(f, "project path contains an invalid component")
            }
            Self::WindowsReservedName => {
                write!(f, "project path contains a Windows reserved device name")
            }
            Self::WindowsAlternateDataStream => {
                write!(
                    f,
                    "project path must not contain Windows alternate data stream syntax"
                )
            }
            Self::WindowsTrailingDotOrSpace => {
                write!(
                    f,
                    "project path components must not end with a trailing dot or space"
                )
            }
            Self::PermittedRootMustBeAbsolute => {
                write!(f, "permitted root must be an absolute path")
            }
            Self::UnsupportedRootPrefix => {
                write!(f, "permitted root uses an unsupported path prefix")
            }
            Self::PermittedRootNotFound => write!(f, "permitted root was not found"),
            Self::PermittedRootNotDirectory => {
                write!(f, "permitted root must be a directory")
            }
            Self::ReparsePointDenied => {
                write!(
                    f,
                    "symlink, junction, or reparse-point components are denied"
                )
            }
            Self::ProjectNotFound => write!(f, "project directory was not found"),
            Self::ProjectNotDirectory => write!(f, "project path must be a directory"),
            Self::ProjectAlreadyExists => write!(f, "project directory already exists"),
            Self::ProjectParentNotFound => {
                write!(f, "project parent directory was not found")
            }
            Self::ProjectParentNotDirectory => {
                write!(f, "project parent path must be a directory")
            }
            Self::PathEscapeDenied => {
                write!(f, "resolved path escapes the permitted root")
            }
            Self::IoFailure(kind) => write!(f, "filesystem I/O failure: {kind:?}"),
        }
    }
}

impl Error for FilesystemAuthorityError {}

impl FilesystemAuthorityError {
    pub(super) fn from_io(err: io::Error) -> Self {
        Self::IoFailure(err.kind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_strings_are_deterministic() {
        let cases: &[(FilesystemAuthorityError, &str)] = &[
            (
                FilesystemAuthorityError::EmptyProjectPath,
                "project path must not be empty",
            ),
            (
                FilesystemAuthorityError::RelativeProjectPathRequired,
                "project path must be a nonempty relative path",
            ),
            (
                FilesystemAuthorityError::InvalidProjectPathComponent,
                "project path contains an invalid component",
            ),
            (
                FilesystemAuthorityError::WindowsReservedName,
                "project path contains a Windows reserved device name",
            ),
            (
                FilesystemAuthorityError::WindowsAlternateDataStream,
                "project path must not contain Windows alternate data stream syntax",
            ),
            (
                FilesystemAuthorityError::WindowsTrailingDotOrSpace,
                "project path components must not end with a trailing dot or space",
            ),
            (
                FilesystemAuthorityError::PermittedRootMustBeAbsolute,
                "permitted root must be an absolute path",
            ),
            (
                FilesystemAuthorityError::UnsupportedRootPrefix,
                "permitted root uses an unsupported path prefix",
            ),
            (
                FilesystemAuthorityError::PermittedRootNotFound,
                "permitted root was not found",
            ),
            (
                FilesystemAuthorityError::PermittedRootNotDirectory,
                "permitted root must be a directory",
            ),
            (
                FilesystemAuthorityError::ReparsePointDenied,
                "symlink, junction, or reparse-point components are denied",
            ),
            (
                FilesystemAuthorityError::ProjectNotFound,
                "project directory was not found",
            ),
            (
                FilesystemAuthorityError::ProjectNotDirectory,
                "project path must be a directory",
            ),
            (
                FilesystemAuthorityError::ProjectAlreadyExists,
                "project directory already exists",
            ),
            (
                FilesystemAuthorityError::ProjectParentNotFound,
                "project parent directory was not found",
            ),
            (
                FilesystemAuthorityError::ProjectParentNotDirectory,
                "project parent path must be a directory",
            ),
            (
                FilesystemAuthorityError::PathEscapeDenied,
                "resolved path escapes the permitted root",
            ),
            (
                FilesystemAuthorityError::IoFailure(io::ErrorKind::PermissionDenied),
                "filesystem I/O failure: PermissionDenied",
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(err.to_string(), *expected);
        }
    }

    #[test]
    fn io_failure_excludes_path_and_os_source_text() {
        let err = FilesystemAuthorityError::from_io(io::Error::new(
            io::ErrorKind::NotFound,
            "C:\\secret\\patient\\record.txt was missing",
        ));
        let text = err.to_string();
        assert_eq!(text, "filesystem I/O failure: NotFound");
        assert!(!text.contains('\\'));
        assert!(!text.contains("patient"));
        assert!(!text.contains("record"));
        assert!(!text.contains("missing"));
    }
}
