# REBUILD — session 5（restorer 复核/补齐，2026-10-17）

agent: restorer-input-session-worldraster
方法: ghidra `--worker shared` fresh 反编译（0x140b46300 / 0x140b463c0→0x1417522e0 /
0x140b46960 / 0x140b46d60 / 0x140b47840）+ radare2 fresh 反汇编（0x140b46d60 前 60 指令）。
证据: `reports/interaction/world_raster/evidence-restorer-2026-10-17/fresh-batch1.json`。

## 1. 逐函数 verdict

| 函数 | VA | verdict | 说明 |
|---|---|---|---|
| SessionId::session_file_list | 0x140b46300 | 判等（复核） | `from_iter<RelativePathBuf, Map<Iter<&str>,…>>`，无 save_folder join；顺序 history.json/screenshot.jpg/meta.json（rdata 0x142aef4f7/0x503/0x511）。Rust 实现一致。 |
| SessionMeta::write_to_disk | 0x140b463c0 → 0x1417522e0 | 判等（复核） | save_folder_path().join("meta.json") → `save_session_meta_to_file`；外层 BTreeMap `{"version": 2, "state": …}`；tmp sibling + sync + close + rename。Rust version=2 一致。 |
| NextSession::create | 0x140b46960 | 判等（复核） | out+0x18..0x78 = 模板 12 qword（kind/id/9 载荷）；+0x78..0x98 = 32 B 尾部；Vec {0,8,0}；+0x98=0。Rust `NextSessionTemplate{kind,id,payload[9]}` + `NextSessionTail{words[4]}` 原样复制。 |
| delete_session（系统） | 0x140b46d60 | 判等（修正：只删最末事件） | fresh：函数开头把 reader 游标写成 events 总计数（完整消费），随后 `movups xmm0,[last_event_ptr]` 取**最末**事件的 16 B SessionId，构 source + deleted(uuid) → `windows::fs::exists(deleted)` → 冲突（Ok(true)）重生成 uuid → rename → 失败 log "Error deleting glade: "。无 create_dir_all / 标记 / remove_dir_all。 |
| SessionFlowPlugin::build | 0x140b47840 | 判等（修正 2 处） | insert 空 SavedSessionList {0,4,0}；insert NextSession::create(kind=0x8000000000000001, nil uuid, 零载荷, Vec{0,8,0})；Startup `refresh_session_list_system(ResMut<SavedSessionList>, Option<Res<AutoSaveState>>)`；Update `(delete_session.run_if(on_event<DeleteSessionCmd>), refresh.run_if(or(on_event<SavingGameComplete>, on_event<DeleteSessionCmd>)))`；register_event NewSessionStartedCmd/SessionLoadedCmd/DeleteSessionCmd。**不插入 AutoSaveState、不插入 ActiveSession**。 |

## 2. 本轮修正

1. `delete_session_system`：`for … in reader.read()` → `reader.read().last()`（原生完整消费
   且只删最末事件）。同时 `delete_session(id)` 保留原生
   source+deleted→exists 冲突重生成→rename→log 流程（无自造三段）。
2. `SessionFlowPlugin::build`：删除 `init_resource::<AutoSaveState>()`（原生 build 只插
   SavedSessionList 与 NextSession；AutoSaveState 由 save 域注册）。
3. `configure_session_flow_systems`：补 `delete_session_system.run_if(on_event::<DeleteSessionCmd>)`
   （原生 Update 第一项就是 system+on_event 条件）。

## 3. 登记

- Startup/Update 分表：crate 无 bevy_app，`configure_session_flow_systems` 按运行期语义
  加到同一 Schedule，宿主按阶段接入（既有约定）。
- `delete_session` 的 std `windows::fs::exists` 返回包（`{is_err, ok_value}`）字节方向：
  采用语义 "目标已占用 → 重生成 uuid"，与 fresh 分支方向一致（Ok(true) → regen）。

## 4. 自检

- `cargo check -p country-core --lib`：通过。
- `cargo test -p country-core --lib`：1979 passed / 0 failed（交付时）。
