
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407dc170:     	pushq	%rbp
1407dc171:     	pushq	%r15
1407dc173:     	pushq	%r14
1407dc175:     	pushq	%r13
1407dc177:     	pushq	%r12
1407dc179:     	pushq	%rsi
1407dc17a:     	pushq	%rdi
1407dc17b:     	pushq	%rbx
1407dc17c:     	subq	$0x3c8, %rsp            # imm = 0x3C8
1407dc183:     	leaq	0x80(%rsp), %rbp
1407dc18b:     	movaps	%xmm15, 0x330(%rbp)
1407dc193:     	movaps	%xmm14, 0x320(%rbp)
1407dc19b:     	movaps	%xmm13, 0x310(%rbp)
1407dc1a3:     	movaps	%xmm12, 0x300(%rbp)
1407dc1ab:     	movaps	%xmm11, 0x2f0(%rbp)
1407dc1b3:     	movaps	%xmm10, 0x2e0(%rbp)
1407dc1bb:     	movaps	%xmm9, 0x2d0(%rbp)
1407dc1c3:     	movaps	%xmm8, 0x2c0(%rbp)
1407dc1cb:     	movaps	%xmm7, 0x2b0(%rbp)
1407dc1d2:     	movaps	%xmm6, 0x2a0(%rbp)
1407dc1d9:     	movq	$-0x2, 0x298(%rbp)
1407dc1e4:     	movq	%r9, 0xb0(%rbp)
1407dc1eb:     	movq	%r8, 0xb8(%rbp)
1407dc1f2:     	movq	%rdx, 0x18(%rbp)
1407dc1f6:     	movq	0x3e0(%rbp), %rdx
1407dc1fd:     	xorps	%xmm0, %xmm0
1407dc200:     	cmpl	$0x1, (%rdx)
1407dc203:     	xorps	%xmm1, %xmm1
1407dc206:     	jne	0x1407dc21c <.text+0x7db21c>
1407dc208:     	movss	0x2149710(%rip), %xmm1  # 0x142925920
1407dc210:     	subss	0x4(%rdx), %xmm1
1407dc215:     	xorps	%xmm2, %xmm2
1407dc218:     	maxss	%xmm2, %xmm1
1407dc21c:     	movq	0x3c0(%rbp), %rax
1407dc223:     	movss	%xmm1, 0x128(%rbp)
1407dc22b:     	cmpl	$0x1, 0x8(%rdx)
1407dc22f:     	jne	0x1407dc245 <.text+0x7db245>
1407dc231:     	movss	0x21496e7(%rip), %xmm0  # 0x142925920
1407dc239:     	subss	0xc(%rdx), %xmm0
1407dc23e:     	xorps	%xmm1, %xmm1
1407dc241:     	maxss	%xmm1, %xmm0
1407dc245:     	movq	0x3b8(%rbp), %r9
1407dc24c:     	movss	%xmm0, 0x124(%rbp)
1407dc254:     	movq	%rax, %rdx
1407dc257:     	decq	%rdx
1407dc25a:     	movq	%rcx, 0x28(%rbp)
1407dc25e:     	js	0x1407dc26a <.text+0x7db26a>
1407dc260:     	xorps	%xmm0, %xmm0
1407dc263:     	cvtsi2ss	%rdx, %xmm0
1407dc268:     	jmp	0x1407dc282 <.text+0x7db282>
1407dc26a:     	movq	%rdx, %r8
1407dc26d:     	shrq	%r8
1407dc270:     	andl	$0x1, %edx
1407dc273:     	orq	%r8, %rdx
1407dc276:     	xorps	%xmm0, %xmm0
1407dc279:     	cvtsi2ss	%rdx, %xmm0
1407dc27e:     	addss	%xmm0, %xmm0
1407dc282:     	movss	0x2149679(%rip), %xmm12 # 0x142925904
1407dc28b:     	movaps	%xmm12, %xmm1
1407dc28f:     	divss	%xmm0, %xmm1
1407dc293:     	movss	%xmm1, 0x120(%rbp)
1407dc29b:     	leaq	(%rax,%rax,2), %rax
1407dc29f:     	leaq	(%r9,%rax,4), %rax
1407dc2a3:     	movq	%r9, 0x140(%rbp)
1407dc2aa:     	movq	%rax, 0x148(%rbp)
1407dc2b1:     	movq	$0x0, 0x150(%rbp)
1407dc2bc:     	leaq	0x128(%rbp), %rax
1407dc2c3:     	movq	%rax, 0x158(%rbp)
1407dc2ca:     	leaq	0x124(%rbp), %rax
1407dc2d1:     	movq	%rax, 0x160(%rbp)
1407dc2d8:     	leaq	0x120(%rbp), %rax
1407dc2df:     	movq	%rax, 0x168(%rbp)
1407dc2e6:     	leaq	0x244d3d3(%rip), %r8    # 0x142c296c0
1407dc2ed:     	leaq	-0x10(%rbp), %rcx
1407dc2f1:     	leaq	0x140(%rbp), %rdx
1407dc2f8:     	callq	0x141262530 <.text+0x1261530>
1407dc2fd:     	movq	(%rbp), %rsi
1407dc301:     	testq	%rsi, %rsi
1407dc304:     	movq	%rsi, 0x210(%rbp)
1407dc30b:     	je	0x1407ddc6a <.text+0x7dcc6a>
1407dc311:     	leaq	(,%rsi,4), %rax
1407dc319:     	movq	%rax, 0x1d0(%rbp)
1407dc320:     	leaq	(%rax,%rax,2), %rcx
1407dc324:     	movzbl	0x2e588de(%rip), %eax   # 0x143634c09
1407dc32b:     	movl	$0x4, %edx
1407dc330:     	movq	%rcx, 0x200(%rbp)
1407dc337:     	callq	0x140613c10 <.text+0x612c10>
1407dc33c:     	testq	%rax, %rax
1407dc33f:     	je	0x1407ddca8 <.text+0x7dcca8>
1407dc345:     	movq	%rsi, 0x140(%rbp)
1407dc34c:     	movq	%rax, 0x148(%rbp)
1407dc353:     	movq	$0x0, 0x150(%rbp)
1407dc35e:     	cmpq	$0x1, %rsi
1407dc362:     	je	0x1407ddc8b <.text+0x7dcc8b>
1407dc368:     	movq	-0x8(%rbp), %rax
1407dc36c:     	movss	0x10(%rax), %xmm8
1407dc372:     	movss	0x4(%rax), %xmm7
1407dc377:     	movss	0xc(%rax), %xmm0
1407dc37c:     	movss	0x14(%rax), %xmm9
1407dc382:     	unpcklps	%xmm0, %xmm9            # xmm9 = xmm9[0],xmm0[0],xmm9[1],xmm0[1]
1407dc386:     	movss	(%rax), %xmm0
1407dc38a:     	movq	%rax, 0x1c8(%rbp)
1407dc391:     	movss	0x8(%rax), %xmm1
1407dc396:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407dc399:     	subps	%xmm1, %xmm9
1407dc39d:     	movshdup	%xmm9, %xmm6            # xmm6 = xmm9[1,1,3,3]
1407dc3a2:     	movaps	0x2149547(%rip), %xmm0  # 0x1429258f0
1407dc3a9:     	xorps	%xmm9, %xmm0
1407dc3ad:     	movss	%xmm0, 0x30(%rbp)
1407dc3b2:     	movss	%xmm6, 0x34(%rbp)
1407dc3b7:     	leaq	0xd0(%rbp), %rcx
1407dc3be:     	leaq	0x30(%rbp), %rdx
1407dc3c2:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407dc3c7:     	subss	%xmm7, %xmm8
1407dc3cc:     	movss	0xd0(%rbp), %xmm7
1407dc3d4:     	movsd	0xd4(%rbp), %xmm0
1407dc3dc:     	movsldup	%xmm9, %xmm1            # xmm1 = xmm9[0,0,2,2]
1407dc3e1:     	movss	%xmm8, %xmm1            # xmm1 = xmm8[0],xmm1[1,2,3]
1407dc3e6:     	mulss	%xmm7, %xmm8
1407dc3eb:     	shufps	$0xd4, %xmm0, %xmm7     # xmm7 = xmm7[0,1],xmm0[1,3]
1407dc3ef:     	shufps	$0xf2, %xmm0, %xmm7     # xmm7 = xmm7[2,0],xmm0[3,3]
1407dc3f3:     	mulps	%xmm1, %xmm7
1407dc3f6:     	mulps	%xmm0, %xmm9
1407dc3fa:     	subps	%xmm9, %xmm7
1407dc3fe:     	mulss	%xmm0, %xmm6
1407dc402:     	subss	%xmm8, %xmm6
1407dc407:     	movaps	%xmm7, %xmm0
1407dc40a:     	mulps	%xmm7, %xmm0
1407dc40d:     	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407dc411:     	addss	%xmm0, %xmm1
1407dc415:     	movaps	%xmm6, %xmm0
1407dc418:     	mulss	%xmm6, %xmm0
1407dc41c:     	addss	%xmm1, %xmm0
1407dc420:     	sqrtss	%xmm0, %xmm0
1407dc424:     	movaps	%xmm12, %xmm8
1407dc428:     	divss	%xmm0, %xmm8
1407dc42d:     	movd	%xmm8, %eax
1407dc432:     	testl	%eax, %eax
1407dc434:     	setns	%cl
1407dc437:     	movl	%eax, %edx
1407dc439:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407dc43f:     	addl	$0xff800000, %edx       # imm = 0xFF800000
1407dc445:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407dc44b:     	setb	%dl
1407dc44e:     	andb	%cl, %dl
1407dc450:     	decl	%eax
1407dc452:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407dc457:     	setb	%bl
1407dc45a:     	orb	%dl, %bl
1407dc45c:     	movaps	0x214948c(%rip), %xmm13 # 0x1429258f0
1407dc464:     	jne	0x1407dc480 <.text+0x7db480>
1407dc466:     	movss	0x215822a(%rip), %xmm6  # 0x142934698
1407dc46e:     	movq	0x150(%rbp), %rsi
1407dc475:     	cmpq	0x140(%rbp), %rsi
1407dc47c:     	je	0x1407dc499 <.text+0x7db499>
1407dc47e:     	jmp	0x1407dc4ac <.text+0x7db4ac>
1407dc480:     	xorps	%xmm13, %xmm6
1407dc484:     	mulss	%xmm8, %xmm6
1407dc489:     	movq	0x150(%rbp), %rsi
1407dc490:     	cmpq	0x140(%rbp), %rsi
1407dc497:     	jne	0x1407dc4ac <.text+0x7db4ac>
1407dc499:     	leaq	0x22abfb0(%rip), %rdx   # 0x142a88450
1407dc4a0:     	leaq	0x140(%rbp), %rcx
1407dc4a7:     	callq	0x140b9f180 <.text+0xb9e180>
1407dc4ac:     	movzbl	%bl, %eax
1407dc4af:     	movd	%eax, %xmm0
1407dc4b3:     	pshufd	$0x50, %xmm0, %xmm0     # xmm0 = xmm0[0,0,1,1]
1407dc4b8:     	xorps	%xmm13, %xmm7
1407dc4bc:     	movsldup	%xmm8, %xmm1            # xmm1 = xmm8[0,0,2,2]
1407dc4c1:     	mulps	%xmm7, %xmm1
1407dc4c4:     	pslld	$0x1f, %xmm0
1407dc4c9:     	psrad	$0x1f, %xmm0
1407dc4ce:     	andps	%xmm0, %xmm1
1407dc4d1:     	andnps	0x21f0e58(%rip), %xmm0  # 0x1429cd330
1407dc4d8:     	orps	%xmm1, %xmm0
1407dc4db:     	leaq	0xf8(%rbp), %rax
1407dc4e2:     	movq	%rax, 0xc8(%rbp)
1407dc4e9:     	movq	0x148(%rbp), %rax
1407dc4f0:     	leaq	(%rsi,%rsi,2), %rcx
1407dc4f4:     	movlps	%xmm0, (%rax,%rcx,4)
1407dc4f8:     	movss	%xmm6, 0x8(%rax,%rcx,4)
1407dc4fe:     	incq	%rsi
1407dc501:     	movq	%rsi, 0x150(%rbp)
1407dc508:     	movq	0x210(%rbp), %rax
1407dc50f:     	leaq	(%rax,%rax,2), %rax
1407dc513:     	movq	0x1c8(%rbp), %rdi
1407dc51a:     	leaq	(%rdi,%rax,4), %rax
1407dc51e:     	movq	%rax, 0x248(%rbp)
1407dc525:     	movb	$0x1, %al
1407dc527:     	leaq	0xd0(%rbp), %r12
1407dc52e:     	leaq	0x30(%rbp), %r13
1407dc532:     	movaps	0x21f0df7(%rip), %xmm6  # 0x1429cd330
1407dc539:     	movss	0x2158157(%rip), %xmm7  # 0x142934698
1407dc541:     	jmp	0x1407dc5a2 <.text+0x7db5a2>
1407dc543:     	nopw	%cs:(%rax,%rax)
1407dc550:     	xorps	%xmm13, %xmm9
1407dc554:     	movsldup	%xmm10, %xmm0           # xmm0 = xmm10[0,0,2,2]
1407dc559:     	mulps	%xmm9, %xmm0
1407dc55d:     	movzbl	%sil, %eax
1407dc561:     	movd	%eax, %xmm1
1407dc565:     	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
1407dc56a:     	pslld	$0x1f, %xmm1
1407dc56f:     	psrad	$0x1f, %xmm1
1407dc574:     	andps	%xmm1, %xmm0
1407dc577:     	andnps	%xmm6, %xmm1
1407dc57a:     	orps	%xmm0, %xmm1
1407dc57d:     	movq	0x148(%rbp), %rax
1407dc584:     	leaq	(%r15,%r15,2), %rcx
1407dc588:     	movlps	%xmm1, (%rax,%rcx,4)
1407dc58c:     	movss	%xmm8, 0x8(%rax,%rcx,4)
1407dc593:     	incq	%r15
1407dc596:     	movq	%r15, 0x150(%rbp)
1407dc59d:     	xorl	%eax, %eax
1407dc59f:     	movq	%rbx, %rcx
1407dc5a2:     	cmpq	0x248(%rbp), %rdi
1407dc5a9:     	je	0x1407dc745 <.text+0x7db745>
1407dc5af:     	leaq	0xc(%rdi), %rdx
1407dc5b3:     	testb	$0x1, %al
1407dc5b5:     	je	0x1407dc5f0 <.text+0x7db5f0>
1407dc5b7:     	cmpq	0x248(%rbp), %rdx
1407dc5be:     	je	0x1407dc745 <.text+0x7db745>
1407dc5c4:     	leaq	0x18(%rdi), %rbx
1407dc5c8:     	cmpq	0x248(%rbp), %rbx
1407dc5cf:     	je	0x1407dc745 <.text+0x7db745>
1407dc5d5:     	movq	%rdi, %rax
1407dc5d8:     	addq	$0x24, %rdi
1407dc5dc:     	movq	%rdx, %r14
1407dc5df:     	jmp	0x1407dc5fc <.text+0x7db5fc>
1407dc5e1:     	nopw	%cs:(%rax,%rax)
1407dc5f0:     	movq	%rdi, %rbx
1407dc5f3:     	movq	%r14, %rax
1407dc5f6:     	movq	%rcx, %r14
1407dc5f9:     	movq	%rdx, %rdi
1407dc5fc:     	movss	0x4(%rbx), %xmm10
1407dc602:     	movss	0x4(%rax), %xmm9
1407dc608:     	movss	(%rbx), %xmm0
1407dc60c:     	movss	0x8(%rbx), %xmm11
1407dc612:     	unpcklps	%xmm0, %xmm11           # xmm11 = xmm11[0],xmm0[0],xmm11[1],xmm0[1]
1407dc616:     	movss	(%rax), %xmm0
1407dc61a:     	movss	0x8(%rax), %xmm1
1407dc61f:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407dc622:     	subps	%xmm1, %xmm11
1407dc626:     	movshdup	%xmm11, %xmm8           # xmm8 = xmm11[1,1,3,3]
1407dc62b:     	movaps	%xmm11, %xmm0
1407dc62f:     	xorps	%xmm13, %xmm0
1407dc633:     	movss	%xmm0, 0x30(%rbp)
1407dc638:     	movss	%xmm8, 0x34(%rbp)
1407dc63e:     	movq	%r12, %rcx
1407dc641:     	movq	%r13, %rdx
1407dc644:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407dc649:     	subss	%xmm9, %xmm10
1407dc64e:     	movss	0xd0(%rbp), %xmm9
1407dc657:     	movsd	0xd4(%rbp), %xmm0
1407dc65f:     	movsldup	%xmm11, %xmm1           # xmm1 = xmm11[0,0,2,2]
1407dc664:     	movss	%xmm10, %xmm1           # xmm1 = xmm10[0],xmm1[1,2,3]
1407dc669:     	mulss	%xmm9, %xmm10
1407dc66e:     	shufps	$0xd4, %xmm0, %xmm9     # xmm9 = xmm9[0,1],xmm0[1,3]
1407dc673:     	shufps	$0xf2, %xmm0, %xmm9     # xmm9 = xmm9[2,0],xmm0[3,3]
1407dc678:     	mulps	%xmm1, %xmm9
1407dc67c:     	mulps	%xmm0, %xmm11
1407dc680:     	subps	%xmm11, %xmm9
1407dc684:     	mulss	%xmm0, %xmm8
1407dc689:     	subss	%xmm10, %xmm8
1407dc68e:     	movaps	%xmm9, %xmm0
1407dc692:     	mulps	%xmm9, %xmm0
1407dc696:     	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407dc69a:     	addss	%xmm0, %xmm1
1407dc69e:     	movaps	%xmm8, %xmm0
1407dc6a2:     	mulss	%xmm8, %xmm0
1407dc6a7:     	addss	%xmm1, %xmm0
1407dc6ab:     	sqrtss	%xmm0, %xmm0
1407dc6af:     	movaps	%xmm12, %xmm10
1407dc6b3:     	divss	%xmm0, %xmm10
1407dc6b8:     	movd	%xmm10, %eax
1407dc6bd:     	testl	%eax, %eax
1407dc6bf:     	setns	%cl
1407dc6c2:     	movl	%eax, %edx
1407dc6c4:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407dc6ca:     	addl	$0xff800000, %edx       # imm = 0xFF800000
1407dc6d0:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407dc6d6:     	setb	%dl
1407dc6d9:     	andb	%cl, %dl
1407dc6db:     	decl	%eax
1407dc6dd:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407dc6e2:     	setb	%sil
1407dc6e6:     	orb	%dl, %sil
1407dc6e9:     	jne	0x1407dc710 <.text+0x7db710>
1407dc6eb:     	movaps	%xmm7, %xmm8
1407dc6ef:     	movq	0x150(%rbp), %r15
1407dc6f6:     	cmpq	0x140(%rbp), %r15
1407dc6fd:     	jne	0x1407dc550 <.text+0x7db550>
1407dc703:     	jmp	0x1407dc72d <.text+0x7db72d>
1407dc705:     	nopw	%cs:(%rax,%rax)
1407dc710:     	xorps	%xmm13, %xmm8
1407dc714:     	mulss	%xmm10, %xmm8
1407dc719:     	movq	0x150(%rbp), %r15
1407dc720:     	cmpq	0x140(%rbp), %r15
1407dc727:     	jne	0x1407dc550 <.text+0x7db550>
1407dc72d:     	leaq	0x140(%rbp), %rcx
1407dc734:     	leaq	0x22abfed(%rip), %rdx   # 0x142a88728
1407dc73b:     	callq	0x140b9f180 <.text+0xb9e180>
1407dc740:     	jmp	0x1407dc550 <.text+0x7db550>
1407dc745:     	movq	0x248(%rbp), %rax
1407dc74c:     	movss	-0x14(%rax), %xmm8
1407dc752:     	movss	-0x8(%rax), %xmm7
1407dc757:     	movss	-0x18(%rax), %xmm0
1407dc75c:     	movss	-0x10(%rax), %xmm9
1407dc762:     	unpcklps	%xmm0, %xmm9            # xmm9 = xmm9[0],xmm0[0],xmm9[1],xmm0[1]
1407dc766:     	movss	-0xc(%rax), %xmm0
1407dc76b:     	movss	-0x4(%rax), %xmm1
1407dc770:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407dc773:     	subps	%xmm1, %xmm9
1407dc777:     	movshdup	%xmm9, %xmm6            # xmm6 = xmm9[1,1,3,3]
1407dc77c:     	movaps	0x214916d(%rip), %xmm0  # 0x1429258f0
1407dc783:     	xorps	%xmm9, %xmm0
1407dc787:     	movss	%xmm0, 0x30(%rbp)
1407dc78c:     	movss	%xmm6, 0x34(%rbp)
1407dc791:     	leaq	0xd0(%rbp), %rcx
1407dc798:     	leaq	0x30(%rbp), %rdx
1407dc79c:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407dc7a1:     	subss	%xmm7, %xmm8
1407dc7a6:     	movss	0xd0(%rbp), %xmm7
1407dc7ae:     	movsd	0xd4(%rbp), %xmm0
1407dc7b6:     	movsldup	%xmm9, %xmm1            # xmm1 = xmm9[0,0,2,2]
1407dc7bb:     	movss	%xmm8, %xmm1            # xmm1 = xmm8[0],xmm1[1,2,3]
1407dc7c0:     	mulss	%xmm7, %xmm8
1407dc7c5:     	shufps	$0xd4, %xmm0, %xmm7     # xmm7 = xmm7[0,1],xmm0[1,3]
1407dc7c9:     	shufps	$0xf2, %xmm0, %xmm7     # xmm7 = xmm7[2,0],xmm0[3,3]
1407dc7cd:     	mulps	%xmm1, %xmm7
1407dc7d0:     	mulps	%xmm0, %xmm9
1407dc7d4:     	subps	%xmm9, %xmm7
1407dc7d8:     	mulss	%xmm0, %xmm6
1407dc7dc:     	subss	%xmm8, %xmm6
1407dc7e1:     	movaps	%xmm7, %xmm0
1407dc7e4:     	mulps	%xmm7, %xmm0
1407dc7e7:     	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407dc7eb:     	addss	%xmm0, %xmm1
1407dc7ef:     	movaps	%xmm6, %xmm0
1407dc7f2:     	mulss	%xmm6, %xmm0
1407dc7f6:     	addss	%xmm1, %xmm0
1407dc7fa:     	sqrtss	%xmm0, %xmm0
1407dc7fe:     	movaps	%xmm12, %xmm8
1407dc802:     	divss	%xmm0, %xmm8
1407dc807:     	movd	%xmm8, %eax
1407dc80c:     	testl	%eax, %eax
1407dc80e:     	setns	%cl
1407dc811:     	movl	%eax, %edx
1407dc813:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407dc819:     	addl	$0xff800000, %edx       # imm = 0xFF800000
1407dc81f:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407dc825:     	setb	%dl
1407dc828:     	andb	%cl, %dl
1407dc82a:     	decl	%eax
1407dc82c:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407dc831:     	setb	%bl
1407dc834:     	orb	%dl, %bl
1407dc836:     	jne	0x1407dc852 <.text+0x7db852>
1407dc838:     	movss	0x2157e58(%rip), %xmm6  # 0x142934698
1407dc840:     	movq	0x150(%rbp), %rsi
1407dc847:     	cmpq	0x140(%rbp), %rsi
1407dc84e:     	je	0x1407dc86b <.text+0x7db86b>
1407dc850:     	jmp	0x1407dc87e <.text+0x7db87e>
1407dc852:     	xorps	%xmm13, %xmm6
1407dc856:     	mulss	%xmm8, %xmm6
1407dc85b:     	movq	0x150(%rbp), %rsi
1407dc862:     	cmpq	0x140(%rbp), %rsi
1407dc869:     	jne	0x1407dc87e <.text+0x7db87e>
1407dc86b:     	leaq	0x22abbf6(%rip), %rdx   # 0x142a88468
1407dc872:     	leaq	0x140(%rbp), %rcx
1407dc879:     	callq	0x140b9f180 <.text+0xb9e180>
1407dc87e:     	movzbl	%bl, %eax
1407dc881:     	movd	%eax, %xmm0
1407dc885:     	pshufd	$0x50, %xmm0, %xmm0     # xmm0 = xmm0[0,0,1,1]
1407dc88a:     	xorps	%xmm13, %xmm7
1407dc88e:     	movsldup	%xmm8, %xmm1            # xmm1 = xmm8[0,0,2,2]
1407dc893:     	mulps	%xmm7, %xmm1
1407dc896:     	pslld	$0x1f, %xmm0
1407dc89b:     	psrad	$0x1f, %xmm0
1407dc8a0:     	andps	%xmm0, %xmm1
1407dc8a3:     	andnps	0x21f0a86(%rip), %xmm0  # 0x1429cd330
1407dc8aa:     	orps	%xmm1, %xmm0
1407dc8ad:     	movq	0x148(%rbp), %rax
1407dc8b4:     	leaq	(%rsi,%rsi,2), %rdi
1407dc8b8:     	movlps	%xmm0, (%rax,%rdi,4)
1407dc8bc:     	movss	%xmm6, 0x8(%rax,%rdi,4)
1407dc8c2:     	movq	0x140(%rbp), %rbx
1407dc8c9:     	movq	0x148(%rbp), %rax
1407dc8d0:     	movq	%rax, 0x208(%rbp)
1407dc8d7:     	movq	0x210(%rbp), %rcx
1407dc8de:     	movq	%rcx, %rax
1407dc8e1:     	shlq	$0x4, %rax
1407dc8e5:     	leaq	(%rax,%rax,2), %rsi
1407dc8e9:     	xorl	%r15d, %r15d
1407dc8ec:     	movabsq	$0x555555555555555, %rax # imm = 0x555555555555555
1407dc8f6:     	cmpq	%rax, %rcx
1407dc8f9:     	movq	%rbx, 0xc0(%rbp)
1407dc900:     	ja	0x1407ddc4c <.text+0x7dcc4c>
1407dc906:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1407dc910:     	cmpq	%rax, %rsi
1407dc913:     	ja	0x1407ddc4c <.text+0x7dcc4c>
1407dc919:     	testq	%rsi, %rsi
1407dc91c:     	je	0x1407dc94d <.text+0x7db94d>
1407dc91e:     	movzbl	0x2e582e4(%rip), %eax   # 0x143634c09
1407dc925:     	movl	$0x4, %r15d
1407dc92b:     	movl	$0x4, %edx
1407dc930:     	movq	%rsi, %rcx
1407dc933:     	callq	0x140613c10 <.text+0x612c10>
1407dc938:     	testq	%rax, %rax
1407dc93b:     	je	0x1407ddc4c <.text+0x7dcc4c>
1407dc941:     	movq	%rax, %r14
1407dc944:     	movq	0x200(%rbp), %rax
1407dc94b:     	jmp	0x1407dc955 <.text+0x7db955>
1407dc94d:     	movl	$0x4, %r14d
1407dc953:     	xorl	%eax, %eax
1407dc955:     	movq	%rax, %r12
1407dc958:     	movq	%rax, 0x228(%rbp)
1407dc95f:     	movq	%r14, 0x230(%rbp)
1407dc966:     	movq	$0x0, 0x238(%rbp)
1407dc971:     	movabsq	$0x2aaaaaaaaaaaaaa, %rax # imm = 0x2AAAAAAAAAAAAAA
1407dc97b:     	cmpq	%rax, 0x210(%rbp)
1407dc982:     	ja	0x1407ddcc9 <.text+0x7dccc9>
1407dc988:     	movzbl	0x2e5827a(%rip), %eax   # 0x143634c09
1407dc98f:     	movl	$0x4, %r15d
1407dc995:     	movl	$0x4, %edx
1407dc99a:     	movq	%rsi, %rcx
1407dc99d:     	callq	0x140613c10 <.text+0x612c10>
1407dc9a2:     	testq	%rax, %rax
1407dc9a5:     	je	0x1407ddccc <.text+0x7dcccc>
1407dc9ab:     	movq	0x1d0(%rbp), %r15
1407dc9b2:     	movq	%r15, 0x270(%rbp)
1407dc9b9:     	movq	%rax, 0x278(%rbp)
1407dc9c0:     	movq	$0x0, 0x280(%rbp)
1407dc9cb:     	movzbl	0x2e58237(%rip), %eax   # 0x143634c09
1407dc9d2:     	movl	$0x4, %edx
1407dc9d7:     	movq	%rsi, %rcx
1407dc9da:     	callq	0x140613c10 <.text+0x612c10>
1407dc9df:     	testq	%rax, %rax
1407dc9e2:     	je	0x1407ddcee <.text+0x7dccee>
1407dc9e8:     	movq	%r14, 0x1f0(%rbp)
1407dc9ef:     	movq	%r15, 0x250(%rbp)
1407dc9f6:     	movq	%rax, 0x258(%rbp)
1407dc9fd:     	movq	$0x0, 0x260(%rbp)
1407dca08:     	movq	0x3f0(%rbp), %rax
1407dca0f:     	testq	%rax, %rax
1407dca12:     	sete	0x26f(%rbp)
1407dca19:     	je	0x1407ddd0b <.text+0x7dcd0b>
1407dca1f:     	movq	-0x10(%rbp), %rcx
1407dca23:     	movq	%rcx, 0x138(%rbp)
1407dca2a:     	movss	0x2148eed(%rip), %xmm13 # 0x142925920
1407dca33:     	movq	0x3e8(%rbp), %rcx
1407dca3a:     	subss	(%rcx), %xmm13
1407dca3f:     	shlq	$0x2, %rax
1407dca43:     	leaq	0xc(,%rdi,4), %rcx
1407dca4b:     	movq	%rcx, 0x10(%rbp)
1407dca4f:     	movq	0x3b0(%rbp), %rcx
1407dca56:     	shlq	$0x3, %rcx
1407dca5a:     	shlq	$0x2, 0xb8(%rbp)
1407dca62:     	xorps	%xmm1, %xmm1
1407dca65:     	xorl	%r15d, %r15d
1407dca68:     	xorps	%xmm0, %xmm0
1407dca6b:     	movaps	%xmm0, 0x90(%rbp)
1407dca72:     	xorps	%xmm15, %xmm15
1407dca76:     	xorps	%xmm10, %xmm10
1407dca7a:     	xorps	%xmm14, %xmm14
1407dca7e:     	xorps	%xmm12, %xmm12
1407dca82:     	movl	$0x0, %esi
1407dca87:     	movl	$0x0, %edx
1407dca8c:     	movq	%rdx, 0x288(%rbp)
1407dca93:     	movl	$0x0, 0x244(%rbp)
1407dca9d:     	xorl	%edi, %edi
1407dca9f:     	movq	%rax, 0x20(%rbp)
1407dcaa3:     	jmp	0x1407dcb7f <.text+0x7dbb7f>
1407dcaa8:     	nopl	(%rax,%rax)
1407dcab0:     	shlq	$0x20, %r13
1407dcab4:     	movl	0x90(%rbp), %eax
1407dcaba:     	orq	%r13, %rax
1407dcabd:     	movq	%rax, 0x1e8(%rbp)
1407dcac4:     	shlq	$0x20, %r14
1407dcac8:     	movl	%edi, %r13d
1407dcacb:     	orq	%r14, %r13
1407dcace:     	movaps	0x2148e1b(%rip), %xmm0  # 0x1429258f0
1407dcad5:     	xorps	%xmm0, %xmm6
1407dcad8:     	xorps	%xmm0, %xmm7
1407dcadb:     	movq	0x258(%rbp), %rax
1407dcae2:     	leaq	(%rbx,%rbx,2), %rcx
1407dcae6:     	movlps	%xmm6, (%rax,%rcx,4)
1407dcaea:     	movss	%xmm7, 0x8(%rax,%rcx,4)
1407dcaf0:     	addq	$0x2, %r12
1407dcaf4:     	movq	%r12, 0x260(%rbp)
1407dcafb:     	movl	$0x1, %edi
1407dcb00:     	movl	$0x1, 0x244(%rbp)
1407dcb0a:     	movq	0x130(%rbp), %rcx
1407dcb11:     	addq	$0x8, 0xb0(%rbp)
1407dcb19:     	movq	0x208(%rbp), %rax
1407dcb20:     	movsd	(%rax,%r15), %xmm1
1407dcb26:     	movss	0x8(%rax,%r15), %xmm0
1407dcb2d:     	movaps	%xmm0, 0x90(%rbp)
1407dcb34:     	addq	$0x4, %rsi
1407dcb38:     	addq	$0xc, %r15
1407dcb3c:     	addq	$-0x8, %rcx
1407dcb40:     	movq	0x8(%rbp), %rax
1407dcb44:     	addq	$0xc, %rax
1407dcb48:     	movaps	%xmm9, %xmm13
1407dcb4c:     	movss	0x240(%rbp), %xmm15
1407dcb55:     	movss	0x200(%rbp), %xmm10
1407dcb5e:     	movaps	0x1d0(%rbp), %xmm14
1407dcb66:     	movaps	0x210(%rbp), %xmm12
1407dcb6e:     	cmpq	0x248(%rbp), %rax
1407dcb75:     	movq	0x20(%rbp), %rax
1407dcb79:     	je	0x1407dd630 <.text+0x7dc630>
1407dcb7f:     	cmpq	%rsi, 0xb8(%rbp)
1407dcb86:     	je	0x1407dd630 <.text+0x7dc630>
1407dcb8c:     	testq	%rcx, %rcx
1407dcb8f:     	je	0x1407dd630 <.text+0x7dc630>
1407dcb95:     	cmpq	%r15, 0x10(%rbp)
1407dcb99:     	je	0x1407dd630 <.text+0x7dc630>
1407dcb9f:     	cmpq	%rsi, %rax
1407dcba2:     	je	0x1407dd630 <.text+0x7dc630>
1407dcba8:     	movaps	%xmm1, -0x50(%rbp)
1407dcbac:     	movq	%rcx, 0x130(%rbp)
1407dcbb3:     	movq	0x1c8(%rbp), %rax
1407dcbba:     	addq	%r15, %rax
1407dcbbd:     	movsd	(%rax), %xmm7
1407dcbc1:     	movq	%rax, 0x8(%rbp)
1407dcbc5:     	movss	0x8(%rax), %xmm6
1407dcbca:     	movss	0x2148d4d(%rip), %xmm9  # 0x142925920
1407dcbd3:     	movq	0x3e8(%rbp), %rax
1407dcbda:     	subss	(%rax,%rsi), %xmm9
1407dcbe0:     	movq	0xb0(%rbp), %rax
1407dcbe7:     	movss	(%rax), %xmm0
1407dcbeb:     	movss	0x4(%rax), %xmm1
1407dcbf0:     	xorps	0x2148cf9(%rip), %xmm1  # 0x1429258f0
1407dcbf7:     	movss	%xmm1, 0xd0(%rbp)
1407dcbff:     	movss	%xmm0, 0xd4(%rbp)
1407dcc07:     	leaq	0x140(%rbp), %rcx
1407dcc0e:     	leaq	0xd0(%rbp), %rdx
1407dcc15:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407dcc1a:     	movq	0x18(%rbp), %rax
1407dcc1e:     	movss	(%rax,%rsi), %xmm0
1407dcc23:     	movss	0x148(%rbp), %xmm2
1407dcc2b:     	mulss	%xmm0, %xmm2
1407dcc2f:     	movsd	0x140(%rbp), %xmm1
1407dcc37:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1407dcc3b:     	mulps	%xmm1, %xmm0
1407dcc3e:     	mulps	0x2148c9b(%rip), %xmm0  # 0x1429258e0
1407dcc45:     	mulss	0x2148c23(%rip), %xmm2  # 0x142925870
1407dcc4d:     	movaps	%xmm7, %xmm1
1407dcc50:     	subps	%xmm0, %xmm1
1407dcc53:     	movaps	%xmm1, 0x210(%rbp)
1407dcc5a:     	movaps	%xmm6, %xmm1
1407dcc5d:     	subss	%xmm2, %xmm1
1407dcc61:     	movss	%xmm1, 0x200(%rbp)
1407dcc69:     	addps	%xmm7, %xmm0
1407dcc6c:     	movaps	%xmm0, 0x1d0(%rbp)
1407dcc73:     	addss	%xmm6, %xmm2
1407dcc77:     	movss	%xmm2, 0x240(%rbp)
1407dcc7f:     	xorps	%xmm0, %xmm0
1407dcc82:     	cmpltps	%xmm9, %xmm0
1407dcc87:     	xorps	%xmm1, %xmm1
1407dcc8a:     	cmpltps	%xmm13, %xmm1
1407dcc8f:     	xorps	%xmm0, %xmm1
1407dcc92:     	movd	%xmm1, %eax
1407dcc96:     	testb	$0x1, %al
1407dcc98:     	je	0x1407dd0f5 <.text+0x7dc0f5>
1407dcc9e:     	movaps	%xmm13, %xmm0
1407dcca2:     	subss	%xmm9, %xmm0
1407dcca7:     	divss	%xmm0, %xmm13
1407dccac:     	movq	0x208(%rbp), %rax
1407dccb3:     	movsd	(%rax,%r15), %xmm1
1407dccb9:     	movss	0x8(%rax,%r15), %xmm0
1407dccc0:     	mulss	%xmm13, %xmm0
1407dccc5:     	movq	0x280(%rbp), %r12
1407dcccc:     	leal	0x1(%r12), %r14d
1407dccd1:     	testb	$0x1, %dil
1407dccd5:     	movss	%xmm0, 0x12c(%rbp)
1407dccdd:     	movaps	%xmm1, -0x40(%rbp)
1407dcce1:     	je	0x1407dcd50 <.text+0x7dbd50>
1407dcce3:     	movq	0x228(%rbp), %rax
1407dccea:     	subq	0x288(%rbp), %rax
1407dccf1:     	cmpq	$0x5, %rax
1407dccf5:     	jbe	0x1407dd526 <.text+0x7dc526>
1407dccfb:     	movq	0x1e8(%rbp), %r8
1407dcd02:     	movq	%r8, %rax
1407dcd05:     	shrq	$0x20, %rax
1407dcd09:     	movq	0x1f0(%rbp), %rcx
1407dcd10:     	movq	0x288(%rbp), %rdx
1407dcd17:     	movl	%r8d, (%rcx,%rdx,4)
1407dcd1b:     	movl	%eax, 0x4(%rcx,%rdx,4)
1407dcd1f:     	movl	%r12d, 0x8(%rcx,%rdx,4)
1407dcd24:     	movl	%r12d, 0xc(%rcx,%rdx,4)
1407dcd29:     	movl	%eax, 0x10(%rcx,%rdx,4)
1407dcd2d:     	movl	%r14d, 0x14(%rcx,%rdx,4)
1407dcd32:     	addq	$0x6, %rdx
1407dcd36:     	movq	%rdx, 0x288(%rbp)
1407dcd3d:     	movq	%rdx, 0x238(%rbp)
1407dcd44:     	movq	0x280(%rbp), %rdi
1407dcd4b:     	jmp	0x1407dcd5a <.text+0x7dbd5a>
1407dcd4d:     	nopl	(%rax)
1407dcd50:     	movq	%r12, %rdi
1407dcd53:     	movq	0x1e8(%rbp), %r8
1407dcd5a:     	shlq	$0x20, %r14
1407dcd5e:     	movl	%r12d, %eax
1407dcd61:     	orq	%r14, %rax
1407dcd64:     	ucomiss	0x2183924(%rip), %xmm9  # 0x142960690
1407dcd6c:     	cmovaq	%rax, %r8
1407dcd70:     	movq	%r8, 0x1e8(%rbp)
1407dcd77:     	seta	0x26e(%rbp)
1407dcd7e:     	cmpq	0x270(%rbp), %rdi
1407dcd85:     	jne	0x1407dcd9a <.text+0x7dbd9a>
1407dcd87:     	leaq	0x270(%rbp), %rcx
1407dcd8e:     	leaq	0x22ab813(%rip), %rdx   # 0x142a885a8
1407dcd95:     	callq	0x140b9f180 <.text+0xb9e180>
1407dcd9a:     	movss	0x2148b61(%rip), %xmm11 # 0x142925904
1407dcda3:     	subss	%xmm13, %xmm11
1407dcda8:     	mulss	%xmm11, %xmm10
1407dcdad:     	movsldup	%xmm11, %xmm8           # xmm8 = xmm11[0,0,2,2]
1407dcdb2:     	mulps	%xmm8, %xmm12
1407dcdb6:     	movsldup	%xmm13, %xmm7           # xmm7 = xmm13[0,0,2,2]
1407dcdbb:     	movaps	%xmm7, %xmm6
1407dcdbe:     	mulps	0x210(%rbp), %xmm6
1407dcdc5:     	addps	%xmm12, %xmm6
1407dcdc9:     	movaps	%xmm13, %xmm12
1407dcdcd:     	mulss	0x200(%rbp), %xmm12
1407dcdd6:     	addss	%xmm10, %xmm12
1407dcddb:     	movq	0x278(%rbp), %rax
1407dcde2:     	leaq	(%rdi,%rdi,2), %rcx
1407dcde6:     	movlps	%xmm6, (%rax,%rcx,4)
1407dcdea:     	movss	%xmm12, 0x8(%rax,%rcx,4)
1407dcdf1:     	leaq	0x1(%rdi), %rbx
1407dcdf5:     	movq	%rbx, 0x280(%rbp)
1407dcdfc:     	cmpq	0x270(%rbp), %rbx
1407dce03:     	jne	0x1407dce18 <.text+0x7dbe18>
1407dce05:     	leaq	0x270(%rbp), %rcx
1407dce0c:     	leaq	0x22ab7ad(%rip), %rdx   # 0x142a885c0
1407dce13:     	callq	0x140b9f180 <.text+0xb9e180>
1407dce18:     	mulss	%xmm11, %xmm15
1407dce1d:     	mulps	%xmm8, %xmm14
1407dce21:     	movaps	%xmm7, %xmm10
1407dce25:     	mulps	0x1d0(%rbp), %xmm10
1407dce2d:     	addps	%xmm14, %xmm10
1407dce31:     	mulss	0x240(%rbp), %xmm13
1407dce3a:     	addss	%xmm15, %xmm13
1407dce3f:     	movq	0x278(%rbp), %rax
1407dce46:     	leaq	(%rbx,%rbx,2), %rcx
1407dce4a:     	movlps	%xmm10, (%rax,%rcx,4)
1407dce4f:     	movss	%xmm13, 0x8(%rax,%rcx,4)
1407dce56:     	addq	$0x2, %rdi
1407dce5a:     	movq	%rdi, 0x280(%rbp)
1407dce61:     	movq	0x260(%rbp), %rdi
1407dce68:     	cmpq	0x250(%rbp), %rdi
1407dce6f:     	jne	0x1407dce84 <.text+0x7dbe84>
1407dce71:     	leaq	0x250(%rbp), %rcx
1407dce78:     	leaq	0x22ab759(%rip), %rdx   # 0x142a885d8
1407dce7f:     	callq	0x140b9f180 <.text+0xb9e180>
1407dce84:     	movaps	0x90(%rbp), %xmm0
1407dce8b:     	mulss	%xmm11, %xmm0
1407dce90:     	movaps	-0x50(%rbp), %xmm1
1407dce94:     	mulps	%xmm8, %xmm1
1407dce98:     	mulps	-0x40(%rbp), %xmm7
1407dce9c:     	addps	%xmm1, %xmm7
1407dce9f:     	addss	0x12c(%rbp), %xmm0
1407dcea7:     	movq	0x258(%rbp), %rax
1407dceae:     	leaq	(%rdi,%rdi,2), %rcx
1407dceb2:     	movlps	%xmm7, (%rax,%rcx,4)
1407dceb6:     	movaps	%xmm0, %xmm11
1407dceba:     	movss	%xmm0, 0x8(%rax,%rcx,4)
1407dcec0:     	leaq	0x1(%rdi), %rbx
1407dcec4:     	movq	%rbx, 0x260(%rbp)
1407dcecb:     	cmpq	0x250(%rbp), %rbx
1407dced2:     	jne	0x1407dcee7 <.text+0x7dbee7>
1407dced4:     	leaq	0x250(%rbp), %rcx
1407dcedb:     	leaq	0x22ab70e(%rip), %rdx   # 0x142a885f0
1407dcee2:     	callq	0x140b9f180 <.text+0xb9e180>
1407dcee7:     	movq	0x258(%rbp), %rax
1407dceee:     	leaq	(%rbx,%rbx,2), %rcx
1407dcef2:     	movlps	%xmm7, (%rax,%rcx,4)
1407dcef6:     	movss	%xmm11, 0x8(%rax,%rcx,4)
1407dcefd:     	addq	$0x2, %rdi
1407dcf01:     	movq	%rdi, 0x260(%rbp)
1407dcf08:     	movq	0x280(%rbp), %r12
1407dcf0f:     	leal	0x1(%r12), %r14d
1407dcf14:     	testb	$0x1, 0x244(%rbp)
1407dcf1b:     	je	0x1407dcf90 <.text+0x7dbf90>
1407dcf1d:     	movq	0x228(%rbp), %rax
1407dcf24:     	subq	0x288(%rbp), %rax
1407dcf2b:     	cmpq	$0x5, %rax
1407dcf2f:     	jbe	0x1407dd5ba <.text+0x7dc5ba>
1407dcf35:     	movq	%r13, %rax
1407dcf38:     	shrq	$0x20, %rax
1407dcf3c:     	movq	0x230(%rbp), %rcx
1407dcf43:     	movq	0x288(%rbp), %rdx
1407dcf4a:     	movl	%r13d, (%rcx,%rdx,4)
1407dcf4e:     	movl	%eax, 0x4(%rcx,%rdx,4)
1407dcf52:     	movl	%r12d, 0x8(%rcx,%rdx,4)
1407dcf57:     	movl	%r12d, 0xc(%rcx,%rdx,4)
1407dcf5c:     	movl	%eax, 0x10(%rcx,%rdx,4)
1407dcf60:     	movq	%rcx, 0x1f0(%rbp)
1407dcf67:     	movl	%r14d, 0x14(%rcx,%rdx,4)
1407dcf6c:     	addq	$0x6, %rdx
1407dcf70:     	movq	%rdx, 0x288(%rbp)
1407dcf77:     	movq	%rdx, 0x238(%rbp)
1407dcf7e:     	movq	0x280(%rbp), %rdi
1407dcf85:     	jmp	0x1407dcf93 <.text+0x7dbf93>
1407dcf87:     	nopw	(%rax,%rax)
1407dcf90:     	movq	%r12, %rdi
1407dcf93:     	shlq	$0x20, %r14
1407dcf97:     	movl	%r12d, %eax
1407dcf9a:     	orq	%r14, %rax
1407dcf9d:     	ucomiss	0x21836eb(%rip), %xmm9  # 0x142960690
1407dcfa5:     	cmovaq	%rax, %r13
1407dcfa9:     	movshdup	%xmm10, %xmm8           # xmm8 = xmm10[1,1,3,3]
1407dcfae:     	addss	0x22021b5(%rip), %xmm8  # 0x1429df16c
1407dcfb7:     	cmpq	0x270(%rbp), %rdi
1407dcfbe:     	jne	0x1407dcfd3 <.text+0x7dbfd3>
1407dcfc0:     	leaq	0x270(%rbp), %rcx
1407dcfc7:     	leaq	0x22ab63a(%rip), %rdx   # 0x142a88608
1407dcfce:     	callq	0x140b9f180 <.text+0xb9e180>
1407dcfd3:     	movq	0x278(%rbp), %rax
1407dcfda:     	leaq	(%rdi,%rdi,2), %rcx
1407dcfde:     	movss	%xmm10, (%rax,%rcx,4)
1407dcfe4:     	movss	%xmm8, 0x4(%rax,%rcx,4)
1407dcfeb:     	movss	%xmm13, 0x8(%rax,%rcx,4)
1407dcff2:     	leaq	0x1(%rdi), %rbx
1407dcff6:     	movq	%rbx, 0x280(%rbp)
1407dcffd:     	movshdup	%xmm6, %xmm8            # xmm8 = xmm6[1,1,3,3]
1407dd002:     	addss	0x2202161(%rip), %xmm8  # 0x1429df16c
1407dd00b:     	cmpq	0x270(%rbp), %rbx
1407dd012:     	jne	0x1407dd027 <.text+0x7dc027>
1407dd014:     	leaq	0x270(%rbp), %rcx
1407dd01b:     	leaq	0x22ab5fe(%rip), %rdx   # 0x142a88620
1407dd022:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd027:     	movq	0x278(%rbp), %rax
1407dd02e:     	leaq	(%rbx,%rbx,2), %rcx
1407dd032:     	movss	%xmm6, (%rax,%rcx,4)
1407dd037:     	movss	%xmm8, 0x4(%rax,%rcx,4)
1407dd03e:     	movss	%xmm12, 0x8(%rax,%rcx,4)
1407dd045:     	addq	$0x2, %rdi
1407dd049:     	movq	%rdi, 0x280(%rbp)
1407dd050:     	movq	0x260(%rbp), %rdi
1407dd057:     	cmpq	0x250(%rbp), %rdi
1407dd05e:     	jne	0x1407dd073 <.text+0x7dc073>
1407dd060:     	leaq	0x250(%rbp), %rcx
1407dd067:     	leaq	0x22ab5ca(%rip), %rdx   # 0x142a88638
1407dd06e:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd073:     	movaps	0x2148876(%rip), %xmm0  # 0x1429258f0
1407dd07a:     	xorps	%xmm0, %xmm7
1407dd07d:     	xorps	%xmm0, %xmm11
1407dd081:     	movq	0x258(%rbp), %rax
1407dd088:     	leaq	(%rdi,%rdi,2), %rcx
1407dd08c:     	movlps	%xmm7, (%rax,%rcx,4)
1407dd090:     	movaps	%xmm11, %xmm6
1407dd094:     	movss	%xmm11, 0x8(%rax,%rcx,4)
1407dd09b:     	leaq	0x1(%rdi), %rbx
1407dd09f:     	movq	%rbx, 0x260(%rbp)
1407dd0a6:     	cmpq	0x250(%rbp), %rbx
1407dd0ad:     	jne	0x1407dd0c2 <.text+0x7dc0c2>
1407dd0af:     	leaq	0x250(%rbp), %rcx
1407dd0b6:     	leaq	0x22ab593(%rip), %rdx   # 0x142a88650
1407dd0bd:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd0c2:     	xorl	%edx, %edx
1407dd0c4:     	movzbl	0x26e(%rbp), %eax
1407dd0cb:     	movb	%al, %dl
1407dd0cd:     	movq	0x258(%rbp), %rax
1407dd0d4:     	leaq	(%rbx,%rbx,2), %rcx
1407dd0d8:     	movlps	%xmm7, (%rax,%rcx,4)
1407dd0dc:     	movss	%xmm6, 0x8(%rax,%rcx,4)
1407dd0e2:     	addq	$0x2, %rdi
1407dd0e6:     	movq	%rdi, 0x260(%rbp)
1407dd0ed:     	movl	%edx, %edi
1407dd0ef:     	movl	%edx, 0x244(%rbp)
1407dd0f5:     	movq	0x130(%rbp), %rcx
1407dd0fc:     	ucomiss	0x218358c(%rip), %xmm9  # 0x142960690
1407dd104:     	movq	0x1e8(%rbp), %r8
1407dd10b:     	jbe	0x1407dcb11 <.text+0x7dbb11>
1407dd111:     	movq	0x280(%rbp), %r9
1407dd118:     	leal	0x1(%r9), %ebx
1407dd11c:     	testb	$0x1, %dil
1407dd120:     	movq	%r9, 0x90(%rbp)
1407dd127:     	movq	%r13, %r12
1407dd12a:     	je	0x1407dd1b0 <.text+0x7dc1b0>
1407dd130:     	movq	0x228(%rbp), %rax
1407dd137:     	subq	0x288(%rbp), %rax
1407dd13e:     	cmpq	$0x5, %rax
1407dd142:     	jbe	0x1407dd56f <.text+0x7dc56f>
1407dd148:     	movq	%rbx, %r13
1407dd14b:     	movq	%r8, %rax
1407dd14e:     	shrq	$0x20, %rax
1407dd152:     	movq	0x230(%rbp), %rcx
1407dd159:     	movq	0x288(%rbp), %rdx
1407dd160:     	movl	%r8d, (%rcx,%rdx,4)
1407dd164:     	movl	%eax, 0x4(%rcx,%rdx,4)
1407dd168:     	movl	%r9d, 0x8(%rcx,%rdx,4)
1407dd16d:     	movl	%r9d, 0xc(%rcx,%rdx,4)
1407dd172:     	movl	%eax, 0x10(%rcx,%rdx,4)
1407dd176:     	movq	%rcx, 0x1f0(%rbp)
1407dd17d:     	movl	%r13d, 0x14(%rcx,%rdx,4)
1407dd182:     	addq	$0x6, %rdx
1407dd186:     	movq	%rdx, 0x288(%rbp)
1407dd18d:     	movq	%rdx, 0x238(%rbp)
1407dd194:     	movq	0x280(%rbp), %rdi
1407dd19b:     	cmpq	0x270(%rbp), %rdi
1407dd1a2:     	je	0x1407dd1bf <.text+0x7dc1bf>
1407dd1a4:     	jmp	0x1407dd1d2 <.text+0x7dc1d2>
1407dd1a6:     	nopw	%cs:(%rax,%rax)
1407dd1b0:     	movq	%r9, %rdi
1407dd1b3:     	movq	%rbx, %r13
1407dd1b6:     	cmpq	0x270(%rbp), %rdi
1407dd1bd:     	jne	0x1407dd1d2 <.text+0x7dc1d2>
1407dd1bf:     	leaq	0x270(%rbp), %rcx
1407dd1c6:     	leaq	0x22ab49b(%rip), %rdx   # 0x142a88668
1407dd1cd:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd1d2:     	movq	0x278(%rbp), %rax
1407dd1d9:     	leaq	(%rdi,%rdi,2), %rcx
1407dd1dd:     	movaps	0x210(%rbp), %xmm0
1407dd1e4:     	movlps	%xmm0, (%rax,%rcx,4)
1407dd1e8:     	movss	0x200(%rbp), %xmm0
1407dd1f0:     	movss	%xmm0, 0x8(%rax,%rcx,4)
1407dd1f6:     	leaq	0x1(%rdi), %r14
1407dd1fa:     	movq	%r14, 0x280(%rbp)
1407dd201:     	cmpq	0x270(%rbp), %r14
1407dd208:     	jne	0x1407dd21d <.text+0x7dc21d>
1407dd20a:     	leaq	0x270(%rbp), %rcx
1407dd211:     	leaq	0x22ab468(%rip), %rdx   # 0x142a88680
1407dd218:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd21d:     	movq	0x208(%rbp), %rax
1407dd224:     	leaq	(%rax,%r15), %rbx
1407dd228:     	movq	0x278(%rbp), %rax
1407dd22f:     	leaq	(%r14,%r14,2), %rcx
1407dd233:     	movaps	0x1d0(%rbp), %xmm0
1407dd23a:     	movlps	%xmm0, (%rax,%rcx,4)
1407dd23e:     	movss	0x240(%rbp), %xmm0
1407dd246:     	movss	%xmm0, 0x8(%rax,%rcx,4)
1407dd24c:     	addq	$0x2, %rdi
1407dd250:     	movq	%rdi, 0x280(%rbp)
1407dd257:     	movl	0x8(%rbx), %eax
1407dd25a:     	movl	%eax, 0x148(%rbp)
1407dd260:     	movq	(%rbx), %rax
1407dd263:     	movq	%rax, 0x140(%rbp)
1407dd26a:     	movq	0x260(%rbp), %rdi
1407dd271:     	cmpq	0x250(%rbp), %rdi
1407dd278:     	jne	0x1407dd28d <.text+0x7dc28d>
1407dd27a:     	leaq	0x250(%rbp), %rcx
1407dd281:     	leaq	0x22ab410(%rip), %rdx   # 0x142a88698
1407dd288:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd28d:     	movq	0x258(%rbp), %rax
1407dd294:     	leaq	(%rdi,%rdi,2), %rcx
1407dd298:     	movl	0x148(%rbp), %edx
1407dd29e:     	movl	%edx, 0x8(%rax,%rcx,4)
1407dd2a2:     	movq	0x140(%rbp), %rdx
1407dd2a9:     	movq	%rdx, (%rax,%rcx,4)
1407dd2ad:     	leaq	0x1(%rdi), %r14
1407dd2b1:     	movq	%r14, 0x260(%rbp)
1407dd2b8:     	movl	0x8(%rbx), %eax
1407dd2bb:     	movl	%eax, 0x148(%rbp)
1407dd2c1:     	movq	(%rbx), %rax
1407dd2c4:     	movq	%rax, 0x140(%rbp)
1407dd2cb:     	cmpq	0x250(%rbp), %r14
1407dd2d2:     	jne	0x1407dd2e7 <.text+0x7dc2e7>
1407dd2d4:     	leaq	0x250(%rbp), %rcx
1407dd2db:     	leaq	0x22ab3ce(%rip), %rdx   # 0x142a886b0
1407dd2e2:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd2e7:     	movq	0x258(%rbp), %rax
1407dd2ee:     	leaq	(%r14,%r14,2), %rcx
1407dd2f2:     	movl	0x148(%rbp), %edx
1407dd2f8:     	movl	%edx, 0x8(%rax,%rcx,4)
1407dd2fc:     	movq	0x140(%rbp), %rdx
1407dd303:     	movq	%rdx, (%rax,%rcx,4)
1407dd307:     	addq	$0x2, %rdi
1407dd30b:     	movq	%rdi, 0x260(%rbp)
1407dd312:     	movq	0x280(%rbp), %rdi
1407dd319:     	leal	0x1(%rdi), %r14d
1407dd31d:     	testb	$0x1, 0x244(%rbp)
1407dd324:     	je	0x1407dd3c0 <.text+0x7dc3c0>
1407dd32a:     	movq	0x228(%rbp), %rax
1407dd331:     	subq	0x288(%rbp), %rax
1407dd338:     	cmpq	$0x5, %rax
1407dd33c:     	jbe	0x1407dd5f5 <.text+0x7dc5f5>
1407dd342:     	movq	%r12, %rdx
1407dd345:     	movq	%r12, %rax
1407dd348:     	shrq	$0x20, %rax
1407dd34c:     	movq	0x230(%rbp), %rcx
1407dd353:     	movq	0x288(%rbp), %r8
1407dd35a:     	movl	%edx, (%rcx,%r8,4)
1407dd35e:     	movl	%eax, 0x4(%rcx,%r8,4)
1407dd363:     	movl	%edi, 0x8(%rcx,%r8,4)
1407dd368:     	movl	%edi, 0xc(%rcx,%r8,4)
1407dd36d:     	movl	%eax, 0x10(%rcx,%r8,4)
1407dd372:     	movq	%rcx, 0x1f0(%rbp)
1407dd379:     	movl	%r14d, 0x14(%rcx,%r8,4)
1407dd37e:     	addq	$0x6, %r8
1407dd382:     	movq	%r8, 0x288(%rbp)
1407dd389:     	movq	%r8, 0x238(%rbp)
1407dd390:     	movq	0x280(%rbp), %r12
1407dd397:     	movshdup	0x1d0(%rbp), %xmm6      # xmm6 = mem[1,1,3,3]
1407dd39f:     	addss	0x2201dc5(%rip), %xmm6  # 0x1429df16c
1407dd3a7:     	cmpq	0x270(%rbp), %r12
1407dd3ae:     	je	0x1407dd3dc <.text+0x7dc3dc>
1407dd3b0:     	jmp	0x1407dd3ef <.text+0x7dc3ef>
1407dd3b2:     	nopw	%cs:(%rax,%rax)
1407dd3c0:     	movq	%rdi, %r12
1407dd3c3:     	movshdup	0x1d0(%rbp), %xmm6      # xmm6 = mem[1,1,3,3]
1407dd3cb:     	addss	0x2201d99(%rip), %xmm6  # 0x1429df16c
1407dd3d3:     	cmpq	0x270(%rbp), %r12
1407dd3da:     	jne	0x1407dd3ef <.text+0x7dc3ef>
1407dd3dc:     	leaq	0x270(%rbp), %rcx
1407dd3e3:     	leaq	0x22ab2de(%rip), %rdx   # 0x142a886c8
1407dd3ea:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd3ef:     	movq	0x278(%rbp), %rax
1407dd3f6:     	leaq	(%r12,%r12,2), %rcx
1407dd3fa:     	movaps	0x1d0(%rbp), %xmm0
1407dd401:     	movss	%xmm0, (%rax,%rcx,4)
1407dd406:     	movss	%xmm6, 0x4(%rax,%rcx,4)
1407dd40c:     	movss	0x240(%rbp), %xmm0
1407dd414:     	movss	%xmm0, 0x8(%rax,%rcx,4)
1407dd41a:     	leaq	0x1(%r12), %rbx
1407dd41f:     	movq	%rbx, 0x280(%rbp)
1407dd426:     	movshdup	0x210(%rbp), %xmm6      # xmm6 = mem[1,1,3,3]
1407dd42e:     	addss	0x2201d36(%rip), %xmm6  # 0x1429df16c
1407dd436:     	cmpq	0x270(%rbp), %rbx
1407dd43d:     	jne	0x1407dd452 <.text+0x7dc452>
1407dd43f:     	leaq	0x270(%rbp), %rcx
1407dd446:     	leaq	0x22ab293(%rip), %rdx   # 0x142a886e0
1407dd44d:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd452:     	movq	0x278(%rbp), %rax
1407dd459:     	leaq	(%rbx,%rbx,2), %rcx
1407dd45d:     	movaps	0x210(%rbp), %xmm0
1407dd464:     	movss	%xmm0, (%rax,%rcx,4)
1407dd469:     	movss	%xmm6, 0x4(%rax,%rcx,4)
1407dd46f:     	movss	0x200(%rbp), %xmm0
1407dd477:     	movss	%xmm0, 0x8(%rax,%rcx,4)
1407dd47d:     	addq	$0x2, %r12
1407dd481:     	movq	%r12, 0x280(%rbp)
1407dd488:     	movq	0x208(%rbp), %rax
1407dd48f:     	movsd	(%rax,%r15), %xmm6
1407dd495:     	movss	0x8(%rax,%r15), %xmm7
1407dd49c:     	movq	0x260(%rbp), %r12
1407dd4a3:     	cmpq	0x250(%rbp), %r12
1407dd4aa:     	jne	0x1407dd4bf <.text+0x7dc4bf>
1407dd4ac:     	leaq	0x250(%rbp), %rcx
1407dd4b3:     	leaq	0x22ab23e(%rip), %rdx   # 0x142a886f8
1407dd4ba:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd4bf:     	movaps	0x214842a(%rip), %xmm0  # 0x1429258f0
1407dd4c6:     	xorps	%xmm0, %xmm6
1407dd4c9:     	xorps	%xmm0, %xmm7
1407dd4cc:     	movq	0x258(%rbp), %rax
1407dd4d3:     	leaq	(%r12,%r12,2), %rcx
1407dd4d7:     	movlps	%xmm6, (%rax,%rcx,4)
1407dd4db:     	movss	%xmm7, 0x8(%rax,%rcx,4)
1407dd4e1:     	leaq	0x1(%r12), %rbx
1407dd4e6:     	movq	%rbx, 0x260(%rbp)
1407dd4ed:     	movq	0x208(%rbp), %rax
1407dd4f4:     	movsd	(%rax,%r15), %xmm6
1407dd4fa:     	movss	0x8(%rax,%r15), %xmm7
1407dd501:     	cmpq	0x250(%rbp), %rbx
1407dd508:     	jne	0x1407dcab0 <.text+0x7dbab0>
1407dd50e:     	leaq	0x250(%rbp), %rcx
1407dd515:     	leaq	0x22ab1f4(%rip), %rdx   # 0x142a88710
1407dd51c:     	callq	0x140b9f180 <.text+0xb9e180>
1407dd521:     	jmp	0x1407dcab0 <.text+0x7dbab0>
1407dd526:     	movq	$0x4, 0x20(%rsp)
1407dd52f:     	movl	$0x6, %r8d
1407dd535:     	movl	$0x4, %r9d
1407dd53b:     	leaq	0x228(%rbp), %rcx
1407dd542:     	movq	0x288(%rbp), %rdx
1407dd549:     	callq	0x1428a7f20 <.text+0x28a6f20>
1407dd54e:     	movq	0x230(%rbp), %rax
1407dd555:     	movq	%rax, 0x1f0(%rbp)
1407dd55c:     	movq	0x238(%rbp), %rax
1407dd563:     	movq	%rax, 0x288(%rbp)
1407dd56a:     	jmp	0x1407dccfb <.text+0x7dbcfb>
1407dd56f:     	movq	%r9, %rdi
1407dd572:     	movq	$0x4, 0x20(%rsp)
1407dd57b:     	movl	$0x6, %r8d
1407dd581:     	movl	$0x4, %r9d
1407dd587:     	leaq	0x228(%rbp), %rcx
1407dd58e:     	movq	0x288(%rbp), %rdx
1407dd595:     	callq	0x1428a7f20 <.text+0x28a6f20>
1407dd59a:     	movq	%rbx, %r13
1407dd59d:     	movq	0x238(%rbp), %rax
1407dd5a4:     	movq	%rax, 0x288(%rbp)
1407dd5ab:     	movq	0x1e8(%rbp), %r8
1407dd5b2:     	movq	%rdi, %r9
1407dd5b5:     	jmp	0x1407dd14b <.text+0x7dc14b>
1407dd5ba:     	movq	$0x4, 0x20(%rsp)
1407dd5c3:     	movl	$0x6, %r8d
1407dd5c9:     	movl	$0x4, %r9d
1407dd5cf:     	leaq	0x228(%rbp), %rcx
1407dd5d6:     	movq	0x288(%rbp), %rdx
1407dd5dd:     	callq	0x1428a7f20 <.text+0x28a6f20>
1407dd5e2:     	movq	0x238(%rbp), %rax
1407dd5e9:     	movq	%rax, 0x288(%rbp)
1407dd5f0:     	jmp	0x1407dcf35 <.text+0x7dbf35>
1407dd5f5:     	movq	$0x4, 0x20(%rsp)
1407dd5fe:     	movl	$0x6, %r8d
1407dd604:     	movl	$0x4, %r9d
1407dd60a:     	leaq	0x228(%rbp), %rcx
1407dd611:     	movq	0x288(%rbp), %rdx
1407dd618:     	callq	0x1428a7f20 <.text+0x28a6f20>
1407dd61d:     	movq	0x238(%rbp), %rax
1407dd624:     	movq	%rax, 0x288(%rbp)
1407dd62b:     	jmp	0x1407dd342 <.text+0x7dc342>
1407dd630:     	movq	0x138(%rbp), %rax
1407dd637:     	testq	%rax, %rax
1407dd63a:     	je	0x1407dd656 <.text+0x7dc656>
1407dd63c:     	shlq	$0x2, %rax
1407dd640:     	leaq	(%rax,%rax,2), %rdx
1407dd644:     	movl	$0x4, %r8d
1407dd64a:     	movq	0x1c8(%rbp), %rcx
1407dd651:     	callq	0x140613c20 <.text+0x612c20>
1407dd656:     	movq	0x230(%rbp), %rdx
1407dd65d:     	leaq	0x140(%rbp), %rcx
1407dd664:     	movq	0x288(%rbp), %r8
1407dd66b:     	callq	0x14129dbd0 <.text+0x129cbd0>
1407dd670:     	movq	0x278(%rbp), %r8
1407dd677:     	movq	0x280(%rbp), %r9
1407dd67e:     	leaq	0xd0(%rbp), %rcx
1407dd685:     	leaq	0x140(%rbp), %rdx
1407dd68c:     	callq	0x14129e890 <.text+0x129d890>
1407dd691:     	movq	0x258(%rbp), %r8
1407dd698:     	movq	0x260(%rbp), %r9
1407dd69f:     	leaq	0x140(%rbp), %rcx
1407dd6a6:     	leaq	0xd0(%rbp), %rsi
1407dd6ad:     	movq	%rsi, %rdx
1407dd6b0:     	callq	0x1412a90c0 <.text+0x12a80c0>
1407dd6b5:     	movq	$0x11, 0x20(%rsp)
1407dd6be:     	leaq	0x22aae1b(%rip), %r9    # 0x142a884e0
1407dd6c5:     	leaq	0x30(%rbp), %rcx
1407dd6c9:     	leaq	0x140(%rbp), %rdx
1407dd6d0:     	movq	0x28(%rbp), %r8
1407dd6d4:     	callq	0x14129e900 <.text+0x129d900>
1407dd6d9:     	cmpl	$0xb, 0x30(%rbp)
1407dd6dd:     	jne	0x1407dd949 <.text+0x7dc949>
1407dd6e3:     	movq	0x38(%rbp), %rax
1407dd6e7:     	movq	%rax, 0x210(%rbp)
1407dd6ee:     	movq	0x40(%rbp), %rax
1407dd6f2:     	movq	%rax, 0x248(%rbp)
1407dd6f9:     	movq	0x48(%rbp), %r13
1407dd6fd:     	movq	0x70(%rbp), %rax
1407dd701:     	movq	%rax, 0x170(%rbp)
1407dd708:     	movups	0x60(%rbp), %xmm0
1407dd70c:     	movups	%xmm0, 0x160(%rbp)
1407dd713:     	movups	0x50(%rbp), %xmm0
1407dd717:     	movups	%xmm0, 0x150(%rbp)
1407dd71e:     	movq	$0x1, 0x140(%rbp)
1407dd729:     	movq	$0x1, 0x148(%rbp)
1407dd734:     	movzbl	0x2e574ce(%rip), %eax   # 0x143634c09
1407dd73b:     	movl	$0x38, %ecx
1407dd740:     	movl	$0x8, %edx
1407dd745:     	callq	0x140613c10 <.text+0x612c10>
1407dd74a:     	testq	%rax, %rax
1407dd74d:     	movq	%rax, 0x1d0(%rbp)
1407dd754:     	je	0x1407ddd1d <.text+0x7dcd1d>
1407dd75a:     	movq	0x170(%rbp), %rcx
1407dd761:     	movq	%rcx, 0x30(%rax)
1407dd765:     	movups	0x140(%rbp), %xmm0
1407dd76c:     	movups	0x150(%rbp), %xmm1
1407dd773:     	movups	0x160(%rbp), %xmm2
1407dd77a:     	movups	%xmm2, 0x20(%rax)
1407dd77e:     	movups	%xmm1, 0x10(%rax)
1407dd782:     	movups	%xmm0, (%rax)
1407dd785:     	movq	%rax, 0x1f8(%rbp)
1407dd78c:     	testq	%r13, %r13
1407dd78f:     	je	0x1407ddaa6 <.text+0x7dcaa6>
1407dd795:     	movq	0x3c8(%rbp), %rax
1407dd79c:     	movq	0xb0(%rax), %rsi
1407dd7a3:     	leaq	0x15c(%rbp), %r12
1407dd7aa:     	shlq	$0x5, %r13
1407dd7ae:     	leaq	0xd0(%rbp), %rbx
1407dd7b5:     	movaps	0x2160904(%rip), %xmm6  # 0x14293e0c0
1407dd7bc:     	xorl	%edi, %edi
1407dd7be:     	jmp	0x1407dd7dc <.text+0x7dc7dc>
1407dd7c0:     	movq	0x8(%rsi), %rax
1407dd7c4:     	movq	%r15, (%rax,%r14,8)
1407dd7c8:     	incq	%r14
1407dd7cb:     	movq	%r14, 0x10(%rsi)
1407dd7cf:     	addq	$0x20, %rdi
1407dd7d3:     	cmpq	%rdi, %r13
1407dd7d6:     	je	0x1407ddaa6 <.text+0x7dcaa6>
1407dd7dc:     	movq	0x248(%rbp), %rax
1407dd7e3:     	movups	(%rax,%rdi), %xmm0
1407dd7e7:     	movups	0x10(%rax,%rdi), %xmm1
1407dd7ec:     	movaps	%xmm1, -0x20(%rbp)
1407dd7f0:     	movaps	%xmm0, -0x30(%rbp)
1407dd7f4:     	leaq	-0x30(%rbp), %rcx
1407dd7f8:     	callq	0x14129dbc0 <.text+0x129cbc0>
1407dd7fd:     	movl	%eax, %eax
1407dd7ff:     	movl	$0xaaaaaaab, %ecx       # imm = 0xAAAAAAAB
1407dd804:     	imulq	%rcx, %rax
1407dd808:     	shrq	$0x21, %rax
1407dd80c:     	subl	$0x1, %eax
1407dd80f:     	movl	$0x0, %ecx
1407dd814:     	cmovbl	%ecx, %eax
1407dd817:     	testl	%eax, %eax
1407dd819:     	je	0x1407dd930 <.text+0x7dc930>
1407dd81f:     	bsrl	%eax, %eax
1407dd822:     	xorl	$0x1f, %eax
1407dd825:     	xorl	%ecx, %ecx
1407dd827:     	movl	$0x1e, %edx
1407dd82c:     	subl	%eax, %edx
1407dd82e:     	jb	0x1407dd832 <.text+0x7dc832>
1407dd830:     	movl	%edx, %ecx
1407dd832:     	callq	0x140833250 <.text+0x832250>
1407dd837:     	movq	%rbx, %rcx
1407dd83a:     	movl	%eax, %edx
1407dd83c:     	callq	0x14082bd60 <.text+0x82ad60>
1407dd841:     	movups	0xd0(%rbp), %xmm0
1407dd848:     	movaps	%xmm0, 0x80(%rbp)
1407dd84f:     	callq	0x14003bb80 <.text+0x3ab80>
1407dd854:     	movaps	0x80(%rbp), %xmm0
1407dd85b:     	movaps	%xmm0, 0xd0(%rbp)
1407dd862:     	movq	0x1f8(%rbp), %rax
1407dd869:     	lock
1407dd86a:     	incq	(%rax)
1407dd86d:     	jle	0x1407ddd2c <.text+0x7dcd2c>
1407dd873:     	movaps	-0x30(%rbp), %xmm0
1407dd877:     	movaps	-0x20(%rbp), %xmm1
1407dd87b:     	movups	%xmm1, 0x4c(%r12)
1407dd881:     	movups	%xmm0, 0x3c(%r12)
1407dd887:     	movaps	%xmm6, 0x140(%rbp)
1407dd88e:     	leaq	0x150(%rbp), %rcx
1407dd895:     	movl	$0x0, 0x8(%rcx)
1407dd89c:     	movq	$0x0, (%rcx)
1407dd8a3:     	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1407dd8ad:     	movq	%rcx, (%r12)
1407dd8b1:     	movl	$0x3f800000, 0x8(%r12)  # imm = 0x3F800000
1407dd8ba:     	movaps	0xd0(%rbp), %xmm0
1407dd8c1:     	movaps	%xmm0, 0x14(%r12)
1407dd8c7:     	movb	$0x1a, 0x180(%rbp)
1407dd8ce:     	movq	%rax, 0x190(%rbp)
1407dd8d5:     	movl	0x3d8(%rbp), %eax
1407dd8db:     	movl	%eax, 0x1b8(%rbp)
1407dd8e1:     	movq	%rbx, %rcx
1407dd8e4:     	movq	0x3d0(%rbp), %rdx
1407dd8eb:     	leaq	0x140(%rbp), %r8
1407dd8f2:     	leaq	0x22aabff(%rip), %r9    # 0x142a884f8
1407dd8f9:     	callq	0x14126e2d0 <.text+0x126d2d0>
1407dd8fe:     	movq	0xf0(%rbp), %r15
1407dd905:     	movq	0x10(%rsi), %r14
1407dd909:     	cmpq	(%rsi), %r14
1407dd90c:     	jne	0x1407dd7c0 <.text+0x7dc7c0>
1407dd912:     	movq	%rsi, %rcx
1407dd915:     	leaq	0x22aabf4(%rip), %rdx   # 0x142a88510
1407dd91c:     	callq	0x140b81df0 <.text+0xb80df0>
1407dd921:     	jmp	0x1407dd7c0 <.text+0x7dc7c0>
1407dd926:     	nopw	%cs:(%rax,%rax)
1407dd930:     	movl	$0x20, %eax
1407dd935:     	xorl	%ecx, %ecx
1407dd937:     	movl	$0x1e, %edx
1407dd93c:     	subl	%eax, %edx
1407dd93e:     	jae	0x1407dd830 <.text+0x7dc830>
1407dd944:     	jmp	0x1407dd832 <.text+0x7dc832>
1407dd949:     	movups	0x70(%rbp), %xmm0
1407dd94d:     	movaps	%xmm0, 0x110(%rbp)
1407dd954:     	movups	0x30(%rbp), %xmm0
1407dd958:     	movups	0x40(%rbp), %xmm1
1407dd95c:     	movups	0x50(%rbp), %xmm2
1407dd960:     	movups	0x60(%rbp), %xmm3
1407dd964:     	movaps	%xmm3, 0x100(%rbp)
1407dd96b:     	movaps	%xmm2, 0xf0(%rbp)
1407dd972:     	movaps	%xmm1, 0xe0(%rbp)
1407dd979:     	movaps	%xmm0, 0xd0(%rbp)
1407dd980:     	movq	0x2e4ea21(%rip), %rax   # 0x14362c3a8
1407dd987:     	movq	(%rax), %rax
1407dd98a:     	testq	%rax, %rax
1407dd98d:     	je	0x1407dda6d <.text+0x7dca6d>
1407dd993:     	movq	%rsi, 0x80(%rbp)
1407dd99a:     	leaq	-0xea71(%rip), %rax     # 0x1407cef30 <.text+0x7cdf30>
1407dd9a1:     	movq	%rax, 0x88(%rbp)
1407dd9a8:     	leaq	0x22aaba9(%rip), %rcx   # 0x142a88558
1407dd9af:     	callq	0x14067dd80 <.text+0x67cd80>
1407dd9b4:     	movups	(%rax), %xmm0
1407dd9b7:     	movl	0x10(%rax), %eax
1407dd9ba:     	movq	$0x1, 0x170(%rbp)
1407dd9c5:     	leaq	0x22aaba4(%rip), %rcx   # 0x142a88570
1407dd9cc:     	movq	%rcx, 0x178(%rbp)
1407dd9d3:     	movq	$0x33, 0x180(%rbp)
1407dd9de:     	leaq	0x22aab63(%rip), %rdx   # 0x142a88548
1407dd9e5:     	movq	%rdx, 0x190(%rbp)
1407dd9ec:     	movq	$0x1, 0x198(%rbp)
1407dd9f7:     	leaq	0x80(%rbp), %rdx
1407dd9fe:     	movq	%rdx, 0x1a0(%rbp)
1407dda05:     	movq	$0x1, 0x1a8(%rbp)
1407dda10:     	movq	$0x0, 0x1b0(%rbp)
1407dda1b:     	movq	$0x0, 0x140(%rbp)
1407dda26:     	movq	%rcx, 0x148(%rbp)
1407dda2d:     	movq	$0x33, 0x150(%rbp)
1407dda38:     	movq	$0x0, 0x158(%rbp)
1407dda43:     	movups	%xmm0, 0x160(%rbp)
1407dda4a:     	movl	$0x1, 0x188(%rbp)
1407dda54:     	movl	%eax, 0x18c(%rbp)
1407dda5a:     	leaq	0x1f8(%rbp), %rcx
1407dda61:     	leaq	0x140(%rbp), %rdx
1407dda68:     	callq	0x140d4a330 <.text+0xd49330>
1407dda6d:     	movq	0xd0(%rbp), %rax
1407dda74:     	leaq	-0x9(%rax), %rdx
1407dda78:     	leaq	-0x8(%rax), %r8
1407dda7c:     	xorl	%ecx, %ecx
1407dda7e:     	cmpq	$0x2, %rdx
1407dda82:     	cmovbq	%r8, %rcx
1407dda86:     	testq	%rcx, %rcx
1407dda89:     	je	0x1407ddae3 <.text+0x7dcae3>
1407dda8b:     	cmpq	$0x1, %rcx
1407dda8f:     	jne	0x1407ddc29 <.text+0x7dcc29>
1407dda95:     	leaq	0xd8(%rbp), %rcx
1407dda9c:     	callq	0x140888530 <.text+0x887530>
1407ddaa1:     	jmp	0x1407ddb33 <.text+0x7dcb33>
1407ddaa6:     	movq	0x210(%rbp), %rdx
1407ddaad:     	testq	%rdx, %rdx
1407ddab0:     	je	0x1407ddac8 <.text+0x7dcac8>
1407ddab2:     	shlq	$0x5, %rdx
1407ddab6:     	movl	$0x8, %r8d
1407ddabc:     	movq	0x248(%rbp), %rcx
1407ddac3:     	callq	0x140613c20 <.text+0x612c20>
1407ddac8:     	movq	0x1f8(%rbp), %rax
1407ddacf:     	lock
1407ddad0:     	decq	(%rax)
1407ddad3:     	jne	0x1407ddb33 <.text+0x7dcb33>
1407ddad5:     	leaq	0x1f8(%rbp), %rcx
1407ddadc:     	callq	0x140b5cc00 <.text+0xb5bc00>
1407ddae1:     	jmp	0x1407ddb33 <.text+0x7dcb33>
1407ddae3:     	cmpq	$0x5, %rax
1407ddae7:     	ja	0x1407ddb12 <.text+0x7dcb12>
1407ddae9:     	movl	$0x32, %ecx
1407ddaee:     	btq	%rax, %rcx
1407ddaf2:     	jae	0x1407ddb12 <.text+0x7dcb12>
1407ddaf4:     	movq	0xd8(%rbp), %rdx
1407ddafb:     	testq	%rdx, %rdx
1407ddafe:     	je	0x1407ddb12 <.text+0x7dcb12>
1407ddb00:     	movq	0xe0(%rbp), %rcx
1407ddb07:     	movl	$0x1, %r8d
1407ddb0d:     	callq	0x140613c20 <.text+0x612c20>
1407ddb12:     	movq	0xf0(%rbp), %rdx
1407ddb19:     	testq	%rdx, %rdx
1407ddb1c:     	je	0x1407ddb33 <.text+0x7dcb33>
1407ddb1e:     	movq	0xc8(%rbp), %rax
1407ddb25:     	movq	(%rax), %rcx
1407ddb28:     	movl	$0x1, %r8d
1407ddb2e:     	callq	0x140613c20 <.text+0x612c20>
1407ddb33:     	movq	0x250(%rbp), %rax
1407ddb3a:     	testq	%rax, %rax
1407ddb3d:     	movq	0xc0(%rbp), %rsi
1407ddb44:     	je	0x1407ddb60 <.text+0x7dcb60>
1407ddb46:     	movq	0x258(%rbp), %rcx
1407ddb4d:     	shlq	$0x2, %rax
1407ddb51:     	leaq	(%rax,%rax,2), %rdx
1407ddb55:     	movl	$0x4, %r8d
1407ddb5b:     	callq	0x140613c20 <.text+0x612c20>
1407ddb60:     	movq	0x270(%rbp), %rax
1407ddb67:     	testq	%rax, %rax
1407ddb6a:     	je	0x1407ddb86 <.text+0x7dcb86>
1407ddb6c:     	movq	0x278(%rbp), %rcx
1407ddb73:     	shlq	$0x2, %rax
1407ddb77:     	leaq	(%rax,%rax,2), %rdx
1407ddb7b:     	movl	$0x4, %r8d
1407ddb81:     	callq	0x140613c20 <.text+0x612c20>
1407ddb86:     	movq	0x228(%rbp), %rdx
1407ddb8d:     	testq	%rdx, %rdx
1407ddb90:     	je	0x1407ddba8 <.text+0x7dcba8>
1407ddb92:     	movq	0x230(%rbp), %rcx
1407ddb99:     	shlq	$0x2, %rdx
1407ddb9d:     	movl	$0x4, %r8d
1407ddba3:     	callq	0x140613c20 <.text+0x612c20>
1407ddba8:     	testq	%rsi, %rsi
1407ddbab:     	je	0x1407ddbc7 <.text+0x7dcbc7>
1407ddbad:     	shlq	$0x2, %rsi
1407ddbb1:     	leaq	(%rsi,%rsi,2), %rdx
1407ddbb5:     	movl	$0x4, %r8d
1407ddbbb:     	movq	0x208(%rbp), %rcx
1407ddbc2:     	callq	0x140613c20 <.text+0x612c20>
1407ddbc7:     	movaps	0x2a0(%rbp), %xmm6
1407ddbce:     	movaps	0x2b0(%rbp), %xmm7
1407ddbd5:     	movaps	0x2c0(%rbp), %xmm8
1407ddbdd:     	movaps	0x2d0(%rbp), %xmm9
1407ddbe5:     	movaps	0x2e0(%rbp), %xmm10
1407ddbed:     	movaps	0x2f0(%rbp), %xmm11
1407ddbf5:     	movaps	0x300(%rbp), %xmm12
1407ddbfd:     	movaps	0x310(%rbp), %xmm13
1407ddc05:     	movaps	0x320(%rbp), %xmm14
1407ddc0d:     	movaps	0x330(%rbp), %xmm15
1407ddc15:     	addq	$0x3c8, %rsp            # imm = 0x3C8
1407ddc1c:     	popq	%rbx
1407ddc1d:     	popq	%rdi
1407ddc1e:     	popq	%rsi
1407ddc1f:     	popq	%r12
1407ddc21:     	popq	%r13
1407ddc23:     	popq	%r14
1407ddc25:     	popq	%r15
1407ddc27:     	popq	%rbp
1407ddc28:     	retq
1407ddc29:     	movq	0xd8(%rbp), %rdx
1407ddc30:     	leaq	0xe0(%rbp), %rax
1407ddc37:     	movq	%rax, 0xc8(%rbp)
1407ddc3e:     	testq	%rdx, %rdx
1407ddc41:     	jne	0x1407ddb1e <.text+0x7dcb1e>
1407ddc47:     	jmp	0x1407ddb33 <.text+0x7dcb33>
1407ddc4c:     	movb	$0x1, 0x296(%rbp)
1407ddc53:     	leaq	0x22aa826(%rip), %r8    # 0x142a88480
1407ddc5a:     	movq	%r15, %rcx
1407ddc5d:     	movq	%rsi, %rdx
1407ddc60:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1407ddc65:     	jmp	0x1407ddd2c <.text+0x7dcd2c>
1407ddc6a:     	movq	$0x0, 0x140(%rbp)
1407ddc75:     	movq	$0x4, 0x148(%rbp)
1407ddc80:     	movq	$0x0, 0x150(%rbp)
1407ddc8b:     	leaq	0x22aa7a6(%rip), %r8    # 0x142a88438
1407ddc92:     	movl	$0x1, %ecx
1407ddc97:     	movq	0x210(%rbp), %rdx
1407ddc9e:     	callq	0x1428d9518 <.text+0x28d8518>
1407ddca3:     	jmp	0x1407ddd2c <.text+0x7dcd2c>
1407ddca8:     	movb	$0x1, 0x297(%rbp)
1407ddcaf:     	leaq	0x22aa76a(%rip), %r8    # 0x142a88420
1407ddcb6:     	movl	$0x4, %ecx
1407ddcbb:     	movq	0x200(%rbp), %rdx
1407ddcc2:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1407ddcc7:     	jmp	0x1407ddd2c <.text+0x7dcd2c>
1407ddcc9:     	xorl	%r15d, %r15d
1407ddccc:     	movq	%r12, 0xa8(%rbp)
1407ddcd3:     	movb	$0x1, 0x295(%rbp)
1407ddcda:     	leaq	0x22aa7b7(%rip), %r8    # 0x142a88498
1407ddce1:     	movq	%r15, %rcx
1407ddce4:     	movq	%rsi, %rdx
1407ddce7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1407ddcec:     	jmp	0x1407ddd2c <.text+0x7dcd2c>
1407ddcee:     	movb	$0x1, 0x294(%rbp)
1407ddcf5:     	leaq	0x22aa7b4(%rip), %r8    # 0x142a884b0
1407ddcfc:     	movl	$0x4, %ecx
1407ddd01:     	movq	%rsi, %rdx
1407ddd04:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1407ddd09:     	jmp	0x1407ddd2c <.text+0x7dcd2c>
1407ddd0b:     	leaq	0x22aa7b6(%rip), %r8    # 0x142a884c8
1407ddd12:     	xorl	%ecx, %ecx
1407ddd14:     	xorl	%edx, %edx
1407ddd16:     	callq	0x1428d9518 <.text+0x28d8518>
1407ddd1b:     	jmp	0x1407ddd2c <.text+0x7dcd2c>
1407ddd1d:     	movl	$0x8, %ecx
1407ddd22:     	movl	$0x38, %edx
1407ddd27:     	callq	0x1428d9000 <.text+0x28d8000>
1407ddd2c:     	ud2
1407ddd2e:     	nop
1407ddd30:     	movq	%rdx, 0x10(%rsp)
1407ddd35:     	pushq	%rbp
1407ddd36:     	pushq	%r15
1407ddd38:     	pushq	%r14
1407ddd3a:     	pushq	%r13
1407ddd3c:     	pushq	%r12
1407ddd3e:     	pushq	%rsi
1407ddd3f:     	pushq	%rdi
1407ddd40:     	pushq	%rbx
1407ddd41:     	subq	$0xc8, %rsp
1407ddd48:     	leaq	0x80(%rdx), %rbp
1407ddd4f:     	movaps	%xmm15, 0x20(%rsp)
1407ddd55:     	movaps	%xmm14, 0x30(%rsp)
1407ddd5b:     	movaps	%xmm13, 0x40(%rsp)
1407ddd61:     	movaps	%xmm12, 0x50(%rsp)
1407ddd67:     	movaps	%xmm11, 0x60(%rsp)
1407ddd6d:     	movaps	%xmm10, 0x70(%rsp)
1407ddd73:     	movaps	%xmm9, 0x80(%rsp)
1407ddd7c:     	movaps	%xmm8, 0x90(%rsp)
1407ddd85:     	movaps	%xmm7, 0xa0(%rsp)
1407ddd8d:     	movaps	%xmm6, 0xb0(%rsp)
1407ddd95:     	movq	0x140(%rbp), %rax
1407ddd9c:     	testq	%rax, %rax
1407ddd9f:     	je	0x1407dddbb <.text+0x7dcdbb>
1407ddda1:     	movq	0x148(%rbp), %rcx
1407ddda8:     	shlq	$0x2, %rax
1407dddac:     	leaq	(%rax,%rax,2), %rdx
1407dddb0:     	movl	$0x4, %r8d
1407dddb6:     	callq	0x140613c20 <.text+0x612c20>
1407dddbb:     	movb	$0x1, 0x297(%rbp)
1407dddc2:     	movaps	0xb0(%rsp), %xmm6
1407dddca:     	movaps	0xa0(%rsp), %xmm7
1407dddd2:     	movaps	0x90(%rsp), %xmm8
1407ddddb:     	movaps	0x80(%rsp), %xmm9
1407ddde4:     	movaps	0x70(%rsp), %xmm10
1407dddea:     	movaps	0x60(%rsp), %xmm11
1407dddf0:     	movaps	0x50(%rsp), %xmm12
1407dddf6:     	movaps	0x40(%rsp), %xmm13
1407dddfc:     	movaps	0x30(%rsp), %xmm14
1407dde02:     	movaps	0x20(%rsp), %xmm15
1407dde08:     	addq	$0xc8, %rsp
1407dde0f:     	popq	%rbx
1407dde10:     	popq	%rdi
1407dde11:     	popq	%rsi
1407dde12:     	popq	%r12
1407dde14:     	popq	%r13
1407dde16:     	popq	%r14
1407dde18:     	popq	%r15
1407dde1a:     	popq	%rbp
1407dde1b:     	retq
1407dde1c:     	nopl	(%rax)
1407dde20:     	movq	%rdx, 0x10(%rsp)
1407dde25:     	pushq	%rbp
1407dde26:     	pushq	%r15
1407dde28:     	pushq	%r14
1407dde2a:     	pushq	%r13
1407dde2c:     	pushq	%r12
1407dde2e:     	pushq	%rsi
1407dde2f:     	pushq	%rdi
1407dde30:     	pushq	%rbx
1407dde31:     	subq	$0xc8, %rsp
1407dde38:     	leaq	0x80(%rdx), %rbp
1407dde3f:     	movaps	%xmm15, 0x20(%rsp)
1407dde45:     	movaps	%xmm14, 0x30(%rsp)
1407dde4b:     	movaps	%xmm13, 0x40(%rsp)
1407dde51:     	movaps	%xmm12, 0x50(%rsp)
1407dde57:     	movaps	%xmm11, 0x60(%rsp)
1407dde5d:     	movaps	%xmm10, 0x70(%rsp)
1407dde63:     	movaps	%xmm9, 0x80(%rsp)
1407dde6c:     	movaps	%xmm8, 0x90(%rsp)
1407dde75:     	movaps	%xmm7, 0xa0(%rsp)
1407dde7d:     	movaps	%xmm6, 0xb0(%rsp)
1407dde85:     	movzbl	0x296(%rbp), %ebx
1407dde8c:     	movq	0xc0(%rbp), %rax
1407dde93:     	testq	%rax, %rax
1407dde96:     	je	0x1407ddeb2 <.text+0x7dceb2>
1407dde98:     	shlq	$0x2, %rax
1407dde9c:     	leaq	(%rax,%rax,2), %rdx
1407ddea0:     	movl	$0x4, %r8d
1407ddea6:     	movq	0x208(%rbp), %rcx
1407ddead:     	callq	0x140613c20 <.text+0x612c20>
1407ddeb2:     	andb	$0x1, %bl
1407ddeb5:     	movb	%bl, 0x297(%rbp)
1407ddebb:     	movaps	0xb0(%rsp), %xmm6
1407ddec3:     	movaps	0xa0(%rsp), %xmm7
1407ddecb:     	movaps	0x90(%rsp), %xmm8
1407dded4:     	movaps	0x80(%rsp), %xmm9
1407ddedd:     	movaps	0x70(%rsp), %xmm10
1407ddee3:     	movaps	0x60(%rsp), %xmm11
1407ddee9:     	movaps	0x50(%rsp), %xmm12
1407ddeef:     	movaps	0x40(%rsp), %xmm13
1407ddef5:     	movaps	0x30(%rsp), %xmm14
1407ddefb:     	movaps	0x20(%rsp), %xmm15
1407ddf01:     	addq	$0xc8, %rsp
1407ddf08:     	popq	%rbx
1407ddf09:     	popq	%rdi
1407ddf0a:     	popq	%rsi
1407ddf0b:     	popq	%r12
1407ddf0d:     	popq	%r13
1407ddf0f:     	popq	%r14
1407ddf11:     	popq	%r15
1407ddf13:     	popq	%rbp
1407ddf14:     	retq
1407ddf15:     	nopw	%cs:(%rax,%rax)
1407ddf20:     	movq	%rdx, 0x10(%rsp)
1407ddf25:     	pushq	%rbp
1407ddf26:     	pushq	%r15
1407ddf28:     	pushq	%r14
1407ddf2a:     	pushq	%r13
1407ddf2c:     	pushq	%r12
1407ddf2e:     	pushq	%rsi
1407ddf2f:     	pushq	%rdi
1407ddf30:     	pushq	%rbx
1407ddf31:     	subq	$0xc8, %rsp
1407ddf38:     	leaq	0x80(%rdx), %rbp
1407ddf3f:     	movaps	%xmm15, 0x20(%rsp)
1407ddf45:     	movaps	%xmm14, 0x30(%rsp)
1407ddf4b:     	movaps	%xmm13, 0x40(%rsp)
1407ddf51:     	movaps	%xmm12, 0x50(%rsp)
1407ddf57:     	movaps	%xmm11, 0x60(%rsp)
1407ddf5d:     	movaps	%xmm10, 0x70(%rsp)
1407ddf63:     	movaps	%xmm9, 0x80(%rsp)
1407ddf6c:     	movaps	%xmm8, 0x90(%rsp)
1407ddf75:     	movaps	%xmm7, 0xa0(%rsp)
1407ddf7d:     	movaps	%xmm6, 0xb0(%rsp)
1407ddf85:     	movzbl	0x295(%rbp), %ebx
1407ddf8c:     	movq	0xa8(%rbp), %rdx
1407ddf93:     	testq	%rdx, %rdx
1407ddf96:     	je	0x1407ddfae <.text+0x7dcfae>
1407ddf98:     	movq	0x230(%rbp), %rcx
1407ddf9f:     	shlq	$0x2, %rdx
1407ddfa3:     	movl	$0x4, %r8d
1407ddfa9:     	callq	0x140613c20 <.text+0x612c20>
1407ddfae:     	andb	$0x1, %bl
1407ddfb1:     	movb	%bl, 0x296(%rbp)
1407ddfb7:     	movaps	0xb0(%rsp), %xmm6
1407ddfbf:     	movaps	0xa0(%rsp), %xmm7
1407ddfc7:     	movaps	0x90(%rsp), %xmm8
1407ddfd0:     	movaps	0x80(%rsp), %xmm9
1407ddfd9:     	movaps	0x70(%rsp), %xmm10
1407ddfdf:     	movaps	0x60(%rsp), %xmm11
1407ddfe5:     	movaps	0x50(%rsp), %xmm12
1407ddfeb:     	movaps	0x40(%rsp), %xmm13
1407ddff1:     	movaps	0x30(%rsp), %xmm14
1407ddff7:     	movaps	0x20(%rsp), %xmm15
1407ddffd:     	addq	$0xc8, %rsp
1407de004:     	popq	%rbx
1407de005:     	popq	%rdi
1407de006:     	popq	%rsi
1407de007:     	popq	%r12
1407de009:     	popq	%r13
1407de00b:     	popq	%r14
1407de00d:     	popq	%r15
1407de00f:     	popq	%rbp
1407de010:     	retq
1407de011:     	nopw	%cs:(%rax,%rax)
1407de020:     	movq	%rdx, 0x10(%rsp)
1407de025:     	pushq	%rbp
1407de026:     	pushq	%r15
1407de028:     	pushq	%r14
1407de02a:     	pushq	%r13
1407de02c:     	pushq	%r12
1407de02e:     	pushq	%rsi
1407de02f:     	pushq	%rdi
1407de030:     	pushq	%rbx
1407de031:     	subq	$0xc8, %rsp
1407de038:     	leaq	0x80(%rdx), %rbp
1407de03f:     	movaps	%xmm15, 0x20(%rsp)
1407de045:     	movaps	%xmm14, 0x30(%rsp)
1407de04b:     	movaps	%xmm13, 0x40(%rsp)
1407de051:     	movaps	%xmm12, 0x50(%rsp)
1407de057:     	movaps	%xmm11, 0x60(%rsp)
1407de05d:     	movaps	%xmm10, 0x70(%rsp)
1407de063:     	movaps	%xmm9, 0x80(%rsp)
1407de06c:     	movaps	%xmm8, 0x90(%rsp)
1407de075:     	movaps	%xmm7, 0xa0(%rsp)
1407de07d:     	movaps	%xmm6, 0xb0(%rsp)
1407de085:     	movzbl	0x294(%rbp), %ebx
1407de08c:     	movq	0x270(%rbp), %rax
1407de093:     	testq	%rax, %rax
1407de096:     	je	0x1407de0b2 <.text+0x7dd0b2>
1407de098:     	movq	0x278(%rbp), %rcx
1407de09f:     	shlq	$0x2, %rax
1407de0a3:     	leaq	(%rax,%rax,2), %rdx
1407de0a7:     	movl	$0x4, %r8d
1407de0ad:     	callq	0x140613c20 <.text+0x612c20>
1407de0b2:     	movq	0x228(%rbp), %rax
1407de0b9:     	movq	%rax, 0xa8(%rbp)
1407de0c0:     	andb	$0x1, %bl
1407de0c3:     	movb	%bl, 0x295(%rbp)
1407de0c9:     	movaps	0xb0(%rsp), %xmm6
1407de0d1:     	movaps	0xa0(%rsp), %xmm7
1407de0d9:     	movaps	0x90(%rsp), %xmm8
1407de0e2:     	movaps	0x80(%rsp), %xmm9
1407de0eb:     	movaps	0x70(%rsp), %xmm10
1407de0f1:     	movaps	0x60(%rsp), %xmm11
1407de0f7:     	movaps	0x50(%rsp), %xmm12
1407de0fd:     	movaps	0x40(%rsp), %xmm13
1407de103:     	movaps	0x30(%rsp), %xmm14
1407de109:     	movaps	0x20(%rsp), %xmm15
1407de10f:     	addq	$0xc8, %rsp
1407de116:     	popq	%rbx
1407de117:     	popq	%rdi
1407de118:     	popq	%rsi
1407de119:     	popq	%r12
1407de11b:     	popq	%r13
1407de11d:     	popq	%r14
1407de11f:     	popq	%r15
1407de121:     	popq	%rbp
1407de122:     	retq
1407de123:     	nopw	%cs:(%rax,%rax)
1407de130:     	movq	%rdx, 0x10(%rsp)
1407de135:     	pushq	%rbp
1407de136:     	pushq	%r15
1407de138:     	pushq	%r14
1407de13a:     	pushq	%r13
1407de13c:     	pushq	%r12
1407de13e:     	pushq	%rsi
1407de13f:     	pushq	%rdi
1407de140:     	pushq	%rbx
1407de141:     	subq	$0xc8, %rsp
1407de148:     	leaq	0x80(%rdx), %rbp
1407de14f:     	movaps	%xmm15, 0x20(%rsp)
1407de155:     	movaps	%xmm14, 0x30(%rsp)
1407de15b:     	movaps	%xmm13, 0x40(%rsp)
1407de161:     	movaps	%xmm12, 0x50(%rsp)
1407de167:     	movaps	%xmm11, 0x60(%rsp)
1407de16d:     	movaps	%xmm10, 0x70(%rsp)
1407de173:     	movaps	%xmm9, 0x80(%rsp)
1407de17c:     	movaps	%xmm8, 0x90(%rsp)
1407de185:     	movaps	%xmm7, 0xa0(%rsp)
1407de18d:     	movaps	%xmm6, 0xb0(%rsp)
1407de195:     	movq	0x250(%rbp), %rax
1407de19c:     	testq	%rax, %rax
1407de19f:     	je	0x1407de1bb <.text+0x7dd1bb>
1407de1a1:     	movq	0x258(%rbp), %rcx
1407de1a8:     	shlq	$0x2, %rax
1407de1ac:     	leaq	(%rax,%rax,2), %rdx
1407de1b0:     	movl	$0x4, %r8d
1407de1b6:     	callq	0x140613c20 <.text+0x612c20>
1407de1bb:     	movzbl	0x26f(%rbp), %eax
1407de1c2:     	movb	%al, 0x294(%rbp)
1407de1c8:     	movaps	0xb0(%rsp), %xmm6
1407de1d0:     	movaps	0xa0(%rsp), %xmm7
1407de1d8:     	movaps	0x90(%rsp), %xmm8
1407de1e1:     	movaps	0x80(%rsp), %xmm9
1407de1ea:     	movaps	0x70(%rsp), %xmm10
1407de1f0:     	movaps	0x60(%rsp), %xmm11
1407de1f6:     	movaps	0x50(%rsp), %xmm12
1407de1fc:     	movaps	0x40(%rsp), %xmm13
1407de202:     	movaps	0x30(%rsp), %xmm14
1407de208:     	movaps	0x20(%rsp), %xmm15
1407de20e:     	addq	$0xc8, %rsp
1407de215:     	popq	%rbx
1407de216:     	popq	%rdi
1407de217:     	popq	%rsi
1407de218:     	popq	%r12
1407de21a:     	popq	%r13
1407de21c:     	popq	%r14
1407de21e:     	popq	%r15
1407de220:     	popq	%rbp
1407de221:     	retq
1407de222:     	nopw	%cs:(%rax,%rax)
1407de230:     	movq	%rdx, 0x10(%rsp)
1407de235:     	pushq	%rbp
1407de236:     	pushq	%r15
1407de238:     	pushq	%r14
1407de23a:     	pushq	%r13
1407de23c:     	pushq	%r12
1407de23e:     	pushq	%rsi
1407de23f:     	pushq	%rdi
1407de240:     	pushq	%rbx
1407de241:     	subq	$0xc8, %rsp
1407de248:     	leaq	0x80(%rdx), %rbp
1407de24f:     	movaps	%xmm15, 0x20(%rsp)
1407de255:     	movaps	%xmm14, 0x30(%rsp)
1407de25b:     	movaps	%xmm13, 0x40(%rsp)
1407de261:     	movaps	%xmm12, 0x50(%rsp)
1407de267:     	movaps	%xmm11, 0x60(%rsp)
1407de26d:     	movaps	%xmm10, 0x70(%rsp)
1407de273:     	movaps	%xmm9, 0x80(%rsp)
1407de27c:     	movaps	%xmm8, 0x90(%rsp)
1407de285:     	movaps	%xmm7, 0xa0(%rsp)
1407de28d:     	movaps	%xmm6, 0xb0(%rsp)
1407de295:     	cmpq	$0x0, 0x138(%rbp)
1407de29d:     	je	0x1407de2c0 <.text+0x7dd2c0>
1407de29f:     	movq	0x138(%rbp), %rax
1407de2a6:     	shlq	$0x2, %rax
1407de2aa:     	leaq	(%rax,%rax,2), %rdx
1407de2ae:     	movl	$0x4, %r8d
1407de2b4:     	movq	0x1c8(%rbp), %rcx
1407de2bb:     	callq	0x140613c20 <.text+0x612c20>
1407de2c0:     	movaps	0xb0(%rsp), %xmm6
1407de2c8:     	movaps	0xa0(%rsp), %xmm7
1407de2d0:     	movaps	0x90(%rsp), %xmm8
1407de2d9:     	movaps	0x80(%rsp), %xmm9
1407de2e2:     	movaps	0x70(%rsp), %xmm10
1407de2e8:     	movaps	0x60(%rsp), %xmm11
1407de2ee:     	movaps	0x50(%rsp), %xmm12
1407de2f4:     	movaps	0x40(%rsp), %xmm13
1407de2fa:     	movaps	0x30(%rsp), %xmm14
1407de300:     	movaps	0x20(%rsp), %xmm15
1407de306:     	addq	$0xc8, %rsp
1407de30d:     	popq	%rbx
1407de30e:     	popq	%rdi
1407de30f:     	popq	%rsi
1407de310:     	popq	%r12
1407de312:     	popq	%r13
1407de314:     	popq	%r14
1407de316:     	popq	%r15
1407de318:     	popq	%rbp
1407de319:     	retq
1407de31a:     	nopw	(%rax,%rax)
1407de320:     	movq	%rdx, 0x10(%rsp)
1407de325:     	pushq	%rbp
1407de326:     	pushq	%r15
1407de328:     	pushq	%r14
1407de32a:     	pushq	%r13
1407de32c:     	pushq	%r12
1407de32e:     	pushq	%rsi
1407de32f:     	pushq	%rdi
1407de330:     	pushq	%rbx
1407de331:     	subq	$0xc8, %rsp
1407de338:     	leaq	0x80(%rdx), %rbp
1407de33f:     	movaps	%xmm15, 0x20(%rsp)
1407de345:     	movaps	%xmm14, 0x30(%rsp)
1407de34b:     	movaps	%xmm13, 0x40(%rsp)
1407de351:     	movaps	%xmm12, 0x50(%rsp)
1407de357:     	movaps	%xmm11, 0x60(%rsp)
1407de35d:     	movaps	%xmm10, 0x70(%rsp)
1407de363:     	movaps	%xmm9, 0x80(%rsp)
1407de36c:     	movaps	%xmm8, 0x90(%rsp)
1407de375:     	movaps	%xmm7, 0xa0(%rsp)
1407de37d:     	movaps	%xmm6, 0xb0(%rsp)
1407de385:     	leaq	0x150(%rbp), %rcx
1407de38c:     	callq	0x140be24b0 <.text+0xbe14b0>
1407de391:     	movaps	0xb0(%rsp), %xmm6
1407de399:     	movaps	0xa0(%rsp), %xmm7
1407de3a1:     	movaps	0x90(%rsp), %xmm8
1407de3aa:     	movaps	0x80(%rsp), %xmm9
1407de3b3:     	movaps	0x70(%rsp), %xmm10
1407de3b9:     	movaps	0x60(%rsp), %xmm11
1407de3bf:     	movaps	0x50(%rsp), %xmm12
1407de3c5:     	movaps	0x40(%rsp), %xmm13
1407de3cb:     	movaps	0x30(%rsp), %xmm14
1407de3d1:     	movaps	0x20(%rsp), %xmm15
1407de3d7:     	addq	$0xc8, %rsp
1407de3de:     	popq	%rbx
1407de3df:     	popq	%rdi
1407de3e0:     	popq	%rsi
1407de3e1:     	popq	%r12
1407de3e3:     	popq	%r13
1407de3e5:     	popq	%r14
1407de3e7:     	popq	%r15
1407de3e9:     	popq	%rbp
1407de3ea:     	retq
1407de3eb:     	nopl	(%rax,%rax)
1407de3f0:     	movq	%rdx, 0x10(%rsp)
1407de3f5:     	pushq	%rbp
1407de3f6:     	pushq	%r15
1407de3f8:     	pushq	%r14
1407de3fa:     	pushq	%r13
1407de3fc:     	pushq	%r12
1407de3fe:     	pushq	%rsi
1407de3ff:     	pushq	%rdi
1407de400:     	pushq	%rbx
1407de401:     	subq	$0xc8, %rsp
1407de408:     	leaq	0x80(%rdx), %rbp
1407de40f:     	movaps	%xmm15, 0x20(%rsp)
1407de415:     	movaps	%xmm14, 0x30(%rsp)
1407de41b:     	movaps	%xmm13, 0x40(%rsp)
1407de421:     	movaps	%xmm12, 0x50(%rsp)
1407de427:     	movaps	%xmm11, 0x60(%rsp)
1407de42d:     	movaps	%xmm10, 0x70(%rsp)
1407de433:     	movaps	%xmm9, 0x80(%rsp)
1407de43c:     	movaps	%xmm8, 0x90(%rsp)
1407de445:     	movaps	%xmm7, 0xa0(%rsp)
1407de44d:     	movaps	%xmm6, 0xb0(%rsp)
1407de455:     	cmpq	$0x0, 0x210(%rbp)
1407de45d:     	je	0x1407de47c <.text+0x7dd47c>
1407de45f:     	movq	0x210(%rbp), %rdx
1407de466:     	shlq	$0x5, %rdx
1407de46a:     	movl	$0x8, %r8d
1407de470:     	movq	0x248(%rbp), %rcx
1407de477:     	callq	0x140613c20 <.text+0x612c20>
1407de47c:     	movq	0x1f8(%rbp), %rax
1407de483:     	lock
1407de484:     	decq	(%rax)
1407de487:     	jne	0x1407de495 <.text+0x7dd495>
1407de489:     	leaq	0x1f8(%rbp), %rcx
1407de490:     	callq	0x140b5cc00 <.text+0xb5bc00>
1407de495:     	movaps	0xb0(%rsp), %xmm6
1407de49d:     	movaps	0xa0(%rsp), %xmm7
1407de4a5:     	movaps	0x90(%rsp), %xmm8
1407de4ae:     	movaps	0x80(%rsp), %xmm9
1407de4b7:     	movaps	0x70(%rsp), %xmm10
1407de4bd:     	movaps	0x60(%rsp), %xmm11
1407de4c3:     	movaps	0x50(%rsp), %xmm12
1407de4c9:     	movaps	0x40(%rsp), %xmm13
1407de4cf:     	movaps	0x30(%rsp), %xmm14
1407de4d5:     	movaps	0x20(%rsp), %xmm15
1407de4db:     	addq	$0xc8, %rsp
1407de4e2:     	popq	%rbx
1407de4e3:     	popq	%rdi
1407de4e4:     	popq	%rsi
1407de4e5:     	popq	%r12
1407de4e7:     	popq	%r13
1407de4e9:     	popq	%r14
1407de4eb:     	popq	%r15
1407de4ed:     	popq	%rbp
1407de4ee:     	retq
1407de4ef:     	nop
1407de4f0:     	movq	%rdx, 0x10(%rsp)
1407de4f5:     	pushq	%rbp
1407de4f6:     	pushq	%r15
1407de4f8:     	pushq	%r14
1407de4fa:     	pushq	%r13
1407de4fc:     	pushq	%r12
1407de4fe:     	pushq	%rsi
1407de4ff:     	pushq	%rdi
1407de500:     	pushq	%rbx
1407de501:     	subq	$0xc8, %rsp
1407de508:     	leaq	0x80(%rdx), %rbp
1407de50f:     	movaps	%xmm15, 0x20(%rsp)
1407de515:     	movaps	%xmm14, 0x30(%rsp)
1407de51b:     	movaps	%xmm13, 0x40(%rsp)
1407de521:     	movaps	%xmm12, 0x50(%rsp)
1407de527:     	movaps	%xmm11, 0x60(%rsp)
1407de52d:     	movaps	%xmm10, 0x70(%rsp)
1407de533:     	movaps	%xmm9, 0x80(%rsp)
1407de53c:     	movaps	%xmm8, 0x90(%rsp)
1407de545:     	movaps	%xmm7, 0xa0(%rsp)
1407de54d:     	movaps	%xmm6, 0xb0(%rsp)
1407de555:     	cmpq	$0x0, 0x1d0(%rbp)
1407de55d:     	jne	0x1407de5e4 <.text+0x7dd5e4>
1407de563:     	cmpq	$0x0, 0x210(%rbp)
1407de56b:     	je	0x1407de58a <.text+0x7dd58a>
1407de56d:     	movq	0x210(%rbp), %rdx
1407de574:     	shlq	$0x5, %rdx
1407de578:     	movl	$0x8, %r8d
1407de57e:     	movq	0x248(%rbp), %rcx
1407de585:     	callq	0x140613c20 <.text+0x612c20>
1407de58a:     	movaps	0xb0(%rsp), %xmm6
1407de592:     	movaps	0xa0(%rsp), %xmm7
1407de59a:     	movaps	0x90(%rsp), %xmm8
1407de5a3:     	movaps	0x80(%rsp), %xmm9
1407de5ac:     	movaps	0x70(%rsp), %xmm10
1407de5b2:     	movaps	0x60(%rsp), %xmm11
1407de5b8:     	movaps	0x50(%rsp), %xmm12
1407de5be:     	movaps	0x40(%rsp), %xmm13
1407de5c4:     	movaps	0x30(%rsp), %xmm14
1407de5ca:     	movaps	0x20(%rsp), %xmm15
1407de5d0:     	addq	$0xc8, %rsp
1407de5d7:     	popq	%rbx
1407de5d8:     	popq	%rdi
1407de5d9:     	popq	%rsi
1407de5da:     	popq	%r12
1407de5dc:     	popq	%r13
1407de5de:     	popq	%r14
1407de5e0:     	popq	%r15
1407de5e2:     	popq	%rbp
1407de5e3:     	retq
1407de5e4:     	movaps	0xb0(%rsp), %xmm6
1407de5ec:     	movaps	0xa0(%rsp), %xmm7
1407de5f4:     	movaps	0x90(%rsp), %xmm8
1407de5fd:     	movaps	0x80(%rsp), %xmm9
1407de606:     	movaps	0x70(%rsp), %xmm10
1407de60c:     	movaps	0x60(%rsp), %xmm11
1407de612:     	movaps	0x50(%rsp), %xmm12
1407de618:     	movaps	0x40(%rsp), %xmm13
1407de61e:     	movaps	0x30(%rsp), %xmm14
1407de624:     	movaps	0x20(%rsp), %xmm15
1407de62a:     	addq	$0xc8, %rsp
1407de631:     	popq	%rbx
1407de632:     	popq	%rdi
1407de633:     	popq	%rsi
1407de634:     	popq	%r12
1407de636:     	popq	%r13
1407de638:     	popq	%r14
1407de63a:     	popq	%r15
1407de63c:     	popq	%rbp
1407de63d:     	retq
1407de63e:     	nop
1407de640:     	movq	%rdx, 0x10(%rsp)
1407de645:     	pushq	%rbp
1407de646:     	pushq	%r15
1407de648:     	pushq	%r14
1407de64a:     	pushq	%r13
1407de64c:     	pushq	%r12
1407de64e:     	pushq	%rsi
1407de64f:     	pushq	%rdi
1407de650:     	pushq	%rbx
1407de651:     	subq	$0xc8, %rsp
1407de658:     	leaq	0x80(%rdx), %rbp
1407de65f:     	movaps	%xmm15, 0x20(%rsp)
1407de665:     	movaps	%xmm14, 0x30(%rsp)
1407de66b:     	movaps	%xmm13, 0x40(%rsp)
1407de671:     	movaps	%xmm12, 0x50(%rsp)
1407de677:     	movaps	%xmm11, 0x60(%rsp)
1407de67d:     	movaps	%xmm10, 0x70(%rsp)
1407de683:     	movaps	%xmm9, 0x80(%rsp)
1407de68c:     	movaps	%xmm8, 0x90(%rsp)
1407de695:     	movaps	%xmm7, 0xa0(%rsp)
1407de69d:     	movaps	%xmm6, 0xb0(%rsp)
1407de6a5:     	leaq	0xd0(%rbp), %rcx
1407de6ac:     	callq	0x1407cea70 <.text+0x7cda70>
1407de6b1:     	movaps	0xb0(%rsp), %xmm6
1407de6b9:     	movaps	0xa0(%rsp), %xmm7
1407de6c1:     	movaps	0x90(%rsp), %xmm8
1407de6ca:     	movaps	0x80(%rsp), %xmm9
1407de6d3:     	movaps	0x70(%rsp), %xmm10
1407de6d9:     	movaps	0x60(%rsp), %xmm11
1407de6df:     	movaps	0x50(%rsp), %xmm12
1407de6e5:     	movaps	0x40(%rsp), %xmm13
1407de6eb:     	movaps	0x30(%rsp), %xmm14
1407de6f1:     	movaps	0x20(%rsp), %xmm15
1407de6f7:     	addq	$0xc8, %rsp
1407de6fe:     	popq	%rbx
1407de6ff:     	popq	%rdi
1407de700:     	popq	%rsi
1407de701:     	popq	%r12
1407de703:     	popq	%r13
1407de705:     	popq	%r14
1407de707:     	popq	%r15
1407de709:     	popq	%rbp
1407de70a:     	retq
1407de70b:     	nopl	(%rax,%rax)
1407de710:     	movq	%rdx, 0x10(%rsp)
1407de715:     	pushq	%rbp
1407de716:     	pushq	%r15
1407de718:     	pushq	%r14
1407de71a:     	pushq	%r13
1407de71c:     	pushq	%r12
1407de71e:     	pushq	%rsi
1407de71f:     	pushq	%rdi
1407de720:     	pushq	%rbx
1407de721:     	subq	$0xc8, %rsp
1407de728:     	leaq	0x80(%rdx), %rbp
1407de72f:     	movaps	%xmm15, 0x20(%rsp)
1407de735:     	movaps	%xmm14, 0x30(%rsp)
1407de73b:     	movaps	%xmm13, 0x40(%rsp)
1407de741:     	movaps	%xmm12, 0x50(%rsp)
1407de747:     	movaps	%xmm11, 0x60(%rsp)
1407de74d:     	movaps	%xmm10, 0x70(%rsp)
1407de753:     	movaps	%xmm9, 0x80(%rsp)
1407de75c:     	movaps	%xmm8, 0x90(%rsp)
1407de765:     	movaps	%xmm7, 0xa0(%rsp)
1407de76d:     	movaps	%xmm6, 0xb0(%rsp)
1407de775:     	cmpb	$0x0, 0x297(%rbp)
1407de77c:     	je	0x1407de79e <.text+0x7dd79e>
1407de77e:     	movq	-0x10(%rbp), %rax
1407de782:     	testq	%rax, %rax
1407de785:     	je	0x1407de79e <.text+0x7dd79e>
1407de787:     	movq	-0x8(%rbp), %rcx
1407de78b:     	shlq	$0x2, %rax
1407de78f:     	leaq	(%rax,%rax,2), %rdx
1407de793:     	movl	$0x4, %r8d
1407de799:     	callq	0x140613c20 <.text+0x612c20>
1407de79e:     	movaps	0xb0(%rsp), %xmm6
1407de7a6:     	movaps	0xa0(%rsp), %xmm7
1407de7ae:     	movaps	0x90(%rsp), %xmm8
1407de7b7:     	movaps	0x80(%rsp), %xmm9
1407de7c0:     	movaps	0x70(%rsp), %xmm10
1407de7c6:     	movaps	0x60(%rsp), %xmm11
1407de7cc:     	movaps	0x50(%rsp), %xmm12
1407de7d2:     	movaps	0x40(%rsp), %xmm13
1407de7d8:     	movaps	0x30(%rsp), %xmm14
1407de7de:     	movaps	0x20(%rsp), %xmm15
1407de7e4:     	addq	$0xc8, %rsp
1407de7eb:     	popq	%rbx
1407de7ec:     	popq	%rdi
1407de7ed:     	popq	%rsi
1407de7ee:     	popq	%r12
1407de7f0:     	popq	%r13
1407de7f2:     	popq	%r14
1407de7f4:     	popq	%r15
1407de7f6:     	popq	%rbp
1407de7f7:     	retq
