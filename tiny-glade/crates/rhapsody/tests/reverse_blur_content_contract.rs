//! 原 reverse blur 的顶层 input/output 同 mip：真实 Graph CPU 反例。
//! 当前基线应有一个红例；本文件不执行 shader，也不证明 GPU/原整帧通过。
use ash::vk;
use rhapsody::rg::{
    AccessRange, AccessTable, GraphDeclaration, GraphDefinition, OriginSource, PlanContext,
    RenderGraph, ResourceOrigin, ResourceSpec, compile_definition,
};

const READ: u8 = 0x19;
const GENERAL: u8 = 0x37;

fn top_mip() -> AccessRange {
    AccessRange::Texture {
        aspect_mask: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip_level: 1,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    }
}

fn fixture(
    initialized: bool,
    ordinary_alias: Option<bool>,
    preserving_write: bool,
) -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let descriptor = ResourceSpec::Image(
        rhapsody::resource::TextureDesc::new_2d(16, 16, vk::Format::B10G11R11_UFLOAT_PACK32)
            .mip_levels(2),
    )
    .descriptor();
    // 已有 partial 的真实上游内容在本有界 fixture 中作为显式边界输入。
    let partial = graph
        .import_unowned("partial producer boundary", descriptor, 1, READ)
        .unwrap();
    let full = graph
        .create_raw_resource(
            "full blur pyramid",
            ResourceOrigin {
                descriptor,
                source: if initialized {
                    OriginSource::ImportUnowned {
                        token: 2,
                        initial_access: READ,
                    }
                } else {
                    OriginSource::Create
                },
            },
        )
        .unwrap();
    let mut pass = graph
        .add_pass("rev_blurN sample_self=false")
        .with_side_effect();
    pass.add_resource_access_range(partial, READ, 1, top_mip())
        .unwrap();
    if ordinary_alias == Some(true) {
        pass.add_resource_access_range(full, READ, 1, top_mip())
            .unwrap();
    }
    // 对应原 SPV 的九次真实 fetch；false Phi 不消费该值，不是 inactive。
    pass.add_undefined_content_read_range(full, READ, 1, top_mip())
        .unwrap();
    if ordinary_alias == Some(false) {
        pass.add_resource_access_range(full, READ, 1, top_mip())
            .unwrap();
    }
    if preserving_write {
        pass.add_preserving_resource_access_range(full, GENERAL, 0, top_mip())
            .unwrap();
    } else {
        pass.add_resource_access_range(full, GENERAL, 0, top_mip())
            .unwrap();
    }
    pass.render(|_| true);
    graph.build().declaration
}

fn definition(raw: &GraphDeclaration) -> rhapsody::Result<GraphDefinition> {
    GraphDefinition::from_declaration(raw, &AccessTable::original(), PlanContext::default())
}

#[test]
fn reverse_top_original_read_permission_must_survive_native_general_upgrade() {
    let raw = fixture(false, None, false);
    let upgraded: Vec<_> = raw.passes[0]
        .accesses
        .iter()
        .filter(|access| access.resource_index == 1)
        .collect();
    assert_eq!(upgraded.len(), 2, "原 native 升级的两条记录必须保留");
    assert!(upgraded.iter().all(|access| access.access_code == GENERAL
        && access.synchronization_group == 0
        && access.range == top_mip()));
    eprintln!("real graph upgraded entries: {upgraded:?}");
    // 当前基线在这里报 UninitializedRead；不得用清零、inactive 或假 producer 通过。
    let definition = definition(&raw).expect(
        "confirmed sample_self=false: retain native read/write sync but preserve the original explicit undefined-read content contract",
    );
    let plan = compile_definition(definition).unwrap();
    assert_eq!(plan.retained_passes, [0]);
}

#[test]
fn ordinary_strict_alias_in_either_order_still_requires_defined_old_contents() {
    for first in [false, true] {
        assert!(definition(&fixture(false, Some(first), false)).is_err());
    }
}

#[test]
fn actual_preserving_write_still_requires_defined_old_contents() {
    assert!(definition(&fixture(false, None, true)).is_err());
}

#[test]
fn initialized_control_keeps_two_native_general_accesses_and_compiles() {
    let key = definition(&fixture(true, None, false)).unwrap();
    let upgraded: Vec<_> = key.passes[0]
        .accesses
        .iter()
        .filter(|access| access.resource_index == 1)
        .collect();
    assert_eq!(upgraded.len(), 2);
    assert!(upgraded.iter().all(|access| access.access_code == GENERAL
        && access.synchronization_group == 0
        && access.range == top_mip()));
    let plan = compile_definition(key).unwrap();
    assert_eq!(plan.retained_passes, [0]);
}
