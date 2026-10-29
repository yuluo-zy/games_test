# M01 墙编辑后的越界装饰剔除与删除归档

本轮恢复原 `cull_oob_decorators` 主系统 `14080d540`，以及它必需的 `apply_decorator_op` remove/首次归档合同。业务代码留在 B 版原目录，不新增代码目录，不改已通过的位置检查算法、楼梯剔除或 typed change 函数体。原调度为 set25 / parent19 / slot10，由中央 agent 接回 validator → cull → backup → fullvisual；本文的 World 测试直接运行真实主系统，不代替生产调度和 GPU 验收。

## 原路径和证据

实际核对了 `reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/cull_oob_decorators.rs`，该文件保存 PDB 元数据，不能把 `void()` 占位当成真实函数签名。

| 原入口 | 本轮用途 | 证据 |
| --- | --- | --- |
| `14080d540`，Ghidra body 4680B | 受影响墙、活桶、类型门、碰撞、首次备份、延迟删除 | `m01-cull-oob-fresh.c`、`m01-cull-main.asm` |
| `141264200`，368B | 归档墙键与 PublicWalls 的真实过滤闭包 | `m01-cull-filter-fresh.c` |
| `140b26a80`，323B | `external * mesh.local`，复用既有 `validation_overlap_shape` | `m01-cull-transform-fresh.c` |
| `141276e30`，1467B | remove：归档 → 可选历史 → 真存储删除 → 真事件 | `m01-cull-apply-fresh.c`、`m01-cull-apply.asm` |

`m01-cull-native-identity.json` 记录原 RNE、分析 EXE 的 SHA256 和上述四段机器码完全相等。主函数地址跨度4789B覆盖所有已恢复指令及中间padding；Ghidra body4680B排除部分间隙，不把二者混称为函数长度。这里使用新鲜只读反编译与汇编，无修改共享 Ghidra 类型或数据库。

## 修复前的实质缺口

旧主系统每帧创建局部 `DeferredDecoratorOpEvents::default()`，事件不进入宿主队列；只克隆归档快照的装饰 ID，遗漏活桶中新出现的 ID，并错误扩展到 Terrain/Stair 桶；缺少三种实际墙事件、PublicWalls 过滤、建筑装饰类型门和蓝图物理 overlap。位置无效即边扫描边删除，还没有保留原首次 `oob backup` 与缓存缺失的失败顺序。

旧 apply 的数据归档每次清空、覆盖首快照，第二次删除会破坏本轮原始状态。remove 事件使用调用者记录并按错误 union 偏移写出，丢失实际位置和 `!recording` 的独立状态位。

## 完整业务合同

1. 从 archivist 的墙键出发，并合并 `AdjustWallHeightCmd`、`OnWallChanged`、`EditRoofCmd`；每个键必须仍在真实 PublicWalls。按墙去重，仅扫描该墙当前 `DecoratorStorage` 桶；这里不处理 Terrain/Stair。
2. 每个当前 ID 先核实际 culling 许可，再执行 `ty.wrapping_sub(56) < 8 && ty != 62`。位置算法仍用既有 `is_decorator_location_valid`，不重写或减少分支。
3. 位置有效时还须读取实际蓝图 `collision_mesh`。缺失蓝图、Disabled、Unspecified 按原跳过；Mesh 经原 tuple 专化变换后进入真实 Rapier slot2 overlap。`ordered=true` 仅接受同墙且 ID 更小的对象，不把 interaction mesh 或反序变换替代进去。
4. 收集首次删除候选时，若 History 正在记录，先用真实 `DecoratorArchivalResources` 保存 `oob backup`，再读取缓存世界位置。缺失缓存沿原 panic；此时可已有首快照，但仍没有任何删除或删除事件。
5. 全部扫描结束后才处理候选定位器。重新断言许可、移除蓝图、真实 `remove preview` 归档，再调用存储删除；同地址首快照和其真实碰撞载荷始终保留。每帧临时表复用容量，避免克隆整桶或全部 Info。
6. 主系统不传 `DeferredDecoratorEditHistory`，原本如此；墙编辑历史来自 archivist 的真实 before/after diff，不能人为增加一条伪装饰事务。非录制时仍执行原 remove preview 归档。

remove 的事件活跃字段：

