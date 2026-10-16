# HIGHLIGHTS — system-decorator ui_systems（D2，2026-10-16）

10 条内最重要发现（供中央汇总）：

1. **交互门控 = `DecoratorAffordanceDispatcher`**（非 DecoratorBackup）：`{state u64@0,
   payload u64@8}`，128 位检查等价 `state==N && payload==0`；值域 3=移动/旋转、
   4=高度、6=清空（`clear` 0x1407af330 写 {6,0}）、11=楼梯继续（payload 低字节
   '?'）、20 未知。各 UI 系统按此门控。
2. **目的 tag 双空间（纠错）**：`DecoratorInfo` 目的 tag {0,1}=wall/2=roof/3=terrain/
   4=stair（0x1408334e0/410/470 机器码复核 ✓）；`convert_raycast_hit_to_decorator_dst`
   0x14127c990 输出**自有 tag** {0=wall,1=roof,2=terrain,3=stair-top,4=invalid}，
   调用方按 `tag!=4` 查各自跳表（ui_move 0x142a829e8 等）。与 core `DecoratorDst`
   不同，勿混用。
3. **跳表索引公式统一**：`(tag-2)<3 ? tag-1 : 0`（tag∈{2,3,4}→{1,2,3}），多系统
   （rotate 0x142a82be8 / stair-height 0x142a832b0 / grab_offset 0x142c23990 /
   move_dst 0x142c248b0）一致；墙（0/1）独立处理器，其余同址/共用。
4. **0x28B 与 0x20B 双队列**：UI 系统推 0x28B `DeferredDecoratorOpQueue`
   （kind 0x12=烟囱高度、0x21=移动；记录 {seq@0, kind@8, payload}）；apply_decorator_op
   0x141276e30 写 0x20B `DeferredDecoratorEditHistory`（kind 3/4/5）+ 0x88B 事件。
   D2 初稿误名已由 D3 以排空面 0x140817850 更正。
5. **signifier 记录 48B {1,1,enc_id,id,pos,scale}** → `UiSignifierStream::write`
   （40B 拷贝 0x140b2cf90）；装饰域 name 槽 = 编码 id（`rotate_right(32)`）；
   mesh 名经 SignifierFactory::new（0x14089d8b0，宿主接线）。core 40B
   `UiAppendSignifier` 以 {position,scale,name} 承载，登记偏差。
6. **dragged 相位机**：`bVar20=(dragged-2)<3?dragged-2:3`——悬停(0)→仅指示符、
   拖中(1)→指示符+预览更新、释放(2/3)→`apply_decorator_op` + 推 0x28B op。
7. **'?' 与 ':' 装饰类型**：'?'（0x3f）= 特殊类型标记（楼梯继续/射线遮罩降为 0x23
   排除装饰碰撞体——convert 的 unreachable 前提）；':'（0x3a）= 不可分裂窗型
   （分裂系统 panic 行 40）。
8. **dispatcher gate 用 128 位数学检查 state==N&&payload==0**（playbook §2 的
   niche 模式；`(field0-2)==N` + 借位/回绕等价式），避免被 Ghidra 的
   `sbb`/`setb` 解串误导。
9. **旋转/高度/分裂处理器为函数尾部内联段**（Ghidra NotFound <40B）：跳表目标
   逐个 capstone 解算（rotate 0x1407a91bb/8766/87a6/8814；stair-height
   0x1407adba7/91a/998）；处理器几何以 I 登记，不作零证据推断。
10. **core 增补**：`UiState::{add,add_with_id,widget,widget_context}` +
    `UiWidgetContext::{clicked,hovered,is_active}`（ui-input 章 O：0x14092e5f0/
    620/680）+ `WidgetParams::volume`，随本批编译验证通过。
