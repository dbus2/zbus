use std::env;
use std::error::Error;
use std::fs::File;
use std::result::Result;

use zbus::xml::Node;

mod gen;
use gen::GenTrait;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;

    let node = Node::from_reader(f)?;
    for iface in node.interfaces() {
        println!("{}", GenTrait(&iface));
        println!();
    }

    Ok(())
}
