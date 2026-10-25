use tiny_glade_reconstruction::{
    building_roof_associations::{ResolvedRoofAssociation, RoofAssociations},
    building_roof_creation::RoofCreateRequestObserved,
    building_roof_lifecycle::ResolvedRoofLifecycle,
    building_wall_publication::{WallPublicationCore, publication_metrics},
    building_wall_state::{PublicWallBuildingState, WallId, WallShapeSnapshot},
    wall_height_rules::HeightCacheError,
};

fn wall() -> PublicWallBuildingState {
    let mut bytes = [0; 672];
    bytes[0x10..0x18].copy_from_slice(&3u64.to_le_bytes());
    bytes[0x30..0x34].copy_from_slice(&2f32.to_le_bytes());
    bytes[0x108] = 1;
    bytes[0x10c..0x110].copy_from_slice(&7.5f32.to_le_bytes());
    bytes[0x258..0x25c].copy_from_slice(&1u32.to_le_bytes());
    PublicWallBuildingState::from_observed(bytes)
}

#[test]
fn deleted_roof_remains_associated_until_removed_and_then_notifies_wall_once() {
    let wall = wall();
    let id = WallId(u64::MAX);
    let entity = 0x100000029;
    let mut record = [0; 64];
    record[8..16].copy_from_slice(&id.0.to_le_bytes());
    record[44..48].copy_from_slice(&0.5f32.to_le_bytes());
    record[48..52].copy_from_slice(&1f32.to_le_bytes());
    let mut roof = ResolvedRoofLifecycle::prepare(
        entity,
        [0xa5; 88],
        [0xa5; 24],
        &RoofCreateRequestObserved::from_observed(record),
        &wall,
    )
    .unwrap();
    let mut associations = RoofAssociations::default();
    let first = associations
        .rebuild(
            [ResolvedRoofAssociation {
                wall_id: id,
                entity,
                marked: false,
                public_wall: Some(&wall),
            }],
            &[id],
            0,
        )
        .unwrap();
    assert_eq!(first.wall_presence_changes, vec![id]);
    roof.advance(0.5, 1);
    assert!(roof.consume_delete_requests(&[WallId(0)]).is_none());
    assert!(roof.consume_delete_requests(&[id, id]).is_some());
    roof.deletion_mark_committed();
    let fading = associations
        .rebuild(
            [ResolvedRoofAssociation {
                wall_id: id,
                entity,
                marked: true,
                public_wall: Some(&wall),
            }],
            &[id],
            0,
        )
        .unwrap();
    assert!(fading.wall_presence_changes.is_empty());
    assert_eq!(associations.entity(id), Some(entity));
    assert_eq!(roof.advance(0.5, 2), Some(entity));
    let removed = associations.rebuild([], &[id], 0).unwrap();
    assert_eq!(removed.wall_presence_changes, vec![id]);
    assert!(
        associations
            .rebuild([], &[id], 0)
            .unwrap()
            .wall_presence_changes
            .is_empty()
    );
}

#[test]
fn republishing_invalidates_height_cache_and_preserves_unconfirmed_projection_fields() {
    let mut bytes = [0xa5; 672];
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    let shape = WallShapeSnapshot::from_observed(shape);
    let metrics = publication_metrics(
        &[1., 2., 3.],
        &[[0., 7., 0.], [1., 8., 0.], [2., 9., 0.]],
        0.7,
        false,
    );
    let published = WallPublicationCore {
        source_point_count: 3,
        source_curve_length: 2.,
        shape: &shape,
        style: 2,
        metrics,
    }
    .write_projection(&mut bytes);
    assert_eq!(published.max_y(), Err(HeightCacheError::MissingMaximum));
    assert!(published.validate_read().is_ok());
    assert!(published.is_halftimbered());
    assert_eq!(bytes[0x40], 0xa5);
    assert_eq!(bytes[0x2a0 - 1], 0xa5);
}
