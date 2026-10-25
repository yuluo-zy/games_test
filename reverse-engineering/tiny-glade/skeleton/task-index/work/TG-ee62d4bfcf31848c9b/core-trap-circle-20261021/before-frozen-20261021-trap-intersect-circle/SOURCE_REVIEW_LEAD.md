# 独立源码复核（core business lead）

结论：通过本包最窄入口/叶范围；只做源码审查与隔离 patch 精确重放，未执行 Cargo、运行游戏或 GPU。

完整207B原函数已与独立合同核对：两次有序严格大于提前 None；其余路径照原标量乘加顺序，不增加零距离、有限值、radicand clamp。返回点顺序与 trap 真正调用者的 None 允许、Some 拒绝均已核对。同圆/NaN 是 Some(NaN)，不能写成 None。三组测试覆盖独立3-4-5数值、内外切与原 strict 边界、unordered/zero tag、实际 trap probe 常量与 caller 极性。未改 Rectangle 方法，未假设 circle 正半径约束。

源码：`tiny-glade/crates/utils/src/geometry/circle.rs`。

原始 before：`6f22365a05ad8acb2e81a85f65cd6f964840b38d81e6fadec79303bad067d33a`；冻结 candidate：`2a14ff8248f7ef0133dcf60192336b4cafecfdb4df83a18aa180da2f6067e20d`。

从原始 before 使用 combined/original patch，`patch -p1 -F0 --batch` 在独立临时目录退出0且字节完全等于冻结候选。生产 source 哈希保持 before。完整复核记录见 `SOURCE_REPRODUCTION_LEAD.json`。
