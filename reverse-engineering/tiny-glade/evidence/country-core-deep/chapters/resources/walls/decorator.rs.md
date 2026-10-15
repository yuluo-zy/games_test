# resources/walls/decorator.rs

> 文件作用：装饰附着/信息类型族：DecoratorInfo 72B 记录、DecoratorId(u32)/DecoratorIdGen、DecoratorDst 目的地联合（tag@+0x22）、DecoratorExtraInfo 附加信息联合、DecoratorType 判别族（0x38..0x3f 位运算谓词）、DecoratorRank、DecoratorRotationXZ、TransientDecoratorIdGen、装饰操作事件族。

函数总计 131：业务 56 / 内联无独立符号 13 / 派生胶水 62

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall_mut` | 0x1408332c0 | 0 | DecoratorInfo::dst_as_wall_mut（unreachable dst is Roof/Stair/Terrain） | O | wall\decomp\decomp-0x1408332c0.txt |
| `country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140a48880 | 21 | 序列化 | U |  |
| `country_core::resources::walls::decorator::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x140a480d0 | 21 | 序列化 | U |  |
| `country_core::resources::walls::decorator::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x140a47da0 | 21 | 序列化 | U |  |
| `country_core::resources::walls::decorator::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1402ddd20 | 21 | 序列化 | U |  |
| `country_core::resources::walls::decorator::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140a478f0 | 21 | 序列化 | U |  |
| `country_core::resources::walls::decorator::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1402de8c0 | 21 | 序列化 | U |  |
| `country_core::resources::walls::decorator::DecoratorType::max_rank` | 0x140833130 | 37 | DecoratorType::max_rank（(t+0xc8)<3 → 3 否则 1） | O | wall\decomp\decomp-0x140833130.txt |
| `country_core::resources::walls::decorator::DecoratorType::can_change_seed` | 0x140833150 | 50 | DecoratorType::can_change_seed（(t+0xc8)<8 && 0x27>>shifted&1 → {0x38,0x39,0x3a,0x3d}） | O | wall\decomp\decomp-0x140833150.txt |
| `country_core::resources::walls::decorator::DecoratorType::is_window` | 0x140833160 | 67 | DecoratorType::is_window（(t+0xc8)<3 → {0x38,0x39,0x3a}） | O | wall\decomp\decomp-0x140833160.txt |
| `country_core::resources::walls::decorator::DecoratorType::spawns_clutter` | 0x140833170 | 74 | DecoratorType::spawns_clutter（(t&0x3e)==0x38 → {0x38,0x39}） | O | wall\decomp\decomp-0x140833170.txt |
| `country_core::resources::walls::decorator::DecoratorType::can_place_on_roof` | 0x140833180 | 78 | DecoratorType::can_place_on_flat_roof（t!=0x3a） | O | wall\decomp\decomp-0x140833180.txt |
| `country_core::resources::walls::decorator::DecoratorType::can_place_on_stair_top` | 0x140833190 | 108 | DecoratorType::can_place_on_stair_top（(t+0xc4)<3 || (t&0x38)!=0x38 → 仅窗族 {0x38..0x3b} 不可放楼梯顶） | O（RNE 手解证实，候选U收尾闭合；Ghidra NotFound 原字节手解） | wall\decomp\decomp-0x140833190.txt（rne-disasm 手解替换 NotFound stub） |
| `country_core::resources::walls::decorator::DecoratorType::can_place_on_terrain` | 0x1408331b0 | 119 | DecoratorType::can_place_on_terrain（(t&0x3c)!=0x38） | O | wall\decomp\decomp-0x1408331b0.txt |
| `country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner` | 0x1408331c0 | 137 | DecoratorType::should_offer_snap_to_corner（t!=0x3f） | O | wall\decomp\decomp-0x1408331c0.txt |
| `country_core::resources::walls::decorator::impl$26::hash<std::hash::random::DefaultHasher>` | 0x140a489e0 | 155 | derive 胶水 | U |  |
| `country_core::resources::walls::decorator::DecoratorId::new` | 0x14067dd80 | 161 | FinalizedHoleStorage::inner（恒等返回） | O | audio-misc\decomp\decomp-0x14067dd80.txt, collision-eco\decomp\decomp-0x14067dd80.txt, terrain-water\decomp\decomp-0x14067dd80.txt |
| `country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events` | 0x14067dd80 | 161 | FinalizedHoleStorage::inner（恒等返回） | O | audio-misc\decomp\decomp-0x14067dd80.txt, collision-eco\decomp\decomp-0x14067dd80.txt, terrain-water\decomp\decomp-0x14067dd80.txt |
| `country_core::resources::walls::decorator::ClutterVersion::is_initial` | 0x140833230 | 220 | ClutterVersion::is_initial（cmp byte,0; sete） | O | wall\decomp\decomp-0x140833230.txt |
| `country_core::resources::walls::decorator::DecoratorLocator::new` | 0x140833240 | 237 | DecoratorLocator::new | I | wall\decomp\decomp-0x140833240.txt |
| `country_core::resources::walls::decorator::DecoratorRank::new` | 0x140833250 | 270 | DecoratorRank::new | I | wall\decomp\decomp-0x140833250.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair` | 0x140839490 | 368 | DecoratorExtraInfo::try_as_stair | O | wall\decomp\decomp-0x140839490.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_chimney` | 0x1408392d0 | 377 | 访问器/转换 | I | wall\decomp\decomp-0x1408392d0.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_chimney_mut` | 0x1408392d0 | 377 | 访问器/转换 | I | wall\decomp\decomp-0x1408392d0.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_flag` | 0x140839340 | 377 | DecoratorExtraInfo::as_flag | O | wall\decomp\decomp-0x140839340.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_flag_mut` | 0x140839340 | 377 | DecoratorExtraInfo::as_flag | O | wall\decomp\decomp-0x140839340.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_lantern` | 0x140839420 | 377 | 访问器/转换 | I | wall\decomp\decomp-0x140839420.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_lantern_mut` | 0x140839420 | 377 | 访问器/转换 | I | wall\decomp\decomp-0x140839420.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair` | 0x1408394b0 | 377 | DecoratorExtraInfo::as_stair | O | wall\decomp\decomp-0x1408394b0.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair_mut` | 0x1408394b0 | 377 | DecoratorExtraInfo::as_stair | O | wall\decomp\decomp-0x1408394b0.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_window` | 0x1408393b0 | 377 | 访问器/转换 | I | wall\decomp\decomp-0x1408393b0.txt |
| `country_core::resources::walls::decorator::DecoratorExtraInfo::as_window_mut` | 0x1408393b0 | 377 | 访问器/转换 | I | wall\decomp\decomp-0x1408393b0.txt |
| `country_core::resources::walls::decorator::impl$93::from` | 0x140839520 | 403 | DecoratorExtraInfo From 变体 4（写判别字节 +0x11=4 + 字段拷贝） | O |  |
| `country_core::resources::walls::decorator::impl$94::from` | 0x140839530 | 403 | DecoratorExtraInfo From 变体 5（写判别字节 +0x11=5） | O |  |
| `country_core::resources::walls::decorator::impl$95::from` | 0x140839540 | 403 | DecoratorExtraInfo From 变体 6（写判别字节 +0x11=6） | O |  |
| `country_core::resources::walls::decorator::impl$96::from` | 0x140839550 | 403 | DecoratorExtraInfo From 变体 7（写判别字节 +0x11=7） | O |  |
| `country_core::resources::walls::decorator::impl$97::from` | 0x140839560 | 403 | DecoratorExtraInfo From 变体 8 | O（round3 round2 复查修正：原“写判别字节 +0x11=8”与机器码不符）：机器码 16B 仅拷贝 payload {16B@+0, u32@+0x10}，无判别写入（对照 impl$96 0x140839550 确有 c6 41 11 07） | rne-read 0x140839560/0x140839550 |
| `country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall` | 0x140833260 | 446 | DecoratorInfo::dst_as_wall（tag<=1 → &self 否则 panic 行 40） | O | wall\decomp\decomp-0x140833260.txt |
| `country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall` | 0x1408333a0 | 465 | DecoratorInfo::try_dst_as_wall（[+0x22]<2 ? &self : null） | O | wall\decomp\decomp-0x1408333a0.txt |
| `country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof` | 0x1408333b0 | 482 | DecoratorInfo::dst_as_roof（tag 判别访问） | I | wall\decomp\decomp-0x1408333b0.txt |
| `country_core::resources::walls::decorator::DecoratorInfo::dst_as_terrain` | 0x140833410 | 501 | DecoratorInfo::dst_as_terrain | O | wall\decomp\decomp-0x140833410.txt |
| `country_core::resources::walls::decorator::DecoratorInfo::dst_as_stair` | 0x140833470 | 510 | DecoratorInfo::dst_as_stair（tag 判别访问） | I | wall\decomp\decomp-0x140833470.txt |
| `country_core::resources::walls::decorator::DecoratorDst::is_wall` | 0x1408334d0 | 533 | DecoratorDst::is_wall（tag<2 setb） | O | wall\decomp\decomp-0x1408334d0.txt |
| `country_core::resources::walls::decorator::DecoratorDst::as_roof` | 0x1408334e0 | 553 | DecoratorDst::as_roof | O | wall\decomp\decomp-0x1408334e0.txt |
| `country_core::resources::walls::decorator::DecoratorDst::as_wall` | 0x140833510 | 561 | 访问器/转换 | I | wall\decomp\decomp-0x140833510.txt |
| `country_core::resources::walls::decorator::DecoratorDst::as_wall_mut` | 0x140833540 | 569 | DecoratorDst::as_wall_mut（tag<=1 → &mut self，否则 unreachable） | O | wall\decomp\decomp-0x140833540.txt |
| `country_core::resources::walls::decorator::DecoratorOpEventKind::id` | 0x140833570 | 739 | DecoratorOpEventKind::id（返回 u32 id；v1 名 as_wall_mut 错位） | O | wall\decomp\decomp-0x140833570.txt |
| `country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send` | 0x1408336b0 | 771 | DeferredDecoratorOpEvents::send | O | wall\decomp\decomp-0x1408336b0.txt |
| `country_core::resources::walls::decorator::DeferredDecoratorOpEvents::unleash` | 0x1408337c0 | 775 | DeferredDecoratorOpEvents::unleash（冲刷延迟装饰操作） | O | wall\decomp\decomp-0x1408337c0.txt |
| `country_core::resources::walls::decorator::TransientDecoratorIdGen::new` | 0x1408339c0 | 826 | TransientDecoratorIdGen::new | I | wall\decomp\decomp-0x1408339c0.txt |
| `country_core::resources::walls::decorator::TransientDecoratorIdGen::generate` | 0x1408339e0 | 838 | TransientDecoratorIdGen::generate（两模式：直写 base 或偏移累加） | O | wall\decomp\decomp-0x1408339e0.txt |
| `country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector` | 0x140833a00 | 889 | DecoratorRotationXZ::from_xz_forward_vector | O | wall\decomp\decomp-0x140833a00.txt |
| `country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat` | 0x140833a20 | 895 | DecoratorRotationXZ::from_quat | O | wall\decomp\decomp-0x140833a20.txt |
| `country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat` | 0x140833ac0 | 900 | DecoratorRotationXZ::to_quat | O | wall\decomp\decomp-0x140833ac0.txt |
| `country_core::resources::walls::decorator::DecoratorRotationXZ::to_projective_xz_mat2` | 0x140833b20 | 909 | 访问器/转换 | I | wall\decomp\decomp-0x140833b20.txt |
| `country_core::resources::walls::decorator::DecoratorRotationXZ::offset` | 0x140833b80 | 915 | DecoratorRotationXZ::offset | I | wall\decomp\decomp-0x140833b80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::decorator::DecoratorId::get_inner`（行 166）— 查询/计算
- `country_core::resources::walls::decorator::ClutterVersion::latest`（行 205）— 待推断
- `country_core::resources::walls::decorator::DecoratorRank::is_atomic`（行 256）— 谓词/校验
- `country_core::resources::walls::decorator::DecoratorRank::is_compound`（行 261）— 谓词/校验
- `country_core::resources::walls::decorator::DecoratorRank::x`（行 266）— 待推断
- `country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_chimney`（行 368）— 待推断
- `country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_flag`（行 368）— 待推断
- `country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_lantern`（行 368）— 待推断
- `country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_window`（行 368）— 待推断
- `country_core::resources::walls::decorator::is_default`（行 602）— 谓词/校验
- `country_core::resources::walls::decorator::DecoratorIdGen::reserve`（行 806）— 生成/分配
- `country_core::resources::walls::decorator::DecoratorIdGen::generate`（行 811）— 构造/注册/生成
- `country_core::resources::walls::decorator::DecoratorRotationXZ::to_xz_forward_vector`（行 883）— 访问器/转换

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`cmp`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`hash`、`partial_cmp`、`serialize`、`visit_enum`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 62 项
## 实现方式（算法级）

