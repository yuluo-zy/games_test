//! include扫描和pragma once直接复用原shader-prepper；其余C/HLSL宏条件仍交DXC。
//! 文件provider的相对优先/根回退是明确适配；原完整路径失败策略尚未知。
use crate::shader_pipeline::shader_defines::ShaderDefines;
use relative_path::RelativePath;
use shader_prepper::{
    BoxedIncludeProviderError, IncludeProvider, ResolvedInclude, ResolvedIncludePath,
};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};
#[derive(Debug)]
pub struct PreparedSource {
    pub source: String,
    pub dependencies: Vec<String>,
}
struct FileProvider {
    root: PathBuf,
    logical_root: String,
    dependencies: BTreeSet<String>,
}
impl FileProvider {
    fn relative<'a>(&self, path: &'a str) -> &'a str {
        path.strip_prefix(&format!("{}/", self.logical_root))
            .unwrap_or(path)
    }
}
impl IncludeProvider for FileProvider {
    type IncludeContext = String;
    fn resolve_path(
        &self,
        path: &str,
        context: &String,
    ) -> Result<ResolvedInclude<String>, BoxedIncludeProviderError> {
        let path = path.trim_start_matches('/');
        let parent = RelativePath::new(self.relative(context))
            .parent()
            .unwrap_or_else(|| RelativePath::new(""));
        let relative = parent.join(path).normalize();
        let root_relative = RelativePath::new(path).normalize();
        let chosen = if relative.to_path(&self.root).is_file() {
            relative
        } else {
            root_relative
        };
        let label = format!("{}/{}", self.logical_root, chosen);
        Ok(ResolvedInclude {
            resolved_path: ResolvedIncludePath(label.clone()),
            context: label,
        })
    }
    fn get_include(
        &mut self,
        path: &ResolvedIncludePath,
    ) -> Result<String, BoxedIncludeProviderError> {
        let physical = RelativePath::new(self.relative(&path.0)).to_path(&self.root);
        let source = std::fs::read_to_string(physical)?;
        self.dependencies.insert(path.0.clone());
        Ok(source)
    }
}
/// 每个chunk发出#line，保留原发行嵌入的逻辑文件名；不把-I当include展开。
pub fn prepare_file(
    root: &Path,
    logical_root: &str,
    source: &str,
    defines: &ShaderDefines,
) -> Result<PreparedSource, String> {
    let mut provider = FileProvider {
        root: root.into(),
        logical_root: logical_root.trim_end_matches('/').into(),
        dependencies: BTreeSet::new(),
    };
    let chunks = shader_prepper::process_file(source, &mut provider, String::new())
        .map_err(|e| e.to_string())?;
    let mut text = String::new();
    for (name, value) in &defines.0 {
        if name.is_empty()
            || !name
                .bytes()
                .enumerate()
                .all(|(i, b)| b == b'_' || b.is_ascii_alphabetic() || (i > 0 && b.is_ascii_digit()))
            || value.contains(['\n', '\r'])
        {
            return Err("invalid define input".into());
        }
        text.push_str(&format!("#define {name} {value}\n"));
    }
    for chunk in chunks {
        text.push_str(&format!(
            "#line {} \"{}\"\n",
            chunk.line_offset + 1,
            chunk.file.replace('\\', "/").replace('"', "\\\"")
        ));
        text.push_str(&chunk.source);
        if !text.ends_with('\n') {
            text.push('\n');
        }
    }
    Ok(PreparedSource {
        source: text,
        dependencies: provider.dependencies.into_iter().collect(),
    })
}
