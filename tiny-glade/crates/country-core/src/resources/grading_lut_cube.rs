//! `.cube` 调色 LUT 解析（`GradingLut::load_cube` 0x140b01050 的纯逻辑部分，不触设备）。
//!
//! 证据（O，反编译 + 机器码逐条核对；字符串取自原 EXE）：
//!  * 文件按 `fs::read_to_string` 读入（0x140d59140）；按 `str::lines` 切行（只在有 `\n` 时才剥 `\r`），
//!    行号从 1 起、**每一行都计数**（含空行/注释）。
//!  * 每行先 `str::trim`（0x1400f9c30，**反编译器把返回的长度丢了**，机器码里 `rdx` 是 trim 后长度，
//!    后续所有长度检查都用它）。空行、`#` 开头 → 跳过。
//!  * 依次判断前缀：`TITLE`（跳过）→ `DOMAIN_MIN` → `DOMAIN_MAX` → `LUT_1D_SIZE` → `LUT_3D_SIZE` → 其余当数据行。
//!    `DOMAIN_*` 要求长度 ≥ 10，`LUT_*_SIZE` 要求长度 ≥ 11，较短的行直接当数据行。
//!  * `DOMAIN_MIN`：`parse_rgb(line[10..])`，失败 → context `"Parsing DOMAIN_MIN"`；三分量都 `== 0.0`（含 `-0.0`，NaN 不算）
//!    才合法，否则 `"DOMAIN_MIN must be 0 0 0"`。`DOMAIN_MAX` 同理，三分量都 `== 1.0`，`"DOMAIN_MAX must be 1 1 1"`。
//!  * `LUT_1D_SIZE` → `"1D LUTs are not supported"`（无 context）。
//!  * `LUT_3D_SIZE`：`line[11..].trim()` 经 `u64::from_str`（内联），失败 → context `"Parsing LUT_3D_SIZE"`；
//!    成功后 `texels.resize(size³, 0)`（`size³` 为**回绕乘**，release 无溢出检查）。初始 `size = 0x1f`。
//!  * 数据行：`parse_rgb(line)`，失败 → context `format!("Parsing line {n}: {line}")`（`line` 为 trim 后）；
//!    `idx + 1 > texels.len()` → `"Too many entries in the LUT. Bailing before an overrun"`；
//!    否则 `texels[idx] = pack_rgb9e5(rgb)`（`utils::rgb9e5::pack_rgb9e5` 0x1417c66b0），`idx += 1`。
//!  * 行数不足 `size³` **不报错**：剩余 texel 保持 0。此前 DIGEST 写成“越界 panic（0x1428d9518）”是误读——
//!    那是写入前的冗余下标检查，已被上一条 `idx + 1 > len` 挡掉，不可达。
//!  * 纹理：`TextureDesc::new_3d([size as u32; 3], 0x7b).sampled()`，名字 `"grading lut"`，字节 = texels 的小端序列。
//!    这部分在 `grading_lut.rs::GradingLut::load_cube`（需要设备）。
//!
//! 偏离/U：
//!  * 原版 `size³` 回绕后巨大时直接走分配失败；这里同样交给 `Vec::resize`（容量溢出 panic / 分配失败）。
//!  * 分词器 `0x140b00920` 的 PDB 名是 `str::iter::next`；按“空白切分”的语义取 `split_whitespace`（I，
//!    未逐字节核对其 Unicode 空白表是否与 std 的 `char::is_whitespace` 完全相同）。

use anyhow::{Context, Result, anyhow};
use glam::Vec3;

use utils::rgb9e5::pack_rgb9e5;

/// `LUT_3D_SIZE` 缺省值（O：`uStack_70 = 0x1f`）。
pub const DEFAULT_SIZE: u64 = 0x1f;
/// 3D 纹理调试名（O：0x142ae6c70，11 B）。
pub const TEXTURE_NAME: &str = "grading lut";
/// 纹理格式（O：`new_3d(.., 0x7b)`，即 `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`）。
pub const TEXTURE_FORMAT: u32 = 0x7b;

/// 解析结果：立方体边长与 `size³` 个 RGB9E5 texel。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CubeLut {
    pub size: u64,
    pub texels: Vec<u32>,
}

impl CubeLut {
    /// 上传用字节：texel 按小端依次展开（`size³ × 4` 字节）。
    pub fn texel_bytes(&self) -> Vec<u8> {
        self.texels.iter().flat_map(|t| t.to_le_bytes()).collect()
    }
}

