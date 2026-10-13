# system-water 剩余项反汇编级精读（B 主基线，第二批补全）

2026-10-13。聚焦 water-cpu-analysis 的剩余 unknown：实时笔刷入口 draw_water、
音频网格 update_water_audio_grid、水位 u8 换算、两 SSBO 字段、rasterize_stroke 三闭包体边界。
数据源仅 B patched EXE（sha256 cf048e27…，reva 持久库，worker `water-extra-owner`，只读）+ 发行 RNE 原件
（sha256 f507a6ce…，radare2 机器码 oracle）。PDB 行号表 skeleton/B/crates/**/*.pdb.jsonl.gz。

证据来源：
- **reva**（B 持久库，worker `water-extra-owner`，只读 run-script）：逐函数反编译 + 全反汇编 +
  调用目标 + 内存引用；输出落盘 `/Users/yuluo/ghidra_scripts/water-extra-owner/*.json` 与
  `evidence/toolchain/water-extra/`。
- **radare2**（RNE oracle）：draw_water / update_water_audio_grid / bilinear_sample 入口头窗口与
  B 逐条一致（地址/指令未被 B patch 改动）；.rdata 常量 hexdump 核对。
- **PDB 结构**：WaterMaskSsbo / WaterExcludeClutterMaskSsbo / WaterAudioGrid /
  WaterAudioGridUpdateState 均为 `name_and_relations_only`（fields/size/alignment unknown）。

## 关键新增结论（与 water-cpu-analysis 的差异）

1. **栅格确认为 512×512 u8**（此前"行数未证实"）：bilinear_sample 索引行/列各自 clamp 到
   `0x1ff`（511），行距 SHL 9 = ×512 → 512 行 × 512 列 = 262144 单元（composite/scratch 各 0x40000B）。
2. **u8→f32 归一 = /255.0**：bilinear_sample 四角样本 `CVTSI2SS` 后 `DIVSS [0x14293ac70]`
   （radare2 核对 = 0x437f0000 = 255.0f）。消费侧取 [0,1] 水位，阈值 0.5。
3. **rasterize_stroke 三闭包体边界 = 内联进 8 个 rasterize_write/rasterize 实例**；写公式已提取
   （env0 = max、env1 = min、env2 = 饱和加×笔刷），选层（目标缓冲）由 arg2 决定，实时=scratch。
4. **update_water_audio_grid** 完整解码（见 §2）。

## 模块骨架（硬证据，B VA）

```text
system_water::draw_water::draw_water                      0x141ba9130（body 1044B）
   ← 调用者：FunctionSystem 包装 run_unsafe（ECS），见 reva callers
   → 调 draw_raster<WaterRasterStagingSsbo,WorldRaster<512>,closure_env$1/2/3>（0x142260ae0）

system_water::water_audio_grid::update_water_audio_grid    0x141baefe0（PDB code_size 2477，
   body 1586B；源行 21–106）
   ← 调用者：run_unsafe<FunctionSystem<(Local<WaterAudioGridUpdateState>, Res<WaterRaster>,
      Raycasting, Res<MainCamera>, ResMut<WaterAudioGrid>)>>（0x140e93830）
   （注册：system_water::impl$0::build 0x1406501d0，run_if Option<Res<WaterAudioGrid>>）

zeno::mask::Mask::blend_into<&[Command], rasterize_stroke::closure_env$N<512,Map<Iter<Vec3>,
   iter_points_xz::closure_env$0>>>
   $0 = 0x1425dcda0；$1 = 0x1425dc570；$2 = 0x1425dbd00
   ← 均仅由 rasterize_stroke<512>（0x142261d10）调用（调用点 0x1422622b0 / 0x1422622c8 /
      0x14226228f）

闭包体内联进（游戏闭包 world_raster::rasterize_stroke::closure_env$N 是 zeno 渲染闭包的
写入回调，被内联；边界地址）：
  HeapStorage 实例：0x142821d90（env0）/ 0x142828b30（env1）/ 0x142823e50（env2）
  （另各 blend_into 还分派到 0x142828570/0x142825010/0x142827090/0x1428269a0/0x142822950/
    0x14282c3a0/0x14282d2f0（env0），0x142825ca0/0x142824a50/0x142823780/0x1428255d0/
    0x142827760/0x14282db80/0x14282b220（env1），0x142822350/0x142824450/0x142826260/
    0x142827e30/0x142823040/0x14282cb40/0x14282bad0（env2）—— 其余 7 个分派目标）
```