### DecoratorType 谓词位运算族（O，全部手解，DIGEST §1.5 全表）

| 谓词 | 地址 | 机器码语义（t = DecoratorType u8 判别值） |
|---|---|---|
| can_change_seed | 0x140833150 | (t+0xc8)<8 && ((0x27>>shifted)&1) → t∈{0x38,0x39,0x3a,0x3d} |
| is_window | 0x140833160 | (t+0xc8)<3 → t∈{0x38,0x39,0x3a} |
| spawns_clutter | 0x140833170 | (t&0x3e)==0x38 → t∈{0x38,0x39} |
| can_place_on_flat_roof | 0x140833180 | t!=0x3a |
| can_place_on_stair_top | 0x140833190 | (t+0xc4)<3 \|\| (t&0x38)!=0x38 |
| can_place_on_terrain | 0x1408331b0 | (t&0x3c)!=0x38 → t∉{0x38..0x3b} |
| should_offer_snap_to_corner | 0x1408331c0 | t!=0x3f |
| max_rank | 0x140833130 | (t+0xc8)<3 → 3 否则 1 |
类型值域（O）：0x38 cottage 窗、0x39 gothic 窗、0x3a 第三窗型（不可上屋顶）、0x3b 窗类、0x3c/0x3d/0x3e 楼梯顶可放、0x3f 角吸附型。

