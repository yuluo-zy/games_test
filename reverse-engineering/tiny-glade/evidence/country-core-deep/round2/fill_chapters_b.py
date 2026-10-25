# -*- coding: utf-8 -*-
"""Round-3 chapter filler part B: systems (time/weather/misc) + resources."""
import os

CH = r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters'
D = 'camera-time\\decomp'
R2 = 'round2\\decomp'

def w(rel, content):
    p = os.path.join(CH, rel)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf-8').write(content)

# ---------------- systems/look_at_camera.rs.md ----------------
w('systems/look_at_camera.rs.md', (
"# systems/look_at_camera.rs\n\n"
"> 文件作用：**LookAtCamera 组件驱动**——每帧把带 `LookAtCamera` 组件的实体的 Transform\n"
"> 旋转到朝向记录目标（Query (LookAtCamera, Transform) + 共享工具 FUN_140c96d40 提取、\n"
"> FUN_140c9ce00 写回 + Mut tick）；组件注册为 Bevy derive 的 required-components 胶水。\n\n"
"函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `impl$0::clone_behavior` | 0x140002370 | 6 | 返回 0（组件克隆行为标志：不克隆） | O：`xor eax,eax; ret` | %s\\decomp-0x140002370.txt（hex 手解） |\n"
"| `impl$0::register_required_components` | 0x1408756e0 | 6 | **Bevy derive 注册胶水**（非运行时系统）：required-components 递归注册（bevy_ecs-0.16.0 component.rs panic 串 + 0xd0B 组件记录拷贝） | O：derive 胶水实证（骨架建模成系统是错的） | %s\\decomp-0x1408756e0.txt |\n"
"| `look_at_camera` | 0x140873de0 | 10 | puffin scope；逐实体：FUN_140c96d40 提取目标变换→FUN_140c9ce00 写回实体 Transform（位置+旋转四元数），Mut tick 回写 | O：实证（朝向数学在 utils::camera 共享工具） | %s\\decomp-0x140873de0.txt |\n"
) % ((D,)*3))

# ---------------- systems/time_of_day.rs.md ----------------
w('systems/time_of_day.rs.md', (
"# systems/time_of_day.rs\n\n"
"> 文件作用：**昼夜系统**——13 参数系统：太阳角/小时双插值（exp(−3·dt) 平滑、wrap ±π/±0.5）、\n"
"> 播放状态机（dev 关 state2 倒带 dt/−5；dev 开 state1 前进 dt/3，hour=fmodf(progress·24+8,24)）、\n"
"> 音频上报（TOD-Enabled/Timetravel/DayNight）、suncalc 太阳位置（lat=状态@+0x10、lon=0、\n"
"> 日期=2023<<9|day_of_year）、TodSequence 光照快照插值、remap 小时→fmod 小时关键帧曲线。\n\n"
"函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `time_of_day` | 0x140a73460 | 0 | 见文件作用；资源面：当前态 {hour,angle,角度过渡@+3,小时过渡@+0xa,瞬时@+0xb}、播放态 {state,progress,target_hour,角度@+0xc,纬度@+0x10,日出@+0x14,日落@+0x18,太阳可见@+0x20/24,day_of_year@+0x28}、dev 态 {小时倍率@+0x23,热键@+0xe3}；越界 day_of_year unwrap panic | O：22KB 反编译全析（DIGEST §2.6） | %s\\decomp-0x140a73460.txt |\n"
"| `remap_hour_to_fmod_hour` | 0x140a74790 | 287 | 小时→fmod 小时关键帧：x={0,3.5,9,17,20,24}、y={0,3,11,17,22,24}（冬季 x 插 4.52、y 插 5.0/6.2/9.0），线性插值查找 | O：两 Vec 构建+memmove 插入+lerp 全实证 | %s\\decomp-0x140a74790.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `suncalc_pos_to_vec`（行 348）— suncalc 方位/高度 → 3D 方向向量（colatitude=π/2−(azimuth+π) + 正交基常量 @142ad5910..5984，并入 time_of_day 主体）\n"
) % ((D,)*2))

# ---------------- systems/time_update.rs.md ----------------
w('systems/time_update.rs.md', (
"# systems/time_update.rs\n\n"
"> 文件作用：**游戏时钟系统**——推进 56B 时钟 {secs,nanos,flag@+0x6,total f64@+0x10,\n"
"> 60ths@+0x18,frac@+0x20,frame_count@+0x2c,flag@+0x30}；重放时 delta 取自录制帧\n"
"> @+0xc8（0xd0B 帧，InputRecordReplayState+0x40==i64::MIN+2 判 Replaying），前两帧 clamp 1/60。\n\n"
"函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `time_update` | 0x140922c30 | 10 | 见文件作用；Duration 进位 panic（\"overflow when adding durations\"）+ f32→Duration 转换（std 内联） | O：实证（total 为 f64，纠正合同 f32 说） | %s\\decomp-0x140922c30.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `calculate_delta_seconds`（行 45）— 查询/计算：GameTime Duration(secs@+0x10,nanos@+0x18)→f32 秒（内联的 f32→Duration 位运算族，并入 time_update 主体）\n"
) % D)

