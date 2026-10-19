//! S08 production queue scheduling: independent branches, real paired exclusive
//! ownership and graphics export convergence. Transfer commands are valid on
//! all three classes, so the fixture needs no optional shader capability.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    device::{
        Device, DeviceConfig, Feature, QueueClass, QueueConfig, QueuePreference, SubmissionApi,
    },
    raw::{
        buffer::{Buffer, BufferDesc},
        instance::{InstanceBuilder, ValidationMessenger},
    },
    rg::{
        executor::PassContext,
        graph::RenderGraph,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
const HOST: u8 = 0x26;
const SIZE: u64 = 64;
const TIMEOUT: u64 = 10_000_000_000;
struct Fixture {
    device: Arc<Device>,
    session: VulkanGraphSession,
    buffers: [Arc<Buffer>; 3],
    tokens: [u64; 3],
    validation: ValidationMessenger,
}
impl Fixture {
    fn new(independent: bool) -> Option<Self> {
        Self::with_submission_api(independent, SubmissionApi::Auto)
    }
    fn with_submission_api(independent: bool, submission_api: SubmissionApi) -> Option<Self> {
        let preference = if independent {
            QueuePreference::Require
        } else {
            QueuePreference::Off
        };
        let (device, validation) = gpu_test_support::initialize(
            "multi-queue graph",
            (|| -> rhapsody::Result<_> {
                let mut builder = InstanceBuilder::default();
                builder.debug_markers();
                builder
                    .layers
                    .push(std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap());
                let config = DeviceConfig {
                    submission_api,
                    validation: true,
                    queues: QueueConfig {
                        async_compute: preference,
                        transfer: preference,
                        require_present: false,
                    },
                    ..DeviceConfig::headless()
                };
                let device = Device::new(builder.build()?, None, &config)?;
                let validation = ValidationMessenger::new(device.instance())?;
                Ok((device, validation))
            })(),
        )?;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
        let buffers = [
            Arc::new(Buffer::new(device.clone(), &BufferDesc::new_gpu_only(SIZE, usage)).unwrap()),
            Arc::new(Buffer::new(device.clone(), &BufferDesc::new_gpu_only(SIZE, usage)).unwrap()),
            Arc::new(
                Buffer::new(device.clone(), &BufferDesc::new_gpu_to_cpu(SIZE * 2, usage)).unwrap(),
            ),
        ];
        let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
        let tokens = buffers.each_ref().map(|buffer| {
            backend
                .bind_import(GpuResource::Buffer(buffer.clone()))
                .unwrap()
        });
        Some(Self {
            device,
            session: VulkanGraphSession::new(backend),
            buffers,
            tokens,
            validation,
        })
    }
    fn graph(&mut self, fail_join: bool) -> RenderGraph {
        let mut graph = self.session.begin_frame().unwrap();
        let resources = self
            .buffers
            .each_ref()
            .into_iter()
            .zip(self.tokens)
            .enumerate()
            .map(|(index, (buffer, token))| {
                if index == 2 {
                    graph
                        .import_unowned(
                            format!("buffer {index}"),
                            ResourceSpec::Buffer(buffer.desc).descriptor(),
                            token,
                            HOST,
                        )
                        .unwrap()
                } else {
                    self.session
                        .backend
                        .declare(
                            &mut graph,
                            &format!("buffer {index}"),
                            ResourceSpec::Buffer(buffer.desc),
                        )
                        .unwrap()
                }
            })
            .collect::<Vec<_>>();
        for (index, class, value) in [
            (0, QueueClass::Graphics, 0x11223344),
            (1, QueueClass::Compute, 0x55667788),
        ] {
            let mut pass = graph
                .add_pass(format!("independent {index}"))
                .with_queue(class);
            pass.add_resource_access(resources[index], WRITE, 0)
                .unwrap();
            let device = self.device.clone();
            let resource = resources[index].raw();
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let binding = ctx
                    .managed
                    .unwrap()
                    .buffer_for(resource)
                    .map_err(|error| rhapsody::Error::invalid_usage_with("binding", error))?;

                unsafe {
                    device.raw_device().cmd_fill_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        binding.raw(),
                        0,
                        SIZE,
                        value,
                    );
                }
                Ok(())
            });
        }
        let mut pass = graph.add_pass("join").with_queue(QueueClass::Transfer);
        for &input in &resources[..2] {
            pass.add_resource_access(input, READ, 0).unwrap();
        }
        pass.add_resource_access(resources[2], WRITE, 0).unwrap();
        let device = self.device.clone();
        let ids = resources
            .iter()
            .map(|resource| resource.raw())
            .collect::<Vec<_>>();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            if fail_join {
                return Err(rhapsody::Error::invalid_usage(
                    "injected join failure after ownership acquire",
                ));
            }
            let bindings = ids
                .iter()
                .map(|&resource| {
                    ctx.managed
                        .unwrap()
                        .buffer_for(resource)
                        .map_err(|error| rhapsody::Error::invalid_usage_with("binding", error))
                })
                .collect::<rhapsody::Result<Vec<_>>>()?;
            for index in 0..2 {
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        bindings[index].raw(),
                        bindings[2].raw(),
                        &[vk::BufferCopy {
                            src_offset: 0,
                            dst_offset: index as u64 * SIZE,
                            size: SIZE,
                        }],
                    );
                }
            }
            Ok(())
        });
        graph.export(resources[2], HOST).unwrap();
        graph
    }
    fn pixels(&mut self) -> Vec<u8> {
        self.session.wait_submits_done(TIMEOUT).unwrap();
        let mut pixels = vec![0; SIZE as usize * 2];
        unsafe {
            self.buffers[2]
                .read_bytes_after_completion(0, &mut pixels)
                .unwrap();
        }
        pixels
    }
}
#[test]
fn real_independent_queues_preserve_pixels_and_do_not_wait_between_branches() {
    let Some(mut baseline) = Fixture::new(false) else {
        return;
    };
    let graph = baseline.graph(false);
    let single = baseline.session.execute_frame(graph.build()).unwrap();
    assert_eq!(single.receipts.len(), 1);
    let expected = baseline.pixels();
    let Some(mut multi) = Fixture::new(true) else {
        return;
    };
    let graph = multi.graph(false);
    let frame = multi.session.execute_frame(graph.build()).unwrap();
    assert!(
        frame.receipts.len() > 4,
        "real release/acquire submissions join all actual queue families"
    );
    let captures = &multi.session.backend.submissions;
    assert_ne!(captures[0].queue, captures[1].queue);
    // The first extra submission for B can be a source-family release. Its only
    // dependency may concern B; the independent A receipt must never be added.
    let a_semaphore = multi.session.backend.timeline.handle;
    for capture in captures.iter().take(2).skip(1) {
        assert!(
            capture
                .batches
                .iter()
                .all(|batch| !batch.wait_semaphores.contains(&a_semaphore)),
            "independent B branch was serialized behind A"
        );
    }
    let actual = multi.pixels();
    assert_eq!(actual, expected);
    assert_eq!(&actual[..4], &0x11223344u32.to_ne_bytes());
    assert_eq!(
        &actual[SIZE as usize..SIZE as usize + 4],
        &0x55667788u32.to_ne_bytes()
    );
}
#[test]
fn successful_acquires_survive_later_recording_failure_and_next_frame_recovers() {
    let Some(mut fixture) = Fixture::new(true) else {
        return;
    };
    let graph = fixture.graph(true);
    assert!(fixture.session.execute_frame(graph.build()).is_err());
    fixture.session.wait_submits_done(TIMEOUT).unwrap();
    let graph = fixture.graph(false);
    fixture.session.execute_frame(graph.build()).unwrap();
    let pixels = fixture.pixels();
    assert_eq!(&pixels[..4], &0x11223344u32.to_ne_bytes());
    assert_eq!(
        &pixels[SIZE as usize..SIZE as usize + 4],
        &0x55667788u32.to_ne_bytes()
    );
}

