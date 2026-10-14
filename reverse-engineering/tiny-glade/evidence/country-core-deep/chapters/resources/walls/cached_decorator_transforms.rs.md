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
