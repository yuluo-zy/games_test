
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146d030:     	pushq	%rbp
14146d031:     	pushq	%r15
14146d033:     	pushq	%r14
14146d035:     	pushq	%r13
14146d037:     	pushq	%r12
14146d039:     	pushq	%rsi
14146d03a:     	pushq	%rdi
14146d03b:     	pushq	%rbx
14146d03c:     	subq	$0xa58, %rsp            # imm = 0xA58
14146d043:     	leaq	0x80(%rsp), %rbp
14146d04b:     	movaps	%xmm10, 0x9c0(%rbp)
14146d053:     	movaps	%xmm9, 0x9b0(%rbp)
14146d05b:     	movaps	%xmm8, 0x9a0(%rbp)
14146d063:     	movaps	%xmm7, 0x990(%rbp)
14146d06a:     	movaps	%xmm6, 0x980(%rbp)
14146d071:     	movq	$-0x2, 0x978(%rbp)
14146d07c:     	movq	%r9, 0x960(%rbp)
14146d083:     	movq	%r8, 0x8e0(%rbp)
14146d08a:     	movq	%rdx, 0x958(%rbp)
14146d091:     	movq	%rcx, 0x938(%rbp)
14146d098:     	movq	0xa80(%rbp), %rdi
14146d09f:     	movq	(%rdi), %rsi
14146d0a2:     	movq	0x8(%rdi), %r13
14146d0a6:     	movq	(%rsi), %rax
14146d0a9:     	xorl	%ecx, %ecx
14146d0ab:     	movq	%rax, %r10
14146d0ae:     	subq	0x18(%r13), %r10
14146d0b2:     	cmovbq	%rcx, %r10
14146d0b6:     	subq	0x38(%r13), %rax
14146d0ba:     	cmovbq	%rcx, %rax
14146d0be:     	movq	0x10(%r13), %r12
14146d0c2:     	movq	0x30(%r13), %rdx
14146d0c6:     	movq	%rdx, 0x910(%rbp)
14146d0cd:     	subq	%rax, %rdx
14146d0d0:     	cmovbq	%rcx, %rdx
14146d0d4:     	setbe	%r8b
14146d0d8:     	movq	%r12, 0x950(%rbp)
14146d0df:     	subq	%r10, %r12
14146d0e2:     	cmovbq	%rcx, %r12
14146d0e6:     	setbe	%r9b
14146d0ea:     	testb	%r8b, %r9b
14146d0ed:     	leaq	(%rdx,%r12), %r11
14146d0f1:     	cmoveq	%rcx, %r11
14146d0f5:     	movq	0x40(%r13), %rcx
14146d0f9:     	subq	%r11, %rcx
14146d0fc:     	movq	0xa78(%rbp), %rbx
14146d103:     	movsd	0x90(%rbx), %xmm6
14146d10b:     	movq	(%rbx), %r11
14146d10e:     	movq	%r11, 0x8a8(%rbp)
14146d115:     	movq	0x10(%rbx), %r11
14146d119:     	movq	%r11, 0x560(%rbp)
14146d120:     	movl	0x1c(%rbx), %r11d
14146d124:     	movl	%r11d, 0x8c0(%rbp)
14146d12b:     	movq	0x20(%rbx), %r11
14146d12f:     	movq	%r11, 0x558(%rbp)
14146d136:     	movq	0x30(%rbx), %r11
14146d13a:     	movq	%r11, 0x8d8(%rbp)
14146d141:     	movl	0x3c(%rbx), %r11d
14146d145:     	movl	%r11d, 0x90c(%rbp)
14146d14c:     	movq	0x40(%rbx), %r11
14146d150:     	movq	%r11, 0x918(%rbp)
14146d157:     	movq	0x50(%rbx), %r11
14146d15b:     	movq	%r11, 0x570(%rbp)
14146d162:     	movl	0x5c(%rbx), %r11d
14146d166:     	movl	%r11d, 0x8c4(%rbp)
14146d16d:     	movq	0x60(%rbx), %r11
14146d171:     	movq	%r11, 0x580(%rbp)
14146d178:     	movq	0x70(%rbx), %r11
14146d17c:     	movq	%r11, 0x8b0(%rbp)
14146d183:     	movl	0x7c(%rbx), %r11d
14146d187:     	movl	%r11d, 0x8b8(%rbp)
14146d18e:     	movq	0x80(%rbx), %r11
14146d195:     	movq	%r11, 0x870(%rbp)
14146d19c:     	movq	0x88(%rbx), %r11
14146d1a3:     	movq	%r11, 0x8f8(%rbp)
14146d1aa:     	movq	0x98(%rbx), %r11
14146d1b1:     	movq	%r11, 0x568(%rbp)
14146d1b8:     	movq	0xa8(%rbx), %r11
14146d1bf:     	movq	%r11, 0x528(%rbp)
14146d1c6:     	movl	0xb4(%rbx), %r11d
14146d1cd:     	movl	%r11d, 0x85c(%rbp)
14146d1d4:     	movq	0xb8(%rbx), %r11
14146d1db:     	movq	%r11, 0x878(%rbp)
14146d1e2:     	movq	0xc8(%rbx), %r11
14146d1e9:     	movq	%r11, 0x530(%rbp)
14146d1f0:     	movl	0xd4(%rbx), %r11d
14146d1f7:     	movl	%r11d, 0x860(%rbp)
14146d1fe:     	movq	0x28(%rdi), %r15
14146d202:     	movq	0x38(%rdi), %r14
14146d206:     	movl	0x44(%rdi), %ebx
14146d209:     	movq	0x48(%rdi), %r11
14146d20d:     	movq	%r11, 0x520(%rbp)
14146d214:     	movq	0x58(%rdi), %r11
14146d218:     	movq	%r11, 0x578(%rbp)
14146d21f:     	movl	0x64(%rdi), %r11d
14146d223:     	movl	%r11d, 0x8c8(%rbp)
14146d22a:     	movq	0x68(%rdi), %r11
14146d22e:     	movq	%r11, 0x928(%rbp)
14146d235:     	movq	0x78(%rdi), %r11
14146d239:     	movq	%r11, 0x900(%rbp)
14146d240:     	movl	0x84(%rdi), %r11d
14146d247:     	movl	%r11d, 0x948(%rbp)
14146d24e:     	movq	0x88(%rdi), %r11
14146d255:     	movq	%r11, 0x538(%rbp)
14146d25c:     	movq	0x98(%rdi), %r11
14146d263:     	movq	%r11, 0x540(%rbp)
14146d26a:     	movl	0xa4(%rdi), %r11d
14146d271:     	movl	%r11d, 0x864(%rbp)
14146d278:     	movq	0x8(%r13), %r11
14146d27c:     	movq	0x28(%r13), %rdi
14146d280:     	movq	%rcx, (%rsi)
14146d283:     	testb	%r8b, %r9b
14146d286:     	jne	0x14146d396 <.text+0x146c396>
14146d28c:     	xorl	%ecx, %ecx
14146d28e:     	cmpq	%r10, 0x950(%rbp)
14146d295:     	leaq	(%r10,%r10,4), %r8
14146d299:     	leaq	(%r11,%r8,8), %r8
14146d29d:     	movl	$0x8, %r9d
14146d2a3:     	cmovbq	%r9, %r8
14146d2a7:     	leaq	(%r12,%r12,4), %r10
14146d2ab:     	leaq	-0x28(%r8,%r10,8), %r8
14146d2b0:     	cmovaq	%r8, %rcx
14146d2b4:     	cmpq	%rax, 0x910(%rbp)
14146d2bb:     	leaq	(%rax,%rax,4), %rax
14146d2bf:     	leaq	(%rdi,%rax,8), %rax
14146d2c3:     	cmovbq	%r9, %rax
14146d2c7:     	leaq	(%rdx,%rdx,4), %rdx
14146d2cb:     	leaq	-0x28(%rax,%rdx,8), %rax
14146d2d0:     	cmovbeq	%rcx, %rax
14146d2d4:     	cmpl	$0xe, (%rax)
14146d2d7:     	jne	0x14146d396 <.text+0x146c396>
14146d2dd:     	movq	0x958(%rbp), %rax
14146d2e4:     	movq	(%rax), %rsi
14146d2e7:     	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
14146d2f1:     	cmpq	%rax, 0x18(%rsi)
14146d2f5:     	jne	0x14146d38c <.text+0x146c38c>
14146d2fb:     	leaq	0x20(%rsi), %rcx
14146d2ff:     	movq	%rcx, 0x518(%rbp)
14146d306:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146d30b:     	movq	%rdx, %r8
14146d30e:     	leaq	0x5b0(%rbp), %rcx
14146d315:     	movq	%rax, %rdx
14146d318:     	callq	0x140d6a710 <.text+0xd69710>
14146d31d:     	cmpl	$0x2, 0x5b0(%rbp)
14146d324:     	je	0x14146f8bd <.text+0x146e8bd>
14146d32a:     	movups	0x5b0(%rbp), %xmm0
14146d331:     	movups	0x5c0(%rbp), %xmm1
14146d338:     	movups	0x5d0(%rbp), %xmm2
14146d33f:     	movups	0x5e0(%rbp), %xmm3
14146d346:     	movaps	%xmm0, 0x360(%rbp)
14146d34d:     	movq	0x600(%rbp), %rax
14146d354:     	movq	%rax, 0x3b0(%rbp)
14146d35b:     	movups	0x5f0(%rbp), %xmm0
14146d362:     	movaps	%xmm0, 0x3a0(%rbp)
14146d369:     	movaps	%xmm3, 0x390(%rbp)
14146d370:     	movaps	%xmm2, 0x380(%rbp)
14146d377:     	movaps	%xmm1, 0x370(%rbp)
14146d37e:     	cmpq	$0x2, 0x360(%rbp)
14146d386:     	je	0x14146f8d6 <.text+0x146e8d6>
14146d38c:     	cmpb	$0x0, 0x48(%rsi)
14146d390:     	jne	0x14146f8a5 <.text+0x146e8a5>
14146d396:     	movq	0x938(%rbp), %rax
14146d39d:     	movq	(%rax), %rax
14146d3a0:     	cmpl	$0xe, (%rax)
14146d3a3:     	jne	0x14146f625 <.text+0x146e625>
14146d3a9:     	movq	0x958(%rbp), %rax
14146d3b0:     	movq	(%rax), %r8
14146d3b3:     	movzbl	0x48(%r8), %eax
14146d3b8:     	testl	%eax, %eax
14146d3ba:     	je	0x14146d5c4 <.text+0x146c5c4>
14146d3c0:     	cmpl	$0x2, %eax
14146d3c3:     	jne	0x14146d868 <.text+0x146c868>
14146d3c9:     	movq	0x958(%rbp), %rcx
14146d3d0:     	movl	0x1c(%rcx), %eax
14146d3d3:     	movq	0x10(%rcx), %rcx
14146d3d7:     	movl	%eax, (%rcx)
14146d3d9:     	movq	(%r8), %rcx
14146d3dc:     	movq	0x8(%r8), %rax
14146d3e0:     	movq	0x10(%r8), %rdx
14146d3e4:     	movq	$0x0, (%r8)
14146d3eb:     	movq	$0x8, 0x8(%r8)
14146d3f3:     	movq	$0x0, 0x10(%r8)
14146d3fb:     	movq	%rdx, %rbx
14146d3fe:     	shlq	$0x5, %rbx
14146d402:     	addq	%rax, %rbx
14146d405:     	movq	%rax, 0x360(%rbp)
14146d40c:     	movq	%rax, 0x368(%rbp)
14146d413:     	movq	%rcx, 0x370(%rbp)
14146d41a:     	movq	%rbx, 0x378(%rbp)
14146d421:     	testq	%rdx, %rdx
14146d424:     	je	0x14146d8fc <.text+0x146c8fc>
14146d42a:     	movq	0x8e0(%rbp), %rcx
14146d431:     	movl	0x1c(%rcx), %edx
14146d434:     	movl	%edx, 0x958(%rbp)
14146d43a:     	movq	(%rcx), %rdx
14146d43d:     	movq	%rdx, 0x8f8(%rbp)
14146d444:     	movq	0x10(%rcx), %rcx
14146d448:     	movq	%rcx, 0x938(%rbp)
14146d44f:     	movq	0x928(%rbp), %rcx
14146d456:     	addq	$0x20, %rcx
14146d45a:     	movq	%rcx, 0x918(%rbp)
14146d461:     	movabsq	$-0x7ffffffffffffffd, %r15 # imm = 0x8000000000000003
14146d46b:     	movl	$0x1, %r14d
14146d471:     	jmp	0x14146d4bb <.text+0x146c4bb>
14146d473:     	nopw	%cs:(%rax,%rax)
14146d480:     	movq	0x928(%rbp), %rdx
14146d487:     	movq	0x28(%rdx), %rax
14146d48b:     	leaq	(,%r13,2), %rcx
14146d493:     	addq	%r13, %rcx
14146d496:     	movq	%r12, (%rax,%rcx,8)
14146d49a:     	movlps	%xmm6, 0x8(%rax,%rcx,8)
14146d49f:     	movq	%rsi, 0x10(%rax,%rcx,8)
14146d4a4:     	incq	%r13
14146d4a7:     	movq	%r13, 0x30(%rdx)
14146d4ab:     	incq	0x40(%rdx)
14146d4af:     	movq	%rdi, %rax
14146d4b2:     	cmpq	%rbx, %rdi
14146d4b5:     	je	0x14146d8fc <.text+0x146c8fc>
14146d4bb:     	leaq	0x20(%rax), %rdi
14146d4bf:     	movq	%rdi, 0x368(%rbp)
14146d4c6:     	movq	(%rax), %rcx
14146d4c9:     	cmpq	%r15, %rcx
14146d4cc:     	je	0x14146d8fc <.text+0x146c8fc>
14146d4d2:     	movq	0x8(%rax), %r12
14146d4d6:     	movups	0x10(%rax), %xmm6
14146d4da:     	movq	%rcx, %rax
14146d4dd:     	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
14146d4e7:     	xorq	%rdx, %rax
14146d4ea:     	cmpq	$0x3, %rax
14146d4ee:     	cmovaeq	%r14, %rax
14146d4f2:     	testq	%rax, %rax
14146d4f5:     	je	0x14146d550 <.text+0x146c550>
14146d4f7:     	cmpq	$0x1, %rax
14146d4fb:     	jne	0x14146d590 <.text+0x146c590>
14146d501:     	movl	0x958(%rbp), %eax
14146d507:     	movq	0x938(%rbp), %rdx
14146d50e:     	movl	%eax, (%rdx)
14146d510:     	movq	%rcx, 0x5b0(%rbp)
14146d517:     	movq	%r12, 0x5b8(%rbp)
14146d51e:     	movups	%xmm6, 0x5c0(%rbp)
14146d525:     	movq	0x8f8(%rbp), %rcx
14146d52c:     	leaq	0x5b0(%rbp), %rdx
14146d533:     	leaq	0x17ed8fe(%rip), %r8    # 0x142c5ae38
14146d53a:     	callq	0x140b5a180 <.text+0xb59180>
14146d53f:     	jmp	0x14146d4af <.text+0x146c4af>
14146d544:     	nopw	%cs:(%rax,%rax)
14146d550:     	movq	0x900(%rbp), %rax
14146d557:     	movl	0x948(%rbp), %ecx
14146d55d:     	movl	%ecx, (%rax)
14146d55f:     	movq	0x928(%rbp), %rax
14146d566:     	movq	0x30(%rax), %r13
14146d56a:     	movq	0x40(%rax), %rsi
14146d56e:     	cmpq	0x20(%rax), %r13
14146d572:     	jne	0x14146d480 <.text+0x146c480>
14146d578:     	movq	0x918(%rbp), %rcx
14146d57f:     	leaq	0x17ccbc2(%rip), %rdx   # 0x142c3a148
14146d586:     	callq	0x140562db0 <.text+0x561db0>
14146d58b:     	jmp	0x14146d480 <.text+0x146c480>
14146d590:     	movq	%r12, 0x5b0(%rbp)
14146d597:     	xorl	%eax, %eax
14146d599:     	xchgb	%al, 0x10(%r12)
14146d59e:     	lock
14146d59f:     	decq	(%r12)
14146d5a3:     	jne	0x14146d5b1 <.text+0x146c5b1>
14146d5a5:     	leaq	0x5b0(%rbp), %rcx
14146d5ac:     	callq	0x1402e5d80 <.text+0x2e4d80>
14146d5b1:     	movq	0x368(%rbp), %rdi
14146d5b8:     	movq	0x378(%rbp), %rbx
14146d5bf:     	jmp	0x14146d4af <.text+0x146c4af>
14146d5c4:     	movq	0x50(%r8), %rax
14146d5c8:     	movq	%rax, 0x550(%rbp)
14146d5cf:     	movzbl	0x49(%r8), %eax
14146d5d4:     	movb	%al, 0x928(%rbp)
14146d5da:     	movzbl	0x4a(%r8), %ebx
14146d5df:     	movq	%r8, 0x900(%rbp)
14146d5e6:     	movzbl	0x4b(%r8), %r14d
14146d5eb:     	movq	0x21bedb6(%rip), %rax   # 0x14362c3a8
14146d5f2:     	movq	(%rax), %rax
14146d5f5:     	cmpq	$0x4, %rax
14146d5f9:     	movq	0x520(%rbp), %r13
14146d600:     	movq	0x960(%rbp), %rsi
14146d607:     	jb	0x14146d6e9 <.text+0x146c6e9>
14146d60d:     	movabsq	$-0x8000000000000000, %rdi # imm = 0x8000000000000000
14146d617:     	movq	%rdi, 0x930(%rbp)
14146d61e:     	movb	$0x1, 0x976(%rbp)
14146d625:     	leaq	0x17f6b04(%rip), %rcx   # 0x142c64130
14146d62c:     	callq	0x14067dd80 <.text+0x67cd80>
14146d631:     	movups	(%rax), %xmm0
14146d634:     	movl	0x10(%rax), %eax
14146d637:     	movq	$0x4, 0x5e0(%rbp)
14146d642:     	leaq	0x17f6aff(%rip), %rcx   # 0x142c64148
14146d649:     	movq	%rcx, 0x5e8(%rbp)
14146d650:     	movq	$0x26, 0x5f0(%rbp)
14146d65b:     	leaq	0x17f6abe(%rip), %rdx   # 0x142c64120
14146d662:     	movq	%rdx, 0x600(%rbp)
14146d669:     	movq	$0x1, 0x608(%rbp)
14146d674:     	movq	$0x8, 0x610(%rbp)
14146d67f:     	movq	$0x0, 0x5b0(%rbp)
14146d68a:     	xorps	%xmm1, %xmm1
14146d68d:     	movups	%xmm1, 0x618(%rbp)
14146d694:     	movq	%rcx, 0x5b8(%rbp)
14146d69b:     	movq	$0x26, 0x5c0(%rbp)
14146d6a6:     	movq	$0x0, 0x5c8(%rbp)
14146d6b1:     	movups	%xmm0, 0x5d0(%rbp)
14146d6b8:     	movl	$0x1, 0x5f8(%rbp)
14146d6c2:     	movl	%eax, 0x5fc(%rbp)
14146d6c8:     	movq	%rdi, 0x930(%rbp)
14146d6cf:     	movb	$0x1, 0x976(%rbp)
14146d6d6:     	leaq	0x972(%rbp), %rcx
14146d6dd:     	leaq	0x5b0(%rbp), %rdx
14146d6e4:     	callq	0x140d4a330 <.text+0xd49330>
14146d6e9:     	movq	(%rsi), %rdx
14146d6ec:     	addq	$0x60, %rdx
14146d6f0:     	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
14146d6fa:     	movq	%rcx, 0x8e8(%rbp)
14146d701:     	movq	%rcx, 0x930(%rbp)
14146d708:     	movb	$0x1, 0x976(%rbp)
14146d70f:     	leaq	0x360(%rbp), %rcx
14146d716:     	movq	%rdx, 0x8e0(%rbp)
14146d71d:     	callq	0x140b46000 <.text+0xb45000>
14146d722:     	leaq	0x360(%rbp), %rcx
14146d729:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146d72e:     	movq	%rdx, %r8
14146d731:     	movq	$0xb, 0x20(%rsp)
14146d73a:     	leaq	0x17f6a2d(%rip), %r9    # 0x142c6416e
14146d741:     	leaq	0x1b0(%rbp), %rcx
14146d748:     	movq	%rax, %rdx
14146d74b:     	callq	0x140d62810 <.text+0xd61810>
14146d750:     	movl	$0x0, 0x5b0(%rbp)
14146d75a:     	xorps	%xmm0, %xmm0
14146d75d:     	movups	%xmm0, 0x5b8(%rbp)
14146d764:     	movl	$0x7, 0x5c8(%rbp)
14146d76e:     	movq	$0x0, 0x5cc(%rbp)
14146d779:     	movw	$0x0, 0x5d4(%rbp)
14146d782:     	movb	$0x1, 0x5d1(%rbp)
14146d789:     	movb	$0x1, 0x5d4(%rbp)
14146d790:     	movb	$0x1, 0x5d3(%rbp)
14146d797:     	leaq	0x1b0(%rbp), %rcx
14146d79e:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146d7a3:     	movq	%rdx, %r8
14146d7a6:     	leaq	0x5b0(%rbp), %rcx
14146d7ad:     	movq	%rax, %rdx
14146d7b0:     	callq	0x140d59dc0 <.text+0xd58dc0>
14146d7b5:     	movq	%rax, %r12
14146d7b8:     	movq	%rdx, %r15
14146d7bb:     	movq	0x1b0(%rbp), %rdx
14146d7c2:     	testq	%rdx, %rdx
14146d7c5:     	je	0x14146d7d9 <.text+0x146c7d9>
14146d7c7:     	movq	0x1b8(%rbp), %rcx
14146d7ce:     	movl	$0x1, %r8d
14146d7d4:     	callq	0x140613c20 <.text+0x612c20>
14146d7d9:     	testq	%r12, %r12
14146d7dc:     	je	0x14146d9c8 <.text+0x146c9c8>
14146d7e2:     	movl	%r15d, %eax
14146d7e5:     	andl	$0x3, %eax
14146d7e8:     	cmpl	$0x1, %eax
14146d7eb:     	movq	0x900(%rbp), %r12
14146d7f2:     	jne	0x14146d9d8 <.text+0x146c9d8>
14146d7f8:     	movq	%r15, %rax
14146d7fb:     	decq	%rax
14146d7fe:     	movq	%rax, 0x910(%rbp)
14146d805:     	movq	-0x1(%r15), %rax
14146d809:     	movq	%rax, 0x938(%rbp)
14146d810:     	movq	0x7(%r15), %rax
14146d814:     	movq	%rax, 0x948(%rbp)
14146d81b:     	movq	(%rax), %rax
14146d81e:     	testq	%rax, %rax
14146d821:     	je	0x14146d82c <.text+0x146c82c>
14146d823:     	movq	0x938(%rbp), %rcx
14146d82a:     	callq	*%rax
14146d82c:     	movq	0x938(%rbp), %rcx
14146d833:     	movq	0x948(%rbp), %rax
14146d83a:     	movq	0x8(%rax), %rdx
14146d83e:     	testq	%rdx, %rdx
14146d841:     	je	0x14146d84c <.text+0x146c84c>
14146d843:     	movq	0x10(%rax), %r8
14146d847:     	callq	0x140613c20 <.text+0x612c20>
14146d84c:     	movl	$0x18, %edx
14146d851:     	movl	$0x8, %r8d
14146d857:     	movq	0x910(%rbp), %rcx
14146d85e:     	callq	0x140613c20 <.text+0x612c20>
14146d863:     	jmp	0x14146d9d8 <.text+0x146c9d8>
14146d868:     	movq	%r8, 0x900(%rbp)
14146d86f:     	movl	0x4c(%r8), %r12d
14146d873:     	testl	%r12d, %r12d
14146d876:     	js	0x14146d925 <.text+0x146c925>
14146d87c:     	leaq	0x20(%r15), %rax
14146d880:     	movq	%rax, 0x928(%rbp)
14146d887:     	xorl	%r13d, %r13d
14146d88a:     	jmp	0x14146d8b5 <.text+0x146c8b5>
14146d88c:     	nopl	(%rax)
14146d890:     	incl	%r13d
14146d893:     	movq	0x28(%r15), %rax
14146d897:     	movq	%rdi, %rcx
14146d89a:     	shlq	$0x4, %rcx
14146d89e:     	movq	%rsi, (%rax,%rcx)
14146d8a2:     	movl	$0x3010101, 0x8(%rax,%rcx) # imm = 0x3010101
14146d8aa:     	incq	%rdi
14146d8ad:     	movq	%rdi, 0x30(%r15)
14146d8b1:     	incq	0x40(%r15)
14146d8b5:     	cmpl	%r12d, %r13d
14146d8b8:     	jge	0x14146d9a8 <.text+0x146c9a8>
14146d8be:     	movl	%ebx, (%r14)
14146d8c1:     	movq	0x30(%r15), %rdi
14146d8c5:     	movq	0x40(%r15), %rsi
14146d8c9:     	cmpq	0x20(%r15), %rdi
14146d8cd:     	jne	0x14146d890 <.text+0x146c890>
14146d8cf:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146d8d9:     	movq	%rax, 0x930(%rbp)
14146d8e0:     	movb	$0x1, 0x976(%rbp)
14146d8e7:     	movq	0x928(%rbp), %rcx
14146d8ee:     	leaq	0x17cc853(%rip), %rdx   # 0x142c3a148
14146d8f5:     	callq	0x140562630 <.text+0x561630>
14146d8fa:     	jmp	0x14146d890 <.text+0x146c890>
14146d8fc:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146d906:     	movq	%rax, 0x930(%rbp)
14146d90d:     	movb	$0x1, 0x976(%rbp)
14146d914:     	leaq	0x360(%rbp), %rcx
14146d91b:     	callq	0x14087b2f0 <.text+0x87a2f0>
14146d920:     	jmp	0x14146f625 <.text+0x146e625>
14146d925:     	negl	%r12d
14146d928:     	leaq	0x20(%r15), %rax
14146d92c:     	movq	%rax, 0x928(%rbp)
14146d933:     	xorl	%r13d, %r13d
14146d936:     	jmp	0x14146d965 <.text+0x146c965>
14146d938:     	nopl	(%rax,%rax)
14146d940:     	incl	%r13d
14146d943:     	movq	0x28(%r15), %rax
14146d947:     	movq	%rdi, %rcx
14146d94a:     	shlq	$0x4, %rcx
14146d94e:     	movq	%rsi, (%rax,%rcx)
14146d952:     	movl	$0x3010001, 0x8(%rax,%rcx) # imm = 0x3010001
14146d95a:     	incq	%rdi
14146d95d:     	movq	%rdi, 0x30(%r15)
14146d961:     	incq	0x40(%r15)
14146d965:     	cmpl	%r12d, %r13d
14146d968:     	jge	0x14146d9a8 <.text+0x146c9a8>
14146d96a:     	movl	%ebx, (%r14)
14146d96d:     	movq	0x30(%r15), %rdi
14146d971:     	movq	0x40(%r15), %rsi
14146d975:     	cmpq	0x20(%r15), %rdi
14146d979:     	jne	0x14146d940 <.text+0x146c940>
14146d97b:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146d985:     	movq	%rax, 0x930(%rbp)
14146d98c:     	movb	$0x1, 0x976(%rbp)
14146d993:     	movq	0x928(%rbp), %rcx
14146d99a:     	leaq	0x17cc7a7(%rip), %rdx   # 0x142c3a148
14146d9a1:     	callq	0x140562630 <.text+0x561630>
14146d9a6:     	jmp	0x14146d940 <.text+0x146c940>
14146d9a8:     	movq	0x958(%rbp), %rcx
14146d9af:     	movl	0x1c(%rcx), %eax
14146d9b2:     	movq	0x10(%rcx), %rcx
14146d9b6:     	movl	%eax, (%rcx)
14146d9b8:     	movq	0x900(%rbp), %rax
14146d9bf:     	movb	$0x2, 0x48(%rax)
14146d9c3:     	jmp	0x14146f625 <.text+0x146e625>
14146d9c8:     	movq	%r15, %rcx
14146d9cb:     	callq	*0x1d24a1f(%rip)        # 0x1431923f0
14146d9d1:     	movq	0x900(%rbp), %r12
14146d9d8:     	movq	0x360(%rbp), %rdx
14146d9df:     	testq	%rdx, %rdx
14146d9e2:     	je	0x14146d9f6 <.text+0x146c9f6>
14146d9e4:     	movq	0x368(%rbp), %rcx
14146d9eb:     	movl	$0x1, %r8d
14146d9f1:     	callq	0x140613c20 <.text+0x612c20>
14146d9f6:     	testb	%bl, %bl
14146d9f8:     	je	0x14146db4d <.text+0x146cb4d>
14146d9fe:     	movq	0x21be9a3(%rip), %rax   # 0x14362c3a8
14146da05:     	movq	(%rax), %rax
14146da08:     	cmpq	$0x3, %rax
14146da0c:     	jb	0x14146daee <.text+0x146caee>
14146da12:     	movabsq	$-0x8000000000000000, %rsi # imm = 0x8000000000000000
14146da1c:     	movq	%rsi, 0x930(%rbp)
14146da23:     	movb	$0x1, 0x976(%rbp)
14146da2a:     	leaq	0x17f6787(%rip), %rcx   # 0x142c641b8
14146da31:     	callq	0x14067dd80 <.text+0x67cd80>
14146da36:     	movups	(%rax), %xmm0
14146da39:     	movl	0x10(%rax), %eax
14146da3c:     	movq	$0x3, 0x5e0(%rbp)
14146da47:     	leaq	0x17f66fa(%rip), %rcx   # 0x142c64148
14146da4e:     	movq	%rcx, 0x5e8(%rbp)
14146da55:     	movq	$0x26, 0x5f0(%rbp)
14146da60:     	leaq	0x17f6741(%rip), %rdx   # 0x142c641a8
14146da67:     	movq	%rdx, 0x600(%rbp)
14146da6e:     	movq	$0x1, 0x608(%rbp)
14146da79:     	movq	$0x8, 0x610(%rbp)
14146da84:     	movq	$0x0, 0x5b0(%rbp)
14146da8f:     	xorps	%xmm1, %xmm1
14146da92:     	movups	%xmm1, 0x618(%rbp)
14146da99:     	movq	%rcx, 0x5b8(%rbp)
14146daa0:     	movq	$0x26, 0x5c0(%rbp)
14146daab:     	movq	$0x0, 0x5c8(%rbp)
14146dab6:     	movups	%xmm0, 0x5d0(%rbp)
14146dabd:     	movl	$0x1, 0x5f8(%rbp)
14146dac7:     	movl	%eax, 0x5fc(%rbp)
14146dacd:     	movq	%rsi, 0x930(%rbp)
14146dad4:     	movb	$0x1, 0x976(%rbp)
14146dadb:     	leaq	0x972(%rbp), %rcx
14146dae2:     	leaq	0x5b0(%rbp), %rdx
14146dae9:     	callq	0x140d4a330 <.text+0xd49330>
14146daee:     	movq	0x578(%rbp), %rax
14146daf5:     	movl	0x8c8(%rbp), %ecx
14146dafb:     	movl	%ecx, (%rax)
14146dafd:     	movq	0x30(%r13), %rsi
14146db01:     	movq	0x40(%r13), %rdi
14146db05:     	cmpq	0x20(%r13), %rsi
14146db09:     	jne	0x14146db33 <.text+0x146cb33>
14146db0b:     	leaq	0x20(%r13), %rcx
14146db0f:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146db19:     	movq	%rax, 0x930(%rbp)
14146db20:     	movb	$0x1, 0x976(%rbp)
14146db27:     	leaq	0x17cc61a(%rip), %rdx   # 0x142c3a148
14146db2e:     	callq	0x140562870 <.text+0x561870>
14146db33:     	movq	0x28(%r13), %rax
14146db37:     	movq	%rdi, (%rax,%rsi,8)
14146db3b:     	incq	%rsi
14146db3e:     	movq	%rsi, 0x30(%r13)
14146db42:     	incq	0x40(%r13)
14146db46:     	movq	0x900(%rbp), %r12
14146db4d:     	movq	0x958(%rbp), %rcx
14146db54:     	movl	0x1c(%rcx), %eax
14146db57:     	movq	0x10(%rcx), %rcx
14146db5b:     	movl	%eax, 0x8cc(%rbp)
14146db61:     	movq	%rcx, 0x588(%rbp)
14146db68:     	movl	%eax, (%rcx)
14146db6a:     	movq	0x18(%r12), %rsi
14146db6f:     	movq	0x38(%r12), %rdi
14146db74:     	leaq	0x20(%r12), %rax
14146db79:     	movq	0x40(%r12), %rcx
14146db7e:     	movq	%rcx, 0x910(%rbp)
14146db85:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
14146db8f:     	movq	%rbx, 0x18(%r12)
14146db94:     	movq	%rsi, %rdx
14146db97:     	xorq	%rbx, %rdx
14146db9a:     	cmpq	$0x2, %rdx
14146db9e:     	movl	$0x2, %ecx
14146dba3:     	cmovbq	%rdx, %rcx
14146dba7:     	cmpq	$0x1, %rcx
14146dbab:     	je	0x14146e136 <.text+0x146d136>
14146dbb1:     	cmpq	$0x2, %rcx
14146dbb5:     	jne	0x14146f9e1 <.text+0x146e9e1>
14146dbbb:     	movq	0x10(%rax), %rcx
14146dbbf:     	movq	%rcx, 0x5a0(%rbp)
14146dbc6:     	movups	(%rax), %xmm0
14146dbc9:     	movaps	%xmm0, 0x590(%rbp)
14146dbd0:     	movq	%rsi, 0x830(%rbp)
14146dbd7:     	movaps	0x590(%rbp), %xmm0
14146dbde:     	movups	%xmm0, 0x838(%rbp)
14146dbe5:     	movq	0x5a0(%rbp), %rax
14146dbec:     	movq	%rax, 0x848(%rbp)
14146dbf3:     	testb	%r14b, %r14b
14146dbf6:     	je	0x14146dc02 <.text+0x146cc02>
14146dbf8:     	movl	$0x1b, 0xc0(%rdi)
14146dc02:     	movq	%rdi, 0x948(%rbp)
14146dc09:     	movq	%rbx, 0x8d0(%rbp)
14146dc10:     	movb	$0x1, 0x971(%rbp)
14146dc17:     	movb	$0x1, 0x970(%rbp)
14146dc1e:     	leaq	0x830(%rbp), %rcx
14146dc25:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146dc2a:     	movq	%rax, %r12
14146dc2d:     	movq	%rdx, %r14
14146dc30:     	movq	0x8b0(%rbp), %rax
14146dc37:     	movl	0x8b8(%rbp), %ecx
14146dc3d:     	movl	%ecx, (%rax)
14146dc3f:     	movq	0x560(%rbp), %rax
14146dc46:     	movl	0x8c0(%rbp), %ecx
14146dc4c:     	movl	%ecx, (%rax)
14146dc4e:     	movq	0x8d8(%rbp), %rax
14146dc55:     	movl	0x90c(%rbp), %ecx
14146dc5b:     	movl	%ecx, (%rax)
14146dc5d:     	movq	0x570(%rbp), %rax
14146dc64:     	movl	0x8c4(%rbp), %ecx
14146dc6a:     	movl	%ecx, (%rax)
14146dc6c:     	movq	0x528(%rbp), %rax
14146dc73:     	movl	0x85c(%rbp), %ecx
14146dc79:     	movl	%ecx, (%rax)
14146dc7b:     	movq	0x948(%rbp), %rcx
14146dc82:     	movl	0xc0(%rcx), %eax
14146dc88:     	cmpl	$0x1a, %eax
14146dc8b:     	movq	0x21be676(%rip), %rax   # 0x14362c308
14146dc92:     	setge	(%rax)
14146dc95:     	setg	0x21be375(%rip)         # 0x14362c011
14146dc9c:     	leaq	0xc4(%rcx), %rdx
14146dca3:     	movq	%rbx, 0x940(%rbp)
14146dcaa:     	movb	$0x1, 0x977(%rbp)
14146dcb1:     	movq	0x8a8(%rbp), %rcx
14146dcb8:     	movb	$0x1, %r8b
14146dcbb:     	callq	0x140911520 <.text+0x910520>
14146dcc0:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146dcca:     	movq	%rax, 0x940(%rbp)
14146dcd1:     	movb	$0x1, 0x977(%rbp)
14146dcd8:     	movq	0x8a8(%rbp), %rcx
14146dcdf:     	callq	0x140911b50 <.text+0x910b50>
14146dce4:     	movq	0x948(%rbp), %r8
14146dceb:     	movups	0x94(%r8), %xmm0
14146dcf3:     	movups	0xa4(%r8), %xmm1
14146dcfb:     	movups	0xb0(%r8), %xmm2
14146dd03:     	movq	0x558(%rbp), %rax
14146dd0a:     	movups	%xmm2, 0x1c(%rax)
14146dd0e:     	movups	%xmm1, 0x10(%rax)
14146dd12:     	movups	%xmm0, (%rax)
14146dd15:     	movq	0x30(%r8), %rax
14146dd19:     	movq	%rax, 0x950(%rbp)
14146dd20:     	movq	0x38(%r8), %rax
14146dd24:     	movq	%rax, 0x960(%rbp)
14146dd2b:     	movq	0x40(%r8), %rsi
14146dd2f:     	movl	0x90(%r8), %eax
14146dd36:     	movq	0x568(%rbp), %rcx
14146dd3d:     	movl	%eax, 0x40(%rcx)
14146dd40:     	movups	0x80(%r8), %xmm0
14146dd48:     	movups	%xmm0, 0x30(%rcx)
14146dd4c:     	movups	0x70(%r8), %xmm0
14146dd51:     	movups	%xmm0, 0x20(%rcx)
14146dd55:     	movups	0x60(%r8), %xmm0
14146dd5a:     	movups	%xmm0, 0x10(%rcx)
14146dd5e:     	movups	0x50(%r8), %xmm0
14146dd63:     	movups	%xmm0, (%rcx)
14146dd66:     	movl	0xe8(%r8), %eax
14146dd6d:     	movl	%eax, 0x938(%rbp)
14146dd73:     	movl	0xec(%r8), %eax
14146dd7a:     	movl	%eax, 0x958(%rbp)
14146dd80:     	movups	(%r8), %xmm0
14146dd84:     	movaps	%xmm0, -0x50(%rbp)
14146dd88:     	movq	0x10(%r8), %r9
14146dd8c:     	movq	0x18(%r8), %rax
14146dd90:     	movq	0x20(%r8), %rcx
14146dd94:     	movq	0x28(%r8), %rdx
14146dd98:     	movq	0x48(%r8), %r10
14146dd9c:     	movq	%r10, 0x8b8(%rbp)
14146dda3:     	movl	0xc0(%r8), %r8d
14146ddaa:     	movl	%r8d, 0x90c(%rbp)
14146ddb1:     	shlq	$0x4, %rdx
14146ddb5:     	addq	%rcx, %rdx
14146ddb8:     	movq	%rcx, 0x5b0(%rbp)
14146ddbf:     	movq	%rcx, 0x5b8(%rbp)
14146ddc6:     	movq	%rax, 0x5c0(%rbp)
14146ddcd:     	movq	%rdx, 0x5c8(%rbp)
14146ddd4:     	movaps	%xmm0, 0x1b0(%rbp)
14146dddb:     	movq	%r9, 0x8b0(%rbp)
14146dde2:     	movq	%r9, 0x1c0(%rbp)
14146dde9:     	leaq	0x17cae68(%rip), %r8    # 0x142c38c58
14146ddf0:     	leaq	0x360(%rbp), %rcx
14146ddf7:     	leaq	0x5b0(%rbp), %rdx
14146ddfe:     	callq	0x141737510 <.text+0x1736510>
14146de03:     	movq	%rsi, 0x8d8(%rbp)
14146de0a:     	movups	0x360(%rbp), %xmm0
14146de11:     	movups	%xmm0, 0x5c8(%rbp)
14146de18:     	movq	0x370(%rbp), %rax
14146de1f:     	movq	%rax, 0x5d8(%rbp)
14146de26:     	movaps	0x1b0(%rbp), %xmm0
14146de2d:     	movaps	%xmm0, 0x5b0(%rbp)
14146de34:     	movq	0x1c0(%rbp), %rax
14146de3b:     	movq	%rax, 0x5c0(%rbp)
14146de42:     	movq	0x918(%rbp), %rcx
14146de49:     	callq	0x14088acf0 <.text+0x889cf0>
14146de4e:     	movaps	0x5b0(%rbp), %xmm0
14146de55:     	movaps	0x5c0(%rbp), %xmm1
14146de5c:     	movaps	0x5d0(%rbp), %xmm2
14146de63:     	movq	0x918(%rbp), %rcx
14146de6a:     	movups	%xmm2, 0x20(%rcx)
14146de6e:     	movups	%xmm1, 0x10(%rcx)
14146de72:     	movups	%xmm0, (%rcx)
14146de75:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146de7f:     	movq	%rax, 0x30(%rcx)
14146de83:     	movq	0x8b0(%rbp), %rax
14146de8a:     	movq	%rax, 0x58(%rcx)
14146de8e:     	movq	0x8b8(%rbp), %rax
14146de95:     	movq	%rax, 0x60(%rcx)
14146de99:     	movl	0x938(%rbp), %eax
14146de9f:     	movl	%eax, 0x68(%rcx)
14146dea2:     	movl	0x958(%rbp), %eax
14146dea8:     	movl	%eax, 0x6c(%rcx)
14146deab:     	movl	0x90c(%rbp), %eax
14146deb1:     	movl	%eax, 0x70(%rcx)
14146deb4:     	btq	$0x38, 0x910(%rbp)
14146debd:     	jae	0x14146df22 <.text+0x146cf22>
14146debf:     	movq	0x578(%rbp), %rax
14146dec6:     	movl	0x8c8(%rbp), %ecx
14146decc:     	movl	%ecx, (%rax)
14146dece:     	movq	0x30(%r13), %rsi
14146ded2:     	movq	0x40(%r13), %rdi
14146ded6:     	cmpq	0x20(%r13), %rsi
14146deda:     	movq	0x960(%rbp), %rax
14146dee1:     	movq	0x950(%rbp), %rdx
14146dee8:     	jne	0x14146df0f <.text+0x146cf0f>
14146deea:     	leaq	0x20(%r13), %rcx
14146deee:     	movq	%rdx, 0x940(%rbp)
14146def5:     	movq	%rax, 0x920(%rbp)
14146defc:     	movb	$0x0, 0x977(%rbp)
14146df03:     	leaq	0x17cc23e(%rip), %rdx   # 0x142c3a148
14146df0a:     	callq	0x140562870 <.text+0x561870>
14146df0f:     	movq	0x28(%r13), %rax
14146df13:     	movq	%rdi, (%rax,%rsi,8)
14146df17:     	incq	%rsi
14146df1a:     	movq	%rsi, 0x30(%r13)
14146df1e:     	incq	0x40(%r13)
14146df22:     	cmpb	$0x0, 0x928(%rbp)
14146df29:     	jne	0x14146dfa6 <.text+0x146cfa6>
14146df2b:     	movq	0xa60(%rbp), %r15
14146df32:     	movq	0x950(%rbp), %rax
14146df39:     	movq	%rax, 0x940(%rbp)
14146df40:     	movq	0x960(%rbp), %rax
14146df47:     	movq	%rax, 0x920(%rbp)
14146df4e:     	movb	$0x0, 0x977(%rbp)
14146df55:     	movq	%r15, 0x20(%rsp)
14146df5a:     	leaq	0x5b0(%rbp), %rcx
14146df61:     	movq	0x918(%rbp), %rdx
14146df68:     	movq	%r12, %r8
14146df6b:     	movq	%r14, %r9
14146df6e:     	callq	0x1417a6e50 <.text+0x17a5e50>
14146df73:     	movq	0x5b8(%rbp), %rsi
14146df7a:     	cmpq	$0x3, %rsi
14146df7e:     	je	0x14146dfa6 <.text+0x146cfa6>
14146df80:     	movl	0x5b0(%rbp), %edi
14146df86:     	leaq	0x5c0(%rbp), %rdx
14146df8d:     	leaq	-0x50(%rbp), %rcx
14146df91:     	movl	$0x1a8, %r8d            # imm = 0x1A8
14146df97:     	callq	0x142923c00 <.text+0x2922c00>
14146df9c:     	cmpq	$0x2, %rsi
14146dfa0:     	jne	0x14146eb31 <.text+0x146db31>
14146dfa6:     	movq	0x918(%rbp), %rdi
14146dfad:     	movl	$0xffffffff, 0x68(%rdi) # imm = 0xFFFFFFFF
14146dfb4:     	movq	0x10(%rdi), %rax
14146dfb8:     	testq	%rax, %rax
14146dfbb:     	je	0x14146e7a6 <.text+0x146d7a6>
14146dfc1:     	movq	0x8(%rdi), %rcx
14146dfc5:     	leaq	(%rax,%rax,4), %rax
14146dfc9:     	leaq	(%rcx,%rax,8), %rax
14146dfcd:     	movq	%rax, 0x938(%rbp)
14146dfd4:     	xorl	%r8d, %r8d
14146dfd7:     	movabsq	$-0x7fffffffffffffff, %r10 # imm = 0x8000000000000001
14146dfe1:     	movabsq	$-0x7ffffffffffffff0, %r11 # imm = 0x8000000000000010
14146dfeb:     	leaq	0x17f6bfe(%rip), %rdi   # 0x142c64bf0
14146dff2:     	xorl	%r9d, %r9d
14146dff5:     	jmp	0x14146e018 <.text+0x146d018>
14146dff7:     	nopw	(%rax,%rax)
14146e000:     	movq	0x928(%rbp), %rcx
14146e007:     	addq	$0x28, %rcx
14146e00b:     	cmpq	0x938(%rbp), %rcx
14146e012:     	je	0x14146e7a9 <.text+0x146d7a9>
14146e018:     	movq	0x10(%rcx), %rax
14146e01c:     	testq	%rax, %rax
14146e01f:     	movq	%rcx, 0x928(%rbp)
14146e026:     	je	0x14146e000 <.text+0x146d000>
14146e028:     	movq	0x8(%rcx), %r12
14146e02c:     	leaq	(%rax,%rax,2), %rax
14146e030:     	leaq	(%r12,%rax,8), %rax
14146e034:     	movq	%rax, 0x958(%rbp)
14146e03b:     	jmp	0x14146e06b <.text+0x146d06b>
14146e03d:     	nopl	(%rax)
14146e040:     	movq	0x8(%r12), %rax
14146e045:     	movzbl	0x10(%rax), %edx
14146e049:     	leaq	0x17f6b78(%rip), %rcx   # 0x142c64bc8
14146e050:     	addq	(%rcx,%rdx,8), %rax
14146e054:     	movq	(%rax), %rax
14146e057:     	cmpq	%r9, %rax
14146e05a:     	cmovaq	%rax, %r9
14146e05e:     	addq	$0x18, %r12
14146e062:     	cmpq	0x958(%rbp), %r12
14146e069:     	je	0x14146e000 <.text+0x146d000>
14146e06b:     	movq	(%r12), %rax
14146e06f:     	cmpq	$0x7, %rax
14146e073:     	je	0x14146e0c0 <.text+0x146d0c0>
14146e075:     	cmpl	$0x8, %eax
14146e078:     	je	0x14146e040 <.text+0x146d040>
14146e07a:     	cmpl	$0x9, %eax
14146e07d:     	jne	0x14146e05e <.text+0x146d05e>
14146e07f:     	movq	0x8(%r12), %rax
14146e084:     	movzbl	0x18(%rax), %edx
14146e088:     	addb	$-0x3, %dl
14146e08b:     	cmpb	$0xe, %dl
14146e08e:     	movzbl	%dl, %edx
14146e091:     	movl	$0x3, %ecx
14146e096:     	cmovael	%ecx, %edx
14146e099:     	movzbl	%dl, %edx
14146e09c:     	leaq	0x17f6c4d(%rip), %rcx   # 0x142c64cf0
14146e0a3:     	movslq	(%rcx,%rdx,4), %rdx
14146e0a7:     	addq	%rcx, %rdx
14146e0aa:     	jmpq	*%rdx
14146e0ac:     	addq	$0x20, %rax
14146e0b0:     	jmp	0x14146e054 <.text+0x146d054>
14146e0b2:     	nopw	%cs:(%rax,%rax)
14146e0c0:     	movq	0x10(%r12), %rsi
14146e0c5:     	testq	%rsi, %rsi
14146e0c8:     	je	0x14146e05e <.text+0x146d05e>
14146e0ca:     	movq	0x8(%r12), %rax
14146e0cf:     	shlq	$0x4, %rsi
14146e0d3:     	addq	%rax, %rsi
14146e0d6:     	jmp	0x14146e0ed <.text+0x146d0ed>
14146e0d8:     	nopl	(%rax,%rax)
14146e0e0:     	addq	$0x10, %rax
14146e0e4:     	cmpq	%rsi, %rax
14146e0e7:     	je	0x14146e05e <.text+0x146d05e>
14146e0ed:     	movq	0x8(%rax), %rdx
14146e0f1:     	testq	%rdx, %rdx
14146e0f4:     	je	0x14146e0e0 <.text+0x146d0e0>
14146e0f6:     	movq	(%rax), %rbx
14146e0f9:     	shlq	$0x3, %rdx
14146e0fd:     	xorl	%r14d, %r14d
14146e100:     	movq	(%rbx,%r14), %r15
14146e104:     	movq	0x10(%r15), %rcx
14146e108:     	leaq	(%rcx,%r10), %r13
14146e10c:     	cmpq	%r11, %rcx
14146e10f:     	cmovgeq	%r8, %r13
14146e113:     	movq	(%rdi,%r13,8), %rcx
14146e117:     	movq	(%r15,%rcx), %rcx
14146e11b:     	cmpq	%r9, %rcx
14146e11e:     	cmovaq	%rcx, %r9
14146e122:     	addq	$0x8, %r14
14146e126:     	cmpq	%r14, %rdx
14146e129:     	jne	0x14146e100 <.text+0x146d100>
14146e12b:     	jmp	0x14146e0e0 <.text+0x146d0e0>
14146e12d:     	addq	$0x10, %rax
14146e131:     	jmp	0x14146e054 <.text+0x146d054>
14146e136:     	movb	%r14b, %sil
14146e139:     	movq	0x10(%rax), %rcx
14146e13d:     	movq	%rcx, -0x40(%rbp)
14146e141:     	movups	(%rax), %xmm0
14146e144:     	movaps	%xmm0, -0x50(%rbp)
14146e148:     	movq	%rdi, -0x38(%rbp)
14146e14c:     	leaq	-0x50(%rbp), %r15
14146e150:     	movq	%r15, %rcx
14146e153:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146e158:     	movq	%rdx, %r8
14146e15b:     	movq	0xa70(%rbp), %rcx
14146e162:     	movq	(%rcx), %r9
14146e165:     	leaq	0x1b0(%rbp), %r14
14146e16c:     	movq	%r14, %rcx
14146e16f:     	movq	%rax, %rdx
14146e172:     	callq	0x141470d00 <.text+0x146fd00>
14146e177:     	movzbl	0x1bf(%rbp), %ebx
14146e17e:     	cmpb	$0x2, %bl
14146e181:     	jne	0x14146ea18 <.text+0x146da18>
14146e187:     	movq	0x1b0(%rbp), %rdi
14146e18e:     	movq	%rdi, 0x868(%rbp)
14146e195:     	leaq	0x5b0(%rbp), %rsi
14146e19c:     	movq	%rsi, %rcx
14146e19f:     	callq	0x140c102e0 <.text+0xc0f2e0>
14146e1a4:     	movq	0x960(%rbp), %rax
14146e1ab:     	movl	0x1c(%rax), %ecx
14146e1ae:     	movq	0x10(%rax), %rax
14146e1b2:     	movl	%ecx, (%rax)
14146e1b4:     	movups	0x5b0(%rbp), %xmm0
14146e1bb:     	movq	0x8e0(%rbp), %rax
14146e1c2:     	movups	%xmm0, (%rax)
14146e1c5:     	movq	%r15, 0x1b0(%rbp)
14146e1cc:     	leaq	-0x70c6f3(%rip), %rax   # 0x140d61ae0 <.text+0xd60ae0>
14146e1d3:     	movq	%rax, 0x1b8(%rbp)
14146e1da:     	leaq	0x868(%rbp), %rax
14146e1e1:     	movq	%rax, 0x1c0(%rbp)
14146e1e8:     	leaq	-0x7215df(%rip), %rax   # 0x140d4cc10 <.text+0xd4bc10>
14146e1ef:     	movq	%rax, 0x1c8(%rbp)
14146e1f6:     	movq	$0x2, 0x5b0(%rbp)
14146e201:     	movq	$0x2, 0x5c0(%rbp)
14146e20c:     	movq	$0x0, 0x5d0(%rbp)
14146e217:     	movq	$0x20, 0x5d8(%rbp)
14146e222:     	movb	$0x3, 0x5e0(%rbp)
14146e229:     	movq	$0x2, 0x5e8(%rbp)
14146e234:     	movq	$0x2, 0x5f8(%rbp)
14146e23f:     	movq	$0x1, 0x608(%rbp)
14146e24a:     	movabsq	$0x400000020, %rax      # imm = 0x400000020
14146e254:     	movq	%rax, 0x610(%rbp)
14146e25b:     	movb	$0x3, 0x618(%rbp)
14146e262:     	leaq	0x17f5fe7(%rip), %rax   # 0x142c64250
14146e269:     	movq	%rax, 0x360(%rbp)
14146e270:     	movq	$0x2, 0x368(%rbp)
14146e27b:     	movq	%r14, 0x370(%rbp)
14146e282:     	movq	$0x2, 0x378(%rbp)
14146e28d:     	movq	%rsi, 0x380(%rbp)
14146e294:     	movq	$0x2, 0x388(%rbp)
14146e29f:     	leaq	0x178(%rbp), %rcx
14146e2a6:     	leaq	0x360(%rbp), %rsi
14146e2ad:     	movq	%rsi, %rdx
14146e2b0:     	callq	0x140d79f90 <.text+0xd78f90>
14146e2b5:     	movups	0x178(%rbp), %xmm0
14146e2bc:     	movaps	%xmm0, 0x880(%rbp)
14146e2c3:     	movq	0x188(%rbp), %rax
14146e2ca:     	movq	%rax, 0x890(%rbp)
14146e2d1:     	movq	0x21be0d0(%rip), %rax   # 0x14362c3a8
14146e2d8:     	movq	(%rax), %rax
14146e2db:     	testq	%rax, %rax
14146e2de:     	je	0x14146e3be <.text+0x146d3be>
14146e2e4:     	leaq	0x880(%rbp), %rax
14146e2eb:     	movq	%rax, 0x360(%rbp)
14146e2f2:     	leaq	-0x1432829(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
14146e2f9:     	movq	%rax, 0x368(%rbp)
14146e300:     	leaq	0x17f5f79(%rip), %rcx   # 0x142c64280
14146e307:     	callq	0x14067dd80 <.text+0x67cd80>
14146e30c:     	movups	(%rax), %xmm0
14146e30f:     	movl	0x10(%rax), %eax
14146e312:     	movq	$0x1, 0x5e0(%rbp)
14146e31d:     	leaq	0x17f5e24(%rip), %rcx   # 0x142c64148
14146e324:     	movq	%rcx, 0x5e8(%rbp)
14146e32b:     	movq	$0x26, 0x5f0(%rbp)
14146e336:     	leaq	0x17f5f33(%rip), %rdx   # 0x142c64270
14146e33d:     	movq	%rdx, 0x600(%rbp)
14146e344:     	movq	$0x1, 0x608(%rbp)
14146e34f:     	movq	%rsi, 0x610(%rbp)
14146e356:     	movq	$0x1, 0x618(%rbp)
14146e361:     	movq	$0x0, 0x620(%rbp)
14146e36c:     	movq	$0x0, 0x5b0(%rbp)
14146e377:     	movq	%rcx, 0x5b8(%rbp)
14146e37e:     	movq	$0x26, 0x5c0(%rbp)
14146e389:     	movq	$0x0, 0x5c8(%rbp)
14146e394:     	movups	%xmm0, 0x5d0(%rbp)
14146e39b:     	movl	$0x1, 0x5f8(%rbp)
14146e3a5:     	movl	%eax, 0x5fc(%rbp)
14146e3ab:     	leaq	0x972(%rbp), %rcx
14146e3b2:     	leaq	0x5b0(%rbp), %rdx
14146e3b9:     	callq	0x140d4a330 <.text+0xd49330>
14146e3be:     	movq	0x888(%rbp), %rcx
14146e3c5:     	movq	0x890(%rbp), %rdx
14146e3cc:     	callq	0x1408c10a0 <.text+0x8c00a0>
14146e3d1:     	movb	$0x2, 0x48(%r12)
14146e3d7:     	movl	0x8cc(%rbp), %eax
14146e3dd:     	movq	0x588(%rbp), %rcx
14146e3e4:     	movl	%eax, (%rcx)
14146e3e6:     	movq	0x10(%r12), %rax
14146e3eb:     	movq	%rax, 0x840(%rbp)
14146e3f2:     	movups	(%r12), %xmm0
14146e3f7:     	movaps	%xmm0, 0x830(%rbp)
14146e3fe:     	movq	$0x0, (%r12)
14146e406:     	movq	$0x8, 0x8(%r12)
14146e40f:     	movq	$0x0, 0x10(%r12)
14146e418:     	movq	0x540(%rbp), %rax
14146e41f:     	movl	0x864(%rbp), %ecx
14146e425:     	movl	%ecx, (%rax)
14146e427:     	movq	0x538(%rbp), %rbx
14146e42e:     	movq	0x30(%rbx), %rsi
14146e432:     	movq	0x40(%rbx), %rdi
14146e436:     	cmpq	0x20(%rbx), %rsi
14146e43a:     	jne	0x14146e453 <.text+0x146d453>
14146e43c:     	leaq	0x20(%rbx), %rcx
14146e440:     	movb	$0x1, 0x96f(%rbp)
14146e447:     	leaq	0x17cbcfa(%rip), %rdx   # 0x142c3a148
14146e44e:     	callq	0x140562870 <.text+0x561870>
14146e453:     	movq	0xa50(%rbp), %rax
14146e45a:     	movq	0xa48(%rbp), %rcx
14146e461:     	movq	0x28(%rbx), %rdx
14146e465:     	movq	%rdi, (%rdx,%rsi,8)
14146e469:     	incq	%rsi
14146e46c:     	movq	%rsi, 0x30(%rbx)
14146e470:     	incq	0x40(%rbx)
14146e474:     	movl	0x1c(%rax), %edx
14146e477:     	movq	0x10(%rax), %r8
14146e47b:     	movl	%edx, (%r8)
14146e47e:     	movq	(%rax), %rax
14146e481:     	movb	$0x2, (%rax)
14146e484:     	movl	0x1c(%rcx), %eax
14146e487:     	movq	0x10(%rcx), %rdx
14146e48b:     	movl	%eax, (%rdx)
14146e48d:     	movq	(%rcx), %rax
14146e490:     	incq	0x1d8(%rax)
14146e497:     	movq	$0x1, 0x380(%rbp)
14146e4a2:     	movq	$0x0, 0x388(%rbp)
14146e4ad:     	movabsq	$-0x8000000000000000, %rsi # imm = 0x8000000000000000
14146e4b7:     	movq	%rsi, 0x390(%rbp)
14146e4be:     	movb	$0x0, 0x3d0(%rbp)
14146e4c5:     	movq	%rsi, 0x3d8(%rbp)
14146e4cc:     	movq	$0x0, 0x360(%rbp)
14146e4d7:     	movq	$0x8, 0x368(%rbp)
14146e4e2:     	xorps	%xmm6, %xmm6
14146e4e5:     	movups	%xmm6, 0x370(%rbp)
14146e4ec:     	movb	$0x0, 0x3f8(%rbp)
14146e4f3:     	movq	0x830(%rbp), %rcx
14146e4fa:     	movq	0x838(%rbp), %rax
14146e501:     	movq	0x840(%rbp), %rdi
14146e508:     	shlq	$0x5, %rdi
14146e50c:     	addq	%rax, %rdi
14146e50f:     	movq	%rax, 0x1b0(%rbp)
14146e516:     	movq	%rax, 0x1b8(%rbp)
14146e51d:     	movq	%rcx, 0x1c0(%rbp)
14146e524:     	movq	%rdi, 0x1c8(%rbp)
14146e52b:     	leaq	0x5c0(%rbp), %rbx
14146e532:     	movabsq	$-0x7ffffffffffffffd, %r14 # imm = 0x8000000000000003
14146e53c:     	movl	$0x1, %r15d
14146e542:     	jmp	0x14146e582 <.text+0x146d582>
14146e544:     	nopw	%cs:(%rax,%rax)
14146e550:     	movq	0x368(%rbp), %rax
14146e557:     	movq	%r12, %rcx
14146e55a:     	shlq	$0x5, %rcx
14146e55e:     	movups	0x5b0(%rbp), %xmm0
14146e565:     	movups	0x5c0(%rbp), %xmm1
14146e56c:     	movups	%xmm1, 0x10(%rax,%rcx)
14146e571:     	movups	%xmm0, (%rax,%rcx)
14146e575:     	incq	%r12
14146e578:     	movq	%r12, 0x370(%rbp)
14146e57f:     	movq	%r13, %rax
14146e582:     	cmpq	%rdi, %rax
14146e585:     	je	0x14146e71a <.text+0x146d71a>
14146e58b:     	leaq	0x20(%rax), %r13
14146e58f:     	movq	%r13, 0x1b8(%rbp)
14146e596:     	movq	(%rax), %rcx
14146e599:     	cmpq	%r14, %rcx
14146e59c:     	je	0x14146e71a <.text+0x146d71a>
14146e5a2:     	movq	%rcx, %r8
14146e5a5:     	xorq	%rsi, %r8
14146e5a8:     	cmpq	$0x3, %r8
14146e5ac:     	cmovaeq	%r15, %r8
14146e5b0:     	testq	%r8, %r8
14146e5b3:     	je	0x14146e610 <.text+0x146d610>
14146e5b5:     	movq	0x8(%rax), %rdx
14146e5b9:     	cmpq	$0x1, %r8
14146e5bd:     	jne	0x14146e6e0 <.text+0x146d6e0>
14146e5c3:     	addq	$0x10, %rax
14146e5c7:     	movups	(%rax), %xmm0
14146e5ca:     	movaps	%xmm0, 0x160(%rbp)
14146e5d1:     	movq	%rcx, 0x5b0(%rbp)
14146e5d8:     	movq	%rdx, 0x5b8(%rbp)
14146e5df:     	movups	%xmm0, (%rbx)
14146e5e2:     	movq	0x370(%rbp), %r12
14146e5e9:     	cmpq	0x360(%rbp), %r12
14146e5f0:     	jne	0x14146e550 <.text+0x146d550>
14146e5f6:     	leaq	0x360(%rbp), %rcx
14146e5fd:     	leaq	0x17f5d04(%rip), %rdx   # 0x142c64308
14146e604:     	callq	0x140562c30 <.text+0x561c30>
14146e609:     	jmp	0x14146e550 <.text+0x146d550>
14146e60e:     	nop
14146e610:     	movq	0x21bdd91(%rip), %rax   # 0x14362c3a8
14146e617:     	movq	(%rax), %rcx
14146e61a:     	movq	%r13, %rax
14146e61d:     	cmpq	$0x2, %rcx
14146e621:     	jb	0x14146e582 <.text+0x146d582>
14146e627:     	leaq	0x17f5cc2(%rip), %rcx   # 0x142c642f0
14146e62e:     	callq	0x14067dd80 <.text+0x67cd80>
14146e633:     	movups	(%rax), %xmm0
14146e636:     	movl	0x10(%rax), %eax
14146e639:     	movq	$0x2, 0x5e0(%rbp)
14146e644:     	leaq	0x17f5afd(%rip), %rcx   # 0x142c64148
14146e64b:     	movq	%rcx, 0x5e8(%rbp)
14146e652:     	movq	$0x26, 0x5f0(%rbp)
14146e65d:     	leaq	0x17f5c7c(%rip), %rdx   # 0x142c642e0
14146e664:     	movq	%rdx, 0x600(%rbp)
14146e66b:     	movq	$0x1, 0x608(%rbp)
14146e676:     	movq	$0x8, 0x610(%rbp)
14146e681:     	movq	$0x0, 0x5b0(%rbp)
14146e68c:     	movups	%xmm6, 0x58(%rbx)
14146e690:     	movq	%rcx, 0x5b8(%rbp)
14146e697:     	movq	$0x26, 0x5c0(%rbp)
14146e6a2:     	movq	$0x0, 0x5c8(%rbp)
14146e6ad:     	movups	%xmm0, 0x5d0(%rbp)
14146e6b4:     	movl	$0x1, 0x5f8(%rbp)
14146e6be:     	movl	%eax, 0x5fc(%rbp)
14146e6c4:     	leaq	0x972(%rbp), %rcx
14146e6cb:     	leaq	0x5b0(%rbp), %rdx
14146e6d2:     	callq	0x140d4a330 <.text+0xd49330>
14146e6d7:     	movq	%r13, %rax
14146e6da:     	jmp	0x14146e582 <.text+0x146d582>
14146e6df:     	nop
14146e6e0:     	movq	%rdx, 0x5b8(%rbp)
14146e6e7:     	movq	%rsi, 0x5b0(%rbp)
14146e6ee:     	movq	0x370(%rbp), %r12
14146e6f5:     	cmpq	0x360(%rbp), %r12
14146e6fc:     	jne	0x14146e550 <.text+0x146d550>
14146e702:     	leaq	0x360(%rbp), %rcx
14146e709:     	leaq	0x17f5c10(%rip), %rdx   # 0x142c64320
14146e710:     	callq	0x140562c30 <.text+0x561c30>
14146e715:     	jmp	0x14146e550 <.text+0x146d550>
14146e71a:     	movb	$0x1, 0x96e(%rbp)
14146e721:     	leaq	0x1b0(%rbp), %rcx
14146e728:     	callq	0x14087b2f0 <.text+0x87a2f0>
14146e72d:     	leaq	0x5b0(%rbp), %rdi
14146e734:     	leaq	0x360(%rbp), %rdx
14146e73b:     	movl	$0xa0, %r8d
14146e741:     	movq	%rdi, %rcx
14146e744:     	callq	0x142923c00 <.text+0x2922c00>
14146e749:     	movb	$0x0, 0x96e(%rbp)
14146e750:     	leaq	0x17f5b41(%rip), %r8    # 0x142c64298
14146e757:     	movq	0xa58(%rbp), %rcx
14146e75e:     	movq	%rdi, %rdx
14146e761:     	callq	0x1412e0b90 <.text+0x12dfb90>
14146e766:     	movq	0x880(%rbp), %rdx
14146e76d:     	testq	%rdx, %rdx
14146e770:     	je	0x14146e784 <.text+0x146d784>
14146e772:     	movq	0x888(%rbp), %rcx
14146e779:     	movl	$0x1, %r8d
14146e77f:     	callq	0x140613c20 <.text+0x612c20>
14146e784:     	leaq	0x868(%rbp), %rcx
14146e78b:     	callq	0x140d4cc30 <.text+0xd4bc30>
14146e790:     	movq	-0x50(%rbp), %rdx
14146e794:     	testq	%rdx, %rdx
14146e797:     	je	0x14146f625 <.text+0x146e625>
14146e79d:     	movq	-0x48(%rbp), %rcx
14146e7a1:     	jmp	0x14146f61a <.text+0x146e61a>
14146e7a6:     	xorl	%r9d, %r9d
14146e7a9:     	movq	0x580(%rbp), %rcx
14146e7b0:     	movq	0xc0(%rcx), %rax
14146e7b7:     	cmpq	%rax, %r9
14146e7ba:     	cmovaq	%r9, %rax
14146e7be:     	movq	%rax, 0xc0(%rcx)
14146e7c5:     	movq	0x918(%rbp), %rcx
14146e7cc:     	movl	0x6c(%rcx), %eax
14146e7cf:     	testl	%eax, %eax
14146e7d1:     	movl	$0x0, %edx
14146e7d6:     	cmovgl	%eax, %edx
14146e7d9:     	movl	%edx, 0x6c(%rcx)
14146e7dc:     	movq	0x950(%rbp), %r12
14146e7e3:     	movq	%r12, 0x940(%rbp)
14146e7ea:     	movq	0x960(%rbp), %rsi
14146e7f1:     	movq	%rsi, 0x920(%rbp)
14146e7f8:     	movb	$0x0, 0x977(%rbp)
14146e7ff:     	callq	0x140d635e0 <.text+0xd625e0>
14146e804:     	movq	%rax, 0x590(%rbp)
14146e80b:     	movl	%edx, 0x598(%rbp)
14146e811:     	movq	0x948(%rbp), %rax
14146e818:     	movq	0x48(%rax), %rdi
14146e81c:     	movl	0xc0(%rax), %ebx
14146e822:     	movq	%r12, 0x940(%rbp)
14146e829:     	movq	%rsi, 0x920(%rbp)
14146e830:     	movb	$0x0, 0x977(%rbp)
14146e837:     	leaq	0x360(%rbp), %r13
14146e83e:     	movq	%r13, %rcx
14146e841:     	movq	0x8a8(%rbp), %rdx
14146e848:     	callq	0x140911710 <.text+0x910710>
14146e84d:     	movq	0x558(%rbp), %rax
14146e854:     	movw	0x28(%rax), %r14w
14146e859:     	movl	(%rax), %r15d
14146e85c:     	movups	0x4(%rax), %xmm8
14146e861:     	movups	0x14(%rax), %xmm9
14146e866:     	movss	0x24(%rax), %xmm10
14146e86c:     	movq	0x918(%rbp), %rax
14146e873:     	movsd	0x68(%rax), %xmm7
14146e878:     	movq	0x8(%rax), %rdx
14146e87c:     	movq	0x10(%rax), %rax
14146e880:     	leaq	(%rax,%rax,4), %rax
14146e884:     	leaq	(%rdx,%rax,8), %r8
14146e888:     	movq	%r12, 0x940(%rbp)
14146e88f:     	movq	%rsi, 0x920(%rbp)
14146e896:     	movb	$0x0, 0x977(%rbp)
14146e89d:     	leaq	0x17f55a4(%rip), %r9    # 0x142c63e48
14146e8a4:     	leaq	-0x50(%rbp), %rcx
14146e8a8:     	callq	0x14130dec0 <.text+0x130cec0>
14146e8ad:     	movq	0x918(%rbp), %rax
14146e8b4:     	movq	0x20(%rax), %rdx
14146e8b8:     	movq	0x28(%rax), %r8
14146e8bc:     	shlq	$0x4, %r8
14146e8c0:     	addq	%rdx, %r8
14146e8c3:     	leaq	0x17f557e(%rip), %r9    # 0x142c63e48
14146e8ca:     	leaq	0x1b0(%rbp), %r12
14146e8d1:     	movq	%r12, %rcx
14146e8d4:     	callq	0x1413056c0 <.text+0x13046c0>
14146e8d9:     	movq	0x950(%rbp), %rax
14146e8e0:     	negq	%rax
14146e8e3:     	jo	0x14146ee1e <.text+0x146de1e>
14146e8e9:     	xorl	%r13d, %r13d
14146e8ec:     	movl	$0x28, %ecx
14146e8f1:     	movq	0x8d8(%rbp), %rsi
14146e8f8:     	movq	%rsi, %rax
14146e8fb:     	mulq	%rcx
14146e8fe:     	movq	%rax, %r12
14146e901:     	jo	0x14146f9ca <.text+0x146e9ca>
14146e907:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14146e911:     	cmpq	%rax, %r12
14146e914:     	ja	0x14146f9ca <.text+0x146e9ca>
14146e91a:     	testq	%r12, %r12
14146e91d:     	je	0x14146eb15 <.text+0x146db15>
14146e923:     	movzbl	0x21c62df(%rip), %eax   # 0x143634c09
14146e92a:     	movl	$0x4, %r13d
14146e930:     	movl	$0x4, %edx
14146e935:     	movq	%r12, %rcx
14146e938:     	callq	0x140613c10 <.text+0x612c10>
14146e93d:     	testq	%rax, %rax
14146e940:     	je	0x14146f9ca <.text+0x146e9ca>
14146e946:     	testq	%rsi, %rsi
14146e949:     	leaq	0x360(%rbp), %r13
14146e950:     	je	0x14146ee0c <.text+0x146de0c>
14146e956:     	movq	0x960(%rbp), %r9
14146e95d:     	leaq	0x4(%r9), %rcx
14146e961:     	xorl	%edx, %edx
14146e963:     	movq	%rsi, %r8
14146e966:     	jmp	0x14146e9e0 <.text+0x146d9e0>
14146e968:     	nopl	(%rax,%rax)
14146e970:     	decq	%r8
14146e973:     	movl	0x5b8(%rbp), %r11d
14146e97a:     	movl	%r11d, 0x888(%rbp)
14146e981:     	movq	0x5b0(%rbp), %r11
14146e988:     	movq	%r11, 0x880(%rbp)
14146e98f:     	movl	%r10d, (%rax,%rdx)
14146e993:     	movl	0x888(%rbp), %r10d
14146e99a:     	movl	%r10d, 0xc(%rax,%rdx)
14146e99f:     	movq	0x880(%rbp), %r10
14146e9a6:     	movq	%r10, 0x4(%rax,%rdx)
14146e9ab:     	movq	0x10(%r9), %r10
14146e9af:     	movq	%r10, 0x10(%rax,%rdx)
14146e9b4:     	movl	0x18(%r9), %r10d
14146e9b8:     	movl	%r10d, 0x18(%rax,%rdx)
14146e9bd:     	movq	0x1c(%r9), %r10
14146e9c1:     	movq	%r10, 0x1c(%rax,%rdx)
14146e9c6:     	movl	0x24(%r9), %r10d
14146e9ca:     	addq	$0x28, %r9
14146e9ce:     	movl	%r10d, 0x24(%rax,%rdx)
14146e9d3:     	addq	$0x28, %rdx
14146e9d7:     	testq	%r8, %r8
14146e9da:     	je	0x14146eb09 <.text+0x146db09>
14146e9e0:     	cmpq	%rdx, %r12
14146e9e3:     	je	0x14146eb09 <.text+0x146db09>
14146e9e9:     	xorl	%r10d, %r10d
14146e9ec:     	cmpl	$0x1, (%r9)
14146e9f0:     	jne	0x14146e970 <.text+0x146d970>
14146e9f6:     	movl	0x8(%rcx,%rdx), %r10d
14146e9fb:     	movl	%r10d, 0x5b8(%rbp)
14146ea02:     	movq	(%rcx,%rdx), %r10
14146ea06:     	movq	%r10, 0x5b0(%rbp)
14146ea0d:     	movl	$0x1, %r10d
14146ea13:     	jmp	0x14146e970 <.text+0x146d970>
14146ea18:     	testb	$0x1, 0x1bc(%rbp)
14146ea1f:     	je	0x14146ea2a <.text+0x146da2a>
14146ea21:     	testb	%sil, %sil
14146ea24:     	je	0x14146f65f <.text+0x146e65f>
14146ea2a:     	movq	0x1b0(%rbp), %rdi
14146ea31:     	movl	0x1b8(%rbp), %eax
14146ea37:     	movl	%eax, 0x8f0(%rbp)
14146ea3d:     	movl	0x1bb(%rbp), %eax
14146ea43:     	movl	%eax, 0x8f3(%rbp)
14146ea49:     	movq	%rdi, 0x5b0(%rbp)
14146ea50:     	movl	0x8f0(%rbp), %eax
14146ea56:     	movl	0x8f3(%rbp), %ecx
14146ea5c:     	movl	%eax, 0x5b8(%rbp)
14146ea62:     	movl	%ecx, 0x5bb(%rbp)
14146ea68:     	movb	%bl, 0x5bf(%rbp)
14146ea6e:     	leaq	-0x50(%rbp), %rcx
14146ea72:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146ea77:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
14146ea81:     	movb	%sil, %r14b
14146ea84:     	movq	%rax, %rcx
14146ea87:     	callq	0x140d61cd0 <.text+0xd60cd0>
14146ea8c:     	testq	%rax, %rax
14146ea8f:     	je	0x14146fa64 <.text+0x146ea64>
14146ea95:     	movq	%rdx, %r8
14146ea98:     	movq	$0x9, 0x20(%rsp)
14146eaa1:     	leaq	0x17f5780(%rip), %r9    # 0x142c64228
14146eaa8:     	leaq	0x190(%rbp), %rcx
14146eaaf:     	movq	%rax, %rdx
14146eab2:     	callq	0x140d62810 <.text+0xd61810>
14146eab7:     	movq	0x5b8(%rbp), %rax
14146eabe:     	movq	%rax, 0x910(%rbp)
14146eac5:     	movq	0x190(%rbp), %rsi
14146eacc:     	movups	0x198(%rbp), %xmm0
14146ead3:     	movaps	%xmm0, 0x590(%rbp)
14146eada:     	movq	0x1a8(%rbp), %rax
14146eae1:     	movq	%rax, 0x5a0(%rbp)
14146eae8:     	movq	-0x50(%rbp), %rdx
14146eaec:     	testq	%rdx, %rdx
14146eaef:     	je	0x14146dbd0 <.text+0x146cbd0>
14146eaf5:     	movq	-0x48(%rbp), %rcx
14146eaf9:     	movl	$0x1, %r8d
14146eaff:     	callq	0x140613c20 <.text+0x612c20>
14146eb04:     	jmp	0x14146dbd0 <.text+0x146cbd0>
14146eb09:     	movq	%rsi, 0x8e8(%rbp)
14146eb10:     	jmp	0x14146ee17 <.text+0x146de17>
14146eb15:     	movl	$0x4, %eax
14146eb1a:     	movq	$0x0, 0x8e8(%rbp)
14146eb25:     	leaq	0x360(%rbp), %r13
14146eb2c:     	jmp	0x14146ee17 <.text+0x146de17>
14146eb31:     	movq	%rsi, 0x1b0(%rbp)
14146eb38:     	leaq	0x1b8(%rbp), %rcx
14146eb3f:     	leaq	-0x50(%rbp), %rdx
14146eb43:     	movl	$0x1a8, %r8d            # imm = 0x1A8
14146eb49:     	callq	0x142923c00 <.text+0x2922c00>
14146eb4e:     	btq	$0x30, 0x910(%rbp)
14146eb57:     	jae	0x14146ec9c <.text+0x146dc9c>
14146eb5d:     	movq	0x21bd844(%rip), %rax   # 0x14362c3a8
14146eb64:     	movq	(%rax), %rax
14146eb67:     	cmpq	$0x3, %rax
14146eb6b:     	jb	0x14146ec35 <.text+0x146dc35>
14146eb71:     	movb	$0x1, 0x973(%rbp)
14146eb78:     	leaq	0x17f5f21(%rip), %rcx   # 0x142c64aa0
14146eb7f:     	callq	0x14067dd80 <.text+0x67cd80>
14146eb84:     	movups	(%rax), %xmm0
14146eb87:     	movl	0x10(%rax), %eax
14146eb8a:     	movq	$0x3, 0x5e0(%rbp)
14146eb95:     	leaq	0x17f55ac(%rip), %rcx   # 0x142c64148
14146eb9c:     	movq	%rcx, 0x5e8(%rbp)
14146eba3:     	movq	$0x26, 0x5f0(%rbp)
14146ebae:     	leaq	0x17f5edb(%rip), %rdx   # 0x142c64a90
14146ebb5:     	movq	%rdx, 0x600(%rbp)
14146ebbc:     	movq	$0x1, 0x608(%rbp)
14146ebc7:     	movq	$0x8, 0x610(%rbp)
14146ebd2:     	movq	$0x0, 0x5b0(%rbp)
14146ebdd:     	xorps	%xmm1, %xmm1
14146ebe0:     	movups	%xmm1, 0x618(%rbp)
14146ebe7:     	movq	%rcx, 0x5b8(%rbp)
14146ebee:     	movq	$0x26, 0x5c0(%rbp)
14146ebf9:     	movq	$0x0, 0x5c8(%rbp)
14146ec04:     	movups	%xmm0, 0x5d0(%rbp)
14146ec0b:     	movl	$0x1, 0x5f8(%rbp)
14146ec15:     	movl	%eax, 0x5fc(%rbp)
14146ec1b:     	movb	$0x1, 0x973(%rbp)
14146ec22:     	leaq	0x972(%rbp), %rcx
14146ec29:     	leaq	0x5b0(%rbp), %rdx
14146ec30:     	callq	0x140d4a330 <.text+0xd49330>
14146ec35:     	movq	0x218(%rbp), %rcx
14146ec3c:     	testq	%rcx, %rcx
14146ec3f:     	je	0x14146ec9c <.text+0x146dc9c>
14146ec41:     	movq	0x210(%rbp), %rax
14146ec48:     	leaq	(%rcx,%rcx,4), %rcx
14146ec4c:     	leaq	(%rax,%rcx,8), %rcx
14146ec50:     	jmp	0x14146ec5b <.text+0x146dc5b>
14146ec52:     	addq	$0x28, %rax
14146ec56:     	cmpq	%rcx, %rax
14146ec59:     	je	0x14146ec9c <.text+0x146dc9c>
14146ec5b:     	movq	0x20(%rax), %r8
14146ec5f:     	testq	%r8, %r8
14146ec62:     	je	0x14146ec52 <.text+0x146dc52>
14146ec64:     	movq	0x18(%rax), %rdx
14146ec68:     	imulq	$0x58, %r8, %r8
14146ec6c:     	xorl	%r9d, %r9d
14146ec6f:     	jmp	0x14146ec89 <.text+0x146dc89>
14146ec71:     	nopw	%cs:(%rax,%rax)
14146ec80:     	addq	$0x58, %r9
14146ec84:     	cmpq	%r9, %r8
14146ec87:     	je	0x14146ec52 <.text+0x146dc52>
14146ec89:     	cmpb	$0x3a, 0x52(%rdx,%r9)
14146ec8f:     	jne	0x14146ec80 <.text+0x146dc80>
14146ec91:     	movl	$0x3e7, 0x30(%rdx,%r9)  # imm = 0x3E7
14146ec9a:     	jmp	0x14146ec80 <.text+0x146dc80>
14146ec9c:     	btq	$0x28, 0x910(%rbp)
14146eca5:     	jae	0x14146f79e <.text+0x146e79e>
14146ecab:     	movq	0x21bd6f6(%rip), %rax   # 0x14362c3a8
14146ecb2:     	movq	(%rax), %rax
14146ecb5:     	cmpq	$0x3, %rax
14146ecb9:     	jb	0x14146ed83 <.text+0x146dd83>
14146ecbf:     	movb	$0x1, 0x973(%rbp)
14146ecc6:     	leaq	0x17f5e33(%rip), %rcx   # 0x142c64b00
14146eccd:     	callq	0x14067dd80 <.text+0x67cd80>
14146ecd2:     	movups	(%rax), %xmm0
14146ecd5:     	movl	0x10(%rax), %eax
14146ecd8:     	movq	$0x3, 0x5e0(%rbp)
14146ece3:     	leaq	0x17f545e(%rip), %rcx   # 0x142c64148
14146ecea:     	movq	%rcx, 0x5e8(%rbp)
14146ecf1:     	movq	$0x26, 0x5f0(%rbp)
14146ecfc:     	leaq	0x17f5ded(%rip), %rdx   # 0x142c64af0
14146ed03:     	movq	%rdx, 0x600(%rbp)
14146ed0a:     	movq	$0x1, 0x608(%rbp)
14146ed15:     	movq	$0x8, 0x610(%rbp)
14146ed20:     	movq	$0x0, 0x5b0(%rbp)
14146ed2b:     	xorps	%xmm1, %xmm1
14146ed2e:     	movups	%xmm1, 0x618(%rbp)
14146ed35:     	movq	%rcx, 0x5b8(%rbp)
14146ed3c:     	movq	$0x26, 0x5c0(%rbp)
14146ed47:     	movq	$0x0, 0x5c8(%rbp)
14146ed52:     	movups	%xmm0, 0x5d0(%rbp)
14146ed59:     	movl	$0x1, 0x5f8(%rbp)
14146ed63:     	movl	%eax, 0x5fc(%rbp)
14146ed69:     	movb	$0x1, 0x973(%rbp)
14146ed70:     	leaq	0x972(%rbp), %rcx
14146ed77:     	leaq	0x5b0(%rbp), %rdx
14146ed7e:     	callq	0x140d4a330 <.text+0xd49330>
14146ed83:     	movq	0x248(%rbp), %rax
14146ed8a:     	testq	%rax, %rax
14146ed8d:     	je	0x14146f6d8 <.text+0x146e6d8>
14146ed93:     	movq	0x240(%rbp), %rcx
14146ed9a:     	shlq	$0x4, %rax
14146ed9e:     	xorl	%edx, %edx
14146eda0:     	leaq	0x17f5f01(%rip), %r8    # 0x142c64ca8
14146eda7:     	jmp	0x14146edbe <.text+0x146ddbe>
14146eda9:     	movl	%r10d, %r9d
14146edac:     	movl	%r9d, 0x8(%rcx,%rdx)
14146edb1:     	addq	$0x10, %rdx
14146edb5:     	cmpq	%rdx, %rax
14146edb8:     	je	0x14146f6d8 <.text+0x146e6d8>
14146edbe:     	movl	0x8(%rcx,%rdx), %r10d
14146edc3:     	cmpq	$0x8, %r10
14146edc7:     	ja	0x14146eddb <.text+0x146dddb>
14146edc9:     	movslq	(%r8,%r10,4), %r9
14146edcd:     	addq	%r8, %r9
14146edd0:     	jmpq	*%r9
14146edd3:     	movl	$0x2, %r9d
14146edd9:     	jmp	0x14146edac <.text+0x146ddac>
14146eddb:     	movl	$0x1, %r9d
14146ede1:     	cmpl	$0x3e7, %r10d           # imm = 0x3E7
14146ede8:     	je	0x14146eda9 <.text+0x146dda9>
14146edea:     	jmp	0x14146edac <.text+0x146ddac>
14146edec:     	movl	$0x3, %r9d
14146edf2:     	jmp	0x14146edac <.text+0x146ddac>
14146edf4:     	movl	$0x5, %r9d
14146edfa:     	jmp	0x14146edac <.text+0x146ddac>
14146edfc:     	movl	$0x8, %r9d
14146ee02:     	jmp	0x14146edac <.text+0x146ddac>
14146ee04:     	movl	$0x4, %r9d
14146ee0a:     	jmp	0x14146edac <.text+0x146ddac>
14146ee0c:     	movq	$0x0, 0x8e8(%rbp)
14146ee17:     	leaq	0x1b0(%rbp), %r12
14146ee1e:     	movq	0x568(%rbp), %rsi
14146ee25:     	movzbl	0x42(%rsi), %ecx
14146ee29:     	movzwl	0x40(%rsi), %edx
14146ee2d:     	movl	0x38(%rsi), %r8d
14146ee31:     	movss	0x3c(%rsi), %xmm0
14146ee36:     	movzbl	0x14(%rsi), %r9d
14146ee3b:     	movl	0x10(%rsi), %r10d
14146ee3f:     	movq	%rdi, 0x5f8(%rbp)
14146ee46:     	movl	%ebx, 0x670(%rbp)
14146ee4c:     	movups	0x360(%rbp), %xmm1
14146ee53:     	movups	0x370(%rbp), %xmm2
14146ee5a:     	movups	%xmm1, 0x674(%rbp)
14146ee61:     	movups	%xmm2, 0x684(%rbp)
14146ee68:     	movl	0x380(%rbp), %r11d
14146ee6f:     	movl	%r11d, 0x694(%rbp)
14146ee76:     	movl	%r15d, 0x644(%rbp)
14146ee7d:     	movups	%xmm8, 0x648(%rbp)
14146ee85:     	movups	%xmm9, 0x658(%rbp)
14146ee8d:     	movss	%xmm10, 0x668(%rbp)
14146ee96:     	movw	%r14w, 0x66c(%rbp)
14146ee9e:     	movlps	%xmm7, 0x698(%rbp)
14146eea5:     	movups	-0x50(%rbp), %xmm1
14146eea9:     	movaps	%xmm1, 0x5b0(%rbp)
14146eeb0:     	movq	-0x40(%rbp), %r11
14146eeb4:     	movq	%r11, 0x5c0(%rbp)
14146eebb:     	movq	0x1c0(%rbp), %r11
14146eec2:     	movq	%r11, 0x5d8(%rbp)
14146eec9:     	movups	0x1b0(%rbp), %xmm1
14146eed0:     	movups	%xmm1, 0x5c8(%rbp)
14146eed7:     	movq	0x8e8(%rbp), %r11
14146eede:     	movq	%r11, 0x5e0(%rbp)
14146eee5:     	movq	%rax, 0x5e8(%rbp)
14146eeec:     	movq	0x8d8(%rbp), %rax
14146eef3:     	movq	%rax, 0x5f0(%rbp)
14146eefa:     	movups	(%rsi), %xmm1
14146eefd:     	movaps	%xmm1, 0x600(%rbp)
14146ef04:     	movl	%r10d, 0x610(%rbp)
14146ef0b:     	movb	%r9b, 0x614(%rbp)
14146ef12:     	movups	0x18(%rsi), %xmm1
14146ef16:     	movups	%xmm1, 0x618(%rbp)
14146ef1d:     	movups	0x28(%rsi), %xmm1
14146ef21:     	movups	%xmm1, 0x628(%rbp)
14146ef28:     	movl	%r8d, 0x638(%rbp)
14146ef2f:     	movss	%xmm0, 0x63c(%rbp)
14146ef37:     	movw	%dx, 0x640(%rbp)
14146ef3e:     	movb	%cl, 0x642(%rbp)
14146ef44:     	movzbl	0x21c5cbe(%rip), %eax   # 0x143634c09
14146ef4b:     	movl	$0xf0, %ecx
14146ef50:     	movl	$0x8, %edx
14146ef55:     	callq	0x140613c10 <.text+0x612c10>
14146ef5a:     	testq	%rax, %rax
14146ef5d:     	movq	0x948(%rbp), %rbx
14146ef64:     	movq	0x900(%rbp), %r14
14146ef6b:     	je	0x14146f94e <.text+0x146e94e>
14146ef71:     	leaq	0x5b0(%rbp), %rdx
14146ef78:     	movl	$0xf0, %r8d
14146ef7e:     	movq	%rax, 0x958(%rbp)
14146ef85:     	movq	%rax, %rcx
14146ef88:     	callq	0x142923c00 <.text+0x2922c00>
14146ef8d:     	movq	0x21bd414(%rip), %rax   # 0x14362c3a8
14146ef94:     	movq	(%rax), %rax
14146ef97:     	cmpq	$0x4, %rax
14146ef9b:     	movq	0x918(%rbp), %rdi
14146efa2:     	jb	0x14146f09b <.text+0x146e09b>
14146efa8:     	leaq	0x590(%rbp), %rcx
14146efaf:     	callq	0x140d63910 <.text+0xd62910>
14146efb4:     	movq	%rax, 0x360(%rbp)
14146efbb:     	movl	%edx, 0x368(%rbp)
14146efc1:     	movq	%r13, 0x1b0(%rbp)
14146efc8:     	leaq	-0x6e4a7f(%rip), %rax   # 0x140d8a550 <.text+0xd89550>
14146efcf:     	movq	%rax, 0x1b8(%rbp)
14146efd6:     	leaq	0x17f5b7b(%rip), %rcx   # 0x142c64b58
14146efdd:     	callq	0x14067dd80 <.text+0x67cd80>
14146efe2:     	movups	(%rax), %xmm0
14146efe5:     	movl	0x10(%rax), %eax
14146efe8:     	movq	$0x4, 0x5e0(%rbp)
14146eff3:     	leaq	0x17f514e(%rip), %rcx   # 0x142c64148
14146effa:     	movq	%rcx, 0x5e8(%rbp)
14146f001:     	movq	$0x26, 0x5f0(%rbp)
14146f00c:     	leaq	0x17f5b35(%rip), %rdx   # 0x142c64b48
14146f013:     	movq	%rdx, 0x600(%rbp)
14146f01a:     	movq	$0x1, 0x608(%rbp)
14146f025:     	movq	%r12, 0x610(%rbp)
14146f02c:     	movq	$0x1, 0x618(%rbp)
14146f037:     	movq	$0x0, 0x620(%rbp)
14146f042:     	movq	$0x0, 0x5b0(%rbp)
14146f04d:     	movq	%rcx, 0x5b8(%rbp)
14146f054:     	movq	$0x26, 0x5c0(%rbp)
14146f05f:     	movq	$0x0, 0x5c8(%rbp)
14146f06a:     	movups	%xmm0, 0x5d0(%rbp)
14146f071:     	movl	$0x1, 0x5f8(%rbp)
14146f07b:     	movl	%eax, 0x5fc(%rbp)
14146f081:     	leaq	0x972(%rbp), %rcx
14146f088:     	leaq	0x5b0(%rbp), %rdx
14146f08f:     	callq	0x140d4a330 <.text+0xd49330>
14146f094:     	movq	0x948(%rbp), %rbx
14146f09b:     	movq	0xa68(%rbp), %rsi
14146f0a2:     	movl	$0xf0, %edx
14146f0a7:     	movl	$0x8, %r8d
14146f0ad:     	movq	%rbx, %rcx
14146f0b0:     	callq	0x140613c20 <.text+0x612c20>
14146f0b5:     	movq	(%rsi), %rax
14146f0b8:     	movss	0x114(%rax), %xmm0
14146f0c0:     	movq	0x560(%rbp), %rax
14146f0c7:     	movl	0x8c0(%rbp), %ecx
14146f0cd:     	movl	%ecx, (%rax)
14146f0cf:     	movq	0x8a8(%rbp), %rax
14146f0d6:     	movss	%xmm0, 0x194(%rax)
14146f0de:     	movl	0x68(%rdi), %edi
14146f0e1:     	movq	0x910(%rbp), %rax
14146f0e8:     	subl	%edi, %eax
14146f0ea:     	movl	%eax, 0x858(%rbp)
14146f0f0:     	movq	0x550(%rbp), %rcx
14146f0f7:     	testq	%rcx, %rcx
14146f0fa:     	je	0x14146f1a5 <.text+0x146e1a5>
14146f100:     	cmpl	$-0x1, %edi
14146f103:     	jne	0x14146fa13 <.text+0x146ea13>
14146f109:     	cmpl	%ecx, %eax
14146f10b:     	jge	0x14146f1a7 <.text+0x146e1a7>
14146f111:     	leaq	0x550(%rbp), %rax
14146f118:     	movq	%rax, 0x360(%rbp)
14146f11f:     	leaq	-0x6e1a66(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
14146f126:     	movq	%rax, 0x368(%rbp)
14146f12d:     	leaq	0x858(%rbp), %rax
14146f134:     	movq	%rax, 0x370(%rbp)
14146f13b:     	leaq	-0x6e1bc2(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
14146f142:     	movq	%rax, 0x378(%rbp)
14146f149:     	leaq	0x17f5288(%rip), %rax   # 0x142c643d8
14146f150:     	movq	%rax, 0x5b0(%rbp)
14146f157:     	movq	$0x3, 0x5b8(%rbp)
14146f162:     	movq	$0x0, 0x5d0(%rbp)
14146f16d:     	movq	%r13, 0x5c0(%rbp)
14146f174:     	movq	$0x2, 0x5c8(%rbp)
14146f17f:     	movb	$0x1, 0x975(%rbp)
14146f186:     	movb	$0x1, 0x974(%rbp)
14146f18d:     	leaq	0x17f5274(%rip), %rdx   # 0x142c64408
14146f194:     	leaq	0x5b0(%rbp), %rcx
14146f19b:     	callq	0x1428d9390 <.text+0x28d8390>
14146f1a0:     	jmp	0x14146fa70 <.text+0x146ea70>
14146f1a5:     	xorl	%ecx, %ecx
14146f1a7:     	movl	0x8cc(%rbp), %edx
14146f1ad:     	movq	0x588(%rbp), %r8
14146f1b4:     	movl	%edx, (%r8)
14146f1b7:     	subl	%ecx, %eax
14146f1b9:     	movb	$0x1, 0x48(%r14)
14146f1be:     	movl	%eax, 0x4c(%r14)
14146f1c2:     	movq	0x870(%rbp), %rax
14146f1c9:     	movq	0xf8(%rax), %rcx
14146f1d0:     	movq	0x100(%rax), %rdx
14146f1d7:     	movzbl	0x108(%rax), %r8d
14146f1df:     	testb	%r8b, %r8b
14146f1e2:     	je	0x14146f28b <.text+0x146e28b>
14146f1e8:     	leaq	0x370(%rbp), %r9
14146f1ef:     	leaq	0x378(%rbp), %r10
14146f1f6:     	leaq	0x368(%rbp), %r11
14146f1fd:     	leaq	(,%rdx,4), %rsi
14146f205:     	xorl	%r15d, %r15d
14146f208:     	movq	%rcx, %rax
14146f20b:     	nopl	(%rax,%rax)
14146f210:     	testq	%rsi, %rsi
14146f213:     	je	0x14146f371 <.text+0x146e371>
14146f219:     	movl	(%rax), %r14d
14146f21c:     	addq	$0x4, %rax
14146f220:     	movq	0x8f8(%rbp), %rbx
14146f227:     	movq	0x1a8(%rbx), %rbx
14146f22e:     	leaq	(%r14,%r14,8), %r12
14146f232:     	movq	0x10(%rbx,%r12,8), %r14
14146f237:     	addq	$-0x4, %rsi
14146f23b:     	testq	%r14, %r14
14146f23e:     	je	0x14146f210 <.text+0x146e210>
14146f240:     	leaq	(%rbx,%r12,8), %rsi
14146f244:     	movq	0x870(%rbp), %rbx
14146f24b:     	movq	0x110(%rbx), %rbx
14146f252:     	movq	0x18(%rsi), %r15
14146f256:     	movq	0x38(%rsi), %rsi
14146f25a:     	movq	(%rsi,%rbx,8), %rsi
14146f25e:     	notq	%rsi
14146f261:     	leaq	(%rsi,%rsi,2), %r12
14146f265:     	shlq	$0x4, %r12
14146f269:     	movq	0x28(%r15,%r12), %rsi
14146f26e:     	movq	0x10(%r15,%r12), %rbx
14146f273:     	movq	0x20(%r15,%r12), %r15
14146f278:     	movq	%rbx, 0x360(%rbp)
14146f27f:     	movq	%r15, 0x368(%rbp)
14146f286:     	jmp	0x14146f35a <.text+0x146e35a>
14146f28b:     	leaq	0x5c0(%rbp), %r9
14146f292:     	leaq	0x5c8(%rbp), %r10
14146f299:     	leaq	0x5b8(%rbp), %r11
14146f2a0:     	leaq	(,%rdx,4), %rsi
14146f2a8:     	xorl	%r15d, %r15d
14146f2ab:     	movq	%rcx, %rax
14146f2ae:     	nop
14146f2b0:     	testq	%rsi, %rsi
14146f2b3:     	je	0x14146f371 <.text+0x146e371>
14146f2b9:     	movl	(%rax), %r14d
14146f2bc:     	addq	$0x4, %rax
14146f2c0:     	movq	0x8f8(%rbp), %rbx
14146f2c7:     	movq	0x100(%rbx), %rbx
14146f2ce:     	leaq	(%r14,%r14,4), %r12
14146f2d2:     	shlq	$0x5, %r12
14146f2d6:     	movq	0x58(%rbx,%r12), %r14
14146f2db:     	addq	$-0x4, %rsi
14146f2df:     	testq	%r14, %r14
14146f2e2:     	je	0x14146f2b0 <.text+0x146e2b0>
14146f2e4:     	addq	%r12, %rbx
14146f2e7:     	movl	0x94(%rbx), %esi
14146f2ed:     	movq	0x8f8(%rbp), %r15
14146f2f4:     	movq	0x1a8(%r15), %r15
14146f2fb:     	leaq	(%rsi,%rsi,8), %rsi
14146f2ff:     	movq	0x870(%rbp), %r12
14146f306:     	movq	0x110(%r12), %r12
14146f30e:     	movq	0x18(%r15,%rsi,8), %r13
14146f313:     	movq	0x38(%r15,%rsi,8), %rsi
14146f318:     	movq	(%rsi,%r12,8), %rsi
14146f31c:     	notq	%rsi
14146f31f:     	movq	0x50(%rbx), %rbx
14146f323:     	leaq	(%rsi,%rsi,2), %r15
14146f327:     	shlq	$0x4, %r15
14146f32b:     	movl	0x8(%rbx), %esi
14146f32e:     	leaq	(%rsi,%rsi,4), %rbx
14146f332:     	shlq	$0x4, %rbx
14146f336:     	addq	0x10(%r13,%r15), %rbx
14146f33b:     	shlq	$0x2, %rsi
14146f33f:     	movq	0x20(%r13,%r15), %r12
14146f344:     	addq	%rsi, %r12
14146f347:     	addq	0x28(%r13,%r15), %rsi
14146f34c:     	movq	%rbx, 0x5b0(%rbp)
14146f353:     	movq	%r12, 0x5b8(%rbp)
14146f35a:     	movq	%rsi, (%r9)
14146f35d:     	movlps	%xmm6, (%r10)
14146f361:     	movq	0x8(%r11), %r9
14146f365:     	movq	0x10(%r11), %r10
14146f369:     	movl	$0x1, %r15d
14146f36f:     	jmp	0x14146f376 <.text+0x146e376>
14146f371:     	xorl	%r14d, %r14d
14146f374:     	xorl	%ebx, %ebx
14146f376:     	movq	0x960(%rbp), %rsi
14146f37d:     	movq	0x950(%rbp), %r12
14146f384:     	leaq	(%rcx,%rdx,4), %rcx
14146f388:     	testb	%r8b, %r8b
14146f38b:     	je	0x14146f3c4 <.text+0x146e3c4>
14146f38d:     	cmpq	%r14, %r15
14146f390:     	movq	0x8f8(%rbp), %r11
14146f397:     	jne	0x14146f3f6 <.text+0x146e3f6>
14146f399:     	nopl	(%rax)
14146f3a0:     	cmpq	%rcx, %rax
14146f3a3:     	je	0x14146f45f <.text+0x146e45f>
14146f3a9:     	movl	(%rax), %edx
14146f3ab:     	addq	$0x4, %rax
14146f3af:     	movq	0x1a8(%r11), %r8
14146f3b6:     	leaq	(%rdx,%rdx,8), %rdx
14146f3ba:     	cmpq	$0x0, 0x10(%r8,%rdx,8)
14146f3c0:     	je	0x14146f3a0 <.text+0x146e3a0>
14146f3c2:     	jmp	0x14146f3f6 <.text+0x146e3f6>
14146f3c4:     	cmpq	%r14, %r15
14146f3c7:     	movq	0x8f8(%rbp), %r11
14146f3ce:     	jne	0x14146f3f6 <.text+0x146e3f6>
14146f3d0:     	cmpq	%rcx, %rax
14146f3d3:     	je	0x14146f45f <.text+0x146e45f>
14146f3d9:     	movl	(%rax), %edx
14146f3db:     	addq	$0x4, %rax
14146f3df:     	movq	0x100(%r11), %r8
14146f3e6:     	leaq	(%rdx,%rdx,4), %rdx
14146f3ea:     	shlq	$0x5, %rdx
14146f3ee:     	cmpq	$0x0, 0x58(%r8,%rdx)
14146f3f4:     	je	0x14146f3d0 <.text+0x146e3d0>
14146f3f6:     	testq	%rbx, %rbx
14146f3f9:     	sete	%al
14146f3fc:     	xorb	$0x1, %al
14146f3fe:     	movzbl	%al, %eax
14146f401:     	movq	%rax, 0x5b0(%rbp)
14146f408:     	leaq	0x17f4ba3(%rip), %rax   # 0x142c63fb2
14146f40f:     	movq	%rax, 0x5b8(%rbp)
14146f416:     	movq	$0x5f, 0x5c0(%rbp)
14146f421:     	movb	$0x1, 0x975(%rbp)
14146f428:     	movb	$0x1, 0x974(%rbp)
14146f42f:     	leaq	0x17f4fea(%rip), %rax   # 0x142c64420
14146f436:     	movq	%rax, 0x20(%rsp)
14146f43b:     	leaq	0x17f4a3e(%rip), %rcx   # 0x142c63e80
14146f442:     	leaq	0x17f4a67(%rip), %r9    # 0x142c63eb0
14146f449:     	leaq	0x5b0(%rbp), %r8
14146f450:     	movl	$0x2b, %edx
14146f455:     	callq	0x1428d9760 <.text+0x28d8760>
14146f45a:     	jmp	0x14146fa70 <.text+0x146ea70>
14146f45f:     	testq	%rbx, %rbx
14146f462:     	sete	%al
14146f465:     	je	0x14146f3fc <.text+0x146e3fc>
14146f467:     	shrq	$0x20, %r10
14146f46b:     	movl	%r10d, (%r9)
14146f46e:     	movq	(%rbx), %rax
14146f471:     	movq	%rax, %rcx
14146f474:     	negq	%rcx
14146f477:     	jo	0x14146f492 <.text+0x146e492>
14146f479:     	jae	0x14146f492 <.text+0x146e492>
14146f47b:     	movq	0x8(%rbx), %rcx
14146f47f:     	shlq	$0x3, %rax
14146f483:     	leaq	(%rax,%rax,4), %rdx
14146f487:     	movl	$0x4, %r8d
14146f48d:     	callq	0x140613c20 <.text+0x612c20>
14146f492:     	movq	%r12, (%rbx)
14146f495:     	movq	%rsi, 0x8(%rbx)
14146f499:     	movq	0x8d8(%rbp), %rax
14146f4a0:     	movq	%rax, 0x10(%rbx)
14146f4a4:     	cmpl	%edi, 0x910(%rbp)
14146f4aa:     	jne	0x14146f50f <.text+0x146e50f>
14146f4ac:     	movq	0x530(%rbp), %rax
14146f4b3:     	movl	0x860(%rbp), %ecx
14146f4b9:     	movl	%ecx, (%rax)
14146f4bb:     	movq	0x878(%rbp), %rax
14146f4c2:     	movq	0x30(%rax), %rsi
14146f4c6:     	movq	0x40(%rax), %rdi
14146f4ca:     	cmpq	0x20(%rax), %rsi
14146f4ce:     	jne	0x14146f4f5 <.text+0x146e4f5>
14146f4d0:     	movq	0x878(%rbp), %rax
14146f4d7:     	leaq	0x20(%rax), %rcx
14146f4db:     	movb	$0x0, 0x975(%rbp)
14146f4e2:     	movb	$0x1, 0x974(%rbp)
14146f4e9:     	leaq	0x1819b88(%rip), %rdx   # 0x142c89078
14146f4f0:     	callq	0x140562870 <.text+0x561870>
14146f4f5:     	movq	0x878(%rbp), %rcx
14146f4fc:     	movq	0x28(%rcx), %rax
14146f500:     	movq	%rdi, (%rax,%rsi,8)
14146f504:     	incq	%rsi
14146f507:     	movq	%rsi, 0x30(%rcx)
14146f50b:     	incq	0x40(%rcx)
14146f50f:     	movq	0x8e0(%rbp), %rax
14146f516:     	movups	(%rax), %xmm0
14146f519:     	movaps	%xmm0, 0x5b0(%rbp)
14146f520:     	movb	$0x0, 0x975(%rbp)
14146f527:     	movb	$0x1, 0x974(%rbp)
14146f52e:     	leaq	0x1b0(%rbp), %rcx
14146f535:     	leaq	0x5b0(%rbp), %rdx
14146f53c:     	callq	0x140ae8470 <.text+0xae7470>
14146f541:     	movq	0xa40(%rbp), %rax
14146f548:     	movq	0x918(%rbp), %rdx
14146f54f:     	movq	0x10(%rdx), %rcx
14146f553:     	movq	0x570(%rbp), %r8
14146f55a:     	movl	0x8c4(%rbp), %r9d
14146f561:     	movl	%r9d, (%r8)
14146f564:     	movq	%rcx, 0x58(%rdx)
14146f568:     	movl	0x1c(%rax), %r8d
14146f56c:     	movq	(%rax), %rdx
14146f56f:     	movq	0x10(%rax), %rax
14146f573:     	movq	%rcx, 0x120(%rdx)
14146f57a:     	movl	0x1c8(%rbp), %ecx
14146f580:     	movl	%r8d, (%rax)
14146f583:     	movl	%ecx, 0x140(%rdx)
14146f589:     	movl	$0x0, 0x144(%rdx)
14146f593:     	movq	0x8e0(%rbp), %rax
14146f59a:     	movups	(%rax), %xmm0
14146f59d:     	movups	%xmm0, 0x5e0(%rbp)
14146f5a4:     	movups	0x1b0(%rbp), %xmm0
14146f5ab:     	movups	0x1c0(%rbp), %xmm1
14146f5b2:     	movups	%xmm0, 0x5b8(%rbp)
14146f5b9:     	movups	%xmm1, 0x5c8(%rbp)
14146f5c0:     	movq	0x958(%rbp), %rax
14146f5c7:     	movq	%rax, 0x5d8(%rbp)
14146f5ce:     	movq	$0x3, 0x5b0(%rbp)
14146f5d9:     	movb	$0x0, 0x975(%rbp)
14146f5e0:     	movb	$0x0, 0x974(%rbp)
14146f5e7:     	leaq	0x360(%rbp), %rcx
14146f5ee:     	leaq	0x5b0(%rbp), %r8
14146f5f5:     	callq	0x1408da280 <.text+0x8d9280>
14146f5fa:     	cmpl	$0xb, 0x360(%rbp)
14146f601:     	jne	0x14146f962 <.text+0x146e962>
14146f607:     	movq	0x830(%rbp), %rdx
14146f60e:     	testq	%rdx, %rdx
14146f611:     	je	0x14146f625 <.text+0x146e625>
14146f613:     	movq	0x838(%rbp), %rcx
14146f61a:     	movl	$0x1, %r8d
14146f620:     	callq	0x140613c20 <.text+0x612c20>
14146f625:     	movaps	0x980(%rbp), %xmm6
14146f62c:     	movaps	0x990(%rbp), %xmm7
14146f633:     	movaps	0x9a0(%rbp), %xmm8
14146f63b:     	movaps	0x9b0(%rbp), %xmm9
14146f643:     	movaps	0x9c0(%rbp), %xmm10
14146f64b:     	addq	$0xa58, %rsp            # imm = 0xA58
14146f652:     	popq	%rbx
14146f653:     	popq	%rdi
14146f654:     	popq	%rsi
14146f655:     	popq	%r12
14146f657:     	popq	%r13
14146f659:     	popq	%r14
14146f65b:     	popq	%r15
14146f65d:     	popq	%rbp
14146f65e:     	retq
14146f65f:     	movq	0x1b0(%rbp), %rdi
14146f666:     	movb	0x1bf(%rbp), %bl
14146f66c:     	leaq	-0x50(%rbp), %rcx
14146f670:     	callq	0x140d76ed0 <.text+0xd75ed0>
14146f675:     	movq	%rax, %rcx
14146f678:     	callq	0x140c96a80 <.text+0xc95a80>
14146f67d:     	testq	%rax, %rax
14146f680:     	je	0x14146f87f <.text+0x146e87f>
14146f686:     	leaq	0x17f4dc3(%rip), %rcx   # 0x142c64450
14146f68d:     	movq	%rcx, 0x360(%rbp)
14146f694:     	movq	$0x23, 0x368(%rbp)
14146f69f:     	movq	%rax, 0x370(%rbp)
14146f6a6:     	movq	$0x3, 0x5b0(%rbp)
14146f6b1:     	leaq	0x360(%rbp), %rcx
14146f6b8:     	leaq	0x5b0(%rbp), %rdx
14146f6bf:     	callq	0x1428a8a20 <.text+0x28a7a20>
14146f6c4:     	movq	%rax, %rdi
14146f6c7:     	leaq	0x1b0(%rbp), %rcx
14146f6ce:     	callq	0x14088fa00 <.text+0x88ea00>
14146f6d3:     	jmp	0x14146e18e <.text+0x146d18e>
14146f6d8:     	movq	0x218(%rbp), %rcx
14146f6df:     	testq	%rcx, %rcx
14146f6e2:     	je	0x14146f79e <.text+0x146e79e>
14146f6e8:     	movq	0x210(%rbp), %rax
14146f6ef:     	leaq	(%rcx,%rcx,4), %rcx
14146f6f3:     	leaq	(%rax,%rcx,8), %rcx
14146f6f7:     	leaq	0x17f55ce(%rip), %rdx   # 0x142c64ccc
14146f6fe:     	jmp	0x14146f70d <.text+0x146e70d>
14146f700:     	addq	$0x28, %rax
14146f704:     	cmpq	%rcx, %rax
14146f707:     	je	0x14146f79e <.text+0x146e79e>
14146f70d:     	movq	0x20(%rax), %r9
14146f711:     	testq	%r9, %r9
14146f714:     	je	0x14146f700 <.text+0x146e700>
14146f716:     	movq	0x18(%rax), %r8
14146f71a:     	imulq	$0x58, %r9, %r9
14146f71e:     	xorl	%r10d, %r10d
14146f721:     	jmp	0x14146f741 <.text+0x146e741>
14146f723:     	nopw	%cs:(%rax,%rax)
14146f730:     	movl	%esi, %r11d
14146f733:     	movl	%r11d, 0x30(%r8,%r10)
14146f738:     	addq	$0x58, %r10
14146f73c:     	cmpq	%r10, %r9
14146f73f:     	je	0x14146f700 <.text+0x146e700>
14146f741:     	movzbl	0x52(%r8,%r10), %r11d
14146f747:     	addb	$-0x38, %r11b
14146f74b:     	cmpb	$0x2, %r11b
14146f74f:     	ja	0x14146f738 <.text+0x146e738>
14146f751:     	movl	0x30(%r8,%r10), %esi
14146f756:     	cmpq	$0x8, %rsi
14146f75a:     	ja	0x14146f76e <.text+0x146e76e>
14146f75c:     	movslq	(%rdx,%rsi,4), %r11
14146f760:     	addq	%rdx, %r11
14146f763:     	jmpq	*%r11
14146f766:     	movl	$0x2, %r11d
14146f76c:     	jmp	0x14146f733 <.text+0x146e733>
14146f76e:     	movl	$0x1, %r11d
14146f774:     	cmpl	$0x3e7, %esi            # imm = 0x3E7
14146f77a:     	je	0x14146f730 <.text+0x146e730>
14146f77c:     	jmp	0x14146f733 <.text+0x146e733>
14146f77e:     	movl	$0x3, %r11d
14146f784:     	jmp	0x14146f733 <.text+0x146e733>
14146f786:     	movl	$0x5, %r11d
14146f78c:     	jmp	0x14146f733 <.text+0x146e733>
14146f78e:     	movl	$0x8, %r11d
14146f794:     	jmp	0x14146f733 <.text+0x146e733>
14146f796:     	movl	$0x4, %r11d
14146f79c:     	jmp	0x14146f733 <.text+0x146e733>
14146f79e:     	leaq	0x360(%rbp), %r12
14146f7a5:     	leaq	0x1b0(%rbp), %rdx
14146f7ac:     	movl	$0x1b0, %r8d            # imm = 0x1B0
14146f7b2:     	movq	%r12, %rcx
14146f7b5:     	callq	0x142923c00 <.text+0x2922c00>
14146f7ba:     	leaq	0x5b0(%rbp), %r14
14146f7c1:     	movl	$0x280, %r8d            # imm = 0x280
14146f7c7:     	movq	%r14, %rcx
14146f7ca:     	movq	%r15, %rdx
14146f7cd:     	callq	0x142923c00 <.text+0x2922c00>
14146f7d2:     	movb	$0x0, 0x973(%rbp)
14146f7d9:     	movq	%r12, %rcx
14146f7dc:     	movq	0x580(%rbp), %rdx
14146f7e3:     	movq	%r14, %r8
14146f7e6:     	callq	0x1417a9c50 <.text+0x17a8c50>
14146f7eb:     	btq	$0x28, 0x910(%rbp)
14146f7f4:     	movq	0x918(%rbp), %rax
14146f7fb:     	movl	%edi, 0x68(%rax)
14146f7fe:     	movq	%rax, %rdi
14146f801:     	jae	0x14146dfb4 <.text+0x146cfb4>
14146f807:     	movq	0x20(%rdi), %rax
14146f80b:     	movq	%rax, 0x928(%rbp)
14146f812:     	movq	0x28(%rdi), %rax
14146f816:     	movq	$0x0, 0x28(%rdi)
14146f81e:     	movq	%rax, 0x938(%rbp)
14146f825:     	testq	%rax, %rax
14146f828:     	je	0x14146dfb4 <.text+0x146cfb4>
14146f82e:     	movq	0x938(%rbp), %rsi
14146f835:     	negq	%rsi
14146f838:     	movl	$0x1, %ecx
14146f83d:     	movq	0x928(%rbp), %r15
14146f844:     	jmp	0x14146f865 <.text+0x146e865>
14146f846:     	movq	0x958(%rbp), %rcx
14146f84d:     	leaq	(%rsi,%rcx), %rax
14146f851:     	incq	%rax
14146f854:     	incq	%rcx
14146f857:     	addq	$0x10, %r15
14146f85b:     	cmpq	$0x1, %rax
14146f85f:     	je	0x14146dfb4 <.text+0x146cfb4>
14146f865:     	movq	%rcx, 0x958(%rbp)
14146f86c:     	movq	(%r15), %rax
14146f86f:     	lock
14146f870:     	decq	(%rax)
14146f873:     	jne	0x14146f846 <.text+0x146e846>
14146f875:     	movq	%r15, %rcx
14146f878:     	callq	0x1402e5d80 <.text+0x2e4d80>
14146f87d:     	jmp	0x14146f846 <.text+0x146e846>
14146f87f:     	movl	0x1b8(%rbp), %eax
14146f885:     	movl	0x1bb(%rbp), %ecx
14146f88b:     	movl	%ecx, 0x8f3(%rbp)
14146f891:     	movl	%eax, 0x8f0(%rbp)
14146f897:     	cmpb	$0x2, %bl
14146f89a:     	je	0x14146e18e <.text+0x146d18e>
14146f8a0:     	jmp	0x14146ea49 <.text+0x146da49>
14146f8a5:     	leaq	0x17f47e4(%rip), %rcx   # 0x142c64090
14146f8ac:     	leaq	0x17f4835(%rip), %r8    # 0x142c640e8
14146f8b3:     	movl	$0x58, %edx
14146f8b8:     	callq	0x1428d9430 <.text+0x28d8430>
14146f8bd:     	movq	0x5b8(%rbp), %rax
14146f8c4:     	movq	%rax, 0x368(%rbp)
14146f8cb:     	movq	$0x2, 0x360(%rbp)
14146f8d6:     	leaq	0x368(%rbp), %rcx
14146f8dd:     	callq	0x1405e7a00 <.text+0x5e6a00>
14146f8e2:     	leaq	0x518(%rbp), %rax
14146f8e9:     	movq	%rax, 0x360(%rbp)
14146f8f0:     	leaq	-0x1441007(%rip), %rax  # 0x14002e8f0 <.text+0x2d8f0>
14146f8f7:     	movq	%rax, 0x368(%rbp)
14146f8fe:     	leaq	0x17f471b(%rip), %rax   # 0x142c64020
14146f905:     	movq	%rax, 0x5b0(%rbp)
14146f90c:     	movq	$0x2, 0x5b8(%rbp)
14146f917:     	movq	$0x0, 0x5d0(%rbp)
14146f922:     	leaq	0x360(%rbp), %rax
14146f929:     	movq	%rax, 0x5c0(%rbp)
14146f930:     	movq	$0x1, 0x5c8(%rbp)
14146f93b:     	leaq	0x17f4736(%rip), %rdx   # 0x142c64078
14146f942:     	leaq	0x5b0(%rbp), %rcx
14146f949:     	callq	0x1428d9390 <.text+0x28d8390>
14146f94e:     	movl	$0x8, %ecx
14146f953:     	movl	$0xf0, %edx
14146f958:     	callq	0x1428d9000 <.text+0x28d8000>
14146f95d:     	jmp	0x14146fa70 <.text+0x146ea70>
14146f962:     	movups	0x360(%rbp), %xmm0
14146f969:     	movups	0x370(%rbp), %xmm1
14146f970:     	movups	0x380(%rbp), %xmm2
14146f977:     	movups	0x390(%rbp), %xmm3
14146f97e:     	movaps	%xmm3, 0x5e0(%rbp)
14146f985:     	movaps	%xmm2, 0x5d0(%rbp)
14146f98c:     	movaps	%xmm1, 0x5c0(%rbp)
14146f993:     	movaps	%xmm0, 0x5b0(%rbp)
14146f99a:     	leaq	0x17f4a97(%rip), %rax   # 0x142c64438
14146f9a1:     	movq	%rax, 0x20(%rsp)
14146f9a6:     	leaq	0x17f44d3(%rip), %rcx   # 0x142c63e80
14146f9ad:     	leaq	0x17f44ac(%rip), %r9    # 0x142c63e60
14146f9b4:     	leaq	0x5b0(%rbp), %r8
14146f9bb:     	movl	$0x2b, %edx
14146f9c0:     	callq	0x1428d9760 <.text+0x28d8760>
14146f9c5:     	jmp	0x14146fa70 <.text+0x146ea70>
14146f9ca:     	leaq	0x17c9177(%rip), %r8    # 0x142c38b48
14146f9d1:     	movq	%r13, %rcx
14146f9d4:     	movq	%r12, %rdx
14146f9d7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14146f9dc:     	jmp	0x14146fa70 <.text+0x146ea70>
14146f9e1:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146f9eb:     	movq	%rax, 0x930(%rbp)
14146f9f2:     	movb	$0x1, 0x976(%rbp)
14146f9f9:     	leaq	0x17f47d0(%rip), %rcx   # 0x142c641d0
14146fa00:     	leaq	0x17f47f1(%rip), %r8    # 0x142c641f8
14146fa07:     	movl	$0x28, %edx
14146fa0c:     	callq	0x1428d9430 <.text+0x28d8430>
14146fa11:     	jmp	0x14146fa70 <.text+0x146ea70>
14146fa13:     	leaq	0x17f495e(%rip), %rax   # 0x142c64378
14146fa1a:     	movq	%rax, 0x5b0(%rbp)
14146fa21:     	movq	$0x1, 0x5b8(%rbp)
14146fa2c:     	movq	$0x8, 0x5c0(%rbp)
14146fa37:     	xorps	%xmm0, %xmm0
14146fa3a:     	movups	%xmm0, 0x5c8(%rbp)
14146fa41:     	movb	$0x1, 0x975(%rbp)
14146fa48:     	movb	$0x1, 0x974(%rbp)
14146fa4f:     	leaq	0x17f4932(%rip), %rdx   # 0x142c64388
14146fa56:     	leaq	0x5b0(%rbp), %rcx
14146fa5d:     	callq	0x1428d9390 <.text+0x28d8390>
14146fa62:     	jmp	0x14146fa70 <.text+0x146ea70>
14146fa64:     	leaq	0x17f47a5(%rip), %rcx   # 0x142c64210
14146fa6b:     	callq	0x1428d9310 <.text+0x28d8310>
14146fa70:     	ud2
14146fa72:     	nopw	%cs:(%rax,%rax)
14146fa80:     	movq	%rdx, 0x10(%rsp)
14146fa85:     	pushq	%rbp
14146fa86:     	pushq	%r15
14146fa88:     	pushq	%r14
14146fa8a:     	pushq	%r13
14146fa8c:     	pushq	%r12
14146fa8e:     	pushq	%rsi
14146fa8f:     	pushq	%rdi
14146fa90:     	pushq	%rbx
14146fa91:     	subq	$0x78, %rsp
14146fa95:     	leaq	0x80(%rdx), %rbp
14146fa9c:     	movaps	%xmm10, 0x20(%rsp)
14146faa2:     	movaps	%xmm9, 0x30(%rsp)
14146faa8:     	movaps	%xmm8, 0x40(%rsp)
14146faae:     	movaps	%xmm7, 0x50(%rsp)
14146fab3:     	movaps	%xmm6, 0x60(%rsp)
14146fab8:     	movq	0x360(%rbp), %rdx
14146fabf:     	testq	%rdx, %rdx
14146fac2:     	je	0x14146fad6 <.text+0x146ead6>
14146fac4:     	movq	0x368(%rbp), %rcx
14146facb:     	movl	$0x1, %r8d
14146fad1:     	callq	0x140613c20 <.text+0x612c20>
14146fad6:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146fae0:     	movq	%rax, 0x930(%rbp)
14146fae7:     	movb	$0x1, 0x976(%rbp)
14146faee:     	movaps	0x60(%rsp), %xmm6
14146faf3:     	movaps	0x50(%rsp), %xmm7
14146faf8:     	movaps	0x40(%rsp), %xmm8
14146fafe:     	movaps	0x30(%rsp), %xmm9
14146fb04:     	movaps	0x20(%rsp), %xmm10
14146fb0a:     	addq	$0x78, %rsp
14146fb0e:     	popq	%rbx
14146fb0f:     	popq	%rdi
14146fb10:     	popq	%rsi
14146fb11:     	popq	%r12
14146fb13:     	popq	%r13
14146fb15:     	popq	%r14
14146fb17:     	popq	%r15
14146fb19:     	popq	%rbp
14146fb1a:     	retq
14146fb1b:     	nopl	(%rax,%rax)
14146fb20:     	movq	%rdx, 0x10(%rsp)
14146fb25:     	pushq	%rbp
14146fb26:     	pushq	%r15
14146fb28:     	pushq	%r14
14146fb2a:     	pushq	%r13
14146fb2c:     	pushq	%r12
14146fb2e:     	pushq	%rsi
14146fb2f:     	pushq	%rdi
14146fb30:     	pushq	%rbx
14146fb31:     	subq	$0x78, %rsp
14146fb35:     	leaq	0x80(%rdx), %rbp
14146fb3c:     	movaps	%xmm10, 0x20(%rsp)
14146fb42:     	movaps	%xmm9, 0x30(%rsp)
14146fb48:     	movaps	%xmm8, 0x40(%rsp)
14146fb4e:     	movaps	%xmm7, 0x50(%rsp)
14146fb53:     	movaps	%xmm6, 0x60(%rsp)
14146fb58:     	movq	0x1b0(%rbp), %rdx
14146fb5f:     	testq	%rdx, %rdx
14146fb62:     	je	0x14146fb76 <.text+0x146eb76>
14146fb64:     	movq	0x1b8(%rbp), %rcx
14146fb6b:     	movl	$0x1, %r8d
14146fb71:     	callq	0x140613c20 <.text+0x612c20>
14146fb76:     	movaps	0x60(%rsp), %xmm6
14146fb7b:     	movaps	0x50(%rsp), %xmm7
14146fb80:     	movaps	0x40(%rsp), %xmm8
14146fb86:     	movaps	0x30(%rsp), %xmm9
14146fb8c:     	movaps	0x20(%rsp), %xmm10
14146fb92:     	addq	$0x78, %rsp
14146fb96:     	popq	%rbx
14146fb97:     	popq	%rdi
14146fb98:     	popq	%rsi
14146fb99:     	popq	%r12
14146fb9b:     	popq	%r13
14146fb9d:     	popq	%r14
14146fb9f:     	popq	%r15
14146fba1:     	popq	%rbp
14146fba2:     	retq
14146fba3:     	nopw	%cs:(%rax,%rax)
14146fbb0:     	movq	%rdx, 0x10(%rsp)
14146fbb5:     	pushq	%rbp
14146fbb6:     	pushq	%r15
14146fbb8:     	pushq	%r14
14146fbba:     	pushq	%r13
14146fbbc:     	pushq	%r12
14146fbbe:     	pushq	%rsi
14146fbbf:     	pushq	%rdi
14146fbc0:     	pushq	%rbx
14146fbc1:     	subq	$0x78, %rsp
14146fbc5:     	leaq	0x80(%rdx), %rbp
14146fbcc:     	movaps	%xmm10, 0x20(%rsp)
14146fbd2:     	movaps	%xmm9, 0x30(%rsp)
14146fbd8:     	movaps	%xmm8, 0x40(%rsp)
14146fbde:     	movaps	%xmm7, 0x50(%rsp)
14146fbe3:     	movaps	%xmm6, 0x60(%rsp)
14146fbe8:     	movq	0x938(%rbp), %rcx
14146fbef:     	movq	0x948(%rbp), %rax
14146fbf6:     	movq	0x8(%rax), %rdx
14146fbfa:     	testq	%rdx, %rdx
14146fbfd:     	je	0x14146fc0f <.text+0x146ec0f>
14146fbff:     	movq	0x948(%rbp), %rax
14146fc06:     	movq	0x10(%rax), %r8
14146fc0a:     	callq	0x140613c20 <.text+0x612c20>
14146fc0f:     	movl	$0x18, %edx
14146fc14:     	movl	$0x8, %r8d
14146fc1a:     	movq	0x910(%rbp), %rcx
14146fc21:     	callq	0x140613c20 <.text+0x612c20>
14146fc26:     	movaps	0x60(%rsp), %xmm6
14146fc2b:     	movaps	0x50(%rsp), %xmm7
14146fc30:     	movaps	0x40(%rsp), %xmm8
14146fc36:     	movaps	0x30(%rsp), %xmm9
14146fc3c:     	movaps	0x20(%rsp), %xmm10
14146fc42:     	addq	$0x78, %rsp
14146fc46:     	popq	%rbx
14146fc47:     	popq	%rdi
14146fc48:     	popq	%rsi
14146fc49:     	popq	%r12
14146fc4b:     	popq	%r13
14146fc4d:     	popq	%r14
14146fc4f:     	popq	%r15
14146fc51:     	popq	%rbp
14146fc52:     	retq
14146fc53:     	nopw	%cs:(%rax,%rax)
14146fc60:     	movq	%rdx, 0x10(%rsp)
14146fc65:     	pushq	%rbp
14146fc66:     	pushq	%r15
14146fc68:     	pushq	%r14
14146fc6a:     	pushq	%r13
14146fc6c:     	pushq	%r12
14146fc6e:     	pushq	%rsi
14146fc6f:     	pushq	%rdi
14146fc70:     	pushq	%rbx
14146fc71:     	subq	$0x78, %rsp
14146fc75:     	leaq	0x80(%rdx), %rbp
14146fc7c:     	movaps	%xmm10, 0x20(%rsp)
14146fc82:     	movaps	%xmm9, 0x30(%rsp)
14146fc88:     	movaps	%xmm8, 0x40(%rsp)
14146fc8e:     	movaps	%xmm7, 0x50(%rsp)
14146fc93:     	movaps	%xmm6, 0x60(%rsp)
14146fc98:     	movq	-0x50(%rbp), %rdx
14146fc9c:     	testq	%rdx, %rdx
14146fc9f:     	je	0x14146fcb0 <.text+0x146ecb0>
14146fca1:     	movq	-0x48(%rbp), %rcx
14146fca5:     	movl	$0x1, %r8d
14146fcab:     	callq	0x140613c20 <.text+0x612c20>
14146fcb0:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14146fcba:     	movq	%rax, 0x930(%rbp)
14146fcc1:     	movb	$0x1, 0x976(%rbp)
14146fcc8:     	movaps	0x60(%rsp), %xmm6
14146fccd:     	movaps	0x50(%rsp), %xmm7
14146fcd2:     	movaps	0x40(%rsp), %xmm8
14146fcd8:     	movaps	0x30(%rsp), %xmm9
14146fcde:     	movaps	0x20(%rsp), %xmm10
14146fce4:     	addq	$0x78, %rsp
14146fce8:     	popq	%rbx
14146fce9:     	popq	%rdi
14146fcea:     	popq	%rsi
14146fceb:     	popq	%r12
14146fced:     	popq	%r13
14146fcef:     	popq	%r14
14146fcf1:     	popq	%r15
14146fcf3:     	popq	%rbp
14146fcf4:     	retq
14146fcf5:     	nopw	%cs:(%rax,%rax)
14146fd00:     	movq	%rdx, 0x10(%rsp)
14146fd05:     	pushq	%rbp
14146fd06:     	pushq	%r15
14146fd08:     	pushq	%r14
14146fd0a:     	pushq	%r13
14146fd0c:     	pushq	%r12
14146fd0e:     	pushq	%rsi
14146fd0f:     	pushq	%rdi
14146fd10:     	pushq	%rbx
14146fd11:     	subq	$0x78, %rsp
14146fd15:     	leaq	0x80(%rdx), %rbp
14146fd1c:     	movaps	%xmm10, 0x20(%rsp)
14146fd22:     	movaps	%xmm9, 0x30(%rsp)
14146fd28:     	movaps	%xmm8, 0x40(%rsp)
14146fd2e:     	movaps	%xmm7, 0x50(%rsp)
14146fd33:     	movaps	%xmm6, 0x60(%rsp)
14146fd38:     	leaq	0x5b0(%rbp), %rcx
14146fd3f:     	callq	0x14088fa00 <.text+0x88ea00>
14146fd44:     	movaps	0x60(%rsp), %xmm6
14146fd49:     	movaps	0x50(%rsp), %xmm7
14146fd4e:     	movaps	0x40(%rsp), %xmm8
14146fd54:     	movaps	0x30(%rsp), %xmm9
14146fd5a:     	movaps	0x20(%rsp), %xmm10
14146fd60:     	addq	$0x78, %rsp
14146fd64:     	popq	%rbx
14146fd65:     	popq	%rdi
14146fd66:     	popq	%rsi
14146fd67:     	popq	%r12
14146fd69:     	popq	%r13
14146fd6b:     	popq	%r14
14146fd6d:     	popq	%r15
14146fd6f:     	popq	%rbp
14146fd70:     	retq
14146fd71:     	nopw	%cs:(%rax,%rax)
14146fd80:     	movq	%rdx, 0x10(%rsp)
14146fd85:     	pushq	%rbp
14146fd86:     	pushq	%r15
14146fd88:     	pushq	%r14
14146fd8a:     	pushq	%r13
14146fd8c:     	pushq	%r12
14146fd8e:     	pushq	%rsi
14146fd8f:     	pushq	%rdi
14146fd90:     	pushq	%rbx
14146fd91:     	subq	$0x78, %rsp
14146fd95:     	leaq	0x80(%rdx), %rbp
14146fd9c:     	movaps	%xmm10, 0x20(%rsp)
14146fda2:     	movaps	%xmm9, 0x30(%rsp)
14146fda8:     	movaps	%xmm8, 0x40(%rsp)
14146fdae:     	movaps	%xmm7, 0x50(%rsp)
14146fdb3:     	movaps	%xmm6, 0x60(%rsp)
14146fdb8:     	leaq	0x868(%rbp), %rcx
14146fdbf:     	callq	0x140d4cc30 <.text+0xd4bc30>
14146fdc4:     	movaps	0x60(%rsp), %xmm6
14146fdc9:     	movaps	0x50(%rsp), %xmm7
14146fdce:     	movaps	0x40(%rsp), %xmm8
14146fdd4:     	movaps	0x30(%rsp), %xmm9
14146fdda:     	movaps	0x20(%rsp), %xmm10
14146fde0:     	addq	$0x78, %rsp
14146fde4:     	popq	%rbx
14146fde5:     	popq	%rdi
14146fde6:     	popq	%rsi
14146fde7:     	popq	%r12
14146fde9:     	popq	%r13
14146fdeb:     	popq	%r14
14146fded:     	popq	%r15
14146fdef:     	popq	%rbp
14146fdf0:     	retq
14146fdf1:     	nopw	%cs:(%rax,%rax)
14146fe00:     	movq	%rdx, 0x10(%rsp)
14146fe05:     	pushq	%rbp
14146fe06:     	pushq	%r15
14146fe08:     	pushq	%r14
14146fe0a:     	pushq	%r13
14146fe0c:     	pushq	%r12
14146fe0e:     	pushq	%rsi
14146fe0f:     	pushq	%rdi
14146fe10:     	pushq	%rbx
14146fe11:     	subq	$0x78, %rsp
14146fe15:     	leaq	0x80(%rdx), %rbp
14146fe1c:     	movaps	%xmm10, 0x20(%rsp)
14146fe22:     	movaps	%xmm9, 0x30(%rsp)
14146fe28:     	movaps	%xmm8, 0x40(%rsp)
14146fe2e:     	movaps	%xmm7, 0x50(%rsp)
14146fe33:     	movaps	%xmm6, 0x60(%rsp)
14146fe38:     	movq	0x880(%rbp), %rdx
14146fe3f:     	testq	%rdx, %rdx
14146fe42:     	je	0x14146fe56 <.text+0x146ee56>
14146fe44:     	movq	0x888(%rbp), %rcx
14146fe4b:     	movl	$0x1, %r8d
14146fe51:     	callq	0x140613c20 <.text+0x612c20>
14146fe56:     	movaps	0x60(%rsp), %xmm6
14146fe5b:     	movaps	0x50(%rsp), %xmm7
14146fe60:     	movaps	0x40(%rsp), %xmm8
14146fe66:     	movaps	0x30(%rsp), %xmm9
14146fe6c:     	movaps	0x20(%rsp), %xmm10
14146fe72:     	addq	$0x78, %rsp
14146fe76:     	popq	%rbx
14146fe77:     	popq	%rdi
14146fe78:     	popq	%rsi
14146fe79:     	popq	%r12
14146fe7b:     	popq	%r13
14146fe7d:     	popq	%r14
14146fe7f:     	popq	%r15
14146fe81:     	popq	%rbp
14146fe82:     	retq
14146fe83:     	nopw	%cs:(%rax,%rax)
14146fe90:     	movq	%rdx, 0x10(%rsp)
14146fe95:     	pushq	%rbp
14146fe96:     	pushq	%r15
14146fe98:     	pushq	%r14
14146fe9a:     	pushq	%r13
14146fe9c:     	pushq	%r12
14146fe9e:     	pushq	%rsi
14146fe9f:     	pushq	%rdi
14146fea0:     	pushq	%rbx
14146fea1:     	subq	$0x78, %rsp
14146fea5:     	leaq	0x80(%rdx), %rbp
14146feac:     	movaps	%xmm10, 0x20(%rsp)
14146feb2:     	movaps	%xmm9, 0x30(%rsp)
14146feb8:     	movaps	%xmm8, 0x40(%rsp)
14146febe:     	movaps	%xmm7, 0x50(%rsp)
14146fec3:     	movaps	%xmm6, 0x60(%rsp)
14146fec8:     	leaq	0x1b0(%rbp), %rcx
14146fecf:     	callq	0x14087b2f0 <.text+0x87a2f0>
14146fed4:     	movb	$0x1, 0x96e(%rbp)
14146fedb:     	movaps	0x60(%rsp), %xmm6
14146fee0:     	movaps	0x50(%rsp), %xmm7
14146fee5:     	movaps	0x40(%rsp), %xmm8
14146feeb:     	movaps	0x30(%rsp), %xmm9
14146fef1:     	movaps	0x20(%rsp), %xmm10
14146fef7:     	addq	$0x78, %rsp
14146fefb:     	popq	%rbx
14146fefc:     	popq	%rdi
14146fefd:     	popq	%rsi
14146fefe:     	popq	%r12
14146ff00:     	popq	%r13
14146ff02:     	popq	%r14
14146ff04:     	popq	%r15
14146ff06:     	popq	%rbp
14146ff07:     	retq
14146ff08:     	nopl	(%rax,%rax)
14146ff10:     	movq	%rdx, 0x10(%rsp)
14146ff15:     	pushq	%rbp
14146ff16:     	pushq	%r15
14146ff18:     	pushq	%r14
14146ff1a:     	pushq	%r13
14146ff1c:     	pushq	%r12
14146ff1e:     	pushq	%rsi
14146ff1f:     	pushq	%rdi
14146ff20:     	pushq	%rbx
14146ff21:     	subq	$0x78, %rsp
14146ff25:     	leaq	0x80(%rdx), %rbp
14146ff2c:     	movaps	%xmm10, 0x20(%rsp)
14146ff32:     	movaps	%xmm9, 0x30(%rsp)
14146ff38:     	movaps	%xmm8, 0x40(%rsp)
14146ff3e:     	movaps	%xmm7, 0x50(%rsp)
14146ff43:     	movaps	%xmm6, 0x60(%rsp)
14146ff48:     	leaq	0x5b0(%rbp), %rcx
14146ff4f:     	callq	0x141328890 <.text+0x1327890>
14146ff54:     	movaps	0x60(%rsp), %xmm6
14146ff59:     	movaps	0x50(%rsp), %xmm7
14146ff5e:     	movaps	0x40(%rsp), %xmm8
14146ff64:     	movaps	0x30(%rsp), %xmm9
14146ff6a:     	movaps	0x20(%rsp), %xmm10
14146ff70:     	addq	$0x78, %rsp
14146ff74:     	popq	%rbx
14146ff75:     	popq	%rdi
14146ff76:     	popq	%rsi
14146ff77:     	popq	%r12
14146ff79:     	popq	%r13
14146ff7b:     	popq	%r14
14146ff7d:     	popq	%r15
14146ff7f:     	popq	%rbp
14146ff80:     	retq
14146ff81:     	nopw	%cs:(%rax,%rax)
14146ff90:     	movq	%rdx, 0x10(%rsp)
14146ff95:     	pushq	%rbp
14146ff96:     	pushq	%r15
14146ff98:     	pushq	%r14
14146ff9a:     	pushq	%r13
14146ff9c:     	pushq	%r12
14146ff9e:     	pushq	%rsi
14146ff9f:     	pushq	%rdi
14146ffa0:     	pushq	%rbx
14146ffa1:     	subq	$0x78, %rsp
14146ffa5:     	leaq	0x80(%rdx), %rbp
14146ffac:     	movaps	%xmm10, 0x20(%rsp)
14146ffb2:     	movaps	%xmm9, 0x30(%rsp)
14146ffb8:     	movaps	%xmm8, 0x40(%rsp)
14146ffbe:     	movaps	%xmm7, 0x50(%rsp)
14146ffc3:     	movaps	%xmm6, 0x60(%rsp)
14146ffc8:     	leaq	0x5b0(%rbp), %rcx
14146ffcf:     	callq	0x141328890 <.text+0x1327890>
14146ffd4:     	movaps	0x60(%rsp), %xmm6
14146ffd9:     	movaps	0x50(%rsp), %xmm7
14146ffde:     	movaps	0x40(%rsp), %xmm8
14146ffe4:     	movaps	0x30(%rsp), %xmm9
14146ffea:     	movaps	0x20(%rsp), %xmm10
14146fff0:     	addq	$0x78, %rsp
14146fff4:     	popq	%rbx
14146fff5:     	popq	%rdi
14146fff6:     	popq	%rsi
14146fff7:     	popq	%r12
14146fff9:     	popq	%r13
14146fffb:     	popq	%r14
14146fffd:     	popq	%r15
14146ffff:     	popq	%rbp
141470000:     	retq
141470001:     	nopw	%cs:(%rax,%rax)
141470010:     	movq	%rdx, 0x10(%rsp)
141470015:     	pushq	%rbp
141470016:     	pushq	%r15
141470018:     	pushq	%r14
14147001a:     	pushq	%r13
14147001c:     	pushq	%r12
14147001e:     	pushq	%rsi
14147001f:     	pushq	%rdi
141470020:     	pushq	%rbx
141470021:     	subq	$0x78, %rsp
141470025:     	leaq	0x80(%rdx), %rbp
14147002c:     	movaps	%xmm10, 0x20(%rsp)
141470032:     	movaps	%xmm9, 0x30(%rsp)
141470038:     	movaps	%xmm8, 0x40(%rsp)
14147003e:     	movaps	%xmm7, 0x50(%rsp)
141470043:     	movaps	%xmm6, 0x60(%rsp)
141470048:     	cmpb	$0x0, 0x96e(%rbp)
14147004f:     	je	0x141470091 <.text+0x146f091>
141470051:     	leaq	0x360(%rbp), %rcx
141470058:     	callq	0x14088c400 <.text+0x88b400>
14147005d:     	movb	$0x0, 0x96f(%rbp)
141470064:     	movaps	0x60(%rsp), %xmm6
141470069:     	movaps	0x50(%rsp), %xmm7
14147006e:     	movaps	0x40(%rsp), %xmm8
141470074:     	movaps	0x30(%rsp), %xmm9
14147007a:     	movaps	0x20(%rsp), %xmm10
141470080:     	addq	$0x78, %rsp
141470084:     	popq	%rbx
141470085:     	popq	%rdi
141470086:     	popq	%rsi
141470087:     	popq	%r12
141470089:     	popq	%r13
14147008b:     	popq	%r14
14147008d:     	popq	%r15
14147008f:     	popq	%rbp
141470090:     	retq
141470091:     	movb	$0x0, 0x96f(%rbp)
141470098:     	movaps	0x60(%rsp), %xmm6
14147009d:     	movaps	0x50(%rsp), %xmm7
1414700a2:     	movaps	0x40(%rsp), %xmm8
1414700a8:     	movaps	0x30(%rsp), %xmm9
1414700ae:     	movaps	0x20(%rsp), %xmm10
1414700b4:     	addq	$0x78, %rsp
1414700b8:     	popq	%rbx
1414700b9:     	popq	%rdi
1414700ba:     	popq	%rsi
1414700bb:     	popq	%r12
1414700bd:     	popq	%r13
1414700bf:     	popq	%r14
1414700c1:     	popq	%r15
1414700c3:     	popq	%rbp
1414700c4:     	retq
1414700c5:     	nopw	%cs:(%rax,%rax)
1414700d0:     	movq	%rdx, 0x10(%rsp)
1414700d5:     	pushq	%rbp
1414700d6:     	pushq	%r15
1414700d8:     	pushq	%r14
1414700da:     	pushq	%r13
1414700dc:     	pushq	%r12
1414700de:     	pushq	%rsi
1414700df:     	pushq	%rdi
1414700e0:     	pushq	%rbx
1414700e1:     	subq	$0x78, %rsp
1414700e5:     	leaq	0x80(%rdx), %rbp
1414700ec:     	movaps	%xmm10, 0x20(%rsp)
1414700f2:     	movaps	%xmm9, 0x30(%rsp)
1414700f8:     	movaps	%xmm8, 0x40(%rsp)
1414700fe:     	movaps	%xmm7, 0x50(%rsp)
141470103:     	movaps	%xmm6, 0x60(%rsp)
141470108:     	cmpb	$0x0, 0x96f(%rbp)
14147010f:     	je	0x14147014a <.text+0x146f14a>
141470111:     	leaq	0x830(%rbp), %rcx
141470118:     	callq	0x140876a30 <.text+0x875a30>
14147011d:     	movaps	0x60(%rsp), %xmm6
141470122:     	movaps	0x50(%rsp), %xmm7
141470127:     	movaps	0x40(%rsp), %xmm8
14147012d:     	movaps	0x30(%rsp), %xmm9
141470133:     	movaps	0x20(%rsp), %xmm10
141470139:     	addq	$0x78, %rsp
14147013d:     	popq	%rbx
14147013e:     	popq	%rdi
14147013f:     	popq	%rsi
141470140:     	popq	%r12
141470142:     	popq	%r13
141470144:     	popq	%r14
141470146:     	popq	%r15
141470148:     	popq	%rbp
141470149:     	retq
14147014a:     	movaps	0x60(%rsp), %xmm6
14147014f:     	movaps	0x50(%rsp), %xmm7
141470154:     	movaps	0x40(%rsp), %xmm8
14147015a:     	movaps	0x30(%rsp), %xmm9
141470160:     	movaps	0x20(%rsp), %xmm10
141470166:     	addq	$0x78, %rsp
14147016a:     	popq	%rbx
14147016b:     	popq	%rdi
14147016c:     	popq	%rsi
14147016d:     	popq	%r12
14147016f:     	popq	%r13
141470171:     	popq	%r14
141470173:     	popq	%r15
141470175:     	popq	%rbp
141470176:     	retq
141470177:     	nopw	(%rax,%rax)
141470180:     	movq	%rdx, 0x10(%rsp)
141470185:     	pushq	%rbp
141470186:     	pushq	%r15
141470188:     	pushq	%r14
14147018a:     	pushq	%r13
14147018c:     	pushq	%r12
14147018e:     	pushq	%rsi
14147018f:     	pushq	%rdi
141470190:     	pushq	%rbx
141470191:     	subq	$0x78, %rsp
141470195:     	leaq	0x80(%rdx), %rbp
14147019c:     	movaps	%xmm10, 0x20(%rsp)
1414701a2:     	movaps	%xmm9, 0x30(%rsp)
1414701a8:     	movaps	%xmm8, 0x40(%rsp)
1414701ae:     	movaps	%xmm7, 0x50(%rsp)
1414701b3:     	movaps	%xmm6, 0x60(%rsp)
1414701b8:     	movzbl	0x970(%rbp), %r14d
1414701c0:     	movzbl	0x971(%rbp), %ebx
1414701c7:     	movq	0x8a0(%rbp), %rsi
1414701ce:     	movq	0x8d0(%rbp), %rdi
1414701d5:     	movq	0x830(%rbp), %rdx
1414701dc:     	testq	%rdx, %rdx
1414701df:     	je	0x1414701f3 <.text+0x146f1f3>
1414701e1:     	movq	0x838(%rbp), %rcx
1414701e8:     	movl	$0x1, %r8d
1414701ee:     	callq	0x140613c20 <.text+0x612c20>
1414701f3:     	testb	$0x1, %r14b
1414701f7:     	je	0x14147025f <.text+0x146f25f>
1414701f9:     	movq	0x948(%rbp), %r14
141470200:     	movq	%r14, %rcx
141470203:     	callq	0x14088eb40 <.text+0x88db40>
141470208:     	movl	$0xf0, %edx
14147020d:     	movl	$0x8, %r8d
141470213:     	movq	%r14, %rcx
141470216:     	callq	0x140613c20 <.text+0x612c20>
14147021b:     	movq	%rdi, 0x930(%rbp)
141470222:     	movq	%rsi, 0x548(%rbp)
141470229:     	andb	$0x1, %bl
14147022c:     	movb	%bl, 0x976(%rbp)
141470232:     	movaps	0x60(%rsp), %xmm6
141470237:     	movaps	0x50(%rsp), %xmm7
14147023c:     	movaps	0x40(%rsp), %xmm8
141470242:     	movaps	0x30(%rsp), %xmm9
141470248:     	movaps	0x20(%rsp), %xmm10
14147024e:     	addq	$0x78, %rsp
141470252:     	popq	%rbx
141470253:     	popq	%rdi
141470254:     	popq	%rsi
141470255:     	popq	%r12
141470257:     	popq	%r13
141470259:     	popq	%r14
14147025b:     	popq	%r15
14147025d:     	popq	%rbp
14147025e:     	retq
14147025f:     	movq	%rdi, 0x930(%rbp)
141470266:     	movq	%rsi, 0x548(%rbp)
14147026d:     	andb	$0x1, %bl
141470270:     	movb	%bl, 0x976(%rbp)
141470276:     	movaps	0x60(%rsp), %xmm6
14147027b:     	movaps	0x50(%rsp), %xmm7
141470280:     	movaps	0x40(%rsp), %xmm8
141470286:     	movaps	0x30(%rsp), %xmm9
14147028c:     	movaps	0x20(%rsp), %xmm10
141470292:     	addq	$0x78, %rsp
141470296:     	popq	%rbx
141470297:     	popq	%rdi
141470298:     	popq	%rsi
141470299:     	popq	%r12
14147029b:     	popq	%r13
14147029d:     	popq	%r14
14147029f:     	popq	%r15
1414702a1:     	popq	%rbp
1414702a2:     	retq
1414702a3:     	nopw	%cs:(%rax,%rax)
1414702b0:     	movq	%rdx, 0x10(%rsp)
1414702b5:     	pushq	%rbp
1414702b6:     	pushq	%r15
1414702b8:     	pushq	%r14
1414702ba:     	pushq	%r13
1414702bc:     	pushq	%r12
1414702be:     	pushq	%rsi
1414702bf:     	pushq	%rdi
1414702c0:     	pushq	%rbx
1414702c1:     	subq	$0x78, %rsp
1414702c5:     	leaq	0x80(%rdx), %rbp
1414702cc:     	movaps	%xmm10, 0x20(%rsp)
1414702d2:     	movaps	%xmm9, 0x30(%rsp)
1414702d8:     	movaps	%xmm8, 0x40(%rsp)
1414702de:     	movaps	%xmm7, 0x50(%rsp)
1414702e3:     	movaps	%xmm6, 0x60(%rsp)
1414702e8:     	movq	0x950(%rbp), %rdi
1414702ef:     	movq	%rdi, 0x940(%rbp)
1414702f6:     	movq	0x960(%rbp), %rsi
1414702fd:     	movq	%rsi, 0x920(%rbp)
141470304:     	movb	$0x0, 0x977(%rbp)
14147030b:     	leaq	0x1b0(%rbp), %rcx
141470312:     	callq	0x140894d60 <.text+0x893d60>
141470317:     	movq	%rdi, 0x940(%rbp)
14147031e:     	movq	%rsi, 0x920(%rbp)
141470325:     	movb	$0x0, 0x977(%rbp)
14147032c:     	movaps	0x60(%rsp), %xmm6
141470331:     	movaps	0x50(%rsp), %xmm7
141470336:     	movaps	0x40(%rsp), %xmm8
14147033c:     	movaps	0x30(%rsp), %xmm9
141470342:     	movaps	0x20(%rsp), %xmm10
141470348:     	addq	$0x78, %rsp
14147034c:     	popq	%rbx
14147034d:     	popq	%rdi
14147034e:     	popq	%rsi
14147034f:     	popq	%r12
141470351:     	popq	%r13
141470353:     	popq	%r14
141470355:     	popq	%r15
141470357:     	popq	%rbp
141470358:     	retq
141470359:     	nopl	(%rax)
141470360:     	movq	%rdx, 0x10(%rsp)
141470365:     	pushq	%rbp
141470366:     	pushq	%r15
141470368:     	pushq	%r14
14147036a:     	pushq	%r13
14147036c:     	pushq	%r12
14147036e:     	pushq	%rsi
14147036f:     	pushq	%rdi
141470370:     	pushq	%rbx
141470371:     	subq	$0x78, %rsp
141470375:     	leaq	0x80(%rdx), %rbp
14147037c:     	movaps	%xmm10, 0x20(%rsp)
141470382:     	movaps	%xmm9, 0x30(%rsp)
141470388:     	movaps	%xmm8, 0x40(%rsp)
14147038e:     	movaps	%xmm7, 0x50(%rsp)
141470393:     	movaps	%xmm6, 0x60(%rsp)
141470398:     	movaps	0x5b0(%rbp), %xmm0
14147039f:     	movaps	0x5c0(%rbp), %xmm1
1414703a6:     	movaps	0x5d0(%rbp), %xmm2
1414703ad:     	movq	0x918(%rbp), %rcx
1414703b4:     	movups	%xmm2, 0x20(%rcx)
1414703b8:     	movups	%xmm1, 0x10(%rcx)
1414703bc:     	movups	%xmm0, (%rcx)
1414703bf:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1414703c9:     	movq	%rax, 0x30(%rcx)
1414703cd:     	movq	0x8b0(%rbp), %rax
1414703d4:     	movq	%rax, 0x58(%rcx)
1414703d8:     	movq	0x8b8(%rbp), %rax
1414703df:     	movq	%rax, 0x60(%rcx)
1414703e3:     	movl	0x938(%rbp), %eax
1414703e9:     	movl	%eax, 0x68(%rcx)
1414703ec:     	movl	0x958(%rbp), %eax
1414703f2:     	movl	%eax, 0x6c(%rcx)
1414703f5:     	movl	0x90c(%rbp), %eax
1414703fb:     	movl	%eax, 0x70(%rcx)
1414703fe:     	movq	0x950(%rbp), %rax
141470405:     	movq	%rax, 0x940(%rbp)
14147040c:     	movq	0x960(%rbp), %rax
141470413:     	movq	%rax, 0x920(%rbp)
14147041a:     	movb	$0x0, 0x977(%rbp)
141470421:     	movaps	0x60(%rsp), %xmm6
141470426:     	movaps	0x50(%rsp), %xmm7
14147042b:     	movaps	0x40(%rsp), %xmm8
141470431:     	movaps	0x30(%rsp), %xmm9
141470437:     	movaps	0x20(%rsp), %xmm10
14147043d:     	addq	$0x78, %rsp
141470441:     	popq	%rbx
141470442:     	popq	%rdi
141470443:     	popq	%rsi
141470444:     	popq	%r12
141470446:     	popq	%r13
141470448:     	popq	%r14
14147044a:     	popq	%r15
14147044c:     	popq	%rbp
14147044d:     	retq
14147044e:     	nop
141470450:     	movq	%rdx, 0x10(%rsp)
141470455:     	pushq	%rbp
141470456:     	pushq	%r15
141470458:     	pushq	%r14
14147045a:     	pushq	%r13
14147045c:     	pushq	%r12
14147045e:     	pushq	%rsi
14147045f:     	pushq	%rdi
141470460:     	pushq	%rbx
141470461:     	subq	$0x78, %rsp
141470465:     	leaq	0x80(%rdx), %rbp
14147046c:     	movaps	%xmm10, 0x20(%rsp)
141470472:     	movaps	%xmm9, 0x30(%rsp)
141470478:     	movaps	%xmm8, 0x40(%rsp)
14147047e:     	movaps	%xmm7, 0x50(%rsp)
141470483:     	movaps	%xmm6, 0x60(%rsp)
141470488:     	movq	0x938(%rbp), %rdi
14147048f:     	movq	0x958(%rbp), %rbx
141470496:     	cmpq	%rdi, %rbx
141470499:     	movq	0x928(%rbp), %rsi
1414704a0:     	jne	0x1414704de <.text+0x146f4de>
1414704a2:     	movb	$0x0, 0x973(%rbp)
1414704a9:     	movaps	0x60(%rsp), %xmm6
1414704ae:     	movaps	0x50(%rsp), %xmm7
1414704b3:     	movaps	0x40(%rsp), %xmm8
1414704b9:     	movaps	0x30(%rsp), %xmm9
1414704bf:     	movaps	0x20(%rsp), %xmm10
1414704c5:     	addq	$0x78, %rsp
1414704c9:     	popq	%rbx
1414704ca:     	popq	%rdi
1414704cb:     	popq	%rsi
1414704cc:     	popq	%r12
1414704ce:     	popq	%r13
1414704d0:     	popq	%r14
1414704d2:     	popq	%r15
1414704d4:     	popq	%rbp
1414704d5:     	retq
1414704d6:     	incq	%rbx
1414704d9:     	cmpq	%rdi, %rbx
1414704dc:     	je	0x1414704a2 <.text+0x146f4a2>
1414704de:     	movq	%rbx, %rcx
1414704e1:     	shlq	$0x4, %rcx
1414704e5:     	movq	(%rsi,%rcx), %rax
1414704e9:     	lock
1414704ea:     	decq	(%rax)
1414704ed:     	jne	0x1414704d6 <.text+0x146f4d6>
1414704ef:     	addq	%rsi, %rcx
1414704f2:     	movb	$0x0, 0x973(%rbp)
1414704f9:     	callq	0x1402e5d80 <.text+0x2e4d80>
1414704fe:     	jmp	0x1414704d6 <.text+0x146f4d6>
141470500:     	movq	%rdx, 0x10(%rsp)
141470505:     	pushq	%rbp
141470506:     	pushq	%r15
141470508:     	pushq	%r14
14147050a:     	pushq	%r13
14147050c:     	pushq	%r12
14147050e:     	pushq	%rsi
14147050f:     	pushq	%rdi
141470510:     	pushq	%rbx
141470511:     	subq	$0x78, %rsp
141470515:     	leaq	0x80(%rdx), %rbp
14147051c:     	movaps	%xmm10, 0x20(%rsp)
141470522:     	movaps	%xmm9, 0x30(%rsp)
141470528:     	movaps	%xmm8, 0x40(%rsp)
14147052e:     	movaps	%xmm7, 0x50(%rsp)
141470533:     	movaps	%xmm6, 0x60(%rsp)
141470538:     	leaq	-0x50(%rbp), %rcx
14147053c:     	callq	0x140894d60 <.text+0x893d60>
141470541:     	movq	0x950(%rbp), %rax
141470548:     	movq	%rax, 0x940(%rbp)
14147054f:     	movq	0x960(%rbp), %rax
141470556:     	movq	%rax, 0x920(%rbp)
14147055d:     	movb	$0x0, 0x977(%rbp)
141470564:     	movaps	0x60(%rsp), %xmm6
141470569:     	movaps	0x50(%rsp), %xmm7
14147056e:     	movaps	0x40(%rsp), %xmm8
141470574:     	movaps	0x30(%rsp), %xmm9
14147057a:     	movaps	0x20(%rsp), %xmm10
141470580:     	addq	$0x78, %rsp
141470584:     	popq	%rbx
141470585:     	popq	%rdi
141470586:     	popq	%rsi
141470587:     	popq	%r12
141470589:     	popq	%r13
14147058b:     	popq	%r14
14147058d:     	popq	%r15
14147058f:     	popq	%rbp
141470590:     	retq
141470591:     	nopw	%cs:(%rax,%rax)
1414705a0:     	movq	%rdx, 0x10(%rsp)
1414705a5:     	pushq	%rbp
1414705a6:     	pushq	%r15
1414705a8:     	pushq	%r14
1414705aa:     	pushq	%r13
1414705ac:     	pushq	%r12
1414705ae:     	pushq	%rsi
1414705af:     	pushq	%rdi
1414705b0:     	pushq	%rbx
1414705b1:     	subq	$0x78, %rsp
1414705b5:     	leaq	0x80(%rdx), %rbp
1414705bc:     	movaps	%xmm10, 0x20(%rsp)
1414705c2:     	movaps	%xmm9, 0x30(%rsp)
1414705c8:     	movaps	%xmm8, 0x40(%rsp)
1414705ce:     	movaps	%xmm7, 0x50(%rsp)
1414705d3:     	movaps	%xmm6, 0x60(%rsp)
1414705d8:     	movq	0x1b0(%rbp), %rdx
1414705df:     	testq	%rdx, %rdx
1414705e2:     	je	0x1414705fa <.text+0x146f5fa>
1414705e4:     	movq	0x1b8(%rbp), %rcx
1414705eb:     	shlq	$0x4, %rdx
1414705ef:     	movl	$0x8, %r8d
1414705f5:     	callq	0x140613c20 <.text+0x612c20>
1414705fa:     	movaps	0x60(%rsp), %xmm6
1414705ff:     	movaps	0x50(%rsp), %xmm7
141470604:     	movaps	0x40(%rsp), %xmm8
14147060a:     	movaps	0x30(%rsp), %xmm9
141470610:     	movaps	0x20(%rsp), %xmm10
141470616:     	addq	$0x78, %rsp
14147061a:     	popq	%rbx
14147061b:     	popq	%rdi
14147061c:     	popq	%rsi
14147061d:     	popq	%r12
14147061f:     	popq	%r13
141470621:     	popq	%r14
141470623:     	popq	%r15
141470625:     	popq	%rbp
141470626:     	retq
141470627:     	nopw	(%rax,%rax)
141470630:     	movq	%rdx, 0x10(%rsp)
141470635:     	pushq	%rbp
141470636:     	pushq	%r15
141470638:     	pushq	%r14
14147063a:     	pushq	%r13
14147063c:     	pushq	%r12
14147063e:     	pushq	%rsi
14147063f:     	pushq	%rdi
141470640:     	pushq	%rbx
141470641:     	subq	$0x78, %rsp
141470645:     	leaq	0x80(%rdx), %rbp
14147064c:     	movaps	%xmm10, 0x20(%rsp)
141470652:     	movaps	%xmm9, 0x30(%rsp)
141470658:     	movaps	%xmm8, 0x40(%rsp)
14147065e:     	movaps	%xmm7, 0x50(%rsp)
141470663:     	movaps	%xmm6, 0x60(%rsp)
141470668:     	movq	0x950(%rbp), %rdi
14147066f:     	movq	%rdi, 0x940(%rbp)
141470676:     	movq	0x960(%rbp), %rsi
14147067d:     	movq	%rsi, 0x920(%rbp)
141470684:     	movb	$0x0, 0x977(%rbp)
14147068b:     	leaq	0x5b0(%rbp), %rcx
141470692:     	callq	0x14088eb40 <.text+0x88db40>
141470697:     	movq	%rdi, 0x940(%rbp)
14147069e:     	movq	%rsi, 0x920(%rbp)
1414706a5:     	movb	$0x0, 0x977(%rbp)
1414706ac:     	movaps	0x60(%rsp), %xmm6
1414706b1:     	movaps	0x50(%rsp), %xmm7
1414706b6:     	movaps	0x40(%rsp), %xmm8
1414706bc:     	movaps	0x30(%rsp), %xmm9
1414706c2:     	movaps	0x20(%rsp), %xmm10
1414706c8:     	addq	$0x78, %rsp
1414706cc:     	popq	%rbx
1414706cd:     	popq	%rdi
1414706ce:     	popq	%rsi
1414706cf:     	popq	%r12
1414706d1:     	popq	%r13
1414706d3:     	popq	%r14
1414706d5:     	popq	%r15
1414706d7:     	popq	%rbp
1414706d8:     	retq
1414706d9:     	nopl	(%rax)
1414706e0:     	movq	%rdx, 0x10(%rsp)
1414706e5:     	pushq	%rbp
1414706e6:     	pushq	%r15
1414706e8:     	pushq	%r14
1414706ea:     	pushq	%r13
1414706ec:     	pushq	%r12
1414706ee:     	pushq	%rsi
1414706ef:     	pushq	%rdi
1414706f0:     	pushq	%rbx
1414706f1:     	subq	$0x78, %rsp
1414706f5:     	leaq	0x80(%rdx), %rbp
1414706fc:     	movaps	%xmm10, 0x20(%rsp)
141470702:     	movaps	%xmm9, 0x30(%rsp)
141470708:     	movaps	%xmm8, 0x40(%rsp)
14147070e:     	movaps	%xmm7, 0x50(%rsp)
141470713:     	movaps	%xmm6, 0x60(%rsp)
141470718:     	movq	0x958(%rbp), %rcx
14147071f:     	callq	0x14146cb40 <.text+0x146bb40>
141470724:     	movq	0x950(%rbp), %rax
14147072b:     	movq	%rax, 0x940(%rbp)
141470732:     	movq	0x960(%rbp), %rax
141470739:     	movq	%rax, 0x920(%rbp)
141470740:     	movb	$0x0, 0x977(%rbp)
141470747:     	movaps	0x60(%rsp), %xmm6
14147074c:     	movaps	0x50(%rsp), %xmm7
141470751:     	movaps	0x40(%rsp), %xmm8
141470757:     	movaps	0x30(%rsp), %xmm9
14147075d:     	movaps	0x20(%rsp), %xmm10
141470763:     	addq	$0x78, %rsp
141470767:     	popq	%rbx
141470768:     	popq	%rdi
141470769:     	popq	%rsi
14147076a:     	popq	%r12
14147076c:     	popq	%r13
14147076e:     	popq	%r14
141470770:     	popq	%r15
141470772:     	popq	%rbp
141470773:     	retq
141470774:     	nopw	%cs:(%rax,%rax)
141470780:     	movq	%rdx, 0x10(%rsp)
141470785:     	pushq	%rbp
141470786:     	pushq	%r15
141470788:     	pushq	%r14
14147078a:     	pushq	%r13
14147078c:     	pushq	%r12
14147078e:     	pushq	%rsi
14147078f:     	pushq	%rdi
141470790:     	pushq	%rbx
141470791:     	subq	$0x78, %rsp
141470795:     	leaq	0x80(%rdx), %rbp
14147079c:     	movaps	%xmm10, 0x20(%rsp)
1414707a2:     	movaps	%xmm9, 0x30(%rsp)
1414707a8:     	movaps	%xmm8, 0x40(%rsp)
1414707ae:     	movaps	%xmm7, 0x50(%rsp)
1414707b3:     	movaps	%xmm6, 0x60(%rsp)
1414707b8:     	cmpb	$0x0, 0x973(%rbp)
1414707bf:     	je	0x14147081d <.text+0x146f81d>
1414707c1:     	leaq	0x1b0(%rbp), %rcx
1414707c8:     	callq	0x1408913b0 <.text+0x8903b0>
1414707cd:     	movq	0x950(%rbp), %rax
1414707d4:     	movq	%rax, 0x940(%rbp)
1414707db:     	movq	0x960(%rbp), %rax
1414707e2:     	movq	%rax, 0x920(%rbp)
1414707e9:     	movb	$0x0, 0x977(%rbp)
1414707f0:     	movaps	0x60(%rsp), %xmm6
1414707f5:     	movaps	0x50(%rsp), %xmm7
1414707fa:     	movaps	0x40(%rsp), %xmm8
141470800:     	movaps	0x30(%rsp), %xmm9
141470806:     	movaps	0x20(%rsp), %xmm10
14147080c:     	addq	$0x78, %rsp
141470810:     	popq	%rbx
141470811:     	popq	%rdi
141470812:     	popq	%rsi
141470813:     	popq	%r12
141470815:     	popq	%r13
141470817:     	popq	%r14
141470819:     	popq	%r15
14147081b:     	popq	%rbp
14147081c:     	retq
14147081d:     	movq	0x950(%rbp), %rax
141470824:     	movq	%rax, 0x940(%rbp)
14147082b:     	movq	0x960(%rbp), %rax
141470832:     	movq	%rax, 0x920(%rbp)
141470839:     	movb	$0x0, 0x977(%rbp)
141470840:     	movaps	0x60(%rsp), %xmm6
141470845:     	movaps	0x50(%rsp), %xmm7
14147084a:     	movaps	0x40(%rsp), %xmm8
141470850:     	movaps	0x30(%rsp), %xmm9
141470856:     	movaps	0x20(%rsp), %xmm10
14147085c:     	addq	$0x78, %rsp
141470860:     	popq	%rbx
141470861:     	popq	%rdi
141470862:     	popq	%rsi
141470863:     	popq	%r12
141470865:     	popq	%r13
141470867:     	popq	%r14
141470869:     	popq	%r15
14147086b:     	popq	%rbp
14147086c:     	retq
14147086d:     	nopl	(%rax)
141470870:     	movq	%rdx, 0x10(%rsp)
141470875:     	pushq	%rbp
141470876:     	pushq	%r15
141470878:     	pushq	%r14
14147087a:     	pushq	%r13
14147087c:     	pushq	%r12
14147087e:     	pushq	%rsi
14147087f:     	pushq	%rdi
141470880:     	pushq	%rbx
141470881:     	subq	$0x78, %rsp
141470885:     	leaq	0x80(%rdx), %rbp
14147088c:     	movaps	%xmm10, 0x20(%rsp)
141470892:     	movaps	%xmm9, 0x30(%rsp)
141470898:     	movaps	%xmm8, 0x40(%rsp)
14147089e:     	movaps	%xmm7, 0x50(%rsp)
1414708a3:     	movaps	%xmm6, 0x60(%rsp)
1414708a8:     	movq	0x920(%rbp), %rsi
1414708af:     	movq	0x940(%rbp), %rdi
1414708b6:     	cmpb	$0x0, 0x977(%rbp)
1414708bd:     	je	0x141470947 <.text+0x146f947>
1414708c3:     	movq	%rdi, 0x8d0(%rbp)
1414708ca:     	movq	%rsi, 0x8a0(%rbp)
1414708d1:     	movb	$0x1, 0x971(%rbp)
1414708d8:     	movb	$0x0, 0x970(%rbp)
1414708df:     	movq	0x948(%rbp), %rbx
1414708e6:     	movq	%rbx, %rcx
1414708e9:     	callq	0x140894d60 <.text+0x893d60>
1414708ee:     	movq	0x18(%rbx), %rdx
1414708f2:     	testq	%rdx, %rdx
1414708f5:     	je	0x141470911 <.text+0x146f911>
1414708f7:     	movq	0x948(%rbp), %rax
1414708fe:     	movq	0x20(%rax), %rcx
141470902:     	shlq	$0x4, %rdx
141470906:     	movl	$0x8, %r8d
14147090c:     	callq	0x140613c20 <.text+0x612c20>
141470911:     	movq	0x948(%rbp), %rax
141470918:     	movq	0x30(%rax), %rax
14147091c:     	leaq	(,%rax,2), %rcx
141470924:     	testq	%rcx, %rcx
141470927:     	je	0x141470947 <.text+0x146f947>
141470929:     	movq	0x948(%rbp), %rcx
141470930:     	movq	0x38(%rcx), %rcx
141470934:     	shlq	$0x3, %rax
141470938:     	leaq	(%rax,%rax,4), %rdx
14147093c:     	movl	$0x4, %r8d
141470942:     	callq	0x140613c20 <.text+0x612c20>
141470947:     	movl	$0xf0, %edx
14147094c:     	movl	$0x8, %r8d
141470952:     	movq	0x948(%rbp), %rcx
141470959:     	callq	0x140613c20 <.text+0x612c20>
14147095e:     	movq	%rdi, 0x8d0(%rbp)
141470965:     	movq	%rsi, 0x8a0(%rbp)
14147096c:     	movb	$0x1, 0x971(%rbp)
141470973:     	movb	$0x0, 0x970(%rbp)
14147097a:     	movaps	0x60(%rsp), %xmm6
14147097f:     	movaps	0x50(%rsp), %xmm7
141470984:     	movaps	0x40(%rsp), %xmm8
14147098a:     	movaps	0x30(%rsp), %xmm9
141470990:     	movaps	0x20(%rsp), %xmm10
141470996:     	addq	$0x78, %rsp
14147099a:     	popq	%rbx
14147099b:     	popq	%rdi
14147099c:     	popq	%rsi
14147099d:     	popq	%r12
14147099f:     	popq	%r13
1414709a1:     	popq	%r14
1414709a3:     	popq	%r15
1414709a5:     	popq	%rbp
1414709a6:     	retq
1414709a7:     	nopw	(%rax,%rax)
1414709b0:     	movq	%rdx, 0x10(%rsp)
1414709b5:     	pushq	%rbp
1414709b6:     	pushq	%r15
1414709b8:     	pushq	%r14
1414709ba:     	pushq	%r13
1414709bc:     	pushq	%r12
1414709be:     	pushq	%rsi
1414709bf:     	pushq	%rdi
1414709c0:     	pushq	%rbx
1414709c1:     	subq	$0x78, %rsp
1414709c5:     	leaq	0x80(%rdx), %rbp
1414709cc:     	movaps	%xmm10, 0x20(%rsp)
1414709d2:     	movaps	%xmm9, 0x30(%rsp)
1414709d8:     	movaps	%xmm8, 0x40(%rsp)
1414709de:     	movaps	%xmm7, 0x50(%rsp)
1414709e3:     	movaps	%xmm6, 0x60(%rsp)
1414709e8:     	movzbl	0x975(%rbp), %ebx
1414709ef:     	cmpb	$0x0, 0x974(%rbp)
1414709f6:     	je	0x141470a5d <.text+0x146fa5d>
1414709f8:     	movq	0x958(%rbp), %rcx
1414709ff:     	callq	0x14146cb40 <.text+0x146bb40>
141470a04:     	movq	0x950(%rbp), %rax
141470a0b:     	movq	%rax, 0x8d0(%rbp)
141470a12:     	movq	0x960(%rbp), %rax
141470a19:     	movq	%rax, 0x8a0(%rbp)
141470a20:     	andb	$0x1, %bl
141470a23:     	movb	%bl, 0x971(%rbp)
141470a29:     	movb	$0x0, 0x970(%rbp)
141470a30:     	movaps	0x60(%rsp), %xmm6
141470a35:     	movaps	0x50(%rsp), %xmm7
141470a3a:     	movaps	0x40(%rsp), %xmm8
141470a40:     	movaps	0x30(%rsp), %xmm9
141470a46:     	movaps	0x20(%rsp), %xmm10
141470a4c:     	addq	$0x78, %rsp
141470a50:     	popq	%rbx
141470a51:     	popq	%rdi
141470a52:     	popq	%rsi
141470a53:     	popq	%r12
141470a55:     	popq	%r13
141470a57:     	popq	%r14
141470a59:     	popq	%r15
141470a5b:     	popq	%rbp
141470a5c:     	retq
141470a5d:     	movq	0x950(%rbp), %rax
141470a64:     	movq	%rax, 0x8d0(%rbp)
141470a6b:     	movq	0x960(%rbp), %rax
141470a72:     	movq	%rax, 0x8a0(%rbp)
141470a79:     	andb	$0x1, %bl
141470a7c:     	movb	%bl, 0x971(%rbp)
141470a82:     	movb	$0x0, 0x970(%rbp)
141470a89:     	movaps	0x60(%rsp), %xmm6
141470a8e:     	movaps	0x50(%rsp), %xmm7
141470a93:     	movaps	0x40(%rsp), %xmm8
141470a99:     	movaps	0x30(%rsp), %xmm9
141470a9f:     	movaps	0x20(%rsp), %xmm10
141470aa5:     	addq	$0x78, %rsp
141470aa9:     	popq	%rbx
141470aaa:     	popq	%rdi
141470aab:     	popq	%rsi
141470aac:     	popq	%r12
141470aae:     	popq	%r13
141470ab0:     	popq	%r14
141470ab2:     	popq	%r15
141470ab4:     	popq	%rbp
141470ab5:     	retq
141470ab6:     	nopw	%cs:(%rax,%rax)
141470ac0:     	movq	%rdx, 0x10(%rsp)
141470ac5:     	pushq	%rbp
141470ac6:     	pushq	%r15
141470ac8:     	pushq	%r14
141470aca:     	pushq	%r13
141470acc:     	pushq	%r12
141470ace:     	pushq	%rsi
141470acf:     	pushq	%rdi
141470ad0:     	pushq	%rbx
141470ad1:     	subq	$0x78, %rsp
141470ad5:     	leaq	0x80(%rdx), %rbp
141470adc:     	movaps	%xmm10, 0x20(%rsp)
141470ae2:     	movaps	%xmm9, 0x30(%rsp)
141470ae8:     	movaps	%xmm8, 0x40(%rsp)
141470aee:     	movaps	%xmm7, 0x50(%rsp)
141470af3:     	movaps	%xmm6, 0x60(%rsp)
141470af8:     	movq	0x930(%rbp), %rax
141470aff:     	leaq	(,%rax,2), %rcx
141470b07:     	testq	%rcx, %rcx
141470b0a:     	setne	%cl
141470b0d:     	testb	%cl, 0x976(%rbp)
141470b13:     	je	0x141470b2f <.text+0x146fb2f>
141470b15:     	movq	0x548(%rbp), %rcx
141470b1c:     	shlq	$0x3, %rax
141470b20:     	leaq	(%rax,%rax,4), %rdx
141470b24:     	movl	$0x4, %r8d
141470b2a:     	callq	0x140613c20 <.text+0x612c20>
141470b2f:     	movaps	0x60(%rsp), %xmm6
141470b34:     	movaps	0x50(%rsp), %xmm7
141470b39:     	movaps	0x40(%rsp), %xmm8
141470b3f:     	movaps	0x30(%rsp), %xmm9
141470b45:     	movaps	0x20(%rsp), %xmm10
141470b4b:     	addq	$0x78, %rsp
141470b4f:     	popq	%rbx
141470b50:     	popq	%rdi
141470b51:     	popq	%rsi
141470b52:     	popq	%r12
141470b54:     	popq	%r13
141470b56:     	popq	%r14
141470b58:     	popq	%r15
141470b5a:     	popq	%rbp
141470b5b:     	retq
141470b5c:     	nopl	(%rax)
141470b60:     	movq	%rdx, 0x10(%rsp)
141470b65:     	pushq	%rbp
141470b66:     	pushq	%r15
141470b68:     	pushq	%r14
141470b6a:     	pushq	%r13
141470b6c:     	pushq	%r12
141470b6e:     	pushq	%rsi
141470b6f:     	pushq	%rdi
141470b70:     	pushq	%rbx
141470b71:     	subq	$0x78, %rsp
141470b75:     	leaq	0x80(%rdx), %rbp
141470b7c:     	movaps	%xmm10, 0x20(%rsp)
141470b82:     	movaps	%xmm9, 0x30(%rsp)
141470b88:     	movaps	%xmm8, 0x40(%rsp)
141470b8e:     	movaps	%xmm7, 0x50(%rsp)
141470b93:     	movaps	%xmm6, 0x60(%rsp)
141470b98:     	leaq	0x1b0(%rbp), %rcx
141470b9f:     	callq	0x14088fa00 <.text+0x88ea00>
141470ba4:     	movaps	0x60(%rsp), %xmm6
141470ba9:     	movaps	0x50(%rsp), %xmm7
141470bae:     	movaps	0x40(%rsp), %xmm8
141470bb4:     	movaps	0x30(%rsp), %xmm9
141470bba:     	movaps	0x20(%rsp), %xmm10
141470bc0:     	addq	$0x78, %rsp
141470bc4:     	popq	%rbx
141470bc5:     	popq	%rdi
141470bc6:     	popq	%rsi
141470bc7:     	popq	%r12
141470bc9:     	popq	%r13
141470bcb:     	popq	%r14
141470bcd:     	popq	%r15
141470bcf:     	popq	%rbp
141470bd0:     	retq
141470bd1:     	nopw	%cs:(%rax,%rax)
141470be0:     	movq	%rdx, 0x10(%rsp)
141470be5:     	pushq	%rbp
141470be6:     	pushq	%r15
141470be8:     	pushq	%r14
141470bea:     	pushq	%r13
141470bec:     	pushq	%r12
141470bee:     	pushq	%rsi
141470bef:     	pushq	%rdi
141470bf0:     	pushq	%rbx
141470bf1:     	subq	$0x78, %rsp
141470bf5:     	leaq	0x80(%rdx), %rbp
141470bfc:     	movaps	%xmm10, 0x20(%rsp)
141470c02:     	movaps	%xmm9, 0x30(%rsp)
141470c08:     	movaps	%xmm8, 0x40(%rsp)
141470c0e:     	movaps	%xmm7, 0x50(%rsp)
141470c13:     	movaps	%xmm6, 0x60(%rsp)
141470c18:     	movb	$0x0, 0x975(%rbp)
141470c1f:     	movb	$0x0, 0x974(%rbp)
141470c26:     	leaq	0x5b0(%rbp), %rcx
141470c2d:     	callq	0x14088e590 <.text+0x88d590>
141470c32:     	movb	$0x0, 0x975(%rbp)
141470c39:     	movb	$0x0, 0x974(%rbp)
141470c40:     	movaps	0x60(%rsp), %xmm6
141470c45:     	movaps	0x50(%rsp), %xmm7
141470c4a:     	movaps	0x40(%rsp), %xmm8
141470c50:     	movaps	0x30(%rsp), %xmm9
141470c56:     	movaps	0x20(%rsp), %xmm10
141470c5c:     	addq	$0x78, %rsp
141470c60:     	popq	%rbx
141470c61:     	popq	%rdi
141470c62:     	popq	%rsi
141470c63:     	popq	%r12
141470c65:     	popq	%r13
141470c67:     	popq	%r14
141470c69:     	popq	%r15
141470c6b:     	popq	%rbp
141470c6c:     	retq
141470c6d:     	nopl	(%rax)
141470c70:     	movq	%rdx, 0x10(%rsp)
141470c75:     	pushq	%rbp
141470c76:     	pushq	%r15
141470c78:     	pushq	%r14
141470c7a:     	pushq	%r13
141470c7c:     	pushq	%r12
141470c7e:     	pushq	%rsi
141470c7f:     	pushq	%rdi
141470c80:     	pushq	%rbx
141470c81:     	subq	$0x78, %rsp
141470c85:     	leaq	0x80(%rdx), %rbp
141470c8c:     	movaps	%xmm10, 0x20(%rsp)
141470c92:     	movaps	%xmm9, 0x30(%rsp)
141470c98:     	movaps	%xmm8, 0x40(%rsp)
141470c9e:     	movaps	%xmm7, 0x50(%rsp)
141470ca3:     	movaps	%xmm6, 0x60(%rsp)
141470ca8:     	leaq	0x360(%rbp), %rcx
141470caf:     	callq	0x14087b2f0 <.text+0x87a2f0>
141470cb4:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141470cbe:     	movq	%rax, 0x930(%rbp)
141470cc5:     	movb	$0x1, 0x976(%rbp)
141470ccc:     	movaps	0x60(%rsp), %xmm6
141470cd1:     	movaps	0x50(%rsp), %xmm7
141470cd6:     	movaps	0x40(%rsp), %xmm8
141470cdc:     	movaps	0x30(%rsp), %xmm9
141470ce2:     	movaps	0x20(%rsp), %xmm10
141470ce8:     	addq	$0x78, %rsp
141470cec:     	popq	%rbx
141470ced:     	popq	%rdi
141470cee:     	popq	%rsi
141470cef:     	popq	%r12
141470cf1:     	popq	%r13
141470cf3:     	popq	%r14
141470cf5:     	popq	%r15
141470cf7:     	popq	%rbp
141470cf8:     	retq
