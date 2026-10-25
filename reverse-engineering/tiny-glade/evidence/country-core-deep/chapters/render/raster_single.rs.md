# render/raster_single.rs

> 文件作用：**单物体光栅化包装**——`raster_single_object<1, ColorAttachment[1], closure>`（0x1409ce780/0x1409d00a0/0x1409cdec0 三个泛型实例）：
> 把"单个光栅化操作"包成 RasterPass 并提交到渲染图。消费者 = final_post / resolve_to_swapchain / mei_pass。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…raster_single_object<1,ColorAttachment[1],final_post::closure_env$1>` | 0x1409ce780 | 17 | 渲染/GPU | O：final_post 单物体光栅化（`Final blit` 等操作序列的每物体一次光栅） | render-top/decomp/decomp-0x1409ce780.txt（原 render-loop\decomp（final_post 族）） |
| `…raster_single_object<1,ColorAttachment[1],resolve_to_swapchain::closure_env$0>` | 0x1409d00a0 | 17 | 渲染/GPU | O（round3 round2 复查修正：原无行为证据，补 MCP 反编译）：resolve_to_swapchain 变体；渲染图单例光栅化（add_node 类型哈希/建边 kind 0x1d/closure env 参数/失败 panic 路径） | render-top/decomp/decomp-0x1409d00a0.txt（原 render-loop\decomp；round3 oa2-out/w2-decomp-1409d00a0.txt） |
| `…raster_single_object<1,ColorAttachment[1],mei_pass::closure_env$0>` | 0x1409cdec0 | 17 | 渲染/GPU | O：mei_pass 变体（ui_2d_pass 域单物体光栅化） | render-top/decomp/decomp-0x1409cdec0.txt（原 render-loop\decomp） |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- **处理链**（O，round3 对 resolve_to_swapchain 实例的 MCP 反编译）：RasterPass 构造（1 个颜色附件 tag0/tag1）+ 闭包（closure_env：draw 种类 + 绑定序列）→ `add_node`（类型哈希，round3 记录）→ 建边（kind 0x1d）→ 闭包录制期内执行 draw 终结操作 → 失败 → panic 路径。
- **共性**：三个实例共用 `raster_single_object<const N: usize, A, C>` 泛型体（N=1、A=[ColorAttachment;1]、C=操作闭包）；closure 内容是各自 pass 的操作序列（final_post 的 `Final blit` 12 次 write_uniform + optional_uniform；resolve_to_swapchain 的 msaa/非 msaa 分支；mei_pass 的文本/形状绘制）。
- **数据流**：`final_post`（d_gap §4 完整接线）→ `raster_single_object::<1>` 把单物体操作包成 pass → `resolve_to_swapchain` 用 `sample_count!=1` 选 msaa 着色器；`mei_pass` 的临时目标 tag2（d_main §1.3）。
- U：add_node 类型哈希值（round3 记录中）；closure 内 draw 种类明细。

## 派生胶水（0 项）
