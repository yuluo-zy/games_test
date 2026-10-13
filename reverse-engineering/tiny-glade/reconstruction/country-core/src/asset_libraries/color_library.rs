//! `ColorLibrarySsbo` — GPU color palette buffer (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/asset_libraries/color_library.rs`
//! (source_sha256 `f8a54c0d…`, 6 procs). Decompiles:
//! `_analysis/decomp/C_0x14093b530.c`, `C_0x14093e510.c`.
//!
//! `ColorLibrarySsbo` is 0x250 = 592 bytes = an array of Vec4 color entries (O:
//! `deserialize_from` writes at 16-byte stride; `update_color_library` uploads
//! 0x250 bytes). The 40 named getters index this array (I).

use glam::Vec4;
use serde::{Deserialize, Serialize};

/// Count of Vec4 color entries: 0x250 / 16 = 37 (O byte size; exact count I).
pub const COLOR_ENTRY_COUNT: usize = 0x250 / 16;

/// GPU color palette (0x250 bytes, O).
#[derive(Clone, Debug, PartialEq)]
pub struct ColorLibrarySsbo {
    pub colors: [Vec4; COLOR_ENTRY_COUNT],
}

impl Default for ColorLibrarySsbo {
    fn default() -> Self {
        ColorLibrarySsbo {
            colors: [Vec4::ONE; COLOR_ENTRY_COUNT],
        }
    }
}

/// Field-name → index mapping recovered from `deserialize_from` (O bytewise:
/// the deserializer compares RON field names and writes to the matching slot).
macro_rules! color_getters {
    ($($name:ident => $idx:expr),* $(,)?) => {
        impl ColorLibrarySsbo {
            $(
                /// Named color accessor (O name; slot offset observed in
                /// `deserialize_from`).
                #[allow(dead_code)]
                pub fn $name(&self) -> Vec4 {
                    self.colors[$idx]
                }
            )*
        }
    };
}

// Slot offsets below are byte offsets observed in `ColorLibrarySsbo::deserialize_from`
// (0x14093e510); converted to array indices (÷16).
color_getters! {
    path => 0x50 / 16,          // "path" → +0x50
    grass_1 => 0x48 / 16,       // "grass_1" → +0x48
    grass_2 => 0x4c / 16,       // "grass_2" → +0x4c
    sky_base => 0x54 / 16,      // "sky_base" → +0x54
    terrain_1 => 0x64 / 16,     // "terrain_1" → +0x64 (I: adjacent slot)
    terrain_2 => 0x68 / 16,     // "terrain_2" → +0x68
    wall_moss => 0x78 / 16,     // "wall_moss" → +0x78
    wall_bricks => 0x7c / 16,   // "wall_bricks" → +0x7c (I)
    terrain_highlight => 0x44 / 16, // "terrain_highlight" → +0x44 (I)
    night_tint => 0x40 / 16,    // "night_tint" → +0x40 (I)
    wall_1 => 0x70 / 16,        // "wall_1" → +0x70
    wall_2 => 0x74 / 16,        // "wall_2" → +0x74
    ivy_branch => 0x80 / 16,    // "ivy_branch" → +0x80
    ivy_leaf_1 => 0x84 / 16,    // "ivy_leaf_1" → +0x84
}

impl ColorLibrarySsbo {
    /// Remaining named getters (O names; exact slot U — typed gap).
    #[allow(dead_code)]
    pub fn canopy_color_1(&self) -> Vec4 {
        self.colors[0]
    }
    #[allow(dead_code)]
    pub fn canopy_color_2(&self) -> Vec4 {
        self.colors[1]
    }
    #[allow(dead_code)]
    pub fn fence_color_1(&self) -> Vec4 {
        self.colors[2]
    }
    #[allow(dead_code)]
    pub fn fence_color_2(&self) -> Vec4 {
        self.colors[3]
    }
    #[allow(dead_code)]
    pub fn flower_color_1(&self) -> Vec4 {
        self.colors[4]
    }
    #[allow(dead_code)]
    pub fn flower_color_2(&self) -> Vec4 {
        self.colors[5]
    }
    #[allow(dead_code)]
    pub fn indirect_light_tint(&self) -> Vec4 {
        self.colors[6]
    }
    #[allow(dead_code)]
    pub fn ivy_leaf_2(&self) -> Vec4 {
        self.colors[7]
    }
    #[allow(dead_code)]
    pub fn roof_color_1(&self) -> Vec4 {
        self.colors[8]
    }
    #[allow(dead_code)]
    pub fn roof_color_2(&self) -> Vec4 {
        self.colors[9]
    }
    #[allow(dead_code)]
    pub fn sky_clouds_1(&self) -> Vec4 {
        self.colors[10]
    }
    #[allow(dead_code)]
    pub fn sky_clouds_2(&self) -> Vec4 {
        self.colors[11]
    }
    #[allow(dead_code)]
    pub fn sky_high_day(&self) -> Vec4 {
        self.colors[12]
    }
    #[allow(dead_code)]
    pub fn sky_high_dusk(&self) -> Vec4 {
        self.colors[13]
    }
    #[allow(dead_code)]
    pub fn sky_horizon(&self) -> Vec4 {
        self.colors[14]
    }
    #[allow(dead_code)]
    pub fn sky_low_day(&self) -> Vec4 {
        self.colors[15]
    }
    #[allow(dead_code)]
    pub fn sky_low_dusk(&self) -> Vec4 {
        self.colors[16]
    }
    #[allow(dead_code)]
    pub fn sun_tint_day(&self) -> Vec4 {
        self.colors[17]
    }
    #[allow(dead_code)]
    pub fn sun_tint_dusk(&self) -> Vec4 {
        self.colors[18]
    }
    #[allow(dead_code)]
    pub fn temp_color_1(&self) -> Vec4 {
        self.colors[19]
    }
    #[allow(dead_code)]
    pub fn temp_color_2(&self) -> Vec4 {
        self.colors[20]
    }
    #[allow(dead_code)]
    pub fn temp_color_3(&self) -> Vec4 {
        self.colors[21]
    }
    #[allow(dead_code)]
    pub fn terrain_flattened(&self) -> Vec4 {
        self.colors[22]
    }

