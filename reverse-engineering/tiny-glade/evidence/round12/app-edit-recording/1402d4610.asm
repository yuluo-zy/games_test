
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402d4610:     	pushq	%rbp
1402d4611:     	pushq	%rsi
1402d4612:     	pushq	%rdi
1402d4613:     	subq	$0x190, %rsp            # imm = 0x190
1402d461a:     	leaq	0x80(%rsp), %rbp
1402d4622:     	movq	$-0x2, 0x108(%rbp)
1402d462d:     	movq	%rcx, %rsi
1402d4630:     	xorps	%xmm0, %xmm0
1402d4633:     	movups	%xmm0, -0x18(%rbp)
1402d4637:     	movups	%xmm0, (%rbp)
1402d463b:     	movups	%xmm0, 0x18(%rbp)
1402d463f:     	movups	%xmm0, 0x30(%rbp)
1402d4643:     	movups	%xmm0, 0x48(%rbp)
1402d4647:     	movl	$0x0, 0x58(%rbp)
1402d464e:     	movups	%xmm0, 0x80(%rbp)
1402d4655:     	movups	%xmm0, 0x98(%rbp)
1402d465c:     	movups	%xmm0, 0xb0(%rbp)
1402d4663:     	movups	%xmm0, 0xc8(%rbp)
1402d466a:     	movups	%xmm0, 0xe0(%rbp)
1402d4671:     	movl	$0x0, 0xf0(%rbp)
1402d467b:     	movq	$0x0, -0x60(%rbp)
1402d4683:     	movq	$0x0, -0x38(%rbp)
1402d468b:     	movq	$0x8, -0x30(%rbp)
1402d4693:     	movq	$0x0, -0x28(%rbp)
1402d469b:     	movq	$0x10, -0x20(%rbp)
1402d46a3:     	movq	$0x10, -0x8(%rbp)
1402d46ab:     	movq	$0x10, 0x10(%rbp)
1402d46b3:     	movq	$0x10, 0x28(%rbp)
1402d46bb:     	movq	$0x10, 0x40(%rbp)
1402d46c3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402d46cd:     	movq	%rax, 0x60(%rbp)
1402d46d1:     	leaq	0x26de345(%rip), %rax   # 0x1429b2a1d
1402d46d8:     	movq	%rax, 0x68(%rbp)
1402d46dc:     	movq	$0x5b, 0x70(%rbp)
1402d46e4:     	movq	$0x10, 0x78(%rbp)
1402d46ec:     	movq	$0x10, 0x90(%rbp)
1402d46f7:     	movq	$0x10, 0xa8(%rbp)
1402d4702:     	movq	$0x10, 0xc0(%rbp)
1402d470d:     	movq	$0x10, 0xd8(%rbp)
1402d4718:     	movl	$0x0, 0xf8(%rbp)
1402d4722:     	movw	$0x1, 0xfc(%rbp)
1402d472b:     	movl	$0x0, 0x100(%rbp)
1402d4735:     	movzbl	0x33604cd(%rip), %eax   # 0x143634c09
1402d473c:     	movl	$0x168, %ecx            # imm = 0x168
1402d4741:     	movl	$0x8, %edx
1402d4746:     	callq	0x140613c10 <.text+0x612c10>
1402d474b:     	testq	%rax, %rax
1402d474e:     	je	0x1402d4785 <.text+0x2d3785>
1402d4750:     	movq	%rax, %rdi
1402d4753:     	leaq	-0x60(%rbp), %rdx
1402d4757:     	movl	$0x168, %r8d            # imm = 0x168
1402d475d:     	movq	%rax, %rcx
1402d4760:     	callq	0x142923c00 <.text+0x2922c00>
1402d4765:     	leaq	0x26de0b4(%rip), %r8    # 0x1429b2820
1402d476c:     	movq	%rsi, %rcx
1402d476f:     	movq	%rdi, %rdx
1402d4772:     	callq	0x140ce4b10 <.text+0xce3b10>
1402d4777:     	movq	%rsi, %rax
1402d477a:     	addq	$0x190, %rsp            # imm = 0x190
1402d4781:     	popq	%rdi
1402d4782:     	popq	%rsi
1402d4783:     	popq	%rbp
1402d4784:     	retq
1402d4785:     	movl	$0x8, %ecx
1402d478a:     	movl	$0x168, %edx            # imm = 0x168
1402d478f:     	callq	0x1428d9000 <.text+0x28d8000>
1402d4794:     	ud2
