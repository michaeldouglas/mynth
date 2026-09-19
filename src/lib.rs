//! First semantic core of the Mynth language.
//!
//! This crate intentionally exposes only a representation-independent semantic
//! expression tree and its verifier. Parsing, textual syntax, interpretation,
//! LLVM, OpenVINO, and model integration are outside this initial scope.

pub mod semantic;
pub mod verifier;

pub use semantic::{BinaryOp, ErrorCode, Expr, OperandTypes, SemanticError, Type, Value};
pub use verifier::verify;

