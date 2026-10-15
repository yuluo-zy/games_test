# resources/daily.rs

> 文件作用：**每日主题**——`DailyThemeInfo`：default 置两个 u64=0x8000000000000000 哨兵；
> `is_weekly` = SystemTime::duration_since 成功且 `elapsed > 0x6977f`（432511s ≈ **5.006 天**，
> 非 7 天）；`get_name` = 主题 locale 名选择（kind==5 门 × 5 字节前缀匹配 14 个
> locale 族常量，index→名字解析；悬空引用交叉核对修正：完整 locale 集含 en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP/ko-KR/pl-PL/pt-BR/ru-RU/sv-SE/tr-TR/uk-UA/zh-CN）。CurrentDaily/DailyThemeCredits 占位。

函数总计 10：业务 2 / 内联无独立符号 0 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `DailyThemeInfo::get_name` | 0x14083c240 | 31 | locale 判别主题名选择（jumptable on kind==5 × 5 字节前缀） | O：hex 手解（kind==5 门 + 14 个 locale 前缀匹配，index→名字解析） | hex 手解（悬空引用交叉核对修正：完整 14 个 locale en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP/ko-KR/pl-PL/pt-BR/ru-RU/sv-SE/tr-TR/uk-UA/zh-CN，5 字节前缀匹配，见 round3/DANGLING-FILL-REPORT.md） |
| `DailyThemeInfo::is_weekly` | 0x14083c440 | 52 | 见文件作用（阈值 0x6977f 实证；v1 名 get_name::closure$0 即其内联闭包） | O：实证 | camera-time\decomp\decomp-0x14083c440.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`deserialize`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 8 项

## 实现方式（算法级）

`DailyThemeInfo`：default 置两个 u64=0x8000000000000000 哨兵；`is_weekly`（0x14083c440）= `SystemTime::duration_since` 成功且 `elapsed > 0x6977f`（432511s ≈ **5.006 天**，非 7 天——合同/骨架 7*86400 不符）；`get_name`（0x14083c240）= kind==5 判别 + 5 字节前缀匹配 locale 族（en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP/ko-KR/pl-PL/pt-BR/ru-RU/sv-SE/tr-TR/uk-UA/zh-CN，index→名字解析，缺失回落 index 0）。CurrentDaily/DailyThemeCredits 占位。
