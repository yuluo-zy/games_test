//! cull_shaders.rs — nani 剔除/分桶 compute pass 结构体（B 阶段按机器码重写）。
//!
//! 机器码锚点（B patched EXE，cf048e27…）——9 个 `AsComputeShaderSource` 实现，
//! 每个 `as_compute_pipeline`（~1131/1147 B）+ 部分 `source_hash`（46 B）：
//!
//! | 结构体 | `impl$N` | `as_compute_pipeline` | `source_hash` | 路径（O，长度来自 `mov ecx,N`） |
//! |---|---|---|---|---|
//! | Main | 10 | 0x141623120 | （ICF/内联） | `/nani/cull_and_bucket.cs.hlsl`（29） |
//! | RoofTile | 14 | 0x141623590 | — | `..._roof_tile.cs.hlsl`（39） |
//! | Flowers | 18 | 0x141623a10 | — | `..._flowers.cs.hlsl`（37） |
//! | Bush | 22 | 0x141623e90 | — | `..._bush.cs.hlsl`（34） |
//! | WoodenPlank | 26 | 0x141624300 | — | `..._wooden_plank.cs.hlsl`（42） |
//! | CurvedHalftimber | 30 | 0x1416247a0 | 0x141624770 | `..._curved_halftimber.cs.hlsl`（47） |
//! | Plaster | 34 | 0x141624c40 | 0x141624c10 | `..._plaster.cs.hlsl`（37） |
//! | Smoke | 38 | 0x1416250f0 | 0x1416250c0 | `..._smoke.cs.hlsl`（35） |
//! | StairsFloor | 42 | 0x141625590 | 0x141625560 | `..._stairs_floor.cs.hlsl`（42） |
//!
//! 全部九个 `as_compute_pipeline` 同构（O）：
//! `ComputePipelineDesc { path: <上表路径>, entry: "cs_main", defines: [("PERM_oc_pass", self.to_string())] }`，
//! 其中 `self` 为 4 字节整数（`core::fmt::num::imp::impl$7::fmt` = `i32` Display，`neg/cmovs`；
//! 同址 `source_hash` 对 `&self` 的 4 字节做 `Hash`）。
//! `source_hash`（O）= `utils::calculate_hash(&("<path>:cs_main", &self))`，
//! 即 SipHash-1-3（零密钥）依次写：字符串字节、`0xff`、4 字节整数。
//!
//! PDB 中 `ComputePass::new<NaniCullAndBucket*>`（×5）、`dispatch_with`、`create_cull_and_bucket_pass`、
//! `cull_and_bucket_external` 均属 `systems::render_loop::compute_passes::gpu_generate_draw_lists`
//! 与 `pass_builder`，不在本文件（由各自所有者恢复）。此前文件里的 `cull_and_bucket_for_kind` /
//! `group_draw_cmd_prefix_sum` 两个占位函数无机器码对应（PDB 不存在该符号），已删除；
//! kind→shader 的映射改为 `NaniCullAndBucketFamily::for_kind`（I）。

#![allow(dead_code, unused_imports)]

use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::render::pass_builder::shader_uri::ComputePipelineDesc;

/// 入口名（O：`as_compute_pipeline` 写入 7 字节 `cs_main`，立即数 `0x6e69616d`/`0x6d5f7363`）。
pub const CULL_ENTRY: &str = "cs_main";
/// 编译期特性键（O：12 字节 `PERM_oc_pass`，立即数 `0x5f636f5f4d524550` + `0x73736170`）。
pub const PERM_OC_PASS: &str = "PERM_oc_pass";

/// cull/bucket 着色器族（O：9 个）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NaniCullAndBucketFamily {
    Main,
    RoofTile,
    Flowers,
    Bush,
    WoodenPlank,
    CurvedHalftimber,
    Plaster,
    Smoke,
    StairsFloor,
}

impl NaniCullAndBucketFamily {
    pub const ALL: [NaniCullAndBucketFamily; 9] = [
        Self::Main,
        Self::RoofTile,
        Self::Flowers,
        Self::Bush,
        Self::WoodenPlank,
        Self::CurvedHalftimber,
        Self::Plaster,
        Self::Smoke,
        Self::StairsFloor,
    ];

