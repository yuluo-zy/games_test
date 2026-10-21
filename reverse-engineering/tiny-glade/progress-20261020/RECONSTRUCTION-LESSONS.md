# 2026-10-20 并行恢复经验

## 正确性约束

用户要求保持原作业务逻辑、算法与交互完整性。允许改进设计和接口，不能用简化分支、默认资源、空消费者或提前宣告成功代替原逻辑。沿用目前重写后的 rhapsody 与工具库；原二进制确定行为，不要求恢复旧工程布局。T13 反馈捕获仍排除。

## 本轮有效做法

1. 先核对当前目录和产物身份。原 B 镜像实际位于 workspace/input/tiny-glade.exe，SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。旧安装路径、evidence 目录、public-symbols 与原 PDB 均不能假定仍存在。
2. 五个 agent 按文件族分工；公共类型、宿主注册和 Cargo 只由中央协调。先冻结源码和 SHA，再运行统一检查，避免多次重编译和相互覆盖。
3. 原机器码执行与生产 Rust 使用同一组原始输入和保留输出。几何比较包含采样坐标、顺序、完整载荷和保留字节，不能只比较几个视觉结果。
4. Oracle 的 hook 是验证边界。替换地形、物理或投影返回值，只证明调用者算法；不能据此宣称共享采样器或完整游戏已验证。
5. 让不同 agent 交叉审阅源码和验证脚本。本轮找出了 AppMode/EscMenuState owner 混淆，也找出了 quaternion 测试改了字典却未更新实际输入的问题；修复后重新生成 native fixtures。
6. 检查真实消费者和副作用。存档不能直接生成缺组件的 Roof/Tree 实体；应发原命令，由当前 canonical consumer 创建完整实体、颜色、音频、几何和后续事件。
7. 顺着注册配置追到 schedule label 和 set 的真实身份。本轮由 PE 的 parent→converter→vtable→run wrapper 确认 staging 消费者，再由 fmt 原字符串确认 Update/HistoryStart。资源查询 TypeId 不能单独证明 schedule label。
8. 加载的完整性必须影响状态机。存在实际应用缺口时进入 Failed，不能只 warn 后发送 OpenFile/LoadingGameComplete。成功排队原命令也不能等同后续 GPU 就绪。
9. 测试使用真实构造器和新 API：MainCamera::new(aspect)、UiState::at_startup、positions_without_indices。不要为旧测试加回底层旧接口。
10. 专项测试放独立 integration target，减少无关依赖，但不能把编译或过滤零项算作验证。第一阶段六个外部 fixture 缺失阻断已在同步后消除；本轮 country-core lib 编译成功，实际执行范围按过滤目标记录。

## 验证分层

分别记录：原证据已读、算法已恢复、生产 Rust 对照通过、真实 ECS/文件状态路径通过、主程序编译通过、设备和游戏交互验收通过。任何一层未通过，都不能用另一层替代；本轮不按函数数量直接上调用户给出的总体百分比。

## 第二阶段补充经验

11. 对照实际布局基址：ExtraInfo+0x30 的 tag 在 Info+0x41，不能把 destination+0x10 的判别混入。逐字段写入必须保留 inactive/padding 字节。
12. 通过原 System vtable::name 识别注册成员和嵌套 tuple；保持 deferred boundary。宿主已有拆分实现时只接正确的前后顺序，不再添加旧大函数或重复系统。
13. 原构造器树应实际执行 full/demo 变体，保留父 Entity、FnOnce 释放、按钮 cooldown 与 callback 的先后差异。
14. 真实 ECS 需覆盖非空 owner、实体 generation 复用、跨帧 Local、删除/失败路径；字段值相等并不能证明 Commands 插入、生命周期和事件次序相等。
15. 保留 MINSS/MAXSS、NaN、signed zero 与 duration=0 的原算法。数学上相似的 clamp、除法或插值不必然位级等价。
16. 原码 oracle 的 replacement/hook 逐项标明；原码执行、生产对照、真实 ECS、整机运行分别计数。部分 signifier easing 原码覆盖不等于全部 tag 覆盖。
17. 清理方法须追到真实调用时点及所有容器；不要把 tracking drain 写成清空 authoritative store，或把 validator 内操作挪到任意帧尾。
18. 共用 shader/audio/缓存 owner 使用当前 canonical 类型。遇到缺失 producer，记录缺口，不创建空资源掩盖失败。
19. 冻结实现后统一 Cargo；新模块文件先落盘再导出。最终 manifest 重新绑定实际源 SHA，旧 probe source snapshot 只代表当时取证版本。
20. 所有百分比须来自统一清单复审；叶函数数量、原码样本数量和测试数量不能代替业务完整度。用户要求停止测试时立即停止，保留已完成结果与未验收边界。

