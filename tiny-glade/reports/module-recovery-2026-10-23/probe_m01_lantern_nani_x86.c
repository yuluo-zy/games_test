/* 直接执行原RNE灯实例的完整数值块；原pack_2x16f和半精度依赖仍执行原机器码。
 * 只提供真实字段入口寄存器和输出出口，不在宿主重写TRS/打包算法或Bevy查询。 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <signal.h>
#include <sys/ucontext.h>

extern void native_lantern(const void *, const void *, uint32_t, uint64_t, void *);
extern void lantern_capture(void);
__asm__(
".text\n"
".globl _native_lantern\n"
"_native_lantern:\n"
"pushq %rbp\n pushq %rbx\n pushq %r12\n pushq %r14\n pushq %r15\n"
"subq $0x180,%rsp\n leaq 0x80(%rsp),%rbp\n"
"movq %rdi,%rbx\n movq %rsi,%r15\n movl %edx,%r12d\n"
"movq %rcx,0x60(%rbp)\n movq %r8,0x70(%rbp)\n"
"movabsq $0x142925904,%r11\n movss (%r11),%xmm7\n"
"movabsq $0x14292e980,%r11\n movaps (%r11),%xmm8\n"
"movabsq $0x14207ad95,%r11\n jmpq *%r11\n"
".globl _lantern_capture\n"
"_lantern_capture:\n"
"movq 0x70(%rbp),%r11\n"
"movaps -0x40(%rbp),%xmm0\n movups %xmm0,(%r11)\n"
"movaps -0x30(%rbp),%xmm0\n movups %xmm0,0x10(%r11)\n"
"movaps -0x20(%rbp),%xmm0\n movups %xmm0,0x20(%r11)\n"
"movaps -0x10(%rbp),%xmm0\n movups %xmm0,0x30(%r11)\n"
"movss %xmm3,0x40(%r11)\n"
"addq $0x180,%rsp\n popq %r15\n popq %r14\n popq %r12\n popq %rbx\n popq %rbp\n retq\n");

static volatile uint32_t current_id;
static void fault(int sig, siginfo_t *info, void *context) {
    ucontext_t *cpu = context;
    fprintf(stderr, "signal=%d case=%u rip=%llx address=%p\n", sig, current_id,
        cpu->uc_mcontext->__ss.__rip, info->si_addr);
    _exit(128 + sig);
}

int main(int argc, char **argv) {
    if (argc != 4) return 2;
    int fd = open(argv[1], O_RDONLY);
    struct stat st;
    if (fd < 0 || fstat(fd, &st)) return 3;
    uint8_t *raw = mmap(NULL, st.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (raw == MAP_FAILED) return 4;
    uint32_t nt, image_size, header_size;
    uint16_t sections, optional;
    memcpy(&nt, raw + 0x3c, 4); memcpy(&sections, raw + nt + 6, 2);
    memcpy(&optional, raw + nt + 20, 2);
    memcpy(&image_size, raw + nt + 24 + 56, 4);
    memcpy(&header_size, raw + nt + 24 + 60, 4);
    void *image = mmap((void *)0x140000000ULL, image_size,
        PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANON | MAP_FIXED, -1, 0);
    if (image == MAP_FAILED) return 5;
    memcpy(image, raw, header_size);
    for (unsigned i = 0; i < sections; ++i) {
        const uint8_t *s = raw + nt + 24 + optional + i * 40;
        uint32_t rva, size, offset;
        memcpy(&rva, s + 12, 4); memcpy(&size, s + 16, 4); memcpy(&offset, s + 20, 4);
        if ((uint64_t)offset + size > (uint64_t)st.st_size || (uint64_t)rva + size > image_size) return 6;
        memcpy((uint8_t *)image + rva, raw + offset, size);
    }
    const uint8_t expected[] = {0xf3, 0x41, 0x0f, 0x10, 0x47, 0x0c};
    if (memcmp((void *)0x14207ad95ULL, expected, sizeof(expected))) return 7;
    uint8_t bridge[13] = {0x49,0xbb,0,0,0,0,0,0,0,0,0x41,0xff,0xe3};
    uintptr_t capture = (uintptr_t)&lantern_capture;
    memcpy(bridge + 2, &capture, 8);
    memcpy((void *)0x14207af29ULL, bridge, sizeof(bridge));
    struct sigaction action = {0}; action.sa_sigaction = fault; action.sa_flags = SA_SIGINFO;
    sigaction(SIGSEGV, &action, NULL); sigaction(SIGBUS, &action, NULL);
    FILE *in = fopen(argv[2], "rb"), *out = fopen(argv[3], "wb");
    if (!in || !out) return 8;
    uint32_t row[24], result[18];
    unsigned count = 0;
    while (fread(row, sizeof(row), 1, in) == 1) {
        _Alignas(16) uint32_t transform[12];
        memcpy(transform, row + 1, sizeof(transform));
        result[0] = current_id = row[0];
        native_lantern(transform, row + 13, row[22], row[23], result + 1);
        if (fwrite(result, sizeof(result), 1, out) != 1) return 9;
        ++count;
    }
    fprintf(stderr, "original Lantern Nani numeric block executed %u cases\n", count);
    fclose(in); fclose(out); munmap(raw, st.st_size); close(fd);
    return 0;
}