#[test]
fn barrier2_and_legacy_ownership_scopes_preserve_pixels_and_partial_recovery() {
    let mut reference = None;
    for api in [SubmissionApi::Auto, SubmissionApi::Legacy] {
        let Some(mut fixture) = Fixture::with_submission_api(true, api) else {
            return;
        };
        let sync2 = fixture.device.caps().is_enabled(Feature::Synchronization2);
        if api == SubmissionApi::Legacy {
            assert!(!sync2);
        } else {
            assert_eq!(
                sync2,
                fixture
                    .device
                    .caps()
                    .is_supported(Feature::Synchronization2)
            );
        }
        let graph = fixture.graph(true);
        assert!(fixture.session.execute_frame(graph.build()).is_err());
        assert!(
            fixture.session.pending_frame_count() > 0,
            "successful ownership preludes must retain real completion receipts"
        );
        fixture.session.wait_submits_done(TIMEOUT).unwrap();
        for _ in 0..3 {
            let graph = fixture.graph(false);
            fixture.session.execute_frame(graph.build()).unwrap();
            let actual = fixture.pixels();
            assert_eq!(&actual[..4], &0x11223344u32.to_ne_bytes());
            assert_eq!(
                &actual[SIZE as usize..SIZE as usize + 4],
                &0x55667788u32.to_ne_bytes()
            );
            if let Some(expected) = &reference {
                assert_eq!(&actual, expected);
            } else {
                reference = Some(actual);
            }
        }
        eprintln!(
            "ownership scope API {api:?}: synchronization2 enabled={sync2}; partial recovery and 3 real outputs matched"
        );
        let validation = fixture.validation.collector.clone();
        drop(fixture);
        assert!(validation.errors().is_empty(), "{:?}", validation.errors());
        assert!(
            validation.warnings().is_empty(),
            "{:?}",
            validation.warnings()
        );
    }
}

