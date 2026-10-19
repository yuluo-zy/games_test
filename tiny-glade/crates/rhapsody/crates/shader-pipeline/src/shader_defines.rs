//! Shader define lists. The artifact file key XORs the std `DefaultHasher`
//! (SipHash-1-3, key 0,0) hash of every `(name, value)` pair, starting from 0;
//! it matches the shipped shader artifact file names (see the tests). Since
//! equal pairs cancel, the build entry (shader-compiler `prepare_file`)
//! rejects duplicate names with `duplicate_name`.
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
    /// First define name that occurs twice. Repeated pairs cancel in the XOR key
    /// and differing values redefine a macro; both are rejected before packing.
    pub fn duplicate_name(&self) -> Option<&str> {
        let mut seen = std::collections::BTreeSet::new();
        self.0
            .iter()
            .map(|(name, _)| name.as_str())
            .find(|name| !seen.insert(*name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defines(pairs: &[(&str, &str)]) -> ShaderDefines {
        ShaderDefines(
            pairs
                .iter()
                .map(|(a, b)| ((*a).into(), (*b).into()))
                .collect(),
        )
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
        // The XOR fold does not deduplicate: a repeated pair cancels.
        assert_eq!(defines(&[("A", "1"), ("A", "1")]).artifact_hash(), 0);
    }

    #[test]
    fn duplicate_names_are_reported_without_touching_the_original_key() {
        assert_eq!(ShaderDefines::default().duplicate_name(), None);
        assert_eq!(defines(&[("A", "1"), ("B", "1")]).duplicate_name(), None);
        assert_eq!(
            defines(&[("A", "1"), ("A", "1")]).duplicate_name(),
            Some("A")
        );
        assert_eq!(
            defines(&[("B", "0"), ("A", "1"), ("B", "1")]).duplicate_name(),
            Some("B")
        );
    }

    /// Multi-define samples: keys of shipped compute `.bin` file names.
    #[test]
    fn shipped_multi_define_compute_keys_reproduce() {
        let cases: &[(&[(&str, &str)], u64)] = &[
            (&[("PERM_quality", "0")], 0x6f1d_d5dd_8a66_e104),
            (&[("PERM_quality", "1")], 0x72a2_a95d_73e3_7edc),
            (&[("PERM_oc_pass", "1")], 0x8a7d_2f81_e72c_8eeb),
            (&[("PERM_oc_pass", "0")], 0xd852_0a92_ddeb_65ab),
            (
                &[("PERM_fmt", "\"rgba32f\""), ("PERM_ty", "float4")],
                0xb9dd_0465_835c_d591,
            ),
            (
                &[("PERM_fmt", "\"r16\""), ("PERM_ty", "float")],
                0xdc5f_d7e4_b9bf_9935,
            ),
            (
                &[("PERM_wave32", "0"), ("PERM_sharpen", "1")],
                0x3ee6_a953_408e_4c4c,
            ),
            (
                &[("PERM_wave32", "0"), ("PERM_sharpen", "0")],
                0x7a0a_f184_50cd_bcd1,
            ),
            (
                &[("PERM_wave32", "1"), ("PERM_sharpen", "0")],
                0xb303_f36f_0dbd_d5f6,
            ),
            (
                &[("PERM_wave32", "1"), ("PERM_sharpen", "1")],
                0xf7ef_abb8_1dfe_256b,
            ),
        ];
        for (pairs, expected) in cases {
            assert_eq!(defines(pairs).artifact_hash(), *expected, "{pairs:?}");
            // The fold is order independent: reversed input gives the same key.
            let mut rev = pairs.to_vec();
            rev.reverse();
            assert_eq!(
                defines(&rev).artifact_hash(),
                *expected,
                "reversed {pairs:?}"
            );
        }
    }
}
