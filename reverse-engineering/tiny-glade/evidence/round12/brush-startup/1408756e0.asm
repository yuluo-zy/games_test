
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408756e0:     	pushq	%rbp
1408756e1:     	pushq	%r15
1408756e3:     	pushq	%r14
1408756e5:     	pushq	%r13
1408756e7:     	pushq	%r12
1408756e9:     	pushq	%rsi
1408756ea:     	pushq	%rdi
1408756eb:     	pushq	%rbx
1408756ec:     	subq	$0x308, %rsp            # imm = 0x308
1408756f3:     	leaq	0x80(%rsp), %rbp
1408756fb:     	movdqa	%xmm6, 0x270(%rbp)
140875703:     	movq	$-0x2, 0x268(%rbp)
14087570e:     	movq	%rdx, %r14
140875711:     	movq	0x2f0(%rbp), %r13
140875718:     	movq	(%rdx), %r12
14087571b:     	movq	0x8(%r13), %rdx
14087571f:     	movq	0x10(%r13), %rdi
140875723:     	movq	%r12, %rcx
140875726:     	movq	%rdx, 0x230(%rbp)
14087572d:     	movq	%rdi, %r8
140875730:     	callq	0x140d40d70 <.text+0xd3fd70>
140875735:     	movq	$0x0, 0x240(%rbp)
140875740:     	movq	$0x8, 0x248(%rbp)
14087574b:     	movq	$0x0, 0x250(%rbp)
140875756:     	cmpq	$0x0, 0x30(%r12)
14087575c:     	movabsq	$0xd31c0cda19bce31, %rsi # imm = 0xD31C0CDA19BCE31
140875766:     	movabsq	$-0x3a25d7f8e16b5c38, %rbx # imm = 0xC5DA28071E94A3C8
140875770:     	je	0x140875814 <.text+0x874814>
140875776:     	movq	0x18(%r12), %rax
14087577b:     	movq	0x20(%r12), %rcx
140875780:     	movq	%rcx, %rdx
140875783:     	andq	%rsi, %rdx
140875786:     	xorl	%r8d, %r8d
140875789:     	movdqa	0x20b5c7f(%rip), %xmm0  # 0x14292b410
140875791:     	pcmpeqd	%xmm1, %xmm1
140875795:     	movdqu	(%rax,%rdx), %xmm2
14087579a:     	movdqa	%xmm2, %xmm3
14087579e:     	pcmpeqb	%xmm0, %xmm3
1408757a2:     	pmovmskb	%xmm3, %r9d
1408757a7:     	testl	%r9d, %r9d
1408757aa:     	jne	0x1408757dd <.text+0x8747dd>
1408757ac:     	pcmpeqb	%xmm1, %xmm2
1408757b0:     	pmovmskb	%xmm2, %r9d
1408757b5:     	testl	%r9d, %r9d
1408757b8:     	jne	0x140875814 <.text+0x874814>
1408757ba:     	addq	%r8, %rdx
1408757bd:     	addq	$0x10, %rdx
1408757c1:     	addq	$0x10, %r8
1408757c5:     	andq	%rcx, %rdx
1408757c8:     	jmp	0x140875795 <.text+0x874795>
1408757ca:     	nopw	(%rax,%rax)
1408757d0:     	leal	-0x1(%r9), %r10d
1408757d4:     	andw	%r9w, %r10w
1408757d8:     	movl	%r10d, %r9d
1408757db:     	je	0x1408757ac <.text+0x8747ac>
1408757dd:     	tzcntl	%r9d, %r10d
1408757e2:     	addq	%rdx, %r10
1408757e5:     	andq	%rcx, %r10
1408757e8:     	negq	%r10
1408757eb:     	leaq	(%r10,%r10,2), %r10
1408757ef:     	cmpq	%rbx, -0x18(%rax,%r10,8)
1408757f4:     	jne	0x1408757d0 <.text+0x8747d0>
1408757f6:     	cmpq	%rsi, -0x10(%rax,%r10,8)
1408757fb:     	jne	0x1408757d0 <.text+0x8747d0>
1408757fd:     	leaq	(%rax,%r10,8), %rax
140875801:     	movq	-0x8(%rax), %r15
140875805:     	cmpq	(%r13), %rdi
140875809:     	je	0x140875c15 <.text+0x874c15>
14087580f:     	jmp	0x140875cb1 <.text+0x874cb1>
140875814:     	movzbl	0x60(%r12), %eax
14087581a:     	movq	0x80(%r12), %rax
140875822:     	movq	0x88(%r12), %rdx
14087582a:     	movq	%rdx, %r8
14087582d:     	andq	%rsi, %r8
140875830:     	xorl	%r9d, %r9d
140875833:     	movdqa	0x20b5bd5(%rip), %xmm6  # 0x14292b410
14087583b:     	pcmpeqd	%xmm0, %xmm0
14087583f:     	movdqu	(%rax,%r8), %xmm1
140875845:     	movdqa	%xmm1, %xmm2
140875849:     	pcmpeqb	%xmm6, %xmm2
14087584d:     	pmovmskb	%xmm2, %r10d
140875852:     	testl	%r10d, %r10d
140875855:     	jne	0x14087588d <.text+0x87488d>
140875857:     	pcmpeqb	%xmm0, %xmm1
14087585b:     	pmovmskb	%xmm1, %ecx
14087585f:     	testl	%ecx, %ecx
140875861:     	jne	0x14087595a <.text+0x87495a>
140875867:     	addq	%r9, %r8
14087586a:     	addq	$0x10, %r8
14087586e:     	addq	$0x10, %r9
140875872:     	andq	%rdx, %r8
140875875:     	jmp	0x14087583f <.text+0x87483f>
140875877:     	nopw	(%rax,%rax)
140875880:     	leal	-0x1(%r10), %ecx
140875884:     	andw	%r10w, %cx
140875888:     	movl	%ecx, %r10d
14087588b:     	je	0x140875857 <.text+0x874857>
14087588d:     	tzcntl	%r10d, %r11d
140875892:     	addq	%r8, %r11
140875895:     	andq	%rdx, %r11
140875898:     	movq	%r11, %rcx
14087589b:     	negq	%rcx
14087589e:     	leaq	(,%r11,8), %r15
1408758a6:     	shlq	$0x7, %rcx
1408758aa:     	subq	%r15, %rcx
1408758ad:     	cmpq	%rbx, -0x88(%rax,%rcx)
1408758b5:     	jne	0x140875880 <.text+0x874880>
1408758b7:     	cmpq	%rsi, -0x80(%rax,%rcx)
1408758bc:     	jne	0x140875880 <.text+0x874880>
1408758be:     	movq	%r11, %r8
1408758c1:     	shlq	$0x7, %r8
1408758c5:     	leaq	(%r8,%r11,8), %r9
1408758c9:     	sarq	$0x3, %r9
1408758cd:     	movabsq	$-0xf0f0f0f0f0f0f0f, %r8 # imm = 0xF0F0F0F0F0F0F0F1
1408758d7:     	imulq	%r9, %r8
1408758db:     	leaq	-0x10(%r8), %r9
1408758df:     	andq	%rdx, %r9
1408758e2:     	movdqu	(%rax,%r9), %xmm0
1408758e8:     	pcmpeqd	%xmm1, %xmm1
1408758ec:     	pcmpeqb	%xmm1, %xmm0
1408758f0:     	pmovmskb	%xmm0, %r10d
1408758f5:     	movdqu	(%rax,%r8), %xmm0
1408758fb:     	pcmpeqb	%xmm1, %xmm0
1408758ff:     	pmovmskb	%xmm0, %edx
140875903:     	testl	%r10d, %r10d
140875906:     	je	0x140875913 <.text+0x874913>
140875908:     	bsrw	%r10w, %r10w
14087590d:     	xorl	$0xf, %r10d
140875911:     	jmp	0x140875918 <.text+0x874918>
140875913:     	movw	$0x10, %r10w
140875918:     	addq	%rax, %rcx
14087591b:     	orl	$0x10000, %edx          # imm = 0x10000
140875921:     	tzcntl	%edx, %edx
140875925:     	addl	%edx, %r10d
140875928:     	movb	$-0x80, %dl
14087592a:     	cmpw	$0xf, %r10w
14087592f:     	ja	0x14087593b <.text+0x87493b>
140875931:     	incq	0x90(%r12)
140875939:     	movb	$-0x1, %dl
14087593b:     	movb	%dl, (%rax,%r8)
14087593f:     	movb	%dl, 0x10(%rax,%r9)
140875944:     	decq	0x98(%r12)
14087594c:     	movq	-0x78(%rcx), %rax
140875950:     	cmpq	$0x2, %rax
140875954:     	jne	0x140875c39 <.text+0x874c39>
14087595a:     	movq	0x8(%r14), %rax
14087595e:     	movq	(%rax), %r15
140875961:     	leaq	0x1(%r15), %r13
140875965:     	movq	%r13, (%rax)
140875968:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140875972:     	movq	%rax, 0x208(%rbp)
140875979:     	leaq	0x23c1b62(%rip), %rax   # 0x142c374e2
140875980:     	movq	%rax, 0x210(%rbp)
140875987:     	movq	$0x33, 0x218(%rbp)
140875992:     	movw	$0x100, 0x228(%rbp)     # imm = 0x100
14087599b:     	movq	$0x1, 0x1d0(%rbp)
1408759a6:     	movq	%rbx, 0x1d8(%rbp)
1408759ad:     	movq	%rsi, 0x1e0(%rbp)
1408759b4:     	movq	$0x1, 0x1f8(%rbp)
1408759bf:     	movq	$0x0, 0x200(%rbp)
1408759ca:     	movq	$0x0, 0x220(%rbp)
1408759d5:     	movb	$0x1, 0x22a(%rbp)
1408759dc:     	movq	$0x0, 0x1e8(%rbp)
1408759e7:     	leaq	0x80(%rbp), %rcx
1408759ee:     	leaq	0x1d0(%rbp), %r8
1408759f5:     	movq	%r15, %rdx
1408759f8:     	callq	0x140d3f870 <.text+0xd3e870>
1408759fd:     	cmpq	%r13, 0x10(%r12)
140875a02:     	jae	0x140875a1d <.text+0x874a1d>
140875a04:     	movb	$0x1, 0x267(%rbp)
140875a0b:     	leaq	0x22226be(%rip), %r8    # 0x142a980d0
140875a12:     	movq	%r12, %rcx
140875a15:     	movq	%r13, %rdx
140875a18:     	callq	0x1412f4340 <.text+0x12f3340>
140875a1d:     	movq	0x8(%r12), %r12
140875a22:     	imulq	$0xd0, %r15, %r13
140875a29:     	leaq	(%r12,%r13), %rax
140875a2d:     	movq	%rax, 0x258(%rbp)
140875a34:     	leaq	-0x50(%rbp), %rcx
140875a38:     	leaq	0x80(%rbp), %rdx
140875a3f:     	movl	$0xd0, %r8d
140875a45:     	callq	0x142923c00 <.text+0x2922c00>
140875a4a:     	cmpl	$0x2, (%r12,%r13)
140875a4f:     	je	0x140875a5d <.text+0x874a5d>
140875a51:     	movq	0x258(%rbp), %rcx
140875a58:     	callq	0x1405f0b50 <.text+0x5efb50>
140875a5d:     	leaq	-0x50(%rbp), %rdx
140875a61:     	movl	$0xd0, %r8d
140875a67:     	movq	0x258(%rbp), %rcx
140875a6e:     	callq	0x142923c00 <.text+0x2922c00>
140875a73:     	movq	(%r14), %r12
140875a76:     	cmpq	$0x0, 0x28(%r12)
140875a7c:     	je	0x140875d4c <.text+0x874d4c>
140875a82:     	movq	%r13, 0x258(%rbp)
140875a89:     	movq	0x18(%r12), %rax
140875a8e:     	movq	%r12, 0x238(%rbp)
140875a95:     	movq	0x20(%r12), %rcx
140875a9a:     	xorl	%r8d, %r8d
140875a9d:     	pcmpeqd	%xmm0, %xmm0
140875aa1:     	movq	%rsi, %r9
140875aa4:     	xorl	%r10d, %r10d
140875aa7:     	andq	%rcx, %r9
140875aaa:     	movdqu	(%rax,%r9), %xmm1
140875ab0:     	movdqa	%xmm1, %xmm2
140875ab4:     	pcmpeqb	%xmm6, %xmm2
140875ab8:     	pmovmskb	%xmm2, %r11d
140875abd:     	testl	%r11d, %r11d
140875ac0:     	jne	0x140875b1d <.text+0x874b1d>
140875ac2:     	cmpq	$0x1, %r10
140875ac6:     	movl	$0x1, %r10d
140875acc:     	je	0x140875ae6 <.text+0x874ae6>
140875ace:     	pmovmskb	%xmm1, %edx
140875ad2:     	xorl	%r10d, %r10d
140875ad5:     	testl	%edx, %edx
140875ad7:     	setne	%r10b
140875adb:     	tzcntl	%edx, %r12d
140875ae0:     	addq	%r9, %r12
140875ae3:     	andq	%rcx, %r12
140875ae6:     	pcmpeqb	%xmm0, %xmm1
140875aea:     	pmovmskb	%xmm1, %edx
140875aee:     	testl	%edx, %edx
140875af0:     	jne	0x140875ce7 <.text+0x874ce7>
140875af6:     	addq	%r8, %r9
140875af9:     	addq	$0x10, %r9
140875afd:     	addq	$0x10, %r8
140875b01:     	jmp	0x140875aa7 <.text+0x874aa7>
140875b03:     	nopw	%cs:(%rax,%rax)
140875b10:     	leal	-0x1(%r11), %edx
140875b14:     	andw	%r11w, %dx
140875b18:     	movl	%edx, %r11d
140875b1b:     	je	0x140875ac2 <.text+0x874ac2>
140875b1d:     	tzcntl	%r11d, %edx
140875b22:     	addq	%r9, %rdx
140875b25:     	andq	%rcx, %rdx
140875b28:     	negq	%rdx
140875b2b:     	leaq	(%rdx,%rdx,2), %r13
140875b2f:     	cmpq	%rbx, -0x18(%rax,%r13,8)
140875b34:     	jne	0x140875b10 <.text+0x874b10>
140875b36:     	cmpq	%rsi, -0x10(%rax,%r13,8)
140875b3b:     	jne	0x140875b10 <.text+0x874b10>
140875b3d:     	leaq	(%rdx,%rdx,2), %rcx
140875b41:     	movq	%r15, -0x8(%rax,%rcx,8)
140875b46:     	movups	0x2222513(%rip), %xmm0  # 0x142a98060
140875b4d:     	movaps	%xmm0, 0x90(%rbp)
140875b54:     	movdqu	0x22224f4(%rip), %xmm0  # 0x142a98050
140875b5c:     	movdqa	%xmm0, 0x80(%rbp)
140875b64:     	movb	$0x1, 0x266(%rbp)
140875b6b:     	leaq	0x240(%rbp), %rax
140875b72:     	movq	%rax, 0x20(%rsp)
140875b77:     	movq	%r14, %rdx
140875b7a:     	callq	0x1408756e0 <.text+0x8746e0>
140875b7f:     	movq	(%r14), %rax
140875b82:     	movq	0x8(%rax), %rax
140875b86:     	movaps	0x80(%rbp), %xmm0
140875b8d:     	movaps	0x90(%rbp), %xmm1
140875b94:     	movaps	%xmm1, -0x40(%rbp)
140875b98:     	movaps	%xmm0, -0x50(%rbp)
140875b9c:     	movq	0x258(%rbp), %rdx
140875ba3:     	leaq	0x60(%rax,%rdx), %rcx
140875ba8:     	leaq	0x80(%rax,%rdx), %rdx
140875bb0:     	movl	$0x20, %r8d
140875bb6:     	movl	$0x10, %r9d
140875bbc:     	movq	%rcx, 0x258(%rbp)
140875bc3:     	callq	0x140051340 <.text+0x50340>
140875bc8:     	movq	0x2f0(%rbp), %r13
140875bcf:     	movdqa	-0x50(%rbp), %xmm0
140875bd4:     	movdqa	-0x40(%rbp), %xmm1
140875bd9:     	movq	0x258(%rbp), %rax
140875be0:     	movdqu	%xmm1, 0x10(%rax)
140875be5:     	movdqu	%xmm0, (%rax)
140875be9:     	movq	0x240(%rbp), %rdx
140875bf0:     	testq	%rdx, %rdx
140875bf3:     	je	0x140875c0b <.text+0x874c0b>
140875bf5:     	movq	0x248(%rbp), %rcx
140875bfc:     	shlq	$0x3, %rdx
140875c00:     	movl	$0x8, %r8d
140875c06:     	callq	0x140613c20 <.text+0x612c20>
140875c0b:     	cmpq	(%r13), %rdi
140875c0f:     	jne	0x140875cb1 <.text+0x874cb1>
140875c15:     	leaq	0x222270c(%rip), %rdx   # 0x142a98328
140875c1c:     	movq	%r13, %rcx
140875c1f:     	callq	0x140b81df0 <.text+0xb80df0>
140875c24:     	movq	0x8(%r13), %rax
140875c28:     	movq	%r15, (%rax,%rdi,8)
140875c2c:     	leaq	0x1(%rdi), %rax
140875c30:     	movq	%rax, 0x10(%r13)
140875c34:     	jmp	0x140875cc8 <.text+0x874cc8>
140875c39:     	movups	-0x10(%rcx), %xmm0
140875c3d:     	movups	%xmm0, 0x1c0(%rbp)
140875c44:     	movups	-0x20(%rcx), %xmm0
140875c48:     	movups	%xmm0, 0x1b0(%rbp)
140875c4f:     	movups	-0x30(%rcx), %xmm0
140875c53:     	movups	%xmm0, 0x1a0(%rbp)
140875c5a:     	movdqu	-0x70(%rcx), %xmm0
140875c5f:     	movdqu	-0x60(%rcx), %xmm1
140875c64:     	movdqu	-0x50(%rcx), %xmm2
140875c69:     	movdqu	-0x40(%rcx), %xmm3
140875c6e:     	movdqu	%xmm3, 0x190(%rbp)
140875c76:     	movdqu	%xmm2, 0x180(%rbp)
140875c7e:     	movdqu	%xmm1, 0x170(%rbp)
140875c86:     	movdqu	%xmm0, 0x160(%rbp)
140875c8e:     	movq	%rax, 0x158(%rbp)
140875c95:     	leaq	0x158(%rbp), %rcx
140875c9c:     	movq	%r14, %rdx
140875c9f:     	callq	0x140d3fa40 <.text+0xd3ea40>
140875ca4:     	movq	%rax, %r15
140875ca7:     	cmpq	(%r13), %rdi
140875cab:     	je	0x140875c15 <.text+0x874c15>
140875cb1:     	movq	0x230(%rbp), %rax
140875cb8:     	movq	%r15, (%rax,%rdi,8)
140875cbc:     	movq	%rdi, %rax
140875cbf:     	incq	%rax
140875cc2:     	movq	%rax, 0x10(%r13)
140875cc6:     	je	0x140875ccc <.text+0x874ccc>
140875cc8:     	movq	%rdi, 0x10(%r13)
140875ccc:     	movaps	0x270(%rbp), %xmm6
140875cd3:     	addq	$0x308, %rsp            # imm = 0x308
140875cda:     	popq	%rbx
140875cdb:     	popq	%rdi
140875cdc:     	popq	%rsi
140875cdd:     	popq	%r12
140875cdf:     	popq	%r13
140875ce1:     	popq	%r14
140875ce3:     	popq	%r15
140875ce5:     	popq	%rbp
140875ce6:     	retq
140875ce7:     	movzbl	(%rax,%r12), %edx
140875cec:     	testb	%dl, %dl
140875cee:     	js	0x140875d02 <.text+0x874d02>
140875cf0:     	movdqa	(%rax), %xmm0
140875cf4:     	pmovmskb	%xmm0, %edx
140875cf8:     	tzcntl	%edx, %r12d
140875cfd:     	movzbl	(%rax,%r12), %edx
140875d02:     	andb	$0x1, %dl
140875d05:     	movzbl	%dl, %edx
140875d08:     	movq	0x238(%rbp), %r8
140875d0f:     	subq	%rdx, 0x28(%r8)
140875d13:     	leaq	-0x10(%r12), %rdx
140875d18:     	andq	%rcx, %rdx
140875d1b:     	movb	$0x6, (%rax,%r12)
140875d20:     	movb	$0x6, 0x10(%rax,%rdx)
140875d25:     	incq	0x30(%r8)
140875d29:     	negq	%r12
140875d2c:     	leaq	(%r12,%r12,2), %rcx
140875d30:     	movq	%rbx, -0x18(%rax,%rcx,8)
140875d35:     	movabsq	$0xd31c0cda19bce31, %rdx # imm = 0xD31C0CDA19BCE31
140875d3f:     	movq	%rdx, -0x10(%rax,%rcx,8)
140875d44:     	movq	%r12, %rdx
140875d47:     	jmp	0x140875b3d <.text+0x874b3d>
140875d4c:     	leaq	0x18(%r12), %rcx
140875d51:     	leaq	0x38(%r12), %r8
140875d56:     	movl	$0x1, %edx
140875d5b:     	movb	$0x1, %r9b
140875d5e:     	callq	0x1428d1fc0 <.text+0x28d0fc0>
140875d63:     	jmp	0x140875a82 <.text+0x874a82>
140875d68:     	nopl	(%rax,%rax)
140875d70:     	movq	%rdx, 0x10(%rsp)
140875d75:     	pushq	%rbp
140875d76:     	pushq	%r15
140875d78:     	pushq	%r14
140875d7a:     	pushq	%r13
140875d7c:     	pushq	%r12
140875d7e:     	pushq	%rsi
140875d7f:     	pushq	%rdi
140875d80:     	pushq	%rbx
140875d81:     	subq	$0x38, %rsp
140875d85:     	leaq	0x80(%rdx), %rbp
140875d8c:     	movdqa	%xmm6, 0x20(%rsp)
140875d92:     	leaq	-0x50(%rbp), %rdx
140875d96:     	movl	$0xd0, %r8d
140875d9c:     	movq	0x258(%rbp), %rcx
140875da3:     	callq	0x142923c00 <.text+0x2922c00>
140875da8:     	movb	$0x0, 0x267(%rbp)
140875daf:     	movdqa	0x20(%rsp), %xmm6
140875db5:     	addq	$0x38, %rsp
140875db9:     	popq	%rbx
140875dba:     	popq	%rdi
140875dbb:     	popq	%rsi
140875dbc:     	popq	%r12
140875dbe:     	popq	%r13
140875dc0:     	popq	%r14
140875dc2:     	popq	%r15
140875dc4:     	popq	%rbp
140875dc5:     	retq
140875dc6:     	nopw	%cs:(%rax,%rax)
140875dd0:     	movq	%rdx, 0x10(%rsp)
140875dd5:     	pushq	%rbp
140875dd6:     	pushq	%r15
140875dd8:     	pushq	%r14
140875dda:     	pushq	%r13
140875ddc:     	pushq	%r12
140875dde:     	pushq	%rsi
140875ddf:     	pushq	%rdi
140875de0:     	pushq	%rbx
140875de1:     	subq	$0x38, %rsp
140875de5:     	leaq	0x80(%rdx), %rbp
140875dec:     	movdqa	%xmm6, 0x20(%rsp)
140875df2:     	cmpb	$0x0, 0x267(%rbp)
140875df9:     	je	0x140875e07 <.text+0x874e07>
140875dfb:     	leaq	0x80(%rbp), %rcx
140875e02:     	callq	0x1405f0b50 <.text+0x5efb50>
140875e07:     	movdqa	0x20(%rsp), %xmm6
140875e0d:     	addq	$0x38, %rsp
140875e11:     	popq	%rbx
140875e12:     	popq	%rdi
140875e13:     	popq	%rsi
140875e14:     	popq	%r12
140875e16:     	popq	%r13
140875e18:     	popq	%r14
140875e1a:     	popq	%r15
140875e1c:     	popq	%rbp
140875e1d:     	retq
140875e1e:     	nop
140875e20:     	movq	%rdx, 0x10(%rsp)
140875e25:     	pushq	%rbp
140875e26:     	pushq	%r15
140875e28:     	pushq	%r14
140875e2a:     	pushq	%r13
140875e2c:     	pushq	%r12
140875e2e:     	pushq	%rsi
140875e2f:     	pushq	%rdi
140875e30:     	pushq	%rbx
140875e31:     	subq	$0x38, %rsp
140875e35:     	leaq	0x80(%rdx), %rbp
140875e3c:     	movdqa	%xmm6, 0x20(%rsp)
140875e42:     	movdqa	-0x50(%rbp), %xmm0
140875e47:     	movdqa	-0x40(%rbp), %xmm1
140875e4c:     	movq	0x258(%rbp), %rax
140875e53:     	movdqu	%xmm1, 0x10(%rax)
140875e58:     	movdqu	%xmm0, (%rax)
140875e5c:     	movb	$0x0, 0x266(%rbp)
140875e63:     	movdqa	0x20(%rsp), %xmm6
140875e69:     	addq	$0x38, %rsp
140875e6d:     	popq	%rbx
140875e6e:     	popq	%rdi
140875e6f:     	popq	%rsi
140875e70:     	popq	%r12
140875e72:     	popq	%r13
140875e74:     	popq	%r14
140875e76:     	popq	%r15
140875e78:     	popq	%rbp
140875e79:     	retq
140875e7a:     	nopw	(%rax,%rax)
140875e80:     	movq	%rdx, 0x10(%rsp)
140875e85:     	pushq	%rbp
140875e86:     	pushq	%r15
140875e88:     	pushq	%r14
140875e8a:     	pushq	%r13
140875e8c:     	pushq	%r12
140875e8e:     	pushq	%rsi
140875e8f:     	pushq	%rdi
140875e90:     	pushq	%rbx
140875e91:     	subq	$0x38, %rsp
140875e95:     	leaq	0x80(%rdx), %rbp
140875e9c:     	movdqa	%xmm6, 0x20(%rsp)
140875ea2:     	cmpb	$0x0, 0x266(%rbp)
140875ea9:     	je	0x140875eca <.text+0x874eca>
140875eab:     	leaq	0xa0(%rbp), %rdx
140875eb2:     	leaq	0x80(%rbp), %rcx
140875eb9:     	movl	$0x20, %r8d
140875ebf:     	movl	$0x10, %r9d
140875ec5:     	callq	0x140051340 <.text+0x50340>
140875eca:     	movdqa	0x20(%rsp), %xmm6
140875ed0:     	addq	$0x38, %rsp
140875ed4:     	popq	%rbx
140875ed5:     	popq	%rdi
140875ed6:     	popq	%rsi
140875ed7:     	popq	%r12
140875ed9:     	popq	%r13
140875edb:     	popq	%r14
140875edd:     	popq	%r15
140875edf:     	popq	%rbp
140875ee0:     	retq
140875ee1:     	nopw	%cs:(%rax,%rax)
140875ef0:     	movq	%rdx, 0x10(%rsp)
140875ef5:     	pushq	%rbp
140875ef6:     	pushq	%r15
140875ef8:     	pushq	%r14
140875efa:     	pushq	%r13
140875efc:     	pushq	%r12
140875efe:     	pushq	%rsi
140875eff:     	pushq	%rdi
140875f00:     	pushq	%rbx
140875f01:     	subq	$0x38, %rsp
140875f05:     	leaq	0x80(%rdx), %rbp
140875f0c:     	movdqa	%xmm6, 0x20(%rsp)
140875f12:     	movq	0x240(%rbp), %rdx
140875f19:     	testq	%rdx, %rdx
140875f1c:     	je	0x140875f34 <.text+0x874f34>
140875f1e:     	movq	0x248(%rbp), %rcx
140875f25:     	shlq	$0x3, %rdx
140875f29:     	movl	$0x8, %r8d
140875f2f:     	callq	0x140613c20 <.text+0x612c20>
140875f34:     	movaps	0x20(%rsp), %xmm6
140875f39:     	addq	$0x38, %rsp
140875f3d:     	popq	%rbx
140875f3e:     	popq	%rdi
140875f3f:     	popq	%rsi
140875f40:     	popq	%r12
140875f42:     	popq	%r13
140875f44:     	popq	%r14
140875f46:     	popq	%r15
140875f48:     	popq	%rbp
140875f49:     	retq
