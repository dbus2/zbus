#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://storage.googleapis.com/fdo-gitlab-uploads/project/avatar/3213/zbus-logomark.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]

mod error;
pub use error::{Error, Result};

use quick_xml::{de::Deserializer, se::to_writer};
use serde::{Deserialize, Serialize};
use static_assertions::assert_impl_all;
use std::io::{BufReader, Read, Write};

use zbus_names::{InterfaceName, MemberName, PropertyName};
use zvariant::CompleteType;

/// Annotations are generic key/value pairs of metadata.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Annotation {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
}

assert_impl_all!(Annotation: Send, Sync, Unpin);

impl Annotation {
    /// Return the annotation name/key.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the annotation value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// A direction of an argument
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ArgDirection {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

/// An argument
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Arg<'a> {
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@type", borrow)]
    ty: CompleteType<'a>,
    #[serde(rename = "@direction")]
    direction: Option<ArgDirection>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

assert_impl_all!(Arg<'_>: Send, Sync, Unpin);

impl<'a> Arg<'a> {
    /// Return the argument name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the argument type.
    pub fn ty(&self) -> &CompleteType<'a> {
        &self.ty
    }

    /// Return the argument direction, if any.
    pub fn direction(&self) -> Option<ArgDirection> {
        self.direction
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A method
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Method<'a> {
    #[serde(rename = "@name", borrow)]
    name: MemberName<'a>,
    #[serde(rename = "arg", default, borrow)]
    args: Vec<Arg<'a>>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

assert_impl_all!(Method<'_>: Send, Sync, Unpin);

impl<'a> Method<'a> {
    /// Return the method name.
    pub fn name(&self) -> MemberName<'_> {
        self.name.as_ref()
    }

    /// Return the method arguments.
    pub fn args(&self) -> &[Arg<'a>] {
        &self.args
    }

    /// Return the method annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A signal
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Signal<'a> {
    #[serde(rename = "@name", borrow)]
    name: MemberName<'a>,

    #[serde(rename = "arg", default)]
    args: Vec<Arg<'a>>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

assert_impl_all!(Signal<'_>: Send, Sync, Unpin);

impl<'a> Signal<'a> {
    /// Return the signal name.
    pub fn name(&self) -> MemberName<'_> {
        self.name.as_ref()
    }

    /// Return the signal arguments.
    pub fn args(&self) -> &[Arg<'a>] {
        &self.args
    }

    /// Return the signal annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// The possible property access types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyAccess {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "readwrite")]
    ReadWrite,
}

impl PropertyAccess {
    pub fn read(&self) -> bool {
        matches!(self, PropertyAccess::Read | PropertyAccess::ReadWrite)
    }

    pub fn write(&self) -> bool {
        matches!(self, PropertyAccess::Write | PropertyAccess::ReadWrite)
    }
}

/// A property
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Property<'a> {
    #[serde(rename = "@name", borrow)]
    name: PropertyName<'a>,

    #[serde(rename = "@type")]
    ty: CompleteType<'a>,
    #[serde(rename = "@access")]
    access: PropertyAccess,

    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

assert_impl_all!(Property<'_>: Send, Sync, Unpin);

impl<'a> Property<'a> {
    /// Returns the property name.
    pub fn name(&self) -> PropertyName<'_> {
        self.name.as_ref()
    }

    /// Returns the property type.
    pub fn ty(&self) -> &CompleteType<'a> {
        &self.ty
    }

    /// Returns the property access flags (should be "read", "write" or "readwrite").
    pub fn access(&self) -> PropertyAccess {
        self.access
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// An interface
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Interface<'a> {
    #[serde(rename = "@name", borrow)]
    name: InterfaceName<'a>,

    #[serde(rename = "method", default)]
    methods: Vec<Method<'a>>,
    #[serde(rename = "property", default)]
    properties: Vec<Property<'a>>,
    #[serde(rename = "signal", default)]
    signals: Vec<Signal<'a>>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

assert_impl_all!(Interface<'_>: Send, Sync, Unpin);

impl<'a> Interface<'a> {
    /// Returns the interface name.
    pub fn name(&self) -> InterfaceName<'_> {
        self.name.as_ref()
    }

    /// Returns the interface methods.
    pub fn methods(&self) -> &[Method<'a>] {
        &self.methods
    }

    /// Returns the interface signals.
    pub fn signals(&self) -> &[Signal<'a>] {
        &self.signals
    }

    /// Returns the interface properties.
    pub fn properties(&self) -> &[Property<'_>] {
        &self.properties
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// An introspection tree node (typically the root of the XML document).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Node<'a> {
    #[serde(rename = "@name")]
    name: Option<String>,

    #[serde(rename = "interface", default, borrow)]
    interfaces: Vec<Interface<'a>>,
    #[serde(rename = "node", default, borrow)]
    nodes: Vec<Node<'a>>,
}

assert_impl_all!(Node<'_>: Send, Sync, Unpin);

impl<'a> Node<'a> {
    /// Parse the introspection XML document from reader.
    pub fn from_reader<R: Read>(reader: R) -> Result<Node<'a>> {
        let mut deserializer = Deserializer::from_reader(BufReader::new(reader));
        deserializer.event_buffer_size(Some(1024_usize.try_into().unwrap()));
        Ok(Node::deserialize(&mut deserializer)?)
    }

    /// Write the XML document to writer.
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Need this wrapper until this is resolved: https://github.com/tafia/quick-xml/issues/499
        struct Writer<T>(T);

        impl<T> std::fmt::Write for Writer<T>
        where
            T: Write,
        {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
            }
        }

        to_writer(Writer(writer), &self)?;

        Ok(())
    }

    /// Returns the node name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the children nodes.
    pub fn nodes(&self) -> &[Node<'a>] {
        &self.nodes
    }

    /// Returns the interfaces on this node.
    pub fn interfaces(&self) -> &[Interface<'a>] {
        &self.interfaces
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = Error;

    /// Parse the introspection XML document from `s`.
    fn try_from(s: &'a str) -> Result<Node<'a>> {
        let mut deserializer = Deserializer::from_str(s);
        deserializer.event_buffer_size(Some(1024_usize.try_into().unwrap()));
        Ok(Node::deserialize(&mut deserializer)?)
    }
}
