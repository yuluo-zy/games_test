# camera-time 域 U 项闭合（tg_a6，第二轮，2026-10-14）

工具：v2 索引（sysres-procedures-v2.json）+ 定点反编译（tg_a6r2/tg_a6r3）+ rne-read
（--hex/--unks/直读）+ **pdata 函数边界表**（0x14363xxxx 区域，{begin,u32},{unwind,u32},{end,u32}
12B 条目，116,270 条全函数边界）。结论标注：CLOSED / PARTIAL（卡点注明）/ STILL-U。

## 1. WalkCamera::rotation 0x140994310 的 16B 拷贝语义 —— CLOSED

- 反汇编：`mov rax,rcx; movups xmm0,[rdx+0x18]; movaps [rcx],xmm0; ret`（sret 16B）。
- xrefs：0 条（仅经 vtable 调用，无直接调用点）。
- 决定性证据：`WalkCamera::translate_screen_space` 0x140994700 在 **+0x18..+0x24 读四元数**
  （q={x@+0x18,y@+0x1c,z@+0x20,w@+0x24}）做 `rotation * movement` 旋转；`CameraRig::update`
  0x140993d30 写 +0x18..+0x34 为 7 个 f32 变换（+0x24 双写幂等）。
- 结论：**游戏 dolly 分叉的 CameraTransform = {rotation: Quat @+0x18 (16B), position: Vec3
  @+0x28 (12B)}，rotation 在前**（stock dolly 0.6.0 为 position-first，分叉改序）。
  `rotation()` 返回的 16B = 完整四元数 ✓。骨架的 `rig.final_transform.rotation` 语义正确。

## 2. TodLightingSnapshot 两 u64 字段（@+0x20/+0x28）—— CLOSED（语义）

- ron 序列化器 0x14007ff80（v2：resources/time_of_day.rs impl Serialize）字段调用序列：
  `"sun_sky"`(7B@0x142930744, 值=结构体基址 +0x0，嵌套 8 f32) →
  `"base_fog_thickness"`(18B@0x14293074b, 值 @param_1+0x30) →
  `"fog_tint"`(8B@0x14293075d, 值读取自 +0x20..0x30 区) →
  `"speed_bias"`(10B@0x142930765, 值 @param_1+0x40)。
- 结论：**2×u64 @+0x20/+0x28 = `fog_tint`**（16B 打包雾色，不参与插值——snapshot_at_tod
  0x140b067d0 直接拷贝 ✓ 与"离散分量"一致）。完整快照 = {sun_sky @+0x0 (8×f32 太阳+天空色),
  fog_tint @+0x20 (16B 打包), base_fog_thickness @+0x30 (4×f32), speed_bias @+0x40 (f32)}。
- 卡点：fog_tint 的内部打包格式（RGBA8×2 / RGBA32F×4 / …）仍 U（无消费端反编译）。

## 3. suncalc 经纬度常量 + 日期位打包 —— PARTIAL（核心已闭合）

- **纬度不是常量**：`FUN_1417ab4b0(out4×f64, ms, (double)state[+0x10], 0)` —— lat =
  播放态资源 f32 @+0x10、**lon = 0.0**。@142ad5910..5984 那组 f32（0.762443/0.647056/
  0.418681/−0.493343/0.551646/0.834078/0.806015/0.591895）= **太阳方向的正交基旋转
  常量**（方位/高度 → 3D 方向的两组基向量），非经纬度。
- **日期位打包**：`encoded = day_of_year | 0xFCE00`，0xFCE00 = **2023·512**（2023<<9，
  精确相等）；day_of_year ∈ 1..365（越界 `Result::unwrap()` panic）。解码器 FUN_140caff50
  出 unix 秒 ×1000（ms）；再加 `hour·3600000` ms（3600000@142ad58f0 实证）。即太阳日期
  = 固定基准年 2023 + 一年中第 N 天（确定性昼夜），或 (year−1)<<9 编码 year=2024 的别名。
- 会话序列化还有 `"latitude_degrees"`/`"sun_xz_rotation"` 字段（0x1400802c0/0x140b3bc00，
  {time_of_day, sun_xz_rotation, latitude_degrees} 三字段小结构）——名字含 degrees 但
  suncalc 调用原样传值（suncalc 要弧度）——**卡点**：lat 字段默认值与其存储单位未定。

## 4. MainCamera::reset(0x140911280) 全貌 —— CLOSED（实为 zoom）

