
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14077b900:     	testl	%eax, 0x2(%rdi)
14077b903:     	addb	%al, (%rax)
14077b905:     	addl	%ecx, -0x73(%rax)
14077b908:     	leal	(%rax), %edx
14077b90a:     	addl	%eax, (%rax)
14077b90c:     	addb	%cl, -0x73(%rax)
14077b90f:     	adcl	$0x23040cc, %eax        # imm = 0x23040CC
14077b914:     	callq	0x14078c940 <.text+0x78b940>
14077b919:     	jmp	0x14077b4e0 <.text+0x77a4e0>
14077b91e:     	nop
14077b920:     	movq	0x120(%rbp), %rdi
14077b927:     	movzbl	0x2eb92db(%rip), %eax   # 0x143634c09
14077b92e:     	movl	$0x20, %ecx
14077b933:     	movl	$0x8, %edx
14077b938:     	callq	0x140613c10 <.text+0x612c10>
14077b93d:     	testq	%rax, %rax
14077b940:     	je	0x14077c801 <.text+0x77b801>
14077b946:     	movq	%rax, %rsi
14077b949:     	movq	$0x0, (%rax)
14077b950:     	movq	$0x4, 0x8(%rax)
14077b958:     	movq	$0x0, 0x10(%rax)
14077b960:     	movb	$0x0, 0x18(%rax)
14077b964:     	movq	0xf8(%rbp), %rax
14077b96b:     	movq	0x60(%rax), %rbx
14077b96f:     	jmp	0x14077b982 <.text+0x77a982>
14077b971:     	nopw	%cs:(%rax,%rax)
14077b980:     	pause
14077b982:     	movq	0x8(%rbx), %rax
14077b986:     	nopw	%cs:(%rax,%rax)
14077b990:     	cmpq	$-0x1, %rax
14077b994:     	je	0x14077b980 <.text+0x77a980>
14077b996:     	testq	%rax, %rax
14077b999:     	js	0x14077c5bc <.text+0x77b5bc>
14077b99f:     	leaq	0x1(%rax), %rcx
14077b9a3:     	lock
14077b9a4:     	cmpxchgq	%rcx, 0x8(%rbx)
14077b9a9:     	jne	0x14077b990 <.text+0x77a990>
14077b9ab:     	movq	%rbx, -0x40(%rbp)
14077b9af:     	movq	%rsi, -0x38(%rbp)
14077b9b3:     	movq	0x68(%rbp), %r14
14077b9b7:     	movq	0x70(%rbp), %r15
14077b9bb:     	movq	%r14, 0xa0(%rbp)
14077b9c2:     	movq	%r15, 0xa8(%rbp)
14077b9c9:     	movq	0x110(%rbp), %rax
14077b9d0:     	movq	%rax, 0x238(%rbp)
14077b9d7:     	movq	0x118(%rbp), %rax
14077b9de:     	movq	%rax, 0x228(%rbp)
14077b9e5:     	movq	0x120(%rbp), %r12
14077b9ec:     	leaq	0x210(%rbp), %rcx
14077b9f3:     	callq	0x1400885d0 <.text+0x875d0>
14077b9f8:     	testq	%rdi, %rdi
14077b9fb:     	movq	%rbx, 0x160(%rbp)
14077ba02:     	movq	%rsi, 0x168(%rbp)
14077ba09:     	movq	%r14, 0x170(%rbp)
14077ba10:     	movq	%r15, 0x178(%rbp)
14077ba17:     	movq	0x238(%rbp), %rcx
14077ba1e:     	movq	%rcx, 0x140(%rbp)
14077ba25:     	movq	0x228(%rbp), %rcx
14077ba2c:     	movq	%rcx, 0x148(%rbp)
14077ba33:     	movq	%r12, 0x150(%rbp)
14077ba3a:     	movl	$0x1, 0x158(%rbp)
14077ba44:     	movl	%eax, 0x15c(%rbp)
14077ba4a:     	setne	0x180(%rbp)
14077ba51:     	movq	0x210(%rbp), %r8
14077ba58:     	movb	$0x0, 0x247(%rbp)
14077ba5f:     	leaq	0xa0(%rbp), %rcx
14077ba66:     	movq	0x388(%rbp), %rdx
14077ba6d:     	leaq	0x140(%rbp), %r9
14077ba74:     	callq	0x1411fd080 <.text+0x11fc080>
14077ba79:     	xorl	%eax, %eax
14077ba7b:     	cmpq	0xa0(%rbp), %rax
14077ba82:     	jo	0x14077ba97 <.text+0x77aa97>
14077ba84:     	movb	$0x0, 0x247(%rbp)
14077ba8b:     	leaq	0xa0(%rbp), %rcx
14077ba92:     	callq	0x1411fe1a0 <.text+0x11fd1a0>
14077ba97:     	movq	0x1d0(%rbp), %rsi
14077ba9e:     	addq	$0x18, %rsi
14077baa2:     	movb	$0x0, 0x246(%rbp)
14077baa9:     	movq	%rsi, %rcx
14077baac:     	callq	0x140c81e10 <.text+0xc80e10>
14077bab1:     	movq	0x8(%rbp), %r15
14077bab5:     	movss	0x21b52b3(%rip), %xmm6  # 0x142930d70
14077babd:     	movb	$0x1, %al
14077babf:     	xorl	%ecx, %ecx
14077bac1:     	lock
14077bac2:     	cmpxchgb	%cl, (%rsi)
14077bac5:     	je	0x14077aea0 <.text+0x779ea0>
14077bacb:     	movb	$0x0, 0x246(%rbp)
14077bad2:     	movq	%rsi, %rcx
14077bad5:     	xorl	%edx, %edx
14077bad7:     	callq	0x1428cdb90 <.text+0x28ccb90>
14077badc:     	jmp	0x14077aea0 <.text+0x779ea0>
14077bae1:     	movb	$0x1, 0x246(%rbp)
14077bae8:     	movq	%rbx, %rcx
14077baeb:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14077baf1:     	callq	0x1428cd210 <.text+0x28cc210>
14077baf6:     	jmp	0x14077b402 <.text+0x77a402>
14077bafb:     	leaq	0x249ed76(%rip), %rdx   # 0x142c1a878
14077bb02:     	movq	0x230(%rbp), %rcx
14077bb09:     	callq	0x141209c40 <.text+0x1208c40>
14077bb0e:     	movq	%rax, %rsi
14077bb11:     	testq	%rax, %rax
14077bb14:     	je	0x14077c8a4 <.text+0x77b8a4>
14077bb1a:     	leaq	0x249ee84(%rip), %rax   # 0x142c1a9a5
14077bb21:     	movq	%rax, 0x140(%rbp)
14077bb28:     	movq	$0x3a, 0x148(%rbp)
14077bb33:     	movq	%rsi, %rdi
14077bb36:     	addq	$0x10, %rdi
14077bb3a:     	cmpq	$0x3a, 0x18(%rsi)
14077bb3f:     	jne	0x14077c775 <.text+0x77b775>
14077bb45:     	movq	(%rdi), %rdx
14077bb48:     	leaq	0x249ee56(%rip), %rcx   # 0x142c1a9a5
14077bb4f:     	movl	$0x3a, %r8d
14077bb55:     	callq	0x142923bf0 <.text+0x2922bf0>
14077bb5a:     	testl	%eax, %eax
14077bb5c:     	jne	0x14077c775 <.text+0x77b775>
14077bb62:     	movq	(%rsi), %rcx
14077bb65:     	movq	0x8(%rsi), %rax
14077bb69:     	callq	*0x18(%rax)
14077bb6c:     	movq	%rax, 0x130(%rbp)
14077bb73:     	cmpq	$0x0, 0x130(%rbp)
14077bb7b:     	je	0x14077c8a4 <.text+0x77b8a4>
14077bb81:     	movq	0x60(%rbp), %rdx
14077bb85:     	movl	0x1c(%rdx), %eax
14077bb88:     	movq	0x10(%rdx), %rcx
14077bb8c:     	movl	%eax, (%rcx)
14077bb8e:     	movq	(%rdx), %rax
14077bb91:     	movq	(%rax), %rbx
14077bb94:     	movq	0x18(%rax), %r15
14077bb98:     	movdqa	(%rbx), %xmm0
14077bb9c:     	pmovmskb	%xmm0, %eax
14077bba0:     	notl	%eax
14077bba2:     	leaq	0x10(%rbx), %r12
14077bba6:     	movq	0x58(%rbp), %rcx
14077bbaa:     	movq	(%rcx), %rcx
14077bbad:     	movq	%rcx, 0x238(%rbp)
14077bbb4:     	subq	$0x1, %r15
14077bbb8:     	movq	0x388(%rbp), %rcx
14077bbbf:     	jb	0x14077c03a <.text+0x77b03a>
14077bbc5:     	nopw	%cs:(%rax,%rax)
14077bbd0:     	testw	%ax, %ax
14077bbd3:     	je	0x14077bbe0 <.text+0x77abe0>
14077bbd5:     	leal	-0x1(%rax), %r13d
14077bbd9:     	andl	%eax, %r13d
14077bbdc:     	jmp	0x14077bc0b <.text+0x77ac0b>
14077bbde:     	nop
14077bbe0:     	movdqa	(%r12), %xmm0
14077bbe6:     	pmovmskb	%xmm0, %eax
14077bbea:     	addq	$-0x1100, %rbx          # imm = 0xEF00
14077bbf1:     	addq	$0x10, %r12
14077bbf5:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
14077bbfa:     	je	0x14077bbe0 <.text+0x77abe0>
14077bbfc:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14077bc01:     	subl	%eax, %ecx
14077bc03:     	notl	%eax
14077bc05:     	movl	%eax, %r13d
14077bc08:     	andl	%ecx, %r13d
14077bc0b:     	tzcntl	%eax, %eax
14077bc0f:     	negq	%rax
14077bc12:     	imulq	$0x110, %rax, %r14      # imm = 0x110
14077bc19:     	cmpq	$0x0, -0xa0(%rbx,%r14)
14077bc22:     	je	0x14077c81c <.text+0x77b81c>
14077bc28:     	addq	%rbx, %r14
14077bc2b:     	movq	-0x98(%r14), %rdi
14077bc32:     	testq	%rdi, %rdi
14077bc35:     	je	0x14077c82d <.text+0x77b82d>
14077bc3b:     	leaq	0x18(%rdi), %rcx
14077bc3f:     	xorl	%eax, %eax
14077bc41:     	movb	$0x1, %dl
14077bc43:     	lock
14077bc44:     	cmpxchgb	%dl, 0x18(%rdi)
14077bc48:     	jne	0x14077c018 <.text+0x77b018>
14077bc4e:     	callq	0x140c81c70 <.text+0xc80c70>
14077bc53:     	movq	0x10(%rdi), %rax
14077bc57:     	testq	%rax, %rax
14077bc5a:     	sete	%cl
14077bc5d:     	shlq	$0x2, %rax
14077bc61:     	addq	0x8(%rdi), %rax
14077bc65:     	addq	$-0x4, %rax
14077bc69:     	sete	%dl
14077bc6c:     	orb	%cl, %dl
14077bc6e:     	movq	%rbx, 0x1a0(%rbp)
14077bc75:     	movq	%r15, 0x218(%rbp)
14077bc7c:     	movq	%r12, 0x230(%rbp)
14077bc83:     	movl	%r13d, 0x220(%rbp)
14077bc8a:     	je	0x14077bcb0 <.text+0x77acb0>
14077bc8c:     	movq	0x130(%rbp), %rax
14077bc93:     	movq	0x28(%rax), %rax
14077bc97:     	testq	%rax, %rax
14077bc9a:     	je	0x14077c86c <.text+0x77b86c>
14077bca0:     	leal	-0x1(%rax), %ecx
14077bca3:     	movq	0x130(%rbp), %r8
14077bcaa:     	jmp	0x14077bcbd <.text+0x77acbd>
14077bcac:     	nopl	(%rax)
14077bcb0:     	movl	(%rax), %ecx
14077bcb2:     	movq	0x130(%rbp), %r8
14077bcb9:     	movq	0x28(%r8), %rax
14077bcbd:     	movq	0x20(%r8), %rdx
14077bcc1:     	movups	0x30(%r8), %xmm0
14077bcc6:     	movups	0x40(%r8), %xmm1
14077bccb:     	movups	0x50(%r8), %xmm2
14077bcd0:     	movl	%ecx, 0xf0(%rbp)
14077bcd6:     	movq	%rdx, 0xa0(%rbp)
14077bcdd:     	movq	%rax, 0xa8(%rbp)
14077bce4:     	movups	%xmm0, 0xb0(%rbp)
14077bceb:     	movq	%rdi, 0xc0(%rbp)
14077bcf2:     	movups	%xmm1, 0xc8(%rbp)
14077bcf9:     	movq	%r8, 0xd8(%rbp)
14077bd00:     	movups	%xmm2, 0xe0(%rbp)
14077bd07:     	leaq	0xa0(%rbp), %rcx
14077bd0e:     	callq	0x14120c9d0 <.text+0x120b9d0>
14077bd13:     	movq	-0xb8(%r14), %rbx
14077bd1a:     	movq	-0xb0(%r14), %rax
14077bd21:     	shlq	$0x6, %rax
14077bd25:     	addq	%rbx, %rax
14077bd28:     	movq	%rax, 0x228(%rbp)
14077bd2f:     	xorl	%r15d, %r15d
14077bd32:     	cmpq	0x228(%rbp), %rbx
14077bd39:     	je	0x14077bfcb <.text+0x77afcb>
14077bd3f:     	nop
14077bd40:     	movl	-0x110(%r14), %edi
14077bd47:     	movl	-0x10c(%r14), %r13d
14077bd4e:     	movl	-0xa8(%r14), %esi
14077bd55:     	movl	-0xa4(%r14), %r12d
14077bd5c:     	movq	0x238(%rbp), %rax
14077bd63:     	movss	0x28(%rax), %xmm0
14077bd68:     	divss	0x21a9b68(%rip), %xmm0  # 0x1429258d8
14077bd70:     	addss	0x30(%rbx), %xmm0
14077bd75:     	xorps	%xmm2, %xmm2
14077bd78:     	maxss	%xmm0, %xmm2
14077bd7c:     	movss	0x21a9b80(%rip), %xmm1  # 0x142925904
14077bd84:     	minss	%xmm2, %xmm1
14077bd88:     	movss	%xmm1, 0x30(%rbx)
14077bd8d:     	movss	0x1c(%rbx), %xmm10
14077bd93:     	movss	0x20(%rbx), %xmm12
14077bd99:     	movss	0x24(%rbx), %xmm11
14077bd9f:     	leaq	0x2303c82(%rip), %rcx   # 0x142a7fa28
14077bda6:     	callq	0x140c95ef0 <.text+0xc94ef0>
14077bdab:     	movaps	%xmm0, %xmm6
14077bdae:     	mulss	%xmm0, %xmm10
14077bdb3:     	movss	0x3c(%rbx), %xmm7
14077bdb8:     	movss	0x21a9b43(%rip), %xmm9  # 0x142925904
14077bdc1:     	movaps	%xmm9, %xmm0
14077bdc5:     	subss	%xmm7, %xmm0
14077bdc9:     	mulss	0x21a9bb3(%rip), %xmm0  # 0x142925984
14077bdd1:     	mulss	0x225128f(%rip), %xmm7  # 0x1429cd068
14077bdd9:     	addss	%xmm0, %xmm7
14077bddd:     	movq	0x238(%rbp), %rax
14077bde4:     	movsd	0x10(%rax), %xmm0
14077bde9:     	movaps	0x10(%rbx), %xmm13
14077bdee:     	movss	0x38(%rbx), %xmm1
14077bdf3:     	cvtsd2ss	%xmm0, %xmm0
14077bdf7:     	addss	0x21b28f1(%rip), %xmm1  # 0x14292e6f0
14077bdff:     	mulss	%xmm1, %xmm0
14077be03:     	addss	0x34(%rbx), %xmm0
14077be08:     	callq	0x142923fd0 <.text+0x2922fd0>
14077be0d:     	movshdup	%xmm13, %xmm1           # xmm1 = xmm13[1,1,3,3]
14077be12:     	mulss	%xmm6, %xmm11
14077be17:     	mulss	%xmm6, %xmm12
14077be1c:     	mulss	%xmm7, %xmm0
14077be20:     	subss	%xmm0, %xmm1
14077be24:     	movaps	(%rbx), %xmm3
14077be27:     	movss	0x8(%rbx), %xmm4
14077be2c:     	movaps	%xmm4, %xmm5
14077be2f:     	addss	%xmm4, %xmm5
14077be33:     	mulss	%xmm5, %xmm4
14077be37:     	movaps	%xmm3, %xmm2
14077be3a:     	addps	%xmm3, %xmm2
14077be3d:     	movshdup	%xmm2, %xmm0            # xmm0 = xmm2[1,1,3,3]
14077be41:     	mulss	%xmm3, %xmm0
14077be45:     	movaps	%xmm3, %xmm6
14077be48:     	movsldup	%xmm5, %xmm7            # xmm7 = xmm5[0,0,2,2]
14077be4c:     	mulps	%xmm3, %xmm7
14077be4f:     	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
14077be53:     	mulps	%xmm2, %xmm6
14077be56:     	mulps	%xmm3, %xmm2
14077be59:     	movaps	%xmm3, %xmm8
14077be5d:     	mulss	%xmm5, %xmm8
14077be62:     	shufps	$0xe1, %xmm2, %xmm2     # xmm2 = xmm2[1,0,2,3]
14077be66:     	movshdup	%xmm6, %xmm14           # xmm14 = xmm6[1,1,3,3]
14077be6b:     	movaps	%xmm14, %xmm5
14077be6f:     	addss	%xmm4, %xmm5
14077be73:     	movaps	%xmm9, %xmm3
14077be77:     	subss	%xmm5, %xmm3
14077be7b:     	movaps	%xmm0, %xmm15
14077be7f:     	addss	%xmm8, %xmm15
14077be84:     	movaps	%xmm7, %xmm5
14077be87:     	subps	%xmm2, %xmm5
14077be8a:     	subss	%xmm8, %xmm0
14077be8f:     	addss	%xmm6, %xmm4
14077be93:     	movaps	%xmm9, %xmm8
14077be97:     	subss	%xmm4, %xmm8
14077be9c:     	addps	%xmm7, %xmm2
14077be9f:     	unpcklps	%xmm5, %xmm15           # xmm15 = xmm15[0],xmm5[0],xmm15[1],xmm5[1]
14077bea3:     	shufps	$0x1, %xmm2, %xmm5      # xmm5 = xmm5[1,0],xmm2[0,0]
14077bea7:     	shufps	$0xe2, %xmm2, %xmm5     # xmm5 = xmm5[2,0],xmm2[2,3]
14077beab:     	addss	%xmm14, %xmm6
14077beb0:     	movaps	%xmm9, %xmm4
14077beb4:     	subss	%xmm6, %xmm4
14077beb8:     	mulss	%xmm10, %xmm3
14077bebd:     	movsldup	%xmm10, %xmm6           # xmm6 = xmm10[0,0,2,2]
14077bec2:     	mulps	%xmm15, %xmm6
14077bec6:     	movlhps	%xmm6, %xmm3            # xmm3 = xmm3[0],xmm6[0]
14077bec9:     	shufps	$0x58, %xmm6, %xmm3     # xmm3 = xmm3[0,2],xmm6[1,1]
14077becd:     	mulss	%xmm12, %xmm0
14077bed2:     	movsldup	%xmm12, %xmm6           # xmm6 = xmm12[0,0,2,2]
14077bed7:     	movss	%xmm8, %xmm2            # xmm2 = xmm8[0],xmm2[1,2,3]
14077bedc:     	mulps	%xmm6, %xmm2
14077bedf:     	movlhps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0]
14077bee2:     	shufps	$0x58, %xmm2, %xmm0     # xmm0 = xmm0[0,2],xmm2[1,1]
14077bee6:     	movsldup	%xmm11, %xmm2           # xmm2 = xmm11[0,0,2,2]
14077beeb:     	mulps	%xmm5, %xmm2
14077beee:     	mulss	%xmm11, %xmm4
14077bef3:     	shufps	$0x4, %xmm4, %xmm2      # xmm2 = xmm2[0,1],xmm4[0,0]
14077bef7:     	movlhps	%xmm13, %xmm1           # xmm1 = xmm1[0],xmm13[0]
14077befb:     	shufps	$0xa2, %xmm13, %xmm1    # xmm1 = xmm1[2,0],xmm13[2,2]
14077bf00:     	movaps	%xmm3, 0x1b0(%rbp)
14077bf07:     	movaps	%xmm0, 0x1c0(%rbp)
14077bf0e:     	movaps	%xmm2, 0x1d0(%rbp)
14077bf15:     	movaps	%xmm1, 0x1e0(%rbp)
14077bf1c:     	leaq	0x140(%rbp), %rcx
14077bf23:     	leaq	0x1b0(%rbp), %rdx
14077bf2a:     	callq	0x14089d560 <.text+0x89c560>
14077bf2f:     	testl	%esi, %esi
14077bf31:     	movl	$0x186a3, %eax          # imm = 0x186A3
14077bf36:     	cmovel	%eax, %r12d
14077bf3a:     	movl	%r12d, %ecx
14077bf3d:     	callq	0x140858750 <.text+0x857750>
14077bf42:     	movl	%eax, %esi
14077bf44:     	leaq	0x1b0(%rbp), %rcx
14077bf4b:     	callq	0x1408587c0 <.text+0x8577c0>
14077bf50:     	leal	(%rdi,%r15,2), %eax
14077bf54:     	addl	%r13d, %eax
14077bf57:     	leaq	0x1e0(%rbp), %rdx
14077bf5e:     	movups	(%rdx), %xmm0
14077bf61:     	leaq	0x170(%rbp), %r8
14077bf68:     	movups	%xmm0, (%r8)
14077bf6c:     	movl	0x18(%rdx), %ecx
14077bf6f:     	movl	%ecx, 0x18(%r8)
14077bf73:     	movq	0x10(%rdx), %rcx
14077bf77:     	movq	%rcx, 0x10(%r8)
14077bf7b:     	movl	%eax, 0x18c(%rbp)
14077bf81:     	movl	%esi, 0x190(%rbp)
14077bf87:     	movl	$0x0, 0x194(%rbp)
14077bf91:     	movss	%xmm10, 0x198(%rbp)
14077bf9a:     	movl	$0x40, 0x19c(%rbp)
14077bfa4:     	leaq	0xa0(%rbp), %rcx
14077bfab:     	leaq	0x140(%rbp), %rdx
14077bfb2:     	callq	0x14120c120 <.text+0x120b120>
14077bfb7:     	addq	$0x40, %rbx
14077bfbb:     	incl	%r15d
14077bfbe:     	cmpq	0x228(%rbp), %rbx
14077bfc5:     	jne	0x14077bd40 <.text+0x77ad40>
14077bfcb:     	movq	0xc0(%rbp), %rsi
14077bfd2:     	addq	$0x18, %rsi
14077bfd6:     	movq	%rsi, %rcx
14077bfd9:     	callq	0x140c81e10 <.text+0xc80e10>
14077bfde:     	movq	0x1a0(%rbp), %rbx
14077bfe5:     	movq	0x218(%rbp), %r15
14077bfec:     	movq	0x230(%rbp), %r12
14077bff3:     	movl	0x220(%rbp), %edi
14077bff9:     	movb	$0x1, %al
14077bffb:     	xorl	%ecx, %ecx
14077bffd:     	lock
14077bffe:     	cmpxchgb	%cl, (%rsi)
14077c001:     	jne	0x14077c02e <.text+0x77b02e>
14077c003:     	movl	%edi, %eax
14077c005:     	subq	$0x1, %r15
14077c009:     	movq	0x388(%rbp), %rcx
14077c010:     	jae	0x14077bbd0 <.text+0x77abd0>
14077c016:     	jmp	0x14077c03a <.text+0x77b03a>
14077c018:     	movq	%rcx, %rsi
14077c01b:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14077c021:     	callq	0x1428cd210 <.text+0x28cc210>
14077c026:     	movq	%rsi, %rcx
14077c029:     	jmp	0x14077bc4e <.text+0x77ac4e>
14077c02e:     	movq	%rsi, %rcx
14077c031:     	xorl	%edx, %edx
14077c033:     	callq	0x1428cdb90 <.text+0x28ccb90>
14077c038:     	jmp	0x14077c003 <.text+0x77b003>
14077c03a:     	movq	(%rcx), %rdx
14077c03d:     	movq	0x18(%rcx), %rax
14077c041:     	movq	%rax, 0x220(%rbp)
14077c048:     	movdqa	(%rdx), %xmm0
14077c04c:     	pmovmskb	%xmm0, %eax
14077c050:     	notl	%eax
14077c052:     	movq	%rdx, 0x218(%rbp)
14077c059:     	leaq	0x10(%rdx), %rdi
14077c05d:     	subq	$0x1, 0x220(%rbp)
14077c065:     	jb	0x14077c4f0 <.text+0x77b4f0>
14077c06b:     	nopl	(%rax,%rax)
14077c070:     	testw	%ax, %ax
14077c073:     	je	0x14077c090 <.text+0x77b090>
14077c075:     	leal	-0x1(%rax), %ecx
14077c078:     	andl	%eax, %ecx
14077c07a:     	movl	%ecx, 0x230(%rbp)
14077c080:     	jmp	0x14077c0d4 <.text+0x77b0d4>
14077c082:     	nopw	%cs:(%rax,%rax)
14077c090:     	movq	0x218(%rbp), %rcx
14077c097:     	nopw	(%rax,%rax)
14077c0a0:     	movdqa	(%rdi), %xmm0
14077c0a4:     	pmovmskb	%xmm0, %eax
14077c0a8:     	addq	$-0x500, %rcx           # imm = 0xFB00
14077c0af:     	addq	$0x10, %rdi
14077c0b3:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
14077c0b8:     	je	0x14077c0a0 <.text+0x77b0a0>
14077c0ba:     	movq	%rcx, 0x218(%rbp)
14077c0c1:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14077c0c6:     	subl	%eax, %ecx
14077c0c8:     	notl	%eax
14077c0ca:     	movl	%eax, %edx
14077c0cc:     	andl	%ecx, %edx
14077c0ce:     	movl	%edx, 0x230(%rbp)
14077c0d4:     	tzcntl	%eax, %eax
14077c0d8:     	negq	%rax
14077c0db:     	leaq	(%rax,%rax,4), %rbx
14077c0df:     	shlq	$0x4, %rbx
14077c0e3:     	movq	0x218(%rbp), %rax
14077c0ea:     	cmpq	$0x0, -0x28(%rax,%rbx)
14077c0f0:     	je	0x14077c83b <.text+0x77b83b>
14077c0f6:     	leaq	(%rax,%rbx), %r14
14077c0fa:     	movq	-0x20(%r14), %r15
14077c0fe:     	testq	%r15, %r15
14077c101:     	je	0x14077c849 <.text+0x77b849>
14077c107:     	leaq	0x18(%r15), %rcx
14077c10b:     	xorl	%eax, %eax
14077c10d:     	movb	$0x1, %dl
14077c10f:     	lock
14077c110:     	cmpxchgb	%dl, 0x18(%r15)
14077c115:     	jne	0x14077c4ce <.text+0x77b4ce>
14077c11b:     	callq	0x140c81c70 <.text+0xc80c70>
14077c120:     	movq	0x10(%r15), %rax
14077c124:     	testq	%rax, %rax
14077c127:     	sete	%cl
14077c12a:     	shlq	$0x2, %rax
14077c12e:     	addq	0x8(%r15), %rax
14077c132:     	addq	$-0x4, %rax
14077c136:     	sete	%dl
14077c139:     	orb	%cl, %dl
14077c13b:     	je	0x14077c160 <.text+0x77b160>
14077c13d:     	movq	0x130(%rbp), %rax
14077c144:     	movq	0x28(%rax), %rax
14077c148:     	testq	%rax, %rax
14077c14b:     	je	0x14077c87a <.text+0x77b87a>
14077c151:     	leal	-0x1(%rax), %ecx
14077c154:     	movq	0x130(%rbp), %r8
14077c15b:     	jmp	0x14077c16d <.text+0x77b16d>
14077c15d:     	nopl	(%rax)
14077c160:     	movl	(%rax), %ecx
14077c162:     	movq	0x130(%rbp), %r8
14077c169:     	movq	0x28(%r8), %rax
14077c16d:     	movq	0x20(%r8), %rdx
14077c171:     	movups	0x30(%r8), %xmm0
14077c176:     	movups	0x40(%r8), %xmm1
14077c17b:     	movups	0x50(%r8), %xmm2
14077c180:     	movl	%ecx, 0xf0(%rbp)
14077c186:     	movq	%rdx, 0xa0(%rbp)
14077c18d:     	movq	%rax, 0xa8(%rbp)
14077c194:     	movups	%xmm0, 0xb0(%rbp)
14077c19b:     	movq	%r15, 0xc0(%rbp)
14077c1a2:     	movups	%xmm1, 0xc8(%rbp)
14077c1a9:     	movq	%r8, 0xd8(%rbp)
14077c1b0:     	movups	%xmm2, 0xe0(%rbp)
14077c1b7:     	leaq	0xa0(%rbp), %rcx
14077c1be:     	callq	0x14120c9d0 <.text+0x120b9d0>
14077c1c3:     	movq	0x218(%rbp), %rax
14077c1ca:     	addq	%rbx, %rax
14077c1cd:     	addq	$-0x50, %rax
14077c1d1:     	movq	%rax, 0x1a0(%rbp)
14077c1d8:     	movq	-0x40(%r14), %r12
14077c1dc:     	movq	-0x38(%r14), %rax
14077c1e0:     	shlq	$0x6, %rax
14077c1e4:     	addq	%r12, %rax
14077c1e7:     	movq	%rax, 0x228(%rbp)
14077c1ee:     	xorl	%ebx, %ebx
14077c1f0:     	cmpq	0x228(%rbp), %r12
14077c1f7:     	je	0x14077c49b <.text+0x77b49b>
14077c1fd:     	nopl	(%rax)
14077c200:     	movq	0x1a0(%rbp), %rcx
14077c207:     	callq	0x1400885d0 <.text+0x875d0>
14077c20c:     	movq	%rax, %r13
14077c20f:     	movl	-0x30(%r14), %r15d
14077c213:     	movl	-0x2c(%r14), %esi
14077c217:     	movq	0x238(%rbp), %rax
14077c21e:     	movss	0x28(%rax), %xmm0
14077c223:     	divss	0x21a96ad(%rip), %xmm0  # 0x1429258d8
14077c22b:     	addss	0x30(%r12), %xmm0
14077c232:     	xorps	%xmm2, %xmm2
14077c235:     	maxss	%xmm0, %xmm2
14077c239:     	movss	0x21a96c3(%rip), %xmm1  # 0x142925904
14077c241:     	minss	%xmm2, %xmm1
14077c245:     	movss	%xmm1, 0x30(%r12)
14077c24c:     	movss	0x1c(%r12), %xmm10
14077c253:     	movss	0x20(%r12), %xmm12
14077c25a:     	movss	0x24(%r12), %xmm11
14077c261:     	leaq	0x23037c0(%rip), %rcx   # 0x142a7fa28
14077c268:     	callq	0x140c95ef0 <.text+0xc94ef0>
14077c26d:     	movaps	%xmm0, %xmm6
14077c270:     	mulss	%xmm0, %xmm10
14077c275:     	movss	0x3c(%r12), %xmm14
14077c27c:     	movss	0x21a967f(%rip), %xmm9  # 0x142925904
14077c285:     	movaps	%xmm9, %xmm0
14077c289:     	subss	%xmm14, %xmm0
14077c28e:     	mulss	0x21a96ee(%rip), %xmm0  # 0x142925984
14077c296:     	mulss	0x2250dc9(%rip), %xmm14 # 0x1429cd068
14077c29f:     	addss	%xmm0, %xmm14
14077c2a4:     	movq	0x238(%rbp), %rax
14077c2ab:     	movsd	0x10(%rax), %xmm0
14077c2b0:     	movaps	0x10(%r12), %xmm13
14077c2b6:     	movss	0x38(%r12), %xmm1
14077c2bd:     	cvtsd2ss	%xmm0, %xmm0
14077c2c1:     	addss	0x21b2427(%rip), %xmm1  # 0x14292e6f0
14077c2c9:     	mulss	%xmm1, %xmm0
14077c2cd:     	addss	0x34(%r12), %xmm0
14077c2d4:     	callq	0x142923fd0 <.text+0x2922fd0>
14077c2d9:     	movshdup	%xmm13, %xmm1           # xmm1 = xmm13[1,1,3,3]
14077c2de:     	mulss	%xmm6, %xmm11
14077c2e3:     	mulss	%xmm6, %xmm12
14077c2e8:     	mulss	%xmm14, %xmm0
14077c2ed:     	subss	%xmm0, %xmm1
14077c2f1:     	movaps	(%r12), %xmm3
14077c2f6:     	movss	0x8(%r12), %xmm4
14077c2fd:     	movaps	%xmm4, %xmm5
14077c300:     	addss	%xmm4, %xmm5
14077c304:     	mulss	%xmm5, %xmm4
14077c308:     	movaps	%xmm3, %xmm2
14077c30b:     	addps	%xmm3, %xmm2
14077c30e:     	movshdup	%xmm2, %xmm0            # xmm0 = xmm2[1,1,3,3]
14077c312:     	mulss	%xmm3, %xmm0
14077c316:     	movaps	%xmm3, %xmm6
14077c319:     	movsldup	%xmm5, %xmm14           # xmm14 = xmm5[0,0,2,2]
14077c31e:     	mulps	%xmm3, %xmm14
14077c322:     	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
14077c326:     	mulps	%xmm2, %xmm6
14077c329:     	mulps	%xmm3, %xmm2
14077c32c:     	movaps	%xmm3, %xmm15
14077c330:     	mulss	%xmm5, %xmm15
14077c335:     	shufps	$0xe1, %xmm2, %xmm2     # xmm2 = xmm2[1,0,2,3]
14077c339:     	movshdup	%xmm6, %xmm7            # xmm7 = xmm6[1,1,3,3]
14077c33d:     	movaps	%xmm7, %xmm5
14077c340:     	addss	%xmm4, %xmm5
14077c344:     	movaps	%xmm9, %xmm3
14077c348:     	subss	%xmm5, %xmm3
14077c34c:     	movaps	%xmm0, %xmm8
14077c350:     	addss	%xmm15, %xmm8
14077c355:     	movaps	%xmm14, %xmm5
14077c359:     	subps	%xmm2, %xmm5
14077c35c:     	subss	%xmm15, %xmm0
14077c361:     	addss	%xmm6, %xmm4
14077c365:     	movaps	%xmm9, %xmm15
14077c369:     	subss	%xmm4, %xmm15
14077c36e:     	addps	%xmm14, %xmm2
14077c372:     	unpcklps	%xmm5, %xmm8            # xmm8 = xmm8[0],xmm5[0],xmm8[1],xmm5[1]
14077c376:     	shufps	$0x1, %xmm2, %xmm5      # xmm5 = xmm5[1,0],xmm2[0,0]
14077c37a:     	shufps	$0xe2, %xmm2, %xmm5     # xmm5 = xmm5[2,0],xmm2[2,3]
14077c37e:     	addss	%xmm7, %xmm6
14077c382:     	subss	%xmm6, %xmm9
14077c387:     	mulss	%xmm10, %xmm3
14077c38c:     	movsldup	%xmm10, %xmm6           # xmm6 = xmm10[0,0,2,2]
14077c391:     	mulps	%xmm8, %xmm6
14077c395:     	movlhps	%xmm6, %xmm3            # xmm3 = xmm3[0],xmm6[0]
14077c398:     	shufps	$0x58, %xmm6, %xmm3     # xmm3 = xmm3[0,2],xmm6[1,1]
14077c39c:     	mulss	%xmm12, %xmm0
14077c3a1:     	movsldup	%xmm12, %xmm6           # xmm6 = xmm12[0,0,2,2]
14077c3a6:     	movss	%xmm15, %xmm2           # xmm2 = xmm15[0],xmm2[1,2,3]
14077c3ab:     	mulps	%xmm6, %xmm2
14077c3ae:     	movlhps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0]
14077c3b1:     	shufps	$0x58, %xmm2, %xmm0     # xmm0 = xmm0[0,2],xmm2[1,1]
14077c3b5:     	movsldup	%xmm11, %xmm2           # xmm2 = xmm11[0,0,2,2]
14077c3ba:     	mulps	%xmm5, %xmm2
14077c3bd:     	mulss	%xmm11, %xmm9
14077c3c2:     	shufps	$0x4, %xmm9, %xmm2      # xmm2 = xmm2[0,1],xmm9[0,0]
14077c3c7:     	movlhps	%xmm13, %xmm1           # xmm1 = xmm1[0],xmm13[0]
14077c3cb:     	shufps	$0xa2, %xmm13, %xmm1    # xmm1 = xmm1[2,0],xmm13[2,2]
14077c3d0:     	movaps	%xmm3, 0x1b0(%rbp)
14077c3d7:     	movaps	%xmm0, 0x1c0(%rbp)
14077c3de:     	movaps	%xmm2, 0x1d0(%rbp)
14077c3e5:     	movaps	%xmm1, 0x1e0(%rbp)
14077c3ec:     	leaq	0x140(%rbp), %rcx
14077c3f3:     	leaq	0x1b0(%rbp), %rdx
14077c3fa:     	callq	0x14089d560 <.text+0x89c560>
14077c3ff:     	testl	%r15d, %r15d
14077c402:     	movl	$0x186a3, %eax          # imm = 0x186A3
14077c407:     	cmovel	%eax, %esi
14077c40a:     	movl	%esi, %ecx
14077c40c:     	callq	0x140858750 <.text+0x857750>
14077c411:     	movl	%eax, %esi
14077c413:     	leaq	0x1b0(%rbp), %rcx
14077c41a:     	callq	0x1408587c0 <.text+0x8577c0>
14077c41f:     	leal	(%rbx,%r13), %eax
14077c423:     	addl	$0x989680, %eax         # imm = 0x989680
14077c428:     	leaq	0x1e0(%rbp), %rdx
14077c42f:     	movups	(%rdx), %xmm0
14077c432:     	leaq	0x170(%rbp), %r8
14077c439:     	movups	%xmm0, (%r8)
14077c43d:     	movl	0x18(%rdx), %ecx
14077c440:     	movl	%ecx, 0x18(%r8)
14077c444:     	movq	0x10(%rdx), %rcx
14077c448:     	movq	%rcx, 0x10(%r8)
14077c44c:     	movl	%eax, 0x18c(%rbp)
14077c452:     	movl	%esi, 0x190(%rbp)
14077c458:     	movl	$0x0, 0x194(%rbp)
14077c462:     	movss	%xmm10, 0x198(%rbp)
14077c46b:     	movl	$0x40, 0x19c(%rbp)
14077c475:     	leaq	0xa0(%rbp), %rcx
14077c47c:     	leaq	0x140(%rbp), %rdx
14077c483:     	callq	0x14120c120 <.text+0x120b120>
14077c488:     	addq	$0x40, %r12
14077c48c:     	incl	%ebx
14077c48e:     	cmpq	0x228(%rbp), %r12
14077c495:     	jne	0x14077c200 <.text+0x77b200>
14077c49b:     	movq	0xc0(%rbp), %rsi
14077c4a2:     	addq	$0x18, %rsi
14077c4a6:     	movq	%rsi, %rcx
14077c4a9:     	callq	0x140c81e10 <.text+0xc80e10>
14077c4ae:     	movb	$0x1, %al
14077c4b0:     	xorl	%ecx, %ecx
14077c4b2:     	lock
14077c4b3:     	cmpxchgb	%cl, (%rsi)
14077c4b6:     	jne	0x14077c4e4 <.text+0x77b4e4>
14077c4b8:     	movl	0x230(%rbp), %eax
14077c4be:     	subq	$0x1, 0x220(%rbp)
14077c4c6:     	jae	0x14077c070 <.text+0x77b070>
14077c4cc:     	jmp	0x14077c4f0 <.text+0x77b4f0>
14077c4ce:     	movq	%rcx, %rsi
14077c4d1:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14077c4d7:     	callq	0x1428cd210 <.text+0x28cc210>
14077c4dc:     	movq	%rsi, %rcx
14077c4df:     	jmp	0x14077c11b <.text+0x77b11b>
14077c4e4:     	movq	%rsi, %rcx
14077c4e7:     	xorl	%edx, %edx
14077c4e9:     	callq	0x1428cdb90 <.text+0x28ccb90>
14077c4ee:     	jmp	0x14077c4b8 <.text+0x77b4b8>
14077c4f0:     	cmpq	$0x0, 0x78(%rbp)
14077c4f5:     	je	0x14077c542 <.text+0x77b542>
14077c4f7:     	callq	0x140c18130 <.text+0xc17130>
14077c4fc:     	movq	%rax, %rcx
14077c4ff:     	movq	(%rax), %rax
14077c502:     	cmpq	$0x1, %rax
14077c506:     	jne	0x14077c65d <.text+0x77b65d>
14077c50c:     	addq	$0x8, %rcx
14077c510:     	cmpq	$0x0, (%rcx)
14077c514:     	jne	0x14077c701 <.text+0x77b701>
14077c51a:     	movq	$-0x1, (%rcx)
14077c521:     	movq	%rcx, 0x238(%rbp)
14077c528:     	addq	$0x8, %rcx
14077c52c:     	movq	0x80(%rbp), %rdx
14077c533:     	callq	0x140c170f0 <.text+0xc160f0>
14077c538:     	movq	0x238(%rbp), %rax
14077c53f:     	incq	(%rax)
14077c542:     	movaps	0x250(%rbp), %xmm6
14077c549:     	movaps	0x260(%rbp), %xmm7
14077c550:     	movaps	0x270(%rbp), %xmm8
14077c558:     	movaps	0x280(%rbp), %xmm9
14077c560:     	movaps	0x290(%rbp), %xmm10
14077c568:     	movaps	0x2a0(%rbp), %xmm11
14077c570:     	movaps	0x2b0(%rbp), %xmm12
14077c578:     	movaps	0x2c0(%rbp), %xmm13
14077c580:     	movaps	0x2d0(%rbp), %xmm14
14077c588:     	movaps	0x2e0(%rbp), %xmm15
14077c590:     	addq	$0x378, %rsp            # imm = 0x378
14077c597:     	popq	%rbx
14077c598:     	popq	%rdi
14077c599:     	popq	%rsi
14077c59a:     	popq	%r12
14077c59c:     	popq	%r13
14077c59e:     	popq	%r14
14077c5a0:     	popq	%r15
14077c5a2:     	popq	%rbp
14077c5a3:     	retq
14077c5a4:     	leaq	0x24a0cad(%rip), %rcx   # 0x142c1d258
14077c5ab:     	leaq	0x24a0d06(%rip), %rdx   # 0x142c1d2b8
14077c5b2:     	callq	0x1428a9a00 <.text+0x28a8a00>
14077c5b7:     	jmp	0x14077c8b0 <.text+0x77b8b0>
14077c5bc:     	movb	$0x1, 0x247(%rbp)
14077c5c3:     	leaq	0x24a0c8e(%rip), %rcx   # 0x142c1d258
14077c5ca:     	leaq	0x24a0ce7(%rip), %rdx   # 0x142c1d2b8
14077c5d1:     	callq	0x1428a9a00 <.text+0x28a8a00>
14077c5d6:     	jmp	0x14077c8b0 <.text+0x77b8b0>
14077c5db:     	movq	$0x0, 0x1b0(%rbp)
14077c5e6:     	leaq	0x249e273(%rip), %rax   # 0x142c1a860
14077c5ed:     	movq	%rax, 0x20(%rsp)
14077c5f2:     	leaq	0x140(%rbp), %rdx
14077c5f9:     	leaq	0x1b0(%rbp), %r9
14077c600:     	xorl	%ecx, %ecx
14077c602:     	movq	%r13, %r8
14077c605:     	callq	0x1428cab41 <.text+0x28c9b41>
14077c60a:     	jmp	0x14077c8b0 <.text+0x77b8b0>
14077c60f:     	movq	$0x0, 0x1b0(%rbp)
14077c61a:     	leaq	0x249e23f(%rip), %rax   # 0x142c1a860
14077c621:     	movq	%rax, 0x20(%rsp)
14077c626:     	leaq	0x140(%rbp), %rdx
14077c62d:     	leaq	0x1b0(%rbp), %r9
14077c634:     	xorl	%ecx, %ecx
14077c636:     	movq	%r13, %r8
14077c639:     	callq	0x1428cab41 <.text+0x28c9b41>
14077c63e:     	jmp	0x14077c8b0 <.text+0x77b8b0>
14077c643:     	testq	%rax, %rax
14077c646:     	jne	0x14077c675 <.text+0x77b675>
14077c648:     	xorl	%edx, %edx
14077c64a:     	callq	0x1428f15c0 <.text+0x28f05c0>
14077c64f:     	movq	%rax, %rcx
14077c652:     	testq	%rax, %rax
14077c655:     	jne	0x14077a53c <.text+0x77953c>
14077c65b:     	jmp	0x14077c675 <.text+0x77b675>
14077c65d:     	testq	%rax, %rax
14077c660:     	jne	0x14077c675 <.text+0x77b675>
14077c662:     	xorl	%edx, %edx
14077c664:     	callq	0x1428f15c0 <.text+0x28f05c0>
14077c669:     	movq	%rax, %rcx
14077c66c:     	testq	%rax, %rax
14077c66f:     	jne	0x14077c510 <.text+0x77b510>
14077c675:     	leaq	0x249ce2c(%rip), %rcx   # 0x142c194a8
14077c67c:     	callq	0x1428d80d0 <.text+0x28d70d0>
14077c681:     	movq	$0x0, 0x1b0(%rbp)
14077c68c:     	leaq	0x249e1cd(%rip), %rax   # 0x142c1a860
14077c693:     	movq	%rax, 0x20(%rsp)
14077c698:     	leaq	0x140(%rbp), %rdx
14077c69f:     	leaq	0x1b0(%rbp), %r9
