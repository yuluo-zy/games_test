# components + geometry 模块算法深挖汇总（2026-10-15）

> 任务：把 `chapters/components/**`（3 章）与 `chapters/geometry/**`（4 章）升级为算法级描述。
> 本章为集中入口：组件→required 映射、clone_behavior 归类、drawable/transient_mesh 生命周期与上传算法、geometry 判定。
> 证据方法：movabs TypeId 扫描（`tools/country-core-audit/typeid-extract.py` 方法 + 本轮新增 `comp_register_map.py`）、
> 机器码手解（rne-read --hex / llvm-objdump）、PDB public-symbols（render-architecture / render-skeleton）。
> 严禁项遵守：Bevy 胶水不计独立业务；NotFound 错误文件不作证据（均已改为机器码）。

## 1. 组件→required 映射表（44+12+1=57 组件，全量）

**结论：全部组件 register 函数仅注册自身，无任何 required components（无 `#[require(...)]`）。**

方法（O）：对全二进制 **213 个** `register_required_components` 函数逐个 disassemble（llvm-objdump），收集：
- movabs TypeId 候选常量（排除地址/小整数/hashbrown 控制组 0xf0f0…/i64::MIN 哨兵），
- 每对 (hi,lo) 常量在二进制中的归属函数（`typeid_resolve.find`），提取 `init_state<…>`/`register_component<…>`/`component_ids<…>` 泛型实参作名字证据。

每函数仅出现**自身 TypeId u128 一对（×2 次）**：描述符构造（`ComponentInfo::new` 0x140d3f870）+ `HashMap<TypeId,ComponentId>::insert`（0x140082c20，键=自身 TypeId）。**无任何第二 TypeId**（n_imm 分布：170×6 / 14×5 / 29×0，无 ≥8）。

| 组件（44 mod.rs + 12 drawable + 1 transient） | register 地址 | required 组件 | TypeId 证据 |
|---|---|---|---|
| FollowMouse | 0x140844f20 | 无 | 0x25811ca3a04e00db/0xf006cb42c80dc9f0 |
| TestCube | 0x140845610 | 无 | — |
| TreeComponent | 0x140845d00 | 无 | — |
| ConvertedPreplacedTree | 0x1408463f0 | 无 | — |
| TreeAnimation | 0x140846ae0 | 无 | — |
| TreeMoveBend | 0x1408471d0 | 无 | — |
| TreeMarkedForDeletion | 0x1408478c0 | 无 | — |
| StairsStep | 0x140847fb0 | 无 | — |
| ArrowGizmo | 0x1408486a0 | 无 | — |
| SnappingToEndPreview | 0x140848d90 | 无 | — |
| SnappingToSegmentPreview | 0x140849480 | 无 | — |
| DashedLineLength | 0x140849b70 | 无 | — |
| DontReceiveContactShadows | 0x14084a260 | 无 | — |
| SemiTransparent | 0x14084a950 | 无 | — |
| FlatLighting | 0x14084b040 | 无 | — |
| NormalMap | 0x14084b730 | 无 | — |
| OverwriteRoughness | 0x14084be20 | 无 | — |
| ClothlineCloth | 0x14084c510 | 无 | — |
| GothicWindowBrickDeformation | 0x14084cc10 | 无 | — |
| BrickColor | 0x14084d310 | 无 | — |
| Ui2DFollowMouse | 0x14084da10 | 无 | — |
| UiInactive | 0x14084e100 | 无 | — |
| Ui2dExtent | 0x14084e7f0 | 无 | — |
| Ui2dWsExtent | 0x14084eee0 | 无 | — |
| Ui2dImage | 0x14084f5d0 | 无 | — |
| UiInteractionState | 0x14084fcc0 | 无 | — |
| UiMouseInteractable | 0x1408503b0 | 无 | — |
| UiDrawOrder | 0x140850aa0 | 无 | — |
| UiNav | 0x140851190 | 无 | — |
| HistoryButton | 0x140851880 | 无 | — |
| MeshFollowTerrain | 0x140851f80 | 无 | — |
| KeepOnLevelUnload | 0x140852670 | 无 | — |
| BackgroundTerrain | 0x140852d60 | 无 | — |
| WoodenGate | 0x140853450 | 无 | — |
| BalconyDoor | 0x140853b40 | 无 | — |
| Smoke | 0x140854230 | 无 | — |
| RoofFlagCloth | 0x140854930 | 无 | — |
| WallFlagCloth | 0x140855030 | 无 | — |
| SplashParticle | 0x140855730 | 无 | — |
| TreeAmbience | 0x140855e20 | 无 | — |
| LanternLightParams | 0x140856510 | 无 | — |
| AutoClutterConvertableToDecoClutter | 0x140856c10 | 无 | — |
| AutoClutter | 0x140857300 | 无 | — |
| SittableForBirds | 0x1408579f0 | 无 | — |
| DrawTopology（枚举 enum2$） | 0x14082c3d0 | 无 | 0x541721773055ec85/0xb04edb2d6216372f |
| NaniInstanceDesc | 0x14082cad0 | 无 | 0x2a887be89e49c16b/0x300777adfd693bf9 |
| NaniActiveInstance | 0x14082d1c0 | 无 | 0x7e9fab13a9a7218f/0xe838e52690d279fd |
| DesignatedRenderPass | 0x14082d8b0 | 无 | — |
| UiPass | 0x14082dfa0 | 无 | — |
| WireframePass | 0x14082e690 | 无 | — |
| DontCastShadow | 0x14082ed80 | 无 | — |
| OcclusionCulled | 0x14082f470 | 无 | — |
| AlbedoMap | 0x14082fb60 | 无 | — |
| AlphaMap | 0x140830250 | 无 | — |
| NaniSolidTexture | 0x140830940 | 无 | — |
| PrevTransform | 0x140831040 | 无 | — |
| TransientMesh | 0x140aac640 | 无 | 0xdd365ed488e04adf/0x27040080e4d856fa |

