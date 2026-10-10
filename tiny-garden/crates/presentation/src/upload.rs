//! 共享的网格上传流程：批次来源由各类型实现，预算、就绪和暂存所有权统一处理。
//! 不接收领域对象或编辑票据；过期判定、实体安装、过渡和发布仍属于各类型。
//! 设计说明见 docs/上传与结构规则重构.md。
use bevy::pbr::SpecializedMaterialPipelineCache;
use bevy::render::{
    Render, RenderApp, RenderSystems, camera::ExtractedCamera, mesh::RenderMesh,
    render_asset::RenderAssets as GpuAssets, render_resource::PipelineCache,
    sync_world::MainEntity, view::RenderVisibleEntities,
};
use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use garden_generation::incremental::{PreparedBatch, PreparedBuilding};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{Arc, Mutex},
};

/// 主世界与渲染世界之间的就绪桥，不在 UI 线程阻塞或轮询 GPU。
#[derive(Resource, Clone, Default)]
pub(crate) struct MeshReadiness {
    pub(crate) enabled: bool,
    pub(crate) shared: Arc<Mutex<ReadinessState>>,
}
#[derive(Default)]
pub(crate) struct ReadinessState {
    pub(crate) waiting: BTreeSet<AssetId<Mesh>>,
    pub(crate) ready: BTreeSet<AssetId<Mesh>>,
    pub(crate) waiting_draws: BTreeSet<Entity>,
    pub(crate) ready_draws: BTreeSet<Entity>,
    pub(crate) visible_draws: BTreeSet<Entity>,
}
fn acknowledge_meshes(
    bridge: Res<MeshReadiness>,
    meshes: Res<GpuAssets<RenderMesh>>,
    pipelines: Res<SpecializedMaterialPipelineCache>,
    cache: Res<PipelineCache>,
    views: Query<&RenderVisibleEntities, With<ExtractedCamera>>,
) {
    let mut state = bridge.shared.lock().unwrap();
    let ready = state
        .waiting
        .iter()
        .copied()
        .filter(|id| meshes.get(*id).is_some())
        .collect::<Vec<_>>();
    for id in ready {
        state.waiting.remove(&id);
        state.ready.insert(id);
    }
    let ready = state
        .waiting_draws
        .iter()
        .copied()
        .filter(|entity| {
            pipelines
                .values()
                .filter_map(|view| view.get(&MainEntity::from(*entity)))
                .any(|id| cache.get_render_pipeline(*id).is_some())
        })
        .collect::<Vec<_>>();
    for entity in ready {
        state.waiting_draws.remove(&entity);
        state.ready_draws.insert(entity);
    }
    // 使用相机可见性；仅被阴影灯看到的对象不能等待尚未排队的材质管线。
    state.visible_draws = state
        .waiting_draws
        .iter()
        .chain(&state.ready_draws)
        .copied()
        .filter(|entity| {
            let main = MainEntity::from(*entity);
            views.iter().any(|view| {
                view.classes
                    .get(&std::any::TypeId::of::<Mesh3d>())
                    .is_some_and(|class| {
                        class
                            .entities_cpu_culling
                            .binary_search_by_key(&main, |(_, main)| *main)
                            .is_ok()
                            || class.entities_gpu_culling.contains_key(&main)
                    })
            })
        })
        .collect();
}
impl MeshReadiness {
    /// 一次加锁检查整个一致性组，避免按批次反复竞争互斥锁。
    pub(crate) fn meshes_ready<'a>(&self, handles: impl Iterator<Item = &'a Handle<Mesh>>) -> bool {
        if !self.enabled {
            return true;
        }
        let state = self.shared.lock().unwrap();
        handles.into_iter().all(|h| state.ready.contains(&h.id()))
    }
    /// 不可见对象无需等待相机未排队的管线；调用方仍负责自己的预热时序。
    pub(crate) fn draws_ready(&self, entities: impl Iterator<Item = Entity>) -> bool {
        if !self.enabled {
            return true;
        }
        let state = self.shared.lock().unwrap();
        entities
            .into_iter()
            .all(|e| !state.visible_draws.contains(&e) || state.ready_draws.contains(&e))
    }
    pub(crate) fn install(app: &mut App) {
        let bridge = Self {
            enabled: app.get_sub_app(RenderApp).is_some(),
            ..default()
        };
        app.insert_resource(bridge.clone());
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .insert_resource(bridge)
                .add_systems(Render, acknowledge_meshes.after(RenderSystems::Cleanup));
        }
    }

    pub(crate) fn track_draw(&self, entity: Entity) {
        if self.enabled {
            self.shared.lock().unwrap().waiting_draws.insert(entity);
        }
    }
    pub(crate) fn forget_draw(&self, entity: Entity) {
        let mut state = self.shared.lock().unwrap();
        state.waiting_draws.remove(&entity);
        state.ready_draws.remove(&entity);
        state.visible_draws.remove(&entity);
    }
    pub(crate) fn track(&self, handle: &Handle<Mesh>) {
        if self.enabled {
            self.shared.lock().unwrap().waiting.insert(handle.id());
        }
    }
    pub(crate) fn forget(&self, handle: &Handle<Mesh>) {
        let mut state = self.shared.lock().unwrap();
        state.waiting.remove(&handle.id());
        state.ready.remove(&handle.id());
    }
}

