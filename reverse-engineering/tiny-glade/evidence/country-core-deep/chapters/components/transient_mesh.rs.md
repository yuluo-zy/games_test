# components/transient_mesh.rs

> 文件作用：**瞬时网格组件与注册表：`TransientMeshRegistry`（Mutex+待删 Vec）、`TransientMesh` 组件（持 Arc 注册表，Drop 时登记删除）。backend `delete_dropped_transient_meshes` 每帧排空释放 GPU 网格。**

函数总计 6：业务 5 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>` | 0x1412e9e70 | 12 | `drain_meshes_to_delete<F>`：锁 + 遍历待删 Vec（cap-first {cap@+8,ptr@+0x10,len@+0x18}，元素 8B），对每个句柄调闭包（闭包内两次删除 FUN_14092d750/FUN_1409247a0），清空 len。 | **O** | decomp-0x1412e9e70.txt |
| `country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh` | 0x140aa8070 | 18 | `deferred_delete_mesh`：锁注册表 Mutex（@+0x0），push 句柄进 Vec@+0x8（cap-first，grow FUN_140562870 元素 8B）。 | **O** | decomp-0x140aa8070.txt |
| `country_core::components::transient_mesh::impl$4::clone_behavior` | 0x140002370 | 24 | 共享 `clone_behavior` 0x140002370 = `xor eax,eax; ret`（返回 0 = 无克隆行为）。Bevy 默认胶水。 | **O** | decomp-0x140002370.txt |
| `country_core::components::transient_mesh::impl$4::register_required_components` | 0x140aac640 | 24 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x140aac640.txt |
| `country_core::components::transient_mesh::TransientMesh::new_with_registry` | 0x140aa8200 | 32 | `new_with_registry`：入口 = Arc clone（`lock inc [ptr]` + 溢出检查）→ 构造 TransientMesh（存 Arc + 句柄）。机器码 0x140aa8200。 | **O** | decomp-0x140aa8200.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 1 项：drop 等（derive 直接复用，不计业务）。
