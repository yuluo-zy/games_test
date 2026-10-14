//! `prefix_scan.rs` — u32 inclusive 前缀和（上限 1M 项，三段式；`inclusive_prefix_scan_u32_1m` 0x14098bf60，1811 B）。
//!
//! 机器码（O）：入参 `(recorder, world, buf: &RgBuffer)`；`*(u64*)buf` = 缓冲字节数 `B`，元素个数 `n = B / 4`。
//! * 断言 `assert_eq!(n & 0x3ff, 0)`（`(B >> 2) & 0x3ff` 与 0 比较，`core::panicking::assert_failed<usize,usize>`，消息为空串）。
//! * `blocks = B >> 12`（= n / 1024）；`half = n - (n >> 1)`（`(B>>2) - (B>>3)`）。
//! * **Pass 1** `ComputePass::new("_Prefix 1", "/prefix_scan/inclusive_prefix_scan.cs.hlsl")`（名长 9、URI 长 0x2a）：
//!   `write_rg_storage_buffer(inout_buf, buf)` → `dispatch_1d(half)`。
//! * 子和缓冲：`m = max(blocks, 1)`；`padded = m.next_multiple_of(1024)`（`0x400 - (m & 0x3ff)`，余 0 则不补）；
//!   `BufferDesc::new_gpu_only(padded * 4, 0)`；`create_raw_resource("subtotal_sum_buf")`（名长 0x10）。
//! * **Pass 2** `("_Prefix 2", "/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl")`（名长 9、URI 长 0x34）：
//!   `read_rg_storage_buffer(input_buf, buf)`、`write_rg_storage_buffer(0xecf59a3a8eeaedc6, subtotals)` →
//!   `dispatch_1d(blocks - (blocks >> 1))`（`(B>>12) - (B>>13)`）。
//! * **Pass 3** `("_Prefix merge", "/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl")`（名长 0xd、URI 长 0x30）：
//!   `write_rg_storage_buffer(inout_buf, buf)`、`read_rg_storage_buffer(subtotal_sum_buf, subtotals)` → `dispatch_1d(half)`。
//!
//! U：Pass 2 的写缓冲绑定名哈希 `0xecf59a3a8eeaedc6` 不在重建 HLSL 的标识符里（名字 U，值取立即数）。
//! `grass.rs` 复用本函数（O）。

use crate::render::pass_builder::compute_pass_builder::{RecordedComputePass, RgBuffer};
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding_hash};

pub const PASS1_NAME: &str = "_Prefix 1";
pub const PASS1_URI: &str = "/prefix_scan/inclusive_prefix_scan.cs.hlsl";
pub const PASS2_NAME: &str = "_Prefix 2";
pub const PASS2_URI: &str = "/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl";
pub const MERGE_NAME: &str = "_Prefix merge";
pub const MERGE_URI: &str = "/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl";
pub const SUBTOTAL_NAME: &str = "subtotal_sum_buf";
pub const INOUT_BUF: u64 = binding_hash("inout_buf");
pub const INPUT_BUF: u64 = binding_hash("input_buf");
pub const SUBTOTAL_SUM_BUF: u64 = binding_hash("subtotal_sum_buf");
/// Pass 2 写子和缓冲的绑定名哈希（O 立即数，名 U）。
pub const SUBTOTAL_OUT_BUF: u64 = 0xecf5_9a3a_8eea_edc6;

/// 带字节数的 RG 缓冲句柄（O：`+0` = 字节数，资源号在 +0x10；A4 的 `RgBuffer` 只含资源号）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizedRgBuffer {
    pub buffer: RgBuffer,
    pub size_bytes: u64,
}

/// 子和缓冲的字节数（O）：`max(blocks,1)` 向上取到 1024 的倍数，每项 4 B。
pub fn subtotal_buffer_bytes(size_bytes: u64) -> u64 {
    let blocks = size_bytes >> 12;
    let m = blocks.max(1);
    m.next_multiple_of(0x400) * 4
}

/// 三段 pass（图执行顺序）。
pub struct PrefixScan {
    pub subtotals: RgBuffer,
    pub pass1: RecordedComputePass,
    pub pass2: RecordedComputePass,
    pub merge: RecordedComputePass,
}