/// 每帧所有类型共用的上传预算。超大批次只允许作为该帧首批，以保证最终可推进。
#[derive(Resource, Clone, Copy)]
pub struct UploadBudget {
    pub bytes_per_frame: usize,
    pub batches_per_frame: usize,
}
impl Default for UploadBudget {
    fn default() -> Self {
        Self {
            bytes_per_frame: 512 * 1024,
            batches_per_frame: 8,
        }
    }
}
/// 只由公共上传入口计账，每帧开始统一重置，类型安装器不得覆盖彼此的使用量。
#[derive(Resource, Default)]
pub(crate) struct UploadLedger {
    pub bytes: usize,
    pub batches: usize,
}
impl UploadLedger {
    fn reserve(&mut self, bytes: usize, budget: UploadBudget) -> bool {
        if self.batches >= budget.batches_per_frame.max(1)
            || (self.batches > 0 && self.bytes.saturating_add(bytes) > budget.bytes_per_frame)
        {
            return false;
        }
        self.bytes = self.bytes.saturating_add(bytes);
        self.batches += 1;
        true
    }
}
pub(crate) fn reset_ledger(mut ledger: ResMut<UploadLedger>) {
    *ledger = UploadLedger::default();
}

/// 类型只提供稳定快照中的批次及本地索引，不重复实现预算或 GPU 就绪逻辑。
pub(crate) trait UploadSource {
    type Key: Copy + Ord;
    fn batches(&self) -> impl Iterator<Item = (Self::Key, &PreparedBatch)>;
    fn batch(&self, key: Self::Key) -> &PreparedBatch;
}
impl UploadSource for PreparedBuilding {
    type Key = usize;
    fn batches(&self) -> impl Iterator<Item = (usize, &PreparedBatch)> {
        self.batches.iter().enumerate()
    }
    fn batch(&self, key: usize) -> &PreparedBatch {
        &self.batches[key]
    }
}

/// 借用当帧公共资源，所有类型的上传和回收都经过同一入口。
pub(crate) struct UploadContext<'a> {
    pub meshes: &'a mut Assets<Mesh>,
    pub bridge: &'a MeshReadiness,
    pub budget: UploadBudget,
    pub ledger: &'a mut UploadLedger,
}
impl UploadContext<'_> {
    fn upload(&mut self, batch: &PreparedBatch) -> Option<Handle<Mesh>> {
        if !self.ledger.reserve(upload_bytes(batch), self.budget) {
            return None;
        }
        let handle = self.meshes.add(to_bevy_mesh(batch));
        self.bridge.track(&handle);
        Some(handle)
    }
}
/// 本次推进的用量增量；类型统计不用重新估算或重复判断预算。
#[derive(Default)]
pub(crate) struct UploadProgress {
    pub bytes: usize,
    pub batches: usize,
    pub oversized: usize,
}

/// 暂存期只拥有新上传句柄；复用句柄由已发布的类型驻留结构持有。
/// 取消暂存只回收 owned，不能误删仍可见的旧画面。
pub(crate) struct UploadStage<K> {
    pub pending: VecDeque<K>,
    pub handles: BTreeMap<K, Handle<Mesh>>,
    owned: Vec<Handle<Mesh>>,
    pub reused: usize,
}
impl<K: Copy + Ord> UploadStage<K> {
    pub fn new<S: UploadSource<Key = K>>(
        source: &S,
        mut reuse: impl FnMut(K, &PreparedBatch) -> Option<Handle<Mesh>>,
    ) -> Self {
        let mut stage = Self {
            pending: VecDeque::new(),
            handles: BTreeMap::new(),
            owned: Vec::new(),
            reused: 0,
        };
        for (key, batch) in source.batches() {
            if let Some(handle) = reuse(key, batch) {
                stage.handles.insert(key, handle);
                stage.reused += 1;
            } else {
                stage.pending.push_back(key);
            }
        }
        stage
    }
    pub fn advance<S: UploadSource<Key = K>>(
        &mut self,
        source: &S,
        context: &mut UploadContext,
    ) -> UploadProgress {
        let mut progress = UploadProgress::default();
        while let Some(&key) = self.pending.front() {
            let batch = source.batch(key);
            let Some(handle) = context.upload(batch) else {
                break;
            };
            let bytes = upload_bytes(batch);
            progress.bytes += bytes;
            progress.batches += 1;
            progress.oversized += usize::from(bytes > context.budget.bytes_per_frame);
            self.handles.insert(key, handle.clone());
            self.owned.push(handle);
            self.pending.pop_front();
        }
        progress
    }
    pub fn ready(&self, bridge: &MeshReadiness) -> bool {
        self.pending.is_empty() && bridge.meshes_ready(self.handles.values())
    }
    pub fn discard(self, meshes: &mut Assets<Mesh>, bridge: &MeshReadiness) {
        for handle in self.owned {
            release_mesh(meshes, bridge, &handle);
        }
    }
}

