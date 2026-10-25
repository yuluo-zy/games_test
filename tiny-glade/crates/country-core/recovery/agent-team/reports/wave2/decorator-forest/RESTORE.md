# RESTORE — wave2 decorator/forest（restorer-wave2-decorator-forest，2026-10-17）

> 角色：restorer-wave2-decorator-forest（eco/state 组 wave2）。硬标准（真实逻辑非形状）：
> 逐函数 fresh 反编译核对 → 重建完整控制流 → Rust body 逐语句判等；重建不了标
> `未实现`（列出分歧点），不以近似通过。
>
> 本轮把 re-analyst-hardpoints-ecstate 的已解码逻辑实现进函数体：
> `calc_gothic_half_dormer_profile_ws`（0x1407eb160，profile 3 档点表+半径表）、
> `chimney_smoke_anim_thickness`（0x140818df0，更新公式还原）、
> `manage_tree_prefabs`（0x141bf16c0，has_construct_shape 门控补证落点）。
>
> 工具：ghidra/reva/radare2 `--worker shared`（只读）。fresh 反编译核对：
> - `jobs/ghidra-gothic.result.json`：calc_gothic_half_dormer_profile_ws（719B）+
>   closure$0（0x1407e7dd0）+ call_once closure_env_0（0x1412ac9b0）全量反编译。
> - `jobs/radare-gothic.result.json`：calc_gothic_half_dormer_profile_ws 反汇编
>   （0..2 for_each 双 closure 调用，0x1407eb274/0x1407eb282；尾部两 Curve 拷贝）。
> - `jobs/radare-callonce.result.json`：call_once closure_env_0 反汇编
>   （is_circle 分支 + 半径表 + 墙点 A/B/C/D lerp + 法线）。
> - `jobs/ghidra-consts.result.json`：DAT_142c2b180 = {0.25,0.3,0.29} read_memory。
> - 沿用 hardpoints-ecstate evidence：`reva_chimney_smoke_140818df0.c`（31KB，
>   更新公式 `new=clamp01(old+sign*dt/target)`）、`manage_tree_prefabs_141bf16c0.c`
>   （+0x18 处理字节、阶段 1 形状门控 `(A&&B)|bVar32`）。
>
> 验收：`cargo check -p system-decorator -p forest`。**含全部改动的首次 check ✅
> （40.23s Finished，0 error，约 08:20，当时 workspace 一致）**。其后 08:32 起其他
> restorer 在并发改 `crates/rhapsody` 与 `crates/country-core`（git diff 显示多文件在途；
> 本任务三文件经 git diff 核对为**仅有的改动**：system-decorator ×2 + forest ×1），
> 工作区处于编译流变（rhapsody/country-core 错误随其编辑游移，如
> `AttachmentCache.framebuffers`、`FrameInstance.history_validity`）。本任务不依赖
> rhapsody 的单机 check（`cargo check -p rhapsody` standalone ✅），错误全部位于
> 他组在途文件，非本任务代码；其恢复后重跑即可。

## 判定总表

| 函数（VA） | 判定 | 说明 |
|---|---|---|
| `calc_gothic_half_dormer_profile_ws`（0x1407eb160）| **重建+部分未实现** | profile 3 档点表（country-core 已落库，字节字核验）+ 半径表 {0.25,0.3,0.29} + max_y 截断（O 结构）+ 双 Curve 构建（0..2）；wall-space 逐点变换（A/B/C/D lerp + 法线，raw offsets 缺 country-core PublicWallState）+ 0..2 索引语义 **未实现** |
| `gothic_half_dormer_radius`（新增，call_once 内联）| **判等** | `is_circle()==Some && shape_kind==7 → 表[rank-1]，否则 0.38`；radare2 反汇编核验 |
| `chimney_smoke_anim_thickness`（0x140818df0）| **公式判等（FIX 后）+ 收集未实现** | 更新公式 `new=clamp01(old+sign*dt/target)` 还原（ReVa 全量）；目标 `r*3+(1-r)`/`r*4+2*(1-r)` 按 +0xd 分派、符号按 +0xd 与窗口；spot 收集主循环/rng 种子/retain 谓词/初始插入点 **未实现** |
| `manage_tree_prefabs`（0x141bf16c0）| **门控补证落点（FIX）+ 形状源未实现** | 阶段 2 门控位=`placements+0x18` 处理字节（以 cache.finished 承载，doc 已记）；阶段 1 墙圆查询改为恒运行（density==0&&rock==0 时），形状邻近判定仅在有构造形状时运行；构造形状源 ShapeParameters **未实现** |
| `get_gothic_hat_bones`（0x1407eb500）| 未实现（未动）| 本轮范围外；骨架主体待取证 |

