extern crate bevy_input;
use bevy_input::{keyboard::KeyboardInput,mouse::MouseButtonInput};
fn main(){println!("keyboard {} key {} state {} repeat {} logical {}",size_of::<KeyboardInput>(),std::mem::offset_of!(KeyboardInput,key_code),std::mem::offset_of!(KeyboardInput,state),std::mem::offset_of!(KeyboardInput,repeat),std::mem::offset_of!(KeyboardInput,logical_key));println!("mouse {} button {} state {} window {}",size_of::<MouseButtonInput>(),std::mem::offset_of!(MouseButtonInput,button),std::mem::offset_of!(MouseButtonInput,state),std::mem::offset_of!(MouseButtonInput,window));}
