/* 原两种砖实例完整数值块，半精度助手继续执行原RNE；只注入输入和截取输出。 */
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

extern void native_bricks(uint32_t, const void *, const void *, const void *, void *);
extern void solid_capture(void), gothic_capture(void);
__asm__(
".text\n.globl _native_bricks\n_native_bricks:\n"
"pushq %rbp\n pushq %r12\n pushq %r14\n pushq %r15\n subq $0x188,%rsp\n"
"leaq 0x80(%rsp),%rbp\n movq %r8,0xf0(%rbp)\n movq %rsi,%rax\n testl %edi,%edi\n jnz Lgothic\n"
"movq %rdx,%rdi\n movabsq $0x1420767df,%r11\n jmpq *%r11\n"
"Lgothic:\n movq %rdx,%r9\n movq %rcx,%rdx\n movq %r9,%rcx\n movabsq $0x14207756e,%r11\n jmpq *%r11\n"
".globl _solid_capture\n_solid_capture:\n"
"movq 0xf0(%rbp),%r11\n"
"movaps -0x40(%rbp),%xmm0\n movups %xmm0,(%r11)\n"
"movaps -0x30(%rbp),%xmm0\n movups %xmm0,0x10(%r11)\n"
"movaps -0x20(%rbp),%xmm0\n movups %xmm0,0x20(%r11)\n"
"movaps -0x10(%rbp),%xmm0\n movups %xmm0,0x30(%r11)\n"
"movss %xmm6,0x40(%r11)\n jmp Lbricks_end\n"
".globl _gothic_capture\n_gothic_capture:\n"
"movq 0xf0(%rbp),%r11\n"
"movaps -0x40(%rbp),%xmm0\n movups %xmm0,(%r11)\n"
"movaps -0x30(%rbp),%xmm0\n movups %xmm0,0x10(%r11)\n"
"movaps -0x20(%rbp),%xmm0\n movups %xmm0,0x20(%r11)\n"
"movaps -0x10(%rbp),%xmm0\n movups %xmm0,0x30(%r11)\n"
"movaps (%rbp),%xmm0\n movups %xmm0,0x40(%r11)\n"
"movaps 0x10(%rbp),%xmm0\n movups %xmm0,0x50(%r11)\n"
"movaps 0x20(%rbp),%xmm0\n movups %xmm0,0x60(%r11)\n"
"movss %xmm6,0x70(%r11)\n"
"Lbricks_end:\naddq $0x188,%rsp\n popq %r15\n popq %r14\n popq %r12\n popq %rbp\n retq\n");

static void jump(uintptr_t address,const void *target) {
    uint8_t code[13]={0x49,0xbb,0,0,0,0,0,0,0,0,0x41,0xff,0xe3};
    uintptr_t p=(uintptr_t)target;memcpy(code+2,&p,8);memcpy((void *)address,code,13);
}
int main(int argc,char **argv) {
    if(argc!=4)return 2;int fd=open(argv[1],O_RDONLY);struct stat st;
    if(fd<0||fstat(fd,&st))return 3;
    uint8_t *raw=mmap(NULL,st.st_size,PROT_READ,MAP_PRIVATE,fd,0);if(raw==MAP_FAILED)return 4;
    uint32_t nt,size,headers;uint16_t n,opt;
    memcpy(&nt,raw+0x3c,4);memcpy(&n,raw+nt+6,2);memcpy(&opt,raw+nt+20,2);
    memcpy(&size,raw+nt+80,4);memcpy(&headers,raw+nt+84,4);
    void *image=mmap((void *)0x140000000ULL,size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_ANON|MAP_FIXED,-1,0);
    if(image==MAP_FAILED)return 5;memcpy(image,raw,headers);
    for(unsigned i=0;i<n;++i){uint32_t rva,len,off;uint8_t *s=raw+nt+24+opt+i*40;
        memcpy(&rva,s+12,4);memcpy(&len,s+16,4);memcpy(&off,s+20,4);
        if((uint64_t)off+len>(uint64_t)st.st_size||(uint64_t)rva+len>size)return 6;
        memcpy((uint8_t *)image+rva,raw+off,len);
    }
    jump(0x142076980ULL,solid_capture);jump(0x142077820ULL,gothic_capture);
    FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 7;
    uint32_t row[35],result[31];unsigned count=0;
    while(fread(row,sizeof(row),1,in)==1){
        _Alignas(16) uint32_t transform[12];memcpy(transform,row+2,sizeof(transform));
        memset(result,0,sizeof(result));result[0]=row[0];result[1]=row[1];
        native_bricks(row[1],transform,row+14,row+20,result+2);
        if(!row[1]){result[30]=result[18];result[18]=0;}
        if(fwrite(result,sizeof(result),1,out)!=1)return 8;++count;
    }
    fprintf(stderr,"original two bricks numeric blocks executed %u cases\n",count);
    fclose(in);fclose(out);munmap(raw,st.st_size);close(fd);return 0;
}
