use std::{borrow::Cow, path::{Path, PathBuf}};
use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

use crate::Type;

/// A file name represented as a nul-terminated byte array.
///
/// While `zvariant::Type` and `serde::{Serialize, Deserialize}`, are implemented for [`Path`] and [`PathBuf`], unfortunately `serde` serializes them as UTF-8 strings. This is not the desired behavior in most cases since file paths are not guaranteed to contain only UTF-8 characters.
/// To solve this problem, this type is provided which encodes the underlying file path as a null-terminated byte array. Encoding as byte array is also more efficient.
///
///
/// # Exmples
/// ```
/// use zvariant::FilePath;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("/hello/world");
/// let path_buf = PathBuf::from(path);
///
/// let p1 = FilePath::from(path);
/// let p2 = FilePath::from(path_buf);
/// let p3 = FilePath::from("/hello/world");
///
/// assert_eq!(p1, p2);
/// assert_eq!(p2, p3);
/// ```
#[derive(Type, Debug, Default, PartialEq, Eq)]
#[zvariant(signature = "ay")]
pub struct FilePath<'f>(Cow<'f, Path>);


impl<'f> From<&'f Path> for FilePath<'f> {
    fn from(value: &'f Path) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'f> From<PathBuf> for FilePath<'f> {
    fn from(value: PathBuf) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'f> From<&'f str> for FilePath<'f> {
    fn from(value: &str) -> Self {
        Self(Cow::Owned(PathBuf::from(value)))
    }
}

impl<'de> Deserialize<'de> for FilePath<'de> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct FilePathVisitor;
        impl<'de> Visitor<'de> for FilePathVisitor {
            type Value = FilePath<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FilePath::from(PathBuf::from(
                            // SAFETY: File path do not necessarily contain only a sequence
                            // of UTF-8 characters, thus it's safe to assume that the path *contains*
                            // a non-vaalid UTF-8 characters, hence the `unsafe` block.
                            unsafe{
                                String::from_utf8_unchecked(v.to_vec())
                            }
                )))
            }
        }
        let visitor = FilePathVisitor;
        deserializer.deserialize_bytes(visitor)
    }
}

impl<'f> Serialize for FilePath<'f> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_bytes(&self.0.as_os_str().as_encoded_bytes())
    }
}


impl<'f> AsRef<FilePath<'f>> for FilePath<'f> {
    fn as_ref(& self) -> &FilePath<'f> {
        &self
    }
}

impl<'f> Into<PathBuf> for FilePath<'f> {
    fn into(self) -> PathBuf {
        PathBuf::from(self.0.into_owned())
    }
}

#[cfg(test)]
mod file_path {
    use crate::zvariant::Signature;
    use std::path::{Path, PathBuf};
    use super::*;

    #[test]
    fn filepath_from() {
        let path = Path::new("/hello/world");
        let path_buf = PathBuf::from(path);

        let p1 = FilePath::from(path);
        let p2 = FilePath::from(path_buf);
        let p3 = FilePath::from("/hello/world");

        assert_eq!(p1, p2);
        assert_eq!(p2, p3)
    }

    #[test]
    fn filepath_signature() {
        assert_eq!(
            &Signature::static_array(&Signature::U8),
            FilePath::SIGNATURE
        );
    }

    #[test]
    fn into_test() {
        let first = PathBuf::from("/hello/world");
        let p = FilePath::from(first.clone());
        let second: PathBuf = p.into();
        assert_eq!(first, second);
    }
}
