# TB-R7：圆形与矩形创建消费者恢复复核

本轮范围是 B 原 `circle/create_circle.rs`（14097e560）与 `rectangle/create_rectangle.rs`（140a93790）。保留当前文件及权威数据模型，未另建业务目录，未修改渲染引擎。UI producer 由另一 owner 恢复，事件公共定义和正式注册由中央统一维护。

## 事件与业务顺序

| 字段 | 圆形 payload | 矩形 payload | 当前 typed 事件 |
| --- | --- | --- | --- |
| previous option | +0 判别、+8 WallId | 同左 | `previous_wall: Option<WallId>` |
| shape | +10 center、+18 radius、+1c rotation | +10 center、+18 dimensions、+20 orientation | 对应几何字段，保留圆 rotation |
| computed height | +20 | +28 | `height`，来自 ShapePlacement +40 |
| elevation | +24 | +2c | `elevation`，来自 ShapePlacement +3c |

Circle EventInstance stride48；Rectangle stride56。不能把 EventId 当成业务 payload，也不能把 producer 清零 +40 误读为创建高度恒零：原 calc_shape_height 在调度链中写回 computed height。

两消费者都先发送树隐藏（外扩4，height_limit=elevation），再生成完整几何曲线、`InnerWalls::new_wall_w_params`（height、初始 style1、clutter0、当前 B 非 demo 模式）及 elevation。圆先写 enclosure 再配色/style，矩形先配色/style 再写 enclosure，保留原顺序。

新墙配色继承复用原 `get_building_and_roof_colors` 策略：校验旧公开曲线、检查形状 tag、按完整 WallId 下标继承墙与半木颜色、读实际 Roof 颜色和12字节外观、继承旧 wall style。原私有 ECS 密集表不能仅注册一个空资源假装接通；正式消费者现使用真实 `Query<&Roof>`，通过 ECS 验证 Entity 代数和组件存在性。旧 `RoofColorComponentTables` 仅保留既有验证/兼容入口。

Roof style 在组件存在时始终继承，只有有序 `pitch <= 0.1` 时才替换计算高度。NaN pitch 不替换高度，但仍继承 style/color/appearance；缺组件或过期 Entity 回退默认屋顶设置。墙/半木颜色事件、CreateRoof、原墙/屋顶/可选门颜色历史、toolbar category7及羊检查按原顺序输出。历史直接进入 canonical `History`，不经过旧的空 `RecordColorHistory` 消费函数。门颜色使用已有 `ColorEdit::Gate`（原 target4），记录0→配置 gate StrokeId。

当前 Rust History API 显式返回错误，因此未开始输入录制时保留既有创建输出并发出 `HistoryRecordingFailed`；这属于当前框架错误边界，不宣称原 void History::add 的异常机制逐字等价。源内没有把错误静默丢弃。

## 原算法修正

1. 圆半径 offset 原常量为 `bf666667`，而 Rust `-0.9` 是 `bf666666`。首轮逐位差分失败后读取原 PE 修正，未放宽 ULP。
2. 圆 `u=(radius+offset)/7.1`，进入原 SineInOut 前不 clamp；最终高度为 `(10e+4(1-e))/12` 后 clamp。原圆屋顶 ridge 固定 **0.4**，eave0.2，旧实现把两者都写0.2不正确。
3. 矩形高度独立使用 `(max_dim-2.25)/12.75`，进入 SineInOut 前不 clamp；结果为 `(6e+3(1-e))/12` 后 clamp。不能复用 ridge 的5.25及预先截断。
4. 矩形 ridge 保留原分组：`u=clamp((max_dim-2.25)/5.25)`；`s=(3-(u+u))*(u*u)`；`t=clamp((1.5*s+max(y/x,x/y)-1.5)/1.5)`；`ridge=(3-(t+t))*(t*t)`。不能重排乘加顺序。
5. 原屋顶历史 presence 逐字段保留与 CreateRoof 相同的 profile/height/ridge/eave/direction/style/color；墙历史使用实际继承 wall color，不能填0。

## 证据与验证边界

`shape-create-consumers-evidence.json` 保存只读 Ghidra 函数结果和有界 LLVM 汇编窗口；RNE/PDB SHA、数据库 revision 和独立身份边界已注明。函数 metadata size 是主体字节计数，不能凭 `start+size` 声称连续窗口覆盖完整 CFG。业务顺序同时核对完整反编译及实际调用目标，未将 ICF 错名当作类型依据。

`probe_shape_create_parameters_native.py` 对真实原指令执行圆22、矩形484例，执行原 Easing tag2 跳表，仅导入 cosf 桥接 Darwin libm。相应 Rust 差分共506例：有限数与输入 NaN payload 保持严格 bit 比较；矩形8例仅出现独立证明的 x86/ARM **新生 canonical NaN 符号**差别（ffc00000/7fc00000），按例数单列，不放宽成任意 both-is-NaN。这里不宣称 Windows UCRT 实机数学库逐位等价。

真实 CPU World 集成覆盖两形状的7种 Roof/History边界，共14条完整路径：真实 canvas phase→真实 UI producer→当前消费者→原 Roof 组件继承→canonical History→PublicWalls 发布→墙删除发布；包含非零 computed height/elevation、WallState JSON往返、缺组件、过期 Entity、NaN/阈值 pitch、未开始历史录制。实际运行另一 owner 的 signifier 创建、动画删除、实体释放和 TransientMesh 清理。TestDevice 仅验证 CPU 所有权及卸载，不冒称 GPU 实机画面。

消费者还验证配色命令、门色条件、羊检查、工具切换、事件不重复消费；NaN 圆测试检查原树隐藏事件先发、曲线 unwrap 失败后不创建墙/屋顶/历史。既有颜色策略9测试保留缺 host、非规则 shape、非法曲线/NaN length panic、颜色数组越界及密集表深层 miss 行为。

真实 OS 输入当前按用户要求暂停。本轮没有证明正式宿主的完整屋顶/羊/树消费者、GPU场景及整游戏保存重启验收；这些必须由中央按注册及下游 owner 的结果联合验收，不能由本轮 CPU 输出命令存在来替代。

## 下一轮可复用经验

- 先恢复事件完整 payload，再接 producer/consumer；高度、海拔、rotation和 previous option 都会改变真实业务，不能靠新字段默认值掩盖缺口。
- 追踪资源是否有真实 producer。仅 `init_resource` 不代表派生组件查询已接通；跨 Bevy 版本应复用实际 Query 语义，保留原失配分支而非复制私有表。
- 原始浮点常量用 PE bits，分组用指令顺序；差分失败先保存首例，再找原证，不以 epsilon 修复失败。
- 数学原指令、真实 CPU 接线、CPU 资源卸载、GPU与 OS 输入各自单列证据。跨架构 canonical NaN 符号须独立证明，不应扩展成全部 NaN 宽容。
