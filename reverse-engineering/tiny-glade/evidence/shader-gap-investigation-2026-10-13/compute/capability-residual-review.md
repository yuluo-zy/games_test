# Capability 残差审查（有界，2026-10-13）

隔离 corpus-native-recipes 的 **385 个新包/600 个模块中，281 capability 集合相同、319 不同**。独立按 .bin stage 字段解码并逐指令读取全部原638与新638（含38个未打包但DXC成功的诊断模块）；全raw有300集合相同。逐case原/新SHA、type/指令位置、增减集合及分类见 capability-residual-review.cases.json；输入report SHA、统计见 .summary.json。以下数量按600已打包模块，类别可重叠。

| 差异 | 数量 | 实际归因与边界 |
|---|---:|---|
| 仅移除 RuntimeDescriptorArray(5302) | 274 | 最大组；所有移除该cap的307pack/326raw案例，原/新descriptor数组链均没有RuntimeArray。buffer内部RuntimeArray没有误计为descriptor。属于原声明残留/新清理差异；不据此猜原DXC版本。其他4个color模块确有Runtime bindless_textures_dyn，cap仍保留。 |
| 新增 Int64(11)，同时移除5302 | 13 | 原无Int64/OpTypeInt64，新有u64类型，是额外64位功能需求的compatibility残差；原29次OpBitcast输入uint2(u32×2)，新29次输入u64。直接来源是项目 batch_lift.py:200 主动生成 RawBufferLoad<T>(((uint64_t)addr.y<<32)|addr.x)，不是官方DXC凭空增加，也不能泛归因SPIRV-Cross自动转换。 |
| 图像格式/cap trim | 22 | 原packed R11fG11fB10f等变成新Unknown，带来Read/WriteWithoutFormat(55/56)；tgsr_blur_lum 原R8变新R32f。保留这类源格式/资源精度残差，不靠删cap掩盖。公开storage-RW补丁和现代官方已验正确read/write发cap行为不能自动修复源码格式变化。 |
| ImageQuery(50)移除 | 4 | 这4case原/new都没有OpImageQuery*，属于声明残留；不推广其他shader。 |
| 非均匀索引5301/5307移除 | 4 | 这4depthcase原/new无NonUniform装饰，4color Runtime案例不在此组。 |
| PhysicalStorageBufferAddresses(5347)移除 | 2 | curved_halftimber两变体原也没有physical pointer类型/值，属声明残留；没有误称删掉活跃原物理读。 |
| ComputeDerivativeGroupQuads(5288)新增 | 1 | dof_analyze原/new QuadSwap均4次，新多一个DerivativeGroupQuadsKHR ExecutionMode5289（LocalSize仍8×8）。属于已观察新官方quad lowering/声明策略差异；未确认原fork实现或整shader等价。 |
| GroupNonUniformVote(62)移除 | 1 | final_post depth原Any1、ImageGather1，新body仅Return且资源接口减少；是source recipe/body差异，不应只补cap冒充恢复。未评估全部可观察语义。 |

**原与新全638都没有 Float64 cap或OpTypeFloat64，prepared源也没有double声明；原Int64同样0，新增Int64严格13case。原与新OpConvertUToPtr均0，实际地址转指针为OpBitcast。** 名字/数值依据 [Khronos grammar](https://raw.githubusercontent.com/KhronosGroup/SPIRV-Headers/main/include/spirv/unified1/spirv.core.grammar.json)，摘录已冻结 .grammar-excerpts.json。

作者公开 [RawBufferLoad2补丁16ceb560](https://github.com/h3r2tic/DirectXShaderCompiler/commit/16ceb560954f2238dcdad73a75d5e5842e58c839) 增加uint2地址输入，恰好可表达这里原地址type路径；本机官方1.9真实探针拒绝该名字。补丁全文与HTTP200记录在 shader-build-2026-10-13/provenance/dxc-fork-patch-1.json，probe在dxc/official-fork-capability-probes.json。要保持原无Int64的设备功能需求，应恢复uint2直达physical pointer的公开fork适配路径并验证29处产物；只用u64新源码再删Int64声明不成立。本轮没有构建DXC、修改源码/原binary，未确认原作用此commit或任何精确版本。_logical与fix_physical_loads只是lifting临时适配，不能登记为原私有编译器/打包策略。

132个zero-key compute原配置仍未证明为空tuple。本轮不据cap相同宣称源算法、数值格式、异常或业务等价；原游戏进程未执行。无需重开SDK，先前compute worker仍saved/stopped。

Pipeline独立互审通过：../build-entry/capability-peer-review.json 重新读取原638/new638，核13新增Int64、29对地址类型链、原/newOpConvertUToPtr0、326子集无runtime descriptor变量和4真实color runtime保留。没有据公开补丁匹配原形状认定原EXE编译器身份。
