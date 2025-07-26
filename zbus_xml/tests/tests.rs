use quick_xml::de::DeError;
use std::error::Error;

use zbus_xml::{ArgDirection, Node, NodeEventLimit};

#[test]
fn serde() -> Result<(), Box<dyn Error>> {
    let example = include_str!("data/sample_object0.xml");
    let limit = NodeEventLimit::new(28);
    let node_r = limit.read(example.as_bytes())?;
    let node = limit.parse(example)?;
    assert_eq!(node, node_r);
    assert_eq!(node.interfaces().len(), 1);
    assert_eq!(node.interfaces()[0].methods().len(), 3);
    assert_eq!(
        node.interfaces()[0].methods()[0].args()[0]
            .direction()
            .unwrap(),
        ArgDirection::In
    );
    assert_eq!(node.nodes().len(), 4);

    let node_str: Node<'_> = example.try_into()?;
    assert_eq!(node_str.interfaces().len(), 1);
    assert_eq!(node_str.nodes().len(), 4);

    let mut writer = Vec::with_capacity(128);
    node.to_writer(&mut writer).unwrap();
    Ok(())
}

#[test]
fn invalid_arg_type() {
    let input = include_str!("data/invalid_arg_type.xml");
    let limit = NodeEventLimit::new(1024);
    assert!(matches!(
        limit.parse(input),
        Err(zbus_xml::Error::QuickXml(DeError::Custom(_)))
    ));
}

#[test]
fn small_limit() {
    let input = include_str!("data/sample_object0.xml");
    let limit = NodeEventLimit::new(27);
    assert!(matches!(
        limit.parse(input),
        Err(zbus_xml::Error::QuickXml(DeError::TooManyEvents(_)))
    ));
}
