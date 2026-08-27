#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/z-galaxy/zbus/9f7a90d2b594ddc48b7a5f39fda5e00cd56a7dfb/logo.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]

mod error;
pub use error::{Error, Result, XmlError};

mod xml;
use xml::escape;

pub mod telepathy;

use std::{
    fmt,
    io::{BufWriter, Read, Write},
};

use zbus::{
    Signature, Str,
    names::{InterfaceName, MemberName, PropertyName},
};

/// A warning about document content that was ignored during parsing.
///
/// The D-Bus introspection format is sometimes extended with elements from other vocabularies,
/// most notably the [Telepathy extensions] (`tp:enum`, `tp:struct`, …). The parser skips over
/// any element it has no use for — or understands but cannot make sense of, e. g. a Telepathy
/// type definition missing a required attribute — and records a `Warning`, which
/// [`Node::from_reader_with_warnings`] hands back to the caller.
///
/// [Telepathy extensions]: https://telepathy.freedesktop.org/spec/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    element: String,
    position: usize,
    message: String,
}

impl Warning {
    /// A warning about a skipped element that is not part of the introspection format.
    pub(crate) fn unsupported(element: impl Into<String>, position: usize) -> Self {
        let element = element.into();
        let message = format!("unsupported element `<{element}>` ignored");
        Self {
            element,
            position,
            message,
        }
    }

    /// A warning about a skipped element that is understood but could not be parsed.
    pub(crate) fn malformed(
        element: impl Into<String>,
        position: usize,
        reason: impl fmt::Display,
    ) -> Self {
        let element = element.into();
        let message = format!("malformed element `<{element}>` ignored: {reason}");
        Self {
            element,
            position,
            message,
        }
    }

    /// The name of the element that was ignored.
    pub fn element(&self) -> &str {
        &self.element
    }

    /// The byte offset in the document at which the element starts.
    pub fn position(&self) -> usize {
        self.position
    }

    /// A message describing what was ignored, and why.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (at byte offset {})", self.message, self.position)
    }
}

/// Annotations are generic key/value pairs of metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation<'a> {
    name: Str<'a>,
    value: Str<'a>,
}

impl Annotation<'_> {
    /// Return the annotation name/key.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the annotation value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Annotation<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    pub fn into_owned(self) -> Annotation<'static> {
        Annotation {
            name: self.name.into_owned(),
            value: self.value.into_owned(),
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(
            w,
            "<annotation name=\"{}\" value=\"{}\"/>",
            escape(&self.name),
            escape(&self.value)
        )
    }
}

/// A direction of an argument
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArgDirection {
    In,
    Out,
}

impl ArgDirection {
    fn xml_value(&self) -> &'static str {
        match self {
            ArgDirection::In => "in",
            ArgDirection::Out => "out",
        }
    }
}

/// An argument
#[derive(Debug, Clone, PartialEq)]
pub struct Arg<'a> {
    name: Option<Str<'a>>,
    ty: Signature,
    direction: Option<ArgDirection>,
    annotations: Vec<Annotation<'a>>,
    docstring: Option<Str<'a>>,
    tp_type: Option<Str<'a>>,
}

impl<'a> Arg<'a> {
    /// Return the argument name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the argument type.
    pub fn ty(&self) -> &Signature {
        &self.ty
    }

    /// Return the argument direction, if any.
    pub fn direction(&self) -> Option<ArgDirection> {
        self.direction
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation<'a>] {
        &self.annotations
    }

    /// Return the content of the Telepathy `tp:docstring` extension element, if any.
    ///
    /// The content — typically HTML — is returned as it appears in the document, with the
    /// surrounding whitespace trimmed. Note that docstrings are only captured when parsing;
    /// the writer does not emit them.
    pub fn docstring(&self) -> Option<&str> {
        self.docstring.as_deref()
    }

