.text
.globl _probe_product
_probe_product:
 pushq %rbp
 movq %rdi,%rbp
 movaps (%rsi),%xmm6
 movabsq $0x1407a665b,%rax
 callq *%rax
 popq %rbp
 retq
.globl _probe_grip
_probe_grip:
 pushq %rbp
 pushq %rbx
 subq $40,%rsp
 movq %rdi,%rbx
 movq %rdx,%rbp
 leaq 0x700(%rbp),%rdi
 movq $2,(%rdi)
 movss %xmm0,0x604(%rbp)
 movabsq $0x1407a523e,%rax
 callq *%rax
 movaps %xmm6,%xmm0
 addq $40,%rsp
 popq %rbx
 popq %rbp
 retq
.globl _probe_projection
_probe_projection:
 pushq %rbx
 movq %rcx,%rbx
 movq %rdi,%rcx
 movq %rsi,%r9
 movss 4(%rdx),%xmm2
 movss (%rdx),%xmm1
 subq $32,%rsp
 movabsq $0x140ac8780,%rax
 callq *%rax
 addq $32,%rsp
 movss %xmm0,(%rbx)
 movss %xmm1,4(%rbx)
 popq %rbx
 retq
