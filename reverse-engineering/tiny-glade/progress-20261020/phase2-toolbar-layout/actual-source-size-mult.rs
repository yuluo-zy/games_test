pub fn toolbar_size_multiplier(alignment: u8, window: [u32; 2], icon_size_mult: f32) -> f32 {
    let axis = window[usize::from(alignment == 1 || alignment == 2)] as f32;
    let mut required = 1282.5 * icon_size_mult + 30.0;
    if alignment == 1 {
        required += icon_size_mult * -192.5;
        required += 22.0;
    }
    let ratio = axis / required;
    if ratio < 1.0 { ratio } else { 1.0 }
}

fn main(){
 let bytes=include_bytes!("size-mult-native-cases.bin");
 for (i,r) in bytes.chunks_exact(20).enumerate(){let q=|o|u32::from_le_bytes(r[o..o+4].try_into().unwrap());
 let actual=toolbar_size_multiplier(q(0) as u8,[q(4),q(8)],f32::from_bits(q(12)));
 assert_eq!(actual.to_bits(),q(16),"original multiplier case {i}");}
 println!("{} original multiplier cases passed by exact f32 bits",bytes.len()/20);
}
