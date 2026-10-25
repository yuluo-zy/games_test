# tg-text 独立代码审查（2026-10-13）

当前 FontTable 与 ShapeStream/RawShapeStream CPU 子域，没有发现已确认的可执行代码缺陷、性能缺陷或依赖重造。发现 1 个真实证据/注释错误 T-DOC-1，父集成已修正当前注释并登记历史文档勘误；核心实现不变。完整 tg-text 仍不完善，TextFragment、TextRenderer、字形缓存/atlas 与实际消费者接线均是此前明确声明的缺口。

本次独立审查先读当前源码，再检查原骨架、LLVM 记录与已有原指令证据。骨架符号只作入口参考，不将派生 clone/From/serde、闭包、单态化或私有内联当作独立业务功能分母。未修改 crate、共享 Cargo 或 tracking；本 reviewer 的所有新文件位于本目录。未启动新 MCP worker，也未重复运行未变化的 72 组原数值案例。

## 已确认问题 T-DOC-1：把同址 helper 标签当作 Deserialize 源级请求

原注释 shape_stream.rs:33 和 batch-01 BUSINESS_RULES.md:14/59 等写成 deserialize_str，但实际代码 String::deserialize 正确。原 PDB 链在 [骨架:605](/D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/tg-text/src/shape_stream.rs:605) 明确为 module1186 record2768（serde impls.rs:700 的 String Deserialize）→record2792（RON tag deserialize_string:139）→record2816（RON deserialize_string:455）。[LLVM 导出:490](../../small-libs-batch-01/tg-text/original-module-1186.txt) 保留相同父子记录。0x140fad7f0 调0x141e43360 的物理 helper，同址标签不能裁决源级方法。

独立 request probe 得到 deserialize_string，原 serde1.0.200 源码也如此。没有 str 与 string 的实现兼容 bug。修复是纠正文档/注释，保留 String::deserialize；无需引入自定义 Visitor。父集成修复证据见 ../integration/documentation-fix.json 与 ../integration/ERRATA.md。本目录 comment-only-correction.json 将当前源码还原两条历史注释后计算，SHA 正好等于旧 batch 源码 SHA f375d41e…，证明可执行 Rust 未变。旧 batch 报告及其 hashes 不覆盖。

## CPU 业务与功能核查

| 业务 | 审查结论 | 证据边界 |
|---|---|---|
| FontFamily/FontTable 查找与缺角色 None | 当前表示合理，不添加角色回退 | 原角色 visitor/发行表与原 get 静态；lookup 未执行原完整入口 |
| scale 缺失=1，两个 metric 相乘、family 借用 | 无相反证据；不净化 NaN/Inf/负scale | 原 default/scaled 数值证据已有；非原 Rust ABI 承诺 |
| consuming builders 与 Vec 顺序 | 移动容器、push，无整流 clone，无丢失 source ownership | 原 builder 物理范围与 PDB；std 容器/Drop 直接复用 |
| FillColor、radius、image index/scaling | 当前协议与已有原 word 证据一致；超16位索引截断有证据 | 原 append oracle 包含 NaN/Inf/±0/边界与混合 prefix；本次不扩张 case 数 |
| 私有 Raw::encode / Image::with_scaling | 合并入 append 三分支合理 | 原 inline/I PI 与 word 消费合同；words() 不是原 encode 的冒名替代 |
| clear 与 encode_no_images | clear 保留容量；numeric image 有效，named 触发不可达 | 原 clear 及静态 panic/vtable；原 panic 未执行 |
| RON fields/defaults/errors | 已恢复字段与派生规则合理，直接调用原 serde/RON | 原完整 serde 执行及所有 unknown/error payload 未验收 |
| resolver 或增长失败的部分状态 | 候选 resolver panic 保留已写 prefix，不承诺事务回滚 | 独立候选 panic 观察 [99,0,7]；原错误路径未执行，std增长不重造 |

本轮新候选 probe 接受未知 FontTable/entry/Image 字段、named FontTable/FontTableEntry、raw source 字符串；拒绝 ShapeStream(...) newtype 包装。这些接受集合没有原入口运行对照，不能编成原兼容 bug，也不能升级成完整 schema 验收。原反编译未知字段分支有统一 helper，本次未靠自动标签把它当完整 serde 行为证明。

## 性能与依赖

append 单次为 O(items + resolver work)，只读 source、逐条写 raw，不做去重/排序/整流复制；builder 的 consuming 接口移动 Vec，不复制所有已有 items。clear 为后续 append 复用容量。

独立 release allocation probe（原共享 lock、原 serde/RON；仅本 host 候选实现）对1k/10k/100k条 FillColor 首次分配/重分配次数为10/14/17，clear 后 warmed append 都为0；每条约2.06/2.00/2.43ns。这支持线性工作量与容量复用，无确认性能缺陷。它不是原作性能对照，不承诺真实文本/renderer 性能，也不作为更广泛 benchmark。

当前实际复用 serde1.0.200、PounceLight RON0.8.1 rev2787d8c、std Vec/HashMap/String 与 f32 clamp/cast。metric 乘法、scale 默认和 u32 word packing 属于游戏私有合同。没有手写 RON/serde parser、容器、字体整形/栅格化或 atlas 分配器。后续完整文本链应直接复用原 cosmic-text0.11.2 fork commit3da2a14ea3d063875fdc55695431e74405302b0f 与 etagere0.2.10，恢复游戏调用、失效、淘汰与所有权策略。

## 保留缺口与交付

T-GAP-1 是已有完整性缺口，不是本轮发现的 CPU 回归。当前 crate 没有 TextFragment/editor/prepare/原 primitive 生成，也没有 TextRenderer/font loading/cache/atlas/upload/consume/render，不存在成功 stub。真实 image name catalog、toolbar/SSBO consumer 与字体库接线、原 serde/失败路径、A/C 尚未验收，完整 tg-text 不能标完成。

结构化逐项结论见 findings.json，候选结果见 probe-results.json，执行日志见 scratch/build-result.json，输入快照见 inputs.json。首次临时 build 生成了不同 transitive lock，未作为权威结果；最终成功 probe 先复制共享 lock，使用原版本离线编译。无网络请求或依赖安装，无原游戏/GPU 启动。
