# tg-terrain

已恢复的地形笔画 CPU 编辑和历史插值。按原 B 功能归入 systems/terrain；包名延续旧恢复工程，是当前分包，不声称原作具有完整同名包。

## 来源

原文件：`reverse-engineering/tiny-glade/reconstruction/terrain/crates/tg-terrain/src/{schema,stroke,edit,replay}.rs`。逐文件 Git 提交与哈希见 [building-audit.json](../../../provenance/building-audit.json)。证据为原 `evidence/terrain-editing/findings.md`、`second-batch-findings.md`、`B-delta-edit-verification.json` 和 `milestones/M1/rust-edit-points-diff.json`。

## 已完成

笔画 point insert/morph、已解析参数的 Keep/Absolute/Delta 分支、音频观察顺序、EditTerrainStroke 前进/后退插值，以及已观察 JSON variant/field 类型。新建/移动笔画的完整生命周期尚未支持，相关 replay 明确返回错误。

## 还差什么

完整地形生命周期、ID/异常边界、ECS 与音频消费、原高度 raster、GPU、存档联接。旧 readback.rs 的证据明确记为 proposed、未接线且未原码验收，暂不搬入；water 整链仍有推测，只有完成的独立函数纳入 country-core/systems/audio/water_grid_bits.rs。

## 验证与开发差异

`cargo test -p tg-terrain`。旧 schema 测试把 f32 JSON 与原始 f64 小数逐值比较，报告已有失败记录；此处只修测试合同，检查字段名和类型化 f32 往返，不改生产实现，不宣称存档文本逐字等价。负比较按原 unordered 分支保留 NaN 行为。当前结果见 [validation.json](../../../provenance/validation.json)。
