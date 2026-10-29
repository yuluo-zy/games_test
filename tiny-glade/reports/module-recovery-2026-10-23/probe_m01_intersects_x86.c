/* 原RNE完整Rectangle2d::intersects及itertools迭代依赖直接执行；仅panic出口桥接。 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#include <signal.h>
#include <sys/ucontext.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
typedef uint8_t (__attribute__((ms_abi)) *intersects_fn)(const void *, const void *);
static sigjmp_buf panic_return;
static volatile unsigned current_id;
static void fault(int signal, siginfo_t *info, void *context) {
    ucontext_t *cpu = (ucontext_t *)context;
    fprintf(stderr, "unexpected signal=%d id=%u rip=%llx address=%p\n", signal,
        current_id, cpu->uc_mcontext->__ss.__rip, info->si_addr);
    _exit(128 + signal);
}
static void __attribute__((ms_abi,noreturn)) panic_bridge(void) { siglongjmp(panic_return, 1); }
int main(int argc, char **argv) {
    if (argc != 4) return 2;
    int fd = open(argv[1], O_RDONLY);
    struct stat st;
    if (fd < 0 || fstat(fd, &st)) return 3;
    uint8_t *raw = mmap(NULL, st.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (raw == MAP_FAILED) return 4;
    uint32_t nt; memcpy(&nt, raw + 0x3c, 4);
    uint16_t sections, optional;
    memcpy(&sections, raw + nt + 6, 2); memcpy(&optional, raw + nt + 20, 2);
    uint32_t image_size, header_size;
    memcpy(&image_size, raw + nt + 24 + 56, 4); memcpy(&header_size, raw + nt + 24 + 60, 4);
    void *image = mmap((void *)0x140000000ULL, image_size,
        PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANON | MAP_FIXED, -1, 0);
    if (image == MAP_FAILED) { perror("mmap original RNE"); return 4; }
    memcpy(image, raw, header_size);
    for (unsigned i = 0; i < sections; ++i) {
        uint32_t rva, size, offset;
        uint8_t *section = raw + nt + 24 + optional + i * 40;
        memcpy(&rva, section + 12, 4); memcpy(&size, section + 16, 4); memcpy(&offset, section + 20, 4);
        if ((uint64_t)offset + size > (uint64_t)st.st_size || (uint64_t)rva + size > image_size) return 7;
        memcpy((uint8_t *)image + rva, raw + offset, size);
    }
    munmap(raw, st.st_size);
    uint8_t trampoline[12] = {0x48,0xb8,0,0,0,0,0,0,0,0,0xff,0xe0};
    uintptr_t callback = (uintptr_t)&panic_bridge;
    memcpy(trampoline + 2, &callback, 8);
    memcpy((void *)0x1428d9430ULL, trampoline, sizeof(trampoline));
    FILE *in = fopen(argv[2], "rb"), *out = fopen(argv[3], "wb");
    if (!in || !out) return 5;
    setvbuf(out, NULL, _IONBF, 0);
    struct sigaction action = {0};
    action.sa_sigaction = fault;
    action.sa_flags = SA_SIGINFO;
    sigaction(SIGSEGV, &action, NULL);
    sigaction(SIGBUS, &action, NULL);
    intersects_fn intersects = (intersects_fn)(uintptr_t)0x140c9b990ULL;
    uint32_t row[13], result[2];
    unsigned count = 0, panics = 0;
    while (fread(row, sizeof(row), 1, in) == 1) {
        result[0] = row[0];
        current_id = row[0];
        if (sigsetjmp(panic_return, 1)) { result[1] = 2; ++panics; }
        else { result[1] = intersects(row + 1, row + 7) ? 1 : 0; }
        if (fwrite(result, sizeof(result), 1, out) != 1) return 6;
        ++count;
    }
    fclose(in); fclose(out); close(fd);
    fprintf(stderr, "original intersects x86_64 executed %u cases, %u native panics\n", count, panics);
    return 0;
}
