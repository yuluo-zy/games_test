#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] mod surface;
#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] mod tiles;
use std::io::{self, BufRead};
fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let x:Vec<_>=l.split_whitespace().collect();
if x[0]=="split" {let n:usize=x[1].parse().unwrap();let amp=f32::from_bits(u32::from_str_radix(x[2],16).unwrap());let mut rng=tiles::TileRng(u64::from_str_radix(x[3],16).unwrap());let y=tiles::random_splits(n,amp,&mut rng);print!("{:016x}",rng.0);for v in y {print!(" {:08x}",v.to_bits());}println!();}
else if x[0]=="row"{let p=tiles::circular_row_plan(f32::from_bits(u32::from_str_radix(x[1],16).unwrap()));println!("{} {:08x} {:08x}",p.row_count,p.tile_height.to_bits(),p.denominator.to_bits());}
}}
