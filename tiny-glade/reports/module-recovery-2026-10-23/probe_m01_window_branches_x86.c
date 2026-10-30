/* 原完整XZ投影/seed函数、原窗cloth判别和首个随机数block。只桥接输出，不实现原公式。 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

typedef uint64_t (__attribute__((ms_abi)) *seed_fn)(uint32_t);
typedef void (__attribute__((ms_abi)) *quat_fn)(void *, const void *);
extern uint32_t native_cloth(const void *);
extern uint32_t native_gate(uint64_t);
extern void cloth_true(void), cloth_false(void), gate_capture(void);
__asm__(
".text\n.globl _native_cloth\n_native_cloth:\n"
"pushq %rbp\n subq $0x400,%rsp\n leaq 0x80(%rsp),%rbp\n"
"leaq 72(%rdi),%rax\n movq %rax,0x298(%rbp)\n"
"movabsq $0x1407ee2e4,%r11\n jmpq *%r11\n"
".globl _cloth_true\n_cloth_true:\nmovl $1,%eax\n jmp Lcloth_end\n"
".globl _cloth_false\n_cloth_false:\nxorl %eax,%eax\n"
"Lcloth_end:\n addq $0x400,%rsp\n popq %rbp\n retq\n"
".globl _native_gate\n_native_gate:\n"
"pushq %rsi\n movq %rdi,%rsi\n movabsq $0x1407ee3a5,%r11\n jmpq *%r11\n"
".globl _gate_capture\n_gate_capture:\npopq %rsi\n retq\n");

static void jump(uintptr_t address, const void *target) {
    uint8_t bridge[13] = {0x49,0xbb,0,0,0,0,0,0,0,0,0x41,0xff,0xe3};
    uintptr_t pointer = (uintptr_t)target;
    memcpy(bridge+2, &pointer, 8); memcpy((void *)address, bridge, 13);
}
int main(int argc, char **argv) {
    if (argc != 4) return 2;
    int fd=open(argv[1], O_RDONLY); struct stat st;
    if (fd<0 || fstat(fd,&st)) return 3;
    uint8_t *raw=mmap(NULL,st.st_size,PROT_READ,MAP_PRIVATE,fd,0);
    if(raw==MAP_FAILED) return 4;
    uint32_t nt,image_size,header_size;uint16_t n,opt;
    memcpy(&nt,raw+0x3c,4);memcpy(&n,raw+nt+6,2);memcpy(&opt,raw+nt+20,2);
    memcpy(&image_size,raw+nt+80,4);memcpy(&header_size,raw+nt+84,4);
    void *image=mmap((void *)0x140000000ULL,image_size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_ANON|MAP_FIXED,-1,0);
    if(image==MAP_FAILED)return 5;
    memcpy(image,raw,header_size);
    for(unsigned i=0;i<n;++i){uint32_t rva,size,off;uint8_t *s=raw+nt+24+opt+i*40;
        memcpy(&rva,s+12,4);memcpy(&size,s+16,4);memcpy(&off,s+20,4);
        if((uint64_t)off+size>(uint64_t)st.st_size || (uint64_t)rva+size>image_size)return 6;
        memcpy((uint8_t *)image+rva,raw+off,size);
    }
    // 原两个失败条件仍执行比较/Jcc，只将其业务出口改为输出0。
    for(unsigned i=0;i<2;++i){uintptr_t at=i?0x1407ee306ULL:0x1407ee2f5ULL;
        int32_t relative=(int32_t)(0x1407ee6ceULL-(at+6));memcpy((void *)(at+2),&relative,4);}
    jump(0x1407ee30cULL,cloth_true);jump(0x1407ee6ceULL,cloth_false);
    jump(0x1407ee3cdULL,gate_capture);
    FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 7;
    uint32_t row[6],result[9];unsigned count=0;
    seed_fn seed=(seed_fn)(uintptr_t)0x140c96240ULL;
    quat_fn quat=(quat_fn)(uintptr_t)0x140c911d0ULL;
    while(fread(row,sizeof(row),1,in)==1){
        _Alignas(16) uint8_t info[72]={0};memcpy(info+0x2c,row+1,4);info[0x44]=(uint8_t)row[2];
        result[0]=row[0];result[1]=native_cloth(info);
        uint64_t mixed=seed(row[1]);memcpy(result+7,&mixed,8);
        result[2]=native_gate(mixed);
        _Alignas(16) uint32_t direction[4]={row[3],row[4],row[5],0},rotation[4];
        quat(rotation,direction);memcpy(result+3,rotation,16);
        if(fwrite(result,sizeof(result),1,out)!=1)return 8;++count;
    }
    fprintf(stderr,"original window branch helpers executed %u cases\n",count);
    fclose(in);fclose(out);munmap(raw,st.st_size);close(fd);return 0;
}
