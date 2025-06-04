use crate::binary_op::BinaryOp;
use crate::tree_node::TreeNode;
use crate::errors::MathError;
use rand::Rng;

/// Generates math worksheets with configurable layout
pub struct WorksheetGenerator {
    rows: u32,
    cols: u32, 
    wsep: u32,
}

impl WorksheetGenerator {
    /// Creates a new worksheet generator with the specified layout parameters
    pub fn new(rows: u32, cols: u32, wsep: u32) -> Self {
        Self { rows, cols, wsep }
    }
    
    /// Generates the specified number of worksheet pages
    pub fn generate_pages(&self, pages: u32) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..pages {
            self.generate_single_page()?;
        }
        Ok(())
    }
    
    /// Generates a single page of math problems with answers
    fn generate_single_page(&self) -> Result<(), MathError> {
        let (problems, answers) = self.generate_problems_and_answers()?;
        
        // Print problems
        print!("{}", problems);
        
        // Print answers  
        print!("{}", answers);
        
        Ok(())
    }
    
    /// Generates both the problems and their corresponding answers
    fn generate_problems_and_answers(&self) -> Result<(String, String), MathError> {
        let mut problems = String::new();
        let mut answers = String::new();
        
        let separator = " ".repeat(self.wsep as usize);
        let answer_separator = " ".repeat((self.wsep as usize).saturating_sub(4));
        
        for _row in 0..self.rows {
            for col in 0..self.cols {
                let expression = self.generate_two_digit_expression()?;
                let result = expression.evaluate()?;
                
                // Format problem
                let problem_str = format!("{} = ", expression.format_expression(2));
                
                // Format answer  
                let answer_str = format!("{} = {:3}", expression.format_expression(2), result);
                
                if col < self.cols - 1 {
                    // Not the last column, add separator
                    problems.push_str(&problem_str);
                    problems.push_str(&separator);
                    
                    answers.push_str(&answer_str);
                    answers.push_str(&answer_separator);
                } else {
                    // Last column, add newlines
                    problems.push_str(&problem_str);
                    problems.push_str("\n\n\n");
                    
                    answers.push_str(&answer_str);
                    answers.push_str("\n\n\n");
                }
            }
        }
        
        Ok((problems, answers))
    }
    
    /// Generates a random two-digit addition or subtraction expression
    fn generate_two_digit_expression(&self) -> Result<TreeNode, MathError> {
        let mut rng = rand::thread_rng();
        
        let mut x = rng.gen_range(10..=99);
        let mut y = rng.gen_range(10..=99);
        
        let op = if rng.gen_bool(0.5) {
            // For subtraction, ensure x >= y to avoid negative results
            if x < y {
                std::mem::swap(&mut x, &mut y);
            }
            BinaryOp::Sub
        } else {
            BinaryOp::Add
        };
        
        TreeNode::build_operation(op, x, y)
    }
}
