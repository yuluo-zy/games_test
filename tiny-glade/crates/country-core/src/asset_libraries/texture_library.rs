//! `AssetTextureLibrary` — 纹理注册表 + bindless 描述符表 + 渲染图“预设纹理”（layer A3，已逐函数对照机器码）。
//!
//! 证据全文见 `evidence/country-core-audit/A3_texture.md`；O = 机器码直接观察，I = 推断，U = 未知。
//!
//! 结构体布局（O，`AssetTextureLibrary::new` 0x14093bb10 的逐字段写入，总大小 0xc8）：
//! ```text
//! +0x00 textures : Vec<Texture>                       (stride 0x38；`add` 里 imul 0x38)
//! +0x18 dset     : DescriptorSet                      (0x18 B 托管句柄；new 里由 create_persistent_descriptor_set 写入)
//! +0x30 by_name  : HashMap<NameHash, NamedTexture, NoHashHasher>   (hashbrown 桶 0x18 B：hash 8 + NamedTexture 16)
//! +0x50 presets  : HashMap<TexturePreset, PresetTextureState, RandomState>  (桶 0x68 B)
//! +0x70           RandomState 的 (k0, k1)
//! +0x80 bindless : std::sync::RwLock<DescriptorMap>   (futex 字 @0x80，poison 字节 @0x88，数据 @0x90)
//!        DescriptorMap { pending: Vec<AssetIndex> @0x90, indices: Vec<Option<DescriptorIndex>> @0xa8, next: u32 @0xc0 }
//! ```
//! 旧版（本文件上一版）的错误：把 `by_name` 当成 `HashMap<NameHash, NamedTexture{name,texture}>`（实际值只是
//! `{ handle: AssetIndex, is_virtual: bool }`，纹理本体只存在 `textures` 向量里）；`NameHash` 用 FNV 私造；
//! `bindless` 用 parking_lot 且上限写成 0x1fdf、复用逻辑与 “pending/dirty” 全是臆造；`get_*_by_name` 取 `&str`
//! 而原版取预先算好的 `NameHash`；预设纹理四个函数全是空桩（实际是 26 个预设的创建/清零/导入/导出）。
//!
//! 偏离（占位）：
//!  * `nohash_hasher` 外部库 → 本文件内的 `NoHashHasher`（write_u64 直接存值）；
//!  * rhapsody 设备调用面（`update_texture` / `get_vk_image_view` / `update_persistent_descriptor_set` /
//!    `create_persistent_descriptor_set`）→ `TextureLibraryDevice` trait，`GraphicsDevice` 上给空实现；
//!  * `TexturePreset -> TextureDesc` 的 `From`（0xa59890，1933 B，属 `resources/texture_presets.rs`）尚未恢复，
//!    `create_preset_textures` 以闭包参数接收它（原版是直接调用）；
//!  * `rhapsody::raw::format::vk_format_info` 的 “每像素字节数” → `format_bytes_per_texel`（只覆盖预设用到的格式）。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::collapsible_if)]
// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::RwLock;

use crate::name_hash::NameHash;
use crate::render::load_texture::boundary::TextureDevice;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_contract::{DescriptorSet, GraphicsDevice};
use crate::rhapsody_types::{BackendError, ImportedBuffer, RenderGraph, Texture, TextureDesc};

// ------------------------------------------------------------------ 常量（O）

/// bindless 描述符表里可分配的索引个数上限（O：`handle_to_bindless_index` 里 `cmp r14d, 0x1fe0; jae panic`）。
/// 源码断言 `descriptor_index.0 < GraphicsDevice::REQUIRED_BINDLESS_DESCRIPTOR_COUNT as u32 - RESERVED_DESCRIPTOR_COUNT as u32`
/// 的差值是 0x1fe0；两个常量各自的值 U（按 0x2000 - 0x20 拆分，仅差值是 O）。
pub const REQUIRED_BINDLESS_DESCRIPTOR_COUNT: u32 = 0x2000;
pub const RESERVED_DESCRIPTOR_COUNT: u32 = 0x20;

/// 预设纹理个数：`enum_iterator::All<TexturePreset>` 的跳转表（0x141474bd5）依次产出 0..=0x19，共 26 个（O）。
pub const PRESET_COUNT: u8 = 0x1a;

/// 渲染图导入/导出预设纹理的访问类型字节（O：`mov r9b, 0x22` / `mov r8b, 0x22`）。
pub const RG_TEXTURE_ACCESS: u8 = 0x22;

/// `NameHash::debug_name()`（O 0x140c9cdb0）返回的常量串；`create_preset_textures` 用它给预设纹理命名。
pub const PRESET_TEXTURE_DEBUG_NAME: &str = "(NameHash)";

// ------------------------------------------------------------------ nohash 占位

/// Original nohash-hasher dependency; NameHash hashes its single u64 field.
pub type NoHashHasher = nohash_hasher::NoHashHasher<u64>;
pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

// ------------------------------------------------------------------ 数据类型

/// `texture_library::AssetIndex`：`textures` 向量里的下标（O：`add` 返回 `Vec::len()`；`get_by_name` 用它做
/// `imul 0x38` 取址）。Debug 形态 U（原版经 `asset_libraries::impl$11::fmt<Texture>` 打印）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct AssetIndex(pub usize);

