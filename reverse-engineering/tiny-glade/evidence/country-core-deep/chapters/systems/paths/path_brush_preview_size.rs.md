# systems/paths/path_brush_preview_size.rs

> 文件作用：**路径笔刷预览尺寸（按 UI 模式）**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::paths::path_brush_preview_size::impl$0::register_required_components` | 0x140b1dd70 | 14 | 路径笔刷预览尺寸更新系统（paths 域） | O | terrain-water\decomp\decomp-0x140b1c010.txt

## 实现方式（算法级）

- 系统体 0x140b1c010（SetSizeBasedOnAppModePath）：按 UI 模式设路径笔刷预览尺寸（笔刷尺寸 1.5 分支见 misc-systems 合同；组件注册 0x140b1dd70 为 Bevy 胶水）。
