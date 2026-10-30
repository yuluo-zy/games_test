#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
struct A {void *frame; float *out;};
extern void probe(struct A*);
__asm__(".text\n.globl _probe\n_probe:\n pushq %rbp\n movq %rdi,%rax\n movq (%rax),%rbp\n movq 8(%rax),%r10\n xorps %xmm13,%xmm13\n movl $0x3f000000,%eax\n movd %eax,%xmm14\n movl $0x3f800000,%eax\n movd %eax,%xmm12\n subq $40,%rsp\n movabsq $0x14093010a,%rax\n callq *%rax\n addq $40,%rsp\n movss %xmm6,(%r10)\n movss %xmm7,4(%r10)\n popq %rbp\n retq\n");
int main(void){int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
unsigned char sig[]={0xf3,0x0f,0x10,0x85,0x50,0x03,0,0};if(memcmp((void*)0x14093010a,sig,sizeof sig))return 4;*(unsigned char*)0x14093016c=0xc3;
uint32_t vals[]={0,0x3e4ccccd,0x3f800000,0x40000000,0x40400000,0x7fc12345};for(unsigned i=0;i<6;i++){char frame[2048]={0};float out[2];memcpy(frame+0x350,&vals[i],4);struct A a={frame,out};probe(&a);uint32_t lo,hi;memcpy(&lo,out,4);memcpy(&hi,out+1,4);printf("%08x %08x %08x\n",vals[i],lo,hi);}return 0;}
