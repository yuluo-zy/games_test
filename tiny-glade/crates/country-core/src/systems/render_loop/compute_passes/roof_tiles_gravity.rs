//! `roof_tiles_gravity.rs` — W19 屋顶瓦片重力模拟（O：`roof_tiles_gravity_compute` 0x140a29b10，1793 B；
//! 迭代器 `from_iter` 0x14130d8e0、dispatch 闭包 `call_once` 0x1415f9210，各按其机器码对照）。
//!
//! 流程（逐条对照机器码）：
//! 1. `get_resource_id(SSBO_LIBRARY)`（缺失 → unwrap panic）→ `AssetSsboLibrary::get::<RoofTileSsbo>()`
//!    （未注册 → unwrap panic；`assert_eq!(type_name, "country_core::resources::roof_tile_ssbo::RoofTileSsbo")`，0x35 B）→
//!    `call [vtable+0x18]`（取稀疏实例缓冲，`None` → unwrap panic）。
//! 2. `QueryState::new::<(&RoofTiles, &RoofTileSimulation, &Roof, &RoofAnimation)>`；对每个匹配的屋顶：
//!    `roof_tiles.allocation`（`Option<SparseInstanceBufferAllocation<RoofTileSsbo>>`）若有 →
//!    `allocation_to_draw_multi_instanced` → `Vec<InstancedDraw>`（每项 8 B：`{first_instance, instance_count}`）→
//!    每个 draw 映射成 [`PushConstants`]（20 B）：`{draw.first_instance, draw.instance_count, wall_height_delta, roof_pivot_y, animation_t}`
//!    ——三个 f32 来自该屋顶实体：`RoofTileSimulation` 首字段、`Roof+0x4c`、`RoofAnimation` 首字段
//!    （顺序对应 `roof_tiles_gravity.cs.hlsl` 的 `RoofPushConstants`；字段↔组件的对应为 I）。
//!    全部 `collect` 成 `Vec<PushConstants>`（**先于 `ComputePass::new`**）。
//! 3. `ComputePass::new("Roof tiles", "/roof_tiles_gravity.cs.hlsl")`：`read_storage_buffer<RoofTileSsbo>`（`instanced_data`）；
//!    `get_resource_id(0x4bc362195b80f729, 0x982798ce80cbcefb)`（缺失 → unwrap panic；读 `+0x28` 的 f32 → `write_uniform<f32>`，
//!    键 0x601a45ce43af912f，名字 U）。
//! 4. `dispatch_with(闭包)`：对 `Vec<PushConstants>` 逐项 `push_constants(20 B)` + `dispatch_3d((instance_count, 1, 1))`
//!    （O 0x1415f9210；空表也登记 pass）。
//!
//! SSBO 名 "RoofTileSsbo"（O：SsboId 0x13d939590d705099 = xxh3_64("RoofTileSsbo")）。
//!
//! 范围外：ECS 查询本体（bevy `QueryState`）与 `SparseInstanceBuffer::allocation_to_draw_multi_instanced`——调用方以
//! [`RoofTileRow`]（查询行 + 已展开的 draw 列表）形式给出。

use crate::render::pass_builder::compute_pass_builder::DispatchArgs;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::resources::roofs::roof_tile_ssbo::RoofTileSsbo;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, ResourceView, binding_hash,
};

/// 原版完整类型名（0x35 B，机器断言）；定义在 `RoofTileSsbo` 元素模块。
pub use crate::resources::roofs::roof_tile_ssbo::TYPE_NAME;

pub const PASS_NAME: &str = "Roof tiles";
pub const SHADER_URI: &str = "/roof_tiles_gravity.cs.hlsl";
pub const HASH_INSTANCED_DATA: u64 = binding_hash("instanced_data"); // 0xc3483cd70105de2f
/// 第二个 uniform 键（O：`movabs r8, 0x601a45ce43af912f`；名字 U）。
pub const KEY_FRAME_F32: u64 = 0x601a45ce43af912f;
/// `PushConstants` 的字节数（O：`mov r9d, 0x14`）。
pub const PUSH_SIZE: usize = 0x14;

