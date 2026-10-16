# REVIEW7 — systems/terrain(卡7 回读链终验 → 闭合·高度链)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 卡7(terrain 回读链验证,RESTORE 六.卡7 节);`cargo check --lib` exit 0。

## 核对

### ① read_terrain_data 回读链 — PASS
- `read_terrain_data`(0x14098c680,库内 render_loop/compute_passes/read_terrain_data.rs,
  一轮已核):光栅 "Visual terrain height"(terrain_shader/rocky_terrain 两 draw)→
  "Read terrain height"/"Read terrain rock mask"/"Read water"(demo 门跳过)/"Read garden"
  4 个 compute pass,各 `write_storage_buffer<S::...>` → `ScheduledTerrainReadback::
  on_gpu_copy_issued`(swap 取走 pending + copy_issued |= old!=0)✓。
- 消费链:`delayed_terrain_changed_msg`(0x140aabdd0,一轮已核)经
  `library.get::<TerrainHeightSsbo>().mapped` + `copy_from_gpu` 拷贝双实例(authority+visual),
  两次拷贝间发 OnTerrainChange ✓。生产(compute)→登记(on_gpu_copy_issued)→延迟回读
  (delayed 两帧)衔接完整。

### ② 尺寸断言适配 — PASS(等价)
原版 BufferState 名字比对 = 完整类型路径(TerrainHeightSsbo 0x3a / VisualTerrainHeightSsbo
0x40 / TerrainRockMaskSsbo 0x3f);crate `ssbo_library` 的 `get<T>/get_mut/get_buffer/
get_cpu_shadow_words` 均 `assert_eq!(state.name, type_name::<T>())`,而 `type_name::<T>()`
正是完整路径串——**与原文全路径名比对同源等价**,且 ssbo_library 文档(行 18/30-33/323)
明确原版即用 type_name 路径比对。✓

### ③ height_readback_ssbo! 宏 — PASS
`NAME=stringify!($name)`、`BUFFER_LEN=0x20000`、`MAPPED_MEMORY=MAPPED_MEMORY_READBACK`;
`TerrainHeightSsbo`/`VisualTerrainHeightSsbo` 两实例注册 ✓(与回读线程数 0x20000、
0x40000 u16 高度场一致)。

### ④ mask 四类 GAP 登记 — PASS(诚实,不冒充)
`ReadbackSsbos` 关联类型缺 TerrainRockMaskSsbo(terrain_rock_mask.rs GAP)、
WaterExcludeClutterMaskSsbo、GardenMaskSsbo、WaterMaskSsbo(缺 ssbo_library::SsboType 实现);
高度对(Height/VisualHeight)链已闭合。RESTORE 六.卡7 显式列为"阻塞项/待资源 GAP 闭合",
无冒充闭合。

## 结论与 DISPATCH 建议

- 卡7 复验通过:**terrain 卡7 → 闭合(高度回读链)**,mask 四类归**卡B 资源 GAP**(不阻塞
  高度链验收)。
- 建议:terrain 维持 **review_passed(限定已实现范围)**,卡7 高度回读链计入已实现;
  mask 四类由资源 GAP 闭合后补生产 impl。交互组三模块维持限定通过结论。
