//! `asset_libraries/color_library.rs` —— 调色板 SSBO `ColorLibrarySsbo` 及其 JSON 装载（A3 逐函数核对版）。
//!
//! 原版 6 个过程（PDB `country_core::asset_libraries::color_library::*`），全部对照机器码（O）：
//!
//! | 过程 | 地址 | 大小 | 结论 |
//! |---|---|---|---|
//! | `impl$1::default_value` | 0x14093b430 | 252 | `StorageElement::default_value() -> Option<Self>`：`{tag=1(u32), 37×(1,0,1,1)}`，**品红** |
//! | `impl$3::default` | 0x14093f900 | 245 | `Default`：同一块 37×(1,0,1,1)（0x250 B，**不是**全白） |
//! | `ColorLibrarySsbo::deserialize_from` | 0x14093e510 | 5102 | 吃 `Vec<ColorFromJson>`，按名字写槽，alpha 恒 1.0 |
//! | `deserialize_from_path<&str>` / `<&PathBuf>` | 0x14155a320 / 0x14155a170 | 403 / 419 | `read_to_string` + `serde_json::from_str`，两处 `unwrap` |
//! | `update_color_library` | 0x14093b530 | 1502 | bevy 系统：默认 → 基础 json → 主题 json → 上传 GPU → 写回 CPU 副本 |
//!
//! 旧版的重大错误（逐条已改）：
//!  * 数据格式是 **JSON**（`serde_json 1.0.85`，`from_str::<Vec<ColorFromJson>>`），不是 RON；
//!  * 元素是 `ColorFromJson { name: String, color: [f32; 3] }`（O：`serde_json deserialize_struct` 的字段名表
//!    "name"、"color"，两者缺失都报 `missing_field`；多余字段被 `IgnoredAny` 忽略），不是“按固定字段名的结构体”；
//!  * 调色板共 **37** 个槽（0x250 / 16），槽位与名字的对应来自 `deserialize_from` 内按名字长度跳转的比较链，
//!    旧版的槽偏移（0x50/0x48/…）与 `ColorLibraryRon` 全是臆造；
//!  * 默认值是品红 (1,0,1,1)，缺失条目保持品红；alpha 被强制 1.0（JSON 只给 rgb）；
//!  * 未知名字 → `panic!("Init Color Library failed: {name} entry is missing from ColorLibrarySSBO")`
//!    （O：pieces @0x142ab2208，`String` Display 实参，源码行 66）；
//!  * 加载顺序：先 `assets/colors.json`（`unwrap`，必须存在），再按主题叠加
//!    `assets/glade/<theme>/colors.json`（存在才读），最后把结果上传并写回 CPU 资源。
//!
//! 槽位表（O，由比较链逐项解码；槽字节偏移 = 序号 × 16）：序号 0..36 依次为
//! `temp_color_1/2/3, roof_color_1/2, sky_low_day, sky_high_day, sun_tint_day, sky_low_dusk, sky_high_dusk,
//! sun_tint_dusk, indirect_light_tint, fence_color_1/2, flower_color_1/2, canopy_color_1/2, grass_1/2, path,
//! sky_base, sky_horizon, sky_clouds_1/2, terrain_1/2, terrain_flattened, wall_1/2, wall_moss, wall_bricks,
//! ivy_branch, ivy_leaf_1/2, terrain_highlight, night_tint`。
//! （名字长度 4..=19 走 `jmp [table]`：4 字节 `path`、19 字节 `indirect_light_tint` 用 SSE `pcmpeqb`、
//! `terrain_1/2, wall_moss, wall_bricks, terrain_highlight, night_tint` 走 `bcmp` 调用，其余为内联 xor 比较；
//! 行为等价于“在表里按字节精确查名”，因此这里用线性表查找。）
//!
//! `StorageElement::NAME` = `"ColorLibrary"`（O）：`update_color_library` 里 `FrozenMap::get` 的键立即数
//! `0xef2a68c12cde3f0d`，用原 EXE 的 XXH3 secret（0x142b2ebb0）对 `"ColorLibrary"` 复算一致（见 A3_color.md）。
//! `type_name::<ColorLibrarySsbo>()` 长 0x3e = `country_core::asset_libraries::color_library::ColorLibrarySsbo`。
//!
//! 仍为 I/U：
//!  * I：`ColorLibrarySsbo` 是 `Resource`（`update_color_library` 的第 4 参数被 `memcpy` 0x250 字节写回）；
//!    `BUFFER_LEN == 1`（有 `default_value`、缓冲 0x250 B）；缓冲实际是 `TypedStorageBuffer<ColorLibrarySsbo>`。
//!  * I：主题枚举的类型名与变体名（O 只有 `u8` 下标 → 5 个目录名 `summer/olden/flowery/autumn/winter`）。
//!  * U：`vtable +0x18` 返回 `Buffer*`（为空 → `Option::unwrap` panic，源码行 156）的方法在本重建的
//!    `Ssbo` trait 里对应 `buffer()`（该 trait 的槽顺序见 ssbo_library 文档）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::single_range_in_vec_init)]
use std::path::{Path, PathBuf};

