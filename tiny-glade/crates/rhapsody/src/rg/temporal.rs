//! Session-owned named resources and transactional history publication.
//! Physical identifiers and allocation callbacks stay inside the backend.
use super::{
    graph::{BuildError, Descriptor, FrameInstance, RenderGraph},
    resource::ResourceHandle,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
pub(crate) const TEMPORAL_EXPORT_ACCESS: u8 = 0x37;

/// A graph-scoped named texture. Allocation reuse does not imply valid contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedTexture {
    pub texture: super::resource::TextureId,
    pub history_valid: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedBuffer {
    pub buffer: super::resource::BufferId,
    pub history_valid: bool,
}
/// Current and preceding members are selected by the session's committed parity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalTexturePair {
    current: NamedTexture,
    previous: NamedTexture,
    parity: u32,
}
impl TemporalTexturePair {
    pub fn current(self) -> super::resource::TextureId {
        self.current.texture
    }
    pub fn previous(self) -> super::resource::TextureId {
        self.previous.texture
    }
    pub fn history_valid(self) -> bool {
        self.previous.history_valid
    }
    pub fn current_history_valid(self) -> bool {
        self.current.history_valid
    }
    pub fn parity(self) -> u32 {
        self.parity
    }
    pub(crate) fn from_lookups(
        graph: &RenderGraph,
        pair: TemporalPair,
    ) -> Result<Self, BuildError> {
        Ok(Self {
            current: NamedTexture {
                texture: graph.texture_id(pair.first.handle)?,
                history_valid: pair.first.reused,
            },
            previous: NamedTexture {
                texture: graph.texture_id(pair.second.handle)?,
                history_valid: pair.second.reused,
            },
            parity: pair.parity,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TemporalResource {
    pub descriptor: Descriptor,
    pub token: u64,
    pub access: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TemporalRecordState {
    RawTexture(ResourceHandle),
    RawBuffer(ResourceHandle),
    Exported,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TemporalLookup {
    pub handle: ResourceHandle,
    pub resource: TemporalResource,
    /// Whether the allocation contains history from a confirmed export. An
    /// existing allocation may still require initialization after a failed frame.
    pub reused: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TemporalPair {
    pub first: TemporalLookup,
    pub second: TemporalLookup,
    pub parity: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemporalError {
    AlreadyRequested(Arc<str>),
    WrongResourceKind(Arc<str>),
    DescriptorMismatch(Arc<str>),
    Backend(BuildError),
    ExportMissing(Arc<str>),
    ExportTokenMismatch(Arc<str>),
}
/// 后端拥有真实资源引用；释放缓存引用不等于销毁 GPU 正在使用的对象。
pub(crate) trait TemporalResourceOwner {
    fn create_temporal(&mut self, name: &str, descriptor: Descriptor) -> Result<u64, BuildError>;
    fn release_temporal(&mut self, token: u64);
    /// True only when this owner actually uploaded the complete allocation
    /// during creation. This does not publish reusable frame history.
    fn initial_contents_defined(&self, _token: u64) -> bool {
        false
    }
    fn temporal_sharing(&self, _name: &str) -> crate::resource::ResourceSharing {
        crate::resource::ResourceSharing::Exclusive
    }
}
#[derive(Clone, Copy, Debug)]
struct Record {
    sharing: crate::resource::ResourceSharing,
    resource: TemporalResource,
    state: TemporalRecordState,
    /// GPU 上真正执行过的访问。`export_temporal` 在导出时就把
    /// `resource.access` 写成 0x37，若该帧没有执行，下一帧会带着与实际布局不符的初始访问导入；
    /// [`TemporalRenderGraphState::abandon_frame`] 据此回退。新建资源为 0（undefined）。
    confirmed_access: u8,
    /// New allocations have no history, and a partially submitted frame may
    /// have overwritten only part of existing contents. Keep the allocation
    /// and layout, but require initialization before using it as history.
    history_invalid: bool,
    /// Physical creation may upload known contents even though no frame has
    /// published history yet (native temporal buffers upload a zero slice).
    /// Reset/partial submission revoke this one-allocation initialization fact.
    allocation_contents_defined: bool,
}
#[derive(Clone)]
struct PendingParity {
    previous: Option<u32>,
    current: Option<Arc<str>>,
}
#[derive(Default)]
pub(crate) struct TemporalRenderGraphState {
    resources: HashMap<Arc<str>, Record>,
    parity: HashMap<Arc<str>, u32>,
    retired_tokens: Vec<u64>,
    frame_requested: HashSet<Arc<str>>,
    /// Only members with initialized contents are published by this frame.
    frame_exports: HashMap<Arc<str>, ResourceHandle>,
    pending_parity: HashMap<Arc<str>, PendingParity>,
}
impl TemporalRenderGraphState {
    /// Existing keys are shared; fresh names are ordinary owned allocations,
    /// never entries in a process-global interner.
    fn key_for(&self, name: &str) -> Arc<str> {
        self.resources
            .get_key_value(name)
            .map(|(key, _)| key.clone())
            .or_else(|| self.parity.get_key_value(name).map(|(key, _)| key.clone()))
            .unwrap_or_else(|| Arc::from(name))
    }
    pub(crate) fn reset(&mut self, name: &str) -> Result<(), TemporalError> {
        // Borrowed matching avoids even allocating a key for unknown resets.
        let matches = |key: &str| {
            key == name
                || key
                    .strip_prefix(name)
                    .is_some_and(|suffix| matches!(suffix, ":0" | ":1"))
        };
        if let Some(key) = self
            .frame_requested
            .iter()
            .find(|key| matches(key.as_ref()))
        {
            return Err(TemporalError::AlreadyRequested(Arc::clone(key)));
        }
        for (key, record) in &mut self.resources {
            if matches(key.as_ref()) {
                record.history_invalid = true;
                record.allocation_contents_defined = false;
            }
        }
        self.parity.remove(name);
        Ok(())
    }
    pub(crate) fn owns_token(&self, token: u64) -> bool {
        self.resources
            .values()
            .any(|record| record.resource.token == token)
    }
    pub(crate) fn requested_validity(&self) -> impl Iterator<Item = (u64, bool)> + '_ {
        self.frame_requested
            .iter()
            .filter_map(|key| self.resources.get(key))
            .map(|record| (record.resource.token, !record.history_invalid))
    }

    pub fn clear(&mut self) {
        // 显式清理接口；history reset 本身不会清除此缓存。
        self.retired_tokens
            .extend(self.resources.drain().map(|(_, r)| r.resource.token));
        self.parity.clear();
        self.frame_requested.clear();
        self.frame_exports.clear();
        self.pending_parity.clear();
    }
    pub fn release_retired(&mut self, owner: &mut impl TemporalResourceOwner) {
        for token in self.retired_tokens.drain(..) {
            owner.release_temporal(token);
        }
    }
    #[cfg(test)]
    pub fn retired_tokens(&self) -> &[u64] {
        &self.retired_tokens
    }
    #[cfg(test)]
    pub fn get(&self, name: &str) -> Option<TemporalResource> {
        self.resources.get(name).map(|r| r.resource)
    }
    #[cfg(test)]
    pub fn record_state(&self, name: &str) -> Option<TemporalRecordState> {
        self.resources.get(name).map(|r| r.state)
    }
    #[cfg(test)]
    pub fn parity(&self, name: &str) -> Option<u32> {
        self.parity.get(name).copied()
    }
    /// 帧边界：上一帧请求（Raw）却没有导出就放弃构图的记录恢复为已导出，
    /// 物理资源与访问不变（图从未执行，GPU 状态未变）。否则同名请求会永远 `AlreadyRequested`。
    pub fn begin_frame(&mut self) {
        self.abandon_frame();
    }
    /// 本帧真实发布的导出已随提交进入 GPU；仅这些成员具有可复用历史。
    pub fn confirm_exports(&mut self) {
        // The destination member, not unrelated submitted work, publishes parity.
        // Owned keys were prepared before submission; publication allocates no memory.
        for (key, pending) in self.pending_parity.drain() {
            if !pending
                .current
                .is_some_and(|current| self.frame_exports.contains_key(&current))
            {
                if let Some(previous) = pending.previous {
                    if let Some(parity) = self.parity.get_mut(&key) {
                        *parity = previous;
                    }
                } else {
                    self.parity.remove(&key);
                }
            }
        }
        for key in self.frame_requested.drain() {
            let Some(record) = self.resources.get_mut(&key) else {
                continue;
            };
            if self.frame_exports.remove(&key).is_some() {
                record.confirmed_access = record.resource.access;
                record.history_invalid = false;
            } else {
                record.resource.access = record.confirmed_access;
            }
            record.state = TemporalRecordState::Exported;
        }
        self.frame_exports.clear();
        self.pending_parity.clear();
    }
    /// 本帧未执行（构图放弃、编译/校验/录制失败或空图）：Raw 记录恢复为已导出，所有访问回退到上次确认值。
    pub fn abandon_frame(&mut self) {
        for record in self.resources.values_mut() {
            record.state = TemporalRecordState::Exported;
            record.resource.access = record.confirmed_access;
        }
        self.frame_requested.clear();
        self.frame_exports.clear();
        self.rollback_parity();
    }
    /// Some batches entered the GPU, so the previous contents of every history
    /// requested by this frame are conservatively invalid. The allocation stays
    /// bound to its token; the backend's submitted state repairs its next import
    /// layout. A later successful export makes the new contents reusable.
    pub fn invalidate_submitted_frame(&mut self) {
        for key in &self.frame_requested {
            if let Some(record) = self.resources.get_mut(key) {
                record.history_invalid = true;
                record.allocation_contents_defined = false;
            }
        }
        self.abandon_frame();
    }
    fn rollback_parity(&mut self) {
        for (key, pending) in self.pending_parity.drain() {
            if let Some(previous) = pending.previous {
                if let Some(parity) = self.parity.get_mut(&key) {
                    *parity = previous;
                }
            } else {
                self.parity.remove(&key);
            }
        }
    }
    /// 本帧已请求、尚未导出的记录（键与物理资源），供宿主把物理资源导入后端。
    pub fn requested(&self) -> Vec<(Arc<str>, TemporalResource)> {
        self.resources
            .iter()
            .filter(|(_, record)| !matches!(record.state, TemporalRecordState::Exported))
            .map(|(key, record)| (key.clone(), record.resource))
            .collect()
    }
    pub fn get_and_flip_parity(&mut self, name: impl AsRef<str>) -> u32 {
        let key = self.key_for(name.as_ref());
        self.pending_parity
            .entry(key.clone())
            .or_insert_with(|| PendingParity {
                previous: self.parity.get(&key).copied(),
                current: None,
            });
        let next = self
            .parity
            .get(&key)
            .map_or(0, |old| 1u32.wrapping_sub(*old));
        self.parity.insert(key, next);
        next
    }
    pub fn get_or_create_temporal(
        &mut self,
        graph: &mut RenderGraph,
        name: &str,
        descriptor: Descriptor,
        create: impl FnOnce() -> Result<u64, BuildError>,
    ) -> Result<TemporalLookup, TemporalError> {
        let key = self.key_for(name);
        if self.frame_requested.contains(&key) {
            return Err(TemporalError::AlreadyRequested(key));
        }
        if let Some(old) = self.resources.get(&key).copied() {
            if old.state != TemporalRecordState::Exported {
                return Err(TemporalError::AlreadyRequested(key));
            }
            let replace = match (old.resource.descriptor, descriptor) {
                (Descriptor::Buffer(_), Descriptor::Buffer(_)) => false,
                (Descriptor::Image(a), Descriptor::Image(b)) => {
                    if a.words_at_00_through_18[3..6] != b.words_at_00_through_18[3..6] {
                        true
                    } else {
                        if a != b {
                            return Err(TemporalError::DescriptorMismatch(key));
                        }
                        false
                    }
                }
                _ => return Err(TemporalError::WrongResourceKind(key)),
            };
            if !replace {
                // buffer 命中保留物理描述符，不能把新请求描述符冒充已有资源。
                // 复用与新建都以非拥有方式导入：
                // 物理资源的所有权留在时态表里，图只借用。
                let handle = graph
                    .import_unowned_with_sharing(
                        name,
                        old.resource.descriptor,
                        old.resource.token,
                        old.resource.access,
                        old.sharing,
                    )
                    .map_err(TemporalError::Backend)?;
                self.resources.get_mut(&key).unwrap().state =
                    raw_state(old.resource.descriptor, handle);
                self.frame_requested.insert(key);
                return Ok(TemporalLookup {
                    handle,
                    resource: old.resource,
                    reused: !old.history_invalid,
                });
            }
        }
        let resource = TemporalResource {
            descriptor,
            token: create().map_err(TemporalError::Backend)?,
            access: 0,
        };
        let handle = match graph.import_unowned(name, descriptor, resource.token, 0) {
            Ok(handle) => handle,
            Err(error) => {
                // 图容量错误时须释放新创建的拥有者引用。
                self.retired_tokens.push(resource.token);
                return Err(TemporalError::Backend(error));
            }
        };
        if let Some(old) = self.resources.insert(
            key.clone(),
            Record {
                sharing: crate::resource::ResourceSharing::Exclusive,
                resource,
                state: raw_state(descriptor, handle),
                confirmed_access: 0,
                history_invalid: true,
                allocation_contents_defined: false,
            },
        ) {
            self.retired_tokens.push(old.resource.token);
        }
        self.frame_requested.insert(key);
        Ok(TemporalLookup {
            handle,
            resource,
            reused: false,
        })
    }
    pub fn get_or_create_owned(
        &mut self,
        graph: &mut RenderGraph,
        name: &str,
        descriptor: Descriptor,
        owner: &mut impl TemporalResourceOwner,
    ) -> Result<TemporalLookup, TemporalError> {
        self.get_or_create_owned_inner(graph, name, descriptor, owner, true)
    }
    fn get_or_create_owned_inner(
        &mut self,
        graph: &mut RenderGraph,
        name: &str,
        descriptor: Descriptor,
        owner: &mut impl TemporalResourceOwner,
        release_retired: bool,
    ) -> Result<TemporalLookup, TemporalError> {
        let key = self.key_for(name);
        if self.resources.get(&key).is_some_and(|record| {
            std::mem::discriminant(&record.resource.descriptor)
                != std::mem::discriminant(&descriptor)
        }) {
            return Err(TemporalError::WrongResourceKind(key));
        }
        let sharing = owner.temporal_sharing(name);
        let replaced = if let Some(record) = self.resources.get(&key)
            && (record.sharing != sharing || record.resource.descriptor != descriptor)
        {
            if self.frame_requested.contains(&key) || record.state != TemporalRecordState::Exported
            {
                return Err(TemporalError::AlreadyRequested(key));
            }
            self.resources.remove(&key)
        } else {
            None
        };
        let mut created = false;
        let result = self.get_or_create_temporal(graph, name, descriptor, || {
            let token = owner.create_temporal(name, descriptor)?;
            created = true;
            Ok(token)
        });
        if let Some(old) = replaced {
            if result.is_ok() {
                self.retired_tokens.push(old.resource.token);
            } else {
                self.resources.insert(key.clone(), old);
            }
        }
        if let Ok(lookup) = &result {
            if created {
                self.resources
                    .get_mut(&key)
                    .expect("created temporal record")
                    .allocation_contents_defined =
                    owner.initial_contents_defined(lookup.resource.token);
            }
            graph
                .set_resource_sharing(lookup.handle, sharing)
                .map_err(TemporalError::Backend)?;
            self.resources
                .get_mut(&key)
                .expect("created temporal record")
                .sharing = sharing;
        }
        if release_retired {
            self.release_retired(owner);
        }
        result
    }
    #[cfg(test)]
    pub fn get_or_create_temporal_pair(
        &mut self,
        graph: &mut RenderGraph,
        name: &str,
        descriptor: Descriptor,
        mut create: impl FnMut() -> Result<u64, BuildError>,
    ) -> Result<TemporalPair, TemporalError> {
        // 只支持纹理 pair；buffer pair 尚无使用者。
        if !matches!(descriptor, Descriptor::Image(_)) {
            return Err(TemporalError::WrongResourceKind(Arc::<str>::from(name)));
        }
        let first =
            self.get_or_create_temporal(graph, &format!("{name}:0"), descriptor, &mut create)?;
        let second =
            self.get_or_create_temporal(graph, &format!("{name}:1"), descriptor, create)?;
        // 第二个失败时第一个 mutation 保留，两者成功后才改变 parity。
        let parity = self.get_and_flip_parity(name);
        let current = self.key_for(&format!("{name}:{parity}"));
        if let Some(pending) = self.pending_parity.get_mut(name) {
            pending.current = Some(current);
        }
        let (first, second) = if parity == 0 {
            (first, second)
        } else {
            (second, first)
        };
        Ok(TemporalPair {
            first,
            second,
            parity,
        })
    }
    pub fn get_or_create_owned_pair(
        &mut self,
        graph: &mut RenderGraph,
        name: &str,
        descriptor: Descriptor,
        owner: &mut impl TemporalResourceOwner,
    ) -> Result<TemporalPair, TemporalError> {
        if !matches!(descriptor, Descriptor::Image(_)) {
            return Err(TemporalError::WrongResourceKind(Arc::<str>::from(name)));
        }
        let keys = [
            self.key_for(&format!("{name}:0")),
            self.key_for(&format!("{name}:1")),
        ];
        if let Some(key) = keys
            .iter()
            .find(|key| self.frame_requested.contains((*key).as_ref()))
        {
            return Err(TemporalError::AlreadyRequested(Arc::clone(key)));
        }
        let previous = keys.each_ref().map(|key| self.resources.get(key).copied());
        let resources_len = graph.declaration.resources.len();
        let retired_len = self.retired_tokens.len();
        let requested = (|| {
            let first =
                self.get_or_create_owned_inner(graph, keys[0].as_ref(), descriptor, owner, false)?;
            let second =
                self.get_or_create_owned_inner(graph, keys[1].as_ref(), descriptor, owner, false)?;
            Ok::<_, TemporalError>((first, second))
        })();
        let (first, second) = match requested {
            Ok(pair) => {
                self.release_retired(owner);
                pair
            }
            Err(error) => {
                let mut new_tokens = smallvec::SmallVec::<[u64; 4]>::new();
                let is_previous = |token| {
                    previous
                        .iter()
                        .flatten()
                        .any(|record| record.resource.token == token)
                };
                for (key, old) in keys.into_iter().zip(previous) {
                    if let Some(record) = self.resources.remove(&key)
                        && !is_previous(record.resource.token)
                    {
                        new_tokens.push(record.resource.token);
                    }
                    if let Some(old) = old {
                        self.resources.insert(key.clone(), old);
                    }
                    self.frame_requested.remove(&key);
                }
                for token in self.retired_tokens.drain(retired_len..) {
                    if !is_previous(token) && !new_tokens.contains(&token) {
                        new_tokens.push(token);
                    }
                }
                for token in new_tokens {
                    owner.release_temporal(token);
                }
                graph.declaration.resources.truncate(resources_len);
                graph.foreign_imports.retain(|index| (*index as usize) < resources_len);
                return Err(error);
            }
        };
        let parity = self.get_and_flip_parity(name);
        let current = self.key_for(&format!("{name}:{parity}"));
        if let Some(pending) = self.pending_parity.get_mut(name) {
            pending.current = Some(current);
        }
        let (first, second) = if parity == 0 {
            (first, second)
        } else {
            (second, first)
        };
        Ok(TemporalPair {
            first,
            second,
            parity,
        })
    }
    pub fn export_temporal(&mut self, graph: &mut RenderGraph) -> Result<(), TemporalError> {
        // Physical allocation and valid history are different contracts. In
        // particular, plain allocations do not establish initialized contents.
        // Native temporal buffers instead upload a complete zero slice during
        // creation; preserve that actual producer independently of history.
        // Replay other invalid members as logical Create resources while
        // preserving their physical imports.
        // Only resource origins differ in the replayed declaration, so rewrite
        // them in place and restore them afterwards instead of deep-cloning every
        // pass (and its access list) of the graph on each frame.
        let mut overridden: smallvec::SmallVec<[(usize, super::graph::OriginSource); 8]> =
            smallvec::SmallVec::new();
        let mut checked = Ok(());
        for key in &self.frame_requested {
            let record = &self.resources[key];
            let handle = match record.state {
                TemporalRecordState::RawTexture(handle)
                | TemporalRecordState::RawBuffer(handle) => handle,
                TemporalRecordState::Exported => self.frame_exports[key],
            };
            if let Err(error) = graph.check_handle(handle) {
                checked = Err(TemporalError::Backend(error));
                break;
            }
            if record.history_invalid && !record.allocation_contents_defined {
                let index = handle.raw() as usize;
                let original = std::mem::replace(
                    &mut graph.declaration.resources[index].source,
                    super::graph::OriginSource::Create,
                );
                overridden.push((index, original));
            }
        }
        let initialized = checked.and_then(|()| {
            super::resource_contracts::initialized_resources(
                &graph.declaration,
                crate::raw::access_table::is_write,
            )
            .map_err(TemporalError::Backend)
        });
        // Restore in reverse so a resource named twice ends with its true origin.
        for (index, original) in overridden.into_iter().rev() {
            graph.declaration.resources[index].source = original;
        }
        let initialized = initialized?;
        for (key, record) in &mut self.resources {
            let handle = match record.state {
                TemporalRecordState::RawTexture(h) | TemporalRecordState::RawBuffer(h) => h,
                TemporalRecordState::Exported => continue,
            };
            if record.history_invalid && !initialized[handle.raw() as usize] {
                continue;
            }
            graph
                .export(handle, TEMPORAL_EXPORT_ACCESS)
                .map_err(TemporalError::Backend)?;
            record.resource.access = TEMPORAL_EXPORT_ACCESS;
            record.state = TemporalRecordState::Exported;
            self.frame_exports.insert(key.clone(), handle);
        }
        Ok(())
    }
    /// 校验真实编译输出的导出物理映射，防止只更新名称表而未接消费者。
    pub fn validate_compiled_exports(&self, compiled: &FrameInstance) -> Result<(), TemporalError> {
        for (key, handle) in &self.frame_exports {
            let record = &self.resources[key];
            let id = handle.raw() as usize;
            if record.state != TemporalRecordState::Exported
                || id >= compiled.declarations.resources.len()
                || !compiled.snapshot.exports.contains(&(id as u16))
            {
                return Err(TemporalError::ExportMissing(key.clone()));
            }
            if compiled
                .snapshot
                .physical_mapping
                .get(id)
                .and_then(|mapping| mapping.token)
                != Some(record.resource.token)
            {
                return Err(TemporalError::ExportTokenMismatch(key.clone()));
            }
        }
        Ok(())
    }
}
fn raw_state(descriptor: Descriptor, handle: ResourceHandle) -> TemporalRecordState {
    match descriptor {
        Descriptor::Image(_) => TemporalRecordState::RawTexture(handle),
        Descriptor::Buffer(_) => TemporalRecordState::RawBuffer(handle),
    }
}

impl std::fmt::Display for TemporalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for TemporalError {}

#[cfg(test)]
mod recorder_tests {
    use super::*;

    fn image() -> Descriptor {
        use crate::raw::texture::{TextureDesc, TextureKind};
        super::super::vulkan::ResourceSpec::Image(
            TextureDesc::new(
                ash::vk::Extent3D {
                    width: 8,
                    height: 8,
                    depth: 1,
                },
                TextureKind::D2,
                ash::vk::Format::R8G8B8A8_UNORM,
            )
            .usage(ash::vk::ImageUsageFlags::TRANSFER_DST),
        )
        .descriptor()
    }

    #[test]
    fn failed_dynamic_names_and_unknown_resets_leave_no_owned_keys() {
        let mut state = TemporalRenderGraphState::default();
        for index in 0..1025 {
            let name = format!("failed dynamic history {index}");
            let mut graph = RenderGraph::new();
            let error = state
                .get_or_create_temporal_pair(&mut graph, &name, buffer(), || unreachable!())
                .unwrap_err();
            let TemporalError::WrongResourceKind(key) = error else {
                panic!("expected a typed name error");
            };
            let weak = Arc::downgrade(&key);
            assert_eq!(key.as_ref(), name);
            drop(key);
            assert!(
                weak.upgrade().is_none(),
                "failed names must have no global owner"
            );
            let error = state
                .get_or_create_temporal(&mut graph, &name, image(), || {
                    Err(crate::Error::OutOfHostMemory.into())
                })
                .unwrap_err();
            assert!(matches!(error, TemporalError::Backend(_)));
            state.reset(&name).unwrap();
            assert!(graph.declaration.resources.is_empty());
        }
        assert!(state.resources.is_empty());
        assert!(state.parity.is_empty());
        assert!(state.pending_parity.is_empty());
        assert!(state.frame_requested.is_empty());
        assert!(state.frame_exports.is_empty());
        assert!(state.retired_tokens.is_empty());
        assert_eq!(state.resources.capacity(), 0);
        assert_eq!(state.parity.capacity(), 0);
        assert_eq!(state.pending_parity.capacity(), 0);
        assert_eq!(state.frame_requested.capacity(), 0);
        assert_eq!(state.frame_exports.capacity(), 0);
    }

    #[test]
    fn names_are_shared_by_requests_and_released_with_session_and_errors() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        state
            .get_or_create_temporal(&mut graph, "owned history", image(), || Ok(42))
            .unwrap();
        let key = state.resources.get_key_value("owned history").unwrap().0;
        let weak = Arc::downgrade(key);
        assert!(Arc::ptr_eq(
            key,
            state.frame_requested.get("owned history").unwrap()
        ));
        let error = state
            .get_or_create_temporal(&mut graph, "owned history", image(), || unreachable!())
            .unwrap_err();
        let TemporalError::AlreadyRequested(key) = &error else {
            panic!("expected a duplicate request error");
        };
        assert!(Arc::ptr_eq(
            key,
            state.resources.get_key_value("owned history").unwrap().0
        ));
        state.clear();
        drop(state);
        assert!(
            weak.upgrade().is_some(),
            "the returned error still owns its name"
        );
        drop(error);
        assert!(
            weak.upgrade().is_none(),
            "no process-global owner may retain the name"
        );
    }

    fn initialize(graph: &mut RenderGraph, lookup: TemporalLookup) {
        let mut pass = graph.add_pass("initialize history");
        pass.add_resource_access(lookup.handle, 0x34, 0).unwrap();
        pass.render(|_| true);
    }

    #[test]
    fn fixed_descriptor_change_returns_error_without_advancing_history() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let first = state
            .get_or_create_temporal(&mut graph, "fixed", image(), || Ok(91))
            .unwrap();
        initialize(&mut graph, first);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let Descriptor::Image(mut changed) = image() else {
            unreachable!()
        };
        changed.words_at_00_through_18[2] += 1;
        assert!(matches!(
            state.get_or_create_temporal(
                &mut graph,
                "fixed",
                Descriptor::Image(changed),
                || unreachable!()
            ),
            Err(TemporalError::DescriptorMismatch(_))
        ));
        assert!(graph.declaration.resources.is_empty());
        let again = state
            .get_or_create_temporal(&mut graph, "fixed", image(), || unreachable!())
            .unwrap();
        assert_eq!(again.resource.token, 91);
        assert!(again.reused);
    }
    #[test]
    fn owned_temporal_propagates_real_sharing_and_recreates_when_policy_changes() {
        use crate::resource::{QueueFamilies, ResourceSharing};
        struct Owner {
            next: u64,
            sharing: ResourceSharing,
            released: Vec<u64>,
            fail: bool,
        }
        impl TemporalResourceOwner for Owner {
            fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
                if self.fail {
                    return Err(BuildError::from(crate::Error::OutOfDeviceMemory));
                }
                self.next += 1;
                Ok(self.next)
            }
            fn release_temporal(&mut self, token: u64) {
                self.released.push(token);
            }
            fn temporal_sharing(&self, _: &str) -> ResourceSharing {
                self.sharing
            }
        }
        let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1]).unwrap());
        let mut owner = Owner {
            next: 0,
            sharing,
            released: Vec::new(),
            fail: false,
        };
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let first = state
            .get_or_create_owned(&mut graph, "shared", image(), &mut owner)
            .unwrap();
        assert_eq!(graph.declaration.resources[0].sharing, sharing);
        initialize(&mut graph, first);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let reused = state
            .get_or_create_owned(&mut graph, "shared", image(), &mut owner)
            .unwrap();
        assert!(reused.reused);
        assert_eq!(graph.declaration.resources[0].sharing, sharing);
        state.abandon_frame();
        owner.sharing = ResourceSharing::Exclusive;
        owner.fail = true;
        let mut failed_graph = RenderGraph::new();
        let error = state
            .get_or_create_owned(&mut failed_graph, "shared", image(), &mut owner)
            .unwrap_err();
        assert!(matches!(
            crate::Error::from(error),
            crate::Error::OutOfDeviceMemory
        ));
        assert_eq!(state.get("shared").unwrap().token, first.resource.token);
        assert!(owner.released.is_empty());
        owner.fail = false;
        let mut graph = RenderGraph::new();
        let replacement = state
            .get_or_create_owned(&mut graph, "shared", image(), &mut owner)
            .unwrap();
        assert!(!replacement.reused);
        assert_ne!(replacement.resource.token, first.resource.token);
        assert_eq!(owner.released, vec![first.resource.token]);
        assert_eq!(
            graph.declaration.resources[0].sharing,
            ResourceSharing::Exclusive
        );
    }

    #[test]
    fn partial_submission_invalidates_only_requested_history_until_a_successful_export() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        for (name, token) in [("written", 5), ("untouched", 6)] {
            let lookup = state
                .get_or_create_temporal(&mut graph, name, image(), || Ok(token))
                .unwrap();
            initialize(&mut graph, lookup);
        }
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();

        state.begin_frame();
        let mut graph = RenderGraph::new();
        state
            .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
            .unwrap();
        state.export_temporal(&mut graph).unwrap();
        state.invalidate_submitted_frame();
        assert!(state.retired_tokens().is_empty());

        // The next frame must initialize the damaged contents, while the actual
        // allocation and backend recovery state stay associated with token 5.
        state.begin_frame();
        let mut graph = RenderGraph::new();
        let damaged = state
            .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
            .unwrap();
        assert!(!damaged.reused);
        assert_eq!(damaged.resource.token, 5);
        assert_eq!(damaged.resource.access, TEMPORAL_EXPORT_ACCESS);
        let untouched = state
            .get_or_create_temporal(&mut graph, "untouched", image(), || panic!("retain token"))
            .unwrap();
        assert!(untouched.reused);
        state.abandon_frame();

        // A further unsubmitted failure cannot accidentally validate contents.
        let mut graph = RenderGraph::new();
        let damaged = state
            .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
            .unwrap();
        assert!(!damaged.reused);
        initialize(&mut graph, damaged);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        assert!(
            state
                .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
                .unwrap()
                .reused
        );
    }

