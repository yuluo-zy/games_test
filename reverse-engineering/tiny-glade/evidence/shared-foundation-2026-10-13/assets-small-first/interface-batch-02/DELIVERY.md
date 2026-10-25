# B 资产接口第二批恢复

本批补 11 个对应实现，其中 5 个此前未触及：浅继承、借用具体值、普通 Deserialize 回调、PrefabName Borrow、纹理 concrete_default。原库存仍为 61：已分析/有实现 42、尚未触及 19、原指令比较 2、原游戏消费者接线 0。

| 规则 | 恢复实现与原作对照 | 复验 |
|---|---|---|
| 浅继承 | 仅 Inherit 复制父状态；具体值、None、MissingDefault 不递归覆盖。已接纹理配置。 | 原 RNE 4 种纹理实例，全部有效标签联合组合 160,000 组，0 差异；复合对象与深继承区别测试。 |
| as_concrete | 直接借用具体值，未继承与缺少默认值分别使用原 panic 文本。 | LLVM inlinee/原 tag gate/panic_fmt 对照；借用身份与失败文本测试。未执行任意泛型原 ABI。 |
| deserialize_inheritable | 普通 typed serde/RON 结果映射 Value；错误保留。与 permissive Option 字段回调分开。 | 原 bool/enum 调用和 Result 分支；缺字段/false/true/坏输入/普通 Option 测试。 |
| PrefabName 借用接口 | 标准 Arc + 借用 UTF-8 名称视图，AsRef/Borrow/Deref/From/as_str；原调用方比长度并复用 memcmp。 | 原两种 getter + 两个 Borrow 内联调用；空名/大小写/路径/Unicode/NUL/pointer/hash/HashMap 借用查询测试。 |
| 纹理 concrete_default | 原零标签对应 None/Linear/MipNone/Color，scanner 已接线。 | 原 0x1420d8c24..38 四字段初始化；缺失字段和显式 None 保留测试。 |

全部 19 项 asset/asset-pipe 测试与 strict Clippy all targets 通过；其中发行物回归继续覆盖 388 个纹理档案、28 个原 prefab 输入文件。原图片输入缺失，完整像素/压缩原入口差分没有因此完成。没有新增依赖或库算法重写。

indexed generic concrete_default 的类型/trait 边界仍未知，不用一个内联 None 初始化推断全部泛型实现。其余 19 个未触及接口和 C 新路径保持独立缺口；原 country-core 运行时接线仍未完成。

规则、原地址、差异修复、源码 hash、机器指令比较范围见 [business-comparison.json](business-comparison.json)；统一接口台账见 [interface-coverage.json](../interface-coverage.json)。
