//! B1409a2290: the three independent command streams, authority writes,
//! exact compound history grouping and optional wall-style UI sound.
//! PDB proves the final writer is PlayWallUiSound, not material audio.
use crate::{
    resources::{
        history::wall_edit::{
            SavedWallStyle, WallCompoundHistoryEdit, WallHistoryEdit, WallTerrainPolicy,
            WallValueEdit,
        },
        walls::{InnerWalls, public_walls::PublicWalls},
    },
    systems::{
        audio::PlayWallUiSound,
        wall::{
            SetFreehandHeightTypeCmd, SetPlasterPeelAmountCmd, SetWallStyleCmd,
            history_recording::RecordWallHistory,
        },
    },
};
use bevy_ecs::prelude::*;
use std::sync::Arc;
fn policy(value: u8) -> WallTerrainPolicy {
    match value {
        0 => WallTerrainPolicy::FollowTerrain,
        1 => WallTerrainPolicy::ConstantMaxHeight,
        _ => panic!("invalid wall terrain policy {value}"),
    }
}
fn style(value: u8) -> SavedWallStyle {
    match value {
        0 => SavedWallStyle::Stone,
        1 => SavedWallStyle::StoneWithSkirts,
        2 => SavedWallStyle::HalfTimbered,
        _ => panic!("invalid saved wall style {value}"),
    }
}
fn record(records: &mut EventWriter<RecordWallHistory>, edits: Vec<WallHistoryEdit>) {
    records.write(RecordWallHistory(Arc::new(WallCompoundHistoryEdit {
        groups: edits.into_iter().map(|edit| vec![edit]).collect(),
    })));
}
/// Native samples the original mapped Curve3, before path-intersection edits;
/// its cached U is retained. Missing public rows produce the original origin
/// fallback, while a present invalid curve remains a panic.
fn audio_position(public: &PublicWalls, wall: crate::resources::walls::WallId) -> glam::Vec3 {
    let Some(wall) = public.get(wall) else {
        return glam::Vec3::ZERO;
    };
    let points = &wall.geometry.outline;
    let us = &wall.geometry.outline_u;
    assert!(points.len() >= 2 && us.len() == points.len());
    // Same private CurveU right-biased binary search and inverse lerp owner as
    // the restored Curve2 API, applied to the original cached Curve3 parameters.
    let curve = utils::curve::Curve3::from_cached_parts(
        points.iter().copied().map(glam::Vec3::from_array).collect(),
        us.clone(),
        wall.geometry.mapped_curve_length,
    );
    let (i, t) = curve.coord_at_u(0.5);
    let a = points[i];
    let b = points[i + 1];
    glam::Vec3::from_array(std::array::from_fn(|axis| {
        t * b[axis] + a[axis] * (1.0 - t)
    }))
}
pub fn set_wall_style(
    mut styles: EventReader<SetWallStyleCmd>,
    mut freehand: EventReader<SetFreehandHeightTypeCmd>,
    mut plaster: EventReader<SetPlasterPeelAmountCmd>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    mut history: EventWriter<RecordWallHistory>,
    mut audio: EventWriter<PlayWallUiSound>,
) {
    for command in styles.read() {
        inner.update_style(command.wall_id, command.to);
        if command.record_history != 0 {
            record(
                &mut history,
                vec![WallHistoryEdit::ChangeStyle(WallValueEdit {
                    wall_index: command.wall_id.0,
                    before: style(command.from),
                    after: style(command.to),
                })],
            );
        }
        if command.play_sound != 0 {
            audio.write(PlayWallUiSound {
                kind: 0x10,
                selector: command.to,
                value: 0.0,
                position: audio_position(&public, command.wall_id),
            });
        }
    }
    // Original ordering is Style, Plaster, Freehand despite argument order.
    for command in plaster.read() {
        inner.update_plaster_peel_amount(command.wall_id, command.to);
        if command.record_history != 0 {
            record(
                &mut history,
                vec![WallHistoryEdit::ChangePlasterPeelAmount(WallValueEdit {
                    wall_index: command.wall_id.0,
                    before: command.from,
                    after: command.to,
                })],
            );
        }
    }
    for command in freehand.read() {
        inner.update_freehand_ty(command.wall_id, command.to);
        inner.update_arch_type(command.wall_id, command.to);
        if command.record_history == 1 {
            let edit = WallValueEdit {
                wall_index: command.wall_id.0,
                before: policy(command.from),
                after: policy(command.to),
            };
            record(
                &mut history,
                vec![
                    WallHistoryEdit::ChangeFreehandHeightType(edit.clone()),
                    WallHistoryEdit::ChangeArchType(edit),
                ],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn style_audio_uses_mapped_curve_midpoint() {
        let mut inner = InnerWalls::default();
        let id = inner.new_wall(WallState::new(
            Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(),
            3.,
            0,
            2,
            false,
        ));
        let mut public = PublicWalls::default();
        public.consume(
            &mut inner,
            &crate::resources::walls::publication_source::TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: false,
            },
        );
        assert_eq!(audio_position(&public, id), glam::Vec3::new(4., 0., 0.));
    }
    use crate::resources::{
        roofs::geometry::surface::Curve2,
        walls::{WallId, WallState},
    };
    #[test]
    fn canonical_commands_mutate_authority_record_groups_and_do_not_rerecord_hidden_restore() {
        let mut world = World::new();
        world.init_resource::<InnerWalls>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<Events<SetWallStyleCmd>>();
        world.init_resource::<Events<SetFreehandHeightTypeCmd>>();
        world.init_resource::<Events<SetPlasterPeelAmountCmd>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<PlayWallUiSound>>();
        let id = WallId(38);
        world.resource_mut::<InnerWalls>().new_wall_with_id(
            id,
            WallState::new(
                Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(),
                3.,
                0,
                2,
                false,
            ),
        );
        world.resource_mut::<InnerWalls>().take_changes();
        world
            .resource_mut::<Events<SetWallStyleCmd>>()
            .send(SetWallStyleCmd {
                wall_id: id,
                from: 0,
                to: 2,
                record_history: 1,
                play_sound: 1,
            });
        world
            .resource_mut::<Events<SetFreehandHeightTypeCmd>>()
            .send(SetFreehandHeightTypeCmd {
                wall_id: id,
                from: 0,
                to: 1,
                record_history: 1,
            });
        world
            .resource_mut::<Events<SetPlasterPeelAmountCmd>>()
            .send(SetPlasterPeelAmountCmd {
                wall_id: id,
                from: 0.,
                to: 0.75,
                record_history: 1,
            });
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(set_wall_style);
        schedule.run(&mut world);
        let walls = world.resource::<InnerWalls>();
        let wall = walls.get(id).unwrap();
        assert_eq!(wall.style(), 2);
        assert_eq!(wall.freehand_ty(), 1);
        assert_eq!(wall.arch_type(), 1);
        assert_eq!(wall.plaster_peel_amount(), 0.75);
        assert!(walls.changes().updated.contains(&id));
        let records: Vec<_> = world
            .resource_mut::<Events<RecordWallHistory>>()
            .drain()
            .collect();
        assert_eq!(records.len(), 3);
        assert!(matches!(
            records[0].0.groups[0][0],
            WallHistoryEdit::ChangeStyle(_)
        ));
        assert!(matches!(
            records[1].0.groups[0][0],
            WallHistoryEdit::ChangePlasterPeelAmount(_)
        ));
        assert_eq!(records[2].0.groups.len(), 2);
        assert!(matches!(
            records[2].0.groups[0][0],
            WallHistoryEdit::ChangeFreehandHeightType(_)
        ));
        assert!(matches!(
            records[2].0.groups[1][0],
            WallHistoryEdit::ChangeArchType(_)
        ));
        let sounds: Vec<_> = world
            .resource_mut::<Events<PlayWallUiSound>>()
            .drain()
            .collect();
        assert_eq!(sounds.len(), 1);
        assert_eq!(sounds[0].kind, 0x10);
        assert_eq!(sounds[0].selector, 2);
        assert_eq!(sounds[0].position, glam::Vec3::ZERO);
        world
            .resource_mut::<Events<SetWallStyleCmd>>()
            .send(SetWallStyleCmd {
                wall_id: id,
                from: 2,
                to: 0,
                record_history: 0,
                play_sound: 0,
            });
        schedule.run(&mut world);
        assert_eq!(world.resource::<InnerWalls>().get(id).unwrap().style(), 0);
        assert!(world.resource::<Events<RecordWallHistory>>().is_empty());
        assert!(world.resource::<Events<PlayWallUiSound>>().is_empty());
    }
}
