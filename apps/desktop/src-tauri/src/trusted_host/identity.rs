//! Shared fail-closed identity validation for T032 authority stubs.
//!
//! Validation is a bounded identifier contract only. It is not a persistent ID
//! standard, path interpreter, or permission grant.

use std::error::Error;
use std::fmt;

/// Maximum UTF-8 byte length for a validated authority identifier.
pub(super) const MAX_ID_BYTES: usize = 128;

/// Deterministic construction error for project/workspace identity stubs.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AuthorityIdError {
    /// Identifier was empty.
    Empty,
    /// Identifier exceeded 128 UTF-8 bytes.
    TooLong,
    /// Identifier had leading or trailing whitespace.
    SurroundingWhitespace,
    /// Identifier contained a Unicode control character.
    ControlCharacter,
}

impl fmt::Display for AuthorityIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "authority identifier must not be empty"),
            Self::TooLong => write!(
                f,
                "authority identifier must not exceed {MAX_ID_BYTES} UTF-8 bytes"
            ),
            Self::SurroundingWhitespace => {
                write!(
                    f,
                    "authority identifier must not have leading or trailing whitespace"
                )
            }
            Self::ControlCharacter => {
                write!(
                    f,
                    "authority identifier must not contain control characters"
                )
            }
        }
    }
}

impl Error for AuthorityIdError {}

/// Validate and preserve an opaque authority identifier value.
pub(super) fn validate_authority_id(raw: &str) -> Result<(), AuthorityIdError> {
    if raw.is_empty() {
        return Err(AuthorityIdError::Empty);
    }
    if raw.len() > MAX_ID_BYTES {
        return Err(AuthorityIdError::TooLong);
    }
    let mut chars = raw.chars();
    let Some(first) = chars.next() else {
        return Err(AuthorityIdError::Empty);
    };
    let last = chars.next_back().unwrap_or(first);
    if first.is_whitespace() || last.is_whitespace() {
        return Err(AuthorityIdError::SurroundingWhitespace);
    }
    if raw.chars().any(|c| c.is_control()) {
        return Err(AuthorityIdError::ControlCharacter);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_strings_are_deterministic() {
        assert_eq!(
            AuthorityIdError::Empty.to_string(),
            "authority identifier must not be empty"
        );
        assert_eq!(
            AuthorityIdError::TooLong.to_string(),
            "authority identifier must not exceed 128 UTF-8 bytes"
        );
        assert_eq!(
            AuthorityIdError::SurroundingWhitespace.to_string(),
            "authority identifier must not have leading or trailing whitespace"
        );
        assert_eq!(
            AuthorityIdError::ControlCharacter.to_string(),
            "authority identifier must not contain control characters"
        );
    }
}
