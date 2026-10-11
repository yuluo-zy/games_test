# 建筑实施第一批：真实 CPU 消费者与 World 联接

日期：2026-10-11。实现基线 B，交付目标仍为 C v1.16.0。本批使用 native-rust-reconstruction 方法和三个并行 worker；std、Bevy、fastrand、glam、Serde 和容器直接复用。PDB/source/Cargo 三版归属证据分别保存在 building-constructor、building-placement、building-history-owners，不把共享库实例计为游戏恢复。

## 已交付的业务链

1. **资产与门窗**：原 loader 绑定 → 真实 JSON Vertex_Position float3 → collision bounds → world position 投影原始 Curve2 → 真实 Roof/mapped outline/arch heights 驱动 subtype → 实际碰撞资产 → 原 raw-hole producer → WallHoleStorage 跨帧 finalize。显式 loader 支持 18 个键（四族普通门窗和乡村/哥特半窗）；乡村半窗独立资产，哥特半窗保留原共享资产策略。Roof/拱段高度变化会重新选择普通/半窗并重建开口；定位失败撤除旧输出。World intent 保留与重投影是 CPU adapter，未声称恢复原拖动/anchor UI。
2. **屋顶与历史**：EditRoof 真正解析实体并写 Roof/tick，Keep 仍发 geometry 但不误标 Changed。平斜阈值事件进入 CPU type consumer，重建山墙、反馈和瓦片/平顶输出；source_flag1 的 Flat 分支请求墙源高度 1.6。Roof 五类 owner 在显式账本上 commit/undo/redo/截 future。Visual source 只取最后未读记录，Direction 全部按序录制。计时回放逐命令产生事件，统一移除完成命令、只处理前桶；不在命令之间假造 World flush。创建/删除核心不自动录制，原 producer 的 payload 通过独立输入进入录制 owner。
3. **墙高度**：原 source+54 已与 Raw.extra 的发布输入建立读写证据。底层赋值 owner 无条件写/dirty，包含同值；实际 AdjustWallHeightCmd consumer 只接受 Keep/Absolute，clamp 后同值/NaN 跳过。无锚点使用 Curve U=0.5，加真实 terrain；按原分支执行一次或两次 authority 写及两类音频输出。原 cmd flag 是音频策略，不能解释为 history 录制。命令输出实际进入 raw collection → publication → Roof spatial/cache → opening。PublicWall+296 特殊 flag 的生产者尚未关闭，当前该字段由显式 observed input 提供。
4. **墙砖消费者**：真实 finalized holes → 周期归一化/稳定次序 → 原 row bounds 候选筛选 → resolved plain proposal 裁切或水平拆分 → 每片重新计算原 flags → 96 字节 CPU brick instances，存为真实 Bevy 实体。无变化不重建，移除开口恢复数据，删除宿主清理实体。宿主发布或山墙高度改变后，过期 proposal/context 撤除并进入 pending_proposal_owner，等待原生产者刷新。

## 代码与契约

- 原 wall-constructor 模块 → reconstruction/building/wall_constructor.rs；World owner → crates/tg-building-runtime/src/wall_bricks.rs。
- 原 load_decorators/derive_decorator_subtype/WallSpaceCoord → building/decorator_placement.rs、decorator_subtype.rs；实际资源与宿主联接 → runtime/decorator_assets.rs、decorator_placement.rs。
- 原 roof edit/replay/history → roof/edit_rules.rs、roof/replay.rs、history/building_transactions.rs；实际 Query/录制/桶 → runtime/roof_edits.rs、history.rs。
- 原 InnerWalls/AdjustWallHeightCmd → building/wall_height_authority.rs、wall_height_command.rs；真实 raw owner → runtime/wall_height.rs 与 collect_raw_inputs。
- 当前 21 个阶段是显式 adapter 依赖图。Bevy set tuple 上限通过单独配置 WallGeometry 的 before/after 边处理，没有重写 scheduler。

## 验证范围

工作区 33 项测试通过，其中新增 9 项真实 World 连续测试覆盖资产放置/移动/清理、半窗随类型/方向切换、墙砖裁切/恢复/过期撤除、参数和山墙、Roof 五类历史及计时回放、墙高度命令与 recovered WallHeight replay 请求。新 CLI 连续场景 14 帧。构建与测试日志在 evidence/multi-version，统一重跑记录在 revalidation/summary.json；所有 native 报告在源码冻结后重新执行，不只更新 hash。

限定原指令报告分开解释，不相加计算建筑完成率：

- building-placement/B-placement-verification.json：1668，实际 bounds/原曲线坐标/资源绑定/组合。
- building-placement/B-subtype-verification.json：1326，实际 subtype 入口、比较边界、半窗资产和开口。
- building-history-owners/B-owner-verification.json：1264，752 Roof owner/producer/bucket，512 WallHeight constructor/instant dispatch。
- building-history-owners/B-wall-height-authority-verification.json：1024，原 wrapper/查找/dirty/源写入与原 publisher 读回。
- building-history-owners/B-wall-height-command-verification.json：1280，完整高度命令、原 clamp/terrain/authority/audio。
- building-constructor/B-wall-constructor-verification.json：当前 3516，分行、裁行、洞候选、横向分区、proposal 与 joined consumer；原报告中分开的阶段计数不代表完整 from_curve。
- B-asset-parameter-history-runtime-verification.json：真实 14 帧 CLI，区别于 native/原 fork 验收。

## 尚未闭合的边

**raw authority rows → noise/seed/authority walker → proposal/context** 是下一首要断点。当前 World 使用真实已解析 plain 输入（kind2/tag2/fence-fit false），不能宣称从原始墙输入自动生成完整砖墙。特殊/fence/roof 分支、自由墙缓存/相交、支撑/拱/楼板/门阶、其他装饰和 C 木构仍未恢复完整消费者。

Roof type switch 的 Drawable/VAO/transient mesh 后端命令、TriggerVolume/尖顶/梁、原 fork 调度及跨 writer 消费顺序未验收。Flat→墙源 1.6 在 adapter 中下一更新消费，原顺序仍待 G16 取证。WallHeight 请求已驱动实际状态，完整 Wall compound cursor/其他 history owners、辅助恢复、快照/保存尚未闭合。

ARM 矩形铺瓦 case14 一条 filler Y 的 1ULP 差异继续保留为失败；B CPU 联接不代表 C 迁移通过。system_acceptances 仍为 0，未宣告完整建筑或完整游戏。

## 研究中纠正与下一入口

native wall fixture 的 CRT IAT 必须按实际名称验证：原 memset 与 truncf 曾误作 memmove/cosf，已纠正并重新采集所有 proposal/context；旧 ancillary 报告标记 superseded。当前验证使用原始 RNE，patched EXE 仅静态分析。高度命令差分还纠正无锚点 U=0.5 与 PublicWall+296；失败报告保留在对应 owner 证据中。

下一从 building-constructor/README.md 的原 row seed、noise hash/truncf phase、MyBrick seed 与 authority walker 跟进，复用已有 Curve2 契约，关闭生产边后替换显式输入，再按 BUILDING_SYSTEM_RECOVERY_PLAN.md 继续 G01/G02/G03/G05 与 G14/G15。
