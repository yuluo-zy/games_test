# asset_libraries 未探究项 round3 闭合（2026-10-15）

> 范围：组 E（`round3/REVIEW-UNEXPLORED/groupE-newmodules.md`）列出的 asset_libraries 未探究项。
> 结论分级沿用 O/I/U。证据落盘 `asset-libraries/decomp/decomp-*.txt`（tg_ca1 反编译族）+ `utils/decomp/decomp-0x140a35240.txt`（/tmp 持久化）。
> 不改 reconstruction 源码；FrozenMap::insert 与 get_rg_buffer 只登记待集成裁决。

---

## 1. PlacementDataSsbo JSON schema（原真 U）→ **已闭合（O）**

**现状**：`load<PathBuf>` 0x1409365d0 五步已算法化，但 JSON 字段名/类型无证据；reconstruction `PlacementDataSsbo{position,scale,angle,_gap}` 字段名未证、元素尺寸未知。

**新证据**（tg_ca1 反编译族，落盘 `asset-libraries/decomp/`）：
- `decomp-0x140b3b960.txt` — `__Field` visit_str（serde 派生字段匹配，长度分派）：key = `position`(8)/`scale`(5)/`rotation_angle`(14)/`id`(2)/`custom0`(7)/`custom1`(7)/`bend_vector`(11)/`pad0`(4)/`rotation_degrees`(16)，tag 0..7 + __ignore(8)。
- `decomp-0x14142f860.txt` — **visit_map 大函数（28.5KB）**：对象键 dispatch，调 `FUN_140b3b960`（visit_str）一次、`FUN_140c90df0`（pack_2x16f_uint）两次；缺失字段检查确认 position/scale/rotation_degrees/id **必需**，custom0/custom1/bend_vector/pad0 **可选（缺省 0）**；缺失/重复字段错误串表 0x142c5cd48/0x142c5d028..058（主名长 8/5/16/2/7/7/11/4）。
- `decomp-0x14140cfd0.txt`（f32 解析，scale/rotation 共用）、`decomp-0x14140d270.txt`（i32/u32 解析，id）、`decomp-0x141412c00.txt`（u32 解析，custom0/custom1/pad0）、`decomp-0x1414132b0.txt`（数组解析，bend_vector=[f32;2] 8B）。
- **元素尺寸 0x30=48B** 三处独立证实：`decomp-0x1416164c0.txt`（缓冲创建 `param_4*0x30`）、`decomp-0x141405cf0.txt`（from_str，Vec drop `len*0x30`）、`decomp-0x140967640.txt`（逐元素收集 stride 0x30，12×u32 三组写入 +0x00/+0x10/+0x20）。
- 串表 rne-read：FIELDS 主名 = `rotation_degrees`（16 字符），`rotation_angle` 为别名（两者同 tag 2）；全名/短名串复核。
- 真实资产（`D:/game/ljxsj_92385/Tiny Glade/assets/glade/{theme}/trees/*.json`）：`[{"position":[f32;3],"id":u32,"scale":f32,"rotation_angle":f32(度),"pad0":0,"pad1":0},…]`——`pad1` 不在字段表（未知键跳过）。

**结论**：
- JSON schema（tag=声明序）：`position:[f32;3]`、`scale:f32`、`rotation_degrees:f32`（别名 `rotation_angle`）、`id:i32/u32`、`custom0:u32`（pack_2x16f_uint 产物）、`custom1:u32`（pack_2x16f_uint 产物）、`bend_vector:[f32;2]`、`pad0:u32`；必需 = position/scale/rotation_degrees/id。
- **reconstruction `PlacementDataSsbo{position,scale,angle,_gap}` 不完整**：元素应为 48B，缺 `id/custom0/custom1/bend_vector/pad0`（5 字段 24B）；`angle` 应名 `rotation_degrees`（JSON key 为 rotation_angle/rotation_degrees）。`default_anim_t` 不在 __Field 枚举（属 placement_data.rs 另一类型）。
- +0x2c 的 4B 未映射（I：padding 或 anim_t）。
- 度→弧度：JSON `rotation_angle` 为**度**（真实资产），`to_affine3a`（机器码复核 mulss [rip+0x01e50803]→0x142925870=**0.5**）按**弧度**解释 → 存储前必有转换；解析器为普通 f32（FUN_14140cfd0）无转换，.rdata float 池存在 **π/180 @ 0x142aab4a4**，转换疑在 visit_map 旋转分支 SIMD（Ghidra 扁平化未显式），**I 待集成裁决**。

