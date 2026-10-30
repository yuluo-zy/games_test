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
extern void native_floor(uint32_t,const void*,void*),capture_rectangle(void),capture_circle(void);
__asm__(
".text\n.globl _native_floor\n_native_floor:\n"
"pushq %rbp\n pushq %rbx\n pushq %rdi\n pushq %rsi\n pushq %r12\n pushq %r13\n pushq %r14\n pushq %r15\n subq $0x608,%rsp\n leaq 0x80(%rsp),%rbp\n movq %rdx,0x500(%rbp)\n"
"movss 24(%rsi),%xmm7\n movl $0x3f000000,%eax\n movd %eax,%xmm6\n movabsq $0xbf80000080000000,%rax\n movq %rax,%xmm9\n movabsq $0x142aa3290,%rax\n movaps (%rax),%xmm13\n movq %rsi,%rdx\n testl %edi,%edi\n jz Lfloor_circle\n movabsq $0x1408d3014,%rax\n jmpq *%rax\n"
"Lfloor_circle:\n movabsq $0x1408d3585,%rax\n jmpq *%rax\n"
".globl _capture_rectangle\n_capture_rectangle:\n"
"movq 0x500(%rbp),%r11\n"
"movups 0x178(%rbp),%xmm0\n movups %xmm0,(%r11)\n movq 0x188(%rbp),%rax\n movq %rax,16(%r11)\n"
"movups 0x240(%rbp),%xmm0\n movups %xmm0,24(%r11)\n movq 0x250(%rbp),%rax\n movq %rax,40(%r11)\n"
"movups 0x270(%rbp),%xmm0\n movups %xmm0,48(%r11)\n movq 0x280(%rbp),%rax\n movq %rax,64(%r11)\n jmp Lfloor_end\n"
".globl _capture_circle\n_capture_circle:\n"
"movq 0x500(%rbp),%r11\n"
"movups 0x240(%rbp),%xmm0\n movups %xmm0,(%r11)\n movq 0x250(%rbp),%rax\n movq %rax,16(%r11)\n"
"movups 0x270(%rbp),%xmm0\n movups %xmm0,24(%r11)\n movq 0x280(%rbp),%rax\n movq %rax,40(%r11)\n"
"movups 0xb8(%rbp),%xmm0\n movups %xmm0,48(%r11)\n movq 0xc8(%rbp),%rax\n movq %rax,64(%r11)\n"
"Lfloor_end:\n addq $0x608,%rsp\n popq %r15\n popq %r14\n popq %r13\n popq %r12\n popq %rsi\n popq %rdi\n popq %rbx\n popq %rbp\n retq\n");
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
int main(int argc,char**argv){
 if(argc!=6&&argc!=7)return 2;
 if(strcmp(argv[4],"-")){FILE*t=fopen(argv[4],"rb");if(!t)return 30;trig_count=(unsigned)fread(trig_table,12,65536,t);fclose(t);}
 trig_log=fopen(argv[5],"wb");if(!trig_log)return 32;
 int fd=open(argv[1],O_RDONLY);struct stat st;if(fd<0||fstat(fd,&st))return 3;
 uint8_t *raw=mmap(NULL,st.st_size,PROT_READ,MAP_PRIVATE,fd,0);if(raw==MAP_FAILED)return 4;
 uint32_t nt;memcpy(&nt,raw+60,4);uint16_t count,optional;memcpy(&count,raw+nt+6,2);memcpy(&optional,raw+nt+20,2);
 uint32_t image_size,header_size;memcpy(&image_size,raw+nt+24+56,4);memcpy(&header_size,raw+nt+24+60,4);
 void *image=mmap((void*)0x140000000ULL,image_size,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_PRIVATE|MAP_ANON|MAP_FIXED,-1,0);if(image==MAP_FAILED)return 5;memcpy(image,raw,header_size);
 for(unsigned i=0;i<count;++i){uint8_t*s=raw+nt+24+optional+i*40;uint32_t rva,size,offset;memcpy(&rva,s+12,4);memcpy(&size,s+16,4);memcpy(&offset,s+20,4);if((uint64_t)offset+size>(uint64_t)st.st_size||(uint64_t)rva+size>image_size)return 7;memcpy((uint8_t*)image+rva,raw+offset,size);}
 munmap(raw,st.st_size);patch(0x140613c10ULL,alloc_bridge);patch(0x140613c20ULL,dealloc_bridge);patch(0x140613c30ULL,realloc_bridge);patch(0x140613c40ULL,zero_bridge);
 patch(0x142923e60ULL,ceil_bridge);patch(0x142923e80ULL,cos_bridge);patch(0x142923fd0ULL,sin_bridge);patch(0x142923f10ULL,fmod_bridge);
 patch(0x142923c00ULL,copy_bridge);patch(0x142923c10ULL,move_bridge);
 patch(0x1408d326aULL,capture_rectangle);patch(0x1408d398cULL,capture_circle);
 uintptr_t panics[]={0x1428d9430ULL,0x1428d9390ULL,0x140d8b480ULL,0x1428d8fe3ULL};for(unsigned i=0;i<4;++i)patch(panics[i],panic_bridge);
 struct sigaction a={0};a.sa_sigaction=fault;a.sa_flags=SA_SIGINFO;sigaction(SIGSEGV,&a,NULL);sigaction(SIGBUS,&a,NULL);
 FILE *in=fopen(argv[2],"rb"),*out=fopen(argv[3],"wb");if(!in||!out)return 8;setvbuf(out,NULL,_IONBF,0);unsigned rows=0,failures=0;
 uint32_t row[9];while(fread(row,sizeof(row),1,in)==1){
  uint64_t vectors[9]={0};uint32_t header[5]={row[0],0,0,0,0};current_id=row[0];allocation_count=0;
  if(argc==7){
   typedef void (__attribute__((ms_abi)) *count_fn)(void*,const void*,float);
   uint64_t iterator[4]={0};((count_fn)0x140c97bc0ULL)(iterator,row+2,0.5f);
   fwrite(row,4,1,out);fwrite(iterator+1,8,1,out);++rows;continue;
  }
  if(sigsetjmp(panic_return,1)){header[1]=1;++failures;}else{native_floor(row[1],row+2,vectors);for(unsigned i=0;i<3;++i)header[2+i]=(uint32_t)vectors[i*3+2];}
  fwrite(header,sizeof(header),1,out);if(!header[1])for(unsigned i=0;i<3;++i)fwrite((void*)(uintptr_t)vectors[i*3+1],12,header[2+i],out);
  for(unsigned i=0;i<allocation_count;++i)free(allocations[i]);++rows;
 }
 fclose(in);fclose(out);fclose(trig_log);close(fd);fprintf(stderr,"original floor branches %u cases %u panics; external allocator/CRT libm bridges\n",rows,failures);return 0;
}
