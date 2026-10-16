//! 正弦波工具（utils/sine_wave.rs）。
//!
//! 源图 0 业务（内联/胶水）；split_into_random_rows 的 jitter 族使用。
/// 观测正弦波采样（振幅/相位按调用点）。
pub fn sine_wave(t: f32, amplitude: f32, frequency: f32, phase: f32) -> f32 {
    amplitude * (t * frequency + phase).sin()
}
