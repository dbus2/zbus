#![deny(rust_2018_idioms)]

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    result::Result,
};

use clap::Parser;
use zbus::{
    blocking::{connection, fdo::IntrospectableProxy, Connection},
    names::BusName,
};
use zbus_xml::{Interface, Node};

use zbus_xmlgen::GenTrait;
use zvariant::ObjectPath;

mod cli;

enum OutputTarget {
    SingleFile(File),
    Stdout,
    MultipleFiles,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let DBusInfo(node, service, path, input_src) = match args.command {
        cli::Command::System {
            service,
            object_path,
        } => DBusInfo::new(Connection::system()?, service, object_path)?,
        cli::Command::Session {
            service,
            object_path,
        } => DBusInfo::new(Connection::session()?, service, object_path)?,
        cli::Command::Address {
            address,
            service,
            object_path,
        } => DBusInfo::new(
            connection::Builder::address(&*address)?.build()?,
            service,
            object_path,
        )?,
        cli::Command::File { path } => {
            let input_src = path.file_name().unwrap().to_string_lossy().to_string();
            let f = File::open(path)?;
            DBusInfo(Node::from_reader(f)?, None, None, input_src)
        }
    };

    let fdo_iface_prefix = "org.freedesktop.DBus";
    let (fdo_standard_ifaces, needed_ifaces): (Vec<&Interface<'_>>, Vec<&Interface<'_>>) = node
        .interfaces()
        .iter()
        .partition(|&i| i.name().starts_with(fdo_iface_prefix));

    if !fdo_iface_prefix.is_empty() {
        eprintln!("Skipping `org.freedesktop.DBus` interfaces, please use https://docs.rs/zbus/latest/zbus/fdo/index.html")
    }

    let mut output_target = match args.output.as_deref() {
        Some("-") => OutputTarget::Stdout,
        Some(path) => {
            let file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)?;
            OutputTarget::SingleFile(file)
        }
        _ => OutputTarget::MultipleFiles,
    };

    for interface in needed_ifaces {
        let output = write_interfaces(
            &[&interface],
            &fdo_standard_ifaces,
            service.clone(),
            path.clone(),
            &input_src,
        )?;

        let interface_name = interface.name();
        match output_target {
            OutputTarget::Stdout => println!("{}", output),
            OutputTarget::SingleFile(ref mut file) => {
                file.write_all(output.as_bytes())?;
                println!("Generated code for `{}`", interface_name);
            }
            OutputTarget::MultipleFiles => {
                let filename = interface_name
                    .split('.')
                    .last()
                    .expect("Failed to split name");
                std::fs::write(format!("{}.rs", &filename), output)?;
                println!("Generated code for `{}` in {}.rs", interface_name, filename);
            }
        };
    }

    Ok(())
}

struct DBusInfo<'a>(
    Node<'a>,
    Option<BusName<'a>>,
    Option<ObjectPath<'a>>,
    String,
);

impl<'a> DBusInfo<'a> {
    fn new(
        connection: Connection,
        service: String,
        object_path: String,
    ) -> Result<Self, Box<dyn Error>> {
        let service: BusName<'_> = service.try_into()?;
        let path: ObjectPath<'_> = object_path.try_into()?;

        let input_src = format!(
            "Interface '{}' from service '{}' on system bus",
            path, service,
        );

        let xml = IntrospectableProxy::builder(&connection)
            .destination(service.clone())
            .expect("invalid destination")
            .path(path.clone())
            .expect("invalid path")
            .build()
            .unwrap()
            .introspect()?;

        Ok(DBusInfo(
            Node::from_reader(xml.as_bytes())?,
            Some(service),
            Some(path),
            input_src,
        ))
    }
}

fn write_interfaces(
    interfaces: &[&Interface<'_>],
    standard_interfaces: &[&Interface<'_>],
    service: Option<BusName<'_>>,
    path: Option<ObjectPath<'_>>,
    input_src: &str,
) -> Result<String, Box<dyn Error>> {
    use std::fmt::Write;

    let mut buffer = String::new();
    write_doc_header(&mut buffer, interfaces, standard_interfaces, input_src)?;

    for interface in interfaces {
        let gen = GenTrait {
            interface,
            service: service.as_ref(),
            path: path.as_ref(),
        };

        write!(buffer, "{}", gen)?;
    }

    Ok(buffer)
}

/// Write a doc header, listing the included Interfaces and how the
/// code was generated.
fn write_doc_header<W: std::fmt::Write>(
    w: &mut W,
    interfaces: &[&Interface<'_>],
    standard_interfaces: &[&Interface<'_>],
    input_src: &str,
) -> std::fmt::Result {
    if let Some((first_iface, following_ifaces)) = interfaces.split_first() {
        if following_ifaces.is_empty() {
            writeln!(
                w,
                "//! # DBus interface proxy for: `{}`",
                first_iface.name()
            )?;
        } else {
            write!(
                w,
                "//! # DBus interface proxies for: `{}`",
                first_iface.name()
            )?;
            for iface in following_ifaces {
                write!(w, ", `{}`", iface.name())?;
            }
            writeln!(w)?;
        }
    }

    write!(
        w,
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
        write!(w,
            "//! This DBus object implements
             //! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
             //! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
             //!
            ")?;
        for iface in standard_interfaces {
            let idx = iface.name().rfind('.').unwrap() + 1;
            let name = &iface.name()[idx..];
            writeln!(w, "//! * [`zbus::fdo::{name}Proxy`]")?;
        }
        write!(
            w,
            "//!
             //! …consequently `{}` did not generate code for the above interfaces.
            ",
            env!("CARGO_BIN_NAME")
        )?;
    }

    write!(
        w,
        "
        use zbus::proxy;
        "
    )?;

    Ok(())
}