**裁决项**：reconstruction 落码需按 48B 元素补全字段；SSBO 存储是度还是弧度待定。

---

## 2. texture_watch 消费端（原 U）→ **已闭合（O）**

**现状**：`texture_watch.rs` 章「消费端不在本模块三个过程内，U（A3_texture）」；ALGORITHMS §9 消费端表列 U。

**新证据**：
- `backend-misc/decomp/decomp-0x140aab370.txt`（**23774B 有效**，PDB 2640B）已存在；oa2 w9（`round3/oa2-results/w9_audio_backend.json` index 9）verdict **一致**：`systems::backend::texture_watch::texture_watch` = **0x140aab370**，锁 Mutex@+0x10、日志串逐字节核对（"Texture watch: detected changes: " 0x142add810→{0x142add828,33}；"Error processing textures: " 0x142add878）、FUN_140b73a50×2（u16 位图载体）、FUN_140917860、FUN_140be24b0、func_0x00014003bb80（1B ret no-op）。
- tg_ca1 复核：消费端对每个 changed 名字调 **`FUN_140c9cd80`（= `NameHash::compute`，0x140c9cd80，见 ALGORITHMS §2）** → 查 texture_library `by_name` 表（**@+0x30** hashbrown）→ 重载。

**结论**：消费端 = `0x140aab370`：锁 changed 集合（ArcInner Mutex@+0x10，poison→unwrap panic）→ 非空且级别>2 log → hashbrown SSE 组遍历（FUN_140b73a50 位图载体）→ 逐名字 NameHash::compute → texture_library by_name 查表 → 重载；失败 log "Error processing textures: "；一次消费清空。**按名字（NameHash）查 texture_library by_name 的推断成立（O）**。

**残余**：FUN_140b73a50 身份（位图载体细节）留待后续 worker（I）。

---

## 3. load_json_as_mesh 陈旧引用更新 → **已更新（引用 utils 闭合）**

**现状**：asset-libraries `ALGORITHMS.md` §3「U：load_json_as_mesh 真实实现（utils::load_json，B_x_out 遗留）」；§13 列 U；`mesh_library.rs.md` 纠错节列 U。

**新证据**：utils 深挖已全流程闭合（`utils/ALGORITHMS.md` §1：0x140b0eeb0 族 + `read_attribute_from_json` 0x14083e210 + `Mesh::set_attribute` 0x140a35240，含 indices/attributes/数值解析/type tag 1..4）。另将 `Mesh::set_attribute` decomp（原存 /tmp/out-tgm3）持久化到 `utils/decomp/decomp-0x140a35240.txt`（12959B）。

**结论**：
- ALGORITHMS §3 / §13 的 load_json_as_mesh U 移除，改引 utils §1 闭合结论。
- `mesh_library.rs.md` 纠错行改引 utils 结论 + 证据落盘路径。
- 不留陈旧 U。

---

## 4. get_rg_buffer panic 参数序 → **登记待裁决**

**现状**：ALGORITHMS §1 已记「机器码参数序：全名在前，wp4 文案槽序待重建侧核对」；reconstruction `ssbo_library.rs:21` 注释 + `get_rg_buffer` 实现（`T::NAME` 短名在前）渲染 `"Buffer {short} (type: {full})…"`。

**新证据**（tg_ca1 复核 `decomp-0x1402b31a0.txt`）：panic `format_args!` 参数序 = `[{0x1429fcd5c,0x49=73 全名}, {0x1429ae158→0x1429ae138/28 短名}]`（rne-read：全名 "country_core::…::Cobblestone3RasterStagingSsbo"、短名 "Cobblestone3RasterStagingSsbo"）→ 渲染 **`"Buffer {full} (type: {short}) is not available…"`**。