use bevy_ecs::resource::Resource;
use glam::Vec4;
use serde::Deserialize;

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::{GraphicsDevice, slice_as_bytes};
use crate::render::ssbo::{BufferWriteError, StorageElement, TypedStorageBuffer};

/// 调色板槽数（O：0x250 / 16）。
pub const COLOR_ENTRY_COUNT: usize = 37;

/// 槽名表（O：`deserialize_from` 比较链解码，序号即槽下标）。
pub const COLOR_NAMES: [&str; COLOR_ENTRY_COUNT] = [
    "temp_color_1",
    "temp_color_2",
    "temp_color_3",
    "roof_color_1",
    "roof_color_2",
    "sky_low_day",
    "sky_high_day",
    "sun_tint_day",
    "sky_low_dusk",
    "sky_high_dusk",
    "sun_tint_dusk",
    "indirect_light_tint",
    "fence_color_1",
    "fence_color_2",
    "flower_color_1",
    "flower_color_2",
    "canopy_color_1",
    "canopy_color_2",
    "grass_1",
    "grass_2",
    "path",
    "sky_base",
    "sky_horizon",
    "sky_clouds_1",
    "sky_clouds_2",
    "terrain_1",
    "terrain_2",
    "terrain_flattened",
    "wall_1",
    "wall_2",
    "wall_moss",
    "wall_bricks",
    "ivy_branch",
    "ivy_leaf_1",
    "ivy_leaf_2",
    "terrain_highlight",
    "night_tint",
];

/// 品红 (1,0,1,1)（O：0x142a9be30，`default`/`default_value`/`update_color_library` 栈上初值同一常量）。
pub const MISSING_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 1.0];

/// GPU 调色板（O：0x250 B，4 字节对齐的 37×`[f32; 4]`；`Option<Self>` 的 tag 在 +0、数据在 +4 即由此而来）。
#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ColorLibrarySsbo {
    pub colors: [[f32; 4]; COLOR_ENTRY_COUNT],
}

impl Default for ColorLibrarySsbo {
    /// O 0x14093f900：37 个槽全写品红。
    fn default() -> Self {
        ColorLibrarySsbo {
            colors: [MISSING_COLOR; COLOR_ENTRY_COUNT],
        }
    }
}

impl StorageElement for ColorLibrarySsbo {
    /// O：`SsboId` 键 = xxh3_64("ColorLibrary") = 0xef2a68c12cde3f0d。
    const NAME: &'static str = "ColorLibrary";
    /// I：单个 uniform 结构体（有 `default_value`）。
    const BUFFER_LEN: usize = 1;

    /// O 0x14093b430：`Some(默认调色板)`（tag = 1）。
    fn default_value() -> Option<Self> {
        Some(Self::default())
    }
}

impl SsboType for ColorLibrarySsbo {
    /// I：见模块文档。
    type Storage = TypedStorageBuffer<ColorLibrarySsbo>;

