# resources/editing_highlight.rs

> 文件作用：**编辑高亮资源（wall_highlight 写 rgba 的载体）。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::editing_highlight::EditingHighlight::clear | 0x140976890 | 16 | EditingHighlight::clear：高亮数据清零 | I |  |

## 实现方式（算法级）（I，无 decomp 目标）

**EditingHighlight::clear @0x140976890**：编辑高亮数据清零——wall_highlight 写 rgba 的
载体资源；清除语义按名称与消费域推断（高亮缓存重置），反编译未覆盖（audio-misc 批
无该地址）。I
- 其余 = default 派生胶水。I

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
