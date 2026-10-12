use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_authoring, building_wall_collection, building_wall_state, building_wall_freeform_cache, surface, building_history_transactions};
#[path = "../../building/wall_history_edit.rs"] mod authority;
use authority as building_wall_history_edit;
#[path = "../../building/wall_rectangle_edit.rs"] mod finish;
#[path = "../../building/wall_history_replay.rs"] mod replay;
use replay as building_wall_history_replay;
#[path = "../../building/wall_history_transition.rs"] mod transition;
use transition as building_wall_history_transition;
use authority::{WallHistoryEdit,WallAuthoritySnapshot};
fn bytes<const N: usize>(s: &str)->[u8;N]{std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn hex(b:&[u8])->String{b.iter().map(|v|format!("{v:02x}")).collect()}
fn snapshot(s:&WallAuthoritySnapshot)->String{
 let raw=&s.source;let points:Vec<u8>=raw.curve.points.iter().flatten().flat_map(|f|f.to_le_bytes()).collect();let us:Vec<u8>=raw.curve.points_u.iter().flat_map(|f|f.to_le_bytes()).collect();
 let scalars:Vec<u8>=[raw.extra.to_bits(),raw.terrain_floor.to_bits(),raw.observed_path_scalar.to_bits(),s.metadata.source_60_bits].into_iter().flat_map(|f|f.to_le_bytes()).collect();
 format!(r#"{{"curve":{{"points":"{}","us":"{}","length":{},"n":{}}},"enclosure":"{}","scalar_bits":"{}","metadata":"{}"}}"#,hex(&points),hex(&us),raw.curve.length.to_bits(),raw.curve.points.len(),hex(raw.shape.bytes()),hex(&scalars),hex(&[raw.observed_source_flag_64,raw.special as u8,raw.clutter_version,s.metadata.source_67,raw.style,s.metadata.source_69]))
}
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
 if p[0]=="finish" {
  let mut state=finish::RectangleEditState(bytes(p[1]));let shape=building_wall_state::WallShapeSnapshot::from_observed(bytes(p[2]));
  let mut counter=0;let raw=building_wall_authoring::create_shape_source(&mut counter,building_wall_authoring::CreateShapeRequest{shape:building_wall_authoring::AuthoringShape::Rectangle(authority::rectangle_from_enclosure(&shape).unwrap()),previous_wall_hint:None,height:3.2,elevation:0.7,selected_style:1}).unwrap().source;
  let out=finish::finish_rectangle_edit_decision(&mut state,p[3]=="1",Some(&raw)).unwrap();
  let history=out.history.as_ref().map_or("null".into(),|owner|{let WallHistoryEdit::EnclosureChange{wall,previous,next}=&owner.groups[0][0]else{unreachable!()};format!(r#"{{"wall":{},"previous":"{}","next":"{}"}}"#,wall.0,hex(previous.bytes()),hex(next.bytes()))});
  println!(r#"{{"ui_tag":{},"active_snapshot":{},"history":{},"restore":{},"empty_notification":{},"wall_notification":{}}}"#,state.tag(),out.active_state_snapshot.map_or("null".into(),|b|format!("\"{}\"",hex(&b))),history,out.restore_archived_decorators,out.empty_notification,out.wall_notification.map_or("null".into(),|w|w.0.to_string()));continue;
 }
 let old=building_wall_state::WallShapeSnapshot::from_observed(bytes(p[0]));let new=building_wall_state::WallShapeSnapshot::from_observed(bytes(p[1]));let rect=authority::rectangle_from_enclosure(&new).unwrap();let wall=building_wall_state::WallId(p[9].parse().unwrap());
 let mut counter=0;let raw=building_wall_authoring::create_shape_source(&mut counter,building_wall_authoring::CreateShapeRequest{shape:building_wall_authoring::AuthoringShape::Rectangle(rect),previous_wall_hint:None,height:3.2,elevation:0.7,selected_style:1}).unwrap().source;
 let mut sources=std::collections::HashMap::new();if p[4]=="1"{sources.insert(wall,WallAuthoritySnapshot{source:raw,metadata:authority::WallHistoryMetadata::for_new_wall(false)});}
 let mut cmd=replay::WallReplayCommand{edit:WallHistoryEdit::EnclosureChange{wall,previous:old,next:new},undo:p[2]=="1",timed:p[3]=="1",duration:f32::from_bits(u32::from_str_radix(p[5],16).unwrap()),elapsed:f32::from_bits(u32::from_str_radix(p[6],16).unwrap()),finished:p[7]=="1"};
 let terrain=building_wall_collection::TerrainHeightGrid512{encoded:std::sync::Arc::from(vec![0;512*512]),minimum_y:0.75,maximum_y:0.75,sampling_fix:false};let mut steps=Vec::new();
 for dt in p[8].split(','){let dt=f32::from_bits(u32::from_str_radix(dt,16).unwrap());let out=cmd.advance(&mut sources,dt,&terrain).unwrap();let audio=out.audio.map_or("null".into(),|a|{let b:Vec<u8>=a.anchor.iter().flat_map(|f|f.to_le_bytes()).collect();format!(r#"{{"wall":{},"anchor":"{}","kind":{},"flag":{}}}"#,a.wall.0,hex(&b),a.kind,a.flag)});
 steps.push(format!(r#"{{"elapsed":{},"finished":{},"source_count":{},"updated":{},"source":{},"audio":{}}}"#,cmd.elapsed.to_bits(),cmd.finished,sources.len(),out.updated_source_notifications.len(),sources.get(&wall).map_or("null".into(),snapshot),audio));}println!("[{}]",steps.join(","));}}
