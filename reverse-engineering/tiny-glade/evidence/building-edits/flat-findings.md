# FlatRoofStyle 与 RimStyle 的真实消费者

## 确认的归属与入口

B `system_color::set_roof_customization::set_roof_customization` 位于 0x142299dd0；C同名候选 0x1428ac4f0。原聚焦 direct function index 只选 country_core/system_roof 等，因此会漏 system_color；完整 PDB 明确定位入口并给出全部参数类型。Cargo 中 system-color0.1.0 为 local/path，只能先标 ownership_check_required；private 源路径、typed消费参数和 Roof 字段写入共同确认本入口为游戏逻辑，不以 local 字段单独判断。

完整 PDB FunctionSystem schema 的三个输入 reader 为 SetRoofColorIdCmd、SetFlatRoofStyleCmd、SetFlatRoofRimStyleCmd。Query 是 `Query<&mut Roof>`，没有 Without<RoofMarkedForDeletion>。方向 consumer 也使用该无过滤 Query（flat-direction-query-publics.txt）；缺失对象策略却不同：customization skip，方向 consumer 的失败仍由方向取证独立处理。

本批不恢复 ColorID 第一个循环，仅恢复后二循环，并接 root 已有真实 Bevy 建筑链。

## Rim 与完整外观不能混为一个事件

现有 Roof replay tag4 对应 SetFlatRoofRimStyleCmd，不是 SetFlatRoofStyleCmd。历史的泛称 FlatStyle 必须按实际 writer 类型路由到 rim 消费者。

| 输入 | meaningful字段 | 目标写入 | 后续 |
|---|---|---|---|
| SetFlatRoofStyleCmd EventInstance32 | WallId+8，12-byte style+16，sound字节+28 | Roof40..4c全部12字节 | 不相等才RecalculateWallHoles；sound==1时PlayWallUiSound(kind11/pivot) |
| SetFlatRoofRimStyleCmd EventInstance24 | WallId+8，from+16，to+17，record-history+18 | Roof3d一字节 | 不相等才RecalculateWallHoles；record==1时History::add(tag4/from/to/WallId) |

消息ID、event container header、源padding ABI不重写。byte flag均比较精确 `==1`。Rim没有sound flag，Full Style没有本分支的History记录。两个消费者不发 RecalculateRoofCmd，不得把墙开口通知强行归类为屋顶瓦片 rebuild 通知。

FlatRoofStyle变化比较只看tag；tag2另看+1 byte、+4 dword、+8 dword，后两按原始bits相等判断，不做float近似，也忽略padding。无论通知比较是否相同，原指令都复制12字节并标Roof tick，因此inactive union bytes/padding可以发生改变而没有holes通知。Rim同样无论是否相同都写to及tick。

Rim record-history1即使没有状态变化也记录；Mutation.from来自请求from，不读取当前Roof去替换，也不校验请求from与当前byte是否一致。原game写入Arc<Mutation> meaningful字段为 allocation+10 tag4、+11 from、+12 to、+18 WallId；Arc引用计数/allocator header排除。当前 Rust返回HistoryMutation intent，不能称为已提交History事务、游标或undo/redo全链。

没有Roof height/type guard；已成功解析的pitched/animated/marked Roof亦可写入。PublicWalls::roof_entity没有关联、或后续ECS查询未命中/无效代次/archetype不含目标，都跳过本命令，不写Roof/ticks/holes/sound/history。缺失guard的skip通过原指令单独执行，真实Bevy lookup由root集成验证。

## 指令 oracle

| 基线 | FlatStyle解析后范围 | Rim解析后范围 | History入参截获 |
|---|---|---|---|
| B | 14229a20a → 14229a120 | 14229a599 → 14229a480 | 140a84ec0 History::add |
| C | 1428ac95c → 1428ac860 | 1428acd0d → 1428acbe0 | 140b4b620 History::add |

范围从query已提供成功row之后开始，末尾停在下一条reader循环入口。原游戏的条件、精确写入、tick、holes消息内容、sound kind/pivot和mutation字段生产指令实际执行。Events容量预分配，allocator提供未初始化buffer，History::add仅截原写入入参而不执行事务。未注入预期业务结果，不恢复 Bevy/storage/Arc/container 实现。

`verify_roof_customization.py` 使用真实 workspace release `roof-customization-probe`，每版1536例（Style768、Rim768），覆盖tag0/1/2、payload/padding分歧、等值、flag0/1/2、circle/rectangle pivot、tick及范围外字节保留。B1536/C1536全部一致，另各4个MissingAssoc/invalid ECS lookup guards全部skip。该证明限制于两个resolved业务核心和失败skip，不能当原query/storage/scheduler或完整History验收。

可重跑命令：

```
cargo build --manifest-path reconstruction/Cargo.toml --release --locked --bin roof-customization-probe
python scripts/multi-version/verify_roof_customization.py
```

## 三版比较与新 skill 的实际应用

A完整PDB/源清单选择旧 set_roof_color_id 模块，未找到本FlatStyle/Rim消费者匹配；记录 bounded未匹配，不把B字段/能力塞入A。B/C customization.rs checksum不同，然而此次原指令逐例证明这两个resolved分支相同，不能扩大到ColorID/UI/GPU消费者或整个模块等价。

实际应用 native-rust-reconstruction 的 mapping/instruction-verification规则带来四项判断：

1. direct index负结果不视作consumer不存在，继续完整PDB类型/定义模块查找，才找出system-color入口。
2. Cargo local/path先保留ownership未知，再用private provenance与业务写入确认；Ghidra grow WindowFocusEvent等共享地址别名仍排除容器依赖。
3. raw12bytes保存union/padding，通知比较与实际赋值分开，不因等值而跳过tick/历史/音效。
4. 原oracle桥接每处显式记录；History::add截取与真实提交分开，B/C实际执行不以source checksum或函数同名替代行为证明。

Rust为building/roof_customization.rs；真实Bevy系统、事件/通知和History请求边界由root接线，完整建筑系统仍有其他未恢复链。
