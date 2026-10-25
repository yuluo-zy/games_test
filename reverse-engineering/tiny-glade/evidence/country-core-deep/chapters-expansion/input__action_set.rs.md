# input/action_set.rs

> 文件作用：**<待填>**

函数总计 26：业务 13 / 内联无独立符号 4 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `input::action_set::DigitalActionState::just_released` | 0x14093fac0 | 0 | <待填> | U | — |
| `input::action_set::DigitalActionState::pressed` | 0x14093fa70 | 0 | <待填> | U | — |
| `input::action_set::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1416f92d0 | 17 | <待填> | U | — |
| `input::action_set::_::impl$0::serialize<glam::f32::vec2::Vec2,bool,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400ed690 | 17 | <待填> | U | — |
| `input::action_set::_::impl$0::serialize<glam::f32::vec2::Vec2,bool,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x1416f8680 | 17 | <待填> | U | — |
| `input::action_set::MainActionSet::just_pressed` | 0x140946620 | 35 | <待填> | U | — |
| `input::action_set::MainActionSet::deconstruct` | 0x140946b80 | 43 | <待填> | U | — |
| `input::action_set::MainActionSet::reconstruct` | 0x140947c80 | 64 | <待填> | U | — |
| `input::action_set::ActionSetAction::just_pressed` | 0x1409487b0 | 102 | <待填> | U | — |
| `input::action_set::ActionSetAction::pressed` | 0x1409487e0 | 113 | <待填> | U | — |
| `input::action_set::ActionSetAction::as_analog` | 0x140948800 | 124 | <待填> | U | — |
| `input::action_set::DigitalActionState::released` | 0x14093fa80 | 250 | <待填> | U | — |
| `input::action_set::DigitalActionState::just_pressed` | 0x14093fa90 | 253 | <待填> | U | — |

## 内联函数名称推断

- `input::action_set::is_default` (L4, inline_instance, inlinee_declaration)
- `input::action_set::_::impl$0::deserialize::impl$2::expecting` (L17, inline_instance, inlinee_declaration)
- `input::action_set::_::impl$0::deserialize::impl$2::visit_map` (L17, inline_instance, inlinee_declaration)
- `input::action_set::_::impl$0::deserialize::impl$2::visit_seq` (L17, inline_instance, inlinee_declaration)

## 派生胶水分组

- `input::action_set::_::impl$0::deserialize` (L17)
- `input::action_set::_::impl$0::deserialize::impl$1::deserialize` (L17)
- `input::action_set::impl$4::default` (L17)
- `input::action_set::impl$16::eq` (L148)
- `input::action_set::impl$19::hash` (L148)
- `input::action_set::impl$23::eq` (L211)
- `input::action_set::impl$0::default` (L236)
- `input::action_set::impl$26::default` (L286)
- `input::action_set::impl$31::default` (L301)
