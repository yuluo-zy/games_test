# RE-ANALYSIS — systems/audio/music（播放器随机队列 + 更新算法）

分析师: reanalyst-state
日期: 2026-10-15
范围: DISPATCH 组 state 的 music 缺口——播放器拥有状态、随机队列算法、更新算法、会话元数据消费。
证据 = fresh 反编译（audio-misc/decomp 既有）+ RNE 原字节（FUN_1408b70d0/14132b1a0 手解）。

---

## 1. 重大修正：switch_track 0x1408b70d0 = **洗牌弹出器**（非顺序推进）

- **旧结论（music_player.rs.md 章 / audio-misc ALGORITHMS）**：「switch_track = 推进到下一个
  [+0]==1 曲目（顺序）+ start/stop」。→ **错误**。
- **新证据（RNE 原字节 0x1408b70d0..0x1408b72e0 + fresh decomp）**：
  FUN_1408b70d0 是 **generate_track_playlist（内联）+ 弹出一条曲目索引**：
  1. `+0x110`（剩余数）== 0 → 重建：`+0xf8`（曲目数 N）→ 分配 N×8B → 身份序列 0..N-1
     （`movdqa` 常量 + `paddq` 向量化，或标量 `[ptr+i*8]=i`）→ `FUN_14132b1a0(…, &Vec)` 洗牌 →
     **不重复守卫**：`if (洗牌后首元素 == +0x08 当前曲目索引) { 首元素 <-> element[count&~1] }`
     → 存 `+0x100`=N、`+0x108`=ptr、`+0x110`=N。
  2. 弹尾：`+0x110 = N-1`；`rax = ptr[N-1]`（弹出一条索引）；`+0x00 = 1`（有曲目标志）、`+0x08 = rax`。
  3. 空曲目（N==0）→ `+0x00 = 0`、`+0x08 = 0`。
  **FMOD start/stop 不在本函数**——由调用方 update 完成（见 §2）。
- **随机队列算法**：Fisher-Yates 洗牌（`FUN_14132b1a0`，入口 ≈0x14132b195，RNG 全局
  ≈0x142a9f4xx）。**不重复**=洗牌后当前曲目不位于队首（弹尾顺序下被延到最晚播放）。
- **MusicPlayer 扩展布局（O）**：`+0x00` 有曲目标志 / `+0x08` 当前曲目索引 / `+0x10`(update §3 判定) /
  `+0xf0` 曲目 Vec ptr / `+0xf8` 曲目数 / `+0x100` 播放列表长度 / `+0x108` 播放列表 ptr /
  `+0x110` 播放列表剩余 / `+0x118` state / `+0x11c` time_acc / `+0x120` Duration(i64) / `+0x128` last_position。
  曲目元素 0x30B：`[+0]=1` 使能、`[+8]` FMOD 事件句柄。

---

## 2. update 0x1408b5500（核对 + 补：probabilistic_switch_track 定位）

既有 ALGORITHMS §1.1 状态机（A1-A4/B1-B4 双跳表 0x142a9fa24/0x142a9fa34、time_acc+=dt、
节拍同步 0x143634c68==1 && 0x143634c90&3==0→state=4+start、每帧清 0x143634c90）**维持 O**。
本批补充（fresh decomp 逐行核对）：

- §0 `get_timeline_position`：`param_1[0x1e](+0xf0)+param_1[1](+0x08)*0x30` 取当前曲目，
  仅 `[+0]==1` 使能时读位置（`FUN_1414375b0`，Ok 判别 `-0x7ffffffffffffffc`），失败
  `FUN_140a526b0` 处理 → `iVar6 = max(pos,0)`。
- §3（state==0）：`FUN_1408b70d0(param_1)` 弹下一条曲目（§1）→ `if (flag==0 || index>=count)`
  verbosity>4 log「snapshot:/Track Switching」（@0x142a9f5d0，参数名 @0x142a9f568）→ return；
  否则**主事件启动**（`param_1[8](+0x40)==1` → `FUN_1414372d0(param_1+9)` 即 start(main)；
  失败 trace @0x142ad2bc0）或**曲目启动**（`piVar7[+0]==1` → `FUN_1414372d0(piVar7+2)`；
  失败 trace @0x142ad2c50）→ 再按 playback_state（`FUN_141436fe0`）`set_paused(0)` 恢复
  （失败链 panic @0x142ad2c68）→ `param_1[0x23](+0x118)=1`（state=1）。
- **probabilistic_switch_track 定位**：PDB 内联宿主 = update（skeleton module=491 inline 18232）。
  机器码中无独立随机数调用——「概率性」来自 **播放列表洗牌（§1 随机队列）**，非显式概率阈值。
  **sysres-procedures-v3.json 把 probabilistic_switch_track 映射到 0x1408b7620 为 PDB 错位**：
  0x1408b7620 实际机器码 = get_timeline_beat_properties（§4）。

