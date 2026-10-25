//! Native command/placement/ECS boundary for the existing prefab algorithms.
//! B140840610/140841b00 use Entity identity and one complete placement record.
use super::*;
use crate::resources::session::ActiveSession;
use bevy_ecs::prelude::*;

/// Native PrefabsToDelete is HashSet<Entity>, as shown by the actual PDB
/// EventIterator<DespawnPrefabCmd> map/extend instantiation, not GlobalId.
#[derive(Resource, Default)]
pub struct PrefabsToDelete(pub HashSet<Entity>);

#[allow(clippy::too_many_arguments)]
pub fn spawn_prefab_commands(
    library: Res<PrefabLibrary>,
    atlases: Res<MeshAtlasLibrary>,
    session: Res<ActiveSession>,
    mut gpu: ResMut<GpuWorld>,
    mut audio: ResMut<AudioTriggerLibrary>,
    mut rt: ResMut<RtWorld>,
    mut spawns: EventReader<SpawnPrefabCmd>,
    mut commands: Commands,
    mut despawns: EventReader<DespawnPrefabCmd>,
    mut issued: ResMut<PrefabsToDelete>,
) -> bevy_ecs::error::Result {
    issued.0.clear();
    issued
        .0
        .extend(despawns.read().filter_map(|command| command.entity));
    for command in spawns.read() {
        let entity = command
            .entity
            .ok_or_else(|| anyhow::anyhow!("SpawnPrefabCmd requires its reserved Entity"))?;
        if issued.0.contains(&entity) {
            log::warn!(target: LOG_TARGET, "Issued creation of a prefab {:?} on the same frame as issuing its deletion!", entity);
            continue;
        }
        let name = command
            .prefab_name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("SpawnPrefabCmd requires its PrefabName"))?;
        let prefab = library.get(name);
        let placement = command.placement();
        let transform = PrefabTransform {
            translation: placement.position,
            scale: placement.scale,
            rotation: placement.rotation_angle,
        };
        // 140841b00 reads cull position/scale and GPU record from the SAME
        // pointer. Scale is the original bound radius, with no inferred AABB.
        let cull = CullInstanceSource {
            position: placement.position,
            radius: placement.scale,
        };
        let mut data = PrefabInstance {
            prefab_name: name.clone(),
            ..Default::default()
        };
        let mut pending = Vec::new();
        let havens = build_prefab_contents(
            &mut data,
            prefab,
            &transform,
            &atlases,
            &mut gpu,
            &mut rt,
            &mut audio,
            &mut pending,
            session.meta.glade.native_tag() as usize,
            cull,
            placement,
        );
        // Native callback1412dbe50 is insert<BirdHaven>, as proved by its
        // PDB monomorphization. It is unrelated to an RT worker queue. Radius
        // and center are LOCAL part bounds; the original inserts once for
        // each odd rt_kind, so the final matching part wins on this entity.
        for (handle, radius) in havens {
            let index = data
                .rt_meshes
                .iter()
                .position(|candidate| *candidate == handle)
                .expect("new prefab RT handle is owned by this entity");
            let part = prefab
                .parts
                .iter()
                .filter(|part| part.rt_kind != 2)
                .nth(index)
                .expect("RT records follow the original prefab part order");
            commands
                .entity(entity)
                .insert(crate::resources::tree_storage::BirdHaven::new(
                    part.bounds.center(),
                    radius,
                ));
        }
        commands
            .entity(entity)
            .insert((PrefabInstanceData(placement), data));
    }
    Ok(())
}

pub fn update_prefab_commands(
    mut query: Query<(&PrefabInstanceData, &mut PrefabInstance), Changed<PrefabInstanceData>>,
    mut gpu: ResMut<GpuWorld>,
    mut rt: ResMut<RtWorld>,
    mut audio: ResMut<AudioTriggerLibrary>,
) {
    for (placement, data) in &mut query {
        let placement = placement.0;
        let transform = PrefabTransform {
            translation: placement.position,
            scale: placement.scale,
            rotation: placement.rotation_angle,
        };
        update_prefabs_instance(&data, &transform, &mut gpu, &mut rt, &mut audio, placement);
    }
}

