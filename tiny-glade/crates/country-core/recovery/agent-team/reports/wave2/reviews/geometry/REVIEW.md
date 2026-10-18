# reviewer-wave2-geometry — wave2 几何/网格模块 REVIEW

日期：2026-10-17
角色：wave2 独立 reviewer（geometry）
范围：独立核验 restorer-wave2 落库的 8 项几何/网格逻辑是否真正还原原二进制业务逻辑
（硬标准：fresh 反编译 → 独立复述 → 与 Rust 逐语句判等；近似/高估一律标未实现）。
涉及 RESTORE.md：`reports/wave2/{wall-constructor,clutter,decorator-forest}/RESTORE.md`。

方法：ghidra `--worker shared` fresh 反编译（`jobs/result-batch1/2/from-curve/populate.json`，
本会话生成）+ radare2 fresh 反汇编/hexdump（`jobs/result-r2-onwindows*.json`、
`result-r2-gothic-callonce.json`）+ 既有 evidence 交叉核对。仅评审，未改 src。

## 裁决汇总

| # | 函数/条目 | 地址 | 裁决 | 关键证据（fresh） |
|---|---|---|---|---|
| 1 | `BrickRowSine::random_f` | 宿主 0x140769760 体内 | **判等** | `a=sin((x*10+phase)*12.9898)`、`r0=frac(a*43758.547)`；p0=`frac(sin((r0+r0)*12.9898)*43758.547)*1000`——行 919-931/970-971 逐语句，`(r0+r0)` 先加再乘成立 |
| 2 | `BrickRowSine::sine_wave_deform` | 同上 | **判等** | `s=0.5+0.5*sin(x*0.2)`；顶边门 `0.01<=width`、`0.06*sin((r0*s+1)*x+p0)`；底边 `!bVar45` 门、`0.06*sin(x*(s*r1+1)+p1)`——行 1196-1206 |
| 3 | `merge_nearby_proposals` | 0x140178a70 步骤2 | **判等** | 44B stride、kind@0x28==2 终止、dist<0.01 首命中合并、size_x max/size_z min NaN→new、仅写 existing+8/+0xc——fresh 行 369-445 |
| 4 | `add_autoclutter_on_windows` | 0x1422cd7d0 | **判等（二进制全确认）+ 布局 U 登记** | radare2 fresh 全函数：drain、stride 0x50、跳表 6 项解码、反射公式 `(2xz,2yz,2z²-\|p\|²)`、写块 0x68 逐偏移、未初始化读 +0x08/+0x31/+0x33、NOP 垫 `66 0f 1f 44 00 00` 实读 |
| 5 | `load_clutter_meshes` | 0x1422c0760 | **结构判等** | fresh：**55** 次 `closure_0()` + `from_iter<RangeInclusive<i32>,closure_env_1>` + `insert_resource<ClutterMeshes>`（RESTORE "~59" 微高估，见 §需修） |
| 6 | `gothic_half_dormer_radius` | 0x1412ac9b0 内联 | **判等** | radare2 fresh 0x1412ac9f6-0x1412aca29：`is_circle tag!=0 && shape_kind==7 && rank-1<3 → 表[rank-1]`，否则 0.38——**RESTORE 对 RE-ANALYSIS 方向纠正成立** |
| 7 | `chimney_smoke_anim_thickness` 公式 | 0x140818df0 | **公式判等（FIX 落实）** | fresh 全量：+0xd 分派（0→先丢一抽签后 `r*3+(1-r)`、≠0→`r*4+2*(1-r)`）、符号（0→恒 -1；≠0→窗口 `t<0.72\|\|0.9<=t` ±1）、`new=clamp01(old+sign*dt/target)`（行 554-646）、初始厚度窗口&&+0xd==1→1.0 |
| 8 | `manage_tree_prefabs` 门控 | 0x141bf16c0 | **门控判等 + 形状源未实现** | fresh：阶段1 `\|x\|>65\|\|\|z\|>65→keep`、6 路 bilinear（4 求和 clamp + 2 排除）、rock 单布尔、形状判定门 `((A&&B)\|bVar32)!=1`、墙圆查询半径 7.0 恒运行；阶段2 `+0x18` 处理字节门控、`record+0x1c` jitter、`-65<=x<65` 高度门、帧末置 1 |
| 9 | `calc_gothic_half_dormer_profile_ws` 结构 | 0x1407eb160 | **结构判等 + 变换未实现（U 诚实）** | 0..2 for_each 两次 closure_0；profile 3 档表字节核（**末点 h=-0.95** `0xbf733333` 终写证实）；wall-space 变换/0..2 索引语义 U |

