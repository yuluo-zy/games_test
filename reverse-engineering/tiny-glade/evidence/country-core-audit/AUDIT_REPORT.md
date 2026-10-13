# country-core（提交 664f6a65）审计报告

日期：2026-10-13。对象：提交 `664f6a65 feat(country-core): 全量恢复 21 工作包并整合为可编译 crate`
（`reconstruction/country-core/` 570 个 `.rs`、38,676 行；`reconstruction/country-core-render/wp1–wp21` 交付与合同）。
对照基准：原版 B（`tiny-glade.exe` sha256 `cf048e27…`，`tiny_glade.pdb`）。
复现工具：`tools/country-core-audit/`（见该目录 README）。数据：本目录 `module_stats.tsv`、`data/*.json`。

**可信度说明：** 带“✔本人复核”的条目由我直接对照源码/反汇编核对过；其余来自 8 个只读子 agent 的分域审查与核实报告，
子 agent 均被要求用反汇编或证据文件求证，但未逐条复核。

---

## 1. 总结论

1. **提交完成的是“名字/路径对齐 + 骨架”，不是业务逻辑恢复。** 提交说明自己也写明“全部 implemented_unverified、无 oracle”。
2. **逆向材料本身是真的，不是编造。** 引用的约 9,000 处地址中 89% 恰好落在真实函数起点；抽查的约 16 份 Ghidra 证据文件的调用目标、常量、偏移与反汇编逐条一致。
3. **但合同里标为 O（Observed）的结论只有约 56% 完全正确**（抽查 62 条：35 正确 / 12 部分正确 / 15 错误或标错）。错误几乎全是**读错证据**（偏移单位、符号方向、关键字节、判定对象），而同一工作包的证据文件里往往就有正确答案。
4. **代码比合同更差**：在合同基础上又引入了新错误；多处还把 `reused.rs` 里的外部依赖写成**静默空实现**（返回 `None`/0/丢弃），比 `todo!()` 更危险。
5. **无法运行**：没有任何 `Plugin`/系统注册（`reused::bevy_app::App::add_systems` 是空函数）、0 个测试；本机离线缓存缺依赖（`dolly` 等），无法复现“0 错误”的编译结论。

## 2. 量化

| 指标 | 数值 |
|---|---|
| 函数体分类（`fnstat.py`） | 真实逻辑 603、`todo!/unimplemented!` 441、≤2 行平凡体 1,507、部分 26 |
| `Gap` 占位出现次数 | 1,755（`lib.rs:65`，且派生了 `Resource/Component/Event`） |
| `Res<Gap>`/`ResMut<Gap>` 系统参数 | 59 处 / 30 个文件 |
| 测试 / 系统注册 | 0 / 0 |
| PDB 中 `country_core` 自有函数 | 3,481 个、3,623 KiB 机器码 |
| 被工作包引用过的函数 | 1,731 个（66% 字节）——仅表示“登记过”，不表示“分析过” |

### 2.1 各功能模块挖掘度（分母 = 原版机器码字节；`modstat.py`）

| 模块 | 原版代码 | 未触及 | 仅登记 | 桩 | 浅 | 扎实 |
|---|---|---|---|---|---|---|
| 墙体 / 楼梯 / 碰撞 | 740 KiB | 35% | 52% | 9% | 4% | 1% |
| 历史 / 存档 / 会话 / 预制件 | 521 KiB | 35% | 12% | 30% | 22% | 2% |
| 渲染基础设施（pass/buffer/资产） | 439 KiB | 63% | 13% | 3% | 11% | 10% |
| 启动 / 杂项资源 / 工具 | 400 KiB | 56% | 20% | 17% | 6% | 1% |
| nani + 绘制着色器 + SSBO 库 | 397 KiB | 22% | 41% | 34% | 2% | 0% |
| UI / 输入 | 311 KiB | 18% | 17% | 62% | 3% | 0% |
| 渲染帧 pass（compute/风格化/LUT） | 249 KiB | 15% | 40% | 45% | 0% | 0% |
| 音频 | 175 KiB | 1% | 6% | 55% | 30% | 8% |
| 地形 / 水 / 世界栅格 | 127 KiB | 23% | 43% | 30% | 2% | 2% |
| 植被 / 生物 / 常春藤 | 117 KiB | 34% | 28% | 34% | 3% | 1% |
| 相机 / 时间 / 天气 | 72 KiB | 28% | 30% | 27% | 7% | 8% |
| 其它 | 40 KiB | 20% | 18% | 51% | 6% | 4% |
| 屋顶 | 28 KiB | 35% | 65% | 0% | 0% | 0% |
| **合计** | **3,623 KiB** | **34.1%** | **28.8%** | **26.6%** | **8.2%** | **2.4%** |