## 1) draw_water（0x141ba9130）——实时水面笔刷入口（重确认）

工具 id 分发（reva 反汇编 0x141ba9175–0x141ba91a0）：
```text
R11D = *(uint *)*RCX            ; 工具 id
ESI = R11 - 6; if ESI < 0xf → R10 = R11 - 5    ; 仅 id∈[6,21] 进入
R10 == 4（id 9） → eraser 分支
R10 == 0xa（id 15）→ water 分支；其余直接返回
```

water 分支（id 15）：
1. `RAX = *[RBP+0x140]`（BrushSizes 资源）→ `RAX = [RAX+0x20]` → **`XMM6 = *f32[RAX+0x4]` =
   BrushSizes.water**（+4 字段；set_water clamp[1,5]、reset_water 默认 2.0 见 water-cpu-analysis
   0x1409f18a0/0x1409f1890）。NaN 检查（`&0x7fffffff` vs `0x7f7fffff`）。
2. `clearing_canvas("watereraser_water_")` → `CursorTerrainRaycast::position()`×2 →
   `Circle2d::new(center, radius = [0x142a16cb8] * brush)`（半径=笔刷×常量）→
   `GladeBorder::is_shape_inside`（界内校验）。
3. 界内：写 `EventInstance<WriteSettingsFileCmd>`；`into_drag_state`；`DragState::is_some`
   再写一个 WriteSettingsFileCmd。
4. `DrawState::update`；switch(DrawState) 处理 ShowPinnedDecoClutter 等；推 WriteSettingsFileCmd；
   `ScheduledTerrainReadback::on_terrain_modified`；`TweakerView::as_inner_mut`（笔刷大小）；
   **调 draw_raster<Water…>**。

eraser 分支（id 9）：`inverse_lerp(brush, …)` → `f = 4.0*lerp + (1-lerp)`（0x40800000/0x3f800000，
  常量 [0x142934694]）→ 建 SignifierFactory / UiSignifierStream::write（擦除提示）。
（eraser 分支本身不调 draw_raster 的同一路径；真实擦除走 draw_raster 内 byte@+0x64 区分。）

**笔刷事件→water_rasterize 触发链（入口组织方式，全为 B VA）**：
```text
draw_water(0x141ba9130) 读 BrushSizes.water(+4) + 光标 + 界内校验
  → draw_raster<Water…>(0x142260ae0) 状态机：
     state0 start_new(cursor)；state1：InProgressRasterStroke::last 距离>0.0005
       → add_point → resample_and_smooth → DirtyTiles::clear → iter_points_xz
       → rasterize_world_space_stroke → DirtyTiles::extend×2 → 交换 512B 脏块位图
       → copy_composite_to_scratch<512> → iter_points_xz → rasterize_stroke<512>
       → SSBO lookup(WaterRasterStagingSsbo) → map → copy_to_staging<512> → 事件+原子计数；
     其余状态：commit_scratch_to_composite<512> → SSBO → copy_to_staging
  → copy_to_staging<512>(0x142261760) 置 +0x29a=1（published）
  → render_world(0x1404ec110)@0x1404ec2f6 → water_rasterize_from_cpu(0x140a12680)：
     若 WaterRasterStagingSsbo BufferState.+0x29b ≠ 0 → 写 +0x5c、清 +0x29b、
     memset(+0x98,0,0x200)、建纹理跑 4 compute pass 上传。
```
**+0x29b 置位点未在本批定位**（copy_to_staging 只置 +0x29a；water_rasterize_from_cpu 只清 +0x29b；
draw_raster/replay_raster/三闭包/publish<DirtyTiles> 均不写 +0x29b；DirtyTilePublisher::unique
0x1409393b0 只是 38B 描述符构造）。判定：由 WaterDraw/Erase 事件消费方或 DirtyTilePublisher
系统置位，标 unknown（不伪造）。

## 2) update_water_audio_grid（0x141baefe0）——音频网格 CPU 更新（新证据）

签名（build 注册 + run_unsafe 包装一致）：
`(Local<system_water::water_audio_grid::WaterAudioGridUpdateState>, Res<WaterRaster>,
  Raycasting, Res<MainCamera>, ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)`
run_if `Option<Res<WaterAudioGrid>>`（资源存在才跑）。

