# country-core 逆向复原 SOP(2026-10-17 优化版)

> 从 ~1200 函数 fresh 判等/重建实战提炼。目标:在"只以 fresh 反编译为准"的硬标准下,
> 最大化吞吐、最小化返工。配套:判等协议(2026-10-16-logic-verify-protocol.md,含分歧模式检查表)、
> 域速查索引(2026-10-17-domain-quickref.md)、总账(2026-10-17-logic-master-ledger.md)。

## 1. 分派前(定位与去重,省 30%)

1. 查域速查索引:目标函数一行定位,不重读多份 LOGIC。
2. 查已判等清单:LOGIC §2/§3 + 最近 REBUILD/REVIEW,已判等不重复反编译。
3. 查同构家族:replay_raster×6 / draw_raster×11 / upload_raster×6 / AudioInstance 族——
   判等一个 + 验证签名差异,其余推导。
4. 分类:宏胶水(derive/serde visitor/注册/内联访问器)→ glue-legitimate 跳过;
   真业务→判等;存疑→按业务处理。

## 2. 分派(并发与解锁链)

1. 并发上限按账户实际(2026-10-17 为 3,修复后按新额度;超限 429)。
2. **解锁线先行**:跨域类型独立成线,先于消费方。
   - history WallHistoryEdit/ConcurrentEdits → walls eraser finish
   - collision ColliderDiff/remove_collider → walls restore_decorators / stairs invalidate
   - 颜色库/asset 域类型 → walls get_building_and_roof_colors
3. 主域重建与待判等审计可并行(写盘 vs 只读)。
4. 每路唯一职责:restorer 写代码 / verifier 只审计 / 一个模块一个 restorer(避免 src 互斥冲突)。

## 3. 执行(判等与重建)

1. fresh 反编译每批 ≤20(ghidra `--worker shared`);PDB `void()` 用 read_memory/机器码核对。
2. 逐函数重建控制流:分支条件(含比较方向/NaN/边界)、循环、调用序列、数据流偏移。
3. 分歧模式检查表 8 条逐条对照(panic-vs-return 反/门控极性反/载荷平面近似/事件自造/
   callee 转发错/返回结构不符/排序取最近 vs 首个/偏移表错位)。
4. 重建后逐语句判等;不等即 `未实现-需重做` 并写分歧点,禁止"待取证登记"通过。
5. 缺跨域类型:登记(地址+stride+字段+消费点),不编造;由解锁线补。

## 4. hardpoint 专项(不可逆,一次定案)

1. Ghidra shared → ReVa 签名修复 + save → High P-code,三路都试。
2. 仍不可解(varnode):分段落盘 disasm(逐 puffin scope/CALL 目标),做结构级判等,
   语句级标 `未实现-需重做`;同一 hardpoint 不再重复派线。
3. 单列报告:已解段/未解段/兜底结论。

## 5. 收口(机器可验,防回归)

1. 判等结论转数值断言测试(原生边界常量,如 create_rectangle 0.8977/0.4529)。
2. 每路回执必须带 `cargo check` + 相关单测结果;全库基线单调不减。
3. 回执三态:完成 / 登记(缺类型·跨域·hardpoint)/ 阻塞;登记写明解锁条件。
4. 波次收口后更新域速查索引 + 终版进度图;不因编译通过宣布调度完成。

## 6. 失败处理

- 429 并发 / 403 额度 / 400 网关 / 分类器限流:瞬态或配额 → 立即重试或等空位补派。
- Agent 回执只是"意图",写盘结果以文件/测试为准;关键改动抽查 diff。
- workspace 被他组 WIP 阻断:用 `cargo check -p country-core` / 探针 crate 局部复证,记录归属不代改。
