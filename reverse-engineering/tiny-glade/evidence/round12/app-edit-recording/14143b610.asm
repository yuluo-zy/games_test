
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14143b610:     	pushq	%rbp
14143b611:     	pushq	%rsi
14143b612:     	pushq	%rdi
14143b613:     	subq	$0x1d0, %rsp            # imm = 0x1D0
14143b61a:     	leaq	0x80(%rsp), %rbp
14143b622:     	movq	$-0x2, 0x148(%rbp)
14143b62d:     	movq	%rcx, %rsi
14143b630:     	xorps	%xmm0, %xmm0
14143b633:     	movups	%xmm0, 0x28(%rbp)
14143b637:     	movups	%xmm0, 0x40(%rbp)
14143b63b:     	movups	%xmm0, 0x58(%rbp)
14143b63f:     	movups	%xmm0, 0x70(%rbp)
14143b643:     	movups	%xmm0, 0x88(%rbp)
14143b64a:     	movl	$0x0, 0x98(%rbp)
14143b654:     	movups	%xmm0, 0xc0(%rbp)
14143b65b:     	movups	%xmm0, 0xd8(%rbp)
14143b662:     	movups	%xmm0, 0xf0(%rbp)
14143b669:     	movups	%xmm0, 0x108(%rbp)
14143b670:     	movups	%xmm0, 0x120(%rbp)
14143b677:     	movl	$0x0, 0x130(%rbp)
14143b681:     	movq	$0x0, -0x60(%rbp)
14143b689:     	movq	$0x0, 0x8(%rbp)
14143b691:     	movq	$0x8, 0x10(%rbp)
14143b699:     	movq	$0x0, 0x18(%rbp)
14143b6a1:     	movq	$0x10, 0x20(%rbp)
14143b6a9:     	movq	$0x10, 0x38(%rbp)
14143b6b1:     	movq	$0x10, 0x50(%rbp)
14143b6b9:     	movq	$0x10, 0x68(%rbp)
14143b6c1:     	movq	$0x10, 0x80(%rbp)
14143b6cc:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14143b6d6:     	movq	%rax, 0xa0(%rbp)
14143b6dd:     	leaq	0x18238b3(%rip), %rax   # 0x142c5ef97
14143b6e4:     	movq	%rax, 0xa8(%rbp)
14143b6eb:     	movq	$0x40, 0xb0(%rbp)
14143b6f6:     	movq	$0x10, 0xb8(%rbp)
14143b701:     	movq	$0x10, 0xd0(%rbp)
14143b70c:     	movq	$0x10, 0xe8(%rbp)
14143b717:     	movq	$0x10, 0x100(%rbp)
14143b722:     	movq	$0x10, 0x118(%rbp)
14143b72d:     	movl	$0x0, 0x138(%rbp)
14143b737:     	movw	$0x1, 0x13c(%rbp)
14143b740:     	movl	$0x0, 0x140(%rbp)
14143b74a:     	movzbl	0x21f94b8(%rip), %eax   # 0x143634c09
14143b751:     	movl	$0x1a8, %ecx            # imm = 0x1A8
14143b756:     	movl	$0x8, %edx
14143b75b:     	callq	0x140613c10 <.text+0x612c10>
14143b760:     	testq	%rax, %rax
14143b763:     	je	0x14143b79a <.text+0x143a79a>
14143b765:     	movq	%rax, %rdi
14143b768:     	leaq	-0x60(%rbp), %rdx
14143b76c:     	movl	$0x1a8, %r8d            # imm = 0x1A8
14143b772:     	movq	%rax, %rcx
14143b775:     	callq	0x142923c00 <.text+0x2922c00>
14143b77a:     	leaq	0x18235ef(%rip), %r8    # 0x142c5ed70
14143b781:     	movq	%rsi, %rcx
14143b784:     	movq	%rdi, %rdx
14143b787:     	callq	0x140ce4b10 <.text+0xce3b10>
14143b78c:     	movq	%rsi, %rax
14143b78f:     	addq	$0x1d0, %rsp            # imm = 0x1D0
14143b796:     	popq	%rdi
14143b797:     	popq	%rsi
14143b798:     	popq	%rbp
14143b799:     	retq
14143b79a:     	movl	$0x8, %ecx
14143b79f:     	movl	$0x1a8, %edx            # imm = 0x1A8
14143b7a4:     	callq	0x1428d9000 <.text+0x28d8000>
14143b7a9:     	ud2
