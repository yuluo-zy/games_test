#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
int main(void) {
 int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 uint8_t *start=(void*)0x140aceb8d;uint8_t signature[]={0xf3,0x0f,0x10,0x05};if(memcmp(start,signature,4))return 4;
 /* 原 140acebda 后转出屋顶分支，本有界探针只执行已算出的线性高度并返回。 */
 uint8_t replacement[]={0xf3,0x45,0x0f,0x58,0xc3,0x41,0x0f,0x28,0xc0,0xc3}; memcpy((void*)0x140acebda,replacement,sizeof replacement);
 float inputs[]={0.0f,0.5f,0.75f,1.0f,2.0f,3.0f,5.5f,6.0f,7.0f};
 for(unsigned i=0;i<sizeof(inputs)/sizeof(inputs[0]);i++){
  float out;void *fn=start;
  __asm__ volatile("movss %1, %%xmm10\n\tcall *%2\n\tmovss %%xmm0, %0" : "=m"(out) : "m"(inputs[i]), "r"(fn) : "rax","rcx","rdx","r8","r9","r10","r11","xmm0","xmm1","xmm2","xmm8","xmm10","xmm11","memory","cc");
  uint32_t a,b;memcpy(&a,&inputs[i],4);memcpy(&b,&out,4);printf("%08x %08x\n",a,b);
 }
 return 0;
}
