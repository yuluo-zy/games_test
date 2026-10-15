# resources/walls/dormers.rs

> 文件作用：老虎窗洞 SSBO 记录：DormerHoleSsbo 48B（gothic 4×f32 尾区 / cottage 2×f32+f64 尾区）与 DormerHoleCountPerCellSsbo 计数。

函数总计 4：业务 3 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::dormers::DormerHoleSsbo::dummy` | 0x141286660 | 44 | DormerHoleSsbo::dummy（3×16B 清零 = 全零 48B） | O | wall\decomp\decomp-0x141286660.txt |
| `country_core::resources::walls::dormers::DormerHoleSsbo::gothic` | 0x141286680 | 58 | DormerHoleSsbo::gothic（0x30B 构造，尾区 4×f32） | O | wall\decomp\decomp-0x141286680.txt |
| `country_core::resources::walls::dormers::DormerHoleSsbo::cottage` | 0x1412866e0 | 70 | DormerHoleSsbo::cottage（0x30B 构造，+0x28 处 f64） | O | wall\decomp\decomp-0x1412866e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`eq` 等 1 项
## 实现方式（算法级）

### DormerHoleSsbo 构造器（O，手解，DIGEST §1.12 深化）

- `dummy` 0x141286660：3×16B 清零 = 全零 48B（0x30B 记录）。
- `gothic` 0x141286680：构造 0x30B 记录 {u64@0, u32@+8, u32@+0xc, u64@+0x10, u32@+0x18, u32@+0x1c=0, **f32×4@+0x20..0x2c**}（尾区 4×f32）。
- `cottage` 0x1412866e0：同构，但 +0x28 处写 **f64**（movsd 全局常量）——cottage 尾区 = 2×f32 + f64（gothic 4×f32 vs cottage 2×f32+f64 差异 O）。
- `DormerHoleCountPerCellSsbo`：计数（I，serde/derive 胶水）。

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| f64 常量 | cottage 构造器 movsd 源 | +0x28 尾区 |

### 遗留 U

- 构造器各字段（u64@0/u32@+8 等）语义（窗洞位置/尺寸/朝向）U。
- DormerHoleCountPerCellSsbo 布局 U。
