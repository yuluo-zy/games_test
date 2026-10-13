//! `AssetTextureLibrary` — texture registry + bindless descriptor table
//! (WP4-owned, render-facing surface).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/asset_libraries/texture_library.rs`
//! (source_sha256 `c5db910c…`, 13 procs). Decompiles:
//! `_analysis/decomp/C_0x14093bb10.c`, `C_0x14093d040.c`, `C_0x14093d310.c`,
//! `D_0x14093c720.c`, `D_0x14093cf30.c`, `E_0x14093c200.c`.
//!
//! Bindless convention (from `evidence/render-passes/INDEX.md`): textures are
//! bound through a persistent descriptor set named `"bindless_textures"`;
//! `nani_uber` compiles a `bindless_textures_dyn@2.0` variant.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::rhapsody_contract::{GraphicsDevice, Texture, TextureDesc};

/// Name hash (`utils::name_hash::NameHash`, O: 64-bit).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NameHash(u64);

impl NameHash {
    /// `compute(&str)` — hash a texture name. Exact algorithm U (the original
    /// uses `utils::name_hash`); FNV-1a stands in for the skeleton.
    pub fn compute(name: &str) -> Self {
        let mut h = NoHashHasher(0xcbf29ce484222325);
        h.write(name.as_bytes());
        NameHash(h.finish())
    }
}

/// No-op hasher (observed NoHashHasher usage).
#[derive(Default, Clone, Copy)]
pub struct NoHashHasher(u64);
impl Hasher for NoHashHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(0x100000001b3) ^ u64::from(b);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

/// A named texture entry (`NamedTexture`, 0x78+ bytes, O: Vec stride 0x38).
#[derive(Clone, Debug)]
pub struct NamedTexture {
    pub name: Option<String>,
    pub texture: Texture,
}

/// Bindless descriptor index for a texture (O: `DescriptorIndex`, u32 slot).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DescriptorIndex(u32);

impl DescriptorIndex {
    /// `allocate` (O name; U body — counter-based in the original).
    pub fn allocate() -> Self {
        DescriptorIndex(0)
    }
}

/// Bindless texture library.
/// `impl Resource`: `nani_update_uber` / `nani_update_cloth` /
/// `nani_update_solid_vertex_color_window_glass` 以 `Res<AssetTextureLibrary>` 使用（O）。
#[derive(bevy_ecs::resource::Resource)]
pub struct AssetTextureLibrary {
    /// O: `Vec<NamedTexture>` (stride 0x38).
    textures: Vec<NamedTexture>,
    /// O: `HashMap<NameHash, NamedTexture>` (NoHashHasher; buckets +0x30, mask
    /// +0x38, items +0x48).
    by_name: HashMap<NameHash, NamedTexture, NoHashBuildHasher>,
    /// O: persistent descriptor set created in `new`.
    pub bindless_dset: crate::rhapsody_contract::DescriptorSet,
    /// Bindless table (O: `handle_to_bindless_index`): pending handle list +
    /// dirty flag flushed by `bindless_textures_dset`.
    bindless: RwLock<BindlessTable>,
}

/// O: +0x80 lock, +0x88 dirty flag, +0x90/+0x98/+0xa0 pending Vec, +0xb0/+0xb8
/// index table, +0xc0 count.
#[derive(Default)]
struct BindlessTable {
    dirty: bool,
    pending: Vec<u64>,
    indices: HashMap<u64, u32>,
}

impl AssetTextureLibrary {
    /// `new` (O: 0x14093bb10): allocate `"bindless_textures"` name,
    /// `GraphicsDevice::create_persistent_descriptor_set()`.
    pub fn new(device: &GraphicsDevice) -> Self {
        let dset = device.create_persistent_descriptor_set("bindless_textures");
        AssetTextureLibrary {
            textures: Vec::new(),
            by_name: HashMap::default(),
            bindless_dset: dset,
            bindless: RwLock::new(BindlessTable::default()),
        }
    }

    /// `add(NamedTexture)` (O: 0x14093c200): duplicate name → panic
    /// `"the name is already in the texture library"`; else push to `textures`
    /// and insert NameHash→NamedTexture; debug-log `"Loaded texture as …"`.
    pub fn add(&mut self, named: NamedTexture) {
        if let Some(name) = named.name.clone().as_deref() {
            let hash = NameHash::compute(name);
            if self.by_name.contains_key(&hash) {
                panic!(
                    "AssetTextureLibrary: the name {:?} is already in the texture library",
                    name
                );
            }
            self.textures.push(named.clone());
            self.by_name.insert(hash, named);
            log::debug!(target: "country_core::asset_libraries::texture_library", "Loaded texture as {:?}", name);
        }
    }

