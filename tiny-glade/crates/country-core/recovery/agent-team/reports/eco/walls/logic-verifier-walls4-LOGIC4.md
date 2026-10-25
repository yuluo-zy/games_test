# LOGIC4 — walls 域剩余 fresh 判等（logic-verifier-walls4，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`（宏胶水分类 + 分歧检查表 8 条）、
> `planning/2026-10-17-recovery-sop.md`。本报告**只以本轮 fresh 反编译为准**：
> `reverse-operator.py ghidra --worker shared`，`fields=signature,decompiled_code,callees`；
> 返回值为语义存疑处用 `disassemble_function` 定案。
> fresh 证据落盘 `planning/impl-walls-verify5/decomp/`：
> `batch1..5.result.json`、各函数的 `0xVA__name.c`、`pubwalls-disasm.result.json`。
> 未采信 restorer/reviewer 描述；walls3 已判等/需重做 VDA 不重复（LOGIC.md §2/§3、
> logic-verifier-walls2、logic-verifier-walls3 已消费）。

## 0. 覆盖与结论概览

- 本轮 fresh 反编译/反汇编 **48 个条目（45 个唯一 VA + archivist 家族 3 个新单态）**，
  覆盖任务点名的全部余量：inner_walls/inner_wall_state/public_wall_state（余量 0）、
  decorator_archivist 余量（含 3 单态）、decorator_subtype 余量、public_walls（wall_state/publish 邻域）、
  shapes 余量（含 walls3 §5 全部 20 个闭包 VA）、adjust_wall_height 余量、history_recording（余量 0）。
- verdict：**判等 4 / 判等-注意 3 / 未实现-需重做 26 / 胶水-非业务 15**。
  范围内剩余待判等 **0**（无 VA 的无 body 函数不属判等范围，另册登记）。
- walls3 §5 显式挂起的 shapes 闭包 20 个 VA 本批全部 fresh 定案，均为业务闭包（实体构建/曲线转换），
  无 Rust 对应实现，统一归 `未实现-需重做`；archivist 三个未取证单态已 fresh 归入家族判定。

## 1. 判等（含判等-注意）

| VA | 函数 | Rust 位置 | verdict | 判等/差异依据（fresh） |
|---|---|---|---|---|
| 0x140a91e00 | WallEntities::wall_entity | public_walls.rs:227 | 判等 | hashbrown 探针（16B bucket=WallId+Entity），命中 `MOV RAX,[entry+8]`、未中 XOR EAX → `Option<Entity>`；= `.get(&wall).copied()` ✓ |
| 0x140a91ef0 | PublicWalls::get | public_walls.rs:72 | 判等 | records map（stride 0x2a8）命中后校验 `count>1` 与 `length>0`（NaN 走 panic 分支）否则 `panic_fmt`；= `.inspect(assert_valid)`（validate_public_wall_read 同两条件、NaN 同样失败）✓ |
| 0x140a92010 | PublicWalls::get_unchecked | public_walls.rs:79 | 判等 | 仅探针命中返回 entry（无校验）；= `records.get(&wall)` ✓ |
| 0x140a922b0 | PublicWalls::roof_entity | public_walls.rs:96 | 判等 | map@+0x30（mask+0x38,len+0x48）命中 → `MOV RAX,[entry+8]`、未中 0；= `roofs.get(&wall).copied()` ✓ |
| 0x1413e7230 | archive_current_decorators_in_storage\<&str\> | 家族同 0x1412b4a40 | 判等-注意 | callee 面 = archive_storage_decorators\<&str\>/\<String\> + format_inner + hash_one；与 LOGIC3 §2 已判单态同构。collision 重建仍为跨域登记 |
| 0x1413e77f0 | archive_current_decorators_in_storages\<&str\> | 家族同 0x1412b4f10 | 判等-注意 | tracked_clone_from\<tuple\<\>\> + clone + collision insert + drop_slow\<RaycastMeshData\>；多存储循环同族。collision 重建跨域登记 |
| 0x1421a0470 | archive_current_decorators_in_storage\<&str\>（另单态） | 家族同 0x14123b5c0 | 判等-注意 | get/get_collider/get_collider_mesh + insert + tracked_clone_from + panic_fmt 全链；同族。collision 重建跨域登记 |

