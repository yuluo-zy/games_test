# resources/session.rs

> 文件作用：会话资源层：GladeType 主题判别（0=Summer/1=Olden/2=Flowery/3=Autumn/4=Winter，O）、SessionId 与保存目录协议（saves()/backups()/会话文件三件套 history.json+screenshot.jpg+meta.json）、SessionMeta 元数据（meta.json，serde 字段 {state,glade,mutators,image}）、NextSession 会话请求（0xa0B：on_load 命令 Vec + kind niche + uuid）、SavedSession(List) 会话列表/删除（deleted_saves 两段删除）。

函数总计 70：业务 28 / 内联无独立符号 16 / 派生胶水 26

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::resources::session::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>`` | 0x1403b5a20 | 36 | 序列化/serde | U（serde visit_str 派生胶水，非业务） | —  |
| ``country_core::resources::session::_::impl$0::deserialize<enum2$<serde_json::value::Value> >`` | 0x141328d00 | 36 | 序列化/serde | U（serde Value 反序列化胶水） | —  |
| ``country_core::resources::session::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >`` | 0x1403b52d0 | 36 | 序列化/serde | U（serde SliceRead 反序列化胶水） | —  |
| ``country_core::resources::session::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >`` | 0x1403b5500 | 36 | 序列化/serde | U（serde Serializer 胶水） | —  |
| ``country_core::resources::session::_::impl$0::serialize<serde_json::value::ser::Serializer>`` | 0x141328bb0 | 36 | 序列化/serde | U（serde Value Serializer 胶水） | —  |
| ``country_core::resources::session::GladeType::is_olden`` | 0x140a9fa70 | 94 | 谓词/校验 | O：tag==1 判定（cmp byte[rcx],1; sete） | session-save\DIGEST.md  |
| ``country_core::resources::session::GladeType::is_flowery`` | 0x14092e6a0 | 102 | 谓词/校验 | O：tag==2 判定 | ui-input\decomp\decomp-0x14092e6a0.txt  |
| ``country_core::resources::session::GladeType::is_autumn`` | 0x14092e6c0 | 110 | 谓词/校验 | O：tag==3 判定 | session-save\DIGEST.md  |
| ``country_core::resources::session::GladeType::is_winter`` | 0x140ae8320 | 118 | 谓词/校验 | O：tag==4 判定（与 BackupSaveFileKind::is_corrupted 同址折叠） | session-save\decomp\decomp-0x140ae8320.txt  |
| ``country_core::resources::session::SessionId::new`` | 0x140916420 | 135 | 构造 | O：uuid v4 生成（call 0x140c102e0） | session-save\DIGEST.md  |
| ``country_core::resources::session::SessionId::save_folder_path`` | 0x140b46000 | 139 | IO/解析 | O：paths::saves()(0x140b9db60)/uuid | round2\session-save-CLOSURE.md  |
| ``country_core::resources::session::SessionId::backup_folder_path`` | 0x140b46180 | 147 | 待推断 | O：paths::backups()(0x140b9db90)/uuid（独立根目录，非 saves 子目录） | session-save\decomp\decomp-0x140b46180.txt, round2\session-save-CLOSURE.md  |
| ``country_core::resources::session::SessionId::session_file_list`` | 0x140b46300 | 161 | IO/解析 | O：构建 {history.json, screenshot.jpg, meta.json} 三条完整路径 | session-save\decomp\decomp-0x140b46300.txt  |
| ``country_core::resources::session::SessionMeta::assets_root`` | 0x140b46370 | 198 | 查表/访问 | O："assets/glade"+主题表[+0x58]（summer/olden/flowery/autumn/winter） | round2\decomp-0x140b46370.txt  |
| ``country_core::resources::session::SessionMeta::write_to_disk`` | 0x140b463c0 | 202 | 设置/写入 | O：save_folder_path+"meta.json"+save_session_meta_to_file(0x1417522e0) | round2\decomp-0x140b463c0.txt  |
| ``country_core::resources::session::SessionMeta::try_load_from_path`` | 0x140b46480 | 210 | 序列化/serde | O：读文件+serde 反序列化；字段名 {state,glade,mutators,image}（首字节分发表） | session-save\decomp\decomp-0x140b46480.txt  |
| ``country_core::resources::session::SessionMeta::is_halloween2024`` | 0x140b46860 | 219 | 谓词/校验 | O：any(b==0) 扫描 | session-save\DIGEST.md  |
| ``country_core::resources::session::SessionMeta::is_halloween_any`` | 0x140b46890 | 225 | 谓词/校验 | O：any(b<=1) 扫描（纠错：wp13 合同写 >1 方向相反） | session-save\DIGEST.md  |
| ``country_core::resources::session::SessionMeta::is_xmas2024`` | 0x140b468c0 | 234 | 谓词/校验 | O：any(b==2) 扫描 | session-save\decomp\decomp-0x140b468c0.txt  |
| ``country_core::resources::session::NextSession::load`` | 0x140b46920 | 339 | IO/解析 | O：{cap-first 空 Vec(cap=8), kind=0x8000000000000002, uuid@+0x20, ignore_snapshot=false} | session-save\decomp\decomp-0x140b46920.txt  |
| ``country_core::resources::session::NextSession::create`` | 0x140b46960 | 347 | 构造/注册/生成 | O：96B 模板(rdx)+32B(r8) 构造；kind@+0x18=模板首 qword；Vec 清空 | round2\session-save-CLOSURE.md  |
| ``country_core::resources::session::NextSession::ignore_snapshot`` | 0x140b46a20 | 358 | 设置/写入 | O：写 +0x98 bool + memcpy 0xa0（builder） | session-save\DIGEST.md  |
| ``country_core::resources::session::NextSession::append_on_load<array$<enum2$<country_core::meta_commander::MetaCommand>,3> >`` | 0x1403b5150 | 363 | 待推断 | O：push 3 条 MetaCommand（块 0xa8B）；载荷 U | —  |
| ``country_core::resources::session::NextSession::append_on_load<country_core::meta_commander::MetaCommandFence>`` | 0x141328ac0 | 363 | 待推断 | O：push 0x20B 条目 {u32 0, u32 0x80000000, u64 payload} | session-save\decomp\decomp-0x141328ac0.txt  |
| ``country_core::resources::session::SavedSession::loading_background_path`` | 0x140b46a50 | 407 | IO/解析 | O：save_folder_path+"loading_background.jpg"（v2 名；v1 名 screenshot_path 错位） | session-save\decomp\decomp-0x140b46a50.txt  |
| ``country_core::resources::session::refresh_session_list`` | 0x140b46b00 | 457 | 待推断 | O：清空后 paths::saves() readdir 重建 | round2\decomp-0x140b46b00.txt  |
| ``country_core::resources::session::delete_session`` | 0x140b46d60 | 464 | 删除/清理 | O：deleted_saves()/uuid 标记文件+rename 两段删除；失败日志 Error deleting glade:  | session-save\decomp\decomp-0x140b46d60.txt  |
| ``country_core::resources::session::impl$10::build`` | 0x140b47840 | 482 | 构造/注册/生成 | I：SessionFlowPlugin 域 build（Bevy 事件发送胶水：component.rs/event registry 路径证据） | session-save\decomp\decomp-0x140b47840.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::session::GladeType::assets_folder`（行 53）— 待推断
- `country_core::resources::session::GladeType::theme_id`（行 65）— 待推断
- `country_core::resources::session::GladeType::audio_id`（行 76）— 音频
- `country_core::resources::session::SessionId::snapshots_folder_path`（行 151）— 待推断
- `country_core::resources::session::ActiveSession::new`（行 264）— 构造
- `country_core::resources::session::ActiveSession::assets_root`（行 269）— 待推断
- `country_core::resources::session::ActiveSession::is_winter`（行 274）— 谓词/校验
- `country_core::resources::session::ActiveSession::is_autumn`（行 279）— 谓词/校验
- `country_core::resources::session::ActiveSession::is_olden`（行 284）— 谓词/校验
- `country_core::resources::session::ActiveSession::is_flowery`（行 289）— 谓词/校验
- `country_core::resources::session::impl$6::from`（行 319）— 待推断
- `country_core::resources::session::impl$7::from`（行 325）— 待推断
- `country_core::resources::session::NextSession::load_latest`（行 331）— IO/解析
- `country_core::resources::session::NextSession::append_on_load`（行 363）— 构造/注册/生成
- `country_core::resources::session::SavedSession::screenshot_path`（行 403）— 待推断
- `country_core::resources::session::load_saved_session_list`（行 420）— IO/解析

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 26 项
