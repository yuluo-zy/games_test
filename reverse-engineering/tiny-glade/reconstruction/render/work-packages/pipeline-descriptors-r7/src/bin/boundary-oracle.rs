use ash::vk::{self, Handle};
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead};
use tiny_glade_pipeline_descriptors_r7::{
    persistent_pool_plan, with_compute_pipeline_layout, LayoutPlan,
};

#[derive(Deserialize)]
struct Input {
    op: String,
    #[serde(default)]
    layouts: Vec<u64>,
    #[serde(default)]
    push: u32,
    #[serde(default)]
    limit: u32,
    layout: Option<LayoutPlan>,
}
fn main() {
    for line in io::stdin().lock().lines() {
        let input: Input = serde_json::from_str(&line.unwrap()).unwrap();
        let output: Value = if input.op == "compute" {
            let layouts: Vec<_> = input
                .layouts
                .iter()
                .map(|&x| vk::DescriptorSetLayout::from_raw(x))
                .collect();
            with_compute_pipeline_layout(&layouts, input.push, |info| unsafe {
                let ranges: Vec<_> = (0..info.push_constant_range_count as usize)
                    .map(|i| {
                        let range = &*info.p_push_constant_ranges.add(i);
                        json!([range.stage_flags.as_raw(), range.offset, range.size])
                    })
                    .collect();
                let layouts: Vec<_> = (0..info.set_layout_count as usize)
                    .map(|i| (*info.p_set_layouts.add(i)).as_raw())
                    .collect();
                json!({"s_type":info.s_type.as_raw(),"flags":info.flags.as_raw(),"layouts":layouts,"ranges":ranges})
            })
        } else {
            let plan = persistent_pool_plan(input.layout.as_ref().unwrap(), input.limit);
            let mut event = plan.with_pool_create_info(|info| unsafe {
                assert_eq!(info.s_type, vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO);
                let sizes: Vec<_> = (0..info.pool_size_count as usize)
                    .map(|i| {
                        let size = &*info.p_pool_sizes.add(i);
                        json!([size.ty.as_raw(), size.descriptor_count])
                    })
                    .collect();
                json!({"flags":info.flags.as_raw(),"max_sets":info.max_sets,"sizes":sizes})
            });
            plan.with_allocate_info(
                vk::DescriptorPool::from_raw(0x1234),
                vk::DescriptorSetLayout::from_raw(0xabcdef),
                |info| unsafe {
                    assert_eq!(info.descriptor_set_count, 1);
                    assert_eq!((*info.p_set_layouts).as_raw(), 0xabcdef);
                    assert_eq!(info.descriptor_pool.as_raw(), 0x1234);
                    let ext = &*(info.p_next
                        as *const vk::DescriptorSetVariableDescriptorCountAllocateInfo);
                    assert_eq!(ext.descriptor_set_count, 1);
                    event["variable_descriptor_count"] = json!(*ext.p_descriptor_counts);
                },
            );
            event
        };
        println!("{}", output);
    }
}