/// `RoofPushConstants`（O：20 B；`roof_tiles_gravity.cs.hlsl`：first_instance, instance_count, wall_height_delta,
/// roof_pivot_y, animation_t）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PushConstants {
    pub first_instance: u32,
    pub instance_count: u32,
    pub wall_height_delta: f32,
    pub roof_pivot_y: f32,
    pub animation_t: f32,
}

impl PushConstants {
    /// 小端 20 B（原版直接把结构体内存推给 `push_constants`）。
    pub fn to_bytes(&self) -> [u8; PUSH_SIZE] {
        let mut b = [0u8; PUSH_SIZE];
        b[0..4].copy_from_slice(&self.first_instance.to_le_bytes());
        b[4..8].copy_from_slice(&self.instance_count.to_le_bytes());
        b[8..12].copy_from_slice(&self.wall_height_delta.to_le_bytes());
        b[12..16].copy_from_slice(&self.roof_pivot_y.to_le_bytes());
        b[16..20].copy_from_slice(&self.animation_t.to_le_bytes());
        b
    }
}

/// 一个匹配的屋顶实体（ECS 查询行）+ 其瓦片分配展开出的 draw 列表。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoofTileRow {
    /// `roof_tiles.allocation` 是否存在；`None` 的屋顶不贡献 push constants。
    pub draws: Option<Vec<(u32, u32)>>,
    pub wall_height_delta: f32,
    pub roof_pivot_y: f32,
    pub animation_t: f32,
}

/// 第二个 world 资源（O：`+0x28` 的 f32；其余布局 U）。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RoofGravityFrame {
    _head: [u8; 0x28],
    pub value: f32,
}

impl RoofGravityFrame {
    pub fn with_value(value: f32) -> Self {
        RoofGravityFrame {
            _head: [0; 0x28],
            value,
        }
    }
}

// SAFETY: 只读 `+0x28` 的 f32（O：`movd xmm3, [rax+0x28]`）。
unsafe impl ResourceView for RoofGravityFrame {
    const TYPE_ID: TypeId128 = TypeId128 {
        lo: 0x4bc362195b80f729,
        hi: 0x982798ce80cbcefb,
    };
}

/// `from_iter` 的展开（O 0x14130d8e0 + `flatten::next`）：每行 → 每个 draw 一个 `PushConstants`。
pub fn collect_push_constants(rows: &[RoofTileRow]) -> Vec<PushConstants> {
    rows.iter()
        .flat_map(|r| {
            r.draws
                .iter()
                .flatten()
                .map(move |&(first_instance, instance_count)| PushConstants {
                    first_instance,
                    instance_count,
                    wall_height_delta: r.wall_height_delta,
                    roof_pivot_y: r.roof_pivot_y,
                    animation_t: r.animation_t,
                })
        })
        .collect()
}

/// W19 入口（O 0x140a29b10）。`rows` = ECS 查询结果。
pub fn roof_tiles_gravity_compute(env: &mut PassEnv<'_>, rows: &[RoofTileRow]) {
    roof_tiles_gravity_impl(env, rows, |env| env.resource::<RoofGravityFrame>().value);
}

/// Actual Time::delta_seconds projection supplied by the enclosing host.
pub fn roof_tiles_gravity_compute_with_delta(
    env: &mut PassEnv<'_>,
    rows: &[RoofTileRow],
    value: f32,
) {
    roof_tiles_gravity_impl(env, rows, |_| value);
}

