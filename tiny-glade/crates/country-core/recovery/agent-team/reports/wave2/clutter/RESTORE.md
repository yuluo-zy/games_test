# wave2 clutter 还原报告（restorer-wave2-clutter）

日期：2026-10-17。范围：`crates/systems/clutter`（package `system-clutter`）3 函数：
`add_autoclutter_on_windows` / `populate_autoclutter_regions` / `load_clutter_meshes`。
标准：以 RE-ANALYSIS（hardpoints-interaction §1 / hardpoints-ecstate §1）解码成果为输入，
逐语句判等；重建不了如实标未实现（硬标准，不造近似体）。

> **reviewer-wave2-geometry 复核后修复（2026-10-17）**：分歧点 3 已修 + 附带登记 2 已落
> （见 §5 修复记录）。判等 8 项复核成立。

## 结论速览

| 函数 | VA | 分类 | cargo check |
|---|---|---|---|
| `add_autoclutter_on_windows` | `0x1422cd7d0` | **重建（逐语句）**——完整反汇编 + 跳表 + raw bytes 全量判等 | ✅ |
| `populate_autoclutter_regions` | `0x1422d6ed0` | **重建（分派结构与 kind 槽表）** + 5 项 U 登记（不伪造） | ✅ |
| `load_clutter_meshes` | `0x1422c0760` | **判等（结构）**——55×closure_0 + from_iter 证据；路径表仍 I/U | ✅ |
| `AutoClutterSpawn`（+f58） | mod.rs | 0x68 布局 +0x58 补字段（process 文档表已证） | ✅ |

`cargo check -p system-clutter`：**通过**（exit 0；2026-10-17 实测，country-core 稳定窗口内
一次通过）。中途曾遇 country-core `rhapsody_contract.rs` 被 rhapsody 域并发 agent 删除
`managed::render_pass/framebuffer` 模块的 5 错误（与 clutter 无关，非本批引入）；rhapsody
agent 随后修复，复测通过。

---

## 1. `add_autoclutter_on_windows`（0x1422cd7d0，size 1622）— 逐语句重建

### 证据（本次新增）
- `evidence/`（reports/re-analysis/hardpoints-interaction/）下：
  - radare2 全函数 430 指令反汇编（前批）+ **raw bytes 缺口定解**：0x1422cd8ba..0x1422cd8c0
    = `66 0f 1f 44 00 00`（6 字节 NOP 对齐垫）——**证实写块 +0x08 读栈槽 [rbp-0x60] 全函数
    从未被写 = 原 B 未初始化读**（前批 "unknown ①" 关闭）。
  - 常量 hexdump：`[0x1429cd0c0]=(0,0,1,0)`、`[0x142925904]=1.0`。
  - 跳表 0x143042370 6 项 disp32（interaction/ecstate 两份 RE-ANALYSIS 交叉一致）。

### 判等要点（radare2 逐条）
1. **drain 输入**：`[r9]` 段 {data@+8, len@+0x10}，入口 `mov [r9+0x10],0`（len 清零）。
2. **候选 stride 0x50**：kind@+0x00（==6 跳过）、flag@+0x01、f04@+0x04、host_index@+0x08、
   位置 16B@+0x10、f20@+0x20、f28@+0x28、f40@+0x40、f48@+0x48。
3. **跳表 6 分支**（仅宿主源 + 标志不同）：
   | kind | 宿主源 | 标志 |
   |---|---|---|
   | 0 | base+0xf0 | r12b=2, r14d=0 |
   | 1 | base Vec（data@+8,len@+0x10，以 host_index 为下标；`jae→panic_bounds_check`） | r12b=2, r14d=0 |
   | 2 | base+0xf8 | `NOT r12b; AND r12b,1`（翻转 flag）、r14d=0 |
   | 3 | base+0x1a8 | r12b=2, r14d=0 |
   | 4 | base+0x1b0 | r12b=2, r14d=0 |
   | 5 | base+0x1b8 | r12b=2, r14d=0 |
