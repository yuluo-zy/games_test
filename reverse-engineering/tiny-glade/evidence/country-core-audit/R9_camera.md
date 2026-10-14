# R9：MainCamera / Free 相机（D 包）

日期：2026-10-14。落点：`tiny-glade/crates/country-core/src/systems/camera_rig/{main_camera,free_camera}.rs`，`Cargo.toml` 增加 `dolly = "=0.6.0"`。

## 依赖
`dolly 0.6.0`（`Cargo.lock` 里 `dolly → glam 0.27.0 + mint 0.5.9`，校验和与 `tracking/original-Cargo.lock` 的 dolly / glam 0.27.0 / mint 逐项一致；cargo 默认会把 dolly 的 glam 解析到已有的 0.25.0，已手工对齐到原版的 0.27.0）。

## EXE 核对
| 项 | 地址 | 结果 |
|---|---|---|
| `CameraMode::previous` | 0x140914ac0 | `movabs rax,0x40302010006; shl cl,3; shr rax,cl`：移位数取低 6 位，所以模式 8..12 回绕到 `mode & 7`（`previous(8)=6`）。DIGEST 只记了 8 项表 |
| fov 偏移表 | 0x142aabb98 | 13 个 u64：`8c,144,1c4,264,31c,3a4,8,20,14,2c,8,14,c` |
| `target_vertical_fov_degrees` | 0x1409114d0 | `[self + table[mode] + 0x108]` |
| `immediately_set_vertical_fov_degrees` | 0x1409114f0 → 0x140c974f0 | 基址 `self+table+0x100`，写 `+8` 与 `+4` |
| `store_parameters` | 0x140911710 | 输出 0x24 B：yaw@0（读 +0x154）、pitch@4（读 +0x150）、position@8..0x14（读 +0x138）、0@0x14、fov@0x18（读 +0x190）、0@0x1c、0@0x20；非有限值判定 `bits & 0x7fffffff > 0x7f7fffff`；位置默认 `(0,5,0)`（0x142aab4c0 实读）；日志门是 `*max_level != 0` 即 Error 级，目标 `country_core::systems::camera_rig`（33 B 串实读），四条文案逐条实读 |
| `match_free_camera_to_normal` | 0x14090e6f0 | `Position(+0x138..)`、`YawPitch{yaw=[+0x154], pitch=[+0x150]}`（8 B 里高低字交换）、`Smooth`（第三个 driver 0x40 B，`+0x30..+0x3c` = 1,1,1）；初始变换（0x142aece40）= 旋转 `(0,0,0,1)` + 零位置 |
| `get_position` / `get_yaw_pitch` / `update` | 0x140992f10 / 0x140993010 / 0x140993d30 | 就是 dolly `CameraRig::driver_mut` 和 `update`（TypeId `0x6b84…` / `0x718e…`，panic 串即 dolly rig.rs） |

纠正：DIGEST 的 Normal body 表把 yaw/pitch 偏移写反（"yaw +0x50 / pitch +0x54"）；`store_parameters` 与 `match_free_camera_to_normal` 一致证明是 pitch@+0x150、yaw@+0x154。

## 未做（原因）
* `MainCamera::new`（0x14090ec70，853 行）、六个 body 的默认值与类型化字段。
* `main_camera_update` 的两张跳表（0x142aabd04 / 0x142aabd30）：各模式分支的反编译有损坏；`paused_main_camera_update` 的 case 6/11 同样损坏。
* `FreeCamera::translate_screen_space`（0x1409938b0）：依赖未解的地形采样 0x1409e9640。
* `CameraSnapshot` 插值、`chase_butterfly`、`reset_camera`、Walk/Normal/Butterfly 的 body。
所以 `MainCamera` 现在是 0x4e0 字节原始缓冲加已核对偏移的读写器，不是完整类型。

测试 +10。
