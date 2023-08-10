//! D-Bus address handling.
//!
//! Server addresses consist of a transport name followed by a colon, and then an optional,
//! comma-separated list of keys and values in the form key=value.
//!
//! See also:
//!
//! * [Server addresses] in the D-Bus specification.
//!
//! [Server addresses]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses

use crate::{Error, Result};
#[cfg(all(unix, not(target_os = "macos")))]
use nix::unistd::Uid;
use std::{collections::HashMap, convert::TryFrom, env, str::FromStr};

use std::{
    ffi::OsString,
    fmt::{Display, Formatter},
    str::from_utf8_unchecked,
};

/// A `tcp:` address family.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TcpAddressFamily {
    Ipv4,
    Ipv6,
}

/// A `tcp:` D-Bus address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TcpAddress {
    pub(crate) host: String,
    pub(crate) bind: Option<String>,
    pub(crate) port: u16,
    pub(crate) family: Option<TcpAddressFamily>,
}

impl TcpAddress {
    /// Returns the `tcp:` address `host` value.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the `tcp:` address `bind` value.
    pub fn bind(&self) -> Option<&str> {
        self.bind.as_deref()
    }

    /// Returns the `tcp:` address `port` value.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the `tcp:` address `family` value.
    pub fn family(&self) -> Option<TcpAddressFamily> {
        self.family
    }

    // Helper for FromStr
    fn from_tcp(opts: HashMap<&str, &str>) -> Result<Self> {
        let bind = None;
        if opts.contains_key("bind") {
            return Err(Error::Address("`bind` isn't yet supported".into()));
        }

        let host = opts
            .get("host")
            .ok_or_else(|| Error::Address("tcp address is missing `host`".into()))?
            .to_string();
        let port = opts
            .get("port")
            .ok_or_else(|| Error::Address("tcp address is missing `port`".into()))?;
        let port = port
            .parse::<u16>()
            .map_err(|_| Error::Address("invalid tcp `port`".into()))?;
        let family = opts
            .get("family")
            .map(|f| TcpAddressFamily::from_str(f))
            .transpose()?;

        Ok(Self {
            host,
            bind,
            port,
            family,
        })
    }

    fn write_options(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("host=")?;

        encode_percents(f, self.host.as_ref())?;

        write!(f, ",port={}", self.port)?;

        if let Some(bind) = &self.bind {
            f.write_str(",bind=")?;
            encode_percents(f, bind.as_ref())?;
        }

        if let Some(family) = &self.family {
            write!(f, ",family={family}")?;
        }

        Ok(())
    }
}

#[cfg(any(
    all(feature = "vsock", not(feature = "tokio")),
    feature = "tokio-vsock"
))]
/// A `tcp:` D-Bus address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VsockAddress {
    pub(crate) cid: u32,
    pub(crate) port: u32,
}

#[cfg(any(
    all(feature = "vsock", not(feature = "tokio")),
    feature = "tokio-vsock"
))]
impl VsockAddress {
    /// Create a new VSOCK address.
    pub fn new(cid: u32, port: u32) -> Self {
        Self { cid, port }
    }
}

/// A bus address
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Address {
    /// A path on the filesystem
    Unix(OsString),
    /// TCP address details
    Tcp(TcpAddress),
    /// TCP address details with nonce file path
    NonceTcp {
        addr: TcpAddress,
        nonce_file: Vec<u8>,
    },
    /// Autolaunch address with optional scope
    Autolaunch(Option<String>),
    /// Launchd address with a required env key
    Launchd(String),
    #[cfg(any(
        all(feature = "vsock", not(feature = "tokio")),
        feature = "tokio-vsock"
    ))]
    /// VSOCK address
    ///
    /// This variant is only available when either `vsock` or `tokio-vsock` feature is enabled. The
    /// type of `stream` is `vsock::VsockStream` with `vsock` feature and
    /// `tokio_vsock::VsockStream` with `tokio-vsock` feature.
    Vsock(VsockAddress),
    /// A listenable address using the specified path, in which a socket file with a random file
    /// name starting with 'dbus-' will be created by the server. See [UNIX domain socket address]
    /// reference documentation.
    ///
    /// This address is mostly relevant to server (typically bus broker) implementations.
    ///
    /// [UNIX domain socket address]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-unix-domain-sockets-addresses
    UnixDir(OsString),
    /// The same as UnixDir, except that on platforms with abstract sockets, the server may attempt
    /// to create an abstract socket whose name starts with this directory instead of a path-based
    /// socket.
    ///
    /// This address is mostly relevant to server (typically bus broker) implementations.
    UnixTmpDir(OsString),
}