### DecoratorInfo / DecoratorDst / DecoratorExtraInfo 判别访问（O）

- `dst_as_wall` 0x140833260：tag@+0x22<=1 → &self 否则 panic（行 40）。
- `try_dst_as_wall` 0x1408333a0：`cmp byte[+0x22],2; cmovb` → tag<2 返回 &self 否则 null。
- `dst_as_wall_mut` 0x1408332c0：tag 判别三 panic `unreachable!("dst is Roof/Stair/Terrain")`。
- `dst_as_roof` 0x1408334e0（773B）/`dst_as_terrain` 0x140833410 / `dst_as_stair` 0x140833470（I）。
- `DecoratorDst::as_wall_mut` 0x140833540：tag<=1 → &mut self 否则 unreachable（手解）。
- `DecoratorDst::is_wall` 0x1408334d0：tag<2 setb。
- `DecoratorOpEventKind::id` 0x140833570：返回 u32 id（**纠错**：PDB 名 as_wall_mut 错位，真实为 id()）。
- `DecoratorExtraInfo::as_flag` 0x140839340 / `as_stair` 0x1408394b0 / `try_as_stair` 0x140839490（判别字节 +0x11 访问）。
- From 变体 4..8：0x140839520..560 写判别字节 +0x11 = 4..8 + 字段拷贝（手解）。

