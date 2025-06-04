use std::fmt;

/// Represents binary arithmetic operations with their precedence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub, 
    // Currently not used in problems but kept for API completeness
    #[allow(dead_code)]
    Mul,
    #[allow(dead_code)]
    Div,
}

impl BinaryOp {
    /// Returns the operator precedence (higher number = higher precedence)
    /// Addition and subtraction have precedence 0
    /// Multiplication and division have precedence 1
    pub fn priority(self) -> i32 {
        match self {
            BinaryOp::Add | BinaryOp::Sub => 0,
            BinaryOp::Mul | BinaryOp::Div => 1,
        }
    }
    
    /// Returns the string representation of the operator
    pub fn symbol(self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-", 
            BinaryOp::Mul => "×",
            BinaryOp::Div => "÷",
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
