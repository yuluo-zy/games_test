# system-roof

屋顶 CPU 几何、瓦片生成与编辑/重放算法。按原 B 系统目录归类，供 country-core 建筑业务和后续游戏调度调用。

## 功能模块

`surface` / `ridge` 处理表面与屋脊；`tiles`、`rectangular_*` 和 `pipeline` 生成矩形/圆形瓦片；`edit_rules` 处理已解析编辑；`parameters` 恢复参数布局和檐口计算；`replay` 处理已观察历史事件和视觉插值。

## 来源

原文件：`reverse-engineering/tiny-glade/reconstruction/roof/{surface,ridge,tiles,rectangular_caps,rectangular_edges,rectangular_faces,pipeline,edit_rules,parameters,replay}.rs`。过程包含 Git `156add05`、`c3625e1d`、`531b7d32`、`0f9ded43`；逐文件映射见 [sources.json](../../../provenance/sources.json) 与 [building-audit.json](../../../provenance/building-audit.json)。证据位于原 `SOURCE_RECOVERY.md`、`evidence/multi-version/B-parameter-findings.md`、建筑历史报告，以及 tracking/project.json 的屋顶 local_algorithms。

## 已完成

已解析形状上的表面/屋脊采样、瓦片序列与随机消费顺序、矩形边/面/脊补片组合、Roof 参数和 eave 规则、已观察 VisualEdit 与离散事件生产。旧报告覆盖特定输入域，原浮点差异记录继续保留。

## 还差什么

原始墙/屋顶完整资产与 ECS 接线、全部 freeform/结构生产者、全历史游标、GPU backend、引擎 fork 调度和场景画面对照。建筑生命周期位于 country-core/resources/roofs 与 systems/wall/construction，开发 Bevy 适配位于 tg-building-runtime。

## 验证与开发

`cargo test -p system-roof`；当前结果见 [validation.json](../../../provenance/validation.json)。迁移仅改模块引用并复用原依赖；特定架构/输入域的浮点验收不自动覆盖本机全部输入。
