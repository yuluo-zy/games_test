# components/transient_mesh.rs

> 文件作用：**瞬时网格组件与注册表：`TransientMeshRegistry`（Mutex+待删 Vec）、`TransientMesh` 组件（持 Arc 注册表，Drop 时登记删除）。backend `delete_dropped_transient_meshes` 每帧排空释放 GPU 网格。**

函数总计 6：业务 5 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `TransientMeshRegistry::drain_meshes_to_delete<delete_dropped_transient_meshes::closure_env$0>` | 0x1412e9e70 | 12 | **业务** 排空待删队列并逐句柄调闭包：见下方生命周期算法。闭包捕获两个 ResMut（tick@+0x1c 回写模式，PLAYBOOK §2.6），对每句柄调 `AssetMeshLibrary::remove`（0x14092d750）+ `AssetVAOLibrary::remove`（0x1409247a0）——**释放 GPU 网格与 VAO**。 | **O** | decomp-0x1412e9e70.txt |
| `TransientMeshRegistry::deferred_delete_mesh` | 0x140aa8070 | 18 | **业务** 锁队列 push 句柄：std::sync::Mutex fast-lock（state 字节 CAS 0→1，慢路径 FUN_1428d8750=std futex Mutex::lock_contended）；毒化 → 格式化 panic `called \`Result::unwrap()\` on an \`Err\` value`（0x142adcf70）+ transient_mesh.rs（0x142add2f0）——**`.lock().unwrap()` 语义，非 `if let Ok`**；len==cap 时 grow FUN_140562870（8B 元素）；句柄写入 ptr[len]，len+1；unlock（state==2 时 WakeByAddressSingle 唤醒等待者）。 | **O** | decomp-0x140aa8070.txt |
| `impl$4::clone_behavior` | 0x140002370 | 24 | 胶水：策略归类 — 共享 `xor eax,eax; ret` = `ComponentCloneBehavior::Default`（TransientMesh 未 derive Clone）。 | **O** | 机器码 0x140002370 |
| `impl$4::register_required_components` | 0x140aac640 | 24 | 胶水：策略归类 — Bevy register 单态化，**无 required**（TypeId 0xdd365ed488e04adf/0x27040080e4d856fa 命中 `component_ids<TransientMesh,…>`/`register_info<TransientMesh>`；0x22c9a12b771fb521 为 hashbrown 哈希常量非 TypeId）。 | **O** | decomp-0x140aac640.txt + TypeId 扫描 |
| `TransientMesh::new_with_registry` | 0x140aa8200 | 32 | **业务** 构造（16B 完整函数）：`mov rax,[rdx]`（读 &Arc→ArcInner）→ `lock inc [rax]`（Arc clone，`jle→ud2` 溢出检查）→ `mov rdx,rcx; ret`——RAX:RDX 返回 {Arc, mesh_handle}。**实签名 `(mesh_handle: u64, registry: &Arc<...>)`**（调用方 0x141210110 以 RCX=handle、RDX=&Arc 传入）。0x140aa8210 起 0xd8 栈帧是**相邻独立函数**，非本函数主体。 | **O** | 机器码 0x140aa8200 |

## 内联/无独立符号函数

- `TransientMesh::drop`（impl$2::drop @0x140899f90，424B）— **O**：见生命周期算法（Drop 内联 deferred_delete_mesh，非独立符号表中列出但地址已证实）。注：PDB 中 drop 未列于业务表，本章按实现方式补入。

## 派生胶水

- 1 项：drop 等（derive 直接复用，不计业务）。

## 算法级结论（round 算法深挖，2026-10-15）

### 1. 布局（O，round3 裁决修正）

- `AssetMeshLibrary.transient_meshes`（字段 @+0x18，访问器 0x140002450 `lea rax,[rcx+0x18]; ret`）= **`Arc<TransientMeshRegistry>`**（非独立 Res 资源）。`TransientMeshRegistry` = **`std::sync::Mutex<Vec<u64>>`**，**32B = 0x20**（旧文"24B"错）：Mutex state 字节@+0、poison@+1、pad、Vec{cap@+8, ptr@+0x10, len@+0x18}（cap-first 游戏 Vec，8B 元素 = mesh handle id）。ArcInner 0x30：strong@0/weak@8/registry data@+0x10（state@+0x10、poison@+0x11、cap@+0x18、ptr@+0x20、len@+0x28）。
- `TransientMesh` = `{ registry: Arc<TransientMeshRegistry>@+0, mesh_handle: u64@+8 }`（16B）。
- **锁实现 = std::sync::Mutex（futex 实现，非 parking_lot）**：PDB 权威符号 0x1428d8750=`std::sys::sync::mutex::futex::Mutex::lock_contended`、0x1428d8810=`…::Mutex::wake`；锁内层仍为 1 字节 state（fast CAS 0→1）+ poison。
- **`new_with_registry` 实签名**（0x140aa8200，16B）：`new_with_registry(mesh_handle: u64, registry: &Arc<TransientMeshRegistry>) -> TransientMesh`，入口即 Arc clone（`mov rax,[rdx]; lock inc [rax]` 溢出→ud2）+ `mov rdx,rcx; ret`（RAX:RDX 返回 {Arc,handle}）；reconstruction `(registry: Arc, mesh_handle)` 参数序与按值均不符——登记，不改源码。
- 证据（O）：drop 0x140899f90 decomp `[rcx]`=ArcInner、Mutex state@+0x10/poison@+0x11、Vec@+0x18..+0x28；drain 0x1412e9e70 asm 锁 state@0、len@+0x18/ptr@+0x10/cap@+8；PDB FunctionSystem 符号证实 `delete_dropped_transient_meshes` 参数面仅 `(ResMut<AssetMeshLibrary>, ResMut<AssetVAOLibrary>)`（无 Res<TransientMeshRegistry>）；`construct_wooden_gate_doors` 0x141210110 用 `add → transient_meshes() → new_with_registry(handle, &Arc)`。