/// `texture_library::NamedTexture`（O，16 B）：by_name 表的值。源码断言文本里的字段名 `handle` / `is_virtual`
/// 直接来自 panic 字符串。`is_virtual` = 这是 `create_name_mapping` 建的别名，不是真纹理。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NamedTexture {
    pub handle: AssetIndex,
    pub is_virtual: bool,
}

/// `texture_library::DescriptorIndex`（O：u32，`Option<DescriptorIndex>` 以 u32 标签 + u32 值存放，8 B）。
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct DescriptorIndex(pub u32);

/// `rhapsody::rg::import_export` 导入纹理后的句柄（O：0x24 B，`PresetTextureState.rg_handle` 的 `Option` 以
/// 第 0x22 字节 == 7 表示 None）。字段含义 U，仅保留可比较标识。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ImportedTexture {
    pub resource: u64,
    pub generation: u64,
    pub access: u8,
    /// B 阶段追加（i_api）：`get_rg_texture` 的返回值是 36 B `{id: u32, desc: TextureDesc(32 B)}`
    /// （O 0x140a59800：`impl$0::as_texture` 把 `[rsp+0x2c..0x50]` 整块搬进结果，并对 `+4` 处的 desc 调 `extent_2d`）。
    pub desc: TextureDesc,
}

/// `texture_library::PresetTextureState`（O，0x60 B）：`texture` @+0（0x38 B）+ `rg_handle: Option<ImportedTexture>` @+0x38。
/// 桶内偏移（桶 0x68 B，键 `TexturePreset` 1 B @0，状态 @8）：Texture @8..0x40，`rg_handle` @0x40..0x64，标签字节 @0x62。
#[derive(Clone, Debug)]
pub struct PresetTextureState {
    pub texture: Texture,
    pub rg_handle: Option<ImportedTexture>,
}

/// `DescriptorMap`（RwLock 内的数据，O）。
#[derive(Default, Debug)]
pub struct DescriptorMap {
    /// 已分配、尚未写进描述符集的资产（O：`handle_to_bindless_index` 里 push；`bindless_textures_dset` 里 drain）。
    pub pending: Vec<AssetIndex>,
    /// `AssetIndex -> Option<DescriptorIndex>`，`assign_auto_resize` 维护（缺口填 None）。
    pub indices: Vec<Option<DescriptorIndex>>,
    /// 下一个要发的描述符索引（O：@0xc0，先自增再断言）。
    pub next: u32,
}

/// `update_persistent_descriptor_set` 迭代器的元素（O：closure 0x140a96740 在 bump 里分配 0x20 B：
/// `{0u32, 0u64, image_view, 5u64}`，返回 `{0u32, DescriptorIndex, &entry}`）。含义 U：binding 0、布局值 5。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DescriptorImageUpdate {
    pub binding: u32,
    pub index: DescriptorIndex,
    pub image_view: ImageView,
    pub layout: u64,
}

/// `vk::ImageView`（`get_vk_image_view` 的返回）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ImageView(pub u64);

/// `GraphicsDevice::update_texture` 的更新描述（O：clear_preset_texture 里 0x58..0x90 的结构）。
/// 数据是全 0 的 `w*h*d*bpp` 字节；mip 0；extent = 纹理自身 extent；另有两个标量实参 1 与 0x22（含义 U）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureUpdate {
    pub data: Vec<u8>,
    pub buffer_offset: u64,
    pub mip_level: u32,
    pub extent: [u32; 3],
    pub array_layers: u64,
    pub access: u8,
}

/// 库用到的 rhapsody 设备调用面（外部库占位；调用形态来自机器码）。
pub trait TextureLibraryDevice: TextureDevice {
    /// `create_persistent_descriptor_set(name = "bindless_textures", 1, 4)`（O：new 里传 `{String, u32 1, u8 4}` 与 r9d=1，含义 U）。
    fn create_persistent_descriptor_set(&self, name: &str) -> DescriptorSet;
    /// `update_persistent_descriptor_set(dset, iterator)`。
    fn update_persistent_descriptor_set(
        &self,
        dset: &DescriptorSet,
        updates: &[DescriptorImageUpdate],
    );
    /// `get_vk_image_view(texture, 0)`（O：第 4 参为 0）。
    fn get_vk_image_view(&self, texture: &Texture, mip: u32) -> Result<ImageView, BackendError>;
    /// `update_texture(texture, &update, 1, 0x22)`。
    fn update_texture(&self, texture: &Texture, update: &TextureUpdate)
    -> Result<(), BackendError>;
}

impl TextureLibraryDevice for GraphicsDevice {
    fn create_persistent_descriptor_set(&self, name: &str) -> DescriptorSet {
        GraphicsDevice::create_persistent_descriptor_set(self, name)
    }
    fn update_persistent_descriptor_set(
        &self,
        _dset: &DescriptorSet,
        _updates: &[DescriptorImageUpdate],
    ) {
    }
    fn get_vk_image_view(&self, texture: &Texture, _mip: u32) -> Result<ImageView, BackendError> {
        Ok(ImageView(texture.id))
    }
    fn update_texture(
        &self,
        _texture: &Texture,
        _update: &TextureUpdate,
    ) -> Result<(), BackendError> {
        Ok(())
    }
}