# ---------------- systems/weather_system.rs.md ----------------
w('systems/weather_system.rs.md', (
"# systems/weather_system.rs\n\n"
"> 文件作用：**冬季雪强振荡**——`glade_type@+0x58==4`（冬）时，10 分钟正弦\n"
"> `sin(game_time/300·π)` 正负驱动 SnowIntensity@+0xc ±0.1·dt（GameTime+0x28），clamp [0,1]。\n\n"
"函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `weather_system` | 0x1408e5b00 | 0 | 见文件作用；非冬季不写（连 tick 都不回写）；常量 300@142aa5140、π@1429edba4、0.1@142925984、1e9@1429a7640 | O：实证 | %s\\decomp-0x1408e5b00.txt |\n"
) % D)

# ---------------- systems/timelapse_replay.rs.md ----------------
w('systems/timelapse_replay.rs.md', (
"# systems/timelapse_replay.rs\n\n"
"> 文件作用：**延时摄影录制/回放状态机**——`TimelapseState` 字节 0=idle/1=recording/2=finished；\n"
"> start：消费 StartTimelapseReplayCmd→修剪输入流→有未消费帧→mode_manager::check_if_can_switch；\n"
"> finish：未完成时推 MetaCommand（i64::MIN 哨兵，meta_commander.rs）→state=1，命令队列空后推\n"
"> {tag,byte} 事件（TimelapseReplayFinished/Interrupt 族）；cleanup：有剩余帧→日志→state=2。\n\n"
"函数总计 5：业务 4 / 内联无独立符号 0 / 派生胶水 1\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `TimelapseState::is_active` | 0x14092e6b0 | 44 | 谓词：**`state != 2`**（\"未结束\"；骨架写 ==1 是错的） | O：`cmp byte[rcx],2; setne al` | %s\\decomp-0x14092e6b0.txt（hex 手解） |\n"
"| `start_timelapse` | 0x140a8a310 | 51 | 见文件作用（EventReader 修剪 + check_if_can_switch<closure> 0x140af9e00） | O：实证 | %s\\decomp-0x140a8a310.txt |\n"
"| `finish_timelapse` | 0x140a8a460 | 95 | 见文件作用（MetaCommand 推入 FUN_140b59900；事件 stride 0x10 {tag:i64::MIN, byte}） | O：实证 | %s\\decomp-0x140a8a460.txt |\n"
"| `cleanup_timelapse` | 0x140a8a5d0 | 138 | 有未消费帧→日志（level 3）\"The timelapse has finished\"→state=2 | O：实证 | %s\\decomp-0x140a8a5d0.txt |\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`closure` 等 1 项\n"
) % ((D,)*4))

# ---------------- systems/arrow_gizmo_animation.rs.md ----------------
w('systems/arrow_gizmo_animation.rs.md', (
"# systems/arrow_gizmo_animation.rs\n\n"
"> 文件作用：**箭头 gizmo 拉伸动画**——`EnableArrowGizmoStretch` 组件实体的双 f32 拉伸值：\n"
"> 有输入事件（8B 向量 payload）按**主轴** `stretch=min(stretch+|δ|·0.02, 1.0)`（0.02@1429cd2d0）；\n"
"> 无事件每帧衰减 `stretch=max(stretch−0.1, 0)`（−0.1@1429eb6f8）；Mut tick 回写。\n\n"
"函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `arrow_gizmo_animation` | 0x140a92f80 | 0 | 见文件作用；puffin scope `country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation` | O：实证 | %s\\decomp-0x140a92f80.txt |\n"
) % D)

# ---------------- systems/ui_camera_mode_gizmos.rs.md ----------------
w('systems/ui_camera_mode_gizmos.rs.md', (
"# systems/ui_camera_mode_gizmos.rs\n\n"
"> 文件作用：**相机模式 gizmo 签名符（小窗口）**——四重门槛：AppMode∈{2,10}（10 需 flag=0）、\n"
"> 相机非 Free/Walk（(mode&6)==2 返回）、窗口<3px、签名符模式∈{0,1,4}（0x13>>mode&1，6→0 映射）；\n"
"> 通过后 UiState::add(\"camera_mode_activerotation_\")（状态 int≠20 时）→ 0x20B UiAppendSignifier\n"
"> {1,1,id_string,size,fov@+0xe8} → AppendOnlyVec。闭包=渲染回调（cursor_grab 部件 + vertex_color_shader，\n"
"> 动画 value/1000·20，clamp 25.0@1429edb98）。\n\n"
"函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `ui_camera_mode_gizmos` | 0x1409527a0 | 24 | 见文件作用 | O：实证 | %s\\decomp-0x1409527a0.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `cursor_icon`（行 89）— 光标抓取图标签名符构造（并入闭包 0x140952b60：cursor_grab 部件 + 动画，消费 CameraRotationPivot::use_camera_cursor_grab_icon 谓词）\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`closure` 等 1 项\n"
) % D)

