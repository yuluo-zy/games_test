# Group A 审查：history + session-save + camera-time 未探究清单

> 审查 agent（组 A），2026-10-15。范围：history（I=10）、session-save（I=1/U=5）、camera-time（I=6）
> 三个已分析完成域。只审查，未改任何源码/章节。方法与台账依据：MODULE-DEEP-DIG.md、三域 DIGEST、
> 对应章节、round3/O-AUDIT-REVIEW.md（round1）、round3/O-AUDIT-ROUND2.md（round2）、oa2 抽查分片
> （w6_ui_camera / w7_history_save）、round2 三域 CLOSURE.md。行数据落盘 `round3/REVIEW-UNEXPLORED/groupA_rows.json`。

## 0. 概览

| 域 | 章节行 | O | I | U | 无 decomp 的 I/U 行 | 证据链 ERRORFILE 行 |
|---|---|---|---|---|---|---|
| history | 130 | 120 | 10 | 0 | 0（全部有 decomp） | 6 |
| session-save | 103 | 97 | 1 | 5 | 5/6 | 7 |
| camera-time | 79 | 73 | 6 | 0 | 6/6 | 12 |

- **无结论行检查**：SOURCE_MAP.json 三域 business 函数（带 VA 且 kind=business）已全部在章节有
  O/I/U 行，0 条"业务函数但章节无行"（修复行解析正则后含 focus/lens/eye_forward 3 条），章节内也
  0 条无结论行。全部 business 行有结论。
- **主要系统性发现**：三域共有 **25 行 O 结论引用的 decomp 文件是 NotFound 错误文件**（`{"error":...}`），
  分布 camera-time 12 / session-save 7 / history 6。多数结论经机器码/rne-read 核实正确（与 round1
  round2 审计中 0x14003bb80 簇"结论也错"不同），**证据列本身无效**——同属 O-AUDIT-REVIEW 错误模式 #2
  （"NotFound 证据当作 O"），需要统一改为"hex 手解 / 机器码"注记或补引有效反编译。

---

## 1. U/I 行盘点

### 1.1 history：I=10（全部为回放域泛型实例/闭包）

| # | 文件 | 函数 | VA | 现状结论 | 判定 |
|---|---|---|---|---|---|
| 1-6 | systems/history/replay_raster.rs.md:14-19 | `replay_raster<Cobblestone1/2/3, Garden, Path, Water>` 6 实例 | 0x1409fc290 / 0x1409f1cf0 / 0x1409f9930 / 0x1409f6ff0 / 0x1409febf0 / 0x1409f4650 | I（"领域包装/薄包装"） | **可再挖·中**：6 份 decomp 均为 57-67KB 真实重内联（非 316B 截断），"薄包装"低估了内容；每实例仅 closure_env 不同，共享核心语义是真正的算法缺口（见 §3.1），实例级验证应依附共享核心的解析 |
| 7 | systems/history/replay_garden.rs.md:11 | `replay_garden::closure$1::closure$0` | 0x141447500 | I | **可再挖·低**：decomp 1009B 真实，薄闭包，MCP 一次可证 |
| 8 | systems/history/replay_wall.rs.md:13 | `replay_wall::closure$0` | 0x140abfaa0 | I | **可再挖·低**：decomp 1260B 真实，薄闭包（宿主 replay_wall 0x140951b80 已 O） |
| 9-10 | systems/history/replay_water.rs.md:11-12 | `replay_water::closure$1::closure$0` / `closure$2` | 0x141447640 / 0x140a01560 | I | **可再挖·低**：decomp 1009B/1863B 真实 |
| — | 备注 | oa2 w7 对同族 `generate_replay_cmds_from_edit<water>` 0x141444c70 判**不一致**（316B 截断 decomp ≠ 实函数 10.4KB；跳表基址 0x142c5f2fc 非共享 0x142c5f248），提示 I 行的"薄包装"标签在共享核心未解前不可当作终态 | | | |

**判定说明**：history 的 I 行不是卡点——机器码路径全部存在且 decomp 已落盘，只是价值集中于共享核心
（§3.1）而非各实例。

### 1.2 session-save：I=1 / U=5

