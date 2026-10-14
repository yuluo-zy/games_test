//! `asset_libraries/placement_data_library.rs` —— 放置数据（prefab 摆放记录）注册表（A3 逐函数核对版）。
//!
//! 原版 3 个过程（O）：
//!  * `AssetPlacementDataLibrary::new` 0x140867700（328 B）
//!  * `AssetPlacementDataLibrary::load<PathBuf>` 0x1409365d0（1674 B）
//!  * `impl$2::register_required_components` 0x14086c3d0（1776 B）= bevy `#[derive(Resource)]` 生成，不重写
//!  * 另有 `drop_in_place<AssetPlacementDataLibrary>` 0x1405d0340（两张表的 drop，由 Rust 自动生成）
//!
//! 结构（O，`new` 写入 0x60 B；`drop_inner_table<(Handle<PlacementDataSsbo>, (TypedStorageBuffer<..>, usize))>`
//! 给出元素类型，`load` 的两次 `HashMap::insert` 给出归属）：
//! ```text
//!   +0x00  HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)>   // 表 0x20 B，条目 0x38 B
//!   +0x20  RandomState (k0, k1)
//!   +0x30  HashMap<PathBuf, Handle<PlacementDataSsbo>>                                           // 表 0x20 B，条目 0x28 B
//!   +0x50  RandomState (k0 + 1, k1)
//! ```
//! `new` 两次读线程局部 `RandomState::KEYS`（每次读完 `k0 += 1` 写回），所以两张表的哈希种子不同；
//! 两张表都是空表（静态空 ctrl 组）。条目里的 `usize` 是 JSON 数组的元素个数（`load` 把 `len` 存进去）。
//! `Handle<T>` = 单个 u64 id（`xadd` 全局计数器取旧值，即 `Handle::new`）。
//!
//! `load<PathBuf>(&mut self, device, path) -> (Vec<PlacementDataSsbo>, Handle<PlacementDataSsbo>)`（O，返回 0x20 B：
//! Vec 24 B + handle 8 B）：
//!  1. 路径表非空时按 `PathBuf` 查重；命中 → `panic!("AssetPlacementDataLibrary: the placement data from {} has already
//!     been loaded", path.display())`（源码行 93 列 13；pieces @0x142ab0fd0）；
//!  2. `fs::read_to_string(&path)` 失败 → `panic!("Couldn't find {}", path.display())`（行 100 列 63；pieces @0x142ab1018，
//!     io 错误被丢弃）；
//!  3. `serde_json::from_str::<Vec<PlacementDataSsbo>>(&text).unwrap()`（行 101 列 73）；
//!  4. `TypedStorageBuffer::new(device, Some(&data))`；
//!  5. `handle = Handle::new()`；先 `buffers.insert(handle, (buffer, len))`（返回旧值则 drop，新 handle 唯一故恒无旧值），
//!     再 `paths.insert(path, handle)`；
//!  6. 返回 `(data, handle)`，文本缓冲释放。
//!
//! 旧版的重大错误：`HashMap<PathBuf, PlacementData>` 单表 + 假 `PlacementData`/`Gap` 载荷；`load` 返回 `Result<(),()>`
//! 且根本不读文件；`add`/`clear` 在原版并不存在（O：本模块只有 `new`/`load` 两个过程，`clear` 若存在也是别处内联）。
//!
//! 对外部类型的要求（本文件不拥有）：`resources::placement_data::PlacementDataSsbo` 需要 `serde::Deserialize`
//! （O：JSON 是自定义结构，含 u32/f32/i32/[f32;3]/[f32;2] 字段和 `utils::pack_2x16f_uint` 调用，布局与现有重建的
//! 字段不同，属 WP21 的核对范围）以及 `StorageElement`（`BUFFER_LEN = 0`，因为 `new` 以 `Some(data)` 创建）。
//!
//! 仍为 I/U：`PlacementDataSsbo` 的 JSON schema（U，非本文件）；`Handle::new` 使用的计数器是否与
//! `HANDLE_COUNT` 同一个（0x143634cd0，U；沿用 `asset_libraries::Handle::new`）。

use std::collections::HashMap;
use std::path::PathBuf;

use bevy_ecs::resource::Resource;

use crate::asset_libraries::Handle;
use crate::render::ssbo::TypedStorageBuffer;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::resources::placement_data::PlacementDataSsbo;

/// 放置数据注册表（O：0x60 B，两张带 `RandomState` 的 hashbrown 表）。
#[derive(Resource)]
pub struct AssetPlacementDataLibrary {
    /// +0x00：handle → (GPU 缓冲, 元素个数)。
    pub buffers: HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)>,
    /// +0x30：已加载文件 → handle（`load` 用它查重）。
    pub paths: HashMap<PathBuf, Handle<PlacementDataSsbo>>,
}

