## 2026-10-19 最新中央并行验收检查点

当前上限为9个并发槽（含中央），本轮以8个专项agent并行，旧文的4槽记录仅为较早阶段。中央唯一Cargo owner，生产冻结后再验证；释放的成员动态转交屋顶UI、Load菜单、startup卸载链与独立复核。T13明确排除；其它复杂算法/完整交互不可简化。

独立执行：音频真实生产源43/43生命周期测试通过；MaterialStats对原B实际执行530/530位模式与selector匹配。v14实际GPU Egui单样本及原sRGB4×MSAA两测试通过；capture及实际gameplay/历史测试此前失败，已给候选修复但未重跑，不算通过。新roof原RNE28组执行不等于Rust数值验证通过。

已落但待统一验收：历史9/16变体身份、plaster/support/AutoClutter/Deprecated Decorator保存及nullable NaN往返；完整旧版本迁移、identity权威流与snapshot cursor；timelapse生命周期；Escape真实GUI及OS命令接线；roof tile/spire/trigger；墙typed GPU分配与样式消费者。Load菜单和颜色show/main、roof UI仍在恢复。原支撑梁只有静态零count channel证据，不编造非空producer。

最新发现的完整性断点：第二次同主题会话placement库重复load，需恢复整个unload/despawn生命周期；<19预置树事件有原证但消费者组件归属颠倒（原PrefabInstanceData48B、PrefabInstance88B），不能只注册writer宣布完成。最大缺口仍输入→权威→历史→实体/上传→实际绘制。

升级flag裁决：原wrapper d→bit40/window（GateColors及非邻接冷块三窗Decorator色），e→bit48/ArrowSlit999。两次独立核实完整CFG，不能因旧helper命名或截断连续反汇编再次翻转。webbrowser实际原锁是0.8.10，已按原依赖复用。

# Round12 三主流程：恢复、纠错与统一验收

本轮持续推进中。原作正确性、完整算法和交互优先；T13反馈捕获由用户明确排除，不作为阻碍。最新rhapsody/工具所有权模型沿用，本小组不回填旧库实现。

## 经验与效率

持久经验已更新到 `skeleton/workflow-design/tiny-glade-recovery/references/original-driven-integration.md`，项目记忆包含其入口。重点是：按完整调用链分工；以原指令/完整函数长度/真实参数证明类型和调度；优先精确复用稀疏Git内已有证据；先枚举真实存档残差；集中源码冻结及单一Cargo owner；分别报告CPU、实际GPU与完整交互边界。

本轮明确纠正了Time+58、FMOD启动bool、射线solid实参、Node::used返回值、Wall变体4/5与8/12、LUT默认首格、SavedSession readiness、同帧capture读写依赖等误归属。优化不减少原业务算法或验证要求。

## 源码已落的第一批

| 链路 | 已落内容（不等于本轮全部通过） | 主要剩余 |
|---|---|---|
| 启动→装配→主循环 | 原FMOD DLL/锁定依赖、bank/promise、真实CLI同步选择、session音频生命周期、Free/Walk与摄影模式原set8/9、GUI39 | Escape真实writer/完整菜单、Timelapse生命周期、全部领域事件生产者与完整主App验收 |
| 编辑→历史→回放 | MultiEdit实际aux、更多Wall/Roof/Color/Stairs/Raster保存类型及真实回放、实体生命周期、ArchSegments/Gate数据传递 | 屋顶tiles/spire/support/Nani生产、完整constructor调度/材质/96B GPU、老版本解析迁移与加载完整交互 |
| 帧→构图→GPU | T11真实thumbnail host/readiness、T12真实UI/默认首格/共享metrics/capture owner/图依赖、T16完整动画与延迟销毁、T21原4样本Egui测试候选 | T11实际Load交互、T16完整show/create/hide提交、截图文件/Steam其它机会点、完整非空游戏帧验收 |

细节和证据：`startup/REPORT.md`、`history/REPORT.md`、`render/REPORT.md`。报告冻结后root又修复了MusicPlayer owner替换/双stop，装入生命周期回归；原pending字样不能解读为尚未应用。Node::used11B原证为 `integration/node-used-rne.asm`。

## 新一批与中央协调

用户要求增加agent后，已结束旧三成员的当前轮并创建新三成员，环境同时上限4（含root）。root只集中接口/冲突/补丁集成/统一验收，下一批分别负责：

- session_owner：NextSession第四拥有型变体、parse结果完整wrapper及GladeSettings合同、startup/LoadGameManager。
- roof_visual：真实事件实体→ArchSegments/矩形/高度→RoofTiles/Spire/Support/Nani→Sparse写入及删除。
- menu_interaction：Load/Escape完整MEI树、所有modal/备份/错误/命令/真实截图事件writer。

