# 2026-10-21 15:08 增量只读审计：渲染/WSI/ICC

基线 f72938c81 → 当前 1f73dbc73。只读新报告及当前源码，无 Cargo/测试/GPU/逆向/Git show。百分比是工程范围估计，不能按1241个路径算业务提升。

## 新旧差异

| 范围 | 上次已知 | 本次新证据 | 进度调整建议 |
|---|---|---|---|
| 连续世界帧/资源稳定性 | 旧30帧全执行，但live漂移或峰值断言失败；新维护待执行 | sampler V3冻结版本30帧实际全原断言通过52.56s，live3042..3048，sampler72，frame12→28 memory969166084→968647684，native9→9，退出submitted=completed1696/pending0/无额外idle；20→30像素检查通过 | 渲染生产窄链验收约75–85%→80–90%；这是30帧短稳，不能宣称数小时长稳或合并后GPU复验 |
| 真实GameLoop WSI | 旧仅loading3 native成功；完整gameplay窗口未验 | 实际loading525/world13/native world present13，保存worker/GPU退出通过 | WSI局部验收上调，约65–80%→70–85%；不提升“完整进程启动/可玩”结论 |
| 加载期间生命周期 | 完整矩阵未验 | Suspended→Close旧负例失败，修复后真实窗口通过；普通加载关闭也通过，取消/join在LoopExiting前 | 两条边已执行；综合window_cycle仅编译未运行，零尺寸/resize/fullscreen/focus/Adaptive全矩阵仍缺 |
| ICC尾链 | custom_lut=None、producer未恢复 | canonical ActiveIccProfile+event/plugin+原32³ LCMS变换+真3D上传owner+final_post Some描述/命名空间校验已合入；原锁定LCMS2.15，最终8项CPU通过并有独立C数值oracle | 尾部源码估计85–95%→90–95%；ICC独立源码约70–85%、生产闭环约35–50%。Windows监视器dirty V3和真实GPU Some/替换/取消未合入/未跑，不称ICC已完整 |
| 完整进程启动 | 未经过完整正常入口验收 | 真tiny-glade-host：限600帧退出0但gameplay0，不判成功；取消截断后loading4470/gameplay0，最终exit1/device_ready断言失败 | 不上调；完整启动明确仍失败，不可把GameLoop专门入口通过替代完整可玩 |
| rhapsody整体 | 85–95%源码、75–85%生产成熟度 | 静态sampler目录/读回可移植初值/录制探针等新实现与窄测；sampler8、readback9、probe5执行 | 源码仍85–95%，生产成熟度可收敛至80–90%；完整硬件/长稳/性能矩阵仍未完成 |

## 最大当前断点

STOP-AND-PUSH-20261021.md:22–31明确真实正常进程卡厂商Logo/加载。加载4470帧无gameplay，最后render_frame.rs:303 device_ready断言；错误退出后排队Redraw仍可进入关闭renderer，终态缺口未修。最初加载/渲染错误没有完整分解；MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS入口差异仅候选原因，不能定因。应核LoadingGameComplete→warm-up→channel publish→Logo.finish_wait→SubApp交接，不用跳Logo/伪ready。

## 证据路径与行号（均 D:/game/ 下）

统一报告根 R = tiny-glade/reports/runtime/2026-10-19-main-loop-render-recovery/latest-audit-fa53d49a-20261020/postpush-render-20261021/

- R/STATUS.md:9–15：30帧/13WSI及实际数值；:17 source/binary身份；:21 sampler策略，:23首次GpuToCpu/None零初值属于明确工程策略，不是原Vulkan必为零；:29 debug无最终FPS；:38、45真实入口边界。
- R/ACTUAL-RESULTS-V2.json:2–34：actual_gpu_gates_passed_not_whole_game_complete；30帧与WSI结果。
- R/STOP-AND-PUSH-20261021.md:5：冻结身份合并前；:11–20各项通过边界；:22–31完整进程失败/终态缺口；:35–41 Adaptive、window_cycle、ICC候选、视觉宿主、菜单、存档及可玩未完。
- R/STOP-FREEZE-RESULTS-20261021.json:5：local premerge scope；:232–245最终premerge no-run、ICC8项以及history失败；不得写成当前1f73合并后实测。
- R/tg-full-process-unbounded-v3-20261021.log:483–484：真实device_ready panic，保留失败。

当前代码：
- tiny-glade/crates/country-core/src/systems/backend/icc.rs:13–18尺寸/格式/event；:21 IccTextureDevice；:38 ActiveIccProfile；:55–61 plugin；:65–91原整数网格/alpha255/完整LCMS；:94–124最后事件、先drop旧Some、真实3D纹理上传及失败None。
- tiny-glade/crates/tiny-glade/src/run_game.rs:354与1100安装IccProfilePlugin；render_bridge.rs:200注入同App真实TextureHost。
- tiny-glade/crates/tiny-glade/src/world_postprocess.rs:1132–1166已读ActiveIccProfile、namespace检查、D3/32³/格式0x5b/sample1/sampled校验并绑定Some，:1177传custom_lut。旧“None未恢复”已不成立。
- 全tiny-glade/src对IccProfileChanged生产者检索无实际event写入；event_loop当前未有ICC monitor dirty路径，和冻结报告V3候选未合入一致。不要以核心事件consumer存在算Windows自动变更闭环。
- tiny-glade/crates/tiny-glade/src/render_host.rs:875–1259静态sampler目录、版本/同锁失效、预算不足保留旧路径；:1369资源组准入后补目录；:4137后实际测试入口。
- tiny-glade/crates/tiny-glade/src/event_loop.rs:165–174挂起Close直接dispatch以取消join；其它终态缺口保留报告结论。

## 不变项

T11/T12/T16/T21已接、T13明确排除；完整场景producer、楼梯网格/视觉宿主、全部尾链分支原证与原作同机位画面对照、Windows实机、完整交互/编辑/保存/跨进程恢复仍不完整。Nani和shader完整协议残差本次未有可把原估计上调的证据。整个“帧→图→GPU”可用约80–90%表达窄链工程成熟度，但“从正常程序启动进入可玩游戏”仍失败，不建议将两者合为单个高分。
