#[path="D:/game/tiny-glade/crates/systems/decorator/src/cache_roof_orientation.rs"]
mod production;
use glam::{Vec2,Vec3};
fn main() {
    let text=std::fs::read_to_string("D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-decorator-visual/roof-yaw-selector-native.txt").unwrap();
    for row in text.lines() {
        let v:Vec<u32>=row.split_whitespace().map(|x|x.parse().unwrap()).collect();
        assert_eq!(production::roof_uses_yaw(v[0] as u8),v[1]!=0);
    }
    let text=std::fs::read_to_string("D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-decorator-visual/roof-product-native.txt").unwrap();
    for (line,row) in text.lines().enumerate() {
        let v:Vec<u32>=row.split_whitespace().map(|x|x.parse().unwrap()).collect();
        let q=|slice:&[u32]| glam::Quat::from_array([slice[0],slice[1],slice[2],slice[3]].map(f32::from_bits));
        assert_eq!(production::compose_roof_orientation(q(&v[0..4]),q(&v[4..8])).to_array().map(f32::to_bits),v[8..12],"product {line}");
    }
    std::panic::set_hook(Box::new(|_|{}));
    let text=std::fs::read_to_string("D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-decorator-visual/roof-normal-native.txt").unwrap();
    let mut count=0;
    for (line,row) in text.lines().enumerate() {
        let v:Vec<u32>=row.split_whitespace().map(|x|x.parse().unwrap()).collect();
        let f:Vec<f32>=v.iter().copied().map(f32::from_bits).collect();
        let mut query=Vec::new();
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| production::roof_decorator_normal(v[0] as u8,
            Vec3::new(f[2],f[3],f[4]), v[1]!=0, Vec2::new(f[5],f[6]), |p,xz| {
                query.extend([p.x.to_bits(),p.y.to_bits(),p.z.to_bits(),xz.x.to_bits(),xz.y.to_bits()]);
                Vec2::new(f[7],f[8])
            })));
        assert_eq!(result.is_err(),v[9]!=0,"panic case {line}");
        if let Ok(normal)=result {
            for (actual,expected) in [normal.x,normal.y].into_iter().zip(&f[10..12]) {
                assert!(actual.to_bits()==expected.to_bits() || actual.is_nan()&&expected.is_nan(),"normal case {line}");
            }
            if v[1]!=0 {assert_eq!(query,&v[12..],"query case {line}");}
            else {assert!(query.is_empty());}
        }
        count+=1;
    }
    println!("actual-source roof normals: {count}/{count}, including exact query coordinates/panic gates; NaN payload excluded");
}
