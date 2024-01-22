#![deny(rust_2018_idioms)]

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::{ChildStdin, Command, Stdio},
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

    let mut output_target = match args.output.as_deref() {
        Some("-") => OutputTarget::Stdout,
        Some(path) => {
            let file = OpenOptions::new().create(true).write(true).open(path)?;
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
        match output_target {
            OutputTarget::Stdout => println!("{}", output),
            OutputTarget::SingleFile(ref mut file) => file.write_all(output.as_bytes())?,
            OutputTarget::MultipleFiles => std::fs::write(
                format!(
                    "{}.rs",
                    interface
                        .name()
                        .split('.')
                        .last()
                        .expect("Failed to split name")
                ),
                output,
            )?,
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
    let mut process = match Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // rustfmt may post warnings about features not being enabled on stable rust
        // these can be distracting and are irrevelant to the user, so we hide them
        .stderr(Stdio::null())
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
