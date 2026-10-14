# country-core systems/resources 并行深挖 Playbook（2026-10-14）

本文件是并行 agent 的共享挖掘手册。目标是**产出每模块的真实设计与业务逻辑文档**（不写实现代码），
并在中央索引汇总冲突与纠错。

## 1. 工具入口（Windows 本机）

### 1.1 RE-MCP 批量反编译（首选，已预分析 116,270 函数）

```sh
# 生成 job（从权威函数清单 sysres-procedures.json 抽取指定领域的全部业务过程）
python D:/game/reverse-engineering/tiny-glade/scripts/run-decomp-batch.py <domain> <dbid>
```

- `<domain>` 为领域名（见 §3）；`<dbid>` 为**你专属**的 database id（如 `tg_a1`），
  不得与其他 agent 重复。
- runner 自动：生成 job → 注入 open/wait/close 包装 → 调
  `node D:/game/reverse-engineering/tiny-glade/scripts/mcp-job.mjs re_mcp_ghidra job out`
  → 把每个函数的反编译落盘到 `evidence/country-core-deep/<domain>/decomp/decomp-<va>.txt`。
- 单函数反编译输出 1–100KB。**不要整份读进上下文**：先看大小，大函数分段读或 grep 结构。
- 反编译失败的地址（NotFound，多为 Ghidra 未建函数的小函数）：用 §1.3 原字节手解。

### 1.2 单个/补充 MCP 查询

job 文件是 JSON 步骤数组：

```json
[
  {"tool":"open_database","args":{"file_path":"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe","run_auto_analysis":false,"database_id":"<你的dbid>","keep_open":true},"timeout_ms":300000},
  {"tool":"wait_for_analysis","args":{"database":"<dbid>"},"timeout_ms":300000},
  {"tool":"decompile_function","args":{"database":"<dbid>","address":"0x140a84c50"},"timeout_ms":180000,"allow_error":true},
  {"tool":"get_xrefs_to","args":{"database":"<dbid>","address":"0x140a84c50"},"timeout_ms":120000},
  {"tool":"close_database","args":{"database":"<dbid>","save":false},"timeout_ms":120000}
]
```

`node D:/game/reverse-engineering/tiny-glade/scripts/mcp-job.mjs re_mcp_ghidra job.json out.json`
（out.json 的父目录必须已存在）。可并行多个 MCP 会话（已实测 2 路并发 OK）。

### 1.3 原字节 / 字符串直读（RNE 二进制，比 MCP 快）

```sh
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py <va0> [va1...]   # NUL 结尾 C 串
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --fat <va...>      # &str 胖指针 {ptr,len} → 字符串
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --hex <va> <n>      # 十六进制字节
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --unks <decomp.txt> # 自动解析反编译文件里全部 UNK_142* 引用（胖指针/内联串）
```

- panic 的 `&UNK_...` 地址通常是**胖指针**（用 `--fat`）；`assertion failed: ...` 是内联 C 串（用默认模式）。
- 反编译里的 `FUN_1428d9430(str, line, file)` = panic_str（str/file 都是胖指针，line 为 u32）；
  `FUN_1428d9390(args...)` = 格式化 panic。
- 机器码手解小函数：`--hex` 后按 x64 常识读（`cmp qword [rcx+0x18],0; sete al` 即“字段==0”）。

## 2. 已证实的关键逆向事实（直接复用，不要重新推导）

1. **游戏 Vec 布局 = cap-first**：`{cap@+0x00, ptr@+0x08, len@+0x10}`（与 std 的
   {ptr,cap,len} 不同！）。扩容函数族 FUN_140562ab0/FUN_1405629f0/FUN_140562630/
   FUN_140562870/FUN_140562db0（grow，带 40B/48B/24B 等元素大小）；drop = FUN_14088bae0
   /FUN_14088acf0；memmove 右移 = func_0x142923c10。
2. **Option/哨兵**：`i64::MIN`（0x8000_0000_0000_0000）常用作 None 哨兵（判别复用首字段）。
   `is_recording` 0x140a86970 = `xor eax,eax; cmp rax,[rcx+0x30]; setno al`（+0x30 >= 0）。
3. **enum 判别 niche**：以 i64::MIN 为基址存储（变体 i 存 0x8000...+i）；
   还原 = `x - 0x7fffffffffffffff`（结果 1/2/3 = 变体 0/1/2），负数即非判别（真实指针）。
4. **Arc 克隆** = `LOCK; inc [ptr]; UNLOCK`（+溢出检查）；Arc drop = `LOCK; dec; UNLOCK`，
   归零后 func_0x0001402e5d80 释放。看到该模式即可确认 Arc/共享所有权。
5. **fastrand 全局 RNG** = FUN_14132ade0(&static_0x142ad7918, &range) → u64。
   `HistoryIdentity::generate` 0x140a84220 = fastrand u64 全范围。
6. **Bevy ResMut/Mut 识别**：函数参数若出现 `param[2] = param+0x1c`（tick 回写）且
   首字段是指针 → `&mut Mut<T>`（ResMut<T>）。
7. **PDB 名与行为可能错位**（已实例：0x140a84760 PDB 名 edit_chain_mut 实为
   set_current_snapshot；0x140a86980 PDB 名 is_recording 实为 finish_potential_mutation）。
   **机器码为准**，PDB 名只作线索。发现错位 → 记入 DIGEST 的“错位/纠错”节。
