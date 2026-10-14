//! `AssetShaderLibrary` —— shader 程序注册表（layer A3）。
//!
//! 证据（O = 机器码直接观察；I = 推断；U = 未知；详见 `evidence/country-core-audit/A3_shader.md`）。
//! 原版 6 个过程（0x140936c60 起，共 4.5 KB）：
//!  * `new`                        0x140936c60（656 B）
//!  * `add_raster`                 0x140936ef0（1639 B）
//!  * `add_compute`                0x140937560（886 B）
//!  * `get_compute_by_name`        0x1409378e0（446 B）
//!  * `get_raster_handle_by_name`  0x140937aa0（525 B）
//!  * `debug_raster_get_name`      0x140937cb0（352 B）
//!
//! 布局（O，0xe0 B，由 `new` 的写入偏移与各入口的读取偏移交叉确认）：
//!  * `+0x00` `raster_shaders`：`frozen::FrozenMap<Handle<RasterShaderProgram>, Box<RasterShaderProgram>>`（0x40 B）；
//!    `FrozenMap::insert`/`get` 的单态化名称含 `Handle<RasterShaderProgram>` + `Box<RasterShaderProgram>`。
//!  * `+0x40` `compute_shaders`：同上，`Handle<ComputeShaderProgram>`（`add_compute` 里 `add rsi, 0x40`）。
//!  * `+0x80` `raster_names`：`parking_lot::RwLock<HashMap<&'static str, Handle<RasterShaderProgram>, RandomState>>`
//!    （锁字 +0x80、ctrl +0x88、bucket_mask +0x90、growth_left +0x98、items +0xa0、`RandomState` 密钥 +0xa8/+0xb0；
//!    桶 0x18 B = `&str`(ptr,len) + `Handle`(u64)；键 `hash_one<RandomState, &str>`）。
//!    **键是借用的 `&'static str`，来自程序自身的 `name` 字段**（`RasterShaderProgram` +0x48/+0x50），不是拥有的 `String`。
//!  * `+0xb8` `compute_names`：`RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>, NoHash>>`
//!    （锁字 +0xb8、ctrl +0xc0、mask +0xc8、items +0xd8；桶 0x10 B = key(u64) + Handle(u64)；
//!    探测里直接把 key 当哈希：`h2 = key >> 57`、`pos = key & mask`，即 NoHashHasher）。
//!  * `new`：两张名字表的 `RandomState` 经线程局部 `KEYS` 取得并把 k0 自增（`std::hash::random`，标准库行为），
//!    锁字全 0，表为空。**没有任何别的初始化**。
//!
//! 与旧版（`臆造`）的主要差异：
//!  1. 所有入口取 `&self`（内部可变性：名字表的 `RwLock` + `FrozenMap::insert`），旧版 `add_*` 取 `&mut self`；
//!  2. `add_raster(&self, program, force_new: bool)`：**程序按值传入，名字取自 `program.name`**，第二形参是 bool
//!     （`test bl, bl` 为真则跳过查重）。旧版的 `(name, program)` 形参是臆造；
//!  3. 查重相等的判据是 `existing.desc == program.desc`（`RasterPipelineDesc` 的逐字段比较：path、Option<String>、
//!     Vec<(String,String)>），而不是旧版臆造的 “source_name + compiled_variants.base_pipeline”；相等 → 丢弃新程序、
//!     返回已有 `Handle`；不等 → `panic!("AssetShaderLibrary: the shader name {name} is already taken")`
//!     （pieces `"AssetShaderLibrary: the shader name "` + `" is already taken"`，`{}` = `<str as Display>`，
//!     位置 shader_library.rs:62:21）；
//!  4. `add_compute(&self, program, hash)`：**不比较内容**，键已存在即 `panic!("AssetShaderLibrary: the shader name
//!     {hash:?} is already taken")`（pieces 与 raster 共用同一份常量，参数是 `ComputeShaderSourceHash` 的 derive(Debug)，
//!     位置 :89:13）。旧版的 `"{:016x} … already in the library"` 文案是臆造；
//!  5. 句柄来自全局计数器（0x143634cd0，`lock xadd 1`）= `Handle::new()`；写入顺序：先 `names.insert`，再 `Box`
//!     程序，再 `FrozenMap::insert`，结果 `Err` → `assertion failed: self.raster_shaders.insert(handle,
//!     Box::new(shader)).is_ok()`（:75:9）/ `…compute_shaders…`（:100:9）；整个过程持名字表的**写锁**；
//!  6. `get_compute_by_name(&self, &ComputeShaderSourceHash) -> Option<(Handle, &ComputeShaderProgram)>`（返回 rax:rdx，
//!     rdx = 程序指针，空 = None）：读锁查名字表 → **解锁** → `FrozenMap::get(handle)`，缺失返回 None（不 unwrap）。
//!     旧版只返回 handle；
//!  7. `get_raster_handle_by_name(&self, &str) -> Option<Handle>`（rax=是否命中、rdx=handle）；
//!  8. `debug_raster_get_name(&self, Handle) -> Option<&'static str>`：持读锁**线性扫描**整张名字表，找值 == handle 的桶，
//!     返回其键（借用，非克隆的 `String`）；
//!  9. 删除旧版的臆造入口：`get_compute_handle_by_name` / `get_compute_shader_name` / `get_raster_by_name` /
//!     `get_raster_mut` / `extract_raster_shader_names`（原版没有这些过程；消费者对 `FrozenMap::get` 内联）。
//!
//! 外部库：`frozen::FrozenMap`（`crate::frozen_map`）、`parking_lot::RwLock`（原版开启 deadlock_detection：
//! `acquire_resource`/`release_resource` 成对出现，属 parking_lot 行为，不重写）、`nohash-hasher`（占位见下）。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::collapsible_if)]
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::frozen_map::FrozenMap;
use crate::render::shader::{ComputeShaderProgram, RasterShaderProgram};

