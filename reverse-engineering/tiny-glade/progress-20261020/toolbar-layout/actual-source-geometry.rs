use glam::Vec2;
pub fn activation_rect(alignment: u8, window: Vec2, thin: f32, total: f32) -> (Vec2, Vec2) {
    match alignment {
        0 => (
            Vec2::new(window.x * 0.5, window.y + thin * -0.5),
            Vec2::new(total, thin),
        ),
        1 => (
            Vec2::new(thin * 0.5, window.y * 0.5),
            Vec2::new(thin, total),
        ),
        2 => (
            Vec2::new(window.x + thin * -0.5, window.y * 0.5),
            Vec2::new(thin, total),
        ),
        3 => (
            Vec2::new(window.x * 0.5, thin * 0.5),
            Vec2::new(total, thin),
        ),
        _ => unreachable!("invalid toolbar alignment"),
    }
}

/// Full original geometry (0x142343f2f..0x142343ff9), shared by the real
/// cursor hit test and DebugToolbarActivationZone drawable producer.
/// Preserve the original f32 evaluation order, including count == 0.
pub fn toolbar_activation_geometry(
    alignment: u8,
    window: Vec2,
    scale: f32,
    count: i32,
) -> (Vec2, Vec2) {
    let count = count as f32;
    let button = scale * 110.0;
    let total = (count + -1.0) * 10.0 * scale + button * count;
    let thin = button * 0.3;
    let (center, dimensions) = activation_rect(alignment, window, thin, total);
    let margin = if alignment == 1 || alignment == 2 {
        Vec2::new(scale * 80.0, scale * 40.0)
    } else {
        Vec2::new(scale * 40.0, scale * 80.0)
    };
    (center, dimensions + (margin + margin))
}

pub fn auto_hide_sound_kind(old: f32, new: f32) -> Option<u32> {
    if old == 0.0 && new > 0.0 {
        Some(0x10)
    } else if old == 1.0 && new < 1.0 {
        Some(0x11)
    } else {
        None
    }
}

fn main() {
 let bytes=include_bytes!("geometry-native-cases.bin");
 for (index,record) in bytes.chunks_exact(36).enumerate() {
  let get=|n:usize|u32::from_le_bytes(record[n*4..n*4+4].try_into().unwrap());
  let (pos,size)=toolbar_activation_geometry(get(0) as u8,Vec2::new(f32::from_bits(get(2)),f32::from_bits(get(3))),f32::from_bits(get(4)),get(1) as i32);
  let actual=[pos.x.to_bits(),pos.y.to_bits(),size.x.to_bits(),size.y.to_bits()];
  let expected=[get(5),get(6),get(7),get(8)];
  assert_eq!(actual,expected,"original B geometry case {index}");
 }
 for (old,new,expected) in [(0.,0.1,Some(16)),(1.,0.9,Some(17)),(0.,f32::NAN,None),(1.,f32::NAN,None),(f32::NAN,0.,None)] {
  assert_eq!(auto_hide_sound_kind(old,new),expected);
 }
 println!("{} original B geometry cases ({} f32 bit comparisons) + 5 sound boundary cases passed",bytes.len()/36,bytes.len()/36*4);
}
