# 第二轮（审查+闭合）状态

| 域 | REVIEW | CLOSURE | 状态 |
|---|---|---|---|
| audio-misc | ✅ 40+ 文件：17 FIX / 18 GAP / 18 OK | ✅ 5 U：4 CLOSED + 1 PARTIAL | 完成 |
| session-save | ✅ 16 文件：2 ok / 13 需修 / 1 未知 | ✅ 6 U：4.5 闭合（NextSessionKind 生产点/execute 0-2/on_load 头仍卡） | 完成 |
| history | ✅ 27 文件：13 ok / 14 需修 + roof_edit 整文件缺失 | ✅ 6 U 闭合（跳表全解码、History serde 位置实锤）；2 项转交 | 完成 |

| terrain-water | ✅ 28 文件：OK 7 / STUB 7 / LATER 3 / FIX 9 | ✅ 8 U 闭合（+0x29b 无直接置位已穷举证明、0x07954dbb=空瓦片哨兵、tabloid 链）；3 卡点残留 | 完成 |
| wall | ✅ 50 文件：16 ok / 22 需修 / 6 未知 | ✅ 4 U 全闭合（u_space=1.0f@0x142925904、as_wall_mut 真址、五集合映射、XOR 掩码）；5 卡点残留 | 完成 |
| camera-time | ✅ 22 文件：11 ok / 11 需修 / 0 未知 | ✅ 7 U：4 全闭合 + 3 部分（rotation=rotation-first quat@+0x18、fog_tint 字段名实证、zoom 全貌、模式槽交叉验证） | 完成 |
| ui-input | ✅ 逐文件表（10 处核心错） | ✅ 6 U：4 闭合（check_if_can_switch 尾段=unreachable、EscMenuState 结构+消费者链、槽6/7..12 表）；2 卡点 | 完成 |
| collision-eco | ✅ 22 文件（WRONG 级 5：world/stairs_state/stairs_assembly_params/draw_path/clutter_meshes） | ✅ 6 U：4 闭合（add_node 全手解+更正 DIGEST +0x29→+0x148、±8=两结构、ivy grower 边界、rapier 边界）；2 OPEN | 完成 |

## 已闭合 U 项（audio-misc）

1. **MusicPlayer 状态机 = 4 状态双跳表** CLOSED：节拍同步（0x143634c68/c90）→
   state=4 + start；Duration@+0x120 超时切曲；state=0 → switch_track。
2. **0x14362c420 = panic 进行中标志** CLOSED（与 history 域交叉确认）。
3. **FeedbackReportResources** 0x10..0x38 = 截图元数据区、+0x36 = 可用性枚举 <7 CLOSED。
4. **0x140949c40 = 静态析构注册** CLOSED（唯一引用 0x14368E2FC 为数据引用非 call）。
5. update_steam_inputs = 14 同构块压 14 个 u16 输入码（0x200..0xf00）PARTIAL
   （与具体动作名配对需 ui-input 域枚举）。

## 最高优先 FIX（audio-misc，供实现阶段）

roof/path/signifier 事件名、WallAudioState 固定数组、color_icon 亮度枚举、
color_id 默认值、steam user_id、shutdown 签名。

## 第三轮（章节化填充，chapters/ 就地更新）

| 域 | 章数 | 新反编译 | 空白函数 | 状态 |
|---|---|---|---|---|
| history | 27/27 | 14（累计 85） | 0 | ✅ 完成 |
| wall | 83/83 | 18（+12 手解） | 10（4 闭包实例+UiWallHeight+is_default_arch_height 卡点等，已列明） | ✅ 完成 |
| session-save | 17/17 | 11（+3 手解） | 0 | ✅ 完成 |
| terrain-water | 43/43（+4 补建章；4 replay 章沿用 history 域填充） | 11（+1 手解） | 0 | ✅ 完成 |
| ui-input | 42/42（4 文件无章：不在 SOURCE_MAP） | 0 MCP（15 手解） | 0（5 行 U 待中央裁决） | ✅ 完成 |
| camera-time | 21/21 | 5 | 0 | ✅ 完成 |
| collision-eco | 34/34 | 2（累计 202） | 0 | ✅ 完成 |
| audio-misc | 59（53 骨架 + 补建 6 零函数章） | 0（全有既有证据） | 0 | ✅ 完成 |
