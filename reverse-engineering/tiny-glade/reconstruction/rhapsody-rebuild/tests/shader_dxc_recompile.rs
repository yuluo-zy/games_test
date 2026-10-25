//! 真正执行微软DXC，覆盖恢复compute和原宏raster两变体的完整构建闭包。
//! 参数进程/打包结果测试不把原未知DXC版本、异常清理或shader算法记为已等价。
use rhapsody::{
    shader_compiler::{
        CompileError, DxcRequest, DxcStage,
        build::{BuildSpec, build_pipeline},
    },
    shader_pipeline::{compiled::CompiledShaderPipeline, shader_defines::ShaderDefines},
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
fn project() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn dxc() -> PathBuf {
    std::env::var_os("TINYGLADE_DXC")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            project().join("../../tools/shader-toolchain/dxc-v1.9.2609/bin/x64/dxc.exe")
        })
}
fn work(name: &str) -> PathBuf {
    let path = project()
        .join("evidence/shader-build-2026-10-13/dxc/test-work")
        .join(format!(
            "{}-{}-{}",
            name,
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&path).unwrap();
    path
}
fn spec(source: &str, raster: bool, depth: bool) -> BuildSpec {
    BuildSpec {
        source_root: project().join("reconstruction/shader-source/shaders"),
        logical_root: "shaders".into(),
        source_path: source.into(),
        defines: ShaderDefines(if raster {
            vec![(
                "SHADER_VARIANT_DEPTH_ONLY".into(),
                u8::from(depth).to_string(),
            )]
        } else {
            Vec::new()
        }),
        stages: if raster {
            vec![
                (DxcStage::Vertex, "vs_main".into()),
                (DxcStage::Fragment, "ps_main".into()),
            ]
        } else {
            vec![(DxcStage::Compute, "cs_main".into())]
        },
    }
}
#[test]
fn real_dxc_builds_restored_compute_and_both_original_raster_variants() {
    for (source, filename, raster, depth) in [
        (
            "fill_buffer.cs.hlsl",
            "_fill_buffer.cs.hlsl_cs_main_0.bin",
            false,
            false,
        ),
        (
            "egui.raster.hlsl",
            "_egui.raster.hlsl_b903e43ffb3da915.bin",
            true,
            false,
        ),
        (
            "egui.raster.hlsl",
            "_egui.raster.hlsl_f0adff76827e73ff.bin",
            true,
            true,
        ),
    ] {
        let description = spec(source, raster, depth);
        if raster {
            let expected = u64::from_str_radix(
                if depth {
                    "f0adff76827e73ff"
                } else {
                    "b903e43ffb3da915"
                },
                16,
            )
            .unwrap();
            assert_eq!(description.defines.artifact_hash(), expected);
        }
        let built = build_pipeline(&dxc(), &description, &work(filename)).unwrap();
        let decoded = CompiledShaderPipeline::decode(&built.encoded).unwrap();
        assert_eq!(decoded, built.compiled);
        let original = CompiledShaderPipeline::decode(
            &std::fs::read(
                project()
                    .join("../../ljxsj_92385/Tiny Glade/compiled-shaders")
                    .join(filename),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            decoded.reflection.effective_descriptor_sets(),
            original.reflection.effective_descriptor_sets(),
            "{filename}"
        );
        assert_eq!(
            decoded.reflection.push_constant_size, original.reflection.push_constant_size,
            "{filename}"
        );
        assert_eq!(
            decoded.reflection.local_size, original.reflection.local_size,
            "{filename}"
        );
        assert_eq!(decoded.stages.len(), original.stages.len());
        assert!(
            built
                .prepared
                .dependencies
                .contains(&format!("shaders/{source}"))
        );
        for stage in &decoded.stages {
            assert_eq!(stage.spirv[2], 0x000f0000);
        }
        if depth {
            assert!(
                rhapsody::raw::shader::reflect_spirv_entries(&decoded.stages[1].spirv).unwrap()[0]
                    .descriptors
                    .is_empty()
            );
        }
    }
}
#[test]
fn real_dxc_failure_preserves_diagnostics_and_existing_output_is_rejected() {
    let path = work("diagnostics");
    let input = path.join("bad prepared.hlsl");
    let output = path.join("out.spv");
    std::fs::write(&input, "not valid HLSL !!!").unwrap();
    let request = DxcRequest::new(DxcStage::Compute, "cs_main").unwrap();
    match request.compile_prepared(&dxc(), &input, &output) {
        Err(CompileError::DxcFailed {
            exit_code,
            stdout: _,
            stderr,
        }) => {
            assert!(exit_code.is_some());
            assert!(!stderr.is_empty());
        }
        other => panic!("unexpected compiler result: {other:?}"),
    }
    std::fs::write(&output, b"stale").unwrap();
    assert!(matches!(
        request.compile_prepared(&dxc(), &input, &output),
        Err(CompileError::OutputAlreadyExists(_))
    ));
    assert_eq!(std::fs::read(&output).unwrap(), b"stale");
}
