# country-core 优先模块推进协调简报（2026-10-15 第二轮）

本简报定义三个隔离组的模块/文件所有权、Windows 逆向工具链入口、证据位置和验收纪律。
目标：把 9 个 `implemented_bounded` 优先模块往"完整业务闭包"推进，逐模块复核去水分。

## 总则

- 目标 B 版 tiny-glade 的 core 业务一致性；全部新增逻辑落在 `D:/game/tiny-glade/crates/country-core/`。
- **文件所有权隔离**：每组只写本组拥有的文件。对组外文件（如共享资源 `resources/session.rs`、`resources/glade_settings.rs`、`resources/history/*`、`systems/history.rs`）的任何修改，必须经中央 agent 仲裁后再动。
- **禁止并发放同一个文件**。跨组需要同一文件时，由中央 agent 定序。
- 已经确认的 derive/依赖胶水/Bevy 注册直接复用，不计业务；自定义默认值/字段策略/serde 属性/事件仍是业务。
- 验收纪律：编译通过 ≠ 模块完成。必须：真实业务代码 + 真实消费者 + 原机器码证据 + 诚实的 remaining 标记。禁止伪造加载/成功/空壳。

## 工具链（Windows 本机，实测入口）

- RE-MCP 反编译：
  `node D:/game/reverse-engineering/tiny-glade/scripts/mcp-job.mjs re_mcp_ghidra <job.json> <out.json>`
  job = JSON 步骤数组。需 open_database{file_path:"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe", run_auto_analysis:false, database_id:<唯一id>} + wait_for_analysis + close_database{save:false}。预分析 116,270 函数。catalog 落盘 `evidence/mcp/re_mcp_ghidra-catalog.json`。
- 原字节/字符串直读（快）：
  `python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py <va>|--fat <va>|--hex <va> <n>`
  RNE 二进制 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 权威函数名→地址（v2）：`evidence/country-core-deep/sysres-procedures-v2.json`（1,599 条 country_core systems/resources 过程）。旧 v1 有系统性错位，只作线索。
- 每文件 PDB 元数据：`skeleton/B/crates/country-core/src/**/*.rs` 头注释（函数符号、va、依赖注解）。
- Rust 逆向关键事实：`evidence/country-core-deep/PLAYBOOK.md`（游戏 Vec 是 cap-first {cap@0,ptr@8,len@0x10}、i64::MIN 哨兵、enum niche 判别、Arc 模式等）。
- 内存约束：32GB RAM，空闲波动大；MCP 批量用 `--lock` 全局串行。MCP 反编译不开新项目锁冲突：先查既有作业是否在跑。

## 分组与文件所有权

| 组 | 模块 | 拥有的源文件（相对 crates/country-core/src/） | 历史证据入口 |
|---|---|---|---|
| A 植被 | ivy | `resources/ivy/*`、`systems/ivy.rs`、`systems/ivy/*` | evidence/core-recovery-20261015/ivy/（grower.asm.txt 66KB 等） |
| A 植被 | garden | `systems/garden/*`、`resources/glade_border.rs` | evidence/core-recovery-20261015/garden/ |
| B 生物 | sheep | `resources/critters/*` | evidence/core-recovery-20261015/critters/ |
| B 音频 | music | `systems/audio/music/*`、`systems/audio/mod.rs` | evidence/core-recovery-20261015/music/ |
| B 输入 | input | `systems/backend/input/*` | evidence/core-recovery-20261015/input/ |
| C 世界 | terrain | `systems/terrain/*`、`resources/terrain_editing.rs` | evidence/core-recovery-20261015/terrain/ |
| C 世界 | save | `systems/save/*` | evidence/core-recovery-20261015/save/ |
| C 世界 | session | `resources/session.rs` | evidence/core-recovery-20261015/session/ |
| C 世界 | ui_clutter | `systems/ui_clutter/*` | evidence/core-recovery-20261015/ui-clutter/ |

B 组 SPEC 目录名沿用 `B-creatures-audio/`，input 的 SPEC 也放该目录（input-SPEC.md）。

共享但需仲裁的文件：`resources/history/*`、`systems/history.rs`（history 账本；ui_clutter 与 terrain/garden/save 都可能消费）、`resources/glade_settings.rs`（ivy 读 +1c 旗标）、`resources/session.rs`（music 读 ActiveSession）、`resources/audio*`/FMOD。修改前发消息给中央 agent。

## 每模块剩余缺口（合同 contracts.json，2026-10-15）

- **ivy**：完整 grower/root spawner、wall-change point-validity pruner、root/branch 删除策略、UI/ActionSet 门、wall jump/hole 约束、GPU 上传/渲染（GPU 后置）。`ivy_leaf_spawner` 需接 GladeSettings +1c 真实旗标。
- **garden**：完整 UI 仲裁/其他 WidgetParams 变体、signifier 所有者、原全局调度、56 字节 ShowPinnedDecoClutter union 与 renderer、下游 garden scatter/audio 消费者。新 GardenPinnedClutterCircle 只是观测到的圆/高度投影，不算完整 pin 系统。
- **sheep**：完整 Sheep owner 与生命周期、真实 Sheep query 所有者、save/load 接线、spawn/movement。
- **music**：完整 GladeAudio track producer、AudioCore/device/bank 启动、live FMOD 集成（FMOD 依赖 libfmod 2.222.6，原锁 checksum 75defbe5…）。music_player::load_tracks 已实现但完整 GladeAudio 加载消费者未恢复。
- **input**：键盘/Steam 映射、reset/replay 集成、update_cursor_position、原全局调度、OS 事件生产者。
- **terrain**：terrain UI/new-stroke 输入生产者、原 plugin 资源初始化与全局调度、terrain history/replay 生命周期、audio 播放、GPU 执行（后置）、下游 wall/collision/scatter 订阅链。
- **save**：完整 AsyncSaveCmd 变体、deferred 命令、主后台保存循环/unsaved-file 重试、每私有域的游戏线程捕获/序列化、快照二进制捕获/load、自动/手动备份生产、session 切换自动跑 GC、完整 save-session 生命周期。
- **session**：完整 NextSession(160B) 所有者、GameSaveData(240B)/MetaCommand/SessionOnLoadCmd 派发、SessionLoadAttempt、完整 world load/commit 事务。空 Vec 是 cap 0 + dangling ptr 8；kind 五条 selector 路径；`create` 0x140b46960 复制 metadata + 24B 可选串/路径 + 字节旗标；`append_on_load` 0x1403b5150 三个 56B MetaCommand → VecDeque → 32B SessionOnLoadCmd。
- **ui_clutter**：原 RMB picking、parent/roof transform 解析、auto→decorator/stair 转换、UI 交互、消费删除 ID 的 generation/mesh owner（渲染消费；owner 后置可先留真接口）。

## 中央 agent 职责

- 台账：`evidence/core-recovery-20261015-adv/PROGRESS.md`（每模块状态/证据/剩余）。
- 接收各组进度同步消息，仲裁组外文件修改请求。
- 不代做实现；发现水分向父报告。

## 交付物格式（每组每模块）

- 历史分析 agent 产出 `evidence/core-recovery-20261015-adv/<组>/<模块>-SPEC.md`：
  要实现的剩余业务契约、原地址/机器码引用、真实消费者、待分类项、明确排除项。
- 实时复合 agent 按 SPEC 实现；实现后写 `evidence/core-recovery-20261015-adv/<组>/<模块>-DELIVERY.md`：
  实际接线 / candidate 链测试 / 原侧验证 / 仍缺项，逐项分离。不得用编译通过冒充原侧验证。
