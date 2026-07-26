//! Project identity and non-executable project-authority stub (T032).
//!
//! These types grant no permission, perform no I/O, and are not persistence or
//! IPC contracts. Project–Workspace relationship is intentionally undefined.

use super::identity::{validate_authority_id, AuthorityIdError};

/// Opaque, strongly typed project identifier.
///
/// Distinct from [`crate::trusted_host::WorkspaceId`]. Grants no authority.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProjectId {
    value: String,
}

impl ProjectId {
    /// Construct a validated project identifier, preserving the exact input.
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

/// Type-only project authority stub.
///
/// Owns exactly one [`ProjectId`]. Grants no permission, stores no path, and
/// encodes no workspace relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectAuthority {
    id: ProjectId,
}

impl ProjectAuthority {
    /// Construct a non-executable project authority stub around `id`.
    pub fn new(id: ProjectId) -> Self {
        Self { id }
    }

    /// Read-only access to the owned project identifier.
    pub fn id(&self) -> &ProjectId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trusted_host::WorkspaceId;
    use std::any::TypeId;

    #[test]
    fn valid_project_id_construction_preserves_value() {
        let id = ProjectId::try_new("proj-alpha-01").expect("valid id");
        assert_eq!(id.as_str(), "proj-alpha-01");
    }

    #[test]
    fn empty_project_id_is_rejected() {
        assert_eq!(ProjectId::try_new(""), Err(AuthorityIdError::Empty));
    }

    #[test]
    fn project_id_rejects_129_bytes() {
        let raw = "a".repeat(129);
        assert_eq!(ProjectId::try_new(&raw), Err(AuthorityIdError::TooLong));
    }

    #[test]
    fn project_id_rejects_leading_whitespace() {
        assert_eq!(
            ProjectId::try_new(" leading"),
            Err(AuthorityIdError::SurroundingWhitespace)
        );
    }

    #[test]
    fn project_id_rejects_trailing_whitespace() {
        assert_eq!(
            ProjectId::try_new("trailing "),
            Err(AuthorityIdError::SurroundingWhitespace)
        );
    }

    #[test]
    fn project_id_rejects_control_character() {
        assert_eq!(
            ProjectId::try_new("bad\nid"),
            Err(AuthorityIdError::ControlCharacter)
        );
    }

    #[test]
    fn project_and_workspace_types_are_distinct() {
        assert_ne!(TypeId::of::<ProjectId>(), TypeId::of::<WorkspaceId>());
    }

    #[test]
    fn project_authority_preserves_project_id() {
        let id = ProjectId::try_new("proj-keep").expect("valid id");
        let authority = ProjectAuthority::new(id.clone());
        assert_eq!(authority.id(), &id);
        assert_eq!(authority.id().as_str(), "proj-keep");
    }

    #[test]
    fn project_authority_has_no_workspace_field() {
        // Compile-time shape: ProjectAuthority only owns ProjectId.
        let authority = ProjectAuthority::new(ProjectId::try_new("proj-only").expect("valid"));
        let _id: &ProjectId = authority.id();
        let _ = _id;
    }
}
