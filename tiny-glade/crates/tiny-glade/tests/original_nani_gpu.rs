//! Bounded original Nani master GPU fixture. One real typed instance and an
//! actual triangle atlas; all other generic owned subsets have zero instances.
//! No loading worker, whole-game startup or complete scene render is asserted.
#[path = "support/mod.rs"]
mod test_support;
use test_support::ShaderTestDriver as _;
use test_support::BridgeTestDriver as _;
use ash::vk;
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        configwatch::ConfigWatch, placement_data_library::AssetPlacementDataLibrary,
        shader_library::AssetShaderLibrary, ssbo_library::AssetSsboLibrary,
    },
    name_hash::NameHash,
    render::{
        frame_constants::FrameConstants,
        mesh::{ATTR_POSITION, Mesh},
        nani::{
            MeshAtlas,
            buffers::{
                self, NaniDrawListSsbo, NaniInstanceIndirectionSsbo, NaniNextVisibilityBitmaskSsbo,
                NaniPrevVisibilityBitmaskSsbo, NaniDrawListLengthSsbo,
            },
            cull_shaders::NaniCullAndBucketFamily,
            gpu_world::{GpuWorld, GpuWorldSubsetKind, InstanceCullData},
            instance_data::SolidVertexColorInstanceData,
        },
        ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice},
    },
};
use rhapsody::{
    command::buffer_access, resource::AccessScope, resource::Buffer as NativeBuffer,
    resource::BufferDesc as NativeBufferDesc, resource::with_setup_command,
};
use std::{ops::Range, path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::{render_bridge::CoreRenderBridge, render_host::BufferHost};

/// Readback is an explicit test-only extra usage. Creation/uploads still use
/// this bridge's actual owner table; production resource descriptions do not change.
struct DiagnosticDevice(Arc<BufferHost>);
impl GraphicsDevice for DiagnosticDevice {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
        initial: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        let mut desc = *desc;
        desc.usage |= vk::BufferUsageFlags::TRANSFER_SRC.as_raw();
        self.0.create_buffer(&desc, name, initial)
    }
    fn update_buffer_bytes(
        &self,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        self.0.update_buffer_bytes(buffer, data, ranges, offset)
    }
    fn device_address(&self, buffer: &Buffer) -> u64 {
        self.0.device_address(buffer)
    }
    fn release_buffer(&self, buffer: &Buffer) {
        self.0.release_buffer(buffer)
    }
}

fn read_range(
    device: &Arc<rhapsody::device::Device>,
    host: &BufferHost,
    buffer: &Buffer,
    offset: usize,
    size: usize,
) -> Vec<u8> {
    let source = host.managed_buffer(buffer).unwrap();
    let output = Arc::new(
        NativeBuffer::new(
            device.clone(),
            &NativeBufferDesc::new_gpu_to_cpu(size as u64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let queue = device
        .queue(rhapsody::device::QueueClass::Graphics)
        .unwrap();
    with_setup_command(device, &[], |command| unsafe {
        buffer_access(
            command,
            source.shared(),
            queue,
            offset as u64,
            size as u64,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
        )?;
        buffer_access(
            command,
            output.clone(),
            queue,
            0,
            size as u64,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_WRITE,
                write: true,
            },
        )?;
        device.raw_device().cmd_copy_buffer(
            command.raw_handle(),
            source.raw_handle(),
            output.raw_handle(),
            &[vk::BufferCopy::default()
                .src_offset(offset as u64)
                .size(size as u64)],
        );
        buffer_access(
            command,
            output.clone(),
            queue,
            0,
            size as u64,
            AccessScope {
                stages: vk::PipelineStageFlags::HOST,
                access: vk::AccessFlags::HOST_READ,
                write: false,
            },
        )
    })
    .and_then(|setup| setup.wait(rhapsody::resource::SETUP_WAIT_TIMEOUT_NS))
    .unwrap();
    let mut result = vec![0; size];
    output.read_bytes(0, &mut result).unwrap();
    result
}

struct DiagnosticLogger;
impl log::Log for DiagnosticLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Error
    }
    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!("{}", record.args());
        }
    }
    fn flush(&self) {}
}
static DIAGNOSTIC_LOGGER: DiagnosticLogger = DiagnosticLogger;
#[test]
fn original_nani_master_one_real_instance_builds_indexed_command_and_indirection() {
    verify_original_nani(GpuWorldSubsetKind::SolidVertexColor, 1, 1, false);
}