口径：untouched=任何文档/证据/代码都未引用；cited=只在合同/证据/注释里登记；stub=对应 Rust 函数是 `todo!`/空/常量/`Gap`；
shallow=有真实代码但行数 < 机器码字节/100；solid=有真实代码且体量相称。“solid”不保证逻辑正确。
局限：分母只含 `country_core` 自有函数（不含 `tiny_glade` 主 crate、`utils` 等）；Rust 函数没写地址则不计入（结果偏保守）。
合同层面：墙体合同里可核验的逻辑只占文本约 15–20%，其余是名字、地址、ECS 签名、callee 列表（标“O，调用链”）。

## 3. 合同结论核实（62 条 O 标签，对照机器码）

| 领域 | 抽查 | 正确 | 部分 | 错误/标错 |
|---|---|---|---|---|
| 墙体（WP11/12/21） | 16 | 9 | 4 | 3 |
| 历史/存档/音频（WP13/14/17） | 15 | 5 | 3 | 7 |
| 渲染（WP1–6/18/19） | 15 | 11 | 2 | 2 |
| 内容/相机/时间（WP7–10/15/16/20） | 16 | 10 | 3 | 3 |
| 合计 | 62 | 35（56%） | 12（19%） | 15（24%） |

### 3.1 确认正确（可信赖）
撤销/重做游标规则（`0x140af6081–611`）；`render_world` 40 个 pass 调用顺序与上传参数（`0x1404ec110`）；
`DrawBuilder` 快照（0xb8 记录，Vec 在 builder+0xe0）与 `multi_draw_indexed_indirect` 常量表 `0x142aaaa78`；`ComputePass` 注册表（hashbrown，16B bucket）；
`SparseInstanceBuffer::gpu_flush`（`<<5` 合并）；`ShadowedBuffer`；`GpuWorld` 27×0xc0；`ViewConstants` 总大小 0x320；
天气系统、TimeOfDay 标志位与角度平滑、`time_update` dt≤1/60；`TerrainStroke` 0.001 间距/0x400 上限；FreeCamera 钳制；
`get_wall_attachment_height`、`split_height_into_skirts`（5.9/4.5/+3.5/1.4）、`HighlightColor::to_rgb`、`clamp_wall_height`；
`InnerWalls`/`PublicWalls` 步长 0x78/0x2a8；`WallStyle` 取值；100 条 FMOD 事件名；存档段键 0–8 与填充表。

### 3.2 合同本身错误（Rust 多数照抄）

