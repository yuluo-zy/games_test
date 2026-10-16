# restorer-decorator — decorator 族 RESTORE 报告（卡5）

日期：2026-10-16
状态：in_progress（restorer 已交付，待 reviewer）
涉及文件（新建）：`resources/walls/decorator.rs`、`resources/walls/decorator_storage.rs`、`resources/walls/decorator_subtype.rs`；`resources/walls/mod.rs`（+3 模块声明）
证据包：`reverse-engineering/tiny-glade/recovery/agent-team/reports/eco/walls/RE-DECORATOR-STORAGE.md` + 章节 `chapters/resources/walls/{decorator_storage,decorator,decorator_subtype}.rs.md` + `wall/decomp/*`

## 〇、核心结论（实体机制，O）

**DecoratorStorage 是纯数据权威资源**（`HashMap<WallId, Decorators>` 分墙桶 + `decorator_to_storage` 索引 + terrain/stair 内联桶 + immutable 标志），**不含 bevy Entity**（RE §2）。装饰实体由消费系统读 DecoratorInfo 后另行生成，实体↔装饰关联靠 DecoratorId 组件。三文件实现即按此：存储层只做 DecoratorInfo 72B 数据记录的增删查/迁移/快照，不 spawn 任何实体。

## 一、原函数 → Rust 位置 → 证据

### decorator.rs（原 131 函数：业务 56）
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| DecoratorType 8 谓词 | 0x140833130..c0 | decorator.rs:34-88 | 章节 §DecoratorType 谓词位运算族（全部 O）+ RNE 手解 can_place_on_stair_top |
| DecoratorId new/get_inner/Display | 0x14067dd80 / 行166 / 0x1408331d0 | decorator.rs:97-132 | decomp；serde=u32 十进制化（RE 纠错，0x1402ddd20） |
| DecoratorIdGen reserve/generate | 行806/811 | decorator.rs:139-159 | 内联（I） |
| TransientDecoratorIdGen new/generate | 0x1408339c0/9e0 | decorator.rs:167-212 | 章节 §TransientDecoratorIdGen（O） |
| DecoratorRank new | 0x140833250 | decorator.rs:220-224 | mov eax,ecx 字节直通（O）；is_atomic/is_compound I |
| DecoratorLocator::new | 0x140833240 | decorator.rs:236-241 | I（字段 U） |
| ClutterVersion is_initial/latest | 0x140833230 | decorator.rs:248-264 | cmp byte,0 sete（O） |
| DecoratorExtraInfo as_*/try_as_* | 0x1408392d0..4b0 | decorator.rs:270-449 | decomp 判别字节 +0x11 访问（O） |
| DecoratorExtraInfo From 变体4-8 | 0x140839520..560 | decorator.rs:451-482 | rne-read 手解（判别 4/5/6/7 + 变体8 无判别整载荷拷贝） |
| DecoratorInfo dst_as_* | 0x140833260..470 | decorator.rs:565-619 | decomp（tag<2 → &self / tag==3 → &self 等） |
| DecoratorDst is_wall/as_roof/as_wall(_mut) | 0x1408334d0..540 | decorator.rs:486-560 | decomp（tag@+0x22） |
| DecoratorOpEventKind::id | 0x140833570 | decorator.rs:720-743 | decomp 返回 u32（PDB as_wall_mut 错位纠错） |
| DeferredDecoratorOpEvents send/unleash/events | 0x1408336b0/7c0/0x14067dd80 | decorator.rs:745-834 | decomp（0x88B 事件、send park 1e9ns） |
| DecoratorRotationXZ 6 方法 | 0x140833a00..b80 | decorator.rs:850-905 | decomp（to_quat 分量赋值 / to_projective_xz_mat2 4 浮点） |
| is_default | 行602 | decorator.rs:907-911 | 内联 |

