
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405005e0:     	pushq	%r15
1405005e2:     	pushq	%r14
1405005e4:     	pushq	%r13
1405005e6:     	pushq	%r12
1405005e8:     	pushq	%rsi
1405005e9:     	pushq	%rdi
1405005ea:     	pushq	%rbp
1405005eb:     	pushq	%rbx
1405005ec:     	subq	$0xd8, %rsp
1405005f3:     	movq	%r9, %rsi
1405005f6:     	movq	%rcx, %rbx
1405005f9:     	movq	%rdx, 0x28(%rsp)
1405005fe:     	movq	%r8, 0x30(%rsp)
140500603:     	leaq	0x10(%rcx), %rdi
140500607:     	movq	%rdi, %rcx
14050060a:     	callq	0x1406868b0 <.text+0x6858b0>
14050060f:     	movq	%rax, %r12
140500612:     	leaq	0x28(%rsp), %r15
140500617:     	movq	%r15, %rcx
14050061a:     	callq	0x140683c10 <.text+0x682c10>
14050061f:     	movq	%rax, 0x38(%rsp)
140500624:     	leaq	0x38(%rsp), %r14
140500629:     	movq	%r14, %rcx
14050062c:     	callq	0x1400885d0 <.text+0x875d0>
140500631:     	movq	%rax, 0x58(%rsp)
140500636:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140500640:     	movq	%r13, 0x60(%rsp)
140500645:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
14050064f:     	movq	%rbp, 0x68(%rsp)
140500654:     	leaq	0x58(%rsp), %rcx
140500659:     	callq	0x1405bdf20 <.text+0x5bcf20>
14050065e:     	movq	%r12, 0x58(%rsp)
140500663:     	movq	%rax, 0x60(%rsp)
140500668:     	leaq	0x58(%rsp), %rcx
14050066d:     	callq	0x1405bd910 <.text+0x5bc910>
140500672:     	movq	%rax, %r12
140500675:     	movq	%rdi, %rcx
140500678:     	movq	%rax, %rdx
14050067b:     	callq	0x140686450 <.text+0x685450>
140500680:     	testb	%al, %al
140500682:     	je	0x14050069c <.text+0x4ff69c>
140500684:     	leaq	0x58(%rsp), %rbx
140500689:     	movq	%rbx, %rcx
14050068c:     	movq	%rdi, %rdx
14050068f:     	movq	%r12, %r8
140500692:     	callq	0x140686530 <.text+0x685530>
140500697:     	jmp	0x1405007dc <.text+0x4ff7dc>
14050069c:     	movq	0x312bd05(%rip), %rax   # 0x14362c3a8
1405006a3:     	movq	(%rax), %rax
1405006a6:     	testq	%rax, %rax
1405006a9:     	je	0x140500784 <.text+0x4ff784>
1405006af:     	movq	%rbx, 0x38(%rsp)
1405006b4:     	leaq	0x331e5(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
1405006bb:     	movq	%rax, 0x40(%rsp)
1405006c0:     	movq	%r15, 0x48(%rsp)
1405006c5:     	movq	%rax, 0x50(%rsp)
1405006ca:     	leaq	0x24ed687(%rip), %rcx   # 0x1429edd58
1405006d1:     	callq	0x14067dd80 <.text+0x67cd80>
1405006d6:     	movups	(%rax), %xmm0
1405006d9:     	movl	0x10(%rax), %eax
1405006dc:     	movq	$0x1, 0x88(%rsp)
1405006e8:     	leaq	0x24ed639(%rip), %rcx   # 0x1429edd28
1405006ef:     	movq	%rcx, 0x90(%rsp)
1405006f7:     	movq	$0x14, 0x98(%rsp)
140500703:     	leaq	0x24ed58e(%rip), %rdx   # 0x1429edc98
14050070a:     	movq	%rdx, 0xa8(%rsp)
140500712:     	movq	$0x2, 0xb0(%rsp)
14050071e:     	movq	%r14, 0xb8(%rsp)
140500726:     	movq	$0x2, 0xc0(%rsp)
140500732:     	movq	$0x0, 0xc8(%rsp)
14050073e:     	movq	$0x0, 0x58(%rsp)
140500747:     	movq	%rcx, 0x60(%rsp)
14050074c:     	movq	$0x14, 0x68(%rsp)
140500755:     	movq	$0x0, 0x70(%rsp)
14050075e:     	movups	%xmm0, 0x78(%rsp)
140500763:     	movl	$0x1, 0xa0(%rsp)
14050076e:     	movl	%eax, 0xa4(%rsp)
140500775:     	leaq	0x27(%rsp), %rcx
14050077a:     	leaq	0x58(%rsp), %rdx
14050077f:     	callq	0x140d4a330 <.text+0xd49330>
140500784:     	movups	0x28(%rsp), %xmm0
140500789:     	movq	0x18(%rbx), %rbx
14050078d:     	movups	%xmm0, 0x38(%rsp)
140500792:     	leaq	0x38(%rsp), %rcx
140500797:     	callq	0x140683c10 <.text+0x682c10>
14050079c:     	movq	%rax, 0x58(%rsp)
1405007a1:     	movq	%r13, 0x60(%rsp)
1405007a6:     	movq	%rbp, 0x68(%rsp)
1405007ab:     	leaq	0x58(%rsp), %rcx
1405007b0:     	callq	0x1405bdf20 <.text+0x5bcf20>
1405007b5:     	movq	%rbx, 0x58(%rsp)
1405007ba:     	movq	%rax, 0x60(%rsp)
1405007bf:     	leaq	0x58(%rsp), %rcx
1405007c4:     	callq	0x1405bd910 <.text+0x5bc910>
1405007c9:     	leaq	0x58(%rsp), %rbx
1405007ce:     	movq	%rbx, %rcx
1405007d1:     	movq	%rdi, %rdx
1405007d4:     	movq	%rax, %r8
1405007d7:     	callq	0x1406861b0 <.text+0x6851b0>
1405007dc:     	movq	%rsi, %rcx
1405007df:     	movq	%rbx, %rdx
1405007e2:     	callq	0x1405049f0 <.text+0x5039f0>
1405007e7:     	nop
1405007e8:     	addq	$0xd8, %rsp
1405007ef:     	popq	%rbx
1405007f0:     	popq	%rbp
1405007f1:     	popq	%rdi
1405007f2:     	popq	%rsi
1405007f3:     	popq	%r12
1405007f5:     	popq	%r13
1405007f7:     	popq	%r14
1405007f9:     	popq	%r15
1405007fb:     	retq
