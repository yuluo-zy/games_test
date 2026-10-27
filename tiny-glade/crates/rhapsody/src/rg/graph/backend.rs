//! Backend contracts: resource population, events and failures.
use crate::rg::resource_info::ResourceInfo;
use crate::rg::resource_registry::Buffer;
use crate::rg::resource_registry::RegistryResource;
use crate::rg::resource_registry::ResourceReuseInfo;
use crate::rg::resource_registry::Texture;
use super::Pool;
use super::Registry;
use super::declaration::Descriptor;
use super::declaration::GraphDeclaration;
use super::declaration::OriginSource;
use super::declaration::ResourceDeclaration;

/// 资源创建错误保留设备丢失、显存不足等类别。
pub type BackendFailure = crate::Error;

pub trait ResourceBackend {
    fn frame_storage_pool(&self) -> Option<&super::super::frame_storage::FrameStoragePool> {
        None
    }

    fn history_validity(&self, _token: u64) -> Option<bool> {
        None
    }

    fn prepare_transient_resources(
        &mut self,
        _plan: &super::super::plan::CompiledPlan,
        _graph: &GraphDeclaration,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn has_prepared_resource(&self, _name: &str) -> bool {
        false
    }
    fn is_transient_alias(&self, _token: u64) -> bool {
        false
    }
    /// Whether the pooled `token` can serve a declaration with `sharing`.
    /// Accepting hands the resource to the frame (the pool removes the entry
    /// right after), so a backend that parks idle resources must take it
    /// back here; a token it no longer has (evicted) is refused.
    fn claim_pooled(&mut self, _token: u64, _sharing: crate::tracking::ResourceSharing) -> bool {
        true
    }
    /// The pool took `token` back after a failed frame (or completion): the
    /// backend may park its resource until it is claimed or evicted.
    fn park_pooled(&mut self, _token: u64) {}
    fn device_generation(&self) -> Option<crate::sync::DeviceGeneration> {
        None
    }
    fn create_without_initial_data(
        &mut self,
        descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure>;
    fn set_debug_name(&mut self, token: u64, descriptor: Descriptor, name: &str);
    /// 失败路径归还：`create_without_initial_data` 创建、但本次图因失败不会再使用的 token。
    /// 资源填充失败时的回滚钩子；默认不做任何事。
    fn release_created_resource(&mut self, _token: u64) {}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendEvent {
    CreateWithoutInitialData {
        descriptor: Descriptor,
        name: String,
        token: u64,
    },
    DebugName {
        token: u64,
        name: String,
    },
}

#[derive(Default)]
pub struct EventBackend {
    pub events: Vec<BackendEvent>,
    pub fail_at_creation: Option<(usize, u8)>,
    next_token: u64,
    creations: usize,
}

impl ResourceBackend for EventBackend {
    fn create_without_initial_data(
        &mut self,
        descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure> {
        let attempt = self.creations;
        self.creations += 1;
        if let Some((failure_attempt, tag)) = self.fail_at_creation
            && attempt == failure_attempt
        {
            return Err(crate::Error::invalid_usage(format!(
                "injected resource creation {attempt}, class {tag}"
            )));
        }
        let token = self.next_token;
        self.next_token += 1;
        self.events.push(BackendEvent::CreateWithoutInitialData {
            descriptor,
            name: name.to_owned(),
            token,
        });
        Ok(token)
    }
    fn set_debug_name(&mut self, token: u64, _descriptor: Descriptor, name: &str) {
        self.events.push(BackendEvent::DebugName {
            token,
            name: name.to_owned(),
        });
    }
}

pub struct Population {
    pub registry: Registry,
    pub pool_hits: Vec<u32>,
}

pub struct PopulationFailure {
    pub error: BackendFailure,
    pub partial: Box<Population>,
}

impl std::fmt::Debug for PopulationFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PopulationFailure")
            .field("error", &self.error)
            .field(
                "backing_count",
                &self.partial.registry.backing_resources.len(),
            )
            .finish()
    }
}

pub(super) fn populate_resources_into(
    info: &ResourceInfo,
    declarations: &[ResourceDeclaration],
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    registry: &mut Registry,
    pool_hits: &mut Vec<u32>,
) -> crate::Result<()> {
    // Declarations remain frame-owned. Borrowing the origin avoids cloning
    // every name and descriptor when the immutable plan is already cached.
    fn reserve<T>(values: &mut Vec<T>, len: usize) -> crate::Result<()> {
        if values.capacity() < len {
            values
                .try_reserve_exact(len.saturating_sub(values.len()))
                .map_err(|_| crate::Error::OutOfHostMemory)?;
        }
        Ok(())
    }
    reserve(&mut registry.resources, declarations.len())?;
    reserve(&mut registry.resource_initial_access, declarations.len())?;
    reserve(&mut registry.backing_resources, declarations.len())?;
    reserve(
        &mut registry.exported_logical_handles,
        info.exported_indices.len(),
    )?;
    reserve(pool_hits, declarations.len())?;
    registry.resources.clear();
    registry
        .resources
        .resize(declarations.len(), ResourceReuseInfo::default());
    registry.resource_initial_access.clear();
    registry
        .resource_initial_access
        .resize(declarations.len(), 0);
    registry.backing_resources.clear();
    registry.exported_logical_handles.clear();
    registry
        .exported_logical_handles
        .extend(info.exported_indices.iter().copied());
    pool_hits.clear();
    for event in &info.lifetime_events {
        if event.release {
            continue;
        }
        let logical = usize::from(event.resource);
        let declaration = &declarations[logical];
        assert_eq!(
            registry.resources[logical].backing_index,
            u16::MAX,
            "origin already consumed"
        );
        let initial_access = match declaration.source {
            OriginSource::Create => 0,
            OriginSource::ImportOwned { initial_access, .. }
            | OriginSource::ImportUnowned { initial_access, .. } => initial_access,
        };
        let resource = match declaration.source {
            OriginSource::Create => {
                let usage = match declaration.descriptor {
                    Descriptor::Image(_) => info.image_usage_flags[logical],
                    Descriptor::Buffer(_) => info.buffer_usage_flags[logical],
                };
                let descriptor = declaration.descriptor.with_usage(usage);
                if usage == 0 {
                    RegistryResource::Empty
                } else {
                    let pooled = if backend.has_prepared_resource(&declaration.name) {
                        None
                    } else {
                        match descriptor {
                            Descriptor::Image(desc) => pool
                                .take_texture_where(&desc, |token| {
                                    backend.claim_pooled(*token, declaration.sharing)
                                })
                                .map(RegistryResource::OwnedTexture),
                            Descriptor::Buffer(desc) => pool
                                .take_buffer_where(&desc, |token| {
                                    backend.claim_pooled(*token, declaration.sharing)
                                })
                                .map(RegistryResource::OwnedBuffer),
                        }
                    };
                    let resource = if let Some(resource) = pooled {
                        pool_hits.push(logical as u32);
                        resource
                    } else {
                        let token = match backend
                            .create_without_initial_data(descriptor, &declaration.name)
                        {
                            Ok(token) => token,
                            Err(error) => {
                                return Err(error);
                            }
                        };
                        match descriptor {
                            Descriptor::Image(desc) => RegistryResource::OwnedTexture(Texture {
                                handle: token,
                                desc,
                            }),
                            Descriptor::Buffer(desc) => RegistryResource::OwnedBuffer(Buffer {
                                handle: token,
                                desc,
                            }),
                        }
                    };
                    let token = match &resource {
                        RegistryResource::OwnedTexture(texture) => texture.handle,
                        RegistryResource::OwnedBuffer(buffer) => buffer.handle,
                        _ => unreachable!(),
                    };
                    backend.set_debug_name(token, descriptor, &declaration.name);
                    resource
                }
            }
            OriginSource::ImportOwned { token, .. } => match declaration.descriptor {
                Descriptor::Image(desc) => RegistryResource::OwnedTexture(Texture {
                    handle: token,
                    desc,
                }),
                Descriptor::Buffer(desc) => RegistryResource::OwnedBuffer(Buffer {
                    handle: token,
                    desc,
                }),
            },
            OriginSource::ImportUnowned { token, .. } => match declaration.descriptor {
                Descriptor::Image(desc) => RegistryResource::UnownedTexture(Texture {
                    handle: token,
                    desc,
                }),
                Descriptor::Buffer(desc) => RegistryResource::UnownedBuffer(Buffer {
                    handle: token,
                    desc,
                }),
            },
        };
        assert!(
            registry.backing_resources.len() < 65536,
            "physical backing index exceeds u16"
        );
        registry.resources[logical] = ResourceReuseInfo {
            backing_index: registry.backing_resources.len() as u16,
            reused: pool_hits.last() == Some(&(logical as u32)),
        };
        registry.resource_initial_access[logical] = initial_access;
        registry.backing_resources.push(resource);
    }
    Ok(())
}
