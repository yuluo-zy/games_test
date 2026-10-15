# ivy 剩余业务契约 SPEC（A 植被历史分析，2026-10-15）

模块文件：`resources/ivy/*`、`systems/ivy.rs`、`systems/ivy/*`。
已实现基线见 `evidence/core-recovery-20261015/ivy/REPORT.md`（IvyStorage/IvyLeafStorage/WallCoord/
动画/RemoveIvyCmd/IvyDirectionProposer/leaf spawner/注册）。
本 SPEC 只定义剩余缺口要实现的**原行为契约**。证据源：grower.asm.txt（0x14094a910 主 grower，
1637 行）、insert.asm.txt、spawn.asm.txt、方向/叶子证据、v2/v3 过程表、骨架 PDB 依赖注解、
`sysres-procedures-v3.json`（比 v2 多 `is_point_still_valid` 0x140998dc0 与 `impl$0::add_ivy_systems`）。

区分：**可直接落地**=字段偏移/常量/调用点已核实；**需先取证**=行为/所有者待进一步差分。

---

## G1 完整 grower（ivy_grower，0x14094a910）—— 可直接落地（主体）

### 1.1 系统门控
- 入口先跳表 `jmp [rcx + 0x118]`（按首个参数某判别资源 +0x118 分派模式）。
- `[r14]`（=参数0指针）+0x48 字节 **bit0** 置位才继续（`test cl,1; jne 0x14094ac51`），否则整帧跳过
  （0x14094ac20..4b）。当前 Rust `IvyStorage.flag_48` 保留该字节（"gameplay name 未定"）；grower 读取它。
- 分支含 `ucomiss xmm0, [rcx + 0x11c]`（0x14094ac31）与 `mov dword ptr [r9], [r14+0x1c]`（事件游标复位）。
- **待取证**：+0x118 判别资源身份（AppMode/菜单/全局状态），+0x11c 浮点阈值。

### 1.2 主循环与每 ivy 状态机（0x14094ae00 起，记录步长 0x98 = 152 B）
迭代 IvyStorage 的 hashbrown map（bucket 0x2a8 步进、组 0x10 比较）。记录基址 = r14 − 0x90。字段：
| 偏移 | 汇编访问 | 语义 |
|---|---|---|
| +0x00/0x08/0x10 | [r14-0x90]/[-0x88]/[-0x80] | points VecDeque 数据区/front/base、len |
| +0x18 | [r14-0x78] | point 计数（=len，读、减 1 用于取末点） |
| +0x20 | [r14-0x70] | ivy id（get_direction seed、事件载荷 ivy、leaf map 键） |
| +0x30 | [r14-0x60] | root 副本区（pruner 写新 front） |
| +0x50 | [r14-0x40] | maximum_point_count（`cmp rax,[r14-0x40]; jae continue`，0x14094ae9f） |
| +0x68 | [r14-0x28] dword | pending_growth 判别：0=无、1=有（写入 1/0，见 1.4） |
| +0x6c | [r14-0x24],[-0x20],[-0x1c] | pending from Vec3（从点位置） |
| +0x78 | [r14-0x18],[-0x14],[-0x10] | pending to Vec3（目标点位置） |
| +0x84 | [r14-0xc] | growth_progress f32 |
| +0x88 | [r14-8] | scalar_88 = 生长延迟秒数（倒计时） |
| +0x8c | [r14-4] | flag_8c = 墙跳标记字节（设为 1，见 G6） |

注意：Rust 结构当前 `pending_growth: Option<IvyGrowthSegment>` 判别字节在 +0x68 字节尾——确认
`Option` 判别在 `+0x68` 且 `from@+0x6c`、`to@+0x78`，与汇编一致；`growth_progress` 实为 +0x84。

### 1.3 生长推进（state==1 且 count>1 路径，0x14094b1bb）
- `if scalar_88 > 0.001 (xmm13=0x3a83126f): scalar_88 -= delta ([GameTime+0x28]); continue`
  （0x14094aeaf..adf5）。GameTime.delta 位于 `game_clock.rs` +0x28，当前 `GameClock.delta` 字段吻合。
