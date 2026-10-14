# resources/feedback_report_resources.rs

> 文件作用：**反馈报告资源：截图 Arc 双槽+元数据区+状态机+选择字节。**

函数总计 8：业务 6 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::feedback_report_resources::FeedbackReportBgTexture::is_available | 0x140909860 | 25 | 0x140909860：[+0x36]<7（可用性枚举字节；非独立 bool 资源） | O | audio-misc\decomp\decomp-0x140909860.txt |
| country_core::resources::feedback_report_resources::FeedbackReportTags::to_list | 0x140909870 | 51 | 0x140909870（PDB 名错位）：实为羊叫 4 标志→{aww,think,angry,sob} 列表装配（模块归属存疑） | O（错位） | audio-misc\decomp\decomp-0x140909870.txt |
| country_core::resources::feedback_report_resources::FeedbackReportResources::reset_choices | 0x140909910 | 81 | 0x140909910：[+0x40..0x46]={0,1,1,0,0,0} | O | audio-misc\decomp\decomp-0x140909910.txt |
| country_core::resources::feedback_report_resources::FeedbackReportResources::clear_screenshot | 0x140909930 | 85 | 0x140909930：状态[+0x3e] min(state-7,2) 分派：0(空)跳过/1 锁[+8]Mutex/2 换纹理 Arc；清空后 state=7 | O | audio-misc\decomp\decomp-0x140909930.txt |
| country_core::resources::feedback_report_resources::impl$3::build | 0x140909a70 | 94 | 0x140909a70：bevy 插件 build——TypeId 查表注册 FeedbackReportResources 默认值；缺失 panic | O | audio-misc\decomp\decomp-0x140909a70.txt |
| country_core::resources::feedback_report_resources::feedback_report_texture_receive | 0x140909ab0 | 100 | 0x140909ab0：(ResMut<FeedbackReportResources>,EventReader<…>)：有事件→换[+8]纹理 Arc→拷事件元数据入[+0x10..0x38]→state=7/8 机转；第二分支(state==8)锁 Mutex 更新纹理内部结构 | O | audio-misc\decomp\decomp-0x140909ab0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
