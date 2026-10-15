//! 原定义列表保留重复项；文件键将每个字符串对的 DefaultHasher 结果异或。
//!
//! 证据（O）：
//! - 机器码 0x141795d10（compute 键）与 0x141796080（raster 键）：定义 Vec 元素 stride 0x30（一对 `String`），
//!   循环 `acc ^= hash_pair(elem)`（调用 0x1420cfbf0，0x141795d60..0x141795d73 / 0x1417960d0..0x1417960e3），初值 0；
//! - 逐对哈希是 std `DefaultHasher`（SipHash-1-3，key 0,0）对 `(String,String)` 元组：对发行文件名
//!   `_billboard_forest.raster.hlsl_b903e43ffb3da915.bin` / `_f0adff76827e73ff.bin`，输入
//!   `("SHADER_VARIANT_DEPTH_ONLY","0")` / `("…","1")`（`raster_shader_variants` 0x1417388a0 追加的宏，
//!   名字串 0x142ccd739）数值逐字复现（见下方测试）。旧文档“FNV/SipHash 暴力未复现”是因为输入宏名未知。
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShaderDefines(pub Vec<(String, String)>);
impl ShaderDefines {
    /// 不排序、不去重；空列表为零，同一对出现两次会抵消。
    pub fn artifact_hash(&self) -> u64 {
        self.0.iter().fold(0, |hash, pair| {
            let mut hasher = DefaultHasher::new();
            pair.hash(&mut hasher);
            hash ^ hasher.finish()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defines(pairs: &[(&str, &str)]) -> ShaderDefines {
        ShaderDefines(pairs.iter().map(|(a, b)| ((*a).into(), (*b).into())).collect())
    }

    /// 发行 404 个 .bin 里 raster 只有这两个哈希，均由单个 depth-only 宏产生。
    #[test]
    fn depth_only_macro_reproduces_the_shipped_file_name_hashes() {
        assert_eq!(
            defines(&[("SHADER_VARIANT_DEPTH_ONLY", "0")]).artifact_hash(),
            0xb903_e43f_fb3d_a915
        );
        assert_eq!(
            defines(&[("SHADER_VARIANT_DEPTH_ONLY", "1")]).artifact_hash(),
            0xf0ad_ff76_827e_73ff
        );
    }

    #[test]
    fn empty_is_zero_and_pairs_are_xor_folded() {
        assert_eq!(ShaderDefines::default().artifact_hash(), 0);
        let a = defines(&[("A", "1")]).artifact_hash();
        let b = defines(&[("B", "2")]).artifact_hash();
        assert_eq!(defines(&[("A", "1"), ("B", "2")]).artifact_hash(), a ^ b);
        // 原版 XOR 折叠不去重：同一对出现两次互相抵消。
        assert_eq!(defines(&[("A", "1"), ("A", "1")]).artifact_hash(), 0);
    }

    /// 多宏样本：发行 compute `.bin` 文件名里的键与 `original-compute-tuples.json`（原 producer 的 defines 列表，
    /// 0x140a5de30 等）逐项对拍。含 4 组 ≥2 个宏的组合（gauss_blur 两种、tgsr 四种）+ 单宏对照。
    #[test]
    fn shipped_multi_define_compute_keys_reproduce() {
        let cases: &[(&[(&str, &str)], u64)] = &[
            (&[("PERM_quality", "0")], 0x6f1d_d5dd_8a66_e104),
            (&[("PERM_quality", "1")], 0x72a2_a95d_73e3_7edc),
            (&[("PERM_oc_pass", "1")], 0x8a7d_2f81_e72c_8eeb),
            (&[("PERM_oc_pass", "0")], 0xd852_0a92_ddeb_65ab),
            (&[("PERM_fmt", "\"rgba32f\""), ("PERM_ty", "float4")], 0xb9dd_0465_835c_d591),
            (&[("PERM_fmt", "\"r16\""), ("PERM_ty", "float")], 0xdc5f_d7e4_b9bf_9935),
            (&[("PERM_wave32", "0"), ("PERM_sharpen", "1")], 0x3ee6_a953_408e_4c4c),
            (&[("PERM_wave32", "0"), ("PERM_sharpen", "0")], 0x7a0a_f184_50cd_bcd1),
            (&[("PERM_wave32", "1"), ("PERM_sharpen", "0")], 0xb303_f36f_0dbd_d5f6),
            (&[("PERM_wave32", "1"), ("PERM_sharpen", "1")], 0xf7ef_abb8_1dfe_256b),
        ];
        for (pairs, expected) in cases {
            assert_eq!(defines(pairs).artifact_hash(), *expected, "{pairs:?}");
            // 折叠与顺序无关：反序给出同一键，所以原版不需要先排序。
            let mut rev = pairs.to_vec();
            rev.reverse();
            assert_eq!(defines(&rev).artifact_hash(), *expected, "reversed {pairs:?}");
        }
    }
}
