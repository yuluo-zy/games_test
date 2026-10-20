use std::sync::Arc;use glam::{Vec2,Vec3};use utils::geometry::Ray;use systems::ui::{hover_intersection::{pick_chunk,PickerInput},resources::{widget_params::{WidgetParams,WidgetGeometry},ui_state::WidgetId}};
pub use country_core::resources;mod systems {pub use country_core::systems::camera_rig;pub mod ui {pub mod resources {#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/widget_params.rs"]pub mod widget_params;#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/ui_state.rs"]pub mod ui_state;}#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/ui-picker/proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs"]pub mod hover_intersection;}}
#[test]fn unchanged_original_picker_fixtures(){
{let widgets=vec![(WidgetId(1),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case0 sphere ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case0 sphere ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case0 sphere canvas count");
}
{let widgets=vec![(WidgetId(2),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::SphereCloud{centers:vec![Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1084227584)),Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(1092616192),f32::from_bits(0),f32::from_bits(1065353216))],radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case1 sphere-cloud ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,0,0,0,0,0,0,128,63,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case1 sphere-cloud ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case1 sphere-cloud ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case1 sphere-cloud ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case1 sphere-cloud ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case1 sphere-cloud canvas count");
}
{let widgets=vec![(WidgetId(3),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::TriangleMesh(vec![utils::geometry::intersect::Triangle::new(Vec3::new(f32::from_bits(3212836864),f32::from_bits(3212836864),f32::from_bits(1084227584)),Vec3::new(f32::from_bits(1065353216),f32::from_bits(3212836864),f32::from_bits(1084227584)),Vec3::new(f32::from_bits(0),f32::from_bits(1065353216),f32::from_bits(1084227584))),utils::geometry::intersect::Triangle::new(Vec3::new(f32::from_bits(3212836864),f32::from_bits(3212836864),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(1065353216),f32::from_bits(3212836864),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(0),f32::from_bits(1065353216),f32::from_bits(1073741824)))]));p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case2 triangles ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,128,0,0,128,63,3,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case2 triangles ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case2 triangles canvas count");
}
{let widgets=vec![(WidgetId(4),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::ScreenAabb{min:Vec2::new(f32::from_bits(3212836864),f32::from_bits(3212836864)),max:Vec2::new(f32::from_bits(1065353216),f32::from_bits(1065353216)),distance:f32::from_bits(1048576000)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case3 screen-aabb ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,62,0,0,0,0,0,0,0,0,0,0,128,62,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case3 screen-aabb ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case3 screen-aabb ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case3 screen-aabb ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case3 screen-aabb ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case3 screen-aabb canvas count");
}
{let widgets=vec![(WidgetId(5),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::WorldCanvas{extent:Vec2::new(f32::from_bits(1092616192),f32::from_bits(1101004800)),layer:2});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:Some(Vec3::new(f32::from_bits(1065353216),f32::from_bits(1091567616),f32::from_bits(1073741824)))};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case4 world-canvas ordinary count");
assert_eq!(b.len(),1,"case4 world-canvas canvas count");
{let actual=b[0].1.to_native();let expected=[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,128,63,0,0,16,65,0,0,0,64,0,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case4 world-canvas canvas off12: {x} vs {y}");}
}
}
{let widgets=vec![(WidgetId(6),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::TerrainCanvas{distance:f32::from_bits(1120272384),layer:3});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case5 terrain-canvas-no-terrain ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,198,66,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case5 terrain-canvas-no-terrain ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case5 terrain-canvas-no-terrain ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case5 terrain-canvas-no-terrain ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case5 terrain-canvas-no-terrain ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case5 terrain-canvas-no-terrain canvas count");
}
{let widgets=vec![(WidgetId(6),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::TerrainCanvas{distance:f32::from_bits(1120272384),layer:3});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:Some(Vec3::new(f32::from_bits(1065353216),f32::from_bits(1091567616),f32::from_bits(1073741824)))};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case6 terrain-canvas ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,198,66,0,0,128,63,0,0,16,65,0,0,0,64,0,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case6 terrain-canvas ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case6 terrain-canvas ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case6 terrain-canvas ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case6 terrain-canvas ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case6 terrain-canvas canvas count");
}
{let widgets=vec![(WidgetId(7),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case7 plane-disc ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,0,0,0,0,0,0,0,0,64,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case7 plane-disc ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case7 plane-disc canvas count");
}
{let widgets=vec![(WidgetId(8),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Empty);p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case8 empty ordinary count");
assert_eq!(b.len(),0,"case8 empty canvas count");
}
{let widgets=vec![(WidgetId(11),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3225419776)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case9 sphere-z-3 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,11,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,192,0,0,0,0,0,0,0,0,0,0,128,192,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case9 sphere-z-3 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case9 sphere-z-3 canvas count");
}
{let widgets=vec![(WidgetId(11),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case10 sphere-z-1 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,11,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,192,0,0,0,0,0,0,0,0,0,0,0,192,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case10 sphere-z-1 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case10 sphere-z-1 canvas count");
}
{let widgets=vec![(WidgetId(11),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case11 sphere-z0 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,11,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,0,0,0,0,0,0,0,0,0,0,128,191,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case11 sphere-z0 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case11 sphere-z0 canvas count");
}
{let widgets=vec![(WidgetId(11),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case12 sphere-z1 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,11,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case12 sphere-z1 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case12 sphere-z1 canvas count");
}
{let widgets=vec![(WidgetId(11),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case13 sphere-z3 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,11,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case13 sphere-z3 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case13 sphere-z3 canvas count");
}
{let widgets=vec![(WidgetId(11),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case14 sphere-zero-direction ordinary count");
assert_eq!(b.len(),0,"case14 sphere-zero-direction canvas count");
}
{let widgets=vec![(WidgetId(12),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::SphereCloud{centers:vec![],radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case15 sphere-cloud-empty ordinary count");
assert_eq!(b.len(),0,"case15 sphere-cloud-empty canvas count");
}
{let widgets=vec![(WidgetId(12),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::SphereCloud{centers:vec![Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128))],radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case16 sphere-cloud-tie ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case16 sphere-cloud-tie ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case16 sphere-cloud-tie ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case16 sphere-cloud-tie ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case16 sphere-cloud-tie ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case16 sphere-cloud-tie canvas count");
}
{let widgets=vec![(WidgetId(12),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::SphereCloud{centers:vec![Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3225419776)),Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128))],radius:f32::from_bits(1065353216)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case17 sphere-cloud-behind ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,192,0,0,0,0,0,0,0,0,0,0,128,192,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case17 sphere-cloud-behind ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case17 sphere-cloud-behind ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case17 sphere-cloud-behind ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case17 sphere-cloud-behind ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case17 sphere-cloud-behind canvas count");
}
{let widgets=vec![(WidgetId(13),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::TriangleMesh(vec![]));p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case18 triangles-empty ordinary count");
assert_eq!(b.len(),0,"case18 triangles-empty canvas count");
}
{let widgets=vec![(WidgetId(13),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::TriangleMesh(vec![utils::geometry::intersect::Triangle::new(Vec3::new(f32::from_bits(3212836864),f32::from_bits(3212836864),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(1065353216),f32::from_bits(3212836864),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(0),f32::from_bits(1065353216),f32::from_bits(1073741824))),utils::geometry::intersect::Triangle::new(Vec3::new(f32::from_bits(3212836864),f32::from_bits(3212836864),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(1065353216),f32::from_bits(3212836864),f32::from_bits(1073741824)),Vec3::new(f32::from_bits(0),f32::from_bits(1065353216),f32::from_bits(1073741824)))]));p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case19 triangles-tie ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,128,0,0,128,63,13,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case19 triangles-tie ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case19 triangles-tie canvas count");
}
{let widgets=vec![(WidgetId(14),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::ScreenAabb{min:Vec2::new(f32::from_bits(3212836864),f32::from_bits(3212836864)),max:Vec2::new(f32::from_bits(1065353216),f32::from_bits(1065353216)),distance:f32::from_bits(1048576000)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(3212845253),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case20 screen-edge-1.001 ordinary count");
assert_eq!(b.len(),0,"case20 screen-edge-1.001 canvas count");
}
{let widgets=vec![(WidgetId(14),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::ScreenAabb{min:Vec2::new(f32::from_bits(3212836864),f32::from_bits(3212836864)),max:Vec2::new(f32::from_bits(1065353216),f32::from_bits(1065353216)),distance:f32::from_bits(1048576000)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(3212836864),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case21 screen-edge-1 ordinary count");
assert_eq!(b.len(),0,"case21 screen-edge-1 canvas count");
}
{let widgets=vec![(WidgetId(14),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::ScreenAabb{min:Vec2::new(f32::from_bits(3212836864),f32::from_bits(3212836864)),max:Vec2::new(f32::from_bits(1065353216),f32::from_bits(1065353216)),distance:f32::from_bits(1048576000)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case22 screen-edge0 ordinary count");
{let actual=a[0].to_native();let expected=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,14,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,62,0,0,0,0,0,0,0,0,0,0,128,62,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case22 screen-edge0 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case22 screen-edge0 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case22 screen-edge0 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case22 screen-edge0 ordinary off44: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case22 screen-edge0 canvas count");
}
{let widgets=vec![(WidgetId(14),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::ScreenAabb{min:Vec2::new(f32::from_bits(3212836864),f32::from_bits(3212836864)),max:Vec2::new(f32::from_bits(1065353216),f32::from_bits(1065353216)),distance:f32::from_bits(1048576000)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(1065353216),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case23 screen-edge1 ordinary count");
assert_eq!(b.len(),0,"case23 screen-edge1 canvas count");
}
{let widgets=vec![(WidgetId(14),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::ScreenAabb{min:Vec2::new(f32::from_bits(3212836864),f32::from_bits(3212836864)),max:Vec2::new(f32::from_bits(1065353216),f32::from_bits(1065353216)),distance:f32::from_bits(1048576000)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(1065361605),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case24 screen-edge1.001 ordinary count");
assert_eq!(b.len(),0,"case24 screen-edge1.001 canvas count");
}
{let widgets=vec![(WidgetId(15),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::WorldCanvas{extent:Vec2::new(f32::from_bits(1092616192),f32::from_bits(1101004800)),layer:2});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case25 canvas-edgeNone ordinary count");
assert_eq!(b.len(),1,"case25 canvas-edgeNone canvas count");
{let actual=b[0].1.to_native();let expected=[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case25 canvas-edgeNone canvas off12: {x} vs {y}");}
}
}
{let widgets=vec![(WidgetId(15),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::WorldCanvas{extent:Vec2::new(f32::from_bits(1092616192),f32::from_bits(1101004800)),layer:2});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:Some(Vec3::new(f32::from_bits(1084227584),f32::from_bits(1082130432),f32::from_bits(1092616192)))};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case26 canvas-edge(5, 4, 10) ordinary count");
assert_eq!(b.len(),1,"case26 canvas-edge(5, 4, 10) canvas count");
{let actual=b[0].1.to_native();let expected=[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,128,64,0,0,32,65,0,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case26 canvas-edge(5, 4, 10) canvas off12: {x} vs {y}");}
}
}
{let widgets=vec![(WidgetId(15),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::WorldCanvas{extent:Vec2::new(f32::from_bits(1092616192),f32::from_bits(1101004800)),layer:2});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:Some(Vec3::new(f32::from_bits(1084229681),f32::from_bits(1082130432),f32::from_bits(1092616192)))};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case27 canvas-edge(5.001, 4, 10) ordinary count");
assert_eq!(b.len(),0,"case27 canvas-edge(5.001, 4, 10) canvas count");
}
{let widgets=vec![(WidgetId(15),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::WorldCanvas{extent:Vec2::new(f32::from_bits(1092616192),f32::from_bits(1101004800)),layer:2});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:Some(Vec3::new(f32::from_bits(1084227584),f32::from_bits(1082130432),f32::from_bits(1092617241)))};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case28 canvas-edge(5, 4, 10.001) ordinary count");
assert_eq!(b.len(),0,"case28 canvas-edge(5, 4, 10.001) canvas count");
}
{let widgets=vec![(WidgetId(15),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::WorldCanvas{extent:Vec2::new(f32::from_bits(1092616192),f32::from_bits(1101004800)),layer:2});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:Some(Vec3::new(f32::from_bits(3231711232),f32::from_bits(1082130432),f32::from_bits(3240099840)))};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case29 canvas-edge(-5, 4, -10) ordinary count");
assert_eq!(b.len(),1,"case29 canvas-edge(-5, 4, -10) canvas count");
{let actual=b[0].1.to_native();let expected=[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,192,0,0,128,64,0,0,32,193,0,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case29 canvas-edge(-5, 4, -10) canvas off12: {x} vs {y}");}
}
}
{let widgets=vec![(WidgetId(16),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case30 plane-edge0 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,0,0,0,0,0,0,0,0,64,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case30 plane-edge0 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case30 plane-edge0 canvas count");
}
{let widgets=vec![(WidgetId(16),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(1073733435),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case31 plane-edge1.999 ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,59,223,255,63,0,0,0,0,0,0,64,64,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case31 plane-edge1.999 ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case31 plane-edge1.999 canvas count");
}
{let widgets=vec![(WidgetId(16),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(1073741824),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case32 plane-edge2 ordinary count");
assert_eq!(b.len(),0,"case32 plane-edge2 canvas count");
}
{let widgets=vec![(WidgetId(16),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(1073746018),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case33 plane-edge2.001 ordinary count");
assert_eq!(b.len(),0,"case33 plane-edge2.001 canvas count");
}
{let widgets=vec![(WidgetId(16),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(1065353216),f32::from_bits(0),f32::from_bits(0))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case34 plane-parallel ordinary count");
assert_eq!(b.len(),0,"case34 plane-parallel canvas count");
}
{let widgets=vec![(WidgetId(16),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::PlaneDisc{point:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3225419776)),normal:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),0,"case35 plane-behind ordinary count");
assert_eq!(b.len(),0,"case35 plane-behind canvas count");
}
{let widgets=vec![(WidgetId(17),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1073741824))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case36 sphere-direction(0, 0, 2) ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,0,0,0,0,0,0,0,0,0,0,128,191,17,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,0,0,0,0,0,0,0,128,63,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case36 sphere-direction(0, 0, 2) ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case36 sphere-direction(0, 0, 2) canvas count");
}
{let widgets=vec![(WidgetId(17),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(1036831949),f32::from_bits(1045220557),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case37 sphere-direction(0.1, 0.2, 1) ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,77,111,79,61,77,111,207,61,112,90,126,191,17,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,144,165,129,63,77,111,207,61,77,111,79,62,144,165,129,63,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case37 sphere-direction(0.1, 0.2, 1) ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case37 sphere-direction(0.1, 0.2, 1) canvas count");
}
{let widgets=vec![(WidgetId(17),Arc::new({let mut p=WidgetParams::default().geometry(WidgetGeometry::Sphere{center:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1077936128)),radius:f32::from_bits(1073741824)});p.occlude_by_world=true;p.flags.0=1;p}))];
let input=PickerInput{ray:Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(3197737370),f32::from_bits(1036831949),f32::from_bits(1065353216))},screen:Vec2::new(f32::from_bits(0),f32::from_bits(0)),terrain:None};
let(a,b)=pick_chunk(&widgets,input);
assert_eq!(a.len(),1,"case38 sphere-direction(-0.3, 0.1, 1) ordinary count");
{let actual=a[0].to_native();let expected=[1,0,0,0,98,172,29,190,45,59,82,61,4,155,124,191,17,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,252,100,131,63,98,172,157,190,45,59,210,61,252,100,131,63,1,1,0,0,0,0,0,0];
assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);
{let x=f32::from_le_bytes(actual[32..36].try_into().unwrap());let y=f32::from_le_bytes(expected[32..36].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off32: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[36..40].try_into().unwrap());let y=f32::from_le_bytes(expected[36..40].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off36: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[40..44].try_into().unwrap());let y=f32::from_le_bytes(expected[40..44].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off40: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[44..48].try_into().unwrap());let y=f32::from_le_bytes(expected[44..48].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off44: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[4..8].try_into().unwrap());let y=f32::from_le_bytes(expected[4..8].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off4: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[8..12].try_into().unwrap());let y=f32::from_le_bytes(expected[8..12].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off8: {x} vs {y}");}
{let x=f32::from_le_bytes(actual[12..16].try_into().unwrap());let y=f32::from_le_bytes(expected[12..16].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case38 sphere-direction(-0.3, 0.1, 1) ordinary off12: {x} vs {y}");}
}
assert_eq!(b.len(),0,"case38 sphere-direction(-0.3, 0.1, 1) canvas count");
}
}
#[test]fn unchanged_original_arbitration_fixtures(){
use systems::ui::hover_intersection::{arbitrate,WidgetIntersection};
fn decode(b:&[u8])->WidgetIntersection{let f=|i:usize| f32::from_le_bytes(b[i..i+4].try_into().unwrap());WidgetIntersection{normal:(u32::from_le_bytes(b[..4].try_into().unwrap())!=0).then(||Vec3::new(f(4),f(8),f(12))),widget:WidgetId(u64::from_le_bytes(b[16..24].try_into().unwrap())),element:u64::from_le_bytes(b[24..32].try_into().unwrap())as usize,distance:f(32),position:Vec3::new(f(36),f(40),f(44)),occlude_by_world:b[48]!=0,flags:b[49]}}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(3221225472));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case0 depth-2-occFalse");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(3221225472));
let expected=vec![];
assert_eq!(actual,expected,"arbitration case1 depth-2-occTrue");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(3212836864));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case2 depth-1-occFalse");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(3212836864));
let expected=vec![];
assert_eq!(actual,expected,"arbitration case3 depth-1-occTrue");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(0));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case4 depth0-occFalse");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(0));
let expected=vec![];
assert_eq!(actual,expected,"arbitration case5 depth0-occTrue");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(1065353216));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case6 depth1-occFalse");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(1065353216));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case7 depth1-occTrue");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(1073741824));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case8 depth2-occFalse");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(1073741824));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case9 depth2-occTrue");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(1077936128));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case10 depth3-occFalse");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(3212836864))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(1077936128));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,64,0,0,160,64,0,0,192,64,0,0,224,64,1,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case11 depth3-occTrue");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,192,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(0));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case12 negative1");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,192,0,0,160,64,0,0,192,64,0,0,224,64,0,5,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(0));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,192,0,0,160,64,0,0,192,64,0,0,224,64,0,5,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case13 negative5");
}
{let ordinary=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
let canvases=vec![(4,decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])),(1,decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])),(4,decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]))];
let ray=Ray{origin:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(0)),direction:Vec3::new(f32::from_bits(0),f32::from_bits(0),f32::from_bits(1065353216))};
let actual=arbitrate(ordinary,canvases,ray,glam::Mat4::IDENTITY,f32::from_bits(0));
let expected=vec![decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,63,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0]),decode(&[1,0,0,0,0,0,0,0,0,0,128,63,0,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,255,127,127,0,0,160,64,0,0,192,64,0,0,224,64,0,1,0,0,0,0,0,0])];
assert_eq!(actual,expected,"arbitration case14 stable-layers-and-distance");
}
}