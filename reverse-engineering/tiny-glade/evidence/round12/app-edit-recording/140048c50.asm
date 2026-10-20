
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140048c50:     	pushq	%rbp
140048c51:     	pushq	%rsi
140048c52:     	pushq	%rdi
140048c53:     	subq	$0x190, %rsp            # imm = 0x190
140048c5a:     	leaq	0x80(%rsp), %rbp
140048c62:     	movq	$-0x2, 0x108(%rbp)
140048c6d:     	movq	%rcx, %rsi
140048c70:     	xorps	%xmm0, %xmm0
140048c73:     	movups	%xmm0, -0x18(%rbp)
140048c77:     	movups	%xmm0, (%rbp)
140048c7b:     	movups	%xmm0, 0x18(%rbp)
140048c7f:     	movups	%xmm0, 0x30(%rbp)
140048c83:     	movups	%xmm0, 0x48(%rbp)
140048c87:     	movl	$0x0, 0x58(%rbp)
140048c8e:     	movups	%xmm0, 0x80(%rbp)
140048c95:     	movups	%xmm0, 0x98(%rbp)
140048c9c:     	movups	%xmm0, 0xb0(%rbp)
140048ca3:     	movups	%xmm0, 0xc8(%rbp)
140048caa:     	movups	%xmm0, 0xe0(%rbp)
140048cb1:     	movl	$0x0, 0xf0(%rbp)
140048cbb:     	movq	$0x0, -0x58(%rbp)
140048cc3:     	movq	$0x0, -0x38(%rbp)
140048ccb:     	movq	$0x8, -0x30(%rbp)
140048cd3:     	movq	$0x0, -0x28(%rbp)
140048cdb:     	movq	$0x10, -0x20(%rbp)
140048ce3:     	movq	$0x10, -0x8(%rbp)
140048ceb:     	movq	$0x10, 0x10(%rbp)
140048cf3:     	movq	$0x10, 0x28(%rbp)
140048cfb:     	movq	$0x10, 0x40(%rbp)
140048d03:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140048d0d:     	movq	%rax, 0x60(%rbp)
140048d11:     	leaq	0x28e5695(%rip), %rax   # 0x14292e3ad
140048d18:     	movq	%rax, 0x68(%rbp)
140048d1c:     	movq	$0x4f, 0x70(%rbp)
140048d24:     	movq	$0x10, 0x78(%rbp)
140048d2c:     	movq	$0x10, 0x90(%rbp)
140048d37:     	movq	$0x10, 0xa8(%rbp)
140048d42:     	movq	$0x10, 0xc0(%rbp)
140048d4d:     	movq	$0x10, 0xd8(%rbp)
140048d58:     	movl	$0x0, 0xf8(%rbp)
140048d62:     	movw	$0x1, 0xfc(%rbp)
140048d6b:     	movl	$0x0, 0x100(%rbp)
140048d75:     	movzbl	0x35ebe8d(%rip), %eax   # 0x143634c09
140048d7c:     	movl	$0x160, %ecx            # imm = 0x160
140048d81:     	movl	$0x8, %edx
140048d86:     	callq	0x140613c10 <.text+0x612c10>
140048d8b:     	testq	%rax, %rax
140048d8e:     	je	0x140048dc5 <.text+0x47dc5>
140048d90:     	movq	%rax, %rdi
140048d93:     	leaq	-0x58(%rbp), %rdx
140048d97:     	movl	$0x160, %r8d            # imm = 0x160
140048d9d:     	movq	%rax, %rcx
140048da0:     	callq	0x142923c00 <.text+0x2922c00>
140048da5:     	leaq	0x28e53bc(%rip), %r8    # 0x14292e168
140048dac:     	movq	%rsi, %rcx
140048daf:     	movq	%rdi, %rdx
140048db2:     	callq	0x140ce4b10 <.text+0xce3b10>
140048db7:     	movq	%rsi, %rax
140048dba:     	addq	$0x190, %rsp            # imm = 0x190
140048dc1:     	popq	%rdi
140048dc2:     	popq	%rsi
140048dc3:     	popq	%rbp
140048dc4:     	retq
140048dc5:     	movl	$0x8, %ecx
140048dca:     	movl	$0x160, %edx            # imm = 0x160
140048dcf:     	callq	0x1428d9000 <.text+0x28d8000>
140048dd4:     	ud2
