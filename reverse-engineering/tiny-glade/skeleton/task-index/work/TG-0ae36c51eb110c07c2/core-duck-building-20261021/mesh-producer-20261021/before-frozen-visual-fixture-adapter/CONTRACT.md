# 可视状态接口变更的测试夹具适配

仅改已有 interpolate 同 World 测试的两处 setup。原真实消费逻辑仅检查 canonical StairsVisualState 中边是否存在、force flag；该夹具的条目没有实体或三类 SSBO 分配。原有 reset_edge(edge) 是错误的无资源兼容接口，canonical 真实 reset 现在必须 Commands 与 Brick/Sparkle/Wooden 三个 buffer；不在本测试伪造这些资源。

使用公开 edges_mut().entry(edge).or_default() 建立实际 canonical 空条目，missing gate 测试通过同一公开 map 移除该无资源条目。保留全部原业务断言、上游真实 request producer、历史数据、force/global policy 门；不把空条目宣称实际网格重建或资源卸载验证。真实 reset/remove 析构测试属于 visual 源，需实际 Commands/三 buffer，由 Tree 作者提供。

依据 parent 明确委派，TG-0ae36 claim live owner core-duck-building-20261021 未改变。配对 canonical visual 候选 after262f5440；仅 test 模块改动，不改生产函数。父级独立审核后由 root 统一实际 Cargo。原两 62KB main 尚未恢复完整，不在本适配中假接线。
