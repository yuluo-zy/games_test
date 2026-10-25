
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a844f0:     	pushq	%rbp
140a844f1:     	pushq	%rsi
140a844f2:     	pushq	%rdi
140a844f3:     	pushq	%rbx
140a844f4:     	subq	$0x68, %rsp
140a844f8:     	leaq	0x60(%rsp), %rbp
140a844fd:     	movq	$-0x2, (%rbp)
140a84505:     	movl	0x68(%rcx), %eax
140a84508:     	cmpl	$-0x2, %eax
140a8450b:     	movq	%rdx, -0x38(%rbp)
140a8450f:     	movq	%rax, -0x40(%rbp)
140a84513:     	jle	0x140a845c8 <.text+0xa835c8>
140a84519:     	movq	%rcx, %rsi
140a8451c:     	incl	%eax
140a8451e:     	movl	%eax, 0x68(%rcx)
140a84521:     	movq	0x20(%rdx), %rax
140a84525:     	movq	%rax, -0x10(%rbp)
140a84529:     	movups	(%rdx), %xmm0
140a8452c:     	movups	0x10(%rdx), %xmm1
140a84530:     	movaps	%xmm1, -0x20(%rbp)
140a84534:     	movaps	%xmm0, -0x30(%rbp)
140a84538:     	movq	0x10(%rcx), %rbx
140a8453c:     	cmpq	(%rcx), %rbx
140a8453f:     	jne	0x140a84550 <.text+0xa83550>
140a84541:     	leaq	0x2069198(%rip), %rdx   # 0x142aed6e0
140a84548:     	movq	%rsi, %rcx
140a8454b:     	callq	0x140562ab0 <.text+0x561ab0>
140a84550:     	movq	0x8(%rsi), %rdi
140a84554:     	testq	%rbx, %rbx
140a84557:     	je	0x140a84571 <.text+0xa83571>
140a84559:     	leaq	0x28(%rdi), %rcx
140a8455d:     	leaq	(,%rbx,8), %rax
140a84565:     	leaq	(%rax,%rax,4), %r8
140a84569:     	movq	%rdi, %rdx
140a8456c:     	callq	0x142923c10 <.text+0x2922c10>
140a84571:     	movaps	-0x30(%rbp), %xmm0
140a84575:     	movaps	-0x20(%rbp), %xmm1
140a84579:     	movq	-0x10(%rbp), %rax
140a8457d:     	movq	%rax, 0x20(%rdi)
140a84581:     	movups	%xmm1, 0x10(%rdi)
140a84585:     	movups	%xmm0, (%rdi)
140a84588:     	incq	%rbx
140a8458b:     	movq	%rbx, 0x10(%rsi)
140a8458f:     	movq	$0x0, -0x30(%rbp)
140a84597:     	movq	$-0x1, -0x28(%rbp)
140a8459f:     	movb	$0x0, -0x20(%rbp)
140a845a3:     	leaq	0x205336e(%rip), %rcx   # 0x142ad7918
140a845aa:     	leaq	-0x30(%rbp), %rdx
140a845ae:     	callq	0x14132ade0 <.text+0x1329de0>
140a845b3:     	movq	%rax, 0x60(%rsi)
140a845b7:     	movq	$0x0, 0x58(%rsi)
140a845bf:     	addq	$0x68, %rsp
140a845c3:     	popq	%rbx
140a845c4:     	popq	%rdi
140a845c5:     	popq	%rsi
140a845c6:     	popq	%rbp
140a845c7:     	retq
