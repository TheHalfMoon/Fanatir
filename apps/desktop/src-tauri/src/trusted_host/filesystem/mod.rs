//! T033 internal Rust-only bounded project filesystem mediation.
//!
//! Provides lexical, canonical, and Windows reparse-point containment for a
//! single immutable permitted local root with `project.open` / `project.create`
//! directory operations only.
//!
//! This module does **not** define IPC transport, capability grants,
//! persistence, Workspace roots, or a Project–Workspace relationship.
//!
//! **TOCTOU limitation:** checks and mutations are not race-free against
//! concurrent filesystem remounts or adversarial mutation. OS-handle-level
//! hardening is required before production security claims. This is not a
//! production-grade sandbox, PHI isolation, or compliance certification claim.

mod authority;
mod error;
mod path;

pub use authority::ProjectFilesystemAuthority;
pub use error::FilesystemAuthorityError;
pub use path::{PermittedRoot, ProjectPathRequest, ProjectRoot};

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use std::fs;

    #[test]
    fn public_api_surface_is_exactly_approved_types() {
        // Type presence inventory for the five approved exports.
        fn assert_types(
            _: FilesystemAuthorityError,
            _: PermittedRoot,
            _: ProjectFilesystemAuthority,
            _: ProjectPathRequest,
            _: ProjectRoot,
        ) {
        }

        let tmp = std::env::temp_dir().join(format!(
            "fanatir-t033-boundary-{}-{}",
            std::process::id(),
            0u64
        ));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir(&tmp).unwrap();
        let root = PermittedRoot::try_open(&tmp).unwrap();
        let auth = ProjectFilesystemAuthority::new(root.clone());
        let req = ProjectPathRequest::try_new("p").unwrap();
        let _ = auth.create_project(&req);
        let opened = auth.open_project(&req).unwrap();
        assert_types(
            FilesystemAuthorityError::EmptyProjectPath,
            root,
            auth,
            req,
            opened,
        );
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn authored_source_has_no_tauri_command_or_state_markers() {
        // Build needles at runtime so this test source does not contain the
        // forbidden attribute text as a contiguous literal.
        let command_marker = format!("#[{}]", "tauri::command");
        let state_marker = format!("{}::{}", "tauri", "State");
        let invoke_marker = format!("{}{}", "invoke_", "handler");
        let generate_marker = format!("{}{}", "generate_", "handler");
        let sources = [
            include_str!("mod.rs"),
            include_str!("error.rs"),
            include_str!("path.rs"),
            include_str!("authority.rs"),
        ];
        for src in sources {
            assert!(!src.contains(&command_marker));
            assert!(!src.contains(&state_marker));
            assert!(!src.contains(&invoke_marker));
            assert!(!src.contains(&generate_marker));
        }
    }
}
