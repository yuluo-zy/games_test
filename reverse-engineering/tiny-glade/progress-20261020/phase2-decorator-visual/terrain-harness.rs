#[path="D:/game/tiny-glade/crates/systems/decorator/src/cache_terrain_support.rs"]
mod production;
use glam::Vec2;
fn main() {
    let text=std::fs::read_to_string("D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-decorator-visual/terrain-axis-native.txt").unwrap();
    for (line,row) in text.lines().enumerate() {
        let v:Vec<u32>=row.split_whitespace().map(|x|x.parse().unwrap()).collect();
        let axis=production::terrain_clutter_axis(glam::Quat::from_array([v[0],v[1],v[2],v[3]].map(f32::from_bits)));
        assert_eq!([axis.x.to_bits(),axis.y.to_bits()], [v[4],v[5]],"axis case {line}");
    }
    let text=std::fs::read_to_string("D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-decorator-visual/terrain-native.txt").unwrap();
    let mut count=0;
    for (line,row) in text.lines().enumerate() {
        let v:Vec<u32>=row.split_whitespace().map(|x|x.parse().unwrap()).collect();
        let f:Vec<f32>=v.iter().copied().map(f32::from_bits).collect();
        let mut coordinates=Vec::new();let mut i=0;
        let y=production::terrain_clutter_support_height(Vec2::new(f[0],f[1]),Vec2::new(f[2],f[3]),Vec2::new(f[4],f[5]),f[6],f[7],|p|{
            coordinates.extend([p.x.to_bits(),p.y.to_bits()]);let h=f[8+i];i+=1;h
        });
        // NaN payload propagation is an implementation detail; all finite and
        // infinite results, signed zero, and every sample coordinate are exact.
        assert!(y.to_bits()==v[12] || y.is_nan()&&f[12].is_nan(),"height case {line}: {:x} != {:x}",y.to_bits(),v[12]);
        assert_eq!(coordinates,&v[13..],"sample coordinates case {line}");count+=1;
    }
    println!("actual-source terrain support: {count}/{count}; coordinates bit exact; NaN payload excluded");
}
