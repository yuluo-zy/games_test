#![allow(dead_code)]
#[path="D:/game/tiny-glade/crates/utils/src/easing_functions.rs"] mod easing_functions;
use easing_functions::EasingFunction;
#[derive(Clone,Copy)]pub struct UiCustomSize{kind:u32,value:f32}
pub fn focus_scale(
    select: f32,
    slider: bool,
    reset: bool,
    custom: Option<&UiCustomSize>,
    scale: f32,
) -> f32 {
    let mut target = if slider || reset { 1.2 } else { 1.5 };
    if let Some(custom) = custom {
        if custom.kind == 1 {
            target = custom.value;
        }
    }
    let ease = EasingFunction::CubicOut.apply(select);
    (target * ease + (1. - ease)) * scale
}
pub fn appear_scale(appear: f32, scale: f32) -> f32 {
    let ease = EasingFunction::CubicOut.apply(appear);
    // Preserve native floating operations, including NaN/Infinity behavior.
    ((1. - ease) * 0. + ease) * scale
}
pub fn click_sound(
    slider: bool,
    locked: bool,
    photo: Option<u8>,
    app: bool,
    category: bool,
) -> Option<u32> {
    if slider {
        None
    } else if locked {
        Some(11)
    } else if photo == Some(9) {
        Some(29)
    } else if app || category {
        None
    } else {
        Some(14)
    }
}
pub fn drag_sound(slider: bool, state: u8) -> Option<u32> {
    if !slider {
        None
    } else {
        match state {
            2 | 4 => None,
            3 => Some(26),
            _ => Some(27),
        }
    }
}
fn n(b:&[u8],a:usize)->u32{u32::from_le_bytes(b[a..a+4].try_into().unwrap())}fn f(b:&[u8],a:usize)->f32{f32::from_bits(n(b,a))}
fn main(){let mut nan_payload_differences=0;
 let fixture=include_bytes!("native-focus.bin");for (i,b) in fixture.chunks_exact(20).enumerate(){let custom=UiCustomSize{kind:b[3] as u32,value:f(b,4)};let actual=focus_scale(f(b,8),b[0]!=0,b[1]!=0,(b[2]!=0).then_some(&custom),f(b,12));if actual.is_nan()&&f(b,16).is_nan(){nan_payload_differences+=usize::from(actual.to_bits()!=n(b,16));}else{assert_eq!(actual.to_bits(),n(b,16),"focus{i}");}}
 let fixture=include_bytes!("native-appear.bin");for (i,b) in fixture.chunks_exact(12).enumerate(){let actual=appear_scale(f(b,0),f(b,4));if actual.is_nan()&&f(b,8).is_nan(){nan_payload_differences+=usize::from(actual.to_bits()!=n(b,8));}else{assert_eq!(actual.to_bits(),n(b,8),"appear{i}");}}
 let mut sounds=Vec::new();
 for i in 0..578 {let mask=if i<576 {i/18}else{0};sounds.push(if mask&1!=0{10}else{15});}
 for i in 0..578 {let mask=if i<576 {i/18}else{0};if let Some(sound)=click_sound(mask&8!=0,mask&1!=0,(mask&16!=0).then_some((i%18) as u8),mask&2!=0,mask&4!=0){sounds.push(sound);}}
 for i in 0..578 {let mask=if i<576 {i/18}else{0};for state in 0..=255 {if let Some(sound)=drag_sound(mask&8!=0,state){sounds.push(sound);}}}
 let expected=include_bytes!("native-sounds-output.bin").chunks_exact(4).map(|b|n(b,0)).collect::<Vec<_>>();assert_eq!(sounds,expected);
 println!("Actual source1848focus+66appear nonNaNbits/NaNclassification and {} original sound emissions match",sounds.len());println!("NaN payload/sign differences permitted by Rust arithmetic: {nan_payload_differences}; all nonNaN output bits exact");
}
