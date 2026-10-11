use crate::*;
use core::fmt;

/// Error returned when a value is outside the allowed range of a
/// [`FieldNumber`] (`1..=`[`FieldNumber::MAX_ALLOWED_U32`]).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FieldNumberIntoError {
    pub(crate) invalid_value: u32,
}

impl FieldNumberIntoError {
    /// Value that could not be converted to a [`FieldNumber`].
    #[inline]
    pub const fn invalid_value(&self) -> u32 {
        self.invalid_value
    }
}

impl fmt::Display for FieldNumberIntoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field number error: Value '{}' is outside the allowed range of '1..={}'.",
            self.invalid_value,
            FieldNumber::MAX_ALLOWED_U32
        )
    }
}

impl core::error::Error for FieldNumberIntoError {}

#[cfg(test)]
mod test {
    use super::*;
    use core::error::Error;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn invalid_value() {
        assert_eq!(0, FieldNumber::try_from_u32(0).unwrap_err().invalid_value());
        assert_eq!(
            FieldNumber::MAX_ALLOWED_U32 + 1,
            FieldNumber::try_from_u32(FieldNumber::MAX_ALLOWED_U32 + 1)
                .unwrap_err()
                .invalid_value()
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            "FieldNumberIntoError { invalid_value: 0 }",
            format!("{:?}", FieldNumberIntoError { invalid_value: 0 })
        );
    }

    #[test]
    fn clone_eq_hash() {
        let err = FieldNumberIntoError { invalid_value: 0 };
        assert_eq!(err, err.clone());
        let hash_a = {
            let mut hasher = DefaultHasher::new();
            err.hash(&mut hasher);
            hasher.finish()
        };
        let hash_b = {
            let mut hasher = DefaultHasher::new();
            err.clone().hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash_a, hash_b);
    }

    #[test]
    fn fmt() {
        assert_eq!(
            "Field number error: Value '0' is outside the allowed range of '1..=536870911'.",
            format!("{}", FieldNumberIntoError { invalid_value: 0 })
        );
    }

    #[test]
    fn source() {
        assert!(FieldNumberIntoError { invalid_value: 0 }.source().is_none());
    }
}
