# resources/drawing_input_smoother.rs

> 文件作用：**绘制输入平滑器：把徒手原始输入折线做弧长均匀重采样 + 最小间距抽稀，输出 (x,z) 点对序列（drawing 类工具的输入预处理）。**

函数总计 4：业务 1 / 内联无独立符号 3 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::drawing_input_smoother::DrawingInputSmoother::process` | 0x14145c350 | 30 | 沿输入折线 t=i/N 参数步进 lerp 取样，最小间距门控（fVar6<dist && fVar7<dist*fVar11）抽稀，输出 8B(x,z) 对入 Vec，回写首点+置位标志 | O | terrain-water\decomp\decomp-0x14145c350.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::drawing_input_smoother::RawInputSmoothed::point_count`（行 16）— 平滑后点数
- `country_core::resources::drawing_input_smoother::RawInputSmoothed::is_empty`（行 20）— 平滑后点列空谓词
- `country_core::resources::drawing_input_smoother::RawInputSmoothed::points`（行 24）— 平滑后点列视图
