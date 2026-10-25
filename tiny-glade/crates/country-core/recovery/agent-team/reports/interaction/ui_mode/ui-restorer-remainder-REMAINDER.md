# UI 域余量收口 — prefab 16 + grading_lut 5(ui-restorer-remainder)

日期: 2026-10-17
依据: fresh ghidra shared 只读(`get_functions fields=signature,decompiled_code,callees` +
`disassemble_function` + `read_memory`),全部证据落盘
`recovery/agent-team/work/ui-restorer-remainder/`(`ghidra-read*-result.json`、`fn_*.c`、`disasm-*.txt`)。
身份 B patched EXE `tiny-glade.exe`(sha256 cf048e27…6323)。
协议: `planning/2026-10-16-logic-verify-protocol.md`(8 条分歧检查表)、`2026-10-17-recovery-sop.md`。

## 汇总

| 模块 | 判等/实现 | 实现(带 seam) | 胶水 | 阻塞剩余 |
|---|---|---|---|---|
| prefab resources(4) | 4 | 0 | 0 | 0 |
| prefab systems(14) | 2(despawn×2) | 5(spawn/create_rt/create_gpu/update/hot_reload) | 4(build/register×2/clone_behavior) | 0 |
| grading_lut(5) | 4 | 1(asset_name 缺省 I) | 0 | 0 |

编译: `cargo check -p country-core --lib` **PASS**(0 errors;既有他模块 14 warnings)。
单测: `cargo test -p country-core --lib -- prefab:: grading_lut::` **24 passed / 0 failed**
(prefab resources 4 + systems 10 + grading 10)。

## prefab resources — `src/resources/prefab.rs`

| VA | 函数 | verdict | 分歧点/证据 |
|---|---|---|---|
| 0x1408e2540 | `PrefabLibrary::add` | **判等** | fresh:`insert<PrefabName,Prefab,RandomState,Global>` + 被替换旧值逐 `PrefabPart` `drop_in_place`;Rust `self.inner.insert`(旧值 drop 同序)。 |
| 0x141552d70 | `get<&PrefabName>` | **判等** | 空表直 miss;`as_name_ref`→`hash_one<&str>` SIMD probe+memcmp;miss → `from_iter<Keys>` 收 `Vec<&PrefabName>` + `panic_fmt`。panic 文案 `No such prefab {:?}; available: {:?}`(0x142c6c7a0/7c0 逐字)。 |
| 0x141552ff0 | `has<&PrefabNameRef>` | **判等** | 表 `+0x18` 为 0 时跳过哈希直接 false;否则 `hash_one<&str>`+memcmp 命中返回;= `contains_key`。 |
| inline | `iter` | **判等** | PDB 内联宿主 `load_mesh_atlases`;= `inner.iter()`。 |

**类型核对结论**(任务第 1 步):`asset_libraries/` 无 prefab 模块;权威类型在现有
`resources::prefab`(`HashMap<PrefabName, Prefab>`,RandomState),`PrefabName/Ref` 在
`resources/render/mesh_atlas_library.rs`(已有,按机器码核对)。**未新增 asset_libraries 类型**(不编造)。

`PrefabPart` 本轮按 fresh 收紧为 **0x98 B 锚定语义**(+0x18 Option / +0x20 kind 名指针 /
+0x28 tag=4 / +0x30+0x38 名 String / +0x40 rank / +0x84 kind 字节);未锚定区以 seam 表出。

## prefab systems — `src/systems/prefab.rs`

