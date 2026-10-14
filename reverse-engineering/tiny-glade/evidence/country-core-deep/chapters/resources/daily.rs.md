# resources/daily.rs

> 文件作用：**每日主题**——`DailyThemeInfo`：default 置两个 u64=0x8000000000000000 哨兵；
> `is_weekly` = SystemTime::duration_since 成功且 `elapsed > 0x6977f`（432511s ≈ **5.006 天**，
> 非 7 天）；`get_name` = 主题 locale 名选择（[rdx+4] 首字符匹配 en-US/de-DE/es-ES/fr-FR/
> it-IT/ja-JP 族常量，r8==5 门）。CurrentDaily/DailyThemeCredits 占位。

函数总计 10：业务 2 / 内联无独立符号 0 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `DailyThemeInfo::get_name` | 0x14083c240 | 31 | locale 判别主题名选择（jumptable on kind==5 × 首字符） | O：hex 手解（常量族 "n-U"/"e-D"/"s-E"/"r-F"/"t-I"/"a-J" = locale 码片段） | camera-time\decomp\decomp-0x14083c240.txt（hex 手解） |
| `DailyThemeInfo::is_weekly` | 0x14083c440 | 52 | 见文件作用（阈值 0x6977f 实证；v1 名 get_name::closure$0 即其内联闭包） | O：实证 | camera-time\decomp\decomp-0x14083c440.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`deserialize`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 8 项