    /// I：无初始数据走 `TypedStorageBuffer::new(.., None)`（`BUFFER_LEN == 1` → `default_value`，即品红调色板）；
    /// 带字节数据走按字节创建器。
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        match initial {
            None => TypedStorageBuffer::<ColorLibrarySsbo>::new(device, None),
            Some(bytes) => {
                TypedStorageBuffer::<ColorLibrarySsbo>::new_storage_buffer(device, Some(bytes))
            }
        }
    }
}

/// JSON 里的一项（O：`serde_json deserialize_struct` 的字段表 `name`、`color`；
/// `color` 是 `[f32; 3]`（`ArrayVisitor<[f32;3]>`），所以结构体 0x28 B = String 0x18 + 3×f32 + 填充）。
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ColorFromJson {
    pub name: String,
    pub color: [f32; 3],
}

/// 槽下标查找（O：`deserialize_from` 的比较链；名字长度不在 4..=19 同样落到 panic 分支）。
pub fn slot_index(name: &str) -> Option<usize> {
    COLOR_NAMES.iter().position(|n| *n == name)
}

macro_rules! color_getters {
    ($($name:ident => $idx:expr),* $(,)?) => {
        impl ColorLibrarySsbo {
            $(
                /// 命名取色（O：槽序号见 `COLOR_NAMES`）。
                pub fn $name(&self) -> Vec4 {
                    Vec4::from_array(self.colors[$idx])
                }
            )*
        }
    };
}

color_getters! {
    temp_color_1 => 0,
    temp_color_2 => 1,
    temp_color_3 => 2,
    roof_color_1 => 3,
    roof_color_2 => 4,
    sky_low_day => 5,
    sky_high_day => 6,
    sun_tint_day => 7,
    sky_low_dusk => 8,
    sky_high_dusk => 9,
    sun_tint_dusk => 10,
    indirect_light_tint => 11,
    fence_color_1 => 12,
    fence_color_2 => 13,
    flower_color_1 => 14,
    flower_color_2 => 15,
    canopy_color_1 => 16,
    canopy_color_2 => 17,
    grass_1 => 18,
    grass_2 => 19,
    path => 20,
    sky_base => 21,
    sky_horizon => 22,
    sky_clouds_1 => 23,
    sky_clouds_2 => 24,
    terrain_1 => 25,
    terrain_2 => 26,
    terrain_flattened => 27,
    wall_1 => 28,
    wall_2 => 29,
    wall_moss => 30,
    wall_bricks => 31,
    ivy_branch => 32,
    ivy_leaf_1 => 33,
    ivy_leaf_2 => 34,
    terrain_highlight => 35,
    night_tint => 36,
}

impl ColorLibrarySsbo {
    /// O 0x14093e510：逐项（输入顺序）把 `[r, g, b, 1.0]` 写进名字对应的槽；未出现的槽保持原值。
    /// 未知名字 → panic（文案见模块文档）；panic 前已处理的条目已写入（原版同样是边遍历边写）。
    pub fn deserialize_from(&mut self, entries: Vec<ColorFromJson>) {
        for entry in entries {
            let Some(slot) = slot_index(&entry.name) else {
                panic!(
                    "Init Color Library failed: {} entry is missing from ColorLibrarySSBO",
                    entry.name
                );
            };
            let [r, g, b] = entry.color;
            self.colors[slot] = [r, g, b, 1.0];
        }
    }

    /// O 0x14155a170（`&PathBuf`）/ 0x14155a320（`&str`）：泛型 `AsRef<Path>`。
    /// `fs::read_to_string(path).unwrap()`（源码行 114），`serde_json::from_str::<Vec<ColorFromJson>>(..).unwrap()`
    ///（行 115），然后 `deserialize_from`。任何一步失败都 panic（`Result::unwrap`）。
    pub fn deserialize_from_path<P: AsRef<Path>>(&mut self, path: P) {
        let text = std::fs::read_to_string(path).unwrap();
        let entries: Vec<ColorFromJson> = serde_json::from_str(&text).unwrap();
        self.deserialize_from(entries);
    }
}

