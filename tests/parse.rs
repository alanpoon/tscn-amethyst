extern crate tscn_amethyst;
use std::path::Path;
use tscn_amethyst::parse;
#[test]
fn tscn_from_file() {
    
    let my_conf = parse(Path::new("tests/GridMap.tscn"));
}