use super::*;
use crate::rg::graph;
use ash::vk;
use crate::native::AccessSpec;
use crate::device::DeviceCapabilities;
use graph::AccessDependency;

#[test]
fn original_graphics_scope_omits_optional_stages_that_were_not_enabled() {
    let mut table = AccessTable::original();
    let original = table.0[&0x1d];
    let optional = vk::PipelineStageFlags::GEOMETRY_SHADER
        | vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
        | vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER;
    assert!(original.scope.stages.contains(optional));
    table.normalize_enabled_stages(DeviceCapabilities::default());
    let (native, layout) = table.native_semantics(0x1d);
    assert_eq!(native.stages, original.scope.stages & !optional);
    assert!(!native.stages.is_empty());
    assert_eq!(native.access, original.scope.access);
    assert_eq!(layout, original.layout);
    assert_eq!(table.access_mask(0x1d), original.scope.access);
    assert_eq!(
        crate::native::access_table::ACCESS_TABLE[0x1d].scope.stages,
        original.scope.stages
    );
}

#[test]
fn custom_optional_only_scopes_fall_back_and_empty_no_access_stays_empty() {
    let mut table = AccessTable::default();
    for (code, stages, access) in [
        (
            200,
            vk::PipelineStageFlags::GEOMETRY_SHADER,
            vk::AccessFlags::SHADER_READ,
        ),
        (
            201,
            vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                | vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::AccessFlags::SHADER_READ,
        ),
        (
            202,
            vk::PipelineStageFlags::empty(),
            vk::AccessFlags::SHADER_WRITE,
        ),
        (
            203,
            vk::PipelineStageFlags::empty(),
            vk::AccessFlags::empty(),
        ),
    ] {
        table.0.insert(
            code,
            AccessSpec {
                scope: crate::native::AccessScope {
                    stages,
                    access,
                    write: access.contains(vk::AccessFlags::SHADER_WRITE),
                },
                layout: vk::ImageLayout::GENERAL,
            },
        );
    }
    table.normalize_enabled_stages(DeviceCapabilities::default());
    assert_eq!(
        table.native_semantics(200).0.stages,
        vk::PipelineStageFlags::ALL_COMMANDS
    );
    assert_eq!(
        table.native_semantics(201).0.stages,
        vk::PipelineStageFlags::FRAGMENT_SHADER
    );
    assert_eq!(
        table.native_semantics(202).0.stages,
        vk::PipelineStageFlags::ALL_COMMANDS
    );
    assert_eq!(
        table.native_semantics(203).0.stages,
        vk::PipelineStageFlags::empty()
    );
}

#[test]
fn explicitly_enabled_geometry_and_tessellation_scopes_are_preserved() {
    for (geometry_shader, tessellation_shader) in [(true, true), (true, false), (false, true)] {
        let mut table = AccessTable::original();
        let original = table.0[&0x1d].scope.stages;
        table.normalize_enabled_stages(DeviceCapabilities {
            geometry_shader,
            tessellation_shader,
            ..Default::default()
        });
        let native = table.native_semantics(0x1d).0.stages;
        assert_eq!(
            native.contains(vk::PipelineStageFlags::GEOMETRY_SHADER),
            geometry_shader
        );
        assert_eq!(
            native.contains(vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER),
            tessellation_shader
        );
        assert_eq!(
            native.contains(vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER),
            tessellation_shader
        );
        if geometry_shader && tessellation_shader {
            assert_eq!(native, original);
        }
    }
}
