use ash::vk::{self, Handle};
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead};
use tiny_glade_descriptor_binding_r8::*;
use tiny_glade_pipeline_descriptors_r7::{LayoutBinding, LayoutPlan};

#[derive(Deserialize)]
struct Input {
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

unsafe fn capture_write(w: &vk::WriteDescriptorSet<'_>) -> Value {
    let images: Vec<_> = if w.p_image_info.is_null() {
        vec![]
    } else {
        (0..w.descriptor_count as usize).map(|i| { let x=&*w.p_image_info.add(i); json!({"sampler":x.sampler.as_raw(),"image_view":x.image_view.as_raw(),"image_layout":x.image_layout.as_raw()}) }).collect()
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
    assert!(w.p_next.is_null());
    assert!(w.p_texel_buffer_view.is_null());
    json!({"dst_set":w.dst_set.as_raw(),"dst_binding":w.dst_binding,"dst_array_element":w.dst_array_element,"descriptor_count":w.descriptor_count,"descriptor_type":w.descriptor_type.as_raw(),"images":images,"buffers":buffers})
}
fn main() {
    for line in io::stdin().lock().lines() {
        let input: Input = serde_json::from_str(&line.unwrap()).unwrap();
        let output = if input.op == "layout" {
            let result = sparse_descriptor_set_layout(
                &input.entries,
                vk::ShaderStageFlags::from_raw(input.stage),
                input.limit,
            )
            .map(|plan| {
                plan.with_create_info(|info| unsafe {
                    let ext =
                        &*(info.p_next as *const vk::DescriptorSetLayoutBindingFlagsCreateInfo);
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
            });
            serde_json::to_value(result).unwrap()
        } else if input.op == "write" {
            let context = input.context.unwrap();
            let mut offsets = Vec::new();
            let result = descriptor_write(context.descriptor_set, &input.writes[0], &mut offsets)
                .map(|write| {
                    let plan = BindPlan {
                        update_call: true,
                        writes: vec![write],
                        dynamic_offsets: offsets.clone(),
                    };
                    let result = std::cell::RefCell::new(Value::Null);
                    plan.execute(
                        &context,
                        &mut 0,
                        |writes| unsafe {
                            *result.borrow_mut() = capture_write(&writes[0]);
                        },
                        |_, _, _, _, _, _| {},
                    );
                    json!({"write":result.into_inner(),"offsets":offsets})
                });
            serde_json::to_value(result).unwrap()
        } else {
            let context = input.context.unwrap();
            let selected = if input.op == "sparse-bind" {
                compute_pass_bind_plan(&context, &input.tuples, &input.resources)
            } else {
                descriptor_bind_plan(&context, &input.writes)
            };
            let result=selected.map(|plan| {
                let events=std::cell::RefCell::new(Vec::<Value>::new());
                let mut counter=input.counter;
                plan.execute(&context,&mut counter,|writes| unsafe {events.borrow_mut().push(json!({"event":"update","writes":writes.iter().map(|w|capture_write(w)).collect::<Vec<_>>() }));},|command,point,layout,first,set,offsets|{ events.borrow_mut().push(json!({"event":"bind","command_buffer":command.as_raw(),"bind_point":point.as_raw(),"pipeline_layout":layout.as_raw(),"first_set":first,"sets":[set.as_raw()],"offsets":offsets})); });
                json!({"counter":counter,"events":events.into_inner()})
            });
            serde_json::to_value(result).unwrap()
        };
        println!("{}", output);
    }
}
