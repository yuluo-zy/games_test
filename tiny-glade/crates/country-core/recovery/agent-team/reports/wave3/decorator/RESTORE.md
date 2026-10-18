# RESTORE — wave3 decorator visual/GPU（restorer-wave3-decorator-visual，2026-10-17）

> 角色：restorer-wave3-decorator-visual（wave3）。硬标准（真实逻辑非形状）：每函数
> fresh 反编译 → 重建真实控制流 → Rust body 逐语句判等；还原不了标 `未实现`
> （列分歧点），不以近似通过。
>
> 本轮目标：把 `crates/systems/decorator` 中依赖 core 新类型（SignifierFactory /
> SignifierEntityBuilder / PlayUiSound#4 / SparseSsbo / SsboType / ReplayBucket）的
> **视觉族与 GPU 未实现函数** fresh 复核落库。
>
> 工具：ghidra `--worker shared`（只读）fresh 反编译。本轮新取证：
> - `evidence/decomp_0x14080c8d0.c`：replay_decorator（997B，O 全文）
> - `evidence/decomp_0x1412ab270.c`：`WindowVisualEntityBuilder::build<
>   DecoratorTransientVisualState>`（3281B，O 全文）
> - `evidence/decomp_0x1412a9b30.c`：`WindowVisualEntityBuilder::build<
>   DecoratorPersistentVisualState>`（3281B，O 全文；尾部 = impl$4）
> - `evidence/ghidra-deco-key.result.json` / `ghidra-build-persist.result.json`
>   （原始结果）
> 沿用既有 fresh：`reports/state/decorator/review-evidence/fresh-decomp/
> 0x1407ce920_maintain_dormer_tiles.c`、`0x1407b5980_window_lights.c`、
> `0x140818df0_chimney_smoke_anim_thickness.c`、`0x140811c30_add_decorator_
> visual_entity.c`（reviewer 独立 diff 15/15 一致）+ `reports/re-analysis/
> hardpoints-ecstate/evidence/decomp/reva_chimney_smoke_140818df0.c`（31KB）。
>
> 验收：`cargo check -p system-decorator` **FIX-E 前 ✅**（本会话 10:08-10:13 多次
> Finished，0 error，6 警告全为既有 LOG_TARGET dead_code / replay `mut storage`）。
> **FIX-E 后重跑被并发 rhapsody 编辑阻断**（crates/rhapsody/src/rg/{vulkan.rs,
> sync_plan/bind.rs, vulkan_queue.rs} 11:22-11:32 被其他 agent 在途修改，10 个
> E0308/E0599 全部位于 rhapsody，非 decorator）；decorator 5 文件 rustfmt --check
> 解析通过（仅行宽重排建议），FIX-E 为纯语句搬移（预写 + 循环内 +1，类型不变）。
> rhapsody 恢复后重跑即可（与 wave2 同模式）。

## 判定总表

