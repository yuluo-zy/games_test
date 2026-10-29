/* 只执行原1408197f7..81e算术；尾部movaps/ret仅将原XMM1返回给CPU探针。
 * 不重写厚度公式，不桥数学函数；输入/输出均使用原始f32位型。 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>

__attribute__((naked)) static float invoke(void *code, float old, float sign,
                                         float target, float delta) {
    __asm__ volatile(
        "subq $88, %rsp\n"
        "movss %xmm0, 48(%rsp)\n"
        "movss %xmm3, 40(%rsp)\n"
        "movaps %xmm1, %xmm13\n"
        "movaps %xmm2, %xmm6\n"
        "movl $0x3f800000, %eax\n"
        "movd %eax, %xmm8\n"
        "movq %rsp, %rax\n"
        "leaq 56(%rsp), %rcx\n"
        "xorq %r11, %r11\n"
        "callq *%rdi\n"
        "addq $88, %rsp\n"
        "retq\n");
}

int main(int argc, char **argv) {
    if (argc != 4) return 2;
    FILE *fragment = fopen(argv[1], "rb");
    FILE *input = fopen(argv[2], "rb"), *output = fopen(argv[3], "wb");
    if (!fragment || !input || !output) return 3;
    uint8_t *code = mmap(NULL, 4096, PROT_READ | PROT_WRITE | PROT_EXEC,
                         MAP_PRIVATE | MAP_ANON, -1, 0);
    if (code == MAP_FAILED) return 4;
    size_t size = fread(code, 1, 512, fragment);
    if (size != 39 || memcmp(code, "\xf3\x44\x0f\x59\x68\x28", 6)) return 5;
    /* 原片段输出XMM1；这四字节不是算法，只是宿主调用的返回桥。 */
    memcpy(code + size, "\x0f\x28\xc1\xc3", 4);
    uint32_t words[4];
    while (fread(words, sizeof words, 1, input) == 1) {
        float args[4], result;
        memcpy(args, words, sizeof args);
        result = invoke(code, args[0], args[1], args[2], args[3]);
        if (fwrite(&result, sizeof result, 1, output) != 1) return 6;
    }
    return fclose(output) != 0;
}
