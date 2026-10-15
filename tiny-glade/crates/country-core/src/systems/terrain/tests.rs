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
    let device = crate::rhapsody_contract::GraphicsDevice(0);
    library.add_new::<TerrainHeightSsbo>(&device);
    library.add_new::<VisualTerrainHeightSsbo>(&device);
    library.get_mut::<TerrainHeightSsbo>().unwrap().mapped[0].0 = 0x0002_0001;
    library.get_mut::<VisualTerrainHeightSsbo>().unwrap().mapped[0].0 = 0x0004_0003;
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
    let mut audio_cursor = world.resource::<Events<PlayTerrainSound>>().get_cursor();
    let audio: Vec<_> = audio_cursor
        .read(world.resource::<Events<PlayTerrainSound>>())
        .copied()
        .collect();
    assert_eq!(
        audio
            .iter()
            .map(PlayTerrainSound::native_tag)
            .collect::<Vec<_>>(),
        [3, 2, 0]
    );
    assert!(matches!(
        audio[0],
        PlayTerrainSound::Points { height: 2.5, .. }
    ));
    assert!(matches!(
        audio[1],
        PlayTerrainSound::Height { height: 3.5, .. }
    ));
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
