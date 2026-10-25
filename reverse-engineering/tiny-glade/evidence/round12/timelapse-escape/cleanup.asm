
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8a5d0:     	pushq	%rsi
140a8a5d1:     	subq	$0xb0, %rsp
140a8a5d8:     	movq	(%rdx), %rax
140a8a5db:     	movq	0x8(%rdx), %rdx
140a8a5df:     	movq	(%rax), %r8
140a8a5e2:     	xorl	%r9d, %r9d
140a8a5e5:     	movq	%r8, %r10
140a8a5e8:     	subq	0x18(%rdx), %r10
140a8a5ec:     	cmovbq	%r9, %r10
140a8a5f0:     	subq	0x38(%rdx), %r8
140a8a5f4:     	cmovbq	%r9, %r8
140a8a5f8:     	movq	0x30(%rdx), %r11
140a8a5fc:     	subq	%r8, %r11
140a8a5ff:     	cmovbq	%r9, %r11
140a8a603:     	seta	%r8b
140a8a607:     	movq	0x10(%rdx), %rsi
140a8a60b:     	subq	%r10, %rsi
140a8a60e:     	cmovbq	%r9, %rsi
140a8a612:     	movq	0x40(%rdx), %rdx
140a8a616:     	seta	%r9b
140a8a61a:     	addq	%r11, %rsi
140a8a61d:     	movq	%rdx, %r10
140a8a620:     	subq	%rsi, %r10
140a8a623:     	movq	%r10, (%rax)
140a8a626:     	orb	%r8b, %r9b
140a8a629:     	je	0x140a8a700 <.text+0xa89700>
140a8a62f:     	movq	%rdx, (%rax)
140a8a632:     	movq	0x2ba1d6f(%rip), %rax   # 0x14362c3a8
140a8a639:     	movq	(%rax), %rax
140a8a63c:     	cmpq	$0x2, %rax
140a8a640:     	jbe	0x140a8a6f1 <.text+0xa896f1>
140a8a646:     	leaq	0x204eaab(%rip), %rax   # 0x142ad90f8
140a8a64d:     	movq	%rcx, %rsi
140a8a650:     	movq	%rax, %rcx
140a8a653:     	callq	0x14067dd80 <.text+0x67cd80>
140a8a658:     	movups	(%rax), %xmm0
140a8a65b:     	movl	0x10(%rax), %eax
140a8a65e:     	movq	$0x3, 0x60(%rsp)
140a8a667:     	leaq	0x204eaa2(%rip), %rcx   # 0x142ad9110
140a8a66e:     	movq	%rcx, 0x68(%rsp)
140a8a673:     	movq	$0x27, 0x70(%rsp)
140a8a67c:     	leaq	0x204ea65(%rip), %rdx   # 0x142ad90e8
140a8a683:     	movq	%rdx, 0x80(%rsp)
140a8a68b:     	movq	$0x1, 0x88(%rsp)
140a8a697:     	movq	$0x8, 0x90(%rsp)
140a8a6a3:     	movq	$0x0, 0x30(%rsp)
140a8a6ac:     	xorps	%xmm1, %xmm1
140a8a6af:     	movups	%xmm1, 0x98(%rsp)
140a8a6b7:     	movq	%rcx, 0x38(%rsp)
140a8a6bc:     	movq	$0x27, 0x40(%rsp)
140a8a6c5:     	movq	$0x0, 0x48(%rsp)
140a8a6ce:     	movups	%xmm0, 0x50(%rsp)
140a8a6d3:     	movl	$0x1, 0x78(%rsp)
140a8a6db:     	movl	%eax, 0x7c(%rsp)
140a8a6df:     	leaq	0x2f(%rsp), %rcx
140a8a6e4:     	leaq	0x30(%rsp), %rdx
140a8a6e9:     	callq	0x140d4a330 <.text+0xd49330>
140a8a6ee:     	movq	%rsi, %rcx
140a8a6f1:     	movl	0x1c(%rcx), %eax
140a8a6f4:     	movq	0x10(%rcx), %rdx
140a8a6f8:     	movl	%eax, (%rdx)
140a8a6fa:     	movq	(%rcx), %rax
140a8a6fd:     	movb	$0x2, (%rax)
140a8a700:     	addq	$0xb0, %rsp
140a8a707:     	popq	%rsi
140a8a708:     	retq
140a8a709:     	int3
140a8a70a:     	int3
140a8a70b:     	int3
140a8a70c:     	int3
140a8a70d:     	int3
140a8a70e:     	int3
140a8a70f:     	int3