    /// Return the named Telepathy type of the argument (its `tp:type` attribute), if any.
    ///
    /// The name refers to a [type definition](telepathy::TypeDef) in scope, with one `[]`
    /// suffix per level of array nesting (e. g. `Playlist[]`).
    pub fn tp_type(&self) -> Option<&str> {
        self.tp_type.as_deref()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Arg<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    pub fn into_owned(self) -> Arg<'static> {
        Arg {
            name: self.name.map(Str::into_owned),
            ty: self.ty,
            direction: self.direction,
            annotations: self
                .annotations
                .into_iter()
                .map(Annotation::into_owned)
                .collect(),
            docstring: self.docstring.map(Str::into_owned),
            tp_type: self.tp_type.map(Str::into_owned),
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(w, "<arg")?;
        if let Some(name) = &self.name {
            write!(w, " name=\"{}\"", escape(name))?;
        }
        write!(w, " type=\"{}\"", escape(&self.ty.to_string()))?;
        if let Some(direction) = self.direction {
            write!(w, " direction=\"{}\"", direction.xml_value())?;
        }
        if self.annotations.is_empty() {
            return write!(w, "/>");
        }
        write!(w, ">")?;
        for annotation in &self.annotations {
            annotation.write_xml(w)?;
        }
        write!(w, "</arg>")
    }
}

/// A method
#[derive(Debug, Clone, PartialEq)]
pub struct Method<'a> {
    name: MemberName<'a>,
    args: Vec<Arg<'a>>,
    annotations: Vec<Annotation<'a>>,
    docstring: Option<Str<'a>>,
}

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
    pub fn annotations(&self) -> &[Annotation<'a>] {
        &self.annotations
    }

    /// Return the content of the Telepathy `tp:docstring` extension element, if any.
    ///
    /// The content — typically HTML — is returned as it appears in the document, with the
    /// surrounding whitespace trimmed. Note that docstrings are only captured when parsing;
    /// the writer does not emit them.
    pub fn docstring(&self) -> Option<&str> {
        self.docstring.as_deref()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Method<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    pub fn into_owned(self) -> Method<'static> {
        Method {
            name: self.name.into_owned(),
            args: self.args.into_iter().map(Arg::into_owned).collect(),
            annotations: self
                .annotations
                .into_iter()
                .map(Annotation::into_owned)
                .collect(),
            docstring: self.docstring.map(Str::into_owned),
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(w, "<method name=\"{}\"", escape(self.name.as_str()))?;
        if self.args.is_empty() && self.annotations.is_empty() {
            return write!(w, "/>");
        }
        write!(w, ">")?;
        for arg in &self.args {
            arg.write_xml(w)?;
        }
        for annotation in &self.annotations {
            annotation.write_xml(w)?;
        }
        write!(w, "</method>")
    }
}

/// A signal
#[derive(Debug, Clone, PartialEq)]
pub struct Signal<'a> {
    name: MemberName<'a>,

    args: Vec<Arg<'a>>,
    annotations: Vec<Annotation<'a>>,
    docstring: Option<Str<'a>>,
}

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
    pub fn annotations(&self) -> &[Annotation<'a>] {
        &self.annotations
    }

    /// Return the content of the Telepathy `tp:docstring` extension element, if any.
    ///
    /// The content — typically HTML — is returned as it appears in the document, with the
    /// surrounding whitespace trimmed. Note that docstrings are only captured when parsing;
    /// the writer does not emit them.
    pub fn docstring(&self) -> Option<&str> {
        self.docstring.as_deref()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Signal<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    pub fn into_owned(self) -> Signal<'static> {
        Signal {
            name: self.name.into_owned(),
            args: self.args.into_iter().map(Arg::into_owned).collect(),
            annotations: self
                .annotations
                .into_iter()
                .map(Annotation::into_owned)
                .collect(),
            docstring: self.docstring.map(Str::into_owned),
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(w, "<signal name=\"{}\"", escape(self.name.as_str()))?;
        if self.args.is_empty() && self.annotations.is_empty() {
            return write!(w, "/>");
        }
        write!(w, ">")?;
        for arg in &self.args {
            arg.write_xml(w)?;
        }
        for annotation in &self.annotations {
            annotation.write_xml(w)?;
        }
        write!(w, "</signal>")
    }
}

