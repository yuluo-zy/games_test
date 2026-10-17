//! 从 B 发行 SPIR-V OpModuleProcessed 调试记录恢复的 DXC 请求合同。
//! 子模块接通公开预处理器、真实 DXC、原 SPIR-Q 和发行格式打包；完整原配置/异常策略仍待核。
//! 原输入是临时文件；源码 include 路径属于显式候选适配，不计入原命令。
use rspirv::{dr::Operand, spirv::ExecutionModel};
pub use shader_pipeline;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DxcStage {
    Compute,
    Vertex,
    Fragment,
}
impl DxcStage {
    pub fn profile(self) -> &'static str {
        match self {
            Self::Compute => "cs_6_4",
            Self::Vertex => "vs_6_4",
            Self::Fragment => "ps_6_4",
        }
    }
    fn execution_model(self) -> ExecutionModel {
        match self {
            Self::Compute => ExecutionModel::GLCompute,
            Self::Vertex => ExecutionModel::Vertex,
            Self::Fragment => ExecutionModel::Fragment,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxcRequest {
    stage: DxcStage,
    entry: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbeddedDxcRequest {
    pub request: DxcRequest,
    pub input_temp: PathBuf,
    pub output_temp: PathBuf,
}
#[derive(Debug)]
pub enum CompileError {
    UnsupportedCommand(String),
    InvalidSpirv(String),
    MissingEmbeddedRequest,
    AmbiguousEmbeddedRequest,
    InterfaceMismatch,
    Io(std::io::Error),
    OutputAlreadyExists(PathBuf),
    DxcFailed {
        exit_code: Option<i32>,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    },
}
impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DxcFailed {
                exit_code,
                stdout,
                stderr,
            } => write!(
                f,
                "DXC failed ({exit_code:?}):\n{}{}",
                String::from_utf8_lossy(stdout),
                String::from_utf8_lossy(stderr)
            ),
            Self::Io(error) => std::fmt::Display::fmt(error, f),
            _ => write!(f, "{self:?}"),
        }
    }
}
impl std::error::Error for CompileError {}
impl From<std::io::Error> for CompileError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl DxcRequest {
    pub fn new(stage: DxcStage, entry: impl Into<String>) -> Result<Self, CompileError> {
        let entry = entry.into();
        if !entry
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
            || !entry
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            return Err(CompileError::UnsupportedCommand(
                "unsupported entry identifier".into(),
            ));
        }
        Ok(Self { stage, entry })
    }
    pub fn stage(&self) -> DxcStage {
        self.stage
    }
    pub fn entry(&self) -> &str {
        &self.entry
    }
    /// 为已准备输入生成参数，保持原作顺序；路径作为单个 OS 参数传递。
    pub fn prepared_arguments(&self, input: &Path, output: &Path) -> Vec<OsString> {
        vec![
            input.as_os_str().into(),
            "-E".into(),
            self.entry.clone().into(),
            "-T".into(),
            self.stage.profile().into(),
            "-spirv".into(),
            "-WX".into(),
            "-Ges".into(),
            "-fspv-target-env=vulkan1.1".into(),
            "-HV".into(),
            "2021".into(),
            "-Zi".into(),
            "-Fo".into(),
            output.as_os_str().into(),
            "-Qembed_debug".into(),
        ]
    }
    /// 恢复源码树的候选适配。原嵌入命令没有 -I；预处理等价性需另行验证。
    pub fn source_arguments(
        &self,
        input: &Path,
        output: &Path,
        include_root: &Path,
    ) -> Vec<OsString> {
        let mut args = self.prepared_arguments(input, output);
        args.extend([OsString::from("-I"), include_root.as_os_str().into()]);
        args
    }
    pub fn validate_module(&self, bytes: &[u8]) -> Result<(), CompileError> {
        let module =
            rspirv::dr::load_bytes(bytes).map_err(|e| CompileError::InvalidSpirv(e.to_string()))?;
        let matches = module.entry_points.iter().filter(|ep| {
            matches!(ep.operands.first(), Some(Operand::ExecutionModel(model)) if *model == self.stage.execution_model())
                && matches!(ep.operands.get(2), Some(Operand::LiteralString(name)) if name == &self.entry)
        }).count();
        if matches != 1 {
            return Err(CompileError::InterfaceMismatch);
        }
        Ok(())
    }
    /// 候选进程适配，诊断保留原字节。拒绝已有输出，防止旧产物掩盖失败。
    /// 输入/输出及临时文件由调用者独占和管理；未恢复原作异常清理规则。
    pub fn compile_prepared(
        &self,
        dxc: &Path,
        input: &Path,
        output: &Path,
    ) -> Result<Vec<u8>, CompileError> {
        if output.try_exists()? {
            return Err(CompileError::OutputAlreadyExists(output.into()));
        }
        let result = Command::new(dxc)
            .args(self.prepared_arguments(input, output))
            .output()?;
        if !result.status.success() {
            return Err(CompileError::DxcFailed {
                exit_code: result.status.code(),
                stdout: result.stdout,
                stderr: result.stderr,
            });
        }
        let bytes = std::fs::read(output)?;
        self.validate_module(&bytes)?;
        Ok(bytes)
    }
}

