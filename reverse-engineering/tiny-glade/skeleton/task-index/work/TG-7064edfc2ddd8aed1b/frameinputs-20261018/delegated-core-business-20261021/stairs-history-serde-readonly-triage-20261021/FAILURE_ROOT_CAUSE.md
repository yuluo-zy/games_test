# StairsHistoryEdit serde CPU 失败只读分诊

结论：**SupportsDelete 的旧测试期望错误，当前 tuple 输出与原完整serializer及实际发行历史一致。最窄方向是只改该测试的一个JSON期望，不改业务serde或图/历史算法。** 本轮无source候选、无新claim、无Cargo运行。

## 失败与实际测试

`/private/tmp/tg-business17-country_core-tests-20261021.log` 第3373..3378行显示：当前输出 `{"SupportsDelete":{"edge":[7,8]}}`，旧期望 `{"SupportsDelete":{"edge":{"a":7,"b":8}}}`。source `systems/history/tests.rs` before SHA `1d53ea0e05ff6b3398d248b87c9a785d095ef06e9cac3a1100141a5398898905`。

该同一测试上半已经要求 SupportsRecreate 的edge=[3,4]，且注明原发行历史为tuple数组；下半仅SupportsDelete保留旧struct `{a,b}`期望。不能因此让共享的StairGraphEdge按历史变体输出不同wire shape。建议root必要修复仅将下半期望变成 `{"SupportsDelete":{"edge":[7,8]}}`，继续保留原断言/上半roundtrip；若加delete roundtrip也应复用同真实类型，不另造兼容对象格式。

## 原完整serde与实际资产双证

复用已存在的EditType original-wire caller定位，PDB锁定原StairsHistoryEdit serialize140259af0/820B；原函数此前仅有调用侧pack，本次有界收集该820B，不扫新family/不启动或修改Ghidra worker。原RNE f507/PDB2d9686身份，native-pack/report ok；187条指令、全部820字节已独立连续核原PE bytes，无gap，见NATIVE-SERDE-RECHECK.json。

原9b03读variant bit0；recreate9b2b用SupportsRecreate字符串，delete9c25用SupportsDelete；原常量1429a4c82/4c90/4c94已实际读出SupportsDelete/edge/SupportsRecreate。两分支均把payload指针移到enum+4，分别在140259baa/140259ca4调用**同一个**edge serialize_field callee1404bd820，传相同edge键和同两u32域，未把variant传给edge serializer。不存在delete专用`{a,b}`对象形状分支。原调用侧EditType::Stairs还真实调用该serializer，不是孤立未消费函数。

直接读取原RNE同目录的发行资产，不依赖统计报告：Version27的full/28-wizard-tower/history.json同时含SupportsDelete edge[10,11]、edge[11,12]，以及SupportsRecreate edge[11,12]；full/10-circle-ruin/history.json同样两变体都是数组。SHIPPED-HISTORY-EVIDENCE.json保文件SHA和准确JSON pointer/最小载荷，两个原文件共7项相关编辑全部是两个u32数组。此为实际wire证据，不把缓存报告当原资料。

## 当前生产消费链

当前resources/stairs/stairs_state.rs的StairGraphEdge是语义具名字段a/b，但自定义Serialize明确委托 `(a,b)`，Deserialize读同tuple后沿canonical new。历史enum的两个变体都复用同EdgeEntry{edge:StairGraphEdge}，因此当前输出数组正符合原证；具名Rust字段不决定原wire是对象。

History::add_decorator_diff真实两循环都从真实stairs diff的(a,b)构建StairGraphEdge：命中集合登记SupportsDelete，移除命中登记SupportsRecreate。StairsReplayCmd::expanded两变体都用edge.nodes()产生单边命令，仅recreate位决定回放record/erase；无对象shape特殊分支。修改共享serializer反而会破坏两条生产消费链以及现已正确的recreate断言/发行存档读取。

这里只审本次tuple/object失败，不据此给Malformed JSON、多entry访问顺序或不规范逆序边的所有反序列化行为背书；这些边界没有为修fixture而被改写或掩盖。

## owner与交付边界

live TG706 owner frameinputs-20261018保留；StairsHistoryEdit TG4e4c与replay TG154均NULL，本worker没有claim或写它们候选。现代tests.rs不在原PDB source family任务中，本轮也不修改。所有源码当前SHA/失败logSHA在AUDIT-MANIFEST.json。root/lead可据原证安排只修测试期望后定向重跑；无理由为此次失败改变Serde业务实现。