/// The possible property access types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyAccess {
    Read,
    Write,
    ReadWrite,
}

impl PropertyAccess {
    pub fn read(&self) -> bool {
        matches!(self, PropertyAccess::Read | PropertyAccess::ReadWrite)
    }

    pub fn write(&self) -> bool {
        matches!(self, PropertyAccess::Write | PropertyAccess::ReadWrite)
    }

    fn xml_value(&self) -> &'static str {
        match self {
            PropertyAccess::Read => "read",
            PropertyAccess::Write => "write",
            PropertyAccess::ReadWrite => "readwrite",
        }
    }
}

/// A property
#[derive(Debug, Clone, PartialEq)]
pub struct Property<'a> {
    name: PropertyName<'a>,

    ty: Signature,
    access: PropertyAccess,

    annotations: Vec<Annotation<'a>>,
    docstring: Option<Str<'a>>,
    tp_type: Option<Str<'a>>,
}

impl<'a> Property<'a> {
    /// Returns the property name.
    pub fn name(&self) -> PropertyName<'_> {
        self.name.as_ref()
    }

    /// Returns the property type.
    pub fn ty(&self) -> &Signature {
        &self.ty
    }

    /// Returns the property access flags (should be "read", "write" or "readwrite").
    pub fn access(&self) -> PropertyAccess {
        self.access
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation<'a>] {
        &self.annotations
    }

    /// Return the content of the Telepathy `tp:docstring` extension element, if any.
    ///
    /// The content — typically HTML — is returned as it appears in the document, with the
    /// surrounding whitespace trimmed. Note that docstrings are only captured when parsing;
    /// the writer does not emit them.
    pub fn docstring(&self) -> Option<&str> {
        self.docstring.as_deref()
    }

    /// Return the named Telepathy type of the property (its `tp:type` attribute), if any.
    ///
    /// The name refers to a [type definition](telepathy::TypeDef) in scope, with one `[]`
    /// suffix per level of array nesting (e. g. `Playlist[]`).
    pub fn tp_type(&self) -> Option<&str> {
        self.tp_type.as_deref()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Property<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    pub fn into_owned(self) -> Property<'static> {
        Property {
            name: self.name.into_owned(),
            ty: self.ty,
            access: self.access,
            annotations: self
                .annotations
                .into_iter()
                .map(Annotation::into_owned)
                .collect(),
            docstring: self.docstring.map(Str::into_owned),
            tp_type: self.tp_type.map(Str::into_owned),
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(
            w,
            "<property name=\"{}\" type=\"{}\" access=\"{}\"",
            escape(self.name.as_str()),
            escape(&self.ty.to_string()),
            self.access.xml_value()
        )?;
        if self.annotations.is_empty() {
            return write!(w, "/>");
        }
        write!(w, ">")?;
        for annotation in &self.annotations {
            annotation.write_xml(w)?;
        }
        write!(w, "</property>")
    }
}

/// An interface
#[derive(Debug, Clone, PartialEq)]
pub struct Interface<'a> {
    name: InterfaceName<'a>,

    methods: Vec<Method<'a>>,
    properties: Vec<Property<'a>>,
    signals: Vec<Signal<'a>>,
    annotations: Vec<Annotation<'a>>,
    docstring: Option<Str<'a>>,
    telepathy_types: Vec<telepathy::TypeDef<'a>>,
}

