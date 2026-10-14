//! 游戏私有 packed ARGB/SRGBA 策略。低8位为alpha；该函数不是通用颜色库。
/// 原 utils::scale_u32_packed_srgba_color 0x140c918d0 只缩放最低8位，保留颜色。
pub fn scale_u32_packed_srgba_color(color: u32, scale: f32) -> u32 {
    let alpha = (((color & 255) as f32 / 255. * scale * 255.) + 0.5).clamp(0., 255.) as u8;
    (color & 0xffff_ff00) | u32::from(alpha)
}