#[test]
fn original_nani_fine_lists_keep_all_instances_with_a_live_input_command_bound() {
    for kind in [GpuWorldSubsetKind::Canopy, GpuWorldSubsetKind::TreeTrunk] {
        verify_original_nani(kind, 65, 1, false);
        verify_original_nani(kind, 65, 3, false);
        verify_original_nani(kind, 65, 3, true);
    }
}

fn verify_original_nani(kind: GpuWorldSubsetKind, instance_count: usize, part_count: usize, rare_tail: bool) {
    if log::set_logger(&DIAGNOSTIC_LOGGER).is_ok() {
        log::set_max_level(log::LevelFilter::Error);
    }
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict original Nani GPU initialization: {error}"
                );
                eprintln!("SKIP original Nani GPU initialization: {error}");
                return;
            }
        };
    let installation = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(
        device.clone(),
        installation.canonicalize().unwrap(),
        shaders,
    )
    .unwrap();
    let mut world = World::new();
    bridge.install_asset_libraries(&mut world).unwrap();
    world.insert_resource(AssetPlacementDataLibrary::new());
    // Explicit diagnostic camera, not a substitute per-frame game producer.
    // Identity main matrices and z=-1 keep the sphere inside all side planes.
    let mut constants = FrameConstants::default();
    constants.inner_rendering_resolution = [64, 64];
    world.insert_resource(constants);
    let buffers = bridge.hosts().buffers.clone();
    let diagnostic = DiagnosticDevice(buffers.clone());
    let ssbos = world.resource::<AssetSsboLibrary>();
    let palette = country_core::startup::init_color_library::init_color_library(
        &diagnostic,
        ssbos,
        &mut ConfigWatch::new(false),
    );
    buffers::populate_nani_ssbo_library(ssbos, &diagnostic);

    let mut gpu = GpuWorld::new(&diagnostic);
    for subset_kind in GpuWorldSubsetKind::all() {
        if NaniCullAndBucketFamily::for_kind(subset_kind) == NaniCullAndBucketFamily::Main {
            gpu.assign_subset(
                subset_kind,
                country_core::startup::startup_nani::create_world_subset(
                    &diagnostic,
                    subset_kind,
                    if subset_kind == kind { part_count } else { 0 },
                ),
            );
        }
    }
    let mut mesh = Mesh::new("bounded Nani triangle fixture");
    mesh.set_attribute(
        ATTR_POSITION,
        vec![
            glam::Vec3::new(-1.0, 0.0, 0.0),
            glam::Vec3::new(1.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
        ],
    );
    mesh.set_indices(vec![0, 1, 2]);
    let mut atlas = MeshAtlas::default();
    for part in 0..part_count {
        assert_eq!(atlas.add(NameHash::compute(&format!("bounded Nani triangle {part}")).into(), &mesh).unwrap(), part as u32);
    }
    assert_eq!(atlas.layouts[0].count, 3);
    assert_eq!(atlas.layouts[0].radius, 1.0);
    gpu.update_mesh_atlas_slices(&diagnostic, kind, &atlas.layouts);
    let fine = country_core::render::nani::draw_ext::uses_indirect_count(kind, false, false);
    let cull = InstanceCullData::new(glam::Vec3::new(0.0, 0.0, -1.0), if fine { 0.1 } else { 0.01 }, 0);
    let mut record = SolidVertexColorInstanceData::default();
    // Native affine3 packed payload: 3x3 diagonal scale and real translation.
    for i in [0, 4, 8] {
        record.raw[i] = 0.01f32.to_bits();
    }
    record.raw[11] = (-1.0f32).to_bits();
    let mut pending = Vec::new();
    let cull: Vec<_> = (0..instance_count).map(|index| InstanceCullData { mesh_index: if rare_tail { index.saturating_sub(instance_count - 3) as u16 } else { (index % part_count) as u16 }, ..cull }).collect();
    if kind == GpuWorldSubsetKind::SolidVertexColor {
        gpu.create_instances(kind, &cull, &vec![record; instance_count], &mut pending);
    } else {
        use country_core::resources::placement_data::PlacementDataSsbo;
        let records = vec![PlacementDataSsbo {
            position: glam::Vec3::new(0.0, 0.0, -1.0), scale: 0.1,
            anim_t_packed: country_core::resources::placement_data::default_anim_t(),
            ..Default::default()
        }; instance_count];
        gpu.create_instances(kind, &cull, &records, &mut pending);
    }
    assert_eq!(pending.len(), instance_count);
    gpu.flush_instance_buffers(&diagnostic).unwrap();
    let cull_handle = gpu.owned_subset_mut(kind).cull_buffer().buffer;
    let params = gpu.draw_list_params();
    let list_index = kind as usize * 4;
    let command_index = params.max_instance_offsets[list_index] as usize;
    let indirection_index = params.cmd_offsets[list_index] as usize;
    let global_instance = params.max_instance_offsets[list_index] as usize;
    let word_count = (params.total_cmd_offset >> 5) as usize;
    // An explicitly seeded previous-frame visibility bit makes the first
    // native no-HZB round retain this one instance. New visibility starts zero.
    let mut previous = vec![NaniPrevVisibilityBitmaskSsbo([0]); word_count];
    for index in global_instance..global_instance + instance_count {
        previous[index / 32].0[0] |= 1 << (index % 32);
    }
    ssbos.add_new_with_data(&diagnostic, &previous);
    ssbos.add_new_with_data(
        &diagnostic,
        &vec![NaniNextVisibilityBitmaskSsbo([0]); word_count],
    );
    world.insert_resource(gpu);
    world.insert_resource(palette);

    let (_, built) = bridge
        .build_frame(|frame| {
            frame.import_gpu_world(&mut world)?;
            let result =
                tiny_glade::frame_setup::with_frame_resources(frame, &mut world, |frame, world| {
                    tiny_glade::scene_compute::generate_nani_draw_lists(frame, world, None)?;
                    // Resolve this exact full-generation handle through the same
                    // session's physical-Arc dedup. Import adds no pass access:
                    // without the real Nani metadata the assertion still fails.
                    // Match the actual imported owner's initial upload state;
                    // the cull pass below independently declares shader read.
                    let cull_input = frame.import_buffer(&cull_handle, 0x35)?;
                    frame.with_graph(|graph| {
                        let declaration = graph.declaration();
                        let cull = declaration.passes.iter()
                            .find(|pass| pass.name.as_ref() == "Nani cull & bucket")
                            .expect("actual original Nani cull builder pass");
                        let reads: Vec<_> = cull.accesses.iter()
                            .filter(|access| access.resource_index == cull_input.raw()
                                && access.access_code == 0x1a
                                && access.synchronization_group == 1)
                            .collect();
                        assert_eq!(reads.len(), 1, "the actual Owned cull positions owner");
                        assert!(cull.accesses.iter().any(|access|
                            access.resource_index != cull_input.raw()
                                && access.access_code == 0x1a
                                && access.synchronization_group == 1),
                            "original descriptor SSBO reads, including previous visibility, remain separate");
                        assert_eq!(reads[0].range, rhapsody::rg::AccessRange::Buffer {
                            offset: 0, size: (16 * instance_count) as u64,
                        }, "only the live GpuInstance prefix, not full capacity");
                        let input = &declaration.resources[reads[0].resource_index as usize];
                        let rhapsody::rg::Descriptor::Buffer(desc) = input.descriptor else {
                            panic!("cull BDA must name the actual imported buffer");
                        };
                        assert_eq!(desc.word_at_00, cull_handle.byte_size);
                        assert!(matches!(input.source,
                            rhapsody::rg::OriginSource::ImportUnowned { .. }));
                        Ok(())
                    })?;
                    Ok(())
                });
            let export = frame.export_gpu_world(&mut world);
            result.and(export)
        })
        .unwrap();
    bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(60))
        .unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    eprintln!(
        "original Nani prepared passes: {:?}",
        compiled.snapshot().pass_order
    );
    for required in ["Nani clear draw lists", "Nani clear histograms"] {
        assert!(
            compiled
                .snapshot()
                .pass_order
                .iter()
                .any(|name| name == required),
            "the original initializer {required} must remain before atomic/in-place consumers"
        );
    }
    let receipt = bridge.execute_after_uploads(compiled).unwrap();
    eprintln!(
        "original Nani compiled passes: {:?}",
        receipt.snapshot.pass_order
    );
    assert!(!receipt.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let ssbos = world.resource::<AssetSsboLibrary>();
    let fine = country_core::render::nani::draw_ext::uses_indirect_count(kind, false, false);
    let command_count = if fine {
        let bytes = read_range(&device, &buffers,
            ssbos.get_buffer::<NaniDrawListLengthSsbo>().unwrap(), list_index * 4, 4);
        let count = u32::from_le_bytes(bytes.try_into().unwrap()) as usize;
        let bound = world.resource::<GpuWorld>().owned_subset_mut(kind).main_cull_command_bound() as usize;
        if rare_tail { assert_eq!(bound, 5); }
        if part_count == 1 { assert_eq!(count, instance_count.div_ceil(32)); }
        assert!(count <= bound, "main cull groups cannot exceed live mesh-bin bound");
        assert!(count > 1, "one mesh part can produce several fine commands");
        count
    } else { 1 };
    let command = read_range(&device, &buffers,
        ssbos.get_buffer::<NaniDrawListSsbo>().unwrap(), command_index * 20, command_count * 20);
    let mut spans = Vec::new();
    for bytes in command.chunks_exact(20) {
        let words: Vec<_> = bytes.chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!([words[0], words[3]], [3, 0]);
        assert_eq!(words[2] % 3, 0);
        assert!(words[2] < part_count as u32 * 3);
        assert!(words[1] > 0);
        spans.push((words[4], words[1]));
    }
    spans.sort_unstable();
    let mut at = indirection_index as u32;
    for (first, count) in spans {
        assert_eq!(first, at, "fine commands cover disjoint contiguous indirection ranges");
        at += count;
    }
    assert_eq!(at, (indirection_index + instance_count) as u32);
    let indirection = read_range(&device, &buffers,
        ssbos.get_buffer::<NaniInstanceIndirectionSsbo>().unwrap(),
        indirection_index * 4, instance_count * 4);
    let mut actual: Vec<_> = indirection.chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
    actual.sort_unstable();
    assert_eq!(actual, (0..instance_count as u32).collect::<Vec<_>>(),
        "no input instance is truncated by the live command bound");
    let next = read_range(
        &device,
        &buffers,
        ssbos.get_buffer::<NaniNextVisibilityBitmaskSsbo>().unwrap(),
        global_instance / 32 * 4,
        4,
    );
    assert_ne!(
        u32::from_le_bytes(next.try_into().unwrap()) & (1 << (global_instance % 32)),
        0,
        "the native visibility atomic must preserve and publish this instance"
    );
    bridge.collect_completed_frames().unwrap();
}
