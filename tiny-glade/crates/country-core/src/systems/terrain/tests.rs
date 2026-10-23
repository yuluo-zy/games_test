use super::*;
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{
        terrain_editing::{StrokeId, TerrainEditingData},
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo},
        time::GameTime,
    },
};
use delayed_terrain_changed_msg::{OnTerrainChange, ScheduledTerrainReadback};
use draw_terrain_stroke::EvTerrainStrokeChanged;
use edit_terrain_stroke::{EditTerrainStrokeCmd, SetTerrainStrokePtsCmd};
use std::sync::atomic::Ordering;
use tg_terrain::edit::{Adjustment, EditCommand};

#[test]
fn edits_coalesced_changes_and_delayed_readback_use_the_actual_owners() {
    let mut world = World::new();
    register_terrain_events(&mut world);
    let mut terrain = tg_terrain::stroke::TerrainEditingData::default();
    assert_eq!(terrain.new_stroke([0.0; 3]).unwrap(), Some(StrokeId(0)));
    world.insert_resource(TerrainEditingData(terrain));
    world.insert_resource(GameTime {
        delta: 0.2,
        ..Default::default()
    });
    world.insert_resource(ScheduledTerrainReadback::default());
    world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
    world.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
    let mut library = AssetSsboLibrary::new();
    // Existing cfg(test) adapter supplies mapped CPU bytes only; no GPU claim.
    let device = crate::render::test_device::TestDevice(0);
    library.add_new::<TerrainHeightSsbo>(&device);
    library.add_new::<VisualTerrainHeightSsbo>(&device);
    library.get_mut::<TerrainHeightSsbo>().unwrap().with_cpu_write(|samples| samples[0].0 = 0x0002_0001).unwrap();
    library.get_mut::<VisualTerrainHeightSsbo>().unwrap().with_cpu_write(|samples| samples[0].0 = 0x0004_0003).unwrap();
    world.insert_resource(library);
    world
        .resource_mut::<Events<SetTerrainStrokePtsCmd>>()
        .send(SetTerrainStrokePtsCmd {
            id: StrokeId(0),
            points: vec![[3.0, 4.0, 0.0]],
            position: Vec3::new(9.0, 8.0, 7.0),
        });
    world
        .resource_mut::<Events<EditTerrainStrokeCmd>>()
        .send(EditTerrainStrokeCmd(EditCommand {
            id: StrokeId(0),
            falloff: Adjustment::Keep,
            height: Adjustment::Delta(1.0),
            position: [1.0, 2.0, 3.0],
        }));
    // Missing IDs log and skip without cancelling the following real changes.
    world
        .resource_mut::<Events<EditTerrainStrokeCmd>>()
        .send(EditTerrainStrokeCmd(EditCommand {
            id: StrokeId(999),
            falloff: Adjustment::Keep,
            height: Adjustment::Keep,
            position: [0.0; 3],
        }));
    world
        .resource_mut::<Events<EvTerrainStrokeChanged>>()
        .send_batch([
            EvTerrainStrokeChanged(StrokeId(0)),
            EvTerrainStrokeChanged(StrokeId(0)),
        ]);
    let mut schedule = Schedule::default();
    add_terrain_cpu_systems(&mut schedule);
    schedule.run(&mut world);
    let stroke = &world.resource::<TerrainEditingData>().strokes[&StrokeId(0)];
    assert_eq!(stroke.points, [[3.0, 4.0, 0.0]]);
    assert_eq!(stroke.height_ws, 3.5);
    assert_eq!(world.resource::<Events<RecalculateHeightMapCmd>>().len(), 3);
    assert_eq!(
        world
            .resource::<ScheduledTerrainReadback>()
            .pending
            .load(Ordering::SeqCst),
        3
    );
    let events = world.resource::<Events<PlayTerrainSound>>();
    let mut reader = events.get_cursor();
    let sounds: Vec<_> = reader.read(events).copied().collect();
    assert_eq!(
        sounds
            .iter()
            .map(PlayTerrainSound::native_tag)
            .collect::<Vec<_>>(),
        [3, 2, 0]
    );
    assert!(matches!(sounds[0],PlayTerrainSound::Points{magnitude,..}if magnitude>0.0));
    assert!(matches!(sounds[1],PlayTerrainSound::Height{height,..}if height==3.5));
    // Simulate only the existing renderer owner's issued-copy notification.
    world
        .resource_mut::<ScheduledTerrainReadback>()
        .on_gpu_copy_issued(17);
    for _ in 0..2 {
        schedule.run(&mut world);
        assert!(world.resource::<Events<OnTerrainChange>>().is_empty());
    }
    schedule.run(&mut world);
    assert_eq!(
        &world
            .resource::<TerrainHeightsData<TerrainHeightSsbo>>()
            .heights()[..2],
        [1, 2]
    );
    assert_eq!(
        &world
            .resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>()
            .heights()[..2],
        [3, 4]
    );
    assert_eq!(world.resource::<Events<OnTerrainChange>>().len(), 1);
    assert_eq!(world.resource::<Events<RecalculateHeightMapCmd>>().len(), 3);
}

