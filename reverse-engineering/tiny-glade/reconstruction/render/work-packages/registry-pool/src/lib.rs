use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureDesc {
    pub words_at_00_through_18: [u32; 7],
    pub bytes_at_1c_through_1e: [u8; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BufferDesc {
    pub word_at_00: u64,
    pub word_at_08: u32,
    pub bytes_at_0c_through_0e: [u8; 3],
}

#[derive(Debug, PartialEq, Eq)]
pub struct Texture<Handle> {
    pub handle: Handle,
    pub desc: TextureDesc,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Buffer<Handle> {
    pub handle: Handle,
    pub desc: BufferDesc,
}

pub struct ResourcePool<TextureHandle, BufferHandle> {
    textures: HashMap<TextureDesc, Vec<Texture<TextureHandle>>>,
    buffers: HashMap<BufferDesc, Vec<Buffer<BufferHandle>>>,
}

impl<TextureHandle, BufferHandle> Default for ResourcePool<TextureHandle, BufferHandle> {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
            buffers: HashMap::new(),
        }
    }
}

impl<TextureHandle, BufferHandle> ResourcePool<TextureHandle, BufferHandle> {
    pub fn take_texture(&mut self, desc: &TextureDesc) -> Option<Texture<TextureHandle>> {
        self.textures.get_mut(desc).and_then(Vec::pop)
    }

    pub fn insert_texture(&mut self, texture: Texture<TextureHandle>) {
        self.textures.entry(texture.desc).or_default().push(texture);
    }

    pub fn take_buffer(&mut self, desc: &BufferDesc) -> Option<Buffer<BufferHandle>> {
        self.buffers.get_mut(desc).and_then(Vec::pop)
    }

    pub fn insert_buffer(&mut self, buffer: Buffer<BufferHandle>) {
        self.buffers.entry(buffer.desc).or_default().push(buffer);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
        self.buffers.clear();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceReuseInfo {
    pub backing_index: u16,
    pub reused: bool,
}

impl Default for ResourceReuseInfo {
    fn default() -> Self {
        Self {
            backing_index: u16::MAX,
            reused: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegistryResource<TextureHandle, BufferHandle, Other> {
    OwnedTexture(Texture<TextureHandle>),
    UnownedTexture(Texture<TextureHandle>),
    OwnedBuffer(Buffer<BufferHandle>),
    UnownedBuffer(Buffer<BufferHandle>),
    Other(Other),
    Empty,
}

pub struct ResourceRegistry<TextureHandle, BufferHandle, Other> {
    pub resources: Vec<ResourceReuseInfo>,
    pub resource_initial_access: Vec<u8>,
    pub backing_resources: Vec<RegistryResource<TextureHandle, BufferHandle, Other>>,
    pub exported_logical_handles: HashSet<u16>,
}

pub struct ResourceExports<TextureHandle, BufferHandle, Other> {
    pub remaining: Vec<RegistryResource<TextureHandle, BufferHandle, Other>>,
    pub exported: HashMap<u16, RegistryResource<TextureHandle, BufferHandle, Other>>,
}

impl<TextureHandle, BufferHandle, Other> ResourceRegistry<TextureHandle, BufferHandle, Other> {
    fn backing(&self, logical: u32) -> &RegistryResource<TextureHandle, BufferHandle, Other> {
        let reuse = self.resources[logical as usize];
        &self.backing_resources[usize::from(reuse.backing_index)]
    }

    pub fn resource(
        &self,
        logical: u32,
    ) -> (
        &RegistryResource<TextureHandle, BufferHandle, Other>,
        u8,
        bool,
    ) {
        let backing = self.backing(logical);
        let initial_access = self.resource_initial_access[logical as usize];
        (
            backing,
            initial_access,
            self.resources[logical as usize].reused,
        )
    }

    pub fn image_from_raw_handle(&self, logical: u32) -> &Texture<TextureHandle> {
        match self.backing(logical) {
            RegistryResource::OwnedTexture(texture) | RegistryResource::UnownedTexture(texture) => {
                texture
            }
            _ => panic!("resource is not a texture"),
        }
    }

    pub fn buffer(&self, logical: u32) -> &Buffer<BufferHandle> {
        match self.backing(logical) {
            RegistryResource::OwnedBuffer(buffer) | RegistryResource::UnownedBuffer(buffer) => {
                buffer
            }
            _ => panic!("resource is not a buffer"),
        }
    }

    pub fn into_exports(mut self) -> ResourceExports<TextureHandle, BufferHandle, Other> {
        let exported = self
            .exported_logical_handles
            .into_iter()
            .map(|logical| {
                let physical = self.resources[usize::from(logical)].backing_index;
                let resource = std::mem::replace(
                    &mut self.backing_resources[usize::from(physical)],
                    RegistryResource::Empty,
                );
                (logical, resource)
            })
            .collect();
        self.backing_resources
            .retain(|resource| !matches!(resource, RegistryResource::Empty));
        ResourceExports {
            remaining: self.backing_resources,
            exported,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn texture_desc() -> TextureDesc {
        TextureDesc {
            words_at_00_through_18: [1, 2, 3, 4, 5, 6, 7],
            bytes_at_1c_through_1e: [1, 2, 3],
        }
    }

    fn buffer_desc() -> BufferDesc {
        BufferDesc {
            word_at_00: 4096,
            word_at_08: 7,
            bytes_at_0c_through_0e: [1, 2, 3],
        }
    }

    #[test]
    fn pool_is_exact_key_lifo_and_retains_empty_buckets() {
        let mut pool: ResourcePool<String, String> = ResourcePool::default();
        let desc = texture_desc();
        pool.insert_texture(Texture {
            handle: "first".into(),
            desc,
        });
        pool.insert_texture(Texture {
            handle: "second".into(),
            desc,
        });
        let mut mismatch = desc;
        mismatch.words_at_00_through_18[0] += 1;
        assert!(pool.take_texture(&mismatch).is_none());
        assert_eq!(pool.take_texture(&desc).unwrap().handle, "second");
        assert_eq!(pool.take_texture(&desc).unwrap().handle, "first");
        assert!(pool.take_texture(&desc).is_none());
        assert!(pool.textures.contains_key(&desc));
        let desc = buffer_desc();
        pool.insert_buffer(Buffer {
            handle: "first".into(),
            desc,
        });
        pool.insert_buffer(Buffer {
            handle: "second".into(),
            desc,
        });
        let mut mismatch = desc;
        mismatch.bytes_at_0c_through_0e[2] += 1;
        assert!(pool.take_buffer(&mismatch).is_none());
        assert_eq!(pool.take_buffer(&desc).unwrap().handle, "second");
        assert_eq!(pool.take_buffer(&desc).unwrap().handle, "first");
        assert!(pool.buffers.contains_key(&desc));
    }

    struct DropProbe(Rc<Cell<usize>>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn moves_nonclone_handles_and_clear_drops_once_retaining_capacity() {
        let count = Rc::new(Cell::new(0));
        let mut pool: ResourcePool<DropProbe, DropProbe> = ResourcePool::default();
        let desc = texture_desc();
        pool.insert_texture(Texture {
            handle: DropProbe(count.clone()),
            desc,
        });
        let moved = pool.take_texture(&desc).unwrap();
        assert_eq!(count.get(), 0);
        pool.insert_texture(moved);
        pool.insert_buffer(Buffer {
            handle: DropProbe(count.clone()),
            desc: buffer_desc(),
        });
        let capacities = (pool.textures.capacity(), pool.buffers.capacity());
        pool.clear();
        assert_eq!(count.get(), 2);
        assert_eq!(
            (pool.textures.capacity(), pool.buffers.capacity()),
            capacities
        );
        drop(pool);
        assert_eq!(count.get(), 2);
    }

    fn registry() -> ResourceRegistry<String, String, String> {
        ResourceRegistry {
            resources: vec![
                ResourceReuseInfo {
                    backing_index: 1,
                    reused: true,
                },
                ResourceReuseInfo {
                    backing_index: 0,
                    reused: false,
                },
                ResourceReuseInfo {
                    backing_index: 2,
                    reused: false,
                },
            ],
            resource_initial_access: vec![9, 8, 7],
            backing_resources: vec![
                RegistryResource::OwnedBuffer(Buffer {
                    handle: "buffer".into(),
                    desc: buffer_desc(),
                }),
                RegistryResource::UnownedTexture(Texture {
                    handle: "texture".into(),
                    desc: texture_desc(),
                }),
                RegistryResource::Other("other".into()),
                RegistryResource::Empty,
            ],
            exported_logical_handles: HashSet::from([0]),
        }
    }

    #[test]
    fn accessors_follow_logical_mapping_and_preserve_metadata() {
        let registry = registry();
        assert_eq!(registry.image_from_raw_handle(0).handle, "texture");
        assert_eq!(registry.buffer(1).handle, "buffer");
        let (_, initial, reused) = registry.resource(0);
        assert_eq!((initial, reused), (9, true));
        assert!(matches!(registry.resource(2).0, RegistryResource::Other(_)));
    }

    #[test]
    #[should_panic(expected = "resource is not a texture")]
    fn wrong_variant_panics() {
        registry().image_from_raw_handle(1);
    }

    #[test]
    fn exports_move_payload_and_keep_remaining_order() {
        let exports = registry().into_exports();
        assert!(matches!(
            exports.exported.get(&0),
            Some(RegistryResource::UnownedTexture(_))
        ));
        assert_eq!(exports.remaining.len(), 2);
        assert!(matches!(
            exports.remaining[0],
            RegistryResource::OwnedBuffer(_)
        ));
        assert!(matches!(exports.remaining[1], RegistryResource::Other(_)));
    }

    #[test]
    fn aliased_export_slot_is_empty_after_first_move() {
        let mut registry = registry();
        registry.resources[2].backing_index = 1;
        registry.exported_logical_handles.insert(2);
        let exports = registry.into_exports();
        assert_eq!(exports.exported.len(), 2);
        assert_eq!(
            exports
                .exported
                .values()
                .filter(|resource| matches!(resource, RegistryResource::Empty))
                .count(),
            1
        );
        assert_eq!(
            exports
                .exported
                .values()
                .filter(|resource| matches!(resource, RegistryResource::UnownedTexture(_)))
                .count(),
            1
        );
    }
}
