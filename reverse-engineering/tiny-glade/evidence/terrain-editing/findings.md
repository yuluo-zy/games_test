# 首批地形编辑取证（2026-10-11）

## 执行与证据范围

本会话仅新增地形专题文件；未修改任何 AGENTS.md、共用依赖政策、项目进度、其他会话成果、原二进制或数据库注释/类型。

通过现有 Ghidra 数据库执行 `-process tiny-glade.exe -readOnly -noanalysis`，实际取得15个函数的反编译与汇编：`B-core/report.json`。数据库身份是 patched B EXE；`native-provenance.json` 对69段函数体逐段核查，全部与原始 B `tiny-glade.rne` 相同。此结论仅是字节/反汇编来源核查，不是行为差分或系统验收。

首次 Java21 启动失败（已缓存脚本 class version70，Java21只支持65），随后使用已安装 Java26 成功，没有修改工具配置。失败输出仍在会话任务记录。工具链/分析器无全程序自动分析。

## 三版来源与依赖筛除

`source-ownership.json` 索引17个地形私有源路径在A/B/C的校验和；这些是游戏业务候选，不是作者源文本。以 `tracking/dependency-exclusions.json` 的已确认来源为基础，std/core/alloc、Bevy事件/ECS、Vec/Arc/HashMap、serde/rkyv/ron、glam等库展开不计为地形业务恢复。

实例：`B-dossier/dossier.json` 显示编辑核心的入口caller是 Bevy 的 FunctionSystem 包装。其泛型参数能提供游戏事件/资源类型证据，但包装/事件存储实现仍是依赖。`delayed_terrain_changed_msg.c` 的 grow_one 标签被自动标成 WriteSettingsFileCmd，不能据此给游戏事件定名；真实 SystemParam 表明对应 EventWriter 是 OnTerrainChange。

## 已从代码与原始指令确认的业务

### 1. stroke点列维护并非普通固定步长重采样

`B-core/140a97880.c:18-63`：
- 无点时写入第一点。
- 一个点时，仅当新点到原点的3D距离大于位常量 `0x3a83126f`（约0.001）才添第二点。
- 两个或更多点时，比较的是当前末两点的距离与传入阈值，而不是直接比较新点到末点距离。
- 若已有末段长度达到阈值且 count<1024，追加新点；若末段短则用新点覆盖末点；count达到1024且追加分支成立时不追加。
- Vec扩容/分配是依赖，恢复的是上述游戏决定。

阈值的生产者、点列坐标/第三分量语义、调用输入限制尚未闭合；不能仅因底层是Vec3认定点列三分量均为世界XYZ。

### 2. stroke新建有明确ID上限

`B-core/140924a10.c:10-28` 与 `.asm.txt:6-38`：资源+0x38计数小于0x3fffff时先加1；随后把寄存器里的计数减1作为插入key。因此本次key是递增前的计数，不能把“更新后的计数”当返回ID。初始payload含一个12字节点、位常量0x3ecccccd与0x40200000（0.4/2.5）以及key，具体字段名仍需消费者对齐。已有值冲突进入panic路径。HashMap/allocator实现排除，不从反编译器缺失局部变量直接恢复真实Rust返回ABI。

### 3. deferred历史消费者会取走pending内容并调用History::add

`B-core/140ac9770.c:18-40`：将pending首字段置为0x8000000000000002的空态，旧值非空时移动有效字段、构造记录并实际调用 History::add。

`B-dossier/dossier.json` 的 FunctionSystem 签名同时包含 History、PendingTerrainHistory 和 DebugHistoryMsg，因此后续事件名称以真实参数为准，不采用伪代码误标的通用Vec泛型名称。

这仅证明 pending消费者，不证明谁填充pending、哪一帧提交、完整history cursor/snapshot事务已恢复。

### 4. replay对reapply/revert的真实CALL已定位

`B-dossier/dossier.json`：replay_terrain在 `0x14099539a` 调用 reapply，在 `0x14099536a` 调用 revert。它们不再是仅靠同名推测的边。New/Edit/Move各variant的字段与判别值仍须对照这两个消费者和真实JSON，不能把persisted index/id自动合并。

### 5. 读回实际更新CPU高度数据与dirty mask

