# geometry 工程化重构 · 2026-10-18

用户授权以功能正确性优先调整恢复代码。当前模块说明见 [geometry/README.md](../../crates/country-core/src/geometry/README.md)，问题状态见 [REVIEW.md](../../crates/country-core/src/geometry/REVIEW.md)。

## 改动

- G1：资产库与 draw 视图增加独立底层缓冲查询，砖绘制不再把稀疏缓冲当作普通实例计数缓冲。保留完整类型校验，并增加真实资产库进入砖绘制的组合回归。
- G2：WallBrickInstance 直接再导出 BrickTransformSsbo，CPU 生产者与 GPU 元素成为同一类型；墙组件持有独占分配，真实 PostUpdate 资产维护阶段完成注册、更新、flush 和删除回收，宿主安装相同设备的库与就绪状态。
- 增加明确的待提交状态。重复调用不清空旧几何，显式空更新正确删除；空间不足时旧几何和 CPU 更新均保留，错误缓冲拒绝写入。
- 稀疏批量替换复用已有 chunk，并在修改前一次检查容量；设备失败保留 dirty 标记，下帧重试成功前由墙砖回调阻止消费。
- 盒体改用具名面结构，规范化反向范围；平面边长规范化；球体在分配前验证半径，顶点数饱和计数，UV 夹取舍入误差。
- 生产代码与测试分文件；生产注释用中文描述职责和约束。布局检查复用 bytemuck，锁访问复用 parking_lot，球体细分继续复用 hexasphere。

## 性能与拥有权

空场景和新空墙不会创建砖缓冲。创建共享缓冲不再分配并上传额外的整块 192 MiB 清零数组；仍保留一份 192 MiB CPU 影子和对应设备容量，避免在本轮改变 shader 的固定容量合同。成功上传后释放每墙的待提交 Vec。

每次更新只锁一次空闲 chunk 列表，连续复制整块数据；同帧合并设备上传区间。静止帧不清空分配、不复制砖列表、不执行设备写入。活动 chunk 直接从位图迭代，不再创建中间 chunk Vec。性能结论由调用次数与内存分配路径支持，没有宣称未经测量的帧率或倍数提升。

墙组件销毁后分配进入既有延迟回收队列。GPU 缓冲仍由同一宿主资源表持有，不引入第二套分配器或设备。设备更新失败不声称事务回滚；通过保留全部脏范围与暂缓墙砖消费恢复。

## 验证

最终检查均通过，详见 [checks.json](checks.json)：

| 检查 | 结果 | 日志 |
|---|---|---|
| country-core 全部 lib 测试 | 2087 passed / 0 failed / 0 ignored | [regression-final.log](regression-final.log) |
| 资产库集成回归 | 6 passed / 0 failed / 0 ignored | 同上 |
| tiny-glade 的 geometry_gpu 测试编译 | 退出 0 | [check-gpu-attempt3.log](check-gpu-attempt3.log) |
| 强制 Vulkan 宿主 GPU 实跑 | 1 passed / 0 failed / 0 ignored / 0 SKIP | [gpu-host.log](gpu-host.log) |
| 本模块、稀疏缓冲和 GPU 测试格式检查 | 退出 0 | [format.log](format.log) |

GPU 回归用实际 BufferHost 和同设备 TextureHost 执行生产 ECS 维护调度，回读跨两个 chunk 的 33 个完整实例，并验证静止帧数据不变、修改后字节相符及实体删除回收。测试没有用跳过 GPU 或忽略测试取得成功。

保留 check-core.log、geometry-tests.log、check-gpu-test.log、regression.log、check-gpu-final.log 等首次失败尝试。构建中遇到同工作区资产模块重复定义和输入接口重构：删除重复 sampled_views / named_textures，保留原实现；toolbar 中三个调用适配了新的 Result 返回值，记录来源为内部 deconstruct/retain，使用明确的内部合同 expect，没有改输入算法。未以失败尝试替代最终成功日志。

未执行全 workspace 或严格 Clippy。既有模块外警告保留；锁定的旧 bytemuck derive 在当前编译器上也会对生成的布局检查辅助项产生 dead_code 警告，未通过全局关闭 lint 隐藏。

## API 调整

| 原接口 | 当前接口 |
|---|---|
| 直接修改 public brick_transforms | 通过 update 提交；pending_instances 只读检查 |
| 上传时传 Option<writer>，失败 panic | 传共享稀疏缓冲；分配由墙持有，返回 Result<bool, WallUploadError> |
| Vec 为空代表无数据或空更新 | 独立 pending 标记，空更新和静止帧可区分 |
| CPU/GPU 两种同字段记录 | 一个公共实例类型，旧 CPU 名称保留再导出 |
| SphereError struct | TooManyVertices / InvalidRadius 枚举 |
| 任意复制墙组件 | 移除 Clone，避免隐式复制分配拥有权 |

bytemuck 使用 workspace 已锁定版本与 derive 特性，没有新增算法库或更改依赖版本。原反汇编行为、旧差分和重构前的 6 项诊断只作历史证据，不用于验证本轮行为；当前指纹见 verification.json。

## 范围

本轮闭合几何元素到共享缓冲和现有砖绘制查询的接口问题；没有创建完整场景 draw-list/culling，也没有补造墙代理实体或以局部测试宣称整游戏画面完成。独立 GPU 测试额外开启 TRANSFER_SRC 仅用于回读，生产分配/上传继续使用真实 BufferHost。
