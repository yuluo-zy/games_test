# 完整位置检查源码候选 v2

本候选仅追加到原TG719合法work，原owner/claim不变。根agent独占生产整合、Cargo/真实GPU；本作者没有修改生产或执行测试。原完整8143B/1203B/200B逐指令与发行RNE复核通过，详见相邻location-validator-root-contract-v1/NATIVE_BYTE_PROOF_V1.json。

复用原location-result-20261020/LOCATION-CONTRACT-V1.md及finite-original-tables.json、probe-world-position-addendum-v1/ADDENDUM.md，不整文件照搬旧候选。补充核查：原wall分支f56c在subtype1门之前已required缓存，候选保留此顺序；地形/楼梯直接Valid不读取缓存或目录。原原生标签11有效，0..10失败payload全部保留，禁止旧tag1有效注释。

源码完成范围：实际ID/存储地址/同世界库与Query接入，完整wall与roof目的分派，原3次borrowed CurveSlice采样、cut与arch独立数组、sharpturn、真实五态Snap、4角环形U距离、原Roof section脊线、完整11reason。迁移同文件旧cull_one只传真实依赖和storage地址；其它归档/剔除仍partial且不挂宿主。

依赖：corelead唯一canonical四表+ID map候选；现完整Bounds1561B/profile418/393B/Snap3277B、已恢复CurveSlice921B/WallSpace793B/Rectangle2d核心方法。新发现Circle.closest_point84B当前有normalize_or回退，与原不符；单独必要canonical leaf追加中，不私造几何算法。此叶独立恢复后才能声称该必要依赖闭合。

10项测试已写，覆盖全部result载荷/terrain-stair早返/roof缺失/cache required/双端点/cut与lower/sharpturn/实际corner先后/flat和gable半开带/非有限传播；仅rustfmt解析成功，未运行Cargo。测试是显式输入fixture，非原程序执行、真实加载目录或完整App编辑恢复。默认宿主validation仍未挂载，完整539+8161callback+473 signifier反馈消费者正在恢复。

## v3 独审修复

v2被冻结后独审发现两处作者引入错误：rustfmt stdout带文件标题被误复制进Rust、归档walls局部Vec覆盖了PublicWalls资源。v3只移除该标题并将局部桶变量改为archived_wall_buckets；业务算法/测试不变。v2的已parse宣称只适用于复制前输入，不适用于最终v2文件，不能作为v2有效验证。v3重新对自身文件执行只解析；不复制stdout结果。
