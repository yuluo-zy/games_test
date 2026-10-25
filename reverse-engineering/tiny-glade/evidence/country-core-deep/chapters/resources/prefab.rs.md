# resources/prefab.rs

> 文件作用：prefab 库资源：hashbrown HashMap<PrefabName, Prefab>（RandomState；桶 0x28B = hash 8B + 键 16B + 值 16B；Prefab 值为 {ptr,len} 胖指针类——reconstruction 的 Vec 布局存疑）+ Bevy 组件注册。

函数总计 8：业务 4 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::resources::prefab::PrefabLibrary::add`` | 0x1408e2540 | 27 | 构造/注册/生成 | O：HashMap::insert（被替换旧值逐 PrefabPart drop） | round3\decomp-0x1408e2540.txt  |
| ``country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> >`` | 0x141552d70 | 31 | 查表/访问 | O：hashbrown 查表（0x1405b1a70 哈希 + SIMD group probe + memcmp）返回 &Prefab | session-save\DIGEST.md  |
| ``country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabNameRef> >`` | 0x141552ff0 | 42 | 待推断 | O：同 get 返回 bool（contains_key） | session-save\DIGEST.md  |
| ``country_core::resources::prefab::impl$7::register_required_components`` | 0x1408e5d10 | 108 | 构造/注册/生成 | O：Bevy 组件注册（PrefabInstance/PrefabInstanceData 等） | round3\decomp-0x1408e5d10.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::prefab::PrefabLibrary::iter`（行 46）— 迭代/变换

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default` 等 3 项

## 实现方式（算法级，2026-10-15 深挖）

- **PrefabLibrary（hashbrown HashMap<PrefabName, Prefab>）**：桶 0x28B = {hash u64,
  PrefabName 16B, **Prefab 值 16B {ptr,len} 胖指针类**}（0x141552d70 get / 0x141552ff0
  has：FUN_1405b1a70 哈希 + SIMD group probe + memcmp 0x142923bf0）——
  reconstruction 的 `Prefab { parts: Vec<PrefabPart> }`（24B）布局存疑（DIGEST #16）。
- **add 0x1408e2540**：HashMap::insert（被替换旧值逐 PrefabPart drop）。
- register_required_components 0x1408e5d10：Bevy 组件注册胶水（非业务）。
