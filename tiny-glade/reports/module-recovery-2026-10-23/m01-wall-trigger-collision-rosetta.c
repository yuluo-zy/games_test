/* 原RNE完整get_collision_geo直接执行；只桥接分配和panic，Triangle/inverse_lerp/SSE均原码。 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#include <signal.h>
#include <sys/ucontext.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
typedef void (__attribute__((ms_abi)) *geo_fn)(void*,const void*,const void*);
static sigjmp_buf panic_return;
static unsigned current_id,allocation_count;
static void *allocations[1024];
static void __attribute__((ms_abi,noreturn)) panic_bridge(void){siglongjmp(panic_return,1);}
static void *__attribute__((ms_abi)) alloc_bridge(uint64_t size,uint64_t alignment){
 void *p=NULL;if(alignment<sizeof(void*))alignment=sizeof(void*);
 if(posix_memalign(&p,alignment,size))return NULL;
 if(allocation_count>=1024)_exit(21);allocations[allocation_count++]=p;return p;
}
static void fault(int signal,siginfo_t *info,void *context){
 ucontext_t *cpu=context;fprintf(stderr,"unexpected signal%d case%u rip%llx address%p\n",signal,current_id,cpu->uc_mcontext->__ss.__rip,info->si_addr);_exit(128+signal);
}
static void patch(uintptr_t at,void *callback){uint8_t b[12]={0x48,0xb8,0,0,0,0,0,0,0,0,0xff,0xe0};memcpy(b+2,&callback,8);memcpy((void*)at,b,12);}
int main(int argc,char**argv){
 if(argc!=4)return 2;int fd=open(argv[1],O_RDONLY);struct stat st;if(fd<0||fstat(fd,&st))return 3;
 uint8_t *raw=mmap(NULL,st.st_size,PROT_READ,MAP_PRIVATE,fd,0);if(raw==MAP_FAILED)return 4;
 uint32_t nt;memcpy(&nt,raw+60,4);uint16_t count,optional;memcpy(&count,raw+nt+6,2);memcpy(&optional,raw+nt+20,2);
 uint32_t image_size,header_size;memcpy(&image_size,raw+nt+24+56,4);memcpy(&header_size,raw+nt+24+60,4);
 void *image=mmap((void*)0x140000000ULL,image_size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_ANON|MAP_FIXED,-1,0);if(image==MAP_FAILED)return 5;memcpy(image,raw,header_size);
 for(unsigned i=0;i<count;++i){uint8_t*s=raw+nt+24+optional+i*40;uint32_t rva,size,offset;memcpy(&rva,s+12,4);memcpy(&size,s+16,4);memcpy(&offset,s+20,4);if((uint64_t)offset+size>(uint64_t)st.st_size||(uint64_t)rva+size>image_size)return 7;memcpy((uint8_t*)image+rva,raw+offset,size);}
 munmap(raw,st.st_size);patch(0x140613c10ULL,alloc_bridge);
 uintptr_t panics[]={0x1428d9430ULL,0x1428d9390ULL,0x140d8b480ULL,0x1428d8fe3ULL};for(unsigned i=0;i<4;++i)patch(panics[i],panic_bridge);
 struct sigaction a={0};a.sa_sigaction=fault;a.sa_flags=SA_SIGINFO;sigaction(SIGSEGV,&a,NULL);sigaction(SIGBUS,&a,NULL);
 FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 8;setvbuf(out,NULL,_IONBF,0);unsigned rows=0,failures=0;
 uint32_t header[3];uint8_t params[32];
 while(fread(header,sizeof(header),1,in)==1){
  if(fread(params,sizeof(params),1,in)!=1)return 9;size_t bytes=(size_t)header[1]*120;void *quad=malloc(bytes?bytes:1);if(bytes&&fread(quad,bytes,1,in)!=1)return 10;
  uint64_t volume[13]={header[1],(uintptr_t)quad,header[1]},geo[9]={0};((uint8_t*)volume)[96]=header[2];current_id=header[0];allocation_count=0;
  uint32_t status[5]={header[0],0,0,0,0};if(sigsetjmp(panic_return,1)){status[1]=1;++failures;}else{((geo_fn)(uintptr_t)0x1408e8d70ULL)(geo,volume,params);for(unsigned i=0;i<3;++i)status[i+2]=(uint32_t)geo[i*3+2];}
  fwrite(status,sizeof(status),1,out);if(!status[1])for(unsigned i=0;i<3;++i)fwrite((void*)(uintptr_t)geo[i*3+1],i==2?24:36,status[i+2],out);
  for(unsigned i=0;i<allocation_count;++i)free(allocations[i]);free(quad);++rows;
 }
 fclose(in);fclose(out);close(fd);unsigned mxcsr;__asm__("stmxcsr %0":"=m"(mxcsr));fprintf(stderr,"original walltrigger x86_64: %u cases %u panics MXCSR%08x\n",rows,failures,mxcsr);return 0;
}
