# 本轮独立审查经验

这些规则来自实际本轮反例，用于改进后续候选与验收；不代表业务已完成。

1. **最终冻结文件才是检查对象。** location V2误将 rustfmt 多文件 stdout 的绝对路径标题复制进 Rust，且 PublicWalls 被同名归档桶覆盖。候选生成只写独立文件；最终文件自身再解析，stdout只作日志；冻结后给每文件SHA与最窄补丁。类型/API跨包变更先查已pin源码，解析不等于类型检查。V3另冻修复，保留V2及失败原因。
2. **验收量应直接对应实际动作。** loading WSI旧 any_presented 对 OUT_OF_DATE/FULL_SCREEN_LOST 也为真，不能证明原生 present成功。保旧恢复策略，新增只统计原始Result::is_ok的只读计数；v3实际3成功present。不能由“走过函数”推断GPU/文件写入/编辑落盘已成功。
3. **按机器码与身份纠正证据归属。** 旧 shader-assets-b/0.spv 是lowered artifact，文件名不能证明原包IR。按安装包字段实际提取offset23/47172B原VS，再逐指令比关键链。官方版本源码、安装dylib、CPU CLI的身份分开记录。日志文字也要核源码：MoltenVK1.4.2的UINT blend warning分支不查blendEnable，故警告不能直接推出应用启用混合。
4. **先测阶段，后改实现。** 世界submit约13秒不能直接归因GPU；可选固定栈trace测出状态准备11.19秒，队列约25ms、GPU等待约101ms。定位每private read重放全部prefix，才作原loop逐字保留的增量事务；保每边界预算准入、revision/producer和首错误顺序，以完整重放为差分oracle。不能扩大容量/删draw/改断言换通过。
5. **所有权接缝用同authority闭合。** ShaderHost分页仍同layout，整group互斥准备，with_push迁页重放冻结FrameGlobals；计算常量接线复用同App现有Arc池并核ArcDevice。禁止另建影子资源、按Device全局找其他App池、取消时重置仍有CPU/GPU owner的页。每份报告列明原事实、schema、工程适配与真实运行证据。
6. **以失败保留边界推进。** 单帧内容与3次loading present已取得真实证据；30帧全执行后live drift仍失败，后续memory/TAA断言未执行就是未验收。保失败日志、原阈值与图像。重build会覆盖同路径测试二进制，执行前后记录SHA及源epoch；独审赶不上旧binary时只认当时执行者记录，不冒称后来重算吻合。

独审报告见本目录具体JSON；root唯一执行生产整合/Cargo/GPU。候选worker只读原证、写合法work，静审者不接管foreign claim。
