# process_tree_cmds 业务逻辑分析（B 主基线，反汇编级证据）

2026-10-13。函数 VA `0x142267430`（B patched exe），PDB size 4758（0x1296），Ghidra 函数体
`[[0x142267430,0x142267658][0x142267660,0x142267a3d][0x142267a40,0x142267b20]
[0x142267b30,0x142267b6d][0x142267b70,0x142267ba5][0x142267bb0,0x142268298]
[0x1422682a0,0x1422682f2][0x142268300,0x1422685e5]]`。源文件
`crates/systems/tree-tool/src/process_tree_cmds.rs`，PDB 行号表覆盖源码 24–201 行
（skeleton/B/crates/systems/tree-tool/src/process_tree_cmds.rs.pdb.jsonl.gz，offsets 相对函数入口）。

证据来源（本次精读）：
- **reva**（B 持久库，worker `tree-owner`，只读脚本）：`0x142267430` 全 939 条指令反汇编、
  `0x14226b1a0` ui_move_tree 反汇编/反编译、TreeStorage::get/insert/remove（0x140a38460/
  0x140a38560/0x140a385a0）反汇编、ActiveSession/WaterRaster 相关常量与 .data 字符串读取。
- **radare2**（RNE 发行原件 oracle，`0x142267430` 及三处分支头反汇编）：与 reva/B 反汇编逐条一致，
  确认地址/指令未被 B patch 改动；另读 .data 常量与字符串。
- **PDB 行号表**：offset→源码行 24/43/45/46/47/51/52/53/54/56/58/61/63/74/78/81/83/85–91/
  99–130/143–176/182–201。

## 系统角色（硬证据：调用者泛型，不变）

```rust
pub fn process_tree_cmds(
    mut tree_storage: ResMut<TreeStorage>,             // arg1 RCX
    mut prefab_instances: Query<&mut PrefabInstanceData>, // arg2 RDX
    mut move_cmds: EventReader<MoveTreeCmd>,           // arg3 R8
    mut delete_cmds: EventReader<DeleteTreeCmd>,       // arg4 R9
    mut insert_cmds: EventReader<InsertTreeCmd>,       // arg5 [RSP+0x28]=RBP+0x240
    mut spawn_prefab: EventWriter<SpawnPrefabCmd>,     // arg6 RBP+0x248
    commands: Commands,                                // arg7 RBP+0x250
    mut play_sound: EventWriter<PlayTreeSoundCmd>,     // arg8 RBP+0x258
    active_session: Res<ActiveSession>,                // arg9 RBP+0x260
    water_raster: Res<WaterRaster>,                    // arg10 RBP+0x268
    mut despawn_prefab: EventWriter<DespawnPrefabCmd>, // arg11 RBP+0x270
)
```

参数映射来自 prologue 保存与各处读取：arg1=RBX、arg2=R14、arg3=RDI、arg4=[RBP+0x10]，
arg5–11 为入口栈参数 [E+0x28..0x58]（E=entry RSP，RBP=E-0x218 换算到 RBP+0x240..0x270）。

## 三段命令分支（证据，处理顺序：Move → Insert → Delete）

处理顺序与源码行号顺序一致（Move 43–74 → Insert 74–176 → Delete 182–201），
**推翻此前"Insert→Delete→Move"推断**。每段独立迭代对应 Events 双缓冲（+0x8/+0x10、
+0x28/+0x30 两个 Vec，+0x40 计数），事件槽 stride 即各命令 EventInstance 大小。

### 1) Move 分支（offset 0x1BC–0x4B8；VA 0x1422675ec–0x1422678e8；源码行 43–74）
- 循环头 0x210（VA 0x142267640），逐事件读：index @slot+0x8（8B）、position @slot+0x10/+0x18。
- `TreeStorage::get(&mut self, index)`（0x140a38460，@0x274）；`None`→日志
  `"MoveTreeCmd references a non-existing tree"`（0x143035018，len 0x2b，日志记录@0x143035048），
  @0x2A4–0x354。
- `Some`→ 用 TreeEntry 值（*entry，8B：低32=prefab 表下标、高32=entity）定位 PrefabInstanceData
  （经 Query 表，stride 0x14=20，@0x35E–0x424），把新 Vec3 写进组件 +0（@0x424）。
- `inverse_lerp`（0x140c904a0，@0x447）+ `bilinear_sample_at_world_xz<RasterBufferRef<512>>`
  （0x140a14df0，@0x468）→ `0.0 < 采样值`（SETA，@0x47C）＝"在水中"。
