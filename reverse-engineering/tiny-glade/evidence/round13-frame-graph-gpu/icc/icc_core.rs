//! Original B display ICC producer (update@14093d830, grid@1413036c0).
//! Little CMS parsing/color conversion is reused from original lcms2 5.6.0.
use std::{path::{Path, PathBuf}, sync::Arc};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use crate::render::load_texture::boundary::{Texture, TextureDesc, TextureDevice};

pub const ICC_LUT_EDGE: u32 = 32;
pub const ICC_LUT_FORMAT: u32 = 0x5b; // R16G16B16A16_UNORM, not half float.

#[derive(Event, Clone, Debug)]
pub struct IccProfileChanged(pub PathBuf);

/// The current App's real texture namespace. Installed with its device runtime.
#[derive(Resource, Clone)]
pub struct IccTextureDevice(pub Arc<dyn TextureDevice + Send + Sync>);

struct OwnedIccLut {
    name: String,
    texture: Texture,
    device: Arc<dyn TextureDevice + Send + Sync>,
}
impl Drop for OwnedIccLut {
    fn drop(&mut self) { self.device.release_texture(&self.texture); }
}

/// Canonical resource consumed by final_post. None is the original default.
/// Native Some owns a display name and texture; no native ABI is transmuted.
#[derive(Resource, Default)]
pub struct ActiveIccProfile { active: Option<OwnedIccLut> }
impl ActiveIccProfile {
    pub fn texture(&self) -> Option<&Texture> {
        self.active.as_ref().map(|value| &value.texture)
    }
    pub fn name(&self) -> Option<&str> {
        self.active.as_ref().map(|value| value.name.as_str())
    }
}

pub struct IccProfilePlugin;
impl Plugin for IccProfilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveIccProfile>()
            .add_event::<IccProfileChanged>()
            .add_systems(Update, update_window_icc_profile);
    }
}

/// Native flat_map order: R changes fastest, then G, then B. The integer
/// quotient truncates; alpha is the observed u16 value 255, not 65535.
fn input_grid() -> Vec<[u16; 4]> {
    let mut values = Vec::with_capacity((ICC_LUT_EDGE.pow(3)) as usize);
    let channel = |value: u32| (value * u16::MAX as u32 / (ICC_LUT_EDGE - 1)) as u16;
    for b in 0..ICC_LUT_EDGE {
        for g in 0..ICC_LUT_EDGE {
            for r in 0..ICC_LUT_EDGE {
                values.push([channel(r), channel(g), channel(b), 255]);
            }
        }
    }
    values
}

fn generate_icc_lut(path: &Path) -> anyhow::Result<Vec<[u16; 4]>> {
    let bytes = std::fs::read(path)?;
    let output = lcms2::Profile::new_icc(&bytes)?;
    let input = lcms2::Profile::new_srgb();
    let transform = lcms2::Transform::<[u16; 4], [u16; 4]>::new(
        &input, lcms2::PixelFormat::RGBA_16,
        &output, lcms2::PixelFormat::RGBA_16, lcms2::Intent::Perceptual,
    )?;
    let mut grid = input_grid();
    transform.transform_in_place(&mut grid);
    Ok(grid)
}

pub fn update_window_icc_profile(
    mut events: EventReader<IccProfileChanged>,
    mut active: ResMut<ActiveIccProfile>,
    device: Res<IccTextureDevice>,
) {
    let Some(event) = events.read().last() else { return; };
    // Original clears/releases the previous Some before file I/O or parsing.
    // An invalid new profile therefore leaves None; it does not retain old LUT.
    active.active.take();
    match generate_icc_lut(&event.0) {
        Ok(grid) => {
            // Native GPU creation is expect("ICC LUT texture"): an allocation
            // or upload failure is fatal, not the recoverable parse warning.
            let texture = device.0.create_texture("ICC",
                &TextureDesc::new_3d([ICC_LUT_EDGE; 3], ICC_LUT_FORMAT).sampled(),
                bytemuck::cast_slice(&grid),
            ).expect("ICC LUT texture");
            let shown = event.0.file_name().unwrap_or_else(|| event.0.as_os_str());
            active.active = Some(OwnedIccLut {
                name: shown.to_string_lossy().into_owned(), texture, device: device.0.clone(),
            });
        }
        Err(error) => log::warn!(target: "country_core::systems::backend::icc",
            "Failed creating an ICC LUT: {error}"),
    }
}
