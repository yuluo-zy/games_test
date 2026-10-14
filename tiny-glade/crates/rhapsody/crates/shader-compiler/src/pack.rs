//! 游戏发行后处理合同：保留调试文件/行号和编译命令，源正文只保留换行。
//! 新header的generator直接来自原rspirv0.11默认，不手写或重实现公共assembler。
use crate::shader_pipeline::compiled::{CompiledShaderPipeline, CompiledShaderStage};
use rspirv::{
    binary::Assemble,
    dr::{ModuleHeader, Operand},
    spirv::Op,
};
pub fn prepare_spirv_for_package(bytes: &[u8]) -> Result<Vec<u32>, String> {
    let mut module = rspirv::dr::load_bytes(bytes).map_err(|e| e.to_string())?;
    let old = module.header.as_ref().ok_or("missing SPIR-V header")?;
    let version = old.version();
    let mut header = ModuleHeader::new(old.bound);
    header.set_version(version.0, version.1);
    module.header = Some(header);
    for instruction in &mut module.debug_string_source {
        if matches!(instruction.class.opcode, Op::Source | Op::SourceContinued) {
            for operand in &mut instruction.operands {
                if let Operand::LiteralString(source) = operand {
                    *source = source.chars().filter(|c| *c == '\n').collect();
                }
            }
        }
    }
    Ok(module.assemble())
}
/// 仅用新编译stage与实际provider依赖生成容器；不接收外部stored reflection。
pub fn package(
    stages: Vec<CompiledShaderStage>,
    source_dependencies: Vec<String>,
) -> Result<CompiledShaderPipeline, String> {
    let reflection = super::reflect::generate_reflection(&stages)?;
    Ok(CompiledShaderPipeline {
        stages,
        source_dependencies,
        reflection,
        trailing_bytes: Vec::new(),
    })
}
