# B 原版每帧常量与真实输入生产链

范围：`prepare_frame_constants@140af40b0`（PDB3989B）及必要资源构造、TOD末尾写盘、GameLoop时序状态和视图消费者。按root委派先完成原证据/候选，随后在root明确授权下**合入生产源码**；没有运行Cargo，整App系统登记及GPU结果由root/boot统一验证。不能把本报告的原证据、已合源码和运行闭合混作一项完成度。

生产现在已有canonical Weather、TimeOfDay、DerivedAtmosphere、TAA、RenderTemporalState、OutlineStairsCmd、真实14输入prepare与13输入TOD。root独立审查并合入GUI字段（不接管旧claim），boot负责真正主App的原set36/44、资源构造及GameLoop计数调用。
原B `.rne` SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。完整匹配PDB、原字节验证、工具身份见 `os-evidence/native-pack/report.json`。Ghidra `tiny-glade.exe` revision `1791719479858:b17d372d:3` 仅辅助，其 decomp 用 float 表示原 uint 的地方已由原指令和 SPIR-V纠正。

## 原完整入口

PDB具名模板实例 `0x1403c6680` 确认第一组是十四个 `Res`，随后为 `ResMut<RenderTemporalState>`, `ResMut<FrameConstants>`, `EventReader<OutlineStairsCmd>`。Win64 RCX 中各Res 0x20B：

|偏移|资源|实际消费|
|---|---|---|
|000|DirectionalLight|view、tint|
|020|ImageFormationConfigs|selected +c0，stride40，grading bypass/EV/artistic|
|040|DerivedAtmosphere|完整31float，见下面布局|
|060|MainCamera|前四矩阵256B，mode byte+4d0|
|080|GameTime|total_secs f64+10，delta f32+28|
|0a0|FrameRenderConfig|extent+10给视口/inner；extent+8宽给mip bias|
|0c0|utils::camera::TaaJitterSet|Vec<Vec2>，按temporal帧号取样|
|0e0|ShaderRefactoringState|disc>=2时覆盖帧号为payload&3|
|100|EditingHighlight|直接读color，不检查enabled|
|120|GameConfig|video.brightness_shift，native+a0|
|140|AppMode|native tag19 Photomode|
|160|TransientStairEdgeIdAllocator|OutlineStairs的StairGraphEdge查现有id|
|180|ActiveSession|native主题byte+58查表[0,3,1,2,4]|
|1a0|TimeOfDay|phase f32+8|

## 视图与时序

原 `MainCamera` 前0x100B依次为 projection / inverse_projection / view / inverse_view。当前 `camera_matrices()` 仅返回两矩阵且不覆盖这个完整合同，候选用已公开的 `read_matrix` 四次读取。

初次ViewportBuilder在 `140af4363` 输出rbp+920。`43c0..43d7` 复制无抖动值到rbp+600；`440b` 对rbp+920调用set_pixel_offset；`46de..46f6` 复制这个**已抖动**值到rbp+2e0。最终 `4a56..4ab7` 组装：

|view槽|原数据|
|---|---|
|0|主视图，带当前与上一帧TAA抖动|
|1|同一主相机，不带TAA抖动|
|2|太阳cascade0|
|3|太阳cascade1|
|4|太阳cascade2|

早先口述的两个main顺序已纠正，以这条真实memcpy链为准。太阳projection不是我方猜测的常用范围：X/Y scale原字节分别 `3c7c0fc1`、`3bfc0fc1`、`3b100901`（1/65、1/130、1/455），Z scale `ba800000`（-1/1024），translation(-0,-0,0.5,1)，每个viewport1200²。前/后矩阵对太阳视图均相同，不借主相机前帧。

