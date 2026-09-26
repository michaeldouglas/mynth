//! Semantic constants and their values.

use super::Type;

/// A constant value in the initial Mynth semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    /// A signed 32-bit integer constant.
    I32(i32),
    /// A boolean constant.
    Bool(bool),
    /// The unit constant.
    Unit,
}

impl Value {
    /// Returns the semantic type of this value.
    pub const fn ty(self) -> Type {
        match self {
            Self::I32(_) => Type::I32,
            Self::Bool(_) => Type::Bool,
            Self::Unit => Type::Unit,
        }
    }
}

