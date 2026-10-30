#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
typedef void (__attribute__((ms_abi)) *rayfn)(void *, const void *, float, float, float, float);
int main(int argc,char **argv){
 if(argc!=4)return 2;int fd=open(argv[1],O_RDONLY);struct stat st;if(fd<0||fstat(fd,&st))return 3;
 void *p=mmap((void*)0x140000000ULL,st.st_size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_FIXED,fd,0);if(p==MAP_FAILED)return 4;
 const unsigned char signature[]={0x48,0x83,0xec,0x18,0x0f,0x29,0x34,0x24};if(memcmp((void*)0x140c97980ULL,signature,sizeof(signature)))return 5;
 FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 6;
 uint32_t r[7],o[3];while(fread(r,sizeof(r),1,in)==1){float v[4];memcpy(v,r+3,16);memset(o,0,sizeof(o));((rayfn)(uintptr_t)0x140c97980ULL)(o,r,v[0],v[1],v[2],v[3]);if(fwrite(o,sizeof(o),1,out)!=1)return 7;}return 0;
}
