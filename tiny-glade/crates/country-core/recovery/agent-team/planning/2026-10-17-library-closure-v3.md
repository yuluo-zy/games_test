# 库内闭合清单(2026-10-17 收口 v3)

> 本轮目标:把"库内还差的(可继续自己补)"全部补齐。
> 基线:default **2069 passed / 0 failed**;native-physics **2084 passed / 0 failed**。
> 全部改动都在主树 `games_test/tiny-glade/crates/country-core`(reconstruction/skeleton 未用于本轮)。

## A 组:已补齐(本轮)

| 项 | 结果 |
|---|---|
| **BrickTransformSsbo** | 0x60B 布局(affine@0/deformation@0x30/surface@0x40/seed@0x4c/material@0x50/wall_distance@0x54/flags@0x5c),NAME="BrickTransforms",BUFFER_LEN=0x200000,CHUNK=0x10000,SsboId=0xeb575d061b90ebf5;接入 SparseInstanceBuffer |
| **AlignKind / PreferredNodeAlignment** | 24B / 0x20 按 fresh 0x140959f00/f50 锚定 |
| **AutoClutterObject** | 0x68B,key@0x40(0x142612aa0 IMUL 0x68/[+0x40]);回接 wall_roof_hashmap |
| **UiState** | 补 +0x28 交集字段,纠正 is_active;移除 todo |
| **UiWallHeight** | +0/+8/+9 及 lift/name/env 公式回接 ui_adjust_wall_height |
| **MorphPremadeShape** | serde(wall_index/before/after)经 ConcurrentEdits→WallHistoryCompoundEdit;回接 finish_edit_rectangle |
| **Color history owner** | create_rectangle 第二条 Color owner(gates 门+32B 载荷) |
| **roof_uv 管线** | 码 0/1/3/4 + 两级表 trait;roof_uv×2 落地 |
| **MagicSupports::clear / reset_edge** | Arc downgrade+双 writer clear;reset_edge_with 两次 clear 顺序 |
| **RoofTileSsbo** | 64B / CHUNK 0x2000 / SparseInstanceBuffer;gravity 去 Gap |
| **collision 剩余** | closure 去重/门控/0x40B artifact;registry free-list+gen bump;ColliderMeshInstance 0x18B;barycentric/法线/spherecast witness;new_indexed/unindexed 0x78B;legacy_spherecast 10 槽;subworld/from_slot 映射表 |
| **UI seam 7** | RtMesh 0x60/RtWorld Resource;cull 16B 来源;PrefabWatcher(100ms);load_prefabs_impl;GradingLuts 0x38 TextureRecord;PrefabInstanceData 统一;debug_display/set_history_start |

新增测试:本轮 +45 左右(drawable 4、下游 20、UI 43 相关、roofs/render 各批)。

## 仍登记(库内不属"可直接补")

### 渲染/宿主语义
- 14 个 build 闭包:依赖 `SignifierEntityBuilder::build` / render-host vtable
- edit_rectangle/ui_create_rectangle 写面与 6 个 UI 系统:宿主 `WallShapeAlignmentWrites` 已落,系统注册仍缺
- Brick 铺砖写面 / stairs reset_edge 渲染分配:需接线(类型已就绪)
- TypedStorageBuffer::new None 零填阈值(≤0x6c00B 填、≥0x80000B 不填,中间未锚定)

### 第三方/设备
- obvhs 未入 workspace(build_gpu_bvh)
- construct_arch_geo 逐顶点 operand
- free-list 空表哨兵、mesh 源链接
- RtBvhRecord 0x50 字段
- 事件 tag 谓词、插件注册胶水、mesh 载入、audio name/rank

### 不可逆工具面
- update_rapier_colliders 语句级(varnode;13 段结构判等已兜底)
- update_rapier_colliders 之外的部分 UNRECOVERED_JUMPTABLE(ui_draw/ui_edit terrain、signifier_animation 8..0x10)

### 库外宿主接线(验收层)
- 全局调度 PreStartup/OnEnter/OnExit、G1-G6 资产/音频组
- 完整快照往返 + init_game_state_from_snapshot 全链
- GraphicsDevice/MusicBackend/Studio/Asset*Library/GameTime/AppMode 注入

## 结论

"库内可继续自己补"的部分已全部落地;当前剩余项要么依赖渲染/宿主语义、第三方 crate,要么属不可逆工具面。代码层基线 2069/2084 双绿。
