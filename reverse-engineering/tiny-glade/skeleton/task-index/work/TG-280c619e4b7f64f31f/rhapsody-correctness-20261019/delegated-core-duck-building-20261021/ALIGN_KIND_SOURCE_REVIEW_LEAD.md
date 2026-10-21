# AlignKind20字节必要类型纠正源码独审

core_business_lead，2026-10-21：独立核原两完整函数片段后通过最窄源码。原preferences14080c0b9..107的24 stride中+14是score，不是kind；原best-neighbor14080c4de/4e4读取20字节，再527 ID@0、52f 16B@4、537末4B@14，24 stride确为ID4+kind20。旧尾pad_14是误算外tuple，不应因既有测试硬断言24而保错。

此次只删假pad/NATIVE_SIZE20、对齐4断言、同源两初始化和错误文档；两个position_offset/outgoing_dir数学字段行为不改。无foreign pad_14字段引用。源码after56b005ab5127879798db0145e8330104559bc181953968272df4069dcb076e29，完整before/diff在同目录；worker未Cargo、未改生产。完整楼梯request/图/曲线/segment资源仍未因此恢复，不把本类型纠正计作producer闭包完成。
