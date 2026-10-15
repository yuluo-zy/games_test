# backend-misc 域 算法级深挖（2026-10-15）

> 覆盖 16 章：`systems/backend/*`（12，排除 input/）+ `systems/{mod, utils, utils_intersect,
> brush_preview}`（4）。
> 证据 = `backend-misc/decomp/*.txt` + rne-read 常量。标注：O=机器码证实 / I=推断 / U=未知。

---

## 1. 处理链图

### 1.1 屏幕日志链（flush_screen_log）

```
[任意线程] log_error_to_screen(log_warning_to_screen)
    │  LOCK 静态 Mutex(0x14362bfe8) → 克隆字符串 → 按需扩容 Vec（cap-first @0x14362bff0）
    │  → 压入 {cap@0, ptr@8, len@0x10, kind@0x18}（kind 0=Error/1=Warning）→ UNLOCK+唤醒
    ▼
[ECS 主线程] flush_error_log_to_screen(ResMut<ScreenLog>)：
    LOCK；panic 中检查（0x14362c420&MAX != 0 → uVar8=!FUN_1428d86b0()）
    取走全局 Vec（ptr/len，len 清零）；is_processing!=0 → panic "Mutex poisoned"(0x142aa03b8)
    逐条：FUN_14093fe50(screen_log, text, len, 颜色表[kind]) → 释放字符串
    尾部清理未消费条目；若 panic 中且 !is_reporting → is_processing=1；UNLOCK+WakeByAddressSingle
    ▼
[ScreenLog 5 槽环] FUN_14093fe50：
    分配+memcpy 文本 → 槽 = count*0x20 {String{cap@+0x10,ptr@+0x18,len@+0x20},
    color u32@+0x28, 5.0f32@+0x2c}
    count<5 → 写槽 + count = (count+1)%5（((n+1)/5)*-5 + n+1）；count≥5 → panic_bounds_check（防御）
```

### 1.2 帧常量链（prepare_frame_constants）

```
输入：Res<GameFrameIndex>（2×u32）、ResMut<FrameConstants>、相机配置、TAA jitter 表资源、Events
① 相机槽字节@+0xc0*0x40 → 槽状态@+0x28：0 → 拷贝当前槽 ViewConstants（0x100B）；
   非 0 → 7B 调试名串 + 回退 ViewConstants（_UNK_142a9be30/38）
② GameFrameIndex：首 u32>1 门禁 → frame_constants+0x108 = 次 u32 & 3（帧号 idx&3）
③ TAA jitter：全局 0x14362c0a0==0 → jitter=(0,0)；否则表 Vec len==0 → panic(0x142ae4fa8)；
   idx = frame&3；len<=idx → idx%=len；jitter = *(u64*)(表ptr + idx*8)（8B=2×f32）
④ memcpy 0x100×2 + 4×FUN_140b18f60（ViewConstants 构建，含 0x100B/800B 帧常量块拷贝）
   + 写 jitter → 帧常量+0x100（CONCAT44(jitter.y, jitter.x)）
⑤ 排空事件队列（EventReader）收尾
```

### 1.3 草生长链（grow_grass）

```
[每帧] GrassGrowthTimer {elapsed f32@0, grown u8@4}：
① grown!=0 → 早退（只长一次）
② elapsed += Time@+0x58；elapsed <= 300.0（_UNK_142aa5140=0x43960000）→ 早退
③ 解码 15B 静态类型路径串（XOR 0x66789500ee05fddf / 0xefcd42ec / {0xa4,0xc5,0x8c}；
   源 func_0x000141563a40(&0x142cbf2d1,...) = "...RecordCobblestone..." 片段）
④ FUN_140d59dc0(&dst, 解码15B, 0xf) → 64B 摘要（后台线程计算 + CloseHandle 等待）
⑤ 解码第二 64B 静态（func_0x000141563a60(&0x142cbf295,...) XOR 8 qword：
   0x46d6abe57d16d0c2/0x6aeeb742e6e20430/0xd30f64de3cd38161/0x7011530ccd03d994/
   0x9c41e008146f38b4/0xa082c071a856b528/0x42fa0155e91f1ad7/0x4adb514658c4a79d）
⑥ len==0x40 → memcmp(解码64B, 摘要64B, 0x40)：==0 → 跳过事件（bVar66）
⑦ 不一致 → EventWriter push 8B 事件（RecordCobblestone 家族，paths/record_path_history 域）
⑧ grown = 1
```

