use std::io::{self, BufRead};
use glam::Vec3;
use perchance::PerchanceContext;
use tg_critters_evidence_harness::firefly::*;
fn f(s:&str)->f32 { f32::from_bits(u32::from_str_radix(s,16).unwrap()) }
fn v(p:&[&str])->Vec3 { Vec3::new(f(p[0]),f(p[1]),f(p[2])) }
fn bv(v:Vec3)->String { format!("{:08x} {:08x} {:08x}",v.x.to_bits(),v.y.to_bits(),v.z.to_bits()) }
fn main(){
 for line in io::stdin().lock().lines(){
  let line=line.unwrap();let p:Vec<_>=line.split_whitespace().collect();if p.is_empty(){continue}
  match p[0] {
   "goal"=>{
    let seed=u128::from_str_radix(p[1],16).unwrap();let advance:usize=p[2].parse().unwrap();
    let mut rng=PerchanceContext::new(seed);for _ in 0..advance{rng.uniform_f32();}
    let mut firefly=Firefly{goal_timer:f(p[3])};let pos=v(&p[4..7]);let vel=v(&p[7..10]);
    let mut goal=if p[10]=="1" {Some(v(&p[11..14]))} else {None};
    update_goal(&mut firefly,&mut goal,pos,vel,f(p[14]),p[15]=="1",&mut rng);
    let goal_out=goal.map(bv).unwrap_or_else(||"none".into());
    // Safe public API fingerprint of remaining RNG sequence; no private layout or RNG rewrite.
    let mut follow=rng.clone();let fp=(0..4).map(|_|format!("{:08x}",follow.uniform_f32().to_bits())).collect::<Vec<_>>().join(" ");
    println!("{:08x} {} | {}",firefly.goal_timer.to_bits(),goal_out,fp);
   }
   "velocity"=>{
    let pos=v(&p[1..4]);let goal=if p[4]=="1"{Some(v(&p[5..8]))}else{None};
    let mut target=VelocityTarget{direction:v(&p[8..11]),retention_per_second:f(p[11])};let mut m=f(p[12]);
    set_velocity_target(pos,goal,&mut target,&mut m);
    println!("{} {:08x} {:08x}",bv(target.direction),target.retention_per_second.to_bits(),m.to_bits());
   }
   "smooth"=>{
    let mut vel=v(&p[1..4]);let target=VelocityTarget{direction:v(&p[4..7]),retention_per_second:f(p[7])};smooth_velocity(&mut vel,target,f(p[8]));println!("{}",bv(vel));
   }
   "sphere"=>{println!("{}",bv(sample_sphere_game(f(p[1]),f(p[2]))));}
   _=>panic!("unrecognized request"),
  }
 }
}
