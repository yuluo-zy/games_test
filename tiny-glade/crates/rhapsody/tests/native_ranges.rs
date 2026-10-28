//! Native byte/subresource synchronization through the production graph session.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{device::{Device, DeviceConfig, QueueClass, QueueConfig, QueuePreference}, rg::{executor::{ExecutionBackend, PassContext}, graph::RenderGraph, parallel::ParallelRecording, resource::{AccessRange, ResourceHandle}, sync_plan::{BarrierKind, BarrierPhase, DependencyRef}, vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession}}};
use rhapsody::tracking::OwnershipRange;
use rhapsody::tracking::range_state::RangeOwner;
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::resource::texture::{Texture, TextureDesc, TextureKind};
use rhapsody::native::instance::{InstanceBuilder, ValidationMessenger};
use std::sync::Arc;

const WRITE: u8 = 0x34;
const READ: u8 = 0x25;
const HOST: u8 = 0x26;
const GENERAL_WRITE: u8 = 0x33;
const GENERAL_READ: u8 = 0x24;
const TIMEOUT: u64 = 10_000_000_000;

struct Fixture {
    session: VulkanGraphSession,
    device: Arc<Device>,
    validation: ValidationMessenger,
}
impl Fixture {
    fn execute(
        &mut self,
        graph: RenderGraph,
    ) -> rhapsody::Result<rhapsody::rg::vulkan::SessionFrameReceipt> {
        let built = graph.build();
        let context = self.session.backend.plan_context();
        let definition = rhapsody::rg::plan::GraphDefinition::from_declaration(
            &built.declaration,
            &self.session.backend.accesses,
            context,
        )
        .unwrap();
        let plan = rhapsody::rg::plan::compile_definition(definition).unwrap();
        let result = self.session.execute_frame(built);
        rhapsody::rg::plan_check::check_bound_synchronization(
            &plan.range_program,
            self.session.last_synchronization(),
        )
        .unwrap();
        result
    }
    fn new(parallel: ParallelRecording) -> Option<Self> {
        Self::with_queues(parallel, false)
    }
    fn with_queues(parallel: ParallelRecording, independent: bool) -> Option<Self> {
        let (device, validation) = gpu_test_support::initialize(
            "native resource ranges",
            (|| -> rhapsody::Result<_> {
                let mut builder = InstanceBuilder::default();
                builder.debug_markers();
                builder
                    .layers
                    .push(c"VK_LAYER_KHRONOS_validation".to_owned());
                let device = Device::new(
                    &builder.build()?,
                    None,
                    &DeviceConfig {
                        validation: true,
                        queues: QueueConfig {
                            async_compute: if independent {
                                QueuePreference::Require
                            } else {
                                QueuePreference::Off
                            },
                            transfer: if independent {
                                QueuePreference::Require
                            } else {
                                QueuePreference::Off
                            },
                            require_present: false,
                        },
                        ..DeviceConfig::headless()
                    },
                )?;
                let validation = ValidationMessenger::new(device.instance())?;
                Ok((device, validation))
            })(),
        )?;
        let mut accesses = AccessTable::original();
        let mut write = accesses.0[&WRITE];
        write.layout = vk::ImageLayout::GENERAL;
        accesses.0.insert(GENERAL_WRITE, write);
        let mut read = accesses.0[&READ];
        read.layout = vk::ImageLayout::GENERAL;
        accesses.0.insert(GENERAL_READ, read);
        let mut session =
            VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), accesses).unwrap());
        session.chain.parallel = parallel;
        Some(Self {
            session,
            device,
            validation,
        })
    }
    fn buffer(&mut self, size: u64, host: bool) -> (Arc<Buffer>, u64) {
        let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
        let desc = if host {
            BufferDesc::new_gpu_to_cpu(size, usage)
        } else {
            BufferDesc::new_gpu_only(size, usage)
        };
        let buffer = Arc::new(Buffer::new(self.device.clone(), &desc).unwrap());
        let token = self
            .session
            .backend
            .bind_import(GpuResource::Buffer(buffer.clone()))
            .unwrap();
        (buffer, token)
    }
    fn image(&mut self) -> (Arc<Texture>, u64) {
        self.image_shared(false)
    }
    fn image_shared(&mut self, concurrent: bool) -> (Arc<Texture>, u64) {
        let desc = TextureDesc::new(
            vk::Extent3D {
                width: 8,
                height: 8,
                depth: 1,
            },
            TextureKind::D2Array,
            vk::Format::R8G8B8A8_UNORM,
        )
        .array_elements(2)
        .unwrap()
        .mip_levels(2)
        .usage(vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST);
        let texture = Arc::new(if concurrent {
            let mut families = self
                .device
                .queue_plan()
                .queues
                .iter()
                .map(|q| q.family)
                .collect::<Vec<_>>();
            families.sort_unstable();
            families.dedup();
            Texture::new_concurrent(self.device.clone(), &desc, &families).unwrap()
        } else {
            Texture::new(self.device.clone(), &desc).unwrap()
        });
        let token = self
            .session
            .backend
            .bind_import(GpuResource::Image(texture.clone()))
            .unwrap();
        (texture, token)
    }
    fn finish(mut self) {
        self.session.wait_submits_done(TIMEOUT).unwrap();
        let collector = self.validation.collector.clone();
        drop(self.session);
        self.device.wait_idle().unwrap();
        drop(self.device);
        assert!(collector.errors().is_empty(), "{:?}", collector.errors());
        assert!(
            collector.warnings().is_empty(),
            "{:?}",
            collector.warnings()
        );
    }
}
fn bytes(offset: u64, size: u64) -> AccessRange {
    AccessRange::Buffer { offset, size }
}
fn image_range(mip: u32, layer: u32) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip_level: mip,
        level_count: 1,
        base_array_layer: layer,
        layer_count: 1,
    }
}
fn native_image(mip: u32, layer: u32) -> OwnershipRange {
    OwnershipRange::Image {
        aspects: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip: mip,
        mip_count: 1,
        base_layer: layer,
        layer_count: 1,
    }
}
fn import_buffer(
    graph: &mut RenderGraph,
    buffer: &Buffer,
    token: u64,
    name: &str,
) -> ResourceHandle {
    graph
        .import_unowned(
            name,
            ResourceSpec::Buffer(buffer.desc).descriptor(),
            token,
            HOST,
        )
        .unwrap()
}
fn import_image(
    graph: &mut RenderGraph,
    image: &Texture,
    token: u64,
    initialized: bool,
) -> ResourceHandle {
    graph
        .import_unowned_with_sharing(
            "subresources",
            ResourceSpec::Image(image.desc).descriptor(),
            token,
            if initialized { WRITE } else { 0 },
            image.sharing(),
        )
        .unwrap()
}
fn fill(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    buffer: ResourceHandle,
    offset: u64,
    size: u64,
    value: u32,
) {
    fill_on(
        graph,
        device,
        buffer,
        (offset, size),
        value,
        QueueClass::Graphics,
    );
}
fn fill_on(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    buffer: ResourceHandle,
    range: (u64, u64),
    value: u32,
    queue: QueueClass,
) {
    let (offset, size) = range;
    let mut pass = graph
        .add_pass(format!("fill {offset}..{}", offset + size))
        .with_queue(queue)
        .with_side_effect();
    pass.add_resource_access_range(buffer, WRITE, 0, bytes(offset, size))
        .unwrap();
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        let target = ctx.managed.unwrap().buffer_for(buffer.raw())?;
        unsafe {
            device.raw_device().cmd_fill_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                target.raw(),
                offset,
                size,
                value,
            );
        }
        Ok(())
    });
}
fn clear(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    image: ResourceHandle,
    subresource: (u32, u32),
    general: bool,
    value: u8,
    fail: bool,
) {
    clear_on(
        graph,
        device,
        image,
        (subresource.0, subresource.1, general, value, fail),
        QueueClass::Graphics,
    );
}
fn clear_on(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    image: ResourceHandle,
    cell: (u32, u32, bool, u8, bool),
    queue: QueueClass,
) {
    let (mip, layer, general, value, fail) = cell;
    let mut pass = graph
        .add_pass(format!("clear mip {mip} layer {layer}"))
        .with_queue(queue)
        .with_side_effect();
    pass.add_resource_access_range(
        image,
        if general { GENERAL_WRITE } else { WRITE },
        0,
        image_range(mip, layer),
    )
    .unwrap();
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        if fail {
            return Err(rhapsody::Error::invalid_usage(
                "injected subresource recording failure",
            ));
        }
        let target = ctx.managed.unwrap().texture_for(image.raw())?;
        unsafe {
            device.raw_device().cmd_clear_color_image(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                target.raw(),
                if general {
                    vk::ImageLayout::GENERAL
                } else {
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL
                },
                &vk::ClearColorValue {
                    float32: [f32::from(value) / 255.0; 4],
                },
                &[vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(mip)
                    .level_count(1)
                    .base_array_layer(layer)
                    .layer_count(1)],
            );
        }
        Ok(())
    });
}
fn read_image(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    image: ResourceHandle,
    target: ResourceHandle,
    subresource: (u32, u32),
    general: bool,
    offset: u64,
) {
    read_image_on(
        graph,
        device,
        image,
        target,
        (subresource.0, subresource.1, general, offset),
        QueueClass::Graphics,
    );
}
fn read_image_on(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    image: ResourceHandle,
    target: ResourceHandle,
    cell: (u32, u32, bool, u64),
    queue: QueueClass,
) {
    let (mip, layer, general, offset) = cell;
    let extent = 8u32 >> mip;
    let size = u64::from(extent * extent * 4);
    let mut pass = graph
        .add_pass(format!("copy mip {mip} layer {layer}"))
        .with_queue(queue)
        .with_side_effect();
    pass.add_resource_access_range(
        image,
        if general { GENERAL_READ } else { READ },
        0,
        image_range(mip, layer),
    )
    .unwrap();
    pass.add_resource_access_range(target, WRITE, 0, bytes(offset, size))
        .unwrap();
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        let source = ctx.managed.unwrap().texture_for(image.raw())?;
        let target = ctx.managed.unwrap().buffer_for(target.raw())?;
        unsafe {
            device.raw_device().cmd_copy_image_to_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                source.raw(),
                if general {
                    vk::ImageLayout::GENERAL
                } else {
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL
                },
                target.raw(),
                &[vk::BufferImageCopy {
                    buffer_offset: offset,
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: mip,
                        base_array_layer: layer,
                        layer_count: 1,
                    },
                    image_extent: vk::Extent3D {
                        width: extent,
                        height: extent,
                        depth: 1,
                    },
                    ..Default::default()
                }],
            );
        }
        Ok(())
    });
}
fn read(buffer: &Buffer, size: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    unsafe {
        buffer.read_bytes_after_completion(0, &mut data).unwrap();
    }
    data
}

