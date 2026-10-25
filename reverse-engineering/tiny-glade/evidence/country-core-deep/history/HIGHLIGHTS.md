# history 域 HIGHLIGHTS（2026-10-14，agent tg_a1）

1. **sysres-procedures.json 在本域有 11 处名/址系统性错位，S_PUB32 是唯一权威名表。**
   最重一例：任务指定的“closest_history_snapshot 0x140b36460（75KB 快照解析/筛选系统）”
   实为 `<MultiEdit as Deserialize>::deserialize`（serde 反序列化巨函数；
   0x140b365e0/0x140b36840 是其 visit_seq/visit_map，Ghidra 未切开边界）。
   其余错位见 DIGEST §7。

2. **add_decorator_diff 0x140a854a0 完整流程已 O 级恢复**（4 参签名）：
   pending 未录制 panic → （非空路径）nuke 判定 + 重建 pending 桶 → 楼梯图
   checksum 校验（全局开关 0x14362c011）→ 两个 u32 边对循环（hashbrown remap
   查表后 push tag 0xE/tag 1 的 24B HistoryEdit）→ 用 diff/stairs 重建 64B/96B
   表并 Arc 替换 pending.decorators/stairs。**reconstruction 的“空 diff 直接
   return”是错的**：空 decorator diff 仍走 stairs 边记录与 Arc 替换。

3. **history_process 0x140af5b30 第二事件流坐实**：`Local<双游标>` 读两条 16B
   记录流；record byte[+9] 经跳表 0x142ae5420 分派（0x3 =
   ReplayBuffer::make_animations_instant、0x13 = 弹 0x30 记录…）；undo 且
   cmds>1 时**命令 Vec 整体反转**；push 桶后按 ReplayCmd 判别经跳表
   0x142ae545c 逐条去重校验；录制中 log "Can't process History while
   recording a new edit is active!"。ReplayBuffer 静态实例 = 0x142ae5430。

4. **0x140b3f910 双符号同址（ICF）**：`is_default_lantern_brightness` =
   `is_default_stair_width` = `x == 1.0f`（cmpeqss 对静态 0x142925904）。
   sysres 给 0x140b3f930 的“is_default_lantern_brightness”名错——那是
   `<DecoratorHistoryState as From<&…>>`（type 字节@+0x41==8 → panic
   "Attempting to capture a DecoratorHistoryState of a preview "）。

5. **TerrainEdit 三函数纠错**：0x140858d30 = revert（0.99f 动画进度阈值 +
   panic "Requested terrain stroke {id}" + hashbrown 48B 元素表清 active 标志）、
   0x1408592d0 = reapply、0x140858c80 = Debug::fmt；duration 公式
   `clamp(Σ点距*0.01, 0.4, 3.0)`（空 0.3，"stroke_length.is_finite()" 断言）
   实现在 `TerrainReplayCmd::forward` 0x140994f30 内联（0.01 = cm→m）。

6. **墙历史链坐实**：ev_height_changed 0x140a83f80（0x30B 小函数，机器码全解）
   = new_height_changed（0x140a83fb0，写 24B 记录 {1,1,niche+2@+0x10,wall,h0,h1}
   + 0x108B blob + FUN_141696f50 校验）→ History::add(tag 7)。add 第三参数 =
   **track-caller 文件路径 &'static str**。ev_new_wall 断言
   "new_wall.curve.length_ws() != 0.0"。CompoundEditBuilder::build 真址
   0x140a82d50（非任务给的 0x1414022c0，那是 serde Deserialize 族）。

7. **generate_replay_cmds_from_edit ×4（0x14143d2c0 等）= 316B 跳板**：按 tag 经
   跳表 0x142c5f248 尾调共享核心；generate_path_replay_cmds 0x1416286d0 同理
   （niche 判别 dec/cmp 4 过滤后尾跳 0x1414433xx）——故 Ghidra batch 报
   NotFound，机器码已解。replay_raster 六单态共享 panic
   "!cmd.edit.stroke.points.is_empty()"。

8. **replay_pop_bucket 0x140b38250 全解**：前桶 cmds 空 → pop_front 取
   (decorators Arc, stairs Arc) → FUN_14155ba30 装饰恢复 + 楼梯应用链
   （FUN_1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0）
   → Arc drop → flush/archivist 收尾。ReplayBucket 0x40B =
   {decorators@+0x00, stairs@+0x08, cmds Vec@+0x10..+0x28}。

9. **进程全局族语义线索**（配合 audio-misc 域）：0x14362c420 = **panic-in-
   progress 门控**（finish_potential_mutation 的 token Drop 守卫前
   `&0x7fffffffffffffff != 0 → 跳过 panic`，对应 `std::thread::panicking()`）；
   0x14362c3a8 = **全局日志级别**（历史域四个日志点阈值 !=0 / >1 / >3 / >=4
   递增）；0x14362c011 = 楼梯 checksum 校验开关。

10. **MultiEdit::deserialize（0x140b36460）的字段名证据**给出快照负载线索：
    chain_index/history_start/edit_chain/snapshots、snapshot_id、
    **time_of_day/sun_xz_rotation/latitude_degrees（快照负载含太阳环境状态）**、
    wall_index/roof_shape_params/bend_vector/pad0/custom0/custom1、
    StairsRemovedSupports/edges。快照真正主场在 save::snapshot 域
    （validate_and_load_closest_snapshot_data 0x1417a6e50 等，已登记跨域）。

## 算法级深挖（2026-10-15）

- history_process 0x140af5b30 42KB 全文解码：HistoryCmd byte[1]==4||byte[0]==2=Interrupt；
  undo 反转 = 0x30 元素两两交换；第二事件流消费 = 清 Events+计数器 push；每条记录
  追加 Vec<16B> {计数器, byte[+0xb]} 命令日志。
- add_decorator_diff 0x140a854a0 全文：两循环均 push tag 0xE（type 0/1 区分移除/新增，
  修正早前 tag 1 结论）；checksum 扫描比对 stairs 表（非装饰表）；尾部 Arc 替换
  = 表体整体后移 0x10 重包 Arc{1,1}。
- ReplayBucket 布局修正：cmds Vec cap-first @+0x00（replay_wall/history_process 直读）。
- History::add tag-7 校验三串新读（曲线无效/None CreateWall/DeleteWall）；nuke =
  truncate+clamp count+retain snapshots+无条件重随机 nonce。
- 快照二分族统一算法：partition_point + 距离平局（|prev-idx| vs |next-idx|，平局取后）；
  current_snapshot 仅 idx==cursor 返回；set_current_snapshot 断言不替换+有序插入。
- 0x14362c011（楼梯 checksum 开关）= load_game 版本门控（v > 0x1a）——跨域闭合。
- 交付：history/ALGORITHMS.md（处理链图/逐函数算法/状态机/常量表）+ 27 章就地升级。
