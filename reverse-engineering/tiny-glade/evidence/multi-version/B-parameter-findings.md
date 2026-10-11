# B 参数与屋顶历史源码恢复增量

身份：[baselines.json](../../tracking/baselines.json)。原指令 oracle 使用 B 的 tiny-glade.rne（SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03）。已有 Ghidra 数据库来自登记的 patched EXE，只读、不重新分析；地址和指令差分以 RNE 为准。

## 字段与消费者

RoofShapeParams 的 28 字节观察块对应 Roof88 的 0x24..0x40：tip x/y 在 +0/+4，profile +8，height +12，ridge +16，eave +20，ridge direction +24。原 serde 消费者确认 Width=0、Length=1；+25 是 rim 字段，+26/+27 为保留字节。历史 JSON 缺失 rim 的默认值尚未确认，适配器保留原字节，不补造默认值。

原 eave_length_ws（0x1408e3c30）保留 SSE 运算次序；MOVSS 寄存器形式保留的上层 lane 含 sqrt(profile)，最初将其视为零的候选被原指令差分否定并纠正。1024 个参数映射案例、2097 个有限 eave 案例通过：[参数报告](B-parameters-verification.json)。非有限 eave 输入未验收。

## 历史与回放

`tg-history` 复用 B 原锁文件 serde 1.0.200、serde_json 1.0.85。保留原 JSON 值、未知命令、扩展字段、u64 ID、cursor 后的尾部；typed f32 视图不覆盖原值。A/B/C 共 98 个真实历史文件、733 条屋顶命令解码核查通过：[格式报告](history-rust-verification.json)。这是格式与字段核查，不是 A/C 的二进制行为验收。

B RoofEdit::serialize（0x140259e30）与 RoofReplayCmd::process（0x1421b48e0）共同确认 VisualEdit tag=2，payload 不含外层历史 entry 的 16 字节前缀。字段位置见 [visual-field-map.json](B-roof-replay-schema/visual-field-map.json)。VisualEdit 的模式低 bit 控制计时，duration=0/elapsed=0 保留原 x86 indefinite NaN，不将无序比较误写为立即完成。1600 个 replay state24/event53 案例通过：[VisualEdit 报告](B-replay-verification.json)。

同一 process 的 tag 0/1/3/4 已恢复为观察字节接口：Create/Delete 按撤销标志互换并立即完成；Direction/FlatStyle 只在 elapsed==0 时发事件，只有 mode==1 才等待 duration。创建事件携带原 presence 的 44 字节，删除事件含标志1；方向选择 from/to；样式事件复制同一选择值到两个字节并写标志0。这里保留完整观察结果，尚不为未确认字段命名或补默认值。7168 个案例通过：[离散回放报告](B-discrete-replay-verification.json)，原执行分别检查五个 writer 的消息计数、sequence、change tick。

B 的 118 条真实 VisualEdit 在进度0、0.25、1及正向/撤销下，共708例 JSON→回放事件→已解析 Roof 状态与原指令一致：[联接报告](B-history-chain-verification.json)。只验证 Roof88、变化字段、tick、elapsed/finished、反馈条数；该联接报告不额外验收反馈数值。

## 当前 Mac 构建

编辑核心的旧 Windows 报告保留；当前 ARM 构建最初有43个无穷运算生成 NaN 符号位差异。现已在游戏算术边界保留 x86 indefinite NaN，glam 长度限制仍调用原版本库 API。修改后4514个原指令案例全部通过，包括编辑状态、通知数值与边界：[当前编辑报告](B-edit-rules-host-verification.json)。这不等于所有 NaN payload、SSE flags 或全部屋顶算法的跨主机验收。

工程当前15个 registry package 的 version/source/checksum 与 B 原锁文件一致：[依赖报告](workspace-dependency-verification.json)。features 和原引擎分叉尚未全部核实。扩展 workspace 以前的瓦片/曲面全链报告只作为历史构建证据；原 Cargo 文件快照有哈希校验。

## 未关闭的边界

ECS 实体查找、create/delete/style/direction 消费者、事件调度、事务提交/游标/撤销顺序、快照装载和迁移、Flat 几何、墙体权威形状及 A/C 的内存布局仍待恢复。B 有22个缺失快照引用，另两个版本只提供2个同ID候选，未验证可替换。私有 rhapsody2/shabriri 的匿名访问失败，源码可用性仍未确认。没有完整场景、渲染或游戏交互验收。
