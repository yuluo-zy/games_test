# REVIEW — resources/session（组 state）

> 审查员：reviewer-state，2026-10-15。独立核对，以原二进制证据为准。
> 范围：`src/resources/session.rs`。
> 证据：session-save/DIGEST.md §2.3/§2.4、round3/SESSION-RENDER-U-CLOSURE.md（5 个 serde 单态 O）、
> 本审查 RNE 串/表直读。

## 逐函数判定

| 函数 | verdict | 说明 |
|---|---|---|
| `SessionId::new` | OK | uuid v4（0x140916420→0x140c102e0）。 |
| `SessionId::save_folder_path` | OK | paths::saves()/uuid。 |
| `SessionId::backup_folder_path` | OK | **独立顶级目录** paths::backup_saves()/uuid（round2 闭合；paths crate 有 backup_saves()）。 |
| `SessionId::snapshots_folder_path` | OK | save_folder/"snapshots"（快照 GC 目录语义；0x140b46300 实为 session_file_list，snapshots_folder_path 是 src 新增 helper，未映射到原址，合理）。 |
| `SessionId::session_file_list` | **OK** | 0x140b46300 构建 {history.json, screenshot.jpg, meta.json} 三路径。 |
| `GladeType`（ALL/native_tag/assets_folder/is_olden/flowery/autumn/winter） | OK | 5 变体 0..4 钉死；assets 表 {summer/olden/flowery/autumn/winter}；谓词 tag==1/2/3/4（0x140a9fa70/0x14092e6a0/0x14092e6c0/0x140ae8320）。 |
| `GladeType::audio_id` | **OK** | 表 0x142ad67f0 = [0.0, 3.0, 1.0, 2.0, 4.0]（RNE 实测 f32 解码），与 src 数组一致；preview_glade_parameter 消费正确。 |
| `GladeType::parse` / Deserialize / Serialize | **OK** | visit_str 0x1403b5a20：按串长+常量匹配，**首字母大小写不敏感**（"Summer"/"summer"、"Olden"/"olden" 均收；"SuMmEr"/"SUMMER" 拒绝）——src 测试行为与机器码一致；Serialize 按 tag 写主题串（0x1429ccb18 表）。 |
| `SessionMutator`（Halloween2024/2025, Xmas2024） | OK | 3 变体 0..2，季节名串 0x1429ccac6/ccad3/ccae0。 |
| `SessionMeta`（glade/mutators/image 字段、#[serde(default)]） | **OK** | 字段名 {state, glade, mutators, image}（try_load_from_path 0x140b46480 分发表 0x142aef526）；缺字段默认策略；image skip 当 None。测试断言严格 roundtrip。 |
| `SessionMeta::try_load_from_path` | OK | 读文件→取 "state"→serde。外层 version 不校验（src 注释准确）。 |
| `SessionMeta::write_to_path`/`write_to_disk` | **OK**（待取证项） | 0x1417522e0：ensure parent→写 sibling ".tmp"→sync→rename；src 加 version=1 外层。**外层 "version" 键是否原版写入未独立取证**（DIGEST default +0x00=1 暗示存在），roundtrip 自洽。 |
| `SessionMeta::is_halloween2024/is_halloween_any/is_xmas2024` | **OK** | any(b==0)/any(b<=1)/any(b==2) 机器码扫描；src 的 matches! 与 DIGEST 方向纠错一致。 |
| `SessionMeta::assets_root` | OK | "assets/glade"+主题表（0x140b46370）。 |
| `ActiveSession`（new/assets_root/is_*） | OK | 原序列化器 0x1403b5920：meta@0, id@0x60（src 字段 id+meta，2 键）。 |
| 缺失：`NextSession`（load/create/ignore_snapshot/append_on_load×3）、`SavedSession`、`refresh_session_list`、`load_saved_session_list`、`delete_session`、`SessionFlowPlugin::build` | **GAP** | 合同 remaining "full NextSession/GameSaveData/MetaCommand and world startup lifecycle" 已披露。**无伪造加载成功、无假实现**——符合"restorer 不伪造"要求。 |

## 核对要点
- **会话切换生命周期完整缺口在，且被诚实披露**：NextSessionKind 解码、on_load/MetaCommand 回放、PostGameLoadCommand、GameSaveData 组装均未实现，不冒充。
- meta.json 格式自洽（write {"version":1,"state":meta} ↔ read 取 state），与 try_load 入口一致；"version" 键来源待取证（低优先）。
- GladeType 的 Deserialize 同时支持 `"String"` 与 `{"Variant":..}` map 形（visit_enum unit_variant），与 SESSION-RENDER-U-CLOSURE 的 "string or map" 一致。

## FIX 清单
1. **待取证（低）** meta.json 外层 "version" 键：请补证 save_session_meta_to_file 0x1417522e0 是否写 version（DIGEST SessionMeta default +0x00=1 支持在 meta 内或外层存在 version）。
2. **待取证（低）** 季节枚举与字段名 `mutators` 的映射标 I（SESSION-RENDER-U-CLOSURE 注明"字段名↔Vec 映射未独立核实"）——src 已按 mutators 命名并 roundtrip 自洽，可接受。

## 结论
已实现部分（ID/主题/schema/元数据往返/音乐消费）全部核对通过或仅剩低优先待取证；完整切换生命周期（NextSession/GameSaveData/MetaCommand/会话列表/删除/插件注册）为已披露 GAP。**无伪造实现**。状态建议：现状可 `review_passed`（限定于已实现范围），完整模块验收仍 blocked（NextSession/MetaCommand 缺口）。
