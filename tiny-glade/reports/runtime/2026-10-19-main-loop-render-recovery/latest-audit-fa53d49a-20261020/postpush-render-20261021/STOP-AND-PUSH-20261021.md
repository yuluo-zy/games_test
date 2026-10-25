# 停止实现与当前成果冻结（2026-10-21）

用户明确要求“不继续了，整理当前内容和进度，全部推送”。已停止功能实现、原版取证和 GPU 试跑，所有业务/渲染/审查智能体只交付最终冻结导航。提交保存已合入生产源码、尚未合入的候选、原证、独审、成功与失败日志；**不标记整项目、完整启动或可玩闭环完成**。

本轮本地基线为 `08ecee0011d9449ebe79dffcd2f9388c490d2f53`。提交前发现远端新增 `fe772f685`、`6e5c0ca75`、`f72938c81`，涉及墙分段与 indirect draw；将用正常合并保留两边历史，不强推。精确本地执行输入、日志 SHA 与已编译测试二进制身份见 [STOP-FREEZE-RESULTS-20261021.json](STOP-FREEZE-RESULTS-20261021.json)。该身份明确属于合并前快照，合并后的构建单独记录。

## 当前能够证明的内容

| 链路 | 实际结果 | 验收边界 |
|---|---|---|
| 原资产世界连续 30 帧 | 在 sampler V3 冻结版本实际通过全部原断言，52.56 秒；live 3042–3048，samplers 72；完成点无退役积压；像素收敛门通过 | 不是数小时长稳、完整画面或最终冻结/合并后版本的 GPU 验收 |
| GameLoop 双 App 与 WSI | 另一次实际 loading 525、world 13、native world present 13，保存 worker/GPU 正常关闭 | 使用 GameLoop 验收入口；不替代完整进程入口及用户点击/编辑 |
| 加载期间关闭 | 实际原生 present 后，普通加载关闭及 Suspended→Close 两种模式通过，准备任务取消/join 在 LoopExiting 前完成；旧挂起关闭负例失败已保留 | 暂停/零尺寸/全屏/resize 全矩阵未执行完成 |
| 描述符与首次 readback | sampler V3 实际 8 项、readback 最终 9 项通过 | readback 首次清零为已标记的平台工程策略，不伪称原 Vulkan 分配保证为零 |
| 业务库 | ICC 合入前 core 2853/0/1 ignored；最终 decorator 402/0/2 ignored；utils 167/0/0 | 不等于尚未挂载的视觉宿主和完整业务已完成 |
| ICC 核心 | 原锁定 lcms2 5.6.0 / sys 4.0.1，静态 LCMS 2.15；最终 8 项 CPU 通过 | Active ICC 的真实 GPU Some/替换/取消候选未合入、未运行 |
| ICC 数值参考 | 独立 C API 实际执行；序列化 sRGB 的全 32³ LUT 指纹 `a4368867549a25f2`，与两个内存 profile 的严格 identity 控制区分 | 修正的是错误测试夹具，未放宽误差或改变生产变换 |
| 平台 ICC | Windows 返回长度 4 项、零尺寸资源门 1 项 CPU 通过；真实 Mac layer type/sRGB CFEqual 与所选 swapchain 色彩身份通过 | Windows monitor dirty V3 仍是候选；Mac 观测仅构造时采样 |
| 性能观测 | 可选 recording CPU probe 5 项通过；GPU timing observer 已编译 | 没有 release FPS 或最终渲染性能结论 |
| 最终提交前构建 | tiny-glade lib 与两个指定集成测试 no-run 实际通过，1 分 22 秒 | 构建成功不算完整游戏启动成功 |

## Logo 卡住：当前最重要的未完成项

用户确认看到的是本 agent 打开的厂商 Logo/加载窗口。真实 `tiny-glade-host` 完整进程入口已另外执行，不以三帧 loading-only 或只构建成功代替：

