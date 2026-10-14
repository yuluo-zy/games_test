# systems/ui_camera_mode_gizmos.rs

> 文件作用：**相机模式 gizmo 签名符（小窗口）**——四重门槛：AppMode∈{2,10}（10 需 flag=0）、
> 相机非 Free/Walk（(mode&6)==2 返回）、窗口<3px、签名符模式∈{0,1,4}（0x13>>mode&1，6→0 映射）；
> 通过后 UiState::add("camera_mode_activerotation_")（状态 int≠20 时）→ 0x20B UiAppendSignifier
> {1,1,id_string,size,fov@+0xe8} → AppendOnlyVec。闭包=渲染回调（cursor_grab 部件 + vertex_color_shader，
> 动画 value/1000·20，clamp 25.0@1429edb98）。

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ui_camera_mode_gizmos` | 0x1409527a0 | 24 | 见文件作用 | O：实证 | camera-time\decomp\decomp-0x1409527a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `cursor_icon`（行 89）— 光标抓取图标签名符构造（并入闭包 0x140952b60：cursor_grab 部件 + 动画，消费 CameraRotationPivot::use_camera_cursor_grab_icon 谓词）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
