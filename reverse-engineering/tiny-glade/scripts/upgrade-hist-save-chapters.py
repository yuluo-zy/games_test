# -*- coding: utf-8 -*-
"""2026-10-15 history/session-save 域章节算法级升级：为各章追加"实现方式（算法级）"节。
只追加文档节，不改函数表。"""
import io, sys

BASE = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

SECTIONS = {}

SECTIONS["resources/history/mod.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 history/ALGORITHMS.md）

- **History::add 0x140a84ec0**（3 参：`(&mut self, edit: &HistoryEdit 24B, file: &'static str)`）：
  1) `pending(+0x30) < 0` → panic "Tried writing {edit:?}…"（@0x142ad8250，参数 = EditType Debug::fmt 0x140a842c0 + track-caller 文件）；2) 桶空且 `cursor - len < -2` →
  先 `nuke_future_states 0x140a84c50` 再重建 pending 桶（64B 装饰表 Arc{1,1}+模板指针
  0x142af1470..0x142af1488+全局序列 FUN_1400d9b20；96B 楼梯表 Arc{1,1}+两空 Vec）；
  3) **tag 7（墙）校验**：records = 组数组（16B {入口指针数组, 组数}），逐条目 niche
  解码（`载荷+0x10 - 0x7fffffffffffffff`）：变体 0 → 断言 +0x100 f32 > +0xfc f32
  （否则 panic "Slice start_u must be smaller than end_u. Otherwise an invalid curve
  will be created!" @0x142ad8170）；变体 1/2 → 断言 +0x18 >= 0（panic "None is not
  allowed in `CreateWall`/`DeleteWall` history command" @0x142ad81b8/0x142ad8200）；
  4) push 24B {tag, records, count} 进 pending.edits（元素 0x18）。
- **nuke_future_states 0x140a84c50**：`cursor == len-1` → 仅重随机 nonce；`cursor < 0`
  → 全重置（drop edit_chain + 重建 {cap=0,ptr=8,len=0, pending=i64::MIN, cursor=-1,
  tail=0}）；否则 truncate 至 cursor+1（逐个 drop 尾元素）+ `count(+0x58)=min(count,len)`
  + `FUN_1412f5320` = snapshots.retain(chain_index<=cursor)；末尾**无条件**
  `nonce(+0x60) = fastrand u64`。
- **insert_starting_edit 0x140a844f0**：断言 cursor>=-1 → cursor+1 → edit_chain 首部
  插入（`func_0x000142923c10` memmove 右移 0x28）→ nonce=fastrand、count=0。
- **finish_input_recording 0x140a86730**：pending 先置 i64::MIN（原值 <0 → panic
  "Called finish_input_recording when history recording was already Finished"
  @0x142ad84a8）；MultiEdit::is_empty 四条件（edits.len==0 && 装饰表+0x28==0 &&
  楼梯表+0x40==0 && +0x58==0）→ drop 返回 0；非空 → push 0x28B 上链、cursor+1
  （断言 >=-1）、nonce=fastrand；仅当 cursor == len-1 时快照追加（48B 记录
  {i64::MIN 待定 id, 1, 计数器}，否则 panic @0x142ad8440）。
- **start_input_recording 0x140a86270**：谓词 `func_0x000140a825e0(mode, 0xd)` =
  `app_mode != Replay(0xd)`，失败 → panic "assertion failed: **app_mode !=
  AppMode::Replay"（@0x142ad83f8）；建新桶。**try_start 0x140a86510**：谓词通过且
  pending < 0 → 建桶返回 1，否则 0。
- **current_snapshot 0x140a84650**：snapshots（+0x20/+0x28，元素 16B {Arc, idx@+8}）
  partition_point(idx < cursor) 二分 + **距离平局**（|prev-idx| vs |next-idx|，平局取
  后者）；Arc clone 后 **idx == cursor 才返回 Some**。**closest_resolved_snapshot
  0x140a84a50**：先 filter_map（TryFrom 0x140a84260 谓词）收集，再同一二分+平局，
  不 clone（借用）。**set_current_snapshot 0x140a84760**：二分后若存在 idx==cursor
  快照 → panic（断言不替换，@0x142ad8050 mod.rs:0x33）；否则在 partition_point 处
  memmove 右移 0x10 有序插入 {Arc, cursor}。
- **finish_potential_mutation 0x140a86980**：was_active → 清空 snapshots（逐记录
  LOCK dec Arc，归零 drop 0x1402e5d80）+ nonce=fastrand；token 未消费 panic
  "History was mutated, but wasn't told about it"（@0x142a98ad8）由 Drop 守卫触发，
  panic-in-progress 门控 `*0x14362c420`。
"""

SECTIONS["resources/history/replay_cmd_buffer.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 history/ALGORITHMS.md）

- **ReplayBuffer 布局**（本域机器码修正）：头部 = 桶 Vec（cap-first，**元素 0x40**）
  @+0x00..+0x18 + 活跃计数 @+0x18 + 速率 f32 @+0x28（replay_wall/terrain/stairs 首行
  读 `+5` qword 即 +0x28）；reconstruction 的"环形 {head,buf,cap,len,
  modification_counter}"无证据（U）。
- **ReplayBucket 0x40B**：cmds Vec cap-first {cap@+0x00, ptr@+0x08, len@+0x10}
  （replay_wall 直读 +0x08/+0x10 迭代 0x30 步长证实；DIGEST 的"arcs@+0x00/+0x08"
  表述以此修正）、+0x18 标志字节区、decorators Arc / stairs Arc @+0x20/+0x28 区、
  +0x30..+0x3f 保留（精确槽位 U）。
- **ReplayCmd 0x30B**：判别 u8@+0x00、instant/标志@+0x08、duration f32@+0x0c、
  负载@+0x10、elapsed@+0x18、undo@+0x1c；有效判别 0..=10（10=Stairs），>10 →
  容量错误路径（0x140af6660）。
- **pop_front 0x1409f18f0**：弹前桶 + active--；**make_animations_instant 0x1409f1ab0**：
  全命令即时完成（duration=0 路径，replay_terrain instant 分支实证）。
- 命令完成后的移除 = 逆序索引 + memmove 压缩 + 按判别字节 drop 分派（见 ALGORITHMS
  §2.6 通用骨架）。
"""

