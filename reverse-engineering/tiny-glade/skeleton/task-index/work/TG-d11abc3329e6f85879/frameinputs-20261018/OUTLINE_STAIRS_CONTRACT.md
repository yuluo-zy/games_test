# B OutlineStairsCmd真实结构与消费

此文件族的canonical命令已合生产；OutlineWallCmd独立gap没有冒充完成。原typed命令不是16B enum：PDB只给名义接口，实际字段来自独立writer、UIproducer和structure_outline/prepare两个consumer。

- EventWriter::write@142291100完整98B：EventInstance stride24，id0..8，memcpy payload16。
- ui_color_windows_and_flags@1422b0e00 的1422b1533调用原StairSegment::new（与GraphEdge::new共享140901260，unsigned min/max），1538/153e写两端点，1544写strength2.0，154e写kind0/connected1，1568提交同一canonical事件。
- structure_outline@140959fc0首个reader为OutlineStairsCmd；14095a387起读取record+10 f32，仅>0时发几何；record+14的kind交closure1，record+15决定单segment或StairsState::get_all_connected_nodes。
- prepare_frame_constants@140af40b0仅筛kind3，直接将端点当GraphEdge查TransientStairEdgeIdAllocator，设置原RGB(.1,.03,0)，首个allocated id即break，保留剩余未读事件；不添加strength检查。

精确payload：StairSegment8B + strength f32@8 + kind u8@12 + include_connected bool@13 + padding2B。类型名StairSegment、DecoratorId来自原PDB；成员名strength/kind/include_connected为本次依据消费语义的投影命名，不冒充作者字段名。生产Event身份仍为systems::wall::structure_outline::OutlineStairsCmd，不新建另一个事件来绕过producer。

当前system_color::ui_color_windows_and_flags完整拾取/UiState producer仍显式未实现（生产源中已有gap），本次提供其已证字段工厂connected_color_hover仅用于后续真实consumer接线，不在运行期制造hover事件。typed命令恢复不能计作该UI域整体完成。

完整辅助报告：structure-outline-direct.json、event-writer-direct.json、color-stairs-producer-direct.json。原B raw consumer snippets/identity在TG-379258a80656c7937d/main-chain-frame-20261018/os-evidence/consumer-index-evidence.json；主合同包含TAA/TOD/frame全部闭包。没有执行原侧行为差分；root统一Cargo验证。
