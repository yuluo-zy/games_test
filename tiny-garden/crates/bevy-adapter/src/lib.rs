//! Bevy orchestration only. Domain authority stays in Editor, projection is disposable.
pub mod context;
pub mod slots;
use slots::{GenerationSlot, GenerationSlots};
pub mod prepare;
pub mod strokes;
use context::{ContextCommitWork, ContextProjection};
pub mod transition;
use bevy_app::{App, Plugin, TaskPoolPlugin, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_tasks::{Task, TaskPool, TaskPoolBuilder, futures::check_ready};
use garden_application::{BuildRequest, Change, EditCommand, EditError, Editor, JobTicket};
use garden_domain::{Building, BuildingId};
use garden_generation::incremental::{Cancellation, PreparedBuilding};
use garden_generation::mesh::{BuildingMesh, GeometryProfile, MeshError};
use garden_generation::{BuildingLayout, RULES_REVISION, compile};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Arc,
};

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub history_limit: usize,
    pub command_capacity: usize,
    pub feedback_capacity: usize,
    pub commands_per_frame: usize,
    pub max_jobs: usize,
    pub results_per_frame: usize,
    /// Threads for the geometry pool owned by this plugin.
    ///
    /// Generation must never run on Bevy's `AsyncComputeTaskPool`: the renderer
    /// compiles shader pipelines there, so a build request would queue behind
    /// seconds of pipeline work before its own millisecond of mesh math starts.
    pub generation_threads: usize,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            history_limit: 128,
            command_capacity: 128,
            feedback_capacity: 128,
            commands_per_frame: 16,
            max_jobs: 4,
            results_per_frame: 2,
            generation_threads: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterError {
    InvalidSettings,
    QueueFull,
}

#[derive(Default)]
pub struct GardenPlugin {
    settings: Settings,
    geometry: GeometryProfile,
    compiler: Option<Arc<dyn ProjectionCompiler>>,
}

