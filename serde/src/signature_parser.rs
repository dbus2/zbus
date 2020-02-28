use crate::Signature;
use crate::{Error, Result};

pub(crate) struct SignatureParser<'s> {
    signature: Signature<'s>,
    pos: usize,
}

impl<'s> SignatureParser<'s> {
    pub fn new(signature: Signature<'s>) -> Self {
        Self { signature, pos: 0 }
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn next_char(&self) -> Result<char> {
        // TODO: Better error here with more info
        self.signature
            .chars()
            .nth(self.pos)
            .ok_or(Error::InsufficientData)
    }

    pub fn parse_char(&mut self, expected: Option<char>) -> Result<()> {
        let c = self.next_char()?;
        if let Some(expected) = expected {
            if c != expected {
                // TODO: Better error here with more info
                return Err(Error::IncorrectType);
            }
        }
        self.skip_chars(1)?;

        Ok(())
    }

    pub fn skip_chars(&mut self, num_chars: usize) -> Result<()> {
        self.pos += num_chars;

        // We'll be going one char beyond at the end of parsing but not beyond that.
        if self.pos > self.signature.len() {
            // TODO: Better error
            return Err(Error::InsufficientData);
        }

        Ok(())
    }

    pub fn rewind_chars(&mut self, num_chars: usize) {
        self.pos -= num_chars;
    }
}