pub fn despawn_prefab_commands(
    mut commands: Commands,
    mut events: EventReader<DespawnPrefabCmd>,
    mut query: Query<&mut PrefabInstance>,
    gpu: Res<GpuWorld>,
    mut rt: ResMut<RtWorld>,
    mut audio: ResMut<AudioTriggerLibrary>,
) {
    for command in events.read() {
        let Some(entity) = command.entity else {
            continue;
        };
        if let Ok(mut data) = query.get_mut(entity) {
            despawn_prefab_contents(&mut data, &gpu, &mut rt, &mut audio);
        }
        if let Ok(mut entity) = commands.get_entity(entity) {
            entity.despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{
        nani::gpu_world::{GpuWorldSubset, OwnedGpuWorldSubset},
        test_device::TestDevice,
    };
    use crate::resources::{
        prefab::{PrefabAudioTrigger, PrefabPart},
        session::{SessionId, SessionMeta},
    };

    #[test]
    fn actual_ecs_spawn_move_delete_preserves_placement_and_all_owned_contents() {
        let mut world = World::new();
        let device = TestDevice(1);
        let kind = GpuWorldSubsetKind::Canopy;
        let mut gpu = GpuWorld::new(&device);
        gpu.assign_subset(
            kind,
            GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<PlacementDataSsbo>(&device, kind, 1)),
        );
        world.insert_resource(gpu);
        let name = PrefabName::new("tree/runtime-test");
        let bounds = Aabb3 {
            min: Vec3::ZERO,
            max: Vec3::new(2.0, 4.0, 8.0),
        };
        let mut library = PrefabLibrary::default();
        library.add(
            name.clone(),
            Prefab {
                parts: vec![PrefabPart {
                    audio_trigger: Some(PrefabAudioTrigger {
                        kind_name: "Tree".into(),
                        bounds,
                    }),
                    bounds,
                    rt_kind: 1,
                    ..Default::default()
                }],
            },
        );
        world.insert_resource(library);
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("test");
        mesh.set_indices(vec![0, 1, 2]);
        atlas.add(kind, name.clone(), &mesh);
        world.insert_resource(atlas);
        world.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        world.init_resource::<AudioTriggerLibrary>();
        world.init_resource::<RtWorld>();
        world.init_resource::<PrefabsToDelete>();
        world.init_resource::<Events<SpawnPrefabCmd>>();
        world.init_resource::<Events<DespawnPrefabCmd>>();
        let entity = world.spawn_empty().id();
        let command = SpawnPrefabCmd {
            entity: Some(entity),
            prefab_name: Some(name),
            position: Vec3::new(8.0, 2.0, 9.0),
            scale: 0.75,
            rotation: 0.4,
            variant: 19,
            one: 0.6,
            flags: [1, 2, 3, 4, 0, 0, 128, 63, 0, 0, 128, 62, 0, 0, 0, 63],
            mode: 3,
        };
        let expected = command.placement();
        world.send_event(command.clone());
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                spawn_prefab_commands,
                update_prefab_commands,
                despawn_prefab_commands,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert_eq!(world.get::<PrefabInstanceData>(entity).unwrap().0, expected);
        let data = world.get::<PrefabInstance>(entity).unwrap();
        assert_eq!(
            (
                data.gpu_instances.len(),
                data.rt_meshes.len(),
                data.audio_triggers.len()
            ),
            (1, 1, 1)
        );
        assert_eq!(data.audio_triggers[0].shape, bounds);
        let handle = data.audio_triggers[0].handle.unwrap();
        assert_eq!(
            world
                .resource::<AudioTriggerLibrary>()
                .get_triggers(AudioTriggerKind::Primary)
                .triggers[handle],
            bounds
                .scaled(Vec3::splat(0.75))
                .translated(expected.position)
        );
        let haven = world
            .get::<crate::resources::tree_storage::BirdHaven>(entity)
            .unwrap();
        assert_eq!(haven.sphere, (bounds.center(), 1.0));
        assert_eq!(haven.cooldown, 10.0);
        assert_eq!(haven.observed_state, 0);
        assert!(!haven.observed_flag);
        world
            .get_mut::<PrefabInstanceData>(entity)
            .unwrap()
            .0
            .position = Vec3::new(4.0, 6.0, 7.0);
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<AudioTriggerLibrary>()
                .get_triggers(AudioTriggerKind::Primary)
                .triggers[handle],
            bounds
                .scaled(Vec3::splat(0.75))
                .translated(Vec3::new(4.0, 6.0, 7.0))
        );
        // Same-frame spawn+delete must suppress creation using Entity identity.
        let count = world.resource::<GpuWorld>().total_instance_count;
        world.send_event(command);
        world.send_event(DespawnPrefabCmd {
            entity: Some(entity),
            _gap: 0,
        });
        schedule.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().total_instance_count, count);
        assert!(world.get_entity(entity).is_err());
        assert!(world.resource::<RtWorld>().meshes.is_empty());
        assert!(
            world
                .resource::<AudioTriggerLibrary>()
                .get_triggers(AudioTriggerKind::Primary)
                .triggers
                .is_empty()
        );
    }
}
