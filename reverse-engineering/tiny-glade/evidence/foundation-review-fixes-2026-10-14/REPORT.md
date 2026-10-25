# utils / asset / asset-pipe 修复与完成度复核

后续数值修复见 [numeric-followup/REPORT.md](numeric-followup/REPORT.md)：当前 utils 93 项通过，但跨平台超越函数校验合同已调整；下文 87/3 与 manifest 为上轮历史快照。

2026-10-14。本轮修复前次 review 确认的问题，复用已有快照，不新增原程序查询。
当前完整验收未通过，仍为候选/限定验证。代码存在或 partial 状态不等于主线业务闭包完成。

## 实现变化

- **数组缓存**：所有叶源文件记录路径、mtime、文件长度、XXH3 内容指纹，替代仅依赖目录 mtime 的错误判断。红→蓝编辑同时保留文件时间和大小仍触发重建；未改变的其他层保持一致。复用锁内已有 xxhash-rust 0.8.10，不手写算法。
- **缓存迁移**：新增 `TextureAssetMeta.sources` 使用 serde default，旧 metadata 无指纹时重建一次。metadata RON 有变化，纹理 blob/version/wire layout 未改。
- **参数边界**：`mip_count` 返回 Result；空图、零 resize、Limited(0)、u32 块对齐溢出在像素转换/缩放/编码前返回错误。压缩的 sRGB 灰度组合返回错误，批量任务仍等待其他任务结束，失败不写 metadata/list。
- **独立法线选择**：closest_edge_normal 按尺寸归一化的局部分量选轴，平局选择 Y、零选择负方向；get_closest_edge_id 保持逐线段距离选边。非正方形、旋转、中心和平局用独立反例覆盖。
- **距离后续差异**：法线通过后，组合快照暴露了此前被首个失败遮住的距离问题。已有 80 个快照以及原保存函数记录一致表明 directed local-edge 查询没有 abs；删除额外 abs。此函数不是通用对称 box SDF，负坐标行为明确写入注释和回归。

缓存指纹和参数错误是新的产品策略，不能沿用旧 B 原行为等价报告。公开接口变更：mip_count 返回 Result；cache_is_current 接收源指纹切片；TextureAssetMeta 新增 sources 字段。现有调用方已迁移。

## 验证结果

| 检查 | 结果 |
|---|---|
| utils 全部 lib 测试 | 87 通过 / 3 失败 / 0 忽略（退出 101） |
| 矩形组合快照 | 80 个既有样例通过，沿用原 2e-5 相对/绝对容差 |
| 新增矩形回归 | 3 通过 |
| asset all-targets | 15 通过，包含 388 原纹理 archive 字节往返 |
| asset-pipe all-targets | 15 通过（14 pipeline + 1 allocation） |
| 三库严格 all-targets Clippy | 通过 |
| 修改过的 Rust 文件 rustfmt / diff whitespace | 通过 |

原始输出见 `utils-tests.log`、`asset-tests.log`；完整源码/测试输入/依赖锁 SHA 见 `validation.json`。未修改已有原 fixture、比较容差或测试忽略策略。

### 未解决的精确数值差异

- circle_boundary：1050289184 vs 1050289188，4 ULP。
- curve2_rules：越界切线 0x7fc00000 vs 0xffc00000，NaN 符号差。
- easing：3148161199 vs 3148161200，1 ULP。

以上仍为失败。libm/编译平台可能相关，但没有同平台和中间值闭包，不能宣称已证明由平台造成或不可移植修复。逐位验收与产品数值合同的取舍仍需明确；这里没有放宽测试来消除失败。

## 完成度和证据边界纠正

- 185 项库存仍是 140 未业务分析、42 伪码未解释、3 编译体未定位，分母不完整；不是所有缺代码函数，也不是只缺 185 个差分测试。
- utils:34/35/40 等 scope_only、视图轴及样条候选规则保留各自缺口。真实游戏消费者与 C 不在本次验收范围。
- WallSpaceCoord::{bound,is_bounded,to_affine3} 改为 compiled_body_not_located：当前已检查记录未定位编译体/内联宿主，不等于证明未编译、没有消费者或永远不可恢复。
- 本轮未把任何功能的完整接受标志提升为 true。旧 utils-gaps 报告保留历史 SHA，其测试数不能将新测试再次累加。

## 保留的范围限制

缓存产物仍仅按 header 判定，不新增 archive payload 完整性验证；嵌套数组/DDS 数组、DDS 多层数据语义及完整真实 producer→consumer 尚未验收。源指纹需要读取源文件内容，作为防止旧缓存误命中的成本；整图零复制压缩回归仍通过。macOS 链接 Intel 依赖的已有 kernel_ispc.o 平台标记警告保留，Clippy 通过不表示所有链接诊断不存在。
