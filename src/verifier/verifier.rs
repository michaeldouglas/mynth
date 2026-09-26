//! Type verification for representation-independent semantic expressions.

use crate::semantic::{BinaryOp, Expr, OperandTypes, SemanticError, Type};

/// Determines the type of an expression or returns a structured type error.
pub fn verify(expression: &Expr) -> Result<Type, SemanticError> {
    match expression {
        Expr::Constant(value) => Ok(value.ty()),
        Expr::Add(left, right) => verify_binary(BinaryOp::Add, left, right),
        Expr::Sub(left, right) => verify_binary(BinaryOp::Sub, left, right),
        Expr::Mul(left, right) => verify_binary(BinaryOp::Mul, left, right),
        Expr::Eq(left, right) => verify_binary(BinaryOp::Eq, left, right),
        Expr::Lt(left, right) => verify_binary(BinaryOp::Lt, left, right),
    }
}

fn verify_binary(
    operation: BinaryOp,
    left: &Expr,
    right: &Expr,
) -> Result<Type, SemanticError> {
    let left_type = verify(left)?;
    let right_type = verify(right)?;

    match (operation, left_type, right_type) {
        (BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul, Type::I32, Type::I32) => {
            Ok(Type::I32)
        }
        (BinaryOp::Lt, Type::I32, Type::I32) => Ok(Type::Bool),
        (BinaryOp::Eq, left, right) if left == right => Ok(Type::Bool),
        (operation, left_type, right_type) => {
            let expected = match operation {
                BinaryOp::Eq => OperandTypes::Same,
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Lt => {
                    OperandTypes::Both(Type::I32)
                }
            };

            Err(SemanticError::type_mismatch(
                operation,
                left_type,
                right_type,
                expected,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::verify;
    use crate::semantic::{BinaryOp, ErrorCode, Expr, OperandTypes, Type, Value};

    fn i32_constant(value: i32) -> Expr {
        Expr::constant(Value::I32(value))
    }

    fn bool_constant(value: bool) -> Expr {
        Expr::constant(Value::Bool(value))
    }

    #[test]
    fn verifies_constant_types() {
        assert_eq!(verify(&i32_constant(7)), Ok(Type::I32));
        assert_eq!(verify(&bool_constant(true)), Ok(Type::Bool));
        assert_eq!(verify(&Expr::constant(Value::Unit)), Ok(Type::Unit));
    }

    #[test]
    fn verifies_integer_arithmetic() {
        assert_eq!(
            verify(&Expr::add(i32_constant(1), i32_constant(2))),
            Ok(Type::I32)
        );
        assert_eq!(
            verify(&Expr::sub(i32_constant(3), i32_constant(2))),
            Ok(Type::I32)
        );
        assert_eq!(
            verify(&Expr::mul(i32_constant(2), i32_constant(3))),
            Ok(Type::I32)
        );
    }

    #[test]
    fn verifies_comparisons() {
        assert_eq!(
            verify(&Expr::lt(i32_constant(1), i32_constant(2))),
            Ok(Type::Bool)
        );
        assert_eq!(
            verify(&Expr::eq(i32_constant(1), i32_constant(1))),
            Ok(Type::Bool)
        );
        assert_eq!(
            verify(&Expr::eq(bool_constant(true), bool_constant(false))),
            Ok(Type::Bool)
        );
        assert_eq!(
            verify(&Expr::eq(Expr::constant(Value::Unit), Expr::constant(Value::Unit))),
            Ok(Type::Bool)
        );
    }

    #[test]
    fn rejects_mismatched_add_operands_with_machine_readable_error() {
        let result = verify(&Expr::add(i32_constant(1), bool_constant(true)));

        assert_eq!(
            result,
            Err(crate::semantic::SemanticError {
                code: ErrorCode::TypeMismatch,
                operation: BinaryOp::Add,
                left_type: Type::I32,
                right_type: Type::Bool,
                expected: OperandTypes::Both(Type::I32),
            })
        );
        let error = result.expect_err("the mixed-type addition must be rejected");
        assert_eq!(error.code.as_str(), "TYPE_MISMATCH");
        assert_eq!(error.to_string(), "TYPE_MISMATCH");
    }

    #[test]
    fn rejects_non_integer_less_than_operands() {
        let result = verify(&Expr::lt(bool_constant(false), bool_constant(true)));

        assert_eq!(
            result,
            Err(crate::semantic::SemanticError {
                code: ErrorCode::TypeMismatch,
                operation: BinaryOp::Lt,
                left_type: Type::Bool,
                right_type: Type::Bool,
                expected: OperandTypes::Both(Type::I32),
            })
        );
    }

    #[test]
    fn rejects_equality_between_different_types() {
        let result = verify(&Expr::eq(i32_constant(1), bool_constant(true)));

        assert_eq!(
            result,
            Err(crate::semantic::SemanticError {
                code: ErrorCode::TypeMismatch,
                operation: BinaryOp::Eq,
                left_type: Type::I32,
                right_type: Type::Bool,
                expected: OperandTypes::Same,
            })
        );
    }

    #[test]
    fn rejects_nested_type_errors() {
        let expression = Expr::mul(
            Expr::add(i32_constant(1), bool_constant(false)),
            i32_constant(2),
        );

        let error = verify(&expression).expect_err("the invalid nested expression must fail");
        assert_eq!(error.operation, BinaryOp::Add);
        assert_eq!(error.code, ErrorCode::TypeMismatch);
    }
}

