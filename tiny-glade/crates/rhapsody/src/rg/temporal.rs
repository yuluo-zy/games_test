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
/// 原 `TemporalRenderGraphRecorder`（0x110 字节）与表内 `Record`（0x68 字节）的已读偏移。
/// 证据：`export_temporal` 0x1404f7ca0（以**输入**recorder 为基准读 `+0xb0..+0x108`，输出是去掉设备引用的
/// `(RenderGraph, TemporalRenderGraphState)` 元组，所以输出里同样的表整体前移 8 字节：`+0xa8` 起）、
/// `get_or_create_temporal<&str>` 0x14096c310 / `<String>` 0x1404f6850 / `get_or_create_temporal_pair` 0x1404f7450、
/// `TemporalRenderGraphState::get_and_flip_parity` 0x140beced0。
///
/// * Recorder（输入基准）：`+0x00..0xa8` `RenderGraph`；**`+0xa8` = `&GraphicsDevice`**（`get_or_create_temporal`
///   把 `[recorder+0xa8]` 作为 `GraphicsDevice::create_buffer`（0x1416a2dc0）的第 2 参；`export_temporal` 不拷它）；
///   `+0xb0..+0x110` = 内嵌的 `TemporalRenderGraphState`（0x60 字节，两张 `HashMap<Ustr, _, RandomState>`）：
///   - 表 1（temporal 资源，元素 0x68）：`+0xb0` 控制字节指针、`+0xb8` bucket_mask、`+0xc0` growth_left、`+0xc8` items、
///     `+0xd0/+0xd8` 哈希种子 k0/k1（`hash_one` 0x1412e8120 的 `rcx = recorder+0xd0`）；
///   - 表 2（奇偶翻转 `Ustr → u32`，`get_and_flip_parity(recorder+0xb0, key)`，体内读 `state+0x48` items、`state+0x50` 种子）：
///     `+0xe0` 控制字节、`+0xe8` mask、`+0xf0` growth_left、`+0xf8` items、`+0x100/+0x108` 种子。
///   即旧文档"+0xd8..+0x108 含义未读出"已读出：就是第二张表（奇偶表）。现 Rust `TemporalRenderGraphState` 的
///   `resources` / `parity` 两张表与之对应，`retired_tokens`（候选）原版没有。
/// * Record（相对元素起点；元素末尾地址 `P = 起点+0x68`，代码里的 `[P-0x60]` 即 `+0x08`）：`+0x00` 键 `Ustr`、
///   `+0x08` 状态 u32（0 raw 纹理 / 1 raw 缓冲 / 2 已导出；`get_or_create_temporal` 命中时要求 `== 2` 否则报错，
///   然后改写为 1）、`+0x0c` 起为负载。状态 0/1 时负载是 `{u32 资源下标 @0x0c, u64 @0x10, u64 @0x18, u64 @0x20, u64 @0x28}`
///   （纹理 0x140bd6380 / 缓冲 0x140bd5120 的 `export` 入参，缓冲少一个字段）；导出后写回状态 2、
///   `+0x0c` 字节 = 0x37（导出访问码）、其后原样放 export 的返回载体。
///   命中路径：`+0x0c` 的字节作为 `import_unowned`（0x140bd4fc0）第 4 参（`initial_access`），`+0x30` 起为该调用的
///   第 2 参（导出载体），调用返回 20 字节 `ImportedBuffer` 写回 `+0x0c..+0x20`；**`+0x66` 字节必须 == 7**（缓冲变体的
///   后备资源类型标记，不等则报"wrong kind"）。
///   返回值（`Result` 的 sret，0x20 字节）：`{ImportedBuffer 0x14 @0, tag u8 @0x18}`，tag 0 = 新建、1 = 复用、2 = 错误（载荷为 String）。
///   `+0x24..+0x30`、`+0x67` 仍无读出的字段名。
pub mod original_layout {
    pub const RECORD_SIZE: usize = 0x68;
    pub const RECORD_KEY: usize = 0x00;
    pub const RECORD_STATE: usize = 0x08;
    pub const RECORD_PAYLOAD: usize = 0x0c;
    /// 命中路径里 `import_unowned` 的第 2 参（导出载体）所在偏移。
    pub const RECORD_EXPORT_CARRIER: usize = 0x30;
    /// 缓冲变体的后备资源类型标记所在字节，值必须为 [`RECORD_BUFFER_KIND`]。
    pub const RECORD_KIND_TAG: usize = 0x66;
    pub const RECORD_BUFFER_KIND: u8 = 7;
    pub const RECORDER_GRAPH_BYTES: usize = 0xa8;
    /// `&GraphicsDevice`。
    pub const RECORDER_DEVICE: usize = 0xa8;
    /// 内嵌 `TemporalRenderGraphState` 的起点与大小（Recorder 总长 = 起点 + 大小 = 0x110）。
    pub const RECORDER_STATE: usize = 0xb0;
    pub const RECORDER_STATE_BYTES: usize = 0x60;
    pub const RECORDER_SIZE: usize = 0x110;
    /// 表 1（temporal 资源）：控制字节指针 / mask / growth_left / items / 种子。
    pub const RECORDER_TABLE_CTRL: usize = 0xb0;
    pub const RECORDER_TABLE_LEN: usize = 0xc8;
    pub const RECORDER_TABLE_SEEDS: usize = 0xd0;
    /// 表 2（奇偶翻转）起点（相对 recorder）= `RECORDER_STATE + 0x30`。
    pub const RECORDER_PARITY_TABLE: usize = 0xe0;
    pub const RECORDER_PARITY_LEN: usize = 0xf8;
    pub const RECORDER_PARITY_SEEDS: usize = 0x100;
    pub const EXPORT_ACCESS_CODE: u8 = 0x37;
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
                // 原版复用与新建都走 `Texture::import_unowned`（0x1404f6e50）：
                // 物理资源的所有权留在时态表里，图只借用。
                let handle = graph
                    .import_unowned(
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
        let handle = match graph.import_unowned(name, descriptor, resource.token, 0) {
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

/// 原 `TemporalRenderGraphRecorder`：把跨帧状态与正在录制的图绑在一起；
/// 键是 `Ustr`（0x1404f6e50 用 `Ustr::from(ptr,len)`），`&str`/`String` 两种实例共用一份实现。
pub struct TemporalRenderGraphRecorder<'a> {
    pub state: &'a mut TemporalRenderGraphState,
    pub graph: &'a mut RenderGraph,
}
/// 原 trait `GetOrCreateTemporal(Pair)`：`get_or_create_temporal` 与 `_pair` 两个入口。
pub trait GetOrCreateTemporal {
    fn get_or_create_temporal(
        &mut self,
        name: &str,
        descriptor: Descriptor,
        create: impl FnOnce() -> Result<u64, BuildError>,
    ) -> Result<TemporalLookup, TemporalError>;
    fn get_or_create_temporal_pair(
        &mut self,
        name: &str,
        descriptor: Descriptor,
        create: impl FnMut() -> Result<u64, BuildError>,
    ) -> Result<TemporalPair, TemporalError>;
    fn export_temporal(&mut self) -> Result<(), TemporalError>;
}
impl GetOrCreateTemporal for TemporalRenderGraphRecorder<'_> {
    fn get_or_create_temporal(
        &mut self,
        name: &str,
        descriptor: Descriptor,
        create: impl FnOnce() -> Result<u64, BuildError>,
    ) -> Result<TemporalLookup, TemporalError> {
        self.state
            .get_or_create_temporal(self.graph, name, descriptor, create)
    }
    fn get_or_create_temporal_pair(
        &mut self,
        name: &str,
        descriptor: Descriptor,
        create: impl FnMut() -> Result<u64, BuildError>,
    ) -> Result<TemporalPair, TemporalError> {
        self.state
            .get_or_create_temporal_pair(self.graph, name, descriptor, create)
    }
    fn export_temporal(&mut self) -> Result<(), TemporalError> {
        self.state.export_temporal(self.graph)
    }
}

#[cfg(test)]
mod recorder_tests {
    use super::*;

