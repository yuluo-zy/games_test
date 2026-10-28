//! Raster pipeline descriptions and options.
use ash::vk;
use crate::Result;
use std::ffi::CString;
use std::sync::Arc;
use super::layout::PipelineLayout;
use super::layout::ShaderModule;
use super::reflection::InterfaceError;

/// Formats of a dynamic-rendering pipeline; graphics state is explicit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DynamicRenderingFormats {
    pub colors: Vec<vk::Format>,
    pub depth: vk::Format,
    pub stencil: vk::Format,
    /// Multiview is deliberately not enabled by this implementation yet.
    pub view_mask: u32,
}

impl DynamicRenderingFormats {
    pub fn color(format: vk::Format) -> Self {
        Self {
            colors: vec![format],
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            view_mask: 0,
        }
    }
    pub fn validate(&self, max_colors: u32) -> Result<()> {
        if self.view_mask != 0 {
            return Err(crate::Error::unsupported(
                "dynamic rendering multiview is not enabled",
            ));
        }
        if self.colors.len() > max_colors as usize {
            return Err(crate::Error::invalid_usage(
                "dynamic rendering color count exceeds maxColorAttachments",
            ));
        }
        for &format in &self.colors {
            if format != vk::Format::UNDEFINED
                && crate::format::vk_format_valid_aspect_flags(format)
                    != vk::ImageAspectFlags::COLOR
            {
                return Err(crate::Error::invalid_usage(
                    "dynamic rendering color format has depth/stencil aspects",
                ));
            }
        }
        for (format, aspect) in [
            (self.depth, vk::ImageAspectFlags::DEPTH),
            (self.stencil, vk::ImageAspectFlags::STENCIL),
        ] {
            if format != vk::Format::UNDEFINED
                && !crate::format::vk_format_valid_aspect_flags(format).contains(aspect)
            {
                return Err(crate::Error::invalid_usage(
                    "dynamic rendering depth/stencil format lacks its aspect",
                ));
            }
        }
        if self.depth != vk::Format::UNDEFINED
            && self.stencil != vk::Format::UNDEFINED
            && self.depth != self.stencil
        {
            return Err(crate::Error::invalid_usage(
                "dynamic depth and stencil formats must match the shared attachment",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RasterTargetSignature {
    Dynamic {
        formats: DynamicRenderingFormats,
        samples: vk::SampleCountFlags,
    },
}

#[derive(Clone)]
pub struct RasterPipelineDesc {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Option<Arc<ShaderModule>>,
    pub entry: CString,
    /// 原发行 vertex/fragment 入口通常分别为 vs_main/ps_main，不能误用同一名称。
    pub fragment_entry: Option<CString>,
    pub layout: Arc<PipelineLayout>,
    /// Required color/depth/stencil signature for dynamic rendering.
    pub dynamic_rendering: Option<DynamicRenderingFormats>,
    pub vertex_bindings: Vec<vk::VertexInputBindingDescription>,
    pub vertex_attributes: Vec<vk::VertexInputAttributeDescription>,
    pub topology: vk::PrimitiveTopology,
    pub polygon_mode: vk::PolygonMode,
    pub cull_mode: vk::CullModeFlags,
    pub front_face: vk::FrontFace,
    pub samples: vk::SampleCountFlags,
    pub depth_test: bool,
    pub depth_write: bool,
    pub depth_compare: vk::CompareOp,
    pub stencil_test: bool,
    pub stencil: vk::StencilOpState,
    pub blend_attachments: Vec<vk::PipelineColorBlendAttachmentState>,
}

impl RasterPipelineDesc {
    /// 构造带必需 vertex/layout 的默认图形管线描述；其余固定状态由调用方填写。
    pub fn new(vertex: Arc<ShaderModule>, layout: Arc<PipelineLayout>) -> Self {
        Self {
            vertex,
            fragment: None,
            entry: c"vs_main".to_owned(),
            fragment_entry: None,
            layout,
            dynamic_rendering: None,
            vertex_bindings: Vec::new(),
            vertex_attributes: Vec::new(),
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            samples: vk::SampleCountFlags::TYPE_1,
            depth_test: false,
            depth_write: false,
            depth_compare: vk::CompareOp::LESS_OR_EQUAL,
            stencil_test: false,
            stencil: vk::StencilOpState::default(),
            blend_attachments: Vec::new(),
        }
    }
    pub fn use_dynamic_rendering(&mut self, formats: DynamicRenderingFormats) {
        self.dynamic_rendering = Some(formats);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RasterPipelineOpts(pub u32);

impl RasterPipelineOpts {
    pub fn new() -> Self {
        Self(0)
    }
    fn byte(self, index: u32) -> u32 {
        (self.0 >> (index * 8)) & 0xff
    }
    fn set_bits(&mut self, shift: u32, width: u32, value: u32) {
        let mask = ((1u32 << width) - 1) << shift;
        self.0 = (self.0 & !mask) | ((value << shift) & mask);
    }
    pub fn topology(self) -> Result<vk::PrimitiveTopology> {
        match self.byte(0) & 3 {
            0 => Ok(vk::PrimitiveTopology::TRIANGLE_LIST),
            1 => Ok(vk::PrimitiveTopology::LINE_LIST),
            2 => Ok(vk::PrimitiveTopology::LINE_STRIP),
            _ => Err(crate::Error::invalid_usage(
                "RasterPipelineOpts 拓扑编码无效",
            )),
        }
    }
    pub fn set_topology(&mut self, value: vk::PrimitiveTopology) -> Result<()> {
        let code = match value {
            vk::PrimitiveTopology::TRIANGLE_LIST => 0,
            vk::PrimitiveTopology::LINE_LIST => 1,
            vk::PrimitiveTopology::LINE_STRIP => 2,
            other => {
                return Err(crate::Error::invalid_usage(format!(
                    "RasterPipelineOpts 不支持拓扑 {other:?}"
                )));
            }
        };
        self.set_bits(0, 2, code);
        Ok(())
    }
    pub fn cull_mode(self) -> vk::CullModeFlags {
        vk::CullModeFlags::from_raw((self.byte(0) >> 2) & 3)
    }
    pub fn set_cull_mode(&mut self, value: vk::CullModeFlags) {
        self.set_bits(2, 2, value.as_raw());
    }
    pub fn depth_compare_op(self) -> Result<vk::CompareOp> {
        match (self.byte(0) >> 4) & 3 {
            0 => Ok(vk::CompareOp::LESS),
            1 => Ok(vk::CompareOp::GREATER),
            2 => Ok(vk::CompareOp::EQUAL),
            _ => Err(crate::Error::invalid_usage(
                "RasterPipelineOpts 深度比较编码无效",
            )),
        }
    }
    pub fn set_depth_compare_op(&mut self, value: vk::CompareOp) -> Result<()> {
        let code = match value {
            vk::CompareOp::LESS => 0,
            vk::CompareOp::GREATER => 1,
            vk::CompareOp::EQUAL => 2,
            other => {
                return Err(crate::Error::invalid_usage(format!(
                    "RasterPipelineOpts 不支持深度比较 {other:?}"
                )));
            }
        };
        self.set_bits(4, 2, code);
        Ok(())
    }
    pub fn depth_write(self) -> bool {
        (self.byte(0) >> 6) & 1 != 0
    }
    pub fn set_depth_write(&mut self, value: bool) {
        self.set_bits(6, 1, u32::from(value));
    }
    pub fn depth_test(self) -> bool {
        (self.byte(0) >> 7) & 1 != 0
    }
    pub fn set_depth_test_enable(&mut self, value: bool) {
        self.set_bits(7, 1, u32::from(value));
    }
    pub fn stencil_test(self) -> bool {
        self.byte(1) & 1 != 0
    }
    pub fn set_stencil_test_enable(&mut self, value: bool) {
        self.set_bits(8, 1, u32::from(value));
    }
    pub fn stencil_fail_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 1) & 7) as i32)
    }
    pub fn set_stencil_fail_op(&mut self, value: vk::StencilOp) {
        self.set_bits(9, 3, value.as_raw() as u32);
    }
    pub fn stencil_pass_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 4) & 7) as i32)
    }
    pub fn set_stencil_pass_op(&mut self, value: vk::StencilOp) {
        self.set_bits(12, 3, value.as_raw() as u32);
    }
    /// 3 位横跨 b1 bit7（低位）与 b2 bit0–1（高两位）。
    pub fn stencil_depth_fail_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 7) + (self.byte(2) & 3) * 2) as i32)
    }
    pub fn set_stencil_depth_fail_op(&mut self, value: vk::StencilOp) {
        let raw = value.as_raw() as u32;
        self.set_bits(15, 1, raw & 1);
        self.set_bits(16, 2, raw >> 1);
    }
    pub fn stencil_compare_op(self) -> vk::CompareOp {
        vk::CompareOp::from_raw(((self.byte(2) >> 2) & 7) as i32)
    }
    pub fn set_stencil_compare_op(&mut self, value: vk::CompareOp) {
        self.set_bits(18, 3, value.as_raw() as u32);
    }
    /// 8 位横跨 b2 bit5–7（低 3 位）与 b3 bit0–4（高 5 位）。
    pub fn stencil_compare_mask(self) -> u32 {
        ((self.byte(3) << 3) | (self.byte(2) >> 5)) & 0xff
    }
    pub fn set_stencil_compare_mask(&mut self, value: u32) {
        self.set_bits(21, 3, value & 7);
        self.set_bits(24, 5, (value >> 3) & 0x1f);
    }
    pub fn blend_mode(self) -> u8 {
        ((self.byte(3) >> 5) & 3) as u8
    }
    pub fn set_blend_mode(&mut self, value: u8) {
        self.set_bits(29, 2, u32::from(value));
    }
    pub fn polygon_mode(self) -> vk::PolygonMode {
        if self.byte(3) >> 7 != 0 {
            vk::PolygonMode::LINE
        } else {
            vk::PolygonMode::FILL
        }
    }
    pub fn set_polygon_mode(&mut self, value: vk::PolygonMode) -> Result<()> {
        let code = match value {
            vk::PolygonMode::FILL => 0,
            vk::PolygonMode::LINE => 1,
            other => {
                return Err(crate::Error::invalid_usage(format!(
                    "RasterPipelineOpts 不支持多边形模式 {other:?}"
                )));
            }
        };
        self.set_bits(31, 1, code);
        Ok(())
    }
    /// front/back 共用的模板状态；write mask 恒 `0xffffffff`，reference 0（动态）。
    pub fn stencil_state(self) -> vk::StencilOpState {
        vk::StencilOpState::default()
            .fail_op(self.stencil_fail_op())
            .pass_op(self.stencil_pass_op())
            .depth_fail_op(self.stencil_depth_fail_op())
            .compare_op(self.stencil_compare_op())
            .compare_mask(self.stencil_compare_mask())
            .write_mask(0xffff_ffff)
            .reference(0)
    }
    /// 把位域展开到管线描述（固定状态部分；`front_face` 恒 COUNTER_CLOCKWISE=0，线宽 1.0）。
    /// The blend attachment state is replicated for every color attachment.
    pub fn apply_to(self, desc: &mut RasterPipelineDesc, color_attachments: usize) -> Result<()> {
        // Decode every fallible field before mutating the caller's description.
        let topology = self.topology()?;
        let depth_compare = self.depth_compare_op()?;
        desc.topology = topology;
        desc.polygon_mode = self.polygon_mode();
        desc.cull_mode = self.cull_mode();
        desc.front_face = vk::FrontFace::COUNTER_CLOCKWISE;
        desc.depth_test = self.depth_test();
        desc.depth_write = self.depth_write();
        desc.depth_compare = depth_compare;
        desc.stencil_test = self.stencil_test();
        desc.stencil = self.stencil_state();
        desc.blend_attachments = vec![get_vk_blend_mode(self.blend_mode() << 5); color_attachments];
        Ok(())
    }
}