### decorator_storage.rs（原 114 函数：业务 60）
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| Decorators iter_modified(_ids/_mut)/removed_ids | 0x140a1b030/070/0b0/100 | decorator_storage.rs:166-214 | 章节布局 +0x60 modified_ids / +0x00 removed_ids |
| Decorators tracked_clone_from / clear / merge_with | 0x141256e40/0x1412573b0 / 0x141257900 / 0x14155b170 | decorator_storage.rs:216-289 | 章节 §tracked_clone_from / clear / merge_with |
| DecoMoveToStorage::new | 0x140a1b430 | decorator_storage.rs:296-318 | 3 字段 {r8,r9,rdx}（O） |
| DecoratorToStorageMap remove/insert/update | 0x140a1b440/4e0/5a0 | decorator_storage.rs:92-135 | decomp（Arc 包装；panic "d2s remove: no such {id}" / "d2s insert: already exists; {id}"） |
| DecoratorStorageAddr as_wall/try_as_wall | 0x140a1c240/290 | decorator_storage.rs:28-75 | decomp（[+0]==0 → [+8]；try Option） |
| DecoratorStorage mark_mutable/mark_immutable | 0x140a1c2b0 / 行542 | decorator_storage.rs:340-351 | 清/置 +0x1b0 标志 |
| DecoratorStorage get/get_mut | 0x140a1c2c0/cde0 | decorator_storage.rs:354-370 | decomp（per_wall 探针 +0x98 步长；terrain/stair +0x90/+0x120 首 qword 门） |
| lookup_storage/try_lookup_storage | 0x140a1c3e0/510 | decorator_storage.rs:375-383 | d2s 索引查询 |
| decorator_info(_mut) | 0x140a1c5f0/800 | decorator_storage.rs:388-395 | 双层：索引→桶→内嵌 map |
| lookup_decorator_info(_mut) | 0x140a1c930/cbf0 | decorator_storage.rs:400-408 | decomp 双层（0x78 ctrl 门 + 24B 条目 + 0x50B 二段） |
| get_or_insert_default | 0x140a1cf30 | decorator_storage.rs:411-424 | decomp 21KB（immutable 断言；墙探针 / terrain/stair 内联懒初始化 +0x30 注册表） |
| remove_storage | 0x140a1dba0 | decorator_storage.rs:428-440 | I |
| wall_decorators/ids/mut、iter_terrain/stair | 0x140a1e190/300/420、0x140a1e280/2c0 | decorator_storage.rs:443-471 | 章节迭代器族（{tag=1, ptr=+0x90/+0x120}） |
| iter/iter_mut/iter_walls(_mut) | 0x140a1e5b0/630/100/540 | decorator_storage.rs:473-490 | per_wall 迭代 |
| iter_removed_storages | 0x140a1e6e0 | decorator_storage.rs:495-500 | [+0x30]/[+0x38]/[+0x48] |
| clear_modification_tracking | 0x140a1e720 | decorator_storage.rs:503-514 | I |
| on_walls_merged | 0x140a1ea40 | decorator_storage.rs:519-550 | decomp 21KB（merge_with + d2s remove/insert） |
| on_wall_removed | 0x140a1f550 | decorator_storage.rs:553-565 | I |
| on_wall_split | 0x140a1f590 | decorator_storage.rs:567-616 | decomp 46KB（immutable 门 + 装饰迁移 + 子墙插入 + d2s 更新；0x28B 记录语义 U） |
| apply_decorator_remove | 0x140a20670 | decorator_storage.rs:620-661 | decomp（modified_ids 移除 + removed 标记 + 内嵌 map 移除 panic "…doesn't exist: "） |
| apply_decorator_change | 0x140a20890 | decorator_storage.rs:665-679 | apply_decorator_remove 闭包（I 分支） |
| apply_decorator_place | 0x140a20c70 | decorator_storage.rs:683-702 | decomp 2.1KB（get_or_insert_default + 内嵌 map 写 + modified 标记） |
| take_snapshot/restore_snapshot | 0x140a20d70 | decorator_storage.rs:707-761 | decomp 26KB（前置 6 断言；16B XOR 掩码 0x1429cd080/090；旋转还原） |
| restore_decorators | 0x14155ba30 | decorator_storage.rs:763-772 | history replay_pop_bucket 闭包（I） |
| verify_integrity | 0x14003bb80 | decorator_storage.rs:775 | ret 空函数（O） |

