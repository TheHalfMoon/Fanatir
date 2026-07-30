//! Path types for the T033 internal Rust-only project filesystem boundary.
//!
//! `PermittedRoot` and returned `ProjectRoot` values are canonical absolute
//! directories. `ProjectPathRequest` values are validated relative requests under
//! that root. This module is not IPC, capability, persistence, Workspace-root,
//! or production-grade sandbox authority.

use std::fs;
use std::hash::{Hash, Hasher};
#[cfg(windows)]
use std::path::Prefix;
use std::path::{Component, Path, PathBuf};

use super::error::FilesystemAuthorityError;

/// Windows `FILE_ATTRIBUTE_REPARSE_POINT` bit (documented Win32 constant).
#[cfg(windows)]
const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;

/// One immutable canonical permitted local filesystem root.
///
/// Constructed only via [`PermittedRoot::try_open`]. Not persisted. Not a
/// Workspace root. Not attached to `ProjectAuthority`. Internal Rust-only in
/// T033 — no IPC or capability exposure.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PermittedRoot {
    canonical: PathBuf,
}

impl Hash for PermittedRoot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical.hash(state);
    }
}

impl PermittedRoot {
    /// Open an existing absolute local directory as a permitted root.
    ///
    /// The path is validated, inspected for reparse/symlink denial at the root
    /// entry, then canonicalized. TOCTOU hardening is incomplete; this is not a
    /// production-grade sandbox claim.
    pub fn try_open(path: impl AsRef<Path>) -> Result<Self, FilesystemAuthorityError> {
        let path = path.as_ref();
        if !path.is_absolute() {
            return Err(FilesystemAuthorityError::PermittedRootMustBeAbsolute);
        }
        validate_permitted_root_prefix(path)?;

        let meta = fs::symlink_metadata(path).map_err(classify_root_open_io)?;
        if is_denied_link_or_reparse(&meta) {
            return Err(FilesystemAuthorityError::ReparsePointDenied);
        }
        if !meta.is_dir() {
            return Err(FilesystemAuthorityError::PermittedRootNotDirectory);
        }

        let canonical = fs::canonicalize(path).map_err(FilesystemAuthorityError::from_io)?;
        if !canonical.is_absolute() {
            return Err(FilesystemAuthorityError::PermittedRootMustBeAbsolute);
        }
        let canonical_meta =
            fs::symlink_metadata(&canonical).map_err(FilesystemAuthorityError::from_io)?;
        if is_denied_link_or_reparse(&canonical_meta) {
            return Err(FilesystemAuthorityError::ReparsePointDenied);
        }
        if !canonical_meta.is_dir() {
            return Err(FilesystemAuthorityError::PermittedRootNotDirectory);
        }

        Ok(Self { canonical })
    }

    /// Borrow the canonical absolute permitted-root path.
    pub fn as_path(&self) -> &Path {
        &self.canonical
    }
}

/// Validated nonempty relative project-path request.
///
/// Relative to a [`PermittedRoot`]. Invalid requests are rejected; they are
/// never silently normalized. Internal Rust-only in T033.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectPathRequest {
    relative: PathBuf,
}

impl Hash for ProjectPathRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.relative.hash(state);
    }
}

impl ProjectPathRequest {
    /// Validate a UTF-8 project-path request without trimming or case folding.
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self, FilesystemAuthorityError> {
        let raw = raw.as_ref();
        if raw.is_empty() {
            return Err(FilesystemAuthorityError::EmptyProjectPath);
        }

        let path = Path::new(raw);
        if path.is_absolute() {
            return Err(FilesystemAuthorityError::RelativeProjectPathRequired);
        }
        if path
            .components()
            .any(|c| matches!(c, Component::Prefix(_) | Component::RootDir))
        {
            return Err(FilesystemAuthorityError::RelativeProjectPathRequired);
        }

        // Lexical split rejects `.` / `..` even when a platform Path parser
        // collapses current-directory components.
        let mut normals = 0usize;
        for part in raw.split(['/', '\\']) {
            if part.is_empty() {
                return Err(FilesystemAuthorityError::InvalidProjectPathComponent);
            }
            if part == "." || part == ".." {
                return Err(FilesystemAuthorityError::InvalidProjectPathComponent);
            }
            validate_windows_component(part)?;
            normals += 1;
        }
        if normals == 0 {
            return Err(FilesystemAuthorityError::EmptyProjectPath);
        }

        // Also reject any non-normal Path components the platform surfaces.
        for component in path.components() {
            match component {
                Component::Normal(_) => {}
                Component::CurDir | Component::ParentDir => {
                    return Err(FilesystemAuthorityError::InvalidProjectPathComponent);
                }
                Component::Prefix(_) | Component::RootDir => {
                    return Err(FilesystemAuthorityError::RelativeProjectPathRequired);
                }
            }
        }

        Ok(Self {
            relative: PathBuf::from(raw),
        })
    }

    /// Borrow the preserved validated relative request path.
    pub fn as_path(&self) -> &Path {
        &self.relative
    }
}

