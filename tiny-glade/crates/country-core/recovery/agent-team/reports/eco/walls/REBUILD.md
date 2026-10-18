# REBUILD — walls 域 11 需重做函数真实逻辑重建（rebuild-walls，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。依据：`LOGIC.md` §1 的
> fresh 反编译分歧点 + 本轮 **fresh 反编译/反汇编复核**（`reverse-operator.py ghidra
> --worker shared`，`fields=decompiled_code,callees` + `disassemble_function`，
> 2026-10-17，落盘 `/tmp/rebuild-walls/`）。重建逐分支与机器码对应。

## 0. 分类结论

11 个需重做函数**全部为真实业务函数**（0 个宏生成胶水）——均有真实算法/控制流/调用链，
无 derive/serde visitor/Bevy 注册。

| VA | 函数 | 分类 | 重建 |
|---|---|---|---|
| 0x140a1f590 | on_wall_split | 真实业务（迁移 + u 区间搜索） | ✅ |
| 0x14155b170 | merge_with\<DecoMoveToStorage\> | 真实业务（3 循环 d2s 重索引） | ✅ |
| 0x140a1ea40 | on_walls_merged | 真实业务（双源/单源分支） | ✅ |
| 0x140a20d70 | restore_snapshot/take_snapshot | 真实业务（逐记录恢复） | ✅ |
| 0x1411ff330 | contains_shape | 真实业务（几何采样） | ✅ |
| 0x1411ff680 | expand | 真实业务（几何） | ✅ |
| 0x140a6ad00 | snapping_maintain | 真实业务（全状态发布） | ✅ |
| 0x140a6b5e0 | snapping_to_segments | 真实业务（排序取最近） | ✅ |
| 0x140aa5a10 | query_with_aabb_clamped | 真实业务（排序去重） | ✅ |
| 0x140a82b20/2b40/2d50 | CompoundEditBuilder 三件 | 真实业务（builder 语义） | ✅ |

## 1. 逐函数重建（分歧点 → 重建内容）

### 1.1 `DecoratorStorage::on_wall_split` 0x140a1f590
- **fresh 反汇编定案**：参数 = `(wall, children: &[(WallId, low, high)], curve: &Curve2)`
  ——子段记录 16B = {WallId@0, low@8, high@0xc}；侧 0 分支点 =
  `Vec2::new(f32@dst_payload 高 32, f32@u18 高 32)`，get_approx_u_from_pos →
  clamp[0,1]（MAXSS/MINSS）→ 区间包含 `low<=u<=high`（SETC/CMOVC 有序比较，两端含）。
- 重建：签名改为三参；侧判定 = **判别字节 DecoratorInfo+0x22**（= `dst_tag()`，非
  +0x47 state）；侧 0 **命中直接回插子墙桶**（get_or_insert_default + Decorators::insert
  + **d2s 重注册到子墙**——原生 `Decorators::insert<DecoMoveToStorage>` 内含 d2s
  insert/update），未命中落 removed 注册表（键 = **原墙地址**）；侧 1 落注册表（原墙
  地址，d2s 不重注册）；侧 ≥2 unreachable panic。**状态字节==2 循环 break**（fresh
  break 先于 d2s.remove）。源已 removed_ids 并入注册表（原生首块）。
- 分歧点对照：Rust 此前删侧 0 u 区间搜索与直接回插、`children[side]` 下标替代归属判定、
  侧 0/侧 1 合并同一注册表路径——已重建。

### 1.2 `Decorators::merge_with<DecoMoveToStorage>` 0x14155b170
- 重建：签名改为 `merge_with(&mut self, source, d2s: &mut DecoratorToStorageMap,
  addr: DecoratorStorageAddr)`，**3 循环**：
  ① 迭代 source.removed_ids，目标内嵌 map 无此 id → 目标 modified_ids 传播；
  ② 迭代目标内嵌 map → 逐 id d2s remove（miss panic）+ d2s insert(addr)；
  ③ 迭代 source 内嵌 map（状态字节==2 break）→ d2s remove + 目标 removed_ids remove +
  modified insert + **DecoratorInfo 重插** + d2s update(addr)。