| 函数（VA） | 判定 | 说明 |
|---|---|---|
| `maintain_dormer_tiles`（0x1407ce920）| **结构判等；GPU 退役未实现；游标写序 FIX-E 已修** | 游标推进/ids.push/双列表遍历判等；**游标写序修正**（FIX-E reviewer wave3：二进制循环前写 `total-lenB'-lenA'` + 每瓦 +1 → 末值=total，原 Rust 循环后写不符）；每瓦 hash_one+remove_entry+条件 drop 缺 rhapsody 回调注册表 + RoofTileSsbo SparseSsbo（core-type-gap）；**已删早期虚构"瓦变换写 FUN_1405b0d70"注释** |
| `window_lights`（0x1407b5980）| **常量/灯态/帧末判等；收集主循环未实现** | 开灯区间 0.51/0.44/0.72（read_memory 核验）、逐条目随机重计时、帧末计时/开关判等；**retain 次序修正**（fresh：retain 在帧末前，原 Rust 帧末后）；墙/屋顶收集主循环缺 PublicWalls 密集表（core-type-gap）；rng 种子/retain 谓词/lerp 实参 U |
| `chimney_smoke_anim_thickness`（0x140818df0）| **窗口/目标/更新公式判等；收集主循环未实现** | t 窗口 `t<0.72||t>=0.9`、目标 `r*3.0+(1-r)`/`r*4.0+2*(1-r)`、符号、更新 `clamp01(old+sign*dt/target)`（ReVa 全量，fVar59 改写为符号 + divisor=目标字节重拼）、[0,1] 钳判等；spot 收集主循环缺 PublicWalls 密集表（同 window_lights 源）；rng 种子/retain 谓词/初始插入点 U |
| `replay_decorator`（0x14080c8d0）| **未实现（core-type-gap）** | **本轮 fresh 反编译**：front_mut_bucket → 桶 `+0x18/+0x20` `Arc<[DecoratorId]>` cull-allow 表 → 扫描 tag-7 0x30B 命令（flags bit0 立即完成 / `max(dur/speed,0.05)`<elapsed 完成 / elapsed+=delta）→ 反转索引 Vec::remove + tag 跳表释放；core ReplayCommand 无 Decorator 变体 + core ReplayBucket 无 Arc<[DecoratorId]> 字段（precondition"若 core 已补"未满足）；**DURATION_FLOOR 0.1→0.05 修正**（fresh 证据） |
| `WindowVisualEntityBuilder::build<Transient/Persistent>`（0x1412ab270 / 0x1412a9b30 等 6 实例）| **未实现-需重做（近似体）** | **本轮 fresh 反编译**（两 build 同体 3281B）：构建器 0x150+B 记录；mesh 名分派（len switch 9..0x24）→ `Commands::spawn<NaniBundle<InstanceData>>`（实例数据 = Ssbo 类型，core `render::nani::instance_data` 已落，字段 U）或默认 `spawn<DrawableMeshBundle>`；条件组件入队链（ColorId/GothicWindowBrickDeformation/BrickColor/ClothlineCloth/SemiTransparent/WindowLight/BalconyDoor/Smoke/AlbedoMap/NormalMap/RoofFlagCloth/WallFlagCloth/LanternLightParams/SittableForBirds）；末尾 build<Persistent>→impl$4 / build<Transient>→impl$5（**autoclutter 表**，D1 build_transient 目标不符，登记）。**NaniBundle/DrawableMeshBundle 结构体未落库** → mesh 分派无法接线；构建器字段语义 U → 当前 D1 Transform+本地标记组件为近似占位，已标 |
| `impl$5::add_decorator_visual_entity`（0x140811c30）| **未实现（core-type-gap）** | autoclutter 子项表 `HashMap<u32, Vec<DisplayChildAutoClutterInfo>>`；`DisplayChildAutoClutterInfo`（core `resources::clutter_meshes`）未落库（FIX-9 登记延续） |
| ui_systems WidgetParams 变体裁决 | **已用 core WidgetParams（判等面）** | `country_core::systems::ui::resources::widget_params::WidgetParams`（含 UiSounds/UiVolumeShape/UiFocus）已由 D2 接入（ui_decorator_affordance_dispatch/ui_focus_stairs 等）；无 PlayUiSound 消费（该变体#4 属 eco/glade-border feedback 域） |

**净判等（部分）3 / 未实现-需重做 4 / 未实现（core-type-gap）2 / 近似体降级 1。**

---

## 1. `maintain_dormer_tiles`（0x1407ce920）— decorator_visual/cottage_dormers/maintain_dormer_tiles_draw.rs

### 判等（结构面，fresh 逐语句）

- `prev = *cursor`；`startA = prev - countA`、`startB = prev - countB`（各钳 0）；
  `lenA = (lenA >= startA) ? lenA - startA : 0`（空哨兵 8）；