| 项 | 合同写法 | 二进制实际 | 证据地址 |
|---|---|---|---|
| `calc_wall_bend` ✔本人复核（取 `+8`、倒序索引） | 前 4 段 | **后 4 段（n-1-i）**，读 **X/Z**（偏移 0 与 8） | `0x140a8abb7–abf0` |
| `snapping_init` | 1.65 比第一段；投影到第一段 | 1.65 比**曲线总长**（+0x30）；投影到**最长段**；`mode=1` 无条件设置 | `0x140ada704` |
| `SnappingMode` | `{mode@0,snapped@1}`，1=线段，2=端点 | 吸附线段写 **3**，端点写 2；`new` 清 +0 与 +0x20 | `0x140ade610`、`0x140a6b8db` |
| HistoryCmd 中断 | byte[1]==4 或 byte[0]==2 | 那是“未弹出命令”的 Option 出口；真正中断是 **byte[1]==2**（`0x02000200`） | `0x140af5ff7`、`0x140af5b20` |
| `finish_input_recording` | 往 `snapshots` 追加 0x30B 记录 | 是一次 **EventWriter 发送**（写第二个参数） | `0x140a8685b–68aa` |
| `nuke_future_states` | 只重置游标 | 同时 `history_start=-1`（`mov [rdi+0x68],-1`） | `0x140a84d4e` |
| `finish_replay` 模式范围 | 6..=20 | 仅 **6..=19** 调 `switch_mode` | `0x1408b3afb–3b65` |
| `finish_replay` 哨兵 ✔本人复核 | `i64::MIN` | **`-0x7fffffffffffffff`**（`0x8000000000000001`） | `0x1408b3b97` |
| `finish_replay` 单位事件 | byte[1]==1 | **byte[0]==1** ✔本人复核（`cmp byte [r10+4*rax],1`） | `0x1408b36f8` |
| 存档键 ✔本人复核 | `"snapshot"` | **`"snapshots"`**（EXE `.rdata` 字符串） | `0x142ace328` 起 |
| `is_halloween_any` ✔本人复核（`cmp byte,1`） | 任一字节 >1 | 任一字节 **≤1** | `0x140b468ac` |
| `NormalCamera::update` 常量 ✔本人复核 | （Rust 写 11.0125/33.45/46/90） | **12.025 / 33.475 / 44.85 / 89.7**；另有 10.0/0.15 | `0x140949fe0` |
| `is_default_arch_height` ✔本人复核 | 常量未知 | `== 2.8f` | `0x1408c5690` |
| `GpuWorldSubsetKind` 名字 | Main/Bush/…（取自 cull shader 文件名） | 引用的 `0x140a241d0` 是 `InstanceKey` 的 Debug；真名表在 `0x140992a60`（Canopy、TreeTrunk、BackgroundTree、SolidVertexColor…共 27 个） | `0x140992a60`、表 `0x142aba868/940` |
| `ColorLibrary` 偏移 | 字节偏移（path→+0x50…） | Ghidra 的 **dword 单位**；真实 `lea [rdi+0x140…]` = HLSL 序号×16（path 在第 20 槽） | `0x14093e510` |
| `ViewConstants` 布局 | +0x200 像素偏移、+0x304 缩放 | 那是**构建器输入**偏移；输出 +0x200 起是矩阵，+0x300 起是 sample_offset | `0x140b18f60` |
| `remap_hour_to_fmod_hour` 值 | {48,17,11,24,22} | **{0,3,11,17,22,24}**（打包 qword 解码错） | `0x140a74790` |
| `DuckSpawnLocations` | 9×4，z∈{-52,-40.22,-28.9,-18.66} | **10×10**，z=lerp(-52,52,j/9) | `0x140f8ee90` |
| `ShaderUri::parse` 示例 | name=`cull_and_bucket` | name 止于**最后一个 `.`**（`cull_and_bucket.cs`）；无 `:` 时 entry=None | `0x140968960` |

小问题：若干地址多一个 0（`0x140143fb50`→`0x14143fb50`、`0x14017bdae0`→`0x1417bdae0`、`0x1401234970`→`0x141234970`）；
`spawn_fish` 标注实为 `despawn_fish`/`fish`；`valid-enclosure.md` 的 4 个函数误用 `contains_point` 的地址（真实 `0x1411ff600–660`）；
`get_enclosure_params`（`0x1408c56b0`：tag<2 时返回 `&self+0x38`）与 `StructureType::Fence=1` 被标成 U，实际可直接读出。

### 3.3 合同对、代码错
`HistoryCmd::redo()` 应为 `0x0000_0100`（代码 `0x0000_0001`，byte[1]=0 被当成 undo）；`adjust_wall_height_impl` 两个输出分支应**互斥**（代码两行都发、t=1.0）；
墙洞接缝复制（应仅当 `min.x<0` 复制 +L、`max.x>L` 复制 −L，只改 X，按 maxX 排序；代码用 `center_x>=0`、X/Y 同移、按 min.x 排序、丢了指纹短路）；
`delete_wall`（第一个 map 的 remove 结果被丢弃，只 despawn 第二个及 plaster Vec）；
`weather_system`/`time_of_day` 合同已完整，代码仍是 `todo!()`。

