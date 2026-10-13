use rhapsody::shader_pipeline::{
    compiled::{CompiledShaderPipeline, ShaderStage},
    pipeline_desc::{ComputePipelineDesc, RasterPipelineDesc},
    reflection::{BindingArray, BindingKind, BindingNameHash},
    shader_defines::ShaderDefines,
};
use std::path::PathBuf;
fn original_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../ljxsj_92385/Tiny Glade/compiled-shaders")
}
#[test]
fn all_shipped_containers_roundtrip_with_stored_reflection() {
    let mut files = 0;
    let mut stages = 0;
    let mut uniform_blocks = 0;
    let mut vertex_inputs = 0;
    for entry in std::fs::read_dir(original_dir()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|x| x.to_str()) != Some("bin") {
            continue;
        }
        let bytes = std::fs::read(&path).unwrap();
        let pipeline = CompiledShaderPipeline::decode(&bytes)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert!(pipeline.trailing_bytes.is_empty(), "{}", path.display());
        assert_eq!(pipeline.encode().unwrap(), bytes, "{}", path.display());
        for stage in &pipeline.stages {
            let parsed = rspirv::dr::load_words(&stage.spirv).unwrap();
            let model = match stage.stage {
                ShaderStage::Compute => rspirv::spirv::ExecutionModel::GLCompute,
                ShaderStage::Vertex => rspirv::spirv::ExecutionModel::Vertex,
                ShaderStage::Fragment => rspirv::spirv::ExecutionModel::Fragment,
            };
            assert!(parsed.entry_points.iter().any(|entry| matches!(entry.operands.first(),Some(rspirv::dr::Operand::ExecutionModel(m)) if *m==model) && matches!(entry.operands.get(2),Some(rspirv::dr::Operand::LiteralString(n)) if n==&stage.entry_point)));
        }
        for (name, location) in &pipeline.reflection.binding_names {
            assert_eq!(
                pipeline
                    .reflection
                    .binding_by_hash(BindingNameHash::new(name)),
                Some(*location),
                "{} / {name}",
                path.display()
            );
            let set = &pipeline.reflection.descriptor_sets[location.set as usize];
            let stored = set
                .iter()
                .rev()
                .find(|(key, _)| *key == location.binding)
                .unwrap();
            assert_eq!(&stored.1.name, name);
        }
        if let Some(block) = &pipeline.reflection.uniform_block {
            uniform_blocks += 1;
            assert_eq!(
                pipeline.reflection.binding_by_name("$Globals"),
                Some(block.binding)
            );
            for (name, offset) in &block.name_offsets {
                assert!(*offset < block.byte_size);
                assert!(
                    block
                        .name_hash_offsets
                        .iter()
                        .any(|(h, o)| *h == BindingNameHash::new(name).0 && o == offset)
                );
            }
        }
        for (_, input) in &pipeline.reflection.vertex_inputs {
            assert!(input.ty.nbyte().is_some());
            vertex_inputs += 1;
        }
        files += 1;
        stages += pipeline.stages.len();
    }
    assert_eq!((files, stages), (404, 638));
    assert!(uniform_blocks > 0);
    assert!(vertex_inputs > 0);
}
#[test]
fn rejects_truncation_invalid_enum_and_utf8_without_magic_scan() {
    let original =
        std::fs::read(original_dir().join("_fill_buffer.cs.hlsl_cs_main_0.bin")).unwrap();
    for n in [0, 3, 8, 12, 22, 23, 100, original.len() - 1] {
        assert!(
            CompiledShaderPipeline::decode(&original[..n]).is_err(),
            "prefix {n}"
        );
    }
    let mut bad = original.clone();
    bad[4..8].copy_from_slice(&3u32.to_le_bytes());
    assert!(CompiledShaderPipeline::decode(&bad).is_err());
    let mut bad = original.clone();
    bad[12] = 255;
    assert!(CompiledShaderPipeline::decode(&bad).is_err());
    let mut appended = original.clone();
    appended.extend_from_slice(&[1, 2, 3]);
    let decoded = CompiledShaderPipeline::decode(&appended).unwrap();
    assert_eq!(decoded.trailing_bytes, [1, 2, 3]);
    assert_eq!(decoded.encode().unwrap(), appended);
}
#[test]
fn original_binding_hash_suffix_rule_and_array_tags() {
    assert_eq!(BindingNameHash::new("output").0, 1517523925760426884);
    assert_eq!(
        BindingNameHash::new("output_dims"),
        BindingNameHash::new("output").with_suffix_dims()
    );
    assert_eq!(
        BindingNameHash(u64::MAX).with_suffix_dims(),
        BindingNameHash(0)
    );
    assert_eq!(
        BindingNameHash::new("_dims").0,
        const_fnv1a_hash::fnv1a_hash_64(b"_dims", None).wrapping_add(1)
    );
    let pipeline = CompiledShaderPipeline::decode(
        &std::fs::read(original_dir().join("_fill_buffer.cs.hlsl_cs_main_0.bin")).unwrap(),
    )
    .unwrap();
    let binding = &pipeline.reflection.descriptor_sets[0][0].1;
    assert_eq!(
        (binding.kind, binding.array),
        (BindingKind::StorageBuffer, BindingArray::Fixed(1))
    );
}
#[test]
fn original_raster_macro_keys_and_paths() {
    let defs =
        |value: &str| ShaderDefines(vec![("SHADER_VARIANT_DEPTH_ONLY".into(), value.into())]);
    assert_eq!(defs("0").artifact_hash(), 0xb903e43ffb3da915);
    assert_eq!(defs("1").artifact_hash(), 0xf0adff76827e73ff);
    let mut compute = ComputePipelineDesc::new("/fill_buffer.cs.hlsl");
    assert_eq!(
        compute.compiled_file_path(),
        PathBuf::from("compiled-shaders/_fill_buffer.cs.hlsl_cs_main_0.bin")
    );
    compute.defines = ShaderDefines(vec![("a".into(), "b".into()), ("a".into(), "b".into())]);
    assert_eq!(compute.defines.artifact_hash(), 0);
    let mut raster = RasterPipelineDesc::new_vertex_fragment("/egui.raster.hlsl");
    raster.defines = defs("0");
    assert_eq!(
        raster.compiled_file_path(),
        PathBuf::from("compiled-shaders/_egui.raster.hlsl_b903e43ffb3da915.bin")
    );
}