> 说明：3 个 archivist 单态按任务「同构家族判等一个 + 验证签名差异」复核 callee 面一致，
> 结论沿用家族的 collision 跨域登记；未以旧报告替代 fresh。

## 2. 未实现-需重做（fresh 分歧点，逐函数）

### 2.1 public_walls.rs 余量

| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140a92100 | PublicWalls::has_roof | 原生 **两个 16B-entry map 探测**：① map@+0x60（mask+0x68,len+0x78）命中即 `XOR ESI,ESI`（结果 false）；② map@+0x30（=roofs）命中后 `CMP dword[value+4],0; SETNZ AL`（`Option<Entity>::is_some` 的 generation 非零判据）再 `AND SIL,AL`。即 `!map1.contains_key && roof_entity.is_some()`（map1 为 failures 等 16B 表，PDB inline 含 contains_key/is_some 佐证）。Rust 仅 `self.roofs.contains_key(&wall)`：**缺第一表门 + 用 contains_key 代替 is_some（gen 校验）** |
| 0x140a923a0 | PublicWalls::roof_component | **无 Rust body**。原生：roofs 探针→`Option::and_then`（PDB）→ 经 Vec 索引（`[+0x10]` 边界检查）/InnerWalls 记录表（stride 0x48，`+0x110` 反索引）/存在位集 `(index>>6,bit)`→返回 `component base + index*0x58` 指针（`Option<&RoofComponent>`），失败全路径返 0 |
| 0x140a92530 | PublicWalls::get_shape_params | **无 Rust body**。原生：`PublicWalls::get`（含 count<2 / length<=0 → `panic()`）+ `Option::and_then`；命中后 `entry+0x260`：<2 → 返回 24B ShapeParams 拷贝（out+0/+0xc/+0x14）；≥2 → `*out=2`（None）；越界/无效走 panic（0x142ada193/0x142ada200，line 0x32） |

### 2.2 shapes / 创建与 signifier

| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140a93790 | create_rectangle | 公式段已判等：ratio=max(x/y,y/x)（NaN 位技巧）、max_dim、`u=clamp01((max_dim−2.25)/5.25)`、`EasingFunction::apply`、`height=clamp01((6e+3(1−e))/12)`、`ridge=smoothstep(clamp01((1.5·smooth+ratio−1.5)/1.5))`；RoofCmd 载荷 +0x28=0.5、+0x2c=height、+0x30=ridge、+0x34=0.2、+0x38=`w<h`、+0x39=style。**分歧**：① 原生 `if *piVar52==1` 走旧屋顶探针：`style=roof+0x3d`，且 `pitch=roof+0x30` 若 `pitch<=0.1` **覆盖 height_01**（Rust 无继承分支，style 恒 0）；② 原生 `new_wall_w_params → update_elevation → get_building_and_roof_colors → update_style → update_wall_state_enclosure` 分步；Rust 把 elevation/style 并入 `new_wall_w_params(4.0,…,false)` 且无颜色链；③ 原生 `ev_new_wall + History::add ×2`，Rust 仅单条 `RecordWallHistory` |
| 0x140a94880 | get_building_and_roof_colors | 逻辑在 `recovered_building/wall_authoring.rs:61 select_creation_appearance` 有近似体（settings 四色/style；previous Some → 校验；tag<2 → 继承 wall/timber/style、roof 色 `roof[0x50..0x54]`、`roof[0x40]!=0` 时拷贝 `[0x40..0x4c]`）；**分歧**：原生校验失败 `panic()`（get 内联），Rust `?` 返回 Err（panic-vs-return 反，检查表 #1）；且 create_rectangle 未调用该策略 |
| 0x14165cd40 | circle_signifier | 原生：`format_inner`（`utils::geometry::circle::impl$3::fmt`）格式化成**动态 String**，分配 40B 记录（[0]=1,[1]=1,[2]=str.ptr,[3]=len,[4]=bool），并返回 (len,ptr,len,alloc,&kind_str)。Rust `circle_signifier(center,radius)` 为静态 `UiAppendSignifier{tag=1,kind=0,position,scale,name=0}`：**无格式化/分配，kind 与 payload 不同** |
| 0x14155e460 | rectangle_signifier | 原生 `format_inner`（rectangle impl$7::fmt）+ 同款 40B 记录；closure 再做 `as_curve2 + crashy_resample_remap_per_segment`（见 §2.3 0x141559a10）。Rust 静态记录 + 把 dimensions 位打包进 name：**缺格式化与 3D 曲线重采样，payload 为自造打包** |
| 0x140ad9f10 | ui_edit_rectangle_orientation::skewed_box_trigger_vol | **无 Rust body**。原生无调用：从两输入记录（RDX/R8，各 0x28B：translation/rotation/scale）拷贝到 out（0xf0B），并按 base=in_R9D 生成固定 12 三角索引（+1..+7 轮转）写入 out+0x60..（每项 0x10B 双索引） |
| 0x140ada0e0 | ui_edit_rectangle_orientation::box_trigger_volume | **无 Rust body**。原生 `from_iter<Triangle, Map<Chunks<usize>, closure$0>>`：把索引 chunks 映射为 Triangle 集合并组触发体积 |

