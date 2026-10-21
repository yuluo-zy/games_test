# 完整楼梯插值、比较与历史 consumer 交付

本包基root当前已应用的ladder7eb00583；candidate.patch只在同stairs_assemble.rs补完整原8622B consumer、typed元数据读取和10个实际World测试，更新本源头注释。未改生产、Cargo、共享注册、其它U函数或用户文件。原完整合同47b5a1b3和parent独审94629d68均保留；未执行Cargo/原target/C验证。

## 已实现与实际调用面

真实六资源SystemParam → canonical requests/metadata/RemovedSupports → old Curve3XZ插值 → ordered motion/width/rawbit/lazy/visual门 → output OptionCurve3 → 同previous.sampled_points重插或fresh/dirty → 消失identity四Vec drop；同previous.sub_segments增量extend、arch_support不动。完整失败索引顺序、原optional raw-disc门、同数reuse取旧点/异数取当前点、fresh CurveErr仍插history/dirty均保留。不会把第四opaque Vec<u32>误作Params辅助Vec<f32>。

10个test均真正World运行已恢复generate_playermade_stair_requests::<false>生成Params，再运行本consumer首帧生成旧历史；只在下一帧显式修改待测canonical输入。没有Default Params或假visual。旧/新点数不同时，测试明确四Vec可独立长度；该罕见输入只验证native无额外长度预检，不宣称正常上游必产此形状。

覆盖首次/删除、独立三历史表、同数旧几何复用/旧dirty不清、异数插值、XZ重采、dt0仍按插值前delta标脏、切向反号/零fallbackX、width严格边界/NaN有序比较、raw±0与NaNpayload/四可缺元数据、missing/nonStair/未知rawdisc门、真实visual缺失/force与globaloff、NaN-Y旧XZ可用而XYZ输出Err、fresh XYZErr仍history/dirty。所有本模块global测试共用Mutex且RAII恢复原AtomicBool。

节点fixture沿真实request consumer输入域提供graph、typedStairExtra、terrain destination和真实cache；Info.ty未在该原链读取，不凭名字擅定3f，未由此宣称外部point producer ty域已锁。无假高度/私有Curve/默认transform。

## 必须配对的共享增量

- unique canonical Params/Requests完整owner、全请求producer已由root应用；当前必要资源/flags/visual getter仍复用root统一owner。
- utils factor40B已由root应用；不内联另一份factor。
- **Curve3XZ原constructor域必须应用tree候选3275955f**（TG9cab旧clutter/delegated…/curve3xz-constructor-policy-20261021）。当前root af00仍错误地提前拒绝NaN Y，mandatory World case会真实失败。该依赖不靠本地替代绕过；outputXYZ仍保其原失败语义。
- 上游Spline2D原失败域沿已审811b088c；本包不修改solver或try域。

## 接线和剩余边界

完整consumer目前可由World直接运行；本轮源码rg只找到生产定义，没有实际App caller。原Plugin member0/1的specialization/run_if仍未闭合；不能用现代constfalse或假global门推定并mount。完整cache9331B以及terrain from_forward607B必要共享leaf仍在同队独立合同，不能用sentinel/fallback掩盖未完成计算。楼梯两完整mesh装配器、支撑/碰撞/订阅等本源仍标U的函数本包保持原样，未算完成。

本包F0精确重放与rustfmt语法解析成功只证明候选可复现/能解析；10项尚未Cargo运行。下一步由独立reviewer核source/原证，root配对共享XZ后执行实际CPU，再按原注册证闭合宿主与cache；GPU/性能/完整编辑保存恢复不由本包冒充验收。
