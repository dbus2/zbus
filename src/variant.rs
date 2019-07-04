use byteorder::ByteOrder;
use std::borrow::Cow;
use std::{error, fmt, str};

#[derive(Debug)]
pub enum VariantError {
    IncorrectType,
    InvalidUtf8,
    InsufficientData,
    UnsupportedType,
}

impl error::Error for VariantError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantError::IncorrectType => write!(f, "incorrect type"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType => write!(f, "unsupported type"),
        }
    }
}

pub trait VariantType<'a>: Sized {
    const SIGNATURE: char;
    const SIGNATURE_STR: &'static str;
    fn encode(&self) -> Vec<u8>;
    fn extract_slice(data: &'a [u8]) -> Result<&'a [u8], VariantError>;

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a;
}

impl<'a> VariantType<'a> for u32 {
    const SIGNATURE: char = 'u';
    const SIGNATURE_STR: &'static str = "u";

    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4);
        bytes.extend(&self.to_ne_bytes());

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..4])
    }

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        Ok(byteorder::NativeEndian::read_u32(bytes))
    }
}

impl<'a> VariantType<'a> for &'a str {
    const SIGNATURE: char = 's';
    const SIGNATURE_STR: &'static str = "s";

    fn encode(&self) -> Vec<u8> {
        let len = self.len();
        let mut bytes = Vec::with_capacity(5 + len);

        bytes.extend(&(len as u32).to_ne_bytes());
        bytes.extend(self.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = byteorder::NativeEndian::read_u32(bytes) as usize + 5;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        str::from_utf8(&bytes[4..]).map_err(|_| VariantError::InvalidUtf8)
    }
}

pub struct ObjectPath<'a>(pub &'a str);

// FIXME: Find a way to share code with &str implementation above
impl<'a> VariantType<'a> for ObjectPath<'a> {
    const SIGNATURE: char = 'o';
    const SIGNATURE_STR: &'static str = "o";

    fn encode(&self) -> Vec<u8> {
        let len = self.0.len();
        let mut bytes = Vec::with_capacity(5 + len);

        bytes.extend(&(len as u32).to_ne_bytes());
        bytes.extend(self.0.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = byteorder::NativeEndian::read_u32(bytes) as usize + 5;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        str::from_utf8(&bytes[4..])
            .map(|s| Self(s))
            .map_err(|_| VariantError::InvalidUtf8)
    }
}

pub struct Signature<'a>(pub &'a str);

// FIXME: Find a way to share code with &str implementation above
impl<'a> VariantType<'a> for Signature<'a> {
    const SIGNATURE: char = 'g';
    const SIGNATURE_STR: &'static str = "g";

    fn encode(&self) -> Vec<u8> {
        let len = self.0.len();
        let mut bytes = Vec::with_capacity(2 + len);

        bytes.push(len as u8);
        bytes.extend(self.0.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = bytes[0] as usize + 1;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        str::from_utf8(&bytes[1..])
            .map(|s| Self(s))
            .map_err(|_| VariantError::InvalidUtf8)
    }
}

pub struct Variant<'a> {
    // FIXME: This should be an `&str`
    signature: String,
    value: Cow<'a, [u8]>,
}

impl<'a> Variant<'a> {
    pub fn from_data(data: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        let value = match signature {
            // FIXME: There has to be a shorter way to do this
            u32::SIGNATURE_STR => u32::extract_slice(data)?,
            <(&str)>::SIGNATURE_STR => <(&str)>::extract_slice(data)?,
            ObjectPath::SIGNATURE_STR => ObjectPath::extract_slice(data)?,
            Signature::SIGNATURE_STR => Signature::extract_slice(data)?,
            _ => return Err(VariantError::UnsupportedType),
        };

        Ok(Self {
            value: Cow::from(value),
            signature: String::from(signature),
        })
    }

    pub fn from<T: 'a + VariantType<'a>>(value: T) -> Self
    where
        Self: 'a,
    {
        Self {
            value: Cow::from(value.encode()),
            signature: String::from(T::SIGNATURE_STR),
        }
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    pub fn get<T: 'a + VariantType<'a>>(&'a self) -> Result<T, VariantError> {
        VariantType::extract(&self.value)
    }

    // FIXME: Return a slice
    pub fn get_bytes(&self) -> &[u8] {
        &self.value
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}