- **游标循环前预写** `*cursor = total - lenB' - lenA'`（fresh line 47）；
- 遍历 `A[startA..]` 链 `B[startB..]` 0x18B 瓦元素，**每瓦**：
  1. `*cursor += 1`；
  2. 实例 id 入 Vec（`*[ids槽]=+0x1c 值`）；
  3. `hash_one<RandomState, &GlobalId>` + `RawTable::remove_entry<u64,
     Box<dyn FnOnce<(&mut *mut c_void, bool)>>>`；
  4. 返回值非空 → `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`；
- **末值 = total**（预写 total-lenA'-lenB' + 每瓦 +1 × (lenA'+lenB')）。

Rust 对照：游标/ids/遍历逐语句一致；**写序 FIX-E**（reviewer
wave3-toolbar-decorator）：二进制循环前写 end（末值=total），原 Rust 循环后写
end（末值=total-(lenA'+lenB')，不符）——**已修**（maintain_dormer_tiles_draw.rs
现循环前预写）。每瓦主体 `let _ = tile;` 占位。

### 未实现（FIX-2 延续，core-type-gap）

③ 的 `RawTable` 回调注册表 = **rhapsody 内部稀疏分配回调表**（`RawTable<u64,
Box<dyn FnOnce>>`，键 = GlobalId 哈希），core 未暴露；④ 的 `RoofTileSsbo`
（`country_core::resources::roof_tile_ssbo::RoofTileSsbo`）在 core
`compute_passes/roof_tiles_gravity.rs` 为 `Gap` 承载，**未实现 `SparseSsbo`**——
`SparseInstanceBufferAllocation<RoofTileSsbo>` 的 drop 无法接线。上述闭合后
重建为系统并注册（LEDGER §1/§6）。**不伪造表模型**。

### 分歧点修正（wave3）

1. **游标写序（FIX-E）**：见上，循环前预写 + 每瓦 +1，末值=total（已修）。
2. 早期 D1 注释所称"瓦变换写 FUN_1405b0d70(state.base+0x20, elem+8) +
   FUN_140053320 四参写入"在 fresh 0x1407ce920 **不存在**（每瓦体即上述四步）——
   **已删虚构注释**（maintain_dormer_tiles_draw.rs 头 + 函数体）。

---

## 2. `window_lights`（0x1407b5980）— window_lights.rs

### 判等面（fresh 逐语句）

- 参数面（O）：in_RCX=ResMut<Control>（tick 回写 +0x1c→+0x10、+0x30 写
  lights_on）、in_RDX=ResMut<PublicWalls 复合>、in_R8=Res<昼夜循环>（t `*+8`）、
  in_R9=Res<GameTime>（delta `*+0x28`）。
- 开灯：`deny_load()(=is_olden 同址) ? 0.44 : 0.51` 下界、0.72 上界
  （read_memory 核验 0x3f028f5c/0x3ee147ae/0x3f3851ec）；`lights_on =
  lower<=t && t<0.72`。
- 灯态变化：`target != lights_on → timer = 2*rng.f32()` + `target=lights_on`。
- 帧末：`timer>0 → timer-=delta（钳 0），==0 且非 NaN → actual=target`；
  `brightness = time_continuous_lerp`。

### 未实现（core-type-gap + U）

- **收集主循环**（墙 +0x108==0 走 +0xf8 id 数组 + 0xa0 段桶 / 屋顶 0x48 段桶，
  `+0x110` 索引反查实体记录）——遍历的是 **PublicWalls/InnerWalls 密集窗桶表**
  （stride 0xa0/0x48），core `PublicWalls` 为 HashMap 抽象，密集表未落库（walls
  组在途域，LEDGER §1）→ 缺失插 `{brightness=0, actual=target=lights_on}`、
  形状变化 `brightness=1.0/target=1` 无法实现。
