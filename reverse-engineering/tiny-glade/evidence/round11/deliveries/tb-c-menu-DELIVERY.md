# TB-C 交付：菜单 / 拍照 / 文本剩余项（`crates/systems/toolbar`）

> 契约：`evidence/round11/contracts/tb-toolbar-CONTRACT.md`（§2 剩余 40 项中的 C 组）。
> 工作目录：`D:/game/tiny-glade`。文件权：`menu/**`、`photomode.rs`、`mei/widgets.rs`、
> `resouces_and_components/**`、`tests/**`（未改 `mei/core.rs` / `mei/layout.rs` /
> `ui_interaction.rs` / `toolbar_button.rs` / `lib.rs`）。
> 证据优先级：既有 fresh 反编译（`upper-modules-plan/systems/toolbar/jobs/decomp/*.c`）
> > PDB 依赖注解（`skeleton/B/...`）> ANALYSIS-B/C。未闭合处一律登记 U，不填默认值。

## 0. 构建与测试

| 命令 | 结果 |
|---|---|
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-tbc2 cargo check -p toolbar` | **exit 0**（19:10，C 组源码全部落地时；`Finished dev profile`） |
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-tbc cargo test -p toolbar --lib` | **50 passed / 0 failed**（19:07；随后新增的 credits scroll/load hovered 两项测试处在同一编译单元，未被单独计数） |
| 收口重跑（`CARGO_BUILD_JOBS=4`, `timeout 240/280`） | **被共享构建锁截断**（多 agent 并发重建 country-core），未重试同一命令；不宣称最终快照全绿 |

收口说明：19:12-19:14 另一个 agent（TB-D）并发重写了同 crate 的
`toolbar_layout.rs` / `toolbar_systems.rs` / `toolbar_update.rs`（`ToolbarLayout` 与
`PhotomodeToolbarLayout` 拆成两个资源 + `ToolbarContext` trait）。这发生在本次
exit-0 检查之后；本组源码未再改动（仅测试文件新增）。最终整包构建需由集成 owner
在并发写者停止后统一验证。

## 1. widgets 剩余视图（契约 #26-#33）

| # | 函数 | 状态 | 位置/证据 | 真实输出 |
|---:|---|---|---|---|
| 27 | `ImageView::blur_extend` | **已完成（O）** | `mei/widgets.rs`；fresh `0x14067eaf0`：`ImageContent+0x44=1` | 新增 `ImageContent.blur_extend`；`prepare_layout` 选择 `ImageScaling::BlurExtend`（真实 tg-text 编码） |
| 29 | `MenuButtonView::force_highlight` | **已完成（O）** | fresh `0x14067e9e0`：`+0x25c=bool` | `NodeContext::force_highlight` 写 `always_white`；draw 的白色分支真实消费 |
| 30 | `MenuButtonView::hotkey` / `PillButtonView::hotkey` | **字节写面已完成（O）** | fresh `0x14067e6d0` / `0x1406816c0`：`+0x25d` / `+0x44` | `menu_hotkey` / `pill_hotkey`；`0x12` = 无热键分支 |
| 31 | `PillButtonView::inactive_opacity` | **已完成（O）** | fresh `0x1406814f0`：`PillButtonContent+0x40=f32` | `NodeContext::inactive_opacity` 写 `strength`；pill 绘制的非 hover 透明分支真实消费 |
| 32 | `make_hotkey_tooltip` | **U（不伪造）** | PDB inline，宿主 0x14067e6d0/0x1406816c0 | 需要原静态 action→hint 表与 tooltip 模板；只落 O 的字节写面 |
| 33 | `PillButtonView::mute_click` | **U** | PDB 依赖：`NodeContext::sounds` + `UiSounds::default` | 原生 `sounds()` 写面在当前 core 无同名等价 API；未猜 hover/click id |

`back_button`（#26）沿用既有 `NodeContext::back_button`（未改语义）；共享 `content_mut`（#28）
走 `get_content_mut_impl`，本组新增的 `ScrollContent`/`TextEditContent` 均实现
`DefaultNodeContent` 并由该真实路径初始化。

## 2. 菜单（credits / load / report_bug）

### 2.1 `menu/credits.rs`（新增，契约 #34-#36）

- **#34 `ScrollView::content_mut`**：`ScrollContent` 实现 `NodeContent + DefaultNodeContent`，
  访问器走 `NodeContext::content_mut`（真实 `get_content_mut_impl` 路径）。
- **#35 `ScrollView::max_scroll_pos`**：`(content_max - content_min) - 450.0`（ANALYSIS C9），
  另有 `offset` / `background_color`（源码行 304/314 的内联写面）。
- **#36 `spawn_third_party_license_http_server`**：`SingleFileServer::http/run`
  （O 0x14234d690 / 0x14286bf40）——绑定 `127.0.0.1:8123`、`tiny_http` 单文件循环、
  文件缺失 500、请求异常退出；系统按 `EscMenuState::Credits` 门控并 spawn 线程，
  成功/失败写入 `Local`（失败存错误串，对应原生判别子 2）。
  U：被服务文件路径（原生 `+0x30`）与 `Content-Type` 头；由
  `ThirdPartyLicensePath` 资源注入。