**结论（裁决项）**：机器码 = 全名在前，短名在后；reconstruction = 短名在前、全名在后。**槽序相反**（消息内容等价，参数顺序不同）。待集成裁决是否对齐机器码参数序（若对齐，reconstruction 注释与 `get_rg_buffer` 的 panic 文案两参数需交换）。

---

## 5. FrozenMap::insert 集成裁决 → **登记待裁决（不改源码）**

**现状**：reconstruction `frozen_map.rs:5-6` 注释/实现：`insert -> Result<(), &V>`，键已存在 → 不替换、返回**已有值引用**；`get_mut`/`remove` 走内部 RwLock。ALGORITHMS §0.2（round3 机器码）：键已存在 → **丢弃新值 Box + 表不变 + 返回 Err(1)**，调用方再查表。

**新证据**：无新反编译（复用 ALGORITHMS §0.2 + ssbo `add_new` insert-first 实测：Err(1) → 读表内既有 BufferState 拼双文案 panic）。reconstruction `register`（ssbo_library.rs:259）用 `Err(existing)` 直接取既有值引用拼消息。

**结论（裁决项）**：
- 行为等价（新值都不进表、调用方都能拿到既有值）但**结构不同**：机器码返回 `Err(())` 且**在 insert 内显式 drop 新 Box**，调用方重查表；reconstruction 返回 `Err(&V)` 借用已有值（新 Box 靠参数作用域析构）、调用方不重查。
- **借用字节 @+8 语义缺失**：机器码 `get_mut`/`remove` 命中借用字节 → `try_borrow().unwrap()` 冲突 panic；reconstruction 占位 `FrozenMap`（parking_lot RwLock）无此借用字节，未建模冲突 panic。需补或登记降级。
- **只登记，不改 reconstruction 源码**。

---

## 证据清单（新落盘）

`asset-libraries/decomp/`：
- `decomp-0x140b3b960.txt`（__Field visit_str，3.7KB）
- `decomp-0x14142f860.txt`（visit_map，28.5KB）
- `decomp-0x14140cfd0.txt`（f32 解析）/ `decomp-0x14140d270.txt`（id 解析）/ `decomp-0x141412c00.txt`（u32 解析）/ `decomp-0x1414132b0.txt`（[f32;2] 数组解析）
- `decomp-0x1416164c0.txt`（TypedStorageBuffer 缓冲创建，元素 0x30）/ `decomp-0x141405cf0.txt`（from_str）/ `decomp-0x1414106e0.txt`（Vec 包装）/ `decomp-0x140967640.txt`（逐元素收集 stride 0x30）
- `decomp-0x140ad5020.txt`（to_affine3a）/ `decomp-0x1402b31a0.txt`（get_rg_buffer，panic 槽序）
- `xrefs_to-0x140b3b960.json`、`xrefs_to-0x140c90df0.json`、`xrefs_to-0x141405cf0.json`、`xrefs_to-0x1416164c0.json`、`xrefs_from-0x140b3b960.json`、`closure-r3-report*.json`、`closure-r3-job*.json`

`utils/decomp/`：`decomp-0x140a35240.txt`（Mesh::set_attribute，12.6KB，自 /tmp 持久化）

`backend-misc/decomp/`：`decomp-0x140aab370.txt`（texture_watch 消费端，既有，复核为有效 23774B）

## 更新文件

- `asset-libraries/ALGORITHMS.md`：§3（load_json_as_mesh 引用）、§7（PlacementDataSsbo schema 闭合）、§9（texture_watch 消费端 O）、§11（调用图）、§13（遗留 U 收窄）、§14（round3 裁决项新增）、头注。
- `chapters/asset_libraries/placement_data_library.rs.md`：JSON schema 闭合。
- `chapters/asset_libraries/mesh_library.rs.md`：load_json_as_mesh U 解除。
- `chapters/asset_libraries/texture_watch.rs.md`：消费端闭合。
- `chapters/asset_libraries/ssbo_library.rs.md`：get_rg_buffer panic 槽序登记。
- `chapters/frozen_map.rs.md`：insert 机器码语义头注 + reconstruction 注释差异登记。
- `utils/decomp/decomp-0x140a35240.txt`：Mesh::set_attribute 证据自 /tmp 持久化（组 E 清单 #15 一并处理）。