- 否则：`progress = clamp(progress + delta / 0.075, 0, 1.0)`（0x14094b1bb..1ce；0.075=0x3d99999a，
  1.0=0x3f800000）。`clamp` 为 `maxss 0` / `minss 1.0`（SSE NaN 语义：minss 返回第二操作数）。
- 写当前末点位置：`lerp(pending.from, pending.to, progress)`，写入 points 数据区
  `[rdx+idx*8+8..0x10]`（IvyPoint stride 0x28=40B：id@0, pos@8, dir@0x14, thickness@0x20）与记录
  内 pending（0x14094b1e0..b24e 的两组 movlps/movss）。
- `if progress <= 0.99 (0x3f7d70a4): continue`（0x14094b25a）。

### 1.4 完成段 → 事件（0x14094b267..b311）
1. `state=0; progress=0`（0x14094b267..276）。
2. `Ivy::last_segment_point_ids`（0x140869540，已实现）取 (a,b)。
3. 写 32B 事件行到 Bevy `Events<IvySegmentDoneGrowingMsg>` 双缓冲（0x14094b2d3..b30d）：
   `[data + count*0x20]`=+0 游标值, `+8`=a, `+0x10`=b, `+0x18`=ivy_id。
   **当前 `IvySegmentDoneGrowingMsg` 24B（a,b,ivy）只保留后 3 个 u64；+0 为 Bevy EventId 行前缀，
   消费者 EventReader 读不到，保持现状即可。**
4. 事件资源布局（写入侧观测）：+0x20=data、+0x28=capacity、+0x30=写游标、+0x40=读游标；
   写满走 `0x140562c30` 扩容 panic 路径。
5. 完成时不 add_point；下一帧回 state 0 → 走 proposal 路径加新点（见 G2）。

### 1.5 proposal 路径（state!=1，0x14094aef0..）→ 新增点（G2）
- 只处理 count>0。计算末点索引 `idx = (len-1 + front) mod cap`（0x14094aef9..b24），读末点 pos。
- `closest_segment_index`（0x14094c800）已实现于 `systems/ivy.rs`；调用点 0x14094b06a，
  命中要求 `[rbp-0x40]==1` 且 `0.0 < distance`（0x14094b088 `[rip+0x2168108]=0.0`），否则 continue。
- 命中后在**墙空间哈希**按 wall id 查找（`call 0x1405b0d70`=hash，随后 hashbrown 组循环，0x14094b0e6..
  b1b6）；找到的 wall 需 `seg_count >= 2`（`[wall-0x290]`）且 `length/厚度 [wall-0x270] > 0`
  （0x14094b31d..b33d）。
- 计算候选点：沿 wall 段投影的复杂 SIMD 块（0x14094b343..b703），产出：
  - `WallCoord{u,v}` at [rbp+0x120]/[0x124]，经 `WallCoord::into_world_space`（0x140b199e0）→ [rbp+0x108]。
  - 方向：`IvyDirectionProposer::get_direction`（0x141234590，rcx=&proposer, rdx=&pos, r8=ivy_id seed）
    结果 [rbp+0x20]，乘 **0.21**（0x14094b64f..661，0x3e570a3e）→ 候选位移。
- Y 约束（0x14094b73d..77a，可直接落地）：
  - `if [wall-0xa]!=0`（双面墙）→ `cand_y <= wall_y([wall-0x1c]) + 0.2`；
  - `else if [wall-0x48] < 2`（side 判别）→ `cand_y <= wall_y + (-0.79) + (-0.1)`（即 −0.89）；
  - `else` → `cand_y <= wall_y + (-0.46)`。
  - 违反则 `continue`。常数：0.2=0x3e4ccccd, −0.79=0xbf4a3d71, −0.1=0xbdcccccd, −0.46=0xbeeb851f。
  - 该处 wall 基址 = `[rbp+0x1c0]`（wall 记录指针）；`[wall-0x1c]`=wall 顶 Y、`[wall-0xa]`=双面标记、
    `[wall-0x48]`=side 判别 dword（与 pruner `is_point_still_valid` 同一套字段）。
