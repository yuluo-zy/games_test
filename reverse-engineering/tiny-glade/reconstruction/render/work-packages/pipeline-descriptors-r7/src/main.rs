use ash::vk::{self, Handle};
use serde::Deserialize;
use std::io::{self, BufRead};
use tiny_glade_pipeline_descriptors_r7::{
    descriptor_set_layout, Binding, LayoutBinding, LayoutPlan,
};

#[derive(Deserialize)]
struct Input {
    bindings: Vec<Binding>,
    first: u64,
    stage: u32,
    limit: u32,
}

fn main() {
    for line in io::stdin().lock().lines() {
        let input: Input = serde_json::from_str(&line.unwrap()).unwrap();
        let result = descriptor_set_layout(
            &input.bindings,
            input.first,
            vk::ShaderStageFlags::from_raw(input.stage),
            input.limit,
        )
        .map(|plan| {
            plan.with_create_info(|info| unsafe {
                assert_eq!(
                    info.s_type,
                    vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO
                );
                let extension =
                    &*(info.p_next as *const vk::DescriptorSetLayoutBindingFlagsCreateInfo);
                assert_eq!(extension.binding_count, info.binding_count);
                LayoutPlan {
                    flags: info.flags.as_raw(),
                    binding_flags: (0..extension.binding_count as usize)
                        .map(|i| (*extension.p_binding_flags.add(i)).as_raw())
                        .collect(),
                    bindings: (0..info.binding_count as usize)
                        .map(|i| {
                            let b = &*info.p_bindings.add(i);
                            LayoutBinding {
                                binding: b.binding,
                                descriptor_type: b.descriptor_type.as_raw(),
                                descriptor_count: b.descriptor_count,
                                stage_flags: b.stage_flags.as_raw(),
                                immutable_sampler: if b.p_immutable_samplers.is_null() {
                                    None
                                } else {
                                    Some((*b.p_immutable_samplers).as_raw())
                                },
                            }
                        })
                        .collect(),
                }
            })
        });
        println!("{}", serde_json::to_string(&result).unwrap());
    }
}