`RenderViewIndex::is_shadow_view` 原PDB inline（frame_constants元数据）落在 `draw_bricks<0>@1408a53e7` 的 `cmp byte[rcx+f0],2; jae`；另实例按掩码0x1c测2/3/4。`shadow_pass@1409cc270` 实际三个CommonDrawParams值是cascade+2。审查前生产源码 `is_shadow_view !=0`、`SUN_CASCADE_0=1`、`sun_view(cascade+1)` 都错；root已按本证据改为>=2/cascade+2。MAIN_VIEW=0正确。原PDB本次仅找到is_shadow_view，不把其余数组投影helper名称假称原方法签名。

`RenderTemporalState` 原构造（run_game_thread@14059ef47..efbd；loading同形@1405a4e33..4e71）：四个**实际构造的相机矩阵**0..100，previous_jitter Vec2+100=0，frame_index u32+108=0，size110。不能用四个identity矩阵代替真实相机。prepare在index0时以当前相机作previous；否则用temporal.camera。随后更新temporal.camera与previous_jitter；不自增index。

真正的自增在原 `GameLoop::simulate_and_render_frame` line738 `1404de63a: incl dword[temporal+108]`，紧接render@1404de5bc返回。原line631比较generation及缓存extent：generation相同、缓存标志=1且当前extent10和缓存extent相同则跳过重建；否则重建分支line651 `1404ddc19` 将index清0，line657保存新extent。候选提供显式方法，由GameLoop真实分支调用，未塞进prepare。

## TAA constructor

`TaaJitterSet::default@140c97500` 原range bytes=`start1,end128,exhausted0`，map/fold@14217f990。每项是radical-inverse(n,2/3)-0.5，**n=1..=128**。base3 reciprocal是原f32 `3eaaaaab`，商用原f32乘法转u32，按原次序累加。`USE_TAA_JITTER@14362c0a0` 初始byte1；开时读取samples[index%len]，len0原panic；关时当前jitter0。初次index0也读取第一个样本，没有额外跳过。

## DerivedAtmosphere真实producer

`insert_resources@140940f83..fb9` 原startup对0x7c字节清0；这仅是初始资源，不能证明已有每帧光照。`time_of_day@140a73460` 才是完整运行时writer。原末尾`140a7432c..74397`写全部31float：

|offset|字段|
|---|---|
|00|selected light tint Vec3|
|0c|fake sun color Vec3|
|18|sun direction Vec3|
|24|fake moon color Vec3|
|30|moon direction Vec3|
|3c|sky low Vec3|
|48|sky high Vec3|
|54|snapshot.base_fog_thickness + Weather[0]|
|58|snapshot.fog_tint Vec3|
|64|sun direction Vec3|
|70|moon direction Vec3|

snapshot十二颜色float实际是四Vec3：sun_tint、moon_tint、sky_low、sky_high；当前a/b/c三Vec4容器存储相同，可投影而无需复制资源身份。原current=tod+20，previous=tod+1c：

- current<.5：light朝sun，w=(1-previous)*(1-2current)，selected=sunTint*w，fakeSun=sunTint*(1-w)，fakeMoon=moonTint。
- current>=.5（ucomiss unordered也走此分支）：light朝moon，w=(1-previous)*((current-.5)+(current-.5))，selected=moonTint*w，fakeSun=sunTint，fakeMoon=moonTint*(1-w)。

审查前TOD source final块两分支反向、weight符号反向且仅写TodLightingSeam；当前生产已替换为完整31float真实writer，直接消费canonical GameConfig、Weather、GUI TimeOfDayScreen。config原+e3就是dev.day_night_cycle，+118就是game.tod_speed，未新造资源身份。原完整签名还证明LoadingGameComplete属于save::load_game，已改掉旧mode_manager同名事件。输入按键使用已有KeyButtonInput作为显式宿主适配，而非假称原Bevy ABI。

## Header完整映射

