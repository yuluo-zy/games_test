
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403cbe60:     	pushq	%rbp
1403cbe61:     	pushq	%rsi
1403cbe62:     	pushq	%rdi
1403cbe63:     	subq	$0x1f0, %rsp            # imm = 0x1F0
1403cbe6a:     	leaq	0x80(%rsp), %rbp
1403cbe72:     	movq	$-0x2, 0x168(%rbp)
1403cbe7d:     	movq	%rcx, %rsi
1403cbe80:     	xorps	%xmm0, %xmm0
1403cbe83:     	movups	%xmm0, 0x48(%rbp)
1403cbe87:     	movups	%xmm0, 0x60(%rbp)
1403cbe8b:     	movups	%xmm0, 0x78(%rbp)
1403cbe8f:     	movups	%xmm0, 0x90(%rbp)
1403cbe96:     	movups	%xmm0, 0xa8(%rbp)
1403cbe9d:     	movl	$0x0, 0xb8(%rbp)
1403cbea7:     	movups	%xmm0, 0xe0(%rbp)
1403cbeae:     	movups	%xmm0, 0xf8(%rbp)
1403cbeb5:     	movups	%xmm0, 0x110(%rbp)
1403cbebc:     	movups	%xmm0, 0x128(%rbp)
1403cbec3:     	movups	%xmm0, 0x140(%rbp)
1403cbeca:     	movl	$0x0, 0x150(%rbp)
1403cbed4:     	movq	$0x0, -0x58(%rbp)
1403cbedc:     	movq	$0x0, 0x28(%rbp)
1403cbee4:     	movq	$0x8, 0x30(%rbp)
1403cbeec:     	movq	$0x0, 0x38(%rbp)
1403cbef4:     	movq	$0x10, 0x40(%rbp)
1403cbefc:     	movq	$0x10, 0x58(%rbp)
1403cbf04:     	movq	$0x10, 0x70(%rbp)
1403cbf0c:     	movq	$0x10, 0x88(%rbp)
1403cbf17:     	movq	$0x10, 0xa0(%rbp)
1403cbf22:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403cbf2c:     	movq	%rax, 0xc0(%rbp)
1403cbf33:     	leaq	0x2604d43(%rip), %rax   # 0x1429d0c7d
1403cbf3a:     	movq	%rax, 0xc8(%rbp)
1403cbf41:     	movq	$0x35, 0xd0(%rbp)
1403cbf4c:     	movq	$0x10, 0xd8(%rbp)
1403cbf57:     	movq	$0x10, 0xf0(%rbp)
1403cbf62:     	movq	$0x10, 0x108(%rbp)
1403cbf6d:     	movq	$0x10, 0x120(%rbp)
1403cbf78:     	movq	$0x10, 0x138(%rbp)
1403cbf83:     	movl	$0x0, 0x158(%rbp)
1403cbf8d:     	movw	$0x1, 0x15c(%rbp)
1403cbf96:     	movl	$0x0, 0x160(%rbp)
1403cbfa0:     	movzbl	0x3268c62(%rip), %eax   # 0x143634c09
1403cbfa7:     	movl	$0x1c0, %ecx            # imm = 0x1C0
1403cbfac:     	movl	$0x8, %edx
1403cbfb1:     	callq	0x140613c10 <.text+0x612c10>
1403cbfb6:     	testq	%rax, %rax
1403cbfb9:     	je	0x1403cbff0 <.text+0x3caff0>
1403cbfbb:     	movq	%rax, %rdi
1403cbfbe:     	leaq	-0x58(%rbp), %rdx
1403cbfc2:     	movl	$0x1c0, %r8d            # imm = 0x1C0
1403cbfc8:     	movq	%rax, %rcx
1403cbfcb:     	callq	0x142923c00 <.text+0x2922c00>
1403cbfd0:     	leaq	0x2604b29(%rip), %r8    # 0x1429d0b00
1403cbfd7:     	movq	%rsi, %rcx
1403cbfda:     	movq	%rdi, %rdx
1403cbfdd:     	callq	0x140ce4b10 <.text+0xce3b10>
1403cbfe2:     	movq	%rsi, %rax
1403cbfe5:     	addq	$0x1f0, %rsp            # imm = 0x1F0
1403cbfec:     	popq	%rdi
1403cbfed:     	popq	%rsi
1403cbfee:     	popq	%rbp
1403cbfef:     	retq
1403cbff0:     	movl	$0x8, %ecx
1403cbff5:     	movl	$0x1c0, %edx            # imm = 0x1C0
1403cbffa:     	callq	0x1428d9000 <.text+0x28d8000>
1403cbfff:     	ud2
