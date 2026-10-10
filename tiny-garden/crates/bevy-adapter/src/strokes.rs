//! One latest-wins dependency snapshot for the bounded wall/path slice.
use super::{EditorState, GardenSet, GenerationPool};
#[path = "stroke_context.rs"]
mod context_geometry;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_tasks::{Task, futures::check_ready};
use garden_domain::strokes::StrokeId;
#[cfg(test)]
use garden_domain::strokes::{Stroke, StrokeKind};
#[cfg(test)]
use garden_generation::strokes::{WallInput, compile_paths, compile_wall, wall_inputs};
use garden_generation::{
    incremental::{Cancellation, PreparedBuilding},
    mesh::MeshError,
};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Resource, Default)]
pub struct StrokeTargets {
    /// 修订号零也可能是合法快照；初始默认值不能冒充已经生成的空场景。
    pub ready: bool,
    pub revision: u64,
    pub meshes: BTreeMap<StrokeId, Arc<PreparedBuilding>>,
    pub generated_walls: u64,
    pub reused_walls: u64,
    pub error: Option<MeshError>,
}
#[derive(Clone, Default)]
struct Cache {
    #[cfg(test)]
    walls: BTreeMap<StrokeId, WallInput>,
    meshes: BTreeMap<StrokeId, Arc<PreparedBuilding>>,
    #[cfg(test)]
    paths: Vec<Arc<Stroke>>,
    context_keys: BTreeMap<StrokeId, context_geometry::ContextMeshKey>,
    groups: garden_generation::context_mesh::GroupRegistry,
}
struct Output {
    revision: u64,
    cache: Cache,
    generated: u64,
    reused: u64,
}
struct Running {
    _slot: super::slots::GenerationSlot,
    revision: u64,
    cancel: Cancellation,
    task: Task<Result<Output, MeshError>>,
}
#[derive(Resource, Default)]
struct Work {
    job: Option<Running>,
    cache: Cache,
    attempted: Option<u64>,
}
pub struct StrokeGenerationPlugin;
impl Plugin for StrokeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StrokeTargets>()
            .init_resource::<Work>()
            .add_systems(Update, update.after(GardenSet::Commit));
    }
}
fn update(
    editor: Res<EditorState>,
    pool: Res<GenerationPool>,
    mut work: ResMut<Work>,
    mut targets: ResMut<StrokeTargets>,
    context: Res<super::context::ContextProjection>,
    slots: Res<super::slots::GenerationSlots>,
) {
    let revision = context.revision;
    if let Some(job) = &mut work.job {
        if job.revision != revision {
            job.cancel.cancel();
        }
        if let Some(result) = check_ready(&mut job.task) {
            work.job = None;
            match result {
                Ok(out) if out.revision == revision => {
                    targets.ready = true;
                    targets.revision = revision;
                    targets.meshes = out.cache.meshes.clone();
                    targets.generated_walls += out.generated;
                    targets.reused_walls += out.reused;
                    targets.error = None;
                    work.cache = out.cache;
                }
                Err(error) if work.attempted == Some(revision) => {
                    targets.revision = revision;
                    targets.error = Some(error);
                }
                _ => {}
            }
        }
    }
    if work.job.is_none() && work.attempted != Some(revision) {
        let Some(slot) = slots.try_acquire() else {
            return;
        };
        targets.error = None;
        let snapshot = super::context::input(editor.editor().snapshot());
        let semantic = context.semantic.clone();
        let baseline = work.cache.clone();
        let cancel = Cancellation::default();
        let worker = cancel.clone();
        let task = pool.0.spawn(async move {
            context_geometry::build_context(revision, &snapshot, &semantic, baseline, &worker)
        });
        work.job = Some(Running {
            _slot: slot,
            revision,
            cancel,
            task,
        });
        work.attempted = Some(revision);
    }
}
#[cfg(test)]
fn build(
    revision: u64,
    strokes: &[Arc<Stroke>],
    baseline: Cache,
    cancel: &Cancellation,
) -> Result<Output, MeshError> {
    let mut cache = Cache::default();
    let mut generated = 0;
    let mut reused = 0;
    for input in wall_inputs(strokes) {
        cancel.check()?;
        let id = input.wall.id;
        let mesh = if baseline.walls.get(&id) == Some(&input) {
            reused += 1;
            baseline.meshes[&id].clone()
        } else {
            generated += 1;
            let mut mesh = PreparedBuilding::whole(compile_wall(&input, cancel)?);
            super::prepare::prepare(&mut mesh, cancel)?;
            Arc::new(mesh)
        };
        cache.walls.insert(id, input);
        cache.meshes.insert(id, mesh);
    }
    let paths = strokes
        .iter()
        .filter(|s| s.kind == StrokeKind::Path)
        .cloned()
        .collect::<Vec<_>>();
    if !paths.is_empty() {
        let mesh = if paths == baseline.paths && baseline.meshes.contains_key(&StrokeId(0)) {
            baseline.meshes[&StrokeId(0)].clone()
        } else {
            let mut mesh = PreparedBuilding::whole(compile_paths(&paths, cancel)?);
            super::prepare::prepare(&mut mesh, cancel)?;
            Arc::new(mesh)
        };
        cache.meshes.insert(StrokeId(0), mesh);
    }
    cache.paths = paths;
    cancel.check()?;
    Ok(Output {
        revision,
        cache,
        generated,
        reused,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::strokes::Point;
    #[test]
    fn dependency_cache_reuses_remote_wall_and_path_delete_restores_wall() {
        let stroke = |id, kind, z| {
            Arc::new(Stroke {
                id: StrokeId(id),
                kind,
                points: vec![Point { x: 0., z }, Point { x: 6., z }],
                width: 1.,
                height: 2.4,
            })
        };
        let a = stroke(1, StrokeKind::Wall, 0.);
        let b = stroke(2, StrokeKind::Wall, 9.);
        let first = build(
            1,
            &[a.clone(), b.clone()],
            Cache::default(),
            &Cancellation::default(),
        )
        .unwrap();
        let old = first.cache.meshes[&b.id].clone();
        let path = Arc::new(Stroke {
            id: StrokeId(3),
            kind: StrokeKind::Path,
            points: vec![Point { x: 3., z: -3. }, Point { x: 3., z: 3. }],
            width: 1.,
            height: 2.4,
        });
        let next = build(
            2,
            &[a.clone(), b.clone(), path],
            first.cache,
            &Cancellation::default(),
        )
        .unwrap();
        assert_eq!(next.generated, 1);
        assert_eq!(next.reused, 1);
        assert!(Arc::ptr_eq(&old, &next.cache.meshes[&b.id]));
        let restored = build(3, &[a, b], next.cache, &Cancellation::default()).unwrap();
        assert_eq!(restored.generated, 1);
        assert!(!restored.cache.meshes.contains_key(&StrokeId(0)));
    }
}