/// 渲染图的纹理导入/导出（`rhapsody::rg::import_export::impl$0::{import_unowned, export}`，O 0x140bd60a0 / 0x140bd6380）。
pub trait TextureRenderGraph {
    fn import_unowned_texture(&mut self, texture: &Texture, access: u8) -> ImportedTexture;
    fn export_texture(&mut self, handle: ImportedTexture, access: u8);
}

/// 占位 `RenderGraph`：复用缓冲导入/导出的记录向量（资源号取纹理 id），便于测试观察调用。
impl TextureRenderGraph for RenderGraph {
    fn import_unowned_texture(&mut self, texture: &Texture, access: u8) -> ImportedTexture {
        let b = self.import_unowned(texture.id, access);
        // desc：占位 `Texture` 只有 width/height/format（真实 `Texture+0x18` 是整块 `TextureDesc`，U）。
        let desc = TextureDesc::new_2d([texture.width, texture.height], texture.format);
        ImportedTexture {
            resource: b.resource,
            generation: b.generation,
            access,
            desc,
        }
    }
    fn export_texture(&mut self, handle: ImportedTexture, access: u8) {
        self.export(
            ImportedBuffer {
                resource: handle.resource,
                generation: handle.generation,
                access: handle.access as u32,
            },
            access,
        );
    }
}

/// `rhapsody::raw::format::vk_format_info(format).bytes_per_texel`（外部库；O：取返回结构 +4 的 u32）。
/// 只覆盖 `TexturePreset` 用到的格式（0x09/0x25/0x46/0x4c/0x4d/0x53/0x61/0x63）；其余按 4 字节（U）。
pub fn format_bytes_per_texel(format: u32) -> u32 {
    match format {
        9 => 1,
        0x46 | 0x4c => 2,
        0x25 | 0x53 | 0x63 => 4,
        0x61 => 8,
        _ => 4,
    }
}

// ------------------------------------------------------------------ 库本体

/// `AssetTextureLibrary`（O：`Res<AssetTextureLibrary>` 被 nani_update_* 系统使用）。
pub struct AssetTextureLibrary {
    textures: Vec<Texture>,
    dset: DescriptorSet,
    by_name: HashMap<NameHash, NamedTexture, NoHashBuildHasher>,
    presets: HashMap<TexturePreset, PresetTextureState>,
    bindless: RwLock<DescriptorMap>,
}

impl bevy_ecs::resource::Resource for AssetTextureLibrary {}

/// `utils::impl$4::assign_auto_resize`（O 0x1412f8f60，泛型实例 `Option<DescriptorIndex>`）：
/// `idx == len` → push；`idx < len` → 覆盖；`idx > len` → 先用 `Default`（None，全 0）补到 `idx` 再 push。
pub fn assign_auto_resize<T: Default + Clone>(v: &mut Vec<T>, idx: usize, value: T) {
    if idx >= v.len() {
        v.resize(idx, T::default());
        v.push(value);
    } else {
        v[idx] = value;
    }
}

impl AssetTextureLibrary {
    /// `new`（O 0x14093bb10）：只做一件事——创建名为 `"bindless_textures"` 的持久描述符集；其余字段全部空。
    /// `presets` 的 `RandomState` 取线程局部 KEYS（k0 自增，hashmap_random_keys）。
    pub fn new(device: &impl TextureLibraryDevice) -> Self {
        let dset = device.create_persistent_descriptor_set("bindless_textures");
        AssetTextureLibrary {
            textures: Vec::new(),
            dset,
            by_name: HashMap::default(),
            presets: HashMap::new(),
            bindless: RwLock::new(DescriptorMap::default()),
        }
    }

    /// `add(name, texture) -> AssetIndex`（O 0x14093c200）。
    ///  1. `name` 为 Some：先查 by_name，已有 → `panic!("AssetTextureLibrary: the name {name} is already taken")`
    ///     （此时什么都没改）；
    ///  2. `textures.push(texture)`，`handle = 旧 len`；
    ///  3. `trace!("Loaded texture {:?} as {:?}", name, handle)`（对 None 名字同样打印，且在 by_name 之前）；
    ///  4. `name` 为 Some：`assert!(by_name.insert(NameHash::compute(&n), NamedTexture{handle, is_virtual:false}).is_none())`。
    ///  None 名字只入向量不入表。
    pub fn add(&mut self, name: Option<String>, texture: Texture) -> AssetIndex {
        if let Some(n) = &name {
            if self.by_name.contains_key(&NameHash::compute(n)) {
                panic!("AssetTextureLibrary: the name {} is already taken", n);
            }
        }
        let handle = AssetIndex(self.textures.len());
        self.textures.push(texture);
        log::trace!(
            target: "country_core::asset_libraries::texture_library",
            "Loaded texture {:?} as {:?}",
            name,
            handle
        );
        if let Some(n) = name {
            let old = self.by_name.insert(
                NameHash::compute(&n),
                NamedTexture {
                    handle,
                    is_virtual: false,
                },
            );
            assert!(
                old.is_none(),
                "assertion failed: self.by_name.insert(NameHash::compute(&n),\n        NamedTexture {{ handle, is_virtual: false }}).is_none()"
            );
        }
        handle
    }

