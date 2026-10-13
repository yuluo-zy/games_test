#define PERM_quality 1
#define TGV_TEST_LIFTED 1
#line 1 "shaders/main.cs.hlsl"
#if TGV_TEST_LIFTED
[numthreads(8, 1, 1)]
#else
[numthreads(1, 1, 1)]
#endif
void cs_main() {}
