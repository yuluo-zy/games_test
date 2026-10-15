# resources/world_raster.rs

> 文件作用：**世界栅格核心容器 WorldRaster<512>（composite/scratch 双缓冲 u8 栅格 + tabloid DirtyTiles 脏瓦片协议 + 订阅发布）与 world_raster_defs 子模块（命名栅格资源 + AllWorldRasterRead/Write 全栅格 SystemParam）。水/路/鹅卵石/花园六种栅格的公共基础设施；栅格化走 zeno 依赖。**

函数总计 54：业务 22 / 内联无独立符号 19 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::world_raster::impl$2::clear_buffer<512>` | 0x140a15a10 | 0 | 按选择字节清空 composite 或 scratch 单缓冲（0x200 段或全 0x40000B） | O | terrain-water\decomp\decomp-0x140a15a10.txt
| `country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >` | 0x140a16960 | 0 | 点列→zeno Command 表（28B/条，首点 move-to 其余 line-to，grid=(world/130+0.5)*512）并按模式字节+bit0 选三 blend 闭包（min/max/饱和加×笔刷）写入目标层（composite/scratch 按模式字节选） | O | terrain-water\decomp\decomp-0x140a16960.txt
| `country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >` | 0x140a16960 | 0 | rasterize_stroke 另一迭代器实例（impl$3::iter_points_xz 闭包，与 impl$2 实例同地址同语义） | O | terrain-water\decomp\decomp-0x140a16960.txt
| `country_core::resources::world_raster::DirtyTilePublisher::unique` | 0x1409393b0 | 64 | 构造 unique 变体发布器：判别 0 + 两个空 Vec(cap=8)（38B） | O | terrain-water/DIGEST.md
| `country_core::resources::world_raster::WorldRaster::new<512>` | 0x140a15530 | 136 | 分配 composite/scratch 各 0x40000B（512×512 u8）+ DirtyTiles 位图 0x4000B（4096 u32 瓦片状态，初值 0x07954dbb）+ 拷贝 56B DirtyTiles 头模板（含 parking_lot Mutex + 订阅 Vec）+ 清零 staging 0x204B | O | terrain-water\decomp\decomp-0x140a15530.txt
| `country_core::resources::world_raster::WorldRaster::subscribe<512>` | 0x140a158e0 | 181 | 创建 0x28B Arc 订阅记录压入 +0x78 订阅 Vec；非 primary 发布器 panic 'Can't subscribe to a secondary dirty tile publisher' | O | terrain-water\decomp\decomp-0x140a158e0.txt
| `country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x140a197c0 | 261 | 存档反序列化：按 raster 名读取栅格（错误串 'Expected raster '/'End of stream while expecting raster '） | O | terrain-water\decomp\decomp-0x140a197c0.txt
| `country_core::resources::world_raster::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x14003c3b0 | 261 | 存档序列化（CompactFormatter）——serde 胶水 | O | terrain-water/DIGEST.md
| `country_core::resources::world_raster::impl$2::clear_all<512>` | 0x140a15da0 | 287 | 双缓冲全清 + 全部 4096 瓦片状态重置为 0x07954dbb（SIMD 四字 + 标量尾） | O | terrain-water\decomp\decomp-0x140a15da0.txt
| `country_core::resources::world_raster::impl$2::copy_composite_to_scratch<512>` | 0x140a161a0 | 309 | composite→scratch 纯 memcpy + len 断言（'Length mismatch; expected '） | O | terrain-water\decomp\decomp-0x140a161a0.txt
| `country_core::resources::world_raster::impl$2::commit_scratch_to_composite<512>` | 0x140a16580 | 318 | scratch→composite 纯 memcpy + len 断言 | O | terrain-water\decomp\decomp-0x140a16580.txt
| `country_core::resources::world_raster::impl$2::copy_to_staging<512>` | 0x140a172d0 | 409 | +0x299 脏→清零 out 区；+0x298=选择字节；DirtyTiles::retain 按脏瓦片拷入 out(SSBO 映射)；+0x29a=1 发布 | O | terrain-water\decomp\decomp-0x140a172d0.txt
| `country_core::resources::world_raster::build_tile_hasher` | 0x1409393e0 | 562 | 写 4×u64 种子常量（ahash 风格 4-key RandomState）到 +0/+8/+0x10/+0x18——瓦片哈希器构造 | O | terrain-water\decomp\decomp-0x1409393e0.txt
| `country_core::resources::world_raster_defs::PathRaster::new` | 0x14083eaf0 | 665 | PathRaster 资源构造（委托 WorldRaster::new） | O | terrain-water\decomp\decomp-0x14083eaf0.txt
| `country_core::resources::world_raster_defs::_::impl$0::apply` | 0x14003bb80 | 678 | SystemParam apply（ICF 折叠=裸 ret，无状态） | O | terrain-water\decomp\decomp-0x14003bb80.txt
| `country_core::resources::world_raster_defs::_::impl$0::init_state` | 0x14083fe20 | 678 | SystemParam init_state：注册 6 个栅格资源（get_resource 调用序列） | O | terrain-water\decomp\decomp-0x14083fe20.txt
| `country_core::resources::world_raster_defs::_::impl$0::new_archetype` | 0x14003bb80 | 678 | SystemParam new_archetype（ICF=裸 ret） | O | terrain-water\decomp\decomp-0x14003bb80.txt
| `country_core::resources::world_raster_defs::_::impl$0::queue` | 0x14003bb80 | 678 | SystemParam queue（ICF=裸 ret） | O | terrain-water\decomp\decomp-0x14003bb80.txt
| `country_core::resources::world_raster_defs::AllWorldRasterWrite::ordered_names` | 0x14083eb10 | 703 | 按 6 槽名序（PathRaster/Cobblestone1-3/WaterRaster/GardenRaster）输出类型名清单 | O | terrain-water\decomp\decomp-0x14083eb10.txt
| `country_core::resources::world_raster_defs::AllWorldRasterWrite::write` | 0x14083ebe0 | 712 | 每槽：borrow 标志→清 composite；否则 len 断言+memcpy 入 composite；memset(staging@+0x98,0xff,0x200) 全脏 + +0x299=1 | O | terrain-water\decomp\decomp-0x14083ebe0.txt
| `country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed` | 0x14083f410 | 726 | 按槽位比对类型名（'PathRaster' 等 0xa 字符）做写冲突检测；缺失资源走 Bevy panic 'Resource does not exist…' | O | terrain-water\decomp\decomp-0x14083f410.txt
| `country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging` | 0x14083fa60 | 753 | 每槽按 type-id（0x142c2e210 等 6 个）查 StagingSsbo（memcmp 全名）→ vtable 取映射指针 → copy_to_staging(selector=0) 发布 composite | O | terrain-water\decomp\decomp-0x14083fa60.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::world_raster::DirtyTilePublisher::shared`（行 68）— 构造 shared 变体发布器（行 68；共享发布，subscribe 仅限 primary）
- `country_core::resources::world_raster::DirtyTilePublisher::secondary`（行 72）— 构造 secondary 变体发布器（行 72；不可订阅，见 subscribe panic）
- `country_core::resources::world_raster::WorldRaster::as_float_grid`（行 158）— 将栅格视为 FloatGrid 视图（71 处内联；供 bilinear_sample_at_world_xz 采样）
- `country_core::resources::world_raster::WorldRaster::ws_to_px`（行 173）— 世界坐标→栅格像素（(w/130+0.5)*512；8 处内联）
- `country_core::resources::world_raster::WorldRaster::last_staged`（行 192）— 返回最近 staged 的内容视图（14 处内联）
- `country_core::resources::world_raster::WorldRaster::replace_composite`（行 199）— 整体替换 composite 缓冲（7 处内联；save 加载路径）
- `country_core::resources::world_raster::WorldRaster::clear_composite`（行 214）— 清空 composite（7 处内联）
- `country_core::resources::world_raster::WorldRaster::prepare_tile`（行 225）— 准备/分配瓦片状态（4 处内联）
- `country_core::resources::world_raster::impl$2::extend_dirty`（行 405）— 扩展脏瓦片集合（rasterize_world_space_stroke 辅助）
- `country_core::resources::world_raster::impl$2::on_uploaded_to_texture`（行 547）— 上传到纹理后回调（事件响应）
- `country_core::resources::world_raster::impl$2::on_texture_processed`（行 556）— 纹理处理完成后回调（事件响应）
- `country_core::resources::world_raster::impl$4::load_index_unchecked`（行 583）— 按索引直接加载栅格单元（无边界检查）
- `country_core::resources::world_raster_defs::Cobblestone1Raster::new`（行 665）— 鹅卵石 1 光栅构造（同 PathRaster::new 模板）
- `country_core::resources::world_raster_defs::Cobblestone2Raster::new`（行 665）— 鹅卵石 2 光栅构造
- `country_core::resources::world_raster_defs::Cobblestone3Raster::new`（行 665）— 鹅卵石 3 光栅构造
- `country_core::resources::world_raster_defs::GardenRaster::new`（行 665）— 花园光栅构造
- `country_core::resources::world_raster_defs::_::impl$0::get_param`（行 678）— SystemParam get_param（AllWorldRasterRead 实例 0x1401507ce0/0x1401508130）
- `country_core::resources::world_raster_defs::_::impl$0::validate_param`（行 678）— SystemParam validate_param（谓词校验）
- `country_core::resources::world_raster_defs::AllWorldRasterRead::read`（行 686）— 读全部 6 个栅格的当前缓冲视图（IO/解析）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`deref`、`deref_mut`、`deserialize`、`fmt`、`visit_enum`、`visit_map`、`visit_str` 等 13 项

