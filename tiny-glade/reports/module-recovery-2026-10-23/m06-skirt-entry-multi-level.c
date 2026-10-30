/* 原RNE两种完整floor几何分支，以及原圆采样数量入口；
 * 分配/CRT/panic是显式外部边界，几何与采样控制流均执行原指令。 */
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
#include <math.h>
static sigjmp_buf panic_return;
static unsigned current_id,allocation_count;
static void *allocations[1024];
static void __attribute__((ms_abi,noreturn)) panic_bridge(void){siglongjmp(panic_return,1);}
static void *__attribute__((ms_abi)) alloc_bridge(uint64_t size,uint64_t alignment){
 void *p=NULL;if(alignment<sizeof(void*))alignment=sizeof(void*);
 if(posix_memalign(&p,alignment,size))return NULL;
 if(allocation_count>=1024)_exit(21);allocations[allocation_count++]=p;return p;
}
static void __attribute__((ms_abi)) dealloc_bridge(void *p,uint64_t size,uint64_t alignment){
 (void)size;(void)alignment;for(unsigned i=0;i<allocation_count;++i)if(allocations[i]==p){free(p);allocations[i]=NULL;return;}_exit(22);
}
static void *__attribute__((ms_abi)) realloc_bridge(void *old,uint64_t size,uint64_t alignment,uint64_t next){
 void *p=alloc_bridge(next,alignment);if(!p)return NULL;memcpy(p,old,size<next?size:next);dealloc_bridge(old,size,alignment);return p;
}
static void *__attribute__((ms_abi)) zero_bridge(uint64_t size,uint64_t alignment){void*p=alloc_bridge(size,alignment);if(p)memset(p,0,size);return p;}
static float __attribute__((ms_abi)) ceil_bridge(float x){return ceilf(x);}
/* 外部CRT是显式边界：记录原调用参数；可重放目标ARM libm的返回值，
 * 区分原算法差异和跨体系结构外部三角函数差异，不改变任何原几何指令。 */
static uint32_t trig_table[65536][3];static unsigned trig_count;static FILE *trig_log;
static float trig(unsigned kind,float x){
 uint32_t argument,result;memcpy(&argument,&x,4);float value;
 if(trig_count){unsigned i;for(i=0;i<trig_count;++i)if(trig_table[i][0]==kind&&trig_table[i][1]==argument)break;
  if(i==trig_count)_exit(31);result=trig_table[i][2];memcpy(&value,&result,4);
 }else{value=kind?sinf(x):cosf(x);memcpy(&result,&value,4);}
 if(trig_log){uint32_t row[3]={kind,argument,result};fwrite(row,sizeof(row),1,trig_log);}return value;
}
static float __attribute__((ms_abi)) cos_bridge(float x){return trig(0,x);}
static float __attribute__((ms_abi)) sin_bridge(float x){return trig(1,x);}
static float __attribute__((ms_abi)) fmod_bridge(float x,float y){return fmodf(x,y);}
static void *__attribute__((ms_abi)) copy_bridge(void*d,const void*s,uint64_t n){return memcpy(d,s,n);}
static void *__attribute__((ms_abi)) move_bridge(void*d,const void*s,uint64_t n){return memmove(d,s,n);}
static void fault(int signal,siginfo_t *info,void *context){
 ucontext_t *cpu=context;fprintf(stderr,"unexpected signal%d case%u rip%llx address%p\n",signal,current_id,cpu->uc_mcontext->__ss.__rip,info->si_addr);_exit(128+signal);
}
static void patch(uintptr_t at,void *callback){uint8_t b[12]={0x48,0xb8,0,0,0,0,0,0,0,0,0xff,0xe0};memcpy(b+2,&callback,8);memcpy((void*)at,b,12);}

