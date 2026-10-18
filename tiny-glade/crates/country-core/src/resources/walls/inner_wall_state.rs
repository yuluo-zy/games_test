//! Owned B wall authority. Field roles come from the InnerWalls update stores;
//! this representation does not reproduce native Vec headers or Rust ABI.

use crate::resources::history::wall_edit::WallAuthoritySnapshot;
use crate::resources::history::wall_edit::WallHistoryMetadata;
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::publication_source::RawWallPublicationSource;

#[path = "saved_wall_state.rs"]
mod saved_wall_state;

impl<'de> serde::Deserialize<'de> for WallState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        saved_wall_state::deserialize_wall_state(deserializer)
    }
}

impl serde::Serialize for WallState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        saved_wall_state::serialize_wall_state(self, serializer)
    }
}

pub const DEFAULT_ARCH_HEIGHT: f32 = f32::from_bits(0x40333333);

/// B StructureType（inner_wall_state 域；`hint_camera_display_condition` 0x14227c910
/// 的 `StructureType::is_fence` 调用面 + `ui_edit_roof` 族）。布局 U（字段表待取证）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StructureType(pub u8);

impl StructureType {
    /// 0x1408c5680（O，coregap-2 fresh 反编译 2026-10-17 + RNE `0fb601c3` =
    /// `movzx eax, byte ptr [rcx]; ret`）：读取字节 0（非零 = 栅栏；调用方以
    /// `== 0` 判别）。字节 0 是否 bool 字段（直通返回）或判别字节（非零即真）待取证。
    pub fn is_fence(&self) -> bool {
        self.0 != 0
    }

    /// 0x140833230（fresh disasm 2026-10-17：`CMP byte[RCX],0; SETZ AL`）：
    /// 与 `FreehandHeightType::is_follow_terrain` 共享地址（单态合并），
    /// 判别式 = 字节 0 等于 0。
    pub fn is_wall(&self) -> bool {
        self.0 == 0
    }
}

/// B `FreehandHeightType`（inner_wall_state 域）。WallState +0x66 承载该枚举字节
/// （`_try_update_state<update_freehand_ty>` 写 +0x66）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FreehandHeightType(pub u8);

impl FreehandHeightType {
    /// 0x1408c5660（fresh disasm 2026-10-17）：`*self ^= 1`。
    pub fn toggle(&mut self) {
        self.0 ^= 1;
    }

    /// 0x140833230（fresh disasm `CMP byte[RCX],0; SETZ AL`）：变体 0 =
    /// 跟随地形。该地址同时是 `StructureType::is_wall` 的单态合并实现。
    pub fn is_follow_terrain(&self) -> bool {
        self.0 == 0
    }
}

/// B `WallStyle`（inner_wall_state 域；WallState +0x68 承载该字节）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WallStyle(pub u8);

impl WallStyle {
    pub const STONE: WallStyle = WallStyle(0);
    pub const STONE_WITH_SKIRTS: WallStyle = WallStyle(1);
    pub const HALF_TIMBERED: WallStyle = WallStyle(2);
    pub const WATTLE_AND_DAUB: WallStyle = WallStyle(2);

    /// 0x1408c5670（fresh disasm 2026-10-17：`CMP byte[RCX],2; SETNC AL`）：
    /// 无符号 `style >= 2`。同址符号另见 `gable_roof_style`。
    pub fn is_halftimbered(&self) -> bool {
        self.0 >= 2
    }
}

/// `deserialize_wall_style` 0x14085ebc0 / `visit_str` 0x140860d50（fresh 反编译
/// 2026-10-17）：字符串映射 `"Stone"→0`、`"StoneWithSkirts"→1`、
/// `"HalfTimbered"→2`、`"WattleAndDaub"→2`，其余 `unknown_variant`。
impl<'de> serde::Deserialize<'de> for WallStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct WallStyleVisitor;
        impl serde::de::Visitor<'_> for WallStyleVisitor {
            type Value = WallStyle;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a wall style variant identifier")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<WallStyle, E> {
                match value {
                    "Stone" => Ok(WallStyle::STONE),
                    "StoneWithSkirts" => Ok(WallStyle::STONE_WITH_SKIRTS),
                    "HalfTimbered" => Ok(WallStyle::HALF_TIMBERED),
                    "WattleAndDaub" => Ok(WallStyle::WATTLE_AND_DAUB),
                    other => Err(E::unknown_variant(
                        other,
                        &["Stone", "StoneWithSkirts", "HalfTimbered", "WattleAndDaub"],
                    )),
                }
            }
        }
        deserializer.deserialize_str(WallStyleVisitor)
    }
}

