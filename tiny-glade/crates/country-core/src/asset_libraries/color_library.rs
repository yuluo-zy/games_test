//! 主题调色板。加载基础颜色与主题覆盖，GPU 上传成功后发布 CPU 副本。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

#![allow(clippy::single_range_in_vec_init)]
use std::path::{Path, PathBuf};

use bevy_ecs::resource::Resource;
use glam::Vec4;
use serde::Deserialize;

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};

pub const COLOR_ENTRY_COUNT: usize = 37;

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

pub const MISSING_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 1.0];

/// 37 个 RGBA 槽，布局与 GPU 调色板保持一致。
#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, PartialEq, bytemuck::NoUninit)]
pub struct ColorLibrarySsbo {
    pub colors: [[f32; 4]; COLOR_ENTRY_COUNT],
}

impl Default for ColorLibrarySsbo {
    fn default() -> Self {
        ColorLibrarySsbo {
            colors: [MISSING_COLOR; COLOR_ENTRY_COUNT],
        }
    }
}

impl StorageElement for ColorLibrarySsbo {
    const NAME: &'static str = "ColorLibrary";
    const BUFFER_LEN: usize = 1;

    fn default_value() -> Option<Self> {
        Some(Self::default())
    }
}

impl SsboType for ColorLibrarySsbo {
    type Storage = TypedStorageBuffer<ColorLibrarySsbo>;

    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        match initial {
            None => TypedStorageBuffer::<ColorLibrarySsbo>::new(device, None),
            Some(bytes) => {
                TypedStorageBuffer::<ColorLibrarySsbo>::new_storage_buffer(device, Some(bytes))
            }
        }
    }
}

/// 配置中的颜色条目，alpha 在加载时固定为 1。
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ColorFromJson {
    pub name: String,
    pub color: [f32; 3],
}

pub fn slot_index(name: &str) -> Option<usize> {
    COLOR_NAMES.iter().position(|n| *n == name)
}

macro_rules! color_getters {
    ($($name:ident => $idx:expr),* $(,)?) => {
        impl ColorLibrarySsbo {
            $(
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
    /// 按输入顺序覆盖命名颜色槽，alpha 固定为 1。
    pub fn deserialize_from(&mut self, entries: Vec<ColorFromJson>) {
        self.try_deserialize_from(entries).expect("加载调色板失败");
    }

    /// 在栈上构建候选副本，未知名字或非法数值不会留下部分写入。
    pub fn try_deserialize_from(&mut self, entries: Vec<ColorFromJson>) -> anyhow::Result<()> {
        let mut next = *self;
        for entry in entries {
            let slot = slot_index(&entry.name).ok_or_else(|| {
                anyhow::anyhow!(
                    "Init Color Library failed: {} entry is missing from ColorLibrarySSBO",
                    entry.name
                )
            })?;
            anyhow::ensure!(
                entry.color.iter().all(|channel| channel.is_finite()),
                "颜色必须为有限数值"
            );
            let [red, green, blue] = entry.color;
            next.colors[slot] = [red, green, blue, 1.0];
        }
        *self = next;
        Ok(())
    }

    /// 读取 JSON 颜色条目。
    pub fn deserialize_from_path<P: AsRef<Path>>(&mut self, path: P) {
        let text = std::fs::read_to_string(path).unwrap();
        let entries: Vec<ColorFromJson> = serde_json::from_str(&text).unwrap();
        self.deserialize_from(entries);
    }
}

/// 主题目录选择。
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
    pub fn dir_name(self) -> &'static str {
        ["summer", "olden", "flowery", "autumn", "winter"][self as usize]
    }
}

pub const ASSETS_ROOT: &str = "assets";

pub fn theme_colors_path(root: &Path, theme: GladeTheme) -> PathBuf {
    root.join("glade")
        .join(theme.dir_name())
        .join("colors.json")
}

/// 使用默认资源目录更新主题调色板。
pub fn update_color_library(
    theme: GladeTheme,
    ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    cpu_palette: &mut ColorLibrarySsbo,
) {
    update_color_library_in(Path::new(ASSETS_ROOT), theme, ssbos, device, cpu_palette)
}

/// 加载并上传主题调色板，成功后才发布 CPU 状态。
pub fn update_color_library_in(
    root: &Path,
    theme: GladeTheme,
    ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    cpu_palette: &mut ColorLibrarySsbo,
) {
    try_update_color_library_in(root, theme, ssbos, device, cpu_palette).unwrap();
}

/// 可恢复的主题更新入口，供文件监听消费者使用。
pub fn try_update_color_library_in(
    root: &Path,
    theme: GladeTheme,
    ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    cpu_palette: &mut ColorLibrarySsbo,
) -> anyhow::Result<()> {
    fn apply(palette: &mut ColorLibrarySsbo, path: &Path) -> anyhow::Result<()> {
        use anyhow::Context;
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("读取颜色失败：{}", path.display()))?;
        let entries: Vec<ColorFromJson> = serde_json::from_str(&text)?;
        palette.try_deserialize_from(entries)
    }
    let mut palette = ColorLibrarySsbo::default();
    apply(&mut palette, &root.join("colors.json"))?;
    let theme_path = theme_colors_path(root, theme);
    match std::fs::metadata(&theme_path) {
        Ok(_) => apply(&mut palette, &theme_path)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    let buffer = ssbos
        .get_buffer::<ColorLibrarySsbo>()
        .ok_or_else(|| anyhow::anyhow!("调色板缓冲未注册"))?;
    let bytes = bytemuck::cast_slice(&palette.colors);
    anyhow::ensure!(
        buffer.byte_size >= bytes.len() as u64,
        "调色板 GPU 缓冲过小"
    );
    device.update_buffer_bytes(buffer, bytes, &[0..bytes.len()], 0)?;
    *cpu_palette = palette;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::test_device::TestDevice as Device;

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
    fn unknown_name_does_not_publish_a_partial_palette() {
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
            Vec4::from_array(MISSING_COLOR),
            "失败时必须保留整张旧调色板"
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
    #[should_panic(expected = "调色板缓冲未注册")]
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
        assert!(msg.contains("调色板 GPU 缓冲过小"), "{msg}");
        assert_eq!(cpu, before);
    }
}
