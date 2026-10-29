# M01 墙高控件与历史复核（2026-10-26）

基线：main `858da5891`。本轮只补原 `ui_adjust_wall_height`（140b2fc00）及其必要控件工厂，复核既有 `ui_adjust_wall_height_mode`（140ac6d60）、`history_record_adjust_wall_height`（140ac79e0）与 clamp（140ac7f30）。聚焦预制墙的 `ui_focus_mode_shape_height` 和 Raise 保留，不另建替代事件或第二套墙状态。

原始身份：RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。共享 Ghidra 只读调用均在 `isError=false` 后解析内层结果；完整 body/包装/两个闭包保存在本目录 `m01-height*-native.json`。RNE objdump 的 `m01-height-{ui,widget,signifier,factories,clamp}.asm` 是控制流与常量判据。

新增UI文件前已实际查看B骨架同路径`reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs`，原Source SHA256 `385d5bd51199fed384c80b21a4006c12709b7007e71c448624809097c63a498c`。按此所属目录恢复，未创建新目录。

## 原调用合同

生产者严格对应 PDB 十一项参数：AppMode、PublicWalls、Roof Query、UiState、Finish/Adjust/Init 三事件 Writer、RoofAnimation Query、UiSignifierStream、MainCameraMoved Reader、GameTime。没有临时资源、假输入或新增 command。

| 链条 | 恢复/核验结果 |
|---|---|
| 候选 | 外层 mode>=6 先返回且不消费相机事件；屋顶已关联时 Query 动画首通道必须有序等于1；PublicWalls 再验证曲线；bulldoze 跳过 |
| 聚焦 | Wall/Roof 仅当前ID且shape>=2；其余编辑子模式返回；预制聚焦球由既有专属系统负责 |
| 体积 | 真正注册 `edit_height_{WallId}` SphereCloud；读取原+0x88的height_outline，按原 min(points, heights)；高度取已准备轮廓对应的 arch_segment_heights，模式0加0.6，模式1按 flat+rim 加精确0xbe947ae2 |
| 半径与反馈 | mode0或预制0.5，其他0.7；相机运动静音四槽，否则tags=1/2/3/3；保留extra=0x202/0、record_history flag1；普通选择遮挡、聚焦不遮挡 |
| 拖拽 | 跳表142aeba90 四项分别30577/304a1/30582/305e9；phase2无事件，3→Adjust+Init，4→Adjust，其他→Finish；相机移动时继续已经捕获的拖拽 |
| 画面 | 普通选择需要真实context.is_active，生成Empty Mesh并挂WallHeightSignifier，由既有定制动画展开；聚焦每帧带frame_count名字的Points工厂；两者真实DrawTopology::Alt(3)，瞬态网格有Receipt |
| 命令 | 既有投影mode首次抓取存offset，后续写真正AdjustWallHeightCmd；真实impl更新InnerWalls并写原音频事件 |
| 历史 | 原from/to分别clamp、有序相等跳过、NaN不等；缺墙保留原值；一个WallCompound的一组AdjustHeight；预制才Hint；只在Finish后清五个Option |
| 装饰物 | 保留既有mode对墙+地形的真实archive与culling，不在新UI复制第二套档案 |
| 卸载 | 真实signifier_create消费工厂后走mark→反向动画→despawn；TransientMesh Receipt进入统一回收 |

历史与 mode 原有业务经逐行/原ASM复核未发现需改算法之处，本轮只新增整链World验收，不为增加变更量重写已恢复代码。补充：本轮继续恢复了原高度装饰物六参/四reader消费者，见m01-height-decorator-recovery.md。公共图元变更/羊/树等跨系统消费者仍由统一墙变更接线负责，不能把UI产事件等同于这些消费者已验收；中央集成验收另记。

## 修复的旧误判

1. 旧UiWallHeight辅助 guard 将active_widget与wall裸ID比较。原signifier收到UiWidgetContext，必须比较格式化控件名字的真实WidgetId。
2. 旧signifier名字把结尾当点数。140956cfa保存的是R9D（GameTime.frame_count）；140956fa4给u32格式化引用的是该栈槽。聚焦必须逐帧换名以替换旧曲线。
3. 字面 `-0.29` 与原 `0xbe947ae2` 相差一ULP；用原bits保留近零高度与 signed zero 结果。
4. 既有signifier_animation误把原+0xe0数组映射为terrain_y；原140b2a890和freeform_height_cache证实它是arch_segment_heights，+0x88是prepared Curve3。已同时修producer/定制动画，测试使用真实prepare_paths后的cache，不能让高度标记画在地面。
5. 旧章节把roof条件称为no-rim，实际140b303ed对+3d是非零；生产者与动画都使用flat+rim，未照抄章节中的命名。
6. Ghidra把四项跳表反编译出多项伪case并越入rdata坏指令。本轮从原16B表解码四个相对目标，按有界ASM恢复，不扩大输入枚举。

## 独立验证

`m01-height-widget-native.py` 在隔离Unicorn执行原1409566d0，仅桥接allocator、format、UiState::add容器边界；保留全部原循环、浮点、半径及声音参数写入。160例覆盖mode×flag×camera×rim×shape×长度0/1/2/3/7，含-0、NaN载荷与±Inf。Rust测试读真实原输出逐位比较点、radius、声音四槽、extra、occlusion和flags。原输出及身份在同名JSON，脚本可重跑，不使用自己生成的数学公式作oracle。

World测试包括：真实控件phase3→4→1；投影mode首帧archive与抓取；后续权威命令、原音频载荷和历史一次记录；相机运动捕获；模式门；roof动画0/.5/1/NaN；两个真实工厂的定制动画、删除和瞬态网格回收。没有OS鼠标/键盘输入，不将这些World测试宣称为GPU/真实输入验收。

2026-10-26实跑 `CARGO_INCREMENTAL=0 cargo test -p country-core --lib --offline systems::wall::adjust_wall_height:: -- --test-threads=1`：**50 passed，0 failed**（完整日志`/tmp/m01-height-test.log`）。第一次50项实跑的三处失败为本轮测试fixture与旧动画断言：需要将UiState intersection_tag置0激活真实工厂、动画应对arch heights断言而非地面y；已修正并复跑全50项通过。中途他owner circle fixture include路径错误也已由其修正，没有加桩绕过。

## 下轮可复用经验

- 先由PDB拿SystemParam，再按真实调用地址分出生产者、mode、impl、历史、定制动画；不要以章节的推测参数决定资源接线。
- 匿名闭包的vtable可从原rdata读call槽，先跟call_once包装找到真实closure；本例14094f410→140957390、14094f4b0→1409576d0。
- 反编译出现坏指令/巨量switch时直接读有限原表；不在共享Ghidra修改函数体或重分析数据库。
- 对widget记录可在原UiState::add边界截取，保留原全部业务循环，轻量生成数百条bits fixture；不必模拟整个Bevy内存。
- 发布曲线只借用，只有原必须拥有的点云/工厂闭包分配Vec，避免为控件构造额外克隆两份发布数组。