/// B 0x1408c5690: ordered equality, including NaN and signed-zero behavior.
pub fn is_default_arch_height(value: f32) -> bool {
    value == DEFAULT_ARCH_HEIGHT
}

#[derive(Clone, Debug)]
pub struct WallState {
    pub(crate) snapshot: WallAuthoritySnapshot,
}

impl WallState {
    /// B 0x14085a370. The demo flag comes from the actual startup owner.
    pub fn new(curve: Curve2, height: f32, style: u8, clutter_version: u8, demo: bool) -> Self {
        let mut shape = [0; 28];
        shape[..4].copy_from_slice(&2u32.to_le_bytes());
        Self::from_snapshot(
            WallAuthoritySnapshot {
                source: RawWallPublicationSource {
                    curve,
                    curve_kind_tag: 2,
                    terrain_floor: 0.0,
                    shape: WallShapeSnapshot::from_observed(shape),
                    style,
                    extra: height,
                    special: false,
                    observed_path_scalar: DEFAULT_ARCH_HEIGHT,
                    clutter_version,
                    observed_source_flag_64: 0,
                },
                metadata: WallHistoryMetadata::for_new_wall(demo),
            },
            0,
        )
    }

    /// `load_state`（0x14085ad80）输入记录内的 WallState 0x70B 解码。
    ///
    /// 已证偏移（LOGIC3 §0 fresh disasm 定案）：+0x34 Curve2 length /
    /// +0x38 shape（0x1cB，tag 在首 u32）/ +0x54 height / +0x58 terrain_floor /
    /// +0x5c arch scalar / +0x60 plaster bits / +0x64 arch_type /
    /// +0x65 clutter_version（source flag 65）/ +0x66 freehand / +0x67 supports /
    /// +0x68 style / +0x69 metadata byte（语义 U）。
    ///
    /// **登记（缺数据面）**：原生记录内的 Curve2 含堆指针（points/points_u Vec），
    /// 不可从字节记录重建；本函数保留 length，点列留空。解锁条件：save 侧
    /// `parse_wall_section` 打包曲线点列（曲线区打包待①）。
    pub fn from_record_bytes(bytes: &[u8; 0x70]) -> Self {
        let at = |o: usize| f32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let mut shape = [0u8; 28];
        shape.copy_from_slice(&bytes[0x38..0x54]);
        let shape = WallShapeSnapshot::from_observed(shape);
        Self {
            snapshot: WallAuthoritySnapshot {
                source: RawWallPublicationSource {
                    curve: Curve2 {
                        points: Vec::new(),
                        points_u: Vec::new(),
                        length: at(0x34),
                    },
                    curve_kind_tag: shape.tag(),
                    terrain_floor: at(0x58),
                    shape,
                    style: bytes[0x68],
                    extra: at(0x54),
                    special: bytes[0x65] != 0,
                    observed_path_scalar: at(0x5c),
                    clutter_version: bytes[0x65],
                    observed_source_flag_64: bytes[0x64],
                },
                metadata: WallHistoryMetadata {
                    source_60_bits: u32::from_le_bytes(bytes[0x60..0x64].try_into().unwrap()),
                    source_66: Some(bytes[0x66]),
                    source_67: bytes[0x67],
                    source_69: bytes[0x69],
                },
            },
        }
    }

    /// Import the real source +66, including older portable snapshots that
    /// omitted it. The metadata becomes its single authoritative storage.
    pub fn from_snapshot(mut snapshot: WallAuthoritySnapshot, freehand_ty: u8) -> Self {
        // The old portable snapshot has two representations of source +65.
        // Preserve its raw byte (historically named clutter_version), deriving
        // the collection predicate so the core never has two mutable owners.
        snapshot.source.special = snapshot.source.clutter_version != 0;
        snapshot.metadata.source_66 = Some(freehand_ty);
        Self { snapshot }
    }

    pub fn snapshot(&self) -> &WallAuthoritySnapshot {
        &self.snapshot
    }

    pub fn into_snapshot(self) -> (WallAuthoritySnapshot, u8) {
        let freehand_ty = self.freehand_ty();
        (self.snapshot, freehand_ty)
    }

    pub fn source(&self) -> &RawWallPublicationSource {
        &self.snapshot.source
    }

    /// Original source +65, copied verbatim to Public +296. The original game
    /// field name is unresolved; it is distinct from freehand type at +66.
    pub fn source_flag_65(&self) -> u8 {
        self.snapshot.source.clutter_version
    }

    pub fn is_special(&self) -> bool {
        self.source_flag_65() != 0
    }