| VA | 函数 | verdict | 已还原要点 / 分歧点 / 解锁条件 |
|---|---|---|---|
| 0x140840430 | `PrefabPlugin::build` | **胶水** | 注册 `SpawnPrefabCmd`/`DespawnPrefabCmd`、`add_systems` 链、`insert_resource<PrefabsToDelete>`。Bevy 插件注册,不重建。 |
| 0x1408446b0 / 0x1408e5d10 | `register_required_components` ×2 | **胶水-legitimate** | 反编译体 = `ComponentInfo::new` + 注册表扩容 + 递归注册,无业务分支。 |
| 0x140002370 | `impl$3::clone_behavior` | **胶水** | 3B `return`;与 `WireframePass::clone_behavior` ICF 同址。 |
| 0x140840610 | `spawn_prefabs` | **实现(核心判等,ECS 接线 seam)** | GlobalId 去重(warn `Issued creation of a prefab {:?} on the same frame as issuing its deletion!`,0x142a941xx 逐字)→`PrefabLibrary::get`→`create_prefab_gpu_instances`→逐 part(stride 0x98)音频触发(Option 门 + `tag==4 && 首4字节"Tree"`,否则 panic `Unknown audio trigger kind … in prefab …`)→`create_prefab_rt`→输出 `(PrefabInstance, PrefabInstanceData)`。**seam**:`SpawnPrefabCmd` 载荷未锚定(以 `PrefabSpawnRequest` 接收)、`Commands::insert`/`PrefabsToDelete` 清空为 host。解锁:事件载荷字段契约(asset/forest 域)。 |
| 0x140841550 | `create_prefab_rt` | **实现(数值判等)** | kind@+0x84 == 2 → 跳过;颜色 `sqrt(c)*255+0.5` 饱和 + `r|g<<8|b<<16`(复用 `rt::color_to_u32`,与机器码同);矩阵按 `sinf/cosf(rot*0.5)` 逐 16 项写序(绕 Z,`[12..16]=t.xyzz`);kind&1 → `0.5*min(extent)`(NaN 用 `f32::min` 语义)。**seam**:0x48 B 延迟命令记录(`10.0`、`panic` handler)与 `{2,handle}` 事件为宿主队列入队(登记)。 |
| inline | `create_prefab_audio_trigger` | **实现** | `Aabb3::scaled(splat(scale))`→`translated(translation)`→`AudioTriggerLibrary::add_trigger(Primary)`;记录写 0x28 元素(+0 Aabb3、+0x18 Option、+0x20 键)。 |
| 0x140841b00 | `create_prefab_gpu_instances` | **实现(结构判等)** | `MeshAtlasLibrary::get_by_name<&PrefabNameRef>`;Trace 级 `Creating {} parts for prefab {:?}`(target 0x1d=29);每 `PrefabMeshAtlasPart` 调 `GpuWorld::create_instances<PlacementDataSsbo>`,`mesh_index=part_index`。**seam**:每 part 的 cull/data 单元素切片的寄存器来源(closure_env$0 载荷)未完全锚定,以 `base_cull`+`data` 参数表出。 |
| 0x140841d60 | `update_prefabs` | **实现(核心判等,RT seam)** | GPU Vec 逐 `update_instance_data`;RT 以同一矩阵重算并做变更检测;音频 `scaled/translated`→`update_trigger_shape`,键失效 → `unwrap` panic。**seam**:`RtMesh` 矩阵字段(rt 域 0x50 B 布局无矩阵)以 `(handle,matrix)` 返回;GPU position/radius 寄存器来源登记。解锁:rt 域补矩阵字段 + `RtWorld: Resource`。 |
| 0x140842d60 | `despawn_prefabs` | **判等(结构)** | `EventReader<DespawnPrefabCmd>`(+0x08 packed Entity)→`Query<&mut PrefabInstanceData>` 命中 → `despawn_prefab_contents` → `EntityCommands::despawn`。**seam**:原生命中分支里的外部计数/GPU 数据写(资源归 host)。 |
| 0x140843390 | `despawn_prefab_contents` | **判等(结构)** | GPU Vec 遇 `kind==0x1b` **break**(终止哨兵),否则 `remove_instance`;RT Vec hashbrown `remove_entry`(= `HashMap::remove`,Arc drop);音频 Vec Option 判别(`2`=None→return)后 `remove_trigger`。**seam**:RT 移除后的 +0x38 脏位与 refcount 分支(外部资源)。 |
| 0x140843740 | `hot_reload_prefabs` | **实现(核心判等)** | 先收 `HashSet<PrefabName>`/`HashSet<GpuWorldSubsetKind>`;实例命中(名∈集合 **或** 任一 GPU 记录 subset∈集合)→`despawn_prefab_contents`→`get`→重建(spawn 同段)。**seam**:`PrefabWatcher`(notify)与两事件集合的生产(startup 域),RT 写回同上。 |

**布局锚点**:`PrefabInstanceData` 0x58 B(`+0x08/+0x20/+0x38` 三 Vec、`+0x48/+0x50` PrefabName);
变换是独立 0x30 B 组件(`+0x00 Vec3,+0x0c scale,+0x10 rot`);GPU 记录 0xc、音频记录 0x28。
**跨文件登记**:`components::trees::PrefabInstanceData` 与本文件同名不同体(旧 0x20 语义),
集成需统一(不在本任务文件范围)。

