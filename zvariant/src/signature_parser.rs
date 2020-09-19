use crate::Result;
use crate::Signature;

#[derive(Debug, Clone)]
pub(crate) struct SignatureParser<'s> {
    signature: Signature<'s>,
    pos: usize,
}

impl<'s> SignatureParser<'s> {
    pub fn new(signature: Signature<'s>) -> Self {
        Self { signature, pos: 0 }
    }

    pub fn signature(&self) -> Signature {
        Signature::from_str_unchecked(&self.signature[self.pos..])
    }

    pub fn next_char(&self) -> char {
        // SAFETY: Other methods that increment `self.pos` must ensure we don't go beyond signature
        // length.
        char::from(self.signature.as_bytes()[self.pos])
    }

    #[inline]
    pub fn skip_char(&mut self) -> Result<()> {
        self.skip_chars(1)
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
}
