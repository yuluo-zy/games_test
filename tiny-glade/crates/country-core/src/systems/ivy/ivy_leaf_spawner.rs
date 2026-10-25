//! B 0x140996080; business body 0x1409964f6..0x140996e9a.
use crate::resources::{
    glade_settings::GladeSettings,
    ivy::{IvyId, IvyLeafInput, IvyLeafRng, IvyLeafStorage, IvyPointId, IvySpawnNoise, IvyStorage},
    time::GameTime,
};
use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};

/// Original payload is 24 bytes; its native Bevy event row is 32 bytes.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct IvySegmentDoneGrowingMsg {
    pub point_a: IvyPointId,
    pub point_b: IvyPointId,
    pub ivy: IvyId,
}

/// Generate one event's leaf if capacity, selection RNG and noise accept it.
/// Missing ivy warns; absent point IDs panic just as the original unwraps do.
pub fn spawn_segment_leaf(
    message: &IvySegmentDoneGrowingMsg,
    storage: &mut IvyStorage,
    leaves: &mut IvyLeafStorage,
    rng: &mut IvyLeafRng,
    noise: &IvySpawnNoise,
    frame: u32,
) {
    if leaves.leaf_count() >= 10_000 {
        return;
    }
    if storage.rng_mut().uniform_f32() < 0.4 {
        return;
    }
    let Some(ivy) = storage.get(message.ivy) else {
        log::warn!(target: "country_core::systems::ivy::ivy_leaf_spawner", "Missing ivy {} for completed segment", message.ivy);
        return;
    };
    let a = ivy
        .get_point_id(message.point_a)
        .expect("missing first ivy segment point");
    let b = ivy
        .get_point_id(message.point_b)
        .expect("missing second ivy segment point");
    let sample = a.position + a.position + Vec3::new(89.0, -3456.0, -34.0);
    if !(noise.noise.get_noise3d(sample.x, sample.y, sample.z) > -0.05) {
        return;
    }

    let normal = ((b.direction + a.direction) * 0.5).normalize();
    let disc = rng.0.uniform_disc_vec2() * 0.15;
    let branch_direction = (b.position - a.position).normalize();
    let position =
        branch_direction * disc.y + (branch_direction.cross(normal) * disc.x + a.position);

    // Original normal calls are x, z, y. Preserve their stream order.
    let x = rng.0.normal_f32().clamp(0.0, 1.0);
    let z = rng.0.normal_f32().clamp(0.0, 1.0);
    let y = rng.0.normal_f32().clamp(0.0, 1.0);
    let jitter = Vec3::new(x - (1.0 - x), y - (1.0 - y), z - (1.0 - z)) * 0.5;
    let vector_58 = (jitter + normal).normalize();
    let angle = rng.0.normal_f32();
    let downward = rng.0.normal_f32();
    let size_sample = rng.0.normal_f32();
    let angle = (angle / 1.5).clamp(-1.0, 1.0) * 80.0 * f32::from_bits(0x3c8e_fa35);
    let mut vector_2c = Quat::from_rotation_y(angle) * normal;
    let downward = downward.abs() / 1.5;
    vector_2c.y -= if 1.0 < downward { 1.0 } else { downward };
    let vector_2c = vector_2c.normalize();
    let delay = rng.0.uniform_f32() * 0.3 + 0.0;
    let size_mix = size_sample.abs() / 3.0;
    let size_mix = if 1.0 < size_mix { 1.0 } else { size_mix };
    leaves.add(IvyLeafInput {
        position,
        vector_2c,
        vector_58,
        branch_direction,
        ivy: message.ivy,
        point_a: message.point_a,
        point_b: message.point_b,
        size: (1.0 - size_mix) * 0.8 + size_mix * 1.2,
        growth_delay: delay,
        frame,
        source_18: ivy.source_60,
    });
}

