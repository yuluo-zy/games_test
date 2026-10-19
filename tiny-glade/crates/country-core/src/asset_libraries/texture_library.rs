//! 纹理资产、预设与描述符发布。设备失败保留待更新队列，完整描述随资产传递。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::RwLock;

use crate::name_hash::NameHash;
use crate::render::load_texture::boundary::TextureDevice;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_contract::{DescriptorSet, GraphicsDevice};
use crate::rhapsody_types::{BackendError, ImportedBuffer, RenderGraph, Texture, TextureDesc};

pub const REQUIRED_BINDLESS_DESCRIPTOR_COUNT: u32 = 0x2000;
pub const RESERVED_DESCRIPTOR_COUNT: u32 = 0x20;

pub const PRESET_COUNT: u8 = TexturePreset::COUNT;

pub const RG_TEXTURE_ACCESS: u8 = 0x22;

pub const PRESET_TEXTURE_DEBUG_NAME: &str = "(NameHash)";

pub type NoHashHasher = nohash_hasher::NoHashHasher<u64>;
pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

/// 纹理向量下标，不是全局 HandleId。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct AssetIndex(pub usize);

/// 名字指向的纹理身份；虚拟名字可以重新映射。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NamedTexture {
    pub handle: AssetIndex,
    pub is_virtual: bool,
}

/// 采样描述符槽位，不能与纹理向量下标混用。
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct DescriptorIndex(pub u32);

/// 图资源身份及其权威描述。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ImportedTexture {
    pub resource: u64,
    pub generation: u64,
    pub access: u8,
    pub desc: TextureDesc,
}

/// 预设的物理资产、完整描述和当前图句柄。
#[derive(Clone, Debug)]
pub struct PresetTextureState {
    pub texture: Texture,
    /// 创建资源时使用的完整描述，导入时不能从宽高重新猜测。
    pub desc: TextureDesc,
    pub rg_handle: Option<ImportedTexture>,
}

/// 并发采样请求的索引与待发布队列。
#[derive(Default, Debug)]
pub struct DescriptorMap {
    pub pending: Vec<AssetIndex>,
    /// `AssetIndex -> Option<DescriptorIndex>`，`assign_auto_resize` 维护（缺口填 None）。
    pub indices: Vec<Option<DescriptorIndex>>,
    pub next: u32,
}

/// 向宿主提交的只读采样描述符更新。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DescriptorImageUpdate {
    pub binding: u32,
    pub index: DescriptorIndex,
    pub image_view: ImageView,
    /// 宿主纹理表身份，用于验证视图所有权。
    pub texture_id: u64,
    pub layout: u64,
}

/// 经过宿主验证的原生视图身份。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ImageView(pub u64);

/// 一个 mip 的连续数组层像素更新。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureUpdate {
    pub data: Vec<u8>,
    pub buffer_offset: u64,
    pub mip_level: u32,
    pub extent: [u32; 3],
    pub array_layers: u64,
    pub access: u8,
}

/// 纹理宿主能力；创建、上传和描述符更新都必须报告失败。
pub trait TextureLibraryDevice: TextureDevice {
    fn create_persistent_descriptor_set(&self, name: &str) -> Result<DescriptorSet, BackendError>;
    fn update_persistent_descriptor_set(
        &self,
        dset: &DescriptorSet,
        updates: &[DescriptorImageUpdate],
    ) -> Result<(), BackendError>;
    fn get_vk_image_view(&self, texture: &Texture, mip: u32) -> Result<ImageView, BackendError>;
    /// 提交指定 mip 的像素更新。
    fn update_texture(&self, texture: &Texture, update: &TextureUpdate)
    -> Result<(), BackendError>;
}