/// 位置、法线、UV、切线共 48 字节/顶点，U32 索引为 4 字节。
pub(crate) fn upload_bytes(batch: &PreparedBatch) -> usize {
    batch.data.positions.len() * 48 + batch.data.indices.len() * 4
}
pub(crate) fn release_mesh(
    meshes: &mut Assets<Mesh>,
    bridge: &MeshReadiness,
    handle: &Handle<Mesh>,
) {
    bridge.forget(handle);
    meshes.remove(handle.id());
}
/// 网格属性转换只在公共上传入口执行；类型安装器复用已上传的句柄。
pub fn to_bevy_mesh(batch: &PreparedBatch) -> Mesh {
    to_bevy_mesh_data(&batch.data, &batch.tangents)
}

/// 离线静态场景和编辑建筑共享属性转换；调用前必须完成网格/切线校验。
pub(crate) fn to_bevy_mesh_data(
    data: &garden_generation::mesh::MeshData,
    tangents: &[[f32; 4]],
) -> Mesh {
    debug_assert_eq!(tangents.len(), data.positions.len());
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents.to_vec())
    .with_inserted_indices(Indices::U32(data.indices.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cancelled_stage_keeps_reused_mesh_and_forgets_only_new_resources() {
        use garden_domain::{Building, BuildingId, sample_building};
        use garden_generation::{
            compile,
            incremental::Cancellation,
            mesh::{GeometryProfile, compile_mesh},
        };
        let b = Building::try_new(sample_building(BuildingId::new(1).unwrap(), 6., 3.)).unwrap();
        let mut source = PreparedBuilding::whole(
            compile_mesh(&compile(&b), GeometryProfile::default()).unwrap(),
        );
        garden_bevy::prepare::prepare(&mut source, &Cancellation::default()).unwrap();
        assert!(source.batches.len() > 1);
        let mut meshes = Assets::<Mesh>::default();
        let bridge = MeshReadiness {
            enabled: true,
            ..default()
        };
        let reused = meshes.add(to_bevy_mesh(&source.batches[0]));
        bridge.track(&reused);
        let mut stage = UploadStage::new(&source, |key, _| (key == 0).then(|| reused.clone()));
        let mut ledger = UploadLedger::default();
        let progress = stage.advance(
            &source,
            &mut UploadContext {
                meshes: &mut meshes,
                bridge: &bridge,
                ledger: &mut ledger,
                budget: UploadBudget {
                    bytes_per_frame: usize::MAX,
                    batches_per_frame: usize::MAX,
                },
            },
        );
        assert_eq!(stage.reused, 1);
        assert_eq!(progress.batches, source.batches.len() - 1);
        assert!(!stage.ready(&bridge), "CPU 安装不能冒充渲染就绪");
        let owned = stage.owned.clone();
        {
            let mut state = bridge.shared.lock().unwrap();
            state.ready.extend(stage.handles.values().map(Handle::id));
        }
        assert!(stage.ready(&bridge));
        stage.discard(&mut meshes, &bridge);
        assert!(meshes.get(&reused).is_some());
        assert!(owned.iter().all(|h| meshes.get(h).is_none()));
        let state = bridge.shared.lock().unwrap();
        assert!(state.waiting.contains(&reused.id()));
        assert!(
            owned
                .iter()
                .all(|h| !state.waiting.contains(&h.id()) && !state.ready.contains(&h.id()))
        );
    }
    #[test]
    fn budget_is_shared_and_oversized_batch_makes_progress() {
        let budget = UploadBudget {
            bytes_per_frame: 8,
            batches_per_frame: 3,
        };
        let mut ledger = UploadLedger::default();
        assert!(ledger.reserve(6, budget));
        assert!(!ledger.reserve(4, budget));
        assert!(ledger.reserve(2, budget));
        assert!(!ledger.reserve(100, budget));
        ledger = UploadLedger::default();
        assert!(ledger.reserve(100, budget));
        assert!(!ledger.reserve(1, budget));
        ledger = UploadLedger::default();
        assert!(ledger.reserve(
            1,
            UploadBudget {
                bytes_per_frame: 0,
                batches_per_frame: 0
            }
        ));
        assert!(!ledger.reserve(
            1,
            UploadBudget {
                bytes_per_frame: 0,
                batches_per_frame: 0
            }
        ));
    }
}
