# no-world 真实 undefined 读：当前架构合同（仅方案，待07审）

状态：contract_ready；未写实现候选、未改生产、未Cargo/GPU。目标仅B原消费者对当前rhapsody适配，不声明C或原内部ABI等价。

## 结论与边界

rhapsody已有 `PassBuilder::add_undefined_content_read_range` 完整表达本需求。它保持 `reads_contents(true/false按write判定)=true`、`contents_unused=false`、`allow_undefined_contents=true`，允许当前一次真实只读访问的内容无来源；保留已存在producer、同步访问、owner、描述符、view/range、usage、生命周期。它不产生初始化、不豁免普通后续读或导出。因此本方案**不改rhapsody planner/validator/cache语义**，只补游戏raster适配缺失的逐binding入口。

“结果不影响可观察输出”由原消费者CFG证明，不是一个新的graph优化标志。现有真实内容读仍保留blur生产链，不能借最终OpSelect裁剪原始无条件工作或移除读取。inactive_texture_contents的合同是“不发生fetch”，不适用本原路径。

## 证据与严格caller gate

- 原B RNE身份 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；本轮不新增MCP/全PDB/全shader扫描。
- 复用 `TG-0ef1874c29a03d0dec/main-chain-frame-20261018/main-graph-team-20261019/evidence/no-world-finalpost-frozen.json`。原final_post b903 Fragment SHA `dd921e013549a6b4e3101e621e3e2e1d207f47b906271447443c11fd2ebbb04c`；exact-snippets SHA `fb7c770e0f543ae73d26c1fc3012db07b54991c694c3f5975d7f8f31c3646e5d`。
- input读取/采样7940..7945；blur10482..10501；shadow数据load10736..10737；tony采样10788..10790。随后Globals成员5在10795..10798加载，10802的OpSelect在flag0选择vec3(0.180000007)，后grading使用选后值1727。真实fetch不能标inactive。
- 当前实际链：`production_renderer::build_full_world_stage` :239创建main color→:244 `select_main_world`→:260所选分支无world且非PrimaryRT时不写color，产生独立depth/CoC→:282 `resolve_world_tail_controls`→`world_postprocess::build_world_tail` :1076无条件bidirectional blur→:1130将同一decision的`flag_0x1ac0`写入FinalPostArgs.flag94→`final_post::final_blit_body`写U_FLAG94。
- 严格gate是该权威decision传播的 `controls.flag_0x1ac0 == false`（最终 `env.flag94 == 0`）。不得由swapchain/display模式、缺MainCamera、缺shader/resource、任意编译失败推断允许undefined。
- **只许可两次访问**：该tail的首blur0 `HASH_INPUT_TEX=0x11ba5697cc40b3dd`；该final_blit的`B_SRC0`同hash。blur后续mip有真实writer，B_SRC1/full blur、Tony/所有LUT上传资产、shadow实际buffer owner继续普通严格访问。不得补clear、伪造initialized import、扩到全资源或全部shader。
- 此许可依赖已冻结原shader实现。未来shader实现变更（含热重载）必须重证调用者合同；仅同名或reflection接口相同不能证明新的CFG仍无输出依赖。当前验收只覆盖该原shader实现。

## 最窄适配接口

1. DrawPacket新增 `undefined_texture_bindings: Vec<u32>`，默认空；明确宿主metadata，非原native字段。Vec保存真实反射binding index，不保存资源id/伪造owner/句柄。默认包和其它DrawPacket构造保持空。
2. DrawBuilder增加与compute同名的 `allow_undefined_texture_contents(hash)`。必须已绑定真实graph texture且只读，绑定缺反射名称按现有方法返回self；反射命中却无对应access、buffer/write、或与inactive同binding冲突须拒绝。重复调用幂等。inactive setter反向调用顺序也拒绝冲突。
3. final_blit_body在B_SRC0/B_SRC1真实绑定后，仅 `env.flag94 == 0` 对B_SRC0调用新方法。B_SRC1、disk/LUT/Tony、ShadowMaskMean不调用。
4. `raster_content_intents`把原有同pass-slot bool map改为三态map：Inactive、UndefinedAllowed、DefinedRequired。每个packet/access贡献：inactive→Inactive；仅本逐binding许可→UndefinedAllowed；默认真实read→DefinedRequired。跨packet同slot合并取更严格：DefinedRequired胜UndefinedAllowed胜Inactive。对同access同时两个metadata、非texture/write、陈旧binding/slot及不存在实际access拒绝。沿用原preserving-buffer独立验证，不混成许可。
5. `raster_host`按实际 `raster_binding_ranges`返回的view/range与slot，对非attachment的只读texture：Inactive→已有inactive API，UndefinedAllowed→已有`add_undefined_content_read_range`，DefinedRequired→已有严格API。attachment、indirect、vertex/index、external implicit访问不取得许可。不同slot同物理资源由rhapsody access_ledger的严格优先/范围contract合并；不能用owner级或全图bool覆盖。
6. blur原公用入口默认strict。新增显式 `bidirectional_blur_pyramid_allowing_undefined_input`，复用内部blur实现，仅首blur0在已绑定HASH_INPUT_TEX后调用现compute `allow_undefined_texture_contents`，不改code0x19/group/view、dispatch、输出和reverse链。`world_postprocess`仅在上述flag0 tail选择这个入口，其它所有blur使用者不变。