## §1 wall-constructor

### 1.1 `BrickRowSine`（sine_wave.rs）——判等

fresh 反编译 `review-fresh-0x140769760-from_curve.c` 行 919-931/970-971（random_f）、
1196-1206（sine_wave_deform）：

- random_f：`fVar49=sinf((fVar65*local_118*10.0+local_27c)*12.9898)`（= sin((x*10+phase)*A)）、
  `fVar57=truncf(); fVar57=fVar49*43758.547-fVar57`（frac）；`fVar63=sinf((fVar57+fVar57)*12.9898)`
  ——**先加再乘**；`local_270=(fVar63*43758.547-fVar57)*1000.0`（p0）。
  与 Rust `((r0+r0)*A).sin()`、`.fract()*1000.0` 逐位一致。
- sine_wave_deform：`fVar57=sinf(fVar50*0.2); fVar57=fVar57*0.5+0.5`（s）；
  `if (0.01<=local_118) local_114=sinf((local_278*fVar57+1.0)*fVar50+local_270)*0.06`（顶边门）；
  `if (!bVar45) local_3a8._0_4_=sinf(fVar50*(fVar57*local_274+1.0)+local_26c)*0.06`（底边门）。

RESTORE "frac=x-trunc(x)，符号可负"、"非 2*r0*A 折叠" 均以反编译为准，成立。

### 1.2 `merge_nearby_proposals`（pillars_process_proposals.rs）——判等

fresh 反编译 `review-fresh-0x140178a70-*.c` 行 369-445：`pauVar27=...*0x2c+...`（stride 0x2c）；
`cVar37=pauVar20[2][8]; if(cVar37=='\x02') break;`（kind@0x28）；pos 打包 u64@0..8；
`do{...}while(0.01<=SQRT(fVar44*fVar44+fVar38*fVar38))`（dist<0.01 退出）→ 命中写块；
`if(fVar39<=fVar38) auVar40._0_4_=fVar38`（size_x max）、`NAN(fVar38)?fVar39:auVar40._0_4_`
（NaN→new）；size_z 同构 min。**仅写 existing+8/+0xc，其余字段保留**。未命中 `goto LAB_140178e66`
前完整 44B 拷贝。

宿主系统步骤 4-6（StairColliderRegistry/RtWorld/RtMeshHandle）依赖 core-type-gap，
宿主 `process_playermade_stair_pillars_proposals` 以 `todo!()` 登记——诚实。

## §2 clutter

### 2.1 `add_autoclutter_on_windows`（add_autoclutter_on_windows.rs）——判等（二进制）+ 布局 U

radare2 fresh 全函数（`review-r2-1422cd7d0.rawdisasm{,2}.txt`）逐条核验：

- 输入：`mov [rax+0x10],0` 清 len（drain）；`lea r10,[rdx+rdx*4]; shl r10,4`（stride 0x50）。
- 候选读：kind@+0x00（`CMP ECX,6; JZ` 跳过）、flag@+0x01、f04@+0x04、host_index@+0x08、
  16B pos@+0x10、+0x20、+0x28、+0x40、+0x48。
- 跳表 0x143042370（hexdump 实读）6 项解码：0→0x1422cd9b7、1→0x1422cdbd8、2→0x1422cdac7、
  3→0x1422cdb50、4→0x1422cda3f、5→0x1422cdc72。宿主源：kind0→base+0xf0、kind1→base Vec
  （`cmp rax,[rcx+0x10]; jae→0x1422cdd58` panic_bounds_check）、kind2→base+0xf8、kind3→base+0x1a8、
  kind4→base+0x1b0、kind5→base+0x1b8；kind2 分支 `not r12b; and r12b,1` 翻转 flag。
