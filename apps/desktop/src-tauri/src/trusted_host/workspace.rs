//! Workspace identity and non-executable workspace-authority stub (T032).
//!
//! These types grant no permission, perform no I/O, and are not persistence,
//! IPC, or Supabase contracts. Project–Workspace relationship is intentionally
//! undefined.

use super::identity::{validate_authority_id, AuthorityIdError};

/// Opaque, strongly typed workspace identifier.
///
/// Distinct from [`crate::trusted_host::ProjectId`]. Grants no authority.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct WorkspaceId {
    value: String,
}

impl WorkspaceId {
    /// Construct a validated workspace identifier, preserving the exact input.
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self, AuthorityIdError> {
        let raw = raw.as_ref();
        validate_authority_id(raw)?;
        Ok(Self {
            value: raw.to_owned(),
        })
    }

    /// Borrow the preserved identifier text.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Type-only workspace authority stub.
///
/// Owns exactly one [`WorkspaceId`]. Grants no permission, stores no
/// collaboration or Supabase state, and encodes no project relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceAuthority {
    id: WorkspaceId,
}

impl WorkspaceAuthority {
    /// Construct a non-executable workspace authority stub around `id`.
    pub fn new(id: WorkspaceId) -> Self {
        Self { id }
    }

    /// Read-only access to the owned workspace identifier.
    pub fn id(&self) -> &WorkspaceId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trusted_host::ProjectId;
    use std::any::TypeId;

    #[test]
    fn valid_workspace_id_construction_preserves_value() {
        let id = WorkspaceId::try_new("ws-alpha-01").expect("valid id");
        assert_eq!(id.as_str(), "ws-alpha-01");
    }

    #[test]
    fn empty_workspace_id_is_rejected() {
        assert_eq!(WorkspaceId::try_new(""), Err(AuthorityIdError::Empty));
    }

    #[test]
    fn workspace_id_rejects_129_bytes() {
        let raw = "b".repeat(129);
        assert_eq!(WorkspaceId::try_new(&raw), Err(AuthorityIdError::TooLong));
    }

    #[test]
    fn workspace_id_rejects_leading_whitespace() {
        assert_eq!(
            WorkspaceId::try_new(" leading"),
            Err(AuthorityIdError::SurroundingWhitespace)
        );
    }

    #[test]
    fn workspace_id_rejects_trailing_whitespace() {
        assert_eq!(
            WorkspaceId::try_new("trailing "),
            Err(AuthorityIdError::SurroundingWhitespace)
        );
    }

    #[test]
    fn workspace_id_rejects_control_character() {
        assert_eq!(
            WorkspaceId::try_new("bad\tid"),
            Err(AuthorityIdError::ControlCharacter)
        );
    }

    #[test]
    fn project_and_workspace_types_are_distinct() {
        assert_ne!(TypeId::of::<WorkspaceId>(), TypeId::of::<ProjectId>());
    }

    #[test]
    fn workspace_authority_preserves_workspace_id() {
        let id = WorkspaceId::try_new("ws-keep").expect("valid id");
        let authority = WorkspaceAuthority::new(id.clone());
        assert_eq!(authority.id(), &id);
        assert_eq!(authority.id().as_str(), "ws-keep");
    }

    #[test]
    fn workspace_authority_has_no_project_field() {
        // Compile-time shape: WorkspaceAuthority only owns WorkspaceId.
        let authority = WorkspaceAuthority::new(WorkspaceId::try_new("ws-only").expect("valid"));
        let _id: &WorkspaceId = authority.id();
        let _ = _id;
    }
}