### 3.4 证据真实性
抽查 `wp2/0x1412e3c00.c`、`wp6/0x142080640`、`wp12/0x140a8a710.c`、`0x140ac1f20.c`、`0x140ac80e0.c`、`wp13/0x140a84c50.c`、`0x140a86730.c`、
`wp17/0x140a7c550.c`、`0x1408b36a0.c`、`wp20/analysis/0x140949fe0_update.txt`、`0x140a74790_*.txt`、`wp8/work/decomp/0x140a97880.c`、`wp10/g3-result.json`、`wp21 g1` 等：
调用目标、常量、偏移与反汇编**逐条一致，未发现伪造**。注意：链接器会合并相同代码，部分共享辅助函数（如 `0x14003bb80`、`0x140563300`）的 PDB 名字与其调用者无关，不能望名生义。
许多 `wp11` 叶子 `.c` 是空 `void` 桩（Ghidra 诚实输出，无信息）。

## 4. 代码缺陷（按危害）

**会写坏数据 / 未定义行为**（✔ = 本人复核）
- ✔ `systems/wall/eraser/wall_eraser.rs:61-71`：每次拖动把**所有墙**的曲线覆盖为空 `Curve`（原函数 20,675B，此处 30 行）。
- ✔ `systems/wall/draw_wall.rs:71`：`current_wall_id` 每帧 `+1` 凭空造 `WallId`。
- ✔ `resources/walls/inner_walls.rs:46`：`next_id` 从 0 起，而 `WallId::is_valid()`（`!=0`）使首面墙无效。
- ✔ `render/shadowed_buffer.rs:32-41`：`with_capacity` 后未 `set_len`，`shrink_to_fit` 释放空间，shadow 长度为 0，随后的 `write`/`copy` 无边界检查 → 堆越界写。
- `render/pass_builder/*`：`ComputePass::new` 的 `resource_registry`、`DrawBuilder::new` 的 `context/shader_library` 为 null 后被解引用；`SparseInstanceBuffer::gpu_flush` 对 null 调 `from_raw_parts`；`MappedStorageBuffer::new_storage_buffer` 必 panic。
- ✔ `render/pass_builder/shader_uri.rs:28`：`ShaderUri::parse` 取最后 `/` **之前**的部分，对 `/nani/cull_and_bucket.cs.hlsl:cs_main`、`/tgsr.cs.hlsl` 等所有真实 URI panic（子 agent 用 rustc 复现）。
- `ResourceRegistry::get`（`compute_pass_builder.rs:76-79`）把 value 读成 key 的字节；`DrawBuilder::push_snapshot` 写到自己的 Vec（构建器销毁即丢失），且第三次 draw 会 panic。

**业务逻辑错误**
- ✔ `systems/history/history_process.rs:27`：`redo` 被当 undo；中断判定永不匹配。
- ✔ `systems/save/save_game.rs:43`：键 `"snapshot"`（WP13 的 `tests.rs` 把这个错误值固定成了测试）。
- ✔ `systems/camera_rig/normal.rs:172-185`：4 个常量被改且注释声称“已对齐”；wp20 自己的证据文件里就是正确值。
- `is_halloween_any` 反了；`ActiveSession::is_winter()` 恒 false，冬季水体永远选不到；`Roof::is_flat()` 恒 true、`height()` 恒 0。
- `get_wall_attachment_height` 自由墙返回 `Ok(0.0)`（旧 `building/decorator_info.rs:83-99` 已实现）；`corner_pos_and_normal_xz` present=true 但坐标为零。
- `ColorLibrary` getter 把相隔 4 字节的偏移 ÷16，导致 `night_tint/terrain_highlight/grass_1/grass_2` 全落在槽 4；`ViewConstants` 约 0x210B、字段名为编造。
- 8/14 个 `SsboType::rg_buffer_hash` 返回 0，彼此冲突；`grid_gen_cw/ccw` 忽略 `row`。
- `WallHistoryCompoundEdit::new(ConcurrentEdits::default())` 被构造后丢弃，另 9 处只调用空函数 `History::mark_changed()`：**墙编辑不进入历史**。

**静默空实现**（`reused.rs`）：`FrozenMap::get`→`None`、`FastNoise`→0、`PerchanceContext`→0、`PriorityQueue::pop`→`None`、`AppendOnlyVec::iter`→空、`conservative_plot_line`→空、`bevy_app::App::add_systems`→空、自建假 `bevy_input`。
违反“直接复用原依赖”约束；使资产库查找永远失败却不报错。

