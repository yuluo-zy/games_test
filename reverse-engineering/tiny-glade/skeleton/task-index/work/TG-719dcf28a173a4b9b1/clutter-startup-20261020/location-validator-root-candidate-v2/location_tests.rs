#[cfg(test)]
mod location_native_tests {
    use super::*;
    use bevy_ecs::system::SystemState;
    use country_core::resources::walls::{InnerWalls, WallState};
    use country_core::resources::walls::decorator_subtype::{
        DecoratorCollision, DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes,
    };
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
    use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;
    use country_core::systems::collision::world::RaycastMeshData;
    use std::sync::Arc;

    // 这些是显式测试输入，不作为游戏启动已拥有原资源的证据。
    struct Fixture {
        world: World,
        walls: PublicWalls,
        subtypes: DerivedDecoratorSubtypes,
        meshes: DecoratorMeshLibrary,
        cache: CachedDecoratorTransforms,
        raycast: RaycastWorld,
    }
    impl Fixture {
        fn new(subtype: u8, shape: u32) -> Self {
            let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::splat(8.0));
            let points = rectangle.as_points2().map(|p| [p.x, p.y]);
            let curve = Curve2::try_new(points.to_vec(), true).unwrap();
            let mut snapshot = WallState::new(curve, 3.0, 0, 0, false).snapshot().clone();
            let mut bytes = [0; 28];
            bytes[..4].copy_from_slice(&shape.to_le_bytes());
            for (index, value) in [1.0_f32, 0.0, 0.0, 0.0, 8.0, 8.0].into_iter().enumerate() {
                bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
            }
            snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
            snapshot.source.curve_kind_tag = shape;
            let mut inner = InnerWalls::new();
            inner.new_wall_with_id(WallId(42), WallState::from_snapshot(snapshot, 0));
            let mut walls = PublicWalls::default();
            walls.consume(&mut inner, &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(), minimum_y: 0.0,
                maximum_y: 0.0, sampling_fix: false,
            });
            let wall = walls.get_mut(WallId(42)).unwrap();
            wall.prepared_path = Some(PreparedWallPath {
                positions: vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [8.0, 0.0, 0.0]],
                points_u: vec![0.0, 0.5, 1.0], length: 8.0, segment_values: vec![0.0; 3],
            });
            wall.segment_heights = vec![0.0; 3];
            let mut subtypes = DerivedDecoratorSubtypes::default();
            subtypes.insert(7, DecoratorSubtype(subtype));
            let mut meshes = DecoratorMeshLibrary::default();
            meshes.collision.insert(DecoratorLibKey::new(subtype, 1), DecoratorCollision {
                triangles: Vec::new(), mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::splat(2.0), Vec2::X),
            });
            let mut cache = CachedDecoratorTransforms::default();
            cache.insert(7, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: Vec3::new(4.0, 2.0, 0.0), scale: Vec3::ONE,
            });
            Self { world: World::new(), walls, subtypes, meshes, cache, raycast: RaycastWorld::new() }
        }
        fn info(&self, tag: u8, u: f32, y: f32) -> DecoratorInfo {
            DecoratorInfo { dst_wall: u64::from(u.to_bits()) | (u64::from(y.to_bits()) << 32),
                decorator_id: 99, rank: 1, ty: 0x3f, u20: u32::from(tag) << 16,
                ..DecoratorInfo::default() }
        }
        fn check(&mut self, info: &DecoratorInfo, dst: DecoratorStorageAddr) -> DecoratorLocationCheckResult {
            let mut state = SystemState::<Query<&Roof>>::new(&mut self.world);
            let roofs = state.get(&self.world);
            is_decorator_location_valid(DecoratorId(7), info, dst, &self.walls, &roofs,
                DerivedDecoratorInfoView::new(&self.subtypes, &self.meshes), &self.cache, &self.raycast)
        }
        fn add_roof(&mut self, flat: bool, gable: bool) {
            let mut bytes = [0_u8; 88];
            bytes[8..12].copy_from_slice(&1_u32.to_le_bytes());
            for (offset, value) in [(12, 1.0_f32), (16, 0.0), (20, 0.0), (24, 0.0),
                (28, 8.0), (32, 8.0), (48, if flat { 0.0 } else { 0.5 }),
                (52, if gable { 1.0 } else { 0.0 })] {
                bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            }
            let entity = self.world.spawn(Roof::from_observed(bytes)).id();
            self.walls.associate_roof(WallId(42), entity);
        }
    }

    #[test]
    fn terrain_and_stair_return_native_eleven_before_any_required_lookup() {
        let mut f = Fixture::new(0, 1);
        f.cache = CachedDecoratorTransforms::default();
        f.subtypes = DerivedDecoratorSubtypes::default();
        f.meshes = DecoratorMeshLibrary::default();
        for tag in [3, 4] {
            let info = f.info(tag, f32::NAN, f32::NAN);
            assert_eq!(f.check(&info, DecoratorStorageAddr::Terrain), DecoratorLocationCheckResult::Valid);
        }
    }
    #[test]
    fn native_result_schema_preserves_every_reason_payload_and_tag() {
        use DecoratorLocationInvalidReason::*;
        let wall = WallId(42);
        let coord = CurveInternalCoord { segment: 9, t: 0.25 };
        let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::ONE);
        let reasons = [WallCutoutBoundary { wall, coord },
            RoofBounds { position: Vec3::ONE, direction: Vec3::X, wall },
            WallEnds { left_oob: true, right_oob: false, wall }, SharpWallTurn { wall, coord },
            WallTop { padding: 0.79, wall }, WallCorner { wall, rectangle, corner: 3 },
            BalconyUpperClearance { wall }, FlatRoofForbidden, GableBandForbidden,
            MissingRoof, WallLowerBoundary { wall }];
        for (index, reason) in reasons.into_iter().enumerate() {
            let result = DecoratorLocationCheckResult::Invalid(reason);
            assert_eq!(result.native_tag(), index as u8);
            assert!(!result.is_valid());
            assert_eq!(result.get_invalid_reason(), Some(&reason));
        }
        assert_eq!(DecoratorLocationCheckResult::Valid.native_tag(), 11);
    }
    #[test]
    fn absent_roof_is_reason_nine_without_reading_catalog_or_cache() {
        let mut f = Fixture::new(0, 1);
        f.subtypes = DerivedDecoratorSubtypes::default();
        f.cache = CachedDecoratorTransforms::default();
        let info = f.info(2, 0.0, 2.0);
        assert_eq!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))).native_tag(), 9);
    }
    #[test]
    fn wall_requires_actual_modified_id_cache_before_other_wall_work() {
        let mut f = Fixture::new(0, 1);
        let info = f.info(0, 4.0, 2.0);
        f.cache = CachedDecoratorTransforms::default();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42))))).is_err());
    }
    #[test]
    fn open_wall_checks_both_unclamped_endpoints() {
        let mut f = Fixture::new(0, 2);
        let info = f.info(0, -0.5, 2.0);
        assert_eq!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))),
            DecoratorLocationCheckResult::Invalid(DecoratorLocationInvalidReason::WallEnds {
                wall: WallId(42), left_oob: true, right_oob: false,
            }));
    }
    #[test]
    fn positive_cut_and_zero_lower_boundary_are_distinct_native_reasons() {
        let mut f = Fixture::new(0, 2);
        f.walls.get_mut(WallId(42)).unwrap().segment_heights.fill(1.0);
        let info = f.info(0, 4.0, 1.0);
        let result = f.check(&info, DecoratorStorageAddr::Wall(WallId(42)));
        assert!(matches!(result, DecoratorLocationCheckResult::Invalid(
            DecoratorLocationInvalidReason::WallCutoutBoundary { coord, .. })
            if coord.segment == 0 && coord.t == 0.75));
        f.walls.get_mut(WallId(42)).unwrap().segment_heights.fill(0.0);
        let info = f.info(0, 4.0, 0.0);
        assert_eq!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))).native_tag(), 10);
    }
    #[test]
    fn real_three_point_path_turn_precedes_the_unready_arch_height_lookup() {
        let mut f = Fixture::new(0, 2);
        let wall = f.walls.get_mut(WallId(42)).unwrap();
        wall.prepared_path.as_mut().unwrap().positions =
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]];
        let info = f.info(0, 1.0, 2.0);
        let result = f.check(&info, DecoratorStorageAddr::Wall(WallId(42)));
        assert!(matches!(result, DecoratorLocationCheckResult::Invalid(
            DecoratorLocationInvalidReason::SharpWallTurn { coord, .. })
            if coord.segment == 0 && coord.t == 1.0));
    }
    #[test]
    fn first_actual_rectangle_corner_wins_before_optional_roof_check() {
        let mut f = Fixture::new(0, 1);
        let mut info = f.info(0, 0.5, 2.0);
        info.ty = 0x38;
        assert!(matches!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))),
            DecoratorLocationCheckResult::Invalid(DecoratorLocationInvalidReason::WallCorner {
                wall: WallId(42), corner: 0, .. })));
    }
    #[test]
    fn flat_roof_forbidden_and_gable_half_open_bands_use_current_type_and_coords() {
        let mut f = Fixture::new(0, 1);
        f.add_roof(true, false);
        let mut info = f.info(2, 0.125, 1.0);
        info.ty = 0x3a;
        assert_eq!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))).native_tag(), 7);
        f.add_roof(false, true);
        info.ty = 0x38;
        assert_eq!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))).native_tag(), 8);
        info.dst_wall = u64::from(0.25_f32.to_bits()) | (u64::from(1.0_f32.to_bits()) << 32);
        assert!(f.check(&info, DecoratorStorageAddr::Wall(WallId(42))).is_valid());
    }
    #[test]
    fn zero_segment_and_unordered_ridge_math_do_not_fabricate_finite_directions() {
        assert!(location_unit3(Vec3::ZERO).is_nan());
        assert!(location_unit2(Vec2::ZERO).is_nan());
        let points = [[0.0, 0.0, 0.0], [0.0, 5.0, 0.0]];
        assert!(location_curve_coord(&points, 0.0).t.is_nan());
        let amount = f32::INFINITY;
        let top = location_elevated(Vec3::ONE, amount);
        assert!(top.x.is_nan() && top.z.is_nan() && top.y.is_infinite());
    }
}
