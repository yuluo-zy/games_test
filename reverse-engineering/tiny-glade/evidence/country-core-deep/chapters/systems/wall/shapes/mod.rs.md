# systems/wall/shapes/mod.rs

> 文件作用：形状子模块根：add_wall_shape_ux 注册 CalcShapeHeight/PreviewCircle/PreviewRectangle/ShapePlacement 四资源。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::impl$0::add_wall_shape_ux` | 0x140945f50 | 28 | add_wall_shape_ux（4 资源注册） | O | wall\decomp\decomp-0x140945f50.txt |

## 实现方式（算法级）

**add_wall_shape_ux（O）**：shapes 子系统的 `App` 插件注册函数（`app.add_plugin(ShapePlugin)` 等价物）。处理链：

1. **8 次系统注册**：`FUN_1400dd6f0(lVar6, hash1, hash2)` ×8，其中 `lVar6 = FUN_141542a30(...) + 0x38`（App/World 结构 +0x38 处的调度表入口）。每次注册携带一对 **128 位类型哈希**（Bevy `IntoSystemConfigs` 的 system type-id）：
   `0xc38a6a0c228dcd67/0x6c42844f9798e528`、`0x1a0858bd8956ea1/0x5c44f8ff69c6ded`、`0xdcbb5a7f787a5734/0x603a1659202e883c`、`0x4a1db5837d11c93f/0xb383024f698fb77d`、`0x58adde1121b70c78/0xf1ff5349e0cd3a82`、`0xf4e90bdaaff68cb/0x84dca00f0716e277`、`0xfbdd85d7ac4b26cb/0xa6ebe12552cee171`、`0x23ea1ac5bdb3f981/0xfbcf99f336ac1d2a`。
   每个注册后紧跟**类型表检查**：`lVar3==0 || 哈希越界 || 表中条目空 || 函数指针空` 任一成立则调用 `FUN_141517790/1415120f0/14150a530/1415139b0/14151c1d0/14150f5a0/141519cb0/14150d080`（各系统类型自注册兜底）。8 个系统即 shapes 子系统全部业务系统（calc_shape_height/shape_placement/shape_placement_signifiers/ui_create_circle/ui_create_rectangle/ui_edit_circle/ui_edit_rectangle_dims/ui_edit_rectangle_orientation 等；哈希与系统的——映射 U）。
2. **前置注册路径**：`FUN_140b16c50(auStack_258)`（初始化构建器）→ `FUN_1417332c0(&uStack_108)` → `FUN_14175dce0/141450870/141453000/14144f660/140b67280(param_1,&uStack_178)` —— Bevy App builder 链（add_systems/init_resource 等价物），`auStack_1e8[0] = 0x11`（17 = 调度 set 标记）。
3. 资源注册以类型推断进行（无显式参数），注册对象即本章列出的 `CalcShapeHeight/PreviewCircle/PreviewRectangle/ShapePlacement`（I，来源为各系统参数面）。
4. **类型名读取**：`FUN_140b218a0(uRam000000014362c370, puVar4, &UNK_142c5fd60)` 读全局 0x14362c370（App 类型注册表）——`UNK_142c5fd60` 为类型名胖指针（--fat 解析失败，U）。

**数据结构**：无游戏自建记录；全部操作走 Bevy App 构建器栈布局（uStack_178 = 16B App 句柄 + 0x1e8 调度配置）。
