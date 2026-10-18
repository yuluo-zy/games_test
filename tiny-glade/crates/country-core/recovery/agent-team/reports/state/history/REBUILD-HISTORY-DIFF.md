# REBUILD-HISTORY-DIFF — DecoratorStorageHistoryDiff/AddrHistoryState 类型锚定 + archivist 链 5（2026-10-17）

> restorer: impl-history-difftypes，2026-10-17。依据 2026-10-16-logic-verify-protocol.md。
> fresh 反编译（ghidra shared，batch1 8 函数）落盘
> `recovery/agent-team/impl-history-difftypes/decomp/*.txt` + `results/batch1-fresh.json`。
> 目的：解锁 walls archivist 迭代链 5（REVIEW-PENDING-WALLS 跨域登记段）。
> 红线遵守：只以 fresh 反编译为准；缺类型不编造；history/walls 两域此链可改，不越其他模块。

---

## 1. 类型锚定（history 域，落 `src/resources/history/decorator_edit.rs`）

### 1.1 `DecoratorHistoryState` 重载为原生 ~0x4fB 布局（0x140b3f930 `impl$1::from`）

原 Rust `{id: u64, style, color: ColorId, lantern_brightness, stair_width}` 5 字段
平面近似（LOGIC §1.2 判 `结构不对/载荷假`）**已替换**。fresh decomp-0x140b3f930 逐写
确认原生布局：**DecoratorInfo 前 0x48B 直拷 + style 派生簇 + style 字节 @+0x4e**：

| 偏移 | 字段 | 来源 |
|---|---|---|
| +0x00/+0x08/+0x10/+0x18 | dst_wall/dst_secondary/dst_payload/u18 | `in_RDX[0..3]` 直拷 |
| +0x20/+0x24 | u20/u24 | `in_RDX[4]`（u64 = u20:u24） |
| +0x28 | u28:u64 | `in_RDX[5]`（= `(decorator_id<<32)\|u28`） |
| +0x30 | u30 | `bVar12==2\|6`（style==4 或 8+/<2）→ `u30` low，否则 0 |
| +0x34 | u34 | style==6（Window）→ `u30` low，否则 0 |
| +0x38 | u38 | style==7（Lantern f32）→ `u30` low，否则 1.0 |
| +0x3c | u3c | style 8+ 或 <2 → `u30` high，否则 1.0 |
| +0x40 | u40 | style 8+ 或 <2 → `u38`，否则 0 |
| +0x44 | u44 | style 8+ 或 <2 → `u3c`，否则 0 |
| +0x48/+0x49/+0x4a | rank/u45/ty | `in_RDX+0x44/0x45/0x46` 直拷 |
| +0x4b/+0x4c/+0x4d | u4b/u4c/u4d | 8+/<2→`u40`byte0；style 5→`u30`byte0；style 4→`u30`byte4 否则 1 |
| +0x4e | style | `DecoratorInfo.extra_style()`（@+0x41） |

**`from(&DecoratorInfo)` 已实现**：入口 `info.state != 0` → panic
"Attempting to capture a DecoratorHistoryState of a preview"；style 分变体派生逐句对
机器码（含 `(byte)(style-8)<0xfa` 默认簇分支、Lantern/Window/Flag/Chimney 载荷）。

**`to_info()` 逆映射已实现**（restore_decorators 回填面）：style 派生簇收敛回
`DecoratorInfo.u30/u38/u3c/u40` + `decorator_id` 分解；**state 字节写 2=live 为
登记 U**（原生 restore 逐句未还原，fresh 0x14155afe0 insert 写 2）；**style 2..7
的 +0x4e 为 0（from 仅 read_branch 写 style），restore 侧由派生簇推断 style
（唯一非默认者）——登记 U**（原生 restore 经桶内 DecoratorInfo `u40` byte1 直读）。

**挂账**：state 内部 serde 原生为 style 分变体选字段写
（`convert_before_after_states_to_changes` 0x140b3fab0 / `send_state_diff_events`
0x1417bdae0 的 `__SerializeWith`）——Rust 侧暂全字段 derive，登记不编造。

### 1.2 `DecoratorStorageAddrHistoryState` / `DecoratorStorageHistoryDiff`（新类型）

`DecoratorArchivist::extract_diff`（0x140b2cb30）callee 型名权威：
- 中间 map `HashMap<Addr, (DecoratorStorageAddrHistoryState, DecoratorStorageAddrHistoryState)>`
  （`from_iter` 签名）。
- 输出 `HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>`（filter_map closure 收敛）。
- `DecoratorStorageAddrHistoryState` 构建 = `extract_before_after_impl`（0x140b2be70）
  的 `from_iter<DecoratorId, DecoratorHistoryState, Map<Iter<HashMap<DecoratorId,DecoratorInfo>>, impl$1::from>>`
  → `{states: HashMap<DecoratorId, DecoratorHistoryState>, removed: HashSet<DecoratorId>}`
  （removed 面为恢复语义所需，字段精确位置以类型面登记）。
- `DecoratorStorageHistoryDiff = {before, after}`。

### 1.3 `DecoratorInfo::extra_style()`（walls 域 `decorator.rs`）

