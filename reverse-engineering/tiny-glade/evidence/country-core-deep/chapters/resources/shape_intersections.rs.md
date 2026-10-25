# resources/shape_intersections.rs

> 文件作用：**ShapeIntersections 标记/容器资源**——`ShapeIntersections::default`（行 8）唯一业务面是 `Default`。
> 该资源记录形状（Rectangle/Roof 等）之间的求交结果，供 ui/hover 与 shape 吸附系统消费（交叉域：systems/ui/hover_intersection、systems/wall/shapes）。
> 本章只有派生胶水，无独立业务函数。

函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（`impl$0::default`，`#[derive(Default)]`）
