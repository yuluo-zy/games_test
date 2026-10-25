# 逐函数逻辑判等审计协议(2026-10-16)

> 用户硬标准(2026-10-16 纠错):此前工作"大部分是数据结构和函数签名,函数内部逻辑全是假的,
> 没还原真正的逻辑和调用链"。本协议规定逐函数判等方法,替代"结构/签名/常量/诚实登记"式审核。

## 判等方法(每函数)

1. **fresh 反编译**:ghidra `--worker shared` 只读,`get_functions(fields=decompiled_code,callees)`,每批≤20。
2. **重建真实控制流图**:从反编译提取每分支条件(含比较方向/边界/NaN 语义)、循环/展开结构、数据流
   (读写字段偏移)、调用序列(callee 语义,含每调用的实参来源)。
3. **读 Rust body**,逐语句对照:
   - 分支条件逐条判等(不等长/缺分支/条件反向=未实现)
   - 循环边界与步进判等
   - 调用链:callee 是否一致、实参/返回值流动是否一致
   - 数据流:读写的字段/偏移是否一致
   - 特判:NaN/inf/符号零/回绕/溢出/位操作语义
4. **verdict**:
   - `判等`:每个分支/循环/调用/数据流在 fresh 反编译中可逐条对应,无遗漏无编造。
   - `未实现-需重做`:body 缺分支/条件错/循环错/callee 错/数据流错/自造近似(附分歧点:函数+具体哪句与原不符)。
   - `胶水-非业务`:derive/serde visitor/注册/转发,按 AGENTS.md 不计独立业务,不审计。

## 纪律

- **不得以"待取证登记"通过**:逻辑未还原到可判等的函数一律标 `未实现-需重做`。
- **不得采信作者/此前 reviewer 的描述**:只以 fresh 反编译为准。
- **每模块产出 `<模块>-LOGIC.md`**:函数表(VA/名/verdict/分歧点),不写冗长 CFG 全文。
- PDB `void()` 占位不当真实 ABI;空反编译先核对机器码。
- 脚本/写库才用各自唯一 `--write` worker;只读用 shared。

## 重建前必做:宏生成函数 vs 真实业务函数 分类(2026-10-17 用户补充)

对每个 `未实现-需重做`/`未实现(无 body)` 的函数,**重建前先判定其类别**:
1. **宏生成胶水(不重建为业务)**:derive Debug/Fmt/PartialEq/Eq/Clone/Hash/Default/serde_derive visitor/
   Bevy register_required_components/type registration/资源插入。判定依据:src 该文件是否有对应 derive 属性、
   inventory 该函数的 kind 分类(glue/inline)、反编译体是否就是 fmt 分派/空体。此类**标记 glue-legitimate**,
   不按业务重建;若 derive 缺失/错误才修正 derive。
2. **真实业务函数(重建)**:有真实算法/控制流/调用链。按 fresh 反编译重建 body,逐语句与机器码对。
3. **存疑(半胶水半业务)**:如 Debug::fmt 里含真实状态输出/序列化含字段策略——按业务重载 Debug/serde 实现,
   不当作纯宏胶水。

每函数产出分类结论+重建后 verdict,记入 `<模块>-REBUILD.md`。

## 审计范围

各 verifier 领到的模块清单以 DISPATCH.md「逻辑判等」节为准;领到模块的全部**已实现业务函数**逐函数过。
未实现的(无 Rust body)不在本次范围,单独登记。

## 分歧模式检查表(2026-10-17,第一遍判等逐条对照,减少打回循环)

8 类系统性分歧在 8 个 verifier 反复出现。判等时逐条检查,命中任一条即标 `未实现-需重做`:
1. **panic-vs-return 反**:原版某门 `panic`(unwrap/assert),Rust `return`/静默跳过(或反之)。
2. **门控极性反**:条件方向反(`if mode∈{13,14} 等待` vs `if !matches 直接消费`、`tag!=0` vs `tag==0`、`pressed` vs `!pressed`)。
3. **载荷平面近似**:原版按变体/target/style 分派写互异载荷,src 平面结构字段不全(edit 载荷 5 个、AutoSaveState 布局)。
4. **事件/命令自造**:原版无此事件/命令,src 为对齐机制编造(PlayTerrainSound 系、PlayClutterSoundCmd、TerrainHoverInput.pressed 模型)。
5. **callee 转发错**:原转发 A 函数,Rust 转发 B 函数(Raycasting::raycast 转发 raycast_w_terrain vs 纯 rapier)。
6. **返回结构不符**:原返回 ~0x40B 结构/多值/指针,Rust 降为 Option/单值/bool(RaycastWorldResult、is_point_still_valid 返 wall 指针)。
7. **排序取最近 vs 取首个**:原 sort_by 后取最近,Rust 取遍历首个(snapping_to_segments/query_with_aabb_clamped)。
8. **偏移表/字段错位**:原 body/fov/offset 表不同,或有参数名错(keycode@4 vs @0、+0x47 状态字节、channel swap)。

另:第二遍起复用已判等清单,避免重复反编译;同构家族函数(replay_raster×6/draw_raster×11)判等一个+验证签名差异即可推导。
