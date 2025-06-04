// src/worksheet.rs
use crate::binary_op::BinaryOp;
use crate::tree_node::TreeNode;
use crate::errors::MathError;
use rand::Rng;

/// Generates math worksheets with configurable layout
pub struct WorksheetGenerator {
    rows: u32,
    wsep: u32,
}

impl WorksheetGenerator {
    /// Creates a new worksheet generator with the specified layout parameters
    pub fn new(rows: u32, wsep: u32) -> Self {
        // Even if cols is passed, the new logic will fix it to 3 types of problems per row.
        // We could assert cols == 3 or ignore the input `cols` if it's fixed.
        // For now, let's assume the intention is that each row has these 3 problems.
        // If `cols` from args is meant to repeat this 3-problem pattern, the logic would differ.
        // Based on "每一行是3列", it means a fixed 3 columns per row.
        Self { rows, wsep } // Forcing cols to 3 as per new requirement
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
        print!("{}", answers); // Consider if answers should be on a separate page or section

        Ok(())
    }

    /// Generates a random two-digit by two-digit addition expression
    /// e.g., 23 + 45
    fn generate_two_digit_addition(&self) -> Result<TreeNode, MathError> {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(10..=99);
        let y = rng.gen_range(10..=99);
        // Ensure result doesn't exceed a certain number of digits if necessary,
        // e.g., 99 + 99 = 198 (3 digits). This is generally fine.
        TreeNode::build_operation(BinaryOp::Add, x, y)
    }

    /// Generates a random two-digit by two-digit subtraction expression, ensuring positive result
    /// e.g., 67 - 12
    fn generate_two_digit_subtraction(&self) -> Result<TreeNode, MathError> {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(10..=99);
        let mut y = rng.gen_range(10..=99);

        if x < y {
            std::mem::swap(&mut x, &mut y); // Ensure x >= y for a non-negative result
        }
        // To ensure it's not X - X = 0 too often, though random chance makes this rare.
        // if x == y { y = rng.gen_range(10..x.saturating_sub(1)) } // Optional: avoid X-X if y can be 0.
                                                                // Current range 10..=99 prevents y being 0.
        TreeNode::build_operation(BinaryOp::Sub, x, y)
    }

    /// Generates a random one-digit by two-digit multiplication expression
    /// e.g., 7 × 34
    fn generate_one_by_two_digit_multiplication(&self) -> Result<TreeNode, MathError> {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(2..=9);    // One-digit number (2-9)
        let y = rng.gen_range(10..=99); // Two-digit number
        TreeNode::build_operation(BinaryOp::Mul, x, y)
    }


    /// Generates both the problems and their corresponding answers according to the new 3-column structure
    fn generate_problems_and_answers(&self) -> Result<(String, String), MathError> {
        let mut problems_text = String::new();
        let mut answers_text = String::new();

        let col_separator = " ".repeat(self.wsep as usize);
        // Answer separator might need adjustment based on result widths.
        // Add: XX+XX=YYY (max 3 digits). Sub: XX-XX=YY (max 2 digits). Mul: X*XX=YYY (max 3 digits)
        // Let's use a slightly more generous answer separator than problem separator to accommodate results.
        let answer_col_separator = " ".repeat((self.wsep as usize).saturating_sub(2).max(4));


        for _row_idx in 0..self.rows {
            let mut current_row_problems = Vec::with_capacity(3);
            let mut current_row_answers = Vec::with_capacity(3);

            // Column 1: Two-digit + Two-digit
            let expr1 = self.generate_two_digit_addition()?;
            let res1 = expr1.evaluate()?;
            current_row_problems.push(format!("{} = ", expr1.format_expression(0)));
            current_row_answers.push(format!("{} = {:3}", expr1.format_expression(0), res1));

            // Column 2: Two-digit - Two-digit
            let expr2 = self.generate_two_digit_subtraction()?;
            let res2 = expr2.evaluate()?;
            current_row_problems.push(format!("{} = ", expr2.format_expression(0)));
            current_row_answers.push(format!("{} = {:3}", expr2.format_expression(0), res2));

            // Column 3: One-digit * Two-digit
            let expr3 = self.generate_one_by_two_digit_multiplication()?;
            let res3 = expr3.evaluate()?;
            current_row_problems.push(format!("{} = ", expr3.format_expression(0)));
            current_row_answers.push(format!("{} = {:3}", expr3.format_expression(0), res3));

            // Assemble the row for problems text
            problems_text.push_str(&current_row_problems.join(&col_separator));
            problems_text.push_str("\n\n\n"); // Newlines after each row of problems

            // Assemble the row for answers text
            answers_text.push_str(&current_row_answers.join(&answer_col_separator));
            answers_text.push_str("\n\n\n"); // Newlines after each row of answers
        }

        Ok((problems_text, answers_text))
    }
}
