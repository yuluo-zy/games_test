# 自主推进的执行顺序与验收边界

2026-10-20 用户明确授权自主判断：先将主链路全部复现完成，再深入每个核心业务模块。沿用原作先取证、保留现有重构架构、不以简化掩盖缺口的要求；root 负责方向、生产整合、构建和 GPU，既有负责人提供有界候选与独审。维持总数不超过 8，不再创建子团队。

## 第一阶段：主链路优先

实际入口为启动资源注册 → loading/gameplay App 交接 → OS 输入 → 同一 World Update → 实际实例/栅格/光照数据生产 → 世界图 → 后处理 → 主图提交 → 交换链呈现 → 编辑与保存/加载 → 第二 session/reload → 正确清理。

完成要求包括实际完整源码构建、非空真实数据经过原 shader 与间接绘制、连续帧呈现、首笔真实输入编辑与存档重载、退出与第二 session，以及相关失败路径。记录实际硬件帧耗时和资源持有情况。空实例、手造加载完成事件、替身资源、固定 draw count、忽略 GPU 错误或未执行测试均不能替代这些结果。原版 B 有界证明与最终目标 C 范围继续分开。

当前优先处理的是已存在消费者的三个具体断点：

1. **Nani producer 与 set44 顺序**：从现 7 类维护候选推进可证的 9 类 canonical 更新；恢复原 VAO build → rebuild → maintain → 显式 ApplyDeferred → update tuple → flush。不能把整个资源维护从 PostUpdate 粗暴搬移，也不能重复注册补顺序。
2. **真实场景杂物 producer**：当前 decorator clutter 创建 builder 后直接丢弃，没有 solid_texture 的 Some writer。以原完整 native body、helper、筛选/几何/资源和实际生成/删除为恢复单位；canonical 类型迁移本身不代表场景已产生。
3. **reverse blur 顶层内容依赖**：原 shader 实际采样新 full 顶 mip 后丢弃其结果；现同步升级把内容策略覆盖成 preserving。需同时保留原同步行为与准确内容合同，不能仅扩大许可、标 inactive 或补无原证清零。

待用户补交的模块保持明确边界。2026-10-20 本轮 fetch 成功，HEAD 与 upstream 仍同为 bd8f74a5cfb3c9cc279efb1abd27695bb1bfb8bf，sampling 仍缺源；继续其余已有源码工作，不重建这些待补交实现，也不借旧 country-core 二进制验收当前源。

## 第二阶段：核心业务逐模块深入

第一阶段通过后，按当前源码的权威模块与实际调用关系逐项取证和修复：启动/会话、输入/工具模式/UI、墙/屋顶/楼梯/碰撞、地形/水/栅格、门窗与装饰/材质、植被与生物/动画、光照与天气/后处理、资源库/热更新、历史/存档/预制件/回放、音频及平台接口。公共依赖和 derive/注册胶水复用原锁定依赖，不按同名符号数量计算业务缺口。

每个模块都按真实输入 → 权威状态 → 修改/通知 → 消费输出 → 历史/重载 → 删除/清理检查；核对完整原函数范围、分支、数值顺序、RNG、错误和性能。文档分别记录原事实、实现、同源编译、CPU/GPU/场景验证与未知项。某个局部通过不升级为整模块或整游戏完成。

## 本轮人员与边界

- scene_startup_team：canonical selector 与 9 类 Nani 维护闭包；不改另组 runtime/game_loop 或 clutter。
- main_loop_team：原两段 VAO 行为与 Update44 前段；不改变无关 reclaim/reload/geometry。
- main_graph_team：真实 decorator clutter 和必要 helper；不覆盖 canonical/实例维护组的共享文件。
- rhapsody_correctness_team：reverse blur 原内容合同与渲染图窄修设计。
- validation_integration_team：独立核原证、设计与冻结候选；不以作者解释代替原事实。
- root：处理共享接口冲突、合法 claim、生产整合及真正构建/GPU验证；其余已有岗位待实际缺口再启用。

当前主链路仍未完成，第二阶段尚未宣称开始或完成。

## 后续主链路收敛点（2026-10-20 续推）

reverse blur 库内容合同、精确 view 布局与原始 shader 的 N=1/N=3 本机 GPU 像素回读已有限定验证；当前 rhapsody CPU 387+4+7 通过，先前 65 项兼容和图合同修复后的 26 项选定 GPU 另有冻结输入，不外推整帧。真实 clutter Startup、typed catalog、几何/owner、blueprint 生成与 collider-only 消费已 source-integrated，上层新测试未执行。Roof 仅接实际 Roof-bearing 子域，仍九类完整 ECS 合同加受限支线，不计十类完整。详见各 `validation/*integration*.json`。

最新缺口是完整视觉差分消费者与原真实调度，不是另造 catalog→Atlas 桥：已有 PreStartup 从 RON 构造同一 MeshAtlasLibrary/Nani/typed buffers 的路径。观察到的五项 RON 对照与原 helper/builder 的 subset 不同，但侧车未绑定原 B 同发布，不能据此改原常量或复制 Atlas。原 typed enum 输入 0..61/63 已证，62 不在已证构造域；当前 raw-u8 的 checked 拒绝不算缺失的合法业务。

