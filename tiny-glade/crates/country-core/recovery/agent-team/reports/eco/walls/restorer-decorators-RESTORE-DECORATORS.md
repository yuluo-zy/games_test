# RESTORE-DECORATORS — walls `restore_decorators`（0x14155ba30）重建

> 日期：2026-10-17。角色：restorer（walls 域）。
> 协议：`planning/2026-10-16-logic-verify-protocol.md`（只以 fresh 反编译为准）。
> 任务：重建 `DecoratorStorage::restore_decorators<closure_env_0>`（Ghidra 函数界
> 4438B；PDB `code_size = 0x1907 = 6407B`，调用 0x1417bdae0 位于函数内 0x1714，
> 故以 PDB 边界为准）。本轮 fresh 证据实时生成：
> - `planning/impl-walls-restore-decorators/fresh-result.json`（ghidra shared，
>   `get_functions(0x14155ba30, fields=decompiled_code,callees)`）
> - `.../disasm-result.json` + `decomp/14155ba30-disasm.txt`（`disassemble_function`，1243 条）
> - `.../xref-result.json`（调用方 `replay_pop_bucket` 0x140b38250，调用点 0x140b38385）
> - `.../caller-disasm-result.json`（调用点实参恢复）
> - `.../14155ba30-fresh.txt`、`.../140b38250-replay_pop_bucket.txt`
> - 辅助：`.../re-result.json`（remove_entry ×3）、`.../ins-result.json`
>   （insert ×3）、`.../rc-result.json`（remove_collider）、`.../helper-result.json`
>   （get_mut / Decorators::insert）、`.../mem-result.json`（跳表体）、
>   `.../jt-result.json`（0x142c6e844）、`.../str-result.json`（panic 串）。

## 改动文件（仅 walls 域）

- `crates/country-core/src/resources/walls/decorator_storage.rs`
  - 新增 `DecoratorColliderRemover` 端口 + `DecoratorColliderMaps`
    （`cfg(feature="native-physics")`，直接调用 collision 已落地的
    `systems::collision::world::remove_collider`）。
  - `restore_decorators<C: DecoratorColliderRemover>` 全量重建（原为占位
    passthrough）；拆出 `restore_decorators_state_diff`（原生步骤 2/3，返回
    `HashMap<DecoratorId, DecoratorBeforeAfterWithStorage>`）。
  - 新增 8 个单测。
- `crates/country-core/src/resources/walls/decorator.rs`
  - `DecoratorIdGen::reserve` 纠正为原生语义 `next = max(next, id+1)`
    （原无参返回当前游标，无调用方）。

## 函数签名复原（PDB 占位 + 调用点逆推）

`replay_pop_bucket` 调用点 0x140b38385 实参：

| 位置 | 原生 | 语义 |
|---|---|---|
| RCX | 0x140b38337 `*RSI` | `&mut DecoratorStorage` |
| RDX | `RDI + 0x10`（RDI=装饰 diff Arc 载荷） | `&HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>` |
| R8 | `MOVZX byte [RBP-0x8]` | `reverse: bool`（侧选择） |
| R9 | `[[RBP+0x80]]` | `&mut DecoratorIdGen` |
| 栈参 1 | `*R14, R14=[RBP+0x90]` | `&mut RaycastWorld` |
| 栈参 2 | `[[RBP+0x98]]` | 相邻两张碰撞 map（+0x00 collision_a / +0x30 collision_b） |
| 栈参 3 | `[RBP+0xa0]` | `send_state_diff_events` 闭包环境（`[env]`=DeferredDecoratorOpEvents） |

输入 map 值经机器码定尺：键 `DecoratorStorageAddr{+0 disc,+8 payload}`（0x10B），
值 0x60B = 两个 `HashMap<DecoratorId, DecoratorHistoryState>`（各 0x30B，侧偏移
+0x00/+0x30；内层条目 stride 0x58 = key 4 + pad 4 + state 0x50）。`reverse`
选择哪个侧作为移除侧（false=before 移除 / after 新增；true 对调）。

## 逐分支判等

