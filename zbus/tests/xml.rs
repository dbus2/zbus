#![cfg(feature = "xml")]

use std::convert::TryFrom;

use zbus::xml::Node;
use quick_xml::de::DeError;

#[test]
fn invalid_arg_type() {
    let input = include_str!("data/invalid_arg_type.xml");
    assert!(matches!(
        Node::try_from(input),
        Err(zbus::Error::QuickXml(DeError::Custom(_)))
    ));
}
