# WallAttachedDecoTracker：B 原语义与最小接入设计

状态：设计审查输入；未创建源码候选，未改 production，未执行 Cargo、游戏或 GPU。归属 claim `TG-bd3d82a9ddc435cfb2` / owner `deco-trackers-20261020`。本包仅恢复 tracker 协议；02 拥有完整 blueprint instantiate，06 拥有 DecoratorStorage/Decorators 注册和 cleanup，root/07 决定整合。

## 结论与覆盖

两资源应共用当前权威 `DecoratorStorageAddr`、`WallId`、`DecoratorId` 和 assembly 参数类型，恢复为“每墙装饰差异 map + dirty wall set”，而不是另建装饰地址注册表或复制上一帧全表。原 add/remove 是严格状态迁移；原 `with_diffs` 按 dirty 墙调用消费者、随后消费差异。当前 `previous: HashMap<u32,P>` / `clone_from(current)` 不满足原协议，当前 96B `DecoDiff` 和统一 40B Door 参数解释错误。

B：8 个 tracker 完整过程、3 个嵌入游戏 removal callback 的 retain 实例、两个资源注册过程、move-anchor host，全部对原 RNE + 对应 PDB 采集。fullvisual / Option Eq 原证只读复用 02 的完整 pack。C：只列 PDB 地址/长度导航，未读取其机器码、未验证等价，不宣称 C 恢复。

## 身份和权威字段

- B 原 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- 原 `App::insert_resource`：Door `0x140139080` / 1540B；Chimney `0x14013e110` / 1540B。原名称字符串、descriptor 96B/align8、对应 drop_ptr 同时对齐。见 `resource-identity-B/`、`resource-identity-PDB.json`、`native-B-extra-data.json`。
- 原 descriptor TypeId 两个 u64（按 descriptor 写入顺序）：Door `[0xb3367ec6976611c5, 0x8db22aa21282ec70]`；Chimney `[0x475e08016210c1c6, 0x2f388f0588fa8a62]`。这些只绑定 B 原身份；新 Rust 编译必须使用其实际泛型 `TypeId`，不得硬编码原值。它们不是 SystemTypeSet 的 TypeId。
- 两者是不同 `ResMut<WallAttachedDecoTracker<...>>`，不是两个 ECS marker。泛型实参原名分别属于 `chimney_assemble_bricks::ChimneyAssemblyParams` / `door_stairs_assemble_bricks::DoorStairsAssemblyParams`。
- 96B tracker 的业务字段推断：`+0x00` 为 `HashMap<WallId, inner log>`（48B），`+0x30` 为 `HashSet<WallId>`（48B）。外 map bucket stride56B（WallId8 + inner map48）；内 map bucket stride32B（DecoratorId4 + 对齐 + diff24）。该布局由 add/remove/with_diffs 的同一 offset 读写共同约束；不以 PDB 泛型名独自推字段。
- `DecoratorStorageAddr` / `WallId` / `DecoratorId` 归 country-core 现类型；tracker 仅保墙差异，不再维护 id→storage、live decorator、removed-id 或 modification-tracking 的第二份索引。wall 事件清理 tracker，不能清 DecoratorStorage live map。

## 参数承载与比较

native add 在验证 `storage.as_wall()` 之后克隆并 Box 分配：Chimney 40B/align4；Door 32B/align4。Box 分配发生在 map lookup 之前；同 id 非法重复 add 也先分配，随后 panic。`as_wall` 对 Terrain/Stair 等非 Wall 地址 panic，不能改为忽略。

建议当前暂保 opaque words，避免猜尚未证明的语义字段；只能有一个 canonical 类型。Chimney 为 `[u32;10]`，Door 为 `[u32;8]`，不凭零值猜 `Default`。参数类型可位于原 assembly 模块，再由 diff 模块重导出以兼容现调用；或者 root 为最小补丁保留当前定义位置、从原模块重导出。两种组织均须使用 re-export/type alias，禁止建立第二个结构体类型。最终字段位置由 root/02 协调，tracker 只拥有 diff 状态。

现 `derive(PartialEq, Eq)` 的 words 位比较不可用。02 原 Eq pack：

- `0x140824fe0` / 172B：Chimney `+8..+1c` 六 f32 逐项 ordered equality；`+0` u32 tag 相同；tag==1 时不读/比较 `+4`，其余相同 tag 比 `+4` f32；`+20` u32 equality；`+24` f32 ordered equality。
- `0x140825150` / 152B：Door `+0..+1c` 八 f32 ordered equality。

因此相同 NaN payload 仍不等，+0/-0 相等；Chimney tag1 的 `+4` 即便不同或 NaN 也不影响相等。opaque words 的 `PartialEq` 应以 `f32::from_bits` 执行上述比较，不能实现 `Eq`。比较是 fullvisual 决定 remove/add 的调用者合同，tracker add/remove 本身不比较 `P`，不应为 tracker 添加 `P: PartialEq` 约束。几何字段语义、枚举 variant 正式名称尚未全部恢复，不能据此伪称完整参数领域模型。

