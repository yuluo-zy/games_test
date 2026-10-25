use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_authoring, building_wall_collection, building_wall_state, building_wall_freeform_cache, surface};
#[path = "../../building/wall_history_edit.rs"]
mod authority;
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
fn main(){
 for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
  let args = if p[0]=="instant" { &p[5..] } else { &p[1..] };
  let geom=bytes::<28>(args[0]);let tail=bytes::<56>(args[1]);let f=|at|f32::from_le_bytes(geom[at..at+4].try_into().unwrap());
  let shape=if geom[0]==0{building_wall_authoring::AuthoringShape::Circle(tiny_glade_reconstruction::building_wall_freeform_cache::PreviewCircleObserved{center:[f(4),f(8)],radius:f(12),angle:f(16)})}else{building_wall_authoring::AuthoringShape::Rectangle(tiny_glade_reconstruction::building_wall_freeform_cache::PreviewRectangleObserved{axes:[f(4),f(8)],center:[f(12),f(16)],dimensions:[f(20),f(24)]})};
  let f=|at|f32::from_le_bytes(tail[at..at+4].try_into().unwrap());let mut counter=0;let mut source=building_wall_authoring::create_shape_source(&mut counter,building_wall_authoring::CreateShapeRequest{shape,previous_wall_hint:None,height:f(28),elevation:f(32),selected_style:tail[48]}).unwrap().source;
  source.shape=building_wall_state::WallShapeSnapshot::from_observed(tail[..28].try_into().unwrap());source.curve_kind_tag=source.shape.tag();source.observed_path_scalar=f(36);source.observed_source_flag_64=tail[44];source.special=tail[45]!=0;source.clutter_version=tail[46];
  let source=WallAuthoritySnapshot{source,metadata:authority::WallHistoryMetadata{source_60_bits:u32::from_le_bytes(tail[40..44].try_into().unwrap()),source_67:tail[47],source_69:tail[49]}};
  let wall=building_wall_state::WallId(args[2].parse().unwrap());let color=args[3].parse().unwrap();
  let record = if p[0]=="delete" || (p[0]=="instant" && p[1]=="delete") { Ok(authority::record_deleted_wall(wall,&source,color)) } else { authority::record_new_wall(wall,&source,color) };
  match record {
   Ok(record)=>{
    if p[0]!="instant" {
     println!("[{}]",record.groups.iter().map(|g|format!("[{}]",g.iter().map(edit).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(","));
    } else {
     let mut sources=std::collections::HashMap::new();if p[4]=="1" { sources.insert(wall,source); }
     let terrain=building_wall_collection::TerrainHeightGrid512 { encoded:std::sync::Arc::from(vec![0;512*512]),minimum_y:0.75,maximum_y:0.75,sampling_fix:false };
     let e=&record.groups[p[2].parse::<usize>().unwrap()][0];
     match authority::apply_instant_wall_edit(&mut sources,e,p[3]=="1",&terrain) {
      Ok(out)=>{
       let stored=sources.get(&wall).map_or("null".into(),snapshot);
       let color=out.color_request.map_or("null".into(),|c|format!(r#"{{"wall":{},"color":{},"slot":{}}}"#,c.wall.0,c.color_id,if c.observed_special_owner{2}else{1}));
       let audio=out.audio.map_or("null".into(),|a|{let anchor:Vec<u8>=a.anchor.iter().flat_map(|f|f.to_le_bytes()).collect();format!(r#"{{"wall":{},"anchor":"{}","kind":{},"flag":{}}}"#,a.wall.0,hex(&anchor),a.kind,a.flag)});
       println!(r#"{{"source_count":{},"new":{},"updated":{},"removed":{},"source":{},"color":{},"audio":{}}}"#,sources.len(),out.new_source_notification.is_some(),out.updated_source_notification.is_some(),out.removed_source_notification.is_some(),stored,color,audio);
      },Err(e)=>println!("error {e:?}"),
     }
    }
   },
   Err(e)=>println!("error {e:?}"),
  }
 }
}
