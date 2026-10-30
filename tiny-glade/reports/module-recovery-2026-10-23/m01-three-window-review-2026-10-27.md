# M01 三类窗户入口的独立有界核验

本轮核对 `cottage_wall_window.rs`、`cottage_corner_window.rs`、`gothic_wall_window.rs` 的实际主生成器，不以旧family测试数量或进度表判断完成。保留现有手工重构的骨架、helper和蓝图owner；没有另建目录或 renderer。

## 原B入口与完整参数

实际读取 B 骨架三个原文件及各自 PDB procedure 记录，并从原RNE输出完整地址范围的ASM。原码与分析EXE的范围一致性、完整参数和文件SHA在 `m01-three-window-identity.json`。

| 入口 | PDB完整过程长度 | 原参数 |
| --- | --- | --- |
| 农舍墙窗 `1407ebfd0` | 14303B | DerivedDecoratorInfoResources、Storage、CachedTransforms、PublicWalls、Blueprints、AssetMeshLibrary、UiSignifierStream |
| 农舍角窗 `1407f66b0` | 9983B | DerivedDecoratorInfoResources、Storage、CachedTransforms、PublicWalls、Blueprints、AssetMeshLibrary |
| 哥特墙窗 `1407e87b0` | 10658B | DerivedDecoratorInfoResources、Storage、PublicWalls、Blueprints、UiSignifierStream |

Ghidra 当前识别的 body 分别仅11843/8168/8889B，fresh C不包含全部主分支，不能把该大小当作完整过程或只审 C 中可见部分。完整原ASM保留在 `m01-window-*-complete.asm`。

三个原入口都是 Storage 外层墙表，按0x98B桶步进；不遍历Terrain/Stair。原 `140a1b100` 迭代器有ICF别名 `iter_removed_ids`，不能按名字解释为removed集合。每个墙桶的 `iter_modified@140a1b070` 返回modified迭代状态和该桶指针；主函数的内联next必须在同桶查到Info，再调用subtype。查不到沿 `unwrap_failed` 失败，不能静默filter_map。

## 修复与业务核验

三个入口均改为显式遍历 `modified_ids` 并在同桶required lookup，保留查表先于subtype/目的地门控的顺序，不全局改core迭代API。补哥特窗第五个必需UiSignifierStream资源，保留原调度依赖；不凭该依赖生成默认反馈。

原农舍墙窗在 `1407eccd3`、角窗在 `1407f6e7b / f6f22 / f7020` 明确调用 `xz_projective_rotation_bad@140c911d0`。对应生产调用改用既有Vec3 XZ投影专化，不继续借用名字近似的Vec2入口；没有改共享数学helper。

| 生成器 | 保留的实际业务 |
| --- | --- |
| 农舍墙窗 | subtype0/1与rank1..3；矩形缓存/曲线两端位姿；预制墙前移与玻璃后移；真实collision/interaction Arc；圆墙洞宽修正；普通窗lintel、半老虎窗孔、非预制碎玻璃；cloth/flowerbed/shutter的独立RNG与真实候选 |
| 农舍角窗 | subtype3；矩形角附着与非矩形三次try_normalize失败skip；两侧姿态和主框前移；真实网格、玻璃、主洞+lintel洞；rank1/2不同cloth宽度、cloth前移和flowerbed资产/候选 |
| 哥特墙窗 | 原 `(subtype & 1eh)==6` 且dst-tag<=1；rank宽度、真实帽骨；半木/半老虎窗full-hat选择；rank1砖/其它rank变形；颜色999的独立frame shader；预制玻璃与albedo；真实碰撞；三张墙洞表；subtype7的rank尾表老虎窗孔 |

保留原失败/部分写入顺序：例如角窗先发布frame和outline再读interaction，哥特窗先发布帽/玻璃/frame再查interaction，不能为了原子回滚抹掉这些既有副作用。现有逐项required失败测试继续运行。

