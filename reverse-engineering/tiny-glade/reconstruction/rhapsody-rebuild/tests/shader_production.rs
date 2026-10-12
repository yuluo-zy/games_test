//! 按原发行反射目录验收布局生产；CPU 计划通过不代表所有设备都支持全部布局。
use ash::vk;
use rhapsody::raw::shader::{
    InterfaceError, ReflectedEntry, ShaderInterfacePlan, get_vk_blend_mode,
    map_vertex_attribute_format,
};
use std::collections::BTreeMap;

#[test]
fn original_catalog_produces_sparse_merged_layouts() {
    let catalog: serde_json::Value = serde_json::from_str(include_str!("../../../evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.json")).unwrap();
    let mut entries_count = 0;
    for file in catalog["files"].as_array().unwrap() {
        let entries = file["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|module| module["entries"].as_array().unwrap())
            .map(|entry| serde_json::from_value::<ReflectedEntry>(entry.clone()).unwrap())
            .collect::<Vec<_>>();
        let samplers = entries
            .iter()
            .flat_map(|entry| &entry.descriptors)
            .filter(|descriptor| descriptor.descriptor_type == "Sampler")
            .map(|descriptor| (descriptor.name.clone(), 1))
            .collect::<BTreeMap<_, _>>();
        let plan = ShaderInterfacePlan::from_reflection(&entries, &samplers, 4096).unwrap();
        for entry in &entries {
            for descriptor in &entry.descriptors {
                let binding = plan.sets[descriptor.set as usize]
                    .bindings
                    .iter()
                    .find(|binding| binding.binding == descriptor.binding)
                    .unwrap();
                assert_eq!(
                    binding.descriptor_count,
                    if descriptor.count == 0 {
                        4064
                    } else {
                        descriptor.count
                    }
                );
            }
        }
        entries_count += entries.len();
    }
    assert_eq!(entries_count, 638);
}

#[test]
fn reflected_layout_rejects_conflicting_stages_and_missing_sampler() {
    let entry: ReflectedEntry = serde_json::from_value(serde_json::json!({"entry_point":"main","stage":"Vertex","descriptors":[{"set":2,"binding":9,"name":"data","count":1,"descriptor_type":"UniformBuffer"}]})).unwrap();
    let mut other = entry.clone();
    other.stage = "Fragment".into();
    other.descriptors[0].descriptor_type = "StorageBuffer(ReadWrite)".into();
    assert!(matches!(
        ShaderInterfacePlan::from_reflection(&[entry.clone(), other], &BTreeMap::new(), 4096),
        Err(InterfaceError::IncompatibleBinding(2, 9))
    ));
    let plan = ShaderInterfacePlan::from_reflection(&[entry], &BTreeMap::new(), 4096).unwrap();
    assert_eq!(plan.sets.len(), 3);
    assert!(plan.sets[0].bindings.is_empty());
    assert_eq!(plan.sets[2].bindings[0].binding, 9);
}

#[test]
fn original_vertex_and_blend_mapping() {
    for (tag, base) in [(2, 99), (3, 98), (4, 100)] {
        for components in 1..=4 {
            assert_eq!(
                map_vertex_attribute_format(tag, components * 4)
                    .unwrap()
                    .as_raw(),
                base + (components as i32 - 1) * 3
            );
        }
    }
    assert!(map_vertex_attribute_format(1, 4).is_err());
    assert!(map_vertex_attribute_format(4, 6).is_err());
    for flags in 0u8..=255 {
        let result = get_vk_blend_mode(flags);
        let mode = usize::from(flags >> 5 & 3);
        assert_eq!(result.blend_enable, u32::from(mode != 0));
        assert_eq!(result.dst_color_blend_factor.as_raw(), [0, 1, 7, 16][mode]);
        assert_eq!(result.dst_alpha_blend_factor.as_raw(), [0, 1, 7, 18][mode]);
        assert_eq!(result.color_write_mask, vk::ColorComponentFlags::RGBA);
    }
}

#[test]
fn reflected_stage_io_and_runtime_binding_validate_before_vulkan() {
    let vertex: ReflectedEntry = serde_json::from_value(serde_json::json!({"entry_point":"vs","stage":"Vertex","descriptors":[],"outputs":[{"location":0,"component":0,"type":{"detail":{"kind":"vec2<f32>"},"minimum_bytes":8}}]})).unwrap();
    let fragment: ReflectedEntry = serde_json::from_value(serde_json::json!({"entry_point":"ps","stage":"Fragment","descriptors":[],"inputs":[{"location":0,"component":0,"type":{"detail":{"kind":"vec3<f32>"},"minimum_bytes":12}}]})).unwrap();
    assert!(matches!(
        ShaderInterfacePlan::from_reflection(&[vertex, fragment], &BTreeMap::new(), 4096),
        Err(InterfaceError::IncompatibleLocation(0, 0))
    ));
    let runtime: ReflectedEntry = serde_json::from_value(serde_json::json!({"entry_point":"main","stage":"GLCompute","descriptors":[{"set":0,"binding":0,"name":"textures","count":0,"descriptor_type":"SampledImage"},{"set":0,"binding":1,"name":"buffer","count":1,"descriptor_type":"UniformBuffer"}]})).unwrap();
    assert!(matches!(
        ShaderInterfacePlan::from_reflection(&[runtime], &BTreeMap::new(), 4096),
        Err(InterfaceError::RuntimeBindingNotLast(0, 0))
    ));
}
