//! Pipeline-stage and access-scope coverage predicates.
use ash::vk;
use crate::native::AccessScope;

pub(super) fn stages_cover(provided: vk::PipelineStageFlags, required: vk::PipelineStageFlags) -> bool {
    let mut remaining = required;
    if provided.contains(vk::PipelineStageFlags::ALL_COMMANDS) {
        remaining &= vk::PipelineStageFlags::HOST;
    }
    if provided.contains(vk::PipelineStageFlags::ALL_GRAPHICS) {
        remaining &= !(vk::PipelineStageFlags::DRAW_INDIRECT
            | vk::PipelineStageFlags::VERTEX_INPUT
            | vk::PipelineStageFlags::VERTEX_SHADER
            | vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
            | vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
            | vk::PipelineStageFlags::GEOMETRY_SHADER
            | vk::PipelineStageFlags::FRAGMENT_SHADER
            | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS
            | vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);
    }
    provided.contains(remaining)
}

pub(super) fn wait_stages_cover(provided: vk::PipelineStageFlags, required: vk::PipelineStageFlags) -> bool {
    let device = required & !vk::PipelineStageFlags::HOST;
    stages_cover(provided, if device.is_empty() { vk::PipelineStageFlags::ALL_COMMANDS } else { device })
}

pub(super) fn scope_covers(provided: AccessScope, required: AccessScope) -> bool {
    let mut access = required.access;
    if provided.access.contains(vk::AccessFlags::MEMORY_READ) {
        access &= !(vk::AccessFlags::INDIRECT_COMMAND_READ
            | vk::AccessFlags::INDEX_READ
            | vk::AccessFlags::VERTEX_ATTRIBUTE_READ
            | vk::AccessFlags::UNIFORM_READ
            | vk::AccessFlags::INPUT_ATTACHMENT_READ
            | vk::AccessFlags::SHADER_READ
            | vk::AccessFlags::COLOR_ATTACHMENT_READ
            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
            | vk::AccessFlags::TRANSFER_READ
            | vk::AccessFlags::HOST_READ);
    }
    if provided.access.contains(vk::AccessFlags::MEMORY_WRITE) {
        access &= !(vk::AccessFlags::SHADER_WRITE
            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE
            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
            | vk::AccessFlags::TRANSFER_WRITE
            | vk::AccessFlags::HOST_WRITE);
    }
    stages_cover(provided.stages, required.stages) && provided.access.contains(access)
}

pub(super) fn union(a: AccessScope, b: AccessScope) -> AccessScope {
    AccessScope {
        stages: a.stages | b.stages,
        access: a.access | b.access,
        write: a.write || b.write,
    }
}
