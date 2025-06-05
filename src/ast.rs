// src/ast.rs
use crate::token::Token;

/// For now, one AST node per source line:
#[derive(Debug)]
pub enum AstNode {
    /// Represents a single line of tokens (raw storage)
    Line(Vec<Token>),
}

