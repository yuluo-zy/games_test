# 原墙装饰分段迭代器恢复候选

root 明确委派 TG-652d8e2c9cc79a01e3 必要增量；live show 已核 owner=scene-clutter-producer-20261020。保留原 claim，在本唯一目录交付。未写生产，未 Cargo/GPU/Git。

原 `subdivide_wall_decorator` 的坐标复制实现没有真实调用；原闭包签名亦未获证明。本候选替换这一尾部，恢复真实 Range→墙坐标→现有真实 CurveSampling→相邻点→Transform 的迭代器，不自建曲线或点数组。复用现有 `utils::xz_rotation_bad`，不复制旋转/曲线/闭环算法。

工程接口接收原派生的 WallSpace、实际 WallSpaceCoord、真实碰撞宽度、pieces 与借用 CurveSampling。native pieces 来自完整窗 producer 的 rank/分支；caller 尚待恢复，不能记作场景已可达。宽度与 pieces 的数值、0乘法、XY再Z的长度和、sum后乘半、unit scale 均保留原证求值顺序。零数量、非有限宽度不新增兜底，真实 CurveU 失败继续传播。

新增5项待执行 CPU 测试：真实折线逐段pose、开墙两端含原负U方向、闭环缝、退化宽度与共享旋转策略、非有限值真实曲线失败。仅 rustfmt emit stdout 解析成功；未类型检查、未执行测试，不冒称行为验证。

本包可独立整合 helper 与测试，当前没有旧API业务caller；后续原窗 producer 将收集该迭代器。完整窗生产者仍依赖 canonical collision四表与Info window_extra；这些由 core_business_lead 协调，不能用 bbox/默认几何替代。
