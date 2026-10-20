
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f9d590:     	pushq	%rbp
140f9d591:     	pushq	%r14
140f9d593:     	pushq	%rsi
140f9d594:     	pushq	%rdi
140f9d595:     	pushq	%rbx
140f9d596:     	subq	$0xb0, %rsp
140f9d59d:     	leaq	0x80(%rsp), %rbp
140f9d5a5:     	movq	$-0x2, 0x28(%rbp)
140f9d5ad:     	movq	(%rcx), %rdi
140f9d5b0:     	cmpq	$0x0, 0x1b8(%rdi)
140f9d5b8:     	je	0x140f9d685 <.text+0xf9c685>
140f9d5be:     	movl	%edx, %ebx
140f9d5c0:     	movq	%rcx, %rsi
140f9d5c3:     	leaq	0x8(%rcx), %rdx
140f9d5c7:     	leaq	0x1c0(%rdi), %rcx
140f9d5ce:     	callq	0x1405b0d70 <.text+0x5afd70>
140f9d5d3:     	movq	0x1a0(%rdi), %rcx
140f9d5da:     	movq	0x1a8(%rdi), %rdx
140f9d5e1:     	movq	%rdx, %r8
140f9d5e4:     	andq	%rax, %r8
140f9d5e7:     	shrq	$0x39, %rax
140f9d5eb:     	movd	%eax, %xmm0
140f9d5ef:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140f9d5f3:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140f9d5f8:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140f9d5fd:     	leaq	-0xc0(%rcx), %rax
140f9d604:     	movq	0x8(%rsi), %r9
140f9d608:     	xorl	%r10d, %r10d
140f9d60b:     	pcmpeqd	%xmm1, %xmm1
140f9d60f:     	movdqu	(%rcx,%r8), %xmm2
140f9d615:     	movdqa	%xmm2, %xmm3
140f9d619:     	pcmpeqb	%xmm0, %xmm3
140f9d61d:     	pmovmskb	%xmm3, %r11d
140f9d622:     	testl	%r11d, %r11d
140f9d625:     	je	0x140f9d650 <.text+0xf9c650>
140f9d627:     	tzcntl	%r11d, %esi
140f9d62c:     	addq	%r8, %rsi
140f9d62f:     	andq	%rdx, %rsi
140f9d632:     	negq	%rsi
140f9d635:     	leaq	(%rsi,%rsi,2), %r14
140f9d639:     	shlq	$0x6, %r14
140f9d63d:     	cmpq	(%rax,%r14), %r9
140f9d641:     	je	0x140f9d66e <.text+0xf9c66e>
140f9d643:     	leal	-0x1(%r11), %esi
140f9d647:     	andw	%r11w, %si
140f9d64b:     	movl	%esi, %r11d
140f9d64e:     	jne	0x140f9d627 <.text+0xf9c627>
140f9d650:     	pcmpeqb	%xmm1, %xmm2
140f9d654:     	pmovmskb	%xmm2, %r11d
140f9d659:     	testl	%r11d, %r11d
140f9d65c:     	jne	0x140f9d685 <.text+0xf9c685>
140f9d65e:     	addq	%r10, %r8
140f9d661:     	addq	$0x10, %r8
140f9d665:     	addq	$0x10, %r10
140f9d669:     	andq	%rdx, %r8
140f9d66c:     	jmp	0x140f9d60f <.text+0xf9c60f>
140f9d66e:     	leaq	(%rcx,%r14), %rsi
140f9d672:     	movq	-0x38(%rcx,%r14), %rcx
140f9d677:     	testq	%rcx, %rcx
140f9d67a:     	je	0x140f9d69d <.text+0xf9c69d>
140f9d67c:     	movq	-0x30(%rsi), %rax
140f9d680:     	jmp	0x140f9d802 <.text+0xf9c802>
140f9d685:     	leaq	0x1c119b4(%rip), %rcx   # 0x142baf040
140f9d68c:     	leaq	0x1c119bd(%rip), %r8    # 0x142baf050
140f9d693:     	movl	$0xb, %edx
140f9d698:     	callq	0x1428d9330 <.text+0x28d8330>
140f9d69d:     	testb	%bl, %bl
140f9d69f:     	jne	0x140f9d842 <.text+0xf9c842>
140f9d6a5:     	movq	0x260(%rdi), %rdi
140f9d6ac:     	lock
140f9d6ad:     	incq	(%rdi)
140f9d6b0:     	jle	0x140f9d8ae <.text+0xf9c8ae>
140f9d6b6:     	movq	%rdi, 0x20(%rbp)
140f9d6ba:     	movzbl	0x2697548(%rip), %eax   # 0x143634c09
140f9d6c1:     	movl	$0x48, %ecx
140f9d6c6:     	movl	$0x8, %edx
140f9d6cb:     	callq	0x140613c10 <.text+0x612c10>
140f9d6d0:     	testq	%rax, %rax
140f9d6d3:     	je	0x140f9d88e <.text+0xf9c88e>
140f9d6d9:     	pxor	%xmm0, %xmm0
140f9d6dd:     	movdqu	%xmm0, -0x3b(%rbp)
140f9d6e2:     	movdqu	%xmm0, -0x44(%rbp)
140f9d6e7:     	movdqu	%xmm0, -0x54(%rbp)
140f9d6ec:     	movq	$0x1, (%rax)
140f9d6f3:     	movq	$0x1, 0x8(%rax)
140f9d6fb:     	movb	$0x0, 0x10(%rax)
140f9d6ff:     	movl	$0x0, 0x14(%rax)
140f9d706:     	movups	-0x58(%rbp), %xmm1
140f9d70a:     	movdqu	-0x48(%rbp), %xmm2
140f9d70f:     	movdqu	-0x3b(%rbp), %xmm3
140f9d714:     	movdqu	%xmm3, 0x35(%rax)
140f9d719:     	movdqu	%xmm2, 0x28(%rax)
140f9d71e:     	movups	%xmm1, 0x18(%rax)
140f9d722:     	movq	$0x0, -0x18(%rbp)
140f9d72a:     	movl	$0x1, 0x1c(%rbp)
140f9d731:     	movl	$0x1, (%rbp)
140f9d738:     	movq	$0x0, -0x58(%rbp)
140f9d740:     	movdqa	0x1c10e98(%rip), %xmm1  # 0x142bae5e0
140f9d748:     	movdqu	%xmm1, -0x10(%rbp)
140f9d74d:     	movq	$0x0, -0x48(%rbp)
140f9d755:     	movabsq	$0x3f80000000000000, %rcx # imm = 0x3F80000000000000
140f9d75f:     	movq	%rcx, 0x14(%rbp)
140f9d763:     	movdqu	%xmm0, 0x4(%rbp)
140f9d768:     	movq	%rax, -0x20(%rbp)
140f9d76c:     	movl	$0x0, -0x28(%rbp)
140f9d773:     	movq	$0x0, -0x38(%rbp)
140f9d77b:     	lock
140f9d77c:     	decq	(%rdi)
140f9d77f:     	jne	0x140f9d78a <.text+0xf9c78a>
140f9d781:     	leaq	0x20(%rbp), %rcx
140f9d785:     	callq	0x140b8e120 <.text+0xb8d120>
140f9d78a:     	movzbl	0x2697478(%rip), %eax   # 0x143634c09
140f9d791:     	movl	$0x78, %ecx
140f9d796:     	movl	$0x8, %edx
140f9d79b:     	callq	0x140613c10 <.text+0x612c10>
140f9d7a0:     	testq	%rax, %rax
140f9d7a3:     	je	0x140f9d89f <.text+0xf9c89f>
140f9d7a9:     	movq	%rax, %rcx
140f9d7ac:     	movq	0x18(%rbp), %rax
140f9d7b0:     	movq	%rax, 0x70(%rcx)
140f9d7b4:     	movups	0x8(%rbp), %xmm0
140f9d7b8:     	movups	%xmm0, 0x60(%rcx)
140f9d7bc:     	movups	-0x8(%rbp), %xmm0
140f9d7c0:     	movups	%xmm0, 0x50(%rcx)
140f9d7c4:     	movups	-0x18(%rbp), %xmm0
140f9d7c8:     	movups	%xmm0, 0x40(%rcx)
140f9d7cc:     	movdqu	-0x58(%rbp), %xmm0
140f9d7d1:     	movdqu	-0x48(%rbp), %xmm1
140f9d7d6:     	movdqu	-0x38(%rbp), %xmm2
140f9d7db:     	movdqu	-0x28(%rbp), %xmm3
140f9d7e0:     	movdqu	%xmm3, 0x30(%rcx)
140f9d7e5:     	movdqu	%xmm2, 0x20(%rcx)
140f9d7ea:     	movdqu	%xmm1, 0x10(%rcx)
140f9d7ef:     	movdqu	%xmm0, (%rcx)
140f9d7f3:     	movq	%rcx, -0x38(%rsi)
140f9d7f7:     	leaq	0x1c11cf2(%rip), %rax   # 0x142baf4f0
140f9d7fe:     	movq	%rax, -0x30(%rsi)
140f9d802:     	callq	*0x28(%rax)
140f9d805:     	movq	%rax, %rsi
140f9d808:     	movq	%rax, %rcx
140f9d80b:     	callq	*0x18(%rdx)
140f9d80e:     	testq	%rsi, %rsi
140f9d811:     	je	0x140f9d876 <.text+0xf9c876>
140f9d813:     	movabsq	$-0xac5969bfd58541b, %r8 # imm = 0xF53A696402A7ABE5
140f9d81d:     	cmpq	%r8, %rax
140f9d820:     	jne	0x140f9d876 <.text+0xf9c876>
140f9d822:     	movabsq	$-0x51a045a1c4472226, %rcx # imm = 0xAE5FBA5E3BB8DDDA
140f9d82c:     	cmpq	%rcx, %rdx
140f9d82f:     	jne	0x140f9d876 <.text+0xf9c876>
140f9d831:     	movq	%rsi, %rax
140f9d834:     	addq	$0xb0, %rsp
140f9d83b:     	popq	%rbx
140f9d83c:     	popq	%rdi
140f9d83d:     	popq	%rsi
140f9d83e:     	popq	%r14
140f9d840:     	popq	%rbp
140f9d841:     	retq
140f9d842:     	leaq	0x1c1192f(%rip), %rax   # 0x142baf178
140f9d849:     	movq	%rax, -0x58(%rbp)
140f9d84d:     	movq	$0x1, -0x50(%rbp)
140f9d855:     	movq	$0x8, -0x48(%rbp)
140f9d85d:     	pxor	%xmm0, %xmm0
140f9d861:     	movdqu	%xmm0, -0x40(%rbp)
140f9d866:     	leaq	0x1c1191b(%rip), %rdx   # 0x142baf188
140f9d86d:     	leaq	-0x58(%rbp), %rcx
140f9d871:     	callq	0x1428d9390 <.text+0x28d8390>
140f9d876:     	leaq	0x1c117eb(%rip), %rcx   # 0x142baf068
140f9d87d:     	leaq	0x1c117ec(%rip), %r8    # 0x142baf070
140f9d884:     	movl	$0x8, %edx
140f9d889:     	callq	0x1428d9330 <.text+0x28d8330>
140f9d88e:     	movl	$0x8, %ecx
140f9d893:     	movl	$0x48, %edx
140f9d898:     	callq	0x1428d9000 <.text+0x28d8000>
140f9d89d:     	jmp	0x140f9d8ae <.text+0xf9c8ae>
140f9d89f:     	movl	$0x8, %ecx
140f9d8a4:     	movl	$0x78, %edx
140f9d8a9:     	callq	0x1428d9000 <.text+0x28d8000>
140f9d8ae:     	ud2
140f9d8b0:     	movq	%rdx, 0x10(%rsp)
140f9d8b5:     	pushq	%rbp
140f9d8b6:     	pushq	%r14
140f9d8b8:     	pushq	%rsi
140f9d8b9:     	pushq	%rdi
140f9d8ba:     	pushq	%rbx
140f9d8bb:     	subq	$0x20, %rsp
140f9d8bf:     	leaq	0x80(%rdx), %rbp
140f9d8c6:     	movq	0x20(%rbp), %rax
140f9d8ca:     	lock
140f9d8cb:     	decq	(%rax)
140f9d8ce:     	jne	0x140f9d8d9 <.text+0xf9c8d9>
140f9d8d0:     	leaq	0x20(%rbp), %rcx
140f9d8d4:     	callq	0x140b8e120 <.text+0xb8d120>
140f9d8d9:     	nop
140f9d8da:     	addq	$0x20, %rsp
140f9d8de:     	popq	%rbx
140f9d8df:     	popq	%rdi
140f9d8e0:     	popq	%rsi
140f9d8e1:     	popq	%r14
140f9d8e3:     	popq	%rbp
140f9d8e4:     	retq
140f9d8e5:     	nopw	%cs:(%rax,%rax)
140f9d8f0:     	movq	%rdx, 0x10(%rsp)
140f9d8f5:     	pushq	%rbp
140f9d8f6:     	pushq	%r14
140f9d8f8:     	pushq	%rsi
140f9d8f9:     	pushq	%rdi
140f9d8fa:     	pushq	%rbx
140f9d8fb:     	subq	$0x20, %rsp
140f9d8ff:     	leaq	0x80(%rdx), %rbp
140f9d906:     	movq	-0x20(%rbp), %rax
140f9d90a:     	lock
140f9d90b:     	decq	(%rax)
140f9d90e:     	jne	0x140f9d919 <.text+0xf9c919>
140f9d910:     	leaq	-0x20(%rbp), %rcx
140f9d914:     	callq	0x140b5d090 <.text+0xb5c090>
140f9d919:     	nop
140f9d91a:     	addq	$0x20, %rsp
140f9d91e:     	popq	%rbx
140f9d91f:     	popq	%rdi
140f9d920:     	popq	%rsi
140f9d921:     	popq	%r14
140f9d923:     	popq	%rbp
140f9d924:     	retq
