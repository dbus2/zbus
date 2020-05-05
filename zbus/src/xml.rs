#![cfg(feature = "xml")]

//! Introspection XML support (`xml` feature)
//!
//! Thanks to the [`org.freedesktop.DBus.Introspectable`] interface, objects may be introspected at
//! runtime, returning an XML string that describes the object.
//!
//! This optional `xml` module provides facilities to parse the XML data into more convenient Rust
//! structures. The XML string may be parsed to a tree with [`Node.from_reader()`].
//!
//! See also:
//!
//! * [Introspection format] in the DBus specification
//!
//! [`Node.from_reader()`]: struct.Node.html#method.from_reader
//! [Introspection format]: https://dbus.freedesktop.org/doc/dbus-specification.html#introspection-format
//! [`org.freedesktop.DBus.Introspectable`]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-introspectable

use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_reader, to_writer, Error};
use std::io::{Read, Write};
use std::result::Result;

/// Annotations are generic key/value pairs of metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    name: String,
    value: String,
}

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

/// An argument
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Arg {
    name: Option<String>,
    r#type: String,
    direction: Option<String>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Arg {
    /// Return the argument name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the argument type.
    pub fn ty(&self) -> &str {
        &self.r#type
    }

    /// Return the argument direction (should be "in" or "out"), if any.
    pub fn direction(&self) -> Option<&str> {
        self.direction.as_deref()
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A method
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Method {
    name: String,

    #[serde(rename = "arg", default)]
    args: Vec<Arg>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Method {
    /// Return the method name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the method arguments.
    pub fn args(&self) -> &[Arg] {
        &self.args
    }

    /// Return the method annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A signal
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Signal {
    name: String,

    #[serde(rename = "arg", default)]
    args: Vec<Arg>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Signal {
    /// Return the signal name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the signal arguments.
    pub fn args(&self) -> &[Arg] {
        &self.args
    }

    /// Return the signal annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A property
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Property {
    name: String,
    r#type: String,
    access: String,

    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Property {
    /// Returns the property name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the property type.
    pub fn ty(&self) -> &str {
        &self.r#type
    }

    /// Returns the property access flags (should be "read", "write" or "readwrite").
    pub fn access(&self) -> &str {
        &self.access
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// An interface
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interface {
    name: String,

    #[serde(rename = "method", default)]
    methods: Vec<Method>,
    #[serde(rename = "signal", default)]
    signals: Vec<Signal>,
    #[serde(rename = "property", default)]
    properties: Vec<Property>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Interface {
    /// Returns the interface name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the interface methods.
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    /// Returns the interface signals.
    pub fn signals(&self) -> &[Signal] {
        &self.signals
    }

    /// Returns the interface properties.
    pub fn properties(&self) -> &[Property] {
        &self.properties
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// An introspection tree node (typically the root of the XML document).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
    name: Option<String>,

    #[serde(rename = "node", default)]
    nodes: Vec<Node>,
    #[serde(rename = "interface", default)]
    interfaces: Vec<Interface>,
}

impl Node {
    /// Parse the introspection XML document from reader.
    pub fn from_reader<R: Read>(reader: R) -> Result<Node, Error> {
        from_reader(reader)
    }

    /// Write the XML document to writer.
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<(), Error> {
        to_writer(writer, &self)
    }

    /// Returns the node name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the children nodes.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the interfaces on this node.
    pub fn interfaces(&self) -> &[Interface] {
        &self.interfaces
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::Node;

    static EXAMPLE: &str = r##"
<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN"
  "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
 <node name="/com/example/sample_object0">
   <interface name="com.example.SampleInterface0">
     <method name="Frobate">
       <arg name="foo" type="i" direction="in"/>
       <arg name="bar" type="s" direction="out"/>
       <arg name="baz" type="a{us}" direction="out"/>
       <annotation name="org.freedesktop.DBus.Deprecated" value="true"/>
     </method>
     <method name="Bazify">
       <arg name="bar" type="(iiu)" direction="in"/>
       <arg name="bar" type="v" direction="out"/>
     </method>
     <method name="Mogrify">
       <arg name="bar" type="(iiav)" direction="in"/>
     </method>
     <signal name="Changed">
       <arg name="new_value" type="b"/>
     </signal>
     <property name="Bar" type="y" access="readwrite"/>
   </interface>
   <node name="child_of_sample_object"/>
   <node name="another_child_of_sample_object"/>
</node>
"##;

    #[test]
    fn serde() -> Result<(), Box<dyn Error>> {
        let node = Node::from_reader(EXAMPLE.as_bytes())?;
        assert_eq!(node.interfaces().len(), 1);
        assert_eq!(node.nodes().len(), 2);

        // TODO: Fails at the moment, this seems fresh & related:
        // https://github.com/RReverser/serde-xml-rs/pull/129
        //let mut writer = Vec::with_capacity(128);
        //node.to_writer(&mut writer).unwrap();
        Ok(())
    }
}
