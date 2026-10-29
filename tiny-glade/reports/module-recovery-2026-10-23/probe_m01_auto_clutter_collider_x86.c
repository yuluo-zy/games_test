/* 原自动杂物包围盒数值块1408d0b93..0bfb；只注入96B字段并截取六个实际输出。 */
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
extern void native_bounds(const void*,void*),bounds_capture(void);
__asm__(
".text\n.globl _native_bounds\n_native_bounds:\n"
"pushq %rbp\n subq $0x300,%rsp\n leaq 0x80(%rsp),%rbp\n movq %rsi,0x240(%rbp)\n"
"movq %rdi,%rcx\n xorps %xmm8,%xmm8\n movabsq $0x3f0000003f000000,%rax\n movq %rax,%xmm9\n"
"movabsq $0x1408d0b93,%rax\n jmpq *%rax\n"
".globl _bounds_capture\n_bounds_capture:\n"
"movq 0x240(%rbp),%rsi\n movups 0x1b0(%rbp),%xmm0\n movups %xmm0,(%rsi)\n"
"movq 0x1c0(%rbp),%rax\n movq %rax,0x10(%rsi)\n addq $0x300,%rsp\n popq %rbp\n retq\n");
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
    jump(0x1408d0bfbULL,bounds_capture);
    FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 7;
    uint32_t input[5],result[7],component[24];unsigned count=0;
    while(fread(input,sizeof(input),1,in)==1){
        memset(component,0,sizeof(component));memcpy(component+20,input+1,16);result[0]=input[0];
        native_bounds(component,result+1);
        if(fwrite(result,sizeof(result),1,out)!=1)return 8;++count;
    }
    fprintf(stderr,"original automatic clutter dimensions/local center executed %u cases\n",count);
    fclose(in);fclose(out);munmap(raw,st.st_size);close(fd);return 0;
}