- 反射公式（6 case 同一 30 指令块）：`2(p·q)p + (w²-\|p\|²)q`，q=(0,0,1,0)（0x1429cd0c0）、w=0
  → 结果 `(2xz, 2yz, 2z²-\|p\|²)`，与 Rust `Vec3::new(2*p.x*p.z, 2*p.y*p.z, 2*p.z*p.z-s)` 逐位一致。
- 写块 0x68：+0x00=5、+0x18=1、+0x20=cand+0x48、+0x30=r12b、+0x34=r14d(0)、+0x38=r13
  （宿主源首 qword）、+0x40=rdi(cand+0x40)、+0x48=cand+0x20、+0x50=cand+0x28、+0x54/+0x58=
  reflect x/z（movlps）、+0x5c=reflect y（unpckhpd）、+0x60=1.0。
- **未初始化读证实**：`[rbp-0x60]`（+0x08）、`[rbp+0x55]`（+0x31 u16）、`[rbp+0x57]`（+0x33）全函数
  仅读不写；0x1422cd8ba..0x1422cd8c0 hexdump = `66 0f 1f 44 00 00`（6 字节 NOP 对齐垫），无隐藏 store。
- 满缓冲 `call 0x14078c880` grow + panic 串 "crates/systems/clutter/src/autoclutter/add_autoclutter_on_windo*"。

Rust 逻辑（kind/flag 翻转/反射 xzy/宿主源/type=5/scale=1.0/key·f48·f50 映射）与二进制语义逐项一致。
`AutoClutterSpawn` 精确 0x68 偏移为**语义字段承载**（doc 已明示"精确偏移与二进制 0x68 布局为
类型合同分歧（U）"）——注意 doc 中 +0x38=position/+0x40=host_id 与二进制 +0x38=host_id/+0x40=
cand+0x40 存在标注错位，已 U 登记，不影响逻辑判等。**无高估**。

### 2.2 `populate_autoclutter_regions`（populate_autoclutter_regions.rs）——结构判等 + 需修（分歧点）

fresh 全量 120KB（`review-fresh-0x1422d6ed0-*.c`，2958 行）核验：

- kind switch（`switch(*(char*)(local_138-7))`）：case 0/2/3/4 建 9/25/25/37 槽；case 1→
  独立处理器；`is_halloween2024()` 非零跳过 switch。**每槽 `[entry+0x30]==-1`（None 哨兵）→ 权重 0**
  ——逐槽确认。
- mesh-type 跳表（DAT_14304382c）：0/1 加权随机（`init_rng_u64` + `fastrand::Rng::f32` +
  `fVar40+=weight/total` 累和）；2 跳过；3/4 固定 `+0x25/+0x26`；7 计数 `[cand+0xad]++`；
  9/0xa/0x11/0x14/0x15/0x18 → `halt_baddata()`；默认透传。
- 落点：`maxps(0.01, Δ)`（0x3c23d70a）→ `floorf` → `xvy`（行 2078-2102）。
- 过滤器：`RaycastWorld::raycast()` → 结果==2 走水面；4× `bilinear_sample_at_world_xz` 求和
  clamp[0,1]；`0.0 < fVar40`（阈值 0.0）→ 写候选（行 2179-2236）。

**分歧点（需修/待证，非全判等）**：
- ① **kind-0 mesh-10 权重**：反编译写块 mesh-10 权重 = `(float)local_108 * fVar53`，且
  `local_108` 在写前被 `CONCAT44(..., (float)local_108*1.5)` 更新（行 768）→ 权重实际为
  **scale×1.5 链**；RESTORE 表 `(10, 1.0)` 与其自身注释"槽 8 权重=区域 scale*1.5"矛盾
  （若 scale=1.0 则为 1.5）。kind-2 slot-0（mesh 10）权重同构 = `scale*1.5*X`（行 955），
  RESTORE 表取 1.5——两表内部不一致。属寄存器别名歧义（类似 U④ 面），静态表取值需
  定点核对。
