use std::env;
use std::error::Error;
use std::fs::File;
use std::result::Result;

use ::rustfmt::format_input;
use zbus::xml::Node;

mod gen;
use gen::GenTrait;

fn main() -> Result<(), Box<dyn Error>> {
    let fmtconfig = ::rustfmt::config::Config::default();

    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;

    let node = Node::from_reader(f)?;
    for iface in node.interfaces() {
        let gen = format!("{}", GenTrait(&iface));
        let mut out: Vec<u8> = Vec::new();
        let (summary, filemap, _) =
            format_input(::rustfmt::Input::Text(gen), &fmtconfig, Some(&mut out)).unwrap();
        assert!(summary.has_no_errors());
        println!("{}", filemap[0].1)
    }

    Ok(())
}