- sunRGB=(light.tint*light_scale)*atmosphere.selected*30，alpha1；skyLow=atmo.low*(tint*scale)*8.16，skyHigh同形*2.72。fake colors=atmo.fake*30*scale，alpha1；方向alpha0。
- light_scale=exp2(brightness + (ev_shift + (lighting_only?-1:0)))，ev_mult=exp2(ev_shift)。grading bypass仅把ev_shift取0；艺术标志仍来自selected配置。
- fogVec4=(tint.rgb,thickness)，原shufps39。
- frame_counter=temporal.frame_index；delta=GameTime.delta；inner=extent10；theme表[0,3,1,2,4]。
- game_time_sec为**uint**，clamp(total_secs,0,4294967295)再native cvttsd2si存low32；fraction=f64 fmod(total_secs,1)转f32（负数不做欧式余数）。原SPIR-V成员13uint已被root独立验证。
- editingColor直接资源color；time_of_day=tod.phase；sun/moon方向分别atmo+64/+70；camera_idx=MainCamera+4d0。
- overrides先取lighting_only原byte；tree开加2；cameraMode3复原base；最后AppMode!=19强制base+2。
- mip_bias=log2(extent10.width/max(extent8.width,1))；stairid见下；pad1=1、pad2=2，debug全0。

## OutlineStairs完整结构，已由进一步原消费者纠正

最初只看prepare将kind3猜成enum tag，这是中间假设，现已被原structure_outline@140959fc0和UIproducer@1422b0e00推翻。原命令是16B结构：

|payload offset|字段|证据|
|---|---|---|
|00..08|StairSegment，排序的两个DecoratorId/u32端点|UI写两个返回寄存器；new与StairGraphEdge共用140901260|
|08|strength f32|UI写2.0；structure_outline仅处理>0|
|0c|kind u8（本次推定字段名）|prepare筛3，structure_outline交给closure作样式|
|0d|include_connected bool|true调用StairsState::get_all_connected_nodes，false选单segment|
|0e..10|padding|非业务字段|

Bevy EventInstance再带8B id，stride24。production canonical OutlineStairsCmd已从`_gap:u64`替换成精确结构，StairSegment8B原排序构造、到GraphEdge的同值投影、Allocator同身份Resource都已恢复。prepare真实EventReader按顺序读取：kind3使editingColor原(.1,.03,0)，查已有id；没找到继续，首个命中即break，留后续事件未读；没有命中id0。prepare原没有strength>0检查，不额外添加。

`EventWriter@142291100`16B复制、UI actualproducer@1422b1538..1568（strength2/kind0/connected1）和structure_outline reader交叉证据已落 `TG-d11abc3329e6f85879/frameinputs-20261018/*-direct.json`、`os-evidence/*consumer*`。现system_color::ui_color_windows_and_flags整体仍显式未实现，命令类型恢复不能冒充其完整UI producer已恢复；OutlineWallCmd也仍独立typed gap。生产没有伪造hover事件来强行触发高亮。

## 进一步闭合的TOD真实构造、资产与天气

