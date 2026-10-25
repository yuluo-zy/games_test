//! 着色器程序注册表。光栅程序按名字与描述去重，计算程序按源码哈希索引。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

#![allow(clippy::collapsible_if)]
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::frozen_map::FrozenMap;
use crate::render::shader::{ComputeShaderProgram, RasterShaderProgram};

/// 计算程序的源码身份，不与资产句柄混用。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputeShaderSourceHash {
    pub hash: u64,
}

impl ComputeShaderSourceHash {
    pub fn from_hash(hash: u64) -> Self {
        ComputeShaderSourceHash { hash }
    }
}

pub type NoHashHasher = nohash_hasher::NoHashHasher<u64>;

pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

/// 保存已编译程序，并维护业务名字到句柄的索引。
/// One Arc owner of the current App's physical shader registry. Consumers in
/// other crates borrow the same TypeId; no second AssetShaderLibrary is created.
#[derive(bevy_ecs::resource::Resource, Clone)]
pub struct SharedAssetShaderLibrary(pub std::sync::Arc<AssetShaderLibrary>);

pub struct AssetShaderLibrary {
    pub raster_shaders: FrozenMap<Handle<RasterShaderProgram>, RasterShaderProgram>,
    pub compute_shaders: FrozenMap<Handle<ComputeShaderProgram>, ComputeShaderProgram>,
    raster_names: RwLock<HashMap<&'static str, Handle<RasterShaderProgram>>>,
    compute_names:
        RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>, NoHashBuildHasher>>,
}

impl AssetShaderLibrary {
    /// 创建初始状态。
    pub fn new() -> Self {
        AssetShaderLibrary {
            raster_shaders: FrozenMap::new(),
            compute_shaders: FrozenMap::new(),
            raster_names: RwLock::new(HashMap::new()),
            compute_names: RwLock::new(HashMap::with_hasher(NoHashBuildHasher::default())),
        }
    }

    /// 按名字和描述复用程序；强制新建时发布新的名字映射。
    pub fn add_raster(
        &self,
        program: RasterShaderProgram,
        force_new: bool,
    ) -> Handle<RasterShaderProgram> {
        let mut names = self.raster_names.write();
        let name = program.name;
        if !force_new && !names.is_empty() {
            if let Some(&existing) = names.get(name) {
                let existing_program = self.raster_shaders.get(&existing).unwrap();
                if existing_program.desc == program.desc {
                    drop(names);
                    drop(program);
                    return existing;
                }
                panic!(
                    "AssetShaderLibrary: the shader name {} is already taken",
                    name
                );
            }
        }
        let handle = Handle::new();
        names.insert(name, handle);
        let inserted = self.raster_shaders.insert(handle, Box::new(program));
        assert!(
            inserted.is_ok(),
            "assertion failed: self.raster_shaders.insert(handle, Box::new(shader)).is_ok()"
        );
        handle
    }

    /// 登记计算程序；重复源码身份视为注册冲突。
    pub fn add_compute(
        &self,
        program: ComputeShaderProgram,
        hash: ComputeShaderSourceHash,
    ) -> Handle<ComputeShaderProgram> {
        let mut names = self.compute_names.write();
        if !names.is_empty() && names.contains_key(&hash) {
            panic!(
                "AssetShaderLibrary: the shader name {:?} is already taken",
                hash
            );
        }
        let handle = Handle::new();
        names.insert(hash, handle);
        let inserted = self.compute_shaders.insert(handle, Box::new(program));
        assert!(
            inserted.is_ok(),
            "assertion failed: self.compute_shaders.insert(handle, Box::new(shader)).is_ok()"
        );
        handle
    }

    pub fn get_compute_by_name(
        &self,
        hash: &ComputeShaderSourceHash,
    ) -> Option<(Handle<ComputeShaderProgram>, &ComputeShaderProgram)> {
        let handle = {
            let names = self.compute_names.read();
            if names.is_empty() {
                None
            } else {
                names.get(hash).copied()
            }
        }?;
        self.compute_shaders
            .get(&handle)
            .map(|program| (handle, program))
    }

    pub fn get_raster_handle_by_name(&self, name: &str) -> Option<Handle<RasterShaderProgram>> {
        let names = self.raster_names.read();
        if names.is_empty() {
            return None;
        }
        names.get(name).copied()
    }

    pub fn debug_raster_get_name(
        &self,
        handle: Handle<RasterShaderProgram>,
    ) -> Option<&'static str> {
        let names = self.raster_names.read();
        names
            .iter()
            .find(|(_, h)| **h == handle)
            .map(|(name, _)| *name)
    }

    pub fn get_raster(&self, handle: Handle<RasterShaderProgram>) -> Option<&RasterShaderProgram> {
        self.raster_shaders.get(&handle)
    }

    pub fn get_raster_by_id(&self, raw: u64) -> Option<&RasterShaderProgram> {
        self.get_raster(Handle {
            id: super::HandleId::from_raw(raw),
            _marker: core::marker::PhantomData,
        })
    }

    pub fn get_compute(
        &self,
        handle: Handle<ComputeShaderProgram>,
    ) -> Option<&ComputeShaderProgram> {
        self.compute_shaders.get(&handle)
    }

    pub fn get_compute_by_id(&self, raw: u64) -> Option<&ComputeShaderProgram> {
        self.get_compute(Handle {
            id: super::HandleId::from_raw(raw),
            _marker: core::marker::PhantomData,
        })
    }
}

impl Default for AssetShaderLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl bevy_ecs::resource::Resource for AssetShaderLibrary {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::shader_uri::ComputePipelineDesc;
    use crate::render::test_device::TestDevice as Device;
    use crate::rhapsody_types::RasterPipelineDesc;

