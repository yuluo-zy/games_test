#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
typedef struct {void *frame;float length,threshold;} Args;
extern void run_filter(Args *);
__asm__(".text\n.globl _run_filter\n_run_filter:\n"
"pushq %rbp\n pushq %rbx\n pushq %r12\n pushq %r13\n pushq %r14\n pushq %r15\n"
"movq %rdi,%rax\n movss 8(%rax),%xmm9\n movss 12(%rax),%xmm15\n movq (%rax),%rbp\n"
"subq $40,%rsp\n movabsq $0x140acdf66,%rax\n callq *%rax\n addq $40,%rsp\n"
"popq %r15\n popq %r14\n popq %r13\n popq %r12\n popq %rbx\n popq %rbp\n retq\n");
int main(void){int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 const uint8_t signature[]={0x48,0x8b,0x85,0x58,0x04,0x00,0x00};if(memcmp((void*)0x140acdf66,signature,7))return 4;
 *(uint8_t*)0x140ace406=0xc3;
 uint32_t samples[][6]={{0,0x3ccccccc,0x3e800000,0x3f000000,0x3f400000,0x3f800000},
 {0,0x3d088889,0x3e800000,0x3f000000,0x3f400000,0x3f800000},
 {0,0x3e800000,0x3f000000,0x3f010000,0x3f400000,0x3f800000},
 {0,0x3e800000,0x3f000000,0x3f400000,0x3f700000,0x3f710000},
 {0,0x7fc12345,0x3e800000,0x3f000000,0x3f400000,0x3f800000}};
 for(int i=0;i<5;i++){uint8_t frame[2048]={0};uint32_t v[6];memcpy(v,samples[i],sizeof v);uint64_t ptr=(uint64_t)v,n=3;memcpy(frame+256+1104,&ptr,8);memcpy(frame+256+1112,&n,8);Args a={frame+256,6.f,0.2f};run_filter(&a);memcpy(&n,frame+256+1112,8);if(n>3)return 5;printf("case%d n%llu",i,n);for(int j=0;j<n*2;j++)printf(" %08x",v[j]);puts("");}
}
