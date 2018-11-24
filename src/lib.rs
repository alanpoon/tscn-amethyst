#![crate_name = "tscn_amethyst"]
#![crate_type = "lib"]
#![warn(missing_docs)]

use std::io::prelude::*;
use std::path::Path;
#[macro_use]
extern crate serde;
extern crate ini;
#[macro_use]
extern crate nom;
use nom::digit;
use ini::Ini;
use std::str::FromStr;
#[derive(Serialize)]
pub struct ObjFormat{}
#[derive(Serialize)]
pub struct File(String,ObjFormat,());
#[derive(Serialize)]
pub struct Asset(File);
#[derive(Serialize)]
pub struct graphics{
    mesh:Asset
}
#[derive(Serialize)]
pub struct transform{
    matrix:Vec<f32>
}
#[derive(Serialize)]
pub struct data{
    graphics:graphics,
    transform:transform
}
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}
named!(transform_<Vec<&str>>,do_parse!(
    a:take_until!(" ") >>
    b:separated_list!(tag!(","),take_until!(" ")) >>
    (b.iter().map(|x|std::str::from_utf8(x).unwrap()).collect())
    )
);
//named!( )
pub fn parse(filename:&Path){
    let conf = Ini::load_from_file(filename).unwrap();
    let mut ext_resource_vec = vec![];
    let mut entities:Vec<data> = vec![];
    // iterating
    for (sec, prop) in &conf {
        let sec_string = sec.to_owned().unwrap();
        let sec_split:Vec<&str>  =sec_string.split(" ").collect();
        if sec_split.get(0).unwrap()==&"ext_resource"{
            let sec_sec_split:Vec<&str> = sec_split.get(1).unwrap().split("path=").collect();
            ext_resource_vec.push(sec_sec_split.get(1).unwrap());
        }
        if sec_split.get(0).unwrap()==&"node"{
            let e_r_r = prop["mesh"].to_string();
            let e_r = e_r_r.substring(12,prop["mesh"].find(")").unwrap()-1).parse::<usize>().unwrap();
            let e_t = transform_(prop["transfrom"].as_bytes());
            let e_t_f32:Vec<f32> = e_t.unwrap().1.iter().map(|x| x.parse::<f32>().unwrap()).collect();
            let e_r_i = e_r - 1;
            let ex_i=ext_resource_vec.get(e_r_i).unwrap().to_string();
            let data_1 = data{
                graphics:graphics{
                    mesh:Asset(File(ex_i,ObjFormat{},()))
                },
                transform:transform{
                    matrix:vec![]
                }
            };
        }
        println!("Section: {:?}", sec);
        for (key, value) in prop{
            println!("{:?}:{:?}", key, value);
        }
    }
}