register 函数体统一结构（bevy_ecs-0.16.0 单态化胶水）：
`enforce_no_required_components_recursion`（0x140d40d70）→ `register_component::<Self>()`（排队 0xd0B QueuedRegistration：`ComponentInfo::new` + `HashMap<TypeId,ComponentId>::insert`）→ 经 `register_component_unchecked` 递归自身 register fn（indices 命中终止）→ 递归栈 push/pop。

推论：
- 组件实体的 Transform/可见性等附着全部由外部 bundle / spawn 显式插入完成（nani maintain/spawn 路径），不依赖 required-components。
- 附带证实：游戏自有 `utils::components::transform::Transform` 的 register 同样无 required——bevy 官方 0.16 `Transform::require(GlobalTransform, TransformTreeChanged)` 在**分叉中不成立**（分叉 Transform 无 required 声明）。

## 2. clone_behavior 归类

`ComponentCloneBehavior`（bevy 0.16）：`Default=0 / Ignore=1 / Custom(ComponentCloneFn)=2`。

| 模式 | 机器码 | 归类 | 组件 |
|---|---|---|---|
| 共享 0x140002370 | `31 c0 c3` = `xor eax,eax; ret` | **Default**——组件未实现 Clone（ICF 折叠共享；register 描述符用全局默认克隆 handler） | SnappingToSegmentPreview、WireframePass、TransientMesh 及所有未单列组件 |
| 每 impl 独立 | `lea rdx,[类型表]; mov eax,2; ret` | **Custom(component_clone_via_clone::<T>)**——组件 derive Clone（autoderef 特化 `DefaultCloneBehaviorViaClone`），rdx = 克隆 handler 类型表 | ClothlineCloth(0x14084cc00)、GothicWindowBrickDeformation(0x14084d300)、BrickColor(0x14084da00)、HistoryButton(0x140851f70)、Smoke(0x140854920)、RoofFlagCloth(0x140855020)、WallFlagCloth(0x140855720)、LanternLightParams(0x140856c00)；drawable：DrawTopology(0x14082cac0)、NaniSolidTexture(0x140831030)、PrevTransform(0x140831730)；他域：BrushPreview(0x140867080)、Roof(0x1408e6af0)、CurvedHalfTimberInstanceData(0x140990af0)、ColorId(0x140b1d670)、utils Transform(0x140c9da40)、CritterLocomotion(0x140f8a840)，共 17 |