    /// 着色器路径（O：`/nani/cull_and_bucket[_<name>].cs.hlsl`）。
    pub const fn path(self) -> &'static str {
        match self {
            Self::Main => "/nani/cull_and_bucket.cs.hlsl",
            Self::RoofTile => "/nani/cull_and_bucket_roof_tile.cs.hlsl",
            Self::Flowers => "/nani/cull_and_bucket_flowers.cs.hlsl",
            Self::Bush => "/nani/cull_and_bucket_bush.cs.hlsl",
            Self::WoodenPlank => "/nani/cull_and_bucket_wooden_plank.cs.hlsl",
            Self::CurvedHalftimber => "/nani/cull_and_bucket_curved_halftimber.cs.hlsl",
            Self::Plaster => "/nani/cull_and_bucket_plaster.cs.hlsl",
            Self::Smoke => "/nani/cull_and_bucket_smoke.cs.hlsl",
            Self::StairsFloor => "/nani/cull_and_bucket_stairs_floor.cs.hlsl",
        }
    }

    /// `source_hash` 里被哈希的字符串 `"<path>:cs_main"`（O：`rax=str, len=0x37/0x2f+8`）。
    pub fn source_uri(self) -> String {
        format!("{}:{}", self.path(), CULL_ENTRY)
    }

    /// `GpuWorldSubsetKind` → 着色器族（I：族名与 kind 真名一一对应——Bush/CurvedHalfTimber/Flower/Plaster/
    /// RoofTile/Smoke/StairsFloor/WoodenPlank；其余 19 个 kind 使用 Main。`PreplacedFlower` 归 Main 为 I）。
    pub fn for_kind(kind: GpuWorldSubsetKind) -> Self {
        match kind {
            GpuWorldSubsetKind::Bush => Self::Bush,
            GpuWorldSubsetKind::CurvedHalfTimber => Self::CurvedHalftimber,
            GpuWorldSubsetKind::Flower => Self::Flowers,
            GpuWorldSubsetKind::Plaster => Self::Plaster,
            GpuWorldSubsetKind::RoofTile => Self::RoofTile,
            GpuWorldSubsetKind::Smoke => Self::Smoke,
            GpuWorldSubsetKind::StairsFloor => Self::StairsFloor,
            GpuWorldSubsetKind::WoodenPlank => Self::WoodenPlank,
            _ => Self::Main,
        }
    }
}

/// 公共实现：`as_compute_pipeline`（O）。
fn compute_pipeline(family: NaniCullAndBucketFamily, oc_pass: i32) -> ComputePipelineDesc {
    ComputePipelineDesc {
        path: family.path().to_owned(),
        entry: CULL_ENTRY.to_owned(),
        defines: vec![(PERM_OC_PASS.to_owned(), oc_pass.to_string())],
    }
}

/// 公共实现：`source_hash`（O，SipHash-1-3 零密钥；与 `utils::calculate_hash` 一致）。
fn source_hash(family: NaniCullAndBucketFamily, oc_pass: i32) -> u64 {
    crate::render::utils::calculate_hash(&(family.source_uri().as_str(), &oc_pass))
}

/// 单个 shader 结构体的统一接口（`AsComputeShaderSource` 的两个方法；A4 `shader_uri` 的
/// trait 当前返回 `ComputeShaderSource`，与机器码返回的 `ComputePipelineDesc` 不一致，
/// 因此此处给出同名自有方法，集成者在 A4 重写完成后改为实现其 trait）。
pub trait NaniCullShader {
    const FAMILY: NaniCullAndBucketFamily;
    /// `PERM_oc_pass` 的值（O：struct 的唯一 4 字节字段）。
    fn oc_pass(&self) -> i32;

    /// `as_compute_pipeline`（O）。
    fn as_compute_pipeline(&self) -> ComputePipelineDesc {
        compute_pipeline(Self::FAMILY, self.oc_pass())
    }
    /// `source_hash`（O）。
    fn source_hash(&self) -> u64 {
        source_hash(Self::FAMILY, self.oc_pass())
    }
}