- 接受（0x14094bf47..bfad）：
  1. `add_point(pos=[当前末点位置], direction=[rbp-0x18]=双面法向)`（0x140869460）；
  2. `state=1`；`pending.from` = 当前末点位置；`pending.to` = (候选位移 x/z + 末点 x/z, y)；
     写顺序见 0x14094bf85..bfad（from.x=xmm15, from.y=edx, from.z=xmm7, to.x=eax, to.y=xmm11, to.z=ecx）。
  3. 回主循环。
- **可直接落地但实现者须逐条转录 SIMD 顺序**：0x14094b703..b711 的 xmm9/xmm5、0x14094bf47..5c 的
  xmm15/edx/xmm7 分别来自 0x14094af25/af2c/af36 读出的末点 pos，保证与现有 leaf 测试一致。

### 1.6 常量总表（grower）
| 值 | 地址 | 用途 |
|---|---|---|
| 0.001 | 0x1429cd060 | scalar_88 延迟阈值 |
| 0.075 | 0x142a9abd4 | progress 速率除数（delta/0.075） |
| 1.0 | 0x142925904 | clamp 上限 |
| 0.99 | 0x142a948d8 | 完成阈值 |
| 0.21 | 0x142ab3194 | 候选位移步长 |
| 0.2 / −0.79 / −0.1 / −0.46 | 0x1429cd068 / 0x142a7e320 / 0x1429eb6f8 / 0x142a7f250 | Y 约束常数（与 pruner 复用同常量） |
| 0.42 | 0x142ab31b0 | 跳跃/落点最大距离（见 G6） |

---

## G2 完整 root spawner（ivy_spawner，0x14094cf90）—— 可直接落地（主体）

### 2.1 门控与预算
- 首个栈参（第 5 参）解引用后 `[res+0x1c] == 1` 才继续（0x14094d006..0a）→ 全局植被开关字节。
  **待取证**：`res` 资源身份（疑似 `GladeSettings.vegetation_around_buildings` 对应原生位或会话旗标）。
- 总点预算：`total_point_count + 0x13 < 0x9c40 (40000)`（0x14094d09b..a7）；超限直接返回。
- 每帧新增点数上限 **0x46 (70)**：`eax=0x9c41-rbp; rsi=min(0x46, eax); rbp+=rsi`（0x14094db6e..b9e）。

### 2.2 两条根生成路径（按 `[res+0x120]` 判别，cmp 3）
- **path A（判别 < 3 = 光标旁根，try_ivy_root_next_to_cursor）**（0x14094d0c1..d292）：
  1. 光标 `Option<Vec2>`（`test byte [r9],1; x=+4, z=+0xc`）。
  2. 就近 wall：经 `closest_segment_index`（0x14094d203）→ 段上最近点 lerp（0x14094d221..d28a）。
  3. 距离门：`ucomiss dist, [rdi+0x78]; jbe skip`（0x14094d479）→ **待取证** [rdi+0x78] 阈值资源
     （疑似"根距光标最小/最大半径"）。
  4. 该 wall 需 `seg_count>=2 && [wall-0x270]>0`；取 wall 后按 `[wall-0x48]`（side<2）分岔
     （0x14094d58f..d786）投影根位置。
- **path B（判别 >= 3 = 随机根，try_random_ivy_root）**（0x14094d297..）：
  1. 遍历墙空间哈希选随机 wall：`call 0x141625a50/0x141625ae0`（perchance range 原生函数，依赖侧）；
  2. 随机 u 坐标：`u = clamp(rng*mult+add)/len + t0`，NaN 跳过，门 `1.0 >= u >= [rip+0x2012a8a]`
     （0x14094dbb4..dc06）→ **待取证** 该阈值浮点（未成功读取，偏移解析失败；≈0）。
  3. 若 `[wall-0x48]>=2` 走另一法向分支（0x14094d68c..d786 双面法向符号翻转）。
  4. 根点 = 段点 + 法向偏移（乘法掩码 0x14094d6ee/d7f2 的符号翻转与 0.63 缩放，0x1429258d4）。
  5. 距离门同 path A。

