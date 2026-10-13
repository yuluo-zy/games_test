//! 原发行资产经共享codec进入真实运行时布局消费者。
//! catalog仅作为独立测试oracle；运行时生产加载不读取其偏移或metadata。
use ash::vk;
use rhapsody::raw::shader::{ReflectedEntry, ShaderInterfacePlan, compiled_pipeline_entries, stored_pipeline_entry};
use rhapsody::shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
use std::collections::BTreeMap;

fn original_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../ljxsj_92385/Tiny Glade/compiled-shaders")
}

#[test]
fn original_containers_feed_all_runtime_interfaces_without_catalog_offsets() {
    let catalog: serde_json::Value = serde_json::from_str(include_str!(
        "../../../evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.json"
    )).unwrap();
    let mut counts = [0usize; 3];
    for file in catalog["files"].as_array().unwrap() {
        let bytes = std::fs::read(original_dir().join(file["file"].as_str().unwrap())).unwrap();
        let compiled = CompiledShaderPipeline::decode(&bytes).unwrap();
        let entries = compiled_pipeline_entries(&compiled).unwrap();
        let expected = file["modules"].as_array().unwrap().iter()
            .flat_map(|module| module["entries"].as_array().unwrap())
            .map(|entry| serde_json::from_value::<ReflectedEntry>(entry.clone()).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(entries.len(), expected.len(), "{}", file["file"]);
        for (actual, expected) in entries.iter().zip(expected) {
            assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(expected).unwrap(), "{}", file["file"]);
        }
        let samplers = entries.iter().flat_map(|entry| &entry.descriptors)
            .filter(|descriptor| descriptor.descriptor_type == "Sampler")
            .map(|descriptor| (descriptor.name.clone(), 1)).collect::<BTreeMap<_,_>>();
        let plan = ShaderInterfacePlan::from_compiled_pipeline(&compiled, &samplers, 4096).unwrap();
        let public_plan = ShaderInterfacePlan::from_reflection(&entries, &samplers, 4096).unwrap();
        // stored schema与公开重反射必须独立比较，不只验证fallback输出。
        assert_eq!(plan.sets.len(), public_plan.sets.len(), "{}", file["file"]);
        for (stored, public) in plan.sets.iter().zip(&public_plan.sets) {
            assert_eq!(stored.flags, public.flags);
            assert_eq!(stored.binding_flags, public.binding_flags);
            for (stored, public) in stored.bindings.iter().zip(&public.bindings) {
                assert_eq!((stored.binding, stored.descriptor_type, stored.descriptor_count, stored.immutable_sampler), (public.binding, public.descriptor_type, public.descriptor_count, public.immutable_sampler));
            }
            assert_eq!(stored.bindings.len(), public.bindings.len());
        }
        assert_eq!(compiled.reflection.push_constant_size, public_plan.push_constants.iter().map(|range| range.size).max().unwrap_or(0), "{}", file["file"]);
        let flags = if entries[0].stage == "GLCompute" {
            counts[0] += 1;
            vk::ShaderStageFlags::COMPUTE
        } else {
            counts[1] += 1;
            vk::ShaderStageFlags::ALL_GRAPHICS
        };
        // scalar4096大于原保留32，4064是合法runtime数组容量，非零尺寸猜测。
        assert!(plan.sets.iter().flat_map(|set| &set.bindings).all(|binding| binding.stage_flags == flags.as_raw()));
        assert!(plan.push_constants.iter().all(|range| range.stage_flags == flags));
        for entry in &entries {
            for descriptor in &entry.descriptors {
                let binding = plan.sets[descriptor.set as usize].bindings.iter()
                    .find(|binding| binding.binding == descriptor.binding).unwrap();
                assert_eq!(binding.descriptor_count, if descriptor.count == 0 {4064} else {descriptor.count});
            }
        }
        counts[2] += entries.len();
    }
    assert_eq!(counts, [170, 234, 638]);
}

#[test]
fn serialized_stage_and_entry_must_agree_with_payload_before_vulkan() {
    let bytes = std::fs::read(original_dir().join("_fill_buffer.cs.hlsl_cs_main_0.bin")).unwrap();
    let mut compiled = CompiledShaderPipeline::decode(&bytes).unwrap();
    assert_eq!(compiled_pipeline_entries(&compiled).unwrap()[0].entry_point, "cs_main");
    compiled.stages[0].stage = ShaderStage::Vertex;
    assert!(compiled_pipeline_entries(&compiled).is_err());
    assert!(stored_pipeline_entry(&compiled, 0).is_err());
    assert!(ShaderInterfacePlan::from_compiled_pipeline(&compiled, &BTreeMap::new(), 4096).is_err());
    compiled.stages[0].stage = ShaderStage::Compute;
    compiled.stages[0].entry_point = "not_an_entry".into();
    assert!(compiled_pipeline_entries(&compiled).is_err());
    assert!(stored_pipeline_entry(&compiled, 0).is_err());
    assert!(ShaderInterfacePlan::from_compiled_pipeline(&compiled, &BTreeMap::new(), 4096).is_err());
}

#[test]
fn stored_metadata_drives_layout_instead_of_recomputed_spirv() {
    let bytes = std::fs::read(original_dir().join("_fill_buffer.cs.hlsl_cs_main_0.bin")).unwrap();
    let mut compiled = CompiledShaderPipeline::decode(&bytes).unwrap();
    let public = compiled_pipeline_entries(&compiled).unwrap();
    assert_eq!(public[0].descriptors[0].binding, 13);
    assert_eq!(public[0].descriptors[0].name, "__alias_output");
    assert_eq!(stored_pipeline_entry(&compiled, 0).unwrap().descriptors[0].name, "output");
    let (binding, stored) = &mut compiled.reflection.descriptor_sets[0][0];
    *binding = 7;
    stored.name = "changed_owner_name".into();
    compiled.reflection.push_constant_size = 16;
    let plan = ShaderInterfacePlan::from_compiled_pipeline(&compiled, &BTreeMap::new(), 4096).unwrap();
    assert_eq!(plan.sets[0].bindings[0].binding, 7);
    assert_eq!(plan.push_constants[0].size, 16);
    let entry = stored_pipeline_entry(&compiled, 0).unwrap();
    assert_eq!(entry.descriptors[0].binding, 7);
    assert_eq!(entry.descriptors[0].name, "changed_owner_name");
    assert_eq!(compiled_pipeline_entries(&compiled).unwrap()[0].descriptors[0].binding, 13);
}
