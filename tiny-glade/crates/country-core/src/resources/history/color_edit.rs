//! B `resources/history/color_edit.rs` — 颜色编辑负载（fresh 0x14003c5d0 serialize /
//! 0x1417bf8a0 visit_str / 0x1417befe0 deserialize）。
//!
//! ## fresh 定案（2026-10-17，restorer-history-remainder）
//!
//! 原版 `ColorEdit` 是**按 ColorTarget 变体互异载荷**的手写 serde：
//! 单 key map `{"<target 名>": {"<id 键>": id, "from": 值, "to": 值}}`。
//!
//! - serialize 0x14003c5d0：判别字节 `[edit+8]`，`idx = clamp(target-3, 0..=0xe)`，
//!   跳表 0x14292c628（15 项，fresh read_memory 逐项解码）。因此内存判别值为
//!   **3..=16**（Wall=3 … StairSupport=16）；index 14（target 17）写空载荷 `{}`，
//!   但 visit_str 不产生该变体。
//! - visit_str 0x1417bf8a0 的键名/变体（字符串常量 @0x14292c11e 起）：
//!   Wall(3)/Gate(4)/Roof(5)/FlatRoof(6)/Decorator(7)/WindowLight(8)/
//!   Halftimber(9)/FlagBunting(10)/Brightness(11)/ChimneySmoke(12)/
//!   StairLadder(13)/StairRailing(14)/MagicColor(15)/StairSupport(16)；
//!   另 len-16 字符串（`DecoratorTerrain`）也接受为 Decorator。
//! - 内层键：wall 族 `wall_id`、decorator 族 `decorator_id`，两侧均为
//!   `from` / `to`（字符串常量与 target 共享，serde 折叠）。
//! - 载荷类型（serialize_entry 单态名逐项对账，跳表 index→类型表）：
//!
//! | target | id | from/to |
//! |---|---|---|
//! | 3 Wall / 4 Gate / 5 Roof / 9 Halftimber | WallId | TerrainStrokeId ×2 |
//! | 6 FlatRoof | WallId | FlatRoofStyle ×2 |
//! | 7 Decorator / 8 WindowLight / 13 StairLadder / 15 MagicColor | DecoratorId | TerrainStrokeId ×2 |
//! | 10 FlagBunting / 12 ChimneySmoke | DecoratorId | bool ×2 |
//! | 11 Brightness | DecoratorId | f32 ×2 |
//! | 14 StairRailing | DecoratorId | StairRailingType ×2 |
//! | 16 StairSupport | DecoratorId | StairSupportType ×2 |
//!
//! ## 跨域类型登记（不编造）
//!
//! - `country_core::resources::terrain_editing::TerrainStrokeId`：crate 内现存
//!   `terrain_editing::StrokeId`（tg_terrain）为同一 4 字节 id 概念，本文件以
//!   [`TerrainStrokeId`] 别名对接；若地形域后续导出独立 `TerrainStrokeId` 类型，
//!   在此切换即可（wire 同形）。
//! - `StairRailingType` / `StairSupportType`（`resources::walls::decorator` 域）：
//!   crate 内**缺失**。fresh 只证明它们是独立的 `serialize_entry` 单态（与
//!   bool/f32 不同型），大小未取证。本文件以 `u8` 不透明持有并显式登记，
//!   待 decorator 域补类型后替换（解锁条件：decorator 域导出两枚举）。

use crate::resources::color_id::ColorId;
use crate::resources::roofs::roof_shape::FlatRoofStyle;
use crate::resources::terrain_editing::StrokeId as TerrainStrokeId;
use crate::resources::walls::WallId;
use crate::resources::walls::decorator::DecoratorId;
use serde::{Deserializer, Serializer};