/// 主题（O：`update_color_library` 第 1 参数指向的设置里 `+0x58` 处的 `u8`，用作 5 项目录名表的下标，
/// 表 @0x142ab22b8/0x142ab22e0 = {"summer", "olden", "flowery", "autumn", "winter"}；类型名/变体名为 I）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GladeTheme {
    Summer = 0,
    Olden = 1,
    Flowery = 2,
    Autumn = 3,
    Winter = 4,
}

impl GladeTheme {
    /// O：目录名表。
    pub fn dir_name(self) -> &'static str {
        ["summer", "olden", "flowery", "autumn", "winter"][self as usize]
    }
}

/// 资源根目录（O：字面量 "assets/colors.json" 的前缀 + "assets/glade"）。
pub const ASSETS_ROOT: &str = "assets";

/// 主题覆盖文件路径（O：`Path::join("assets/glade", theme).join("colors.json")`）。
pub fn theme_colors_path(root: &Path, theme: GladeTheme) -> PathBuf {
    root.join("glade")
        .join(theme.dir_name())
        .join("colors.json")
}

/// O 0x14093b530：bevy 系统 `update_color_library`。
///
/// 1. `lib = default()`（品红）；
/// 2. `lib.deserialize_from_path("assets/colors.json")`（文件缺失 → `unwrap` panic）；
/// 3. `assets/glade/<theme>/colors.json` 存在（`fs::stat` 成功）才再叠加一次；
/// 4. `ssbos` 里按键取 `ColorLibrarySsbo` 的缓冲：未注册 → `Option::unwrap` panic（行 156）；类型名断言
///    （ssbo_library.rs 行 210，`assert_eq!`）；缓冲 < 0x250 B → 越界错误；整块 `update_buffer_bytes(.., [0..0x250], 0)`，
///    失败 `Result::unwrap` panic（行 158）；
/// 5. 成功后把 `lib` 的 0x250 字节写回 CPU 资源 `cpu`（O：`memcpy(r9, lib, 0x250)`，只在上传成功之后）。
pub fn update_color_library(
    theme: GladeTheme,
    ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    cpu: &mut ColorLibrarySsbo,
) {
    update_color_library_in(Path::new(ASSETS_ROOT), theme, ssbos, device, cpu)
}