#[test]
fn buffer_barriers_follow_disjoint_and_overlapping_bytes_across_worker_chunk_sizes() {
    {
        for parallel in [ParallelRecording::PARALLEL, ParallelRecording::EVERY_PASS] {
            let Some(mut fixture) = Fixture::new(parallel) else {
                return;
            };
            let (source, token) = fixture.buffer(64, false);
            let (target, output_token) = fixture.buffer(64, true);
            let mut graph = fixture.session.begin_frame().unwrap();
            let src = import_buffer(&mut graph, &source, token, "source");
            let dst = import_buffer(&mut graph, &target, output_token, "output");
            fill(&mut graph, fixture.device.clone(), src, 0, 32, 0x11223344);
            fill(&mut graph, fixture.device.clone(), src, 32, 32, 0x55667788);
            fill(&mut graph, fixture.device.clone(), src, 16, 32, 0x99aabbcc);
            let mut pass = graph.add_pass("read complete buffer");
            pass.add_resource_access_range(src, READ, 0, bytes(0, 64))
                .unwrap();
            pass.add_resource_access_range(dst, WRITE, 0, bytes(0, 64))
                .unwrap();
            let device = fixture.device.clone();
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let src = ctx.managed.unwrap().buffer_for(src.raw())?;
                let dst = ctx.managed.unwrap().buffer_for(dst.raw())?;
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        src.raw(),
                        dst.raw(),
                        &[vk::BufferCopy {
                            src_offset: 0,
                            dst_offset: 0,
                            size: 64,
                        }],
                    );
                }
                Ok(())
            });
            graph.export(dst, HOST).unwrap();
            fixture.execute(graph).unwrap();
            fixture.session.wait_submits_done(TIMEOUT).unwrap();
            for (index, word) in read(&target, 64).as_chunks::<4>().0.iter().enumerate() {
                let expected: u32 = if index < 4 {
                    0x11223344
                } else if index < 12 {
                    0x99aabbcc
                } else {
                    0x55667788
                };
                assert_eq!(*word, expected.to_ne_bytes());
            }
            let native = fixture.session.backend.bound_synchronization();
            let transitions: Vec<_> = native
                .transitions()
                .iter()
                .filter(|t| t.resource == src.raw())
                .collect();
            assert!(!transitions.is_empty());
            let overlap: Vec<_> = transitions
                .iter()
                .filter(|t| t.phase == BarrierPhase::BeforePass(2))
                .collect();
            assert!(!overlap.is_empty());
            assert!(
                overlap.iter().all(
                    |t| matches!(t.range, OwnershipRange::Buffer { offset, size }
                if offset >= 16 && offset + size <= 48)
                ),
                "the overlapping write must synchronize exactly its actual byte range"
            );
            for byte in 16..48 {
                assert!(overlap.iter().any(
                    |t| matches!(t.range, OwnershipRange::Buffer { offset, size }
                    if offset <= byte && byte < offset + size)
                ));
            }
            drop(source);
            drop(target);
            fixture.finish();
        }
    }
}

