pub mod ast;
pub mod lex;

/// Represents a location within an input string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Span {
    /// Location of upper-left-most char `[line, column]`.
    pub min: [usize; 2],

    /// Location of bottom-right-most char `[line, column]`.
    pub max: [usize; 2],
}

impl Default for Span {
    fn default() -> Self {
        Self {
            min: [0, 0],
            max: [0, 0],
        }
    }
}

/// Utility trait to return span information
pub trait Spanned {}