### decorator_subtype.rs（原 49 函数：业务 20）
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| impl$0::first | 0x140002370 | decorator_subtype.rs:26-29 | xor eax,eax → 0（O） |
| DecoratorSubtype can_be_on_corner | 0x140b25b50 | decorator_subtype.rs:32-36 | 字节标志读取（I：corner=3 观察） |
| DerivedDecoratorSubtypes::get | 0x140b25b60 | decorator_subtype.rs:135-149 | decomp（HashMap<u32, Arc> 未命中 panic） |
| DecoratorLibKey::new | 0x140b25c90 | decorator_subtype.rs:65-70 | 两参交换闭包同址（O） |
| DecoratorCollision::empty | 0x140b25ca0 | decorator_subtype.rs:76-97 | decomp（Rectangle3d+parry3d cuboid+IsoTransform 全零） |
| DecoratorMeshLibrary insert/same/get_outline/get_collision_outline | 0x140b25ef0/0x140b262a0/0x140b26690/0x140b267f0 | decorator_subtype.rs:158-214 | 章节（Arc 克隆插 4 表；map3@+0x60 / map4@+0x90） |
| dims_wall_space | 0x141fdc7b0 | decorator_subtype.rs:219-227 | decomp（bbx 差值；panic "The decorator mesh library doesn't contain "） |
| DerivedDecoratorInfo subtype/try_get_subtype | 0x140b26970/980 | decorator_subtype.rs:118-130 | Arc 子类型访问 |
| impl$6/7 into_decorator_subtype | 0x140833ba0/0x140833250 | decorator_subtype.rs:41-100 | 尾跳转发 / 字节直通 |
| impl$8::from | 0x140b26a70 | decorator_subtype.rs:233-238 | 双字段提取 {+0x0,+0x20}（I 目标类型） |
| queue/apply/new_archetype/init_state/validate_param | 0x14003bb80/0x140b26e40/0x14026c6a0 | decorator_subtype.rs:264-278 | ret 空函数（SystemParam derive 默认 no-op，O）+ validate_param 胶水排除 |

## 二、实现说明（关键决策）

1. **DecoratorInfo 72B 具名结构**（`repr(C)`，13 字段）：`dst_wall/+0x00`、`dst_secondary/+0x08`、`dst_payload/+0x10`、`u18/+0x18`、`u20/+0x20`（dst tag 字节 @+0x22）、`u24/+0x24`、`u28/+0x28`、`decorator_id/+0x2c`、`u30/+0x30`、`u38/+0x38`、`u3c/+0x3c`、`u40/+0x40`、`rank_ty/+0x44`（rank u8@0x44、type u8@0x46）。未知字段保留 raw 名，不编造业务语义（完整字段表 = RE §6.1 U 项）。
2. **DecoratorExtraInfo 判别位置偏离**：原二进制判别 @ DecoratorInfo+0x11（与 f32/u32 载荷字节 1 重叠，From 先写载荷后写判别 → 存储即破坏载荷字节 1）。Rust 侧判别存于 dst_payload 高位字节使载荷无损往返；偏离已登记注释（decorator.rs extra_disc）。
3. **Decorators::insert/apply_decorator_place 语义**：内嵌 map 写入 + modified_ids 标记（O）。Decorators::remove 断言 `self.map.remove(&id).is_some()` 对应 panic 常量 0x142ac9068。
4. **d2s 索引**：`DecoratorToStorageMap` = `Arc<HashMap<u32, DecoratorStorageAddr>>` 语义包装（原 +0x0/+0x8 Arc 计数、+0x10 map 指针）；insert/remove 带原 panic 消息。墙事件（merged/split/removed）同步维护。
5. **on_wall_split**：原 0x28B 装饰记录（u 区间/子侧状态字节）完整语义 U（RE §6.4），以调用方 `side_of(&DecoratorInfo) -> u8` 判定子侧；迁移逻辑（原墙移除、子墙默认构造 + removed 标记、d2s 更新 + 二段移除 panic 0x142ac9300）按 decomp 结构实现。
6. **快照**：16B XOR 掩码（_UNK_1429cd080/090）数值 U，暂 0；编码 = `rotate_right32(id) XOR mask`，解码 = `rotate_left32 XOR mask`（O 公式）。take_snapshot 前置 6 断言按 decomp 实现，快照记录收集留 I。
7. **serde**：DecoratorId = u32 十进制化（newtype serde，RE 纠错）；DecoratorStorageAddr/DecoratorDst 因 WallId 未实现 serde 不派生（wire 格式 U）。

## 三、依赖复用边界
- `WallId`（resources/walls/mod.rs，B 0x140b34f20）复用；未给其补 serde（不在本批范围）。
- glam（Vec2/Vec3/Vec4/Mat2/Quat）、serde、bevy_ecs（Resource/SystemParam/Event 语义）、utils::geometry::Rectangle3d 复用依赖。
- DeferredDecoratorOpEvents 的 futex park（1e9ns）/全局等待队列 = std 并发内部；Rust 侧 Mutex+Condvar 语义等价。

