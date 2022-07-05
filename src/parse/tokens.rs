use std::{vec::IntoIter, iter::Peekable};

use crate::error::{Result, Error};

/// A wrapper around a mutable position and a token vector for convenient
/// recursive descent parsing.
pub struct Tokens<T> {
    tokens: Peekable<IntoIter<T>>,
}

impl<T> Tokens<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Consumes the next token.
    pub fn next(&mut self) -> Result<T> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(Error::Eof)
        }
    }

    /// Peeks the next token.
    pub fn peek(&mut self) -> Result<&T> {
        if let Some(token) = self.tokens.peek() {
            Ok(token)
        } else {
            Err(Error::Eof)
        }
    }
}
