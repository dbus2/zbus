use crate::Result;
use crate::Signature;

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
        self.signature.chars().nth(self.pos).ok_or_else(|| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Other("end of signature"),
                &"a signature character",
            )
        })
    }

    pub fn parse_char(&mut self, expected: Option<char>) -> Result<()> {
        let c = self.next_char()?;
        if let Some(expected) = expected {
            if c != expected {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Char(c),
                    &expected.to_string().as_str(),
                ));
            }
        }
        self.skip_chars(1)?;

        Ok(())
    }

    pub fn skip_chars(&mut self, num_chars: usize) -> Result<()> {
        self.pos += num_chars;

        // We'll be going one char beyond at the end of parsing but not beyond that.
        if self.pos > self.signature.len() {
            return Err(serde::de::Error::invalid_length(
                self.signature.len(),
                &format!(">= {} characters", self.pos).as_str(),
            ));
        }

        Ok(())
    }

    pub fn rewind_chars(&mut self, num_chars: usize) {
        self.pos -= num_chars;
    }
}