- TimeOfDay原default@140940ed0..f00：phase=.3，sun angle0，latitude59.3293，night区间两界.505/.815，previous_continuous=.15，day_of_year207，Idle。原通用源码把这些全猜成0/1/.25/.75，现已纠正。runtime+24首次读取前由TOD写，不把未显式初始化的padding/缓存当原业务默认值。
- Weather原16B default(0,1,0,0)。load_glade_tod_and_weather@140ae7810先读`assets/glade/<theme>/tod.json`，仅I/O失败时回退`assets/tod/default.json`；JSON坏不回退。真实TodSequence serde已接四Vec3颜色字段和全部required entry字段，原temp_ui_id full-u64由文件读取，新增entry默认使用原fastrand全usize区间。
- 加载成功后Weather前两槽取GladeSettings.weather_fog/wind_strength，+8=0，snow_amount按winter初始化1/0。完整weather_system@1408e5b00只在winter按sin(elapsed/300*pi)符号增/减delta*.1，clamp0..1。+8名称仍未证实，保留field_8，不发明rain业务。
- SunSkyColors保留当前12float算术容器，使用private serde DTO对接原四Vec3字段。这是CPU序列化投影，不声称当前Vec4容器的alignment就是原ABI。实际DerivedAtmosphere reprC124B和FrameConstants按原GPU布局写。
- 主方向原初始向量是(cos(colat)*cos(elev),sin(elev),-sin(colat)*cos(elev))；旧source反了X/Z。已按原SSE向量组装修正并使用glam完整Quat乘法，不将Y旋转简化成忽略舍入的2D旋转。
- GUI default13个value字节0，TimeOfDayScreen+0/+4目标float，+8 just_opened、+9 was_visible、+a visible、+b时间拖动pending、+c角度拖动pending。root已独立审原包并合canonical结构；本worker未接管其旧claim。完整UIproducer见native-tod-resources/gui-tod*.json。
- 原set事实：TOD helper1402cc220调用14059fc0b=36；Weather helper1403429a0调用14059fc5b=36；prepare helper1403c7a30调用14059fc70=44。同set不擅自补chain先后。原资源装配由boot的mainApp负责，不向loadingApp制造ActiveSession。
- 旧core/startup/run_game兼容registry已清除TOD seam和固定(0,10,0)假灯，只注册真实consumer/事件，要求caller提供真实资源producer，不为未知Schedule编造新加载次序。

## 源码、验证和剩余边界

原纯算法候选已升级为生产ECS接口：十四Res元组、ResMut<RenderTemporalState>、ResMut<FrameConstants>、EventReader<canonical OutlineStairsCmd>。ShaderRefactoringState使用resources的现有重导出身份；Taa在utils本类型上derive Resource并补bevy_ecs0.16依赖，没有另造Res wrapper。所有必要module exports已接。

当前17个生产文件由本worker变更（含获root授权的聚合依赖与旧registry清理），具体SHA与清单见frame-inputs-manifest.json。源文件rustfmt语法解析通过；Cargo/check/单测由root统一执行，尚不据此报告通过。原native evidence pack各批status=ok，仅代表原字节/身份/PDB证据成立，不代表原侧执行差分成功。

尚待主App实际登记/运行验证、GameLoop按原分支调用temporal reset/increment。system_color完整拾取hover producer和OutlineWall几何输出仍独立未完成；本报告不把typed命令恢复算作该域整体完成。没有用FrameConstants::default、空事件、零光照或identity相机代替实际游戏输入闭包。

## Canonical ECS 集成测试（2026-10-18）

`prepare_frame_constants.rs::tests::canonical_ecs_prepare_preserves_view_order_jitter_and_unread_outline_events`直接把真实14个资源、Temporal/FrameConstants输出与canonical OutlineStairsCmd放入Bevy World，运行实际prepare系统，未直接调用纯算法helper充当集成结果。MainCamera通过原构造器保留真实Dolly owners；原构造器本身初始四矩阵identity，这是明确的startup测试边界，不证明后续相机更新系统。SessionMeta fixture复制原09-garden/meta.json（Autumn）并走真实文件loader；Atmosphere fixture复制原default TOD的0.27 snapshot并通过实际writer产生非零数据；Replay时钟由实际GameClock.update生成。灯构造参数、方向角、视口、重放时间和tag3楼梯命令是明确的受控输入，不称原启动值或UI真实发射。Allocator实际分配两个不同非零ID。

同一个Schedule连续执行三次验证原EventReader提前break的保留语义：tag0不匹配，未分配tag3继续，首个已分配ID结束；第二次消费保留的后续ID，第三次回退资源color/id0。检查view0抖动/view1无抖动/sun2..4三scale与真实Light.view、首两TAA sample/上一帧偏移、原GameLoop render-return方法与prepare不自增的分工。

parent已报告旧TOD系统7项、TOD资源/schema11项、prepare shadow1项、intro23state解析1项通过；本worker未运行Cargo，新增ECS测试等待parent统一执行。源码及SHA已同步本目录applied与manifest。
