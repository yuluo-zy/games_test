# system-water CPU 状态链业务逻辑分析（B 主基线，反汇编级证据）

2026-10-13。聚焦水域 CPU 状态链：水位栅格初始化、CPU 栅格化、历史回放恢复水位。
数据源仅 B patched EXE（sha256 cf048e27…，Ghidra/ReVa 持久库）+ 发行 RNE 原件
（sha256 f507a6ce…，radare2 机器码 oracle）。PDB 行号表：skeleton/B/crates/**/*.pdb.jsonl.gz。

证据来源（本次精读）：
- **reva**（B 持久库，worker `water-owner`，只读 run-script）：下述每个函数反编译 + 全反汇编 +
  调用目标 + 内存引用；输出落盘 `ghidra_scripts/water-owner/*.json` 与
  `evidence/toolchain/water-cpu/`。
- **radare2**（RNE oracle）：各函数入口头窗口反汇编与 B/reva 逐条一致（startup_water /
  water_rasterize_from_cpu / rasterize_stroke<512> / replay_raster<Water> / bilinear_sample
  均核对），确认地址/指令未被 B patch 改动；另读 .rdata 字符串与浮点常量。
- **PDB 行号表**：见各节。

## 模块骨架与调用者（硬证据）

`system-water-inventory.json` 已修正 VA。本次确认/补充的关键泛型签名（Ghidra 符号名）：

```text
country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
    0x140a12680   ← caller: render_world @0x1404ec2f6（0x1404ec110）

country_core::systems::world_raster::draw_raster<
    WaterRasterStagingSsbo, WorldRaster<512>,
    system_water::draw_water::draw_water::closure_env$1/2/3>
    0x142260ae0（size 2214）   ← caller: draw_water（0x141ba9130，尾块）

country_core::resources::world_raster::impl$2::rasterize_stroke<
    512, Map<Iter<Vec3>, raster_stroke::impl$2::iter_points_xz::closure_env$0>>
    0x142261d10（Water 专用实例，size 2407；同源码模板实例 0x140a16960）
    ← caller: draw_raster<Water> @0x142260fe7；replay_raster<Water> @0x1409f4…（模板 0x140a16960）
copy_to_staging<512>           0x142261760 ← draw_raster @0x142261065/0x142261290；replay_raster
copy_composite_to_scratch<512> 0x142262680 ← draw_raster @0x142260faa；replay_raster（模板 0x140a161a0）
commit_scratch_to_composite<512>0x142262a60 ← draw_raster @0x1422611f7；replay_raster（模板 0x140a16580）

country_core::systems::history::replay_raster::replay_raster<
    WaterRasterStagingSsbo, WorldRaster<512>, replay_water::replay_water::closure_env$2>
    0x1409f4650（size 10654/8329）
    ← caller: run_unsafe<FunctionSystem<replay_raster::RasterReplayResources,
       EventWriter<WaterDrawEvent>, ResMut<WaterRaster>,
       EventWriter<RecalculateHeightMapCmd>, Res<ScheduledTerrainReadback>>>
    （PDB 亦见 replay_raster 泛型为 Path/Garden/Cobblestone1-3 等栅格共用）

history_process::generate_replay_cmds_from_edit<
    replay_water::replay_water::closure$1::closure_env$0>
    0x141444c70（PDB size 10371；Ghidra 只见 60B 跳表 stub）
    ← caller: call_once<replay_water::closure_env$1, (EditType, &mut Vec<ReplayCmd>)> @0x14083c110

replay_water::replay_water::closure$2  0x140a01560（191B）
bilinear_sample_at_world_xz<RasterBufferRef<512>>  0x140a14df0（835B）
BrushSizes::set_water 0x1409f18a0（30B）/ reset_water 0x1409f1890（8B）
WorldRaster::new<512> 0x140a15530；WorldRaster::subscribe<512> 0x140a158e0
system_water::impl$0::build 0x1406501d0（注册 record_water_history 等系统；无 caller）
```

