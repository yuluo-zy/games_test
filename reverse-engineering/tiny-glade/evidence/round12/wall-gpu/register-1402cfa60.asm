
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402cfa60:     	pushq	%rbp
1402cfa61:     	pushq	%rsi
1402cfa62:     	pushq	%rdi
1402cfa63:     	subq	$0x1c0, %rsp            # imm = 0x1C0
1402cfa6a:     	leaq	0x80(%rsp), %rbp
1402cfa72:     	movq	$-0x2, 0x138(%rbp)
1402cfa7d:     	movq	%rcx, %rsi
1402cfa80:     	xorps	%xmm0, %xmm0
1402cfa83:     	movups	%xmm0, 0x18(%rbp)
1402cfa87:     	movups	%xmm0, 0x30(%rbp)
1402cfa8b:     	movups	%xmm0, 0x48(%rbp)
1402cfa8f:     	movups	%xmm0, 0x60(%rbp)
1402cfa93:     	movups	%xmm0, 0x78(%rbp)
1402cfa97:     	movl	$0x0, 0x88(%rbp)
1402cfaa1:     	movups	%xmm0, 0xb0(%rbp)
1402cfaa8:     	movups	%xmm0, 0xc8(%rbp)
1402cfaaf:     	movups	%xmm0, 0xe0(%rbp)
1402cfab6:     	movups	%xmm0, 0xf8(%rbp)
1402cfabd:     	movups	%xmm0, 0x110(%rbp)
1402cfac4:     	movl	$0x0, 0x120(%rbp)
1402cface:     	movq	$0x0, -0x60(%rbp)
1402cfad6:     	movq	$0x0, -0x8(%rbp)
1402cfade:     	movq	$0x8, (%rbp)
1402cfae6:     	movq	$0x0, 0x8(%rbp)
1402cfaee:     	movq	$0x10, 0x10(%rbp)
1402cfaf6:     	movq	$0x10, 0x28(%rbp)
1402cfafe:     	movq	$0x10, 0x40(%rbp)
1402cfb06:     	movq	$0x10, 0x58(%rbp)
1402cfb0e:     	movq	$0x10, 0x70(%rbp)
1402cfb16:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402cfb20:     	movq	%rax, 0x90(%rbp)
1402cfb27:     	leaq	0x26e1ef2(%rip), %rax   # 0x1429b1a20
1402cfb2e:     	movq	%rax, 0x98(%rbp)
1402cfb35:     	movq	$0x3d, 0xa0(%rbp)
1402cfb40:     	movq	$0x10, 0xa8(%rbp)
1402cfb4b:     	movq	$0x10, 0xc0(%rbp)
1402cfb56:     	movq	$0x10, 0xd8(%rbp)
1402cfb61:     	movq	$0x10, 0xf0(%rbp)
1402cfb6c:     	movq	$0x10, 0x108(%rbp)
1402cfb77:     	movl	$0x0, 0x128(%rbp)
1402cfb81:     	movw	$0x1, 0x12c(%rbp)
1402cfb8a:     	movl	$0x0, 0x130(%rbp)
1402cfb94:     	movzbl	0x336506e(%rip), %eax   # 0x143634c09
1402cfb9b:     	movl	$0x198, %ecx            # imm = 0x198
1402cfba0:     	movl	$0x8, %edx
1402cfba5:     	callq	0x140613c10 <.text+0x612c10>
1402cfbaa:     	testq	%rax, %rax
1402cfbad:     	je	0x1402cfbe4 <.text+0x2cebe4>
1402cfbaf:     	movq	%rax, %rdi
1402cfbb2:     	leaq	-0x60(%rbp), %rdx
1402cfbb6:     	movl	$0x198, %r8d            # imm = 0x198
1402cfbbc:     	movq	%rax, %rcx
1402cfbbf:     	callq	0x142923c00 <.text+0x2922c00>
1402cfbc4:     	leaq	0x26e1d3d(%rip), %r8    # 0x1429b1908
1402cfbcb:     	movq	%rsi, %rcx
1402cfbce:     	movq	%rdi, %rdx
1402cfbd1:     	callq	0x140ce4b10 <.text+0xce3b10>
1402cfbd6:     	movq	%rsi, %rax
1402cfbd9:     	addq	$0x1c0, %rsp            # imm = 0x1C0
1402cfbe0:     	popq	%rdi
1402cfbe1:     	popq	%rsi
1402cfbe2:     	popq	%rbp
1402cfbe3:     	retq
1402cfbe4:     	movl	$0x8, %ecx
1402cfbe9:     	movl	$0x198, %edx            # imm = 0x198
1402cfbee:     	callq	0x1428d9000 <.text+0x28d8000>
1402cfbf3:     	ud2
