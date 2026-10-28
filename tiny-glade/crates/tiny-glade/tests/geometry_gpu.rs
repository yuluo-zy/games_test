//! 真实设备验证墙砖 ECS 发布、分块上传、静止帧复用与更新后的字节回读。
#[path = "support/mod.rs"]
mod test_support;
use ash::vk;
use bevy_ecs::{schedule::Schedule, world::World};
use country_core::{
    asset_libraries::{
        runtime::{AssetLibraryRuntime, add_asset_library_systems},
        ssbo_library::AssetSsboLibrary,
    },
    geometry::{
        instanced_wall::{BrickTransformSsbo, InstancedWall},
        runtime::WallBrickUploadStatus,
    },
    render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice},
};
use rhapsody::{command::buffer_access, resource::Buffer as NativeBuffer, resource::BufferDesc as NativeBufferDesc};
use rhapsody::transfer::with_setup_command;
use rhapsody::native::AccessScope;
use std::{ops::Range, sync::Arc};
use tiny_glade::render_host::{BufferHost, TextureHost};

/// 回读诊断只增加 TRANSFER_SRC 用途；分配和上传仍走生产 BufferHost。
struct ReadbackDevice(Arc<BufferHost>);
impl GraphicsDevice for ReadbackDevice {
    fn create_buffer(
        &self,
        description: &BufferDesc,
        name: &str,
        initial: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        let mut description = *description;
        description.usage |= vk::BufferUsageFlags::TRANSFER_SRC.as_raw();
        self.0.create_buffer(&description, name, initial)
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
    fn release_buffer(&self, buffer: &Buffer) {
        self.0.release_buffer(buffer);
    }
}

fn read_prefix(
    device: &Arc<rhapsody::device::Device>,
    host: &BufferHost,
    buffer: &Buffer,
    size: usize,
) -> Vec<u8> {
    // Host writes are queued; submit them before reading the device copy, as the
    // renderer does before every graph.
    test_support::until_ready(|| host.poll_pending_uploads()).unwrap();
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
    // SAFETY: 源和目标均属于当前设备，范围有效且通过资源访问跟踪登记；setup 等待复制结束。
    with_setup_command(device, &[], |command| unsafe {
        buffer_access(
            command,
            &source.shared(),
            queue,
            0,
            size as u64,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
        )?;
        buffer_access(
            command,
            &output.clone(),
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
            &[vk::BufferCopy::default().size(size as u64)],
        );
        buffer_access(
            command,
            &output.clone(),
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
    .and_then(|setup| setup.wait(rhapsody::transfer::SETUP_WAIT_TIMEOUT_NS))
    .unwrap();
    let mut bytes = vec![0; size];
    output.read_bytes(0, &mut bytes).unwrap();
    bytes
}

#[test]
fn wall_brick_schedule_updates_real_device_bytes_and_reuses_idle_storage() {
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                    "GPU 初始化失败：{error}"
                );
                eprintln!("SKIP 墙砖 GPU 验证：{error}");
                return;
            }
        };
    let host = Arc::new(BufferHost::new(device.clone()));
    let mut world = World::new();
    world.insert_resource(AssetLibraryRuntime::new(
        Arc::new(ReadbackDevice(host.clone())),
        Arc::new(TextureHost::new(device.clone())),
    ));
    world.init_resource::<AssetSsboLibrary>();
    world.init_resource::<WallBrickUploadStatus>();
    let bricks: Vec<_> = (0..33)
        .map(|index| BrickTransformSsbo {
            seed: index,
            material: 0x8000_0001,
            flags: 6,
            ..Default::default()
        })
        .collect();
    let expected: Vec<u8> = bricks
        .iter()
        .flat_map(|brick| brick.observed_bytes())
        .collect();
    let entity = world.spawn(InstancedWall::new(7, bricks)).id();
    let mut schedule = Schedule::default();
    add_asset_library_systems(&mut schedule);
    schedule.run(&mut world);
    assert!(world.resource::<WallBrickUploadStatus>().gpu_ready);
    let buffer = world
        .resource::<AssetSsboLibrary>()
        .get::<BrickTransformSsbo>()
        .unwrap()
        .buffer
        .buffer;
    assert_eq!(
        read_prefix(&device, &host, &buffer, expected.len()),
        expected
    );
    let render_snapshot = world.resource::<AssetSsboLibrary>().clone();
    assert!(render_snapshot.get::<BrickTransformSsbo>().is_none());
    assert_eq!(render_snapshot.sparse_draw_view::<BrickTransformSsbo>().unwrap()
        .to_draw_multi_instanced(), [(0, 33)]);
    schedule.run(&mut world);
    assert_eq!(
        read_prefix(&device, &host, &buffer, expected.len()),
        expected
    );
    let replacement = BrickTransformSsbo {
        seed: 999,
        ..Default::default()
    };
    world
        .component_mut::<InstancedWall<BrickTransformSsbo>>(entity)
        .unwrap()
        .update(vec![replacement]);
    schedule.run(&mut world);
    assert!(world.resource::<WallBrickUploadStatus>().gpu_ready);
    assert_eq!(
        read_prefix(&device, &host, &buffer, 96),
        replacement.observed_bytes()
    );
    world.despawn(entity);
    schedule.run(&mut world);
    assert!(
        world
            .resource::<AssetSsboLibrary>()
            .get::<BrickTransformSsbo>()
            .unwrap()
            .to_draw_multi_instanced()
            .is_empty()
    );
    // CPU source destruction cannot free a GPU allocation still held by an
    // extracted frame. Its draw metadata also survives edits and reclamation.
    drop(world.remove_resource::<AssetSsboLibrary>().unwrap());
    assert!(host.managed_buffer(&buffer).is_some());
    assert_eq!(render_snapshot.sparse_draw_view::<BrickTransformSsbo>().unwrap()
        .to_draw_multi_instanced(), [(0, 33)]);
    assert_eq!(read_prefix(&device, &host, &buffer, 96), replacement.observed_bytes());
    drop(render_snapshot);
    assert!(host.managed_buffer(&buffer).is_none());
    world.init_resource::<AssetSsboLibrary>();
    world.spawn(InstancedWall::new(8, vec![replacement]));
    schedule.run(&mut world);
    assert!(world.resource::<WallBrickUploadStatus>().gpu_ready);
    let next_buffer = world
        .resource::<AssetSsboLibrary>()
        .get::<BrickTransformSsbo>()
        .unwrap()
        .buffer
        .buffer;
    assert_eq!(next_buffer.id & 0xffff_ffff, buffer.id & 0xffff_ffff, "释放后的宿主槽应被复用");
    assert_ne!(next_buffer.id, buffer.id);
    assert_ne!(next_buffer.generation, buffer.generation);
    assert!(host.managed_buffer(&buffer).is_none());
    assert_eq!(
        read_prefix(&device, &host, &next_buffer, 96),
        replacement.observed_bytes()
    );
    drop(world);
    assert!(host.managed_buffer(&next_buffer).is_none());
}

use country_core::render::extract::WorldComponentWrite as _;
