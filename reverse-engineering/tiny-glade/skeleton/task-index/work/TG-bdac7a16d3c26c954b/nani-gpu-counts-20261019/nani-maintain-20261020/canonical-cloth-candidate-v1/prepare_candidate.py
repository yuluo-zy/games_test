from pathlib import Path
import json
import hashlib
import difflib

REPO = Path('/Users/yuluo/Desktop/game/games_test')
OUT = Path(__file__).parent
DESIGN = OUT.parent / 'canonical-cloth-design-v1'
sources = json.loads((DESIGN / 'SOURCE_SNAPSHOT_V1.json').read_text())['sources']
expected = {Path(x['source']): x['sha256'] for x in sources}
sha = lambda b: hashlib.sha256(b).hexdigest()

def read(rel):
    p = REPO / 'tiny-glade' / rel
    b = p.read_bytes()
    assert sha(b) == expected[p], (p, sha(b), expected[p])
    return p, b.decode()

def replace(s, old, new):
    assert s.count(old) == 1, (s.count(old), old[:90])
    return s.replace(old, new)

files = []
def finish(p, before, after):
    name = p.name
    for suffix, text in [('.before', before), ('.candidate', after)]:
        q = OUT / (name + suffix)
        assert not q.exists()
        q.write_text(text)
    patch = ''.join(difflib.unified_diff(before.splitlines(True), after.splitlines(True),
        fromfile=str(p), tofile=str(p)))
    (OUT / (name + '.patch')).write_text(patch)
    files.append({'source': str(p), 'before_sha256': sha(before.encode()),
                  'candidate': name + '.candidate', 'candidate_sha256': sha(after.encode()),
                  'patch': name + '.patch', 'patch_sha256': sha(patch.encode())})

p, before = read('crates/country-core/src/components/drawable.rs')
after = before + '''
/// B ClothlineCloth: twelve scalar bytes read by14207c0a0 and copied by the
/// real decorator builder. Field names/schema are not recovered; these are
/// explicit value words, not ClothInfo or WallFlagCloth. No default payload
/// or original ABI promise is provided. B14084cc00 selects Clone handling.
#[derive(bevy_ecs::component::Component, Clone, Debug)]
pub struct ClothlineCloth([u32; 3]);

impl ClothlineCloth {
    pub const fn from_words(words: [u32; 3]) -> Self {
        Self(words)
    }

    pub const fn words(&self) -> [u32; 3] {
        self.0
    }
}
'''
finish(p, before, after)

p, before = read('crates/systems/decorator/src/utils/visual_entity_builder.rs')
after = replace(before, '''    raw_component!(
        /// B `components::ClothlineCloth`（u64 + u32，12 B，U）。
        ClothlineCloth,
        12
    );''', '''    // One canonical12-byte value, also read by the actual Nani ECS query.
    pub use country_core::components::drawable::ClothlineCloth;''')
after = replace(after, '''    raw_component!(
        /// B `components::drawable::NormalMap`（8 B 句柄，U）。
        NormalMap,
        8
    );''', '''    // Existing disk-texture NameHash owner, not a local raw component.
    pub use country_core::components::NormalMap;''')
after = replace(after, '''    /// `clothline`（行 147）：打开 `ClothlineCloth`（+0x80==1）；载荷 U（0,0）。
    pub fn clothline(&mut self) {
        self.clothline_cloth = Some((0, 0));
    }''', '''    /// Supply all original value words explicitly; no zero/default producer.
    /// This host API spelling does not claim the original Rust declaration.
    pub fn clothline(&mut self, cloth: ClothlineCloth) {
        let [a, b, tail] = cloth.words();
        self.clothline_cloth = Some((u64::from(a) | (u64::from(b) << 32), tail));
    }''')
after = replace(after, '''            let mut payload = [0u8; 12];
            payload[0..8].copy_from_slice(&a.to_le_bytes());
            payload[8..12].copy_from_slice(&b.to_le_bytes());
            commands.entity(entity).insert(ClothlineCloth(payload));''', '''            commands.entity(entity).insert(ClothlineCloth::from_words([
                a as u32, (a >> 32) as u32, b,
            ]));''')
after = replace(after, '.insert(NormalMap(handle.to_le_bytes()));', '.insert(NormalMap(handle));')
needle = '''    #[test]
    fn real_build_skips_three_native_none_tags_in_some_payloads()'''