### 2.3 批量生成（0x14094da23..de8c）
- `batch = 2 + perchance.usize_less_than(3)`（`call 0x141625a50` edx=3; add rax,2 → 2..4 个/帧）。
- 每 ivy 调 `IvyStorage::spawn_ivy`（0x1408696f0，已实现）参数（0x14094db12..4d）：
  - `position` = 根点 [rsp+0x50]、`direction` = 根法向 [rsp+0x70]；
  - `scalar_range` = **(0.0, 表[path])**，表 `[rip+0x21657b5]` = **cursor→3.0、random→1.0**
    （0x14094da5c..68，0x142ab3218）；即初始延迟 scalar_88 ∈ [0,3] / [0,1]；
  - `count_range` = **(0x14, ?)**：栈 [rsp+0x28]=0x14 (20)，[rsp+0x30]=rsi（此时为 ivy storage map
    data ptr）→ **count_range.end 语义需再核实**（Rust ABI 第 6/7 参映射），现行 spawn_ivy
    只断言 start<=end，传 20..<合理上限> 即可，**行为上 maximum_point_count 由 RNG 在 [start,end) 取**。
  - `source_60` = wall id（[rsp+0xf8]）。
- 循环逐帧补点到预算上限；跳过条件含 `rng.uniform_range_f32` 噪声门（**FastNoise::get_noise 调用在
  spawner 内仅 1 处**，见骨架 PDB；采样点/阈值 -0.02 见 insert 0x1409441cb `0xbca3d70a`）→
  **待取证**：spawner 内 get_noise 采样点与比较方向（root spawn 噪声门）。

### 2.4 UI/ActionSet 门（G5 合并）
- 骨架 PDB 内联依赖含 `country_core::input::action_set::DigitalActionState::pressed`
  （`systems/ivy/ivy_spawner.rs` source_line 240）与 `CursorTerrainRaycast::position` → spawner 读取
  某 ActionSet 的 pressed 决定是否尝试根。**待取证**：对应 DigitalAction 键（原生按钮），需在
  `input/action_set.rs` 与原生按键映射处差分。

---

## G3 wall-change point-validity pruner（ivy_pruner 主函数 0x140997410）—— 可直接落地（主体）

### 3.1 结构
- 与 grower 同 prologue；参数 4 个（rcx..r9）+ 栈参。迭代 ivy map（0x98 步进）。
- 对每 ivy：取末点，经 `is_point_still_valid`（0x140998dc0，实参 `(point=&[rbp-0x40], wall, r8, r9)`）
  判断**末点**与**根点**是否仍在墙支持上。

### 3.2 is_point_still_valid（0x140998dc0）—— 可直接落地
返回 `Option<&PublicWallState>`（rax；null=无效）。其有效性判据（pruner 内联调用点 0x14099817a..32f）：
- 取最近 wall（经墙空间哈希查询 `0x1405b0d70` + 组循环，要求 `[wall-0x290]>=2 && [wall-0x270]>0`，
  `cmp [wall-0x48],1; ja skip`）。
- `if [wall+0x296]!=0`（双面）：`valid = wall_y([wall+0x284]) + 0.2 > point.y`；
- `else if [wall+0x258] < 2`：`valid = wall_y + (−0.79) + (−0.1) > point.y`；
- `else`：`valid = wall_y + (−0.46) > point.y`。
- 常量加载 0x140997f57..f71：xmm7=0.2, xmm8=−0.79, xmm9=−0.1, xmm10=−0.46（与 grower 同一套）。

### 3.3 末点失效 → 尾部删除（0x140998335..b0）
`is_point_still_valid(末点)==null`（或 Y 不满足）时：
1. `len -= 1`（`[r13-0x78]--`）；若原 len==0 走 panic（0x140998931/0x140998946）。
2. 读被删末点 id（`[data + idx*40]`）。
3. `progress=0`（`[r13-0xc]`）、`state=0`（`[r13-0x28]`）——丢弃进行中的生长。
4. leaf 存储 map 查 `[r13-0x70]`（ivy id），调 `0x1412f5490` = `delete_affected_leaves`（现行实现
   的 retain 谓词 0x1412f54b1..15 同函数）。
