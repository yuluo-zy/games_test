//! Bevy orchestration only. Domain authority stays in Editor, projection is disposable.
pub mod transition;
use bevy_app::{App, Plugin, TaskPoolPlugin, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_tasks::{AsyncComputeTaskPool, Task, futures::check_ready};
use garden_application::{BuildRequest, Change, EditCommand, EditError, Editor, JobTicket};
use garden_domain::{Building, BuildingId};
use garden_generation::mesh::{BuildingMesh, GeometryProfile, MeshError, compile_mesh};
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
}
impl GardenPlugin {
    pub fn new(settings: Settings) -> Result<Self, AdapterError> {
        if [
            settings.command_capacity,
            settings.feedback_capacity,
            settings.commands_per_frame,
            settings.max_jobs,
            settings.results_per_frame,
        ]
        .contains(&0)
        {
            return Err(AdapterError::InvalidSettings);
        }
        Ok(Self {
            settings,
            geometry: GeometryProfile::default(),
        })
    }
    pub fn with_geometry(mut self, geometry: GeometryProfile) -> Result<Self, MeshError> {
        geometry.validate()?;
        self.geometry = geometry;
        Ok(self)
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
            .init_resource::<Jobs>()
            .init_resource::<ProjectionIndex>()
            .init_resource::<ProjectionUpdates>()
            .init_resource::<PipelineStats>()
            .init_resource::<GenerationFailures>()
            .configure_sets(
                Update,
                (GardenSet::Commit, GardenSet::Collect, GardenSet::Dispatch).chain(),
            )
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
struct Completed {
    ticket: JobTicket,
    layout: BuildingLayout,
    mesh: Result<BuildingMesh, MeshError>,
}
#[derive(Resource, Default)]
struct Jobs(Vec<Task<Completed>>);
#[derive(Resource, Default)]
struct ProjectionIndex(BTreeMap<BuildingId, Entity>);

/// Coalesced accepted-projection notifications; no full-scene renderer scan.
#[derive(Resource, Default)]
pub struct ProjectionUpdates(BTreeSet<Entity>);
impl ProjectionUpdates {
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

/// One coarse entity per building. Mesh batches/instances will live under it later.
/// `target` is not an animated current pose and is never written back into domain.
#[derive(Component)]
pub struct BuildingProjection {
    pub ticket: JobTicket,
    pub target: Arc<BuildingLayout>,
    pub mesh: Arc<BuildingMesh>,
}

#[derive(Resource, Default, Debug)]
pub struct PipelineStats {
    pub launched: u64,
    pub applied: u64,
    pub discarded: u64,
    pub failed: u64,
    pub in_flight: usize,
}

#[derive(Resource, Default)]
pub struct GenerationFailures(VecDeque<(JobTicket, MeshError)>);
impl GenerationFailures {
    pub fn drain(&mut self) -> impl Iterator<Item = (JobTicket, MeshError)> + '_ {
        self.0.drain(..)
    }
}

fn commit(
    mut projections: ProjectionWriter,
    mut editor: ResMut<EditorState>,
    mut inbox: ResMut<CommandInbox>,
    mut feedback: ResMut<EditFeedback>,
    budget: Res<WorkBudget>,
    mut jobs: ResMut<Jobs>,
) {
    for _ in 0..budget.0.commands_per_frame {
        if feedback.queue.len() == feedback.capacity {
            break;
        }
        let Some(operation) = inbox.queue.pop_front() else {
            break;
        };
        let result = match operation {
            Operation::Edit(edit) => editor.0.execute(edit),
            Operation::Invalidate(id) => editor.0.invalidate(id).map(|()| None),
            Operation::ReplaceScene(buildings) => editor.0.replace_scene(buildings).map(|()| {
                projections.clear();
                // Drop old handles. A running CPU calculation is not forcibly preempted.
                jobs.0.clear();
                None
            }),
        };
        if let Ok(Some(change)) = &result
            && change.after.is_none()
        {
            projections.remove(change.building);
        }
        feedback.queue.push_back(Outcome { result });
    }
}

fn collect(
    mut projections: ProjectionWriter,
    editor: Res<EditorState>,
    mut jobs: ResMut<Jobs>,
    mut stats: ResMut<PipelineStats>,
    budget: Res<WorkBudget>,
    mut failures: ResMut<GenerationFailures>,
) {
    let mut i = 0;
    let mut completed = 0;
    while i < jobs.0.len() && completed < budget.0.results_per_frame {
        let Some(result) = check_ready(&mut jobs.0[i]) else {
            i += 1;
            continue;
        };
        drop(jobs.0.swap_remove(i));
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
                continue;
            }
        };
        let projection = BuildingProjection {
            ticket: result.ticket,
            target: Arc::new(result.layout),
            mesh: Arc::new(mesh),
        };
        projections.insert(projection);
        stats.applied += 1;
    }
}

fn dispatch(
    mut editor: ResMut<EditorState>,
    mut jobs: ResMut<Jobs>,
    budget: Res<WorkBudget>,
    mut stats: ResMut<PipelineStats>,
    profile: Res<GenerationProfile>,
) {
    let available = budget.0.max_jobs.saturating_sub(jobs.0.len());
    for BuildRequest { ticket, snapshot } in editor.0.take_requests(available) {
        let profile = profile.0;
        jobs.0.push(AsyncComputeTaskPool::get().spawn(async move {
            let layout = compile(&snapshot);
            let mesh = compile_mesh(&layout, profile);
            Completed {
                ticket,
                layout,
                mesh,
            }
        }));
        stats.launched += 1;
    }
    stats.in_flight = jobs.0.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{BlockId, BuildingEdit, Roof, sample_building};
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
        let old_mesh = compile_mesh(&old_layout, GeometryProfile::default());
        app.world_mut()
            .resource_mut::<Jobs>()
            .0
            .push(AsyncComputeTaskPool::get().spawn(async move {
                Completed {
                    ticket: old_ticket,
                    layout: old_layout,
                    mesh: old_mesh,
                }
            }));
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
