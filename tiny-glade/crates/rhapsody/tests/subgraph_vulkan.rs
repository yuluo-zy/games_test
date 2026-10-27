//! Cached local analyses still execute real GPU work and synchronize boundaries.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::rg::{resource::AccessRange, vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession}};
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::device::selection::create_headless_device;
use std::sync::Arc;
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
const HOST_READ: u8 = 0x26;

#[test]
fn unchanged_subgraph_hits_retain_real_writes_and_cross_boundary_visibility() {
    let Some(device) =
        gpu_test_support::initialize("subgraph GPU device", create_headless_device())
    else {
        return;
    };
    let Some(backend) = gpu_test_support::initialize(
        "subgraph GPU backend",
        VulkanGraphBackend::new(device.clone(), AccessTable::original()),
    ) else {
        return;
    };
    let mut session = VulkanGraphSession::new(backend);
    let middle_desc = BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let output_desc = BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST);
    let middle = Arc::new(Buffer::new(device.clone(), &middle_desc).unwrap());
    let output = Arc::new(Buffer::new(device.clone(), &output_desc).unwrap());
    let middle_token = session
        .backend
        .bind_import(GpuResource::Buffer(middle))
        .unwrap();
    let output_token = session
        .backend
        .bind_import(GpuResource::Buffer(output.clone()))
        .unwrap();
    for frame in 0..8u32 {
        let mut graph = session.begin_frame().unwrap();
        let middle = graph
            .import_unowned(
                "middle",
                ResourceSpec::Buffer(middle_desc).descriptor(),
                middle_token,
                READ,
            )
            .unwrap();
        let target = graph
            .import_unowned(
                "output",
                ResourceSpec::Buffer(output_desc).descriptor(),
                output_token,
                HOST_READ,
            )
            .unwrap();
        graph
            .subgraph("stable producer", |scope| {
                let mut pass = scope.graph().add_pass("fill actual GPU buffer");
                pass.add_resource_access(middle, WRITE, 0)?;
                let device = device.clone();
                pass.render_with_context(move |ctx| {
                    let Some(managed) = ctx.managed else {
                        return false;
                    };
                    let Ok(buffer) = managed.buffer_for(middle.raw()) else {
                        return false;
                    };

                    unsafe {
                        device.raw_device().cmd_fill_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            buffer.raw(),
                            0,
                            64,
                            0x1020_3000 + frame,
                        );
                    }
                    true
                });
                scope.output(middle, AccessRange::Whole, WRITE)?;
                Ok(())
            })
            .unwrap();
        graph
            .subgraph("changing consumer", |scope| {
                scope.interface_mut().implementation_signature = u64::from(frame);
                scope.input(middle, AccessRange::Whole, READ)?;
                let mut pass = scope.graph().add_pass("copy actual GPU output");
                pass.add_resource_access(middle, READ, 0)?;
                pass.add_resource_access(target, WRITE, 0)?;
                let device = device.clone();
                pass.render_with_context(move |ctx| {
                    let Some(managed) = ctx.managed else {
                        return false;
                    };
                    let (Ok(source), Ok(output)) = (
                        managed.buffer_for(middle.raw()),
                        managed.buffer_for(target.raw()),
                    ) else {
                        return false;
                    };

                    unsafe {
                        device.raw_device().cmd_copy_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            source.raw(),
                            output.raw(),
                            &[vk::BufferCopy {
                                src_offset: 0,
                                dst_offset: 0,
                                size: 64,
                            }],
                        );
                    }
                    true
                });
                scope.output(target, AccessRange::Whole, WRITE)?;
                Ok(())
            })
            .unwrap();
        graph.export(target, HOST_READ).unwrap();
        let receipt = session.execute_frame(graph.build()).unwrap();
        assert_eq!(receipt.snapshot.pass_order.len(), 2);
        assert!(
            session
                .last_synchronization()
                .transitions()
                .iter()
                .any(|transition| transition.phase
                    == rhapsody::rg::sync_plan::BarrierPhase::BeforePass(1)
                    && transition.source.write
                    && !transition.destination.write)
        );
        session.wait_submits_done(10_000_000_000).unwrap();
        let mut bytes = [0u8; 64];
        unsafe {
            output.read_bytes_after_completion(0, &mut bytes).unwrap();
        }
        for word in bytes.as_chunks::<4>().0 {
            assert_eq!(u32::from_ne_bytes(*word), 0x1020_3000 + frame);
        }
        let stats = session.chain.subgraph_cache_stats();
        assert_eq!(stats.hits, u64::from(frame));
        assert_eq!(stats.compilations, u64::from(frame) + 2);
        assert!(stats.bytes <= 16 * 1024 * 1024);
    }
    session.release_import(middle_token).unwrap();
    session.release_import(output_token).unwrap();
}