impl TextureLibraryDevice for GraphicsDevice {
    fn create_persistent_descriptor_set(&self, _name: &str) -> Result<DescriptorSet, BackendError> {
        Err(BackendError::unsupported("纹理资产库需要连接 TextureHost"))
    }
    fn update_persistent_descriptor_set(
        &self,
        _descriptor_set: &DescriptorSet,
        _updates: &[DescriptorImageUpdate],
    ) -> Result<(), BackendError> {
        Err(BackendError::unsupported("描述符更新需要连接 TextureHost"))
    }
    fn get_vk_image_view(&self, _texture: &Texture, _mip: u32) -> Result<ImageView, BackendError> {
        Err(BackendError::unsupported(
            "资产 ID 不能作为原生 image view 使用",
        ))
    }
    fn update_texture(
        &self,
        _texture: &Texture,
        _update: &TextureUpdate,
    ) -> Result<(), BackendError> {
        Err(BackendError::unsupported("纹理上传需要连接 TextureHost"))
    }
}

/// 物理纹理与渲染图之间的导入、导出接口。
pub trait TextureRenderGraph {
    fn import_texture(
        &mut self,
        texture: &Texture,
        desc: TextureDesc,
        access: u8,
    ) -> ImportedTexture;
    fn export_texture(&mut self, handle: ImportedTexture, access: u8);
}

