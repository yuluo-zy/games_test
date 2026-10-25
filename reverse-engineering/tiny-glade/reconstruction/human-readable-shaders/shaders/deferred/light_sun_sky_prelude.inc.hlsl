// ==RECONSTRUCTED== shaders/deferred/light_sun_sky_prelude.inc.hlsl | status: O (include edge); I (any extra helpers — inlined away)
// MERGE (gi-deferred review 2026-10-13): adopted candidate B (include-umbrella shape matches
// the target: prelude included early, right after common, pulling frame_constants /
// halfres / fullscreen_triangle_vert).
// 中文说明:
//   这是延迟渲染"太阳+天空"光照 pass 的公共前导头文件,被 8 个 bin
//   (deferred_{canopy,grass,plant,solid}_light_sun_sky) 以 include 方式嵌入。
//   作用:在 common 伞头之后尽早引入,统一拉取帧常量 / 半分辨率工具 /
//   全屏三角形顶点着色器,并声明全屏三角形 VS 入口。各 pass 自己的绑定
//   与光照主体位于 raster 源文件 / light_sun_sky.inc.hlsl 中。

#ifndef TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
#define TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/halfres.hlsl"
#include "inc/pack_unpack.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

#endif // TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
