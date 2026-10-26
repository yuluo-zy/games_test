
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14228b3d0:     	pushq	%r15
14228b3d2:     	pushq	%r14
14228b3d4:     	pushq	%r13
14228b3d6:     	pushq	%r12
14228b3d8:     	pushq	%rsi
14228b3d9:     	pushq	%rdi
14228b3da:     	pushq	%rbp
14228b3db:     	pushq	%rbx
14228b3dc:     	subq	$0x108, %rsp            # imm = 0x108
14228b3e3:     	movaps	%xmm6, 0xf0(%rsp)
14228b3eb:     	movq	%r9, %rdi
14228b3ee:     	movq	%rcx, %rbx
14228b3f1:     	movq	%rdx, 0xc8(%rsp)
14228b3f9:     	movq	%r8, 0xd0(%rsp)
14228b401:     	leaq	0x10(%rcx), %rsi
14228b405:     	movq	%rsi, %rcx
14228b408:     	callq	0x1406868b0 <.text+0x6858b0>
14228b40d:     	movq	%rax, %r12
14228b410:     	leaq	0xc8(%rsp), %r15
14228b418:     	movq	%r15, %rcx
14228b41b:     	callq	0x140683c10 <.text+0x682c10>
14228b420:     	movq	%rax, 0x20(%rsp)
14228b425:     	leaq	0x20(%rsp), %r14
14228b42a:     	movq	%r14, %rcx
14228b42d:     	callq	0x1400885d0 <.text+0x875d0>
14228b432:     	movq	%rax, 0x40(%rsp)
14228b437:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
14228b441:     	movq	%r13, 0x48(%rsp)
14228b446:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
14228b450:     	movq	%rbp, 0x50(%rsp)
14228b455:     	leaq	0x40(%rsp), %rcx
14228b45a:     	callq	0x14228d080 <.text+0x228c080>
14228b45f:     	movq	%r12, 0x40(%rsp)
14228b464:     	movq	%rax, 0x48(%rsp)
14228b469:     	leaq	0x40(%rsp), %rcx
14228b46e:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b473:     	movq	%rax, %r12
14228b476:     	movq	%rsi, %rcx
14228b479:     	movq	%rax, %rdx
14228b47c:     	callq	0x140686450 <.text+0x685450>
14228b481:     	testb	%al, %al
14228b483:     	je	0x14228b4dc <.text+0x228a4dc>
14228b485:     	movq	(%rdi), %rbx
14228b488:     	movq	0x8(%rdi), %r14
14228b48c:     	movq	0x10(%rdi), %r15
14228b490:     	leaq	0xd8(%rsp), %rdi
14228b498:     	movq	%rdi, %rcx
14228b49b:     	movq	%rsi, %rdx
14228b49e:     	movq	%r12, %r8
14228b4a1:     	callq	0x140686530 <.text+0x685530>
14228b4a6:     	leaq	0x40(%rsp), %rsi
14228b4ab:     	movq	%rsi, %rcx
14228b4ae:     	movq	%rdi, %rdx
14228b4b1:     	movq	%rbx, %r8
14228b4b4:     	movq	%r14, %r9
14228b4b7:     	callq	0x14068bf00 <.text+0x68af00>
14228b4bc:     	leaq	0x20(%rsp), %rdi
14228b4c1:     	movq	%rdi, %rcx
14228b4c4:     	movq	%rsi, %rdx
14228b4c7:     	movl	$0x505050ff, %r8d       # imm = 0x505050FF
14228b4cd:     	callq	0x14067e4a0 <.text+0x67d4a0>
14228b4d2:     	movss	(%r15), %xmm6
14228b4d7:     	jmp	0x14228b65c <.text+0x228a65c>
14228b4dc:     	movq	0x13a0ec5(%rip), %rax   # 0x14362c3a8
14228b4e3:     	movq	(%rax), %rax
14228b4e6:     	testq	%rax, %rax
14228b4e9:     	je	0x14228b5c1 <.text+0x228a5c1>
14228b4ef:     	movq	%rbx, 0x20(%rsp)
14228b4f4:     	leaq	-0x1d57c5b(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
14228b4fb:     	movq	%rax, 0x28(%rsp)
14228b500:     	movq	%r15, 0x30(%rsp)
14228b505:     	movq	%rax, 0x38(%rsp)
14228b50a:     	leaq	0xdaf10f(%rip), %rcx    # 0x14303a620
14228b511:     	callq	0x14067dd80 <.text+0x67cd80>
14228b516:     	movups	(%rax), %xmm0
14228b519:     	movl	0x10(%rax), %eax
14228b51c:     	movq	$0x1, 0x70(%rsp)
14228b525:     	leaq	0xdaf10c(%rip), %rcx    # 0x14303a638
14228b52c:     	movq	%rcx, 0x78(%rsp)
14228b531:     	movq	$0x14, 0x80(%rsp)
14228b53d:     	leaq	0xdaf064(%rip), %rdx    # 0x14303a5a8
14228b544:     	movq	%rdx, 0x90(%rsp)
14228b54c:     	movq	$0x2, 0x98(%rsp)
14228b558:     	movq	%r14, 0xa0(%rsp)
14228b560:     	movq	$0x2, 0xa8(%rsp)
14228b56c:     	movq	$0x0, 0xb0(%rsp)
14228b578:     	movq	$0x0, 0x40(%rsp)
14228b581:     	movq	%rcx, 0x48(%rsp)
14228b586:     	movq	$0x14, 0x50(%rsp)
14228b58f:     	movq	$0x0, 0x58(%rsp)
14228b598:     	movups	%xmm0, 0x60(%rsp)
14228b59d:     	movl	$0x1, 0x88(%rsp)
14228b5a8:     	movl	%eax, 0x8c(%rsp)
14228b5af:     	leaq	0xd8(%rsp), %rcx
14228b5b7:     	leaq	0x40(%rsp), %rdx
14228b5bc:     	callq	0x140d4a330 <.text+0xd49330>
14228b5c1:     	movq	(%rdi), %r14
14228b5c4:     	movq	0x8(%rdi), %r15
14228b5c8:     	movq	0x10(%rdi), %r12
14228b5cc:     	movups	0xc8(%rsp), %xmm0
14228b5d4:     	movq	0x18(%rbx), %rdi
14228b5d8:     	movups	%xmm0, 0x20(%rsp)
14228b5dd:     	leaq	0x20(%rsp), %rcx
14228b5e2:     	callq	0x140683c10 <.text+0x682c10>
14228b5e7:     	movq	%rax, 0x40(%rsp)
14228b5ec:     	movq	%r13, 0x48(%rsp)
14228b5f1:     	movq	%rbp, 0x50(%rsp)
14228b5f6:     	leaq	0x40(%rsp), %rcx
14228b5fb:     	callq	0x14228d080 <.text+0x228c080>
14228b600:     	movq	%rdi, 0x40(%rsp)
14228b605:     	movq	%rax, 0x48(%rsp)
14228b60a:     	leaq	0x40(%rsp), %rcx
14228b60f:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b614:     	leaq	0xd8(%rsp), %rdi
14228b61c:     	movq	%rdi, %rcx
14228b61f:     	movq	%rsi, %rdx
14228b622:     	movq	%rax, %r8
14228b625:     	callq	0x1406861b0 <.text+0x6851b0>
14228b62a:     	leaq	0x40(%rsp), %rsi
14228b62f:     	movq	%rsi, %rcx
14228b632:     	movq	%rdi, %rdx
14228b635:     	movq	%r14, %r8
14228b638:     	movq	%r15, %r9
14228b63b:     	callq	0x14068bf00 <.text+0x68af00>
14228b640:     	leaq	0x20(%rsp), %rdi
14228b645:     	movq	%rdi, %rcx
14228b648:     	movq	%rsi, %rdx
14228b64b:     	movl	$0x505050ff, %r8d       # imm = 0x505050FF
14228b651:     	callq	0x14067e4a0 <.text+0x67d4a0>
14228b656:     	movss	(%r12), %xmm6
14228b65c:     	movq	%rdi, %rcx
14228b65f:     	callq	0x140681d30 <.text+0x680d30>
14228b664:     	movq	%rax, %rcx
14228b667:     	movaps	%xmm6, %xmm1
14228b66a:     	callq	0x1425f0ac0 <.text+0x25efac0>
14228b66f:     	movq	0x30(%rsp), %rax
14228b674:     	movq	%rax, 0x50(%rsp)
14228b679:     	movups	0x20(%rsp), %xmm0
14228b67e:     	movaps	%xmm0, 0x40(%rsp)
14228b683:     	leaq	0x20(%rsp), %rsi
14228b688:     	leaq	0x40(%rsp), %rdx
14228b68d:     	movq	%rsi, %rcx
14228b690:     	movb	$0x1, %r8b
14228b693:     	callq	0x14067e420 <.text+0x67d420>
14228b698:     	leaq	0x40(%rsp), %rcx
14228b69d:     	movq	%rsi, %rdx
14228b6a0:     	movw	$0x2bc, %r8w            # imm = 0x2BC
14228b6a5:     	callq	0x14067e460 <.text+0x67d460>
14228b6aa:     	movaps	0xf0(%rsp), %xmm6
14228b6b2:     	addq	$0x108, %rsp            # imm = 0x108
14228b6b9:     	popq	%rbx
14228b6ba:     	popq	%rbp
14228b6bb:     	popq	%rdi
14228b6bc:     	popq	%rsi
14228b6bd:     	popq	%r12
14228b6bf:     	popq	%r13
14228b6c1:     	popq	%r14
14228b6c3:     	popq	%r15
14228b6c5:     	retq
14228b6c6:     	int3
14228b6c7:     	int3
14228b6c8:     	int3
14228b6c9:     	int3
14228b6ca:     	int3
14228b6cb:     	int3
14228b6cc:     	int3
14228b6cd:     	int3
14228b6ce:     	int3
14228b6cf:     	int3
