
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1d680:     	pushq	%rbp
140b1d681:     	pushq	%r15
140b1d683:     	pushq	%r14
140b1d685:     	pushq	%r13
140b1d687:     	pushq	%r12
140b1d689:     	pushq	%rsi
140b1d68a:     	pushq	%rdi
140b1d68b:     	pushq	%rbx
140b1d68c:     	subq	$0x2e8, %rsp            # imm = 0x2E8
140b1d693:     	leaq	0x80(%rsp), %rbp
140b1d69b:     	movq	$-0x2, 0x260(%rbp)
140b1d6a6:     	movq	%rdx, %r14
140b1d6a9:     	movq	0x2d0(%rbp), %rsi
140b1d6b0:     	movq	(%rdx), %r15
140b1d6b3:     	movq	0x8(%rsi), %rbx
140b1d6b7:     	movq	0x10(%rsi), %rdi
140b1d6bb:     	movq	%r15, %rcx
140b1d6be:     	movq	%rbx, %rdx
140b1d6c1:     	movq	%rdi, %r8
140b1d6c4:     	callq	0x140d40d70 <.text+0xd3fd70>
140b1d6c9:     	movq	$0x0, 0x230(%rbp)
140b1d6d4:     	movq	$0x8, 0x238(%rbp)
140b1d6df:     	movq	$0x0, 0x240(%rbp)
140b1d6ea:     	cmpq	$0x0, 0x30(%r15)
140b1d6ef:     	movabsq	$-0x306abbe6b64e807b, %rax # imm = 0xCF95441949B17F85
140b1d6f9:     	movabsq	$0x6d65e656c0a8d05c, %rcx # imm = 0x6D65E656C0A8D05C
140b1d703:     	je	0x140b1d7a3 <.text+0xb1c7a3>
140b1d709:     	movq	0x18(%r15), %rdx
140b1d70d:     	movq	0x20(%r15), %r8
140b1d711:     	movq	%r8, %r9
140b1d714:     	andq	%rax, %r9
140b1d717:     	xorl	%r10d, %r10d
140b1d71a:     	movdqa	0x1e43efe(%rip), %xmm0  # 0x142961620
140b1d722:     	pcmpeqd	%xmm1, %xmm1
140b1d726:     	movdqu	(%rdx,%r9), %xmm2
140b1d72c:     	movdqa	%xmm2, %xmm3
140b1d730:     	pcmpeqb	%xmm0, %xmm3
140b1d734:     	pmovmskb	%xmm3, %r11d
140b1d739:     	testl	%r11d, %r11d
140b1d73c:     	jne	0x140b1d76d <.text+0xb1c76d>
140b1d73e:     	pcmpeqb	%xmm1, %xmm2
140b1d742:     	pmovmskb	%xmm2, %r11d
140b1d747:     	testl	%r11d, %r11d
140b1d74a:     	jne	0x140b1d7a3 <.text+0xb1c7a3>
140b1d74c:     	addq	%r10, %r9
140b1d74f:     	addq	$0x10, %r9
140b1d753:     	addq	$0x10, %r10
140b1d757:     	andq	%r8, %r9
140b1d75a:     	jmp	0x140b1d726 <.text+0xb1c726>
140b1d75c:     	nopl	(%rax)
140b1d760:     	leal	-0x1(%r11), %r12d
140b1d764:     	andw	%r11w, %r12w
140b1d768:     	movl	%r12d, %r11d
140b1d76b:     	je	0x140b1d73e <.text+0xb1c73e>
140b1d76d:     	tzcntl	%r11d, %r12d
140b1d772:     	addq	%r9, %r12
140b1d775:     	andq	%r8, %r12
140b1d778:     	negq	%r12
140b1d77b:     	leaq	(%r12,%r12,2), %r12
140b1d77f:     	cmpq	%rcx, -0x18(%rdx,%r12,8)
140b1d784:     	jne	0x140b1d760 <.text+0xb1c760>
140b1d786:     	cmpq	%rax, -0x10(%rdx,%r12,8)
140b1d78b:     	jne	0x140b1d760 <.text+0xb1c760>
140b1d78d:     	leaq	(%rdx,%r12,8), %rax
140b1d791:     	movq	-0x8(%rax), %r12
140b1d795:     	cmpq	(%rsi), %rdi
140b1d798:     	je	0x140b1daf0 <.text+0xb1caf0>
140b1d79e:     	jmp	0x140b1db99 <.text+0xb1cb99>
140b1d7a3:     	movq	%rbx, 0x248(%rbp)
140b1d7aa:     	movzbl	0x60(%r15), %edx
140b1d7af:     	movq	0x80(%r15), %rdx
140b1d7b6:     	movq	0x88(%r15), %r9
140b1d7bd:     	movq	%r9, %r10
140b1d7c0:     	andq	%rax, %r10
140b1d7c3:     	xorl	%r11d, %r11d
140b1d7c6:     	movdqa	0x1e43e52(%rip), %xmm0  # 0x142961620
140b1d7ce:     	pcmpeqd	%xmm1, %xmm1
140b1d7d2:     	movdqu	(%rdx,%r10), %xmm2
140b1d7d8:     	movdqa	%xmm2, %xmm3
140b1d7dc:     	pcmpeqb	%xmm0, %xmm3
140b1d7e0:     	pmovmskb	%xmm3, %r12d
140b1d7e5:     	testl	%r12d, %r12d
140b1d7e8:     	jne	0x140b1d81e <.text+0xb1c81e>
140b1d7ea:     	pcmpeqb	%xmm1, %xmm2
140b1d7ee:     	pmovmskb	%xmm2, %r8d
140b1d7f3:     	testl	%r8d, %r8d
140b1d7f6:     	jne	0x140b1d8ee <.text+0xb1c8ee>
140b1d7fc:     	addq	%r11, %r10
140b1d7ff:     	addq	$0x10, %r10
140b1d803:     	addq	$0x10, %r11
140b1d807:     	andq	%r9, %r10
140b1d80a:     	jmp	0x140b1d7d2 <.text+0xb1c7d2>
140b1d80c:     	nopl	(%rax)
140b1d810:     	leal	-0x1(%r12), %r8d
140b1d815:     	andw	%r12w, %r8w
140b1d819:     	movl	%r8d, %r12d
140b1d81c:     	je	0x140b1d7ea <.text+0xb1c7ea>
140b1d81e:     	tzcntl	%r12d, %r13d
140b1d823:     	addq	%r10, %r13
140b1d826:     	andq	%r9, %r13
140b1d829:     	movq	%r13, %r8
140b1d82c:     	negq	%r8
140b1d82f:     	leaq	(,%r13,8), %rbx
140b1d837:     	shlq	$0x7, %r8
140b1d83b:     	subq	%rbx, %r8
140b1d83e:     	cmpq	%rcx, -0x88(%rdx,%r8)
140b1d846:     	jne	0x140b1d810 <.text+0xb1c810>
140b1d848:     	cmpq	%rax, -0x80(%rdx,%r8)
140b1d84d:     	jne	0x140b1d810 <.text+0xb1c810>
140b1d84f:     	movq	%r13, %r10
140b1d852:     	shlq	$0x7, %r10
140b1d856:     	leaq	(%r10,%r13,8), %r11
140b1d85a:     	sarq	$0x3, %r11
140b1d85e:     	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
140b1d868:     	imulq	%r11, %r10
140b1d86c:     	leaq	-0x10(%r10), %r11
140b1d870:     	andq	%r9, %r11
140b1d873:     	movdqu	(%rdx,%r11), %xmm0
140b1d879:     	pcmpeqd	%xmm1, %xmm1
140b1d87d:     	pcmpeqb	%xmm1, %xmm0
140b1d881:     	pmovmskb	%xmm0, %r12d
140b1d886:     	movdqu	(%rdx,%r10), %xmm0
140b1d88c:     	pcmpeqb	%xmm1, %xmm0
140b1d890:     	pmovmskb	%xmm0, %r9d
140b1d895:     	testl	%r12d, %r12d
140b1d898:     	je	0x140b1d8a5 <.text+0xb1c8a5>
140b1d89a:     	bsrw	%r12w, %r12w
140b1d89f:     	xorl	$0xf, %r12d
140b1d8a3:     	jmp	0x140b1d8aa <.text+0xb1c8aa>
140b1d8a5:     	movw	$0x10, %r12w
140b1d8aa:     	addq	%rdx, %r8
140b1d8ad:     	orl	$0x10000, %r9d          # imm = 0x10000
140b1d8b4:     	tzcntl	%r9d, %r9d
140b1d8b9:     	addl	%r9d, %r12d
140b1d8bc:     	movb	$-0x80, %r9b
140b1d8bf:     	cmpw	$0xf, %r12w
140b1d8c4:     	ja	0x140b1d8d0 <.text+0xb1c8d0>
140b1d8c6:     	incq	0x90(%r15)
140b1d8cd:     	movb	$-0x1, %r9b
140b1d8d0:     	movb	%r9b, (%rdx,%r10)
140b1d8d4:     	movb	%r9b, 0x10(%rdx,%r11)
140b1d8d9:     	decq	0x98(%r15)
140b1d8e0:     	movq	-0x78(%r8), %rdx
140b1d8e4:     	cmpq	$0x2, %rdx
140b1d8e8:     	jne	0x140b1db14 <.text+0xb1cb14>
140b1d8ee:     	movq	0x8(%r14), %rdx
140b1d8f2:     	movq	(%rdx), %r12
140b1d8f5:     	leaq	0x1(%r12), %r13
140b1d8fa:     	movq	%r13, (%rdx)
140b1d8fd:     	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
140b1d907:     	movq	%rdx, 0x208(%rbp)
140b1d90e:     	leaq	0x2119b4b(%rip), %rdx   # 0x142c37460
140b1d915:     	movq	%rdx, 0x210(%rbp)
140b1d91c:     	movq	$0x4d, 0x218(%rbp)
140b1d927:     	movw	$0x100, 0x228(%rbp)     # imm = 0x100
140b1d930:     	movq	$0x1, 0x1d0(%rbp)
140b1d93b:     	movq	%rcx, 0x1d8(%rbp)
140b1d942:     	movq	%rax, 0x1e0(%rbp)
140b1d949:     	movq	$0x8, 0x1f8(%rbp)
140b1d954:     	movq	$0x8, 0x200(%rbp)
140b1d95f:     	movq	$0x0, 0x220(%rbp)
140b1d96a:     	movb	$0x1, 0x22a(%rbp)
140b1d971:     	movq	$0x0, 0x1e8(%rbp)
140b1d97c:     	leaq	0x80(%rbp), %rcx
140b1d983:     	leaq	0x1d0(%rbp), %r8
140b1d98a:     	movq	%r12, %rdx
140b1d98d:     	callq	0x140d3f870 <.text+0xd3e870>
140b1d992:     	cmpq	%r13, 0x10(%r15)
140b1d996:     	jae	0x140b1d9b1 <.text+0xb1c9b1>
140b1d998:     	movb	$0x1, 0x25f(%rbp)
140b1d99f:     	leaq	0x1fcb822(%rip), %r8    # 0x142ae91c8
140b1d9a6:     	movq	%r15, %rcx
140b1d9a9:     	movq	%r13, %rdx
140b1d9ac:     	callq	0x1412f4340 <.text+0x12f3340>
140b1d9b1:     	movq	0x8(%r15), %rbx
140b1d9b5:     	imulq	$0xd0, %r12, %r15
140b1d9bc:     	leaq	(%rbx,%r15), %rax
140b1d9c0:     	movq	%rax, 0x250(%rbp)
140b1d9c7:     	leaq	-0x50(%rbp), %rcx
140b1d9cb:     	leaq	0x80(%rbp), %rdx
140b1d9d2:     	movl	$0xd0, %r8d
140b1d9d8:     	callq	0x142923c00 <.text+0x2922c00>
140b1d9dd:     	cmpl	$0x2, (%rbx,%r15)
140b1d9e2:     	je	0x140b1d9f0 <.text+0xb1c9f0>
140b1d9e4:     	movq	0x250(%rbp), %rcx
140b1d9eb:     	callq	0x1405f0b50 <.text+0x5efb50>
140b1d9f0:     	leaq	-0x50(%rbp), %rdx
140b1d9f4:     	movl	$0xd0, %r8d
140b1d9fa:     	movq	0x250(%rbp), %rcx
140b1da01:     	callq	0x142923c00 <.text+0x2922c00>
140b1da06:     	movq	(%r14), %rcx
140b1da09:     	addq	$0x18, %rcx
140b1da0d:     	movabsq	$0x6d65e656c0a8d05c, %rdx # imm = 0x6D65E656C0A8D05C
140b1da17:     	movabsq	$-0x306abbe6b64e807b, %r8 # imm = 0xCF95441949B17F85
140b1da21:     	movq	%r12, %r9
140b1da24:     	callq	0x140082c20 <.text+0x81c20>
140b1da29:     	movups	0x1fcb6b0(%rip), %xmm0  # 0x142ae90e0
140b1da30:     	movaps	%xmm0, 0x90(%rbp)
140b1da37:     	movdqu	0x1fcb691(%rip), %xmm0  # 0x142ae90d0
140b1da3f:     	movdqa	%xmm0, 0x80(%rbp)
140b1da47:     	movb	$0x1, 0x25e(%rbp)
140b1da4e:     	leaq	0x230(%rbp), %rax
140b1da55:     	movq	%rax, 0x20(%rsp)
140b1da5a:     	movq	%r14, %rdx
140b1da5d:     	callq	0x140b1d680 <.text+0xb1c680>
140b1da62:     	movq	(%r14), %rax
140b1da65:     	movq	0x8(%rax), %rax
140b1da69:     	movaps	0x80(%rbp), %xmm0
140b1da70:     	movaps	0x90(%rbp), %xmm1
140b1da77:     	movaps	%xmm1, -0x40(%rbp)
140b1da7b:     	movaps	%xmm0, -0x50(%rbp)
140b1da7f:     	leaq	0x60(%rax,%r15), %rcx
140b1da84:     	leaq	0x80(%rax,%r15), %rdx
140b1da8c:     	movl	$0x20, %r8d
140b1da92:     	movl	$0x10, %r9d
140b1da98:     	movq	%rcx, 0x250(%rbp)
140b1da9f:     	callq	0x140051340 <.text+0x50340>
140b1daa4:     	movq	0x248(%rbp), %rbx
140b1daab:     	movdqa	-0x50(%rbp), %xmm0
140b1dab0:     	movdqa	-0x40(%rbp), %xmm1
140b1dab5:     	movq	0x250(%rbp), %rax
140b1dabc:     	movdqu	%xmm1, 0x10(%rax)
140b1dac1:     	movdqu	%xmm0, (%rax)
140b1dac5:     	movq	0x230(%rbp), %rdx
140b1dacc:     	testq	%rdx, %rdx
140b1dacf:     	je	0x140b1dae7 <.text+0xb1cae7>
140b1dad1:     	movq	0x238(%rbp), %rcx
140b1dad8:     	shlq	$0x3, %rdx
140b1dadc:     	movl	$0x8, %r8d
140b1dae2:     	callq	0x140613c20 <.text+0x612c20>
140b1dae7:     	cmpq	(%rsi), %rdi
140b1daea:     	jne	0x140b1db99 <.text+0xb1cb99>
140b1daf0:     	leaq	0x1fcbae9(%rip), %rdx   # 0x142ae95e0
140b1daf7:     	movq	%rsi, %rcx
140b1dafa:     	callq	0x140b81df0 <.text+0xb80df0>
140b1daff:     	movq	0x8(%rsi), %rax
140b1db03:     	movq	%r12, (%rax,%rdi,8)
140b1db07:     	leaq	0x1(%rdi), %rax
140b1db0b:     	movq	%rax, 0x10(%rsi)
140b1db0f:     	jmp	0x140b1dba9 <.text+0xb1cba9>
140b1db14:     	movups	-0x10(%r8), %xmm0
140b1db19:     	movups	%xmm0, 0x1c0(%rbp)
140b1db20:     	movups	-0x20(%r8), %xmm0
140b1db25:     	movups	%xmm0, 0x1b0(%rbp)
140b1db2c:     	movups	-0x30(%r8), %xmm0
140b1db31:     	movups	%xmm0, 0x1a0(%rbp)
140b1db38:     	movdqu	-0x70(%r8), %xmm0
140b1db3e:     	movdqu	-0x60(%r8), %xmm1
140b1db44:     	movdqu	-0x50(%r8), %xmm2
140b1db4a:     	movdqu	-0x40(%r8), %xmm3
140b1db50:     	movdqu	%xmm3, 0x190(%rbp)
140b1db58:     	movdqu	%xmm2, 0x180(%rbp)
140b1db60:     	movdqu	%xmm1, 0x170(%rbp)
140b1db68:     	movdqu	%xmm0, 0x160(%rbp)
140b1db70:     	movq	%rdx, 0x158(%rbp)
140b1db77:     	leaq	0x158(%rbp), %rcx
140b1db7e:     	movq	%r14, %rdx
140b1db81:     	callq	0x140d3fa40 <.text+0xd3ea40>
140b1db86:     	movq	%rax, %r12
140b1db89:     	movq	0x248(%rbp), %rbx
140b1db90:     	cmpq	(%rsi), %rdi
140b1db93:     	je	0x140b1daf0 <.text+0xb1caf0>
140b1db99:     	movq	%r12, (%rbx,%rdi,8)
140b1db9d:     	movq	%rdi, %rax
140b1dba0:     	incq	%rax
140b1dba3:     	movq	%rax, 0x10(%rsi)
140b1dba7:     	je	0x140b1dbad <.text+0xb1cbad>
140b1dba9:     	movq	%rdi, 0x10(%rsi)
140b1dbad:     	addq	$0x2e8, %rsp            # imm = 0x2E8
140b1dbb4:     	popq	%rbx
140b1dbb5:     	popq	%rdi
140b1dbb6:     	popq	%rsi
140b1dbb7:     	popq	%r12
140b1dbb9:     	popq	%r13
140b1dbbb:     	popq	%r14
140b1dbbd:     	popq	%r15
140b1dbbf:     	popq	%rbp
140b1dbc0:     	retq
140b1dbc1:     	nopw	%cs:(%rax,%rax)
140b1dbd0:     	movq	%rdx, 0x10(%rsp)
140b1dbd5:     	pushq	%rbp
140b1dbd6:     	pushq	%r15
140b1dbd8:     	pushq	%r14
140b1dbda:     	pushq	%r13
140b1dbdc:     	pushq	%r12
140b1dbde:     	pushq	%rsi
140b1dbdf:     	pushq	%rdi
140b1dbe0:     	pushq	%rbx
140b1dbe1:     	subq	$0x28, %rsp
140b1dbe5:     	leaq	0x80(%rdx), %rbp
140b1dbec:     	leaq	-0x50(%rbp), %rdx
140b1dbf0:     	movl	$0xd0, %r8d
140b1dbf6:     	movq	0x250(%rbp), %rcx
140b1dbfd:     	callq	0x142923c00 <.text+0x2922c00>
140b1dc02:     	movb	$0x0, 0x25f(%rbp)
140b1dc09:     	addq	$0x28, %rsp
140b1dc0d:     	popq	%rbx
140b1dc0e:     	popq	%rdi
140b1dc0f:     	popq	%rsi
140b1dc10:     	popq	%r12
140b1dc12:     	popq	%r13
140b1dc14:     	popq	%r14
140b1dc16:     	popq	%r15
140b1dc18:     	popq	%rbp
140b1dc19:     	retq
140b1dc1a:     	nopw	(%rax,%rax)
140b1dc20:     	movq	%rdx, 0x10(%rsp)
140b1dc25:     	pushq	%rbp
140b1dc26:     	pushq	%r15
140b1dc28:     	pushq	%r14
140b1dc2a:     	pushq	%r13
140b1dc2c:     	pushq	%r12
140b1dc2e:     	pushq	%rsi
140b1dc2f:     	pushq	%rdi
140b1dc30:     	pushq	%rbx
140b1dc31:     	subq	$0x28, %rsp
140b1dc35:     	leaq	0x80(%rdx), %rbp
140b1dc3c:     	cmpb	$0x0, 0x25f(%rbp)
140b1dc43:     	je	0x140b1dc51 <.text+0xb1cc51>
140b1dc45:     	leaq	0x80(%rbp), %rcx
140b1dc4c:     	callq	0x1405f0b50 <.text+0x5efb50>
140b1dc51:     	nop
140b1dc52:     	addq	$0x28, %rsp
140b1dc56:     	popq	%rbx
140b1dc57:     	popq	%rdi
140b1dc58:     	popq	%rsi
140b1dc59:     	popq	%r12
140b1dc5b:     	popq	%r13
140b1dc5d:     	popq	%r14
140b1dc5f:     	popq	%r15
140b1dc61:     	popq	%rbp
140b1dc62:     	retq
140b1dc63:     	nopw	%cs:(%rax,%rax)
140b1dc70:     	movq	%rdx, 0x10(%rsp)
140b1dc75:     	pushq	%rbp
140b1dc76:     	pushq	%r15
140b1dc78:     	pushq	%r14
140b1dc7a:     	pushq	%r13
140b1dc7c:     	pushq	%r12
140b1dc7e:     	pushq	%rsi
140b1dc7f:     	pushq	%rdi
140b1dc80:     	pushq	%rbx
140b1dc81:     	subq	$0x28, %rsp
140b1dc85:     	leaq	0x80(%rdx), %rbp
140b1dc8c:     	movdqa	-0x50(%rbp), %xmm0
140b1dc91:     	movdqa	-0x40(%rbp), %xmm1
140b1dc96:     	movq	0x250(%rbp), %rax
140b1dc9d:     	movdqu	%xmm1, 0x10(%rax)
140b1dca2:     	movdqu	%xmm0, (%rax)
140b1dca6:     	movb	$0x0, 0x25e(%rbp)
140b1dcad:     	addq	$0x28, %rsp
140b1dcb1:     	popq	%rbx
140b1dcb2:     	popq	%rdi
140b1dcb3:     	popq	%rsi
140b1dcb4:     	popq	%r12
140b1dcb6:     	popq	%r13
140b1dcb8:     	popq	%r14
140b1dcba:     	popq	%r15
140b1dcbc:     	popq	%rbp
140b1dcbd:     	retq
140b1dcbe:     	nop
140b1dcc0:     	movq	%rdx, 0x10(%rsp)
140b1dcc5:     	pushq	%rbp
140b1dcc6:     	pushq	%r15
140b1dcc8:     	pushq	%r14
140b1dcca:     	pushq	%r13
140b1dccc:     	pushq	%r12
140b1dcce:     	pushq	%rsi
140b1dccf:     	pushq	%rdi
140b1dcd0:     	pushq	%rbx
140b1dcd1:     	subq	$0x28, %rsp
140b1dcd5:     	leaq	0x80(%rdx), %rbp
140b1dcdc:     	cmpb	$0x0, 0x25e(%rbp)
140b1dce3:     	je	0x140b1dd04 <.text+0xb1cd04>
140b1dce5:     	leaq	0xa0(%rbp), %rdx
140b1dcec:     	leaq	0x80(%rbp), %rcx
140b1dcf3:     	movl	$0x20, %r8d
140b1dcf9:     	movl	$0x10, %r9d
140b1dcff:     	callq	0x140051340 <.text+0x50340>
140b1dd04:     	nop
140b1dd05:     	addq	$0x28, %rsp
140b1dd09:     	popq	%rbx
140b1dd0a:     	popq	%rdi
140b1dd0b:     	popq	%rsi
140b1dd0c:     	popq	%r12
140b1dd0e:     	popq	%r13
140b1dd10:     	popq	%r14
140b1dd12:     	popq	%r15
140b1dd14:     	popq	%rbp
140b1dd15:     	retq
140b1dd16:     	nopw	%cs:(%rax,%rax)
140b1dd20:     	movq	%rdx, 0x10(%rsp)
140b1dd25:     	pushq	%rbp
140b1dd26:     	pushq	%r15
140b1dd28:     	pushq	%r14
140b1dd2a:     	pushq	%r13
140b1dd2c:     	pushq	%r12
140b1dd2e:     	pushq	%rsi
140b1dd2f:     	pushq	%rdi
140b1dd30:     	pushq	%rbx
140b1dd31:     	subq	$0x28, %rsp
140b1dd35:     	leaq	0x80(%rdx), %rbp
140b1dd3c:     	movq	0x230(%rbp), %rdx
140b1dd43:     	testq	%rdx, %rdx
140b1dd46:     	je	0x140b1dd5e <.text+0xb1cd5e>
140b1dd48:     	movq	0x238(%rbp), %rcx
140b1dd4f:     	shlq	$0x3, %rdx
140b1dd53:     	movl	$0x8, %r8d
140b1dd59:     	callq	0x140613c20 <.text+0x612c20>
140b1dd5e:     	nop
140b1dd5f:     	addq	$0x28, %rsp
140b1dd63:     	popq	%rbx
140b1dd64:     	popq	%rdi
140b1dd65:     	popq	%rsi
140b1dd66:     	popq	%r12
140b1dd68:     	popq	%r13
140b1dd6a:     	popq	%r14
140b1dd6c:     	popq	%r15
140b1dd6e:     	popq	%rbp
140b1dd6f:     	retq