## DecoDiff 状态机（语义名为本设计命名）

native diff 是 24B 带 tag 值的枚举，payload 为一个或两个 Box 指针；不是 96B 带代际记录。`0x14010b940/ba60` 是 tracker Default（274B），其 TLS 与递增是 std RandomState 依赖；现 generation/header 常量解释须撤销。

| native tag | 本设计语义名 | first pointer | second pointer |
|---|---|---|---|
| 0 | Current | current | 无 |
| 1 | Added | current | 无 |
| 2 | Removed | old | 无 |
| 3 | Changed | new | old |
| 4 | Poison | 无有效 payload | 无 |

Poison 是 `mem::replace` 临时状态，普通消费者不能收到合法 Poison。Rust 新枚举的内存 tag 排布不是外部 ABI；不需要 unsafe、手工指针或强迫编译器 native 布局。

| 操作 | 原前态 | 原后态/错误 |
|---|---|---|
| add 新 wall/id | absent | Added(new)，dirty.insert(wall) |
| add 同 wall/id | Removed(old) | Changed(new,old)，dirty.insert(wall) |
| add 同 wall/id | Current / Added / Changed | panic（不静默覆盖）；新 Box 由正常 unwind 所有权处理 |
| remove | Current(cur) | Removed(cur)，dirty.insert(wall) |
| remove | Changed(new,old) | drop new；Removed(old)，dirty.insert(wall) |
| remove | wall/id absent、Added、Removed | panic（Added 后 remove 不是“抵消新增”） |
| add/remove | Poison | 原 trap/panic；不作为正常协议 |

原 remove 对 wall/id 缺失都 panic。`remove` 不是按 id 搜整个世界，也不接收新的 storage 取代旧 storage。02 fullvisual 对 changed current 的顺序已经证明：用 previous 的旧 storage/id remove，随后 current 的新 storage/id add。同墙变成 Changed；换墙保旧墙 Removed + 新墙 Added。不能把换墙简单覆盖同一 id。

原证入口：Door add `1412acd70/1992`，Chimney add `1412ad540/2008`；Door remove `1412add20/1170`，Chimney remove `1412ae1c0/1170`。相对跳表保留在 `native-B-switch-tables.json`，不能只读 Ghidra 截断函数体判断 tag3 分支。

## with_diffs / logs 的消费合同

`1412ae660/3377` 和 `1412af3a0/2035` 已包含两个真实 assembly consumer 的 inline 身体，不能把全部指令都当通用 tracker 成本。

1. drain dirty wall set，保留原 allocation 和 hasher；只处理本批 dirty wall。原 native 临时摘下 set table，在返回时恢复同 table、len0。建议直接复用 std `HashSet::drain`，不要 `mem::take + new HashSet` 或先收 Vec。
2. 每 wall 查 outer map；没有 inner log 也可进入 wall consumer（单纯墙几何变化会标 dirty）。回调拿到可空/空 log，不能用“log 缺失则不更新墙”过滤。
3. `with_each_deco_diff` 访问该墙内 map 的记录；`current()` 对 Current/Added/Changed 给 current，Removed 为 None。Door 重建该墙 floor 时枚举所有 current，包含原来未变化的 Current，不能只回调新/变 id。Chimney同样用 current 参数组装，并读状态区分新增/变化的音效边界。
4. 消费后执行 `with_removed` 的 retain 回调：Current/Added→Current；Removed 调用 `(id, old params)` removal callback 后 drop old + erase entry；Changed→Current(new) 并 drop old，**不调用** removal callback；Poison panic。因此 Changed 的旧参数不能冒充一个真正删除事件。
5. Door removal callback 是空业务动作；Chimney 在真实 removed 时有原声音/装饰信息读取业务 callback。若 PublicWalls 找不到墙，正常 fallback 仍执行空 removal callback 的状态归一化。不得因 wall miss 留住 Removed 或 Added 状态；也不得因“清理”虚构本来没有的声音事件。
6. 普通 drain / map bucket 顺序；未发现稳定 id 排序合同，不添加排序或每帧全世界扫描。std HashMap/HashSet 的公共实现直接复用，不还原其 SIMD probe 算法。

retain 原游戏闭包完整 host：Door `141268770/553`；Chimney 有业务 callback `1412681c0/860`；Chimney fallback 空 callback `1412689a0/553`。这三者仅用于证明嵌入的本地游戏状态分支；不把 hashbrown::retain 认作待逆向游戏算法。消费跳表见 `native-B-extra-data.json`。`with_removed` 正式 Rust 参数传值/借用拼写、两个 log wrapper 的原字段名和精确高层签名 PDB 未给出；上述正常执行语义已由代码确定，不伪称原源码签名。

