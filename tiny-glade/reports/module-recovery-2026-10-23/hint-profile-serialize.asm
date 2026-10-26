
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a658a0:     	pushq	%rbp
140a658a1:     	pushq	%rsi
140a658a2:     	pushq	%rdi
140a658a3:     	subq	$0xd0, %rsp
140a658aa:     	leaq	0x80(%rsp), %rbp
140a658b2:     	movq	$-0x2, 0x48(%rbp)
140a658ba:     	movq	%r8, %rax
140a658bd:     	movq	%rdx, %rdi
140a658c0:     	movq	%rcx, %rsi
140a658c3:     	movq	$0x5, 0x20(%rsp)
140a658cc:     	leaq	0x206eca5(%rip), %r8    # 0x142ad4578
140a658d3:     	leaq	-0x10(%rbp), %rcx
140a658d7:     	movl	$0xb, %r9d
140a658dd:     	movq	%rax, %rdx
140a658e0:     	callq	0x1413132c0 <.text+0x13122c0>
140a658e5:     	movzbl	-0x10(%rbp), %eax
140a658e9:     	cmpb	$0x32, %al
140a658eb:     	jne	0x140a65a29 <.text+0xa64a29>
140a658f1:     	movq	0x8(%rbp), %rax
140a658f5:     	movups	-0x8(%rbp), %xmm0
140a658f9:     	movaps	%xmm0, -0x30(%rbp)
140a658fd:     	movq	%rax, -0x20(%rbp)
140a65901:     	leaq	0x1f4(%rdi), %rax
140a65908:     	movb	$0x1, 0x47(%rbp)
140a6590c:     	movq	%rax, 0x20(%rsp)
140a65911:     	leaq	0x206ec6b(%rip), %r8    # 0x142ad4583
140a65918:     	leaq	-0x10(%rbp), %rcx
140a6591c:     	leaq	-0x30(%rbp), %rdx
140a65920:     	movl	$0x8, %r9d
140a65926:     	callq	0x141311310 <.text+0x1310310>
140a6592b:     	movzbl	-0x10(%rbp), %eax
140a6592f:     	cmpb	$0x32, %al
140a65931:     	jne	0x140a65a65 <.text+0xa64a65>
140a65937:     	leaq	0x1e0(%rdi), %rax
140a6593e:     	movb	$0x1, 0x47(%rbp)
140a65942:     	movq	%rax, 0x20(%rsp)
140a65947:     	leaq	0x206ec3d(%rip), %r8    # 0x142ad458b
140a6594e:     	leaq	-0x10(%rbp), %rcx
140a65952:     	leaq	-0x30(%rbp), %rdx
140a65956:     	movl	$0x12, %r9d
140a6595c:     	callq	0x141311b90 <.text+0x1310b90>
140a65961:     	movzbl	-0x10(%rbp), %eax
140a65965:     	cmpb	$0x32, %al
140a65967:     	jne	0x140a65a65 <.text+0xa64a65>
140a6596d:     	leaq	0x1c8(%rdi), %rax
140a65974:     	movb	$0x1, 0x47(%rbp)
140a65978:     	movq	%rax, 0x20(%rsp)
140a6597d:     	leaq	0x206ec19(%rip), %r8    # 0x142ad459d
140a65984:     	leaq	-0x10(%rbp), %rcx
140a65988:     	leaq	-0x30(%rbp), %rdx
140a6598c:     	movl	$0x14, %r9d
140a65992:     	callq	0x1413121a0 <.text+0x13111a0>
140a65997:     	movzbl	-0x10(%rbp), %eax
140a6599b:     	cmpb	$0x32, %al
140a6599d:     	jne	0x140a65a65 <.text+0xa64a65>
140a659a3:     	movb	$0x1, 0x47(%rbp)
140a659a7:     	movq	%rdi, 0x20(%rsp)
140a659ac:     	leaq	0x206ebfe(%rip), %r8    # 0x142ad45b1
140a659b3:     	leaq	-0x10(%rbp), %rcx
140a659b7:     	leaq	-0x30(%rbp), %rdx
140a659bb:     	movl	$0xc, %r9d
140a659c1:     	callq	0x141312480 <.text+0x1311480>
140a659c6:     	movzbl	-0x10(%rbp), %eax
140a659ca:     	cmpb	$0x32, %al
140a659cc:     	jne	0x140a65a65 <.text+0xa64a65>
140a659d2:     	addq	$0x1d8, %rdi            # imm = 0x1D8
140a659d9:     	movb	$0x1, 0x47(%rbp)
140a659dd:     	movq	%rdi, 0x20(%rsp)
140a659e2:     	leaq	0x206ebd4(%rip), %r8    # 0x142ad45bd
140a659e9:     	leaq	-0x10(%rbp), %rcx
140a659ed:     	leaq	-0x30(%rbp), %rdx
140a659f1:     	movl	$0xf, %r9d
140a659f7:     	callq	0x141310af0 <.text+0x130faf0>
140a659fc:     	movzbl	-0x10(%rbp), %eax
140a65a00:     	cmpb	$0x32, %al
140a65a02:     	jne	0x140a65a65 <.text+0xa64a65>
140a65a04:     	movq	-0x20(%rbp), %rax
140a65a08:     	movq	%rax, (%rbp)
140a65a0c:     	movaps	-0x30(%rbp), %xmm0
140a65a10:     	movaps	%xmm0, -0x10(%rbp)
140a65a14:     	movb	$0x0, 0x47(%rbp)
140a65a18:     	leaq	-0x10(%rbp), %rdx
140a65a1c:     	movq	%rsi, %rcx
140a65a1f:     	callq	0x141312a40 <.text+0x1311a40>
140a65a24:     	jmp	0x140a65aae <.text+0xa64aae>
140a65a29:     	movups	-0xf(%rbp), %xmm0
140a65a2d:     	movups	(%rbp), %xmm1
140a65a31:     	movups	0x10(%rbp), %xmm2
140a65a35:     	movups	0x20(%rbp), %xmm3
140a65a39:     	movups	%xmm1, -0x41(%rbp)
140a65a3d:     	movaps	%xmm0, -0x50(%rbp)
140a65a41:     	movq	0x30(%rbp), %rcx
140a65a45:     	movq	%rcx, 0x40(%rsi)
140a65a49:     	movups	%xmm3, 0x30(%rsi)
140a65a4d:     	movups	%xmm2, 0x20(%rsi)
140a65a51:     	movups	-0x41(%rbp), %xmm0
140a65a55:     	movups	%xmm0, 0x10(%rsi)
140a65a59:     	movaps	-0x50(%rbp), %xmm0
140a65a5d:     	movups	%xmm0, 0x1(%rsi)
140a65a61:     	movb	%al, (%rsi)
140a65a63:     	jmp	0x140a65aae <.text+0xa64aae>
140a65a65:     	movq	0x30(%rbp), %rcx
140a65a69:     	movq	%rcx, 0x40(%rsi)
140a65a6d:     	movups	-0xf(%rbp), %xmm0
140a65a71:     	movups	0x1(%rbp), %xmm1
140a65a75:     	movups	0x11(%rbp), %xmm2
140a65a79:     	movups	0x21(%rbp), %xmm3
140a65a7d:     	movups	%xmm3, 0x31(%rsi)
140a65a81:     	movups	%xmm2, 0x21(%rsi)
140a65a85:     	movups	%xmm1, 0x11(%rsi)
140a65a89:     	movups	%xmm0, 0x1(%rsi)
140a65a8d:     	movb	%al, (%rsi)
140a65a8f:     	movq	-0x28(%rbp), %rax
140a65a93:     	cmpl	$0x1, (%rax)
140a65a96:     	jne	0x140a65aae <.text+0xa64aae>
140a65a98:     	movq	0x8(%rax), %rcx
140a65a9c:     	incq	%rcx
140a65a9f:     	movq	$-0x1, %rdx
140a65aa6:     	cmovneq	%rcx, %rdx
140a65aaa:     	movq	%rdx, 0x8(%rax)
140a65aae:     	movq	%rsi, %rax
140a65ab1:     	addq	$0xd0, %rsp
140a65ab8:     	popq	%rdi
140a65ab9:     	popq	%rsi
140a65aba:     	popq	%rbp
140a65abb:     	retq
140a65abc:     	nopl	(%rax)
140a65ac0:     	movq	%rdx, 0x10(%rsp)
140a65ac5:     	pushq	%rbp
140a65ac6:     	pushq	%rsi
140a65ac7:     	pushq	%rdi
140a65ac8:     	subq	$0x30, %rsp
140a65acc:     	leaq	0x80(%rdx), %rbp
140a65ad3:     	cmpb	$0x0, 0x47(%rbp)
140a65ad7:     	je	0x140a65af8 <.text+0xa64af8>
140a65ad9:     	movq	-0x28(%rbp), %rax
140a65add:     	cmpl	$0x1, (%rax)
140a65ae0:     	jne	0x140a65af8 <.text+0xa64af8>
140a65ae2:     	movq	0x8(%rax), %rcx
140a65ae6:     	incq	%rcx
140a65ae9:     	movq	$-0x1, %rdx
140a65af0:     	cmovneq	%rcx, %rdx
140a65af4:     	movq	%rdx, 0x8(%rax)
140a65af8:     	addq	$0x30, %rsp
140a65afc:     	popq	%rdi
140a65afd:     	popq	%rsi
140a65afe:     	popq	%rbp
140a65aff:     	retq
140a65b00:     	pushq	%rsi
140a65b01:     	subq	$0x20, %rsp
140a65b05:     	movq	%rcx, %rax
140a65b08:     	leaq	-0x8(%r8), %rcx
140a65b0c:     	cmpq	$0xc, %rcx
140a65b10:     	ja	0x140a65c0b <.text+0xa64c0b>
140a65b16:     	leaq	0x206edd7(%rip), %r9    # 0x142ad48f4
140a65b1d:     	movslq	(%r9,%rcx,4), %rcx
140a65b21:     	addq	%r9, %rcx
140a65b24:     	jmpq	*%rcx
140a65b26:     	movabsq	$0x6c6169726f747574, %rcx # imm = 0x6C6169726F747574
140a65b30:     	cmpq	%rcx, (%rdx)
140a65b33:     	jne	0x140a65c0b <.text+0xa64c0b>
140a65b39:     	movb	$0x0, 0x1(%rax)
140a65b3d:     	jmp	0x140a65c12 <.text+0xa64c12>
140a65b42:     	leaq	0x206ea68(%rip), %r9    # 0x142ad45b1
140a65b49:     	movq	%rdx, %rcx
140a65b4c:     	movq	%r9, %rdx
140a65b4f:     	movq	%rax, %rsi
140a65b52:     	callq	0x142923bf0 <.text+0x2922bf0>
140a65b57:     	movl	%eax, %ecx
140a65b59:     	movq	%rsi, %rax
140a65b5c:     	testl	%ecx, %ecx
140a65b5e:     	jne	0x140a65c0b <.text+0xa64c0b>
140a65b64:     	movb	$0x3, 0x1(%rax)
140a65b68:     	jmp	0x140a65c12 <.text+0xa64c12>
140a65b6d:     	movdqu	(%rdx), %xmm0
140a65b71:     	movd	0x10(%rdx), %xmm1
140a65b76:     	pcmpeqb	0x1f82be2(%rip), %xmm1  # 0x1429e8760
140a65b7e:     	pcmpeqb	0x1f82bea(%rip), %xmm0  # 0x1429e8770