# ---------------- systems/clear_canvas.rs.md ----------------
w('systems/clear_canvas.rs.md', (
"# systems/clear_canvas.rs\n\n"
"> 文件作用：**清空画布系统**——事件驱动（EventReader 修剪流）：日志 \"Clearing the canvas.\" →\n"
"> InnerWalls::remove_all（0x14085a730）→ 世界光栅 clear_all<512>（0x14093bee0，变体 0x13/0x14/\n"
"> 6/5/7/1/2/3/4 + 9/0xb/0xd/0xe）→ 7×clear_preset_texture（0x140a15da0，3 个 color_id 0x5000=\n"
"> 20KB 缓冲）→ 上传记录/杂项清零（0x140aaae80×2、0x140aa5150、0x1408950d0、0x14088acf0）。\n\n"
"函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `clear_canvas` | 0x140a7abd0 | 33 | 见文件作用（16 参数系统） | O：实证 | %s\\decomp-0x140a7abd0.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `clear_terrain_gpu`（行 187）— 删除/清理：地形 GPU 缓冲重置（0x14000=80KB 分配 + color_id.rs panic 点，并入 clear_canvas 主体尾部）\n"
) % D)

# ---------------- systems/despawn.rs.md ----------------
w('systems/despawn.rs.md', (
"# systems/despawn.rs\n\n"
"> 文件作用：**关卡拆除实体清理**——Query 迭代，world 字节 @+0x108 二分：A 支（stride 0xa0、\n"
"> count@+0x58、实体数组@+0x50、0x10B 元素）与 B 支（stride 0x48、count@+0x10、数组@+0x8、\n"
"> 8B 元素）；逐槽逐实体 Commands::despawn（0x140d14770）；u64 实体 id 高 32 位==0 提前返回。\n\n"
"函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `despawn_glade_entities` | 0x1408b3510 | 5 | 见文件作用 | O：实证 | %s\\decomp-0x1408b3510.txt |\n"
) % D)

# ---------------- resources/time_of_day.rs.md ----------------
w('resources/time_of_day.rs.md', (
"# resources/time_of_day.rs\n\n"
"> 文件作用：**昼夜数据资源**——`TodSequence`（entries: Vec<TodSequenceEntry>，0x50B 步长：\n"
"> {start_hour,end_hour,snapshot@+0xc}）与 `TodLightingSnapshot`（0x44B：sun_sky@+0x0 8×f32、\n"
"> fog_tint@+0x20 2×u64 打包雾色（**不插值**）、base_fog_thickness@+0x30 4×f32、speed_bias@+0x40）\n"
"> 的逐帧插值；会话序列化：TodSequence{entries}、TimeOfDay{time_of_day,snapshot,temp_ui_ids}；\n"
"> 目标态小结构 {time_of_day, sun_xz_rotation, latitude_degrees}；TodEasing 默认 fastrand 选段。\n\n"
"函数总计 24：业务 5 / 内联无独立符号 3 / 派生胶水 16\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `TodSequence::snapshot_at_tod` | 0x140b067d0 | 0 | `t=tod.rem_euclid(1)` 定段→clamp01 逆 lerp→TodLightingSnapshot::lerp（u64 对直接拷贝）；len==0 panic | O：实证（tod 经 sret 缓冲首 4B 传入） | %s\\decomp-0x140b067d0.txt |\n"
"| `_::impl$0::serialize<serde_json::value::Serializer>` | 0x1400802c0 | 15 | 目标态小结构 {time_of_day, sun_xz_rotation, latitude_degrees} 序列化 | O：字段名串实证（round2） | %s\\r2-0x1400802c0.txt |\n"
"| `_::impl$0::deserialize::visit_str` | 0x140080710 | 127 | 同结构反序列化变体名匹配 | I（serde 胶水面） |  |\n"
"| `_::impl$0::serialize<serde_json Pretty>` | 0x14007fb80 | 127 | TodSequence 序列化（单字段 \"entries\"） | O：实证（round2） | %s\\r2-0x14007fb80.txt |\n"
"| `walls::_::impl$0::serialize<serde_json Compact>` | 0x140080160 | 179 | 数值字段序列化（u32/u64，9999 年份上限检查 + 数字 LUT \"000102…99\"）——std 胶水复用面 | O：实证（round2） | %s\\r2-0x140080160.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `TimeOfDay::hour`（行 59）— 当前小时 getter（f32 直读）\n"
"- `TodLightingSnapshot::lerp`（行 145）— 快照插值（f32 组 lerp、fog_tint u64 对拷贝；0x140a97650）\n"
"- `snapshot_lerp`（行 234）— 两个快照间插值（并入 snapshot_at_tod 的分段插值调用）\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`clone`、`default`、`deserialize`、`eq`、`expecting`、`hash`、`serialize`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 16 项\n"
) % ((D, R2, R2, R2)))