### 2.3 业务闭包（walls3 §5 挂起项 + 本批新增；父系统 Rust 为代理/空桩，闭包无对应实现）

| VA | 闭包（父函数） | 分歧点 |
|---|---|---|
| 0x14165cf80 | ui_create_circle::circle_signifier::closure$0 | `perimeter` 取周长 + 分配/`SignifierEntityBuilder::build` 构建 signifier 实体；Rust circle_signifier 无闭包 |
| 0x14165b330 | call_once<circle_signifier::closure$0::env,()> | `Circle2d::as_curve3` 生成 3D 曲线写入返回 24B；Rust 无 |
| 0x14155e6b0 | ui_create_rectangle::rectangle_signifier::closure$0 | `rectangle perimeter` + build 实体；Rust 无 |
| 0x141559a10 | call_once<rectangle_signifier::closure$0::env,()> | `Rectangle2d::as_curve2` + `crashy_resample_remap_per_segment<Vec2→Vec3>`；Rust 无 |
| 0x140a65510 | edit_line::signifier_edit_circle_line::closure$0 | Circle2d 周长 + build（父 0x140a64a40 已判代理） |
| 0x140b35650 | edit_arrow::signifier_edit_circle_arrow::closure$0 | build 实体（父 0x140b35120 门控 tag!=2 已判需重做） |
| 0x1409ae3d0 | ui_rotate_circle::closure$0 | build 实体（父已判等-注意/角度源 render 域） |
| 0x1408de7e0 | signifier_edit_dims_line::signifier_edit_dims::closure$0 | `insert<EditRectangleLinePreview>` + build；父 0x1408dda10 已判代理 |
| 0x1408df6c0 | shape_placement_signifiers::closure$1 | 0xc 步长打包 + build；父 0x1408dec70 已判需重做 |
| 0x1408dfa00 | shape_placement_signifiers::closure$2 | build 实体；同上 |
| 0x14155f620 | ui_modes::edit_dims::impl$1::signifier::closure$0 | `rotation_from_normal` + `x0y` + build；父 0x14155eba0 已判代理 |
| 0x14155f9b0 | ui_modes::edit_dims::impl$1::signifier::closure$1 | build 实体；同上 |
| 0x14083df50 | ui_modes::edit_orientation::impl$1::signifier::closure$0 | build 实体；父 0x14083da10 已判空桩 |
| 0x141660600 | ui_edit_circle::closure$2 | build 实体；父 0x14165d460 已判需重做 |
| 0x141660b30 | ui_edit_circle::show_full_circle_signifier::closure$0 | build 实体；父 0x141660880 已判需重做/跨域 |
| 0x1409a3620 | call_once<show_full_circle_signifier::closure$0::env,()>（同址亦 terrain focus closure） | `Circle2d::as_curve3`；共享地址（identical-code folding），Rust 无 |
| 0x140957390 | adjust_wall_ui_mode::adjust_wall_height_curve::closure$0 | 0xc 步长打包 + build；Rust 现仅单文件 adjust_wall_height.rs 简化消费，无该曲线闭包 |

