use super::*;
use crate::rg::executor::pass_context::image_layout_from_plan;
use crate::rg::{
    AccessRange, AccessTable, GraphDefinition, OriginSource, PlanContext, RenderGraph,
    ResourceOrigin, ResourceSpec, compile_definition,
};
fn mips(base: u32, count: u32) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: 1,
        base_mip_level: base,
        level_count: count,
        base_array_layer: 0,
        layer_count: 1,
    }
}
fn plan(alias: bool) -> super::super::plan::CompiledPlan {
    let mut graph = RenderGraph::new();
    let descriptor = ResourceSpec::Image(
        crate::resource::TextureDesc::new_2d(16, 16, vk::Format::R32_SFLOAT).mip_levels(3),
    )
    .descriptor();
    let image = graph.import_unowned("image", descriptor, 1, 0x19).unwrap();
    graph
        .import_unowned(
            "wrong kind",
            super::super::graph::Descriptor::Buffer(crate::rg::resource_registry::BufferDesc {
                word_at_00: 16,
                word_at_08: 0,
                bytes_at_0c_through_0e: [0; 3],
            }),
            2,
            0x1a,
        )
        .unwrap();
    let scratch = graph
        .create_raw_resource(
            "culled scratch",
            ResourceOrigin {
                descriptor,
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut culled = graph.add_pass("culled original index0");
    culled
        .add_resource_access_range(scratch, 0x37, 0, mips(0, 1))
        .unwrap();
    culled.render(|_| true);
    let mut pass = graph
        .add_pass("scheduled index0 original index1")
        .with_side_effect();
    pass.add_resource_access_range(image, 0x19, 1, mips(0, 1))
        .unwrap();
    pass.add_resource_access_range(image, 0x19, 1, mips(1, 1))
        .unwrap();
    if alias {
        pass.add_resource_access_range(image, 0x37, 0, mips(1, 1))
            .unwrap();
    }
    pass.render(|_| true);
    let definition = GraphDefinition::from_declaration(
        &graph.build().declaration,
        &AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    compile_definition(definition).unwrap()
}
#[test]
fn exact_views_use_compiled_scheduled_layout_after_culling() {
    let plan = plan(true);
    assert_eq!(plan.retained_passes, [1]);
    assert_eq!(
        image_layout_from_plan(&plan, 0, 0, mips(0, 1)).unwrap(),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    );
    assert_eq!(
        image_layout_from_plan(&plan, 0, 0, mips(1, 1)).unwrap(),
        vk::ImageLayout::GENERAL
    );
    assert!(image_layout_from_plan(&plan, 0, 0, mips(0, 2)).is_err());
    assert!(image_layout_from_plan(&plan, 1, 0, mips(1, 1)).is_err());
}
#[test]
fn union_coverage_is_complete_and_wrong_or_missing_views_fail() {
    let plan = plan(false);
    assert_eq!(
        image_layout_from_plan(&plan, 0, 0, mips(0, 2)).unwrap(),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    );
    for range in [
        mips(0, 3),
        mips(2, 1),
        mips(3, 1),
        mips(0, 0),
        AccessRange::Whole,
    ] {
        assert!(image_layout_from_plan(&plan, 0, 0, range).is_err());
    }
    assert!(image_layout_from_plan(&plan, 0, 1, mips(0, 1)).is_err());
    assert!(image_layout_from_plan(&plan, 0, 99, mips(0, 1)).is_err());
    assert!(image_layout_from_plan(&plan, 0, 2, mips(0, 1)).is_err());
}
#[test]
fn inconsistent_scheduled_projection_is_rejected() {
    let mut plan = plan(true);
    plan.range_program.passes[0].pass = 1;
    assert!(image_layout_from_plan(&plan, 0, 0, mips(1, 1)).is_err());
    plan.range_program.passes[0].pass = 0;
    plan.range_program.passes[0].uses.end = usize::MAX;
    assert!(image_layout_from_plan(&plan, 0, 0, mips(1, 1)).is_err());
}
