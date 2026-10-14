// ============================================================
// inc/consts.hlsl —— 共享游戏常量
// 用途:存放跨 pass 的共享常量。内容为暂定:命名常量无法从编译后
//   SPIR-V 恢复(会折叠成字面量),只在消费族确认共享常量时才补充。
// 入口:无(纯头文件)
// 关键常量:RENDER_SCALE 渲染缩放/半分辨率因子
// ============================================================
// ==RECONSTRUCTED== shaders/inc/consts.hlsl | evidence: embedded in 250 bins; no named constants recoverable (constants fold to literals; only TEXTURE_SIZE_WS appears as a named define, in the water family, not here); no inlined code attributed to this file | status: U (member names/values Inference — cannot be recovered from SPIR-V; kept minimal so consumers that need specific game constants carry them locally)
//
// Shared game constants. Content is provisional: named constants are not
// recoverable from compiled SPIR-V (they fold into literals). Keep additions
// here only when a consumer family confirms a shared constant.

#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL

// 跨多个 pass 使用的渲染缩放/半分辨率因子。
static const float RENDER_SCALE = 1.0f;

#endif // TINYGLADE_INC_CONSTS_HLSL
