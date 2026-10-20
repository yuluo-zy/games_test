
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d0af0:     	pushq	%r15
1403d0af2:     	pushq	%r14
1403d0af4:     	pushq	%r13
1403d0af6:     	pushq	%r12
1403d0af8:     	pushq	%rsi
1403d0af9:     	pushq	%rdi
1403d0afa:     	pushq	%rbp
1403d0afb:     	pushq	%rbx
1403d0afc:     	subq	$0x58, %rsp
1403d0b00:     	movq	%rcx, 0x38(%rsp)
1403d0b05:     	leaq	0x2600c80(%rip), %rax   # 0x1429d178c
1403d0b0c:     	movq	%rax, 0x28(%rsp)
1403d0b11:     	leaq	0x28(%rsp), %rax
1403d0b16:     	movq	0x28(%rsp), %rcx
1403d0b1b:     	movabsq	$0x4a200c7e6cc64f5c, %rax # imm = 0x4A200C7E6CC64F5C
1403d0b25:     	movq	%rax, 0x28(%rsp)
1403d0b2a:     	leaq	0x28(%rsp), %rax
1403d0b2f:     	movq	0x28(%rsp), %rdx
1403d0b34:     	callq	0x1403cf310 <.text+0x3ce310>
1403d0b39:     	movq	(%rax), %rcx
1403d0b3c:     	movq	%rcx, 0x30(%rsp)
1403d0b41:     	movq	0x8(%rax), %rsi
1403d0b45:     	movq	0x10(%rax), %rbp
1403d0b49:     	movq	0x18(%rax), %r13
1403d0b4d:     	movq	0x20(%rax), %r12
1403d0b51:     	movq	0x28(%rax), %r15
1403d0b55:     	movq	0x30(%rax), %r14
1403d0b59:     	movq	0x38(%rax), %rbx
1403d0b5d:     	movl	0x40(%rax), %edi
1403d0b60:     	movzbl	0x32640a2(%rip), %eax   # 0x143634c09
1403d0b67:     	movl	$0x44, %ecx
1403d0b6c:     	movl	$0x1, %edx
1403d0b71:     	callq	0x140613c10 <.text+0x612c10>
1403d0b76:     	testq	%rax, %rax
1403d0b79:     	je	0x1403d0c4c <.text+0x3cfc4c>
1403d0b7f:     	xorl	$0x1c5217d1, %edi       # imm = 0x1C5217D1
1403d0b85:     	movabsq	$0x3224caaa5fb9abc, %rcx # imm = 0x3224CAAA5FB9ABC
1403d0b8f:     	xorq	%rcx, %rbx
1403d0b92:     	movabsq	$0x6b7c08028bbbfa3c, %rcx # imm = 0x6B7C08028BBBFA3C
1403d0b9c:     	xorq	%rcx, %r14
1403d0b9f:     	movabsq	$0x6b1af7459d71d8d0, %rcx # imm = 0x6B1AF7459D71D8D0
1403d0ba9:     	xorq	%rcx, %r15
1403d0bac:     	movabsq	$-0x39b0678fce7aa4fe, %rcx # imm = 0xC64F987031855B02
1403d0bb6:     	xorq	%rcx, %r12
1403d0bb9:     	movabsq	$0x2f5515b329d67cf7, %rcx # imm = 0x2F5515B329D67CF7
1403d0bc3:     	xorq	%rcx, %r13
1403d0bc6:     	movabsq	$0x377d14e71a67e740, %rcx # imm = 0x377D14E71A67E740
1403d0bd0:     	xorq	%rcx, %rbp
1403d0bd3:     	movabsq	$0x4fa408136071426d, %rcx # imm = 0x4FA408136071426D
1403d0bdd:     	xorq	%rcx, %rsi
1403d0be0:     	movabsq	$0x22a4a09645caa29a, %rcx # imm = 0x22A4A09645CAA29A
1403d0bea:     	movq	0x30(%rsp), %rdx
1403d0bef:     	xorq	%rcx, %rdx
1403d0bf2:     	movq	%rdx, (%rax)
1403d0bf5:     	movq	%rsi, 0x8(%rax)
1403d0bf9:     	movq	%rbp, 0x10(%rax)
1403d0bfd:     	movq	%r13, 0x18(%rax)
1403d0c01:     	movq	%r12, 0x20(%rax)
1403d0c05:     	movq	%r15, 0x28(%rax)
1403d0c09:     	movq	%r14, 0x30(%rax)
1403d0c0d:     	movq	%rbx, 0x38(%rax)
1403d0c11:     	movl	%edi, 0x40(%rax)
1403d0c14:     	movq	$0x44, 0x40(%rsp)
1403d0c1d:     	movq	%rax, 0x48(%rsp)
1403d0c22:     	movq	$0x44, 0x50(%rsp)
1403d0c2b:     	leaq	0x40(%rsp), %rdx
1403d0c30:     	movq	0x38(%rsp), %rcx
1403d0c35:     	callq	0x14026d580 <.text+0x26c580>
1403d0c3a:     	nop
1403d0c3b:     	addq	$0x58, %rsp
1403d0c3f:     	popq	%rbx
1403d0c40:     	popq	%rbp
1403d0c41:     	popq	%rdi
1403d0c42:     	popq	%rsi
1403d0c43:     	popq	%r12
1403d0c45:     	popq	%r13
1403d0c47:     	popq	%r14
1403d0c49:     	popq	%r15
1403d0c4b:     	retq
1403d0c4c:     	leaq	0x2600de5(%rip), %r8    # 0x1429d1a38
1403d0c53:     	movl	$0x1, %ecx
1403d0c58:     	movl	$0x44, %edx
1403d0c5d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403d0c62:     	int3