---

## 2. 逐函数算法（O）

### 2.1 flush_screen_log（3 函数，O）

- **log_error_to_screen @0x1408c10a0 / log_warning_to_screen @0x1408c1310**：alloc+memcpy 克隆
  String → Mutex 队列压入条目 `{cap@0, ptr@8, len@0x10, kind@0x18}`（0=Err/1=Warn）；
  队列 Vec cap-first @0x14362bff0 {cap,ptr,len}，满 → FUN_140562c30 扩容。
- **flush_error_log_to_screen @0x1408c0d90**：见 §1.1。颜色表 0x142aa159c 按 kind 索引
  （0=0xff0000ff 红, 1=0xffff00ff 黄）。**虽名 flush_error 实际排空 error+warning**。
- **FUN_14093fe50 @0x14093fe50**（ScreenLog::push_line 助手）：见 §1.1。
  `0x40a00000` = 5.0f 写入槽 +0x2c（语义 I：持续时间/字号）。

### 2.2 configwatch / meshwatch / texture_watch（O）

- **configwatch @0x14085b1c0**：锁 ConfigWatch → log `'Configwatch: detected changes: N'`
  （verbosity>2 门控）→ FUN_14093b530 重载 ColorLibrary(colors.json)/SSBO → 清理；
  panic Result::unwrap（configwatch.rs:43）。
- **meshwatch @0x14085b4b0**：锁 MeshWatch → log 变更数 → 位图迭代 →
  FUN_14092d8d0 = AssetMeshLibrary::reload_mesh（load JSON；失败 log
  "Asset mesh library couldnt reload a mesh: " / 不存在 "was requested to reload a non-existing mesh path"）。
- **texture_watch @0x140aab370**：锁 TextureWatch → log `'Texture watch: detected changes: N'` /
  `'Error processing textures: '` → 位图迭代 → FUN_140b73a50 重载（纹理位图树）。

### 2.3 icc（O）

- **build @0x140a58440**：2 指令尾跳板 → 0x140b65a80（App 调度注册；Schedules 缺失 panic
  "SchedulesRequested resource does not exist in the World"）。
- **update_window_icc_profile @0x14093d830**：lcms2-5.6.0 open/transform →
  生成 LUT 写 ActiveIccProfile/SSBO；colorspace≠0xb → panic；
  失败 log "Failed creating an ICC LUT: "。
- **generate_icc_lut**（内联，I）：`terrain_1 terrain_2 wall_moss wall_bricks terrain_highlight
  night_tint` 六通道 → ColorLibrarySSBO（缺失 → panic "entry is missing from ColorLibrarySSBO"）。

### 2.4 vao_update（O）

- **build_missing_vaos @0x1408646b0**：mesh 空 → log "Mesh is empty. Skip building VAO"；
  失败 → "Couldn't build VAO: "；调 asset_libraries::vao_library。
- **rebuild_vaos @0x140864bf0**：无 VAO → "WARNING: Tried rebuilding a mesh that has no VAOs!"；
  失败 → "Couldn't rebuild VAO: "。

### 2.5 systems/mod.rs（O）

- **is_gameplay_active @0x140a90e00**：两个 Res 判 `[rcx]+0x30==1 && [rdx]+0==1`（资源身份 U）。
- **dyn_clone/as_dyn_eq/dyn_hash @0x140a90e20/70/80**：GameSystemSet 1 字节判别
  （dyn_clone 分配 1 字节）。
- **utils::add_stages @0x140945ed0**：`enum_iterator::All<GameSystemSet>` → `App::configure_sets(Update)`。

