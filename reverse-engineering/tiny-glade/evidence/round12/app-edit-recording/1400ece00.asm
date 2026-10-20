
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400ece00:     	pushq	%rbp
1400ece01:     	pushq	%rsi
1400ece02:     	pushq	%rdi
1400ece03:     	subq	$0x180, %rsp            # imm = 0x180
1400ece0a:     	leaq	0x80(%rsp), %rbp
1400ece12:     	movq	$-0x2, 0xf8(%rbp)
1400ece1d:     	movq	%rcx, %rsi
1400ece20:     	xorps	%xmm0, %xmm0
1400ece23:     	movups	%xmm0, -0x28(%rbp)
1400ece27:     	movups	%xmm0, -0x10(%rbp)
1400ece2b:     	movups	%xmm0, 0x8(%rbp)
1400ece2f:     	movups	%xmm0, 0x20(%rbp)
1400ece33:     	movups	%xmm0, 0x38(%rbp)
1400ece37:     	movl	$0x0, 0x48(%rbp)
1400ece3e:     	movups	%xmm0, 0x70(%rbp)
1400ece42:     	movups	%xmm0, 0x88(%rbp)
1400ece49:     	movups	%xmm0, 0xa0(%rbp)
1400ece50:     	movups	%xmm0, 0xb8(%rbp)
1400ece57:     	movups	%xmm0, 0xd0(%rbp)
1400ece5e:     	movl	$0x0, 0xe0(%rbp)
1400ece68:     	movq	$0x0, -0x60(%rbp)
1400ece70:     	movq	$0x0, -0x48(%rbp)
1400ece78:     	movq	$0x8, -0x40(%rbp)
1400ece80:     	movq	$0x0, -0x38(%rbp)
1400ece88:     	movq	$0x10, -0x30(%rbp)
1400ece90:     	movq	$0x10, -0x18(%rbp)
1400ece98:     	movq	$0x10, (%rbp)
1400ecea0:     	movq	$0x10, 0x18(%rbp)
1400ecea8:     	movq	$0x10, 0x30(%rbp)
1400eceb0:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1400eceba:     	movq	%rax, 0x50(%rbp)
1400ecebe:     	leaq	0x2850522(%rip), %rax   # 0x14293d3e7
1400ecec5:     	movq	%rax, 0x58(%rbp)
1400ecec9:     	movq	$0x4b, 0x60(%rbp)
1400eced1:     	movq	$0x10, 0x68(%rbp)
1400eced9:     	movq	$0x10, 0x80(%rbp)
1400ecee4:     	movq	$0x10, 0x98(%rbp)
1400eceef:     	movq	$0x10, 0xb0(%rbp)
1400ecefa:     	movq	$0x10, 0xc8(%rbp)
1400ecf05:     	movl	$0x0, 0xe8(%rbp)
1400ecf0f:     	movw	$0x1, 0xec(%rbp)
1400ecf18:     	movl	$0x0, 0xf0(%rbp)
1400ecf22:     	movzbl	0x3547ce0(%rip), %eax   # 0x143634c09
1400ecf29:     	movl	$0x158, %ecx            # imm = 0x158
1400ecf2e:     	movl	$0x8, %edx
1400ecf33:     	callq	0x140613c10 <.text+0x612c10>
1400ecf38:     	testq	%rax, %rax
1400ecf3b:     	je	0x1400ecf72 <.text+0xebf72>
1400ecf3d:     	movq	%rax, %rdi
1400ecf40:     	leaq	-0x60(%rbp), %rdx
1400ecf44:     	movl	$0x158, %r8d            # imm = 0x158
1400ecf4a:     	movq	%rax, %rcx
1400ecf4d:     	callq	0x142923c00 <.text+0x2922c00>
1400ecf52:     	leaq	0x285024f(%rip), %r8    # 0x14293d1a8
1400ecf59:     	movq	%rsi, %rcx
1400ecf5c:     	movq	%rdi, %rdx
1400ecf5f:     	callq	0x140ce4b10 <.text+0xce3b10>
1400ecf64:     	movq	%rsi, %rax
1400ecf67:     	addq	$0x180, %rsp            # imm = 0x180
1400ecf6e:     	popq	%rdi
1400ecf6f:     	popq	%rsi
1400ecf70:     	popq	%rbp
1400ecf71:     	retq
1400ecf72:     	movl	$0x8, %ecx
1400ecf77:     	movl	$0x158, %edx            # imm = 0x158
1400ecf7c:     	callq	0x1428d9000 <.text+0x28d8000>
1400ecf81:     	ud2
