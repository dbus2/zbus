use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use crate::{Error, Result, Type};

/// File name represented as a nul-terminated byte array.
///
/// While `zbus::Type` and `serde::{Serialize, Deserialize}`, are implemented for [`Path`] and
/// [`PathBuf`], unfortunately `serde` serializes them as UTF-8 strings and that limits the number
/// of possible characters to use on a file path. This is not the desired behavior since file paths
/// are not guaranteed to contain only UTF-8 characters.
///
/// To solve this problem, this type is provided which encodes the underlying file path as a
/// null-terminated byte array.
///
/// # Converting back to standard types
///
/// The conversions to [`OsString`], [`PathBuf`], [`&OsStr`](OsStr) and [`&Path`](Path) are
/// [`TryFrom`] implementations. On unix, where paths are byte strings just like this type, they
/// preserve the bytes exactly and never fail. On other platforms no such lossless mapping exists,
/// so they succeed only for valid UTF-8 and return [`Error::Utf8`] otherwise.
/// [`as_c_str`](Self::as_c_str) hands back the bytes as they are on every platform.
///
/// # Examples:
///
/// ```
/// use zbus::FilePath;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("/hello/world\0");
/// let path_buf = PathBuf::from(path);
///
/// let p1 = FilePath::from(path);
/// let p2 = FilePath::from(path_buf);
/// let p3 = FilePath::from("/hello/world");
///
/// assert_eq!(p1, p2);
/// assert_eq!(p2, p3);
/// ```
#[derive(Type, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone, Ord, PartialOrd)]
#[zbus(signature = "ay")]
pub struct FilePath<'f>(Cow<'f, CStr>);

impl<'f> FilePath<'f> {
    /// Create a new `FilePath` from a nul-terminated byte string, either borrowed or owned.
    pub fn new(cow: Cow<'f, CStr>) -> Self {
        Self(cow)
    }

    /// The path as a nul-terminated byte string.
    ///
    /// This is the lossless view of the path on every platform; the conversions to the standard
    /// path types are lossless on unix only.
    pub fn as_c_str(&self) -> &CStr {
        &self.0
    }

    /// Returns a lossy UTF-8 representation of the file path.
    ///
    /// Invalid UTF-8 sequences are replaced with `U+FFFD REPLACEMENT CHARACTER`.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.0.to_string_lossy()
    }
}

impl From<CString> for FilePath<'_> {
    fn from(value: CString) -> Self {
        FilePath(Cow::Owned(value))
    }
}

impl<'f> From<&'f CString> for FilePath<'f> {
    fn from(value: &'f CString) -> Self {
        FilePath(Cow::Borrowed(value.as_c_str()))
    }
}

impl<'f> From<&'f OsStr> for FilePath<'f> {
    fn from(value: &'f OsStr) -> FilePath<'f> {
        FilePath(bytes_with_null(value.as_encoded_bytes()))
    }
}

impl<'f> From<&'f OsString> for FilePath<'f> {
    fn from(value: &'f OsString) -> FilePath<'f> {
        FilePath(bytes_with_null(value.as_encoded_bytes()))
    }
}

impl From<OsString> for FilePath<'_> {
    fn from(value: OsString) -> Self {
        FilePath(vec_to_cstr(value.as_encoded_bytes().to_vec()))
    }
}

impl<'f> From<&'f PathBuf> for FilePath<'f> {
    fn from(value: &'f PathBuf) -> FilePath<'f> {
        FilePath::from(value.as_os_str())
    }
}

impl From<PathBuf> for FilePath<'_> {
    fn from(value: PathBuf) -> FilePath<'static> {
        FilePath::from(OsString::from(value))
    }
}

impl<'f> From<&'f Path> for FilePath<'f> {
    fn from(value: &'f Path) -> Self {
        Self::from(value.as_os_str())
    }
}

impl<'f> From<&'f CStr> for FilePath<'f> {
    fn from(value: &'f CStr) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'f> From<&'f str> for FilePath<'f> {
    fn from(value: &'f str) -> Self {
        Self::from(OsStr::new(value))
    }
}

impl<'f> AsRef<FilePath<'f>> for FilePath<'f> {
    fn as_ref(&self) -> &FilePath<'f> {
        self
    }
}

impl AsRef<CStr> for FilePath<'_> {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}
impl<'f> TryFrom<&'f FilePath<'f>> for &'f OsStr {
    type Error = Error;

    fn try_from(value: &'f FilePath<'f>) -> Result<Self> {
        #[cfg(unix)]
        {
            Ok(OsStr::from_bytes(value.0.to_bytes()))
        }
        #[cfg(not(unix))]
        {
            value.0.to_str().map(OsStr::new).map_err(Error::Utf8)
        }
    }
}

impl<'f> TryFrom<&'f FilePath<'f>> for &'f Path {
    type Error = Error;

    fn try_from(value: &'f FilePath<'f>) -> Result<Self> {
        <&OsStr>::try_from(value).map(Path::new)
    }
}

impl TryFrom<FilePath<'_>> for OsString {
    type Error = Error;

    fn try_from(value: FilePath<'_>) -> Result<Self> {
        #[cfg(unix)]
        {
            Ok(OsString::from_vec(value.0.into_owned().into_bytes()))
        }
        #[cfg(not(unix))]
        {
            <&OsStr>::try_from(&value).map(OsString::from)
        }
    }
}

impl TryFrom<FilePath<'_>> for PathBuf {
    type Error = Error;

    fn try_from(value: FilePath<'_>) -> Result<Self> {
        OsString::try_from(value).map(PathBuf::from)
    }
}