- rng 种子（fastrand::Rng::f32，种子 U）、retain 谓词（closure_env_1 未解）、
  time_continuous_lerp 实参（(0,1,bits,0.05) 猜测）。

### 分歧点修正（wave3）

- **retain 次序**：fresh 显示 `HashMap::retain<closure_env_1>`（LAB_1407b62ed）在
  **帧末推进循环之前**（顺序 = 灯态变化 → 收集 → retain → 帧末）；原 Rust 把
  retain 放帧末后。**已调整**（window_lights.rs 现 retain → 帧末），与二进制次序
  对齐（谓词 U 仍登记）。

---

## 3. `chimney_smoke_anim_thickness`（0x140818df0）— chimney_smoke_anim_thickness.rs

### 判等面（ReVa 31KB 全量 + wave3 复核）

- 参数面（O）：in_RCX=ResMut<ChimneySmokeThickness>、in_RDX=ResMut<PublicWalls
  复合>、in_R8=Res<昼夜循环>、in_R9=Res<GameTime>（delta `*+0x28` =
  GameClock.delta@+0x28 对齐）。
- t 窗口 `t < 0.72 || t >= 0.9`（0x142a80d44/0x142a7c88c 核验）。
- 每 spot `init_rng` 后：`+0xd==0 →` 丢弃一次抽签后 `target=r*3.0+(1-r)`、
  符号恒 -1.0；`+0xd!=0 → target=r*4.0+2*(1-r)`、符号按窗口 ±1.0。
- **更新公式（wave2 已还原，wave3 复核）**：`fVar59` 算完目标被**改写为符号**
  （`fVar59 = fVar21/fVar58`），`CONCAT13(divisor)` = 目标字节重拼 → 实为
  `new = clamp01(old + sign*dt/target)`；`clamp [0,1]`。

Rust 对照：窗口/目标/符号/更新/钳制判等（`entry.flag` 承载 +0xd，布局分歧 U）。

### 未实现（core-type-gap + U）

- **spot 收集主循环**（墙/屋顶双模式桶，同 window_lights 源，PublicWalls 密集
  表 core 未落库）→ 初始厚度插入点（窗口内且 +0xd==1 → 1.0，否则 0.0）、
  逐 spot 独立 init_rng、retain 谓词无法实现；当前以条目全量维护承载。
- rng 种子来源（init_rng 实参寄存器未取到）以 `delta.to_bits()` 承载（U）。

---

## 4. `replay_decorator`（0x14080c8d0）— replay_decorator.rs

### 本轮 fresh 反编译（997B，evidence/decomp_0x14080c8d0.c）

- 参数（O）：in_RCX=ResMut<GameTime>（delta `*+0x28`）、in_RDX=ResMut<ReplayBuffer>
  （speed `*+0x28`、tick 回写 +0x10/+0x1c）、in_R8=ResMut<DecoratorArchivist>
  （cull-allow 集合 + tick 回写 +0x10/+0x1c）。
- 行为（O）：front_mut_bucket → 取桶 `+0x18/+0x20` 的 **`Arc<[DecoratorId]>`
  cull-allow id 表**（指针置 0 = 取出）逐 id 入 `HashMap<DecoratorId,()>` → Arc
  drop → 扫描命令（桶 `+0x8` 指针 / `+0x10` 长度，每条 0x30B，布局
  `{tag@0, flags@+8, duration f32@+0xc, payload@+0x10, elapsed f32@+0x18}`；
  **跳过非 tag-7**，只处理 tag==7 装饰命令）：`flags bit0==0 → 立即完成`；
  否则 `required = max(duration/speed, **0.05**)`，`required < elapsed → 完成`；
  未完成 `elapsed += delta` → 完成索引**反转**后逐条 `Vec::remove`（0x30B memmove）
  + tag 跳表释放（0/1/2=RasterEdit Arc、3=TerrainEdit Arc、4=drop_in_place
  <WallReplayCmd>、5=RoofEdit Arc、6/7=DecoratorHistoryEdit Arc、8=TreeEdit Arc、
  9=CrashMiscInfo Mutex、default=winit PhysicalSize Mutex）。

