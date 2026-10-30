#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
int main(void){int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;unsigned char signature[]={0xf3,0x0f,0x5c,0xc8,0xf3,0x0f,0x59,0xca,0xf3,0x0f,0x58,0xc1,0xc3};if(memcmp((void*)0x140c904d0,signature,sizeof signature))return 4;uint32_t bits[]={0x3f1af100,0x3f29fa8b,0x3f71e191};float x[3],result;memcpy(x,bits,sizeof x);float (*fn)(float,float,float)=(void*)0x140c904d0;result=fn(x[0],x[1],x[2]);uint32_t word;memcpy(&word,&result,4);printf("%08x\n",word);return 0;}
