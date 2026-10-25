# 本轮原证驱动恢复经验增量

这些是本轮具体反例的方法记录，不是业务恢复完成量，也不替代 frozen evidence、独立审查或真实运行结果。

1. **差分跳过域要沿完整 CFG 核对。** same-Arc / fullEq 只跳过原 fullvisual 的实体重建，之前的 WallHoles、Window candidates、ClothInfo 仍每帧输出。把高层“没变化”扩大为不执行整个系统会丢失真正逐帧数据。Window V1 的 changed-only 设计已保留失效，V2 才接受。
2. **同名方法不能当函数身份。** 当前 tracking helper 误套 destructive clear 的原地址，清掉 authority；原真正 tracking cleanup 保留 authority，清两集合和 removed registry，且在 validate 入口执行。修正方法时要追完依赖旧行为的 restore 调用者，不能只改方法体。
3. **原共享资源默认空不等于可以空资源接线。** Window 原 Vec 构造确实为空，但只有 canonical producer→collector→同 owner drain 真正连接后才有业务；初始化类型本身不算窗口业务完成。
4. **参数类型和相等策略要一起恢复。** 原 builder/dormer/assembly payload 中混合 ordered floats、整数和tag选定字段。原 raw words/bytes 是承载，不证明 memcmp 正确；NaN、signed zero、忽略字段均可能改变实体 diff。
5. **机器写入的目标先核 ABI。** 返回缓冲区里的 Option niche/tag 写入不能被命名成输入对象的业务状态；应追 RCX/RDX、sret 保存和被调用函数的写入目标。本轮 state2 解释正在有界原证复核，尚未据此改 foreign-owned 源码。
6. **实际初始路径先查，避免另造桥。** 当前已有 PreStartup RON→同 Atlas/MeshLibrary→startup_nani/GPU。CPU clutter catalog 不是另一个 Atlas 输入。相邻资产文件需要发布身份绑定；原字符串/kind 与 sibling RON 不符时不能直接修改原算法。
7. **调度边界就是业务合同。** 原两个碰撞阶段、中间 validate/archive、member14 nested chain_ignore_deferred 与最后 copy 不得合成自定直线。注册函数存在也要找实际 App caller，旧shim没有被调用就不能算接入。
8. **验证工具自身也有输入。** Cargo workspace 实为 edition2024，检查器误传2021造成6个let-chain解析失败。保留v5失败，v6读取实际Cargo后重验；没有为了检查器修改源码。语法成功仍不能证明上层typecheck或运行。
9. **冻结证据不能被整合状态覆盖。** before/candidate/review全哈希通过后才修改生产；整合观察写另一个事件。Cargo重建会覆盖同名测试二进制，运行后要立即保留独立副本，不能用新的二进制回填旧结果。

详细状态与有界实际验证见 [CORE-CHAIN-CONTINUATION.md](CORE-CHAIN-CONTINUATION.md)。


## 后续数值与调度核验补充

- `chain()` 与 `chain_ignore_deferred()` 属业务可见性合同。不能只调整函数列表后就声称原顺序恢复；Commands 在 manager 与 animation 之间是否应用会改变首帧动画和删除行为。原 enum 字节要保留进制，raw 0x24 对应现离散标号36，不能按文本24映射。
- World 中存在 Events<T> 仅说明可读写，不证明 App 维护其更新与清理生命周期。登记实际事件拥有者和 App 注册入口分别核查，不用空资源弥补未注册系统。
- 原常量按每个实际指令操作数的原 PE bytes 核实；一个看似合理的 +1.6 与真实 −0.4 会直接改变烟囱实体几何。旧推断冻结物保留，用强制 addendum 纠正，不能静默改写原证。
- 修复 float 算法必须同时审极端值的组合消费者。恢复 f32 舍入后，旧 f64 增强测试可能不再是目标合同；应保原 case 和历史断言，明确原行为预期及未验证组合，不能扩大容差、ignore 或拆 fixture 后掩盖组合变化。
- 独立源码测试模板在 Cargo 前复制到 runtime，真实执行后立即复制 binary，并保存实际锁裁剪对照。共享数学7项已按此执行；该事实只支持所含原样源码切片，不外推完整缺源包或游戏运行。


## 2026-10-20 04:35：当前源、实际编译与栈槽生命周期

- parse成功不能替代模块可见性与类型检查。WallSpace新测试的错误import由实际编译发现，保留V4红结果，仅修测试import后56/59/63测试逐步实跑；砖宿主跨crate导入cfg(test)设备也在独审被拒，V2只修测试边界。当前缺源依然阻碍整core编译，不能拿旧rlib宣布通过。
- 原机器码必须追最后一次定义。Window新反例误把同一栈槽早期值跨中间覆写连接到后续append；独立复核完整body和连续PC后撤回，旧冻结物保留，未据错误反例改正确源码。生产者每个写入分别标最后定义与合流，不能只凭字段名或相邻hash连值。
- 原失败与类型身份属于业务。窗灯真实Query被误认成PublicWalls双表，导入expf被猜成sin，会产生结构近似但行为错误。先用PDB具体泛型wrapper锁输入类型，再用原字节/导入/常量核标量算法，最后迁移同一canonical组件到生产者与消费者。
- 完成单位仍是实际consumer闭包；必要共享叶接口可以先整合，但必须同时列未迁移caller和未跑测试。文件数、工具成功、native字节数均不计为主链完成。