- ② **select_mesh 边界/兜底算子**：二进制 `do{...}while(acc<=r)` 在 **acc>r** 选中、耗尽兜底
  **首槽**（uVar25=0）；Rust `if r<=acc` 在 **acc>=r** 选中、兜底 `slots.last()`。边界概率
  与兜底行为微差（mesh 索引→资产 U，实际影响仅 gate）。
- ③ live 路径 `weighted_mesh_landing` 用 `MeshGroupWeights.group_0`（装配层资源）而非
  KIND_MESH_SLOTS 权重做落点；KIND_MESH_SLOTS 仅经 `select_mesh().is_some()` 作门。
  若装配层 group_0 权重与槽表不一致，选中概率与二进制有差（U 域）。

其余 U 登记（①网格表→资产、②kind1/default 槽表、③记录 stride 0x48 逐字节、④kind4 权重
memmove 歧义）诚实。

### 2.3 `load_clutter_meshes`（load_clutter_meshes.rs）——结构判等（closure 计数微高估）

fresh：**55** 次 `load_clutter_meshes::closure_0();` + `from_iter<ref_<ClutterMesh>,
Map<RangeInclusive<i32>,closure_env_1>>` + `insert_resource<ClutterMeshes>`。
RESTORE 表述 "~59 次" 高估 ~4 个（本会话 shared DB 计数 55）；"56 变体 register_all + insert"
成立。路径表 CLUTTER_MESH_PATHS 标 I 推断/U 登记诚实（未取 strings）。

## §3 decorator-forest

### 3.1 `gothic_half_dormer_radius` —— 判等（方向纠正成立）

radare2 fresh 0x1412ac9f6-0x1412aca29：
`cmp dword[rsp+0x28],0; movss xmm7,[0x142c2b0c8]; je default; cmp byte[rax],7; jne default;
dec al; cmp al,3; jae 0x1412acc08(panic); movss xmm7,[0x142c2b180+rax*4]`。
→ **circle && shape_kind==7 && rank∈1..3 用表 {0.25,0.3,0.29}，否则 0.38**。
RESTORE 纠正 RE-ANALYSIS §4.4 方向（"is_circle? 0.38 : 表"反了）——**成立**。
微差：rank∉1..3 时二进制 `jae→panic`（bounds check），Rust 返回默认 0.38（低影响登记）。

### 3.2 `calc_gothic_half_dormer_profile_ws` —— 结构判等 + 变换未实现（U 诚实）

fresh 719B 反编译可见 `__rust_alloc` + **两次 closure_0**（0..2 for_each）+
panic_bounds_check（unwrap 失败路径）。profile 3 档表字节核：
`gothic_half_dormer_profile_curve_1409a4cc0.c` 终写 `*puVar2 = 0xbf733333`（各档末点 y=-0.95），
country-core 表逐点一致。max_y 截断/wall-space 逐点变换（A/B lerp + C/D 法线 + inverse_lerp）、
0..2 索引语义 U 登记诚实。

### 3.3 `chimney_smoke_anim_thickness` 公式 —— 判等（FIX 落实）

fresh 全量（`review-fresh-0x140818df0-*.c`，711 行）：
- +0xd==0：`fastrand::f32()` 后再抽一次（行 554-557），目标 `r*3.0+(1-r)`、符号恒 `-1.0`（行 563）。
- +0xd!=0：目标 `r*4.0+(1-r)+(1-r)`、符号 `fVar53`（`t<0.72||0.9<=t` 时 +1.0 否则 -1.0，行 477-479）。
- 更新（行 633-646）：`fVar55=(sign * *(float*)(lVar30+0x28)) / CONCAT13(target字节重拼) + old`；
  `clamp01`（行 636-644）→ **`new=clamp01(old + sign*dt/target)`**。
- 初始厚度（行 383-384）：窗口内且 +0xd==1 → 0x3f800000（1.0）。

Rust `(target, sign)` 分派、丢弃抽签、`clamp(0.0,1.0)` 全部落实。spot 收集主循环/rng 种子/
retain 谓词 U 登记诚实。

