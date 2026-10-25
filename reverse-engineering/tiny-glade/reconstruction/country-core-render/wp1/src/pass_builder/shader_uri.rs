//! shader_uri.rs — ShaderUri 解析与 AsComputeShaderSource trait。
//!
//! 机器码锚点：
//! - `ShaderUri::parse` 0x140968960（48B，3 个 &str 切片）。
//! - `impl$1::source_hash<&str>` 0x1408a4300（37B）。
//! - `impl$1::as_compute_pipeline<&str>` 0x1408a4330；`<String>` module15 rva 355632。
//! - 18 个游戏实现类型（DoF/nani/bin_lights/taa/vermeer 等）见 contracts/shader_uri.md §3。

use core::fmt;

/// 解析后的 shader URI（48B = 3×16B &str 对）。
/// URI 格式：`/<path>/<name>.(cs|raster).hlsl:<entry>`。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderUri<'a> {
    /// 整串（截断到最后一个 `:`）。
    pub whole: &'a str,
    /// 最后一个 `/` 之后、最后一个 `.` 之前的名字段（不含扩展名）。
    pub name: &'a str,
    /// 最后一个 `:` 之后的入口段。
    pub entry: &'a str,
}

impl<'a> ShaderUri<'a> {
    /// 依次按最后 `:`、最后 `/`、最后 `.` 切分；名字段无 `.` 时 `unwrap_failed`。
    /// 对应 0x140968960（三个 memrchr 循环 + slice_error_fail / unwrap_failed 分支）。
    pub fn parse(uri: &'a str) -> ShaderUri<'a> {
        let (whole, entry) = split_last(uri, ':')
            .unwrap_or_else(|| core::panicking::panic("no ':' in shader uri"));
        let (name, _ext) = split_last(whole, '/').unwrap();
        let (name_no_ext, _dot) = split_last(name, '.').expect("shader uri has no '.' in name");
        ShaderUri {
            whole,
            name: name_no_ext,
            entry,
        }
    }

    /// 完整 URI（含 entry），供 Debug/Display。
    pub fn full(&self) -> &'a str {
        self.whole
    }
}

/// 最后一个分隔符切分辅助（对应 memrchr 行为；`whole` 不含分隔符本身）。
#[inline]
fn split_last(s: &str, sep: char) -> Option<(&str, &str)> {
    let idx = s.rfind(sep)?;
    // 名字段必须非空（观测到 slice_error_fail 分支）。
    Some((&s[..idx], &s[idx + sep.len_utf8()..]))
}

impl fmt::Debug for ShaderUri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // impl$18::fmt 0x140939420（行 214）：输出完整 URI。
        write!(f, "{}", self.whole)
    }
}

impl fmt::Display for ShaderUri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.whole)
    }
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
    pub fn compute(_source: &str) -> u64 {
        // 算法未恢复（typed gap）：由 utils::calculate_hash 承担。
        crate::render::utils::calculate_hash(_source)
    }
}

/// 声明一个 compute pass 的 shader 来源。
/// `source_hash`（行 48）与 `as_compute_pipeline`（行 52）对应 trait 两方法。
/// 18 个游戏实现类型 = compute pass 清单（contracts/shader_uri.md §3）。
pub trait AsComputeShaderSource {
    fn source_hash(&self) -> u64;
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_>;
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

impl AsComputeShaderSource for alloc::string::String {
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
}
