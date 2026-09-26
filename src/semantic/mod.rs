//! Representation-independent semantic building blocks.

pub mod error;
pub mod expr;
pub mod types;
pub mod value;

pub use error::{ErrorCode, OperandTypes, SemanticError};
pub use expr::{BinaryOp, Expr};
pub use types::Type;
pub use value::Value;

