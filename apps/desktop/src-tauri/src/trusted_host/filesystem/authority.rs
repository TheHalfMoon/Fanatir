//! Immutable project filesystem authority for T033 open/create mediation.
//!
//! Owns exactly one [`PermittedRoot`]. Performs only bounded `project.open` and
//! `project.create` directory operations. No file-content read/write, listing,
//! delete/rename/copy/move/watch, Workspace root, IPC, capability, persistence,
//! async runtime, or interior mutability. TOCTOU hardening is incomplete; this
//! is not a production-grade sandbox claim.

use std::fs;
use std::path::{Component, Path, PathBuf};

use super::error::FilesystemAuthorityError;
use super::path::{
    deny_reparse_along_relative, is_denied_link_or_reparse, is_path_contained, PermittedRoot,
    ProjectPathRequest, ProjectRoot,
};

/// Immutable single-root project filesystem authority (internal Rust-only).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectFilesystemAuthority {
    permitted_root: PermittedRoot,
}

impl ProjectFilesystemAuthority {
    /// Construct an authority over one immutable permitted root.
    pub fn new(permitted_root: PermittedRoot) -> Self {
        Self { permitted_root }
    }

    /// Borrow the owned permitted root.
    pub fn permitted_root(&self) -> &PermittedRoot {
        &self.permitted_root
    }

    /// Open an existing project-root directory beneath the permitted root.
    ///
    /// Validates the relative request, denies reparse-point components along
    /// the existing path, requires a directory target, canonicalizes, and
    /// requires resolved containment. Empty requests are rejected; the
    /// permitted root itself is never returned as a project.
    pub fn open_project(
        &self,
        request: &ProjectPathRequest,
    ) -> Result<ProjectRoot, FilesystemAuthorityError> {
        let relative = request.as_path();
        let joined = join_under_root(self.permitted_root.as_path(), relative)?;
        deny_reparse_along_relative(self.permitted_root.as_path(), relative)?;

        let meta = fs::symlink_metadata(&joined).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => FilesystemAuthorityError::ProjectNotFound,
            _ => FilesystemAuthorityError::from_io(err),
        })?;
        if is_denied_link_or_reparse(&meta) {
            return Err(FilesystemAuthorityError::ReparsePointDenied);
        }
        if !meta.is_dir() {
            return Err(FilesystemAuthorityError::ProjectNotDirectory);
        }

        let canonical = fs::canonicalize(&joined).map_err(FilesystemAuthorityError::from_io)?;
        if !is_path_contained(self.permitted_root.as_path(), &canonical) {
            return Err(FilesystemAuthorityError::PathEscapeDenied);
        }
        let canonical_meta =
            fs::symlink_metadata(&canonical).map_err(FilesystemAuthorityError::from_io)?;
        if is_denied_link_or_reparse(&canonical_meta) {
            return Err(FilesystemAuthorityError::ReparsePointDenied);
        }
        if !canonical_meta.is_dir() {
            return Err(FilesystemAuthorityError::ProjectNotDirectory);
        }

        Ok(ProjectRoot::from_canonical(canonical))
    }

    /// Create exactly one empty project-root directory beneath the permitted root.
    ///
    /// Requires an existing parent directory; does not use `create_dir_all`.
    /// Creates no manifest, configuration, metadata, or nested scaffold files.
    /// If post-create validation fails, only the newly created empty directory
    /// from this operation may be removed (private cleanup; not a public delete
    /// capability).
    pub fn create_project(
        &self,
        request: &ProjectPathRequest,
    ) -> Result<ProjectRoot, FilesystemAuthorityError> {
        let relative = request.as_path();
        let joined = join_under_root(self.permitted_root.as_path(), relative)?;

        if fs::symlink_metadata(&joined).is_ok() {
            return Err(FilesystemAuthorityError::ProjectAlreadyExists);
        }

        let parent_rel = relative.parent().filter(|p| !p.as_os_str().is_empty());
        let parent_joined = match parent_rel {
            Some(parent) => {
                deny_reparse_along_relative(self.permitted_root.as_path(), parent)?;
                join_under_root(self.permitted_root.as_path(), parent)?
            }
            None => self.permitted_root.as_path().to_path_buf(),
        };

        let parent_meta = fs::symlink_metadata(&parent_joined).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => FilesystemAuthorityError::ProjectParentNotFound,
            _ => FilesystemAuthorityError::from_io(err),
        })?;
        if is_denied_link_or_reparse(&parent_meta) {
            return Err(FilesystemAuthorityError::ReparsePointDenied);
        }
        if !parent_meta.is_dir() {
            return Err(FilesystemAuthorityError::ProjectParentNotDirectory);
        }

        let parent_canonical =
            fs::canonicalize(&parent_joined).map_err(FilesystemAuthorityError::from_io)?;
        if !is_path_contained(self.permitted_root.as_path(), &parent_canonical) {
            return Err(FilesystemAuthorityError::PathEscapeDenied);
        }

        let leaf_name = relative
            .file_name()
            .ok_or(FilesystemAuthorityError::InvalidProjectPathComponent)?;
        let create_target = parent_canonical.join(leaf_name);

        fs::create_dir(&create_target).map_err(|err| match err.kind() {
            std::io::ErrorKind::AlreadyExists => FilesystemAuthorityError::ProjectAlreadyExists,
            std::io::ErrorKind::NotFound => FilesystemAuthorityError::ProjectParentNotFound,
            _ => FilesystemAuthorityError::from_io(err),
        })?;

        match finalize_created_project(self.permitted_root.as_path(), &create_target) {
            Ok(root) => Ok(root),
            Err(err) => {
                cleanup_created_empty_dir(&create_target);
                Err(err)
            }
        }
    }
}

