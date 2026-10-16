# REVIEW — decorator 族（卡5）终验（reviewer-eco，2026-10-16）

复核对象：restorer-decorator 交付（RESTORE-DECORATOR.md + 三文件新建 decorator.rs /
decorator_storage.rs / decorator_subtype.rs）。证据：RE-DECORATOR-STORAGE.md + 章节
decorator_storage.rs.md + apply_decorator_place decomp 0x140a20c70 + 章节
clear_modification_tracking 0x141257900。
**注：当前全 crate check 被 audio_resources.rs:183（audio 组在途）阻塞，测试目标被
meta_commander.rs:314（state 组）阻塞——均非 decorator 文件；decorator 族自身无编译错误，
全库级复验待他组收口后为准。restorer 自报 decorator 17 测试通过。**

## 逐项核对（协调者 7+1 点）

### 1. DecoratorInfo 72B 具名 repr(C) 结构 — ⚠️ PARTIAL（读面 +0x48..0x56 未覆盖）

- 已知字段对齐 ✓：`dst_wall/+0x00`、`dst_secondary/+0x08`、`dst_payload/+0x10`（u64×3）、
  `dst tag@+0x22`（u20 高字节）、`decorator_id/+0x2c`、`rank@+0x44`、`DecoratorType@+0x46`
  （rank_ty 打包 u32）——与 RE 读面对齐。未知字段 raw 名（u18/u20/u24/u28/u30/u38/u3c/u40），
  不编造业务语义 ✓。
- **缺陷/对账项**：apply_decorator_place decomp 0x140a20c70 读面还读
  `+0x48`（param_2[9]=uStack_88）、`+0x4c`（uVar1=插入 id）、`+0x50`（uStack_a0）、
  `+0x55/+0x56`（uStack_82/84 两 u8）——共 5 个偏移**未被 13 字段 72B 结构覆盖**（结构止于
  +0x47）。需对账：这些偏移属 DecoratorInfo（则应扩结构至 ≥0x57）还是包裹记录/映射条目
  （则 72B 成立）。`+0x4c` 还被当插入 id 传入 `FUN_14155afe0`，与 `+0x2c` DecoratorId 并存，
  更指向记录大于 72B 或读面基准非 info 起点。**登记为 PARTIAL，建议 fresh decomp 逐偏移定字段。**

### 2. d2s 24B 判别/payload 语义 — ✅ 语义包装对，roof 第 4 目的地 U 诚实

- `DecoratorToStorageMap = Arc<HashMap<u32, DecoratorStorageAddr>>`（语义包装，非字节 24B；
  注释声明 std 容器≠原 hashbrown，非 ABI 契约）。
- `DecoratorStorageAddr` disc 0/1/2 = Wall/Terrain/Stair，payload=WallId；insert/remove 带原
  panic 串（"d2s insert: already exists" / "d2s remove: no such"）✓。
- roof 第 4 目的地 U（RE §6.2 / 章节遗留 U）——登记，未伪造。✓

### 3. on_wall_split 迁移语义 — ⚠️ PARTIAL（side_of 近似 + removed 路径缺失）

- 结构对应 decomp（46KB 主链：immutable 门 → 原墙移除 → 子墙默认构造 → 装饰迁移 → d2s 移除
  panic 0x142ac9300 → 子墙插入 + d2s 更新）✓。
- 原 0x28B 装饰记录（u 区间/子侧状态字节）完整语义 U（RE §6.4）——以调用方 `side_of` 闭包
  判定子侧，偏离已登记 ✓。
- **注**：注释称"子墙未中 → 默认构造 + 标 removed"，但代码 `side = side_of(info)` 恒把装饰
  归到 child_a(side==0) 或 child_b(side!=0)，**无"两侧均不中→标 removed"路径**。若原生确有
  removed 标记分支，此处缺失——登记为 PARTIAL（待 0x28B 记录语义定案后对齐）。

### 4. clear_modification_tracking — ✅ 全重置（对章节 O）

章节 0x141257900（O）：`modified 集 len!=0 → 释放 → len=0`；**清 decorators map@+0x30**
（RawTable::clear）。restorer `clear_modification_tracking` = `modified_ids.clear() +
decorators.clear()`——两者均清，**全重置** ✓。存储级（0x140a1e720，章节 I）逐桶调用 ✓。

### 5. 快照 XOR 掩码 — ✅ 待取证诚实，公式对

