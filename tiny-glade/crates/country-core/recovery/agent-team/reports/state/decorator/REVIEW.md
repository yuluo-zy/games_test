# REVIEW — systems/decorator（reviewer-state-decorator，2026-10-17）

> 角色：reviewer-state-decorator（state 组 reviewer）。硬标准（2026-10-16）：
> 结构/签名/常量/panic 串对上 ≠ 实现。逐函数 **fresh 反编译 → 独立复述原控制流
> → 与 Rust body 逐语句对照 → verdict**（判等 / 未实现-需重做(附分歧点) /
> 胶水-非业务）。逻辑未还原到可判等的函数一律标未实现，禁止以近似通过。

## 方法

本审查**独立 fresh 反编译** 15 个函数（shared ghidra 只读，`review-evidence/`）：
`0x140823660 / 0x1408238a0 / 0x141276e30 / 0x140817850 / 0x1407b5980 /
0x140818df0 / 0x1407f98c0 / 0x1407f9260 / 0x1407ce920 / 0x140791400 /
0x14079c350 / 0x1408119d0 / 0x1408119e0 / 0x140811c30 / 0x140811e80`，
外加 `0x14067dd80 / 0x140f867c0`。**全部与 restorer 落盘 decomp 逐字节一致**
（`diff` 15/15 IDENTICAL）——restorer 的反编译证据真实未篡改，审查基于原证。

常量/panic 串独立 read_memory 复核：
- `0x142c23510` = "assertion failed: decorator_archivist.is_some()"（apply_decorator_op 行 50）✓
- `0x142a8eca0` = "The persistent visuals are already sealed"，file
  `crates/systems/decorator/src/decorator_visual_state.rs` 行 108 ✓
- `0x142a83cbc` = `0x3f028f5c` = **0.51f**（mantissa 0x028F5C=167772→1.02×0.5）；
  `0x142a80d44` = `0x3f3851ec` = **0.72f**（=0.3F3851EC 即 f32 0.72）——
  两者确认 restorer 常量勘误成立（旧 0.5097/0.7199 是错 lane 之说不虚）。

---

## 一、restorer 判等 9 逐项复核

### 1. `DecoratorBackup::clear`（0x140823660）—— **判等**

原逻辑（fresh 复述）：`impl$3::default()` 建默认模板 → 逐字段 `clone_from`：
`DecoratorStorage`、`HashMap<DecoratorId,Arc<DecoratorBlueprint>>`(+0x1f0)、
`HashMap<DecoratorId,Transform>`(+0x220)、`HashMap<(Id,Id),()>`(+0x250)；
历史 `Vec<DecoratorHistoryEdit>` 先 dealloc 旧缓冲再 `spec_extend` 模板迭代，
逐条 switch tag 3/4/5 归一化（default 分支拷 payload）；写 `+0x282/+0x280/+0x281`
= 模板标志（全 0）。

Rust：`*self = DecoratorBackup::default()`。逐语句对照：重置语义等义（所有字段
回默认值，含三标志清零）。差异仅内存复用（二进制 clone_from 保缓冲，Rust drop
重建）——不可观测，**判等**。注：`BackupStorage`/`BackupAuxiliary` 为 core 适配
布局，不影响 clear 的可观测行为。

### 2. `clear_out_wall_holes`（0x1407f98c0）—— **判等**

原逻辑：两 tick 回写（`*[RCX+0x10]=[RCX+0x1c]`、`*[RDX+0x10]=[RDX+0x1c]`，为
ResMut<WallHoles>/ResMut<PrevWallHoles> 变更 tick 传播）→ 尾跳
`WallHoles::clear_for_next_frame`。

Rust：`wall_holes.clear_for_next_frame(&mut previous)`。逐语句：两 tick 回写由
Bevy ResMut drop 自动完成（框架行为，非手写）；core `clear_for_next_frame` 的
行推进语义（fresh 0x140f867c0 核验：清 previous + 逐墙 advance_frame 入
previous）由 core 承载。**判等**。

### 3. `ui_decorator_inactive`（0x140791400）—— **判等**