**完整性核验（O，comp_register_map.py 全二进制扫描）**：regmap 213 行中 `clone_behavior` 恰 **17 个 per-impl 符号**，与上表 17 个 Custom 一一对应、无遗漏、无多余；其余组件（含 mod.rs 44 中未列出的 35 个、drawable 其余 9 个、transient 1 个）clone_behavior 全部共享 0x140002370（Default）→ 原代码对这些组件**未 derive Clone**。

与 reconstruction 全组件 `#[derive(Clone,…)]` 标注**不符**：二进制仅上述 17 个有 Custom handler，其余（SemiTransparent、FlatLighting、FollowMouse 等）= Default → 原代码未 derive Clone。差异登记，不改源码。

## 3. `cache_prev_object_transforms`（0x14082bd80）流程

**实现方式**：纯内联指针运算系统（decomp 无外部调用）。

```
参数面（I）：param_1 = { [0]=world A（sparse instance buffer）, [1]=world B（实例表）,
              +0x14=u32 kind }
world A：+0xf8=chunk id 数组 ptr（u32 元素） | +0x100=数组尾 | +0x108=布局标志字节
        | +0x110/+0x118=两帧/槽索引
world B：+0x100=chunk 描述符表（0xa0 stride） | +0x1a8=记录表 ptr（0x48 stride）

标志(+0x108)==0 分支：                   标志≠0 分支：
  按 chunk id 迭代（跳过 desc+0x58 == 0）    直接按记录表迭代（rec+0x10=实例数）
  实例数 = desc+0x94                        记录：+0x18=实例数组 base
  记录（rec+0x18=base, +0x38=指针数组,       +0x38=指针数组, +0x40=len
       +0x40=len）
  每实例槽索引 = (desc+0x50)+8 + idx*0x10 的 u32  每实例槽索引 = 循环计数（idx）
  源槽 = *(base+0x10 + ~ptrarr[(A+0x110)]*0x30)     （同左）
  目标槽 = *(base+0x10 + ~ptrarr[(A+0x118)]*0x30)   （同左）
  帧>=len 或入口==0 → 源=0（空槽）

每实例：48B（Affine3A：3×16B 移动）源槽→目标槽；
        u32 kind（param_1+0x14）写入 *(目标槽的 base+0x28 指针 + idx*4)
```

要点：**槽定位用取补索引**（`~slot*0x30` 负偏移，槽从数组尾向前分配 = SparseInstanceBuffer 空洞复用方案）；两帧索引（+0x110/+0x118）= 当前/上一帧（推断）；`+0x108` 标志切换两套计数布局（==0 按 chunk 描述符表、≠0 直接按记录表）。**两布局差异**：==0 分支每实例槽索引取自 chunk 专属的 per-instance 描述符表（`(desc+0x50)+8 + idx*0x10` 的 u32，stride 0x10），≠0 分支槽索引直接就是循环计数（无描述符表）。语义（I）：把实例当前变换缓存进 PrevTransform 槽 + 写 kind（velocity_only/wren 消费）。

**纠错**：旧文"跳过 desc 首字段==0"——实际空 chunk 判定为 **desc+0x58 == 0**（0x82bd80 里 `*(*(B+0x100)+0x58 + id*0xa0)` 判零），非 desc+0x00；kind 写入目标是目标槽的 kind 指针表（`base+0x28 + ~target*0x30`，每实例偏移 idx*4）。

## 4. transient_mesh 生命周期图

**布局（O，裁决 closed）：** `AssetMeshLibrary.transient_meshes`（字段 @+0x18，访问器 0x140002450 `lea rax,[rcx+0x18]; ret`）= **`Arc<TransientMeshRegistry>`**；`TransientMeshRegistry` = **`std::sync::Mutex<Vec<u64>>`**（32B = 0x20：Mutex state 字节@+0、poison@+1、Vec{cap@+8,ptr@+0x10,len@+0x18}，元素 = u64 mesh handle id）。ArcInner 0x30：strong@0/weak@8/registry data@+0x10（state@+0x10、poison@+0x11、cap@+0x18、ptr@+0x20、len@+0x28）。`TransientMesh` = `{ registry: Arc<TransientMeshRegistry>@+0, mesh_handle: u64@+8 }`（16B）。

