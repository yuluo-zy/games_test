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
    /// 原load_compiled先读文件再解码；两类失败各自展开path和底层错误，不启动编译。
    /// 返回String为现工程便捷适配；原入口以anyhow::Error::msg保存同一展开文本。
    pub fn load_compiled(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|error| {
            format!("Failed opening compiled shader pipeline from {path:?}: {error}")
        })?;
        Self::decode(&bytes).map_err(|error| {
            format!("Failed deserializing compiled shader pipeline from {path:?}: {error}")
        })
    }

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
impl CompiledShaderPipeline {
    /// 本工程的可选新鲜度检查（原 runtime 只读不判旧）：发行 `source_dependencies` 只存
    /// 逻辑路径（如 `shaders/inc/common.hlsl`），无内容 hash，因此按 mtime 比较。
    /// 依赖相对 `source_base`（含 `shaders/` 的目录）解析；任一依赖晚于 `compiled_path`
    /// 或已不存在即视为过期。绝对路径或逃出 `source_base` 的依赖报 InvalidData。
    /// 不改动文件命名/键：调用者在缓存命中后自行决定是否重编。
    pub fn is_stale(
        &self,
        compiled_path: impl AsRef<std::path::Path>,
        source_base: impl AsRef<std::path::Path>,
    ) -> std::io::Result<bool> {
        let built = std::fs::metadata(compiled_path)?.modified()?;
        for dependency in &self.source_dependencies {
            let path = dependency_path(source_base.as_ref(), dependency)?;
            match std::fs::metadata(&path) {
                Ok(meta) if meta.modified()? <= built => {}
                Ok(_) => return Ok(true),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(true),
                Err(error) => return Err(error),
            }
        }
        Ok(false)
    }
}
/// 依赖标签按 `/` 词法归一；原发行含 `eval/../x` 这类根内 `..`，只拒绝逃出根的。
fn dependency_path(base: &std::path::Path, label: &str) -> std::io::Result<std::path::PathBuf> {
    let invalid = || {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("source dependency {label:?} escapes the source base"),
        )
    };
    let mut parts: Vec<&str> = Vec::new();
    for part in label.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop().ok_or_else(invalid)?;
            }
            _ if part.contains(':') => return Err(invalid()),
            _ => parts.push(part),
        }
    }
    if label.starts_with(['/', '\\']) || parts.is_empty() {
        return Err(invalid());
    }
    Ok(parts
        .iter()
        .fold(base.to_path_buf(), |path, part| path.join(part)))
}
impl CompiledShaderStage {
    pub fn spirv_bytes(&self) -> Vec<u8> {
        self.spirv
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::dependency_path;
    use std::path::Path;

    #[test]
    fn dependency_labels_resolve_inside_the_base_only() {
        let base = Path::new("base");
        // 原发行依赖保留未归一的根内 `..`（如 canopy 材质）。
        assert_eq!(
            dependency_path(
                base,
                "shaders/inc/material/eval/../canopy_material.inc.hlsl"
            )
            .unwrap(),
            base.join("shaders/inc/material/canopy_material.inc.hlsl")
        );
        for bad in [
            "../x.hlsl",
            "shaders/../../x.hlsl",
            "/etc/passwd",
            "C:/x.hlsl",
            "",
            "..",
        ] {
            let error = dependency_path(base, bad).unwrap_err();
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidData, "{bad}");
        }
    }
}