- 无"整 ivy 删除"；删尾后 ivy 继续留 map（与现行 RemoveIvyCmd 行为一致）。

### 3.4 根点失效 → 根部删除（0x140998430..711）
- 复制根点到局部（[rbp+0x70]），对根点再调 `is_point_still_valid`。
- 根也失效（0x140998587 rax==0）：
  1. `pop_front`：`front=(front+1) mod cap`（0x1409985bd..5eb 对 [r13-0x80]/[r13-0x90] 操作），
     `len--`（[r13-0x78]）。
  2. 新 front 覆盖 root 副本（[r13-0x68]/[r13-0x48]）与 `[rsi]`/`[rbp+0x100]` 根字段
     （id、pos、dir、thickness：0x140998610..64b）。
  3. leaf 清理同 3.3。
- 根仍有效（0x1409986e0）：只走 leaf map 清理分支（0x1409986e0..6fb 也调 0x140051030/0x1412f5490）。

### 3.5 循环出口与临时数组（0x140998748..88d）
- 收集被删 ivy id 到 `[rbp+0xc8]` 数组（Vec 扩容 0x140b81df0），逐 id 对 leaf map 清键
  （0x1409987b0..80f `0x140b78f60` = HashMap::remove）。**待取证**：这是否完整"整 ivy 删除"策略——
  观察上仅清理受影响叶子；ivy 本体不删。

### 3.6 pruner_cmd / remove_ivy_at_location
已实现（0x140999330/0x140999860），无需改动。

---

## G4 root/branch 删除策略（合并于 G3）
见 3.3/3.4：墙变动后 ①末点失效删尾（含叶子/生长复位）②根失效弹根并更新 root 副本。**无整 ivy
删除路径被观察到**；"删除策略"的完整闭环（含 source_60 语义）待取证。

---

## G5 UI/ActionSet 门
- grower/spawner 无 `In<UiState>` 参数；spawner 通过 ActionSet pressed + 光标 raycast 门控根生成（G2.4）。
- 现行 `ivy_pruner_cmd`/grower 不读 AppMode；grower 的模式判别在 +0x118 资源（G1.1）。**待取证**：
  +0x118/+0x120 资源、ActionSet 键、`[res+0x1c]` 旗标资源身份。

---

## G6 wall jump / hole 约束 —— 可直接落地（主体）
- `PrevWallHoles::wall_holes`（0x140f86d00）返回洞 slice（rsi=ptr, rdx=end，条目 0x14=20B：
  `[+0..0x10]` 盒数据 + `[+0x10]` 启用字节）。
- grower 0x14094b7be..b86e：候选点经 `0x141240140`（盒包含判定，`test al,al`）逐洞测试；命中 →
  **`flag_8c=1`**（`mov byte [r14-4],1`）并跳过本轮加点到主循环。
- `flag_8c` 在下一轮主循环 0x14094ae63..96 被消费：非 0 时对 leaf 存储 map 查 ivy id，随后
  **清零**（`mov byte [r14-4],0`）。→ 洞命中后当帧清叶子的精确谓词**待取证**（该块只做 map
  查找/清标记，未见显式 delete 调用，可能是下一帧 grower 复查）。
- 落点最大距离 0.42（0x14094bd38..40 与 0x14094bf26..2e）超出则拒绝该候选点。

---

## G7 ivy_leaf_spawner 的 GladeSettings +1c 旗标接线 —— 可直接落地
- 原生签名（PDB FunctionSystem 泛型串）＝
  `EventReader<IvySegmentDoneGrowingMsg>, ResMut<IvyStorage>, ResMut<IvyLeafStorage>, ResMut<IvyLeafRng>,
  Res<IvySpawnNoise>, Res<GameTime>, Res<GladeSettings>`——**是 `Res<GladeSettings>`，不是 `In<bool>`**。
  现行 `ivy_leaf_spawner` 读 `settings.ivy_leaves`（serde 字段，映射原生 `GladeSettings+0x1c` 字节）。
