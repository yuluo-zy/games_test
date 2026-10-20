//! B14092a100/1265B: only position is overwritten, extent remains intact.
use crate::{components::{Ui2dExtent,Ui2DFollowMouse}, resources::mouse_terrain_raycast::CursorPositionSS};
use bevy_ecs::prelude::*;
pub fn ui_2d_follow_mouse(
    mut query: Query<(&mut Ui2dExtent, &Ui2DFollowMouse)>,
    cursor: Res<CursorPositionSS>,
) {
    for (mut extent, _) in &mut query {
        extent.pos = cursor.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_nonempty_follow_preserves_extent_and_unmarked_rows() {
        let mut world = World::new();
        world.insert_resource(CursorPositionSS(glam::Vec2::new(12., 34.)));
        let ids: Vec<_> = (0..3)
            .map(|_| {
                world
                    .spawn((
                        Ui2DFollowMouse,
                        Ui2dExtent {
                            pos: glam::Vec2::new(1., 2.),
                            size: glam::Vec2::new(30., 40.),
                        },
                    ))
                    .id()
            })
            .collect();
        let excluded = world
            .spawn(Ui2dExtent {
                pos: glam::Vec2::ONE,
                size: glam::Vec2::splat(5.),
            })
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_2d_follow_mouse);
        schedule.run(&mut world);
        for id in ids {
            let extent = world.get::<Ui2dExtent>(id).unwrap();
            assert_eq!(extent.pos, glam::Vec2::new(12., 34.));
            assert_eq!(extent.size, glam::Vec2::new(30., 40.));
        }
        assert_eq!(
            world.get::<Ui2dExtent>(excluded).unwrap().pos,
            glam::Vec2::ONE
        );
    }
}
