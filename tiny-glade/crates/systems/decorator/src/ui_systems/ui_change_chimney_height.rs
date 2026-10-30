//! B `ui_systems/ui_change_chimney_height.rs` — 烟囱高度调整（0x1407ae2e0，O）。
//!
//! 原 1407ae2e0 用 dispatcher 的真实 widget/id、蓝图 +128 的
//! ChimneyAssemblyParams 和跨帧 Local；当前以 PrevDecoratorBlueprints 接线，
//! 原资源的 nominal owner 仍待 typed SystemParam 证据独立确认。Started 保存投影锚点；Held/Finished
//! 以 [-1.5,11] 投影竖直线，发送装饰 change 与 PlayDecoratorSoundCmd。
use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use glam::Vec3;

use crate::apply_decorator_op::{
    apply_decorator_op, DecoratorChangePatch, DecoratorOp,
};
use crate::deferred_decorator_history::DeferredDecoratorEditHistory;
use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;

/// `Local<EditingState>`（ui_change_chimney_height 域；I）。
#[derive(Default)]
pub struct EditingState {
    pub origin: Vec3,
    pub grab: f32,
}

/// B 0x1407ae2e0（O 结构）：烟囱高度调整主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_change_chimney_height(
    ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<
        country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    >,
    archival: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
    previous: Res<crate::blueprint::PrevDecoratorBlueprints>,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    window: Res<country_core::resources::window::WindowSize>,
    camera: Res<country_core::systems::camera_rig::MainCamera>,
    time: Res<country_core::resources::time::GameTime>,
    mut audio: EventWriter<country_core::systems::audio::PlayDecoratorSoundCmd>,
    mut history: ResMut<DeferredDecoratorEditHistory>,
    mut local: Local<EditingState>,
) {
    let Some(target) = dispatcher.height_adjustment() else { return };
    let context = ui.widget_context(target.widget);
    if !context.is_active() { return }
    let decorator_id = target.decorator_id;
    let Some(params) = previous.map.get(&decorator_id.0).and_then(|bp| bp.chimney_assembly.as_deref()) else { return };
    let dragged = context.dragged();
    if dragged < 2 {
        history.pending = true;
    }
    let dst = storage.lookup_storage(decorator_id).expect("chimney decorator storage missing");
    let _info = *storage.get(dst).and_then(|bucket| bucket.get(decorator_id)).expect("chimney decorator info missing");
    let v = |i: usize| f32::from_bits(params.words[i]);
    let project = |origin: Vec3| {
        use country_core::utils::project_cursor_onto_ray::{ProportionalScreenSpace, ProportionalScreenSpaceCoord};
        let screen = ProportionalScreenSpace::from_window_size(window.as_dims());
        ProportionalScreenSpaceCoord(screen.transform_screen_space(cursor.0.x, cursor.0.y))
            .project_onto_ray(&screen, &camera.read_matrix(0x80), &camera.read_matrix(0),
                utils::geometry::Ray { origin, direction: Vec3::Y }, [-1.5, 11.0])
    };
    let phase = if dragged.wrapping_sub(2) < 3 {
        dragged.wrapping_sub(2)
    } else {
        3
    };
    match phase {
        // Held/Finished: apply change then send the sound event.
        2 | 3 => {
            let projected = project(local.origin);
            let delta = (projected.y - local.origin.y) - local.grab;
            let pos = Vec3::new(local.origin.x + 0.0 * delta, local.origin.y + delta, local.origin.z + 0.0 * delta);
            let patch = DecoratorChangePatch { id: decorator_id, destination: None, rank: None,
                ty: None, word_2c_delta: None, seed_demo_mode: false,
                extra_low: Some(delta.to_bits()), extra_high: None, extra_byte: None,
                event_position: pos, event_position_flag: 0 };
            let op = DecoratorOp::from_change(dst, patch);
            // The native apply checks immutability before taking the first
            // storage/collider snapshot. The generic apply below then finds
            // that same first snapshot and does not replace it.
            assert!(!storage.immutable, "assertion failed: !self.immutable");
            archivist.archive_current_decorators_at_with_resources(dst, &storage, &archival, "move preview src");
            apply_decorator_op(
                &op,
                u8::from(dragged > 1),
                dst,
                &mut storage,
                Some(&mut *archivist),
                Some(&mut *history),
                &events,
            );
            audio.write(country_core::systems::audio::PlayDecoratorSoundCmd {
                kind: 0x12, selector: 0, ratio: (v(5) - (delta + v(9))).abs() / time.delta,
                position_context: 0, position: pos, flag: false,
            });
        }
        // Started: preserve the anchor and cursor offset across frames.
        1 => {
            show_signifiers(params, decorator_id, &mut stream);
            local.origin = Vec3::new(v(2), v(9), v(3));
            local.grab = project(local.origin).y - v(5);
        }
        // 悬停（O：bVar20==0 → 仅 show）。
        _ => {
            show_signifiers(params, decorator_id, &mut stream);
        }
    }
}

