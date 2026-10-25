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
## 实现方式（算法级）

### WallHoles 结构（O，DIGEST §1.7 深化）

- `WallHoles` = HashMap<WallId, WallHoleStorage>，条目 **0x68B**（add 0x140f86500 探针 *-0x68，键 WallId 比较@+8）。
- `WallHoleStorage` 0x68B（new_in_progress 0x140f86de0 置位）：
  - +0x00：phase u32（0=InProgress，1=Finalized；as_finalized 判 ==1、as_wip_mut 判 bit0==0）
  - +0x08..：InProgressHoleStorage（payload 联合）：
    - +0x08/+0x10/+0x18 = holes Vec（cap-first {cap@+0x8, ptr@+0x10, len@+0x18}，元素 **20B** = {center 8B, dims 8B, kind u8@+0x10}）
    - +0x38/+0x40 = **HoleChecksum {sum u64, xor u64}**（InProgressHoleStorage::add 0x140f87860 每条累加/异或）
  - +0x20/+0x28/+0x30 = finalized holes Vec（cap-first，元素 20B = {Aabb2 16B, kind u8@0x10, origin u8@0x11}）
  - +0x40..+0x58 = 尾区 4 qword（finalize 保存/复写，语义 I：seam/长度对）
  - +0x60 = wall_length f32；+0x64 = closed u8（finalize 写 piStack_140[6]/[7] = +0x20/+0x24 相对 +0x8 载荷）
- `FinalizedWallHoles`：holes Vec@+0x0..（ptr@+8, len@+0x10）、wall_length f32@+0x18、closed u8@+0x1c。
- `PrevWallHoles` = HashMap<WallId, FinalizedWallHoles>（wall_holes 0x140f86d00 = contains_key 探针，len@+0x18==0 → null）。

### WallHoles::add 0x140f86500（O）

```
键 param_2=WallId → FUN_1405b0d70(+0x20) 哈希 → SIMD 探针（0x68B 条目）
命中: 检查目标条目 phase（*（entry-0x60）==1）→ 若已 Finalized 走 as_wip_mut 复制路径；否则直接原地
未中: hashbrown insert 新条目（默认 WallHoleStorage，new_in_progress 布局）→ 把 param_3（16B {center,dims}）+ param_4（kind）写入 InProgressHoleStorage::add
```

### WallHoleStorage::finalize 0x140f87300（O，8.6KB）

```
签: finalize(self, wall_length f32, closed bool) -> u8（返回 uStack_92 = 尾区两 qword 对比较 = 是否跨界 seam）
if (*self == 1) → fmt panic（0x142bad4d0 = "finalize called on a wall hole storage that has not been cleared yet."）
保存并清零当前 finalized 区（cap/ptr/len + 尾区 4 qword）
uVar5 = raw holes len（+0x18）；puVar16 = raw ptr（+0x10）
if (!closed):
  for 每条 raw 20B: FUN_14123fff0(&out, cx, cy, dx, dy) 转 Aabb2（min/max 各 2 f32）→ 写 {Aabb2, kind@0x10, origin=0}
  容量不足 → FUN_1428a9a60（0x14 元素扩容）
else（闭合墙）:
  for 每条 raw: 转换；若 min.x >= 0 → 单条 origin=0
    否则写两条: 原样 origin=0 + 平移 (min.x±wall_length, max.x±wall_length) origin=1（跨 seam 副本，0x142bad378/390 常量定位）
收尾: phase=1，写 wall_length@+0x60、closed@+0x64、finalized holes Vec 与尾区
返回: 保存的 +0x40..+0x58 两 qword 对比较（是否需闭合处理）
```

### clear_for_next_frame 0x140f867c0（32KB，O）

