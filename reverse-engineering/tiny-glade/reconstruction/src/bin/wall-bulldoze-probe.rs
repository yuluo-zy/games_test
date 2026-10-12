use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_authoring, building_wall_collection, building_wall_state, building_wall_freeform_cache, surface};
#[path = "../../building/wall_history_edit.rs"]
mod authority;
use authority as building_wall_history_edit;
use tiny_glade_reconstruction::building_history_transactions;
#[path="../../building/wall_bulldoze.rs"]
mod bulldoze;
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
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();let geom=bytes::<28>(p[1]);let tail=bytes::<56>(p[2]);let f=|at|f32::from_le_bytes(geom[at..at+4].try_into().unwrap());let shape=if geom[0]==0{building_wall_authoring::AuthoringShape::Circle(tiny_glade_reconstruction::building_wall_freeform_cache::PreviewCircleObserved{center:[f(4),f(8)],radius:f(12),angle:f(16)})}else{building_wall_authoring::AuthoringShape::Rectangle(tiny_glade_reconstruction::building_wall_freeform_cache::PreviewRectangleObserved{axes:[f(4),f(8)],center:[f(12),f(16)],dimensions:[f(20),f(24)]})};let f=|at|f32::from_le_bytes(tail[at..at+4].try_into().unwrap());let mut counter=0;let mut raw=building_wall_authoring::create_shape_source(&mut counter,building_wall_authoring::CreateShapeRequest{shape,previous_wall_hint:None,height:f(28),elevation:f(32),selected_style:tail[48]}).unwrap().source;raw.shape=building_wall_state::WallShapeSnapshot::from_observed(tail[..28].try_into().unwrap());raw.curve_kind_tag=raw.shape.tag();raw.observed_path_scalar=f(36);raw.observed_source_flag_64=tail[44];raw.special=tail[45]!=0;raw.clutter_version=tail[46];let full=WallAuthoritySnapshot{source:raw,metadata:authority::WallHistoryMetadata{source_60_bits:u32::from_le_bytes(tail[40..44].try_into().unwrap()),source_67:tail[47],source_69:tail[49]}};let wall=building_wall_state::WallId(p[3].parse().unwrap());let color=p[4].parse().unwrap();let low=f32::from_bits(u32::from_str_radix(p[5],16).unwrap());let high=f32::from_bits(u32::from_str_radix(p[6],16).unwrap());
if p[0]=="record"{let roof=if p[7]=="-"{None}else{Some(bytes::<88>(p[7]))};let r=bulldoze::record_wall_bulldoze(wall,&full,low,high,color,roof.as_ref()).unwrap();let mut owners=Vec::new();if let Some(roof)=r.roof{owners.push(format!(r#"{{"tag":8,"roof_payload":"{}"}}"#,hex(&roof.payload()[..56])));}owners.push(format!(r#"{{"tag":7,"groups":[{}]}}"#,r.wall.groups.iter().map(|g|format!("[{}]",g.iter().map(edit).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",")));println!("[{}]",owners.join(","));}
else{let anchor=bytes::<12>(p[7]);let anchor=std::array::from_fn(|i|f32::from_le_bytes(anchor[i*4..i*4+4].try_into().unwrap()));let mut c=bulldoze::WallBulldozeCommand{original:full.clone(),wall,low,high,anchor,phase:f32::from_bits(u32::from_str_radix(p[9],16).unwrap()),finished:false};let mut sources=std::collections::HashMap::new();if p[8]=="1"{sources.insert(wall,full);}let mut steps=Vec::new();for dt in p[10].split(','){let out=c.advance(&mut sources,f32::from_bits(u32::from_str_radix(dt,16).unwrap())).unwrap();let notifications=if out.updated.is_some(){vec!["updated"]}else if out.removed.is_some(){vec!["removed"]}else{vec![]};let audio=out.audio.map_or("null".into(),|a|{let anchor:Vec<u8>=a.anchor.iter().flat_map(|f|f.to_le_bytes()).collect();format!(r#"{{"wall":{},"anchor":"{}","kind":{},"flag":{}}}"#,a.wall.0,hex(&anchor),a.kind,a.flag)});steps.push(format!(r#"{{"source_count":{},"source":{},"phase":{},"finished":{},"buffer_count":{},"notifications":{:?},"audio":{}}}"#,sources.len(),sources.get(&wall).map_or("null".into(),snapshot),c.phase.to_bits(),c.finished,usize::from(!c.finished),notifications,audio));}println!("[{}]",steps.join(","));}
}}
