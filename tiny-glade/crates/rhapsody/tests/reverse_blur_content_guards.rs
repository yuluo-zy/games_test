//! Additional real-Graph guards for native upgrade content provenance.
//! This is an independent candidate; root owns compilation and execution.
use ash::vk;
use rhapsody::rg::{
    AccessRange, AccessTable, GraphDeclaration, GraphDefinition, OriginSource, PlanContext,
    RenderGraph, ResourceOrigin, ResourceSpec, compile_definition,
};

const READ: u8 = 0x19;
const GENERAL: u8 = 0x37;

fn mip(base: u32, count: u32) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip_level: base,
        level_count: count,
        base_array_layer: 0,
        layer_count: 1,
    }
}

fn definition(raw: &GraphDeclaration) -> rhapsody::Result<GraphDefinition> {
    GraphDefinition::from_declaration(raw, &AccessTable::original(), PlanContext::default())
}

fn alias_fixture(
    initialize_top: bool,
    strict_range: AccessRange,
    strict_first: bool,
) -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let spec = ResourceSpec::Image(
        rhapsody::resource::TextureDesc::new_2d(16, 16, vk::Format::B10G11R11_UFLOAT_PACK32)
            .mip_levels(2),
    );
    let full = graph
        .create_raw_resource(
            "full",
            ResourceOrigin {
                descriptor: spec.descriptor(),
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut producer = graph.add_pass("genuine earlier initialized range");
    producer
        .add_resource_access_range(
            full,
            GENERAL,
            0,
            if initialize_top { mip(0, 2) } else { mip(0, 1) },
        )
        .unwrap();
    producer.render(|_| true);
    let mut pass = graph.add_pass("top original alias").with_side_effect();
    if strict_first {
        pass.add_preserving_resource_access_range(full, GENERAL, 0, strict_range)
            .unwrap();
    }
    pass.add_undefined_content_read_range(full, READ, 1, mip(1, 1))
        .unwrap();
    pass.add_resource_access_range(full, GENERAL, 0, mip(1, 1))
        .unwrap();
    if !strict_first {
        pass.add_preserving_resource_access_range(full, GENERAL, 0, strict_range)
            .unwrap();
    }
    pass.render(|_| true);
    graph.build().declaration
}

fn assert_uninitialized(raw: &GraphDeclaration) {
    let error = definition(raw).expect_err("strict old content must be checked at entry");
    assert!(
        format!("{error:?}").contains("UninitializedRead"),
        "must fail for missing old content, not an unrelated native layout or mode error: {error:?}"
    );
}

#[test]
fn partial_overlap_preserving_alias_uses_entry_content_in_either_order() {
    for strict_first in [true, false] {
        let raw = alias_fixture(false, mip(0, 2), strict_first);
        let main = &raw.passes[1];
        assert_eq!(main.accesses.len(), 3);
        assert!(main.accesses.iter().all(|a| a.access_code == GENERAL));
        assert_uninitialized(&raw);
    }
}

#[test]
fn post_upgrade_preserving_same_range_tightens_the_saved_output() {
    // Appending code37/group0 is native-compatible dedup, unlike code19 after
    // the native upgrade. An earlier synthetic write cannot satisfy this read.
    let raw = alias_fixture(false, mip(1, 1), false);
    assert_eq!(raw.passes[1].accesses.len(), 2);
    assert_uninitialized(&raw);
}

#[test]
fn initialized_partial_overlap_control_is_native_compatible_in_either_order() {
    for strict_first in [true, false] {
        let plan =
            compile_definition(definition(&alias_fixture(true, mip(0, 2), strict_first)).unwrap())
                .unwrap();
        assert_eq!(plan.retained_passes, [0, 1]);
        assert!(plan.range_program.uses.iter().all(|usage| {
            usage.layout == vk::ImageLayout::GENERAL && usage.access_code == GENERAL
        }));
    }
}

#[test]
fn initialized_disjoint_strict_range_does_not_expand_top_permission() {
    // Top has no earlier writer. The disjoint strict mip0 does have one.
    let plan =
        compile_definition(definition(&alias_fixture(false, mip(0, 1), false)).unwrap()).unwrap();
    assert_eq!(plan.retained_passes, [0, 1]);
}

#[test]
fn allowed_real_read_retains_a_genuine_earlier_top_producer() {
    let mut graph = RenderGraph::new();
    let full = graph
        .create_raw_resource(
            "full",
            ResourceOrigin {
                descriptor: ResourceSpec::Image(
                    rhapsody::resource::TextureDesc::new_2d(
                        16,
                        16,
                        vk::Format::B10G11R11_UFLOAT_PACK32,
                    )
                    .mip_levels(2),
                )
                .descriptor(),
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut producer = graph.add_pass("real top writer");
    producer
        .add_resource_access_range(full, GENERAL, 0, mip(1, 1))
        .unwrap();
    producer.render(|_| true);
    let mut pass = graph
        .add_pass("allowed top read and overwrite")
        .with_side_effect();
    pass.add_undefined_content_read_range(full, READ, 1, mip(1, 1))
        .unwrap();
    pass.add_resource_access_range(full, GENERAL, 0, mip(1, 1))
        .unwrap();
    pass.render(|_| true);
    let plan = compile_definition(definition(&graph.build().declaration).unwrap()).unwrap();
    assert_eq!(plan.retained_passes, [0, 1]);
    assert!(plan.dependency_edges.contains(&(0, 1)));
}

#[test]
fn ordinary_nonwrite_with_previous_false_still_requires_defined_contents() {
    let mut graph = RenderGraph::new();
    let full = graph
        .create_raw_resource(
            "fresh",
            ResourceOrigin {
                descriptor: ResourceSpec::Image(rhapsody::resource::TextureDesc::new_2d(
                    16,
                    16,
                    vk::Format::R32_SFLOAT,
                ))
                .descriptor(),
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut pass = graph.add_pass("ordinary read").with_side_effect();
    pass.add_resource_access_range(full, READ, 1, mip(0, 1))
        .unwrap();
    pass.render(|_| true);
    let raw = graph.build().declaration;
    assert!(!raw.passes[0].accesses[0].reads_previous);
    assert_uninitialized(&raw);
}

#[test]
fn real_top_output_initializes_only_its_own_range_for_later_read_and_export() {
    for export in [false, true] {
        for later_mip in [0, 1] {
            let mut graph = RenderGraph::new();
            let full = graph
                .create_raw_resource(
                    "full",
                    ResourceOrigin {
                        descriptor: ResourceSpec::Image(
                            rhapsody::resource::TextureDesc::new_2d(16, 16, vk::Format::R32_SFLOAT)
                                .mip_levels(2),
                        )
                        .descriptor(),
                        source: OriginSource::Create,
                    },
                )
                .unwrap();
            let mut pass = graph.add_pass("real top output");
            pass.add_undefined_content_read_range(full, READ, 1, mip(1, 1))
                .unwrap();
            pass.add_resource_access_range(full, GENERAL, 0, mip(1, 1))
                .unwrap();
            pass.render(|_| true);
            if export {
                graph.export_range(full, READ, mip(later_mip, 1)).unwrap();
            } else {
                let mut pass = graph.add_pass("strict later read").with_side_effect();
                pass.add_resource_access_range(full, READ, 1, mip(later_mip, 1))
                    .unwrap();
                pass.render(|_| true);
            }
            let raw = graph.build().declaration;
            if later_mip == 1 {
                compile_definition(definition(&raw).unwrap()).unwrap();
            } else {
                assert_uninitialized(&raw);
            }
        }
    }
}
