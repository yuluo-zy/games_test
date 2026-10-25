# closure1 与短请求可执行候选

两个完整 helper 已依据 parent 独审合同实现；同 stairs.rs 既有用户架构，无新生产文件、无生产/Cargo 修改。aligned_stair_control_point 保 required 点→两次同 directed edge/widthoffset，且不读取 target point。make_short_stair_request 保原孤点/failed-edge 共用短出射业务、未归一 Quat/负零、CurveTry 唯一 None、两 f32 顺序、两返回 SubSegment、VisualTerrain 中心严格比较、Info/derive 失败先后、原 sorted dirtykey。

8 个真实同 World / canonical owners测试已落：directional controlpoint、required先后、failed edge 第二锚点但首点几何、真实非单位 Quat、NaN terrain mask、墙normal无需cached读取、CurveTry先于late lookup、首derive先于第二Info。未执行 Cargo/rustc；root为唯一验证执行者。此包不将完整两30KB系统/host视作完成，不在 helper 替代上游 Spline。

candidate.patch 对冻结生产87435；after-preferences.patch 对前候选a1bf0488，root按实际已消费版本选择。完整源/patch/contract/依赖指纹见 freeze.json。
