# WALL 域候选业务 U 收尾闭合（round3）

日期：2026-10-15
作用域：country-core wall 域 6 个候选业务 U + decorator / ui_settings 2 个已有 decomp 行。
方法：定点反编译（MCP dbid `tg_uw1`，job `wall/uw1-job.json`）+ NotFound 小函数 RNE 原字节手解（rne-disasm / rne-read）。
不改 reconstruction 源码；证据落盘 `wall/decomp/` 与 `ui-input/decomp/`。

## 逐条判定

| # | 函数 | VA | 判定 | 结论 | 证据 |
|---|---|---|---|---|---|
| 1 | `decorator_archivist::_::impl$0::init_state` | 0x140b30a60 | 胶水排除（机器码证实） | **bevy_ecs `SystemParam::init_state` derive 生成**：调 bevy_ecs 内部参数状态初始化器 FUN_14136ca50/FUN_14134c460（World/SystemMeta 布局 +0x38/+0x1d8/+0x5a8 等），返回初始参数状态。非游戏自定义业务。 | wall\decomp\decomp-0x140b30a60.txt（反编译） |
| 2 | `decorator_archivist::_::impl$0::validate_param` | 0x1403f7af0 | 胶水排除（机器码+串证实） | **bevy_ecs `SystemParam::validate_param` derive 生成**：校验 param 索引表（param_4+0x210 指针表、+0x218 计数、+0x1e0 0x60B 记录），输出 `Option<BevyError>`（i64::MIN 判别 + 错误串）。串证：0x1429d62a0 "Resource does not exist"、0x142947223 = 参数类型名拼接（Res/ResMut/EventWriter/Query…）。错误文本 = bevy_ecs 资源存在性校验。 | wall\decomp\decomp-0x1403f7af0.txt（反编译）+ rne-read 0x1429d62a0/0x142947223 |
| 3 | `decorator_storage::impl$8::remove` | 0x140a1b440 | 业务 O（机器码证实） | **d2s 索引存在性守卫**：SIMD 探针 FUN_140b78140（结果 3=未命中空槽，已核对其尾 `param_1[1]=3`），未命中 → fmt panic **"d2s remove: no such {id}"**（fat 0x142ac8fb0，Display 闭包 FUN_140a24330 拼 id）；命中 → 直接返回（本函数不执行删除，仅断言存在）。decorator_to_storage 索引（24B 条目）不变式。 | wall\decomp\decomp-0x140a1b440.txt + rne-disasm 0x140a1b440 + decomp-0x140b78140.txt |
| 4 | `decorator_storage::impl$8::insert` | 0x140a1b4e0 | 业务 O（机器码证实） | **d2s 索引插入**：经 FUN_14152bc60（Arc 克隆 + 插入，chapter merge_with 已证语义）插入；结果 3=成功（空槽）直接返回；冲突 → fmt panic **"d2s insert: already exists; {id}"**（fat 0x142ac8fe8）。decorator_to_storage 索引唯一性不变式。 | wall\decomp\decomp-0x140a1b4e0.txt + rne-read --fat 0x142ac8fe8 |
| 5 | `decorator_storage::impl$10::remove` | 0x140a1b700 | 业务 O（机器码证实） | **另一 map（self.map）remove 断言**：探针未命中（状态 3）→ panic_str **"assertion failed: self.map.remove(&id).is_some()"**（0x142ac9068, line 0x30）；命中 → 返回（与 impl$8::remove 同为存在性守卫风格）。 | wall\decomp\decomp-0x140a1b700.txt + rne-disasm |
| 6 | `decorator_subtype::_::impl$0::validate_param` | 0x14026c6a0 | 胶水排除（机器码+串证实） | 同 #2：**bevy_ecs `SystemParam::validate_param` derive 生成**。串证：0x1429a7502 "::subtypes"、0x1429a74f8 "::mesh_lib::subtypes"、0x1429a6c32 "Resource does not exist"、0x14294586a = 参数类型名拼接（含 DerivedDecoratorInfoResources）。 | wall\decomp\decomp-0x14026c6a0.txt + rne-read |
| 7 | `decorator::DecoratorType::can_place_on_stair_top` | 0x140833190 | 业务 O（RNE 手解证实） | **DecoratorType 谓词**（Ghidra NotFound，原字节手解）：`((t+0xc4) mod 256 < 3) \|\| (t&0x38 != 0x38)` → 仅窗族 t∈{0x38,0x39,0x3a,0x3b} 为 false（不可放楼梯顶），0x3c/0x3d/0x3e 楼梯顶可放。与 can_place_on_flat_roof/can_place_on_terrain/should_offer_snap_to_corner 谓词族一致。章节行原 O 结论与机器码相符，现补真实证据（替换 NotFound stub）。 | wall\decomp\decomp-0x140833190.txt（rne-disasm 手解） |
| 8 | `ui_settings::_::impl$0::previous` | 0x140b12f30 | 胶水排除（RNE 手解证实） | **enum_iterator `Sequence::previous` 依赖 trait 默认方法**（Ghidra NotFound=ICF 折叠，原字节手解）：`mov eax,0x2010004; shr eax,(t<<3)` = 打包字节表 {b0=0x04,b1=0x00,b2=0x01,b3=0x02}[t]；0x04 = Option None niche。previous(Bottom=0)=None、previous(Left=1)=Some(Bottom)、previous(Right=2)=Some(Left)、previous(Top=3)=Some(Right)。与 render_config.rs QualityPreset::previous @0x140968f40（已证 O）同族。非自定义业务。 | ui-input\decomp\decomp-0x140b12f30.txt（rne-disasm 手解） |

## 要点

- **init_state/validate_param ×2（#1/#2/#6）= bevy_ecs `SystemParam` derive 胶水**：方法集 {init_state, validate_param, queue, apply, new_archetype, get_param} 恰为 bevy_ecs `SystemParam` trait（queue/apply/new_archetype 已证为 0x14003bb80 空函数 ret）。validate_param 输出串 "Resource does not exist / Resource requested by X does not exist" 为 bevy_ecs `system_param.rs` 资源存在性错误原文。
- **decorator_storage remove/insert（#3/#4/#5）= 游戏自定义方法**：作用域为 `decorator_to_storage`（d2s，+0x60 的 24B 条目 DecoratorId→{dst,ptr} 索引）与 self.map，携带域特定不变式 panic（"d2s remove: no such"、"d2s insert: already exists"、`self.map.remove(&id).is_some()` 断言）。下层 hashbrown 为依赖，包装逻辑为游戏自有代码 → 计业务 O（集合包装级，算法简单）。
- 探针辅助函数共享：FUN_140b78140（SIMD find，结果 3=空槽/未命中）、FUN_14152bc60（Arc 克隆插入）为共享 hashbrown 包装。
- #7/#8 均为 Ghidra NotFound（小函数/ICF 折叠），以 RNE 原字节手解闭合，decomp 落盘为 rne-disasm 证据文本。
- 章节行已就地更新（decorator_archivist / decorator_storage / decorator_subtype / decorator / ui_settings），标注"候选U收尾闭合"。
