#![deny(rust_2018_idioms)]

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
};

use clap::Parser;
use snakecase::ascii::to_snakecase;
use zbus::{
    blocking::{connection, fdo::IntrospectableProxy, Connection},
    names::BusName,
    zvariant::ObjectPath,
};
use zbus_xml::{Interface, Node, NodeEventLimit};

use zbus_xmlgen::write_interfaces;

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
            let limit = NodeEventLimit::new(4096);
            DBusInfo(limit.read(f)?, None, None, input_src)
        }
    };

    let fdo_iface_prefix = "org.freedesktop.DBus";
    let (fdo_standard_ifaces, needed_ifaces): (Vec<Interface<'_>>, Vec<Interface<'_>>) = node
        .interfaces()
        .iter()
        .cloned()
        .partition(|i| i.name().starts_with(fdo_iface_prefix));

    if !fdo_standard_ifaces.is_empty() {
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
            &[interface.clone()],
            &fdo_standard_ifaces,
            service.clone(),
            path.clone(),
            &input_src,
            env!("CARGO_BIN_NAME"),
            env!("CARGO_PKG_VERSION"),
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
                    .next_back()
                    .expect("Failed to split name");
                let filename = to_snakecase(filename);
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

impl DBusInfo<'_> {
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

        let limit = NodeEventLimit::new(4096);
        Ok(DBusInfo(
            limit.read(xml.as_bytes())?,
            Some(service),
            Some(path),
            input_src,
        ))
    }
}
