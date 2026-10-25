from pathlib import Path
import hashlib
import difflib

w = Path(__file__).parent
source = w / 'visual_entity_builder.rs.candidate'
text = source.read_text()
assert hashlib.sha256(source.read_bytes()).hexdigest() == 'b742fd24ece7a09dd13545641077a96d99859ca36af1268a6b8efef7a50d72c1'
needle = '    #[test]\n    fn absent_cloth_payload_is_not_fabricated_by_real_build()'
assert text.count(needle) == 1
test = '''    #[test]
    fn actual_builder_commands_feed_same_world_cloth_maintain_update_and_flush() {
        use bevy_ecs::{schedule::{ApplyDeferred, IntoScheduleConfigs, Schedule},
            world::{CommandQueue, World}};
        use country_core::{asset_libraries::texture_library::{AssetTextureLibrary,
            DescriptorImageUpdate, ImageView, TextureLibraryDevice, TextureUpdate},
            render::{load_texture::boundary::TextureDevice, mesh::Mesh,
                nani::{gpu_world::{GpuWorld, GpuWorldSubset, InstanceDataBuffer, InstanceKey, OwnedGpuWorldSubset},
                    systems::{nani_maintain_instances_ecs, nani_update_cloth_ecs,
                        nani_flush_instance_updates}},
                ssbo::rhapsody_boundary::{Buffer, BufferDesc, GraphicsDevice}},
            resources::render::mesh_atlas_library::MeshAtlasLibrary,
            rhapsody_contract::DescriptorSet,
            rhapsody_types::{BackendError, Texture, TextureDesc}};
        use std::cell::{Cell, RefCell};
        // Recording CPU boundary only. It neither creates nor claims a GPU;
        // texture upload/publish is forbidden here, and full rows are asserted.
        #[derive(Default)]
        struct RecordingDevice { next: Cell<u64>, writes: RefCell<Vec<Vec<u8>>> }
        impl GraphicsDevice for RecordingDevice {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>)
                -> Result<Buffer, BackendError> {
                let id = self.next.get() + 1;
                self.next.set(id);
                Ok(Buffer { id, generation: 1, byte_size: desc.size as u64, _gap10: 0, _gap20: 0 })
            }
            fn update_buffer_bytes(&self, _: &Buffer, data: &[u8],
                _: &[core::ops::Range<usize>], _: usize) -> Result<(), BackendError> {
                self.writes.borrow_mut().push(data.to_vec());
                Ok(())
            }
            fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
                let page = Box::leak(vec![0u128; (buffer.byte_size as usize).div_ceil(16)].into_boxed_slice());
                Some(unsafe { core::slice::from_raw_parts_mut(page.as_mut_ptr().cast::<u8>(),
                    buffer.byte_size as usize) })
            }
        }
        impl TextureDevice for RecordingDevice {
            fn create_texture(&self, _: &str, _: &TextureDesc, _: &[u8]) -> Result<Texture, BackendError> {
                panic!("CPU test registers explicit texture values; upload is outside this boundary")
            }
        }
        impl TextureLibraryDevice for RecordingDevice {
            fn create_persistent_descriptor_set(&self, _: &str) -> Result<DescriptorSet, BackendError> {
                Ok(DescriptorSet::default())
            }
            fn update_persistent_descriptor_set(&self, _: &DescriptorSet, _: &[DescriptorImageUpdate])
                -> Result<(), BackendError> { panic!("no GPU descriptor publication in CPU test") }
            fn get_vk_image_view(&self, _: &Texture, _: u32) -> Result<ImageView, BackendError> {
                panic!("no native image view in CPU test")
            }
            fn update_texture(&self, _: &Texture, _: &TextureUpdate) -> Result<(), BackendError> {
                panic!("no GPU texture update in CPU test")
            }
        }
        let device = RecordingDevice::default();
        let mut gpu = GpuWorld::new(&device);
        gpu.assign_subset(GpuWorldSubsetKind::Cloth, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<ClothInstanceData>(&device, GpuWorldSubsetKind::Cloth)));
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("same-World cloth CPU triangle");
        mesh.set_attribute(country_core::render::mesh::ATTR_POSITION,
            vec![Vec3::X, Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 3.0)]);
        mesh.set_indices(vec![0, 1, 2]);
        atlas.add(GpuWorldSubsetKind::Cloth, "decorators/setdressing_clothline_cloth", &mesh);
        let mut textures = AssetTextureLibrary::new(&device);
        let tex = |id| Texture { id, width: 4, height: 4, format: 0x25 };
        let first = textures.add(Some("first_normal".into()), tex(81));
        textures.add(Some("unused_normal".into()), tex(82));
        let normal = textures.add(Some("actual_cloth_normal".into()), tex(83));
        assert_eq!(normal.0, 2);
        assert_eq!(textures.handle_to_bindless_index(first).0, 0);
        let mut world = World::new();
        world.insert_resource(gpu);
        world.insert_resource(atlas);
        world.insert_resource(textures);
        let mut queue = CommandQueue::default();
        let mut state = DecoratorTransientVisualState::default();
        let mut builder = WindowVisualEntityBuilder::new(Vec3::new(3.0, 4.0, 5.0),
            Quat::from_rotation_y(0.31), 67);
        builder.mesh_name = "decorators/setdressing_clothline_cloth";
        builder.nani_name = "nani_cloth";
        builder.scale_override = Some(Vec3::new(1.3, 0.7, 2.5));
        builder.clothline(ClothlineCloth::from_words([0x3e80_0000, 0xbe00_0000, 71]));
        builder.normal_map = Some(NameHash::compute("actual_cloth_normal").0);
        let entity = builder.build(&mut Commands::new(&mut queue, &world), 67,
            &AssetMeshLibrary::new(), &AssetShaderLibrary::new(), &mut state);
        queue.apply(&mut world);
        assert!(world.get::<InstanceKey>(entity).is_none());
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_cloth_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(entity).unwrap();
        {
            let subset = world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth);
            let row = subset.data().as_any().downcast_ref::<InstanceDataBuffer<ClothInstanceData>>()
                .unwrap().shadow.data[key.index as usize].raw;
            let expected = country_core::render::nani::instance_updates::build_cloth(
                builder.rotation, builder.translation, builder.scale_override.unwrap(),
                [0x3e80_0000, 0xbe00_0000, 71], 1).words;
            assert_eq!(row, expected); // actual producer TRS/payload/descriptor reach the same row
            assert_eq!(&row[12..], &[0x3e80_0000, 0xbe00_0000, 71, 1]);
            assert_eq!(subset.instance_count(), 1);
            assert_eq!(subset.positions()[key.index as usize].position, Vec3::new(3.0, 4.0, 5.0));
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &device);
        let tail = [0x3e80_0000u32, 0xbe00_0000, 71, 1].map(u32::to_le_bytes).concat();
        assert!(device.writes.borrow().iter().any(|bytes| bytes.len() == 64 && bytes[48..] == tail));
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 1);
        assert_eq!(world.resource::<AssetTextureLibrary>().handle_to_bindless_index(normal).0, 1);
        assert_eq!(state.entities_of(67), Some([entity].as_slice()));
    }

'''
text = text.replace(needle, test + needle)
p = w / 'visual_entity_builder.rs.final-candidate'
assert not p.exists()
p.write_text(text)
before = (w / 'visual_entity_builder.rs.before').read_text()
live = '/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/decorator/src/utils/visual_entity_builder.rs'
patch = ''.join(difflib.unified_diff(before.splitlines(True), text.splitlines(True), fromfile=live, tofile=live))
(w / 'visual_entity_builder.rs.final.patch').write_text(patch)
print(hashlib.sha256(p.read_bytes()).hexdigest())