| # | 文件 | 函数 | VA | 现状结论 | 判定 |
|---|---|---|---|---|---|
| U1-U5 | resources/session.rs.md:11-15 | `_::impl$0::deserialize::visit_str<serde_json::error::Error>` / `deserialize<enum2$<Value>>` / `deserialize<SliceRead>` / `serialize<CompactFormatter>` / `serialize<Value::Serializer>` | 0x1403b5a20 / 0x141328d00 / 0x1403b52d0 / 0x1403b5500 / 0x141328bb0 | U（"serde 派生胶水，非业务"） | **可再挖·中**：5 个地址**从未反编译**（无 decomp 文件）。项目规则下 serde 字段名/默认值是类型合同、属业务；这些 monomorph 属 session.rs 的哪个类型（SessionMeta/NextSession/SavedSession）未确认。低成本 MCP 定点反编译即可：若字段名已被 try_load_from_path(0x140b46480) 覆盖则关闭为胶水，否则补全合同 |
| I1 | resources/session.rs.md:38 | `impl$10::build` | 0x140b47840 | I（SessionFlowPlugin build，Bevy 事件胶水证据） | **可再挖·低**：decomp 22640B 真实；注册胶水身份可一次确认，不计业务 |

### 1.3 camera-time：I=6（全部 serde 胶水面）

| # | 文件 | 函数 | VA | 现状结论 | 判定 |
|---|---|---|---|---|---|
| I1 | resources/time_of_day.rs.md:17 | `_::impl$0::deserialize::visit_str` | 0x140080710 | I（serde 胶水面） | **可再挖·低**：未反编译；TimeOfDay serde 契约（"hour" 字段）已可从相邻 monomorph 0x1400802c0 / 0x140b3bc00 推断，本址仅确认身份 |
| I2 | resources/glade_settings.rs.md:13 | `_::impl$0::deserialize::visit_str` | 0x140a468b0 | I（serde 胶水面） | **可再挖·低**：未反编译；GladeSettings serde 字段（8+ 项）如未在他处覆盖则值得补 |
| I3-I5 | systems/camera_rig.rs.md:30,32,33 | `_::impl$0::deserialize::visit_str<serde_json>` / `serialize<serde_json Pretty>` / `serialize<Value>` | 0x1403b5ba0 / 0x1403b5000 / 0x1403b4e10 | I（同族） | **卡点·低价值**：会话相机记录字段名已由主 monomorph 0x1403b4ca0（serialize，O）全录（yaw/pitch/position/…/position_cache），三者是同一 impl 的重复实例，纯 serde 胶水，不补也无合同缺口 |
| I6 | systems/main_camera.rs.md:17 | `_::impl$0::deserialize::visit_str` | 0x14003b6a0 | I（derive 反序列化胶水面，未反编译） | **可再挖·低**：CameraRotationPivot 7 变体名已由 0x14003ac90（O）全录，本址仅确认身份；注意 0x14003b6a0 位于 ICF 折叠区，须"PDB 精确符号+机器码"双证 |

---

## 2. 无结论行检查（SOURCE_MAP 交叉）

方法：`scripts/review_groupA_srcmap.py` 将三域 SOURCE_MAP 的 `kind=="business"` 函数 VA 与章节行 VA
集合比对（含 focus/lens/eye_forward 行名带尾注的解析）。

**结论：0 条遗漏**。三域 business 函数全部在章节有 O/I/U 行，章节内无空结论行。需要登记的边角：

- camera-time 早期版本（本审查提取器首版）曾把 focus/lens/eye_forward 三行漏解析，系行内函数名带
  "（focus_mut 同址）"等尾注的正则问题，非章节缺行；现已计入（camera-time 79 行 O=73）。
- SOURCE_MAP business 中 `MainCamera::eye_forward`（0x140911df0）与 DIGEST 用名 `eye_position`
  （0x140911dd0）为两个相邻转发器，章节均已覆盖，无冲突。
- 三域章节的"内联/无独立符号"列表里标"待推断"的项（如 load_game 的 apply_parsed_game_data /
  apply_backward_compatibility_overrides / upgrade_window_color；session.rs 的 SavedSession::screenshot_path
  / SessionId::snapshots_folder_path 等）是**内联符号**，属 §3 算法缺口而非"章节无结论"。

---

## 3. 算法未深挖清单（关键业务算法未展开处）

DIGEST/章节对多数结论只到行为级；以下为**结论级而非算法级**、且机器码路径存在可再挖的点。

### 3.1 history（4 项，2 可再挖 / 2 转交）

