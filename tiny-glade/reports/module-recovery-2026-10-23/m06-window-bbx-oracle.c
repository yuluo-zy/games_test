#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
typedef void (__attribute__((ms_abi)) *F)(void*,void*,uint32_t,uint32_t);
int main(void){
 int fd=open("/private/tmp/m6-timber-rne-image",0);struct stat st;if(fd<0||fstat(fd,&st))return 2;
 if(mmap((void*)0x140000000,st.st_size,7,MAP_PRIVATE|MAP_FIXED,fd,0)==MAP_FAILED)return 3;
 uint8_t signature[]={0x56,0x57,0x55,0x53,0x48,0x83,0xec,0x68};if(memcmp((void*)0x141fdc7b0,signature,8))return 4;
 /* 橋接 hash_one 為零；單槽表、原查鍵與32字節搬運仍由原指令執行。 */
 uint8_t *hash=(void*)0x1411f7dc0;hash[0]=0x31;hash[1]=0xc0;hash[2]=0xc3;
 uint8_t buf[256]={0};uint8_t *ctrl=buf+128; memset(ctrl,0xff,16);ctrl[0]=0;
 uint64_t table[6]={(uint64_t)ctrl,15,0,1,0,0};uint64_t view[2]={0,(uint64_t)table};
 for(int i=0;i<4;i++) {uint8_t subtype=i==3?26:(uint8_t)(i+3),rank=(uint8_t)(i+1);ctrl[-72]=subtype;ctrl[-71]=rank;
  uint32_t expected[8],out[8];for(int j=0;j<8;j++)expected[j]=0x3f000000u+(i*8+j)*0x10001u;memcpy(ctrl-32,expected,32);memset(out,0,sizeof out);
  ((F)0x141fdc7b0)(out,view,subtype,i==3?99:rank);
  printf("case %d:",i);for(int j=0;j<8;j++)printf(" %08x",out[j]);printf(" %s\n",memcmp(out,expected,32)?"FAIL":"PASS");if(memcmp(out,expected,32))return 5;
 }
 return 0;
}
