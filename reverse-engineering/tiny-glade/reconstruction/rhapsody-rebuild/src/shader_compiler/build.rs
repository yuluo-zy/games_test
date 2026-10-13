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
    /// 显式的重建源码分支选择宏；不冒认为原作配置。
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
    let mut stages = Vec::new();
    for (index, (stage, entry)) in spec.stages.iter().enumerate() {
        let output = work_directory.join(format!("stage-{index}.spv"));
        let request = DxcRequest::new(*stage, entry.clone()).map_err(|e| e.to_string())?;
        let bytes = request
            .compile_prepared(dxc, &input, &output)
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
