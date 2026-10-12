# G01 原创建、形变、source 删除与 Roof 失效消费者

本轮恢复 B CPU 业务，A/C 仅用于私有模块来源和依赖筛除。[三版来源](three-version-provenance.json)登记原 country-core 的 Circle/Rectangle authoring、InnerWalls、Wall History，以及 system_roof/delete_roof；容器、HashMap/hashbrown、Bevy Query/Event/Commands、Arc、CRT 数学保持库边界。

## 已恢复的实际输入与 API

- `create_building_from_placement` 接受实际 shape、80B ShapePlacement、初始 ID counter、设置以及已解析 Public/Roof context。ShapePlacement+2c!=2 才携带 +20 旧 WallId；+40 原样写 source.extra/+54，+3c 原样写 elevation/+58。这里的额外高度字段由实际 UI 写入与实际创建消费者同时证明，没有直接猜 preview 字节含义。
- ID 使用 InnerWalls+c0 的 wrapping+1。Circle 使用原 `.5` spacing，Rectangle 复用已恢复原 Rectangle2d→Curve2；没有复制几何近似算法。几何先成功，ID 才分配。new_wall_w_params 实际默认 source+5c 为 bits40333333（2.8），+65 special=false、+66 clutter=0；source+30 新建通知与 +60 更新通知区分。
- `select_creation_appearance`：未提供 hint 或 Public 查不到均 settings fallback；已找到 Public 先检查点数>=2/raw length>0，shape>=2 跳过颜色和样式继承。Regular host 读取真实墙/木框 color-ID arrays、Public+299 style，匹配 Roof 则读 +50 color、+40..4c appearance。Roof None 的 inactive payload 规范化零，不能把未初始化 payload 当枚举 DWORD。
- Roof lookup 与 Public/颜色规则独立。实际 prior Roof 即使 Public 不存在/为 freeform，仍覆盖 rim+3d，并仅在 old pitch+30<=.1（有序比较）时覆盖 shape-derived pitch。NaN 不命中该 pitch 条件。Circle/Rectangle preset 的 SineEase/smoothstep 与原常量已逐位比较。
- `edit_rectangle_source` 接受标准 EventReader::last 已选出的64B事件与实际 GameTime+58。重建 raw Curve2，更新 enclosure tag1，found时两次+60通知，保留 height/elevation/path-scalar/style/special/clutter。Missing ID跳过两次状态写入，仍产生 material kind4 与 motion 两条音效请求。Motion velocity=event+34/GameTime+58，flag+38 选择20或200分母后 clamp；x86 invalid DIVSS 的负NaN位已保留。
- `remove_source` 是实际 InnerWalls.remove：found先+90，后std map remove，counter不回退；missing原函数panic，adapter返回typed MissingSource。`clear_preview_for_inactive_mode` 是两UI完整入口的mode/subtype退出分支，只清preview首DWORD、标change tick，不创建ID或事件。
- `process_invalid_roof_notifications` 是实际 RoofIsInvalid→DeleteRoof 消费者。normal路径对未marked且WallId匹配的每个Query row发一次flag0删除事件，duplicate invalid IDs不重复；同时生产实际 `RoofHistoryEdit::delete_from_roof`。reset通知优先，向全部Roof row（含marked）发flag0，不发History mutation。返回mutation必须交给真实配置的ledger，返回值不等于History提交成功。

## 原码完整性与差分

[正式报告](B-authority-sections-verification.json)绑定原 B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。完整 creator、edit、source-remove found、inactive UI、appearance 与 invalid-roof 每次执行都断言实际 RIP==return sentinel。Game geometry、Easing、source ID/default/writes、getcolors、ev_new_wall、History.add、Roof Query 与事件业务指令全部真实执行；不hook游戏输出。

| 样本 | 比较或观察范围 |
|---|---|
| 256 source constructor sections /7513 points | ID、实际source fields、Curve2全points/U/length、+30与三次+60 |
| 512完整Circle/Rectangle creator /15026 points | 256无prior+256真实inherited context；全raw curve、ID/source、颜色/样式、Roof事件有效字段、preset26B；原两条History调用及pending count2 |
| 256成功UI写event | hint条件、shape/+40/+3c原样复制、标准事件counter/tick |
| 1024 source updates | elevation/enclosure/style，含missing与同值仍dirty |
| 256 source removes | 192完整RET；64实际missing panic guard，map和通知状态受控 |
| 128 inactive UI | Circle/Rectangle整函数mode mismatch→RET，preview其余字节保留且tick更新 |
| 512 appearance getter | 整函数RET，regular/freeform/missing与真实Roof Entity/query |
| 512 Rectangle edit +64 empty/read-consumed | 完整RET、旧/新两个buffer的last选择、384found/128missing；found共1920 points；两音效有效字段和metadata保留 |
| 192 invalid-roof消费者 | 整函数RET与Rust对比删除事件有效字段、实际History payload tag+44Bpresence+WallId；marked、duplicate、missing-ID、reset、多row |

