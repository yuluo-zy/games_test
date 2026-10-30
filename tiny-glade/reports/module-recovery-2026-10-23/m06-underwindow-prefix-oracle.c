#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
static float result_points[12];
static uint32_t reached;
__attribute__((naked,used)) static void capture(void) {
 __asm__ volatile("movl $1, _reached(%rip)\n"
 "movups 192(%rsp), %xmm0\n movups %xmm0, _result_points(%rip)\n"
 "movups 208(%rsp), %xmm0\n movups %xmm0, _result_points+16(%rip)\n"
 "movups 224(%rsp), %xmm0\n movups %xmm0, _result_points+32(%rip)\n"
 "movaps 336(%rsp), %xmm6\n movaps 352(%rsp), %xmm7\n"
 "movaps 368(%rsp), %xmm8\n movaps 384(%rsp), %xmm9\n"
 "movaps 400(%rsp), %xmm10\n movaps 416(%rsp), %xmm11\n"
 "movaps 432(%rsp), %xmm12\n movaps 448(%rsp), %xmm13\n"
 "movaps 464(%rsp), %xmm14\n movaps 480(%rsp), %xmm15\n"
 "addq $504, %rsp\n popq %rbx\n popq %rbp\n popq %rdi\n popq %rsi\n"
 "popq %r12\n popq %r13\n popq %r14\n popq %r15\n retq\n");
}
typedef void (__attribute__((ms_abi)) *F)(void*,void*,float,float,void*,float,void*,void*,void*,void*,int,float);
int main(void){
 int fd=open("/private/tmp/m6-timber-rne-image",0); struct stat st; if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 uint8_t *start=(void*)0x141fd1c30,*patch=(void*)0x141fd1e3f;
 uint8_t first[]={0x41,0x57,0x41,0x56}, at[]={0x0f,0xb6,0x1a};
 if(memcmp(start,first,4)||memcmp(patch,at,3))return 4;
 uintptr_t target=(uintptr_t)capture;patch[0]=0x48;patch[1]=0xb8;memcpy(patch+2,&target,8);patch[10]=0xff;patch[11]=0xe0;
 F f=(F)start; struct {float a[4];uint8_t pad,subtype,extra[2];} w={0};float base[4]={0};uint8_t flag=0;
 struct C {uint8_t tag;float top,bottom,left,right,floor;} cases[]={
 {0,1,0,1,3,0},{0,1,0,1,3,-1},{0,0.5,0,1,3,0},{0,2,0,1,3,0},
 {0x16,1,0,1,3,0},{0,1.375f,-0.125f,-2.75f,6.625f,-0.125f}};
 for(unsigned i=0;i<sizeof(cases)/sizeof(cases[0]);i++){
  struct C c=cases[i];w.subtype=c.tag;w.a[1]=c.top;base[1]=c.bottom;flag=0;reached=0;memset(result_points,0,sizeof result_points);
  f(&w,base,c.left,c.right,&flag,12,0,0,0,0,0,c.floor);
  printf("%u %u %u",i,reached,flag);for(int j=0;j<12;j++){uint32_t bits;memcpy(&bits,&result_points[j],4);printf(" %08x",bits);}puts("");
 }
 return 0;
}