### 判定：未实现（core-type-gap）

precondition"ReplayBuffer 变体若 core 已补"**未满足**：
- core `ReplayCommand` 枚举无 **Decorator 变体**（bucket 展开 DecoratorV5 →
  `UnrestoredReplayOwner`）；
- core `ReplayBucket`（{entry_index, undo, commands, timing}）**无
  `Arc<[DecoratorId]>` cull-allow 表字段**（二进制桶 +0x18/+0x20）；
- 二进制命令时序内嵌于 0x30B 记录（+8/+0xc/+0x18），非 core `timing:
  Vec<CommandTiming>` 分离模型。

### 修正（wave3）

- `DecoratorReplayCmd::DURATION_FLOOR` **0.1 → 0.05**（fresh `if (fVar22 <=
  0.05) fVar22 = 0.05`；旧 0.1 与 stairs 族混淆不符）。该模型用于 tag-7 完成判定
  与推进，core 接线后照用。

---

## 5. `WindowVisualEntityBuilder::build<Transient/Persistent>`（0x1412ab270 等）

### 本轮 fresh 反编译（3281B，evidence/decomp_0x1412ab270.c）

真实结构（O，逐语句）：
1. 构建器 **0x150+B 记录**（字段语义多 U）；读 pos/rot（+0xc0/+0xc4/+0xc8/+0xcc）、
   旋转分量（+0xd0/+0xd8）→ 四元数旋转向量（标准 q 旋转公式）。
2. **mesh 名分派**（`String{len@+0xb0, ptr@+0xb8}`，len switch
   9/10/0x11/0x15/0x17/0x19/0x1b/0x1e/0x23/0x24）→ `Commands::spawn<NaniBundle<T>>`：
   UberInstanceData / ClothInstanceData / SmokeInstanceData / BalconyDoorInstanceData /
   LanternEmissiveInstanceData / SolidVertexColorInstanceData / GothicWindowBricksInstanceData /
   (SolidAlbedoMapInstanceData, NaniSolidTexture) / SolidVertexColorWindowInstanceData /
   SolidVertexColorBricksInstanceData / (SolidAlbedoMapEmissiveInstanceData, NaniSolidTexture) /
   SolidVertexColorWindowGlassInstanceData；默认分支 `AssetMeshLibrary::
   get_handle_by_name` + `AssetShaderLibrary::get_raster_handle_by_name`（expect
   panic）→ `spawn<DrawableMeshBundle>`。**实例数据 = Ssbo 类型**（core
   `render::nani::instance_data` 已落：0x40/0x30/0x70 元素，字段 U）。
3. **条件组件入队链**（O 序）：ColorId（+0xf0 bit0）、GothicWindowBrickDeformation
   （+0x40==1）、BrickColor（+0x114==2）、ClothlineCloth（+0x80==1）、
   SemiTransparent（+0x14e!=0）、WindowLight（+0x10 bit0）、BalconyDoor
   （+0x14f!=0）、Smoke（+0x14c!=2）、AlbedoMap（+0x20==1）、NormalMap（*+0 bit0）、
   RoofFlagCloth（+0xf8==1）、WallFlagCloth（+0x13c!=2）、LanternLightParams
   （+0x118==1）、SittableForBirds（+0x90==1），各含具体载荷。
