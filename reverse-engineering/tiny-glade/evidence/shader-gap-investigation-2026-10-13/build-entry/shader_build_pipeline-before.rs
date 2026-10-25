use rhapsody::{
    shader_compiler::{
        DxcStage,
        build::{BuildSpec, build_pipeline, raster_variants},
        preprocess::prepare_file,
        reflect::generate_reflection,
    },
    shader_pipeline::{
        compiled::CompiledShaderPipeline, pipeline_desc::RasterPipelineDesc,
        reflection::ShaderReflection, shader_defines::ShaderDefines,
    },
};
use std::path::PathBuf;
fn project() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn game() -> PathBuf {
    project().join("../../ljxsj_92385/Tiny Glade/compiled-shaders")
}
fn canonical(mut r: ShaderReflection) -> ShaderReflection {
    for set in &mut r.descriptor_sets {
        set.sort_by_key(|(i, _)| *i);
    }
    r.binding_names.sort_by(|a, b| a.0.cmp(&b.0));
    r.binding_name_hashes.sort_by_key(|x| x.0);
    r.vertex_inputs.sort_by(|a, b| a.0.cmp(&b.0));
    if let Some(u) = &mut r.uniform_block {
        u.name_offsets.sort_by(|a, b| a.0.cmp(&b.0));
        u.name_hash_offsets.sort_by_key(|x| x.0);
    }
    r
}
#[test]
fn regenerate_all_original_metadata_without_stored_injection() {
    let mut files = 0;
    let mut modules = 0;
    let mut mismatches = Vec::new();
    for entry in std::fs::read_dir(game()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("bin") {
            continue;
        }
        let original = CompiledShaderPipeline::decode(&std::fs::read(&path).unwrap()).unwrap();
        match generate_reflection(&original.stages) {
            Ok(generated)
                if canonical(generated.clone()) == canonical(original.reflection.clone()) => {}
            Ok(generated) => mismatches.push(format!(
                "{}: generated={generated:?}; expected={:?}",
                path.display(),
                original.reflection
            )),
            Err(e) => mismatches.push(format!("{}: {e}", path.display())),
        };
        files += 1;
        modules += original.stages.len();
    }
    std::fs::create_dir_all(project().join("evidence/shader-build-2026-10-13/pipeline")).unwrap();
    std::fs::write(
        project().join("evidence/shader-build-2026-10-13/pipeline/reflection-differences.txt"),
        mismatches.join("\n"),
    )
    .unwrap();
    assert_eq!((files, modules), (404, 638));
    assert!(
        mismatches.is_empty(),
        "{} metadata mismatches; first={:?}",
        mismatches.len(),
        mismatches.first()
    );
}
#[test]
fn exact_include_library_preserves_macro_conditions_and_tracks_dependencies() {
    let root = project().join("evidence/shader-build-2026-10-13/pipeline/preprocess-fixture");
    std::fs::create_dir_all(root.join("inc")).unwrap();
    std::fs::write(root.join("main.hlsl"),"#include \"inc/shared.hlsl\"\n#include \"inc/shared.hlsl\"\n#if FLAG\nfloat x=VALUE;\n#endif\n").unwrap();
    std::fs::write(
        root.join("inc/shared.hlsl"),
        "#pragma once\n#define VALUE 7\n",
    )
    .unwrap();
    let prepared = prepare_file(
        &root,
        "shaders",
        "main.hlsl",
        &ShaderDefines(vec![("FLAG".into(), "1".into())]),
    )
    .unwrap();
    assert_eq!(prepared.source.matches("#define VALUE 7").count(), 1);
    assert!(prepared.source.contains("#if FLAG"));
    assert!(prepared.source.contains("#line"));
    assert_eq!(
        prepared.dependencies,
        vec!["shaders/inc/shared.hlsl", "shaders/main.hlsl"]
    );
    std::fs::write(root.join("main.hlsl"), "#include \"missing.hlsl\"\n").unwrap();
    assert!(prepare_file(&root, "shaders", "main.hlsl", &ShaderDefines::default()).is_err());
}
#[test]
fn actual_dxc_preprocess_reflect_pack_selected_original_contracts() {
    let dxc = project().join("../../tools/shader-toolchain/dxc-v1.9.2609/bin/x64/dxc.exe");
    let output = project().join("evidence/shader-build-2026-10-13/pipeline");
    let source_root = project().join("reconstruction/shader-source/shaders");
    let mut cases = vec![(
        "fill",
        BuildSpec {
            source_root: source_root.clone(),
            logical_root: "shaders".into(),
            source_path: "fill_buffer.cs.hlsl".into(),
            defines: ShaderDefines::default(),
            stages: vec![(DxcStage::Compute, "cs_main".into())],
        },
        "_fill_buffer.cs.hlsl_cs_main_0.bin".to_string(),
    )];
    let variants = raster_variants(&RasterPipelineDesc::new_vertex_fragment(
        "/egui.raster.hlsl",
    ));
    for (index, v) in variants.into_iter().enumerate() {
        cases.push((
            if index == 0 {
                "egui-color"
            } else {
                "egui-depth"
            },
            BuildSpec {
                source_root: source_root.clone(),
                logical_root: "shaders".into(),
                source_path: "egui.raster.hlsl".into(),
                defines: v.defines,
                stages: vec![
                    (DxcStage::Vertex, "vs_main".into()),
                    (DxcStage::Fragment, "ps_main".into()),
                ],
            },
            if index == 0 {
                "_egui.raster.hlsl_b903e43ffb3da915.bin"
            } else {
                "_egui.raster.hlsl_f0adff76827e73ff.bin"
            }
            .into(),
        ));
    }
    for (name, spec, original_file) in cases {
        let work = output.join(format!("fresh-{name}-{}", std::process::id()));
        let built = build_pipeline(&dxc, &spec, &work).unwrap_or_else(|e| panic!("{name}: {e}"));
        std::fs::write(output.join(format!("fresh-{name}.bin")), &built.encoded).unwrap();
        let decoded = CompiledShaderPipeline::decode(&built.encoded).unwrap();
        assert_eq!(decoded, built.compiled);
        let original =
            CompiledShaderPipeline::decode(&std::fs::read(game().join(original_file)).unwrap())
                .unwrap();
        assert_eq!(
            canonical(decoded.reflection.clone()),
            canonical(original.reflection),
            "{name} metadata"
        );
        for stage in decoded.stages {
            assert_eq!(stage.spirv[2], 0x000f0000);
            let module = rspirv::dr::load_words(&stage.spirv).unwrap();
            for i in module.debug_string_source {
                if matches!(
                    i.class.opcode,
                    rspirv::spirv::Op::Source | rspirv::spirv::Op::SourceContinued
                ) {
                    for op in i.operands {
                        if let rspirv::dr::Operand::LiteralString(s) = op {
                            assert!(s.bytes().all(|b| b == b'\n'));
                        }
                    }
                }
            }
            assert!(!module.debug_module_processed.is_empty());
        }
        assert!(
            built
                .prepared
                .dependencies
                .iter()
                .any(|p| p.ends_with(&spec.source_path))
        );
    }
}

