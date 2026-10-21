# 渲染/平台模块只读进度审计（2026-10-21，main f72938c81）

本轮仅阅读当前源码、git log 与既有证据；未运行 Cargo、测试或 GPU。百分比为按主要能力及调用链覆盖给出的保守工程估计，非原函数数量统计；源码完成度与生产接线/验收成熟度分别列出，不能平均为全游戏完成率。rhapsody 依照当前重构架构估计，不按旧库存缺失惩罚或恢复旧实现。

| 模块 | 源码实现估计 | 生产接线/验收估计 | 已有能力 | 主要缺口 |
|---|---:|---:|---|---|
| rhapsody 渲染图/设备/同步/资源提交 | 85–95% | 75–85% | 类型化范围图、事务提交、alias、temporal、真实完成退役、分页bindless及M3适配已接；真实生产GPU单帧读回 | 完整游戏长稳尚未通过；最新headless完成维护补丁尚无新30帧成功记录；完整目标硬件/异常/长跑矩阵 |
| core world frame（W1–W54→宿主→GPU） | 85–90% | 75–85% | 生产builder、全世界单帧、4×MSAA resolve、真实资产与截图owner；实际M3 30帧曾全执行 | 全部场景producer、场景完整性、稳定帧断言和原作同机位对照未完成 |
| compile_main 尾部/截图/UI合成（排除T13） | 85–95% | 75–85% | T11 session preview、T12 LUT gallery、T16 canonical ECS UI、T21主帧egui均已接；截图键/PhotoModeCmd/Steam/Meta/存档缩略图producer已落源 | custom_lut仍None（ICC producer未接）；完整菜单/照片/截图/存档联合运行验收不足；部分旧头注已过期 |
| Nani/GPU世界实例上传 | 75–85% | 55–70% | maintain→ApplyDeferred→已映射tuple→flush接set44；九类映射、cloth、限定roof-bearing floor、canonical窗灯glass、Uber/Velocity已有consumer | 原20种更新tuple并未全装；floor-only和若干Wren/Plaster/Smoke等producer契约仍缺；decorator完整visual宿主未挂，不能以更新函数存在算场景可达 |
| shader-pipeline/编译工具与运行时 | 80–90% | 75–85% | Speedy资产协议、key/path、预处理→DXC→反射→打包、运行时解码、bindless ABI转换；旧全404包638stage构建成功 | 原宏producer 132 zero-key tuple、7 schema残差、离线工具/fork身份/完整资产状态机及完整原画面一致性未闭 |
| 窗口/主循环/呈现平台链 | 80–90% | 65–80% | OS→loading/gameplay→App更新→draw→图→present生产路径；真实M3 loading3次native present成功，Linux曾有gameplay提交/排队present | 新严格完整GameLoop WSI验收入口尚待执行；Windows原生、全屏/显示器、生命周期/第二session与完整游戏长稳未验收 |

信心：存在/接线/未验证边界为高；百分比为中低（没有统一完整业务分母）。不能把任一范围上限当已验收精确值。

## 最新提交与旧状态更正

- f72938c81 已在 raster_host.rs:1755–1790 按 MESA_LLVMPIPE + DrawIndirectCount 门将普通 indirect draw 转为逐命令 count-form（1867/1893）。源码注释记录Mesa25.2.8第2帧草叶消失复现、命令有效/间接数据GPU核对。此为新修复落源；仓库未见它之后绑定新HEAD的全30帧稳定成功结果，不能将08ece之前失败直接归为该补丁失败，也不能将commit标题视为全部验收。
- 6e5c0ca75 是墙 set19 chain2 sample→calculate→init 原版顺序复核及拓扑序测试/未接表，不把它外推完整墙渲染/编辑完成。
- 旧“缺T11/T12/T16/T21”过期，当前 world_postprocess.rs:1203–1232、production_renderer.rs:391–412 已有真实消费。T13明确不推进，从分母排除。
- 旧渲染尾报告称截图purpose0/3/4、Meta、存档预览无producer也已过期：screenshot_capture.rs:333–450当前gather已接这些实际资源/事件与16帧照片保持状态机。
- CURRENT-INTEGRATION早段“validate_blueprints仍缺”被该文件末段92行后完整验证器与11类原因交付覆盖；但完整cache/楼梯网格与visual宿主仍未完成。
- 旧rhapsody文档中的duck编译错误被10月21日默认/native-audio check成功覆盖，不再作为当前构建阻点。

## 既有证据及其范围

1. reports/main-loop-render-recovery-2026-10-19/latest-audit-fa53d49a-20261020/CURRENT-INTEGRATION.md:31–47：真实M3 GPU页原语6/6、ABI4/4、VAO1/1、compute3/3、原资源单帧1/1、共享常量7/7，loading WSI3 native成功；真实GameLoop gameplay窗口尚未通过。
2. 同目录WORLD-HIGHWATER-30-RESULT-20261021.json：30帧全部渲染，54.64秒，live3042..6022失败，原阈值16不变；后续memory/pixels未到。是旧二进制epoch，不是f729运行。
3. production_renderer.rs:168现在包含compile成功后collect_completed_frames补丁；CURRENT-INTEGRATION.md:114明确新headless V2/真实GameLoop WSI执行置于推送之后，目前未见通过收据。
4. 同目录WORLD-PREFIX-RESULT-20261021.json：单帧范围准备优化，同physical cells635、patches3536、uses4438、private reads1478，PPM逐字节一致；状态准备约91.8倍、rg_submit约48.8倍只适用于该测量段，不是游戏FPS。
5. rhapsody/REFACTOR_PROGRESS.md:5–40：10月19日all-features731、no-default-lib375等旧领域验证；:57–70仍无M0–M9全门槛PASS。保留其日期与源码边界，结合21日生产GPU新证据，不能全当当前HEAD复验。
6. shader-pipeline/README.md与shader-compiler/README.md明确404包/638stage、397/404反射一致及残差；这不是638个shader算法均已原画面验证。

## 可直接引用的源码证据

- tiny-glade/crates/tiny-glade/src/world_postprocess.rs:1145 ICC缺口；:1203 T11；:1205 T12；:1207 T13排除；:1231 T16。
- tiny-glade/crates/tiny-glade/src/production_renderer.rs:391 T21；:168完成维护新补丁。
- tiny-glade/crates/tiny-glade/src/screenshot_capture.rs:333 gather；:404 T13不生产。
- tiny-glade/crates/country-core/src/render/nani/systems.rs:969–995真实已映射set44 tuple与显式floor-only缺口。
- tiny-glade/crates/tiny-glade/src/raster_host.rs:1755–1790最新Mesa工程适配；:1867/:1893实际录制分支。
- tiny-glade/crates/rhapsody/crates/shader-pipeline/README.md:9–11；shader-compiler/README.md:9–11历史实编译与未闭项。

最大缺口不再是“没有GPU帧”：是完整场景producer→正确几何/实例→连续帧稳定→真实gameplay窗口→交互编辑/保存/重启这一整体验收。渲染路径独立估计接线/验收约75–85%，不外推全游戏。
