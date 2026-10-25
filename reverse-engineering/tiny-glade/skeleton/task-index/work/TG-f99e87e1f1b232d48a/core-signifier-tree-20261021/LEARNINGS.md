# 本轮可复用经验

1. **ICF 同址名字必须跟回 typed caller 与原内联。** 树选择和插入的 `deny_load / is_corrupted / dragged` 都是假宿主解释，指令传入同一个 `ActiveSession+0x58`，PDB inline 原 `is_olden / is_winter / is_autumn` 及 flowery 才是真业务。先新增假的加载/损坏资源会把实现越接越偏。
2. **向量 lane 的来源比相邻常量更重要。** place_tree 的 MOVSS memory→XMM8 高96位清零；旋转下限是零。不能把邻地址浮点 .49230769（此前甚至错误抄成 .49220002）当第二lane常量。
3. **随机状态与调用顺序也是存档业务。** 同一存储index初始化本地fastrand1.8 RNG，f32→i32(0..10)→f32；把它拆成线程全局随机与手写mix即使范围正确，也无法保恢复确定性。确认库版本后直接复用pinned依赖，独立原指令oracle只做验证不进生产。
4. **先恢复事件提交/失败，再判断能否闭环。** Insert在History之前提交；原History缺input recording直接panic。不能忽略现代Result掩盖历史丢失，也不能未经callee原证就任意expect。已保原pending Query失败与删除阈值NaN边界。
5. **裸字节默认不一定是缺口。** 原SpawnPrefab的16B flags确实全零；反而事件+0x2c是variant、+0x30是bool、+0x40是EventId。不要增加不存在的height/flags模型。
6. **可见UI必须跟到工厂唤醒和回收。** 原tree光标32B是ArcInner，不是可永远leak的裸载荷；原Empty网格分支也要注册真实Mesh/Drawable/TransientMesh。只造Ui2dImage或只排原vtable地址会把成功伪装成占位实体。
7. **保留当前架构，给最窄依赖闭包。** 原地恢复shared builder的五支owned输入，Factory保持现Arc API；仅两个raw调用者更新资源库传参。树第二阶段明确依赖第一阶段after hash，不重新冻结生产覆盖别人的集成。
8. **复用证据必须验证原字节身份。** 本轮完整build3840B与旧Unicorn fixture函数hash吻合才复用其分支数据；fixture中的library/Bevy边界仍不能算整机或GPU验证。native-evidence-pack负责SHA/PDB/范围/机器码验证，不替代语义独审和Rust测试。
