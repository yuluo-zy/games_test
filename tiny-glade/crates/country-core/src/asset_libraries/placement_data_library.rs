//! 摆放数据注册表。按路径去重，保存数据缓冲、元素数与资产句柄。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use std::collections::HashMap;
use std::path::PathBuf;

use bevy_ecs::resource::Resource;

use crate::asset_libraries::Handle;
use crate::render::ssbo::TypedStorageBuffer;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::resources::placement_data::PlacementDataSsbo;

/// 持有已加载的摆放数据缓冲及来源索引。
#[derive(Resource)]
pub struct AssetPlacementDataLibrary {
    pub buffers: HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)>,
    pub paths: HashMap<PathBuf, Handle<PlacementDataSsbo>>,
}

impl AssetPlacementDataLibrary {
    /// 创建初始状态。
    pub fn new() -> Self {
        AssetPlacementDataLibrary {
            buffers: HashMap::new(),
            paths: HashMap::new(),
        }
    }

    /// B uninit_clearing@140a7a5fd..60c drops buffers before path keys.
    /// Retain table allocations, and let each typed buffer release its real lease.
    pub fn clear(&mut self) {
        self.buffers.clear();
        self.paths.clear();
    }

    /// 加载并注册摆放数据，同一路径不能重复加载。
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
        let json_text = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Couldn't find {}", path.display()));
        let placements: Vec<PlacementDataSsbo> = serde_json::from_str(&json_text).unwrap();
        let storage_buffer = TypedStorageBuffer::new(device, Some(&placements));
        let handle = Handle::new();
        self.buffers
            .insert(handle, (storage_buffer, placements.len()));
        self.paths.insert(path, handle);
        (placements, handle)
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
    use crate::render::test_device::TestDevice as Device;

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
        let f = tmp_file(
            "ok",
            r#"[{"position":[1,0,2],"scale":1,"rotation_angle":0,"id":1},{"position":[3,0,4],"scale":2,"rotation_angle":90,"id":2}]"#,
        );
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