- 写 PlayTreeSoundCmd（kind=1，@0x4B8 前）。

### 2) Insert 分支（offset 0x5F9–0xDD1；VA 0x142267a29–0x142268201；源码行 74–176）
- 循环头 0x5F9（VA 0x142267a29），逐事件（stride 0x48）读字段。
- `is_dragging(&ActiveSession[+0x58])`（0x14092e6a0，@0x67A）：非拖拽走默认路径（@0x700，
  [RBP+0xf8]=0），拖拽中则取事件 flags（@+0x2c 低2位）。
- 树名 String 匹配：`memcmp`/SIMD 比较已知 prefab 名（"clearing/naked_tree/..." 等，
  .data 0x143034d30–0x143034d6f；"FAB_NAME"@0x143034d70），@0x6A2–0x790。
- `deny_load(&ActiveSession[+0x58])`（0x140a9fa70，@0x79E）+ `is_corrupted(&ActiveSession[+0x58])`
  （0x140ae8320，@0x7A8），与名字匹配共同门控生成。
- `Commands::spawn_empty`（0x140d146a0，@0x7F2）→ 经 Commands 插入组件
  TreeComponent（@0x82C，0x30B）、TreeAnimation（@0x86D，0x30B）、TreeMoveBend（@0x8CD，0x28B）、
  TreeAmbience（@0x935，0x28B）。
- prefab 名选择：`random_tree_prefab`（0x14226a170，@0xD16）或 `PrefabName::new`
  （0x141556080，@0xA02），"MISSING_TREE_PREFAB_NAME" 兜底（.data 0x143034d80）。
- 写 SpawnPrefabCmd（0x50B 槽，@0xA9D）；`TreeStorage::insert(&mut self, index=event@+0x38)`
  （0x140a38560，@0xB10，更新 next_index=max(_,index+1)）；`bilinear_sample`（@0xB26）；
  写 PlayTreeSoundCmd（kind=0，@0xBA1）。
- 条件路径（insert 返回非零高32，@0xC03–0xE46）：日志 + 写 DespawnPrefabCmd（0x10B 槽，
  @0x5D0 附近块 0x142267a00，槽值=insert 返回值）。

### 3) Delete 分支（offset 0xDD1–0x10CE；VA 0x142268201–0x1422684fe；源码行 182–201）
- 循环头 0xDD1（VA 0x142268201），逐事件（stride 0x10）读 index @slot+0x8。
- `TreeStorage::remove(&mut self, index)`（0x140a385a0，@0xEF3）。
- `Entities::contains(entity)`（0x140ceafa0，@0xF1B）→ 存在才继续。
- 经 Commands 推入 TreeMarkedForDeletion 命令（@0xF3E，0x20B 命令）。
- 查 prefab 实例位置 → 写 PlayTreeSoundCmd（kind=2，@0x10A4 块 0x1422682a0）。
- **Delete 分支不发送 DespawnPrefabCmd**（推翻推断）：只打 TreeMarkedForDeletion 标记 + 移除音效，
  实际预制体销毁由同文件的 `despawn_trees` 系统负责（证据边界：despawn_trees 的 DespawnPrefabCmd
  发送未在本函数确认，PDB 符号在 B 库被并入相邻函数，待单独核对）。

## 命令字段布局（证据：事件槽 stride 与槽内偏移，radare2/reva 一致）

事件槽 stride：move 0x20、delete 0x10、insert 0x48、spawn 0x50、despawn 0x10、sound 0x20
（循环内 `lea r15,[rcx+0x20]` / `lea r14,[r12+0x48]` / `shl ...,0x4` 等）。

游戏字段相对槽起始的读取/写入偏移（证据）：

| 命令 | 槽 stride | 槽内字段 |
|---|---|---|
| MoveTreeCmd | 0x20 | index 8B @+0x8；position Vec3 @+0x10（x,y 为 MOVSD，z @+0x18） |
| DeleteTreeCmd | 0x10 | index 8B @+0x8 |
| InsertTreeCmd | 0x48 | name String @+0x8（ptr）/ +0x10（len）；position Vec3 @+0x18/+0x20；height f32 @+0x24；f32 @+0x28；flags u32 @+0x2c（低2位拖拽门控）；byte @+0x30；index 8B @+0x38；@+0x40..0x48 未读 |
| SpawnPrefabCmd | 0x50 | name String @+0x0（ptr）/ +0x8（len）；u64 @+0x10；position Vec3 @+0x18/+0x20；f32 @+0x24、@+0x28；flags u32 @+0x2c；1.0 @+0x30；0 @+0x34..0x40；u32 @+0x44；count/id @+0x48 |
| DespawnPrefabCmd | 0x10 | u64 @+0x8（Insert 分支写 TreeStorage::insert 返回值） |
| PlayTreeSoundCmd | 0x20 | count/id 8B @+0x0；kind u8 @+0x8（insert=0/move=1/delete=2）；in_water bool @+0x9；flag bool @+0xa；height f32 @+0xc；position Vec3 @+0x10 |

