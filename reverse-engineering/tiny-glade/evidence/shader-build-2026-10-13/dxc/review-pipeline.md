# 完整预处理、反射与打包互审

审查者shader_compiler。4模块按原公开依赖API组织真实数据：FileProvider→shader-prepper→宏/#line prepared→真实DXC→SPIR-Q→typed原wire→rspirv后处理→Speedy。原metadata仅用于新产物生成之后的独立差分，producer没有注入原stored字段/依赖或复制原SPIR-V。

独立原证据：638头均0xf0000；13994Source+498Continued全LF/空，支持pack的源正文仅保留LF及原rspirv header规则。反射producer从原404/638重新计算后与所有原stored语义字段一致，实际4/4pipeline tests通过；HashMap序列排序变化被独立consumer1/1记录，未伪称原wire字节完全相同。

全量发现provider错误实际来自自写helper三级引用缺../，owner修引用而没有扩大任意搜索；复验后编译新包成功。4cross-stage资源冲突独立raw OpName/Decorate与原metadata确定是重建色库b14撞VS t14，修两消费者原b21而未覆盖反射producer冲突。完整业务名称去__alias_政策只以B观察闭环，未知碰撞/异常不冒认为原策略。

source dependencies必须来自actual get_include，不从旧包注入；最终213相同191不同，明确源码依赖图/恢复源组织残差。compute零hash不能唯一反演empty tuple，170未独证、其中38非zero未知；raster234真实0/1tuple与source行为验证分别登记。

最终重验根tests-current.log 10/10、strict Clippy通过，实际fresh-fill GPU64words正确；fresh-egui色/深度布局正确、失败注册保持旧布局，没有raster draw。完整404新容器638stage生成后仍有原schema、publicIO、capability及代码/依赖残差，原compiler版本/fork/异常清理未声称等价。

复审当前范围通过，无未修复构建阻断。完整original compiler/config策略与shader算法残差见dxc/README.md、merged-audit.json；不扩大至资产状态机或整帧。