### 3.4 `manage_tree_prefabs` —— 门控判等 + 形状源未实现

fresh 全量（`review-fresh-0x141bf16c0-*.c`，565 行）：
- 阶段1：`\|x\|>65\|\|\|z\|>65→keep=1`；6 路 bilinear，后 2 路（water/garden）任一>0→keep=0；
  前 4 路求和 clamp[0,1]（行 257-266）；rock `SETA/OR` 单布尔（行 267）；
  **形状邻近判定门 `(((bVar44||bVar46)&&(bVar43||bVar45))|bVar32)!=1`**（有构造形状 &&
  density==0 && rock==0）——点 `dist<radius+7.0`/矩形 `Rectangle2d::closest_point_on_perimeter`
  分派（行 270-283）；墙圆查询 `Circle2d::new(x,z,7.0)` + `query_with_circle_callback` 在
  `bVar33!=0` 恒运行（行 286-291）。
- 阶段2：**`*(char*)(lVar24+0x18)=='\x01'`** 门控随机 scale `r*0.75+(1-r)*0.3`（行 321-326）；
  首帧读 `record+0x1c` f32 作 jitter、scale=0、flag32=0x3f800000（行 332-334）；
  高度采样门 `(-65<=x&&x<65)&&(-65<=z&&z<65)`（行 373）；`sample+jitter*10.0`（行 388）；
  keep≠0 且 `entity>>32==0` → spawn（InteractableForestTree + TreeAmbience）、原子自增、
  `SpawnPrefabCmd`、写 `record+0x40`；keep==0 且已生成未标记 → `insert::<TreeMarkedForDeletion>`
  （panic handler，行 551-558）；帧末 `*(lVar24+0x18)=1`（行 495）。

**+0x18 处理状态字节门控（非事件存在性）——补证成立**；阶段1 墙圆恒运行修正与二进制一致。
构造形状源 ShapeParameters（core-type-gap）未实现登记诚实。

## 观察

- 全部 8 项均以本会话 fresh 反编译/反汇编为据，未发现把近似体当判等的冒充。
- 最扎实：BrickRowSine 公式、merge 合并块、add_autoclutter_on_windows 全函数、
  chimney 更新公式——均逐语句判等，且关键常量（12.9898/43758.547/0.06/0.2/0.72/0.9/
  3.0/4.0/7.0/65.0/0.01）在反编译中逐一命中。
- 需修/分歧点集中 `populate_autoclutter_regions` 权重与算子（②/③微差、①权重歧义需定点），
  及 load_clutter_meshes closure 计数微高估（55 vs ~59）。均低影响（mesh→资产 U 域内）。
- RESTORE 三份对 U 面（未初始化读、布局分歧、类型缺、路径表推断）标注诚实。

## 统计

判等 8（BrickRowSine×2、merge、on_windows 二进制、load 结构、gothic radius、chimney 公式、
manage 门控）/ calc_gothic 结构判等（0..2/profile 表/末点 -0.95；wall-space 变换 U）/ 需修
分歧点 3（populate mesh-10 权重歧义、select_mesh 边界兜底、closure 计数微高估）+ 附带登记 2
（gothic radius rank 越界 panic-vs-default、AutoClutterSpawn +0x38/+0x40 标注错位）/ 未实现
（均诚实 U 登记，无高估）。

## 证据

- `jobs/result-batch1.json` / `result-batch2.json`（ghidra fresh：0x1407eb160/0x140818df0/
  0x141bf16c0/0x140178a70/0x1422cd7d0/0x1422c0760）
- `jobs/result-from-curve.json`（0x140769760）、`jobs/result-populate.json`（0x1422d6ed0）
- `jobs/result-r2-onwindows{2,3,4}.json`（radare2 fresh 反汇编 + 跳表/常量 hexdump）、
  `jobs/result-r2-gothic-callonce.json`（0x1412ac9b0 半径选择反汇编）
- 提取文件：`review-fresh-*.c`、`review-r2-1422cd7d0.rawdisasm{,2}.txt`、`review-r2-onwindows-hexdump-*.txt`
