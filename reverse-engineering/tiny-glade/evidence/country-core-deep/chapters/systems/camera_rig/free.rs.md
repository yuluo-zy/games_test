# systems/camera_rig/free.rs

> 文件作用：**FreeCamera**——dolly rig（Position+YawPitch+Smooth(1,1)）封装：driver 按 typeid
> 查找（Position=0x6b8463577afc1a66/YawPitch=0x718e4fd258330fe5）、屏幕空间平移（±91 clamp、
> 地形+0.75 贴地、20/60 步长、smooth(dt)）、yaw 绕 720° fmod + pitch 钳 [-90,90]。

函数总计 9：业务 9 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$1::translate_screen_space` | 0x1409938b0 | 0 | 屏幕空间平移：quat 旋转移动→clamp xz ±91→terrain.max(0)+0.75 贴地→smooth(dt)=1−exp(−8dt)→delta×20/60→Position.translate | O：全常量实证（±91@142aba9b4/8、0.75、20/60） | camera-time\decomp\decomp-0x1409938b0.txt |
| `FreeCamera::get_position` | 0x140992f10 | 18 | typeid 查 Position driver → sret Vec3 | O：实证 | camera-time\decomp\decomp-0x140992f10.txt |
| `FreeCamera::get_yaw_pitch` | 0x140993010 | 22 | typeid 查 YawPitch driver → (yaw,pitch) 首 8B | O：实证 | camera-time\decomp\decomp-0x140993010.txt |
| `FreeCamera::set` | 0x140993100 | 27 | 重建 rig：alloc Position(12B)+YawPitch(8B)+Smooth(0x40B, {0,0,0,0,fnptr,1.0f}) 三 driver → dt=0 跑一遍存 final_transform | O：实证（与 match_free_camera_to_normal 同构建器） | camera-time\decomp\decomp-0x140993100.txt |
| `impl$1::rotate_yaw_pitch`（free/walk 共享） | 0x140993750 | 44 | YawPitch driver_mut(vtable+0x40)：`yaw=fmodf(yaw·0.5+old, 720)`、`pitch=clamp(old+pitch·0.5, −90, 90)` | O：实证（720@142aba9ac、±90@142aba9b0/1429edb94） | camera-time\decomp\decomp-0x140993750.txt |
| `impl$1::constrain_to_be_above_terrain` | 0x14003bb80 | 58 | 空操作（折叠 trampoline，仅 `ret`） | O：实证 | camera-time\decomp\decomp-0x14003bb80.txt |
| `impl$1::update`（free/walk 共享） | 0x140993d30 | 103 | dolly rig 迭代：逐 driver vtable+0x30(driver, parent_transform, dt)，写回 rig+0x18..0x34，sret 28B {pos,rot} | O：实证 | camera-time\decomp\decomp-0x140993d30.txt |
| `walk::impl$2::rotate_yaw_pitch` | 0x140993750 | 44 | 同 free（共享函数） | O：同上 | camera-time\decomp\decomp-0x140993750.txt |
| `walk::impl$2::update` | 0x140993d30 | 103 | 同 free（共享函数） | O：同上 | camera-time\decomp\decomp-0x140993d30.txt |

## 实现方式（算法级）

```
rotate_yaw_pitch（0x140993750，free/walk 共享）：
  查 YawPitch driver（typeid 0x718e4fd258330fe5，vtable+0x40）→ 未找到 panic "No <…YawPitch…> driver found in the CameraRig"
  yaw   = fmodf(yaw   + delta_yaw·0.5, 720.0)     // 0.5@0x142925870、720@0x142aba9ac
  pitch = clamp(pitch + delta_pitch·0.5, −90, 90) // ±90@0x142aba9b0/0x1429edb94
translate_screen_space（0x1409938b0）：
  quat 旋转移动 → clamp xz ±91（0x142aba9b4/8）→ terrain.max(0)+0.75 贴地
  smooth(dt)=1−exp(−8dt) → delta×20/60 → Position.translate
get_position/get_yaw_pitch：typeid 查 driver sret。
set（0x140993100）：重建 rig Position+YawPitch+Smooth(1,1)（Smooth 0x40B 初始化）→ dt=0 跑一遍。
update（0x140993d30，free/walk 共享）：rig 迭代 vtable+0x30 写回 final_transform@+0x18..0x34，sret 28B。
constrain_to_be_above_terrain → 折叠 0x14003bb80（ret，Free 不约束）。
```
