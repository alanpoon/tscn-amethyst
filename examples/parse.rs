extern crate tscn_amethyst;
use std::path::Path;
use tscn_amethyst::parse;
fn main() {
    
    let entities = parse(Path::new("tests/t.tscn"));
    println!("entities {:?}",entities);
}