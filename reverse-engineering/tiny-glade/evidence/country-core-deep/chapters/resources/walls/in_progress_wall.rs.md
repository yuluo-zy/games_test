# resources/walls/in_progress_wall.rs

> 文件作用：编辑中墙：InProgressWall 组件与 InProgressWallResource（Option 语义）；徒手墙绘制输入到预备轮廓到提交的前置状态（finish_wall 取 .wall.take()）。

函数总计 2：业务 0 / 内联无独立符号 1 / 派生胶水 1

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::in_progress_wall::InProgressWall::new`（行 31）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`fmt` 等 1 项
## 实现方式（算法级）

### 该文件无独立业务函数（依据：sysres-procedures.json 不含 in_progress_wall.rs）

sysres-procedures.json 无 `resources/walls/in_progress_wall.rs` 键。InProgressWall 组件 / InProgressWallResource 的业务（徒手墙输入→预备轮廓→提交，finish_wall 取 .wall.take()）**内联于** draw_wall 0x140a7cba0 / finish_wall 0x140a800a0 / ui_draw_wall 0x140aafe40 等系统；`InProgressWall::new`（行 31）为内联构造，无独立符号。

### 遗留 U

- InProgressWallResource 的 Option<InProgressWall> 布局（Option 语义先验）U。
