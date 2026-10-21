/* 仅独立依赖数值参考：运行原锁定Little CMS 2.15的公开C API，不执行原游戏或替代生产算法。 */
#include "lcms2.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <inttypes.h>

#define EDGE 32
#define COUNT (EDGE * EDGE * EDGE)
static cmsUInt16Number values[COUNT][4];
static unsigned channel(unsigned i) { return i * 65535u / 31u; }
static void grid(void) {
    unsigned i = 0;
    for (unsigned b = 0; b < EDGE; ++b)
        for (unsigned g = 0; g < EDGE; ++g)
            for (unsigned r = 0; r < EDGE; ++r, ++i) {
                values[i][0] = (cmsUInt16Number)channel(r);
                values[i][1] = (cmsUInt16Number)channel(g);
                values[i][2] = (cmsUInt16Number)channel(b);
                values[i][3] = 255;
            }
}
static int run(cmsHPROFILE input, cmsHPROFILE output, const char* label, const char* directory) {
    grid();
    cmsHTRANSFORM transform = cmsCreateTransformTHR(NULL, input, TYPE_RGBA_16,
        output, TYPE_RGBA_16, INTENT_PERCEPTUAL, 0);
    if (!transform) { fprintf(stderr, "cannot create %s transform\n", label); return 1; }
    cmsDoTransform(transform, values, values, COUNT);
    cmsDeleteTransform(transform);
    char path[4096];
    if (snprintf(path, sizeof(path), "%s/%s.rgba16le", directory, label) >= (int)sizeof(path)) return 2;
    FILE* file = fopen(path, "wb");
    if (!file) return 3;
    uint64_t fnv = UINT64_C(14695981039346656037);
    unsigned changed = 0, maximum = 0, maximum_i = 0, maximum_c = 0, max_c[3] = {0,0,0};
    for (unsigned i = 0; i < COUNT; ++i) {
        unsigned expected[4] = {channel(i % 32), channel(i / 32 % 32), channel(i / 1024), 255};
        for (unsigned c = 0; c < 4; ++c) {
            unsigned v = values[i][c];
            unsigned delta = v > expected[c] ? v - expected[c] : expected[c] - v;
            if (delta) ++changed;
            if (c < 3 && delta > max_c[c]) max_c[c] = delta;
            if (delta > maximum) { maximum = delta; maximum_i = i; maximum_c = c; }
            for (unsigned byte = 0; byte < 2; ++byte) {
                unsigned char value = (unsigned char)(v >> (8 * byte));
                if (fputc(value, file) == EOF) { fclose(file); return 4; }
                fnv = (fnv ^ value) * UINT64_C(1099511628211);
            }
        }
    }
    if (fclose(file)) return 5;
    printf("{\"label\":\"%s\",\"cmm\":%d,\"pixels\":%d,\"fnv1a64_le\":\"%016" PRIx64 "\",\"changed_channels\":%u,\"max_abs\":%u,\"max_index\":%u,\"max_channel\":%u,\"per_channel_max\":[%u,%u,%u],\"sample31\":[%u,%u,%u,%u]}\n",
        label, cmsGetEncodedCMMversion(), COUNT, fnv, changed, maximum, maximum_i, maximum_c,
        max_c[0], max_c[1], max_c[2], values[31][0], values[31][1], values[31][2], values[31][3]);
    return 0;
}
static void tag(cmsHPROFILE memory, cmsHPROFILE reopened, cmsTagSignature signature, const char* name) {
    const cmsCIEXYZ* a = cmsReadTag(memory, signature);
    const cmsCIEXYZ* b = cmsReadTag(reopened, signature);
    if (a && b) printf("{\"tag\":\"%s\",\"memory\":[%.17g,%.17g,%.17g],\"serialized\":[%.17g,%.17g,%.17g]}\n", name, a->X,a->Y,a->Z,b->X,b->Y,b->Z);
}
int main(int argc, char** argv) {
    if (argc != 2 || cmsGetEncodedCMMversion() != 2150) { fprintf(stderr, "requires output directory and LCMS 2.15\n"); return 10; }
    cmsHPROFILE input = cmsCreate_sRGBProfileTHR(NULL);
    cmsHPROFILE memory = cmsCreate_sRGBProfileTHR(NULL);
    if (!input || !memory) return 11;
    cmsUInt32Number size = 0;
    if (!cmsSaveProfileToMem(memory, NULL, &size)) return 12;
    void* bytes = malloc(size);
    if (!bytes || !cmsSaveProfileToMem(memory, bytes, &size)) return 13;
    char path[4096];
    if (snprintf(path,sizeof(path),"%s/reference-srgb.icc",argv[1]) >= (int)sizeof(path)) return 14;
    FILE* file = fopen(path,"wb");
    if (!file) return 15;
    if (fwrite(bytes,1,size,file) != size) { fclose(file); return 16; }
    if (fclose(file)) return 17;
    cmsHPROFILE reopened = cmsOpenProfileFromMemTHR(NULL, bytes, size);
    free(bytes);
    if (!reopened) return 18;
    tag(memory,reopened,cmsSigRedColorantTag,"rXYZ");
    tag(memory,reopened,cmsSigGreenColorantTag,"gXYZ");
    tag(memory,reopened,cmsSigBlueColorantTag,"bXYZ");
    const cmsToneCurve* a = cmsReadTag(memory,cmsSigRedTRCTag);
    const cmsToneCurve* b = cmsReadTag(reopened,cmsSigRedTRCTag);
    if (a && b) printf("{\"tag\":\"rTRC@0.5\",\"memory\":%.17g,\"serialized\":%.17g}\n",(double)cmsEvalToneCurveFloat(a,0.5f),(double)cmsEvalToneCurveFloat(b,0.5f));
    int result = run(input,memory,"memory-to-memory",argv[1]);
    if (!result) result = run(input,reopened,"memory-to-serialized",argv[1]);
    cmsCloseProfile(reopened);
    cmsCloseProfile(memory);
    cmsCloseProfile(input);
    return result;
}