#[test]
fn concurrent_disjoint_writes_do_not_inherit_whole_object_waits() {
    use rhapsody::{
        resource::{QueueFamilies, ResourceSharing},
        rg::resource::AccessRange,
    };
    let Some(mut fixture) = Fixture::new(true) else {
        return;
    };
    let mut families = fixture
        .device
        .queue_plan()
        .queues
        .iter()
        .map(|queue| queue.family)
        .collect::<Vec<_>>();
    families.sort_unstable();
    families.dedup();
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&families).unwrap());
    let shared = Arc::new(
        Buffer::new_concurrent(
            fixture.device.clone(),
            &BufferDesc::new_gpu_only(
                SIZE * 2,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
            &families,
        )
        .unwrap(),
    );
    let token = fixture
        .session
        .backend
        .bind_import(GpuResource::Buffer(shared.clone()))
        .unwrap();
    let mut graph = fixture.session.begin_frame().unwrap();
    let input = graph
        .import_unowned_with_sharing(
            "concurrent",
            ResourceSpec::Buffer(shared.desc).descriptor(),
            token,
            0,
            sharing,
        )
        .unwrap();
    let output = graph
        .import_unowned(
            "output",
            ResourceSpec::Buffer(fixture.buffers[2].desc).descriptor(),
            fixture.tokens[2],
            HOST,
        )
        .unwrap();
    for (index, class, value) in [
        (0, QueueClass::Graphics, 0x11223344),
        (1, QueueClass::Compute, 0x55667788),
    ] {
        let mut pass = graph
            .add_pass(format!("disjoint {index}"))
            .with_queue(class);
        pass.add_resource_access_range(
            input,
            WRITE,
            0,
            AccessRange::Buffer {
                offset: index * SIZE,
                size: SIZE,
            },
        )
        .unwrap();
        let shared = shared.clone();
        let device = fixture.device.clone();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            unsafe {
                device.raw_device().cmd_fill_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    shared.handle,
                    index * SIZE,
                    SIZE,
                    value,
                );
            }
            Ok(())
        });
    }
    let mut pass = graph.add_pass("join both ranges");
    pass.add_resource_access(input, READ, 0).unwrap();
    pass.add_resource_access(output, WRITE, 0).unwrap();
    let destination = fixture.buffers[2].clone();
    let device = fixture.device.clone();
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        unsafe {
            device.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                shared.handle,
                destination.handle,
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: SIZE * 2,
                }],
            );
        }
        Ok(())
    });
    graph.export(output, HOST).unwrap();
    fixture.session.execute_frame(graph.build()).unwrap();
    let captures = &fixture.session.backend.submissions;
    assert_ne!(captures[0].queue, captures[1].queue);
    assert!(
        captures[1]
            .batches
            .iter()
            .all(|batch| batch.wait_semaphores.is_empty()),
        "independent disjoint write inherited an unnecessary whole-resource wait"
    );
    let pixels = fixture.pixels();
    for (index, word) in pixels.as_chunks::<4>().0.iter().enumerate() {
        assert_eq!(
            word,
            &if index < SIZE as usize / 4 {
                0x11223344u32
            } else {
                0x55667788u32
            }
            .to_ne_bytes()
        );
    }
}