| 步骤 | fresh 结构 | Rust | verdict |
|---|---|---|---|
| 1 入口断言 | 0x14155ba2b `CMP byte [RCX+0x1b0],0` → `panic("assertion failed: !self.immutable")`（0x142c6e2b8） | `assert!(!self.immutable, ...)` | 判等 |
| 2 移除侧 | 外层按 addr 迭代；`get_mut(addr)`（0x140a1cde0；RAX=Option，RDX=self+0x60=d2s）None → 下一 addr。逐 id：`modified_ids.remove`（remove_entry on bucket+0x60，0x140b776d0）+ `removed_ids.insert`（insert<tuple<>> on bucket+0x00，0x1402c9200）+ `decorators.remove`（remove_entry<(id,DecoratorInfo)>，0x140b77b20；None 由 niche `state 字节=2` 编码，0x140b77c0d）。Some(info) → `d2s.remove`（remove_entry<(id,DecoratorStorageAddr)>，0x140b78140；判别 3=miss → panic "assertion failed: self.map.remove(&id).is_some()"，0x142ac9068）→ `DecoratorHistoryState::from`（0x140b3f930）→ 输出 `before={addr,state}`、`after.disc=3`（0x14155c27d） | 逐句对应 | 判等 |
| 3 新增侧 | 第二遍外层；新增侧 map `items==0` → 跳过（0x14155c362）。`get_or_insert_default(addr)`（0x140a1cf30）。逐 id：`DecoratorIdGen::reserve(id)`（`next=max(next,id+1)`，0x14155c4e1）→ 输出 map 探针（0x14155c6d6）：命中覆盖 after，未命中插入 `{before.disc=3, after.disc=3}` 后写 after（0x14155c808/0x14155c814/0x14155c82b）→ 按 `state.ty` 跳表 0x142c6e844 重建 DecoratorInfo → `removed_ids.remove`（bucket+0x00）+ `modified_ids.insert`（bucket+0x60）+ 桶 map insert（0x1411fbfa0）→ 旧条目 None：`d2s.insert`（0x14152bc60，冲突 panic "d2s insert: already exists"）；Some：`d2s.update`（探针 overwrite；miss panic "d2s update: no such ..."，0x142ac9018/0x142ac9030） | 逐句对应；info 经 history `to_info` + `state=0` | 判等（info 重建见分歧 D4） |
| 4 collision | 遍历输出 map（stride 0xc8）。`before.disc==3` 跳过；`after.disc==3` 或两侧地址不等（disc 不同，或 disc==0 且 payload 不同）→ 两次 `remove_collider<DecoratorId>`：map1=[0x438]+0x00、map2=[0x438]+0x30，world=[0x430]；每次命中（out 标志 !=0x11）递减句柄内 `Arc<RaycastMeshData>`（`LOCK; dec; ==0 → drop_slow`） | 条件逐条对应；两次调用经端口聚合；Arc 递减由 collision `remove_collider` 内部 drop 实例完成 | 判等（表面差异 D1） |
| 5 事件 | 输出 map 拷到栈 (0x190→0x1c0) → `send_state_diff_events`（0x1417bdae0），RCX=&map、RDX=[0x440]=闭包环境 | `send_state_diff_events(&out, events)` 一次 | 判等 |

**verdict：已重建（判等）；4 项登记差异（D1-D4，见下，不冒顶）。**

## 与已就绪 storage diff 的接入关系

- 步骤 3 的状态→`DecoratorInfo` 回填复用
  `resources/history/decorator_edit.rs::DecoratorHistoryState::to_info()`
  （`apply_storage_diff` / `restore_decorators_diff` 同一回填原语）。
- **未逐字调用 `apply_storage_diff` / `restore_decorators_diff`**：fresh 显示
  restore 是两遍、逐 id 交错的流程（移除侧含 `modified_ids` 维护、d2s
  remove/insert/update 三种带 panic 语义；新增侧与事件 map 同步构建）。
  现有 `apply_storage_diff` 为单遍简化（不维护 `modified_ids`、无 d2s
  insert/update 区分），直接调用会偏离原生控制流；故按 fresh 实现，二者保留为
  简化批处理 API。判等依据以 fresh 为准（协议红线）。

## 登记差异（不冒顶）

- **D1 `Arc<RaycastMeshData>` drop 表面差异**：原生 `remove_collider` 把移除实例
  的网格 Arc 放入 64B 句柄（out+0x38）返回，调用方在命中后
  `dec; ==0 → drop_slow`；Rust `remove_collider` 在内部 drop
  `ColliderMeshInstance`（含该 Arc）后只返回 `SubworldColliderHandle`。**递减次数
  与时机等价**（每次命中一次），差异仅在 drop 发生位置/返回值面。
  解锁条件（若需字节级句柄语义）：collision 域提供“返回被移除实例/Arc”的变体。
  另：collision 既有登记（registry free-list / generation bump 未建模）不变。
