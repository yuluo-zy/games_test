// ==RECONSTRUCTED== shaders/inc/math_const.hlsl | evidence: included in 335 bins; values cross-checked against inlined literals in consumer SPIR-V (e.g. _billboard_forest: 6.283185482025146484375f = TWO_PI, 1.6180340051651f = GOLDEN_RATIO; _blit_stretch_decode_srgb: 0.0404499992728f, 0.947867333889f = 1/1.055, 0.077399380505f = 1/12.92) | status: I (constant names Inference — constants fold to literals in SPIR-V, names not embedded); O (values match observed literals where present)
//
// Standard mathematical constants. All are compile-time constants; dxc folds
// them into literals at use sites, so names are not recoverable from SPIR-V.

#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif // TINYGLADE_INC_MATH_CONST_HLSL
