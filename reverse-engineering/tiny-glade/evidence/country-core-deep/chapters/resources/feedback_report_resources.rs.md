# resources/feedback_report_resources.rs

> 文件作用：**反馈报告资源：截图 Arc 双槽+元数据区+状态机+选择字节。**

函数总计 8：业务 6 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::feedback_report_resources::FeedbackReportBgTexture::is_available | 0x140909860 | 25 | 0x140909860：[+0x36]<7（可用性枚举字节；非独立 bool 资源） | O | audio-misc\decomp\decomp-0x140909860.txt |
| country_core::resources::feedback_report_resources::FeedbackReportTags::to_list | 0x140909870 | 51 | 0x140909870（PDB 名错位）：实为羊叫 4 标志→{aww,think,angry,sob} 列表装配（模块归属存疑） | O（错位） | audio-misc\decomp\decomp-0x140909870.txt |
| country_core::resources::feedback_report_resources::FeedbackReportResources::reset_choices | 0x140909910 | 81 | 0x140909910：[+0x40..0x46]=**{0,0,0,0,1,1,0}**（dword+0x40=0、word+0x44=0x0101、byte+0x46=0；review-audit 纠错：两个"1"在 +0x44/+0x45） | O | audio-misc\decomp\decomp-0x140909910.txt |
| country_core::resources::feedback_report_resources::FeedbackReportResources::clear_screenshot | 0x140909930 | 85 | 0x140909930：状态[+0x3e] min(state-7,2) 分派：0(空)跳过/1 锁[+8]Mutex/2 换纹理 Arc；清空后 state=7 | O | audio-misc\decomp\decomp-0x140909930.txt |
| country_core::resources::feedback_report_resources::impl$3::build | 0x140909a70 | 94 | 0x140909a70：bevy 插件 build——TypeId 查表注册 FeedbackReportResources 默认值；缺失 panic | O | audio-misc\decomp\decomp-0x140909a70.txt |
| country_core::resources::feedback_report_resources::feedback_report_texture_receive | 0x140909ab0 | 100 | 0x140909ab0：(ResMut<FeedbackReportResources>,EventReader<…>)：有事件→换[+8]纹理 Arc→拷事件元数据入[+0x10..0x38]→state=7/8 机转；第二分支(state==8)锁 Mutex 更新纹理内部结构 | O | audio-misc\decomp\decomp-0x140909ab0.txt |

## 实现方式（算法级）（O，DIGEST §2.8 + ALGORITHMS §2.6）

**FeedbackReportResources 状态机**：
- `is_available @0x140909860`：`byte@+0x36 < 7`（背景纹理可用性枚举；重建独立 bool
  资源模型错）。（O）
- `clear_screenshot @0x140909930`：状态@+0x3e → `min(state−7, 2)` 分派：`0`（state=7 空）
  跳过；`1` 锁 `[+8]` Arc<Mutex> 清纹理；`2` 换纹理 Arc 路径；清空后 state=7。（O）
- `feedback_report_texture_receive @0x140909ab0`：`(ResMut<FeedbackReportResources>,
  EventReader<…>)`（tick 回写 O）；有事件 → 按状态字节换 `[+8]` 纹理 Arc（LOCK dec+drop）
  → 把事件 6 qword 元数据拷入 `[+0x18..0x36]`（宽/高/格式/句柄，逐字段 U）。（O）
- `reset_choices @0x140909910`：`[+0x40..0x46] = {0,0,0,0,1,1,0}`（dword+0x40=0、word+0x44=
  0x0101、byte+0x46=0；review-audit 纠错）。（O）
- `0x140909870`（PDB FeedbackReportTags::to_list 错位）：**4 标志字节 → 羊叫串列表**
  `{"aww"(0x142aaae08,3), "think"(0x142aaae0b,5), "angry"(0x142aaae10,5), "sob"(0x142aaae15,3)}`
  经 from_iter 闭包入 Vec<String>（模块归属存疑 §7）。（O）
- `impl$3::build @0x140909a70`：bevy 插件 build——TypeId
  `{0x8e1bd251962ba784, 0x6a782374c0e6529b}` 查组件表 + 注册默认资源；缺失 → panic
  （fmt 0x142af2cc0，0x27=39B）。（O）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