/// Original 1407ae910 factories and 1407aecc0/1407aeea0 builders.
pub fn show_signifiers(
    params: &crate::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams,
    id: DecoratorId,
    stream: &mut UiSignifierStream,
) {
    use country_core::systems::ui::signifiers::{SignifierEntityBuilder, SignifierFactory};
    let value = |i: usize| f32::from_bits(params.words[i]);
    let top = value(5);
    let arrow_position = Vec3::new(value(2), top + 0.3, value(3));
    let sphere_position = Vec3::new(value(2), top + -0.5, value(3));
    let rotation = utils::xz_rotation_bad(glam::Vec2::new(-value(7), value(6)));
    let arrow_name = format!("chimney_height_{}_arrow_{arrow_position:?}", id.0);
    let arrow = SignifierFactory::from_closure(&arrow_name, std::sync::Arc::new(move |meshes, shaders, commands| {
        vec![SignifierEntityBuilder::from_mesh_name("brush_arrow_flat_single_up")
            .animation_duration(0.2).rotation(rotation).scale(1.5).position(arrow_position)
            .build(meshes, shaders, commands)]
    }));
    stream.write_factory(&arrow);
    let sphere_name = format!("height_gizmo_{}{sphere_position:?}", id.0);
    let sphere = SignifierFactory::from_closure(&sphere_name, std::sync::Arc::new(move |meshes, shaders, commands| {
        vec![SignifierEntityBuilder::from_mesh_name("temp_sphere")
            .position(sphere_position).scale(0.5).animation_duration(0.1)
            .build(meshes, shaders, commands)]
    }));
    stream.write_factory(&sphere);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use country_core::{
        resources::{game_clock::GameClock, mouse_terrain_raycast::CursorPositionSS,
            walls::{decorator::{ChimneyExtra, DecoratorExtraInfo, DecoratorInfo, DecoratorType, DeferredDecoratorOpEvents},
                decorator_archivist::LiveDecoratorArchivist, decorator_storage::DecoratorStorageAddr,
                decorator_collision::DecoratorPhysicsColliders},
            window::WindowSize},
        systems::{audio::PlayDecoratorSoundCmd, camera_rig::MainCamera,
            collision::world::{RaycastWorldResult, RaycastWorld, RaycastGeoSource, RaycastMeshData, upsert_collider},
            ui::resources::ui_state::{UiInteraction, WidgetId}},
    };
    use crate::{blueprint::{DecoratorBlueprint, PrevDecoratorBlueprints},
        deferred_decorator_history::DeferredDecoratorRecord,
        decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams,
        ui_systems::ui_decorator_affordance_dispatch::{DecoratorAffordanceInteraction, DecoratorTargetAffordance}};

    #[test]
    fn drag_then_finish_updates_chimney_history_and_audio() {
        let mut world = World::new();
        let id = DecoratorId(71);
        let widget = WidgetId(1234);
        let addr = DecoratorStorageAddr::Terrain;
        let mut info = DecoratorInfo::default();
        info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra { f: 0.0, smoke: 1 }));
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(addr).insert(id, info);
        storage.decorator_to_storage.insert(id, addr);
        world.insert_resource(storage);
        let mut raycast = RaycastWorld::new();
        let mut colliders = DecoratorPhysicsColliders::default();
        let mesh_a = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::Y));
        let mesh_b = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::X));
        upsert_collider(&mut raycast, &mut colliders.first, id,
            RaycastGeoSource::decorator(id, addr), mesh_a.clone(), Vec3::new(3.0, 4.0, 5.0), glam::Quat::IDENTITY);
        upsert_collider(&mut raycast, &mut colliders.second, id,
            RaycastGeoSource::decorator_interaction(id, addr, 0), mesh_b.clone(), Vec3::new(-2.0, 1.0, 8.0), glam::Quat::IDENTITY);
        world.insert_resource(raycast);
        world.insert_resource(colliders);
        let mut words = [0; 10];
        for (i, value) in [(2, 2.0f32), (3, 3.0), (5, 6.0), (9, 4.0)] { words[i] = value.to_bits(); }
        let mut previous = PrevDecoratorBlueprints::default();
        previous.map.insert(id.0, Arc::new(DecoratorBlueprint { chimney_assembly: Some(Box::new(ChimneyAssemblyParams { words })), ..Default::default() }));
        world.insert_resource(previous);
        let target = DecoratorTargetAffordance { widget, decorator_id: id, ty: DecoratorType(59), hit: RaycastWorldResult::miss() };
        world.insert_resource(DecoratorAffordanceDispatcher { state: 4, payload: 0, ty: 59, interaction: Some(DecoratorAffordanceInteraction::ChangeHeight(target)) });
        let mut ui = UiState::new(UiInteraction { native_tag: 2, native_phase: 3, widget });
        ui.set_intersection_tag(0);
        world.insert_resource(ui);
        world.insert_resource(CursorPositionSS(glam::Vec2::new(50.0, 50.0)));
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(MainCamera::new(1.0));
        world.insert_resource(GameClock { delta: 0.5, ..Default::default() });
        world.init_resource::<Events<PlayDecoratorSoundCmd>>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<DeferredDecoratorEditHistory>();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_change_chimney_height);
        schedule.run(&mut world);
        assert!(world.resource::<DeferredDecoratorEditHistory>().records.is_empty());
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let names: Vec<_> = entries.iter().map(|entry| match entry {
            country_core::systems::ui::signifiers::UiSignifierEntry::Factory(factory) => factory.name().unwrap(),
            _ => panic!("chimney signifier must retain its factory"),
        }).collect();
        assert_eq!(names, ["chimney_height_71_arrow_Vec3(2.0, 6.3, 3.0)", "height_gizmo_71Vec3(2.0, 5.5, 3.0)"]);
        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 0, widget });
        schedule.run(&mut world);
        let actual = world.resource::<DecoratorStorage>().get(addr).unwrap().get(id).unwrap().extra().as_chimney();
        assert_eq!(actual.f, 2.0);
        assert_eq!(actual.smoke, 1);
        let archive = world.resource::<LiveDecoratorArchivist>();
        assert_eq!(archive.collision_a[&id.0].0.translation, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(archive.collision_b[&id.0].0.translation, Vec3::new(-2.0, 1.0, 8.0));
        assert!(Arc::ptr_eq(&archive.collision_a[&id.0].1.data, &mesh_a));
        assert!(Arc::ptr_eq(&archive.collision_b[&id.0].1.data, &mesh_b));
        let history = world.resource::<DeferredDecoratorEditHistory>();
        assert!(history.pending);
        assert_eq!(history.records.len(), 1);
        assert_eq!(history.records[0].kind, DeferredDecoratorRecord::KIND_CHANGE);
        assert_eq!(world.resource::<DeferredDecoratorOpEvents>().with_events(|events| events.len()), 1);
        let events = world.resource::<Events<PlayDecoratorSoundCmd>>();
        let commands: Vec<_> = events.get_cursor().read(events).copied().collect();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].kind, 0x12);
        assert_eq!(commands[0].position, Vec3::new(2.0, 6.0, 3.0));
        assert_eq!(commands[0].ratio, 0.0);
    }
}
