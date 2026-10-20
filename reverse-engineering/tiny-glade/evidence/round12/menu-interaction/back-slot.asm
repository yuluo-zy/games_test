
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fb8600:     	pushq	%r15
140fb8602:     	pushq	%r14
140fb8604:     	pushq	%r13
140fb8606:     	pushq	%r12
140fb8608:     	pushq	%rsi
140fb8609:     	pushq	%rdi
140fb860a:     	pushq	%rbp
140fb860b:     	pushq	%rbx
140fb860c:     	subq	$0xe8, %rsp
140fb8613:     	movq	%r9, %rsi
140fb8616:     	movq	%rcx, %rdi
140fb8619:     	movq	%rdx, 0xc0(%rsp)
140fb8621:     	movq	%r8, 0xc8(%rsp)
140fb8629:     	movq	0x18(%rcx), %r13
140fb862d:     	leaq	0xc0(%rsp), %rbx
140fb8635:     	movq	%rbx, %rcx
140fb8638:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fb863d:     	movq	%rax, 0x40(%rsp)
140fb8642:     	movabsq	$0x41223169ff28813b, %r15 # imm = 0x41223169FF28813B
140fb864c:     	movq	%r15, 0x48(%rsp)
140fb8651:     	movabsq	$-0x58648d975d569727, %r12 # imm = 0xA79B7268A2A968D9
140fb865b:     	movq	%r12, 0x50(%rsp)
140fb8660:     	leaq	0x40(%rsp), %rcx
140fb8665:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fb866a:     	movq	%r13, 0x40(%rsp)
140fb866f:     	movq	%rax, 0x48(%rsp)
140fb8674:     	leaq	0x40(%rsp), %rcx
140fb8679:     	callq	0x1405bd910 <.text+0x5bc910>
140fb867e:     	movq	%rax, 0x40(%rsp)
140fb8683:     	movq	0x10(%rdi), %rbp
140fb8687:     	cmpq	$0x0, 0x1b8(%rbp)
140fb868f:     	je	0x140fb874e <.text+0xfb774e>
140fb8695:     	movq	%rax, %r14
140fb8698:     	leaq	0x1c0(%rbp), %rcx
140fb869f:     	leaq	0x40(%rsp), %rdx
140fb86a4:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb86a9:     	movq	0x1a0(%rbp), %rcx
140fb86b0:     	movq	0x1a8(%rbp), %rdx
140fb86b7:     	movq	%rdx, %r8
140fb86ba:     	andq	%rax, %r8
140fb86bd:     	shrq	$0x39, %rax
140fb86c1:     	movd	%eax, %xmm0
140fb86c5:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb86c9:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb86ce:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb86d3:     	leaq	-0xc0(%rcx), %rax
140fb86da:     	xorl	%r9d, %r9d
140fb86dd:     	pcmpeqd	%xmm1, %xmm1
140fb86e1:     	movdqu	(%rcx,%r8), %xmm2
140fb86e7:     	movdqa	%xmm2, %xmm3
140fb86eb:     	pcmpeqb	%xmm0, %xmm3
140fb86ef:     	pmovmskb	%xmm3, %r10d
140fb86f4:     	testl	%r10d, %r10d
140fb86f7:     	je	0x140fb8730 <.text+0xfb7730>
140fb86f9:     	tzcntl	%r10d, %r11d
140fb86fe:     	addq	%r8, %r11
140fb8701:     	andq	%rdx, %r11
140fb8704:     	negq	%r11
140fb8707:     	leaq	(%r11,%r11,2), %r11
140fb870b:     	shlq	$0x6, %r11
140fb870f:     	cmpq	%r14, (%rax,%r11)
140fb8713:     	je	0x140fb89b2 <.text+0xfb79b2>
140fb8719:     	leal	-0x1(%r10), %r11d
140fb871d:     	andw	%r10w, %r11w
140fb8721:     	movl	%r11d, %r10d
140fb8724:     	jne	0x140fb86f9 <.text+0xfb76f9>
140fb8726:     	nopw	%cs:(%rax,%rax)
140fb8730:     	pcmpeqb	%xmm1, %xmm2
140fb8734:     	pmovmskb	%xmm2, %r10d
140fb8739:     	testl	%r10d, %r10d
140fb873c:     	jne	0x140fb874e <.text+0xfb774e>
140fb873e:     	addq	%r9, %r8
140fb8741:     	addq	$0x10, %r8
140fb8745:     	addq	$0x10, %r9
140fb8749:     	andq	%rdx, %r8
140fb874c:     	jmp	0x140fb86e1 <.text+0xfb76e1>
140fb874e:     	leaq	0x10(%rdi), %r14
140fb8752:     	movq	0x2673c4f(%rip), %rax   # 0x14362c3a8
140fb8759:     	movq	(%rax), %rax
140fb875c:     	testq	%rax, %rax
140fb875f:     	je	0x140fb8842 <.text+0xfb7842>
140fb8765:     	movq	%rdi, 0x20(%rsp)
140fb876a:     	leaq	-0xa84ed1(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
140fb8771:     	movq	%rax, 0x28(%rsp)
140fb8776:     	movq	%rbx, 0x30(%rsp)
140fb877b:     	movq	%rax, 0x38(%rsp)
140fb8780:     	leaq	0x1bfb709(%rip), %rcx   # 0x142bb3e90
140fb8787:     	callq	0x14067dd80 <.text+0x67cd80>
140fb878c:     	movdqu	(%rax), %xmm0
140fb8790:     	movl	0x10(%rax), %eax
140fb8793:     	movq	$0x1, 0x70(%rsp)
140fb879c:     	leaq	0x1bfb66d(%rip), %rcx   # 0x142bb3e10
140fb87a3:     	movq	%rcx, 0x78(%rsp)
140fb87a8:     	movq	$0x14, 0x80(%rsp)
140fb87b4:     	leaq	0x1bfb685(%rip), %rdx   # 0x142bb3e40
140fb87bb:     	movq	%rdx, 0x90(%rsp)
140fb87c3:     	movq	$0x2, 0x98(%rsp)
140fb87cf:     	leaq	0x20(%rsp), %rdx
140fb87d4:     	movq	%rdx, 0xa0(%rsp)
140fb87dc:     	movq	$0x2, 0xa8(%rsp)
140fb87e8:     	movq	$0x0, 0xb0(%rsp)
140fb87f4:     	movq	$0x0, 0x40(%rsp)
140fb87fd:     	movq	%rcx, 0x48(%rsp)
140fb8802:     	movq	$0x14, 0x50(%rsp)
140fb880b:     	movq	$0x0, 0x58(%rsp)
140fb8814:     	movdqu	%xmm0, 0x60(%rsp)
140fb881a:     	movl	$0x1, 0x88(%rsp)
140fb8825:     	movl	%eax, 0x8c(%rsp)
140fb882c:     	leaq	0xd0(%rsp), %rcx
140fb8834:     	leaq	0x40(%rsp), %rdx
140fb8839:     	callq	0x140d4a330 <.text+0xd49330>
140fb883e:     	movq	0x18(%rdi), %r13
140fb8842:     	movups	0xc0(%rsp), %xmm0
140fb884a:     	movups	%xmm0, 0x20(%rsp)
140fb884f:     	leaq	0x20(%rsp), %rcx
140fb8854:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fb8859:     	movq	%rax, 0x40(%rsp)
140fb885e:     	movq	%r15, 0x48(%rsp)
140fb8863:     	movq	%r12, 0x50(%rsp)
140fb8868:     	leaq	0x40(%rsp), %rcx
140fb886d:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fb8872:     	movq	%r13, 0x40(%rsp)
140fb8877:     	movq	%rax, 0x48(%rsp)
140fb887c:     	leaq	0x40(%rsp), %rcx
140fb8881:     	callq	0x1405bd910 <.text+0x5bc910>
140fb8886:     	leaq	0xd0(%rsp), %rdi
140fb888e:     	movq	%rdi, %rcx
140fb8891:     	movq	%r14, %rdx
140fb8894:     	movq	%rax, %r8
140fb8897:     	callq	0x1406861b0 <.text+0x6851b0>
140fb889c:     	leaq	0x1bfbaa5(%rip), %r8    # 0x142bb4348
140fb88a3:     	leaq	0x20(%rsp), %rbx
140fb88a8:     	movl	$0x9, %r9d
140fb88ae:     	movq	%rbx, %rcx
140fb88b1:     	movq	%rdi, %rdx
140fb88b4:     	callq	0x1406811c0 <.text+0x6801c0>
140fb88b9:     	movq	%rbx, %rcx
140fb88bc:     	movb	$0x1, %dl
140fb88be:     	callq	0x140f9de80 <.text+0xf9ce80>
140fb88c3:     	movb	$0x12, 0x44(%rax)
140fb88c7:     	movq	0x30(%rsp), %rax
140fb88cc:     	movq	%rax, 0x50(%rsp)
140fb88d1:     	movdqu	0x20(%rsp), %xmm0
140fb88d7:     	movdqa	%xmm0, 0x40(%rsp)
140fb88dd:     	movq	0x40(%rsp), %rdi
140fb88e2:     	cmpq	$0x0, 0x1b8(%rdi)
140fb88ea:     	je	0x140fb8be3 <.text+0xfb7be3>
140fb88f0:     	leaq	0x48(%rsp), %rdx
140fb88f5:     	leaq	0x1c0(%rdi), %rcx
140fb88fc:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb8901:     	movq	0x1a0(%rdi), %rcx
140fb8908:     	movq	0x1a8(%rdi), %rdx
140fb890f:     	movq	%rdx, %r8
140fb8912:     	andq	%rax, %r8
140fb8915:     	shrq	$0x39, %rax
140fb8919:     	movd	%eax, %xmm0
140fb891d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb8921:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb8926:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb892b:     	leaq	-0xc0(%rcx), %rax
140fb8932:     	xorl	%r9d, %r9d
140fb8935:     	movq	0x48(%rsp), %r10
140fb893a:     	pcmpeqd	%xmm1, %xmm1
140fb893e:     	movdqu	(%rcx,%r8), %xmm2
140fb8944:     	movdqa	%xmm2, %xmm3
140fb8948:     	pcmpeqb	%xmm0, %xmm3
140fb894c:     	pmovmskb	%xmm3, %r11d
140fb8951:     	testl	%r11d, %r11d
140fb8954:     	je	0x140fb8990 <.text+0xfb7990>
140fb8956:     	tzcntl	%r11d, %edi
140fb895b:     	addq	%r8, %rdi
140fb895e:     	andq	%rdx, %rdi
140fb8961:     	negq	%rdi
140fb8964:     	leaq	(%rdi,%rdi,2), %rdi
140fb8968:     	shlq	$0x6, %rdi
140fb896c:     	cmpq	(%rax,%rdi), %r10
140fb8970:     	je	0x140fb8ada <.text+0xfb7ada>
140fb8976:     	leal	-0x1(%r11), %edi
140fb897a:     	andw	%r11w, %di
140fb897e:     	movl	%edi, %r11d
140fb8981:     	jne	0x140fb8956 <.text+0xfb7956>
140fb8983:     	nopw	%cs:(%rax,%rax)
140fb8990:     	pcmpeqb	%xmm1, %xmm2
140fb8994:     	pmovmskb	%xmm2, %r11d
140fb8999:     	testl	%r11d, %r11d
140fb899c:     	jne	0x140fb8be3 <.text+0xfb7be3>
140fb89a2:     	addq	%r9, %r8
140fb89a5:     	addq	$0x10, %r8
140fb89a9:     	addq	$0x10, %r9
140fb89ad:     	andq	%rdx, %r8
140fb89b0:     	jmp	0x140fb893e <.text+0xfb793e>
140fb89b2:     	movq	%rbp, 0xd0(%rsp)
140fb89ba:     	movq	%r14, 0xd8(%rsp)
140fb89c2:     	movq	$0x0, 0xe0(%rsp)
140fb89ce:     	leaq	0x1bfb973(%rip), %r8    # 0x142bb4348
140fb89d5:     	leaq	0x20(%rsp), %rdi
140fb89da:     	leaq	0xd0(%rsp), %rdx
140fb89e2:     	movl	$0x9, %r9d
140fb89e8:     	movq	%rdi, %rcx
140fb89eb:     	callq	0x1406811c0 <.text+0x6801c0>
140fb89f0:     	movq	%rdi, %rcx
140fb89f3:     	movb	$0x1, %dl
140fb89f5:     	callq	0x140f9de80 <.text+0xf9ce80>
140fb89fa:     	movb	$0x12, 0x44(%rax)
140fb89fe:     	movq	0x30(%rsp), %rax
140fb8a03:     	movq	%rax, 0x50(%rsp)
140fb8a08:     	movups	0x20(%rsp), %xmm0
140fb8a0d:     	movaps	%xmm0, 0x40(%rsp)
140fb8a12:     	movq	0x40(%rsp), %rdi
140fb8a17:     	cmpq	$0x0, 0x1b8(%rdi)
140fb8a1f:     	je	0x140fb8be3 <.text+0xfb7be3>
140fb8a25:     	leaq	0x48(%rsp), %rdx
140fb8a2a:     	leaq	0x1c0(%rdi), %rcx
140fb8a31:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb8a36:     	movq	0x1a0(%rdi), %rcx
140fb8a3d:     	movq	0x1a8(%rdi), %rdx
140fb8a44:     	movq	%rdx, %r8
140fb8a47:     	andq	%rax, %r8
140fb8a4a:     	shrq	$0x39, %rax
140fb8a4e:     	movd	%eax, %xmm0
140fb8a52:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb8a56:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb8a5b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb8a60:     	leaq	-0xc0(%rcx), %rax
140fb8a67:     	xorl	%r9d, %r9d
140fb8a6a:     	movq	0x48(%rsp), %r10
140fb8a6f:     	pcmpeqd	%xmm1, %xmm1
140fb8a73:     	movdqu	(%rcx,%r8), %xmm2
140fb8a79:     	movdqa	%xmm2, %xmm3
140fb8a7d:     	pcmpeqb	%xmm0, %xmm3
140fb8a81:     	pmovmskb	%xmm3, %r11d
140fb8a86:     	testl	%r11d, %r11d
140fb8a89:     	je	0x140fb8ab8 <.text+0xfb7ab8>
140fb8a8b:     	tzcntl	%r11d, %edi
140fb8a90:     	addq	%r8, %rdi
140fb8a93:     	andq	%rdx, %rdi
140fb8a96:     	negq	%rdi
140fb8a99:     	leaq	(%rdi,%rdi,2), %rdi
140fb8a9d:     	shlq	$0x6, %rdi
140fb8aa1:     	cmpq	(%rax,%rdi), %r10
140fb8aa5:     	je	0x140fb8bef <.text+0xfb7bef>
140fb8aab:     	leal	-0x1(%r11), %edi
140fb8aaf:     	andw	%r11w, %di
140fb8ab3:     	movl	%edi, %r11d
140fb8ab6:     	jne	0x140fb8a8b <.text+0xfb7a8b>
140fb8ab8:     	pcmpeqb	%xmm1, %xmm2
140fb8abc:     	pmovmskb	%xmm2, %r11d
140fb8ac1:     	testl	%r11d, %r11d
140fb8ac4:     	jne	0x140fb8be3 <.text+0xfb7be3>
140fb8aca:     	addq	%r9, %r8
140fb8acd:     	addq	$0x10, %r8
140fb8ad1:     	addq	$0x10, %r9
140fb8ad5:     	andq	%rdx, %r8
140fb8ad8:     	jmp	0x140fb8a73 <.text+0xfb7a73>
140fb8ada:     	movw	$0x900, -0x4(%rcx,%rdi) # imm = 0x900
140fb8ae1:     	movq	0x50(%rsp), %rax
140fb8ae6:     	movq	%rax, 0x30(%rsp)
140fb8aeb:     	movaps	0x40(%rsp), %xmm0
140fb8af0:     	movaps	%xmm0, 0x20(%rsp)
140fb8af5:     	leaq	0x20(%rsp), %rcx
140fb8afa:     	movb	$0x1, %dl
140fb8afc:     	callq	0x140f9de80 <.text+0xf9ce80>
140fb8b01:     	movb	$0x12, 0x44(%rax)
140fb8b05:     	movdqa	0x20(%rsp), %xmm0
140fb8b0b:     	movdqa	%xmm0, 0x40(%rsp)
140fb8b11:     	movq	0x30(%rsp), %rax
140fb8b16:     	movq	%rax, 0x50(%rsp)
140fb8b1b:     	movq	0x40(%rsp), %rdi
140fb8b20:     	cmpq	$0x0, 0x1b8(%rdi)
140fb8b28:     	je	0x140fb8d29 <.text+0xfb7d29>
140fb8b2e:     	leaq	0x48(%rsp), %rdx
140fb8b33:     	leaq	0x1c0(%rdi), %rcx
140fb8b3a:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb8b3f:     	movq	0x1a0(%rdi), %rcx
140fb8b46:     	movq	0x1a8(%rdi), %rdx
140fb8b4d:     	movq	%rdx, %r8
140fb8b50:     	andq	%rax, %r8
140fb8b53:     	shrq	$0x39, %rax
140fb8b57:     	movd	%eax, %xmm0
140fb8b5b:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb8b5f:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb8b64:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb8b69:     	leaq	-0xc0(%rcx), %rax
140fb8b70:     	xorl	%r9d, %r9d
140fb8b73:     	movq	0x48(%rsp), %r10
140fb8b78:     	pcmpeqd	%xmm1, %xmm1
140fb8b7c:     	movdqu	(%rcx,%r8), %xmm2
140fb8b82:     	movdqa	%xmm2, %xmm3
140fb8b86:     	pcmpeqb	%xmm0, %xmm3
140fb8b8a:     	pmovmskb	%xmm3, %r11d
140fb8b8f:     	testl	%r11d, %r11d
140fb8b92:     	je	0x140fb8bc1 <.text+0xfb7bc1>
140fb8b94:     	tzcntl	%r11d, %edi
140fb8b99:     	addq	%r8, %rdi
140fb8b9c:     	andq	%rdx, %rdi
140fb8b9f:     	negq	%rdi
140fb8ba2:     	leaq	(%rdi,%rdi,2), %rdi
140fb8ba6:     	shlq	$0x6, %rdi
140fb8baa:     	cmpq	(%rax,%rdi), %r10
140fb8bae:     	je	0x140fb8cee <.text+0xfb7cee>
140fb8bb4:     	leal	-0x1(%r11), %edi
140fb8bb8:     	andw	%r11w, %di
140fb8bbc:     	movl	%edi, %r11d
140fb8bbf:     	jne	0x140fb8b94 <.text+0xfb7b94>
140fb8bc1:     	pcmpeqb	%xmm1, %xmm2
140fb8bc5:     	pmovmskb	%xmm2, %r11d
140fb8bca:     	testl	%r11d, %r11d
140fb8bcd:     	jne	0x140fb8d29 <.text+0xfb7d29>
140fb8bd3:     	addq	%r9, %r8
140fb8bd6:     	addq	$0x10, %r8
140fb8bda:     	addq	$0x10, %r9
140fb8bde:     	andq	%rdx, %r8
140fb8be1:     	jmp	0x140fb8b7c <.text+0xfb7b7c>
140fb8be3:     	leaq	0x1a5f406(%rip), %rcx   # 0x142a17ff0
140fb8bea:     	callq	0x1428d9310 <.text+0x28d8310>
140fb8bef:     	movw	$0x900, -0x4(%rcx,%rdi) # imm = 0x900
140fb8bf6:     	movq	0x50(%rsp), %rax
140fb8bfb:     	movq	%rax, 0x30(%rsp)
140fb8c00:     	movaps	0x40(%rsp), %xmm0
140fb8c05:     	movaps	%xmm0, 0x20(%rsp)
140fb8c0a:     	leaq	0x20(%rsp), %rcx
140fb8c0f:     	movb	$0x1, %dl
140fb8c11:     	callq	0x140f9de80 <.text+0xf9ce80>
140fb8c16:     	movb	$0x12, 0x44(%rax)
140fb8c1a:     	movaps	0x20(%rsp), %xmm0
140fb8c1f:     	movaps	%xmm0, 0x40(%rsp)
140fb8c24:     	movq	0x30(%rsp), %rax
140fb8c29:     	movq	%rax, 0x50(%rsp)
140fb8c2e:     	movq	0x40(%rsp), %rdi
140fb8c33:     	cmpq	$0x0, 0x1b8(%rdi)
140fb8c3b:     	je	0x140fb8d29 <.text+0xfb7d29>
140fb8c41:     	leaq	0x48(%rsp), %rdx
140fb8c46:     	leaq	0x1c0(%rdi), %rcx
140fb8c4d:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb8c52:     	movq	0x1a0(%rdi), %rcx
140fb8c59:     	movq	0x1a8(%rdi), %rdx
140fb8c60:     	movq	%rdx, %r8
140fb8c63:     	andq	%rax, %r8
140fb8c66:     	shrq	$0x39, %rax
140fb8c6a:     	movd	%eax, %xmm0
140fb8c6e:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb8c72:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb8c77:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb8c7c:     	leaq	-0xc0(%rcx), %rax
140fb8c83:     	xorl	%r9d, %r9d
140fb8c86:     	movq	0x48(%rsp), %r10
140fb8c8b:     	pcmpeqd	%xmm1, %xmm1
140fb8c8f:     	movdqu	(%rcx,%r8), %xmm2
140fb8c95:     	movdqa	%xmm2, %xmm3
140fb8c99:     	pcmpeqb	%xmm0, %xmm3
140fb8c9d:     	pmovmskb	%xmm3, %r11d
140fb8ca2:     	testl	%r11d, %r11d
140fb8ca5:     	je	0x140fb8cd0 <.text+0xfb7cd0>
140fb8ca7:     	tzcntl	%r11d, %edi
140fb8cac:     	addq	%r8, %rdi
140fb8caf:     	andq	%rdx, %rdi
140fb8cb2:     	negq	%rdi
140fb8cb5:     	leaq	(%rdi,%rdi,2), %rdi
140fb8cb9:     	shlq	$0x6, %rdi
140fb8cbd:     	cmpq	(%rax,%rdi), %r10
140fb8cc1:     	je	0x140fb8cee <.text+0xfb7cee>
140fb8cc3:     	leal	-0x1(%r11), %edi
140fb8cc7:     	andw	%r11w, %di
140fb8ccb:     	movl	%edi, %r11d
140fb8cce:     	jne	0x140fb8ca7 <.text+0xfb7ca7>
140fb8cd0:     	pcmpeqb	%xmm1, %xmm2
140fb8cd4:     	pmovmskb	%xmm2, %r11d
140fb8cd9:     	testl	%r11d, %r11d
140fb8cdc:     	jne	0x140fb8d29 <.text+0xfb7d29>
140fb8cde:     	addq	%r9, %r8
140fb8ce1:     	addq	$0x10, %r8
140fb8ce5:     	addq	$0x10, %r9
140fb8ce9:     	andq	%rdx, %r8
140fb8cec:     	jmp	0x140fb8c8f <.text+0xfb7c8f>
140fb8cee:     	testb	$0x30, -0x2(%rcx,%rdi)
140fb8cf3:     	je	0x140fb8d14 <.text+0xfb7d14>
140fb8cf5:     	movl	0x1c(%rsi), %eax
140fb8cf8:     	movq	0x10(%rsi), %rcx
140fb8cfc:     	movl	%eax, (%rcx)
140fb8cfe:     	movq	(%rsi), %rcx
140fb8d01:     	movq	$0x3, 0x40(%rsp)
140fb8d0a:     	leaq	0x40(%rsp), %rdx
140fb8d0f:     	callq	0x14093fad0 <.text+0x93ead0>
140fb8d14:     	nop
140fb8d15:     	addq	$0xe8, %rsp
140fb8d1c:     	popq	%rbx
140fb8d1d:     	popq	%rbp
140fb8d1e:     	popq	%rdi
140fb8d1f:     	popq	%rsi
140fb8d20:     	popq	%r12
140fb8d22:     	popq	%r13
140fb8d24:     	popq	%r14
140fb8d26:     	popq	%r15
140fb8d28:     	retq
140fb8d29:     	leaq	0x1a5e4d8(%rip), %rcx   # 0x142a17208
140fb8d30:     	leaq	0x1a5f2a1(%rip), %r8    # 0x142a17fd8
140fb8d37:     	movl	$0x16, %edx
140fb8d3c:     	callq	0x1428d9330 <.text+0x28d8330>
140fb8d41:     	int3
