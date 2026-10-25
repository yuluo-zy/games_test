# 当前 rhapsody workers 的执行指导

2026-10-12。由用户明确要求发送给当前正在执行的agent。适用目标与依赖边界沿PROJECT_MEMORY；本指导落实WORKFLOW_RESET，覆盖第六批README中“新增独立crate”的后续安排。不删除、不覆盖正在工作的R8文件；已获得证据仍保持其原范围。

## 立刻执行：主流程整合owner

1. 把本文件全文或对应角色段发送给你自己的 sequence_finish、queue_finish、pipeline_descriptors workers。记录实际agent名和收到确认；完成的worker复用现有结果，不为换流程重做原码实验。
2. 等正在执行的工具调用到安全边界，保存原结果；告知worker停止case扩张和另建crate。不要强杀持久数据库会话或丢弃未保存证据。
3. 唯一共享入口沿用 `reconstruction/render/lib.rs` 和 `Cargo.toml`；由你独占编辑。本指导不是要求马上把所有work-packages物理搬走。为避免旧ResourceInfo候选反向依赖render造成Cargo cycle，必要时在共享crate内部迁移其已恢复源码，或采用明确无环的单workspace基础模块；先画依赖边，再改manifest。
4. 主流程独占共享 `runtime` 模块、核心类型、manifest/lock、公开入口、链测试、PROJECT_MEMORY/tracking/进度。worker可提供已有模块及迁移patch给owner；未经明确授予具体文件不要多人改核心类型。此指导发送方只写本文，不并行改你的Rust/台账。
5. 在写代码前确定第一条入口的最小合同：输入graph资源/pass/export声明、原版本依赖classifier/API、外部resource/command/Vulkan后端、用户callback；不能把compiled snapshot、手填first/last access、sequence7、预造VkSubmitInfo作为输入。

## 主流程第一条交付链

`graph声明 → ResourceInfo/Registry → compiled批次/访问链 → recorder → DeviceCounters产生sequence → queue提交数组 → API调用参数/receipt`

入口名由owner结合当前代码选择，不需要另建命名框架。共享入口必须从真实compiled字段取得资源kind/initial/reused、first/last访问、pass范围和export目标，不能调用R7 fixture()作为生产数据来源。

sequence-runtime已有Command到R7 record_command的adapter可作为迁移来源；实际序号须来自counter producer，无+1/-1翻译。R8queue接收这些命令，不重新构造人工sequence。queue的receipt再反馈真实pending/completion消费者。可以分两步交付：先graph→record→API submit输出，随后completion/retire跨帧；每步都明确未闭合边界。

首个原作graph compile构图闭包仍未恢复时，GraphDeclaration是重建入口，不能宣称恢复原完整compile ABI。frame caller接线、CPU候选链和原整链oracle分别记录。

## Worker：sequence_finish / sequence-runtime

**目标consumer：** 共享record和queue submission/completion，直接消费原counter产生的command。

- 先复核你已获得的R8接口与证据，提交owner可调用的begin、track、drop/waste、receipt→pending、complete→command pool/resource drain API；不要继续增加908等案例总数。
- Queue、Sequence、Timeline、Command语义定义由owner确定唯一来源；你返回字段要求/不变量和迁移patch。raw88字节用于原ABI/oracle桥，不要求owner把全生产状态都做成byte数组。
- begin成功只增counter一次；已经begin的command不得重复增counter。Sequence与Timeline类型分离。保留全局u64 bucket替换后断言的原作用顺序，不人为修复跨队列碰撞。
- 如果graph/record resource handle与ControlBlock不相容，交付具体转换前后字段及原producer/consumer证据，不能用额外BTreeMap影子资源池悄悄绕过。
- **验收：** 主流程调用counter→begin→record→submit→receipt→complete，证明相同序号贯穿；completion不足不回收，达标CB回池，sequence gap只阻塞bucket drain。旧leaf oracle只对改动影响重跑。
- **退出条件：** owner能直接编译并调用接口；指出仍存在的private边界和真实resource追踪接线。仅你自己的crate绿测试不算交付完成。

