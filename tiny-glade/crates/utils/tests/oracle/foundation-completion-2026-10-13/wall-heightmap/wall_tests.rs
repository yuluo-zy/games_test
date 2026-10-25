use super::*;
fn f(b:u32)->f32{f32::from_bits(b)}
fn equal(a:f32,b:u32){if f(b).is_nan(){assert!(a.is_nan());}else{assert_eq!(a.to_bits(),b);}}
#[test]fn original_wall_projection_wrapper_policies(){
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3245342720),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3245342720);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3236954112),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3236954112);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3212836864),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3212836864);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2147483648),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(0),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1056964608),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1065353216),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,1065353216);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1089470464),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,1089470464);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1090519040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,1090519040);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1097859072),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,1097859072);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2143289344),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:false,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2139095040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,2139095040);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3245342720),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3245342720);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3236954112),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3236954112);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3212836864),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3212836864);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2147483648),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(0),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1056964608),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1065353216),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1065353216);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,1065353216);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1089470464),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,1089470464);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1090519040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,1090519040);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1097859072),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,1097859072);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2143289344),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:false,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2139095040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,2139095040);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3245342720),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3245342720);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3236954112),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3236954112);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3212836864),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,3212836864);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2147483648),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(0),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1056964608),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1065353216),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1065353216);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,1065353216);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1089470464),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1089470464);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1089470464);equal(WallSpaceCoord::new(&space,d.x,2.).u,1089470464);equal(c.with_offset_unbounded(&space,d).u,1089470464);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1090519040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1089470464);equal(WallSpaceCoord::new(&space,d.x,2.).u,1089470464);equal(c.with_offset_unbounded(&space,d).u,1090519040);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1097859072),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1089470464);equal(WallSpaceCoord::new(&space,d.x,2.).u,1089470464);equal(c.with_offset_unbounded(&space,d).u,1097859072);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2143289344),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:false,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2139095040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),false);
equal(c.with_offset_clamped(&space,d).u,1089470464);equal(WallSpaceCoord::new(&space,d.x,2.).u,1089470464);equal(c.with_offset_unbounded(&space,d).u,2139095040);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3245342720),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3236954112),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3212836864),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2147483648),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(0),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1056964608),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1065353216),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1089470464),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1090519040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1097859072),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2143289344),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(0)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2139095040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3245342720),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2147483648);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2147483648);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,2147483648);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3236954112),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3212836864),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2147483648);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2147483648);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,2147483648);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2147483648),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(0),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1056964608),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1065353216),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1089470464),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1090519040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1097859072),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2143289344),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(1065353216)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2139095040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3245342720),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2147483648);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2147483648);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,2147483648);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3236954112),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2147483648);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2147483648);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,2147483648);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(3212836864),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1087373312);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1087373312);equal(WallSpaceCoord::new(&space,d.x,2.).u,1087373312);equal(c.with_offset_unbounded(&space,d).u,1087373312);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2147483648),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,2147483648);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(0),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1056964608),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1065353216),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1065353216);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1065353216);equal(WallSpaceCoord::new(&space,d.x,2.).u,1065353216);equal(c.with_offset_unbounded(&space,d).u,1065353216);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1089470464),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1090519040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,1056964608);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,1056964608);equal(WallSpaceCoord::new(&space,d.x,2.).u,1056964608);equal(c.with_offset_unbounded(&space,d).u,1056964608);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(1097859072),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,0);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,0);equal(WallSpaceCoord::new(&space,d.x,2.).u,0);equal(c.with_offset_unbounded(&space,d).u,0);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2143289344),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
{let space=WallSpace{is_loop:true,length:f(1089470464)};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f(2139095040),3.);
let got=c.with_offset(&space,d);assert_eq!(got.is_some(),true);
equal(got.unwrap().u,2143289344);assert_eq!(got.unwrap().y,5.);
equal(c.with_offset_clamped(&space,d).u,2143289344);equal(WallSpaceCoord::new(&space,d.x,2.).u,2143289344);equal(c.with_offset_unbounded(&space,d).u,2143289344);}
}
