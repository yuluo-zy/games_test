# game_key_bindings.rs

> 文件作用：**<待填>**

函数总计 17：业务 10 / 内联无独立符号 4 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `game_key_bindings::KeyBinding::action_desc` | 0x14089cb60 | 0 | <待填> | U | — |
| `game_key_bindings::MouseBinding::action_desc` | 0x14089c8a0 | 0 | <待填> | U | — |
| `game_key_bindings::mouse_button_to_string` | 0x14089c970 | 0 | <待填> | U | — |
| `game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >` | 0x140366a40 | 4 | <待填> | U | — |
| `game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >` | 0x140831740 | 4 | <待填> | U | — |
| `game_key_bindings::KeyBindings::assign_mouse_button` | 0x14089b280 | 110 | <待填> | U | — |
| `game_key_bindings::KeyBindings::assign_key` | 0x14089b2a0 | 117 | <待填> | U | — |
| `game_key_bindings::MouseBinding::get_mouse_button` | 0x14089c880 | 286 | <待填> | U | — |
| `game_key_bindings::KeyBinding::get_keycode` | 0x14089d300 | 423 | <待填> | U | — |
| `game_key_bindings::keycode_to_str` | 0x14089d490 | 468 | <待填> | U | — |

## 内联函数名称推断

- `game_key_bindings::_::impl$0::deserialize::impl$2::visit_map` (L4, inline_instance, inlinee_declaration)
- `game_key_bindings::impl$1::assign_key::closure$0` (L118, inline_instance, inlinee_declaration)
- `game_key_bindings::KeyBindingDesc::text` (L361, inline_instance, inlinee_declaration)
- `game_key_bindings::KeyBindingDesc::icon` (L368, inline_instance, inlinee_declaration)

## 派生胶水分组

- `game_key_bindings::_::impl$0::deserialize` (L4)
- `game_key_bindings::_::impl$0::deserialize::impl$1::deserialize` (L4)
- `game_key_bindings::impl$0::default` (L59)
