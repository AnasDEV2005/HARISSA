// src/ast.rs

/// For now, one AST node per source line:
#[derive(Debug)]
pub enum AstNode {
    /// Represents a single line of tokens (raw storage)
    Line(Vec<crate::scanner::Token>),
}

