use crate::binary_op::BinaryOp;
use crate::errors::MathError;
use std::fmt;

/// Represents a node in a binary expression tree
/// Can be either a leaf node (containing an integer value) or an operation node
#[derive(Debug, Clone)]
pub enum TreeNode {
    /// Leaf node containing an integer value
    Value(i32),
    /// Operation node with operator and left/right children
    Operation {
        op: BinaryOp,
        left: Box<TreeNode>,
        right: Box<TreeNode>,
    },
}

impl TreeNode {
    /// Creates a new leaf node with the given value
    pub fn new_value(value: i32) -> Self {
        TreeNode::Value(value)
    }
    
    /// Creates a new operation node with the given operator and operands
    pub fn new_operation(op: BinaryOp, left: TreeNode, right: TreeNode) -> Self {
        TreeNode::Operation {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
    /// Builds an operation node and validates it can be evaluated
    pub fn build_operation(op: BinaryOp, a: i32, b: i32) -> Result<Self, MathError> {
        let node = TreeNode::new_operation(
            op,
            TreeNode::new_value(a),
            TreeNode::new_value(b),
        );
        
        // Validate by attempting to evaluate
        node.evaluate()?;
        Ok(node)
    }
    
    /// Returns the priority of this node (for parentheses handling)
    pub fn priority(&self) -> i32 {
        match self {
            TreeNode::Value(_) => 2, // Highest priority (default)
            TreeNode::Operation { op, .. } => op.priority(),
        }
    }
    
    /// Evaluates the expression tree and returns the result
    pub fn evaluate(&self) -> Result<i32, MathError> {
        match self {
            TreeNode::Value(v) => Ok(*v),
            TreeNode::Operation { op, left, right } => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                
                match op {
                    BinaryOp::Add => Ok(left_val + right_val),
                    BinaryOp::Sub => Ok(left_val - right_val),
                    BinaryOp::Mul => Ok(left_val * right_val),
                    BinaryOp::Div => {
                        if right_val == 0 {
                            Err(MathError::DivisionByZero)
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                }
            }
        }
    }
    
    /// Formats the expression as a string with proper parentheses
    pub fn format_expression(&self, width: usize) -> String {
        match self {
            TreeNode::Value(v) => format!("{:width$}", v, width = width),
            TreeNode::Operation { op, left, right } => {
                let left_str = if left.priority() < op.priority() {
                    format!("({})", left.format_expression(0))
                } else {
                    left.format_expression(0)
                };
                
                let right_str = if right.priority() <= op.priority() {
                    format!("({})", right.format_expression(0))
                } else {
                    right.format_expression(0)
                };
                
                if width > 0 {
                    format!("{:width$} {} {}", left_str, op, right_str, width = width)
                } else {
                    format!("{} {} {}", left_str, op, right_str)
                }
            }
        }
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_expression(0))
    }
}
