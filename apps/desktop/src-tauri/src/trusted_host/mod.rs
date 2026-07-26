//! Compile-time Trusted Host authority boundary (T032 skeleton).
//!
//! Types in this module are non-executable stubs. They grant no permission,
//! perform no I/O, and are not persistence or IPC contracts. Project and
//! Workspace relationship is intentionally undefined.

mod identity;
mod project;
mod workspace;

pub use identity::AuthorityIdError;
pub use project::{ProjectAuthority, ProjectId};
pub use workspace::{WorkspaceAuthority, WorkspaceId};
