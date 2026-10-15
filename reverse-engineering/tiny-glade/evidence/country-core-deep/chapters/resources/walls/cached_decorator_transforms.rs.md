# resources/walls/cached_decorator_transforms.rs

> 文件作用：装饰物缓存变换资源：墙高/几何变化后装饰重定位的缓存；transform 入口对缓存 3x3 矩阵做有限性（NaN/Inf）校验并 panic（0x140a1ada0）。

函数总计 11：业务 7 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position` | 0x140a1a9f0 | 22 | 变换缓存方法族 | O | wall\decomp\decomp-0x140a1a9f0.txt |
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform` | 0x140a1aa20 | 27 | 变换缓存方法族 | O | wall\decomp\decomp-0x140a1aa20.txt |
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_position` | 0x140a1ab80 | 39 | 变换缓存方法族 | O | wall\decomp\decomp-0x140a1ab80.txt |
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_transform` | 0x140a1ac80 | 43 | 变换缓存方法族 | O | wall\decomp\decomp-0x140a1ac80.txt |
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert` | 0x140a1ada0 | 53 | transform（缓存 3×3 矩阵 9 分量有限性校验 + panic） | O | wall\decomp\decomp-0x140a1ada0.txt |
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::swap_buffers_and_clear` | 0x140a1aeb0 | 60 | CachedDecoratorTransforms::swap_buffers（双缓冲字段互换） | O | wall\decomp\decomp-0x140a1aeb0.txt |
| `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage::get` | 0x140a1af40 | 70 | 变换缓存方法族 | O | wall\decomp\decomp-0x140a1af40.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone_from`、`default` 等 4 项
## 实现方式（算法级）

### transform / insert 0x140a1ada0（O，缓存 3×3 矩阵有限性校验）

```
签: (out, id u32, mat 3×3 f32×9 param_3)
校验 1: if ((mat[4]&0x7fffffff)<0x7f800000 && (mat[5]&..)<0x7f800000 && (mat[6]&..)<0x7f800000)
  → 非 NaN/Inf 才继续；否则 panic_str "assertion failed: transform.translation.is_finite()"（0x142ac8e00, line 0x33, file 0x142ac8e80）
校验 2: 拷贝 mat[0..3] → func_0x0001412b7f50（矩阵分量校验/规范化）→ movmskps==0xf（4 分量全有限）
  否则 panic_str "assertion failed: transform.rotation.is_finite()"（0x142ac8e98, line 0x30, file 0x142ac8ec8）
校验 3: if ((mat[7]&..)<0x7f800000 && (mat[8]&..)<0x7f800000 && (mat[9]&..)<0x7f800000)
  → FUN_14152aee0(&stack, out, id, mat) 写缓存；否则 panic_str "assertion failed: transform.scale.is_finite()"（0x142ac8ee0, line 0x2d, file 0x142ac8f10）
```
即 3×3 矩阵 9 分量分批有限性校验（translation[4..6] / rotation[0..3] / scale[7..9]），任一 NaN/Inf → 对应 panic。

### 变换缓存方法族（O）

- `position` 0x140a1a9f0：查缓存取位置（Vec3）。
- `transform` 0x140a1aa20：查缓存取 3×3（校验入口的另一实例）。
- `try_position` 0x140a1ab80 / `try_transform` 0x140a1ac80：Option 版本（未命中返回 None）。
- `CachedDecoratorTransformStorage::get` 0x140a1af40：存储查表。
- `swap_buffers_and_clear` 0x140a1aeb0（双缓冲互换）：两缓冲 param_1/param_2 各 6 qword（0x30B）逐字段互换（[0..5] qword + 8×u32 半字）；随后把 param_2 的 Vec 数据拷入 param_1（0xff 填充 +0x11 偏移 memmove），param_1 的 len 归 0 并重算 cap（(n+1&~7)-(n+1>>3)）。双缓冲 = 上一帧/当前帧，交换后清一册供本帧重写。

### 新常量表

| 串 | 地址 | 语义 |
|---|---|---|
| "assertion failed: transform.translation.is_finite()" | 0x142ac8e00 | mat[4..6] 有限性（line 0x33） |
| "assertion failed: transform.rotation.is_finite()" | 0x142ac8e98 | mat[0..3] 有限性（line 0x30） |
| "assertion failed: transform.scale.is_finite()" | 0x142ac8ee0 | mat[7..9] 有限性（line 0x2d） |

### 跨模块调用

func_0x0001412b7f50（矩阵分量校验）、FUN_14152aee0（写缓存）、FUN_1428d9430（panic_str）。消费于墙高/几何变化后装饰重定位路径。

### 遗留 U

- CachedDecoratorTransforms 存储内部 map/容器布局 U。
- swap_buffers 的 0xff+0x11 memmove 精确 drop 语义 I。
