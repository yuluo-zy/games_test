# pass_resources.rs 业务契约

标注：**O**=反编译直接支持；**I**=调用链/行号组合；**U**=未恢复。

## 1. `PassResources::new_from`（O，行 24；0x140909310）

`PassResources::new_from(world: &World) -> PassResources`（56B = 7×u64 指针）。

按 7 个预计算 hash 从 ECS world 资源表取 7 个资源指针（`Components::get_resource_id` + world +0x1e0/+0x210/+0x218 资源表）；首个必选，其余缺失填 0，但链式依赖仍 `unwrap_failed`：

| 字段 | hash | 必选 |
|---|---|---|
| r0 | 0x8597ee0727b461b3 | 必选 |
| r1 | 0xf0b963d163fe760f | 可选（缺→0） |
| r2 | 0xa68561b396784eaa | 必选（依赖 r1 存在才查） |
| r3 | 0x8226cc786e679355 | 必选 |
| r4 | 0x874b1541877dacfd | 必选 |
| r5 | 0x47c94cb78283a72e | 必选 |
| r6 | 0xb65bf3141f93e938 | 必选 |

资源名称映射 **U**（I：应为帧级核心资源，如 frame_constants / frame_constants_storage_buffer / camera / Globals / sampler set，见 INDEX.md descriptor 约定 set1/b0、set1/b1、set0/0..12、set0/13）。`PassResources` 作为 `PassBuilder::render<PassResources, _>` 的 UserData 泛型参数被全工程引用。