> **布局裁决（groupE #5，机器码为准）**：mesh_library 章（Arc<Mutex<Vec>>@+0x18、ArcInner 0x30、std Mutex futex）与 components 章（Mutex<Vec<u64>> 内层偏移）**描述的是同一队列**——AssetMeshLibrary.transient_meshes 字段，非两个独立结构，也非独立 Resource。两侧各对一半、各错一半：
> - mesh_library 对：Arc 包装、ArcInner 0x30 偏移、std futex Mutex 标签、字段 @+0x18；**错**：元素类型 U（占位 Handle<Mesh>）→ 实为 **u64 mesh handle id**（drain 逐 8B 元素直调 `AssetMeshLibrary::remove(h)`/`AssetVAOLibrary::remove(h)`）。
> - components 对：注册表内层偏移（state@0/poison@1/Vec@+8{cap,ptr,len}）、元素 u64、cap-first Vec、len 清 0 保留容量；**错**：总尺寸"24B" → **32B（0x20）**；"parking_lot 锁" → **std::sync::Mutex（futex 实现）**（PDB：0x1428d8750=`std::sys::sync::mutex::futex::Mutex::lock_contended`、0x1428d8810=`…::wake`）；把 `TransientMeshRegistry` 当独立 `Res` 资源 → 实为 AssetMeshLibrary 的 Arc 字段。
>
> 证据链（O）：①drain 0x1412e9e70 asm 锁 state@0、len@+0x18/ptr@+0x10/cap@+8、尾部仅清 len@+0x18；②drop 0x140899f90 decomp `[rcx]`=ArcInner、Mutex state@ArcInner+0x10、Vec@+0x18…+0x28；③PDB FunctionSystem 符号揭示 `delete_dropped_transient_meshes` 系统参数面 = **仅 `(ResMut<AssetMeshLibrary>, ResMut<AssetVAOLibrary>)`**（无 Res<TransientMeshRegistry>）；④`construct_wooden_gate_doors` 0x141210110（141212981/141212993）：`AssetMeshLibrary::add` → `transient_meshes()` 访问器取 &Arc → `new_with_registry(handle, &Arc)`。即注册表 = AssetMeshLibrary.transient_meshes，TransientMesh 组件经该 Arc 共享同一队列。

```
new_with_registry 0x140aa8200（16B，RAX:RDX 返回 TransientMesh）
  └ 实签名 `new_with_registry(mesh_handle: u64, registry: &Arc<TransientMeshRegistry>)`：
     `mov rax,[rdx]; lock inc [rax]`（Arc clone，溢出→ud2）→ `mov rdx,rcx; ret`
     （reconstruction `(registry: Arc, mesh_handle)` 参数序/按值均不符——登记）

实体移除 → TransientMesh::drop 0x140899f90（内联 deferred_delete_mesh 逻辑，独立 panic 点）
  └ std::sync::Mutex 锁（fast CAS data+0 → 1；慢路径 0x1428d8750；
     毒化→panic "called `Result::unwrap()` on an `Err` value" 0x142a98440 + transient_mesh.rs）
    └ push mesh_handle（len==cap → grow 0x140562870 8B 元素）

每帧 backend::delete_dropped_transient_meshes（参数面=ResMut<AssetMeshLibrary>+ResMut<AssetVAOLibrary>，
  持有 Arc clone）→ drain_meshes_to_delete 0x1412e9e70
  └ 锁 Mutex（毒化→同款 panic 0x142c2ef40 + 0x142c2f398）
    └ 锁内遍历 Vec（ptr@+0x10 / len@+0x18，8B 元素）
      └ 每句柄调闭包（捕获 2 个 ResMut：tick@+0x1c 回写模式）
        = AssetMeshLibrary::remove(h) 0x14092d750
        + AssetVAOLibrary::remove(h) 0x1409247a0    → 释放 GPU 网格 + VAO
    └ len 清 0（0x18..0x1f）——容量保留，非 mem::take
  └ 解锁（state==2 → WakeByAddressSingle）
```

