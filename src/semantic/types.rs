//! Semantic types supported by the initial Mynth core.

/// The type of a semantic Mynth value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    /// A signed 32-bit integer.
    I32,
    /// A boolean value.
    Bool,
    /// The unit value, representing the absence of a meaningful value.
    Unit,
}

impl Type {
    /// Returns the stable machine-readable name of this type.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::I32 => "I32",
            Self::Bool => "BOOL",
            Self::Unit => "UNIT",
        }
    }
}

