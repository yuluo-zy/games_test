//! `bin_lights.rs` — W40 光源分格（O：`bin_lights` 0x1409e7770，3891 B；`BinLights::as_compute_pipeline` 0x1409e8db0）。
//!
//! 返回（O：`sret`，0x30 B）：`LightBinning { grid: RgBuffer @0, indices: RgBuffer @0x18 }`
//! （各 = `BufferDesc`(16 B) + 资源号）。
//!
//! 流程（逐条对照机器码）：
//! 1. `get_resource_id(0x8226cc786e679355, 0x65cfda6a55a841ac)`（= `AssetSsboLibrary`，缺失 → unwrap panic）；
//!    `library.get::<LanternSsbo>()`（未注册 → unwrap panic；`assert_eq!(type_name, "country_core::resources::render::lantern::LanternSsbo")`）→
//!    `InstanceBuffer::instance_count`（`None` → unwrap panic）= `count`。
//! 2. `count == 0`：只建两个占位缓冲 `new_gpu_only(4, 0x20)`——"Light indirection grid placeholder"、
//!    "Light indices placeholder"（先 grid 后 indices），不登记任何 pass。
//! 3. 否则（全部缓冲 `usage = 0x20`）：
//!    1. grid = "Light indirection grid"（**0x10000** B）；`clear_rg_buffer(grid)`；
//!    2. pass "Count cell lights"（`BinLights(0)`）：`read_storage_buffer<LanternSsbo>`(`lantern_buf`)、
//!       `write_rg_storage_buffer`(`indirection_grid`, grid)、`write_uniform<i32>`(`lantern_count`, count)、`dispatch_1d(count)`；
//!    3. subtotals = "Light indirection subtotals"（**0x44** B）；
//!    4. pass "Light count prefix"（`/light_binning/cell_prefix_sum.cs.hlsl`）：写 A(`inout_buf`)=grid、
//!       写 B(`subtotals_buf`)=subtotals、`dispatch_1d(0x2000)`；
//!    5. pass "Light count subtotal prefix"（`cell_subtotal_prefix_sum`）：写 A=subtotals、push constants `[0x10u32]`、`dispatch_1d(8)`；
//!    6. pass "Light count prefix merge"（`cell_prefix_merge`）：写 A=grid、读 B=subtotals、`dispatch_1d(0x4000)`；
//!    7. indices = "Light indices"（**0x100000** B）；
//!    8. pass "Bin lights to cells"（`BinLights(1)`）：读 Lantern、写 `indirection_grid`=grid、写 `light_indices`=indices、
//!       `write_uniform<i32>`(同上, count)、`dispatch_1d(count)`。
//!
//! `BinLights(u32)`：`as_compute_pipeline` = `{ path "/light_binning/bin_lights_to_cells.cs.hlsl", entry "cs_main",
//! defines [("PERM_write", format!("{}", self.0))] }`（O：`u32` Display）；`source_hash` 为 `calculate_hash(&(path:entry, &BinLights))`
//! （O：`ComputePass::new<BinLights>` 调 `calculate_hash<(&str, &_)>`，哈希写 4 字节 = u32 字段）。
//!
//! `PERM_write` define 经 `AsComputeShaderSource::pipeline_defines`（A4，i_api）进入 `ComputePipelineDesc.defines`；
//! `ShaderDefValue(u64)` 仍只带 `perm`（与既有实现兼容）。
//!
//! 灯笼缓冲类型 `L` 是泛型（真实 `resources::render::lantern::LanternSsbo`）：`AssetSsboLibrary::get::<L>()` 内含
//! `assert_eq!(state.name, type_name::<L>())`，即 [`LANTERN_TYPE_NAME`]（0x35 B）的比对由库完成，这里不再造第二份。

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::RgBuffer;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::render::pass_builder::shader_uri::{
    AsComputeShaderSource, ComputePipelineDesc, ComputeShaderSource, ShaderDefValue, ShaderDefine,
    ShaderUri,
};
use crate::render::rg_helpers::clear_rg_buffer;
use crate::render::ssbo::InstanceCounted;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding_hash};