    /// `create_name_mapping(name, target)`（O 0x14093c720）：给已存在的纹理 `target` 起别名 `name`。
    ///  1. 查 `target`，缺失 → `panic!("Was asked to map {name} to {target}, but the latter does not exist as a named texture")`；
    ///  2. `by_name.insert(NameHash(name), NamedTexture{ handle: target.handle, is_virtual: true })`；
    ///  3. 若旧值存在且 `!is_virtual`（即 name 本来是真纹理）→ panic `"Onoz, {name} already exists, and is a real
    ///     texture. Create a new (arbitrary) name, and map it to a real texture instead, like 'imaginary/pony' ->
    ///     'the_real_texture_asset'."`。**注意：insert 已经发生**，panic 前真纹理的表项已被别名覆盖（O：panic 在 insert 之后）。
    ///     旧值为别名则静默覆盖。
    pub fn create_name_mapping(&mut self, name: &str, target: &str) {
        let target_entry = match self.by_name.get(&NameHash::compute(target)) {
            Some(e) => *e,
            None => panic!(
                "Was asked to map {} to {}, but the latter does not exist as a named texture",
                name, target
            ),
        };
        let old = self.by_name.insert(
            NameHash::compute(name),
            NamedTexture {
                handle: target_entry.handle,
                is_virtual: true,
            },
        );
        if let Some(old) = old {
            if !old.is_virtual {
                panic!(
                    "Onoz, {} already exists, and is a real texture. Create a new (arbitrary) name, and map it to a real texture instead, like 'imaginary/pony' -> 'the_real_texture_asset'.",
                    name
                );
            }
        }
    }

    /// `get_handle_and_texture_by_name(hash)`（O 0x14093c970）：未命中 → None；命中但下标越界（`cmp rax,[len]; cmovb`）→ None。
    pub fn get_handle_and_texture_by_name(&self, hash: NameHash) -> Option<(AssetIndex, &Texture)> {
        let entry = self.by_name.get(&hash)?;
        self.textures.get(entry.handle.0).map(|t| (entry.handle, t))
    }

    /// `get_by_name(hash) -> &Texture`（O 0x14093ca90）：
    ///  * 未命中 → `panic!("No such texture: {hash:?}. Available: {all_keys:?}")`（`all_keys` 是 `Vec<&NameHash>`）；
    ///  * 命中但下标 >= len → `panic!("Texture not loaded: {hash:?}")`。
    ///  NameHash 的格式化特征（`utils::name_hash::impl$7::fmt`）是 Debug 还是 Display：U（这里用 Debug）。
    pub fn get_by_name(&self, hash: NameHash) -> &Texture {
        match self.by_name.get(&hash) {
            None => {
                let available: Vec<&NameHash> = self.by_name.keys().collect();
                panic!("No such texture: {:?}. Available: {:?}", hash, available)
            }
            Some(entry) => match self.textures.get(entry.handle.0) {
                Some(t) => t,
                None => panic!("Texture not loaded: {:?}", hash),
            },
        }
    }

