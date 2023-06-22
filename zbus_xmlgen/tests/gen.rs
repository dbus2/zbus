use pretty_assertions::assert_eq;
use std::{env, error::Error, io::Write, path::Path, result::Result};

use zbus::xml::Node;
use zbus_xmlgen::GenTrait;

#[test]
fn sample_object0() -> Result<(), Box<dyn Error>> {
    let input = include_str!("data/sample_object0.xml");
    let expected = include_str!("data/sample_object0.rs");
    #[cfg(windows)]
    let expected = expected.replace("\r\n", "\n");

    let node = Node::from_reader(input.as_bytes())?;
    let gen = GenTrait {
        interface: &node.interfaces()[0],
        path: None,
        service: None,
    }
    .to_string();

    if env::var("TEST_OVERWRITE").is_ok() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("sample_object0.rs");
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        f.write_all(gen.as_bytes())?;
        f.flush()?;
        return Ok(());
    }

    assert_eq!(gen, expected);
    Ok(())
}
