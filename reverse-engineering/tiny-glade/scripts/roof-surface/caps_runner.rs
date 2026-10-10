#[path="../../reconstruction/roof/tiles.rs"] mod tiles;
#[path="../../reconstruction/roof/surface.rs"] mod surface;
#[path="../../reconstruction/roof/ridge.rs"] mod ridge;
#[path="../../reconstruction/roof/rectangular_caps.rs"] mod rectangular_caps;
use std::io::{self,BufRead};
fn main(){
    for line in io::stdin().lock().lines(){let line=line.unwrap();let w:Vec<_>=line.split_whitespace().collect();
        let f=|i|f32::from_bits(u32::from_str_radix(w[i],16).unwrap());
        let input=rectangular_caps::CapInput {endpoints:[[f(0),f(1),f(2)],[f(3),f(4),f(5)]],
            height_01:f(6),profile_01:f(7),is_gable:w[8]=="1",gable_style_byte:w[9].parse().unwrap(),
            roof_id:w[10].parse().unwrap(),special_mode:w[11]=="1"};
        let mut rng=tiles::TileRng(u64::from_str_radix(w[12],16).unwrap());let mut ordinal=w[13].parse().unwrap();
        let decode=|s:&str|->Vec<u8>{(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()};
        let rect:[u8;24]=decode(w[14]).try_into().unwrap();let roof:[u8;88]=decode(w[15]).try_into().unwrap();
        let actual_endpoints=rectangular_caps::cap_endpoints_from_observed(&rect,&roof);
        assert_eq!(actual_endpoints.map(|v|v.map(f32::to_bits)),input.endpoints.map(|v|v.map(f32::to_bits)),"endpoint bits");
        let out=rectangular_caps::assemble_caps_from_observed(&rect,&roof,input.special_mode,&mut rng,&mut ordinal);
        print!("{:016x} {ordinal}",rng.0);for record in out {print!(" ");for b in record.0 {print!("{b:02x}");}}println!();
    }
}
