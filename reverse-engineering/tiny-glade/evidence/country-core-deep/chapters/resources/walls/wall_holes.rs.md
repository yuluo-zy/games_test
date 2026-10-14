# resources/walls/wall_holes.rs

> 文件作用：墙门窗洞存储：WallHoleStorage 0x68B 两阶段（raw Vec 20B {center,dims,kind} 到 finalized 20B {Aabb2,kind,origin}）+ sum/xor 指纹 + wall_length/closed；finalize 按 maxX 排序、闭合时跨 seam 平移副本（origin 0/1）；clear_for_next_frame 迁入 PrevWallHoles。

函数总计 42：业务 17 / 内联无独立符号 8 / 派生胶水 17

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>` | 0x14123fc70 | 0 | FinalizedWallHoles::iter_holes_with_padding | O | wall\decomp\decomp-0x14123fc70.txt |
| `country_core::resources::walls::wall_holes::WallHoles::add` | 0x140f86500 | 15 | WallHoles::add（entry_or_default + 指纹累加） | O | wall\decomp\decomp-0x140f86500.txt |
| `country_core::resources::walls::wall_holes::WallHoles::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType> > > >` | 0x1412ba9f0 | 23 | 累加/追加 | I | wall\decomp\decomp-0x1412ba9f0.txt |
| `country_core::resources::walls::wall_holes::WallHoles::clear_for_next_frame` | 0x140f867c0 | 35 | WallHoles::clear_for_next_frame（advance_frame + finalized 迁入 PrevWallHoles） | O | wall\decomp\decomp-0x140f867c0.txt |
| `country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates` | 0x140f86cd0 | 124 | clear_for_next_frame 闭包（+8/+0x10 ×4 缩放迭代） | O | wall\decomp\decomp-0x140f86cd0.txt |
| `country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length` | 0x140f86cf0 | 165 | holes_no_duplicates 闭包（f32@+0x18 wall_length） | O | wall\decomp\decomp-0x140f86cf0.txt |
| `country_core::resources::walls::wall_holes::PrevWallHoles::wall_holes` | 0x140f86d00 | 181 | PrevWallHoles::wall_holes（contains_key） | O | wall\decomp\decomp-0x140f86d00.txt |
| `country_core::resources::walls::wall_holes::WallHoleStorage::new_in_progress` | 0x140f86de0 | 243 | WallHoleStorage::new_in_progress | O | wall\decomp\decomp-0x140f86de0.txt |
| `country_core::resources::walls::wall_holes::WallHoleStorage::new_finalized` | 0x140f86e50 | 247 | WallHoleStorage::new_finalized | O | wall\decomp\decomp-0x140f86e50.txt |
| `country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized` | 0x140f86eb0 | 251 | as_finalized（phase==1 否则 panic） | O | wall\decomp\decomp-0x140f86eb0.txt |
| `country_core::resources::walls::wall_holes::WallHoleStorage::as_wip_mut` | 0x140f86f00 | 258 | as_wip_mut（phase==0 否则 panic） | O | wall\decomp\decomp-0x140f86f00.txt |
| `country_core::resources::walls::wall_holes::WallHoleStorage::clone_and_reopen` | 0x140f86f50 | 308 | clone_and_reopen（深拷贝 + phase 复位 0） | O | wall\decomp\decomp-0x140f86f50.txt |
| `country_core::resources::walls::wall_holes::WallHoleStorage::finalize` | 0x140f87300 | 333 | WallHoleStorage::finalize（raw→Aabb2，maxX 排序，闭合跨 seam 副本；panic not cleared yet） | O | wall\decomp\decomp-0x140f87300.txt |
| `country_core::resources::walls::wall_holes::InProgressHoleStorage::holes` | 0x140c37760 | 407 | InProgressHoleStorage::holes（{ptr@+8,len@+0x10}） | O | wall\decomp\decomp-0x140c37760.txt |
| `country_core::resources::walls::wall_holes::InProgressHoleStorage::add` | 0x140f87860 | 410 | InProgressHoleStorage::add（0x14 步长 push） | O | wall\decomp\decomp-0x140f87860.txt |
| `country_core::resources::walls::wall_holes::InProgressHoleStorage::extend<alloc::vec::Vec<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global> >` | 0x14123fe30 | 415 | 累加/追加 | I | wall\decomp\decomp-0x14123fe30.txt |
| `country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner` | 0x14067dd80 | 440 | FinalizedHoleStorage::inner（恒等返回） | O | audio-misc\decomp\decomp-0x14067dd80.txt, collision-eco\decomp\decomp-0x14067dd80.txt, terrain-water\decomp\decomp-0x14067dd80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::wall_holes::FinalizedWallHoles::clear`（行 64）— 删除/清理
- `country_core::resources::walls::wall_holes::FinalizedWallHoles::is_empty`（行 68）— 谓词/校验
- `country_core::resources::walls::wall_holes::FinalizedWallHoles::finalize_from`（行 72）— 收尾/提交
- `country_core::resources::walls::wall_holes::HoleChecksum::add`（行 229）— 构造/注册/生成
- `country_core::resources::walls::wall_holes::WallHoleStorage::clear`（行 265）— 删除/清理
- `country_core::resources::walls::wall_holes::InProgressHoleStorage::extend`（行 415）— 累加/追加
- `country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes`（行 429）— 待推断
- `country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates`（行 436）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`clone_from`、`closure`、`default`、`eq`、`hash` 等 17 项