impl Address {
    /// Get the address for session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
    /// variable. If we don't recognize the value (or it's not set) we fall back to
    /// $XDG_RUNTIME_DIR/bus
    pub fn session() -> Result<Self> {
        match env::var("DBUS_SESSION_BUS_ADDRESS") {
            Ok(val) => Self::from_str(&val),
            _ => {
                #[cfg(windows)]
                {
                    #[cfg(feature = "windows-gdbus")]
                    return Self::from_str("autolaunch:");

                    #[cfg(not(feature = "windows-gdbus"))]
                    return Self::from_str("autolaunch:scope=*user");
                }

                #[cfg(all(unix, not(target_os = "macos")))]
                {
                    let runtime_dir = env::var("XDG_RUNTIME_DIR")
                        .unwrap_or_else(|_| format!("/run/user/{}", Uid::effective()));
                    let path = format!("unix:path={runtime_dir}/bus");

                    Self::from_str(&path)
                }

                #[cfg(target_os = "macos")]
                return Self::from_str("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
            }
        }
    }

    /// Get the address for system bus respecting the DBUS_SYSTEM_BUS_ADDRESS environment
    /// variable. If we don't recognize the value (or it's not set) we fall back to
    /// /var/run/dbus/system_bus_socket
    pub fn system() -> Result<Self> {
        match env::var("DBUS_SYSTEM_BUS_ADDRESS") {
            Ok(val) => Self::from_str(&val),
            _ => {
                #[cfg(all(unix, not(target_os = "macos")))]
                return Self::from_str("unix:path=/var/run/dbus/system_bus_socket");

                #[cfg(windows)]
                return Self::from_str("autolaunch:");

                #[cfg(target_os = "macos")]
                return Self::from_str("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
            }
        }
    }

    // Helper for FromStr
    #[cfg(any(unix, not(feature = "tokio")))]
    fn from_unix(opts: HashMap<&str, &str>) -> Result<Self> {
        let path = opts.get("path");
        let abs = opts.get("abstract");
        let dir = opts.get("dir");
        let tmpdir = opts.get("tmpdir");
        let addr = match (path, abs, dir, tmpdir) {
            (Some(p), None, None, None) => Address::Unix(OsString::from(p)),
            (None, Some(p), None, None) => {
                let mut s = OsString::from("\0");
                s.push(p);
                Address::Unix(s)
            }
            (None, None, Some(p), None) => Address::UnixDir(OsString::from(p)),
            (None, None, None, Some(p)) => Address::UnixTmpDir(OsString::from(p)),
            _ => {
                return Err(Error::Address("unix: address is invalid".to_owned()));
            }
        };

        Ok(addr)
    }

    #[cfg(all(feature = "vsock", not(feature = "tokio")))]
    fn from_vsock(opts: HashMap<&str, &str>) -> Result<Self> {
        let cid = opts
            .get("cid")
            .ok_or_else(|| Error::Address("VSOCK address is missing cid=".into()))?;
        let cid = cid
            .parse::<u32>()
            .map_err(|e| Error::Address(format!("Failed to parse VSOCK cid `{}`: {}", cid, e)))?;
        let port = opts
            .get("port")
            .ok_or_else(|| Error::Address("VSOCK address is missing port=".into()))?;
        let port = port
            .parse::<u32>()
            .map_err(|e| Error::Address(format!("Failed to parse VSOCK port `{}`: {}", port, e)))?;

        Ok(Address::Vsock(VsockAddress { cid, port }))
    }
}

impl FromStr for TcpAddressFamily {
    type Err = Error;

    fn from_str(family: &str) -> Result<Self> {
        match family {
            "ipv4" => Ok(Self::Ipv4),
            "ipv6" => Ok(Self::Ipv6),
            _ => Err(Error::Address(format!(
                "invalid tcp address `family`: {family}"
            ))),
        }
    }
}

impl Display for TcpAddressFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ipv4 => write!(f, "ipv4"),
            Self::Ipv6 => write!(f, "ipv6"),
        }
    }
}

fn decode_hex(c: char) -> Result<u8> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),

        _ => Err(Error::Address(
            "invalid hexadecimal character in percent-encoded sequence".to_owned(),
        )),
    }
}

fn decode_percents(value: &str) -> Result<Vec<u8>> {
    let mut iter = value.chars();
    let mut decoded = Vec::new();

    while let Some(c) = iter.next() {
        if matches!(c, '-' | '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '/' | '.' | '\\' | '*') {
            decoded.push(c as u8)
        } else if c == '%' {
            decoded.push(
                decode_hex(iter.next().ok_or_else(|| {
                    Error::Address("incomplete percent-encoded sequence".to_owned())
                })?)?
                    << 4
                    | decode_hex(iter.next().ok_or_else(|| {
                        Error::Address("incomplete percent-encoded sequence".to_owned())
                    })?)?,
            );
        } else {
            return Err(Error::Address("Invalid character in address".to_owned()));
        }
    }

    Ok(decoded)
}

