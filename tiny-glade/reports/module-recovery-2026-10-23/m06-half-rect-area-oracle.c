#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
typedef struct { uint64_t cap; void *ptr; uint64_t len; } Vec;
__attribute__((ms_abi)) static void grow(Vec *v,void *location){(void)location;v->cap=v->cap?v->cap*2:4;v->ptr=realloc(v->cap==4?NULL:v->ptr,v->cap*16);if(!v->ptr)abort();}
typedef void (__attribute__((ms_abi)) *F)(Vec *,void *,uint64_t);
int main(void){int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 uint8_t *fn=(void*)0x141fd18a0,*target=(void*)0x14068d3d0; uint8_t signature[]={0x55,0x41,0x57,0x41,0x56};if(memcmp(fn,signature,5))return 4;
 uintptr_t v=(uintptr_t)grow;target[0]=0x48;target[1]=0xb8;memcpy(target+2,&v,8);target[10]=0xff;target[11]=0xe0;
 struct {float axis[2],center[2],size[2];} rect={{1,0},{0,0},{4,2}};float len=12,levels[]={0,2,5};Vec lv={3,levels,3};
 struct {uint64_t tag,a,b;} gable={1,0,2};uint8_t wall[672]={0};wall[264]=1;float maximum=3;memcpy(wall+268,&maximum,4);
 struct {void *rect,*len,*levels,*gable,*wall;} env={&rect,&len,&lv,&gable,wall};
 for(int i=0;i<4;i++){Vec out={0,(void*)4,0};((F)fn)(&out,&env,i);printf("side%d n%llu\n",i,out.len);for(int j=0;j<out.len;j++){float *r=out.ptr;for(int k=0;k<4;k++){uint32_t bits;memcpy(&bits,&r[j*4+k],4);printf("%08x%c",bits,k==3?'\n':' ');}}if(out.cap)free(out.ptr);}
 return 0;}
