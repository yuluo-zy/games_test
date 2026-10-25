# 原存档数字精度失败的独立核查

## 结论

这是将两种原版序列化边界混为一谈造成的测试失败，当前 TreeEditData 的 f32/Vec3 序列化没有丢失精度。不要改业务 serializer、公共 serde_json feature、资产数值或断言容差。

必须纠正最初对日志的左右解释：原测试 `assert_eq!(original, saved)`，LEFT 是原资产 `25.744875`，RIGHT 是当前 `to_value` 的 `25.744874954223633`，并非相反。原文件 full/03-halftimber-roof/history.json 的实际 position[0] 确为左值。两个 f64 值不相等，但反序列化至该字段的 f32 位完全相同（0x41cdf581），将 f32 再无损扩展为 f64 正是右值。

## 原机器码与当前运行包

发行 RNE/PDB 身份及所有 7 份完整函数的连续原字节独立核对见 PRECISION_NATIVE_BYTES.json；并不把 serde/glam/Ryu 依赖实现计作恢复业务。

- 原 TreeData CompactFormatter 实例14003d4e0/354B，将 position 交1404c0740/213B、scale/rot交1404bdc80/434B。后者直接保留 f32，1404bdd38 调原 ryu::pretty::format32@140b8d520；因此产生原文本里的 f32 最短表示。
- 原 TreeData Value 实例1417c2770/492B，position 交141592b70/596B → glam Vec3 Value实例14085e0c0/704B；原14085e12d、1a7、20c分别CVTSS2SD，三路均直接写 Number::Float(f64)。scale/rot同一141595a20/502B，在141595ae8做CVTSS2SD，随后写Number tag2和f64载荷。原该路径没有字符串数字和Ryu f32步骤，PDB明确依赖serde_json1.0.85/glam0.29.3。
- root 指定实际 build JSON 的 serde_json package_id 为1.0.85，features仅default/std；不是arbitrary_precision。对应.d及workspace锁一致。不能沿最初AP假设修改feature。
- root保留的 sampler-catalog-v3 可执行文件 SHA/构建绑定见 PRECISION_TRIAGE_BINDINGS.json。该实际ARM64二进制 Value::serialize_f32 在10116b458是 `fcvt d0,s0`，后直接调用serialize_f64；见 RUNNING_VALUE_F32.asm.txt。与原B机器码数值策略相同。

## 真实调用边界和最窄候选

当前 load_game.rs:1012..1017 对每个 SavedMultiEdit 调 `SerializedJson::new`，save_game.rs:55 内部是 `serde_json::to_string(value)`，这是现有生产 CompactFormatter 边界。另一方面，save_game_to_memory:138..139 按原完整重写路径先 MultiEdit→Value 再 to_string；这一条路径合法地把字段展开为f64文本。不能声称全部保存接口都直接使用compact TreeData，也不能要求原本不同的两个接口产生完全相同的数字字符。

root已明确授权 test-only 候选 precision-wire-test-candidate/ROOT_ORIGINAL_HISTORY_WIRE_TEST_HUNK.patch：原资产文本保留断言仅绑定 CompactFormatter→bytes→parse Value 的实际wire结果；loaded/resaved也走同一边界。原Value→from_value→Value自一致性另外原样强断言，依然覆盖原Value路径。原payload递归断言、所有字段、数组顺序、未知数据、(33,2120,408,63)计数均未改，没有容差或数值归一化。生产serializer完全不变。

候选已独立 `patch -F0` 精确复现并rustfmt syntax parse通过；没有Cargo、真实语料执行或生产改写。根agent需实际运行原33份语料才可宣布通过。此修正不能单独证明原完整保存/重启闭环完成。