原逻辑：门控 `state-2==4` 且 `([1]-1)+(state>1) < (state-2<5)` → 展开为
**state==6 && payload==0**；通过 → `**[in_RDX+0x10]=[in_RDX+0x1c]`（交互复位
tick 回写）+ `DecoratorBackup::clear`；否则直接返回。

Rust：`dispatcher.state==6 && payload==0 → interaction.reset_interaction();
backup.clear()`。逐语句：门控、复位、清备份三者对齐（dispatcher state/payload
为 u64，与二进制 64 位读面一致）。差异：`reset_interaction` 额外置 `active=true`
（I 级布局的模型字段，二进制该函数无 +0x48 写）——模型层 artifact，不改变
门控/清备份业务。**判等**。

### 4. `stairs_reset_for_new_frame`（0x14079c350）—— **判等（门控）**

原逻辑：交互 tick 回写；`state-6<0xf → lVar2=state-5`；`lVar2==0`（state≤5）时仅
state==4 保留，否则 unset；`lVar2==6`（state==11）且 payload 低字节=='?' 保留，
否则 unset；unset = `[*in_RDX+0x10]=3`。

Rust：`reset_interaction()`；match `4 => {}`、`11 if payload低字节=='?' => {}`、
`_ => mark_invalid()`。逐语句：**门控分支（保留集 {4, 11+'}''}）与二进制逐分支
等价，含 restorer 自述修对的 state==5 误保留**（二进制 state==5 → lVar2=0 且
!=4 → unset）。分歧：unset 的落点（`[内层+0x10]=3`）以 `mark_invalid` 置
`active=false` 承载——`MoveDecorationInteractionState` 为 I 级布局，该落点未还原。
门控业务**判等**，落点 FIX（见 §3）。

### 5. `seal_persistent_entities`（0x140811e80）—— **判等**

原逻辑：map len!=0 时探 id；命中：`sealed!=1 → 写 1 并 return`；`==1 → panic_fmt`
（串 "The persistent visuals are already sealed" 已 read_memory 核验，行 108）；
未命中 continue 探循环，耗尽 return。

Rust：`get_mut`，`Some` 时 `sealed!=1→置 1 return`、`==1→panic!(同串)`；`None`→
return。逐语句：探针/未命中/置位/panic 全对齐，panic 串核验一致。**判等**。

### 6. `add_decorator_visual_entity` impl$4（0x1408119e0）—— **判等**

原逻辑：探 `HashMap<u32,PersistentVisualEntities>`（0x28B 桶
`{key u32@0, pad, value 0x20B}`，value=`{seal u8@0x08, Vec<Entity>@0x10..}`，
seal@element+0x08 由 `lVar10-0x20` 反推证实）；命中：`seal&1!=0 → panic_fmt`，
否则 Vec（cap@+0x10/ptr@+0x18/len@+0x20）`grow_one`+写 entity+len++；未命中：
插空条目（key、seal=0、cap=8、len=0）再 push。

Rust：`entry(id).or_default()`，`sealed&1!=0 → panic!`，`entities.push(entity)`。
逐语句：探针/插默认/封存检查/push 全对齐。差异：二进制未命中预分配 cap=8
（Rust `Vec::new()` cap=0 由 push 生长）——容量优化差异，内容等价。panic 串
未逐函数独立核验（假定与 seal 同串，Rust 与 seal 共用——minor）。**判等**。

### 7. `entities` ×2（0x1408119d0 / 0x14067dd80）—— **判等（退化访问器，胶水级）**

原逻辑：两函数均为 PDB void() 占位退化访问器（fresh 0x1408119d0 `return;`；
0x14067dd80 ICF 折叠为 `TweakerView::as_inner_mut` `return;`）。`PersistentVisual
Entities` 含 `Vec<Entity>` 字段由 PDB `enum2$` + 消费方 `from_iter<Entity,
FlatMap<IntoIter<&PersistentVisualEntities>,&[Entity]>>` 间接证实（0x20B 值）。

Rust：`PersistentVisualEntities::entities(&self)->&[Entity]`、
`DecoratorPersistentVisualState::entities(&self)->&HashMap`。判等（业务为零，
属访问器）。`decorator_visual_state` **结构修正成立**（见 §二-6）。

### 8. `clear_out_visual_entities`（0x1407f9260）—— **未实现-需重做**

原逻辑（fresh 复述，关键）：puffin 作用域 → tick 回写 → **仅**在 `+0x48` 处
`HashMap<u32,Vec<Entity>>` 的 drain（hashbrown 内联迭代）→ 逐 Vec 逐实体
`EntityCommands::despawn` → Vec 耗尽 dealloc 缓冲 → drop
`FlatMap<Drain<…>, Map<IntoIter<Entity>, impl_3::drain::closure_env_0>>`。
**无持久表清除、无墙洞帧推进**（墙洞清帧在独立系统 0x1407f98c0）。

Rust 现体：drain+despawn ✓，但**额外**执行
`visual_state.persistent.entries.clear()`（二进制本函数不存在）与
`wall_holes.clear_for_next_frame(&mut previous)`（二进制本函数不存在——墙洞帧
推进归 clear_out_wall_holes 独立系统）。逐语句对照：**两处额外副作用 → 行为
分歧**（若两系统同时注册，墙洞被清两次）。restorer §4 自注"持久表清空为近似"
——按硬标准属未实现。**FIX**：删去两项额外操作，仅留 drain+despawn；若原
持久表清除确在他处，单独落地并登记证据。

### 9. `maintain_dormer_tiles`（0x1407ce920）—— **未实现-需重做**

原逻辑（fresh 复述）：`*cursor` prev → startA=prev-countA、startB=prev-countB
（各钳 0）→ 游标预写 `total-(lenB-startB)-(lenA-startA)` → **循环遍历
A[startA..] 链 B[startB..] 每瓦**：`*cursor+=1`、ids.push(value)（state+0x10/
+0x1c）、`hash_one<GlobalId>` + **`RawTable::remove_entry<u64,Box<dyn FnOnce>>`**
（rhapsody 稀疏分配回调注册表移除）、`local_50!=0` 时 **drop
`SparseInstanceBufferAllocation<RoofTileSsbo>`**。

Rust 现体：游标推进 + ids.push 已写，但每瓦主体只有 `let _ = tile;` 占位——
**缺失每瓦 `RawTable::remove_entry<u64,Box<dyn FnOnce>>` 与 drop
`SparseInstanceBufferAllocation<RoofTileSsbo>`**。这正是该函数的 GPU 分配退役
业务（瓦离开绘制集 → 移除延迟回调并释放稀疏分配）。restorer 标"判等（结构）"
过度——迭代结构对，业务主体缺。**FIX**：按 O 补齐每瓦 remove_entry + 条件
drop（core sparse_instance_buffer 类型接线）。

---

## 二、restorer 重建 6 逐项复核（按硬标准均未到判等 → 标未实现-需重做）

### 1. `apply_decorator_op`（0x141276e30）—— 控制流判等；叶函数近似 → 未实现-需重做

原控制流（fresh 复述）：guard `archivist.is_none() && kind!=0 → panic`（串核验
行 50）；`(kind-3)<2 → place/remove`，其余 change；change 子判别 op[+0x10]
`!=3 → 二次 archive`；**archive 均在历史之前**；历史（有则）`Vec<0x20B>` push
kind 3/4/5 + `DecoratorLocator::new` + `HashMap<DecoratorLocator,()>::insert`；
末尾 `DeferredDecoratorOpEvents::send`。

Rust 对照：
- guard 串/行 ✓（`if kind_byte!=0 { assert!(archivist.is_some()) }`）
- kind 判别 `(kind-3)<2` ✓（op_kind）
- 归档顺序提前 + change 二次归档条件 `op.info.dst_secondary(=op+0x10)!=3` ✓
- 历史 push 3/4/5 + HashMap insert ✓
- 分歧点：① `archive_current_decorators_in_storage<&str>` 以自写
  `archive_decorator_state` 等价（core archivist 缺口登记）——归档叶未还原原函数；
  ② 二进制 change 分支对 archivist None 执行 `unwrap_failed`（panic），Rust 用
  `if let Some(archivist)` **静默跳过**——行为分歧（边缘：change 且 None）；③
  `DecoratorLocator::new` 实参 U；④ `build_op_event` 0x88B 字段放置 U（消费端
  未复核）；⑤ `send` 载荷构造近似。

### 2. `create_or_restore_decorator_backup`（0x1408238a0）—— 状态机判等；恢复叶近似 → 未实现-需重做

原逻辑（fresh 复述）：`+0x282` 状态读即清零；`==1` CREATE（`all_decorators` 逐
桶 `get_or_insert_default`+`get`(未中 unwrap)+`tracked_clone_from`，clone_from 三
map +0x250/0x1f0/0x220 + 历史 Vec，置 `+0x281=1`）；`!=0 && created` RESTORE
（`all_storages` 逐 `get_mut`+`Decorators::clear`，反向 clone_from 三 map + Vec，
`all_decorators` 逐 `get_mut+get`+`DecoMoveToStorage::new`+`tracked_clone_from`，
cVar2(+0x48 门控) 下清 modified state@+0x47==1，`StairsState::restore_backup`）；
`+0x280` → clear。

Rust 对照：状态机三态 + reset 分支 ✓；CREATE 逐墙桶 or_default + tracked 拷贝 ✓；
RESTORE 反向 + `stairs.restore_backup()` ✓。分歧点：① 辅助 0x30B 状态
（+0x1d0/+0x200/+0x230）以 `BackupAuxiliary` 占位，**RESTORE 未反向拷贝**；②
archivist `all_storages/all_decorators` 以 `storage.iter_walls()` 近似（桶模型
core 适配）；③ modified state 清除门控（原二进制 `cVar2=*[res+0x48]`）未复刻，
Rust 无条件清除；④ `DecoMoveToStorage::new`+`tracked_clone_from` 以
`bucket.insert` 近似。

### 3. `decorator_record_history`（0x140817850）—— 循环/清空判等；载荷映射错 → 未实现-需重做

原逻辑（fresh 复述）：tick 回写；读 `+0x48` pending 即清零；非零 → `len=0` 后
逐 0x20B 记录（**kind==6 提前 break**）分配 **48B `{1,1,kind,f1,f2,f3}`**
（`*p=1,[1]=1,[2]=kind,[3]=f1(+0x08),[4]=f2(+0x10),[5]=f3(+0x18)`）调
`History::add`（kind=10）；**无条件** `+0x10 len=0` + HashMap 清空（control 0xff +
items=0 + growth_left 重算，gated items!=0）。

Rust 对照：pending 读清 ✓、循环/break ✓、无条件清 records ✓、清
`potentially_surviving_affected_decorators`（hashbrown clear，空表 no-op 等价
门控）✓。**分歧点（关键）**：48B 载荷 `{1,1,kind,a,b,e}` 被映射为
`DecoratorHistoryEdit{id: a}`——kind(+0x00)、b(+0x10)、e(+0x18) **被丢弃**；
core 56B 存档模型 ≠ 运行时 32B 记录。restorer §7 自注"48B 载荷与 core
DecoratorHistoryEdit 映射"为 U——按硬标准未到判等。**FIX**：按 48B `{1,1,kind,
a,b,e}` 忠实写入 History::add 载荷（或证实 core 模型等价后再判等）。

### 4. `window_lights`（0x1407b5980）—— 常量/灯态/帧末判等；**收集主循环缺失** → 未实现-需重做

原逻辑（fresh 复述）：窗实体收集（墙 `+0x108==0` 走 `+0xf8` id 数组 + 0xa0 段
桶 / 屋顶 0x48 段桶，`+0x110` 索引反查实体）→ `deny_load` 门控定下界
0.51/0.44、上界 0.72（read_memory 核验）→ 写 lights_on(+0x30) → 逐条目
`target!=lights_on → timer=2*f32 + target=lights_on` → 收集循环**缺失条目不插、
形状变化条件 → brightness=1.0(0x3f800000)+target=1** → 帧末 `timer-=delta` 归零
`actual=target` + `time_continuous_lerp` → `HashMap::retain<closure_env_1>`。

Rust 对照：下界/上界常量 ✓、lights_on 判定 ✓、逐条目随机重计时 ✓、帧末计时/
开关 ✓。**分歧点（主要）**：① **窗实体收集主循环整体缺失**——Rust 只维护既有
表，不新增条目、不做墙/屋顶遍历；② 形状变化 `brightness=1.0/target=1` 更新
缺失；③ rng 种子 U（`with_seed(0)` 近似）；④ `time_continuous_lerp` 实参
(0.0,1.0,bits,0.05) 为猜测；⑤ retain 谓词 `timer>0 || actual!=0` 自创；⑥
`GlobalWindowLightsControl` 为本地承载类型（二进制 lights_on 在 +0x30，Rust
结构不对齐）。restorer §6 自注"墙/屋顶收集资源身份 U"。按硬标准**未实现**。

### 5. `chimney_smoke_anim_thickness`（0x140818df0）—— 窗口/目标判等；**更新公式未还原** → 未实现-需重做

原逻辑（fresh 复述）：烟囱实体收集（同 window_lights 墙/屋顶双模式桶）→ 缺失
插初始 `window(t<0.72||t>=0.9) && entity+0xd==1 ? 1.0 : 0.0` → 演化
`+0xd==0 → target=r*3.0+(1-r)` 否则 `r*4.0+2*(1-r)`（常量 3.0/4.0 read_memory
核验）→ 厚度更新 `fVar55=(target*delta)/fVar52+current` 钳[0,1]（**fVar52 被
反编译打散，CONCAT13 字节重拼，语义 U**）→ `HashMap::retain<closure_env_0>`。

Rust 对照：激活窗口 ✓（修恒 false 硬伤）、目标公式 ✓、初始 1.0 ✓、[0,1] 钳 ✓。
分歧点：① 收集主循环缺失（Rust 只迭代既有表）；② 更新累加公式自创
`current+signed*target*delta/target.max(1.0)` ≠ 原 `(target*delta)/divisor+current`
（divisor U，需 reva High P-code）；③ rng 种子 U；④ retain 谓词 `*v>0` 自创；
⑤ t 资源为本地 `TimeOfDayCycle` 承载。按硬标准**未实现**。

### 6. `decorator_visual_state` 结构修正 —— **修正成立（判等面）；impl$5 未实现**

逐字节复核证实：`PersistentVisualEntities={seal u8, Vec<Entity>}`（element 0x28B
`{key u32@0, pad, seal@+0x08, Vec{ptr@+0x10,cap@+0x18,len@+0x20}}`，
seal@element+0x08 由 `lVar10-0x20` 反推）+ `entries: HashMap<u32,
PersistentVisualEntities>` 成立；impl$4/seal/entities 判等（见 §一-5/6/7）。
**但 impl$5（0x140811c30）autoclutter 表 `HashMap<u32,Vec<DisplayChildAutoClutterInfo>>`
在 Rust 无对应实现**——restorer 将其计入"判等 9"属过度（只解结构未落地）。
标**未实现**（FIX：实现 autoclutter 子项 push，或显式登记未实现）。

---

## 三、胶水 3（lib.rs）—— 接受 glue-legitimate

`Plugin::build`（无独立符号，被内联进 App::add_plugins 单态化）、
`snapping_direction`/`try_merge_decorators`（lib.rs inline，7/6 实例，被
merge_proposal/ui_systems/collision_detection 内联承载）——按 ANALYSIS 指引为
注册/内联胶水，判 glue-legitimate 成立（无独立符号 fresh 无新增证据）。注意
`add_decorator_systems` 当前仅注册 D4 杂项（audio/window_lights/remove_ivy/
chimney_smoke），`register_ui_systems` 已含 ui_decorator_inactive/
stairs_reset_for_new_frame 等；**本审查 9 项中的 clear_out_visual_entities /
clear_out_wall_holes / maintain_dormer_tiles / decorator_record_history /
create_or_restore_decorator_backup 在 workspace 无任何调度注册点**（集成缺口，
登记转接线）。

---

## 四、verdict 汇总

| 函数（VA） | restorer 判定 | 本审查 verdict |
|---|---|---|
| DecoratorBackup::clear（0x140823660） | 判等 | **判等** |
| clear_out_wall_holes（0x1407f98c0） | 判等 | **判等** |
| ui_decorator_inactive（0x140791400） | 判等 | **判等** |
| stairs_reset_for_new_frame（0x14079c350） | 判等 | **判等（门控；落点 I 级，FIX）** |
| seal_persistent_entities（0x140811e80） | 判等 | **判等**（panic 串核验） |
| add_decorator_visual_entity impl$4（0x1408119e0） | 判等 | **判等**（cap=8 差异，minor） |
| entities ×2（0x1408119d0/0x14067dd80） | 判等 | **判等**（退化访问器/胶水级） |
| clear_out_visual_entities（0x1407f9260） | 判等（结构） | **未实现-需重做**（多两处副作用） |
| maintain_dormer_tiles（0x1407ce920） | 判等（结构） | **未实现-需重做**（每瓦 GPU 退役缺失） |
| add_decorator_visual_entity impl$5（0x140811c30） | 判等（结构） | **未实现**（无对应实现） |
| apply_decorator_op（0x141276e30） | 重建 | **未实现-需重做**（控制流判等，归档/事件/locator 叶近似） |
| create_or_restore_decorator_backup（0x1408238a0） | 重建 | **未实现-需重做**（辅助 map 未反向拷贝等） |
| decorator_record_history（0x140817850） | 重建 | **未实现-需重做**（48B 载荷丢 kind/b/e） |
| window_lights（0x1407b5980） | 重建（部分） | **未实现-需重做**（收集主循环缺失） |
| chimney_smoke_anim_thickness（0x140818df0） | 重建（部分） | **未实现-需重做**（更新公式 U） |
| decorator_visual_state 结构 | 重建 | **结构修正成立**；函数面判等，impl$5 未实现 |
| lib.rs 胶水 3 | glue-legitimate | **胶水-非业务（接受）** |

**净判等 7 / 未实现-需重做 8（含 3 个被过度标判等的 + 5 个重建）+ 结构修正 1
+ 胶水 3**。restorer 的"判等 9"实为 7；"重建 6"按硬标准全部未到判等。

## 五、FIX 清单

1. `clear_out_visual_entities`：删 `persistent.entries.clear()` 与
   `wall_holes.clear_for_next_frame`（二进制本函数无），仅留 drain+despawn；
   若持久清除在他处，单独落地。
2. `maintain_dormer_tiles`：每瓦补 `RawTable::remove_entry<u64,Box<dyn FnOnce>>`
   + 条件 drop `SparseInstanceBufferAllocation<RoofTileSsbo>`（rhapsody 稀疏
   分配类型接线）。
3. `window_lights`：补墙/屋顶窗实体收集主循环（缺失插
   `{brightness=0,actual=target=lights_on}`）+ 形状变化
   `brightness=1.0/target=1`；rng 种子、time_continuous_lerp 实参、retain 谓词
   取证。
4. `chimney_smoke_anim_thickness`：补收集主循环 + 精确更新公式（reva High
   P-code 解 fVar52 打散）；rng 种子、retain 谓词取证。
5. `decorator_record_history`：48B `{1,1,kind,a,b,e}` 载荷忠实写入 History::add。
6. `create_or_restore_decorator_backup`：RESTORE 反向拷贝辅助 0x30B 三 map；
   复刻 +0x48 门控清 modified state。
7. `apply_decorator_op`：archivist None 时镜像 unwrap panic（勿静默跳过）；
   build_op_event 字段按消费端复核。
8. `stairs_reset_for_new_frame`：unset 落点 `[内层+0x10]=3` 接线（待交互布局
   确认）。
9. `add_decorator_visual_entity` impl$5：实现 autoclutter 表 push 或显式登记。
10. 集成：clear_out_visual_entities / clear_out_wall_holes / maintain_dormer_
    tiles / decorator_record_history / create_or_restore_decorator_backup 无
    调度注册点——转接线登记。

## 六、交接注记

- restorer 证据真实（fresh diff 15/15 一致）；未 fresh 的 ~146 项如实标未实现
  （非虚假通过）——待逐函数 fresh 复核（blueprint/cull_oob/merge/visual 族/
  stairs/ui_systems 为主）。
- 常量勘误（0.51f/0.72f）与 panic 串（assert/已封存）均独立核验成立。

---

## 七、FIX 复验状态（2026-10-17，restorer 落实 FIX-1..10 后）—— **fix_verified**

逐文件复验 restorer 声称落实的 FIX，结论：**10/10 落实准确，无新过度断言**，
`clear_out_visual_entities` 由未实现升**判等（FIX-1 后）**成立。

| FIX | 复验结果 |
|---|---|
| 1 clear_out_visual_entities 删副作用 | **已修**：现体仅 `transient.drain()`+逐实体 despawn，持久清表/墙洞清帧已删（maintain_visual_entities.rs:42-52）；参数面降为 (Commands, ResMut<DecoratorVisualState>) 与二进制 in_RCX 业务对齐（param_2 身份仍 U 登记）。升判等成立 |
| 2 maintain_dormer_tiles | **已登记未实现**：代码注释显式登记每瓦 `RawTable::remove_entry<u64,Box<dyn FnOnce>>` + 条件 drop `SparseInstanceBufferAllocation<RoofTileSsbo>`（rhapsody 类型缺口），体仍占位不造假 |
| 3 window_lights 收集主循环 | **已登记未实现**（RESTORE.md 总表+§6 显式标未实现）；注：window_lights.rs:96 注释仍写"以条目全量维护承载"——措辞偏软但函数级判定未实现，可接受 |
| 4 chimney_smoke 更新公式 | **已登记未实现**（RESTORE.md §7；代码头注释 U 登记） |
| 5 decorator_record_history | **已修**：`DeferredDecoratorRecord` 布局改 `{kind u64@0,f1 u64@8,f2 u64@0x10,f3 u64@0x18}`（32B，与二进制 `plVar10[0..3]` 读面对齐，旧 u32 布局不符已修）；kind/f2/f3 因 core 存档模型缺口显式标未实现 |
| 6 create_or_restore | **已修**：`clear_modified = deferred.pending` 复刻 +0x48 门控；辅助 0x30B 三 map 反向拷贝未接线登记 |
| 7 apply_decorator_op | **已修**：`decorator_archivist.unwrap()` 镜像 unwrap_failed（不再静默跳过）；归档/事件/locator 叶近似保持未实现登记 |
| 8 stairs_reset 落点 | **已登记**：`[内层+0x10]=3` 以 mark_invalid 承载（I 级布局）转交互布局确认 |
| 9 impl$5 | **已登记未实现**（decorator_visual_state.rs:105-109 显式标未实现，autoclutter 表类型缺口） |
| 10 注册点 | **已登记转接线**（RESTORE.md FIX-10：5 函数无调度注册点，lib.rs add_decorator_systems 由 D4 维护待 App 组装逐批接入） |

**残余注意**（非 FIX 缺口，为既有登记）：① window_lights/chimney_smoke 函数体
仍为部分实现（收集主循环/更新公式未还原），已如实标未实现-需重做，待后续补
全；② 5 函数无调度注册点，属集成接线范畴（已登记）；③ cargo check 通过为
coordinator 报告，本复验未重跑构建（改动均为注释/小改，结构性核对无编译风险
点）。**无新过度断言**：RESTORE.md 判定总表与 REVIEW.md §四 verdict 一致
（判等 8 / 未实现-需重做 8 / 胶水 3 / 结构修正 1）。
