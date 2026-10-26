use country_core::input::action_set::{DigitalActionPhase as P,DigitalActionState as S};
use country_core::systems::backend::input::calculate_input_delta::digital_delta;
fn main(){
    for (p,t,dt) in [(P::Held,0.19f32,0.02f32),(P::Repeating,-0.18,0.02)]{
        let result=digital_delta(true,true,S{phase:p,timer_seconds:t},dt,true);
        println!("before={:?} timer={} dt={} -> {:?} timer={} bits={:08x} fires={}",p,t,dt,result.phase,result.timer_seconds,result.timer_seconds.to_bits(),result.just_pressed());
    }
}
