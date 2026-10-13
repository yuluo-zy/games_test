//! 发行 .bin 的全部声明字段；编解码复用原 speedy 0.8.6。
//! 源文件归属、类型标签与消费者合同见本轮原指令证据。
use super::reflection::ShaderReflection;
use speedy::{LittleEndian, Readable, Writable};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Readable, Writable)]
pub enum ShaderStage {
    Compute,
    Vertex,
    Fragment,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct CompiledShaderStage {
    pub stage: ShaderStage,
    pub entry_point: String,
    pub spirv: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
struct CompiledWire {
    stages: Vec<CompiledShaderStage>,
    source_dependencies: Vec<String>,
    reflection: ShaderReflection,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompiledShaderPipeline {
    pub stages: Vec<CompiledShaderStage>,
    pub source_dependencies: Vec<String>,
    pub reflection: ShaderReflection,
    /// speedy原read_from_buffer接受未消费尾字节；保留它们以免回写丢失。
    pub trailing_bytes: Vec<u8>,
}
#[derive(Debug)]
pub struct CompiledDecodeError(pub speedy::Error);
impl std::fmt::Display for CompiledDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for CompiledDecodeError {}
impl CompiledShaderPipeline {
    /// 严格按字段顺序读取，绝不搜索任意偏移的 SPIR-V magic。
    /// 原入口允许尾字节，存入trailing_bytes；格式合法性由speedy检查。
    pub fn decode(bytes: &[u8]) -> Result<Self, CompiledDecodeError> {
        let (prefix, used) =
            CompiledWire::read_with_length_from_buffer_with_ctx(LittleEndian {}, bytes);
        let prefix = prefix.map_err(CompiledDecodeError)?;
        Ok(Self {
            stages: prefix.stages,
            source_dependencies: prefix.source_dependencies,
            reflection: prefix.reflection,
            trailing_bytes: bytes[used..].to_vec(),
        })
    }
    /// 发行字段和有序map记录逐字节回写，尾字节仍保留。
    pub fn encode(&self) -> Result<Vec<u8>, speedy::Error> {
        let mut bytes = CompiledWire {
            stages: self.stages.clone(),
            source_dependencies: self.source_dependencies.clone(),
            reflection: self.reflection.clone(),
        }
        .write_to_vec_with_ctx(LittleEndian {})?;
        bytes.extend_from_slice(&self.trailing_bytes);
        Ok(bytes)
    }
}
impl CompiledShaderStage {
    pub fn spirv_bytes(&self) -> Vec<u8> {
        self.spirv
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect()
    }
}
