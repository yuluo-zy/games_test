# tg-text：原字体模型与帧缓存规则

2026-10-14。直接修改当前 `tiny-glade/crates/tg-text`。本轮按用户要求补齐 B 版模块内部的字体角色/共享表和帧编号缓存失效规则；不恢复游戏调用方或 GPU 接线。旧显式传表、字符串字体、u64 epoch 和额外 allocation counter 不再代替原内部逻辑。

## 字体内部模型

- 角色 tag 经原 visitor 执行确认：Label=0、MenuButton=1、TextEditor=2、Hand=3。
- 原构造 `new(metrics, role, Arc<FontTable>)` 直接持有传入共享表，按角色查表并 unwrap；从 scaled metrics 创建 `Buffer::new_empty` 和 `Editor::new`，持有 `AttrsOwned`。不需要 renderer，不提前整形，不构造虚拟字体表。
- 未缩放基准 metrics 与当前 Buffer 的实际 scaled metrics 分开。已证 `From<f32>` 将 scalar 复制成两分量；new/set_metrics 的缩放是 scale*input。
- `set_family(role)` 保留原作已执行确认的行为：比较构造角色，临时 clone/drop 共享 Arc，替换实际字体和缩放，**不写回角色字段**。因此切回构造角色可早退，后续 set_metrics 仍查构造角色。这是原行为，不修成通常预期的角色切换。
- `apply_buffer_attrs` 重建每行 AttrsList（清 spans）、置 redraw；take_editor 保留当前 scaled metrics，外层共享表/角色/基准/属性/动作队列不变；metrics 重建先移走 lines，再 new_empty，保留原失败副作用和标准 unwrap/panic。
- prepare 的 y 读取当前 Buffer line_height；此前把 Owned/Borrowed/Arc BufferRef 分支误写为字体角色分支的说明已纠正。

作者 metrics 类型名没有留在当前 PDB，二 f32 类型恢复命名为 `FontMetrics`，不是宣称取得作者类型名。没有添加未经证实的 tuple From；这项名称缺失不作为业务适配或规则缺口。具体函数、表达式顺序、初值和失败位置见 [字体合同](font/CONTRACT.md) / [原入口 17 项](font/native-font.json)。

## 原帧/缓存状态

renderer `frame` / `invalidation_frame` 和 fragment `generated_at` / `validated_at` 均为 u32。构造分别1/1和0/0。scale、成功 overflow 复制当前 frame 到失效 stamp；consume 成功在清流和 CPU glyph 上传处理后 wrapping32 +1。没有额外递增 epoch。

恢复原无符号 `<` 决策：redraw或生成 stamp落后才直接生成；否则仅当 validated_at落后于当前帧时顺序验证 glyph IDs。每项成功立即更新 usage.last_frame，后项失败保留前项副作用；失败将生成 stamp置0，再按原 `<` 决定是否生成。越界保持 Vec bounds panic，frame0/MAX回绕和 stamp0也按原比较处理。

usage 改为原 slot Vec：low16 index、高16 etagere generation，last_frame u32、generation u16。scale清零 slots但保留长度；overflow释放旧 usage。get_glyph命中不触碰 usage，expiry保留last0哨兵并用 wrapping_add(1)<frame；按 slot顺序调用原 etagere.deallocate，仅删除本次 removed ID集合对应的缓存。相同 raw AllocId被复用后原验证会接受，不保留候选额外 counter 的拒绝策略。

原 render 的判断已接回实际路径：不隐式 prepare、不强制左对齐、不因布局参数变化提前改 dirty、不在生成后清 redraw；生成只更新 generated_at，不更新 validated_at。prepare与render分开调用，布局变化只发生在原判定需要生成的分支。具体写入图、791项原指令切片与40条完整 warm leaf路径的覆盖边界见 [缓存合同](cache/CONTRACT.md) / [原结果](cache/native-cache-oracle.json)。[追加21项](cache/native-same-id-oracle.json)确认 raw ID0复用与 usage间隙零初始化。

GPU处理仍是边界：consume的CPU返回数据携带 glyph_uploads，consume_with返回 `(closure_result, glyph_uploads)`，供以后接线，内部上传处理与帧推进顺序已恢复。AtlasDescriptor/PendingGlyphUpload原有u64 generation数据字段只无损承载u32 stamp，不参与内部判断。没有实现假GPU提交或声称native Texture所有权已恢复。本库测试/示例更新新API以保证编译；`gpu.rs`与游戏调用方未改，GPU未运行。

## 检查与源绑定

实际 Windows x86_64、rustc/cargo 1.97.1。当前源码 [verification.json](verification.json) 记录：

- Debug / Release `cargo test --offline --locked -p tg-text --all-targets` 各 **33通过、0失败、0忽略**。
- all-targets 严格 Clippy `-D warnings`、fmt check通过。
- 可选gpu特性 all-targets **仅编译**通过，未做GPU接线/运行。
- 源码和Cargo上下文在最终检查期间不变；[当前源码审计](../../../../tiny-glade/provenance/text-internal-audit.json)逐文件绑定原输入/新输出SHA。依赖身份见 [dependencies.json](dependencies.json)；没有新增依赖或修改 Cargo.lock。

本轮原代码受控案例合计 **829**（字体17、缓存791、追加21）。标准容器、引用计数、字体整形、atlas分配继续直接用原依赖。原 RNE SHA为 `f507a6ce…`；font harness只桥接CRT/原hash输入边界，不运行Arc最后一次drop；cache marker只截取原决策后继，不宣称完整cold glyph/GPU入口执行。791文件保持原SHA，追加21单独保存；测试fixture均保留在本库。

旧“stale refs一律拒绝”的断言在第一次目标测试失败，因为它来自候选额外 counter；据原同ID规则与追加21项原指令结果改为明确验证原接受条件，而非放宽未知行为。另一次Clippy仅指出测试模块位置，移动到文件末尾后门禁通过；初次完整门禁保留在 `history/first-checks/`。

本轮指定的两项内部规则已闭合。真实调用方、native GPU设备/纹理/提交、A/C、所有字体/整形/生成异常输入及原完整进程不在本次验收范围；不宣称整个tg-text、整个游戏或GPU等价。历史 shared-foundation、导入快照和旧原指令报告保留，没有回写旧源码或原发行物，也没有新增MCP后端。