/// Immutable canonical project-root directory under a permitted root.
///
/// Constructed only by filesystem authority operations after lexical, reparse,
/// and resolved containment checks. Not a Workspace root. Internal Rust-only.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectRoot {
    canonical: PathBuf,
}

impl Hash for ProjectRoot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical.hash(state);
    }
}

impl ProjectRoot {
    pub(super) fn from_canonical(canonical: PathBuf) -> Self {
        Self { canonical }
    }

    /// Borrow the canonical absolute project-root path.
    pub fn as_path(&self) -> &Path {
        &self.canonical
    }
}

fn classify_root_open_io(err: std::io::Error) -> FilesystemAuthorityError {
    match err.kind() {
        std::io::ErrorKind::NotFound => FilesystemAuthorityError::PermittedRootNotFound,
        _ => FilesystemAuthorityError::from_io(err),
    }
}

fn validate_permitted_root_prefix(path: &Path) -> Result<(), FilesystemAuthorityError> {
    #[cfg(windows)]
    {
        let mut components = path.components();
        match components.next() {
            Some(Component::Prefix(prefix)) => match prefix.kind() {
                Prefix::Disk(_) => Ok(()),
                Prefix::VerbatimDisk(_)
                | Prefix::UNC(_, _)
                | Prefix::VerbatimUNC(_, _)
                | Prefix::DeviceNS(_)
                | Prefix::Verbatim(_) => Err(FilesystemAuthorityError::UnsupportedRootPrefix),
            },
            _ => Err(FilesystemAuthorityError::UnsupportedRootPrefix),
        }
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        Ok(())
    }
}