#[test]
fn ui_draw_canvas_gate_and_prev_wrap_match_fresh_state_machine() {
    use crate::{
        resources::cursor_terrain_raycast::CursorTerrainRaycast,
        resources::glade_border::{BorderShape, GladeBorder},
        systems::mode_manager::AppMode,
        systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId},
    };
    use utils::geometry::Circle2d;

    let mut world = World::new();
    register_terrain_events(&mut world);
    world.insert_resource(AppMode::Terrain);
    world.insert_resource(CursorTerrainRaycast {
        tag: 0,
        ..Default::default()
    });
    world.insert_resource(crate::resources::world_raster_defs::WaterRaster::new(
        crate::resources::world_raster::DirtyTilePublisher::unique(),
    ));
    world.insert_resource(TerrainEditingData(
        tg_terrain::stroke::TerrainEditingData::default(),
    ));
    // 大圆边界 → is_shape_inside = true。
    world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
        glam::Vec2::ZERO,
        100.0,
    ))));
    let draw_id = WidgetId::clearing_canvas("terrain_draw");
    world.insert_resource(UiState::new(UiInteraction {
        native_tag: 2,
        native_phase: 3,
        widget: draw_id,
    }));
    let mut schedule = Schedule::default();
    ui_draw_terrain::register_test_buffers(&mut world);
    schedule.add_systems(ui_draw_terrain::ui_draw_terrain);
    schedule.run(&mut world);
    // height=0 → canvas 1（terrain_deny_draw）不注册；canvas 2（terrain_draw）无条件注册。
    assert!(
        world
            .resource::<UiState>()
            .clearing_canvas_params(WidgetId::clearing_canvas("terrain_deny_draw"))
            .is_none()
    );
    assert!(
        world
            .resource::<UiState>()
            .clearing_canvas_params(draw_id)
            .is_some()
    );
    // subtype=3 → uVar10=1；prev=0 → (prev-1)=255 > 1 → state = inside = 1（u8 回绕）。
    assert_eq!(
        world.resource::<ui_draw_terrain::TerrainDrawState>().state,
        1
    );
    schedule.run(&mut world);
    // prev=1 → (prev-1)=0 不 >1 → state=2。
    assert_eq!(
        world.resource::<ui_draw_terrain::TerrainDrawState>().state,
        2
    );
    // subtype=2 → uVar10=0 → state=0（else 分支 uVar10==0）。
    world
        .resource_mut::<UiState>()
        .replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: 2,
            widget: draw_id,
        });
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<ui_draw_terrain::TerrainDrawState>().state,
        0
    );
}

#[test]
fn ui_edit_widget_type_byte_is_three_times_id_mismatch_plus_four() {
    use crate::{
        resources::cursor_terrain_raycast::CursorTerrainRaycast,
        systems::mode_manager::AppMode,
        systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId},
    };

    let mut world = World::new();
    register_terrain_events(&mut world);
    world.insert_resource(AppMode::Terrain);
    // cursor.tag == 0 → 按 (hovered_id != stroke_id)*3+4 计算。
    world.insert_resource(CursorTerrainRaycast::default());
    world.insert_resource(terrain_hover_idx::HoveredTerrainStrokeId { tag: 1, id: 1 });
    world.insert_resource(UiState::new(UiInteraction {
        native_tag: 0,
        native_phase: 0,
        widget: WidgetId(0),
    }));
    let mut editing = tg_terrain::stroke::TerrainEditingData::default();
    editing.new_stroke([0.0; 3]).unwrap();
    editing.new_stroke([1.0; 3]).unwrap();
    world.insert_resource(TerrainEditingData(editing));
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_edit_terrain::ui_edit_terrain);
    schedule.run(&mut world);
    let state = world.resource::<UiState>();
    let same = state
        .widget(WidgetId::from_name("terrain_edit_1__canvas"))
        .expect("hovered stroke widget");
    let other = state
        .widget(WidgetId::from_name("terrain_edit_0__canvas"))
        .expect("other stroke widget");
    assert_eq!(same.volume.tag, 4); // hovered id == stroke id
    assert_eq!(other.volume.tag, 7); // (0 != 1)*3 + 4
}

