# render/uniform.rs

> 文件作用：**uniform 写值工具**——`write_uniform_to_buffer<C,T>`（把 T 的值写入 `&mut [u8]` 缓冲，写宽按 T 决定）+
> `bind_lazy_uniform`（惰性 uniform 绑定闭包）。被 `UniformBinder::write_uniform`（pass_builder/uniform_binder.rs）内联。

## 内联/无独立符号函数

- `country_core::render::uniform::impl$23::write_uniform_to_buffer`（行 43）— **O**：写宽表（同 uniform_binder 章）：u32/i32/f32/bool→4B（bool 以 u32 写）、[u32;2]/Vec2→8B、Vec3→12B、Vec4→16B、Mat4→64B；值经 memcpy 落入 `&mut [u8]` 目标；越界 → `slice_start_index_len_fail`（由调用方守卫）
- `country_core::render::uniform::impl$24::write_uniform_to_buffer`（行 55）— 同上（另一泛型实例/另一 C 约束）
- `country_core::render::uniform::bind_lazy_uniform`（行 62）— **I**：把闭包封装为可命中的惰性 uniform 写（命中反射表才求值，供 `write_optional_uniform` 使用）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- **写值链**：`write_uniform<C,T>`（UniformBinder 章四道守卫）→ 命中反射表取偏移 → 本文件的 `write_uniform_to_buffer<C,T>` 按 `size_of::<T>` 写宽落值（bool 走 `movzx eax, r9b` 整 u32）→ 返回是否写入。
- **惰性链**：`write_optional_uniform` 的闭包经 `bind_lazy_uniform` 包装——反射表未命中则不调用闭包（纹理 extent 等惰性值只有命中才求，O）。
- 这些函数是内联辅助（SOURCE_MAP 无 VA），真实写宽/越界行为在 `UniformBinder::write_uniform` 单态化实例（0x14096b570..0x14096bc10）中核实，本文件只承载 Rust 源码中的模板体。

## 派生胶水（0 项）