### 2.6 utils_intersect（O）

- **line_x_shape_overlap @0x14175ada0**：圆/矩形射线求交 + t∈[0,1] 夹取 + fmod 回绕；
  返回 `{hit bool, t1 f32, t2 f32}`（Option<RayBoxIntersection> 类）；
  assert "assertion failed: (0.0..=1.0).contains(&t)"（utils/curve/curve_u.rs:42）；
  签名含额外 param_3（射线源偏移）与 param_6（f32）。

### 2.7 brush_preview（3 函数，terrain-water 已证 O，保留）

### 2.8 backend/mod.rs（O）

- **add_backend_systems @0x140942c80**：注册 ~7 系统（GameSystemSet 变体 1/2/3/5/10/0x2c，
  条件系统 config+0xe7）。
- **insert_backend_resources @0x140942f80**：3 分支状态机。
- **add_backend_events @0x140943660**：6 类型注册（sysres 错位：closure$0 名挂在它上，
  真实 closure$0 地址 U）。

---

## 3. 数据结构

| 结构 | 偏移 | 大小 | 含义 |
|---|---|---|---|
| GameFrameIndex | +0x00/+0x04 | 4B×2 | 门槛（>1 才写帧号）/ 当前帧号 |
| GrassGrowthTimer | +0x00/+0x04 | f32 + u8 | elapsed / grown（is_grown 读 flag@4） |
| 屏幕日志全局队列 | 0x14362bfe8 | Mutex 字节{0/1/2}+is_processing+Vec{cap,ptr,len} | 进程级静态 |
| 队列条目 | 0x20B | {len@0, ptr@8, len@0x10, kind@0x18} | String(cap-first)+kind |
| ScreenLog 槽 | 0x20B | {cap@+0x10, ptr@+0x18, len@+0x20, color u32@+0x28, 5.0f@+0x2c} | 5 槽环，count@+0xb0 |
| FrameConstants | +0x108 | 4B | 帧号 idx&3 |
| TAA jitter 表 | 0x142ae4fc0 区 | 8B/项 | {0,3,1,2,4,...} f32 对 |

---

## 4. 常量表

| 地址 | 值 | 用途 |
|---|---|---|
| 0x142aa5140 | f32 300.0（0x43960000） | grow_grass 阈值 |
| 0x142cbf2d1 | "...RecordCobblestone..." 类型路径串（加密 15B 源） | grow_grass 摘要输入 |
| 0x142cbf295 | "_ecs::event::writer::EventWriter<country_core::...RecordCobblestone" | grow_grass 对比 64B |
| 0x142ae4fc0 | TAA jitter 表 | prepare_frame_constants 采样 |
| 0x142ae4fa8 | panic 串（jitter 表空） | prepare_frame_constants |
| 0x14362c0a0 | TAA 全局门控字节 | prepare_frame_constants |
| 0x142aa159c | {0xff0000ff, 0xffff00ff} | 屏幕日志颜色 |
| 0x142aa03b8 | "called Result::unwrap() on an Err value" | flush 锁失败 panic |
| 0x142a949f0 | 同串（configwatch/meshwatch） | watch 锁失败 panic |
| 0x142a95080/0x142a95140/0x142add810 | Configwatch/Meshwatch/Texture watch 变更日志 | watch 系统 |
| 0x142a96490/0x142a96508/0x142a96568/0x142a965c0 | VAO 四串 | vao_update |
| 0x142aa53ed | "assertion failed: (0.0..=1.0).contains(&t)" | line_x_shape_overlap |

---

## 5. 未决（U）

- game_frame_index / transient_mesh / panic_reporter_fetch_active_session 全内联（I，无独立符号）。
- is_gameplay_active 两个 Res 资源身份。
- grow_grass 8B 事件的确切类型名/消费者（疑 paths/record_cobblestone）。
- ScreenLog 槽 +0x2c 5.0f 的语义。
- insert_backend_resources::closure$0 真实地址（sysres 错位）。
