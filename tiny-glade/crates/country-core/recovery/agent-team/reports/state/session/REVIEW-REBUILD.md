# REVIEW — rebuild-session 复验（组 state：session/music/camera 重建）

> 审查员：reviewer-state，2026-10-17。复核 rebuild-session 对 session/music/camera 的重建。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（13 警告，含他组在途）；
> **全 lib test 目标当前被并行组 in-flight 错误打断**（56 错误全在他组文件：backend/input/mod_key
> 20、asset_libraries/* 15、render_loop/* 12、wall/snapping、terrain——**零个在 state 文件**），
> 状态组重建编译一致。会话测试断言 version==2 已改。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| 分类：10 全真业务 | **OK** | 5 session（session_file_list/write_to_disk/NextSession::create/delete_session/SessionFlowPlugin::build）+ camera current_dyn/current_dyn_mut/current_body_offset/load_parameters/eye×3 均为真实算法/调用链，无宏胶水混入。 |
| session_file_list → Vec<RelativePathBuf> | **OK** | 相对路径（不 join save_folder）；rdata 0x142aef4f7/503/511 三串实读，顺序确认。 |
| write_to_disk version=2 | **OK** | 代码 `{"version": 2, "state": self}`（原误写 1）；测试断言 `saved["version"]==2`。fresh 0x1417522e0 BTreeMap version→2。 |
| NextSession::create 9qword 模板 + 4qword 尾部 | **OK** | `NextSessionTemplate{kind,id,payload}`/`NextSessionTail{words}`；create 原样复制载荷（+0x18 kind/+0x20 id/+0x30..0x78 9qword/+0x78..0x98 4qword）；load 补零初值（行为不变）。 |
| delete_session 去伪造 + exists 冲突重生成 | **OK（字节方向残留）** | 删自造 create_dir_all+标记+remove_dir_all；source+deleted(uuid) → exists 冲突 → 重生成 uuid → rename → log。残留：regen 分支 exists 结果 byte{0,1} 方向与 LOGIC 措辞出入，已标注待 verifier。 |
| SessionFlowPlugin::build | **OK** | insert 空 SavedSessionList + NextSession(kind=LoadSavedGame,nil uuid,零载荷) + Events(NewSessionStarted/SessionLoaded/DeleteSessionCmd)；**不插 ActiveSession**；新增 refresh_session_list_system(Startup)+delete_session_system+configure_session_flow_systems(run_game 已接)；crate 不依赖 bevy_app，Startup 语义由宿主接入。 |
| music 无改码 | **OK** | fresh 反汇编 0x140a72cfb/0x140a72ecb：mode==2 → start 目标 photomode(+0x70)、else → first_person(+0xa0)；与现有 `update_music_audio` 逐分支一致。LOGIC"start 互换"=verifier 误读，**未改码正确**。 |
| camera current_dyn/load_parameters | **OK（已核）** | 交互组 camera_rig/REBUILD：current_dyn 六 case 4 元输出（body_abs [0x70..0x388] 修正、vtable/dyn_body_ptr 补回）；load_parameters 轨道四元数 YXZ/ActiveRotation/+0x194=pitch_offset；rotations 原生=替换（dealloc+len=1）非 push，标注待 cameraui 吸收。 |

## 结论与 DISPATCH 建议
- **session/music 需重做项 → 闭合**。5 个 session 重建判等；music 经 fresh 反汇编证 Rust 原已正确（无改码）；
  camera current_dyn/load_parameters 重建在交互组 REBUILD 核。delete_session regen 字节方向 + rotations
  替换语义为登记残留（待 verifier/cameraui）。
- **编译告警**：全 lib test 目标当前被并行组 in-flight 错误打断（非 state 文件）；建议主会话协调各域
  收口后重验全 crate 绿窗口。
- session/music → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（外部依赖 + 全局调度）。
