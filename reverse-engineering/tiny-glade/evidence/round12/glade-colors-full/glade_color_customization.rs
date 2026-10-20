//! Original B15-field Colors wire. Full native serializers and visitor
//! executions are retained in round12/glade-colors-full, including all enums.
use crate::resources::{color_id::ColorId,
    roofs::roof_shape::FlatRoofRimStyle,
    walls::{inner_wall_state::{ArchType,WallStyle},decorator::StairRailingType}};

/// B GCC+8..18: independent stair, ladder and magic colors plus railing type.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StairCustomization {
    pub stairs_color:ColorId,
    pub ladder_color:ColorId,
    pub railing_ty:StairRailingType,
    #[serde(default)]
    pub magic_color:ColorId,
}
impl Default for StairCustomization {
    fn default()->Self {
        Self { stairs_color:ColorId(100003),ladder_color:ColorId(0),
            railing_ty:StairRailingType::None,magic_color:ColorId(0) }
    }
}

#[derive(bevy_ecs::resource::Resource,Clone,Copy,Debug,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct GladeColorCustomization {
    pub freeform_wall:ColorId,
    pub building:ColorId,
    pub roof:ColorId,
    pub building_wall_style:WallStyle,
    pub freehand_ty:ArchType,
    pub halftimber:ColorId,
    pub lantern_light:ColorId,
    pub fences:ColorId,
    pub window_light:ColorId,
    pub flag:ColorId,
    pub windows:Option<ColorId>,
    pub gates:ColorId,
    #[serde(default)]
    pub flat_rim_style:FlatRoofRimStyle,
    #[serde(default="default_lantern_brightness")]
    pub lantern_brightness:f32,
    #[serde(default)]
    pub stairs:StairCustomization,
}
fn default_lantern_brightness()->f32 { 1.0 }
impl Default for GladeColorCustomization {
    /// Complete original82B default1408c5600, executed with poison-filled
    /// storage and then serialized by original14085ee90 to verify all fields.
    fn default()->Self {
        Self { freeform_wall:ColorId(0),building:ColorId(8),roof:ColorId(0),
            building_wall_style:WallStyle::STONE_WITH_SKIRTS,
            freehand_ty:ArchType::FollowTerrain,halftimber:ColorId(0),
            lantern_light:ColorId(0),fences:ColorId(0),window_light:ColorId(0),
            flag:ColorId(0),windows:None,gates:ColorId(0),
            flat_rim_style:FlatRoofRimStyle::Rim,lantern_brightness:1.0,
            stairs:StairCustomization::default() }
    }
}
