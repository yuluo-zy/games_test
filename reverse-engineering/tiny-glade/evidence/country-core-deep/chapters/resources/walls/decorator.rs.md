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
| `country_core::resources::walls::decorator::DecoratorType::can_place_on_stair_top` | 0x140833190 | 108 | DecoratorType::can_place_on_stair_top（(t+0xc4)<3 || (t&0x38)!=0x38） | O | wall\decomp\decomp-0x140833190.txt |
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
| `country_core::resources::walls::decorator::impl$97::from` | 0x140839560 | 403 | DecoratorExtraInfo From 变体 8（写判别字节 +0x11=8） | O |  |
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
