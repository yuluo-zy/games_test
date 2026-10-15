# systems/brush_preview.rs

> 文件作用：**笔刷预览：BrushPreview 组件（克隆行为=判别 2 + 克隆函数指针 0x1412eff70）+ brush_preview 系统（预览网格跟随光标）。**

函数总计 6：业务 3 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::brush_preview::impl$6::clone_behavior` | 0x140867080 | 13 | 返回 ComponentCloneBehavior 枚举：判别 2 + 克隆函数指针 0x1412eff70（手解：lea rdx,[rip+X]; mov eax,2; ret） | O | terrain-water/DIGEST.md |
| `country_core::systems::brush_preview::impl$6::register_required_components` | 0x140866800 | 13 | 注册 BrushPreview 组件所需组件（Bevy 派生胶水，27KB 内联） | O | terrain-water\decomp\decomp-0x140866800.txt |
| `country_core::systems::brush_preview::brush_preview` | 0x140865050 | 40 | 笔刷预览系统：预览网格跟随光标/笔刷大小更新（26KB） | O | terrain-water\decomp\decomp-0x140865050.txt |

## 实现方式（算法级）

**brush_preview 三函数（O，terrain-water 已证）**：
- `clone_behavior @0x140867080`：返回 ComponentCloneBehavior：判别 2 + 克隆函数指针 0x1412eff70（`lea rdx,[rip+X]; mov eax,2; ret`）。（O）
- `register_required_components @0x140866800`：Bevy 派生注册胶水，注册 BrushPreview 所需组件（27KB 内联）。（O）
- `brush_preview @0x140865050`：预览系统，预览网格跟随光标/笔刷大小更新（26KB）；`brush_size`（行 94）为内联的 BrushSizes 读取辅助。（I）

**处理链（I）**：光标移动 → 读 BrushSizes → 更新 BrushPreview 预览网格变换；细节见 terrain-water decomp-0x140865050.txt。（I）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::brush_preview::brush_size`（行 94）— 笔刷尺寸读取（BrushSizes 访问辅助）

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`fmt` 等 2 项