    /// 只读访问：按 `AssetIndex` 取纹理（O：closure 0x140a96740 里的 `textures[idx]`，越界即 `panic_bounds_check`）。
    pub fn texture(&self, handle: AssetIndex) -> &Texture {
        &self.textures[handle.0]
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    // ---------------------------------------------------------------- 预设纹理

    /// `create_preset_textures(device)`（O 0x14093bd80）：对 `enum_iterator::All<TexturePreset>`（0..=0x19）逐个：
    /// `TextureDesc::from(preset)` → `device.create_texture(name = "(NameHash)", desc, TextureInitialData::None)`
    /// （Err → `Result::unwrap` panic）→ `PresetTextureState { texture, rg_handle: None }`，收集进一张**新** HashMap
    /// （新的 RandomState）后整体替换 `self.presets`（旧表连同其纹理句柄被 drop）。
    /// 原版经 `ResMut<AssetTextureLibrary>` 的 DerefMut 调用，会把 changed tick 置为 this_run（O：末尾 `[rcx]=[rsi+0x1c]`），
    /// 在 ECS 之外无对应物。`desc_of` 代替尚未恢复的 `impl From<TexturePreset> for TextureDesc`（0xa59890）。
    pub fn create_preset_textures<D: TextureDevice>(
        &mut self,
        device: &D,
        desc_of: impl Fn(TexturePreset) -> TextureDesc,
    ) {
        let mut presets = HashMap::new();
        for p in 0..PRESET_COUNT {
            let preset = TexturePreset(p);
            let desc = desc_of(preset);
            let texture = device
                .create_texture(PRESET_TEXTURE_DEBUG_NAME, &desc, &[])
                .expect("called `Result::unwrap()` on an `Err` value");
            presets.insert(
                preset,
                PresetTextureState {
                    texture,
                    rg_handle: None,
                },
            );
        }
        self.presets = presets;
    }

    /// 按预设取状态；缺失 → `expect("no entry found for key")`（O：`HashMap` 的 Index 语义，三个函数共用此文案）。
    fn preset_state(&self, preset: TexturePreset) -> &PresetTextureState {
        self.presets.get(&preset).expect("no entry found for key")
    }

    /// `clear_preset_texture(device, preset)`（O 0x14093bee0）：把预设纹理整张写成 0。
    /// `size = bytes_per_texel(format) * (w*h*d as u32)`（宽高深的乘积按 **u32 回绕**，再扩到 u64；O：`imul edi,ebx; imul edi,r13d`），
    /// 以全 0 的 `size` 字节、mip 0、extent = 纹理 extent 调 `device.update_texture(texture, update, 1, 0x22)`；Err → `Result::unwrap` panic。
    pub fn clear_preset_texture(&self, device: &impl TextureLibraryDevice, preset: TexturePreset) {
        let state = self.preset_state(preset);
        let tex = &state.texture;
        let extent = [tex.width, tex.height, texture_depth(tex)];
        let texels = extent[0].wrapping_mul(extent[1]).wrapping_mul(extent[2]);
        let size = u64::from(format_bytes_per_texel(tex.format)) * u64::from(texels);
        let update = TextureUpdate {
            data: vec![0u8; size as usize],
            buffer_offset: 0,
            mip_level: 0,
            extent,
            array_layers: 1,
            access: RG_TEXTURE_ACCESS,
        };
        device
            .update_texture(tex, &update)
            .expect("called `Result::unwrap()` on an `Err` value");
    }

    /// `import_texture_presets_to_rg(rg)`（O 0x14093cd30）：遍历全部预设；每个先 `assert!(preset.rg_handle.is_none())`
    /// （否则 panic `"assertion failed: preset.rg_handle.is_none()"`），再 `rg_handle = Some(rg.import_unowned(&texture, 0x22))`。
    /// 空表直接返回。遍历顺序为哈希表顺序（原版随机）；断言失败发生在已导入若干预设之后（无回滚）。
    pub fn import_texture_presets_to_rg(&mut self, rg: &mut impl TextureRenderGraph) {
        for preset in self.presets.values_mut() {
            assert!(
                preset.rg_handle.is_none(),
                "assertion failed: preset.rg_handle.is_none()"
            );
            preset.rg_handle = Some(rg.import_unowned_texture(&preset.texture, RG_TEXTURE_ACCESS));
        }
    }

    /// `export_preset_textures_from_rg(rg)`（O 0x14093ce30）：遍历全部预设；每个 `rg_handle.take().unwrap()`
    /// （None → `Option::unwrap` panic，**句柄已先被置 None**）再 `rg.export(handle, 0x22)`。
    pub fn export_preset_textures_from_rg(&mut self, rg: &mut impl TextureRenderGraph) {
        for preset in self.presets.values_mut() {
            let handle = preset.rg_handle.take().unwrap();
            rg.export_texture(handle, RG_TEXTURE_ACCESS);
        }
    }

    /// `get_rg_texture(preset) -> ImportedTexture`（O 0x14093cf30）：缺失 → `expect("no entry found for key")`；
    /// 句柄为 None → `Option::unwrap` panic。
    pub fn get_rg_texture(&self, preset: TexturePreset) -> ImportedTexture {
        self.preset_state(preset).rg_handle.unwrap()
    }

    /// 只读：预设状态（测试/调用方观察用；原版无此函数）。
    pub fn preset(&self, preset: TexturePreset) -> Option<&PresetTextureState> {
        self.presets.get(&preset)
    }

    // ---------------------------------------------------------------- bindless

    /// `handle_to_bindless_index(asset) -> DescriptorIndex`（O 0x14093d310；`&self`，经 RwLock）：
    ///  1. 读锁（poison → `Result::unwrap` panic）；`indices[asset]` 存在且为 Some → 返回它；
    ///  2. 否则释放读锁，取写锁（poison 同上）。**不重新检查**（O：写锁后直接分配，并发时可能重复分配）；
    ///  3. `idx = next; next += 1`（先自增），然后 `assert!(idx < REQUIRED - RESERVED)`（0x1fe0；失败时写锁持有中，
    ///     锁因 panic 被毒化），`pending.push(asset)`，`assign_auto_resize(indices, asset, Some(idx))`。
    pub fn handle_to_bindless_index(&self, asset: AssetIndex) -> DescriptorIndex {
        {
            let map = self.bindless.read().unwrap();
            if let Some(Some(idx)) = map.indices.get(asset.0) {
                return *idx;
            }
        }
        let mut map = self.bindless.write().unwrap();
        let idx = map.next;
        map.next = idx.wrapping_add(1);
        assert!(
            idx < REQUIRED_BINDLESS_DESCRIPTOR_COUNT - RESERVED_DESCRIPTOR_COUNT,
            "assertion failed: descriptor_index.0 <\n    GraphicsDevice::REQUIRED_BINDLESS_DESCRIPTOR_COUNT as u32 -\n        RESERVED_DESCRIPTOR_COUNT as u32"
        );
        let descriptor = DescriptorIndex(idx);
        map.pending.push(asset);
        assign_auto_resize(&mut map.indices, asset.0, Some(descriptor));
        descriptor
    }

    /// `bindless_textures_dset(device) -> &DescriptorSet`（O 0x14093d040；内联了 `update_bindless_texture_table`）：
    /// 取写锁（poison → unwrap panic）；`pending` 非空才更新：把 `pending` 整体 drain（长度置 0），逐项映射成
    /// `{ binding 0, descriptor = indices[asset].expect("asset descriptor index"), view = device.get_vk_image_view(&textures[asset], 0).unwrap(), layout 5 }`
    /// 后调 `device.update_persistent_descriptor_set(&dset, ..)`。`pending` 为空则什么都不调。最后返回持久描述符集 `&self.dset`。
    /// （原版用 bumpalo 分配每个 0x20 B 的更新项，这里用 Vec。）
    pub fn bindless_textures_dset(&self, device: &impl TextureLibraryDevice) -> &DescriptorSet {
        let mut map = self.bindless.write().unwrap();
        if !map.pending.is_empty() {
            let pending = std::mem::take(&mut map.pending);
            let updates: Vec<DescriptorImageUpdate> = pending
                .into_iter()
                .map(|asset| {
                    let index = map.indices[asset.0].expect("asset descriptor index");
                    let texture = &self.textures[asset.0];
                    let view = device.get_vk_image_view(texture, 0).unwrap();
                    DescriptorImageUpdate {
                        binding: 0,
                        index,
                        image_view: view,
                        layout: 5,
                    }
                })
                .collect();
            device.update_persistent_descriptor_set(&self.dset, &updates);
        }
        &self.dset
    }

    /// `impl$2::image_name_to_index(name)`（O 0x14093d690，某 trait 的实现）：`by_name` 里按 `NameHash::compute(name)` 查，
    /// 未命中 → `panic!("No such texture: {name}")`；命中则 `handle_to_bindless_index(entry.handle)`。
    /// （注意：不检查 `handle` 是否越界，也不看 `is_virtual`。）
    pub fn image_name_to_index(&self, name: &str) -> DescriptorIndex {
        match self.by_name.get(&NameHash::compute(name)) {
            Some(entry) => self.handle_to_bindless_index(entry.handle),
            None => panic!("No such texture: {}", name),
        }
    }

    /// 只读观察：当前待写入的 bindless 资产（测试用）。
    pub fn pending_bindless(&self) -> Vec<AssetIndex> {
        self.bindless.read().unwrap().pending.clone()
    }

    /// 只读观察：持久描述符集（O：`bindless_textures_dset` 的返回值就是它）。
    pub fn dset(&self) -> &DescriptorSet {
        &self.dset
    }
}

/// 纹理深度（O：`Texture + 0x2c`，占位 `Texture` 无此字段 → 2D 取 1，U）。
fn texture_depth(_t: &Texture) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct MockDevice {
        created: RefCell<Vec<(String, TextureDesc)>>,
        updates: RefCell<Vec<(Texture, TextureUpdate)>>,
        dset_updates: RefCell<Vec<Vec<DescriptorImageUpdate>>>,
        fail_create: bool,
    }
    impl TextureDevice for MockDevice {
        fn create_texture(
            &self,
            name: &str,
            desc: &TextureDesc,
            _data: &[u8],
        ) -> Result<Texture, BackendError> {
            if self.fail_create {
                return Err(BackendError("boom".into()));
            }
            let id = self.created.borrow().len() as u64 + 100;
            self.created.borrow_mut().push((name.to_owned(), *desc));
            Ok(Texture {
                id,
                width: desc.extent[0],
                height: desc.extent[1],
                format: u32::from(desc.arg3),
            })
        }
    }
    impl TextureLibraryDevice for MockDevice {
        fn create_persistent_descriptor_set(&self, _name: &str) -> DescriptorSet {
            DescriptorSet
        }
        fn update_persistent_descriptor_set(
            &self,
            _d: &DescriptorSet,
            u: &[DescriptorImageUpdate],
        ) {
            self.dset_updates.borrow_mut().push(u.to_vec());
        }
        fn get_vk_image_view(&self, t: &Texture, mip: u32) -> Result<ImageView, BackendError> {
            assert_eq!(mip, 0);
            Ok(ImageView(t.id + 1000))
        }
        fn update_texture(&self, t: &Texture, u: &TextureUpdate) -> Result<(), BackendError> {
            self.updates.borrow_mut().push((t.clone(), u.clone()));
            Ok(())
        }
    }

