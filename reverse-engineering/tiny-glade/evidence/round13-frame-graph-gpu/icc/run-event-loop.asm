
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14026e4a0:     	pushq	%rbp
14026e4a1:     	pushq	%r15
14026e4a3:     	pushq	%r14
14026e4a5:     	pushq	%r13
14026e4a7:     	pushq	%r12
14026e4a9:     	pushq	%rsi
14026e4aa:     	pushq	%rdi
14026e4ab:     	pushq	%rbx
14026e4ac:     	subq	$0x3b8, %rsp            # imm = 0x3B8
14026e4b3:     	leaq	0x80(%rsp), %rbp
14026e4bb:     	movaps	%xmm9, 0x320(%rbp)
14026e4c3:     	movaps	%xmm8, 0x310(%rbp)
14026e4cb:     	movaps	%xmm7, 0x300(%rbp)
14026e4d2:     	movaps	%xmm6, 0x2f0(%rbp)
14026e4d9:     	movq	$-0x2, 0x2e8(%rbp)
14026e4e4:     	movq	%r8, 0x1e0(%rbp)
14026e4eb:     	movq	%rcx, 0x1d8(%rbp)
14026e4f2:     	movupd	0x3a8(%rbp), %xmm0
14026e4fa:     	movq	0x3a0(%rbp), %rax
14026e501:     	movq	%rdx, 0x2c0(%rbp)
14026e508:     	movq	%r9, 0x1c8(%rbp)
14026e50f:     	movq	%rax, 0x1d0(%rbp)
14026e516:     	movupd	%xmm0, 0x88(%rbp)
14026e51e:     	movl	$0x0, 0x1c4(%rbp)
14026e528:     	movq	0x18(%rdx), %rcx
14026e52c:     	callq	0x140697b00 <.text+0x696b00>
14026e531:     	movq	$0x1, 0x78(%rbp)
14026e539:     	movq	%rax, 0x80(%rbp)
14026e540:     	movb	$0x1, 0x2e3(%rbp)
14026e547:     	movl	$0x2, 0x54(%rbp)
14026e54e:     	movq	$0x0, 0x200(%rbp)
14026e559:     	xorps	%xmm6, %xmm6
14026e55c:     	movups	%xmm6, 0x210(%rbp)
14026e563:     	movq	$0x8, 0x208(%rbp)
14026e56e:     	movl	$0x0, 0x2ac(%rbp)
14026e578:     	movq	$0x0, 0x1e8(%rbp)
14026e583:     	movl	$0x0, 0x1f8(%rbp)
14026e58d:     	callq	0x140d635e0 <.text+0xd625e0>
14026e592:     	movq	%rax, 0x108(%rbp)
14026e599:     	movl	%edx, 0x1fc(%rbp)
14026e59f:     	movb	$0x1, 0x2cf(%rbp)
14026e5a6:     	movq	0x1e0(%rbp), %rax
14026e5ad:     	addq	$0x8, %rax
14026e5b1:     	movq	%rax, 0x100(%rbp)
14026e5b8:     	leaq	0x220(%rbp), %r12
14026e5bf:     	leaq	0x27397a2(%rip), %r14   # 0x1429a7d68
14026e5c6:     	movq	%rax, 0x1f0(%rbp)
14026e5cd:     	movl	$0x0, 0x2bc(%rbp)
14026e5d7:     	cmpb	$0x0, 0x2cf(%rbp)
14026e5de:     	je	0x14026fa20 <.text+0x26ea20>
14026e5e4:     	movq	0x1d8(%rbp), %rcx
14026e5eb:     	movq	0x10(%rcx), %rax
14026e5ef:     	movl	$0x3b9aca00, 0x18(%rax) # imm = 0x3B9ACA00
14026e5f6:     	movq	0x100(%rbp), %rax
14026e5fd:     	movq	%rax, 0x220(%rbp)
14026e604:     	leaq	0x2c0(%rbp), %rax
14026e60b:     	movq	%rax, 0x228(%rbp)
14026e612:     	leaq	0x1c8(%rbp), %rax
14026e619:     	movq	%rax, 0x230(%rbp)
14026e620:     	leaq	0x2cf(%rbp), %rax
14026e627:     	movq	%rax, 0x238(%rbp)
14026e62e:     	leaq	0x1c4(%rbp), %rax
14026e635:     	movq	%rax, 0x240(%rbp)
14026e63c:     	leaq	0x78(%rbp), %rax
14026e640:     	movq	%rax, 0x248(%rbp)
14026e647:     	leaq	0x2e3(%rbp), %rax
14026e64e:     	movq	%rax, 0x250(%rbp)
14026e655:     	leaq	0x54(%rbp), %rax
14026e659:     	movq	%rax, 0x258(%rbp)
14026e660:     	leaq	0x1e8(%rbp), %rax
14026e667:     	movq	%rax, 0x260(%rbp)
14026e66e:     	leaq	0x2ac(%rbp), %rax
14026e675:     	movq	%rax, 0x268(%rbp)
14026e67c:     	leaq	0x1f8(%rbp), %rax
14026e683:     	movq	%rax, 0x270(%rbp)
14026e68a:     	xorl	%edx, %edx
14026e68c:     	movl	$0x4c4b40, %r8d         # imm = 0x4C4B40
14026e692:     	movq	%r12, %r9
14026e695:     	callq	0x14053c400 <.text+0x53b400>
14026e69a:     	testl	%eax, %eax
14026e69c:     	jne	0x14026fa20 <.text+0x26ea20>
14026e6a2:     	movl	$0x4, %r15d
14026e6a8:     	xorl	%ebx, %ebx
14026e6aa:     	jmp	0x14026e6c0 <.text+0x26d6c0>
14026e6ac:     	movq	%rsi, 0x1e8(%rbp)
14026e6b3:     	nopw	%cs:(%rax,%rax)
14026e6c0:     	movq	0x88(%rbp), %rax
14026e6c7:     	movq	0x90(%rbp), %rdx
14026e6ce:     	testq	%rax, %rax
14026e6d1:     	movq	%r15, %r13
14026e6d4:     	je	0x14026e6f0 <.text+0x26d6f0>
14026e6d6:     	cmpl	$0x1, %eax
14026e6d9:     	jne	0x14026e700 <.text+0x26d700>
14026e6db:     	leaq	-0x38(%rbp), %rcx
14026e6df:     	callq	0x140266d40 <.text+0x265d40>
14026e6e4:     	jmp	0x14026e709 <.text+0x26d709>
14026e6e6:     	nopw	%cs:(%rax,%rax)
14026e6f0:     	leaq	-0x38(%rbp), %rcx
14026e6f4:     	callq	0x1404f2610 <.text+0x4f1610>
14026e6f9:     	jmp	0x14026e709 <.text+0x26d709>
14026e6fb:     	nopl	(%rax,%rax)
14026e700:     	leaq	-0x38(%rbp), %rcx
14026e704:     	callq	0x14051ff70 <.text+0x51ef70>
14026e709:     	movq	-0x38(%rbp), %rcx
14026e70d:     	movabsq	$-0x7ffffffffffffff9, %rax # imm = 0x8000000000000007
14026e717:     	cmpq	%rax, %rcx
14026e71a:     	je	0x14026f430 <.text+0x26e430>
14026e720:     	movq	-0x30(%rbp), %rsi
14026e724:     	movl	-0x28(%rbp), %eax
14026e727:     	movsd	-0x24(%rbp), %xmm9
14026e72d:     	movq	%rcx, %rdx
14026e730:     	movabsq	$-0x8000000000000000, %r8 # imm = 0x8000000000000000
14026e73a:     	xorq	%r8, %rdx
14026e73d:     	cmpq	$0x7, %rdx
14026e741:     	movl	$0x3, %r8d
14026e747:     	cmovaeq	%r8, %rdx
14026e74b:     	movslq	(%r14,%rdx,4), %rdx
14026e74f:     	addq	%r14, %rdx
14026e752:     	movq	%r13, %r15
14026e755:     	jmpq	*%rdx
14026e757:     	movq	%rsi, %rax
14026e75a:     	shrq	$0x20, %rax
14026e75e:     	movq	%rax, 0x1f0(%rbp)
14026e765:     	movl	%esi, %r15d
14026e768:     	jmp	0x14026e6c0 <.text+0x26d6c0>
14026e76d:     	movq	%rcx, 0x110(%rbp)
14026e774:     	movq	%rsi, 0x118(%rbp)
14026e77b:     	movl	%eax, 0x120(%rbp)
14026e781:     	movlps	%xmm9, 0x124(%rbp)
14026e789:     	leaq	-0x1c(%rbp), %rcx
14026e78d:     	movups	0x4c(%rcx), %xmm0
14026e791:     	leaq	0x12c(%rbp), %rax
14026e798:     	movups	%xmm0, 0x4c(%rax)
14026e79c:     	movups	0x40(%rcx), %xmm0
14026e7a0:     	movups	%xmm0, 0x40(%rax)
14026e7a4:     	movupd	(%rcx), %xmm0
14026e7a8:     	movups	0x10(%rcx), %xmm1
14026e7ac:     	movups	0x20(%rcx), %xmm2
14026e7b0:     	movups	0x30(%rcx), %xmm3
14026e7b4:     	movups	%xmm3, 0x30(%rax)
14026e7b8:     	movups	%xmm2, 0x20(%rax)
14026e7bc:     	movups	%xmm1, 0x10(%rax)
14026e7c0:     	movupd	%xmm0, (%rax)
14026e7c4:     	movq	0x100(%rbp), %rax
14026e7cb:     	movq	(%rax), %rsi
14026e7ce:     	leaq	0x10(%rsi), %rdi
14026e7d2:     	xorl	%eax, %eax
14026e7d4:     	movb	$0x1, %cl
14026e7d6:     	lock
14026e7d7:     	cmpxchgb	%cl, 0x10(%rsi)
14026e7db:     	jne	0x14026f3d1 <.text+0x26e3d1>
14026e7e1:     	movq	0x33bdc38(%rip), %rax   # 0x14362c420
14026e7e8:     	movq	(%rax), %rax
14026e7eb:     	shlq	%rax
14026e7ee:     	testq	%rax, %rax
14026e7f1:     	jne	0x14026f3e5 <.text+0x26e3e5>
14026e7f7:     	movl	$0x0, 0x2d0(%rbp)
14026e801:     	movq	%rdi, 0x2d8(%rbp)
14026e808:     	movzbl	0x11(%rsi), %eax
14026e80c:     	testb	%al, %al
14026e80e:     	jne	0x14026fbd5 <.text+0x26ebd5>
14026e814:     	leaq	0x18(%rsi), %rcx
14026e818:     	movq	0x2c0(%rbp), %rdx
14026e81f:     	addq	$0x10, %rdx
14026e823:     	movq	0x180(%rbp), %rax
14026e82a:     	movq	%rax, 0x290(%rbp)
14026e831:     	movups	0x170(%rbp), %xmm0
14026e838:     	movaps	%xmm0, 0x280(%rbp)
14026e83f:     	movups	0x160(%rbp), %xmm0
14026e846:     	movaps	%xmm0, 0x270(%rbp)
14026e84d:     	movups	0x150(%rbp), %xmm0
14026e854:     	movaps	%xmm0, 0x260(%rbp)
14026e85b:     	movupd	0x110(%rbp), %xmm0
14026e863:     	movups	0x120(%rbp), %xmm1
14026e86a:     	movups	0x130(%rbp), %xmm2
14026e871:     	movups	0x140(%rbp), %xmm3
14026e878:     	movaps	%xmm3, 0x250(%rbp)
14026e87f:     	movaps	%xmm2, 0x240(%rbp)
14026e886:     	movaps	%xmm1, 0x230(%rbp)
14026e88d:     	movapd	%xmm0, 0x220(%rbp)
14026e895:     	movq	%r12, %r8
14026e898:     	callq	0x140677f40 <.text+0x676f40>
14026e89d:     	cmpb	$0x0, 0x2d0(%rbp)
14026e8a4:     	movq	0x2d8(%rbp), %rdi
14026e8ab:     	jne	0x14026e8c3 <.text+0x26d8c3>
14026e8ad:     	movq	0x33bdb6c(%rip), %rax   # 0x14362c420
14026e8b4:     	movq	(%rax), %rax
14026e8b7:     	shlq	%rax
14026e8ba:     	testq	%rax, %rax
14026e8bd:     	jne	0x14026f411 <.text+0x26e411>
14026e8c3:     	xorl	%eax, %eax
14026e8c5:     	xchgb	%al, (%rdi)
14026e8c7:     	cmpb	$0x2, %al
14026e8c9:     	jne	0x14026e6c0 <.text+0x26d6c0>
14026e8cf:     	movb	$0x0, 0x2e5(%rbp)
14026e8d6:     	movq	%rdi, %rcx
14026e8d9:     	callq	0x1428d8810 <.text+0x28d7810>
14026e8de:     	jmp	0x14026e6c0 <.text+0x26d6c0>
14026e8e3:     	testl	%eax, %eax
14026e8e5:     	je	0x14026f29d <.text+0x26e29d>
14026e8eb:     	cmpl	$0x1, %eax
14026e8ee:     	jne	0x14026f35a <.text+0x26e35a>
14026e8f4:     	movq	0x2c0(%rbp), %rdx
14026e8fb:     	addq	$0x10, %rdx
14026e8ff:     	movq	%r12, %rcx
14026e902:     	xorl	%r8d, %r8d
14026e905:     	callq	0x14026d8a0 <.text+0x26c8a0>
14026e90a:     	cmpl	$0x2, 0x220(%rbp)
14026e911:     	movq	%r13, %r15
14026e914:     	jne	0x14026e990 <.text+0x26d990>
14026e916:     	movq	0x238(%rbp), %rax
14026e91d:     	movl	%eax, %ecx
14026e91f:     	andl	$0x3, %ecx
14026e922:     	cmpl	$0x1, %ecx
14026e925:     	jne	0x14026e990 <.text+0x26d990>
14026e927:     	leaq	-0x1(%rax), %rcx
14026e92b:     	movq	%rcx, 0x2b0(%rbp)
14026e932:     	movq	-0x1(%rax), %rcx
14026e936:     	movq	%rcx, 0x2d8(%rbp)
14026e93d:     	movq	0x7(%rax), %rax
14026e941:     	movq	%rax, 0x2d0(%rbp)
14026e948:     	movq	(%rax), %rax
14026e94b:     	testq	%rax, %rax
14026e94e:     	je	0x14026e959 <.text+0x26d959>
14026e950:     	movq	0x2d8(%rbp), %rcx
14026e957:     	callq	*%rax
14026e959:     	movq	0x2d8(%rbp), %rcx
14026e960:     	movq	0x2d0(%rbp), %rax
14026e967:     	movq	0x8(%rax), %rdx
14026e96b:     	testq	%rdx, %rdx
14026e96e:     	je	0x14026e979 <.text+0x26d979>
14026e970:     	movq	0x10(%rax), %r8
14026e974:     	callq	0x140613c20 <.text+0x612c20>
14026e979:     	movl	$0x18, %edx
14026e97e:     	movl	$0x8, %r8d
14026e984:     	movq	0x2b0(%rbp), %rcx
14026e98b:     	callq	0x140613c20 <.text+0x612c20>
14026e990:     	movl	$0x2, 0x54(%rbp)
14026e997:     	xorl	%ebx, %ebx
14026e999:     	movq	0x200(%rbp), %rcx
14026e9a0:     	movq	0x218(%rbp), %rax
14026e9a7:     	cmpq	%rcx, %rax
14026e9aa:     	je	0x14026f378 <.text+0x26e378>
14026e9b0:     	jmp	0x14026f39c <.text+0x26e39c>
14026e9b5:     	movq	0x2c0(%rbp), %rsi
14026e9bc:     	movq	0x18(%rsi), %rdi
14026e9c0:     	movq	0x10(%rsi), %rax
14026e9c4:     	lock
14026e9c5:     	incq	(%rax)
14026e9c8:     	jle	0x14026fc1a <.text+0x26ec1a>
14026e9ce:     	movq	%rax, 0x110(%rbp)
14026e9d5:     	movb	$0x1, 0x2ce(%rbp)
14026e9dc:     	movq	%rdi, %rcx
14026e9df:     	movq	%rax, 0x2d8(%rbp)
14026e9e6:     	callq	0x140695d20 <.text+0x694d20>
14026e9eb:     	addq	$0x20, %rsi
14026e9ef:     	movq	%rdi, 0x220(%rbp)
14026e9f6:     	movq	0x2d8(%rbp), %rcx
14026e9fd:     	movq	%rcx, 0x228(%rbp)
14026ea04:     	movb	%al, 0x230(%rbp)
14026ea0a:     	movb	$0x1, 0x231(%rbp)
14026ea11:     	movb	$0x0, 0x2ce(%rbp)
14026ea18:     	movq	%rsi, %rcx
14026ea1b:     	movq	%r12, %rdx
14026ea1e:     	callq	0x14053f2c0 <.text+0x53e2c0>
14026ea23:     	jmp	0x14026e6c0 <.text+0x26d6c0>
14026ea28:     	movq	%rsi, 0x98(%rbp)
14026ea2f:     	movl	%eax, 0xa0(%rbp)
14026ea35:     	movlps	%xmm9, 0xa4(%rbp)
14026ea3d:     	leaq	-0x1c(%rbp), %rdx
14026ea41:     	movl	0x30(%rdx), %eax
14026ea44:     	leaq	0xac(%rbp), %rcx
14026ea4b:     	movl	%eax, 0x30(%rcx)
14026ea4e:     	movupd	(%rdx), %xmm0
14026ea52:     	movups	0x10(%rdx), %xmm1
14026ea56:     	movups	0x20(%rdx), %xmm2
14026ea5a:     	movups	%xmm2, 0x20(%rcx)
14026ea5e:     	movups	%xmm1, 0x10(%rcx)
14026ea62:     	movupd	%xmm0, (%rcx)
14026ea66:     	movq	0x2c0(%rbp), %rdx
14026ea6d:     	addq	$0x10, %rdx
14026ea71:     	movb	$0x1, 0x2e7(%rbp)
14026ea78:     	movq	%r12, %rcx
14026ea7b:     	callq	0x140697a90 <.text+0x696a90>
14026ea80:     	movq	%r12, %rcx
14026ea83:     	leaq	0x98(%rbp), %rdx
14026ea8a:     	leaq	0x240(%rbp), %r8
14026ea91:     	callq	0x140526a70 <.text+0x525a70>
14026ea96:     	testb	$0x1, %al
14026ea98:     	je	0x14026f036 <.text+0x26e036>
14026ea9e:     	movq	%rdx, %rdi
14026eaa1:     	movq	0x220(%rbp), %rdx
14026eaa8:     	testq	%rdx, %rdx
14026eaab:     	je	0x14026eac3 <.text+0x26dac3>
14026eaad:     	movq	0x228(%rbp), %rcx
14026eab4:     	shlq	$0x3, %rdx
14026eab8:     	movl	$0x8, %r8d
14026eabe:     	callq	0x140613c20 <.text+0x612c20>
14026eac3:     	cmpb	$0x0, 0xdc(%rbp)
14026eaca:     	je	0x14026f05e <.text+0x26e05e>
14026ead0:     	testb	$0x1, 0xdd(%rbp)
14026ead7:     	je	0x14026ee78 <.text+0x26de78>
14026eadd:     	movb	$0x1, 0x2e7(%rbp)
14026eae4:     	leaq	-0x58(%rbp), %rcx
14026eae8:     	movq	%rdi, %rdx
14026eaeb:     	leaq	0x98(%rbp), %r8
14026eaf2:     	callq	0x1400453c0 <.text+0x443c0>
14026eaf7:     	cmpq	$0x0, -0x58(%rbp)
14026eafc:     	je	0x14026ee78 <.text+0x26de78>
14026eb02:     	movl	%ebx, %r13d
14026eb05:     	movupd	-0x58(%rbp), %xmm0
14026eb0a:     	movups	-0x48(%rbp), %xmm1
14026eb0e:     	movaps	%xmm1, 0x1b0(%rbp)
14026eb15:     	movapd	%xmm0, 0x1a0(%rbp)
14026eb1d:     	movb	$0x1, 0x2e6(%rbp)
14026eb24:     	leaq	0x1a0(%rbp), %rcx
14026eb2b:     	callq	0x1406979b0 <.text+0x6969b0>
14026eb30:     	movl	%edx, %ebx
14026eb32:     	movl	%eax, %eax
14026eb34:     	xorps	%xmm0, %xmm0
14026eb37:     	cvtsi2sd	%rax, %xmm0
14026eb3c:     	movb	$0x1, 0x2e6(%rbp)
14026eb43:     	callq	0x1406977f0 <.text+0x6967f0>
14026eb48:     	movl	%eax, %esi
14026eb4a:     	movl	%ebx, %eax
14026eb4c:     	xorps	%xmm0, %xmm0
14026eb4f:     	cvtsi2sd	%rax, %xmm0
14026eb54:     	movb	$0x1, 0x2e6(%rbp)
14026eb5b:     	callq	0x1406977f0 <.text+0x6967f0>
14026eb60:     	shlq	$0x20, %rax
14026eb64:     	movl	%esi, %ecx
14026eb66:     	orq	%rax, %rcx
14026eb69:     	movzwl	0x1bc(%rbp), %eax
14026eb70:     	movl	0x1b8(%rbp), %edx
14026eb76:     	movq	%rcx, 0xcc(%rbp)
14026eb7d:     	movl	%edx, 0xd4(%rbp)
14026eb83:     	movw	%ax, 0xd8(%rbp)
14026eb8a:     	movl	$0x1, 0xc8(%rbp)
14026eb94:     	movq	0x33bd80d(%rip), %rax   # 0x14362c3a8
14026eb9b:     	movq	(%rax), %rax
14026eb9e:     	cmpq	$0x3, %rax
14026eba2:     	jb	0x14026ec95 <.text+0x26dc95>
14026eba8:     	leaq	0x1a0(%rbp), %rax
14026ebaf:     	movq	%rax, 0x110(%rbp)
14026ebb6:     	leaq	0x437c83(%rip), %rax    # 0x1406a6840 <.text+0x6a5840>
14026ebbd:     	movq	%rax, 0x118(%rbp)
14026ebc4:     	movb	$0x1, 0x2e6(%rbp)
14026ebcb:     	leaq	0x2738ffe(%rip), %rcx   # 0x1429a7bd0
14026ebd2:     	callq	0x14067dd80 <.text+0x67cd80>
14026ebd7:     	movupd	(%rax), %xmm0
14026ebdb:     	movl	0x10(%rax), %eax
14026ebde:     	movq	$0x3, 0x250(%rbp)
14026ebe9:     	leaq	0x2738f90(%rip), %rcx   # 0x1429a7b80
14026ebf0:     	movq	%rcx, 0x258(%rbp)
14026ebf7:     	movq	$0x16, 0x260(%rbp)
14026ec02:     	leaq	0x2738fb7(%rip), %rdx   # 0x1429a7bc0
14026ec09:     	movq	%rdx, 0x270(%rbp)
14026ec10:     	movq	$0x1, 0x278(%rbp)
14026ec1b:     	leaq	0x110(%rbp), %rdx
14026ec22:     	movq	%rdx, 0x280(%rbp)
14026ec29:     	movq	$0x1, 0x288(%rbp)
14026ec34:     	movq	$0x0, 0x290(%rbp)
14026ec3f:     	movq	$0x0, 0x220(%rbp)
14026ec4a:     	movq	%rcx, 0x228(%rbp)
14026ec51:     	movq	$0x16, 0x230(%rbp)
14026ec5c:     	movq	$0x0, 0x238(%rbp)
14026ec67:     	movupd	%xmm0, 0x240(%rbp)
14026ec6f:     	movl	$0x1, 0x268(%rbp)
14026ec79:     	movl	%eax, 0x26c(%rbp)
14026ec7f:     	movb	$0x1, 0x2e6(%rbp)
14026ec86:     	leaq	0x2e4(%rbp), %rcx
14026ec8d:     	movq	%r12, %rdx
14026ec90:     	callq	0x140d4a330 <.text+0xd49330>
14026ec95:     	movq	0x2c0(%rbp), %rdx
14026ec9c:     	addq	$0x10, %rdx
14026eca0:     	movapd	0x1a0(%rbp), %xmm0
14026eca8:     	movaps	0x1b0(%rbp), %xmm1
14026ecaf:     	leaq	0x228(%rbp), %rax
14026ecb6:     	movups	%xmm1, 0x10(%rax)
14026ecba:     	movupd	%xmm0, (%rax)
14026ecbe:     	movq	$0x1, 0x220(%rbp)
14026ecc9:     	movb	$0x0, 0x2e6(%rbp)
14026ecd0:     	leaq	0xe0(%rbp), %rcx
14026ecd7:     	movq	%r12, %r8
14026ecda:     	callq	0x140698280 <.text+0x697280>
14026ecdf:     	cmpq	$0x0, 0xe0(%rbp)
14026ece7:     	je	0x14026f3c9 <.text+0x26e3c9>
14026eced:     	movupd	0xe0(%rbp), %xmm0
14026ecf5:     	movups	0xf0(%rbp), %xmm1
14026ecfc:     	movaps	%xmm1, 0x120(%rbp)
14026ed03:     	movapd	%xmm0, 0x110(%rbp)
14026ed0b:     	movb	$0x0, 0xdd(%rbp)
14026ed12:     	movq	0x33bd68f(%rip), %rax   # 0x14362c3a8
14026ed19:     	movq	(%rax), %rax
14026ed1c:     	testq	%rax, %rax
14026ed1f:     	movl	%r13d, %ebx
14026ed22:     	je	0x14026edfe <.text+0x26ddfe>
14026ed28:     	leaq	0x110(%rbp), %rax
14026ed2f:     	movq	%rax, 0x40(%rbp)
14026ed33:     	leaq	-0x154a(%rip), %rax     # 0x14026d7f0 <.text+0x26c7f0>
14026ed3a:     	movq	%rax, 0x48(%rbp)
14026ed3e:     	leaq	0x2738edb(%rip), %rcx   # 0x1429a7c20
14026ed45:     	callq	0x14067dd80 <.text+0x67cd80>
14026ed4a:     	movupd	(%rax), %xmm0
14026ed4e:     	movl	0x10(%rax), %eax
14026ed51:     	movq	$0x1, 0x250(%rbp)
14026ed5c:     	leaq	0x2738e1d(%rip), %rcx   # 0x1429a7b80
14026ed63:     	movq	%rcx, 0x258(%rbp)
14026ed6a:     	movq	$0x16, 0x260(%rbp)
14026ed75:     	leaq	0x2738e94(%rip), %rdx   # 0x1429a7c10
14026ed7c:     	movq	%rdx, 0x270(%rbp)
14026ed83:     	movq	$0x1, 0x278(%rbp)
14026ed8e:     	leaq	0x40(%rbp), %rdx
14026ed92:     	movq	%rdx, 0x280(%rbp)
14026ed99:     	movq	$0x1, 0x288(%rbp)
14026eda4:     	movq	$0x0, 0x290(%rbp)
14026edaf:     	movq	$0x0, 0x220(%rbp)
14026edba:     	movq	%rcx, 0x228(%rbp)
14026edc1:     	movq	$0x16, 0x230(%rbp)
14026edcc:     	movq	$0x0, 0x238(%rbp)
14026edd7:     	movupd	%xmm0, 0x240(%rbp)
14026eddf:     	movl	$0x1, 0x268(%rbp)
14026ede9:     	movl	%eax, 0x26c(%rbp)
14026edef:     	leaq	0x2e4(%rbp), %rcx
14026edf6:     	movq	%r12, %rdx
14026edf9:     	callq	0x140d4a330 <.text+0xd49330>
14026edfe:     	movq	0x120(%rbp), %rax
14026ee05:     	movl	%eax, %ecx
14026ee07:     	andl	$0x3, %ecx
14026ee0a:     	cmpl	$0x1, %ecx
14026ee0d:     	jne	0x14026ee78 <.text+0x26de78>
14026ee0f:     	leaq	-0x1(%rax), %rcx
14026ee13:     	movq	%rcx, 0x2b0(%rbp)
14026ee1a:     	movq	-0x1(%rax), %rcx
14026ee1e:     	movq	%rcx, 0x2d8(%rbp)
14026ee25:     	movq	0x7(%rax), %rax
14026ee29:     	movq	%rax, 0x2d0(%rbp)
14026ee30:     	movq	(%rax), %rax
14026ee33:     	testq	%rax, %rax
14026ee36:     	je	0x14026ee41 <.text+0x26de41>
14026ee38:     	movq	0x2d8(%rbp), %rcx
14026ee3f:     	callq	*%rax
14026ee41:     	movq	0x2d8(%rbp), %rcx
14026ee48:     	movq	0x2d0(%rbp), %rax
14026ee4f:     	movq	0x8(%rax), %rdx
14026ee53:     	testq	%rdx, %rdx
14026ee56:     	je	0x14026ee61 <.text+0x26de61>
14026ee58:     	movq	0x10(%rax), %r8
14026ee5c:     	callq	0x140613c20 <.text+0x612c20>
14026ee61:     	movl	$0x18, %edx
14026ee66:     	movl	$0x8, %r8d
14026ee6c:     	movq	0x2b0(%rbp), %rcx
14026ee73:     	callq	0x140613c20 <.text+0x612c20>
14026ee78:     	movq	0x2c0(%rbp), %rdx
14026ee7f:     	addq	$0x10, %rdx
14026ee83:     	movq	$0x0, 0x228(%rbp)
14026ee8e:     	movq	$0x1, 0x230(%rbp)
14026ee99:     	movq	%rdi, 0x238(%rbp)
14026eea0:     	movq	$0x1, 0x220(%rbp)
14026eeab:     	movb	$0x1, 0x2e7(%rbp)
14026eeb2:     	leaq	0xe0(%rbp), %rcx
14026eeb9:     	movq	%r12, %r8
14026eebc:     	callq	0x140698280 <.text+0x697280>
14026eec1:     	cmpq	$0x0, 0xe0(%rbp)
14026eec9:     	je	0x14026f222 <.text+0x26e222>
14026eecf:     	movupd	0xe0(%rbp), %xmm0
14026eed7:     	movups	0xf0(%rbp), %xmm1
14026eede:     	movaps	%xmm1, 0x120(%rbp)
14026eee5:     	movapd	%xmm0, 0x110(%rbp)
14026eeed:     	movb	$0x0, 0xdc(%rbp)
14026eef4:     	movq	0x33bd4ad(%rip), %rax   # 0x14362c3a8
14026eefb:     	movq	(%rax), %rax
14026eefe:     	testq	%rax, %rax
14026ef01:     	je	0x14026efe6 <.text+0x26dfe6>
14026ef07:     	leaq	0x110(%rbp), %rax
14026ef0e:     	movq	%rax, 0x1a0(%rbp)
14026ef15:     	leaq	-0x172c(%rip), %rax     # 0x14026d7f0 <.text+0x26c7f0>
14026ef1c:     	movq	%rax, 0x1a8(%rbp)
14026ef23:     	leaq	0x2738d46(%rip), %rcx   # 0x1429a7c70
14026ef2a:     	callq	0x14067dd80 <.text+0x67cd80>
14026ef2f:     	movupd	(%rax), %xmm0
14026ef33:     	movl	0x10(%rax), %eax
14026ef36:     	movq	$0x1, 0x250(%rbp)
14026ef41:     	leaq	0x2738c38(%rip), %rcx   # 0x1429a7b80
14026ef48:     	movq	%rcx, 0x258(%rbp)
14026ef4f:     	movq	$0x16, 0x260(%rbp)
14026ef5a:     	leaq	0x2738cff(%rip), %rdx   # 0x1429a7c60
14026ef61:     	movq	%rdx, 0x270(%rbp)
14026ef68:     	movq	$0x1, 0x278(%rbp)
14026ef73:     	leaq	0x1a0(%rbp), %rdx
14026ef7a:     	movq	%rdx, 0x280(%rbp)
14026ef81:     	movq	$0x1, 0x288(%rbp)
14026ef8c:     	movq	$0x0, 0x290(%rbp)
14026ef97:     	movq	$0x0, 0x220(%rbp)
14026efa2:     	movq	%rcx, 0x228(%rbp)
14026efa9:     	movq	$0x16, 0x230(%rbp)
14026efb4:     	movq	$0x0, 0x238(%rbp)
14026efbf:     	movupd	%xmm0, 0x240(%rbp)
14026efc7:     	movl	$0x1, 0x268(%rbp)
14026efd1:     	movl	%eax, 0x26c(%rbp)
14026efd7:     	leaq	0x2e4(%rbp), %rcx
14026efde:     	movq	%r12, %rdx
14026efe1:     	callq	0x140d4a330 <.text+0xd49330>
14026efe6:     	movq	0x120(%rbp), %rax
14026efed:     	movl	%eax, %ecx
14026efef:     	andl	$0x3, %ecx
14026eff2:     	cmpl	$0x1, %ecx
14026eff5:     	jne	0x14026f222 <.text+0x26e222>
14026effb:     	leaq	-0x1(%rax), %rcx
14026efff:     	movq	%rcx, 0x2b0(%rbp)
14026f006:     	movq	-0x1(%rax), %rcx
14026f00a:     	movq	%rcx, 0x2d8(%rbp)
14026f011:     	movq	0x7(%rax), %rax
14026f015:     	movq	%rax, 0x2d0(%rbp)
14026f01c:     	movq	(%rax), %rax
14026f01f:     	testq	%rax, %rax
14026f022:     	je	0x14026f1eb <.text+0x26e1eb>
14026f028:     	movq	0x2d8(%rbp), %rcx
14026f02f:     	callq	*%rax
14026f031:     	jmp	0x14026f1eb <.text+0x26e1eb>
14026f036:     	movq	0x2c0(%rbp), %rax
14026f03d:     	movq	0x18(%rax), %rcx
14026f041:     	callq	0x140697b00 <.text+0x696b00>
14026f046:     	movq	%rax, %rdi
14026f049:     	movq	0x220(%rbp), %rdx
14026f050:     	testq	%rdx, %rdx
14026f053:     	jne	0x14026eaad <.text+0x26daad>
14026f059:     	jmp	0x14026eac3 <.text+0x26dac3>
14026f05e:     	movq	0x2c0(%rbp), %rdx
14026f065:     	addq	$0x10, %rdx
14026f069:     	movq	$0x0, 0x220(%rbp)
14026f074:     	movb	$0x1, 0x2e7(%rbp)
14026f07b:     	leaq	0xe0(%rbp), %rcx
14026f082:     	movq	%r12, %r8
14026f085:     	callq	0x140698280 <.text+0x697280>
14026f08a:     	cmpq	$0x0, 0xe0(%rbp)
14026f092:     	je	0x14026f222 <.text+0x26e222>
14026f098:     	movupd	0xe0(%rbp), %xmm0
14026f0a0:     	movups	0xf0(%rbp), %xmm1
14026f0a7:     	movaps	%xmm1, 0x120(%rbp)
14026f0ae:     	movapd	%xmm0, 0x110(%rbp)
14026f0b6:     	movq	0x33bd2eb(%rip), %rax   # 0x14362c3a8
14026f0bd:     	movq	(%rax), %rax
14026f0c0:     	testq	%rax, %rax
14026f0c3:     	je	0x14026f1a8 <.text+0x26e1a8>
14026f0c9:     	leaq	0x110(%rbp), %rax
14026f0d0:     	movq	%rax, 0x1a0(%rbp)
14026f0d7:     	leaq	-0x18ee(%rip), %rax     # 0x14026d7f0 <.text+0x26c7f0>
14026f0de:     	movq	%rax, 0x1a8(%rbp)
14026f0e5:     	leaq	0x2738a7c(%rip), %rcx   # 0x1429a7b68
14026f0ec:     	callq	0x14067dd80 <.text+0x67cd80>
14026f0f1:     	movupd	(%rax), %xmm0
14026f0f5:     	movl	0x10(%rax), %eax
14026f0f8:     	movq	$0x1, 0x250(%rbp)
14026f103:     	leaq	0x2738a76(%rip), %rcx   # 0x1429a7b80
14026f10a:     	movq	%rcx, 0x258(%rbp)
14026f111:     	movq	$0x16, 0x260(%rbp)
14026f11c:     	leaq	0x2738a35(%rip), %rdx   # 0x1429a7b58
14026f123:     	movq	%rdx, 0x270(%rbp)
14026f12a:     	movq	$0x1, 0x278(%rbp)
14026f135:     	leaq	0x1a0(%rbp), %rdx
14026f13c:     	movq	%rdx, 0x280(%rbp)
14026f143:     	movq	$0x1, 0x288(%rbp)
14026f14e:     	movq	$0x0, 0x290(%rbp)
14026f159:     	movq	$0x0, 0x220(%rbp)
14026f164:     	movq	%rcx, 0x228(%rbp)
14026f16b:     	movq	$0x16, 0x230(%rbp)
14026f176:     	movq	$0x0, 0x238(%rbp)
14026f181:     	movupd	%xmm0, 0x240(%rbp)
14026f189:     	movl	$0x1, 0x268(%rbp)
14026f193:     	movl	%eax, 0x26c(%rbp)
14026f199:     	leaq	0x2e4(%rbp), %rcx
14026f1a0:     	movq	%r12, %rdx
14026f1a3:     	callq	0x140d4a330 <.text+0xd49330>
14026f1a8:     	movq	0x120(%rbp), %rax
14026f1af:     	movl	%eax, %ecx
14026f1b1:     	andl	$0x3, %ecx
14026f1b4:     	cmpl	$0x1, %ecx
14026f1b7:     	jne	0x14026f222 <.text+0x26e222>
14026f1b9:     	leaq	-0x1(%rax), %rcx
14026f1bd:     	movq	%rcx, 0x2b0(%rbp)
14026f1c4:     	movq	-0x1(%rax), %rcx
14026f1c8:     	movq	%rcx, 0x2d8(%rbp)
14026f1cf:     	movq	0x7(%rax), %rax
14026f1d3:     	movq	%rax, 0x2d0(%rbp)
14026f1da:     	movq	(%rax), %rax
14026f1dd:     	testq	%rax, %rax
14026f1e0:     	je	0x14026f1eb <.text+0x26e1eb>
14026f1e2:     	movq	0x2d8(%rbp), %rcx
14026f1e9:     	callq	*%rax
14026f1eb:     	movq	0x2d8(%rbp), %rcx
14026f1f2:     	movq	0x2d0(%rbp), %rax
14026f1f9:     	movq	0x8(%rax), %rdx
14026f1fd:     	testq	%rdx, %rdx
14026f200:     	je	0x14026f20b <.text+0x26e20b>
14026f202:     	movq	0x10(%rax), %r8
14026f206:     	callq	0x140613c20 <.text+0x612c20>
14026f20b:     	movl	$0x18, %edx
14026f210:     	movl	$0x8, %r8d
14026f216:     	movq	0x2b0(%rbp), %rcx
14026f21d:     	callq	0x140613c20 <.text+0x612c20>
14026f222:     	movq	0xd8(%rbp), %rax
14026f229:     	leaq	0x228(%rbp), %rcx
14026f230:     	movq	%rax, 0x40(%rcx)
14026f234:     	movupd	0x98(%rbp), %xmm0
14026f23c:     	movups	0xa8(%rbp), %xmm1
14026f243:     	movups	0xb8(%rbp), %xmm2
14026f24a:     	movups	0xc8(%rbp), %xmm3
14026f251:     	movups	%xmm3, 0x30(%rcx)
14026f255:     	movups	%xmm2, 0x20(%rcx)
14026f259:     	movups	%xmm1, 0x10(%rcx)
14026f25d:     	movupd	%xmm0, (%rcx)
14026f261:     	movw	$0x9, 0x220(%rbp)
14026f26a:     	movb	$0x0, 0x2e7(%rbp)
14026f271:     	leaq	0x110(%rbp), %rcx
14026f278:     	leaq	0x1c8(%rbp), %rdx
14026f27f:     	movq	%r12, %r8
14026f282:     	callq	0x1403c58b0 <.text+0x3c48b0>
14026f287:     	movzwl	0x110(%rbp), %eax
14026f28e:     	cmpw	$0x12, %ax
14026f292:     	je	0x14026e6c0 <.text+0x26d6c0>
14026f298:     	jmp	0x14026f9f3 <.text+0x26e9f3>
14026f29d:     	movq	0x2c0(%rbp), %rdx
14026f2a4:     	addq	$0x10, %rdx
14026f2a8:     	movq	%r12, %rcx
14026f2ab:     	movb	$0x1, %r8b
14026f2ae:     	callq	0x14026d8a0 <.text+0x26c8a0>
14026f2b3:     	cmpl	$0x2, 0x220(%rbp)
14026f2ba:     	movq	%r13, %r15
14026f2bd:     	jne	0x14026f339 <.text+0x26e339>
14026f2bf:     	movq	0x238(%rbp), %rax
14026f2c6:     	movl	%eax, %ecx
14026f2c8:     	andl	$0x3, %ecx
14026f2cb:     	cmpl	$0x1, %ecx
14026f2ce:     	jne	0x14026f339 <.text+0x26e339>
14026f2d0:     	leaq	-0x1(%rax), %rcx
14026f2d4:     	movq	%rcx, 0x2b0(%rbp)
14026f2db:     	movq	-0x1(%rax), %rcx
14026f2df:     	movq	%rcx, 0x2d8(%rbp)
14026f2e6:     	movq	0x7(%rax), %rax
14026f2ea:     	movq	%rax, 0x2d0(%rbp)
14026f2f1:     	movq	(%rax), %rax
14026f2f4:     	testq	%rax, %rax
14026f2f7:     	je	0x14026f302 <.text+0x26e302>
14026f2f9:     	movq	0x2d8(%rbp), %rcx
14026f300:     	callq	*%rax
14026f302:     	movq	0x2d8(%rbp), %rcx
14026f309:     	movq	0x2d0(%rbp), %rax
14026f310:     	movq	0x8(%rax), %rdx
14026f314:     	testq	%rdx, %rdx
14026f317:     	je	0x14026f322 <.text+0x26e322>
14026f319:     	movq	0x10(%rax), %r8
14026f31d:     	callq	0x140613c20 <.text+0x612c20>
14026f322:     	movl	$0x18, %edx
14026f327:     	movl	$0x8, %r8d
14026f32d:     	movq	0x2b0(%rbp), %rcx
14026f334:     	callq	0x140613c20 <.text+0x612c20>
14026f339:     	movl	$0x0, 0x54(%rbp)
14026f340:     	movlps	%xmm9, 0x60(%rbp)
14026f345:     	movq	0x200(%rbp), %rcx
14026f34c:     	movq	0x218(%rbp), %rax
14026f353:     	cmpq	%rcx, %rax
14026f356:     	je	0x14026f378 <.text+0x26e378>
14026f358:     	jmp	0x14026f39c <.text+0x26e39c>
14026f35a:     	movshdup	%xmm9, %xmm7            # xmm7 = xmm9[1,1,3,3]
14026f35f:     	movb	$0x1, %bl
14026f361:     	movaps	%xmm9, %xmm8
14026f365:     	movq	0x200(%rbp), %rcx
14026f36c:     	movq	0x218(%rbp), %rax
14026f373:     	cmpq	%rcx, %rax
14026f376:     	jne	0x14026f39c <.text+0x26e39c>
14026f378:     	leaq	0x200(%rbp), %rcx
14026f37f:     	leaq	0x2738782(%rip), %rdx   # 0x1429a7b08
14026f386:     	callq	0x140511380 <.text+0x510380>
14026f38b:     	movq	0x200(%rbp), %rcx
14026f392:     	movq	0x218(%rbp), %rax
14026f399:     	movq	%r13, %r15
14026f39c:     	addq	0x210(%rbp), %rax
14026f3a3:     	cmpq	%rcx, %rax
14026f3a6:     	movl	$0x0, %edx
14026f3ab:     	cmovaeq	%rcx, %rdx
14026f3af:     	subq	%rdx, %rax
14026f3b2:     	movq	0x208(%rbp), %rcx
14026f3b9:     	movq	%rsi, (%rcx,%rax,8)
14026f3bd:     	incq	0x218(%rbp)
14026f3c4:     	jmp	0x14026e6c0 <.text+0x26d6c0>
14026f3c9:     	movl	%r13d, %ebx
14026f3cc:     	jmp	0x14026f222 <.text+0x26e222>
14026f3d1:     	movb	$0x1, 0x2e5(%rbp)
14026f3d8:     	movq	%rdi, %rcx
14026f3db:     	callq	0x1428d8750 <.text+0x28d7750>
14026f3e0:     	jmp	0x14026e7e1 <.text+0x26d7e1>
14026f3e5:     	movb	$0x1, 0x2e5(%rbp)
14026f3ec:     	callq	0x1428d86b0 <.text+0x28d76b0>
14026f3f1:     	xorb	$0x1, %al
14026f3f3:     	movl	%eax, 0x2d0(%rbp)
14026f3f9:     	movq	%rdi, 0x2d8(%rbp)
14026f400:     	movzbl	0x11(%rsi), %eax
14026f404:     	testb	%al, %al
14026f406:     	je	0x14026e814 <.text+0x26d814>
14026f40c:     	jmp	0x14026fbd5 <.text+0x26ebd5>
14026f411:     	movb	$0x0, 0x2e5(%rbp)
14026f418:     	callq	0x1428d86b0 <.text+0x28d76b0>
14026f41d:     	testb	%al, %al
14026f41f:     	jne	0x14026e8c3 <.text+0x26d8c3>
14026f425:     	movb	$0x1, 0x11(%rsi)
14026f429:     	jmp	0x14026e8c3 <.text+0x26d8c3>
14026f42e:     	nop
14026f430:     	cmpb	$0x0, -0x30(%rbp)
14026f434:     	jne	0x14026fa20 <.text+0x26ea20>
14026f43a:     	movq	%r13, %rdx
14026f43d:     	testb	$0x1, %bl
14026f440:     	je	0x14026f50e <.text+0x26e50e>
14026f446:     	xorps	%xmm0, %xmm0
14026f449:     	cvtss2sd	%xmm8, %xmm0
14026f44e:     	movq	0x2c0(%rbp), %r15
14026f455:     	callq	0x140697820 <.text+0x696820>
14026f45a:     	movl	%eax, %edi
14026f45c:     	xorps	%xmm0, %xmm0
14026f45f:     	cvtss2sd	%xmm7, %xmm0
14026f463:     	callq	0x140697820 <.text+0x696820>
14026f468:     	movl	%eax, %esi
14026f46a:     	leaq	0x10(%r15), %rcx
14026f46e:     	callq	0x14069d680 <.text+0x69c680>
14026f473:     	movq	%rax, %rbx
14026f476:     	testb	$0x1, %dl
14026f479:     	jne	0x14026f491 <.text+0x26e491>
14026f47b:     	movq	0x33bcf9e(%rip), %rax   # 0x14362c420
14026f482:     	movq	(%rax), %rax
14026f485:     	shlq	%rax
14026f488:     	testq	%rax, %rax
14026f48b:     	jne	0x14026f98f <.text+0x26e98f>
14026f491:     	xorl	%eax, %eax
14026f493:     	xchgb	%al, (%rbx)
14026f495:     	cmpb	$0x2, %al
14026f497:     	je	0x14026f968 <.text+0x26e968>
14026f49d:     	xorps	%xmm0, %xmm0
14026f4a0:     	cvtsi2sd	%edi, %xmm0
14026f4a4:     	callq	0x140697820 <.text+0x696820>
14026f4a9:     	movl	%eax, %edi
14026f4ab:     	xorps	%xmm0, %xmm0
14026f4ae:     	cvtsi2sd	%esi, %xmm0
14026f4b2:     	callq	0x140697820 <.text+0x696820>
14026f4b7:     	movl	%eax, %ebx
14026f4b9:     	xorps	%xmm0, %xmm0
14026f4bc:     	cvtsi2sd	%edi, %xmm0
14026f4c0:     	callq	0x140697820 <.text+0x696820>
14026f4c5:     	movl	%eax, %esi
14026f4c7:     	xorps	%xmm0, %xmm0
14026f4ca:     	cvtsi2sd	%ebx, %xmm0
14026f4ce:     	callq	0x140697820 <.text+0x696820>
14026f4d3:     	movl	%esi, 0x220(%rbp)
14026f4d9:     	movl	%eax, 0x224(%rbp)
14026f4df:     	movq	0x18(%r15), %rcx
14026f4e3:     	movq	%r12, %rdx
14026f4e6:     	callq	*0x2f2330c(%rip)        # 0x1431927f8
14026f4ec:     	testl	%eax, %eax
14026f4ee:     	je	0x14026f506 <.text+0x26e506>
14026f4f0:     	movl	0x220(%rbp), %ecx
14026f4f6:     	movl	0x224(%rbp), %edx
14026f4fc:     	callq	*0x2f234fe(%rip)        # 0x143192a00
14026f502:     	testl	%eax, %eax
14026f504:     	jne	0x14026f50b <.text+0x26e50b>
14026f506:     	callq	0x140d6bb70 <.text+0xd6ab70>
14026f50b:     	movq	%r13, %rdx
14026f50e:     	cmpl	$0x4, %edx
14026f511:     	jne	0x14026f740 <.text+0x26e740>
14026f517:     	movq	0x218(%rbp), %rdi
14026f51e:     	movq	$0x0, 0x218(%rbp)
14026f529:     	leaq	0x200(%rbp), %rax
14026f530:     	movq	%rax, -0x38(%rbp)
14026f534:     	movq	%rdi, -0x30(%rbp)
14026f538:     	leaq	-0x1c(%rbp), %rax
14026f53c:     	movups	%xmm6, -0xc(%rax)
14026f540:     	movq	%rdi, -0x18(%rbp)
14026f544:     	xorl	%esi, %esi
14026f546:     	movq	%rdi, %rbx
14026f549:     	nopl	(%rax)
14026f550:     	subq	$0x1, %rbx
14026f554:     	jb	0x14026f5d0 <.text+0x26e5d0>
14026f556:     	movq	0x210(%rbp), %rax
14026f55d:     	addq	%rsi, %rax
14026f560:     	movq	0x200(%rbp), %rcx
14026f567:     	cmpq	%rcx, %rax
14026f56a:     	movl	$0x0, %edx
14026f56f:     	cmovaeq	%rcx, %rdx
14026f573:     	movq	0x208(%rbp), %rcx
14026f57a:     	subq	%rdx, %rax
14026f57d:     	incq	%rsi
14026f580:     	movq	%rsi, -0x28(%rbp)
14026f584:     	movq	%rbx, -0x18(%rbp)
14026f588:     	movq	(%rcx,%rax,8), %rax
14026f58c:     	movq	%rax, 0x228(%rbp)
14026f593:     	movw	$0xf, 0x220(%rbp)
14026f59c:     	leaq	0x110(%rbp), %rcx
14026f5a3:     	leaq	0x1c8(%rbp), %rdx
14026f5aa:     	movq	%r12, %r8
14026f5ad:     	callq	0x1403c58b0 <.text+0x3c48b0>
14026f5b2:     	movzwl	0x110(%rbp), %eax
14026f5b9:     	cmpw	$0x12, %ax
14026f5bd:     	je	0x14026f550 <.text+0x26e550>
14026f5bf:     	jmp	0x14026f9a5 <.text+0x26e9a5>
14026f5c4:     	nopw	%cs:(%rax,%rax)
14026f5d0:     	movq	0x218(%rbp), %r8
14026f5d7:     	testq	%r8, %r8
14026f5da:     	jne	0x14026f975 <.text+0x26e975>
14026f5e0:     	leaq	0x210(%rbp), %rax
14026f5e7:     	movups	%xmm6, (%rax)
14026f5ea:     	cmpb	$0x0, 0x2e3(%rbp)
14026f5f1:     	je	0x14026f870 <.text+0x26e870>
14026f5f7:     	movb	$0x0, 0x2e3(%rbp)
14026f5fe:     	cmpl	$0x1, 0x78(%rbp)
14026f602:     	jne	0x14026f870 <.text+0x26e870>
14026f608:     	leaq	-0x38(%rbp), %rcx
14026f60c:     	leaq	0x80(%rbp), %rdx
14026f613:     	callq	0x1403ac400 <.text+0x3ab400>
14026f618:     	xorl	%eax, %eax
14026f61a:     	cmpq	-0x38(%rbp), %rax
14026f61e:     	jno	0x14026f820 <.text+0x26e820>
14026f624:     	movq	-0x30(%rbp), %rax
14026f628:     	movq	%rax, 0x98(%rbp)
14026f62f:     	movq	0x33bcd72(%rip), %rax   # 0x14362c3a8
14026f636:     	movq	(%rax), %rax
14026f639:     	cmpq	$0x2, %rax
14026f63d:     	jb	0x14026f722 <.text+0x26e722>
14026f643:     	leaq	0x98(%rbp), %rax
14026f64a:     	movq	%rax, 0x110(%rbp)
14026f651:     	leaq	0xadd5c8(%rip), %rax    # 0x140d4cc20 <.text+0xd4bc20>
14026f658:     	movq	%rax, 0x118(%rbp)
14026f65f:     	leaq	0x27386ba(%rip), %rcx   # 0x1429a7d20
14026f666:     	callq	0x14067dd80 <.text+0x67cd80>
14026f66b:     	movupd	(%rax), %xmm0
14026f66f:     	movl	0x10(%rax), %eax
14026f672:     	movq	$0x2, 0x250(%rbp)
14026f67d:     	leaq	0x27384fc(%rip), %rcx   # 0x1429a7b80
14026f684:     	movq	%rcx, 0x258(%rbp)
14026f68b:     	movq	$0x16, 0x260(%rbp)
14026f696:     	leaq	0x2738673(%rip), %rdx   # 0x1429a7d10
14026f69d:     	movq	%rdx, 0x270(%rbp)
14026f6a4:     	movq	$0x1, 0x278(%rbp)
14026f6af:     	leaq	0x110(%rbp), %rdx
14026f6b6:     	movq	%rdx, 0x280(%rbp)
14026f6bd:     	movq	$0x1, 0x288(%rbp)
14026f6c8:     	movq	$0x0, 0x290(%rbp)
14026f6d3:     	movq	$0x0, 0x220(%rbp)
14026f6de:     	movq	%rcx, 0x228(%rbp)
14026f6e5:     	movq	$0x16, 0x230(%rbp)
14026f6f0:     	movq	$0x0, 0x238(%rbp)
14026f6fb:     	movupd	%xmm0, 0x240(%rbp)
14026f703:     	movl	$0x1, 0x268(%rbp)
14026f70d:     	movl	%eax, 0x26c(%rbp)
14026f713:     	leaq	0x2e4(%rbp), %rcx
14026f71a:     	movq	%r12, %rdx
14026f71d:     	callq	0x140d4a330 <.text+0xd49330>
14026f722:     	leaq	0x98(%rbp), %rcx
14026f729:     	callq	0x140d4cc30 <.text+0xd4bc30>
14026f72e:     	jmp	0x14026f870 <.text+0x26e870>
14026f733:     	nopw	%cs:(%rax,%rax)
14026f740:     	leal	-0x2(%rdx), %eax
14026f743:     	cmpl	$0x2, %eax
14026f746:     	movl	$0x2, %ecx
14026f74b:     	cmovael	%ecx, %eax
14026f74e:     	testl	%eax, %eax
14026f750:     	je	0x14026f7b0 <.text+0x26e7b0>
14026f752:     	cmpl	$0x1, %eax
14026f755:     	jne	0x14026f7f7 <.text+0x26e7f7>
14026f75b:     	movq	0x1f0(%rbp), %rax
14026f762:     	testl	%eax, %eax
14026f764:     	je	0x14026fba1 <.text+0x26eba1>
14026f76a:     	movl	0x2ac(%rbp), %ecx
14026f770:     	notl	%eax
14026f772:     	andl	%ecx, %eax
14026f774:     	movl	%eax, 0x2ac(%rbp)
14026f77a:     	testl	%ecx, %ecx
14026f77c:     	je	0x14026f517 <.text+0x26e517>
14026f782:     	movb	$0x1, %dl
14026f784:     	testl	%eax, %eax
14026f786:     	jne	0x14026f517 <.text+0x26e517>
14026f78c:     	cmpl	$0x0, 0x2bc(%rbp)
14026f793:     	je	0x14026f7e2 <.text+0x26e7e2>
14026f795:     	movq	0x1e8(%rbp), %rax
14026f79c:     	testq	%rax, %rax
14026f79f:     	je	0x14026f517 <.text+0x26e517>
14026f7a5:     	movzbl	%dl, %ecx
14026f7a8:     	movl	%ecx, 0x8(%rax)
14026f7ab:     	jmp	0x14026f517 <.text+0x26e517>
14026f7b0:     	movq	0x1f0(%rbp), %rdx
14026f7b7:     	testl	%edx, %edx
14026f7b9:     	je	0x14026fbbb <.text+0x26ebbb>
14026f7bf:     	movl	0x2ac(%rbp), %eax
14026f7c5:     	movl	%eax, %ecx
14026f7c7:     	orl	%edx, %ecx
14026f7c9:     	movl	%ecx, 0x2ac(%rbp)
14026f7cf:     	testl	%eax, %eax
14026f7d1:     	jne	0x14026f517 <.text+0x26e517>
14026f7d7:     	xorl	%edx, %edx
14026f7d9:     	cmpl	$0x0, 0x2bc(%rbp)
14026f7e0:     	jne	0x14026f795 <.text+0x26e795>
14026f7e2:     	movq	0x2c0(%rbp), %rcx
14026f7e9:     	addq	$0x10, %rcx
14026f7ed:     	callq	0x14026db70 <.text+0x26cb70>
14026f7f2:     	jmp	0x14026f959 <.text+0x26e959>
14026f7f7:     	cmpl	$0x0, 0x2bc(%rbp)
14026f7fe:     	je	0x14026f8ef <.text+0x26e8ef>
14026f804:     	testl	%edx, %edx
14026f806:     	je	0x14026f93f <.text+0x26e93f>
14026f80c:     	movl	%edx, 0x2bc(%rbp)
14026f812:     	cmpl	$0x1, %edx
14026f815:     	je	0x14026f910 <.text+0x26e910>
14026f81b:     	jmp	0x14026f517 <.text+0x26e517>
14026f820:     	movupd	-0x38(%rbp), %xmm0
14026f825:     	movups	-0x28(%rbp), %xmm1
14026f829:     	leaq	0x228(%rbp), %rax
14026f830:     	movups	%xmm1, 0x10(%rax)
14026f834:     	movupd	%xmm0, (%rax)
14026f838:     	movw	$0x11, 0x220(%rbp)
14026f841:     	leaq	0x110(%rbp), %rcx
14026f848:     	leaq	0x1c8(%rbp), %rdx
14026f84f:     	movq	%r12, %r8
14026f852:     	callq	0x1403c58b0 <.text+0x3c48b0>
14026f857:     	cmpw	$0x12, 0x110(%rbp)
14026f85f:     	jne	0x14026fb76 <.text+0x26eb76>
14026f865:     	nopw	%cs:(%rax,%rax)
14026f870:     	callq	0x140d635e0 <.text+0xd625e0>
14026f875:     	movq	%rax, 0x68(%rbp)
14026f879:     	movl	%edx, 0x70(%rbp)
14026f87c:     	leaq	0x68(%rbp), %rcx
14026f880:     	movq	0x108(%rbp), %rdx
14026f887:     	movl	0x1fc(%rbp), %r8d
14026f88e:     	callq	0x140d63680 <.text+0xd62680>
14026f893:     	xorl	%ecx, %ecx
14026f895:     	testl	%edx, %edx
14026f897:     	setne	%cl
14026f89a:     	xorl	%edx, %edx
14026f89c:     	cmpq	$0x2, %rax
14026f8a0:     	setae	%dl
14026f8a3:     	cmpq	$0x1, %rax
14026f8a7:     	cmovnel	%edx, %ecx
14026f8aa:     	cmpb	$0x1, %cl
14026f8ad:     	jne	0x14026f8de <.text+0x26e8de>
14026f8af:     	cmpl	$0x1, 0x2bc(%rbp)
14026f8b6:     	jne	0x14026f8ca <.text+0x26e8ca>
14026f8b8:     	movq	0x2c0(%rbp), %rcx
14026f8bf:     	addq	$0x10, %rcx
14026f8c3:     	xorl	%edx, %edx
14026f8c5:     	callq	0x14026db70 <.text+0x26cb70>
14026f8ca:     	movq	0x68(%rbp), %rax
14026f8ce:     	movq	%rax, 0x108(%rbp)
14026f8d5:     	movl	0x70(%rbp), %eax
14026f8d8:     	movl	%eax, 0x1fc(%rbp)
14026f8de:     	xorl	%ecx, %ecx
14026f8e0:     	movl	$0xf4240, %edx          # imm = 0xF4240
14026f8e5:     	callq	0x140d55980 <.text+0xd54980>
14026f8ea:     	jmp	0x14026e5d7 <.text+0x26d5d7>
14026f8ef:     	movl	%edx, 0x2bc(%rbp)
14026f8f5:     	cmpl	$0x1, %edx
14026f8f8:     	jne	0x14026f517 <.text+0x26e517>
14026f8fe:     	movq	0x2c0(%rbp), %rcx
14026f905:     	addq	$0x10, %rcx
14026f909:     	xorl	%edx, %edx
14026f90b:     	callq	0x14026db70 <.text+0x26cb70>
14026f910:     	movq	0x1f0(%rbp), %rcx
14026f917:     	movl	%ecx, 0x1f8(%rbp)
14026f91d:     	movq	0x1e8(%rbp), %rax
14026f924:     	movl	$0x1, 0x2bc(%rbp)
14026f92e:     	testq	%rax, %rax
14026f931:     	je	0x14026f517 <.text+0x26e517>
14026f937:     	movl	%ecx, 0xc(%rax)
14026f93a:     	jmp	0x14026f517 <.text+0x26e517>
14026f93f:     	movq	0x2c0(%rbp), %rcx
14026f946:     	addq	$0x10, %rcx
14026f94a:     	cmpl	$0x0, 0x2ac(%rbp)
14026f951:     	sete	%dl
14026f954:     	callq	0x14026db70 <.text+0x26cb70>
14026f959:     	movl	$0x0, 0x2bc(%rbp)
14026f963:     	jmp	0x14026f517 <.text+0x26e517>
14026f968:     	movq	%rbx, %rcx
14026f96b:     	callq	0x1428d8810 <.text+0x28d7810>
14026f970:     	jmp	0x14026f49d <.text+0x26e49d>
14026f975:     	movq	%r8, %r9
14026f978:     	negq	%r9
14026f97b:     	leaq	0x200(%rbp), %rcx
14026f982:     	movq	%rdi, %rdx
14026f985:     	callq	0x1428a1fc0 <.text+0x28a0fc0>
14026f98a:     	jmp	0x14026f5e0 <.text+0x26e5e0>
14026f98f:     	callq	0x1428d86b0 <.text+0x28d76b0>
14026f994:     	testb	%al, %al
14026f996:     	jne	0x14026f491 <.text+0x26e491>
14026f99c:     	movb	$0x1, 0x1(%rbx)
14026f9a0:     	jmp	0x14026f491 <.text+0x26e491>
14026f9a5:     	movzwl	%ax, %ecx
14026f9a8:     	addl	$-0x4, %ecx
14026f9ab:     	cmpw	$0xe, %cx
14026f9af:     	movl	$0x7, %eax
14026f9b4:     	cmovbl	%ecx, %eax
14026f9b7:     	cmpw	$0xc, %ax
14026f9bb:     	ja	0x14026faef <.text+0x26eaef>
14026f9c1:     	movzwl	%ax, %eax
14026f9c4:     	movl	$0x1f5f, %ecx           # imm = 0x1F5F
14026f9c9:     	btl	%eax, %ecx
14026f9cc:     	jae	0x14026fabd <.text+0x26eabd>
14026f9d2:     	movq	0x218(%rbp), %r8
14026f9d9:     	testq	%r8, %r8
14026f9dc:     	jne	0x14026fb87 <.text+0x26eb87>
14026f9e2:     	xorpd	%xmm0, %xmm0
14026f9e6:     	leaq	0x210(%rbp), %rax
14026f9ed:     	movupd	%xmm0, (%rax)
14026f9f1:     	jmp	0x14026fa20 <.text+0x26ea20>
14026f9f3:     	movzwl	%ax, %ecx
14026f9f6:     	addl	$-0x4, %ecx
14026f9f9:     	cmpw	$0xe, %cx
14026f9fd:     	movl	$0x7, %eax
14026fa02:     	cmovbl	%ecx, %eax
14026fa05:     	cmpw	$0xc, %ax
14026fa09:     	ja	0x14026fb4f <.text+0x26eb4f>
14026fa0f:     	movzwl	%ax, %eax
14026fa12:     	movl	$0x1f5f, %ecx           # imm = 0x1F5F
14026fa17:     	btl	%eax, %ecx
14026fa1a:     	jae	0x14026fb16 <.text+0x26eb16>
14026fa20:     	movq	0x200(%rbp), %rdx
14026fa27:     	testq	%rdx, %rdx
14026fa2a:     	je	0x14026fa42 <.text+0x26ea42>
14026fa2c:     	movq	0x208(%rbp), %rcx
14026fa33:     	shlq	$0x3, %rdx
14026fa37:     	movl	$0x8, %r8d
14026fa3d:     	callq	0x140613c20 <.text+0x612c20>
14026fa42:     	leaq	0x88(%rbp), %rcx
14026fa49:     	callq	0x140607fc0 <.text+0x606fc0>
14026fa4e:     	leaq	0x1c8(%rbp), %rcx
14026fa55:     	callq	0x140608df0 <.text+0x607df0>
14026fa5a:     	movq	0x1e0(%rbp), %rcx
14026fa61:     	callq	0x1405f18c0 <.text+0x5f08c0>
14026fa66:     	movq	0x2c0(%rbp), %rax
14026fa6d:     	lock
14026fa6e:     	decq	(%rax)
14026fa71:     	jne	0x14026fa7f <.text+0x26ea7f>
14026fa73:     	leaq	0x2c0(%rbp), %rcx
14026fa7a:     	callq	0x140631840 <.text+0x630840>
14026fa7f:     	movq	0x1d8(%rbp), %rcx
14026fa86:     	callq	0x1405f97d0 <.text+0x5f87d0>
14026fa8b:     	movaps	0x2f0(%rbp), %xmm6
14026fa92:     	movaps	0x300(%rbp), %xmm7
14026fa99:     	movaps	0x310(%rbp), %xmm8
14026faa1:     	movaps	0x320(%rbp), %xmm9
14026faa9:     	addq	$0x3b8, %rsp            # imm = 0x3B8
14026fab0:     	popq	%rbx
14026fab1:     	popq	%rdi
14026fab2:     	popq	%rsi
14026fab3:     	popq	%r12
14026fab5:     	popq	%r13
14026fab7:     	popq	%r14
14026fab9:     	popq	%r15
14026fabb:     	popq	%rbp
14026fabc:     	retq
14026fabd:     	cmpl	$0x5, %eax
14026fac0:     	jne	0x14026fade <.text+0x26eade>
14026fac2:     	movq	0x118(%rbp), %rdx
14026fac9:     	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
14026fad3:     	cmpq	%rax, %rdx
14026fad6:     	jl	0x14026f9d2 <.text+0x26e9d2>
14026fadc:     	jmp	0x14026faf6 <.text+0x26eaf6>
14026fade:     	leaq	0x110(%rbp), %rcx
14026fae5:     	callq	0x1405e8000 <.text+0x5e7000>
14026faea:     	jmp	0x14026f9d2 <.text+0x26e9d2>
14026faef:     	movq	0x118(%rbp), %rdx
14026faf6:     	testq	%rdx, %rdx
14026faf9:     	je	0x14026f9d2 <.text+0x26e9d2>
14026faff:     	movq	0x120(%rbp), %rcx
14026fb06:     	movl	$0x1, %r8d
14026fb0c:     	callq	0x140613c20 <.text+0x612c20>
14026fb11:     	jmp	0x14026f9d2 <.text+0x26e9d2>
14026fb16:     	cmpl	$0x5, %eax
14026fb19:     	jne	0x14026fb37 <.text+0x26eb37>
14026fb1b:     	movq	0x118(%rbp), %rdx
14026fb22:     	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
14026fb2c:     	cmpq	%rax, %rdx
14026fb2f:     	jl	0x14026fa20 <.text+0x26ea20>
14026fb35:     	jmp	0x14026fb56 <.text+0x26eb56>
14026fb37:     	movb	$0x0, 0x2e7(%rbp)
14026fb3e:     	leaq	0x110(%rbp), %rcx
14026fb45:     	callq	0x1405e8000 <.text+0x5e7000>
14026fb4a:     	jmp	0x14026fa20 <.text+0x26ea20>
14026fb4f:     	movq	0x118(%rbp), %rdx
14026fb56:     	testq	%rdx, %rdx
14026fb59:     	je	0x14026fa20 <.text+0x26ea20>
14026fb5f:     	movq	0x120(%rbp), %rcx
14026fb66:     	movl	$0x1, %r8d
14026fb6c:     	callq	0x140613c20 <.text+0x612c20>
14026fb71:     	jmp	0x14026fa20 <.text+0x26ea20>
14026fb76:     	leaq	0x110(%rbp), %rcx
14026fb7d:     	callq	0x140268ca0 <.text+0x267ca0>
14026fb82:     	jmp	0x14026fa20 <.text+0x26ea20>
14026fb87:     	movq	%r8, %r9
14026fb8a:     	negq	%r9
14026fb8d:     	leaq	0x200(%rbp), %rcx
14026fb94:     	movq	%rdi, %rdx
14026fb97:     	callq	0x1428a1fc0 <.text+0x28a0fc0>
14026fb9c:     	jmp	0x14026f9e2 <.text+0x26e9e2>
14026fba1:     	leaq	0x27380e0(%rip), %rcx   # 0x1429a7c88
14026fba8:     	leaq	0x2738119(%rip), %r8    # 0x1429a7cc8
14026fbaf:     	movl	$0x24, %edx
14026fbb4:     	callq	0x1428d9430 <.text+0x28d8430>
14026fbb9:     	jmp	0x14026fc1a <.text+0x26ec1a>
14026fbbb:     	leaq	0x27380c6(%rip), %rcx   # 0x1429a7c88
14026fbc2:     	leaq	0x27380e7(%rip), %r8    # 0x1429a7cb0
14026fbc9:     	movl	$0x24, %edx
14026fbce:     	callq	0x1428d9430 <.text+0x28d8430>
14026fbd3:     	jmp	0x14026fc1a <.text+0x26ec1a>
14026fbd5:     	movq	0x2d8(%rbp), %rax
14026fbdc:     	movq	%rax, 0x220(%rbp)
14026fbe3:     	movl	0x2d0(%rbp), %eax
14026fbe9:     	movb	%al, 0x228(%rbp)
14026fbef:     	leaq	0x2737f2a(%rip), %rax   # 0x1429a7b20
14026fbf6:     	movq	%rax, 0x20(%rsp)
14026fbfb:     	leaq	0x2737cc6(%rip), %rcx   # 0x1429a78c8
14026fc02:     	leaq	0x2737c9f(%rip), %r9    # 0x1429a78a8
14026fc09:     	leaq	0x220(%rbp), %r8
14026fc10:     	movl	$0x2b, %edx
14026fc15:     	callq	0x1428d9760 <.text+0x28d8760>
14026fc1a:     	ud2
14026fc1c:     	nopl	(%rax)
14026fc20:     	movq	%rdx, 0x10(%rsp)
14026fc25:     	pushq	%rbp
14026fc26:     	pushq	%r15
14026fc28:     	pushq	%r14
14026fc2a:     	pushq	%r13
14026fc2c:     	pushq	%r12
14026fc2e:     	pushq	%rsi
14026fc2f:     	pushq	%rdi
14026fc30:     	pushq	%rbx
14026fc31:     	subq	$0x68, %rsp
14026fc35:     	leaq	0x80(%rdx), %rbp
14026fc3c:     	movaps	%xmm9, 0x20(%rsp)
14026fc42:     	movaps	%xmm8, 0x30(%rsp)
14026fc48:     	movaps	%xmm7, 0x40(%rsp)
14026fc4d:     	movaps	%xmm6, 0x50(%rsp)
14026fc52:     	leaq	0x98(%rbp), %rcx
14026fc59:     	callq	0x140d4cc30 <.text+0xd4bc30>
14026fc5e:     	movaps	0x50(%rsp), %xmm6
14026fc63:     	movaps	0x40(%rsp), %xmm7
14026fc68:     	movaps	0x30(%rsp), %xmm8
14026fc6e:     	movaps	0x20(%rsp), %xmm9
14026fc74:     	addq	$0x68, %rsp
14026fc78:     	popq	%rbx
14026fc79:     	popq	%rdi
14026fc7a:     	popq	%rsi
14026fc7b:     	popq	%r12
14026fc7d:     	popq	%r13
14026fc7f:     	popq	%r14
14026fc81:     	popq	%r15
14026fc83:     	popq	%rbp
14026fc84:     	retq
14026fc85:     	nopw	%cs:(%rax,%rax)
14026fc90:     	movq	%rdx, 0x10(%rsp)
14026fc95:     	pushq	%rbp
14026fc96:     	pushq	%r15
14026fc98:     	pushq	%r14
14026fc9a:     	pushq	%r13
14026fc9c:     	pushq	%r12
14026fc9e:     	pushq	%rsi
14026fc9f:     	pushq	%rdi
14026fca0:     	pushq	%rbx
14026fca1:     	subq	$0x68, %rsp
14026fca5:     	leaq	0x80(%rdx), %rbp
14026fcac:     	movaps	%xmm9, 0x20(%rsp)
14026fcb2:     	movaps	%xmm8, 0x30(%rsp)
14026fcb8:     	movaps	%xmm7, 0x40(%rsp)
14026fcbd:     	movaps	%xmm6, 0x50(%rsp)
14026fcc2:     	movq	0x2d8(%rbp), %rcx
14026fcc9:     	movq	0x2d0(%rbp), %rax
14026fcd0:     	movq	0x8(%rax), %rdx
14026fcd4:     	testq	%rdx, %rdx
14026fcd7:     	je	0x14026fce9 <.text+0x26ece9>
14026fcd9:     	movq	0x2d0(%rbp), %rax
14026fce0:     	movq	0x10(%rax), %r8
14026fce4:     	callq	0x140613c20 <.text+0x612c20>
14026fce9:     	movl	$0x18, %edx
14026fcee:     	movl	$0x8, %r8d
14026fcf4:     	movq	0x2b0(%rbp), %rcx
14026fcfb:     	callq	0x140613c20 <.text+0x612c20>
14026fd00:     	movb	$0x0, 0x2e6(%rbp)
14026fd07:     	movaps	0x50(%rsp), %xmm6
14026fd0c:     	movaps	0x40(%rsp), %xmm7
14026fd11:     	movaps	0x30(%rsp), %xmm8
14026fd17:     	movaps	0x20(%rsp), %xmm9
14026fd1d:     	addq	$0x68, %rsp
14026fd21:     	popq	%rbx
14026fd22:     	popq	%rdi
14026fd23:     	popq	%rsi
14026fd24:     	popq	%r12
14026fd26:     	popq	%r13
14026fd28:     	popq	%r14
14026fd2a:     	popq	%r15
14026fd2c:     	popq	%rbp
14026fd2d:     	retq
14026fd2e:     	nop
14026fd30:     	movq	%rdx, 0x10(%rsp)
14026fd35:     	pushq	%rbp
14026fd36:     	pushq	%r15
14026fd38:     	pushq	%r14
14026fd3a:     	pushq	%r13
14026fd3c:     	pushq	%r12
14026fd3e:     	pushq	%rsi
14026fd3f:     	pushq	%rdi
14026fd40:     	pushq	%rbx
14026fd41:     	subq	$0x68, %rsp
14026fd45:     	leaq	0x80(%rdx), %rbp
14026fd4c:     	movaps	%xmm9, 0x20(%rsp)
14026fd52:     	movaps	%xmm8, 0x30(%rsp)
14026fd58:     	movaps	%xmm7, 0x40(%rsp)
14026fd5d:     	movaps	%xmm6, 0x50(%rsp)
14026fd62:     	movq	0x2d8(%rbp), %rcx
14026fd69:     	movq	0x2d0(%rbp), %rax
14026fd70:     	movq	0x8(%rax), %rdx
14026fd74:     	testq	%rdx, %rdx
14026fd77:     	je	0x14026fd89 <.text+0x26ed89>
14026fd79:     	movq	0x2d0(%rbp), %rax
14026fd80:     	movq	0x10(%rax), %r8
14026fd84:     	callq	0x140613c20 <.text+0x612c20>
14026fd89:     	movl	$0x18, %edx
14026fd8e:     	movl	$0x8, %r8d
14026fd94:     	movq	0x2b0(%rbp), %rcx
14026fd9b:     	callq	0x140613c20 <.text+0x612c20>
14026fda0:     	movaps	0x50(%rsp), %xmm6
14026fda5:     	movaps	0x40(%rsp), %xmm7
14026fdaa:     	movaps	0x30(%rsp), %xmm8
14026fdb0:     	movaps	0x20(%rsp), %xmm9
14026fdb6:     	addq	$0x68, %rsp
14026fdba:     	popq	%rbx
14026fdbb:     	popq	%rdi
14026fdbc:     	popq	%rsi
14026fdbd:     	popq	%r12
14026fdbf:     	popq	%r13
14026fdc1:     	popq	%r14
14026fdc3:     	popq	%r15
14026fdc5:     	popq	%rbp
14026fdc6:     	retq
14026fdc7:     	nopw	(%rax,%rax)
14026fdd0:     	movq	%rdx, 0x10(%rsp)
14026fdd5:     	pushq	%rbp
14026fdd6:     	pushq	%r15
14026fdd8:     	pushq	%r14
14026fdda:     	pushq	%r13
14026fddc:     	pushq	%r12
14026fdde:     	pushq	%rsi
14026fddf:     	pushq	%rdi
14026fde0:     	pushq	%rbx
14026fde1:     	subq	$0x68, %rsp
14026fde5:     	leaq	0x80(%rdx), %rbp
14026fdec:     	movaps	%xmm9, 0x20(%rsp)
14026fdf2:     	movaps	%xmm8, 0x30(%rsp)
14026fdf8:     	movaps	%xmm7, 0x40(%rsp)
14026fdfd:     	movaps	%xmm6, 0x50(%rsp)
14026fe02:     	movq	0x2d8(%rbp), %rcx
14026fe09:     	movq	0x2d0(%rbp), %rax
14026fe10:     	movq	0x8(%rax), %rdx
14026fe14:     	testq	%rdx, %rdx
14026fe17:     	je	0x14026fe29 <.text+0x26ee29>
14026fe19:     	movq	0x2d0(%rbp), %rax
14026fe20:     	movq	0x10(%rax), %r8
14026fe24:     	callq	0x140613c20 <.text+0x612c20>
14026fe29:     	movl	$0x18, %edx
14026fe2e:     	movl	$0x8, %r8d
14026fe34:     	movq	0x2b0(%rbp), %rcx
14026fe3b:     	callq	0x140613c20 <.text+0x612c20>
14026fe40:     	movaps	0x50(%rsp), %xmm6
14026fe45:     	movaps	0x40(%rsp), %xmm7
14026fe4a:     	movaps	0x30(%rsp), %xmm8
14026fe50:     	movaps	0x20(%rsp), %xmm9
14026fe56:     	addq	$0x68, %rsp
14026fe5a:     	popq	%rbx
14026fe5b:     	popq	%rdi
14026fe5c:     	popq	%rsi
14026fe5d:     	popq	%r12
14026fe5f:     	popq	%r13
14026fe61:     	popq	%r14
14026fe63:     	popq	%r15
14026fe65:     	popq	%rbp
14026fe66:     	retq
14026fe67:     	nopw	(%rax,%rax)
14026fe70:     	movq	%rdx, 0x10(%rsp)
14026fe75:     	pushq	%rbp
14026fe76:     	pushq	%r15
14026fe78:     	pushq	%r14
14026fe7a:     	pushq	%r13
14026fe7c:     	pushq	%r12
14026fe7e:     	pushq	%rsi
14026fe7f:     	pushq	%rdi
14026fe80:     	pushq	%rbx
14026fe81:     	subq	$0x68, %rsp
14026fe85:     	leaq	0x80(%rdx), %rbp
14026fe8c:     	movaps	%xmm9, 0x20(%rsp)
14026fe92:     	movaps	%xmm8, 0x30(%rsp)
14026fe98:     	movaps	%xmm7, 0x40(%rsp)
14026fe9d:     	movaps	%xmm6, 0x50(%rsp)
14026fea2:     	movq	0x2d8(%rbp), %rcx
14026fea9:     	movq	0x2d0(%rbp), %rax
14026feb0:     	movq	0x8(%rax), %rdx
14026feb4:     	testq	%rdx, %rdx
14026feb7:     	je	0x14026fec9 <.text+0x26eec9>
14026feb9:     	movq	0x2d0(%rbp), %rax
14026fec0:     	movq	0x10(%rax), %r8
14026fec4:     	callq	0x140613c20 <.text+0x612c20>
14026fec9:     	movl	$0x18, %edx
14026fece:     	movl	$0x8, %r8d
14026fed4:     	movq	0x2b0(%rbp), %rcx
14026fedb:     	callq	0x140613c20 <.text+0x612c20>
14026fee0:     	movb	$0x1, 0x2e7(%rbp)
14026fee7:     	movaps	0x50(%rsp), %xmm6
14026feec:     	movaps	0x40(%rsp), %xmm7
14026fef1:     	movaps	0x30(%rsp), %xmm8
14026fef7:     	movaps	0x20(%rsp), %xmm9
14026fefd:     	addq	$0x68, %rsp
14026ff01:     	popq	%rbx
14026ff02:     	popq	%rdi
14026ff03:     	popq	%rsi
14026ff04:     	popq	%r12
14026ff06:     	popq	%r13
14026ff08:     	popq	%r14
14026ff0a:     	popq	%r15
14026ff0c:     	popq	%rbp
14026ff0d:     	retq
14026ff0e:     	nop
14026ff10:     	movq	%rdx, 0x10(%rsp)
14026ff15:     	pushq	%rbp
14026ff16:     	pushq	%r15
14026ff18:     	pushq	%r14
14026ff1a:     	pushq	%r13
14026ff1c:     	pushq	%r12
14026ff1e:     	pushq	%rsi
14026ff1f:     	pushq	%rdi
14026ff20:     	pushq	%rbx
14026ff21:     	subq	$0x68, %rsp
14026ff25:     	leaq	0x80(%rdx), %rbp
14026ff2c:     	movaps	%xmm9, 0x20(%rsp)
14026ff32:     	movaps	%xmm8, 0x30(%rsp)
14026ff38:     	movaps	%xmm7, 0x40(%rsp)
14026ff3d:     	movaps	%xmm6, 0x50(%rsp)
14026ff42:     	movq	0x2d8(%rbp), %rcx
14026ff49:     	movq	0x2d0(%rbp), %rax
14026ff50:     	movq	0x8(%rax), %rdx
14026ff54:     	testq	%rdx, %rdx
14026ff57:     	je	0x14026ff69 <.text+0x26ef69>
14026ff59:     	movq	0x2d0(%rbp), %rax
14026ff60:     	movq	0x10(%rax), %r8
14026ff64:     	callq	0x140613c20 <.text+0x612c20>
14026ff69:     	movl	$0x18, %edx
14026ff6e:     	movl	$0x8, %r8d
14026ff74:     	movq	0x2b0(%rbp), %rcx
14026ff7b:     	callq	0x140613c20 <.text+0x612c20>
14026ff80:     	movb	$0x1, 0x2e7(%rbp)
14026ff87:     	movaps	0x50(%rsp), %xmm6
14026ff8c:     	movaps	0x40(%rsp), %xmm7
14026ff91:     	movaps	0x30(%rsp), %xmm8
14026ff97:     	movaps	0x20(%rsp), %xmm9
14026ff9d:     	addq	$0x68, %rsp
14026ffa1:     	popq	%rbx
14026ffa2:     	popq	%rdi
14026ffa3:     	popq	%rsi
14026ffa4:     	popq	%r12
14026ffa6:     	popq	%r13
14026ffa8:     	popq	%r14
14026ffaa:     	popq	%r15
14026ffac:     	popq	%rbp
14026ffad:     	retq
14026ffae:     	nop
14026ffb0:     	movq	%rdx, 0x10(%rsp)
14026ffb5:     	pushq	%rbp
14026ffb6:     	pushq	%r15
14026ffb8:     	pushq	%r14
14026ffba:     	pushq	%r13
14026ffbc:     	pushq	%r12
14026ffbe:     	pushq	%rsi
14026ffbf:     	pushq	%rdi
14026ffc0:     	pushq	%rbx
14026ffc1:     	subq	$0x68, %rsp
14026ffc5:     	leaq	0x80(%rdx), %rbp
14026ffcc:     	movaps	%xmm9, 0x20(%rsp)
14026ffd2:     	movaps	%xmm8, 0x30(%rsp)
14026ffd8:     	movaps	%xmm7, 0x40(%rsp)
14026ffdd:     	movaps	%xmm6, 0x50(%rsp)
14026ffe2:     	leaq	0x120(%rbp), %rcx
14026ffe9:     	callq	0x1405e7a00 <.text+0x5e6a00>
14026ffee:     	movb	$0x0, 0x2e6(%rbp)
14026fff5:     	movaps	0x50(%rsp), %xmm6
14026fffa:     	movaps	0x40(%rsp), %xmm7
14026ffff:     	movaps	0x30(%rsp), %xmm8
140270005:     	movaps	0x20(%rsp), %xmm9
14027000b:     	addq	$0x68, %rsp
14027000f:     	popq	%rbx
140270010:     	popq	%rdi
140270011:     	popq	%rsi
140270012:     	popq	%r12
140270014:     	popq	%r13
140270016:     	popq	%r14
140270018:     	popq	%r15
14027001a:     	popq	%rbp
14027001b:     	retq
14027001c:     	nopl	(%rax)
140270020:     	movq	%rdx, 0x10(%rsp)
140270025:     	pushq	%rbp
140270026:     	pushq	%r15
140270028:     	pushq	%r14
14027002a:     	pushq	%r13
14027002c:     	pushq	%r12
14027002e:     	pushq	%rsi
14027002f:     	pushq	%rdi
140270030:     	pushq	%rbx
140270031:     	subq	$0x68, %rsp
140270035:     	leaq	0x80(%rdx), %rbp
14027003c:     	movaps	%xmm9, 0x20(%rsp)
140270042:     	movaps	%xmm8, 0x30(%rsp)
140270048:     	movaps	%xmm7, 0x40(%rsp)
14027004d:     	movaps	%xmm6, 0x50(%rsp)
140270052:     	leaq	0x120(%rbp), %rcx
140270059:     	callq	0x1405e7a00 <.text+0x5e6a00>
14027005e:     	movb	$0x1, 0x2e7(%rbp)
140270065:     	movaps	0x50(%rsp), %xmm6
14027006a:     	movaps	0x40(%rsp), %xmm7
14027006f:     	movaps	0x30(%rsp), %xmm8
140270075:     	movaps	0x20(%rsp), %xmm9
14027007b:     	addq	$0x68, %rsp
14027007f:     	popq	%rbx
140270080:     	popq	%rdi
140270081:     	popq	%rsi
140270082:     	popq	%r12
140270084:     	popq	%r13
140270086:     	popq	%r14
140270088:     	popq	%r15
14027008a:     	popq	%rbp
14027008b:     	retq
14027008c:     	nopl	(%rax)
140270090:     	movq	%rdx, 0x10(%rsp)
140270095:     	pushq	%rbp
140270096:     	pushq	%r15
140270098:     	pushq	%r14
14027009a:     	pushq	%r13
14027009c:     	pushq	%r12
14027009e:     	pushq	%rsi
14027009f:     	pushq	%rdi
1402700a0:     	pushq	%rbx
1402700a1:     	subq	$0x68, %rsp
1402700a5:     	leaq	0x80(%rdx), %rbp
1402700ac:     	movaps	%xmm9, 0x20(%rsp)
1402700b2:     	movaps	%xmm8, 0x30(%rsp)
1402700b8:     	movaps	%xmm7, 0x40(%rsp)
1402700bd:     	movaps	%xmm6, 0x50(%rsp)
1402700c2:     	leaq	0x120(%rbp), %rcx
1402700c9:     	callq	0x1405e7a00 <.text+0x5e6a00>
1402700ce:     	movb	$0x1, 0x2e7(%rbp)
1402700d5:     	movaps	0x50(%rsp), %xmm6
1402700da:     	movaps	0x40(%rsp), %xmm7
1402700df:     	movaps	0x30(%rsp), %xmm8
1402700e5:     	movaps	0x20(%rsp), %xmm9
1402700eb:     	addq	$0x68, %rsp
1402700ef:     	popq	%rbx
1402700f0:     	popq	%rdi
1402700f1:     	popq	%rsi
1402700f2:     	popq	%r12
1402700f4:     	popq	%r13
1402700f6:     	popq	%r14
1402700f8:     	popq	%r15
1402700fa:     	popq	%rbp
1402700fb:     	retq
1402700fc:     	nopl	(%rax)
140270100:     	movq	%rdx, 0x10(%rsp)
140270105:     	pushq	%rbp
140270106:     	pushq	%r15
140270108:     	pushq	%r14
14027010a:     	pushq	%r13
14027010c:     	pushq	%r12
14027010e:     	pushq	%rsi
14027010f:     	pushq	%rdi
140270110:     	pushq	%rbx
140270111:     	subq	$0x68, %rsp
140270115:     	leaq	0x80(%rdx), %rbp
14027011c:     	movaps	%xmm9, 0x20(%rsp)
140270122:     	movaps	%xmm8, 0x30(%rsp)
140270128:     	movaps	%xmm7, 0x40(%rsp)
14027012d:     	movaps	%xmm6, 0x50(%rsp)
140270132:     	cmpb	$0x0, 0x2e6(%rbp)
140270139:     	je	0x140270180 <.text+0x26f180>
14027013b:     	movq	0x1a0(%rbp), %rcx
140270142:     	movl	$0xdc, %edx
140270147:     	movl	$0x4, %r8d
14027014d:     	callq	0x140613c20 <.text+0x612c20>
140270152:     	movb	$0x1, 0x2e7(%rbp)
140270159:     	movaps	0x50(%rsp), %xmm6
14027015e:     	movaps	0x40(%rsp), %xmm7
140270163:     	movaps	0x30(%rsp), %xmm8
140270169:     	movaps	0x20(%rsp), %xmm9
14027016f:     	addq	$0x68, %rsp
140270173:     	popq	%rbx
140270174:     	popq	%rdi
140270175:     	popq	%rsi
140270176:     	popq	%r12
140270178:     	popq	%r13
14027017a:     	popq	%r14
14027017c:     	popq	%r15
14027017e:     	popq	%rbp
14027017f:     	retq
140270180:     	movb	$0x1, 0x2e7(%rbp)
140270187:     	movaps	0x50(%rsp), %xmm6
14027018c:     	movaps	0x40(%rsp), %xmm7
140270191:     	movaps	0x30(%rsp), %xmm8
140270197:     	movaps	0x20(%rsp), %xmm9
14027019d:     	addq	$0x68, %rsp
1402701a1:     	popq	%rbx
1402701a2:     	popq	%rdi
1402701a3:     	popq	%rsi
1402701a4:     	popq	%r12
1402701a6:     	popq	%r13
1402701a8:     	popq	%r14
1402701aa:     	popq	%r15
1402701ac:     	popq	%rbp
1402701ad:     	retq
1402701ae:     	nop
1402701b0:     	movq	%rdx, 0x10(%rsp)
1402701b5:     	pushq	%rbp
1402701b6:     	pushq	%r15
1402701b8:     	pushq	%r14
1402701ba:     	pushq	%r13
1402701bc:     	pushq	%r12
1402701be:     	pushq	%rsi
1402701bf:     	pushq	%rdi
1402701c0:     	pushq	%rbx
1402701c1:     	subq	$0x68, %rsp
1402701c5:     	leaq	0x80(%rdx), %rbp
1402701cc:     	movaps	%xmm9, 0x20(%rsp)
1402701d2:     	movaps	%xmm8, 0x30(%rsp)
1402701d8:     	movaps	%xmm7, 0x40(%rsp)
1402701dd:     	movaps	%xmm6, 0x50(%rsp)
1402701e2:     	movq	0x2d8(%rbp), %rcx
1402701e9:     	movl	0x2d0(%rbp), %edx
1402701ef:     	callq	0x140043000 <.text+0x42000>
1402701f4:     	movb	$0x0, 0x2e5(%rbp)
1402701fb:     	movaps	0x50(%rsp), %xmm6
140270200:     	movaps	0x40(%rsp), %xmm7
140270205:     	movaps	0x30(%rsp), %xmm8
14027020b:     	movaps	0x20(%rsp), %xmm9
140270211:     	addq	$0x68, %rsp
140270215:     	popq	%rbx
140270216:     	popq	%rdi
140270217:     	popq	%rsi
140270218:     	popq	%r12
14027021a:     	popq	%r13
14027021c:     	popq	%r14
14027021e:     	popq	%r15
140270220:     	popq	%rbp
140270221:     	retq
140270222:     	nopw	%cs:(%rax,%rax)
140270230:     	movq	%rdx, 0x10(%rsp)
140270235:     	pushq	%rbp
140270236:     	pushq	%r15
140270238:     	pushq	%r14
14027023a:     	pushq	%r13
14027023c:     	pushq	%r12
14027023e:     	pushq	%rsi
14027023f:     	pushq	%rdi
140270240:     	pushq	%rbx
140270241:     	subq	$0x68, %rsp
140270245:     	leaq	0x80(%rdx), %rbp
14027024c:     	movaps	%xmm9, 0x20(%rsp)
140270252:     	movaps	%xmm8, 0x30(%rsp)
140270258:     	movaps	%xmm7, 0x40(%rsp)
14027025d:     	movaps	%xmm6, 0x50(%rsp)
140270262:     	movb	$0x1, 0x2e5(%rbp)
140270269:     	leaq	0x220(%rbp), %rcx
140270270:     	callq	0x1405d2870 <.text+0x5d1870>
140270275:     	movb	$0x1, 0x2e5(%rbp)
14027027c:     	movaps	0x50(%rsp), %xmm6
140270281:     	movaps	0x40(%rsp), %xmm7
140270286:     	movaps	0x30(%rsp), %xmm8
14027028c:     	movaps	0x20(%rsp), %xmm9
140270292:     	addq	$0x68, %rsp
140270296:     	popq	%rbx
140270297:     	popq	%rdi
140270298:     	popq	%rsi
140270299:     	popq	%r12
14027029b:     	popq	%r13
14027029d:     	popq	%r14
14027029f:     	popq	%r15
1402702a1:     	popq	%rbp
1402702a2:     	retq
1402702a3:     	nopw	%cs:(%rax,%rax)
1402702b0:     	movq	%rdx, 0x10(%rsp)
1402702b5:     	pushq	%rbp
1402702b6:     	pushq	%r15
1402702b8:     	pushq	%r14
1402702ba:     	pushq	%r13
1402702bc:     	pushq	%r12
1402702be:     	pushq	%rsi
1402702bf:     	pushq	%rdi
1402702c0:     	pushq	%rbx
1402702c1:     	subq	$0x68, %rsp
1402702c5:     	leaq	0x80(%rdx), %rbp
1402702cc:     	movaps	%xmm9, 0x20(%rsp)
1402702d2:     	movaps	%xmm8, 0x30(%rsp)
1402702d8:     	movaps	%xmm7, 0x40(%rsp)
1402702dd:     	movaps	%xmm6, 0x50(%rsp)
1402702e2:     	movq	0x220(%rbp), %rdx
1402702e9:     	testq	%rdx, %rdx
1402702ec:     	je	0x140270304 <.text+0x26f304>
1402702ee:     	movq	0x228(%rbp), %rcx
1402702f5:     	shlq	$0x3, %rdx
1402702f9:     	movl	$0x8, %r8d
1402702ff:     	callq	0x140613c20 <.text+0x612c20>
140270304:     	movb	$0x1, 0x2e7(%rbp)
14027030b:     	movaps	0x50(%rsp), %xmm6
140270310:     	movaps	0x40(%rsp), %xmm7
140270315:     	movaps	0x30(%rsp), %xmm8
14027031b:     	movaps	0x20(%rsp), %xmm9