| 缺口 | 函数/地址 | 类别 | 具体缺口 | 建议下一步 |
|---|---|---|---|---|
| **generate_replay_cmds_from_edit 共享核心逐 tag 语义** | 0x14143fb50 / 0x14143d2c0 / 0x1414423e0 / 0x141444c70（各单态跳表基址不同：0x142c5f248 仅属其一，water=0x142c5f2fc） | **可再挖（最高价值）** | 实函数约 10.4KB（oa2 w7 判"316B 跳板"为 Ghidra 截断）；HistoryEdit tag(0..=8+)→ReplayCmd 判别字节/参数提取的**逐 tag 映射表**未枚举（现仅散落墙 tag7/地形/楼梯判别 10/光栅等个案）。这是回放命令生成的枢纽 | pdata 函数边界表（round2 camera-time-CLOSURE 附录技巧，.rdata 0x14363c000 12B 条目）+ 原字节解跳表；或 MCP 对共享核心启用跳表恢复后全文读 |
| **ReplayBuffer push 校验/去重规则** | history_process 0x140af5b30 内 0x140af6381 起 | **可再挖·中** | "逐条校验（重复/冗余命令丢弃逻辑）"的**具体判据**（哪些判别字节组合相邻会被丢弃）未写成规则；第二事件流 0x142ae5420 case 0x4-0xe 是"校验/合并"簇但逐 handler 字段级语义未解 | 对着 0x142ae5420 已解码表 + 42KB 反编译逐 case 归纳判据（round2 CLOSURE §4 已解表目标，只差 handler 语义） |
| **第二事件流的事件类型** | history_process param_4（Local 双游标 16B 记录） | **可再挖·中** | 记录 byte[+9] 分派已解码，但**产生该 16B 记录流的 Event/通道类型未命名**（HistoryCmd 是 4B，不是它） | 搜 Events 注册（add_history_events 0x140943f40 注册清单）+ 事件大小 16B 的类型 |
| **add_decorator_diff 楼梯 checksum 计算规则** | 0x140a854a0 步骤 4 / FUN_140a879b0 | **转交（卡点）** | 校验 `stairs.tag1 == 前表 +0x28 u64` 的 u64 从边集如何算出未解（构造方在楼梯域） | 转 stairs/wall owner（round2 CLOSURE 已登记） |

### 3.2 session-save（6 项，4 可再挖 / 2 转交）

| 缺口 | 函数/地址 | 类别 | 具体缺口 | 建议下一步 |
|---|---|---|---|---|
| **快照 schema 分节字段级映射** | capture_game_snapshot 0x1417a1d00（184KB closure）/ capture_game_snapshot_save_data 0x1417a0430（~0x400B 结构） | **可再挖（最高价值）** | 顶层节序（Version→Walls→…→Raster）已知，但**每节的 JSON 字段名/结构**仅碎片已知（StairsSnapshot{edges,checksum}、TerrainEditingData、DecoratorStorageSnapshot）；0x400B 结构的逐字段对应（~9 组世界资源→字段）未枚举。这是存档格式合同的核心 | 在 0x1417a0430 反编译（OK 文件）上做字段级标注；0x1417a1d00 分段读（巨大，建议按节序串标记分片） |
| **init_game_state_from_snapshot 应用映射** | 0x1417a9c50 | **可再挖·中** | "应用解析数据到世界"未写成"节→世界资源"映射表 | 反编译全文后按 9 节分组标注写回目标 |
| **load_game 内联业务逻辑** | apply_parsed_game_data / apply_backward_compatibility_overrides / upgrade_window_color / extract_max_wall_id_from_history（均在 load_game 0x14146d030 内） | **可再挖·中** | 除颜色升级映射 {1→2,2→5,4→8,5→3,8→4} 与 max wall id 一行外，前三个**完全未描述**（backward-compat overrides 是什么、upgrade_window_color 的窗口类型分支） | 对 0x14146d030 反编译按串标记切片，逐内联函数归纳 |
| **snapshot_gc 解析规则** | parse_save_file_snapshots 0x141286730 | **可再挖·低-中** | "解析存档内快照引用"的具体规则（如何从 save 文件找快照引用、比对目录）未解 | 反编译 0x141286730（OK 文件） |
| **NextSessionKind 变体 1 生产者** | —（NextSession::create 0x140b46960 由调用方模板决定 kind） | **转交（卡点）** | 变体 1（解析存档）的模板填充点未定位；vtable 间接调用，Ghidra xref 无索引 | 转 ui-input/startup/session_selection（渲染域）消费点倒查 |
| **execute_save_cmd 变体 0/1/2 存在性 + AsyncSaveCmd 发送端清单** | 0x141721aa0 | **转交（卡点）** | 无专属 case/无 debug 串；发送端 send 0x1408da280 的 xref 不可得 | 转中央/发命令的保存触发域（meta_commander/toolbar） |

### 3.3 camera-time（5 项，3 可再挖 / 2 转交）