record_water_history 主逻辑被内联进 Bevy FunctionSystem（PDB 仅 inlinee 记录，无独立 S_GPROC32，
与清单 notes 一致），本次未取到其独立地址；由 build 注册参数
`(RasterHistoryRecordResources, EventReader<RecordWaterEditToHistoryCmd>)` 佐证。

## 1) startup_water（0x1408f1a20，PDB size 3632 / body 3223）

源文件 `crates/country-core/src/startup/startup_water.rs`，PDB 行号 19–227
（skeleton/B/crates/country-core/src/startup/startup_water.rs.pdb.jsonl.gz）。

业务步骤（reva 反编译 + radare2 核对入口头）：
1. 开头判 `_DEV_OVERRIDE_IS_DEMO_MODE`（byte @0x143634d90）：非 0 → 整函数 no-op 直接返回。
2. 逐资源调用 `load_json_as_mesh<&str>`（0x140b0eeb0）→ `unwrap_failed` → `load_mesh_into_library`
   （0x140b1aca0）；`load_shader_into_library<&str>`（0x140b3fe40）/
   `load_shader_into_library_replace_previous<&str>`（0x140b405e0）。
   - .rdata 字符串（0x142aa7a10–0x142aa7d70，radare2 读）：
     meshes `assets/meshes/water_plane.json / lilypad.json / fallen_leaves.json /
     lilypad_flower.json / water_grass_winter.json / reed.json / reed_autumn.json /
     water_stones.json`；shader 名 `water_plane/water_plane.raster.hlsl / ice_plane.raster.hlsl /
     water_plane_refraction_copy.raster.hlsl / water_snow_plane.raster.hlsl / lilypad.raster.hlsl /
     reed.raster.hlsl / lilypad_flower.raster.hlsl / water_x_paths.raster.hlsl / lilypad_shader /
     lilypad_flower_shader …`
3. 每次装载后用 `Commands::spawn_empty`（0x140d146a0）压入 `insert<DrawableMeshBundle
   (+DontCastShadow, +DesignatedRenderPass)>` 等命令；命令槽（0x88/0x78B）字段写入：
   transform scale=1.0（0x3f800000）、translation、mesh handle @+0x38/+0x40、
   render-pass 标签 u32 @+0x48（0xf/0xe/…）、byte @+0x58=0、panic handler @+0x78。
4. 依据 `*(char*)(lVar2 + 0x58) == 4` 条件性多 spawn 一组 mesh（DrawableMeshBundle 少组件版本）。

**角色修正**：startup_water 是水网格/着色器资源装载 + 水面实体 spawn，**不是**水位 mask 的
CPU 初始化；水位 CPU 状态在 `WorldRaster<512>`（WaterRaster 资源）中，见下节。

## 2) WorldRaster<512> 布局与 CPU 栅格链（world_raster.rs 275–541 行）

PDB 行号表（world_raster.rs.pdb.jsonl.gz）：`clear_buffer<512>` 0x140a15a10（275–285）、
0x140a15da0（287–307）、copy_composite_to_scratch 0x140a161a0（309–316）、
commit_scratch_to_composite 0x140a16580（318–325）、rasterize_stroke 0x140a16960（327–403）、
copy_to_staging 0x140a172d0（409–541）；Water 专用实例在 0x142261760–0x142262a60。

