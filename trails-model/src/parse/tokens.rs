#![allow(dead_code)]

use std::{vec::IntoIter, iter::Peekable, fmt::Debug};

use trails_base::log::debug;
use trails_base::{Result, bail};

/// A wrapper around a mutable position and a token vector for convenient
/// recursive descent parsing.
pub struct Tokens<T> {
    tokens: Peekable<IntoIter<T>>,
}

impl<T> Tokens<T> where T: Debug {
    pub fn new(tokens: Vec<T>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Consumes the next token.
    pub fn next(&mut self) -> Result<T> {
        if let Some(token) = self.tokens.next() {
            debug!("Consuming {:?}", token);
            Ok(token)
        } else {
            bail!("Cannot consume beyond eof")
        }
    }

    /// Peeks the next token.
    pub fn peek(&mut self) -> Result<&T> {
        if let Some(token) = self.tokens.peek() {
            Ok(token)
        } else {
            bail!("Cannot peek beyond eof")
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
            bail!("Expected {:?}, but was {:?}", expected, token)
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
