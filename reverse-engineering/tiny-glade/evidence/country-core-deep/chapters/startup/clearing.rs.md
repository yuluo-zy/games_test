# startup/clearing.rs

> 文件作用：**clearing（林间空地）初始化/反初始化：载入 clearing 网格与 shader、反初始化时恢复音频 snapshot。**

函数总计 11：业务 3 / 内联无独立符号 1 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140a14620 | 56 | `visit_str<serde_json::Error>`：serde 枚举字符串反序列化胶水。长度分派跳表（len-6 ∈ [0,19]，表 0x142ac7dbc）+ 常量比较 → 变体判别：`prefab`(6)→0、`placements`(10)→1、`particle_emitter`(16)→2、billboard 族(24)→3、未知→4；写 out{@0=0, @1=判别}。 | **O** | hex 手解（rne-read+capstone）→ startup/decomp/decomp-0x140a14620.txt；原 tg_b2 NotFound |
| `country_core::startup::clearing::init_clearing` | 0x140a78aa0 | 70 | `init_clearing`：载入 clearing 网格 `assets/meshes/{ice_cracks,distant_billboard}.json` + `clearing/tree/bush`，注册 `ice_cracks/water/ice_cracks.raster.hlsl` shader，deserialize `trees.json`（clearing 外围树）。 | **O** | decomp-0x140a78aa0.txt |
| `country_core::startup::clearing::uninit_clearing` | 0x140a7a5b0 | 265 | `uninit_clearing`：恢复 FMOD `snapshot:/Loading`（音频快照），卸载 clearing 状态。 | **O** | decomp-0x140a7a5b0.txt |

## 内联/无独立符号函数

- `country_core::startup::clearing::util_add_clearing_shader`（行 41）— **I**：`util_add_clearing_shader` — 添加 clearing shader 的内联 helper。

## 派生胶水

- 7 项：closure$0, closure$1, deserialize, expecting, visit_map, visit_seq 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **init_clearing 0x140a78aa0（26KB，O）** 顺序：
  1. 名式网格 2x（load_mesh_into_library：tree/bush）+ 文件式 2x（'assets/meshes/ice_cracks.json'、
     'assets/meshes/distant_billboard.json' 0x142ad6e71）
  2. **直接调用 startup_flowery（0x141553120）+ startup_olden（0x1415539d0）** ← 跨模块主题装载边
  3. clearing shader 注册（ice_cracks/water + ice_cracks.raster.hlsl 族，util_add_clearing_shader 内联）
  4. deserialize trees.json（'deserialize trees' 0x142ad6e18）→ 注册记录（FUN_1412dd460 等记录 fn-ptr）
  5. 6x unwrap panic（'called Result::unwrap…'，0x142ad69a0 区）
- **uninit_clearing 0x140a7a5b0（O）**：恢复 FMOD `snapshot:/Loading`（0x142ad6ef0）+ 卸载状态。