// 重建helper位于nani/inc；三个nani共有记录必须按上一级路径展开。
#[test]
fn owned_custom_includes_resolve_to_existing_nani_dependencies() {
    let root = project().join("reconstruction/shader-source/shaders");
    let mut inventory = Vec::new();
    for family in ["curved_halftimber", "plaster", "smoke", "stairs_floor"] {
        let source = format!("nani/cull_and_bucket_{family}.cs.hlsl");
        let prepared = prepare_file(&root, "shaders", &source, &ShaderDefines::default()).unwrap();
        for dependency in [
            "shaders/nani/draw_lists.inc.hlsl",
            "shaders/nani/cull_and_bucket.inc.hlsl",
            "shaders/nani/atlas_part_info.inc.hlsl",
            "shaders/nani/inc/cull_and_bucket_owned_custom.hlsl",
        ] {
            assert!(
                prepared.dependencies.iter().any(|p| p == dependency),
                "{source}: missing {dependency}"
            );
        }
        assert!(!prepared.dependencies.iter().any(|p| p.contains("/../")));
        inventory.push((source, prepared.dependencies));
    }
    std::fs::write(
        project().join("evidence/shader-build-2026-10-13/pipeline/owned-custom-dependencies.json"),
        serde_json::to_vec_pretty(&inventory).unwrap(),
    )
    .unwrap();
}
