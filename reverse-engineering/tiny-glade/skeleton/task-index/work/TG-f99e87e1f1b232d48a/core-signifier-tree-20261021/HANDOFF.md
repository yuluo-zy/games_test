# 树光标与共享指示符构建器配对交付

先应用已经独审的树producer/consumer第一阶段；本组4文件配对应用，不能单独换builder而留下raw调用。精确路径、before/afterSHA、patch及依赖见paired-handoff.json和各task的hashes.json。

原证：11个builder完整procedure与两个真实caller闭包分别保留native/report.json，均ok。树closure584B在原ui task native/report.json。原build3840B与先前原指令probe的哈希一致，未把probe里的隔离库边界当成整机/GPU验证。

新增生产行为：原五种网格来源全部经现CPU Mesh和AssetLibrary实现；命名网格保库所有权，动态四支绑定真实TransientMesh；完整drawable/transform/animation/pass/lifespan/parent，严格mesh→reserve→shader失败顺序；水/边界仅typed接口接线。树光标使用owned Arc工厂，原尺寸与图像、原Empty动态网格生命周期，移除泄漏32B原始载荷和空seed_cursor_icon。

测试候选：core新增5项覆盖五支真实geometry/句柄/回收、Named所有权、默认动画、pass可选组件、回调一次释放、失败发生在reserve前/后；border原测试补真实mesh/shader库；树新增2项覆盖原尺寸NaN/Inf与完整工厂去重/鼠标变化回收/Arc释放。沿用全部既有树producer/consumer/replay测试。

只运行了候选rustfmt语法解析，没有Cargo/rustc或GPU、没有生产/Git编辑。root需要配对编译测试与真实GPU验收。Empty网格原本没有顶点属性/索引；宿主应沿原逻辑跳过零索引draw，而不是添加假几何。DEV demo原全局仍保现适配边界，本组不宣称所有工具demo门或所有UI宿主已完成。
