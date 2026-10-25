
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405bfb20:     	pushq	%rbp
1405bfb21:     	pushq	%rsi
1405bfb22:     	pushq	%rdi
1405bfb23:     	subq	$0x190, %rsp            # imm = 0x190
1405bfb2a:     	leaq	0x80(%rsp), %rbp
1405bfb32:     	movq	$-0x2, 0x108(%rbp)
1405bfb3d:     	movq	%rcx, %rsi
1405bfb40:     	xorps	%xmm0, %xmm0
1405bfb43:     	movups	%xmm0, -0x18(%rbp)
1405bfb47:     	movups	%xmm0, (%rbp)
1405bfb4b:     	movups	%xmm0, 0x18(%rbp)
1405bfb4f:     	movups	%xmm0, 0x30(%rbp)
1405bfb53:     	movups	%xmm0, 0x48(%rbp)
1405bfb57:     	movl	$0x0, 0x58(%rbp)
1405bfb5e:     	movups	%xmm0, 0x80(%rbp)
1405bfb65:     	movups	%xmm0, 0x98(%rbp)
1405bfb6c:     	movups	%xmm0, 0xb0(%rbp)
1405bfb73:     	movups	%xmm0, 0xc8(%rbp)
1405bfb7a:     	movups	%xmm0, 0xe0(%rbp)
1405bfb81:     	movl	$0x0, 0xf0(%rbp)
1405bfb8b:     	movq	$0x0, -0x58(%rbp)
1405bfb93:     	movq	$0x0, -0x38(%rbp)
1405bfb9b:     	movq	$0x8, -0x30(%rbp)
1405bfba3:     	movq	$0x0, -0x28(%rbp)
1405bfbab:     	movq	$0x10, -0x20(%rbp)
1405bfbb3:     	movq	$0x10, -0x8(%rbp)
1405bfbbb:     	movq	$0x10, 0x10(%rbp)
1405bfbc3:     	movq	$0x10, 0x28(%rbp)
1405bfbcb:     	movq	$0x10, 0x40(%rbp)
1405bfbd3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405bfbdd:     	movq	%rax, 0x60(%rbp)
1405bfbe1:     	leaq	0x24459d5(%rip), %rax   # 0x142a055bd
1405bfbe8:     	movq	%rax, 0x68(%rbp)
1405bfbec:     	movq	$0x45, 0x70(%rbp)
1405bfbf4:     	movq	$0x10, 0x78(%rbp)
1405bfbfc:     	movq	$0x10, 0x90(%rbp)
1405bfc07:     	movq	$0x10, 0xa8(%rbp)
1405bfc12:     	movq	$0x10, 0xc0(%rbp)
1405bfc1d:     	movq	$0x10, 0xd8(%rbp)
1405bfc28:     	movl	$0x0, 0xf8(%rbp)
1405bfc32:     	movw	$0x1, 0xfc(%rbp)
1405bfc3b:     	movl	$0x0, 0x100(%rbp)
1405bfc45:     	movzbl	0x3074fbd(%rip), %eax   # 0x143634c09
1405bfc4c:     	movl	$0x160, %ecx            # imm = 0x160
1405bfc51:     	movl	$0x8, %edx
1405bfc56:     	callq	0x140613c10 <.text+0x612c10>
1405bfc5b:     	testq	%rax, %rax
1405bfc5e:     	je	0x1405bfc95 <.text+0x5bec95>
1405bfc60:     	movq	%rax, %rdi
1405bfc63:     	leaq	-0x58(%rbp), %rdx
1405bfc67:     	movl	$0x160, %r8d            # imm = 0x160
1405bfc6d:     	movq	%rax, %rcx
1405bfc70:     	callq	0x142923c00 <.text+0x2922c00>
1405bfc75:     	leaq	0x2445714(%rip), %r8    # 0x142a05390
1405bfc7c:     	movq	%rsi, %rcx
1405bfc7f:     	movq	%rdi, %rdx
1405bfc82:     	callq	0x140ce4b10 <.text+0xce3b10>
1405bfc87:     	movq	%rsi, %rax
1405bfc8a:     	addq	$0x190, %rsp            # imm = 0x190
1405bfc91:     	popq	%rdi
1405bfc92:     	popq	%rsi
1405bfc93:     	popq	%rbp
1405bfc94:     	retq
1405bfc95:     	movl	$0x8, %ecx
1405bfc9a:     	movl	$0x160, %edx            # imm = 0x160
1405bfc9f:     	callq	0x1428d9000 <.text+0x28d8000>
1405bfca4:     	ud2
