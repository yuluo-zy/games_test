#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
typedef struct { void *frame; const float *points; const float *us; uint64_t n; float length; } Args;
extern void run_original(Args *);
__asm__(
".text\n.globl _run_original\n_run_original:\n"
"pushq %rbp\n pushq %rbx\n pushq %r12\n pushq %r13\n pushq %r14\n pushq %r15\n"
"movq %rdi,%rax\n movq 8(%rax),%rsi\n movq 16(%rax),%rdi\n movq 24(%rax),%rbx\n decq %rbx\n movq %rbx,%r14\n movss 32(%rax),%xmm9\n movq (%rax),%rbp\n"
"subq $40,%rsp\n movabsq $0x140acd98f,%rax\n callq *%rax\n addq $40,%rsp\n"
"popq %r15\n popq %r14\n popq %r13\n popq %r12\n popq %rbx\n popq %rbp\n retq\n");
int main(void){
 int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 const uint8_t signature[]={0x45,0x0f,0x57,0xd2};if(memcmp((void*)0x140acd98f,signature,4))return 4;
 /* 转出完整 UI 主体，原段循环及 circle-ray/lerp 不桥接。 */
 *(uint8_t*)0x140acde09=0xc3;
 float points[]={0,0, 2,0, 4,0, 6,0},us[]={0,1.f/3,2.f/3,1};
 float circles[][3]={{3,0,1},{3,3,1},{0,0,1},{3,0,4},{2,0,0.15f}};
 for(unsigned i=0;i<sizeof circles/sizeof circles[0];i++){
  uint8_t frame[2048]={0}; float output[200]={0};void *frameptr=frame+256;
  float circ[4]={circles[i][0],circles[i][1],circles[i][2],0};memcpy(frame+256-72,circ,16);
  uint64_t count=4,cap=100,ptr=(uint64_t)output;memcpy(frame+256+1080,&count,8);memcpy(frame+256+1096,&cap,8);memcpy(frame+256+1104,&ptr,8);
  Args args={frameptr,points,us,4,6};run_original(&args);uint64_t n;memcpy(&n,frame+256+1112,8);if(n>100)return 5;
  printf("case%u n%llu",i,n);for(uint64_t j=0;j<n*2;j++){uint32_t bits;memcpy(&bits,output+j,4);printf(" %08x",bits);}puts("");
 }
}
