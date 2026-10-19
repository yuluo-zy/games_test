//! The graph's u8 access-code vocabulary: each code names one typed
//! [`AccessSpec`] (stages, access, write flag and image layout).
//!
//! Codes follow the `vk-sync` `AccessType` order with six extra multi-stage
//! read codes (0x1b..=0x1f, 0x23); write codes are exactly `0x28..=0x37`.
//! Barriers are never derived from codes directly: the render graph lowers
//! each code to its [`AccessScope`] and the shared range-state machine and
//! barrier builder (`resource::range_state`, `raw::barrier`) take it from
//! there.
use crate::resource::range_state::AccessScope;
use ash::vk::{self, AccessFlags as A, ImageLayout as L, PipelineStageFlags as S};

/// The native semantics of one access code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccessSpec {
    pub scope: AccessScope,
    pub layout: vk::ImageLayout,
}

pub const ACCESS_CODE_COUNT: usize = 64;

const fn stages(bits: &[S]) -> S {
    let mut raw = 0;
    let mut index = 0;
    while index < bits.len() {
        raw |= bits[index].as_raw();
        index += 1;
    }
    S::from_raw(raw)
}
const fn access(bits: &[A]) -> A {
    let mut raw = 0;
    let mut index = 0;
    while index < bits.len() {
        raw |= bits[index].as_raw();
        index += 1;
    }
    A::from_raw(raw)
}
const fn spec(code: u8, stages: S, access: A, layout: L) -> AccessSpec {
    AccessSpec {
        scope: AccessScope {
            stages,
            access,
            write: is_write(code),
        },
        layout,
    }
}

