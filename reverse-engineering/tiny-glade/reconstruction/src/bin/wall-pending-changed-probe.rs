use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_authoring, building_wall_collection, building_wall_state, building_wall_freeform_cache, surface};
#[path = "../../building/wall_history_edit.rs"]
mod authority;
use authority as building_wall_history_edit;
use tiny_glade_reconstruction::building_history_transactions;
#[path="../../building/wall_pending_changed.rs"]
mod pending;
use authority::{WallHistoryEdit, WallAuthoritySnapshot};
fn bytes<const N: usize>(s: &str) -> [u8; N] { std::array::from_fn(|i| u8::from_str_radix(&s[i*2..i*2+2],16).unwrap()) }
fn hex(b: &[u8]) -> String { b.iter().map(|v|format!("{v:02x}")).collect() }
fn snapshot(s: &WallAuthoritySnapshot) -> String {
 let source=&s.source;let tag=source.shape.tag();let active=if tag==0 {20} else if tag==1 {28} else {4};
 let points:Vec<u8>=source.curve.points.iter().flatten().flat_map(|v|v.to_le_bytes()).collect();
 let us:Vec<u8>=source.curve.points_u.iter().flat_map(|v|v.to_le_bytes()).collect();
 let scalars:Vec<u8>=[source.extra.to_bits(),source.terrain_floor.to_bits(),source.observed_path_scalar.to_bits(),s.metadata.source_60_bits].iter().flat_map(|v|v.to_le_bytes()).collect();
 format!(r#"{{"curve":{{"points":"{}","us":"{}","length":{},"n":{}}},"enclosure":"{}","scalar_bits":"{}","metadata":"{}"}}"#,hex(&points),hex(&us),source.curve.length.to_bits(),source.curve.points.len(),hex(&source.shape.bytes()[..active]),hex(&scalars),hex(&[source.observed_source_flag_64,u8::from(source.special),source.clutter_version,s.metadata.source_67,source.style,s.metadata.source_69]))
}
fn edit(e:&WallHistoryEdit)->String {
 match e {
  WallHistoryEdit::Create{wall,source,color_id,..}=>format!(r#"{{"kind":"create","wall":{},"color":{},"source":{}}}"#,wall.0,color_id,snapshot(source)),
  WallHistoryEdit::Delete{wall,source,color_id}=>format!(r#"{{"kind":"delete","wall":{},"color":{},"source":{}}}"#,wall.0,color_id,snapshot(source)),
  WallHistoryEdit::EnclosureChange{wall,previous,next}=>format!(r#"{{"kind":"enclosure_change","wall":{},"previous":"{}","next":"{}"}}"#,wall.0,hex(previous.bytes()),hex(next.bytes())),
  WallHistoryEdit::StateChange{wall,previous,next,observed_operation}=>format!(r#"{{"kind":"state_change","wall":{},"previous":{},"next":{},"range":"{}"}}"#,wall.0,snapshot(previous),snapshot(next),hex(observed_operation)),
 }
}
fn curve(points:&str,us:&str,length:&str)->surface::Curve2{let p:Vec<u8>=(0..points.len()/2).map(|i|u8::from_str_radix(&points[i*2..i*2+2],16).unwrap()).collect();let u:Vec<u8>=(0..us.len()/2).map(|i|u8::from_str_radix(&us[i*2..i*2+2],16).unwrap()).collect();surface::Curve2{points:p.chunks_exact(8).map(|b|[f32::from_le_bytes(b[..4].try_into().unwrap()),f32::from_le_bytes(b[4..].try_into().unwrap())]).collect(),points_u:u.chunks_exact(4).map(|b|f32::from_le_bytes(b.try_into().unwrap())).collect(),length:f32::from_bits(u32::from_str_radix(length,16).unwrap())}}
fn source(curve:surface::Curve2)->WallAuthoritySnapshot{let mut counter=0;let mut s=building_wall_authoring::create_shape_source(&mut counter,building_wall_authoring::CreateShapeRequest{shape:building_wall_authoring::AuthoringShape::Rectangle(building_wall_freeform_cache::PreviewRectangleObserved{axes:[1.,0.],center:[0.,0.],dimensions:[2.,3.]}),previous_wall_hint:None,height:3.2,elevation:0.7,selected_style:1}).unwrap().source;let mut shape=[0;28];shape[..4].copy_from_slice(&2u32.to_le_bytes());s.shape=building_wall_state::WallShapeSnapshot::from_observed(shape);s.curve_kind_tag=2;s.curve=curve;s.special=true;s.clutter_version=2;s.observed_source_flag_64=197;WallAuthoritySnapshot{source:s,metadata:authority::WallHistoryMetadata{source_60_bits:0x12345678,source_67:31,source_69:5}}}
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();let old=source(curve(p[0],p[1],p[2]));let next=source(curve(p[3],p[4],p[5]));let mut item=pending::try_pending_wall_changed(building_wall_state::WallId(9),old,next,p[6]=="1").unwrap();
 let initial=item.as_ref().map_or("".into(),|p|format!(r#","previous":{},"next":{},"wall":{},"operation":"{}""#,snapshot(&p.previous),snapshot(&p.next),p.wall.0,hex(&p.observed_operation)));let present=item.is_some();let owner=pending::take_pending_wall_changed(&mut item);let groups=owner.as_ref().map_or("[]".into(),|o|format!("[{}]",o.groups.iter().map(|g|format!("[{}]",g.iter().map(edit).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",")));
 println!(r#"{{"present":{}{},"pending_cleared":{},"history_count":{},"groups":{}}}"#,present,initial,item.is_none(),usize::from(owner.is_some()),groups);
}}
