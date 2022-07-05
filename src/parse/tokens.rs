use std::{vec::IntoIter, iter::Peekable, fmt::Debug};

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

impl<T> Tokens<T> where T: Eq + Debug {
    /// Consumes a token by expecting one.
    pub fn expect(&mut self, expected: &T) -> Result<()> {
        let token = self.next()?;
        if &token == expected {
            Ok(())
        } else {
            Err(Error::UnexpectedToken(format!("Expected {:?}, but was {:?}", expected, token)))
        }
    }

    /// Consumes a token if it matches, otherwise does nothing.
    pub fn expect_optionally(&mut self, expected: &T) -> Result<bool> {
        let token = self.peek()?;
        if token == expected {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
