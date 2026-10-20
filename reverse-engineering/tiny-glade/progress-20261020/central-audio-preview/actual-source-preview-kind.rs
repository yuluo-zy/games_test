#[derive(Clone, Copy)] struct DecoratorType(u8);
pub fn preview_kind(decorator_type: DecoratorType, subtype: u8) -> (u8, u8) {
    let slot = decorator_type.0.wrapping_sub(0x38);
    let slot = if slot < 8 { slot } else { 6 };
    match slot {
        // O：窗族 0x38..0x3a。
        0..=2 => (7, u8::from(subtype == 2)),
        // O：0x3b 保持缺省 kind。
        3 => (0x16, u8::from(subtype == 2)),
        // O：0x3c。
        4 => (0xc, u8::from(subtype == 3)),
        // O：0x3d。
        5 => match subtype.wrapping_sub(2) {
            0 | 2 => (0x1c, 1),
            1 => (0x1d, 0),
            _ => (0x1c, 0),
        },
        // B1408234b2 tests DL (the original decorator type), not CL (subtype).
        // Out-of-range tags deliberately select this same slot at 1408233be.
        6 => {
            if decorator_type.0 == 9 {
                (0xc, u8::from(subtype == 3))
            } else {
                (0x24, u8::from(subtype != 3))
            }
        }
        // Slot 7 only; tags outside 0x38..=0x3f already selected slot 6.
        _ => (0x16, 0),
    }
}

fn main() {
 let expected = include_bytes!("native-preview-kind.bin");
 let mut failures=0;
 for ty in 0..=255u8 { for sub in 0..=255u8 {
   let (kind, flag)=preview_kind(DecoratorType(ty), sub);
   let n=(ty as usize*256+sub as usize)*2;
   if [kind,flag] != expected[n..n+2] {
     if failures<12 { println!("ty={ty:02x} sub={sub} actual={kind:02x}/{flag} original={:02x}/{}",expected[n],expected[n+1]); }
     failures+=1;
   }
 }}
 println!("65536 original instruction comparisons, failures={failures}");
 assert_eq!(failures,0);
}
