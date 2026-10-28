//! B 0x1412357c0: pinned-clutter volume selection and ECS icon lifetime.
//! Restores the shared typed event and reads authoritative decorator storage,
//! transforms, mesh bounds and collision data. Native-physics is required for
//! real occlusion. Invalid mesh type 0x3e remains explicitly unsupported.
use bevy_ecs::prelude::*;
use utils::geometry::{Circle2d, Rectangle2d, Rectangle3d};

/// Semantic projection of the B payload. Rust layout is not the native union ABI.
#[derive(Event, Clone, Copy, Debug)]
pub enum ShowPinnedDecoClutter {
    Circle {
        circle: Circle2d,
        flag: bool,
        bottom: f32,
        top: f32,
        exclude_wall: Option<crate::resources::walls::WallId>,
    },
    Rectangle {
        rectangle: Rectangle2d,
        bottom: f32,
        top: f32,
        exclude_wall: Option<crate::resources::walls::WallId>,
    },
    Wall {
        wall: crate::resources::walls::WallId,
    },
}

impl ShowPinnedDecoClutter {
    fn intersects(
        &self,
        bounds: Rectangle3d,
        bucket_wall: Option<crate::resources::walls::WallId>,
        dst: u8,
        walls: &crate::resources::walls::public_walls::PublicWalls,
    ) -> bool {
        match *self {
            Self::Circle {
                circle,
                flag,
                bottom,
                top,
                exclude_wall,
            } => {
                if dst == 2 {
                    let wall = bucket_wall.expect("wall destination in a non-wall storage");
                    if exclude_wall == Some(wall) {
                        return false;
                    }
                }
                if !(bottom <= bounds.top && bounds.bottom <= top) {
                    return false;
                }
                let center = bounds.footprint.center;
                if flag
                    && (circle.contains_point(center)
                        || (center - circle.center).length() < circle.radius)
                {
                    return true;
                }
                bounds.footprint.intersects_circle(circle)
            }
            Self::Rectangle {
                rectangle,
                bottom,
                top,
                exclude_wall,
            } => {
                if dst == 2 {
                    let wall = bucket_wall.expect("wall destination in a non-wall storage");
                    if exclude_wall == Some(wall) {
                        return false;
                    }
                }
                bottom <= bounds.top
                    && bounds.bottom <= top
                    && bounds.footprint.intersects(rectangle)
            }
            Self::Wall { wall } => {
                let Some(state) = walls.get(wall) else {
                    return false;
                };
                let points = state.geometry.outline.as_slice();
                assert!(
                    points.len() >= 2 && state.simplified_curve_2d().length > 0.0,
                    "invalid published wall curve"
                );
                let upper = if state.authority.freehand_ty() == 0 {
                    state.building_projection().float_0x284()
                } else {
                    state.max_y()
                };
                points.iter().any(|p| {
                    let bottom = if state.authority.freehand_ty() == 0 {
                        p[1]
                    } else {
                        0.0
                    };
                    bottom <= bounds.top
                        && bounds.bottom <= upper
                        && bounds.footprint.contains_point(glam::Vec2::new(p[0], p[2]))
                })
            }
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoClutterLock {
    pub decorator_id: u32,
    pub progress: f32,
    pub base_extent: glam::Vec2,
    pub elapsed: f32,
    pub expiring: bool,
}

// B 0x141236bd9..c00; equal signed zeros select x, unordered upper selects 0.5.
fn pin_extent(size: glam::Vec2) -> f32 {
    let minimum = if size.x.is_nan() {
        size.y
    } else if size.y < size.x {
        size.y
    } else {
        size.x
    };
    let extent = minimum * 0.5;
    if extent < 0.5 { extent } else { 0.5 }
}

/// `decoclutter_show_pins` @0x1412357c0。
pub fn decoclutter_show_pins(
    mut show_events: EventReader<ShowPinnedDecoClutter>,
    meshes: Res<crate::resources::clutter_meshes::ClutterMeshes>,
    storage: Res<crate::resources::walls::decorator_storage::DecoratorStorage>,
    transforms: Res<
        crate::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    time: Res<crate::resources::time::GameTime>,
    raycast: Res<crate::systems::collision::world::RaycastWorld>,
    camera: Res<crate::systems::camera_rig::main_camera::MainCamera>,
    mut query: Query<(
        Entity,
        &mut DecoClutterLock,
        &mut crate::components::ui::Ui2dWsExtent,
    )>,
    mut commands: Commands,
) {
    use crate::components::ui::{Ui2dImage, Ui2dWsExtent};
    use crate::resources::clutter_meshes::{ClutterMeshTy, clutter_mesh_ty_data};
    let request = show_events.read().last();
    // Preserve candidate ordering and duplicates; the native system compares
    // against existing entities, rather than deduplicating this frame's input.
    let mut candidates = Vec::new();
    if let Some(request) = request {
        let buckets = storage
            .iter_walls()
            .map(|(wall, bucket)| (Some(wall), bucket))
            .chain(
                storage
                    .iter_terrain_decorators()
                    .map(|bucket| (None, bucket)),
            )
            .chain(storage.iter_stair_decorators().map(|bucket| (None, bucket)));
        for (bucket_wall, bucket) in buckets {
            for (id, info) in bucket.iter_maybe_mut() {
                let d = info.ty.wrapping_sub(0x38);
                if d <= 7 && d != 6 {
                    continue;
                }
                // Native 0x3e passes the first gate but has no proved valid
                // mesh producer. Keep this unsupported input explicit.
                let transform = transforms.transform(id);
                let ty = ClutterMeshTy::from_native_tag(info.ty)
                    .expect("unrecovered pinned clutter mesh type");
                let mut bounds = clutter_mesh_ty_data(ty, &meshes).mesh.bounds;
                bounds.expand(glam::Vec3::splat(0.5));
                let x = transform.rotation * glam::Vec3::X;
                let axis = glam::Vec2::new(x.x, x.z);
                bounds.set_orientation_xz(axis / (axis.x * axis.x + axis.y * axis.y).sqrt());
                bounds.translate(transform.translation);
                if !request.intersects(bounds, bucket_wall, info.dst_tag(), &walls) {
                    continue;
                }
                let eye = camera.eye_position();
                let v = transform.translation - eye;
                let length = ((v.x * v.x + v.y * v.y) + v.z * v.z).sqrt();
                let hit =
                    raycast.raycast_simple_with_max_toi(eye, v * (1.0 / length), length - 0.2, 3);
                if !hit.is_miss() {
                    continue;
                }
                let extent = pin_extent(bounds.footprint.size);
                candidates.push((id, extent, transform.translation));
            }
        }
    }
    let mut existing = Vec::new();
    for (entity, mut lock, mut extent) in &mut query {
        existing.push(lock.decorator_id);
        lock.elapsed += time.delta;
        if candidates
            .iter()
            .any(|candidate| candidate.0 == lock.decorator_id)
        {
            lock.expiring = false;
        } else if lock.elapsed > 0.75 {
            lock.expiring = true;
        }
        let period = if lock.expiring { -0.2 } else { 0.3 };
        lock.progress += time.delta / period;
        if lock.progress <= 0.0 && lock.expiring {
            commands.entity(entity).despawn();
            continue;
        }
        lock.progress = lock.progress.clamp(0.0, 1.0);
        extent.extent = lock.base_extent
            * utils::easing_functions::EasingFunction::BackOut.apply(lock.progress);
    }
    for (id, size, position) in candidates {
        if existing.contains(&id) {
            continue;
        }
        let base_extent = glam::Vec2::splat(size);
        commands.spawn((
            DecoClutterLock {
                decorator_id: id,
                base_extent,
                ..Default::default()
            },
            Ui2dWsExtent {
                extent: base_extent,
                position,
            },
            Ui2dImage::new("menu/tod/locked"),
        ));
    }
}

/// `register_required_components` @0x141237200（O：bevy 组件注册；`DecoClutterLock`）。
pub fn register_required_components(world: &mut World) {
    world.register_component::<DecoClutterLock>();
}

/// `clone_behavior` @0x140002370（O：ICF 折叠单行；行为标记，无独立逻辑）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CloneBehavior(pub u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extent_matches_original_instruction_bits() {
        let data: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-pin-extent-fragment-fixture.json"))).unwrap();
        assert_eq!(
            data["pe_sha256"],
            "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
        );
        for case in data["cases"].as_array().unwrap() {
            let bits = |name: &str| u32::from_str_radix(case[name].as_str().unwrap(), 16).unwrap();
            assert_eq!(
                pin_extent(glam::Vec2::new(
                    f32::from_bits(bits("x_bits")),
                    f32::from_bits(bits("y_bits"))
                ))
                .to_bits(),
                bits("extent_bits")
            );
        }
    }

    #[test]
    fn real_storage_event_spawns_and_no_event_expires_pin() {
        use crate::components::ui::Ui2dWsExtent;
        use crate::resources::{
            clutter_meshes::{ClutterMesh, ClutterMeshes},
            walls::{
                cached_decorator_transforms::{
                    CachedDecoratorTransform, CachedDecoratorTransforms,
                },
                decorator::{DecoratorId, DecoratorInfo},
                decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
                public_walls::PublicWalls,
            },
        };
        let mut world = World::new();
        let fixed =
            std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test", 0, None, false, false));
        world.insert_resource(ClutterMeshes::from_parts(fixed, Vec::new()));
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(DecoratorId(7), DecoratorInfo::default());
        world.insert_resource(storage);
        let mut transforms = CachedDecoratorTransforms::default();
        transforms.insert(
            7,
            CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY,
                translation: glam::Vec3::new(2.0, 1.0, 2.0),
                scale: glam::Vec3::ONE,
            },
        );
        world.insert_resource(transforms);
        world.init_resource::<PublicWalls>();
        world.init_resource::<crate::resources::time::GameTime>();
        world.init_resource::<crate::systems::camera_rig::main_camera::MainCamera>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        world.send_event(ShowPinnedDecoClutter::Circle {
            circle: Circle2d::new(glam::Vec2::ZERO, 10.0),
            flag: true,
            bottom: 0.0,
            top: 3.0,
            exclude_wall: None,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(decoclutter_show_pins);
        schedule.run(&mut world);
        let mut pins = world.query::<(&DecoClutterLock, &Ui2dWsExtent)>();
        let (pin, extent) = pins.single(&world).unwrap();
        assert_eq!(pin.decorator_id, 7);
        assert_eq!(extent.position, glam::Vec3::new(2.0, 1.0, 2.0));
        assert_eq!(extent.extent, glam::Vec2::splat(0.25));
        // No new event still advances the real entity's lifetime.
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 0.1;
        schedule.run(&mut world);
        assert!(pins.single(&world).unwrap().0.progress > 0.0);
        for _ in 0..12 {
            schedule.run(&mut world);
        }
        assert_eq!(pins.iter(&world).count(), 0);
        // The freehand producers send the Wall variant. Exercise it against
        // an actual authority publication, rather than a second circle proxy.
        let mut inner = crate::resources::walls::InnerWalls::default();
        let wall = inner.new_wall(crate::resources::walls::WallState::new(
            crate::resources::roofs::geometry::surface::Curve2::try_new(
                vec![[2.0, 2.0], [8.0, 2.0]],
                false,
            )
            .unwrap(),
            3.0,
            0,
            0,
            false,
        ));
        let terrain = crate::resources::walls::publication_source::TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        world.resource_mut::<PublicWalls>().consume_with_paths(
            &mut inner,
            &terrain,
            None,
            &crate::resources::walls::public_walls::WallPublicationDirty::default(),
            &crate::resources::walls::public_walls::WallSpatialHash::default(),
        );
        world.send_event(ShowPinnedDecoClutter::Wall { wall });
        schedule.run(&mut world);
        let entity = world
            .query_filtered::<Entity, With<DecoClutterLock>>()
            .single(&world)
            .unwrap();
        assert_eq!(
            world.get::<DecoClutterLock>(entity).unwrap().decorator_id,
            7
        );
        world.despawn(entity);
        world.send_event(ShowPinnedDecoClutter::Wall {
            wall: crate::resources::walls::WallId(u64::MAX),
        });
        schedule.run(&mut world);
        assert_eq!(world.query::<&DecoClutterLock>().iter(&world).count(), 0);
        let expired = world
            .spawn((
                DecoClutterLock {
                    decorator_id: 7,
                    progress: f32::NAN,
                    elapsed: 1.0,
                    expiring: true,
                    base_extent: glam::Vec2::ONE,
                },
                Ui2dWsExtent::default(),
            ))
            .id();
        schedule.run(&mut world);
        assert!(
            world
                .get::<DecoClutterLock>(expired)
                .unwrap()
                .progress
                .is_nan()
        );
        world.despawn(expired);
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 0.0;
        let alive = world
            .spawn((
                DecoClutterLock {
                    decorator_id: 7,
                    progress: f32::NAN,
                    base_extent: glam::Vec2::ONE,
                    ..Default::default()
                },
                Ui2dWsExtent::default(),
            ))
            .id();
        schedule.run(&mut world);
        assert!(
            world
                .get::<DecoClutterLock>(alive)
                .unwrap()
                .progress
                .is_nan()
        );
        world.despawn(alive);
        #[cfg(feature = "native-physics")]
        {
            use crate::systems::collision::world::{
                ColliderMeshInstance, RaycastGeoSource, RaycastMeshData, RaycastWorld,
                upsert_collider,
            };
            let eye = world
                .resource::<crate::systems::camera_rig::main_camera::MainCamera>()
                .eye_position();
            let target = glam::Vec3::new(2.0, 1.0, 2.0);
            let mut instances = std::collections::HashMap::<u32, ColliderMeshInstance>::new();
            let mut physics = world.resource_mut::<RaycastWorld>();
            upsert_collider(
                &mut physics,
                &mut instances,
                1,
                RaycastGeoSource::Slot0,
                std::sync::Arc::new(RaycastMeshData::new_cuboid_at(
                    glam::Vec3::splat(0.4),
                    glam::Vec3::ZERO,
                )),
                eye + (target - eye) * 0.5,
                glam::Quat::IDENTITY,
            );
            physics.flush_colliders();
            drop(physics);
            world.send_event(ShowPinnedDecoClutter::Circle {
                circle: Circle2d::new(glam::Vec2::ZERO, 10.0),
                flag: true,
                bottom: 0.0,
                top: 3.0,
                exclude_wall: None,
            });
            schedule.run(&mut world);
            assert_eq!(
                world.query::<&DecoClutterLock>().iter(&world).count(),
                0,
                "occluded decorations must not produce pins"
            );
        }
    }

    #[test]
    fn owner_exclusion_and_unordered_height_match_native_gate() {
        use crate::resources::walls::{WallId, public_walls::PublicWalls};
        let walls = PublicWalls::default();
        let bounds = Rectangle3d::new(glam::Vec3::ZERO, glam::Vec3::ONE, glam::Vec2::X);
        let request = ShowPinnedDecoClutter::Circle {
            circle: Circle2d::new(glam::Vec2::ZERO, 2.0),
            flag: true,
            bottom: -1.0,
            top: 1.0,
            exclude_wall: Some(WallId(3)),
        };
        assert!(!request.intersects(bounds, Some(WallId(3)), 2, &walls));
        assert!(request.intersects(bounds, Some(WallId(4)), 2, &walls));
        assert!(std::panic::catch_unwind(|| request.intersects(bounds, None, 2, &walls)).is_err());
        let nan = ShowPinnedDecoClutter::Circle {
            circle: Circle2d::new(glam::Vec2::ZERO, 2.0),
            flag: true,
            bottom: f32::NAN,
            top: 1.0,
            exclude_wall: None,
        };
        assert!(!nan.intersects(bounds, None, 0, &walls));
        let surrounding = ShowPinnedDecoClutter::Rectangle {
            rectangle: Rectangle2d::new(glam::Vec2::ZERO, glam::Vec2::splat(10.0), glam::Vec2::X),
            bottom: -1.0,
            top: 1.0,
            exclude_wall: None,
        };
        assert!(
            !surrounding.intersects(bounds, None, 0, &walls),
            "native rectangle tests boundaries rather than containment"
        );
    }
}
