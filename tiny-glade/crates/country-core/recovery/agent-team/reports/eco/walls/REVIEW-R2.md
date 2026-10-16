# REVIEW-R2 — cached_decorator_transforms（reviewer-eco，2026-10-16）

复核对象：restorer-decorator R2（RESTORE-DECORATOR.md §九：cached_decorator_transforms.rs 新建）。
实测：cargo check 过；cargo test --lib **1671/1**（唯一败 geometry/sphere 他组，collision/world.rs
在途已消）；cached_decorator_transforms 4/4 过。未改 src。

## 逐项核对（协调者 4 点）

### 1. 缓存条目 = 10 f32 紧凑变换 — ✅ 纠偏确认

`CachedDecoratorTransform { rotation: Quat@[0..4], translation: Vec3@[4..7], scale:
Vec3@[7..10] }`——insert 0x140a1ada0 读面 O；**章节"3×3 矩阵 9 分量"为近似，rotation 实为
4 分量 Quat（10 f32），movmskps==0xf 全 4 分量证实**。`from_components/to_components` 分量
往返（测试过）。✓ 纠偏成立。

### 2. insert 三批有限性校验 — ✅

- translation → panic `"assertion failed: transform.translation.is_finite()"`
  （0x142ac8e00, line 0x33）；
- rotation → panic `"…rotation.is_finite()"`（0x142ac8e98, line 0x30，movmskps==0xf）；
- scale → panic `"…scale.is_finite()"`（0x142ac8ee0, line 0x2d）；
- 任一 NaN/Inf → panic → `FUN_14152aee0` 落缓存。restorer insert 按 translation→rotation→scale
  序断言 ✓；测试覆盖三批 NaN/Inf panic ✓。

### 3. 双缓冲 + 转发器 + get — ✅

- `buffers: [CachedDecoratorTransformStorage; 2]` + `current_write`（写入册），读取册 =
  current_write ^ 1（I：章节仅述互换，读/写册归属推断登记）✓。
- `swap_buffers_and_clear`（0x140a1aeb0）：toggle + 清新写入册 ✓。
- `transform`（0x140a1aa20，未中 panic）/`position`/`try_transform`（0x140a1ac80）/
  `try_position`（0x140a1ab80）转发器读读取册 ✓；`get`（0x140a1af40，HashMap 探针）✓。

### 4. 纯数据缓存 + 留存 U — ✅

纯数据缓存，非实体存储（与卡5 实体机制一致：实体由消费系统读后生成）✓。
留存 U 诚实：双缓冲读/写册归属（I）、swap_buffers 0xff+0x11 memmove 精确 drop 语义、
墙高/几何变化后装饰重定位消费（宿主接线）。✓

## 结论与 DISPATCH 建议

- **R2 = review_passed（限定），闭合**：缓存条目 10-f32 紧凑变换（Quat 纠偏）、三批有限性
  panic 文案、双缓冲 swap/转发器/get、纯数据边界——全部核过。
- 登记项（不阻塞）：双缓冲归属 I、swap memmove 精确语义、重定位宿主接线。
- 建议 DISPATCH：R2=review_passed（限定）；卡5/卡A1/卡F 维持 review_passed。
