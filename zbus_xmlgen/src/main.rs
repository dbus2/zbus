use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::result::Result;

use zbus::xml::Node;

mod gen;
use gen::GenTrait;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: zbus-xmlgen <interface.xml>");
        return Ok(());
    }

    let f = File::open(&args[1])?;

    let node = Node::from_reader(f)?;
    let mut process = match Command::new("rustfmt").stdin(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn rustfmt: {}", why),
        Ok(process) => process,
    };
    for iface in node.interfaces() {
        let gen = GenTrait(&iface).to_string();
        process.stdin.as_mut().unwrap().write_all(gen.as_bytes())?;
    }
    process.wait()?;
    Ok(())
}