- **D2 两张碰撞 map 的宿主类型**：原生第 6 参指向相邻
  `collision_a@+0x00 / collision_b@+0x30`（`HashMap<DecoratorId,
  (OffsetColliderTransform,RaycastMesh)>`，见 `drop_inner_table` 符号）。当前
  `DecoratorArchivist` 的 `collision_a/b` 是 `HashMap<u32, C>`（recovered_building
  域近似）。walls 侧以 `DecoratorColliderMaps` 接受 collision 具体类型
  `HashMap<DecoratorId, ColliderMeshInstance>` 两张 map。
  解锁条件：replay/archivist 接线时提供两张真实碰撞 map（或 adapter）。
- **D3 输入状态类型多出的 `removed` 集**：原生单侧仅
  `HashMap<DecoratorId, DecoratorHistoryState>`（0x30B）；Rust
  `DecoratorStorageAddrHistoryState` 另带 `removed: HashSet<DecoratorId>`（历史域
  早期近似）。restore 只消费 `states`。解锁条件：history 域按 0x30B 对齐该类型
  后可删字段（本任务不改 history）。
- **D4 `to_info` 重建分派差异**：原生按 **DecoratorType**（`state.ty`）跳表
  0x142c6e844（8 案：0x38/0x39/0x3a→style6/u30=u34；0x3b→style4/u4d 入 u30
  高字节；0x3c→style7/u30=u38；0x3d→style5/u30 低字节=u4c；0x3e→style3；
  0x3f→default 簇 u30=(u3c<<32)|u30、u38/u3c=u40/u44、u4b/style 直取）；原生对
  非本类型字段写的是**寄存器残余值**（如窗口案 `u40` byte0=0x44、u38/u3c 取前序
  scratch），属 don't-care。history `to_info` 按 style 簇重建并写规范化值；两侧
  在类型专属字段上一致。**walls 侧按 fresh 0x14155c968 覆写
  `info.state = 0`**（history `to_info` 固定写 2）。解锁条件：history 域若需
  逐字节对齐，按 ty 跳表重写 `to_info`。
- 其它小项：Terrain/Stair 的 `storage_payload` 原生原样拷贝键第二 qword，Rust 写
  0（仅在 disc==0 参与比较，语义不影响）；输出条目缺侧 payload/state 原生未初始化，
  Rust 置 0。

## 测试与编译

新增单测（`decorator_storage.rs::tests`，8 个）：
`restore_decorators_removes_d2s_bucket_and_marks_removed`（d2s 移除 +
removed_ids/modified_ids + 移除事件 kind 4 + 碰撞移除一次）、
`restore_decorators_adds_bucket_and_inserts_d2s`（reserve + 桶回填 + d2s insert +
新增事件 kind 3 + state 字节 0）、
`restore_decorators_assembles_before_after_with_storage`（两侧 storage_disc/
payload/state 组装）、
`restore_decorators_removes_collider_when_address_changes`（地址变 → 碰撞移除；
变更事件）、
`restore_decorators_reverse_swaps_sides`（reverse=true 侧对调）、
`restore_decorators_panics_when_d2s_entry_missing`、
`restore_decorators_asserts_not_immutable`，另 `decorator_id_gen_reserve_semantics`。

结果（本轮实测，最终复跑）：
- `cargo check -p country-core --lib`：**Finished**（0 error，仅既有警告）。
- `cargo check -p country-core --lib --features native-physics`：**Finished**。
- `cargo test -p country-core --lib resources::walls::decorator_storage`：
  **20 passed / 0 failed**（13 既有 + 7 新增）。
- `cargo test -p country-core --lib resources::walls::decorator`：**37 passed**。
- `cargo test -p country-core --lib wall`：**142 passed / 0 failed**。
- 备注（并发归属）：19:46-19:48 他组正在编辑
  `systems/backend/input/input_replay_for_trailers.rs`、
  `systems/backend/input/input_record_replay.rs`、
  `systems/wall/shapes/circle/ui_create_circle.rs`，期间 lib/测试目标出现这些文件的
  瞬时编译错误（E0596/E0308/E0061）；walls 本函数所在文件无错误，对方落盘后
  上述最终结果复跑通过。

## 自检

- 未改 history/collision 业务文件；仅调用其已落地接口。
- 缺字段/字节级差异全部登记（D1-D4），未以近似体冒充判等。
- 宏胶水未重建。