impl<'a> Interface<'a> {
    /// Returns the interface name.
    pub fn name(&self) -> InterfaceName<'_> {
        self.name.as_ref()
    }

    /// Returns the interface methods.
    pub fn methods(&self) -> impl Iterator<Item = &Method<'a>> {
        self.methods.iter()
    }

    /// Returns the interface signals.
    pub fn signals(&self) -> impl Iterator<Item = &Signal<'a>> {
        self.signals.iter()
    }

    /// Returns the interface properties.
    pub fn properties(&self) -> impl Iterator<Item = &Property<'a>> {
        self.properties.iter()
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation<'a>> {
        self.annotations.iter()
    }

    /// Return the content of the Telepathy `tp:docstring` extension element, if any.
    ///
    /// The content — typically HTML — is returned as it appears in the document, with the
    /// surrounding whitespace trimmed. Note that docstrings are only captured when parsing;
    /// the writer does not emit them.
    pub fn docstring(&self) -> Option<&str> {
        self.docstring.as_deref()
    }

    /// Return the Telepathy type definitions on this interface.
    pub fn telepathy_types(&self) -> impl Iterator<Item = &telepathy::TypeDef<'a>> {
        self.telepathy_types.iter()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Interface<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    pub fn into_owned(self) -> Interface<'static> {
        Interface {
            name: self.name.into_owned(),
            methods: self.methods.into_iter().map(Method::into_owned).collect(),
            properties: self
                .properties
                .into_iter()
                .map(Property::into_owned)
                .collect(),
            signals: self.signals.into_iter().map(Signal::into_owned).collect(),
            annotations: self
                .annotations
                .into_iter()
                .map(Annotation::into_owned)
                .collect(),
            docstring: self.docstring.map(Str::into_owned),
            telepathy_types: self
                .telepathy_types
                .into_iter()
                .map(telepathy::TypeDef::into_owned)
                .collect(),
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(w, "<interface name=\"{}\"", escape(self.name.as_str()))?;
        if self.methods.is_empty()
            && self.properties.is_empty()
            && self.signals.is_empty()
            && self.annotations.is_empty()
        {
            return write!(w, "/>");
        }
        write!(w, ">")?;
        for method in &self.methods {
            method.write_xml(w)?;
        }
        for property in &self.properties {
            property.write_xml(w)?;
        }
        for signal in &self.signals {
            signal.write_xml(w)?;
        }
        for annotation in &self.annotations {
            annotation.write_xml(w)?;
        }
        write!(w, "</interface>")
    }
}

/// An introspection tree node (typically the root of the XML document).
#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    name: Option<Str<'a>>,

    interfaces: Vec<Interface<'a>>,
    nodes: Vec<Node<'a>>,
    docstring: Option<Str<'a>>,
    telepathy_types: Vec<telepathy::TypeDef<'a>>,
}