### 字段布局（证据：copy_*/commit/copy_to_staging 的偏移读写）
| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x08 | composite 数据指针 | memcpy(ptr+8, ptr+0x20, len@+0x10) |
| +0x10 | composite len（字节） | len 比对 / 拷贝长度 |
| +0x20 | scratch 数据指针 | memcpy(ptr+0x20, ptr+8, len@+0x28) |
| +0x28 | scratch len（字节） | len 比对 |
| +0x30 | DirtyTiles/retain 闭包状态 | retain 参数 RDX=&self+0x30 |
| +0x50..0x5f | 16B（dirty tile 元数据） | MOVUPS [self+0x50] → retain 闭包 |
| +0x60/+0x68 | parking_lot Mutex（Arc 结构） | lock_slow/acquire_resource 前后 |
| +0x98..+0x297 | 512B staging/脏区 | memset 0x200 / memcpy 0x200 / replay OR 位图 |
| +0x298 | 读缓冲选择字节（0→composite, 1→scratch） | CMOVNZ 选择 |
| +0x299 | staging 清零脏字节 | copy_to_staging 开头 |
| +0x29a | published 字节 | copy_to_staging 尾置 1 |
| +0x29b | 再上传脏字节 | water_rasterize_from_cpu 判 0 |

栅格单元：`bilinear_sample_at_world_xz<RasterBufferRef<512>>`（0x140a14df0）反汇编以
`MOVZX byte ptr [buf + row*512 + col]` 读样本，行距 `SHL …,0x9`（×512）、邻域 `ADD …,0x200/0x201`。
**栅格为 u8 单元、512 列**（行数未证实，≥513 行）。`WorldRaster::new<512>`（0x140a15530）分配
composite+scratch（alloc_zeroed）并填一块 0x4000B 缓冲为重复 u64 对
{0x07954dbb_07954dbb, _UNK_142ac7e58}——初始水位值（原值/解码语义 unknown，接近 0/空水）。

### rasterize_stroke<512>（0x142261d10）——笔画→栅格命令
签名暴露输入迭代器：`Map<Iter<Vec3>, raster_stroke::impl$2::iter_points_xz::closure_env$0>`
（Vec3 点经 iter_points_xz 取 xz）。入口 R8/R9=点切片 begin/end（radare2 见 `sub r15, r8`）。

世界→栅格变换（反汇编常量，radare2 读 .rdata 核对）：
```text
grid = (world / 130.0 + 0.5) * 512.0
  130.0 = 0x43020000（读 @0x142a7b658 亦为 130.0）
  0.5   = 0x3f000000
  512.0 = 0x44000000（x 用立即数；z 用 .rdata 0x1429a3c84 = 512.0f）
```
即世界窗口约 130 单位 → 512 栅格，原点对应格 256（居中窗口）。

生成 `zeno::command::Command` 列表（stride 28B，prealloc ≈ n/3 * 28，n=点数）：
- 首点命令 `{variant=0（move-to）, gx,fz}`（2×f32 栅格坐标）；
- 后续每点 `{variant=1（line-to）, gx,fz}`。
随后按两个标志选 blend 路径（bit0 与 byte@0x30）：
`(0,0)→closure_env$1；(0,1)→closure_env$0；(1,0)→panic；(1,1)→closure_env$2`，
调 `zeno::mask::Mask::blend_into<&[Command], closure_env$N>`（0x1425dcda0/$0、
0x1425dc570/$1、0x1425dbd00/$2）。

`blend_into` 读 Mask 结构（+0x0..0x20 参数、+0x28 命令数、+0x30 存储指针、+0x20 借用标志），
按命令数/存储类型分派到 `zeno::raster::rasterize_write<HeapStorage|AdaptiveStorage,…>` 或
`rasterize<…>`，把游戏闭包 closure_env$N 作为写入回调内联。**zeno 为依赖**，实际逐像素写入
（笔刷权重、mask 合成）在闭包体（内联进 0x142821d90 等 rasterize_write 实例）中；本轮未逐值
提取闭包体，标 unknown。

### copy_composite_to_scratch<512> / commit_scratch_to_composite<512>
纯 memcpy（len 校验 `+0x10 == +0x28` 不符则 `len_mismatch_fail`）：
`composite(+0x8)→scratch(+0x20)` / `scratch→composite`，长度取各自 len。双缓冲。

