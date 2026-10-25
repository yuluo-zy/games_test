use std::io::{self,BufRead};
use tiny_glade_reconstruction::{history_transactions::*,building_roof_customization::RoofRimHistoryMutation,building_wall_state::WallId};
fn edit(id:u64)->RoofRimHistoryMutation { RoofRimHistoryMutation {observed_tag:4,wall_id:WallId(id),from:(id%3) as u8,to:((id+1)%3) as u8} }
fn state(h:&RimHistoryTransactions)->String {
 let l=h.ledger();
 let groups=l.entries.iter().map(|x|x.edits.iter().map(|m|m.wall_id.0.to_string()).collect::<Vec<_>>().join(",")).collect::<Vec<_>>().join(";");
 let snaps=l.resolved_snapshots.iter().map(|x|x.chain_index.to_string()).collect::<Vec<_>>().join(",");
 format!("{} {} {} {} {} {} {}",l.cursor,l.undo_anchor,l.observed_count_58,l.mutation_nonce,h.pending().map(|x|x.edits.len() as i64).unwrap_or(-1),if groups.is_empty(){"-"}else{&groups},if snaps.is_empty(){"-"}else{&snaps})
}
fn main(){
 for line in io::stdin().lock().lines(){
  let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();
  let len:usize=p[0].parse().unwrap();let cursor:i32=p[1].parse().unwrap();let anchor:i32=p[2].parse().unwrap();let count:usize=p[3].parse().unwrap();let nonce:u64=p[4].parse().unwrap();
  let snapshots=if p[5]=="-"{vec![]}else{p[5].split(',').map(|x|ResolvedHistorySnapshot {chain_index:x.parse().unwrap(),value:()}).collect()};
  let mut h=HistoryTransactions::from_observed(ObservedHistoryLedger {entries:(0..len).map(|i|HistoryEntry::from_edits(vec![edit(100+i as u64)])).collect(),cursor,undo_anchor:anchor,resolved_snapshots:snapshots,observed_count_58:count,mutation_nonce:nonce});
  let mut next=9000_u64;let mut fresh=||{next+=1;next};let mut out=Vec::new();
  for op in p[6].split(','){
   let result=if let Some(m)=op.strip_prefix('s') {h.start_input_recording(m.parse().unwrap()).map(|_|"ok".into())}
   else if let Some(m)=op.strip_prefix('t'){Ok(if h.try_start_input_recording(m.parse().unwrap()){"1"}else{"0"}.into())}
   else if let Some(id)=op.strip_prefix('a'){h.add_with_nonce_provider(edit(id.parse().unwrap()),&mut fresh).map(|_|"ok".into())}
   else if op=="f"{h.finish_with_nonce_provider(&mut fresh).map(|x|if x.is_some(){"1"}else{"0"}.into())}
   else if op=="m0"||op=="m1"{let mut token=HistoryMutationToken::from_observed_active(op=="m1");h.finish_potential_with_nonce_provider(&mut token,&mut fresh);Ok("ok".into())}
   else {let undo=op.starts_with('u');h.cursor_step(if undo {HistoryCursorStep::Undo}else{HistoryCursorStep::Redo},op.ends_with('!')).map(|x|x.map(|r|format!("{}:{}:{}#{}",r.entry_index,u8::from(r.undo),r.ordered_edits.iter().map(|e|e.wall_id.0.to_string()).collect::<Vec<_>>().join(";"),r.rim_events().iter().map(|e|match e {tiny_glade_reconstruction::roof_replay::ObservedReplayEvent::FlatRimStyle {wall_index,style}=>format!("{wall_index}={style}"),_=>unreachable!()}).collect::<Vec<_>>().join(";"))).unwrap_or_else(||"none".into()))};
   let res=result.unwrap_or_else(|e|format!("E{}",match e{HistoryTransactionError::ReplayRecordingForbidden=>1,HistoryTransactionError::RecordingNotStarted=>2,HistoryTransactionError::InvalidCursor=>3,HistoryTransactionError::CursorNotAtCommittedTail=>4,HistoryTransactionError::AuxiliaryReplayOwnerRequired=>5}));
   out.push(format!("{res}|{}",state(&h)));
  }
  println!("{}",out.join(" / "));
 }
}
