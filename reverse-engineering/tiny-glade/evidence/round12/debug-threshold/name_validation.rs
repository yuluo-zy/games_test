use glam::Vec3;
#[derive(Clone,Copy)] enum DebugColor {White,Red,Blue,Green,Grey,Custom(Vec3)}
struct DebugSphere {color:DebugColor,radius:f32,center:Vec3,alpha:f32,solid:bool}
impl DebugSphere {
    fn signifier_name(&self) -> String {
        let color = match self.color {
            DebugColor::White => "White".to_owned(),
            DebugColor::Red => "Red".to_owned(),
            DebugColor::Blue => "Blue".to_owned(),
            DebugColor::Green => "Green".to_owned(),
            DebugColor::Grey => "Grey".to_owned(),
            DebugColor::Custom(rgb) => format!("Custom({:?})", rgb.to_array()),
        };
        format!(
            "DebugSphere {{ color: {}, radius: {:?}, position: {:?}, line_width: {:?}, filled_in: {:?} }}",
            color, self.radius, self.center, self.alpha, self.solid
        )
    }
}
fn main(){
assert_eq!(DebugSphere{color:DebugColor::White,radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),"DebugSphere { color: White, radius: 0.1, position: Vec3(1.0, 2.0, 3.0), line_width: 1.0, filled_in: false }");
assert_eq!(DebugSphere{color:DebugColor::Red,radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),"DebugSphere { color: Red, radius: 0.1, position: Vec3(1.0, 2.0, 3.0), line_width: 1.0, filled_in: false }");
assert_eq!(DebugSphere{color:DebugColor::Blue,radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),"DebugSphere { color: Blue, radius: 0.1, position: Vec3(1.0, 2.0, 3.0), line_width: 1.0, filled_in: false }");
assert_eq!(DebugSphere{color:DebugColor::Green,radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),"DebugSphere { color: Green, radius: 0.1, position: Vec3(1.0, 2.0, 3.0), line_width: 1.0, filled_in: false }");
assert_eq!(DebugSphere{color:DebugColor::Grey,radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),"DebugSphere { color: Grey, radius: 0.1, position: Vec3(1.0, 2.0, 3.0), line_width: 1.0, filled_in: false }");
assert_eq!(DebugSphere{color:DebugColor::Custom(Vec3::new(0.2,0.4,0.6)),radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),"DebugSphere { color: Custom([0.2, 0.4, 0.6]), radius: 0.1, position: Vec3(1.0, 2.0, 3.0), line_width: 1.0, filled_in: false }");
println!("PASS: exact Rust production Sphere identity method; all6 original complete-show Debug names match");}