impl TextureRenderGraph for RenderGraph {
    fn import_texture(
        &mut self,
        texture: &Texture,
        desc: TextureDesc,
        access: u8,
    ) -> ImportedTexture {
        let imported = self.import_unowned(texture.id, access);
        ImportedTexture {
            resource: imported.resource,
            generation: imported.generation,
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

/// 复用已恢复的 rhapsody 格式表，未知格式显式失败。
pub fn format_bytes_per_texel(format: u32) -> u32 {
    rhapsody::resource::try_vk_format_info(ash::vk::Format::from_raw(format as i32))
        .expect("不支持的纹理格式")
        .word_at_04()
}

/// 拥有普通纹理、名字别名、预设状态和延迟描述符更新。
pub struct AssetTextureLibrary {
    textures: Vec<Texture>,
    descriptor_set: DescriptorSet,
    by_name: HashMap<NameHash, NamedTexture, NoHashBuildHasher>,
    presets: Vec<PresetTextureState>,
    bindless: RwLock<DescriptorMap>,
}

impl bevy_ecs::resource::Resource for AssetTextureLibrary {}

/// 写入向量指定位置，用默认值填充中间空位。
pub fn assign_auto_resize<T: Default + Clone>(values: &mut Vec<T>, index: usize, value: T) {
    if index >= values.len() {
        values.resize(index, T::default());
        values.push(value);
    } else {
        values[index] = value;
    }
}

impl AssetTextureLibrary {
    /// 创建初始状态。
    pub fn new(device: &(impl TextureLibraryDevice + ?Sized)) -> Self {
        Self::try_new(device).expect("创建纹理描述符表失败")
    }

    pub fn try_new(device: &(impl TextureLibraryDevice + ?Sized)) -> Result<Self, BackendError> {
        let descriptor_set = device.create_persistent_descriptor_set("bindless_textures")?;
        Ok(AssetTextureLibrary {
            textures: Vec::new(),
            descriptor_set,
            by_name: HashMap::default(),
            presets: Vec::new(),
            bindless: RwLock::new(DescriptorMap::default()),
        })
    }

    /// 注册资产并返回稳定身份。
    pub fn add(&mut self, name: Option<String>, texture: Texture) -> AssetIndex {
        if let Some(texture_name) = &name {
            if self.by_name.contains_key(&NameHash::compute(texture_name)) {
                panic!(
                    "AssetTextureLibrary: the name {} is already taken",
                    texture_name
                );
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
        if let Some(texture_name) = name {
            let old = self.by_name.insert(
                NameHash::compute(&texture_name),
                NamedTexture {
                    handle,
                    is_virtual: false,
                },
            );
            assert!(
                old.is_none(),
                "assertion failed: self.by_name.insert(NameHash::compute(&texture_name),\n        NamedTexture {{ handle, is_virtual: false }}).is_none()"
            );
        }
        handle
    }

    pub fn create_name_mapping(&mut self, name: &str, target: &str) {
        let target_entry = match self.by_name.get(&NameHash::compute(target)) {
            Some(entry) => *entry,
            None => panic!(
                "Was asked to map {} to {}, but the latter does not exist as a named texture",
                name, target
            ),
        };
        if self
            .by_name
            .get(&NameHash::compute(name))
            .is_some_and(|entry| !entry.is_virtual)
        {
            panic!(
                "Onoz, {} already exists, and is a real texture. Create a new (arbitrary) name, and map it to a real texture instead, like 'imaginary/pony' -> 'the_real_texture_asset'.",
                name
            );
        }
        self.by_name.insert(
            NameHash::compute(name),
            NamedTexture {
                handle: target_entry.handle,
                is_virtual: true,
            },
        );
    }

    pub fn get_handle_and_texture_by_name(&self, hash: NameHash) -> Option<(AssetIndex, &Texture)> {
        let entry = self.by_name.get(&hash)?;
        self.textures
            .get(entry.handle.0)
            .map(|texture| (entry.handle, texture))
    }

    /// 按业务名字读取已注册资产。
    pub fn get_by_name(&self, hash: NameHash) -> &Texture {
        match self.by_name.get(&hash) {
            None => {
                let available: Vec<&NameHash> = self.by_name.keys().collect();
                panic!("No such texture: {:?}. Available: {:?}", hash, available)
            }
            Some(entry) => match self.textures.get(entry.handle.0) {
                Some(texture) => texture,
                None => panic!("Texture not loaded: {:?}", hash),
            },
        }
    }

    pub fn texture(&self, handle: AssetIndex) -> &Texture {
        &self.textures[handle.0]
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// 按预设描述创建资源，全部成功后替换预设表。
    pub fn create_preset_textures<D: TextureDevice + ?Sized>(&mut self, device: &D) {
        self.try_create_preset_textures(device)
            .expect("创建预设纹理失败");
    }

    pub fn try_create_preset_textures<D: TextureDevice + ?Sized>(
        &mut self,
        device: &D,
    ) -> Result<(), BackendError> {
        self.try_create_preset_textures_with(device, |preset| preset.texture().desc)
    }

    pub fn create_preset_textures_with<D: TextureDevice + ?Sized>(
        &mut self,
        device: &D,
        describe: impl Fn(TexturePreset) -> TextureDesc,
    ) {
        self.try_create_preset_textures_with(device, describe)
            .expect("创建预设纹理失败");
    }

    fn try_create_preset_textures_with<D: TextureDevice + ?Sized>(
        &mut self,
        device: &D,
        describe: impl Fn(TexturePreset) -> TextureDesc,
    ) -> Result<(), BackendError> {
        if self.presets.iter().any(|preset| preset.rg_handle.is_some()) {
            return Err(BackendError::invalid_usage("预设仍在图中使用，不能重建"));
        }
        let mut replacements: Vec<PresetTextureState> = Vec::with_capacity(PRESET_COUNT as usize);
        for preset in 0..PRESET_COUNT {
            let desc = describe(TexturePreset(preset));
            match device.create_texture(PRESET_TEXTURE_DEBUG_NAME, &desc, &[]) {
                Ok(texture) => replacements.push(PresetTextureState {
                    texture,
                    desc,
                    rg_handle: None,
                }),
                Err(error) => {
                    for created in replacements {
                        device.release_texture(&created.texture);
                    }
                    return Err(error);
                }
            }
        }
        for previous in std::mem::replace(&mut self.presets, replacements) {
            device.release_texture(&previous.texture);
        }
        Ok(())
    }

    fn preset_state(&self, preset: TexturePreset) -> &PresetTextureState {
        self.presets
            .get(preset.0 as usize)
            .expect("no entry found for key")
    }

    pub fn clear_preset_texture(
        &self,
        device: &(impl TextureLibraryDevice + ?Sized),
        preset: TexturePreset,
    ) {
        self.try_clear_preset_texture(device, preset)
            .expect("预设纹理清零失败");
    }

    /// 清零 mip 0 的所有数组层；使用完整描述和受检查长度，禁止整数回绕。
    pub fn try_clear_preset_texture(
        &self,
        device: &(impl TextureLibraryDevice + ?Sized),
        preset: TexturePreset,
    ) -> Result<(), BackendError> {
        let state = self
            .presets
            .get(preset.0 as usize)
            .ok_or_else(|| BackendError::invalid_usage("预设纹理未创建"))?;
        let desc = state.desc;
        let format =
            rhapsody::resource::try_vk_format_info(ash::vk::Format::from_raw(desc.format as i32))
                .map_err(|_| BackendError::unsupported("预设纹理格式未知"))?;
        if format.byte_at_08() != 1 {
            return Err(BackendError::unsupported("压缩纹理不支持按像素清零"));
        }
        let layers = u64::from(desc.array_elements.max(1));
        let size = desc
            .extent
            .iter()
            .try_fold(u64::from(format.word_at_04()) * layers, |size, extent| {
                size.checked_mul(u64::from(*extent))
            })
            .and_then(|size| usize::try_from(size).ok())
            .ok_or(BackendError::OutOfHostMemory)?;
        let mut data = Vec::new();
        data.try_reserve_exact(size)
            .map_err(|_| BackendError::OutOfHostMemory)?;
        data.resize(size, 0);
        device.update_texture(
            &state.texture,
            &TextureUpdate {
                data,
                buffer_offset: 0,
                mip_level: 0,
                extent: desc.extent,
                array_layers: layers,
                access: RG_TEXTURE_ACCESS,
            },
        )
    }

    pub fn import_texture_presets_to_rg(&mut self, rg: &mut impl TextureRenderGraph) {
        for preset in self.presets.iter_mut() {
            assert!(
                preset.rg_handle.is_none(),
                "assertion failed: preset.rg_handle.is_none()"
            );
            preset.rg_handle =
                Some(rg.import_texture(&preset.texture, preset.desc, RG_TEXTURE_ACCESS));
        }
    }

    pub fn export_preset_textures_from_rg(&mut self, rg: &mut impl TextureRenderGraph) {
        for preset in self.presets.iter_mut() {
            let handle = preset.rg_handle.take().unwrap();
            rg.export_texture(handle, RG_TEXTURE_ACCESS);
        }
    }

    pub fn get_rg_texture(&self, preset: TexturePreset) -> ImportedTexture {
        self.preset_state(preset).rg_handle.unwrap()
    }

    pub fn preset(&self, preset: TexturePreset) -> Option<&PresetTextureState> {
        self.presets.get(preset.0 as usize)
    }

    /// 首次查询分配采样槽位；并发请求共享同一结果。
    pub fn handle_to_bindless_index(&self, asset: AssetIndex) -> DescriptorIndex {
        self.try_handle_to_bindless_index(asset)
            .expect("分配纹理描述符失败")
    }

    /// 容量不足或资产无效时返回错误，不消耗槽位，也不毒化索引锁。
    pub fn try_handle_to_bindless_index(
        &self,
        asset: AssetIndex,
    ) -> Result<DescriptorIndex, BackendError> {
        if asset.0 >= self.textures.len() {
            return Err(BackendError::invalid_usage("纹理资产下标越界"));
        }
        if let Some(Some(index)) = self.bindless.read().unwrap().indices.get(asset.0) {
            return Ok(*index);
        }
        let mut descriptors = self.bindless.write().unwrap();
        if let Some(Some(index)) = descriptors.indices.get(asset.0) {
            return Ok(*index);
        }
        let index = if let Some(index) = self
            .descriptor_set
            .allocate_texture(&self.textures[asset.0])?
        {
            DescriptorIndex(index)
        } else {
            let capacity = self
                .descriptor_set
                .sampled_capacity()
                .min(REQUIRED_BINDLESS_DESCRIPTOR_COUNT - RESERVED_DESCRIPTOR_COUNT);
            if descriptors.next >= capacity {
                return Err(BackendError::unsupported("纹理描述符容量已耗尽"));
            }
            let index = DescriptorIndex(descriptors.next);
            descriptors.next += 1;
            descriptors.pending.push(asset);
            index
        };
        assign_auto_resize(&mut descriptors.indices, asset.0, Some(index));
        Ok(index)
    }

    pub fn bindless_textures_dset(
        &self,
        device: &(impl TextureLibraryDevice + ?Sized),
    ) -> &DescriptorSet {
        self.try_bindless_textures_dset(device)
            .expect("提交纹理描述符失败")
    }

    /// 更新失败时保留队列，下次调用继续提交；成功后清空并复用容量。
    pub fn try_bindless_textures_dset(
        &self,
        device: &(impl TextureLibraryDevice + ?Sized),
    ) -> Result<&DescriptorSet, BackendError> {
        let mut descriptors = self.bindless.write().unwrap();
        if descriptors.pending.is_empty() {
            return Ok(&self.descriptor_set);
        }
        let updates = descriptors
            .pending
            .iter()
            .map(|asset| {
                Ok(DescriptorImageUpdate {
                    binding: 0,
                    index: descriptors.indices[asset.0].expect("资产缺少描述符索引"),
                    image_view: device.get_vk_image_view(&self.textures[asset.0], 0)?,
                    texture_id: self.textures[asset.0].id,
                    layout: ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL.as_raw() as u64,
                })
            })
            .collect::<Result<Vec<_>, BackendError>>()?;
        device.update_persistent_descriptor_set(&self.descriptor_set, &updates)?;
        descriptors.pending.clear();
        Ok(&self.descriptor_set)
    }

    /// 按名字发布只读纹理视图，包含别名；复用现有索引而不复制资产。
    pub fn named_textures(&self) -> impl Iterator<Item = (NameHash, &Texture)> {
        self.by_name.iter().filter_map(|(name, entry)| {
            self.textures
                .get(entry.handle.0)
                .map(|texture| (*name, texture))
        })
    }

    pub fn image_name_to_index(&self, name: &str) -> DescriptorIndex {
        match self.by_name.get(&NameHash::compute(name)) {
            Some(entry) => self.handle_to_bindless_index(entry.handle),
            None => panic!("No such texture: {}", name),
        }
    }

    /// 替换已有纹理，所有别名继续指向同一资产；采样者下次查询获得新描述符。
    pub fn replace(&mut self, name: NameHash, texture: Texture) -> Result<Texture, BackendError> {
        let asset = self
            .by_name
            .get(&name)
            .ok_or_else(|| BackendError::invalid_usage("待重载纹理未注册"))?
            .handle;
        let previous = std::mem::replace(&mut self.textures[asset.0], texture);
        let descriptors = self.bindless.get_mut().unwrap();
        if self.descriptor_set.heap().is_ok() {
            if let Some(index) = descriptors.indices.get_mut(asset.0).and_then(Option::take) {
                self.descriptor_set.retire(index.0);
            }
        } else if descriptors
            .indices
            .get(asset.0)
            .is_some_and(Option::is_some)
            && !descriptors.pending.contains(&asset)
        {
            // 延迟后端更新已有槽位；没有 GPU 租约的记录表不能在每次重载时永久消耗容量。
            // 已在队列中的资产直接使用最新纹理，尚未请求过描述符的资产仍保持惰性。
            descriptors.pending.push(asset);
        }
        Ok(previous)
    }

    /// 只读观察：当前待写入的 bindless 资产（测试用）。
    pub fn pending_bindless(&self) -> Vec<AssetIndex> {
        self.bindless.read().unwrap().pending.clone()
    }

    pub fn dset(&self) -> &DescriptorSet {
        &self.descriptor_set
    }
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
                return Err(BackendError::OutOfDeviceMemory);
            }
            let id = self.created.borrow().len() as u64 + 100;
            self.created.borrow_mut().push((name.to_owned(), *desc));
            Ok(Texture {
                id,
                width: desc.extent[0],
                height: desc.extent[1],
                format: desc.format,
            })
        }
    }
    impl TextureLibraryDevice for MockDevice {
        fn create_persistent_descriptor_set(
            &self,
            _name: &str,
        ) -> Result<DescriptorSet, BackendError> {
            Ok(DescriptorSet::default())
        }
        fn update_persistent_descriptor_set(
            &self,
            _d: &DescriptorSet,
            u: &[DescriptorImageUpdate],
        ) -> Result<(), BackendError> {
            self.dset_updates.borrow_mut().push(u.to_vec());
            Ok(())
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
        TextureDesc::new([8, 4, 1], 2, 0x25)
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
    fn create_name_mapping_over_real_preserves_original_on_failure() {
        let mut l = lib();
        l.add(Some("real".into()), tex(1));
        l.add(Some("other".into()), tex(2));
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            l.create_name_mapping("real", "other")
        }));
        // 冲突必须在发布别名之前发现，保留原来的真实资产。
        assert!(!l.by_name[&NameHash::compute("real")].is_virtual);
        assert_eq!(l.get_by_name(NameHash::compute("real")).id, 1);
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
    fn bindless_capacity_failure_does_not_consume_or_poison_slots() {
        let mut library = lib();
        for index in 0..=0x1fe0usize {
            library.add(None, tex(index as u64));
        }
        for index in 0..0x1fe0usize {
            assert_eq!(
                library
                    .try_handle_to_bindless_index(AssetIndex(index))
                    .unwrap(),
                DescriptorIndex(index as u32)
            );
        }
        assert!(
            library
                .try_handle_to_bindless_index(AssetIndex(0x1fe0))
                .is_err()
        );
        assert_eq!(library.bindless.read().unwrap().next, 0x1fe0);
        assert_eq!(
            library.handle_to_bindless_index(AssetIndex(0)),
            DescriptorIndex(0)
        );
    }

    #[test]
    #[should_panic(expected = "PoisonError")]
    fn bindless_poisoned_lock_unwraps() {
        let mut l = lib();
        l.add(None, tex(0));
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
                        texture_id: 2,
                        layout: 5
                    },
                    DescriptorImageUpdate {
                        binding: 0,
                        index: DescriptorIndex(1),
                        image_view: ImageView(1001),
                        texture_id: 1,
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
    fn create_preset_textures_makes_all_25_and_replaces() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures_with(&dev, desc_of);
        assert_eq!(dev.created.borrow().len(), 25);
        assert!(dev.created.borrow().iter().all(|(n, _)| n == "(NameHash)"));
        assert!((0..25u8).all(|p| l.preset(TexturePreset(p)).is_some()));
        assert!(l.preset(TexturePreset(25)).is_none());
        assert!(l.preset(TexturePreset(3)).unwrap().rg_handle.is_none());
        // 再次创建：整体替换（新纹理 id）
        let before = l.preset(TexturePreset(0)).unwrap().texture.id;
        l.create_preset_textures_with(&dev, desc_of);
        assert_ne!(l.preset(TexturePreset(0)).unwrap().texture.id, before);
    }

    #[test]
    #[should_panic(expected = "创建预设纹理失败")]
    fn create_preset_textures_unwraps_error() {
        let dev = MockDevice {
            fail_create: true,
            ..Default::default()
        };
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures_with(&dev, desc_of);
    }

    #[test]
    fn clear_preset_texture_writes_zeros_sized_by_format() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures_with(&dev, |_| TextureDesc::new([8, 4, 1], 2, 0x61)); // 8 B/texel
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
    #[should_panic(expected = "预设纹理未创建")]
    fn clear_preset_texture_missing_panics() {
        let dev = MockDevice::default();
        lib().clear_preset_texture(&dev, TexturePreset(1));
    }

    #[test]
    fn rg_import_export_roundtrip() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures_with(&dev, desc_of);
        let mut rg = RenderGraph::default();
        l.import_texture_presets_to_rg(&mut rg);
        assert_eq!(rg.imports.len(), 25);
        assert!(rg.imports.iter().all(|&(_, a)| a == 0x22));
        let h = l.get_rg_texture(TexturePreset(7));
        assert_eq!(h.access, 0x22);
        assert_eq!(h.resource, l.preset(TexturePreset(7)).unwrap().texture.id);
        l.export_preset_textures_from_rg(&mut rg);
        assert_eq!(rg.exports.len(), 25);
        assert!(l.preset(TexturePreset(7)).unwrap().rg_handle.is_none());
    }

    #[test]
    #[should_panic(expected = "assertion failed: preset.rg_handle.is_none()")]
    fn import_twice_asserts() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures_with(&dev, desc_of);
        let mut rg = RenderGraph::default();
        l.import_texture_presets_to_rg(&mut rg);
        l.import_texture_presets_to_rg(&mut rg);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn export_without_import_panics() {
        let dev = MockDevice::default();
        let mut l = AssetTextureLibrary::new(&dev);
        l.create_preset_textures_with(&dev, desc_of);
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
        l.create_preset_textures_with(&dev, desc_of);
        l.get_rg_texture(TexturePreset(1));
    }
}