/// Immutable CPU-only art compilation runs inside versioned async jobs. No
/// Bevy asset handles enter worker threads or the domain model.
pub trait ProjectionCompiler: Send + Sync + 'static {
    fn compile(
        &self,
        layout: &mut BuildingLayout,
        profile: GeometryProfile,
    ) -> Result<BuildingMesh, MeshError>;
    /// Compatibility fallback for third-party whole-house compilers. Production
    /// kits override this; cache identity must include their template revision.
    fn compile_parts(
        &self,
        layout: &mut BuildingLayout,
        profile: GeometryProfile,
        _baseline: Option<&PreparedBuilding>,
        cancel: &Cancellation,
    ) -> Result<PreparedBuilding, MeshError> {
        cancel.check()?;
        self.compile(layout, profile).map(PreparedBuilding::whole)
    }
}
impl GardenPlugin {
    pub fn new(settings: Settings) -> Result<Self, AdapterError> {
        if [
            settings.command_capacity,
            settings.feedback_capacity,
            settings.commands_per_frame,
            settings.max_jobs,
            settings.results_per_frame,
            settings.generation_threads,
        ]
        .contains(&0)
        {
            return Err(AdapterError::InvalidSettings);
        }
        Ok(Self {
            settings,
            geometry: GeometryProfile::default(),
            compiler: None,
        })
    }
    pub fn with_geometry(mut self, geometry: GeometryProfile) -> Result<Self, MeshError> {
        geometry.validate()?;
        self.geometry = geometry;
        Ok(self)
    }
    pub fn with_compiler(mut self, compiler: Arc<dyn ProjectionCompiler>) -> Self {
        self.compiler = Some(compiler);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum GardenSet {
    Commit,
    Collect,
    Dispatch,
}

impl Plugin for GardenPlugin {
    fn build(&self, app: &mut App) {
        debug_assert_eq!(Editor::new(0).rules_revision(), RULES_REVISION);
        if !app.is_plugin_added::<TaskPoolPlugin>() {
            app.add_plugins(TaskPoolPlugin::default());
        }
        app.insert_resource(EditorState(Editor::new(self.settings.history_limit)))
            .insert_resource(CommandInbox {
                capacity: self.settings.command_capacity,
                queue: VecDeque::new(),
            })
            .insert_resource(EditFeedback {
                capacity: self.settings.feedback_capacity,
                queue: VecDeque::new(),
            })
            .insert_resource(WorkBudget(self.settings))
            .insert_resource(GenerationProfile(self.geometry))
            .insert_resource(ArtCompiler(self.compiler.clone()))
            .insert_resource(GenerationPool(
                TaskPoolBuilder::new()
                    .num_threads(self.settings.generation_threads)
                    .thread_name("garden-generation".to_string())
                    .build(),
            ))
            .init_resource::<Jobs>()
            .init_resource::<GenerationCache>()
            .init_resource::<PendingTargets>()
            .init_resource::<ProjectionIndex>()
            .init_resource::<ProjectionUpdates>()
            .init_resource::<PipelineStats>()
            .init_resource::<GenerationFailures>()
            .configure_sets(
                Update,
                (GardenSet::Commit, GardenSet::Collect, GardenSet::Dispatch).chain(),
            )
            .init_resource::<ContextProjection>()
            .init_resource::<ContextCommitWork>()
            .insert_resource(GenerationSlots::new(self.settings.max_jobs))
            .add_systems(Update, commit.in_set(GardenSet::Commit))
            .add_systems(Update, collect.in_set(GardenSet::Collect))
            .add_systems(Update, dispatch.in_set(GardenSet::Dispatch));
    }
}

#[derive(Resource)]
pub struct EditorState(Editor);
impl EditorState {
    pub fn editor(&self) -> &Editor {
        &self.0
    }
}

/// Platform/input adapters translate mouse/touch gestures into these operations.
#[derive(Debug)]
pub enum Operation {
    Edit(EditCommand),
    Invalidate(BuildingId),
    ReplaceScene(Vec<Building>),
}

#[derive(Resource)]
pub struct CommandInbox {
    capacity: usize,
    queue: VecDeque<Operation>,
}
impl CommandInbox {
    /// Backpressure is explicit: callers must not silently drop a commit.
    pub fn submit(&mut self, operation: Operation) -> Result<(), AdapterError> {
        if self.queue.len() >= self.capacity {
            return Err(AdapterError::QueueFull);
        }
        self.queue.push_back(operation);
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Debug)]
pub struct Outcome {
    pub result: Result<Option<Change>, EditError>,
}
#[derive(Resource)]
pub struct EditFeedback {
    capacity: usize,
    queue: VecDeque<Outcome>,
}
impl EditFeedback {
    pub fn drain(&mut self) -> impl Iterator<Item = Outcome> + '_ {
        self.queue.drain(..)
    }
}

#[derive(Resource)]
struct WorkBudget(Settings);
#[derive(Resource)]
struct GenerationProfile(GeometryProfile);
#[derive(Resource)]
struct ArtCompiler(Option<Arc<dyn ProjectionCompiler>>);
/// Private worker pool for layout/mesh compilation.
///
/// Bevy's renderer compiles shader pipelines on `AsyncComputeTaskPool`, which
/// only gets a quarter of the machine's threads. Sharing that pool made a
/// build request wait seconds for its turn while the mesh math itself takes
/// milliseconds, so generation owns its threads instead.
#[derive(Resource)]
pub struct GenerationPool(pub TaskPool);
struct Completed {
    ticket: JobTicket,
    layout: BuildingLayout,
    mesh: Result<PreparedBuilding, MeshError>,
}
struct Running {
    _slot: GenerationSlot,
    ticket: JobTicket,
    cancel: Cancellation,
    task: Task<Completed>,
}
#[derive(Resource, Default)]
struct Jobs(Vec<Running>);
#[derive(Resource, Default)]
struct GenerationCache(BTreeMap<BuildingId, Arc<PreparedBuilding>>);
/// Valid committed intent is available in the same frame, independently of
/// expensive projection work. The renderer acknowledges the displayed ticket.
#[derive(Resource, Default)]
pub struct PendingTargets(BTreeMap<BuildingId, PendingTarget>);
pub struct PendingTarget {
    pub ticket: JobTicket,
    pub building: Arc<Building>,
    pub failed: bool,
}
impl PendingTargets {
    pub fn get(&self, id: BuildingId) -> Option<&PendingTarget> {
        self.0.get(&id)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn displayed(&mut self, ticket: JobTicket) {
        if self
            .0
            .get(&ticket.building)
            .is_some_and(|p| p.ticket == ticket)
        {
            self.0.remove(&ticket.building);
        }
    }
}
#[derive(Resource, Default)]
struct ProjectionIndex(BTreeMap<BuildingId, Entity>);
#[derive(SystemParam)]
struct GenerationWork<'w> {
    jobs: ResMut<'w, Jobs>,
    cache: ResMut<'w, GenerationCache>,
    pending: ResMut<'w, PendingTargets>,
}

/// Coalesced accepted-projection notifications; no full-scene renderer scan.
#[derive(Resource, Default)]
pub struct ProjectionUpdates(BTreeSet<Entity>);
impl ProjectionUpdates {
    /// Reconsider an existing prepared target after presentation rolls back an
    /// unpublished stage. This doesn't submit an edit or change its ticket.
    pub fn schedule(&mut self, entity: Entity) {
        self.0.insert(entity);
    }
    pub fn take(&mut self, limit: usize) -> Vec<Entity> {
        (0..limit).filter_map(|_| self.0.pop_first()).collect()
    }
}

#[derive(SystemParam)]
struct ProjectionWriter<'w, 's> {
    commands: Commands<'w, 's>,
    index: ResMut<'w, ProjectionIndex>,
    updates: ResMut<'w, ProjectionUpdates>,
}
impl ProjectionWriter<'_, '_> {
    fn remove(&mut self, id: BuildingId) {
        if let Some(entity) = self.index.0.remove(&id) {
            self.updates.0.remove(&entity);
            self.commands.entity(entity).despawn();
        }
    }
    fn clear(&mut self) {
        for (_, entity) in std::mem::take(&mut self.index.0) {
            self.commands.entity(entity).despawn();
        }
        self.updates.0.clear();
    }
    fn insert(&mut self, projection: BuildingProjection) {
        let id = projection.ticket.building;
        let entity = if let Some(&entity) = self.index.0.get(&id) {
            self.commands.entity(entity).insert(projection);
            entity
        } else {
            let entity = self.commands.spawn(projection).id();
            self.index.0.insert(id, entity);
            entity
        };
        self.updates.0.insert(entity);
    }
}

/// One projection root per building, immutable prepared chunks beneath it in
/// presentation. `target` is never written back into the authoritative domain.
#[derive(Component)]
pub struct BuildingProjection {
    pub ticket: JobTicket,
    pub target: Arc<BuildingLayout>,
    pub mesh: Arc<PreparedBuilding>,
}

#[derive(Resource, Default, Debug)]
pub struct PipelineStats {
    pub launched: u64,
    pub applied: u64,
    pub discarded: u64,
    pub failed: u64,
    pub in_flight: usize,
    pub cancelled: u64,
    pub rebuilt_parts: u64,
    pub reused_parts: u64,
    pub generated_vertices: u64,
}

#[derive(Resource, Default)]
pub struct GenerationFailures(VecDeque<(JobTicket, MeshError)>);
impl GenerationFailures {
    pub fn drain(&mut self) -> impl Iterator<Item = (JobTicket, MeshError)> + '_ {
        self.0.drain(..)
    }
}

