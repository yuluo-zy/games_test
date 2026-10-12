# G02：B 自由墙 base cache 与实际预览 shape-height 消费者

日期：2026-10-11。实现与行为验收仅 B。原模块归属按 A/B/C source/PDB 与 `tracking/dependency-exclusions.json` 核对，两模块均为 `building_business_candidate`；见 `three-version-dependency-boundary.json`。A/C source checksum 不同，只建立出处，不授予跨版本等价。

已交付 `reconstruction/building/wall_freeform_cache.rs`、`src/bin/wall-freeform-cache-probe.rs`、`scripts/multi-version/verify_wall_freeform_cache.py`。父任务负责共享 lib、真实 World 注册与源绑定，后续由父或另一个 agent 独立 review。未修改其他业务源码、Cargo manifest 或 lock。

## 一：init_wall_height_and_arch_segment_height 的完整已解析自由分支

原入口 B `14078b550`，证据复用 `../building-runtime/B-14078b550.asm.txt` 和 `B-focused/14078b550.c`。恢复从 PublicWalls 查找成功后的 `14078b7e7` 开始，包含原 third-array 初始化、shape guard、两个自由分支、两缓存写入、事件 loop 收尾及真实 RET；PublicWalls std lookup/EventReader 与整体调度不在此解析边界。

稳定 API：

```rust
freeform_wall_height_cache(
    prepared_positions: &[[f32; 3]],
    shape: &WallShapeSnapshot,
    constrained_extra: f32,
    clutter_version: u8,
    prepared_segment_count: usize,
) -> Result<FreeformWallHeightCache, FreeformCacheError>
```

结果含 `maximum_y`、`arch_segment_heights`、`segment_heights`；`write_projection` 只写两个已证 cache flag/value，数组由 typed Vec 所有，不重建 Vec/allocator ABI。

| 输入/输出 | 实际作用与原证据 |
|---|---|
| prepared_positions | 原 Public+90 ptr/+98 count。initial mapped Curve3+38..70 不参与此 cache 分支。 |
| prepared_segment_count | 原 Public+d0；14078b7e7..14078b854 按该 count alloc_zeroed(count*4)，替换 third Vec+f0/+f8/+100。不是 prepared point count。 |
| shape tag | 原 Public+258；14078b85b 的 `>=2` 分支。0/1 由已恢复 regular cache 处理，本接口明确拒绝。tag>=2 是控制流范围，未声称所有任意整数 tag 是合法 World 类型。 |
| clutter_version | 原 Public+295，来自原 InnerWall+66；真实字段与 source policy 的 ClutterVersion 一致，非通用生命周期/特殊 flag。默认值未猜测。 |
| constrained_extra | 原 Public+284。来自发布后约束标量，非 PublicMaxY。 |
| arch heights | Ptr+e0/count+e8，输出长度为 prepared+98。原清除旧 length 后写入；重复执行不积累前次高度。 |
| base/gable cache | 原+108/+10c、+110/+114，两 flag DWORD 写 1，值相同。自由分支结束直接返回 event loop，不进入 regular 的 Roof/gable 后段。 |

**ClutterVersion!=0**：prepared Y 以 -2.5 起点求原 maxnum；加 constrained_extra；MAXSS floor 为 bits `3f75c290`（约0.960000038）；整个 arch 数组写同值。这里没有 regular 分支的 -0.46。

**ClutterVersion==0**：每个 prepared Y 加 constrained_extra 后按原 MAXSS/MAXPS floor0.5；对所得 arch 数组以0起点求原 maxnum，两 scalar cache 写最大值。空 prepared 数组的这个分支返回 maximum0；uniform 分支仍执行 -2.5+extra/floor 的原规则。

third array 的 fresh zero 仅证明**本次 initializer 输出**。后续支撑/elevation writer 未排除；本模块不把 third 当永久零或用任意零替代未知 source。其 count 与 prepared count 的对齐仍必须由真实 segmentation owner 证明。

### 原码差分

`B-freeform-cache-verification.json`：**1536 cases，0 failures，1536 actual-return assertions**。覆盖 prepared point count0/1/2/3/7/8/9/15/16/17/31、两 ClutterVersion 路径、非均匀 Y、NaN/Inf/sign-zero/floor 邻域、independent segment count、旧 arch capacity/length 与非零旧 third 内容。

oracle 的 initial mapped Vec 明确另设为单个 ±1,000,000 Y 点；结果仍只由 prepared+90/+98 驱动。third count 独立变化，读取实际新 ptr/count 并逐元素确认 zero。两 flags/values、arch 值/length 和所有其他 Public 字段保持均核查。generic Vec reserve 指令真实执行，只有 std allocator/free/realloc 桥；alloc_zeroed 明确清零，不依赖初始 emulator 页为零。

## 二：calc_shape_height 真实前段与完整消费者

**它不是自由墙 handler**。原函数 `country_core::systems::wall::shapes::calc_shape_height::calc_shape_height` B `140ac80e0`，是圆/矩形预览工具消费者。首次匹配原 FunctionSystem metadata（`B-registration/report.json`，initialize `140b13050`）明确：

`Res<AppMode>, Res<PreviewRectangle>, Res<PreviewCircle>, Res<TerrainHeightsData<TerrainHeightSsbo>>, ResMut<ShapePlacement>, ResMut<TimberframeConstructionParamsCache>, Res<GameTime>`。

恢复 `calculate_shape_height(ShapeHeightInputs, &mut ShapePlacement observed68-byte field window, &mut TimberframeConstructionParamsCache observed16-byte window, two change ticks, current_tick)`。这些 byte windows 只声明本消费者实际读写字段，不声称完整原资源 ABI；未证 fields/padding 保持原字节。

