use crate::{static_str_type, Signature, Type};

static_str_type!(std::path::Path);
static_str_type!(std::path::PathBuf);

#[cfg(feature = "camino")]
static_str_type!(camino::Utf8Path);
#[cfg(feature = "camino")]
static_str_type!(camino::Utf8PathBuf);
