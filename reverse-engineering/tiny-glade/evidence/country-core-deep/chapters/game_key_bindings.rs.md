# game_key_bindings.rs

> 文件作用：**按键→动作映射（profile 持久化）**。`KeyBindings` = 40 动作槽（12 B 步距，u32 默认键码）
> + `[0x1b0]=0xc3`(unset) + u16 鼠标键 `{1,2,1}`@0x1bc/0x1c0/0x1c4。`default` @0x14089b110 写 36 默认键
> （前 10 `0x2a,0x26,0x14,0x17,0x18,0x24,0xc3,0xc3,0xc3,0xc3` + 后 26 与重建一致）。
> ron serialize 给出精确字段表：move_forward…mouse_focus_edit（§2.3 DIGEST）。`action_desc` 44 分支
> 生成 UI 标签/图标；`assign_key` 清 0xc3 + 冲突交换；`mouse_button_to_string` LMB/MMB/RMB。
> 证据：default/assign_key/action_desc/serialize 反编译 + 原字节。

函数总计 17：业务 10 / 内联无独立符号 2 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `KeyBinding::action_desc` | 0x14089cb60 | 0 | 动作 UI 描述/图标 | **O** | 44 分支 switch；MoveUp/MoveDown/Move/Rotate/ZoomIn/ZoomOut/Undo/Redo/FocusOrDeselect/…/HideUiInPhotoMode + toolbar 图标 |
| `MouseBinding::action_desc` | 0x14089c8a0 | 0 | 鼠标键描述 | **O** | btn0→Rotate、1→Move、else→FocusOrDeselect |
| `mouse_button_to_string` | 0x14089c970 | 0 | 鼠标键→字符串 | **O** | 12 项跳表 → LMB/MMB/RMB |
| `deserialize::visit_str<ron>` | 0x140366a40 | 4 | ron 变体匹配 | **O** | 键名表匹配 |
| `serialize<ron>` | 0x140831740 | 4 | ron 序列化 | **O** | 40 字段名表（stride 0xc） |
| `KeyBindings::assign_mouse_button` | 0x14089b280 | 110 | 指定鼠标键 | **O** | 写 `{key,mouse} u16` @0x1bc+action*4（重建已证） |
| `KeyBindings::assign_key` | 0x14089b2a0 | 117 | 指定按键（冲突交换） | **O**（交换体 I） | 26 块逐槽扫描、清 0xc3、冲突交换、变更事件 |
| `MouseBinding::get_mouse_button` | 0x14089c880 | 286 | 取鼠标键 | **O** | 取值 |
| `KeyBinding::get_keycode` | 0x14089d300 | 423 | 取键码 | **O** | 取值 |
| `keycode_to_str` | 0x14089d490 | 468 | 键码→标签 | **O** | 跳表 |

## 内联/无独立符号函数

- `KeyBindingDesc::text`（行 361）— **O**（静态标签，内联于 action_desc）
- `KeyBindingDesc::icon`（行 368）— **O**（静态图标，内联于 action_desc）

## 派生胶水（5 项：closure、default、deserialize、visit_map 等）
