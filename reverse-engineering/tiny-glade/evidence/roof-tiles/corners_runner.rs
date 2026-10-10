#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] mod surface;
#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] mod tiles;
use std::io::{self,BufRead};fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let raw:[u8;24]=(0..48).step_by(2).map(|i|u8::from_str_radix(&l[i..i+2],16).unwrap()).collect::<Vec<_>>().try_into().unwrap();for point in tiles::ObservedRectangle::from_bytes(&raw).corners(){for x in point{for b in x.to_le_bytes(){print!("{b:02x}");}}}println!();}}
