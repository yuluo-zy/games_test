## 2026-10-19 最新中央并行验收检查点

当前上限为9个并发槽（含中央），本轮以8个专项agent并行，旧文的4槽记录仅为较早阶段。中央唯一Cargo owner，生产冻结后再验证；释放的成员动态转交屋顶UI、Load菜单、startup卸载链与独立复核。T13明确排除；其它复杂算法/完整交互不可简化。

独立执行：音频真实生产源43/43生命周期测试通过；MaterialStats对原B实际执行530/530位模式与selector匹配。v14实际GPU Egui单样本及原sRGB4×MSAA两测试通过；capture及实际gameplay/历史测试此前失败，已给候选修复但未重跑，不算通过。新roof原RNE28组执行不等于Rust数值验证通过。

已落但待统一验收：历史9/16变体身份、plaster/support/AutoClutter/Deprecated Decorator保存及nullable NaN往返；完整旧版本迁移、identity权威流与snapshot cursor；timelapse生命周期；Escape真实GUI及OS命令接线；roof tile/spire/trigger；墙typed GPU分配与样式消费者。Load菜单和颜色show/main、roof UI仍在恢复。原支撑梁只有静态零count channel证据，不编造非空producer。

最新发现的完整性断点：第二次同主题会话placement库重复load，需恢复整个unload/despawn生命周期；<19预置树事件有原证但消费者组件归属颠倒（原PrefabInstanceData48B、PrefabInstance88B），不能只注册writer宣布完成。最大缺口仍输入→权威→历史→实体/上传→实际绘制。

升级flag裁决：原wrapper d→bit40/window（GateColors及非邻接冷块三窗Decorator色），e→bit48/ArrowSlit999。两次独立核实完整CFG，不能因旧helper命名或截断连续反汇编再次翻转。webbrowser实际原锁是0.8.10，已按原依赖复用。

# 2026-10-19 三主流程继续恢复

用户明确排除 T13 反馈捕获/状态生产者；本轮不调查、不恢复、不以此阻断其它路径。其它路径仍遵守原逻辑/算法不能简化、保留最新 rhapsody/工具架构的约束。

## 中央协调与新一批 agent（用户追加要求）

本环境最多同时运行4个agent（包含root）。已结束的成员交付报告后释放槽，再创建新模块成员；不得将旧成员/排队任务的数量冒称同时并行数。root负责接口裁决、共享文件集成、单一Cargo owner与验收。

| 新成员 | 完整调用链与交付边界 | 源码写入约束 |
|---|---|---|
| session_owner | Load解析成功→拥有ParsedGameData的NextSession variant3→startup_session/LoadGameManager；真实GladeSettings parser合同 | 当前验证期间仅读原证，准备完整patch；canonical session/parser由root集中集成 |
| roof_visual | Roof创建/修改/删除→ArchSegments/真实高度→RoofTiles/Spire/SupportBeams/Nani→Sparse上传与清理 | 当前仅读取证，完整patch进入独立evidence目录；不写rhapsody |
| menu_interaction | MEI Load/Escape UI→所有命令/存档/错误/备份/会话转移→截图事件writer | 与session_owner按UI/数据owner分界；原Timelapse启动tag2有原证 |

后续轮换队列：T16完整颜色菜单show/create/hide所有提交分支；墙constructor完整调度、MaterialStats/style与96B实例GPU链。先接完整producer/consumer与所有权合同，再进入集中源码写入窗口。重复取证和工具修复不作为业务交付。

第二批session_owner/roof_visual已交付evidence-only候选并结束，尚未合入生产；新建save_migration（完整旧版本迁移、metadata、identity权威投影）与menu_widget（完整MenuButtonContent/vtable与typedMEI接口）接力，menu_interaction继续并行。active为这三者+root。metadata升级标志的session-owner换名候选与migration独立原证发生冲突，**暂停所有flag交换hunk**，须完成writer→wrapper→load→snapshot字段消费的逐步核对后再裁决。

## 第一批交接

| 负责人 | 实际链路 | 当前工作 |
|---|---|---|
| startup_chain | 原菜单/输入 → 模式 → Walk teleport/移动；音频初始化 → bank/promise → 真实播放 | 恢复真实 DLL adapter、两阶段 bank 加载、相机入口与原调度 |
| history_chain | 原存档 → 完整 edit/aux → replay → roof/terrain/tree/color → undo/redo/save | 恢复命名载荷与完整实体生命周期，追踪视觉消费者 |
| render_tail | 菜单状态/布局 → T11/T12/T16 → GPU | 恢复缩略图宿主、readiness writer、共享布局控件与图标动画 |
| root | 真实 gameplay App/完整设备装配 → 原版场景加载 → 主图执行/物理输出；菜单/LUT真实截图生产者 | 集成调度、资源与 GPU 依赖，沿实际失败点修复并保留日志 |

`original_gameplay_chain` 使用真实 `run_game_thread`、同一生产设备装配 tuple、原发行起始场景及临时账号目录。未移除系统或替换世界以获取成功。该验收仍在推进，不能据局部通过声称全场景完整。

独立 RE-MCP 尝试将原 RNE（SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03）的副本打开到唯一数据库 `round12_capture`，禁用全程序分析。open 返回 opening；wait 返回 SpawnFailed / worker subprocess exited，工具发现为空。没有取得新反编译，也没有修改原数据库。后续复用 Git HEAD 中稀疏排除的保留反编译并直接读原 RNE 限定 LLVM 反汇编。

截图新取证范围：gather@14031af10/8740B，opportunity@14031d140/6359B，menu_bg@1408c9a30/1927B。新 asm 和从 HEAD 取出的保留 .c 在 integration/，新生产代码须经编译与真实 GPU 验证后才能记为已验收。