#[test]
fn concurrent_temporal_image_preserves_its_real_sharing_across_frames() {
    use rhapsody::{
        raw::texture::TextureDesc,
        resource::{QueueFamilies, ResourceSharing},
    };
    let Some(mut fixture) = Fixture::new(true) else {
        return;
    };
    let mut families = fixture
        .device
        .queue_plan()
        .queues
        .iter()
        .map(|queue| queue.family)
        .collect::<Vec<_>>();
    families.sort_unstable();
    families.dedup();
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&families).unwrap());
    let desc = TextureDesc::new_2d(2, 2, vk::Format::R8G8B8A8_UNORM).usage(
        vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::COLOR_ATTACHMENT,
    );
    for frame in 0..2 {
        let mut graph = fixture.session.begin_frame().unwrap();
        let lookup = fixture
            .session
            .named_texture_with_sharing(&mut graph, "shared history", desc, sharing)
            .unwrap();
        assert_eq!(lookup.history_valid, frame != 0);
        let image = fixture
            .session
            .named_texture_resource(&graph, lookup.texture)
            .unwrap();
        assert_eq!(image.sharing(), sharing);
        let out = graph
            .import_unowned(
                "output",
                ResourceSpec::Buffer(fixture.buffers[2].desc).descriptor(),
                fixture.tokens[2],
                HOST,
            )
            .unwrap();
        let mut pass = graph
            .add_pass("write concurrent history")
            .with_queue(QueueClass::Compute);
        pass.add_resource_access(lookup.texture.raw(), WRITE, 0)
            .unwrap();
        let target = image.clone();
        let device = fixture.device.clone();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            unsafe {
                device.raw_device().cmd_clear_color_image(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    target.handle,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue {
                        float32: [1., 0., 0., 1.],
                    },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1)],
                );
            }
            Ok(())
        });
        let mut pass = graph
            .add_pass("read concurrent history")
            .with_queue(QueueClass::Transfer);
        pass.add_resource_access(lookup.texture.raw(), READ, 0)
            .unwrap();
        pass.add_resource_access(out, WRITE, 0).unwrap();
        let output = fixture.buffers[2].clone();
        let device = fixture.device.clone();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            unsafe {
                device.raw_device().cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    image.handle,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    output.handle,
                    &[vk::BufferImageCopy::default()
                        .image_subresource(
                            vk::ImageSubresourceLayers::default()
                                .aspect_mask(vk::ImageAspectFlags::COLOR)
                                .layer_count(1),
                        )
                        .image_extent(vk::Extent3D {
                            width: 2,
                            height: 2,
                            depth: 1,
                        })],
                );
            }
            Ok(())
        });
        graph.export(out, HOST).unwrap();
        fixture.session.execute_frame(graph.build()).unwrap();
        let pixels = fixture.pixels();
        assert!(
            pixels[..16]
                .as_chunks::<4>()
                .0
                .iter()
                .all(|pixel| *pixel == [255, 0, 0, 255])
        );
    }
}
