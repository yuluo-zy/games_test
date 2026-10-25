
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140112b10:     	pushq	%rbp
140112b11:     	pushq	%rsi
140112b12:     	pushq	%rdi
140112b13:     	subq	$0x210, %rsp            # imm = 0x210
140112b1a:     	leaq	0x80(%rsp), %rbp
140112b22:     	movq	$-0x2, 0x188(%rbp)
140112b2d:     	movq	%rcx, %rsi
140112b30:     	xorps	%xmm0, %xmm0
140112b33:     	movups	%xmm0, 0x68(%rbp)
140112b37:     	movups	%xmm0, 0x80(%rbp)
140112b3e:     	movups	%xmm0, 0x98(%rbp)
140112b45:     	movups	%xmm0, 0xb0(%rbp)
140112b4c:     	movups	%xmm0, 0xc8(%rbp)
140112b53:     	movl	$0x0, 0xd8(%rbp)
140112b5d:     	movups	%xmm0, 0x100(%rbp)
140112b64:     	movups	%xmm0, 0x118(%rbp)
140112b6b:     	movups	%xmm0, 0x130(%rbp)
140112b72:     	movups	%xmm0, 0x148(%rbp)
140112b79:     	movups	%xmm0, 0x160(%rbp)
140112b80:     	movl	$0x0, 0x170(%rbp)
140112b8a:     	movq	$0x0, -0x60(%rbp)
140112b92:     	movq	$0x0, 0x48(%rbp)
140112b9a:     	movq	$0x8, 0x50(%rbp)
140112ba2:     	movq	$0x0, 0x58(%rbp)
140112baa:     	movq	$0x10, 0x60(%rbp)
140112bb2:     	movq	$0x10, 0x78(%rbp)
140112bba:     	movq	$0x10, 0x90(%rbp)
140112bc5:     	movq	$0x10, 0xa8(%rbp)
140112bd0:     	movq	$0x10, 0xc0(%rbp)
140112bdb:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140112be5:     	movq	%rax, 0xe0(%rbp)
140112bec:     	leaq	0x283d6e2(%rip), %rax   # 0x1429502d5
140112bf3:     	movq	%rax, 0xe8(%rbp)
140112bfa:     	movq	$0x3e, 0xf0(%rbp)
140112c05:     	movq	$0x10, 0xf8(%rbp)
140112c10:     	movq	$0x10, 0x110(%rbp)
140112c1b:     	movq	$0x10, 0x128(%rbp)
140112c26:     	movq	$0x10, 0x140(%rbp)
140112c31:     	movq	$0x10, 0x158(%rbp)
140112c3c:     	movl	$0x0, 0x178(%rbp)
140112c46:     	movw	$0x1, 0x17c(%rbp)
140112c4f:     	movl	$0x0, 0x180(%rbp)
140112c59:     	movzbl	0x3521fa9(%rip), %eax   # 0x143634c09
140112c60:     	movl	$0x1e8, %ecx            # imm = 0x1E8
140112c65:     	movl	$0x8, %edx
140112c6a:     	callq	0x140613c10 <.text+0x612c10>
140112c6f:     	testq	%rax, %rax
140112c72:     	je	0x140112ca9 <.text+0x111ca9>
140112c74:     	movq	%rax, %rdi
140112c77:     	leaq	-0x60(%rbp), %rdx
140112c7b:     	movl	$0x1e8, %r8d            # imm = 0x1E8
140112c81:     	movq	%rax, %rcx
140112c84:     	callq	0x142923c00 <.text+0x2922c00>
140112c89:     	leaq	0x283d4b8(%rip), %r8    # 0x142950148
140112c90:     	movq	%rsi, %rcx
140112c93:     	movq	%rdi, %rdx
140112c96:     	callq	0x140ce4b10 <.text+0xce3b10>
140112c9b:     	movq	%rsi, %rax
140112c9e:     	addq	$0x210, %rsp            # imm = 0x210
140112ca5:     	popq	%rdi
140112ca6:     	popq	%rsi
140112ca7:     	popq	%rbp
140112ca8:     	retq
140112ca9:     	movl	$0x8, %ecx
140112cae:     	movl	$0x1e8, %edx            # imm = 0x1E8
140112cb3:     	callq	0x1428d9000 <.text+0x28d8000>
140112cb8:     	ud2