原完整入口与 helpers 见 `B-shape-height/`、`B-shape-height-helpers/`。输入角色与规则：

- AppMode tag6 + subtype1(circle)/2(rectangle) 有效；其他 mode 仅写 Timberframe cache present DWORD0、mark其 tick，保留其他 payload，不修改 ShapePlacement/tick。
- Rectangle `140c98cf0`：实际 axes/center/dimensions 产生四角与重复首点；保留原角点各自乘加顺序和 width*length>0 guard。
- Circle `140c97cf0`：原 `ceil(radius*TAU/.2).max(3)` 点数；private closure 内 `u=index/(count-1)`、`phase=angle.rem_euclid(TAU)`、`phase+(1-u)*TAU`，cosf/sinf 后 `center+radius*direction`。第四 circle float 为角度，不能默认零。private arithmetic 在 std Map collector 内，只有这段游戏 point policy 被恢复；容器 collector 不计为 core。证据 `B-circle-private-point-policy.*`。
- 原 private remap `14146afe0` 按 .5 physical segment spacing，不强制 odd count，保留 endpoints；每点 Y 为 `maxnum(Terrain sample, ShapePlacement+3c)`。这个 +3c 下界原样参与，**没有 publisher 的正值选择/-2.7默认规则**。复用已恢复 `TerrainHeightGrid512` sampler 与 Curve2；原 Curve3 总长度/U 顺序与采样点也全部比较。
- 对 mapped Y 顺序求 sum 与从0起点的 maximum。subtype rectangle base6，circle base8。若 ShapePlacement+8 observed tag2，选择 base加 +0 byte判别的+4 scalar（byte0则加0）；否则选择 mean(Y)+base-.46。减去 maximum-.46，再按原 SSE MINSS/MAXSS clamp .7..14，写 ShapePlacement+40及tick。没有推测 tag2 的业务名称或默认值。
- +3c 等于0（含-0）时，Timberframe cache baseline 写 -2.5；否则保留+3c。world target为 `(extra+maximum)-.46`。无 prior value 时以 target 起步；否则以 cache+4 prior 起步。实际 `140c904e0` time_continuous_lerp 为 `previous+(target-previous)*(1-exp(-delta*10))`。写 present1、smoothed value、baseline与 value-baseline，tick行为分别核查。

### 完整原码与差分

`B-shape-height-verification.json`：**384 full-entry cases，0 failures**；32 guard cases、352 valid cases；**9847 preview point records / 13581 terrain mapped point records** 的所有点、U、length 均逐 f32 bits 一致。同时比较完整 observed ShapePlacement68/cache16 bytes、两 ticks、target/extra。384 次从 `140ac80e0` 执行到真实 RET，每次断言 RIP==sentinel，未被 emulator instruction budget 截断。

oracle 执行真正 circle/rectangle generation、terrain sampler、Curve2/Curve3 helpers、算术与连续 lerp；capture hook 只读取中间结果，不改指令/寄存器。桥仅为 generic allocator/free/realloc/memory 与**实际 IAT 名称校验**的 ceilf/cosf/fmodf/sinf/roundf/expf。每轮 PE import/thunk 校验落 `B-CRT-IAT.json`。

保留 `first-sincos-combination-failure.json`：初次14个 circle case 出现1ULP坐标差；case58 point0 X `bff85732` vs `bff85733`，继续影响 Curve2 length/U。release binary `nm -u` 证实 LLVM 把 Rust 相邻 cos/sin 合并成 macOS `___sincosf_stret`，而原 point policy 是分别 cosf/sinf。当前两个 `#[inline(never)]` wrapper 仅保留这两个原标准库调用边界，没有实现三角函数、改变 oracle 或放宽比较。当前 imports 为 `_cosf`、`_sinf`，全384精确重跑通过。

## 构建、复现与冻结

```sh
cd reconstruction
cargo build --release --bin wall-freeform-cache-probe
cd ..
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_freeform_cache.py
```

probe 中固定 terrain grid 为明确差分 fixture `(i*73+(i/512)*101)&65535`；生产 API 接真实 TerrainHeightGrid512，不固定地形。两个报告绑定原 untouched B RNE SHA256、candidate binary、相关 Rust/source/manifest/lock、rustc host与 verifier hash；校验前后相关来源不变。仅使用此 owned file-set，不把并发父任务的全部 workspace 当作本次证明。

## 验收边界与父任务下一联接

本次恢复了完整 resolved **B tag>=2 base-cache consumer** 与真正 **B preview calc_shape_height consumer**，未实现其原 SystemParam storage、原 scheduling或 UI state/default producer，不直接创建/更新 World 对象。

父任务须从 segmentation owner 提供真实 prepared Curve3 与真实 +d0 count 后接 freeform cache，再将实际 arch/third arrays 接后续砖/支撑消费者。不得用 initial terrain outline 值代替 prepared，也不得为了满足 producer 的 array count 条件裁短/填零。地形对 freeform cache 的作用通过 prepared Y 输入传递，cache 本身不再采地形。

calc_shape_height 的源对象是 **ShapePlacement**，其+40输出不能未经真实后续 consumer 直接解释成 RawWall.extra/source+54。需要独立恢复 shape-tool后续权威对象生成/提交，才能算实际源码链联接。

仍不授予 prepared segmentation producer/相交所有权、自由墙完整编辑与删除/历史、后续 third-array writer/arch结构、A/C行为、原 fork/GPU/视觉验收。已有 ARM roof filler 1ULP失败与此次 circle库调用边界修复是不同报告，不自动关闭。