impl AssetPlacementDataLibrary {
    /// O 0x140867700：两张空表，各自一份 `RandomState::new()`（k0 逐次 +1，由 `HashMap::new` 保证）。
    pub fn new() -> Self {
        AssetPlacementDataLibrary {
            buffers: HashMap::new(),
            paths: HashMap::new(),
        }
    }

    /// O 0x1409365d0：见模块文档。
    pub fn load(
        &mut self,
        device: &dyn GraphicsDevice,
        path: PathBuf,
    ) -> (Vec<PlacementDataSsbo>, Handle<PlacementDataSsbo>) {
        if !self.paths.is_empty() && self.paths.contains_key(&path) {
            panic!(
                "AssetPlacementDataLibrary: the placement data from {} has already been loaded",
                path.display()
            );
        }
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Couldn't find {}", path.display()));
        let data: Vec<PlacementDataSsbo> = serde_json::from_str(&text).unwrap();
        let buffer = TypedStorageBuffer::new(device, Some(&data));
        let handle = Handle::new();
        self.buffers.insert(handle, (buffer, data.len()));
        self.paths.insert(path, handle);
        (data, handle)
    }
}

impl Default for AssetPlacementDataLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rhapsody_contract::GraphicsDevice as Device;

    fn tmp_file(tag: &str, text: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("placement_lib_{}_{}", tag, std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let f = d.join("placement.json");
        std::fs::write(&f, text).unwrap();
        f
    }

    #[test]
    fn new_has_two_empty_tables() {
        let lib = AssetPlacementDataLibrary::new();
        assert!(lib.buffers.is_empty());
        assert!(lib.paths.is_empty());
    }

    #[test]
    fn load_registers_buffer_and_path_and_returns_data_with_handle() {
        let f = tmp_file("ok", r#"[{"x":1.0,"y":2.0},{"x":3.0,"y":4.0}]"#);
        let mut lib = AssetPlacementDataLibrary::new();
        let (data, handle) = lib.load(&Device(1), f.clone());
        assert_eq!(data.len(), 2);
        assert_eq!(lib.paths.get(&f), Some(&handle));
        let (buf, len) = lib.buffers.get(&handle).unwrap();
        assert_eq!(*len, 2, "条目里的 usize 是元素个数");
        // BUFFER_LEN == 0：缓冲大小 = 元素数 * size_of::<T>()
        assert_eq!(
            buf.byte_size(),
            2 * core::mem::size_of::<PlacementDataSsbo>()
        );
    }

    #[test]
    fn each_load_gets_a_fresh_handle() {
        let a = tmp_file("a", "[]");
        let b = tmp_file("b", "[]");
        let mut lib = AssetPlacementDataLibrary::new();
        let (_, ha) = lib.load(&Device(1), a);
        let (_, hb) = lib.load(&Device(1), b);
        assert_ne!(ha, hb);
        assert_eq!(lib.buffers.len(), 2);
        assert_eq!(lib.paths.len(), 2);
    }

    #[test]
    fn empty_array_creates_zero_element_entry() {
        let f = tmp_file("empty", "[]");
        let mut lib = AssetPlacementDataLibrary::new();
        let (data, handle) = lib.load(&Device(1), f);
        assert!(data.is_empty());
        assert_eq!(lib.buffers.get(&handle).unwrap().1, 0);
    }

    #[test]
    #[should_panic(expected = "AssetPlacementDataLibrary: the placement data from ")]
    fn loading_the_same_path_twice_panics() {
        let f = tmp_file("dup", "[]");
        let mut lib = AssetPlacementDataLibrary::new();
        lib.load(&Device(1), f.clone());
        lib.load(&Device(1), f);
    }

    #[test]
    #[should_panic(expected = "has already been loaded")]
    fn duplicate_message_tail() {
        let f = tmp_file("dup2", "[]");
        let mut lib = AssetPlacementDataLibrary::new();
        lib.load(&Device(1), f.clone());
        lib.load(&Device(1), f);
    }

    #[test]
    #[should_panic(expected = "Couldn't find ")]
    fn missing_file_panics_with_couldnt_find() {
        AssetPlacementDataLibrary::new()
            .load(&Device(1), PathBuf::from("/nonexistent/placement.json"));
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn bad_json_panics_via_unwrap() {
        let f = tmp_file("bad", "{not json");
        AssetPlacementDataLibrary::new().load(&Device(1), f);
    }

    #[test]
    fn failed_load_leaves_tables_untouched() {
        let f = tmp_file("bad2", "nope");
        let mut lib = AssetPlacementDataLibrary::new();
        let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            lib.load(&Device(1), f);
        }));
        assert!(r.is_err());
        assert!(lib.buffers.is_empty() && lib.paths.is_empty());
    }
}
