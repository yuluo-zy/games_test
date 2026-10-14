//! 原跨帧合同：raw texture=0、raw buffer=1、exported=2；同帧只允许请求一次。
//! 描述符规则来自原 0x14096c310/0x14096c890，键复用原依赖 Ustr。
use super::{
    graph::{BuildError, CompiledGraph, Descriptor, RenderGraph},
    resource::ResourceHandle,
};
use std::collections::{HashMap, HashSet};
use ustr::Ustr;
pub const TEMPORAL_EXPORT_ACCESS: u8 = 0x37;

/// 跨帧双缓冲的奇偶标记；0/1 两态，`flip` 翻转。`get_and_flip_parity` 内联等价逻辑，
/// 本类型只是把该位域包成具名薄包装，供需要独立持有 parity 的调用者使用。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Parity(pub u32);
impl Parity {
    pub fn flip(self) -> Self {
        Self(1u32.wrapping_sub(self.0))
    }
    pub fn value(self) -> u32 {
        self.0
    }
}

/// 一对跨帧值按当前 parity 排序；`split_pair` 还原成 `(当前帧值, 上一帧值)`。
/// 原入口的 pair 交换发生在 `get_or_create_temporal_pair`；本类型把结果結构化，
/// 不改变已有调用点的奇偶选择顺序。
#[derive(Clone, Copy, Debug)]
pub struct Temporal<T> {
    current: T,
    previous: T,
    parity: Parity,
}
impl<T> Temporal<T> {
    pub fn new(current: T, previous: T, parity: Parity) -> Self {
        Self {
            current,
            previous,
            parity,
        }
    }
    pub fn into_inner(self) -> (T, T) {
        (self.current, self.previous)
    }
    pub fn as_ref(&self) -> Temporal<&T> {
        Temporal {
            current: &self.current,
            previous: &self.previous,
            parity: self.parity,
        }
    }
    pub fn split_pair(self) -> (T, T, Parity) {
        (self.current, self.previous, self.parity)
    }
    pub fn parity(&self) -> Parity {
        self.parity
    }
    /// parity==0 保持 (current, previous) 原序；parity==1 交换，对应
    /// `get_or_create_temporal_pair`/`_owned_pair` 里原有的手动 if/else 排序。
    pub fn into_pair_ordering(self) -> (T, T, Parity) {
        if self.parity.value() == 0 {
            (self.current, self.previous, self.parity)
        } else {
            (self.previous, self.current, self.parity)
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalResource {
    pub descriptor: Descriptor,
    pub token: u64,
    pub access: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalRecordState {
    RawTexture(ResourceHandle),
    RawBuffer(ResourceHandle),
    Exported,
}
impl TemporalRecordState {
    pub fn original_tag(self) -> u8 {
        match self {
            Self::RawTexture(_) => 0,
            Self::RawBuffer(_) => 1,
            Self::Exported => 2,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalLookup {
    pub handle: ResourceHandle,
    pub resource: TemporalResource,
    pub reused: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalPair {
    pub first: TemporalLookup,
    pub second: TemporalLookup,
    pub parity: u32,
    pub both_reused: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemporalError {
    AlreadyRequested(Ustr),
    WrongResourceKind(Ustr),
    Backend(BuildError),
    ExportMissing(Ustr),
    ExportTokenMismatch(Ustr),
}
/// 后端拥有真实资源引用；释放缓存引用不等于销毁 GPU 正在使用的对象。
pub trait TemporalResourceOwner {
    fn create_temporal(&mut self, name: &str, descriptor: Descriptor) -> Result<u64, BuildError>;
    fn release_temporal(&mut self, token: u64);
}
#[derive(Clone, Copy, Debug)]
struct Record {
    resource: TemporalResource,
    state: TemporalRecordState,
}
#[derive(Default)]
pub struct TemporalRenderGraphState {
    resources: HashMap<Ustr, Record>,
    parity: HashMap<Ustr, u32>,
    retired_tokens: Vec<u64>,
}
impl TemporalRenderGraphState {
    pub fn clear(&mut self) {
        // 显式候选清理接口；原游戏 history reset 本身不会清除此缓存。
        self.retired_tokens
            .extend(self.resources.drain().map(|(_, r)| r.resource.token));
        self.parity.clear();
    }
    pub fn clear_owned(&mut self, owner: &mut impl TemporalResourceOwner) {
        self.clear();
        self.release_retired(owner);
    }
    pub fn release_retired(&mut self, owner: &mut impl TemporalResourceOwner) {
        for token in self.retired_tokens.drain(..) {
            owner.release_temporal(token);
        }
    }
    /// token 后端只持一个缓存 Arc；被新状态继续使用的同一物理资源必须保留。
    pub(crate) fn release_superseded(
        &mut self,
        retained: &Self,
        owner: &mut impl TemporalResourceOwner,
    ) {
        let retained = retained
            .resources
            .values()
            .map(|r| r.resource.token)
            .collect::<HashSet<_>>();
        self.clear();
        for token in self.retired_tokens.drain(..).collect::<HashSet<_>>() {
            if !retained.contains(&token) {
                owner.release_temporal(token);
            }
        }
    }
    pub fn retired_tokens(&self) -> &[u64] {
        &self.retired_tokens
    }
    pub fn len(&self) -> usize {
        self.resources.len()
    }
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
    pub fn get(&self, name: &str) -> Option<TemporalResource> {
        self.resources.get(&Ustr::from(name)).map(|r| r.resource)
    }
    pub fn record_state(&self, name: &str) -> Option<TemporalRecordState> {
        self.resources.get(&Ustr::from(name)).map(|r| r.state)
    }
    pub fn parity(&self, name: &str) -> Option<u32> {
        self.parity.get(&Ustr::from(name)).copied()
    }
    pub fn get_and_flip_parity(&mut self, name: impl AsRef<str>) -> u32 {
        let key = Ustr::from(name.as_ref());
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
        let key = Ustr::from(name);
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
                        // 原固定字段差异路径保持 panic，不能静默重建。
                        assert_eq!(a, b, "跨帧纹理固定描述符不一致");
                        false
                    }
                }
                _ => return Err(TemporalError::WrongResourceKind(key)),
            };
            if !replace {
                // buffer 命中保留物理描述符，不能把新请求描述符冒充已有资源。
                let handle = graph
                    .import_owned(
                        name,
                        old.resource.descriptor,
                        old.resource.token,
                        old.resource.access,
                    )
                    .map_err(TemporalError::Backend)?;
                self.resources.get_mut(&key).unwrap().state =
                    raw_state(old.resource.descriptor, handle);
                return Ok(TemporalLookup {
                    handle,
                    resource: old.resource,
                    reused: true,
                });
            }
        }
        let resource = TemporalResource {
            descriptor,
            token: create().map_err(TemporalError::Backend)?,
            access: 0,
        };
        let handle = match graph.import_owned(name, descriptor, resource.token, 0) {
            Ok(handle) => handle,
            Err(error) => {
                // 原 import 是无失败分支的边界；候选图容量错误仍须释放新创建的拥有者引用。
                self.retired_tokens.push(resource.token);
                return Err(TemporalError::Backend(error));
            }
        };
        if let Some(old) = self.resources.insert(
            key,
            Record {
                resource,
                state: raw_state(descriptor, handle),
            },
        ) {
            self.retired_tokens.push(old.resource.token);
        }
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
        let result = self.get_or_create_temporal(graph, name, descriptor, || {
            owner.create_temporal(name, descriptor)
        });
        self.release_retired(owner);
        result
    }
    pub fn get_or_create_temporal_pair(
        &mut self,
        graph: &mut RenderGraph,
        name: &str,
        descriptor: Descriptor,
        mut create: impl FnMut() -> Result<u64, BuildError>,
    ) -> Result<TemporalPair, TemporalError> {
        // 原已知实例是纹理 pair；buffer pair 尚无原实例，不冒充已恢复功能。
        if !matches!(descriptor, Descriptor::Image(_)) {
            return Err(TemporalError::WrongResourceKind(Ustr::from(name)));
        }
        let first =
            self.get_or_create_temporal(graph, &format!("{name}:0"), descriptor, &mut create)?;
        let second =
            self.get_or_create_temporal(graph, &format!("{name}:1"), descriptor, create)?;
        // 第二个失败时第一个 mutation 保留，两者成功后才改变 parity。
        let parity = self.get_and_flip_parity(name);
        let both_reused = first.reused && second.reused;
        let (first, second, parity) =
            Temporal::new(first, second, Parity(parity)).into_pair_ordering();
        Ok(TemporalPair {
            first,
            second,
            parity: parity.value(),
            both_reused,
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
            return Err(TemporalError::WrongResourceKind(Ustr::from(name)));
        }
        let first = self.get_or_create_owned(graph, &format!("{name}:0"), descriptor, owner)?;
        let second = self.get_or_create_owned(graph, &format!("{name}:1"), descriptor, owner)?;
        let parity = self.get_and_flip_parity(name);
        let both_reused = first.reused && second.reused;
        let (first, second, parity) =
            Temporal::new(first, second, Parity(parity)).into_pair_ordering();
        Ok(TemporalPair {
            first,
            second,
            parity: parity.value(),
            both_reused,
        })
    }
    pub fn export_temporal(&mut self, graph: &mut RenderGraph) -> Result<(), TemporalError> {
        for record in self.resources.values_mut() {
            let handle = match record.state {
                TemporalRecordState::RawTexture(h) | TemporalRecordState::RawBuffer(h) => h,
                TemporalRecordState::Exported => continue,
            };
            graph
                .export(handle, TEMPORAL_EXPORT_ACCESS)
                .map_err(TemporalError::Backend)?;
            record.resource.access = TEMPORAL_EXPORT_ACCESS;
            record.state = TemporalRecordState::Exported;
        }
        Ok(())
    }
    /// 校验真实编译输出的导出物理映射，防止只更新名称表而未接消费者。
    pub fn validate_compiled_exports(&self, compiled: &CompiledGraph) -> Result<(), TemporalError> {
        for (&key, record) in &self.resources {
            if let Some((id, _)) = compiled
                .declarations
                .resources
                .iter()
                .enumerate()
                .find(|(_, r)| r.name == key.as_str())
            {
                if record.state != TemporalRecordState::Exported
                    || !compiled.snapshot.exports.contains(&(id as u16))
                {
                    return Err(TemporalError::ExportMissing(key));
                }
                if compiled.snapshot.physical_mapping[id].token != Some(record.resource.token) {
                    return Err(TemporalError::ExportTokenMismatch(key));
                }
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