## 四、仍缺失/待取证（边界外，不伪造）
1. **DecoratorInfo 72B 完整字段表**（+0x00..0x20 与 +0x24..0x44 逐字段语义；ExtraInfo 联合载荷）——RE §6.1 U。
2. **d2s 24B 条目判别/payload 语义**（Wall/Terrain/Stair 之外 roof 第 4 目的地）——RE §6.2 U；本实现仅 Wall/Terrain/Stair。
3. **装饰实体生成系统 VA**（render nani / ui_clutter spawn）——RE §6.3 确认"实体不存于 DecoratorStorage"，生成器待 render 域定位。
4. **on_wall_split 0x28B 记录**（u 区间/子侧状态字节）——RE §6.4 U；本实现以 side_of 闭包注入。
5. **DecoratorStorageAddr/DecoratorDst wire 序列化**（WallId 无 serde）——U。
6. **apply_decorator_change 精确分支**（apply_decorator_remove 闭包）——I。
7. **DecoratorRotationXZ to_quat 乘数**（_UNK_142925870）与 from_xz_forward_vector 符号约定——U。
8. **on_walls_merged/on_wall_split 与 publish_public_wall_state（0x140a8b530）事件消费接线**（OnWallChanged/OnWallDeleted → on_wall_*）——wall 发布链库外接线，登记。
9. **ui_focus_decoclutter（0x1409a92c0）→ DecoratorStorage::get → DecoratorInfo 消费路径**——ui_clutter 组消费端，登记。

## 五、编译与测试
- `cargo check -p country-core`：**通过**（lib；其余组在途文件仅警告，无 error）。
- `cargo test -p country-core --lib`：**1626 过 / 1 败**（败项 geometry::sphere 为他组既有，非 decorator 族）。
- decorator 族 17 测试全过：谓词位运算全 u8 值域核对、info/dst/extra 访问器、TransientIdGen 两模式、rotation quat 往返、存储增删查/合并/分裂/快照编码往返/immutable 门、mesh 库 4 表插查、panic 路径。

## 六、inventory 建议
- `resources/walls/decorator_storage.rs`、`resources/walls/decorator.rs`、`resources/walls/decorator_subtype.rs`：新建在位，建议 reviewer 按本报告逐函数核对（重点：DecoratorInfo 字段表、d2s 语义、on_wall_split 迁移、快照掩码、实体机制边界）。

## 七、卡5 待补项修复（restorer-decorator，2026-10-16；reviewer-eco 终验反馈）

依据：fresh 反编译 `ghidra-decorator-followup`（0x140a20c70 / 0x14155afe0 / 0x140a1f590，--worker shared 只读），结果落盘 `evidence/country-core-deep/eco-reanalyst/decomp/decorator-followup/result.json`。

### 待补项 1：DecoratorInfo 72B 读面 +0x48/+0x4c/+0x50/+0x55/+0x56 → 裁决
- **结论**：`+0x48/+0x4c/+0x50/+0x55/+0x56` **属于包裹记录（op 事件），非 DecoratorInfo**。fresh `0x14155afe0`（Decorators::insert<DecoMoveToStorage>）：`HashMap::insert<DecoratorId, DecoratorInfo>` 用 `in_R8D`（= +0x4c）作插入键；输出 72B 结构（`in_RCX`）逐字段写回 +0x00..+0x47，并在 `if (cStack_31==2)` 分支写 **`*(in_RCX + 0x47) = 2`**（状态字节=live/placed）。内嵌 map 条目 0x50B={u32 key, pad, DecoratorInfo 0x48B@8} 证实 DecoratorInfo 恰 72B。
- **修复**：结构字段由 `rank_ty: u32` 拆为 `rank u8@+0x44`、`u45 u8@+0x45`、`ty u8@+0x46`、`state u8@+0x47`（+0x47 = 状态字节，O）；新增 `DecoratorInfo::state()`。`+0x4c`（插入键）与 `+0x2c`（记录内 DecoratorId）并存，二者独立——apply_decorator_place 的 id 参数即 +0x4c 键，info.decorator_id 即 +0x2c，均按原传递。
- **留存 U**：+0x48/+0x4c/+0x50/+0x55/+0x56 包裹记录逐字段语义；DecoratorInfo +0x45 字节。

