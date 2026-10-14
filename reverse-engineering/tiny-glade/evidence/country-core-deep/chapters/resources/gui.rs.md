# resources/gui.rs

> 文件作用：GUI 状态资源族：EscMenuState 8B {sub u32@0, disc u32@4}（is_open: qword∉{2,0xa} 或 0xa 时 sub!=1；is_closed: qword==2 或 (0xa 且 sub!=0xff)；cast_ui_shadows: qword∈{3,4} 或 (disc7&&sub1)；should_show_world: 0xfe..0x106 状态位掩码 0xaf/0x50 判定；can_limit_frame_rate: !(disc7&&sub1)）；NextEscMenuState::set（哨兵 0xb；字符串@+0x30 释放 + 16B@+0x50 拷贝）、PrevEscMenuState::get（0xb→None）；LoadMenuThumbnailMetrics、VideoSettingsSubMenu、DebugGui::log_to_screen、ShowDebugRaycastGeo、DebugTriggerProxyVis。

函数总计 36：业务 14 / 内联无独立符号 3 / 派生胶水 19

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::gui::DebugTriggerProxyVis::as_string` | 0x14093ffd0 | 0 | DebugTriggerProxyVis::as_string | O(结构)：u8 索引串表（FUN_1428d8fe3 长度 0x12/0xe/0x10 查询 + 不可恢复跳表 match），生成调试代理可见性描述 | ui-input\decomp\decomp-0x14093ffd0.txt |
| `country_core::resources::gui::PendingEguiContextResource::into_egui_context_resource` | 0x14067dd80 | 18 | PendingEguiContextResource::into_egui_context_resource | O：ICF 折叠（与 UiState::widget 同码；egui Context 转换桥） | ui-input\decomp\decomp-0x14067dd80.txt |
| `country_core::resources::gui::NextEscMenuState::set` | 0x14093fad0 | 128 | NextEscMenuState::set | O（手解）：当前状态==0xb 或 ∈{0xfe..0x106} 且 !=4 时走不同分支；释放 self+0x30 字符串资源；拷贝新状态 {qword@0, 16B@+0x50} |  |
| `country_core::resources::gui::PrevEscMenuState::get` | 0x14093fbb0 | 139 | PrevEscMenuState::get | O（手解）：状态==0xb → None（返回 null），否则 Some(&self)——0xb 为哨兵 |  |
| `country_core::resources::gui::EscMenuState::is_open` | 0x14093fbc0 | 145 | EscMenuState::is_open | O（手解）：qword∈{2,0xa} 之外→true；==2→false；==0xa→(sub!=1)（mouse_capture 推 {sub2,disc3} 时 true ✓） |  |
| `country_core::resources::gui::EscMenuState::is_closed` | 0x14093fbe0 | 149 | EscMenuState::is_closed | O（手解）：qword==2→true；==0xa→(sub!=0xff)；其余 false（与 is_open 互补） |  |
| `country_core::resources::gui::EscMenuState::cast_ui_shadows` | 0x14093fc00 | 159 | EscMenuState::cast_ui_shadows | O（手解）：qword∈{3,4}（单位变体）或 (disc==7 && sub==1)——注：sysres-procedures 把此地址标为 is_open，SOURCE_MAP 归 cast_ui_shadows，以 SOURCE_MAP 为准 | ui-input\decomp\decomp-0x14093fc00.txt |
| `country_core::resources::gui::EscMenuState::should_show_world` | 0x14093fc20 | 176 | EscMenuState::should_show_world | O（手解）：qword-0xfe<9 时按 0xaf 位掩码（位 4/6→true）、再按 0x50 位掩码（位 4/6→(sub+1)）判定；越界按位 4(=0)→true |  |
| `country_core::resources::gui::EscMenuState::can_limit_frame_rate` | 0x14093fc60 | 207 | EscMenuState::can_limit_frame_rate | O（手解）：!(disc==7 && sub==1)（SOURCE_MAP 名；sysres 清单标为 allow_time_passage——同名异址错位） | ui-input\decomp\decomp-0x14093fc60.txt |
| `country_core::resources::gui::LoadMenuThumbnailMetrics::positions` | 0x14093fc70 | 238 | LoadMenuThumbnailMetrics::positions | O（手解）：构造 4×u64 {…}（缩略图位置度量构造，无分支） |  |
| `country_core::resources::gui::VideoSettingsSubMenu::is_in_submenu` | 0x14093fcd0 | 307 | VideoSettingsSubMenu::is_in_submenu | O（手解）：`cmp byte [rcx],0; sete al` = 判别 0 |  |
| `country_core::resources::gui::DebugGui::log_to_screen` | 0x14093fe50 | 361 | DebugGui::log_to_screen | O（手解结构）：读全局开关字节（rip+0x2cf4d80 区）→ 格式化行追加（屏上调试日志） |  |
| `country_core::resources::gui::_::impl$0::first` | 0x140002370 | 385 | impl$0::first | O：折叠单行（NotFound；容器 first 元素） | ui-input\decomp\decomp-0x140002370.txt |
| `country_core::resources::gui::ShowDebugRaycastGeo::as_string` | 0x140940170 | 425 | ShowDebugRaycastGeo::as_string | O（手解结构）：`movzx eax,[rcx]; 两张相对偏移表 {rel_ptr u32, rel_len i32} 查表拼 &str`（表址校准 U） |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::gui::EscMenuState::allow_time_passage`（行 168）— I：内联；语义由 can_limit_frame_rate（0x14093fc60 同码区）佐证：!(disc7&&sub1)
- `country_core::resources::gui::EscMenuState::can_use_static_world_capture`（行 189）— 谓词/校验
- `country_core::resources::gui::EscMenuState::allow_painterly_overlay`（行 197）— I：按名推断——笔触叠加层开关谓词（内联）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`deref`、`eq`、`next` 等 19 项
