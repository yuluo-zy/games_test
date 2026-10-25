# spherecast options / max-TOI 有界增量合同

目标为当前 B 工程。沿用已手工重构的 RaycastWorld、Raycasting、结果类型、terrain owner 与模块结构；没有恢复旧 Rust ABI、重建 walk 模块或另造碰撞后端。冻结 v1 未修改，本候选以已整合并格式化的 v1 源码为基线。

## 交付与 API

`world.rs` / `sphere-options.patch` / `manifest.json`。保留两个现有 `spherecast` 的参数和返回类型，兼容入口以依赖默认 ShapeCastOptions 委托给新增 `spherecast_with_options`。实际扫描算法只有现有 Rapier 私有核心这一份。新增入口完整接收锁定 parry3d 0.17.0 的 ShapeCastOptions，含 maximum、target_distance 和两个 bool；没有自建近似 Options 类型或强塞原内存布局。

当前工作区没有调用这两个现代 spherecast 的外部源码，只有本文件包装链；system-critters 的同名调用走其独立 legacy_spherecast。本补丁不更改 legacy 接口/算法，也不依赖缺失的上层模块才能调用新增入口。

## 原版观察事实

- B `RaycastWorld::spherecast` 0x1408ccfc0：radius 来自 XMM3，Options 指针来自栈实参。0x1408cd040/044/050/054 分别取 Options+0/+4/+8/+9，即 max_time_of_impact、target_distance、stop_at_penetration、compute_impact_geometry_on_penetration。它们与锁定 parry3d 0.17.0 的四字段完全对应。调用 cast_shape 前复制四字段；0x1408cd23f 在命中后仅以 hit TOI 更新 XMM7，下一槽最大 TOI 因此收紧。当前源码每槽调用 default()，丢弃所有调用方 Options 且允许更远的后槽覆盖。
- 同函数未命中时仍将初始 Options.max_time_of_impact 写入 result+0x30。候选保留这一已初始化标量，其余原未初始化 miss payload 继续使用现结果类型的安全默认值，不声称 ABI 一致。
- B `Raycasting::spherecast` 0x1408cc390：0x1408cc3b9/3c1 接收同一 Options 指针和 mask 并转发；0x1408cc3ec/3f4 计算 `options.max_time_of_impact + radius` 作为 terrain.intersect_ray 的上限。之后计算球体 TOI，再在 0x1408cc4a0 与原 Options.max_time_of_impact 比较。当前代码以 f32::MAX 查询地形并将球体 TOI 与 radius 比较，属于参数错绑。
- 距离运算顺序由 0x1408cc410..475 确认：`sqrt((dy² + dx²) + dz²)`；方向长度为 `sqrt(dz² + (dy² + dx²))`。方向不要求归一化。
- **旧报告的 4e-8 有误**：0x1408cc47c 从 0x142aa3270 加载的 8 字节为 `00000000 08e53c1e`，即 `(0.0, f32::from_bits(0x1e3ce508)) = (0.0, 1e-20)`。MAXPS 后还有显式 NaN fallback。候选用原位型和有序比较保留边界；不调用会改掉表达式顺序的 Vec3::length。
- 球体地形 TOI 是 `max(distance-radius,0) / max(direction_length,1e-20)`，NaN 分别回退到 0 与 1e-20。只有 `toi <= options.max_time_of_impact` 时才成为候选；已有 collider TOI 严格更小才保留 collider，等距选 terrain。原法线翻转与 world witness 输出沿用已有核心算法。

## 实际原消费者与当前边界

fresh callers 只发现 walk camera 的 0x140994320 `constrain_to_be_above_terrain` 与 0x140994700 `translate_screen_space`。前者使用 radius=0.45、maxTOI=f32::MAX、target_distance=0、两 bool=false（先 terrain mask 0x20，再传入选定 mask）；后者 radius=0.5、maxTOI=1、target_distance=0、两 bool=false。这些消费者说明 Options 是实际业务输入，不能用 radius 代替。

当前 camera_rig 已有其他重构模块，但没有 walk.rs。未为满足旧目录/ABI重建它；未来现 camera owner 接入时直接调用现新增 with_options，并传上述已证实配置。对完整 walk 移动/滑动或相机调度不扩大验收。全局 registry/user_data/source payload 的开放项与 v1 保持；它们不是这次原 Options 链的完成声明。

## 证据、核验与状态

`sphere-original.json`、`sphere-callsite-assembly.json`、`sphere-camera-callers.json` 为只读 shared B 数据库 fresh 结果。`static-byte-identity.json` 将 wrapper、world、两个原 caller 与 fallback 常量共五个范围逐字节对照 analyzed B 与发行 RNE，全部一致；SHA 均显式记录。没有查询 C 地址或修改数据库。

`sphere_native_probe.py` 复用项目现有 `scripts/parallel-camera/native.py` 的身份绑定指令执行器。已在宿主执行 B 原 wrapper 0x1408cc390，40 个有界输入，检查完整四字段 Options / radius / mask 的转发。原指令完成 terrain limit 运算、极小方向/NaN fallback、球体 TOI、mesh/terrain 严格近邻与等距选择。仅两次实际 world/terrain 查询返回由已解析边界提供；不声称运行真实 Rapier 或地形发现。结果 `sphere-native-wrapper.json` 显式 `target_executed=true`、`candidate_executed=false`。

候选附三项针对性测试：真实依赖的 maxTOI/target_distance/跨槽最近命中；真实 terrain owner 在域外 y=0 平面的 maxTOI 与兼容入口；12 个发行原指令数值投影 fixture。rustfmt 解析通过、git apply --check 通过；未编译或运行 Rust 测试，按 root 约定由整合者统一验证。当前为 implemented_unverified，原指令边界、候选源码测试、完整游戏验收分别记录。

冻结指纹见 manifest.json：base `a6cc0bfae56138abe6b9f15eaf9ea5b389608f0692b6e9797fb14a7b1c04e766`；candidate `0b00c2d8051e180f0b6a46e8c4393605e197ac6357c99ac22881f781256954af`；patch `980e924ffd7a4507fde26168acc5eaa7e6baf8bb419c3bbf9d7ea93ce6a95017`。