SECTIONS["resources/history/wall_history.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **ev_new_wall 0x140a83300**：断言 `new_wall.curve.length_ws() != 0.0`
  （wall_history.rs，@0x142ad7d48）；构建 0x108B WallHistoryEdit（载荷）+ FUN_141696f50
  校验 → `History::add({tag 7, records=&组表, count}, file)`；records = 组数组
  （16B {入口指针数组, 组数}），add 侧按 niche 分变体做曲线/None 校验（见 mod.rs
  算法节）。
- **ev_height_changed 0x140a83f80**：栈 24B 缓冲 → `EditType::new_height_changed(buf,
  wall, h0, h1)`（0x140a83fb0：写 {tag 7, records=&0x108B 载荷, count} + 载荷
  {1, 1, niche 0x8000000000000002=变体2, wall, h0, h1} + 0x108B 拷贝 + 校验）→
  `History::add(self, &buf, &静态文件串 0x142ae7818)`（track-caller）。
"""

SECTIONS["resources/history/wall_compound_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **CompoundEditBuilder**：new_edit_sequence 0x140a82b20（开始新序列）→ next
  0x140a82b40（推进到下一组）→ build 0x140a82d50（组装 Option<WallHistoryCompoundEdit>
  0x108B）；`wall_deleted 0x140a876a0` = systems::wall::deferred_wall_history 的 trait
  impl（墙删除合并入口）。panic_deprecated_operation 0x140a82e20 = 已弃用操作 panic。
- **WallHistoryEdit Debug::fmt 0x140a82eb0**：首字段 niche 判别（`*p -
  0x7fffffffffffffff` 得变体）经跳表 0x142ad888c 分派打印各操作变体。
- **update_compatibility / gather_wall_ids**：无独立符号（内联；CLOSURE §1 排除
  0x140a82940..0x140a82b20 全为 drop/fmt glue）。
- serde deserialize 族（0x141401960/0x1414022c0/0x141402680/0x141402790）：操作 tag
  串 ChangeFreehandHeightType / PlasterPeelAmount / ChangePlasterPeelAmount /
  MorphPremadeShape / MorphFreehandWall…（~40 变体 visitor）。
"""

SECTIONS["resources/history/decorator_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **DecoratorHistoryState::From 0x140b3f930**：源状态 type 字节 @+0x41 == 8（preview）
  → panic "Attempting to capture a DecoratorHistoryState of a preview "
  （decorator_edit.rs，@0x142aeeea0）；变体 2/3 读 u32@+0x30、4/5/6/7 读 byte@+0x30、
  其余读 +0x34；输出 0x50B+ 记录（字节字段 @+0x44..+0x4e）。
- **convert_before_after_states_to_changes 0x140b3fab0**（`(out, before, after)`）：
  按 style 字节（+0x32）分变体比较浮点（变体 2/3 比 4 个 f32、4 比 6 个、default 比
  8 个 f32 + 字节 + u64）；style 族字节（+0x5a，-0x38 后 clamp 6）不等 → 输出 '@'
  哨兵；输出 = after 拷贝 + changed 布尔 @+0x40/@+0x52。
- **is_default_lantern_brightness 0x140b3f910**（= is_default_stair_width 同址 ICF）：
  `x == 1.0f`（常量 @0x142925904）；reconstruction 的 `(record)->bool` 签名错。
- **send_state_diff_events 0x1417bdae0**：replay_pop_bucket 消费（restore_decorators
  宿主 0x140b38250 的 9 参参数面）。
"""

SECTIONS["resources/history/terrain_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **TerrainEdit 判别**（Debug::fmt 0x140858c80）：首字段 niche `x^0x8000000000000000
  < 2` → 变体 0/1/2（0=NewStroke(+4 处 stroke id)、1=+1 处 0x11B 载荷、2=+4 处
  0x10B 载荷）。
- **revert 0x140858d30 / reapply 0x1408592d0**（对称）：判别 0（NewStroke）且
  `param_6 < 0.99f`（静态 0x142a948d8 = 动画进度阈值）→ panic "Requested terrain
  stroke {id}"（terrain_edit.rs，@0x142a94cf8）；否则 hashbrown 查 48B/元素 stroke
  表（FUN_1405b0ed0），命中 → 清/置该 stroke active 标志（+0x28）。
- **duration**：无独立符号；公式内联于 `TerrainReplayCmd::forward 0x140994f30`：
  `clamp(Σ相邻点距*0.01, 0.4, 3.0)`（cm→m 换算），无点默认 0.3；"assertion failed:
  stroke_length.is_finite()"（terrain_edit.rs:43，@0x142a94c34）。
- **update_terrain_curve 0x140859740**（reapply 闭包宿主）：hash 查 48B stroke 表 →
  拷贝 12B 点集 → FUN_141469230 fold 求进度 → 与全局时间 FUN_140c95ef0
  (&0x142a94d48) 做 `(1-t)*u + t` lerp → 0..1 范围断言（复用 "Tried to init CurveU
  from an invalid u: "）→ 回写 stroke 记录 Vec。
"""

SECTIONS["resources/history/roof_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **RoofEdit 操作枚举**：FlatRoofRimStyleChange（含变体）/ SwitchRoofRidgeDir 等 tag；
  手写 serde visitor 族（serialize 0x140259e30 / deserialize 0x140b38b80 / visit_str
  0x140b392d0/0x140b39400/0x140b394f0/0x140b395d0）——按操作 tag 串匹配分派，加载存档
  时还原屋顶编辑负载。重建侧缺此模块（按合同恢复，无自定义业务算法）。
"""

SECTIONS["resources/history/color_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **ColorEdit 负载**：目标类型（Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/
  StairLadder/Brightness）+ from/to 颜色 + 目标 id + chimney_h_offset/allow_connection
  字段；手写 serde visitor 族（serialize 0x14003c5d0 / deserialize 0x1417befe0 /
  visit_str 0x1417bf8a0，字段名 lantern_brightness/chimney_smoke/…）。
- 运行侧消费：load_game 旧版颜色 ID 升级（{1→2,2→5,4→8,5→3,8→4}，+0x52∈
  {0x38..0x3a} 记录 +0x30 字段）与 save_game Colors 节（to_json_string 0x14085ee90）。
"""

SECTIONS["resources/history/path_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **RasterEdit**（points/brush_size/is_additive/blend_mode）：ReplayCmd 判别 0/1/2 的
  Arc 负载（make_animations_instant 的 instant 路径走判别 0/1/2 的 Arc 变体 drop）。
- **RasterEditSerialized**（存档载荷）：手写 serde（serialize 0x1404896f0/0x1416e8340、
  deserialize 0x1416e8510——**注意地址为 0x1416e8510，丢一位 1 的旧写法 0x16e8510
  错**）；`impl$1::from 0x140a976b0` = DecoratorHistoryState::From 同族转换。
"""

SECTIONS["resources/history/auto_clutter_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **AutoClutterEdit**（墙派生装饰记录：host wall_id + decorator_ids）：手写 serde
  visitor 族（serialize 0x14003d300 / deserialize 0x140a18500）。
- 运行侧消费：replay_auto_clutter 0x140aa4a60（undo==0 插墙集 / !=0 从
  DeletedAutoClutter erase (DecoratorId, DecoratorId)）。
"""

SECTIONS["resources/history/stairs_history_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **StairsHistoryEdit/StairsHistoryDiff**（边对 before/after + hash）：手写 serde
  （serialize 0x140259af0 / deserialize 0x140a18ee0）。
- **运行时 StairsDiffTable 96B**（add_decorator_diff 实证）：{tag0@+0x00, tag1@+0x08,
  图标志@+0x10, checksum@+0x18, +0x20..+0x2f u32×4, vec_a@+0x30 {cap,ptr,len},
  vec_b@+0x48 {cap,ptr,len}}；Arc 包装版 = 表体整体后移 0x10（{1,1}@+0x00/+0x08）。
  is_stairs_history_diff_empty 0x140a842a0 读 cap(+0x30/+0x48)（对空 Vec 与 len 等价）。
- **checksum 语义**（add_decorator_diff 消费）：入参 tag1 必须 == edit_chain 中最近
  一个图标志表（Arc 表 +0x20==1）的 +0x28；不符 → log + panic checksum mismatch。
"""

SECTIONS["resources/history/tree_edit.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **TreeEdit 负载**（missing_prefab 0x1409e6210 等符号族）：树编辑操作（种树/删除/
  形态）载荷 + 手写 serde visitor 族；运行侧由 replay_raster 类系统消费（树为
  Raster 域之一）。
- missing_prefab = 树引用缺失 prefab 的判定/记录（具体规则 U；同族符号无自定义
  算法级行为）。
"""

# ---------------- systems/history ----------------

SECTIONS["systems/history/history_process.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 history/ALGORITHMS.md §2.3）

- **HistoryCmd 4B 记录**：byte[0]=tag、byte[1]=子命令（0=Undo、1=Redo、4=中断标志）、
  byte[2..3]=标志（传 generate_replay_cmds 的 u16 低位）。
- **history_process 0x140af5b30 主循环**：
  1. param_4 Local 双流（16B 记录）：`剩余 = 计数 - 游标`（负钳 0），计数 -= 两流剩余；
  2. 两流均空 → 守卫（ReplayBuffer+0x18 != 0 → return 让回放排空；param_9 队列 +0x10
     != 0 → return；Events<HistoryCmd> 空 → return）→ 消费一条 4B：
     byte[1]==4 或 byte[0]==2 → **Interrupt 直接 return**；byte[1]==0 → Undo 三重守卫
     （cursor>=0、cursor>history_start、cursor<len）后 cursor--；byte[1]==1 → Redo
     断言 cursor>=-1 后 cursor=目标；其余 → unreachable（history_process.rs:40）；
  3. 对 edit_chain[cursor] 的 edits 逐个 generate_replay_cmds（FUN_14143fb50 单态）
     写 0x30B 命令 Vec；Arc clone 两表（LOCK inc+溢出 panic）；
     **undo 且命令数>1 → 命令 Vec 整体反转**（0x30=6 qword 两两交换，双指针向中）；
  4. 0x40B 桶 push 进 ReplayBuffer（0x140af6381 区：桶 Vec 追加 + 命令区/索引 Vec
     维护 + 0x120 长度校验），逐命令经跳表 0x142ae545c 校验（重复/冗余丢弃：
     `tag&7!=0 → 无效`、tag0 instant+duration 全等 → 重复、tag4 count==1 置既有
     命令 +0x49 |= 0x10、tag>10 → 容量错误）；
  5. 两流非空 → 录制态检查（pending>=0 → log "Can't process History while recording
     a new edit is active!" @0x142ae5290，级别>1，return）→ `switch(byte[+9])` 经
     0x142ae5420：0x0/0x1 = 末 HistoryCmd byte[1]==1/0 门控、0x2 = 消费跳过、0x3 =
     make_animations_instant（0x1409f1ab0）、0x4-0xe = 校验/合并、0x13 = 按索引移除
     命令（拷贝 24B + memmove 压缩 + drop 分派）、0x17 = Arc dec、0x18 = return、
     0x19-0x1b 无效；消费动作 = 清 Events<HistoryCmd> + param_8 计数器 push 其
     Vec<u64>；每条记录（tag!=3）追加 param_6 的 Vec<16B> {计数器, byte[+0xb]}。
- **replay_edit 0x140af6170**：同构独立入口（生成→反转→桶 push→同套校验）。
- **generate_replay_cmds_from_edit ×4**：按 HistoryEdit.tag 经跳表（garden 单态
  0x142c5f248 / water 0x142c5f2fc，各单态独立）尾调共享核心（~10.4KB，非 316B
  跳板——316B 为 Ghidra 跳表恢复截断）；generate_path_replay_cmds 0x1416286d0 =
  niche 判别 0..3 过滤后尾调 0x1414423e0。
"""

SECTIONS["systems/history/history_recording.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **history_finish_recording 0x1409dd790**：多事件游标（param_7/8/9）+ AppMode
  检查（0x140b2cb30 等辅助）→ `History::finish_input_recording 0x140a86730`
  （取 pending → 四条件空判定 → 上链 + cursor+1 + 快照追加，见 resources/history/
  mod.rs 算法节）。
- history_start_recording 无独立符号（内联进调用点；start_input_recording
  0x140a86270 的 AppMode!=Replay 断言）。
"""

SECTIONS["systems/history/mod.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- add_history_systems 0x1409438a0 / insert_history_resources 0x140943e10 /
  add_history_events 0x140943f40：Bevy 注册胶水（~10 系统注册、History/ReplayBuffer
  等资源插入、HistoryCmd 事件添加；0x14362c370 静态经 FUN_140b218a0 读写）——
  按依赖过滤不计业务；AppMode::Replay = 0xd 的边界定义在此（replay_mode 0x140a7c550
  同址复核）。
"""

SECTIONS["systems/history/finish_replay.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **finish_replay 0x1408b36a0**：`*app_mode == 0xd(Replay)` 门控 → 检测
  ReplayBuffer 与待处理命令（param_9 队列）双空：
  1. pending 恰 1 条 redo → CmdUnfreezeSheep（羊冻结解除事件）；
  2. 否则 → RestoreAppTo 写回 AppMode、toolbar switch_mode（+0x80/+0x88 条件）、
     MetaCommander 顶部哨兵 log+pop（FUN_1408b2270）；
  3. 日志 "History replay has naturally finished. Unblocking game commander."
     （@0x142a9f208）/ "History replay was not active. Unblocking game commander."
     （@0x142a9f298，级别 >=4）。
"""

SECTIONS["systems/history/replay_wall.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_wall 0x140951b80**（前桶迭代骨架）：
  1. `active(+0x18)==0 → return`；speed = ReplayBuffer+0x28；
  2. 前桶 cmds（ptr@+0x08, len@+0x10）0x30 步长迭代，仅 tag 4：
     - 全局序列 FUN_1400d9b20（墙变更事件代）；
     - `count@+0x20 != 0` 循环：对元素内 24B WallReplayEditCmd 子记录（fat 指针
       {ptr@+0x08, len@+0x10}）逐个 `process 0x140abfb30(cmd, dt*speed, …)`（自递归，
       instant/动画两路径分派）；`done = 子记录+0x15` AND 聚合；非全 done → 本帧停止；
     - 全 done → **收集已改墙**：每子记录 niche 解码（+0x10 - 0x7fffffffffffffff）
       经 vtable 0x142ab4910 取目标，FUN_1402ca2f0 收集入局部 Vec；count>1 再收集
       下一条元素墙集，FUN_1408b1fb0 合并（非 0 才 count-- 消费）；
  3. 收尾：完成索引**逆序**移除（边界断言 → memmove 压缩 → 按判别字节 drop 分派：
     tag 4 → FUN_140891eb0 墙内联 drop）。
- **_deprecated_morph_curve 0x140952570**：已弃用形态曲线应用（保留兼容，无调用方
  业务路径）。
- **WallReplayEditCmd::process 0x140abfb30**：8 参自递归分派（instant →
  process_instant 按判别 cubic_in_out；动画 → process_animated）。
"""

SECTIONS["systems/history/replay_terrain.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；补强本节）

- **replay_terrain 0x140995150**（前桶迭代骨架 + 下列）：
  1. **instant 路径**（`duration(+0x10)==0.0 且非 NaN 且 undo(+0x1c)==0`）：拷贝点集
     （12B/点，断言点数 < 0x3fffff @0x142aae990）→ 更新地形 stroke 计数（+0x38 =
     max(索引+1, 原值)）→ FUN_141526d10 直接插入 TerrainEditingData（+0x8 起点）→
     释放拷贝；构造默认命令 {ptr, 0.4f, 2, stroke_id}（0x3ecccccd = 0.4f）；
  2. **动画路径**：`t = clamp(elapsed/duration, 0, 1)`，`eased = 1-(1-t)^3`
     （ease_out_cubic；0.4 为 clamp 下限 @0x1429cd060、1.0 @0x142925904）；
     `undo==0 → reapply 0x1408592d0(payload+0x10, …, eased)`，`!=0 → revert
     0x140858d30`；`eased >= 1.0` 时 +0x18 计数--，<0 → 完成入列表；
  3. `elapsed += dt*speed`（+0x10，dt = Time+0x28）；
  4. 收尾同通用骨架（逆序移除 + tag 3 → FUN_140b5c2a0 drop）。
- **TerrainReplayCmd::forward 0x140994f30 / backward 0x140995040**（同构 1533B）：
  duration = clamp(Σ点距*0.01, 0.4, 3.0)（无点 0.3）；"assertion failed:
  stroke_length.is_finite()"（terrain_edit.rs:43）；输出 {data ptr, +0x8=0,
  duration f32@+0xc, +0x10=2}。
"""

SECTIONS["systems/history/replay_stairs.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_stairs 0x140af75b0**（前桶迭代骨架 + 下列）：
  1. 命令 {tag 10, flags byte@+0x08（bit0=instant）, duration f32@+0x0c, payload@+0x10,
     elapsed f32@+0x18, undo@+0x1c}；payload+0x10 = 边记录 {edge1 u32, edge2 u32,
     标志字节}；
  2. **动画门控**：`elapsed < max(duration/speed, 下限 _UNK_142930d70)` → 仅
     `elapsed += dt` 等下一帧；满足后（或 instant）→ 处理；
  3. **处理**：`undo==0 → hashbrown insert 边入 StairsRemovedSupports`（FUN_1405b1be0
     哈希 + FUN_140b76ce0，表基 +0x20）；`!=0 → FUN_1402c8640 remove_entry(edge1,
     edge2)`；查 DecoratorStorage::lookup_decorator_info（FUN_140a1c930；type 字节
     +0x41 | 0xf8 > 0xf9 → 跳过）+ CachedDecoratorTransforms（FUN_1405b0ed0 SIMD 探测，
     键 = edge1）→ **push PlayStairsAudioCmd**（Events<PlayStairsAudioCmd>，
     {edge id, decorator info u64 对}）；
  4. 收尾同通用骨架（逆序移除 + tag 10 → default drop）。
"""

SECTIONS["systems/history/replay_auto_clutter.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_auto_clutter 0x140aa4a60**：前桶迭代骨架（speed = ReplayBuffer+0x28）；
  每命令：`undo(+0x1c)==0 → HashMap<WallId,()> insert`（墙派生装饰恢复）；
  `!=0 → DeletedAutoClutter.erase((DecoratorId, DecoratorId))`；instant/动画节奏
  （elapsed += dt*speed，完成后索引入列表）；收尾逆序移除（tag 9 → 0x1402e5d80
  drop）。
"""

SECTIONS["systems/history/replay_garden.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；补强本节）

- replay_garden = `replay_raster<GardenRasterStagingSsbo, WorldRaster<512>,
  GardenFilter>` 单态（0x1409f6ff0，67KB 重内联）——**算法全在泛型
  replay_raster<T,R,F>**（见 replay_raster.rs 算法节）；命令生成闭包宿主
  0x14143d2c0（generate_replay_cmds_from_edit garden 单态，跳表基址 0x142c5f248）。
  本包装自身无附加业务逻辑。
"""

SECTIONS["systems/history/replay_mode.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **replay_mode 0x140a7c550**：`*app_mode != 0xd(Replay) → return`（再次确认
  AppMode::Replay = 0xd）；进入/离开 log "Replay mode started"/"Replay mode
  finished"（target country_core::systems::history::replay_mode，级别 >3，
  @0x142ad7120/0x142ad7190）；RestoreAppTo::set 记录回放前 AppMode；长按快进
  （HoldToFastReplay，FUN_140c904e0 时间函数）时发 redo HistoryCmd；驱动
  ReplayModeElapsedTime（enum2$ 判别）计时；写回放状态资源（+0x28 f32 = 1.0 等）。
"""

SECTIONS["systems/history/replay_path.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **generate_path_replay_cmds 0x1416286d0**：读 [rcx] niche 判别 → `dec; cmp 4; jae`
  过滤变体 0..3（DirtPath/Cobblestone1/2/3）→ 尾调 0x1414423e0
  （generate_replay_cmds_from_edit<path closure_env$0>；round3 round2 修正原
  0x1414433xx 为 e9 rel32 化验算）。
- **replay_path** = replay_raster<Path> 领域包装（单态 0x1409febf0，DirtPathFilter；
  Cobblestone1-3 = 0x1409fc290/0x1409f1cf0/0x1409f9930）——算法在 replay_raster.rs。
"""

SECTIONS["systems/history/replay_pop_bucket.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **replay_pop_bucket 0x140b38250**（109 行全文）：
  1. `active(+0x18) != 0` 且**前桶 cmds len(+0x10) == 0** → `FUN_1409f18f0` pop_front
     取 (decorators Arc, stairs Arc)；
  2. decorators Arc 非空 → `FUN_14155ba30(storage, arc+2, …)` = 装饰 diff 恢复
     （DecoratorStorage::restore_decorators）→ LOCK dec（归零 FUN_140b5cb60 drop）；
  3. stairs Arc 非空 → 楼梯应用链：FUN_1408ff210（restore_backup）→ FUN_140900ba0
     （apply_diff）→ FUN_14157cec0(+0xb8, +0x28, +0x148) → FUN_140b50cc0(+0xb8) →
     FUN_140b50d60(+0x100) → `(+0x178)=0` → FUN_1408fefe0（
     StairsState::restore_backup/apply_diff/iter_edges_and_stubs 链）→ LOCK dec
     （归零 FUN_140b5c420 drop）；
  4. 收尾（两分支共用）：`FUN_140b2cb30(…, 1)` + `FUN_140b743e0(…, 0x70, 0x10)` +
     `FUN_1408cd5c0(*param_7)`（RaycastWorld flush / archivist extract）。
"""

SECTIONS["systems/history/replay_raster.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_raster<T,R,F> 泛型**（六单态 0x1409f1cf0 Cobblestone2 / 0x1409f4650
  Water / 0x1409f6ff0 Garden / 0x1409f9930 Cobblestone3 / 0x1409fc290 Cobblestone1 /
  0x1409febf0 Path，57-67KB 重内联）：前桶过滤本域命令 → **undo 清缓冲 + 重绘存储
  stroke，redo 画 stroke + 发绘制事件**（RasterReplayCmd 载荷驱动）；panic
  "assertion failed: !cmd.edit.stroke.points.is_empty()"（replay_raster.rs，
  @0x142ac5958）；"Tried to init CurveU from an invalid u: "（@0x142aa53c0）复用。
- **RasterReplayCmd::forward 0x141700430 / backward 0x141700620**（同构 1533B）：
  构造 {ptr, +0x8=0, duration f32@+0xc, +0x10=2}。
- **get_stroke_from_0_to_t**（内联）：按进度 t 切取 stroke 点段（与 0x140859740 的
  fold 同族）；**duration_based_on_stroke_length**（内联）：与 TerrainEdit::duration
  同公式族（clamp(Σ点距*0.01, 0.4, 3.0)）。
- **RasterReplayResources SystemParam**（0x140ae5830 get_param）：Bevy 胶水。
"""

SECTIONS["systems/history/replay_water.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；补强本节）

- replay_water = `replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, WaterFilter>`
  单态（0x1409f4650，67KB 重内联）——算法全在泛型 replay_raster<T,R,F>
  （见 replay_raster.rs 算法节）；命令生成闭包宿主 0x141444c70（water 单态，跳表
  基址 0x142c5f2fc）。本包装自身无附加业务逻辑。
"""

SECTIONS["systems/history/signifier_replay_active.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **signifier_replay_active 0x140b1b860**：ReplayModeElapsedTime（enum2$ 判别）活跃
  时累加；`Query<(&mut Transform, &SignifierReplayIndex)>` 对每实体把 Transform 朝
  光标目标做弹性抖动：`0.5 * exp2(-10t) * sin(7.853982t)`（ease_out_elastic_jiggle；
  系数来自反编译常量，精确系数以 reconstruction 的 1-(1-t)^3 族为准，U）。
- register_required_components 0x140b1d680：SignifierReplayIndex 的必需组件注册
  （Bevy 胶水）。
"""

# ---------------- session-save ----------------

SECTIONS["resources/session.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **会话目录协议**：save_folder_path 0x140b46000 = paths::saves()/uuid（Display fmt
  失败 panic "a Display implementation returned an error unexpectedly"
  @0x142aef358）；backup_folder_path 0x140b46180 = **paths::backups()/uuid（独立顶级
  目录，round2 闭合——无额外叶名）**；session_file_list 0x140b46300 = {history.json,
  screenshot.jpg, meta.json} 三叶名路径构建；loading_background_path 0x140b46a50 =
  +"loading_background.jpg"（PDB 名 screenshot_path 错位）；delete_session
  0x140b46d60 = deleted_saves()/uuid 标记文件 → rename 移动 → 两段删除（失败 log
  "Error deleting glade: " @0x142aef5a0）。
- **SessionMeta::try_load_from_path 0x140b46480**：读文件 → serde 反序列化；字段名
  {state, glade, mutators, image}（首字节分发表 @0x142aef526；字段 4 载荷 =
  serde_json::Value）；季节谓词为"存在字节匹配"扫描：is_halloween2024 = any(b==0)、
  is_halloween_any = any(b<=1)（**纠错：合同"扫描 >1"方向相反**）、is_xmas2024 =
  any(b==2)；assets_root 0x140b46370 = "assets/glade" + 主题表
  {summer/olden/flowery/autumn/winter}[+0x58]（GladeType 0=Summer 值域 0..4 闭合）；
  write_to_disk 0x140b463c0 = save_folder_path+"meta.json" + save_session_meta_to_file。
- **NextSession**：load 0x140b46920 = {cap-first 空 Vec(cap=8), kind=
  0x8000000000000002, uuid@+0x20, ignore_snapshot=false@+0x98}；create 0x140b46960 =
  96B 模板(rdx)+32B(r8) 构造（kind 由调用方模板决定——变体 1 生产者在 UI/会话流
  vtable，卡点）；ignore_snapshot 0x140b46a20 = 写 +0x98 + memcpy 0xa0（builder）；
  append_on_load<MetaCommandFence> 0x141328ac0 = push 0x20B {u32 tag=0, u32
  0x80000000, u64 payload} + memcpy 0xa0（on_load 列表 fence =
  0x8000000000000003，load_game 回放终止条件）。
"""

SECTIONS["resources/loading_screen.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **startup_loading_screen 0x140a1a1e0**：once 初始化 → load_mesh_into_library
  ("loading_screen", hash 0x42ac8c61) + load_shader_into_library
  ("loading_screen/loading_screen.raster.hlsl") → spawn 0x88B bundle
  {DrawableMeshBundle(vt 0x1412da5d0), +0x48=0x12 渲染 pass, +0x58=0 DontCastShadow,
  单位变换}。
- **update_loading_screen 0x140a1a190**：以占位 UUID **33333333-3333-3333-3333-
  333333333333**（@0x1429503e0）为键对两张 hashbrown 表（+0x70/+0xd8）做插入/删除
  （0x80 控制字节/0xff 墓碑，0xd0B 值条目，progress u32@+0x58 源 @+0x5b0，rehash
  0x140d3fa40）——LoadingScreenProgress 资源内的按会话/资产 16B id 索引的加载进度
  条目；占位 UUID = "无会话"（boot 屏）固定键（round2 闭合）。业务增删时机 U。
"""

SECTIONS["resources/logo_animation.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **LogoAnimation 布局**：5 阶段进度 f32@+0x00..+0x14 + 5 阶段时长 f32@+0x14..+0x28；
  is_waiting = (+0x0c) > 0.0（0x140b19d10）；is_finished = (+0x10) >= 1.0（0x140b19d22，
  常量 @0x142ae8b10）；finish_wait = (+0x0c) = 1.0f（0x140b19d33）；play 0x140b19d00 =
  清零进度区；build 0x140b19d40 = insert_resource + Startup + Update 注册（默认
  durations {const×4, 0.5f}）。
- **animate_logo 0x140b1a6b0**（Update 系统 (ResMut<LogoAnimation>, Res<Time>,
  Res<AudioCore>)）：顺序找第一个 progress<1.0 的阶段 i → `stage[i] = clamp(stage[i]
  + dt/duration[i], 0, 1)`；**阶段 0 起始（stage[0]==0.0）播 FMOD
  event:/UI/Logo/Scribble、阶段 2 起始播 event:/UI/Logo/Branches**（串
  @0x142ae945b/0x142ae9472，代码内联拼 23 字符事件名）。
- startup_animated_logo 0x140b19d90：load tiny_glade_logo/logo_branches JSON 网格 +
  logo_animation.raster.hlsl/logo_branches_animation.raster.hlsl + spawn。
"""

SECTIONS["resources/prefab.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **PrefabLibrary（hashbrown HashMap<PrefabName, Prefab>）**：桶 0x28B = {hash u64,
  PrefabName 16B, **Prefab 值 16B {ptr,len} 胖指针类**}（0x141552d70 get / 0x141552ff0
  has：FUN_1405b1a70 哈希 + SIMD group probe + memcmp 0x142923bf0）——
  reconstruction 的 `Prefab { parts: Vec<PrefabPart> }`（24B）布局存疑（DIGEST #16）。
- **add 0x1408e2540**：HashMap::insert（被替换旧值逐 PrefabPart drop）。
- register_required_components 0x1408e5d10：Bevy 组件注册胶水（非业务）。
"""

SECTIONS["systems/prefab.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **spawn_prefabs 0x140840610**：事件消费 → PrefabLibrary 查找（缺失 panic
  prefab.rs @0x142a94158）→ 实例创建。
- **create_prefab_rt 0x140841550**：Prefab → 位置查表（@0x142a94358 按变体）→
  四元数旋转矩阵 + 缩放 → GPU 实例缓冲 push（FUN_1415634c0）+ 打包颜色 u32
  (b<<16|g<<8|r) → 条件 spawn（+0x84 标志字节：&1 spawn、==2 跳过）。
- **create_prefab_audio_trigger 0x140841b00**：查 audio_trigger_library（0x140a382d0）
  → 注册/生成（0x1408b9f20）。
- **update_prefabs 0x140841d60**：GPU 实例数据刷新（触 audio_trigger_library 路径）。
- **despawn_prefabs 0x140842d60**：逐条目（0x58 步长）despawn_prefab_contents
  （0x140843390）→ Commands::despawn（0x140d14770）。
- **hot_reload_prefabs 0x140843740**：watch 事件 → 加载 JSON 资产（0x140b712b0 族）
  → despawn_prefab_contents → create_prefab_audio_trigger → create_prefab_rt
  全量重建实例链。
- **clone_behavior 0x140002370**：`xor eax,eax; ret`（返回 0/false，O）。
"""

SECTIONS["systems/save/mod.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- add_save_load_systems 0x1409443b0 = 注册 save_game/load_game/save_thread/
  check_load_completion 等系统；add_save_load_events_and_resources 0x1409447e0 =
  注册事件/资源（SaveGameCmd/SavingGameComplete/LoadingGameComplete 等）——Bevy
  注册胶水，按依赖过滤不计业务（系统本体算法见各章 + ALGORITHMS.md）。
"""

SECTIONS["systems/save/save_game.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **save_game 0x140a3f470**（6 段 SystemParam，行 ~507-591 内联节拍器）：
  1. **备份节拍器**：{counter u32@+0x140, 累计 f32@+0x144 clamp[0,1], 阈值
     f32@+0x148}；仅当阈值 < 60.0（或 NaN）累计（断言 delta>=0.0
     @0x142a9f2c0）；累计 >= 1.0 → counter++ 并扫**除数桶 {300,600,3600,7200}**
     （@0x142a9f348，从大到小整除匹配），命中后 counter -= 除数，**可一次消耗多个
     间隔**（内层按升序重新校验，桶内容为 0 → panic 除零）；
  2. 触发备份：log "Capturing a game snapshot" → capture_game_snapshot_save_data
     0x1417a0430 → Arc 克隆 → History::set_current_snapshot 0x140a84760 →
     FUN_1408da280 枚举分发发送 AsyncSaveCmd::CaptureSnapshot(9)；
  3. 同步保存：GameSaveData 组装（~9 资源 +0x28..+0x100 密集字段）→
     save_game_to_file；错误串 "Error while performing a synchronous save: " /
     "Sync response from save thread" / "Failed writing session meta: "。
- **save_game_to_file<Cursor 0x141750c90 / File 0x140241ca0>**：**手工拼 JSON**——
  写 '{' 0x7b、`"History":{` 字面量、edit 间 `",\n"` 0xa2c；节名走 json_key
  （0x140a43810 双查表 @0x142ace328 len + @0x142ace378 ptr）；padding @0x142ace3c8
  = [0,16,0,128,32,128,64,8,8,2048]（10 节协议：Version/Identity/BackwardCompatibility/
  Camera/TimeOfDay/Sheep/Colors/chain_index/history_start/snapshot）。
- **write_json_sections<Cursor 0x141751c90 / File 0x140242b40>**：遍历 0x28B 节条目：
  json_key（FUN_140d79f90）+ 载荷 to_json_string（FUN_140b883f0）。
- **ensure_current_snapshot 0x140a439b0**（= add_history_edit_to_save_file）：
  1) `offset = 影子文件+0x38`（追加偏移）→ push 进 edit_offsets 列表；2) 零填充扩容 +
   写 u16 0x0a2c (",\n") + mark_dirty(offset, offset+2)；3) alloc 0x80B 缓冲 →
   **MultiEdit::serialize**（FUN_140b39b80，h63c9 单态）→ 扩容 + memcpy + 更新 len +
   mark_dirty(旧端, 新端)。**增量写协议实锤**（round2 CLOSURE 的"History+0x38 快照
   id"描述修正为影子缓冲追加偏移）。
- **save_session_meta_to_file 0x1417522e0**：SessionMeta serialize → File::create
  （FUN_140d59dc0）→ write_all → sync → 错误日志族。
"""

SECTIONS["systems/save/save_thread.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **save_thread 0x141720680 主循环**：
  1. 每 ≥1s tick（time 基线比较）：state!=3 且脏列表非空 → 第一个脏条目
     ShadowFile::sync_all（0x14091a230）→ 结果组装 0x40B 条目（FUN_140d61b00 +
     FUN_141720490）→ 失败列表（grow FUN_1405627b0）；
  2. mutex + condvar 等待 ≤1s（FUN_1428cd210, 1e9ns）→ UNLOCK；
  3. 失败列表 diff（0x40B 条目逐字段：结构比较 + +0x20 int + +0x38 qword + +0x30
     memcmp）→ 新列表空：log "Auto-save I/O error resolved"（级别>2）；非空：log
     "Save thread I/O error; affected file count: "（级别!=0）→ FUN_1412f86a0 +
     FUN_1408da430 经 channel 回传游戏线程；
  4. state!=3 → drop 当前存档（枚举 0/1/2 → FUN_14167c600/1416777a0/1416d6a60）；
  5. 断言 locked_backups.remove(&path) 成功（panic @0x142cca978）；
  6. recv 命令（FUN_14152d950）→ 解码（0/1/2 → FUN_14167c8a0/141677330/1416d64e0）；
  7. **0xb Shutdown**：需 flush → state=3 → CurrentSaveFile::flush_deferred_and_sync
     0x14171f7f0（失败 warn "Error flushing save file on game thread disconnect."
     @0x142ccaa60）→ channel 发回确认（FUN_140052a30）→ 清理 → return 退出线程；
     否则 state==3 → 睡 10ms 继续；state!=3 → 直接 return；
  8. **3 OpenFile 前置**：save_folder_path + join("history.json") →
     UnsavedFileList::contains_file 0x1417201a0 → 命中 → panic "Was asked to load an
     unsaved file"（@0x142ccaa00）；
  9. execute_save_cmd 0x141721aa0。
- **execute_save_cmd**（switch {3,4,5,6,7,9,10}+default；公共断言 = truncate <=
  history_edit_offsets.len()，@0x142cca478）：
  - 3 OpenFile：debug log "AsyncSaveCmd::OpenFile(" → 旧态 flush（失败 warn "Flush
    error while closing previous save file (upon open request)" @0x142ccaf00）→
    UnsavedFileList::add → save_folder_path+join("snapshots") → **save_game_to_file
    <Cursor> 全量重写**（失败 log "Rewriting history file" @0x142ccaf31）→
    ShadowFile::create("history.json") → 0x270B 状态组装；
  - 4 CloseFile / 5 ChangeSection / 6 ReplaceEdits：**state==3（无活动档）→ panic
    "No active save file, but AsyncSaveCmd::X has been issued"**；5 改节 + mark_dirty
    ×2（0x14171ece0 互证）；6 逐 edit 追加（"Edit appended to save file"
    @0x142cca538 / "Error appending edit to save file: " @0x142cca5b0）；
  - 7 BackupSave：`history-auto` 前缀 + 备份限额（"Backup limit reached; removing " /
    "Error deleting stale backup " / "Error reading backup file list: "）；
  - 9 CaptureSnapshot：创建 `snapshots` 目录（"creating snapshot folder"）→ 快照
    JSON 落盘；10 Flush：flush 当前档（"Error flushing changes to disk"）；
  - default（含 8=ManualBackup）："Copying inactive save file to backup" /
    "Writing backup from shadow copy" / "Error flushing history file for backup"；
    按载荷比对文件身份字节（+0x53/+0x299）与 "snapshot"（0x746f687370616e73）；
  - 0/1/2 无专属 case（I：死/历史变体）。
- **snapshot_gc_thread 0x14172b280**：parse_save_file_snapshots（0x141286730）→
  snapshots 目录比对 → "Deleting unreferenced snapshot "（@0x142ccb088/0x142ccb0e0）
  → 结果经 channel（FUN_1408da040）；竞态守卫 "Save file changed before snapshot GC
  could finish. Shutting down the GC thread"；错误 "Error opening backup file " /
  "Error parsing history snapshots fromfile"。
"""

SECTIONS["systems/save/shadow_file.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **ShadowFile 布局**（本域机器码新证）：数据 Vec cap-first {cap@+0x20, ptr@+0x28,
  len@+0x30}、追加偏移@+0x38、文件状态@+0x40（i64::MIN 哨兵 = Some/已开）、句柄
  @+0x48、脏位图 ptr@+0x60、位计数@+0x68、flushed_block_count@+0x78。
- **flush 0x1409193d0**（unwrap_open_mut 同址）：未开 → 建 `<path>.tmp`（.tmp
  @0x142aaca18）→ 写数据（"Write data"）→ "Setting file length" → "Sync data" →
  log "Save file successfully opened and written to disk"；错误 "Could not create
  temp file for save rewrite: " / "Error writing save data to temp file: "。
- **sync_all 0x14091a230**（flush_impl）：先 flush → 断言 +0x40 < 0（否则 panic
  "ShadowBackendFile is not open" @0x142aace80）→ OS sync → "Asking the OS to sync
  writes to disk"（@0x142aad1d8）→ flushed_block_count@+0x78 → 返回 ok-bool。
- **mark_dirty 0x14091a450**（脏区间协议）：`end <= start → return`；**4KB 块**：
  start_block = start>>12、end_block = ceil(end/4096)；位图不足 → grow
  （FUN_140915c20 @+0x60）；越界 → 格式化 panic @0x142aabe08；置位 [start, end)；
  debug log "Marking block range dirty: .."（级别>4，@0x142aad220）。
  **DIGEST 的"8 槽 &7"修正为 4KB 块位图**（&7 是字内位偏移运算）。
- create 0x1409184d0 = 未开状态初始化（"ShadowBackendFile is not open" 前置语义）。
"""

SECTIONS["systems/save/snapshot.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **capture_game_snapshot_save_data 0x1417a0430**：串匹配 @0x142cdc3b0（0x46 字，
  panic @0x142cdc480/0x142cdc498）+ 路径 join（FUN_140c16d40）→ 从 ~9 组世界数据
  （FUN_141305170/141308b40/141309840 收集器）构建 ~0x400B 快照结构（JSON 键串表
  @0x142a93bfe 起）→ 返回 **Arc**（alloc 0x18 {strong=1,weak=1} + LOCK inc）。
- **capture_game_snapshot 0x1417a1d00**（184KB 系统体）：全 Query 串化；已知结构
  StairsSnapshot{edges, checksum} / StairsRemovedSupports{edges} /
  TerrainEditingData{strokes, next_stroke_id, next_stroke_index} /
  DecoratorStorageSnapshot{states}。
- **validate_and_load_closest_snapshot_data 0x1417a6e50**（快照 JSON 解析器）：
  按节名键分派（FUN_1414744d0 匹配 + 各节跳表 @0x142cdca80/0x142cdca98/0x142cdcab0）：
  **节序 Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain
  → Trees → Raster**；每节 log "parsing X"（@0x142cdc709 块）；Version 不符 → panic
  "Version mismatch; expected , got "（@0x142cdc840）；Terrain 数据 **flate2 deflate
  解压**（"decompressing terrain data" @0x142cdc800，flate2-1.0.24）；Raster 键白名单
  外 → "Ignoring unknown raster data in snapshot: "（@0x142cdc7c0）。
- **init_game_state_from_snapshot 0x1417a9c50**：load_game 拷贝 0x1b0B 解析数据 →
  应用各域资源（颜色/时间/羊/地形…）。
"""

SECTIONS["systems/save/snapshot_gc.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **parse_save_file_snapshots<BufReader<File>> 0x141286730**：打开存档文件（失败 →
  "Error opening backup file " / "Error parsing history snapshots fromfile" 族）→
  解析 `snapshot` 节内的快照 id 引用集合（HistorySnapshotId 列表），供
  snapshot_gc_thread 判定未引用快照（消费点 = 0x14172b280 行 ~165）。
"""

SECTIONS["systems/save/load_game.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **load_game 0x14146d030**（13+ 段 SystemParam）：
  1. 门控 `*app_mode == 0xe`（LoadGameData 态）→ 否则 return；写 loading.tmp；
  2. **状态字节 +9**：0 → 按 +0x4c 计数（可负）发 16B 加载进度记录 {计数器, tag
     0x3010001/0x3010101} 入 Vec<16B>，置 2，return（多帧节拍）；2 → 主流程；其他
     → panic @0x142c641d0；
  3. **NextSessionKind 解码**：`decoded = kind ^ 0x8000000000000000; sel =
     min(decoded, 2)`；==1 → parse_saved_game_data（FUN_141470d00 内联：节序
     "Making save file backup for upgrade"→"Reading file data"→"Parsing JSON"→
     "Version"→"BackwardCompatibility"→"Camera"→"parsing camera"→"Time of day"
     （缺失 → "This save file had no 'Time of day' data"）；失败 → "Couldn't load
     save file from "）；==2 → on_load 命令回放 + 内存快照路径；
  4. **MetaCommand 回放**（on_load 4B 条目，fence 0x8000000000000003 终止）：变体 0
     → 写 manager 状态 + push 24B 事件；变体 1 → FUN_140b5a180 push MetaCommand
     （meta_commander.rs @0x142c5ae38）；变体 2+ → Arc LOCK dec drop；
  5. **PostGameLoadCommand 回放**（第二轮遍历）：变体 0（SwitchAppMode）→ log
     "Ignoring PostGameLoadCommand::SwitchAppMode"（@0x142c642e0）跳过；变体 1 →
     push 24B {tag, 载荷×2}；断言 "Command skipping must not be used together with
     snapshots" / "Can't skip "（@0x142c64378/0x142c643d8）→ FUN_1412e0b90 应用；
  6. **版本 → 全局开关**：`0x14362c308 = (v > 0x19)`；`0x14362c011 = (v > 0x1a)`
     （后者 = **history 域楼梯 checksum 校验开关**，跨域互证）；
  7. **旧版颜色 ID 升级**（"Detected old version of window color IDs. Upgrading..." /
     "Detected old version of arrow slits color IDs. Upgrading..."）：+0x52 ∈
     {0x38..0x3a} 的 0x58 步长记录，+0x30 字段按 {0,3,6,7→原值; 1→2; 2→5; 4→8;
     5→3; 8→4; 999→999; default→1} 改写；
  8. **extract_max_wall_id_from_history**：遍历 MultiEdit 取最大 wall id → 计数器
     资源 +0xc0；
  9. **init_game_state_from_snapshot**（FUN_1417a9c50）：拷贝 0x1b0B 数据应用 →
     清理（快照 Arc 列表 dec 循环）；事件 LoadingGameComplete/StartupReveal…；
     "Cloning game state for save rewrite took "（@0x142c64b48）。
"""

SECTIONS["systems/save/save_backup_list.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **BackupSaveFileKind::is_corrupted 0x140ae8320**：`tag == 4`（机器码；batch PDB 名
  is_auto 错位）；iter_backup_files 0x140ae8330 = 迭代备份文件。
- **new_from_session 0x140ae8470**：扫备份目录（`history-auto` 前缀）建列表；错误
  "Error reading save backup directory" / "Error creating backup save folder: "
  （@0x142ae367c）。
"""

SECTIONS["systems/save/save_backup_metronome.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖；内联于 save_game 0x140a3f470 行 ~507-591）

- **字段**：{counter u32@+0x140, 累计 f32@+0x144, 阈值 f32@+0x148}。
- **算法**：
  1. 仅当 `阈值 < 60.0`（或 NaN）时累计：断言 `delta >= 0.0`（panic @0x142a9f2c0）
     → `累计 += delta` → clamp [0, 1]；
  2. `累计 >= 1.0` → **counter++ 并扫除数桶 {300, 600, 3600, 7200}**
     （@0x142a9f348 起；counter % 除数从大到小匹配）→ `累计 -= 1.0`；
  3. 命中后 `counter -= 除数`（可一次消耗多个间隔：内层循环按升序 {300..7200}
     重新校验整除，桶内容为 0 → panic 除零 @0x142c6e9d8）；
  4. 备份触发标志 → save_game 捕获流程（"Capturing a game snapshot"）。
- 档位语义：counter 以"1.0 累计"为步进 ≈ 活跃秒数；300=5min、600=10min、3600=1h、
  7200=2h。
"""

SECTIONS["systems/save/save_file_identity.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **try_approx_parse_save_file_identity 0x141e58f20**：alloc 0x800B 缓冲 → Windows
  ReadFile → UTF-8 转换 → 搜索字面量 `"Identity"`（**带引号**，@0x142f92a40 len 10）
  → 解析其后 JSON 值为 SaveFileIdentity（与 SaveFileSection::json_key "Identity"
  的 len 8 变体互为印证）。
- **derive_fallback_save_file_identity 0x141673140**：解析失败回退：SystemTime 取时 +
  FUN_14145aa70 二次尝试 + 文件元数据派生（字段序 U）。
"""

SECTIONS["systems/save/load_completion_checker.rs.md"] = """## 实现方式（算法级，2026-10-15 深挖）

- **check_load_completion 0x140a9f810**：save_folder_path + join + FUN_140d69590 +
  File::create×2（文件存在性/就绪探测；精确条件 I）。
- **is_session_ok_to_load 0x140a9fa80**：save_folder_path×2 + join×2 + File::create×4
  （会话文件就绪检查）。
- SessionLoadReadiness::is_ok 0x140833230（is_initial 折叠）；deny_load 0x140a9fa70
  （tag==1 判定，与 GladeType::is_olden 同址折叠）。
"""

# ============ 写入 ============

def main():
    import os
    for rel, section in SECTIONS.items():
        path = os.path.join(BASE, rel)
        if not os.path.exists(path):
            print("MISSING:", path); continue
        with io.open(path, "r", encoding="utf-8") as f:
            text = f.read()
        marker = "<!-- 未覆盖函数数：0 -->"
        block = "\n" + section.rstrip() + "\n"
        if "实现方式（算法级" in text:
            print("SKIP (already has):", rel); continue
        if marker in text:
            text = text.replace(marker, marker + block)
        else:
            text = text.rstrip() + "\n" + block
        with io.open(path, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        print("OK:", rel)

if __name__ == "__main__":
    main()
