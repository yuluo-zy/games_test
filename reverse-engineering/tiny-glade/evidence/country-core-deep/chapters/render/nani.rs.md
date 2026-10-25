# render/nani.rs（mod）

> 文件作用：nani 模块根：`InstanceHandle`（12B，u32 对 + kind 的薄封装）+ `InstanceDataMarker<T>` 组件（`NaniActiveInstance` 的 required-component 标记）。

## 内联/无独立符号函数

- `country_core::render::nani::InstanceHandle::subset`（行 48）— 取 kind 字段 → GpuWorldSubsetKind —— I（12B `{id:u32,index:u32,kind:u8}`，见 gpu_world 章 InstanceKey）
- `country_core::render::nani::impl$3::component_ids`（行 53）— `InstanceDataMarker<T>` 的 bevy bundle 处理序（声明序）—— O（派生展开，B_n_world §1）
- `country_core::render::nani::impl$5::get_components`（行 53）— 同上 bevy 派生查询—— O（派生展开）

## 派生胶水（1 项：clone 等）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- 模块根角色：`InstanceDataMarker<T>` 是 19 个 `impl$36::register_required_components`（0x141572d80 系）注册的 required-component 标记——每类 `InstanceData<T>` 一个零大小组件，`nani_update_*` 系统用 `With<InstanceDataMarker<T>>` 查询定位要打包的实例（systems 章 §1）。
- `InstanceHandle`/`InstanceKey` 关系：`components::drawable::NaniActiveInstance` = `InstanceKey` 别名（12B `{id,index,kind}`）；`subset()` 读 kind→`GpuWorldSubsetKind`（gpu_world 章 27 变体）。
- 数据流：maintain 把 12B key 写进 `Insert<NaniActiveInstance>` 命令 → ECS 组件持 key → update 系统按 key 定位 compact 槽 → `nani_draw` 按 kind 选 draw list。
- 注：本文件由模块 agent 填充（原本"待推断"），O 事实引用 gpu_world/systems 章与 B_n_world §1；`InstanceDataMarker` 的 bevy 派生不算独立业务（项目规则）。