逐墙遍历 WallHoles：对每条 WallHoleStorage 执行 `advance_frame`（InProgress→Finalized 阶段推进 + finalized 区迁移）并把 **finalized 结果插入 PrevWallHoles**（hashbrown insert FUN_1428c3cc0，元素 **0x14 步长 memmove**）；新帧 WallHoles 重置为 new_in_progress。尾闭包 holes_no_duplicates 0x140f86cf0（f32@+0x18 wall_length）、clear_for_next_frame 闭包 0x140f86cd0（+8/+0x10 ×4 缩放迭代，手解）。

### 阶段访问器（O）

- `new_in_progress` 0x140f86de0：phase=0，holes Vec{cap@+0x8,ptr@+0x10,len@+0x18} 与 finalized Vec 三字段置 0（Ghidra 显示 +0x10/+0x28 为常量 4——该偏移实为 ptr 槽，疑 Ghidra 对 with_capacity(4) 的近似；以 add/finalize 的 cap-first 读法为准），checksum/尾区清 0。
- `as_finalized` 0x140f86eb0：phase==1 → return &self+0x8（&FinalizedHoleStorage）；否则 fmt panic。
- `as_wip_mut` 0x140f86f00：phase bit0==0 → return &self+0x8（&mut InProgressHoleStorage）；否则 fmt panic（0x142bad460）。
- `clone_and_reopen` 0x140f86f50：phase bit0==0 时深拷贝 InProgressHoleStorage（holes Vec：len@+0x18 × 0x14 字节分配，溢出界 0x7ffffffffffffffc）+ 重置 phase=0，供重开修改。
- `InProgressHoleStorage::holes` 0x140c37760 = {ptr@+8, len@+0x10} 切片（手解）。
- `FinalizedHoleStorage::inner` 0x14067dd80 = 恒等返回（手解）。
- `InProgressHoleStorage::add` 0x140f87860：FUN_14145b910(&{cx,cy,dx,dy}) 生成**指纹**→ checksum.sum+=、checksum.xor^=；Vec push 20B（len==cap 时 FUN_140563600 扩容），kind@+0x10。

### iter_holes_with_padding 0x14123fc70（O，padding 参数 f32）

```
签: iter_holes_with_padding(self, padding, closure_env 2×f32, out Vec)
if (!closed @+0x1c): 逐条 finalized 孔 → out 写 16B {min.x,min.y,max.x,max.y} 按 closure_env 的 {scaleX, scaleY} 缩放
else（闭合墙）:
  fVar2 = wall_length@+0x18
  跳过 origin!=0 的副本孔（只处理 origin==0 主孔）
  每条主孔: 写缩放副本；再按 padding 判定跨界:
    - padding <= min.x → 写平移 +wall_length 副本（min.x+wL, max.x+wL）
    - wall_length < max.x + padding → 写平移 -wall_length 副本（min.x-wL, max.x-wL）
闭包类型 system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0（缩放因子）
```

### 新常量/panic 表

| 串 | 地址 | 位置 |
|---|---|---|
| "finalize called on a wall hole storage that has not been cleared yet." | 0x140f87300 域（0x142bad4d0） | finalize phase==1 |
| "assertion failed: !triangles.is_empty()"（邻接 0x140f86d00 域无） | — | 见 wall_trigger_volume |

### 跨模块调用

FUN_14123fff0（center+dims→Aabb2）、FUN_14145b910（指纹哈希）、FUN_140563600（Vec 扩容）、FUN_1428a9a60（0x14 元素扩容）、FUN_1428c3cc0（hashbrown insert，PrevWallHoles）、func_0x140f86cd0（FinalizedWallHoles::holes 迭代器）、FUN_14068d3d0（out Vec 扩容）、FUN_1428d9390（fmt panic）。iter_holes_with_padding 跨模块消费于 system_wall_constructor::wall_constructor::normalize_holes。

### 遗留 U

- finalize 返回值的精确语义（+0x40..+0x58 两 qword 对比较 = seam 标志，I）。
- WallHoleStorage +0x40..+0x58 尾区 4 qword 字段名。
- new_in_progress 的 with_capacity 常量与 cap-first 读法的小矛盾（I，按使用点读法为准）。
