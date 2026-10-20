
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fc3840:     	pushq	%rbp
140fc3841:     	pushq	%r15
140fc3843:     	pushq	%r14
140fc3845:     	pushq	%r13
140fc3847:     	pushq	%r12
140fc3849:     	pushq	%rsi
140fc384a:     	pushq	%rdi
140fc384b:     	pushq	%rbx
140fc384c:     	subq	$0x428, %rsp            # imm = 0x428
140fc3853:     	leaq	0x80(%rsp), %rbp
140fc385b:     	movaps	%xmm10, 0x390(%rbp)
140fc3863:     	movaps	%xmm9, 0x380(%rbp)
140fc386b:     	movaps	%xmm8, 0x370(%rbp)
140fc3873:     	movdqa	%xmm7, 0x360(%rbp)
140fc387b:     	movdqa	%xmm6, 0x350(%rbp)
140fc3883:     	movq	$-0x2, 0x348(%rbp)
140fc388e:     	movq	(%rdx), %rdi
140fc3891:     	cmpq	$0x0, 0x1b8(%rdi)
140fc3899:     	je	0x140fc5671 <.text+0xfc4671>
140fc389f:     	movq	%rdx, %rsi
140fc38a2:     	movq	%rcx, %r15
140fc38a5:     	leaq	0x8(%rdx), %r13
140fc38a9:     	leaq	0x1c0(%rdi), %rcx
140fc38b0:     	movq	%r13, %rdx
140fc38b3:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc38b8:     	movq	0x1a0(%rdi), %rcx
140fc38bf:     	movq	0x1a8(%rdi), %rdx
140fc38c6:     	movq	%rdx, %r8
140fc38c9:     	andq	%rax, %r8
140fc38cc:     	shrq	$0x39, %rax
140fc38d0:     	movd	%eax, %xmm0
140fc38d4:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc38d8:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc38dd:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc38e2:     	leaq	-0xc0(%rcx), %rax
140fc38e9:     	movq	0x8(%rsi), %r9
140fc38ed:     	xorl	%r10d, %r10d
140fc38f0:     	pcmpeqd	%xmm1, %xmm1
140fc38f4:     	movdqu	(%rcx,%r8), %xmm2
140fc38fa:     	movdqa	%xmm2, %xmm3
140fc38fe:     	pcmpeqb	%xmm0, %xmm3
140fc3902:     	pmovmskb	%xmm3, %r11d
140fc3907:     	testl	%r11d, %r11d
140fc390a:     	je	0x140fc3940 <.text+0xfc2940>
140fc390c:     	tzcntl	%r11d, %edi
140fc3911:     	addq	%r8, %rdi
140fc3914:     	andq	%rdx, %rdi
140fc3917:     	negq	%rdi
140fc391a:     	leaq	(%rdi,%rdi,2), %rdi
140fc391e:     	shlq	$0x6, %rdi
140fc3922:     	cmpq	(%rax,%rdi), %r9
140fc3926:     	je	0x140fc3962 <.text+0xfc2962>
140fc3928:     	leal	-0x1(%r11), %edi
140fc392c:     	andw	%r11w, %di
140fc3930:     	movl	%edi, %r11d
140fc3933:     	jne	0x140fc390c <.text+0xfc290c>
140fc3935:     	nopw	%cs:(%rax,%rax)
140fc3940:     	pcmpeqb	%xmm1, %xmm2
140fc3944:     	pmovmskb	%xmm2, %r11d
140fc3949:     	testl	%r11d, %r11d
140fc394c:     	jne	0x140fc5671 <.text+0xfc4671>
140fc3952:     	addq	%r10, %r8
140fc3955:     	addq	$0x10, %r8
140fc3959:     	addq	$0x10, %r10
140fc395d:     	andq	%rdx, %r8
140fc3960:     	jmp	0x140fc38f4 <.text+0xfc28f4>
140fc3962:     	movq	%rsi, 0x18(%rbp)
140fc3966:     	orb	$0x4, -0x1(%rcx,%rdi)
140fc396b:     	movq	(%r15), %rax
140fc396e:     	movq	0x8(%r15), %rcx
140fc3972:     	movq	%rcx, 0x228(%rbp)
140fc3979:     	movq	(%rax), %r14
140fc397c:     	movq	%rax, 0x30(%rbp)
140fc3980:     	movq	0x8(%rax), %rax
140fc3984:     	leaq	(%rax,%rax,2), %rax
140fc3988:     	shlq	$0x4, %rax
140fc398c:     	addq	%r14, %rax
140fc398f:     	movq	0x10(%r15), %rcx
140fc3993:     	movq	%rcx, 0x38(%rbp)
140fc3997:     	movq	0x18(%r15), %rcx
140fc399b:     	movq	%rcx, 0x40(%rbp)
140fc399f:     	movq	0x20(%r15), %rcx
140fc39a3:     	movq	%rcx, 0x48(%rbp)
140fc39a7:     	movq	0x28(%r15), %rcx
140fc39ab:     	movq	%rcx, 0x50(%rbp)
140fc39af:     	movq	0x30(%r15), %rcx
140fc39b3:     	movq	%rcx, 0x58(%rbp)
140fc39b7:     	movq	0x38(%r15), %rcx
140fc39bb:     	movq	%rcx, 0x60(%rbp)
140fc39bf:     	movq	0x40(%r15), %rcx
140fc39c3:     	movq	%rcx, 0x150(%rbp)
140fc39ca:     	movq	%rax, %r15
140fc39cd:     	pcmpeqd	%xmm7, %xmm7
140fc39d1:     	xorps	%xmm8, %xmm8
140fc39d5:     	movd	0x1961e9f(%rip), %xmm6  # 0x14292587c
140fc39dd:     	movaps	0x1bef7bb(%rip), %xmm9  # 0x142bb31a0
140fc39e5:     	movsd	0x1a1a312(%rip), %xmm10 # 0x1429ddd00
140fc39ee:     	leaq	0x98(%rbp), %rcx
140fc39f5:     	leaq	0x1bf0614(%rip), %rbx   # 0x142bb4010
140fc39fc:     	leaq	0x280(%rbp), %rdi
140fc3a03:     	movq	%rax, 0x28(%rbp)
140fc3a07:     	cmpq	%r15, %r14
140fc3a0a:     	je	0x140fc56bc <.text+0xfc46bc>
140fc3a10:     	movl	0x20(%r14), %edx
140fc3a14:     	movl	0x24(%r14), %r8d
140fc3a18:     	callq	0x140c91d90 <.text+0xc90d90>
140fc3a1d:     	movl	0x98(%rbp), %r8d
140fc3a24:     	movq	0x9c(%rbp), %r9
140fc3a2b:     	movzwl	0xa4(%rbp), %eax
140fc3a32:     	movzbl	0xa6(%rbp), %ecx
140fc3a39:     	shll	$0x18, %ecx
140fc3a3c:     	shll	$0x8, %eax
140fc3a3f:     	addl	%ecx, %eax
140fc3a41:     	incl	%eax
140fc3a43:     	movl	%eax, 0x20(%rsp)
140fc3a47:     	movq	%rdi, %rsi
140fc3a4a:     	movq	%rdi, %rcx
140fc3a4d:     	movq	%rbx, %rdx
140fc3a50:     	callq	0x141e58d40 <.text+0x1e57d40>
140fc3a55:     	cmpl	$0x1, 0x280(%rbp)
140fc3a5c:     	je	0x140fc5718 <.text+0xfc4718>
140fc3a62:     	movq	0x288(%rbp), %rax
140fc3a69:     	movq	%rax, 0xa8(%rbp)
140fc3a70:     	movq	0x290(%rbp), %rax
140fc3a77:     	movq	0x298(%rbp), %r12
140fc3a7e:     	movq	(%r13), %rdi
140fc3a82:     	movq	%rax, 0x148(%rbp)
140fc3a89:     	movq	%rax, 0xd0(%rbp)
140fc3a90:     	movq	%r12, 0xd8(%rbp)
140fc3a97:     	leaq	0xd0(%rbp), %rcx
140fc3a9e:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fc3aa3:     	movq	%rax, 0x280(%rbp)
140fc3aaa:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140fc3ab4:     	movq	%rax, 0x288(%rbp)
140fc3abb:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
140fc3ac5:     	movq	%rax, 0x290(%rbp)
140fc3acc:     	movq	%rsi, %rcx
140fc3acf:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fc3ad4:     	movq	%rdi, 0x280(%rbp)
140fc3adb:     	movq	%rax, 0x288(%rbp)
140fc3ae2:     	movq	%rsi, %rcx
140fc3ae5:     	callq	0x1405bd910 <.text+0x5bc910>
140fc3aea:     	leaq	0x160(%rbp), %rcx
140fc3af1:     	movq	0x18(%rbp), %rdx
140fc3af5:     	movq	%rax, %r8
140fc3af8:     	callq	0x1406861b0 <.text+0x6851b0>
140fc3afd:     	movq	0x160(%rbp), %rdi
140fc3b04:     	cmpq	$0x0, 0x1b8(%rdi)
140fc3b0c:     	je	0x140fc567d <.text+0xfc467d>
140fc3b12:     	leaq	0x1c0(%rdi), %rcx
140fc3b19:     	leaq	0x168(%rbp), %rdx
140fc3b20:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc3b25:     	leaq	0x30(%r14), %rcx
140fc3b29:     	movq	%rcx, 0x20(%rbp)
140fc3b2d:     	movq	0x1a0(%rdi), %rcx
140fc3b34:     	movq	0x1a8(%rdi), %rdx
140fc3b3b:     	movq	%rdx, %r8
140fc3b3e:     	andq	%rax, %r8
140fc3b41:     	shrq	$0x39, %rax
140fc3b45:     	movd	%eax, %xmm0
140fc3b49:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc3b4d:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc3b52:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc3b57:     	leaq	-0xc0(%rcx), %rax
140fc3b5e:     	movq	0x168(%rbp), %r9
140fc3b65:     	xorl	%r10d, %r10d
140fc3b68:     	movdqu	(%rcx,%r8), %xmm1
140fc3b6e:     	movdqa	%xmm1, %xmm2
140fc3b72:     	pcmpeqb	%xmm0, %xmm2
140fc3b76:     	pmovmskb	%xmm2, %r11d
140fc3b7b:     	testl	%r11d, %r11d
140fc3b7e:     	je	0x140fc3bb0 <.text+0xfc2bb0>
140fc3b80:     	tzcntl	%r11d, %edi
140fc3b85:     	addq	%r8, %rdi
140fc3b88:     	andq	%rdx, %rdi
140fc3b8b:     	negq	%rdi
140fc3b8e:     	leaq	(%rdi,%rdi,2), %rdi
140fc3b92:     	shlq	$0x6, %rdi
140fc3b96:     	cmpq	(%rax,%rdi), %r9
140fc3b9a:     	je	0x140fc3be0 <.text+0xfc2be0>
140fc3b9c:     	leal	-0x1(%r11), %edi
140fc3ba0:     	andw	%r11w, %di
140fc3ba4:     	movl	%edi, %r11d
140fc3ba7:     	jne	0x140fc3b80 <.text+0xfc2b80>
140fc3ba9:     	nopl	(%rax)
140fc3bb0:     	pcmpeqb	%xmm7, %xmm1
140fc3bb4:     	pmovmskb	%xmm1, %r11d
140fc3bb9:     	testl	%r11d, %r11d
140fc3bbc:     	jne	0x140fc567d <.text+0xfc467d>
140fc3bc2:     	addq	%r10, %r8
140fc3bc5:     	addq	$0x10, %r8
140fc3bc9:     	addq	$0x10, %r10
140fc3bcd:     	andq	%rdx, %r8
140fc3bd0:     	jmp	0x140fc3b68 <.text+0xfc2b68>
140fc3bd2:     	nopw	%cs:(%rax,%rax)
140fc3be0:     	movups	%xmm8, -0x90(%rcx,%rdi)
140fc3be9:     	movups	%xmm8, -0xa0(%rcx,%rdi)
140fc3bf2:     	movabsq	$0x4120000000000000, %rax # imm = 0x4120000000000000
140fc3bfc:     	movq	%rax, -0x80(%rcx,%rdi)
140fc3c01:     	movups	%xmm8, -0x78(%rcx,%rdi)
140fc3c07:     	movups	%xmm8, -0x6a(%rcx,%rdi)
140fc3c0d:     	movb	$0x4, -0x5a(%rcx,%rdi)
140fc3c12:     	movq	0x170(%rbp), %rax
140fc3c19:     	movq	%rax, 0xc0(%rbp)
140fc3c20:     	movdqu	0x160(%rbp), %xmm0
140fc3c28:     	movdqa	%xmm0, 0xb0(%rbp)
140fc3c30:     	movq	0xb0(%rbp), %rdi
140fc3c37:     	cmpq	$0x0, 0x1b8(%rdi)
140fc3c3f:     	je	0x140fc568e <.text+0xfc468e>
140fc3c45:     	movq	0xb8(%rbp), %rax
140fc3c4c:     	movq	%rax, 0x68(%rbp)
140fc3c50:     	leaq	0x1c0(%rdi), %rcx
140fc3c57:     	leaq	0xb8(%rbp), %rdx
140fc3c5e:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc3c63:     	movq	0x1a0(%rdi), %rbx
140fc3c6a:     	movq	0x1a8(%rdi), %rdx
140fc3c71:     	movq	%rdx, %r8
140fc3c74:     	andq	%rax, %r8
140fc3c77:     	shrq	$0x39, %rax
140fc3c7b:     	movd	%eax, %xmm0
140fc3c7f:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc3c83:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc3c88:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc3c8d:     	leaq	-0xc0(%rbx), %rax
140fc3c94:     	movq	0xb8(%rbp), %r9
140fc3c9b:     	xorl	%r10d, %r10d
140fc3c9e:     	movdqu	(%rbx,%r8), %xmm1
140fc3ca4:     	movdqa	%xmm1, %xmm2
140fc3ca8:     	pcmpeqb	%xmm0, %xmm2
140fc3cac:     	pmovmskb	%xmm2, %edi
140fc3cb0:     	testl	%edi, %edi
140fc3cb2:     	je	0x140fc3cdd <.text+0xfc2cdd>
140fc3cb4:     	tzcntl	%edi, %r11d
140fc3cb9:     	addq	%r8, %r11
140fc3cbc:     	andq	%rdx, %r11
140fc3cbf:     	negq	%r11
140fc3cc2:     	leaq	(%r11,%r11,2), %r11
140fc3cc6:     	shlq	$0x6, %r11
140fc3cca:     	cmpq	(%rax,%r11), %r9
140fc3cce:     	je	0x140fc3d00 <.text+0xfc2d00>
140fc3cd0:     	leal	-0x1(%rdi), %r11d
140fc3cd4:     	andw	%di, %r11w
140fc3cd8:     	movl	%r11d, %edi
140fc3cdb:     	jne	0x140fc3cb4 <.text+0xfc2cb4>
140fc3cdd:     	pcmpeqb	%xmm7, %xmm1
140fc3ce1:     	pmovmskb	%xmm1, %r11d
140fc3ce6:     	testl	%r11d, %r11d
140fc3ce9:     	jne	0x140fc568e <.text+0xfc468e>
140fc3cef:     	addq	%r10, %r8
140fc3cf2:     	addq	$0x10, %r8
140fc3cf6:     	addq	$0x10, %r10
140fc3cfa:     	andq	%rdx, %r8
140fc3cfd:     	jmp	0x140fc3c9e <.text+0xfc2c9e>
140fc3cff:     	nop
140fc3d00:     	movzbl	0x28(%r14), %eax
140fc3d05:     	leaq	0x1bf0ef8(%rip), %rcx   # 0x142bb4c04
140fc3d0c:     	movslq	(%rcx,%rax,4), %rax
140fc3d10:     	addq	%rcx, %rax
140fc3d13:     	jmpq	*%rax
140fc3d15:     	movl	$0x6, %eax
140fc3d1a:     	movq	%rax, 0x270(%rbp)
140fc3d21:     	leaq	0x1bf0338(%rip), %rax   # 0x142bb4060
140fc3d28:     	jmp	0x140fc3d67 <.text+0xfc2d67>
140fc3d2a:     	movl	$0x4, %eax
140fc3d2f:     	movq	%rax, 0x270(%rbp)
140fc3d36:     	leaq	0x1bf0331(%rip), %rax   # 0x142bb406e
140fc3d3d:     	jmp	0x140fc3d67 <.text+0xfc2d67>
140fc3d3f:     	movl	$0x4, %eax
140fc3d44:     	movq	%rax, 0x270(%rbp)
140fc3d4b:     	leaq	0x1bf0314(%rip), %rax   # 0x142bb4066
140fc3d52:     	jmp	0x140fc3d67 <.text+0xfc2d67>
140fc3d54:     	movl	$0x4, %eax
140fc3d59:     	movq	%rax, 0x270(%rbp)
140fc3d60:     	leaq	0x1bf0303(%rip), %rax   # 0x142bb406a
140fc3d67:     	movq	%rax, 0x278(%rbp)
140fc3d6e:     	movq	%rsi, %rcx
140fc3d71:     	movzbl	-0x2(%rbx,%r11), %eax
140fc3d77:     	movb	%al, 0x32f(%rbp)
140fc3d7d:     	leaq	0xb0(%rbp), %rdx
140fc3d84:     	movq	0x148(%rbp), %r8
140fc3d8b:     	movq	%r12, %r9
140fc3d8e:     	callq	0x14068bfc0 <.text+0x68afc0>
140fc3d93:     	movq	%rsi, %rcx
140fc3d96:     	movb	$0x1, %dl
140fc3d98:     	callq	0x140f9dba0 <.text+0xf9cba0>
140fc3d9d:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
140fc3da7:     	movdqu	0x280(%rbp), %xmm0
140fc3daf:     	movdqa	%xmm0, 0x160(%rbp)
140fc3db7:     	movq	0x290(%rbp), %rax
140fc3dbe:     	movq	%rax, 0x170(%rbp)
140fc3dc5:     	movq	0x160(%rbp), %rdi
140fc3dcc:     	cmpq	$0x0, 0x1b8(%rdi)
140fc3dd4:     	je	0x140fc569f <.text+0xfc469f>
140fc3dda:     	leaq	0x1c0(%rdi), %rcx
140fc3de1:     	leaq	0x168(%rbp), %rdx
140fc3de8:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc3ded:     	movq	0x1a0(%rdi), %rcx
140fc3df4:     	movq	0x1a8(%rdi), %rdx
140fc3dfb:     	movq	%rdx, %r8
140fc3dfe:     	andq	%rax, %r8
140fc3e01:     	shrq	$0x39, %rax
140fc3e05:     	movd	%eax, %xmm0
140fc3e09:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc3e0d:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc3e12:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc3e17:     	leaq	-0xc0(%rcx), %rax
140fc3e1e:     	movq	0x168(%rbp), %r9
140fc3e25:     	xorl	%r10d, %r10d
140fc3e28:     	movdqu	(%rcx,%r8), %xmm1
140fc3e2e:     	movdqa	%xmm1, %xmm2
140fc3e32:     	pcmpeqb	%xmm0, %xmm2
140fc3e36:     	pmovmskb	%xmm2, %r11d
140fc3e3b:     	testl	%r11d, %r11d
140fc3e3e:     	je	0x140fc3e69 <.text+0xfc2e69>
140fc3e40:     	tzcntl	%r11d, %edi
140fc3e45:     	addq	%r8, %rdi
140fc3e48:     	andq	%rdx, %rdi
140fc3e4b:     	negq	%rdi
140fc3e4e:     	leaq	(%rdi,%rdi,2), %rdi
140fc3e52:     	shlq	$0x6, %rdi
140fc3e56:     	cmpq	(%rax,%rdi), %r9
140fc3e5a:     	je	0x140fc3e8b <.text+0xfc2e8b>
140fc3e5c:     	leal	-0x1(%r11), %edi
140fc3e60:     	andw	%r11w, %di
140fc3e64:     	movl	%edi, %r11d
140fc3e67:     	jne	0x140fc3e40 <.text+0xfc2e40>
140fc3e69:     	pcmpeqb	%xmm7, %xmm1
140fc3e6d:     	pmovmskb	%xmm1, %r11d
140fc3e72:     	testl	%r11d, %r11d
140fc3e75:     	jne	0x140fc569f <.text+0xfc469f>
140fc3e7b:     	addq	%r10, %r8
140fc3e7e:     	addq	$0x10, %r8
140fc3e82:     	addq	$0x10, %r10
140fc3e86:     	andq	%rdx, %r8
140fc3e89:     	jmp	0x140fc3e28 <.text+0xfc2e28>
140fc3e8b:     	testb	$0x30, -0x2(%rcx,%rdi)
140fc3e90:     	je	0x140fc4f63 <.text+0xfc3f63>
140fc3e96:     	movq	%rsi, %rcx
140fc3e99:     	movq	0x228(%rbp), %rdx
140fc3ea0:     	callq	0x140b46000 <.text+0xb45000>
140fc3ea5:     	movq	%rsi, %rdi
140fc3ea8:     	movq	%rsi, %rcx
140fc3eab:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc3eb0:     	movq	%rdx, %r8
140fc3eb3:     	movq	$0xc, 0x20(%rsp)
140fc3ebc:     	leaq	-0x18(%rbp), %rcx
140fc3ec0:     	movq	%rax, %rdx
140fc3ec3:     	leaq	0x1bf01a8(%rip), %r9    # 0x142bb4072
140fc3eca:     	callq	0x140d62810 <.text+0xd61810>
140fc3ecf:     	movq	0x280(%rbp), %rdx
140fc3ed6:     	testq	%rdx, %rdx
140fc3ed9:     	je	0x140fc3eed <.text+0xfc2eed>
140fc3edb:     	movq	0x288(%rbp), %rcx
140fc3ee2:     	movl	$0x1, %r8d
140fc3ee8:     	callq	0x140613c20 <.text+0x612c20>
140fc3eed:     	movq	0x228(%rbp), %rax
140fc3ef4:     	movdqu	(%rax), %xmm0
140fc3ef8:     	movdqa	%xmm0, 0x160(%rbp)
140fc3f00:     	movq	%rdi, %rcx
140fc3f03:     	leaq	0x160(%rbp), %rdx
140fc3f0a:     	callq	0x141e23c00 <.text+0x1e22c00>
140fc3f0f:     	xorl	%eax, %eax
140fc3f11:     	cmpq	0x280(%rbp), %rax
140fc3f18:     	jno	0x140fc3f50 <.text+0xfc2f50>
140fc3f1a:     	leaq	0x160(%rbp), %rcx
140fc3f21:     	callq	0x140b468f0 <.text+0xb458f0>
140fc3f26:     	leaq	0x288(%rbp), %rcx
140fc3f2d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc3f32:     	jmp	0x140fc3fac <.text+0xfc2fac>
140fc3f34:     	movl	$0x1, %eax
140fc3f39:     	movq	%rax, 0x278(%rbp)
140fc3f40:     	movq	$0x0, 0x270(%rbp)
140fc3f4b:     	jmp	0x140fc3d6e <.text+0xfc2d6e>
140fc3f50:     	movups	0x2d0(%rbp), %xmm0
140fc3f57:     	movaps	%xmm0, 0x1b0(%rbp)
140fc3f5e:     	movups	0x2c0(%rbp), %xmm0
140fc3f65:     	movaps	%xmm0, 0x1a0(%rbp)
140fc3f6c:     	movdqu	0x280(%rbp), %xmm0
140fc3f74:     	movdqu	0x290(%rbp), %xmm1
140fc3f7c:     	movdqu	0x2a0(%rbp), %xmm2
140fc3f84:     	movdqu	0x2b0(%rbp), %xmm3
140fc3f8c:     	movdqa	%xmm3, 0x190(%rbp)
140fc3f94:     	movdqa	%xmm2, 0x180(%rbp)
140fc3f9c:     	movdqa	%xmm1, 0x170(%rbp)
140fc3fa4:     	movdqa	%xmm0, 0x160(%rbp)
140fc3fac:     	movq	%rdi, %rcx
140fc3faf:     	leaq	0x160(%rbp), %rdx
140fc3fb6:     	callq	0x140b46370 <.text+0xb45370>
140fc3fbb:     	leaq	0xd0(%rbp), %rcx
140fc3fc2:     	movq	%rdi, %rdx
140fc3fc5:     	callq	0x1408bbbc0 <.text+0x8babc0>
140fc3fca:     	movq	0x160(%rbp), %rdx
140fc3fd1:     	testq	%rdx, %rdx
140fc3fd4:     	je	0x140fc3fe8 <.text+0xfc2fe8>
140fc3fd6:     	movq	0x168(%rbp), %rcx
140fc3fdd:     	movl	$0x1, %r8d
140fc3fe3:     	callq	0x140613c20 <.text+0x612c20>
140fc3fe8:     	movq	0x178(%rbp), %rdx
140fc3fef:     	movq	%rdx, %rax
140fc3ff2:     	negq	%rax
140fc3ff5:     	jo	0x140fc400b <.text+0xfc300b>
140fc3ff7:     	jae	0x140fc400b <.text+0xfc300b>
140fc3ff9:     	movq	0x180(%rbp), %rcx
140fc4000:     	movl	$0x1, %r8d
140fc4006:     	callq	0x140613c20 <.text+0x612c20>
140fc400b:     	movq	%r14, %rcx
140fc400e:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4013:     	movq	%rdx, %r8
140fc4016:     	leaq	0x8(%rbp), %rcx
140fc401a:     	movq	%rax, %rdx
140fc401d:     	leaq	0xd0(%rbp), %r9
140fc4024:     	callq	0x141470d00 <.text+0x146fd00>
140fc4029:     	cmpb	$0x2, 0x17(%rbp)
140fc402d:     	movq	%r13, 0x158(%rbp)
140fc4034:     	jne	0x140fc417b <.text+0xfc317b>
140fc403a:     	movq	0x8(%rbp), %rax
140fc403e:     	movq	%rax, 0x110(%rbp)
140fc4045:     	movq	0x266835c(%rip), %rax   # 0x14362c3a8
140fc404c:     	movq	(%rax), %rax
140fc404f:     	testq	%rax, %rax
140fc4052:     	je	0x140fc4137 <.text+0xfc3137>
140fc4058:     	leaq	0x110(%rbp), %rax
140fc405f:     	movq	%rax, 0x160(%rbp)
140fc4066:     	leaq	-0x27744d(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
140fc406d:     	movq	%rax, 0x168(%rbp)
140fc4074:     	leaq	0x1bf0145(%rip), %rcx   # 0x142bb41c0
140fc407b:     	callq	0x14067dd80 <.text+0x67cd80>
140fc4080:     	movdqu	(%rax), %xmm0
140fc4084:     	movl	0x10(%rax), %eax
140fc4087:     	movq	$0x1, 0x2b0(%rbp)
140fc4092:     	leaq	0x1bf0077(%rip), %rdx   # 0x142bb4110
140fc4099:     	movq	%rdx, 0x2b8(%rbp)
140fc40a0:     	movq	$0x13, 0x2c0(%rbp)
140fc40ab:     	leaq	0x1bf00fe(%rip), %rcx   # 0x142bb41b0
140fc40b2:     	movq	%rcx, 0x2d0(%rbp)
140fc40b9:     	movq	$0x1, 0x2d8(%rbp)
140fc40c4:     	leaq	0x160(%rbp), %rcx
140fc40cb:     	movq	%rcx, 0x2e0(%rbp)
140fc40d2:     	movq	$0x1, 0x2e8(%rbp)
140fc40dd:     	movq	$0x0, 0x2f0(%rbp)
140fc40e8:     	movq	$0x0, 0x280(%rbp)
140fc40f3:     	movq	%rdx, 0x288(%rbp)
140fc40fa:     	movq	$0x13, 0x290(%rbp)
140fc4105:     	movq	$0x0, 0x298(%rbp)
140fc4110:     	movdqu	%xmm0, 0x2a0(%rbp)
140fc4118:     	movl	$0x1, 0x2c8(%rbp)
140fc4122:     	movl	%eax, 0x2cc(%rbp)
140fc4128:     	leaq	0x138(%rbp), %rcx
140fc412f:     	movq	%rdi, %rdx
140fc4132:     	callq	0x140d4a330 <.text+0xd49330>
140fc4137:     	movq	0x10(%r14), %rbx
140fc413b:     	testq	%rbx, %rbx
140fc413e:     	js	0x140fc578d <.text+0xfc478d>
140fc4144:     	movq	0x8(%r14), %r12
140fc4148:     	je	0x140fc49ba <.text+0xfc39ba>
140fc414e:     	movzbl	0x2670ab4(%rip), %eax   # 0x143634c09
140fc4155:     	movl	$0x1, %edi
140fc415a:     	movl	$0x1, %edx
140fc415f:     	movq	%rbx, %rcx
140fc4162:     	callq	0x140613c10 <.text+0x612c10>
140fc4167:     	testq	%rax, %rax
140fc416a:     	je	0x140fc578f <.text+0xfc478f>
140fc4170:     	movq	%rax, %r13
140fc4173:     	movq	%rsi, %rdi
140fc4176:     	jmp	0x140fc49c0 <.text+0xfc39c0>
140fc417b:     	movdqu	0x8(%rbp), %xmm0
140fc4180:     	movdqa	%xmm0, 0x80(%rbp)
140fc4188:     	movb	$0x1, 0x344(%rbp)
140fc418f:     	leaq	0x160(%rbp), %rbx
140fc4196:     	movq	%rbx, %rcx
140fc4199:     	movq	0x228(%rbp), %rdx
140fc41a0:     	callq	0x140b46180 <.text+0xb45180>
140fc41a5:     	movq	%rbx, %rcx
140fc41a8:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc41ad:     	leaq	0x230(%rbp), %r15
140fc41b4:     	movq	%rax, %rbx
140fc41b7:     	movq	%rdx, %r12
140fc41ba:     	movq	%r15, %rcx
140fc41bd:     	callq	0x140c96690 <.text+0xc95690>
140fc41c2:     	movq	%r15, 0x138(%rbp)
140fc41c9:     	leaq	-0xf88700(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140fc41d0:     	movq	%rax, 0x140(%rbp)
140fc41d7:     	leaq	0x1befeba(%rip), %rax   # 0x142bb4098
140fc41de:     	movq	%rax, 0x280(%rbp)
140fc41e5:     	movq	$0x2, 0x288(%rbp)
140fc41f0:     	leaq	0x138(%rbp), %rax
140fc41f7:     	movq	%rax, 0x290(%rbp)
140fc41fe:     	movq	$0x1, 0x298(%rbp)
140fc4209:     	movq	$0x0, 0x2a0(%rbp)
140fc4214:     	leaq	0x200(%rbp), %rcx
140fc421b:     	movq	%rdi, %rdx
140fc421e:     	callq	0x140d79f90 <.text+0xd78f90>
140fc4223:     	movq	0x230(%rbp), %rdx
140fc422a:     	testq	%rdx, %rdx
140fc422d:     	je	0x140fc4241 <.text+0xfc3241>
140fc422f:     	movq	0x238(%rbp), %rcx
140fc4236:     	movl	$0x1, %r8d
140fc423c:     	callq	0x140613c20 <.text+0x612c20>
140fc4241:     	movq	0x200(%rbp), %rax
140fc4248:     	movq	%rax, 0x338(%rbp)
140fc424f:     	movq	0x208(%rbp), %r9
140fc4256:     	movq	0x210(%rbp), %rax
140fc425d:     	movq	%rax, 0x20(%rsp)
140fc4262:     	leaq	0x110(%rbp), %rcx
140fc4269:     	movq	%rbx, %rdx
140fc426c:     	movq	%r12, %r8
140fc426f:     	movq	%r9, 0x330(%rbp)
140fc4276:     	callq	0x140d62810 <.text+0xd61810>
140fc427b:     	movq	0x338(%rbp), %rdx
140fc4282:     	testq	%rdx, %rdx
140fc4285:     	je	0x140fc4299 <.text+0xfc3299>
140fc4287:     	movl	$0x1, %r8d
140fc428d:     	movq	0x330(%rbp), %rcx
140fc4294:     	callq	0x140613c20 <.text+0x612c20>
140fc4299:     	movq	0x160(%rbp), %rdx
140fc42a0:     	testq	%rdx, %rdx
140fc42a3:     	je	0x140fc42b7 <.text+0xfc32b7>
140fc42a5:     	movq	0x168(%rbp), %rcx
140fc42ac:     	movl	$0x1, %r8d
140fc42b2:     	callq	0x140613c20 <.text+0x612c20>
140fc42b7:     	movq	0x30(%rbp), %rax
140fc42bb:     	movq	(%rax), %r13
140fc42be:     	movq	0x8(%rax), %rax
140fc42c2:     	leaq	(%rax,%rax,2), %rdi
140fc42c6:     	shlq	$0x4, %rdi
140fc42ca:     	addq	%r13, %rdi
140fc42cd:     	movq	0x38(%rbp), %rax
140fc42d1:     	movq	(%rax), %r15
140fc42d4:     	jmp	0x140fc42f5 <.text+0xfc32f5>
140fc42d6:     	movl	$0x18, %edx
140fc42db:     	movl	$0x8, %r8d
140fc42e1:     	movq	0x320(%rbp), %rcx
140fc42e8:     	callq	0x140613c20 <.text+0x612c20>
140fc42ed:     	testb	%bl, %bl
140fc42ef:     	jne	0x140fc4c6b <.text+0xfc3c6b>
140fc42f5:     	cmpq	%rdi, %r13
140fc42f8:     	je	0x140fc467f <.text+0xfc367f>
140fc42fe:     	movq	%r13, %r12
140fc4301:     	leaq	0x28(%r13), %rcx
140fc4305:     	movb	$0x1, 0x347(%rbp)
140fc430c:     	movb	$0x1, 0x346(%rbp)
140fc4313:     	callq	0x140ae8320 <.text+0xae7320>
140fc4318:     	leaq	0x30(%r12), %r13
140fc431d:     	testb	%al, %al
140fc431f:     	jne	0x140fc42f5 <.text+0xfc32f5>
140fc4321:     	movl	$0x0, 0x280(%rbp)
140fc432b:     	leaq	0x288(%rbp), %rax
140fc4332:     	movups	%xmm8, (%rax)
140fc4336:     	movl	$0x7, 0x298(%rbp)
140fc4340:     	movw	$0x0, 0x1c(%rax)
140fc4346:     	movq	$0x0, 0x14(%rax)
140fc434e:     	movb	$0x1, 0x2a0(%rbp)
140fc4355:     	movb	$0x1, 0x347(%rbp)
140fc435c:     	movb	$0x1, 0x346(%rbp)
140fc4363:     	movq	%r12, %rcx
140fc4366:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc436b:     	movq	%rdx, %r8
140fc436e:     	movb	$0x1, 0x347(%rbp)
140fc4375:     	movb	$0x1, 0x346(%rbp)
140fc437c:     	movq	%rsi, %rcx
140fc437f:     	movq	%rax, %rdx
140fc4382:     	callq	0x140d59dc0 <.text+0xd58dc0>
140fc4387:     	movq	%rax, 0x200(%rbp)
140fc438e:     	movq	%rdx, 0x208(%rbp)
140fc4395:     	testb	$0x1, %al
140fc4397:     	jne	0x140fc4600 <.text+0xfc3600>
140fc439d:     	movq	%rdx, %rcx
140fc43a0:     	callq	0x141e58f20 <.text+0x1e57f20>
140fc43a5:     	movq	%rdx, %rbx
140fc43a8:     	testq	%rax, %rax
140fc43ab:     	je	0x140fc44df <.text+0xfc34df>
140fc43b1:     	movq	%rbx, 0x230(%rbp)
140fc43b8:     	movq	0x2667fe9(%rip), %rax   # 0x14362c3a8
140fc43bf:     	movq	(%rax), %rax
140fc43c2:     	cmpq	$0x4, %rax
140fc43c6:     	jb	0x140fc44c0 <.text+0xfc34c0>
140fc43cc:     	movq	%r12, 0x160(%rbp)
140fc43d3:     	leaq	-0x2628fa(%rip), %rax   # 0x140d61ae0 <.text+0xd60ae0>
140fc43da:     	movq	%rax, 0x168(%rbp)
140fc43e1:     	leaq	0x230(%rbp), %rax
140fc43e8:     	movq	%rax, 0x170(%rbp)
140fc43ef:     	leaq	-0x2777d6(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
140fc43f6:     	movq	%rax, 0x178(%rbp)
140fc43fd:     	leaq	0x1befea4(%rip), %rcx   # 0x142bb42a8
140fc4404:     	callq	0x14067dd80 <.text+0x67cd80>
140fc4409:     	movdqu	(%rax), %xmm0
140fc440d:     	movl	0x10(%rax), %eax
140fc4410:     	movq	$0x4, 0x2b0(%rbp)
140fc441b:     	leaq	0x1befcee(%rip), %rdx   # 0x142bb4110
140fc4422:     	movq	%rdx, 0x2b8(%rbp)
140fc4429:     	movq	$0x13, 0x2c0(%rbp)
140fc4434:     	leaq	0x1befe4d(%rip), %rcx   # 0x142bb4288
140fc443b:     	movq	%rcx, 0x2d0(%rbp)
140fc4442:     	movq	$0x2, 0x2d8(%rbp)
140fc444d:     	leaq	0x160(%rbp), %rcx
140fc4454:     	movq	%rcx, 0x2e0(%rbp)
140fc445b:     	movq	$0x2, 0x2e8(%rbp)
140fc4466:     	movq	$0x0, 0x2f0(%rbp)
140fc4471:     	movq	$0x0, 0x280(%rbp)
140fc447c:     	movq	%rdx, 0x288(%rbp)
140fc4483:     	movq	$0x13, 0x290(%rbp)
140fc448e:     	movq	$0x0, 0x298(%rbp)
140fc4499:     	movdqu	%xmm0, 0x2a0(%rbp)
140fc44a1:     	movl	$0x1, 0x2c8(%rbp)
140fc44ab:     	movl	%eax, 0x2cc(%rbp)
140fc44b1:     	leaq	0x138(%rbp), %rcx
140fc44b8:     	movq	%rsi, %rdx
140fc44bb:     	callq	0x140d4a330 <.text+0xd49330>
140fc44c0:     	movq	%r12, %rcx
140fc44c3:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc44c8:     	movq	%rax, %rcx
140fc44cb:     	callq	0x141673140 <.text+0x1672140>
140fc44d0:     	movq	%rax, %rbx
140fc44d3:     	leaq	0x230(%rbp), %rcx
140fc44da:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc44df:     	cmpq	0x60(%r15), %rbx
140fc44e3:     	jne	0x140fc4600 <.text+0xfc3600>
140fc44e9:     	movq	0x2667eb8(%rip), %rax   # 0x14362c3a8
140fc44f0:     	movq	(%rax), %rax
140fc44f3:     	movb	$0x1, %bl
140fc44f5:     	cmpq	$0x3, %rax
140fc44f9:     	jb	0x140fc4602 <.text+0xfc3602>
140fc44ff:     	movq	%r12, %rcx
140fc4502:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4507:     	movq	%rax, %rcx
140fc450a:     	callq	0x140d61e00 <.text+0xd60e00>
140fc450f:     	movq	%rax, 0x160(%rbp)
140fc4516:     	movq	%rdx, 0x168(%rbp)
140fc451d:     	leaq	0x160(%rbp), %rax
140fc4524:     	movq	%rax, 0x230(%rbp)
140fc452b:     	leaq	-0xcc52(%rip), %rax     # 0x140fb78e0 <.text+0xfb68e0>
140fc4532:     	movq	%rax, 0x238(%rbp)
140fc4539:     	leaq	0x1befde0(%rip), %rcx   # 0x142bb4320
140fc4540:     	callq	0x14067dd80 <.text+0x67cd80>
140fc4545:     	movdqu	(%rax), %xmm0
140fc4549:     	movl	0x10(%rax), %eax
140fc454c:     	movq	$0x3, 0x2b0(%rbp)
140fc4557:     	leaq	0x1befbb2(%rip), %rdx   # 0x142bb4110
140fc455e:     	movq	%rdx, 0x2b8(%rbp)
140fc4565:     	movq	$0x13, 0x2c0(%rbp)
140fc4570:     	leaq	0x1befd89(%rip), %rcx   # 0x142bb4300
140fc4577:     	movq	%rcx, 0x2d0(%rbp)
140fc457e:     	movq	$0x2, 0x2d8(%rbp)
140fc4589:     	leaq	0x230(%rbp), %rcx
140fc4590:     	movq	%rcx, 0x2e0(%rbp)
140fc4597:     	movq	$0x1, 0x2e8(%rbp)
140fc45a2:     	movq	$0x0, 0x2f0(%rbp)
140fc45ad:     	movq	$0x0, 0x280(%rbp)
140fc45b8:     	movq	%rdx, 0x288(%rbp)
140fc45bf:     	movq	$0x13, 0x290(%rbp)
140fc45ca:     	movq	$0x0, 0x298(%rbp)
140fc45d5:     	movdqu	%xmm0, 0x2a0(%rbp)
140fc45dd:     	movl	$0x1, 0x2c8(%rbp)
140fc45e7:     	movl	%eax, 0x2cc(%rbp)
140fc45ed:     	leaq	0x138(%rbp), %rcx
140fc45f4:     	movq	%rsi, %rdx
140fc45f7:     	callq	0x140d4a330 <.text+0xd49330>
140fc45fc:     	jmp	0x140fc4602 <.text+0xfc3602>
140fc45fe:     	nop
140fc4600:     	xorl	%ebx, %ebx
140fc4602:     	cmpl	$0x1, 0x200(%rbp)
140fc4609:     	jne	0x140fc42ed <.text+0xfc32ed>
140fc460f:     	movq	0x208(%rbp), %rax
140fc4616:     	movl	%eax, %ecx
140fc4618:     	andl	$0x3, %ecx
140fc461b:     	cmpl	$0x1, %ecx
140fc461e:     	jne	0x140fc42ed <.text+0xfc32ed>
140fc4624:     	leaq	-0x1(%rax), %rcx
140fc4628:     	movq	%rcx, 0x320(%rbp)
140fc462f:     	movq	-0x1(%rax), %rcx
140fc4633:     	movq	%rcx, 0x338(%rbp)
140fc463a:     	movq	0x7(%rax), %rax
140fc463e:     	movq	%rax, 0x330(%rbp)
140fc4645:     	movq	(%rax), %rax
140fc4648:     	testq	%rax, %rax
140fc464b:     	je	0x140fc4656 <.text+0xfc3656>
140fc464d:     	movq	0x338(%rbp), %rcx
140fc4654:     	callq	*%rax
140fc4656:     	movq	0x338(%rbp), %rcx
140fc465d:     	movq	0x330(%rbp), %rax
140fc4664:     	movq	0x8(%rax), %rdx
140fc4668:     	testq	%rdx, %rdx
140fc466b:     	je	0x140fc42d6 <.text+0xfc32d6>
140fc4671:     	movq	0x10(%rax), %r8
140fc4675:     	callq	0x140613c20 <.text+0x612c20>
140fc467a:     	jmp	0x140fc42d6 <.text+0xfc32d6>
140fc467f:     	movq	0x2667d22(%rip), %rax   # 0x14362c3a8
140fc4686:     	movq	(%rax), %rax
140fc4689:     	cmpq	$0x3, %rax
140fc468d:     	jb	0x140fc4820 <.text+0xfc3820>
140fc4693:     	movb	$0x1, 0x347(%rbp)
140fc469a:     	movb	$0x1, 0x346(%rbp)
140fc46a1:     	leaq	-0x18(%rbp), %rcx
140fc46a5:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc46aa:     	movq	%rdx, %r8
140fc46ad:     	movb	$0x1, 0x347(%rbp)
140fc46b4:     	movb	$0x1, 0x346(%rbp)
140fc46bb:     	movl	$0x3, %r9d
140fc46c1:     	leaq	0x230(%rbp), %rcx
140fc46c8:     	movq	%rax, %rdx
140fc46cb:     	callq	0x1416f6ac0 <.text+0x16f5ac0>
140fc46d0:     	leaq	0x110(%rbp), %rcx
140fc46d7:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc46dc:     	leaq	0x160(%rbp), %rdi
140fc46e3:     	movq	%rdx, %r8
140fc46e6:     	movl	$0x3, %r9d
140fc46ec:     	movq	%rdi, %rcx
140fc46ef:     	movq	%rax, %rdx
140fc46f2:     	callq	0x1416f6ac0 <.text+0x16f5ac0>
140fc46f7:     	leaq	0x230(%rbp), %rax
140fc46fe:     	movq	%rax, 0x200(%rbp)
140fc4705:     	leaq	-0x262c2c(%rip), %rax   # 0x140d61ae0 <.text+0xd60ae0>
140fc470c:     	movq	%rax, 0x208(%rbp)
140fc4713:     	movq	%rdi, 0x210(%rbp)
140fc471a:     	movq	%rax, 0x218(%rbp)
140fc4721:     	leaq	0x1bef9d0(%rip), %rcx   # 0x142bb40f8
140fc4728:     	callq	0x14067dd80 <.text+0x67cd80>
140fc472d:     	movq	%rsi, %rdx
140fc4730:     	movdqu	(%rax), %xmm0
140fc4734:     	movl	0x10(%rax), %eax
140fc4737:     	movq	$0x3, 0x2b0(%rbp)
140fc4742:     	leaq	0x1bef9c7(%rip), %r8    # 0x142bb4110
140fc4749:     	movq	%r8, 0x2b8(%rbp)
140fc4750:     	movq	$0x13, 0x2c0(%rbp)
140fc475b:     	leaq	0x1bef976(%rip), %rcx   # 0x142bb40d8
140fc4762:     	movq	%rcx, 0x2d0(%rbp)
140fc4769:     	movq	$0x2, 0x2d8(%rbp)
140fc4774:     	leaq	0x200(%rbp), %rcx
140fc477b:     	movq	%rcx, 0x2e0(%rbp)
140fc4782:     	movq	$0x2, 0x2e8(%rbp)
140fc478d:     	movq	$0x0, 0x2f0(%rbp)
140fc4798:     	movq	$0x0, 0x280(%rbp)
140fc47a3:     	movq	%r8, 0x288(%rbp)
140fc47aa:     	movq	$0x13, 0x290(%rbp)
140fc47b5:     	movq	$0x0, 0x298(%rbp)
140fc47c0:     	movdqu	%xmm0, 0x2a0(%rbp)
140fc47c8:     	movl	$0x1, 0x2c8(%rbp)
140fc47d2:     	movl	%eax, 0x2cc(%rbp)
140fc47d8:     	leaq	0x138(%rbp), %rcx
140fc47df:     	callq	0x140d4a330 <.text+0xd49330>
140fc47e4:     	movq	0x160(%rbp), %rdx
140fc47eb:     	testq	%rdx, %rdx
140fc47ee:     	je	0x140fc4802 <.text+0xfc3802>
140fc47f0:     	movq	0x168(%rbp), %rcx
140fc47f7:     	movl	$0x1, %r8d
140fc47fd:     	callq	0x140613c20 <.text+0x612c20>
140fc4802:     	movq	0x230(%rbp), %rdx
140fc4809:     	testq	%rdx, %rdx
140fc480c:     	je	0x140fc4820 <.text+0xfc3820>
140fc480e:     	movq	0x238(%rbp), %rcx
140fc4815:     	movl	$0x1, %r8d
140fc481b:     	callq	0x140613c20 <.text+0x612c20>
140fc4820:     	movq	0x40(%rbp), %rax
140fc4824:     	movq	(%rax), %rcx
140fc4827:     	movq	0x228(%rbp), %rax
140fc482e:     	movups	(%rax), %xmm0
140fc4831:     	movaps	%xmm0, 0x160(%rbp)
140fc4838:     	movdqu	0x110(%rbp), %xmm0
140fc4840:     	movdqu	0x120(%rbp), %xmm1
140fc4848:     	movdqa	%xmm1, 0x290(%rbp)
140fc4850:     	movdqa	%xmm0, 0x280(%rbp)
140fc4858:     	movb	$0x0, 0x347(%rbp)
140fc485f:     	movb	$0x1, 0x346(%rbp)
140fc4866:     	leaq	0x160(%rbp), %rdx
140fc486d:     	movq	%rsi, %r8
140fc4870:     	callq	0x141e59860 <.text+0x1e58860>
140fc4875:     	testq	%rax, %rax
140fc4878:     	je	0x140fc4caf <.text+0xfc3caf>
140fc487e:     	movq	%rax, 0x230(%rbp)
140fc4885:     	movq	0x2667b1c(%rip), %rax   # 0x14362c3a8
140fc488c:     	movq	(%rax), %rax
140fc488f:     	testq	%rax, %rax
140fc4892:     	leaq	0x230(%rbp), %rbx
140fc4899:     	je	0x140fc497a <.text+0xfc397a>
140fc489f:     	movq	%rsi, %rdi
140fc48a2:     	movq	%rbx, 0x160(%rbp)
140fc48a9:     	leaq	-0x277c90(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
140fc48b0:     	movq	%rax, 0x168(%rbp)
140fc48b7:     	leaq	0x1bef8a2(%rip), %rcx   # 0x142bb4160
140fc48be:     	callq	0x14067dd80 <.text+0x67cd80>
140fc48c3:     	movdqu	(%rax), %xmm0
140fc48c7:     	movl	0x10(%rax), %eax
140fc48ca:     	movq	$0x1, 0x2b0(%rbp)
140fc48d5:     	leaq	0x1bef834(%rip), %rdx   # 0x142bb4110
140fc48dc:     	movq	%rdx, 0x2b8(%rbp)
140fc48e3:     	movq	$0x13, 0x2c0(%rbp)
140fc48ee:     	leaq	0x1bef85b(%rip), %rcx   # 0x142bb4150
140fc48f5:     	movq	%rcx, 0x2d0(%rbp)
140fc48fc:     	movq	$0x1, 0x2d8(%rbp)
140fc4907:     	leaq	0x160(%rbp), %rcx
140fc490e:     	movq	%rcx, 0x2e0(%rbp)
140fc4915:     	movq	$0x1, 0x2e8(%rbp)
140fc4920:     	movq	$0x0, 0x2f0(%rbp)
140fc492b:     	movq	$0x0, 0x280(%rbp)
140fc4936:     	movq	%rdx, 0x288(%rbp)
140fc493d:     	movq	$0x13, 0x290(%rbp)
140fc4948:     	movq	$0x0, 0x298(%rbp)
140fc4953:     	movdqu	%xmm0, 0x2a0(%rbp)
140fc495b:     	movl	$0x1, 0x2c8(%rbp)
140fc4965:     	movl	%eax, 0x2cc(%rbp)
140fc496b:     	leaq	0x138(%rbp), %rcx
140fc4972:     	movq	%rdi, %rdx
140fc4975:     	callq	0x140d4a330 <.text+0xd49330>
140fc497a:     	movb	$0x0, 0x347(%rbp)
140fc4981:     	movb	$0x1, 0x346(%rbp)
140fc4988:     	movq	%rbx, %rcx
140fc498b:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc4990:     	movq	0x80(%rbp), %rcx
140fc4997:     	movq	%rcx, 0x338(%rbp)
140fc499e:     	callq	0x140fa9980 <.text+0xfa8980>
140fc49a3:     	movl	$0xf0, %edx
140fc49a8:     	movl	$0x8, %r8d
140fc49ae:     	movq	0x338(%rbp), %rcx
140fc49b5:     	jmp	0x140fc4dda <.text+0xfc3dda>
140fc49ba:     	movl	$0x1, %r13d
140fc49c0:     	movq	%r13, %rcx
140fc49c3:     	movq	%r12, %rdx
140fc49c6:     	movq	%rbx, %r8
140fc49c9:     	callq	0x142923c00 <.text+0x2922c00>
140fc49ce:     	movq	%rbx, 0x230(%rbp)
140fc49d5:     	movq	%r13, 0x238(%rbp)
140fc49dc:     	movq	%rbx, 0x240(%rbp)
140fc49e3:     	movzbl	0x18(%r14), %eax
140fc49e8:     	movb	%al, 0x248(%rbp)
140fc49ee:     	movb	$0x1, 0x345(%rbp)
140fc49f5:     	leaq	0x230(%rbp), %rcx
140fc49fc:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4a01:     	movq	0x158(%rbp), %r13
140fc4a08:     	movb	$0x1, 0x345(%rbp)
140fc4a0f:     	movq	%rax, %rcx
140fc4a12:     	callq	0x140d61e00 <.text+0xd60e00>
140fc4a17:     	testq	%rax, %rax
140fc4a1a:     	je	0x140fc5778 <.text+0xfc4778>
140fc4a20:     	movq	%rdx, %r8
140fc4a23:     	movb	$0x1, 0x345(%rbp)
140fc4a2a:     	movq	%rdi, %rcx
140fc4a2d:     	movq	%rax, %rdx
140fc4a30:     	callq	0x140d58e70 <.text+0xd57e70>
140fc4a35:     	cmpl	$0x1, 0x280(%rbp)
140fc4a3c:     	je	0x140fc5763 <.text+0xfc4763>
140fc4a42:     	movq	0x288(%rbp), %rax
140fc4a49:     	movq	0x290(%rbp), %rcx
140fc4a50:     	movq	%rax, -0x28(%rbp)
140fc4a54:     	movq	%rcx, -0x20(%rbp)
140fc4a58:     	cmpq	$0xa, %rcx
140fc4a5c:     	jb	0x140fc4a81 <.text+0xfc3a81>
140fc4a5e:     	movq	(%rax), %rcx
140fc4a61:     	movabsq	$0x6574707572726f63, %rdx # imm = 0x6574707572726F63
140fc4a6b:     	xorq	%rdx, %rcx
140fc4a6e:     	movzwl	0x8(%rax), %eax
140fc4a72:     	xorq	$0x2d64, %rax           # imm = 0x2D64
140fc4a78:     	orq	%rcx, %rax
140fc4a7b:     	je	0x140fc4deb <.text+0xfc3deb>
140fc4a81:     	movb	$0x1, 0x345(%rbp)
140fc4a88:     	leaq	0x230(%rbp), %rcx
140fc4a8f:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4a94:     	movb	$0x1, 0x345(%rbp)
140fc4a9b:     	movq	%rax, %rcx
140fc4a9e:     	callq	0x140d61cd0 <.text+0xd60cd0>
140fc4aa3:     	movq	%rax, %rbx
140fc4aa6:     	testq	%rax, %rax
140fc4aa9:     	je	0x140fc57a3 <.text+0xfc47a3>
140fc4aaf:     	movq	%rdx, %r12
140fc4ab2:     	leaq	-0x28(%rbp), %rax
140fc4ab6:     	movq	%rax, 0x200(%rbp)
140fc4abd:     	leaq	-0xebac34(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140fc4ac4:     	movq	%rax, 0x208(%rbp)
140fc4acb:     	leaq	0x1bef75e(%rip), %rax   # 0x142bb4230
140fc4ad2:     	movq	%rax, 0x280(%rbp)
140fc4ad9:     	movq	$0x1, 0x288(%rbp)
140fc4ae4:     	leaq	0x200(%rbp), %rax
140fc4aeb:     	movq	%rax, 0x290(%rbp)
140fc4af2:     	movq	$0x1, 0x298(%rbp)
140fc4afd:     	movq	$0x0, 0x2a0(%rbp)
140fc4b08:     	movb	$0x1, 0x345(%rbp)
140fc4b0f:     	leaq	-0x40(%rbp), %rcx
140fc4b13:     	movq	%rdi, %rdx
140fc4b16:     	callq	0x140d79f90 <.text+0xd78f90>
140fc4b1b:     	movq	-0x40(%rbp), %rax
140fc4b1f:     	movq	%rax, 0x338(%rbp)
140fc4b26:     	movq	-0x38(%rbp), %r9
140fc4b2a:     	movq	-0x30(%rbp), %rax
140fc4b2e:     	movq	%rax, 0x20(%rsp)
140fc4b33:     	leaq	0x160(%rbp), %rcx
140fc4b3a:     	movq	%rbx, %rdx
140fc4b3d:     	movq	%r12, %r8
140fc4b40:     	movq	%r9, 0x330(%rbp)
140fc4b47:     	callq	0x140d62810 <.text+0xd61810>
140fc4b4c:     	movq	0x338(%rbp), %rdx
140fc4b53:     	testq	%rdx, %rdx
140fc4b56:     	je	0x140fc4b6a <.text+0xfc3b6a>
140fc4b58:     	movl	$0x1, %r8d
140fc4b5e:     	movq	0x330(%rbp), %rcx
140fc4b65:     	callq	0x140613c20 <.text+0x612c20>
140fc4b6a:     	leaq	0x230(%rbp), %rcx
140fc4b71:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4b76:     	movq	%rax, %rbx
140fc4b79:     	movq	%rdx, %r12
140fc4b7c:     	leaq	0x160(%rbp), %rcx
140fc4b83:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4b88:     	movq	%rdx, %r9
140fc4b8b:     	movq	%rbx, %rcx
140fc4b8e:     	movq	%r12, %rdx
140fc4b91:     	movq	%rax, %r8
140fc4b94:     	callq	0x140d69870 <.text+0xd68870>
140fc4b99:     	testq	%rax, %rax
140fc4b9c:     	je	0x140fc4c75 <.text+0xfc3c75>
140fc4ba2:     	movl	%eax, %ecx
140fc4ba4:     	andl	$0x3, %ecx
140fc4ba7:     	cmpl	$0x1, %ecx
140fc4baa:     	jne	0x140fc4c17 <.text+0xfc3c17>
140fc4bac:     	movq	%rax, %rcx
140fc4baf:     	decq	%rcx
140fc4bb2:     	movq	%rcx, 0x320(%rbp)
140fc4bb9:     	movq	-0x1(%rax), %rcx
140fc4bbd:     	movq	%rcx, 0x338(%rbp)
140fc4bc4:     	movq	0x7(%rax), %rax
140fc4bc8:     	movq	%rax, 0x330(%rbp)
140fc4bcf:     	movq	(%rax), %rax
140fc4bd2:     	testq	%rax, %rax
140fc4bd5:     	je	0x140fc4be0 <.text+0xfc3be0>
140fc4bd7:     	movq	0x338(%rbp), %rcx
140fc4bde:     	callq	*%rax
140fc4be0:     	movq	0x338(%rbp), %rcx
140fc4be7:     	movq	0x330(%rbp), %rax
140fc4bee:     	movq	0x8(%rax), %rdx
140fc4bf2:     	testq	%rdx, %rdx
140fc4bf5:     	je	0x140fc4c00 <.text+0xfc3c00>
140fc4bf7:     	movq	0x10(%rax), %r8
140fc4bfb:     	callq	0x140613c20 <.text+0x612c20>
140fc4c00:     	movl	$0x18, %edx
140fc4c05:     	movl	$0x8, %r8d
140fc4c0b:     	movq	0x320(%rbp), %rcx
140fc4c12:     	callq	0x140613c20 <.text+0x612c20>
140fc4c17:     	movq	0x230(%rbp), %rax
140fc4c1e:     	movq	%rax, 0x338(%rbp)
140fc4c25:     	movq	0x238(%rbp), %rax
140fc4c2c:     	movq	%rax, 0x320(%rbp)
140fc4c33:     	leaq	0x241(%rbp), %rax
140fc4c3a:     	movdqu	-0x1(%rax), %xmm0
140fc4c3f:     	movdqa	%xmm0, 0x70(%rbp)
140fc4c44:     	movq	0x160(%rbp), %rdx
140fc4c4b:     	testq	%rdx, %rdx
140fc4c4e:     	je	0x140fc4e18 <.text+0xfc3e18>
140fc4c54:     	movq	0x168(%rbp), %rcx
140fc4c5b:     	movl	$0x1, %r8d
140fc4c61:     	callq	0x140613c20 <.text+0x612c20>
140fc4c66:     	jmp	0x140fc4e18 <.text+0xfc3e18>
140fc4c6b:     	movb	$0x1, %al
140fc4c6d:     	movl	%eax, 0x338(%rbp)
140fc4c73:     	jmp	0x140fc4cb9 <.text+0xfc3cb9>
140fc4c75:     	movq	0x160(%rbp), %rax
140fc4c7c:     	movq	%rax, 0x338(%rbp)
140fc4c83:     	movq	0x168(%rbp), %rax
140fc4c8a:     	movq	%rax, 0x320(%rbp)
140fc4c91:     	leaq	0x171(%rbp), %rax
140fc4c98:     	movdqu	-0x1(%rax), %xmm0
140fc4c9d:     	movdqa	%xmm0, 0x70(%rbp)
140fc4ca2:     	movb	$0x1, %al
140fc4ca4:     	movl	%eax, 0x330(%rbp)
140fc4caa:     	jmp	0x140fc4e22 <.text+0xfc3e22>
140fc4caf:     	movl	$0x0, 0x338(%rbp)
140fc4cb9:     	movq	%rsi, %rdi
140fc4cbc:     	movq	0x48(%rbp), %rax
140fc4cc0:     	movb	$0x1, (%rax)
140fc4cc3:     	movaps	0x80(%rbp), %xmm0
140fc4cca:     	leaq	0x171(%rbp), %rcx
140fc4cd1:     	movups	%xmm0, 0x1f(%rcx)
140fc4cd5:     	movq	0x228(%rbp), %rax
140fc4cdc:     	movdqu	(%rax), %xmm0
140fc4ce0:     	movdqu	%xmm0, 0xf(%rcx)
140fc4ce5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fc4cef:     	movq	%rax, 0x298(%rbp)
140fc4cf6:     	movq	$0x0, 0x280(%rbp)
140fc4d01:     	movq	$0x8, 0x288(%rbp)
140fc4d0c:     	movq	$0x0, 0x290(%rbp)
140fc4d17:     	movb	$0x0, 0x318(%rbp)
140fc4d1e:     	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
140fc4d28:     	movq	%rax, 0x178(%rbp)
140fc4d2f:     	movq	0x280(%rbp), %rax
140fc4d36:     	movq	%rax, 0x160(%rbp)
140fc4d3d:     	movq	0x288(%rbp), %rax
140fc4d44:     	movq	%rax, 0x168(%rbp)
140fc4d4b:     	movq	$0x0, 0x170(%rbp)
140fc4d56:     	movb	$0x0, 0x1f8(%rbp)
140fc4d5d:     	movq	0x50(%rbp), %rcx
140fc4d61:     	leaq	0x160(%rbp), %rdx
140fc4d68:     	leaq	0x1bef409(%rip), %r8    # 0x142bb4178
140fc4d6f:     	callq	0x140fe0190 <.text+0xfdf190>
140fc4d74:     	movq	0x58(%rbp), %rdx
140fc4d78:     	movl	0x1c(%rdx), %eax
140fc4d7b:     	movq	0x10(%rdx), %rcx
140fc4d7f:     	movl	%eax, (%rcx)
140fc4d81:     	movq	(%rdx), %rax
140fc4d84:     	movb	$0x2, (%rax)
140fc4d87:     	movq	0x60(%rbp), %rdx
140fc4d8b:     	movl	0x1c(%rdx), %eax
140fc4d8e:     	movq	0x10(%rdx), %rcx
140fc4d92:     	movl	%eax, (%rcx)
140fc4d94:     	movq	(%rdx), %rcx
140fc4d97:     	movq	$0x2, 0x280(%rbp)
140fc4da2:     	movl	0x338(%rbp), %ebx
140fc4da8:     	movb	%bl, 0x347(%rbp)
140fc4dae:     	movb	$0x0, 0x346(%rbp)
140fc4db5:     	movq	%rdi, %rdx
140fc4db8:     	callq	0x14093fad0 <.text+0x93ead0>
140fc4dbd:     	testb	%bl, %bl
140fc4dbf:     	je	0x140fc4ddf <.text+0xfc3ddf>
140fc4dc1:     	movq	0x110(%rbp), %rdx
140fc4dc8:     	testq	%rdx, %rdx
140fc4dcb:     	je	0x140fc4ddf <.text+0xfc3ddf>
140fc4dcd:     	movq	0x118(%rbp), %rcx
140fc4dd4:     	movl	$0x1, %r8d
140fc4dda:     	callq	0x140613c20 <.text+0x612c20>
140fc4ddf:     	movq	0x158(%rbp), %r13
140fc4de6:     	jmp	0x140fc4f4b <.text+0xfc3f4b>
140fc4deb:     	movq	0x230(%rbp), %rax
140fc4df2:     	movq	%rax, 0x338(%rbp)
140fc4df9:     	movq	0x238(%rbp), %rax
140fc4e00:     	movq	%rax, 0x320(%rbp)
140fc4e07:     	leaq	0x241(%rbp), %rax
140fc4e0e:     	movdqu	-0x1(%rax), %xmm0
140fc4e13:     	movdqa	%xmm0, 0x70(%rbp)
140fc4e18:     	movl	$0x0, 0x330(%rbp)
140fc4e22:     	callq	0x140d635e0 <.text+0xd625e0>
140fc4e27:     	movl	$0x2, %r8d
140fc4e2d:     	movq	%rax, %rcx
140fc4e30:     	xorl	%r9d, %r9d
140fc4e33:     	callq	0x140d63a20 <.text+0xd62a20>
140fc4e38:     	movq	%rax, %rbx
140fc4e3b:     	movl	%edx, %r12d
140fc4e3e:     	leaq	0x110(%rbp), %rax
140fc4e45:     	movq	%rax, 0x160(%rbp)
140fc4e4c:     	leaq	-0x278233(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
140fc4e53:     	movq	%rax, 0x168(%rbp)
140fc4e5a:     	leaq	0x1bef3df(%rip), %rax   # 0x142bb4240
140fc4e61:     	movq	%rax, 0x280(%rbp)
140fc4e68:     	movq	$0x1, 0x288(%rbp)
140fc4e73:     	leaq	0x160(%rbp), %rax
140fc4e7a:     	movq	%rax, 0x290(%rbp)
140fc4e81:     	movq	$0x1, 0x298(%rbp)
140fc4e8c:     	movq	$0x0, 0x2a0(%rbp)
140fc4e97:     	leaq	-0x58(%rbp), %rcx
140fc4e9b:     	movq	%rdi, %rdx
140fc4e9e:     	callq	0x140d79f90 <.text+0xd78f90>
140fc4ea3:     	movq	0x150(%rbp), %rax
140fc4eaa:     	movq	(%rax), %rdi
140fc4ead:     	movq	(%rdi), %rdx
140fc4eb0:     	movq	%rdx, %rax
140fc4eb3:     	negq	%rax
140fc4eb6:     	jo	0x140fc4ee1 <.text+0xfc3ee1>
140fc4eb8:     	jae	0x140fc4ec9 <.text+0xfc3ec9>
140fc4eba:     	movq	0x8(%rdi), %rcx
140fc4ebe:     	movl	$0x1, %r8d
140fc4ec4:     	callq	0x140613c20 <.text+0x612c20>
140fc4ec9:     	movq	0x20(%rdi), %rdx
140fc4ecd:     	testq	%rdx, %rdx
140fc4ed0:     	je	0x140fc4ee1 <.text+0xfc3ee1>
140fc4ed2:     	movq	0x28(%rdi), %rcx
140fc4ed6:     	movl	$0x1, %r8d
140fc4edc:     	callq	0x140613c20 <.text+0x612c20>
140fc4ee1:     	movq	0x338(%rbp), %rax
140fc4ee8:     	movq	%rax, (%rdi)
140fc4eeb:     	movq	0x320(%rbp), %rax
140fc4ef2:     	movq	%rax, 0x8(%rdi)
140fc4ef6:     	movaps	0x70(%rbp), %xmm0
140fc4efa:     	movups	%xmm0, 0x10(%rdi)
140fc4efe:     	movdqu	-0x58(%rbp), %xmm0
140fc4f03:     	movdqu	%xmm0, 0x20(%rdi)
140fc4f08:     	movq	-0x48(%rbp), %rax
140fc4f0c:     	movq	%rax, 0x30(%rdi)
140fc4f10:     	movq	%rbx, 0x38(%rdi)
140fc4f14:     	movl	%r12d, 0x40(%rdi)
140fc4f18:     	cmpb	$0x0, 0x330(%rbp)
140fc4f1f:     	je	0x140fc4f3f <.text+0xfc3f3f>
140fc4f21:     	movq	0x230(%rbp), %rdx
140fc4f28:     	testq	%rdx, %rdx
140fc4f2b:     	je	0x140fc4f3f <.text+0xfc3f3f>
140fc4f2d:     	movq	0x238(%rbp), %rcx
140fc4f34:     	movl	$0x1, %r8d
140fc4f3a:     	callq	0x140613c20 <.text+0x612c20>
140fc4f3f:     	leaq	0x110(%rbp), %rcx
140fc4f46:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc4f4b:     	movq	-0x18(%rbp), %rdx
140fc4f4f:     	testq	%rdx, %rdx
140fc4f52:     	je	0x140fc4f63 <.text+0xfc3f63>
140fc4f54:     	movq	-0x10(%rbp), %rcx
140fc4f58:     	movl	$0x1, %r8d
140fc4f5e:     	callq	0x140613c20 <.text+0x612c20>
140fc4f63:     	movq	0x150(%rbp), %rax
140fc4f6a:     	movq	(%rax), %r12
140fc4f6d:     	xorl	%eax, %eax
140fc4f6f:     	cmpq	(%r12), %rax
140fc4f73:     	jo	0x140fc5542 <.text+0xfc4542>
140fc4f79:     	movq	%r12, %rcx
140fc4f7c:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4f81:     	movq	%rdx, %r8
140fc4f84:     	leaq	0x230(%rbp), %rcx
140fc4f8b:     	movq	%rax, %rdx
140fc4f8e:     	callq	0x140d62c00 <.text+0xd61c00>
140fc4f93:     	movq	%r14, %rcx
140fc4f96:     	callq	0x140d76ed0 <.text+0xd75ed0>
140fc4f9b:     	movq	%rdx, %r8
140fc4f9e:     	leaq	0xd0(%rbp), %rcx
140fc4fa5:     	movq	%rax, %rdx
140fc4fa8:     	callq	0x140d62c00 <.text+0xd61c00>
140fc4fad:     	movq	0x238(%rbp), %r14
140fc4fb4:     	movq	0xd8(%rbp), %rdi
140fc4fbb:     	cmpq	%rdi, %r14
140fc4fbe:     	jne	0x140fc5009 <.text+0xfc4009>
140fc4fc0:     	movzbl	0x240(%rbp), %ebx
140fc4fc7:     	movzbl	0x268(%rbp), %eax
140fc4fce:     	cmpb	0x108(%rbp), %al
140fc4fd4:     	jne	0x140fc5010 <.text+0xfc4010>
140fc4fd6:     	cmpb	$0x2, 0x269(%rbp)
140fc4fdd:     	jne	0x140fc5010 <.text+0xfc4010>
140fc4fdf:     	cmpb	$0x2, 0x109(%rbp)
140fc4fe6:     	jne	0x140fc5010 <.text+0xfc4010>
140fc4fe8:     	cmpb	$0x6, %bl
140fc4feb:     	jne	0x140fc5424 <.text+0xfc4424>
140fc4ff1:     	cmpb	$0x2, 0xe0(%rbp)
140fc4ff8:     	ja	0x140fc5434 <.text+0xfc4434>
140fc4ffe:     	movb	$0x6, %cl
140fc5000:     	movq	0x230(%rbp), %rax
140fc5007:     	jmp	0x140fc504b <.text+0xfc404b>
140fc5009:     	movzbl	0x240(%rbp), %ebx
140fc5010:     	movq	0x230(%rbp), %rax
140fc5017:     	movb	$0x6, %cl
140fc5019:     	cmpb	$0x6, %bl
140fc501c:     	je	0x140fc504b <.text+0xfc404b>
140fc501e:     	leaq	0x241(%rbp), %rdx
140fc5025:     	movq	0x1f(%rdx), %rcx
140fc5029:     	movq	%rcx, 0x12f(%rbp)
140fc5030:     	movdqu	(%rdx), %xmm0
140fc5034:     	movdqu	0x10(%rdx), %xmm1
140fc5039:     	movdqa	%xmm1, 0x120(%rbp)
140fc5041:     	movdqa	%xmm0, 0x110(%rbp)
140fc5049:     	movl	%ebx, %ecx
140fc504b:     	movzbl	0x26a(%rbp), %edx
140fc5052:     	movzwl	0x268(%rbp), %r8d
140fc505a:     	movq	%rax, 0x160(%rbp)
140fc5061:     	movq	%r14, 0x168(%rbp)
140fc5068:     	movb	%cl, 0x170(%rbp)
140fc506e:     	movq	0x12f(%rbp), %rax
140fc5075:     	leaq	0x171(%rbp), %rcx
140fc507c:     	movq	%rax, 0x1f(%rcx)
140fc5080:     	movaps	0x110(%rbp), %xmm0
140fc5087:     	movaps	0x120(%rbp), %xmm1
140fc508e:     	movups	%xmm1, 0x10(%rcx)
140fc5092:     	movups	%xmm0, (%rcx)
140fc5095:     	movw	%r8w, 0x198(%rbp)
140fc509d:     	movb	%dl, 0x19a(%rbp)
140fc50a3:     	movzbl	0xe0(%rbp), %eax
140fc50aa:     	cmpb	$0x6, %al
140fc50ac:     	je	0x140fc50d5 <.text+0xfc40d5>
140fc50ae:     	leaq	0xe1(%rbp), %rdx
140fc50b5:     	movq	0x1f(%rdx), %rcx
140fc50b9:     	movq	%rcx, 0x21f(%rbp)
140fc50c0:     	movups	(%rdx), %xmm0
140fc50c3:     	movups	0x10(%rdx), %xmm1
140fc50c7:     	movaps	%xmm1, 0x210(%rbp)
140fc50ce:     	movaps	%xmm0, 0x200(%rbp)
140fc50d5:     	movq	0xd0(%rbp), %rcx
140fc50dc:     	movzbl	0x10a(%rbp), %edx
140fc50e3:     	movzwl	0x108(%rbp), %r8d
140fc50eb:     	movq	%rcx, 0x280(%rbp)
140fc50f2:     	movq	%rdi, 0x288(%rbp)
140fc50f9:     	movb	%al, 0x290(%rbp)
140fc50ff:     	movq	0x21f(%rbp), %rax
140fc5106:     	leaq	0x288(%rbp), %rcx
140fc510d:     	movq	%rax, 0x28(%rcx)
140fc5111:     	movdqa	0x200(%rbp), %xmm0
140fc5119:     	movdqa	0x210(%rbp), %xmm1
140fc5121:     	movdqu	%xmm1, 0x19(%rcx)
140fc5126:     	movdqu	%xmm0, 0x9(%rcx)
140fc512b:     	movw	%r8w, 0x2b8(%rbp)
140fc5133:     	movb	%dl, 0x2ba(%rbp)
140fc5139:     	leaq	0x160(%rbp), %rcx
140fc5140:     	movq	%rsi, %rdx
140fc5143:     	callq	0x140fb3560 <.text+0xfb2560>
140fc5148:     	testb	%al, %al
140fc514a:     	je	0x140fc5542 <.text+0xfc4542>
140fc5150:     	callq	0x140d635e0 <.text+0xd625e0>
140fc5155:     	xorl	%ecx, %ecx
140fc5157:     	cmpq	%rax, 0x38(%r12)
140fc515c:     	setne	%cl
140fc515f:     	movl	$0xff, %r8d
140fc5165:     	cmovbl	%r8d, %ecx
140fc5169:     	xorl	%eax, %eax
140fc516b:     	cmpl	%edx, 0x40(%r12)
140fc5170:     	setne	%al
140fc5173:     	cmovbl	%r8d, %eax
140fc5177:     	testb	%cl, %cl
140fc5179:     	cmovnel	%ecx, %eax
140fc517c:     	cmpb	$0x1, %al
140fc517e:     	jne	0x140fc52f9 <.text+0xfc42f9>
140fc5184:     	movq	0xb0(%rbp), %r14
140fc518b:     	leaq	0x1bef0be(%rip), %rax   # 0x142bb4250
140fc5192:     	movq	%rax, 0x280(%rbp)
140fc5199:     	movq	$0xf, 0x288(%rbp)
140fc51a4:     	movq	%rsi, %rcx
140fc51a7:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fc51ac:     	movq	%rax, %rbx
140fc51af:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
140fc51b9:     	xorq	%rax, %rbx
140fc51bc:     	movq	%r14, %rcx
140fc51bf:     	movq	%rbx, %rdx
140fc51c2:     	callq	0x140683ec0 <.text+0x682ec0>
140fc51c7:     	movb	$0x0, 0x5d(%rax)
140fc51cb:     	movb	$0x1, 0xb7(%rax)
140fc51d2:     	testb	$0x1, %dl
140fc51d5:     	jne	0x140fc5221 <.text+0xfc4221>
140fc51d7:     	movq	0x10(%r14), %rdi
140fc51db:     	cmpq	(%r14), %rdi
140fc51de:     	jne	0x140fc51ef <.text+0xfc41ef>
140fc51e0:     	movq	%r14, %rcx
140fc51e3:     	leaq	0x1be9dde(%rip), %rdx   # 0x142baefc8
140fc51ea:     	callq	0x1406319d0 <.text+0x6309d0>
140fc51ef:     	movq	0x8(%r14), %rax
140fc51f3:     	imulq	$0x38, %rdi, %rcx
140fc51f7:     	movq	$0x3, (%rax,%rcx)
140fc51ff:     	movq	0x68(%rbp), %rdx
140fc5203:     	movq	%rdx, 0x10(%rax,%rcx)
140fc5208:     	movups	%xmm9, 0x18(%rax,%rcx)
140fc520e:     	movsd	%xmm10, 0x28(%rax,%rcx)
140fc5215:     	movq	%rbx, 0x30(%rax,%rcx)
140fc521a:     	incq	%rdi
140fc521d:     	movq	%rdi, 0x10(%r14)
140fc5221:     	movq	%r14, 0xd0(%rbp)
140fc5228:     	movq	%rbx, 0xd8(%rbp)
140fc522f:     	movq	$0x0, 0xe0(%rbp)
140fc523a:     	cmpq	$0x0, 0x1b8(%r14)
140fc5242:     	je	0x140fc56f6 <.text+0xfc46f6>
140fc5248:     	leaq	0x1c0(%r14), %rcx
140fc524f:     	leaq	0xd8(%rbp), %rdx
140fc5256:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc525b:     	movq	0x1a0(%r14), %rcx
140fc5262:     	movq	0x1a8(%r14), %rdx
140fc5269:     	movq	%rdx, %r8
140fc526c:     	andq	%rax, %r8
140fc526f:     	shrq	$0x39, %rax
140fc5273:     	movd	%eax, %xmm0
140fc5277:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc527b:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc5280:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc5285:     	leaq	-0xc0(%rcx), %rax
140fc528c:     	movq	0xd8(%rbp), %r9
140fc5293:     	xorl	%r10d, %r10d
140fc5296:     	movdqu	(%rcx,%r8), %xmm1
140fc529c:     	movdqa	%xmm1, %xmm2
140fc52a0:     	pcmpeqb	%xmm0, %xmm2
140fc52a4:     	pmovmskb	%xmm2, %r11d
140fc52a9:     	testl	%r11d, %r11d
140fc52ac:     	je	0x140fc52d7 <.text+0xfc42d7>
140fc52ae:     	tzcntl	%r11d, %edi
140fc52b3:     	addq	%r8, %rdi
140fc52b6:     	andq	%rdx, %rdi
140fc52b9:     	negq	%rdi
140fc52bc:     	leaq	(%rdi,%rdi,2), %rdi
140fc52c0:     	shlq	$0x6, %rdi
140fc52c4:     	cmpq	(%rax,%rdi), %r9
140fc52c8:     	je	0x140fc5349 <.text+0xfc4349>
140fc52ca:     	leal	-0x1(%r11), %edi
140fc52ce:     	andw	%r11w, %di
140fc52d2:     	movl	%edi, %r11d
140fc52d5:     	jne	0x140fc52ae <.text+0xfc42ae>
140fc52d7:     	pcmpeqb	%xmm7, %xmm1
140fc52db:     	pmovmskb	%xmm1, %r11d
140fc52e0:     	testl	%r11d, %r11d
140fc52e3:     	jne	0x140fc56f6 <.text+0xfc46f6>
140fc52e9:     	addq	%r10, %r8
140fc52ec:     	addq	$0x10, %r8
140fc52f0:     	addq	$0x10, %r10
140fc52f4:     	andq	%rdx, %r8
140fc52f7:     	jmp	0x140fc5296 <.text+0xfc4296>
140fc52f9:     	movq	0x150(%rbp), %rax
140fc5300:     	movq	(%rax), %rdi
140fc5303:     	movq	(%rdi), %rdx
140fc5306:     	movq	%rdx, %rax
140fc5309:     	negq	%rax
140fc530c:     	jo	0x140fc5337 <.text+0xfc4337>
140fc530e:     	jae	0x140fc531f <.text+0xfc431f>
140fc5310:     	movq	0x8(%rdi), %rcx
140fc5314:     	movl	$0x1, %r8d
140fc531a:     	callq	0x140613c20 <.text+0x612c20>
140fc531f:     	movq	0x20(%rdi), %rdx
140fc5323:     	testq	%rdx, %rdx
140fc5326:     	je	0x140fc5337 <.text+0xfc4337>
140fc5328:     	movq	0x28(%rdi), %rcx
140fc532c:     	movl	$0x1, %r8d
140fc5332:     	callq	0x140613c20 <.text+0x612c20>
140fc5337:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fc5341:     	movq	%rax, (%rdi)
140fc5344:     	jmp	0x140fc5542 <.text+0xfc4542>
140fc5349:     	movl	$0x42c80000, -0x10(%rcx,%rdi) # imm = 0x42C80000
140fc5351:     	testb	$0x1, 0x32f(%rbp)
140fc5358:     	jne	0x140fc5479 <.text+0xfc4479>
140fc535e:     	movq	0xd0(%rbp), %rdi
140fc5365:     	cmpq	$0x0, 0x1b8(%rdi)
140fc536d:     	je	0x140fc5707 <.text+0xfc4707>
140fc5373:     	leaq	0x1c0(%rdi), %rcx
140fc537a:     	leaq	0xd8(%rbp), %rdx
140fc5381:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc5386:     	movq	0x1a0(%rdi), %rcx
140fc538d:     	movq	0x1a8(%rdi), %rdx
140fc5394:     	movq	%rdx, %r8
140fc5397:     	andq	%rax, %r8
140fc539a:     	shrq	$0x39, %rax
140fc539e:     	movd	%eax, %xmm0
140fc53a2:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc53a6:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc53ab:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc53b0:     	leaq	-0xc0(%rcx), %rax
140fc53b7:     	movq	0xd8(%rbp), %r9
140fc53be:     	xorl	%r10d, %r10d
140fc53c1:     	movdqu	(%rcx,%r8), %xmm1
140fc53c7:     	movdqa	%xmm1, %xmm2
140fc53cb:     	pcmpeqb	%xmm0, %xmm2
140fc53cf:     	pmovmskb	%xmm2, %r11d
140fc53d4:     	testl	%r11d, %r11d
140fc53d7:     	je	0x140fc5402 <.text+0xfc4402>
140fc53d9:     	tzcntl	%r11d, %edi
140fc53de:     	addq	%r8, %rdi
140fc53e1:     	andq	%rdx, %rdi
140fc53e4:     	negq	%rdi
140fc53e7:     	leaq	(%rdi,%rdi,2), %rdi
140fc53eb:     	shlq	$0x6, %rdi
140fc53ef:     	cmpq	(%rax,%rdi), %r9
140fc53f3:     	je	0x140fc5472 <.text+0xfc4472>
140fc53f5:     	leal	-0x1(%r11), %edi
140fc53f9:     	andw	%r11w, %di
140fc53fd:     	movl	%edi, %r11d
140fc5400:     	jne	0x140fc53d9 <.text+0xfc43d9>
140fc5402:     	pcmpeqb	%xmm7, %xmm1
140fc5406:     	pmovmskb	%xmm1, %r11d
140fc540b:     	testl	%r11d, %r11d
140fc540e:     	jne	0x140fc5707 <.text+0xfc4707>
140fc5414:     	addq	%r10, %r8
140fc5417:     	addq	$0x10, %r8
140fc541b:     	addq	$0x10, %r10
140fc541f:     	andq	%rdx, %r8
140fc5422:     	jmp	0x140fc53c1 <.text+0xfc43c1>
140fc5424:     	movzbl	0xe0(%rbp), %eax
140fc542b:     	cmpb	$0x6, %al
140fc542d:     	jne	0x140fc5457 <.text+0xfc4457>
140fc542f:     	cmpb	$0x3, %bl
140fc5432:     	jb	0x140fc5466 <.text+0xfc4466>
140fc5434:     	movq	0x230(%rbp), %rcx
140fc543b:     	movq	0xd0(%rbp), %rdx
140fc5442:     	movq	%r14, %r8
140fc5445:     	callq	0x142923bf0 <.text+0x2922bf0>
140fc544a:     	testl	%eax, %eax
140fc544c:     	jne	0x140fc5010 <.text+0xfc4010>
140fc5452:     	jmp	0x140fc5150 <.text+0xfc4150>
140fc5457:     	cmpb	$0x3, %bl
140fc545a:     	setb	%cl
140fc545d:     	cmpb	$0x3, %al
140fc545f:     	setb	%al
140fc5462:     	xorb	%cl, %al
140fc5464:     	je	0x140fc5434 <.text+0xfc4434>
140fc5466:     	movq	0x230(%rbp), %rax
140fc546d:     	jmp	0x140fc501e <.text+0xfc401e>
140fc5472:     	testb	$0x1, -0x2(%rcx,%rdi)
140fc5477:     	je	0x140fc54c1 <.text+0xfc44c1>
140fc5479:     	movq	0x38(%r12), %rdi
140fc547e:     	movl	0x40(%r12), %ebx
140fc5483:     	callq	0x140d635e0 <.text+0xd625e0>
140fc5488:     	movq	%rax, %rcx
140fc548b:     	xorl	%r8d, %r8d
140fc548e:     	movl	$0xbebc200, %r9d        # imm = 0xBEBC200
140fc5494:     	callq	0x140d63a20 <.text+0xd62a20>
140fc5499:     	xorl	%ecx, %ecx
140fc549b:     	cmpl	%ebx, %edx
140fc549d:     	setb	%cl
140fc54a0:     	xorl	%r8d, %r8d
140fc54a3:     	cmpq	%rdi, %rax
140fc54a6:     	setb	%r8b
140fc54aa:     	cmovnel	%r8d, %ecx
140fc54ae:     	testb	%cl, %cl
140fc54b0:     	cmovnel	%ebx, %edx
140fc54b3:     	cmovneq	%rdi, %rax
140fc54b7:     	movq	%rax, 0x38(%r12)
140fc54bc:     	movl	%edx, 0x40(%r12)
140fc54c1:     	movq	0xe0(%rbp), %rax
140fc54c8:     	movq	%rax, 0x170(%rbp)
140fc54cf:     	movups	0xd0(%rbp), %xmm0
140fc54d6:     	movaps	%xmm0, 0x160(%rbp)
140fc54dd:     	movdqu	0x28(%r12), %xmm0
140fc54e4:     	movdqu	%xmm0, 0x230(%rbp)
140fc54ec:     	movq	$0x0, 0x20(%rsp)
140fc54f5:     	movl	$0x16, %r9d
140fc54fb:     	movq	%rsi, %rcx
140fc54fe:     	leaq	0x160(%rbp), %rdx
140fc5505:     	leaq	0x1fc2ec4(%rip), %r8    # 0x142f883d0
140fc550c:     	callq	0x140f9e970 <.text+0xf9d970>
140fc5511:     	movl	$0x6, %r8d
140fc5517:     	movq	%rsi, %rcx
140fc551a:     	leaq	0x1fc2e51(%rip), %rdx   # 0x142f88372
140fc5521:     	callq	0x140fbc5b0 <.text+0xfbb5b0>
140fc5526:     	movl	$0x5, %r8d
140fc552c:     	movq	%rsi, %rcx
140fc552f:     	leaq	0x1fc2eb0(%rip), %rdx   # 0x142f883e6
140fc5536:     	leaq	0x230(%rbp), %r9
140fc553d:     	callq	0x140fba940 <.text+0xfb9940>
140fc5542:     	movq	0x278(%rbp), %rax
140fc5549:     	movq	%rax, 0xd0(%rbp)
140fc5550:     	movq	0x270(%rbp), %rax
140fc5557:     	movq	%rax, 0xd8(%rbp)
140fc555e:     	leaq	0xd0(%rbp), %rcx
140fc5565:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fc556a:     	movq	%rax, 0x280(%rbp)
140fc5571:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fc557b:     	movq	%rax, 0x288(%rbp)
140fc5582:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fc558c:     	movq	%rax, 0x290(%rbp)
140fc5593:     	movq	%rsi, %rcx
140fc5596:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fc559b:     	movq	0xb8(%rbp), %rcx
140fc55a2:     	movq	%rcx, 0x280(%rbp)
140fc55a9:     	movq	%rax, 0x288(%rbp)
140fc55b0:     	movq	%rsi, %rcx
140fc55b3:     	callq	0x1405bd910 <.text+0x5bc910>
140fc55b8:     	movq	%rsi, %rcx
140fc55bb:     	leaq	0xb0(%rbp), %rdx
140fc55c2:     	movq	%rax, %r8
140fc55c5:     	callq	0x1406861b0 <.text+0x6851b0>
140fc55ca:     	movq	%rsi, %rcx
140fc55cd:     	xorl	%edx, %edx
140fc55cf:     	callq	0x140f9d930 <.text+0xf9c930>
140fc55d4:     	movq	%rax, %rcx
140fc55d7:     	movq	0x278(%rbp), %rdx
140fc55de:     	movq	0x270(%rbp), %r8
140fc55e5:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fc55ea:     	movq	0x290(%rbp), %rax
140fc55f1:     	movq	%rax, 0x170(%rbp)
140fc55f8:     	movdqu	0x280(%rbp), %xmm0
140fc5600:     	movdqa	%xmm0, 0x160(%rbp)
140fc5608:     	leaq	0x160(%rbp), %rcx
140fc560f:     	movb	$0x1, %dl
140fc5611:     	callq	0x140f9d930 <.text+0xf9c930>
140fc5616:     	movq	%rax, %rcx
140fc5619:     	movdqa	%xmm6, %xmm1
140fc561d:     	callq	0x140fac150 <.text+0xfab150>
140fc5622:     	movq	0x20(%rbp), %r12
140fc5626:     	movq	%r12, %r14
140fc5629:     	movq	0xa8(%rbp), %rdx
140fc5630:     	testq	%rdx, %rdx
140fc5633:     	leaq	0x98(%rbp), %rcx
140fc563a:     	leaq	0x1bee9cf(%rip), %rbx   # 0x142bb4010
140fc5641:     	movq	%rsi, %rdi
140fc5644:     	movq	0x28(%rbp), %r15
140fc5648:     	je	0x140fc3a07 <.text+0xfc2a07>
140fc564e:     	movl	$0x1, %r8d
140fc5654:     	movq	%rcx, %rdi
140fc5657:     	movq	0x148(%rbp), %rcx
140fc565e:     	callq	0x140613c20 <.text+0x612c20>
140fc5663:     	movq	%rdi, %rcx
140fc5666:     	movq	%rsi, %rdi
140fc5669:     	movq	%r12, %r14
140fc566c:     	jmp	0x140fc3a07 <.text+0xfc2a07>
140fc5671:     	leaq	0x1a52918(%rip), %rcx   # 0x142a17f90
140fc5678:     	callq	0x1428d9310 <.text+0x28d8310>
140fc567d:     	leaq	0x1a528dc(%rip), %rcx   # 0x142a17f60
140fc5684:     	callq	0x1428d9310 <.text+0x28d8310>
140fc5689:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc568e:     	leaq	0x1a5258b(%rip), %rcx   # 0x142a17c20
140fc5695:     	callq	0x1428d9310 <.text+0x28d8310>
140fc569a:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc569f:     	leaq	0x1a51b62(%rip), %rcx   # 0x142a17208
140fc56a6:     	leaq	0x1a5292b(%rip), %r8    # 0x142a17fd8
140fc56ad:     	movl	$0x16, %edx
140fc56b2:     	callq	0x1428d9330 <.text+0x28d8330>
140fc56b7:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc56bc:     	movaps	0x350(%rbp), %xmm6
140fc56c3:     	movaps	0x360(%rbp), %xmm7
140fc56ca:     	movaps	0x370(%rbp), %xmm8
140fc56d2:     	movaps	0x380(%rbp), %xmm9
140fc56da:     	movaps	0x390(%rbp), %xmm10
140fc56e2:     	addq	$0x428, %rsp            # imm = 0x428
140fc56e9:     	popq	%rbx
140fc56ea:     	popq	%rdi
140fc56eb:     	popq	%rsi
140fc56ec:     	popq	%r12
140fc56ee:     	popq	%r13
140fc56f0:     	popq	%r14
140fc56f2:     	popq	%r15
140fc56f4:     	popq	%rbp
140fc56f5:     	retq
140fc56f6:     	leaq	0x1a528f3(%rip), %rcx   # 0x142a17ff0
140fc56fd:     	callq	0x1428d9310 <.text+0x28d8310>
140fc5702:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc5707:     	leaq	0x1a52512(%rip), %rcx   # 0x142a17c20
140fc570e:     	callq	0x1428d9310 <.text+0x28d8310>
140fc5713:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc5718:     	leaq	0x288(%rbp), %rcx
140fc571f:     	movq	0x10(%rcx), %rax
140fc5723:     	movq	%rax, 0x170(%rbp)
140fc572a:     	movdqu	(%rcx), %xmm0
140fc572e:     	movdqa	%xmm0, 0x160(%rbp)
140fc5736:     	leaq	0x1bee90b(%rip), %rax   # 0x142bb4048
140fc573d:     	movq	%rax, 0x20(%rsp)
140fc5742:     	leaq	0x1bedff7(%rip), %rcx   # 0x142bb3740
140fc5749:     	leaq	0x1bedfd0(%rip), %r9    # 0x142bb3720
140fc5750:     	leaq	0x160(%rbp), %r8
140fc5757:     	movl	$0x2b, %edx
140fc575c:     	callq	0x1428d9760 <.text+0x28d8760>
140fc5761:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc5763:     	movb	$0x1, 0x345(%rbp)
140fc576a:     	leaq	0x1beea7f(%rip), %rcx   # 0x142bb41f0
140fc5771:     	callq	0x1428d9310 <.text+0x28d8310>
140fc5776:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc5778:     	movb	$0x1, 0x345(%rbp)
140fc577f:     	leaq	0x1beea52(%rip), %rcx   # 0x142bb41d8
140fc5786:     	callq	0x1428d9310 <.text+0x28d8310>
140fc578b:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc578d:     	xorl	%edi, %edi
140fc578f:     	leaq	0x1bef92a(%rip), %r8    # 0x142bb50c0
140fc5796:     	movq	%rdi, %rcx
140fc5799:     	movq	%rbx, %rdx
140fc579c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140fc57a1:     	jmp	0x140fc57b6 <.text+0xfc47b6>
140fc57a3:     	movb	$0x1, 0x345(%rbp)
140fc57aa:     	leaq	0x1beea67(%rip), %rcx   # 0x142bb4218
140fc57b1:     	callq	0x1428d9310 <.text+0x28d8310>
140fc57b6:     	ud2
140fc57b8:     	nopl	(%rax,%rax)
140fc57c0:     	movq	%rdx, 0x10(%rsp)
140fc57c5:     	pushq	%rbp
140fc57c6:     	pushq	%r15
140fc57c8:     	pushq	%r14
140fc57ca:     	pushq	%r13
140fc57cc:     	pushq	%r12
140fc57ce:     	pushq	%rsi
140fc57cf:     	pushq	%rdi
140fc57d0:     	pushq	%rbx
140fc57d1:     	subq	$0x78, %rsp
140fc57d5:     	leaq	0x80(%rdx), %rbp
140fc57dc:     	movaps	%xmm10, 0x20(%rsp)
140fc57e2:     	movaps	%xmm9, 0x30(%rsp)
140fc57e8:     	movaps	%xmm8, 0x40(%rsp)
140fc57ee:     	movdqa	%xmm7, 0x50(%rsp)
140fc57f4:     	movdqa	%xmm6, 0x60(%rsp)
140fc57fa:     	movq	0x338(%rbp), %rcx
140fc5801:     	movq	0x330(%rbp), %rax
140fc5808:     	movq	0x8(%rax), %rdx
140fc580c:     	testq	%rdx, %rdx
140fc580f:     	je	0x140fc5821 <.text+0xfc4821>
140fc5811:     	movq	0x330(%rbp), %rax
140fc5818:     	movq	0x10(%rax), %r8
140fc581c:     	callq	0x140613c20 <.text+0x612c20>
140fc5821:     	movl	$0x18, %edx
140fc5826:     	movl	$0x8, %r8d
140fc582c:     	movq	0x320(%rbp), %rcx
140fc5833:     	callq	0x140613c20 <.text+0x612c20>
140fc5838:     	movdqa	0x60(%rsp), %xmm6
140fc583e:     	movdqa	0x50(%rsp), %xmm7
140fc5844:     	movaps	0x40(%rsp), %xmm8
140fc584a:     	movaps	0x30(%rsp), %xmm9
140fc5850:     	movaps	0x20(%rsp), %xmm10
140fc5856:     	addq	$0x78, %rsp
140fc585a:     	popq	%rbx
140fc585b:     	popq	%rdi
140fc585c:     	popq	%rsi
140fc585d:     	popq	%r12
140fc585f:     	popq	%r13
140fc5861:     	popq	%r14
140fc5863:     	popq	%r15
140fc5865:     	popq	%rbp
140fc5866:     	retq
140fc5867:     	nopw	(%rax,%rax)
140fc5870:     	movq	%rdx, 0x10(%rsp)
140fc5875:     	pushq	%rbp
140fc5876:     	pushq	%r15
140fc5878:     	pushq	%r14
140fc587a:     	pushq	%r13
140fc587c:     	pushq	%r12
140fc587e:     	pushq	%rsi
140fc587f:     	pushq	%rdi
140fc5880:     	pushq	%rbx
140fc5881:     	subq	$0x78, %rsp
140fc5885:     	leaq	0x80(%rdx), %rbp
140fc588c:     	movaps	%xmm10, 0x20(%rsp)
140fc5892:     	movaps	%xmm9, 0x30(%rsp)
140fc5898:     	movaps	%xmm8, 0x40(%rsp)
140fc589e:     	movdqa	%xmm7, 0x50(%rsp)
140fc58a4:     	movdqa	%xmm6, 0x60(%rsp)
140fc58aa:     	movl	$0xf0, %edx
140fc58af:     	movl	$0x8, %r8d
140fc58b5:     	movq	0x338(%rbp), %rcx
140fc58bc:     	callq	0x140613c20 <.text+0x612c20>
140fc58c1:     	movdqa	0x60(%rsp), %xmm6
140fc58c7:     	movdqa	0x50(%rsp), %xmm7
140fc58cd:     	movaps	0x40(%rsp), %xmm8
140fc58d3:     	movaps	0x30(%rsp), %xmm9
140fc58d9:     	movaps	0x20(%rsp), %xmm10
140fc58df:     	addq	$0x78, %rsp
140fc58e3:     	popq	%rbx
140fc58e4:     	popq	%rdi
140fc58e5:     	popq	%rsi
140fc58e6:     	popq	%r12
140fc58e8:     	popq	%r13
140fc58ea:     	popq	%r14
140fc58ec:     	popq	%r15
140fc58ee:     	popq	%rbp
140fc58ef:     	retq
140fc58f0:     	movq	%rdx, 0x10(%rsp)
140fc58f5:     	pushq	%rbp
140fc58f6:     	pushq	%r15
140fc58f8:     	pushq	%r14
140fc58fa:     	pushq	%r13
140fc58fc:     	pushq	%r12
140fc58fe:     	pushq	%rsi
140fc58ff:     	pushq	%rdi
140fc5900:     	pushq	%rbx
140fc5901:     	subq	$0x78, %rsp
140fc5905:     	leaq	0x80(%rdx), %rbp
140fc590c:     	movaps	%xmm10, 0x20(%rsp)
140fc5912:     	movaps	%xmm9, 0x30(%rsp)
140fc5918:     	movaps	%xmm8, 0x40(%rsp)
140fc591e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5924:     	movdqa	%xmm6, 0x60(%rsp)
140fc592a:     	leaq	0x230(%rbp), %rcx
140fc5931:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc5936:     	movb	$0x0, 0x347(%rbp)
140fc593d:     	movb	$0x1, 0x346(%rbp)
140fc5944:     	movdqa	0x60(%rsp), %xmm6
140fc594a:     	movdqa	0x50(%rsp), %xmm7
140fc5950:     	movaps	0x40(%rsp), %xmm8
140fc5956:     	movaps	0x30(%rsp), %xmm9
140fc595c:     	movaps	0x20(%rsp), %xmm10
140fc5962:     	addq	$0x78, %rsp
140fc5966:     	popq	%rbx
140fc5967:     	popq	%rdi
140fc5968:     	popq	%rsi
140fc5969:     	popq	%r12
140fc596b:     	popq	%r13
140fc596d:     	popq	%r14
140fc596f:     	popq	%r15
140fc5971:     	popq	%rbp
140fc5972:     	retq
140fc5973:     	nopw	%cs:(%rax,%rax)
140fc5980:     	movq	%rdx, 0x10(%rsp)
140fc5985:     	pushq	%rbp
140fc5986:     	pushq	%r15
140fc5988:     	pushq	%r14
140fc598a:     	pushq	%r13
140fc598c:     	pushq	%r12
140fc598e:     	pushq	%rsi
140fc598f:     	pushq	%rdi
140fc5990:     	pushq	%rbx
140fc5991:     	subq	$0x78, %rsp
140fc5995:     	leaq	0x80(%rdx), %rbp
140fc599c:     	movaps	%xmm10, 0x20(%rsp)
140fc59a2:     	movaps	%xmm9, 0x30(%rsp)
140fc59a8:     	movaps	%xmm8, 0x40(%rsp)
140fc59ae:     	movdqa	%xmm7, 0x50(%rsp)
140fc59b4:     	movdqa	%xmm6, 0x60(%rsp)
140fc59ba:     	leaq	0x298(%rbp), %rcx
140fc59c1:     	callq	0x140fb63f0 <.text+0xfb53f0>
140fc59c6:     	movl	0x338(%rbp), %eax
140fc59cc:     	movb	%al, 0x347(%rbp)
140fc59d2:     	movb	$0x0, 0x346(%rbp)
140fc59d9:     	movdqa	0x60(%rsp), %xmm6
140fc59df:     	movdqa	0x50(%rsp), %xmm7
140fc59e5:     	movaps	0x40(%rsp), %xmm8
140fc59eb:     	movaps	0x30(%rsp), %xmm9
140fc59f1:     	movaps	0x20(%rsp), %xmm10
140fc59f7:     	addq	$0x78, %rsp
140fc59fb:     	popq	%rbx
140fc59fc:     	popq	%rdi
140fc59fd:     	popq	%rsi
140fc59fe:     	popq	%r12
140fc5a00:     	popq	%r13
140fc5a02:     	popq	%r14
140fc5a04:     	popq	%r15
140fc5a06:     	popq	%rbp
140fc5a07:     	retq
140fc5a08:     	nopl	(%rax,%rax)
140fc5a10:     	movq	%rdx, 0x10(%rsp)
140fc5a15:     	pushq	%rbp
140fc5a16:     	pushq	%r15
140fc5a18:     	pushq	%r14
140fc5a1a:     	pushq	%r13
140fc5a1c:     	pushq	%r12
140fc5a1e:     	pushq	%rsi
140fc5a1f:     	pushq	%rdi
140fc5a20:     	pushq	%rbx
140fc5a21:     	subq	$0x78, %rsp
140fc5a25:     	leaq	0x80(%rdx), %rbp
140fc5a2c:     	movaps	%xmm10, 0x20(%rsp)
140fc5a32:     	movaps	%xmm9, 0x30(%rsp)
140fc5a38:     	movaps	%xmm8, 0x40(%rsp)
140fc5a3e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5a44:     	movdqa	%xmm6, 0x60(%rsp)
140fc5a4a:     	movq	0x160(%rbp), %rdx
140fc5a51:     	testq	%rdx, %rdx
140fc5a54:     	je	0x140fc5a68 <.text+0xfc4a68>
140fc5a56:     	movq	0x168(%rbp), %rcx
140fc5a5d:     	movl	$0x1, %r8d
140fc5a63:     	callq	0x140613c20 <.text+0x612c20>
140fc5a68:     	movdqa	0x60(%rsp), %xmm6
140fc5a6e:     	movdqa	0x50(%rsp), %xmm7
140fc5a74:     	movaps	0x40(%rsp), %xmm8
140fc5a7a:     	movaps	0x30(%rsp), %xmm9
140fc5a80:     	movaps	0x20(%rsp), %xmm10
140fc5a86:     	addq	$0x78, %rsp
140fc5a8a:     	popq	%rbx
140fc5a8b:     	popq	%rdi
140fc5a8c:     	popq	%rsi
140fc5a8d:     	popq	%r12
140fc5a8f:     	popq	%r13
140fc5a91:     	popq	%r14
140fc5a93:     	popq	%r15
140fc5a95:     	popq	%rbp
140fc5a96:     	retq
140fc5a97:     	nopw	(%rax,%rax)
140fc5aa0:     	movq	%rdx, 0x10(%rsp)
140fc5aa5:     	pushq	%rbp
140fc5aa6:     	pushq	%r15
140fc5aa8:     	pushq	%r14
140fc5aaa:     	pushq	%r13
140fc5aac:     	pushq	%r12
140fc5aae:     	pushq	%rsi
140fc5aaf:     	pushq	%rdi
140fc5ab0:     	pushq	%rbx
140fc5ab1:     	subq	$0x78, %rsp
140fc5ab5:     	leaq	0x80(%rdx), %rbp
140fc5abc:     	movaps	%xmm10, 0x20(%rsp)
140fc5ac2:     	movaps	%xmm9, 0x30(%rsp)
140fc5ac8:     	movaps	%xmm8, 0x40(%rsp)
140fc5ace:     	movdqa	%xmm7, 0x50(%rsp)
140fc5ad4:     	movdqa	%xmm6, 0x60(%rsp)
140fc5ada:     	cmpq	$0x0, 0x338(%rbp)
140fc5ae2:     	je	0x140fc5afd <.text+0xfc4afd>
140fc5ae4:     	movl	$0x1, %r8d
140fc5aea:     	movq	0x330(%rbp), %rcx
140fc5af1:     	movq	0x338(%rbp), %rdx
140fc5af8:     	callq	0x140613c20 <.text+0x612c20>
140fc5afd:     	movb	$0x1, 0x345(%rbp)
140fc5b04:     	movdqa	0x60(%rsp), %xmm6
140fc5b0a:     	movdqa	0x50(%rsp), %xmm7
140fc5b10:     	movaps	0x40(%rsp), %xmm8
140fc5b16:     	movaps	0x30(%rsp), %xmm9
140fc5b1c:     	movaps	0x20(%rsp), %xmm10
140fc5b22:     	addq	$0x78, %rsp
140fc5b26:     	popq	%rbx
140fc5b27:     	popq	%rdi
140fc5b28:     	popq	%rsi
140fc5b29:     	popq	%r12
140fc5b2b:     	popq	%r13
140fc5b2d:     	popq	%r14
140fc5b2f:     	popq	%r15
140fc5b31:     	popq	%rbp
140fc5b32:     	retq
140fc5b33:     	nopw	%cs:(%rax,%rax)
140fc5b40:     	movq	%rdx, 0x10(%rsp)
140fc5b45:     	pushq	%rbp
140fc5b46:     	pushq	%r15
140fc5b48:     	pushq	%r14
140fc5b4a:     	pushq	%r13
140fc5b4c:     	pushq	%r12
140fc5b4e:     	pushq	%rsi
140fc5b4f:     	pushq	%rdi
140fc5b50:     	pushq	%rbx
140fc5b51:     	subq	$0x78, %rsp
140fc5b55:     	leaq	0x80(%rdx), %rbp
140fc5b5c:     	movaps	%xmm10, 0x20(%rsp)
140fc5b62:     	movaps	%xmm9, 0x30(%rsp)
140fc5b68:     	movaps	%xmm8, 0x40(%rsp)
140fc5b6e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5b74:     	movdqa	%xmm6, 0x60(%rsp)
140fc5b7a:     	cmpq	$0x0, 0x338(%rbp)
140fc5b82:     	je	0x140fc5b9d <.text+0xfc4b9d>
140fc5b84:     	movl	$0x1, %r8d
140fc5b8a:     	movq	0x330(%rbp), %rcx
140fc5b91:     	movq	0x338(%rbp), %rdx
140fc5b98:     	callq	0x140613c20 <.text+0x612c20>
140fc5b9d:     	movdqa	0x60(%rsp), %xmm6
140fc5ba3:     	movdqa	0x50(%rsp), %xmm7
140fc5ba9:     	movaps	0x40(%rsp), %xmm8
140fc5baf:     	movaps	0x30(%rsp), %xmm9
140fc5bb5:     	movaps	0x20(%rsp), %xmm10
140fc5bbb:     	addq	$0x78, %rsp
140fc5bbf:     	popq	%rbx
140fc5bc0:     	popq	%rdi
140fc5bc1:     	popq	%rsi
140fc5bc2:     	popq	%r12
140fc5bc4:     	popq	%r13
140fc5bc6:     	popq	%r14
140fc5bc8:     	popq	%r15
140fc5bca:     	popq	%rbp
140fc5bcb:     	retq
140fc5bcc:     	nopl	(%rax)
140fc5bd0:     	movq	%rdx, 0x10(%rsp)
140fc5bd5:     	pushq	%rbp
140fc5bd6:     	pushq	%r15
140fc5bd8:     	pushq	%r14
140fc5bda:     	pushq	%r13
140fc5bdc:     	pushq	%r12
140fc5bde:     	pushq	%rsi
140fc5bdf:     	pushq	%rdi
140fc5be0:     	pushq	%rbx
140fc5be1:     	subq	$0x78, %rsp
140fc5be5:     	leaq	0x80(%rdx), %rbp
140fc5bec:     	movaps	%xmm10, 0x20(%rsp)
140fc5bf2:     	movaps	%xmm9, 0x30(%rsp)
140fc5bf8:     	movaps	%xmm8, 0x40(%rsp)
140fc5bfe:     	movdqa	%xmm7, 0x50(%rsp)
140fc5c04:     	movdqa	%xmm6, 0x60(%rsp)
140fc5c0a:     	movq	0x230(%rbp), %rdx
140fc5c11:     	testq	%rdx, %rdx
140fc5c14:     	je	0x140fc5c28 <.text+0xfc4c28>
140fc5c16:     	movq	0x238(%rbp), %rcx
140fc5c1d:     	movl	$0x1, %r8d
140fc5c23:     	callq	0x140613c20 <.text+0x612c20>
140fc5c28:     	movdqa	0x60(%rsp), %xmm6
140fc5c2e:     	movdqa	0x50(%rsp), %xmm7
140fc5c34:     	movaps	0x40(%rsp), %xmm8
140fc5c3a:     	movaps	0x30(%rsp), %xmm9
140fc5c40:     	movaps	0x20(%rsp), %xmm10
140fc5c46:     	addq	$0x78, %rsp
140fc5c4a:     	popq	%rbx
140fc5c4b:     	popq	%rdi
140fc5c4c:     	popq	%rsi
140fc5c4d:     	popq	%r12
140fc5c4f:     	popq	%r13
140fc5c51:     	popq	%r14
140fc5c53:     	popq	%r15
140fc5c55:     	popq	%rbp
140fc5c56:     	retq
140fc5c57:     	nopw	(%rax,%rax)
140fc5c60:     	movq	%rdx, 0x10(%rsp)
140fc5c65:     	pushq	%rbp
140fc5c66:     	pushq	%r15
140fc5c68:     	pushq	%r14
140fc5c6a:     	pushq	%r13
140fc5c6c:     	pushq	%r12
140fc5c6e:     	pushq	%rsi
140fc5c6f:     	pushq	%rdi
140fc5c70:     	pushq	%rbx
140fc5c71:     	subq	$0x78, %rsp
140fc5c75:     	leaq	0x80(%rdx), %rbp
140fc5c7c:     	movaps	%xmm10, 0x20(%rsp)
140fc5c82:     	movaps	%xmm9, 0x30(%rsp)
140fc5c88:     	movaps	%xmm8, 0x40(%rsp)
140fc5c8e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5c94:     	movdqa	%xmm6, 0x60(%rsp)
140fc5c9a:     	leaq	0x280(%rbp), %rcx
140fc5ca1:     	callq	0x140fb62b0 <.text+0xfb52b0>
140fc5ca6:     	movdqa	0x60(%rsp), %xmm6
140fc5cac:     	movdqa	0x50(%rsp), %xmm7
140fc5cb2:     	movaps	0x40(%rsp), %xmm8
140fc5cb8:     	movaps	0x30(%rsp), %xmm9
140fc5cbe:     	movaps	0x20(%rsp), %xmm10
140fc5cc4:     	addq	$0x78, %rsp
140fc5cc8:     	popq	%rbx
140fc5cc9:     	popq	%rdi
140fc5cca:     	popq	%rsi
140fc5ccb:     	popq	%r12
140fc5ccd:     	popq	%r13
140fc5ccf:     	popq	%r14
140fc5cd1:     	popq	%r15
140fc5cd3:     	popq	%rbp
140fc5cd4:     	retq
140fc5cd5:     	nopw	%cs:(%rax,%rax)
140fc5ce0:     	movq	%rdx, 0x10(%rsp)
140fc5ce5:     	pushq	%rbp
140fc5ce6:     	pushq	%r15
140fc5ce8:     	pushq	%r14
140fc5cea:     	pushq	%r13
140fc5cec:     	pushq	%r12
140fc5cee:     	pushq	%rsi
140fc5cef:     	pushq	%rdi
140fc5cf0:     	pushq	%rbx
140fc5cf1:     	subq	$0x78, %rsp
140fc5cf5:     	leaq	0x80(%rdx), %rbp
140fc5cfc:     	movaps	%xmm10, 0x20(%rsp)
140fc5d02:     	movaps	%xmm9, 0x30(%rsp)
140fc5d08:     	movaps	%xmm8, 0x40(%rsp)
140fc5d0e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5d14:     	movdqa	%xmm6, 0x60(%rsp)
140fc5d1a:     	movq	0x230(%rbp), %rdx
140fc5d21:     	testq	%rdx, %rdx
140fc5d24:     	je	0x140fc5d38 <.text+0xfc4d38>
140fc5d26:     	movq	0x238(%rbp), %rcx
140fc5d2d:     	movl	$0x1, %r8d
140fc5d33:     	callq	0x140613c20 <.text+0x612c20>
140fc5d38:     	movb	$0x1, 0x347(%rbp)
140fc5d3f:     	movb	$0x1, 0x346(%rbp)
140fc5d46:     	movdqa	0x60(%rsp), %xmm6
140fc5d4c:     	movdqa	0x50(%rsp), %xmm7
140fc5d52:     	movaps	0x40(%rsp), %xmm8
140fc5d58:     	movaps	0x30(%rsp), %xmm9
140fc5d5e:     	movaps	0x20(%rsp), %xmm10
140fc5d64:     	addq	$0x78, %rsp
140fc5d68:     	popq	%rbx
140fc5d69:     	popq	%rdi
140fc5d6a:     	popq	%rsi
140fc5d6b:     	popq	%r12
140fc5d6d:     	popq	%r13
140fc5d6f:     	popq	%r14
140fc5d71:     	popq	%r15
140fc5d73:     	popq	%rbp
140fc5d74:     	retq
140fc5d75:     	nopw	%cs:(%rax,%rax)
140fc5d80:     	movq	%rdx, 0x10(%rsp)
140fc5d85:     	pushq	%rbp
140fc5d86:     	pushq	%r15
140fc5d88:     	pushq	%r14
140fc5d8a:     	pushq	%r13
140fc5d8c:     	pushq	%r12
140fc5d8e:     	pushq	%rsi
140fc5d8f:     	pushq	%rdi
140fc5d90:     	pushq	%rbx
140fc5d91:     	subq	$0x78, %rsp
140fc5d95:     	leaq	0x80(%rdx), %rbp
140fc5d9c:     	movaps	%xmm10, 0x20(%rsp)
140fc5da2:     	movaps	%xmm9, 0x30(%rsp)
140fc5da8:     	movaps	%xmm8, 0x40(%rsp)
140fc5dae:     	movdqa	%xmm7, 0x50(%rsp)
140fc5db4:     	movdqa	%xmm6, 0x60(%rsp)
140fc5dba:     	movq	0x338(%rbp), %rcx
140fc5dc1:     	movq	0x330(%rbp), %rax
140fc5dc8:     	movq	0x8(%rax), %rdx
140fc5dcc:     	testq	%rdx, %rdx
140fc5dcf:     	je	0x140fc5de1 <.text+0xfc4de1>
140fc5dd1:     	movq	0x330(%rbp), %rax
140fc5dd8:     	movq	0x10(%rax), %r8
140fc5ddc:     	callq	0x140613c20 <.text+0x612c20>
140fc5de1:     	movl	$0x18, %edx
140fc5de6:     	movl	$0x8, %r8d
140fc5dec:     	movq	0x320(%rbp), %rcx
140fc5df3:     	callq	0x140613c20 <.text+0x612c20>
140fc5df8:     	movb	$0x1, 0x347(%rbp)
140fc5dff:     	movb	$0x1, 0x346(%rbp)
140fc5e06:     	movdqa	0x60(%rsp), %xmm6
140fc5e0c:     	movdqa	0x50(%rsp), %xmm7
140fc5e12:     	movaps	0x40(%rsp), %xmm8
140fc5e18:     	movaps	0x30(%rsp), %xmm9
140fc5e1e:     	movaps	0x20(%rsp), %xmm10
140fc5e24:     	addq	$0x78, %rsp
140fc5e28:     	popq	%rbx
140fc5e29:     	popq	%rdi
140fc5e2a:     	popq	%rsi
140fc5e2b:     	popq	%r12
140fc5e2d:     	popq	%r13
140fc5e2f:     	popq	%r14
140fc5e31:     	popq	%r15
140fc5e33:     	popq	%rbp
140fc5e34:     	retq
140fc5e35:     	nopw	%cs:(%rax,%rax)
140fc5e40:     	movq	%rdx, 0x10(%rsp)
140fc5e45:     	pushq	%rbp
140fc5e46:     	pushq	%r15
140fc5e48:     	pushq	%r14
140fc5e4a:     	pushq	%r13
140fc5e4c:     	pushq	%r12
140fc5e4e:     	pushq	%rsi
140fc5e4f:     	pushq	%rdi
140fc5e50:     	pushq	%rbx
140fc5e51:     	subq	$0x78, %rsp
140fc5e55:     	leaq	0x80(%rdx), %rbp
140fc5e5c:     	movaps	%xmm10, 0x20(%rsp)
140fc5e62:     	movaps	%xmm9, 0x30(%rsp)
140fc5e68:     	movaps	%xmm8, 0x40(%rsp)
140fc5e6e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5e74:     	movdqa	%xmm6, 0x60(%rsp)
140fc5e7a:     	movq	0x160(%rbp), %rdx
140fc5e81:     	testq	%rdx, %rdx
140fc5e84:     	je	0x140fc5e98 <.text+0xfc4e98>
140fc5e86:     	movq	0x168(%rbp), %rcx
140fc5e8d:     	movl	$0x1, %r8d
140fc5e93:     	callq	0x140613c20 <.text+0x612c20>
140fc5e98:     	movb	$0x1, 0x345(%rbp)
140fc5e9f:     	movdqa	0x60(%rsp), %xmm6
140fc5ea5:     	movdqa	0x50(%rsp), %xmm7
140fc5eab:     	movaps	0x40(%rsp), %xmm8
140fc5eb1:     	movaps	0x30(%rsp), %xmm9
140fc5eb7:     	movaps	0x20(%rsp), %xmm10
140fc5ebd:     	addq	$0x78, %rsp
140fc5ec1:     	popq	%rbx
140fc5ec2:     	popq	%rdi
140fc5ec3:     	popq	%rsi
140fc5ec4:     	popq	%r12
140fc5ec6:     	popq	%r13
140fc5ec8:     	popq	%r14
140fc5eca:     	popq	%r15
140fc5ecc:     	popq	%rbp
140fc5ecd:     	retq
140fc5ece:     	nop
140fc5ed0:     	movq	%rdx, 0x10(%rsp)
140fc5ed5:     	pushq	%rbp
140fc5ed6:     	pushq	%r15
140fc5ed8:     	pushq	%r14
140fc5eda:     	pushq	%r13
140fc5edc:     	pushq	%r12
140fc5ede:     	pushq	%rsi
140fc5edf:     	pushq	%rdi
140fc5ee0:     	pushq	%rbx
140fc5ee1:     	subq	$0x78, %rsp
140fc5ee5:     	leaq	0x80(%rdx), %rbp
140fc5eec:     	movaps	%xmm10, 0x20(%rsp)
140fc5ef2:     	movaps	%xmm9, 0x30(%rsp)
140fc5ef8:     	movaps	%xmm8, 0x40(%rsp)
140fc5efe:     	movdqa	%xmm7, 0x50(%rsp)
140fc5f04:     	movdqa	%xmm6, 0x60(%rsp)
140fc5f0a:     	cmpq	$0x0, 0x338(%rbp)
140fc5f12:     	je	0x140fc5f2d <.text+0xfc4f2d>
140fc5f14:     	movl	$0x1, %r8d
140fc5f1a:     	movq	0x320(%rbp), %rcx
140fc5f21:     	movq	0x338(%rbp), %rdx
140fc5f28:     	callq	0x140613c20 <.text+0x612c20>
140fc5f2d:     	movl	0x330(%rbp), %eax
140fc5f33:     	movb	%al, 0x345(%rbp)
140fc5f39:     	movdqa	0x60(%rsp), %xmm6
140fc5f3f:     	movdqa	0x50(%rsp), %xmm7
140fc5f45:     	movaps	0x40(%rsp), %xmm8
140fc5f4b:     	movaps	0x30(%rsp), %xmm9
140fc5f51:     	movaps	0x20(%rsp), %xmm10
140fc5f57:     	addq	$0x78, %rsp
140fc5f5b:     	popq	%rbx
140fc5f5c:     	popq	%rdi
140fc5f5d:     	popq	%rsi
140fc5f5e:     	popq	%r12
140fc5f60:     	popq	%r13
140fc5f62:     	popq	%r14
140fc5f64:     	popq	%r15
140fc5f66:     	popq	%rbp
140fc5f67:     	retq
140fc5f68:     	nopl	(%rax,%rax)
140fc5f70:     	movq	%rdx, 0x10(%rsp)
140fc5f75:     	pushq	%rbp
140fc5f76:     	pushq	%r15
140fc5f78:     	pushq	%r14
140fc5f7a:     	pushq	%r13
140fc5f7c:     	pushq	%r12
140fc5f7e:     	pushq	%rsi
140fc5f7f:     	pushq	%rdi
140fc5f80:     	pushq	%rbx
140fc5f81:     	subq	$0x78, %rsp
140fc5f85:     	leaq	0x80(%rdx), %rbp
140fc5f8c:     	movaps	%xmm10, 0x20(%rsp)
140fc5f92:     	movaps	%xmm9, 0x30(%rsp)
140fc5f98:     	movaps	%xmm8, 0x40(%rsp)
140fc5f9e:     	movdqa	%xmm7, 0x50(%rsp)
140fc5fa4:     	movdqa	%xmm6, 0x60(%rsp)
140fc5faa:     	movq	0x160(%rbp), %rdx
140fc5fb1:     	testq	%rdx, %rdx
140fc5fb4:     	je	0x140fc5fc8 <.text+0xfc4fc8>
140fc5fb6:     	movq	0x168(%rbp), %rcx
140fc5fbd:     	movl	$0x1, %r8d
140fc5fc3:     	callq	0x140613c20 <.text+0x612c20>
140fc5fc8:     	movb	$0x1, 0x344(%rbp)
140fc5fcf:     	movdqa	0x60(%rsp), %xmm6
140fc5fd5:     	movdqa	0x50(%rsp), %xmm7
140fc5fdb:     	movaps	0x40(%rsp), %xmm8
140fc5fe1:     	movaps	0x30(%rsp), %xmm9
140fc5fe7:     	movaps	0x20(%rsp), %xmm10
140fc5fed:     	addq	$0x78, %rsp
140fc5ff1:     	popq	%rbx
140fc5ff2:     	popq	%rdi
140fc5ff3:     	popq	%rsi
140fc5ff4:     	popq	%r12
140fc5ff6:     	popq	%r13
140fc5ff8:     	popq	%r14
140fc5ffa:     	popq	%r15
140fc5ffc:     	popq	%rbp
140fc5ffd:     	retq
140fc5ffe:     	nop
140fc6000:     	movq	%rdx, 0x10(%rsp)
140fc6005:     	pushq	%rbp
140fc6006:     	pushq	%r15
140fc6008:     	pushq	%r14
140fc600a:     	pushq	%r13
140fc600c:     	pushq	%r12
140fc600e:     	pushq	%rsi
140fc600f:     	pushq	%rdi
140fc6010:     	pushq	%rbx
140fc6011:     	subq	$0x78, %rsp
140fc6015:     	leaq	0x80(%rdx), %rbp
140fc601c:     	movaps	%xmm10, 0x20(%rsp)
140fc6022:     	movaps	%xmm9, 0x30(%rsp)
140fc6028:     	movaps	%xmm8, 0x40(%rsp)
140fc602e:     	movdqa	%xmm7, 0x50(%rsp)
140fc6034:     	movdqa	%xmm6, 0x60(%rsp)
140fc603a:     	leaq	0x160(%rbp), %rcx
140fc6041:     	callq	0x14014b4d0 <.text+0x14a4d0>
140fc6046:     	movdqa	0x60(%rsp), %xmm6
140fc604c:     	movdqa	0x50(%rsp), %xmm7
140fc6052:     	movaps	0x40(%rsp), %xmm8
140fc6058:     	movaps	0x30(%rsp), %xmm9
140fc605e:     	movaps	0x20(%rsp), %xmm10
140fc6064:     	addq	$0x78, %rsp
140fc6068:     	popq	%rbx
140fc6069:     	popq	%rdi
140fc606a:     	popq	%rsi
140fc606b:     	popq	%r12
140fc606d:     	popq	%r13
140fc606f:     	popq	%r14
140fc6071:     	popq	%r15
140fc6073:     	popq	%rbp
140fc6074:     	retq
140fc6075:     	nopw	%cs:(%rax,%rax)
140fc6080:     	movq	%rdx, 0x10(%rsp)
140fc6085:     	pushq	%rbp
140fc6086:     	pushq	%r15
140fc6088:     	pushq	%r14
140fc608a:     	pushq	%r13
140fc608c:     	pushq	%r12
140fc608e:     	pushq	%rsi
140fc608f:     	pushq	%rdi
140fc6090:     	pushq	%rbx
140fc6091:     	subq	$0x78, %rsp
140fc6095:     	leaq	0x80(%rdx), %rbp
140fc609c:     	movaps	%xmm10, 0x20(%rsp)
140fc60a2:     	movaps	%xmm9, 0x30(%rsp)
140fc60a8:     	movaps	%xmm8, 0x40(%rsp)
140fc60ae:     	movdqa	%xmm7, 0x50(%rsp)
140fc60b4:     	movdqa	%xmm6, 0x60(%rsp)
140fc60ba:     	movq	0x280(%rbp), %rdx
140fc60c1:     	testq	%rdx, %rdx
140fc60c4:     	je	0x140fc60d8 <.text+0xfc50d8>
140fc60c6:     	movq	0x288(%rbp), %rcx
140fc60cd:     	movl	$0x1, %r8d
140fc60d3:     	callq	0x140613c20 <.text+0x612c20>
140fc60d8:     	movdqa	0x60(%rsp), %xmm6
140fc60de:     	movdqa	0x50(%rsp), %xmm7
140fc60e4:     	movaps	0x40(%rsp), %xmm8
140fc60ea:     	movaps	0x30(%rsp), %xmm9
140fc60f0:     	movaps	0x20(%rsp), %xmm10
140fc60f6:     	addq	$0x78, %rsp
140fc60fa:     	popq	%rbx
140fc60fb:     	popq	%rdi
140fc60fc:     	popq	%rsi
140fc60fd:     	popq	%r12
140fc60ff:     	popq	%r13
140fc6101:     	popq	%r14
140fc6103:     	popq	%r15
140fc6105:     	popq	%rbp
140fc6106:     	retq
140fc6107:     	nopw	(%rax,%rax)
140fc6110:     	movq	%rdx, 0x10(%rsp)
140fc6115:     	pushq	%rbp
140fc6116:     	pushq	%r15
140fc6118:     	pushq	%r14
140fc611a:     	pushq	%r13
140fc611c:     	pushq	%r12
140fc611e:     	pushq	%rsi
140fc611f:     	pushq	%rdi
140fc6120:     	pushq	%rbx
140fc6121:     	subq	$0x78, %rsp
140fc6125:     	leaq	0x80(%rdx), %rbp
140fc612c:     	movaps	%xmm10, 0x20(%rsp)
140fc6132:     	movaps	%xmm9, 0x30(%rsp)
140fc6138:     	movaps	%xmm8, 0x40(%rsp)
140fc613e:     	movdqa	%xmm7, 0x50(%rsp)
140fc6144:     	movdqa	%xmm6, 0x60(%rsp)
140fc614a:     	leaq	0x230(%rbp), %rcx
140fc6151:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc6156:     	movdqa	0x60(%rsp), %xmm6
140fc615c:     	movdqa	0x50(%rsp), %xmm7
140fc6162:     	movaps	0x40(%rsp), %xmm8
140fc6168:     	movaps	0x30(%rsp), %xmm9
140fc616e:     	movaps	0x20(%rsp), %xmm10
140fc6174:     	addq	$0x78, %rsp
140fc6178:     	popq	%rbx
140fc6179:     	popq	%rdi
140fc617a:     	popq	%rsi
140fc617b:     	popq	%r12
140fc617d:     	popq	%r13
140fc617f:     	popq	%r14
140fc6181:     	popq	%r15
140fc6183:     	popq	%rbp
140fc6184:     	retq
140fc6185:     	nopw	%cs:(%rax,%rax)
140fc6190:     	movq	%rdx, 0x10(%rsp)
140fc6195:     	pushq	%rbp
140fc6196:     	pushq	%r15
140fc6198:     	pushq	%r14
140fc619a:     	pushq	%r13
140fc619c:     	pushq	%r12
140fc619e:     	pushq	%rsi
140fc619f:     	pushq	%rdi
140fc61a0:     	pushq	%rbx
140fc61a1:     	subq	$0x78, %rsp
140fc61a5:     	leaq	0x80(%rdx), %rbp
140fc61ac:     	movaps	%xmm10, 0x20(%rsp)
140fc61b2:     	movaps	%xmm9, 0x30(%rsp)
140fc61b8:     	movaps	%xmm8, 0x40(%rsp)
140fc61be:     	movdqa	%xmm7, 0x50(%rsp)
140fc61c4:     	movdqa	%xmm6, 0x60(%rsp)
140fc61ca:     	cmpl	$0x2, 0x160(%rbp)
140fc61d1:     	jb	0x140fc61df <.text+0xfc51df>
140fc61d3:     	leaq	0x168(%rbp), %rcx
140fc61da:     	callq	0x1405e7a00 <.text+0x5e6a00>
140fc61df:     	movaps	0x60(%rsp), %xmm6
140fc61e4:     	movaps	0x50(%rsp), %xmm7
140fc61e9:     	movaps	0x40(%rsp), %xmm8
140fc61ef:     	movaps	0x30(%rsp), %xmm9
140fc61f5:     	movaps	0x20(%rsp), %xmm10
140fc61fb:     	addq	$0x78, %rsp
140fc61ff:     	popq	%rbx
140fc6200:     	popq	%rdi
140fc6201:     	popq	%rsi
140fc6202:     	popq	%r12
140fc6204:     	popq	%r13
140fc6206:     	popq	%r14
140fc6208:     	popq	%r15
140fc620a:     	popq	%rbp
140fc620b:     	retq
140fc620c:     	nopl	(%rax)
140fc6210:     	movq	%rdx, 0x10(%rsp)
140fc6215:     	pushq	%rbp
140fc6216:     	pushq	%r15
140fc6218:     	pushq	%r14
140fc621a:     	pushq	%r13
140fc621c:     	pushq	%r12
140fc621e:     	pushq	%rsi
140fc621f:     	pushq	%rdi
140fc6220:     	pushq	%rbx
140fc6221:     	subq	$0x78, %rsp
140fc6225:     	leaq	0x80(%rdx), %rbp
140fc622c:     	movaps	%xmm10, 0x20(%rsp)
140fc6232:     	movaps	%xmm9, 0x30(%rsp)
140fc6238:     	movaps	%xmm8, 0x40(%rsp)
140fc623e:     	movdqa	%xmm7, 0x50(%rsp)
140fc6244:     	movdqa	%xmm6, 0x60(%rsp)
140fc624a:     	testb	$0x1, 0x345(%rbp)
140fc6251:     	je	0x140fc62a0 <.text+0xfc52a0>
140fc6253:     	movq	0x230(%rbp), %rdx
140fc625a:     	testq	%rdx, %rdx
140fc625d:     	je	0x140fc6271 <.text+0xfc5271>
140fc625f:     	movq	0x238(%rbp), %rcx
140fc6266:     	movl	$0x1, %r8d
140fc626c:     	callq	0x140613c20 <.text+0x612c20>
140fc6271:     	movdqa	0x60(%rsp), %xmm6
140fc6277:     	movdqa	0x50(%rsp), %xmm7
140fc627d:     	movaps	0x40(%rsp), %xmm8
140fc6283:     	movaps	0x30(%rsp), %xmm9
140fc6289:     	movaps	0x20(%rsp), %xmm10
140fc628f:     	addq	$0x78, %rsp
140fc6293:     	popq	%rbx
140fc6294:     	popq	%rdi
140fc6295:     	popq	%rsi
140fc6296:     	popq	%r12
140fc6298:     	popq	%r13
140fc629a:     	popq	%r14
140fc629c:     	popq	%r15
140fc629e:     	popq	%rbp
140fc629f:     	retq
140fc62a0:     	movdqa	0x60(%rsp), %xmm6
140fc62a6:     	movdqa	0x50(%rsp), %xmm7
140fc62ac:     	movaps	0x40(%rsp), %xmm8
140fc62b2:     	movaps	0x30(%rsp), %xmm9
140fc62b8:     	movaps	0x20(%rsp), %xmm10
140fc62be:     	addq	$0x78, %rsp
140fc62c2:     	popq	%rbx
140fc62c3:     	popq	%rdi
140fc62c4:     	popq	%rsi
140fc62c5:     	popq	%r12
140fc62c7:     	popq	%r13
140fc62c9:     	popq	%r14
140fc62cb:     	popq	%r15
140fc62cd:     	popq	%rbp
140fc62ce:     	retq
140fc62cf:     	nop
140fc62d0:     	movq	%rdx, 0x10(%rsp)
140fc62d5:     	pushq	%rbp
140fc62d6:     	pushq	%r15
140fc62d8:     	pushq	%r14
140fc62da:     	pushq	%r13
140fc62dc:     	pushq	%r12
140fc62de:     	pushq	%rsi
140fc62df:     	pushq	%rdi
140fc62e0:     	pushq	%rbx
140fc62e1:     	subq	$0x78, %rsp
140fc62e5:     	leaq	0x80(%rdx), %rbp
140fc62ec:     	movaps	%xmm10, 0x20(%rsp)
140fc62f2:     	movaps	%xmm9, 0x30(%rsp)
140fc62f8:     	movaps	%xmm8, 0x40(%rsp)
140fc62fe:     	movdqa	%xmm7, 0x50(%rsp)
140fc6304:     	movdqa	%xmm6, 0x60(%rsp)
140fc630a:     	leaq	0x110(%rbp), %rcx
140fc6311:     	callq	0x140d4cc30 <.text+0xd4bc30>
140fc6316:     	movdqa	0x60(%rsp), %xmm6
140fc631c:     	movdqa	0x50(%rsp), %xmm7
140fc6322:     	movaps	0x40(%rsp), %xmm8
140fc6328:     	movaps	0x30(%rsp), %xmm9
140fc632e:     	movaps	0x20(%rsp), %xmm10
140fc6334:     	addq	$0x78, %rsp
140fc6338:     	popq	%rbx
140fc6339:     	popq	%rdi
140fc633a:     	popq	%rsi
140fc633b:     	popq	%r12
140fc633d:     	popq	%r13
140fc633f:     	popq	%r14
140fc6341:     	popq	%r15
140fc6343:     	popq	%rbp
140fc6344:     	retq
140fc6345:     	nopw	%cs:(%rax,%rax)
140fc6350:     	movq	%rdx, 0x10(%rsp)
140fc6355:     	pushq	%rbp
140fc6356:     	pushq	%r15
140fc6358:     	pushq	%r14
140fc635a:     	pushq	%r13
140fc635c:     	pushq	%r12
140fc635e:     	pushq	%rsi
140fc635f:     	pushq	%rdi
140fc6360:     	pushq	%rbx
140fc6361:     	subq	$0x78, %rsp
140fc6365:     	leaq	0x80(%rdx), %rbp
140fc636c:     	movaps	%xmm10, 0x20(%rsp)
140fc6372:     	movaps	%xmm9, 0x30(%rsp)
140fc6378:     	movaps	%xmm8, 0x40(%rsp)
140fc637e:     	movdqa	%xmm7, 0x50(%rsp)
140fc6384:     	movdqa	%xmm6, 0x60(%rsp)
140fc638a:     	cmpq	$0x0, 0x200(%rbp)
140fc6392:     	je	0x140fc63ae <.text+0xfc53ae>
140fc6394:     	movb	$0x1, 0x347(%rbp)
140fc639b:     	movb	$0x1, 0x346(%rbp)
140fc63a2:     	leaq	0x200(%rbp), %rcx
140fc63a9:     	callq	0x140fb64f0 <.text+0xfb54f0>
140fc63ae:     	movb	$0x1, 0x347(%rbp)
140fc63b5:     	movb	$0x1, 0x346(%rbp)
140fc63bc:     	movdqa	0x60(%rsp), %xmm6
140fc63c2:     	movdqa	0x50(%rsp), %xmm7
140fc63c8:     	movaps	0x40(%rsp), %xmm8
140fc63ce:     	movaps	0x30(%rsp), %xmm9
140fc63d4:     	movaps	0x20(%rsp), %xmm10
140fc63da:     	addq	$0x78, %rsp
140fc63de:     	popq	%rbx
140fc63df:     	popq	%rdi
140fc63e0:     	popq	%rsi
140fc63e1:     	popq	%r12
140fc63e3:     	popq	%r13
140fc63e5:     	popq	%r14
140fc63e7:     	popq	%r15
140fc63e9:     	popq	%rbp
140fc63ea:     	retq
140fc63eb:     	nopl	(%rax,%rax)
140fc63f0:     	movq	%rdx, 0x10(%rsp)
140fc63f5:     	pushq	%rbp
140fc63f6:     	pushq	%r15
140fc63f8:     	pushq	%r14
140fc63fa:     	pushq	%r13
140fc63fc:     	pushq	%r12
140fc63fe:     	pushq	%rsi
140fc63ff:     	pushq	%rdi
140fc6400:     	pushq	%rbx
140fc6401:     	subq	$0x78, %rsp
140fc6405:     	leaq	0x80(%rdx), %rbp
140fc640c:     	movaps	%xmm10, 0x20(%rsp)
140fc6412:     	movaps	%xmm9, 0x30(%rsp)
140fc6418:     	movaps	%xmm8, 0x40(%rsp)
140fc641e:     	movdqa	%xmm7, 0x50(%rsp)
140fc6424:     	movdqa	%xmm6, 0x60(%rsp)
140fc642a:     	movzbl	0x346(%rbp), %ebx
140fc6431:     	testb	$0x1, 0x347(%rbp)
140fc6438:     	je	0x140fc6490 <.text+0xfc5490>
140fc643a:     	movq	0x110(%rbp), %rdx
140fc6441:     	testq	%rdx, %rdx
140fc6444:     	je	0x140fc6458 <.text+0xfc5458>
140fc6446:     	movq	0x118(%rbp), %rcx
140fc644d:     	movl	$0x1, %r8d
140fc6453:     	callq	0x140613c20 <.text+0x612c20>
140fc6458:     	andb	$0x1, %bl
140fc645b:     	movb	%bl, 0x344(%rbp)
140fc6461:     	movdqa	0x60(%rsp), %xmm6
140fc6467:     	movdqa	0x50(%rsp), %xmm7
140fc646d:     	movaps	0x40(%rsp), %xmm8
140fc6473:     	movaps	0x30(%rsp), %xmm9
140fc6479:     	movaps	0x20(%rsp), %xmm10
140fc647f:     	addq	$0x78, %rsp
140fc6483:     	popq	%rbx
140fc6484:     	popq	%rdi
140fc6485:     	popq	%rsi
140fc6486:     	popq	%r12
140fc6488:     	popq	%r13
140fc648a:     	popq	%r14
140fc648c:     	popq	%r15
140fc648e:     	popq	%rbp
140fc648f:     	retq
140fc6490:     	andb	$0x1, %bl
140fc6493:     	movb	%bl, 0x344(%rbp)
140fc6499:     	movdqa	0x60(%rsp), %xmm6
140fc649f:     	movdqa	0x50(%rsp), %xmm7
140fc64a5:     	movaps	0x40(%rsp), %xmm8
140fc64ab:     	movaps	0x30(%rsp), %xmm9
140fc64b1:     	movaps	0x20(%rsp), %xmm10
140fc64b7:     	addq	$0x78, %rsp
140fc64bb:     	popq	%rbx
140fc64bc:     	popq	%rdi
140fc64bd:     	popq	%rsi
140fc64be:     	popq	%r12
140fc64c0:     	popq	%r13
140fc64c2:     	popq	%r14
140fc64c4:     	popq	%r15
140fc64c6:     	popq	%rbp
140fc64c7:     	retq
140fc64c8:     	nopl	(%rax,%rax)
140fc64d0:     	movq	%rdx, 0x10(%rsp)
140fc64d5:     	pushq	%rbp
140fc64d6:     	pushq	%r15
140fc64d8:     	pushq	%r14
140fc64da:     	pushq	%r13
140fc64dc:     	pushq	%r12
140fc64de:     	pushq	%rsi
140fc64df:     	pushq	%rdi
140fc64e0:     	pushq	%rbx
140fc64e1:     	subq	$0x78, %rsp
140fc64e5:     	leaq	0x80(%rdx), %rbp
140fc64ec:     	movaps	%xmm10, 0x20(%rsp)
140fc64f2:     	movaps	%xmm9, 0x30(%rsp)
140fc64f8:     	movaps	%xmm8, 0x40(%rsp)
140fc64fe:     	movdqa	%xmm7, 0x50(%rsp)
140fc6504:     	movdqa	%xmm6, 0x60(%rsp)
140fc650a:     	cmpb	$0x0, 0x344(%rbp)
140fc6511:     	je	0x140fc654e <.text+0xfc554e>
140fc6513:     	leaq	0x80(%rbp), %rcx
140fc651a:     	callq	0x140faa000 <.text+0xfa9000>
140fc651f:     	movdqa	0x60(%rsp), %xmm6
140fc6525:     	movdqa	0x50(%rsp), %xmm7
140fc652b:     	movaps	0x40(%rsp), %xmm8
140fc6531:     	movaps	0x30(%rsp), %xmm9
140fc6537:     	movaps	0x20(%rsp), %xmm10
140fc653d:     	addq	$0x78, %rsp
140fc6541:     	popq	%rbx
140fc6542:     	popq	%rdi
140fc6543:     	popq	%rsi
140fc6544:     	popq	%r12
140fc6546:     	popq	%r13
140fc6548:     	popq	%r14
140fc654a:     	popq	%r15
140fc654c:     	popq	%rbp
140fc654d:     	retq
140fc654e:     	movdqa	0x60(%rsp), %xmm6
140fc6554:     	movdqa	0x50(%rsp), %xmm7
140fc655a:     	movaps	0x40(%rsp), %xmm8
140fc6560:     	movaps	0x30(%rsp), %xmm9
140fc6566:     	movaps	0x20(%rsp), %xmm10
140fc656c:     	addq	$0x78, %rsp
140fc6570:     	popq	%rbx
140fc6571:     	popq	%rdi
140fc6572:     	popq	%rsi
140fc6573:     	popq	%r12
140fc6575:     	popq	%r13
140fc6577:     	popq	%r14
140fc6579:     	popq	%r15
140fc657b:     	popq	%rbp
140fc657c:     	retq
140fc657d:     	nopl	(%rax)
140fc6580:     	movq	%rdx, 0x10(%rsp)
140fc6585:     	pushq	%rbp
140fc6586:     	pushq	%r15
140fc6588:     	pushq	%r14
140fc658a:     	pushq	%r13
140fc658c:     	pushq	%r12
140fc658e:     	pushq	%rsi
140fc658f:     	pushq	%rdi
140fc6590:     	pushq	%rbx
140fc6591:     	subq	$0x78, %rsp
140fc6595:     	leaq	0x80(%rdx), %rbp
140fc659c:     	movaps	%xmm10, 0x20(%rsp)
140fc65a2:     	movaps	%xmm9, 0x30(%rsp)
140fc65a8:     	movaps	%xmm8, 0x40(%rsp)
140fc65ae:     	movdqa	%xmm7, 0x50(%rsp)
140fc65b4:     	movdqa	%xmm6, 0x60(%rsp)
140fc65ba:     	movq	-0x18(%rbp), %rdx
140fc65be:     	testq	%rdx, %rdx
140fc65c1:     	je	0x140fc65d2 <.text+0xfc55d2>
140fc65c3:     	movq	-0x10(%rbp), %rcx
140fc65c7:     	movl	$0x1, %r8d
140fc65cd:     	callq	0x140613c20 <.text+0x612c20>
140fc65d2:     	movdqa	0x60(%rsp), %xmm6
140fc65d8:     	movdqa	0x50(%rsp), %xmm7
140fc65de:     	movaps	0x40(%rsp), %xmm8
140fc65e4:     	movaps	0x30(%rsp), %xmm9
140fc65ea:     	movaps	0x20(%rsp), %xmm10
140fc65f0:     	addq	$0x78, %rsp
140fc65f4:     	popq	%rbx
140fc65f5:     	popq	%rdi
140fc65f6:     	popq	%rsi
140fc65f7:     	popq	%r12
140fc65f9:     	popq	%r13
140fc65fb:     	popq	%r14
140fc65fd:     	popq	%r15
140fc65ff:     	popq	%rbp
140fc6600:     	retq
140fc6601:     	nopw	%cs:(%rax,%rax)
140fc6610:     	movq	%rdx, 0x10(%rsp)
140fc6615:     	pushq	%rbp
140fc6616:     	pushq	%r15
140fc6618:     	pushq	%r14
140fc661a:     	pushq	%r13
140fc661c:     	pushq	%r12
140fc661e:     	pushq	%rsi
140fc661f:     	pushq	%rdi
140fc6620:     	pushq	%rbx
140fc6621:     	subq	$0x78, %rsp
140fc6625:     	leaq	0x80(%rdx), %rbp
140fc662c:     	movaps	%xmm10, 0x20(%rsp)
140fc6632:     	movaps	%xmm9, 0x30(%rsp)
140fc6638:     	movaps	%xmm8, 0x40(%rsp)
140fc663e:     	movdqa	%xmm7, 0x50(%rsp)
140fc6644:     	movdqa	%xmm6, 0x60(%rsp)
140fc664a:     	movq	0xa8(%rbp), %rdx
140fc6651:     	testq	%rdx, %rdx
140fc6654:     	je	0x140fc6668 <.text+0xfc5668>
140fc6656:     	movl	$0x1, %r8d
140fc665c:     	movq	0x148(%rbp), %rcx
140fc6663:     	callq	0x140613c20 <.text+0x612c20>
140fc6668:     	movaps	0x60(%rsp), %xmm6
140fc666d:     	movaps	0x50(%rsp), %xmm7
140fc6672:     	movaps	0x40(%rsp), %xmm8
140fc6678:     	movaps	0x30(%rsp), %xmm9
140fc667e:     	movaps	0x20(%rsp), %xmm10
140fc6684:     	addq	$0x78, %rsp
140fc6688:     	popq	%rbx
140fc6689:     	popq	%rdi
140fc668a:     	popq	%rsi
140fc668b:     	popq	%r12
140fc668d:     	popq	%r13
140fc668f:     	popq	%r14
140fc6691:     	popq	%r15
140fc6693:     	popq	%rbp
140fc6694:     	retq