要点/纠错：
- 整链单锁、锁内执行 GPU 删除（延迟释放设计——组件 Drop 不直接碰 GPU）。
- drain 只清 len（保留容量）：reconstruction `std::mem::take` 语义**不符**。
- 毒化=panic（`.lock().unwrap()`，三处独立 panic 点 0x142a98440/0x142adcf70/0x142c2ef40）：reconstruction `if let Ok` 语义**不符**。
- 注册表**非独立 Res**：reconstruction 把 `TransientMeshRegistry` 建模为 `Res<TransientMeshRegistry>` 资源不符（系统参数面仅两个 ResMut）——登记，不改源码。
- 闭包两 ResMut（推断）= backend 系统参数面 `ResMut<AssetMeshLibrary> + ResMut<AssetVAOLibrary>`（PDB FunctionSystem 符号证实）。
- decomp-0x140aa8200.txt / decomp-0x140002370.txt 为 NotFound 错误文件，结论来自机器码（0x140aa8200 首 16B：`48 8b 02 f0 48 ff 00 7e 04 48 89 ca c3 0f 0b`；其后的 0xd8 栈帧函数是相邻独立函数 0x140aa8210，非 new_with_registry 主体）。

## 5. instanced_wall 上传算法

布局：`InstancedWall` = `{id@+0, Vec<BrickTransformSsbo 96B>{cap@+8,ptr@+0x10,len@+0x18}, writer Option@+0x20, storage Option@+0x28}`（48B，cap-first；**修正旧行 ptr@0/len@8 表述**）。

### `update`（0x140858830，77B）
```
旧 cap@+8 满足 0<cap（cap==0 或 i64::MIN 哨兵跳过）
  → dealloc(ptr@+0x10, cap*0x60, align 4)      # 释放旧砖缓冲
新 Vec{ptr,len,cap} 拷入 self+8/+0x10/+0x18     # 整体换入
```

### `upload_to_storage_buffer`（0x140858880）
```
1. 取走 Vec：保存 cap/ptr/len，cap@+8 ← i64::MIN（moved-from 哨兵）
2. writer Option@+0x20 == None → panic（instanced_wall.rs）
3. storage（SparseInstanceBufferWriter<BrickTransformSsbo>）@+0x28：
   parking_lot 锁内部 Mutex@+0x18（0x140c81c70 acquire / 0x140c81e10 release）
   起始槽：空闲槽栈顶（storage+8 数组末 u32）或首砖参数字段(+0x28)-1（空时）
4. clear（0x1412e4760）→ 对每砖 96B push（0x1412e4450）
   失败（返回非 0）→ panic "Out of bricks."（0x142a94b90，源 instanced_wall.rs）
   其余 panic 源 sparse_instance_buffer.rs（0x142c2eb90/0x142c2eb78）
5. dealloc(ptr, cap_orig*0x60, align 4) 释放取走 Vec
6. 解锁（0x1428cdb90 唤醒路径）
```
要点：**clear-then-push 整体重写** sparse writer；writer 自持锁（与 transient_mesh 队列不同——锁在写路径上）。

### `new`（0x140858800，33B 整体拷贝构造）

```
mov rax,rcx                     # out=self
mov rcx,[rsp+0x28]              # rcx = storage（栈上第 4 实参）
mov [rax],rdx                   # self.id ← arg1
movups xmm0,[r8]                # 经指针读 Vec{cap,ptr}（24B 结构按值→指针传递）
movups [rax+8],xmm0             # self.Vec.cap@+8 / ptr@+0x10
mov rdx,[r8+0x10]               # Vec.len
mov [rax+0x18],rdx              # self.Vec.len@+0x18
mov [rax+0x20],r9               # self.writer Option ← arg3
mov [rax+0x28],rcx              # self.storage Option ← arg4
ret
```
入参 `new(id, bricks: Vec<BrickTransformSsbo>, writer: Option, storage: Option)`，确认 §5 布局（id@0/Vec@+8/writer@+0x20/storage@+0x28）。不深拷贝砖数据，只整体搬入 Vec 三字段。

### `BrickSourceId`（0x140858750 / 0x140858780 / 0x140858790）

**PDB 名修正**：0x140858750 = `BrickSourceId::from_wall_id` = **`impl From<ColorId> for BrickSourceId`**；0x140858790（impl$1::from，37B）= **`impl From<WallId> for BrickSourceId`**（两 From 同逻辑，此前 ALGORITHMS 只列 0x140858750/0x140858780，漏 impl$1::from）。