### 待补项 2：on_wall_split removed 路径 → 裁决
- **结论**：fresh `0x140a1f590` 证实迁移语义为 **d2s 移除 + 记入 +0x30 注册表**（`HashMap<DecoratorStorageAddr, HashSet<DecoratorId>>`，rustc_entry + set insert），**不直接拷入子墙内嵌 map**；装饰 info 由消费方按注册表重新落位（archivist/重放）。子侧由记录状态字节 b 判定（`(b-2)<=2 ? b-1 : 0`，b∈{0,1}→侧0、b=2→侧1、b=3→侧2、b=4→侧3；仅侧 0/1 处理，侧 2/3 panic unreachable 0x142ac8d80），侧 0 再经 u 区间包含搜索定子墙。
- **修复**：`removed` 字段类型改为 `HashMap<DecoratorStorageAddr, HashSet<u32>>`（原 `HashSet<u32>` 错误）；`get_or_insert_default` 仅新建桶时取走注册表待恢复 removed ids 种子桶（recovered_building restore_one 观察）；`on_wall_split` 重写为逐装饰 d2s 移除（miss panic 0x142ac9300）+ 子墙桶存在 + 注册表标 removed；`iter_removed_storages` 改迭代注册表键。`Decorators::insert` 补 removed_ids 撤销（重新放置即撤销 removed，fresh 首段 remove_entry）。
- **留存 U**：0x28B 记录完整语义；装饰 info 重新落位的消费方接线（archivist/重放）；注册表取/留（consume vs keep）语义（I）。

### 测试
- decorator 族测试 18 全过（新增：on_wall_split removed 注册表断言、get_or_insert_default 注册表种子）。
- `cargo check -p country-core` 通过；`cargo test -p country-core --lib` **1632 过 / 3 败**——3 败均在**非本模块**（geometry::sphere 既有、resources/roofs/roof_noise ×2 他组在途），decorator 族无失败。

## 八、卡A1 接线（restorer-decorator，2026-10-16；reviewer-eco 终验）

### 1. wall 发布链接线（RE §5，O 链）
- 新建 `systems/wall/decorator_wall_events.rs`：**OnWallDeleted → `DecoratorStorage::on_wall_removed`**（真实删除清理链）。`publish_public_wall_state`（0x140a8b530）→ OnWallChanged/OnWallDeleted 16B 事件 → 本系统消费。
- `register_wall_cpu_resources` 注册 `DecoratorStorage` 资源；`add_wall_cpu_systems` 挂 `decorator_wall_events`（WallCpuStage::Delete，与 delete_wall 同段）。
- immutable 置位（快照/恢复）期间跳过（原 immutable 门控语义）。
- **登记 U**：on_wall_split/on_walls_merged 消费需墙分裂/合并拓扑（子墙 id + u + 0x28B 记录），当前事件仅携带 WallId——待 `wall_compound_edit.rs`（79 函数，卡A2）恢复；接线点已在 decorator_wall_events.rs 注释登记（`decorator_wall_change_pending`）。

### 2. ui_clutter 消费端（RE §4，O 主读路径）
- `ui_focus_decoclutter.rs` 重写：`DecoClutterFocusState{tag, decorator_id}` → `DecoratorStorage::lookup_storage(id)`（d2s 索引）→ `decorator_info(id)`（双层 → 内嵌 map）→ 未中任一直接返回（O：null 或 [Decorators+0x48]==0 → return）→ `DecoClutterFocused{decorator_id, addr, info}`。
- `register_ui_clutter_resources` 注册 `DecoratorStorage`。
- **登记 U/边界**：clutter_mesh_ty_data（0x1408b8e90）/UiState::add（0x1409231f0）/SignifierFactory/EventWriter×4 的实体移动/旋转联动属宿主接线；装饰实体生成（render nani 域）待 render 域定位。
- `DecoratorStorage::archive_current_decorators`（新增）：`archive_current_decorators_in_storage` 0x14123b5c0 的 O 读路径（逐墙 0x90B 记录 + 内嵌 map 拷贝）；archivist 侧 merge_with（0x14155a950）重插入接线属 recovered_building 归档类型边界（登记）。