4. **共享向量数学**：`reflect = 2(p·q)p + (w²-|p|²)q`，q=(0,0,1,0), w=0 →
   `(2xz, 2yz, 2z²-|p|²)`。
5. **写块 0x68 stride**：+0x00=5、+0x18=1、+0x20=候选+0x48、+0x30=标志、
   +0x34=r14d(0)、+0x38=宿主首 qword、+0x40=候选+0x40、+0x48=候选+0x20、
   +0x50=候选+0x28、+0x54=reflect.x、+0x58=reflect.z、+0x5c=reflect.y、+0x60=1.0。
   **+0x08/+0x31/+0x33 = 原 B 未初始化栈读**（raw bytes 证实，登记不伪造）。

### 修复（相对 F3 近似）
- f54/f5c 原误写 `candidate.f40/f04` → 改反射结果 x/y（+0x54/+0x5c）；补 +0x58=z。
- 宿主源原 `min(kind,5)` 粗分派 → 按 6 分支偏移分派 + kind 1 越界 panic。
- 位置原 `(reflect.x, p.y, reflect.z)` → 二进制 xzy swizzle（+0x54..0x60）。

---

## 2. `populate_autoclutter_regions`（0x1422d6ed0，size 44132）— 分派重建 + U 登记

### 证据
- wave2 全量 120KB fresh 反编译（`reports/re-analysis/hardpoints-ecstate/evidence/decomp/
  populate_autoclutter_regions_1422d6ed0.c`）逐语句精读。

### 判等要点
1. **kind switch**（记录+0x41，`switch(*(char*)(local_138-7))`）：case 0/2/3/4 各建
   9/25/25/37 槽 `{网格表指针, f32 权重}`（stride 0x10）；case 1 → 独立处理器 0x1422dd1d7；
   其余 → default；**万圣节（is_halloween2024）跳过 kind switch**（local_180 恒 0）。
   每槽 `[entry+0x30]==-1`（Option None 哨兵）→ 权重 0。
2. **kind → 网格槽表**（写块逐槽提取，已编码 KIND_MESH_SLOTS）：
   - kind 0：`[(9,1.2),(8,1.2),(3,2.5),(4,1),(5,1),(6,1),(7,1),(10,1.5),(11,1)]`
     （mesh-10 = `(float)local_108 * fVar53`，local_108 写前 ×1.5（行 768）→ scale×1.5 链，
     scale=1.0 → **1.5**；reviewer 分歧点①已修）
   - kind 2：`[(10,1.5),(25,.75),(24,.5),(23,1),(21,.75),(20,.75),(19,.5),(17,.75),(16,.75),
     (22,.75),(14,1),(15,1),(9,1.2),(8,1.2),(3,2.0),(18,1.5),(4,1),(5,1),(6,1),(7,1),
     (11,1),(28,.5),(29,.5),(26,.5),(27,.5)]`
   - kind 3：25 槽（idx 17 起 0.75/0.5 权重，首 6 槽 1.0）
   - kind 4：37 槽（memmove 打包循环写出；槽序确定，部分权重④登记）
3. **mesh-type 跳表**（记录+0x47，DAT_14304382c）：0/1 = 加权随机挑网格
   （`init_rng_u64(键哈希)` + `fastrand::f32` + 权重/total 累和选中；落点
   `maxps(0.01,Δ)→floorf→xvy`）；**选中算子（reviewer 分歧点②已修）：`do {acc+=w/total;
   idx++} while(acc<=r)` 在 acc>r 严格选中（等号继续）；count==0 / 耗尽 r>=total 均按原 B
   `panic_bounds_check`（非静默末槽兜底）**；2 = 跳过；3/4 = 固定候选槽 +0x25/+0x26；
   7 = 计数累积 `[cand+0xad]++`；5/6/8/11/12/13/14/15/18/19 → 透传；9/0xa/0x11/0x14/
   0x15/0x10/0x18 → 错误/panic 路径。
