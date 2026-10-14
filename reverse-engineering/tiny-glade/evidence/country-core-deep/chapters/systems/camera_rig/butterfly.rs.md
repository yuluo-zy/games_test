# systems/camera_rig/butterfly.rs

> 文件作用：**ButterflyCamera（蝴蝶追踪相机）**——dolly rig（Position+Rotation+Smooth(2,predictive)+
> Arm(0,1.5,3.5)+Smooth(4)+LookAt(tracking 4)，布局 {rig@+0x0、selected_index@+0x38、
> applied_index@+0x3c}；选中索引变化时 dt=1000 瞬时吸附；rotate/constrain/translate 三操作
> 折叠到空 trampoline（0x14003bb80 = 仅 `ret`）。相机书签缩放换挡（advance 按 zoom 符号）。

函数总计 4：业务 4 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$0::constrain_to_be_above_terrain` | 0x14003bb80 | 20 | 空操作（蝴蝶 body 不约束地形） | O：`ret` trampoline | camera-time\decomp\decomp-0x14003bb80.txt |
| `impl$0::rotate_yaw_pitch` | 0x14003bb80 | 20 | 空操作（同上） | O：`ret` trampoline | camera-time\decomp\decomp-0x14003bb80.txt |
| `impl$0::translate_screen_space` | 0x14003bb80 | 20 | 空操作（同上） | O：`ret` trampoline | camera-time\decomp\decomp-0x14003bb80.txt |
| `impl$0::update` | 0x140a0d090 | 37 | `selected(+0x3c) != applied(+0x38)` → applied=selected、dt=1000（@14295b630）→ rig 迭代 vtable+0x30 → sret 28B | O：实证 | camera-time\decomp\decomp-0x140a0d090.txt |