/// `inclusive_prefix_scan_u32_1m`：对 `buf` 原地做 u32 前缀和。元素数必须是 1024 的倍数（否则 `assert_eq!` panic）。
pub fn inclusive_prefix_scan_u32_1m(env: &mut PassEnv<'_>, buf: &SizedRgBuffer) -> PrefixScan {
    let n = buf.size_bytes >> 2;
    assert_eq!(n & 0x3ff, 0);
    let half = (n - (n >> 1)) as u32;
    let blocks = buf.size_bytes >> 12;

    let pass1 = env
        .pass(PASS1_NAME, PASS1_URI)
        .write_rg_storage_buffer(INOUT_BUF, &buf.buffer)
        .dispatch_1d(half);

    let subtotals = env.create_buffer(
        SUBTOTAL_NAME,
        subtotal_buffer_bytes(buf.size_bytes) as u32,
        0,
    );

    let pass2 = env
        .pass(PASS2_NAME, PASS2_URI)
        .read_rg_storage_buffer(INPUT_BUF, &buf.buffer)
        .write_rg_storage_buffer(SUBTOTAL_OUT_BUF, &subtotals)
        .dispatch_1d((blocks - (blocks >> 1)) as u32);

    let merge = env
        .pass(MERGE_NAME, MERGE_URI)
        .write_rg_storage_buffer(INOUT_BUF, &buf.buffer)
        .read_rg_storage_buffer(SUBTOTAL_SUM_BUF, &subtotals)
        .dispatch_1d(half);

    PrefixScan {
        subtotals,
        pass1,
        pass2,
        merge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{accesses, env, threads};

    fn buf(bytes: u64) -> SizedRgBuffer {
        SizedRgBuffer {
            buffer: RgBuffer { resource: 40 },
            size_bytes: bytes,
        }
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(INOUT_BUF, 0xaab8_61d3_18ad_7248);
        assert_eq!(INPUT_BUF, 0xbb51_ae97_9b5e_9131);
        assert_eq!(SUBTOTAL_SUM_BUF, 0x5067_cfd1_b30b_03ab);
    }

    #[test]
    fn subtotal_buffer_pads_to_1024_entries() {
        // 1024 项 → blocks = 1 → 1024 项 → 4096 B
        assert_eq!(subtotal_buffer_bytes(1024 * 4), 1024 * 4);
        // 0 项 → blocks = 0 → max(0,1) = 1 → 1024 项
        assert_eq!(subtotal_buffer_bytes(0), 1024 * 4);
        // 2048 * 1024 项 → blocks = 2048 → 2048 项（已是 1024 倍数）
        assert_eq!(subtotal_buffer_bytes(2048 * 1024 * 4), 2048 * 4);
        // 1M 项 → blocks = 1024 → 1024 项
        assert_eq!(subtotal_buffer_bytes((1u64 << 20) * 4), 1024 * 4);
        // blocks = 1025 → 2048 项
        assert_eq!(subtotal_buffer_bytes(1025 * 1024 * 4), 2048 * 4);
    }

    #[test]
    fn three_passes_in_order_with_exe_dispatch_sizes() {
        let mut e = env(
            &[INOUT_BUF, INPUT_BUF, SUBTOTAL_OUT_BUF, SUBTOTAL_SUM_BUF],
            &[],
        );
        let n = 4096u64 * 3; // 元素数；字节 = 49152，blocks = 49152 >> 12 = 12
        let out = e.run(|env| inclusive_prefix_scan_u32_1m(env, &buf(n * 4)));
        assert_eq!(
            (out.pass1.name, out.pass2.name, out.merge.name),
            (PASS1_NAME, PASS2_NAME, MERGE_NAME)
        );
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![
                format!("{PASS1_URI}:cs_main"),
                format!("{PASS2_URI}:cs_main"),
                format!("{MERGE_URI}:cs_main")
            ]
        );
        assert_eq!(threads(&out.pass1), (6144, 1, 1), "n - n/2");
        assert_eq!(threads(&out.pass2), (6, 1, 1), "blocks - blocks/2 = 12 - 6");
        assert_eq!(threads(&out.merge), (6144, 1, 1));
        assert_eq!(e.frame.buffers[0].0, SUBTOTAL_NAME);
    }

    #[test]
    fn pass_buffer_directions() {
        let mut e = env(
            &[INOUT_BUF, INPUT_BUF, SUBTOTAL_OUT_BUF, SUBTOTAL_SUM_BUF],
            &[],
        );
        let out = e.run(|env| inclusive_prefix_scan_u32_1m(env, &buf(1024 * 4)));
        let sub = out.subtotals.resource;
        assert_eq!(accesses(&out.pass1), vec![(40, 0x37, 0)]);
        assert_eq!(accesses(&out.pass2), vec![(40, 0x1a, 1), (sub, 0x37, 0)]);
        assert_eq!(accesses(&out.merge), vec![(40, 0x37, 0), (sub, 0x1a, 1)]);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn non_multiple_of_1024_elements_asserts() {
        let mut e = env(&[], &[]);
        e.run(|env| inclusive_prefix_scan_u32_1m(env, &buf(1025 * 4)));
    }
}