8. **Ghidra 数据库没有可读 Rust 符号**（list_names 只有导入表）；权威函数名→地址清单 =
   `evidence/country-core-deep/sysres-procedures.json`（来自 PDB 编译单元元数据，已过滤
   derive/依赖胶水，1,161 条业务过程）。
9. **decompile_function 对 Ghidra 未建函数的小函数返回 NotFound**：这些通常 < 40 字节，
   直接 `--hex` 读字节手解；或反编译其邻接函数（如 0x140a86970 可由 0x140a86980 的
   前导区覆盖）。
10. **空反编译 ≠ 无业务**：先核对机器码与调用点（既有项目教训）。
11. **函数边界表（round2 新证，通用手段）**：PE .rdata 含编译器生成的 CU 函数边界表
    （0x14369a000 / 0x143682f74 / 0x143697308 等），含全部本地符号（私有/内联残留）
    的起止。对 NotFound / S_PUB32 无符号的函数：rne-read --hex 读表枚举边界 →
    候选区间逐个反编译/机器码排除。
12. **索引层级（v3）**：S_PUB32（public-symbols.json）= 仅 pub 项、地址权威；
    骨架 CU"编译实例 RVAs"= 含私有函数、地址较"首条位置"可信；两者合并为
    `sysres-procedures-v3.json`（1,864 过程）。私有函数不在 S_PUB32 是正常现象，
    不代表"全内联"——先查 CU RVAs 与函数边界表再下结论。

## 3. 领域划分与专属 database id

| 领域 | 文件匹配（sysres-procedures.json 的 key 子串） | dbid |
|---|---|---|
| history | `/history`（resources + systems 两域） | tg_a1 |
| wall | `resources/walls/`、`systems/wall/`、`stairs_walls`、`shape_intersections` | tg_a2 |
| session-save | `session.rs`、`systems/save/`、`systems/prefab`、`resources/prefab/`、`loading_screen`、`logo_animation` | tg_a3 |
| terrain-water | `terrain`、`water`、`world_raster`、`raster_stroke`、`garden`、`in_progress_curve`、`drawing_input_smoother`、`brush_sizes`、`cursor_terrain_raycast`、`mouse_terrain_raycast`、`read_mouse_depth_value`、`mesh_follow_terrain`、`rocky_terrain`、`glade_border`、`garden_mask`、`water_mask`、`water_exclude_clutter_mask` | tg_a4 |
| ui-input | `systems/ui/`、`ui_`、`mode_manager`、`mod_key`、`input/`、`mouse_capture`、`toolbar`、`photo_mode`、`onboarding`、`gui` | tg_a5 |
| camera-time | `camera_rig`、`main_camera`、`look_at_camera`、`ui_camera_mode_gizmos`、`time_of_day`、`time_update`、`weather`、`daily`、`glade_settings`、`timelapse_replay`、`arrow_gizmo_animation`、`sparkle`、`clear_canvas`、`despawn` | tg_a6 |
| collision-eco | `collision/`、`stairs/`、`critters`、`paths/`、`ivy/`、`clutter_meshes`、`preplaced_autoclutter` | tg_a7 |
| audio-misc | `audio`、`steam`、`debug`、`shutdown`、`feedback_report_resources`、`gpu_lut`、`grading_lut`、`color_icon`、`color_id`、`gothic_window_profile`、`glade_color_customization`、`image_formation`、`placement_data`、`texture_presets`、`render_ssbo`、`shader_refactoring_state`、`window`、`editing_highlight`、`app_loading_state`、`frozen_map`、`meta_commander`、`name_hash` | tg_a8 |

## 4. 交付物与格式（每领域一份）

`evidence/country-core-deep/<domain>/DIGEST.md`：

1. **模块清单**：覆盖的文件 + 业务过程数（与 runner 汇总一致）。
2. **布局表**：每个资源/结构 = 偏移、大小、含义、证据（函数地址 + 反编译要点）。
3. **方法/系统行为**：逐函数 O（已证实）/ I（推断，注明依据）/ U（未知，不臆造）。
   系统函数写明 ECS 参数面（Query/Res/ResMut/EventReader/Commands 的判别依据）。
4. **panic/日志串清单**（原文 + 地址）。
5. **与现有合同/代码的差异**：对照 `reconstruction/country-core-render/wp*/contracts/*.md`
   与 `reconstruction/country-core/src/**`，逐条列出错误/缺漏（“代码写 X，证据为 Y”）。
6. **跨领域线索**：调用了其他领域的函数/资源（记地址 + 名称，供中央索引关联）。
7. **错位/纠错**：PDB 名与行为不符、合同地址错误等。

反编译落盘到 `<domain>/decomp/`。**不要修改 reconstruction 源码**（文档与证据 only）。
每个 agent 结束时把 10 条以内“最重要发现”写入 `<domain>/HIGHLIGHTS.md`（供中央汇总）。

## 5. 效率规则

- 反编译按批跑（runner 已批量化）；大函数（>8KB）先 grep `FUN_|func_0x|UNK_` 看调用结构。
- 字符串证据优先用 rne-read（快），不要为字符串开 MCP 会话。
- 同一函数不重复反编译；检查 `decomp/` 是否已有。
- 不求穷尽：优先权威状态（资源布局）+ 高价值业务（系统主流程/状态机），derive/泛型胶水跳过。
- 发现跨领域冲突 → 写进 HIGHLIGHTS.md，由中央 agent 裁决（机器码为准）。
