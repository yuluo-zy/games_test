# uniform_binder.rs 业务契约

标注：**O**=反编译直接支持；**I**=调用链/行号组合；**U**=未恢复。

## 1. `UniformBinder`（O，行 21/37/53）

- `new`（0x14096b420，行 21）：创建 uniform 数据缓冲（ptr@+0x00、len@+0x08）+ 绑定表（hashbrown@+0x30/+0x38）。返回约 64B 级结构（尺寸 U）。
- `write_uniform<C,T>`（0x14096b570…0x14096bc10，行 37）：守卫 `(*self+0x0 != 0) && (self[9] != 0)`；按 uniform 名 hash（in_R8）查绑定表 → u32 offset；`offset <= data_len` 时把 `T`（f32/Vec2/Vec3/Vec4/Mat4/[u32;2]/i32/u32/bool）逐字节写 `data_buf[offset..]`；越界 `slice_start_index_len_fail`。
- `write_optional_uniform<C,T>`（0x14096bd20…0x14096bf50，行 53）：同 write_uniform，但值由闭包 env 派生（如 `AssetTextureLibrary::get_by_name` → `TextureDesc::extent_2d` 写 [u32;2]）。

闭包 C 是调用方（ComputePass/DrawBuilder 的 `uniform`/`bind_texture_to_*` 闭包）负责提供 name hash 与值；`write_uniform` 只负责查偏移并写数据。
