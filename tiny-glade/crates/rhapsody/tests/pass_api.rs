use ash::vk::{self, Handle};
use rhapsody::rg::pass_api::{Command, CommandBackend, PassApiError, PipelineKind, RenderPassApi};
use serde_json::{Value, json};

#[derive(Default)]
struct Capture(Vec<Value>, bool);

impl CommandBackend for Capture {
    fn supports_dynamic_rendering(&self) -> bool {
        self.1
    }
    fn dispatch(&mut self, cb: vk::CommandBuffer, command: Command<'_>) {
        let call = match command {
            Command::BeginRendering(info) => json!([
                "begin_dynamic",
                info.color_attachment_count,
                info.layer_count,
                info.view_mask
            ]),
            Command::EndRendering => json!(["end_dynamic"]),
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
            Command::Draw {
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            } => json!([
                "draw",
                vertex_count,
                instance_count,
                first_vertex,
                first_instance
            ]),
            Command::DrawIndexed {
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            } => json!([
                "draw_indexed",
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance
            ]),
            Command::DrawIndirect {
                buffer,
                offset,
                draw_count,
                stride,
            } => json!(["draw_indirect", buffer.as_raw(), offset, draw_count, stride]),
            Command::DrawIndirectCount {
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            } => json!([
                "draw_indirect_count",
                buffer.as_raw(),
                offset,
                count_buffer.as_raw(),
                count_buffer_offset,
                max_draw_count,
                stride
            ]),
            Command::DrawIndexedIndirect {
                buffer,
                offset,
                draw_count,
                stride,
            } => json!([
                "draw_indexed_indirect",
                buffer.as_raw(),
                offset,
                draw_count,
                stride
            ]),
            Command::DrawIndexedIndirectCount {
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            } => json!([
                "draw_indexed_indirect_count",
                buffer.as_raw(),
                offset,
                count_buffer.as_raw(),
                count_buffer_offset,
                max_draw_count,
                stride
            ]),
            Command::DispatchGroups { x, y, z } => json!(["dispatch", x, y, z]),
            Command::DispatchIndirect { buffer, offset } => {
                json!(["dispatch_indirect", buffer.as_raw(), offset])
            }
        };
        self.0.push(json!({"cb": cb.as_raw(), "call": call}));
    }
}

#[test]
fn dynamic_command_sequence_keeps_draw_state_and_copy_parameters() {
    let mut backend = Capture(Vec::new(), true);
    {
        let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(100), 0, true);
        let extent = vk::Extent2D {
            width: 1920,
            height: 1080,
        };
        let info = vk::RenderingInfo::default()
            .layer_count(1)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent,
            });
        api.begin_dynamic_rendering(&info).unwrap();
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
        api.draw(3, 1, 0, 0);
        api.draw_indexed(6, 1, 0, 0, 0);
        api.draw_indirect(vk::Buffer::from_raw(50), 0, 1, 20);
        api.draw_indirect_count(
            vk::Buffer::from_raw(50),
            0,
            vk::Buffer::from_raw(51),
            0,
            4,
            20,
        );
        api.draw_indexed_indirect(vk::Buffer::from_raw(52), 0, 1, 20);
        api.draw_indexed_indirect_count(
            vk::Buffer::from_raw(52),
            0,
            vk::Buffer::from_raw(51),
            0,
            4,
            20,
        );
        api.dispatch_groups_3d(8, 8, 1);
        api.dispatch_threads_3d(
            64,
            64,
            1,
            std::num::NonZeroU32::new(8).unwrap(),
            std::num::NonZeroU32::new(8).unwrap(),
            std::num::NonZeroU32::new(1).unwrap(),
        );
        api.dispatch_indirect(vk::Buffer::from_raw(53), 0);
        assert_eq!(api.draw_call_count(), 9);
    }
    assert_eq!(backend.0.len(), 23);
    assert_eq!(
        backend.0[1]["call"],
        json!(["viewport", 0.0, 1080.0, 1920.0, -1080.0, 0.0, 1.0])
    );
    assert_eq!(backend.0[3]["call"], json!(["stencil", 3, 255]));
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

#[test]
fn dynamic_rendering_dispatch_has_matching_end_and_capability_guard() {
    struct DynamicCapture(Capture);
    impl CommandBackend for DynamicCapture {
        fn dispatch(&mut self, cb: vk::CommandBuffer, command: Command<'_>) {
            self.0.dispatch(cb, command);
        }
        fn supports_dynamic_rendering(&self) -> bool {
            true
        }
    }
    let info = vk::RenderingInfo::default().layer_count(1).view_mask(0);
    let mut unsupported = Capture::default();
    let mut api = RenderPassApi::new(&mut unsupported, vk::CommandBuffer::from_raw(100), 0, false);
    assert_eq!(
        api.begin_dynamic_rendering(&info).unwrap_err().kind(),
        rhapsody::ErrorKind::Unsupported
    );
    assert!(unsupported.0.is_empty());
    let mut supported = DynamicCapture(Capture::default());
    let mut api = RenderPassApi::new(&mut supported, vk::CommandBuffer::from_raw(100), 0, false);
    api.begin_dynamic_rendering(&info).unwrap();
    api.end_render_pass();
    assert_eq!(
        supported.0.0,
        vec![
            json!({"cb":100,"call":["begin_dynamic",0,1,0]}),
            json!({"cb":100,"call":["end_dynamic"]})
        ]
    );
}