流程（reva 反编译 + 反汇编 0x141baefe0–0x141baf50c）：
1. `_DEV_OVERRIDE_IS_DEMO_MODE == 1` → 早退。
2. `Local<WaterAudioGridUpdateState>` 是 **u64 帧计数**：`lVar6=*Local; lVar2=lVar6+1;
   *Local=lVar2`；`uVar12 = lVar6 + lVar2*10 + 1`；**本帧测试位 = uVar12 & 63**（
   每帧按帧号在 64 位掩码中轮换采样位）。
3. `MainCamera::eye_position`；遍历 `Vec<WaterAudioGridCell>`（每条 **0x18B**，lVar7 条）：
   - 每条单元布局（反汇编偏移）：+0x0 位置(x,z) 2×f32；+0x8 u64 水位位掩码（64 帧滚动窗）；
     +0x10 u32 = 2×u16 计数（PADDW/PSUBW 对 +0x10 按 {sVar16,sVar18} 增减）。
   - 距离判定：`XMM12 = [0x142eed900] = 6400.0`（=80²，音频半径阈值）。dist²≥6400 → 走清除。
   - 采样：`bilinear_sample_at_world_xz<RasterBufferRef<512>>(cell.xz, scale=130.0
     [0x142a7b658])`（采样参数 `[RSP+0x20]=0x43020000`）；`XMM13 = [0x142925870] = 0.5`：
     **样本 ≤ 0.5 → 该帧该位判无水面**（u8 归一后阈值）。
   - 水面判定：距离<80 且样本>0.5 → `Raycasting::intersects_any(ray 相机→格心,
     XMM7=1.0, R9D=0x32a3)`（地形遮蔽检测）：
     - 无相交（AL==0，水面可见）→ **置位 `mask |= 1<<bit`**；若该位原为 0（新置）→
       `+0x10 计数 PADDW +{sVar16,sVar18}`。
     - 有相交或距离/采样不满足 → **清位 `mask &= ~(1<<bit)`**；若原为 1 → `+0x10 计数 PSUBW`。
4. 即**音频网格 = 每格 64 位滚动窗 + 双 u16 连续"入水"计数**；water_ambience_audio_system
   （0x140873630）消费这些计数驱动水面音效。

常量（radare2 RNE 核对）：0x142eed900=6400.0、0x142925870=0.5、0x142925904=1.0、
0x142a7b658=130.0。位掩码增量为 1<<(uVar12&63)，清除掩码 = 其余 63 位。
[unknown] WaterAudioGrid 资源整体布局（网格行列数、单元排列）PDB 无 fields；仅单元 24B 布局
与遍历方式由反汇编证实。

## 3) 水位 u8 换算（world_raster 栅格语义，新证据）

**栅格 = 512×512 u8**（bilinear_sample_at_world_xz 0x140a14df0 反汇编：行/列索引各自
`CMP …,0x1ff` clamp 到 511；`SHL 9`=×512 行距；`ADD …,0x200/0x201` 邻域）。

**u8→f32 = /255.0**（四角样本 MOVZX→CVTSI2SS→`DIVSS XMM3,[0x14293ac70]`；radare2 核对
0x437f0000=255.0f），随后三次 lerp。**消费侧水位 ∈ [0,1]**。

**写入语义**（游戏闭包体，见 §5）：栅格值 = 笔刷覆盖度累加：
- env0（max）：`dst = max(dst, alpha)`
- env1（min/擦除）：`dst = min(dst, 255-alpha)`
- env2（加权加）：`dst = saturating_add(dst, alpha*brush_radius_fixed >> 16)`
其中 `alpha = 覆盖度 >> 9`（覆盖度 iVar 为定点，右移 9 得 u8 0..255，clamp 0xff）。

**阈值**：音频网格以归一样本 0.5 判"入水"（≈u8 128）；water-cpu-analysis 另记 0.0005
（笔画点位移）等。**水位/水深的精确物理语义（u8 值=depth vs 覆盖度）未证实**，仅确认
0=无水、值随笔刷累加、消费侧 /255 归一。
[unknown] 初始水位块（WorldRaster::new 填 0x4000B 重复 u64 对）解码仍未知。

相关常量（radare2 hexdump 核对）：
```text
0x14293ac70  255.0f   （u8→f32 归一除数）
0x1429a3c84  512.0f   （z 格距；rasterize_stroke .rdata）
0x1429a3c80  512.0,512.0（SIMD 路径 xz 格距）
0x1429a3cb0  512.0f   （索引格距）
0x142a7b658  130.0f   （world→grid 除数）
0x142925870  0.5f     （音频入水阈值）
0x142925904  1.0f     （clamp / 采样上界）
0x142eed900  6400.0f  （音频网格距离²阈值 = 80²）
0x142aa50e0  65536.0f （env2 笔刷 16.16 定点缩放）
0x142934694  4.0 / -0.0 / -2.0（draw_water eraser lerp / 闭包 env 常量）
```