- U：`credits::ui`（4398B）的模板/许可字符串与 `impl$2::render` 的绘制闭包未逐语句闭合；
  已落滚动速度积分、范围夹取与 `process_input_action` 滚轮输入，不生成假绘制。

### 2.2 `menu/load.rs`（契约 #37-#44）

- **#38 `display_save_file_load_error`**：错误模板 + `header`/`error` 两个 slot，
  真实消费者 `menu/load_ui.rs::draw_error`（已改为调用它）。
- **#39 `hovered_item_pos`**：`update_metrics` 发布的 hovered 索引。
- **#41 `min_scroll_pos` / #42 `nav_scroll_speed` / #43 `run_count`**：从
  `update_metrics` 内联代码拆出并回接（`ThumbnailScrollContent::draw` 真实调用），
  公式与原生 141e1f4c0 一致；新增 `max_scroll_pos` 配对 helper。
- **#44 `show_contextual_buttons`**：原 `load_ui::contextual` 移到 `load.rs` 并公开，
  由 `load::ui` 的 hovered 分支真实调用（删除/换谷类型/备份 200×30、y+250 布局）。
- **#40 `initial_focused_item`**：**U**（原生行 145 的初始化语义未闭合，不返回猜测索引）。

### 2.3 `menu/report_bug.rs`（契约 #46-#49）

- **#49 `try_remove_file`**：沿用上一轮 O 实现与测试（真实 `is_file` + Windows unlink + io 日志）。
- **#46 `TextEditView::content_mut` / #47 `disable_input` / #48 `input_allowed`**：
  新增 `TextEditContent`（`focused`、`initialized`、共享 `Arc<TextEditMemory>`、输入门），
  `NodeContext::text_edit` 走真实 `get_content_mut_impl`；`prepare_layout` 执行原生
  `+0x270`/`status==4` 初始化握手。
  U：`cosmic_text::Editor` 0x198 字节 `replace_editor` 复制未建模（cosmic-text 不是
  toolbar 直接依赖），编辑器文本以共享 `TextEditMemory` 安全承载。
- T13 反馈捕获/GPU 截图按 `AGENTS.md` 与契约 §5 继续排除。

## 3. 拍照（`photomode.rs`，契约 #50-#53/#77）

- **#50 `photomode_buttons`**：`photomode_action(tag)` 显式动作表（可单测）：
  0..=3 相机、4 截图目录、5 相机循环（demo 表）、6/9 `WindowFocusEvent`、
  0xb 回主菜单、0xc Vermeer activate/deactivate、0xd/0xe 翻转
  `iter_non_default_mut` 行标志 `+0x3c/+0x3d`、0xf `PhotomodeState.hide_ui`、
  0x11 `RequestAppModeChange(default)`；每动作插入 `UiPlaySelectAnimation`。
  U：动作 7/8/0xa/0x10 未取证（返回 `None`，不猜）；6/9 的 `WindowFocusEvent`
  载荷 U（按动作字节转发）；截图目录由 `PhotomodeScreenshotFolder` 资源注入
  （`paths` crate 非 toolbar 依赖）。
- **#52 `slider_range`**：9→(0.3,2.0)（ANALYSIS C14）、0xa/0xb/0xc→真实
  `BrushSizes` 范围；0..8 的 `0x14304ed8c/0x14304edc0` 表未取证 → `None`。
- **#53 `slider_widget_size`**：**U**（`Vec2 * 常量` 的常量未取证）。
- **#77 `ui_slider` / `ui_slider_reset` / `slider_val_01`**：
  - `slider_val_01`：0..=5 读当前 `ImageFormationConfig` 字段（`+0x18/+0x30/+0x38`），
    6 读 row word；7/8（MainCamera focus/lens）U。
  - `apply_slider_value`：0..=5 写两条可编辑配置行；9 按 `AppMode`
    写 path/water/garden 并 clamp 0.3..2.0；0xa/0xb/0xc 走真实 `BrushSizes`。
  - `ui_slider`：消费 `UiDragEvent{state==3}`，按 `UiSlider+0x10` kind 与
    `Ui2dExtent`/`CursorPositionSS` 计算 0..1，再经 `slider_range` 映射；
    U 范围不写入。`ui_slider_reset` 处理 0..=5/6/8/0xa/b/c 复位。
  - U：6/7/8 FOV/focus/lens 分支与 0xd+ 截断 case；`UiSlider` 其余 23 字节字段名。
- **#51 `setup_photomode_toolbar`**：**U**（6111B；标签串/ToolbarItemKind 值未取证）。

## 4. 布局资源（契约 #59-#62）

`resouces_and_components/toolbar_layout.rs` 新增（本组落地；随后 TB-D 并发把
`ToolbarLayout`/`PhotomodeToolbarLayout` 扩成两个同构资源并加 `ToolbarContext` trait，
当前文件为准）：

