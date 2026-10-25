# utils/child_wall.rs

> 文件作用：**墙分割子墙工具（游戏自有，wall 域）**。`ChildWall {a, b}` 子墙；`slice` 读
> a.y/a.z 两标量分别 `f32::clamp` 入区间，越界/NaN panic，返回 `{param_3, t1, t2}`。
> 注意：重建签名 `slice(&self, t) -> (ChildWall, ChildWall)` 与机器码返回形不符（机器码返
> 3 值：param_3 原样 + 两个 clamp 结果），见 utils-input-config DIGEST §7。

## 核心算法

```
ChildWall::slice(t):
  t1 = clamp(a.y@+8, min, max)   // a = {…, y@+8, z@+0xc}
  t2 = clamp(a.z@+0xc, min, max)
  越界/NaN → panic（UNK_142aa53c0 / UNK_142ac9aa0 / UNK_142ac9ab8，f32::clamp）
  返回 {param_3, t1, t2}         // param_3 透传（调用方上下文，语义 U）
```

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::utils::child_wall::ChildWall::slice` | 0x140a24000 | 29 | 按参数切分子墙 | **O** | a.y@+8/a.z@+0xc 双 clamp + [0,1] 越界/NaN panic(UNK_142aa53c0/0x142ac9aa0/ab8)；返 {param_3, t1, t2} |

## 内联/无独立符号函数

- `ChildWall::new`（行 17）— **O**（构造 {a, b}）