## 第二阶段补充经验

11. 对照实际布局基址：ExtraInfo+0x30 的 tag 在 Info+0x41，不能把 destination+0x10 的判别混入。逐字段写入必须保留 inactive/padding 字节。
12. 通过原 System vtable::name 识别注册成员和嵌套 tuple；保持 deferred boundary。宿主已有拆分实现时只接正确的前后顺序，不再添加旧大函数或重复系统。
13. 原构造器树应实际执行 full/demo 变体，保留父 Entity、FnOnce 释放、按钮 cooldown 与 callback 的先后差异。
14. 真实 ECS 需覆盖非空 owner、实体 generation 复用、跨帧 Local、删除/失败路径；字段值相等并不能证明 Commands 插入、生命周期和事件次序相等。
15. 保留 MINSS/MAXSS、NaN、signed zero 与 duration=0 的原算法。数学上相似的 clamp、除法或插值不必然位级等价。
16. 原码 oracle 的 replacement/hook 逐项标明；原码执行、生产对照、真实 ECS、整机运行分别计数。部分 signifier easing 原码覆盖不等于全部 tag 覆盖。
17. 清理方法须追到真实调用时点及所有容器；不要把 tracking drain 写成清空 authoritative store，或把 validator 内操作挪到任意帧尾。
18. 共用 shader/audio/缓存 owner 使用当前 canonical 类型。遇到缺失 producer，记录缺口，不创建空资源掩盖失败。
19. 冻结实现后统一 Cargo；新模块文件先落盘再导出。最终 manifest 重新绑定实际源 SHA，旧 probe source snapshot 只代表当时取证版本。
20. 所有百分比须来自统一清单复审；叶函数数量、原码样本数量和测试数量不能代替业务完整度。用户要求停止测试时立即停止，保留已完成结果与未验收边界。

## 第二阶段补充经验

11. 对照实际布局基址：ExtraInfo+0x30 的 tag 在 Info+0x41，不能把 destination+0x10 的判别混入。逐字段写入必须保留 inactive/padding 字节。
12. 通过原 System vtable::name 识别注册成员和嵌套 tuple；保持 deferred boundary。宿主已有拆分实现时只接正确的前后顺序，不再添加旧大函数或重复系统。
13. 原构造器树应实际执行 full/demo 变体，保留父 Entity、FnOnce 释放、按钮 cooldown 与 callback 的先后差异。
14. 真实 ECS 需覆盖非空 owner、实体 generation 复用、跨帧 Local、删除/失败路径；字段值相等并不能证明 Commands 插入、生命周期和事件次序相等。
15. 保留 MINSS/MAXSS、NaN、signed zero 与 duration=0 的原算法。数学上相似的 clamp、除法或插值不必然位级等价。
16. 原码 oracle 的 replacement/hook 逐项标明；原码执行、生产对照、真实 ECS、整机运行分别计数。部分 signifier easing 原码覆盖不等于全部 tag 覆盖。
17. 清理方法须追到真实调用时点及所有容器；不要把 tracking drain 写成清空 authoritative store，或把 validator 内操作挪到任意帧尾。
18. 共用 shader/audio/缓存 owner 使用当前 canonical 类型。遇到缺失 producer，记录缺口，不创建空资源掩盖失败。
19. 冻结实现后统一 Cargo；新模块文件先落盘再导出。最终 manifest 重新绑定实际源 SHA，旧 probe source snapshot 只代表当时取证版本。
20. 所有百分比须来自统一清单复审；叶函数数量、原码样本数量和测试数量不能代替业务完整度。用户要求停止测试时立即停止，保留已完成结果与未验收边界。
