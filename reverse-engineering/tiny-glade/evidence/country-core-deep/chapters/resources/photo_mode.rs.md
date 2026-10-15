# resources/photo_mode.rs

> 文件作用：拍照模式资源：PhotoModeCameraMode u8 {0..3}（next 0→1→2→3→0；next_demo 3→panic 'this camera mode is not part of the demo!'）；PhotoModeCmd 事件；PhotoModePlugin 注册；PhotomodeState::on_enable_photomode f32@0=3.0/1.0(demo)+u32@4=-1。与 CameraMode 槽 6..12 联动见 round2 CLOSURE（写入点在 toolbar::photomode，skeleton 缺该 crate）。

函数总计 6：业务 3 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::photo_mode::impl$0::build` | 0x140b59640 | 7 | 插件注册（PhotoModePlugin build） | O：register_event + 组件注册（bevy event/registry.rs 路径证据） | ui-input\decomp\decomp-0x140b59640.txt |
| `country_core::resources::photo_mode::impl$1::from` | 0x14090e500 | 30 | impl$1::from | O（手解）：`lea eax,[rcx+1]; ret` = 输入 +1 的恒等映射（I：CameraMode→PhotoModeCameraMode 或索引+1 转换） |  |
| `country_core::resources::photo_mode::PhotoModeCameraMode::next_demo` | 0x140b596b0 | 40 | 循环下一模式（next_demo） | O：u8 判别 0→1→2→0；3→panic（串 0x142af1188，photo_mode.rs） | ui-input\decomp\decomp-0x140b596b0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`next` 等 3 项

## 实现方式（算法级）

`PhotoModeCameraMode` u8 {0..3}：`next` 0x140b59720 表 {0→1,1→2,2→3,3→0}；`next_demo` 0x140b596b0 表 {0→1,1→2,2→0}，**变体 3 → panic "this camera mode is not part of the demo!"**（串 @0x142af1188/file @0x142af11c8）。`impl$1::from` 0x14090e500 = `lea eax,[rcx+1]; ret`（+1 恒等映射）。PhotomodeState::on_enable_photomode f32@0=3.0/1.0(demo)+u32@4=-1。与 CameraMode 槽 6..12 联动（round2 PARTIAL，写入点 toolbar::photomode）。
