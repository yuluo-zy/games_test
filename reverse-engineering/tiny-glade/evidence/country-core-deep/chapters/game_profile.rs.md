# game_profile.rs

> 文件作用：**游戏配置（persisted `profile.ron`）**。`GameProfile` = 0x208 B；ron 字段
> `{player_id, game_installation_id, tutorial, tutorial_cooldowns, key_bindings, new_glade_count}`。
> `load_game_profile` = config_dir()+`profile.ron` → read_to_string → ron from_str；缺失/失败回退默认
> （Uuid::new_v4 + 默认键位）+ `write_profile_via_tmp` + insert_resource；`write_profile_via_tmp` =
> NamedTempFile(`.tmp`) → ron to_string_pretty → persist。`save_game_profile` → write_profile_via_tmp。
> 证据：load/save/write/路径/序列化反编译 + 原字节。
> 注意：字段名修正重建的 `tutorial_completion/tutorial_backoff`（实为 tutorial/tutorial_cooldowns），见 DIGEST §5。
> **reanalyst 2026-10-15 schema 全解**：ron 序列化只写 5 字段（struct 头计数 5）——`tutorial`(+500)、
> `tutorial_cooldowns`(+0x1e0)、`game_installation_id`(+0x1c8)、`key_bindings`(+0x000)、
> `new_glade_count`(+0x1d8)；**`player_id` 不序列化**但 deserialize 接受（0x1404b0ed0 visit_str
> 6 名：tutorial→0、tutorial_cooldowns→1、player_id→2、key_bindings→3、new_glade_count→4、
> game_installation_id→5，raw `mov byte[rax+1]` 逐字节确认）。布局 key_bindings@+0x000 /
> game_installation_id@+0x1c8(Uuid 16B) / new_glade_count@+0x1d8(u32) / tutorial_cooldowns@+0x1e0 /
> tutorial@+0x1f4(bool)；总 0x208B。**消费方**：load_game 0x14146d030 第 3 参 Res<GameProfile>
> （新 glade 计数）；load_completion_checker **不含** GameProfile（其参数=AppMode/LoadingGameComplete
> /Local<LoadingCompletionState>/ActiveSession；skeleton 的 GameProfile 引用是 load_game tuple 注释串）。
> 详见 state/save/RE-DOMAIN-TYPES.md §4。

函数总计 15：业务 6 / 内联无独立符号 0 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `deserialize::visit_str<ron>` | 0x1404b0ed0 | 25 | ron 反序列化 | **O** | 6 字段表（player_id…new_glade_count） |
| `serialize<ron>` | 0x140a658a0 | 25 | ron 序列化 | **O** | 同字段表 |
| `profile_file_path` | 0x140916440 | 86 | `config_dir()+"profile.ron"` | **O** | UNK_142aaca1c len 0xb |
| `load_game_profile` | 0x140916480 | 90 | 载入配置（默认回退） | **O** | read_to_string + ron from_str；log `No profile.ron file found. Creating a new one.` / `Failed deserializing profile.ron. Falling back to defaults. The error was: `；默认回退 + tmp 写 + insert_resource；0x208 B memcpy |
| `save_game_profile` | 0x140917230 | 139 | 保存配置 | **O** | → write_profile_via_tmp；错误 log |
| `write_profile_via_tmp` | 0x140917360 | 145 | 原子临时文件写 | **O** | NamedTempFile(`.tmp` UNK_142ab5b48) → ron to_string_pretty → persist profile.ron；unwrap panic |

## 派生胶水（9 项：default、deserialize、fmt、serialize、visit_map、visit_str 等）

- `impl$0::fmt`（0x1400da0c0）— **O**：Debug fmt（大函数，原字节 0x538 帧）
- `impl$1::default`（0x140916420）— **O**：委托默认构造（FUN_140c102e0）