#[test]
fn image_mips_and_layers_keep_distinct_native_layouts_across_frames_without_whole_initialization() {
    {
        for parallel in [ParallelRecording::PARALLEL, ParallelRecording::EVERY_PASS] {
            let Some(mut fixture) = Fixture::new(parallel) else {
                return;
            };
            let (image, token) = fixture.image();
            let (target, output_token) = fixture.buffer(640, true);
            let cells = [
                (0, 0, false, 11, 0),
                (0, 1, true, 22, 256),
                (1, 0, true, 33, 512),
                (1, 1, false, 44, 576),
            ];
            let mut graph = fixture.session.begin_frame().unwrap();
            let img = import_image(&mut graph, &image, token, false);
            for (mip, layer, general, value, _) in cells {
                clear(
                    &mut graph,
                    fixture.device.clone(),
                    img,
                    (mip, layer),
                    general,
                    value,
                    false,
                );
            }
            fixture.execute(graph).unwrap();
            assert!(
                fixture
                    .session
                    .backend
                    .bound_synchronization()
                    .transitions()
                    .iter()
                    .filter(|t| t.resource == img.raw())
                    .all(|t| matches!(
                        t.range,
                        OwnershipRange::Image {
                            mip_count: 1,
                            layer_count: 1,
                            ..
                        }
                    )),
                "first use must initialize only the actual mip/layer"
            );
            let mut graph = fixture.session.begin_frame().unwrap();
            let img = import_image(&mut graph, &image, token, true);
            let dst = import_buffer(&mut graph, &target, output_token, "pixels");
            for (mip, layer, general, _, offset) in cells {
                read_image(
                    &mut graph,
                    fixture.device.clone(),
                    img,
                    dst,
                    (mip, layer),
                    general,
                    offset,
                );
            }
            graph.export(dst, HOST).unwrap();
            fixture.execute(graph).unwrap();
            let native = fixture.session.backend.bound_synchronization();
            for (mip, layer, general, _, _) in cells {
                let expected = if general {
                    vk::ImageLayout::GENERAL
                } else {
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL
                };
                assert!(
                    native
                        .initial_inputs()
                        .iter()
                        .any(|initial| initial.resource == img.raw()
                            && initial.cell.range == native_image(mip, layer)
                            && initial.cell.state.layout == expected)
                );
                assert!(
                    native
                        .transitions()
                        .iter()
                        .filter(|t| t.resource == img.raw() && t.range == native_image(mip, layer))
                        .all(|t| t.old_layout == expected)
                );
            }
            fixture.session.wait_submits_done(TIMEOUT).unwrap();
            let pixels = read(&target, 640);
            for (mip, _, _, value, offset) in cells {
                let length = ((8usize >> mip).pow(2)) * 4;
                assert!(
                    pixels[offset as usize..offset as usize + length]
                        .iter()
                        .all(|&byte| byte == value)
                );
            }
            drop(image);
            drop(target);
            fixture.finish();
        }
    }
}

