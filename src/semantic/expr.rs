//! Representation-independent semantic expressions.

use super::Value;

/// Binary operations supported by the initial Mynth semantic core.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    /// Integer addition.
    Add,
    /// Integer subtraction.
    Sub,
    /// Integer multiplication.
    Mul,
    /// Equality comparison.
    Eq,
    /// Integer less-than comparison.
    Lt,
}

impl BinaryOp {
    /// Returns the stable machine-readable name of this operation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Add => "ADD",
            Self::Sub => "SUB",
            Self::Mul => "MUL",
            Self::Eq => "EQ",
            Self::Lt => "LT",
        }
    }
}

/// A semantic expression tree with no dependency on textual syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A constant value.
    Constant(Value),
    /// Integer addition.
    Add(Box<Self>, Box<Self>),
    /// Integer subtraction.
    Sub(Box<Self>, Box<Self>),
    /// Integer multiplication.
    Mul(Box<Self>, Box<Self>),
    /// Equality comparison.
    Eq(Box<Self>, Box<Self>),
    /// Integer less-than comparison.
    Lt(Box<Self>, Box<Self>),
}

impl Expr {
    /// Builds a constant expression.
    pub const fn constant(value: Value) -> Self {
        Self::Constant(value)
    }

    /// Builds an addition expression.
    pub fn add(left: Self, right: Self) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    /// Builds a subtraction expression.
    pub fn sub(left: Self, right: Self) -> Self {
        Self::Sub(Box::new(left), Box::new(right))
    }

    /// Builds a multiplication expression.
    pub fn mul(left: Self, right: Self) -> Self {
        Self::Mul(Box::new(left), Box::new(right))
    }

    /// Builds an equality expression.
    pub fn eq(left: Self, right: Self) -> Self {
        Self::Eq(Box::new(left), Box::new(right))
    }

    /// Builds a less-than expression.
    pub fn lt(left: Self, right: Self) -> Self {
        Self::Lt(Box::new(left), Box::new(right))
    }
}

