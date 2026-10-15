# asset_libraries/vao_library.rs

> 文件作用：**顶点数组对象（VAO）资产库**——`AssetVAOLibrary`（0x60 B）：`HashMap<Handle<Vao>, VaoEntry>`（
> `VaoEntry { mesh_handle, Vao, index_count }`）+ `HashMap<Handle<Mesh>, Handle<Vao>>`。`add` 按 mesh 建 GPU VAO
> （校验 mesh 存在 + 索引非空 + build_vao），`rebuild_vao` 供 `vao_update::rebuild_vaos`（0x140864bf0）消费 modified
> 列表重建成。证据：`evidence/country-core-audit/A3_mesh.md` §vao_library（O）。

函数总计 10：业务 4 / 内联无独立符号 3 / 派生胶水 3

## 布局表（O，A3_mesh，总 0x60）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `HashMap<Handle<Vao>, VaoEntry>`（桶 0x50；`VaoEntry { key@0, Vao@8（bindings@8、index_buffer@0x20）, usize@0x48 }`） | new 0x140924030 |
| +0x30 | `HashMap<Handle<Mesh>, Handle<Vao>>`（桶 16 B） | new 0x140924030 |

- `VaoEntry` 字段名为 I；`usize` 取值 O：`add`/`rebuild_vao` 写 `mesh.indices.len()`（`[mesh+0x10]`）= index_count。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AssetVAOLibrary::new` | 0x140924030 | 23 | 猜测：构造；证实：两张空表 | **O**（328 B）：两张空表，各取一次线程局部 KEYS | A3_mesh.md |
| `AssetVAOLibrary::add` | 0x140924180 | 34 | 猜测：注册 VAO；证实：**签名 `(self, device, mesh_lib, mesh_handle) -> Result<Handle<Vao>, anyhow::Error>`** | **O**（611 B）：1. `mesh_to_vao` 非空且命中 → Ok(已有 vao handle)；2. `HANDLE_COUNT++` 取 VAO handle（**先于校验**，失败也消耗 id）；3. `mesh_lib.get` 缺失 → `anyhow!("Mesh is not in the  mesh library")`（0x142aae830，**两个空格**）；4. `indices.len()==0` → `anyhow!("Mesh is invalid")`（0x142aae800）；5. `build_vao`（0x140909690）；结果 Err 透传（I：build_vao 内部全 unwrap，不可达）；6. `vaos.insert(vao_handle, (vao, len))`，`mesh_to_vao.insert(mesh, vao_handle)`。旧版 `add(mesh_handle: u64)` 只查重塞空 Vao 错 | A3_mesh.md |
| `AssetVAOLibrary::rebuild_vao` | 0x1409243f0 | 76 | 猜测：重建 VAO；证实：返回 `Option<anyhow::Error>` | **O**（937 B）：1. `mesh_lib.get(mesh).expect("Mesh is not in the mesh library")`（单空格，:84:14）；2. indices 空 → 返回 Ok（**静默**）；3. mesh_to_vao 未命中 → `anyhow!("The requested mesh doesn't exist")`（0x142aae898）；4. vaos 未命中 → `anyhow!("VAO handle is invalid")`（0x142aae8c0）；5. `build_vao`，drop 旧 bindings、drop 旧索引缓冲（rhapsody managed handle drop）后写入，`index_count = indices.len()`。旧版空函数错 | A3_mesh.md |
| `AssetVAOLibrary::remove` | 0x1409247a0 | 102 | 猜测：删除；证实：双表 remove_entry + drop Vao | **O**（263 B）：`mesh_to_vao.remove_entry(mesh)`；命中则 `vaos.remove_entry(vao_handle)` 并 drop `Vao`（Vec<VaoBinding> + 缓冲 handle）。旧版只删单表 | A3_mesh.md |

## 内联/无独立符号函数

- `AssetVAOLibrary::get`（行 30）— **I**：`vaos` 表查询（无独立符号）
- `AssetVAOLibrary::has_vao`（行 57）— **I**：mesh_to_vao 存在性谓词（`rebuild_vaos` 消费者使用，O 调用点）
- `AssetVAOLibrary::build_vao`（行 61）— **I**：薄封装 `render::vao::build_vao`（O 调用点：add/rebuild_vao 直调 0x140909690）

## 派生胶水（3 项：closure 等）

- add/rebuild 的闭包（无独立符号；I）。

## 跨领域线索 / 纠错（A3_mesh，O）

- 消费者 `vao_update::rebuild_vaos` 0x140864bf0：遍历 `mesh_lib.modified`，仅处理 `has_vao` 的 handle（否则 warn 跳过），失败的 handle 收集写回 modified（并 error 日志）。
- 全局 handle 计数器 0x143634cd0 与 mesh_library 共用（`lock xadd`）。
- 旧版 `rebuild_vao` 空函数、`remove` 只删单表、`add` 无设备无校验——全错。
- `render::vao::build_vao` 0x140909690 本体见 render/vao.rs.md（wp2 合同）。

## 算法要点（本批深挖 2026-10-15，decomp: asset-libraries/decomp/）

证据：tg_m1 反编译 `decomp-0x140924180`（add 611 B）、`decomp-0x1409243f0`（rebuild_vao 937 B）。

### add 流程（0x140924180，返回 Result<Handle<Vao>, anyhow::Error>）

1. `mesh_to_vao` 非空且命中 → `Ok(已有 handle)`（幂等）。
2. **`HANDLE_COUNT++`（先于任何校验，失败也消耗 id）**。
3. `mesh_lib.get(mesh)` 缺失 → `anyhow!("Mesh is not in the  mesh library")`（0x142aae830，**两个空格**，文案忠实保留）。
4. `mesh.indices.len()==0` → `anyhow!("Mesh is invalid")`（0x142aae800）。
5. `build_vao`（0x140909690）→ 结果 Err 透传（build_vao 内部全 unwrap，I：不可达）。
6. `vaos.insert(vao_handle, {vao, index_count=indices.len()})` + `mesh_to_vao.insert(mesh, vao_handle)`。

### rebuild_vao 流程（0x1409243f0，返回 Option<anyhow::Error>，rax==0 = Ok）

1. `mesh_lib.get(mesh).expect("Mesh is not in the mesh library")`（0x142aae840，len 0x1f=31，单空格；:84:14——**与 add 的 anyhow 文案是两个不同字符串**）。
2. `indices.len()==0`（`[mesh+0x10]==0`）→ 返回 Ok（静默跳过）。
3. mesh_to_vao 空/未命中 → `anyhow!("The requested mesh doesn't exist")`（0x142aae898）。
4. vaos 空/未命中 → `anyhow!("VAO handle is invalid")`（0x142aae8c0）。
5. `build_vao` 成功 → drop 旧 bindings、drop 旧索引缓冲（rhapsody managed handle drop）→ 写 `VaoEntry+0x48 = indices.len()` → Ok。

### 消费端重建循环（vao_update::rebuild_vaos 0x140864bf0，O，既有 decomp）

遍历 `mesh_lib.modified`（get_mut/remove 压入的 handle 列表）：仅处理 `has_vao(mesh)` 的（否则 warn 跳过）；`rebuild_vao` 返回 Err 的 handle 收集后**写回 modified**（下次帧再试）+ error 日志。→ modified 是"脏标记队列"，重建后由消费方清空。

### 关键常量/布局

| 项 | 值 | 含义 |
|---|---|---|
| 表 A | HashMap<Handle<Vao>, VaoEntry> @+0x00（桶 0x50） | VaoEntry{key@0, Vao@8(bindings@8, index_buffer@0x20), index_count@0x48} |
| 表 B | HashMap<Handle<Mesh>, Handle<Vao>> @+0x30（桶 16 B） | mesh→vao 映射 |
| build_vao | 0x140909690 | render::vao::build_vao（本体见 render/vao.rs.md） |
| 全局 handle | 0x143634cd0 | 与 mesh/shader/ssbo/placement 共用 |

### 模块间调用

- add/rebuild_vao → `mesh_lib.get`（FrozenMap）+ `render::vao::build_vao`；remove → 双表 remove_entry + Vao drop。
- 消费者：`vao_update::rebuild_vaos`（0x140864bf0）← mesh_library.modified；渲染 draw 面经 `get`（内联 vaos 查）。