/// Classify metadata as a denied symlink/junction/reparse-point entry.
pub(super) fn is_denied_link_or_reparse(meta: &fs::Metadata) -> bool {
    if meta.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn validate_windows_component(text: &str) -> Result<(), FilesystemAuthorityError> {
    #[cfg(windows)]
    {
        if text.contains(':') {
            return Err(FilesystemAuthorityError::WindowsAlternateDataStream);
        }
        if text.ends_with('.') || text.ends_with(' ') {
            return Err(FilesystemAuthorityError::WindowsTrailingDotOrSpace);
        }
        if is_windows_reserved_name(text) {
            return Err(FilesystemAuthorityError::WindowsReservedName);
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = text;
        Ok(())
    }
}

#[cfg(windows)]
fn is_windows_reserved_name(component: &str) -> bool {
    let basename = match component.split_once('.') {
        Some((stem, _)) => stem,
        None => component,
    };
    let upper = basename.to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    )
}

/// Return whether `candidate` is equal to or strictly beneath `root`.
///
/// Comparison uses encoded path bytes after canonicalization, not lowercase
/// string folding.
pub(super) fn is_path_contained(root: &Path, candidate: &Path) -> bool {
    let root_bytes = root.as_os_str().as_encoded_bytes();
    let candidate_bytes = candidate.as_os_str().as_encoded_bytes();
    if candidate_bytes.len() < root_bytes.len() {
        return false;
    }
    if &candidate_bytes[..root_bytes.len()] != root_bytes {
        return false;
    }
    if candidate_bytes.len() == root_bytes.len() {
        return true;
    }
    matches!(candidate_bytes[root_bytes.len()], b'\\' | b'/')
}

/// Walk existing components beneath `root` along `relative`, denying reparse points.
pub(super) fn deny_reparse_along_relative(
    root: &Path,
    relative: &Path,
) -> Result<(), FilesystemAuthorityError> {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(FilesystemAuthorityError::InvalidProjectPathComponent);
        };
        current.push(name);
        match fs::symlink_metadata(&current) {
            Ok(meta) => {
                if is_denied_link_or_reparse(&meta) {
                    return Err(FilesystemAuthorityError::ReparsePointDenied);
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // Later components may be missing (create path); stop walking.
                break;
            }
            Err(err) => return Err(FilesystemAuthorityError::from_io(err)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempGuard {
        path: PathBuf,
    }

    impl TempGuard {
        fn create() -> Self {
            let base = std::env::temp_dir();
            let pid = std::process::id();
            for _ in 0..64 {
                let n = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
                let candidate = base.join(format!("fanatir-t033-fs-{pid}-{n}"));
                match fs::create_dir(&candidate) {
                    Ok(()) => return Self { path: candidate },
                    Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(_) => continue,
                }
            }
            panic!("failed to create synthetic test directory under temp_dir");
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempGuard {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn project_path_valid_single_and_nested() {
        let single = ProjectPathRequest::try_new("alpha").unwrap();
        assert_eq!(single.as_path(), Path::new("alpha"));
        let nested = ProjectPathRequest::try_new("alpha/beta").unwrap();
        assert_eq!(nested.as_path(), Path::new("alpha/beta"));
    }

    #[test]
    fn project_path_rejects_empty_absolute_dot_dotdot_mixed() {
        assert_eq!(
            ProjectPathRequest::try_new("").unwrap_err(),
            FilesystemAuthorityError::EmptyProjectPath
        );
        assert_eq!(
            ProjectPathRequest::try_new("/abs").unwrap_err(),
            FilesystemAuthorityError::RelativeProjectPathRequired
        );
        assert_eq!(
            ProjectPathRequest::try_new(".").unwrap_err(),
            FilesystemAuthorityError::InvalidProjectPathComponent
        );
        assert_eq!(
            ProjectPathRequest::try_new("..").unwrap_err(),
            FilesystemAuthorityError::InvalidProjectPathComponent
        );
        assert_eq!(
            ProjectPathRequest::try_new("a/../b").unwrap_err(),
            FilesystemAuthorityError::InvalidProjectPathComponent
        );
        assert_eq!(
            ProjectPathRequest::try_new("a/./b").unwrap_err(),
            FilesystemAuthorityError::InvalidProjectPathComponent
        );
    }

    #[test]
    fn project_path_preserves_value_without_case_fold() {
        let req = ProjectPathRequest::try_new("MiXeD_Case-Name").unwrap();
        assert_eq!(req.as_path(), Path::new("MiXeD_Case-Name"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_project_path_ads_trailing_and_reserved() {
        assert_eq!(
            ProjectPathRequest::try_new("name:stream").unwrap_err(),
            FilesystemAuthorityError::WindowsAlternateDataStream
        );
        assert_eq!(
            ProjectPathRequest::try_new("name:").unwrap_err(),
            FilesystemAuthorityError::WindowsAlternateDataStream
        );
        assert_eq!(
            ProjectPathRequest::try_new("folder.").unwrap_err(),
            FilesystemAuthorityError::WindowsTrailingDotOrSpace
        );
        assert_eq!(
            ProjectPathRequest::try_new("folder ").unwrap_err(),
            FilesystemAuthorityError::WindowsTrailingDotOrSpace
        );
        assert_eq!(
            ProjectPathRequest::try_new("CON").unwrap_err(),
            FilesystemAuthorityError::WindowsReservedName
        );
        assert_eq!(
            ProjectPathRequest::try_new("con.txt").unwrap_err(),
            FilesystemAuthorityError::WindowsReservedName
        );
        assert_eq!(
            ProjectPathRequest::try_new("NUL.json").unwrap_err(),
            FilesystemAuthorityError::WindowsReservedName
        );
        assert_eq!(
            ProjectPathRequest::try_new("COM1").unwrap_err(),
            FilesystemAuthorityError::WindowsReservedName
        );
        assert_eq!(
            ProjectPathRequest::try_new("LPT9.log").unwrap_err(),
            FilesystemAuthorityError::WindowsReservedName
        );
        assert_eq!(
            ProjectPathRequest::try_new("AuX").unwrap_err(),
            FilesystemAuthorityError::WindowsReservedName
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_absolute_disk_path_rejected_as_project_request() {
        assert_eq!(
            ProjectPathRequest::try_new(r"C:\escape").unwrap_err(),
            FilesystemAuthorityError::RelativeProjectPathRequired
        );
    }

    #[test]
    fn permitted_root_accepts_absolute_directory_and_stores_canonical() {
        let tmp = TempGuard::create();
        let root = PermittedRoot::try_open(tmp.path()).unwrap();
        assert!(root.as_path().is_absolute());
        let expected = fs::canonicalize(tmp.path()).unwrap();
        assert_eq!(root.as_path(), expected.as_path());
    }

    #[test]
    fn permitted_root_rejects_relative_missing_and_file() {
        assert_eq!(
            PermittedRoot::try_open("relative-root").unwrap_err(),
            FilesystemAuthorityError::PermittedRootMustBeAbsolute
        );
        let tmp = TempGuard::create();
        let missing = tmp.path().join("missing-child");
        assert_eq!(
            PermittedRoot::try_open(&missing).unwrap_err(),
            FilesystemAuthorityError::PermittedRootNotFound
        );
        let file_path = tmp.path().join("not-a-dir");
        File::create(&file_path).unwrap();
        assert_eq!(
            PermittedRoot::try_open(&file_path).unwrap_err(),
            FilesystemAuthorityError::PermittedRootNotDirectory
        );
    }

    #[cfg(windows)]
    #[test]
    fn attribute_classification_detects_reparse_bit() {
        // Deterministic bit check against the documented constant.
        assert_eq!(FILE_ATTRIBUTE_REPARSE_POINT, 0x400);
        let attrs_with = FILE_ATTRIBUTE_REPARSE_POINT | 0x10;
        assert_ne!(attrs_with & FILE_ATTRIBUTE_REPARSE_POINT, 0);
        let attrs_without = 0x10u32;
        assert_eq!(attrs_without & FILE_ATTRIBUTE_REPARSE_POINT, 0);
    }

    #[test]
    fn path_containment_compares_encoded_prefix() {
        let root = Path::new(r"C:\root");
        assert!(is_path_contained(root, Path::new(r"C:\root")));
        assert!(is_path_contained(root, Path::new(r"C:\root\child")));
        assert!(!is_path_contained(root, Path::new(r"C:\root2")));
        assert!(!is_path_contained(root, Path::new(r"C:\elsewhere")));
    }
}
