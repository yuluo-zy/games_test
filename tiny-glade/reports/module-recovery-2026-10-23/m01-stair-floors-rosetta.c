/* 原RNE gen_stair_floors入口至原Trimesh::builder边界，原复杂几何无桥；只桥allocator/panic并捕获原三数组。 */
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
typedef void (__attribute__((ms_abi)) *floor_fn)(void*,const void*,uint64_t,const void*,uint64_t,const void*,uint64_t,void*,void*,uint32_t,const void*,const void*,uint64_t);
static sigjmp_buf panic_return;
static unsigned current_id,allocation_count;
static void *allocations[4096];static uint32_t *output_indices,*output_positions,*output_normals;
static uint32_t output_counts[3];
static void __attribute__((ms_abi,noreturn)) panic_bridge(void){siglongjmp(panic_return,1);}
static void *__attribute__((ms_abi)) alloc_bridge(uint64_t size,uint64_t alignment){
 void*p=NULL;if(alignment<sizeof(void*))alignment=sizeof(void*);if(posix_memalign(&p,alignment,size))return NULL;
 if(allocation_count>=4096)_exit(21);allocations[allocation_count++]=p;return p;
}
static void __attribute__((ms_abi)) dealloc_bridge(void*p,uint64_t size,uint64_t alignment){(void)size;(void)alignment;for(unsigned i=0;i<allocation_count;++i)if(allocations[i]==p){free(p);allocations[i]=NULL;return;}_exit(22);}
static void *__attribute__((ms_abi)) realloc_bridge(void*p,uint64_t old_size,uint64_t alignment,uint64_t size){void*n=alloc_bridge(size,alignment);if(n){memcpy(n,p,old_size<size?old_size:size);dealloc_bridge(p,old_size,alignment);}return n;}
static uint64_t q(uintptr_t p){uint64_t v;memcpy(&v,(void*)p,8);return v;}
static uint32_t* capture(const void*data,uint64_t words){uint32_t*p=malloc(words*4+1);if(words)memcpy(p,data,words*4);return p;}
static void __attribute__((ms_abi,noreturn)) capture_bridge(void*out,const void*indices,uint64_t count,uintptr_t frame){
 (void)out;output_counts[0]=(uint32_t)count;output_counts[1]=(uint32_t)q(frame+0x280);output_counts[2]=(uint32_t)q(frame+0x260);
 if(count>1000000||output_counts[1]>1000000||output_counts[2]>1000000)_exit(23);
 output_indices=capture(indices,count);output_positions=capture((void*)q(frame+0x278),output_counts[1]*3);output_normals=capture((void*)q(frame+0x258),output_counts[2]*3);
 siglongjmp(panic_return,2);
}
static void fault(int signal,siginfo_t *info,void *context){ucontext_t*cpu=context;fprintf(stderr,"unexpected signal%d case%u rip%llx address%p\n",signal,current_id,cpu->uc_mcontext->__ss.__rip,info->si_addr);_exit(128+signal);}
static void patch(uintptr_t at,void*callback){uint8_t b[12]={0x48,0xb8,0,0,0,0,0,0,0,0,0xff,0xe0};memcpy(b+2,&callback,8);memcpy((void*)at,b,12);}
int main(int argc,char**argv){
 if(argc!=4)return 2;int fd=open(argv[1],O_RDONLY);struct stat st;if(fd<0||fstat(fd,&st))return 3;
 uint8_t *raw=mmap(NULL,st.st_size,PROT_READ,MAP_PRIVATE,fd,0);if(raw==MAP_FAILED)return 4;
 uint32_t nt;memcpy(&nt,raw+60,4);uint16_t count,optional;memcpy(&count,raw+nt+6,2);memcpy(&optional,raw+nt+20,2);
 uint32_t image_size,header_size;memcpy(&image_size,raw+nt+24+56,4);memcpy(&header_size,raw+nt+24+60,4);
 void *image=mmap((void*)0x140000000ULL,image_size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_ANON|MAP_FIXED,-1,0);if(image==MAP_FAILED)return 5;memcpy(image,raw,header_size);
 for(unsigned i=0;i<count;++i){uint8_t*s=raw+nt+24+optional+i*40;uint32_t rva,size,offset;memcpy(&rva,s+12,4);memcpy(&size,s+16,4);memcpy(&offset,s+20,4);if((uint64_t)offset+size>(uint64_t)st.st_size||(uint64_t)rva+size>image_size)return 7;memcpy((uint8_t*)image+rva,raw+offset,size);}
 munmap(raw,st.st_size);patch(0x140613c10ULL,alloc_bridge);patch(0x140613c20ULL,dealloc_bridge);patch(0x140613c30ULL,realloc_bridge);
 uintptr_t panics[]={0x1428d9430ULL,0x1428d9390ULL,0x140d8b480ULL,0x1428d8fe3ULL,0x1428d9518ULL};for(unsigned i=0;i<5;++i)patch(panics[i],panic_bridge);
 uint8_t capture_patch[15]={0x49,0x89,0xe9,0x48,0xb8,0,0,0,0,0,0,0,0,0xff,0xe0};void* capture_callback=capture_bridge;memcpy(capture_patch+5,&capture_callback,8);memcpy((void*)0x14129dbd0ULL,capture_patch,15);
 struct sigaction a={0};a.sa_sigaction=fault;a.sa_flags=SA_SIGINFO;sigaction(SIGSEGV,&a,NULL);sigaction(SIGBUS,&a,NULL);
 FILE*in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 8;setvbuf(out,NULL,_IONBF,0);unsigned rows=0,panic_count=0;uint32_t header[6],endpoints[4];
 while(fread(header,sizeof(header),1,in)==1){
  if(fread(endpoints,sizeof(endpoints),1,in)!=1)return 9;void*inputs[4];unsigned widths[4]={1,2,3,1};
  for(unsigned j=0;j<4;++j){size_t bytes=(size_t)header[j+1]*widths[j]*4;inputs[j]=malloc(bytes+1);if(bytes&&fread(inputs[j],bytes,1,in)!=1)return 10;}
  current_id=header[0];allocation_count=0;memset(output_counts,0,sizeof(output_counts));output_indices=output_positions=output_normals=NULL;
  int result=sigsetjmp(panic_return,1);if(result==0){uint8_t context[256]={0},commands[64]={0};((floor_fn)(uintptr_t)0x1407dc170ULL)((void*)0x11223344,inputs[0],header[1],inputs[1],header[2],inputs[2],header[3],context,commands,header[5],endpoints,inputs[3],header[4]);return 11;}
  uint32_t status[5]={header[0],result==1,output_counts[0],output_counts[1],output_counts[2]};if(result==1)++panic_count;
  fwrite(status,sizeof(status),1,out);fwrite(output_indices,4,status[2],out);fwrite(output_positions,12,status[3],out);fwrite(output_normals,12,status[4],out);
  free(output_indices);free(output_positions);free(output_normals);for(unsigned j=0;j<4;++j)free(inputs[j]);for(unsigned j=0;j<allocation_count;++j)free(allocations[j]);++rows;
 }
 fclose(in);fclose(out);close(fd);fprintf(stderr,"original stair floor geometry: %u cases %u panics\n",rows,panic_count);return 0;
}