- 带 600 加载帧上限的一次运行退出码 0，但 gameplay 为 0，后台初始化尚未完成；明确判为**未通过完整启动**。
- 取消该帧数截断的运行，最后观察到 completed/loading 4470、gameplay 0；最终退出码 1，在 `render_frame.rs:303` 的 `device_ready` 门断言失败。完整日志 [tg-full-process-unbounded-v3-20261021.log](tg-full-process-unbounded-v3-20261021.log)。
- 当前宿主错误退出后没有阻止已经排队的 Redraw 再进入关闭后的 renderer；已发现该终态缺口，但**尚未添加修复或验证**，最初加载/渲染错误仍未完整分解。
- 两个实际 bin 在入口设置 `MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS=1`，此前 GameLoop 测试没有这一入口操作。这仅是已发现的配置差异，尚未证明是故障原因。R32G32_UINT blending 警告也不能单独证明根因，不据此擅改原混合策略。

因此当前不能承诺从该入口正常进入可编辑游戏。下一轮应先让首次错误完整保留并处理退出终态，再沿 LoadingGameComplete→世界预热→结果发布→Logo.finish_wait→SubApp 交接逐段定位；不是强制跳过 Logo 或设置假的 ready。

## 已保存但尚未完成的闭包

1. 原四态 Adaptive/VRAM/TGSR 分辨率算法阶段一已有独立原字节审查，仍未合入；预算来源、真实 cache/temporal/LUT 失效、OOM 与三种 extent 消费的阶段二尚未形成完整源码包。当前 resize 兼容写法尚未完整替换。
2. 综合 window_cycle 验收已合入并编译，覆盖真实 OS resize/全屏往返及明确注入的暂停/零尺寸/focus 语义分支；停止前未执行，不能算通过。
3. Windows monitor dirty V3、真实 ICC Some GPU 验收保留候选；尚未原子合入和执行。TAA/TGSR、DoF、Painterly、Vermeer、UI、截图等尾链的全部分支原证与视觉验收未全部完成。
4. Gothic 完整主算法/hat、变换 cache、Sparse writer、Tree 缺省、结构图已合入；canonical Gothic ECS 三源、楼梯 typed lifecycle 六源及必要几何 helper 已冻结待集成。真实视觉宿主还缺完整楼梯/屋顶瓦片等 payload，不能提前挂载旧简版。
5. 欢迎/新建入口原版合同已锁定，原 MenuFlowPlugin 与独立 welcome/new_session/daily 消费者、NextSession 文本模板拥有分支尚不完整。它们属于 Logo 之后的缺口，不作为这次卡 Logo 的直接解释。
6. 原 33 份存档全量检查仍失败：entry54 的 Wall anchor 数值断言未闭合。Terrain 三个原生 f32 叶按全部 32 位严格比较的窄修正及 3 项负例已通过；未将其它数字统一降精度或删除字段断言。
7. 实际编辑→保存→退出→跨进程重启恢复尚未整体验收。没有给整个项目报虚假完成百分比。

## 冻结导航

- 核心业务：`TG-347c9d16cc440d611d/core-tree-chain-20261021/visible-stage-20261021/STOP_FREEZE_STATE_20261021.json` 及同目录 `STOP_HANDOFF_20261021.md`，汇总 canonical Gothic、楼梯 lifecycle/几何、未实现 NaniFloor 等；候选保留 before/after hash、独审与未执行状态。
- 渲染尾部：`TG-10fd53e1e286bb43af/render-tail-icc-20261021/FINAL_NAVIGATION_STOP_20261021.json`。
- 窗口/独审：`TG-c459c7f73345e24709/main-chain-boot-20261018/window-lifecycle-20261021/FINAL_REVIEWER_NAVIGATION_STOP_20261021.json`。
- 装饰物/菜单入口：`TG-10dd22b74858bd4c2e/decorator-chain-root-20261021/STOP_FREEZE_FINAL_STATUS_20261021.json`。

上述 TG 路径均相对 `reverse-engineering/tiny-glade/skeleton/task-index/work/`。旧错误候选和失败日志保留其过期标记，不与最终候选混用。用户原有 `startup_sheep.rs` 字节 SHA `b504d8977d44877eab4332f6afd7997a0e924077d44da2d7ea68a6736feb3786` 保持不动，未纳入本次提交；构建目录和大型可执行文件不入库，执行身份与实际日志已保存。
