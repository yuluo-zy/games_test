//! 实际构建闭包：原include依赖展开→prepared source→真实DXC→原库反射→发行容器。
//! 本接口的工作目录/失败保留策略为明确候选，不冒认原缓存与临时文件清理。
use super::{
    DxcRequest, DxcStage,
    preprocess::{PreparedSource, prepare_file},
};
use crate::shader_pipeline::{
    compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
    pipeline_desc::RasterPipelineDesc,
    shader_defines::ShaderDefines,
};
use std::path::{Path, PathBuf};
#[derive(Clone, Debug)]
pub struct BuildSpec {
    pub source_root: PathBuf,
    pub logical_root: String,
    pub source_path: String,
    pub defines: ShaderDefines,
    pub stages: Vec<(DxcStage, String)>,
}
#[derive(Debug)]
pub struct BuiltPipeline {
    pub compiled: CompiledShaderPipeline,
    pub encoded: Vec<u8>,
    pub prepared: PreparedSource,
    /// 原请求提供的artifact宏；键只从此集合产生，不能混入复原配方。
    pub artifact_defines: ShaderDefines,
    /// Explicit source-branch selection macros of this build.
    pub recipe_defines: ShaderDefines,
}
/// 原variants函数克隆base后分别追加depth-only=0/1，不去重或改写已有defines。
pub fn raster_variants(base: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
    ["0", "1"].map(|value| {
        let mut result = base.clone();
        result
            .defines
            .0
            .push(("SHADER_VARIANT_DEPTH_ONLY".into(), value.into()));
        result
    })
}
/// 同一工作目录可连续构建（如 depth-only 0/1 两个变体）：只删除本构建自己的
/// `stage-{index}.spv` 旧产物，`compile_prepared` 的拒绝覆盖仍防止旧产物冒充本次成功。
/// 槽位不是普通文件（目录等）时报错而不递归删除；工作目录须由调用者独占。
fn fresh_stage_output(work_directory: &Path, index: usize) -> Result<PathBuf, String> {
    let output = work_directory.join(format!("stage-{index}.spv"));
    match std::fs::symlink_metadata(&output) {
        Ok(meta) if meta.is_file() || meta.is_symlink() => std::fs::remove_file(&output)
            .map_err(|e| format!("removing stale {}: {e}", output.display()))?,
        Ok(_) => {
            return Err(format!(
                "stage output slot {} is not a file",
                output.display()
            ));
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("inspecting {}: {e}", output.display())),
    }
    Ok(output)
}
pub fn build_pipeline(
    dxc: &Path,
    spec: &BuildSpec,
    work_directory: &Path,
) -> Result<BuiltPipeline, String> {
    build_pipeline_with_recipe(dxc, spec, &ShaderDefines::default(), work_directory)
}
/// 将显式复原配方追加到prepared宏输入；原artifact定义保持独立，不修改spec。
pub fn build_pipeline_with_recipe(
    dxc: &Path,
    spec: &BuildSpec,
    recipe_defines: &ShaderDefines,
    work_directory: &Path,
) -> Result<BuiltPipeline, String> {
    let compute = spec.stages.len() == 1 && spec.stages[0].0 == DxcStage::Compute;
    let raster = spec.stages.len() == 2
        && spec.stages[0].0 == DxcStage::Vertex
        && spec.stages[1].0 == DxcStage::Fragment;
    if !compute && !raster {
        return Err("unobserved pipeline stage list".into());
    }
    let mut prepared_defines = spec.defines.clone();
    prepared_defines.0.extend(recipe_defines.0.iter().cloned());
    let prepared = prepare_file(
        &spec.source_root,
        &spec.logical_root,
        &spec.source_path,
        &prepared_defines,
    )?;
    std::fs::create_dir_all(work_directory).map_err(|e| e.to_string())?;
    let input = work_directory.join("prepared.hlsl");
    std::fs::write(&input, &prepared.source).map_err(|e| e.to_string())?;
    // 先清掉本构建全部 stage 槽位，避免前一 stage 失败时留下上次构建的后续产物。
    let outputs = (0..spec.stages.len())
        .map(|index| fresh_stage_output(work_directory, index))
        .collect::<Result<Vec<_>, _>>()?;
    let mut stages = Vec::new();
    for ((stage, entry), output) in spec.stages.iter().zip(&outputs) {
        let request = DxcRequest::new(*stage, entry.clone()).map_err(|e| e.to_string())?;
        let bytes = request
            .compile_prepared(dxc, &input, output)
            .map_err(|e| e.to_string())?;
        let spirv = super::pack::prepare_spirv_for_package(&bytes)?;
        stages.push(CompiledShaderStage {
            stage: match stage {
                DxcStage::Compute => ShaderStage::Compute,
                DxcStage::Vertex => ShaderStage::Vertex,
                DxcStage::Fragment => ShaderStage::Fragment,
            },
            entry_point: entry.clone(),
            spirv,
        });
    }
    let compiled = super::pack::package(stages, prepared.dependencies.clone())?;
    let encoded = compiled.encode().map_err(|e| e.to_string())?;
    Ok(BuiltPipeline {
        compiled,
        encoded,
        prepared,
        artifact_defines: spec.defines.clone(),
        recipe_defines: recipe_defines.clone(),
    })
}