    /// Source +0x54; this is independent of terrain elevation and public max Y.
    pub fn height(&self) -> f32 {
        self.snapshot.source.extra
    }

    pub fn elevation(&self) -> f32 {
        self.snapshot.source.terrain_floor
    }

    /// Source +0x5c, written by 0x140aaa710.
    pub fn arch_height(&self) -> f32 {
        self.snapshot.source.observed_path_scalar
    }

    /// Source +0x66, confirmed from the byte store at 0x140aa9ba7.
    pub fn freehand_ty(&self) -> u8 {
        self.snapshot
            .metadata
            .source_66
            .expect("core wall source +66")
    }

    /// Source +0x60, written as f32 by 0x140aaa6f8.
    pub fn plaster_peel_amount(&self) -> f32 {
        f32::from_bits(self.snapshot.metadata.source_60_bits)
    }

    pub fn arch_type(&self) -> u8 {
        self.snapshot.source.observed_source_flag_64
    }

    pub fn removed_supports(&self) -> u8 {
        self.snapshot.metadata.source_67
    }

    pub fn style(&self) -> u8 {
        self.snapshot.source.style
    }

    pub fn is_premade_shape(&self) -> bool {
        self.snapshot.source.shape.tag() < 2
    }

    /// B 0x1408c56b0 returns the enclosure only for tags below two.
    pub fn get_enclosure_params(&self) -> Option<&WallShapeSnapshot> {
        self.is_premade_shape()
            .then_some(&self.snapshot.source.shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_style_strings_match_native_mapping() {
        let parse = |s: &str| serde_json::from_str::<WallStyle>(s).unwrap();
        assert_eq!(parse("\"Stone\""), WallStyle(0));
        assert_eq!(parse("\"StoneWithSkirts\""), WallStyle(1));
        assert_eq!(parse("\"HalfTimbered\""), WallStyle(2));
        assert_eq!(parse("\"WattleAndDaub\""), WallStyle(2));
        assert!(serde_json::from_str::<WallStyle>("\"Brick\"").is_err());
    }

    #[test]
    fn wall_style_halftimbered_is_byte_ge_two() {
        assert!(!WallStyle(0).is_halftimbered());
        assert!(!WallStyle(1).is_halftimbered());
        assert!(WallStyle(2).is_halftimbered());
        assert!(WallStyle(3).is_halftimbered());
    }

    #[test]
    fn freehand_toggle_flips_low_bit() {
        let mut ty = FreehandHeightType(0);
        assert!(ty.is_follow_terrain());
        ty.toggle();
        assert_eq!(ty.0, 1);
        ty.toggle();
        assert_eq!(ty.0, 0);
        assert!(StructureType(0).is_wall());
        assert!(!StructureType(1).is_wall());
    }

    #[test]
    fn record_bytes_decode_observed_wall_state_offsets() {
        let mut bytes = [0u8; 0x70];
        bytes[0x34..0x38].copy_from_slice(&3.5f32.to_le_bytes()); // curve length
        bytes[0x38..0x3c].copy_from_slice(&1u32.to_le_bytes()); // shape tag
        bytes[0x54..0x58].copy_from_slice(&1.25f32.to_le_bytes()); // height
        bytes[0x58..0x5c].copy_from_slice(&0.5f32.to_le_bytes()); // terrain floor
        bytes[0x5c..0x60].copy_from_slice(&2.8f32.to_le_bytes()); // arch scalar
        bytes[0x60..0x64].copy_from_slice(&0x3f000000u32.to_le_bytes()); // plaster
        bytes[0x64] = 3; // arch_type
        bytes[0x65] = 1; // clutter_version / source flag 65
        bytes[0x66] = 2; // freehand
        bytes[0x67] = 1; // removed supports
        bytes[0x68] = 2; // style
        bytes[0x69] = 5; // metadata byte
        let state = WallState::from_record_bytes(&bytes);
        assert_eq!(state.height(), 1.25);
        assert_eq!(state.elevation(), 0.5);
        assert_eq!(state.arch_height(), 2.8);
        assert_eq!(state.style(), 2);
        assert_eq!(state.freehand_ty(), 2);
        assert_eq!(state.removed_supports(), 1);
        assert_eq!(state.arch_type(), 3);
        assert!(state.is_special());
        assert_eq!(state.plaster_peel_amount(), 0.5);
        assert_eq!(state.snapshot().source.curve.length, 3.5);
        assert_eq!(state.source().curve_kind_tag, 1);
        assert_eq!(state.snapshot().metadata.source_69, 5);
        assert!(state.is_premade_shape()); // tag 1 → premade
    }
}