Native fixture仅桥接标准库 HashMap insert/remove、HashSet insert、RandomState与CRT/IAT；原map查找与游戏条件仍执行。Roof Entity/Query carriers模拟依赖ABI但不预计算业务输出。每个input、候选源及probe、oracle文件哈希已写正式报告。运行命令：`/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_authoring.py`。

## 修复证据与验收边界

[first-edit-motion-invalid-nan-failure.json](first-edit-motion-invalid-nan-failure.json)保留29个原始差分：ARM主机0/0或Inf/Inf得正NaN，原DIVSS得ffc00000；Rust保持原invalid结果后正式512 edit全部逐位通过。另UI fixture counter初置偏移错误、Optional None payload误当DWORD的失败留在目录，均是oracle投影/fixture修正，不是候选业务差分。

本轮正有限 shape几何样本已验收；高度/地面scalar允许原bit模式（含NaN/Inf），不是强加finite值。typed无效Geometry和invalid prior Public guard为adapter边界：原已插入source后才在getcolors中panic，而Rust先拒绝坏host；不声明原panic之前的部分mutation一致。源+67/+69及库指针/容量/闲置union padding不在RawPublication API全字段声明中，+69原global零时为5、否则0，后续Wall History恢复必须明确携带这一来源。

原创建默认关闭settings+38的随机颜色variation；其开启分支尚未恢复。原Wall History ev_new_wall实际执行、两pending记录被检查，但264B Wall snapshot与ConcurrentEdits组/完整replay仍需G14恢复；不能把Roof-only history伪装为创建复合事务。成功UI事件之前的click/border/active drag-cancel未验收，当前只证明mode-exit取消。三版完整行为等价、实际render/framebuffer/GPU与场景不在本轮范围。父任务负责真实World集成及独立review，当前编译通过不表示完整建筑accepted。

## 冻结输入

Raw source64 schema 增加后，本报告已实际重新执行，创建确认0、编辑保留输入字节。正式报告 SHA256：`7a9d2e280205dc004b8c1dd5e4d15bc58e58be772ea883044f692e5dc2acc7fd`。

- `reconstruction/building/roof_creation.rs`：`46e7029d434d88044b7d669b21bcc07918cc51ad43f27285bb16fc5772c716bc`
- `reconstruction/roof/parameters.rs`：`61bc696ffd9dc30eacdfd2739e5edff574c6db22b8ebb6356017d6d0eb371fcf`
- `reconstruction/building/roof_invalid_notifications.rs`：`9f22fa037a0e7d59ad4aa587dbf1b5e1003f45caf6309d958122f16c2771dce2`
- `reconstruction/history/building_transactions.rs`：`0332fbcacf046896290a43720775b8c6ca9dfd72c0ebe560932ec65f62dde9af`
- `reconstruction/building/wall_authoring.rs`：`2043afa2ba091e57d9368a5b2b88acc0803b510fd2d2ea8671e95b9714c7c2bb`
- `reconstruction/src/bin/wall-authoring-probe.rs`：`5e3b2eb3a2574004ebb1155c35c8b4b3791a4179076b455ad676d66b6211d989`
- `reconstruction/building/wall_freeform_cache.rs`：`7d2a2870be3cfd73e4318ce4fb3d240b17f59ac2f3738d5504d11dc7c6b637a6`
- `reconstruction/building/wall_collection.rs`：`b74c6aa577a9c012d4d34fcfb1851a8d33a84ee154872ece077d66e96ab198b6`
- `reconstruction/building/wall_state.rs`：`3ac4555841032f8347e1d28da60be7260eead44ae7408c8de265169ce269cf4d`
- `reconstruction/roof/surface.rs`：`132b17182082410908cae6a0d897638eeebb0e8095183d124767ec886428bbcd`
- `reconstruction/Cargo.lock`：`2df647ccdbfef544cb7e8f9aa1cd499f411dbfca76cda7e86bcfe051a4095bcc`
- release probe：`d8e7ced77e03f20f8033ee81ac8146d30d5d422b91301555a74e85528dcc5523`
- `scripts/multi-version/verify_wall_authoring.py`：`ba6dbaf8075fcabe5302e66e77e14e7ac1cad83a53027e4c82c1e6cc17ee1a36`
- `evidence/building-authoring/native_authoring.py`：`0caa5dc2889b2cb18c5fda8c938d27a1246dca99621ff183650d46a349853aeb`
- `evidence/building-constructor/native_fixture.py`：`3a1d9654cbb346ed458185543ea30778b147e2b658a0301cd59e5ea9b73eec1e`