    fn tex(id: u64) -> Texture {
        Texture {
            id,
            width: 4,
            height: 2,
            format: 0x25,
        }
    }
    fn lib() -> AssetTextureLibrary {
        AssetTextureLibrary::new(&MockDevice::default())
    }
    fn desc_of(_p: TexturePreset) -> TextureDesc {
        TextureDesc::new([8, 4, 1], 0x25, 0)
    }

    #[test]
    fn add_pushes_textures_and_registers_names() {
        let mut l = lib();
        let a = l.add(Some("a".into()), tex(1));
        let b = l.add(None, tex(2));
        let c = l.add(Some("c".into()), tex(3));
        assert_eq!((a, b, c), (AssetIndex(0), AssetIndex(1), AssetIndex(2)));
        assert_eq!(l.len(), 3);
        assert_eq!(l.get_by_name(NameHash::compute("c")).id, 3);
        // None 名字只入向量不入表
        assert!(
            l.get_handle_and_texture_by_name(NameHash::compute("b"))
                .is_none()
        );
    }

    #[test]
    #[should_panic(expected = "AssetTextureLibrary: the name a is already taken")]
    fn add_duplicate_name_panics() {
        let mut l = lib();
        l.add(Some("a".into()), tex(1));
        l.add(Some("a".into()), tex(2));
    }

