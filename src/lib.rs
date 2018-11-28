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
use nom::float;
use ini::Ini;
#[derive(Serialize,Debug)]
pub struct ObjFormat{}
#[derive(Serialize,Debug)]
pub struct File(String,ObjFormat,());
#[derive(Serialize,Debug)]
pub struct Asset(File);
#[derive(Serialize,Debug)]
pub struct graphics{
    mesh:Asset
}
#[derive(Serialize,Debug)]
pub struct transform{
    matrix:[[f32; 4]; 4]
}
#[derive(Serialize,Debug)]
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
/*
named!(transform_<Vec<&str>>,do_parse!(
    _a:take_until_and_consume!(" ") >>
    b:separated_list!(char!(","),digit) >>
    (b.iter().map(|x|{
        println!("x {:?}",x);
        std::str::from_utf8(x).unwrap()}).collect())
    )
);
*/
named!(transform_<&str,Vec<f32>>,
     delimited!(
        tag!("Transform( "),
        separated_list!(tag!(", "), float),
        tag!(" )")
    )
);
named!(transform_sec<(&str,Vec<String>)>,do_parse!(
    node_name:map_res!(take_until!(" "),std::str::from_utf8) >>
    _path: many0!(tag!("path=")) >>
    path_value: many1!(map_res!(take_until!(" "),std::str::from_utf8)) >>
    (node_name,path_value.iter().map(|x|x.to_string()).collect())
));
//named!( )
pub fn parse(filename:&Path)->Vec<data>{
    let conf = Ini::load_from_file(filename).unwrap();
    let mut ext_resource_vec = vec![];
    let mut entities:Vec<data> = vec![];
    // iterating
    for (sec, prop) in &conf {
        let sec_str = sec.to_owned().unwrap().clone();
        println!("sec_str {:?}",sec_str);
        let transform_sec_res = transform_sec(sec_str.as_bytes()).unwrap().1;
        if transform_sec_res.0 =="ext_resource"{
            let sec_path = transform_sec_res.1.get(0).unwrap().clone();
            ext_resource_vec.push(sec_path);
        }
        if transform_sec_res.0 =="node"{
        //if sec_split.get(0).unwrap()==&"node"{
            let e_r_r = prop["mesh"].to_string();
            let e_r = e_r_r.substring(13,e_r_r.len()-15).parse::<usize>().unwrap();
            println!("prop:{:?}",prop["transform"]);
            let e_t = transform_(&prop["transform"]);
            println!("e_t {:?}",e_t);
            let e_t_f32:Vec<f32> = e_t.unwrap().1;
            let mut e_t_f32_arr:[[f32; 4]; 4]=[[0.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0]];
            let mut c:f32 =0.0;
            for t in e_t_f32{
                let y=(c/4.0).floor() as usize;
                let x=(c%4.0) as usize;
                e_t_f32_arr[y][x] =t;
                c+=1.0;
                if x == 3{
                    c+=1.0;
                }
            }
            let e_r_i = e_r - 1;
            println!("e_r_i {:?} length {:?}",e_r_i,ext_resource_vec.len());
            let ex_i=ext_resource_vec.get(e_r_i).unwrap().to_string();
            let data_1 = data{
                graphics:graphics{
                    mesh:Asset(File(ex_i,ObjFormat{},()))
                },
                transform:transform{
                    matrix:e_t_f32_arr
                }
            };
            entities.push(data_1);
        }

    }
    entities
}