# Canonical DecoratorLocator 的 ID 所有权与直接生产链合同

范围：修复受影响集合把同墙多个装饰合并成一个键的问题。只写 TG431 与 TG738 候选及两项必要构造 caller；TG719 的两个构造 hunk 由当前唯一 writer decorator_chain 合并。保持 DeferredDecoratorEditHistory 的真实 HashMap 集合、历史 record 顺序及原有复杂业务，禁止扫桶、拿任意 Info.word2c 充当 map ID。

## 权限与冻结

TG431 live owner main-chain-boot-20261018，root 经 lead 明确窄委派，合法 work 新 delegated-core-locator-id-20261021，生产 before 8c2473a78348d85c05e0bcff74bc718b25c3fc62545570bb44b03efd8afd9104。保当前已整合 window_extra，不重写它。

TG738 live NULL，已正常 claim core-locator-id-20261021，同名 delegated 子目录，before 056d3a1e84212477478d531d3c98be88ea469e226f87c337232490b7c64e536d。队列 target C 只是原文件家族标识，本轮绑定的是完整 B 原字节，不据队列宣称 C 行为已验。

TG0f7 external 与 TG122 chimney 均 live NULL，已正常 claim 同 owner；before 分别 f5925e8830e7080ce57a2f3c242fb30451c25c0b6d166affc1aa1d8b65a09dc5 / fa42c06e87e6f67028b2e928fd70be6c9b1e40dd50144325e3b95520c978d53d。只补 DecoratorOp literal 的实际 ID；不展开其旧完整业务。

TG719 owner clutter-startup-20261020 当前另有 decorator_chain 唯一 writer，禁止本 worker 另建整个 cull source candidate；两个 literal 的独立字段由 peer 合并自己当前冻结候选，本 work 只提供精确 ROOT_CULL_OP_ID_HUNKS.patch 供其消费。

## 原值与 key 来源

复用 TG69/scene-clutter-producer-20261020/window-validation-root-contract-v1/locator-required-native-pack。已独立按 RNE f507 的 PE section 映射重验构造器全部15B/5指令与 apply 全1468B/297指令，详见 NATIVE-IDENTITY-AND-BYTES.json。

原 new140833240：RCX 返回区，RDX/R8 写地址两 qword，R9d 写 +10 的 u32 decorator ID。现代 canonical 值为 `DecoratorLocator { pub addr: DecoratorStorageAddr, pub decorator: u32 }`，`new(addr, decorator)`，原有 Eq/Hash 直接包含这两个真实字段。无需复刻24B传输 padding。

place141276f5e 读取 saved+8c，由原 op+48..57 到 stack80 的复制可确定这是原 op+54 的 key；f74/f77 从 op 地址两字写 RDX/R8，f7a 将 key 写 R9d，再 new→原 HashMap.insert。

change1412770cb 读取 saved+158，由 op+60..6f→stack150 可确定是 op+68 的 key。70b4/b7 先选 source addr，70ba 比较原 destination 判别3；只有不等3才在70c0/c8换成目标地址，之后 new→同 Map.insert。remove 分支只 add kind5，无 locator。两个分支均先 add record、再登记 locator、再 storage apply、最后事件；无新默认成功或吞错。

现 Info.word2c 是可修改业务字，不是原 map ID，且 change 已有独立 typed patch.id。不能只给 Locator 加字段后继续从该 Info 猜 key。现代 DecoratorOp 必须新增 `pub decorator_id: DecoratorId` 独立字段；from_change 明确赋 patch.id。typed branch仍直接使用 patch.id，不用 inactive Info、也不按桶找 ID。原112B union完整传输不在本轮重造。

## 所有实际 caller 与最窄变更

当前 crates 内构造 Locator 的实际调用只有3处：write_place_op_history、write_change_op_history、apply_typed_change。前两用独立 op.decorator_id.0；change helper 在 record.add 后用 op.dst.unwrap_or(source) 决定 locator 地址；typed branch用原已求得 target 与 patch.id.0。原 DeferredHistory HashMap 不换类型/owner。

同 apply_op 文件，storage apply 的 key 必须从独立 op.decorator_id 读取，legacy build_op_event 已有 ID 字段也同源；不改变事件原有编码布局、不把旧事件 codec 标成完整恢复。record inactive qword 旧表达式本轮保留，不凭 metadata 猜完整历史载荷。apply_typed_change 原 patch.id、字段修改/事件序保持。

所有实际 DecoratorOp 构造点：

- `DecoratorOp::from_change`：id=patch.id。现 chimney/stairs/lantern typed callers由此自动传递真实 map key，Info.word2c仍可独立变化。
- apply_op 的既有 height_op 测试 helper：id=函数明确参数，不由构造 Info 反推。
- handle_external_decorator_cmds：id=cmd.decorator_id（已有命令字段），构造literal加一行。
- cull_oob_decorators 两处：id=当前函数参数或先前 map/lookup 收集的循环 id，各literal加一行，交peer唯一writer。
- ui_change_chimney_height：id=当前函数用于实际 lookup_info 的 local decorator_id，加一行。**当前 dispatcher 没有 decorator_id typed payload；该 local 仍来自 gated(4) 的 payload，原上游已知 payload0 断点没有因此修好**。本候选只避免又从 Info.word2c猜key，不声称真实烟囱 UI producer 完整。

全 crates 搜索未发现其它 literal 或直接 new call；ui_stairs_misc 只有旧注释提到 Locator，没有构造此 Rust 值；ui_change_chimney_height 的 write_change_op_history 名称仅保活函数引用，不是另一个实际调用。Default 仍是结构已有 trait，不作为本次任何生产 caller 的 key 来源。新增 field 会让漏掉的非 Default literal 编译失败，root统一编译验证。

## 失败、副作用与测试验收

不改变 archivist assert/unwrap、source required lookup、archive→record→locator→patch apply→storage→event 的现时序；typed patch 字段失败发生在 locator 登记后，必须仍能观察已写 history，而不是事务回滚/默认跳过。None destination 沿 source；Some 明确目标。remove 不新增受影响 locator。

拟新增真实调用测试：同墙两个明确 ID 经 write_place helper 得到两个实际集合键，重复同 locator 合并但 records仍逐条；change helper验证 Some目标与None沿源、同id不同addr分别存续；typed apply通过真实 Storage+Archivist+DeferredHistory+Events，Info.word2c故意不同于 mapkey且再执行 word delta，受影响 locator仍是 patch.id，迁移与原地两路实际 key/事件一致；故意无效 extra 触发真实 panic，验证 exact locator 已登记、entry与事件保持原失败侧效应。均不 mock owner，不执行 Cargo；root负责编译与行为执行。

边界：原 place op+54 的真实生产构造链目前生产没有完整对应 caller，本轮只提供明确 ID 类型入口和该 history 注册面的正确 key；不把 absence/default当作完成。legacy chimney/外部事件 codec、完整原112B union、其它 UI 和保存/撤销仍按各 owner 已登记缺口推进。本合同只关闭 canonical identity 的直接必要链。
