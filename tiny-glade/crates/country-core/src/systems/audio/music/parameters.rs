//! Resolved B music parameter boundaries and NewGladeTime accumulator.
/// B MusicRichness's three resolved boundary bands. This excludes the unknown
/// MusicPlayer/smoothing adapter and accepts finite scalar values only.
pub fn snap_off_boundaries(value: f32) -> f32 {
    if (0.24..0.25).contains(&value) {
        0.251
    } else if (0.49..=0.5).contains(&value) {
        0.501
    } else if (0.74..=0.75).contains(&value) {
        0.751
    } else {
        value
    }
}

/// Resolved NewGladeTime accumulator, projected from MusicAudioState+0x1c8.
/// Finite time/accumulator inputs only; no complete FMOD state or native layout.
pub fn advance_new_glade_time(time: f32, accumulator: &mut f32) -> f32 {
    *accumulator = (time + *accumulator).clamp(0.0, 0.8);
    *accumulator
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_music_parameter_boundaries() {
        assert_eq!(snap_off_boundaries(0.24), 0.251);
        assert_eq!(snap_off_boundaries(0.2499), 0.251);
        assert_eq!(snap_off_boundaries(0.25), 0.25);
        assert_eq!(snap_off_boundaries(0.5), 0.501);
        assert_eq!(snap_off_boundaries(0.74), 0.751);
        assert_eq!(snap_off_boundaries(0.75), 0.751);
        assert_eq!(snap_off_boundaries(0.3), 0.3);
        let mut accumulator = 0.0;
        assert_eq!(advance_new_glade_time(0.5, &mut accumulator), 0.5);
        assert_eq!(advance_new_glade_time(0.5, &mut accumulator), 0.8);
    }
}
