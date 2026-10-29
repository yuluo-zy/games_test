/* 独立重放旧180输入：原完整expand/rotate/intersects/circle与真实callee；仅panic出口桥接。 */
#include <math.h>
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
typedef void (__attribute__((ms_abi)) *intent_fn)(void*,void*,void*,void*,void*,uint32_t,void*,void*,uint64_t,void*,void*,uint64_t,uint64_t,uint8_t,void*,uint8_t,void*,void*);
static float captured_args[2];
static float __attribute__((ms_abi)) atan_bridge(float x,float y){captured_args[0]=x;captured_args[1]=y;return atan2f(x,y);}
static float __attribute__((ms_abi)) fmod_bridge(float x,float y){return fmodf(x,y);}
static void patch(uintptr_t address,uintptr_t fn){uint8_t p[12]={0x48,0xb8,0,0,0,0,0,0,0,0,0xff,0xe0};memcpy(p+2,&fn,8);memcpy((void*)address,p,12);}
static void qword(uint8_t *ptr,size_t offset,uint64_t value){memcpy(ptr+offset,&value,8);}
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
    patch(0x1428d9430ULL,(uintptr_t)&panic_bridge);
    patch(0x1428d9390ULL,(uintptr_t)&panic_bridge);
    patch(0x1428d9330ULL,(uintptr_t)&panic_bridge);
    patch(0x1428d92b0ULL,(uintptr_t)&panic_bridge);
    patch(0x142923e20ULL,(uintptr_t)&atan_bridge);
    patch(0x142923f10ULL,(uintptr_t)&fmod_bridge);
    FILE *in = fopen(argv[2], "rb"), *out = fopen(argv[3], "wb");
    if (!in || !out) return 5;
    setvbuf(out, NULL, _IONBF, 0);
    struct sigaction action = {0};
    action.sa_sigaction = fault;
    action.sa_flags = SA_SIGINFO;
    sigaction(SIGSEGV, &action, NULL);
    sigaction(SIGBUS, &action, NULL);
    intent_fn intent=(intent_fn)(uintptr_t)0x14127cf70ULL;
    uint32_t row[128],result[26];unsigned count=0,panics=0;
    while(fread(row,sizeof(row),1,in)==1){
        memset(result,0,sizeof(result));result[0]=row[0];current_id=row[0];
        uint8_t candidate[64] __attribute__((aligned(16)))={0};
        uint8_t native_output[96] __attribute__((aligned(16)))={0};
        uint8_t info[72]={0}, request[0x150]={0}, requests[24]={0};
        memcpy(candidate,row+6,16);candidate[0x10]=(uint8_t)row[1];
        memcpy(candidate+0x20,row+3,12);uint32_t ids[2]={7,12};
        memcpy(candidate+0x14,ids,8);info[0x44]=1;info[0x46]=(uint8_t)row[2];
        uint64_t n=row[10];
        qword(request,0,n);qword(request,8,(uintptr_t)(row+16));qword(request,0x10,n);
        qword(request,0x18,n);qword(request,0x20,(uintptr_t)(row+64));qword(request,0x28,n);memcpy(request+0x30,row+11,4);
        qword(request,0x50,n);qword(request,0x58,(uintptr_t)(row+56));qword(request,0x60,n);
        qword(request,0x68,n);qword(request,0x70,(uintptr_t)(row+32));qword(request,0x78,n);
        memcpy(request+0x128,ids,8);qword(requests,0,1);qword(requests,8,(uintptr_t)request);qword(requests,0x10,1);
        if(sigsetjmp(panic_return,1)){result[1]=1;++panics;}else{
            if(row[1]==16){
                float (__attribute__((ms_abi)) *from_quat)(void*)=(void*)0x140833a20ULL;
                float angle=from_quat(candidate);memcpy(native_output,captured_args,8);memcpy(native_output+8,&angle,4);
            }else intent(native_output,NULL,NULL,requests,candidate,177,info,(void*)4,0,NULL,NULL,2,0,0,NULL,0,NULL,NULL);
        }
        memcpy(result+2,native_output,96);
        if(fwrite(result,sizeof(result),1,out)!=1)return 6;++count;
    }
    fclose(in); fclose(out); close(fd);
    fprintf(stderr, "original full Intent x86_64 executed %u cases, %u native panics\n", count, panics);
    return 0;
}