    /// `deserialize_from` (O: 0x14093e510, 2522B): RON deserializer; maps field
    /// names to slots; unknown field → panic `"Init Color Library failed: entry
    /// is missing from ColorLibrarySSBO"`. Signature simplified to parse a RON
    /// string (the original is a serde `Deserializer` visitor).
    pub fn deserialize_from(&mut self, ron_text: &str) -> Result<(), ron::error::SpannedError> {
        let value: ColorLibraryRon = ron::from_str(ron_text)?;
        *self = value.into_library();
        Ok(())
    }

    /// `deserialize_from_path(&mut self, path)` (O: 0x14155a170 / 0x14155a320).
    pub fn deserialize_from_path(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), ron::error::SpannedError> {
        let text = std::fs::read_to_string(path).map_err(|_| ron::error::SpannedError {
            code: ron::error::Error::Message("unable to read color library".into()),
            position: ron::error::Position { line: 0, col: 0 },
        })?;
        self.deserialize_from(&text)
    }
}

/// RON mirror of the color library (I: uses the same field names as the
/// deserializer).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ColorLibraryRon {
    #[serde(default)]
    pub path: Vec4,
    #[serde(default)]
    pub grass_1: Vec4,
    #[serde(default)]
    pub grass_2: Vec4,
    #[serde(default)]
    pub sky_base: Vec4,
    #[serde(default)]
    pub wall_1: Vec4,
    #[serde(default)]
    pub wall_2: Vec4,
    #[serde(default)]
    pub terrain_1: Vec4,
    #[serde(default)]
    pub terrain_2: Vec4,
    #[serde(default)]
    pub wall_moss: Vec4,
    #[serde(default)]
    pub wall_bricks: Vec4,
    #[serde(default)]
    pub ivy_branch: Vec4,
    #[serde(default)]
    pub ivy_leaf_1: Vec4,
}

impl ColorLibraryRon {
    fn into_library(self) -> ColorLibrarySsbo {
        let mut lib = ColorLibrarySsbo::default();
        macro_rules! set {
            ($idx:expr, $v:expr) => {
                lib.colors[$idx] = $v;
            };
        }
        set!(0x50 / 16, self.path);
        set!(0x48 / 16, self.grass_1);
        set!(0x4c / 16, self.grass_2);
        set!(0x54 / 16, self.sky_base);
        set!(0x70 / 16, self.wall_1);
        set!(0x74 / 16, self.wall_2);
        set!(0x64 / 16, self.terrain_1);
        set!(0x68 / 16, self.terrain_2);
        set!(0x78 / 16, self.wall_moss);
        set!(0x7c / 16, self.wall_bricks);
        set!(0x80 / 16, self.ivy_branch);
        set!(0x84 / 16, self.ivy_leaf_1);
        lib
    }
}

/// `update_color_library` bevy system (O: 0x14093b530, 1214B): defaults to
/// white; tries `ColorLibrarySsbo::deserialize_from_path` (missing file → keep
/// defaults); locates the SSBO in `AssetSsboLibrary` (type-name assert 0x3e
/// chars) and uploads 0x250 bytes via `GraphicsDevice::update_buffer_bytes`;
/// on upload error falls back to a CPU copy.
pub fn update_color_library(
    path: &std::path::Path,
    ssbos: &mut crate::asset_libraries::ssbo_library::AssetSsboLibrary,
    device: &crate::rhapsody_contract::GraphicsDevice,
) {
    let mut lib = ColorLibrarySsbo::default();
    if path.exists() {
        let _ = lib.deserialize_from_path(path);
    }
    let state = ssbos.get_mut::<ColorLibrarySsbo>();
    // O: uploads 0x250 bytes of the palette into the GPU buffer.
    let bytes = unsafe {
        core::slice::from_raw_parts(
            (&lib as *const ColorLibrarySsbo) as *const u8,
            core::mem::size_of::<ColorLibrarySsbo>(),
        )
    };
    let ok = device.update_buffer_bytes(state.rg_handle(), bytes);
    if ok.is_err() {
        // O: fallback CPU copy of the palette is kept on the buffer state
        // (U: exact mirror field not recovered).
    }
}