共享 `insert_for` 的状态参数本轮由中央按原码修正：state写独立`u138`，interaction flags默认0。三窗口测试同步核对该合同；正式state0才产生两张物理表项，preview state1保留半透明视觉但不产生正式碰撞，不沿用旧错误flags断言。

## 本轮新证据与真实World

`probe_m01_window_branches_x86.c` 在按PE sections映射的原RNE中直接执行完整seed函数、完整XZ投影函数、原cloth比较分支和原首个fastrand指令块。宿主仅桥接输出出口；没有手写RNG、cloth或四元数公式。384组原输出保存在 `m01-window-branches.json/.rs`，probe、原输入/输出与fixtureSHA见 `m01-window-branches-native.json`。这不是完整原生成器或Bevy容器的执行声明。

Rust对原384组seed和四元数逐位比较。三种农舍支路各rank共1152次真实非空World生成，使用含两个真实位置/法线的花槽资产，核对原cloth门、首随机数flowerbed门和shutter位；避免空asset使未实现分支看起来也输出0。候选带真实墙ID，主记录map ID与seed来源Info.word2c保持分离。

新增真实World→正式 `instantiate_blueprints` 验证5个实际subtype×3rank×正式/preview，共30场景：

- 三入口真实资源生成非空蓝图，随后真正fullvisual构造Nani实体、WindowLight/Color/SemiTransparent组件、墙洞与两类Rapier碰撞。
- 相同蓝图下一帧保留实体身份，仍重新收集逐帧墙洞；删除蓝图后实体、物理表、cloth候选卸载，移除目录和previous owner后碰撞mesh Arc归零。
- required modified记录缺失必须先于subtype失败，Terrain/Stair坏modified不被三入口误消费；哥特窗缺第五资源在蓝图发布前失败。
- 另为circle建立同一Circle2d采样曲线、原shape半径与真实PublicWalls的一致输入，避免只换shape tag却保留矩形曲线/零半径的旧fixture被当作有效圆墙场景。

最终执行 `CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib window --offline`：45通过、0失败、0忽略，包含上述新增384组原结果/1152次支路以及30个正式预览、15个一致圆墙场景。执行日志、失败日志与源指纹在 `m01-three-window-validation.json`。日志保留了旧state/flags断言失效以及测试接口错误的过程，没有新增skip、放宽数值比较或公开private生产方法来过测。

## 真实剩余依赖与范围

本轮三个主生成器的分支和fullvisual合同与整个文件/最终GPU画面分开记录。`calc_gothic_half_dormer_profile_ws` 不是这三个主生成器的callee，现由freehand owner恢复dormer tile链；本轮未覆盖该函数，也保留其owner的改动。

哥特窗产生的 `GothicWindowBrickDeformation`、`BrickColor` 仍需后续独立Nani leaf完成canonical TypeId与真实GPU上传；builder当前本地pending类型不能因此冒称已在画面渲染。对应原入口为 `nani_update_solid_vertex_color_bricks@142076230` 和 `nani_update_gothic_window_bricks@142076e40`。该必要依赖由中央另行授权推进。窗口花槽和cloth候选的下游独立模块也不能仅凭本轮producer输出就视为整个业务完成。

## 留给下一轮的经验

1. 完整PDB过程跨度、fresh反编译body和库别名是三种证据，不能混用；缺的基本块必须回原ASM。
2. required查表的失败顺序也是业务，不能通过公共filter_map把坏记录消失；修真正消费入口，不扩散全局迭代器语义。
3. Info里的ID字段、map key和Entity不相同。装饰存储是纯数据，持久实体由DecoratorVisualState关联，不应为测试强造DecoratorId组件。
4. 近似命名的旋转helper应按原callee选择；复用已经恢复的正确专化，避免重写整库。
5. preview状态与interaction来源flags分离，必须有正式/preview两类物理验收。
6. 测试数据必须在有效域自洽；真实圆墙不能仅改tag、真实随机候选不能仅给空资产。
7. 原数值块差分、真实World/fullvisual以及最终GPU像素分别记证据，不相互替代。