#[derive(SystemParam)]
struct ContextRuntime<'w> {
    context: ResMut<'w, ContextProjection>,
    semantic_work: ResMut<'w, ContextCommitWork>,
    slots: Res<'w, GenerationSlots>,
    publication: Option<ResMut<'w, context::PublicationGroups>>,
    pool: Res<'w, GenerationPool>,
}
#[derive(SystemParam)]
struct WorkerInputs<'w> {
    context: Res<'w, ContextProjection>,
    slots: Res<'w, GenerationSlots>,
    pool: Res<'w, GenerationPool>,
}
fn commit(
    mut projections: ProjectionWriter,
    mut editor: ResMut<EditorState>,
    mut inbox: ResMut<CommandInbox>,
    mut feedback: ResMut<EditFeedback>,
    budget: Res<WorkBudget>,
    work: GenerationWork,
    runtime: ContextRuntime,
) {
    let ContextRuntime {
        mut context,
        mut semantic_work,
        slots,
        mut publication,
        pool,
    } = runtime;
    semantic_work
        .retired
        .retain_mut(|(task, _)| check_ready(task).is_none());
    if matches!(inbox.queue.front(), Some(Operation::ReplaceScene(_))) {
        if let Some(job) = semantic_work.running.take() {
            semantic_work.retired.push(job);
        }
        context.busy = false;
    }
    let GenerationWork {
        mut jobs,
        mut cache,
        mut pending,
    } = work;
    for _ in 0..budget.0.commands_per_frame {
        if feedback.queue.len() == feedback.capacity {
            break;
        }
        // 语义工作占用真实池槽位，完成前不越过它执行后续操作。
        let prepared = if let Some((task, _slot)) = &mut semantic_work.running {
            let Some((candidate, resolved, parsed_cache)) = check_ready(task) else {
                break;
            };
            semantic_work.running = None;
            context.busy = false;
            Some((candidate, resolved, parsed_cache))
        } else {
            None
        };
        let operation = if prepared.is_some() {
            None
        } else {
            inbox.queue.pop_front()
        };
        if prepared.is_none() && operation.is_none() {
            break;
        }
        let contextual=operation.as_ref().is_some_and(|op|matches!(op,Operation::Edit(edit)
            if matches!(edit,EditCommand::Context(_)|EditCommand::Stroke(_)) || editor.0.has_context_history(edit)
                || editor.0.strokes().next().is_some() || !editor.0.context().linear.is_empty() || !editor.0.context().openings.is_empty() || !editor.0.context().terrain.tiles.is_empty()));
        if contextual {
            let operation = operation.unwrap();
            let Some(slot) = slots.try_acquire() else {
                inbox.queue.push_front(operation);
                break;
            };
            let Operation::Edit(edit) = operation else {
                unreachable!()
            };
            match editor.0.prepare(edit) {
                Ok(candidate) => {
                    let snapshot = context::input(candidate.snapshot.clone());
                    context.busy = true;
                    let mut parsed_cache = context.cache.clone();
                    semantic_work.running = Some((
                        pool.0.spawn(async move {
                            let resolved = garden_generation::context::resolve_cached(
                                &snapshot,
                                &mut parsed_cache,
                            );
                            (candidate, resolved, parsed_cache)
                        }),
                        slot,
                    ));
                    break;
                }
                Err(error) => {
                    feedback.queue.push_back(Outcome { result: Err(error) });
                    continue;
                }
            }
        }
        let invalidated = match operation.as_ref() {
            Some(Operation::Invalidate(id)) => Some(*id),
            _ => None,
        };
        let replacing = matches!(operation.as_ref(), Some(Operation::ReplaceScene(_)));
        let parsed = prepared.is_some();
        let result = if let Some((candidate, resolved, parsed_cache)) = prepared {
            editor
                .0
                .commit_prepared(candidate, resolved.decisions.clone())
                .map(|changes| {
                    context.cache = parsed_cache;
                    context.revision = editor.0.world_revision();
                    Arc::make_mut(&mut context.parts).update(&resolved);
                    context.semantic = Arc::new(resolved);
                    for object in &changes.objects {
                        if let garden_application::context::ObjectRef::Building(id) = object
                            && editor.0.get(*id).is_none()
                        {
                            projections.remove(*id);
                            cache.0.remove(id);
                            pending.0.remove(id);
                        }
                    }
                    None
                })
        } else {
            match operation.expect("已确认存在待执行操作") {
                Operation::Edit(edit) => editor.0.execute(edit),
                Operation::Invalidate(id) => editor.0.invalidate(id).map(|()| None),
                Operation::ReplaceScene(buildings) => editor.0.replace_scene(buildings).map(|()| {
                    projections.clear();
                    cache.0.clear();
                    pending.0.clear();
                    for job in &jobs.0 {
                        job.cancel.cancel();
                    }
                    None
                }),
            }
        };
        if let Ok(Some(change)) = &result
            && change.after.is_none()
        {
            projections.remove(change.building);
            cache.0.remove(&change.building);
            pending.0.remove(&change.building);
        }
        if result.is_ok() {
            if !parsed {
                context.refresh(&editor.0);
            }
            let changes = editor.0.last_change_set();
            if changes.revision > 0
                && let Some(groups) = &mut publication
            {
                groups.begin(
                    changes.revision,
                    changes.objects.iter().filter_map(|o| {
                        if let garden_application::context::ObjectRef::Building(id) = o {
                            editor.0.get(*id).map(|_| *id)
                        } else {
                            None
                        }
                    }),
                );
            }
            let ids = if replacing {
                editor.0.objects().map(|b| b.id()).collect::<Vec<_>>()
            } else {
                result
                    .as_ref()
                    .ok()
                    .and_then(|c| c.as_ref().map(|c| c.building))
                    .or(invalidated)
                    .into_iter()
                    .collect()
            };
            let ids = ids
                .into_iter()
                .chain(editor.0.last_change_set().objects.iter().filter_map(|o| {
                    if let garden_application::context::ObjectRef::Building(id) = o {
                        Some(*id)
                    } else {
                        None
                    }
                }))
                .collect::<BTreeSet<_>>();
            for id in ids {
                if editor.0.get(id).is_none() {
                    projections.remove(id);
                    cache.0.remove(&id);
                    pending.0.remove(&id);
                }

                if let (Some(ticket), Some(building)) =
                    (editor.0.latest_ticket(id), editor.0.get(id))
                {
                    pending.0.insert(
                        id,
                        PendingTarget {
                            ticket,
                            building: building.clone(),
                            failed: false,
                        },
                    );
                }
            }
        }
        // Keep running handles until cancellation is acknowledged. Dropping a
        // handle is not CPU preemption and must not free its scheduling slot.
        for job in &mut jobs.0 {
            if !editor.0.accepts(job.ticket) {
                job.cancel.cancel();
            }
        }
        feedback.queue.push_back(Outcome { result });
    }
}