- 分歧点对照：Rust 此前纯 map 合并（冲突→removed_ids），完全不含 d2s 重索引与
  removed_ids 迁移——已内置回 merge_with。
- 注：LOGIC §1.2 三循环"① 迭代 source 内嵌 map / ③ 迭代 source.removed_ids"的描述与
  字段偏移（① 控制 @src+0x0、③ 控制 @src+0x30）**迭代集合互为对调**；按偏移 +
  语义（③ 需 DecoratorInfo 才能重插）实现为 ①=removed_ids、③=内嵌 map。

### 1.3 `DecoratorStorage::on_walls_merged` 0x140a1ea40
- 重建：双源 remove_entry 后 **仅两源并存才调 merge_with**（d2s 重索引内置于
  merge_with）；单源路径逐装饰 d2s remove（miss panic）+ d2s insert(merged)，随后
  `per_wall.insert(merged, 源)`。
- 分歧点对照：Rust 此前恒建 target=default、分离 d2s 时机、重复 id 跨源 assert_exists
  panic（原 merge_with 冲突走 removed 去重不 panic）——已重建。

### 1.4 `restore_snapshot` / `take_snapshot` 0x140a20d70
- 重建：`DecoratorAddrSnapshot` 扩展携带 **`info: DecoratorInfo`**（非 16B 近似）；
  restore_snapshot 逐记录 `get_or_insert_default` → **removed_ids.remove +
  modified_ids.insert + 内嵌 map insert(info) + d2s insert（冲突 panic）**。
- 分歧点对照：Rust 此前仅 removed_ids.insert + d2s.update，**不恢复 DecoratorInfo**；
  fresh 证实原确实回填内嵌 map 并逐状态还原——已重建。0x28B/0x16B 节格式逐槽字段名
  与状态跳表（0x142ac9bdc/0x142ac9bfc）字段提取**仍待①**；本 crate 以携带完整
  DecoratorInfo 的捕获模型承载（save 层 JSON→info 映射待①，当前传默认 info）。
- take_snapshot：前置 6 断言保证各桶为空，捕获面为空（与原生同址前置一致）。

### 1.5 `ShapeParameters::contains_shape` 0x1411ff330
- **fresh 反汇编**：4 路采样常量 = `0, 1/6(0x142c197c8), 1/3(0x142a86330),
  1/2(0x142925870), 2/3(0x142b1e8b8), 5/6(0x142c197cc)`（含 1/2 非 1.0）。
- 重建：自=圆/他=圆 **6 采样**；自=圆/他=矩形 **4 角**；自=矩形/他=圆 **6 采样**（修正
  原 {0,1/6,1/3,1.0,2/3,5/6} 的 1.0→1/2）；自=矩形/他=矩形 4 角。
- 分歧点对照：restorer §11.3 的"u=0 边界点"单点纠偏与机器码不符（原非单点）——已重建。

### 1.6 `ShapeParameters::expand` 0x1411ff680
- **fresh 反汇编**：圆 tag==0 → 仅 r += amount；+0x14/+0x18 **保留原值不变**
  （`~uVar7 & (2a+fVar5) | fVar5 & uVar7`，圆时 uVar7=全1 → 保留）。
  矩形 tag≠0 → +0x14/+0x18 **在原值上加 2a**（`2a+fVar4/fVar5`，非覆写）。
- 重建：圆只改 0xc；矩形 hw/hh 加 2a。
- 分歧点对照：restorer 对圆车道"+0x14/+0x18 +=2a"与矩形"覆写 2a"的描述与机器码相反
  ——已重建。

### 1.7 `snapping_maintain` 0x140a6ad00
- **fresh 反汇编**：按 mode 跳表读 snapped/snap/wall/end，**整体发布到 +0x20..+0x38**
  （mode@+0x20、snapped@+0x21、snap@+0x24..+0x30、wall@+0x30、end@+0x38）。
- 重建：SnappingMode 增加 `published: SnappingModePublished`（全状态副本字段）；
  snapping_maintain 发布全状态；snapping_audio 改读 `published.mode`。
- 分歧点对照：Rust 此前只发 1 字节模式——已重建（去重语义仍只比模式字节，等价）。

