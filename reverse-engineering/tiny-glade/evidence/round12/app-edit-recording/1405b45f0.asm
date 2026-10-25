
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405b45f0:     	pushq	%rbp
1405b45f1:     	pushq	%rsi
1405b45f2:     	pushq	%rdi
1405b45f3:     	subq	$0x240, %rsp            # imm = 0x240
1405b45fa:     	leaq	0x80(%rsp), %rbp
1405b4602:     	movq	$-0x2, 0x1b8(%rbp)
1405b460d:     	movq	%rcx, %rsi
1405b4610:     	xorps	%xmm0, %xmm0
1405b4613:     	movups	%xmm0, 0x98(%rbp)
1405b461a:     	movups	%xmm0, 0xb0(%rbp)
1405b4621:     	movups	%xmm0, 0xc8(%rbp)
1405b4628:     	movups	%xmm0, 0xe0(%rbp)
1405b462f:     	movups	%xmm0, 0xf8(%rbp)
1405b4636:     	movl	$0x0, 0x108(%rbp)
1405b4640:     	movups	%xmm0, 0x130(%rbp)
1405b4647:     	movups	%xmm0, 0x148(%rbp)
1405b464e:     	movups	%xmm0, 0x160(%rbp)
1405b4655:     	movups	%xmm0, 0x178(%rbp)
1405b465c:     	movups	%xmm0, 0x190(%rbp)
1405b4663:     	movl	$0x0, 0x1a0(%rbp)
1405b466d:     	movq	$0x0, -0x58(%rbp)
1405b4675:     	movq	$0x0, 0x78(%rbp)
1405b467d:     	movq	$0x8, 0x80(%rbp)
1405b4688:     	movq	$0x0, 0x88(%rbp)
1405b4693:     	movq	$0x10, 0x90(%rbp)
1405b469e:     	movq	$0x10, 0xa8(%rbp)
1405b46a9:     	movq	$0x10, 0xc0(%rbp)
1405b46b4:     	movq	$0x10, 0xd8(%rbp)
1405b46bf:     	movq	$0x10, 0xf0(%rbp)
1405b46ca:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405b46d4:     	movq	%rax, 0x110(%rbp)
1405b46db:     	leaq	0x244d073(%rip), %rax   # 0x142a01755
1405b46e2:     	movq	%rax, 0x118(%rbp)
1405b46e9:     	movq	$0x3f, 0x120(%rbp)
1405b46f4:     	movq	$0x10, 0x128(%rbp)
1405b46ff:     	movq	$0x10, 0x140(%rbp)
1405b470a:     	movq	$0x10, 0x158(%rbp)
1405b4715:     	movq	$0x10, 0x170(%rbp)
1405b4720:     	movq	$0x10, 0x188(%rbp)
1405b472b:     	movl	$0x0, 0x1a8(%rbp)
1405b4735:     	movw	$0x1, 0x1ac(%rbp)
1405b473e:     	movl	$0x0, 0x1b0(%rbp)
1405b4748:     	movzbl	0x30804ba(%rip), %eax   # 0x143634c09
1405b474f:     	movl	$0x210, %ecx            # imm = 0x210
1405b4754:     	movl	$0x8, %edx
1405b4759:     	callq	0x140613c10 <.text+0x612c10>
1405b475e:     	testq	%rax, %rax
1405b4761:     	je	0x1405b4798 <.text+0x5b3798>
1405b4763:     	movq	%rax, %rdi
1405b4766:     	leaq	-0x58(%rbp), %rdx
1405b476a:     	movl	$0x210, %r8d            # imm = 0x210
1405b4770:     	movq	%rax, %rcx
1405b4773:     	callq	0x142923c00 <.text+0x2922c00>
1405b4778:     	leaq	0x244cf01(%rip), %r8    # 0x142a01680
1405b477f:     	movq	%rsi, %rcx
1405b4782:     	movq	%rdi, %rdx
1405b4785:     	callq	0x140ce4b10 <.text+0xce3b10>
1405b478a:     	movq	%rsi, %rax
1405b478d:     	addq	$0x240, %rsp            # imm = 0x240
1405b4794:     	popq	%rdi
1405b4795:     	popq	%rsi
1405b4796:     	popq	%rbp
1405b4797:     	retq
1405b4798:     	movl	$0x8, %ecx
1405b479d:     	movl	$0x210, %edx            # imm = 0x210
1405b47a2:     	callq	0x1428d9000 <.text+0x28d8000>
1405b47a7:     	ud2
