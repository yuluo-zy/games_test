# RESTORE — systems/decorator（restorer-state-decorator，2026-10-17）

> 角色：restorer-state-decorator（state 组）。硬标准（真实逻辑非形状）：每业务函数
> fresh 反编译 → 重建完整控制流 → Rust body 逐语句判等；重建不了标 `未实现`
> （列出分歧点），不以近似通过。
>
> reviewer-state-decorator 复核（REVIEW.md，独立 fresh diff 15/15 一致）后，
> 本版按 FIX-1..10 修订判定与代码。

## 判定总表（按 reviewer 复核修正）

| 函数（VA） | 判定 | 说明 |
|---|---|---|
| `DecoratorBackup::clear`（0x140823660） | **判等** | reviewer §一-1 |
| `clear_out_wall_holes`（0x1407f98c0） | **判等** | reviewer §一-2 |
| `ui_decorator_inactive`（0x140791400） | **判等** | reviewer §一-3 |
| `stairs_reset_for_new_frame`（0x14079c350） | **判等（门控）** | 门控 {4, 11+'}''} 判等；unset 落点 I 级（FIX-8 登记） |
| `seal_persistent_entities`（0x140811e80） | **判等** | panic 串独立核验 |
| `add_decorator_visual_entity` impl$4（0x1408119e0） | **判等** | cap=8 预分配差异 minor |
| `entities` ×2（0x1408119d0/0x14067dd80） | **判等**（退化访问器/胶水级） | — |
| `clear_out_visual_entities`（0x1407f9260） | **判等（FIX-1 后）** | 删两处额外副作用（persistent 清表 + wall_holes 清帧），仅留 drain+despawn |
| `maintain_dormer_tiles`（0x1407ce920） | **未实现-需重做** | FIX-2：每瓦 `RawTable::remove_entry<u64,Box<dyn FnOnce>>` + 条件 drop `SparseInstanceBufferAllocation<RoofTileSsbo>`（rhapsody 注册表类型缺口）；游标/ids 判等 |
| `add_decorator_visual_entity` impl$5（0x140811c30） | **未实现** | FIX-9：autoclutter 表 `Vec<DisplayChildAutoClutterInfo>` 无实现（core 类型缺口，显式登记） |
| `apply_decorator_op`（0x141276e30） | **未实现-需重做**（控制流判等；叶近似） | FIX-7 已补：archivist None → unwrap（非静默跳过）；归档叶 `archive_decorator_state`/`build_op_event`/locator 实参仍近似 |
| `create_or_restore_decorator_backup`（0x1408238a0） | **未实现-需重做**（状态机判等；恢复叶近似） | FIX-6 已补：+0x48 pending 门控清 modified state；辅助 0x30B 三 map 反向拷贝未接线 |
| `decorator_record_history`（0x140817850） | **未实现-需重做**（循环/清空判等；载荷映射错） | FIX-5 已修记录布局 `{kind,f1,f2,f3}`；48B `{1,1,kind,f1,f2,f3}` 载荷 kind/f2/f3 因 core 存档模型缺口丢弃 |
| `window_lights`（0x1407b5980） | **未实现-需重做**（常量/灯态/帧末判等；收集主循环缺失） | 常量 0.51/0.44/0.72（read_memory 核验）、逐条目随机重计时、帧末计时/开关判等；墙/屋顶收集主循环、形状变化 brightness=1.0/target=1、rng 种子、retain 谓词 U |
| `chimney_smoke_anim_thickness`（0x140818df0） | **未实现-需重做**（窗口/目标判等；更新公式 U） | 窗口 `t<0.72||t>=0.9`、目标 `r*3.0+(1-r)`/`r*4.0+2*(1-r)`、初始 1.0、[0,1] 钳判等；更新累加公式打散（reva High P-code）、收集主循环、retain 谓词 U |
| `decorator_visual_state` 结构 | **结构修正成立** | `PersistentVisualEntities={seal u8, Vec<Entity>}`；impl$4/seal/entities 判等，impl$5 未实现 |
| `DecoratorBackup` / visual 壳（chimney/door_stairs_assemble_bricks） | **未实现**（签名面错） | 7×u64/4×u64 参 vs ECS 系统参；with_diffs 差分主体未核 |
| lib.rs `Plugin::build` / `snapping_direction` / `try_merge_decorators` | **glue-legitimate** | 注册/内联胶水（reviewer 接受） |
| 其余 ~146 业务函数（blueprint/load/subtype/cull_oob/merge/move/on_*/visual 族/stairs/ui_systems/utils） | **未实现（本轮未 fresh 复核）** | ANALYSIS-A/B/C 为指引非凭据，待逐函数 fresh |

## FIX 落实（reviewer-state FIX-1..10，2026-10-17）

1. **clear_out_visual_entities**：删除 `persistent.entries.clear()` 与
   `wall_holes.clear_for_next_frame`（二进制本函数无）——已修，仅留 drain+despawn。
2. **maintain_dormer_tiles**：每瓦 GPU 退役（remove_entry + 条件 drop）——
   rhapsody 回调注册表类型未在 core 暴露，**显式标未实现**（代码注释登记）。
3. **window_lights**：收集主循环缺失——**标未实现**（部分判等见总表）。
4. **chimney_smoke**：更新公式打散——**标未实现**（部分判等见总表）。
5. **decorator_record_history**：`DeferredDecoratorRecord` 布局改
   `{kind u64, f1 u64, f2 u64, f3 u64}`（32B，与二进制 `plVar10[0..3]` 读面对齐）
   ——已修；48B `{1,1,kind,f1,f2,f3}` 载荷 kind/f2/f3 因 core
   `DecoratorHistoryEdit` 存档模型缺口丢弃——未实现登记。
6. **create_or_restore_decorator_backup**：+0x48 pending 门控清 modified state
   ——已修（`clear_modified = deferred.pending`）；辅助 0x30B 三 map 反向拷贝
   ——live 侧资源身份 U，未接线登记。
7. **apply_decorator_op**：archivist None 时镜像 unwrap panic（`Option::unwrap`）
   ——已修（不再静默跳过）；`archive_decorator_state`/`build_op_event`/locator
   实参叶近似——未实现登记。
8. **stairs_reset_for_new_frame**：unset 落点 `[内层+0x10]=3` 以 `mark_invalid`
   承载（MoveDecorationInteractionState 为 I 级布局）——登记转交互布局确认。
9. **add_decorator_visual_entity impl$5**：autoclutter 表无实现——显式标未实现。
10. **集成缺口**：clear_out_visual_entities / clear_out_wall_holes /
    maintain_dormer_tiles / decorator_record_history /
    create_or_restore_decorator_backup 在 workspace 无调度注册点——转接线登记
    （lib.rs add_decorator_systems 由 D4 维护，待 App 组装时逐批接入）。

## fresh 反编译证据索引（本会话，`evidence/restorer-state-decorator/`）

- `result-decomp-1.json` → `decomp/0x1408119d0|0x1408119e0|0x140811c30|0x140811e80|0x140823660|0x1408238a0|0x140817a40|0x1408153d0_*.c`
- `result-decomp-2.json` → `decomp/0x1407b5980|0x140818df0|0x1407b1d40|0x140817850|0x14080c8d0|0x141276e30|0x14081a380_*.c`
- `result-decomp-3.json` → `decomp/0x1407f8db0|0x1407ff130|0x1407f9260|0x1407f98c0|0x1407ce920|0x1407f3340|0x140791400|0x14079c350|0x14079c3a0_*.c`
- 常量/panic 串核验（read_memory）：`result-consts.json`、`result-consts2.json`
  （0x142c23510 assert 串 + 行 50）。reviewer 独立 diff 15/15 IDENTICAL。

## 详细重建/判等说明

### §1 apply_decorator_op（0x141276e30）—— 控制流判等；叶未实现

fresh 控制流（逐分支）：guard `archivist.is_none() && kind!=0 → panic`（串核验
行 50）；`(kind-3)<2 → place/remove`，其余 change；change 子判别 op[+0x10]
`!=3 → 二次 archive`；**archive 均在历史之前**；历史 push kind 3/4/5 +
`DecoratorLocator::new` + `HashMap<DecoratorLocator,()>::insert`；末尾
`DeferredDecoratorOpEvents::send`。已修（FIX-7）：archivist None 分支
`unwrap_failed` 镜像（`Option::unwrap`）。叶近似：`archive_decorator_state`
（core `archive_current_decorators_in_storage` 缺口等价）、`DecoratorLocator::new`
实参、`build_op_event` 0x88B 字段放置。

### §2 DecoratorBackup::clear（0x140823660）—— 判等

default 模板 + clone_from + 三标志清零；`*self = default()` 等义（内存复用差异
不可观测）。

### §3 create_or_restore_decorator_backup（0x1408238a0）—— 状态机判等；恢复叶未实现

三态机（+0x282 读即清零）；CREATE = all_decorators 逐桶
get_or_insert_default+get+tracked_clone_from + 三 map + 历史 Vec 拷贝 +
置 +0x281；RESTORE = all_storages 清 + all_decorators
get_mut+get+DecoMoveToStorage+tracked_clone_from（+0x48 门控清 state@+0x47）+ 反向
三 map + Vec + `StairsState::restore_backup`；+0x280 → clear。已修（FIX-6）：
+0x48 pending 门控。未接线：辅助 0x30B 三 map 反向拷贝（live 资源身份 U）；
archivist all_storages/all_decorators 以 storage.iter_walls() 近似（桶模型 core
适配）。

### §4 decorator_visual_state 结构（0x1408119d0/9e0/c30/e80）

`PersistentVisualEntities = {seal u8, Vec<Entity>}`（PDB enum2$ + 消费方展平
&[Entity] 证明）；`entries: HashMap<u32, PersistentVisualEntities>`（桶 0x28B）；
impl$4 entry().or_default().push + seal panic；seal 置 1/重复 panic。impl$5
autoclutter 表未实现（FIX-9）。

### §5 decorator_record_history（0x140817850）—— 循环/清空判等；载荷映射未实现

pending 读即清（无条件）；非零 → 逐 0x20B 记录（kind==6 break）分配 48B
`{1,1,kind,f1,f2,f3}` 调 History::add（kind 10）；无条件清 records Vec + 清
HashMap<DecoratorLocator,()>。记录布局已改 `{kind,f1,f2,f3}`（FIX-5）；48B
kind/f2/f3 因 core 存档模型缺口丢弃（未实现登记）。

### §6 window_lights（0x1407b5980）—— 常量/灯态/帧末判等；收集主循环未实现

下界 deny_load(=is_olden 同址) ? 0.44 : **0.51f**（0x3f028f5c）、上界
**0.72f**（0x3f3851ec，read_memory 核验；旧 0.5097/0.7199 错 lane 勘误成立）；
条目 0x18B {key, brightness f32@8, timer f32@0xc, actual u8@0x10, target
u8@0x11}；灯态变化 `target!=lights_on → timer=2*f32 + target=lights_on`；帧末
timer-=delta 归零 actual=target + time_continuous_lerp。未实现：墙/屋顶收集主
循环（缺失插 `{brightness=0,actual=target=lights_on}`、形状变化
brightness=1.0/target=1）、rng 种子、retain 谓词、time_continuous_lerp 实参。

### §7 chimney_smoke_anim_thickness（0x140818df0）—— 窗口/目标判等；更新公式未实现

激活窗口 `t<0.72||t>=0.9`（修恒 false 硬伤）；目标 `+0xd==0 → r*3.0+(1-r)`、
否则 `r*4.0+2*(1-r)`；初始 1.0；[0,1] 钳制。未实现：更新累加公式
`(target*delta)/divisor+current`（divisor=fVar52 被打散，需 reva High P-code）、
收集主循环、rng 种子、retain 谓词。

## 交接注记

- 优先 fresh：blueprint.rs 12 函数、cull_oob 族、merge_proposal 族、visual 生成
  族按族分批、stairs assemble 4 闭包、ui_systems 21 函数。
- 已取 fresh 未逐语句对：`restore_archived_decorators`、`cache_decorator_
  transforms`、`handle_external_decorator_cmds`、`check_for_door_stairs`、
  `continue_stairs`（证据已落盘，可直接续做）。
- 集成缺口（FIX-10）：5 个判等/部分函数无调度注册点，转接线登记。
- 常量勘误：window_lights 0.5097→0.51f、0.7199→0.72f（read_memory 核验，
  reviewer 独立复核成立）。
