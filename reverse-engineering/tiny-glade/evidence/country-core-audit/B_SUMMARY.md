# B 阶段汇总（A4 / N / D / C / S / H）

范围：country-core 渲染模块第 3–7 层（pass_builder、nani、draw_passes、compute 帧 pass、风格化与帧编排）。
方法与规则见 `B_BRIEF.md`；各组详报见 `B_<组名>.md`（a4_draw、a4_compute、n_world、n_sys、i_nani、d_main、d_shaders、
c1、c2、c3、s_h、i_api、i_infra、x_out、d_gap、d_water、d_cb_a、d_cb_b）。

## 验证状态（实测）

- 统一全树测试台 `tools/country-core-audit/l0-harness`（`./run.sh --lib`，可用 `L0H_REAL` 指向源码目录）：**785 通过 / 0 失败 / 0 ignore**。
- 测试台只挂载渲染相关文件；其中 `render::nani` 在个别测试台里是同签名替身（真实 nani 依赖 half/bytemuck 的 shim）。
- **整个 crate 仍未整体编译**：离线缺 `dolly`、glam 的 `mint` feature、`puffin` 等，各层接线只被测试台编译器检查过。
- 渲染目录内残留 `todo!/unimplemented!`：`grading_lut.rs`、`upload_world_raster.rs`、`mesh_atlas_library.rs`、`draw_lists.rs` 少量（见各组报告）。

## 做了什么

- 每个函数按原 EXE 机器码恢复（O/I/U 标注），绑定名哈希 = FNV-1a-64、SsboId = xxh3_64(NAME)、TypeId 用 `typeid_resolve.py` 反查真名。
- 复用真实依赖：bevy_ecs 0.16、glam、half、bytemuck、xxhash-rust、parking_lot；缺失的底层库只写最小 stub。
- 唯一手写的哈希是 `binding_hash`（const fn，原因见 `pass_kit.rs` 注释，测试里用真实 `fnv` crate 交叉校验）。
- 纠正了大量旧文档与机器码不符之处（见各组报告“纠错”节）。

## 已知遗留（未做，不在本轮范围内）

1. `render_world` / `render` 未接到真实 pass 入口（D/C 的 pass 现为 `(PassEnv/DrawPassCtx, 显式输入)`，原版为 `(rg, world, handles)`）；水/冰需提供 `SurfaceCompute` 实现。
2. 范围外依赖仍是桩：`utils::load_json_as_mesh`、`TerrainHeightsData::sample_at_world_xz`、`TerrainRockMask::sample_at_world_xz`、`grading_lut` 加载、`TextureDesc::from(preset)`（0xa59890）、`resources::walls`、`startup::load_*_into_library`。
3. 仍为 U：`shader_refactoring` 状态机 disc 6..=9、`reset_indirect_args` 的 0x1e0B 暂存区、`CENTERED_PATH` 写入者、3 个仅有 TypeId 的组件、若干 `*View` 视图类型字段（来自机器码偏移，待换成真实类型）。
4. 重复/待统一：`draw_callbacks_a/b` 的 `nani_draw_world`、`LibView` 等；同名 `*View` 字段需取并集；`water_planes_draw` 闭包入口与 `*Tag` 别名待删。
5. `rhapsody_types::TextureDesc` 缺 `sampled_with_different_format`、`color_attachment`；`TypedStorageBuffer::new_storage_buffer` 未建模 `BUFFER_LEN>0` 形态。
6. 超 800 行文件（多为内联测试）：draw_builder、compute_pass_builder、painterly、vermeer、render_world、session_selection、draw_callbacks_a 等。
7. 结果可信度 = 机器码对照 + 隔离测试，**未做 GPU/原版行为对照**。