### TransientDecoratorIdGen（O，0x1408339e0 / 0x1408339c0）

- 结构 {base:*mut u32@+0, +0x8:u64 偏移游标, +0x10:bool 直写模式}。
- `generate`：直写模式（flag=1）→ `*base += 1` 返回旧值；否则 `+0x8 += 1`，返回 `*base + 旧偏移`。
- `new` 0x1408339c0（I，构造器，游标/模式初值）。

### DecoratorRotationXZ（O）

- `from_xz_forward_vector` 0x140833a00：由 XZ 前向向量构造（角度编码）。
- `from_quat` 0x140833a20 / `to_quat` 0x140833ac0：四元数↔XZ 旋转互转。
- `offset` 0x140833b80（I）。

### DeferredDecoratorOpEvents::send 0x1408336b0（O，并发队列 push）

```
结构: 线程安全 Vec<[u8; 0x88]>（**事件 0x88B=136B**）+ 锁标记
LOCK: if (*self == 0) *self = 0x8; UNLOCK          // 空态标记 0x8
若标记已被占用 → FUN_1428cb5f0(self, data, 1000000000)   // park（等待队列，1e9ns 超时）
内存序: FUN_140c81c70 ×2
push: len@[3] == cap@[1] → FUN_140563170(扩容, file decorator.rs 0x142a92948)
  func_0x142923c00(ptr@[2] + len*0x11, data, 0x88)   // 0x88B memmove（步长 0x11×8）
  len += 1；FUN_140c81e10 ×2
释放: 若标记==0x8 → 清 0；遍历全局等待队列 plRam00000001436308f0 唤醒
```

### DeferredDecoratorOpEvents::unleash 0x1408337c0（O，冲刷）

```
出队: 读队列头数组（[0]→lVar2），从 +0x10/+0x18 取待发事件，逐条拷贝到输出 param_2（0x88B），len 归 0
（与 send 配对的 drain 侧，供帧末统一应用延迟装饰操作）
```

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| 1000000000 | 立即数 | send park 超时（ns） |
| 全局等待队列 | plRam00000001436308f0 | futex 唤醒（与 set_pillar_placements 共用） |
| file decorator.rs | 0x142a92948 | send 扩容 panic |

### 跨模块调用

FUN_1428cb5f0（park）、FUN_140c81c70/FUN_140c81e10（内存序）、FUN_140563170（Vec 扩容）、FUN_1400d9b20（Arc 克隆）、FUN_1428d9430/FUN_1428d9390（panic）。

### 遗留 U

- DeferredDecoratorOpEvents 事件 0x88B 记录字段语义（Op 载荷格式）U。
- DecoratorDst::as_wall_mut 0x140833570 的 u32 返回值与名不符待复核（已归 DecoratorOpEventKind::id）。
- DecoratorInfo 72B 全字段表（+0x22/+0x2c/+0x44/+0x46 已证，其余 U）。