/// Disabled frames do not advance this EventReader, matching the native gate.
pub fn ivy_leaf_spawner(
    settings: Res<GladeSettings>,
    mut messages: EventReader<IvySegmentDoneGrowingMsg>,
    mut storage: ResMut<IvyStorage>,
    mut leaves: ResMut<IvyLeafStorage>,
    mut rng: ResMut<IvyLeafRng>,
    noise: Res<IvySpawnNoise>,
    time: Res<GameTime>,
) {
    if !settings.ivy_leaves {
        return;
    }
    for message in messages.read() {
        spawn_segment_leaf(
            message,
            &mut storage,
            &mut leaves,
            &mut rng,
            &noise,
            time.frame_count,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use perchance::PerchanceContext;

    #[test]
    fn glade_settings_controls_actual_ivy_leaf_event_consumption() {
        let directory = tempfile::tempdir().unwrap();
        let settings_path = directory.path().join("settings.json");
        std::fs::write(&settings_path, r#"{"ivy_leaves":false}"#).unwrap();
        let mut world = World::new();
        crate::systems::ivy::registration::register_ivy_storage(&mut world);
        world.insert_resource(GladeSettings::load(directory.path()));
        world.insert_resource(GameTime {
            frame_count: 19,
            ..Default::default()
        });
        let message = {
            let mut storage = world.resource_mut::<IvyStorage>();
            let origin = Vec3::new(-44.5, 1728.0, 17.0);
            let ivy = storage.spawn_ivy(origin, Vec3::Z, 0.0..1.0, 3..4, 12);
            let b = storage
                .get_mut(ivy)
                .unwrap()
                .add_point(origin + Vec3::X, Vec3::Z);
            IvySegmentDoneGrowingMsg {
                ivy,
                point_a: 0,
                point_b: b,
            }
        };
        let mut expected_storage = world.resource::<IvyStorage>().clone();
        let mut expected_leaves = IvyLeafStorage::new();
        let mut expected_rng = IvyLeafRng::default();
        for _ in 0..32 {
            world
                .resource_mut::<Events<IvySegmentDoneGrowingMsg>>()
                .send(message);
            spawn_segment_leaf(
                &message,
                &mut expected_storage,
                &mut expected_leaves,
                &mut expected_rng,
                &IvySpawnNoise::new(),
                19,
            );
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(ivy_leaf_spawner);
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<IvyLeafStorage>().leaf_count(), 0);
        std::fs::write(&settings_path, r#"{"ivy_leaves":true}"#).unwrap();
        world.insert_resource(GladeSettings::load(directory.path()));
        schedule.run(&mut world);
        assert!(expected_leaves.leaf_count() > 0);
        assert_eq!(
            world.resource::<IvyLeafStorage>().get(message.ivy),
            expected_leaves.get(message.ivy)
        );
        assert_eq!(
            world.resource_mut::<IvyStorage>().rng_mut().uniform_f32(),
            expected_storage.rng_mut().uniform_f32()
        );
        assert_eq!(
            world.resource_mut::<IvyLeafRng>().0.uniform_f32(),
            expected_rng.0.uniform_f32()
        );
        let count = world.resource::<IvyLeafStorage>().leaf_count();
        schedule.run(&mut world);
        assert_eq!(world.resource::<IvyLeafStorage>().leaf_count(), count);
    }

    #[test]
    fn generated_leaf_follows_original_disc_normal_and_rotation_order() {
        let mut storage = IvyStorage::new();
        // B's leaf sample 2*p + (89,-3456,-34) is exactly zero here.
        // Perlin's lattice origin gives 0, strictly above the -0.05 gate.
        let origin = Vec3::new(-44.5, 1728.0, 17.0);
        let id = storage.spawn_ivy(origin, Vec3::Z, 0.0..1.0, 3..4, 93);
        let b = storage
            .get_mut(id)
            .unwrap()
            .add_point(origin + Vec3::X, Vec3::Z);
        let message = IvySegmentDoneGrowingMsg {
            point_a: 0,
            point_b: b,
            ivy: id,
        };
        let noise = IvySpawnNoise::new();
        let mut leaves = IvyLeafStorage::new();
        let mut rng = IvyLeafRng::default();
        for _ in 0..100 {
            spawn_segment_leaf(&message, &mut storage, &mut leaves, &mut rng, &noise, 41);
            if leaves.leaf_count() != 0 {
                break;
            }
        }
        let leaf = &leaves.get(id).expect("selection accepted a segment")[0];
        let mut original_rng = PerchanceContext::new(10);
        let disc = original_rng.uniform_disc_vec2() * 0.15;
        // 1409969d1..aa9: cross(+X,+Z)=-Y, with disc.y along the segment.
        assert_eq!(
            leaf.position,
            Vec3::new(origin.x + disc.y, origin.y - disc.x, origin.z)
        );
        let n0 = original_rng.normal_f32().clamp(0.0, 1.0);
        let n1 = original_rng.normal_f32().clamp(0.0, 1.0);
        let n2 = original_rng.normal_f32().clamp(0.0, 1.0);
        let expected_jitter = Vec3::new(
            (n0 - (1.0 - n0)) * 0.5,
            (n2 - (1.0 - n2)) * 0.5,
            (n1 - (1.0 - n1)) * 0.5 + 1.0,
        )
        .normalize();
        assert!(leaf.vector_58.abs_diff_eq(expected_jitter, 1e-6));
        // 140996c03..c41 is normal/1.5 clamped to [-1,1], times 80 degrees;
        // y-axis quaternion half-angle. For +Z the independent result is
        // (sin(full_angle),0,cos(full_angle)) before the y drop.
        let angle =
            (original_rng.normal_f32() / 1.5).clamp(-1.0, 1.0) * 80.0 * f32::from_bits(0x3c8e_fa35);
        let drop = (original_rng.normal_f32().abs() / 1.5).min(1.0);
        let size_mix = (original_rng.normal_f32().abs() / 3.0).min(1.0);
        let expected_facing = Vec3::new(angle.sin(), -drop, angle.cos()).normalize();
        assert!(leaf.vector_2c.abs_diff_eq(expected_facing, 1e-6));
        assert_eq!(leaf.size, (1.0 - size_mix) * 0.8 + size_mix * 1.2);
        assert_eq!(leaf.growth_delay, original_rng.uniform_f32() * 0.3);
        assert_eq!(leaf.branch_direction, Vec3::X);
        assert_eq!(
            (
                leaf.ivy,
                leaf.point_a,
                leaf.point_b,
                leaf.frame,
                leaf.source_18
            ),
            (id, 0, b, 41, 93)
        );
        assert_eq!((leaf.growth, leaf.branch_thickness), (0.0, 0.0));
        assert_eq!(rng.0.uniform_f32(), original_rng.uniform_f32());
    }
}