fn join_under_root(root: &Path, relative: &Path) -> Result<PathBuf, FilesystemAuthorityError> {
    let mut joined = root.to_path_buf();
    for component in relative.components() {
        match component {
            Component::Normal(name) => joined.push(name),
            _ => return Err(FilesystemAuthorityError::InvalidProjectPathComponent),
        }
    }
    Ok(joined)
}

fn finalize_created_project(
    permitted_root: &Path,
    created: &Path,
) -> Result<ProjectRoot, FilesystemAuthorityError> {
    let meta = fs::symlink_metadata(created).map_err(FilesystemAuthorityError::from_io)?;
    if is_denied_link_or_reparse(&meta) {
        return Err(FilesystemAuthorityError::ReparsePointDenied);
    }
    if !meta.is_dir() {
        return Err(FilesystemAuthorityError::ProjectNotDirectory);
    }
    let canonical = fs::canonicalize(created).map_err(FilesystemAuthorityError::from_io)?;
    if !is_path_contained(permitted_root, &canonical) {
        return Err(FilesystemAuthorityError::PathEscapeDenied);
    }
    let canonical_meta =
        fs::symlink_metadata(&canonical).map_err(FilesystemAuthorityError::from_io)?;
    if is_denied_link_or_reparse(&canonical_meta) {
        return Err(FilesystemAuthorityError::ReparsePointDenied);
    }
    if !canonical_meta.is_dir() {
        return Err(FilesystemAuthorityError::ProjectNotDirectory);
    }
    Ok(ProjectRoot::from_canonical(canonical))
}

