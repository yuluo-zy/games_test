use std::ops::Range;
use tg_resource_info_candidate::OriginKind;
pub use tg_resource_info_candidate::{AccessDeclaration, ExportDeclaration, ResourceInfo};
use tg_rhapsody_reconstruction::resource_info::PassResourceAccessType;
pub use tg_rhapsody_reconstruction::resource_info::{
    ResourceAccessRecord, ResourceLifetimeRecord, NO_ACCESS,
};
pub use tiny_glade_registry_pool_candidate::{
    Buffer, BufferDesc, RegistryResource, ResourceExports, ResourcePool, ResourceRegistry,
    ResourceReuseInfo, Texture, TextureDesc,
};

#[path = "../../compiled-state/lib.rs"]
pub mod batch_state;
pub type Registry = ResourceRegistry<u64, u64, ()>;
pub type Pool = ResourcePool<u64, u64>;
pub type Exports = ResourceExports<u64, u64, ()>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Descriptor {
    Image(TextureDesc),
    Buffer(BufferDesc),
}
impl Descriptor {
    pub fn kind_code(self) -> u8 {
        match self {
            Self::Image(_) => 0,
            Self::Buffer(_) => 1,
        }
    }
    pub fn usage(self) -> u32 {
        match self {
            Self::Image(desc) => desc.words_at_00_through_18[0],
            Self::Buffer(desc) => desc.word_at_08,
        }
    }
    pub fn with_usage(self, usage: u32) -> Self {
        match self {
            Self::Image(mut desc) => {
                desc.words_at_00_through_18[0] = usage;
                Self::Image(desc)
            }
            Self::Buffer(mut desc) => {
                desc.word_at_08 = usage;
                Self::Buffer(desc)
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginSource {
    Create,
    ImportOwned { token: u64, initial_access: u8 },
    ImportUnowned { token: u64, initial_access: u8 },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDeclaration {
    pub name: String,
    pub descriptor: Descriptor,
    pub source: OriginSource,
}
impl ResourceDeclaration {
    fn info_origin(&self) -> OriginKind {
        match self.source {
            OriginSource::Create => OriginKind::Tag0 {
                descriptor_code: if self.descriptor.kind_code() == 1 {
                    7
                } else {
                    0
                },
                image_creation_flags: if self.descriptor.kind_code() == 0 {
                    self.descriptor.usage()
                } else {
                    0
                },
                buffer_creation_flags: if self.descriptor.kind_code() == 1 {
                    self.descriptor.usage()
                } else {
                    0
                },
            },
            OriginSource::ImportUnowned { .. } => OriginKind::Tag1 {
                type_code: self.descriptor.kind_code(),
            },
            OriginSource::ImportOwned { .. } => OriginKind::Tag2 {
                type_code: self.descriptor.kind_code(),
            },
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PassDeclaration {
    pub name: String,
    pub accesses: Vec<AccessDeclaration>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GraphDeclaration {
    pub resources: Vec<ResourceDeclaration>,
    pub passes: Vec<PassDeclaration>,
    pub exports: Vec<ExportDeclaration>,
    pub batch_per_pass: bool,
}
pub trait AccessDependency {
    fn access_mask(&mut self, code: u8) -> u32;
    fn is_write(&mut self, code: u8) -> bool;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendFailure {
    pub discriminant: u8,
    pub message: String,
}
pub trait ResourceBackend {
    fn create_without_initial_data(
        &mut self,
        descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure>;
    fn set_debug_name(&mut self, token: u64, descriptor: Descriptor, name: &str);
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
        if let Some((failure_attempt, tag)) = self.fail_at_creation {
            if attempt == failure_attempt {
                return Err(BackendFailure {
                    discriminant: tag,
                    message: format!("oracle backend creation {attempt}"),
                });
            }
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
    pub remaining_origins: Vec<Option<ResourceDeclaration>>,
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

pub fn populate_resources(
    info: &ResourceInfo,
    declarations: &[ResourceDeclaration],
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
) -> Result<Population, PopulationFailure> {
    let mut population = Population {
        registry: Registry {
            resources: vec![ResourceReuseInfo::default(); declarations.len()],
            resource_initial_access: vec![0; declarations.len()],
            backing_resources: Vec::new(),
            exported_logical_handles: info.exported_indices.clone(),
        },
        remaining_origins: declarations.iter().cloned().map(Some).collect(),
        pool_hits: Vec::new(),
    };
    for event in &info.lifetime_events {
        if event & 0x1_0000 != 0 {
            continue;
        }
        let logical = (event & 0xffff) as usize;
        let declaration = population.remaining_origins[logical]
            .take()
            .expect("origin already consumed");
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
                    let pooled = match descriptor {
                        Descriptor::Image(desc) => {
                            pool.take_texture(&desc).map(RegistryResource::OwnedTexture)
                        }
                        Descriptor::Buffer(desc) => {
                            pool.take_buffer(&desc).map(RegistryResource::OwnedBuffer)
                        }
                    };
                    let resource = if let Some(resource) = pooled {
                        population.pool_hits.push(logical as u32);
                        resource
                    } else {
                        let token = match backend
                            .create_without_initial_data(descriptor, &declaration.name)
                        {
                            Ok(token) => token,
                            Err(error) => {
                                assert!(
                                    error.discriminant <= 10,
                                    "unsupported BackendError discriminant"
                                );
                                if error.discriminant >= 9 {
                                    panic!(
                                        "original population fatal backend class: {}",
                                        error.message
                                    );
                                }
                                return Err(PopulationFailure {
                                    error,
                                    partial: Box::new(population),
                                });
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
            population.registry.backing_resources.len() < 65536,
            "physical backing index exceeds u16"
        );
        population.registry.resources[logical] = ResourceReuseInfo {
            backing_index: population.registry.backing_resources.len() as u16,
            reused: false,
        };
        population.registry.resource_initial_access[logical] = initial_access;
        population.registry.backing_resources.push(resource);
    }
    population.remaining_origins.clear();
    Ok(population)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub resource: u32,
    pub physical: u16,
    pub from: u8,
    pub to: u8,
}
fn transition(registry: &Registry, logical: u32, from: u8, to: u8) -> Transition {
    let (payload, _, _) = registry.resource(logical);
    assert!(
        matches!(
            payload,
            RegistryResource::OwnedTexture(_)
                | RegistryResource::UnownedTexture(_)
                | RegistryResource::OwnedBuffer(_)
                | RegistryResource::UnownedBuffer(_)
        ),
        "barrier resource kind is unsupported"
    );
    Transition {
        resource: logical,
        physical: registry.resources[logical as usize].backing_index,
        from,
        to,
    }
}
pub fn initial_transitions(
    info: &ResourceInfo,
    registry: &Registry,
    pending: &mut Vec<u32>,
) -> Vec<Transition> {
    let indices = std::mem::take(pending);
    let mut transitions = Vec::new();
    for logical in indices {
        let first = info.lifetimes[logical as usize].first_access_index;
        if first == NO_ACCESS {
            continue;
        }
        let (_, initial_access, reused) = registry.resource(logical);
        if !reused {
            transitions.push(transition(
                registry,
                logical,
                initial_access,
                info.accesses[first as usize].access_code,
            ));
        }
    }
    transitions
}
pub fn pre_pass_transitions(
    info: &ResourceInfo,
    registry: &Registry,
    range: Range<usize>,
    dependency: &mut impl AccessDependency,
) -> Vec<Transition> {
    let mut transitions = Vec::new();
    for current in &info.accesses[range] {
        let (_, initial_access, reused) = registry.resource(current.resource_index);
        let from = if current.previous_access_index == NO_ACCESS {
            if !reused {
                continue;
            }
            assert_eq!(initial_access, 0, "reused first-access state must be zero");
            0
        } else {
            let previous = &info.accesses[current.previous_access_index as usize];
            let current_type = PassResourceAccessType {
                synchronization_group: current.synchronization_group,
                access: current.access_code,
            };
            let previous_type = PassResourceAccessType {
                synchronization_group: previous.synchronization_group,
                access: previous.access_code,
            };
            if current_type
                .can_be_concurrent_with(&previous_type, |code| dependency.is_write(*code))
            {
                continue;
            }
            previous.access_code
        };
        transitions.push(transition(
            registry,
            current.resource_index,
            from,
            current.access_code,
        ));
    }
    transitions
}
pub fn export_transitions(
    graph: &GraphDeclaration,
    info: &ResourceInfo,
    registry: &Registry,
) -> Vec<Transition> {
    graph
        .exports
        .iter()
        .map(|export| {
            let payload = registry.resource(export.resource_index).0;
            let buffer = matches!(
                payload,
                RegistryResource::OwnedBuffer(_) | RegistryResource::UnownedBuffer(_)
            );
            assert_eq!(buffer, export.tag & 1 != 0, "export resource kind mismatch");
            let lifetime = info.lifetimes[export.resource_index as usize];
            let from = if lifetime.last_access_index == NO_ACCESS {
                registry.resource(export.resource_index).1
            } else {
                info.accesses[lifetime.last_access_index as usize].access_code
            };
            transition(registry, export.resource_index, from, export.access_code)
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalMapping {
    pub logical: u32,
    pub physical: u16,
    pub reused: bool,
    pub initial_access: u8,
    pub payload_tag: u8,
    pub token: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub pass_order: Vec<String>,
    pub pass_ranges: Vec<Range<usize>>,
    pub accesses: Vec<ResourceAccessRecord>,
    pub lifetimes: Vec<ResourceLifetimeRecord>,
    pub image_usage: Vec<u32>,
    pub buffer_usage: Vec<u32>,
    pub lifetime_events: Vec<u32>,
    pub physical_mapping: Vec<PhysicalMapping>,
    pub pending_initial: Vec<u32>,
    pub initial: Vec<Transition>,
    pub pre: Vec<Vec<Transition>>,
    pub export: Vec<Transition>,
    pub batches: Vec<batch_state::Batch>,
    pub imports: Vec<u32>,
    pub exports: Vec<u16>,
    pub pool_hits: Vec<u32>,
}
pub struct CompiledGraph {
    pub declarations: GraphDeclaration,
    pub info: ResourceInfo,
    pub registry: Registry,
    pub snapshot: Snapshot,
    pub pending_initial: Vec<u32>,
}
impl CompiledGraph {
    pub fn into_exports(self) -> Exports {
        self.registry.into_exports()
    }
}
pub fn compile_graph(
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &mut impl AccessDependency,
) -> Result<CompiledGraph, PopulationFailure> {
    assert!(
        graph.resources.len() <= 65536,
        "resource count exceeds lifetime packing domain"
    );
    assert!(
        graph.passes.len() <= 32768,
        "pass count exceeds lifetime packing domain"
    );
    let origins = graph
        .resources
        .iter()
        .map(ResourceDeclaration::info_origin)
        .collect::<Vec<_>>();
    let pass_slices = graph
        .passes
        .iter()
        .map(|pass| pass.accesses.as_slice())
        .collect::<Vec<_>>();
    let mut info = ResourceInfo::default();
    info.calculate(&origins, &graph.exports, &pass_slices, |code| {
        dependency.access_mask(code)
    });
    let population = populate_resources(&info, &graph.resources, pool, backend)?;
    let registry = population.registry;
    let pending_initial = info
        .lifetimes
        .iter()
        .enumerate()
        .filter_map(|(index, lifetime)| {
            (lifetime.first_access_index != NO_ACCESS).then_some(index as u32)
        })
        .collect::<Vec<_>>();
    let mut pending_for_snapshot = pending_initial.clone();
    let initial = initial_transitions(&info, &registry, &mut pending_for_snapshot);
    let pre = info
        .pass_access_ranges
        .iter()
        .map(|(start, end)| {
            pre_pass_transitions(
                &info,
                &registry,
                start.0 as usize..end.0 as usize,
                dependency,
            )
        })
        .collect();
    let mut compiler = batch_state::BatchCompiler::new();
    if graph.batch_per_pass {
        for pass in 0..graph.passes.len() {
            compiler.flush_batch(pass);
        }
    }
    let batches = compiler.finish(graph.passes.len());
    let export = if batches.last().is_some_and(|batch| batch.export_barriers) {
        export_transitions(&graph, &info, &registry)
    } else {
        Vec::new()
    };
    let physical_mapping = registry
        .resources
        .iter()
        .enumerate()
        .map(|(logical, reuse)| {
            let (payload_tag, token) =
                if (reuse.backing_index as usize) < registry.backing_resources.len() {
                    payload_projection(&registry.backing_resources[reuse.backing_index as usize])
                } else {
                    (255, None)
                };
            PhysicalMapping {
                logical: logical as u32,
                physical: reuse.backing_index,
                reused: reuse.reused,
                initial_access: registry.resource_initial_access[logical],
                payload_tag,
                token,
            }
        })
        .collect();
    let mut exports = info.exported_indices.iter().copied().collect::<Vec<_>>();
    exports.sort_unstable();
    let snapshot = Snapshot {
        pass_order: graph.passes.iter().map(|pass| pass.name.clone()).collect(),
        pass_ranges: info
            .pass_access_ranges
            .iter()
            .map(|(start, end)| start.0 as usize..end.0 as usize)
            .collect(),
        accesses: info.accesses.clone(),
        lifetimes: info.lifetimes.clone(),
        image_usage: info.image_usage_flags.clone(),
        buffer_usage: info.buffer_usage_flags.clone(),
        lifetime_events: info.lifetime_events.clone(),
        physical_mapping,
        pending_initial: pending_initial.clone(),
        initial,
        pre,
        export,
        batches,
        imports: graph
            .resources
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                (!matches!(declaration.source, OriginSource::Create)).then_some(index as u32)
            })
            .collect(),
        exports,
        pool_hits: population.pool_hits,
    };
    Ok(CompiledGraph {
        declarations: graph,
        info,
        registry,
        snapshot,
        pending_initial,
    })
}
pub fn payload_projection(resource: &RegistryResource<u64, u64, ()>) -> (u8, Option<u64>) {
    match resource {
        RegistryResource::OwnedTexture(texture) => (0, Some(texture.handle)),
        RegistryResource::UnownedTexture(texture) => (1, Some(texture.handle)),
        RegistryResource::OwnedBuffer(buffer) => (2, Some(buffer.handle)),
        RegistryResource::UnownedBuffer(buffer) => (3, Some(buffer.handle)),
        RegistryResource::Other(_) => (4, None),
        RegistryResource::Empty => (5, None),
    }
}
pub mod dsl;
pub mod snapshot;