tests = '''    #[test]
    fn real_cloth_build_preserves_explicit_words_and_canonical_normal_identity() {
        use bevy_ecs::world::{CommandQueue, World};
        use country_core::components::drawable::ClothlineCloth as CanonicalCloth;
        use country_core::components::NormalMap as CanonicalNormal;
        assert_eq!(core::any::TypeId::of::<pending_core::ClothlineCloth>(),
            core::any::TypeId::of::<CanonicalCloth>());
        assert_eq!(core::any::TypeId::of::<pending_core::NormalMap>(),
            core::any::TypeId::of::<CanonicalNormal>());
        for words in [[0x3e80_0000, 0xbe00_0000, 37], [0x7fc0_0123, 0x8000_0000, u32::MAX]] {
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            let mut state = DecoratorTransientVisualState::default();
            let mut builder = WindowVisualEntityBuilder::new(
                Vec3::new(2.0, 3.0, 4.0), Quat::from_rotation_y(0.37), 61);
            builder.mesh_name = "decorators/setdressing_clothline_cloth";
            builder.nani_name = "nani_cloth";
            let cloth = CanonicalCloth::from_words(words);
            assert_eq!(cloth.clone().words(), words);
            builder.clothline(cloth);
            builder.normal_map = Some(0xb9fb_cd49_9788_b2f1);
            let entity = builder.build(&mut Commands::new(&mut queue, &world), 61,
                &AssetMeshLibrary::new(), &AssetShaderLibrary::new(), &mut state);
            queue.apply(&mut world);
            assert_eq!(world.get::<CanonicalCloth>(entity).unwrap().words(), words);
            assert_eq!(world.get::<CanonicalNormal>(entity).unwrap().0, 0xb9fb_cd49_9788_b2f1);
            assert_eq!(world.get::<NaniInstanceDesc>(entity).unwrap().subset_kind, GpuWorldSubsetKind::Cloth);
            assert!(world.get::<InstanceDataMarker<ClothInstanceData>>(entity).is_some());
            assert_eq!(state.entities_of(61), Some([entity].as_slice()));
        }
    }

    #[test]
    fn absent_cloth_payload_is_not_fabricated_by_real_build() {
        use bevy_ecs::world::{CommandQueue, World};
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut state = DecoratorTransientVisualState::default();
        let mut builder = WindowVisualEntityBuilder::new(Vec3::X, Quat::IDENTITY, 63);
        builder.nani_name = "nani_cloth";
        builder.mesh_name = "decorators/setdressing_clothline_cloth";
        let entity = builder.build(&mut Commands::new(&mut queue, &world), 63,
            &AssetMeshLibrary::new(), &AssetShaderLibrary::new(), &mut state);
        queue.apply(&mut world);
        assert!(world.get::<ClothlineCloth>(entity).is_none());
        assert!(world.get::<NormalMap>(entity).is_none());
    }

'''
after = replace(after, needle, tests + needle)
finish(p, before, after)

p, before = read('crates/country-core/src/render/nani/systems.rs')
after = replace(before, '''// workspace. The remaining eleven payload contracts are explicit gaps; a new
// descriptor for one of those kinds fails before an uninitialized instance can''', '''// workspace. Remaining unavailable payload/producer contracts stay explicit;
// a new descriptor outside these bindings fails before an uninitialized instance can''')
after = replace(after, '''components::{FlatLighting, SemiTransparent, drawable::{AlbedoMap, PrevTransform, NaniSolidTexture}},''',
'''components::{FlatLighting, NormalMap, SemiTransparent,
        drawable::{AlbedoMap, ClothlineCloth, PrevTransform, NaniSolidTexture}},''')
after = replace(after, '''    Has<InstanceDataMarker<StoneFloorInstanceData>>,
);''', '''    Has<InstanceDataMarker<StoneFloorInstanceData>>,
    Has<InstanceDataMarker<ClothInstanceData>>,
);''')
after = replace(after, '''type MappedNaniMarkerPresence = (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);
type MappedNaniInputs = (MappedNaniMarkers, Has<ColorId>, Has<PrevTransform>, Has<Transform>,
    Has<NaniSolidTexture>, Has<Roof>);''', '''type MappedNaniMarkerPresence = (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);
type MappedNaniInputs = (MappedNaniMarkers, Has<ColorId>, Has<PrevTransform>, Has<Transform>,
    Has<NaniSolidTexture>, Has<Roof>, Has<ClothlineCloth>, Has<NormalMap>);''')
after = replace(after, '''    (markers, has_color, has_previous, has_transform, has_selector, has_roof):
        (MappedNaniMarkerPresence, bool, bool, bool, bool, bool),''', '''    (markers, has_color, has_previous, has_transform, has_selector, has_roof, has_cloth, has_normal):
        (MappedNaniMarkerPresence, bool, bool, bool, bool, bool, bool, bool),''')
after = replace(after, '''        GpuWorldSubsetKind::RoofFloor => markers.9 && has_roof,
        _ => false,''', '''        GpuWorldSubsetKind::RoofFloor => markers.9 && has_roof,
        GpuWorldSubsetKind::Cloth => markers.10 && has_cloth && has_normal,
        _ => false,''')
