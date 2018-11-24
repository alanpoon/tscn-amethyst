extern crate tscn_amethyst;
use std::path::Path;
use tscn_amethyst::parse;
extern crate ini;
use ini::Ini;
fn main() {
    
    //parse(Path::new("tests/t.tscn"));
    let conf = Ini::load_from_file("tests/t.tscn").unwrap();

    // iterating
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop{
            println!("{:?}:{:?}", key, value);
        }
    }
}