    #[test]
    fn unsubmitted_history_keeps_its_allocation_but_needs_initialization() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let created = state
            .get_or_create_temporal(&mut graph, "history", image(), || Ok(5))
            .unwrap();
        assert!(!created.reused);
        state.export_temporal(&mut graph).unwrap();
        // Starting another frame also cancels an exported graph that was never
        // submitted; its speculative export layout must not become an import.
        state.begin_frame();
        let mut graph = RenderGraph::new();
        let retry = state
            .get_or_create_temporal(&mut graph, "history", image(), || {
                panic!("retain allocation")
            })
            .unwrap();
        assert_eq!(retry.resource.token, created.resource.token);
        assert_eq!(retry.resource.access, 0);
        assert!(!retry.reused);
        initialize(&mut graph, retry);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        state.begin_frame();
        let mut graph = RenderGraph::new();
        let confirmed = state
            .get_or_create_temporal(&mut graph, "history", image(), || {
                panic!("retain allocation")
            })
            .unwrap();
        assert!(confirmed.reused);
        assert_eq!(confirmed.resource.token, created.resource.token);
        assert_eq!(confirmed.resource.access, TEMPORAL_EXPORT_ACCESS);
    }

    #[test]
    fn pair_parity_commits_only_with_exports_and_rolls_back_failed_frames() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let mut token = 0;
        let first = state
            .get_or_create_temporal_pair(&mut graph, "history", image(), || {
                token += 1;
                Ok(token)
            })
            .unwrap();
        assert_eq!(first.parity, 0);
        state.export_temporal(&mut graph).unwrap();
        state.abandon_frame();
        assert_eq!(state.parity("history"), None);

        let mut graph = RenderGraph::new();
        let committed = state
            .get_or_create_temporal_pair(&mut graph, "history", image(), || panic!("retain pair"))
            .unwrap();
        assert_eq!(committed.parity, 0);
        initialize(&mut graph, committed.first);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        assert_eq!(state.parity("history"), Some(0));

        let mut graph = RenderGraph::new();
        let failed = state
            .get_or_create_temporal_pair(&mut graph, "history", image(), || panic!("retain pair"))
            .unwrap();
        assert_eq!(failed.parity, 1);
        state.export_temporal(&mut graph).unwrap();
        state.invalidate_submitted_frame();
        assert_eq!(state.parity("history"), Some(0));

        let mut graph = RenderGraph::new();
        let recovered = state
            .get_or_create_temporal_pair(&mut graph, "history", image(), || panic!("retain pair"))
            .unwrap();
        assert_eq!(recovered.parity, 1);
        assert!(!(recovered.first.reused && recovered.second.reused));
        initialize(&mut graph, recovered.first);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        assert_eq!(state.parity("history"), Some(1));
    }

    #[test]
    fn starting_a_new_frame_cancels_uncommitted_parity() {
        let mut state = TemporalRenderGraphState::default();
        assert_eq!(state.get_and_flip_parity("pair"), 0);
        state.begin_frame();
        assert_eq!(state.parity("pair"), None);
        assert_eq!(state.get_and_flip_parity("pair"), 0);
        state.confirm_exports();
        assert_eq!(
            state.parity("pair"),
            None,
            "unpublished members cannot commit parity"
        );
        assert_eq!(state.get_and_flip_parity("pair"), 0);
        state.begin_frame();
        assert_eq!(state.parity("pair"), None);
        state.get_and_flip_parity("pair");
        state.clear();
        state.abandon_frame();
        assert_eq!(state.parity("pair"), None);
    }

    #[test]
    fn fresh_pair_publishes_only_initialized_members_and_validates_their_actual_mapping() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let mut token = 10;
        let pair = state
            .get_or_create_temporal_pair(&mut graph, "pair", image(), || {
                token += 1;
                Ok(token)
            })
            .unwrap();
        initialize(&mut graph, pair.first);
        state.export_temporal(&mut graph).unwrap();
        assert_eq!(graph.declaration.exports.len(), 1);
        assert_eq!(
            graph.declaration.exports[0].resource_index,
            pair.first.handle.raw()
        );
        assert_eq!(state.get("pair:1").unwrap().access, 0);
        assert!(matches!(
            state.get_or_create_temporal(&mut graph, "pair:0", image(), || unreachable!()),
            Err(TemporalError::AlreadyRequested(_))
        ));
        let mut compiled = super::super::graph::compile_graph(
            graph.build().declaration,
            &mut Default::default(),
            &mut super::super::graph::EventBackend::default(),
            &mut super::super::vulkan::AccessTable::original(),
        )
        .unwrap();
        state.validate_compiled_exports(&compiled).unwrap();
        let index = pair.first.handle.raw() as usize;
        compiled.tables.snapshot.physical_mapping[index].token = Some(1000);
        assert!(matches!(
            state.validate_compiled_exports(&compiled),
            Err(TemporalError::ExportTokenMismatch(_))
        ));
        compiled.tables.snapshot.physical_mapping[index].token = Some(pair.first.resource.token);
        compiled.tables.snapshot.exports.clear();
        assert!(matches!(
            state.validate_compiled_exports(&compiled),
            Err(TemporalError::ExportMissing(_))
        ));
        state.confirm_exports();

        let mut graph = RenderGraph::new();
        let pair = state
            .get_or_create_temporal_pair(&mut graph, "pair", image(), || unreachable!())
            .unwrap();
        assert_eq!([pair.first.reused, pair.second.reused], [false, true]);
        initialize(&mut graph, pair.first);
        state.export_temporal(&mut graph).unwrap();
        assert_eq!(graph.declaration.exports.len(), 2);
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let reused = state
            .get_or_create_temporal_pair(&mut graph, "pair", image(), || unreachable!())
            .unwrap();
        assert!(reused.first.reused && reused.second.reused);
    }

    fn buffer() -> Descriptor {
        Descriptor::Buffer(super::super::graph::BufferDesc {
            word_at_00: 64,
            word_at_08: 0,
            bytes_at_0c_through_0e: [0; 3],
        })
    }
    fn write_half(graph: &mut RenderGraph, lookup: TemporalLookup, offset: u64) {
        let mut pass = graph.add_pass("write half");
        pass.add_resource_access_range(
            lookup.handle,
            0x34,
            0,
            super::super::resource::AccessRange::Buffer { offset, size: 32 },
        )
        .unwrap();
        pass.render(|_| true);
    }

    #[test]
    fn unused_or_partially_initialized_history_is_never_confirmed_and_first_reads_are_rejected() {
        for descriptor in [image(), buffer()] {
            let mut state = TemporalRenderGraphState::default();
            let mut graph = RenderGraph::new();
            state
                .get_or_create_temporal(&mut graph, "unused", descriptor, || Ok(5))
                .unwrap();
            state.export_temporal(&mut graph).unwrap();
            assert!(graph.declaration.exports.is_empty());
            state.confirm_exports();
            let mut graph = RenderGraph::new();
            let history = state
                .get_or_create_temporal(&mut graph, "unused", descriptor, || unreachable!())
                .unwrap();
            assert!(!history.reused);
            assert_eq!(history.resource.access, 0);
            let mut reader = graph.add_pass("illegal first read");
            reader.add_resource_access(history.handle, 0x25, 0).unwrap();
            reader.render(|_| true);
            assert!(matches!(
                state.export_temporal(&mut graph),
                Err(TemporalError::Backend(BuildError::Contract(
                    super::super::resource::ResourceContractError::UninitializedRead { .. }
                )))
            ));
        }

        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "partial", buffer(), || Ok(6))
            .unwrap();
        write_half(&mut graph, history, 0);
        state.export_temporal(&mut graph).unwrap();
        assert!(graph.declaration.exports.is_empty());
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
            .unwrap();
        assert!(!history.reused);
        write_half(&mut graph, history, 0);
        write_half(&mut graph, history, 32);
        state.export_temporal(&mut graph).unwrap();
        assert_eq!(graph.declaration.exports.len(), 1);
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
            .unwrap();
        assert!(history.reused);
        state.export_temporal(&mut graph).unwrap();
        state.invalidate_submitted_frame();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
            .unwrap();
        assert!(!history.reused);
        write_half(&mut graph, history, 0);
        state.export_temporal(&mut graph).unwrap();
        assert!(
            graph.declaration.exports.is_empty(),
            "half a failed history cannot publish a whole valid history"
        );
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        assert!(
            !state
                .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
                .unwrap()
                .reused
        );
    }
    #[test]
    fn uploaded_initial_contents_are_readable_without_publishing_history_and_survive_cancel() {
        struct UploadedOwner {
            next: u64,
        }
        impl TemporalResourceOwner for UploadedOwner {
            fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
                self.next += 1;
                Ok(self.next)
            }
            fn release_temporal(&mut self, _: u64) {}
            fn initial_contents_defined(&self, _: u64) -> bool {
                true
            }
        }
        fn reader(graph: &mut RenderGraph, lookup: TemporalLookup) {
            let mut pass = graph.add_pass("read actually uploaded bytes");
            pass.add_resource_access(lookup.handle, 0x25, 0).unwrap();
            pass.render(|_| true);
        }
        let mut owner = UploadedOwner { next: 0 };
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let first = state
            .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
            .unwrap();
        assert!(!first.reused, "an upload is not preceding frame history");
        reader(&mut graph, first);
        state.export_temporal(&mut graph).unwrap();
        assert_eq!(graph.declaration.exports.len(), 1);
        state.abandon_frame();
        let mut graph = RenderGraph::new();
        let cancelled = state
            .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
            .unwrap();
        assert!(!cancelled.reused);
        assert_eq!(cancelled.resource.token, first.resource.token);
        reader(&mut graph, cancelled);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let confirmed = state
            .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
            .unwrap();
        assert!(confirmed.reused);
        state.invalidate_submitted_frame();
        let mut graph = RenderGraph::new();
        let failed = state
            .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
            .unwrap();
        assert!(!failed.reused);
        reader(&mut graph, failed);
        assert!(matches!(
            state.export_temporal(&mut graph),
            Err(TemporalError::Backend(BuildError::Contract(
                super::super::resource::ResourceContractError::UninitializedRead { .. }
            )))
        ));
        state.abandon_frame();
        state.reset("uploaded").unwrap();
        let mut graph = RenderGraph::new();
        let reset = state
            .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
            .unwrap();
        reader(&mut graph, reset);
        assert!(state.export_temporal(&mut graph).is_err());
        assert_eq!(
            owner.next, 1,
            "reset/cancel do not secretly allocate or reupload"
        );
    }

    // ---------------------------------------------------------------- 5. temporal 记录放弃 / 确认

    #[test]
    fn an_abandoned_temporal_request_is_released_at_the_next_frame_begin() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
            .unwrap();
        let mut initialize = graph.add_pass("initialize history");
        initialize
            .add_resource_access(history.handle, 0x34, 0)
            .unwrap();
        initialize.render(|_| true);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        // 下一帧请求后放弃构图（不导出、不编译）。
        let mut graph = RenderGraph::new();
        state
            .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
            .unwrap();
        drop(graph);
        // 同一帧内重复请求仍报错。
        let mut same = RenderGraph::new();
        assert!(matches!(
            state.get_or_create_temporal(&mut same, "hist", image(), || Ok(8)),
            Err(TemporalError::AlreadyRequested(_))
        ));
        state.begin_frame();
        let mut graph = RenderGraph::new();
        let again = state
            .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
            .unwrap();
        assert!(again.reused);
        assert_eq!(again.resource.token, 7);
    }

    #[test]
    fn an_unexecuted_export_does_not_promote_a_new_resource_beyond_undefined() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
            .unwrap();
        let mut initialize = graph.add_pass("initialize history");
        initialize
            .add_resource_access(history.handle, 0x34, 0)
            .unwrap();
        initialize.render(|_| true);
        state.export_temporal(&mut graph).unwrap();
        // export_temporal 写入时态导出访问码。
        assert_eq!(state.get("hist").unwrap().access, TEMPORAL_EXPORT_ACCESS);
        // 本帧没有真正执行：放弃后回到 GPU 已确认的访问（新资源 = undefined）。
        state.abandon_frame();
        assert_eq!(state.get("hist").unwrap().access, 0);
        assert_eq!(
            state.record_state("hist"),
            Some(TemporalRecordState::Exported)
        );
        // 执行确认后再放弃不回退。
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
            .unwrap();
        let mut initialize = graph.add_pass("initialize history");
        initialize
            .add_resource_access(history.handle, 0x34, 0)
            .unwrap();
        initialize.render(|_| true);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        state.abandon_frame();
        assert_eq!(state.get("hist").unwrap().access, TEMPORAL_EXPORT_ACCESS);
    }

    #[test]
    fn owned_descriptor_replacement_is_transactional_on_allocation_failure() {
        struct Owner {
            next: u64,
            fail: bool,
            released: Vec<u64>,
        }
        impl TemporalResourceOwner for Owner {
            fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
                if self.fail {
                    return Err(crate::Error::OutOfDeviceMemory.into());
                }
                self.next += 1;
                Ok(self.next)
            }
            fn release_temporal(&mut self, token: u64) {
                self.released.push(token);
            }
        }
        let mut owner = Owner {
            next: 20,
            fail: false,
            released: Vec::new(),
        };
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let original = state
            .get_or_create_owned(&mut graph, "slot", image(), &mut owner)
            .unwrap();
        initialize(&mut graph, original);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        let mut changed = match image() {
            Descriptor::Image(desc) => desc,
            _ => unreachable!(),
        };
        changed.words_at_00_through_18[3] += 1;
        owner.fail = true;
        let mut graph = RenderGraph::new();
        let failure = state
            .get_or_create_owned(&mut graph, "slot", Descriptor::Image(changed), &mut owner)
            .unwrap_err();
        assert_eq!(
            crate::Error::from(failure).kind(),
            crate::ErrorKind::OutOfMemory
        );
        assert!(graph.declaration.resources.is_empty());
        assert_eq!(state.get("slot").unwrap().token, original.resource.token);
        assert_eq!(
            state.get("slot").unwrap().descriptor,
            original.resource.descriptor
        );
        assert_eq!(state.get("slot").unwrap().access, TEMPORAL_EXPORT_ACCESS);
        assert!(owner.released.is_empty());
        owner.fail = false;
        let retained = state
            .get_or_create_owned(&mut graph, "slot", image(), &mut owner)
            .unwrap();
        assert!(retained.reused);
        assert_eq!(retained.resource.token, original.resource.token);
        state.abandon_frame();
        state.reset("slot").unwrap();
        let mut graph = RenderGraph::new();
        let reset = state
            .get_or_create_owned(&mut graph, "slot", image(), &mut owner)
            .unwrap();
        assert!(!reset.reused);
        assert_eq!(reset.resource.token, original.resource.token);
        assert!(state.reset("slot").is_err());
    }

    #[test]
    fn pair_second_allocation_failure_retains_both_confirmed_owners_and_parity() {
        struct Owner {
            next: u64,
            fail_at: u64,
            released: Vec<u64>,
        }
        impl TemporalResourceOwner for Owner {
            fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
                self.next += 1;
                if self.next == self.fail_at {
                    return Err(crate::Error::OutOfDeviceMemory.into());
                }
                Ok(self.next)
            }
            fn release_temporal(&mut self, token: u64) {
                self.released.push(token);
            }
        }
        let mut owner = Owner {
            next: 0,
            fail_at: 4,
            released: Vec::new(),
        };
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        let original = state
            .get_or_create_owned_pair(&mut graph, "pair", image(), &mut owner)
            .unwrap();
        initialize(&mut graph, original.first);
        initialize(&mut graph, original.second);
        state.export_temporal(&mut graph).unwrap();
        state.confirm_exports();
        let mut changed = match image() {
            Descriptor::Image(desc) => desc,
            _ => unreachable!(),
        };
        changed.words_at_00_through_18[3] += 1;
        let mut graph = RenderGraph::new();
        let failure = state
            .get_or_create_owned_pair(&mut graph, "pair", Descriptor::Image(changed), &mut owner)
            .unwrap_err();
        assert_eq!(
            crate::Error::from(failure).kind(),
            crate::ErrorKind::OutOfMemory
        );
        assert!(graph.declaration.resources.is_empty());
        assert_eq!(state.parity("pair"), Some(0));
        assert_eq!(owner.released, vec![3]);
        let retained = state
            .get_or_create_owned_pair(&mut graph, "pair", image(), &mut owner)
            .unwrap();
        assert!(retained.first.reused && retained.second.reused);
        assert_eq!(
            (
                retained.first.resource.token,
                retained.second.resource.token
            ),
            (
                original.second.resource.token,
                original.first.resource.token
            )
        );
    }
}