#[test]
fn original_type_wire_tags_and_opaque_size_boundaries() {
    use rhapsody::shader_pipeline::reflection_ty::{ScalarType, StructMember, StructType, Type};
    use speedy::{LittleEndian, Readable, Writable};
    for (tag, ty) in [
        (3, Type::Image),
        (4, Type::CombinedImageSampler),
        (5, Type::SampledImage),
        (6, Type::StorageImage),
        (7, Type::Sampler),
        (8, Type::SubpassData),
        (11, Type::AccelStruct),
        (12, Type::DeviceAddress),
    ] {
        let bytes = (tag as u32).to_le_bytes();
        let decoded = Type::read_from_buffer_with_ctx(LittleEndian {}, &bytes).unwrap();
        assert_eq!(decoded, ty);
        assert_eq!(
            decoded.write_to_vec_with_ctx(LittleEndian {}).unwrap(),
            bytes
        );
        assert_eq!(decoded.nbyte(), if tag == 12 { Some(8) } else { None });
    }
    // 原StructType只看最后成员；opaque末尾类型贡献0字节，保留末尾offset。
    let ty = Type::Struct(StructType {
        name: None,
        members: vec![StructMember {
            name: None,
            offset: 12,
            ty: Type::Image,
        }],
    });
    assert_eq!(ty.nbyte(), Some(12));
    let ty = Type::Scalar(ScalarType::Float(4));
    assert_eq!(ty.nbyte(), Some(4));
    assert!(Type::read_from_buffer_with_ctx(LittleEndian {}, &14u32.to_le_bytes()).is_err());
}
