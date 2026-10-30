# M01 demo / 开发者热键门：独立复核收据

2026-10-27。只处理maintain依赖的全局门身份、同TypeId语义和有界验证。中央负责输入翻译与宿主初始化，本轮不扩calculate_distortion或其它业务源。

## 两个全局的真实身份

RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。

| 原全局 | PDB PublicSym32 | main唯一已见写者 | 来源配置 |
| --- | --- | --- | --- |
| 143634d90 | `country_core::_DEV_OVERRIDE_IS_DEMO_MODE`；section3 +40336 | 1403dcf6a/f78 | GameConfig基址rbp+290，dev+6，即+E6 / rbp+376：`dev.demo` |
| 143634cd8 | `country_core::game_config::DEVELOPER_HOTKEYS_ENABLED`；section3 +40152 | 1403dd616/624 | 同一配置dev+2，即+E2 / rbp+372：`dev.internal_hotkeys` |

原导入槽14362c098指向d90，14362c020指向cd8。两者不能因地址接近或现代类型名含Demo而混用。PDB PubSym只证明名字/位置；实际一字节bool合同由原DevSettings八个serde bool、main逐字节取出这两个不同字段、相应reader共同确认。两位置均处于PE零填充范围，加载前均为false；主入口再分别写配置，绝非“两个值永远相同”或“main恒写0”。

`developer_hotkeys_enabled` 原14090e070仅 `movzx eax,[143634cd8]; ret`。原maintain140806149..153经导入槽取d90，非零提前返回；Place140a20c9a生成u45=`d90==0 ? 5 : 0`；new_wall14085a37e生成source+69同样为5/0。入口没有cd8参与，也没有观察到第三个有效demo门/getter。

原main反编译将dev.demo写出显示为 `local_8c8._6_1_`，hotkeys显示为同一DevSettings的`._2_1_`；不是两个名字相似的临时变量。原全部静态xref仅main各一处WRITE；其它是reader。这里不推定通过未知指针永远不能写，但没有依据将配置GUI每帧值实时镜像到原全局。

“原全局三态”旧注释错误：正常输入是配置bool的0/1。某reader比较==1、其它reader比较非零，不足以把非法bool字节2发明为Option第三业务状态。

## 发现与最小修正

现host给DemoModeEnabled写入的是`config.dev.demo`，所以maintain**原有运行值正确**；其类型旧注释却把地址写成cd8并描述开发者键，构成真实概念混用。输入translate_input据此读取DemoModeEnabled来开开发者热键，是另一处真实错误，已交中央修复。

本轮仅：

- 修正DemoModeEnabled注释，明确它承接d90/demo。
- 在原B `country-core/src/lib.rs`再导出同一类型为`DevOverrideIsDemoMode`。这只有一个TypeId、一个ECS状态，没有新增bool或默认listener。
- maintain参数使用语义alias并补原指令注释，门算法不变；现代其它旧caller仍可使用原名。

独立热键门复用现有`game_config::developer_hotkeys_enabled`权威AtomicBool及main对internal_hotkeys的初始化，不造第二份热键资源。其旧地址常量14362d6dd、以及“与glade椭圆门共用”的注释也错误；中央负责同文件窄修和输入消费者更正。

原14090dfe0的`is_within_glade_shape`也读d90，完整原指令40个输入已记录：full限制为[-65,65]方形，demo限制为半径65圆；hotkeys的0/1不改变结果。这里不是`GladeBorder`资源的demo半径15用途，不能把不同边界函数混作一个算法或说椭圆由开发者键启用。

## 原指令与真实World验收

[原PDB全局记录](m01-demo-gates-pdb-globals.json)、[静态xrefs](m01-demo-gates-original-xrefs.json)、[main原反编译](m01-demo-gates-main-native.json)、[七个原ASM区间](m01-demo-gates-original.asm)。

[原探针](m01-demo-gates-native.py)先锁RNE摘要、按PE sections展开，只将两个导入指针绑定到PDB静态地址。执行main两个实际writer；直接执行hotkeys getter、maintain完整入口到门后首个外部profile调用或自然return、Place实际u45计算和新墙metadata计算，附完整glade函数。没有以候选逻辑作oracle，也没有声称执行完整main或maintain的后续业务。

| demo | internal_hotkeys | 原getter热键 | 原maintain进入业务 | 原Place u45 | 原new_wall source69 |
| --- | --- | --- | --- | --- | --- |
| false | false | false | true | 5 | 5 |
| false | true | true | true | 5 | 5 |
| true | false | false | false | 0 | 0 |
| true | true | true | false | 0 | 0 |

[原结果](m01-demo-gates-native.json)的4组合被生成Rust夹具，由实际World通过maintain验证缺失节点删除/保留、同TypeId alias、现Place source helper的u45及WallHistoryMetadata+69。World的配置保留两个不同输入字段，不写共享hotkeys AtomicBool，避免该局部测试干扰其它测试线程；热键原getter本身已由原机器码独立验证，生产输入修复验收归中央。

`CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline maintain_stair_graph_recovery_tests -- --test-threads=1`：**8/8通过**，489 filtered，0.02s，日志 `m01-demo-gates-maintain-world-tests.log`。包含之前全部真实图/碰撞/归档/失败先后验收，没有用注释更名替代行为测试。

## 剩余边界与经验

本收据不将中央尚未验收的translate_input/host代码计作完成，也不修改其实现。旧`decorator_storage` change兼容分支、wall_splitter/merge_walls等仍有将此全局硬编码false的记录，应按各自真实业务owner处理，不能因B默认配置为false就宣称支持demo输入；该发现已同步中央，未在本轮越界修复。

后续读者应以原全局地址、PDB名字、写入字段和具体reader为四项身份依据。无参debug签名和同址callee名不能替代字段身份；不从默认值同为false推定两开关相同；不把cmp1与cmp0在非法字节上的差异变造为合法三态；同TypeId再导出可以澄清原模块语义而无需新建镜像状态。