既有八人继续按以下依赖分工，不增加 agent：

- main_graph_team：完整 7991B visual 消费与 diff/实体生命周期；保留每帧输出先于 sameArc/Eq 短路，不每帧重生全部实体。
- world_graph_team：`TG-c89…/window-candidates-20261020` 中恢复 WindowAutoClutterCandidates 的 80B 元素、ownership/构造与消费者；索引仅 namespace 候选，不据此猜原声明位置。
- scene_startup_team：`TG-dbc…/cloth-visual-state-20261020` 中恢复 ClothInfo 与现 DecoratorVisualState 首 Vec；不得混同 Wall raster 20B 参数或 Nani ClothlineCloth。
- performance_team：`TG-bd3…/deco-trackers-20261020` 中恢复 DoorStairs/Chimney 两 tracker 的真实 add/remove/with_diffs 与 float Eq；避免当前全帧 clone 和位比较替代原语义。
- rhapsody_correctness_team：原 decorator phase 注册与真正 clear_modification_tracking 合同；初查原 cleanup 位于 set25 validate 入口，不能按名字放到帧尾，更不能清空权威 decorators。
- main_loop_team：现初始 Atlas 调用/owner/上传等待链、资产身份与 live_link 四链独立调查，不重写已存在加载路径。
- validation_integration_team：逐设计与冻结候选独审；root 是唯一生产写入、构建、GPU 和 Git 操作者。

17:36 UTC 再次 fetch 成功，远端仍 bd8f74a5…，sampling/world_frame 仍缺源。见 `SYNC-RECHECK-20261020.json`；没有占位桩或旧 core 二进制验收，也不因此停止已有业务取证。

### 主视觉与验证闭包的续分工

中央仍独占生产写入与执行；总计沿用 7 位协作者，无新子 agent。01 负责 location→validate反馈与真实 decorator mesh/outline loader，新增必要 audio/UI/subtype/outline/walls-mod/load/cull claim 同 owner；02 负责 blueprint/body/精确Eq（Eq字段子批已合）；03 负责 canonical Window 与原真实drain/manager，三个consumer claim已给；04 负责同一 VisualState 的 ClothInfo、原 clothline/flag_connections 与builder三个None门；05 负责 per-wall tracker及原两assembly入口（显式 Removed消费，撤销自动空清理）；06 负责原phase/cleanup事实，并受root原 building-audit owner明确委派恢复 collision/world 的固定slot2真query/过滤/Option原因；07 独审各冻结合同与候选。

所有新增 claim 使用原 task_queue 正常领取。TG889 Storage foreign owner terrain-curve、TG-abf components/mod foreign owner meihost 均未改锁/越写。新canonical组件必须有实际数据与consumer合同，类型放置尊重现用户架构，不因锁冲突另造镜像；若换现组织路径须明文映射并独审。

接线门槛保持：不把空资源/未注册helper/全世界respawn/总是valid/错误bool反转/丢弃quads当功能完成；不把source-only测试当已执行。完整core/App仍需用户待补交源码，当前工作不会用桩绕过或用旧rlib冒充。


### 02:59 续推记录

root 保持唯一生产/Cargo/GPU writer，7个既有lead继续，没有新建agent。新增实际结果是共享数学原样源码7项测试通过，以及Rectangle原方法、有限最近射线的独审整合；完整App仍有已知缺源。最新限定源核验v10为65文件/61Rust全部预期指纹及语法通过，不作为游戏完成比例。

- main_loop_team：11reason/完整位置、Curve严格f32与旧增强测试语义、Bounds；同合法subtype/loader owner统一原DerivedDecoratorInfo两句柄合同，纠正当前bbox宿主record误映射。
- main_graph_team：真实Dormer48B writer→洞上传，以及独立Storage/Roof→嵌套roof几何→DormerTiles分支；新3原组均已正常领取，2个上传/constructor候选等独审，不以空列表fixture称场景。
- world_graph_team：4个窗口消费源已独审，原manager→animation、ignore_deferred、set0x24=36与真实event生命周期合同继续收敛；共同process metadata丢失为下一closed-chain入口。
- scene_startup_team：共享math已实测；canonical NaniCloth3源设计独审后候选，完整ClothInfo→真实garments/cache/纹理owner继续，不改foreign components/mod。
- performance_team：两真实砖构造按原−0.4与Door完整表达恢复；9参anchor-follow原任务TG-2dfe已正常领取，真实两tracker dirty传播与actualowner初始化为必需链，不添猜测fullvisual参数。
- rhapsody_correctness_team：finite nearest源已合；Snap五态与真实fullconsumer继续审；完整core的14项物理测试模板留档，已知缺源未变故不重复Cargo。
- validation_integration_team：独立核原字节/候选/阶段合同；源pass、已执行slice和完整App/WSI分别记录，V1 Window注册补丁HOLD，等纠正原顺序与App事件companion。
