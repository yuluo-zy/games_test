# 三源 canonical 四表源码独立审查

审查人 core_business_lead，2026-10-21。结论：本三源有界 schema/value 实现通过，供 root 与真实 loader26、真实 subtype producer、已委派消费者成组整合；不是可独立应用的全项目编译闭包，状态 implemented_unverified。

已读取最终三份完整 diff、逐 API 合同、原 four insert/empty/required getters 合同、field-order erratum，重新实算 paired-candidates.json 的3候选与3patch哈希一致。没有 Cargo/rustc/GPU 或原程序执行。

- DecoratorCollision 拥有 Vec<Triangle> + 同 Arc<RaycastMeshData> + Rectangle3d，删除原bbox伪造mesh返回。Clone深复制三角缓冲、共享真实Arc；Debug只展示真实hash/bounds，不捏造物理形状相等。
- same insert 与 four insert 保留原clone、逐表替换释放、最后销毁首collision副本的次序。四表类型正确，显式outline None与缺键区分。required borrowed lookup避免每帧Vec复制，不加第二registry。
- empty仅native-physics域调用真实零尺寸/零平移new_cuboid_at，真实hash由该构造器计算，单位Rectangle3d符合原583B。非native域没有shape None的假成功路径。
- DerivedDecoratorSubtypes唯一map恢复为ID→DecoratorSubtype，View从同owner借用，不再构造Arc<LegacyDerivedDecoratorInfoRecord>。Legacy仅显式残留元数据类型，未存入canonical四表或ID表。
- 原dims/get_collision_bbx_ls完整inline签名尚无证，因此本候选移除错误API，不以命名猜测其含义；窗消费可直接使用已证bounds.footprint.size.x与top-bottom。foreign旧调用必须以真实原调用合同迁移，不能仅为编译把旧bbox换成默认几何。
- 12项源码测试（6普通typed/6native）针对真实owner、exact rank缺失、显式None、Vec/Arc所有权、同key释放、真实零cuboid；未执行。新增原映射decorator_outline.rs只提供已证枚举载荷，不用空display伪造业务已完成。

三源before/after/patch清单以 paired-candidates.json 为准：TG609 after aa33480b6c7e0158c604419619a212a57632499b293b6d11ce87163f9c44ade4；TGda90 new ec8f8262dab54a7c244873b26a308e3626800e8117e9c3826bc2f6a84571d9f4；TG450 after592b4dd6d5f8ede16806450e6b5ed92fd2fa8026705717859c7bdea10d9ea4fc。

验收前置：真实52/34keys loader26、单一ID producer、所有foreign真实caller成组；root控制生产Cargo/lib/PreStartup/验证，保留失败日志。此审查不把151引用盘点计作迁移完成，不外推A/C版本。
