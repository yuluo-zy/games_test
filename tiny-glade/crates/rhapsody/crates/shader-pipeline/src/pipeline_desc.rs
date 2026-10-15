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
    /// 原 `with_entry`（0x140539370 / 0x1409e3ab0）：入口为空串时用默认 `cs_main`，否则 `to_owned`。
    /// 原 `ComputePipelineDesc` 布局 = `{source_path String@0, entry String@0x18, defines Vec@0x30}`
    /// （0x141795d10 读 `[desc+8/+0x10]` 路径、`[desc+0x38/+0x40]` defines）。
    pub fn with_entry(source_path: impl Into<String>, entry_point: impl Into<String>) -> Self {
        let entry_point = entry_point.into();
        Self {
            source_path: source_path.into(),
            entry_point: if entry_point.is_empty() {
                "cs_main".into()
            } else {
                entry_point
            },
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
/// 原 `RasterPipelineDesc` 是 0x48B（O，0x1409e3980 构造、0x141796080 取键）：
/// `{source_path String@0, defines Vec<(String,String)>@0x18, String@0x30}`；第三个字符串在
/// `new_vertex_fragment` 里是 path 的克隆，文件键只用 `@0`/`@0x18`（无入口名）。`@0x30` 的语义
/// 未证实（下面的 `vertex_entry/fragment_entry` 是候选便利字段，不对应原布局；真正入口名来自
/// 编译容器每个 stage 的 entry 字符串，见 0x141477105）。
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_file_names_match_the_shipped_bins() {
        let mut desc = RasterPipelineDesc::new_vertex_fragment("/billboard_forest.raster.hlsl");
        desc.defines.0.push(("SHADER_VARIANT_DEPTH_ONLY".into(), "0".into()));
        assert_eq!(
            desc.compiled_file_path().to_str().unwrap(),
            "compiled-shaders/_billboard_forest.raster.hlsl_b903e43ffb3da915.bin"
        );
        desc.defines.0[0].1 = "1".into();
        assert!(desc
            .compiled_file_path()
            .to_str()
            .unwrap()
            .ends_with("_f0adff76827e73ff.bin"));
    }

    #[test]
    fn compute_entry_defaults_to_cs_main_when_empty() {
        let desc = ComputePipelineDesc::with_entry("/apply_screen_shadows.cs.hlsl", "");
        assert_eq!(desc.entry_point, "cs_main");
        assert_eq!(
            desc.compiled_file_path().to_str().unwrap(),
            "compiled-shaders/_apply_screen_shadows.cs.hlsl_cs_main_0.bin"
        );
    }
}
