#![allow(dead_code)]
#[path="../reconstruction/roof/surface.rs"] mod surface;
#[path="../reconstruction/roof/tiles.rs"] mod tiles;
#[path="../reconstruction/roof/rectangular_edges.rs"] mod rectangular_edges;
use std::io::{self,BufRead};
fn decode(s:&str)->Vec<u8>{(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()}
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
    let full=p[0]=="full";let start=usize::from(full);
    let roof:[u8;88]=decode(p[start]).try_into().unwrap();let raw=decode(p[start+1]);
    let frames=if full {rectangular_edges::frames_from_observed(&raw.try_into().unwrap(),&roof)} else {std::array::from_fn(|i|{let f=|o|f32::from_le_bytes(raw[i*16+o..i*16+o+4].try_into().unwrap());rectangular_edges::EdgeFrame{origin_xz:[f(0),f(4)],delta_xz:[f(8),f(12)]}})};
    let mut rng=tiles::TileRng(0);let mut ordinal=0;
    let records=rectangular_edges::assemble_rectangular_edges(&roof,&frames,&mut rng,&mut ordinal);
    print!("{:016x} ",rng.0);for r in records {for b in r.0 {print!("{b:02x}");}}println!();
}}