## 现有完整传播核对（复用，不修改）

|环节|现有触点|必须保留的事实|
|---|---|---|
|声明|rg/pass_builder.rs:144|拒绝write，范围合法；真实读+allow=true|
|同pass去重|rg/access_ledger.rs:265|ordinary strict alias在两种顺序均胜；inactive+undefined保留真实许可读；RMW不许可|
|动态初态与首次编译|rg/resource_contracts.rs:35,255|只豁免该次读来源检查，不写initialized；后续普通读/export仍检查|
|producer/裁剪|rg/plan.rs:541,729|reads_contents仍true，因此已有部分或完整writer仍形成data dependency|
|独立checker|rg/plan_check.rs:124,254|允许空缺cells但收集所有已知writer；拒绝许可write/RMW/inactive|
|同步/usage/lifetime|rg/sync_plan.rs:375；resource_info.rs:332|照原access code和真实range建全部use；不看许可删barrier/owner/usage/lifetime|
|完整与借用缓存键|resource_info AccessDeclaration derive Hash/Eq；plan.rs:145..190|许可bit纳入完整比较；flag0→flag1的内容策略变化不得误命中旧计划|
|normalized subgraph key|rg/subgraph.rs:539|normalize只改range/version/index，保留许可bit；输入port仍显式要求。本主tail未包subgraph，不改边界规则|
|实例化→record|rg/graph.rs:942..954|plan的逐access字段拷回当帧声明；callback/native绑定仍实际执行|

独立现有 `rhapsody/tests/undefined_contents.rs` 已写五组领域测试，覆盖permission、strict alias、producer/native hazard、非法metadata和cache key。这是可复用测试源码范围；本轮未自跑，不把旧结果当新适配验收。

## 需要的验收（root统一执行，07独审）

- builder/packet：默认strict；已绑定B_SRC0在flag0得到且仅得到许可；flag非0完全无许可；缺反射名不生造access；write/buffer/inactive冲突双顺序拒绝；重复幂等。
- host：同slot strict/undefined/inactive各组合与排列，任一strict alias保持strict；不同slot同资源同view由ledger收紧，部分重叠view严格访问仍要来源；attachment/indirect不可许可；实际descriptor/owner/code/group/range未变。
- blur：普通入口以及flag1完全strict；flag0仅首blur0 input许可，blurN/rev全部不新增许可；真实dispatch及mip数不变。
- plan缓存：同输入重复flag0命中；flag0→flag1变严格并拒绝无writer；flag1真实producer存在则编译且保留producer；后续普通读/export未初始化源仍失败。
- 原shader真实GPU：使用本冻结final_post、非空有效LUT和真实MSAA4×目标，输入未写且flag0，保真实blur/descriptor/owner/同步，实际回读对原select后完整grading输出；不把输出0.18直接当最终sRGB像素。对照flag1+无writer必须失败，flag1+真实producer执行并随输入改变结果；同session多帧与reset后复验无增长。缺宿主模块时单列边界，不用空stub当full-main通过。

## 未解决与不扩大范围

- reverse pyramid最顶mip把sample_self=false的full同mip读写同时声明，当前源码未给inactive/undefined metadata。这里只记录潜在后续strict init阻点；必须独立核原rev_blur SPV CFG，不能仅凭uniform名标inactive，也不能趁本补丁放宽全部reverse读取。
- 原完整main/no-world实际WSI、缺失用户模块、热重载新实现的CFG合同仍须各自验收。本合同不将图能编译等同原游戏完整主链。

## 所有权与写面

root明确委派旧claim：TG-8020f37bb27d3fedc3(main-chain-frame-20261018)/no-world-undefined-20261019 管DrawBuilder及当前host候选；TG-08b1f097c058d1fd4e(main-chain-frame-20261018)/同名子目录管final_post；TG-0f3042e78112fb54f8(render-surface-world-20261018)/同名子目录管DrawPacket。不修改原claim锁。
新合法claim：TG-eac12c8d24c206af39 / owner no-world-undefined-20261019 管blur_pyramid。world_postprocess host gate候选在root工程work以owner声明交中央，避免与02双写。

性能机制：三态复用原slot map，避免为每个普通draw再加一张BTreeMap；新Vec默认不分配，只有selected no-world final_blit一个binding发生一次小metadata分配（后续若有实测热点再优化，不声称零分配）。GPU pass/dispatch、同步、完成等待和resource数量不减少；未实测不声称提速。
