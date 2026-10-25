# 三条主流程的原证驱动增量恢复（2026-10-19）

## 同步与约束

先 fetch 并将 main 从 `f3e97ecf` 快进到 `1253c0ab7cc9706c58c4e57c3a61a0b17c7212f2`，共62个远端提交。拉取前186个本地文件、SHA256与完整diff保存在相邻 `sync-20261019-local-backup/`；四个重叠文件逐个三方合并，未用旧文件覆盖新API。历史Mac证据文件名与稀疏checkout令Windows Git刷新索引时截断条目：从HEAD独立重建完整索引并恢复原排除目录6855项skip-worktree后，复核staged差异为零；仅本仓库设 `core.protectNTFS=false`，原稀疏配置保留。不改原始发行文件，没有提交或推送。

当前 `tiny-glade/crates/rhapsody` 与拉取后的HEAD无源码差异；保留其现有设备/资源/图/receipt/预算与呈现设计。原版用于校正业务算法、状态、副作用与调用者，不恢复旧底层后端。

用户新增最高优先约束已写入根AGENTS、PROJECT_MEMORY与专用恢复经验：首先保证游戏逻辑、算法和设计正确，原复杂业务和算法不得简化；允许多态等设计更新，但必须保留完整交互。以下局部交付不等于三条流程全部完成。

## 启动 → 装配 → 主循环：仍为部分

- meadow按实际session资源根读plants与48B placements，真实buffer/mesh/shader→drawable→同App placement绘制；修复重复拼接资源路径、空placement快照与虚构MainCameraView。
- `load_glade_grading`恢复真实三份ImageFormationConfigs写者，含主题JSON、I/O失败与解析失败的不同分支、Halloween2024常量和saved override只覆盖slots1/2。GPU LUT由独立真实GradingLuts加载，所有名字/纹理共用设备owner；固定键重新证实为`tony_mc_mapface`，画廊图标为`menu/active_lut`。
- 主相机复用已冻结、全PDB长度原证包，纠正13槽/Modifiers/快照状态/动作字段误读，恢复六body、Normal平滑与orbit、Free/Walk约束、快照、set9/set10、真实world ray/terrain/MouseDepth；完整GUI快照Save/Load使用原rfd0.10.0。
- 真实地形光标平面位于同一个TerrainEditingData；成功创建先保存raw drag distance、持续绘制保持、释放即清，即使无最后stroke也清。原副作用顺序和NaN/-0保留。
- 输入录制当前宿主格式2→3，携带原readback记录的depth/mouse/center三值；不把旧缺值录制补零。该格式不是原208B内存ABI。
- 两App复用真实MEI阶段注册：set6输入/set7缩放/set44文本与布局→节点→单次flush。构图、warmup与OOM重试复用本次上传，避免第二次consume丢失计数。

原证、候选与边界见 [startup报告](startup/REPORT.md)。仍缺Walk进入模式的部分原入口、其它原startup bundle/音频owner及全游戏调度；不得据相机移动或局部GPU启动认定完整交互。

## 编辑 → 历史 → 回放：仍为部分

- 纠正stairs Pop所谓“外部两张结构表”为同一backup GraphMap；非空diff按原restore/apply/dirty/rebase顺序运行。
- 所有域删除回放命令时同步压缩timing，修复mixed桶错位/重发。
- 生产宿主接system_color真实副作用、FlatRoof、tree sentinel与prefab真实spawn/update/despawn，传播全部placement、Entity去重、GPU/RT/audio与canonical BirdHaven。
- 纠正world AABB重复缩放平移、错写40B Transform与错误RT回调归属，不保留无人消费的替代队列。
- 深入修复多entry错读最近全局diff：每条HistoryEntry持自己的装饰/楼梯Arc，选中entry时按原storage级方向生成cull_allow。撤销/重做不再复制最近一次diff。

详见 [history报告](history/REPORT.md) 与 [entry-owned aux原证及设计](history/CULL-PRODUCER.md)。保存的MultiEdit aux wire解码、完整roof/terrain/raster/tree UI调度、颜色GPU更新及全部音频播放仍未闭合。

## 帧循环 → 构图 → GPU：仍为部分

- W1–W54与4×MSAA尾部适配到最新rhapsody；T21真实Egui owner在T20后向同一MSAA目标绘制，再export0x22。
- T12用真实String LUT名、canonical配置与GradingLuts；T16恢复全部screen/world query与分支、唯一组件TypeId和真实array layer Vulkan view/owner；没有忽略view的默认降级。
- T11/T13先读真实EscMenuState门控，不再用默认false绕过。T13原消费者可调用；缺capture/state生产者时仅在真实ReportBug路径命名失败。
- T11完整thumbnail host仍缺，Load激活时命名失败；T12 preview capture、T13 capture/state、颜色icon动画生产者及若干尾部控制/截图关联仍缺。**不等于完整compile_main或完整warmup/场景交互**。

原PE字节、布局与图绑定证据见 [render报告](render/REPORT.md)。

## 验证与兼容

最终结果见 `verification.json` 与其绑定的源码/日志指纹。此前失败和在途快照日志保留，不用旧通过替代最新源码验收。特别是首次相机移动回归的dt被真实Vermeer freeze降为0；现测试先验证冻结，再用真实deactivate推进下一帧，不删除移动断言或改原算法。

最终有界回归：country-core库2546项、宿主库44项、native-physics输入全链17项，以及严格Vulkan启动/加载/Egui共4项，全部通过，失败与跳过均为0。T21同一4×MSAA目标的owner与顺序已接通，但独立Egui像素fixture为单采样；尚未验证完整游戏帧、全场景交互、保存流程或与原进程的完整行为等价。

输入integration旧fixture没有生产GameProfile，且假定无按键时Modifiers恒开；现提供真实profile并按真实绑定按键验证阶段。旧手势测试把按下/释放帧移动也计入阈值，与既有原指令`1417cd959`的“按下清零、仅held累加、释放比较既存长度”相反；按原分支修fixture，未改业务来迎合旧预期。三次日志均保留，最终输入17/17通过。

严格GPU回归第一次因Egui测试只认`TG_COMPILED_SHADERS`而非`TG_INSTALL_ROOT`使用了旧默认目录；设置已有显式环境入口后原shader真实执行。随后加载图暴露Windows嵌套纹理名以反斜杠求hash，和原固定`logo/menu`键不符；现加载时规范为发行包的`/`，未改文件路径或用假纹理代替。修复后原加载图真实像素回读通过，失败日志保留。

`rfd`及新加入的全部19个传递包version/source/checksum均与原`build-info/Cargo.lock`匹配，未升级或删除原有包；见 [dependency-delta.json](dependency-delta.json)。GUI文件对话框操作未用自动UI点击触发，磁盘schema和失败保留owner由针对性回归验证。

在线ReVa没有已加载程序，RE-MCP无database；失败见 `tool-read-boundaries.json`。本轮新事实来自原身份RNE/PDB、保留反编译与独立限定LLVM反汇编，不报告不存在的新MCP成功，也不把脚本计作业务交付。