`SNAPSHOT_ID_MASK=0`（_UNK_1429cd080/090 数值待取证）；`encode = rotate_right32(id) XOR
mask`（章节 `(uVar18<<0x20|uVar18>>0x20)^mask` 等价）✓。take_snapshot 前置 6 断言按 decomp ✓。
快照逐记录状态字节编码 U（登记 I）。✓

### 6. 实体机制边界 — ✅ 纯数据，无 bevy Entity

DecoratorStorage 全为 DecoratorId/DecoratorInfo 数据记录，无 Entity 字段；实体由消费系统
读 DecoratorInfo 后另行生成，关联靠 DecoratorId 组件（RE §2 O 结论）✓。实体生成系统 VA =
render 域待定位（RE §6.3）✓。

### 7. DecoratorExtraInfo 判别偏移偏离（+0x11 → dst_payload 高位字节）— ✅ 可接受

- 现状：`extra_disc()` 读 dst_payload 字节 7；`set_extra()` 写字节 7。f32/u32 变体（4/5/6/7）
  载荷只占字节 0-3/0，字节 7 空闲 → **载荷无损往返**（原字节 1 判别会破坏 f32 字节 1，本实现
  严格更优）。Stair 20B 载荷跨 dst_payload/u18/u20，字节 7 与判别重叠——与原字节 1 重叠属
  **同一类内嵌判别联合的固有字节损耗，偏离仅移位**（注释已登记）。
- **裁决**：可接受。①纯数据（无 ABI/serde/GPU 原始字节消费当前）；②访问器（extra/set_extra/
  extra_disc）全 crate 自洽；③行为等价（判别语义一致，f32/u32 载荷反而无损）。
- **约束**：任何读 DecoratorInfo +0x10..+0x17 原始字节的路径（存档/快照/GPU 打包）必须经
  访问器；Stair 载荷字节 7 若为 4..7 会误读变体（与原字节 1 同风险类）——登记。

### 8. 消费端接线 — ✅ 登记

`ui_focus_decoclutter`（0x1409a92c0）→ get + d2s 索引 → 内嵌 map → DecoratorInfo 消费路径、
`archive_current_decorators_in_storage`（0x14123b5c0）归档、on_walls_merged/split 与
publish_public_wall_state 事件接线——均登记为 ui_clutter 组 / wall 发布链库外接线（RE §四
8/9）✓。

## 结论与 DISPATCH 建议

- **decorator 族（卡5）= fix_pending（低）**：
  - ✅ 已核：实体机制（纯数据无 Entity）、d2s 语义包装与 panic、clear_modification_tracking
    全重置（对章节 O）、快照掩码公式+待取证诚实、判别偏移偏离**可接受**、消费端接线登记。
  - ⚠️ 待补：① **DecoratorInfo 字段表未覆盖 apply_decorator_place 读面 +0x48/+0x4c/+0x50/
    +0x55/+0x56**——需 fresh decomp 逐偏移定字段（扩结构 or 证属包裹记录），这是协调者点 1 的
    核心；② on_wall_split 两侧均不中→removed 路径缺失（待 0x28B 记录语义）。
  - 修法建议：对 apply_decorator_place 0x140a20c70 逐偏移映射字段（补 +0x48/+0x4c/+0x50/
    +0x55/+0x56 到 struct 或注释归属），on_wall_split 按 RE §6.4 定案后补 removed 分支。
- 跨组阻塞：audio_resources.rs:183（audio）+ meta_commander.rs:314 测试（state）——非 decorator，
  全库复验待他组收口。若协调者接受上述两项为登记待取证，卡5 可降为 review_passed（限定）。

## 二轮复验（2026-10-16，restorer fresh 反编译定案后）

### ① DecoratorInfo 读面 = 包裹记录 — ✅ 已定案，P/F 撤销

fresh 0x14155afe0（Decorators::insert<DecoMoveToStorage>）证实：HashMap::insert 用
in_R8D（=+0x4c）作插入键；输出 72B 结构逐字段写回 +0x00..+0x47，`if(cStack_31==2)` 写
*(in_RCX+0x47)=2（状态字节=live）；内嵌 map 条目 0x50B={u32 key,pad,DecoratorInfo 0x48B@8}
→ **DecoratorInfo 恰 72B，+0x48..+0x56 属包裹记录（op 事件），非 DecoratorInfo**。
现实现：`rank u8@+0x44 / u45 u8@+0x45(U) / ty u8@+0x46 / state u8@+0x47` 拆分 + 
`DecoratorInfo::state()`；+0x4c/+0x2c 并存按原传递；注释更新 +0x48..+0x56=包裹记录 ✓。
遗留 U：+0x45。

