//! Structured, machine-readable semantic errors.

use std::fmt;

use super::{BinaryOp, Type};

/// Stable category for a semantic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// The operands do not satisfy an operation's type rule.
    TypeMismatch,
}

impl ErrorCode {
    /// Returns the stable machine-readable error code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypeMismatch => "TYPE_MISMATCH",
        }
    }
}

/// Operand type constraint expected by an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandTypes {
    /// Both operands must have the specified type.
    Both(Type),
    /// Both operands must have the same type.
    Same,
}

/// A structured semantic verification error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticError {
    /// Stable error category.
    pub code: ErrorCode,
    /// Operation whose rule was violated.
    pub operation: BinaryOp,
    /// Type inferred for the left operand.
    pub left_type: Type,
    /// Type inferred for the right operand.
    pub right_type: Type,
    /// Operand constraint required by the operation.
    pub expected: OperandTypes,
}

impl SemanticError {
    pub(crate) const fn type_mismatch(
        operation: BinaryOp,
        left_type: Type,
        right_type: Type,
        expected: OperandTypes,
    ) -> Self {
        Self {
            code: ErrorCode::TypeMismatch,
            operation,
            left_type,
            right_type,
            expected,
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.code.as_str())
    }
}

impl std::error::Error for SemanticError {}

