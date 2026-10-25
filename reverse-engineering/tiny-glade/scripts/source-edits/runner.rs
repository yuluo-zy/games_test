#[path="../../reconstruction/roof/edit_rules.rs"] mod edit_rules;
use std::io::{self,BufRead};
use edit_rules::*;
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let w:Vec<_>=line.split_whitespace().collect();
    let decode=|s:&str|->Vec<u8>{(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()};
    let mut roof:[u8;88]=decode(w[0]).try_into().unwrap();let event:[u8;64]=decode(w[1]).try_into().unwrap();let mut tick=u32::from_str_radix(w[2],16).unwrap();let current=u32::from_str_radix(w[3],16).unwrap();
    let out=apply_resolved_roof_edit(&mut roof,&event,&mut tick,current);
    for b in roof{print!("{b:02x}");}print!(" {tick:08x} {}",out.changed_fields);
    for f in [out.changes.profile,out.changes.height,out.changes.ridge,out.changes.eave,out.changes.tip_distance,out.previous_height,out.new_height]{print!(" {:08x}",f.to_bits());}
    print!(" {} {} {} {} {}",out.gable_before_ridge as u8,out.gable_after_ridge as u8,out.rebuild_gable as u8,out.resolved_update as u8,match out.height_crossing{None=>0,Some(HeightCrossing::Down)=>1,Some(HeightCrossing::Up)=>2});
    for feedback in out.feedback {
        let (tag,values,pivot)=match feedback {
            EditFeedback::GableEnter{pivot}=>(10,Vec::new(),pivot),EditFeedback::GableLeave{pivot}=>(11,Vec::new(),pivot),
            EditFeedback::Scalar{gable_after,ceil_height,ceil_other,weight,pivot}=>(2,vec![gable_after as u32,ceil_height.to_bits(),ceil_other.to_bits(),weight.to_bits()],pivot),
            EditFeedback::Tip{current_distance,change_distance,pivot}=>(3,vec![current_distance.to_bits(),change_distance.to_bits()],pivot),
        };print!(" |{tag}");for v in values{print!(" {v:08x}");}for p in pivot.to_array(){print!(" {:08x}",p.to_bits());}
    }println!();
}}
