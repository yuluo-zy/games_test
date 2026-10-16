use shader_compiler::{CompileError, DxcRequest, DxcStage, EmbeddedDxcRequest};
use std::path::{Path, PathBuf};

/// 仓库外层工作区（含 reverse-engineering/ 与默认的 ljxsj_92385/ 发行目录）。
fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../..")
}
/// B 发行 compiled-shaders：`TG_B_GAME_ROOT` 显式覆盖游戏根目录（沿用本仓库 `TG_B_*` 约定）；
/// 未设置时依次找原布局 `ljxsj_92385/Tiny Glade` 与证据目录里的发行副本 oracle-bin
/// （该副本已通过下方 638 模块偏移/命令逐项核对）。都不存在时打印原因并返回 None。
fn shipped_compiled_shaders() -> Option<PathBuf> {
    let candidates = match std::env::var_os("TG_B_GAME_ROOT") {
        Some(root) => vec![PathBuf::from(root).join("compiled-shaders")],
        None => vec![
            workspace().join("ljxsj_92385/Tiny Glade/compiled-shaders"),
            workspace()
                .join("reverse-engineering/tiny-glade/evidence/render-passes/shaders/oracle-bin"),
        ],
    };
    let found = candidates.iter().find(|dir| dir.is_dir()).cloned();
    if found.is_none() {
        eprintln!(
            "跳过：未找到 B 发行 compiled-shaders（已查 {candidates:?}）；设置 TG_B_GAME_ROOT 指向游戏根目录以启用"
        );
    }
    found
}

#[test]
fn shipped_b_638_module_requests_match_debug_strings_and_interfaces() {
    let evidence = workspace().join(
        "reverse-engineering/tiny-glade/evidence/render-passes/validation/compile-commands.json",
    );
    if !evidence.is_file() {
        eprintln!("跳过：未找到命令证据 {}", evidence.display());
        return;
    }
    let Some(binaries) = shipped_compiled_shaders() else {
        return;
    };
    let catalogue: serde_json::Value =
        serde_json::from_slice(&std::fs::read(evidence).unwrap()).unwrap();
    let mut counts = [0; 3];
    let mut seen = 0;
    for bin in catalogue["compile_commands"].as_array().unwrap() {
        let bytes = std::fs::read(binaries.join(bin["file"].as_str().unwrap())).unwrap();
        let compiled = shader_pipeline::compiled::CompiledShaderPipeline::decode(&bytes).unwrap();
        for (index, module) in bin["modules"].as_array().unwrap().iter().enumerate() {
            let offset = module["offset"].as_u64().unwrap() as usize;
            let nword = u32::from_le_bytes(bytes[offset - 4..offset].try_into().unwrap()) as usize;
            let payload = &bytes[offset..offset + nword * 4];
            assert_eq!(compiled.stages[index].spirv_bytes(), payload);
            let parsed = EmbeddedDxcRequest::from_compiled_stage(&compiled.stages[index]).unwrap();
            let raw = module["raw_command"].as_str().unwrap();
            assert_eq!(parsed, EmbeddedDxcRequest::parse(raw).unwrap());
            assert_eq!(parsed.request.entry(), module["entry"].as_str().unwrap());
            assert_eq!(
                parsed.request.stage().profile(),
                module["profile"].as_str().unwrap()
            );
            let args: Vec<_> = parsed
                .request
                .prepared_arguments(&parsed.input_temp, &parsed.output_temp)
                .iter()
                .map(|s| s.to_str().unwrap().to_owned())
                .collect();
            assert_eq!(args.join(" "), raw.strip_prefix("dxc-cl-option: ").unwrap());
            match parsed.request.stage() {
                DxcStage::Compute => counts[0] += 1,
                DxcStage::Vertex => counts[1] += 1,
                DxcStage::Fragment => counts[2] += 1,
            }
            seen += 1;
        }
    }
    assert_eq!(seen, 638);
    assert_eq!(counts, [170, 234, 234]);
}

