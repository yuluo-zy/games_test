#![allow(dead_code)]
#[path="../reconstruction/roof/surface.rs"]mod surface;
#[path="../reconstruction/roof/ridge.rs"]mod ridge;
#[path="../reconstruction/roof/tiles.rs"]mod tiles;
#[path="../reconstruction/roof/rectangular_edges.rs"]mod rectangular_edges;
#[path="../reconstruction/roof/rectangular_caps.rs"]mod rectangular_caps;
#[path="../reconstruction/roof/rectangular_faces.rs"]mod rectangular_faces;
#[path="../reconstruction/roof/pipeline.rs"]mod roof_pipeline;
use std::io::{self,BufRead};
fn decode(s:&str)->Vec<u8>{(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()}
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
 let rect:[u8;24]=decode(p[0]).try_into().unwrap();let roof:[u8;88]=decode(p[1]).try_into().unwrap();
 let batch=roof_pipeline::assemble_roof_tiles_observed(&roof,Some(&rect),p[2]=="1").unwrap();
 let state=batch.rectangular_final_state.unwrap_or((0,0));
 print!("{} {} {} {} {} ",state.0,state.1,batch.edges,batch.ridge_caps,batch.faces_and_fillers);
 for record in batch.records {for b in record.0 {print!("{b:02x}");}}println!();
}}