## 3. 胶水-非业务（不按业务审计）

| VA | 函数 | 依据 |
|---|---|---|
| 0x140341370 | DecoratorArchivist::default | derive Default：4×HashMap 随机键构造（hashmap_random_keys） |
| 0x140766e40 | inner_wall_state impl$7::fmt | derive Debug：`Formatter::write_str` 单串 42B |
| 0x1407eba40 | decorator_subtype impl$15::fmt | derive Debug：write_str 39B |
| 0x140a87a90 | inner_wall_state impl$14::fmt | derive Debug：write_str 42B |
| 0x140a87ac0 | inner_wall_state impl$21::fmt | derive Debug：write_str 39B |
| 0x140a87af0 | inner_wall_state impl$29::fmt | derive Debug：`debug_tuple_field1_finish` 56B |
| 0x140a82390 | inner_wall_state impl$25::clone | derive Clone：Vec 深拷贝 + RawVec 尺寸/溢出检查 |
| 0x140b30a60 | decorator_archivist impl$0::init_state | Bevy SystemParam init_state 两次转发（RaycastWorld / DecoratorPhysicsColliders） |
| 0x1403f7af0 | decorator_archivist impl$0::validate_param | Bevy SystemParam derive 生成的组件存在性校验消息（0x1429dc225 等静态串） |
| 0x14026c6a0 | decorator_subtype impl$0::validate_param | 同上（"::subtypes"/"::mesh_lib::subtypes" 串） |
| 0x14003bb80 | finish\<MainCameraPlugin\>（同址 apply/new_archetype/queue ×多符号） | 1B 空体 `RET`；Plugin::finish 默认空实现 + 地址合并（identical-code folding） |
| 0x14085d900 | signifier_animation impl$1::register_required_components | derive(Component) 宏：enforce_no_required_components_recursion + register_component\<WallHeightSignifier\> |
| 0x140b35ba0 | edit_arrow impl$0::register_required_components | 同款宏：ComponentInfo/RequiredComponent 表登记 |
| 0x140a66c90 | edit_line impl$0::register_required_components | 同款宏 |
| 0x1408e0e80 | signifier_edit_dims_line impl$0::register_required_components | 同款宏 |

## 4. 总结（回传主会话）

- 本轮 fresh 覆盖 **48 条目**：判等 4 / 判等-注意 3 / 未实现-需重做 26 / 胶水-非业务 15；
  任务范围内剩余待判等 **0**（无 VA 的无 body 项按协议另册）。
- 需回派 restorer 的具体分歧：`has_roof` 双表门 + Entity::is_some；`roof_component`/
  `get_shape_params` 全缺；`create_rectangle` 的旧屋顶继承（style/pitch≤0.1 覆盖）与
  `ev_new_wall + History::add×2`；`get_building_and_roof_colors` 的 panic-vs-Err；
  `circle_signifier`/`rectangle_signifier` 的 format_inner + as_curve3/crashy_resample 动态载荷；
  `skewed_box_trigger_vol`/`box_trigger_volume` 无 body；17 个 signifier/曲线业务闭包无对应实现。
- walls3 §5 的 20 个 shapes 闭包 VA 与 archivist 3 单态已全部 fresh 定案，无需再挂「待判等」。
- 未改任何代码；只读 `--worker shared`。
