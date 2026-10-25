#if TGV_TEST_LIFTED
[numthreads(8, 1, 1)]
#else
[numthreads(1, 1, 1)]
#endif
void cs_main() {}