## 4) WaterMaskSsbo / WaterExcludeClutterMaskSsbo（PDB 仅名称，如实标注）

PDB structures（skeleton/B/crates/country-core/src/resources/{water_mask,
water_exclude_clutter_mask}.rs.structures.json.gz）：
```text
country_core::resources::water_mask::WaterMaskSsbo
  definition_status: name_and_relations_only | fields: unknown | size: unknown | align: unknown
country_core::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo（同）
```
生命周期（名称证据）：`MappedStorageBuffer<…>` 包裹、`AssetSsboLibrary::add_new` /
`get_rg_buffer`、`ComputePass::write_storage_buffer`、`pass_resource_access::write_access_buffer`、
`drop_in_place<MappedStorageBuffer<…>>`、`core::any::type_id`、`impl$1::as_any(_mut)`、
`render::ssbo::impl$4::new_storage_buffer`。**fields/size/alignment 一律 unknown，不复原。**
语义沿用 water-cpu-analysis：WaterMask=水面 mask（水位栅格上传产物）；WaterExcludeClutterMask=
排除 clutter 的 mask。

## 5) rasterize_stroke 三闭包体边界（选层语义；zeno 内部只标边界）

rasterize_stroke<512>（0x142261d10）构造三个游戏闭包 `rasterize_stroke::closure_env$N<512,
Map<Iter<Vec3>,iter_points_xz::closure_env$0>>`（$N=0/1/2），由两标志选路径（0x142262221–
0x1422622c8）：
```text
arg6（byte@ActiveRasterStroke+0x64）=0,  arg7 bit0=0 → blend_into<…,closure_env$1>(0x1425dc570)
arg6≠0,                                  bit0=0 → blend_into<…,closure_env$0>(0x1425dcda0)
arg6≠0,                                  bit0=1 → blend_into<…,closure_env$2>(0x1425dbd00)
arg6=0,                                  bit0=1 → panic（非法组合）
```
（arg6/arg7 来自 draw_raster 调用点 0x142260fb7–0x142260fe7 的 `[RDI+0x64]` byte / `[RDI+0x68]`
u32，RDI=ActiveRasterStroke；+0x64 在 draw_raster 尾按 0/非0 推 WaterDraw/Erase 事件。）

**选层（目标缓冲）**：闭包捕获的目标 = `*(WorldRaster+0x8|+0x20)` 数据指针与
`*(+0x10|+0x28)` 长度，由 **arg2**（EDX）低字节选：`CMOVNZ RAX, mask+0x20`（0x142262196–
0x1422621a5）。**实时 draw_raster 以 EDX=1 调用 → 目标 = scratch（双缓冲工作层）**；
release/回放路径再 commit_scratch_to_composite<512> 或 copy_to_staging 发布。

**闭包体公式**（内联进 `rasterize_write<HeapStorage,…>` 实例；`alpha = 覆盖度>>9` clamp 0xff）：
```text
env$1（HeapStorage 0x142828b30）：dst = min(dst, 255 - alpha)        ；擦除
env$0（HeapStorage 0x142821d90）：dst = max(dst, alpha)              ；硬写入
env$2（HeapStorage 0x142823e50）：dst = saturating_add(dst, alpha*brush>>16) ；加权加
```
env$2 是唯一接收笔刷半径指针的（blend_into 第 5 栈参 = &(clamp(brush,0,1)*65536 定点)，
0x142262232–0x14226226c，常量 1.0/0x142925904、65536.0/0x142aa50e0）。zeno 依赖内部
（命令遍历、覆盖度定点、storage 分派）**只标边界不复原**。

## 关键 unknown（不伪造）
- +0x29b（WaterRaster/SSBO 再上传脏字节）置位点未定位。
- WaterMaskSsbo / WaterExcludeClutterMaskSsbo 字段（PDB 仅名称）。
- WaterAudioGrid 资源整体网格尺寸（单元 24B 布局已证实；行/列数未证实）。
- 水位 u8 的"深度/密度"物理语义、初始水位块解码。
- ActiveRasterStroke 完整布局（+0x60/+0x64/+0x68/+0x6c 语义名未定：笔刷/擦除/模式/权重）。
- zeno 覆盖度定点的精确刻度（仅知 >>9、×65536）。
