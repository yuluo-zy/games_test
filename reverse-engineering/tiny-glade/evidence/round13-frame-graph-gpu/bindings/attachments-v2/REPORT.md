# Attachment view / clear / preserving 候选 v2

状态：`implemented_unverified`。仅 evidence 下候选有改动，root 统一合入与 Cargo/GPU 验证。基线为 bindings v1 已冻结候选；原 manifest / patch 不覆写。五个目标文件与当前生产源码 LF 归一内容匹配，`git apply --check --ignore-space-change` 和 rustfmt 成功。原 RNE 静态六个 PDB 完整范围核验成功，不是原作执行或新 Rust 测试通过。

## 已裁决的原业务语义

所谓 `ColorViewWords` 16 B **是 VkClearColorValue 位模式，不是子资源视图**。旧名为早期复原误命名。

- `from_iter @ 0x14157b150`（2015 B）在 `0x14157b1d0` 从 tag1 ColorAttachment +4 搬 16 B；`0x14157b29b` 写 clear presence；`0x14157b4f8` 将位模式放到 RasterPassColorAttachment +0x40。
- 同函数 `0x14157b27d` 仅调用 `TextureViewDesc::level_count(Some(1))`。没有 layer_count(1)；颜色附件不能凭猜测缩成一层。view 的 base_mip=0，其余数组层保持原默认。
- `RenderPassApi::begin_render_pass @ 0x1416f6e90`（2483 B），`0x1416f719b` 测 +0x3c clear presence；`0x1416f71a9` 取原 +0x40 的 16 B；`0x1416f71db` 放入 clear 数组。`0x1416f7506..751f` 将其放入 VkRenderPassBeginInfo.pClearValues 后调用设备 +0x418 的 vkCmdBeginRenderPass。中间不存在尺寸、layer 或格式解释。
- `RasterPass::render` 的实际 callback `0x141697ab0`（11166 B）在 `0x141697b97` 直接调用 begin_render_pass，保持此 attachment 数组。
- 深度 `new_impl @ 0x140867970`（v1 全长已保存）：`0x140867b39 / b5f / b7c` 及写分支 `0x140867c83 / ca9 / cc6` 依次设置 level_count(1)、layer_count(1)、base_layer(caller)。v2 另核三个方法完整原码与 PDB 身份。

原 RNE / patched B 的完整 SHA、六个完整函数字节与 LLVM asm 见 `original-evidence.json`、各 `.bin/.asm`。所有范围在两发行输入逐字节相同。复用保留的 `evidence/country-core-deep/rhapsody/domains/rg/decomp/decomp-0x1416f6e90.txt` 只作辅助解释，以上判断均核了新保存的完整原指令。

当前非空消费者已有明确反例：`transparency_pass/surface_ice.rs` 的 ice roughness attachment 使用 `[0x3f80_0000; 4]`（四个 1.0），旧 host 无条件改成 `[0.0;4]`，原清除值永远丢失。候选按位恢复真实 payload。

## 候选改动及失败路径

`raster_attachment_accesses` 构造一份 typed view：color mip0/level_count1/默认剩余layers；depth mip0/level_count1/base_layer指定/layer_count1。图 range 经现有 `compute_view_range` 投影；record 用同一份 view 经 `rh_view_desc` 创建真实物理 attachment view，不再各自猜范围。

与 native 初始 attachment access 的 resource/access/group=1 逐项核对。非法颜色 tag、错误资源类型、无对应访问、越界 layer/mip 都入正式帧错误回调；record 创建 attachment/view 的错误保留 ctx.report_error，不能只返回 false 丢掉原因。原 code/group、资源 owner 和会话不变。

颜色 CLEAR 使用 `clear_color_bits().map(f32::from_bits)`，不作浮点运算，不丢 NaN/符号零/其它 u32 位模式。当前 AttachmentCache 每次 prepare 从 attachment.clear 构造 VkRenderingAttachmentInfo，缓存仅复用物理 view，不会缓存旧 clear 值。新 `ColorClearWords` 是当前恢复语义名；旧 `View/view/ColorViewWords` 名为源码兼容保留且明确注释，未宣称原作者枚举名。core 的真实 getter 和 producer 一并修正，不另造 payload owner。

preserving 从整个 resource ID 改为 **具体 attachment access index + 实际相交范围**。A layer 的旧 writer 或 A layer 的 LOAD 不能使 B layer 或同 owner 的独立 shader write 自动 preserving。read-only depth 保留真实读取；CLEAR / DONT_CARE 不凭已有 writer变成 LOAD。

## rhapsody 同链增量修复与完整传播核查

当前 ledger 以精确 `(resource, range, version)` 找候选，再按 code/group 决定合并。原 same-code/group dedup 会吞掉后到的 reads_previous，造成真实前序 producer 被图裁剪；相反顺序却能保留，属于顺序错误。候选将 `reads_previous |= incoming` 放到共用 merge_content_policy，覆盖 same-code dedup 与两种原 native read-code in-place merge。

这没有按重叠范围广播标志：非同 range、非同 version 的记录不合并；不同 group 的不兼容访问仍先报错且不修改旧记录。inactive/undefined 合并规则保持原代码，任何 active alias 仍收紧真实读取。非法的“读 code + preserving”位不能在合并时被洗掉，最终仍被 WrongAccessMode / independent checker 拒绝。

完整下游已静态核查并指纹登记，不需要复制或改写现有图架构：

- `AccessDeclaration` 的 Eq/Hash 包含 reads_previous；`GraphDefinition::matches_declaration` 比较整个 declaration（仅版本归一），缓存无法忽略新位。
- subgraph `local_key` 克隆整个 AccessDeclaration，仅重写 resource/range/version，保留 preserving/inactive/undefined 位；normalized cache 区分这些合同。
- `plan::add_producer_access` 和缓存 compose_analysis 都通过 `reads_contents` 保留实际相交的旧 producer；普通 discard 仍可裁剪已被覆盖的 producer。
- `resource_contracts` 验证 preserving writer 必须有真实已初始化输入，`plan_check` 用独立空间 cell 重放核实来源。没有添加 allow_undefined+preserving 豁免。
- GraphDefinition / 子图缓存 / 完整 compile / independent check 由新增 `preserving_dedup.rs` 必要回归串起来，包含两种顺序、真实 producer 保留与普通 discard 裁剪、同位 cache hit 和位改变 miss、disjoint/非同范围记录不合并、active/inactive、非法 read-preserving 的拒绝、失败 group merge 不留半修改。

## 验证边界

新增 `raster_attachment_range_tests` 2 项、rhapsody `preserving_dedup` 4 项，未在 worker 运行。root 应执行 manifest 的两个 Cargo 目标，再用实际非空 frame 的三层 shadow/ice 清除输出验收；代码/asm/fixture 不能替代 GPU 实跑。

保留严格失败：同一个 selected LOAD view 只有部分已初始化覆盖时，图仍严格拒绝未定义部分；没有为了通帧加入 blanket undefined-preserving 权限。若实际非空原调用触发这一剩余场景，应继续带实际 producer/所有 view/原 caller 完整取证，而不是把其它层初始化、清零或扩大 heap。全部 clear16B 与深度 layer 语义已由原消费者闭合；此限定候选不证明完整帧/GPU/游戏还原完成。
