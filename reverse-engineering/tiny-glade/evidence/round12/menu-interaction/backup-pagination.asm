
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f9b5c0:     	pushq	%rbp
140f9b5c1:     	pushq	%r15
140f9b5c3:     	pushq	%r14
140f9b5c5:     	pushq	%r13
140f9b5c7:     	pushq	%r12
140f9b5c9:     	pushq	%rsi
140f9b5ca:     	pushq	%rdi
140f9b5cb:     	pushq	%rbx
140f9b5cc:     	subq	$0xc8, %rsp
140f9b5d3:     	leaq	0x80(%rsp), %rbp
140f9b5db:     	movq	$-0x2, 0x40(%rbp)
140f9b5e3:     	movq	%r8, %rbx
140f9b5e6:     	movq	%rdx, %rdi
140f9b5e9:     	movq	%rcx, %rsi
140f9b5ec:     	movq	(%r9), %r15
140f9b5ef:     	leaq	0x1c150ec(%rip), %r8    # 0x142bb06e2
140f9b5f6:     	leaq	-0x10(%rbp), %r14
140f9b5fa:     	movl	$0x3, %r9d
140f9b600:     	movq	%r14, %rcx
140f9b603:     	callq	0x14068bfc0 <.text+0x68afc0>
140f9b608:     	movq	%r14, %rcx
140f9b60b:     	movb	$0x1, %dl
140f9b60d:     	callq	0x140f9dba0 <.text+0xf9cba0>
140f9b612:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
140f9b61c:     	movdqu	-0x10(%rbp), %xmm0
140f9b621:     	movdqa	%xmm0, 0x20(%rbp)
140f9b626:     	movq	(%rbp), %rax
140f9b62a:     	movq	%rax, 0x30(%rbp)
140f9b62e:     	movq	0x20(%rbp), %r14
140f9b632:     	cmpq	$0x0, 0x1b8(%r14)
140f9b63a:     	je	0x140f9b97e <.text+0xf9a97e>
140f9b640:     	leaq	0x28(%rbp), %rdx
140f9b644:     	leaq	0x1c0(%r14), %rcx
140f9b64b:     	callq	0x1405b0d70 <.text+0x5afd70>
140f9b650:     	movq	0x1a0(%r14), %rcx
140f9b657:     	movq	0x1a8(%r14), %rdx
140f9b65e:     	movq	%rdx, %r8
140f9b661:     	andq	%rax, %r8
140f9b664:     	shrq	$0x39, %rax
140f9b668:     	movd	%eax, %xmm0
140f9b66c:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140f9b670:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140f9b675:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140f9b67a:     	leaq	-0xc0(%rcx), %rax
140f9b681:     	xorl	%r9d, %r9d
140f9b684:     	movq	0x28(%rbp), %r10
140f9b688:     	pcmpeqd	%xmm1, %xmm1
140f9b68c:     	movdqu	(%rcx,%r8), %xmm2
140f9b692:     	movdqa	%xmm2, %xmm3
140f9b696:     	pcmpeqb	%xmm0, %xmm3
140f9b69a:     	pmovmskb	%xmm3, %r11d
140f9b69f:     	testl	%r11d, %r11d
140f9b6a2:     	je	0x140f9b6d0 <.text+0xf9a6d0>
140f9b6a4:     	tzcntl	%r11d, %r14d
140f9b6a9:     	addq	%r8, %r14
140f9b6ac:     	andq	%rdx, %r14
140f9b6af:     	negq	%r14
140f9b6b2:     	leaq	(%r14,%r14,2), %r14
140f9b6b6:     	shlq	$0x6, %r14
140f9b6ba:     	cmpq	(%rax,%r14), %r10
140f9b6be:     	je	0x140f9b6f2 <.text+0xf9a6f2>
140f9b6c0:     	leal	-0x1(%r11), %r14d
140f9b6c4:     	andw	%r11w, %r14w
140f9b6c8:     	movl	%r14d, %r11d
140f9b6cb:     	jne	0x140f9b6a4 <.text+0xf9a6a4>
140f9b6cd:     	nopl	(%rax)
140f9b6d0:     	pcmpeqb	%xmm1, %xmm2
140f9b6d4:     	pmovmskb	%xmm2, %r11d
140f9b6d9:     	testl	%r11d, %r11d
140f9b6dc:     	jne	0x140f9b97e <.text+0xf9a97e>
140f9b6e2:     	addq	%r9, %r8
140f9b6e5:     	addq	$0x10, %r8
140f9b6e9:     	addq	$0x10, %r9
140f9b6ed:     	andq	%rdx, %r8
140f9b6f0:     	jmp	0x140f9b68c <.text+0xf9a68c>
140f9b6f2:     	testb	$0x30, -0x2(%rcx,%r14)
140f9b6f8:     	movq	(%rbx), %r12
140f9b6fb:     	je	0x140f9b70d <.text+0xf9a70d>
140f9b6fd:     	testq	%r12, %r12
140f9b700:     	je	0x140f9b70a <.text+0xf9a70a>
140f9b702:     	decq	%r12
140f9b705:     	movq	%r12, (%rbx)
140f9b708:     	jmp	0x140f9b70d <.text+0xf9a70d>
140f9b70a:     	xorl	%r12d, %r12d
140f9b70d:     	leaq	0x1(%r12), %r13
140f9b712:     	movq	%r13, -0x30(%rbp)
140f9b716:     	leaq	0x1(%r15), %rax
140f9b71a:     	movq	%rax, -0x28(%rbp)
140f9b71e:     	leaq	-0x30(%rbp), %rax
140f9b722:     	movq	%rax, 0x20(%rbp)
140f9b726:     	leaq	-0x20e06d(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
140f9b72d:     	movq	%rax, 0x28(%rbp)
140f9b731:     	leaq	-0x28(%rbp), %rcx
140f9b735:     	movq	%rcx, 0x30(%rbp)
140f9b739:     	movq	%rax, 0x38(%rbp)
140f9b73d:     	leaq	0x1c14fa4(%rip), %rax   # 0x142bb06e8
140f9b744:     	movq	%rax, -0x10(%rbp)
140f9b748:     	movq	$0x2, -0x8(%rbp)
140f9b750:     	leaq	0x20(%rbp), %rax
140f9b754:     	movq	%rax, (%rbp)
140f9b758:     	movq	$0x2, 0x8(%rbp)
140f9b760:     	movq	$0x0, 0x10(%rbp)
140f9b768:     	leaq	-0x48(%rbp), %rcx
140f9b76c:     	leaq	-0x10(%rbp), %rdx
140f9b770:     	callq	0x140d79f90 <.text+0xd78f90>
140f9b775:     	movq	-0x40(%rbp), %rax
140f9b779:     	movq	-0x38(%rbp), %r14
140f9b77d:     	movq	%rax, -0x18(%rbp)
140f9b781:     	movq	%rax, -0x28(%rbp)
140f9b785:     	movq	%r14, -0x20(%rbp)
140f9b789:     	leaq	-0x28(%rbp), %rcx
140f9b78d:     	callq	0x1405bdda0 <.text+0x5bcda0>
140f9b792:     	movq	%rax, -0x10(%rbp)
140f9b796:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140f9b7a0:     	movq	%rax, -0x8(%rbp)
140f9b7a4:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140f9b7ae:     	movq	%rax, (%rbp)
140f9b7b2:     	leaq	-0x10(%rbp), %rcx
140f9b7b6:     	callq	0x1405bdf20 <.text+0x5bcf20>
140f9b7bb:     	movq	0x8(%rdi), %rcx
140f9b7bf:     	movq	%rcx, -0x10(%rbp)
140f9b7c3:     	movq	%rax, -0x8(%rbp)
140f9b7c7:     	leaq	-0x10(%rbp), %rcx
140f9b7cb:     	callq	0x1405bd910 <.text+0x5bc910>
140f9b7d0:     	leaq	-0x10(%rbp), %rcx
140f9b7d4:     	movq	%rdi, %rdx
140f9b7d7:     	movq	%rax, %r8
140f9b7da:     	callq	0x1406861b0 <.text+0x6851b0>
140f9b7df:     	leaq	-0x10(%rbp), %rcx
140f9b7e3:     	xorl	%edx, %edx
140f9b7e5:     	callq	0x140f9d930 <.text+0xf9c930>
140f9b7ea:     	movq	%rax, %rcx
140f9b7ed:     	movq	-0x18(%rbp), %rdx
140f9b7f1:     	movq	%r14, %r8
140f9b7f4:     	callq	0x140fac4c0 <.text+0xfab4c0>
140f9b7f9:     	movq	(%rbp), %rax
140f9b7fd:     	movq	%rax, 0x30(%rbp)
140f9b801:     	movdqu	-0x10(%rbp), %xmm0
140f9b806:     	movdqa	%xmm0, 0x20(%rbp)
140f9b80b:     	leaq	0x20(%rbp), %rcx
140f9b80f:     	movb	$0x1, %dl
140f9b811:     	callq	0x140f9d930 <.text+0xf9c930>
140f9b816:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
140f9b820:     	movq	0x30(%rbp), %rax
140f9b824:     	movq	%rax, (%rbp)
140f9b828:     	movdqa	0x20(%rbp), %xmm0
140f9b82d:     	movdqa	%xmm0, -0x10(%rbp)
140f9b832:     	leaq	-0x10(%rbp), %rcx
140f9b836:     	movb	$0x1, %dl
140f9b838:     	callq	0x140f9d930 <.text+0xf9c930>
140f9b83d:     	movd	0x1a7b5fb(%rip), %xmm1  # 0x142a16e40
140f9b845:     	movq	%rax, %rcx
140f9b848:     	callq	0x140fac150 <.text+0xfab150>
140f9b84d:     	movq	(%rbp), %rax
140f9b851:     	movq	%rax, 0x30(%rbp)
140f9b855:     	movdqa	-0x10(%rbp), %xmm0
140f9b85a:     	movdqa	%xmm0, 0x20(%rbp)
140f9b85f:     	leaq	0x20(%rbp), %rcx
140f9b863:     	movb	$0x1, %dl
140f9b865:     	callq	0x140f9d930 <.text+0xf9c930>
140f9b86a:     	movq	%rax, %rcx
140f9b86d:     	movw	$0x2bc, %dx             # imm = 0x2BC
140f9b871:     	callq	0x140b90ea0 <.text+0xb8fea0>
140f9b876:     	movq	-0x48(%rbp), %rdx
140f9b87a:     	testq	%rdx, %rdx
140f9b87d:     	je	0x140f9b88e <.text+0xf9a88e>
140f9b87f:     	movl	$0x1, %r8d
140f9b885:     	movq	-0x18(%rbp), %rcx
140f9b889:     	callq	0x140613c20 <.text+0x612c20>
140f9b88e:     	leaq	0x1c14e73(%rip), %r8    # 0x142bb0708
140f9b895:     	leaq	-0x60(%rbp), %r14
140f9b899:     	movl	$0x3, %r9d
140f9b89f:     	movq	%r14, %rcx
140f9b8a2:     	movq	%rdi, %rdx
140f9b8a5:     	callq	0x14068bfc0 <.text+0x68afc0>
140f9b8aa:     	movq	%r14, %rcx
140f9b8ad:     	movb	$0x1, %dl
140f9b8af:     	callq	0x140f9dba0 <.text+0xf9cba0>
140f9b8b4:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
140f9b8be:     	movq	-0x60(%rbp), %r14
140f9b8c2:     	cmpq	$0x0, 0x1b8(%r14)
140f9b8ca:     	je	0x140f9b97e <.text+0xf9a97e>
140f9b8d0:     	leaq	-0x58(%rbp), %rdx
140f9b8d4:     	leaq	0x1c0(%r14), %rcx
140f9b8db:     	callq	0x1405b0d70 <.text+0x5afd70>
140f9b8e0:     	movq	0x1a0(%r14), %rcx
140f9b8e7:     	movq	0x1a8(%r14), %rdx
140f9b8ee:     	movq	%rdx, %r8
140f9b8f1:     	andq	%rax, %r8
140f9b8f4:     	shrq	$0x39, %rax
140f9b8f8:     	movd	%eax, %xmm0
140f9b8fc:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140f9b900:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140f9b905:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140f9b90a:     	leaq	-0xc0(%rcx), %rax
140f9b911:     	xorl	%r9d, %r9d
140f9b914:     	movq	-0x58(%rbp), %r10
140f9b918:     	pcmpeqd	%xmm1, %xmm1
140f9b91c:     	movdqu	(%rcx,%r8), %xmm2
140f9b922:     	movdqa	%xmm2, %xmm3
140f9b926:     	pcmpeqb	%xmm0, %xmm3
140f9b92a:     	pmovmskb	%xmm3, %r11d
140f9b92f:     	testl	%r11d, %r11d
140f9b932:     	je	0x140f9b960 <.text+0xf9a960>
140f9b934:     	tzcntl	%r11d, %r14d
140f9b939:     	addq	%r8, %r14
140f9b93c:     	andq	%rdx, %r14
140f9b93f:     	negq	%r14
140f9b942:     	leaq	(%r14,%r14,2), %r14
140f9b946:     	shlq	$0x6, %r14
140f9b94a:     	cmpq	(%rax,%r14), %r10
140f9b94e:     	je	0x140f9b996 <.text+0xf9a996>
140f9b950:     	leal	-0x1(%r11), %r14d
140f9b954:     	andw	%r11w, %r14w
140f9b958:     	movl	%r14d, %r11d
140f9b95b:     	jne	0x140f9b934 <.text+0xf9a934>
140f9b95d:     	nopl	(%rax)
140f9b960:     	pcmpeqb	%xmm1, %xmm2
140f9b964:     	pmovmskb	%xmm2, %r11d
140f9b969:     	testl	%r11d, %r11d
140f9b96c:     	jne	0x140f9b97e <.text+0xf9a97e>
140f9b96e:     	addq	%r9, %r8
140f9b971:     	addq	$0x10, %r8
140f9b975:     	addq	$0x10, %r9
140f9b979:     	andq	%rdx, %r8
140f9b97c:     	jmp	0x140f9b91c <.text+0xf9a91c>
140f9b97e:     	leaq	0x1a7b883(%rip), %rcx   # 0x142a17208
140f9b985:     	leaq	0x1a7c64c(%rip), %r8    # 0x142a17fd8
140f9b98c:     	movl	$0x16, %edx
140f9b991:     	callq	0x1428d9330 <.text+0x28d8330>
140f9b996:     	testb	$0x30, -0x2(%rcx,%r14)
140f9b99c:     	setne	%al
140f9b99f:     	cmpq	%r15, %r12
140f9b9a2:     	setb	%cl
140f9b9a5:     	andb	%al, %cl
140f9b9a7:     	cmpb	$0x1, %cl
140f9b9aa:     	jne	0x140f9b9af <.text+0xf9a9af>
140f9b9ac:     	movq	%r13, (%rbx)
140f9b9af:     	movq	0x10(%rdi), %rax
140f9b9b3:     	movq	%rax, 0x10(%rsi)
140f9b9b7:     	movups	(%rdi), %xmm0
140f9b9ba:     	movups	%xmm0, (%rsi)
140f9b9bd:     	movq	%rsi, %rax
140f9b9c0:     	addq	$0xc8, %rsp
140f9b9c7:     	popq	%rbx
140f9b9c8:     	popq	%rdi
140f9b9c9:     	popq	%rsi
140f9b9ca:     	popq	%r12
140f9b9cc:     	popq	%r13
140f9b9ce:     	popq	%r14
140f9b9d0:     	popq	%r15
140f9b9d2:     	popq	%rbp
140f9b9d3:     	retq
140f9b9d4:     	nopw	%cs:(%rax,%rax)
140f9b9e0:     	movq	%rdx, 0x10(%rsp)
140f9b9e5:     	pushq	%rbp
140f9b9e6:     	pushq	%r15
140f9b9e8:     	pushq	%r14
140f9b9ea:     	pushq	%r13
140f9b9ec:     	pushq	%r12
140f9b9ee:     	pushq	%rsi
140f9b9ef:     	pushq	%rdi
140f9b9f0:     	pushq	%rbx
140f9b9f1:     	subq	$0x28, %rsp
140f9b9f5:     	leaq	0x80(%rdx), %rbp
140f9b9fc:     	movq	-0x48(%rbp), %rdx
140f9ba00:     	testq	%rdx, %rdx
140f9ba03:     	je	0x140f9ba14 <.text+0xf9aa14>
140f9ba05:     	movl	$0x1, %r8d
140f9ba0b:     	movq	-0x18(%rbp), %rcx
140f9ba0f:     	callq	0x140613c20 <.text+0x612c20>
140f9ba14:     	nop
140f9ba15:     	addq	$0x28, %rsp
140f9ba19:     	popq	%rbx
140f9ba1a:     	popq	%rdi
140f9ba1b:     	popq	%rsi
140f9ba1c:     	popq	%r12
140f9ba1e:     	popq	%r13
140f9ba20:     	popq	%r14
140f9ba22:     	popq	%r15
140f9ba24:     	popq	%rbp
140f9ba25:     	retq