/// 颜色编辑目标（原生内存判别 3..=16；`ColorTarget::native_discriminant`）。
///
/// 原生判别基址 3 由 serialize 0x14003c5d0 的 `ADD AL,0xfd; ... target-3` 夹取与
/// 15 项跳表 0x14292c628 共同定案。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ColorTarget {
    Wall = 3,
    Gate = 4,
    Roof = 5,
    FlatRoof = 6,
    Decorator = 7,
    WindowLight = 8,
    Halftimber = 9,
    FlagBunting = 10,
    Brightness = 11,
    ChimneySmoke = 12,
    StairLadder = 13,
    StairRailing = 14,
    MagicColor = 15,
    StairSupport = 16,
}

impl ColorTarget {
    /// 原生内存判别值（serialize 0x14003c5d0 `[edit+8]`）。
    pub const fn native_discriminant(self) -> u8 {
        self as u8
    }
    /// 跳表下标（`target - 3`，0x14003c5d0 `ADD AL,0xfd` / `CMOV` 夹取 0..=0xe）。
    pub const fn table_index(self) -> u8 {
        self as u8 - 3
    }
    /// wire 目标键（visit_str 0x1417bf8a0 字符串常量逐项）。
    pub const fn key(self) -> &'static str {
        match self {
            Self::Wall => "Wall",
            Self::Gate => "Gate",
            Self::Roof => "Roof",
            Self::FlatRoof => "FlatRoof",
            Self::Decorator => "Decorator",
            Self::WindowLight => "WindowLight",
            Self::Halftimber => "Halftimber",
            Self::FlagBunting => "FlagBunting",
            Self::Brightness => "Brightness",
            Self::ChimneySmoke => "ChimneySmoke",
            Self::StairLadder => "StairLadder",
            Self::StairRailing => "StairRailing",
            Self::MagicColor => "MagicColor",
            Self::StairSupport => "StairSupport",
        }
    }
    /// visit_str 逆映射：14 个键 + `DecoratorTerrain`（len16）别名 → Decorator。
    pub fn from_key(key: &str) -> Option<Self> {
        Some(match key {
            "Wall" => Self::Wall,
            "Gate" => Self::Gate,
            "Roof" => Self::Roof,
            "FlatRoof" => Self::FlatRoof,
            // fresh：len-16 "DecoratorTerrain" 与 len-9 "Decorator" 同变体 4（3 基 7）。
            "Decorator" | "DecoratorTerrain" => Self::Decorator,
            "WindowLight" => Self::WindowLight,
            "Halftimber" => Self::Halftimber,
            "FlagBunting" => Self::FlagBunting,
            "Brightness" => Self::Brightness,
            "ChimneySmoke" => Self::ChimneySmoke,
            "StairLadder" => Self::StairLadder,
            "StairRailing" => Self::StairRailing,
            "MagicColor" => Self::MagicColor,
            "StairSupport" => Self::StairSupport,
            _ => return None,
        })
    }
    /// 内层 id 键：wall 族 `wall_id`、decorator 族 `decorator_id`。
    pub const fn id_key(self) -> &'static str {
        match self {
            Self::Wall | Self::Gate | Self::Roof | Self::FlatRoof | Self::Halftimber => "wall_id",
            _ => "decorator_id",
        }
    }
}