fn roof_tiles_gravity_impl(
    env: &mut PassEnv<'_>,
    rows: &[RoofTileRow],
    value: impl FnOnce(&PassEnv<'_>) -> f32,
) {
    // 1. 库与缓冲必须存在（缺失 → unwrap panic）。
    crate::render::pass_builder::pass_resources::ResourceLookup::get_resource_ptr(
        env.world,
        crate::systems::render_loop::compute_passes::bin_lights::SSBO_LIBRARY_TYPE_ID,
    )
    .expect("called `Option::unwrap()` on a `None` value");
    env.pass_resources()
        .ssbo_library()
        .get::<RoofTileSsbo>()
        .expect("called `Option::unwrap()` on a `None` value");
    // 2. push constants（先于 pass 创建）。
    let push = collect_push_constants(rows);
    // 3. pass
    let pass = env
        .pass(PASS_NAME, SHADER_URI)
        // Packaged original SPV loads RoofShingle at %104 and stores its
        // updated record through the same pointer. The legacy native helper
        // calls this a read; the current graph must retain the actual RMW.
        .write_storage_buffer::<RoofTileSsbo>(HASH_INSTANCED_DATA)
        .preserve_storage_buffer_contents(HASH_INSTANCED_DATA);
    let value = value(env);
    let pass = pass.uniform(KEY_FRAME_F32, value);
    // 4. dispatch_with(闭包)
    let recorded = pass.dispatch_with(move |api| {
        for pc in &push {
            api.push_constants(&pc.to_bytes());
            api.dispatch_3d(DispatchArgs {
                x: pc.instance_count,
                y: 1,
                z: 1,
                indirect_index: None,
            });
        }
    });
    env.record(recorded);
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::{ComputeExec, DispatchMode};
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    #[derive(Default)]
    struct Exec {
        log: Vec<String>,
    }
    impl ComputeExec for Exec {
        fn full_barrier_enabled(&self) -> bool {
            false
        }
        fn issue_full_pipeline_barrier(&mut self) {}
        fn count_dispatch(&mut self) {}
        fn cmd_dispatch(&mut self, x: u32, y: u32, z: u32) {
            self.log.push(format!("dispatch {x} {y} {z}"));
        }
        fn cmd_dispatch_indirect(&mut self, _: u32, _: u64) -> Option<()> {
            None
        }
        fn local_size(&self) -> [u32; 3] {
            [64, 1, 1]
        }
        fn push_constants(&mut self, b: &[u8]) {
            self.log.push(format!("push {}", b.len()));
        }
    }

    fn world(value: Option<f32>) -> TestWorld {
        world_with(value, true)
    }
    fn world_with(value: Option<f32>, register_ssbo: bool) -> TestWorld {
        let w = TestWorld::new(
            Reflection::new()
                .bind(&[HASH_INSTANCED_DATA])
                .uniforms(&[KEY_FRAME_F32]),
        );
        let w = if register_ssbo {
            w.with_ssbo::<RoofTileSsbo>()
        } else {
            w
        };
        match value {
            Some(v) => w.with_resource(RoofGravityFrame::with_value(v)),
            None => w,
        }
    }
    fn frame() -> TestFrame {
        TestFrame::default()
    }
    fn rows() -> Vec<RoofTileRow> {
        vec![
            RoofTileRow {
                draws: Some(vec![(0, 10), (16, 4)]),
                wall_height_delta: 0.5,
                roof_pivot_y: 3.0,
                animation_t: 0.25,
            },
            RoofTileRow {
                draws: None,
                wall_height_delta: 9.0,
                roof_pivot_y: 9.0,
                animation_t: 9.0,
            },
            RoofTileRow {
                draws: Some(vec![(100, 7)]),
                wall_height_delta: 1.0,
                roof_pivot_y: 2.0,
                animation_t: 1.0,
            },
        ]
    }

    #[test]
    fn push_constants_flatten_rows_in_order_and_skip_rows_without_allocation() {
        let pcs = collect_push_constants(&rows());
        assert_eq!(pcs.len(), 3);
        assert_eq!(
            pcs[0],
            PushConstants {
                first_instance: 0,
                instance_count: 10,
                wall_height_delta: 0.5,
                roof_pivot_y: 3.0,
                animation_t: 0.25
            }
        );
        assert_eq!(pcs[1].first_instance, 16);
        assert_eq!(pcs[1].roof_pivot_y, 3.0, "同一屋顶的所有 draw 共享三个 f32");
        assert_eq!(pcs[2].first_instance, 100);
        assert_eq!(pcs[2].instance_count, 7);
        assert!(collect_push_constants(&[]).is_empty());
    }

    #[test]
    fn push_constants_are_20_bytes_little_endian() {
        assert_eq!(core::mem::size_of::<PushConstants>(), PUSH_SIZE);
        let b = PushConstants {
            first_instance: 1,
            instance_count: 2,
            wall_height_delta: 1.0,
            roof_pivot_y: 2.0,
            animation_t: 3.0,
        }
        .to_bytes();
        assert_eq!(&b[0..4], &1u32.to_le_bytes());
        assert_eq!(&b[4..8], &2u32.to_le_bytes());
        assert_eq!(&b[16..20], &3.0f32.to_le_bytes());
    }

    #[test]
    fn records_one_pass_whose_closure_pushes_then_dispatches_each_draw() {
        let w = world(Some(0.016));
        let mut f = frame();
        with_env(&w, &mut f, |env| roof_tiles_gravity_compute(env, &rows()));
        assert_eq!(f.passes.len(), 1);
        let p = f.passes.pop().unwrap();
        assert_eq!(p.name, "Roof tiles");
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/roof_tiles_gravity.cs.hlsl:cs_main"
        );
        assert!(matches!(p.dispatch, DispatchMode::With(_)));
        assert_eq!(f32::from_bits(uniform_u32(&p, 0)), 0.016);
        // Original shader performs RMW despite its legacy read helper name.
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x32]);
        assert_eq!(p.preserving_buffers, [p.builder.accesses[1].resource]);
        let mut exec = Exec::default();
        p.execute(&mut exec);
        // 每个 draw：先 push 20 B，再 dispatch_3d((count,1,1))，local size 64 → ceil。
        assert_eq!(
            exec.log,
            [
                "push 20",
                "dispatch 1 1 1",
                "push 20",
                "dispatch 1 1 1",
                "push 20",
                "dispatch 1 1 1"
            ]
        );
    }

    #[test]
    fn dispatch_sizes_follow_instance_counts() {
        let w = world(Some(0.0));
        let mut f = frame();
        let big = vec![RoofTileRow {
            draws: Some(vec![(0, 130), (0, 64), (0, 65)]),
            ..Default::default()
        }];
        with_env(&w, &mut f, |env| roof_tiles_gravity_compute(env, &big));
        let mut exec = Exec::default();
        f.passes.pop().unwrap().execute(&mut exec);
        let d: Vec<&String> = exec
            .log
            .iter()
            .filter(|s| s.starts_with("dispatch"))
            .collect();
        assert_eq!(d, ["dispatch 3 1 1", "dispatch 1 1 1", "dispatch 2 1 1"]);
    }

    #[test]
    fn empty_query_still_registers_a_pass_that_does_nothing() {
        let w = world(Some(1.0));
        let mut f = frame();
        with_env(&w, &mut f, |env| roof_tiles_gravity_compute(env, &[]));
        assert_eq!(f.passes.len(), 1);
        let mut exec = Exec::default();
        f.passes.pop().unwrap().execute(&mut exec);
        assert!(exec.log.is_empty());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn unregistered_roof_tile_buffer_unwraps() {
        let w = world_with(Some(1.0), false);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| roof_tiles_gravity_compute(env, &[]));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_second_resource_unwraps_after_reading_the_buffer() {
        let w = world(None);
        let mut f = frame();
        with_env(&w, &mut f, |env| roof_tiles_gravity_compute(env, &[]));
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(HASH_INSTANCED_DATA, (-0x3cb7c328fefa21d1_i64) as u64);
        assert_eq!(
            RoofGravityFrame::TYPE_ID.hi,
            (-0x67d867317f343105_i64) as u64
        );
        assert_eq!(core::mem::offset_of!(RoofGravityFrame, value), 0x28);
        assert_eq!(TYPE_NAME.len(), 0x35);
        assert_eq!(PASS_NAME.len(), 0xa);
        assert_eq!(SHADER_URI.len(), 0x1b);
    }
}