+0x41 style/判别字节（`u40` byte1；`mov r?,[rdx+0x41]`）访问器，供 from/convert 使用。

---

## 2. archivist 链 5（walls 域，`src/resources/walls/decorator_archivist.rs`）

| 函数 | 处置 | 说明 |
|---|---|---|
| `extract_diff`（0x140b2cb30） | **实现** | 清 collision_a/b → 取 walls + special1(before@+0xc0)/special2(after@+0x150) → 逐墙 `extract_before_after_pair` → `HashMap<Addr, DecoratorStorageHistoryDiff>`；后置断言 walls/special1/special2 空。**before/after 配对语义 U**：原生 `extract_before_after_all` 闭包内联未单解，Rust 侧按"before/after 桶逐 id 覆盖当前桶"重建，非判等（注释登记） |
| `extract_before_after`（0x140b2be70） | **实现** | 单地址桶 → `DecoratorStorageAddrHistoryState`（`tracked_to_addr_state`） |
| `archive_storage_decorators`（0x1412b2a70/0x1412b3910） | **实现 in-domain 核** | 0=墙（已存在 return + clone 入 walls）/1=special1/2=special2（已存在 return + 克隆）。**collision collider/mesh 重建为 collision 域（RaycastMesh/OffsetColliderTransform），登记**。`&str/String` 标签参为日志面，登记 |
| `archive_current_decorators_in_storage`（0x14123b5c0） | **实现** | 逐地址（per_wall/terrain/stairs）归档入 archivist；special1/2 分别承载 Terrain/Stair |
| `archive_current_decorators_in_storages`（0x1412b4f10） | **实现** | 多存储批量归档 |

### 2.1 桥接

- `ObservedDecoratorInfo`（recovered 归档侧 0x48B 字节）↔ `DecoratorInfo`（repr C）：
  `observed_to_info`/`info_to_observed` 逐字节拷贝（`size_of` 断言）。
- `decorators_storage::Decorators` → recovered `TrackedDecorators`：`decos_to_tracked`。

### 2.2 `restore_decorators`（`decorator_storage.rs`）

占位 passthrough `restore(self)` 保留为原生闭包分派形（0x14155ba30 原生即闭包形），
**新增真实 diff 应用路径**：
- `apply_storage_diff(addr, &DecoratorStorageHistoryDiff)`：after 有 before 无 → 桶
  移除 + d2s 移除 + removed_ids；before 有 → 桶 insert(`to_info`) + removed_ids 移除
  + d2s upsert。
- `restore_decorators_diff(&HashMap<Addr, DecoratorStorageHistoryDiff>)`：批量。
- collision 重建仍为 collision 域，登记。

---

## 3. 编译与测试

- `cargo check -p country-core --lib`：**0 错误**（当前树 lib 全绿）。
- 新测试全 PASS：
  - `decorator_edit::tests` 4/4（from 逐 style 派生、preview panic、lantern 默认 1.0）。
  - `decorator_archivist::tests` 2/2（extract_diff drain+diff、archive_current 归档+已存在 return）。
  - `decorator_storage::tests` 12/12（未回归）。
- **tests 构建**：全树 lib test 目标当前被**并发 agent WIP** 阻塞（
  `systems/collision/intersect_meshes.rs` 13:23 被 collision 域 agent 改动中：
  `VecDeque::reverse` 不存在 + 类型不匹配；另有 audio/save/snapping/stairs 未提交
  WIP）——**均非本模块文件**，为记忆记录的"并发 WIP 瞬时报错"已知态。本模块
  filter 测试单独构建运行通过。

---

## 4. 汇总

- **类型落地（history 域）**：`DecoratorHistoryState` 原生 0x4fB 布局重载 + `from`
  + `to_info`；`DecoratorStorageAddrHistoryState`/`DecoratorStorageHistoryDiff` 新落；
  `DecoratorInfo::extra_style()` 访问器。
- **archivist 链实现（walls 域）**：extract_diff / extract_before_after /
  archive_storage_decorators / archive_current_decorators_in_storage / _in_storages；
  restore_decorators 增加 apply_storage_diff/restore_decorators_diff 真实 diff 应用。
- **登记（不编造）**：extract_before_after_all 配对语义 U（内联闭包）；
  state 内部 serde 分变体选字段（convert/send_state_diff_events）；to_info state 字节；
  collision collider/mesh 重建（RaycastMesh/OffsetColliderTransform）与
  archive_storage_decorators `&str/String` 标签参为 collision/日志域。
- 新文件/改动：`resources/history/decorator_edit.rs`（重载+新类型）、
  `resources/walls/decorator_archivist.rs`（链 5）、`resources/walls/decorator_storage.rs`
  （apply_storage_diff/restore_decorators_diff）、`resources/walls/decorator.rs`
  （extra_style）。

**解锁状态**：walls archivist 链 5 的 history 域类型缺口已闭合；`extract_diff` 主逻辑
已实现（before/after 配对 U 标注）。剩余接续：collision 域 `RaycastMesh`/
`OffsetColliderTransform` 类型 + eraser finish/create_rectangle ridge 公式
（REVIEW-PENDING-WALLS §4 登记）仍为 collision/walls 后续回派项。
