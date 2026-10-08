//! A real Bevy update loop without GPU/window/platform SDK requirements.
use bevy_app::App;
use garden_application::EditCommand;
use garden_bevy::{
    BuildingProjection, CommandInbox, EditFeedback, GardenPlugin, Operation, PipelineStats,
};
use garden_domain::{BlockId, BuildingEdit, BuildingId, sample_building};
use garden_geometry::Rect;
use std::time::{Duration, Instant};

fn main() {
    let mut app = App::new();
    app.add_plugins(GardenPlugin::default());
    let id = BuildingId::new(1).unwrap();
    let operations = [
        EditCommand::Create(sample_building(id, 3.0, 3.0)),
        EditCommand::Edit {
            building: id,
            edit: BuildingEdit::Resize {
                block: BlockId::new(1).unwrap(),
                footprint: Rect {
                    x: 0.0,
                    z: 0.0,
                    width: 6.0,
                    depth: 6.0,
                },
                height: 6.0,
            },
        },
        EditCommand::Edit {
            building: id,
            edit: BuildingEdit::Resize {
                block: BlockId::new(1).unwrap(),
                footprint: Rect {
                    x: 0.0,
                    z: 0.0,
                    width: 10.0,
                    depth: 6.0,
                },
                height: 12.0,
            },
        },
        EditCommand::Undo,
    ];
    for (step, command) in operations.into_iter().enumerate() {
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Edit(command))
            .unwrap();
        let expected = step as u64 + 1;
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            app.update();
            for outcome in app.world_mut().resource_mut::<EditFeedback>().drain() {
                outcome.result.unwrap();
            }
            if app.world().resource::<PipelineStats>().applied == expected {
                break;
            }
            assert!(Instant::now() < deadline, "pipeline timeout");
            std::thread::sleep(Duration::from_millis(1));
        }
        let world = app.world_mut();
        let projection = world.query::<&BuildingProjection>().single(world).unwrap();
        let block = &projection.target.blocks[0];
        let front = block
            .windows
            .iter()
            .filter(|w| w.key.face == garden_generation::Face::Front)
            .count();
        println!(
            "step {}: width={}m, stories={}, front_windows={}, roof={:?}, revision={}",
            step + 1,
            block.footprint.width,
            block.stories,
            front,
            block.effective_roof,
            projection.ticket.object_revision
        );
    }
    println!("pipeline: {:?}", app.world().resource::<PipelineStats>());
}
