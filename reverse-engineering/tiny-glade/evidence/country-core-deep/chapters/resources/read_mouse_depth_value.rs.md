# resources/read_mouse_depth_value.rs

> 文件作用：**鼠标深度回读：MouseDepthValue {w f32@0, xyz@+8, 帧哨兵@+0x40}，来自 MouseDepthValueSsbo（2×float4）的 GPU→CPU 映射回读。**

函数总计 4：业务 1 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::read_mouse_depth_value::read_mouse_depth_value_from_gpu` | 0x140a1a480 | 33 | 帧哨兵@+0x40==i64::MIN+3 早退；查 SSBO（名 0x47=71 字符）→ 读 2×float4 做 NaN 过滤（&0x7fffffff<0x7f800000）→ 写 {w, xyz} | O | terrain-water\decomp\decomp-0x140a1a480.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::read_mouse_depth_value::MouseDepthValue::new`（行 28）— 构造（帧哨兵初始化）
- `country_core::resources::read_mouse_depth_value::from_ndc_to_view`（行 118）— NDC→视空间坐标转换（深度值辅助）

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）

`read_mouse_depth_value_from_gpu`（0xa1a480）：帧哨兵@+0x40 == i64::MIN+3 早退（帧有效性）；查 MouseDepthValueSsbo（名 0x47=71 字符）→ 读 2×float4 → NaN 过滤（`&0x7fffffff < 0x7f800000`）→ 写 {w f32@0, xyz@+8}。生产者 = mouse_depth_compute（frame compute）。