/// 光源缓冲 `usage`（O：`new_gpu_only(.., 0x20)` 的第二参数）。
pub const BUFFER_USAGE: u32 = 0x20;
pub const GRID_SIZE: u32 = 0x10000;
pub const SUBTOTALS_SIZE: u32 = 0x44;
pub const INDICES_SIZE: u32 = 0x100000;
pub const PLACEHOLDER_SIZE: u32 = 4;

/// 绑定名（O：FNV-1a-64；`__alias_` 前缀已剥）。
pub const HASH_LANTERNS: u64 = binding_hash("lantern_buf"); // 0x0c6b33eb20992aad
pub const HASH_GRID: u64 = binding_hash("indirection_grid"); // 0x63e587f19a869d94
pub const HASH_INDICES: u64 = binding_hash("light_indices"); // 0xe3b8c78bd96aa31d
/// 前缀和三个 pass 共用的两个绑定名（O）。
pub const HASH_PREFIX_A: u64 = binding_hash("inout_buf"); // 0xaab861d318ad7248
pub const HASH_PREFIX_B: u64 = binding_hash("subtotals_buf"); // 0xc3674440971f6990
pub const KEY_LIGHT_COUNT: u64 = binding_hash("lantern_count"); // 0x9b88329e408ff69f

pub const PASS_COUNT: &str = "Count cell lights";
pub const PASS_PREFIX: &str = "Light count prefix";
pub const PASS_SUBTOTAL_PREFIX: &str = "Light count subtotal prefix";
pub const PASS_MERGE: &str = "Light count prefix merge";
pub const PASS_BIN: &str = "Bin lights to cells";
pub const URI_PREFIX: &str = "/light_binning/cell_prefix_sum.cs.hlsl";
pub const URI_SUBTOTAL_PREFIX: &str = "/light_binning/cell_subtotal_prefix_sum.cs.hlsl";
pub const URI_MERGE: &str = "/light_binning/cell_prefix_merge.cs.hlsl";
pub const BIN_URI: &str = "/light_binning/bin_lights_to_cells.cs.hlsl:cs_main";
/// 三个前缀 pass 的 dispatch 线程数（O：`.rodata` 常量 `{1, n, 1, 1}`）。
pub const PREFIX_THREADS: u32 = 0x2000;
pub const SUBTOTAL_THREADS: u32 = 8;
pub const MERGE_THREADS: u32 = 0x4000;
/// 小计前缀 pass 的 push constant（O：`mov dword ptr [rax], 0x10`）。
pub const SUBTOTAL_PUSH: u32 = 0x10;

/// `bin_lights_to_cells.cs.hlsl:cs_main` 的 `PERM_write` 变体（0 = 只计数，1 = 写索引）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BinLights(pub u32);

impl BinLights {
    /// O 0x1409e8db0：真实的 `ComputePipelineDesc`（含 defines）。
    pub fn pipeline_desc(&self) -> ComputePipelineDesc {
        ComputePipelineDesc {
            path: "/light_binning/bin_lights_to_cells.cs.hlsl".to_owned(),
            entry: "cs_main".to_owned(),
            defines: vec![("PERM_write".to_owned(), format!("{}", self.0))],
        }
    }
}

impl AsComputeShaderSource for BinLights {
    fn source_hash(&self) -> u64 {
        crate::render::utils::calculate_hash(&(BIN_URI, &self.0))
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse(BIN_URI),
            defs: ShaderDefValue(u64::from(self.0)),
        }
    }
    /// O 0x1409e8db0：`[("PERM_write", format!("{}", self.0))]`。
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        self.pipeline_desc().defines
    }
}

/// `LanternSsbo` 的库内类型名（O：`assert_eq!` 的右值，0x35 B）。
pub const LANTERN_TYPE_NAME: &str = "country_core::resources::render::lantern::LanternSsbo";

/// 返回值（O：0x30 B）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LightBinning {
    pub grid: RgBuffer,
    pub indices: RgBuffer,
}

