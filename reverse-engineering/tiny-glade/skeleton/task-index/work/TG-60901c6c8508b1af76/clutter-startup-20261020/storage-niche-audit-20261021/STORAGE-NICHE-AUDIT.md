# 装饰存储 Option niche 只读审计（B，2026-10-21）

本交付只分析 flags→storage 已定位阻点，不写 TG889 或 restore 的候选，不修改 claim、生产、Cargo。生产输入逐文件冻结于 `source-inputs.json`，检查结束三源均与输入一致。本文不是运行验收；不将依赖 `next` 的复用计作游戏恢复。

## 结论

1. **归档恢复判断对象明确错误。** 原 `restore_archived_decorators` 并未直接 call `14155afe0`；它内联相同的移除 removed 标记→标记 modified→`HashMap::insert`→按返回旧 `Option<DecoratorInfo>` 更新 d2s 的业务链。当前 `overwrite_decorator_state_from_archivist` 丢弃 `bucket.insert` 的返回旧值，转而判断新 `decorator_info.state == 2`。应按返回旧值 None/Some 选择原 d2s insert/update。输入记录的 +47 不决定该选择。
2. **merge、tracked clone、wall split 三处的 `2` 比较属于 owned iterator 的 `Option<(DecoratorId, DecoratorInfo)>` 匹配，不是已证明的“live state=2 则业务提前退出”规则。** 这一判断有原 PDB 的具体 `IntoIter::next` / `Drain::next` 内联链支持，非按形似套用。原机器码确实在选中 bucket 后读取其 Info+47 并比较2，不能隐去这一事实；该字节复用了载荷的 niche。现代 Rust 的 `for` 已完成 next 的 Some/None 匹配，不能再把 None 的 niche 常量建模成有效条目的业务状态。
3. **类型边界必须同时纠正。** 对原合法的记录，Some 载荷的 +47 不是 None niche 2；当前手写结构将其声明为任意 u8，并在上游强制写2，使原本不可能的载荷变成可进入 map 的“live记录”，随后又被三个复制出来的 break 截断。owner 应配套恢复实际 flags/输入合同。若向原 ABI 人工注入字节2，它的机器码也会提前退出；这不构成“有效 live=2”业务证据，不应给这种非法载荷补造原行为保证。

## 原身份与有界范围

