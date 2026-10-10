//! 地形、自动笔画与手工窗的桌面输入。候选语义后台解析，正式编辑经统一事务提交。
//! 手势只持有候选快照；取消不写领域、索引或历史。
use crate::{
    camera::{CameraAction, OrbitCamera},
    editor::{DesktopEditor, EditorAction},
    pointer::{DesktopInputSet, PointerFrame},
    strokes::{StrokeAction, StrokeTools},
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::tasks::{Task, futures::check_ready};
use garden_application::{EditCommand, context::ContextEdit, tools::Tool};
use garden_bevy::{
    CommandInbox, EditorState, GardenSet, GenerationPool,
    context::ContextProjection,
    slots::{GenerationSlot, GenerationSlots},
};
use garden_domain::{
    context::*,
    strokes::{Point, StrokeKind},
    terrain::{BrushKind, BrushSample, TerrainDocument},
};
use garden_generation::context::{ContextInput, ResolvedContext, WallSurface};
use std::sync::Arc;
#[derive(Component, Clone, Copy)]
pub enum ContextAction {
    AutoDraw,
    AutoType,
    LockPath,
    LockFence,
    LockWall,
    Style,
    Raise,
    Lower,
    Smooth,
    PlaceWindow,
    EditWindow,
    DeleteWindow,
    WindowWider,
    WindowNarrower,
}
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ContextMode {
    #[default]
    None,
    Raise,
    Lower,
    Smooth,
    PlaceWindow,
    EditWindow,
}
enum Drag {
    Terrain {
        candidate: TerrainDocument,
        revision: u64,
    },
    Window {
        candidate: OpeningIntent,
        surface: WallSurface,
        revision: u64,
    },
}
type PreviewResult = (
    ResolvedContext,
    garden_generation::context_cache::ContextCache,
);
struct PreviewJob {
    sequence: u64,
    revision: u64,
    epoch: u64,
    task: Task<PreviewResult>,
    _slot: GenerationSlot,
}
#[derive(Resource, Default)]
pub struct ContextTools {
    pub mode: ContextMode,
    pub selected: Option<OpeningId>,
    pub pending_stroke: Option<garden_domain::strokes::StrokeId>,
    drag: Option<Drag>,
    pub preview: Option<Arc<ResolvedContext>>,
    desired: Option<(u64, ContextInput)>,
    sequence: u64,
    epoch: u64,
    active: bool,
    resolved_sequence: Option<u64>,
    last_input: Option<ContextInput>,
    job: Option<PreviewJob>,
    cache: garden_generation::context_cache::ContextCache,
    finish: Option<ContextEdit>,
    revision: u64,
    observed_session: Option<u64>,
}
impl ContextTools {
    pub fn owns_pointer(&self) -> bool {
        self.mode != ContextMode::None
    }
    fn clear(&mut self) {
        self.pending_stroke = None;
        self.drag = None;
        self.preview = None;
        self.desired = None;
        self.finish = None;
        self.sequence += 1;
        self.epoch += 1;
        self.active = false;
        self.last_input = None;
        self.resolved_sequence = None;
    }
    pub fn preview_input(&mut self, input: ContextInput, revision: u64) {
        if self.active && self.revision == revision && self.last_input.as_ref() == Some(&input) {
            return;
        }
        self.active = true;
        self.last_input = Some(input.clone());
        self.sequence += 1;
        self.desired = Some((self.sequence, input));
        self.revision = revision;
    }
    pub fn finish_edit(&mut self, edit: ContextEdit) {
        if let ContextEdit::Stroke { stroke, .. } = &edit {
            self.pending_stroke = Some(stroke.id);
        }
        self.finish = Some(edit);
    }
}
#[derive(Resource)]
pub struct GroundMaterial(pub Handle<StandardMaterial>);
#[derive(Component)]
pub struct GroundBase;
fn setup_ground(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(GroundMaterial(materials.add(StandardMaterial {
        base_color: Color::srgb_u8(135, 157, 115),
        perceptual_roughness: 0.95,
        ..default()
    })));
}
fn ground_base_visibility(
    display: Res<crate::strokes::StrokeDisplay>,
    mut ground: Query<&mut Visibility, With<GroundBase>>,
) {
    let wanted = if display.has_terrain() {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    for mut v in &mut ground {
        if *v != wanted {
            *v = wanted;
        }
    }
}
#[cfg(test)]
mod tests;
pub struct ContextToolsPlugin;
impl Plugin for ContextToolsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextTools>()
            .init_resource::<garden_bevy::context::PublicationGroups>()
            .init_gizmo_group::<bevy::gizmos::config::DefaultGizmoConfigGroup>()
            .add_systems(Startup, setup_ground)
            .add_systems(Update, ground_base_visibility.after(GardenSet::Collect))
            .add_systems(
                Update,
                (actions, gestures)
                    .chain()
                    .in_set(DesktopInputSet::Tools)
                    .before(GardenSet::Commit),
            )
            .add_systems(
                Update,
                (preview_jobs, overlay).chain().after(GardenSet::Collect),
            );
    }
}
pub fn ground(ray: Ray3d, terrain: &TerrainDocument) -> Option<Point> {
    garden_generation::context::terrain_hit(
        ray.origin.to_array().map(f64::from),
        ray.direction.to_array().map(f64::from),
        terrain,
    )
    .map(|p| Point { x: p[0], z: p[2] })
}
type ActionButton<'a> = (
    &'a Interaction,
    Option<&'a ContextAction>,
    Option<&'a EditorAction>,
    Option<&'a CameraAction>,
    Option<&'a StrokeAction>,
);
fn guarded(edit: ContextEdit, revision: u64) -> EditCommand {
    EditCommand::Context(ContextEdit::Guarded {
        revision,
        edit: Box::new(edit),
    })
}
fn actions(
    buttons: Query<ActionButton<'_>, Changed<Interaction>>,
    editor: Res<EditorState>,
    context: Res<ContextProjection>,
    mut tools: ResMut<ContextTools>,
    mut stroke_tools: ResMut<StrokeTools>,
    mut desktop: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    for (interaction, action, legacy, camera, stroke) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if legacy.is_some() || camera.is_some() || stroke.is_some() {
            tools.mode = ContextMode::None;
            tools.clear();
        }
        let Some(action) = action else {
            continue;
        };
        if context.busy {
            desktop.status = "上一笔修改正在解析，请稍候。".into();
            continue;
        }
        tools.clear();
        desktop.tools.cancel();
        stroke_tools.cancel();
        match action {
            ContextAction::AutoDraw => {
                tools.mode = ContextMode::None;
                desktop.tools.set_tool(Tool::Path);
                stroke_tools.auto_draw = true;
                desktop.status = "自动画笔：拖出轨迹，选中后增高可变为篱笆或围墙。".into();
            }
            ContextAction::Raise
            | ContextAction::Lower
            | ContextAction::Smooth
            | ContextAction::PlaceWindow
            | ContextAction::EditWindow => {
                let cycling = tools.mode == ContextMode::EditWindow;
                tools.mode = match action {
                    ContextAction::Raise => ContextMode::Raise,
                    ContextAction::Lower => ContextMode::Lower,
                    ContextAction::Smooth => ContextMode::Smooth,
                    ContextAction::PlaceWindow => ContextMode::PlaceWindow,
                    _ => ContextMode::EditWindow,
                };
                desktop.tools.set_tool(Tool::Select);
                desktop.status = "在场景中拖动；右键、失焦或取消可放弃候选。".into();
                if matches!(action, ContextAction::EditWindow) {
                    let openings = &editor.editor().context().openings;
                    let current = tools.selected.filter(|id| openings.contains_key(id));
                    tools.selected = if cycling {
                        openings
                            .keys()
                            .copied()
                            .find(|id| current.is_none_or(|current| *id > current))
                            .or_else(|| openings.keys().next().copied())
                    } else {
                        current.or_else(|| openings.keys().next().copied())
                    };
                    if let Some(id) = tools.selected {
                        desktop.status =
                            format!("已选手工窗 #{}；再次点「编辑窗」切换，包括休眠窗。", id.0);
                    }
                }
            }
            ContextAction::DeleteWindow
            | ContextAction::WindowWider
            | ContextAction::WindowNarrower => {
                let Some(id) = tools.selected else {
                    desktop.status = "请先用「编辑窗」选择手工窗。".into();
                    continue;
                };
                let Some(window) = editor.editor().context().openings.get(&id) else {
                    continue;
                };
                let edit = if matches!(action, ContextAction::DeleteWindow) {
                    ContextEdit::DeleteOpening(id)
                } else {
                    let mut window = window.clone();
                    window.width += (if matches!(action, ContextAction::WindowWider) {
                        1.
                    } else {
                        -1.
                    }) * 0.1;
                    ContextEdit::PutOpening(window)
                };
                desktop.submit(guarded(edit, editor.editor().world_revision()), &mut inbox);
            }
            _ => {
                let Some(id) = stroke_tools.selected else {
                    desktop.status = "请先选择一笔轨迹。".into();
                    continue;
                };
                let Some(stroke) = editor.editor().stroke(id) else {
                    continue;
                };
                let mut stroke = stroke.as_ref().clone();
                let mut intent = editor
                    .editor()
                    .context()
                    .linear
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| LinearIntent::legacy(&stroke));
                match action {
                    ContextAction::AutoType => {
                        intent.mode = StructureMode::Auto;
                        if stroke.kind == StrokeKind::Path && stroke.height >= 0.8 {
                            stroke.height = 0.;
                        }
                        stroke.width = stroke.width.min(0.6);
                    }
                    ContextAction::LockPath => {
                        intent.mode = StructureMode::Locked(StrokeKind::Path);
                        stroke.height = 0.;
                    }
                    ContextAction::LockFence => {
                        intent.mode = StructureMode::Locked(StrokeKind::Fence);
                        stroke.height = stroke.height.max(0.8);
                    }
                    ContextAction::LockWall => {
                        intent.mode = StructureMode::Locked(StrokeKind::Wall);
                        stroke.height = stroke.height.max(0.8);
                    }
                    ContextAction::Style => {
                        intent.style = match intent.style {
                            BoundaryStyle::Auto => BoundaryStyle::Timber,
                            BoundaryStyle::Timber => BoundaryStyle::Stone,
                            BoundaryStyle::Stone => BoundaryStyle::Auto,
                        };
                    }
                    _ => {}
                }
                desktop.submit(
                    guarded(
                        ContextEdit::Stroke { stroke, intent },
                        editor.editor().world_revision(),
                    ),
                    &mut inbox,
                );
            }
        }
    }
}
fn wall_hit(ray: Ray3d, surface: &WallSurface) -> Option<(f64, f64, f64)> {
    let direction = ray.direction;
    let dx = surface.end.x - surface.start.x;
    let dz = surface.end.z - surface.start.z;
    let length = dx.hypot(dz);
    let normal = [-dz / length, dx / length];
    let denominator = direction.x as f64 * normal[0] + direction.z as f64 * normal[1];
    if denominator.abs() < 1e-8 {
        return None;
    }
    let t = ((surface.start.x - ray.origin.x as f64) * normal[0]
        + (surface.start.z - ray.origin.z as f64) * normal[1])
        / denominator;
    if t < 0. {
        return None;
    }
    let p = ray.get_point(t as f32);
    let along =
        ((p.x as f64 - surface.start.x) * dx + (p.z as f64 - surface.start.z) * dz) / length;
    let elevation = p.y as f64 - surface.base;
    (along >= 0. && along <= length && elevation >= 0. && elevation <= surface.height)
        .then_some((t, along, elevation))
}
#[derive(SystemParam)]
struct GestureWorld<'w, 's> {
    frame: Res<'w, PointerFrame>,
    editor: Res<'w, EditorState>,
    context: Res<'w, ContextProjection>,
    cameras: Query<'w, 's, (&'static Camera, &'static Transform), With<OrbitCamera>>,
    time: Option<Res<'w, Time>>,
    display: Res<'w, crate::strokes::StrokeDisplay>,
    pending: Res<'w, garden_bevy::PendingTargets>,
    displayed: Query<
        'w,
        's,
        (
            &'static crate::renderer::DisplayedBuilding,
            &'static Transform,
        ),
    >,
}
fn gestures(
    world: GestureWorld,
    mut tools: ResMut<ContextTools>,
    mut desktop: ResMut<DesktopEditor>,
) {
    if world.frame.cancel_world {
        tools.clear();
        return;
    }
    if !tools.owns_pointer() || desktop.pending.is_some() {
        return;
    }
    if !(world.frame.world_begin || world.frame.world_held || world.frame.world_finish) {
        return;
    }
    let ray = world.frame.cursor.and_then(|p| {
        world
            .cameras
            .iter()
            .next()
            .and_then(|(c, t)| c.viewport_to_world(&GlobalTransform::from(*t), p).ok())
    });
    let Some(ray) = ray else {
        if world.frame.world_finish {
            tools.clear();
        }
        return;
    };
    let editor = world.editor.editor();
    if world.frame.world_begin {
        if world.context.busy
            || world.display.revision != Some(editor.world_revision())
            || !world.pending.is_empty()
        {
            desktop.status = "结构正在显示交接，请稍候再开始编辑。".into();
            return;
        }
        let revision = editor.world_revision();
        if matches!(
            tools.mode,
            ContextMode::Raise | ContextMode::Lower | ContextMode::Smooth
        ) {
            tools.drag = Some(Drag::Terrain {
                candidate: editor.context().terrain.clone(),
                revision,
            });
        } else {
            if world.displayed.iter().any(|(b, t)| {
                let p = b.target.placement;
                let target = Vec3::new(p.x as f32, p.elevation as f32, p.z as f32);
                t.translation.distance_squared(target) > 1e-6
                    || t.rotation
                        .angle_between(Quat::from_rotation_y(p.yaw as f32))
                        > 1e-3
            }) {
                desktop.status = "建筑位置仍在过渡，请稍候再编辑窗户。".into();
                return;
            }
            let terrain_distance = world
                .display
                .has_terrain()
                .then(|| {
                    garden_generation::context::terrain_hit(
                        ray.origin.to_array().map(f64::from),
                        ray.direction.to_array().map(f64::from),
                        &editor.context().terrain,
                    )
                })
                .flatten()
                .map(|p| (Vec3::from_array(p.map(|v| v as f32)) - ray.origin).dot(*ray.direction));
            let visible = world
                .displayed
                .iter()
                .filter_map(|(b, t)| crate::picking::hit_distance(ray, b, t))
                .chain(world.display.nearest_structure_hit(ray))
                .chain(terrain_distance)
                .min_by(f32::total_cmp);
            let nearest = world
                .context
                .semantic
                .facades
                .values()
                .filter_map(|f| {
                    wall_hit(ray, &f.surface)
                        .filter(|hit| {
                            visible.is_some_and(|distance| (hit.0 - distance as f64).abs() < 0.5)
                        })
                        .map(|hit| (hit, f.surface.clone()))
                })
                .min_by(|a, b| a.0.0.total_cmp(&b.0.0));
            let Some(((distance, along, elevation), surface)) = nearest else {
                desktop.status = "请点击可见建筑墙面。".into();
                return;
            };
            // 最近可见主体若比目标墙面更近，不穿过它在背后创建窗户。
            let _ = distance;
            let selected = editor
                .context()
                .openings
                .values()
                .filter(|w| w.host == surface.anchor)
                .min_by(|a, b| {
                    ((a.along - along).hypot(a.elevation - elevation))
                        .total_cmp(&((b.along - along).hypot(b.elevation - elevation)))
                });
            let candidate = if tools.mode == ContextMode::EditWindow {
                let Some(window) = selected.filter(|w| {
                    (w.along - along).abs() < w.width / 2. + 0.2
                        && (w.elevation - elevation).abs() < w.height / 2. + 0.2
                }) else {
                    desktop.status = "此处没有手工窗；休眠窗可通过墙面原位置重新选择。".into();
                    return;
                };
                window.clone()
            } else {
                let Ok(id) = editor.next_opening_id() else {
                    desktop.status = "窗户编号已耗尽。".into();
                    return;
                };
                OpeningIntent {
                    id,
                    host: surface.anchor,
                    along,
                    elevation,
                    width: 0.9,
                    height: 1.2,
                    style: 0,
                }
            };
            tools.selected = Some(candidate.id);
            tools.drag = Some(Drag::Window {
                candidate,
                surface,
                revision,
            });
        }
    }
    let mut edit = None;
    let mode = tools.mode;
    match &mut tools.drag {
        Some(Drag::Terrain { candidate, .. }) => {
            let Some(center) = ground(ray, candidate) else {
                return;
            };
            let kind = match mode {
                ContextMode::Raise => BrushKind::Raise,
                ContextMode::Lower => BrushKind::Lower,
                _ => BrushKind::Smooth,
            };
            let dt = world
                .time
                .as_ref()
                .map_or(0.016, |t| t.delta_secs_f64())
                .clamp(0.001, 0.05);
            match candidate.brushed(BrushSample {
                center,
                radius: 1.5,
                amount: 0.6 * dt,
                kind,
            }) {
                Ok(next) => {
                    *candidate = next;
                    edit = Some(ContextEdit::Terrain(Arc::new(candidate.clone())));
                }
                Err(_) => {
                    desktop.status = "地形笔刷参数无效，未提交。".into();
                    tools.clear();
                    return;
                }
            }
        }
        Some(Drag::Window {
            candidate, surface, ..
        }) => {
            if let Some((_, along, elevation)) = wall_hit(ray, surface) {
                candidate.along = along;
                candidate.elevation = elevation;
                edit = Some(ContextEdit::PutOpening(candidate.clone()));
            }
        }
        None => return,
    }
    let revision = match &tools.drag {
        Some(Drag::Terrain { revision, .. }) | Some(Drag::Window { revision, .. }) => *revision,
        _ => return,
    };
    if let Some(edit) = edit {
        if let Ok(candidate) = editor.prepare(EditCommand::Context(edit.clone())) {
            tools.preview_input(garden_bevy::context::input(candidate.snapshot), revision);
            if world.frame.world_finish {
                tools.finish_edit(edit);
                tools.drag = None;
                desktop.status = "候选已保留，等待语义解析后提交。".into();
            }
        } else {
            desktop.status = "候选不合法，未提交。".into();
            if world.frame.world_finish {
                tools.clear();
            }
        }
    } else if world.frame.world_finish {
        tools.clear();
    }
}
fn preview_jobs(
    mut tools: ResMut<ContextTools>,
    editor: Res<EditorState>,
    pool: Res<GenerationPool>,
    slots: Res<GenerationSlots>,
    mut desktop: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    let revision = editor.editor().world_revision();
    let session = editor.editor().session();
    if tools.observed_session.is_some_and(|old| old != session) {
        tools.clear();
        tools.selected = None;
    }
    tools.observed_session = Some(session);
    if tools.active && tools.revision != revision {
        tools.clear();
        desktop.status = "场景已变化，候选已取消，请重新拖动。".into();
    }
    if tools
        .pending_stroke
        .is_some_and(|id| editor.editor().stroke(id).is_some())
    {
        tools.pending_stroke = None;
    }
    if let Some(job) = &mut tools.job
        && let Some((resolved, cache)) = check_ready(&mut job.task)
    {
        let (sequence, job_revision, epoch) = (job.sequence, job.revision, job.epoch);
        tools.job = None;
        // 移动期间展示同一手势最近完成的语义，避免持续移动使所有结果被序号饿死。
        // 提交仍必须匹配最新序号；旧会话或取消后的结果绝不发布。
        if tools.active
            && epoch == tools.epoch
            && job_revision == revision
            && job_revision == tools.revision
        {
            let sleeping = resolved
                .diagnostics
                .iter()
                .filter(|d| matches!(d, garden_generation::context::Diagnostic::Sleeping(_)))
                .count();
            if sleeping > 0 {
                desktop.status = format!("候选中有{sleeping}个窗户休眠；原参数会保留。");
            }
            tools.cache = cache;
            tools.preview = Some(Arc::new(resolved));
            tools.resolved_sequence = Some(sequence);
        }
    }
    if tools.active
        && tools.resolved_sequence == Some(tools.sequence)
        && let Some(edit) = tools.finish.take()
    {
        let decisions = tools.preview.as_ref().unwrap().decisions.clone();
        desktop.submit(
            guarded(
                ContextEdit::Batch(vec![edit, ContextEdit::Decisions(decisions)]),
                tools.revision,
            ),
            &mut inbox,
        );
        tools.preview = None;
        tools.desired = None;
        tools.last_input = None;
        tools.active = false;
        tools.epoch += 1;
    }
    if tools.job.is_none()
        && tools.desired.is_some()
        && let Some(slot) = slots.try_acquire()
    {
        let (sequence, input) = tools.desired.take().unwrap();
        let mut cache = tools.cache.clone();
        tools.job = Some(PreviewJob {
            sequence,
            revision: tools.revision,
            epoch: tools.epoch,
            _slot: slot,
            task: pool.0.spawn(async move {
                let resolved = garden_generation::context::resolve_cached(&input, &mut cache);
                (resolved, cache)
            }),
        });
    }
}
fn overlay(
    tools: Res<ContextTools>,
    editor: Res<EditorState>,
    context: Res<ContextProjection>,
    mut gizmos: Gizmos,
) {
    let cyan = Color::srgb_u8(58, 213, 219);
    let red = Color::srgb_u8(240, 91, 73);
    if let Some(Drag::Terrain { candidate, .. }) = &tools.drag {
        // 低细节候选地形只绘制发生变化的块，不创建正式网格或修改资产。
        for (id, tile) in &candidate.tiles {
            if editor.editor().context().terrain.tiles.get(id) == Some(tile) {
                continue;
            }
            for i in 0..=8 {
                for j in 0..8 {
                    let a = Point {
                        x: id.0 as f64 * 4. + i as f64 * 0.5,
                        z: id.1 as f64 * 4. + j as f64 * 0.5,
                    };
                    for b in [
                        Point {
                            x: a.x,
                            z: a.z + 0.5,
                        },
                        Point {
                            x: a.x + 0.5,
                            z: a.z,
                        },
                    ] {
                        gizmos.line(
                            Vec3::new(a.x as f32, candidate.height(a) as f32 + 0.02, a.z as f32),
                            Vec3::new(b.x as f32, candidate.height(b) as f32 + 0.02, b.z as f32),
                            cyan,
                        );
                    }
                }
            }
        }
    }
    if let Some(Drag::Window {
        candidate, surface, ..
    }) = &tools.drag
    {
        let sleeping = tools.preview.as_ref().is_some_and(|p| {
            p.diagnostics
                .contains(&garden_generation::context::Diagnostic::Sleeping(
                    candidate.id,
                ))
        });
        draw_window(
            &mut gizmos,
            surface,
            candidate,
            if sleeping { red } else { cyan },
        );
    }
    let semantic = tools.preview.as_deref().unwrap_or(&context.semantic);
    for facade in semantic.facades.values() {
        if tools.preview.is_some()
            && context
                .semantic
                .facades
                .get(&facade.surface.anchor)
                .is_some_and(|old| (old.surface.base - facade.surface.base).abs() > 1e-5)
        {
            // 地形候选同时提示建筑的水平抬升，仍只消费派生墙面，不复制地基规则。
            let surface = &facade.surface;
            let at = |p: Point, y: f64| Vec3::new(p.x as f32, y as f32 + 0.02, p.z as f32);
            gizmos.line(
                at(surface.start, surface.base),
                at(surface.end, surface.base),
                cyan,
            );
            gizmos.line(
                at(surface.start, surface.base + surface.height),
                at(surface.end, surface.base + surface.height),
                cyan,
            );
            gizmos.line(
                at(surface.start, surface.base),
                at(surface.start, surface.base + surface.height),
                cyan,
            );
        }
        for assembly in &facade.assemblies {
            for window in &assembly.openings {
                if tools.selected != Some(window.id) && tools.preview.is_none() {
                    continue;
                }
                draw_window(&mut gizmos, &facade.surface, window, cyan);
            }
        }
        if let Some(id) = tools.selected
            && facade.sleeping.contains(&id)
            && let Some(w) = editor.editor().context().openings.get(&id)
        {
            draw_window(&mut gizmos, &facade.surface, w, red);
        }
        if tools.preview.is_some() {
            for door in &facade.doors {
                let w = OpeningIntent {
                    id: OpeningId(0),
                    host: facade.surface.anchor,
                    along: door.along,
                    elevation: door.height / 2.,
                    width: door.width,
                    height: door.height,
                    style: 0,
                };
                draw_window(&mut gizmos, &facade.surface, &w, cyan);
            }
        }
    }
    if tools.preview.is_some() {
        for (id, openings) in &semantic.wall_openings {
            let line = &semantic.linear[id];
            let curve = garden_generation::strokes::Curve::new(&line.points);
            let ground = &editor.editor().context().terrain;
            for opening in openings {
                let mut previous = None;
                for i in 0..=24 {
                    let u = i as f64 / 24.;
                    let along = opening.start + (opening.end - opening.start) * u;
                    let (point, _) = curve.sample(along);
                    let rise = opening.rise * (1. - (2. * u - 1.).powi(2)).max(0.).sqrt();
                    let current = Vec3::new(
                        point.x as f32,
                        (ground.height(point) + line.elevation_offset + rise + 0.02) as f32,
                        point.z as f32,
                    );
                    if let Some(previous) = previous {
                        gizmos.line(previous, current, cyan);
                    }
                    previous = Some(current);
                }
            }
        }
        for node in &semantic.network.nodes {
            if node.sources.len() > 1 {
                gizmos.sphere(
                    Isometry3d::from_translation(Vec3::new(
                        node.point.x as f32,
                        node.height as f32 + 0.08,
                        node.point.z as f32,
                    )),
                    0.08,
                    cyan,
                );
            }
        }
    }
}
fn draw_window(gizmos: &mut Gizmos, surface: &WallSurface, window: &OpeningIntent, color: Color) {
    let length = surface.start.distance(surface.end);
    let dx = (surface.end.x - surface.start.x) / length;
    let dz = (surface.end.z - surface.start.z) / length;
    let at = |a: f64, y: f64| {
        Vec3::new(
            (surface.start.x + dx * a - dz * 0.02) as f32,
            (surface.base + y) as f32,
            (surface.start.z + dz * a + dx * 0.02) as f32,
        )
    };
    let lo = window.along - window.width / 2.;
    let hi = window.along + window.width / 2.;
    let bottom = window.elevation - window.height / 2.;
    let top = window.elevation + window.height / 2.;
    for (a, b) in [
        (at(lo, bottom), at(hi, bottom)),
        (at(hi, bottom), at(hi, top)),
        (at(hi, top), at(lo, top)),
        (at(lo, top), at(lo, bottom)),
    ] {
        gizmos.line(a, b, color);
    }
}
