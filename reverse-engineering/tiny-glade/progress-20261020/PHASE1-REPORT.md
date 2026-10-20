> 历史记录：第一阶段；当前状态和本轮验证以 CENTRAL-REPORT.md 为准。

# Toolbar / decorator / 存档中央集成记录

日期：2026-10-20。本轮按用户要求建立五个 agent，由中央负责公共类型、宿主注册和独立 Cargo 验证。原 B 镜像 SHA 为 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。

## 已落地的行为修复

| 线路 | 本轮修复 | 验证边界 |
| --- | --- | --- |
| toolbar | 四向自动隐藏激活区域、双轴边距、debug extent、声音条件；锁定点击动画与延迟生命周期 | 1600 原码几何案例及真实 ECS；零点沿用原 IEEE 行为 |
| toolbar / photo | 13 类滑块的真实 owner、六相机参数/焦点转换、重置和 AppMode 门控；LogSlider 逆映射扣除 base | getter1638、相机写入360原码对照；真实组件重置/模式回归 |
| decorator / interaction | u64+u8 手柄名哈希、两个 widget 工厂、四目的地移动、抓取偏移、射线优先级/地形和平面回退 | 实际生产回归消费原 golden：25 hash、24 widget、8完整72B move、400 grab、24 ray |
| decorator / visual | 门台阶完整 geometry leaf 接真实地形、路径、变换、碰撞 bounds 和32B蓝图输出；窄修 dormer 新几何API | 2000原码案例+另一agent独立192边界案例；真实采样资源适配测试 |
| 存档 | DeletedAutoClutter 整体替换；Roof/三类颜色/Tree 走真实命令；incoming 地形失效通知、栅格上传、原应用顺序 | 实际写文件→后台flush→重新读文件→World；原Roof32条/颜色96条payload及各owner错误分支 |
| 加载状态 | 真实恢复缺口不能仅warn后Loaded；失败清理并阻止OpenFile/LoadingGameComplete | 坏文件失败终态+合法文件完整状态转换 |
| 宿主 | 提前维护UploadWorldRasterBuffersToStagingCmd；原消费者接Update / GameSystemSet14 HistoryStart | parent→converter→vtable→leaf 和 Update label 由两agent独立原PE取证；设备传输仍待验收 |

栅格消费者的 `GraphicsDeviceResource` 门控是当前工程 CPU host 的适配条件，不是已证明的原作 guard。实际六个 mapped SSBO 仍由现有原 PreStartup producer 创建，消费者缺缓冲时保留失败行为，没有新增影子缓冲。

中央只改必要的当前API调用及测试fixture：brush JSON loader字符串参数、CPU兼容启动表中已不存在的重复mesh入口、mesh positions读取、UiState真实构造器、MainCamera真实构造器、dormer碰撞测试的canonical存储工厂。未改回 rhapsody 或底层工具旧实现。

## 中央实际执行

专用 target：`D:/game/tmp/progress-20261020-target`。锁定现有Cargo.lock，全部 `--locked --offline -j2`，rustc/rustdoc固定当前 stable。共 **290项 Rust tests 通过，0 ignored**：

| 命令目标 | 通过 | 日志（D:/game/tmp/） |
| --- | ---: | --- |
| toolbar --lib | 57 | progress-20261020-toolbar-lib.log |
| toolbar --test tb_f_auto_hide | 1 | progress-20261020-toolbar-ecs.log |
| toolbar --test tb_f_locked_animation | 2 | progress-20261020-toolbar-ecs.log |
| toolbar --test tb_e_config_slider | 4 | progress-20261020-logslider.log |
| system-decorator --lib | 215 | progress-20261020-decorator-lib-v3.log |
| country-core --test snapshot_native_restoration | 9 | progress-20261020-save-integration-v2.log |
| country-core --test save_load_gap_state | 1 | progress-20261020-save-integration-v2.log |
| country-core --test snapshot_worker_reload | 1 | progress-20261020-save-integration-v2.log |

更早的 `photomode::tests` 过滤器匹配0项，**不算验证**；随后真实执行整个toolbar57项，包含slider_restoration_tests。早期失败和修正记录均保留，没有把先前失败日志称为通过。

`country-core --lib systems::save::` 当前被六个既有外部fixture缺失阻断（material stats、firstperson、brush runtime、human scale、path preview、signifier replay），日志progress-20261020-save-lib-v2.log。本轮九个回归移到独立integration target，执行相同canonical生产入口；没有禁用原测试或伪造fixture。

主程序 `cargo check -p tiny-glade --lib --bins` 的正常路径因旧安装目录 fmod.dll 缺失失败，见progress-20261020-host-check.log。随后通过显式FMOD_LIB_DIR空link-search目录进行**仅metadata类型检查**，该目录不是SDK；日志progress-20261020-host-types-only.log。这不能证明链接、原生音频或游戏运行。最终类型检查结果见validation-results.json。

## 仍未闭合

- toolbar：完整ui_slider绘制/回收、双鼠标输入、导航与焦点/声音生命周期，do_photomode/setup_photomode_toolbar及其准确宿主注册。
- decorator：外层affordance/interaction intent、move/place/rotate/split等实际消费者；premade trigger accessor；门视觉外层生成。移除signifier既有marker/closure/Arc生命周期缺口仍存在。
- 存档：非空Roof/Tree/颜色请求经消费者后的全游戏回载、stairs缓存checksum兼容、完整格式语料、设备上传与GPU画面验收。
- T13反馈捕获继续跳过。用户给出的toolbar≈60%、decorator≈27%不因这批叶子函数和CPU测试直接上调；整链需后续验收。

各agent原证据、源SHA及独立审阅保留在本目录五个子目录；中央最终源码身份见source-manifest.json。经验与用户正确性约束见RECONSTRUCTION-LESSONS.md。
