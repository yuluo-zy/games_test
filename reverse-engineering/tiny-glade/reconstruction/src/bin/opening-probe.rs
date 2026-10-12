use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_holes, building_wall_state};
#[path = "../../building/opening_rules.rs"]
mod opening_rules;
use opening_rules::*;
fn value(text: &str) -> f32 {f32::from_bits(u32::from_str_radix(text,16).unwrap())}
fn main() {
    for line in io::stdin().lock().lines() {
        let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
        let family=match p[0] {"cw"=>OpeningFamily::CottageWindow,"gw"=>OpeningFamily::GothicWindow,"cd"=>OpeningFamily::CottageBalconyDoor,"gd"=>OpeningFamily::GothicBalconyDoor,_=>panic!("unknown family")};
        let mut shape=[0;28];shape[..4].copy_from_slice(&p[2].parse::<u32>().unwrap().to_le_bytes());
        let result=generate_wall_opening(DecoratorOpeningInput {family,size_variant:p[1].parse().unwrap(),center:[value(p[3]),value(p[4])],collision:DecoratorCollisionMetrics {extent_xz:[value(p[5]),value(p[6])],vertical_bounds:[value(p[7]),value(p[8])] }},&building_wall_state::WallShapeSnapshot::from_observed(shape));
        match result {
            Ok(holes)=>println!("{}",holes.into_iter().map(|h|format!("{:08x},{:08x},{:08x},{:08x},{}",h.center[0].to_bits(),h.center[1].to_bits(),h.dimensions[0].to_bits(),h.dimensions[1].to_bits(),h.kind)).collect::<Vec<_>>().join(";")),
            Err(error)=>println!("error:{error:?}"),
        }
    }
}
