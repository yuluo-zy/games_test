/* 直接执行原RNE的完整expand函数；没有重写几何公式或桥接数学库。 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

typedef void (__attribute__((ms_abi)) *expand_fn)(void *, uint64_t, float, float, float);
int main(int argc, char **argv) {
    if (argc != 4) return 2;
    int fd = open(argv[1], O_RDONLY);
    struct stat st;
    if (fd < 0 || fstat(fd, &st)) return 3;
    void *image = mmap((void *)0x140000000ULL, st.st_size,
                      PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_FIXED, fd, 0);
    if (image == MAP_FAILED) { perror("mmap original RNE"); return 4; }
    /* argv[1]必须是按PE sections展开的内存映像，不能把raw RNE文件偏移当RVA。 */
    const unsigned char entry[] = {0x48,0x83,0xec,0x28,0xf3,0x0f,0x10,0x4c};
    if (st.st_size < 0x2b2eaa0 || memcmp((void *)0x140c9c620ULL, entry, sizeof(entry))) {
        fprintf(stderr, "expected mapped original PE image, not raw RNE file\n");
        return 7;
    }
    FILE *in = fopen(argv[2], "rb"), *out = fopen(argv[3], "wb");
    if (!in || !out) return 5;
    expand_fn expand = (expand_fn)(uintptr_t)0x140c9c620ULL;
    uint32_t row[12];
    unsigned count = 0;
    while (fread(row, sizeof(row), 1, in) == 1) {
        uint64_t edge;
        float delta, lower, upper;
        memcpy(&edge, row + 7, 8);
        memcpy(&delta, row + 9, 4);
        memcpy(&lower, row + 10, 4);
        memcpy(&upper, row + 11, 4);
        expand(row + 1, edge, delta, lower, upper);
        if (fwrite(row, sizeof(uint32_t), 7, out) != 7) return 6;
        ++count;
    }
    fclose(in); fclose(out); close(fd);
    fprintf(stderr, "original expand x86_64 executed %u non-panic cases\n", count);
    return 0;
}