    /// `create_name_mapping(name, texture)` (O: 0x14093c720): registers a preset
    /// name→texture mapping; duplicate → panic `"Onoz, already exists …"`,
    /// missing target → panic `"Was asked to map to …"`.
    pub fn create_name_mapping(&mut self, name: &str, texture: Texture) {
        let hash = NameHash::compute(name);
        if self.by_name.contains_key(&hash) {
            panic!(
                "Onoz, already exists and is a registered texture: {:?}",
                name
            );
        }
        let named = NamedTexture {
            name: Some(name.to_owned()),
            texture,
        };
        self.textures.push(named.clone());
        self.by_name.insert(hash, named);
    }

    /// `get_by_name(name)` (O: 0x14093ca90).
    pub fn get_by_name(&self, name: &str) -> Option<&Texture> {
        self.by_name
            .get(&NameHash::compute(name))
            .map(|n| &n.texture)
    }

    /// `get_handle_and_texture_by_name` (O: 0x14093c970).
    pub fn get_handle_and_texture_by_name(
        &self,
        name: &str,
    ) -> Option<(Handle<Texture>, &Texture)> {
        self.by_name
            .get(&NameHash::compute(name))
            .map(|n| (Handle::new(), &n.texture))
    }

    /// `get_handle_by_name` (O name).
    pub fn get_handle_by_name(&self, name: &str) -> Option<Handle<Texture>> {
        self.by_name
            .get(&NameHash::compute(name))
            .map(|_| Handle::new())
    }

    /// `get(handle)` (O name).
    pub fn get(&self, _handle: Handle<Texture>) -> Option<&Texture> {
        // U: handle→texture map not recovered.
        None
    }

    /// `get_mut` / `get_mut_by_name` (O names).
    pub fn get_mut(&mut self, _handle: Handle<Texture>) -> Option<&mut Texture> {
        None
    }
    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut Texture> {
        self.by_name
            .get_mut(&NameHash::compute(name))
            .map(|n| &mut n.texture)
    }

    /// `get_rg_texture(handle) -> Texture` (O: 0x14093cf30): returns the texture
    /// for a handle; missing → `expect` panic.
    pub fn get_rg_texture(&self, _handle: Handle<Texture>) -> Texture {
        // U: the handle→Texture map (stride 0x68) is a typed gap.
        panic!("get_rg_texture: handle not mapped (typed gap)")
    }

    /// `handle_to_bindless_index(&mut self, handle) -> Option<DescriptorIndex>`
    /// (O: 0x14093d310): returns an existing bindless index, or allocates a new
    /// one (limit 0x1fdf = 8191, matching the persistent bindless table size),
    /// pushes the handle to the pending list and marks the table dirty.
    pub fn handle_to_bindless_index(&mut self, handle: Handle<Texture>) -> Option<DescriptorIndex> {
        let mut table = self.bindless.write();
        let raw = handle.id().raw();
        if let Some(&idx) = table.indices.get(&raw) {
            return Some(DescriptorIndex(idx));
        }
        if table.indices.len() > 0x1fdf {
            panic!("bindless texture table full");
        }
        let idx = table.indices.len() as u32;
        table.indices.insert(raw, idx);
        table.pending.push(raw);
        table.dirty = true;
        Some(DescriptorIndex(idx))
    }

    /// `bindless_textures_dset` (O: 0x14093d040): flush pending bindless updates
    /// into the persistent descriptor set (`update_persistent_descriptor_set`).
    pub fn bindless_textures_dset(&mut self) {
        let mut table = self.bindless.write();
        assert!(!table.dirty, "bindless update already in progress");
        if table.pending.is_empty() {
            return;
        }
        // O: drain pending handles; update the persistent descriptor set.
        table.pending.clear();
        table.dirty = false;
    }

    /// `update_bindless_texture_table` (O name; U body).
    pub fn update_bindless_texture_table(&mut self) {
        self.bindless_textures_dset();
    }

    /// `create_preset_textures` / `clear_preset_texture` /
    /// `import_texture_presets_to_rg` / `export_preset_textures_from_rg` —
    /// preset texture lifecycle (O call-graph; bodies U).
    pub fn create_preset_textures(&mut self) {
        // U
    }
    pub fn clear_preset_texture(&mut self) {
        // U
    }
    pub fn import_texture_presets_to_rg(&mut self) {
        // U
    }
    pub fn export_preset_textures_from_rg(&mut self) {
        // U
    }
}

/// `add_raw` (O name) — registers a raw (non-sampled) texture.
pub type RawTexture = TextureDesc;
