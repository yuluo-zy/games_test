//! shader_uri.rs — ShaderUri 解析与 AsComputeShaderSource trait。
//!
//! 机器码锚点（均 O，逐条核对）：
//! - `ShaderUri::parse` 0x140968960（367B）：sret 48B = 3 个 &str 对，见下。
//! - `impl$1::source_hash<&str>` 0x1408a4300（37B）：`calculate_hash(&(self_str, &()))`。
//! - `impl$1::as_compute_pipeline<&str>` 0x1408a4330（400B）：`ShaderUri::parse` →
//!   `ComputePipelineDesc{path: String(uri.path), entry: "cs_main", defines: []}.with_entry(uri.entry)`。
//! - `calculate_hash` 0x1405bdda0：`DefaultHasher::new()`（SipHash-1-3，key=0；`str` 写字节后补 0xff；
//!   `&()` 不写）。`render::utils::calculate_hash` 的 std 写法与之逐位一致。
//! - 18 个游戏实现类型（DoF/nani/bin_lights/taa/vermeer 等）见 contracts/shader_uri.md §3。

use core::fmt;

/// 解析后的 shader URI（48B = 3×(ptr,len)）。
///
/// 格式 `<dir>/<stem>.<ext>[:<entry>]`。`parse` 的三次反向搜索（O，memrchr）：
///  1. 最后一个 `:` → `path = s[..idx]`、`entry = Some(s[idx+1..])`；没有则 `path = s`、`entry = None`
///     （None 的编码是指针 0，即 `Option<&str>` 的 niche）。
///  2. 在 `path` 里找最后一个 `/`，其后为文件名；没有则整段 `path` 是文件名。
///  3. 在文件名里找最后一个 `.`；**找不到 → `Option::unwrap` panic**。`stem = file[..dot]`。
///
/// 旧版把第 1 项当成 "whole"、第 3 项当成 "entry"，并把 entry 当必选，三者含义都不对。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderUri<'a> {
    /// +0x00：`:` 之前的全部（含目录与扩展名）。
    pub path: &'a str,
    /// +0x10：文件名去掉最后一个 `.` 及其后（不含目录）。
    pub stem: &'a str,
    /// +0x20：`:` 之后的入口名；无 `:` 时为 None。
    pub entry: Option<&'a str>,
}

impl<'a> ShaderUri<'a> {
    pub fn parse(uri: &'a str) -> ShaderUri<'a> {
        let (path, entry) = match uri.rfind(':') {
            Some(i) => (&uri[..i], Some(&uri[i + 1..])),
            None => (uri, None),
        };
        let file = match path.rfind('/') {
            Some(i) => &path[i + 1..],
            None => path,
        };
        let dot = file.rfind('.').unwrap();
        ShaderUri {
            path,
            stem: &file[..dot],
            entry,
        }
    }
}

impl fmt::Debug for ShaderUri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // impl$18::fmt 0x140939420（行 214）：具体格式 U，沿用旧实现的字符串输出（I）。
        match self.entry {
            Some(e) => write!(f, "{}:{}", self.path, e),
            None => write!(f, "{}", self.path),
        }
    }
}

impl fmt::Display for ShaderUri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// `shader_pipeline` 外部库占位：`ComputePipelineDesc`（0x48 B：path String @0、entry String @0x18、
/// defines Vec @0x30；O，来自 `with_entry` 0x1409e3ab0 的读写偏移）。库内逻辑不在本工程范围。
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ComputePipelineDesc {
    pub path: String,
    pub entry: String,
    /// 元素类型 U（as_compute_pipeline 恒置空）。
    pub defines: Vec<(String, String)>,
}

/// `defines` 元素：`(宏名, 值)`，值是 `Display` 格式化后的文本（O：`bin_lights` 的 `PERM_write = format!("{}", u32)`；
/// `TgsrMain` 的 `PERM_wave32`/`PERM_sharpen`；DoF 五个源的 `PERM_quality`；`GenerateStrokes` 的 `PERM_photomode`）。
pub type ShaderDefine = (String, String);

/// `ShaderDefine` 构造（`PERM_*` 宏）。
pub fn perm_define(name: &str, value: impl core::fmt::Display) -> ShaderDefine {
    (name.to_owned(), value.to_string())
}

/// 默认入口名（O：`as_compute_pipeline` 写入的 7 字节 `cs_main`）。
pub const DEFAULT_ENTRY: &str = "cs_main";

impl ComputePipelineDesc {
    /// 追加一个 define（B 阶段 i_api；O：`as_compute_pipeline` 往 `+0x30` 的 `Vec<(String, String)>` 里 push）。
    pub fn with_define(mut self, name: &str, value: impl core::fmt::Display) -> Self {
        self.defines.push(perm_define(name, value));
        self
    }

    /// `ComputePipelineDesc::with_entry<&str>`（O 0x1409e3ab0）：参数为 `Option<&str>`，None 取 `cs_main`。
    pub fn with_entry(mut self, entry: Option<&str>) -> Self {
        self.entry = entry.unwrap_or(DEFAULT_ENTRY).to_owned();
        self
    }
}

/// `<&str as AsComputeShaderSource>::as_compute_pipeline` 的真实逻辑（O 0x1408a4330）。
pub fn compute_pipeline_from_uri(uri: &str) -> ComputePipelineDesc {
    let u = ShaderUri::parse(uri);
    ComputePipelineDesc {
        path: u.path.to_owned(),
        entry: DEFAULT_ENTRY.to_owned(),
        defines: Vec::new(),
    }
    .with_entry(u.entry)
}

