use std::io::{self,BufRead};
use tg_critters_c_pet_harness::sheep_pet::{SheepPetRowObserved,advance_sheep_pet};
fn main(){for line in io::stdin().lock().lines(){let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();if p.is_empty(){continue;}
    let bytes=std::array::from_fn(|i|u8::from_str_radix(&p[0][i*2..i*2+2],16).unwrap());
    let mut row=SheepPetRowObserved::from_observed(bytes);
    let delta=f32::from_bits(u32::from_str_radix(p[2],16).unwrap());
    match advance_sheep_pet(&mut row,p[1]=="1",delta){Ok(changed)=>{
        let consumer=row.consumer_state();
        let bytes=row.0.iter().map(|v|format!("{v:02x}")).collect::<String>();
        println!("{} {} {}",bytes,u8::from(changed),u8::from(consumer.being_petted));
    },Err(error)=>println!("error {error:?}"),}
}}