| # | 函数 | 状态 | 输出 |
|---:|---|---|---|
| 62 | `ToolbarLayout::setup_screen_and_toolbar_nodes` | I/O | 两个 `new_leaf`、screen→toolbar `add_child`、screen `AnchoredNode{order 0, side Window}`（native 0x141e4ccb0 只登记 screen 锚点） |
| 59 | `ToolbarLayout::add_contextual_menu` | I | 按 tag 查定义行（缺失即 panic，等价 native `unwrap_failed`）+ 菜单 leaf + `AnchoredNode{order 1, gap 10.0, side Above, relative_to=row}` + 递归 `add_toolbar_item`（sentinel `show_only_fpv==2`）+ 写回 `row.node` |
| 60 | `contextual_menus` | I | 返回真实菜单切片（alignment/size-padding 的消费面） |
| 61 | `link_up_cyclic_list_nav` | O | 对真实 `Entity` 逐项 `Commands::insert(UiNav{prev=ids[(i+n-1)%n], next=ids[(i+1)%n]})`，横向/纵向槽位由 `UiNav::horizontal/vertical` 写面 |

U：`add_contextual_menu` 的 0x20-stride 定义表按 tag 查找由调用方提供；
`ToolbarLayout.categories` 的 `toolbar_startup` 生产者不在本组文件权内。
并发说明：TB-D 的 `toolbar_update.rs` / `toolbar_systems.rs` 在本组工作期间同步改动了
同一批资源类型；本组只对 `toolbar_update.rs` 做了一处借用顺序修复（`node.scale()` 先取
再 `set_scale`），其余以其最终文件为准。

## 5. 测试与遗留

新增单测（均跟随源码）：
- `menu::credits`：`max_scroll_pos` 450 视口、速度积分/夹取、`credits_scroll` 节点内容访问器、
  URL 标签、真实文件+loopback server bind。
- `menu::load`：`run_count` 上取整、min/max 滚动界、`nav_scroll_speed` 反比夹取、
  `hovered_item_pos` 发布索引。
- `menu::report_bug`：输入门 + 共享 `TextEditMemory` Arc。
- `photomode`：动作表（含未知字节 None）、`slider_val_01`、`slider_range`、
  参数写目标（配置行/三种笔刷/非拍照拒绝）。
- `resouces_and_components::toolbar_layout`：screen/toolbar 根与锚点、上下文菜单 sentinel。

显式 U（不伪造）：
1. `make_hotkey_tooltip` / `mute_click`（widgets #32/#33）。
2. `initial_focused_item`（load #40）。
3. `setup_photomode_toolbar`（#51）、`slider_widget_size`（#53）、动作 7/8/0xa/0x10。
4. FOV/focus/lens 滑块分支（6/7/8）与 0xd+ 截断 case。
5. `credits::ui` 模板/许可字符串与 `impl$2::render` 绘制闭包；
   `SingleFileServer` 的 Content-Type 与路径来源。
6. `TextEditMemory` 的 cosmic_text Editor 复制。
7. 菜单 welcome/daily_welcome/save_error/corrupted_save_file/new_session 的 UI 主体：
   本组只落 load/credits/report_bug 的上述真实输出；其余仍由后续轮次按契约 §3.2
   的缺失资源（`CurrentDaily`、`WelcomeScreenState` 等）推进。
8. `advance_menu_state`（菜单推进器）复用 `country-core::resources::gui`，未重写。

构建环境说明：多 agent 共享 workspace/target；`cargo check -p toolbar` 在 19:10
exit 0，`--lib` 50/50 通过。19:12 后 TB-D 的并发资源重构与其它 agent 的
country-core 重编译使后续 `cargo test` 长时间排队；按协调者硬超时规则未原样重试，
登记为“最终快照未验证”。集成测试 `tests/tb_e_config_slider.rs` 为其它 agent
未跟踪文件；早先一次并发窗口内报告 `value_to_param` 逆映射断言失败，本组未修改
`LogSlider` 语义，不认领该失败。

## 6. 计数（本轮 C 组切片）

契约“仍缺 40 项”中属于菜单/拍照/文本/剩余 widgets 的切片共 30 项：

| 分类 | 数量 | 项 |
|---|---:|---|
| 已落地（O/I，真实输出+消费者） | 24 | widgets #27/#29/#30/#31；credits #34/#35/#36；load #38/#39/#41/#42/#43/#44；report_bug #46/#47/#48/#49；photomode #50/#52/#77；布局 #59/#60/#61/#62 |
| 仍 U（不伪造） | 6 | widgets #32/#33；load #40；new_session #45；photomode #51/#53 |

全局 40 项中其余 10 项属 toolbar_startup/taffy 更新/undo-redo/demo 等其它分组，不在本组文件权内。
另有菜单 welcome/daily/save_error/corrupted 的 UI 主体（尚未进入该 40 项清单的资源侧缺口）保持 U。
