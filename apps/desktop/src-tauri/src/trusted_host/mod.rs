//! Compile-time Trusted Host authority boundary (T032 skeleton + T033 FS).
//!
//! Types in this module are non-executable stubs except for the internal
//! Rust-only bounded filesystem mediation under `filesystem`. They grant no
//! WebView permission, define no IPC transport, and are not persistence
//! contracts. Project and Workspace relationship is intentionally undefined.

mod filesystem;
mod identity;
mod project;
mod workspace;

pub use filesystem::{
    FilesystemAuthorityError, PermittedRoot, ProjectFilesystemAuthority, ProjectPathRequest,
    ProjectRoot,
};
pub use identity::AuthorityIdError;
pub use project::{ProjectAuthority, ProjectId};
pub use workspace::{WorkspaceAuthority, WorkspaceId};