#[test]
fn failed_later_batch_preserves_the_submitted_prefix_and_untouched_image_subresources() {
    let Some(mut fixture) = Fixture::new(ParallelRecording::PARALLEL) else {
        return;
    };
    let (image, token) = fixture.image();
    let (target, output_token) = fixture.buffer(640, true);
    let mut graph = fixture.session.begin_frame().unwrap();
    graph.batch_per_pass(true);
    let img = import_image(&mut graph, &image, token, false);
    clear(
        &mut graph,
        fixture.device.clone(),
        img,
        (0, 0),
        false,
        17,
        false,
    );
    clear(
        &mut graph,
        fixture.device.clone(),
        img,
        (1, 1),
        true,
        99,
        true,
    );
    let error = fixture.execute(graph).unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert!(
        fixture
            .session
            .backend
            .submissions
            .iter()
            .any(|capture| capture.result == 0),
        "the prefix must have a real successful submission"
    );
    let mut graph = fixture.session.begin_frame().unwrap();
    let img = import_image(&mut graph, &image, token, true);
    let dst = import_buffer(&mut graph, &target, output_token, "recovery output");
    let cells = [
        (0, 0, false, 17, 0),
        (0, 1, true, 28, 256),
        (1, 0, true, 39, 512),
        (1, 1, false, 50, 576),
    ];
    for (mip, layer, general, value, _) in cells.into_iter().skip(1) {
        clear(
            &mut graph,
            fixture.device.clone(),
            img,
            (mip, layer),
            general,
            value,
            false,
        );
    }
    for (mip, layer, general, _, offset) in cells {
        read_image(
            &mut graph,
            fixture.device.clone(),
            img,
            dst,
            (mip, layer),
            general,
            offset,
        );
    }
    graph.export(dst, HOST).unwrap();
    fixture.execute(graph).unwrap();
    let native = fixture.session.backend.bound_synchronization();
    assert!(
        native
            .initial_inputs()
            .iter()
            .any(|initial| initial.resource == img.raw()
                && initial.cell.range == native_image(0, 0)
                && initial.cell.state.layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL)
    );
    for initial in native
        .initial_inputs()
        .iter()
        .filter(|initial| initial.resource == img.raw())
    {
        if initial.cell.range != native_image(0, 0) {
            assert_eq!(
                initial.cell.state.layout,
                vk::ImageLayout::UNDEFINED,
                "cancelled work must not publish a layout or overwrite untouched cells"
            );
        }
    }
    fixture.session.wait_submits_done(TIMEOUT).unwrap();
    let pixels = read(&target, 640);
    for (mip, _, _, value, offset) in cells {
        let length = ((8usize >> mip).pow(2)) * 4;
        assert!(
            pixels[offset as usize..offset as usize + length]
                .iter()
                .all(|&byte| byte == value)
        );
    }
    drop(image);
    drop(target);
    fixture.finish();
}