#[test]
fn unknown_flags_profiles_and_malformed_commands_are_rejected() {
    let valid = "dxc-cl-option: /tmp/input -E cs_main -T cs_6_4 -spirv -WX -Ges -fspv-target-env=vulkan1.1 -HV 2021 -Zi -Fo /tmp/output -Qembed_debug";
    for bad in [
        valid.replace("cs_6_4", "cs_6_6"),
        valid.replace("-WX ", ""),
        format!("{valid} -I shaders"),
        valid.replace("-E cs_main", "-E"),
        valid.replace("cs_main", "bad;entry"),
        valid.replace("cs_main", "1wrong"),
    ] {
        assert!(matches!(
            EmbeddedDxcRequest::parse(&bad),
            Err(CompileError::UnsupportedCommand(_))
        ));
    }
    assert!(matches!(
        EmbeddedDxcRequest::from_module(&[0; 20]),
        Err(CompileError::InvalidSpirv(_))
    ));
}

#[test]
fn shipped_module_stage_swap_and_embedded_request_count_are_rejected() {
    let valid = "dxc-cl-option: /tmp/input -E cs_main -T cs_6_4 -spirv -WX -Ges -fspv-target-env=vulkan1.1 -HV 2021 -Zi -Fo /tmp/output -Qembed_debug";
    let Some(binaries) = shipped_compiled_shaders() else {
        return;
    };
    let bytes = std::fs::read(binaries.join("_blur.cs.hlsl_cs_main_0.bin")).unwrap();
    let n = u32::from_le_bytes(bytes[19..23].try_into().unwrap()) as usize;
    let payload = &bytes[23..23 + n * 4];
    assert!(matches!(
        DxcRequest::new(DxcStage::Fragment, "cs_main")
            .unwrap()
            .validate_module(payload),
        Err(CompileError::InterfaceMismatch)
    ));
    use rspirv::{
        binary::Assemble,
        dr::{Instruction, Operand},
        spirv::{Op, SourceLanguage},
    };
    let mut module = rspirv::dr::load_bytes(payload).unwrap();
    let command = module
        .debug_module_processed
        .iter()
        .find(|i| {
            i.operands
                .iter()
                .any(|op| matches!(op,Operand::LiteralString(s) if s.starts_with("dxc-cl-option:")))
        })
        .unwrap()
        .clone();
    let mut duplicate = module.clone();
    duplicate.debug_module_processed.push(command);
    let duplicate_bytes: Vec<u8> = duplicate
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert!(matches!(
        EmbeddedDxcRequest::from_module(&duplicate_bytes),
        Err(CompileError::AmbiguousEmbeddedRequest)
    ));
    module.debug_module_processed.retain(|i| {
        !i.operands
            .iter()
            .any(|op| matches!(op,Operand::LiteralString(s) if s.starts_with("dxc-cl-option:")))
    });
    let fake_id = module.header.as_ref().unwrap().bound;
    module.header.as_mut().unwrap().bound += 1;
    module.debug_string_source.push(Instruction::new(
        Op::Source,
        None,
        None,
        vec![
            Operand::SourceLanguage(SourceLanguage::HLSL),
            Operand::LiteralInt32(2021),
            Operand::IdRef(fake_id),
            Operand::LiteralString(valid.into()),
        ],
    ));
    module.debug_string_source.push(Instruction::new(
        Op::String,
        None,
        Some(fake_id),
        vec![Operand::LiteralString(valid.into())],
    ));
    let source_only: Vec<u8> = module
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert!(matches!(
        EmbeddedDxcRequest::from_module(&source_only),
        Err(CompileError::MissingEmbeddedRequest)
    ));
}

#[test]
fn replay_paths_are_single_arguments_and_source_adapter_is_explicit() {
    let request = DxcRequest::new(DxcStage::Compute, "cs_main").unwrap();
    let input = Path::new("D:/shader source/input.hlsl");
    let output = Path::new("D:/shader output/out.spv");
    let original = request.prepared_arguments(input, output);
    assert_eq!(original.len(), 15);
    assert_eq!(original[0], input.as_os_str());
    assert_eq!(original[13], output.as_os_str());
    let adapted = request.source_arguments(input, output, Path::new("D:/include root"));
    assert_eq!(&adapted[..15], &original);
    assert_eq!(adapted.len(), 17);
}