## 1. `calc_gothic_half_dormer_profile_ws`（0x1407eb160）— decorator_visual/gothic_wall_window.rs

### 已解码并实现（fresh 核对）

- **profile 3 档点表**：`country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve(rank)`
  （coregap 已落库；`gothic_half_dormer_profile_curve_1409a4cc0.c` 字节字逐项核验，
  含**末点 h = -0.95**（0xbf733333，非 RE-ANALYSIS 转录的 0）——core 测试
  `half_dormer_points_match_observed_native_words` 已证字节一致）。
- **半径表**：`GOTHIC_HALF_DORMER_RADIUS = {0.25, 0.3, 0.29}`（DAT_142c2b180，
  read_memory 核验）+ `GOTHIC_HALF_DORMER_RADIUS_DEFAULT = 0.38`（0x142c2b0c8）。
- **半径选择（gothic_half_dormer_radius）**：`is_circle()==Some && shape_kind==7 &&
  rank∈1..3 → 表[rank-1]，否则 0.38`。radare2 0x1412ac9f6-0x1412aca29 核：
  `cmp [is_circle_tag],0; je default; cmp [shape_kind],7; jne default; movss xmm7,
  [table+idx]`。
  ⚠ **纠正 RE-ANALYSIS §4.4 简写**"is_circle? 0.38 : 表"方向有误——circle 且
  kind==7 才用表，非"圆用 0.38"。以反汇编为准，RESTORE 报告登记此分歧。
  ⚠ **rank∉1..3 越界登记（reviewer-wave2-geometry §3.1）**：`dec al; cmp al,3;
  jae 0x1412acc08` → 二进制 rank 不在 1..3 时走 **panic_bounds_check**；Rust 返回
  默认 0.38（低影响登记：rank 恒 1..3 输入域，未伪造越界路径）。
- **max_y 截断（closure$0 0x1407e7dd0，O 结构）**：逐点比较 `PublicWallState::max_y()`；
  `cur.y < max_y` → 在 prev→cur 段插 `y==max_y` 截断点
  （`t=(max_y-prev.y)/(cur.y-prev.y)` 三分量 lerp）并丢弃下方段。寄存器别名打散，
  精确流 U，以语义折线裁剪（y>=max_y 保留 + 段跨越处插截断点）承载。
- **双 Curve 构建（O）**：主体 `0..2` for_each（0x1407eb274/0x1407eb282 两次
  closure_0 调用，index 0/1）→ 各 `Curve::try_new_from_points<Vec3,3D>` unwrap →
  压入 out Vec（stride 0x38）。`calc_gothic_half_dormer_profile_ws(out:&mut Vec<Curve3>,
  rank, max_y, is_circle, shape_kind)`。

### 未实现（登记）

- **wall-space 逐点变换**：call_once closure_env_0（0x1412ac9b0）的
  `t=inverse_lerp(0,last,u)`（0x140c904a0 未钳制）+ 墙点 A(+0x28)/B(+0x30 直墙
  /+0x48 圆墙) lerp + C(+0x38)/D(+0x40 直墙/+0x50 圆墙) 插值 normalize 法线 +
  `x0y` 升维 → `Vec3(fVar9*nx+lerp_x, h+lerp_y, fVar9*ny+lerp_z)`。这些 raw 偏移属
  B PublicWallState 内部，country_core 语义模型未暴露 → 变换以 RE-ANALYSIS 简式
  `Vec3(u, h*max_y, 0)` 承载，精确面 U。
- **0..2 索引语义**：两 closure 调用 env 相同，仅 index 0/1 区分；两曲线内容
  （镜像/两段候选）U，当前以同 profile 构建两 Curve 承载。

## 2. `chimney_smoke_anim_thickness`（0x140818df0）— chimney_smoke_anim_thickness.rs

### 公式还原（ReVa 全量 31KB `reva_chimney_smoke_140818df0.c` 核验）

先前实现更新公式打散（FIX-4 登记），本轮按 ReVa 全量还原：

- **目标公式按 spot+0xd 分派**（0x14081956f-0x140819585）：
  - `+0xd==0`：`target = r*3.0 + (1.0-r)`（0x1429cd078=3.0）；分支**先丢弃一次抽签**
    （`fastrand::f32()` 后再次抽签）。
  - `+0xd!=0`：`target = r*4.0 + (1.0-r) + (1.0-r)`（0x142934694=4.0）。