4. **末尾状态表 push（本轮新取证，重要）**：
   - `build<DecoratorPersistentVisualState>`（0x1412a9b30，size 3281B，与本 build
     **同体**）末尾 = `impl$4::add_decorator_visual_entity`（0x1408119e0，持久表
     `HashMap<u32, PersistentVisualEntities>` push）✓ 与 D1 build_persistent 对齐；
   - `build<DecoratorTransientVisualState>`（0x1412ab270）末尾 =
     `impl$5::add_decorator_visual_entity`（0x140811c30，**autoclutter 子项表**
     `HashMap<u32, Vec<DisplayChildAutoClutterInfo>>` push）。
   → **分歧登记**：D1 `build_transient` 当前 push 到瞬态**实体表**
   `DecoratorTransientVisualState::entities`（clear_out_visual_entities 的 drain 源），
   与二进制 build<Transient> 的 impl$5（autoclutter 表）**目标不符**——二进制瞬态
   实体表的填充源不在本 build 函数（另查），autoclutter 表消费端 =
   add_door_autoclutter（reviewer §二-6 登记）。此分歧待集成 owner 裁决（改
   build_transient 目标或确认瞬态表另源）。

### 判定：未实现-需重做（近似体降级）

- `NaniBundle`/`DrawableMeshBundle` 结构体未在 country-core 落库（`components/
  drawable.rs` 只有 `NaniInstanceMeshDesc`/`DrawableMeshBundle::build` 胶水，Bundle
  结构体缺）→ mesh 分派 spawn 无法接线（render 边界）。
- 构建器字段语义 U（+0xb0 起 mesh 名、+0x10..+0x150 条件位/载荷）→ 逐组件载荷
  无法判等。
- 当前 D1 `WindowVisualEntityBuilder` 以小结构 + Transform/本地标记组件承载，
  **属近似占位**（此前未 fresh 复核，wave2 标"未实现（未 fresh）"成立）——保留
  以维持 decorator_visual 族编译，头注释已标近似与真实结构。
- 相关：`impl$5`（0x140811c30）autoclutter 表 push 仍缺 `DisplayChildAutoClutterInfo`
  （core `resources::clutter_meshes` 未落库，FIX-9 延续）。

---

## 6. ui_systems WidgetParams 变体裁决

`country_core::systems::ui::resources::widget_params` 的 `WidgetParams`（含
`UiSounds`/`WidgetVolumeShape`/`UiFocus`/`UiNavState`）已落库，D2 的
ui_decorator_affordance_dispatch/ui_focus_stairs 等**已接入**（`WidgetParams::new()
.sounds(UiSounds::empty())` 等，O 面）。装饰域无 `PlayUiSound` 消费（变体 #4 属
eco/glade-border feedback_input_out_of_border 域，coregap-1 已判等）——本模块
裁决为"用 core 类型"，无需本地变体。

---

## 证据索引

- 本轮 fresh（`reports/wave3/decorator/evidence/`）：`decomp_0x14080c8d0.c`、
  `decomp_0x1412ab270.c`（build<Transient>）、`decomp_0x1412a9b30.c`
  （build<Persistent>）、`ghidra-deco-key.result.json` / `ghidra-build-persist.result.json`
  （原始结果）。
- 沿用 fresh（reviewer 独立 diff 一致）：`reports/state/decorator/review-evidence/
  fresh-decomp/0x1407ce920_maintain_dormer_tiles.c`、
  `0x1407b5980_window_lights.c`、`0x140818df0_chimney_smoke_anim_thickness.c`、
  `0x140811c30_add_decorator_visual_entity.c`；`reports/re-analysis/hardpoints-ecstate/
  evidence/decomp/reva_chimney_smoke_140818df0.c`。

## 遗留

- maintain_dormer_tiles：rhapsody 回调注册表 + RoofTileSsbo SparseSsbo（core-task）。
- window_lights/chimney_smoke：PublicWalls 密集窗桶表（walls 组在途域，core-task）+
  rng 种子/retain 谓词/lerp 实参（re-analysis）。
- replay_decorator：core ReplayCommand Decorator 变体 + ReplayBucket Arc<[DecoratorId]>
  cull 表（core-task）。
- visual_entity_builder build：NaniBundle/DrawableMeshBundle 落库（core-task）+ 构建器
  字段语义（re-analysis）。
