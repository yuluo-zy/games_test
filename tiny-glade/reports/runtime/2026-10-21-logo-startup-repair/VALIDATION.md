# Logo → gameplay：验证记录（2026-10-21，持续更新）

范围仅为真实启动、窗口、Logo/渐显/UI输出、退出生命周期及直接相关资源准备。T13反馈捕获不推进；不改写 rhapsody，不跳Logo，不制造加载完成事件。Git基线为 `1f73dbc73fcaae69711eefe84ee423ddbaa671b8`。

性能统一遵循仓库根 `PERFORMANCE-STANDARD.md`：60 FPS最低门槛、120 FPS才合格；当前约2 FPS明确不达标，功能截图不等于性能验收。

## 已完成的验证

- 原全量 gameplay 初始化和真实GPU warmup通过后台SubApp完成；交接仍要求真实LoadingGameComplete、GPU完成和Logo最终阶段。
- 本机Intel UHD630缺少swapchainMaintenance1。默认Strict仍报Unsupported，未降低其要求。仅显式 `TG_DIAGNOSTIC_RETAIN_WSI=1` 使用库中已有RetainUnproven真实呈现模式，保留无法证明完成的owner；这不证明正常关闭成功。
- `tmp/logo-startup-diagnostic-process.log`：原加载、预热、Logo完成、owner交接，随后60个gameplay帧和60次present入队。CloseMainWindow后返回Unsupported；首次终态错误后无新增绘帧日志。60次入队不等于60次最终呈现完成。
- 该次480×270诊断不是视觉验收：用户观察到Logo黑区、UI缺失和模糊，已作为待修问题。后续可见验证使用960×540强制窗口模式。
- 正常分辨率GPU读回 `tmp/logo-startup-world-960.png`：960×540，mean_luma=129.3，98159种颜色；原阈值未降低。截图位于UI叠加前，因此不证明菜单正常。
- 原loading全合成读回 `tmp/logo-loading-fixture-960.png`：960×540，原背景正常，Logo区域出现约243×243黑矩形。原指令确认两个UI list pass漏了byte3混合位写入；按原设置恢复后，`tmp/logo-loading-fixed-960.png` 黑像素比例11.38%→0%，alpha范围0..255→255..255，原背景保留，CPU位域回归及GPU回归通过。fixture为原Logo进度0、单次Time=1/60及手工文字，不代表完整真实启动时间线；“有非零RGB”不足以验收。
- 原有聚焦CPU检查：生命周期14、窗口5、handoff3、preparation10、world extent6通过。新的背景异步准备9项通过，覆盖真实PNG数据/alpha、缺图、冻结首选择、pending、取消、异常及结果释放。
- 最终增量CPU检查：frame_tests 15、preparation 13、loading_graph 12、handoff 4、Welcome 6、原UI混合状态1通过（组间重复项不累加为新测试数）。Welcome测试使用真实四模板、真实确认键consumer和保存事件，未用假点击覆盖交互。
- 有存档背景的原图临时TexRef不能跨App/跨帧复用。现保存真实native Arc纹理owner，交接前校验两App设备身份、移动owner、每图重新导入并刷新引用；原reveal完成时释放App owner，提交图仍持有GPU完成所需引用。

## 当前限制与不得外推的结论

- Vermeer原算法每次有效增长最多1/90，且有前两帧门；具体启动所需帧数受迁入状态与dt影响。之前60个gameplay帧未到自然揭示结束，不能由此判定UI应该已出现，也不能强设progress/Finished来提前显示。
- 首次Welcome producer已按原三态、模板、确认键、profile owner和保存事件恢复并接Game/set36。Existing Escape/版本/设置按钮链静态已连通，但需要最终像素和交互证据。
- 保存背景图片的读取与解码已改为一个CPU worker、容量1结果队列，主帧只非阻塞查询后使用原TextureHost上传。等待时同一App帧只更新一次；OS事件、关闭和真实gameplay错误仍可处理。不宣称全启动资源均已异步。
- boot纹理/字体初始化、shader文件读取及主图全host prewarm仍存在同步边界。不能简单删除prewarm或把Timeout当成功：现图声明已推进状态，执行可能部分提交，需要完整图续跑与取消协议。
- 一组旧 `run_game::` 宽测试有6个基线fixture错误，缺TerrainHeightsData；实际生产注册路径具备该资源。保留失败日志，未为本轮扩大修改terrain/history业务。
- 正常进程panic hook会abort；catch_unwind单元测试仅证明允许unwind的嵌入/测试宿主路径。

## 诊断开关

