# M01 continue_stairs 原侧指令 oracle（2026-10-27）

本轮职责仅为中央正在恢复的既有 `ui_systems/ui_stairs_misc.rs::continue_stairs` 提供独立原侧行为证据，不修改其生产函数、不复制 Rust 分支当 oracle、不扩完整楼梯工具模块。

## 原始身份与执行范围

原 B 路径为 `skeleton/B/crates/systems/decorator/src/ui_systems/ui_stairs_misc.rs`。主函数 **14079c3a0** 三参数资源是 AppMode、StairsState、DeferredDecoratorOpEvents，原事件步长 **0x88**。本轮直接在 Unicorn 运行身份校验后的 B RNE：

`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`

下列五个业务 helper **均执行原指令**，hook 只观察入口：

- add_node **1408ffd30**：原 checksum/cached flag 清除、调用 GraphMap 和 union-find 的顺序。
- endpoint **1408fe890**：复制实际24字节 endpoint。
- add_edge **1408ffd70**：原相等节点 panic、排序、重复边短路和内部状态清除/容器写入顺序。
- DecoratorLocator::new **140833240**：原16字节地址+u32节点，共写20字节。
- try_set_endpoint **1408fe7b0**：实际执行节点查找、邻接长度判断、写入/失效分支；不是由 Python 推导业务结果。

桥接边界显式列在 JSON：DeferredEvent getter 14067dd80；标准锁 guard instrumentation；GraphMap add_node/add_edge 容器语义（包括物化原 IndexMap 节点 key、邻接长度和边）；IndexMap hash/key lookup；union-find insert 请求记录（本链返回值未使用）；panic 终止。GraphMap/IndexMap 属通用依赖容器，未用 Python 重写 continue_stairs 的 mode、事件、flag、当前 endpoint、degree、locator 分支。慢锁竞争和 Windows unwind 未模拟。

## 覆盖与结论

总计 **324 组**；56组进入业务 helper，230组模式允许但事件为空/全跳过仍标资源 changed，38组模式拒绝，0 panic。原向量不是完整游戏运行场景数。

- Mode：0..21若干边界及u32::MAX，子类型0/63/64，mode高padding非零。原只接受 mode4，或 mode11且 `mode[4]==63`。
- Event：tag0/2/3/4/u32::MAX、高32位padding0/DEADBEEF、类型0/62/63/64、flag0/1/255。原只用 **u32 tag==3** 和 `event[0x81]==63` 过滤。
- 匹配事件总先 add_node，后获取当前 endpoint；只有 endpoint未设或与当前节点相同才跳过 add_edge。
- `flag[0x80]!=0` 不建立新 locator，也不执行 try_set，先前 add_node/add_edge 仍生效。
- 多事件按原顺序遍历，队列内容不 drain、不改写；重复边保留原 wrapper 的短路，后续事件观察上一次 endpoint。
- 同一 endpoint 的 degree0/1 接受，degree2/3 使 endpoint失效；该条件来自原 try_set机器码对真实邻接长度的判断。
- 地址两个word直通，包含高padding；节点u32::MAX保持全位。

发现并交中央修复的差异：**14079c4d9 `cmpl $3,[endpoint]` 只读32位**，不能用原始首u64==3判断未设。故tag3且高32位padding非零也应跳过 add_edge。

另单独核对清除写面：**1408fe880 和 try_set的1408fe85f 都是 `movq $3,[state+10]`**，清除时写满前8字节，不能因为读取32位就改成只写4字节。原读取宽度和写入宽度并不对称。

## Locator 有效位与比较边界

原 new只写前20字节。主函数用同一24字节栈scratch先接 endpoint getter，再构造 locator，所以最后4字节可继承先前 endpoint padding；原 try_set复制整24字节。JSON/TSV保留这些原始字节，但最后4字节不是业务载荷，宿主初始化为零不可因此判差。

Active endpoint 对比前20字节；未设 endpoint只依有效判别合同，不能把未活跃旧payload当业务状态。异常时原写前8B=3u64仍独立核验，不以规范化清空其它16B掩盖差异。

## 复跑与产物

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_continue_stairs_native.py
```

产物：`m01-continue-stairs-native.json` 含全部输入、helper调用序列、容器请求、输出图、原24B endpoint、changed/checksum/cached flag、队列字节、panic；四段 `m01-continue-stairs-{body,endpoint,graph-wrappers,locator}.asm`。

为现有 decorator crate 不新增 serde依赖，另输出 `m01-continue-stairs-native.tsv`。14列以tab分隔：

`name / mode_u32 / mode_type_u8 / mode_padding24 / endpoint_before24_hex / nodes_before_csv / edges_before_a:b_csv / events136_hex_semicolon / endpoint_after24_hex / nodes_after_csv / edges_after_a:b_csv / changed_tick / checksum_word / cached_flag`

空节点/边/事件为`-`，每条事件272个hex字符；多事件用分号。changed_tick初始43981(ABCD)，模式允许后为4660(1234)；checksum初始8608480567731124087(7777777777777777)，匹配事件 add_node 后为0。cached_flag初始1，匹配事件后为0。

中央负责真实 Rust World 差分、生产注册与联合验收；本报告不将纯原侧324组指令执行冒称为 candidate完成或整楼梯模块恢复。
