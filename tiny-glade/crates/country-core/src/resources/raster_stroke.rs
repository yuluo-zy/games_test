//! Owned raster-history payload, B record_raster_history 0x140957990.
//! The outer history edit holds Arc<RasterStroke>; points are Arc<[Vec3]>.
use glam::{Vec2, Vec3};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct RasterStroke {
    pub flags: u32,
    pub strength: f32,
    pub points: Arc<[Vec3]>,
    pub radius: f32,
    pub draw: bool,
}

impl RasterStroke {
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn iter_points_xz(&self) -> impl ExactSizeIterator<Item = Vec2> + '_ {
        self.points.iter().map(|point| Vec2::new(point.x, point.z))
    }
}

// B path_edit::From140a976b0 copies the enum words at+0/+4, Arc slice
// at+8/+10, radius+18, additive+1c. Saved blend is Max or Add{opacity};
// this is distinct from the per-pixel blend closures in world_raster.
#[derive(serde::Serialize, serde::Deserialize)]
enum SavedStrokeBlend {
    Max,
    Add { opacity: f32 },
}
#[derive(serde::Serialize, serde::Deserialize)]
struct SavedRasterStroke {
    points: Vec<Vec3>,
    is_additive: bool,
    brush_size: f32,
    #[serde(default = "max_stroke_blend")]
    blend_mode: SavedStrokeBlend,
}
fn max_stroke_blend() -> SavedStrokeBlend {
    SavedStrokeBlend::Max
}
impl serde::Serialize for RasterStroke {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let blend_mode = match self.flags {
            0 => SavedStrokeBlend::Max,
            1 => SavedStrokeBlend::Add {
                opacity: self.strength,
            },
            other => {
                return Err(serde::ser::Error::custom(format!(
                    "invalid raster blend tag {other}"
                )));
            }
        };
        serde::Serialize::serialize(
            &SavedRasterStroke {
                points: self.points.to_vec(),
                is_additive: self.draw,
                brush_size: self.radius,
                blend_mode,
            },
            s,
        )
    }
}
impl<'de> serde::Deserialize<'de> for RasterStroke {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = <SavedRasterStroke as serde::Deserialize>::deserialize(d)?;
        let (flags, strength) = match v.blend_mode {
            SavedStrokeBlend::Max => (0, 0.0),
            SavedStrokeBlend::Add { opacity } => (1, opacity),
        };
        Ok(Self {
            flags,
            strength,
            points: Arc::from(v.points),
            radius: v.brush_size,
            draw: v.is_additive,
        })
    }
}
