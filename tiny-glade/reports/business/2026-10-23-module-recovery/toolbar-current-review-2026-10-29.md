# M0 工具栏当前复核（2026-10-29）

| 函数/链 | 当前实现和原证 | 已验证范围 | 剩余缺口 |
|---|---|---|---|
| 工具栏输入、焦点与菜单 | `ToolbarPlugin` 的 set6/7/8，Welcome set36；B 原注册与布局证据见现有台账 | `toolbar_module_chain` 8 项和 `toolbar_focus_chain` 命令通过，前者有历史跨进程保存与 Reload；CPU 支架自己组装系统 | 生产整图的真实输入与窗口未验；Welcome 原节点测试待本轮执行 |
| 摄影/TOD | 原 `run_game_thread@14059d450` `1405a0512→in_set@140611040` 将8项 tuple 放 set39，TOD 为第5项；完整 PDB 及图查询见 `toolbar-tod-registration-evidence-2026-10-29.json` | 既有 TOD 映射和控件测试；摄影组合调度测试通过；中央生产共享 installer 测试另记 | 三个 tuple 元素是类型擦除 ScheduleConfigs，内部关系尚须原构造核定；当前私有 GUI schedule 的菜单更新与 TOD 相对顺序无显式边，不能凭现证强制同帧 |
| 设置、新建/Daily、保存错误 | 已有设置路由、Daily发布、真实存储与告警消费；原模板及旧原指令差分在各专题报告 | toolbar 109 常规测试和9项 B 资产测试通过，含 NewSession、DailyWelcome、保存错误与恢复 | 测试不证明所有输入的原算法一致；真实外网与窗口设备状态未验 |
| Demo 与 onboarding | Demo 双提示和摄影提示生产者、消费者已接线；Subject 提示定位现有真实分支，旧注释不能当缺失证据 | B模板9项中含 Demo 双提示；focus链覆盖提示抑制及菜单删除 | 生产真实全链和其它编辑业务不由局部 fixture 证明；墙体编辑归 M6 |

`toolbar_menu_chain` 使用真实 headless Vulkan 和原模板，但自建 World 与 CPU 输入 schedule，手动将 Welcome 和 MEI post-update 接在输入后；它不安装生产 ToolbarPlugin、Game gate 或完整生产 set 图，不代表 OS 输入。原 GUI 调用图静态搜索不能证明动态分派完整；类型擦除配置项不得假定无内部顺序。真实 Steam SDK、许可证、Credits、反馈、OS 输入与性能按本轮范围排除或暂缓。

本机命令 `TG_INSTALL_ROOT='/Users/yuluo/Desktop/game/B版本内容' CARGO_INCREMENTAL=0 cargo test --offline -p tiny-glade --test toolbar_menu_chain real_welcome_layout_blocks_toolbar_until_its_own_confirm_handler_dismisses_it -- --test-threads=1` 退出101；`/private/tmp/m0-toolbar-menu-20261029.log` 保存原失败。编译成功，但真实 headless Vulkan 创建设备返回 `Unsupported("ERROR_INCOMPATIBLE_DRIVER")`，未进入菜单业务，不能记为通过。真实 App 变体还需要完整 SystemParam/状态与真实设备；不能以假设备、关闭系统或原 CPU schedule 冒充生产 App 验收。

按既有 M3 验证配置（`VK_ICD_FILENAMES=/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json`、`VK_LAYER_PATH=/opt/homebrew/share/vulkan/explicit_layer.d`、`DYLD_FALLBACK_LIBRARY_PATH=/opt/homebrew/lib`、`RHAPSODY_REQUIRE_VULKAN=1`）以受审查权限运行已编译同一测试，退出0，1/1通过，日志 `/private/tmp/m0-toolbar-menu-vulkan-20261029.log`。前述两次失败保留；其原因分别是沙箱设备访问和受审查进程未指定 loader 搜索路径，不能当成机器没有 GPU。此结果仍限手工 CPU 调度支架。

真实 App 补验：`toolbar_menu_chain.rs` 抽取资源及 B 资产初始化，新增 `production_app_welcome_and_reload_use_real_toolbar_installers`，源码 SHA256 `9e44410674225d0343f0caf051923f0eda5d222a1a5f0399e01caf8ba1341029`。真实 App 先安装生产事件维护，再装生产输入、MEI、ToolbarPlugin、摄影及模式消费者，原 set2 时间和 set3 输入重置；通过同设备 canonical SSBO 和 `attach_loading_mei` 使用真实 M3/MoltenVK、B 模板字体。隔离子进程中 Welcome 遮挡→确认关闭→Water、Held Reload 旧实体移除与重入 Taffy 总数/Water 唯一、旧捕获清除、无新模式请求→释放后真实新点 Path 均通过。`/private/tmp/m0-real-app-14-20261029.log` 退出0；命令为上文 MoltenVK 环境与 `CARGO_INCREMENTAL=0 cargo test --offline -p tiny-glade --test toolbar_menu_chain production_app_welcome_and_reload_use_real_toolbar_installers -- --test-threads=1 --nocapture`，受审查权限。抽取后的旧 Welcome 测试独立复验1/1通过 `/private/tmp/m0-menu-refactor-20261029.log`。原设备失败与 fixture 缺资源/漏 set3 的失败日志保留在 `/private/tmp/m0-real-app-{1..13}-20261029.log`（首轮无数字）及上文，不作生产缺陷。该合成输入与 headless GPU 测试不代表 OS 输入、完整游戏图或整模块算法逐位判等。toolbar crate lib.rs 仅修正过期说明，SHA256 `35400f9488ff6aea71e3ff0f2979c30510e0908b1d8ec9a93f13b15f79c80376`。

第14轮菜单阻断断言不足：等待24帧后 Water 已自动隐藏为零尺寸。第16轮将等待缩至4帧，检查 Welcome 为 Showing、Water 当帧尺寸非零、鼠标在该帧 extent 内且活动 widget 不是 Water，再按放无模式切换；关闭后取新布局点击 Water，随后 Held Reload 和新点 Path 仍通过。生产事件维护在初始化 fixture 之前调用 `register_gameplay_events`，EventCursor 检查新增请求，不将旧事件长度当新事件。`/private/tmp/m0-real-app-16-20261029.log` 退出0，源码 SHA256 `de40661e54c51fdab4e24b2ff7494345a71fe6cde4714cf4fface99e0fa5e705`；命令及 M3/MoltenVK 环境同上，受审查运行。此结果替代第14轮弱菜单断言，仍限合成输入与 headless App。