继续排队专项：T16完整颜色菜单、墙constructor/GPU全链、原存档旧版本迁移。详细边界见 `WORKLIST.md`。新批次在当前编译期间仅读证据并准备可审patch，避免边编译边修改共享core。

session_owner/roof_visual交付候选后，已再新建save_migration与menu_widget接力，当前与menu_interaction并行。升级flag候选仍暂停：已确定wrapper d/e分别进入bit40/48并作用于不同的subtype数值/颜色字段，但旧函数命名本身也可能颠倒。先前“现生产名称正确”的判断撤回；须再核原visitor/字段消费及原字符串，将位序与业务名称分别证明后统一裁决全部算法和命名。相关交换hunk从未集成生产，不能将中间报告当最终结论。

当前最大缺口是用户操作→权威状态→拥有数据/实体→真实渲染上传的跨crate合同；独立资源已创建、序列化成功或图能导出，都不能证明完整交互。

## 验收记录（更新中）

- v11：编译失败；构建期间新增的decorator依赖与session音频函数造成跨快照不可见，当前源码已具备二者。没有测试执行通过。
- v12：编译失败，三处canonical/projection WallId转换漏项；均已集中修正。完整退出/源码变化记录在 `validation-v12-result.json`，不记通过。
- v13：生产库完成编译，测试诊断把`&Arc<AssetShaderLibrary>`传给`ShaderNames`导致测试编译失败；已改`shaders.as_ref()`。结果101，不计测试通过；源码变化记录明确含其它独立任务的decorator变更。
- v14：集中运行全部三个验收目标（`--no-fail-fast`），覆盖真实App起始场景→生产世界图、原共享捕获shader/实际像素与owner、Egui单样本及原0x2b格式4样本输出。同一个实际core库还验证33原历史/2120条/408decorator aux/63stairs aux保存重载及双向命令展开，避免为外部资产诊断单独重编整个内部单元测试harness。结果以最终日志/退出码为准，不等于所有旧版本迁移/回放实际副作用已验收。
- 原33存档/2120条为待执行的验收预期，不能把ignored测试或编译通过当通过。

`original_gameplay_chain`使用临时账号、原发行资产及生产共用设备装配，不删除系统、不制造LoadingGameComplete。CPU墙集合及物理输出仍不等于全部建筑可见，测试保留constructor残差与实际DrawLists诊断。

## 后续独立验收增量

- v15只因新增原锁webbrowser直接依赖需更新lock而未进入编译；v16暴露新增Load模块直接依赖，v17纠正中央接线两处符号路径，v18仅剩跨crate cfg(test)夹具导入错误。夹具已改成wall-constructor测试域自己的CPU shadow device，并在误调用upload时显式Err，不改生产device、不删回归。编译/检查成功范围与整个all-targets通过区分。
- v19原roof fixture实际Cargo运行失败，case8 positions[288]差1.5858588；这是矩形cap角点顺序错误，不能放宽容差。恢复者按原SSE纠正两个ridge方向的配对后，直接rustc include实际生产源对28个原RNEcase通过，max_abs5.96e-8/354个不同f32位。阈值2e-5未改。中央Cargo复跑仍待安排；v19本身保留failed。
- v20开始统一执行6个实际integration目标（gameplay/capture/egui/history wire/Escape/Load），冻结源/命令/实际结果由validation-v20-*记录。未结束前不预记通过。
- UI拾取专项已独立分配：原hover_intersection+真实tasks closure、ui_update/get_ui_volume，set6。单有roof/color widgets不能证明点击/拖拽；不以手工interaction替代真实输入验收。

## 2026-10-19 中央独立水音复核

central-independent-water/results.json：448/448实际原SSE对候选确切函数逐bit通过。读取原1.0/1/64/42.25常量，执行1409ed2b8..2c6及1409ed355..372，无FMOD或ECS helper替换；候选函数与三常量原文截取编译，函数SHA绑定，未写另一份Rust期望公式。范围只到volume算术，不验证完整water producer/循环/卸载。

性能治理：中央v20六目标仍运行，8专项只准备候选；同机多份1–2GB Rustc造成资源竞争，已要求已运行编译正常结束后不再新增LLVM codegen，后续窗口统一调度。避免因静默日志重启有效compiler。PowerShell读取超过默认yield会返回session，必须保留ID；中央改持久Node fs读取，避免丢弃后续输出或反复起壳。

候选完整性：17文件原始base哈希核验发现save_game差异，作者核证纯CRLF/LF并修生成器改read_bytes，未覆盖生产；后续仍逐字节核基础。UiState资源候选必须先于24B PlayUiSound窄补丁，防整文件拷贝回退。