    #[test]
    fn add_duplicate_leaves_library_untouched() {
        let mut l = lib();
        l.add(Some("a".into()), tex(1));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            l.add(Some("a".into()), tex(2))
        }));
        assert!(r.is_err());
        assert_eq!(l.len(), 1, "重名检查发生在 push 之前");
    }

    #[test]
    fn create_name_mapping_creates_virtual_alias() {
        let mut l = lib();
        l.add(Some("real".into()), tex(7));
        l.create_name_mapping("alias", "real");
        let (h, t) = l
            .get_handle_and_texture_by_name(NameHash::compute("alias"))
            .unwrap();
        assert_eq!((h, t.id), (AssetIndex(0), 7));
        assert!(l.by_name[&NameHash::compute("alias")].is_virtual);
        // 别名可以被再次改指（旧值是 virtual）
        l.add(Some("real2".into()), tex(8));
        l.create_name_mapping("alias", "real2");
        assert_eq!(l.get_by_name(NameHash::compute("alias")).id, 8);
    }

    #[test]
    #[should_panic(
        expected = "Was asked to map x to nope, but the latter does not exist as a named texture"
    )]
    fn create_name_mapping_missing_target_panics() {
        lib().create_name_mapping("x", "nope");
    }

    #[test]
    #[should_panic(
        expected = "Onoz, real already exists, and is a real texture. Create a new (arbitrary) name"
    )]
    fn create_name_mapping_over_real_texture_panics() {
        let mut l = lib();
        l.add(Some("real".into()), tex(1));
        l.add(Some("other".into()), tex(2));
        l.create_name_mapping("real", "other");
    }

    #[test]
    fn create_name_mapping_over_real_overwrites_before_panicking() {
        let mut l = lib();
        l.add(Some("real".into()), tex(1));
        l.add(Some("other".into()), tex(2));
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            l.create_name_mapping("real", "other")
        }));
        // O：insert 在 panic 之前已发生
        assert!(l.by_name[&NameHash::compute("real")].is_virtual);
        assert_eq!(l.get_by_name(NameHash::compute("real")).id, 2);
    }

    #[test]
    #[should_panic(expected = "No such texture: NameHash { hash: ")]
    fn get_by_name_missing_lists_available() {
        lib().get_by_name(NameHash::compute("zzz"));
    }

    #[test]
    fn get_by_name_missing_message_has_available_keys() {
        let mut l = lib();
        l.add(Some("k".into()), tex(1));
        let k = NameHash::compute("k");
        let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            l.get_by_name(NameHash::compute("zzz"))
        }))
        .unwrap_err();
        let msg = err.downcast_ref::<String>().unwrap();
        assert!(msg.contains(". Available: ["), "{msg}");
        assert!(msg.contains(&format!("{:?}", k)), "{msg}");
    }

    #[test]
    #[should_panic(expected = "Texture not loaded: ")]
    fn get_by_name_index_out_of_range_panics() {
        let mut l = lib();
        l.add(Some("k".into()), tex(1));
        l.textures.clear(); // 表项在、向量空
        l.get_by_name(NameHash::compute("k"));
    }

    #[test]
    fn get_handle_and_texture_out_of_range_is_none() {
        let mut l = lib();
        l.add(Some("k".into()), tex(1));
        l.textures.clear();
        assert!(
            l.get_handle_and_texture_by_name(NameHash::compute("k"))
                .is_none()
        );
    }

    #[test]
    fn bindless_index_allocation_reuse_and_gap_fill() {
        let mut l = lib();
        for i in 0..6 {
            l.add(None, tex(i));
        }
        let d5 = l.handle_to_bindless_index(AssetIndex(5));
        assert_eq!(d5, DescriptorIndex(0));
        // 复用
        assert_eq!(
            l.handle_to_bindless_index(AssetIndex(5)),
            DescriptorIndex(0)
        );
        assert_eq!(
            l.handle_to_bindless_index(AssetIndex(2)),
            DescriptorIndex(1)
        );
        let map = l.bindless.read().unwrap();
        assert_eq!(map.pending, vec![AssetIndex(5), AssetIndex(2)]);
        assert_eq!(map.next, 2);
        assert_eq!(map.indices.len(), 6);
        assert_eq!(map.indices[0], None, "缺口填 None");
        assert_eq!(map.indices[2], Some(DescriptorIndex(1)));
    }

    #[test]
    fn bindless_limit_is_0x1fe0_and_next_already_incremented() {
        let l = lib();
        for i in 0..0x1fe0usize {
            assert_eq!(
                l.handle_to_bindless_index(AssetIndex(i)),
                DescriptorIndex(i as u32)
            );
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            l.handle_to_bindless_index(AssetIndex(0x1fe0))
        }));
        let err = r.unwrap_err();
        let msg = err
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap();
        assert!(
            msg.starts_with("assertion failed: descriptor_index.0 <"),
            "{msg}"
        );
        // 写锁持有时 panic -> 锁被毒化，next 已自增
        assert!(l.bindless.is_poisoned());
    }

    #[test]
    #[should_panic(expected = "PoisonError")]
    fn bindless_poisoned_lock_unwraps() {
        let l = lib();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = l.bindless.write().unwrap();
            panic!("poison it");
        }));
        l.handle_to_bindless_index(AssetIndex(0));
    }

    #[test]
    fn bindless_dset_flushes_pending_once() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.add(Some("a".into()), tex(1));
        l.add(Some("b".into()), tex(2));
        let ib = l.image_name_to_index("b");
        let ia = l.image_name_to_index("a");
        assert_eq!((ib, ia), (DescriptorIndex(0), DescriptorIndex(1)));
        let _ = l.bindless_textures_dset(&dev);
        {
            let ups = dev.dset_updates.borrow();
            assert_eq!(ups.len(), 1);
            assert_eq!(
                ups[0],
                vec![
                    DescriptorImageUpdate {
                        binding: 0,
                        index: DescriptorIndex(0),
                        image_view: ImageView(1002),
                        layout: 5
                    },
                    DescriptorImageUpdate {
                        binding: 0,
                        index: DescriptorIndex(1),
                        image_view: ImageView(1001),
                        layout: 5
                    },
                ]
            );
        }
        assert!(l.pending_bindless().is_empty());
        // pending 为空 -> 不再调设备
        let _ = l.bindless_textures_dset(&dev);
        assert_eq!(dev.dset_updates.borrow().len(), 1);
    }

    #[test]
    #[should_panic(expected = "No such texture: missing")]
    fn image_name_to_index_missing_panics() {
        lib().image_name_to_index("missing");
    }

    #[test]
    fn image_name_to_index_goes_through_alias() {
        let mut l = lib();
        l.add(Some("real".into()), tex(1));
        l.create_name_mapping("alias", "real");
        assert_eq!(
            l.image_name_to_index("alias"),
            l.image_name_to_index("real")
        );
    }

    #[test]
    fn assign_auto_resize_semantics() {
        let mut v: Vec<Option<u32>> = vec![];
        assign_auto_resize(&mut v, 0, Some(1));
        assign_auto_resize(&mut v, 3, Some(4));
        assert_eq!(v, vec![Some(1), None, None, Some(4)]);
        assign_auto_resize(&mut v, 1, Some(9));
        assert_eq!(v, vec![Some(1), Some(9), None, Some(4)]);
    }

    #[test]
    fn create_preset_textures_makes_all_26_and_replaces() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, desc_of);
        assert_eq!(dev.created.borrow().len(), 26);
        assert!(dev.created.borrow().iter().all(|(n, _)| n == "(NameHash)"));
        assert!((0..26u8).all(|p| l.preset(TexturePreset(p)).is_some()));
        assert!(l.preset(TexturePreset(26)).is_none());
        assert!(l.preset(TexturePreset(3)).unwrap().rg_handle.is_none());
        // 再次创建：整体替换（新纹理 id）
        let before = l.preset(TexturePreset(0)).unwrap().texture.id;
        l.create_preset_textures(&dev, desc_of);
        assert_ne!(l.preset(TexturePreset(0)).unwrap().texture.id, before);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn create_preset_textures_unwraps_error() {
        let dev = MockDevice {
            fail_create: true,
            ..Default::default()
        };
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, desc_of);
    }

    #[test]
    fn clear_preset_texture_writes_zeros_sized_by_format() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, |_| TextureDesc::new([8, 4, 1], 0x61, 0)); // 8 B/texel
        l.clear_preset_texture(&dev, TexturePreset(5));
        let ups = dev.updates.borrow();
        assert_eq!(ups.len(), 1);
        let (_, u) = &ups[0];
        assert_eq!(u.data.len(), 8 * 4 * 8);
        assert!(u.data.iter().all(|&b| b == 0));
        assert_eq!(
            (u.mip_level, u.extent, u.array_layers, u.access),
            (0, [8, 4, 1], 1, 0x22)
        );
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn clear_preset_texture_missing_panics() {
        let dev = MockDevice::default();
        lib().clear_preset_texture(&dev, TexturePreset(1));
    }

    #[test]
    fn rg_import_export_roundtrip() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, desc_of);
        let mut rg = RenderGraph::default();
        l.import_texture_presets_to_rg(&mut rg);
        assert_eq!(rg.imports.len(), 26);
        assert!(rg.imports.iter().all(|&(_, a)| a == 0x22));
        let h = l.get_rg_texture(TexturePreset(7));
        assert_eq!(h.access, 0x22);
        assert_eq!(h.resource, l.preset(TexturePreset(7)).unwrap().texture.id);
        l.export_preset_textures_from_rg(&mut rg);
        assert_eq!(rg.exports.len(), 26);
        assert!(l.preset(TexturePreset(7)).unwrap().rg_handle.is_none());
    }

    #[test]
    #[should_panic(expected = "assertion failed: preset.rg_handle.is_none()")]
    fn import_twice_asserts() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, desc_of);
        let mut rg = RenderGraph::default();
        l.import_texture_presets_to_rg(&mut rg);
        l.import_texture_presets_to_rg(&mut rg);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn export_without_import_panics() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, desc_of);
        l.export_preset_textures_from_rg(&mut RenderGraph::default());
    }

    #[test]
    fn empty_presets_are_noops() {
        let mut l = lib();
        let mut rg = RenderGraph::default();
        l.import_texture_presets_to_rg(&mut rg);
        l.export_preset_textures_from_rg(&mut rg);
        assert!(rg.imports.is_empty() && rg.exports.is_empty());
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn get_rg_texture_missing_preset_panics() {
        lib().get_rg_texture(TexturePreset(1));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn get_rg_texture_not_imported_panics() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures(&dev, desc_of);
        l.get_rg_texture(TexturePreset(1));
    }
}