/// `impl$2::load_cube::parse_rgb`（O 0x140b01cc0）：空白切分，逐个取词并立刻 `f32::from_str`
///（0x140d8ba40）。词不够 → `"Invalid RGB triplet"`；解析失败 → 该 `ParseFloatError`。多余的词被忽略。
pub fn parse_rgb(line: &str) -> Result<[f32; 3]> {
    let mut words = line.split_whitespace();
    let mut next = || -> Result<f32> {
        let word = words.next().ok_or_else(|| anyhow!("Invalid RGB triplet"))?;
        Ok(word.parse::<f32>()?)
    };
    Ok([next()?, next()?, next()?])
}

/// `load_cube` 的解析主体（见文件头）。
pub fn parse_cube(text: &str) -> Result<CubeLut> {
    let mut size = DEFAULT_SIZE;
    let mut texels: Vec<u32> = Vec::new();
    let mut idx = 0usize;
    for (i, raw) in text.lines().enumerate() {
        let line_no = i + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("TITLE") {
            continue;
        }
        if let Some(rest) = line.strip_prefix("DOMAIN_MIN") {
            let v = parse_rgb(rest).context("Parsing DOMAIN_MIN")?;
            if v.iter().all(|c| *c == 0.0) {
                continue;
            }
            return Err(anyhow!("DOMAIN_MIN must be 0 0 0"));
        }
        if let Some(rest) = line.strip_prefix("DOMAIN_MAX") {
            let v = parse_rgb(rest).context("Parsing DOMAIN_MAX")?;
            if v.iter().all(|c| *c == 1.0) {
                continue;
            }
            return Err(anyhow!("DOMAIN_MAX must be 1 1 1"));
        }
        if line.starts_with("LUT_1D_SIZE") {
            return Err(anyhow!("1D LUTs are not supported"));
        }
        if let Some(rest) = line.strip_prefix("LUT_3D_SIZE") {
            size = rest.trim().parse::<u64>().context("Parsing LUT_3D_SIZE")?;
            let n = size.wrapping_mul(size).wrapping_mul(size) as usize;
            texels.resize(n, 0);
            continue;
        }
        let rgb = parse_rgb(line).with_context(|| format!("Parsing line {line_no}: {line}"))?;
        if texels.len() < idx + 1 {
            return Err(anyhow!(
                "Too many entries in the LUT. Bailing before an overrun"
            ));
        }
        texels[idx] = pack_rgb9e5(Vec3::from(rgb));
        idx += 1;
    }
    Ok(CubeLut { size, texels })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn err_chain(e: &anyhow::Error) -> String {
        e.chain()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    }

    #[test]
    fn parse_rgb_takes_three_words_and_ignores_extra() {
        assert_eq!(parse_rgb("  0.5\t1  -2.25 ").unwrap(), [0.5, 1.0, -2.25]);
        assert_eq!(parse_rgb("1 2 3 4 5").unwrap(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn parse_rgb_errors() {
        assert_eq!(
            parse_rgb("1 2").unwrap_err().to_string(),
            "Invalid RGB triplet"
        );
        assert_eq!(
            parse_rgb("").unwrap_err().to_string(),
            "Invalid RGB triplet"
        );
        // 第一个词就非法：在检查第 2 个词之前失败（原版逐词解析）。
        assert_eq!(
            parse_rgb("x").unwrap_err().to_string(),
            "invalid float literal"
        );
    }

    #[test]
    fn identity_2x2x2_cube() {
        let text = "TITLE \"t\"\n# c\nDOMAIN_MIN 0 0 0\nDOMAIN_MAX 1 1 1\nLUT_3D_SIZE 2\n\
                    0 0 0\n1 0 0\n0 1 0\n1 1 0\n0 0 1\n1 0 1\n0 1 1\n1 1 1\n";
        let lut = parse_cube(text).unwrap();
        assert_eq!(lut.size, 2);
        assert_eq!(lut.texels.len(), 8);
        for (i, t) in lut.texels.iter().enumerate() {
            let (r, g, b) = ((i & 1) as f32, ((i >> 1) & 1) as f32, ((i >> 2) & 1) as f32);
            assert_eq!(*t, pack_rgb9e5(Vec3::new(r, g, b)), "texel {i}");
        }
        assert_eq!(lut.texel_bytes().len(), 32);
        assert_eq!(&lut.texel_bytes()[..4], &lut.texels[0].to_le_bytes());
    }

    #[test]
    fn crlf_and_indentation_and_missing_final_newline() {
        let text = "  LUT_3D_SIZE   2 \r\n\t0 0 0\r\n1 1 1";
        let lut = parse_cube(text).unwrap();
        assert_eq!(lut.size, 2);
        assert_eq!(lut.texels[0], pack_rgb9e5(Vec3::ZERO));
        assert_eq!(lut.texels[1], pack_rgb9e5(Vec3::ONE));
        assert_eq!(&lut.texels[2..], &[0; 6]);
    }

    #[test]
    fn short_data_leaves_zero_texels_without_error() {
        let lut = parse_cube("LUT_3D_SIZE 2\n1 1 1\n").unwrap();
        assert_eq!(lut.texels.len(), 8);
        assert_ne!(lut.texels[0], 0);
        assert!(lut.texels[1..].iter().all(|t| *t == 0));
    }

    #[test]
    fn data_before_size_is_too_many_entries() {
        let e = parse_cube("1 1 1\n").unwrap_err();
        assert_eq!(
            e.to_string(),
            "Too many entries in the LUT. Bailing before an overrun"
        );
    }

    #[test]
    fn too_many_entries() {
        let e = parse_cube("LUT_3D_SIZE 1\n1 1 1\n0 0 0\n").unwrap_err();
        assert_eq!(
            e.to_string(),
            "Too many entries in the LUT. Bailing before an overrun"
        );
    }

    #[test]
    fn default_size_is_31_when_never_declared() {
        let lut = parse_cube("# nothing\n").unwrap();
        assert_eq!(lut.size, 0x1f);
        assert!(lut.texels.is_empty());
    }

    #[test]
    fn domain_checks() {
        assert!(parse_cube("DOMAIN_MIN 0 0 0\nDOMAIN_MAX 1 1 1\n").is_ok());
        assert!(parse_cube("DOMAIN_MIN -0.0 0 0\n").is_ok());
        assert_eq!(
            parse_cube("DOMAIN_MIN 0 0 0.1\n").unwrap_err().to_string(),
            "DOMAIN_MIN must be 0 0 0"
        );
        assert_eq!(
            parse_cube("DOMAIN_MAX 1 1 2\n").unwrap_err().to_string(),
            "DOMAIN_MAX must be 1 1 1"
        );
        let e = parse_cube("DOMAIN_MIN 0 0\n").unwrap_err();
        assert_eq!(err_chain(&e), "Parsing DOMAIN_MIN | Invalid RGB triplet");
        let e = parse_cube("DOMAIN_MAX a b c\n").unwrap_err();
        assert_eq!(err_chain(&e), "Parsing DOMAIN_MAX | invalid float literal");
        // NaN 不等于 0。
        assert!(parse_cube("DOMAIN_MIN nan 0 0\n").is_err());
    }

    #[test]
    fn one_d_luts_rejected() {
        let e = parse_cube("LUT_1D_SIZE 16\n").unwrap_err();
        assert_eq!(err_chain(&e), "1D LUTs are not supported");
    }

    #[test]
    fn lut_size_parse_errors_carry_context() {
        let e = parse_cube("LUT_3D_SIZE x\n").unwrap_err();
        assert_eq!(
            err_chain(&e),
            "Parsing LUT_3D_SIZE | invalid digit found in string"
        );
        let e = parse_cube("LUT_3D_SIZE\n").unwrap_err();
        assert_eq!(
            err_chain(&e),
            "Parsing LUT_3D_SIZE | cannot parse integer from empty string"
        );
        let e = parse_cube("LUT_3D_SIZE -3\n").unwrap_err();
        assert_eq!(
            err_chain(&e),
            "Parsing LUT_3D_SIZE | invalid digit found in string"
        );
        let e = parse_cube("LUT_3D_SIZE 99999999999999999999\n").unwrap_err();
        assert_eq!(
            err_chain(&e),
            "Parsing LUT_3D_SIZE | number too large to fit in target type"
        );
        // '+' 前缀合法。
        assert_eq!(parse_cube("LUT_3D_SIZE +1\n0 0 0\n").unwrap().size, 1);
    }

    #[test]
    fn bad_data_line_reports_line_number_and_trimmed_text() {
        let e = parse_cube("# a\n\nLUT_3D_SIZE 2\n   1 2 oops  \n").unwrap_err();
        assert_eq!(
            err_chain(&e),
            "Parsing line 4: 1 2 oops | invalid float literal"
        );
    }

    #[test]
    fn short_lines_are_data_lines_not_keywords() {
        // 长度 < 10 的行不会被当成 DOMAIN_*；TITLE 需要完整前缀。
        let e = parse_cube("TITL 1 2 3\n").unwrap_err();
        assert!(err_chain(&e).starts_with("Parsing line 1: TITL 1 2 3"));
    }

    #[test]
    fn resize_truncates_when_size_shrinks() {
        let lut = parse_cube("LUT_3D_SIZE 2\n1 1 1\nLUT_3D_SIZE 1\n").unwrap();
        assert_eq!((lut.size, lut.texels.len()), (1, 1));
    }
}