# ---------------- resources/time.rs.md ----------------
w('resources/time.rs.md', (
"# resources/time.rs\n\n"
"> 文件作用：**游戏时间资源门面**——`GameTime` 实测布局：{game_time 累计 Duration（secs@+0x0/\n"
"> nanos@+0x8）、delta Duration（secs@+0x10/nanos@+0x18）、delta f32@+0x28；+0x20..0x27 与尾部 U}；\n"
"> 消费方：weather（game_time 正弦）、time_update（delta）、time_of_day（dt@+0x28）。\n"
"> `Time` = bevy_time 0.16 离线门面。\n\n"
"函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`default` 等 1 项\n"
))

# ---------------- resources/weather.rs.md ----------------
w('resources/weather.rs.md', (
"# resources/weather.rs\n\n"
"> 文件作用：**天气资源占位**——`Weather` 空资源（wp21 misc-resources：O 名、`impl$0::default`\n"
"> 内联、audio weather 消费；布局 U）。雪强逻辑在 systems/weather_system.rs（SnowIntensity@+0xc）。\n\n"
"函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`default` 等 1 项\n"
))

# ---------------- resources/daily.rs.md ----------------
w('resources/daily.rs.md', (
"# resources/daily.rs\n\n"
"> 文件作用：**每日主题**——`DailyThemeInfo`：default 置两个 u64=0x8000000000000000 哨兵；\n"
"> `is_weekly` = SystemTime::duration_since 成功且 `elapsed > 0x6977f`（432511s ≈ **5.006 天**，\n"
"> 非 7 天）；`get_name` = 主题 locale 名选择（[rdx+4] 首字符匹配 en-US/de-DE/es-ES/fr-FR/\n"
"> it-IT/ja-JP 族常量，r8==5 门）。CurrentDaily/DailyThemeCredits 占位。\n\n"
"函数总计 10：业务 2 / 内联无独立符号 0 / 派生胶水 8\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `DailyThemeInfo::get_name` | 0x14083c240 | 31 | locale 判别主题名选择（jumptable on kind==5 × 首字符） | O：hex 手解（常量族 \"n-U\"/\"e-D\"/\"s-E\"/\"r-F\"/\"t-I\"/\"a-J\" = locale 码片段） | %s\\decomp-0x14083c240.txt（hex 手解） |\n"
"| `DailyThemeInfo::is_weekly` | 0x14083c440 | 52 | 见文件作用（阈值 0x6977f 实证；v1 名 get_name::closure$0 即其内联闭包） | O：实证 | %s\\decomp-0x14083c440.txt |\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`closure`、`default`、`deserialize`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 8 项\n"
) % ((D,)*2))

# ---------------- resources/glade_settings.rs.md ----------------
w('resources/glade_settings.rs.md', (
"# resources/glade_settings.rs\n\n"
"> 文件作用：**谷地设置存档**——`GladeSettings` 默认布局 {+0x0 u32=0、+0x8 f32=0.01、\n"
"> +0xc f32=1.0、+0x10 u32=8、+0x18 f32=0.0、+0x1c f32=1.0、+0x20 u8=1}；`load` 从\n"
"> glade_settings.json serde_json Option 读取，失败回退 Default（字段语义 U）。\n\n"
"函数总计 9：业务 2 / 内联无独立符号 1 / 派生胶水 6\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `_::impl$0::deserialize::visit_str` | 0x140a468b0 | 4 | json 反序列化变体名匹配 | I（serde 胶水面） |  |\n"
"| `GladeSettings::load` | 0x1408bbbc0 | 49 | 见文件作用 | O：实证 | %s\\decomp-0x1408bbbc0.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `GladeSettings::try_load`（行 43）— Option 返回版（并入 load 的 unwrap_or_default 路径）\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`default`、`deserialize`、`expecting`、`visit_map`、`visit_seq` 等 6 项\n"
) % D)

print('wrote systems part B + resources')