### 2. 生命周期图（O）

```
TransientMesh::new_with_registry(0x140aa8200)（16B：Arc clone + RAX:RDX 返回）
  └─ Arc::clone（`mov rax,[rdx]; lock inc [rax]` + 溢出 ud2）

组件被移除/实体 despawn → TransientMesh::drop（0x140899f90，内联 deferred_delete_mesh 逻辑、独立 panic 点）
  └─ std::sync::Mutex 锁注册表 Mutex（fast CAS @data+0；慢路径 0x1428d8750；毒化→panic
     "called `Result::unwrap()` on an `Err` value" @0x142a98440，源 transient_mesh.rs）
      └─ mesh_handle push 进 pending_delete Vec（len==cap → grow 0x140562870，8B）
  └─ unlock

每帧 backend::delete_dropped_transient_meshes（参数面=ResMut<AssetMeshLibrary>+ResMut<AssetVAOLibrary>，持 Arc clone）
  └─ drain_meshes_to_delete<closure_env$0>（0x1412e9e70）
      └─ 锁注册表 Mutex（毒化→同款 panic @0x142c2ef40 + transient_mesh.rs @0x142c2f398）
          └─ 在锁内遍历 Vec（ptr@+0x10，len@+0x18，元素 8B）
              └─ 对每句柄调闭包（捕获两个 ResMut：AssetMeshLibrary + AssetVAOLibrary，
                 tick@+0x1c 回写）：AssetMeshLibrary::remove(h)（0x14092d750）
                 + AssetVAOLibrary::remove(h)（0x1409247a0）→ 释放 GPU 网格 + VAO
          └─ len 清 0（0x18..0x1f 写 0）——**容量保留**（非 mem::take：ptr/cap 不动）
      └─ 释放锁
```

要点：
- **整链单锁**：push 与 drain 都在注册表 Mutex 下完成；drain 持锁期间执行 GPU 删除（延迟释放设计，避免每组件 drop 直接碰 GPU）。
- **drain 非 mem::take**：机器码 = 锁内遍历全部句柄后仅把 len 清 0（0x18..0x1f），ptr/cap 保留 → 等价 `q.clear()` 语义（容量保留）；reconstruction 的 `std::mem::take`（整体换空 Vec）与证据不符——登记差异，不改源码。
- **毒化 = panic**（`.lock().unwrap()`，三处独立 panic 点 0x142a98440/0x142adcf70/0x142c2ef40），reconstruction 的 `if let Ok` 语义不符。
- **注册表非独立 Res**：reconstruction 把 `TransientMeshRegistry` 建模为 `Res<TransientMeshRegistry>` 资源，而系统参数面仅两个 ResMut（PDB FunctionSystem 符号证实）——登记差异，不改源码。
- 闭包捕获的两个"ResMut"（tick@+0x1c 回写）→ backend 系统参数面为 `ResMut<AssetMeshLibrary> + ResMut<AssetVAOLibrary>`（PDB 符号证实，非仅推断）。

### 3. 证据修正

- decomp-0x140aa8200.txt 与 decomp-0x140002370.txt 为 Ghidra NotFound 错误文件（小函数未建），结论来自机器码手解；证据列已更新。
- `new_with_registry` 的"Arc clone + 溢出检查"证据：0x140aa8200 首 16B `48 8b 02 f0 48 ff 00 7e 04 48 89 ca c3 0f 0b`（mov rax,[rdx]; lock inc [rax]; jle +4; mov rdx,rcx; ret; ud2）——函数到此为止（0x140aa8200..0x140aa820c），其后 `cc`（int3）与 0x140aa8210 的 0xd8 栈帧函数为相邻独立函数。
- round3 布局裁决（groupE #5）：注册表 = `AssetMeshLibrary.transient_meshes`（Arc 字段），非独立 Res；Mutex = **std::sync::Mutex**（futex 实现，PDB `std::sys::sync::mutex::futex::Mutex::lock_contended`@0x1428d8750 / `wake`@0x1428d8810）；注册表总尺寸 **32B（0x20）**；元素 = u64 mesh handle id。详见 `components-geometry/CLOSURE-ROUND3.md`。
