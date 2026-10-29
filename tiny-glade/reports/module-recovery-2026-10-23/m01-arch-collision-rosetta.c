/* 原RNE完整construct_arch_geo直接执行，预留真实输出容量，原tuple_windows/Triangle/normal/SSE均原码。 */
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
 FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 8;setvbuf(out,NULL,_IONBF,0);unsigned rows=0;
 uint32_t header[4];
 while(fread(header,sizeof(header),1,in)==1){
  uint32_t nh=header[1],np=header[2],seed=header[3];
  void *heights=malloc(nh*4+1),*positions=malloc(np*12+1);
  if(nh&&fread(heights,nh*4,1,in)!=1)return 9;
  if(np&&fread(positions,np*12,1,in)!=1)return 10;
  void *normalbuf=calloc(2048,36),*tribuf=calloc(2048,36);
  for(unsigned i=0;i<seed*9;++i){((uint32_t*)normalbuf)[i]=0x3f800000+i;((uint32_t*)tribuf)[i]=0xbf800000+i;}
  uint64_t segment[13]={nh,(uintptr_t)heights,nh,np,(uintptr_t)positions,np,0,4,0,0,0,0,0};
  uint64_t normals[3]={2048,(uintptr_t)normalbuf,seed},triangles[3]={2048,(uintptr_t)tribuf,seed};current_id=header[0];allocation_count=0;
  if(sigsetjmp(panic_return,1))return 11;
  ((geo_fn)(uintptr_t)0x1408d7ed0ULL)(segment,normals,triangles);
  uint32_t status[3]={header[0],(uint32_t)normals[2],(uint32_t)triangles[2]};fwrite(status,sizeof(status),1,out);
  fwrite(normalbuf,36,status[1],out);fwrite(tribuf,36,status[2],out);
  if(allocation_count)return 12;free(normalbuf);free(tribuf);free(heights);free(positions);++rows;
 }
 fclose(in);fclose(out);close(fd);fprintf(stderr,"original construct_arch_geo x86_64: %u cases\n",rows);return 0;
}
