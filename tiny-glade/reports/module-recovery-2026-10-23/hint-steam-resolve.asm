
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14227e5e0:     	pushq	%rbp
14227e5e1:     	pushq	%r15
14227e5e3:     	pushq	%r14
14227e5e5:     	pushq	%r13
14227e5e7:     	pushq	%r12
14227e5e9:     	pushq	%rsi
14227e5ea:     	pushq	%rdi
14227e5eb:     	pushq	%rbx
14227e5ec:     	subq	$0x138, %rsp            # imm = 0x138
14227e5f3:     	leaq	0x80(%rsp), %rbp
14227e5fb:     	movq	$-0x2, 0xb0(%rbp)
14227e606:     	movq	%r9, %rbx
14227e609:     	movq	%r8, %rdi
14227e60c:     	movl	(%rdx), %r8d
14227e60f:     	leal	-0xc4(%r8), %r14d
14227e616:     	cmpl	$0x9, %r14d
14227e61a:     	movl	$0x4, %r13d
14227e620:     	cmovael	%r13d, %r14d
14227e624:     	movq	%rcx, %rsi
14227e627:     	leaq	0xdba60e(%rip), %rax    # 0x143038c3c
14227e62e:     	movslq	(%rax,%r14,4), %rcx
14227e632:     	addq	%rax, %rcx
14227e635:     	jmpq	*%rcx
14227e637:     	movups	(%rdx), %xmm0
14227e63a:     	movups	0x10(%rdx), %xmm1
14227e63e:     	movups	0x20(%rdx), %xmm2
14227e642:     	movups	%xmm2, 0x20(%rsi)
14227e646:     	movups	%xmm1, 0x10(%rsi)
14227e64a:     	movups	%xmm0, (%rsi)
14227e64d:     	jmp	0x14227ed6c <.text+0x227dd6c>
14227e652:     	movq	%r14, 0x78(%rbp)
14227e656:     	movq	%rsi, 0x70(%rbp)
14227e65a:     	movq	%r8, 0x68(%rbp)
14227e65e:     	movups	0x8(%rdx), %xmm0
14227e662:     	movaps	%xmm0, 0x80(%rbp)
14227e669:     	movq	%rdx, 0x98(%rbp)
14227e670:     	movq	0x18(%rdx), %rax
14227e674:     	movq	%rax, 0x90(%rbp)
14227e67b:     	movq	0x88(%rbp), %r14
14227e682:     	leaq	(%rax,%rax,2), %rsi
14227e686:     	shlq	$0x4, %rsi
14227e68a:     	addq	%r14, %rsi
14227e68d:     	leaq	-0x30(%rbp), %r15
14227e691:     	leaq	0xdba5e8(%rip), %r12    # 0x143038c80
14227e698:     	jmp	0x14227ea83 <.text+0x227da83>
14227e69d:     	movq	%r8, 0x68(%rbp)
14227e6a1:     	movq	%rdx, 0x98(%rbp)
14227e6a8:     	movzwl	0x4(%rdx), %r9d
14227e6ad:     	leaq	0x38(%rbp), %rcx
14227e6b1:     	movq	%rbx, %rdx
14227e6b4:     	movq	%rdi, %r8
14227e6b7:     	callq	0x1409e6280 <.text+0x9e5280>
14227e6bc:     	movq	0x38(%rbp), %rax
14227e6c0:     	movq	%rax, %rcx
14227e6c3:     	negq	%rcx
14227e6c6:     	jno	0x14227ec30 <.text+0x227dc30>
14227e6cc:     	movl	$0xc4, (%rsi)
14227e6d2:     	jmp	0x14227ed6c <.text+0x227dd6c>
14227e6d7:     	movq	%r14, 0x78(%rbp)
14227e6db:     	movq	%rsi, 0x70(%rbp)
14227e6df:     	movq	%r8, 0x68(%rbp)
14227e6e3:     	movups	0x8(%rdx), %xmm0
14227e6e7:     	movaps	%xmm0, 0x80(%rbp)
14227e6ee:     	movq	%rdx, 0x98(%rbp)
14227e6f5:     	movq	0x18(%rdx), %rax
14227e6f9:     	movq	%rax, 0x90(%rbp)
14227e700:     	movq	0x88(%rbp), %r14
14227e707:     	leaq	(%rax,%rax,2), %rsi
14227e70b:     	shlq	$0x4, %rsi
14227e70f:     	addq	%r14, %rsi
14227e712:     	movq	%rbp, %r15
14227e715:     	leaq	0xdba584(%rip), %r12    # 0x143038ca0
14227e71c:     	jmp	0x14227e753 <.text+0x227d753>
14227e71e:     	jae	0x14227e735 <.text+0x227d735>
14227e720:     	movq	0x10(%r14), %rcx
14227e724:     	shlq	$0x4, %rdx
14227e728:     	movl	$0x8, %r8d
14227e72e:     	nop
14227e730:     	callq	0x140613c20 <.text+0x612c20>
14227e735:     	movups	0x38(%rbp), %xmm0
14227e739:     	movups	0x48(%rbp), %xmm1
14227e73d:     	movups	0x58(%rbp), %xmm2
14227e741:     	movups	%xmm2, 0x20(%r14)
14227e746:     	movups	%xmm1, 0x10(%r14)
14227e74b:     	movups	%xmm0, (%r14)
14227e74f:     	addq	$0x30, %r14
14227e753:     	cmpq	%rsi, %r14
14227e756:     	je	0x14227e9ff <.text+0x227d9ff>
14227e75c:     	movups	(%r14), %xmm0
14227e760:     	movups	0x10(%r14), %xmm1
14227e765:     	movups	0x20(%r14), %xmm2
14227e76a:     	movaps	%xmm2, 0x20(%rbp)
14227e76e:     	movaps	%xmm1, 0x10(%rbp)
14227e772:     	movaps	%xmm0, (%rbp)
14227e776:     	movl	$0xc4, (%r14)
14227e77d:     	leaq	0x38(%rbp), %rcx
14227e781:     	movq	%r15, %rdx
14227e784:     	movq	%rdi, %r8
14227e787:     	movq	%rbx, %r9
14227e78a:     	callq	0x14227e5e0 <.text+0x227d5e0>
14227e78f:     	movl	(%r14), %eax
14227e792:     	movl	$0xffffff3c, %ecx       # imm = 0xFFFFFF3C
14227e797:     	addl	%ecx, %eax
14227e799:     	cmpl	$0x9, %eax
14227e79c:     	cmovael	%r13d, %eax
14227e7a0:     	cmpl	$0x7, %eax
14227e7a3:     	ja	0x14227e81d <.text+0x227d81d>
14227e7a5:     	movslq	(%r12,%rax,4), %rax
14227e7a9:     	addq	%r12, %rax
14227e7ac:     	jmpq	*%rax
14227e7ae:     	movq	0x8(%r14), %rdx
14227e7b2:     	movq	%rdx, %rax
14227e7b5:     	negq	%rax
14227e7b8:     	jno	0x14227e71e <.text+0x227d71e>
14227e7be:     	movq	0x10(%r14), %rdx
14227e7c2:     	movq	%rdx, %rax
14227e7c5:     	negq	%rax
14227e7c8:     	jo	0x14227e735 <.text+0x227d735>
14227e7ce:     	jae	0x14227e735 <.text+0x227d735>
14227e7d4:     	movq	0x18(%r14), %rcx
14227e7d8:     	movl	$0x1, %r8d
14227e7de:     	jmp	0x14227e730 <.text+0x227d730>
14227e7e3:     	movq	0x10(%r14), %rcx
14227e7e7:     	movq	0x18(%r14), %rdx
14227e7eb:     	movq	%r14, 0xa8(%rbp)
14227e7f2:     	movq	%rcx, 0xa0(%rbp)
14227e7f9:     	callq	0x140eaade0 <.text+0xea9de0>
14227e7fe:     	jmp	0x14227e838 <.text+0x227d838>
14227e800:     	movq	0x10(%r14), %rcx
14227e804:     	movq	0x18(%r14), %rdx
14227e808:     	movq	%r14, 0xa8(%rbp)
14227e80f:     	movq	%rcx, 0xa0(%rbp)
14227e816:     	callq	0x140eaade0 <.text+0xea9de0>
14227e81b:     	jmp	0x14227e838 <.text+0x227d838>
14227e81d:     	movq	0x10(%r14), %rcx
14227e821:     	movq	0x18(%r14), %rdx
14227e825:     	movq	%r14, 0xa8(%rbp)
14227e82c:     	movq	%rcx, 0xa0(%rbp)
14227e833:     	callq	0x140eaade0 <.text+0xea9de0>
14227e838:     	movq	0xa8(%rbp), %r14
14227e83f:     	movq	0x8(%r14), %rax
14227e843:     	testq	%rax, %rax
14227e846:     	je	0x14227e735 <.text+0x227d735>
14227e84c:     	shlq	$0x4, %rax
14227e850:     	leaq	(%rax,%rax,2), %rdx
14227e854:     	movl	$0x8, %r8d
14227e85a:     	movq	0xa0(%rbp), %rcx
14227e861:     	jmp	0x14227e730 <.text+0x227d730>
14227e866:     	movq	%r14, 0x78(%rbp)
14227e86a:     	movq	%rsi, 0x70(%rbp)
14227e86e:     	movq	%r8, 0x68(%rbp)
14227e872:     	movups	0x8(%rdx), %xmm0
14227e876:     	movaps	%xmm0, 0x80(%rbp)
14227e87d:     	movq	%rdx, 0x98(%rbp)
14227e884:     	movq	0x18(%rdx), %rax
14227e888:     	movq	%rax, 0x90(%rbp)
14227e88f:     	movq	0x88(%rbp), %r14
14227e896:     	leaq	(%rax,%rax,2), %rsi
14227e89a:     	shlq	$0x4, %rsi
14227e89e:     	addq	%r14, %rsi
14227e8a1:     	leaq	-0x60(%rbp), %r15
14227e8a5:     	leaq	0xdba3b4(%rip), %r12    # 0x143038c60
14227e8ac:     	jmp	0x14227e8e8 <.text+0x227d8e8>
14227e8ae:     	nop
14227e8b0:     	shlq	$0x4, %rax
14227e8b4:     	leaq	(%rax,%rax,2), %rdx
14227e8b8:     	movl	$0x8, %r8d
14227e8be:     	movq	0xa0(%rbp), %rcx
14227e8c5:     	callq	0x140613c20 <.text+0x612c20>
14227e8ca:     	movups	0x38(%rbp), %xmm0
14227e8ce:     	movups	0x48(%rbp), %xmm1
14227e8d2:     	movups	0x58(%rbp), %xmm2
14227e8d6:     	movups	%xmm2, 0x20(%r14)
14227e8db:     	movups	%xmm1, 0x10(%r14)
14227e8e0:     	movups	%xmm0, (%r14)
14227e8e4:     	addq	$0x30, %r14
14227e8e8:     	cmpq	%rsi, %r14
14227e8eb:     	je	0x14227ea24 <.text+0x227da24>
14227e8f1:     	movups	(%r14), %xmm0
14227e8f5:     	movups	0x10(%r14), %xmm1
14227e8fa:     	movups	0x20(%r14), %xmm2
14227e8ff:     	movaps	%xmm2, -0x40(%rbp)
14227e903:     	movaps	%xmm1, -0x50(%rbp)
14227e907:     	movaps	%xmm0, -0x60(%rbp)
14227e90b:     	movl	$0xc4, (%r14)
14227e912:     	leaq	0x38(%rbp), %rcx
14227e916:     	movq	%r15, %rdx
14227e919:     	movq	%rdi, %r8
14227e91c:     	movq	%rbx, %r9
14227e91f:     	callq	0x14227e5e0 <.text+0x227d5e0>
14227e924:     	movl	(%r14), %eax
14227e927:     	movl	$0xffffff3c, %ecx       # imm = 0xFFFFFF3C
14227e92c:     	addl	%ecx, %eax
14227e92e:     	cmpl	$0x9, %eax
14227e931:     	cmovael	%r13d, %eax
14227e935:     	cmpl	$0x7, %eax
14227e938:     	ja	0x14227e9b2 <.text+0x227d9b2>
14227e93a:     	movslq	(%r12,%rax,4), %rax
14227e93e:     	addq	%r12, %rax
14227e941:     	jmpq	*%rax
14227e943:     	movq	0x8(%r14), %rdx
14227e947:     	movq	%rdx, %rax
14227e94a:     	negq	%rax
14227e94d:     	jno	0x14227e9e6 <.text+0x227d9e6>
14227e953:     	movq	0x10(%r14), %rdx
14227e957:     	movq	%rdx, %rax
14227e95a:     	negq	%rax
14227e95d:     	jo	0x14227e8ca <.text+0x227d8ca>
14227e963:     	jae	0x14227e8ca <.text+0x227d8ca>
14227e969:     	movq	0x18(%r14), %rcx
14227e96d:     	movl	$0x1, %r8d
14227e973:     	jmp	0x14227e8c5 <.text+0x227d8c5>
14227e978:     	movq	0x10(%r14), %rcx
14227e97c:     	movq	0x18(%r14), %rdx
14227e980:     	movq	%r14, 0xa8(%rbp)
14227e987:     	movq	%rcx, 0xa0(%rbp)
14227e98e:     	callq	0x140eaade0 <.text+0xea9de0>
14227e993:     	jmp	0x14227e9cd <.text+0x227d9cd>
14227e995:     	movq	0x10(%r14), %rcx
14227e999:     	movq	0x18(%r14), %rdx
14227e99d:     	movq	%r14, 0xa8(%rbp)
14227e9a4:     	movq	%rcx, 0xa0(%rbp)
14227e9ab:     	callq	0x140eaade0 <.text+0xea9de0>
14227e9b0:     	jmp	0x14227e9cd <.text+0x227d9cd>
14227e9b2:     	movq	0x10(%r14), %rcx
14227e9b6:     	movq	0x18(%r14), %rdx
14227e9ba:     	movq	%r14, 0xa8(%rbp)
14227e9c1:     	movq	%rcx, 0xa0(%rbp)
14227e9c8:     	callq	0x140eaade0 <.text+0xea9de0>
14227e9cd:     	movq	0xa8(%rbp), %r14
14227e9d4:     	movq	0x8(%r14), %rax
14227e9d8:     	testq	%rax, %rax
14227e9db:     	jne	0x14227e8b0 <.text+0x227d8b0>
14227e9e1:     	jmp	0x14227e8ca <.text+0x227d8ca>
14227e9e6:     	jae	0x14227e8ca <.text+0x227d8ca>
14227e9ec:     	movq	0x10(%r14), %rcx
14227e9f0:     	shlq	$0x4, %rdx
14227e9f4:     	movl	$0x8, %r8d
14227e9fa:     	jmp	0x14227e8c5 <.text+0x227d8c5>
14227e9ff:     	movq	0x90(%rbp), %rax
14227ea06:     	movq	0x70(%rbp), %rsi
14227ea0a:     	movq	%rax, 0x18(%rsi)
14227ea0e:     	movaps	0x80(%rbp), %xmm0
14227ea15:     	movups	%xmm0, 0x8(%rsi)
14227ea19:     	movl	$0xca, (%rsi)
14227ea1f:     	jmp	0x14227ebc5 <.text+0x227dbc5>
14227ea24:     	movq	0x90(%rbp), %rax
14227ea2b:     	movq	0x70(%rbp), %rsi
14227ea2f:     	movq	%rax, 0x18(%rsi)
14227ea33:     	movaps	0x80(%rbp), %xmm0
14227ea3a:     	movups	%xmm0, 0x8(%rsi)
14227ea3e:     	movl	$0xcc, (%rsi)
14227ea44:     	jmp	0x14227ebc5 <.text+0x227dbc5>
14227ea49:     	movq	0x18(%r14), %rcx
14227ea4d:     	movl	$0x1, %r8d
14227ea53:     	nopw	%cs:(%rax,%rax)
14227ea60:     	callq	0x140613c20 <.text+0x612c20>
14227ea65:     	movups	0x38(%rbp), %xmm0
14227ea69:     	movups	0x48(%rbp), %xmm1
14227ea6d:     	movups	0x58(%rbp), %xmm2
14227ea71:     	movups	%xmm2, 0x20(%r14)
14227ea76:     	movups	%xmm1, 0x10(%r14)
14227ea7b:     	movups	%xmm0, (%r14)
14227ea7f:     	addq	$0x30, %r14
14227ea83:     	cmpq	%rsi, %r14
14227ea86:     	je	0x14227eba5 <.text+0x227dba5>
14227ea8c:     	movups	(%r14), %xmm0
14227ea90:     	movups	0x10(%r14), %xmm1
14227ea95:     	movups	0x20(%r14), %xmm2
14227ea9a:     	movaps	%xmm2, -0x10(%rbp)
14227ea9e:     	movaps	%xmm1, -0x20(%rbp)
14227eaa2:     	movaps	%xmm0, -0x30(%rbp)
14227eaa6:     	movl	$0xc4, (%r14)
14227eaad:     	leaq	0x38(%rbp), %rcx
14227eab1:     	movq	%r15, %rdx
14227eab4:     	movq	%rdi, %r8
14227eab7:     	movq	%rbx, %r9
14227eaba:     	callq	0x14227e5e0 <.text+0x227d5e0>
14227eabf:     	movl	(%r14), %eax
14227eac2:     	movl	$0xffffff3c, %ecx       # imm = 0xFFFFFF3C
14227eac7:     	addl	%ecx, %eax
14227eac9:     	cmpl	$0x9, %eax
14227eacc:     	cmovael	%r13d, %eax
14227ead0:     	cmpl	$0x7, %eax
14227ead3:     	ja	0x14227eb43 <.text+0x227db43>
14227ead5:     	movslq	(%r12,%rax,4), %rax
14227ead9:     	addq	%r12, %rax
14227eadc:     	jmpq	*%rax
14227eade:     	movq	0x8(%r14), %rdx
14227eae2:     	movq	%rdx, %rax
14227eae5:     	negq	%rax
14227eae8:     	jno	0x14227eb8c <.text+0x227db8c>
14227eaee:     	movq	0x10(%r14), %rdx
14227eaf2:     	movq	%rdx, %rax
14227eaf5:     	negq	%rax
14227eaf8:     	jo	0x14227ea65 <.text+0x227da65>
14227eafe:     	jb	0x14227ea49 <.text+0x227da49>
14227eb04:     	jmp	0x14227ea65 <.text+0x227da65>
14227eb09:     	movq	0x10(%r14), %rcx
14227eb0d:     	movq	0x18(%r14), %rdx
14227eb11:     	movq	%r14, 0xa8(%rbp)
14227eb18:     	movq	%rcx, 0xa0(%rbp)
14227eb1f:     	callq	0x140eaade0 <.text+0xea9de0>
14227eb24:     	jmp	0x14227eb5e <.text+0x227db5e>
14227eb26:     	movq	0x10(%r14), %rcx
14227eb2a:     	movq	0x18(%r14), %rdx
14227eb2e:     	movq	%r14, 0xa8(%rbp)
14227eb35:     	movq	%rcx, 0xa0(%rbp)
14227eb3c:     	callq	0x140eaade0 <.text+0xea9de0>
14227eb41:     	jmp	0x14227eb5e <.text+0x227db5e>
14227eb43:     	movq	0x10(%r14), %rcx
14227eb47:     	movq	0x18(%r14), %rdx
14227eb4b:     	movq	%r14, 0xa8(%rbp)
14227eb52:     	movq	%rcx, 0xa0(%rbp)
14227eb59:     	callq	0x140eaade0 <.text+0xea9de0>
14227eb5e:     	movq	0xa8(%rbp), %r14
14227eb65:     	movq	0x8(%r14), %rax
14227eb69:     	testq	%rax, %rax
14227eb6c:     	je	0x14227ea65 <.text+0x227da65>
14227eb72:     	shlq	$0x4, %rax
14227eb76:     	leaq	(%rax,%rax,2), %rdx
14227eb7a:     	movl	$0x8, %r8d
14227eb80:     	movq	0xa0(%rbp), %rcx
14227eb87:     	jmp	0x14227ea60 <.text+0x227da60>
14227eb8c:     	jae	0x14227ea65 <.text+0x227da65>
14227eb92:     	movq	0x10(%r14), %rcx
14227eb96:     	shlq	$0x4, %rdx
14227eb9a:     	movl	$0x8, %r8d
14227eba0:     	jmp	0x14227ea60 <.text+0x227da60>
14227eba5:     	movq	0x90(%rbp), %rax
14227ebac:     	movq	0x70(%rbp), %rsi
14227ebb0:     	movq	%rax, 0x18(%rsi)
14227ebb4:     	movaps	0x80(%rbp), %xmm0
14227ebbb:     	movups	%xmm0, 0x8(%rsi)
14227ebbf:     	movl	$0xcb, (%rsi)
14227ebc5:     	movq	0x78(%rbp), %r14
14227ebc9:     	movq	0x68(%rbp), %rax
14227ebcd:     	addb	$0x36, %al
14227ebcf:     	cmpb	$0x3, %al
14227ebd1:     	jb	0x14227ed6c <.text+0x227dd6c>
14227ebd7:     	cmpl	$0x7, %r14d
14227ebdb:     	ja	0x14227ed20 <.text+0x227dd20>
14227ebe1:     	movl	%r14d, %eax
14227ebe4:     	leaq	0xdba0d5(%rip), %rcx    # 0x143038cc0
14227ebeb:     	movslq	(%rcx,%rax,4), %rax
14227ebef:     	addq	%rcx, %rax
14227ebf2:     	jmpq	*%rax
14227ebf4:     	movq	0x98(%rbp), %r8
14227ebfb:     	movq	0x8(%r8), %rdx
14227ebff:     	movq	%rdx, %rax
14227ec02:     	negq	%rax
14227ec05:     	jno	0x14227edbb <.text+0x227ddbb>
14227ec0b:     	movq	0x10(%r8), %rdx
14227ec0f:     	movq	%rdx, %rax
14227ec12:     	negq	%rax
14227ec15:     	jo	0x14227ed6c <.text+0x227dd6c>
14227ec1b:     	jae	0x14227ed6c <.text+0x227dd6c>
14227ec21:     	movq	0x18(%r8), %rcx
14227ec25:     	movl	$0x1, %r8d
14227ec2b:     	jmp	0x14227ed67 <.text+0x227dd67>
14227ec30:     	movq	0x40(%rbp), %rcx
14227ec34:     	movq	0x48(%rbp), %rdx
14227ec38:     	shlq	$0x5, %rdx
14227ec3c:     	addq	%rcx, %rdx
14227ec3f:     	movq	%rcx, 0x38(%rbp)
14227ec43:     	movq	%rcx, 0x40(%rbp)
14227ec47:     	movq	%rax, 0x48(%rbp)
14227ec4b:     	movq	%rdx, 0x50(%rbp)
14227ec4f:     	leaq	0xdb9aba(%rip), %r8     # 0x143038710
14227ec56:     	leaq	0x80(%rbp), %rcx
14227ec5d:     	leaq	0x38(%rbp), %rdx
14227ec61:     	callq	0x141c58550 <.text+0x1c57550>
14227ec66:     	movq	0x90(%rbp), %rax
14227ec6d:     	testq	%rax, %rax
14227ec70:     	je	0x14227ed83 <.text+0x227dd83>
14227ec76:     	cmpq	$0x1, %rax
14227ec7a:     	jne	0x14227ed9a <.text+0x227dd9a>
14227ec80:     	movq	0x80(%rbp), %rcx
14227ec87:     	movq	0x88(%rbp), %rax
14227ec8e:     	leaq	0x30(%rax), %rdx
14227ec92:     	movq	%rax, 0x38(%rbp)
14227ec96:     	movq	%rcx, 0x48(%rbp)
14227ec9a:     	movq	%rdx, 0x50(%rbp)
14227ec9e:     	movq	%rdx, 0x40(%rbp)
14227eca2:     	movl	(%rax), %ecx
14227eca4:     	cmpl	$0xcd, %ecx
14227ecaa:     	je	0x14227edcd <.text+0x227ddcd>
14227ecb0:     	movl	%ecx, (%rsi)
14227ecb2:     	movups	0x4(%rax), %xmm0
14227ecb6:     	movups	0x14(%rax), %xmm1
14227ecba:     	movups	0x20(%rax), %xmm2
14227ecbe:     	movups	%xmm0, 0x4(%rsi)
14227ecc2:     	movups	%xmm1, 0x14(%rsi)
14227ecc6:     	movups	%xmm2, 0x20(%rsi)
14227ecca:     	leaq	0x38(%rbp), %rcx
14227ecce:     	callq	0x140ea8930 <.text+0xea7930>
14227ecd3:     	jmp	0x14227ebc9 <.text+0x227dbc9>
14227ecd8:     	movq	0x98(%rbp), %rax
14227ecdf:     	movq	0x10(%rax), %rcx
14227ece3:     	movq	%rax, 0x98(%rbp)
14227ecea:     	movq	0x18(%rax), %rdx
14227ecee:     	movq	%rcx, 0xa8(%rbp)
14227ecf5:     	callq	0x140eaade0 <.text+0xea9de0>
14227ecfa:     	jmp	0x14227ed42 <.text+0x227dd42>
14227ecfc:     	movq	0x98(%rbp), %rax
14227ed03:     	movq	0x10(%rax), %rcx
14227ed07:     	movq	%rax, 0x98(%rbp)
14227ed0e:     	movq	0x18(%rax), %rdx
14227ed12:     	movq	%rcx, 0xa8(%rbp)
14227ed19:     	callq	0x140eaade0 <.text+0xea9de0>
14227ed1e:     	jmp	0x14227ed42 <.text+0x227dd42>
14227ed20:     	movq	0x98(%rbp), %rax
14227ed27:     	movq	0x10(%rax), %rcx
14227ed2b:     	movq	%rax, 0x98(%rbp)
14227ed32:     	movq	0x18(%rax), %rdx
14227ed36:     	movq	%rcx, 0xa8(%rbp)
14227ed3d:     	callq	0x140eaade0 <.text+0xea9de0>
14227ed42:     	movq	0x98(%rbp), %rax
14227ed49:     	movq	0x8(%rax), %rax
14227ed4d:     	testq	%rax, %rax
14227ed50:     	je	0x14227ed6c <.text+0x227dd6c>
14227ed52:     	shlq	$0x4, %rax
14227ed56:     	leaq	(%rax,%rax,2), %rdx
14227ed5a:     	movl	$0x8, %r8d
14227ed60:     	movq	0xa8(%rbp), %rcx
14227ed67:     	callq	0x140613c20 <.text+0x612c20>
14227ed6c:     	movq	%rsi, %rax
14227ed6f:     	addq	$0x138, %rsp            # imm = 0x138
14227ed76:     	popq	%rbx
14227ed77:     	popq	%rdi
14227ed78:     	popq	%rsi
14227ed79:     	popq	%r12
14227ed7b:     	popq	%r13
14227ed7d:     	popq	%r14
14227ed7f:     	popq	%r15
14227ed81:     	popq	%rbp
14227ed82:     	retq
14227ed83:     	movl	$0xc4, (%rsi)
14227ed89:     	leaq	0x80(%rbp), %rcx
14227ed90:     	callq	0x140ea8740 <.text+0xea7740>
14227ed95:     	jmp	0x14227ebc9 <.text+0x227dbc9>
14227ed9a:     	movq	0x90(%rbp), %rax
14227eda1:     	movq	%rax, 0x18(%rsi)
14227eda5:     	movups	0x80(%rbp), %xmm0
14227edac:     	movups	%xmm0, 0x8(%rsi)
14227edb0:     	movl	$0xca, (%rsi)
14227edb6:     	jmp	0x14227ebc9 <.text+0x227dbc9>
14227edbb:     	jae	0x14227ed6c <.text+0x227dd6c>
14227edbd:     	movq	0x10(%r8), %rcx
14227edc1:     	shlq	$0x4, %rdx
14227edc5:     	movl	$0x8, %r8d
14227edcb:     	jmp	0x14227ed67 <.text+0x227dd67>
14227edcd:     	leaq	0xdb9a3c(%rip), %rcx    # 0x143038810
14227edd4:     	callq	0x1428d9310 <.text+0x28d8310>
14227edd9:     	ud2
14227eddb:     	nopl	(%rax,%rax)
14227ede0:     	movq	%rdx, 0x10(%rsp)
14227ede5:     	pushq	%rbp
14227ede6:     	pushq	%r15
14227ede8:     	pushq	%r14
14227edea:     	pushq	%r13
14227edec:     	pushq	%r12
14227edee:     	pushq	%rsi
14227edef:     	pushq	%rdi
14227edf0:     	pushq	%rbx
14227edf1:     	subq	$0x28, %rsp
14227edf5:     	leaq	0x80(%rdx), %rbp
14227edfc:     	leaq	0x38(%rbp), %rcx
14227ee00:     	callq	0x140ea8930 <.text+0xea7930>
14227ee05:     	nop
14227ee06:     	addq	$0x28, %rsp
14227ee0a:     	popq	%rbx
14227ee0b:     	popq	%rdi
14227ee0c:     	popq	%rsi
14227ee0d:     	popq	%r12
14227ee0f:     	popq	%r13
14227ee11:     	popq	%r14
14227ee13:     	popq	%r15
14227ee15:     	popq	%rbp
14227ee16:     	retq
14227ee17:     	nopw	(%rax,%rax)
14227ee20:     	movq	%rdx, 0x10(%rsp)
14227ee25:     	pushq	%rbp
14227ee26:     	pushq	%r15
14227ee28:     	pushq	%r14
14227ee2a:     	pushq	%r13
14227ee2c:     	pushq	%r12
14227ee2e:     	pushq	%rsi
14227ee2f:     	pushq	%rdi
14227ee30:     	pushq	%rbx
14227ee31:     	subq	$0x28, %rsp
14227ee35:     	leaq	0x80(%rdx), %rbp
14227ee3c:     	movq	0x98(%rbp), %rax
14227ee43:     	movq	0x8(%rax), %rax
14227ee47:     	testq	%rax, %rax
14227ee4a:     	je	0x14227ee66 <.text+0x227de66>
14227ee4c:     	shlq	$0x4, %rax
14227ee50:     	leaq	(%rax,%rax,2), %rdx
14227ee54:     	movl	$0x8, %r8d
14227ee5a:     	movq	0xa8(%rbp), %rcx
14227ee61:     	callq	0x140613c20 <.text+0x612c20>
14227ee66:     	nop
14227ee67:     	addq	$0x28, %rsp
14227ee6b:     	popq	%rbx
14227ee6c:     	popq	%rdi
14227ee6d:     	popq	%rsi
14227ee6e:     	popq	%r12
14227ee70:     	popq	%r13
14227ee72:     	popq	%r14
14227ee74:     	popq	%r15
14227ee76:     	popq	%rbp
14227ee77:     	retq
14227ee78:     	nopl	(%rax,%rax)
14227ee80:     	movq	%rdx, 0x10(%rsp)
14227ee85:     	pushq	%rbp
14227ee86:     	pushq	%r15
14227ee88:     	pushq	%r14
14227ee8a:     	pushq	%r13
14227ee8c:     	pushq	%r12
14227ee8e:     	pushq	%rsi
14227ee8f:     	pushq	%rdi
14227ee90:     	pushq	%rbx
14227ee91:     	subq	$0x28, %rsp
14227ee95:     	leaq	0x80(%rdx), %rbp
14227ee9c:     	movq	0x98(%rbp), %rax
14227eea3:     	movq	0x8(%rax), %rax
14227eea7:     	testq	%rax, %rax
14227eeaa:     	je	0x14227eec6 <.text+0x227dec6>
14227eeac:     	shlq	$0x4, %rax
14227eeb0:     	leaq	(%rax,%rax,2), %rdx
14227eeb4:     	movl	$0x8, %r8d
14227eeba:     	movq	0xa8(%rbp), %rcx
14227eec1:     	callq	0x140613c20 <.text+0x612c20>
14227eec6:     	nop
14227eec7:     	addq	$0x28, %rsp
14227eecb:     	popq	%rbx
14227eecc:     	popq	%rdi
14227eecd:     	popq	%rsi
14227eece:     	popq	%r12
14227eed0:     	popq	%r13
14227eed2:     	popq	%r14
14227eed4:     	popq	%r15
14227eed6:     	popq	%rbp
14227eed7:     	retq
14227eed8:     	nopl	(%rax,%rax)
14227eee0:     	movq	%rdx, 0x10(%rsp)
14227eee5:     	pushq	%rbp
14227eee6:     	pushq	%r15
14227eee8:     	pushq	%r14
14227eeea:     	pushq	%r13
14227eeec:     	pushq	%r12
14227eeee:     	pushq	%rsi
14227eeef:     	pushq	%rdi
14227eef0:     	pushq	%rbx
14227eef1:     	subq	$0x28, %rsp
14227eef5:     	leaq	0x80(%rdx), %rbp
14227eefc:     	movq	0x98(%rbp), %rax
14227ef03:     	movq	0x8(%rax), %rax
14227ef07:     	testq	%rax, %rax
14227ef0a:     	je	0x14227ef26 <.text+0x227df26>
14227ef0c:     	shlq	$0x4, %rax
14227ef10:     	leaq	(%rax,%rax,2), %rdx
14227ef14:     	movl	$0x8, %r8d
14227ef1a:     	movq	0xa8(%rbp), %rcx
14227ef21:     	callq	0x140613c20 <.text+0x612c20>
14227ef26:     	nop
14227ef27:     	addq	$0x28, %rsp
14227ef2b:     	popq	%rbx
14227ef2c:     	popq	%rdi
14227ef2d:     	popq	%rsi
14227ef2e:     	popq	%r12
14227ef30:     	popq	%r13
14227ef32:     	popq	%r14
14227ef34:     	popq	%r15
14227ef36:     	popq	%rbp
14227ef37:     	retq
14227ef38:     	nopl	(%rax,%rax)
14227ef40:     	movq	%rdx, 0x10(%rsp)
14227ef45:     	pushq	%rbp
14227ef46:     	pushq	%r15
14227ef48:     	pushq	%r14
14227ef4a:     	pushq	%r13
14227ef4c:     	pushq	%r12
14227ef4e:     	pushq	%rsi
14227ef4f:     	pushq	%rdi
14227ef50:     	pushq	%rbx
14227ef51:     	subq	$0x28, %rsp
14227ef55:     	leaq	0x80(%rdx), %rbp
14227ef5c:     	movq	0xa8(%rbp), %rax
14227ef63:     	movq	0x8(%rax), %rax
14227ef67:     	testq	%rax, %rax
14227ef6a:     	je	0x14227ef86 <.text+0x227df86>
14227ef6c:     	shlq	$0x4, %rax
14227ef70:     	leaq	(%rax,%rax,2), %rdx
14227ef74:     	movl	$0x8, %r8d
14227ef7a:     	movq	0xa0(%rbp), %rcx
14227ef81:     	callq	0x140613c20 <.text+0x612c20>
14227ef86:     	nop
14227ef87:     	addq	$0x28, %rsp
14227ef8b:     	popq	%rbx
14227ef8c:     	popq	%rdi
14227ef8d:     	popq	%rsi
14227ef8e:     	popq	%r12
14227ef90:     	popq	%r13
14227ef92:     	popq	%r14
14227ef94:     	popq	%r15
14227ef96:     	popq	%rbp
14227ef97:     	retq
14227ef98:     	nopl	(%rax,%rax)
14227efa0:     	movq	%rdx, 0x10(%rsp)
14227efa5:     	pushq	%rbp
14227efa6:     	pushq	%r15
14227efa8:     	pushq	%r14
14227efaa:     	pushq	%r13
14227efac:     	pushq	%r12
14227efae:     	pushq	%rsi
14227efaf:     	pushq	%rdi
14227efb0:     	pushq	%rbx
14227efb1:     	subq	$0x28, %rsp
14227efb5:     	leaq	0x80(%rdx), %rbp
14227efbc:     	movq	0xa8(%rbp), %rax
14227efc3:     	movq	0x8(%rax), %rax
14227efc7:     	testq	%rax, %rax
14227efca:     	je	0x14227efe6 <.text+0x227dfe6>
14227efcc:     	shlq	$0x4, %rax
14227efd0:     	leaq	(%rax,%rax,2), %rdx
14227efd4:     	movl	$0x8, %r8d
14227efda:     	movq	0xa0(%rbp), %rcx
14227efe1:     	callq	0x140613c20 <.text+0x612c20>
14227efe6:     	nop
14227efe7:     	addq	$0x28, %rsp
14227efeb:     	popq	%rbx
14227efec:     	popq	%rdi
14227efed:     	popq	%rsi
14227efee:     	popq	%r12
14227eff0:     	popq	%r13
14227eff2:     	popq	%r14
14227eff4:     	popq	%r15
14227eff6:     	popq	%rbp
14227eff7:     	retq
14227eff8:     	nopl	(%rax,%rax)
14227f000:     	movq	%rdx, 0x10(%rsp)
14227f005:     	pushq	%rbp
14227f006:     	pushq	%r15
14227f008:     	pushq	%r14
14227f00a:     	pushq	%r13
14227f00c:     	pushq	%r12
14227f00e:     	pushq	%rsi
14227f00f:     	pushq	%rdi
14227f010:     	pushq	%rbx
14227f011:     	subq	$0x28, %rsp
14227f015:     	leaq	0x80(%rdx), %rbp
14227f01c:     	movq	0xa8(%rbp), %rax
14227f023:     	movq	0x8(%rax), %rax
14227f027:     	testq	%rax, %rax
14227f02a:     	je	0x14227f046 <.text+0x227e046>
14227f02c:     	shlq	$0x4, %rax
14227f030:     	leaq	(%rax,%rax,2), %rdx
14227f034:     	movl	$0x8, %r8d
14227f03a:     	movq	0xa0(%rbp), %rcx
14227f041:     	callq	0x140613c20 <.text+0x612c20>
14227f046:     	nop
14227f047:     	addq	$0x28, %rsp
14227f04b:     	popq	%rbx
14227f04c:     	popq	%rdi
14227f04d:     	popq	%rsi
14227f04e:     	popq	%r12
14227f050:     	popq	%r13
14227f052:     	popq	%r14
14227f054:     	popq	%r15
14227f056:     	popq	%rbp
14227f057:     	retq
14227f058:     	nopl	(%rax,%rax)
14227f060:     	movq	%rdx, 0x10(%rsp)
14227f065:     	pushq	%rbp
14227f066:     	pushq	%r15
14227f068:     	pushq	%r14
14227f06a:     	pushq	%r13
14227f06c:     	pushq	%r12
14227f06e:     	pushq	%rsi
14227f06f:     	pushq	%rdi
14227f070:     	pushq	%rbx
14227f071:     	subq	$0x28, %rsp
14227f075:     	leaq	0x80(%rdx), %rbp
14227f07c:     	movq	0xa8(%rbp), %rax
14227f083:     	movq	0x8(%rax), %rax
14227f087:     	testq	%rax, %rax
14227f08a:     	je	0x14227f0a6 <.text+0x227e0a6>
14227f08c:     	shlq	$0x4, %rax
14227f090:     	leaq	(%rax,%rax,2), %rdx
14227f094:     	movl	$0x8, %r8d
14227f09a:     	movq	0xa0(%rbp), %rcx
14227f0a1:     	callq	0x140613c20 <.text+0x612c20>
14227f0a6:     	nop
14227f0a7:     	addq	$0x28, %rsp
14227f0ab:     	popq	%rbx
14227f0ac:     	popq	%rdi
14227f0ad:     	popq	%rsi
14227f0ae:     	popq	%r12
14227f0b0:     	popq	%r13
14227f0b2:     	popq	%r14
14227f0b4:     	popq	%r15
14227f0b6:     	popq	%rbp
14227f0b7:     	retq
14227f0b8:     	nopl	(%rax,%rax)
14227f0c0:     	movq	%rdx, 0x10(%rsp)
14227f0c5:     	pushq	%rbp
14227f0c6:     	pushq	%r15
14227f0c8:     	pushq	%r14
14227f0ca:     	pushq	%r13
14227f0cc:     	pushq	%r12
14227f0ce:     	pushq	%rsi
14227f0cf:     	pushq	%rdi
14227f0d0:     	pushq	%rbx
14227f0d1:     	subq	$0x28, %rsp
14227f0d5:     	leaq	0x80(%rdx), %rbp
14227f0dc:     	movq	0xa8(%rbp), %rax
14227f0e3:     	movq	0x8(%rax), %rax
14227f0e7:     	testq	%rax, %rax
14227f0ea:     	je	0x14227f106 <.text+0x227e106>
14227f0ec:     	shlq	$0x4, %rax
14227f0f0:     	leaq	(%rax,%rax,2), %rdx
14227f0f4:     	movl	$0x8, %r8d
14227f0fa:     	movq	0xa0(%rbp), %rcx
14227f101:     	callq	0x140613c20 <.text+0x612c20>
14227f106:     	nop
14227f107:     	addq	$0x28, %rsp
14227f10b:     	popq	%rbx
14227f10c:     	popq	%rdi
14227f10d:     	popq	%rsi
14227f10e:     	popq	%r12
14227f110:     	popq	%r13
14227f112:     	popq	%r14
14227f114:     	popq	%r15
14227f116:     	popq	%rbp
14227f117:     	retq
14227f118:     	nopl	(%rax,%rax)
14227f120:     	movq	%rdx, 0x10(%rsp)
14227f125:     	pushq	%rbp
14227f126:     	pushq	%r15
14227f128:     	pushq	%r14
14227f12a:     	pushq	%r13
14227f12c:     	pushq	%r12
14227f12e:     	pushq	%rsi
14227f12f:     	pushq	%rdi
14227f130:     	pushq	%rbx
14227f131:     	subq	$0x28, %rsp
14227f135:     	leaq	0x80(%rdx), %rbp
14227f13c:     	movq	0xa8(%rbp), %rax
14227f143:     	movq	0x8(%rax), %rax
14227f147:     	testq	%rax, %rax
14227f14a:     	je	0x14227f166 <.text+0x227e166>
14227f14c:     	shlq	$0x4, %rax
14227f150:     	leaq	(%rax,%rax,2), %rdx
14227f154:     	movl	$0x8, %r8d
14227f15a:     	movq	0xa0(%rbp), %rcx
14227f161:     	callq	0x140613c20 <.text+0x612c20>
14227f166:     	nop
14227f167:     	addq	$0x28, %rsp
14227f16b:     	popq	%rbx
14227f16c:     	popq	%rdi
14227f16d:     	popq	%rsi
14227f16e:     	popq	%r12
14227f170:     	popq	%r13
14227f172:     	popq	%r14
14227f174:     	popq	%r15
14227f176:     	popq	%rbp
14227f177:     	retq
14227f178:     	nopl	(%rax,%rax)
14227f180:     	movq	%rdx, 0x10(%rsp)
14227f185:     	pushq	%rbp
14227f186:     	pushq	%r15
14227f188:     	pushq	%r14
14227f18a:     	pushq	%r13
14227f18c:     	pushq	%r12
14227f18e:     	pushq	%rsi
14227f18f:     	pushq	%rdi
14227f190:     	pushq	%rbx
14227f191:     	subq	$0x28, %rsp
14227f195:     	leaq	0x80(%rdx), %rbp
14227f19c:     	movq	0xa8(%rbp), %rax
14227f1a3:     	movq	0x8(%rax), %rax
14227f1a7:     	testq	%rax, %rax
14227f1aa:     	je	0x14227f1c6 <.text+0x227e1c6>
14227f1ac:     	shlq	$0x4, %rax
14227f1b0:     	leaq	(%rax,%rax,2), %rdx
14227f1b4:     	movl	$0x8, %r8d
14227f1ba:     	movq	0xa0(%rbp), %rcx
14227f1c1:     	callq	0x140613c20 <.text+0x612c20>
14227f1c6:     	nop
14227f1c7:     	addq	$0x28, %rsp
14227f1cb:     	popq	%rbx
14227f1cc:     	popq	%rdi
14227f1cd:     	popq	%rsi
14227f1ce:     	popq	%r12
14227f1d0:     	popq	%r13
14227f1d2:     	popq	%r14
14227f1d4:     	popq	%r15
14227f1d6:     	popq	%rbp
14227f1d7:     	retq
14227f1d8:     	nopl	(%rax,%rax)
14227f1e0:     	movq	%rdx, 0x10(%rsp)
14227f1e5:     	pushq	%rbp
14227f1e6:     	pushq	%r15
14227f1e8:     	pushq	%r14
14227f1ea:     	pushq	%r13
14227f1ec:     	pushq	%r12
14227f1ee:     	pushq	%rsi
14227f1ef:     	pushq	%rdi
14227f1f0:     	pushq	%rbx
14227f1f1:     	subq	$0x28, %rsp
14227f1f5:     	leaq	0x80(%rdx), %rbp
14227f1fc:     	movq	0xa8(%rbp), %rax
14227f203:     	movq	0x8(%rax), %rax
14227f207:     	testq	%rax, %rax
14227f20a:     	je	0x14227f226 <.text+0x227e226>
14227f20c:     	shlq	$0x4, %rax
14227f210:     	leaq	(%rax,%rax,2), %rdx
14227f214:     	movl	$0x8, %r8d
14227f21a:     	movq	0xa0(%rbp), %rcx
14227f221:     	callq	0x140613c20 <.text+0x612c20>
14227f226:     	nop
14227f227:     	addq	$0x28, %rsp
14227f22b:     	popq	%rbx
14227f22c:     	popq	%rdi
14227f22d:     	popq	%rsi
14227f22e:     	popq	%r12
14227f230:     	popq	%r13
14227f232:     	popq	%r14
14227f234:     	popq	%r15
14227f236:     	popq	%rbp
14227f237:     	retq
14227f238:     	nopl	(%rax,%rax)
14227f240:     	movq	%rdx, 0x10(%rsp)
14227f245:     	pushq	%rbp
14227f246:     	pushq	%r15
14227f248:     	pushq	%r14
14227f24a:     	pushq	%r13
14227f24c:     	pushq	%r12
14227f24e:     	pushq	%rsi
14227f24f:     	pushq	%rdi
14227f250:     	pushq	%rbx
14227f251:     	subq	$0x28, %rsp
14227f255:     	leaq	0x80(%rdx), %rbp
14227f25c:     	movq	0xa8(%rbp), %rax
14227f263:     	movq	0x8(%rax), %rax
14227f267:     	testq	%rax, %rax
14227f26a:     	je	0x14227f286 <.text+0x227e286>
14227f26c:     	shlq	$0x4, %rax
14227f270:     	leaq	(%rax,%rax,2), %rdx
14227f274:     	movl	$0x8, %r8d
14227f27a:     	movq	0xa0(%rbp), %rcx
14227f281:     	callq	0x140613c20 <.text+0x612c20>
14227f286:     	nop
14227f287:     	addq	$0x28, %rsp
14227f28b:     	popq	%rbx
14227f28c:     	popq	%rdi
14227f28d:     	popq	%rsi
14227f28e:     	popq	%r12
14227f290:     	popq	%r13
14227f292:     	popq	%r14
14227f294:     	popq	%r15
14227f296:     	popq	%rbp
14227f297:     	retq
14227f298:     	nopl	(%rax,%rax)
14227f2a0:     	movq	%rdx, 0x10(%rsp)
14227f2a5:     	pushq	%rbp
14227f2a6:     	pushq	%r15
14227f2a8:     	pushq	%r14
14227f2aa:     	pushq	%r13
14227f2ac:     	pushq	%r12
14227f2ae:     	pushq	%rsi
14227f2af:     	pushq	%rdi
14227f2b0:     	pushq	%rbx
14227f2b1:     	subq	$0x28, %rsp
14227f2b5:     	leaq	0x80(%rdx), %rbp
14227f2bc:     	movups	0x38(%rbp), %xmm0
14227f2c0:     	movups	0x48(%rbp), %xmm1
14227f2c4:     	movups	0x58(%rbp), %xmm2
14227f2c8:     	movq	0xa8(%rbp), %rax
14227f2cf:     	movups	%xmm2, 0x20(%rax)
14227f2d3:     	movups	%xmm1, 0x10(%rax)
14227f2d7:     	movups	%xmm0, (%rax)
14227f2da:     	addq	$0x28, %rsp
14227f2de:     	popq	%rbx
14227f2df:     	popq	%rdi
14227f2e0:     	popq	%rsi
14227f2e1:     	popq	%r12
14227f2e3:     	popq	%r13
14227f2e5:     	popq	%r14
14227f2e7:     	popq	%r15
14227f2e9:     	popq	%rbp
14227f2ea:     	retq
14227f2eb:     	nopl	(%rax,%rax)
14227f2f0:     	movq	%rdx, 0x10(%rsp)
14227f2f5:     	pushq	%rbp
14227f2f6:     	pushq	%r15
14227f2f8:     	pushq	%r14
14227f2fa:     	pushq	%r13
14227f2fc:     	pushq	%r12
14227f2fe:     	pushq	%rsi
14227f2ff:     	pushq	%rdi
14227f300:     	pushq	%rbx
14227f301:     	subq	$0x28, %rsp
14227f305:     	leaq	0x80(%rdx), %rbp
14227f30c:     	movups	0x38(%rbp), %xmm0
14227f310:     	movups	0x48(%rbp), %xmm1
14227f314:     	movups	0x58(%rbp), %xmm2
14227f318:     	movq	0xa8(%rbp), %rax
14227f31f:     	movups	%xmm2, 0x20(%rax)
14227f323:     	movups	%xmm1, 0x10(%rax)
14227f327:     	movups	%xmm0, (%rax)
14227f32a:     	addq	$0x28, %rsp
14227f32e:     	popq	%rbx
14227f32f:     	popq	%rdi
14227f330:     	popq	%rsi
14227f331:     	popq	%r12
14227f333:     	popq	%r13
14227f335:     	popq	%r14
14227f337:     	popq	%r15
14227f339:     	popq	%rbp
14227f33a:     	retq
14227f33b:     	nopl	(%rax,%rax)
14227f340:     	movq	%rdx, 0x10(%rsp)
14227f345:     	pushq	%rbp
14227f346:     	pushq	%r15
14227f348:     	pushq	%r14
14227f34a:     	pushq	%r13
14227f34c:     	pushq	%r12
14227f34e:     	pushq	%rsi
14227f34f:     	pushq	%rdi
14227f350:     	pushq	%rbx
14227f351:     	subq	$0x28, %rsp
14227f355:     	leaq	0x80(%rdx), %rbp
14227f35c:     	movups	0x38(%rbp), %xmm0
14227f360:     	movups	0x48(%rbp), %xmm1
14227f364:     	movups	0x58(%rbp), %xmm2
14227f368:     	movq	0xa8(%rbp), %rax
14227f36f:     	movups	%xmm2, 0x20(%rax)
14227f373:     	movups	%xmm1, 0x10(%rax)
14227f377:     	movups	%xmm0, (%rax)
14227f37a:     	addq	$0x28, %rsp
14227f37e:     	popq	%rbx
14227f37f:     	popq	%rdi
14227f380:     	popq	%rsi
14227f381:     	popq	%r12
14227f383:     	popq	%r13
14227f385:     	popq	%r14
14227f387:     	popq	%r15
14227f389:     	popq	%rbp
14227f38a:     	retq
14227f38b:     	nopl	(%rax,%rax)
14227f390:     	movq	%rdx, 0x10(%rsp)
14227f395:     	pushq	%rbp
14227f396:     	pushq	%r15
14227f398:     	pushq	%r14
14227f39a:     	pushq	%r13
14227f39c:     	pushq	%r12
14227f39e:     	pushq	%rsi
14227f39f:     	pushq	%rdi
14227f3a0:     	pushq	%rbx
14227f3a1:     	subq	$0x28, %rsp
14227f3a5:     	leaq	0x80(%rdx), %rbp
14227f3ac:     	leaq	0x80(%rbp), %rcx
14227f3b3:     	callq	0x140ea8740 <.text+0xea7740>
14227f3b8:     	nop
14227f3b9:     	addq	$0x28, %rsp
14227f3bd:     	popq	%rbx
14227f3be:     	popq	%rdi
14227f3bf:     	popq	%rsi
14227f3c0:     	popq	%r12
14227f3c2:     	popq	%r13
14227f3c4:     	popq	%r14
14227f3c6:     	popq	%r15
14227f3c8:     	popq	%rbp
14227f3c9:     	retq
14227f3ca:     	nopw	(%rax,%rax)
14227f3d0:     	movq	%rdx, 0x10(%rsp)
14227f3d5:     	pushq	%rbp
14227f3d6:     	pushq	%r15
14227f3d8:     	pushq	%r14
14227f3da:     	pushq	%r13
14227f3dc:     	pushq	%r12
14227f3de:     	pushq	%rsi
14227f3df:     	pushq	%rdi
14227f3e0:     	pushq	%rbx
14227f3e1:     	subq	$0x28, %rsp
14227f3e5:     	leaq	0x80(%rdx), %rbp
14227f3ec:     	leaq	0x80(%rbp), %rcx
14227f3f3:     	callq	0x140ea8740 <.text+0xea7740>
14227f3f8:     	nop
14227f3f9:     	addq	$0x28, %rsp
14227f3fd:     	popq	%rbx
14227f3fe:     	popq	%rdi
14227f3ff:     	popq	%rsi
14227f400:     	popq	%r12
14227f402:     	popq	%r13
14227f404:     	popq	%r14
14227f406:     	popq	%r15
14227f408:     	popq	%rbp
14227f409:     	retq
14227f40a:     	nopw	(%rax,%rax)
14227f410:     	movq	%rdx, 0x10(%rsp)
14227f415:     	pushq	%rbp
14227f416:     	pushq	%r15
14227f418:     	pushq	%r14
14227f41a:     	pushq	%r13
14227f41c:     	pushq	%r12
14227f41e:     	pushq	%rsi
14227f41f:     	pushq	%rdi
14227f420:     	pushq	%rbx
14227f421:     	subq	$0x28, %rsp
14227f425:     	leaq	0x80(%rdx), %rbp
14227f42c:     	leaq	0x80(%rbp), %rcx
14227f433:     	callq	0x140ea8740 <.text+0xea7740>
14227f438:     	nop
14227f439:     	addq	$0x28, %rsp
14227f43d:     	popq	%rbx
14227f43e:     	popq	%rdi
14227f43f:     	popq	%rsi
14227f440:     	popq	%r12
14227f442:     	popq	%r13
14227f444:     	popq	%r14
14227f446:     	popq	%r15
14227f448:     	popq	%rbp
14227f449:     	retq
14227f44a:     	nopw	(%rax,%rax)
14227f450:     	movq	%rdx, 0x10(%rsp)
14227f455:     	pushq	%rbp
14227f456:     	pushq	%r15
14227f458:     	pushq	%r14
14227f45a:     	pushq	%r13
14227f45c:     	pushq	%r12
14227f45e:     	pushq	%rsi
14227f45f:     	pushq	%rdi
14227f460:     	pushq	%rbx
14227f461:     	subq	$0x28, %rsp
14227f465:     	leaq	0x80(%rdx), %rbp
14227f46c:     	movq	0x68(%rbp), %rax
14227f470:     	addb	$0x36, %al
14227f472:     	cmpb	$0x3, %al
14227f474:     	jae	0x14227f487 <.text+0x227e487>
14227f476:     	addq	$0x28, %rsp
14227f47a:     	popq	%rbx
14227f47b:     	popq	%rdi
14227f47c:     	popq	%rsi
14227f47d:     	popq	%r12
14227f47f:     	popq	%r13
14227f481:     	popq	%r14
14227f483:     	popq	%r15
14227f485:     	popq	%rbp
14227f486:     	retq
14227f487:     	movq	0x98(%rbp), %rcx
14227f48e:     	callq	0x14227e420 <.text+0x227d420>
14227f493:     	jmp	0x14227f476 <.text+0x227e476>
14227f495:     	int3
14227f496:     	int3
14227f497:     	int3
14227f498:     	int3
14227f499:     	int3
14227f49a:     	int3
14227f49b:     	int3
14227f49c:     	int3
14227f49d:     	int3
14227f49e:     	int3
14227f49f:     	int3