### copy_to_staging<512>（0x142261760）
1. 若 `+0x299` 非 0：把目标 staging 区清零（memset 0x200）并清 `+0x299`。
2. `+0x298 = 选择字节`（0→读 composite，1→读 scratch）。
3. `DirtyTiles::retain<copy_to_staging::closure_env$0<512>>`（0x1425da150）按脏块迭代复制。
4. `memcpy(选中缓冲, self+0x98, 0x200)` + `publish<DirtyTiles>`（0x1425db920）；
   `+0x29a = 1`。
即把 CPU 计算出的 512B staging 内容发布进当前读缓冲并发布脏块（GPU 消费方见 water_rasterize_from_cpu）。

### water_rasterize_from_cpu（0x140a12680）——CPU staging→GPU
读 WaterRasterStagingSsbo（type-id 0x1ad182936286a0ee）：若资源存在且 `+0x29b` 脏：
写 `+0x5c = world+0x5b0`、清 `+0x29b`、`memset(+0x98,0,0x200)`，然后建纹理并跑 **4 个 compute pass**
（每 pass：bind TexturePreset / bind Handle<Texture>、write_uniform<f32>、write_uniform<i32>、
render dispatch；dispatch 常量 `{1,2,2}` / `_UNK_142aa2508`），把 CPU staging 上传成水 mask 纹理。

## 3) 水 mask 资源（GPU 侧，字段 unknown）

`country_core::resources::water_mask::WaterMaskSsbo`（water_mask.rs）与
`…::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo`（water_exclude_clutter_mask.rs）：
- 均为 `MappedStorageBuffer<…>` 包裹的 GPU SSBO（PDB structures：name_and_relations_only，
  **fields/size/alignment unknown**）。
- 生命周期：`AssetSsboLibrary::add_new`（注册）、`get_rg_buffer`（取句柄）、
  `ComputePass::write_storage_buffer`（GPU 写）、`pass_resource_access::write_access_buffer`。
- 语义：WaterMask = 水面 mask（CPU 水位栅格上传产物）；WaterExcludeClutterMask = 排除
  clutter/装饰物的 mask（水域内不摆 clutter；`systems/clutter/add_lantern_water_setdresssing.rs`
  相关）。draw_raster 里对 SSBO 断言 type-name/magic（`extraout_RAX[3]==0x42` 且
  memcmp 0x42B 名称）为 `world_raster_defs::WaterRasterStagingSsbo`。

## 4) 历史回放恢复水位

### draw_water（0x141ba9130，size 1044）——实时水面编辑入口
判 demo mode → 按工具 id 分支（`*in_RCX` u32）：水工具分支（lVar9==10）：
`fVar12 = BrushSizes.water（self+4，clamp[1,5]，默认 2.0）`；`UiState::clearing_canvas`；
`CursorTerrainRaycast::position`；`Circle2d::new`；`GladeBorder::is_shape_inside` 校验在界内；
写 `WriteSettingsFileCmd` 事件；末尾 `TweakerView::as_inner_mut`（取笔刷大小）后
**调 draw_raster<Water…>**。

### draw_raster<Water>（0x142260ae0，size 2214）——实时链编排
`ActiveRasterStroke::index_mut` → 状态 0：`start_new`（cursor 位置）；状态 1：
`InProgressRasterStroke::last` 距离 > ~0.0005 → `add_point`、`resample_and_smooth`、
`DirtyTiles::clear`、`iter_points_xz`、`rasterize_world_space_stroke`、`DirtyTiles::extend`×2；
交换双份 512B 脏块位图（+0x70..+0x268 ↔ +0x270..+0x468，共 64 u64）；
`copy_composite_to_scratch<512>` → `iter_points_xz` → `rasterize_stroke<512>` →
SSBO lookup（WaterRasterStagingSsbo）→ map 写 → `copy_to_staging<512>` → 发事件 +
`WriteSettingsFileCmd` + 原子计数。

### replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, closure_env$2>（0x1409f4650）
系统参数（FunctionSystem 包装签名）：`RasterReplayResources, EventWriter<WaterDrawEvent>,
ResMut<WaterRaster>, EventWriter<RecalculateHeightMapCmd>, Res<ScheduledTerrainReadback>`。
流程（reva 反编译）：
1. 迭代 `Vec<ReplayCmd>`（48B/条）缓冲；每条 = 一笔划（点列 + 标志 + 权重 f32@+0x14 +
   `+0x19` 处理标记、`+0x1a` 事件标记、`+0x18` 清除标志 byte）。
2. 清除记录（`+0x18` 非 0）：`clear_buffer<512>` 后重放嵌套命令；普通记录：
   `cubic_in_out` 缓动 → `Curve::try_new_from_points<Vec3, CurveCoordSemantic3D>` +
   `slice_into` 重采样 → `copy_composite_to_scratch<512>` → `rasterize_stroke<512>` →
   `rasterize_world_space_stroke`（脏块位图 OR 进 `WaterRaster+0x98..0x290`，64 u64）→
   SSBO lookup（WaterRasterStagingSsbo）→ map → `copy_to_staging<512>`；
   权重 fVar5≥0.01 时进入 blend 分支（`new = brush*scale + old`，归一 clamp，`fVar36>=1.0`
   时 `commit_scratch_to_composite<512>`）；`closure$2`（内联）发事件 + 原子计数。
3. 处理完按 `Vec<usize>` 记录的下标 memmove 移除已处理条。
**水位恢复 = 把存档的 ReplayCmd 笔画再跑一遍同一条 rasterize_stroke→staging→composite 链**，
最终 `copy_to_staging` 后由 water_rasterize_from_cpu 上 GPU。

### generate_replay_cmds_from_edit<replay_water>（0x141444c70）
Ghidra 只见 60B 跳表 stub：`mov rax,[RCX]; movsxd rax,[表(0x142c5f2fc)+rax*4]; jmp`
——按 `EditType` 枚举判别值经跳表分发到各编辑处理（水分支内联在 10371B 主体内）。
调用方 `call_once<replay_water::closure_env$1,(EditType,&mut Vec<ReplayCmd>)>`（0x14083c110），
即 replay_water 的 `closure$1` 把存档 EditType 转成 ReplayCmd 入队。

### replay_water::closure$2（0x140a01560，191B）
push 两个 Vec（grow_one：`Vec<(DecoratorId,…)>` 与 `Vec<EventInstance<WriteSettingsFileCmd>>`，
Ghidra 类型标注可能为泛型误读）+ LOCK 原子计数自增。即"回放完成"通知/计数闭包，
replay_raster 内联同一代码。

## BrushSizes（笔刷尺寸）
`set_water`（0x1409f18a0）：`self+4 = clamp(value, 1.0, 5.0)`（`0x3f800000`/`0x40a00000`）；
`reset_water`（0x1409f1890）：`self+4 = 2.0`（0x40000000）。水笔刷默认 2.0、上限 5.0。
（水位采样/笔刷相关阈值常量：0.5 @0x142925870、0.9 @0x142a7c88c、0.0005 @0x3a83126f。）

## 关键 unknown（不伪造）
- WorldRaster<512> 栅格行数 / 总尺寸（u8 单元、512 列已证实；`new` 初始填 0x4000B 块的角色未定）。
- rasterize_stroke 的 3 个 blend 闭包体（closure_env$0/1/2）逐像素写入值、笔刷权重公式（zeno 依赖内）。
- WaterMaskSsbo / WaterExcludeClutterMaskSsbo 字段布局（PDB 仅名称）。
- 水位数值范围/码值（u8 0..255 的具体语义、fVar5 权重与栅格值的换算）。
- record_water_history 主逻辑独立地址（内联，未定位）。
- generate_replay_cmds 10371B 主体内各 EditType 分支细节（跳表 0x142c5f2fc 未逐项展开）。
- draw_raster 的 closure_env$1/2/3 三个实时编辑闭包体（water draw 模式）。
- ActiveRasterStroke / InProgressRasterStroke 完整布局。