| 缺口 | 函数/地址 | 类别 | 具体缺口 | 建议下一步 |
|---|---|---|---|---|
| **immediately_finish_interpolating 逐分支语义** | 0x140911b50（106KB，相邻函数粘连） | **可再挖·中** | 双 jumptable（mode + 快照字节）只到"面"，**逐分支行为未枚举**；decomp 带 bad instruction data 警告 | pdata 边界切片（round2 camera-time-CLOSURE 附录）+ 分段反编译 |
| **paused_main_camera_update case 6/11** | 0x1409145a0 | **可再挖·中** | case 6/11 反编译损坏（bad instruction data）；case 7..10 的 fov 对已录 | 原字节/rne-read 手解该 case 段 |
| **camera_snapshot 逐模式 jumptable** | 0x140911e40（757B 反编译不足） | **可再挖·低-中** | "按模式分发调 body_fn"未逐 case 枚举（哪种模式→哪个 body_fn/快照字段）；jump table 未恢复 | 原字节解跳表 + pdata 边界 |
| **CameraMode 6..12 名字/语义 + PhotoModeCameraMode 联动** | —（模式字节 0..12，前 6 body 实证） | **转交（卡点）** | 6..9=拍照子模式（复用 Normal body fov 槽）为推论；名字无符号级证据 | 转 ui-input photo_mode.rs（next_demo 0x140b596b0）联动 |
| **fog_tint 打包格式 / latitude_degrees 单位 / time_of_day param_3,10,11 类型 / GameTime +0x20.. 字段** | TodLightingSnapshot@+0x20；0x1400802c0 区；0x140a73460；0x140922c30 | **转交/卡点** | round2 camera-time-CLOSURE 已逐项 PARTIAL：fog_tint 内部格式（无消费端）、lat 存储单位（生产端在 startup）、param 类型（无符号、成本高）、GameTime 尾部（无消费端证据） | 均需跨域（startup/渲染消费端）才能闭合；成本>收益者可保留占位 |

**已闭合不重列**（round2 CLOSURE 已定论，勿重复挖）：WalkCamera::rotation 16B=rotation-first 四元数；
MainCamera::reset 0x140911280=zoom（582B 手解）；TodLightingSnapshot 2×u64=fog_tint 语义；suncalc
lat 非常量+lon=0.0+日期 2023<<9 编码；0x14003bb80=ICF 437 符号折叠簇（含 butterfly 3 空方法）。

---

## 4. 证据链抽查

### 4.1 全量扫描（非仅 10 条，覆盖全部 O 行 decomp 引用）

方法：解析三域全部 O 行证据列的 `decomp-<va>.txt` 引用，按 `<域>/decomp`、`decomp`、`round2`、`round3`
四处定位文件并读取内容判定。

| 结果 | 数量 | 说明 |
|---|---|---|
| OK（文件存在且为真实反编译） | **181** | |
| 无 decomp 引用（证据=公共符号名/DIGEST/"见文件作用"） | 77 | 较弱但合法 |
| **ERRORFILE（文件存在但内容是 `{"error":"NotFound"}`/No function）** | **25** | 系统性问题，见下 |
| MISSING（引用路径指向不存在文件） | 8 | 已逐一核对：全部可落到替代位置（round2/hist-closure、round2/decomp r2-、ui-input/decomp、terrain-water/decomp、round3/）或行内已注明"hex 手解"（如 0x140911230/240/260、0x1409149d0）→ **路径漂移/手解注记，非结论错误** |

### 4.2 抽查明细（固定种子 20261015，每域 10 条 O 行 = 30 条）

**history（10）**：7 引用有效（0x140abfb30/0x140a86510/0x140a854a0/0x140b392d0/0x140b363e0/0x140a7c550/0x1408aa9f0 → OK）；
0x1416286d0（generate_path_replay_cmds）→ **ERRORFILE**（Ghidra 无函数；行内已注 round3 机器码修正：尾调目标=0x1414423e0，结论正确但证据列仍引坏文件）；
0x140259e30/0x140316500 → 无 decomp 引用（serde 入口，证据在文件头）。

**session-save（10）**：8 引用有效（0x14091a230/0x141721aa0/0x140b385e0/0x141750c90/0x141720490/0x1417a6d70→round3 文件真实/0x1417a0430/0x140a1a190 → OK）；
0x140b19d10（LogoAnimation::is_waiting）→ **ERRORFILE**（capstone 已解 `movss+ucomiss+seta`，结论正确证据坏）；
0x140002370（clone_behavior）→ 无 decomp 引用（行内直接给机器码 xor eax,eax）。

