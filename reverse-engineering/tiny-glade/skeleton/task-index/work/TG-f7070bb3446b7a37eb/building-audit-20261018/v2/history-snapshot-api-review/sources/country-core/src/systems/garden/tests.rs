use super::*;
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::active_raster_stroke::ActiveRasterStrokesStorage;
use crate::resources::brush_sizes::BrushSizes;
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::glade_border::BorderShape;
use crate::resources::glade_border::DisplayOutOfBorder;
use crate::resources::glade_border::FeedbackInputOutOfBorder;
use crate::resources::glade_border::GladeBorder;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::walls::decorator_archivist::DecoratorArchivist;
use crate::resources::walls::decorator_archivist::DecoratorCullPolicy;
use crate::resources::walls::EraserSizeMultiplier;
use crate::resources::world_raster::DirtyTilePublisher;
use crate::resources::world_raster_defs::GardenRaster;
use crate::resources::world_raster_defs::GardenRasterStagingSsbo;
use crate::systems::mode_manager::AppMode;
use crate::systems::ui::resources::ui_state::UiInteraction;
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::resources::ui_state::WidgetId;
use glam::Vec2;
use std::collections::{HashMap, HashSet};

#[test]
fn garden_drag_border_resume_commit_records_the_live_raster_once() {
    let mut world = World::new();
    register_garden_events(&mut world);
    world.insert_resource(AppMode::Garden);
    let widget = WidgetId::clearing_canvas("garden");
    world.insert_resource(UiState::new(UiInteraction {
        native_tag: 2,
        native_phase: 3,
        widget,
    }));
    world.insert_resource(BrushSizes {
        path: 0.8,
        water: 2.0,
        garden: 3.0,
    });
    world.insert_resource(EraserSizeMultiplier(1.0));
    world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
        Vec2::ZERO,
        10.0,
    ))));
    let mut cursor = CursorTerrainRaycast::new();
    cursor.update(Some(Vec3::new(-2.0, 0.0, 0.0)));
    world.insert_resource(cursor);
    world.insert_resource(ActiveRasterStrokesStorage::default());
    world.insert_resource(DecoratorArchivist::<()> {
        walls: HashMap::new(),
        special1: None,
        special2: None,
        collision_a: HashMap::new(),
        collision_b: HashMap::new(),
        culling: DecoratorCullPolicy {
            allowed_ids: HashSet::new(),
            allow_any: false,
        },
    });
    world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
    // Existing cfg(test) adapter allocates mapped host bytes. This verifies the
    // real staging writes, not a graphics driver or GPU execution.
    let library = AssetSsboLibrary::new();
    library.add_new::<GardenRasterStagingSsbo>(&crate::render::test_device::TestDevice(0));
    world.insert_resource(library);
    let mut history: History = History::from_observed(ObservedHistoryLedger {
        entries: vec![],
        cursor: -1,
        undo_anchor: -1,
        resolved_snapshots: vec![],
        observed_count_58: 0,
        mutation_nonce: 0,
    });
    history.start_input_recording(17).unwrap();
    world.insert_resource(history);
    let mut schedule = Schedule::default();
    add_garden_cpu_systems::<(), ()>(&mut schedule);
    schedule.run(&mut world); // Begin.
    let canvas = world
        .resource::<UiState>()
        .clearing_canvas_params(widget)
        .unwrap();
    assert_eq!((canvas.argument_79, canvas.layer), (5, 3));
    assert_eq!(canvas.extent, Vec2::splat(130.0));
    assert!(world.resource::<DecoratorArchivist<()>>().culling.allow_any);

    world
        .resource_mut::<UiState>()
        .replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: 4,
            widget,
        });
    world
        .resource_mut::<CursorTerrainRaycast>()
        .update(Some(Vec3::new(20.0, 0.0, 0.0)));
    schedule.run(&mut world); // Outside: stays in extension state without drawing.
    assert_eq!(world.resource::<Events<DisplayOutOfBorder>>().len(), 1);
    assert_eq!(
        world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
        1
    );
    assert!(world.resource::<Events<GardenDrawEvent>>().is_empty());
    world
        .resource_mut::<CursorTerrainRaycast>()
        .update(Some(Vec3::new(2.0, 0.0, 0.0)));
    schedule.run(&mut world); // Reenter: resumes the existing stroke.
    assert_eq!(world.resource::<Events<GardenDrawEvent>>().len(), 1);
    assert_eq!(
        world.resource::<Events<GardenPinnedClutterCircle>>().len(),
        1
    );
    assert!(world
        .resource::<GardenRaster>()
        .scratch()
        .iter()
        .any(|&value| value != 0));
    assert!(world
        .resource::<AssetSsboLibrary>()
        .get::<GardenRasterStagingSsbo>()
        .unwrap()
        .mapped
        .iter()
        .any(|value| value.0 != 0));
    world
        .resource_mut::<UiState>()
        .replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: 5,
            widget,
        });
    schedule.run(&mut world); // Commit; the same frame's record consumer takes slot 5.
    assert_eq!(
        world.resource::<History>().pending().unwrap().edits.len(),
        1
    );
    let EditType::Garden(stroke) = &world.resource::<History>().pending().unwrap().edits[0] else {
        panic!("wrong edit owner")
    };
    assert_eq!(
        (stroke.flags, stroke.strength, stroke.radius, stroke.draw),
        (1, 0.5, 3.0, true)
    );
    assert!(stroke.points.len() >= 2);
    assert!(world
        .resource::<GardenRaster>()
        .composite()
        .iter()
        .any(|&value| value != 0));
    assert!(world.resource::<ActiveRasterStrokesStorage>()[5]
        .resampled()
        .is_empty());
    schedule.run(&mut world); // EventReader must not rerecord the old notification.
    assert_eq!(
        world.resource::<History>().pending().unwrap().edits.len(),
        1
    );
    assert!(world
        .resource::<Events<crate::systems::history::recording::HistoryRecordingFailed>>()
        .is_empty());
}
