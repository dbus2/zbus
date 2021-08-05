use crate::introspect::{IntrospectionHandle, IntrospectionInfo, PrimaryType};
use crate::signature_parser::SignatureParser;
use crate::{Basic, Fd, ObjectPath, Signature};

#[cfg(feature = "gvariant")]
use crate::utils::MAYBE_SIGNATURE_CHAR;
use crate::utils::{
    ARRAY_SIGNATURE_CHAR, DICT_ENTRY_SIG_START_CHAR, SERIALIZE_DICT_SIG_END_STR,
    SERIALIZE_DICT_SIG_START_CHAR, SERIALIZE_DICT_SIG_START_STR, STRUCT_SIG_END_STR,
    STRUCT_SIG_START_CHAR, STRUCT_SIG_START_STR, VARIANT_SIGNATURE_CHAR,
};

// All signatures can be converted into `Signature<'static>`
impl IntrospectionInfo for SignatureParser<'static> {
    fn copy(&self) -> IntrospectionHandle {
        Box::new(self.clone())
    }

    fn name(&self) -> Option<&'static str> {
        None
    }

    fn primary_type(&self) -> PrimaryType {
        match self.next_char() {
            bool::SIGNATURE_CHAR => PrimaryType::Boolean,
            i16::SIGNATURE_CHAR => PrimaryType::Int16,
            u16::SIGNATURE_CHAR => PrimaryType::UInt16,
            i32::SIGNATURE_CHAR => PrimaryType::Int32,
            u32::SIGNATURE_CHAR => PrimaryType::UInt32,
            i64::SIGNATURE_CHAR => PrimaryType::Int64,
            u64::SIGNATURE_CHAR => PrimaryType::UInt64,
            f64::SIGNATURE_CHAR => PrimaryType::Double,
            <&str>::SIGNATURE_CHAR => PrimaryType::String,
            ObjectPath::SIGNATURE_CHAR => PrimaryType::ObjectPath,
            Signature::SIGNATURE_CHAR => PrimaryType::Signature,
            Fd::SIGNATURE_CHAR => PrimaryType::Fd,
            VARIANT_SIGNATURE_CHAR => PrimaryType::Variant,
            ARRAY_SIGNATURE_CHAR => PrimaryType::Array,
            STRUCT_SIG_START_CHAR => PrimaryType::Struct,
            DICT_ENTRY_SIG_START_CHAR => PrimaryType::Struct,
            SERIALIZE_DICT_SIG_START_CHAR => PrimaryType::Struct,

            #[cfg(feature = "gvariant")]
            MAYBE_SIGNATURE_CHAR => PrimaryType::Option,

            c => PrimaryType::InvalidSignatureCharacter(c),
        }
    }

    fn member_by_index(&self, which: usize) -> Option<(&'static str, IntrospectionHandle)> {
        match (self.next_char(), which) {
            (u8::SIGNATURE_CHAR
             | bool::SIGNATURE_CHAR
             | i16::SIGNATURE_CHAR
             | u16::SIGNATURE_CHAR
             | i32::SIGNATURE_CHAR
             | u32::SIGNATURE_CHAR
             | i64::SIGNATURE_CHAR
             | u64::SIGNATURE_CHAR
             | f64::SIGNATURE_CHAR
             | <&str>::SIGNATURE_CHAR
             | ObjectPath::SIGNATURE_CHAR
             | Signature::SIGNATURE_CHAR
             | Fd::SIGNATURE_CHAR
             | VARIANT_SIGNATURE_CHAR, _) => None,
            (ARRAY_SIGNATURE_CHAR, 0) => {
                let mut new = self.clone();
                new.skip_char().ok()?;
                Some(("item", Box::new(new)))
            },
            (STRUCT_SIG_START_CHAR | SERIALIZE_DICT_SIG_START_CHAR | DICT_ENTRY_SIG_START_CHAR, n) => {
                let mut new = self.clone();
                new.skip_char().ok()?;
                for i in 0..n  {
                    new.parse_next_signature().ok()?;
                }
                Some(("", Box::new(new)))
            },
            #[cfg(feature = "gvariant")]
            (MAYBE_SIGNATURE_CHAR, 0) => {
                let mut new = self.clone();
                new.skip_char().ok()?;
                Some(("value", Box::new(new)))
            },
            _  => None,
        }
    }
}
