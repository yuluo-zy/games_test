
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fbc5b0:     	pushq	%rbp
140fbc5b1:     	pushq	%r15
140fbc5b3:     	pushq	%r14
140fbc5b5:     	pushq	%r13
140fbc5b7:     	pushq	%r12
140fbc5b9:     	pushq	%rsi
140fbc5ba:     	pushq	%rdi
140fbc5bb:     	pushq	%rbx
140fbc5bc:     	subq	$0x118, %rsp            # imm = 0x118
140fbc5c3:     	leaq	0x80(%rsp), %rbp
140fbc5cb:     	movq	$-0x2, 0x90(%rbp)
140fbc5d6:     	movq	%rcx, %rsi
140fbc5d9:     	movq	%rdx, -0x60(%rbp)
140fbc5dd:     	movq	%r8, -0x58(%rbp)
140fbc5e1:     	movq	0x18(%rcx), %r12
140fbc5e5:     	leaq	-0x60(%rbp), %rbx
140fbc5e9:     	movq	%rbx, %rcx
140fbc5ec:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbc5f1:     	movq	%rax, -0x28(%rbp)
140fbc5f5:     	movabsq	$0x41223169ff28813b, %r14 # imm = 0x41223169FF28813B
140fbc5ff:     	movq	%r14, -0x20(%rbp)
140fbc603:     	movabsq	$-0x58648d975d569727, %r15 # imm = 0xA79B7268A2A968D9
140fbc60d:     	movq	%r15, -0x18(%rbp)
140fbc611:     	leaq	-0x28(%rbp), %rcx
140fbc615:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbc61a:     	movq	%r12, -0x28(%rbp)
140fbc61e:     	movq	%rax, -0x20(%rbp)
140fbc622:     	leaq	-0x28(%rbp), %rcx
140fbc626:     	callq	0x1405bd910 <.text+0x5bc910>
140fbc62b:     	movq	%rax, -0x28(%rbp)
140fbc62f:     	movq	0x10(%rsi), %r13
140fbc633:     	cmpq	$0x0, 0x1b8(%r13)
140fbc63b:     	je	0x140fbc6fe <.text+0xfbb6fe>
140fbc641:     	movq	%rax, %rdi
140fbc644:     	leaq	0x1c0(%r13), %rcx
140fbc64b:     	leaq	-0x28(%rbp), %rdx
140fbc64f:     	callq	0x1405b0d70 <.text+0x5afd70>
140fbc654:     	movq	0x1a0(%r13), %rcx
140fbc65b:     	movq	0x1a8(%r13), %rdx
140fbc662:     	movq	%rdx, %r8
140fbc665:     	andq	%rax, %r8
140fbc668:     	shrq	$0x39, %rax
140fbc66c:     	movd	%eax, %xmm0
140fbc670:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fbc674:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fbc679:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fbc67e:     	leaq	-0xc0(%rcx), %rax
140fbc685:     	xorl	%r9d, %r9d
140fbc688:     	pcmpeqd	%xmm1, %xmm1
140fbc68c:     	movdqu	(%rcx,%r8), %xmm2
140fbc692:     	movdqa	%xmm2, %xmm3
140fbc696:     	pcmpeqb	%xmm0, %xmm3
140fbc69a:     	pmovmskb	%xmm3, %r10d
140fbc69f:     	testl	%r10d, %r10d
140fbc6a2:     	je	0x140fbc6e0 <.text+0xfbb6e0>
140fbc6a4:     	tzcntl	%r10d, %r11d
140fbc6a9:     	addq	%r8, %r11
140fbc6ac:     	andq	%rdx, %r11
140fbc6af:     	negq	%r11
140fbc6b2:     	leaq	(%r11,%r11,2), %r11
140fbc6b6:     	shlq	$0x6, %r11
140fbc6ba:     	cmpq	%rdi, (%rax,%r11)
140fbc6be:     	je	0x140fbc903 <.text+0xfbb903>
140fbc6c4:     	leal	-0x1(%r10), %r11d
140fbc6c8:     	andw	%r10w, %r11w
140fbc6cc:     	movl	%r11d, %r10d
140fbc6cf:     	jne	0x140fbc6a4 <.text+0xfbb6a4>
140fbc6d1:     	nopw	%cs:(%rax,%rax)
140fbc6e0:     	pcmpeqb	%xmm1, %xmm2
140fbc6e4:     	pmovmskb	%xmm2, %r10d
140fbc6e9:     	testl	%r10d, %r10d
140fbc6ec:     	jne	0x140fbc6fe <.text+0xfbb6fe>
140fbc6ee:     	addq	%r9, %r8
140fbc6f1:     	addq	$0x10, %r8
140fbc6f5:     	addq	$0x10, %r9
140fbc6f9:     	andq	%rdx, %r8
140fbc6fc:     	jmp	0x140fbc68c <.text+0xfbb68c>
140fbc6fe:     	leaq	0x10(%rsi), %rdi
140fbc702:     	movq	0x266fc9f(%rip), %rax   # 0x14362c3a8
140fbc709:     	movq	(%rax), %rax
140fbc70c:     	testq	%rax, %rax
140fbc70f:     	je	0x140fbc7c1 <.text+0xfbb7c1>
140fbc715:     	movq	%rsi, 0x58(%rbp)
140fbc719:     	leaq	-0xa88e80(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
140fbc720:     	movq	%rax, 0x60(%rbp)
140fbc724:     	movq	%rbx, 0x68(%rbp)
140fbc728:     	movq	%rax, 0x70(%rbp)
140fbc72c:     	leaq	0x1bf775d(%rip), %rcx   # 0x142bb3e90
140fbc733:     	callq	0x14067dd80 <.text+0x67cd80>
140fbc738:     	movdqu	(%rax), %xmm0
140fbc73c:     	movl	0x10(%rax), %eax
140fbc73f:     	movq	$0x1, 0x8(%rbp)
140fbc747:     	leaq	0x1bf76c2(%rip), %rcx   # 0x142bb3e10
140fbc74e:     	movq	%rcx, 0x10(%rbp)
140fbc752:     	movq	$0x14, 0x18(%rbp)
140fbc75a:     	leaq	0x1bf76df(%rip), %rdx   # 0x142bb3e40
140fbc761:     	movq	%rdx, 0x28(%rbp)
140fbc765:     	movq	$0x2, 0x30(%rbp)
140fbc76d:     	leaq	0x58(%rbp), %rdx
140fbc771:     	movq	%rdx, 0x38(%rbp)
140fbc775:     	movq	$0x2, 0x40(%rbp)
140fbc77d:     	movq	$0x0, 0x48(%rbp)
140fbc785:     	movq	$0x0, -0x28(%rbp)
140fbc78d:     	movq	%rcx, -0x20(%rbp)
140fbc791:     	movq	$0x14, -0x18(%rbp)
140fbc799:     	movq	$0x0, -0x10(%rbp)
140fbc7a1:     	movdqu	%xmm0, -0x8(%rbp)
140fbc7a6:     	movl	$0x1, 0x20(%rbp)
140fbc7ad:     	movl	%eax, 0x24(%rbp)
140fbc7b0:     	leaq	-0x50(%rbp), %rcx
140fbc7b4:     	leaq	-0x28(%rbp), %rdx
140fbc7b8:     	callq	0x140d4a330 <.text+0xd49330>
140fbc7bd:     	movq	0x18(%rsi), %r12
140fbc7c1:     	movdqu	-0x60(%rbp), %xmm0
140fbc7c6:     	movdqu	%xmm0, 0x58(%rbp)
140fbc7cb:     	leaq	0x58(%rbp), %rcx
140fbc7cf:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbc7d4:     	movq	%rax, -0x28(%rbp)
140fbc7d8:     	movq	%r14, -0x20(%rbp)
140fbc7dc:     	movq	%r15, -0x18(%rbp)
140fbc7e0:     	leaq	-0x28(%rbp), %rcx
140fbc7e4:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbc7e9:     	movq	%r12, -0x28(%rbp)
140fbc7ed:     	movq	%rax, -0x20(%rbp)
140fbc7f1:     	leaq	-0x28(%rbp), %rcx
140fbc7f5:     	callq	0x1405bd910 <.text+0x5bc910>
140fbc7fa:     	leaq	-0x50(%rbp), %rcx
140fbc7fe:     	movq	%rdi, %rdx
140fbc801:     	movq	%rax, %r8
140fbc804:     	callq	0x1406861b0 <.text+0x6851b0>
140fbc809:     	leaq	0x1bf7b41(%rip), %rcx   # 0x142bb4351
140fbc810:     	movl	$0x14, %edx
140fbc815:     	callq	0x140b9d480 <.text+0xb9c480>
140fbc81a:     	movq	%rax, 0x78(%rbp)
140fbc81e:     	movq	%rdx, 0x80(%rbp)
140fbc825:     	leaq	0x78(%rbp), %rax
140fbc829:     	movq	%rax, -0x38(%rbp)
140fbc82d:     	leaq	-0xeb29a4(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140fbc834:     	movq	%rax, -0x30(%rbp)
140fbc838:     	leaq	0x1bf7b31(%rip), %rax   # 0x142bb4370
140fbc83f:     	movq	%rax, -0x28(%rbp)
140fbc843:     	movq	$0x1, -0x20(%rbp)
140fbc84b:     	leaq	-0x38(%rbp), %rax
140fbc84f:     	movq	%rax, -0x18(%rbp)
140fbc853:     	movq	$0x1, -0x10(%rbp)
140fbc85b:     	movq	$0x0, -0x8(%rbp)
140fbc863:     	leaq	0x58(%rbp), %rcx
140fbc867:     	leaq	-0x28(%rbp), %rdx
140fbc86b:     	callq	0x140d79f90 <.text+0xd78f90>
140fbc870:     	movq	0x60(%rbp), %rax
140fbc874:     	movq	0x68(%rbp), %rsi
140fbc878:     	movq	%rax, 0x88(%rbp)
140fbc87f:     	movq	%rax, 0x78(%rbp)
140fbc883:     	movq	%rsi, 0x80(%rbp)
140fbc88a:     	leaq	0x78(%rbp), %rcx
140fbc88e:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbc893:     	movq	%rax, -0x28(%rbp)
140fbc897:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fbc8a1:     	movq	%rax, -0x20(%rbp)
140fbc8a5:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fbc8af:     	movq	%rax, -0x18(%rbp)
140fbc8b3:     	leaq	-0x28(%rbp), %rcx
140fbc8b7:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbc8bc:     	movq	-0x48(%rbp), %rcx
140fbc8c0:     	movq	%rcx, -0x28(%rbp)
140fbc8c4:     	movq	%rax, -0x20(%rbp)
140fbc8c8:     	leaq	-0x28(%rbp), %rcx
140fbc8cc:     	callq	0x1405bd910 <.text+0x5bc910>
140fbc8d1:     	leaq	-0x28(%rbp), %rcx
140fbc8d5:     	leaq	-0x50(%rbp), %rdx
140fbc8d9:     	movq	%rax, %r8
140fbc8dc:     	callq	0x1406861b0 <.text+0x6851b0>
140fbc8e1:     	leaq	-0x28(%rbp), %rcx
140fbc8e5:     	xorl	%edx, %edx
140fbc8e7:     	callq	0x140f9d930 <.text+0xf9c930>
140fbc8ec:     	movq	%rax, %rcx
140fbc8ef:     	movq	0x88(%rbp), %rdx
140fbc8f6:     	movq	%rsi, %r8
140fbc8f9:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fbc8fe:     	jmp	0x140fbca04 <.text+0xfbba04>
140fbc903:     	movq	%r13, -0x50(%rbp)
140fbc907:     	movq	%rdi, -0x48(%rbp)
140fbc90b:     	movq	$0x0, -0x40(%rbp)
140fbc913:     	leaq	0x1bf7a37(%rip), %rcx   # 0x142bb4351
140fbc91a:     	movl	$0x14, %edx
140fbc91f:     	callq	0x140b9d480 <.text+0xb9c480>
140fbc924:     	movq	%rax, 0x78(%rbp)
140fbc928:     	movq	%rdx, 0x80(%rbp)
140fbc92f:     	leaq	0x78(%rbp), %rax
140fbc933:     	movq	%rax, -0x38(%rbp)
140fbc937:     	leaq	-0xeb2aae(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140fbc93e:     	movq	%rax, -0x30(%rbp)
140fbc942:     	leaq	0x1bf7a27(%rip), %rax   # 0x142bb4370
140fbc949:     	movq	%rax, -0x28(%rbp)
140fbc94d:     	movq	$0x1, -0x20(%rbp)
140fbc955:     	leaq	-0x38(%rbp), %rax
140fbc959:     	movq	%rax, -0x18(%rbp)
140fbc95d:     	movq	$0x1, -0x10(%rbp)
140fbc965:     	movq	$0x0, -0x8(%rbp)
140fbc96d:     	leaq	0x58(%rbp), %rcx
140fbc971:     	leaq	-0x28(%rbp), %rdx
140fbc975:     	callq	0x140d79f90 <.text+0xd78f90>
140fbc97a:     	movq	0x60(%rbp), %rax
140fbc97e:     	movq	0x68(%rbp), %rsi
140fbc982:     	movq	%rax, 0x88(%rbp)
140fbc989:     	movq	%rax, 0x78(%rbp)
140fbc98d:     	movq	%rsi, 0x80(%rbp)
140fbc994:     	leaq	0x78(%rbp), %rcx
140fbc998:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbc99d:     	movq	%rax, -0x28(%rbp)
140fbc9a1:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fbc9ab:     	movq	%rax, -0x20(%rbp)
140fbc9af:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fbc9b9:     	movq	%rax, -0x18(%rbp)
140fbc9bd:     	leaq	-0x28(%rbp), %rcx
140fbc9c1:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbc9c6:     	movq	%rdi, -0x28(%rbp)
140fbc9ca:     	movq	%rax, -0x20(%rbp)
140fbc9ce:     	leaq	-0x28(%rbp), %rcx
140fbc9d2:     	callq	0x1405bd910 <.text+0x5bc910>
140fbc9d7:     	leaq	-0x28(%rbp), %rcx
140fbc9db:     	leaq	-0x50(%rbp), %rdx
140fbc9df:     	movq	%rax, %r8
140fbc9e2:     	callq	0x1406861b0 <.text+0x6851b0>
140fbc9e7:     	leaq	-0x28(%rbp), %rcx
140fbc9eb:     	xorl	%edx, %edx
140fbc9ed:     	callq	0x140f9d930 <.text+0xf9c930>
140fbc9f2:     	movq	%rax, %rcx
140fbc9f5:     	movq	0x88(%rbp), %rdx
140fbc9fc:     	movq	%rsi, %r8
140fbc9ff:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fbca04:     	movq	0x58(%rbp), %rdx
140fbca08:     	testq	%rdx, %rdx
140fbca0b:     	je	0x140fbca1f <.text+0xfbba1f>
140fbca0d:     	movl	$0x1, %r8d
140fbca13:     	movq	0x88(%rbp), %rcx
140fbca1a:     	callq	0x140613c20 <.text+0x612c20>
140fbca1f:     	nop
140fbca20:     	addq	$0x118, %rsp            # imm = 0x118
140fbca27:     	popq	%rbx
140fbca28:     	popq	%rdi
140fbca29:     	popq	%rsi
140fbca2a:     	popq	%r12
140fbca2c:     	popq	%r13
140fbca2e:     	popq	%r14
140fbca30:     	popq	%r15
140fbca32:     	popq	%rbp
140fbca33:     	retq
140fbca34:     	nopw	%cs:(%rax,%rax)
140fbca40:     	movq	%rdx, 0x10(%rsp)
140fbca45:     	pushq	%rbp
140fbca46:     	pushq	%r15
140fbca48:     	pushq	%r14
140fbca4a:     	pushq	%r13
140fbca4c:     	pushq	%r12
140fbca4e:     	pushq	%rsi
140fbca4f:     	pushq	%rdi
140fbca50:     	pushq	%rbx
140fbca51:     	subq	$0x28, %rsp
140fbca55:     	leaq	0x80(%rdx), %rbp
140fbca5c:     	movq	0x58(%rbp), %rdx
140fbca60:     	testq	%rdx, %rdx
140fbca63:     	je	0x140fbca77 <.text+0xfbba77>
140fbca65:     	movl	$0x1, %r8d
140fbca6b:     	movq	0x88(%rbp), %rcx
140fbca72:     	callq	0x140613c20 <.text+0x612c20>
140fbca77:     	nop
140fbca78:     	addq	$0x28, %rsp
140fbca7c:     	popq	%rbx
140fbca7d:     	popq	%rdi
140fbca7e:     	popq	%rsi
140fbca7f:     	popq	%r12
140fbca81:     	popq	%r13
140fbca83:     	popq	%r14
140fbca85:     	popq	%r15
140fbca87:     	popq	%rbp
140fbca88:     	retq
140fbca89:     	nopl	(%rax)
140fbca90:     	movq	%rdx, 0x10(%rsp)
140fbca95:     	pushq	%rbp
140fbca96:     	pushq	%r15
140fbca98:     	pushq	%r14
140fbca9a:     	pushq	%r13
140fbca9c:     	pushq	%r12
140fbca9e:     	pushq	%rsi
140fbca9f:     	pushq	%rdi
140fbcaa0:     	pushq	%rbx
140fbcaa1:     	subq	$0x28, %rsp
140fbcaa5:     	leaq	0x80(%rdx), %rbp
140fbcaac:     	movq	0x58(%rbp), %rdx
140fbcab0:     	testq	%rdx, %rdx
140fbcab3:     	je	0x140fbcac7 <.text+0xfbbac7>
140fbcab5:     	movl	$0x1, %r8d
140fbcabb:     	movq	0x88(%rbp), %rcx
140fbcac2:     	callq	0x140613c20 <.text+0x612c20>
140fbcac7:     	nop
140fbcac8:     	addq	$0x28, %rsp
140fbcacc:     	popq	%rbx
140fbcacd:     	popq	%rdi
140fbcace:     	popq	%rsi
140fbcacf:     	popq	%r12
140fbcad1:     	popq	%r13
140fbcad3:     	popq	%r14
140fbcad5:     	popq	%r15
140fbcad7:     	popq	%rbp
140fbcad8:     	retq
