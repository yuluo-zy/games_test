//! B `resources/glade_color_customization.rs` — 15 字段颜色定制（≥0x48B）。
//!
//! 存档 Colors 节载荷 = `to_json_string<GladeColorCustomization>` 0x14085ee90；
//! 字段名/索引与偏移由 re-analyst 域类型证据包（RE-DOMAIN-TYPES.md §3）全表给出：
//! `visit_str` 0x14085f550 长度分派 → index 0..0xe，`serialize` 0x14085f980
//! `serialize_entry` 泛型。多为 `ColorId(u32)`；+0x40/+0x41/+0x42 三字节枚举
//! （含 `building_wall_style = StairRailingType` 候选）；`flag` bool、
//! `lantern_brightness` f32。
use crate::resources::color_id::ColorId;

/// 颜色定制（字段序按原 visit_str index 0..0xe；偏移见 RE-DOMAIN-TYPES §3）。
///
/// 三个字节枚举字段（freehand_ty/flat_rim_style/building_wall_style）的**变体名待取证**；
/// 以 `u8` 保留字节存储与 serde 数字形态，枚举变体恢复后改为具名枚举。
#[derive(
    bevy_ecs::resource::Resource,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct GladeColorCustomization {
    /// index 0，+0x18
    pub freeform_wall: ColorId,
    /// index 1，+0x1c
    pub building: ColorId,
    /// index 2，+0x20
    pub roof: ColorId,
    /// index 3，+0x42（字节枚举；StairRailingType 候选，变体名待取证）
    pub building_wall_style: u8,
    /// index 4，+0x40（字节枚举，变体名待取证）
    pub freehand_ty: u8,
    /// index 5，+0x24
    pub halftimber: ColorId,
    /// index 6，+0x28
    pub lantern_light: ColorId,
    /// index 7，+0x2c
    pub fence: ColorId,
    /// index 8，+0x30
    pub window_light: ColorId,
    /// index 9，+0x34
    pub flag: bool,
    /// index 10，native +0x00 tag / +0x04 payload.
    /// None preserves the original automatic/inherited window color; Some(0)
    /// is an explicit color and remains distinct in memory and in saved JSON.
    /// B14085f085 calls the Option<ColorId> serialize_entry specialization.
    pub windows: Option<ColorId>,
    /// index 11，+0x38
    pub gates: ColorId,
    /// index 12，+0x41（字节枚举，变体名待取证）
    pub flat_rim_style: u8,
    /// index 13，+0x3c
    pub lantern_brightness: f32,
    /// index 14，+0x08
    pub stairs: ColorId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_roundtrip_keeps_all_fifteen_fields() {
        let value = GladeColorCustomization {
            freeform_wall: ColorId(1),
            building: ColorId(2),
            roof: ColorId(3),
            building_wall_style: 1,
            freehand_ty: 2,
            halftimber: ColorId(4),
            lantern_light: ColorId(5),
            fence: ColorId(6),
            window_light: ColorId(7),
            flag: true,
            windows: Some(ColorId(8)),
            gates: ColorId(9),
            flat_rim_style: 3,
            lantern_brightness: 0.5,
            stairs: ColorId(10),
        };
        let json = serde_json::to_value(&value).unwrap();
        let back: GladeColorCustomization = serde_json::from_value(json).unwrap();
        assert_eq!(value, back);
        // ColorId 序列化为透明 u32（裸数字，非元组数组）。
        assert_eq!(
            serde_json::to_value(&ColorId(42)).unwrap(),
            serde_json::json!(42)
        );
    }

    #[test]
    fn windows_wire_preserves_none_and_explicit_zero() {
        // Original B14085ee90 was executed directly for each fixture; see
        // round12/color-menu/gcc-windows-original-execution.json. The native
        // serializer always emits the windows key, with null for None.
        for (windows, wire) in [
            (None, serde_json::Value::Null),
            (Some(ColorId(0)), serde_json::json!(0)),
            (Some(ColorId(8)), serde_json::json!(8)),
            (Some(ColorId(u32::MAX)), serde_json::json!(u32::MAX)),
        ] {
            let colors = GladeColorCustomization {
                windows,
                ..Default::default()
            };
            let saved = serde_json::to_value(colors).unwrap();
            assert_eq!(saved["windows"], wire);
            assert_eq!(saved.as_object().unwrap().len(), 15);
            let loaded: GladeColorCustomization = serde_json::from_value(saved).unwrap();
            assert_eq!(loaded.windows, windows);
        }
        assert_ne!(None, Some(ColorId(0)));
    }
}