fn encode_percents(f: &mut Formatter<'_>, mut value: &[u8]) -> std::fmt::Result {
    const LOOKUP: &str = "\
%00%01%02%03%04%05%06%07%08%09%0a%0b%0c%0d%0e%0f\
%10%11%12%13%14%15%16%17%18%19%1a%1b%1c%1d%1e%1f\
%20%21%22%23%24%25%26%27%28%29%2a%2b%2c%2d%2e%2f\
%30%31%32%33%34%35%36%37%38%39%3a%3b%3c%3d%3e%3f\
%40%41%42%43%44%45%46%47%48%49%4a%4b%4c%4d%4e%4f\
%50%51%52%53%54%55%56%57%58%59%5a%5b%5c%5d%5e%5f\
%60%61%62%63%64%65%66%67%68%69%6a%6b%6c%6d%6e%6f\
%70%71%72%73%74%75%76%77%78%79%7a%7b%7c%7d%7e%7f\
%80%81%82%83%84%85%86%87%88%89%8a%8b%8c%8d%8e%8f\
%90%91%92%93%94%95%96%97%98%99%9a%9b%9c%9d%9e%9f\
%a0%a1%a2%a3%a4%a5%a6%a7%a8%a9%aa%ab%ac%ad%ae%af\
%b0%b1%b2%b3%b4%b5%b6%b7%b8%b9%ba%bb%bc%bd%be%bf\
%c0%c1%c2%c3%c4%c5%c6%c7%c8%c9%ca%cb%cc%cd%ce%cf\
%d0%d1%d2%d3%d4%d5%d6%d7%d8%d9%da%db%dc%dd%de%df\
%e0%e1%e2%e3%e4%e5%e6%e7%e8%e9%ea%eb%ec%ed%ee%ef\
%f0%f1%f2%f3%f4%f5%f6%f7%f8%f9%fa%fb%fc%fd%fe%ff";

    loop {
        let pos = value.iter().position(
            |c| !matches!(c, b'-' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' | b'/' | b'.' | b'\\' | b'*'),
        );

        if let Some(pos) = pos {
            // SAFETY: The above `position()` call made sure that only ASCII chars are in the string
            // up to `pos`
            f.write_str(unsafe { from_utf8_unchecked(&value[..pos]) })?;

            let c = value[pos];
            value = &value[pos + 1..];

            let pos = c as usize * 3;
            f.write_str(&LOOKUP[pos..pos + 3])?;
        } else {
            // SAFETY: The above `position()` call made sure that only ASCII chars are in the rest
            // of the string
            f.write_str(unsafe { from_utf8_unchecked(value) })?;
            return Ok(());
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn fmt_unix_path(
            f: &mut Formatter<'_>,
            path: &OsString,
            _is_abstract: bool,
        ) -> std::fmt::Result {
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStrExt;

                let bytes = if _is_abstract {
                    &path.as_bytes()[1..]
                } else {
                    path.as_bytes()
                };
                encode_percents(f, bytes)?;
            }

            #[cfg(windows)]
            write!(f, "{}", path.to_str().ok_or(std::fmt::Error)?)?;

            Ok(())
        }

        match self {
            Self::Tcp(addr) => {
                f.write_str("tcp:")?;
                addr.write_options(f)?;
            }

            Self::NonceTcp { addr, nonce_file } => {
                f.write_str("nonce-tcp:noncefile=")?;
                encode_percents(f, nonce_file)?;
                f.write_str(",")?;
                addr.write_options(f)?;
            }

            Self::Unix(path) => {
                let is_abstract = {
                    #[cfg(unix)]
                    {
                        use std::os::unix::ffi::OsStrExt;

                        path.as_bytes().first() == Some(&b'\0')
                    }
                    #[cfg(not(unix))]
                    false
                };

                if is_abstract {
                    f.write_str("unix:abstract=")?;
                } else {
                    f.write_str("unix:path=")?;
                }

                fmt_unix_path(f, path, is_abstract)?;
            }

            Self::UnixDir(path) => {
                f.write_str("unix:dir=")?;
                fmt_unix_path(f, path, false)?;
            }

            Self::UnixTmpDir(path) => {
                f.write_str("unix:tmpdir=")?;
                fmt_unix_path(f, path, false)?;
            }

            #[cfg(any(
                all(feature = "vsock", not(feature = "tokio")),
                feature = "tokio-vsock"
            ))]
            Self::Vsock(addr) => {
                write!(f, "vsock:cid={},port={}", addr.cid, addr.port)?;
            }

            Self::Autolaunch(scope) => {
                write!(f, "autolaunch:")?;
                if let Some(scope) = scope {
                    write!(f, "scope={scope}")?;
                }
            }

            Self::Launchd(env) => {
                write!(f, "launchd:env={}", env)?;
            }
        }

        Ok(())
    }
}