impl EmbeddedDxcRequest {
    /// 从共享发行格式的 stage 读取请求，同时核对序列化阶段/入口与产物。
    pub fn from_compiled_stage(
        stage: &crate::shader_pipeline::compiled::CompiledShaderStage,
    ) -> Result<Self, CompileError> {
        use crate::shader_pipeline::compiled::ShaderStage;
        let parsed = Self::from_module(&stage.spirv_bytes())?;
        let expected = match stage.stage {
            ShaderStage::Compute => DxcStage::Compute,
            ShaderStage::Vertex => DxcStage::Vertex,
            ShaderStage::Fragment => DxcStage::Fragment,
        };
        if parsed.request.stage != expected || parsed.request.entry != stage.entry_point {
            return Err(CompileError::InterfaceMismatch);
        }
        Ok(parsed)
    }

    /// 只接受 B 已观察到的 638 条命令语法；未知 profile、附加选项或含空白的旧路径报错。
    pub fn parse(raw: &str) -> Result<Self, CompileError> {
        let body = raw.strip_prefix("dxc-cl-option: ").ok_or_else(|| {
            CompileError::UnsupportedCommand("missing debug-string prefix".into())
        })?;
        let words: Vec<_> = body.split_whitespace().collect();
        let expected = [
            (1, "-E"),
            (3, "-T"),
            (5, "-spirv"),
            (6, "-WX"),
            (7, "-Ges"),
            (8, "-fspv-target-env=vulkan1.1"),
            (9, "-HV"),
            (10, "2021"),
            (11, "-Zi"),
            (12, "-Fo"),
            (14, "-Qembed_debug"),
        ];
        if words.len() != 15 || expected.iter().any(|(i, value)| words[*i] != *value) {
            return Err(CompileError::UnsupportedCommand(
                "unobserved argument shape".into(),
            ));
        }
        let stage = match words[4] {
            "cs_6_4" => DxcStage::Compute,
            "vs_6_4" => DxcStage::Vertex,
            "ps_6_4" => DxcStage::Fragment,
            _ => {
                return Err(CompileError::UnsupportedCommand(
                    "unobserved profile".into(),
                ));
            }
        };
        Ok(Self {
            request: DxcRequest::new(stage, words[2])?,
            input_temp: words[0].into(),
            output_temp: words[13].into(),
        })
    }
    /// 公共 SPIR-V 解码复用原 rspirv，按模块配对并核对入口与阶段。
    pub fn from_module(bytes: &[u8]) -> Result<Self, CompileError> {
        let module =
            rspirv::dr::load_bytes(bytes).map_err(|e| CompileError::InvalidSpirv(e.to_string()))?;
        let mut requests = module
            .debug_module_processed
            .iter()
            .filter(|i| i.class.opcode == rspirv::spirv::Op::ModuleProcessed)
            .flat_map(|i| &i.operands)
            .filter_map(|op| match op {
                Operand::LiteralString(s) if s.starts_with("dxc-cl-option:") => Some(s.as_str()),
                _ => None,
            });
        let raw = requests
            .next()
            .ok_or(CompileError::MissingEmbeddedRequest)?;
        if requests.next().is_some() {
            return Err(CompileError::AmbiguousEmbeddedRequest);
        }
        let parsed = Self::parse(raw)?;
        parsed.request.validate_module(bytes)?;
        Ok(parsed)
    }
}

// 公共预处理器、反射和原格式打包的完整构建入口。
pub mod build;
pub mod bindless;
pub mod pack;
pub mod preprocess;
pub mod reflect;
