#![crate_name = "tscn_amethyst"]
#![crate_type = "lib"]
#![warn(missing_docs)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[macro_use]
extern crate serde;
extern crate ini;
use ini::Ini;
#[derive(Serialize)]
pub struct ObjFormat
#[derive(Serialize)]
pub struct File(String,ObjFormat,Result)
#[derive(Serialize)]
pub struct Asset(File)
#[derive(Serialize)]
pub struct data{
    graphics:{
        mesh:Asset
    },
    transform:{
        matrix:[f32;12]
    }
}
named!(transform_,do_parse!(
    take_until!(" ") >>
    separated_list!(tag!(","))
));
pub fn parse(filename:&Path){
    let conf = Ini::load_from_file(filename).unwrap();
    let ext_resource_vec = vec![];
    let entities = vec![];
    // iterating
    for (sec, prop) in &conf {
        let sec_split  =sec.split(" ");
        if (sec_split[0]=="ext_resource"){
            ext_resource_vec.push(sec_split[1].split("path=")[1]);
        }
        if (sec_split[0]=="node"){
            let e_r = prop["mesh"][12:prop["mesh"].find(")")-1].parse::<usize>().unwrap();
            let e_t = prop["transfrom"].split(" ");
            let data_1 = data{
                graphics:{
                    mesh:Asset(File(ext_resource_vec[e_r-1],ObjFormat,()))
                },
                transform:{
                    matrix:many0!(tag!(""))
                }
            };
        }
        println!("Section: {:?}", sec);
        for (key, value) in prop{
            println!("{:?}:{:?}", key, value);
        }
    }
}