4. **过滤器**：`legacy_raycast`（掩码 0x30fb）→ 结果==2 走水面；4 角 bilinear
   （半径 130）求和 clamp[0,1]；**水位阈值 0.0**。

### U 登记（不伪造）
- ① **网格表 `local_100[n]` → 具体 ClutterMeshTy 资产 = U**（需 load_clutter_meshes 资产
  清单对照；本批未判等）。
- ② kind 1 处理器 0x1422dd1d7 body、default（kind 5–25）槽表 = U（反编译未完整还原）。
- ③ 区域记录 stride 0x48 逐字节布局 / 逐区域记录循环 vs 现每格循环 = U（现以
  AutoClutterRegion 语义承载）。
- ④ kind 4 权重经 memmove 打包循环写出，部分权重寄存器复用歧义 = U。
- `select_group`（F3 近似）已 `#[deprecated]`，以 `KIND_MESH_SLOTS` + `select_mesh` 取代。

---

## 3. `load_clutter_meshes`（0x1422c0760，size 5383）— 结构判等

- wave2 fresh 反编译（`--worker shared`）+ reviewer 复核：全函数 = **55 次内联
  `closure_0()`**（reviewer 分歧点③已修：原"~59"微高估，shared DB 实测计数 55）+
  `from_iter<RangeInclusive<i32>, Map<RangeInclusive<i32>, closure_env_1>>` +
  `Commands::insert_resource<ClutterMeshes>`。结构判等落地（register_all 56 变体 + insert）。
- 路径表（CLUTTER_MESH_PATHS）仍为 **I 推断 + U 登记**（RE-ANALYSIS §1.4：strings 清单本轮
  未取（成本高、价值低））；已证名 anvil/door_bell/door_krans/stall_bread/rack_brooms/
  flowerbox/birdnest/firewood/cart_w_pumpkins/cart_w_p_candles 保留，其余不判等。
- 尝试 strings 提取（radare2 无 list_strings 于 operator 白名单；Ghidra search_strings 在
  listing 组，单 job 内目录快照不含——登记为未完成项）。

---

## 4. 附带修复

- `AutoClutterSpawn` 补 `f58: u32`（0x68 布局 +0x58；process 文档表
  "+0x58=源+0x54 高 32" 已证）；5 个构造器 + Default 同步。

---

## 5. reviewer-wave2-geometry 复核后修复记录（2026-10-17）

| 分歧点 | 修复 |
|---|---|
| ① kind-0 mesh-10 权重 | `(10,1.0)` → `(10,1.5)`（`(float)local_108*fVar53` + local_108 ×1.5（行 768）→ scale×1.5 链；与注释、kind-2 slot-0 一致） |
| ② select_mesh 边界/兜底 | `if r<=acc`（acc>=r 选中）→ `if acc>r`（严格，do-while `acc<=r` 继续）；兜底 `slots.last()` → count==0/耗尽按原 B `panic_bounds_check` |
| ③ load closure 计数 | "~59" → **55**（shared DB fresh 实测） |
| 附带登记 1：gothic rank∉1..3 | decorator-forest RESTORE.md 已登记（二进制 `jae→panic_bounds_check` vs Rust 默认 0.38） |
| 附带登记 2：AutoClutterSpawn +0x38/+0x40 标注错位 | mod.rs doc 已登记（process 语义 position@+0x38 vs on_windows 二进制 +0x38=host_id——并置/重载，不修） |

## 证据索引
- `reports/re-analysis/hardpoints-interaction/evidence/jobs+results/restorer-wave2-clutter-*`
  （r2-gap / r2-prologue / ghidra-load / ghidra-strings / ghidra-closures）。
- `reports/re-analysis/hardpoints-ecstate/evidence/decomp/populate_autoclutter_regions_1422d6ed0.c`。
- `reports/re-analysis/hardpoints-interaction/evidence/scripts/extract_kind_slots.py`、
  `extract_write_blocks.py`（kind 槽表提取脚本）。
- `reports/wave2/reviews/geometry/REVIEW.md`（reviewer 复核）。