#[test]
fn rock_mask_reads_u32_word_bit_and_panics_out_of_bounds() {
    use crate::resources::terrain_rock_mask::sample_at_world_xz;
    // x/z 都 clamp 到 0 → index 0；位 0。
    let mut mask = vec![0u32; 1];
    mask[0] = 1 << 0;
    assert_eq!(sample_at_world_xz(&mask, -100.0, -100.0), 1);
    mask[0] = 1 << 5;
    assert_eq!(sample_at_world_xz(&mask, -100.0, -100.0), 0);
    // z=0 → zi=256；index = 256<<9，word = 4096；位 0。
    let mut wide = vec![0u32; 4097];
    wide[4096] = 1;
    assert_eq!(sample_at_world_xz(&wide, -100.0, 0.0), 1);
    // 越界 → panic_bounds_check（不是静默 0）。
    assert!(std::panic::catch_unwind(|| sample_at_world_xz(&[0u32; 1], 100.0, 100.0)).is_err());
}

#[test]
fn terrain_focus_mode_no_stroke_path_writes_window_focus_once() {
    use crate::{
        systems::mode_manager::{AppMode, EditSubject},
        systems::ui::signifiers::signifier_create::UiSignifierStream,
    };
    let mut world = World::new();
    register_terrain_events(&mut world);
    world.insert_resource(AppMode::Editing(EditSubject::Terrain(0)));
    world.insert_resource(ui_terrain_focus_mode::TerrainFocusRaycast {
        tag: 0,
        hit_pos: glam::Vec3::new(10.0, 0.0, 0.0),
        camera_pos: glam::Vec3::ZERO,
    });
    world.insert_resource(ui_terrain_focus_mode::TerrainFocusHover {
        tag: 1,
        pos: glam::Vec3::new(1.0, 0.0, 0.0),
    });
    world.insert_resource(ui_terrain_focus_mode::TerrainFocusSelection { tag: 1, value: 7 });
    world.insert_resource(TerrainEditingData(
        tg_terrain::stroke::TerrainEditingData::default(),
    ));
    world.insert_resource(UiSignifierStream::default());
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_terrain_focus_mode::ui_terrain_focus_mode);
    schedule.run(&mut world);
    let triggers = world.resource::<Events<ui_terrain_focus_mode::TerrainFocusTrigger>>();
    assert_eq!(triggers.len(), 1);
}

#[test]
fn busy_terrain_readback_retries_and_publishes_both_heights_atomically() {
    use crate::render::{ssbo::{MappedStorageBuffer, StorageElement}, test_device::TestReadbackAccess};
    use std::sync::Arc;
    let authority_access = Arc::new(TestReadbackAccess::default());
    let visual_access = Arc::new(TestReadbackAccess::default());
    authority_access.set_ready(true);
    let library = AssetSsboLibrary::new();
    library.register::<TerrainHeightSsbo>(MappedStorageBuffer::from_mapping_with_access(
        Box::leak(vec![TerrainHeightSsbo(0x0002_0001); TerrainHeightSsbo::BUFFER_LEN].into_boxed_slice()),
        Default::default(), Some(authority_access.clone())));
    library.register::<VisualTerrainHeightSsbo>(MappedStorageBuffer::from_mapping_with_access(
        Box::leak(vec![VisualTerrainHeightSsbo(0x0004_0003); VisualTerrainHeightSsbo::BUFFER_LEN].into_boxed_slice()),
        Default::default(), Some(visual_access.clone())));
    let mut world = World::new();
    world.insert_resource(library);
    world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
    world.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
    world.insert_resource(ScheduledTerrainReadback::default());
    world.insert_resource(Events::<OnTerrainChange>::default());
    world.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(1);
    let mut schedule = Schedule::default();
    schedule.add_systems(delayed_terrain_changed_msg::delayed_terrain_changed_msg);
    for _ in 0..5 { schedule.run(&mut world); }
    assert!(world.resource::<Events<OnTerrainChange>>().is_empty());
    assert_eq!(world.resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights()[0], 0);
    assert_eq!(world.resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>().heights()[0], 0);
    assert_eq!(visual_access.reads(), 0);
    // The ready half changes before the other half finishes. The retry must
    // resample it, rather than combine values cached in different Updates.
    world.resource_mut::<AssetSsboLibrary>().get_mut::<TerrainHeightSsbo>().unwrap()
        .with_cpu_write(|samples| samples[0].0 = 0x0006_0005).unwrap();
    visual_access.set_ready(true);
    schedule.run(&mut world);
    assert_eq!(&world.resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights()[..2], &[5, 6]);
    assert_eq!(&world.resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>().heights()[..2], &[3, 4]);
    assert_eq!(world.resource::<Events<OnTerrainChange>>().len(), 1);
    schedule.run(&mut world);
    assert_eq!(world.resource::<Events<OnTerrainChange>>().len(), 1);
}
