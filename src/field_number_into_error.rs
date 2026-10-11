#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FieldNumberIntoError {
    pub(crate) invalid_value: u32,
}
