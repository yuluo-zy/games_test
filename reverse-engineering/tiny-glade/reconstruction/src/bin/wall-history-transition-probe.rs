use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_authoring, building_wall_collection, building_wall_state, building_wall_freeform_cache, surface};
#[path = "../../building/wall_history_edit.rs"]
mod authority;
use authority as building_wall_history_edit;
use tiny_glade_reconstruction::building_history_transactions;
#[path = "../../building/wall_history_replay.rs"]
mod replay;
use replay as building_wall_history_replay;
#[path = "../../building/wall_history_transition.rs"]
mod transition;
use transition as building_wall_history_transition;
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
fn main() {
 for line in io::stdin().lock().lines() {
  let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
  let system=p[0]=="system";let p=if system{&p[1..]}else{&p[..]};
  let geom=bytes::<28>(p[8]);let tail=bytes::<56>(p[9]);let f=|at|f32::from_le_bytes(geom[at..at+4].try_into().unwrap());
  let shape=if geom[0]==0{building_wall_authoring::AuthoringShape::Circle(tiny_glade_reconstruction::building_wall_freeform_cache::PreviewCircleObserved{center:[f(4),f(8)],radius:f(12),angle:f(16)})}else{building_wall_authoring::AuthoringShape::Rectangle(tiny_glade_reconstruction::building_wall_freeform_cache::PreviewRectangleObserved{axes:[f(4),f(8)],center:[f(12),f(16)],dimensions:[f(20),f(24)]})};
  let f=|at|f32::from_le_bytes(tail[at..at+4].try_into().unwrap());let mut counter=0;let mut raw=building_wall_authoring::create_shape_source(&mut counter,building_wall_authoring::CreateShapeRequest{shape,previous_wall_hint:None,height:f(28),elevation:f(32),selected_style:tail[48]}).unwrap().source;
  raw.shape=building_wall_state::WallShapeSnapshot::from_observed(tail[..28].try_into().unwrap());raw.curve_kind_tag=raw.shape.tag();raw.observed_path_scalar=f(36);raw.observed_source_flag_64=tail[44];raw.special=tail[45]!=0;raw.clutter_version=tail[46];
  let full=WallAuthoritySnapshot{source:raw,metadata:authority::WallHistoryMetadata{source_60_bits:u32::from_le_bytes(tail[40..44].try_into().unwrap()),source_67:tail[47],source_69:tail[49]}};
  let wall=building_wall_state::WallId(p[10].parse().unwrap());let color=p[11].parse().unwrap();let mut record=authority::record_new_wall(wall,&full,color).unwrap();
  let oldextra=f32::from_bits(u32::from_str_radix(p[5],16).unwrap());let oldfloor=f32::from_bits(u32::from_str_radix(p[6],16).unwrap());
  if system {
   if p[2]=="00000001"{if let WallHistoryEdit::StateChange{wall,..}=&mut record.groups[1][0]{wall.0+=1000;}}
   let undo=p[0]=="1";let instant=p[1]=="1";let mut groups=replay::expand_wall_compound(&record,undo,instant).unwrap();let mut sources=std::collections::HashMap::new();if undo{sources.insert(wall,full);}
   let terrain=building_wall_collection::TerrainHeightGrid512{encoded:std::sync::Arc::from(vec![0;512*512]),minimum_y:0.75,maximum_y:0.75,sampling_fix:false};let mut steps=Vec::new();
   for dt in p[7].split(',') {let dt=f32::from_bits(u32::from_str_radix(dt,16).unwrap());let out=replay::advance_wall_groups(&mut groups,&mut sources,dt,&terrain).unwrap();let mut notifications=Vec::new();let mut audio=Vec::new();let mut colors=Vec::new();
    for effect in out.effects{for _ in effect.updated_source_notifications{notifications.push("updated");}if let Some(a)=effect.audio{let b:Vec<u8>=a.anchor.iter().flat_map(|f|f.to_le_bytes()).collect();audio.push(format!(r#"{{"wall":{},"anchor":"{}","kind":{},"flag":{}}}"#,a.wall.0,hex(&b),a.kind,a.flag));}if let Some(instant)=effect.instantaneous{if instant.new_source_notification.is_some(){notifications.push("new");}if instant.updated_source_notification.is_some(){notifications.push("updated");}if instant.removed_source_notification.is_some(){notifications.push("removed");}if let Some(c)=instant.color_request{colors.push(format!(r#"{{"wall":{},"color":{},"slot":{}}}"#,c.wall.0,c.color_id,if c.observed_special_owner{2}else{1}));}if let Some(a)=instant.audio{let b:Vec<u8>=a.anchor.iter().flat_map(|f|f.to_le_bytes()).collect();audio.push(format!(r#"{{"wall":{},"anchor":"{}","kind":{},"flag":{}}}"#,a.wall.0,hex(&b),a.kind,a.flag));}}}
    steps.push(format!(r#"{{"group_count":{},"source_count":{},"source":{},"notifications":{:?},"audio":[{}],"colors":[{}]}}"#,groups.len(),sources.len(),sources.get(&wall).map_or("null".into(),snapshot),notifications,audio.join(","),colors.join(",")));
   }println!("[{}]",steps.join(","));continue;
  }
  let e=&mut record.groups[1][0];if let WallHistoryEdit::StateChange{previous,..}=e{previous.source.extra=oldextra;previous.source.terrain_floor=oldfloor;}
  if let WallHistoryEdit::StateChange{previous,next,observed_operation,..}=e{let operation:u32=p[12].parse().unwrap();if operation==1{previous.source.curve=next.source.curve.clone();}observed_operation[..4].copy_from_slice(&operation.to_le_bytes());observed_operation[4..8].copy_from_slice(&u32::from_str_radix(p[13],16).unwrap().to_le_bytes());observed_operation[8..12].copy_from_slice(&u32::from_str_radix(p[14],16).unwrap().to_le_bytes());}
  let undo=p[0]=="1";let mut sources=std::collections::HashMap::new();if p[1]=="1"{let initial=match e{WallHistoryEdit::StateChange{previous,next,..}=>if undo{next.clone()}else{previous.clone()},_=>unreachable!()};sources.insert(wall,initial);}
  let mut command=replay::WallReplayCommand{edit:e.clone(),timed:true,duration:f32::from_bits(u32::from_str_radix(p[2],16).unwrap()),elapsed:f32::from_bits(u32::from_str_radix(p[3],16).unwrap()),undo,finished:p[4]=="1"};
  let terrain=building_wall_collection::TerrainHeightGrid512{encoded:std::sync::Arc::from(vec![0;512*512]),minimum_y:0.75,maximum_y:0.75,sampling_fix:false};let mut steps=Vec::new();
  for dt in p[7].split(','){let dt=f32::from_bits(u32::from_str_radix(dt,16).unwrap());match transition::advance_wall_transition(&mut command,&mut sources,dt,&terrain){Ok(out)=>{
   let stored=sources.get(&wall).map_or("null".into(),snapshot);
   let audio=out.audio.map_or("null".into(),|a|{let anchor:Vec<u8>=a.anchor.iter().flat_map(|f|f.to_le_bytes()).collect();format!(r#"{{"wall":{},"anchor":"{}","kind":{},"flag":{}}}"#,a.wall.0,hex(&anchor),a.kind,a.flag)});
   let height=out.height_request.map_or("null".into(),|h|format!(r#"{{"wall":{},"height":{},"flag":{}}}"#,h.wall_id,h.absolute_height.to_bits(),h.observed_audio_flag));
   steps.push(format!(r#"{{"elapsed":{},"finished":{},"source_count":{},"updated":{},"source":{},"audio":{},"height":{}}}"#,command.elapsed.to_bits(),command.finished,sources.len(),out.updated_source_notifications.len(),stored,audio,height));
  },Err(e)=>{steps.push(format!("\"error {e:?}\""));break;}}}
  println!("[{}]",steps.join(","));
 }
}
