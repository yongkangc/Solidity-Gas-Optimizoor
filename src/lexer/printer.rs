use crate::LexedOutput;

use super::token::Token;

/// Pretty prints the source code given tokens from the lexer.
pub fn generate_output(output: LexedOutput) -> String {
    let mut output_string = String::new();
    let slices = output.slices;
    // Iterate through the slices to print out the source code
    for slice in slices {
        output_string.push_str(&slice);
    }
    output_string
}