**camera-time（10）**：7 引用有效（0x140922c30/0x140994320/0x140949e70/0x140a7abd0/0x140911710/0x140993d30/0x140a8a460 → OK）；
**0x14003bb80 × 2**（butterfly constrain/rotate_yaw_pitch）→ **ERRORFILE**（结论正确：S_PUB32 实测该址折叠 437 符号，含 butterfly 两 impl 方法，机器码 1B ret；证据列引坏文件）；
0x14090e510（init_walk_camera）→ 无 decomp 引用（实证）。

### 4.3 ERRORFILE 25 行清单（结论普遍正确，证据列待修）

- **camera-time（12）**：0x14003bb80×4（butterfly 3 + free 1）、0x140994310（walk rotation，round2 已 CLOSED）、
  0x140911280（zoom，round2 已 582B 手解）、0x1409114d0/0x1409114f0/0x140911e30（fov 访问器，hex 手解）、
  0x140914ac0（previous）、0x140002370（clone_behavior）、0x14092e6b0（timelapse is_active）。
- **session-save（7）**：0x140ae8320×2（is_corrupted/is_winter 折叠）、0x140b468c0（is_xmas2024）、
  0x140b46920（NextSession::load）、0x140b19d00/0x140b19d10（logo play/is_waiting）、0x140a439a0
  （ensure_current_snapshot 同址 add_history_edit_to_save_file）。另 0x14092e6a0（GladeType::is_flowery）
  在扫描中落 MISSING，实际证据文件存在于 ui-input/decomp（路径漂移非坏文件）。
- **history（6）**：0x140a84290/0x140a842a0（is_*_diff_empty）、0x140a84c40（debug_set_history_start，3B 手解）、
  0x140a86970（is_recording）、0x140af5b20（HistoryCmd::interrupt，6B 机器码解）、0x1416286d0（generate_path_replay_cmds）。

**模式判断**：与 round1 审计 0x14003bb80 簇（结论也错）不同，本三域的 25 行**结论几乎全部正确**
（多为 3-16B 小函数经 rne-read/capstone 手解，或折叠簇经 S_PUB32 身份核实），但**证据列沿用失败的反编译
文件**。修正成本低：把证据改为"hex 手解 / S_PUB32 N 符号折叠"注记即可。唯一需要复核身份的是
0x14003bb80 上的 butterfly 两行与 free 一行（折叠簇内成员确认，S_PUB32 已含精确符号）。

---

## 5. 汇总与建议

**未探究项计数**：history 可再挖 3（核心为 generate 共享核心）+ 转交 1；session-save 可再挖 4 + 转交 2 +
可再挖 serde 合同 1 组（5 个 U 单态）；camera-time 可再挖 3 + 转交 2 + 可再挖 serde 身份 4（低价值）。

**最值得挖的 5 个**：
1. `generate_replay_cmds_from_edit` 共享核心逐 tag 映射（history，0x14143fb50 族 10.4KB）——回放命令
   生成枢纽，pdata+原字节可解。
2. 快照 schema 分节字段映射（session-save，0x1417a0430 + 0x1417a1d00）——存档格式合同核心。
3. load_game 内联 apply_parsed_game_data / apply_backward_compatibility_overrides / upgrade_window_color
   （session-save，0x14146d030 切片）。
4. `MainCamera::immediately_finish_interpolating`（0x140911b50）逐分支 + paused case 6/11
   （camera-time，pdata 切片 + 原字节）。
5. session.rs 5 个 serde monomorph（0x1403b5a20 族）——低成本 MCP 定点，闭合 SessionMeta 等 serde 合同或
   正式判定胶水。

**证据链**：181/25/8/77（OK/ERRORFILE/路径漂移/无引用）。25 行 ERRORFILE 建议统一修证据注记；
8 行 MISSING 为路径漂移（文件在 round2/hist-closure、round2/decomp r2-、ui-input 等处）或行内已注
"hex 手解"，无结论风险。

**登记给集成 owner 的裁决项**：0x14003bb80 三行（butterfly/free 空方法）的证据列；0x140b19d10、
0x1416286d0 等小函数证据改"机器码手解"；camera-time audit 口径 O=72 vs 本审查 73（focus/lens/eye_forward
3 行解析差）。

**佐证文件**：`round3/REVIEW-UNEXPLORED/groupA_rows.json`（三域章节行全量）、
`scripts/review_groupA_extract.py` / `review_groupA_srcmap.py` / `review_groupA_evid.py` /
`review_groupA_evid_sweep.py`（复现脚本）。
