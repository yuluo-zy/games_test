//! 全部原发行shader的真实新编译/反射/打包与残差清单。
//! 原资产仅作独立oracle；每个新包只由恢复源码和公开原依赖生产。
use rhapsody::raw::shader::reflect_spirv_entries;
use rhapsody::{
    shader_compiler::{
        DxcRequest, DxcStage,
        build::{BuildSpec, build_pipeline},
    },
    shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage},
        reflection::ShaderReflection,
        shader_defines::ShaderDefines,
    },
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::PathBuf};
fn semantic(r: &ShaderReflection) -> Value {
    let sets = r.effective_descriptor_sets().into_iter().map(|set|set.into_iter().map(|(index,binding)|json!({"binding":index,"kind":format!("{:?}",binding.kind),"array":format!("{:?}",binding.array),"name":binding.name})).collect::<Vec<_>>()).collect::<Vec<_>>();
    let names = r
        .binding_names
        .iter()
        .map(|(name, l)| (name.clone(), [l.set, l.binding]))
        .collect::<BTreeMap<_, _>>();
    let hashes = r
        .binding_name_hashes
        .iter()
        .map(|(name, l)| (*name, [l.set, l.binding]))
        .collect::<BTreeMap<_, _>>();
    let uniform=r.uniform_block.as_ref().map(|u|json!({"bytes":u.byte_size,"location":[u.binding.set,u.binding.binding],"names":u.name_offsets.iter().cloned().collect::<BTreeMap<_,_>>(),"hashes":u.name_hash_offsets.iter().copied().collect::<BTreeMap<_,_>>()}));
    let inputs = r
        .effective_vertex_inputs()
        .into_iter()
        .map(|(name, input)| {
            (
                name,
                json!({"location":input.location,"type":format!("{:?}",input.ty)}),
            )
        })
        .collect::<BTreeMap<_, _>>();
    json!({"sets":sets,"names":names,"hashes":hashes,"uniform":uniform,"inputs":inputs,"local_size":r.local_size,"push_bytes":r.push_constant_size})
}
fn histogram(
    stage: &CompiledShaderStage,
    include_debug: bool,
) -> Result<BTreeMap<String, usize>, String> {
    let module = rspirv::dr::load_words(&stage.spirv).map_err(|e| e.to_string())?;
    let mut result = BTreeMap::new();
    for instruction in module.all_inst_iter() {
        let opcode = instruction.class.opcode;
        if !include_debug
            && matches!(
                opcode,
                rspirv::spirv::Op::Source
                    | rspirv::spirv::Op::SourceContinued
                    | rspirv::spirv::Op::SourceExtension
                    | rspirv::spirv::Op::String
                    | rspirv::spirv::Op::Line
                    | rspirv::spirv::Op::NoLine
                    | rspirv::spirv::Op::ModuleProcessed
                    | rspirv::spirv::Op::Name
                    | rspirv::spirv::Op::MemberName
            )
        {
            continue;
        }
        *result.entry(format!("{opcode:?}")).or_default() += 1;
    }
    Ok(result)
}
fn capabilities(stage: &CompiledShaderStage) -> Result<Vec<u32>, String> {
    let module = rspirv::dr::load_words(&stage.spirv).map_err(|e| e.to_string())?;
    let mut caps = module
        .capabilities
        .iter()
        .filter_map(|inst| match inst.operands.first() {
            Some(rspirv::dr::Operand::Capability(c)) => Some(*c as u32),
            _ => None,
        })
        .collect::<Vec<_>>();
    caps.sort_unstable();
    caps.dedup();
    Ok(caps)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let dxc = args.get(1).map(PathBuf::from).unwrap_or_else(|| {
        project.join("../../tools/shader-toolchain/dxc-v1.9.2609/bin/x64/dxc.exe")
    });
    let output = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| project.join("evidence/shader-build-2026-10-13/dxc/corpus"));
    if output.exists() {
        return Err("audit output directory already exists; choose fresh directory".into());
    }
    std::fs::create_dir_all(&output)?;
    let catalogue: Value = serde_json::from_slice(&std::fs::read(
        project.join("evidence/render-passes/validation/compile-commands.json"),
    )?)?;
    let originals = project.join("../../ljxsj_92385/Tiny Glade/compiled-shaders");
    let source_root = project.join("reconstruction/shader-source/shaders");
    let mut rows = Vec::new();
    let mut failed_pipeline_stage_attempts = 0;
    let mut failed_pipeline_stage_successes = 0;
    let mut stage_preprocessing_failures = 0;
    let mut fresh_modules = 0;
    let mut compiled_bins = 0;
    let mut interface_matches = 0;
    let mut capability_matches = 0;
    let mut code_histogram_matches = 0;
    let mut schema_matches = 0;
    for item in catalogue["compile_commands"]
        .as_array()
        .ok_or("bad catalogue")?
    {
        let name = item["file"].as_str().ok_or("missing filename")?;
        let original = CompiledShaderPipeline::decode(&std::fs::read(originals.join(name))?)?;
        let source = item["main_source_path"]
            .as_str()
            .ok_or("missing source")?
            .strip_prefix("shaders/")
            .ok_or("unexpected source prefix")?;
        let mut defines = ShaderDefines::default();
        if original.stages.len() == 2 {
            let depth = if name.ends_with("_f0adff76827e73ff.bin") {
                "1"
            } else if name.ends_with("_b903e43ffb3da915.bin") {
                "0"
            } else {
                return Err("unobserved raster variant key".into());
            };
            defines
                .0
                .push(("SHADER_VARIANT_DEPTH_ONLY".into(), depth.into()));
        }
        let stages = item["modules"]
            .as_array()
            .ok_or("missing stages")?
            .iter()
            .map(|m| {
                let stage = match m["profile"].as_str() {
                    Some("cs_6_4") => Ok(DxcStage::Compute),
                    Some("vs_6_4") => Ok(DxcStage::Vertex),
                    Some("ps_6_4") => Ok(DxcStage::Fragment),
                    _ => Err("unexpected profile"),
                }?;
                Ok((
                    stage,
                    m["entry"].as_str().ok_or("missing entry")?.to_owned(),
                ))
            })
            .collect::<Result<Vec<_>, &str>>()?;
        let spec = BuildSpec {
            source_root: source_root.clone(),
            logical_root: "shaders".into(),
            source_path: source.into(),
            defines,
            stages,
        };
        let work = output.join(name.trim_end_matches(".bin"));
        let started = std::time::Instant::now();
        match build_pipeline(&dxc, &spec, &work) {
            Ok(built) => {
                compiled_bins += 1;
                fresh_modules += built.compiled.stages.len();
                let bin_path = work.join("fresh.bin");
                std::fs::write(&bin_path, &built.encoded)?;
                let decoded = CompiledShaderPipeline::decode(&built.encoded)?;
                if decoded != built.compiled {
                    return Err("new package roundtrip mismatch".into());
                }
                let old_schema = semantic(&original.reflection);
                let new_schema = semantic(&decoded.reflection);
                let schema_equal = old_schema == new_schema;
                schema_matches += usize::from(schema_equal);
                let mut module_rows = Vec::new();
                for (index, (old, new)) in original.stages.iter().zip(&decoded.stages).enumerate() {
                    let old_io = reflect_spirv_entries(&old.spirv)
                        .map_err(|e| format!("original reflection: {e:?}"))?;
                    let new_io = reflect_spirv_entries(&new.spirv)
                        .map_err(|e| format!("fresh reflection: {e:?}"))?;
                    let io_equal = serde_json::to_value(&old_io)? == serde_json::to_value(&new_io)?;
                    interface_matches += usize::from(io_equal);
                    let old_caps = capabilities(old)?;
                    let new_caps = capabilities(new)?;
                    let cap_equal = old_caps == new_caps;
                    capability_matches += usize::from(cap_equal);
                    let old_ops = histogram(old, false)?;
                    let new_ops = histogram(new, false)?;
                    let ops_equal = old_ops == new_ops;
                    code_histogram_matches += usize::from(ops_equal);
                    module_rows.push(json!({"index":index,"entry":new.entry_point,"interface_equal":io_equal,"capabilities_equal":cap_equal,"code_opcode_histogram_equal":ops_equal,"original_capabilities":old_caps,"fresh_capabilities":new_caps,"original_interface":old_io,"fresh_interface":new_io,"original_code_opcodes":old_ops,"fresh_code_opcodes":new_ops,"original_generator":old.spirv[2],"fresh_generator":new.spirv[2]}));
                }
                rows.push(json!({"file":name,"source":source,"defines":spec.defines.0,"status":"fresh_package_generated","fresh_package":bin_path,"elapsed_ms":u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),"semantic_schema_equal":schema_equal,"original_schema":old_schema,"fresh_schema":new_schema,"dependencies":built.prepared.dependencies,"modules":module_rows}));
            }
            Err(error) => {
                let prepared_file = work.join("prepared.hlsl");
                let mut stage_attempts = Vec::new();
                for (index, (stage, entry)) in spec.stages.iter().enumerate() {
                    if prepared_file.is_file() {
                        failed_pipeline_stage_attempts += 1;
                        let output = work.join(format!("audit-retry-stage-{index}.spv"));
                        let result = DxcRequest::new(*stage, entry.clone())
                            .and_then(|r| r.compile_prepared(&dxc, &prepared_file, &output));
                        match result {
                            Ok(bytes)=>{failed_pipeline_stage_successes+=1;stage_attempts.push(json!({"index":index,"entry":entry,"status":"dxc_generated_spirv_without_package","bytes":bytes.len(),"output":output}));},
                            Err(error)=>stage_attempts.push(json!({"index":index,"entry":entry,"status":"dxc_failed","error":error.to_string()})),
                        }
                    } else {
                        stage_preprocessing_failures += 1;
                        stage_attempts.push(json!({"index":index,"entry":entry,"status":"blocked_by_preprocessing"}));
                    }
                }
                rows.push(json!({"file":name,"source":source,"defines":spec.defines.0,"status":"build_failed","elapsed_ms":u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),"error":error,"work_directory":work,"stage_attempts":stage_attempts}));
            }
        }
        if rows.len() % 20 == 0 {
            eprintln!(
                "checked {} assets; generated {} packages / {} modules",
                rows.len(),
                compiled_bins,
                fresh_modules
            );
        }
    }
    let summary = json!({"assets_attempted":rows.len(),"original_modules":638,"packages_generated":compiled_bins,"fresh_modules_generated":fresh_modules,"failed_assets":rows.len()-compiled_bins,"dxc_module_successes_including_unpacked":fresh_modules+failed_pipeline_stage_successes,"original_stage_compile_attempts_or_preprocessing_failures":fresh_modules+failed_pipeline_stage_attempts+stage_preprocessing_failures,"stage_preprocessing_failures":stage_preprocessing_failures,"public_interface_equal_modules":interface_matches,"capability_equal_modules":capability_matches,"non_debug_opcode_histogram_equal_modules":code_histogram_matches,"semantic_schema_equal_assets":schema_matches,"actual_dxc":dxc,"original_full_shader_semantics_verified":false,"original_dxc_version":null});
    let report = json!({"summary":summary,"rows":rows});
    std::fs::write(
        output.join("report.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