impl FromStr for Address {
    type Err = Error;

    /// Parse a D-BUS address and return its path if we recognize it
    fn from_str(address: &str) -> Result<Self> {
        let col = address
            .find(':')
            .ok_or_else(|| Error::Address("address has no colon".to_owned()))?;
        let transport = &address[..col];
        let mut options = HashMap::new();

        if address.len() > col + 1 {
            for kv in address[col + 1..].split(',') {
                let (k, v) = match kv.find('=') {
                    Some(eq) => (&kv[..eq], &kv[eq + 1..]),
                    None => {
                        return Err(Error::Address(
                            "missing = when parsing key/value".to_owned(),
                        ))
                    }
                };
                if options.insert(k, v).is_some() {
                    return Err(Error::Address(format!(
                        "Key `{k}` specified multiple times"
                    )));
                }
            }
        }

        match transport {
            #[cfg(any(unix, not(feature = "tokio")))]
            "unix" => Self::from_unix(options),
            "tcp" => TcpAddress::from_tcp(options).map(Self::Tcp),

            "nonce-tcp" => Ok(Self::NonceTcp {
                nonce_file: decode_percents(
                    options
                        .get("noncefile")
                        .ok_or_else(|| Error::Address("missing nonce file parameter".into()))?,
                )?,
                addr: TcpAddress::from_tcp(options)?,
            }),
            #[cfg(all(feature = "vsock", not(feature = "tokio")))]
            "vsock" => Self::from_vsock(options),
            "autolaunch" => Ok(Self::Autolaunch(
                options
                    .get("scope")
                    .map(|scope| -> Result<_> {
                        String::from_utf8(decode_percents(scope)?).map_err(|_| {
                            Error::Address("autolaunch scope is not valid UTF-8".to_owned())
                        })
                    })
                    .transpose()?,
            )),
            "launchd" => Ok(Self::Launchd(
                options
                    .get("env")
                    .ok_or_else(|| Error::Address("missing env key".into()))?
                    .to_string(),
            )),

            _ => Err(Error::Address(format!(
                "unsupported transport '{transport}'"
            ))),
        }
    }
}