#[test]
fn concurrent_families_initialize_disjoint_image_layouts_without_serializing_and_join_real_points()
{
    {
        let Some(mut fixture) = Fixture::with_queues(ParallelRecording::PARALLEL, true) else {
            return;
        };
        let graphics = fixture.device.queue_plan().queue(QueueClass::Graphics);
        let compute = fixture.device.queue_plan().queue(QueueClass::Compute);
        assert_ne!(
            fixture
                .device
                .queue_plan()
                .physical(QueueClass::Graphics)
                .family,
            fixture
                .device
                .queue_plan()
                .physical(QueueClass::Compute)
                .family,
            "this fixture requires real independent families"
        );
        let (image, token) = fixture.image_shared(true);
        let (target, output_token) = fixture.buffer(320, true);
        let mut graph = fixture.session.begin_frame().unwrap();
        let img = import_image(&mut graph, &image, token, false);
        clear_on(
            &mut graph,
            fixture.device.clone(),
            img,
            (0, 0, false, 51, false),
            QueueClass::Graphics,
        );
        clear_on(
            &mut graph,
            fixture.device.clone(),
            img,
            (1, 1, true, 102, false),
            QueueClass::Compute,
        );
        fixture.execute(graph).unwrap();
        let bound = fixture.session.backend.bound_synchronization();
        assert!(
            bound.waits().is_empty(),
            "disjoint initial writes inherited a whole-image wait"
        );
        assert!(
            bound
                .transitions()
                .iter()
                .filter(|t| t.resource == img.raw())
                .all(|t| t.kind == BarrierKind::Memory)
        );
        let captures = &fixture.session.backend.submissions;
        assert!(
            captures
                .iter()
                .map(|capture| capture.queue)
                .collect::<std::collections::HashSet<_>>()
                .len()
                >= 2
        );
        assert!(captures.iter().all(|capture| {
            capture
                .batches
                .iter()
                .all(|batch| batch.wait_semaphores.is_empty())
        }));

        let mut graph = fixture.session.begin_frame().unwrap();
        let img = import_image(&mut graph, &image, token, true);
        let dst = import_buffer(&mut graph, &target, output_token, "concurrent join");
        read_image_on(
            &mut graph,
            fixture.device.clone(),
            img,
            dst,
            (0, 0, false, 0),
            QueueClass::Transfer,
        );
        read_image_on(
            &mut graph,
            fixture.device.clone(),
            img,
            dst,
            (1, 1, true, 256),
            QueueClass::Transfer,
        );
        graph.export(dst, HOST).unwrap();
        fixture.execute(graph).unwrap();
        let bound = fixture.session.backend.bound_synchronization();
        for queue in [graphics, compute] {
            assert!(bound.waits().iter().any(|wait| matches!(wait.dependency,
                DependencyRef::Point(point) if point.queue.index() == queue.index() && point.value > 0)),
                "join omitted a real producer timeline frontier");
        }
        fixture.session.wait_submits_done(TIMEOUT).unwrap();
        let pixels = read(&target, 320);
        assert!(pixels[..256].iter().all(|&byte| byte == 51));
        assert!(pixels[256..].iter().all(|&byte| byte == 102));
        drop(image);
        drop(target);
        fixture.finish();
    }
}