    fn raster(name: &'static str, path: &str) -> RasterShaderProgram {
        let desc = RasterPipelineDesc {
            path: path.to_owned(),
            ..Default::default()
        };
        RasterShaderProgram::new_draw(name, desc, &Device(1))
    }

    fn compute(name: &'static str, path: &str) -> ComputeShaderProgram {
        let desc = ComputePipelineDesc {
            path: path.to_owned(),
            ..Default::default()
        };
        ComputeShaderProgram::new(name, desc, &Device(1))
    }

    fn key(h: u64) -> ComputeShaderSourceHash {
        ComputeShaderSourceHash::from_hash(h)
    }

    #[test]
    fn new_is_empty() {
        let lib = AssetShaderLibrary::new();
        assert_eq!(lib.raster_shaders.len(), 0);
        assert_eq!(lib.compute_shaders.len(), 0);
        assert!(lib.get_raster_handle_by_name("x").is_none());
        assert!(lib.get_compute_by_name(&key(1)).is_none());
    }

    #[test]
    fn add_raster_registers_name_and_program() {
        let lib = AssetShaderLibrary::new();
        let h = lib.add_raster(raster("sky", "/sky.hlsl"), false);
        assert_eq!(lib.get_raster_handle_by_name("sky"), Some(h));
        assert_eq!(lib.get_raster(h).unwrap().name, "sky");
        assert_eq!(lib.raster_shaders.len(), 1);
        assert_eq!(lib.debug_raster_get_name(h), Some("sky"));
    }

    #[test]
    fn add_raster_handles_are_distinct_and_increasing() {
        let lib = AssetShaderLibrary::new();
        let a = lib.add_raster(raster("a", "/a"), false);
        let b = lib.add_raster(raster("b", "/b"), false);
        assert!(b.id().raw() > a.id().raw());
        assert_eq!(lib.raster_shaders.len(), 2);
    }

    #[test]
    fn add_raster_same_name_same_desc_returns_existing() {
        let lib = AssetShaderLibrary::new();
        let a = lib.add_raster(raster("sky", "/sky.hlsl"), false);
        let b = lib.add_raster(raster("sky", "/sky.hlsl"), false);
        assert_eq!(a, b);
        assert_eq!(lib.raster_shaders.len(), 1, "新程序被丢弃，不入库");
    }

    #[test]
    #[should_panic(expected = "AssetShaderLibrary: the shader name sky is already taken")]
    fn add_raster_same_name_different_desc_panics() {
        let lib = AssetShaderLibrary::new();
        lib.add_raster(raster("sky", "/sky.hlsl"), false);
        lib.add_raster(raster("sky", "/other.hlsl"), false);
    }

    #[test]
    fn add_raster_force_new_skips_dedup_and_remaps_name() {
        let lib = AssetShaderLibrary::new();
        let a = lib.add_raster(raster("sky", "/sky.hlsl"), false);
        let b = lib.add_raster(raster("sky", "/other.hlsl"), true);
        assert_ne!(a, b);
        assert_eq!(
            lib.get_raster_handle_by_name("sky"),
            Some(b),
            "名字映射被覆盖"
        );
        assert_eq!(lib.raster_shaders.len(), 2, "旧程序仍在库中");
        assert_eq!(lib.debug_raster_get_name(a), None, "旧 handle 已无名字");
        assert_eq!(lib.debug_raster_get_name(b), Some("sky"));
    }

    #[test]
    fn add_compute_registers_and_looks_up() {
        let lib = AssetShaderLibrary::new();
        let h = lib.add_compute(compute("taa", "/tgsr.cs.hlsl"), key(0xdead_beef));
        let (got, prog) = lib.get_compute_by_name(&key(0xdead_beef)).unwrap();
        assert_eq!(got, h);
        assert_eq!(prog.name, "taa");
        assert!(lib.get_compute_by_name(&key(0xdead_bee0)).is_none());
        assert_eq!(lib.get_compute(h).unwrap().name, "taa");
    }

    #[test]
    #[should_panic(
        expected = "AssetShaderLibrary: the shader name ComputeShaderSourceHash { hash: 7 } is already taken"
    )]
    fn add_compute_duplicate_hash_panics_even_if_program_differs() {
        let lib = AssetShaderLibrary::new();
        lib.add_compute(compute("a", "/a"), key(7));
        lib.add_compute(compute("b", "/b"), key(7));
    }

    #[test]
    fn compute_and_raster_share_one_handle_counter() {
        let lib = AssetShaderLibrary::new();
        let r = lib.add_raster(raster("r", "/r"), false);
        let c = lib.add_compute(compute("c", "/c"), key(1));
        assert!(c.id().raw() > r.id().raw());
    }

    #[test]
    fn get_compute_by_name_none_when_program_missing() {
        // 名字表有项而 FrozenMap 没有对应程序：返回 None（原版不 unwrap）。
        let lib = AssetShaderLibrary::new();
        lib.compute_names.write().insert(key(5), Handle::new());
        assert!(lib.get_compute_by_name(&key(5)).is_none());
    }

    #[test]
    fn debug_raster_get_name_scans_all_entries() {
        let lib = AssetShaderLibrary::new();
        let hs: Vec<_> = ["a", "b", "c", "d", "e"]
            .iter()
            .map(|n| lib.add_raster(raster(n, n), false))
            .collect();
        for (h, n) in hs.iter().zip(["a", "b", "c", "d", "e"]) {
            assert_eq!(lib.debug_raster_get_name(*h), Some(n));
        }
        assert_eq!(lib.debug_raster_get_name(Handle::new()), None);
    }

    #[test]
    fn no_hash_hasher_uses_key_as_hash() {
        use std::hash::BuildHasher;
        let b = NoHashBuildHasher::default();
        assert_eq!(b.hash_one(key(0x1234)), 0x1234);
    }
}
