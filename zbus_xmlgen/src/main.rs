#![deny(rust_2018_idioms)]

use std::{
    env::args,
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::{ChildStdin, Command, Stdio},
    result::Result,
};

use zbus::{
    blocking::{connection, fdo::IntrospectableProxy, Connection},
    names::BusName,
};
use zbus_xml::{Interface, Node};

use zbus_xmlgen::GenTrait;
use zvariant::ObjectPath;

fn usage() {
    eprintln!(
        r#"Usage:
  zbus-xmlgen <interface.xml> [-o|--output <file_path>]
  zbus-xmlgen --system|--session <service> <object_path> [-o|--output <file_path>]
  zbus-xmlgen --address <address> <service> <object_path> [-o|--output <file_path>]
"#
    );
}

enum OutputTarget {
    SingleFile(File),
    Stdout,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_src;

    let proxy = |conn: Connection, service, path| -> IntrospectableProxy<'_> {
        IntrospectableProxy::builder(&conn)
            .destination(service)
            .expect("invalid destination")
            .path(path)
            .expect("invalid path")
            .build()
            .unwrap()
    };

    let (node, service, path) = match args().nth(1) {
        Some(bus) if bus == "--system" || bus == "--session" => {
            let connection = if bus == "--system" {
                Connection::system()?
            } else {
                Connection::session()?
            };
            let service: BusName<'_> = args()
                .nth(2)
                .expect("Missing param for service")
                .try_into()?;
            let path: ObjectPath<'_> = args()
                .nth(3)
                .expect("Missing param for object path")
                .try_into()?;

            input_src = format!(
                "Interface '{}' from service '{}' on {} bus",
                path,
                service,
                bus.trim_start_matches("--")
            );

            let xml = proxy(connection, service.clone(), path.clone()).introspect()?;
            (
                Node::from_reader(xml.as_bytes())?,
                Some(service),
                Some(path),
            )
        }
        Some(address) if address == "--address" => {
            let address = args().nth(2).expect("Missing param for address path");
            let service: BusName<'_> = args()
                .nth(3)
                .expect("Missing param for service")
                .try_into()?;
            let path: ObjectPath<'_> = args()
                .nth(4)
                .expect("Missing param for object path")
                .try_into()?;

            let connection = connection::Builder::address(&*address)?.build()?;

            input_src = format!("Interface '{path}' from service '{service}'");

            let xml = proxy(connection, service.clone(), path.clone()).introspect()?;
            (
                Node::from_reader(xml.as_bytes())?,
                Some(service),
                Some(path),
            )
        }
        Some(help) if help == "--help" || help == "-h" => {
            usage();
            return Ok(());
        }
        Some(path) => {
            input_src = Path::new(&path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let f = File::open(path)?;
            (Node::from_reader(f)?, None, None)
        }
        None => {
            usage();
            return Ok(());
        }
    };

    let fdo_iface_prefix = "org.freedesktop.DBus";
    let (fdo_standard_ifaces, needed_ifaces): (Vec<&Interface<'_>>, Vec<&Interface<'_>>) = node
        .interfaces()
        .iter()
        .partition(|&i| i.name().starts_with(fdo_iface_prefix));

    let output_arg = args()
        .position(|arg| arg == "-o" || arg == "--output")
        .map(|pos| pos + 1)
        .and_then(|pos| args().nth(pos));
    let mut output_target = match output_arg.as_deref() {
        Some("-") => OutputTarget::Stdout,
        Some(path) => {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(path)
                .expect("Failed to open file");
            OutputTarget::SingleFile(file)
        }
        _ => OutputTarget::Stdout,
    };

    for interface in needed_ifaces {
        let output = write_interfaces(
            &[&interface],
            &fdo_standard_ifaces,
            service.clone(),
            path.clone(),
            &input_src,
        )?;
        match output_target {
            OutputTarget::SingleFile(ref mut file) => {
                file.write_all(output.as_bytes())?;
            }
            OutputTarget::Stdout => println!("{:?}", output),
        };
    }

    Ok(())
}

fn write_interfaces(
    interfaces: &[&Interface<'_>],
    standard_interfaces: &[&Interface<'_>],
    service: Option<BusName<'_>>,
    path: Option<ObjectPath<'_>>,
    input_src: &str,
) -> Result<String, Box<dyn Error>> {
    let mut process = match Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn rustfmt: {}", why),
        Ok(process) => process,
    };
    let rustfmt_stdin = process.stdin.as_mut().unwrap();
    let mut rustfmt_stdout = process.stdout.take().unwrap();

    write_doc_header(rustfmt_stdin, interfaces, standard_interfaces, input_src)?;

    for interface in interfaces {
        writeln!(rustfmt_stdin)?;
        let gen = GenTrait {
            interface,
            service: service.as_ref(),
            path: path.as_ref(),
        }
        .to_string();
        rustfmt_stdin.write_all(gen.as_bytes())?;
    }

    process.wait()?;
    let mut buffer = String::new();
    rustfmt_stdout.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Write a doc header, listing the included Interfaces and how the
/// code was generated.
fn write_doc_header(
    rustfmt_stdin: &mut ChildStdin,
    interfaces: &[&Interface<'_>],
    standard_interfaces: &[&Interface<'_>],
    input_src: &str,
) -> std::io::Result<()> {
    if let Some((first_iface, following_ifaces)) = interfaces.split_first() {
        if following_ifaces.is_empty() {
            writeln!(
                rustfmt_stdin,
                "//! # DBus interface proxy for: `{}`",
                first_iface.name()
            )?;
        } else {
            write!(
                rustfmt_stdin,
                "//! # DBus interface proxies for: `{}`",
                first_iface.name()
            )?;
            for iface in following_ifaces {
                write!(rustfmt_stdin, ", `{}`", iface.name())?;
            }
            writeln!(rustfmt_stdin)?;
        }
    }

    write!(
        rustfmt_stdin,
        "//!
         //! This code was generated by `{}` `{}` from DBus introspection data.
         //! Source: `{}`.
         //!
         //! You may prefer to adapt it, instead of using it verbatim.
         //!
         //! More information can be found in the
         //! [Writing a client proxy](https://dbus2.github.io/zbus/client.html)
         //! section of the zbus documentation.
         //!
        ",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        input_src,
    )?;

    if !standard_interfaces.is_empty() {
        write!(rustfmt_stdin,
            "//! This DBus object implements
             //! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
             //! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
             //!
            ")?;
        for iface in standard_interfaces {
            let idx = iface.name().rfind('.').unwrap() + 1;
            let name = &iface.name()[idx..];
            writeln!(rustfmt_stdin, "//! * [`zbus::fdo::{name}Proxy`]")?;
        }
        write!(
            rustfmt_stdin,
            "//!
             //! …consequently `{}` did not generate code for the above interfaces.
            ",
            env!("CARGO_BIN_NAME")
        )?;
    }

    write!(
        rustfmt_stdin,
        "
        use zbus::dbus_proxy;
        "
    )?;

    Ok(())
}