fn collect(
    mut projections: ProjectionWriter,
    editor: Res<EditorState>,
    mut stats: ResMut<PipelineStats>,
    budget: Res<WorkBudget>,
    mut failures: ResMut<GenerationFailures>,
    work: GenerationWork,
) {
    let GenerationWork {
        mut jobs,
        mut cache,
        mut pending,
    } = work;
    let mut i = 0;
    let mut completed = 0;
    while i < jobs.0.len() && completed < budget.0.results_per_frame {
        let Some(result) = check_ready(&mut jobs.0[i].task) else {
            i += 1;
            continue;
        };
        let job = jobs.0.swap_remove(i);
        if job.cancel.is_cancelled() {
            stats.cancelled += 1;
        }
        completed += 1;
        if !editor.0.accepts(result.ticket) {
            stats.discarded += 1;
            continue;
        }
        let mesh = match result.mesh {
            Ok(mesh) => mesh,
            Err(error) => {
                stats.failed += 1;
                if failures.0.len() == 64 {
                    failures.0.pop_front();
                }
                failures.0.push_back((result.ticket, error));
                if let Some(target) = pending.0.get_mut(&result.ticket.building) {
                    target.failed = true;
                }
                continue;
            }
        };
        stats.rebuilt_parts += mesh.rebuilt_parts as u64;
        stats.reused_parts += mesh.reused_parts as u64;
        stats.generated_vertices += mesh.generated_vertices as u64;
        let mesh = Arc::new(mesh);
        cache.0.insert(result.ticket.building, mesh.clone());
        let projection = BuildingProjection {
            ticket: result.ticket,
            target: Arc::new(result.layout),
            mesh,
        };
        projections.insert(projection);
        stats.applied += 1;
    }
}