- v2 名 = `MainCamera::zoom::he9cf30add4fb44de`；pdata 边界 0x911280–0x9114c6（0x246B）。
- 全函数手解（582B 机器码）：入参 (rcx=&mut self, xmm1=amount, r8=&anchor: Vec3)。
  ① amount 有限性检查（|bits|&0x7fffffff > 0x7f7fffff → 错误路径 0x9114a0）；
  ② anchor 三分量有限性检查（各自 setl 后 AND，任一非有限 → 0x9114a0）；
  ③ 算 anchor 水平距离 sqrt(x²+z²)；④ `mode@+0x4d0` jumptable 分发：Normal→self+0x100、
  TiltShift→self+0x1b8、TopDown→self+0x390，其余模式走默认路径；
  ⑤ lower 常量查表 table2[(mode==1)?1:0]（TiltShift 专用下界）；`[body+0x18]==1`
  （active_rotation Option 判别）分支：有旋转 → `factor = exp2(clamp(amount·−0.08, −1, 1))`、
  `target_arm_length@+0x20 = clamp(factor·target, lower, radial_max@+0x64)`；无旋转 →
  offset=(pos+pending)−anchor 按 factor 缩放写回 pending（骨架 normal.rs::zoom 对应两分支 ✓）。
  ⑤ 错误路径 0x9114a0：lea r8,[const]+movaps×2+call（日志/panic 辅助）。
- 骨架 `normal.rs::zoom` 与证据一致；MainCamera 层 dispatcher 缺失（登记 REVIEW 缺失项）。

## 5. time_of_day 的 param_3 / param_10 / param_11 —— PARTIAL

- param_3 = `Res<单 f32 资源>`，在函数尾加进光照结构 [0x15]（+0x54）字段
  （`light[0x15] = 快照派生值 + *param_3`）。候选：TodEasing 分量 / 太阳强度 / TimeOfDay
  小时——类型名 U（无符号/无序列化引用实证）。
- param_10/param_11 = 两个 `EventReader<T>`（标准双缓冲 {counter, start@+0x18, end@+0x38,
  buf@+0x28, 0x10B 步长} 各修剪一次），两者皆空 → 直接写 piVar29[8]；否则
  FUN_140c90670(piVar29+8, fVar52) 合并写太阳可见标志。事件步长 0x10、内容未读
  （只判空/非空）→ 是"太阳可见覆盖/会话事件"族。类型名 U（候选：SunVisibilityCmd、
  NewSessionStartedCmd 类）。卡点：无符号可查，需消费端 xrefs（成本高，放弃）。

## 6. 模式 6..12 变体名 —— PARTIAL（跨域交叉完成一半）

- **CameraRotationPivot 与 CameraMode 是两个东西**（第二轮纠正）：0x1408b99e0/0x140914ac0
  的 enum_iterator::Sequence 属 **CameraRotationPivot**（7 变体：CameraRotationPivot/
  CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain，
  ron 0x14003ac90 实证）。previous 表 [6,0,1,2,3,4,0,0] 即其 previous(0)=6 回绕 ✓。
- **CameraMode 核心 6 变体 + 模式字节 0..12 共 13 槽**：focus 表 @0x142aabb68 13 项
  {0x70,0x128,0x1a8,0x248,0x300,0x388,0x8c,0x8c,0x144,0x1c4,0x264,0x31c,0x3a4} 与
  lens 表 @0x142aabb98 13 项 {0x8c,0x144,0x1c4,0x264,0x31c,0x3a4,0x08,0x20,0x14,0x2c,
  0x08,0x14,0x0c}（focus=body 内 +0x70 槽、lens=body 内 fov 槽；槽 6..9 落在 Normal body
  +0x8/+0x10/+0x14/+0x2c 区）。
- **PhotoModeCameraMode = 4 变体**（ui-input 域 photo_mode.rs，本域交叉验证）：
  `next` 0x140b59720 表 {0→1,1→2,2→3,3→0}；`next_demo` 0x140b596b0 表 {0→1,1→2,2→0}，
  **变体 3 在 demo 直接 panic**（jumptable + "…" @0x142af1188/file @0x142af11c8）。
- 推论（I）：模式字节 6..9 = 拍照 4 子模式（6+photo_variant），拍照槽复用 Normal body 的
  focus/lens 字段；槽 10..12（lens {0x08,0x14,0x0c}）另有其主（表尾数据外溢或第三组模式）。
  **卡点**：6..12 各槽的名字与用途未获符号级证据；paused_main_camera_update 的 case 7..11
  反编译损坏（bad instruction data），无法逐槽对 body。→ 交 ui-input/中央裁决。

## 7. GameTime 头部布局 —— PARTIAL

- 实证：{+0x0/+0x8 = game_time 累计（secs u64 + nanos u32，weather 以 secs+nanos/1e9 用）、
  +0x10/+0x18 = delta Duration（secs u64 + nanos u32，time_update 读）、+0x28 = delta f32
  （weather/time_of_day 读）}。
- 卡点：+0x20..0x27 与 +0x30 之后字段语义（候选：f64 秒/时间倍率/暂停标志），无消费端
  证据；不再扩展（代价/收益不划算）。

## 附：pdata 函数边界表用法（可复用技巧登记）

.rdata/.pdata 区 0x14363c000 起 12B 条目 {begin u32, unwind u32, end u32}（按 begin 排序，
116,270 条），直接读文件 RVA 0x3632400 起 0x154c00 字节二进制搜索即可枚举任意函数的
精确起止（含 Ghidra NotFound 函数）。本域实例：zoom = 0x911280–0x9114c6；mode/focus/lens
（0x911230/240/260）为无栈叶子函数（无条目，落入前一函数区间外）——与 Ghidra NotFound
相互印证。