- **符号**：`+0xd==0` → 恒 `-1.0`（fVar21=0xbf800000）；`+0xd!=0` → `fVar58`，
  窗口内（t<0.72 || t>=0.9）`+1.0` / 窗口外 `-1.0`（0x14081947f-0x140819492）。
- **更新**：`new = clamp01(old + fVar59 * dt / CONCAT13(divisor))`，divisor 即刚算
  目标值的字节重拼 → **`new = clamp01(old + sign*dt/target)`**（0x140819646-
  0x140819654）。dt 源 = `*(f32*)(GameTime+0x28)`（country_core `GameClock.delta`
  @+0x28 已对齐）。
- **初始厚度（新条目）**：窗口内且 +0xd==1 → 1.0，否则 0.0（0x140819390-
  0x140819393）。

### 落点与分歧（U 登记）

- `ChimneySmokeThickness` 值型改 `ChimneySmokeEntry { thickness f32, flag u8 }`——
  +0xd 烟型标记按 spot 记录持有，当前无 spot 收集主循环，以条目字段持久化承载
  （二进制表值仍只存 f32 —— **布局分歧 U**，RESTORE §2 登记）。
- rng 种子来源（init_rng 实参寄存器未取到）以 `delta.to_bits()` 承载（U）。
- spot 收集主循环（墙 0xa0 段桶 / 屋顶 0x48 段桶）、retain 谓词、初始插入点
  **未实现**（收集 stub 沿用既有表）。

## 3. `manage_tree_prefabs`（0x141bf16c0）— forest/manage_tree_prefabs.rs

### has_construct_shape 门控补证落点（RE-ANALYSIS §6.1）

- **阶段 2 门控位 = `placements 资源 +0x18` 处理状态字节**（`*(char*)(lVar24+0x18)`
  ==1 → 后续帧随机 scale 分支；帧末置 1；非"事件存在性"）。country_core
  `InteractableForestPlacements` 无该字段 → 以 `MemCache.finished`（Local）承载
  语义等价簿记，模块 doc 已记（分歧 U）。
- **阶段 1 形状门控修正**：fresh 核对 0x141bf1d20-0x141bf1d44：
  `bVar32 = 0.0<fVar50|AL`（密度/岩石）后 `bVar33 = bVar32^1`；形状邻近判定仅在
  `((A&&B)|bVar32)!=1`（有构造形状 && density==0 && rock==0）时运行，随后
  `if (bVar33 != 0)` **恒运行**墙圆查询 `Circle2d::new(x,z,7.0)` +
  `query_with_circle_callback`。原实现 `!has_construct_shape → true`（跳过墙圆
  查询）与二进制不符 → **已修**：墙圆查询改为 density==0&&rock==0 时恒运行，
  形状邻近判定以 `has_construct_shape` 门控。
- **高度采样门控**：改 `(-65.0 <= x && x < 65.0) && (-65.0 <= z && z < 65.0)`
  （原 `|x|<65 && |z|<65` 在 ±65 边界分歧）。

### 未实现（登记）

- 构造形状源（ShapeParameters 0x28B 墙形状，来自 ConstructTimberframeCmd 事件缓冲
  末条）当前 country_core 为 `Vec<u8>` 占位 → 点 `dist<radius+7.0` / 矩形
  `dist(closest_perimeter)<7.0` 分派**未实现**（core-type-gap）。

## 交付物

- 代码：`crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs`、
  `crates/systems/decorator/src/chimney_smoke_anim_thickness.rs`、
  `crates/systems/forest/src/manage_tree_prefabs.rs`。
- 证据：`reports/wave2/decorator-forest/jobs/*.json`（fresh 反编译/反汇编/常量）。
- 验收：`cargo check -p system-decorator -p forest` ✅（0 error）。

## 遗留

- `calc_gothic_half_dormer_profile_ws`：wall-space 逐点变换、0..2 索引语义待
  core PublicWallState 暴露 raw 面后重建。
- `chimney_smoke_anim_thickness`：spot 收集主循环、rng 种子、retain 谓词。
- `manage_tree_prefabs`：构造形状源 ShapeParameters（core-type-gap）。
- RE-ANALYSIS §4.4 半径选择方向纠正已在本报告 §1 登记（供集成 owner 裁决）。
