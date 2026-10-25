# render/pass_builder/uniform_binder.rs

> 文件作用：UniformBinder：uniform 块映射内存 + 按 64 位名字哈希查偏移写值（四道守卫静默返回；bool 写 u32；越界 slice_start_index_len_fail）。

函数总计 18：业务 15 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::uniform_binder::UniformBinder::new` | 0x14096b420 | 21 | 反射 +0xd8==0（无 uniform 块）→ None（niche 标签 0xa）；否则 DynamicConstants::allocate(size=reflection[+0x128]) → into_uniform_buffer，记录 set/绑定号到 +0x28 | O | render-top/decomp/decomp-0x14096b420.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b420）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >` | 0x14048f660 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14048f660.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>` | 0x14096ba30 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096ba30.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>` | 0x14096b940 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096b940.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>` | 0x14096b660 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096b660.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>` | 0x14096b570 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096b570.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<u32>,u32>` | 0x14048f570 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14048f570.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >` | 0x14096b760 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096b760.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>` | 0x14096b850 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096b850.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<glam::f32::sse2::mat4::Mat4>,glam::f32::sse2::mat4::Mat4>` | 0x14096bc10 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096bc10.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<glam::f32::sse2::vec4::Vec4>,glam::f32::sse2::vec4::Vec4>` | 0x14096bb20 | 37 | 同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回 | O | render-top/decomp/decomp-0x14096bb20.txt（原 B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TextureFromDisk>,array$<u32,2> >` | 0x14096bd20 | 53 | 同 write_uniform 守卫；值由闭包**惰性**求（命中后才调用）；返回 bool | O | render-top/decomp/decomp-0x14096bd20.txt（原 B_a4_compute §2（0x14096bd20 系）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >,array$<u32,2> >` | 0x14048f750 | 53 | 同 write_uniform 守卫；值由闭包**惰性**求（命中后才调用）；返回 bool | O | render-top/decomp/decomp-0x14048f750.txt（原 B_a4_compute §2（0x14096bd20 系）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >,array$<u32,2> >` | 0x14096be40 | 53 | 同 write_uniform 守卫；值由闭包**惰性**求（命中后才调用）；返回 bool | O | render-top/decomp/decomp-0x14096be40.txt（原 B_a4_compute §2（0x14096bd20 系）） |
| `country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$7<country_core::resources::texture_presets::TexturePreset>,array$<u32,2> >` | 0x14096bf50 | 53 | 同 write_uniform 守卫；值由闭包**惰性**求（命中后才调用）；返回 bool | O | render-top/decomp/decomp-0x14096bf50.txt（原 B_a4_compute §2（0x14096bd20 系）） |

## 内联/无独立符号函数

- `country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform`（行 37）— write_uniform 泛型（行 37，10 个实例）—— O（见实例）
- `country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform`（行 53）— write_optional_uniform 泛型（行 53，4 个实例）—— O（见实例）

## 派生胶水（1 项：closure 等）

1 项 glue = write_optional_uniform::closure$0（惰性值闭包）。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `UniformBinder::new`（0x14096b420）—— 映射分配
处理链：取着色器反射 → **反射 +0xd8==0（无 uniform 块）→ 返回 None（niche 标签 0xa）**；否则 `DynamicConstants::allocate(size = reflection[+0x128])` → `into_uniform_buffer`（+0x28 记录 set/绑定号）。ComputePass 与 DrawBuilder 各持一份（+0x110/0x120/0x136 区）。

### 2. `write_uniform<C,T>`（0x14096b570..0x14096bc10 系 10 实例）—— 哈希定位写值
处理链（四道守卫，**全部静默返回**，O）：
1. 反射有效（首字非 0）；
2. 字段表项数非 0（表 = hashbrown，键=绑定名哈希 u64，值=偏移）；
3. 名字哈希（64 位直接作 hashbrown 哈希）命中 → 取 u32 偏移；
4. `块长 < offset` → **`slice_start_index_len_fail` panic**（越界）；否则 memcpy 写值。
- 写宽表（O）：u32/i32/f32/bool 4B（bool 以 `movzx eax, r9b` 写整 u32）、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B。
- 键：由旧版 `BindingIndex(u32)` 改为 **u64 名字哈希**（`BindingNameHash::new(name)` = FNV-1a-64；`<name>_dims` 字段 = `fnv(name)+1`，O 0x140c87650）。
- `write_optional_uniform`（0x14096bd20 系 4 实例）：同守卫；**值由闭包惰性求**（命中后才调用，如纹理 extent 闭包）；返回 bool = 是否写入。

### 3. 调用链
`bind_texture` 维度 uniform（键 name+1，`[w,h]`）、`bind_common`（bool=`flag==0`）、`bind_ray_tracing` 缓冲字 uniform 均经此层；`ComputePass::new`/`DrawBuilder::new` 在无 uniform 块时跳过分配（None niche）。

### 4. panic 分支
唯一 panic = 偏移越界 `slice_start_index_len_fail`；守卫失败全部静默（绑定名不存在不报错——原版对可选 uniform 的语义）。

## 证据注记（tg_rn1）
- **守卫四道**（O，全部静默返回）：① reflection[+0xe8 结构]首字非 0 ② 字段表项数非 0 ③ 名字哈希（64 位直接作 hashbrown 哈希）命中取 u32 偏移 ④ `chunk_len < offset` → `slice_start_index_len_fail`。
- **写宽**（O）：u32/i32/f32/bool 4B（bool 以 `movzx eax, r9b` 写整 u32）、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B。
- `write_optional_uniform` 返回 bool = 写了（值由闭包惰性求，如纹理 extent）。
- 键由旧版 `BindingIndex(u32)` 改为 u64 名字哈希（O：查表 `cmp [..], r8` 64 位）。