- 行为合同：`GladeSettings+0x1c` 字节为 0 → 系统直接返回且**不推进 EventReader**（现行实现吻合）。
- **改动建议（提交中央仲裁，勿直接改 `resources/glade_settings.rs`）**：原生 11 键 serde schema
  （0x142aceb88）与默认值（0x1408c2240）已恢复；`ivy_leaves` 首字段对应原生 +0x1c 字节——建议在
  `GladeSettings` 增加文档注释锁定"首字段=原生 +0x1c 开关"，并把 leaf 门的读取保持 `settings.ivy_leaves`
  （Rust 字段，勿改为硬编码偏移）。**不改布局**；仅当中央要求 ABI 对齐时才考虑显式字段序。
- 事件维护：`Events<IvySegmentDoneGrowingMsg>` 由 `register_ivy_storage` 已初始化；缺 grower 生产。

---

## G8 GPU 上传/渲染边界（本轮后置，只定义 CPU producer/consumer）
- producer（CPU）：grower 完成段事件 → leaf spawner 写 `IvyLeafStorage`；branch 点进 `IvyStorage`。
- 消费（原生，后置）：
  - `ivy_send_to_gpu_segments`（0x140a9b4e0）—— branch 段上传。
  - `ivy_send_to_gpu_leaf`（0x1408c1580）—— leaf 上传。
  - 渲染 shader：`instanced_ivy.rs` draw_instanced_ivy_branch/leaf/flower_shader（0x1408dbc50/
    0x1408dc180/0x1408dc1a0）与 `ivy_leaf_shader`（0x1408dc1c0，v3）。
- 真实边界：CPU 仅生产存储，不写 SSBO 布局；SSBO 元素布局属渲染域，**本组不实现**。SPEC 只要求
  storage 字段（IvyLeaf 已含 position/vector_2c/vector_58/branch_direction/size/branch_thickness/
  growth/frame 等）与原生 `IvyLeafStorage` 内联布局对应。

---

## G9 原全局调度 —— 部分取证
- `impl$0::add_ivy_systems`（0x140944070，v3）：`mov dl,0x16`（**22** 系统链）→
  `0x141664d80`（SystemConfigSet 构造）→ `0x141450fa0` → `Schedule::add_systems`（0x140b67280）。
  → 原生把 ivy 以 22 个系统链整体 `add_systems` 进某 schedule（Update）。**待取证**：链内 22 系统
  精确顺序与 set 归属（内联遮蔽，需从插件/调用点差分）。
- `insert_ivy_resources`（0x1409440c0，insert.asm.txt 已覆盖）：插入 IvyStorage/LeafRng(seed10)/
  IvySpawnNoise(threshold −0.02)/...，`IvySpawnNoise` 位于 `+0x78` 阈值。已实现于 registration。
- `add_ivy_events`（0x140944280）：注册事件（含 hash 常量 0x43c6e997585378f3 等）。已实现。

---

## 待取证清单（无证据不落地）
1. grower/spawner 门控资源身份（+0x118 模式判别、+0x11c、spawner `[res+0x1c]`、`[res+0x120]`）。
2. spawn_ivy `count_range.end` 的确切值（当前 20..? 的 ? 语义）。
3. spawner 内 FastNoise::get_noise 采样点与噪声门比较方向（阈值 −0.02 已证于 insert）。
4. `[rdi+0x78]` 光标根距离阈值资源。
5. flag_8c 洞命中后的叶子清理谓词闭环。
6. 22 系统链内精确顺序与 garden 等系统的相对序。
7. source_60 业务命名（wall id 传递到 leaf source_18）。

## 明确排除（本组不实现）
- GPU SSBO 上传与 shader（G8 边界内定义）。
- crate 依赖内部实现：perchance/bracket-noise/glam/bevy_ecs/hashbrown（复用原依赖）。
- `resources/glade_settings.rs` 修改需中央仲裁。
