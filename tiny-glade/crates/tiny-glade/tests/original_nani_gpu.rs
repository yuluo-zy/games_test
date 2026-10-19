//! Bounded original Nani master GPU fixture. One real typed instance and an
//! actual triangle atlas; all other generic owned subsets have zero instances.
//! No loading worker, whole-game startup or complete scene render is asserted.
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
                NaniPrevVisibilityBitmaskSsbo,
            },
            cull_shaders::NaniCullAndBucketFamily,
            gpu_world::{GpuWorld, GpuWorldSubsetKind, InstanceCullData},
            instance_data::SolidVertexColorInstanceData,
        },
        ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice},
    },
};
use rhapsody::{
    raw::{
        buffer::{Buffer as NativeBuffer, BufferDesc as NativeBufferDesc},
        resource_access::buffer_access,
        setup::with_setup_command,
    },
    resource::range_state::AccessScope,
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
            command.handle,
            source.handle,
            output.handle,
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
    .and_then(|setup| setup.wait(rhapsody::raw::setup::SETUP_WAIT_TIMEOUT_NS))
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
    for kind in GpuWorldSubsetKind::all() {
        if NaniCullAndBucketFamily::for_kind(kind) == NaniCullAndBucketFamily::Main {
            gpu.assign_subset(
                kind,
                country_core::startup::startup_nani::create_world_subset(
                    &diagnostic,
                    kind,
                    usize::from(kind == GpuWorldSubsetKind::SolidVertexColor),
                ),
            );
        }
    }
    let kind = GpuWorldSubsetKind::SolidVertexColor;
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
    assert_eq!(
        atlas
            .add(
                NameHash::compute("bounded Nani triangle fixture").into(),
                &mesh
            )
            .unwrap(),
        0
    );
    assert_eq!(atlas.layouts[0].count, 3);
    assert_eq!(atlas.layouts[0].radius, 1.0);
    gpu.update_mesh_atlas_slices(&diagnostic, kind, &atlas.layouts);
    let cull = InstanceCullData::new(glam::Vec3::new(0.0, 0.0, -1.0), 0.01, 0);
    let mut record = SolidVertexColorInstanceData::default();
    // Native affine3 packed payload: 3x3 diagonal scale and real translation.
    for i in [0, 4, 8] {
        record.raw[i] = 0.01f32.to_bits();
    }
    record.raw[11] = (-1.0f32).to_bits();
    let mut pending = Vec::new();
    gpu.create_instances(kind, &[cull], &[record], &mut pending);
    assert_eq!(pending.len(), 1);
    gpu.flush_instance_buffers(&diagnostic);
    let params = gpu.draw_list_params();
    let list_index = kind as usize * 4;
    let command_index = params.max_instance_offsets[list_index] as usize;
    let indirection_index = params.cmd_offsets[list_index] as usize;
    let global_instance = params.max_instance_offsets[list_index] as usize;
    let word_count = (params.total_cmd_offset >> 5) as usize;
    // An explicitly seeded previous-frame visibility bit makes the first
    // native no-HZB round retain this one instance. New visibility starts zero.
    let mut previous = vec![NaniPrevVisibilityBitmaskSsbo([0]); word_count];
    previous[global_instance / 32].0[0] |= 1 << (global_instance % 32);
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
                    Ok(())
                });
            let export = frame.export_gpu_world(&mut world);
            result.and(export)
        })
        .unwrap();
    bridge
        .shader_backend()
        .prewarm(Duration::from_secs(60))
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
    let receipt = bridge.execute_compiled(compiled).unwrap();
    eprintln!(
        "original Nani compiled passes: {:?}",
        receipt.snapshot.pass_order
    );
    assert!(!receipt.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let ssbos = world.resource::<AssetSsboLibrary>();
    let command = read_range(
        &device,
        &buffers,
        ssbos.get_buffer::<NaniDrawListSsbo>().unwrap(),
        command_index * 20,
        20,
    );
    let words: Vec<_> = command
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    assert_eq!(
        words,
        [3, 1, 0, 0, indirection_index as u32],
        "the original atlas+instance must generate one actual indexed indirect command"
    );
    let indirection = read_range(
        &device,
        &buffers,
        ssbos.get_buffer::<NaniInstanceIndirectionSsbo>().unwrap(),
        indirection_index * 4,
        4,
    );
    assert_eq!(
        u32::from_le_bytes(indirection.try_into().unwrap()),
        0,
        "the first live instance must survive sorting into the actual indirection table"
    );
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