索引字段在 8 字节槽位以完整 8 字节读取并作 TreeStorage hash 键（get 内
`cmp qword ptr [entry], rsi`），**index 为 u64**（修正草稿 u32）；`next_index` 亦 u64。

**bevy 事件框架（依赖内部，未完全钉死）**：sound 事件槽 @+0 写入的 8B 为事件计数
（写入来自 [Events+0x40]，即 bevy EventInstance 的书签/id）；而 spawn 事件把 count/id 写在
@+0x48。说明 bevy send 的框架放置并不统一（依赖内部细节），游戏 payload 字段偏移以本表
槽内偏移为准；重建时把 @+0x0 前的 8B 当作 bevy EventInstance 框架保留（依赖复用，不还原）。

## TreeStorage 布局（证据：0x140a38460/0x140a38560/0x140a385a0 反汇编）
- +0x18：u64 计数/非空标记（get 先判 0→None）。
- +0x20：hashbrown（hashbrown SwissTable）hash map，键 u64（index），值 8B。
- +0x30：next_index u64；`insert(index)` = hash 插入 + `next_index = max(next_index, index+1)`。
- `get` 返回值（*entry，8B）= { 低32：prefab 表下标；高32：entity }。
- 注意：**+0x298 字节标志属于 WaterRaster（Res 内容），不属于 TreeStorage**（prologue
  `as_inner_mut()` 作用于 arg10=water_raster，见下）。

## ActiveSession 与 WaterRaster（证据）
- **WaterRaster（arg10，prologue @0x282）**：内容为双缓冲 RasterBuffer——指针对 @+0x8/+0x10、
  @+0x20/+0x28，字节标志 @+0x298 选当前读缓冲；选中的两指针存入栈，供
  `bilinear_sample_at_world_xz(context, x, z)`（0x140a14df0）在 Move（@0x468）与 Insert（@0xB26）
  两处采样水面；`0.0 < 采样值`＝树入水，写入 PlayTreeSoundCmd.in_water。附带常量 130.0
  （0x142a7b658）作采样参数；inverse_lerp 阈值 0.5/0.9（0x142925870/0x142a7c88c）。
- **ActiveSession（arg9，prologue @0x543）**：内容 +0x58 被取址（RBX=&ActiveSession[+0x58]），
  Insert 分支调用其方法 `is_dragging`（@0x67A）、`deny_load`（@0x79E）、`is_corrupted`（@0x7A8）
  门控种树（拖拽中/加载中/存档损坏时相关逻辑）。三方法同址源于 PDB 对共享类型标注不清，
  实际类型待定（unknown）。

## PrefabInstanceData 关联（证据）
- Move 分支**按下标定位目标实例**：MoveTreeCmd.index → TreeStorage hash map 值
  {prefab_index u32, entity u32} → 以 prefab_index 索引 Query 表（stride 0x14=20，
  @0x35E），校验表项 entity 与高32一致 → 经 Query fetch 把新 Vec3 写入组件 +0。
- PrefabInstanceData 组件：position Vec3 @+0（12B），f32 @+0xc（inverse_lerp 读取）。

## 剩余 unknown（不伪造）
- InsertTreeCmd 槽 @+0x40..0x48（payload 尾 8B）未读字段。
- SpawnPrefabCmd 72B payload 除 name/position/flags 外的字段语义（@+0x10 u64、@+0x30=1.0、@+0x44 u32 等）。
- ActiveSession[+0x58] 的实际 Rust 类型（三方法同址）。
- Insert 分支 DespawnPrefabCmd 触发条件的确切含义（`insert` 返回高32非零＝槽位已占用，
  疑似替换/重放时销毁旧预制体）。
- despawn_trees 系统（同文件，PDB VA 0x142268A30）的 DespawnPrefabCmd 发送细节（B 库中该地址
  与相邻函数边界不符，未在本轮核对）。
- TreeStorage::insert 内部 hash 插入调用 0x140eec970 的语义。
