use pretty_assertions::assert_eq;
use std::{env, error::Error, io::Write, path::Path};

use zbus_xml::Node;
use zbus_xmlgen::GenTrait;

macro_rules! gen_diff {
    ($infile:literal, $outfile:literal) => {{
        let input = include_str!(concat!("data/", $infile));
        let expected = include_str!(concat!("data/", $outfile));
        #[cfg(windows)]
        let expected = expected.replace("\r\n", "\n");
        let node = Node::from_reader(input.as_bytes(), 1024)?;
        let gen = GenTrait {
            interface: &node.interfaces()[0],
            path: None,
            service: None,
            format: true,
        }
        .to_string();

        if env::var("TEST_OVERWRITE").is_ok() {
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("data")
                .join($outfile);
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
    }};
}

#[test]
fn sample_object0() -> Result<(), Box<dyn Error>> {
    gen_diff!("sample_object0.xml", "sample_object0.rs")
}
