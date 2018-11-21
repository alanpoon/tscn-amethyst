extern crate libtscn;
use std::path::Path;

#[test]
fn tscn_from_file() {
    let my_conf = reader::from_file(Path::new("tests/GridMap.tscn"));
}