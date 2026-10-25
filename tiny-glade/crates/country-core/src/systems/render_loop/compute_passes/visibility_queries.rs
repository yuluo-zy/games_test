//! `visibility_queries.rs` — 萤火虫可见性查询（W31，`run_visibility_queries` 0x140a2f2e0，904 B）。
//!
//! 机器码（O）：`ComputePass::new("Firefly vis query", "/critters/firefly_vis_query.cs.hlsl")`（名长 0x11、URI 长 0x23），
//! 然后依次：`depth_tex`（`&Handle`，读 0x19）→ `terrain_heightmap`（预设 9）→ `visual_terrain_heightmap`（预设 10）→
//! `read_storage_buffer<FireflySsbo>(firefly_buffer)` → `write_storage_buffer<FireflyVisibilitySsbo>(firefly_visibility_buffer)` →
//! `dispatch_1d(0x60)`（闭包状态常量 `{1, 0x60, 1, 1}`，0x142acad00）。无条件分支、无 world 资源读取。
//!
//! 两个 SSBO 类型属别的模块（`resources::critters::{FireflySsbo, FireflyVisibilitySsbo}`，其 `SsboType` 实现尚未恢复，NAME U），
//! 故这里作泛型参数（原版同样是单态化泛型）。SsboId 立即数（O）：Firefly `0x29d4a64d5ea39871`、
//! FireflyVisibility `0x9248897309ea4fea`。

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};

pub const NAME: &str = "Firefly vis query";
pub const URI: &str = "/critters/firefly_vis_query.cs.hlsl";
pub const FIREFLY_BUFFER: u64 = binding_hash("firefly_buffer");
pub const FIREFLY_VISIBILITY_BUFFER: u64 = binding_hash("firefly_visibility_buffer");
/// `dispatch_1d` 的线程总数（O：0x142acad00 = `{1, 0x60, 1, 1}`）。
pub const DISPATCH_THREADS: u32 = 0x60;

/// W31：萤火虫可见性 compute pass。`depth` = 调用方的深度句柄（`param_3`）。
pub fn run_visibility_queries<Firefly: SsboType, Visibility: SsboType>(
    env: &mut PassEnv<'_>,
    depth: &RgTexture,
) -> RecordedComputePass {
    env.pass(NAME, URI)
        .bind_texture(binding::DEPTH_TEX, &depth, 0x19)
        .bind_texture(binding::TERRAIN_HEIGHTMAP, &TexturePreset(9), 0x19)
        .bind_texture(binding::VISUAL_TERRAIN_HEIGHTMAP, &TexturePreset(10), 0x19)
        .read_storage_buffer::<Firefly>(FIREFLY_BUFFER)
        .write_storage_buffer::<Visibility>(FIREFLY_VISIBILITY_BUFFER)
        .dispatch_1d(DISPATCH_THREADS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env_with, rgtex, test_carrier, threads,
    };

    test_carrier!(Firefly, "Firefly");
    test_carrier!(FireflyVis, "FireflyVisibility");

    #[test]
    fn binding_names_match_exe_immediates() {
        assert_eq!(FIREFLY_BUFFER, 0x7d0b_7ff9_bdcd_8389);
        assert_eq!(FIREFLY_VISIBILITY_BUFFER, 0x66db_876b_0140_f6f2);
        assert_eq!(binding::TERRAIN_HEIGHTMAP, 0x61b0_8b5e_0470_c5f2);
        assert_eq!(binding::VISUAL_TERRAIN_HEIGHTMAP, 0x297d_1e4c_dae8_f657);
    }

    #[test]
    fn declares_reads_then_writes_and_dispatches_96() {
        let mut e = env_with(
            &[
                binding::DEPTH_TEX,
                binding::TERRAIN_HEIGHTMAP,
                binding::VISUAL_TERRAIN_HEIGHTMAP,
                FIREFLY_BUFFER,
                FIREFLY_VISIBILITY_BUFFER,
            ],
            &[],
            |lib, dev| {
                lib.add_new::<Firefly>(dev);
                lib.add_new::<FireflyVis>(dev);
            },
        );
        let depth = rgtex(77, 640, 360);
        let pass = e.run(|env| run_visibility_queries::<Firefly, FireflyVis>(env, &depth));
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(&pass), (0x60, 1, 1));
        let a = accesses(&pass);
        assert_eq!(a.len(), 5);
        assert_eq!(a[0], (77, 0x19, 1), "depth read first");
        // 缓冲：读 0x1a / 写 0x32（write_storage_buffer）
        assert_eq!(a[3].1, 0x1a);
        assert_eq!(a[4].1, 0x32);
        assert_eq!(
            pass.access_ranges.len(),
            1 + 5,
            "color library + 5 declared bindings"
        );
    }
}