after = replace(after, 'markers.4, markers.5, markers.6, markers.7, markers.8, markers.9]',
    'markers.4, markers.5, markers.6, markers.7, markers.8, markers.9, markers.10]')
needle = 'pub fn nani_update_uber_ecs(\n'
adapter = '''/// Original tuple13/kind20: the same canonical12-byte cloth words and
/// disk-texture name owner emitted by the real decorator builder. With-only
/// required payload Query; no Changed gate or parallel instance owner.
pub fn nani_update_cloth_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &ClothlineCloth, &NormalMap),
        With<InstanceDataMarker<ClothInstanceData>>>,
    textures: Res<AssetTextureLibrary>, gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Cloth);
    for (mut key, transform, cloth, normal_map) in &mut rows {
        let asset = textures.get_handle_and_texture_by_name(NameHash(normal_map.0))
            .expect("called `Option::unwrap()` on a `None` value").0;
        let normal_index = textures.handle_to_bindless_index(asset).0;
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_cloth(r, p, s, cloth.words(), normal_index), |raw| ClothInstanceData { raw });
    }
}

'''
after = replace(after, needle, adapter + needle)
after = replace(after, '''/// Nine complete current component mappings plus a restricted Roof-bearing
/// kind9 branch are installed. Floor-only and other unavailable contracts
/// fail explicitly; this is not ten complete original kind contracts.''', '''/// The previously integrated nine mappings, canonical Cloth binding and
/// restricted Roof-bearing kind9 branch are installed. Floor-only and other
/// unavailable contracts fail explicitly. A binding does not establish the
/// separate garment producer or whole-scene reachability.''')
after = replace(after, '''         nani_update_balcony_door_ecs,
         nani_update_uber_ecs,''', '''         nani_update_balcony_door_ecs,
         nani_update_cloth_ecs,
         nani_update_uber_ecs,''')
