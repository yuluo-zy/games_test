/* 原Move数学有界重放：完整619B朝向、174B投影、原Quat/握点基本块；不代替主系统验收。 */
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

    // 两个纯原指令块以其原地址执行，仅把已选定基本块末端变成RET。
    *(uint8_t*)0x1407a66bbULL=0xc3;
    *(uint8_t*)0x1407a52caULL=0xc3;
    *(uint8_t*)0x1407a57caULL=0xc3;
    extern void probe_product(void*,void*);
    extern float probe_grip(void*,void*,void*,float);
    extern void probe_projection(void*,void*,void*,void*);
    uint32_t row[128],result[6];unsigned count=0,panics=0;
    while(fread(row,sizeof(row),1,in)==1){
        memset(result,0,sizeof(result));result[0]=row[0];current_id=row[0];
        uint8_t frame[0x800] __attribute__((aligned(16)))={0};
        uint8_t req[96]={0},info[72]={0};
        if(sigsetjmp(panic_return,1)){result[1]=1;++panics;}else{
            if(row[1]==1){
                void (__attribute__((ms_abi)) *rotation)(void*,void*)=(void*)0x140c91640ULL;
                rotation(frame+0x700,row+2);memcpy(result+2,frame+0x700,16);
            }else if(row[1]==2){
                memcpy(frame+0x6a0,row+2,16);memcpy(frame+0x720,row+6,16);probe_product(frame,frame+0x720);memcpy(result+2,frame+0x6a0,16);
            }else if(row[1]==3){
                memcpy(frame+0x400,row+2,256);probe_projection(row+66,frame+0x400,row+69,result+2);
            }else if(row[1]==4){
                req[0x5c]=(uint8_t)row[2];memcpy(req+0x34,row+6,4);
                info[0x22]=(uint8_t)row[3];memcpy(info+0x30,row+4,4);
                float base;memcpy(&base,row+5,4);float y=probe_grip(req,info,frame,base);memcpy(result+2,&y,4);
            }else return 9;
        }
        if(fwrite(result,sizeof(result),1,out)!=1)return 6;++count;
    }
    fclose(in);fclose(out);close(fd);
    fprintf(stderr,"original move math: %u cases, %u panics\n",count,panics);
    return 0;
}