## Worker：queue_finish / queue-submit

**目标consumer：** 共享recorder的真实待提交commands和等待资源，输出原queue的Vulkan参数及receipt。

- 输入来自sequence/record的command状态、queue identity及executor waits/signals，不接受人工构造的VkSubmitInfo数组作为入口。
- 复用你已恢复的queue前段/后段和实际接口；把private数组组装放进可合并模块，ash0.38 API调用为明确外部边界。
- typed API需保留take/部分失败副作用、最后命令end、queue ownership、timeline递增、wait/signal顺序及receipt回写。Unknown必须typed gap或显式不支持，不用默认值补造。
- 向owner交付可调用接口、所有权（成功/错误后的命令归谁）、事件字段、descriptor/sequence接口依赖和迁移patch。停止另建“integration-r9”或新的独立oracle项目。
- **验收：** compiled→record产出的命令原样传入，API捕获能观察command handle/sequence关联、wait/signal/timeline，以及失败时无伪成功receipt。从共享入口执行，不能仅重放queue函数自身fixture。
- **退出条件：** owner能通过同一入口调用并反馈receipt到sequence runtime；Vulkan未实际运行仍保持CPU/API合同范围。

## Worker：pipeline_descriptors / descriptor-binding

**目标consumer：** 真实pipeline/record callback对shader reflection和资源句柄的绑定使用。

- 保留你已有sparse binding、descriptor writes证据，停止重新反射404/638库存、扩大声明adapter案例。
- 交付reflection producer→BindingIndex→layout/pool→descriptor write/update→bind的最小完整源码闭包与consumer入口。必须保留set/binding/type/count、push constant、image layout、buffer range、immutable sampler来源；已观察字段与推断分开。
- 不把enum tag/描述符类型兼容当成shader→pass关联。真实shader/reflection选择producer缺失时明确阻塞，并只围绕这个producer取证。
- 第一链暂不需要的raster、所有sampler分支和缓存策略留到下一完整闭包，不为抢case扩张；不能从整个rhapsody最终范围删除它们。
- 向owner提供绑定后端trait或已有API以及可合并文件，核心handle类型先对齐。由owner安排具体新共享文件独占写入，避免你和queue同改lib/Cargo。
- **验收：** 一个实际消费接口从reflection+真实资源映射产生write与bind参数，能进入共享record callback。缺真实game pass时只报告framework consumer，不声称country-core已绑定。

## 验证与报告规则

先做少量能暴露跨模块错误的共享入口场景：非空buffer/image、相邻read/write、import/export、callback失败、队列失败与completion。输入应来自graph声明或实际consumer，不手填内部状态。未改动旧oracle以其source/dependency/input hash复用；新歧义才运行针对性原码差分。失败时缩减整链输入定位断点，不反射全部库存/重跑所有case作为默认动作。

每个worker汇报只需以下信息，证据细节引用原文件：

1. 已接通consumer的源码文件与公开入口。
2. 移除的私有stub/固定fixture（列前后数据来源）。
3. 外部依赖/API边界仍有哪些；未接通private边界另列。
4. 共享入口执行结果、确实对应的原侧验证范围。
5. 一个当前阻塞和所需证据；取证/实现/验证/整合耗时若可记录则记录，没有数据不要补造。

主流程必须用自己的共享入口调用worker代码并做最终验收；报告一致性和候选case数只是辅助。不要通过放松原错误副作用或隐藏private stubs让链测试变绿。

## 本轮完成定义

至少交付共享render入口中的一条实际可调用链，移除手填record访问字段和sequence输入；queue前段接通或明确typed gap；不新增独立candidate/integration crate。候选CPU运行、原侧API/oracle覆盖、live Vulkan和game-pass画面四种状态分别标记。若没达到，报告具体阻塞，不用新批次成功数替代。
