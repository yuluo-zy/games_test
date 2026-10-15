# game_config.rs

> 文件作用：**游戏配置与开发者选项**（MODULE_GAP 判 0% 覆盖，本节补齐）。配置解析/serde 为派生胶水；真实业务为 4 个小函数，均 Ghidra NotFound 已用 rne-read 手解机器码。

函数总计 14：业务 4 / 内联 0 / 派生胶水 10（serde deserialize/serialize/visit_*/fmt/clone/default）。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `DevSettings::live_link` | 0x14090df40 | 12 | 访问器 | O：`movzx eax,[rcx+7]; ret` —— 读 dev-settings **字节 @+0x07**（live_link 布尔标志） | expansion-b2\decomp\decomp-0x14090df40.txt（NotFound，手解） |
| `GameConfig::upgrade` | 0x14090df90 | 44 | 版本升级迁移 | O（round3 round2 复查修正常量地址）：`test byte[rcx+0x88],1; je +1; ret`（+0x88 位 0 已置即返回）；随后 `[rcx+0x84]` f32 比较常量 1.0@0x142925904（原行 0x14202018e 错——该址非浮点常量）、再 vs 0.125@0x142aab384；`cmp dword[rcx+0x80],0`、写 `[rcx+0x80]=0`、末写 `[rcx+0x88]=1` 且 `[rcx+0x8c]=旧值`——配置版本迁移：清零 0x80、标记已升级（字段语义 U） | objdump（dis.py 0x14090df90）；rne-read 0x142925904/0x142aab384 |
| `is_within_glade_shape` | 0x14090dfe0 | 80 | 形状包含测试 | O：读全局字节 `0x14362d6dd`（门控）；`mulps/movsd` 序列把向量分量与常量 0x1420319d8/0x1420319dc 比较——**glade 形状椭圆边界测试（x²/a²+y²/b²≤1），启用位 0x14362d6dd==0 时短路** | expansion-b2\decomp\decomp-0x14090dfe0.txt（NotFound，手解） |
| `developer_hotkeys_enabled` | 0x14090e070 | 112 | 开发者热键开关 | O：`movzx eax,[rip+0x02d26c61]` → 读全局字节 **0x14362d6dd**（与 is_within_glade_shape 同一全局）——**开发者模式字节，直接返回** | expansion-b2\decomp\decomp-0x14090e070.txt（NotFound，手解） |

## 内联/无独立符号函数

无。

## 派生胶水（10 项：deserialize、serialize、visit_str、visit_map、visit_enum、fmt、clone、default 等）

`GameConfig`/`DevSettings` 的 serde ron/json 双后端派生；`impl$0::next/previous`（0x1408b99e0/0x140914840）为枚举迭代器 Sequence 胶水（与 main_camera 同址 ICF）。不计独立业务。

## 关键结论

- `developer_hotkeys_enabled` 与 `is_within_glade_shape` 共用全局字节 **0x14362d6dd**（开发者模式开关，跨域全局族）。
- `GameConfig::upgrade` 为版本迁移：检测 +0x88 已升级标志、重置 0x80 字段、回填常量（具体版本号 U）。
- `DevSettings::live_link` = +0x07 字节访问器。
