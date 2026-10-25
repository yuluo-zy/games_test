# 父 agent 共享集成待核对

- 状态：并行实现中，未验证；阶段4 54/67报告是历史快照。
- 当前新增 gates / Rapier backend / DecoratorArchivist 核心 export；新增模块与旧报告不自动形成全工程证明。
- Nani 门柱使用 SolidVertexColor，不能复用 roof StoneFloor 上传格式；真实 GPU record/上传消费者未恢复，CPU 快照与最终上传边界必须分列。
- 门、矩形编辑和碰撞 runtime 登记需要等待各 agent 明确 register、事件及阶段依赖；父独占共享 lib.rs。
- 当前 Rapier 查询仅平台 subworld7；DirtyTiles 完整原始 raster 和其下游、其他子世界查询过滤须继续恢复。

- 门 bundle+0x30 是 Named mesh descriptor 判别0，已纠正初步额外component解释；Solid flags单独由真实Query owner给出，不复制该判别。
- 新 Solid64 CPU schema和矩形/存储/free-wall CLI输入已实现但未验证。Stage新增PlatformCollision和Gates分别排在Stairs后、Arches后；原fork完整顺序仍未恢复。

- Solid flags现已由FunctionSystem元数据与实际fetch定位为SemiTransparent/FlatLighting存在性；使用真实optional Component query，移除初版required flags DTO。新增GPU record CPU生成仍未验证，真实NaniActiveInstance仍只由GPU后端产生。

- 已登记gates、wall_platform_collision、矩形/archivist/pending、decorator_info_binding六个运行模块；父新增dirty_tiles/decorator_info/Solid64 core aliases。
- Native Info72 binding after Placement/before Openings；旧placement intent已按NativeDecoratorOpeningIds退出，旧archivist由owned agent添加NativeID guard。最终核对仍需覆盖同ID换owner、忘记映射、旧intent/removal与native upsert同帧竞争。
- CLI已接UI80/finish、实际storage/archive/Info绑定、free-wall snapshot、64row dirty配置及platform ray query。新增inspection是运行候选输出，不构成执行证据；当前未构建/未测试。

- Fence依赖perchance精确锁定=0.4.0（原B Cargo有该包，复用RNG不实现）；新增manifest尚未执行Cargo lock解析/构建，Cargo.lock需最终统一生成并核对macaw/once_cell/ordered-float原版本。禁止将旧158身份报告套到新manifest。
- 原nearest_segment仅公开可见性供anchor owner复用，算法不改；旧proof仍历史快照，最终统一验证。
