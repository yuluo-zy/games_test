# 全员共同启动约束

用户最新要求：总共8个agent（中央root+7个执行者），禁止创建任何子agent。此前56人和组内6人要求已撤销；不要唤醒旧子agent。当前任务仍是正确、高性能地打通真实主循环和渲染循环。

## 起点与真实输入

- 工作根 /Users/yuluo/Desktop/game；Git仓库 /Users/yuluo/Desktop/game/games_test；Rust workspace 为其中tiny-glade。同步基线bd8f74a5cfb3c9cc279efb1abd27695bb1bfb8bf，HEAD与origin/main一致。先读当前源，再读与你闭包有关的既有合同/候选，不使用旧统计推断实现完成度。
- 用户已有未提交的3个源文件必须保留且不写：crates/systems/critters/src/bird_spots.rs、sheep/sheep_behavior.rs、sheep/sheep_walk_animation.rs。原始指纹在../SYNC-BASELINE.json。当前checkout有部分声明却缺文件的问题，必须核实，不能凭调用名声称已实现。
- 阅读 reverse-engineering/tiny-glade/AGENTS.md、AI_REVERSE_TOOLS.md、skeleton/task-index/AGENTS.md，以及 /Users/yuluo/.codex/skills/tiny-glade-recovery/SKILL.md、native-rust-reconstruction/SKILL.md。先复用保留原证；B原发行RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，路径 /Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne。目标版本边界明确，不把B证明写成C等价。

## 工作方式与权限

1. 每人对本文件指定的消费者闭包独立推进，禁止spawn或转派agent。复用旧子任务已经完成的证据，但不得将其唤醒。
2. 保留用户现有重构rhapsody与core架构；原证用于纠正行为和补必要API，不能重写另一套引擎/ABI镜像/平行资源类型。锁定公开依赖直接复用。
3. 生产源码、Cargo/lib/注册、Git变更由root唯一整合。你只在合法claim的work_dir中输出候选和最窄patch。不得改/抢foreign claim，遇外部owner先给root接口需求；已明确委派的root旧claim可以在原work下新子目录，不改变原锁。
4. 只读Git status/diff/show/log允许。禁止自行Cargo/build/GPU/清target/杀进程/提交推送，避免7人争用设备和缓存。只提出最小必要验证命令，由root统一跑。shared Ghidra/ReVa坏库不重启、不修改；有界已有asm/PDB和PE+capstone优先，必要工具调用遵守AI说明。
5. 保持顺序、条件、状态和失败副作用。禁止假默认Resource/View、空世界成功、固定max替代GPU count、CPU回读绕GPU驱动、清未定义历史、取消guard、吞错、仅改注释假装接线。
6. 性能属于各闭包验收：解释新增每帧分配/锁/阻塞等待、缓存的正确失效条件和资源有界性。优化结论必须区分机制、实际测量与未测，不以lavapipe耗时代表M3。先保正确再对真实热点做窄修。

## 交付协议

- 首报：当前源中的第一个具体可达阻碍（文件/函数/调用者/实参或资源身份），已由用户修好的旧问题，所需跨任务接口；不要泛泛统计TODO。
- 实现前：写清原作事实/当前工程适配/未知项，列真实输入→状态→输出→消费及失败/取消/重载。无法取证的分支不能自创。
- 实现后：交完整可合并闭包、每个源before SHA、candidate/patch SHA、实际原证范围及版本身份、最小调用接线hunk、必要回归用例；缺模块导出/依赖边也明确给root。
- 独立审查由root交给验收agent；作者不得自称原版等价。对审查发现立即在同候选修复，不用增加镜像或放宽断言掩盖。
- 仅在源/依赖/输入改变或具体歧义出现时补证/重验，复用未变证据，不重复全PDB/SPV扫描。结果区分：已证事实、已合源码、当前运行结果、剩余消费者。
- 不把报告、case数、哈希检查或编译通过当整条游戏链完成。完成必须指向真实消费者和可观察输出。