- RNE `/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne`，SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- PDB `/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb`，SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`；GUID `09F1A02A-49F2-F8C8-4C4C-44205044422E`，age1。
- 已存三 loop 完整 decomp 位于 `evidence/country-core-deep/wall/decomp`；未找到相应完整 asm 后，按已存 PDB procedure-index 精确范围补了5个 bounded packs，共9844字节，无程序/全PDB重扫。`native-pack/report.json` 与 `inline-pack/report.json` 均 status=ok，每请求ok，每条指令与原文件字节一致；仅有限入口/branch点做 source/inline symbolization。
- 三loop完整大小/原 procedure：merge `14155b170/2231B`（1618:37952）；tracked clone d2s `141256e40/1381B`（1488:264）；tracked clone unit `1412573b0/1349B`（1488:8400）；wall split `140a1f590/4311B`（540:79480）。依赖 insert `1411fbfa0/572B` 的具体 `HashMap<DecoratorId,DecoratorInfo,RandomState>` 名称见 `pdb-procedures.json`。
- 归档恢复复用已存 `evidence/round12/archival-owner/restore-original.asm`，原函数 `140817a40/3967B`（460:264）。本次将该文件全部891条指令重新逐字节对照 RNE，891/891匹配，详见 `restore-asm-byte-verification.json`，未重取整函数。

## 插入返回 ABI：先区分输入、输出、外层 API

`14155afe0` 的已存完整 asm 在：
`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-decorator-info/B-storage-consumer/14155afe0.asm.txt`。

- 入口 `14155afef` 保存 R9 为输入记录，`14155aff8` 保存 RCX 为返回缓冲。
- `14155b03b` 将 RCX 设为 rsp+a0 的局部返回缓冲，`14155b046` 将 R9 设为原输入，`14155b049` 调用 `1411fbfa0`。
- `14155b04e` 比较的是局部**返回** `rsp+a0+47`，不是输入 R9+47。
- 返回旧 None：走 d2s insert，`14155b076` 写2的是调用者返回缓冲 RDI+47；不写输入、也不把新 map 条目的 +47 改2。
- 返回旧 Some：把旧记录搬到调用者返回缓冲，再调用 d2s update。
- 依赖 insert 的独立完整原码再次确认：`1411fc0a8..cc` 把旧 map 值复制到原 RCX 返回缓冲；`1411fc0cf..f2` 另将输入记录写入 map。缺旧值分支 `1411fc191` 只向输出 RSI+47 写2，随后返回输出地址。

**外层 apply place 是另一层。** 其已存 `B-producers/140a20c70.asm.txt` 中 `140a20cd8` 把入口 R8B 写入新 Info+47，`140a20ceb` 写独立 +45 config 字节；`140a20d27` 为 inner 准备独立返回区 rsp+98，`140a20d3a` 调用 inner 后直接返回。不能将 inner 返回 Option 推成外层 API 也必须返回 Option；本合同不变更现代外层接口，且不将 R8B 字节写入单独冒充已恢复了 bool 的业务命名。

## 三个循环：原 next 类型、字节来源和退出位置

| 当前源码位置 | 原 next 与实际 owner | 原计数/结束门 | niche 比较及退出 |
|---|---|---|---|
| decorator_storage.rs merge_with:278 | std HashMap IntoIter→hashbrown IntoIter，源 owned map | `14155b53d` remaining减1；借位下溢退出到 `14155b75e` | `14155b59a` 从bucket末字节读入AL，`14155b5a0` 比2，等于退出同一 `14155b75e` |
| tracked_clone_from:322 | std HashMap Drain→hashbrown RawDrain，目标旧map | `141257020` remaining减1；借位下溢退出到 `141257094` | `141257077` 比bucket末字节2，等于退出 `141257094`；否则 `14125708b` 调真实d2s remove |
| unit泛型对应同源循环 | 同 tracked clone 原源行124，独立实例 | `141257570` remaining减1 | `1412575c8` 比2，等于退出 `1412575e2`，否则 `1412575d8` 调unit callback |
| on_wall_split:822 | std HashMap IntoIter→hashbrown IntoIter，被移除墙桶的 owned map | `140a1faac` remaining==0退出 `140a1f9d3` | `140a1fb08` 读bucket末字节到CL，`140a1fb0d` 比2，等于退出同一 `140a1f9d3` |

每个 bucket 步长0x50：键位于bucket末端-0x50，Info从-0x48开始，末端-1即Info+47。**原码确实读取载荷字节**；不是一个独立存储的“迭代状态字段”。匹配 Some 后才能安全地把它解释成有效记录。当前源的额外 `if info.state == 2 { break; }` 将这一 ABI discriminant 重复变成业务条件。

关键 inline 逐项保留在 `iterator-abi-evidence.json`：

- `14155b53d`：RawIter::next(raw3664)→RawIntoIter::next(raw3888)→map IntoIter::next(map3276)→std map next(std2475)→游戏merge原行259。
- `14155b59a`：仍处于 RawIntoIter / map / std next 内联范围；比较 `14155b5a0` 对应同一游戏for原行259。
- `141257020`：RawIter::next(raw3664)→RawDrain::next(raw3960)→map Drain::next(map3438)→std map next(std2713)→游戏tracked clone原行124。比较 `141257077` 对应同一游戏for原行124。
- `140a1faac`、`140a1fb08`：RawIntoIter→map IntoIter→std map next→游戏split原行871。

本地公开 `hashbrown0.15.2` 与这些原行精确相符，正文摘录及整文件SHA在 `dependency-next-source.json`：IntoIter::next 返回 `Option<(K,V)>` 并委托 inner.next；RawIntoIter返回 `Some(self.iter.next()?.read())`；Drain同样返回Option并读出item。此处直接复用依赖，未恢复容器算法、未声称整份std vendor/fork全身份已验证。

## 归档恢复：是同链内联，并非直接复用函数地址

原函数全部 direct call 中没有 `14155afe0`。确切业务段为：

1. `14081821b` removed-id remove；`14081822a` modified-id insert。
2. `140818234` 令RCX=rsp+50（返回旧 Option）；`140818239`令RDX=真实map；`140818241`令R9=rsp+f0（新输入Info）。
3. `140818249` 调 `1411fbfa0`。
4. `14081824e` 比较 rsp+97，即**输出** rsp+50+47。它不比较输入rsp+f0+47（rsp+137）。
5. `140818262` 返回旧 Some 则跳 `140817ff0`，`140817ff7` 调 `140a1b5a0`（update）；返回旧 None 则 `14081826f` 调 `140a1b4e0`（insert）。

当前 helper:147 的 `bucket.insert(..)` 返回 Option 已存在且可直接复用，却被丢弃；148判断新 info.state。最窄必要修复由 owner 在当前架构内保存返回旧值，按 old.is_none()/Some 分支选择d2s即可。不得因为修这一 helper 顺便将上层 restore ECS未闭合部分标成完成。

## owner 可执行修复/验证边界（本工未写候选）

- 保留唯一 storage map、唯一 d2s；不可建立另一套 flags/state registry。
- 普通 for 负责迭代结束，Info迁移为原实际 flags 合同，禁止将 raw niche=2 当新有效业务状态。现 `on_wall_split_skips_state2_live_decorators` 测试在验证这种错误模型，应由owner随类型合同修正，不应以保留旧测试为由保存错误规则。
- `merge_with` 原 owned `IntoIter`、tracked clone原 `Drain` 是明确生命周期事实；当前 merge接受 `&Decorators`，属于现代借用适配。未在本范围审完其全部调用者，不能仅凭接口形状断言现代借用适配必错；如要追所有权/失败时清空副作用，须由当前owner协调同链消费者，不在本报告扩改。
- 至少验证 fresh insert 的 d2s insert、replace 的 d2s update（新 flags为0/1均应按旧值决定）；多个合法0/1记录完整迁移/重索引；空集结束；clone旧项移除与新项登记；split后旧墙消失/新墙与d2s一致。不要把原无效 niche字节2作为正常live fixture。
- 原 d2s冲突/缺失 panic 与修改顺序仍要保留；不将“无额外break”变成缺失索引自动成功。
- 本工未运行Cargo/native target/GPU。只有原字节/metadata与现源只读审计事实；候选与完整真实UI→flags→storage→derive→visual链仍须root/owner实现并验收。

## 尚未越证的事项

+47原字段确切业务名称、+45原flags完整策略及所有创建/加载/回放入口由lead/root当前flags闭包负责；本报告不凭字节位置发明名称。人工raw字节2的native行为不等于合法Rust值。merge现代借用边界/泛型hook完整panic时副作用未独立恢复。其他restore历史守卫、碰撞消费者和尾部diff未在这次窄审范围内。
