use country_core::systems::save::snapshot::TerrainSectionJson;
fn main() {
 for (label,text) in [
 ("missing_both",r#"{}"#),
 ("missing_strokes",r#"{"next_stroke_id":1}"#),
 ("missing_counter",r#"{"strokes":{}}"#),
 ("invalid_key",r#"{"next_stroke_id":1,"strokes":{"not-a-number":{"falloff":0.4,"height_ws":2.0,"index":0,"points":[[0.0,0.0,0.0]]}}}"#),
 ("overflow_key",r#"{"next_stroke_id":1,"strokes":{"4294967296":{"falloff":0.4,"height_ws":2.0,"index":0,"points":[[0.0,0.0,0.0]]}}}"#)] {
  match serde_json::from_str::<TerrainSectionJson>(text) {
   Ok(section)=>{let n=section.strokes.len();let value=section.into_editing_data();println!("{} accepted input_strokes={} output_strokes={} next={}",label,n,value.strokes.len(),value.next_index);},
   Err(error)=>println!("{} rejected {}",label,error)
  }
 }
}