| 偏移 | 真实内容 |
| --- | --- |
| `0..8` | remove 判别4 |
| `8..24` | 实际源存储地址判别和墙ID |
| `18..60`（十六进制） | `apply_decorator_remove` 真正返回的72B Info |
| `60..64` | 独立 map key u32；与 Info.word2c 分离 |
| `70..7c` | CachedDecoratorTransforms 的真实世界位置xyz |
| `7c` | `!History::is_recording()` |
| `80` | kind flag；该主系统原 EDX=0 |
| `81` | 真正移出 Info 的类型 |

remove 可选延迟历史仅写判别5，未激活的 union 字节归零，不将 Info.word2c 解释成历史字段。原子事务、历史条目提交、碰撞重建由现有 owner 继续完成。

## 验收

执行环境为本机 macOS、真实 Bevy World，`system-decorator` 的 country-core 依赖显式开启 `native-physics`。物理用真实 Rapier、真实 ColliderMeshInstance/网格登记表与归档 Arc；没有手工 overlap 回执或空物理替身。

- `CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib cull_oob_decorators --offline`：15/15，通过10个既有位置验证和5个新增主系统测试。
- `CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline`：428通过，0失败，2项既有 ignored（仅要求 `TG_INSTALL_ROOT` 完整 B 安装树的资源加载测试）；没有新增 skip。
- 四种候选来源分别验证当前活桶 ID，重复事件不重复删除；无候选、不存在公共墙、未获许可与 special 桶保留；256种类型逐个验证原门控。
- 真实碰撞验证较小/相等/较大 ID、异墙、Terrain、Disabled 和 collision/interaction 区别；非交换姿态使错误 `local * external` 不能碰巧过测。
- 两帧检查事件不重复；两个删除保留完整首次快照及真实 collider pose/mesh Arc。`extract_diff` 后通过正式 `restore_decorators` 和真实两碰撞表适配器执行 undo/redo，验证 d2s、原记录、最终网格 owner 释放。
- 缺缓存：归档后失败、零删除、零事件。缺 ID：归档与可选历史已发生，然后存储断言，零事件。
- remove 独立测试用故意陈旧的 op.Info，证明事件来自真正移出记录；map key/Info.word2c 不同；`non_recording × kind_flag` 四种组合检查 `7c/80` 独立。

所有测试日志已保存，`m01-cull-validation.json` 记录路径及 SHA256。保留了初次注册调用尚未同步、测试泛型/类型错误，以及非法测试 fixture 导致历史规范化差异的失败日志。后者是窗口型56/57却给style0；改为该类型的原style6后验证完整原记录，未放宽断言或修改生产历史逻辑。最终测试后仅对新增/本轮独占片段做机械格式整理与注释修订。

## 本轮界限和下一轮经验

完成的是墙域主剔除及它实际依赖的 remove 合同，不是整个装饰模块或渲染模块。`cull_stair_attached_decorators*` 保持原状，仍需原独立算法、共享事件和位置载荷核验；不能因主系统完成就注册 outer4 全族并宣称通过。place/旧兼容 change 的旧事件写面不纳入本轮声明；typed change 函数体未修改。生产帧调度、保存/重启、GPU画面和真实输入由中央联合验收，本轮未操作 OS 窗口或执行这些验收。

可复用的取证经验：

- 容器迭代函数有 ICF/错误别名；`iter_removed_ids` 标签不证明遍历 removed IDs。追 fold 闭包、stride和 PublicWalls 过滤，再确定真正 owner。
- 不根据反编译函数参数名猜 union。apply 的第三参数是位置状态，归档资源也不是可省略的 sink；必须逐寄存器/栈写面追到事件偏移。
- `History`、延迟装饰历史和原始快照是三个 owner。首快照不可在逐次 remove 时覆盖，不能凭同一个 label 推断不同历史桶。
- 收集阶段与应用阶段不可混并；失败时哪些归档已经发生、哪些删除尚未发生也是原业务合同。
- 测试必须构造与类型一致的真实域记录。原历史会规范化活跃 extra 判别；遇到差异先核原转换合同，不改期望为近似比较。
- 已恢复几何/物理原语直接复用；两个矩阵组合专化顺序不同，即便名称相近也不能互换。