/// 编译期特性值（`AsShaderDefValue::as_shader_def_value` 的载体）。
/// 与 INDEX.md 观测到的“每 raster pass 两个 16-hex 变体对（编译特性）”对应。
/// 具体编码 U —— typed gap：仅作 trait 返回类型。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderDefValue(pub u64);

/// 把计算 pass 结构体转成编译期特性值。
/// impl$3/impl$4（行 130 附近）。
pub trait AsShaderDefValue {
    fn as_shader_def_value(&self) -> ShaderDefValue;
}

/// `ComputePass::new` 使用的源码描述：URI + 特性值。
pub struct ComputeShaderSource<'a> {
    pub uri: ShaderUri<'a>,
    pub defs: ShaderDefValue,
}

/// compute 源码 hash 计算器（`ComputeShaderSourceHash::compute`，inline 22）。
/// hash 算法本身 U；原入口 source_hash 为 37B 薄封装。
#[derive(Clone, Copy, Debug, Default)]
pub struct ComputeShaderSourceHash;

impl ComputeShaderSourceHash {
    /// O（0x1408a4300 → 0x1405bdda0）：`calculate_hash(&(source, &()))`。
    pub fn compute(source: &str) -> u64 {
        crate::render::utils::calculate_hash(&(source, &()))
    }
}

/// 声明一个 compute pass 的 shader 来源。
/// `source_hash`（行 48）与 `as_compute_pipeline`（行 52）对应 trait 两方法。
/// 18 个游戏实现类型 = compute pass 清单（contracts/shader_uri.md §3）。
pub trait AsComputeShaderSource {
    fn source_hash(&self) -> u64;
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_>;
    /// `as_compute_pipeline` 返回的 `ComputePipelineDesc.defines`（B 阶段 i_api 追加；缺省空 = `&str`/`String` 实现的 O 行为）。
    /// 带 `PERM_*` 的源（`TgsrMain`、`BinLights`、DoF、`GaussBlur`、`GenerateStrokes`…）覆写它；`ComputePass::new` 把它交给
    /// `ComputeShaderProgram::new(.., desc, ..)`。`ComputeShaderSource.defs: ShaderDefValue(u64)` 装不下 `Vec<(String, String)>`，
    /// 故不放在 `defs` 里（`defs` 保持单个 u64 的特性值，与既有实现源码兼容）。
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        Vec::new()
    }
}

/// `&str` / `String` 的通用实现（ComputePass::new<&str> / new<String> 默认路径）。
/// impl$1（行 48/52）。
impl AsComputeShaderSource for &str {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute(self)
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse(self),
            defs: ShaderDefValue::default(),
        }
    }
}

impl AsComputeShaderSource for String {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute(self)
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse(self),
            defs: ShaderDefValue::default(),
        }
    }
}

/// 动态 trait 对象形式（`ComputePass::new<&dyn AsComputeShaderSource>`）。
impl AsComputeShaderSource for Box<dyn AsComputeShaderSource> {
    fn source_hash(&self) -> u64 {
        (**self).source_hash()
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        (**self).as_compute_pipeline()
    }
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        (**self).pipeline_defines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_entry() {
        let u = ShaderUri::parse("/tgsr.cs.hlsl:cs_main");
        assert_eq!(u.path, "/tgsr.cs.hlsl");
        assert_eq!(u.stem, "tgsr.cs");
        assert_eq!(u.entry, Some("cs_main"));
    }

    #[test]
    fn parse_without_entry_and_dir() {
        let u = ShaderUri::parse("/light_binning/cell_prefix_sum.cs.hlsl");
        assert_eq!(u.path, "/light_binning/cell_prefix_sum.cs.hlsl");
        assert_eq!(u.stem, "cell_prefix_sum.cs");
        assert_eq!(u.entry, None);
        assert_eq!(ShaderUri::parse("a.b").stem, "a");
    }

    #[test]
    #[should_panic]
    fn parse_without_dot_panics() {
        ShaderUri::parse("/dir/noext:main");
    }

    #[test]
    fn defines_default_to_empty_and_with_define_formats_display() {
        assert!("/a.cs.hlsl".pipeline_defines().is_empty());
        let d = compute_pipeline_from_uri("/a.cs.hlsl")
            .with_define("PERM_write", 1u32)
            .with_define("PERM_x", "s");
        assert_eq!(
            d.defines,
            vec![
                ("PERM_write".to_owned(), "1".to_owned()),
                ("PERM_x".to_owned(), "s".to_owned())
            ]
        );
    }

    #[test]
    fn desc_defaults_entry() {
        let d = compute_pipeline_from_uri("/taa.cs.hlsl");
        assert_eq!(d.entry, "cs_main");
        let d = compute_pipeline_from_uri("/x.cs.hlsl:other");
        assert_eq!((d.path.as_str(), d.entry.as_str()), ("/x.cs.hlsl", "other"));
    }

    #[test]
    fn hash_is_siphash13_key0() {
        // 与手写 DefaultHasher（SipHash-1-3, k=0）路径一致：str 字节 + 0xff。
        use core::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        h.write(b"/a.cs.hlsl");
        h.write_u8(0xff);
        assert_eq!(ComputeShaderSourceHash::compute("/a.cs.hlsl"), h.finish());
        let _ = <() as Hash>::hash::<std::collections::hash_map::DefaultHasher>;
    }
}