pub static ACCESS_TABLE: [AccessSpec; ACCESS_CODE_COUNT] = [
    // 0x00 Nothing
    spec(0x00, S::empty(), A::empty(), L::UNDEFINED),
    // 0x01 CommandBufferReadNVX
    spec(
        0x01,
        S::COMMAND_PREPROCESS_NV,
        A::COMMAND_PREPROCESS_READ_NV,
        L::UNDEFINED,
    ),
    // 0x02 IndirectBuffer
    spec(
        0x02,
        S::ALL_COMMANDS,
        access(&[A::INDIRECT_COMMAND_READ, A::SHADER_READ]),
        L::UNDEFINED,
    ),
    // 0x03 IndexBuffer
    spec(0x03, S::VERTEX_INPUT, A::INDEX_READ, L::UNDEFINED),
    // 0x04 VertexBuffer
    spec(
        0x04,
        S::VERTEX_INPUT,
        A::VERTEX_ATTRIBUTE_READ,
        L::UNDEFINED,
    ),
    // 0x05 VertexShaderReadUniformBuffer
    spec(0x05, S::VERTEX_SHADER, A::SHADER_READ, L::UNDEFINED),
    // 0x06 VertexShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x06,
        S::VERTEX_SHADER,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x07 VertexShaderReadOther
    spec(0x07, S::VERTEX_SHADER, A::SHADER_READ, L::GENERAL),
    // 0x08 TessellationControlShaderReadUniformBuffer
    spec(
        0x08,
        S::TESSELLATION_CONTROL_SHADER,
        A::UNIFORM_READ,
        L::UNDEFINED,
    ),
    // 0x09 TessellationControlShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x09,
        S::TESSELLATION_CONTROL_SHADER,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x0a TessellationControlShaderReadOther
    spec(
        0x0a,
        S::TESSELLATION_CONTROL_SHADER,
        A::SHADER_READ,
        L::GENERAL,
    ),
    // 0x0b TessellationEvaluationShaderReadUniformBuffer
    spec(
        0x0b,
        S::TESSELLATION_EVALUATION_SHADER,
        A::UNIFORM_READ,
        L::UNDEFINED,
    ),
    // 0x0c TessellationEvaluationShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x0c,
        S::TESSELLATION_EVALUATION_SHADER,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x0d TessellationEvaluationShaderReadOther
    spec(
        0x0d,
        S::TESSELLATION_EVALUATION_SHADER,
        A::SHADER_READ,
        L::GENERAL,
    ),
    // 0x0e GeometryShaderReadUniformBuffer
    spec(0x0e, S::GEOMETRY_SHADER, A::UNIFORM_READ, L::UNDEFINED),
    // 0x0f GeometryShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x0f,
        S::GEOMETRY_SHADER,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x10 GeometryShaderReadOther
    spec(0x10, S::GEOMETRY_SHADER, A::SHADER_READ, L::GENERAL),
    // 0x11 FragmentShaderReadUniformBuffer
    spec(0x11, S::FRAGMENT_SHADER, A::UNIFORM_READ, L::UNDEFINED),
    // 0x12 FragmentShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x12,
        S::FRAGMENT_SHADER,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x13 FragmentShaderReadColorInputAttachment
    spec(
        0x13,
        S::FRAGMENT_SHADER,
        A::INPUT_ATTACHMENT_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x14 FragmentShaderReadDepthStencilInputAttachment
    spec(
        0x14,
        S::FRAGMENT_SHADER,
        A::INPUT_ATTACHMENT_READ,
        L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    ),
    // 0x15 FragmentShaderReadOther
    spec(0x15, S::FRAGMENT_SHADER, A::SHADER_READ, L::GENERAL),
    // 0x16 ColorAttachmentRead
    spec(
        0x16,
        S::COLOR_ATTACHMENT_OUTPUT,
        A::COLOR_ATTACHMENT_READ,
        L::COLOR_ATTACHMENT_OPTIMAL,
    ),
    // 0x17 DepthStencilAttachmentRead
    spec(
        0x17,
        stages(&[S::EARLY_FRAGMENT_TESTS, S::LATE_FRAGMENT_TESTS]),
        A::DEPTH_STENCIL_ATTACHMENT_READ,
        L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    ),
    // 0x18 ComputeShaderReadUniformBuffer
    spec(0x18, S::COMPUTE_SHADER, A::UNIFORM_READ, L::UNDEFINED),
    // 0x19 ComputeShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x19,
        S::COMPUTE_SHADER,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x1a ComputeShaderReadOther
    spec(0x1a, S::COMPUTE_SHADER, A::SHADER_READ, L::GENERAL),
    // 0x1b (unnamed read)
    spec(
        0x1b,
        stages(&[
            S::VERTEX_SHADER,
            S::TESSELLATION_CONTROL_SHADER,
            S::TESSELLATION_EVALUATION_SHADER,
            S::GEOMETRY_SHADER,
            S::FRAGMENT_SHADER,
        ]),
        A::UNIFORM_READ,
        L::UNDEFINED,
    ),
    // 0x1c (unnamed read)
    spec(
        0x1c,
        stages(&[
            S::VERTEX_SHADER,
            S::TESSELLATION_CONTROL_SHADER,
            S::TESSELLATION_EVALUATION_SHADER,
            S::GEOMETRY_SHADER,
            S::FRAGMENT_SHADER,
        ]),
        access(&[A::VERTEX_ATTRIBUTE_READ, A::UNIFORM_READ]),
        L::UNDEFINED,
    ),
    // 0x1d (unnamed read)
    spec(
        0x1d,
        stages(&[
            S::VERTEX_SHADER,
            S::TESSELLATION_CONTROL_SHADER,
            S::TESSELLATION_EVALUATION_SHADER,
            S::GEOMETRY_SHADER,
            S::FRAGMENT_SHADER,
        ]),
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x1e (unnamed read)
    spec(
        0x1e,
        stages(&[
            S::VERTEX_SHADER,
            S::TESSELLATION_CONTROL_SHADER,
            S::TESSELLATION_EVALUATION_SHADER,
            S::GEOMETRY_SHADER,
            S::FRAGMENT_SHADER,
            S::EARLY_FRAGMENT_TESTS,
            S::LATE_FRAGMENT_TESTS,
        ]),
        access(&[A::SHADER_READ, A::DEPTH_STENCIL_ATTACHMENT_READ]),
        L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    ),
    // 0x1f (unnamed read)
    spec(
        0x1f,
        stages(&[
            S::VERTEX_SHADER,
            S::TESSELLATION_CONTROL_SHADER,
            S::TESSELLATION_EVALUATION_SHADER,
            S::GEOMETRY_SHADER,
            S::FRAGMENT_SHADER,
            S::EARLY_FRAGMENT_TESTS,
            S::LATE_FRAGMENT_TESTS,
        ]),
        A::SHADER_READ,
        L::GENERAL,
    ),
    // 0x20 AnyShaderReadUniformBuffer
    spec(0x20, S::ALL_COMMANDS, A::UNIFORM_READ, L::UNDEFINED),
    // 0x21 AnyShaderReadUniformBufferOrVertexBuffer
    spec(
        0x21,
        S::ALL_COMMANDS,
        access(&[A::VERTEX_ATTRIBUTE_READ, A::UNIFORM_READ]),
        L::UNDEFINED,
    ),
    // 0x22 AnyShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x22,
        S::ALL_COMMANDS,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x23 (unnamed read)
    spec(
        0x23,
        S::ALL_COMMANDS,
        access(&[A::SHADER_READ, A::DEPTH_STENCIL_ATTACHMENT_READ]),
        L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    ),
    // 0x24 AnyShaderReadOther
    spec(0x24, S::ALL_COMMANDS, A::SHADER_READ, L::GENERAL),
    // 0x25 TransferRead
    spec(0x25, S::TRANSFER, A::TRANSFER_READ, L::TRANSFER_SRC_OPTIMAL),
    // 0x26 HostRead
    spec(0x26, S::HOST, A::HOST_READ, L::GENERAL),
    // 0x27 Present
    spec(0x27, S::empty(), A::empty(), L::PRESENT_SRC_KHR),
    // 0x28 CommandBufferWriteNVX
    spec(
        0x28,
        S::COMMAND_PREPROCESS_NV,
        A::COMMAND_PREPROCESS_WRITE_NV,
        L::UNDEFINED,
    ),
    // 0x29 VertexShaderWrite
    spec(0x29, S::VERTEX_SHADER, A::SHADER_WRITE, L::GENERAL),
    // 0x2a TessellationControlShaderWrite
    spec(
        0x2a,
        S::TESSELLATION_CONTROL_SHADER,
        A::SHADER_WRITE,
        L::GENERAL,
    ),
    // 0x2b TessellationEvaluationShaderWrite
    spec(
        0x2b,
        S::TESSELLATION_EVALUATION_SHADER,
        A::SHADER_WRITE,
        L::GENERAL,
    ),
    // 0x2c GeometryShaderWrite
    spec(0x2c, S::GEOMETRY_SHADER, A::SHADER_WRITE, L::GENERAL),
    // 0x2d FragmentShaderWrite
    spec(0x2d, S::FRAGMENT_SHADER, A::SHADER_WRITE, L::GENERAL),
    // 0x2e ColorAttachmentWrite
    spec(
        0x2e,
        S::COLOR_ATTACHMENT_OUTPUT,
        A::COLOR_ATTACHMENT_WRITE,
        L::COLOR_ATTACHMENT_OPTIMAL,
    ),
    // 0x2f DepthStencilAttachmentWrite
    spec(
        0x2f,
        stages(&[S::EARLY_FRAGMENT_TESTS, S::LATE_FRAGMENT_TESTS]),
        access(&[
            A::DEPTH_STENCIL_ATTACHMENT_READ,
            A::DEPTH_STENCIL_ATTACHMENT_WRITE,
        ]),
        L::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    ),
    // 0x30 DepthAttachmentWriteStencilReadOnly
    spec(
        0x30,
        stages(&[S::EARLY_FRAGMENT_TESTS, S::LATE_FRAGMENT_TESTS]),
        access(&[
            A::DEPTH_STENCIL_ATTACHMENT_READ,
            A::DEPTH_STENCIL_ATTACHMENT_WRITE,
        ]),
        L::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
    ),
    // 0x31 StencilAttachmentWriteDepthReadOnly
    spec(
        0x31,
        stages(&[S::EARLY_FRAGMENT_TESTS, S::LATE_FRAGMENT_TESTS]),
        access(&[
            A::DEPTH_STENCIL_ATTACHMENT_READ,
            A::DEPTH_STENCIL_ATTACHMENT_WRITE,
        ]),
        L::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
    ),
    // 0x32 ComputeShaderWrite
    spec(0x32, S::COMPUTE_SHADER, A::SHADER_WRITE, L::GENERAL),
    // 0x33 AnyShaderWrite
    spec(0x33, S::ALL_COMMANDS, A::SHADER_WRITE, L::GENERAL),
    // 0x34 TransferWrite
    spec(
        0x34,
        S::TRANSFER,
        A::TRANSFER_WRITE,
        L::TRANSFER_DST_OPTIMAL,
    ),
    // 0x35 HostWrite
    spec(0x35, S::HOST, A::HOST_WRITE, L::GENERAL),
    // 0x36 ColorAttachmentReadWrite
    spec(
        0x36,
        S::COLOR_ATTACHMENT_OUTPUT,
        access(&[A::COLOR_ATTACHMENT_READ, A::COLOR_ATTACHMENT_WRITE]),
        L::COLOR_ATTACHMENT_OPTIMAL,
    ),
    // 0x37 General
    spec(
        0x37,
        S::ALL_COMMANDS,
        access(&[A::MEMORY_READ, A::MEMORY_WRITE]),
        L::GENERAL,
    ),
    // 0x38 RayTracingShaderReadSampledImageOrUniformTexelBuffer
    spec(
        0x38,
        S::RAY_TRACING_SHADER_KHR,
        A::SHADER_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x39 RayTracingShaderReadColorInputAttachment
    spec(
        0x39,
        S::RAY_TRACING_SHADER_KHR,
        A::INPUT_ATTACHMENT_READ,
        L::SHADER_READ_ONLY_OPTIMAL,
    ),
    // 0x3a RayTracingShaderReadDepthStencilInputAttachment
    spec(
        0x3a,
        S::RAY_TRACING_SHADER_KHR,
        A::INPUT_ATTACHMENT_READ,
        L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    ),
    // 0x3b RayTracingShaderReadAccelerationStructure
    spec(
        0x3b,
        S::RAY_TRACING_SHADER_KHR,
        A::ACCELERATION_STRUCTURE_READ_KHR,
        L::UNDEFINED,
    ),
    // 0x3c RayTracingShaderReadOther
    spec(0x3c, S::RAY_TRACING_SHADER_KHR, A::SHADER_READ, L::GENERAL),
    // 0x3d AccelerationStructureBuildWrite
    spec(
        0x3d,
        S::ACCELERATION_STRUCTURE_BUILD_KHR,
        A::ACCELERATION_STRUCTURE_WRITE_KHR,
        L::UNDEFINED,
    ),
    // 0x3e AccelerationStructureBuildRead
    spec(
        0x3e,
        S::ACCELERATION_STRUCTURE_BUILD_KHR,
        A::ACCELERATION_STRUCTURE_READ_KHR,
        L::UNDEFINED,
    ),
    // 0x3f AccelerationStructureBufferWrite
    spec(
        0x3f,
        S::ACCELERATION_STRUCTURE_BUILD_KHR,
        A::TRANSFER_WRITE,
        L::UNDEFINED,
    ),
];

pub fn access_info(code: u8) -> Option<AccessSpec> {
    ACCESS_TABLE.get(usize::from(code)).copied()
}

/// Write codes are exactly `0x28..=0x37`.
pub const fn is_write(code: u8) -> bool {
    code.wrapping_sub(0x28) < 0x10
}
