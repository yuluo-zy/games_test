//! shader 编译输入与发行文件路径；描述不同于 Vulkan 固定状态。
//! 文件路径/defines键已原指令对照；描述字段与构造函数为候选便利API，不声明原完整对象还原。
use super::shader_defines::ShaderDefines;
use std::path::PathBuf;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComputePipelineDesc {
    pub source_path: String,
    pub entry_point: String,
    pub defines: ShaderDefines,
}
impl ComputePipelineDesc {
    /// 候选便利构造：选择发行样本常见cs_main；不声称原new的完整默认字段/optional语义。
    pub fn new(source_path: impl Into<String>) -> Self {
        Self::with_entry(source_path, "cs_main")
    }
    /// 候选显式输入构造，不等同原with_entry的所有Option/所有权分支。
    pub fn with_entry(source_path: impl Into<String>, entry_point: impl Into<String>) -> Self {
        Self {
            source_path: source_path.into(),
            entry_point: entry_point.into(),
            defines: ShaderDefines::default(),
        }
    }
    /// 原函数只替换正斜线，不做系统路径归一化。
    pub fn compiled_file_path(&self) -> PathBuf {
        format!(
            "compiled-shaders/{}_{}_{:x}.bin",
            self.source_path.replace('/', "_"),
            self.entry_point,
            self.defines.artifact_hash()
        )
        .into()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RasterPipelineDesc {
    pub source_path: String,
    pub vertex_entry: String,
    pub fragment_entry: String,
    pub defines: ShaderDefines,
}
impl RasterPipelineDesc {
    /// 候选便利构造：常见vs_main/ps_main、空defines；原depth-only宏由外部variants消费者追加。
    /// 只表示此API的输入选择，不将空defines记为原构造默认事实。
    pub fn new_vertex_fragment(source_path: impl Into<String>) -> Self {
        Self {
            source_path: source_path.into(),
            vertex_entry: "vs_main".into(),
            fragment_entry: "ps_main".into(),
            defines: ShaderDefines::default(),
        }
    }
    /// 原 raster 文件键没有入口名。
    pub fn compiled_file_path(&self) -> PathBuf {
        format!(
            "compiled-shaders/{}_{:x}.bin",
            self.source_path.replace('/', "_"),
            self.defines.artifact_hash()
        )
        .into()
    }
}
