//! X1 driver: build every manifest pipeline with DXC, apply the production bindless
//! lowering, and dump original + lowered SPIR-V for downstream translation experiments
//! (tools/metal_experiments/x1). No Metal code is involved; this runs anywhere DXC does.
//!
//! cargo run -p shader-compiler --example x1_lower_all -- \
//!     --manifest shaders/manifest.json --shaders shaders --dxc ~/tools/dxc/bin/dxc \
//!     --out /tmp/x1 [--limit 64] [--filter substring] [--jobs 4]
//!
//! Per pipeline `<out>/<bin>/`:
//!   orig-<i>.spv, lowered-<i>.spv  one per stage
//!   meta.json                      file, defines, stage list, SPIR-V capabilities/extensions
//!   abi.txt                        the lowered ABI (Debug)
//! `<out>/summary.tsv` has one row per pipeline (status, error text). Failures are results,
//! not crashes: the run always finishes and reports them.
use serde_json::{Value, json};
use shader_compiler::{
    DxcStage,
    bindless::lower_pipeline_with_limit,
    build::{BuildSpec, build_pipeline_with_recipe},
    shader_pipeline::shader_defines::ShaderDefines,
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

struct Job {
    bin: String,
    file: String,
    stages: Vec<(DxcStage, String)>,
    artifact_defines: Vec<(String, String)>,
    recipe_defines: Vec<(String, String)>,
}

struct Args {
    manifest: PathBuf,
    shaders: PathBuf,
    dxc: PathBuf,
    out: PathBuf,
    limit: u32,
    filter: Option<String>,
    jobs: usize,
}

fn parse_args() -> Result<Args, String> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut it = std::env::args().skip(1);
    while let Some(key) = it.next() {
        let key = key.strip_prefix("--").ok_or(format!("unexpected argument {key}"))?.to_string();
        map.insert(key.clone(), it.next().ok_or(format!("--{key} needs a value"))?);
    }
    let get = |k: &str| map.get(k).cloned().ok_or(format!("--{k} is required"));
    Ok(Args {
        manifest: get("manifest")?.into(),
        shaders: get("shaders")?.into(),
        dxc: get("dxc")?.into(),
        out: get("out")?.into(),
        limit: map.get("limit").map(|v| v.parse().map_err(|e| format!("--limit: {e}"))).transpose()?.unwrap_or(64),
        filter: map.get("filter").cloned(),
        jobs: map.get("jobs").map(|v| v.parse().map_err(|e| format!("--jobs: {e}"))).transpose()?.unwrap_or(4).max(1),
    })
}

fn stage_of(profile: &str) -> Option<DxcStage> {
    match profile.get(..2)? {
        "cs" => Some(DxcStage::Compute),
        "vs" => Some(DxcStage::Vertex),
        "ps" => Some(DxcStage::Fragment),
        _ => None,
    }
}

/// Group manifest build entries by produced `bin`; a raster pipeline has a vs and a ps entry.
fn jobs_from_manifest(path: &Path, filter: Option<&str>) -> Result<Vec<Job>, String> {
    let manifest: Value = serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let build = manifest["build"].as_object().ok_or("manifest has no build object")?;
    let mut jobs = Vec::new();
    for (file, entries) in build {
        if filter.is_some_and(|f| !file.contains(f)) {
            continue;
        }
        let mut by_bin: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
        for entry in entries.as_array().ok_or("build entry is not an array")? {
            by_bin.entry(entry["bin"].as_str().unwrap_or_default().to_string()).or_default().push(entry);
        }
        for (bin, entries) in by_bin {
            let mut stages = Vec::new();
            for entry in &entries {
                let profile = entry["profile"].as_str().unwrap_or_default();
                let stage = stage_of(profile).ok_or(format!("{file}: unknown profile {profile}"))?;
                stages.push((stage, entry["entry"].as_str().unwrap_or_default().to_string()));
            }
            // The builder requires [compute] or [vertex, fragment]; the manifest order is arbitrary
            // (final_post lists ps before vs), so order by stage.
            stages.sort_by_key(|(stage, _)| match stage {
                DxcStage::Compute | DxcStage::Vertex => 0,
                DxcStage::Fragment => 1,
            });
            let first = entries[0];
            let pairs = |key: &str| -> Vec<(String, String)> {
                first[key].as_array().into_iter().flatten().filter_map(|p| {
                    Some((p.get(0)?.as_str()?.to_string(), p.get(1)?.as_str()?.to_string()))
                }).collect()
            };
            // `defines` are macro names (TGV_<hash>) defined to 1; `artifact_defines` are (name, value).
            let recipe: Vec<(String, String)> = first["defines"].as_array().into_iter().flatten()
                .filter_map(|d| d.as_str().map(|n| (n.to_string(), "1".to_string()))).collect();
            jobs.push(Job { bin, file: file.clone(), stages, artifact_defines: pairs("artifact_defines"), recipe_defines: recipe });
        }
    }
    Ok(jobs)
}

