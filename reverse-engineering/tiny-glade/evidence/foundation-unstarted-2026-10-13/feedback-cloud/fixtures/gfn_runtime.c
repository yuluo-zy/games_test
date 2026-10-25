#include <stdint.h>
static int32_t code;
static unsigned char cloud;
static int32_t sequence;
__declspec(dllexport) int32_t gfnInitializeRuntimeSdk(int32_t language) {
    sequence=sequence*10+1; return language == 0 ? code : -9999;
}
__declspec(dllexport) int32_t gfnShutdownRuntimeSdk(void) {
    sequence=sequence*10+3; return -99;
}
__declspec(dllexport) unsigned char gfnIsRunningInCloud(void) {
    sequence=sequence*10+2; return cloud;
}
__declspec(dllexport) void gfnFixtureSet(int32_t value,unsigned char state) {
    code=value; cloud=state; sequence=0;
}
__declspec(dllexport) int32_t gfnFixtureSequence(void) { return sequence; }