**结构性问题**
- 全部未恢复资源合并为同一个 `Gap` 资源：6 个系统同时持有 `ResMut<Gap>`+`Res<Gap>`（`time_of_day`、`clear_canvas`、`reset_input_state`、`calculate_input_delta`、`input_record_replay`、`translate_input_to_cursor_moved`），注册即触发 Bevy B0002 冲突。
- 82 个结构体名重复定义（`GameTime`、`UiState`、`Modifiers` 等）；`AppMode` 三个互不兼容版本（history 版 `{Unknown,Replay,Other}` 为编造）；`ResourceHandle` 三套（`rhapsody_contract`、`ssbo::rhapsody_boundary`、`rhapsody-rebuild::rg`）。
- `rhapsody` 路径依赖已声明但**零使用**；`rhapsody_contract::PassBuilder` 与 rhapsody-rebuild 的 `PassBuilder<'graph>` 不兼容。
- 缺口：`History`/`EditType`/`SessionMeta` 无 serde，真实存档（`history.json`+`meta.json`+`snapshots/*.snapshot`）无法读写；`structure_outline`（23,774B）、橡皮擦 finish（21,197B）、`wall_splitter`、`merge_walls`、`finish_wall` 仅剩调用链草图；`utils::curve::Curve` 是 `{raw:Gap}`；无墙体几何生成。
- 比旧独立 crate 更差/未整合：`building/decorator_info.rs`、`building/wall_holes.rs`、`history/transaction.rs`、`terrain/tg-terrain`、`roof/`（≈2.5k 行）、`critters/`（duck/fish/butterfly/wren）。

## 5. 渲染基础 + 渲染帧：依赖分析（`calls.py` + `layers.py`）

方法：对 1,252 个渲染相关函数反汇编抽取直接 `call/jmp`（8,811 条边），与仓库 `callgraph-full.json` 在函数级交叉核对约 87% 一致（差异主要在尾调用与函数内部跳转）。
组件级最初出现一个把所有层绕在一起的大环，查明是**假象**：泛型实例回调 trait 方法（`ComputePass::new<T>`→`T::as_compute_pipeline`、`new_storage_buffer<T>`→`T::default_value`，共 20 条边）+ 3 个小辅助函数/`mesh_atlas` 分类过粗。修正后为严格 DAG。

### 5.1 自底向上层次

| 层 | 组件 | 函数 | 体量 | 独立单元 | 依赖的自有层 |
|---|---|---|---|---|---|
| 0 | A1 ssbo/buffer 基元 | 156 | 92 KiB | 10 | 无 |
| 0 | A1 mesh/vao/杂项 | 58 | 31 KiB | 41 | 无 |
| 0 | A1 shader_uri/shader 程序 | 10 | 6 KiB | — | 无 |
| 0 | A1 uniform/frame/view 常量 | 3 | 3 KiB | — | 无 |
| 0 | A4 帧 pass 辅助（clear/copy） | 4 | 2 KiB | — | 无 |
| 1 | A2 mesh_atlas | 1 | 3 KiB | 1 | A1 |
| 1 | A2 sparse_instance_buffer | 34 | 36 KiB | 11 | A1 |
| 2 | A3 资产库（SSBO/shader/mesh/纹理） | 324 | 173 KiB | 82 | A1、A2 |
| 3 | A4 pass_builder | 341 | 220 KiB | 65 | A1、A3 |
| 4 | N nani（GPU 驱动） | 157 | 152 KiB | 68 | A1、A3、A4 |
| 5 | D draw_passes | 63 | 121 KiB | 61 | A3、A4、N |
| 6 | C compute 帧 pass | 77 | 199 KiB | 67 | A2、A3、A4、D |
| 7 | S 风格化/预览/LUT | 21 | 47 KiB | 21 | A3、A4、C |
| 7 | H 帧编排器 `render_world`（`tiny_glade` crate） | 3 | 20 KiB | — | A4、C、D |

