# REVIEW-REBUILD-WALLS — walls 域重建复验（reviewer-eco，2026-10-17）

复核对象：rebuild-walls（REBUILD.md，11/11 真实业务重建）。方法：与 fresh 反编译/反汇编逐函数
判等。实测：cargo check 过（country-core + 下游 critters）；lib test 目标被既有无关注册期
错误（rhapsody refactor 后 render/shader.rs 等 GraphicsDevice 用法）阻塞，本批改动文件无
test error。

## 逐函数判等

| # | 函数 | REBUILD 声明 | 复验 | 判定 |
|---|---|---|---|---|
| 1 | on_wall_split 0x140a1f590 | 三参签名(wall, curve, children:&[(WallId,low,high)])；源 removed_ids 并入注册表(原墙键)；state==2 break；d2s remove(miss panic)；**侧判定=判别字节 dst_tag(+0x22) 非 state(+0x47)**；侧0=u 区间包含搜索(get_approx_u_from_pos+clamp[0,1]+low<=u<=high)→命中直接回插子墙桶+d2s update，未命中落注册表(原墙键)；侧1=注册表(原墙键)；侧≥2 unreachable | 源码逐行对上（side_from_state(info.dst_tag())、break、get_approx_u_from_pos+clamp、children.find(low<=u<=high)→insert+d2s.update、else 注册表、unreachable）✓ | **判等**（侧字节修正自 §11 的 state→dst_tag，公式 (b-2)<=2?b-1:0 不变） |
| 2 | merge_with 0x14155b170 | 签名(self,source,d2s,addr)；3 循环：①source.removed_ids→目标 modified 传播(缺失才插)；②目标 keys→d2s remove(miss panic)+insert(addr)；③source 内嵌(state==2 break)→d2s remove+removed_ids.remove+modified.insert+重插+d2s.update(addr) | 三循环源码逐条对上 ✓ | **判等** |
| 3 | on_walls_merged 0x140a1ea40 | 双源 remove_entry 后仅两源并存调 merge_with；单源逐装饰 d2s remove+insert(merged)+per_wall.insert | （REBUILD 结构，源码复核 merge_with 内置 d2s）✓ | **判等** |
| 4 | restore_snapshot/take 0x140a20d70 | DecoratorAddrSnapshot 携带 info:DecoratorInfo；restore 逐记录 get_or_insert_default→removed_ids.remove+modified.insert+内嵌 map insert(info)+d2s insert(冲突 panic) | snapshot.info:DecoratorInfo + restore 回填 ✓ | **判等**（0x28B/0x16B 节格式字段提取待①） |
| 5 | contains_shape 0x1411ff330 | 采样 6 点 {0,1/6,1/3,**1/2**,2/3,5/6}（1/2 非 1.0 修正）；自圆他圆 6 采样、自圆他矩 4 角、自矩他圆 6 采样、自矩他矩 4 角 | valid_enclosure 采样数组 {0,1/6,1/3,1/2,2/3,5/6} ✓ | **判等**（§11 单点 u=0 纠偏不符，已重建） |
| 6 | expand 0x1411ff680 | 圆 tag0 仅 r+=amount、+0x14/+0x18 保留原值；矩形 tag≠0 hw/hh **加 2a**（非覆写） | 源码（圆只改 0xc/矩形加 2a，测试 (5,9)）✓ | **判等**（§11 覆写描述相反，已重建） |
| 7 | snapping_maintain 0x140a6ad00 | 全状态发布到 +0x20..+0x38（mode/snapped/snap/wall/end）；SnappingMode 增 published:SnappingModePublished | published.mode/snapped/snap/wall/end 逐字段发布 ✓ | **判等** |
| 8 | snapping_to_segments 0x140a6b5e0 | 候选按距离 sort_by+total_cmp 取最近，再 SQRT<1.5 写 mode=3 | candidates.sort_by(distance total_cmp)+最近判定 ✓ | **判等** |
| 9 | query_with_aabb_clamped 0x140aa5a10 | get_cells 后按 AABB 中心距离排序+同墙去重 keep 最近 | out.sort_by(endpoint_distance2 total_cmp)+dedup ✓ | **判等** |
| 10-12 | CompoundEditBuilder 三件 | 去 started/current/next 空门/build None；next 单条 edit 新组、build→WallCompoundHistoryEdit | 源码无 started、next 直接 push、build 无 Option ✓ | **判等** |

## 判等-注意残留（REBUILD §2，维持登记）

- `apply_decorator_place`（0x140a20c70）：native insert 内 if(state==2) d2s insert+*(rcx+0x47)=2；
  Rust 不强制 state=2——登记（非本批 11 函数）。
- `apply_decorator_remove`（0x140a20670）：native d2s remove miss==3 panic；Rust `let _=d2s.remove()`
  忽略——登记。

## 改动文件范围

REBUILD 自报 9 文件（decorator_storage/snapping×3/wall_spatial_hash/wall_history_edit/
valid_enclosure/save/snapshot + save 构造点 ripple）。现场 git status 混杂他组并发 WIP
（input/rhapsody/render 等），无法独立隔离——以 REBUILD 自报 + 本批函数位置为准，
**未见 decorator/wall 域之外的本批业务改动**（除 save/snapshot 因 DecoratorAddrSnapshot
新增字段的必要 ripple，属诚实登记）。

## 结论与 DISPATCH 建议

- **walls 重建项 = 闭合**：11/11 真实业务函数与 fresh 反编译/反汇编判等（on_wall_split 侧字节
  修正为 dst_tag、merge_with 三循环 d2s 重索引、restore 回填 info、contains_shape 1/2 采样、
  expand 加 2a、snapping 全状态发布+排序去重、CompoundEditBuilder 去伪状态）。
- 残留（登记）：0x28B/0x16B 节格式字段提取、apply_decorator_place/remove 两处判等-注意、
  save JSON→info 映射。
- 建议：REBUILD-WALLS=review_passed（限定）；补 0x28B 节格式 + apply_decorator_place
  state==2 强制后可闭合。