### 3. decorator_archivist / cached_decorator_transforms 核对
- `decorator_archivist.rs`（库内）继续 re-export recovered_building `DecoratorArchivist<C>`/`DecoratorStorage`（garden draw_garden 以资源消费，独立既有类型）；本卡5 权威 `decorator_storage::DecoratorStorage` 与它并存，路径不同名不冲突。归档消费（archive_current_decorators → archivist backing）经 `DecoratorStorage::archive_current_decorators` 衔接，backing 类型差异登记。
- **cached_decorator_transforms.rs 无 src 文件**（仅章节 60 行 + inventory 条目，B 版 9.7MB 骨架）；属后续批次原文件，未在本接线范围，登记。

### 测试/编译
- `cargo test -p country-core --lib`：**1667 过 / 1 败**（败项 geometry::sphere 既有非本模块）；wall 12、ui_clutter 2、decorator 19、**decorator_wall_events 2**（新增：OnWallDeleted→on_wall_removed 清理链、immutable 跳过）全过。
- `cargo check -p country-core` 通过。

## 九、R2 cached_decorator_transforms.rs（restorer-decorator，2026-10-16）

实现：新建 `resources/walls/cached_decorator_transforms.rs`（原 11 函数：业务 7）。章节 `chapters/resources/walls/cached_decorator_transforms.rs.md` + `wall/decomp/decomp-0x140a1a9f0..af40.txt`。

### 原函数 → Rust 位置 → 证据
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| CachedDecoratorTransforms::transform | 0x140a1aa20 | cached_decorator_transforms.rs:110 | 查缓存取变换（未命中 panic，try 版 Option） |
| position | 0x140a1a9f0 | cached_decorator_transforms.rs:117 | transform(id).translation |
| try_transform / try_position | 0x140a1ac80 / 0x140a1ab80 | cached_decorator_transforms.rs:122-130 | Option 版 |
| insert | 0x140a1ada0 | cached_decorator_transforms.rs:134-157 | decomp：3 批有限性校验（translation[4..6]/rotation[0..3] movmskps==0xf/scale[7..9]）→ 对应 panic 串 0x142ac8e00/98/ee0 → FUN_14152aee0 落缓存 |
| swap_buffers_and_clear | 0x140a1aeb0 | cached_decorator_transforms.rs:160-163 | 双缓冲互换 + 清一册 |
| CachedDecoratorTransformStorage::get | 0x140a1af40 | cached_decorator_transforms.rs:74-78 | hashbrown 探针（+0x18 空门，+0x20 hasher） |

### 实现说明
- **缓存条目 = 10 f32 紧凑变换** `{rotation: Quat@[0..4], translation: Vec3@[4..7], scale: Vec3@[7..10]}`（insert 读面 O；章节"3×3 矩阵 9 分量"为近似，rotation 实为 4 分量 Quat）。`CachedDecoratorTransform` 具名结构 + `from_components/to_components`。
- **有限性校验**（O）：translation → "assertion failed: transform.translation.is_finite()"；rotation → "…rotation.is_finite()"（movmskps==0xf）；scale → "…scale.is_finite()"，任一 NaN/Inf panic。
- **双缓冲**：`buffers: [CachedDecoratorTransformStorage; 2]` + `current_write` 写入册索引（读取册 = current_write ^ 1）；swap 后清新写入册。读/写册归属 I（章节仅述互换）。
- **纯数据缓存，非实体存储**（卡5 实体机制一致）。

### 测试/编译
- cached_decorator_transforms 测试 **4 全过**（insert→swap→读、分量往返、三批 NaN/Inf panic、get 未中）。
- `cargo test -p country-core --lib` **1671 过 / 1 败**（败项 geometry::sphere 既有非本模块；本模块实现时现场验证）；`cargo check -p country-core` 通过。
- **注**：终验现场 `systems/collision/world.rs` 出现他组在途 error（RaycastWorld::raycast，卡A5），与本模块无关；以碰撞组收口后的现场 check 为准。
- 留存 U：双缓冲读/写册归属、swap_buffers 0xff+0x11 memmove 精确 drop 语义、消费于墙高/几何变化后装饰重定位路径（宿主接线）。

## 十、E2 取证对账：DecoratorInfo +0x45（restorer-decorator，2026-10-16）