/// 颜色编辑（原生 ColorTarget 变体互异载荷；跳表 index 0..=14 逐项对账）。
#[derive(Clone, Debug, PartialEq)]
pub enum ColorEdit {
    /// target 3
    Wall {
        wall_id: WallId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 4
    Gate {
        wall_id: WallId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 5
    Roof {
        wall_id: WallId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 6
    FlatRoof {
        wall_id: WallId,
        from: FlatRoofStyle,
        to: FlatRoofStyle,
    },
    /// target 7
    Decorator {
        decorator_id: DecoratorId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 8
    WindowLight {
        decorator_id: DecoratorId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 9（Halftimber 以 WallId 为键）
    Halftimber {
        wall_id: WallId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 10
    FlagBunting {
        decorator_id: DecoratorId,
        from: bool,
        to: bool,
    },
    /// target 11
    Brightness {
        decorator_id: DecoratorId,
        from: f32,
        to: f32,
    },
    /// target 12
    ChimneySmoke {
        decorator_id: DecoratorId,
        from: bool,
        to: bool,
    },
    /// target 13
    StairLadder {
        decorator_id: DecoratorId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 14（StairRailingType 缺失 → u8 不透明，登记）
    StairRailing {
        decorator_id: DecoratorId,
        from: u8,
        to: u8,
    },
    /// target 15
    MagicColor {
        decorator_id: DecoratorId,
        from: TerrainStrokeId,
        to: TerrainStrokeId,
    },
    /// target 16（StairSupportType 缺失 → u8 不透明，登记）
    StairSupport {
        decorator_id: DecoratorId,
        from: u8,
        to: u8,
    },
}

impl ColorEdit {
    /// 目标变体（原生 `[edit+8]` 判别）。
    pub const fn target(&self) -> ColorTarget {
        match self {
            Self::Wall { .. } => ColorTarget::Wall,
            Self::Gate { .. } => ColorTarget::Gate,
            Self::Roof { .. } => ColorTarget::Roof,
            Self::FlatRoof { .. } => ColorTarget::FlatRoof,
            Self::Decorator { .. } => ColorTarget::Decorator,
            Self::WindowLight { .. } => ColorTarget::WindowLight,
            Self::Halftimber { .. } => ColorTarget::Halftimber,
            Self::FlagBunting { .. } => ColorTarget::FlagBunting,
            Self::Brightness { .. } => ColorTarget::Brightness,
            Self::ChimneySmoke { .. } => ColorTarget::ChimneySmoke,
            Self::StairLadder { .. } => ColorTarget::StairLadder,
            Self::StairRailing { .. } => ColorTarget::StairRailing,
            Self::MagicColor { .. } => ColorTarget::MagicColor,
            Self::StairSupport { .. } => ColorTarget::StairSupport,
        }
    }
}

/// 手写 `Serialize`（0x14003c5d0）：`{"<target>":{"<id>":..,"from":..,"to":..}}`。
impl serde::Serialize for ColorEdit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let target = self.target();
        let mut outer = serializer.serialize_map(Some(1))?;
        outer.serialize_entry(target.key(), &ColorEditInner { edit: self })?;
        outer.end()
    }
}

/// 手工内层载荷序列化（顺序：id → from → to；键名与 native 共享常量的折叠一致）。
struct ColorEditInner<'a> {
    edit: &'a ColorEdit,
}

impl serde::Serialize for ColorEditInner<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        match &self.edit {
            ColorEdit::Wall { wall_id, from, to }
            | ColorEdit::Gate { wall_id, from, to }
            | ColorEdit::Roof { wall_id, from, to }
            | ColorEdit::Halftimber { wall_id, from, to } => {
                map.serialize_entry("wall_id", &wall_id.0)?;
                map.serialize_entry("from", from)?;
                map.serialize_entry("to", to)?;
            }
            ColorEdit::FlatRoof { wall_id, from, to } => {
                map.serialize_entry("wall_id", &wall_id.0)?;
                map.serialize_entry("from", from)?;
                map.serialize_entry("to", to)?;
            }
            ColorEdit::Decorator {
                decorator_id,
                from,
                to,
            }
            | ColorEdit::WindowLight {
                decorator_id,
                from,
                to,
            }
            | ColorEdit::StairLadder {
                decorator_id,
                from,
                to,
            }
            | ColorEdit::MagicColor {
                decorator_id,
                from,
                to,
            } => {
                map.serialize_entry("decorator_id", decorator_id)?;
                map.serialize_entry("from", from)?;
                map.serialize_entry("to", to)?;
            }
            ColorEdit::FlagBunting {
                decorator_id,
                from,
                to,
            }
            | ColorEdit::ChimneySmoke {
                decorator_id,
                from,
                to,
            } => {
                map.serialize_entry("decorator_id", decorator_id)?;
                map.serialize_entry("from", from)?;
                map.serialize_entry("to", to)?;
            }
            ColorEdit::Brightness {
                decorator_id,
                from,
                to,
            } => {
                map.serialize_entry("decorator_id", decorator_id)?;
                map.serialize_entry("from", from)?;
                map.serialize_entry("to", to)?;
            }
            ColorEdit::StairRailing {
                decorator_id,
                from,
                to,
            } => {
                let name = |value: u8| -> Result<&str, S::Error> {
                    match value {
                        0 => Ok("None"),
                        1 => Ok("Low"),
                        2 => Ok("High"),
                        3 => Ok("Wooden"),
                        _ => Err(serde::ser::Error::custom("invalid StairRailingType")),
                    }
                };
                map.serialize_entry("decorator_id", decorator_id)?;
                map.serialize_entry("from", name(*from)?)?;
                map.serialize_entry("to", name(*to)?)?;
            }
            ColorEdit::StairSupport {
                decorator_id,
                from,
                to,
            } => {
                map.serialize_entry("decorator_id", decorator_id)?;
                map.serialize_entry("from", from)?;
                map.serialize_entry("to", to)?;
            }
        }
        map.end()
    }
}

/// 手写 `Deserialize`（0x1417befe0 + visit_str 0x1417bf8a0）：
/// 单 entry map → 目标键 → 三字段内层 map。
impl<'de> serde::Deserialize<'de> for ColorEdit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as _;
        let value = serde_json::Value::deserialize(deserializer)?;
        let serde_json::Value::Object(mut outer) = value else {
            return Err(D::Error::custom("expected a single color-target key"));
        };
        if outer.len() != 1 {
            return Err(D::Error::custom("expected exactly one color-target key"));
        }
        let (key, payload) = outer.into_iter().next().unwrap();
        let target = ColorTarget::from_key(&key).ok_or_else(|| {
            D::Error::unknown_variant(
                &key,
                &[
                    "Wall",
                    "Gate",
                    "Roof",
                    "FlatRoof",
                    "Decorator",
                    "WindowLight",
                    "Halftimber",
                    "FlagBunting",
                    "Brightness",
                    "ChimneySmoke",
                    "StairLadder",
                    "StairRailing",
                    "MagicColor",
                    "StairSupport",
                ],
            )
        })?;
        let mut fields = payload
            .as_object()
            .cloned()
            .ok_or_else(|| D::Error::custom("expected a color edit payload map"))?;
        let take = |fields: &mut serde_json::Map<String, serde_json::Value>, k: &str| {
            fields
                .remove(k)
                .ok_or_else(|| D::Error::missing_field("color edit payload field"))
        };
        let result = match target {
            ColorTarget::Wall | ColorTarget::Gate | ColorTarget::Roof => {
                let id = WallId(
                    serde_json::from_value::<u64>(take(&mut fields, "wall_id")?)
                        .map_err(D::Error::custom)?,
                );
                let from =
                    serde_json::from_value(take(&mut fields, "from")?).map_err(D::Error::custom)?;
                let to =
                    serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?;
                match target {
                    ColorTarget::Wall => ColorEdit::Wall {
                        wall_id: id,
                        from,
                        to,
                    },
                    ColorTarget::Gate => ColorEdit::Gate {
                        wall_id: id,
                        from,
                        to,
                    },
                    _ => ColorEdit::Roof {
                        wall_id: id,
                        from,
                        to,
                    },
                }
            }
            ColorTarget::FlatRoof => ColorEdit::FlatRoof {
                wall_id: WallId(
                    serde_json::from_value::<u64>(take(&mut fields, "wall_id")?)
                        .map_err(D::Error::custom)?,
                ),
                from: serde_json::from_value(take(&mut fields, "from")?)
                    .map_err(D::Error::custom)?,
                to: serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?,
            },
            ColorTarget::Halftimber => ColorEdit::Halftimber {
                wall_id: WallId(
                    serde_json::from_value::<u64>(take(&mut fields, "wall_id")?)
                        .map_err(D::Error::custom)?,
                ),
                from: serde_json::from_value(take(&mut fields, "from")?)
                    .map_err(D::Error::custom)?,
                to: serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?,
            },
            ColorTarget::Decorator
            | ColorTarget::WindowLight
            | ColorTarget::StairLadder
            | ColorTarget::MagicColor => {
                let id = serde_json::from_value(take(&mut fields, "decorator_id")?)
                    .map_err(D::Error::custom)?;
                let from =
                    serde_json::from_value(take(&mut fields, "from")?).map_err(D::Error::custom)?;
                let to =
                    serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?;
                match target {
                    ColorTarget::Decorator => ColorEdit::Decorator {
                        decorator_id: id,
                        from,
                        to,
                    },
                    ColorTarget::WindowLight => ColorEdit::WindowLight {
                        decorator_id: id,
                        from,
                        to,
                    },
                    ColorTarget::StairLadder => ColorEdit::StairLadder {
                        decorator_id: id,
                        from,
                        to,
                    },
                    _ => ColorEdit::MagicColor {
                        decorator_id: id,
                        from,
                        to,
                    },
                }
            }
            ColorTarget::FlagBunting | ColorTarget::ChimneySmoke => {
                let id = serde_json::from_value(take(&mut fields, "decorator_id")?)
                    .map_err(D::Error::custom)?;
                let from =
                    serde_json::from_value(take(&mut fields, "from")?).map_err(D::Error::custom)?;
                let to =
                    serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?;
                match target {
                    ColorTarget::FlagBunting => ColorEdit::FlagBunting {
                        decorator_id: id,
                        from,
                        to,
                    },
                    _ => ColorEdit::ChimneySmoke {
                        decorator_id: id,
                        from,
                        to,
                    },
                }
            }
            ColorTarget::Brightness => ColorEdit::Brightness {
                decorator_id: serde_json::from_value(take(&mut fields, "decorator_id")?)
                    .map_err(D::Error::custom)?,
                from: serde_json::from_value(take(&mut fields, "from")?)
                    .map_err(D::Error::custom)?,
                to: serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?,
            },
            ColorTarget::StairRailing => {
                let railing = |v: serde_json::Value| -> Result<u8, D::Error> {
                    match v.as_str() {
                        Some("None") => Ok(0),
                        Some("Low") => Ok(1),
                        Some("High") => Ok(2),
                        Some("Wooden") => Ok(3),
                        _ => Err(D::Error::custom("invalid StairRailingType")),
                    }
                };
                ColorEdit::StairRailing {
                    decorator_id: serde_json::from_value(take(&mut fields, "decorator_id")?)
                        .map_err(D::Error::custom)?,
                    from: railing(take(&mut fields, "from")?)?,
                    to: railing(take(&mut fields, "to")?)?,
                }
            }
            ColorTarget::StairSupport => ColorEdit::StairSupport {
                decorator_id: serde_json::from_value(take(&mut fields, "decorator_id")?)
                    .map_err(D::Error::custom)?,
                from: serde_json::from_value(take(&mut fields, "from")?)
                    .map_err(D::Error::custom)?,
                to: serde_json::from_value(take(&mut fields, "to")?).map_err(D::Error::custom)?,
            },
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_discriminants_and_keys_match_fresh_tables() {
        // 0x14003c5d0：target-3 跳表；0x1417bf8a0：键名→变体。
        for (target, key) in [
            (ColorTarget::Wall, "Wall"),
            (ColorTarget::Gate, "Gate"),
            (ColorTarget::Roof, "Roof"),
            (ColorTarget::FlatRoof, "FlatRoof"),
            (ColorTarget::Decorator, "Decorator"),
            (ColorTarget::WindowLight, "WindowLight"),
            (ColorTarget::Halftimber, "Halftimber"),
            (ColorTarget::FlagBunting, "FlagBunting"),
            (ColorTarget::Brightness, "Brightness"),
            (ColorTarget::ChimneySmoke, "ChimneySmoke"),
            (ColorTarget::StairLadder, "StairLadder"),
            (ColorTarget::StairRailing, "StairRailing"),
            (ColorTarget::MagicColor, "MagicColor"),
            (ColorTarget::StairSupport, "StairSupport"),
        ] {
            assert_eq!(ColorTarget::from_key(key), Some(target));
            assert_eq!(target.key(), key);
            assert_eq!(target.native_discriminant() - 3, target.table_index());
        }
        assert_eq!(ColorTarget::Wall.native_discriminant(), 3);
        assert_eq!(ColorTarget::StairSupport.native_discriminant(), 16);
        assert_eq!(ColorTarget::StairSupport.table_index(), 13);
        assert_eq!(
            ColorTarget::from_key("DecoratorTerrain"),
            Some(ColorTarget::Decorator)
        );
    }

    #[test]
    fn wall_variant_wire_matches_native_payload_order() {
        let edit = ColorEdit::Wall {
            wall_id: WallId(7),
            from: TerrainStrokeId(1),
            to: TerrainStrokeId(2),
        };
        assert_eq!(
            serde_json::to_string(&edit).unwrap(),
            r#"{"Wall":{"wall_id":7,"from":1,"to":2}}"#
        );
        let back: ColorEdit =
            serde_json::from_str(r#"{"Wall":{"wall_id":7,"from":1,"to":2}}"#).unwrap();
        assert_eq!(back, edit);
    }

    #[test]
    fn all_targets_round_trip() {
        let cases = [
            serde_json::json!({"Wall":{"wall_id":1,"from":2,"to":3}}),
            serde_json::json!({"Gate":{"wall_id":1,"from":2,"to":3}}),
            serde_json::json!({"Roof":{"wall_id":1,"from":2,"to":3}}),
            serde_json::json!({"FlatRoof":{"wall_id":1,"from":"MatchingStoneOnly","to":{"Override":{"material":"Stone","color_id":0,"pattern_id":0}}}}),
            serde_json::json!({"Decorator":{"decorator_id":4,"from":2,"to":3}}),
            serde_json::json!({"WindowLight":{"decorator_id":4,"from":2,"to":3}}),
            serde_json::json!({"Halftimber":{"wall_id":1,"from":2,"to":3}}),
            serde_json::json!({"FlagBunting":{"decorator_id":4,"from":true,"to":false}}),
            serde_json::json!({"Brightness":{"decorator_id":4,"from":0.5,"to":0.75}}),
            serde_json::json!({"ChimneySmoke":{"decorator_id":4,"from":true,"to":true}}),
            serde_json::json!({"StairLadder":{"decorator_id":4,"from":2,"to":3}}),
            serde_json::json!({"StairRailing":{"decorator_id":4,"from":"Low","to":"High"}}),
            serde_json::json!({"MagicColor":{"decorator_id":4,"from":2,"to":3}}),
            serde_json::json!({"StairSupport":{"decorator_id":4,"from":1,"to":2}}),
        ];
        for value in cases {
            let edit: ColorEdit = serde_json::from_value(value.clone()).unwrap();
            let out = serde_json::to_value(&edit).unwrap();
            assert_eq!(out, value);
        }
    }

    #[test]
    fn decorator_terrain_alias_parses_as_decorator() {
        let edit: ColorEdit =
            serde_json::from_str(r#"{"DecoratorTerrain":{"decorator_id":1,"from":2,"to":3}}"#)
                .unwrap();
        assert!(matches!(
            edit,
            ColorEdit::Decorator {
                decorator_id: DecoratorId(1),
                ..
            }
        ));
    }
}
