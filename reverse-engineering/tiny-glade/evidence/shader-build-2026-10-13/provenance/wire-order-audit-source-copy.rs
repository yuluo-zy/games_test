//! 原stage/SPIR-V保持不变，隔离元数据重新生产造成的wire排序差异。
//! 原stored reflection只作expected消费者合同，绝不传给package。
use rhapsody::{shader_compiler::pack::package, shader_pipeline::compiled::CompiledShaderPipeline};
use std::path::PathBuf;
#[test]
fn regenerated_packages_preserve_business_metadata_while_recording_wire_order_changes() {
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let game = project.join("../../ljxsj_92385/Tiny Glade/compiled-shaders");
    let mut paths = std::fs::read_dir(game).unwrap().map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|x|x.to_str())==Some("bin")).collect::<Vec<_>>();
    paths.sort();
    let mut facts = Vec::new();
    let mut exact = 0usize;
    let mut order_differences = [0usize; 6];
    for path in paths {
        let original_bytes = std::fs::read(&path).unwrap();
        let original = CompiledShaderPipeline::decode(&original_bytes).unwrap();
        let rebuilt = package(original.stages.clone(),original.source_dependencies.clone()).unwrap();
        assert_eq!(original.reflection.effective_descriptor_sets(),rebuilt.reflection.effective_descriptor_sets());
        assert_eq!(original.reflection.effective_vertex_inputs(),rebuilt.reflection.effective_vertex_inputs());
        for (name,expected) in &original.reflection.binding_names {
            assert_eq!(rebuilt.reflection.binding_by_name(name),Some(*expected));
        }
        let rebuilt_bytes = rebuilt.encode().unwrap();
        let roundtrip = CompiledShaderPipeline::decode(&rebuilt_bytes).unwrap();
        assert_eq!(roundtrip,rebuilt);
        let equal = original_bytes == rebuilt_bytes;
        exact += usize::from(equal);
        let ordering = [
            original.reflection.descriptor_sets != rebuilt.reflection.descriptor_sets,
            original.reflection.binding_names != rebuilt.reflection.binding_names,
            original.reflection.binding_name_hashes != rebuilt.reflection.binding_name_hashes,
            original.reflection.vertex_inputs != rebuilt.reflection.vertex_inputs,
            original.reflection.uniform_block.as_ref().map(|u|&u.name_offsets)
                != rebuilt.reflection.uniform_block.as_ref().map(|u|&u.name_offsets),
            original.reflection.uniform_block.as_ref().map(|u|&u.name_hash_offsets)
                != rebuilt.reflection.uniform_block.as_ref().map(|u|&u.name_hash_offsets),
        ];
        for (index,diff) in ordering.iter().enumerate() {order_differences[index] += usize::from(*diff);}
        assert_eq!(original_bytes.len(),rebuilt_bytes.len());
        if !equal { assert!(ordering.iter().any(|x|*x),"non-order wire change: {}",path.display()); }
        facts.push(serde_json::json!({"file":path.file_name().unwrap().to_string_lossy(),
            "byte_exact":equal,"bytes":original_bytes.len(),"first_changed_byte":original_bytes.iter().zip(&rebuilt_bytes).position(|(a,b)|a!=b),
            "changed_byte_count":original_bytes.iter().zip(&rebuilt_bytes).filter(|(a,b)|a!=b).count(),"ordering_differences":ordering}));
    }
    assert_eq!(facts.len(),404);
    let evidence = serde_json::json!({"mode":"metadata_regeneration_from_original_spirv",
        "files":facts.len(),"byte_exact":exact,"byte_different":facts.len()-exact,
        "metadata_business_equal":true,"original_metadata_injected":false,"original_shader_recompiled":false,
        "order_field_names":["descriptor_sets","binding_names","binding_name_hashes","vertex_inputs","uniform_name_offsets","uniform_name_hash_offsets"],
        "ordering_difference_counts":order_differences,"per_file":facts});
    let output = project.join("evidence/shader-build-2026-10-13/provenance/wire-order-audit.json");
    std::fs::write(output,serde_json::to_string_pretty(&evidence).unwrap()).unwrap();
}
