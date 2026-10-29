#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
typedef float (__attribute__((ms_abi)) *fun)(void*);
int main(){ int f=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(f<0||fstat(f,&st))return 2;void *p=mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,f,0);if(p==MAP_FAILED)return 3;unsigned char code[]={0x48,0x83,0xec,0x28,0x48,0x89,0xc8};if(memcmp((void*)0x1417d3f10,code,7))return 4;fun fn=(fun)0x1417d3f10;struct {uint32_t tag;float x[6];} v={0};for(int shape=0;shape<2;shape++)for(int i=0;i<5;i++){memset(&v,0,sizeof(v));v.tag=shape;float s=(float[]){0,0.9,2.25,8,15}[i];if(shape){v.x[4]=s;v.x[5]=s;}else v.x[2]=s;float r=fn(&v);uint32_t bits;memcpy(&bits,&r,4);printf("%d %g %08x\n",shape,s,bits);}return 0;}
