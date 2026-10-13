// ============================================================
// nani/inc/cull_and_bucket_owned_custom.hlsl
// 功能:所有"自有实例"风格剔除着色器共享的 include 伞头。
//       先于各建筑的记录头文件被包含,提供标准剔除 include 集合
//       以及共用的剔除输入缓冲(cull input buffers)。
// 消费方:_nani_cull_and_bucket_{plaster,smoke,stairs_floor,
//         curved_halftimber}.cs.hlsl 等自有实例变体。
// 绑定:draw_list_info (t20, space0) —— 各绘制列表的描述信息。
// ============================================================

// ==RECONSTRUCTED== shaders/nani/inc/cull_and_bucket_owned_custom.hlsl | evidence: raw disasm of
// _nani_cull_and_bucket_{plaster,smoke,stairs_floor,curved_halftimber}.cs.hlsl bins: the include
// appears FIRST in the embedded source order and carries the standard cull include set plus the
// cull buffers; the owned-instance variants compile this umbrella before their building record
// inc | status: O (include closure); I (body split)

#ifndef TINYGLADE_NANI_CULL_AND_BUCKET_OWNED_CUSTOM_HLSL
#define TINYGLADE_NANI_CULL_AND_BUCKET_OWNED_CUSTOM_HLSL

// 标准剔除 include 集合(数学、帧常量、绘制列表与图集信息)。
#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "../draw_lists.inc.hlsl"
#include "../cull_and_bucket.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "../atlas_part_info.inc.hlsl"

// Shared cull input buffers (present in every owned-custom variant).
// 所有"自有实例"变体共用的剔除输入缓冲:每张绘制列表的描述信息。
StructuredBuffer<NaniDrawListInfo> draw_list_info : register(t20, space0);

#endif // TINYGLADE_NANI_CULL_AND_BUCKET_OWNED_CUSTOM_HLSL
