//! 本工程构建适配的回归：工作目录复用、include 根逃逸、重复宏、依赖新鲜度。
//! 真实 DXC 用例只在找到编译器时运行（`DXC` 覆盖，默认 `~/tools/dxc/bin/dxc`），否则打印跳过。
use shader_compiler::{
    DxcStage,
    build::{BuildSpec, build_pipeline, raster_variants},
    preprocess::prepare_file,
};
use shader_pipeline::{
    compiled::CompiledShaderPipeline, pipeline_desc::RasterPipelineDesc,
    shader_defines::ShaderDefines,
};
use std::path::{Path, PathBuf};

/// 每个用例独占的临时目录；先清掉同名残留，保证可重复运行。
fn scratch(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("tg-shader-compiler-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
fn dxc() -> Option<PathBuf> {
    let path = std::env::var_os("DXC").map(PathBuf::from).or_else(|| {
        std::env::var_os("HOME").map(|home| PathBuf::from(home).join("tools/dxc/bin/dxc"))
    })?;
    if path.is_file() {
        Some(path)
    } else {
        eprintln!(
            "跳过：未找到 DXC（{}）；设置 DXC 指向 dxc 可执行文件以启用",
            path.display()
        );
        None
    }
}
const RASTER: &str = "struct VsOut { float4 pos : SV_Position; };
VsOut vs_main(uint vid : SV_VertexID) {
    VsOut o;
    o.pos = float4(float(vid & 1), float(vid >> 1), 0.0, 1.0);
    return o;
}
float4 ps_main(VsOut i) : SV_Target {
#if SHADER_VARIANT_DEPTH_ONLY
    return 0.0;
#else
    return float4(i.pos.xy, 0.0, 1.0);
#endif
}
";
fn raster_spec(root: &Path, defines: ShaderDefines) -> BuildSpec {
    BuildSpec {
        source_root: root.into(),
        logical_root: "shaders".into(),
        source_path: "tri.raster.hlsl".into(),
        defines,
        stages: vec![
            (DxcStage::Vertex, "vs_main".into()),
            (DxcStage::Fragment, "ps_main".into()),
        ],
    }
}

#[test]
fn rebuild_in_same_work_directory_drops_own_stale_stage_outputs() {
    // 不依赖 DXC：残留的 stage-N.spv 不能再以 OutputAlreadyExists 挡住新构建，
    // 而失败时也不能留下旧产物冒充成功。
    let dir = scratch("stale-stage");
    let root = dir.join("src");
    write(&root.join("tri.raster.hlsl"), RASTER);
    let work = dir.join("work");
    write(&work.join("stage-0.spv"), "stale");
    write(&work.join("stage-1.spv"), "stale");
    let missing_dxc = dir.join("no-such-dxc");
    let error = build_pipeline(
        &missing_dxc,
        &raster_spec(&root, ShaderDefines::default()),
        &work,
    )
    .unwrap_err();
    assert!(!error.contains("OutputAlreadyExists"), "{error}");
    assert!(!work.join("stage-0.spv").exists());
    assert!(!work.join("stage-1.spv").exists());
}

#[test]
fn stage_output_slot_that_is_not_a_file_is_reported() {
    let dir = scratch("stage-dir");
    let root = dir.join("src");
    write(&root.join("tri.raster.hlsl"), RASTER);
    let work = dir.join("work");
    std::fs::create_dir_all(work.join("stage-0.spv")).unwrap();
    let error = build_pipeline(
        &dir.join("no-such-dxc"),
        &raster_spec(&root, ShaderDefines::default()),
        &work,
    )
    .unwrap_err();
    assert!(error.contains("stage-0.spv"), "{error}");
}

#[test]
fn both_raster_depth_variants_build_in_one_work_directory_with_real_dxc() {
    let Some(dxc) = dxc() else {
        return;
    };
    let dir = scratch("variants");
    let root = dir.join("src");
    write(&root.join("tri.raster.hlsl"), RASTER);
    let work = dir.join("work");
    let variants = raster_variants(&RasterPipelineDesc::new_vertex_fragment("/tri.raster.hlsl"));
    let mut built = Vec::new();
    for variant in variants {
        let result = build_pipeline(&dxc, &raster_spec(&root, variant.defines), &work).unwrap();
        assert_eq!(
            CompiledShaderPipeline::decode(&result.encoded).unwrap(),
            result.compiled
        );
        assert_eq!(
            result.compiled.source_dependencies,
            ["shaders/tri.raster.hlsl"]
        );
        built.push(result.compiled);
    }
    // 两个变体由宏分支产生不同 SPIR-V，证明第二次没有复用第一次的旧产物。
    assert_ne!(built[0].stages[1].spirv, built[1].stages[1].spirv);
}

#[test]
fn includes_escaping_the_source_root_are_rejected() {
    let dir = scratch("escape");
    let root = dir.join("src");
    write(&dir.join("secret.hlsl"), "float secret;\n");
    write(&root.join("inc/ok.hlsl"), "float ok;\n");
    write(&root.join("top.hlsl"), "#include \"../secret.hlsl\"\n");
    write(
        &root.join("nested/deep.hlsl"),
        "#include \"../../secret.hlsl\"\n",
    );
    write(
        &root.join("nested/fine.hlsl"),
        "#include \"../inc/ok.hlsl\"\n",
    );
    for source in ["top.hlsl", "nested/deep.hlsl", "../secret.hlsl"] {
        let error = prepare_file(&root, "shaders", source, &ShaderDefines::default()).unwrap_err();
        assert!(error.contains("escapes"), "{source}: {error}");
    }
    // 根内的 `..` 仍正常解析（原发行依赖里就有 `eval/../x` 形式）。
    let prepared = prepare_file(
        &root,
        "shaders",
        "nested/fine.hlsl",
        &ShaderDefines::default(),
    )
    .unwrap();
    assert_eq!(
        prepared.dependencies,
        ["shaders/inc/ok.hlsl", "shaders/nested/fine.hlsl"]
    );
}

#[test]
fn duplicate_define_names_are_rejected_before_they_cancel_in_the_key() {
    let dir = scratch("dup-define");
    write(&dir.join("a.hlsl"), "float a;\n");
    let pairs = |list: &[(&str, &str)]| {
        ShaderDefines(
            list.iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
        )
    };
    for defines in [
        pairs(&[("A", "1"), ("A", "1")]),
        pairs(&[("A", "1"), ("B", "2"), ("A", "0")]),
    ] {
        let error = prepare_file(&dir, "shaders", "a.hlsl", &defines).unwrap_err();
        assert!(error.contains("duplicate define `A`"), "{error}");
    }
    assert!(prepare_file(&dir, "shaders", "a.hlsl", &pairs(&[("A", "1"), ("B", "1")])).is_ok());
}

#[test]
fn freshly_built_dependencies_drive_the_opt_in_staleness_check() {
    let dir = scratch("stale-check");
    let root = dir.join("src");
    write(&root.join("inc/common.hlsl"), "float c;\n");
    write(&root.join("main.hlsl"), "#include \"inc/common.hlsl\"\n");
    let prepared = prepare_file(&root, "shaders", "main.hlsl", &ShaderDefines::default()).unwrap();
    let compiled = CompiledShaderPipeline {
        stages: Vec::new(),
        source_dependencies: prepared.dependencies,
        reflection: shader_pipeline::reflection::ShaderReflection {
            descriptor_sets: Vec::new(),
            uniform_block: None,
            binding_names: Vec::new(),
            binding_name_hashes: Vec::new(),
            vertex_inputs: Vec::new(),
            local_size: None,
            push_constant_size: 0,
        },
        trailing_bytes: Vec::new(),
    };
    let bin = dir.join("compiled.bin");
    std::fs::write(&bin, compiled.encode().unwrap()).unwrap();
    // 依赖标签含 logical_root，与原发行一致；新鲜度检查以其父目录为基准。
    let base = dir.clone();
    std::fs::rename(&root, dir.join("shaders")).unwrap();
    let old = std::time::SystemTime::now() - std::time::Duration::from_secs(60);
    for file in ["shaders/inc/common.hlsl", "shaders/main.hlsl"] {
        std::fs::File::options()
            .write(true)
            .open(base.join(file))
            .unwrap()
            .set_modified(old)
            .unwrap();
    }
    assert!(!compiled.is_stale(&bin, &base).unwrap());
    std::fs::File::options()
        .write(true)
        .open(base.join("shaders/inc/common.hlsl"))
        .unwrap()
        .set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60))
        .unwrap();
    assert!(compiled.is_stale(&bin, &base).unwrap());
    // 依赖被删除同样视为过期；逃出基准目录的标签报错而不是静默通过。
    std::fs::remove_file(base.join("shaders/inc/common.hlsl")).unwrap();
    assert!(compiled.is_stale(&bin, &base).unwrap());
    let escaping = CompiledShaderPipeline {
        source_dependencies: vec!["../outside.hlsl".into()],
        ..compiled
    };
    assert!(escaping.is_stale(&bin, &base).is_err());
}

#[test]
fn cross_stage_binding_name_conflict_names_stage_and_field_with_real_dxc() {
    let Some(dxc) = dxc() else {
        return;
    };
    let dir = scratch("name-conflict");
    let root = dir.join("src");
    write(
        &root.join("tri.raster.hlsl"),
        "[[vk::binding(0, 0)]] Texture2D<float4> vs_tex;
[[vk::binding(0, 0)]] Texture2D<float4> ps_tex;
float4 vs_main(uint vid : SV_VertexID) : SV_Position {
    return vs_tex.Load(int3(vid, 0, 0));
}
float4 ps_main(float4 pos : SV_Position) : SV_Target {
    return ps_tex.Load(int3(pos.xy, 0));
}
",
    );
    let error = build_pipeline(
        &dxc,
        &raster_spec(&root, ShaderDefines::default()),
        &dir.join("work"),
    )
    .unwrap_err();
    assert!(
        error.contains("set=0 binding=0")
            && error.contains("fragment stage `ps_main` differs in name"),
        "{error}"
    );
}