    #[test]
    fn original_layout_constants_agree_with_the_implementation() {
        assert_eq!(original_layout::EXPORT_ACCESS_CODE, TEMPORAL_EXPORT_ACCESS);
        assert_eq!(TemporalRecordState::Exported.original_tag(), 2);
        assert!(original_layout::RECORD_PAYLOAD < original_layout::RECORD_SIZE);
        use original_layout as l;
        // 表 1 在 +0xb0..0xe0，表 2 在 +0xe0..0x110，各 RawTable(0x20)+RandomState(0x10)=0x30。
        assert_eq!(l::RECORDER_STATE, l::RECORDER_DEVICE + 8);
        assert_eq!(l::RECORDER_STATE + l::RECORDER_STATE_BYTES, l::RECORDER_SIZE);
        assert_eq!(l::RECORDER_PARITY_TABLE, l::RECORDER_STATE + 0x30);
        assert_eq!(l::RECORDER_TABLE_LEN, l::RECORDER_TABLE_CTRL + 0x18);
        assert_eq!(l::RECORDER_PARITY_LEN, l::RECORDER_PARITY_TABLE + 0x18);
        assert_eq!(l::RECORDER_TABLE_SEEDS, l::RECORDER_TABLE_CTRL + 0x20);
        assert_eq!(l::RECORDER_PARITY_SEEDS, l::RECORDER_PARITY_TABLE + 0x20);
        assert!(l::RECORD_KIND_TAG < l::RECORD_SIZE && l::RECORD_EXPORT_CARRIER < l::RECORD_KIND_TAG);
    }
    use crate::rg::graph::TextureDesc;

    fn image() -> Descriptor {
        Descriptor::Image(TextureDesc {
            words_at_00_through_18: [1, 2, 3, 4, 5, 6, 7],
            bytes_at_1c_through_1e: [0; 3],
        })
    }

    #[test]
    fn recorder_creates_then_reuses_after_export_through_the_trait() {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        {
            let mut rec = TemporalRenderGraphRecorder { state: &mut state, graph: &mut graph };
            let fresh = rec.get_or_create_temporal("hist", image(), || Ok(5)).unwrap();
            assert!(!fresh.reused);
            rec.export_temporal().unwrap();
        }
        let mut graph = RenderGraph::new();
        let mut rec = TemporalRenderGraphRecorder { state: &mut state, graph: &mut graph };
        let again = rec.get_or_create_temporal("hist", image(), || panic!("must reuse")).unwrap();
        assert!(again.reused);
        assert_eq!(again.resource.token, 5);
    }
}
