# game_config.rs

> 文件作用：**<待填>**

函数总计 31：业务 9 / 内联无独立符号 10 / 派生胶水 12

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `game_config::DevSettings::live_link` | 0x14090df40 | 0 | <待填> | U | — |
| `game_config::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >` | 0x1402e7de0 | 0 | <待填> | U | — |
| `game_config::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >` | 0x1402e73b0 | 0 | <待填> | U | — |
| `game_config::_::impl$0::next` | 0x1408b99e0 | 52 | <待填> | U | — |
| `game_config::_::impl$0::previous` | 0x140914840 | 52 | <待填> | U | — |
| `systems::main_camera::_::impl$0::next` | 0x1408b99e0 | 52 | <待填> | U | — |
| `game_config::GameConfig::upgrade` | 0x14090df90 | 128 | <待填> | U | — |
| `game_config::is_within_glade_shape` | 0x14090dfe0 | 155 | <待填> | U | — |
| `game_config::developer_hotkeys_enabled` | 0x14090e070 | 182 | <待填> | U | — |

## 内联函数名称推断

- `game_config::_::impl$0::deserialize::impl$2::visit_map` (L0, inline_instance, inlinee_declaration)
- `game_config::_::impl$0::deserialize::impl$0::visit_str` (L52, inline_instance, inlinee_declaration)
- `game_config::_::impl$0::deserialize::impl$2::visit_enum` (L52, inline_instance, inlinee_declaration)
- `game_config::clamp_to_rounded_glade_shape` (L138, inline_instance, inlinee_declaration)
- `game_config::clamp_to_glade_shape` (L146, inline_instance, inlinee_declaration)
- `game_config::clamp_to_circle` (L159, inline_instance, inlinee_declaration)
- `game_config::clamp_to_rounded_box` (L164, inline_instance, inlinee_declaration)
- `game_config::deserialize_optional` (L186, inline_instance, inlinee_declaration)
- `game_config::serialize_optional` (L195, inline_instance, inlinee_declaration)
- `game_config::is_default` (L205, inline_instance, inlinee_declaration)

## 派生胶水分组

- `game_config::_::impl$0::deserialize` (L0)
- `game_config::_::impl$0::deserialize::impl$1::deserialize` (L0)
- `game_config::impl$1::fmt` (L0)
- `game_config::impl$3::clone` (L11)
- `game_config::impl$5::default` (L11)
- `game_config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize` (L68)
- `game_config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize` (L68)
- `game_config::_::impl$0::serialize::impl$0::serialize` (L68)
- `game_config::impl$10::default` (L68)
- `game_config::impl$8::clone` (L68)
- `game_config::impl$11::clone` (L114)
- `game_config::impl$13::default` (L114)