层内（泛型去重）同样无环：A4 pass_builder 子层 49/11/5（3 层，是整条链的瓶颈）；A3 71/10/1；N 43/24/1；D 56/5；C 62/5；S 20/1。
上层（C、D、S）几乎全是互相独立的叶子，只有个位数函数互相调用。

注意：做“渲染帧 pass”（C）必须先有 D（draw_passes），D 又依赖 N（nani）；但口径很窄（D→N 仅 4 条边，C→D 仅 5 条边），可先只恢复被调用到的最小接口。

### 5.2 各层需要先备好的外部依赖
A1 ssbo：`rhapsody` 391 边、puffin、bumpalo；A3 资产库：`frozen` 174、`parking_lot` 184、`rhapsody` 173、hotwatch；A4：`rhapsody` 488、bevy 161、`vk_sync_fork`、`shader-pipeline`；C：`rhapsody` 274、bevy。
→ 动手前须先：① 接入真实 `frozen/parking_lot/puffin/bumpalo`（离线缺，需联网环境）；② 统一 `rhapsody_contract` 与 `rhapsody-rebuild`、合并三套 `ResourceHandle`；③ `shader-pipeline` 是原作本地分叉，只能保留契约接口。
渲染范围外的 country_core 依赖很少：`texture_presets`(7)、`load_shader_into_library`(6)、`color_id_roof`(4)、`load_mesh_into_library`(3)、`walls`(2)，可先桩占位。

### 5.3 建议顺序与验收门槛

| 步骤 | 内容 | 验收门槛（通过后再往上） |
|---|---|---|
| 0 | 修已知缺陷 + 接通外部依赖 + 统一 handle | 能编译；`ShaderUri::parse`/`ShadowedBuffer` 等清零 |
| 1 | A1：ssbo/buffer、uniform、shader_uri、mesh | 结构体大小/偏移与 PDB、反汇编一致；单元测试 |
| 2 | A2：mesh_atlas、sparse_instance_buffer | `gpu_flush` 合并行为与反汇编一致 |
| 3 | A3：资产库（先 SSBO 库） | 查找不再返回空；`ColorLibrary` 槽位与 HLSL 一致 |
| 4 | A4：pass_builder（按 3 个子层） | 绑定顺序、push constant 与 shader-source 寄存器逐项一致 |
| 5 | N / D：先最小接口再补全 | `GpuWorldSubsetKind` 用真名；绘制 pass 绑定与 HLSL 一致 |
| 6 | C：compute 帧 pass（按家族） | 与 shader-source 绑定表对照；lavapipe 可执行 |
| 7 | S + H | 40 个 pass 顺序与反汇编一致；整帧可跑 |

### 5.4 工程量估算（估算，非实测；误差约 ±2 倍）
单位：新产生/读取的 token。步骤 0+A1+A2 100–200 万；A3 150–250 万；A4 150–250 万；N+D 350–600 万；C 350–700 万；S+H 100–200 万；合计约 1,200–2,200 万
（只做 N/D 最小接口则约 850–1,600 万）。多 agent 重复读上下文后的实际处理量约为其 3–5 倍（多为缓存命中）。
估算依据：本次审计 8 个子 agent 共约 120 万 token 核实 62 条合同结论，平均每条约 2 万；按“每个独立单元 1–8 万”并计入返工。
建议先做一个垂直切片（`gpu_generate_draw_lists` 11.5KB + `render_world` 前几个 pass，约 60–100 万 token）校准真实速率。

## 6. 本报告的局限
- 抽查而非全量：62 条合同结论、约 16 份证据文件；未抽到的部分可信度未知。
- 依赖图只含**直接调用**；209 处间接调用点（vtable/`dyn`，其中 89 处在资产库）未解析。
- 模块挖掘度按“地址出现在 Rust 函数注释/函数体”映射，未写地址的实现不计入；“solid”不代表逻辑正确。
- 本机离线且缺依赖（`dolly` 等），**未能复现** `cargo check`/`clippy` 的“0 错误”；缺陷依据静态阅读与反汇编，部分（如 `ShaderUri::parse`）由子 agent 用独立 rustc 小程序复现。
- 地址表由本次独立解析 PDB 得到（`pdbprocs.py`），与仓库现有符号表相互独立。