依据：re-analyst-eco E2 取证（fresh 0x140a20670 apply_decorator_remove：移除拷贝 **0x48B = {8×u64@+0x00..0x40, state u8@+0x47}**；apply_decorator_place 读面 rank@+0x44、type@+0x48/+0x46 并存；**+0x45 = rank(+0x44) 与 type(+0x46) 间 1 字节，rank_ty/state 簇 +0x44..0x47，精确语义 U**）。

### 对账结论
- 卡5 `DecoratorInfo` 结构（decorator.rs）**与 E2 完全一致**：`rank u8@+0x44`、`u45 u8@+0x45`、`ty u8@+0x46`、`state u8@+0x47`。`+0x45` 在 src 有对应字段 **`u45`（raw 名）**，维持登记，**不强行定语义**（U 待装饰渲染/实体消费端读面）。
- E2 的"type@+0x48/+0x46 并存"与卡5 §7 裁决一致：+0x46 = DecoratorInfo 内 type；+0x48 = 包裹记录（op 事件）type，非 DecoratorInfo（72B 边界 +0x48）。
- E2 移除拷贝 0x48B = {8×u64@+0x00..0x40, state u8@+0x47}：第 8 个 u64@+0x40 覆盖 +0x40..0x47，与 rank/state 簇重叠，簇内 rank/u45/ty/state 逐字节命名即该 u64 的低字节分解——一致。
- 改动：仅注释更新（decorator.rs struct doc + `u45` 字段注释，登记 E2 出处）；无结构/字段改名。

### 编译/测试
- `cargo check -p country-core`（本模块范围通过；终验现场碰撞组在途 RaycastWorld error 已另登记）。decorator 族测试不受影响（结构未变）。

## 十一、库内可闭合缺口闭合（restorer-decorator，2026-10-16；三项）

### 1. on_wall_split 子侧公式（E2 定案替换 side_of 闭包）
- 新增 `side_from_state(b)`：E2 公式 `(b-2)<=2 ? b-1 : 0`（u8 回绕；b∈{0,1}→侧0、b=2→侧1、b=3→侧2、b=4→侧3、其余→侧0）。`on_wall_split` 移除 `side_of` 闭包参数，改为读 `DecoratorInfo::state`（+0x47）经公式求侧；侧 2/3 → `unreachable!("internal error: entered unreachable code")`（0x142ac8d80）。
- 测试：`side_from_state_formula`（全值域 b 0/1/2/3/4/5/0xff/0x84）、`on_wall_split_marks_removed_and_unregisters`（state 0→侧0、state 2→侧1 迁移 + removed 注册表）、`on_wall_split_panics_unreachable_for_side_23`（state 3→侧2 panic）。
- 留存 U：0x28B 记录完整语义（分裂操作设置 info.state 的时机/载荷）；侧 0 的 u 区间包含搜索细化未并入（get_approx_u_from_pos + {low,high} 对数组）。

### 2. u45 消费端读面
- 核对：库内**无** DecoratorInfo 的 rank/ty/state/u45 消费读面（唯一读方 ui_focus_decoclutter 仅搬运 info 到事件；storage 增删查）。render/实体消费端在**库外** `system_decorator` crate（PDB inline 宿主：decorator_visual::stairs_assemble/stairs、ui_systems::ui_focus_stairs）。
- 登记：u45 精确语义维持 U，待 system_decorator（render/实体域）读面；库内不强行定语义。

### 3. contains_shape 自圈分支（按章节定案）
- `systems/wall/valid_enclosure/mod.rs` `Shape::contains_shape` 的 `Self::Circle` 分支：`other.center()` 改为 **`other.get_pos_at_u(0.0)`**（章节 O：圆→0x140c975a0(payload,0)、矩形→0x140c99600(payload,0)，非 center() 访问器；把 other 视为 u=0 边界点做保守包含）。
- 测试：`circle_contains_shape_uses_other_u0_boundary_point`（圆心(4,0) r=1 的 u=0 边界点(5,0) 判不包含，而 center() 会判包含——演示纠偏）。
- 注：该文件属 wall 域（卡B valid_enclosure），经中央派本 restorer 定案；已按章节 O 修正。

### 测试/编译
- `cargo test -p country-core --lib`：**1712 过 / 1 败**（败项 geometry::sphere 既有非本模块；1712 = 基线 1709 + 本批 3 测试）。decorator 27、valid_enclosure 6 全过；`cargo check -p country-core` 通过。
