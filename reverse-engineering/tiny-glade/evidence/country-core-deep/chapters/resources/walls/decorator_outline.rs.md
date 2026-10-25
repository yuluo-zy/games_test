# resources/walls/decorator_outline.rs

> 文件作用：装饰描边数据（DecoratorOutline{尺寸+0x20..,位置+0x30..,判别}/DecoratorCollisionOutline{Lines(Vec<Vec3>)/Sphere}）与调试显示（Mat3+平移逐顶点变换）。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display` | 0x140b33920 | 24 | DecoratorCollisionOutline::display（Mat3+平移逐顶点） | O | wall\decomp\decomp-0x140b33920.txt |
| `country_core::resources::walls::decorator_outline::DecoratorOutline::display` | 0x140b33b00 | 61 | DecoratorOutline::display 闭包 | O | wall\decomp\decomp-0x140b33b00.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
## 实现方式（算法级）

### DecoratorOutline / DecoratorCollisionOutline 结构（O 骨架）

- DecoratorOutline{尺寸+0x20.., 位置+0x30.., 判别}；DecoratorCollisionOutline{Lines(Vec<Vec3>)/Sphere}（判别+0x18 偏移位移）。
- 与现有重建代码一致（DIGEST §1.12）。

### display 0x140b33920（O）

```
签: (self, out 点缓冲 param_2, 变换 param_3, param_4 u8)
FUN_140b04f70(&uStack_90, param_3)     // 查询（WallSpatialHash/碰撞域）
FUN_140b02520(auStack_b8, param_3)     // 变换矩阵构造
分配: FUN_1428d8fe3(uVar21, 顶点数×0xc, &UNK_142aecba0)   // 顶点×12B（Vec3）分配
逐顶点: Mat3 旋转/缩放 + 平移（param_3）→ 写 Vec3 到 out
```

### DecoratorOutline::display 0x140b33b00 / 0x140b348e0（O）

同 display 族闭包（Lines/Sphere 判别 + Mat3+平移逐顶点变换），两个实例化（Lines 与 Sphere 变体），与现有代码一致。

### 跨模块调用

FUN_140b04f70（空间查询）、FUN_140b02520（矩阵）、FUN_1428d8fe3（fmt/分配）。display 供装饰描边调试渲染。

### 遗留 U

- DecoratorOutline 尺寸/位置字段精确偏移表 U（判别+0x18 已证）。
