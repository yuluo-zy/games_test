# TB-R7 装饰归档与真实历史前置复核

本轮只恢复 Raise 所需的原装饰归档与历史链，沿用 B 的既有文件和当前手工重构 owner。未修改渲染引擎、Daily 成果、Steam SDK，也未进行真实 OS 输入。

## 原证与修复

原 B RNE SHA-256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

| 原入口 | 原业务 | 本轮落点 |
| --- | --- | --- |
| `archive_current_decorators_in_storages@1413e77f0` | 地址切片依次归档；缺失桶也保留空快照，同地址只取首次状态 | 既有 archivist 新增带真实资源的地址切片入口 |
| `archive_storage_decorators@1412b2a70/@1412b3910` | 归档桶后读取两张装饰碰撞表；从活 collider 取位姿和 user_data，再精确取网格 | 依次保存独立 first/second 载荷，不保存会失效的句柄 |
| `extract_diff@140b2cb30` | 读当前存储，按地址得到旧/新状态，剔除相同项，空差分不输出，释放碰撞快照并重置剔除许可 | 删除旧无参近似；记录与回放消费者统一传真实 storage |
| `extract_and_restore_to_storage@140b2c4b0` | 释放碰撞快照，依墙、Terrain、Stair 顺序移回归档桶并维护全局 id 索引 | 复用现权威 tracked_clone_into；真实物理与卸载测试覆盖 |
| `extract_diff` 内联判等 `141523a60..141523eef` | 目的变体只比较有效载荷；浮点遵守 NaN 不等和正负零相等 | 私有原语义判等，未借全局原始位字段的派生 PartialEq |

`LiveColliderPayload` 旧首元错误地使用含 shape Arc 的 `OffsetColliderTransform`。原指令仅复制 collider 的 28B Isometry，再克隆完整 RaycastMesh，故本轮改为现有 `IsoTransform` 与 `RaycastMesh` 的元组，不额外克隆 shape Arc。

原 `+0xc0/+0x150` 是 Terrain/Stair 两个独立桶。旧代码误把它们用作所有墙的全局 before/after，既没有当前存储参数，也不能正确生成撤销载荷。本轮按每个真实地址的归档与当前条目生成差分，并恢复相等条目的双侧删除。

原记录调用 `1409dd921` 传 `skip_preview_validation=false`；原回放调用 `140b382ce/@140b38467/@140b384a9` 传 true。此标志只跳过附加预览诊断；原 `From<DecoratorInfo>` 本身仍拒绝预览条目，没有把非法预览写入历史。

## 调用与职责

`ui_raise_shape` 由 demo owner 接入 `[Wall(w), Terrain]`、真实 `DecoratorArchivalResources` 和 `change_elevation`。本轮同职责补上 `ui_adjust_wall_height_mode` 已有但未使用的资源参数，原来源为 `adjust_wall_height`，来自 `140ac7115` 调用及 `142adf9b0` 常量。

记录完成调用 `extract_diff(&storage, false)`；泛型 replay adapter 和两种 feature 的 ECS pop 均使用相同 storage 与 true。缺少提取所必需的 storage 时，回放在弹桶前返回错误，保留桶和载荷。

## 验证边界

| 构建 | 归档 | 记录生命周期 | replay_pop_bucket | 调整墙高 |
| --- | ---: | ---: | ---: | ---: |
| 默认 CPU | 10 通过 | 5 通过 | 2 通过 | 16 通过 |
| native-physics | 12 通过 | 5 通过 | 2 通过 | 16 通过 |

物理测试构建真实 Rapier collider 与 mesh owner，验证两种碰撞网格各自的原位姿、通过活 user_data 取得网格、陈旧表回执不干扰归档、归档跨活世界销毁持有网格、恢复释放 Arc，以及陈旧活句柄按原断言失败。历史测试通过权威放置接口建立索引，执行修改/删除/新增后提取差分，再通过生产恢复函数 undo、redo。

原指令判等差分 10,000 例通过，包含五种合法目的 tag、80 字节逐字节扰动和 NaN/±0/±∞。执行从已解析同 id 条目处开始，在相等/不等分支结束，没有判等桥接；排除 HashMap 寻址、分配和删除。Rust 候选由正式源提取，编译目标为 aarch64-apple-darwin，原指令在 Unicorn 执行。该证明不等于原生 Windows 整进程执行。

首次测试的一个夹具误在非空存储调用要求空存储的 `restore_snapshot`，已修正为生产 `apply_decorator_place`，保留失败日志，未修改业务实现掩盖失败。全部命令、源指纹和日志见 `toolbar-decorator-archival-handoff.json`。

## 尚须区分的范围

其余墙业务仍有数据桶兼容归档入口，不能因此宣称所有编辑都已接入真实碰撞资源。原 `decorator_colliders@140005c60` 和 `decorator_interactions@1403c03c0` 分别返回归档碰撞表 `+0x60/+0x90`；当前碰撞快照没有真实查询读侧，已通知中央与 Raise owner 核对约束算法依赖。保存真实载荷与恢复全部碰撞约束是不同验收项。

## 下轮复用经验

1. PDB 的 `void()` 是占位，参数应从调用点恢复；漏掉当前存储会让整条历史链只有形式没有真实业务。
2. 相邻相同类型字段不代表 before/after，必须先核对地址判别和三路迭代器。
3. 碰撞归档应从活 collider.user_data 取得精确 generation 与网格；外部回执可能陈旧。
4. 先恢复 enum 有效载荷的比较语义，再进行原指令差分。不要对浮点位字段直接派生等价。
5. 避免递归格式化已冻结子模块，父模块 rustfmt 使用 `skip_children=true`；保持首次失败和修复原因可追踪。