---

## 3. load_tracks 0x1408b6b10（O，fresh）

- 输入：曲目名切片（元素 0x18B {ptr,len} = &str/String cap-first）。
- 先 drop 现有曲目 Vec（+0xf0/+0xf8，元素 0x30B，`FUN_14089a280` drop 每个 AudioInstance + free 名字 String）
  与 +0xd8/+0xe0 事件注册 Vec（元素 0x18B，含名字 free）。
- 逐名：`FUN_14157c850(&result, &name, &0x1408b7620, 0x1000)` = **instantiate_audio(name,
  回调=0x1408b7620, flags=0x1000)** → 成功判别 `0x8000000000000004` → 追加到曲目 Vec
  （`FUN_1405629f0` 扩容，元素 0x30B：`+0x00` 实例判别、`+0x10/+0x20` 字段）+
  +0xd8 事件注册 Vec（`FUN_140672ed0`，元素 0x18B）。
- 结束置 `param_1+0x118 = 0`（state=0 → 下一帧 switch_track 播放首曲）。
- **曲目名串**（调用方 run_game_thread/load_glade_audio 传入；本章曲库装载即本函数）——
  `load_glade_audio` 侧串证据见 startup/audio 域。

---

## 4. get_timeline_beat_properties 0x1408b7620（核对 + 角色澄清）

- 机器码（audio-misc/decomp + 手解）：非 getter——**FMOD 事件回调**，把事件时间线节拍属性
  {i32,i32,f32,i32,i32} 写入 6 个进程全局 0x143634c68/c70(f32)/c78/c80/c88/c90
  （c88/c90 = running max；c90 = update 每帧清零的节拍计数）。
- 本批新证：0x1408b7620 作为**回调函数指针**传入 `FUN_14157c850`（load_tracks §3）——
  即每个曲目实例都注册该回调 → 播放时 FMOD 回调节拍属性 → update 读取做节拍同步（state A3）。
- **PDB 名冲突登记**：sysres-v3 `probabilistic_switch_track` @0x1408b7620 = **错位**；
  0x1408b7620 机器码 = beat-properties 发布器（章节「get_timeline_beat_properties」结论成立）。
  重建签名 `->(i32,f32)` 与机器码不符（回调形态，见 audio-misc ALGORITHMS §2.2）。

---

## 5. music_system（核对，无新增）

- `music_audio_system 0x140a72810`（O）：richness 三档 snap-off 0.24/0.49/0.74 + time_continuous_lerp
  （0x140c904e0）→ State+0x1c0；NewGladeTime clamp(time+State+0x1c8, 0, 0.8)；`(AppMode|8)==0xa`
  → MusicPlayer::update 否则 pause。**会话元数据消费**：music_system 消费 ActiveSession 主题
  （`[arg1]+0==0x13` 强制 richness=1.0 等）——ActiveSession 已接（README）。
- `music_audio_init_system 0x140a71f20`：GladeAudio::default → 3×instantiate_audio
  （mus_GladePreview×2 + Amb/GladePreview）→ MusicPlayer::init → insert_resource<MusicAudioState>。
- `music_debug_gui 0x14083c4a0`（O）：egui 窗读 [+0x100] 开关 + MusicPlayer+0x1c0 richness。

---

## 6. 对 restorer 的可用产出

- **MusicPlayer 结构**（字段 + 偏移表见 §1）+ 播放列表字段 {+0x100 count, +0x108 ptr, +0x110 remaining}。
- **随机队列算法**：身份填充 → Fisher-Yates 洗牌（RNG 全局 ≈0x142a9f4xx）→ 不重复守卫 → 弹尾。
  可直接落 Rust：`fn switch_track(&mut self) { if self.playlist_remaining==0 { self.regenerate_playlist(); } ... }`。
- **节拍同步**：回调 0x1408b7620 → 6 全局 → update A3 条件 `c68==1 && c90&3==0 → state=4+start`。
- **修正项**：章节「switch_track 顺序推进」→ 洗牌弹出；sysres probabilistic_switch_track→0x1408b7620 错位。

---

## 7. 未决疑问

1. 不重复守卫的交换目标索引 `count & ~1`（偶数 count 越界疑点）——需对该指令段再做字节级复核
   （可能 Ghidra 对 `lea`/`and` 解码伪影）。
2. 曲目名串完整清单（load_tracks 输入源在 run_game_thread/load_glade_audio，串证据归 startup 域）。
3. update 跳表 A2 的 Duration round-to-even 超时推进细节（0x1408b5b52 后续动作块）——已有字节级
   结论，字段级语义可进一步展开。
4. `+0x10` 字段语义（update §3 主事件 fallback 判定）。