impl TryFrom<&str> for Address {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, TcpAddress, TcpAddressFamily};
    use crate::Error;
    use std::str::FromStr;
    use test_log::test;

    #[test]
    fn parse_dbus_addresses() {
        match Address::from_str("").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "address has no colon"),
            _ => panic!(),
        }
        match Address::from_str("foo").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "address has no colon"),
            _ => panic!(),
        }
        match Address::from_str("foo:opt").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "missing = when parsing key/value"),
            _ => panic!(),
        }
        match Address::from_str("foo:opt=1,opt=2").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "Key `opt` specified multiple times"),
            _ => panic!(),
        }

        match Address::from_str("tcp:host=localhost").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "tcp address is missing `port`"),
            _ => panic!(),
        }
        match Address::from_str("tcp:host=localhost,port=32f").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "invalid tcp `port`"),
            _ => panic!(),
        }
        match Address::from_str("tcp:host=localhost,port=123,family=ipv7").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "invalid tcp address `family`: ipv7"),
            _ => panic!(),
        }
        match Address::from_str("unix:foo=blah").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "unix: address is invalid"),
            _ => panic!(),
        }
        match Address::from_str("unix:path=/tmp,abstract=foo").unwrap_err() {
            Error::Address(e) => {
                assert_eq!(e, "unix: address is invalid")
            }
            _ => panic!(),
        }
        assert_eq!(
            Address::Unix("/tmp/dbus-foo".into()),
            Address::from_str("unix:path=/tmp/dbus-foo").unwrap()
        );
        assert_eq!(
            Address::Unix("\0/tmp/dbus-foo".into()),
            Address::from_str("unix:abstract=/tmp/dbus-foo").unwrap()
        );
        assert_eq!(
            Address::Unix("/tmp/dbus-foo".into()),
            Address::from_str("unix:path=/tmp/dbus-foo,guid=123").unwrap()
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: None
            }),
            Address::from_str("tcp:host=localhost,port=4142").unwrap()
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: Some(TcpAddressFamily::Ipv4)
            }),
            Address::from_str("tcp:host=localhost,port=4142,family=ipv4").unwrap()
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: Some(TcpAddressFamily::Ipv6)
            }),
            Address::from_str("tcp:host=localhost,port=4142,family=ipv6").unwrap()
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: Some(TcpAddressFamily::Ipv6)
            }),
            Address::from_str("tcp:host=localhost,port=4142,family=ipv6,noncefile=/a/file/path")
                .unwrap()
        );
        assert_eq!(
            Address::NonceTcp {
                addr: TcpAddress {
                    host: "localhost".into(),
                    port: 4142,
                    bind: None,
                    family: Some(TcpAddressFamily::Ipv6),
                },
                nonce_file: b"/a/file/path to file 1234".to_vec()
            },
            Address::from_str(
                "nonce-tcp:host=localhost,port=4142,family=ipv6,noncefile=/a/file/path%20to%20file%201234"
            )
            .unwrap()
        );
        assert_eq!(
            Address::Autolaunch(None),
            Address::from_str("autolaunch:").unwrap()
        );
        assert_eq!(
            Address::Autolaunch(Some("*my_cool_scope*".to_owned())),
            Address::from_str("autolaunch:scope=*my_cool_scope*").unwrap()
        );
        assert_eq!(
            Address::Launchd("my_cool_env_key".to_owned()),
            Address::from_str("launchd:env=my_cool_env_key").unwrap()
        );

        #[cfg(all(feature = "vsock", not(feature = "tokio")))]
        assert_eq!(
            Address::Vsock(crate::VsockAddress {
                cid: 98,
                port: 2934
            }),
            Address::from_str("vsock:cid=98,port=2934,guid=123").unwrap()
        );
        assert_eq!(
            Address::UnixDir("/some/dir".into()),
            Address::from_str("unix:dir=/some/dir").unwrap()
        );
        assert_eq!(
            Address::UnixTmpDir("/some/dir".into()),
            Address::from_str("unix:tmpdir=/some/dir").unwrap()
        );
    }

    #[test]
    fn stringify_dbus_addresses() {
        assert_eq!(
            Address::Unix("/tmp/dbus-foo".into()).to_string(),
            "unix:path=/tmp/dbus-foo"
        );
        assert_eq!(
            Address::UnixDir("/tmp/dbus-foo".into()).to_string(),
            "unix:dir=/tmp/dbus-foo"
        );
        assert_eq!(
            Address::UnixTmpDir("/tmp/dbus-foo".into()).to_string(),
            "unix:tmpdir=/tmp/dbus-foo"
        );
        // FIXME: figure out how to handle abstract on Windows
        #[cfg(unix)]
        assert_eq!(
            Address::Unix("\0/tmp/dbus-foo".into()).to_string(),
            "unix:abstract=/tmp/dbus-foo"
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: None
            })
            .to_string(),
            "tcp:host=localhost,port=4142"
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: Some(TcpAddressFamily::Ipv4)
            })
            .to_string(),
            "tcp:host=localhost,port=4142,family=ipv4"
        );
        assert_eq!(
            Address::Tcp(TcpAddress {
                host: "localhost".into(),
                port: 4142,
                bind: None,
                family: Some(TcpAddressFamily::Ipv6)
            })
            .to_string(),
            "tcp:host=localhost,port=4142,family=ipv6"
        );
        assert_eq!(
            Address::NonceTcp {
                addr: TcpAddress {
                    host: "localhost".into(),
                    port: 4142,
                    bind: None,
                    family: Some(TcpAddressFamily::Ipv6),
                },
                nonce_file: b"/a/file/path to file 1234".to_vec()
            }
            .to_string(),
            "nonce-tcp:noncefile=/a/file/path%20to%20file%201234,host=localhost,port=4142,family=ipv6"
        );
        assert_eq!(Address::Autolaunch(None).to_string(), "autolaunch:");
        assert_eq!(
            Address::Autolaunch(Some("*my_cool_scope*".to_owned())).to_string(),
            "autolaunch:scope=*my_cool_scope*"
        );
        assert_eq!(
            Address::Launchd("my_cool_key".to_owned()).to_string(),
            "launchd:env=my_cool_key"
        );

        #[cfg(all(feature = "vsock", not(feature = "tokio")))]
        assert_eq!(
            Address::Vsock(crate::VsockAddress {
                cid: 98,
                port: 2934
            })
            .to_string(),
            "vsock:cid=98,port=2934", // no support for guid= yet..
        );
    }
}
