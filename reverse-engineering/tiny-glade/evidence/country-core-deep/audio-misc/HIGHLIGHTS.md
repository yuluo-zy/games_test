# audio-misc 域 HIGHLIGHTS（≤10 条最重要发现）

1. **GradingLut::load_cube = 完整 .cube 解析器全量恢复**（0x140b01050）：逐行解析
   TITLE/DOMAIN_MIN(必须 0,0,0)/DOMAIN_MAX(必须 1,1,1)/LUT_3D_SIZE（手写 u64 十进制解析，
   溢出检查）/数据行 → parse_rgb → **RGB9E5 打包** → 3D 纹理（调试名 "grading lut"，
   目录 "assets/luts"）。关键业务串：`"1D LUTs are not supported"`、`"Too many entries in
   the LUT. Bailing before an overrun"`。现有 reconstruction 的 grading_lut.rs 全部是
   `unimplemented!()`——本批给出了可直接落码的完整算法与错误语义。

2. **ColorMenuBrightnessValue 现有代码错误**：机器码（0x140b06770）= 判别式索引 f32 表
   **{0.25, 1.0, 4.0}**（表 0x142ae785c）的三变体无载荷枚举；reconstruction 建模成
   `{Default, Custom(f32)}` 与证据不符。默认变体 = 1（1.0，default_f32 常量 O）。

3. **ColorId 默认值纠错**：wp16 合同与 color_id.rs 代码写 `default_value = ColorId(0)`
   （引 0x14099ca50）；该地址返回 **1** 且 PDB 归属 **color_id_roof.rs**。屋顶默认色
   索引 = 1（非 0），"默认色=0" 的过滤语义需重审。

4. **FMOD 失败全局报告机制**（新业务面）：set_parameter/set_position 失败时拼
   `"module_p_<名>path!(…) :: Warn: <err>"` 写入每函数独立槽位的进程全局注册表
   （0x143634d60/d58 与 0x143634d80/d78），锁失败 panic `"Mutex was poisonned"`（原文双
   n）；受全局 verbosity（0x14362c3a8）与进程状态字节（0x14362c420）门控。

5. **music 域两处行为纠错**：(a) 0x1408b72e0（PDB 名 current_track）实为
   **stop_immediately**（遍历全部曲目 stop(ALLOWFADEOUT)+主事件）；(b)
   `get_timeline_beat_properties`（0x1408b7620）**不是 getter** —— 把节拍属性写入 6 个
   进程全局（0x143634c68..c90，含 running max；c90 由 MusicPlayer::update 每帧清零）。

6. **Steam 动作表全量恢复**：`SteamMainActionSet` = 动作集 "GameControls" + 28 动作
   （CameraPan/cursor_move/tool_*×10/zoom_in/out/ui_* 等，0x142ce2049 起串表）；
   Menu 集 ui_tab_prev/next；另 CottageWindow/GothicWindow 两层。`user_id()` 每次现算
   `client.user().steam_id()`（现有代码存字段模型错误）；`is_floating_gamepad_text_input_
   open` 是进程全局字节 0x143634d94（回调缓存，非 API 调用）。

7. **AudioInstance 布局钉死**：Active = `{disc u64@0, event 句柄@8, name String
   cap-first {cap@0x10,ptr@0x18,len@0x20}, state u8@0x28}`（0x140a54440 写序）；
   start_if_not_playing 的 playback 位掩码 0xb（{Playing,Sustaining,Starting} 跳过）；
   Bank::load 失败 = **panic**（"Error loading FMOD bank file: {path} {err}"）；
   AudioSamplesLoadedPromise::recv = crossbeam 三通道 recv_timeout(1s)，断开 → unwrap panic。

8. **PDB 名大面积错位（8 例，机器码为准）**：0x140a53a00 = update（非
   DelayedBankLoader::load）；0x140a54810 = set_vca_volume；0x140a57670 = set_parameter
   （非 pause）；0x140941450 = add_audio_events（非 add_audio_startup_systems）；
   0x140b01050 = load_cube（非 asset_name）；0x140949c40 = 析构例程（非 aspect_ratio）；
   0x140909870 = 羊叫串装配（非 FeedbackReportTags::to_list）；0x140b1f210 = `fn
   disabled(&self)->bool`（非构造器）。

9. **窗口/输入小函数纠正**：`WindowSize::as_vec2` = 头 8 字节直接返回（f32×2，现有
   u32 模型错误）；`FeedbackReportBgTexture::is_available` = 状态字节 `[+0x36] < 7`
   （非独立 bool 资源）；`reset_choices` = `{0,1,1,0,0,0}`（现有代码 O 一致）。

10. **跨域共享全局**：0x143634d90 同供 image_formation 档 2 与 gpu_lut 二条目门禁
    （非 demo 开关）；path_audio_init 事件名补 `Path Draw`/`Erase Draw`；
    elevation_supports 补 BrickColumnAppear/WoodenColumnAppear；signifier init 5 事件名
    全量（含原版拼写 `HoldTimeSiginfier`）。

未解决：MusicPlayer::update 两跳表 case 语义、update_steam_inputs 逐动作映射表、
0x14362c420 语义、FeedbackReportResources 0x18..0x36 字段。
