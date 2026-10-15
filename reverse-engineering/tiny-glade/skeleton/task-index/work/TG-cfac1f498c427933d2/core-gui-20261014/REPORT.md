# GUI 文件恢复断点，2026-10-15

目标仅 B，覆盖索引默认 C。整合 owner 按用户授权直接修改正式 country-core；没有修改上层 crate、发行文件或共享数据库。最新要求为实现优先，不继续扩充测试矩阵。

## 已实现

| 正式源码 | 业务与原证据 |
|---|---|
| `resources/gui.rs` | EscMenuState 拥有加载/设置/报告/摄影等载荷，Next/Prev 的 None 默认、移动替换/get、共享 Arc clone/drop；set `0x14093fad0`、drop `0x14014af10`、clone `0x140fe5150`、初始化 `0x14068de50` |
| 同文件 | `advance_menu_state` 取走 next；只在 Closed 时先发设置写入请求；旧 current 移入 previous。原消费者 `toolbar::menu::advance_menu_state 0x14068e5b0`，在 core 提供算法和回调接收面；原调度/事件 sink 尚未迁入 |
| 同文件 | LoadMenuThumbnailMetrics 默认和 positions：按行/列排列，分离乘加，eager Vec::IntoIter；`0x14093fc70`/`0x141304fb0`，真实 producer `0x141e1f4c0` |
| 同文件 | 时间策略来自 `time_update 0x140922c30` 内联：is_closed 或 Settings(Video(Image))；VideoSettingsSubMenu::is_in_submenu 精确为 tag==0；两组调试标签已按原字符串恢复 |
| `systems/render_loop/lut_gallery.rs` | 共享 gui metrics 和位置算法，删除 host.cell_positions；读取 EscMenuState 而非 AppMode。原 TypeId 两字在 `0x1408b0635/63f` 与插入者 `0x140fd13da/13e4` 相同 |
| `resources/session.rs` | 菜单所需 SessionId(Uuid)、new_v4 和 serde，复用 B 原锁 uuid 1.15.1；new `0x140916420`，PDB session.rs line123 serde->uuid。未展开完整 session 域 |

## 取证纠错

- 原 owned 菜单为 96B；+0x30 不是 String，而是 Arc<parking_lot::Mutex<LoadMenuThumbnailMetrics>>。Load 克隆复制 ID/选择/页号，但共享 metrics。
- 原菜单 word 2/3/4/5/7/8/9/10 对应 Closed/Main/Quit/New/Settings/ReportBug/Credits/Photomode；Load 使用 Option<usize> 的 0/1 niche。word6 为分派默认槽，不增造第六种菜单。
- 设置菜单由 `0x140506d00` 写入 Audio3/Video0/Game5/UI6/Controls9/Dev8/Language7；视频入口 `0x140247870` 写 Image1/Display2。byte4 是 niche 默认槽；tag10 被 clone 保留，但 `0x1405073a0` 渲染会 panic，原名/生产者未定位，暂名 UnreachableTag10。
- `0x140504f10` 的 menu/exit_icon 写 word4；`0x140505080` 的 Menu.Credits 写 word9，不按旧章节猜测变体名。字段/变体为描述性命名，不承诺作者原名或当前 Rust enum 的二进制布局。
- positions 的 scroll_offset 不应再次加到 origin；原 producer 已计算屏幕原点。Default 的 Option::None 载荷未初始化，不能宣称整个 0x34B 全零。
- get_functions 错地址 `0x140fe5350` 返回其他包含函数，未作为 GUI 证据；正确 clone `0x140fe5150` 已另查。Ghidra 跳表 size/CFG 有截断，采用 PDB 范围和原 RNE 指令判定。

## 检查与剩余

当前执行 `cargo check -p country-core --all-targets --offline` 成功。锁文件只为 country-core 添加既有 uuid 依赖边，没有更换任何包版本。此前本轮前缀/缩略图/标签的原码对照、debug/release 日志原样保留；它们不覆盖之后新增拥有状态，未在最新授权后扩展或重复跑测试。

下一个仍是 gui.rs：PhotomodeScreen、完整 DebugGui/log_to_screen、PendingEguiContextResource。ReportBug bool、warning_session 准确含义、tag10 名字仍待查；全局调度/持久化/真实 GPU host 未接。整个 GUI/core 均未完成。

## 身份与证据

- 原 B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- B PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- 只读 Ghidra B patched SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
- 同目录 `gui-state-result.json`、`gui-consumers-result.json`、`gui-metrics-result.json`、`gui-owned-state-result.json`；均 checked ok/isError。原指令 fixture 在正式 `tests/fixtures/gui-contracts-b.json`，保留第一次非有限值失败日志，未抹去历史。
