//! B AdjustWallHeightCmd transport into the actual InnerWalls authority.
use crate::{
    recovered_building::wall_height_command::{
        AdjustWallHeightRequest, ResolvedWallHeightClamp, WallHeightCommandError,
        WallHeightFeedback, apply_resolved_height_command,
    },
    resources::{
        roofs::Roof,
        time::GameTime,
        walls::{InnerWalls, WallId, public_walls::PublicWalls},
    },
    systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain,
};
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Copy, Debug)]
pub struct AdjustWallHeightCmd(pub AdjustWallHeightRequest);
#[derive(Event, Clone, Copy, Debug)]
pub struct WallHeightAudio(pub WallHeightFeedback);
#[derive(Event, Clone, Copy, Debug)]
pub struct WallHeightCommandFailed {
    pub wall: WallId,
    pub error: WallHeightCommandError,
}

pub fn register_wall_height_events(world: &mut World) {
    world.init_resource::<Events<AdjustWallHeightCmd>>();
    world.init_resource::<Events<WallHeightAudio>>();
    world.init_resource::<Events<WallHeightCommandFailed>>();
}

/// Shared command algorithm retains ordered comparison, equal-height skipping,
/// source +65 bounds, terrain anchor and the original repeated authority write.
pub fn adjust_wall_height(
    mut requests: EventReader<AdjustWallHeightCmd>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    terrain: Res<WallPublicationTerrain>,
    time: Res<GameTime>,
    mut audio: EventWriter<WallHeightAudio>,
    mut failures: EventWriter<WallHeightCommandFailed>,
) {
    for event in requests.read() {
        let request = &event.0;
        // The native no-op scalar variant does not look up a source.
        if request.mode == 0 {
            continue;
        }
        let wall = WallId(request.wall.0);
        let Some(state) = inner.get(wall) else {
            failures.write(WallHeightCommandFailed {
                wall,
                error: WallHeightCommandError::MissingAuthority,
            });
            continue;
        };
        let clamp = public.get(wall).map(|state| ResolvedWallHeightClamp {
            roof_height_01: public_roof_height(&public, wall, &roofs),
            special_flag: state.authority.source_flag_65(),
        });
        let mut height = state.height();
        match apply_resolved_height_command(
            request,
            &mut height,
            state.source().curve_kind_tag,
            &state.source().curve,
            &terrain.0,
            clamp,
            time.delta,
        ) {
            Ok(Some(result)) => {
                for update in result.authority_updates {
                    inner.update_wall_state_height(wall, update.requested_height);
                }
                audio.write(WallHeightAudio(result.feedback));
            }
            Ok(None) => {}
            Err(error) => {
                failures.write(WallHeightCommandFailed { wall, error });
            }
        }
    }
}

fn public_roof_height(public: &PublicWalls, wall: WallId, roofs: &Query<&Roof>) -> Option<f32> {
    public
        .roof_entity(wall)
        .and_then(|entity| roofs.get(entity).ok())
        .map(Roof::height_01)
}
