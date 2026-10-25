
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404e4a70:     	pushq	%rbp
1404e4a71:     	pushq	%rsi
1404e4a72:     	pushq	%rdi
1404e4a73:     	subq	$0x1f0, %rsp            # imm = 0x1F0
1404e4a7a:     	leaq	0x80(%rsp), %rbp
1404e4a82:     	movq	$-0x2, 0x168(%rbp)
1404e4a8d:     	movq	%rcx, %rsi
1404e4a90:     	xorps	%xmm0, %xmm0
1404e4a93:     	movups	%xmm0, 0x48(%rbp)
1404e4a97:     	movups	%xmm0, 0x60(%rbp)
1404e4a9b:     	movups	%xmm0, 0x78(%rbp)
1404e4a9f:     	movups	%xmm0, 0x90(%rbp)
1404e4aa6:     	movups	%xmm0, 0xa8(%rbp)
1404e4aad:     	movl	$0x0, 0xb8(%rbp)
1404e4ab7:     	movups	%xmm0, 0xe0(%rbp)
1404e4abe:     	movups	%xmm0, 0xf8(%rbp)
1404e4ac5:     	movups	%xmm0, 0x110(%rbp)
1404e4acc:     	movups	%xmm0, 0x128(%rbp)
1404e4ad3:     	movups	%xmm0, 0x140(%rbp)
1404e4ada:     	movl	$0x0, 0x150(%rbp)
1404e4ae4:     	movq	$0x0, -0x58(%rbp)
1404e4aec:     	movq	$0x0, 0x28(%rbp)
1404e4af4:     	movq	$0x8, 0x30(%rbp)
1404e4afc:     	movq	$0x0, 0x38(%rbp)
1404e4b04:     	movq	$0x10, 0x40(%rbp)
1404e4b0c:     	movq	$0x10, 0x58(%rbp)
1404e4b14:     	movq	$0x10, 0x70(%rbp)
1404e4b1c:     	movq	$0x10, 0x88(%rbp)
1404e4b27:     	movq	$0x10, 0xa0(%rbp)
1404e4b32:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1404e4b3c:     	movq	%rax, 0xc0(%rbp)
1404e4b43:     	leaq	0x25061d3(%rip), %rax   # 0x1429ead1d
1404e4b4a:     	movq	%rax, 0xc8(%rbp)
1404e4b51:     	movq	$0x35, 0xd0(%rbp)
1404e4b5c:     	movq	$0x10, 0xd8(%rbp)
1404e4b67:     	movq	$0x10, 0xf0(%rbp)
1404e4b72:     	movq	$0x10, 0x108(%rbp)
1404e4b7d:     	movq	$0x10, 0x120(%rbp)
1404e4b88:     	movq	$0x10, 0x138(%rbp)
1404e4b93:     	movl	$0x0, 0x158(%rbp)
1404e4b9d:     	movw	$0x1, 0x15c(%rbp)
1404e4ba6:     	movl	$0x0, 0x160(%rbp)
1404e4bb0:     	movzbl	0x3150052(%rip), %eax   # 0x143634c09
1404e4bb7:     	movl	$0x1c0, %ecx            # imm = 0x1C0
1404e4bbc:     	movl	$0x8, %edx
1404e4bc1:     	callq	0x140613c10 <.text+0x612c10>
1404e4bc6:     	testq	%rax, %rax
1404e4bc9:     	je	0x1404e4c00 <.text+0x4e3c00>
1404e4bcb:     	movq	%rax, %rdi
1404e4bce:     	leaq	-0x58(%rbp), %rdx
1404e4bd2:     	movl	$0x1c0, %r8d            # imm = 0x1C0
1404e4bd8:     	movq	%rax, %rcx
1404e4bdb:     	callq	0x142923c00 <.text+0x2922c00>
1404e4be0:     	leaq	0x2505ee1(%rip), %r8    # 0x1429eaac8
1404e4be7:     	movq	%rsi, %rcx
1404e4bea:     	movq	%rdi, %rdx
1404e4bed:     	callq	0x140ce4b10 <.text+0xce3b10>
1404e4bf2:     	movq	%rsi, %rax
1404e4bf5:     	addq	$0x1f0, %rsp            # imm = 0x1F0
1404e4bfc:     	popq	%rdi
1404e4bfd:     	popq	%rsi
1404e4bfe:     	popq	%rbp
1404e4bff:     	retq
1404e4c00:     	movl	$0x8, %ecx
1404e4c05:     	movl	$0x1c0, %edx            # imm = 0x1C0
1404e4c0a:     	callq	0x1428d9000 <.text+0x28d8000>
1404e4c0f:     	ud2
