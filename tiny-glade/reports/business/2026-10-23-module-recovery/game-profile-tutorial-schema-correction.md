# 原玩家档案嵌套 schema 纠错（2026-10-24）

关闭吸附提示修复的保存／重启测试发现：旧 `GameProfile` 把 `tutorial` 错作单个 bool，其余完成标志一律 `skip_serializing`，导致教程刚完成却在重载后丢失。此处按原指令重审，不沿用之前“顶层只有5字段所以其余不保存”的结论。

## 原证

- `GameProfile::serialize` **140a658a0**：顶层字段数5；tutorial 的实际单态化是 `serialize_field<TutorialCompletionState>` **141311310**，其调用原私有 serializer **140a65fe0**。字段传参为 `profile+1f4`，不是一个 bool 的类型证据。
- 原 `TutorialCompletionState::serialize` **140a65fe0..140a664b9**：`serialize_struct` 字段数 **19**，每字段交给 bool serializer；覆盖 `profile+1f4..206`，包括 Welcome 完成字节。
- 原 `TutorialExponentialBackOff::serialize` **140a65c30**：`deselect_hint`、`focused_edit_hint`、`active_time`；前两项为 **140a65e20** 的 `ExpBackOff { elapsed, current_cooldown }`。旧五个扁平 float 只是运行时接口，不能当磁盘形状。
- 顶层第三字段的原常量 **142ad459d** 及调用长度 **20**，明确是 **game_installation_id**。原值的类型名为 `PlayerId`，其 serializer **1409d46f0** 输出 UUID 连字符文本；旧把 `player_id` 当原字段名的“纠错”有误。
- 原 `load_game_profile` **140916480** 的无文件／读取失败／解析失败三条 fallback 以栈 `rbp+250` 为 profile；UUID 写到 `+418`，而 `+430`、`+440`、`+44f` 的零写覆盖原 `+1e0..206`。因此冷却初值全零。**720秒是 `exponential_back_off` 首次遇零后的初始化阈值，不能提前写入 profile 默认值。**

## 原19字段与当前权威接口

| 原 tutorial 字段 | 原 profile 偏移 | 当前公有字段 |
| --- | --- | --- |
| deselect_complete | 1f4 | tutorial |
| focused_edit_complete | 1f5 | focused_edit_completed |
| camera_pan_complete | 1f6 | camera_pan_completed |
| camera_rotate_complete | 1f7 | camera_rotate_completed |
| camera_zoom_complete | 1f8 | camera_zoom_completed |
| terrain_complete | 1f9 | terrain_layering_completed |
| terrain_steepness_complete | 1fa | terrain_steepness_completed |
| terrain_focus_mode | 1fb | terrain_focus_completed |
| snapping_complete | 1fc | disable_snapping_completed |
| unlink_windows | 1fd | unlink_windows_completed |
| undo_complete | 1fe | undo_completed |
| roof_shape_complete | 1ff | roof_shape_completed |
| focus_clutter_complete | 200 | clutter_focused_edit_completed |
| stair_create_start_complete | 201 | stairs_stage1_completed |
| stair_create_finish_complete | 202 | stairs_stage2_completed |
| stair_focus_complete | 203 | stairs_stage3_completed |
| stair_continue_complete | 204 | stairs_stage4_completed |
| stair_detach_complete | 205 | stairs_stage5_completed |
| welcome_message_read | 206 | hint_reset_flag_206 |

## 修复策略

保留现有公有业务接口与唯一 `GameProfile` 权威。序列化仅建立短寿命投影，写原五个顶层字段、19项 tutorial 记录及原嵌套 cooldowns。反序列化再还原同一公有字段；不在资源里维护第二份教程状态。已知输入逐字段构造，避免为马上覆盖的 `player_id` 无用生成随机 UUID。

此前工程写出的 `tutorial: bool`、扁平 cooldowns 与 `player_id` 仍能读取；下一次保存写回原嵌套格式。旧文件未保存的其它标志无法推断，不能用一个已完成的 deselect 标志伪造全部教程完成。

## 验证边界

`review_tutorial_serde_native.py` 执行原私有 serializer 的完整成功路径，以锁定 RON 的 `serialize_struct`／`serialize_field<bool>`／`end` 为输出桥，保留原字段选择、命名、顺序和偏移。279样本包括全零、全一、交错、19个 one-hot 和256个确定性伪随机 mask。不是 RON 内部格式化或分配失败路径的指令差分。

Rust 测试通过真实完成操作接口设置标志，和原 fixture 逐字段比较，并进行 RON 往返与生产临时文件原子替换写盘／读回。关闭吸附上层测试另外连接 `TutorialHintComplete(3)`→真实 `SaveGameProfileCmd`→重载→`hints_startup`。最终测试结果与源码 hash 收入本轮公共提示收口报告，不以此文单独宣称整 toolbar 验收完成。

## 经验

1. 顶层字段数量不代表叶子字段数量，必须检查真实 `serialize_field<T>` 单态化类型和被传对象地址。
2. “fresh反编译”标签也不能替代原常量及调用长度。20字节字段名与9字节 `player_id` 不可能是同一个名字。
3. 非持久化缺口要由保存／重启测试发现，不得放宽断言让失效链通过。
4. 默认值与首次消费的惰性初始化必须分开取证；提前写入720会改变原状态流。