tests = '''    fn cloth_world(dev: &MockDevice) -> World {
        use crate::render::test_device::TestDevice;
        use crate::rhapsody_types::Texture;
        let mut gpu = GpuWorld::new(dev);
        gpu.assign_subset(GpuWorldSubsetKind::Cloth, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<ClothInstanceData>(dev, GpuWorldSubsetKind::Cloth)));
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("cloth CPU boundary triangle");
        mesh.set_attribute(crate::render::mesh::ATTR_POSITION,
            vec![Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 3.0)]);
        mesh.set_indices(vec![0, 1, 2]);
        atlas.add(GpuWorldSubsetKind::Cloth, "decorators/setdressing_clothline_cloth", &mesh);
        let mut textures = AssetTextureLibrary::new(&TestDevice(91));
        let tex = |id| Texture { id, width: 4, height: 4, format: 0x25 };
        let first = textures.add(Some("cloth_cpu_first".into()), tex(91));
        textures.add(Some("cloth_cpu_unused".into()), tex(92));
        let actual = textures.add(Some("cloth_cpu_normal".into()), tex(93));
        assert_eq!(textures.handle_to_bindless_index(first).0, 0);
        assert_eq!(actual.0, 2); // actual descriptor below is1, not this asset index
        let mut world = World::new();
        world.insert_resource(gpu);
        world.insert_resource(atlas);
        world.insert_resource(textures);
        world
    }

    fn spawn_cpu_cloth(world: &mut World) -> Entity {
        world.spawn((crate::components::drawable::NaniBundle::<ClothInstanceData> {
            desc: NaniInstanceDesc { mesh: NaniMeshSelector::Named(
                NameHash::compute("decorators/setdressing_clothline_cloth")), subset_kind: GpuWorldSubsetKind::Cloth },
            transform: Transform { rotation: glam::Quat::from_rotation_y(0.37),
                translation: Vec3::new(2.0, 3.0, 4.0), scale: Vec3::new(1.2, 0.8, 2.5) },
            marker: InstanceDataMarker::default(),
        }, ClothlineCloth::from_words([0x3e80_0000, 0xbe00_0000, 37]),
            NormalMap(NameHash::compute("cloth_cpu_normal").0))).id()
    }

    fn actual_cloth_payload(world: &World, entity: Entity) -> [u32; 16] {
        let key = world.get::<InstanceKey>(entity).unwrap();
        world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth)
            .data().as_any().downcast_ref::<
                crate::render::nani::gpu_world::InstanceDataBuffer<ClothInstanceData>>()
            .unwrap().shadow.data[key.index as usize].raw
    }

    #[test]
    fn cloth_uses_canonical_payload_actual_texture_owner_barrier_flush_and_removal() {
        let dev = MockDevice::default();
        let mut world = cloth_world(&dev);
        let entity = spawn_cpu_cloth(&mut world);
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_cloth_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(entity).unwrap();
        assert_eq!(key.kind, GpuWorldSubsetKind::Cloth);
        assert_eq!(&actual_cloth_payload(&world, entity)[12..], &[0x3e80_0000, 0xbe00_0000, 37, 1]);
        let textures = world.resource::<AssetTextureLibrary>();
        let asset = textures.get_handle_and_texture_by_name(NameHash::compute("cloth_cpu_normal")).unwrap().0;
        assert_eq!(asset.0, 2);
        assert_eq!(textures.handle_to_bindless_index(asset).0, 1);
        assert_eq!(textures.pending_bindless().len(), 2);
        {
            let subset = world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth);
            assert_eq!(subset.instance_count(), 1);
            assert_eq!(subset.positions()[key.index as usize].position, Vec3::new(2.0, 3.0, 4.0));
            assert_eq!(subset.positions_dirty().to_range(), Some(0..1));
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
        assert!(dev.writes.borrow().iter().any(|(ranges, offset, bytes)|
            ranges == &[0..64] && *offset == 0 && *bytes == 64));
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 1);
        world.clear_trackers();
        world.entity_mut(entity).insert(ClothlineCloth::from_words([0x8000_0000, 0x7fc0_0123, 99]));
        schedule.run(&mut world);
        assert_eq!(world.get::<InstanceKey>(entity).unwrap().id, key.id);
        assert_eq!(&actual_cloth_payload(&world, entity)[12..], &[0x8000_0000, 0x7fc0_0123, 99, 1]);
        assert_eq!(world.resource::<AssetTextureLibrary>().pending_bindless().len(), 2);
        world.clear_trackers();
        world.despawn(entity);
        schedule.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth).instance_count(), 0);
    }

    #[test]
    fn cloth_missing_canonical_inputs_fail_before_key_and_count_publication() {
        for missing in ["cloth", "normal", "transform", "marker", "ambiguous"] {
            let dev = MockDevice::default();
            let mut world = cloth_world(&dev);
            let entity = spawn_cpu_cloth(&mut world);
            match missing {
                "cloth" => { world.entity_mut(entity).remove::<ClothlineCloth>(); }
                "normal" => { world.entity_mut(entity).remove::<NormalMap>(); }
                "transform" => { world.entity_mut(entity).remove::<Transform>(); }
                "marker" => { world.entity_mut(entity).remove::<InstanceDataMarker<ClothInstanceData>>(); }
                _ => { world.entity_mut(entity).insert(InstanceDataMarker::<PlantInstanceData>::default()); }
            }
            let mut schedule = Schedule::default();
            schedule.add_systems(nani_maintain_instances_ecs);
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                schedule.run(&mut world))).is_err(), "accepted missing {missing}");
            assert!(world.get::<InstanceKey>(entity).is_none());
            let gpu = world.resource::<GpuWorld>();
            assert_eq!(gpu.total_instance_count, 0);
            assert_eq!(gpu.owned_subset_mut(GpuWorldSubsetKind::Cloth).instance_count(), 0);
        }
    }

    #[test]
    fn cloth_unknown_texture_name_fails_in_actual_update() {
        let dev = MockDevice::default();
        let mut world = cloth_world(&dev);
        let entity = spawn_cpu_cloth(&mut world);
        world.entity_mut(entity).insert(NormalMap(0x9abc_def0_1234_5678));
        let mut allocation = Schedule::default();
        allocation.add_systems((nani_maintain_instances_ecs, ApplyDeferred).chain_ignore_deferred());
        allocation.run(&mut world);
        let before_writes = dev.writes.borrow().len();
        let mut update = Schedule::default();
        update.add_systems(nani_update_cloth_ecs);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            update.run(&mut world))).is_err());
        assert_eq!(dev.writes.borrow().len(), before_writes); // no flush/success after failure
        assert_eq!(world.resource::<AssetTextureLibrary>().pending_bindless().len(), 1);
    }

    #[test]
    fn empty_cloth_query_still_requires_owned_subset_and_preserves_zero_count() {
        let dev = MockDevice::default();
        let mut world = cloth_world(&dev);
        let before_writes = dev.writes.borrow().len();
        let mut update = Schedule::default();
        update.add_systems(nani_update_cloth_ecs);
        update.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth).instance_count(), 0);
        assert_eq!(dev.writes.borrow().len(), before_writes);
        world.insert_resource(GpuWorld::new(&dev));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            update.run(&mut world))).is_err());
    }

'''
needle = '    fn world_with(kind: GpuWorldSubsetKind, dev: &MockDevice) -> GpuWorld {\n'
after = replace(after, needle, tests + needle)
finish(p, before, after)
(OUT / 'FILES_V1.json').write_text(json.dumps(files, indent=2) + '\n')
print(json.dumps({'candidate_sources': len(files), 'new_tests': 6, 'compiled_or_run': False}))
