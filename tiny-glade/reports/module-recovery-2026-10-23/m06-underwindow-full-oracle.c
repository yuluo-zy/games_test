#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <math.h>
static uint32_t reached;
static uint8_t records[3][64];
__attribute__((ms_abi)) static float native_fmod(float a,float b){return fmodf(a,b);}
__attribute__((naked,used)) static void capture(void) {
 __asm__ volatile("pushq %r10\n pushq %r11\n"
 "movl _reached(%rip), %r10d\n shlq $6, %r10\n leaq _records(%rip), %r11\n addq %r10, %r11\n"
 "movq 272(%rsp), %r10\n movq %r10, 0(%r11)\n"
 "movq 280(%rsp), %r10\n movq %r10, 8(%r11)\n"
 "movq 288(%rsp), %r10\n movq %r10, 16(%r11)\n"
 "movq 296(%rsp), %r10\n movq %r10, 24(%r11)\n"
 "movq 304(%rsp), %r10\n movq %r10, 32(%r11)\n"
 "movq 312(%rsp), %r10\n movq %r10, 40(%r11)\n"
 "movq 320(%rsp), %r10\n movq %r10, 48(%r11)\n"
 "movq 328(%rsp), %r10\n movq %r10, 56(%r11)\n"
 "incl _reached(%rip)\n popq %r11\n popq %r10\n xorl %eax,%eax\n"
 "movabsq $0x141fd2308,%r10\n jmp *%r10\n");
}
typedef void (__attribute__((ms_abi)) *F)(void*,void*,float,float,void*,float,void*,void*,void*,void*,int,float);
int main(void){
 int fd=open("/private/tmp/m6-timber-rne-image",0); struct stat st; if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 uint8_t *start=(void*)0x141fd1c30,*patch=(void*)0x141fd22f8,*fmod_thunk=(void*)0x142923f10;
 uint8_t first[]={0x41,0x57,0x41,0x56};if(memcmp(start,first,4))return 4;
 uintptr_t target=(uintptr_t)capture;patch[0]=0x48;patch[1]=0xb8;memcpy(patch+2,&target,8);patch[10]=0xff;patch[11]=0xe0;
 target=(uintptr_t)native_fmod;fmod_thunk[0]=0x48;fmod_thunk[1]=0xb8;memcpy(fmod_thunk+2,&target,8);fmod_thunk[10]=0xff;fmod_thunk[11]=0xe0;
 F f=(F)start; struct {float a[4];uint8_t pad,subtype,extra[2];} w={0};float base[4]={0};uint8_t flag=0,failed=0;
 struct {uint64_t cap;float (*points)[2];uint64_t len;uint64_t ucap;float *u;uint64_t ulen;float length;} curve;
 float points[2][2]={{0,0},{4,0}},u[2]={0,1};curve.cap=2;curve.points=points;curve.len=2;curve.ucap=2;curve.u=u;curve.ulen=2;curve.length=4;
 struct {uint32_t tag;float axis[2],center[2],size[2];} shape={1,{1,0},{0,0},{4,2}};
 struct {uintptr_t writer;uint8_t *failed;} closure={1,&failed};
 w.a[1]=1;f(&w,base,1,3,&flag,4,&curve,&shape,&closure,(void*)17,4,0);
 printf("%u %u %u\n",reached,flag,failed);
 for(int i=0;i<reached;i++){for(int j=0;j<16;j++){uint32_t bits;memcpy(&bits,records[i]+j*4,4);printf("%08x%c",bits,j==15?'\n':' ');}}
 return 0;
}
