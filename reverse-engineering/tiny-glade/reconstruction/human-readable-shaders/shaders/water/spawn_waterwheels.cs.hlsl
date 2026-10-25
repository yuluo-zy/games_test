// ==RECONSTRUCTED== shaders/water/spawn_waterwheels.cs.hlsl | evidence: spirv-cross + raw disasm of _water_spawn_waterwheels.cs.hlsl_cs_main_0.bin — entry is EMPTY (only LocalSize 64,1,1 + cs_main; no bindings, no body ops); header carries OpCapability RuntimeDescriptorArray + SPV_EXT_descriptor_indexing + GLSL.std.450 import, which the original frame_constants.hlsl emitted via a dead unbounded-array declaration (verified 307/307 bins: frame_constants included <=> RDA) | status: O (empty body + capabilities verified from SPIR-V); I (why: waterwheel spawn happens CPU-side, this GPU pass is a stub; RDA/import reproduced via a folding construct)
// 中文说明:
//   用途: 水车生成的水上 pass 是空桩(实际生成在 CPU 侧);此 shader 只为在
//         模块头保留 RuntimeDescriptorArray/SPV_EXT_descriptor_indexing 能力
//         与 GLSL.std.450 import,以对齐原 bin 的头信息。
//   入口: cs_main(compute, numthreads 64x1x1),函数体为空。
//   变体: 无(TGV 守卫不存在,只有单一实现)。
//   绑定: 无实际资源;仅保留 frame_constants 死声明占位。

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/perlin.hlsl"

// 原始的 frame_constants.hlsl 带有一个 unbounded-array 声明,即使它处于死亡
// 状态也会发射 RuntimeDescriptorArray + SPV_EXT_descriptor_indexing。重建的
// inc/ 层省略了它;这里照原样补回(dxc 会丢弃该声明)。
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    // 常量折叠的固有函数:保留原模块的 GLSL.std.450 import(出货模块本身为空)。
    float _dead = smoothstep(0.0f, 1.0f, 0.5f);
}
