# REVIEW — terrain+ui_clutter 深度收口(REBUILD-DEEP 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-terrain-deep REBUILD-DEEP.md(ReVa 专属 --write worker 定点恢复);
编译注:现全仓唯一 error 在 `render/shader.rs:104`(anyhow::Error::into_boxed_dyn_error 方法
缺失,**render 组在途重构**,非本模块文件;REBUILD-DEEP 已预告);terrain/ui_clutter 文件
clean,rustfmt 解析通过。

## 逐项复验

### Batch 1 判等-注意 4 — PASS
- `history_record_terrain`(0x140ac9770):删 `if let Err → log+return`,`let _ = history.add`
  忽略返回值后**无条件**写 8B DebugHistoryMsg(fresh:add 后无分支直接事件写)✓。
- `history_record_auto_clutter`(0x1409a9160):删 `if let Err → HistoryRecordingFailed`
  分支 + 签名去 failures 参数(fresh:add 结果无分支)✓。
- `RockyTerrainMesh::load`(0x1408bbf50):返回 `anyhow::Result<Self>`(fresh:anyhow
  from<io::Error>/from<serde_json::Error>,原 Result<_, io::Error> 改)✓。
- `get_last_mut_stroke`/`new_stroke`:fresh next_index==0→unwrap_failed panic、碰撞 panic_fmt;
  tg-terrain 为共享基础 crate(红线不改),Rust Result Err 为**API 适配**,主路径逐条判等 ✓。
- `load_index_unchecked`:disasm 证实 u16/65535 DIVSS + v*max+(1-v)*min;原版 min/max@+0x10/
  +0x14,Rust @+0x50/+0x54——**布局合并适配非 ABI 契约**,语义相同 ✓。

### Batch 2 ui_edit_terrain case 4 — PASS(ReVa 定点恢复)
jt_404c@0x142ae404c 第 4 项 → case4 处理器 0x140af02e2 → 子转移表 **jt_405c@0x142ae405c
12 项**(索引 8..19):8→ev_shift/contrast、10→saturation(XOR 0x6e6f "on")、12→bypass_color、
16→16B 位掩码(PAND+PMOVMSKB==0xffff)、19→双 16B PAND;其余/越界→共享尾 [0]=0,[1]=6。
已按名常量 + 响应字节写语义入 switch ✓。

### Batch 3 focus signifier 链 — PASS
closure$1 "vertex_color_shader"(字节 RNE 直读)+ `insert<DrawTopology>`;closure$2
"brush_arrow_flat_to_single_forward"、closure$3 "brush_arrow_flat_to_w_flat_single_up"
(RNE 字节)——已落 `BRUSH_ARROW_SINGLE_FORWARD_NAME`/`BRUSH_ARROW_W_FLAT_SINGLE_UP_NAME`
常量 + UiSignifierStream CPU 载体;closure$2-4 入流与复杂体(焦点 HashMap/as_curve3/
CircularF32/RequestAppModeChange/相机旋转)宿主实体机制登记待接,**不伪造实体接线** ✓。

### Batch 4 decoclutter 几何 — PASS
`pinned_footprint_intersects`(纯几何,O 序列 expand(+0.5,+0.5)→set_orientation_xz(axis)→
translate(实体世界位置)→overlaps_rectangle3d)+ 单测;双 Vec 缓冲 + PinVisibilityToggle
保留;per-pinned 几何源与实体 spawn/SparseInstanceBuffer 宿主接线登记待接 ✓。

### Batch 5 EditType::Terrain 定案 — PASS
fresh 0x140ac9770 记录 = 直接 0x38B `{tag=1,[1]=1,old_state,pending[0..3]}` 入 History;
`History::add` 仅对 tag-7 Wall 做 add 期校验,地形记录**通用存储**、**非 RasterEdit 族**
(world_raster Path/Cobblestone/Water/Garden 为另一记录路径)。Rust EditType::Terrain +
ReplayCommand::Terrain 为 CPU 侧等效模型,HistoryRecordingValidation 已对齐 ✓。

### Batch 6 stairs/decorator 消费链 — PASS(登记)
生产者核验:edit_terrain_stroke(tag 1/2/3/4)/terrain_stroke_changed(tag 0)/ui_edit_terrain
(tag 2) 写 StairsPlacementBucket(indexmap::Bucket 等价),tests 断言 tags [3,2,0];
消费链(→DecoratorStorage::Stair→ui_clutter 实体生成)登记待 stairs 域宿主接线,不伪造 ✓。

## 结论与 DISPATCH 建议

- terrain+ui_clutter 深度收口复验通过:**6 项收口(8 函数实现/2 定案/3 登记)全部闭合**,
  Batch 1 通过时编译 0 error;现 render/shader.rs 断点为他组在途。
- 建议:深收口 → 闭合(实现层面);宿主实体机制接线(focus 复杂体/decoclutter 几何源+spawn/
  stairs 消费链)由上层模块批次推进;render 断点解除后重跑全量测试。
