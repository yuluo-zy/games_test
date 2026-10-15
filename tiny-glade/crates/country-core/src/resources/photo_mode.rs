//! B photo camera selection, distinct from MainCamera's internal camera modes.
use crate::systems::camera_rig::main_camera::CameraMode;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PhotoModeCameraMode {
    #[default]
    TiltShift = 0,
    Free = 1,
    Walk = 2,
    TopDown = 3,
}

impl PhotoModeCameraMode {
    /// B 0x140b59720, packed table 0x00030201.
    pub fn next(self) -> Self {
        match self {
            Self::TiltShift => Self::Free,
            Self::Free => Self::Walk,
            Self::Walk => Self::TopDown,
            Self::TopDown => Self::TiltShift,
        }
    }

    /// B 0x140b596b0, original table 0x142af1278. TopDown is rejected in demo.
    pub fn next_demo(self) -> Self {
        match self {
            Self::TiltShift => Self::Free,
            Self::Free => Self::Walk,
            Self::Walk => Self::TiltShift,
            Self::TopDown => panic!("TopDown camera is unavailable in demo mode"),
        }
    }
}

impl From<PhotoModeCameraMode> for CameraMode {
    /// B 0x14090e500: photo mode tag plus one.
    fn from(value: PhotoModeCameraMode) -> Self {
        Self(value as u8 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_and_demo_cycles_and_camera_conversion_follow_native_tables() {
        let mut mode = PhotoModeCameraMode::TiltShift;
        for expected in [1, 2, 3, 0] {
            assert_eq!(CameraMode::from(mode).0, mode as u8 + 1);
            mode = mode.next();
            assert_eq!(mode as u8, expected);
        }
        for expected in [1, 2, 0] {
            mode = mode.next_demo();
            assert_eq!(mode as u8, expected);
        }
    }
}
