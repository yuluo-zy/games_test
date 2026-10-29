# M01 自由墙移动与控制点编辑，2026-10-26

本轮基于 main@858da5891，按 B 已存在的 `ui_move_freehand_walls.rs`、`ui_edit_freehand_walls.rs` 路径增加正式实现，不新建业务目录。沿用现有 InnerWalls、PublicWalls、Curve2、UiState、History、DecoratorArchivist 和共用移动手柄。

## 原始证据与反编译修正

- 执行基准 B `tiny-glade.rne` SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；分析库 patched EXE SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。已逐范围比对原指令字节，未把两者文件身份混用。
- 原移动系统 `1409781d0`，控制点系统 `140ac98e0`。参数依据同目录 `m01-system-params-from-pdb.md` 的原 FunctionSystem，分别16/15组，展开22/20参数。
- 共享库原 `get_functions` 输出均达120000字符截断，并跳到下一业务函数。其1251/2202字节 function body 与PDB边界不符，不可用这些输出直接复现业务。
- 专属 ReVa worker `m01-freehand-20261026`：只对两段原指令SHA匹配的范围修 Function body，剔除其他原函数占有范围；原生四项相对跳表由 `JumpTable.writeOverride` 写入当前函数事务。共享程序未修改。操作、字节SHA、原始目标与返回状态见 `m01-freehand-flow-prepare.json`。
- 移动表 `142ab7504`，跳转 `140978638`：None→97983e、Started→97863a、Held→978861、Finished→978917。
- 编辑表 `142ae0cb8`，跳转 `140aca189`：None→ac9dc5、Started→aca18b、Held→aca3e0、Finished→aca360。
- 修后原反编译50166/61241字符，无超时、无截断。旁附原RNE有限反汇编及原闭包 `140acc850`。反编译仍不等于已核准Rust ABI，关键分支以ASM确认。

## 已补业务

移动：原曲线中点、单位法线与相机水平朝向决定手柄偏移；实际地形/墙 elevation 决定高度；复用 `ui_move_shapes::move_gizmo_and_trigger_vol`，返回真正的 TriangleMesh Widget。Started归档整墙/地面及活碰撞载荷、扩大剔除许可、保存相对装饰锚点与完整源曲线。Held按原 `(cursor-grab_offset)-first_point` 平移，保留缓存 points_u/length，逐点执行谷地边界；发真实移动事件、树与图钉请求。越界不写权威曲线或播放编辑声音。Finished按原MorphFreehandWall单组并发历史记录，不按几何相等吞掉release。

控制点：每个原控制点注册半径0.75的真实球Widget；工厂使用原 `temp_sphere`，活动/普通尺度0.3/0.15。Started记录装饰最近曲线段/段内t、完整before曲线、抓取偏移。Held只移动所选点，闭合墙首尾同步；调用完整原曲线参数重建算法，按原顺序检查 index±1 邻边0.1最小长度及全部边界。短边/越界保留原权威；树与图钉仍发。Finished发真实装饰重定位和MorphFreehandWall历史。

两个系统沿用原 AudioCursorMoved 读取门；编辑音频flag从公开墙+296读取原 source_flag_65 后乘2，不能误用 freehand_height_type。

修正现有两个 decorator 消费者：与core共享事件，删除原版没有的kind；自由墙元素由错误16B{id,segment,t,y}改为真实12B{id,segment,t}。原14081bda4/14081bddb从Info+4取墙空间y；移动140812227保留anchor_ws.y。逐项标记修改再消费，空桶/屋顶项不额外查公开墙，消费者直接借用权威Curve2及其原弧长缓存，最近点采样复用utils的借用算法，不逐装饰分配曲线。

## 验证与剩余边界

原生有限指令oracle：`m01-freehand-native.py/.json`执行原移动标量加法环3组和原邻边阈值6组（精确0.1、过近、负零、大数、NaN），保留输出f32 bits与实际跳转目标。范围明确只覆盖这些原指令，不宣称执行完整原游戏系统。

Rust已加入与该原生fixture的位级/分支差分，以及同一个Bevy World/Schedule跨Started/Held/越界/Finished的真实widget、权威曲线、公开发布、两次历史、保存/恢复完整曲线历史、树事件和卸载测试。

共享ShowPinnedDecoClutter的真实for_wall及for_circle已整合。统一主干编译曾被其他owner的中间接口及磁盘耗尽阻挡；root已清理本轮可重建缓存，统一CARGO_INCREMENTAL=0。同target的v9 country-core自由墙相关10项均通过，包含本轮位级差分、完整多帧World与序列化恢复、既有原fixture的redo/undo。原9组机器码oracle已执行成功。system-decorator自由墙消费者相关3项、移动消费者5项也均通过，其中新增同一真实World把两条UI producer、权威发布、两个decorator消费者、两次墙历史及装饰归档diff/释放完整串起；没有直接注入OnShapeMoved/OnFreehandWallEdited替代UI。root负责正式调度：UI→权威墙发布→装饰消费者→既有deferred history装饰差分收集；未执行OS真实输入，遵守用户暂不验收的约束。真实GPU视觉、游戏reload与共享pins消费待联合验证。

## 可复用经验

先检查结果截断与function body是否可信；PDB函数跨度只是取证边界，不能强迫吞并其他函数。相对跳表必须以原二进制的base+sign_extend(i32)计算；小状态枚举优先原四个目标的override，在独立worker做事务并检查success/timedOut/stdoutTruncated。JPype的地址差值为Java Long，调用getBytes(Address,int)必须转Python int。只读共享库可并发，写入独立副本，不能为恢复一个函数改共享CFG。

验证日志：`/tmp/m01-freehand-tests-v6.log`保留失败/磁盘耗尽事实；`/tmp/m01-freehand-tests-v9.log`记录10项核心通过；后续消费者限定`--lib`，避免重复运行与本链无关的integration targets。

最终验证：

- `CARGO_INCREMENTAL=0 cargo test -p country-core -p system-decorator freehand --offline -- --nocapture`：country-core 10项与system-decorator 3项均通过（日志 `/tmp/m01-freehand-tests-v9.log`）。
- `CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib decorator_on_move_shape --offline -- --nocapture`：5项通过（`/tmp/m01-freehand-move-consumer-tests-v2.log`）。新增同World UI联动先移动整墙并保留装饰WallSpaceCoord.y，再弯折端点后按固定segment/t重新定位装饰；2次历史已产生，真实Archivist提取非空装饰差分并释放Wall/Terrain归档；销毁Schedule/World验证所有权释放。
- `git diff --check`：本轮owner文件通过。

全游戏GPU视觉/Reload由root联合验证；本轮仅声明上述具体业务链与证明范围。circle已接手同原decorator文件中的circle consumer，本轮freehand不改动其后续分段。