## grading_lut — `src/resources/grading_lut.rs`

| VA | 函数 | verdict | 分歧点/证据 |
|---|---|---|---|
| 0x140b00bc0 | `GradingLut::load_unwrapped` | **重建→判等(签名纠正)** | fresh disasm 订正:签名是 `(device, name: String) -> GradingLut`;**函数体内** `format!("assets/luts/{}.png")`(片段 0x142ae6bf6 len12 + 0x142ae6c02 len4),再 open→decode→to_rgba8→`new_3d([16;3],0x25).sampled()`→`create_texture("grading lut")`;open/decode/create 失败全是 `unwrap` panic(尾部 `MOV RAX,RSI; RET` 无 Result 判别子)。旧实现 `(device,path,name)->Result` 属**返回结构不符+panic-vs-return 反**,已改;`load_unwrapped_from_path` 保留为同链测试内核。 |
| 0x140b01df0 | `GradingLuts::load` | **实现→判等(语义模型)** | `read_dir(..).unwrap()`→`entry.unwrap()`→extension/file_stem(非 UTF-8 跳过);`"cube"`→`load_cube`(Err → `log::error!(target:"country_core::resources::grading_lut","Failed parsing LUT {}: {}", path, err)`,0x142ae32b0/0x142ae32f0 片段);`"png"`→`load_unwrapped`(原版无 Err 分支,等价 `.unwrap()`);其它 → `log::warn!("Unsupported LUT: {}", path)`(0x142ae3274)。返回值原生为**两个并行 Vec**(0x28 元数据 / 0x38 纹理,闭包每次成功各压一条);Rust 以 `grading_luts.metas/textures` 表出。**登记**:原生纹理记录 0x38 B 布局(本 crate `Texture` 为 0x10 语义模型)。 |
| inline | `lut_name_to_asset_name` | **判等** | `format!("grading_lut:{name}")`,前缀 O(0x142ae6c60 侧)。 |
| inline | `asset_name` | **实现(默认分支 I)** | PDB inline 依赖 `Option::map_or_else` + `lut_name_to_asset_name`;落地 `asset_name(Option<&str>)`,`None` 缺省空串(内联缺省分支未单独锚定,登记)。 |
| 0x140b00b20 | `ColorGradingState::flattened` | **判等(复核勿改)** | flag@+0x28==0 → 克隆 name+读 4 f32;否则 "default"(len 7)+{1,0,1,1};两种都写 flag=0;其余字段原样。源码一致,未改。 |

## 登记/解锁条件

1. `SpawnPrefabCmd`/`update_prefabs` 查询载荷字段契约(GlobalId/变换/名称)→ spawn 的 ECS 接线。
2. `resources::rt`:`RtMesh` 矩阵字段(create/update 写面)、`RtWorld: Resource`、+0x38 脏位语义 → update/hot_reload/despawn 的 RT 写回。
3. `create_prefab_gpu_instances` 每 part 的 cull/data 单元素切片来源(closure_env$0 寄存器载荷)。
4. `PrefabWatcher`(notify crate)+ `PrefabsModifiedCmd`/`MeshAtlasesModifiedCmd` 生产 → hot_reload 事件收集。
5. `startup::load_prefabs_impl`(运行时 `Prefab`/`PrefabPart` 的真实生产者)未恢复 → `PrefabPart` 的 mesh/音频/RT 字段最终以该生产链为准复核。
6. `asset_name` 缺省分支;原生 `GradingLuts` 纹理 0x38 B 布局。
7. 跨文件:`components::trees::PrefabInstanceData` 与 `systems::prefab::PrefabInstanceData` 统一。
8. `PlacementDataSsbo` 本轮补 `Default`(原版 `InstanceElement` 要求;`resources/mod.rs` 1 行,属本域数据合同)。

## 纪律声明

- 只改 prefab/grading 域文件 + `resources/mod.rs` 的 `PlacementDataSsbo` derive;未动 gui/toolbar/image_formation/photo_mode/rt/history。
- 宏胶水不重建;seam 处均写明原偏移/VA/解锁条件,不编造字段。
- 未以“编译通过”宣称完整:spawn/update/hot_reload 的 ECS/RT 宿主面按 seam 登记。