fn spirv_facts(bytes: &[u8]) -> Value {
    match rspirv::dr::load_bytes(bytes) {
        Ok(module) => {
            let caps: Vec<String> = module.capabilities.iter().map(|i| format!("{:?}", i.operands[0])).collect();
            let exts: Vec<String> = module.extensions.iter().map(|i| format!("{:?}", i.operands[0])).collect();
            json!({ "capabilities": caps, "extensions": exts, "binding_types": binding_types(&module) })
        }
        Err(e) => json!({ "parse_error": format!("{e:?}") }),
    }
}


/// For every (set, binding): the distinct resource types declared on it. After the production
/// lowering, descriptor arrays of one Vulkan descriptor type share a binding, so a binding with
/// more than one image/buffer type is exactly the "aliased array" case SPIRV-Cross must handle.
fn binding_types(module: &rspirv::dr::Module) -> Value {
    use rspirv::{dr::Operand, spirv::{Decoration, Op}};
    let mut ty: std::collections::HashMap<u32, &rspirv::dr::Instruction> = std::collections::HashMap::new();
    for inst in &module.types_global_values {
        if let Some(id) = inst.result_id {
            ty.insert(id, inst);
        }
    }
    let id_operand = |inst: &rspirv::dr::Instruction, index: usize| -> Option<u32> {
        match inst.operands.get(index) { Some(Operand::IdRef(id)) => Some(*id), _ => None }
    };
    // Describe a type id, looking through pointers and arrays.
    fn describe(id: u32, ty: &std::collections::HashMap<u32, &rspirv::dr::Instruction>) -> String {
        use rspirv::{dr::Operand, spirv::Op};
        let Some(inst) = ty.get(&id) else { return format!("%{id}") };
        let next = |i: usize| match inst.operands.get(i) { Some(Operand::IdRef(n)) => Some(*n), _ => None };
        match inst.class.opcode {
            Op::TypePointer => next(1).map(|n| describe(n, ty)).unwrap_or_default(),
            Op::TypeRuntimeArray | Op::TypeArray => next(0).map(|n| describe(n, ty)).unwrap_or_default(),
            Op::TypeSampledImage => format!("sampled({})", next(0).map(|n| describe(n, ty)).unwrap_or_default()),
            Op::TypeImage => {
                let sampled_type = next(0).map(|n| describe(n, ty)).unwrap_or_default();
                let words: Vec<String> = inst.operands[1..].iter().map(|o| match o {
                    Operand::Dim(d) => format!("{d:?}"),
                    Operand::LiteralInt32(v) => v.to_string(),
                    Operand::ImageFormat(f) => format!("{f:?}"),
                    other => format!("{other:?}"),
                }).collect();
                format!("image<{sampled_type},{}>", words.join(","))
            }
            Op::TypeSampler => "sampler".into(),
            Op::TypeStruct => format!("struct%{}", inst.result_id.unwrap_or(0)),
            Op::TypeFloat | Op::TypeInt => format!("{:?}{:?}", inst.class.opcode, inst.operands),
            other => format!("{other:?}"),
        }
    }
    let mut set_of: std::collections::HashMap<u32, u32> = Default::default();
    let mut binding_of: std::collections::HashMap<u32, u32> = Default::default();
    for inst in &module.annotations {
        if inst.class.opcode != Op::Decorate { continue }
        let (Some(target), Some(Operand::Decoration(dec)), Some(Operand::LiteralInt32(value))) =
            (id_operand(inst, 0), inst.operands.get(1), inst.operands.get(2)) else { continue };
        match dec {
            Decoration::DescriptorSet => { set_of.insert(target, *value); }
            Decoration::Binding => { binding_of.insert(target, *value); }
            _ => {}
        }
    }
    let mut groups: BTreeMap<(u32, u32), std::collections::BTreeSet<String>> = BTreeMap::new();
    for inst in &module.types_global_values {
        if inst.class.opcode != Op::Variable { continue }
        let (Some(id), Some(ptr)) = (inst.result_id, inst.result_type) else { continue };
        let (Some(&set), Some(&binding)) = (set_of.get(&id), binding_of.get(&id)) else { continue };
        groups.entry((set, binding)).or_default().insert(describe(ptr, &ty));
    }
    let all: Vec<Value> = groups.iter().map(|((set, binding), types)| json!({
        "set": set, "binding": binding, "types": types.iter().collect::<Vec<_>>(),
    })).collect();
    let aliased: Vec<&Value> = all.iter().filter(|g| g["types"].as_array().is_some_and(|t| t.len() > 1)).collect();
    json!({ "bindings": all.len(), "aliased": aliased })
}