- `--windowed --window-size 960x540`：本轮始终使用；显式CLI高于fullscreen环境值，持久配置保留原显示设置。
- `TG_STARTUP_VISUAL_TRACE=1`：有界记录同帧Vermeer/UI门、MEI数量、物理/声明目标及swapchain尺寸，默认关闭。
- `TG_STARTUP_UI_CAPTURE=<绝对路径.png>`：仅见过实际gameplay reveal活跃、原算法自然完成后再等0.5秒，使用既有FinalFrame读回链记录T21之后的最终源；不恢复T13业务。
- `TG_LOADING_SIZE=960x540`、`TG_LOADING_PIXEL_OUT=<绝对路径.png>`：原loading fixture导出。

所有tmp路径均相对于仓库根 `D:/game`。最终结果以随后完整构建、定向回归和真实窗口记录补充，当前不能标记整个视觉启动模块完成。

## 渲染顺序专项复核增量

- 独立专查重新对 SHA `f507…` 的原 B 解码，覆盖调用顺序、状态位和颜色附件来源；不能用 retained decomp 中附近的其他枚举值代替实际传入参数。
- Loading 原调用 `0x140165347` 清零第四参数，MEI 应直接绘制而无阴影临时目标。已移除错误的加载页阴影分支。实际 GPU receipt 验证 `Vermeer combine < Mei 2D < UI`，且无 `UI shadow combine`；960×540 全合成读回黑像素比例0、alpha全255。日志 `tmp/logo-loading-order-960.log`，图 `tmp/logo-loading-order-960.png`。该 fixture 仍不代替真实自然 Logo/Welcome 时间线。
- T11 存档预览原 `0x1408f83b0` 写入 tag0，`0x1408f8404` 将同一附件地址传入创建器：恢复 LOAD 及相应读写依赖。
- T12 LUT gallery 原 `0x1408b0b13` 从 `0x142a9cc30` 读取 tag1/RGB0.18，再于 `0x1408b0b23` 写 alpha1：恢复原清屏颜色，通过现有 ColorAttachment::View 传递精确位模式。MenuBG 原为 tag2，保持不变。
- 上述 T11/T12 是原证和代码修正，尚不能据此宣称两个菜单分支的实际 GPU/交互验收通过。重新构建及独立复核进行中。详细原证见 `render-state-evidence/manifest.json`。

## 并行工作边界

用户新增独立业务中央 agent，按 toolbar→地形→建筑推进并管理子 agent。各模块必须分别交付真实输入、权威状态/实体/碰撞/画面、历史与保存、生命周期清理、原证与实际链路五项验收。该线报告位于 `../business-module-recovery-2026-10-21/`。本报告仅记录渲染/启动线，不用联合验收或共享渲染成功补记业务模块完成。

## 16:34–16:36 真实存档启动结果

- Release 可执行 SHA256 `3FBBDEA8FF80CA8B74208FA27851EF8E8272EE66A3F61BDB90791047CC1083A7`；构建日志 `tmp/logo-visual-release-order-build.log`。显式960×540窗口覆盖 TG_FULLSCREEN=1，未跳 Logo。命令/环境见 `tmp/run-logo-visual-window.ps1`。
- 真实存档恢复 walls=3、trees=281、raster=6，并走有保存背景的加载分支和原 owner 交接。803个loading帧后进入gameplay；第92游戏帧原Vermeer自然完成，第93帧UI门开启，83个UI元素、341个MEI图元进入绘制。
- `tmp/logo-visual-real-ui-960.png` 为自然reveal后、T21后FinalFrame实际读回；中央已目视确认欢迎面板、确认按钮、右上角设置/版本和世界共同显示。正常main/depth/TAA及最终目标均960×540，最终目标4×MSAA，resolve源与交换链同尺寸；没有把32×32预热目标用作正常画面。
- 本次累计143个gameplay帧、143次提交和143次present入队。正常窗口关闭请求后返回硬件呈现完成证明Unsupported，错误后没有新增绘制。仍不证明Strict/正常退出通过，也不证明物理扫描输出完成。
- 真实性边界：此截图不证明鼠标可交互、完整菜单、T11/T12激活分支、窗口resize连续性或像素与原版完全等同。业务中央已发现真实picker缺口，正在单独恢复。此可执行也不覆盖随后W34目标修复、shader异步拆分或新性能诊断。
- 性能仍不合格：游戏主循环实测约2 FPS。用户同时观察GPU未满，不能据此猜单一瓶颈；下一步保持分辨率/完整算法，使用TG_FRAME_TIMING区分App.update、图声明/prewarm/compile、record/submit、acquire/present、队列节流与limiter。

原始日志、图和退出边界的哈希/事实见 `real-window-result.json`。
