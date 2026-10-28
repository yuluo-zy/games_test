# 与启动主链联动的资源构造待办

2026-10-18 主链 owner 提交的72项原序 producer 候选，尚未在 core 本轮接受或编译。继续复用 canonical 资源类型，不创建重复同名 SSBO。

原证入口：reverse-engineering/tiny-glade/skeleton/task-index/work/TG-81dc0cd75bc98816f4/startup-producer-20261018/，先核 PRODUCER-CONTRACT.md、integration-prerequisites.json、registration-order.json 与 extra-constructors 身份/原字节。

本轮后续须独立核实并增量修复：

- StairsCounter 的实际类型和原构造。
- DormerHole：48字节 ×11250；DormerCount：4字节 ×225。
- UI 两项原资源，先辨明当前 canonical owner。
- Bush CPU 原8字节 ×9601，总0x12c08；原 NAME 为 `Bush positions for CPU`。
- WaterMask 三项：u32 ×0x2000；原 NAME 没有 Ssbo 后缀。
- Grounded 对应原 NAME `Building Shapes`。

主链已自行窄合 DoF autofocus None 上传128个f32(30.0)、ShadowHistogram None 上传1024零字节、Pebble None不上传，以及 deferred_raster 附件纠正；core 不重复覆盖这些 render_loop 文件。完整72项仍为候选，不以已有几项修复推定全部资源正确。当前构造、init_none/显式数据区别、缓冲字节数/元素数、实际consumer接线均需逐项记录。

## 后续断点（同日约21:00）

- BushCPU与Water/Exclude/Garden：canonical源、原证独审与有界实际library/consumer回归已完成；不计GPU同步/完整后续消费者。
- Dormer48B×11250、DormerCountu32×225：已独审合入；draw别名指向同canonical类型，统一Cargo进行中。
- StairsCounteru32×1、Lantern本地None、Grounded Building Shapes名称/None：已独审合入，统一Cargo进行中。Counter映射比较returned Buffer.byte_size；通用ssbo文件有其他合法owner，root用本类型exact原工厂而未抢改generic。
- UI2、水/花园5、Bush/Flower None、Boat/Stairs等render_loop窄改归主链owner；root保留其现源，不再重复恢复。
- RockyTerrain由building-rocky-20261018原资源组独占，真实31MB资产+18原范围静态独审在收口；尚未生产整合。原startup_terrain确在 **Startup8tuple第4项**，不是PreStartup color init；等待正式freeze/owner patch。
- 同步发觉实际input set5和save set43缺口：见主链host-edit-save-audit；root未持该两组、未动生产，后续实际owner必须先合法claim，禁止装整个core测试helper来重复模式/历史。

当前源和等待门槛见主链COORDINATION.md。以上具体完成范围不能替代72项全部按原序创建和真实启动/提交/消费者验收。

## 21:20 更新

Dormer4项、counter/instance3项已随producers-r1通过；完整核心2401+实际编辑图1项/0漂移。Rocky canonical147输入已整合，5项专项rocky-r2全部通过/0漂移，实际31MB资产可解析/打包并由现library创建两个正确buffer；actualStartup/W4/W5同canonical消费者尚未闭合。主链72 producer现已落盘PreStartup，实际72资源/CustomCursor验收由对应owner进行，不能由core专项推定通过。
