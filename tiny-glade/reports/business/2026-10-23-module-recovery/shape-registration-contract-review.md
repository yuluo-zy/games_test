# TB-R7：add_wall_shape_ux 原注册合同复核

原入口 `140945f50`，B `crates/country-core/src/systems/wall/shapes/mod.rs`。只读复核，没有修改生产文件或运行Cargo。二进制身份、26条FnSystem.name原PE位置/长度/构造指令、9资源默认写入、8事件及可重复原指令参数截获均在 `shape-registration-contract-native.json`。

## 原调度树

```rust
let a = (
    shape_placement_params,
    shape_placement_signifiers,
    calc_shape_height,
    ui_edit_circle,
    signifier_edit_circle_arrow,
    signifier_edit_circle_line,
    ui_rotate_circle,
    ui_create_circle,
    create_circle,
).chain_ignore_deferred();

let b = (
    (
        ui_edit_rectangle_dims,
        ui_edit_rectangle_rotate,
        ui_raise_shape,
        ui_adjust_wall_height,
    ),
    (do_ui_edit_rectangle_dims, do_ui_edit_rectangle_rotate),
).chain();

let c = (
    ui_move_shapes,
    ui_move_freehand_walls,
    ui_edit_freehand_walls,
    b,
    ui_focus_mode_shape_height,
    impl_raise_shape,
    ui_create_rectangle,
    signifier_edit_dims,
    create_rectangle,
    edit_rectangle,
    finish_edit_rectangle,
).chain_ignore_deferred();

(a, c).chain_ignore_deferred()
    .in_set(/* 原Set变体17 */)
    .run_if(country_core::systems::is_gameplay_active);
```

两个B内层tuple是普通并列组；不能平铺为6项chain，也不能只凭函数构造调用顺序为并列组成员增加先后约束。B的`.chain()`在两个组之间保留deferred同步语义。A/C/最外层使用IgnoreDeferred，不应统一替换为chain。

原 `141450870` 输出两个配置的Vec（len/cap=2），并非第三个boxed系统。反编译签名中的Box是IntoScheduleConfigs的目标泛型类型，与tuple成员不是同一层级。

证据：A构造`140b16c50`；B构造`1417332c0`（组4、组2、最外2）；C构造`14175dce0`（11）；最外构造`141450870`（2）；run_if构造`14144f660`。主体CFG正常字节1634，不能用PDB含EH的1736字节数推断连续主体覆盖。

系统名通过每个FunctionSystem的name字段指针和长度从原RNE读取，不由具有相同参数类型的函数名推测。run_if真实name在 `142c5fc6c`、长度`29h`；原目标`140a90e00`的返回条件为 `GameTime.ran`（原+30 byte）为1且`State<AppLoadingState>`首byte为1（当前实际enum为Game）。这是时钟ran门控，不能改为单纯LoadingState=Game。

## 原注册的事件与资源

**原本入口注册8个事件，显式插入9个资源。** 事件存在性检查保持已有Events内容；缺失时创建两空Vec和零事件计数，随后登记update回调，复用Bevy本身的框架行为。

| 原事件 | register_event helper |
| --- | --- |
| rectangle::CreateRectangleCmd | `141517790` |
| rectangle::EditRectangleCmd | `1415120f0` |
| rectangle::FinishEditRectangleCmd | `14150a530` |
| circle::create_circle::CreateCircleCmd | `1415139b0` |
| shape_placement::DontRecalculateShapePlacement | `14151c1d0` |
| ui_raise_shape::HandleRaiseShapeUiInteraction | `14150f5a0` |
| rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims | `141519cb0` |
| rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate | `14150d080` |

资源以下列typed insert_resource确认；数据偏移为原ABI，不要求复制到新Rust架构。enum非活动payload和padding没有显式写入时保持未知，不能把临时缓冲残留当默认业务数据。

| 原资源 | 原默认显式写入 |
| --- | --- |
| rectangle::WipRectangle | +0 u32=0 |
| shape_placement::ShapePlacement | +0 u32=0；+8 u32=0；+2c byte=2；+30..4f=0 |
| shape_placement::SnapMode | byte=0 |
| circle::ui_edit_circle::AnimatedCircleSignifierResource | +38 byte=2 |
| rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource | +0 qword=8000000000000000h |
| rectangle::ui_create_rectangle::PreviewRectangle | +0 u32=0；+10..1f f32=[1,0,0,0]；+20..27=[6,6]；+28 f32=0 |
| circle::ui_create_circle::PreviewCircle | +0 u32=0；+10..1f f32=[0,0,3,0]；+20 f32=0 |
| rectangle::ui_create_rectangle::ShapesDragMouseMoved | +0..c=0，其他padding未知 |
| calc_shape_height::TimberframeConstructionParamsCache | +0 u32=0；+8..f=0 |

当前旧`shapes/mod.rs`将PreviewRectangle描述为center+10/dims+18/orientation+20，与这里原默认构造的各字段职责不能直接混用。新增或修改默认实现前应以其实际消费者读取位置确认；尤其尺寸6、圆半径3和方向[1,0]不能沿用derive(Default)全部0。ShapePlacement、SnapMode、AnimatedCircleSignifierResource及UiEditRectangleActiveWidgetResource仍有字段/enum语义归属需要业务负责人结合消费者恢复；本报告提供准确原写入，不替缺失字段发明名字或变体。

## 复现边界与经验

`probe_shape_registration_contract.py` 使用持久化 `shape-registration-contract-input.json` 的已验证有限函数元数据和原RNE即可重跑。原 `140946154..1409465a7` 初始化指令真实执行，所有Bevy调用均截获跳过，resource lookup强制missing以走各事件原register分支。探针只验证原参数构造，没有运行框架注册、真实系统、窗口或交互，也不声明TB-R7完整验收。

Ghidra返回数据集来自revision `1791719479858:445f58e8:3`，批次函数元数据不是独立import身份凭证；RNE原SHA独立校验。所有辅助drop_ptr等ICF别名不用于认定事件类型。

恢复注册时应先输出结构树，再对每个叶取FnSystem.name。泛型目标类型与tuple元素、普通tuple与chain、chain与IgnoreDeferred必须分别核对；编译通过与运行次序正确是不同验收项。默认值应记录有效写入掩码，而不是对enum整块内存做零填充猜测。
