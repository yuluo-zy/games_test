# resources/session.rs

> 文件作用：会话资源层：GladeType 主题判别（0=Summer/1=Olden/2=Flowery/3=Autumn/4=Winter，O）、SessionId 与保存目录协议（saves()/backups()/会话文件三件套 history.json+screenshot.jpg+meta.json）、SessionMeta 元数据（meta.json，serde 字段 {state,glade,mutators,image}）、NextSession 会话请求（0xa0B：on_load 命令 Vec + kind niche + uuid）、SavedSession(List) 会话列表/删除（deleted_saves 两段删除）。

函数总计 70：业务 28 / 内联无独立符号 16 / 派生胶水 26

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::resources::session::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>`` | 0x1403b5a20 | 36 | 序列化/serde | O：**GladeType Deserialize 访问器 visit_str**——大小写不敏感匹配 5 主题串 "Summer"→0/"Olden"→1/"Flowery"→2/"Autumn"→3/"Winter"→4；未知串→serde unknown_variant（FUN_1428a2480，期望列表 @0x1429ccb38） | session-save\decomp\decomp-0x1403b5a20.txt, round3\SESSION-RENDER-U-CLOSURE.md |
| ``country_core::resources::session::_::impl$0::deserialize<enum2$<serde_json::value::Value> >`` | 0x141328d00 | 36 | 序列化/serde | O：GladeType 自 serde_json::Value 反序列化（tag 3=Value::String→主题串变体索引、tag 5=Value::Object→hashbrown 查变体名；错误 "string or map"） | session-save\decomp\decomp-0x141328d00.txt, round3\SESSION-RENDER-U-CLOSURE.md |
| ``country_core::resources::session::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >`` | 0x1403b52d0 | 36 | 序列化/serde | O：**季节 mutator 枚举（3 变体 {Halloween2024=0,Halloween2025=1,Xmas2024=2}）流式 JSON 反序列化**——经 FUN_14005e900 匹配季节名串（@0x1429ccac6/ccad3/ccae0，期望列表 @0x1429ccae8） | session-save\decomp\decomp-0x1403b52d0.txt, round3\SESSION-RENDER-U-CLOSURE.md |
| ``country_core::resources::session::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >`` | 0x1403b5500 | 36 | 序列化/serde | O：GladeType 流式 JSON 序列化——switch tag 0..4 写主题串（表 @0x1429ccb18："Summer"@+0/"Olden"@+6/"Flowery"@+11/"Autumn"@+18/"Winter"@+24） | session-save\decomp\decomp-0x1403b5500.txt, round3\SESSION-RENDER-U-CLOSURE.md |
| ``country_core::resources::session::_::impl$0::serialize<serde_json::value::ser::Serializer>`` | 0x141328bb0 | 36 | 序列化/serde | O：GladeType 序列化为 serde_json::Value（同表构造 Value::String） | session-save\decomp\decomp-0x141328bb0.txt, round3\SESSION-RENDER-U-CLOSURE.md |
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

`clone`、`closure`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 26 项（**round3：5 单态已反编译，两会话枚举 serde 串合同闭合——GladeType 变体名 {Summer,Olden,Flowery,Autumn,Winter}、tag 0..4、大小写不敏感；季节枚举 {Halloween2024,Halloween2025,Xmas2024}、tag 0..2**；其余仍按 derive 复用）

## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **会话目录协议**：save_folder_path 0x140b46000 = paths::saves()/uuid（Display fmt
  失败 panic "a Display implementation returned an error unexpectedly"
  @0x142aef358）；backup_folder_path 0x140b46180 = **paths::backups()/uuid（独立顶级
  目录，round2 闭合——无额外叶名）**；session_file_list 0x140b46300 = {history.json,
  screenshot.jpg, meta.json} 三叶名路径构建；loading_background_path 0x140b46a50 =
  +"loading_background.jpg"（PDB 名 screenshot_path 错位）；delete_session
  0x140b46d60 = deleted_saves()/uuid 标记文件 → rename 移动 → 两段删除（失败 log
  "Error deleting glade: " @0x142aef5a0）。
- **SessionMeta::try_load_from_path 0x140b46480**：读文件 → serde 反序列化；字段名
  {state, glade, mutators, image}（首字节分发表 @0x142aef526；字段 4 载荷 =
  serde_json::Value）；季节谓词为"存在字节匹配"扫描：is_halloween2024 = any(b==0)、
  is_halloween_any = any(b<=1)（**纠错：合同"扫描 >1"方向相反**）、is_xmas2024 =
  any(b==2)；assets_root 0x140b46370 = "assets/glade" + 主题表
  {summer/olden/flowery/autumn/winter}[+0x58]（GladeType 0=Summer 值域 0..4 闭合）；
  write_to_disk 0x140b463c0 = save_folder_path+"meta.json" + save_session_meta_to_file。
- **GladeType serde 合同（round3 闭合，5 单态反编译）**：**Serialize**（Compact 流式 0x1403b5500 / Value 0x141328bb0）按变体 tag 0..4 写**主题串** {0:"Summer",1:"Olden",2:"Flowery",3:"Autumn",4:"Winter"}（串表 @0x1429ccb18："Summer"@+0/"Olden"@+6/"Flowery"@+11/"Autumn"@+18/"Winter"@+24）；**Deserialize**（SliceRead 流式 0x1403b52d0 为**季节枚举**（见下）、Value 0x141328d00 为 GladeType）访问器 visit_str 0x1403b5a20 **大小写不敏感**匹配（"Summer"与"summer"均收）→ 变体 0..4，未知串→ serde unknown_variant（期望列表 @0x1429ccb38）；也接受 `{"Variant":..}` map 形（visit_enum）。**meta.json 的 `glade` 字段 JSON 形态 = 主题串**。
- **季节 mutator 枚举（round3 新证，3 变体）**：{**Halloween2024=0, Halloween2025=1, Xmas2024=2**}——deserialize\<SliceRead\> 0x1403b52d0 经 FUN_14005e900 匹配季节名串（@0x1429ccac6 len13 / @0x1429ccad3 len13 / @0x1429ccae0 len8，期望列表 @0x1429ccae8 3 项）；为该字节 Vec（字段 @+0x08 ptr/+0x10 len，元素 0/1/2）的元素类型，与季节谓词 any(b==0)/any(b<=1)/any(b==2) 互证（**DIGEST"1=其他万圣节"钉死为 Halloween2025**；与字段名 mutators 对应标 I）。
- **SessionMeta/ActiveSession 访问器 expecting（round3 新证）**：0x140b47bb0="variant identifier"（visit_enum variant-seed）、0x140b47bd0="struct SessionMeta"、0x140b47bf0="struct ActiveSession"（各 lea rdx+len+jmp fmt thunk，串 @0x142aef5e8/5fa/60c）——session.rs 另有两结构体 serde 访问器，字段名分发表仍在 try_load_from_path。
- **NextSession**：load 0x140b46920 = {cap-first 空 Vec(cap=8), kind=
  0x8000000000000002, uuid@+0x20, ignore_snapshot=false@+0x98}；create 0x140b46960 =
  96B 模板(rdx)+32B(r8) 构造（kind 由调用方模板决定——变体 1 生产者在 UI/会话流
  vtable，卡点）；ignore_snapshot 0x140b46a20 = 写 +0x98 + memcpy 0xa0（builder）；
  append_on_load<MetaCommandFence> 0x141328ac0 = push 0x20B {u32 tag=0, u32
  0x80000000, u64 payload} + memcpy 0xa0（on_load 列表 fence =
  0x8000000000000003，load_game 回放终止条件）。