### 1.8 `snapping_to_segments` 0x140a6b5e0
- 重建：query 候选按到游标距离**排序**后取**最近**（sort_by + total_cmp），再判
  `SQRT < 1.5` 写 mode=3（mode.end 不再写 +0x18，fresh 机器码不写）。
- 分歧点对照：Rust 此前遍历取任意首个（无排序），空间哈希返回序非距离序时结果不同
  ——已重建。

### 1.9 `WallSpatialHash::query_with_aabb_clamped` 0x140aa5a10
- 重建：get_cells 收集后 **按 AABB 中心距离排序**（复用圆查询比较器语义；AABB 无圆心，
  参考点取 AABB 中心）+ **同墙去重 keep 最近**（原生 sort 后相邻同墙折叠）。
- 分歧点对照：Rust 此前直接遍历 cell 无序收集、按端点位型去重——已重建。
  返回类型 `Vec<([f32;2], WallId)>` 不变（critters 消费者兼容）。

### 1.10-12 `CompoundEditBuilder::new_edit_sequence / next / build` 0x140a82b20/2b40/2d50
- **fresh 反汇编**：`new_edit_sequence` 只写 Vec {ptr=0, cap=8, len=0}；`next` alloc
  0xf8 + memcpy 单条 edit（Arc 0x108 承载）→ grow_one → push 新组（无 is_empty 门）；
  `build` 直接 from_iter 收集（无 None/空判据）。
- 重建：builder 去掉 `started`/`current`/`next_empty`/`with_concurrent`；`next(edit)`
  push 单条 edit 新组；`build() -> WallCompoundHistoryEdit`（无 Option）。
- 分歧点对照：Rust 引入的 started 标志、next 空门、build None 分支在机器码中无对应
  ——已重建（Arc 为原生组内承载细节，公开 `WallCompoundHistoryEdit` 组类型保持不变，
  避免跨模块 ripples）。

## 2. 判等-注意补正（重建触发的登记）

- `apply_decorator_place`（0x140a20c70）：native insert 内 `if(state==2) d2s insert +
  *(rcx+0x47)=2`；Rust 不强制 state=2 —— 维持登记（非本批 11 函数，不重建）。
- `apply_decorator_remove`（0x140a20670）：native d2s remove miss==3 → panic；Rust
  `let _ = d2s.remove()` 忽略 —— 维持登记。

## 3. 自检

- `cargo check -p country-core` **通过**（无新增 error/warning）。
- `cargo check -p system-critters`（query_with_aabb_clamped 下游消费者）**通过**。
- 独立几何验证（`/tmp/walls-verify`，复用 utils crate）：Curve2 get_approx_u_from_pos
  @(5,0)/(2,0) 沿 (0,0)-(10,0) = 0.5/0.2 ✓；圆 6 采样/矩形 4 角包含判定 ✓；
  expand 矩形加 2a → (5,9) ✓。
- 独立算法验证（`/tmp/walls-verify2/3`，std-only 镜像重建分支）：on_wall_split
  侧0命中（d2s 重注册）/侧0未命中/侧1/state==2 break ✓；side_from_state 判别字节 ✓；
  merge_with 三循环（含 state==2 break、Loop① modified 传播）✓；restore_snapshot
  （内嵌 map 回填 + d2s insert）✓。
- **限制**：country-core lib test 目标存在**与本次无关的既有编译错误**（rhapsody
  refactor 后 render/shader.rs、render_loop/*、ssbo_library.rs、garden/tests、
  terrain/tests 等的 `GraphicsDevice`/`Device` 结构体 vs 函数用法），无法运行
  `cargo test -p country-core --lib`；本批改动文件（decorator_storage/snapping/
  wall_spatial_hash/wall_history_edit/valid_enclosure/save/snapshot）无 test error。

## 4. 结论

- 分类：11/11 真实业务（0 胶水）。
- 重建：11/11（含 take/restore 合并计数）。
- 编译：country-core + 下游 critters 通过；测试运行被既有无关错误阻塞。
- 红线遵守：只以 fresh 反编译/反汇编为准；重建逐分支一致；未删除判等函数；
  未改他模块业务（仅 save/snapshot.rs 构造点随 DecoratorAddrSnapshot 新增字段做必要
  ripple）。