macro_rules! cull_shader {
    ($(#[$m:meta])* $name:ident, $family:ident) => {
        $(#[$m])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
        pub struct $name {
            /// `PERM_oc_pass`（O：`i32` Display 格式化）。字段名 I。
            pub oc_pass: i32,
        }
        impl NaniCullShader for $name {
            const FAMILY: NaniCullAndBucketFamily = NaniCullAndBucketFamily::$family;
            fn oc_pass(&self) -> i32 {
                self.oc_pass
            }
        }
    };
}

cull_shader!(
    /// 通用主实例（impl$10，0x141623120）。
    NaniCullAndBucketMain, Main
);
cull_shader!(
    /// 屋顶瓦（impl$14，0x141623590）。
    NaniCullAndBucketRoofTile, RoofTile
);
cull_shader!(
    /// 花（impl$18，0x141623a10）。
    NaniCullAndBucketFlowers, Flowers
);
cull_shader!(
    /// 灌木（impl$22，0x141623e90）。
    NaniCullAndBucketBush, Bush
);
cull_shader!(
    /// 木板（impl$26，0x141624300）。
    NaniCullAndBucketWoodenPlank, WoodenPlank
);
cull_shader!(
    /// 弯曲半木（impl$30，0x1416247a0 / source_hash 0x141624770）。
    NaniCullAndBucketCurvedHalftimber, CurvedHalftimber
);
cull_shader!(
    /// 石膏（impl$34，0x141624c40 / 0x141624c10）。
    NaniCullAndBucketPlaster, Plaster
);
cull_shader!(
    /// 烟（impl$38，0x1416250f0 / 0x1416250c0）。
    NaniCullAndBucketSmoke, Smoke
);
cull_shader!(
    /// 楼梯地板（impl$42，0x141625590 / 0x141625560）。
    NaniCullAndBucketStairsFloor, StairsFloor
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_lengths_match_machine_code_allocations() {
        // `mov ecx, N` 传给 `__rust_alloc` 的路径长度（O）。
        let expected = [29usize, 39, 37, 34, 42, 47, 37, 35, 42];
        let order = [
            NaniCullAndBucketFamily::Main,
            NaniCullAndBucketFamily::RoofTile,
            NaniCullAndBucketFamily::Flowers,
            NaniCullAndBucketFamily::Bush,
            NaniCullAndBucketFamily::WoodenPlank,
            NaniCullAndBucketFamily::CurvedHalftimber,
            NaniCullAndBucketFamily::Plaster,
            NaniCullAndBucketFamily::Smoke,
            NaniCullAndBucketFamily::StairsFloor,
        ];
        for (f, n) in order.iter().zip(expected) {
            assert_eq!(f.path().len(), n, "{f:?}");
        }
    }

    #[test]
    fn source_uri_lengths_match_source_hash_immediates() {
        // source_hash 里 `mov qword ptr [rsp+0x28], len`：curved_halftimber = 0x37。
        assert_eq!(
            NaniCullAndBucketFamily::CurvedHalftimber.source_uri().len(),
            0x37
        );
        assert_eq!(
            NaniCullAndBucketFamily::CurvedHalftimber.source_uri(),
            "/nani/cull_and_bucket_curved_halftimber.cs.hlsl:cs_main"
        );
    }

    #[test]
    fn pipeline_desc_shape() {
        let d = NaniCullAndBucketSmoke { oc_pass: 1 }.as_compute_pipeline();
        assert_eq!(d.path, "/nani/cull_and_bucket_smoke.cs.hlsl");
        assert_eq!(d.entry, "cs_main");
        assert_eq!(
            d.defines,
            vec![("PERM_oc_pass".to_string(), "1".to_string())]
        );
        // i32 Display：负数带符号（neg/cmovs 路径）。
        let d = NaniCullAndBucketMain { oc_pass: -3 }.as_compute_pipeline();
        assert_eq!(d.defines[0].1, "-3");
        assert_eq!(PERM_OC_PASS.len(), 12);
        assert_eq!(CULL_ENTRY.len(), 7);
    }

    #[test]
    fn source_hash_depends_on_family_and_pass() {
        let a = NaniCullAndBucketPlaster { oc_pass: 0 }.source_hash();
        let b = NaniCullAndBucketPlaster { oc_pass: 1 }.source_hash();
        let c = NaniCullAndBucketSmoke { oc_pass: 0 }.source_hash();
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_eq!(a, NaniCullAndBucketPlaster { oc_pass: 0 }.source_hash());
    }

    #[test]
    fn kind_to_family() {
        assert_eq!(
            NaniCullAndBucketFamily::for_kind(GpuWorldSubsetKind::Flower),
            NaniCullAndBucketFamily::Flowers
        );
        assert_eq!(
            NaniCullAndBucketFamily::for_kind(GpuWorldSubsetKind::Canopy),
            NaniCullAndBucketFamily::Main
        );
        assert_eq!(
            NaniCullAndBucketFamily::for_kind(GpuWorldSubsetKind::SolidVertexColor),
            NaniCullAndBucketFamily::Main
        );
    }
}
