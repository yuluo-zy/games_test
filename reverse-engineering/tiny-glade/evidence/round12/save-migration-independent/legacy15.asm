
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141471f9d:     	movq	0x10(%r8), %rcx
141471fa1:     	cmpq	%r14, %rcx
141471fa4:     	jne	0x141472090 <.text+0x1471090>
141471faa:     	leaq	0x10(%r8), %rcx
141471fae:     	movq	0x18(%r8), %rax
141471fb2:     	movq	%rax, 0x100(%rbp)
141471fb9:     	movl	0x20(%r8), %r9d
141471fbd:     	leaq	0x24(%r8), %rax
141471fc1:     	movq	%rax, 0x318(%rbp)
141471fc8:     	cmpb	$0x0, 0x3c(%r8)
141471fcd:     	leaq	0x160(%rbp), %rax
141471fd4:     	leaq	0x270(%rbp), %rdx
141471fdb:     	cmovneq	%rdx, %rax
141471fdf:     	movl	%r9d, %edx
141471fe2:     	cmovnel	%edi, %edx
141471fe5:     	movl	%edx, 0x158(%rbp)
141471feb:     	cmovel	%edi, %r9d
141471fef:     	movl	%r9d, 0x320(%rbp)
141471ff6:     	movq	0x34(%r8), %rdx
141471ffa:     	movq	%rdx, 0x10(%rax)
141471ffe:     	movups	0x24(%r8), %xmm0
141472003:     	movups	%xmm0, (%rax)
141472006:     	movq	%r8, 0x308(%rbp)
14147200d:     	callq	0x140a82940 <.text+0xa81940>
141472012:     	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
14147201c:     	movq	0x308(%rbp), %rdx
141472023:     	movq	%rax, 0x10(%rdx)
141472027:     	movq	0x100(%rbp), %rax
14147202e:     	movq	%rax, 0x18(%rdx)
141472032:     	movl	0x158(%rbp), %eax
141472038:     	movl	%eax, 0x20(%rdx)
14147203b:     	movq	0x170(%rbp), %rax
141472042:     	movq	0x318(%rbp), %rcx
141472049:     	movq	%rax, 0x10(%rcx)
14147204d:     	movups	0x160(%rbp), %xmm0
141472054:     	movups	%xmm0, (%rcx)
141472057:     	movl	0x320(%rbp), %eax
14147205d:     	movl	%eax, 0x3c(%rdx)
141472060:     	movups	0x270(%rbp), %xmm0
141472067:     	movups	%xmm0, 0x40(%rdx)
14147206b:     	movq	0x280(%rbp), %rax
141472072:     	movq	%rax, 0x50(%rdx)
141472076:     	movb	$0x1, %al
141472078:     	movq	$-0x1, %r10
