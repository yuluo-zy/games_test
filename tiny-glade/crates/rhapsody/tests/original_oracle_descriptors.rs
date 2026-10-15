//! 原版机器码 oracle 重放：设备计数器、描述符布局、管线布局/池、绑定与写入。
mod common;
use ash::vk::{self, Handle};
use common::{jstr, replay, text, word};
use rhapsody::raw::descriptor_binding::*;
use rhapsody::raw::descriptor_set::{
    Binding, LayoutBinding, LayoutPlan, descriptor_set_layout, persistent_pool_plan,
    with_compute_pipeline_layout,
};
use rhapsody::raw::device::raw_begin_projection;
use serde::Deserialize;
use serde_json::{Value, json};

fn capture_layout(plan: &LayoutPlan) -> LayoutPlan {
    plan.with_create_info(|info| unsafe {
        assert_eq!(info.s_type, vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO);
        let ext = &*(info.p_next as *const vk::DescriptorSetLayoutBindingFlagsCreateInfo);
        assert_eq!(ext.binding_count, info.binding_count);
        LayoutPlan {
            flags: info.flags.as_raw(),
            binding_flags: (0..ext.binding_count as usize)
                .map(|i| (*ext.p_binding_flags.add(i)).as_raw())
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
}

#[test]
fn device_counters_match_original() {
    replay("device-counters", "parallel-batch-04/device-counters/native-projection-oracle.jsonl.gz", |input| {
        let values: Vec<u64> = text(input).split_whitespace().map(word).collect();
        let mut counters = values[2..].to_vec();
        let mut out = match raw_begin_projection(&mut counters, values[0] as u32, values[1] as u32) {
            Some(sequence) => format!("ok {sequence:x}"),
            None => "bounds".to_string(),
        };
        for counter in counters {
            out.push_str(&format!(" {counter:x}"));
        }
        jstr(out)
    });
}

#[derive(Deserialize)]
struct LayoutInput {
    bindings: Vec<Binding>,
    first: u64,
    stage: u32,
    limit: u32,
}

#[test]
fn pipeline_layout_matches_original() {
    replay("pipeline-layout", "parallel-batch-05/pipeline-descriptors/native-layout-oracle.jsonl.gz", |input| {
        let i: LayoutInput = serde_json::from_value(input.clone()).unwrap();
        let result = descriptor_set_layout(&i.bindings, i.first, vk::ShaderStageFlags::from_raw(i.stage), i.limit)
            .map(|plan| capture_layout(&plan));
        serde_json::to_value(result).unwrap()
    });
}

#[derive(Deserialize)]
struct BoundaryInput {
    op: String,
    #[serde(default)]
    layouts: Vec<u64>,
    #[serde(default)]
    push: u32,
    #[serde(default)]
    limit: u32,
    layout: Option<LayoutPlan>,
}

#[test]
fn pipeline_boundary_matches_original() {
    replay("pipeline-boundary", "parallel-batch-05/pipeline-descriptors/native-boundary-oracle.jsonl.gz", |input| {
        let i: BoundaryInput = serde_json::from_value(input.clone()).unwrap();
        if i.op == "compute" {
            let layouts: Vec<_> = i.layouts.iter().map(|&x| vk::DescriptorSetLayout::from_raw(x)).collect();
            return with_compute_pipeline_layout(&layouts, i.push, |info| unsafe {
                let ranges: Vec<_> = (0..info.push_constant_range_count as usize)
                    .map(|n| {
                        let r = &*info.p_push_constant_ranges.add(n);
                        json!([r.stage_flags.as_raw(), r.offset, r.size])
                    })
                    .collect();
                let layouts: Vec<_> = (0..info.set_layout_count as usize)
                    .map(|n| (*info.p_set_layouts.add(n)).as_raw())
                    .collect();
                json!({"s_type":info.s_type.as_raw(),"flags":info.flags.as_raw(),"layouts":layouts,"ranges":ranges})
            });
        }
        let plan = persistent_pool_plan(i.layout.as_ref().unwrap(), i.limit);
        let mut event = plan.with_pool_create_info(|info| unsafe {
            assert_eq!(info.s_type, vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO);
            let sizes: Vec<_> = (0..info.pool_size_count as usize)
                .map(|n| {
                    let s = &*info.p_pool_sizes.add(n);
                    json!([s.ty.as_raw(), s.descriptor_count])
                })
                .collect();
            json!({"flags":info.flags.as_raw(),"max_sets":info.max_sets,"sizes":sizes})
        });
        plan.with_allocate_info(
            vk::DescriptorPool::from_raw(0x1234),
            vk::DescriptorSetLayout::from_raw(0xabcdef),
            |info| unsafe {
                assert_eq!(info.descriptor_set_count, 1);
                let ext = &*(info.p_next as *const vk::DescriptorSetVariableDescriptorCountAllocateInfo);
                event["variable_descriptor_count"] = json!(*ext.p_descriptor_counts);
            },
        );
        event
    });
}

#[derive(Deserialize)]
struct BindingInput {
    op: String,
    #[serde(default)]
    entries: Vec<SparseBinding>,
    #[serde(default)]
    stage: u32,
    #[serde(default)]
    limit: u32,
    #[serde(default)]
    writes: Vec<BindingWrite>,
    context: Option<BindContext>,
    #[serde(default)]
    counter: u64,
    #[serde(default)]
    tuples: Vec<PassResourceBinding>,
    #[serde(default)]
    resources: Vec<ResourceBinding>,
}

fn capture_write(w: &vk::WriteDescriptorSet<'_>) -> Value {
    unsafe {
        let images: Vec<_> = if w.p_image_info.is_null() {
            vec![]
        } else {
            (0..w.descriptor_count as usize)
                .map(|i| {
                    let x = &*w.p_image_info.add(i);
                    json!({"sampler":x.sampler.as_raw(),"image_view":x.image_view.as_raw(),"image_layout":x.image_layout.as_raw()})
                })
                .collect()
        };
        let buffers: Vec<_> = if w.p_buffer_info.is_null() {
            vec![]
        } else {
            (0..w.descriptor_count as usize)
                .map(|i| {
                    let x = &*w.p_buffer_info.add(i);
                    json!({"buffer":x.buffer.as_raw(),"offset":x.offset,"range":x.range})
                })
                .collect()
        };
        assert_eq!(w.s_type, vk::StructureType::WRITE_DESCRIPTOR_SET);
        assert!(w.p_next.is_null() && w.p_texel_buffer_view.is_null());
        json!({"dst_set":w.dst_set.as_raw(),"dst_binding":w.dst_binding,"dst_array_element":w.dst_array_element,"descriptor_count":w.descriptor_count,"descriptor_type":w.descriptor_type.as_raw(),"images":images,"buffers":buffers})
    }
}

#[test]
fn descriptor_binding_matches_original() {
    replay("descriptor-binding", "parallel-batch-06/descriptor-binding/native-oracle.jsonl.gz", |input| {
        let i: BindingInput = serde_json::from_value(input.clone()).unwrap();
        match i.op.as_str() {
            "layout" => {
                let result = sparse_descriptor_set_layout(&i.entries, vk::ShaderStageFlags::from_raw(i.stage), i.limit)
                    .map(|plan| capture_layout(&plan));
                serde_json::to_value(result).unwrap()
            }
            "write" => {
                let context = i.context.unwrap();
                let mut offsets = Vec::new();
                let result = descriptor_write(context.descriptor_set, &i.writes[0], &mut offsets).map(|write| {
                    let plan = BindPlan { update_call: true, writes: vec![write], dynamic_offsets: offsets.clone() };
                    let captured = std::cell::RefCell::new(Value::Null);
                    plan.execute(&context, &mut 0, |w| *captured.borrow_mut() = capture_write(&w[0]), |_, _, _, _, _, _| {});
                    json!({"write":captured.into_inner(),"offsets":offsets})
                });
                serde_json::to_value(result).unwrap()
            }
            op => {
                let context = i.context.unwrap();
                let selected = if op == "sparse-bind" {
                    compute_pass_bind_plan(&context, &i.tuples, &i.resources)
                } else {
                    descriptor_bind_plan(&context, &i.writes)
                };
                let result = selected.map(|plan| {
                    let events = std::cell::RefCell::new(Vec::<Value>::new());
                    let mut counter = i.counter;
                    plan.execute(
                        &context,
                        &mut counter,
                        |writes| events.borrow_mut().push(json!({"event":"update","writes":writes.iter().map(capture_write).collect::<Vec<_>>()})),
                        |command, point, layout, first, set, offsets| {
                            events.borrow_mut().push(json!({"event":"bind","command_buffer":command.as_raw(),"bind_point":point.as_raw(),"pipeline_layout":layout.as_raw(),"first_set":first,"sets":[set.as_raw()],"offsets":offsets}));
                        },
                    );
                    json!({"counter":counter,"events":events.into_inner()})
                });
                serde_json::to_value(result).unwrap()
            }
        }
    });
}