fn run_job(job: &Job, args: &Args) -> Result<Value, String> {
    let work = args.out.join(&job.bin);
    std::fs::create_dir_all(&work).map_err(|e| e.to_string())?;
    let spec = BuildSpec {
        source_root: args.shaders.clone(),
        logical_root: "shaders".into(),
        source_path: job.file.clone(),
        defines: ShaderDefines(job.artifact_defines.clone()),
        stages: job.stages.clone(),
    };
    let built = build_pipeline_with_recipe(&args.dxc, &spec, &ShaderDefines(job.recipe_defines.clone()), &work.join("dxc"))?;
    let mut stage_meta = Vec::new();
    for (i, stage) in built.compiled.stages.iter().enumerate() {
        let orig = stage.spirv_bytes();
        std::fs::write(work.join(format!("orig-{i}.spv")), &orig).map_err(|e| e.to_string())?;
        stage_meta.push(json!({ "index": i, "stage": format!("{:?}", stage.stage), "orig": spirv_facts(&orig) }));
    }
    let (abi, lowered) = lower_pipeline_with_limit(&built.compiled, args.limit).map_err(|e| format!("lowering: {e:?}"))?;
    for (i, stage) in lowered.iter().enumerate() {
        let bytes = stage.spirv_bytes();
        std::fs::write(work.join(format!("lowered-{i}.spv")), &bytes).map_err(|e| e.to_string())?;
        stage_meta[i]["lowered"] = spirv_facts(&bytes);
    }
    std::fs::write(work.join("abi.txt"), format!("{abi:#?}\n")).map_err(|e| e.to_string())?;
    let meta = json!({
        "bin": job.bin, "file": job.file, "limit": args.limit,
        "artifact_defines": job.artifact_defines, "recipe_defines": job.recipe_defines,
        "stages": stage_meta,
    });
    std::fs::write(work.join("meta.json"), serde_json::to_vec_pretty(&meta).unwrap()).map_err(|e| e.to_string())?;
    Ok(meta)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    if !args.dxc.is_file() {
        return Err(format!("dxc not found at {}", args.dxc.display()).into());
    }
    let jobs = jobs_from_manifest(&args.manifest, args.filter.as_deref())?;
    std::fs::create_dir_all(&args.out)?;
    let rows = Mutex::new(Vec::<(String, String, String, String)>::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..args.jobs {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                let (status, error) = match run_job(job, &args) {
                    Ok(_) => ("ok".to_string(), String::new()),
                    Err(e) => ("fail".to_string(), e.replace(['\n', '\t'], " ").chars().take(400).collect()),
                };
                rows.lock().unwrap().push((job.bin.clone(), job.file.clone(), status, error));
            });
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort();
    let mut tsv = String::from("bin\tfile\tstatus\terror\n");
    for (bin, file, status, error) in &rows {
        tsv.push_str(&format!("{bin}\t{file}\t{status}\t{error}\n"));
    }
    std::fs::write(args.out.join("summary.tsv"), tsv)?;
    let failed = rows.iter().filter(|r| r.2 != "ok").count();
    println!("{} pipelines, {} ok, {} failed; limit {}", rows.len(), rows.len() - failed, failed, args.limit);
    Ok(())
}