/// `update_color_library` 的可注入资源根版本（原版根为相对工作目录的 "assets"，这里仅为可测试而参数化）。
pub fn update_color_library_in(
    root: &Path,
    theme: GladeTheme,
    ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    cpu: &mut ColorLibrarySsbo,
) {
    let mut lib = ColorLibrarySsbo::default();
    lib.deserialize_from_path(root.join("colors.json"));

    let themed = theme_colors_path(root, theme);
    if std::fs::metadata(&themed).is_ok() {
        lib.deserialize_from_path(&themed);
    }

    let buffer = ssbos.get_buffer::<ColorLibrarySsbo>().unwrap();
    let bytes = slice_as_bytes(core::slice::from_ref(&lib));
    let result: Result<(), BufferWriteError> = if (buffer.byte_size as usize) < bytes.len() {
        Err(BufferWriteError::OutOfBounds)
    } else {
        device
            .update_buffer_bytes(buffer, bytes, &[0..bytes.len()], 0)
            .map_err(|_| BufferWriteError::Device)
    };
    result.unwrap();
    *cpu = lib;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rhapsody_contract::GraphicsDevice as Device;

    fn tmp_root(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("color_lib_{}_{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn write(path: &Path, text: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    fn registered() -> (AssetSsboLibrary, Device) {
        let dev = Device(7);
        let lib = AssetSsboLibrary::new();
        lib.add_new::<ColorLibrarySsbo>(&dev);
        (lib, dev)
    }

    #[test]
    fn layout_is_0x250_bytes_four_byte_aligned() {
        assert_eq!(core::mem::size_of::<ColorLibrarySsbo>(), 0x250);
        assert_eq!(core::mem::align_of::<ColorLibrarySsbo>(), 4);
        // Option<Self> 的 tag 在 +0、数据在 +4（default_value 的 [rcx+4..0x254]）
        assert_eq!(core::mem::size_of::<Option<ColorLibrarySsbo>>(), 0x254);
    }

    #[test]
    fn default_and_default_value_are_magenta() {
        let d = ColorLibrarySsbo::default();
        assert!(d.colors.iter().all(|c| *c == [1.0, 0.0, 1.0, 1.0]));
        assert_eq!(ColorLibrarySsbo::default_value(), Some(d));
    }

    #[test]
    fn storage_element_contract() {
        assert_eq!(<ColorLibrarySsbo as StorageElement>::NAME, "ColorLibrary");
        assert_eq!(<ColorLibrarySsbo as StorageElement>::BUFFER_LEN, 1);
        // 原版断言 type_name 长 0x3e = "country_core::" (14) + 48；测试台里 crate 名不同，只比后半段
        let full = core::any::type_name::<ColorLibrarySsbo>();
        let tail = "asset_libraries::color_library::ColorLibrarySsbo";
        assert!(full.ends_with(tail));
        assert_eq!(tail.len() + "country_core::".len(), 0x3e);
    }

    #[test]
    fn slot_table_matches_decoded_machine_code() {
        assert_eq!(COLOR_NAMES.len(), 37);
        for (i, n) in COLOR_NAMES.iter().enumerate() {
            assert!(
                (4..=19).contains(&n.len()),
                "{n}: 名字长度必须落在跳转表 4..=19"
            );
            assert_eq!(slot_index(n), Some(i));
        }
        // 抽查：槽字节偏移 = 序号 * 16（rdi+0x140 = path、+0x240 = night_tint、+0xb0 = indirect_light_tint）
        assert_eq!(slot_index("path"), Some(0x140 / 16));
        assert_eq!(slot_index("night_tint"), Some(0x240 / 16));
        assert_eq!(slot_index("indirect_light_tint"), Some(0xb0 / 16));
        assert_eq!(slot_index("terrain_flattened"), Some(0x1b0 / 16));
        assert_eq!(slot_index("sky_base"), Some(0x150 / 16));
        assert_eq!(slot_index("temp_color_1"), Some(0));
        assert_eq!(slot_index("wall_1"), Some(0x1c0 / 16));
        assert_eq!(slot_index("terrain_highlight"), Some(0x230 / 16));
        assert_eq!(slot_index("nope"), None);
        assert_eq!(slot_index("Path"), None, "区分大小写");
        assert_eq!(slot_index("path "), None);
    }

    #[test]
    fn named_getters_read_their_slot() {
        let mut lib = ColorLibrarySsbo::default();
        lib.colors[20] = [0.1, 0.2, 0.3, 1.0];
        lib.colors[36] = [0.4, 0.5, 0.6, 1.0];
        assert_eq!(lib.path(), Vec4::new(0.1, 0.2, 0.3, 1.0));
        assert_eq!(lib.night_tint(), Vec4::new(0.4, 0.5, 0.6, 1.0));
        assert_eq!(lib.temp_color_1(), Vec4::new(1.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn deserialize_from_writes_rgb_and_forces_alpha_one() {
        let mut lib = ColorLibrarySsbo::default();
        lib.deserialize_from(vec![
            ColorFromJson {
                name: "grass_1".into(),
                color: [0.25, 0.5, 0.75],
            },
            ColorFromJson {
                name: "sky_base".into(),
                color: [0.0, 0.0, 0.0],
            },
        ]);
        assert_eq!(
            lib.colors[slot_index("grass_1").unwrap()],
            [0.25, 0.5, 0.75, 1.0]
        );
        assert_eq!(
            lib.colors[slot_index("sky_base").unwrap()],
            [0.0, 0.0, 0.0, 1.0]
        );
        // 其余槽保持原值（品红）
        assert_eq!(lib.colors[slot_index("path").unwrap()], MISSING_COLOR);
    }

    #[test]
    fn deserialize_from_later_entry_overrides_and_runs_in_order() {
        let mut lib = ColorLibrarySsbo::default();
        lib.deserialize_from(vec![
            ColorFromJson {
                name: "path".into(),
                color: [1.0, 1.0, 1.0],
            },
            ColorFromJson {
                name: "path".into(),
                color: [0.5, 0.5, 0.5],
            },
        ]);
        assert_eq!(lib.path(), Vec4::new(0.5, 0.5, 0.5, 1.0));
    }

    #[test]
    fn unknown_name_panics_after_earlier_entries_were_applied() {
        let mut lib = ColorLibrarySsbo::default();
        let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            lib.deserialize_from(vec![
                ColorFromJson {
                    name: "path".into(),
                    color: [0.0, 0.0, 0.0],
                },
                ColorFromJson {
                    name: "bogus".into(),
                    color: [0.0, 0.0, 0.0],
                },
            ]);
        }));
        assert!(r.is_err());
        assert_eq!(
            lib.path(),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            "panic 前的条目已写入"
        );
    }

    #[test]
    #[should_panic(
        expected = "Init Color Library failed: bogus entry is missing from ColorLibrarySSBO"
    )]
    fn unknown_name_panic_message() {
        ColorLibrarySsbo::default().deserialize_from(vec![ColorFromJson {
            name: "bogus".into(),
            color: [0.0; 3],
        }]);
    }

    #[test]
    #[should_panic(expected = "Init Color Library failed:  entry is missing from ColorLibrarySSBO")]
    fn out_of_range_name_length_takes_the_same_panic() {
        // 长度 0 < 4 → `ja` 直接落到 panic 分支
        ColorLibrarySsbo::default().deserialize_from(vec![ColorFromJson {
            name: String::new(),
            color: [0.0; 3],
        }]);
    }

    #[test]
    #[should_panic(expected = "Init Color Library failed: indirect_light_tint_ entry is missing")]
    fn name_longer_than_19_panics() {
        ColorLibrarySsbo::default().deserialize_from(vec![ColorFromJson {
            name: "indirect_light_tint_".into(),
            color: [0.0; 3],
        }]);
    }

    #[test]
    fn json_schema_requires_name_and_color_and_ignores_extras() {
        let ok: Vec<ColorFromJson> =
            serde_json::from_str(r#"[{"name":"path","color":[0.1,0.2,0.3],"note":"ignored"}]"#)
                .unwrap();
        assert_eq!(ok[0].name, "path");
        assert!(serde_json::from_str::<Vec<ColorFromJson>>(r#"[{"name":"path"}]"#).is_err());
        assert!(serde_json::from_str::<Vec<ColorFromJson>>(r#"[{"color":[0,0,0]}]"#).is_err());
        assert!(
            serde_json::from_str::<Vec<ColorFromJson>>(r#"[{"name":"p","color":[0,0]}]"#).is_err()
        );
    }

    #[test]
    fn deserialize_from_path_reads_json_for_str_and_pathbuf() {
        let root = tmp_root("path");
        let f = root.join("c.json");
        write(&f, r#"[{"name":"wall_moss","color":[0.5,0.25,0.125]}]"#);
        let mut a = ColorLibrarySsbo::default();
        a.deserialize_from_path(f.to_str().unwrap());
        let mut b = ColorLibrarySsbo::default();
        b.deserialize_from_path(&f);
        assert_eq!(a, b);
        assert_eq!(a.wall_moss(), Vec4::new(0.5, 0.25, 0.125, 1.0));
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn deserialize_from_path_missing_file_panics() {
        ColorLibrarySsbo::default().deserialize_from_path("/nonexistent/dir/colors.json");
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn deserialize_from_path_bad_json_panics() {
        let root = tmp_root("bad");
        let f = root.join("c.json");
        write(&f, "not json");
        ColorLibrarySsbo::default().deserialize_from_path(&f);
    }

    #[test]
    fn theme_dirs_and_paths() {
        let names: Vec<_> = [
            GladeTheme::Summer,
            GladeTheme::Olden,
            GladeTheme::Flowery,
            GladeTheme::Autumn,
            GladeTheme::Winter,
        ]
        .iter()
        .map(|t| t.dir_name())
        .collect();
        assert_eq!(names, ["summer", "olden", "flowery", "autumn", "winter"]);
        assert_eq!(
            theme_colors_path(Path::new("assets"), GladeTheme::Autumn),
            Path::new("assets")
                .join("glade")
                .join("autumn")
                .join("colors.json")
        );
    }

    #[test]
    fn update_loads_base_then_theme_override_uploads_and_writes_cpu_copy() {
        let root = tmp_root("update");
        write(
            &root.join("colors.json"),
            r#"[{"name":"path","color":[0.1,0.1,0.1]},{"name":"grass_1","color":[0.2,0.2,0.2]}]"#,
        );
        write(
            &theme_colors_path(&root, GladeTheme::Winter),
            r#"[{"name":"grass_1","color":[0.9,0.9,0.9]}]"#,
        );
        let (ssbos, dev) = registered();
        let mut cpu = ColorLibrarySsbo::default();
        cpu.colors[0] = [9.0; 4];
        update_color_library_in(&root, GladeTheme::Winter, &ssbos, &dev, &mut cpu);
        assert_eq!(cpu.path(), Vec4::new(0.1, 0.1, 0.1, 1.0));
        assert_eq!(
            cpu.grass_1(),
            Vec4::new(0.9, 0.9, 0.9, 1.0),
            "主题文件覆盖基础文件"
        );
        assert_eq!(
            cpu.temp_color_1(),
            Vec4::new(1.0, 0.0, 1.0, 1.0),
            "每次从品红重新开始，不继承旧 CPU 副本"
        );
    }

    #[test]
    fn update_without_theme_file_uses_base_only() {
        let root = tmp_root("nothemefile");
        write(
            &root.join("colors.json"),
            r#"[{"name":"path","color":[0.3,0.3,0.3]}]"#,
        );
        let (ssbos, dev) = registered();
        let mut cpu = ColorLibrarySsbo::default();
        update_color_library_in(&root, GladeTheme::Summer, &ssbos, &dev, &mut cpu);
        assert_eq!(cpu.path(), Vec4::new(0.3, 0.3, 0.3, 1.0));
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn update_missing_base_file_panics() {
        let root = tmp_root("nobase");
        let (ssbos, dev) = registered();
        update_color_library_in(
            &root,
            GladeTheme::Summer,
            &ssbos,
            &dev,
            &mut ColorLibrarySsbo::default(),
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn update_with_unregistered_buffer_panics() {
        let root = tmp_root("noreg");
        write(&root.join("colors.json"), "[]");
        let ssbos = AssetSsboLibrary::new();
        update_color_library_in(
            &root,
            GladeTheme::Summer,
            &ssbos,
            &Device(1),
            &mut ColorLibrarySsbo::default(),
        );
    }

    #[test]
    fn update_failure_does_not_touch_cpu_copy() {
        // 缓冲小于 0x250 → 越界 → unwrap panic，且 memcpy 回 CPU 副本在 unwrap 之后，故副本保持不变
        use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc};
        struct Tiny;
        impl GraphicsDevice for Tiny {
            fn create_buffer(
                &self,
                _: &BufferDesc,
                _: &str,
                _: Option<&[u8]>,
            ) -> Result<Buffer, BackendError> {
                Ok(Buffer {
                    byte_size: 16,
                    ..Buffer::default()
                })
            }
            fn update_buffer_bytes(
                &self,
                _: &Buffer,
                _: &[u8],
                _: &[core::ops::Range<usize>],
                _: usize,
            ) -> Result<(), BackendError> {
                Ok(())
            }
        }
        let root = tmp_root("tiny");
        write(
            &root.join("colors.json"),
            r#"[{"name":"path","color":[0.3,0.3,0.3]}]"#,
        );
        let ssbos = AssetSsboLibrary::new();
        ssbos.add_new::<ColorLibrarySsbo>(&Tiny);
        let mut cpu = ColorLibrarySsbo::default();
        cpu.colors[0] = [5.0; 4];
        let before = cpu;
        let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            update_color_library_in(&root, GladeTheme::Summer, &ssbos, &Tiny, &mut cpu);
        }));
        let msg = *r.unwrap_err().downcast::<String>().unwrap();
        assert!(msg.contains("OutOfBounds"), "{msg}");
        assert_eq!(cpu, before);
    }
}