/// compute 源码哈希（O：Debug 实现 0x140939420 = `debug_struct_field1_finish("ComputeShaderSourceHash",
/// "hash", &u64)`，即 `#[derive(Debug)] struct ComputeShaderSourceHash { hash: u64 }`）。
///
/// 原版类型在 `render::pass_builder::shader_uri`（`ComputeShaderSourceHash::compute(&str) -> u64` 的算法见该文件）；
/// `shader_uri.rs` 里的同名单元结构只承载 `compute`，不能当键，故此处另放键类型（调用方 `.compute()` → `from_hash`）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputeShaderSourceHash {
    pub hash: u64,
}

impl ComputeShaderSourceHash {
    pub fn from_hash(hash: u64) -> Self {
        ComputeShaderSourceHash { hash }
    }
}

/// Original dependency reused; game policy selects integer source hashes as keys.
pub type NoHashHasher = nohash_hasher::NoHashHasher<u64>;

pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

/// shader 程序库（O，布局见模块注释）。
pub struct AssetShaderLibrary {
    /// +0x00。
    pub raster_shaders: FrozenMap<Handle<RasterShaderProgram>, RasterShaderProgram>,
    /// +0x40。
    pub compute_shaders: FrozenMap<Handle<ComputeShaderProgram>, ComputeShaderProgram>,
    /// +0x80：名字 → handle（键借用自程序的 `name`）。
    raster_names: RwLock<HashMap<&'static str, Handle<RasterShaderProgram>>>,
    /// +0xb8：源码哈希 → handle。
    compute_names:
        RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>, NoHashBuildHasher>>,
}

impl AssetShaderLibrary {
    /// `new`（O 0x140936c60）：全空。
    pub fn new() -> Self {
        AssetShaderLibrary {
            raster_shaders: FrozenMap::new(),
            compute_shaders: FrozenMap::new(),
            raster_names: RwLock::new(HashMap::new()),
            compute_names: RwLock::new(HashMap::with_hasher(NoHashBuildHasher::default())),
        }
    }

    /// `add_raster(&self, program, force_new)`（O 0x140936ef0）。
    ///
    /// 持名字表写锁；`force_new == false` 且表非空时先查重（见模块注释第 3 点）；
    /// 然后 分配 handle → 名字表插入 → `Box` → `FrozenMap::insert`（失败 = 断言）。
    /// `force_new == true`（I：形参名未知）跳过查重，同名则覆盖名字映射，旧程序仍留在 `raster_shaders`。
    pub fn add_raster(
        &self,
        program: RasterShaderProgram,
        force_new: bool,
    ) -> Handle<RasterShaderProgram> {
        let mut names = self.raster_names.write();
        let name = program.name;
        // O：`if !force_new && names.len() != 0`（`cmp [rdi+0xa0], 0` 为空则直接跳到插入）。
        if !force_new && !names.is_empty() {
            if let Some(&existing) = names.get(name) {
                // O：`FrozenMap::get(existing).unwrap()`（0x142ab1080 处的 `Option::unwrap_failed`）。
                let cur = self.raster_shaders.get(&existing).unwrap();
                if cur.desc == program.desc {
                    // O：相等 → 先放锁再 `drop_in_place<RasterShaderProgram>(新程序)`，返回已有 handle。
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

    /// `add_compute(&self, program, hash)`（O 0x140937560）。
    ///
    /// 持写锁；键已存在 → panic（不比较程序内容）；否则 分配 handle → 名字表 → `Box` → `FrozenMap::insert`。
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

    /// `get_compute_by_name(&self, &hash)`（O 0x1409378e0）：读锁查名字表，**放锁后**再 `FrozenMap::get`。
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
        self.compute_shaders.get(&handle).map(|p| (handle, p))
    }

    /// `get_raster_handle_by_name(&self, name)`（O 0x140937aa0）：读锁 + 字符串键查找。
    pub fn get_raster_handle_by_name(&self, name: &str) -> Option<Handle<RasterShaderProgram>> {
        let names = self.raster_names.read();
        if names.is_empty() {
            return None;
        }
        names.get(name).copied()
    }

    /// `debug_raster_get_name(&self, handle)`（O 0x140937cb0）：读锁下线性扫描全部桶，返回值 == handle 的键。
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

    /// 按 handle 取 raster 程序（I：原版消费者直接内联 `FrozenMap::get`，不是独立过程）。
    pub fn get_raster(&self, handle: Handle<RasterShaderProgram>) -> Option<&RasterShaderProgram> {
        self.raster_shaders.get(&handle)
    }

    /// 按 handle 取 compute 程序（I：同上）。
    pub fn get_compute(
        &self,
        handle: Handle<ComputeShaderProgram>,
    ) -> Option<&ComputeShaderProgram> {
        self.compute_shaders.get(&handle)
    }
}

impl Default for AssetShaderLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::shader_uri::ComputePipelineDesc;
    use crate::rhapsody_contract::GraphicsDevice as Device;
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
