# systems/prefab.rs

> 文件作用：prefab 实例化系统族：spawn（命令→库查找→RT 实例+音频触发）、update（GPU 实例数据刷新）、despawn（内容清理+实体销毁）、hot_reload（watch→加载 JSON→despawn→audio→rt 全链重建）；PrefabPlugin 注册 2 个 Bevy 类型。

函数总计 14：业务 10 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::prefab::impl$0::build`` | 0x140840430 | 32 | 构造/注册/生成 | O：build = 注册 2 个 Bevy 类型（type-id 0xa73802c6af7eaf33/0x188dddb3c672d859 与 0x98f655b4397350da/0x107a31ef8cad0cb1，I：PrefabLibrary+PrefabsModifiedCmd）+ 系统配置 | session-save\decomp\decomp-0x140840430.txt  |
| ``country_core::systems::prefab::impl$3::clone_behavior`` | 0x140002370 | 61 | 待推断 | O：clone_behavior = xor eax,eax; ret（返回 0/false；13B 小函数） | session-save\DIGEST.md  |
| ``country_core::systems::prefab::impl$3::register_required_components`` | 0x1408446b0 | 61 | 构造/注册/生成 | O：Bevy 注册胶水（ComponentId/registry/drop，非业务） | session-save\decomp\decomp-0x1408446b0.txt  |
| ``country_core::systems::prefab::spawn_prefabs`` | 0x140840610 | 69 | 构造/注册/生成 | O：spawn_prefabs = 事件消费→库查找→实例创建（panic 路径 prefab.rs @0x142a94158；Vec grow 0x140562ab0） | round3\decomp-0x140840610.txt  |
| `country_core::systems::prefab::create_prefab_rt` | 0x140841550 | 148 | 构造/注册/生成 | O：create_prefab_rt = Prefab→位置查表(变体)→四元数旋转矩阵+缩放→GPU 实例缓冲 push+颜色打包 u32(b<<16|g<<8|r)→条件 spawn（+0x84 标志 &1） | session-save\decomp\decomp-0x140841550.txt |
| ``country_core::systems::prefab::create_prefab_gpu_instances`` | 0x140841b00 | 229 | 构造/注册/生成 | O：create_prefab_gpu_instances = 音频触发创建链（audio_trigger_library 0x140a382d0 + 0x1408b9f20；v1 批名 create_prefab_audio_trigger 错位） | session-save\decomp\decomp-0x140841b00.txt  |
| ``country_core::systems::prefab::update_prefabs`` | 0x140841d60 | 264 | 系统:周期性处理 | O：update_prefabs = GPU 实例数据刷新（触 audio_trigger_library.rs 路径） | session-save\decomp\decomp-0x140841d60.txt  |
| ``country_core::systems::prefab::despawn_prefabs`` | 0x140842d60 | 317 | 删除/清理 | O：despawn_prefabs = 逐条目（0x58 步长）despawn_prefab_contents + Commands::despawn(0x140d14770) | session-save\decomp\decomp-0x140842d60.txt  |
| ``country_core::systems::prefab::despawn_prefab_contents`` | 0x140843390 | 344 | 删除/清理 | O：despawn_prefab_contents = 内容清理（0x58 步长条目） | session-save\decomp\decomp-0x140843390.txt  |
| ``country_core::systems::prefab::hot_reload_prefabs`` | 0x140843740 | 363 | 事件 | O：hot_reload_prefabs = watch 事件→加载 JSON（0x140b712b0 族）→despawn_prefab_contents→create_prefab_audio_trigger→create_prefab_rt 全链重建 | session-save\decomp\decomp-0x140843740.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::prefab::create_prefab_audio_trigger`（行 198）— 构造/注册/生成

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项
