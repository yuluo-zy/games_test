# input/input_stream.rs

> 文件作用：**<待填>**

函数总计 24：业务 10 / 内联无独立符号 9 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `input::input_stream::InputStream::frame_count` | 0x140949180 | 0 | <待填> | U | — |
| `input::input_stream::InputStream::sync_point_count` | 0x140949170 | 0 | <待填> | U | — |
| `input::input_stream::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1413fe800 | 0 | <待填> | U | — |
| `input::input_stream::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1402e0060 | 0 | <待填> | U | — |
| `input::input_stream::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x1413fe6f0 | 0 | <待填> | U | — |
| `input::input_stream::InputStream::add_frame` | 0x140949190 | 131 | <待填> | U | — |
| `input::input_stream::InputStream::last_frame_mut` | 0x140949480 | 152 | <待填> | U | — |
| `input::input_stream::InputStream::on_history_sync` | 0x1409494c0 | 158 | <待填> | U | — |
| `input::input_stream::InputStream::drop_oldest_sync_point` | 0x140949660 | 178 | <待填> | U | — |
| `input::input_stream::InputRecordReplayState::try_load` | 0x1409499c0 | 231 | <待填> | U | — |

## 内联函数名称推断

- `input::input_stream::_::impl$0::deserialize::impl$0::visit_str` (L0, inline_instance, inlinee_declaration)
- `input::input_stream::_::impl$0::deserialize::impl$2::expecting` (L0, inline_instance, inlinee_declaration)
- `input::input_stream::_::impl$0::deserialize::impl$2::visit_map` (L0, inline_instance, inlinee_declaration)
- `input::input_stream::_::impl$0::deserialize::impl$2::visit_seq` (L0, inline_instance, inlinee_declaration)
- `input::input_stream::InputReplayInitialState::new` (L62, inline_instance, inlinee_declaration)
- `input::input_stream::InputStream::new` (L108, inline_instance, inlinee_declaration)
- `input::input_stream::InputStream::sync_point` (L123, inline_instance, inlinee_declaration)
- `input::input_stream::InputRecordReplayState::new` (L224, inline_instance, inlinee_declaration)
- `input::input_stream::InputRecordReplayState::seek_to_sync_point` (L242, inline_instance, inlinee_declaration)

## 派生胶水分组

- `input::input_stream::_::impl$0::deserialize` (L0)
- `input::input_stream::_::impl$0::deserialize::impl$1::deserialize` (L0)
- `input::input_stream::impl$5::default` (L82)
- `input::input_stream::impl$2::index` (L209)
- `input::input_stream::impl$8::fmt` (L258)
