# backend-misc 域 HIGHLIGHTS（top ≤10 发现，供中央汇总）

1. **三个 watch 系统同构**（configwatch 0x14085b1c0 / meshwatch 0x14085b4b0 / texture_watch 0x140aab370）：锁互斥队列 → 日志 `'{X}watch: detected changes: N'`（门槛 `全局>2`）→ 位图迭代 → 逐项重载（ColorLibrary+SSBO / AssetMeshLibrary::reload_mesh / 纹理树遍历），panic `Result::unwrap` 均在行 43。
2. **flush_screen_log 为全局静态队列** `Mutex<Vec<(String, LogType)>>`（0x14362bfe8 起，条目 0x20B {len,ptr,len,kind}）；`log_error_to_screen`/`log_warning_to_screen` 不是 log 框架调用而是压队列；`flush_error_log_to_screen` 是 `ResMut<ScreenLog>` 系统，排空并 5 槽环着色上屏（错误红 0xff0000ff / 警告黄 0xffff00ff）。
3. **prepare_frame_constants 0x140af40b0 帧常量组装证实**：`frame_constants+0x108 = idx & 3`；TAA jitter 表 0x142ae4fc0={0,3,1,2,4,...}，按全局门控 `cRam000000014362c0a0` 用帧号 `%table_len` 采样；4×ViewConstants（FUN_140b18f60）。
4. **GameFrameIndex 资源证据为 2×u32**（门槛 >1 + 帧号），重构单字段 u32 不完整（I）。
5. **grow_grass 0x14172fe80**：Timer `{f32 elapsed@0, u8 grown@4}`，300.0s 阈值后解码 15B 静态（XOR）→64B 哈希 memcmp → EventWriter 发 8B 事件 → 置 grown；重构 f64/is_grown 与机器码不符。
6. **icc 0x14093d830**：lcms2-5.6.0 `cmsOpenProfileFromMemTHR` + transform，对具名色（terrain_1/terrain_2/wall_moss/wall_bricks/terrain_highlight/night_tint）生成 LUT 写 ColorLibrarySSBO；build 0x140a58440 为跳板→0x140b65a80（App 调度注册，Schedules 缺失 panic）。
7. **systems/mod.rs GameSystemSet 判别仅 1 字节**（dyn_clone 分配 1B、dyn_hash 写 u8）；`is_gameplay_active` 是双 Res 谓词 `[rcx]+0x30==1 && [rdx]+0==1`（资源身份 U），与重构 `&mut World` 不符。
8. **utils::add_stages 0x140945ed0 证实** `enum_iterator::All<GameSystemSet>` → `App::configure_sets(Update)`；变体表 8 项。
9. **utils_intersect::line_x_shape_overlap 0x14175ada0**：返回 `{hit, t1, t2}`（非 bool），圆/矩形射线求交 + t∈[0,1] 断言（源 curve_u.rs:42）；签名含额外 param_3/param_6。
10. **索引错位**：sysres-procedures-v3.json 的 `insert_backend_resources::closure$0`→0x140943660 实际是 `add_backend_events`；`flush_error_log_to_screen` 命名误导（实际排空错误+警告）。