fn join_buffer(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    source: ResourceHandle,
    target: ResourceHandle,
    fail: bool,
) {
    let mut pass = graph
        .add_pass("join exclusive halves")
        .with_queue(QueueClass::Transfer)
        .with_side_effect();
    pass.add_resource_access_range(source, READ, 0, bytes(0, 64))
        .unwrap();
    pass.add_resource_access_range(target, WRITE, 0, bytes(0, 64))
        .unwrap();
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        if fail {
            return Err(rhapsody::Error::invalid_usage(
                "injected join failure after successful releases",
            ));
        }
        let source = ctx.managed.unwrap().buffer_for(source.raw())?;
        let target = ctx.managed.unwrap().buffer_for(target.raw())?;
        unsafe {
            device.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                source.raw(),
                target.raw(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: 64,
                }],
            );
        }
        Ok(())
    });
    graph.export(target, HOST).unwrap();
}

#[test]
fn exclusive_halves_keep_exact_release_pairs_after_failed_acquire_batch_and_recover() {
    {
        let Some(mut fixture) = Fixture::with_queues(ParallelRecording::PARALLEL, true) else {
            return;
        };
        let (source, token) = fixture.buffer(64, false);
        let (target, output_token) = fixture.buffer(64, true);
        let graphics = fixture
            .device
            .queue_plan()
            .physical(QueueClass::Graphics)
            .family;
        let compute = fixture
            .device
            .queue_plan()
            .physical(QueueClass::Compute)
            .family;
        let transfer = fixture
            .device
            .queue_plan()
            .physical(QueueClass::Transfer)
            .family;
        assert_ne!(graphics, compute);
        assert_ne!(graphics, transfer);
        assert_ne!(compute, transfer);
        let mut graph = fixture.session.begin_frame().unwrap();
        let src = import_buffer(&mut graph, &source, token, "exclusive halves");
        fill_on(
            &mut graph,
            fixture.device.clone(),
            src,
            (0, 32),
            0x12345678,
            QueueClass::Graphics,
        );
        fill_on(
            &mut graph,
            fixture.device.clone(),
            src,
            (32, 32),
            0x9abcdef0,
            QueueClass::Compute,
        );
        fixture.execute(graph).unwrap();
        fixture
            .session
            .backend
            .resource_native_state(token, |cells| {
                assert!(cells.iter().any(|cell| cell.range
                    == OwnershipRange::Buffer {
                        offset: 0,
                        size: 32
                    }
                    && cell.state.owner == RangeOwner::Owned(graphics)));
                assert!(cells.iter().any(|cell| cell.range
                    == OwnershipRange::Buffer {
                        offset: 32,
                        size: 32
                    }
                    && cell.state.owner == RangeOwner::Owned(compute)));
            })
            .unwrap();
        let mut graph = fixture.session.begin_frame().unwrap();
        let src = import_buffer(&mut graph, &source, token, "exclusive halves");
        let dst = import_buffer(&mut graph, &target, output_token, "failed join");
        join_buffer(&mut graph, fixture.device.clone(), src, dst, true);
        assert_eq!(
            fixture.execute(graph).unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
        let released = fixture
            .session
            .backend
            .resource_native_state(token, |cells| {
                cells
                    .iter()
                    .map(|cell| {
                        let RangeOwner::Released {
                            transfer: pair,
                            point,
                        } = cell.state.owner
                        else {
                            panic!(
                                "unsubmitted acquire cannot publish destination ownership: {:?}",
                                cell.state.owner
                            );
                        };
                        assert_eq!(pair.destination_family, transfer);
                        assert_eq!(pair.range, cell.range);
                        assert!(point.value > 0);
                        (cell.range, pair, point)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap();
        assert_eq!(released.len(), 2);
        let mut graph = fixture.session.begin_frame().unwrap();
        let src = import_buffer(&mut graph, &source, token, "exclusive halves");
        let dst = import_buffer(&mut graph, &target, output_token, "recovered join");
        join_buffer(&mut graph, fixture.device.clone(), src, dst, false);
        fixture.execute(graph).unwrap();
        let bound = fixture.session.backend.bound_synchronization();
        for (range, pair, point) in released {
            assert!(bound.transitions().iter().any(|t| t.resource == src.raw()
                && t.range == range
                && t.kind == BarrierKind::Acquire { pair: pair.pair }
                && t.source_family == pair.source_family
                && t.destination_family == pair.destination_family));
            assert!(
                bound
                    .waits()
                    .iter()
                    .any(|wait| wait.dependency == DependencyRef::Point(point))
            );
        }
        fixture.session.wait_submits_done(TIMEOUT).unwrap();
        for (index, word) in read(&target, 64).as_chunks::<4>().0.iter().enumerate() {
            assert_eq!(
                *word,
                if index < 8 {
                    0x12345678u32
                } else {
                    0x9abcdef0u32
                }
                .to_ne_bytes()
            );
        }
        fixture
            .session
            .backend
            .resource_native_state(token, |cells| {
                assert!(
                    cells
                        .iter()
                        .all(|cell| cell.state.owner == RangeOwner::Owned(transfer))
                )
            })
            .unwrap();
        drop(source);
        drop(target);
        fixture.finish();
    }
}