/// Blend state from the two blend-mode bits (5..=6) of the encoded raster flags.
pub fn get_vk_blend_mode(encoded_raster_flags: u8) -> vk::PipelineColorBlendAttachmentState {
    let mode = usize::from(encoded_raster_flags >> 5 & 3);
    let enabled = [0, 1, 1, 1][mode];
    let dst_color = [0, 1, 7, 16][mode];
    let dst_alpha = [0, 1, 7, 18][mode];
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(enabled != 0)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::from_raw(dst_color))
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::from_raw(dst_alpha))
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}

/// 原类型 tag 2/3/4 对应 32 位有符号/无符号/浮点，字节宽度仅允许 4/8/12/16。
pub fn map_vertex_attribute_format(type_tag: u32, bytes: u64) -> Result<vk::Format> {
    let base = match type_tag {
        2 => 99,
        3 => 98,
        4 => 100,
        _ => return Err(InterfaceError::InvalidLayout.into()),
    };
    if !(4..=16).contains(&bytes) || !bytes.is_multiple_of(4) {
        return Err(InterfaceError::InvalidLayout.into());
    }
    Ok(vk::Format::from_raw(base + 3 * (bytes as i32 / 4 - 1)))
}

pub fn vertex_input_from_reflection(
    reflection: &shader_pipeline::reflection::ShaderReflection,
) -> Result<(
    Vec<vk::VertexInputBindingDescription>,
    Vec<vk::VertexInputAttributeDescription>,
)> {
    use shader_pipeline::reflection_ty::{ScalarType, Type};
    let mut bindings = Vec::new();
    let mut attributes = Vec::new();
    for input in reflection.effective_vertex_inputs().into_values() {
        let bytes = input
            .ty
            .nbyte()
            .ok_or(InterfaceError::VertexInputByteCount)? as u64;
        let scalar = match &input.ty {
            Type::Scalar(s) => s,
            Type::Vector(v) => &v.scalar_ty,
            other => {
                return Err(
                    InterfaceError::UnsupportedVertexAttributeType(format!("{other:?}")).into(),
                );
            }
        };
        // The reflection schema stores scalar *bytes*, not SPIR-V bits: the
        // attribute format uses the full Type::nbyte() and the scalar tag;
        // the vertex stride uses that same complete attribute byte count.
        let tag = match scalar {
            ScalarType::Signed(_) => 2,
            ScalarType::Unsigned(_) => 3,
            ScalarType::Float(_) => 4,
            _ => return Err(InterfaceError::InvalidLayout.into()),
        };
        let format = map_vertex_attribute_format(tag, bytes)?;
        bindings.push(
            vk::VertexInputBindingDescription::default()
                .binding(input.location)
                .stride(bytes as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
        );
        attributes.push(
            vk::VertexInputAttributeDescription::default()
                .location(input.location)
                .binding(input.location)
                .format(format)
                .offset(0),
        );
    }
    Ok((bindings, attributes))
}