## 实现方式（算法级）

### 处理链（O，本轮修正一处布局）

```
AllWorldRasterWrite::write(up_to)  每槽(0..up_to)：
  槽 Option=None → memset(composite,0,len)（资源缺失→清空）
  否则 len 断言("Length mismatch; expected ", 行43) → memcpy 槽数据→composite
  memset(+0x98, 0xff, 0x200)   ← 4096-bit 脏瓦片位图全置1（非"staging 缓冲"；每 8×8px 瓦片 1 bit）
  +0x299 = 1                    ← staging 脏标志
→ upload_to_staging（0x83fa60）：6 槽按 type-id 查 *StagingSsbo → copy_to_staging(selector=0)
→ copy_to_staging（0xa172d0）：+0x299 脏→memset(out,0,len)；+0x298=selector；
    DirtyTiles::retain(bitmap@+0x98, frame) 按脏瓦片拷入 SSBO 映射区 → tabloid publish → +0x29a=1
```

### 逐函数算法

| 函数 | 算法 |
|---|---|
| WorldRaster::new<512>（0xa15530） | alloc composite+scratch 各 0x40000B + 瓦片状态 Vec(4096×u32, 初值 0x07954dbb) + 拷贝 56B DirtyTiles 头模板 + memset(+0x98,0,0x200) 清位图 |
| clear_buffer（0xa15a10）/ clear_all（0xa15da0） | 按选择字节清单缓冲（0x200 段或全 0x40000）；clear_all 双缓冲全清 + 4096 瓦片状态重置 0x07954dbb（SIMD+标量尾） |
| copy_composite_to_scratch / commit_scratch_to_composite | 纯 memcpy + len 断言 |
| rasterize_stroke（0xa16960） | 点数<3 早退；点列→zeno Command 表（28B/条，首点 move-to 其余 line-to，grid=(w/130+0.5)*512）；blend 闭包按 (arg6,bit0) 选 min/max/饱和加×笔刷(16.16)；(0,1)→panic；目标层按模式字节选 composite/scratch |
| copy_to_staging（0xa172d0） | 见处理链；+0x50..0x5c 4×u32 脏瓦片元数据进 retain 帧 |
| build_tile_hasher（0x9393e0） | 写 4×u64 ahash 种子（0xbe734ee571db45b0 / 0xfca4335f3c9b800a / 0x36de4a1709f2df62 / 0xee5497aefccb8a97）到 +0/+8/+0x10/+0x18 |
| DirtyTilePublisher::unique | 38B：判别 0 + 双空 Vec(cap=8) |
| subscribe（0xa158e0） | primary 校验（secondary→panic "Can't subscribe…"）；0x28B Arc 记录 {Arc 计数@0, lock@0x10, Arc<闭包>@0x20} 压入 +0x78 Vec；tabloid publish 换装闭包消费 |
| write（0x83ebe0） | 见处理链 |
| would_write_succeed（0x83f410） | 按槽比对类型名做写冲突检测；缺失资源→Bevy panic |
| upload_to_staging（0x83fa60） | 6 槽 type-id 查 SSBO + memcmp 全名 → vtable 映射指针 → copy_to_staging(selector=0) |
| PathRaster::new（0x83eaf0） | 委托 WorldRaster::new |

### panic 分支

- "Length mismatch; expected "（0x142ac8648/0x142a93cf0，行 43 world_raster_defs.rs，unwrap Err 包装）
- "Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead."（0x142ac85f8）
- "Expected raster " / "End of stream while expecting raster "（deserialize\<Value>，0x142a93d80/8）
- 尾部 panic/abort 胶水：`lea rcx,[world_raster_defs.rs fat]; call 0x1428d9310; int3`（0x14083f2d6）
