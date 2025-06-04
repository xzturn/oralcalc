use std::fmt;

/// Custom error types for mathematical operations
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Division by zero error
    DivisionByZero,
    /// Division with remainder (for integer division)
    #[allow(dead_code)]
    DivisionHasRemainder,
    /// Invalid leaf node structure
    #[allow(dead_code)]
    InvalidLeafNode,
    /// Invalid operation node
    #[allow(dead_code)]
    InvalidOpNode,
    /// Unknown node type
    #[allow(dead_code)]
    UnknownNode,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::DivisionHasRemainder => write!(f, "Division has remainder"),
            MathError::InvalidLeafNode => write!(f, "Invalid leaf node"),
            MathError::InvalidOpNode => write!(f, "Invalid operation node"),
            MathError::UnknownNode => write!(f, "Unknown node type"),
        }
    }
}

impl std::error::Error for MathError {}