impl<'a> Node<'a> {
    /// Parse the introspection XML document from reader.
    ///
    /// The whole reader is buffered into memory before parsing, so the returned tree owns its
    /// data and is not tied to the reader's lifetime. For a zero-copy parse that borrows from a
    /// document already in memory, parse it with `Node::try_from(&str)` instead.
    ///
    /// Note that `reader` is consumed until end-of-stream before parsing, so this must not be
    /// used with a reader that stays open past the end of the document (e.g. a socket).
    pub fn from_reader<R: Read>(reader: R) -> Result<Node<'static>> {
        Ok(Node::from_reader_with_warnings(reader)?.0)
    }

    /// Parse the introspection XML document from reader, collecting warnings.
    ///
    /// In addition to the parsed node, a [`Warning`] is returned for every element that is not
    /// part of the [introspection format] (except Telepathy docstrings, which are captured —
    /// see [`Interface::docstring`]) and was therefore ignored, e. g. the type-definition
    /// elements of the Telepathy extensions (`tp:enum`, `tp:struct`, …).
    ///
    /// The whole reader is buffered into memory before parsing, so the returned tree owns its
    /// data and is not tied to the reader's lifetime. For a zero-copy parse that borrows from a
    /// document already in memory, parse it with `Node::try_from(&str)` instead.
    ///
    /// Note that `reader` is consumed until end-of-stream before parsing, so this must not be
    /// used with a reader that stays open past the end of the document (e.g. a socket).
    ///
    /// [introspection format]: https://dbus.freedesktop.org/doc/dbus-specification.html#introspection-format
    pub fn from_reader_with_warnings<R: Read>(
        mut reader: R,
    ) -> Result<(Node<'static>, Vec<Warning>)> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let (node, warnings) = xml::parse_with_warnings(&input)?;

        Ok((node.into_owned(), warnings))
    }

    /// Write the XML document to writer.
    ///
    /// Note that data which is only captured when parsing — Telepathy docstrings, type
    /// definitions and `tp:type` references — is not written. Consequently, a document that
    /// carried any does not compare equal to its written-and-reparsed self.
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        let mut writer = BufWriter::new(writer);
        self.write_xml(&mut writer)?;
        writer.flush()?;

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

    /// Return the content of the Telepathy `tp:docstring` extension element, if any.
    ///
    /// The content — typically HTML — is returned as it appears in the document, with the
    /// surrounding whitespace trimmed. Note that docstrings are only captured when parsing;
    /// the writer does not emit them.
    pub fn docstring(&self) -> Option<&str> {
        self.docstring.as_deref()
    }

    /// Return the Telepathy type definitions on this node.
    pub fn telepathy_types(&self) -> &[telepathy::TypeDef<'a>] {
        &self.telepathy_types
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Node<'static> {
        self.clone().into_owned()
    }

    /// Converts `self` into an owned tree, copying the strings it borrows.
    ///
    /// Nested `<node>`s — the one axis on which a tree can nest arbitrarily deep — are walked
    /// iteratively on an explicit stack, as the parser does, so converting a deeply nested tree
    /// does not grow the call stack.
    pub fn into_owned(self) -> Node<'static> {
        // A node whose own data is converted, paired with the children still to convert.
        struct Pending<'a> {
            node: Node<'static>,
            children: std::vec::IntoIter<Node<'a>>,
        }

        impl<'a> Pending<'a> {
            fn new(node: Node<'a>) -> Self {
                let children = node.nodes.into_iter();
                let node = Node {
                    name: node.name.map(Str::into_owned),
                    interfaces: node
                        .interfaces
                        .into_iter()
                        .map(Interface::into_owned)
                        .collect(),
                    nodes: Vec::with_capacity(children.len()),
                    docstring: node.docstring.map(Str::into_owned),
                    telepathy_types: node
                        .telepathy_types
                        .into_iter()
                        .map(telepathy::TypeDef::into_owned)
                        .collect(),
                };

                Pending { node, children }
            }
        }

        let mut open = vec![Pending::new(self)];
        loop {
            let pending = open
                .last_mut()
                .expect("the root is popped only by returning");
            if let Some(child) = pending.children.next() {
                open.push(Pending::new(child));
                continue;
            }
            let finished = open.pop().expect("non-empty").node;
            match open.last_mut() {
                Some(parent) => parent.node.nodes.push(finished),
                None => return finished,
            }
        }
    }

    fn write_xml<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(w, "<node")?;
        if let Some(name) = &self.name {
            write!(w, " name=\"{}\"", escape(name))?;
        }
        if self.interfaces.is_empty() && self.nodes.is_empty() {
            return write!(w, "/>");
        }
        write!(w, ">")?;
        for interface in &self.interfaces {
            interface.write_xml(w)?;
        }
        for node in &self.nodes {
            node.write_xml(w)?;
        }
        write!(w, "</node>")
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = Error;

    /// Parse the introspection XML document from `s`, borrowing from it where possible.
    ///
    /// The returned tree borrows attribute values and docstrings from `s`, so it cannot outlive
    /// it. Call [`Node::into_owned`] to detach the tree from `s` and keep it around for longer.
    fn try_from(s: &'a str) -> Result<Node<'a>> {
        xml::parse(s)
    }
}
