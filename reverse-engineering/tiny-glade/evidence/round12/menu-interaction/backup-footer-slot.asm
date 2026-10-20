
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fc66a0:     	pushq	%r14
140fc66a2:     	pushq	%rsi
140fc66a3:     	pushq	%rdi
140fc66a4:     	pushq	%rbp
140fc66a5:     	pushq	%rbx
140fc66a6:     	subq	$0x60, %rsp
140fc66aa:     	movq	%rdx, %rdi
140fc66ad:     	movq	%rcx, %rsi
140fc66b0:     	movq	(%rcx), %rbx
140fc66b3:     	cmpq	$0x0, (%rbx)
140fc66b7:     	je	0x140fc6867 <.text+0xfc5867>
140fc66bd:     	movq	0x8(%rdi), %r14
140fc66c1:     	leaq	0x1bedc70(%rip), %rax   # 0x142bb4338
140fc66c8:     	movq	%rax, 0x40(%rsp)
140fc66cd:     	movq	$0xa, 0x48(%rsp)
140fc66d6:     	leaq	0x40(%rsp), %rcx
140fc66db:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fc66e0:     	movq	%rax, 0x20(%rsp)
140fc66e5:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140fc66ef:     	movq	%rax, 0x28(%rsp)
140fc66f4:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
140fc66fe:     	movq	%rax, 0x30(%rsp)
140fc6703:     	leaq	0x20(%rsp), %rcx
140fc6708:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fc670d:     	movq	%r14, 0x20(%rsp)
140fc6712:     	movq	%rax, 0x28(%rsp)
140fc6717:     	leaq	0x20(%rsp), %rcx
140fc671c:     	callq	0x1405bd910 <.text+0x5bc910>
140fc6721:     	leaq	0x20(%rsp), %rcx
140fc6726:     	movq	%rdi, %rdx
140fc6729:     	movq	%rax, %r8
140fc672c:     	callq	0x1406861b0 <.text+0x6851b0>
140fc6731:     	movq	0x20(%rsp), %r14
140fc6736:     	cmpq	$0x0, 0x1b8(%r14)
140fc673e:     	je	0x140fc69df <.text+0xfc59df>
140fc6744:     	leaq	0x28(%rsp), %rdx
140fc6749:     	leaq	0x1c0(%r14), %rcx
140fc6750:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc6755:     	movq	0x1a0(%r14), %rcx
140fc675c:     	movq	0x1a8(%r14), %rdx
140fc6763:     	movq	%rdx, %r8
140fc6766:     	andq	%rax, %r8
140fc6769:     	shrq	$0x39, %rax
140fc676d:     	movd	%eax, %xmm0
140fc6771:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc6775:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc677a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc677f:     	leaq	-0xc0(%rcx), %rax
140fc6786:     	xorl	%r9d, %r9d
140fc6789:     	movq	0x28(%rsp), %r10
140fc678e:     	pcmpeqd	%xmm1, %xmm1
140fc6792:     	movdqu	(%rcx,%r8), %xmm2
140fc6798:     	movdqa	%xmm2, %xmm3
140fc679c:     	pcmpeqb	%xmm0, %xmm3
140fc67a0:     	pmovmskb	%xmm3, %r11d
140fc67a5:     	testl	%r11d, %r11d
140fc67a8:     	je	0x140fc67e0 <.text+0xfc57e0>
140fc67aa:     	tzcntl	%r11d, %r14d
140fc67af:     	addq	%r8, %r14
140fc67b2:     	andq	%rdx, %r14
140fc67b5:     	negq	%r14
140fc67b8:     	leaq	(%r14,%r14,2), %r14
140fc67bc:     	shlq	$0x6, %r14
140fc67c0:     	cmpq	(%rax,%r14), %r10
140fc67c4:     	je	0x140fc6802 <.text+0xfc5802>
140fc67c6:     	leal	-0x1(%r11), %ebp
140fc67ca:     	andw	%r11w, %bp
140fc67ce:     	movl	%ebp, %r11d
140fc67d1:     	jne	0x140fc67aa <.text+0xfc57aa>
140fc67d3:     	nopw	%cs:(%rax,%rax)
140fc67e0:     	pcmpeqb	%xmm1, %xmm2
140fc67e4:     	pmovmskb	%xmm2, %r11d
140fc67e9:     	testl	%r11d, %r11d
140fc67ec:     	jne	0x140fc69df <.text+0xfc59df>
140fc67f2:     	addq	%r9, %r8
140fc67f5:     	addq	$0x10, %r8
140fc67f9:     	addq	$0x10, %r9
140fc67fd:     	andq	%rdx, %r8
140fc6800:     	jmp	0x140fc6792 <.text+0xfc5792>
140fc6802:     	pxor	%xmm0, %xmm0
140fc6806:     	movdqu	%xmm0, -0x90(%rcx,%r14)
140fc6810:     	movdqu	%xmm0, -0xa0(%rcx,%r14)
140fc681a:     	movaps	0x1bec98f(%rip), %xmm1  # 0x142bb31b0
140fc6821:     	movups	%xmm1, -0x80(%rcx,%r14)
140fc6827:     	movdqu	%xmm0, -0x70(%rcx,%r14)
140fc682e:     	movq	$0x0, -0x62(%rcx,%r14)
140fc6837:     	movb	$0x4, -0x5a(%rcx,%r14)
140fc683d:     	movq	0x30(%rsp), %rax
140fc6842:     	movq	%rax, 0x50(%rsp)
140fc6847:     	movups	0x20(%rsp), %xmm0
140fc684c:     	movaps	%xmm0, 0x40(%rsp)
140fc6851:     	movq	0x8(%rsi), %r8
140fc6855:     	leaq	0x20(%rsp), %rcx
140fc685a:     	leaq	0x40(%rsp), %rdx
140fc685f:     	movq	%rbx, %r9
140fc6862:     	callq	0x140f9b5c0 <.text+0xf9a5c0>
140fc6867:     	leaq	0x1bedad4(%rip), %rcx   # 0x142bb4342
140fc686e:     	movl	$0x6, %edx
140fc6873:     	callq	0x140b9d480 <.text+0xb9c480>
140fc6878:     	movq	%rdx, %r9
140fc687b:     	leaq	0x20(%rsp), %rbx
140fc6880:     	movq	%rbx, %rcx
140fc6883:     	movq	%rdi, %rdx
140fc6886:     	movq	%rax, %r8
140fc6889:     	callq	0x14068bfc0 <.text+0x68afc0>
140fc688e:     	movq	%rbx, %rcx
140fc6891:     	movb	$0x1, %dl
140fc6893:     	callq	0x140f9dba0 <.text+0xf9cba0>
140fc6898:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
140fc68a2:     	movups	0x20(%rsp), %xmm0
140fc68a7:     	movaps	%xmm0, 0x40(%rsp)
140fc68ac:     	movq	0x30(%rsp), %rax
140fc68b1:     	movq	%rax, 0x50(%rsp)
140fc68b6:     	leaq	0x40(%rsp), %rcx
140fc68bb:     	movb	$0x1, %dl
140fc68bd:     	callq	0x140f9dba0 <.text+0xf9cba0>
140fc68c2:     	movb	$0x12, 0x25d(%rax)
140fc68c9:     	movdqa	0x40(%rsp), %xmm0
140fc68cf:     	movdqa	%xmm0, 0x20(%rsp)
140fc68d5:     	movq	0x50(%rsp), %rax
140fc68da:     	movq	%rax, 0x30(%rsp)
140fc68df:     	movq	0x20(%rsp), %rdi
140fc68e4:     	cmpq	$0x0, 0x1b8(%rdi)
140fc68ec:     	je	0x140fc69c7 <.text+0xfc59c7>
140fc68f2:     	leaq	0x28(%rsp), %rdx
140fc68f7:     	leaq	0x1c0(%rdi), %rcx
140fc68fe:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc6903:     	movq	0x1a0(%rdi), %rcx
140fc690a:     	movq	0x1a8(%rdi), %rdx
140fc6911:     	movq	%rdx, %r8
140fc6914:     	andq	%rax, %r8
140fc6917:     	shrq	$0x39, %rax
140fc691b:     	movd	%eax, %xmm0
140fc691f:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc6923:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc6928:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc692d:     	leaq	-0xc0(%rcx), %rax
140fc6934:     	xorl	%r9d, %r9d
140fc6937:     	movq	0x28(%rsp), %r10
140fc693c:     	pcmpeqd	%xmm1, %xmm1
140fc6940:     	movdqu	(%rcx,%r8), %xmm2
140fc6946:     	movdqa	%xmm2, %xmm3
140fc694a:     	pcmpeqb	%xmm0, %xmm3
140fc694e:     	pmovmskb	%xmm3, %r11d
140fc6953:     	testl	%r11d, %r11d
140fc6956:     	je	0x140fc6990 <.text+0xfc5990>
140fc6958:     	tzcntl	%r11d, %edi
140fc695d:     	addq	%r8, %rdi
140fc6960:     	andq	%rdx, %rdi
140fc6963:     	negq	%rdi
140fc6966:     	leaq	(%rdi,%rdi,2), %rdi
140fc696a:     	shlq	$0x6, %rdi
140fc696e:     	cmpq	(%rax,%rdi), %r10
140fc6972:     	je	0x140fc69ae <.text+0xfc59ae>
140fc6974:     	leal	-0x1(%r11), %edi
140fc6978:     	andw	%r11w, %di
140fc697c:     	movl	%edi, %r11d
140fc697f:     	jne	0x140fc6958 <.text+0xfc5958>
140fc6981:     	nopw	%cs:(%rax,%rax)
140fc6990:     	pcmpeqb	%xmm1, %xmm2
140fc6994:     	pmovmskb	%xmm2, %r11d
140fc6999:     	testl	%r11d, %r11d
140fc699c:     	jne	0x140fc69c7 <.text+0xfc59c7>
140fc699e:     	addq	%r9, %r8
140fc69a1:     	addq	$0x10, %r8
140fc69a5:     	addq	$0x10, %r9
140fc69a9:     	andq	%rdx, %r8
140fc69ac:     	jmp	0x140fc6940 <.text+0xfc5940>
140fc69ae:     	testb	$0x30, -0x2(%rcx,%rdi)
140fc69b3:     	je	0x140fc69bc <.text+0xfc59bc>
140fc69b5:     	movq	0x10(%rsi), %rax
140fc69b9:     	movb	$0x1, (%rax)
140fc69bc:     	addq	$0x60, %rsp
140fc69c0:     	popq	%rbx
140fc69c1:     	popq	%rbp
140fc69c2:     	popq	%rdi
140fc69c3:     	popq	%rsi
140fc69c4:     	popq	%r14
140fc69c6:     	retq
140fc69c7:     	leaq	0x1a5083a(%rip), %rcx   # 0x142a17208
140fc69ce:     	leaq	0x1a51603(%rip), %r8    # 0x142a17fd8
140fc69d5:     	movl	$0x16, %edx
140fc69da:     	callq	0x1428d9330 <.text+0x28d8330>
140fc69df:     	leaq	0x1a5157a(%rip), %rcx   # 0x142a17f60
140fc69e6:     	callq	0x1428d9310 <.text+0x28d8310>
140fc69eb:     	int3
