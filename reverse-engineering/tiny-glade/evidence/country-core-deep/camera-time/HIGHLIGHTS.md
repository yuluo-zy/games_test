# camera-time 域 HIGHLIGHTS（tg_a6，2026-10-14，10 条）

1. **CameraMode 不是 6 变体而是 13 槽（0..12）**。fov 偏移表 0x142aabb98 共 13 项
   {0x8c,0x144,0x1c4,0x264,0x31c,0x3a4,0x08,0x20,0x14,0x2c,0x08,0x14,0x0c}；
   `previous` 表 [6,0,1,2,3,4,0,0]（Normal.previous=6）；paused_main_camera_update 有
   case 6..11；两处把模式 6 映射为 0。0..5 = 六个 body（Normal@+0x100/TiltShift@+0x1b8/
   Free@+0x270/Walk@+0x2f0/TopDown@+0x390/Butterfly@+0x448），6..12 = 拍照/工具模式复用
   这 6 个 body + 各自 fov 槽。变体名待 ui-input 域 photo_mode 联动。

2. **MainCamera 前 0x100 字节 = CameraMatrices 矩阵块**（paused update 每帧
   calc_matrices + memcpy 0x100 写入 @+0x0），body 从 +0x100 才开始——补全了合同只到
   body 偏移的布局；+0x4d0 模式字节、+0x4d8 aspect=w/h、+0x344 鼠标深度字节全实证。

3. **dolly 0.6.0 接线面闭合**：Free = Position+YawPitch+Smooth(1,1)、Walk = Position+
   YawPitch、Butterfly = Position+Rotation+Smooth(2,predictive)+Arm+Smooth(4)+LookAt(4)；
   driver 查找 = vtable+0x38 比对 TypeId（Position 0x6b8463577afc1a66 / YawPitch
   0x718e4fd258330fe5）；`match_free_camera_to_normal` 0x14090e6f0 完整展示 rig 重建
   （读 Normal position/yaw/pitch → 重建 Free rig → dt=0 跑一遍）。游戏自有逻辑 =
   模式切换/地形碰撞约束/快照插值，纯 dolly 数学不重实现。

4. **remap_hour_to_fmod_hour 合同纠错**：y 基值实为 {0,3,11,17,22,24}（合同写
   {48,17,11,24,22,…}），x 基 {0,3.5,9,17,20,24} ✓，冬季插入 4.52（x）与 5.0/6.2/9.0（y）
   ✓，常量地址全实证。

5. **time_of_day 状态机三资源实证**：当前 {hour,angle,角度过渡@+3,小时过渡@+0xa,瞬时@+0xb}；
   播放 {state(0/1/2), progress, target_hour, 当前角, 日出/日落小时, day_of_year(1..365，
   越界 unwrap panic)}；开发者 {小时倍率 f32@+0x23（合同写 +0x118，纠正）, 热键 byte@+0xe3}。
   状态机：dev 关 state2 倒带 dt/−5；dev 开 state1 前进 dt/3；hour=fmodf(progress·24+8,24)
   → 音频参数 "TOD-Enabled"/"Timetravel"/"DayNight" → suncalc::get_position（复用 crate）。

6. **time_update 56B 时钟纠错一处**：+0x10 是 f64 total（合同写 f32）；其余
   {secs,nanos,flag@+0x6,60ths@+0x18,frac@+0x20,frame_count@+0x2c} 全证；重放时 delta
   来自录制帧 @+0xc8（0xd0B 帧），前两帧 clamp 1/60。

7. **weather_system 全证**：glade_type@+0x58==4（冬）→ sin(game_time/300·π) 正负驱动
   intensity@+0xc ±0.1·dt，clamp [0,1]；game_time = secs + nanos/1e9（1e9@1429a7640）。

8. **会话存档相机 schema 现形**：0x1403b4ca0（PDB 名 MainCamera::xz_projective_rotation，
   实为 ron 反序列化器）= 字段 {yaw,pitch,position,camera_rig_arm_length,pitch_offset,
   yaw_offset,position_cache}（session.rs），与 0x24B CameraRigParametersCache 元素一致；
   store_parameters 捕获 {yaw@+0x154,pitch@+0x150,pos@+0x138,fov@+0x190} 并逐个 NaN 校验
   （"…invalid. Overriding param cache value." 四条日志）。

9. **timelapse/is_active 纠错**：0x14092e6b0 机器码 = `state != 2`（骨架写 ==1）；相邻还有
   ==3 / <2 谓词。start→check_if_can_switch<closure>；finish→推 MetaCommand（i64::MIN 哨兵）
   + state=1，命令队列空后推 {tag,byte} 事件；cleanup→"The timelapse has finished" + state=2。

10. **look_at_camera 的 register_required_components 是 derive 注册胶水**（bevy_ecs
    component.rs 递归，28KB），不是骨架建模的运行时插入系统；look_at 系统本体用共享工具
    FUN_140c96d40/FUN_140c96d80/FUN_140c9ce00 逐实体写 Transform。另外 arrow_gizmo 动画 =
    主轴拉伸（事件 +0.02 增量/无事件 −0.1 衰减，clamp 1.0），gizmo 签名符四重门槛
    （AppMode∈{2,10}、非 Free/Walk、窗口<3px、模式∈{0,1,4}）全实证。