## 最小安全 Rust API 提案（待 root/07 授权）

调用者现在确定需要：

```rust
tracker.add(storage: DecoratorStorageAddr, id: DecoratorId, params: &P);
tracker.remove(storage: DecoratorStorageAddr, id: DecoratorId);
tracker.on_wall_changed(wall: WallId);
tracker.on_wall_removed(wall: WallId);
```

`P: Clone` 即可供 add clone；当前两个 opaque 参数可 Copy/Clone，但不需要比较/Default trait。资源使用实际 `WallAttachedDecoTracker<CanonicalP>` 泛型类型；由正常 Bevy Resource derive/impl 给出 Send+Sync 约束，不用 unsafe Sync。

内部只需 `wall_logs: HashMap<WallId, HashMap<DecoratorId, DecoDiff<P>>>` 与 `dirty_walls: HashSet<WallId>`。字段名是恢复实现选择，不宣称 PDB 原名。公开字段和 `previous` 全表 seam 应收回为方法，防止调用者绕过迁移。`on_wall_changed` 仅 insert dirty；`on_wall_removed` 先 remove/drop inner log 再移除 dirty，不因缺失 panic，也不发逐装饰 Removed callback。前者在 move-anchor host `14081cc8d/cc9d` 两个 insert；后者原 fullvisual tail `140828f5c..14082921a`。

建议一个 scoped wall-diff adapter 对 callback 提供只读 `for_each_diff`/`current` 与一次 `with_removed` 消费；callback 返回后若未显式消费，自动用空业务 callback 归一化。用局部借用和消费标志保证恰一次 retain，空 log 不分配。该 adapter 是安全 Rust 的语义适配提案，不宣称已恢复原 wrapper 的精确类型拼写。不得同时暴露 map 的共享/可变别名，不需 UnsafeCell，不延长借用 lifetime。正常路径与原分支保持一致；panic 后继续复用 tracker 的恢复合同没有证明，不提供 rollback 或强异常安全承诺。

02 仅依赖 add/remove/on_wall_removed 四个真实副作用，不应为接入再构造 current map。原现 chimney/door 装配 seam 的 `current: HashMap` 参数不能保留为假 consumer；其真实 assembly 主体尚未在本包恢复，source 候选必须标明相应未恢复范围，并与 root 决定最窄编译适配。

## 成本与验证边界

原 add 每次分配一个40/32B Box，外/内 map 和 dirty set 仅按容量需要增长；remove 的正常迁移不复制 payload，Changed remove 释放 new；消费每 dirty wall 扫其 log，retain 也是一遍该墙容量范围，drain/iter 的公共容器成本受容量影响。正常帧不 clone 全 world map、不排序、不新增第2个 registry。空消费不要求堆临时 Vec。以上是源/指令复杂度和 allocation 位置分析，没有运行时间/吞吐量测量，不能声称快几倍或性能达标。

root 可统一执行的有意义合同检查：同墙 remove→add→消费、跨墙迁移、Changed→remove 保留最旧 old、合法 Current 再消费、缺墙 dirty 仍回调、PublicWalls miss 仍归一化、wall remove 同时清 map+dirty、重复 add/remove 与非Wall panic、40/32参数 byte clone、NaN/+0/-0 和 tag1 忽略+4。比较差异应放 fullvisual 调用者，不把参数比较塞进 tracker 热路径。检查 drop 次数与 removal callback 计数分别验证 Changed 和 Removed。测试集合可用确定性小 fixture，不要求 benchmark 或 GPU。

异常路径按原 panic 类保留；精确 panic 文案、panic 发生后日志中 Poison 及 dirty drain 的全部 unwind 细节未完整证明，禁止宣称事务回滚。C 全部算法仍待该版本机器码验证。native collector 的 elapsed_seconds 是证据采集耗时，不是游戏性能数据。

## 原证复用路径

02 原证相对 games_test：

- `reverse-engineering/tiny-glade/skeleton/task-index/work/TG-b3ec56669fe9d97baf/scene-clutter-producer-20261020/evidence/instantiate_blueprints-bounded.asm`（B 140827910，7991B）；
- `reverse-engineering/tiny-glade/skeleton/task-index/work/TG-69cc2f973d7c14cb28/scene-clutter-producer-20261020/evidence/{insert-B-full,clear_blueprints-B-full,copy_blueprints_to_prev-B-full}.asm`；
- 同一 work 下 `fullvisual-v1/evidence/{chimney_option_eq-B-full,doorstairs_option_eq-B-full}.asm` 与 `CORE_EQ_NATIVE_RANGES.json`。

完整 SHA256 清单见 `MANIFEST.json`。旧 Ghidra 两 remove 的 body size738 而 PDB1170，属于分段 CFG 截断，仅作为导航诊断，结论由原 RNE 完整字节 pack 支持。最初 removal collector cwd 错误的失败报告保留为诊断，不混入成功覆盖计数。
