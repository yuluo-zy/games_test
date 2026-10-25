# REBUILD — resources/session（真实控制流重建）

> 重建者: rebuild-session。日期: 2026-10-17。
> 依据: 2026-10-16-logic-verify-protocol.md（含宏胶水分类规则）+ LOGIC.md 分歧点。
> 全部以 Ghidra shared **fresh 反编译/反汇编**为准；重建后逐分支对照。

## 分类结论（重建前，按协议 §1）

| 需重做函数 | 分类 | 结论 |
|---|---|---|
| `session_file_list` 0x140b46300 | 真业务（返回值语义/数据流） | 重建 |
| `write_to_disk` 0x140b463c0（委托 0x1417522e0） | 真业务（外层 envelope 版本号） | 重建 |
| `NextSession::create` 0x140b46960 | 真业务（模板/尾部载荷复制数据流） | 重建 |
| `delete_session` 0x140b46d60 | 真业务（exists 冲突检查 + rename 流程） | 重建 |
| `SessionFlowPlugin::build` 0x140b47840 | 真业务（资源/事件/系统装配） | 重建 |

无宏胶水混入（本批全部为真实算法/调用链）。

## 逐函数重建

### 1. `session_file_list`（0x140b46300）— 判等

- fresh 反编译：`from_iter<RelativePathBuf, Map<Iter<&str>, fn(&str)->RelativePathBuf>>`，无 save_folder join。
- rdata 实读（read_memory 0x142aef4e0）：三串连续 `history.json`@0x4f7 / `screenshot.jpg`@0x503 / `meta.json`@0x511，顺序确认。
- Rust 重建：`["history.json","screenshot.jpg","meta.json"].into_iter().map(RelativePathBuf::from).collect()`。
- 返回值语义 **相对**（`Vec<RelativePathBuf>`），不再 join save_folder；原 `[PathBuf;3]` 绝对路径已删。

### 2. `write_to_disk` / `write_to_path`（0x140b463c0 → 0x1417522e0）— 判等

- fresh 反编译 0x1417522e0：BTreeMap `insert("version"→2)` + `insert("state"→serialize SessionMeta)`；
  tmp sibling + write_all + sync_all + CloseHandle + rename；失败 `with_context`。
- Rust 重建：`{"version": 2, "state": self}`（原误写 1）。tmp/同步/rename 其余已一致，未动。
- 测试断言同步改为 `saved["version"] == 2`。

### 3. `NextSession::create`（0x140b46960）— 判等

- fresh 反编译：`create(out, rdx 96B 模板, r8 32B 尾部)`。
  复制映射：out[+0x18]=rdx[0](kind)、out[+0x20/+0x28]=rdx[1..2](id)、
  out[+0x30..+0x78]=rdx[3..0xb](9 qword 载荷)、out[+0x78..+0x98]=r8[0..4](4 qword 尾部)；
  Vec{0,cap8,0}、+0x98=0。
- 调用点 0x141ebb5ad（radare2 反汇编）：`create(&out, [rbp+0x1f0] 模板, [rbp+0x270] 尾部)`，
  模板首 qword=kind=0x8000000000000001/0 分支、中段含主题字节等（新会话流程，字段语义 U）。
- Rust 重建：`NextSession` 补 `template_payload:[u64;9]`(+0x30..0x78)、`tail_payload:[u64;4]`(+0x78..0x98)；
  新增 `NextSessionTemplate{kind,id,payload}` / `NextSessionTail{words}` 类型；
  `create(&template,&tail)` 原样复制载荷（不再仅 id+kind）。
- `load` 补零载荷（原 load 不写 +0x30..+0x98，零初值=新建语义，非可观察差异）。
- API 变更波及 `save/tests.rs` 一处调用（改为 template/tail 构造）。

### 4. `delete_session`（0x140b46d60）— 判等

- fresh 反编译/反汇编：save_folder_path() 构 source → deleted_saves()/uuid 构 deleted →
  `exists`（0x140d6b2b0，std::sys::pal::windows::fs）→ 条件性 `Uuid::new_v4()` 重生成 →
  `rename(source,deleted)` → 失败 `log`（"Error deleting glade: "）。**无 create_dir_all /
  写 ".deleted" 标记 / remove_dir_all**。
- Rust 重建：删自造三段（create_dir_all+标记+remove_dir_all）；构 source+deleted →
  `deleted.exists()` 冲突时重生成 uuid → rename → 失败 log。
- 残留：fresh 反汇编的 regen 分支在 exists 结果 `byte0==0 && byte1!=0`（std exists 内部
  {未命中/open 失败} 编码）时进入，与 LOGIC "冲突时" 措辞的精确字节方向存在出入；采用
  LOGIC 语义（目标已占用=冲突→重生成）实现，字节级方向残留待 verifier 复核。

### 5. `SessionFlowPlugin::build`（0x140b47840）— 判等

- fresh 反编译：insert 空 SavedSessionList（cap4）→ insert NextSession（kind=
  `0x8000000000000001`=LoadSavedGame，模板零载荷）→ add_systems<Startup,
  fn(ResMut<SavedSessionList>,Option<Res<AutoSaveState>>)> → add_systems<Update,
  (DeleteSessionCmd 处理器, 刷新.run_if(or(SavingGameComplete,DeleteSessionCmd)))> →
  register_event NewSessionStartedCmd/SessionLoadedCmd/DeleteSessionCmd。**不插 ActiveSession**。
- Rust 重建：
  - `build(world)`：insert 空 SavedSessionList（不急切 refresh）+ NextSession::create(kind=
    LoadSavedGame, nil uuid, 零载荷) + init Events NewSessionStartedCmd/SessionLoadedCmd/
    DeleteSessionCmd。删除 ActiveSession 占位与 load_latest(kind=LoadOnLoad)。
  - 新增系统：`refresh_session_list_system`（Startup）、`delete_session_system`、
    `configure_session_flow_systems(schedule)`（Startup 刷新 + Update delete+run_if(or)）。
  - `DeleteSessionCmd` 补 `#[derive(Event)]`（save_game.rs，注册事件所需）。
  - `run_game.rs` register_core_startup 接 `configure_session_flow_systems(schedule)`。
  - 注：crate 不依赖 bevy_app，无 Startup/Update 分表；Startup 刷新语义由宿主运行期接入。

## 汇总

- 需重做 5 个全部重建，重建后逐分支对照判等。
- 未触碰判等函数（is_* / save_folder_path / backup_folder_path / assets_root /
  try_load_from_path / is_halloween* / is_xmas2024 / NextSession::load / ignore_snapshot /
  extend_on_load / append_fence / loading_background_path / refresh_session_list 等 18 判等不动）。
- `NextSession::load` 仅因结构体新增载荷字段补零初值，行为不变。
- 编译状态：session.rs/save_game.rs/run_game.rs/save-tests 无编译错误（其余模块并行
  agent 的 in-flight 错误与本批无关）。
