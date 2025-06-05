// src/parser.rs

use crate::ast::AstNode;
use crate::token::Token;

/// Takes the flat `Vec<Token>` stream and splits on Newline, bundling each line’s tokens into one `AstNode::Line`.
pub fn parse_lines(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut nodes = Vec::new();
    let mut current = Vec::new();

    for tok in tokens {
        match tok {
            Token::Newline => {
                if !current.is_empty() {
                    nodes.push(AstNode::Line(current.clone()));
                    current.clear();
                }
            }
            other => {
                current.push(other.clone());
            }
        }
    }
    // In case the file doesn’t end with a blank line
    if !current.is_empty() {
        nodes.push(AstNode::Line(current));
    }
    nodes
}