### ② on_wall_split removed 路径 — ✅ 已修复，GAP 撤销

fresh 0x140a1f590 证实：逐装饰记录 d2s remove_entry（miss==3 panic 0x142ac9300）+
**+0x30 注册表 `HashMap<DecoratorStorageAddr, HashSet<DecoratorId>>`**（rustc_entry+set
insert，不拷入子墙内嵌 map）。现实现：`removed: HashMap<DecoratorStorageAddr, HashSet<u32>>`、
on_wall_split 重写（d2s remove + 注册表 insert + get_or_insert_default 新建桶取种子）、
iter_removed_storages 改迭代键、insert 补 removed_ids 撤销 ✓。
**残留 PARTIAL（低）**：子侧判定仍用注入 `side_of(info)` 闭包；fresh 定案为状态字节
`(b-2)<=2 ? b-1 : 0`、侧 2/3 panic unreachable——闭包应实现该公式，侧 2/3 的 unreachable
panic 未强制。

### 实测

decorator 族 **18/18 测试过**（新增注册表断言/种子）；cargo check 过；`cargo test --lib`
**1634/1**（唯一败 geometry/sphere 他组，roofs/roof_noise 已消）。

### 结论

卡5 = **review_passed（限定）**：① DecoratorInfo 读面裁决（包裹记录，72B 成立）✅；
② on_wall_split removed 路径（注册表 + d2s remove）✅。登记项：子侧状态字节公式/侧 2/3
panic（低）、+0x45、0x28B 记录/info 重放消费/注册表取留（I）。

### E2 对账快速确认（2026-10-16，注释级）— ✅ 闭合

1. `+0x45` = src `u45`（raw 名）与 E2 rank/state 簇 `+0x44..0x47` 吻合；E2 移除拷贝 0x48B =
   {8×u64@+0x00..0x40, state@+0x47}——第 8 u64@+0x40 覆盖簇，rank/u45/ty/state 为该 u64
   低字节分解，一致 ✓。
2. type@+0x48/+0x46 并存与卡5 §7 一致（+0x46=info 内 type，+0x48=包裹记录 type，72B 边界
   +0x48 外）✓。
3. 仅注释更新（struct doc + u45 字段注释登记 E2 出处），无结构/字段改名；u45 精确语义 U 保持 ✓。
实测：decorator 25/25 过；cargo test --lib 1672/1（唯一败 geometry/sphere 他组）。

### §11 三项闭合（2026-10-16，快速确认）— ✅ 全过

1. **side_from_state 全值域**：`(b-2)<=2 ? b-1 : 0`（u8 回绕）——`2 < b.wrapping_sub(2)`→0 否则
   `b.wrapping_sub(1)`；b∈{0,1}→0、2→1、3→2、4→3、其余→0 ✓（测试全值域 0/1/2/3/4/5/0xff/0x84）。
   `on_wall_split` 移除 side_of 闭包，改读 `DecoratorInfo::state(+0x47)` + `side_from_state`；侧 2/3
   → `unreachable!("internal error: entered unreachable code")`（0x142ac8d80）✓；3 测试（全值域/
   state0→侧0+state2→侧1 迁移+removed 注册表/state3→侧2 panic）。
2. **u45 消费端读面**：全 src grep 无库内 rank/ty/state/u45 消费（唯一 ui_focus_decoclutter 搬运
   DecoratorInfo 载荷）；render/实体消费在库外 system_decorator（PDB inline 宿主
   decorator_visual::stairs_assemble / ui_systems::ui_focus_stairs）——u45 语义 U 待 system_decorator ✓。
3. **contains_shape 自圈纠偏**：Self::Circle 分支改 `other.get_pos_at_u(0.0)`（非 center()），
   对章节 O（圆 0x140c975a0(payload,0) / 矩形 0x140c99600(payload,0)）✓；矩形含圆采样集与 4 角
   不变 ✓；u=0 边界点测试。

⚠️ 现场全 lib 测试目标被 he组在途 StairsState drain_dirty_edges 借用错阻塞（非 eco），
decorator 27/valid_enclosure 6 以协调者基线为准。
