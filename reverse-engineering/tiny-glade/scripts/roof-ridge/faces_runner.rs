mod roof {
    #[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] pub mod surface;
    #[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] pub mod tiles;
    #[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/rectangular_faces.rs"] pub mod rectangular_faces;
}
use std::io::{self,BufRead};
fn bytes(s:&str)->Vec<u8> {(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()}
fn fs(s:&str)->Vec<f32> {bytes(s).chunks_exact(4).map(|b|f32::from_le_bytes(b.try_into().unwrap())).collect()}
fn main(){for line in io::stdin().lock().lines(){
    let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
    if p[0]=="full" {
        let rect:[u8;24]=bytes(p[1]).try_into().unwrap();let roof:[u8;88]=bytes(p[2]).try_into().unwrap();
        let mut ordinal=p[4].parse().unwrap();let mut rng=roof::tiles::TileRng(p[5].parse().unwrap());
        let tiles=roof::rectangular_faces::assemble_faces_from_observed(&rect,&roof,p[3]=="1",&mut rng,&mut ordinal).unwrap();
        print!("{} {} ",rng.0,ordinal);for t in tiles {for b in t.0 {print!("{b:02x}");}}println!();continue;
    }
    let rows:Vec<[f32;6]>=fs(p[0]).chunks_exact(6).map(|r|r.try_into().unwrap()).collect();
    let points:Vec<[f32;2]>=fs(p[1]).chunks_exact(2).map(|r|r.try_into().unwrap()).collect();
    let curve=roof::surface::Curve2{points,points_u:fs(p[2]),length:fs(p[3])[0]};
    let planes=fs(p[6]);let mut rng=roof::tiles::TileRng(p[11].parse().unwrap());let mut ordinal=p[10].parse().unwrap();
    let input=roof::rectangular_faces::FaceRowsInput{rows:&rows,world_profile:&curve,row_count:fs(p[4])[0],rectangle:fs(p[5]).try_into().unwrap(),
        seam_planes:[planes[0..4].try_into().unwrap(),planes[4..8].try_into().unwrap()],roof_id:p[7].parse().unwrap(),special_mode:p[8]=="1",overhang:p[9]=="1"};
    let tiles=roof::rectangular_faces::assemble_rectangular_face_tiles(&input,&mut rng,&mut ordinal);
    print!("{} {} ",rng.0,ordinal);for t in tiles {for b in t.0 {print!("{b:02x}");}}println!();
}}
