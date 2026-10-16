# REVIEW4 — systems/save（组 state 终验：卡1 域类型 + GameSaveData 组装）

> 审查员：reviewer-state，2026-10-15。复核 restorer 卡1 落地（4 域类型 + assemble_game_save_sections）
> 与 RE-DOMAIN-TYPES.md 逐字段一致性。编译基线实测：`cargo check -p country-core --offline` 通过；
> 本组（save/session/music/meta_commander/game_profile/glade_color/history）**49 通过 0 败**（= 19/19 声明核对）；
> 全 crate **1598/1**（唯一失败 = `geometry::sphere::tests::uv_formula_matches_the_closure`，components-geometry 组，非 state）。

## 逐类型核对（vs RE-DOMAIN-TYPES.md）

| 类型 | verdict | 说明 |
|---|---|---|
| `CameraRigParametersCache`（systems/camera_rig/mod.rs） | **OK** | 6 字段 yaw/pitch/position/camera_rig_arm_length/pitch_offset/yaw_offset（0x24B 语义）；position `#[serde(default)]` 且 Default=(0,5,0)；6 字段修正旧 7 字段表。与 §1 一致。 |
| `HistoryIdentity`（resources/history/mod.rs） | **OK** | `#[serde(transparent)] pub struct(u64)`；`generate`=fastrand::u64(..)（0x140a84220）；`mutate` 重随机。与 §2 一致。 |
| `GladeColorCustomization`（resources/glade_color_customization.rs） | **OK** | 15 字段按 visit_str index 0..0xe 声明（freeform_wall/building/roof/building_wall_style/freehand_ty/halftimber/lantern_light/fence/window_light/flag/windows/gates/flat_rim_style/lantern_brightness/stairs）；3 字节枚举（+0x40/41/42）以 u8 登记（变体名待取证，诚实）；ColorId=transparent u32。与 §3 一致。 |
| `GameProfile`（game_profile.rs） | **PARTIAL(低)** | 5 序列化字段 + `player_id #[serde(skip_serializing, default=Uuid::new_v4)]` + default=new_v4，均与 §4 一致；**ron 字段声明序与原文 serialize 序不同**：src 序 key_bindings→game_installation_id→new_glade_count→tutorial_cooldowns→tutorial；原文 serialize 序 tutorial→tutorial_cooldowns→game_installation_id→key_bindings→new_glade_count（§4②）。ron 按名匹配，读写兼容但**字节序不同**。建议按原文 serialize 序重排字段（低）。 |
| `ron` 依赖 | **OK** | workspace `ron = { git = "https://github.com/PounceLight/ron", rev = "2787d8c" }` == 原锁 `git+…/ron?rev=2787d8c`（0.8.1）。版本来源确认。 |

## assemble_game_save_sections 核对
| 项 | verdict | 说明 |
|---|---|---|
| Version/Identity/Camera/Colors 由域类型串化 | **OK** | version(i32)/HistoryIdentity(透明 u64)/CameraRigParametersCache(6 字段)/GladeColorCustomization(15 字段) 直接 `SerializedJson::new`。 |
| 其余 5 节由域提供 | OK | time_of_day/sheep/chain_index/history_start/snapshots 为调用方传入的 `SerializedJson`；缺参即编译错误（10 节全必需），不伪造。 |
| BackwardCompatibility | PARTIAL(低) | src 写 `0u8`；原版兼容节值为 Value，未独立取证（DIGEST 仅记"兼容节（Value）"）。建议登记待取证。 |
| "缺失 panic"语义 | OK | 每节必写由参数必需性保证；无占位空值。 |

## load_completion_checker / save_file_identity 澄清
| 项 | verdict | 说明 |
|---|---|---|
| load_completion_checker 不含 GameProfile（代码签名） | OK | `check_load_completion(SessionId)` / `is_session_ok_to_load(SessionId, &UnsavedFileList)` 均不含 GameProfile；SystemParam 面（AppMode/EventReader<LoadingGameComplete>/Local<LoadingCompletionState>/ActiveSession）由宿主装配。 |
| load_completion_checker **头注释过期** | **FIX(低)** | 第 9 行注释仍写"系统级接线依赖 GameProfile/MetaCommander 等未建资源"——与 RE-DOMAIN-TYPES §4④（不含 GameProfile）矛盾；代码正确，注释需更新。 |
| save_file_identity = HistoryIdentity 别名 | **OK** | `pub type SaveFileIdentity = HistoryIdentity`；try_approx_parse 返回 HistoryIdentity。 |

## 结论与 DISPATCH 建议
- **卡1（域类型 + GameSaveData 组装）→ 闭合**（域类型+组装部分）。4 类型逐字段与 RE-DOMAIN-TYPES 一致；assemble 不伪造、每节必写。
- 低优先待行动：①GameProfile 字段声明序对齐原文 serialize 序（ron 字节序）；②load_completion_checker 头注释更新（去掉 GameProfile 依赖）；③BackwardCompatibility 取值登记待取证。
- **仍阻塞项（登记）**：save_game 本体完整资源组装系统（0x140a3f470 的 ECS 参数面 + AsyncSaveCmd 发送）、capture_game_snapshot 域 Query（0x1417a1d00）、commander_update（0x14091f180）、全局接线。
- 模块状态建议：save → **review_passed（限定已实现范围）**，卡1 闭合登记；完整 save 验收仍 blocked（上述阻塞项）。session/music 维持前轮 review_passed（本轮无改动）。
