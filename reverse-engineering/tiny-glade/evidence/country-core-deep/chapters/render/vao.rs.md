# render/vao.rs

> 文件作用：**顶点数组对象（Vao/VaoBinding）+ `build_vao`**——从属性表构建 GPU 顶点缓冲 + 绑定列表。
> 消费者：`AssetVAOLibrary::add`/`rebuild_vao`（A3_mesh）、`DrawBuilder`/`RasterPass`（WP1）。
> 证据：`reconstruction/country-core-render/wp2/contracts/vao.md`（O）。

函数总计 4：业务 1 / 内联无独立符号 2 / 派生胶水 1

## 布局表（O，wp2 vao 合同）

### `Vao` — 64 B（8 qword）

| 偏移 | 字段 | 状态 |
|---|---|---|
| 0x00 | buffer_lo / 0x08 buffer_hi（GPU 顶点缓冲句柄） | O |
| 0x10 | `bindings: Vec<VaoBinding>`（ptr/len/cap） | O |
| 0x28 / 0x30 / 0x38 | f6 / f7 / f8 | U（typed gap；疑顶点数/步长/VAO 属性，未观测） |

- `VaoBinding` 字段布局与 WP4 mesh 联合持有（U）。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `build_vao` | 0x140909690 | 37 | 猜测：构建 VAO；证实：GPU-only 缓冲 + 属性→绑定 | **O**（463 B）：`fn build_vao(attributes: HashMap<String, VertexAttributeValues>, device) -> Vao`；`BufferDesc::new_gpu_only()` + `GraphicsDevice::create_buffer<Option<&[u8]>>()`（非 Ok → unwrap_failed，tag 0xb）；`Vec<VaoBinding>` = `attributes.iter().map(build_vao::closure$0).collect()`；闭包用 `VertexAttributeValues::size_in_bytes`（O 依赖）+ 原始属性字节索引 + unwrap 把 `(&String, &VAV)` → VaoBinding；返回 8 qword | wp2 vao |

## 内联/无独立符号函数

- `Vao::new`（行 21）— **U/I**：构造器（无依赖，签名未恢复）
- `Vao::rebuild`（行 31）— **I**：drop 旧缓冲后重建（drop_in_place 依赖 O）

## 派生胶水（1 项：closure）

- `build_vao::closure$0`：内联于 build_vao（I，deps O 见上）。

## 纠错/差异

- `VaoBinding` 布局与 WP4 mesh 联合（勿单独钉死）；f6–f8 无消费证据，勿编造。
- 消费者 `AssetVAOLibrary::add`/`rebuild_vao` 直调本函数（A3_mesh O）；`rebuild_vao` 失败路径的 Err 透传 I：不可达（内部全 unwrap）。
- reconstruction `vao.rs`（107 B）已按合同实现。
