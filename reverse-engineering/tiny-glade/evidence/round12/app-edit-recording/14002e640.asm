
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14002e640:     	pushq	%rbp
14002e641:     	pushq	%rsi
14002e642:     	pushq	%rdi
14002e643:     	subq	$0x320, %rsp            # imm = 0x320
14002e64a:     	leaq	0x80(%rsp), %rbp
14002e652:     	movq	$-0x2, 0x298(%rbp)
14002e65d:     	movq	%rcx, %rsi
14002e660:     	xorps	%xmm0, %xmm0
14002e663:     	movups	%xmm0, -0x40(%rbp)
14002e667:     	movups	%xmm0, -0x28(%rbp)
14002e66b:     	movups	%xmm0, -0x10(%rbp)
14002e66f:     	movups	%xmm0, 0x8(%rbp)
14002e673:     	movups	%xmm0, 0x20(%rbp)
14002e677:     	movl	$0x0, 0x30(%rbp)
14002e67e:     	movups	%xmm0, 0x58(%rbp)
14002e682:     	movups	%xmm0, 0x70(%rbp)
14002e686:     	movups	%xmm0, 0x88(%rbp)
14002e68d:     	movups	%xmm0, 0xa0(%rbp)
14002e694:     	movups	%xmm0, 0xb8(%rbp)
14002e69b:     	movl	$0x0, 0xc8(%rbp)
14002e6a5:     	movq	$0x0, -0x60(%rbp)
14002e6ad:     	movq	$0x8, -0x58(%rbp)
14002e6b5:     	movq	$0x0, -0x50(%rbp)
14002e6bd:     	movq	$0x10, -0x48(%rbp)
14002e6c5:     	movq	$0x10, -0x30(%rbp)
14002e6cd:     	movq	$0x10, -0x18(%rbp)
14002e6d5:     	movq	$0x10, (%rbp)
14002e6dd:     	movq	$0x10, 0x18(%rbp)
14002e6e5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14002e6ef:     	movq	%rax, 0x38(%rbp)
14002e6f3:     	leaq	0x28fa269(%rip), %rcx   # 0x142928963
14002e6fa:     	movq	%rcx, 0x40(%rbp)
14002e6fe:     	movq	$0x4f, 0x48(%rbp)
14002e706:     	movq	$0x10, 0x50(%rbp)
14002e70e:     	movq	$0x10, 0x68(%rbp)
14002e716:     	movq	$0x10, 0x80(%rbp)
14002e721:     	movq	$0x10, 0x98(%rbp)
14002e72c:     	movq	$0x10, 0xb0(%rbp)
14002e737:     	movl	$0x0, 0xd0(%rbp)
14002e741:     	movw	$0x1, 0xd4(%rbp)
14002e74a:     	movq	%rax, 0xd8(%rbp)
14002e751:     	movl	$0x0, 0x290(%rbp)
14002e75b:     	movzbl	0x36064a7(%rip), %eax   # 0x143634c09
14002e762:     	movl	$0x2f8, %ecx            # imm = 0x2F8
14002e767:     	movl	$0x8, %edx
14002e76c:     	callq	0x140613c10 <.text+0x612c10>
14002e771:     	testq	%rax, %rax
14002e774:     	je	0x14002e7ab <.text+0x2d7ab>
14002e776:     	movq	%rax, %rdi
14002e779:     	leaq	-0x60(%rbp), %rdx
14002e77d:     	movl	$0x2f8, %r8d            # imm = 0x2F8
14002e783:     	movq	%rax, %rcx
14002e786:     	callq	0x142923c00 <.text+0x2922c00>
14002e78b:     	leaq	0x28f9f06(%rip), %r8    # 0x142928698
14002e792:     	movq	%rsi, %rcx
14002e795:     	movq	%rdi, %rdx
14002e798:     	callq	0x140ce4b10 <.text+0xce3b10>
14002e79d:     	movq	%rsi, %rax
14002e7a0:     	addq	$0x320, %rsp            # imm = 0x320
14002e7a7:     	popq	%rdi
14002e7a8:     	popq	%rsi
14002e7a9:     	popq	%rbp
14002e7aa:     	retq
14002e7ab:     	movl	$0x8, %ecx
14002e7b0:     	movl	$0x2f8, %edx            # imm = 0x2F8
14002e7b5:     	callq	0x1428d9000 <.text+0x28d8000>
14002e7ba:     	ud2