/// Converts a `Vec<u8>` into a null-terminated `CStr`.
///
/// Truncates the vector at the first null byte, if present. If no null byte exists, appends one to
/// ensure proper termination.
///
/// # Returns
///
/// A `Cow<'_, CStr>` containing a *guaranteed* null-terminated string.
fn vec_to_cstr(mut bytes: Vec<u8>) -> Cow<'static, CStr> {
    if let Some(pos) = bytes.iter().position(|&b| b == 0) {
        bytes.truncate(pos + 1);
    } else {
        bytes.push(0);
    }
    // unwrap is fine here since we append the null byte.
    Cow::Owned(CString::from_vec_with_nul(bytes).unwrap())
}

/// Converts a byte slice into a null-terminated [CStr].
///
/// Returns a borrowed [CStr] if the slice already contains a null byte; otherwise, returns an
/// owned [CStr] with a null byte appended.
///
/// # Returns
///
/// A [Cow<'_, CStr>] containing a *guaranteed* null-terminated string.
fn bytes_with_null(bytes: &[u8]) -> Cow<'_, CStr> {
    if let Ok(cstr) = CStr::from_bytes_until_nul(bytes) {
        return Cow::Borrowed(cstr);
    }
    // unwrap is fine, as we handled the null termination case above.
    Cow::Owned(CString::new(bytes).unwrap())
}

#[cfg(test)]
mod file_path_test {
    use super::*;
    use crate::Signature;
    use std::path::{Path, PathBuf};

    #[test]
    fn from_test() {
        let path = Path::new("/hello/world");
        let path_buf = PathBuf::from(path);
        let osstr = OsStr::new("/hello/world");
        let os_string = OsString::from("/hello/world");
        let cstr = CStr::from_bytes_until_nul("/hello/world\0".as_bytes()).unwrap_or_default();
        let cstring = CString::new("/hello/world").unwrap_or_default();

        let p1 = FilePath::from(path);
        let p2 = FilePath::from(path_buf);
        let p3 = FilePath::from(osstr);
        let p4 = FilePath::from(os_string);
        let p5 = FilePath::from(cstr);
        let p6 = FilePath::from(cstring);
        let p7 = FilePath::from("/hello/world");

        assert_eq!(p1, p2);
        assert_eq!(p2, p3);
        assert_eq!(p3, p4);
        assert_eq!(p4, p5);
        assert_eq!(p5, p6);
        assert_eq!(p5, p7);
    }

    /// The `CStr` view is lossless everywhere, UTF-8 or not.
    #[test]
    fn as_c_str_test() {
        let bytes = c"/hello/\xff\xfe/world";
        let file_path = FilePath::from(bytes);

        assert_eq!(file_path.as_c_str(), bytes);
        assert_eq!(<FilePath<'_> as AsRef<CStr>>::as_ref(&file_path), bytes);
    }

    #[test]
    fn filepath_signature() {
        assert_eq!(
            &Signature::static_array(&Signature::U8),
            FilePath::SIGNATURE
        );
    }

    #[test]
    fn try_into_test() {
        let path = Path::new("/hello/world");
        let file_path = FilePath::from(path);

        assert_eq!(<&OsStr>::try_from(&file_path).unwrap(), path.as_os_str());
        assert_eq!(<&Path>::try_from(&file_path).unwrap(), path);
        assert_eq!(
            OsString::try_from(file_path.clone()).unwrap(),
            path.as_os_str()
        );
        assert_eq!(PathBuf::try_from(file_path).unwrap(), path);
    }

    /// Arbitrary bytes are exactly what the unix conversions exist for, so nothing may be lost or
    /// replaced along the way.
    #[cfg(unix)]
    #[test]
    fn non_utf8_try_into_test() {
        let os_str = OsStr::from_bytes(b"/hello/\xff\xfe/world");
        let file_path = FilePath::from(os_str);

        assert_eq!(<&OsStr>::try_from(&file_path).unwrap(), os_str);
        assert_eq!(<&Path>::try_from(&file_path).unwrap(), Path::new(os_str));
        assert_eq!(OsString::try_from(file_path.clone()).unwrap(), os_str);
        assert_eq!(PathBuf::try_from(file_path).unwrap(), Path::new(os_str));
    }

    /// Off unix only UTF-8 gets through.
    #[cfg(not(unix))]
    #[test]
    fn non_utf8_try_into_test() {
        let file_path = FilePath::from(c"/hello/\xff\xfe/world");

        assert!(matches!(
            <&OsStr>::try_from(&file_path),
            Err(Error::Utf8(_))
        ));
        assert!(matches!(<&Path>::try_from(&file_path), Err(Error::Utf8(_))));
        assert!(matches!(
            OsString::try_from(file_path.clone()),
            Err(Error::Utf8(_))
        ));
        assert!(matches!(PathBuf::try_from(file_path), Err(Error::Utf8(_))));
    }

    #[test]
    fn vec_nul_termination() {
        fn call_vec_to_cstr(v: Vec<u8>) -> Cow<'static, CStr> {
            vec_to_cstr(v)
        }
        let v1 = vec![];
        let v2 = vec![0x0];
        let v3 = vec![0x1, 0x2, 0x0];
        let v4 = vec![0x0, 0x0];
        let v5 = vec![0x1, 0x0, 0x2, 0x0];

        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x0]).unwrap()),
            call_vec_to_cstr(v1)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x0]).unwrap()),
            call_vec_to_cstr(v2)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x1, 0x2, 0x0]).unwrap()),
            call_vec_to_cstr(v3)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x0]).unwrap()),
            call_vec_to_cstr(v4)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x1, 0x0]).unwrap()),
            call_vec_to_cstr(v5)
        );
    }
}
