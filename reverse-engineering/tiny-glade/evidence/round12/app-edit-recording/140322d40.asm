
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140322d40:     	pushq	%rbp
140322d41:     	pushq	%rsi
140322d42:     	pushq	%rdi
140322d43:     	pushq	%rbx
140322d44:     	subq	$0x238, %rsp            # imm = 0x238
140322d4b:     	leaq	0x80(%rsp), %rbp
140322d53:     	movq	$-0x2, 0x1b0(%rbp)
140322d5e:     	movl	%edx, %ebx
140322d60:     	movq	%rcx, %rsi
140322d63:     	xorps	%xmm0, %xmm0
140322d66:     	movups	%xmm0, 0x20(%rbp)
140322d6a:     	movups	%xmm0, 0x38(%rbp)
140322d6e:     	movups	%xmm0, 0x50(%rbp)
140322d72:     	movups	%xmm0, 0x68(%rbp)
140322d76:     	movups	%xmm0, 0x80(%rbp)
140322d7d:     	movl	$0x0, 0x90(%rbp)
140322d87:     	movups	%xmm0, 0xb8(%rbp)
140322d8e:     	movups	%xmm0, 0xd0(%rbp)
140322d95:     	movups	%xmm0, 0xe8(%rbp)
140322d9c:     	movups	%xmm0, 0x100(%rbp)
140322da3:     	movups	%xmm0, 0x118(%rbp)
140322daa:     	movl	$0x0, 0x128(%rbp)
140322db4:     	movq	$0x0, -0x58(%rbp)
140322dbc:     	movq	$0x0, (%rbp)
140322dc4:     	movq	$0x8, 0x8(%rbp)
140322dcc:     	movq	$0x0, 0x10(%rbp)
140322dd4:     	movq	$0x10, 0x18(%rbp)
140322ddc:     	movq	$0x10, 0x30(%rbp)
140322de4:     	movq	$0x10, 0x48(%rbp)
140322dec:     	movq	$0x10, 0x60(%rbp)
140322df4:     	movq	$0x10, 0x78(%rbp)
140322dfc:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140322e06:     	movq	%rax, 0x98(%rbp)
140322e0d:     	leaq	0x269a7a9(%rip), %rax   # 0x1429bd5bd
140322e14:     	movq	%rax, 0xa0(%rbp)
140322e1b:     	movq	$0x4a, 0xa8(%rbp)
140322e26:     	movq	$0x10, 0xb0(%rbp)
140322e31:     	movq	$0x10, 0xc8(%rbp)
140322e3c:     	movq	$0x10, 0xe0(%rbp)
140322e47:     	movq	$0x10, 0xf8(%rbp)
140322e52:     	movq	$0x10, 0x110(%rbp)
140322e5d:     	movl	$0x0, 0x130(%rbp)
140322e67:     	movw	$0x1, 0x134(%rbp)
140322e70:     	movl	$0x0, 0x138(%rbp)
140322e7a:     	movzbl	0x3311d88(%rip), %eax   # 0x143634c09
140322e81:     	movl	$0x198, %ecx            # imm = 0x198
140322e86:     	movl	$0x8, %edx
140322e8b:     	callq	0x140613c10 <.text+0x612c10>
140322e90:     	testq	%rax, %rax
140322e93:     	je	0x140322f4b <.text+0x321f4b>
140322e99:     	movq	%rax, %rdi
140322e9c:     	leaq	-0x58(%rbp), %rdx
140322ea0:     	movl	$0x198, %r8d            # imm = 0x198
140322ea6:     	movq	%rax, %rcx
140322ea9:     	callq	0x142923c00 <.text+0x2922c00>
140322eae:     	leaq	0x269a473(%rip), %r8    # 0x1429bd328
140322eb5:     	leaq	0x140(%rbp), %rcx
140322ebc:     	movq	%rdi, %rdx
140322ebf:     	callq	0x140ce4b10 <.text+0xce3b10>
140322ec4:     	movb	%bl, -0x58(%rbp)
140322ec7:     	movq	0x33094a2(%rip), %rcx   # 0x14362c370
140322ece:     	leaq	0x26080c3(%rip), %r8    # 0x14292af98
140322ed5:     	leaq	-0x58(%rbp), %rdx
140322ed9:     	callq	0x140364d20 <.text+0x363d20>
140322ede:     	movq	%rdx, %r8
140322ee1:     	leaq	0x140(%rbp), %rcx
140322ee8:     	movq	%rax, %rdx
140322eeb:     	callq	0x1400d68c0 <.text+0xd58c0>
140322ef0:     	movups	0x1a0(%rbp), %xmm0
140322ef7:     	movups	%xmm0, 0x60(%rsi)
140322efb:     	movups	0x190(%rbp), %xmm0
140322f02:     	movups	%xmm0, 0x50(%rsi)
140322f06:     	movups	0x180(%rbp), %xmm0
140322f0d:     	movups	%xmm0, 0x40(%rsi)
140322f11:     	movups	0x140(%rbp), %xmm0
140322f18:     	movups	0x150(%rbp), %xmm1
140322f1f:     	movups	0x160(%rbp), %xmm2
140322f26:     	movups	0x170(%rbp), %xmm3
140322f2d:     	movups	%xmm3, 0x30(%rsi)
140322f31:     	movups	%xmm2, 0x20(%rsi)
140322f35:     	movups	%xmm1, 0x10(%rsi)
140322f39:     	movups	%xmm0, (%rsi)
140322f3c:     	movq	%rsi, %rax
140322f3f:     	addq	$0x238, %rsp            # imm = 0x238
140322f46:     	popq	%rbx
140322f47:     	popq	%rdi
140322f48:     	popq	%rsi
140322f49:     	popq	%rbp
140322f4a:     	retq
140322f4b:     	movl	$0x8, %ecx
140322f50:     	movl	$0x198, %edx            # imm = 0x198
140322f55:     	callq	0x1428d9000 <.text+0x28d8000>
140322f5a:     	ud2