fn dispatch(
    mut editor: ResMut<EditorState>,
    budget: Res<WorkBudget>,
    mut stats: ResMut<PipelineStats>,
    profile: Res<GenerationProfile>,
    compiler: Res<ArtCompiler>,
    inputs: WorkerInputs,
    work: GenerationWork,
) {
    let WorkerInputs {
        pool,
        context,
        slots,
    } = inputs;
    let GenerationWork {
        mut jobs, cache, ..
    } = work;
    let available = budget.0.max_jobs.saturating_sub(jobs.0.len());
    let busy = jobs
        .0
        .iter()
        .map(|job| job.ticket.building)
        .collect::<BTreeSet<_>>();
    let leases = (0..available)
        .filter_map(|_| slots.try_acquire())
        .collect::<Vec<_>>();
    let requests = editor
        .0
        .take_requests_where(leases.len(), |id| !busy.contains(&id));
    for (BuildRequest { ticket, snapshot }, slot) in requests.into_iter().zip(leases) {
        let profile = profile.0;
        let compiler = compiler.0.clone();
        let baseline = cache.0.get(&ticket.building).cloned();
        let cancel = Cancellation::default();
        let worker_cancel = cancel.clone();
        let parts = context.parts.clone();
        let context = context.semantic.clone();
        let task = pool.0.spawn(async move {
            let mut layout = if worker_cancel.is_cancelled() {
                BuildingLayout {
                    building: ticket.building,
                    placement: snapshot.placement(),
                    blocks: Vec::new(),
                }
            } else {
                compile(&snapshot)
            };
            garden_generation::context::apply_layout_with_parts(&mut layout, &context, &parts);
            let mesh = worker_cancel.check().and_then(|()| {
                let mut mesh = if let Some(compiler) = compiler {
                    compiler.compile_parts(
                        &mut layout,
                        profile,
                        baseline.as_deref(),
                        &worker_cancel,
                    )?
                } else {
                    garden_generation::incremental::compile_structure(
                        &layout,
                        profile,
                        baseline.as_deref(),
                        &worker_cancel,
                    )?
                };
                prepare::prepare(&mut mesh, &worker_cancel)?;
                Ok(mesh)
            });
            Completed {
                ticket,
                layout,
                mesh,
            }
        });
        jobs.0.push(Running {
            _slot: slot,
            ticket,
            cancel,
            task,
        });
        stats.launched += 1;
    }
    stats.in_flight = jobs.0.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{BlockId, BuildingEdit, Roof, sample_building};
    use garden_generation::mesh::compile_mesh;
    use std::time::{Duration, Instant};
    fn id() -> BuildingId {
        BuildingId::new(1).unwrap()
    }
    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(GardenPlugin::default());
        app
    }
    #[test]
    fn one_running_job_keeps_latest_pending_without_losing_undo_history() {
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        struct Controlled {
            gate: Arc<AtomicBool>,
            started: Arc<AtomicBool>,
            active: Arc<AtomicUsize>,
        }
        impl ProjectionCompiler for Controlled {
            fn compile(
                &self,
                layout: &mut BuildingLayout,
                profile: GeometryProfile,
            ) -> Result<BuildingMesh, MeshError> {
                compile_mesh(layout, profile)
            }
            fn compile_parts(
                &self,
                layout: &mut BuildingLayout,
                profile: GeometryProfile,
                _: Option<&PreparedBuilding>,
                cancel: &Cancellation,
            ) -> Result<PreparedBuilding, MeshError> {
                assert_eq!(
                    self.active.fetch_add(1, Ordering::SeqCst),
                    0,
                    "same building concurrently generating"
                );
                self.started.store(true, Ordering::SeqCst);
                // Controlled non-preemptible section verifies that requesting
                // cancellation does not free a slot before acknowledgement.
                let deadline = Instant::now() + Duration::from_secs(5);
                while !self.gate.load(Ordering::SeqCst) && Instant::now() < deadline {
                    std::thread::yield_now();
                }
                self.active.fetch_sub(1, Ordering::SeqCst);
                cancel.check()?;
                self.compile(layout, profile).map(PreparedBuilding::whole)
            }
        }
        let gate = Arc::new(AtomicBool::new(false));
        let started = Arc::new(AtomicBool::new(false));
        let active = Arc::new(AtomicUsize::new(0));
        let mut app = App::new();
        app.add_plugins(GardenPlugin::default().with_compiler(Arc::new(Controlled {
            gate: gate.clone(),
            started: started.clone(),
            active,
        })));
        submit(&mut app, EditCommand::Create(sample_building(id(), 6., 6.)));
        app.update();
        let deadline = Instant::now() + Duration::from_secs(5);
        while !started.load(Ordering::SeqCst) {
            assert!(Instant::now() < deadline);
            std::thread::yield_now();
        }
        for i in 0..10 {
            submit(
                &mut app,
                EditCommand::Edit {
                    building: id(),
                    edit: BuildingEdit::SetRoof {
                        block: BlockId::new(1).unwrap(),
                        roof: if i % 2 == 0 { Roof::Flat } else { Roof::Hipped },
                    },
                },
            );
            app.update();
            assert_eq!(app.world().resource::<Jobs>().0.len(), 1);
            assert_eq!(app.world().resource::<PipelineStats>().launched, 1);
            assert!(app.world().resource::<Jobs>().0[0].cancel.is_cancelled());
            app.world_mut()
                .resource_mut::<EditFeedback>()
                .drain()
                .for_each(|o| {
                    o.result.unwrap();
                });
        }
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            11
        );
        gate.store(true, Ordering::SeqCst);
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().applied == 1
        });
        assert_eq!(app.world().resource::<PipelineStats>().launched, 2);
        assert_eq!(app.world().resource::<PipelineStats>().cancelled, 1);
        let world = app.world_mut();
        assert_eq!(
            world
                .query::<&BuildingProjection>()
                .single(world)
                .unwrap()
                .target
                .blocks[0]
                .effective_roof,
            Roof::Hipped
        );
    }
    #[test]
    fn preview_intent_is_immediate_validated_and_deleted_with_object() {
        let mut app = app();
        submit(&mut app, EditCommand::Create(sample_building(id(), 6., 6.)));
        app.update();
        let ticket = app
            .world()
            .resource::<PendingTargets>()
            .get(id())
            .unwrap()
            .ticket;
        submit(
            &mut app,
            EditCommand::Edit {
                building: id(),
                edit: BuildingEdit::Resize {
                    block: BlockId::new(1).unwrap(),
                    footprint: garden_geometry::Rect {
                        x: 0.,
                        z: 0.,
                        width: 6.,
                        depth: 6.,
                    },
                    height: -1.,
                },
            },
        );
        app.update();
        assert_eq!(
            app.world()
                .resource::<PendingTargets>()
                .get(id())
                .unwrap()
                .ticket,
            ticket
        );
        submit(&mut app, EditCommand::Delete(id()));
        app.update();
        assert!(app.world().resource::<PendingTargets>().get(id()).is_none());
    }
    #[test]
    fn rejected_geometry_reports_failure_without_installing_a_bad_projection() {
        let mut app = app();
        let mut draft = sample_building(id(), 6.0, 3.0);
        draft.placement.x = 1e30;
        submit(&mut app, EditCommand::Create(draft));
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().failed == 1
        });
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id())
                .is_some()
        );
        let world = app.world_mut();
        assert_eq!(world.query::<&BuildingProjection>().iter(world).count(), 0);
        assert_eq!(
            world.resource_mut::<GenerationFailures>().drain().count(),
            1
        );
    }
    fn submit(app: &mut App, command: EditCommand) {
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Edit(command))
            .unwrap();
    }
    fn tick_until(app: &mut App, ready: impl Fn(&mut App) -> bool) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            app.update();
            for outcome in app.world_mut().resource_mut::<EditFeedback>().drain() {
                outcome.result.unwrap();
            }
            if ready(app) {
                break;
            }
            assert!(Instant::now() < deadline, "async pipeline timed out");
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    #[test]
    fn scene_swap_removes_old_projection_and_rebuilds_new_session() {
        let mut app = app();
        submit(
            &mut app,
            EditCommand::Create(sample_building(id(), 6.0, 3.0)),
        );
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().applied == 1
        });
        let world = app.world_mut();
        let old_session = world
            .query::<&BuildingProjection>()
            .single(world)
            .unwrap()
            .ticket
            .session;
        let replacement = Building::try_new(sample_building(id(), 10.0, 12.0)).unwrap();
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::ReplaceScene(vec![replacement]))
            .unwrap();
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().applied == 2
        });
        let world = app.world_mut();
        let projection = world.query::<&BuildingProjection>().single(world).unwrap();
        assert!(projection.ticket.session > old_session);
        assert_eq!(projection.target.blocks[0].stories, 4);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            0
        );
    }
    #[test]
    fn async_pipeline_projects_then_deletes_and_undo_restores() {
        let mut app = app();
        submit(
            &mut app,
            EditCommand::Create(sample_building(id(), 10.0, 9.0)),
        );
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().applied == 1
        });
        let world = app.world_mut();
        let mut query = world.query::<&BuildingProjection>();
        let projection = query.single(world).unwrap();
        assert_eq!(projection.target.blocks[0].stories, 3);
        submit(&mut app, EditCommand::Delete(id()));
        app.update();
        let world = app.world_mut();
        assert_eq!(world.query::<&BuildingProjection>().iter(world).count(), 0);
        submit(&mut app, EditCommand::Undo);
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().applied == 2
        });
    }
    #[test]
    fn zero_generation_threads_is_rejected_like_every_other_budget() {
        assert!(
            GardenPlugin::new(Settings {
                generation_threads: 0,
                ..Settings::default()
            })
            .is_err()
        );
    }
    #[test]
    fn generation_runs_on_its_own_pool_not_bevys_shared_compute_pool() {
        let mut app = App::new();
        app.add_plugins(
            GardenPlugin::new(Settings {
                generation_threads: 3,
                ..Settings::default()
            })
            .unwrap(),
        );
        assert_eq!(app.world().resource::<GenerationPool>().0.thread_num(), 3);
    }
    #[test]
    fn batched_commands_coalesce_and_enforce_backpressure() {
        let settings = Settings {
            command_capacity: 3,
            ..Settings::default()
        };
        let mut app = App::new();
        app.add_plugins(GardenPlugin::new(settings).unwrap());
        submit(
            &mut app,
            EditCommand::Create(sample_building(id(), 6.0, 3.0)),
        );
        for roof in [Roof::Flat, Roof::Hipped] {
            submit(
                &mut app,
                EditCommand::Edit {
                    building: id(),
                    edit: BuildingEdit::SetRoof {
                        block: BlockId::new(1).unwrap(),
                        roof,
                    },
                },
            );
        }
        assert_eq!(
            app.world_mut()
                .resource_mut::<CommandInbox>()
                .submit(Operation::Edit(EditCommand::Undo)),
            Err(AdapterError::QueueFull)
        );
        tick_until(&mut app, |app| {
            app.world().resource::<PipelineStats>().applied == 1
        });
        assert_eq!(app.world().resource::<PipelineStats>().launched, 1);
        let world = app.world_mut();
        assert_eq!(
            world
                .query::<&BuildingProjection>()
                .single(world)
                .unwrap()
                .target
                .blocks[0]
                .effective_roof,
            Roof::Hipped
        );
    }
    #[test]
    fn stale_completion_never_replaces_latest_target() {
        let mut app = app();
        submit(
            &mut app,
            EditCommand::Create(sample_building(id(), 6.0, 3.0)),
        );
        app.update();
        // Force a ready old result to exercise the collector deterministically.
        app.world_mut().resource_mut::<Jobs>().0.clear();
        let old_ticket = app
            .world()
            .resource::<EditorState>()
            .0
            .latest_ticket(id())
            .unwrap();
        let old_layout = compile(app.world().resource::<EditorState>().0.get(id()).unwrap());
        let old_mesh =
            compile_mesh(&old_layout, GeometryProfile::default()).map(PreparedBuilding::whole);
        let stale = app
            .world()
            .resource::<GenerationPool>()
            .0
            .spawn(async move {
                Completed {
                    ticket: old_ticket,
                    layout: old_layout,
                    mesh: old_mesh,
                }
            });
        let slot = app
            .world()
            .resource::<GenerationSlots>()
            .try_acquire()
            .unwrap();
        app.world_mut().resource_mut::<Jobs>().0.push(Running {
            _slot: slot,
            ticket: old_ticket,
            cancel: Cancellation::default(),
            task: stale,
        });
        submit(
            &mut app,
            EditCommand::Edit {
                building: id(),
                edit: BuildingEdit::SetRoof {
                    block: BlockId::new(1).unwrap(),
                    roof: Roof::Flat,
                },
            },
        );
        tick_until(&mut app, |app| {
            let stats = app.world().resource::<PipelineStats>();
            stats.applied == 1 && stats.discarded == 1
        });
        assert_eq!(app.world().resource::<PipelineStats>().discarded, 1);
        let world = app.world_mut();
        assert_eq!(
            world
                .query::<&BuildingProjection>()
                .single(world)
                .unwrap()
                .target
                .blocks[0]
                .effective_roof,
            Roof::Flat
        );
    }
}