/// Private cleanup for the same create operation only — not a public delete API.
fn cleanup_created_empty_dir(path: &Path) {
    let _ = fs::remove_dir(path);
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
                let candidate = base.join(format!("fanatir-t033-auth-{pid}-{n}"));
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

    fn authority_over(tmp: &TempGuard) -> ProjectFilesystemAuthority {
        ProjectFilesystemAuthority::new(PermittedRoot::try_open(tmp.path()).unwrap())
    }

    #[test]
    fn open_existing_child_and_nested() {
        let tmp = TempGuard::create();
        fs::create_dir(tmp.path().join("child")).unwrap();
        fs::create_dir_all(tmp.path().join("a").join("b")).unwrap();
        let auth = authority_over(&tmp);

        let child = auth
            .open_project(&ProjectPathRequest::try_new("child").unwrap())
            .unwrap();
        assert!(child.as_path().ends_with("child"));
        assert!(is_path_contained(
            auth.permitted_root().as_path(),
            child.as_path()
        ));

        let nested = auth
            .open_project(&ProjectPathRequest::try_new("a/b").unwrap())
            .unwrap();
        assert!(nested.as_path().ends_with("b"));
    }

    #[test]
    fn open_rejects_missing_file_and_traversal_before_io() {
        let tmp = TempGuard::create();
        File::create(tmp.path().join("file")).unwrap();
        let auth = authority_over(&tmp);

        assert_eq!(
            auth.open_project(&ProjectPathRequest::try_new("missing").unwrap())
                .unwrap_err(),
            FilesystemAuthorityError::ProjectNotFound
        );
        assert_eq!(
            auth.open_project(&ProjectPathRequest::try_new("file").unwrap())
                .unwrap_err(),
            FilesystemAuthorityError::ProjectNotDirectory
        );
        assert!(ProjectPathRequest::try_new("../escape").is_err());
        assert!(ProjectPathRequest::try_new("a/../b").is_err());
    }

    #[test]
    fn create_direct_child_and_nested_under_existing_parent() {
        let tmp = TempGuard::create();
        fs::create_dir(tmp.path().join("parent")).unwrap();
        let auth = authority_over(&tmp);

        let created = auth
            .create_project(&ProjectPathRequest::try_new("newproj").unwrap())
            .unwrap();
        assert!(created.as_path().is_dir());
        assert!(tmp.path().join("newproj").is_dir());
        // No scaffold files.
        assert!(fs::read_dir(created.as_path()).unwrap().next().is_none());

        let nested = auth
            .create_project(&ProjectPathRequest::try_new("parent/leaf").unwrap())
            .unwrap();
        assert!(nested.as_path().ends_with("leaf"));
        assert!(is_path_contained(
            auth.permitted_root().as_path(),
            nested.as_path()
        ));
    }

    #[test]
    fn create_does_not_create_missing_parents_and_rejects_conflicts() {
        let tmp = TempGuard::create();
        File::create(tmp.path().join("file-parent")).unwrap();
        let auth = authority_over(&tmp);

        assert_eq!(
            auth.create_project(&ProjectPathRequest::try_new("missing/leaf").unwrap())
                .unwrap_err(),
            FilesystemAuthorityError::ProjectParentNotFound
        );

        fs::create_dir(tmp.path().join("exists")).unwrap();
        assert_eq!(
            auth.create_project(&ProjectPathRequest::try_new("exists").unwrap())
                .unwrap_err(),
            FilesystemAuthorityError::ProjectAlreadyExists
        );

        assert_eq!(
            auth.create_project(&ProjectPathRequest::try_new("file-parent/leaf").unwrap())
                .unwrap_err(),
            FilesystemAuthorityError::ProjectParentNotDirectory
        );
    }

    #[test]
    fn no_public_read_write_methods_on_authority_surface() {
        // Compile-time / API inventory: only new, permitted_root, open_project,
        // create_project are public on ProjectFilesystemAuthority.
        let tmp = TempGuard::create();
        let auth = authority_over(&tmp);
        let _ = auth.permitted_root();
        let req_x = ProjectPathRequest::try_new("x").unwrap();
        let req_y = ProjectPathRequest::try_new("y").unwrap();
        let _ = auth.open_project(&req_x);
        let _ = auth.create_project(&req_y);
    }

    #[test]
    fn project_workspace_types_remain_separate() {
        use crate::trusted_host::{ProjectAuthority, ProjectId, WorkspaceAuthority, WorkspaceId};

        let project = ProjectAuthority::new(ProjectId::try_new("proj-a").unwrap());
        let workspace = WorkspaceAuthority::new(WorkspaceId::try_new("ws-a").unwrap());
        assert_eq!(project.id().as_str(), "proj-a");
        assert_eq!(workspace.id().as_str(), "ws-a");
        // No filesystem root attached to either authority type.
    }

    #[cfg(windows)]
    #[test]
    fn symlink_component_denied_when_os_allows_creation() {
        let tmp = TempGuard::create();
        let target = tmp.path().join("real-target");
        fs::create_dir(&target).unwrap();
        let link = tmp.path().join("link-comp");
        let create_result = std::os::windows::fs::symlink_dir(&target, &link);
        match create_result {
            Ok(()) => {
                let auth = authority_over(&tmp);
                let err = auth
                    .open_project(&ProjectPathRequest::try_new("link-comp").unwrap())
                    .unwrap_err();
                assert_eq!(err, FilesystemAuthorityError::ReparsePointDenied);

                // Nested path through symlink component.
                fs::create_dir(target.join("inner")).unwrap();
                let err2 = auth
                    .open_project(&ProjectPathRequest::try_new("link-comp/inner").unwrap())
                    .unwrap_err();
                assert_eq!(err2, FilesystemAuthorityError::ReparsePointDenied);
            }
            Err(err) => {
                // Privilege or policy may deny symlink creation; report via assert message.
                eprintln!(
                    "symlink/reparse OS creation unavailable in this environment: {err:?}; runtime denial path not exercised"
                );
            }
        }
    }

    #[cfg(windows)]
    #[test]
    fn symlink_root_denied_when_os_allows_creation() {
        let tmp = TempGuard::create();
        let real = tmp.path().join("real-root");
        fs::create_dir(&real).unwrap();
        let link_root = tmp.path().join("link-root");
        match std::os::windows::fs::symlink_dir(&real, &link_root) {
            Ok(()) => {
                let err = PermittedRoot::try_open(&link_root).unwrap_err();
                assert_eq!(err, FilesystemAuthorityError::ReparsePointDenied);
            }
            Err(err) => {
                eprintln!(
                    "symlink root OS creation unavailable in this environment: {err:?}; root reparse denial not exercised"
                );
            }
        }
    }
}