/// 复刻 `AssetSsboLibrary` 的 TypeId（`bin_lights` 第一个 `get_resource_id`，缺失 → unwrap panic）。
pub const SSBO_LIBRARY_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x8226cc786e679355,
    hi: 0x65cfda6a55a841ac,
};

/// W40 入口（O 0x1409e7770）。`L` = `LanternSsbo`（库侧实例缓冲）。
pub fn bin_lights<L: LibrarySsbo>(env: &mut PassEnv<'_>) -> LightBinning
where
    L::Storage: InstanceCounted,
{
    // 库资源必须存在（缺失 → `Option::unwrap`）。
    env.world
        .get_resource_ptr(SSBO_LIBRARY_TYPE_ID)
        .expect("called `Option::unwrap()` on a `None` value");
    let count = env
        .ssbo_instance_count::<L>()
        .expect("called `Option::unwrap()` on a `None` value") as u32;

    if count == 0 {
        let grid = env.create_buffer(
            "Light indirection grid placeholder",
            PLACEHOLDER_SIZE,
            BUFFER_USAGE,
        );
        let indices =
            env.create_buffer("Light indices placeholder", PLACEHOLDER_SIZE, BUFFER_USAGE);
        return LightBinning { grid, indices };
    }

    let grid = env.create_buffer("Light indirection grid", GRID_SIZE, BUFFER_USAGE);
    clear_rg_buffer(&mut *env.frame, grid.resource);

    let pass = env
        .new_pass(PASS_COUNT, &BinLights(0))
        .read_storage_buffer::<L>(HASH_LANTERNS)
        .write_rg_storage_buffer(HASH_GRID, &grid)
        .uniform(KEY_LIGHT_COUNT, count as i32)
        .dispatch_1d(count);
    env.record(pass);

    let subtotals = env.create_buffer("Light indirection subtotals", SUBTOTALS_SIZE, BUFFER_USAGE);

    let pass = env
        .pass(PASS_PREFIX, URI_PREFIX)
        .write_rg_storage_buffer(HASH_PREFIX_A, &grid)
        .write_rg_storage_buffer(HASH_PREFIX_B, &subtotals)
        .dispatch_1d(PREFIX_THREADS);
    env.record(pass);

    let pass = env
        .pass(PASS_SUBTOTAL_PREFIX, URI_SUBTOTAL_PREFIX)
        .write_rg_storage_buffer(HASH_PREFIX_A, &subtotals)
        .push_constants(&SUBTOTAL_PUSH.to_le_bytes())
        .dispatch_1d(SUBTOTAL_THREADS);
    env.record(pass);

    let pass = env
        .pass(PASS_MERGE, URI_MERGE)
        .write_rg_storage_buffer(HASH_PREFIX_A, &grid)
        .read_rg_storage_buffer(HASH_PREFIX_B, &subtotals)
        .dispatch_1d(MERGE_THREADS);
    env.record(pass);

    let indices = env.create_buffer("Light indices", INDICES_SIZE, BUFFER_USAGE);

    let pass = env
        .new_pass(PASS_BIN, &BinLights(1))
        .read_storage_buffer::<L>(HASH_LANTERNS)
        .write_rg_storage_buffer(HASH_GRID, &grid)
        .write_rg_storage_buffer(HASH_INDICES, &indices)
        .uniform(KEY_LIGHT_COUNT, count as i32)
        .dispatch_1d(count);
    env.record(pass);

    LightBinning { grid, indices }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::render::pass_builder::pass_resource_access::PassBinding;
    use crate::render::ssbo::{InstanceBuffer, StorageElement};
    use crate::render::test_device::TestDevice;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    /// 灯笼缓冲替身：`InstanceBuffer<T>` 存储（实例数 = 注册时的数据长度）。
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::NoUninit)]
    struct L(u32);
    impl StorageElement for L {
        const NAME: &'static str = "LanternSsbo";
        const BUFFER_LEN: usize = 64;
    }
    impl LibrarySsbo for L {
        type Storage = InstanceBuffer<L>;
        fn new_storage_buffer(
            device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            InstanceBuffer::new_storage_buffer(device, initial)
        }
    }

    /// `count = Some(n)`：注册 `n` 个实例（0 → `add_new` 无缺省元素 → 实例数 0）；`None`：不注册。
    fn world_with(count: Option<usize>) -> TestWorld {
        let w = TestWorld::new(
            Reflection::new()
                .bind(&[
                    HASH_LANTERNS,
                    HASH_GRID,
                    HASH_INDICES,
                    HASH_PREFIX_A,
                    HASH_PREFIX_B,
                ])
                .uniforms(&[KEY_LIGHT_COUNT]),
        );
        match count {
            Some(0) => w.ssbos.add_new::<L>(&TestDevice(1)),
            Some(n) => w
                .ssbos
                .add_new_with_data::<L>(&TestDevice(1), &vec![L(0); n]),
            None => {}
        }
        w
    }
    fn buffer_log(f: &TestFrame) -> Vec<(&str, usize, u32)> {
        f.buffers
            .iter()
            .map(|b| (b.0.as_str(), b.1.size, b.1.usage))
            .collect()
    }

    #[test]
    fn zero_lanterns_creates_only_the_two_placeholders() {
        let w = world_with(Some(0));
        let mut f = TestFrame::default();
        let r = with_env(&w, &mut f, bin_lights::<L>);
        assert_eq!(
            buffer_log(&f),
            [
                ("Light indirection grid placeholder", 4, 0x20),
                ("Light indices placeholder", 4, 0x20)
            ]
        );
        assert!(f.passes.is_empty() && f.rg_passes.is_empty());
        assert_eq!((r.grid, r.indices), (f.buffers[0].2, f.buffers[1].2));
        assert_eq!(w.backend.loads.get(), 0);
    }

    #[test]
    fn full_pipeline_order_names_and_dispatches() {
        let w = world_with(Some(37));
        let mut f = TestFrame::default();
        let r = with_env(&w, &mut f, bin_lights::<L>);
        assert_eq!(
            f.order,
            [
                "Clear buffer",
                "Count cell lights",
                "Light count prefix",
                "Light count subtotal prefix",
                "Light count prefix merge",
                "Bin lights to cells"
            ]
        );
        assert_eq!(
            buffer_log(&f),
            [
                ("Light indirection grid", 0x10000, 0x20),
                ("Light indirection subtotals", 0x44, 0x20),
                ("Light indices", 0x100000, 0x20)
            ]
        );
        assert_eq!(f.rg_passes, [("Clear buffer", r.grid.resource, 0x34)]);
        let d: Vec<_> = f
            .passes
            .iter()
            .map(|p| match p.dispatch {
                DispatchMode::Threads { x, y, z } => (x, y, z),
                _ => panic!("expected 1d thread dispatch"),
            })
            .collect();
        assert_eq!(
            d,
            [
                (37, 1, 1),
                (0x2000, 1, 1),
                (8, 1, 1),
                (0x4000, 1, 1),
                (37, 1, 1)
            ]
        );
        assert_eq!(f.passes[2].push_constants, 0x10u32.to_le_bytes());
        assert!(f.passes[0].push_constants.is_empty());
        let paths = w.backend.paths.borrow().clone();
        assert_eq!(
            paths,
            [
                "/light_binning/bin_lights_to_cells.cs.hlsl:cs_main",
                "/light_binning/cell_prefix_sum.cs.hlsl:cs_main",
                "/light_binning/cell_subtotal_prefix_sum.cs.hlsl:cs_main",
                "/light_binning/cell_prefix_merge.cs.hlsl:cs_main",
                // 第二次 `BinLights` 的源哈希不同（`PERM_write` 不同）→ 再编译一次。
                "/light_binning/bin_lights_to_cells.cs.hlsl:cs_main",
            ]
        );
    }

    #[test]
    fn buffer_wiring_per_pass() {
        let w = world_with(Some(5));
        let mut f = TestFrame::default();
        let r = with_env(&w, &mut f, bin_lights::<L>);
        let (grid, indices) = (r.grid.resource, r.indices.resource);
        let res = |p: &crate::render::pass_builder::compute_pass_builder::RecordedComputePass| {
            p.bindings.iter().map(|b| b.resource()).collect::<Vec<_>>()
        };
        // 计数 pass：color library、lantern（库缓冲）、grid。
        let c = &f.passes[0];
        assert_eq!(
            c.builder
                .accesses
                .iter()
                .map(|a| a.access)
                .collect::<Vec<_>>(),
            [0x1a, 0x1a, 0x37]
        );
        assert_eq!(*res(c).last().unwrap(), grid);
        assert_eq!(uniform_u32(c, 0), 5);
        // 前缀和：先 grid，后 subtotals；小计前缀只写 subtotals；merge 写 grid 读 subtotals（同步组 0 / 1）。
        let p1 = &f.passes[1];
        assert_eq!(&res(p1)[1..], &[grid, p1.bindings[2].resource()]);
        let sub = p1.bindings[2].resource();
        assert_eq!(res(&f.passes[2])[1..], [sub]);
        let m = &f.passes[3];
        assert_eq!(res(m)[1..], [grid, sub]);
        let groups: Vec<u32> = m.builder.accesses.iter().map(|a| a.group).collect();
        assert_eq!(groups[1..], [0, 1]);
        assert!(matches!(m.bindings[1], PassBinding::Buffer { .. }));
        // 分格 pass：写 grid、写 indices。
        let b = &f.passes[4];
        assert_eq!(res(b)[2..], [grid, indices]);
        assert_eq!(uniform_u32(b, 0), 5);
    }

    #[test]
    fn bin_lights_variants_have_distinct_hashes_and_defines() {
        let (a, b) = (BinLights(0), BinLights(1));
        assert_ne!(a.source_hash(), b.source_hash());
        assert_eq!(
            a.pipeline_desc().defines,
            [("PERM_write".to_string(), "0".to_string())]
        );
        assert_eq!(
            b.pipeline_desc().defines,
            [("PERM_write".to_string(), "1".to_string())]
        );
        assert_eq!(
            b.pipeline_desc().path,
            "/light_binning/bin_lights_to_cells.cs.hlsl"
        );
        assert_eq!(b.pipeline_desc().entry, "cs_main");
        assert_eq!(b.as_compute_pipeline().uri.entry, Some("cs_main"));
        // 经 `AsComputeShaderSource::pipeline_defines` 到达 `ComputePass::new` 的 `ComputePipelineDesc.defines`。
        assert_eq!(a.pipeline_defines(), a.pipeline_desc().defines);
        assert_eq!(
            b.pipeline_defines(),
            [("PERM_write".to_string(), "1".to_string())]
        );
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(HASH_LANTERNS, 0x0c6b33eb20992aad);
        assert_eq!(HASH_GRID, 0x63e587f19a869d94);
        assert_eq!(HASH_PREFIX_A, (-0x55479e2ce7528db8_i64) as u64);
        assert_eq!(HASH_PREFIX_B, (-0x3c98bbbf68e09670_i64) as u64);
        assert_eq!(HASH_INDICES, (-0x1c47387426955ce3_i64) as u64);
        assert_eq!(KEY_LIGHT_COUNT, (-0x6477cd61bf700961_i64) as u64);
        assert_eq!(SSBO_LIBRARY_TYPE_ID.lo, (-0x7dd9338791986cab_i64) as u64);
        assert_eq!(LANTERN_TYPE_NAME.len(), 0x35);
        assert_eq!(BIN_URI.len(), 0x32);
        assert_eq!(PASS_COUNT.len(), 0x11);
        assert_eq!(PASS_PREFIX.len(), 0x12);
        assert_eq!(PASS_SUBTOTAL_PREFIX.len(), 0x1b);
        assert_eq!(PASS_MERGE.len(), 0x18);
        assert_eq!(PASS_BIN.len(), 0x13);
        assert_eq!(URI_PREFIX.len(), 0x26);
        assert_eq!(URI_SUBTOTAL_PREFIX.len(), 0x2f);
        assert_eq!(URI_MERGE.len(), 0x28);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn unregistered_lantern_buffer_unwraps() {
        let w = world_with(None);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, bin_lights::<L>);
    }
}