static void *__attribute__((ms_abi)) set_bridge(void*d,int c,uint64_t n){return memset(d,c,n);}
int main(int argc,char**argv){
 if(argc!=9)return 2;
 int fd=open(argv[1],O_RDONLY);struct stat st;if(fd<0||fstat(fd,&st))return 3;
 uint8_t *raw=mmap(NULL,st.st_size,PROT_READ,MAP_PRIVATE,fd,0);if(raw==MAP_FAILED)return 4;
 uint32_t nt;memcpy(&nt,raw+60,4);uint16_t count,optional;memcpy(&count,raw+nt+6,2);memcpy(&optional,raw+nt+20,2);
 uint32_t image_size,header_size;memcpy(&image_size,raw+nt+24+56,4);memcpy(&header_size,raw+nt+24+60,4);
 void *image=mmap((void*)0x140000000ULL,image_size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_ANON|MAP_FIXED,-1,0);if(image==MAP_FAILED)return 5;memcpy(image,raw,header_size);
 for(unsigned i=0;i<count;++i){uint8_t*s=raw+nt+24+optional+i*40;uint32_t rva,size,offset;memcpy(&rva,s+12,4);memcpy(&size,s+16,4);memcpy(&offset,s+20,4);if((uint64_t)offset+size>(uint64_t)st.st_size||(uint64_t)rva+size>image_size)return 7;memcpy((uint8_t*)image+rva,raw+offset,size);}munmap(raw,st.st_size);
 patch(0x140613c10ULL,alloc_bridge);patch(0x140613c20ULL,dealloc_bridge);patch(0x140613c30ULL,realloc_bridge);patch(0x140613c40ULL,zero_bridge);patch(0x142923e60ULL,ceil_bridge);patch(0x142923c00ULL,copy_bridge);patch(0x142923c20ULL,set_bridge);
 struct sigaction a={0};a.sa_sigaction=fault;a.sa_flags=SA_SIGINFO;sigaction(SIGSEGV,&a,NULL);sigaction(SIGBUS,&a,NULL);
 float positions[]={0,0,0,3,0,0,3,0,3};float outline[]={0,0,3,0,3,3};float us[]={0,.5,1};float bottom[]={strtof(argv[2],0),strtof(argv[3],0),strtof(argv[4],0)}, top[]={strtof(argv[5],0),strtof(argv[6],0),strtof(argv[7],0)};
 uint8_t public[0x2a0]={0}, options[24]={0}, holes[0x68]={0};uint64_t output[3]={0,8,0};uint64_t v;
 #define STORE(off,val) do { __typeof__(val) q=(val);memcpy(public+(off),&q,sizeof(q)); } while(0)
 STORE(0,(uint64_t)3);STORE(8,(uintptr_t)outline);STORE(16,(uint64_t)3);STORE(24,(uint64_t)3);STORE(32,(uintptr_t)us);STORE(40,(uint64_t)3);STORE(48,(float)6);STORE(0x90,(uintptr_t)positions);STORE(0x98,(uint64_t)3);STORE(0xf8,(uintptr_t)top);STORE(0x100,(uint64_t)3);STORE(0x108,(uint8_t)1);STORE(0x10c,(float)10);STORE(0x258,(uint32_t)2);STORE(0x284,(float)strtof(argv[8],0));STORE(0x28c,(float)6);
 float opts[]={0,1,1,0,0};memcpy(options,opts,20);options[20]=2;v=4;memcpy(holes+8,&v,8);
 uint64_t curve[7]={3,(uintptr_t)outline,3,3,(uintptr_t)us,3,0};float six=6;memcpy(&curve[6],&six,4);uint64_t final_holes[7]={0,4,0,0,4,0,0};
 typedef void (__attribute__((ms_abi)) *skirt_fn)(void*,uint64_t,void*,void*,float*,uint64_t,float*,uint64_t,void*);
 ((skirt_fn)0x1412221f0ULL)(public,17,output,curve,bottom,3,top,3,final_holes);
 uint64_t h[3];memcpy(h,holes,24);printf("records=%llu holes=%llu\n",output[2],h[2]);
 for(size_t i=0;i<output[2]*96;i++)printf("%02x",((uint8_t*)(uintptr_t)output[1])[i]);puts("");for(size_t i=0;i<h[2]*20;i++)printf("%02x",((uint8_t*)(uintptr_t)h[1])[i]);puts("");return 0;
}