`B-core/1409ea020.c:125-259`，对应原RNE字节已核查：
- 读取资源+0x50/+0x54高度边界；与位常量0xc0200000/0x41c00000不同则写入该常量对（-2.5/24.0），将dirty mask全部设为1。
- GPU来源地址与CPU资源+8指向的数据逐16字节比较；不同才复制并标dirty。
- 64个外层块，每块8段×64组×16字节，总计0x80000（524288）字节，符合512×512×u16大小。
- 每个16字节差异对其64位mask word中的一个bit置位；同bit跨8段累计，每个bit关联8×16=128字节（64个u16样本）。这与8×8样本tile容量一致，但线性/分块布局到世界tile的映射仍须核对SSBO schema，不能仅凭容量确认空间映射。
- 任意值/边界变化才调用 Publisher<DirtyTiles>::publish；mask大小0x200=64×u64。

这证明CPU高度网格在此路径由GPU来源数据更新，且该producer生成dirty通知；尚未证明所有GPU输出都已完成同步或CPU网格是唯一权威态。tabloid的发布容器实现复用，只恢复调用与mask语义。

### 6. 延迟读回与两种高度资源

`B-core/140aabdd0.c:44-98,100-111`：本地倒计时逐项减1；首次到0时取得SSBO映射，调用copy_from_gpu两次（对应游戏高度与视觉高度资源），其间发送OnTerrainChange。调度资源的+8字节被取走/清零，非零时新加值2的倒计时，再移除归零项。

`B-dossier/dossier.json` 签名确认两个独立 ResMut<TerrainHeightsData<TerrainHeightSsbo>> 与 ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>。共享地址在伪代码里都选用了TerrainHeightSsbo别名，不能据此丢掉视觉资源。

`B-core/140aabdc0.c:9-11` 是原子自增；完整调度字段解释仍需其producer、调用参数和内联写入核对，不能只因方法名给锁定字段命名。

### 7. GPU业务契约不是“把u16网格直接用CPU笔刷改掉”

`B-core/14098c680.c:395-443` 实际构建写TerrainHeightSsbo与VisualTerrainHeightSsbo的compute pass。

`B-core/1408c7120.c:723-725,750-752,829` 包含stroke dispatch与两次GaussBlur pass构建。Gaussian blur是内部计算阶段证据，不等于独立“平滑工具”。shader公式、资源绑定含义、dispatch维度、不同stroke重叠顺序尚未取证，暂不实现臆造CPU核。

## 函数边界误差已核查

编辑核心PDB整段4814字节，数据库body是11段共4114字节；`B-original-14094e000.asm.txt` 对完整4814逐字节解码，和数据库同段memory相同。正常body后原始尾部有UD2、padding及保存/恢复寄存器的异常清理funclet，长度差不能直接称为遗漏主业务分支。

仍需审阅不连续body中的所有跳转和异常范围；反编译报告successful并不意味着恢复了Rust签名或已经完成行为验证。其他函数的contiguous body-byte长度不可直接当完整范围长度，完整核查使用B-dossier ranges。

## 下一批按依赖执行

1. 从编辑/UI的事件字段和真实SystemParam恢复命令结构，确认height/falloff操作枚举、no-op、SetTerrainStrokePts语义与音频/重新计算通知。只恢复游戏策略，不恢复事件队列和哈希实现。
2. 读取replay_terrain及reapply/revert，关闭New/Edit/Move、pending producer→History::add；独立核查删除，不能由C animate_terrain_delete名字推出B行为。
3. 核查rasterize/read_terrain_data的shader封装、uniform与资源绑定，确定stroke权威态、计算输出和读回契约；不把.bin当SPIR-V。
4. 沿copy_from_gpu的DirtyTiles producer接已有wall subscriber/republish，再查碰撞、楼梯、树和道路/水的真实消费者及清理。
5. 在上述结构足够明确后建立原指令局部oracle，才恢复Rust业务模块；C/A各自验证，最后snapshot与端到端保存/重放。

## 尚未完成

尚未实现地形Rust模块，未运行原作或业务数值差分，未恢复完整GPU求值、场景、snapshot或C迁移。当前成果是首批实际反编译、字节来源核查与业务发现，系统验收仍not_verified。
