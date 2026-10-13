# B 曲线真实消费者闭包合同

## Observed

- 本轮 TG-9cab24dd1d18f9a94a / TG-3594e8acede6203727 仅 B。工具库采样/平滑/translate 已存在于共享 utils，旧 FOUNDATION_BUSINESS_COMPARISON 不能再把这些列为源码缺失。
- PDB B curve/mod.rs RVAs 0x1468880 / 0x146bb70、模块1607记录43364 /75652 把 weighted 与 wall `publish_public_wall_state::process::closure_env$0` 精确绑定。原库依赖不恢复。
- 发行 RNE 原机器码 0x14146bc8a..0x14146bcce：从第二个 callback capture 指针+0x58 读 f32。若该值严格大于0，取该值为 floor；否则使用 .rdata@0x142c6328c f32 -2.7（0xc02ccccd）。terrain sample NaN 返回 floor；有限 sample 与 floor 取较大值。Ghidra void() 伪码遗漏这段，不可只写 sample→xvy。
- 0x14146bedc..0x14146bf0f 对后续每个输出采样点执行相同策略。原 xvy 输出 Vec3(x,height,z)；原 0x14146bf8* 调 try_new_from_points<...,3D> 完整重新计算 XYZ u/length，不能复制 donor 的2D缓存。
- `_2` 每段按几何间距采样。ensure_midpoint=true只把偶数点数加1。输出先采初点，各段再输出 t=i/(count-1) for i=1..count-1；共享端点不会重复调用。重复donor点仍可形成重复sample（构造使用false），callback调用不被预先去重。
- weighted 原 0x141468880 保持两端点；内点为 (center*weight + old_previous*.5 + old_next*.5)/(weight+1)，每轮保留旧邻居，使用usize权重。当前共享源码匹配，无候选修改。
- 18 weighted /90真实terrain consumer 原入口案例在明确桥接条件下与当前共享源码+terrain_curve.rs完全相同；点、u、length和callback调用次序均逐bit比较。13验证公共依赖原锁version/source/checksum全匹配。

## Inferred

- capture+0x58 的参数疑似墙高度约束/地基策略；本轮只按字段偏移命名，不命名为具体 Wall.height。
- terrain_curve.rs 是恢复的 `process` 局部数值闭包，函数名是合成可调用入口，不声称作者原命名/ABI。

## Unknown

- 真实 `InnerWall` producer→capture+0x58 字段的schema、闭包捕获指针生命周期、完整墙发布ECS更新/事件/删除/历史链。
- terrain sampler算法不在本证明范围，原入口已用受控响应桥接；不能声称地形到墙的完整runtime链。
- 当前country-core没有共享utils依赖；publish_public_wall_state仍typed gap。候选只能作为完成原系统字段恢复后的接线代码，不应凭此把整个publish标成完成。
- C未查询/验证，索引默认C状态不升级；不将B合同套到C。

## Delivery and validation

原结果 native.json/native-0.txt/native-1.txt；原汇编consumer.asm.txt；fixtures.json/verify_native.py/test.log；inputs.json完整源输入hash；dependency-comparison-initial.json保留首次漂移，dependency-comparison.json为最终13/13。candidate.patch只增加消费闭包，依赖前置明确留给整合owner。没有共享生产修改。
