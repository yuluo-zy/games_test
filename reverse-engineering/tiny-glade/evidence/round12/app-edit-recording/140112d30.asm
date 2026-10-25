
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140112d30:     	pushq	%rbp
140112d31:     	pushq	%rsi
140112d32:     	pushq	%rdi
140112d33:     	pushq	%rbx
140112d34:     	subq	$0x218, %rsp            # imm = 0x218
140112d3b:     	leaq	0x80(%rsp), %rbp
140112d43:     	movq	$-0x2, 0x190(%rbp)
140112d4e:     	movl	%edx, %ebx
140112d50:     	movq	%rcx, %rsi
140112d53:     	xorps	%xmm0, %xmm0
140112d56:     	movups	%xmm0, 0x70(%rbp)
140112d5a:     	movups	%xmm0, 0x88(%rbp)
140112d61:     	movups	%xmm0, 0xa0(%rbp)
140112d68:     	movups	%xmm0, 0xb8(%rbp)
140112d6f:     	movups	%xmm0, 0xd0(%rbp)
140112d76:     	movl	$0x0, 0xe0(%rbp)
140112d80:     	movups	%xmm0, 0x108(%rbp)
140112d87:     	movups	%xmm0, 0x120(%rbp)
140112d8e:     	movups	%xmm0, 0x138(%rbp)
140112d95:     	movups	%xmm0, 0x150(%rbp)
140112d9c:     	movups	%xmm0, 0x168(%rbp)
140112da3:     	movl	$0x0, 0x178(%rbp)
140112dad:     	movq	$0x0, 0x18(%rbp)
140112db5:     	movq	$0x0, 0x50(%rbp)
140112dbd:     	movq	$0x8, 0x58(%rbp)
140112dc5:     	movq	$0x0, 0x60(%rbp)
140112dcd:     	movq	$0x10, 0x68(%rbp)
140112dd5:     	movq	$0x10, 0x80(%rbp)
140112de0:     	movq	$0x10, 0x98(%rbp)
140112deb:     	movq	$0x10, 0xb0(%rbp)
140112df6:     	movq	$0x10, 0xc8(%rbp)
140112e01:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140112e0b:     	movq	%rax, 0xe8(%rbp)
140112e12:     	leaq	0x283d4fa(%rip), %rax   # 0x142950313
140112e19:     	movq	%rax, 0xf0(%rbp)
140112e20:     	movq	$0x3b, 0xf8(%rbp)
140112e2b:     	movq	$0x10, 0x100(%rbp)
140112e36:     	movq	$0x10, 0x118(%rbp)
140112e41:     	movq	$0x10, 0x130(%rbp)
140112e4c:     	movq	$0x10, 0x148(%rbp)
140112e57:     	movq	$0x10, 0x160(%rbp)
140112e62:     	movl	$0x0, 0x180(%rbp)
140112e6c:     	movw	$0x1, 0x184(%rbp)
140112e75:     	movl	$0x0, 0x188(%rbp)
140112e7f:     	movzbl	0x3521d83(%rip), %eax   # 0x143634c09
140112e86:     	movl	$0x178, %ecx            # imm = 0x178
140112e8b:     	movl	$0x8, %edx
140112e90:     	callq	0x140613c10 <.text+0x612c10>
140112e95:     	testq	%rax, %rax
140112e98:     	je	0x140112f35 <.text+0x111f35>
140112e9e:     	movq	%rax, %rdi
140112ea1:     	leaq	0x18(%rbp), %rdx
140112ea5:     	movl	$0x178, %r8d            # imm = 0x178
140112eab:     	movq	%rax, %rcx
140112eae:     	callq	0x142923c00 <.text+0x2922c00>
140112eb3:     	leaq	0x283d346(%rip), %r8    # 0x142950200
140112eba:     	leaq	-0x58(%rbp), %rcx
140112ebe:     	movq	%rdi, %rdx
140112ec1:     	callq	0x140ce4b10 <.text+0xce3b10>
140112ec6:     	movb	%bl, 0x18(%rbp)
140112ec9:     	movq	0x35194a0(%rip), %rcx   # 0x14362c370
140112ed0:     	leaq	0x28180c1(%rip), %r8    # 0x14292af98
140112ed7:     	leaq	0x18(%rbp), %rdx
140112edb:     	callq	0x140364d20 <.text+0x363d20>
140112ee0:     	movq	%rdx, %r8
140112ee3:     	leaq	-0x58(%rbp), %rcx
140112ee7:     	movq	%rax, %rdx
140112eea:     	callq	0x1400d68c0 <.text+0xd58c0>
140112eef:     	movups	0x8(%rbp), %xmm0
140112ef3:     	movups	%xmm0, 0x60(%rsi)
140112ef7:     	movups	-0x8(%rbp), %xmm0
140112efb:     	movups	%xmm0, 0x50(%rsi)
140112eff:     	movups	-0x18(%rbp), %xmm0
140112f03:     	movups	%xmm0, 0x40(%rsi)
140112f07:     	movups	-0x58(%rbp), %xmm0
140112f0b:     	movups	-0x48(%rbp), %xmm1
140112f0f:     	movups	-0x38(%rbp), %xmm2
140112f13:     	movups	-0x28(%rbp), %xmm3
140112f17:     	movups	%xmm3, 0x30(%rsi)
140112f1b:     	movups	%xmm2, 0x20(%rsi)
140112f1f:     	movups	%xmm1, 0x10(%rsi)
140112f23:     	movups	%xmm0, (%rsi)
140112f26:     	movq	%rsi, %rax
140112f29:     	addq	$0x218, %rsp            # imm = 0x218
140112f30:     	popq	%rbx
140112f31:     	popq	%rdi
140112f32:     	popq	%rsi
140112f33:     	popq	%rbp
140112f34:     	retq
140112f35:     	movl	$0x8, %ecx
140112f3a:     	movl	$0x178, %edx            # imm = 0x178
140112f3f:     	callq	0x1428d9000 <.text+0x28d8000>
140112f44:     	ud2