两 From 共用算法（机器码，0x80f 位掩码）：
```
ecx = id - 100000（0xfffe7960 = -100000）
if ecx <= 11 且 bit(ecx) in 0x80f（掩码位 {0,1,2,3,11}）:   # bt edx,ecx + jae
    return 查表[0x142a953f0][id-100000] = 0x80000000 | idx   # 瞬态楼梯边
else:
    return id（原样，Wall）
```
查表 0x142a953f0 = `[0x80000000,0x80000001,0x80000002,0x80000003,0x80000000×7,0x8000000b]`（i32::MIN 高判别位 + 楼梯边索引）。**纠错**：旧文"wall_id∈[100000,100011] 全为瞬态楼梯边"——实际仅掩码 `{0,1,2,3,11}` 五个 id（100000..100003、100011）映射瞬态边，且返回值是 0x80000000|idx（查表），非"100000+idx"。

`with_transient_stair_edge_id`（0x140858780，13B）：`(edge_id<<4) | self | 0x40000000`。**纠错**：常量是 **0x40000000**（bit30 家族位），旧文"0x40"错误（`or eax, 0x40000000`）。

## 6. cube / plane 判定

**两处待证项均已判定：都是 `From` 转换（PDB `impl$5::from` / `impl$1::from`），完整构建 Mesh。**

### cube `From<Box> for Mesh`（0x140a05530，3643B）
1. 读 Box{min@+0, max@+0xc}；栈上 8 角 `{pos 12B, uv 8B, color 4B}`（0x18/角）：min/max 三分量组合 ± 号常量（0x3f800000bf800000=(1.0,-1.0) 等）+ UV/白色常量。
2. 两嵌套循环写 6 面 × 6 索引（u32 Vec grow）。
3. 属性：`Vertex_Position`（0x142ac6553,15）/`Vertex_Normal`（0x142ac6562,13）/`Vertex_Uv`（0x142ac656f,9）/`Vertex_Color`（0x142ac6590）；`Mesh::set_attribute` 0x140a34c90（Vec<[f32;3]>）×3 + 0x140a355f0（Vec<[f32;2]>）。
4. 4 处分配失败 panic 源 cube.rs（0x142ac6508/0x142ac6520/0x142ac6538/0x142ac6578）；收尾 drop Mesh（0x140888b30）。

### plane `From<Plane> for Mesh`（0x140a06370，1934B）
1. 半尺寸 `size*0.5`（0x142925870）；4 角 `(±h,0,∓h)`（y=0 XZ 平面）。
2. 2 三角 6 索引（alloc 0x18 + 0x200000003=(2,3) 打包常量）。
3. 同族 4 属性（Position/Normal/Uv/Color）；panic 源 plane.rs（0x142ac6618）。

共同点：cube/plane/sphere 共用 `Mesh::set_attribute` helper 与属性名表（含 `BoxVertex_Position` 前缀 format 名）；reconstruction 未列 `Vertex_Color` 属性与 plane 的 XZ 平面取向——登记差异。

## 7. 证据登记

- 新增脚本：`tools/country-core-audit/comp_register_map.py`（TypeId movabs/rodata 批量扫描 + 二进制归属解析；本轮方法）。
- 数据：`/tmp/regmap.json`（213 个 register/clone_behavior 函数扫描结果，含 PDB 名与 TypeId 对）。
- 修正：4 个 NotFound 错误文件引用（0x140002370、0x14082bd60、0x14084cc00、0x140aa8200）→ 机器码手解证据；`From<MeshAtlasPart>`"值=0"→ part 原样保留；`InstancedWall` Vec 布局 ptr@0 → cap-first。
- round3 追加修正（见 `CLOSURE-ROUND3.md`）：mod.rs 组件计数 45→**44**（44+12+1=57）；transient_mesh 布局裁决（注册表=AssetMeshLibrary.transient_meshes Arc 字段、std futex Mutex、32B、元素 u64、new_with_registry 参数序）；cache_prev_object_transforms 空 chunk 判定 desc+0x58 + per-instance 描述符表；BrickSourceId 掩码 0x80f/{0,1,2,3,11} 与 0x40000000 常量、补 impl$1::from。
- round3 新增手解证据（capstone，RE-MCP NotFound → 机器码）：`components-geometry/decomp/hand-0x140858800-new.txt`（new）、`hand-0x140858790-from.txt`（impl$1::from）；`decomp-0x140858880.txt`/`decomp-0x140946b80.txt` 由 geometry-gap/expansion-b2 拷贝入本目录作集中证据。
- 不改 reconstruction 源码（差异仅登记）。
