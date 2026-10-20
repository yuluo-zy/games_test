
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405389b0:     	pushq	%rbp
1405389b1:     	pushq	%rsi
1405389b2:     	pushq	%rdi
1405389b3:     	pushq	%rbx
1405389b4:     	subq	$0x598, %rsp            # imm = 0x598
1405389bb:     	leaq	0x80(%rsp), %rbp
1405389c3:     	movq	$-0x2, 0x510(%rbp)
1405389ce:     	movq	%rcx, %rsi
1405389d1:     	movzbl	0x30fc231(%rip), %eax   # 0x143634c09
1405389d8:     	movl	$0x230, %ecx            # imm = 0x230
1405389dd:     	movl	$0x8, %edx
1405389e2:     	callq	0x140613c10 <.text+0x612c10>
1405389e7:     	testq	%rax, %rax
1405389ea:     	je	0x140538daa <.text+0x537daa>
1405389f0:     	movq	%rax, 0x508(%rbp)
1405389f7:     	xorps	%xmm0, %xmm0
1405389fa:     	movups	%xmm0, -0x40(%rbp)
1405389fe:     	movups	%xmm0, -0x28(%rbp)
140538a02:     	movups	%xmm0, -0x10(%rbp)
140538a06:     	movups	%xmm0, 0x8(%rbp)
140538a0a:     	movups	%xmm0, 0x20(%rbp)
140538a0e:     	movl	$0x0, 0x30(%rbp)
140538a15:     	movups	%xmm0, 0x58(%rbp)
140538a19:     	movups	%xmm0, 0x70(%rbp)
140538a1d:     	movups	%xmm0, 0x88(%rbp)
140538a24:     	movups	%xmm0, 0xa0(%rbp)
140538a2b:     	movups	%xmm0, 0xb8(%rbp)
140538a32:     	movl	$0x0, 0xc8(%rbp)
140538a3c:     	movq	$0x0, -0x60(%rbp)
140538a44:     	movq	$0x8, -0x58(%rbp)
140538a4c:     	movq	$0x0, -0x50(%rbp)
140538a54:     	movq	$0x10, -0x48(%rbp)
140538a5c:     	movq	$0x10, -0x30(%rbp)
140538a64:     	movq	$0x10, -0x18(%rbp)
140538a6c:     	movq	$0x10, (%rbp)
140538a74:     	movq	$0x10, 0x18(%rbp)
140538a7c:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140538a86:     	movq	%rbx, 0x38(%rbp)
140538a8a:     	leaq	0x24bdb8f(%rip), %rax   # 0x1429f6620
140538a91:     	movq	%rax, 0x40(%rbp)
140538a95:     	movq	$0x4d, 0x48(%rbp)
140538a9d:     	movq	$0x10, 0x50(%rbp)
140538aa5:     	movq	$0x10, 0x68(%rbp)
140538aad:     	movq	$0x10, 0x80(%rbp)
140538ab8:     	movq	$0x10, 0x98(%rbp)
140538ac3:     	movq	$0x10, 0xb0(%rbp)
140538ace:     	movl	$0x0, 0xd0(%rbp)
140538ad8:     	movw	$0x1, 0xd4(%rbp)
140538ae1:     	movq	%rbx, 0x1c8(%rbp)
140538ae8:     	movl	$0x0, 0x340(%rbp)
140538af2:     	movzbl	0x30fc110(%rip), %eax   # 0x143634c09
140538af9:     	movl	$0x3a8, %ecx            # imm = 0x3A8
140538afe:     	movl	$0x8, %edx
140538b03:     	callq	0x140613c10 <.text+0x612c10>
140538b08:     	testq	%rax, %rax
140538b0b:     	je	0x140538db9 <.text+0x537db9>
140538b11:     	movq	%rax, %rdi
140538b14:     	leaq	-0x60(%rbp), %rdx
140538b18:     	movl	$0x3a8, %r8d            # imm = 0x3A8
140538b1e:     	movq	%rax, %rcx
140538b21:     	callq	0x142923c00 <.text+0x2922c00>
140538b26:     	leaq	0x24bd7eb(%rip), %r8    # 0x1429f6318
140538b2d:     	leaq	0x498(%rbp), %rcx
140538b34:     	movq	%rdi, %rdx
140538b37:     	callq	0x140ce4b10 <.text+0xce3b10>
140538b3c:     	leaq	0x428(%rbp), %rcx
140538b43:     	callq	0x1404e7060 <.text+0x4e6060>
140538b48:     	leaq	0x3b8(%rbp), %rcx
140538b4f:     	callq	0x140176fe0 <.text+0x175fe0>
140538b54:     	leaq	0x348(%rbp), %rcx
140538b5b:     	callq	0x140293720 <.text+0x292720>
140538b60:     	leaq	-0x60(%rbp), %rcx
140538b64:     	callq	0x1402cfa60 <.text+0x2cea60>
140538b69:     	movups	0x4f8(%rbp), %xmm0
140538b70:     	movq	0x508(%rbp), %rax
140538b77:     	movups	%xmm0, 0x60(%rax)
140538b7b:     	movups	0x4e8(%rbp), %xmm0
140538b82:     	movups	%xmm0, 0x50(%rax)
140538b86:     	movups	0x4d8(%rbp), %xmm0
140538b8d:     	movups	%xmm0, 0x40(%rax)
140538b91:     	movups	0x498(%rbp), %xmm0
140538b98:     	movups	0x4a8(%rbp), %xmm1
140538b9f:     	movups	0x4b8(%rbp), %xmm2
140538ba6:     	movups	0x4c8(%rbp), %xmm3
140538bad:     	movups	%xmm3, 0x30(%rax)
140538bb1:     	movups	%xmm2, 0x20(%rax)
140538bb5:     	movups	%xmm1, 0x10(%rax)
140538bb9:     	movups	%xmm0, (%rax)
140538bbc:     	movups	0x428(%rbp), %xmm0
140538bc3:     	movups	0x438(%rbp), %xmm1
140538bca:     	movups	0x448(%rbp), %xmm2
140538bd1:     	movups	0x458(%rbp), %xmm3
140538bd8:     	movups	%xmm0, 0x70(%rax)
140538bdc:     	movups	%xmm1, 0x80(%rax)
140538be3:     	movups	%xmm2, 0x90(%rax)
140538bea:     	movups	%xmm3, 0xa0(%rax)
140538bf1:     	movups	0x468(%rbp), %xmm0
140538bf8:     	movups	%xmm0, 0xb0(%rax)
140538bff:     	movups	0x478(%rbp), %xmm0
140538c06:     	movups	%xmm0, 0xc0(%rax)
140538c0d:     	movups	0x488(%rbp), %xmm0
140538c14:     	movups	%xmm0, 0xd0(%rax)
140538c1b:     	movups	0x418(%rbp), %xmm0
140538c22:     	movups	%xmm0, 0x140(%rax)
140538c29:     	movups	0x408(%rbp), %xmm0
140538c30:     	movups	%xmm0, 0x130(%rax)
140538c37:     	movups	0x3f8(%rbp), %xmm0
140538c3e:     	movups	%xmm0, 0x120(%rax)
140538c45:     	movups	0x3b8(%rbp), %xmm0
140538c4c:     	movups	0x3c8(%rbp), %xmm1
140538c53:     	movups	0x3d8(%rbp), %xmm2
140538c5a:     	movups	0x3e8(%rbp), %xmm3
140538c61:     	movups	%xmm3, 0x110(%rax)
140538c68:     	movups	%xmm2, 0x100(%rax)
140538c6f:     	movups	%xmm1, 0xf0(%rax)
140538c76:     	movups	%xmm0, 0xe0(%rax)
140538c7d:     	movups	0x348(%rbp), %xmm0
140538c84:     	movups	0x358(%rbp), %xmm1
140538c8b:     	movups	0x368(%rbp), %xmm2
140538c92:     	movups	0x378(%rbp), %xmm3
140538c99:     	movups	%xmm0, 0x150(%rax)
140538ca0:     	movups	%xmm1, 0x160(%rax)
140538ca7:     	movups	%xmm2, 0x170(%rax)
140538cae:     	movups	%xmm3, 0x180(%rax)
140538cb5:     	movups	0x388(%rbp), %xmm0
140538cbc:     	movups	%xmm0, 0x190(%rax)
140538cc3:     	movups	0x398(%rbp), %xmm0
140538cca:     	movups	%xmm0, 0x1a0(%rax)
140538cd1:     	movups	0x3a8(%rbp), %xmm0
140538cd8:     	movups	%xmm0, 0x1b0(%rax)
140538cdf:     	movups	-0x60(%rbp), %xmm0
140538ce3:     	movups	-0x50(%rbp), %xmm1
140538ce7:     	movups	-0x40(%rbp), %xmm2
140538ceb:     	movups	-0x30(%rbp), %xmm3
140538cef:     	movups	%xmm0, 0x1c0(%rax)
140538cf6:     	movups	%xmm1, 0x1d0(%rax)
140538cfd:     	movups	%xmm2, 0x1e0(%rax)
140538d04:     	movups	%xmm3, 0x1f0(%rax)
140538d0b:     	movups	-0x20(%rbp), %xmm0
140538d0f:     	movups	%xmm0, 0x200(%rax)
140538d16:     	movups	(%rbp), %xmm0
140538d1a:     	movups	%xmm0, 0x220(%rax)
140538d21:     	movups	-0x10(%rbp), %xmm0
140538d25:     	movups	%xmm0, 0x210(%rax)
140538d2c:     	movq	$0x5, -0x58(%rbp)
140538d34:     	movq	%rax, -0x50(%rbp)
140538d38:     	movq	$0x5, -0x48(%rbp)
140538d40:     	movq	$0x0, -0x40(%rbp)
140538d48:     	movq	$0x8, -0x38(%rbp)
140538d50:     	xorps	%xmm0, %xmm0
140538d53:     	movups	%xmm0, -0x30(%rbp)
140538d57:     	movq	%rbx, -0x60(%rbp)
140538d5b:     	leaq	-0x28(%rbp), %rcx
140538d5f:     	callq	0x1401705d0 <.text+0x16f5d0>
140538d64:     	movups	(%rbp), %xmm0
140538d68:     	movups	%xmm0, 0x60(%rsi)
140538d6c:     	movups	-0x10(%rbp), %xmm0
140538d70:     	movups	%xmm0, 0x50(%rsi)
140538d74:     	movups	-0x20(%rbp), %xmm0
140538d78:     	movups	%xmm0, 0x40(%rsi)
140538d7c:     	movups	-0x60(%rbp), %xmm0
140538d80:     	movups	-0x50(%rbp), %xmm1
140538d84:     	movups	-0x40(%rbp), %xmm2
140538d88:     	movups	-0x30(%rbp), %xmm3
140538d8c:     	movups	%xmm3, 0x30(%rsi)
140538d90:     	movups	%xmm2, 0x20(%rsi)
140538d94:     	movups	%xmm1, 0x10(%rsi)
140538d98:     	movups	%xmm0, (%rsi)
140538d9b:     	movq	%rsi, %rax
140538d9e:     	addq	$0x598, %rsp            # imm = 0x598
140538da5:     	popq	%rbx
140538da6:     	popq	%rdi
140538da7:     	popq	%rsi
140538da8:     	popq	%rbp
140538da9:     	retq
140538daa:     	movl	$0x8, %ecx
140538daf:     	movl	$0x230, %edx            # imm = 0x230
140538db4:     	callq	0x1428d9000 <.text+0x28d8000>
140538db9:     	movl	$0x8, %ecx
140538dbe:     	movl	$0x3a8, %edx            # imm = 0x3A8
140538dc3:     	callq	0x1428d9000 <.text+0x28d8000>
140538dc8:     	ud2
