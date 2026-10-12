use ash::vk::{self, Handle};
use rhapsody::rg::pass_api::{Command, CommandBackend, PassApiError, PipelineKind, RenderPassApi};
use serde_json::{Value, json};

#[derive(Default)]
struct Capture(Vec<Value>);

impl CommandBackend for Capture {
    fn dispatch(&mut self, cb: vk::CommandBuffer, command: Command<'_>) {
        let call = match command {
            Command::BeginRenderPass(info) => json!([
                "begin",
                info.render_pass.as_raw(),
                info.framebuffer.as_raw(),
                info.render_area.extent.width,
                info.render_area.extent.height,
                info.clear_value_count
            ]),
            Command::EndRenderPass => json!(["end"]),
            Command::SetViewport(v) => json!([
                "viewport",
                v.x,
                v.y,
                v.width,
                v.height,
                v.min_depth,
                v.max_depth
            ]),
            Command::SetScissor(v) => json!([
                "scissor",
                v.offset.x,
                v.offset.y,
                v.extent.width,
                v.extent.height
            ]),
            Command::SetStencilRef(v) => json!(["stencil", 3, v]),
            Command::SetLineWidth(v) => json!(["line_width", v]),
            Command::BindVertexBuffer { binding, buffer } => {
                json!(["vertex", binding, buffer.as_raw(), 0])
            }
            Command::BindIndexBuffer { buffer, ty } => {
                json!(["index", buffer.as_raw(), 0, ty.as_raw()])
            }
            Command::PushConstants {
                layout,
                stages,
                bytes,
            } => json!(["push", layout.as_raw(), stages.as_raw(), 0, bytes]),
            Command::CopyBuffer {
                source,
                destination,
                regions,
            } => json!([
                "copy",
                source.as_raw(),
                destination.as_raw(),
                regions
                    .iter()
                    .map(|v| [v.src_offset, v.dst_offset, v.size])
                    .collect::<Vec<_>>()
            ]),
            Command::FillBuffer {
                buffer,
                offset,
                size,
                data,
            } => json!(["fill", buffer.as_raw(), offset, size, data]),
            Command::ClearColorImage { image, range } => json!([
                "clear",
                image.as_raw(),
                7,
                [0.0, 0.0, 0.0, 0.0],
                [
                    range.aspect_mask.as_raw(),
                    range.base_mip_level,
                    range.level_count,
                    range.base_array_layer,
                    range.layer_count
                ]
            ]),
        };
        self.0.push(json!({"cb": cb.as_raw(), "call": call}));
    }
}

#[test]
fn command_sequence_binary_fixture() {
    let mut backend = Capture::default();
    {
        let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(100), 0, true);
        let extent = vk::Extent2D {
            width: 1920,
            height: 1080,
        };
        let info = vk::RenderPassBeginInfo::default()
            .render_pass(vk::RenderPass::from_raw(10))
            .framebuffer(vk::Framebuffer::from_raw(11))
            .render_area(vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent,
            });
        api.begin_render_pass(&info);
        api.set_default_view_and_scissor(extent);
        api.set_stencil_ref(0);
        api.set_stencil_ref(255);
        api.set_stencil_ref(255);
        api.set_line_width(2.5);
        api.bind_vertex_buffer_by_binding_point(3, vk::Buffer::from_raw(20));
        api.bind_index_buffer(vk::Buffer::from_raw(21), vk::IndexType::UINT32);
        api.push_constants(
            vk::PipelineLayout::from_raw(30),
            PipelineKind::Graphics,
            &[1, 2, 3, 4],
        );
        api.push_constants(
            vk::PipelineLayout::from_raw(31),
            PipelineKind::Compute,
            &[5, 6, 7, 8],
        );
        api.end_render_pass();
        api.copy_buffer(
            vk::Buffer::from_raw(20),
            vk::Buffer::from_raw(21),
            &[vk::BufferCopy {
                src_offset: 4,
                dst_offset: 8,
                size: 16,
            }],
        );
        api.fill_buffer(vk::Buffer::from_raw(21), None, 0xdeadbeef)
            .unwrap();
        api.fill_buffer(vk::Buffer::from_raw(21), Some((4, 16)), 42)
            .unwrap();
        api.clear_color_image(vk::Image::from_raw(40), 4, 6);
    }
    assert_eq!(backend.0.len(), 14);
    assert_eq!(
        backend.0[1]["call"],
        json!(["viewport", 0.0, 1080.0, 1920.0, -1080.0, 0.0, 1.0])
    );
    assert_eq!(backend.0[3]["call"], json!(["stencil", 3, 255]));
    // 该输出用于限定原入口差分；候选测试本身不代表执行过原二进制。
    println!("PASS_API_FIXTURE={}", json!(backend.0));
}

#[test]
fn invalid_fill_range_never_reaches_backend() {
    let mut backend = Capture::default();
    let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(100), 0, false);
    assert_eq!(
        api.fill_buffer(vk::Buffer::from_raw(21), Some((1, 16)), 0),
        Err(PassApiError::UnalignedFillOffset)
    );
    assert_eq!(
        api.fill_buffer(vk::Buffer::from_raw(21), Some((4, 17)), 0),
        Err(PassApiError::UnalignedFillSize)
    );
    api.set_line_width(9.0);
    assert!(backend.0.is_empty());
}

#[test]
fn negative_zero_height_and_existing_stencil_cache() {
    let mut backend = Capture::default();
    let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(1), 7, true);
    api.set_stencil_ref(7);
    api.set_default_view_and_scissor(vk::Extent2D {
        width: u32::MAX,
        height: 0,
    });
    assert_eq!(backend.0.len(), 2);
    assert_eq!(
        backend.0[0]["call"][4].as_f64().unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
}
