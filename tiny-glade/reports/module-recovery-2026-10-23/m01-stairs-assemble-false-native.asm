
../B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140018040:     	pushq	%rbp
140018041:     	pushq	%r15
140018043:     	pushq	%r14
140018045:     	pushq	%r13
140018047:     	pushq	%r12
140018049:     	pushq	%rsi
14001804a:     	pushq	%rdi
14001804b:     	pushq	%rbx
14001804c:     	subq	$0x948, %rsp            # imm = 0x948
140018053:     	leaq	0x80(%rsp), %rbp
14001805b:     	movdqa	%xmm15, 0x8b0(%rbp)
140018064:     	movaps	%xmm14, 0x8a0(%rbp)
14001806c:     	movdqa	%xmm13, 0x890(%rbp)
140018075:     	movdqa	%xmm12, 0x880(%rbp)
14001807e:     	movaps	%xmm11, 0x870(%rbp)
140018086:     	movaps	%xmm10, 0x860(%rbp)
14001808e:     	movaps	%xmm9, 0x850(%rbp)
140018096:     	movaps	%xmm8, 0x840(%rbp)
14001809e:     	movaps	%xmm7, 0x830(%rbp)
1400180a5:     	movaps	%xmm6, 0x820(%rbp)
1400180ac:     	movq	$-0x2, 0x818(%rbp)
1400180b7:     	movq	%r9, %r15
1400180ba:     	movq	%r8, 0x368(%rbp)
1400180c1:     	movq	%rdx, 0x268(%rbp)
1400180c8:     	movq	%rcx, %rdi
1400180cb:     	movq	0x930(%rbp), %rax
1400180d2:     	movq	(%rax), %rcx
1400180d5:     	movq	%rcx, 0x7b0(%rbp)
1400180dc:     	movq	0x10(%rax), %r12
1400180e0:     	movl	0x1c(%rax), %r13d
1400180e4:     	movq	0x20(%rax), %rcx
1400180e8:     	movq	%rcx, 0x2e0(%rbp)
1400180ef:     	movq	0x30(%rax), %rcx
1400180f3:     	movq	%rcx, 0x90(%rbp)
1400180fa:     	movl	0x3c(%rax), %ecx
1400180fd:     	movl	%ecx, 0x238(%rbp)
140018103:     	movq	0x40(%rax), %rcx
140018107:     	movq	%rcx, 0x360(%rbp)
14001810e:     	movq	0x50(%rax), %rcx
140018112:     	movq	%rcx, 0x330(%rbp)
140018119:     	movl	0x5c(%rax), %ecx
14001811c:     	movl	%ecx, 0x534(%rbp)
140018122:     	movq	0x60(%rax), %rcx
140018126:     	movq	%rcx, 0x338(%rbp)
14001812d:     	movq	0x70(%rax), %rcx
140018131:     	movq	%rcx, 0x258(%rbp)
140018138:     	movl	0x7c(%rax), %ecx
14001813b:     	movl	%ecx, 0x424(%rbp)
140018141:     	movq	0x80(%rax), %rcx
140018148:     	movq	%rcx, 0x240(%rbp)
14001814f:     	movq	0xa0(%rax), %rax
140018156:     	movq	%rax, 0x328(%rbp)
14001815d:     	movq	0x960(%rbp), %rax
140018164:     	movq	(%rax), %rcx
140018167:     	movq	%rcx, 0xe8(%rbp)
14001816e:     	movq	0x20(%rax), %rcx
140018172:     	movq	%rcx, 0x260(%rbp)
140018179:     	movq	0x28(%rax), %rax
14001817d:     	movq	%rax, 0x250(%rbp)
140018184:     	callq	0x140c16a00 <.text+0xc15a00>
140018189:     	movb	%al, 0x79f(%rbp)
14001818f:     	testb	%al, %al
140018191:     	je	0x1400183c7 <.text+0x173c7>
140018197:     	movl	$0x4f, %r14d
14001819d:     	movl	$0x4f, %ecx
1400181a2:     	callq	0x1400299b0 <.text+0x289b0>
1400181a7:     	leaq	0x290f202(%rip), %rsi   # 0x1429273b0
1400181ae:     	cmpq	$0x1, %rax
1400181b2:     	jne	0x140018259 <.text+0x17259>
1400181b8:     	movq	%rdx, %r9
1400181bb:     	testq	%rdx, %rdx
1400181be:     	je	0x1400181f6 <.text+0x171f6>
1400181c0:     	cmpq	$0x4f, %r9
1400181c4:     	jae	0x1400181f4 <.text+0x171f4>
1400181c6:     	leaq	0x290f1e3(%rip), %rax   # 0x1429273b0
1400181cd:     	cmpb	$-0x41, (%r9,%rax)
1400181d2:     	jg	0x1400181f6 <.text+0x171f6>
1400181d4:     	leaq	0x290f29d(%rip), %rax   # 0x142927478
1400181db:     	movq	%rax, 0x20(%rsp)
1400181e0:     	leaq	0x290f1c9(%rip), %rcx   # 0x1429273b0
1400181e7:     	movl	$0x4f, %edx
1400181ec:     	xorl	%r8d, %r8d
1400181ef:     	callq	0x1428d9940 <.text+0x28d8940>
1400181f4:     	jne	0x1400181d4 <.text+0x171d4>
1400181f6:     	movq	%r9, %rcx
1400181f9:     	callq	0x1400299b0 <.text+0x289b0>
1400181fe:     	cmpq	$0x1, %rax
140018202:     	jne	0x140018259 <.text+0x17259>
140018204:     	movq	%rdx, %r8
140018207:     	addq	$0x2, %r8
14001820b:     	je	0x140018246 <.text+0x17246>
14001820d:     	cmpq	$0x4f, %r8
140018211:     	jae	0x140018244 <.text+0x17244>
140018213:     	leaq	0x290f196(%rip), %rax   # 0x1429273b0
14001821a:     	cmpb	$-0x41, (%r8,%rax)
14001821f:     	jg	0x140018246 <.text+0x17246>
140018221:     	leaq	0x290f268(%rip), %rax   # 0x142927490
140018228:     	movq	%rax, 0x20(%rsp)
14001822d:     	leaq	0x290f17c(%rip), %rcx   # 0x1429273b0
140018234:     	movl	$0x4f, %edx
140018239:     	movl	$0x4f, %r9d
14001823f:     	callq	0x1428d9940 <.text+0x28d8940>
140018244:     	jne	0x140018221 <.text+0x17221>
140018246:     	movl	$0x4d, %r14d
14001824c:     	subq	%rdx, %r14
14001824f:     	leaq	0x290f15a(%rip), %rsi   # 0x1429273b0
140018256:     	addq	%r8, %rsi
140018259:     	movq	%r15, 0x808(%rbp)
140018260:     	leaq	0x290e241(%rip), %r8    # 0x1429264a8
140018267:     	leaq	0x290e1fa(%rip), %r15   # 0x142926468
14001826e:     	nop
140018270:     	movsbl	-0x1(%r8), %eax
140018275:     	testl	%eax, %eax
140018277:     	js	0x140018290 <.text+0x17290>
140018279:     	decq	%r8
14001827c:     	cmpl	$0x5c, %eax
14001827f:     	jne	0x1400182e4 <.text+0x172e4>
140018281:     	jmp	0x1400182fc <.text+0x172fc>
140018283:     	nopw	%cs:(%rax,%rax)
140018290:     	movzbl	-0x2(%r8), %ecx
140018295:     	cmpb	$-0x40, %cl
140018298:     	jge	0x1400182bd <.text+0x172bd>
14001829a:     	movzbl	-0x3(%r8), %edx
14001829f:     	cmpb	$-0x40, %dl
1400182a2:     	jge	0x1400182c6 <.text+0x172c6>
1400182a4:     	movzbl	-0x4(%r8), %r9d
1400182a9:     	addq	$-0x4, %r8
1400182ad:     	andl	$0x7, %r9d
1400182b1:     	shll	$0x6, %r9d
1400182b5:     	andl	$0x3f, %edx
1400182b8:     	orl	%r9d, %edx
1400182bb:     	jmp	0x1400182cd <.text+0x172cd>
1400182bd:     	addq	$-0x2, %r8
1400182c1:     	andl	$0x1f, %ecx
1400182c4:     	jmp	0x1400182d5 <.text+0x172d5>
1400182c6:     	addq	$-0x3, %r8
1400182ca:     	andl	$0xf, %edx
1400182cd:     	shll	$0x6, %edx
1400182d0:     	andl	$0x3f, %ecx
1400182d3:     	orl	%edx, %ecx
1400182d5:     	shll	$0x6, %ecx
1400182d8:     	andb	$0x3f, %al
1400182da:     	movzbl	%al, %eax
1400182dd:     	orl	%ecx, %eax
1400182df:     	cmpl	$0x5c, %eax
1400182e2:     	je	0x1400182fc <.text+0x172fc>
1400182e4:     	cmpl	$0x2f, %eax
1400182e7:     	je	0x1400182fc <.text+0x172fc>
1400182e9:     	cmpq	%r15, %r8
1400182ec:     	jne	0x140018270 <.text+0x17270>
1400182ee:     	movl	%r13d, %ebx
1400182f1:     	movq	%r12, %r13
1400182f4:     	movl	$0x40, %r12d
1400182fa:     	jmp	0x14001834f <.text+0x1734f>
1400182fc:     	leaq	0x290e165(%rip), %r15   # 0x142926468
140018303:     	subq	%r15, %r8
140018306:     	incq	%r8
140018309:     	je	0x14001833d <.text+0x1733d>
14001830b:     	cmpq	$0x40, %r8
14001830f:     	jae	0x14001833b <.text+0x1733b>
140018311:     	cmpb	$-0x41, (%r8,%r15)
140018316:     	jg	0x14001833d <.text+0x1733d>
140018318:     	leaq	0x290f139(%rip), %rax   # 0x142927458
14001831f:     	movq	%rax, 0x20(%rsp)
140018324:     	leaq	0x290e13d(%rip), %rcx   # 0x142926468
14001832b:     	movl	$0x40, %edx
140018330:     	movl	$0x40, %r9d
140018336:     	callq	0x1428d9940 <.text+0x28d8940>
14001833b:     	jne	0x140018318 <.text+0x17318>
14001833d:     	movl	%r13d, %ebx
140018340:     	movq	%r12, %r13
140018343:     	movl	$0x40, %r12d
140018349:     	subq	%r8, %r12
14001834c:     	addq	%r8, %r15
14001834f:     	callq	0x140c18130 <.text+0xc17130>
140018354:     	movq	%rax, %rcx
140018357:     	movq	(%rax), %rax
14001835a:     	cmpq	$0x1, %rax
14001835e:     	jne	0x14002045e <.text+0x1f45e>
140018364:     	addq	$0x8, %rcx
140018368:     	cmpq	$0x0, (%rcx)
14001836c:     	jne	0x1400206e0 <.text+0x1f6e0>
140018372:     	movq	$-0x1, (%rcx)
140018379:     	movq	%rcx, 0x7a0(%rbp)
140018380:     	addq	$0x8, %rcx
140018384:     	movq	%r12, 0x20(%rsp)
140018389:     	movq	$0x0, 0x30(%rsp)
140018392:     	movq	$0x1, 0x28(%rsp)
14001839b:     	movq	%rsi, %rdx
14001839e:     	movq	%r14, %r8
1400183a1:     	movq	%r15, %r9
1400183a4:     	callq	0x140c16d40 <.text+0xc15d40>
1400183a9:     	movq	%rax, 0x270(%rbp)
1400183b0:     	movq	0x7a0(%rbp), %rax
1400183b7:     	incq	(%rax)
1400183ba:     	movq	%r13, %r12
1400183bd:     	movl	%ebx, %r13d
1400183c0:     	movq	0x808(%rbp), %r15
1400183c7:     	leaq	0x2995df2(%rip), %rdx   # 0x1429ae1c0
1400183ce:     	movq	%rdi, %rcx
1400183d1:     	callq	0x1405ca870 <.text+0x5c9870>
1400183d6:     	movq	%rax, %rbx
1400183d9:     	testq	%rax, %rax
1400183dc:     	je	0x140020a55 <.text+0x1fa55>
1400183e2:     	leaq	0x29e4ac8(%rip), %rax   # 0x1429fceb1
1400183e9:     	movq	%rax, 0x6c0(%rbp)
1400183f0:     	movq	$0x3a, 0x6c8(%rbp)
1400183fb:     	movq	%rbx, %r14
1400183fe:     	addq	$0x10, %r14
140018402:     	cmpq	$0x3a, 0x18(%rbx)
140018407:     	jne	0x1400205f4 <.text+0x1f5f4>
14001840d:     	movq	(%r14), %rdx
140018410:     	leaq	0x29e4a9a(%rip), %rcx   # 0x1429fceb1
140018417:     	movl	$0x3a, %r8d
14001841d:     	callq	0x142923bf0 <.text+0x2922bf0>
140018422:     	testl	%eax, %eax
140018424:     	jne	0x1400205f4 <.text+0x1f5f4>
14001842a:     	movq	(%rbx), %rcx
14001842d:     	movq	0x8(%rbx), %rax
140018431:     	callq	*0x18(%rax)
140018434:     	movq	%rax, 0x120(%rbp)
14001843b:     	cmpq	$0x0, 0x120(%rbp)
140018443:     	je	0x140020a55 <.text+0x1fa55>
140018449:     	leaq	0x2995d68(%rip), %rdx   # 0x1429ae1b8
140018450:     	movq	%rdi, %rcx
140018453:     	callq	0x1405ca870 <.text+0x5c9870>
140018458:     	movq	%rax, %rbx
14001845b:     	testq	%rax, %rax
14001845e:     	je	0x140020a66 <.text+0x1fa66>
140018464:     	leaq	0x29e47ea(%rip), %rax   # 0x1429fcc55
14001846b:     	movq	%rax, 0x6c0(%rbp)
140018472:     	movq	$0x3d, 0x6c8(%rbp)
14001847d:     	movq	%rbx, %r14
140018480:     	addq	$0x10, %r14
140018484:     	cmpq	$0x3d, 0x18(%rbx)
140018489:     	jne	0x140020628 <.text+0x1f628>
14001848f:     	movq	(%r14), %rdx
140018492:     	leaq	0x29e47bc(%rip), %rcx   # 0x1429fcc55
140018499:     	movl	$0x3d, %r8d
14001849f:     	callq	0x142923bf0 <.text+0x2922bf0>
1400184a4:     	testl	%eax, %eax
1400184a6:     	jne	0x140020628 <.text+0x1f628>
1400184ac:     	movq	(%rbx), %rcx
1400184af:     	movq	0x8(%rbx), %rax
1400184b3:     	callq	*0x18(%rax)
1400184b6:     	movq	%rax, 0x128(%rbp)
1400184bd:     	cmpq	$0x0, 0x128(%rbp)
1400184c5:     	je	0x140020a66 <.text+0x1fa66>
1400184cb:     	leaq	0x2995cbe(%rip), %rdx   # 0x1429ae190
1400184d2:     	movq	%rdi, %rcx
1400184d5:     	callq	0x1405ca870 <.text+0x5c9870>
1400184da:     	movq	%rax, %rdi
1400184dd:     	testq	%rax, %rax
1400184e0:     	je	0x140020a77 <.text+0x1fa77>
1400184e6:     	leaq	0x29e473b(%rip), %rax   # 0x1429fcc28
1400184ed:     	movq	%rax, 0x6c0(%rbp)
1400184f4:     	movq	$0x2d, 0x6c8(%rbp)
1400184ff:     	movq	%rdi, %rbx
140018502:     	addq	$0x10, %rbx
140018506:     	cmpq	$0x2d, 0x18(%rdi)
14001850b:     	jne	0x14002065c <.text+0x1f65c>
140018511:     	movq	(%rbx), %rdx
140018514:     	leaq	0x29e470d(%rip), %rcx   # 0x1429fcc28
14001851b:     	movl	$0x2d, %r8d
140018521:     	callq	0x142923bf0 <.text+0x2922bf0>
140018526:     	testl	%eax, %eax
140018528:     	jne	0x14002065c <.text+0x1f65c>
14001852e:     	movq	(%rdi), %rcx
140018531:     	movq	0x8(%rdi), %rax
140018535:     	callq	*0x18(%rax)
140018538:     	movq	%rax, 0x130(%rbp)
14001853f:     	cmpq	$0x0, 0x130(%rbp)
140018547:     	je	0x140020a77 <.text+0x1fa77>
14001854d:     	movq	0x948(%rbp), %rax
140018554:     	movq	0x940(%rbp), %rcx
14001855b:     	movl	%r13d, (%r12)
14001855f:     	movq	%rcx, %r13
140018562:     	movq	0x7b0(%rbp), %rcx
140018569:     	leaq	0x198(%rcx), %rdx
140018570:     	movq	%rdx, 0x2d0(%rbp)
140018577:     	movq	0x8(%rcx), %rbx
14001857b:     	imulq	$0x150, 0x10(%rcx), %rdx # imm = 0x150
140018583:     	addq	%rbx, %rdx
140018586:     	movq	%rdx, 0xf0(%rbp)
14001858d:     	movl	0x1c(%rax), %edx
140018590:     	movl	%edx, 0x23c(%rbp)
140018596:     	movq	(%rax), %rdx
140018599:     	movq	%rdx, 0x98(%rbp)
1400185a0:     	movq	0x10(%rax), %rax
1400185a4:     	movq	%rax, 0xa0(%rbp)
1400185ab:     	movl	0x1c(%r15), %eax
1400185af:     	movl	%eax, 0x324(%rbp)
1400185b5:     	movq	(%r15), %rdx
1400185b8:     	movq	0x10(%r15), %rax
1400185bc:     	movq	%rax, 0x110(%rbp)
1400185c3:     	movq	0x2e0(%rbp), %rax
1400185ca:     	movzbl	0x30(%rax), %eax
1400185ce:     	movb	%al, 0x790(%rbp)
1400185d4:     	leaq	0x18(%rcx), %rax
1400185d8:     	movq	%rax, 0x248(%rbp)
1400185df:     	leaq	0x1b8(%rcx), %rax
1400185e6:     	movq	%rax, 0xc0(%rbp)
1400185ed:     	leaq	0x60(%rdx), %rax
1400185f1:     	movq	%rax, 0xb0(%rbp)
1400185f8:     	movq	%rdx, 0xb8(%rbp)
1400185ff:     	leaq	0x30(%rdx), %rax
140018603:     	movq	%rax, 0xa8(%rbp)
14001860a:     	movq	0x328(%rbp), %rax
140018611:     	addq	$0x20, %rax
140018615:     	movq	%rax, 0x108(%rbp)
14001861c:     	leaq	0x60(%rcx), %rax
140018620:     	movq	%rax, 0x340(%rbp)
140018627:     	xorl	%r14d, %r14d
14001862a:     	leaq	0x590(%rbp), %r12
140018631:     	jmp	0x14001fa40 <.text+0x1ea40>
140018636:     	nopw	%cs:(%rax,%rax)
140018640:     	movdqu	(%r8,%r10), %xmm1
140018646:     	movdqa	%xmm1, %xmm2
14001864a:     	pcmpeqb	%xmm0, %xmm2
14001864e:     	pmovmskb	%xmm2, %r11d
140018653:     	testl	%r11d, %r11d
140018656:     	je	0x1400186a0 <.text+0x176a0>
140018658:     	movq	0x7e0(%rbp), %rdx
14001865f:     	movl	0x128(%rdx), %ecx
140018665:     	movl	0x12c(%rdx), %edx
14001866b:     	tzcntl	%r11d, %edi
140018670:     	addq	%r10, %rdi
140018673:     	andq	%r9, %rdi
140018676:     	shlq	$0x3, %rdi
14001867a:     	movq	%r8, %rsi
14001867d:     	subq	%rdi, %rsi
140018680:     	cmpl	-0x8(%rsi), %ecx
140018683:     	jne	0x14001868a <.text+0x1768a>
140018685:     	cmpl	-0x4(%rsi), %edx
140018688:     	je	0x1400186c7 <.text+0x176c7>
14001868a:     	leal	-0x1(%r11), %esi
14001868e:     	andw	%r11w, %si
140018692:     	movl	%esi, %r11d
140018695:     	jne	0x14001866b <.text+0x1766b>
140018697:     	nopw	(%rax,%rax)
1400186a0:     	pcmpeqb	0x290d2e8(%rip), %xmm1  # 0x142925990
1400186a8:     	pmovmskb	%xmm1, %ecx
1400186ac:     	testl	%ecx, %ecx
1400186ae:     	jne	0x14001fa40 <.text+0x1ea40>
1400186b4:     	addq	%rax, %r10
1400186b7:     	addq	$0x10, %r10
1400186bb:     	addq	$0x10, %rax
1400186bf:     	andq	%r9, %r10
1400186c2:     	jmp	0x140018640 <.text+0x17640>
1400186c7:     	callq	0x140959e70 <.text+0x958e70>
1400186cc:     	movq	%rax, 0x438(%rbp)
1400186d3:     	leaq	0x460(%rbp), %rdi
1400186da:     	movq	0x438(%rbp), %rcx
1400186e1:     	callq	0x140c96270 <.text+0xc95270>
1400186e6:     	movq	%rax, 0x428(%rbp)
1400186ed:     	movq	0x7e0(%rbp), %rdx
1400186f4:     	cmpq	0x98(%rdx), %r14
1400186fb:     	jo	0x140020a20 <.text+0x1fa20>
140018701:     	movq	0xa0(%rdx), %rax
140018708:     	movq	%rax, 0x648(%rbp)
14001870f:     	movq	0xa8(%rdx), %rax
140018716:     	movq	%rax, 0x780(%rbp)
14001871d:     	movq	0x958(%rbp), %rax
140018724:     	movq	(%rax), %rcx
140018727:     	movl	0x128(%rdx), %edx
14001872d:     	callq	0x140a1c930 <.text+0xa1b930>
140018732:     	testq	%rax, %rax
140018735:     	je	0x14001873c <.text+0x1773c>
140018737:     	movl	0x28(%rax), %eax
14001873a:     	jmp	0x140018741 <.text+0x17741>
14001873c:     	movl	$0x186a3, %eax          # imm = 0x186A3
140018741:     	movq	%rax, 0x788(%rbp)
140018748:     	movq	0x958(%rbp), %rax
14001874f:     	movq	(%rax), %rcx
140018752:     	movq	0x670(%rbp), %rax
140018759:     	movl	(%rax), %edx
14001875b:     	callq	0x140a1c930 <.text+0xa1b930>
140018760:     	movl	$0x0, 0x640(%rbp)
14001876a:     	testq	%rax, %rax
14001876d:     	je	0x140018789 <.text+0x17789>
14001876f:     	addq	$0x30, %rax
140018773:     	movq	%rax, %rcx
140018776:     	callq	0x140839490 <.text+0x838490>
14001877b:     	testq	%rax, %rax
14001877e:     	je	0x140018789 <.text+0x17789>
140018780:     	movl	0x8(%rax), %eax
140018783:     	movl	%eax, 0x640(%rbp)
140018789:     	movq	0x958(%rbp), %rax
140018790:     	movq	(%rax), %rcx
140018793:     	movq	0x670(%rbp), %rax
14001879a:     	movl	(%rax), %edx
14001879c:     	callq	0x140a1c930 <.text+0xa1b930>
1400187a1:     	testq	%rax, %rax
1400187a4:     	je	0x1400187e8 <.text+0x177e8>
1400187a6:     	addq	$0x30, %rax
1400187aa:     	movq	%rax, %rcx
1400187ad:     	callq	0x140839490 <.text+0x838490>
1400187b2:     	testq	%rax, %rax
1400187b5:     	je	0x1400187e8 <.text+0x177e8>
1400187b7:     	movzbl	0x10(%rax), %eax
1400187bb:     	movl	%eax, 0x6a8(%rbp)
1400187c1:     	xorl	%r14d, %r14d
1400187c4:     	movq	0x328(%rbp), %rax
1400187cb:     	cmpq	$0x0, 0x18(%rax)
1400187d0:     	jne	0x140018803 <.text+0x17803>
1400187d2:     	movl	$0x0, 0x530(%rbp)
1400187dc:     	movq	0x7e0(%rbp), %rsi
1400187e3:     	jmp	0x1400188ef <.text+0x178ef>
1400187e8:     	movl	$0x0, 0x6a8(%rbp)
1400187f2:     	xorl	%r14d, %r14d
1400187f5:     	movq	0x328(%rbp), %rax
1400187fc:     	cmpq	$0x0, 0x18(%rax)
140018801:     	je	0x1400187d2 <.text+0x177d2>
140018803:     	movq	0x108(%rbp), %rcx
14001880a:     	movq	0x670(%rbp), %rdx
140018811:     	callq	0x1405b1be0 <.text+0x5b0be0>
140018816:     	movq	0x328(%rbp), %rdx
14001881d:     	movq	(%rdx), %rcx
140018820:     	movq	0x8(%rdx), %rdx
140018824:     	movq	%rdx, %r8
140018827:     	andq	%rax, %r8
14001882a:     	shrq	$0x39, %rax
14001882e:     	movd	%eax, %xmm0
140018832:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140018836:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14001883b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140018840:     	xorl	%eax, %eax
140018842:     	movq	0x7e0(%rbp), %rsi
140018849:     	movdqu	(%rcx,%r8), %xmm1
14001884f:     	movdqa	%xmm1, %xmm2
140018853:     	pcmpeqb	%xmm0, %xmm2
140018857:     	pmovmskb	%xmm2, %r9d
14001885c:     	testl	%r9d, %r9d
14001885f:     	je	0x1400188c0 <.text+0x178c0>
140018861:     	movq	0x670(%rbp), %r10
140018868:     	movq	(%r10), %xmm2
14001886d:     	tzcntl	%r9d, %r10d
140018872:     	addq	%r8, %r10
140018875:     	andq	%rdx, %r10
140018878:     	shlq	$0x3, %r10
14001887c:     	movq	%rcx, %r11
14001887f:     	subq	%r10, %r11
140018882:     	movq	-0x8(%r11), %xmm3
140018888:     	pcmpeqd	%xmm2, %xmm3
14001888c:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
140018891:     	movmskpd	%xmm3, %r10d
140018896:     	movb	$0x1, %r11b
140018899:     	movl	%r11d, 0x530(%rbp)
1400188a0:     	cmpl	$0x3, %r10d
1400188a4:     	je	0x1400188ef <.text+0x178ef>
1400188a6:     	leal	-0x1(%r9), %r10d
1400188aa:     	andw	%r9w, %r10w
1400188ae:     	movl	%r10d, %r9d
1400188b1:     	jne	0x14001886d <.text+0x1786d>
1400188b3:     	nopw	%cs:(%rax,%rax)
1400188c0:     	pcmpeqb	0x290d0c8(%rip), %xmm1  # 0x142925990
1400188c8:     	pmovmskb	%xmm1, %r9d
1400188cd:     	testl	%r9d, %r9d
1400188d0:     	jne	0x1400188e5 <.text+0x178e5>
1400188d2:     	addq	%rax, %r8
1400188d5:     	addq	$0x10, %r8
1400188d9:     	addq	$0x10, %rax
1400188dd:     	andq	%rdx, %r8
1400188e0:     	jmp	0x140018849 <.text+0x17849>
1400188e5:     	movl	$0x0, 0x530(%rbp)
1400188ef:     	leaq	0xd0(%rsi), %rax
1400188f6:     	movq	%rax, 0x2d8(%rbp)
1400188fd:     	cmpl	$0x1, (%rax)
140018900:     	jne	0x140018918 <.text+0x17918>
140018902:     	movss	0xd4(%rsi), %xmm0
14001890a:     	movss	%xmm0, 0x548(%rbp)
140018912:     	movl	$0x1, %r14d
140018918:     	xorl	%eax, %eax
14001891a:     	cmpl	$0x1, 0xf0(%rsi)
140018921:     	jne	0x140018938 <.text+0x17938>
140018923:     	movss	0xf4(%rsi), %xmm0
14001892b:     	movss	%xmm0, 0x370(%rbp)
140018933:     	movl	$0x1, %eax
140018938:     	movl	%eax, 0x540(%rbp)
14001893e:     	movq	0x90(%rbp), %rax
140018945:     	movl	0x238(%rbp), %ecx
14001894b:     	movl	%ecx, (%rax)
14001894d:     	movl	0x128(%rsi), %r8d
140018954:     	movl	0x12c(%rsi), %r9d
14001895b:     	movq	0x128(%rbp), %rax
140018962:     	movq	%rax, 0x38(%rsp)
140018967:     	movq	0x130(%rbp), %rax
14001896e:     	movq	%rax, 0x30(%rsp)
140018973:     	movq	0x120(%rbp), %rax
14001897a:     	movq	%rax, 0x28(%rsp)
14001897f:     	movq	%r13, 0x20(%rsp)
140018984:     	leaq	0x140(%rbp), %rcx
14001898b:     	movq	0x2e0(%rbp), %rdx
140018992:     	callq	0x140aa8a60 <.text+0xaa7a60>
140018997:     	movl	%r14d, 0x500(%rbp)
14001899e:     	movq	0x7e0(%rbp), %r14
1400189a5:     	leaq	0x130(%r14), %r15
1400189ac:     	leaq	0xf0(%r14), %rax
1400189b3:     	movq	%rax, 0x118(%rbp)
1400189ba:     	movq	0x780(%rbp), %r9
1400189c1:     	leaq	(%r9,%r9,2), %rax
1400189c5:     	movq	0x648(%rbp), %r10
1400189cc:     	leaq	(%r10,%rax,4), %rax
1400189d0:     	movq	0x58(%r14), %rdx
1400189d4:     	movq	0x60(%r14), %rcx
1400189d8:     	leaq	(%rdx,%rcx,4), %r8
1400189dc:     	cmpq	%r9, %rcx
1400189df:     	cmovaeq	%r9, %rcx
1400189e3:     	movq	%r10, 0x6c0(%rbp)
1400189ea:     	movq	%rax, 0x6c8(%rbp)
1400189f1:     	movq	%rdx, 0x6d0(%rbp)
1400189f8:     	movq	%r8, 0x6d8(%rbp)
1400189ff:     	movq	%rcx, 0x6e8(%rbp)
140018a06:     	movq	%r9, 0x6f0(%rbp)
140018a0d:     	movq	$0x0, 0x6f8(%rbp)
140018a18:     	xorl	%eax, %eax
140018a1a:     	jmp	0x140018a2e <.text+0x17a2e>
140018a1c:     	nopl	(%rax)
140018a20:     	movq	0x6e0(%rbp), %rax
140018a27:     	movq	0x6e8(%rbp), %rcx
140018a2e:     	cmpq	%rcx, %rax
140018a31:     	jae	0x140018cff <.text+0x17cff>
140018a37:     	leaq	0x1(%rax), %rcx
140018a3b:     	movq	%rcx, 0x6e0(%rbp)
140018a42:     	leaq	(%rax,%rax,2), %rcx
140018a46:     	shlq	$0x2, %rcx
140018a4a:     	addq	0x6c0(%rbp), %rcx
140018a51:     	shlq	$0x2, %rax
140018a55:     	addq	0x6d0(%rbp), %rax
140018a5c:     	cmpq	$0x0, 0x6f8(%rbp)
140018a64:     	je	0x140018aa0 <.text+0x17aa0>
140018a66:     	movq	0x708(%rbp), %rdx
140018a6d:     	movq	0x710(%rbp), %r8
140018a74:     	movq	%rcx, 0x708(%rbp)
140018a7b:     	movq	%rax, 0x710(%rbp)
140018a82:     	movq	%rdx, 0x6f8(%rbp)
140018a89:     	movq	%r8, 0x700(%rbp)
140018a90:     	jmp	0x140018b15 <.text+0x17b15>
140018a95:     	nopw	%cs:(%rax,%rax)
140018aa0:     	movq	$0x1, 0x460(%rbp)
140018aab:     	movq	%rcx, 0x468(%rbp)
140018ab2:     	movq	%rax, 0x470(%rbp)
140018ab9:     	leaq	0x6c0(%rbp), %rax
140018ac0:     	movq	%rax, 0x478(%rbp)
140018ac7:     	movq	%r12, %rcx
140018aca:     	movq	%rdi, %rdx
140018acd:     	callq	0x1402e13b0 <.text+0x2e03b0>
140018ad2:     	movups	0x590(%rbp), %xmm0
140018ad9:     	movdqu	0x5a0(%rbp), %xmm1
140018ae1:     	leaq	0x6f8(%rbp), %rax
140018ae8:     	movdqu	%xmm1, 0x10(%rax)
140018aed:     	movups	%xmm0, (%rax)
140018af0:     	movq	0x6f8(%rbp), %rdx
140018af7:     	testq	%rdx, %rdx
140018afa:     	je	0x140018cff <.text+0x17cff>
140018b00:     	movq	0x700(%rbp), %r8
140018b07:     	movq	0x708(%rbp), %rcx
140018b0e:     	movq	0x710(%rbp), %rax
140018b15:     	movd	(%rdx), %xmm0
140018b19:     	movss	0x8(%rdx), %xmm1
140018b1e:     	movd	(%rcx), %xmm2
140018b22:     	movss	0x8(%rcx), %xmm3
140018b27:     	movss	(%r8), %xmm4
140018b2c:     	movss	(%rax), %xmm5
140018b30:     	movaps	%xmm4, %xmm6
140018b33:     	cmpunordss	%xmm4, %xmm6
140018b38:     	movaps	%xmm6, %xmm7
140018b3b:     	andps	%xmm5, %xmm7
140018b3e:     	maxss	%xmm4, %xmm5
140018b42:     	andnps	%xmm5, %xmm6
140018b45:     	orps	%xmm7, %xmm6
140018b48:     	mulss	0x290cd20(%rip), %xmm6  # 0x142925870
140018b50:     	movq	0x1f8(%rbp), %rax
140018b57:     	movq	%rax, 0x460(%rbp)
140018b5e:     	mulss	0x290cd0e(%rip), %xmm6  # 0x142925874
140018b66:     	addss	0x290cd0a(%rip), %xmm6  # 0x142925878
140018b6e:     	movd	%xmm0, 0x590(%rbp)
140018b76:     	movss	%xmm1, 0x594(%rbp)
140018b7e:     	movd	%xmm2, 0x598(%rbp)
140018b86:     	movss	%xmm3, 0x59c(%rbp)
140018b8e:     	xorl	%eax, %eax
140018b90:     	xorl	%esi, %esi
140018b92:     	nopw	%cs:(%rax,%rax)
140018ba0:     	cmpq	$0x2, %rsi
140018ba4:     	je	0x140018a20 <.text+0x17a20>
140018baa:     	leaq	0x1(%rsi), %rcx
140018bae:     	movd	0x590(%rbp,%rsi,8), %xmm0
140018bb7:     	movd	0x594(%rbp,%rsi,8), %xmm2
140018bc0:     	testb	$0x1, %al
140018bc2:     	je	0x140018be0 <.text+0x17be0>
140018bc4:     	movq	%rcx, %rsi
140018bc7:     	movdqa	%xmm0, %xmm12
140018bcc:     	movdqa	%xmm11, %xmm13
140018bd1:     	movdqa	%xmm1, %xmm0
140018bd5:     	movdqa	%xmm2, %xmm11
140018bda:     	jmp	0x140018c13 <.text+0x17c13>
140018bdc:     	nopl	(%rax)
140018be0:     	cmpq	$0x2, %rcx
140018be4:     	je	0x140018a20 <.text+0x17a20>
140018bea:     	movq	$0x2, 0x5a0(%rbp)
140018bf5:     	movd	0x598(%rbp,%rsi,8), %xmm12
140018bff:     	movd	0x59c(%rbp,%rsi,8), %xmm11
140018c09:     	movl	$0x2, %esi
140018c0e:     	movdqa	%xmm2, %xmm13
140018c13:     	movb	$0x0, 0x380(%rbp)
140018c1a:     	movss	0x290cc52(%rip), %xmm7  # 0x142925874
140018c22:     	movaps	%xmm7, %xmm1
140018c25:     	movd	0x290cc4e(%rip), %xmm8  # 0x14292587c
140018c2e:     	movdqa	%xmm8, %xmm2
140018c33:     	callq	0x142923ef0 <.text+0x2922ef0>
140018c38:     	movdqa	%xmm0, %xmm14
140018c3d:     	movdqa	%xmm13, %xmm0
140018c42:     	movaps	%xmm7, %xmm1
140018c45:     	movdqa	%xmm8, %xmm2
140018c4a:     	callq	0x142923ef0 <.text+0x2922ef0>
140018c4f:     	movd	%xmm14, %ecx
140018c54:     	movd	%xmm0, %eax
140018c58:     	shlq	$0x20, %rax
140018c5c:     	orq	%rax, %rcx
140018c5f:     	callq	0x140b84e40 <.text+0xb83e40>
140018c64:     	movdqa	%xmm0, %xmm13
140018c69:     	movaps	%xmm1, %xmm14
140018c6d:     	movdqa	%xmm12, %xmm0
140018c72:     	movss	0x290cbfa(%rip), %xmm7  # 0x142925874
140018c7a:     	movaps	%xmm7, %xmm1
140018c7d:     	movss	0x290cbf6(%rip), %xmm8  # 0x14292587c
140018c86:     	movaps	%xmm8, %xmm2
140018c8a:     	callq	0x142923ef0 <.text+0x2922ef0>
140018c8f:     	movdqa	%xmm0, %xmm15
140018c94:     	movdqa	%xmm11, %xmm0
140018c99:     	movaps	%xmm7, %xmm1
140018c9c:     	movaps	%xmm8, %xmm2
140018ca0:     	callq	0x142923ef0 <.text+0x2922ef0>
140018ca5:     	movd	%xmm15, %ecx
140018caa:     	movd	%xmm0, %eax
140018cae:     	shlq	$0x20, %rax
140018cb2:     	orq	%rax, %rcx
140018cb5:     	callq	0x140b84e40 <.text+0xb83e40>
140018cba:     	movdqa	%xmm0, %xmm2
140018cbe:     	movaps	%xmm1, %xmm3
140018cc1:     	leaq	0x380(%rbp), %rax
140018cc8:     	movq	%rax, 0x30(%rsp)
140018ccd:     	movq	%rdi, 0x28(%rsp)
140018cd2:     	movss	%xmm6, 0x20(%rsp)
140018cd8:     	movdqa	%xmm13, %xmm0
140018cdd:     	movaps	%xmm14, %xmm1
140018ce1:     	callq	0x1402582d0 <.text+0x2572d0>
140018ce6:     	movb	$0x1, %al
140018ce8:     	cmpb	$0x1, 0x380(%rbp)
140018cef:     	movdqa	%xmm12, %xmm1
140018cf4:     	jne	0x140018ba0 <.text+0x17ba0>
140018cfa:     	jmp	0x140018a20 <.text+0x17a20>
140018cff:     	movl	0x23c(%rbp), %eax
140018d05:     	movq	0xa0(%rbp), %rcx
140018d0c:     	movl	%eax, (%rcx)
140018d0e:     	movl	$0x1, %edx
140018d13:     	movq	0x98(%rbp), %rcx
140018d1a:     	movq	0x438(%rbp), %r8
140018d21:     	callq	0x1400cce50 <.text+0xcbe50>
140018d26:     	movq	%rax, 0x6a0(%rbp)
140018d2d:     	movq	0x6a0(%rbp), %rax
140018d34:     	movq	$0x0, 0x50(%rax)
140018d3c:     	movq	0x788(%rbp), %rax
140018d43:     	addl	$0xfffe7960, %eax       # imm = 0xFFFE7960
140018d48:     	cmpl	$0xb, %eax
140018d4b:     	ja	0x140018d7c <.text+0x17d7c>
140018d4d:     	leaq	0x290eb6c(%rip), %rcx   # 0x1429278c0
140018d54:     	movslq	(%rcx,%rax,4), %rax
140018d58:     	addq	%rcx, %rax
140018d5b:     	jmpq	*%rax
140018d5d:     	movsd	0x290cb5b(%rip), %xmm0  # 0x1429258c0
140018d65:     	movsd	%xmm0, 0x590(%rbp)
140018d6d:     	movl	$0x3e2e147b, 0x598(%rbp) # imm = 0x3E2E147B
140018d77:     	jmp	0x140018e01 <.text+0x17e01>
140018d7c:     	movl	$0x0, 0x598(%rbp)
140018d86:     	movq	$0x0, 0x590(%rbp)
140018d91:     	jmp	0x140018e01 <.text+0x17e01>
140018d93:     	movsd	0x290cb05(%rip), %xmm0  # 0x1429258a0
140018d9b:     	movsd	%xmm0, 0x590(%rbp)
140018da3:     	movl	$0x3e051eb8, 0x598(%rbp) # imm = 0x3E051EB8
140018dad:     	jmp	0x140018e01 <.text+0x17e01>
140018daf:     	movsd	0x290caf9(%rip), %xmm0  # 0x1429258b0
140018db7:     	movsd	%xmm0, 0x590(%rbp)
140018dbf:     	movl	$0x3e0f5c29, 0x598(%rbp) # imm = 0x3E0F5C29
140018dc9:     	jmp	0x140018e01 <.text+0x17e01>
140018dcb:     	movsd	0x290cabd(%rip), %xmm0  # 0x142925890
140018dd3:     	movsd	%xmm0, 0x590(%rbp)
140018ddb:     	movl	$0x3e570a3d, 0x598(%rbp) # imm = 0x3E570A3D
140018de5:     	jmp	0x140018e01 <.text+0x17e01>
140018de7:     	movsd	0x290ca91(%rip), %xmm0  # 0x142925880
140018def:     	movsd	%xmm0, 0x590(%rbp)
140018df7:     	movl	$0x3e2147ae, 0x598(%rbp) # imm = 0x3E2147AE
140018e01:     	leaq	0x6c0(%rbp), %rcx
140018e08:     	movq	%r12, %rdx
140018e0b:     	callq	0x1408bc460 <.text+0x8bb460>
140018e10:     	testb	$0x1, 0x6c0(%rbp)
140018e17:     	je	0x140018e36 <.text+0x17e36>
140018e19:     	movq	0x6c8(%rbp), %rax
140018e20:     	movq	%rax, 0x460(%rbp)
140018e27:     	movq	%rdi, %rcx
140018e2a:     	callq	0x1400885d0 <.text+0x875d0>
140018e2f:     	orl	$0x1000000, %eax        # imm = 0x1000000
140018e34:     	jmp	0x140018e3c <.text+0x17e3c>
140018e36:     	movl	0x6c4(%rbp), %eax
140018e3c:     	movq	0x6a0(%rbp), %rcx
140018e43:     	movl	%eax, 0x58(%rcx)
140018e46:     	movq	$0x0, 0x440(%rbp)
140018e51:     	xorps	%xmm0, %xmm0
140018e54:     	leaq	0x450(%rbp), %rax
140018e5b:     	movups	%xmm0, (%rax)
140018e5e:     	movq	$0x8, 0x448(%rbp)
140018e69:     	movq	$0x0, 0x278(%rbp)
140018e74:     	movq	$0x10, 0x280(%rbp)
140018e7f:     	movq	$0x0, 0x288(%rbp)
140018e8a:     	movq	0x140(%r14), %rax
140018e91:     	cmpq	$0x2, %rax
140018e95:     	jb	0x140018ea5 <.text+0x17ea5>
140018e97:     	movq	0x130(%r14), %r15
140018e9e:     	movq	0x138(%r14), %rax
140018ea5:     	addq	$0x40, 0x6a0(%rbp)
140018ead:     	leaq	(%r15,%rax,4), %rax
140018eb1:     	movq	%rax, 0x538(%rbp)
140018eb8:     	movq	0x780(%rbp), %rax
140018ebf:     	decq	%rax
140018ec2:     	movq	%rax, 0x580(%rbp)
140018ec9:     	movq	%r15, %r8
140018ecc:     	xorl	%edi, %edi
140018ece:     	movq	%rbx, 0x88(%rbp)
140018ed5:     	jmp	0x140018efc <.text+0x17efc>
140018ed7:     	nopw	(%rax,%rax)
140018ee0:     	movq	0x7e0(%rbp), %r14
140018ee7:     	movq	0x560(%rbp), %r8
140018eee:     	movq	0x698(%rbp), %rdi
140018ef5:     	leaq	0x590(%rbp), %r12
140018efc:     	cmpq	0x538(%rbp), %r8
140018f03:     	je	0x14001d368 <.text+0x1c368>
140018f09:     	movzwl	0x2(%r8), %r9d
140018f0e:     	leaq	(%rdi,%r9), %r10
140018f12:     	testq	%rdi, %rdi
140018f15:     	sete	%bl
140018f18:     	cmpq	0x580(%rbp), %r10
140018f1f:     	sete	%r11b
140018f23:     	cmpq	$-0x1, %r10
140018f27:     	je	0x1400203d9 <.text+0x1f3d9>
140018f2d:     	leaq	0x1(%r10), %rcx
140018f31:     	movq	%rcx, %rax
140018f34:     	subq	%rdi, %rax
140018f37:     	jb	0x1400203f1 <.text+0x1f3f1>
140018f3d:     	cmpq	0x780(%rbp), %r10
140018f44:     	jae	0x14002040f <.text+0x1f40f>
140018f4a:     	movq	0x78(%r14), %rdx
140018f4e:     	cmpq	%rdx, %r10
140018f51:     	jae	0x14002042e <.text+0x1f42e>
140018f57:     	movq	0x60(%r14), %rdx
140018f5b:     	cmpq	%rdx, %r10
140018f5e:     	jae	0x140020446 <.text+0x1f446>
140018f64:     	movq	%rax, 0x808(%rbp)
140018f6b:     	leaq	0x4(%r8), %rax
140018f6f:     	movq	%rax, 0x560(%rbp)
140018f76:     	leaq	(%rdi,%rdi,2), %rax
140018f7a:     	movq	0x648(%rbp), %rdx
140018f81:     	leaq	(%rdx,%rax,4), %r15
140018f85:     	leaq	(,%rdi,8), %rax
140018f8d:     	addq	0x70(%r14), %rax
140018f91:     	movq	%rax, 0x7a0(%rbp)
140018f98:     	leaq	(,%rdi,4), %r13
140018fa0:     	movq	0x58(%r14), %rax
140018fa4:     	addq	%r13, %rax
140018fa7:     	movq	%rax, 0x6b0(%rbp)
140018fae:     	movq	0x90(%r14), %rdx
140018fb5:     	testq	%rdx, %rdx
140018fb8:     	movq	%r10, 0x698(%rbp)
140018fbf:     	movb	%r11b, 0x588(%rbp)
140018fc6:     	movb	%bl, 0x678(%rbp)
140018fcc:     	movq	%rdi, 0x430(%rbp)
140018fd3:     	je	0x140019050 <.text+0x18050>
140018fd5:     	cmpq	%rdx, %r10
140018fd8:     	pxor	%xmm1, %xmm1
140018fdc:     	jae	0x1400204a9 <.text+0x1f4a9>
140018fe2:     	addq	0x88(%r14), %r13
140018fe9:     	movzbl	(%r8), %eax
140018fed:     	movzbl	0x1(%r8), %ecx
140018ff2:     	movb	%al, 0x806(%rbp)
140018ff8:     	movb	%cl, 0x807(%rbp)
140018ffe:     	cmpq	$0x0, 0x808(%rbp)
140019006:     	je	0x14001906c <.text+0x1806c>
140019008:     	movzbl	%al, %ecx
14001900b:     	leaq	0x4(,%r9,4), %rdx
140019013:     	xorl	%r8d, %r8d
140019016:     	nopw	%cs:(%rax,%rax)
140019020:     	cmpq	%r8, %rdx
140019023:     	je	0x14001907c <.text+0x1807c>
140019025:     	movss	(%r13,%r8), %xmm0
14001902c:     	addq	$0x4, %r8
140019030:     	ucomiss	%xmm1, %xmm0
140019033:     	ja	0x140019020 <.text+0x18020>
140019035:     	movq	$0x0, 0x7d8(%rbp)
140019040:     	jmp	0x140019100 <.text+0x18100>
140019045:     	nopw	%cs:(%rax,%rax)
140019050:     	movq	0x88(%r14), %r13
140019057:     	movzbl	(%r8), %eax
14001905b:     	movzbl	0x1(%r8), %ecx
140019060:     	movb	%al, 0x806(%rbp)
140019066:     	movb	%cl, 0x807(%rbp)
14001906c:     	movb	$0x1, %cl
14001906e:     	movq	%rcx, 0x7d8(%rbp)
140019075:     	xorl	%edx, %edx
140019077:     	jmp	0x140019107 <.text+0x18107>
14001907c:     	leal	-0x2(%rax), %edx
14001907f:     	decq	%rcx
140019082:     	xorl	%r8d, %r8d
140019085:     	cmpb	$0x3, %dl
140019088:     	movl	$0x0, %edx
14001908d:     	movq	%rdx, 0x7d8(%rbp)
140019094:     	cmovaeq	%r8, %rcx
140019098:     	testq	%rcx, %rcx
14001909b:     	je	0x1400190a3 <.text+0x180a3>
14001909d:     	cmpq	$0x3, %rcx
1400190a1:     	jne	0x140019100 <.text+0x18100>
1400190a3:     	movb	$0x2, 0x806(%rbp)
1400190aa:     	leaq	0x806(%rbp), %rax
1400190b1:     	movq	%rax, 0x290(%rbp)
1400190b8:     	movq	0x958(%rbp), %rax
1400190bf:     	movq	%rax, 0x298(%rbp)
1400190c6:     	movq	0x670(%rbp), %rax
1400190cd:     	movq	%rax, 0x2a0(%rbp)
1400190d4:     	movq	0x248(%rbp), %rax
1400190db:     	movq	%rax, 0x2a8(%rbp)
1400190e2:     	movq	$0x0, 0x7d8(%rbp)
1400190ed:     	movq	0x808(%rbp), %rax
1400190f4:     	movq	%rax, 0x7f0(%rbp)
1400190fb:     	jmp	0x14001914e <.text+0x1814e>
1400190fd:     	nopl	(%rax)
140019100:     	movq	0x808(%rbp), %rdx
140019107:     	leaq	0x806(%rbp), %rcx
14001910e:     	movq	%rcx, 0x290(%rbp)
140019115:     	movq	0x958(%rbp), %rcx
14001911c:     	movq	%rcx, 0x298(%rbp)
140019123:     	movq	0x670(%rbp), %rcx
14001912a:     	movq	%rcx, 0x2a0(%rbp)
140019131:     	movq	0x248(%rbp), %rcx
140019138:     	movq	%rcx, 0x2a8(%rbp)
14001913f:     	cmpb	$0x3, %al
140019141:     	movq	%rdx, 0x7f0(%rbp)
140019148:     	je	0x140019bba <.text+0x18bba>
14001914e:     	movq	0x200(%rbp), %rax
140019155:     	movb	$0x1, 0x817(%rbp)
14001915c:     	movq	%rax, 0x48(%rsp)
140019161:     	movq	0x6a0(%rbp), %rax
140019168:     	movq	%rax, 0x40(%rsp)
14001916d:     	leaq	0x278(%rbp), %rax
140019174:     	movq	%rax, 0x38(%rsp)
140019179:     	movq	0x808(%rbp), %rsi
140019180:     	movq	%rsi, 0x28(%rsp)
140019185:     	movq	0x6b0(%rbp), %rax
14001918c:     	movq	%rax, 0x20(%rsp)
140019191:     	movq	$0x1, 0x30(%rsp)
14001919a:     	movq	%r15, %rcx
14001919d:     	movq	%rsi, %rdx
1400191a0:     	movq	0x7a0(%rbp), %r8
1400191a7:     	movq	%rsi, %r9
1400191aa:     	callq	0x1407de8b0 <.text+0x7dd8b0>
1400191af:     	movzbl	0x806(%rbp), %eax
1400191b6:     	movb	%al, 0x7d0(%rbp)
1400191bc:     	movq	0x670(%rbp), %rax
1400191c3:     	movq	(%rax), %rax
1400191c6:     	movq	%rax, 0x568(%rbp)
1400191cd:     	movq	%rsi, %rax
1400191d0:     	movl	$0xc, %ecx
1400191d5:     	mulq	%rcx
1400191d8:     	movq	%rax, %rbx
1400191db:     	jo	0x14002083d <.text+0x1f83d>
1400191e1:     	movl	$0x0, %r12d
1400191e7:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1400191f1:     	cmpq	%rax, %rbx
1400191f4:     	ja	0x140020840 <.text+0x1f840>
1400191fa:     	testq	%rbx, %rbx
1400191fd:     	je	0x140019240 <.text+0x18240>
1400191ff:     	movzbl	0x361ba03(%rip), %eax   # 0x143634c09
140019206:     	movl	$0x4, %r12d
14001920c:     	movl	$0x4, %edx
140019211:     	movq	%rbx, %rcx
140019214:     	callq	0x140613c10 <.text+0x612c10>
140019219:     	testq	%rax, %rax
14001921c:     	je	0x140020840 <.text+0x1f840>
140019222:     	movq	%rax, %rsi
140019225:     	movq	0x808(%rbp), %r14
14001922c:     	movq	%r14, %rdi
14001922f:     	jmp	0x14001924e <.text+0x1824e>
140019231:     	nopw	%cs:(%rax,%rax)
140019240:     	movl	$0x4, %esi
140019245:     	xorl	%edi, %edi
140019247:     	movq	0x808(%rbp), %r14
14001924e:     	movq	%rsi, %rcx
140019251:     	movq	%r15, %rdx
140019254:     	movq	%rbx, %r8
140019257:     	callq	0x142923c00 <.text+0x2922c00>
14001925c:     	movq	%rdi, 0x590(%rbp)
140019263:     	movq	%rsi, 0x598(%rbp)
14001926a:     	movq	%r14, 0x5a0(%rbp)
140019271:     	movb	$0x1, 0x817(%rbp)
140019278:     	leaq	0x6c0(%rbp), %rcx
14001927f:     	leaq	0x590(%rbp), %rdx
140019286:     	movl	$0x1, %r8d
14001928c:     	callq	0x140153640 <.text+0x152640>
140019291:     	xorl	%eax, %eax
140019293:     	cmpq	0x6c0(%rbp), %rax
14001929a:     	jo	0x140018ee0 <.text+0x17ee0>
1400192a0:     	movq	0x6f0(%rbp), %rax
1400192a7:     	movq	%rax, 0x3b0(%rbp)
1400192ae:     	movups	0x6c0(%rbp), %xmm0
1400192b5:     	movups	0x6d0(%rbp), %xmm1
1400192bc:     	movdqu	0x6e0(%rbp), %xmm2
1400192c4:     	movdqa	%xmm2, 0x3a0(%rbp)
1400192cc:     	movaps	%xmm1, 0x390(%rbp)
1400192d3:     	movaps	%xmm0, 0x380(%rbp)
1400192da:     	movq	0x390(%rbp), %rbx
1400192e1:     	movq	%rbx, %rax
1400192e4:     	movl	$0xc, %ecx
1400192e9:     	mulq	%rcx
1400192ec:     	movq	%rax, %r12
1400192ef:     	jo	0x140020894 <.text+0x1f894>
1400192f5:     	movl	$0x0, %esi
1400192fa:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140019304:     	cmpq	%rax, %r12
140019307:     	ja	0x140020896 <.text+0x1f896>
14001930d:     	movq	0x388(%rbp), %r14
140019314:     	testq	%r12, %r12
140019317:     	movq	%rbx, 0x528(%rbp)
14001931e:     	je	0x14001934e <.text+0x1834e>
140019320:     	movzbl	0x361b8e2(%rip), %eax   # 0x143634c09
140019327:     	movl	$0x4, %esi
14001932c:     	movl	$0x4, %edx
140019331:     	movq	%r12, %rcx
140019334:     	callq	0x140613c10 <.text+0x612c10>
140019339:     	testq	%rax, %rax
14001933c:     	je	0x140020896 <.text+0x1f896>
140019342:     	movq	%rax, %rdi
140019345:     	movq	%rbx, 0x790(%rbp)
14001934c:     	jmp	0x14001935e <.text+0x1835e>
14001934e:     	movl	$0x4, %edi
140019353:     	movq	$0x0, 0x790(%rbp)
14001935e:     	movq	%r13, %rbx
140019361:     	movq	%rdi, %rcx
140019364:     	movq	%r14, %rdx
140019367:     	movq	%r12, %r8
14001936a:     	callq	0x142923c00 <.text+0x2922c00>
14001936f:     	movq	0x3a8(%rbp), %r15
140019376:     	leaq	(,%r15,4), %r12
14001937e:     	movq	%r15, %rax
140019381:     	shrq	$0x3e, %rax
140019385:     	jne	0x14002091e <.text+0x1f91e>
14001938b:     	movl	$0x0, %r14d
140019391:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14001939b:     	cmpq	%rax, %r12
14001939e:     	ja	0x140020921 <.text+0x1f921>
1400193a4:     	movq	0x3a0(%rbp), %rdx
1400193ab:     	testq	%r12, %r12
1400193ae:     	je	0x1400193e4 <.text+0x183e4>
1400193b0:     	movq	%rdx, %r13
1400193b3:     	movzbl	0x361b84f(%rip), %eax   # 0x143634c09
1400193ba:     	movl	$0x4, %r14d
1400193c0:     	movl	$0x4, %edx
1400193c5:     	movq	%r12, %rcx
1400193c8:     	callq	0x140613c10 <.text+0x612c10>
1400193cd:     	testq	%rax, %rax
1400193d0:     	je	0x140020921 <.text+0x1f921>
1400193d6:     	movq	%rax, %rsi
1400193d9:     	movq	%rdi, %r14
1400193dc:     	movq	%r15, %rdi
1400193df:     	movq	%r13, %rdx
1400193e2:     	jmp	0x1400193ee <.text+0x183ee>
1400193e4:     	movq	%rdi, %r14
1400193e7:     	movl	$0x4, %esi
1400193ec:     	xorl	%edi, %edi
1400193ee:     	movq	%rbx, %r13
1400193f1:     	movq	%rsi, %rcx
1400193f4:     	movq	%r12, %r8
1400193f7:     	callq	0x142923c00 <.text+0x2922c00>
1400193fc:     	movss	0x3b0(%rbp), %xmm0
140019404:     	movq	0x790(%rbp), %rax
14001940b:     	movq	%rax, 0x6c0(%rbp)
140019412:     	movq	%r14, 0x6c8(%rbp)
140019419:     	movq	0x528(%rbp), %rax
140019420:     	movq	%rax, 0x6d0(%rbp)
140019427:     	movq	%rdi, 0x6d8(%rbp)
14001942e:     	movq	%rsi, 0x6e0(%rbp)
140019435:     	movq	%r15, 0x6e8(%rbp)
14001943c:     	movss	%xmm0, 0x6f0(%rbp)
140019444:     	leaq	0x590(%rbp), %rcx
14001944b:     	leaq	0x6c0(%rbp), %rdx
140019452:     	movd	0x290c416(%rip), %xmm2  # 0x142925870
14001945a:     	movl	$0x1, %r9d
140019460:     	callq	0x140152b00 <.text+0x151b00>
140019465:     	movq	0x7e0(%rbp), %r14
14001946c:     	movq	0x590(%rbp), %rax
140019473:     	movq	%rax, %rcx
140019476:     	negq	%rcx
140019479:     	jno	0x140019520 <.text+0x18520>
14001947f:     	movq	0x6c0(%rbp), %rax
140019486:     	testq	%rax, %rax
140019489:     	je	0x1400194a5 <.text+0x184a5>
14001948b:     	movq	0x6c8(%rbp), %rcx
140019492:     	shlq	$0x2, %rax
140019496:     	leaq	(%rax,%rax,2), %rdx
14001949a:     	movl	$0x4, %r8d
1400194a0:     	callq	0x140613c20 <.text+0x612c20>
1400194a5:     	movq	0x6d8(%rbp), %rdx
1400194ac:     	testq	%rdx, %rdx
1400194af:     	je	0x1400194c7 <.text+0x184c7>
1400194b1:     	movq	0x6e0(%rbp), %rcx
1400194b8:     	shlq	$0x2, %rdx
1400194bc:     	movl	$0x4, %r8d
1400194c2:     	callq	0x140613c20 <.text+0x612c20>
1400194c7:     	movq	0x380(%rbp), %rax
1400194ce:     	testq	%rax, %rax
1400194d1:     	je	0x1400194ed <.text+0x184ed>
1400194d3:     	movq	0x388(%rbp), %rcx
1400194da:     	shlq	$0x2, %rax
1400194de:     	leaq	(%rax,%rax,2), %rdx
1400194e2:     	movl	$0x4, %r8d
1400194e8:     	callq	0x140613c20 <.text+0x612c20>
1400194ed:     	movq	0x398(%rbp), %rdx
1400194f4:     	testq	%rdx, %rdx
1400194f7:     	je	0x140018ee7 <.text+0x17ee7>
1400194fd:     	movq	0x3a0(%rbp), %rcx
140019504:     	shlq	$0x2, %rdx
140019508:     	movl	$0x4, %r8d
14001950e:     	callq	0x140613c20 <.text+0x612c20>
140019513:     	jmp	0x140018ee7 <.text+0x17ee7>
140019518:     	nopl	(%rax,%rax)
140019520:     	leaq	0x598(%rbp), %rcx
140019527:     	movups	(%rcx), %xmm0
14001952a:     	movups	0x10(%rcx), %xmm1
14001952e:     	movdqu	0x20(%rcx), %xmm2
140019533:     	leaq	0x468(%rbp), %rcx
14001953a:     	movdqu	%xmm2, 0x20(%rcx)
14001953f:     	movups	%xmm1, 0x10(%rcx)
140019543:     	movups	%xmm0, (%rcx)
140019546:     	movq	%rax, 0x460(%rbp)
14001954d:     	movq	0x6c0(%rbp), %rax
140019554:     	testq	%rax, %rax
140019557:     	je	0x140019573 <.text+0x18573>
140019559:     	movq	0x6c8(%rbp), %rcx
140019560:     	shlq	$0x2, %rax
140019564:     	leaq	(%rax,%rax,2), %rdx
140019568:     	movl	$0x4, %r8d
14001956e:     	callq	0x140613c20 <.text+0x612c20>
140019573:     	movq	0x6d8(%rbp), %rdx
14001957a:     	testq	%rdx, %rdx
14001957d:     	je	0x140019595 <.text+0x18595>
14001957f:     	movq	0x6e0(%rbp), %rcx
140019586:     	shlq	$0x2, %rdx
14001958a:     	movl	$0x4, %r8d
140019590:     	callq	0x140613c20 <.text+0x612c20>
140019595:     	cmpb	$0x0, 0x7d8(%rbp)
14001959c:     	jne	0x1400208ad <.text+0x1f8ad>
1400195a2:     	movq	0x470(%rbp), %rdi
1400195a9:     	testq	%rdi, %rdi
1400195ac:     	movq	%r13, 0x558(%rbp)
1400195b3:     	je	0x1400196d5 <.text+0x186d5>
1400195b9:     	leaq	(,%rdi,4), %rsi
1400195c1:     	movzbl	0x361b641(%rip), %eax   # 0x143634c09
1400195c8:     	movl	$0x4, %edx
1400195cd:     	movq	%rsi, %rcx
1400195d0:     	callq	0x140613c10 <.text+0x612c10>
1400195d5:     	testq	%rax, %rax
1400195d8:     	je	0x14002093f <.text+0x1f93f>
1400195de:     	movq	0x470(%rbp), %rbx
1400195e5:     	movq	%rdi, 0x650(%rbp)
1400195ec:     	movq	%rax, 0x658(%rbp)
1400195f3:     	movq	$0x0, 0x660(%rbp)
1400195fe:     	testq	%rbx, %rbx
140019601:     	je	0x1400196f6 <.text+0x186f6>
140019607:     	leaq	(,%rbx,8), %rsi
14001960f:     	movzbl	0x361b5f3(%rip), %eax   # 0x143634c09
140019616:     	movl	$0x4, %edx
14001961b:     	movq	%rsi, %rcx
14001961e:     	callq	0x140613c10 <.text+0x612c10>
140019623:     	testq	%rax, %rax
140019626:     	je	0x1400209ba <.text+0x1f9ba>
14001962c:     	movq	0x470(%rbp), %rdi
140019633:     	movq	%rbx, 0x680(%rbp)
14001963a:     	movq	%rax, 0x688(%rbp)
140019641:     	movq	$0x0, 0x690(%rbp)
14001964c:     	testq	%rdi, %rdi
14001964f:     	je	0x140019717 <.text+0x18717>
140019655:     	leaq	(,%rdi,4), %rax
14001965d:     	leaq	(%rax,%rax,2), %rsi
140019661:     	movzbl	0x361b5a1(%rip), %eax   # 0x143634c09
140019668:     	movl	$0x4, %edx
14001966d:     	movq	%rsi, %rcx
140019670:     	callq	0x140613c10 <.text+0x612c10>
140019675:     	testq	%rax, %rax
140019678:     	je	0x1400209ee <.text+0x1f9ee>
14001967e:     	movq	0x470(%rbp), %rbx
140019685:     	movq	%rdi, 0x508(%rbp)
14001968c:     	movq	%rax, 0x510(%rbp)
140019693:     	movq	$0x0, 0x518(%rbp)
14001969e:     	testq	%rbx, %rbx
1400196a1:     	je	0x140019738 <.text+0x18738>
1400196a7:     	leaq	(,%rbx,4), %rsi
1400196af:     	movzbl	0x361b553(%rip), %eax   # 0x143634c09
1400196b6:     	movl	$0x4, %edx
1400196bb:     	movq	%rsi, %rcx
1400196be:     	callq	0x140613c10 <.text+0x612c10>
1400196c3:     	testq	%rax, %rax
1400196c6:     	je	0x140020a07 <.text+0x1fa07>
1400196cc:     	movq	0x470(%rbp), %r14
1400196d3:     	jmp	0x140019742 <.text+0x18742>
1400196d5:     	movq	$0x0, 0x650(%rbp)
1400196e0:     	movq	$0x4, 0x658(%rbp)
1400196eb:     	movq	$0x0, 0x660(%rbp)
1400196f6:     	movq	$0x0, 0x680(%rbp)
140019701:     	movq	$0x4, 0x688(%rbp)
14001970c:     	movq	$0x0, 0x690(%rbp)
140019717:     	movq	$0x0, 0x508(%rbp)
140019722:     	movq	$0x4, 0x510(%rbp)
14001972d:     	movq	$0x0, 0x518(%rbp)
140019738:     	movl	$0x4, %eax
14001973d:     	xorl	%ebx, %ebx
14001973f:     	xorl	%r14d, %r14d
140019742:     	movq	%rbx, 0x590(%rbp)
140019749:     	movq	%rax, 0x598(%rbp)
140019750:     	movq	$0x0, 0x5a0(%rbp)
14001975b:     	movq	0x468(%rbp), %rax
140019762:     	movq	%rax, 0x7d8(%rbp)
140019769:     	movq	0x480(%rbp), %rax
140019770:     	movq	%rax, 0x790(%rbp)
140019777:     	movq	0x488(%rbp), %rax
14001977e:     	cmpq	%r14, %rax
140019781:     	cmovbq	%rax, %r14
140019785:     	xorl	%r12d, %r12d
140019788:     	xorl	%r13d, %r13d
14001978b:     	jmp	0x1400197b8 <.text+0x187b8>
14001978d:     	nopl	(%rax)
140019790:     	incq	%r13
140019793:     	subss	%xmm6, %xmm7
140019797:     	mulss	%xmm7, %xmm11
14001979c:     	addss	%xmm6, %xmm11
1400197a1:     	movq	0x598(%rbp), %rax
1400197a8:     	movss	%xmm11, (%rax,%r12,4)
1400197ae:     	incq	%r12
1400197b1:     	movq	%r12, 0x5a0(%rbp)
1400197b8:     	cmpq	%r14, %r13
1400197bb:     	jae	0x1400199f0 <.text+0x189f0>
1400197c1:     	leaq	(,%r13,2), %rax
1400197c9:     	addq	%r13, %rax
1400197cc:     	movq	0x7d8(%rbp), %rcx
1400197d3:     	movl	(%rcx,%rax,4), %r15d
1400197d7:     	movss	0x4(%rcx,%rax,4), %xmm6
1400197dd:     	movl	0x8(%rcx,%rax,4), %esi
1400197e1:     	movq	0x790(%rbp), %rax
1400197e8:     	movss	(%rax,%r13,4), %xmm1
1400197ee:     	leaq	0x380(%rbp), %rcx
1400197f5:     	callq	0x140153380 <.text+0x152380>
1400197fa:     	movl	%eax, %r12d
1400197fd:     	movq	0x808(%rbp), %rdx
140019804:     	cmpq	%r12, %rdx
140019807:     	jbe	0x1400206cc <.text+0x1f6cc>
14001980d:     	leaq	0x1(%r12), %rbx
140019812:     	cmpq	%rdx, %rbx
140019815:     	jae	0x1400206a4 <.text+0x1f6a4>
14001981b:     	movaps	%xmm0, %xmm11
14001981f:     	movq	0x7a0(%rbp), %rax
140019826:     	movsd	(%rax,%r12,8), %xmm7
14001982c:     	movsd	0x8(%rax,%r12,8), %xmm8
140019833:     	movq	0x6b0(%rbp), %rax
14001983a:     	movss	(%rax,%r12,4), %xmm10
140019840:     	movss	0x4(%rax,%r12,4), %xmm9
140019847:     	cmpb	$0x2, 0x7d0(%rbp)
14001984e:     	jae	0x1400198b5 <.text+0x188b5>
140019850:     	movd	%r15d, %xmm1
140019855:     	movd	%esi, %xmm2
140019859:     	movq	0x240(%rbp), %rdx
140019860:     	movq	0x8(%rdx), %rax
140019864:     	movq	0x10(%rdx), %rcx
140019868:     	addq	%rcx, %rcx
14001986b:     	movq	%rax, 0x6c0(%rbp)
140019872:     	movq	%rcx, 0x6c8(%rbp)
140019879:     	movsd	0x50(%rdx), %xmm0
14001987e:     	movsd	%xmm0, 0x6d0(%rbp)
140019886:     	leaq	0x6c0(%rbp), %rcx
14001988d:     	callq	0x140257ba0 <.text+0x256ba0>
140019892:     	addss	0x290c086(%rip), %xmm0  # 0x142925920
14001989a:     	movaps	%xmm6, %xmm1
14001989d:     	cmpunordss	%xmm6, %xmm1
1400198a2:     	movaps	%xmm1, %xmm2
1400198a5:     	andps	%xmm0, %xmm2
1400198a8:     	maxss	%xmm6, %xmm0
1400198ac:     	andnps	%xmm0, %xmm1
1400198af:     	orps	%xmm2, %xmm1
1400198b2:     	movaps	%xmm1, %xmm6
1400198b5:     	movq	0x660(%rbp), %rdi
1400198bc:     	cmpq	0x650(%rbp), %rdi
1400198c3:     	jne	0x1400198d8 <.text+0x188d8>
1400198c5:     	leaq	0x650(%rbp), %rcx
1400198cc:     	leaq	0x290cad5(%rip), %rdx   # 0x1429263a8
1400198d3:     	callq	0x140650050 <.text+0x64f050>
1400198d8:     	subss	%xmm10, %xmm9
1400198dd:     	mulss	%xmm11, %xmm9
1400198e2:     	addss	%xmm10, %xmm9
1400198e7:     	movq	0x658(%rbp), %rax
1400198ee:     	movss	%xmm9, (%rax,%rdi,4)
1400198f4:     	incq	%rdi
1400198f7:     	movq	%rdi, 0x660(%rbp)
1400198fe:     	movq	0x690(%rbp), %rdi
140019905:     	cmpq	0x680(%rbp), %rdi
14001990c:     	jne	0x140019921 <.text+0x18921>
14001990e:     	leaq	0x680(%rbp), %rcx
140019915:     	leaq	0x290caa4(%rip), %rdx   # 0x1429263c0
14001991c:     	callq	0x140b81f90 <.text+0xb80f90>
140019921:     	subps	%xmm7, %xmm8
140019925:     	movsldup	%xmm11, %xmm0           # xmm0 = xmm11[0,0,2,2]
14001992a:     	mulps	%xmm8, %xmm0
14001992e:     	addps	%xmm7, %xmm0
140019931:     	movq	0x688(%rbp), %rax
140019938:     	movlps	%xmm0, (%rax,%rdi,8)
14001993c:     	incq	%rdi
14001993f:     	movq	%rdi, 0x690(%rbp)
140019946:     	movq	0x518(%rbp), %rdi
14001994d:     	cmpq	0x508(%rbp), %rdi
140019954:     	jne	0x140019969 <.text+0x18969>
140019956:     	leaq	0x508(%rbp), %rcx
14001995d:     	leaq	0x290ca74(%rip), %rdx   # 0x1429263d8
140019964:     	callq	0x140b9f180 <.text+0xb9e180>
140019969:     	movq	0x510(%rbp), %rax
140019970:     	leaq	(%rdi,%rdi,2), %rcx
140019974:     	movl	%r15d, (%rax,%rcx,4)
140019978:     	movss	%xmm6, 0x4(%rax,%rcx,4)
14001997e:     	movl	%esi, 0x8(%rax,%rcx,4)
140019982:     	incq	%rdi
140019985:     	movq	%rdi, 0x518(%rbp)
14001998c:     	movq	0x7f0(%rbp), %rdx
140019993:     	cmpq	%r12, %rdx
140019996:     	jbe	0x140020690 <.text+0x1f690>
14001999c:     	cmpq	%rdx, %rbx
14001999f:     	jae	0x1400206b8 <.text+0x1f6b8>
1400199a5:     	movq	0x558(%rbp), %rax
1400199ac:     	movss	(%rax,%r12,4), %xmm6
1400199b2:     	movss	0x4(%rax,%r12,4), %xmm7
1400199b9:     	movq	0x5a0(%rbp), %r12
1400199c0:     	cmpq	0x590(%rbp), %r12
1400199c7:     	jne	0x140019790 <.text+0x18790>
1400199cd:     	leaq	0x590(%rbp), %rcx
1400199d4:     	leaq	0x290ca15(%rip), %rdx   # 0x1429263f0
1400199db:     	callq	0x140650050 <.text+0x64f050>
1400199e0:     	jmp	0x140019790 <.text+0x18790>
1400199e5:     	nopw	%cs:(%rax,%rax)
1400199f0:     	movq	0x650(%rbp), %rsi
1400199f7:     	leaq	0x658(%rbp), %rax
1400199fe:     	movups	(%rax), %xmm0
140019a01:     	movaps	%xmm0, 0x6c0(%rbp)
140019a08:     	movq	0x690(%rbp), %rax
140019a0f:     	leaq	0x6f8(%rbp), %rcx
140019a16:     	movq	%rax, -0x18(%rcx)
140019a1a:     	movups	0x680(%rbp), %xmm0
140019a21:     	movups	%xmm0, -0x28(%rcx)
140019a25:     	movups	0x508(%rbp), %xmm0
140019a2c:     	movaps	%xmm0, 0x300(%rbp)
140019a33:     	movq	0x518(%rbp), %rax
140019a3a:     	movq	%rax, 0x310(%rbp)
140019a41:     	movq	0x590(%rbp), %rbx
140019a48:     	movq	0x598(%rbp), %r15
140019a4f:     	movaps	0x6c0(%rbp), %xmm0
140019a56:     	movaps	0x6d0(%rbp), %xmm1
140019a5d:     	movaps	%xmm0, 0x3e0(%rbp)
140019a64:     	movaps	%xmm1, 0x3f0(%rbp)
140019a6b:     	movq	0x6e0(%rbp), %rax
140019a72:     	movq	%rax, 0x400(%rbp)
140019a79:     	movq	0x460(%rbp), %rax
140019a80:     	testq	%rax, %rax
140019a83:     	je	0x140019a9f <.text+0x18a9f>
140019a85:     	movq	0x468(%rbp), %rcx
140019a8c:     	shlq	$0x2, %rax
140019a90:     	leaq	(%rax,%rax,2), %rdx
140019a94:     	movl	$0x4, %r8d
140019a9a:     	callq	0x140613c20 <.text+0x612c20>
140019a9f:     	movq	0x478(%rbp), %rdx
140019aa6:     	testq	%rdx, %rdx
140019aa9:     	movl	$0x0, %edi
140019aae:     	movq	0x7e0(%rbp), %r14
140019ab5:     	je	0x140019acd <.text+0x18acd>
140019ab7:     	movq	0x480(%rbp), %rcx
140019abe:     	shlq	$0x2, %rdx
140019ac2:     	movl	$0x4, %r8d
140019ac8:     	callq	0x140613c20 <.text+0x612c20>
140019acd:     	movq	0x380(%rbp), %rax
140019ad4:     	testq	%rax, %rax
140019ad7:     	je	0x140019af3 <.text+0x18af3>
140019ad9:     	movq	0x388(%rbp), %rcx
140019ae0:     	shlq	$0x2, %rax
140019ae4:     	leaq	(%rax,%rax,2), %rdx
140019ae8:     	movl	$0x4, %r8d
140019aee:     	callq	0x140613c20 <.text+0x612c20>
140019af3:     	movq	0x398(%rbp), %rdx
140019afa:     	testq	%rdx, %rdx
140019afd:     	je	0x140019b15 <.text+0x18b15>
140019aff:     	movq	0x3a0(%rbp), %rcx
140019b06:     	shlq	$0x2, %rdx
140019b0a:     	movl	$0x4, %r8d
140019b10:     	callq	0x140613c20 <.text+0x612c20>
140019b15:     	movq	%rsi, %rax
140019b18:     	negq	%rax
140019b1b:     	jo	0x140018ee7 <.text+0x17ee7>
140019b21:     	movaps	0x3e0(%rbp), %xmm0
140019b28:     	leaq	0x570(%rbp), %rax
140019b2f:     	movups	%xmm0, (%rax)
140019b32:     	leaq	0x3f0(%rbp), %rcx
140019b39:     	movq	0x10(%rcx), %rax
140019b3d:     	movq	%rax, 0x660(%rbp)
140019b44:     	movups	(%rcx), %xmm0
140019b47:     	movaps	%xmm0, 0x650(%rbp)
140019b4e:     	movq	0x310(%rbp), %rax
140019b55:     	movq	%rax, 0x690(%rbp)
140019b5c:     	movaps	0x300(%rbp), %xmm0
140019b63:     	movaps	%xmm0, 0x680(%rbp)
140019b6a:     	movq	%rsi, 0x568(%rbp)
140019b71:     	movzbl	0x806(%rbp), %esi
140019b78:     	leal	-0x2(%rsi), %eax
140019b7b:     	movq	%rsi, %rcx
140019b7e:     	decq	%rcx
140019b81:     	cmpb	$0x3, %al
140019b83:     	cmovaeq	%rdi, %rcx
140019b87:     	leaq	0x290dd62(%rip), %rdx   # 0x1429278f0
140019b8e:     	movslq	(%rdx,%rcx,4), %rax
140019b92:     	addq	%rdx, %rax
140019b95:     	movq	%rbx, 0x7d0(%rbp)
140019b9c:     	movq	%r15, 0x7f0(%rbp)
140019ba3:     	jmpq	*%rax
140019ba5:     	cmpb	$0x4, %sil
140019ba9:     	jne	0x14001a1f9 <.text+0x191f9>
140019baf:     	xorl	%r14d, %r14d
140019bb2:     	xorl	%r15d, %r15d
140019bb5:     	jmp	0x14001a5f4 <.text+0x195f4>
140019bba:     	movq	0x808(%rbp), %rax
140019bc1:     	movq	%rax, 0x78(%rbp)
140019bc5:     	cmpq	$0x2, %rax
140019bc9:     	jne	0x1400205b8 <.text+0x1f5b8>
140019bcf:     	movsd	(%r15), %xmm15
140019bd4:     	movss	0x8(%r15), %xmm12
140019bda:     	movsd	0xc(%r15), %xmm13
140019be0:     	movss	0x14(%r15), %xmm14
140019be6:     	movq	0x950(%rbp), %rax
140019bed:     	movq	(%rax), %rax
140019bf0:     	movl	0x128(%r14), %ecx
140019bf7:     	movl	0x12c(%r14), %edx
140019bfe:     	movsd	%xmm15, 0x590(%rbp)
140019c07:     	movss	%xmm12, 0x598(%rbp)
140019c10:     	movsd	%xmm13, 0x6c0(%rbp)
140019c19:     	movss	%xmm14, 0x6c8(%rbp)
140019c22:     	movb	$0x1, 0x817(%rbp)
140019c29:     	movl	%edx, 0x38(%rsp)
140019c2d:     	movl	%ecx, 0x30(%rsp)
140019c31:     	movq	0x2d0(%rbp), %rcx
140019c38:     	movq	%rcx, 0x28(%rsp)
140019c3d:     	movq	%rax, 0x20(%rsp)
140019c42:     	leaq	0x650(%rbp), %rcx
140019c49:     	movq	0x2d8(%rbp), %rdx
140019c50:     	movq	%r12, %r8
140019c53:     	leaq	0x6c0(%rbp), %r9
140019c5a:     	callq	0x1407d8fe0 <.text+0x7d7fe0>
140019c5f:     	movq	0x7e0(%rbp), %r10
140019c66:     	movl	0x128(%r10), %r8d
140019c6d:     	movl	0x12c(%r10), %r9d
140019c74:     	movl	%r8d, 0x590(%rbp)
140019c7b:     	movl	%r9d, 0x594(%rbp)
140019c82:     	movq	0x7b0(%rbp), %rax
140019c89:     	movq	0x70(%rax), %rdi
140019c8d:     	testq	%rdi, %rdi
140019c90:     	leaq	0x590(%rbp), %rsi
140019c97:     	je	0x140019da2 <.text+0x18da2>
140019c9d:     	cmpq	$0x1, %rdi
140019ca1:     	jne	0x140019cc6 <.text+0x18cc6>
140019ca3:     	movq	0x7b0(%rbp), %rax
140019caa:     	movq	0x68(%rax), %rax
140019cae:     	cmpl	%r8d, 0x8(%rax)
140019cb2:     	jne	0x140019da2 <.text+0x18da2>
140019cb8:     	cmpl	%r9d, 0xc(%rax)
140019cbc:     	jne	0x140019da2 <.text+0x18da2>
140019cc2:     	xorl	%edx, %edx
140019cc4:     	jmp	0x140019d1e <.text+0x18d1e>
140019cc6:     	movq	0x7b0(%rbp), %rax
140019ccd:     	movq	0x98(%rax), %rcx
140019cd4:     	movq	0xa0(%rax), %rdx
140019cdb:     	callq	0x140106840 <.text+0x105840>
140019ce0:     	movb	$0x1, 0x817(%rbp)
140019ce7:     	movq	0x340(%rbp), %rcx
140019cee:     	movq	%rax, %rdx
140019cf1:     	movq	%rsi, %r8
140019cf4:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
140019cf9:     	cmpq	$0x1, %rax
140019cfd:     	movq	0x7e0(%rbp), %r10
140019d04:     	jne	0x140019da2 <.text+0x18da2>
140019d0a:     	cmpq	%rdi, %rdx
140019d0d:     	jae	0x140020b82 <.text+0x1fb82>
140019d13:     	movq	0x7b0(%rbp), %rax
140019d1a:     	movq	0x68(%rax), %rax
140019d1e:     	leaq	(%rdx,%rdx,4), %rcx
140019d22:     	movups	0x10(%rax,%rcx,8), %xmm0
140019d27:     	movaps	%xmm0, 0x460(%rbp)
140019d2e:     	movl	0x20(%rax,%rcx,8), %eax
140019d32:     	movl	%eax, 0x470(%rbp)
140019d38:     	movzbl	0x460(%rbp), %ecx
140019d3f:     	movb	$0x2, 0x34c(%rbp)
140019d46:     	movl	0x35c(%rbp), %eax
140019d4c:     	movl	%eax, 0x5a0(%rbp)
140019d52:     	movzbl	0x34c(%rbp), %eax
140019d59:     	movb	%al, 0x590(%rbp)
140019d5f:     	movq	0x34d(%rbp), %rax
140019d66:     	movq	%rax, 0x591(%rbp)
140019d6d:     	movl	0x355(%rbp), %eax
140019d73:     	movl	%eax, 0x599(%rbp)
140019d79:     	movzwl	0x359(%rbp), %eax
140019d80:     	movw	%ax, 0x59d(%rbp)
140019d87:     	movzbl	0x35b(%rbp), %eax
140019d8e:     	movb	%al, 0x59f(%rbp)
140019d94:     	leaq	0x460(%rbp), %rax
140019d9b:     	cmpb	$0x3, %cl
140019d9e:     	je	0x140019dfe <.text+0x18dfe>
140019da0:     	jmp	0x140019e01 <.text+0x18e01>
140019da2:     	movb	$0x3, 0x460(%rbp)
140019da9:     	movb	$0x2, 0x34c(%rbp)
140019db0:     	movl	0x35c(%rbp), %eax
140019db6:     	movl	%eax, 0x5a0(%rbp)
140019dbc:     	movzbl	0x34c(%rbp), %eax
140019dc3:     	movb	%al, 0x590(%rbp)
140019dc9:     	movq	0x34d(%rbp), %rax
140019dd0:     	movq	%rax, 0x591(%rbp)
140019dd7:     	movl	0x355(%rbp), %eax
140019ddd:     	movl	%eax, 0x599(%rbp)
140019de3:     	movzwl	0x359(%rbp), %eax
140019dea:     	movw	%ax, 0x59d(%rbp)
140019df1:     	movzbl	0x35b(%rbp), %eax
140019df8:     	movb	%al, 0x59f(%rbp)
140019dfe:     	movq	%rsi, %rax
140019e01:     	movl	0x10(%rax), %ecx
140019e04:     	movl	%ecx, 0x390(%rbp)
140019e0a:     	movups	(%rax), %xmm0
140019e0d:     	movaps	%xmm0, 0x380(%rbp)
140019e14:     	movl	0x128(%r10), %r9d
140019e1b:     	movl	0x12c(%r10), %r8d
140019e22:     	movl	%r8d, 0x590(%rbp)
140019e29:     	movl	%r9d, 0x594(%rbp)
140019e30:     	movq	0x7b0(%rbp), %rax
140019e37:     	movq	0x70(%rax), %rdi
140019e3b:     	testq	%rdi, %rdi
140019e3e:     	je	0x140019eeb <.text+0x18eeb>
140019e44:     	cmpq	$0x1, %rdi
140019e48:     	jne	0x140019e6d <.text+0x18e6d>
140019e4a:     	movq	0x7b0(%rbp), %rax
140019e51:     	movq	0x68(%rax), %rax
140019e55:     	cmpl	%r8d, 0x8(%rax)
140019e59:     	jne	0x140019eeb <.text+0x18eeb>
140019e5f:     	cmpl	%r9d, 0xc(%rax)
140019e63:     	jne	0x140019eeb <.text+0x18eeb>
140019e69:     	xorl	%edx, %edx
140019e6b:     	jmp	0x140019eba <.text+0x18eba>
140019e6d:     	movq	0x7b0(%rbp), %rax
140019e74:     	movq	0x98(%rax), %rcx
140019e7b:     	movq	0xa0(%rax), %rdx
140019e82:     	callq	0x140106840 <.text+0x105840>
140019e87:     	movb	$0x1, 0x817(%rbp)
140019e8e:     	movq	0x340(%rbp), %rcx
140019e95:     	movq	%rax, %rdx
140019e98:     	movq	%rsi, %r8
140019e9b:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
140019ea0:     	cmpq	$0x1, %rax
140019ea4:     	jne	0x140019eeb <.text+0x18eeb>
140019ea6:     	cmpq	%rdi, %rdx
140019ea9:     	jae	0x140020ba0 <.text+0x1fba0>
140019eaf:     	movq	0x7b0(%rbp), %rax
140019eb6:     	movq	0x68(%rax), %rax
140019eba:     	leaq	(%rdx,%rdx,4), %rcx
140019ebe:     	movups	0x10(%rax,%rcx,8), %xmm0
140019ec3:     	movaps	%xmm0, 0x590(%rbp)
140019eca:     	movl	0x20(%rax,%rcx,8), %eax
140019ece:     	cmpb	$0x3, 0x590(%rbp)
140019ed5:     	movl	%eax, 0x5a0(%rbp)
140019edb:     	movq	%rsi, %rax
140019ede:     	leaq	0x34c(%rbp), %rcx
140019ee5:     	cmoveq	%rcx, %rax
140019ee9:     	jmp	0x140019ef9 <.text+0x18ef9>
140019eeb:     	movb	$0x3, 0x590(%rbp)
140019ef2:     	leaq	0x34c(%rbp), %rax
140019ef9:     	leaq	0x460(%rbp), %rcx
140019f00:     	movl	0x10(%rax), %r8d
140019f04:     	leaq	0x6f8(%rbp), %rdx
140019f0b:     	movl	%r8d, -0x14(%rdx)
140019f0f:     	movups	(%rax), %xmm0
140019f12:     	movups	%xmm0, -0x24(%rdx)
140019f16:     	movaps	0x380(%rbp), %xmm0
140019f1d:     	movaps	%xmm0, 0x6c0(%rbp)
140019f24:     	movl	0x390(%rbp), %eax
140019f2a:     	movl	%eax, 0x6d0(%rbp)
140019f30:     	movlps	%xmm15, 0x460(%rbp)
140019f38:     	movss	%xmm12, 0x468(%rbp)
140019f41:     	movlps	%xmm13, 0x590(%rbp)
140019f49:     	movss	%xmm14, 0x598(%rbp)
140019f52:     	movb	$0x1, 0x817(%rbp)
140019f59:     	movq	%rsi, %rdx
140019f5c:     	movq	0x2d8(%rbp), %r8
140019f63:     	leaq	0x6c0(%rbp), %r9
140019f6a:     	callq	0x1407de800 <.text+0x7dd800>
140019f6f:     	movb	%al, 0x804(%rbp)
140019f75:     	movq	0x7a0(%rbp), %rax
140019f7c:     	movsd	(%rax), %xmm6
140019f80:     	movsd	0x8(%rax), %xmm11
140019f86:     	movups	(%rax), %xmm0
140019f89:     	movaps	%xmm0, 0x300(%rbp)
140019f90:     	movq	0x6b0(%rbp), %rax
140019f97:     	movsd	(%rax), %xmm0
140019f9b:     	movsd	%xmm0, 0x550(%rbp)
140019fa3:     	movq	0x7f0(%rbp), %rax
140019faa:     	cmpq	$0x1, %rax
140019fae:     	je	0x140020b12 <.text+0x1fb12>
140019fb4:     	testq	%rax, %rax
140019fb7:     	je	0x140020af6 <.text+0x1faf6>
140019fbd:     	movsd	(%r13), %xmm10
140019fc3:     	cmpb	$0x0, 0x530(%rbp)
140019fca:     	je	0x14001c469 <.text+0x1b469>
140019fd0:     	movb	$0x0, 0x804(%rbp)
140019fd7:     	movzbl	0x361ac2b(%rip), %eax   # 0x143634c09
140019fde:     	movl	$0x18, %ecx
140019fe3:     	movl	$0x4, %edx
140019fe8:     	callq	0x140613c10 <.text+0x612c10>
140019fed:     	testq	%rax, %rax
140019ff0:     	leaq	0x590(%rbp), %rcx
140019ff7:     	je	0x140020b60 <.text+0x1fb60>
140019ffd:     	movq	0x660(%rbp), %rdx
14001a004:     	movq	%rdx, 0x10(%rax)
14001a008:     	movups	0x650(%rbp), %xmm0
14001a00f:     	movups	%xmm0, (%rax)
14001a012:     	movq	$0x2, 0x6c0(%rbp)
14001a01d:     	movq	%rax, 0x6c8(%rbp)
14001a024:     	movq	$0x2, 0x6d0(%rbp)
14001a02f:     	movb	$0x1, 0x817(%rbp)
14001a036:     	leaq	0x6c0(%rbp), %rdx
14001a03d:     	xorl	%r8d, %r8d
14001a040:     	callq	0x140153640 <.text+0x152640>
14001a045:     	xorl	%esi, %esi
14001a047:     	cmpq	0x590(%rbp), %rsi
14001a04e:     	jo	0x14001c469 <.text+0x1b469>
14001a054:     	movq	0x5c0(%rbp), %rax
14001a05b:     	movq	%rax, 0x6f0(%rbp)
14001a062:     	movups	0x590(%rbp), %xmm0
14001a069:     	movups	0x5a0(%rbp), %xmm1
14001a070:     	movups	0x5b0(%rbp), %xmm2
14001a077:     	movaps	%xmm2, 0x6e0(%rbp)
14001a07e:     	movaps	%xmm1, 0x6d0(%rbp)
14001a085:     	movaps	%xmm0, 0x6c0(%rbp)
14001a08c:     	leaq	0x460(%rbp), %rcx
14001a093:     	leaq	0x6c0(%rbp), %rdx
14001a09a:     	movd	0x290b7ce(%rip), %xmm2  # 0x142925870
14001a0a2:     	movl	$0x1, %r9d
14001a0a8:     	callq	0x140152b00 <.text+0x151b00>
14001a0ad:     	movq	0x6c0(%rbp), %rax
14001a0b4:     	testq	%rax, %rax
14001a0b7:     	movq	0x338(%rbp), %r13
14001a0be:     	movaps	0x290b86a(%rip), %xmm9  # 0x142925930
14001a0c6:     	je	0x14001a0e2 <.text+0x190e2>
14001a0c8:     	movq	0x6c8(%rbp), %rcx
14001a0cf:     	shlq	$0x2, %rax
14001a0d3:     	leaq	(%rax,%rax,2), %rdx
14001a0d7:     	movl	$0x4, %r8d
14001a0dd:     	callq	0x140613c20 <.text+0x612c20>
14001a0e2:     	movq	0x6d8(%rbp), %rdx
14001a0e9:     	testq	%rdx, %rdx
14001a0ec:     	je	0x14001a104 <.text+0x19104>
14001a0ee:     	movq	0x6e0(%rbp), %rcx
14001a0f5:     	shlq	$0x2, %rdx
14001a0f9:     	movl	$0x4, %r8d
14001a0ff:     	callq	0x140613c20 <.text+0x612c20>
14001a104:     	cmpq	0x460(%rbp), %rsi
14001a10b:     	jo	0x14001c469 <.text+0x1b469>
14001a111:     	movups	0x460(%rbp), %xmm0
14001a118:     	movups	0x470(%rbp), %xmm1
14001a11f:     	movups	0x480(%rbp), %xmm2
14001a126:     	movaps	%xmm1, 0x5a0(%rbp)
14001a12d:     	movaps	%xmm2, 0x5b0(%rbp)
14001a134:     	movq	0x490(%rbp), %rax
14001a13b:     	movq	%rax, 0x5c0(%rbp)
14001a142:     	movaps	%xmm0, 0x590(%rbp)
14001a149:     	movq	0x5a0(%rbp), %rbx
14001a150:     	movq	0x5b8(%rbp), %rax
14001a157:     	cmpq	%rbx, %rax
14001a15a:     	cmovbq	%rax, %rbx
14001a15e:     	leaq	(,%rbx,8), %rdi
14001a166:     	movq	%rbx, %rax
14001a169:     	shrq	$0x3d, %rax
14001a16d:     	jne	0x140020d2d <.text+0x1fd2d>
14001a173:     	movl	$0x0, %esi
14001a178:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14001a182:     	cmpq	%rax, %rdi
14001a185:     	movss	0x290b776(%rip), %xmm8  # 0x142925904
14001a18e:     	ja	0x140020d2f <.text+0x1fd2f>
14001a194:     	movq	0x5b0(%rbp), %r14
14001a19b:     	testq	%rdi, %rdi
14001a19e:     	je	0x14001a22d <.text+0x1922d>
14001a1a4:     	movzbl	0x361aa5e(%rip), %eax   # 0x143634c09
14001a1ab:     	movl	$0x4, %esi
14001a1b0:     	movl	$0x4, %edx
14001a1b5:     	movq	%rdi, %rcx
14001a1b8:     	callq	0x140613c10 <.text+0x612c10>
14001a1bd:     	testq	%rax, %rax
14001a1c0:     	je	0x140020d2f <.text+0x1fd2f>
14001a1c6:     	movq	%rbx, 0x808(%rbp)
14001a1cd:     	testq	%rbx, %rbx
14001a1d0:     	jne	0x14001a246 <.text+0x19246>
14001a1d2:     	jmp	0x14001a313 <.text+0x19313>
14001a1d7:     	cmpb	$0x0, 0x6a8(%rbp)
14001a1de:     	je	0x14001ad53 <.text+0x19d53>
14001a1e4:     	cmpb	$0x4, %sil
14001a1e8:     	jne	0x14001a972 <.text+0x19972>
14001a1ee:     	xorl	%r14d, %r14d
14001a1f1:     	xorl	%r15d, %r15d
14001a1f4:     	jmp	0x14001ac65 <.text+0x19c65>
14001a1f9:     	cmpq	$0x0, 0x430(%rbp)
14001a201:     	je	0x14001a5cb <.text+0x195cb>
14001a207:     	xorl	%eax, %eax
14001a209:     	movq	0x698(%rbp), %rcx
14001a210:     	cmpq	0x580(%rbp), %rcx
14001a217:     	jne	0x14001a5ea <.text+0x195ea>
14001a21d:     	cmpb	$0x1, 0x108(%r14)
14001a225:     	sete	%cl
14001a228:     	jmp	0x14001a5ec <.text+0x195ec>
14001a22d:     	movl	$0x4, %eax
14001a232:     	movq	$0x0, 0x808(%rbp)
14001a23d:     	testq	%rbx, %rbx
14001a240:     	je	0x14001a313 <.text+0x19313>
14001a246:     	cmpq	$0x8, %rbx
14001a24a:     	jae	0x14001a401 <.text+0x19401>
14001a250:     	xorl	%r8d, %r8d
14001a253:     	movq	%r8, %rcx
14001a256:     	orq	$0x1, %rcx
14001a25a:     	testb	$0x1, %bl
14001a25d:     	je	0x14001a296 <.text+0x19296>
14001a25f:     	movss	(%r14,%r8,4), %xmm0
14001a265:     	movaps	%xmm8, %xmm1
14001a269:     	subss	%xmm0, %xmm1
14001a26d:     	movsd	0x300(%rbp), %xmm2
14001a275:     	movsd	0x308(%rbp), %xmm3
14001a27d:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14001a281:     	mulps	%xmm2, %xmm1
14001a284:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14001a288:     	mulps	%xmm3, %xmm0
14001a28b:     	addps	%xmm1, %xmm0
14001a28e:     	movlps	%xmm0, (%rax,%r8,8)
14001a293:     	movq	%rcx, %r8
14001a296:     	cmpq	%rcx, %rbx
14001a299:     	je	0x14001a313 <.text+0x19313>
14001a29f:     	nop
14001a2a0:     	movss	(%r14,%r8,4), %xmm0
14001a2a6:     	movaps	%xmm8, %xmm1
14001a2aa:     	subss	%xmm0, %xmm1
14001a2ae:     	movsd	0x300(%rbp), %xmm2
14001a2b6:     	movsd	0x308(%rbp), %xmm3
14001a2be:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14001a2c2:     	mulps	%xmm2, %xmm1
14001a2c5:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14001a2c9:     	mulps	%xmm3, %xmm0
14001a2cc:     	addps	%xmm1, %xmm0
14001a2cf:     	movlps	%xmm0, (%rax,%r8,8)
14001a2d4:     	movss	0x4(%r14,%r8,4), %xmm0
14001a2db:     	movaps	%xmm8, %xmm1
14001a2df:     	subss	%xmm0, %xmm1
14001a2e3:     	movsd	0x300(%rbp), %xmm2
14001a2eb:     	movsd	0x308(%rbp), %xmm3
14001a2f3:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14001a2f7:     	mulps	%xmm2, %xmm1
14001a2fa:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14001a2fe:     	mulps	%xmm3, %xmm0
14001a301:     	addps	%xmm1, %xmm0
14001a304:     	movlps	%xmm0, 0x8(%rax,%r8,8)
14001a30a:     	addq	$0x2, %r8
14001a30e:     	cmpq	%r8, %rbx
14001a311:     	jne	0x14001a2a0 <.text+0x192a0>
14001a313:     	movq	%rax, 0x7a0(%rbp)
14001a31a:     	movq	0x5a0(%rbp), %r14
14001a321:     	movq	0x5b8(%rbp), %rax
14001a328:     	cmpq	%r14, %rax
14001a32b:     	cmovbq	%rax, %r14
14001a32f:     	leaq	(,%r14,4), %rdi
14001a337:     	movq	%r14, %rax
14001a33a:     	shrq	$0x3e, %rax
14001a33e:     	jne	0x140020d4d <.text+0x1fd4d>
14001a344:     	movl	$0x0, %esi
14001a349:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14001a353:     	cmpq	%rax, %rdi
14001a356:     	ja	0x140020d4f <.text+0x1fd4f>
14001a35c:     	movq	0x5b0(%rbp), %r15
14001a363:     	testq	%rdi, %rdi
14001a366:     	je	0x14001a39b <.text+0x1939b>
14001a368:     	movzbl	0x361a89a(%rip), %eax   # 0x143634c09
14001a36f:     	movl	$0x4, %esi
14001a374:     	movl	$0x4, %edx
14001a379:     	movq	%rdi, %rcx
14001a37c:     	callq	0x140613c10 <.text+0x612c10>
14001a381:     	testq	%rax, %rax
14001a384:     	je	0x140020d4f <.text+0x1fd4f>
14001a38a:     	movq	%r14, 0x6b0(%rbp)
14001a391:     	testq	%r14, %r14
14001a394:     	jne	0x14001a3b4 <.text+0x193b4>
14001a396:     	jmp	0x14001c19f <.text+0x1b19f>
14001a39b:     	movl	$0x4, %eax
14001a3a0:     	movq	$0x0, 0x6b0(%rbp)
14001a3ab:     	testq	%r14, %r14
14001a3ae:     	je	0x14001c19f <.text+0x1b19f>
14001a3b4:     	cmpq	$0x8, %r14
14001a3b8:     	jae	0x14001a514 <.text+0x19514>
14001a3be:     	xorl	%r8d, %r8d
14001a3c1:     	movq	%r8, %rcx
14001a3c4:     	orq	$0x1, %rcx
14001a3c8:     	testb	$0x1, %r14b
14001a3cc:     	je	0x14001c13b <.text+0x1b13b>
14001a3d2:     	movss	(%r15,%r8,4), %xmm0
14001a3d8:     	movaps	%xmm8, %xmm1
14001a3dc:     	subss	%xmm0, %xmm1
14001a3e0:     	movsd	0x550(%rbp), %xmm2
14001a3e8:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001a3eb:     	mulps	%xmm2, %xmm1
14001a3ee:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001a3f2:     	addss	%xmm1, %xmm0
14001a3f6:     	movss	%xmm0, (%rax,%r8,4)
14001a3fc:     	jmp	0x14001c19a <.text+0x1b19a>
14001a401:     	leaq	(%rax,%rbx,8), %rcx
14001a405:     	addq	$-0x8, %rcx
14001a409:     	cmpq	%rax, %rcx
14001a40c:     	jb	0x14001a250 <.text+0x19250>
14001a412:     	leaq	0x4(%rax), %rdx
14001a416:     	leaq	(%rax,%rbx,8), %rcx
14001a41a:     	addq	$-0x4, %rcx
14001a41e:     	cmpq	%rdx, %rcx
14001a421:     	jb	0x14001a250 <.text+0x19250>
14001a427:     	leaq	-0x1(%rbx), %rcx
14001a42b:     	shrq	$0x3d, %rcx
14001a42f:     	movl	$0x0, %r8d
14001a435:     	jne	0x14001a253 <.text+0x19253>
14001a43b:     	addq	%rax, %rdi
14001a43e:     	leaq	(%r14,%rbx,4), %rcx
14001a442:     	cmpq	%rax, %rcx
14001a445:     	seta	%r9b
14001a449:     	cmpq	%rdi, %r14
14001a44c:     	setb	%r8b
14001a450:     	leaq	0x310(%rbp), %rcx
14001a457:     	cmpq	%rax, %rcx
14001a45a:     	seta	%cl
14001a45d:     	leaq	0x300(%rbp), %rdx
14001a464:     	cmpq	%rdi, %rdx
14001a467:     	setb	%dl
14001a46a:     	testb	%r8b, %r9b
14001a46d:     	jne	0x14001a250 <.text+0x19250>
14001a473:     	movl	$0x0, %r8d
14001a479:     	andb	%dl, %cl
14001a47b:     	jne	0x14001a253 <.text+0x19253>
14001a481:     	movaps	%xmm6, 0x7a0(%rbp)
14001a488:     	movq	%rbx, %r8
14001a48b:     	movabsq	$0x1ffffffffffffffc, %rcx # imm = 0x1FFFFFFFFFFFFFFC
14001a495:     	andq	%rcx, %r8
14001a498:     	movaps	0x300(%rbp), %xmm0
14001a49f:     	movaps	%xmm0, %xmm1
14001a4a2:     	shufps	$0xff, %xmm0, %xmm1     # xmm1 = xmm1[3,3],xmm0[3,3]
14001a4a6:     	movaps	%xmm0, %xmm2
14001a4a9:     	shufps	$0xaa, %xmm0, %xmm2     # xmm2 = xmm2[2,2],xmm0[2,2]
14001a4ad:     	movaps	%xmm0, %xmm3
14001a4b0:     	shufps	$0x55, %xmm0, %xmm3     # xmm3 = xmm3[1,1],xmm0[1,1]
14001a4b4:     	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14001a4b8:     	xorl	%ecx, %ecx
14001a4ba:     	nopw	(%rax,%rax)
14001a4c0:     	movups	(%r14,%rcx,4), %xmm4
14001a4c5:     	movaps	%xmm9, %xmm5
14001a4c9:     	subps	%xmm4, %xmm5
14001a4cc:     	movaps	%xmm5, %xmm6
14001a4cf:     	mulps	%xmm0, %xmm6
14001a4d2:     	mulps	%xmm3, %xmm5
14001a4d5:     	movaps	%xmm4, %xmm7
14001a4d8:     	mulps	%xmm2, %xmm7
14001a4db:     	addps	%xmm6, %xmm7
14001a4de:     	mulps	%xmm1, %xmm4
14001a4e1:     	addps	%xmm5, %xmm4
14001a4e4:     	movaps	%xmm7, %xmm5
14001a4e7:     	unpcklps	%xmm4, %xmm5            # xmm5 = xmm5[0],xmm4[0],xmm5[1],xmm4[1]
14001a4ea:     	unpckhps	%xmm4, %xmm7            # xmm7 = xmm7[2],xmm4[2],xmm7[3],xmm4[3]
14001a4ed:     	movups	%xmm7, 0x10(%rax,%rcx,8)
14001a4f2:     	movups	%xmm5, (%rax,%rcx,8)
14001a4f6:     	addq	$0x4, %rcx
14001a4fa:     	cmpq	%rcx, %r8
14001a4fd:     	jne	0x14001a4c0 <.text+0x194c0>
14001a4ff:     	cmpq	%r8, %rbx
14001a502:     	movaps	0x7a0(%rbp), %xmm6
14001a509:     	jne	0x14001a253 <.text+0x19253>
14001a50f:     	jmp	0x14001a313 <.text+0x19313>
14001a514:     	leaq	(%rax,%rdi), %rdx
14001a518:     	addq	%r15, %rdi
14001a51b:     	leaq	0x558(%rbp), %rcx
14001a522:     	cmpq	%rax, %rcx
14001a525:     	seta	%r8b
14001a529:     	leaq	0x550(%rbp), %rcx
14001a530:     	cmpq	%rdx, %rcx
14001a533:     	setb	%r9b
14001a537:     	cmpq	%rax, %rdi
14001a53a:     	seta	%cl
14001a53d:     	cmpq	%rdx, %r15
14001a540:     	setb	%dl
14001a543:     	testb	%r9b, %r8b
14001a546:     	jne	0x14001a3be <.text+0x193be>
14001a54c:     	movl	$0x0, %r8d
14001a552:     	andb	%dl, %cl
14001a554:     	jne	0x14001a3c1 <.text+0x193c1>
14001a55a:     	movq	%r14, %r8
14001a55d:     	movabsq	$0x3ffffffffffffff8, %rcx # imm = 0x3FFFFFFFFFFFFFF8
14001a567:     	andq	%rcx, %r8
14001a56a:     	movaps	0x550(%rbp), %xmm0
14001a571:     	movaps	%xmm0, %xmm1
14001a574:     	shufps	$0x0, %xmm0, %xmm1      # xmm1 = xmm1[0,0],xmm0[0,0]
14001a578:     	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
14001a57c:     	xorl	%ecx, %ecx
14001a57e:     	nop
14001a580:     	movups	(%r15,%rcx,4), %xmm2
14001a585:     	movups	0x10(%r15,%rcx,4), %xmm3
14001a58b:     	movaps	%xmm9, %xmm4
14001a58f:     	subps	%xmm2, %xmm4
14001a592:     	movaps	%xmm9, %xmm5
14001a596:     	subps	%xmm3, %xmm5
14001a599:     	mulps	%xmm1, %xmm4
14001a59c:     	mulps	%xmm1, %xmm5
14001a59f:     	mulps	%xmm0, %xmm2
14001a5a2:     	addps	%xmm4, %xmm2
14001a5a5:     	mulps	%xmm0, %xmm3
14001a5a8:     	addps	%xmm5, %xmm3
14001a5ab:     	movups	%xmm2, (%rax,%rcx,4)
14001a5af:     	movups	%xmm3, 0x10(%rax,%rcx,4)
14001a5b4:     	addq	$0x8, %rcx
14001a5b8:     	cmpq	%rcx, %r8
14001a5bb:     	jne	0x14001a580 <.text+0x19580>
14001a5bd:     	cmpq	%r8, %r14
14001a5c0:     	jne	0x14001a3c1 <.text+0x193c1>
14001a5c6:     	jmp	0x14001c19f <.text+0x1b19f>
14001a5cb:     	cmpb	$0x1, 0xe8(%r14)
14001a5d3:     	sete	%al
14001a5d6:     	movq	0x698(%rbp), %rcx
14001a5dd:     	cmpq	0x580(%rbp), %rcx
14001a5e4:     	je	0x14001a21d <.text+0x1921d>
14001a5ea:     	xorl	%ecx, %ecx
14001a5ec:     	movzbl	%al, %r14d
14001a5f0:     	movzbl	%cl, %r15d
14001a5f4:     	movq	0x938(%rbp), %rax
14001a5fb:     	movq	(%rax), %rdi
14001a5fe:     	movb	0x807(%rbp), %bl
14001a604:     	movb	$0x1, 0x816(%rbp)
14001a60b:     	movb	$0x1, 0x815(%rbp)
14001a612:     	leaq	0x290(%rbp), %rcx
14001a619:     	callq	0x1400284b0 <.text+0x274b0>
14001a61e:     	movzwl	%r15w, %ecx
14001a622:     	shll	$0x8, %ecx
14001a625:     	movzwl	%r14w, %r8d
14001a629:     	orl	%ecx, %r8d
14001a62c:     	movl	0x500(%rbp), %ecx
14001a632:     	movl	%ecx, 0x6c0(%rbp)
14001a638:     	movss	0x548(%rbp), %xmm0
14001a640:     	movss	%xmm0, 0x6c4(%rbp)
14001a648:     	movl	0x540(%rbp), %ecx
14001a64e:     	movl	%ecx, 0x6c8(%rbp)
14001a654:     	movss	0x370(%rbp), %xmm0
14001a65c:     	movss	%xmm0, 0x6cc(%rbp)
14001a664:     	movb	$0x1, 0x816(%rbp)
14001a66b:     	movb	$0x1, 0x815(%rbp)
14001a672:     	andb	$0x1, %dl
14001a675:     	movb	%dl, 0x68(%rsp)
14001a679:     	andb	$0x1, %al
14001a67b:     	movb	%al, 0x60(%rsp)
14001a67f:     	movb	%bl, 0x58(%rsp)
14001a683:     	movq	%r12, 0x80(%rsp)
14001a68b:     	movq	0x7f0(%rbp), %rax
14001a692:     	movq	%rax, 0x78(%rsp)
14001a697:     	leaq	0x6c0(%rbp), %rax
14001a69e:     	movq	%rax, 0x70(%rsp)
14001a6a3:     	movb	%sil, 0x50(%rsp)
14001a6a8:     	movl	0x6a8(%rbp), %eax
14001a6ae:     	movb	%al, 0x48(%rsp)
14001a6b2:     	movw	%r8w, 0x40(%rsp)
14001a6b8:     	movq	%rdi, 0x38(%rsp)
14001a6bd:     	movl	0x640(%rbp), %eax
14001a6c3:     	movl	%eax, 0x30(%rsp)
14001a6c7:     	movq	0x788(%rbp), %rax
14001a6ce:     	movl	%eax, 0x28(%rsp)
14001a6d2:     	leaq	0x428(%rbp), %rax
14001a6d9:     	movq	%rax, 0x20(%rsp)
14001a6de:     	leaq	0x140(%rbp), %rcx
14001a6e5:     	leaq	0x568(%rbp), %rdx
14001a6ec:     	leaq	0x650(%rbp), %r8
14001a6f3:     	leaq	0x680(%rbp), %r9
14001a6fa:     	callq	0x1407f98e0 <.text+0x7f88e0>
14001a6ff:     	movb	$0x1, 0x816(%rbp)
14001a706:     	movb	$0x1, 0x815(%rbp)
14001a70d:     	movq	0x268(%rbp), %rcx
14001a714:     	callq	0x140987ca0 <.text+0x986ca0>
14001a719:     	movq	0x570(%rbp), %rdx
14001a720:     	movq	0x578(%rbp), %r8
14001a727:     	movq	0x658(%rbp), %r9
14001a72e:     	movq	0x660(%rbp), %rcx
14001a735:     	movups	0x688(%rbp), %xmm0
14001a73c:     	movl	0x500(%rbp), %r10d
14001a743:     	movl	%r10d, 0x6c0(%rbp)
14001a74a:     	movss	0x548(%rbp), %xmm1
14001a752:     	movss	%xmm1, 0x6c4(%rbp)
14001a75a:     	movl	0x540(%rbp), %r10d
14001a761:     	movl	%r10d, 0x6c8(%rbp)
14001a768:     	movss	0x370(%rbp), %xmm1
14001a770:     	movss	%xmm1, 0x6cc(%rbp)
14001a778:     	movb	$0x1, 0x816(%rbp)
14001a77f:     	movb	$0x1, 0x815(%rbp)
14001a786:     	movq	%r12, 0x60(%rsp)
14001a78b:     	movq	0x7f0(%rbp), %r10
14001a792:     	movq	%r10, 0x58(%rsp)
14001a797:     	leaq	0x6c0(%rbp), %r10
14001a79e:     	movq	%r10, 0x50(%rsp)
14001a7a3:     	movq	0x788(%rbp), %r10
14001a7aa:     	movl	%r10d, 0x48(%rsp)
14001a7af:     	movq	0x940(%rbp), %r10
14001a7b6:     	movq	%r10, 0x40(%rsp)
14001a7bb:     	leaq	0x140(%rbp), %r10
14001a7c2:     	movq	%r10, 0x38(%rsp)
14001a7c7:     	movups	%xmm0, 0x28(%rsp)
14001a7cc:     	movq	%rcx, 0x20(%rsp)
14001a7d1:     	movq	%rax, %rcx
14001a7d4:     	callq	0x1407dc170 <.text+0x7db170>
14001a7d9:     	movq	0x7e0(%rbp), %rax
14001a7e0:     	cmpb	$0x0, 0xe8(%rax)
14001a7e7:     	je	0x14001a939 <.text+0x19939>
14001a7ed:     	xorl	%eax, %eax
14001a7ef:     	movq	0x7f0(%rbp), %r10
14001a7f6:     	movd	0x290b0d6(%rip), %xmm0  # 0x1429258d4
14001a7fe:     	testb	%al, %al
14001a800:     	jne	0x14001a80a <.text+0x1980a>
14001a802:     	movd	0x290b0ce(%rip), %xmm0  # 0x1429258d8
14001a80a:     	movq	0x570(%rbp), %rcx
14001a811:     	movq	0x578(%rbp), %rdx
14001a818:     	movq	0x658(%rbp), %r8
14001a81f:     	movq	0x660(%rbp), %r9
14001a826:     	movups	0x688(%rbp), %xmm1
14001a82d:     	movq	0x938(%rbp), %rax
14001a834:     	movq	(%rax), %rax
14001a837:     	movb	$0x1, 0x816(%rbp)
14001a83e:     	movb	$0x1, 0x815(%rbp)
14001a845:     	movq	%r12, 0x78(%rsp)
14001a84a:     	movq	%r10, 0x70(%rsp)
14001a84f:     	movq	0x240(%rbp), %r10
14001a856:     	movq	%r10, 0x68(%rsp)
14001a85b:     	movd	%xmm0, 0x60(%rsp)
14001a861:     	movl	0x6a8(%rbp), %r10d
14001a868:     	movb	%r10b, 0x58(%rsp)
14001a86d:     	movq	%rax, 0x48(%rsp)
14001a872:     	movq	0x788(%rbp), %rax
14001a879:     	movl	%eax, 0x40(%rsp)
14001a87d:     	leaq	0x140(%rbp), %rax
14001a884:     	movq	%rax, 0x38(%rsp)
14001a889:     	leaq	0x428(%rbp), %rax
14001a890:     	movq	%rax, 0x30(%rsp)
14001a895:     	movups	%xmm1, 0x20(%rsp)
14001a89a:     	movb	$0x1, 0x80(%rsp)
14001a8a2:     	movw	$0x0, 0x50(%rsp)
14001a8a9:     	callq	0x1407da0b0 <.text+0x7d90b0>
14001a8ae:     	cmpb	$0x0, 0x6a8(%rbp)
14001a8b5:     	je	0x14001af32 <.text+0x19f32>
14001a8bb:     	movzbl	0x806(%rbp), %ebx
14001a8c2:     	cmpb	$0x1, %bl
14001a8c5:     	ja	0x14001a95f <.text+0x1995f>
14001a8cb:     	movzbl	0x807(%rbp), %esi
14001a8d2:     	movq	0x7e0(%rbp), %r10
14001a8d9:     	movl	0x128(%r10), %r8d
14001a8e0:     	movl	0x12c(%r10), %r9d
14001a8e7:     	movl	%r8d, 0x6c0(%rbp)
14001a8ee:     	movl	%r9d, 0x6c4(%rbp)
14001a8f5:     	movq	0x7b0(%rbp), %rax
14001a8fc:     	movq	0x70(%rax), %rdi
14001a900:     	testq	%rdi, %rdi
14001a903:     	je	0x14001aa14 <.text+0x19a14>
14001a909:     	cmpq	$0x1, %rdi
14001a90d:     	jne	0x14001a9a6 <.text+0x199a6>
14001a913:     	movq	0x7b0(%rbp), %rax
14001a91a:     	movq	0x68(%rax), %rax
14001a91e:     	cmpl	%r8d, 0x8(%rax)
14001a922:     	jne	0x14001aa14 <.text+0x19a14>
14001a928:     	cmpl	%r9d, 0xc(%rax)
14001a92c:     	jne	0x14001aa14 <.text+0x19a14>
14001a932:     	xorl	%edx, %edx
14001a934:     	jmp	0x14001aa05 <.text+0x19a05>
14001a939:     	cmpb	$0x0, 0x108(%rax)
14001a940:     	sete	%al
14001a943:     	movq	0x7f0(%rbp), %r10
14001a94a:     	movd	0x290af82(%rip), %xmm0  # 0x1429258d4
14001a952:     	testb	%al, %al
14001a954:     	je	0x14001a802 <.text+0x19802>
14001a95a:     	jmp	0x14001a80a <.text+0x1980a>
14001a95f:     	movq	0x7e0(%rbp), %r14
14001a966:     	movq	0x7d0(%rbp), %r15
14001a96d:     	jmp	0x14001b19c <.text+0x1a19c>
14001a972:     	cmpq	$0x0, 0x430(%rbp)
14001a97a:     	movq	0x698(%rbp), %rcx
14001a981:     	je	0x14001ac43 <.text+0x19c43>
14001a987:     	xorl	%eax, %eax
14001a989:     	cmpq	0x580(%rbp), %rcx
14001a990:     	jne	0x14001ac5b <.text+0x19c5b>
14001a996:     	cmpb	$0x1, 0x108(%r14)
14001a99e:     	sete	%cl
14001a9a1:     	jmp	0x14001ac5d <.text+0x19c5d>
14001a9a6:     	movq	0x7b0(%rbp), %rax
14001a9ad:     	movq	0x98(%rax), %rcx
14001a9b4:     	movq	0xa0(%rax), %rdx
14001a9bb:     	callq	0x140106840 <.text+0x105840>
14001a9c0:     	movb	$0x1, 0x816(%rbp)
14001a9c7:     	movb	$0x1, 0x815(%rbp)
14001a9ce:     	movq	0x340(%rbp), %rcx
14001a9d5:     	movq	%rax, %rdx
14001a9d8:     	leaq	0x6c0(%rbp), %r8
14001a9df:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14001a9e4:     	cmpq	$0x1, %rax
14001a9e8:     	movq	0x7e0(%rbp), %r10
14001a9ef:     	jne	0x14001aa14 <.text+0x19a14>
14001a9f1:     	cmpq	%rdi, %rdx
14001a9f4:     	jae	0x140020f67 <.text+0x1ff67>
14001a9fa:     	movq	0x7b0(%rbp), %rax
14001aa01:     	movq	0x68(%rax), %rax
14001aa05:     	leaq	(%rdx,%rdx,4), %rcx
14001aa09:     	cmpb	$0x0, 0x10(%rax,%rcx,8)
14001aa0e:     	je	0x14001aad8 <.text+0x19ad8>
14001aa14:     	movl	0x128(%r10), %r9d
14001aa1b:     	movl	0x12c(%r10), %r8d
14001aa22:     	movl	%r8d, 0x6c0(%rbp)
14001aa29:     	movl	%r9d, 0x6c4(%rbp)
14001aa30:     	movq	0x7b0(%rbp), %rax
14001aa37:     	movq	0x70(%rax), %rdi
14001aa3b:     	testq	%rdi, %rdi
14001aa3e:     	je	0x14001ab5c <.text+0x19b5c>
14001aa44:     	cmpq	$0x1, %rdi
14001aa48:     	jne	0x14001aa6d <.text+0x19a6d>
14001aa4a:     	movq	0x7b0(%rbp), %rax
14001aa51:     	movq	0x68(%rax), %rax
14001aa55:     	cmpl	%r8d, 0x8(%rax)
14001aa59:     	jne	0x14001ab5c <.text+0x19b5c>
14001aa5f:     	cmpl	%r9d, 0xc(%rax)
14001aa63:     	jne	0x14001ab5c <.text+0x19b5c>
14001aa69:     	xorl	%edx, %edx
14001aa6b:     	jmp	0x14001aac9 <.text+0x19ac9>
14001aa6d:     	movq	0x7b0(%rbp), %rax
14001aa74:     	movq	0x98(%rax), %rcx
14001aa7b:     	movq	0xa0(%rax), %rdx
14001aa82:     	callq	0x140106840 <.text+0x105840>
14001aa87:     	movb	$0x1, 0x816(%rbp)
14001aa8e:     	movb	$0x1, 0x815(%rbp)
14001aa95:     	movq	0x340(%rbp), %rcx
14001aa9c:     	movq	%rax, %rdx
14001aa9f:     	leaq	0x6c0(%rbp), %r8
14001aaa6:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14001aaab:     	cmpq	$0x1, %rax
14001aaaf:     	jne	0x14001ab5c <.text+0x19b5c>
14001aab5:     	cmpq	%rdi, %rdx
14001aab8:     	jae	0x140020faa <.text+0x1ffaa>
14001aabe:     	movq	0x7b0(%rbp), %rax
14001aac5:     	movq	0x68(%rax), %rax
14001aac9:     	leaq	(%rdx,%rdx,4), %rcx
14001aacd:     	cmpb	$0x0, 0x10(%rax,%rcx,8)
14001aad2:     	jne	0x14001ab5c <.text+0x19b5c>
14001aad8:     	leaq	(%rax,%rcx,8), %rax
14001aadc:     	addq	$0x10, %rax
14001aae0:     	movq	0x938(%rbp), %rcx
14001aae7:     	movq	(%rcx), %rcx
14001aaea:     	movdqu	0x570(%rbp), %xmm0
14001aaf2:     	movups	0x658(%rbp), %xmm1
14001aaf9:     	movdqu	0x688(%rbp), %xmm2
14001ab01:     	movss	0x4(%rax), %xmm3
14001ab06:     	movss	0x8(%rax), %xmm4
14001ab0b:     	movb	$0x1, 0x816(%rbp)
14001ab12:     	movb	$0x1, 0x815(%rbp)
14001ab19:     	movss	%xmm4, 0x68(%rsp)
14001ab1f:     	movss	%xmm3, 0x60(%rsp)
14001ab25:     	movb	%sil, 0x58(%rsp)
14001ab2a:     	andb	$0x1, %bl
14001ab2d:     	movb	%bl, 0x50(%rsp)
14001ab31:     	movdqu	%xmm2, 0x40(%rsp)
14001ab37:     	movups	%xmm1, 0x30(%rsp)
14001ab3c:     	movdqu	%xmm0, 0x20(%rsp)
14001ab42:     	movq	0x788(%rbp), %rdx
14001ab49:     	movl	0x6a8(%rbp), %r8d
14001ab50:     	leaq	0x140(%rbp), %r9
14001ab57:     	callq	0x1407febb0 <.text+0x7fdbb0>
14001ab5c:     	xorl	%edx, %edx
14001ab5e:     	movq	0x698(%rbp), %rax
14001ab65:     	cmpq	0x580(%rbp), %rax
14001ab6c:     	sete	%dl
14001ab6f:     	xorl	%eax, %eax
14001ab71:     	cmpq	$0x0, 0x430(%rbp)
14001ab79:     	sete	%al
14001ab7c:     	movq	0x950(%rbp), %rcx
14001ab83:     	movq	(%rcx), %rcx
14001ab86:     	movdqu	0x570(%rbp), %xmm0
14001ab8e:     	movups	0x658(%rbp), %xmm1
14001ab95:     	movdqu	0x688(%rbp), %xmm2
14001ab9d:     	movq	0x368(%rbp), %r8
14001aba4:     	movq	(%r8), %r8
14001aba7:     	movq	0x958(%rbp), %r9
14001abae:     	movq	(%r9), %r9
14001abb1:     	movq	0x938(%rbp), %r10
14001abb8:     	movq	(%r10), %r10
14001abbb:     	shll	$0x8, %edx
14001abbe:     	orl	%edx, %eax
14001abc0:     	movb	$0x1, 0x816(%rbp)
14001abc7:     	movb	$0x1, 0x815(%rbp)
14001abce:     	movq	%r10, 0x80(%rsp)
14001abd6:     	movq	%r9, 0x78(%rsp)
14001abdb:     	movq	%r8, 0x70(%rsp)
14001abe0:     	movdqu	%xmm2, 0x60(%rsp)
14001abe6:     	movups	%xmm1, 0x50(%rsp)
14001abeb:     	movdqu	%xmm0, 0x40(%rsp)
14001abf1:     	leaq	0x140(%rbp), %rdx
14001abf8:     	movq	%rdx, 0x38(%rsp)
14001abfd:     	movl	0x6a8(%rbp), %edx
14001ac03:     	movb	%dl, 0x30(%rsp)
14001ac07:     	movl	0x640(%rbp), %edx
14001ac0d:     	movl	%edx, 0x28(%rsp)
14001ac11:     	movq	0x788(%rbp), %rdx
14001ac18:     	movl	%edx, 0x20(%rsp)
14001ac1c:     	movw	%ax, 0x88(%rsp)
14001ac24:     	movq	0x7b0(%rbp), %rdx
14001ac2b:     	movq	0x670(%rbp), %r8
14001ac32:     	leaq	0x428(%rbp), %r9
14001ac39:     	callq	0x1407fcc80 <.text+0x7fbc80>
14001ac3e:     	jmp	0x14001af32 <.text+0x19f32>
14001ac43:     	cmpb	$0x1, 0xe8(%r14)
14001ac4b:     	sete	%al
14001ac4e:     	cmpq	0x580(%rbp), %rcx
14001ac55:     	je	0x14001a996 <.text+0x19996>
14001ac5b:     	xorl	%ecx, %ecx
14001ac5d:     	movzbl	%al, %r14d
14001ac61:     	movzbl	%cl, %r15d
14001ac65:     	movq	0x938(%rbp), %rax
14001ac6c:     	movq	(%rax), %rdi
14001ac6f:     	movb	0x807(%rbp), %bl
14001ac75:     	movb	$0x1, 0x816(%rbp)
14001ac7c:     	movb	$0x1, 0x815(%rbp)
14001ac83:     	leaq	0x290(%rbp), %rcx
14001ac8a:     	callq	0x1400284b0 <.text+0x274b0>
14001ac8f:     	movl	$0x0, 0x6c0(%rbp)
14001ac99:     	movl	$0x0, 0x6c8(%rbp)
14001aca3:     	movzwl	%r15w, %ecx
14001aca7:     	shll	$0x8, %ecx
14001acaa:     	movzwl	%r14w, %r8d
14001acae:     	orl	%ecx, %r8d
14001acb1:     	movb	$0x1, 0x816(%rbp)
14001acb8:     	movb	$0x1, 0x815(%rbp)
14001acbf:     	andb	$0x1, %dl
14001acc2:     	movb	%dl, 0x68(%rsp)
14001acc6:     	andb	$0x1, %al
14001acc8:     	movb	%al, 0x60(%rsp)
14001accc:     	movb	%bl, 0x58(%rsp)
14001acd0:     	movq	%r12, 0x80(%rsp)
14001acd8:     	movq	0x7f0(%rbp), %rax
14001acdf:     	movq	%rax, 0x78(%rsp)
14001ace4:     	leaq	0x6c0(%rbp), %rax
14001aceb:     	movq	%rax, 0x70(%rsp)
14001acf0:     	movb	%sil, 0x50(%rsp)
14001acf5:     	movl	0x6a8(%rbp), %eax
14001acfb:     	movb	%al, 0x48(%rsp)
14001acff:     	movw	%r8w, 0x40(%rsp)
14001ad05:     	movq	%rdi, 0x38(%rsp)
14001ad0a:     	movl	0x640(%rbp), %eax
14001ad10:     	movl	%eax, 0x30(%rsp)
14001ad14:     	movq	0x788(%rbp), %rax
14001ad1b:     	movl	%eax, 0x28(%rsp)
14001ad1f:     	leaq	0x428(%rbp), %rax
14001ad26:     	movq	%rax, 0x20(%rsp)
14001ad2b:     	leaq	0x140(%rbp), %rcx
14001ad32:     	leaq	0x568(%rbp), %rdx
14001ad39:     	leaq	0x650(%rbp), %r8
14001ad40:     	leaq	0x680(%rbp), %r9
14001ad47:     	callq	0x1407f98e0 <.text+0x7f88e0>
14001ad4c:     	movq	0x7e0(%rbp), %r14
14001ad53:     	cmpb	$0x0, 0xe8(%r14)
14001ad5b:     	je	0x14001ad69 <.text+0x19d69>
14001ad5d:     	movl	$0x0, 0x7a0(%rbp)
14001ad67:     	jmp	0x14001ad7a <.text+0x19d7a>
14001ad69:     	cmpb	$0x0, 0x108(%r14)
14001ad71:     	sete	%al
14001ad74:     	movl	%eax, 0x7a0(%rbp)
14001ad7a:     	movq	0x570(%rbp), %rax
14001ad81:     	movq	%rax, 0x808(%rbp)
14001ad88:     	movq	0x578(%rbp), %rbx
14001ad8f:     	movq	0x658(%rbp), %r15
14001ad96:     	movq	0x660(%rbp), %rdi
14001ad9d:     	movq	0x688(%rbp), %rsi
14001ada4:     	movq	0x690(%rbp), %r14
14001adab:     	movq	0x938(%rbp), %rax
14001adb2:     	movq	(%rax), %r13
14001adb5:     	movb	$0x1, 0x816(%rbp)
14001adbc:     	movb	$0x1, 0x815(%rbp)
14001adc3:     	movq	0x2d8(%rbp), %rcx
14001adca:     	callq	0x140959ef0 <.text+0x958ef0>
14001adcf:     	testq	%rax, %rax
14001add2:     	je	0x14001adff <.text+0x19dff>
14001add4:     	cmpq	$0x0, 0x578(%rbp)
14001addc:     	je	0x140020db4 <.text+0x1fdb4>
14001ade2:     	movq	0x570(%rbp), %rax
14001ade9:     	movss	0x290aadf(%rip), %xmm0  # 0x1429258d0
14001adf1:     	ucomiss	(%rax), %xmm0
14001adf4:     	setae	%al
14001adf7:     	movl	%eax, 0x6b0(%rbp)
14001adfd:     	jmp	0x14001ae09 <.text+0x19e09>
14001adff:     	movl	$0x0, 0x6b0(%rbp)
14001ae09:     	movb	$0x1, 0x816(%rbp)
14001ae10:     	movb	$0x1, 0x815(%rbp)
14001ae17:     	movq	0x118(%rbp), %rcx
14001ae1e:     	callq	0x140959ef0 <.text+0x958ef0>
14001ae23:     	testq	%rax, %rax
14001ae26:     	je	0x14001ae7c <.text+0x19e7c>
14001ae28:     	movq	0x578(%rbp), %rax
14001ae2f:     	testq	%rax, %rax
14001ae32:     	je	0x140020dd7 <.text+0x1fdd7>
14001ae38:     	movq	0x570(%rbp), %rcx
14001ae3f:     	leaq	(%rcx,%rax,4), %rax
14001ae43:     	addq	$-0x4, %rax
14001ae47:     	testq	%rax, %rax
14001ae4a:     	je	0x140020dd7 <.text+0x1fdd7>
14001ae50:     	movd	0x290aa7c(%rip), %xmm0  # 0x1429258d4
14001ae58:     	cmpb	$0x0, 0x7a0(%rbp)
14001ae5f:     	jne	0x14001ae69 <.text+0x19e69>
14001ae61:     	movd	0x290aa6f(%rip), %xmm0  # 0x1429258d8
14001ae69:     	movss	0x290aa5f(%rip), %xmm1  # 0x1429258d0
14001ae71:     	ucomiss	(%rax), %xmm1
14001ae74:     	jb	0x14001ae95 <.text+0x19e95>
14001ae76:     	movw	$0x100, %ax             # imm = 0x100
14001ae7a:     	jmp	0x14001ae97 <.text+0x19e97>
14001ae7c:     	movd	0x290aa50(%rip), %xmm0  # 0x1429258d4
14001ae84:     	cmpb	$0x0, 0x7a0(%rbp)
14001ae8b:     	jne	0x14001ae95 <.text+0x19e95>
14001ae8d:     	movd	0x290aa43(%rip), %xmm0  # 0x1429258d8
14001ae95:     	xorl	%eax, %eax
14001ae97:     	movzbl	0x6b0(%rbp), %ecx
14001ae9e:     	movzwl	%ax, %eax
14001aea1:     	orl	%ecx, %eax
14001aea3:     	movb	$0x1, 0x816(%rbp)
14001aeaa:     	movb	$0x1, 0x815(%rbp)
14001aeb1:     	movw	%ax, 0x50(%rsp)
14001aeb6:     	movq	%r13, 0x48(%rsp)
14001aebb:     	movq	%r14, 0x28(%rsp)
14001aec0:     	movq	%rsi, 0x20(%rsp)
14001aec5:     	movq	%r12, 0x78(%rsp)
14001aeca:     	movq	0x7f0(%rbp), %rax
14001aed1:     	movq	%rax, 0x70(%rsp)
14001aed6:     	movq	0x240(%rbp), %rax
14001aedd:     	movq	%rax, 0x68(%rsp)
14001aee2:     	movd	%xmm0, 0x60(%rsp)
14001aee8:     	movl	0x6a8(%rbp), %eax
14001aeee:     	movb	%al, 0x58(%rsp)
14001aef2:     	movq	0x788(%rbp), %rax
14001aef9:     	movl	%eax, 0x40(%rsp)
14001aefd:     	leaq	0x140(%rbp), %rax
14001af04:     	movq	%rax, 0x38(%rsp)
14001af09:     	leaq	0x428(%rbp), %rax
14001af10:     	movq	%rax, 0x30(%rsp)
14001af15:     	movb	$0x0, 0x80(%rsp)
14001af1d:     	movq	%r15, %r8
14001af20:     	movq	%rdi, %r9
14001af23:     	movq	0x808(%rbp), %rcx
14001af2a:     	movq	%rbx, %rdx
14001af2d:     	callq	0x1407da0b0 <.text+0x7d90b0>
14001af32:     	movzbl	0x806(%rbp), %ebx
14001af39:     	cmpb	$0x2, %bl
14001af3c:     	movq	0x7e0(%rbp), %r14
14001af43:     	movq	0x7d0(%rbp), %r15
14001af4a:     	jae	0x14001b19c <.text+0x1a19c>
14001af50:     	movl	0x128(%r14), %r8d
14001af57:     	movl	0x12c(%r14), %r9d
14001af5e:     	movl	%r8d, 0x6c0(%rbp)
14001af65:     	movl	%r9d, 0x6c4(%rbp)
14001af6c:     	movq	0x7b0(%rbp), %rax
14001af73:     	movq	0x70(%rax), %rdi
14001af77:     	testq	%rdi, %rdi
14001af7a:     	je	0x14001b195 <.text+0x1a195>
14001af80:     	cmpq	$0x1, %rdi
14001af84:     	jne	0x14001afa9 <.text+0x19fa9>
14001af86:     	movq	0x7b0(%rbp), %rax
14001af8d:     	movq	0x68(%rax), %rax
14001af91:     	cmpl	%r8d, 0x8(%rax)
14001af95:     	jne	0x14001b195 <.text+0x1a195>
14001af9b:     	cmpl	%r9d, 0xc(%rax)
14001af9f:     	jne	0x14001b195 <.text+0x1a195>
14001afa5:     	xorl	%edx, %edx
14001afa7:     	jmp	0x14001b013 <.text+0x1a013>
14001afa9:     	movq	0x7b0(%rbp), %rax
14001afb0:     	movq	0x98(%rax), %rcx
14001afb7:     	movq	0xa0(%rax), %rdx
14001afbe:     	callq	0x140106840 <.text+0x105840>
14001afc3:     	movb	$0x1, 0x816(%rbp)
14001afca:     	movb	$0x1, 0x815(%rbp)
14001afd1:     	movq	0x340(%rbp), %rcx
14001afd8:     	movq	%rax, %rdx
14001afdb:     	leaq	0x6c0(%rbp), %r8
14001afe2:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14001afe7:     	cmpq	$0x1, %rax
14001afeb:     	movq	0x7e0(%rbp), %r14
14001aff2:     	movq	0x7d0(%rbp), %r15
14001aff9:     	jne	0x14001b195 <.text+0x1a195>
14001afff:     	cmpq	%rdi, %rdx
14001b002:     	jae	0x140020eb7 <.text+0x1feb7>
14001b008:     	movq	0x7b0(%rbp), %rax
14001b00f:     	movq	0x68(%rax), %rax
14001b013:     	leaq	(%rdx,%rdx,4), %rcx
14001b017:     	cmpb	$0x0, 0x10(%rax,%rcx,8)
14001b01c:     	jne	0x14001b195 <.text+0x1a195>
14001b022:     	movzbl	0x12(%rax,%rcx,8), %edx
14001b027:     	orb	0x11(%rax,%rcx,8), %dl
14001b02b:     	testb	$0x1, %dl
14001b02e:     	je	0x14001b195 <.text+0x1a195>
14001b034:     	movl	0x128(%r14), %ecx
14001b03b:     	movl	0x12c(%r14), %edx
14001b042:     	movb	$0x1, 0x816(%rbp)
14001b049:     	movb	$0x1, 0x815(%rbp)
14001b050:     	callq	0x140959e70 <.text+0x958e70>
14001b055:     	movq	%rax, %rdi
14001b058:     	movq	%rax, 0x6c0(%rbp)
14001b05f:     	movq	0x7b0(%rbp), %rax
14001b066:     	cmpq	$0x0, 0x1b0(%rax)
14001b06e:     	movq	0x7d0(%rbp), %r15
14001b075:     	je	0x14001b18e <.text+0x1a18e>
14001b07b:     	movq	0xc0(%rbp), %rcx
14001b082:     	leaq	0x6c0(%rbp), %rdx
14001b089:     	callq	0x1405b0d70 <.text+0x5afd70>
14001b08e:     	movq	0x7b0(%rbp), %rdx
14001b095:     	movq	0x198(%rdx), %rcx
14001b09c:     	movq	0x1a0(%rdx), %rdx
14001b0a3:     	movq	%rdx, %r8
14001b0a6:     	andq	%rax, %r8
14001b0a9:     	shrq	$0x39, %rax
14001b0ad:     	movd	%eax, %xmm0
14001b0b1:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14001b0b5:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14001b0ba:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14001b0bf:     	leaq	-0x20(%rcx), %rax
14001b0c3:     	xorl	%r9d, %r9d
14001b0c6:     	movdqu	(%rcx,%r8), %xmm1
14001b0cc:     	movdqa	%xmm1, %xmm2
14001b0d0:     	pcmpeqb	%xmm0, %xmm2
14001b0d4:     	pmovmskb	%xmm2, %r10d
14001b0d9:     	testl	%r10d, %r10d
14001b0dc:     	je	0x14001b108 <.text+0x1a108>
14001b0de:     	tzcntl	%r10d, %r11d
14001b0e3:     	addq	%r8, %r11
14001b0e6:     	andq	%rdx, %r11
14001b0e9:     	movq	%r11, %rsi
14001b0ec:     	shlq	$0x5, %rsi
14001b0f0:     	movq	%rax, %rbx
14001b0f3:     	subq	%rsi, %rbx
14001b0f6:     	cmpq	(%rbx), %rdi
14001b0f9:     	je	0x14001b12a <.text+0x1a12a>
14001b0fb:     	leal	-0x1(%r10), %r11d
14001b0ff:     	andw	%r10w, %r11w
14001b103:     	movl	%r11d, %r10d
14001b106:     	jne	0x14001b0de <.text+0x1a0de>
14001b108:     	pcmpeqb	0x290a880(%rip), %xmm1  # 0x142925990
14001b110:     	pmovmskb	%xmm1, %r10d
14001b115:     	testl	%r10d, %r10d
14001b118:     	jne	0x14001b18e <.text+0x1a18e>
14001b11a:     	addq	%r9, %r8
14001b11d:     	addq	$0x10, %r8
14001b121:     	addq	$0x10, %r9
14001b125:     	andq	%rdx, %r8
14001b128:     	jmp	0x14001b0c6 <.text+0x1a0c6>
14001b12a:     	negq	%r11
14001b12d:     	shlq	$0x5, %r11
14001b131:     	movq	-0x8(%rcx,%r11), %rax
14001b136:     	cmpq	$0x2, %rax
14001b13a:     	jb	0x14001b148 <.text+0x1a148>
14001b13c:     	movq	-0x18(%rcx,%r11), %rdx
14001b141:     	movq	-0x10(%rcx,%r11), %rax
14001b146:     	jmp	0x14001b150 <.text+0x1a150>
14001b148:     	leaq	(%rcx,%r11), %rdx
14001b14c:     	addq	$-0x18, %rdx
14001b150:     	shlq	$0x2, %rax
14001b154:     	xorl	%ecx, %ecx
14001b156:     	nopw	%cs:(%rax,%rax)
14001b160:     	cmpq	%rcx, %rax
14001b163:     	je	0x14001b18e <.text+0x1a18e>
14001b165:     	cmpb	$0x3, (%rdx,%rcx)
14001b169:     	leaq	0x4(%rcx), %rcx
14001b16d:     	jne	0x14001b160 <.text+0x1a160>
14001b16f:     	movb	$0x1, %sil
14001b172:     	movq	0x7e0(%rbp), %r14
14001b179:     	testq	%r15, %r15
14001b17c:     	leaq	0x590(%rbp), %r12
14001b183:     	jne	0x14001b6fa <.text+0x1a6fa>
14001b189:     	jmp	0x14001b713 <.text+0x1a713>
14001b18e:     	movq	0x7e0(%rbp), %r14
14001b195:     	movzbl	0x806(%rbp), %ebx
14001b19c:     	cmpb	$0x4, %bl
14001b19f:     	jne	0x14001b674 <.text+0x1a674>
14001b1a5:     	movq	0x578(%rbp), %rcx
14001b1ac:     	testq	%rcx, %rcx
14001b1af:     	je	0x140020bbe <.text+0x1fbbe>
14001b1b5:     	movq	0x570(%rbp), %rax
14001b1bc:     	leaq	(%rax,%rcx,4), %rcx
14001b1c0:     	addq	$-0x4, %rcx
14001b1c4:     	je	0x140020be1 <.text+0x1fbe1>
14001b1ca:     	movss	(%rax), %xmm0
14001b1ce:     	movss	(%rcx), %xmm1
14001b1d2:     	movaps	%xmm0, %xmm2
14001b1d5:     	cmpunordss	%xmm0, %xmm2
14001b1da:     	movaps	%xmm2, %xmm3
14001b1dd:     	andps	%xmm1, %xmm3
14001b1e0:     	maxss	%xmm0, %xmm1
14001b1e4:     	andnps	%xmm1, %xmm2
14001b1e7:     	orps	%xmm3, %xmm2
14001b1ea:     	ucomiss	0x290a6df(%rip), %xmm2  # 0x1429258d0
14001b1f1:     	setbe	%al
14001b1f4:     	orb	0x530(%rbp), %al
14001b1fa:     	movb	$0x1, %sil
14001b1fd:     	jne	0x14001b6ee <.text+0x1a6ee>
14001b203:     	movq	$0x0, 0x460(%rbp)
14001b20e:     	movq	$0x1, 0x468(%rbp)
14001b219:     	movq	0x690(%rbp), %xmm0
14001b221:     	pshufd	$0x44, %xmm0, %xmm0     # xmm0 = xmm0[0,1,0,1]
14001b226:     	paddq	0x290a6e2(%rip), %xmm0  # 0x142925910
14001b22e:     	movdqu	%xmm0, 0x470(%rbp)
14001b236:     	xorl	%r14d, %r14d
14001b239:     	xorl	%esi, %esi
14001b23b:     	nopl	(%rax,%rax)
14001b240:     	cmpq	$0x20, %r14
14001b244:     	je	0x14001b795 <.text+0x1a795>
14001b24a:     	movq	0x468(%rbp,%r14), %r8
14001b252:     	cmpq	$-0x1, %r8
14001b256:     	je	0x14002054e <.text+0x1f54e>
14001b25c:     	movq	0x460(%rbp,%r14), %rcx
14001b264:     	leaq	0x1(%r8), %r9
14001b268:     	movq	%r9, %rdi
14001b26b:     	movq	%rcx, 0x808(%rbp)
14001b272:     	subq	%rcx, %rdi
14001b275:     	jb	0x14002056d <.text+0x1f56d>
14001b27b:     	movq	0x690(%rbp), %rdx
14001b282:     	cmpq	%rdx, %r8
14001b285:     	jae	0x140020596 <.text+0x1f596>
14001b28b:     	movq	%r8, 0x7a0(%rbp)
14001b292:     	movq	%r9, 0x6b0(%rbp)
14001b299:     	movq	%rdi, %rax
14001b29c:     	movl	$0xc, %ecx
14001b2a1:     	mulq	%rcx
14001b2a4:     	jo	0x140020ac8 <.text+0x1fac8>
14001b2aa:     	movq	%rsi, %r13
14001b2ad:     	movl	$0x0, %esi
14001b2b2:     	movabsq	$0x7ffffffffffffffc, %rcx # imm = 0x7FFFFFFFFFFFFFFC
14001b2bc:     	cmpq	%rcx, %rax
14001b2bf:     	movq	0x100(%rbp), %rbx
14001b2c6:     	ja	0x140020ad1 <.text+0x1fad1>
14001b2cc:     	movq	0x688(%rbp), %r15
14001b2d3:     	testq	%rax, %rax
14001b2d6:     	je	0x14001b310 <.text+0x1a310>
14001b2d8:     	movzbl	0x361992a(%rip), %ecx   # 0x143634c09
14001b2df:     	movl	$0x4, %esi
14001b2e4:     	movl	$0x4, %edx
14001b2e9:     	movq	%rax, %rcx
14001b2ec:     	movq	%rax, %rbx
14001b2ef:     	callq	0x140613c10 <.text+0x612c10>
14001b2f4:     	testq	%rax, %rax
14001b2f7:     	je	0x140020ad1 <.text+0x1fad1>
14001b2fd:     	movq	%rdi, %rsi
14001b300:     	movq	%rax, %rbx
14001b303:     	jmp	0x14001b317 <.text+0x1a317>
14001b305:     	nopw	%cs:(%rax,%rax)
14001b310:     	movl	$0x4, %ebx
14001b315:     	xorl	%esi, %esi
14001b317:     	movq	0x808(%rbp), %rax
14001b31e:     	leaq	(%rax,%rax,2), %rax
14001b322:     	leaq	(%r15,%rax,4), %rdx
14001b326:     	leaq	(,%rdi,4), %r15
14001b32e:     	leaq	(%r15,%r15,2), %r8
14001b332:     	movq	%rbx, %rcx
14001b335:     	callq	0x142923c00 <.text+0x2922c00>
14001b33a:     	movq	%rsi, 0x6c0(%rbp)
14001b341:     	movq	%rbx, 0x100(%rbp)
14001b348:     	movq	%rbx, 0x6c8(%rbp)
14001b34f:     	movq	%rdi, 0x6d0(%rbp)
14001b356:     	movb	$0x1, 0x816(%rbp)
14001b35d:     	movb	$0x1, 0x815(%rbp)
14001b364:     	leaq	0x18(%rbp), %rcx
14001b368:     	leaq	0x6c0(%rbp), %rdx
14001b36f:     	movl	$0x1, %r8d
14001b375:     	callq	0x140153640 <.text+0x152640>
14001b37a:     	addq	$0x10, %r14
14001b37e:     	leaq	0x1(%r13), %rsi
14001b382:     	xorl	%eax, %eax
14001b384:     	cmpq	0x18(%rbp), %rax
14001b388:     	jo	0x14001b240 <.text+0x1a240>
14001b38e:     	movq	0x330(%rbp), %rax
14001b395:     	movl	0x534(%rbp), %ecx
14001b39b:     	movl	%ecx, (%rax)
14001b39d:     	movq	0x48(%rbp), %rax
14001b3a1:     	movq	%rax, 0x5c0(%rbp)
14001b3a8:     	movups	0x18(%rbp), %xmm0
14001b3ac:     	movups	0x28(%rbp), %xmm1
14001b3b0:     	movdqu	0x38(%rbp), %xmm2
14001b3b5:     	movdqa	%xmm2, 0x5b0(%rbp)
14001b3bd:     	movaps	%xmm1, 0x5a0(%rbp)
14001b3c4:     	movaps	%xmm0, 0x590(%rbp)
14001b3cb:     	movq	0x578(%rbp), %rdx
14001b3d2:     	cmpq	%rdx, 0x7a0(%rbp)
14001b3d9:     	jae	0x140020796 <.text+0x1f796>
14001b3df:     	movq	%r13, %rbx
14001b3e2:     	movq	%r14, 0x558(%rbp)
14001b3e9:     	movq	0x808(%rbp), %rax
14001b3f0:     	leaq	(,%rax,4), %r14
14001b3f8:     	addq	0x570(%rbp), %r14
14001b3ff:     	testq	%rdi, %rdi
14001b402:     	je	0x14001b42a <.text+0x1a42a>
14001b404:     	movzbl	0x36197fe(%rip), %eax   # 0x143634c09
14001b40b:     	movl	$0x4, %edx
14001b410:     	movq	%r15, %rcx
14001b413:     	callq	0x140613c10 <.text+0x612c10>
14001b418:     	testq	%rax, %rax
14001b41b:     	je	0x140020e9e <.text+0x1fe9e>
14001b421:     	movq	%rdi, 0x7d8(%rbp)
14001b428:     	jmp	0x14001b43a <.text+0x1a43a>
14001b42a:     	movl	$0x4, %eax
14001b42f:     	movq	$0x0, 0x7d8(%rbp)
14001b43a:     	movq	%rax, 0x138(%rbp)
14001b441:     	movq	%rax, %rcx
14001b444:     	movq	%r14, %rdx
14001b447:     	movq	%r15, %r8
14001b44a:     	callq	0x142923c00 <.text+0x2922c00>
14001b44f:     	movq	0x660(%rbp), %rdx
14001b456:     	cmpq	%rdx, 0x7a0(%rbp)
14001b45d:     	jae	0x1400207ae <.text+0x1f7ae>
14001b463:     	movq	%rdi, %rax
14001b466:     	shrq	$0x3c, %rax
14001b46a:     	jne	0x140020cff <.text+0x1fcff>
14001b470:     	movq	0x808(%rbp), %rax
14001b477:     	leaq	(,%rax,8), %r13
14001b47f:     	addq	0x658(%rbp), %r13
14001b486:     	leaq	(,%rdi,8), %r14
14001b48e:     	testq	%rdi, %rdi
14001b491:     	je	0x14001b4b9 <.text+0x1a4b9>
14001b493:     	movzbl	0x361976f(%rip), %eax   # 0x143634c09
14001b49a:     	movl	$0x4, %edx
14001b49f:     	movq	%r14, %rcx
14001b4a2:     	callq	0x140613c10 <.text+0x612c10>
14001b4a7:     	testq	%rax, %rax
14001b4aa:     	je	0x140020e61 <.text+0x1fe61>
14001b4b0:     	movq	%rdi, 0x790(%rbp)
14001b4b7:     	jmp	0x14001b4c9 <.text+0x1a4c9>
14001b4b9:     	movl	$0x4, %eax
14001b4be:     	movq	$0x0, 0x790(%rbp)
14001b4c9:     	movq	%rax, 0x2f8(%rbp)
14001b4d0:     	movq	%rax, %rcx
14001b4d3:     	movq	%r13, %rdx
14001b4d6:     	movq	%r14, %r8
14001b4d9:     	callq	0x142923c00 <.text+0x2922c00>
14001b4de:     	cmpq	%r12, 0x7a0(%rbp)
14001b4e5:     	jae	0x1400207c6 <.text+0x1f7c6>
14001b4eb:     	testq	%rdi, %rdi
14001b4ee:     	je	0x14001b515 <.text+0x1a515>
14001b4f0:     	movzbl	0x3619712(%rip), %eax   # 0x143634c09
14001b4f7:     	movl	$0x4, %edx
14001b4fc:     	movq	%r15, %rcx
14001b4ff:     	callq	0x140613c10 <.text+0x612c10>
14001b504:     	testq	%rax, %rax
14001b507:     	je	0x140020e85 <.text+0x1fe85>
14001b50d:     	movq	%rax, %r13
14001b510:     	movq	%rdi, %r14
14001b513:     	jmp	0x14001b51e <.text+0x1a51e>
14001b515:     	movl	$0x4, %r13d
14001b51b:     	xorl	%r14d, %r14d
14001b51e:     	movq	0x7f0(%rbp), %rax
14001b525:     	movq	0x808(%rbp), %rcx
14001b52c:     	leaq	(%rax,%rcx,4), %rdx
14001b530:     	movq	%r13, %rcx
14001b533:     	movq	%r15, %r8
14001b536:     	callq	0x142923c00 <.text+0x2922c00>
14001b53b:     	cmpq	$0x1, %rbx
14001b53f:     	movb	$0x2, %al
14001b541:     	sbbb	$0x0, %al
14001b543:     	movaps	0x590(%rbp), %xmm0
14001b54a:     	movaps	0x5a0(%rbp), %xmm1
14001b551:     	movdqa	0x5b0(%rbp), %xmm2
14001b559:     	movaps	%xmm0, 0x6c0(%rbp)
14001b560:     	movaps	%xmm1, 0x6d0(%rbp)
14001b567:     	movdqa	%xmm2, 0x6e0(%rbp)
14001b56f:     	movq	0x5c0(%rbp), %rcx
14001b576:     	movq	%rcx, 0x6f0(%rbp)
14001b57d:     	movq	0x790(%rbp), %rcx
14001b584:     	movq	%rcx, 0x6f8(%rbp)
14001b58b:     	movq	0x2f8(%rbp), %rcx
14001b592:     	movq	%rcx, 0x700(%rbp)
14001b599:     	movq	%rdi, 0x708(%rbp)
14001b5a0:     	movq	0x7d8(%rbp), %rcx
14001b5a7:     	movq	%rcx, 0x710(%rbp)
14001b5ae:     	movq	0x138(%rbp), %rcx
14001b5b5:     	movq	%rcx, 0x718(%rbp)
14001b5bc:     	movq	%rdi, 0x720(%rbp)
14001b5c3:     	movq	%r14, 0x728(%rbp)
14001b5ca:     	movq	%r13, 0x730(%rbp)
14001b5d1:     	movq	%rdi, 0x738(%rbp)
14001b5d8:     	movq	0x438(%rbp), %rcx
14001b5df:     	movq	%rcx, 0x740(%rbp)
14001b5e6:     	movw	$0x0, 0x754(%rbp)
14001b5ef:     	movq	0x788(%rbp), %rcx
14001b5f6:     	movl	%ecx, 0x750(%rbp)
14001b5fc:     	movq	0x670(%rbp), %rcx
14001b603:     	movq	(%rcx), %rcx
14001b606:     	movq	%rcx, 0x748(%rbp)
14001b60d:     	movb	%al, 0x75b(%rbp)
14001b613:     	movl	$0x0, 0x756(%rbp)
14001b61d:     	movb	$0x0, 0x75a(%rbp)
14001b624:     	movq	0x360(%rbp), %rbx
14001b62b:     	movq	0x10(%rbx), %rdi
14001b62f:     	cmpq	(%rbx), %rdi
14001b632:     	jne	0x14001b643 <.text+0x1a643>
14001b634:     	movq	%rbx, %rcx
14001b637:     	leaq	0x290b46a(%rip), %rdx   # 0x142926aa8
14001b63e:     	callq	0x140563480 <.text+0x562480>
14001b643:     	leaq	(%rdi,%rdi,4), %rcx
14001b647:     	shlq	$0x5, %rcx
14001b64b:     	addq	0x8(%rbx), %rcx
14001b64f:     	movl	$0xa0, %r8d
14001b655:     	leaq	0x6c0(%rbp), %rdx
14001b65c:     	callq	0x142923c10 <.text+0x2922c10>
14001b661:     	incq	%rdi
14001b664:     	movq	%rdi, 0x10(%rbx)
14001b668:     	movq	0x558(%rbp), %r14
14001b66f:     	jmp	0x14001b240 <.text+0x1a240>
14001b674:     	cmpb	$0x0, 0xe8(%r14)
14001b67c:     	leaq	0x460(%rbp), %rcx
14001b683:     	jne	0x14001b693 <.text+0x1a693>
14001b685:     	cmpb	$0x0, 0x108(%r14)
14001b68d:     	je	0x14001ba77 <.text+0x1aa77>
14001b693:     	movq	0x690(%rbp), %rax
14001b69a:     	movq	%rax, 0x6d0(%rbp)
14001b6a1:     	movaps	0x680(%rbp), %xmm0
14001b6a8:     	movaps	%xmm0, 0x6c0(%rbp)
14001b6af:     	movb	$0x0, 0x816(%rbp)
14001b6b6:     	movb	$0x1, 0x815(%rbp)
14001b6bd:     	leaq	0x6c0(%rbp), %rdx
14001b6c4:     	movl	$0x1, %r8d
14001b6ca:     	callq	0x140153640 <.text+0x152640>
14001b6cf:     	xorl	%eax, %eax
14001b6d1:     	cmpq	0x460(%rbp), %rax
14001b6d8:     	jno	0x14001b7bb <.text+0x1a7bb>
14001b6de:     	xorl	%esi, %esi
14001b6e0:     	movq	0x7e0(%rbp), %r14
14001b6e7:     	movq	0x7d0(%rbp), %r15
14001b6ee:     	testq	%r15, %r15
14001b6f1:     	leaq	0x590(%rbp), %r12
14001b6f8:     	je	0x14001b713 <.text+0x1a713>
14001b6fa:     	shlq	$0x2, %r15
14001b6fe:     	movl	$0x4, %r8d
14001b704:     	movq	0x7f0(%rbp), %rcx
14001b70b:     	movq	%r15, %rdx
14001b70e:     	callq	0x140613c20 <.text+0x612c20>
14001b713:     	testb	%sil, %sil
14001b716:     	movq	0x698(%rbp), %rdi
14001b71d:     	je	0x14001b745 <.text+0x1a745>
14001b71f:     	movq	0x680(%rbp), %rax
14001b726:     	testq	%rax, %rax
14001b729:     	je	0x14001b745 <.text+0x1a745>
14001b72b:     	movq	0x688(%rbp), %rcx
14001b732:     	shlq	$0x2, %rax
14001b736:     	leaq	(%rax,%rax,2), %rdx
14001b73a:     	movl	$0x4, %r8d
14001b740:     	callq	0x140613c20 <.text+0x612c20>
14001b745:     	movq	0x650(%rbp), %rdx
14001b74c:     	testq	%rdx, %rdx
14001b74f:     	je	0x14001b767 <.text+0x1a767>
14001b751:     	movq	0x658(%rbp), %rcx
14001b758:     	shlq	$0x3, %rdx
14001b75c:     	movl	$0x4, %r8d
14001b762:     	callq	0x140613c20 <.text+0x612c20>
14001b767:     	movq	0x568(%rbp), %rdx
14001b76e:     	testq	%rdx, %rdx
14001b771:     	je	0x14001b789 <.text+0x1a789>
14001b773:     	movq	0x570(%rbp), %rcx
14001b77a:     	shlq	$0x2, %rdx
14001b77e:     	movl	$0x4, %r8d
14001b784:     	callq	0x140613c20 <.text+0x612c20>
14001b789:     	movq	0x560(%rbp), %r8
14001b790:     	jmp	0x140018efc <.text+0x17efc>
14001b795:     	movq	0x7e0(%rbp), %r14
14001b79c:     	movq	0x7d0(%rbp), %r15
14001b7a3:     	movb	$0x1, %sil
14001b7a6:     	testq	%r15, %r15
14001b7a9:     	leaq	0x590(%rbp), %r12
14001b7b0:     	jne	0x14001b6fa <.text+0x1a6fa>
14001b7b6:     	jmp	0x14001b713 <.text+0x1a713>
14001b7bb:     	movq	0x490(%rbp), %rax
14001b7c2:     	movq	%rax, 0x5c0(%rbp)
14001b7c9:     	movups	0x460(%rbp), %xmm0
14001b7d0:     	movups	0x470(%rbp), %xmm1
14001b7d7:     	movdqu	0x480(%rbp), %xmm2
14001b7df:     	movdqa	%xmm2, 0x5b0(%rbp)
14001b7e7:     	movaps	%xmm1, 0x5a0(%rbp)
14001b7ee:     	movaps	%xmm0, 0x590(%rbp)
14001b7f5:     	movq	0x568(%rbp), %rax
14001b7fc:     	movq	%rax, 0x808(%rbp)
14001b803:     	movq	0x570(%rbp), %rcx
14001b80a:     	movq	0x578(%rbp), %rsi
14001b811:     	movq	0x650(%rbp), %rax
14001b818:     	movq	%rax, 0x7a0(%rbp)
14001b81f:     	movq	0x658(%rbp), %rdx
14001b826:     	movq	0x660(%rbp), %r14
14001b82d:     	movb	$0x1, %bl
14001b82f:     	movq	0x7e0(%rbp), %rax
14001b836:     	cmpb	$0x1, 0xe8(%rax)
14001b83d:     	movb	$0x1, %dil
14001b840:     	movq	%rcx, 0x6b0(%rbp)
14001b847:     	movq	%rdx, 0x7d8(%rbp)
14001b84e:     	je	0x14001b888 <.text+0x1a888>
14001b850:     	movl	0x128(%rax), %r8d
14001b857:     	movl	0x12c(%rax), %r9d
14001b85e:     	movq	0x950(%rbp), %rcx
14001b865:     	movq	0x2d0(%rbp), %rdx
14001b86c:     	callq	0x1400275b0 <.text+0x265b0>
14001b871:     	movl	%eax, %edi
14001b873:     	movq	0x7e0(%rbp), %rax
14001b87a:     	movq	0x6b0(%rbp), %rcx
14001b881:     	movq	0x7d8(%rbp), %rdx
14001b888:     	cmpb	$0x1, 0x108(%rax)
14001b88f:     	je	0x14001b8c9 <.text+0x1a8c9>
14001b891:     	movq	0x7e0(%rbp), %rax
14001b898:     	movl	0x128(%rax), %r9d
14001b89f:     	movl	0x12c(%rax), %r8d
14001b8a6:     	movq	0x950(%rbp), %rcx
14001b8ad:     	movq	0x2d0(%rbp), %rdx
14001b8b4:     	callq	0x1400275b0 <.text+0x265b0>
14001b8b9:     	movl	%eax, %ebx
14001b8bb:     	movq	0x6b0(%rbp), %rcx
14001b8c2:     	movq	0x7d8(%rbp), %rdx
14001b8c9:     	movq	0x5c0(%rbp), %rax
14001b8d0:     	movq	%rax, 0x6f0(%rbp)
14001b8d7:     	movaps	0x590(%rbp), %xmm0
14001b8de:     	movaps	0x5a0(%rbp), %xmm1
14001b8e5:     	movdqa	0x5b0(%rbp), %xmm2
14001b8ed:     	movdqa	%xmm2, 0x6e0(%rbp)
14001b8f5:     	movaps	%xmm1, 0x6d0(%rbp)
14001b8fc:     	movaps	%xmm0, 0x6c0(%rbp)
14001b903:     	movq	0x7a0(%rbp), %rax
14001b90a:     	movq	%rax, 0x6f8(%rbp)
14001b911:     	movq	%rdx, 0x700(%rbp)
14001b918:     	movq	%r14, 0x708(%rbp)
14001b91f:     	movq	0x808(%rbp), %rax
14001b926:     	movq	%rax, 0x710(%rbp)
14001b92d:     	movq	%rcx, 0x718(%rbp)
14001b934:     	movq	%rsi, 0x720(%rbp)
14001b93b:     	movq	0x7d0(%rbp), %rax
14001b942:     	movq	%rax, 0x728(%rbp)
14001b949:     	movq	0x7f0(%rbp), %rax
14001b950:     	movq	%rax, 0x730(%rbp)
14001b957:     	movq	%r12, 0x738(%rbp)
14001b95e:     	movq	0x438(%rbp), %rax
14001b965:     	movq	%rax, 0x740(%rbp)
14001b96c:     	movq	0x670(%rbp), %rax
14001b973:     	movq	(%rax), %rax
14001b976:     	movq	%rax, 0x748(%rbp)
14001b97d:     	movq	0x788(%rbp), %rax
14001b984:     	movl	%eax, 0x750(%rbp)
14001b98a:     	movb	%dil, 0x754(%rbp)
14001b991:     	movb	%bl, 0x755(%rbp)
14001b997:     	movw	$0x0, 0x756(%rbp)
14001b9a0:     	movzbl	0x678(%rbp), %eax
14001b9a7:     	movb	%al, 0x758(%rbp)
14001b9ad:     	movzbl	0x588(%rbp), %eax
14001b9b4:     	movb	%al, 0x759(%rbp)
14001b9ba:     	movw	$0x300, 0x75a(%rbp)     # imm = 0x300
14001b9c3:     	movq	0x430(%rbp), %rax
14001b9ca:     	movq	%rax, 0x760(%rbp)
14001b9d1:     	movq	0x698(%rbp), %rdi
14001b9d8:     	movq	%rdi, 0x768(%rbp)
14001b9df:     	movb	$0x0, 0x770(%rbp)
14001b9e6:     	movq	0x440(%rbp), %rax
14001b9ed:     	movq	0x458(%rbp), %rsi
14001b9f4:     	cmpq	%rax, %rsi
14001b9f7:     	leaq	0x590(%rbp), %r12
14001b9fe:     	movq	0x7e0(%rbp), %r14
14001ba05:     	jne	0x14001ba28 <.text+0x1aa28>
14001ba07:     	leaq	0x440(%rbp), %rcx
14001ba0e:     	leaq	0x290afeb(%rip), %rdx   # 0x142926a00
14001ba15:     	callq	0x1405114a0 <.text+0x5104a0>
14001ba1a:     	movq	0x440(%rbp), %rax
14001ba21:     	movq	0x458(%rbp), %rsi
14001ba28:     	movq	0x450(%rbp), %rcx
14001ba2f:     	addq	%rsi, %rcx
14001ba32:     	cmpq	%rax, %rcx
14001ba35:     	movl	$0x0, %edx
14001ba3a:     	cmovaeq	%rax, %rdx
14001ba3e:     	subq	%rdx, %rcx
14001ba41:     	imulq	$0xb8, %rcx, %rcx
14001ba48:     	addq	0x448(%rbp), %rcx
14001ba4f:     	movl	$0xb8, %r8d
14001ba55:     	leaq	0x6c0(%rbp), %rdx
14001ba5c:     	callq	0x142923c10 <.text+0x2922c10>
14001ba61:     	incq	%rsi
14001ba64:     	movq	%rsi, 0x458(%rbp)
14001ba6b:     	movq	0x560(%rbp), %r8
14001ba72:     	jmp	0x140018efc <.text+0x17efc>
14001ba77:     	movq	0x688(%rbp), %rsi
14001ba7e:     	movq	0x690(%rbp), %rax
14001ba85:     	testq	%rax, %rax
14001ba88:     	sete	%cl
14001ba8b:     	leaq	(%rax,%rax,2), %rax
14001ba8f:     	leaq	(%rsi,%rax,4), %rdi
14001ba93:     	cmpq	$0xc, %rdi
14001ba97:     	sete	%al
14001ba9a:     	orb	%cl, %al
14001ba9c:     	jne	0x14001bf40 <.text+0x1af40>
14001baa2:     	movq	0x660(%rbp), %rdx
14001baa9:     	cmpq	$0x1, %rdx
14001baad:     	jbe	0x140020efe <.text+0x1fefe>
14001bab3:     	movss	0x8(%rsi), %xmm6
14001bab8:     	movss	0x2909db0(%rip), %xmm0  # 0x142925870
14001bac0:     	mulss	%xmm0, %xmm6
14001bac4:     	movsd	(%rsi), %xmm8
14001bac9:     	movsd	-0xc(%rdi), %xmm7
14001bace:     	movss	-0x4(%rdi), %xmm9
14001bad4:     	mulss	%xmm0, %xmm9
14001bad9:     	movq	0x658(%rbp), %rax
14001bae0:     	movss	0x8(%rax), %xmm0
14001bae5:     	movss	0xc(%rax), %xmm1
14001baea:     	xorps	0x2909dff(%rip), %xmm1  # 0x1429258f0
14001baf1:     	movss	%xmm1, 0x590(%rbp)
14001baf9:     	movss	%xmm0, 0x594(%rbp)
14001bb01:     	movb	$0x1, 0x816(%rbp)
14001bb08:     	movb	$0x1, 0x815(%rbp)
14001bb0f:     	leaq	0x6c0(%rbp), %rcx
14001bb16:     	leaq	0x590(%rbp), %rdx
14001bb1d:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14001bb22:     	movq	0x578(%rbp), %rdx
14001bb29:     	cmpq	$0x1, %rdx
14001bb2d:     	jbe	0x140020f22 <.text+0x1ff22>
14001bb33:     	movq	0x570(%rbp), %rax
14001bb3a:     	movss	0x4(%rax), %xmm12
14001bb40:     	mulss	0x2909d27(%rip), %xmm12 # 0x142925870
14001bb49:     	addss	0x2909dae(%rip), %xmm12 # 0x142925900
14001bb52:     	movq	0x660(%rbp), %rdx
14001bb59:     	cmpq	$0x1, %rdx
14001bb5d:     	jbe	0x140020f46 <.text+0x1ff46>
14001bb63:     	movsd	0x6c0(%rbp), %xmm10
14001bb6c:     	movss	0x6c8(%rbp), %xmm11
14001bb75:     	mulss	%xmm12, %xmm11
14001bb7a:     	movq	0x658(%rbp), %rax
14001bb81:     	movss	0x8(%rax), %xmm0
14001bb86:     	movss	0xc(%rax), %xmm1
14001bb8b:     	xorps	0x2909d5e(%rip), %xmm1  # 0x1429258f0
14001bb92:     	movss	%xmm1, 0x590(%rbp)
14001bb9a:     	movss	%xmm0, 0x594(%rbp)
14001bba2:     	movb	$0x1, 0x816(%rbp)
14001bba9:     	movb	$0x1, 0x815(%rbp)
14001bbb0:     	leaq	0x6c0(%rbp), %rcx
14001bbb7:     	leaq	0x590(%rbp), %rdx
14001bbbe:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14001bbc3:     	movq	0x578(%rbp), %rdx
14001bbca:     	cmpq	$0x1, %rdx
14001bbce:     	jbe	0x140020f89 <.text+0x1ff89>
14001bbd4:     	movq	0x570(%rbp), %rax
14001bbdb:     	movss	0x4(%rax), %xmm14
14001bbe1:     	movsd	0x6c0(%rbp), %xmm13
14001bbea:     	movss	0x6c8(%rbp), %xmm15
14001bbf3:     	movzbl	0x361900f(%rip), %eax   # 0x143634c09
14001bbfa:     	movl	$0x24, %ecx
14001bbff:     	movl	$0x4, %edx
14001bc04:     	callq	0x140613c10 <.text+0x612c10>
14001bc09:     	testq	%rax, %rax
14001bc0c:     	je	0x140020edc <.text+0x1fedc>
14001bc12:     	movaps	0x2909cc7(%rip), %xmm0  # 0x1429258e0
14001bc19:     	mulps	%xmm0, %xmm8
14001bc1d:     	mulps	%xmm0, %xmm7
14001bc20:     	addps	%xmm8, %xmm7
14001bc24:     	addss	%xmm9, %xmm6
14001bc29:     	movsldup	%xmm12, %xmm0           # xmm0 = xmm12[0,0,2,2]
14001bc2e:     	mulps	%xmm0, %xmm10
14001bc32:     	addps	%xmm7, %xmm10
14001bc36:     	addss	%xmm6, %xmm11
14001bc3b:     	mulss	0x2909c2c(%rip), %xmm14 # 0x142925870
14001bc44:     	addss	0x2909cb3(%rip), %xmm14 # 0x142925900
14001bc4d:     	mulss	%xmm14, %xmm15
14001bc52:     	movlps	%xmm10, (%rax)
14001bc56:     	movss	%xmm11, 0x8(%rax)
14001bc5c:     	movlps	%xmm7, 0xc(%rax)
14001bc60:     	movss	%xmm6, 0x14(%rax)
14001bc65:     	subss	%xmm15, %xmm6
14001bc6a:     	movsldup	%xmm14, %xmm0           # xmm0 = xmm14[0,0,2,2]
14001bc6f:     	mulps	%xmm0, %xmm13
14001bc73:     	subps	%xmm13, %xmm7
14001bc77:     	movlps	%xmm7, 0x18(%rax)
14001bc7b:     	movss	%xmm6, 0x20(%rax)
14001bc80:     	movss	(%rsi), %xmm6
14001bc84:     	subss	-0xc(%rdi), %xmm6
14001bc89:     	movss	0x8(%rsi), %xmm7
14001bc8e:     	subss	-0x4(%rdi), %xmm7
14001bc93:     	movq	$0x3, 0x6c0(%rbp)
14001bc9e:     	movq	%rax, 0x6c8(%rbp)
14001bca5:     	movq	$0x3, 0x6d0(%rbp)
14001bcb0:     	movb	$0x1, 0x816(%rbp)
14001bcb7:     	movb	$0x1, 0x815(%rbp)
14001bcbe:     	leaq	0x460(%rbp), %rcx
14001bcc5:     	leaq	0x6c0(%rbp), %rdx
14001bccc:     	movl	$0x1, %r8d
14001bcd2:     	callq	0x140153640 <.text+0x152640>
14001bcd7:     	xorl	%eax, %eax
14001bcd9:     	cmpq	0x460(%rbp), %rax
14001bce0:     	jo	0x14001bf40 <.text+0x1af40>
14001bce6:     	movq	0x490(%rbp), %rax
14001bced:     	movq	%rax, 0x6f0(%rbp)
14001bcf4:     	movups	0x460(%rbp), %xmm0
14001bcfb:     	movups	0x470(%rbp), %xmm1
14001bd02:     	movups	0x480(%rbp), %xmm2
14001bd09:     	movaps	%xmm2, 0x6e0(%rbp)
14001bd10:     	movaps	%xmm1, 0x6d0(%rbp)
14001bd17:     	movaps	%xmm0, 0x6c0(%rbp)
14001bd1e:     	movl	$0x3, %r8d
14001bd24:     	leaq	0x508(%rbp), %rcx
14001bd2b:     	movss	0x2909bd5(%rip), %xmm1  # 0x142925908
14001bd33:     	leaq	0x290ac36(%rip), %r9    # 0x142926970
14001bd3a:     	callq	0x1402b6ad0 <.text+0x2b5ad0>
14001bd3f:     	testq	%r12, %r12
14001bd42:     	movss	0x2909bb9(%rip), %xmm8  # 0x142925904
14001bd4b:     	movzbl	0x588(%rbp), %ebx
14001bd52:     	movzbl	0x678(%rbp), %esi
14001bd59:     	je	0x140020fcc <.text+0x1ffcc>
14001bd5f:     	movq	0x7f0(%rbp), %rax
14001bd66:     	movss	(%rax), %xmm1
14001bd6a:     	movl	$0x3, %r8d
14001bd70:     	leaq	0x3e0(%rbp), %rcx
14001bd77:     	leaq	0x290ac22(%rip), %r9    # 0x1429269a0
14001bd7e:     	callq	0x1402b6ad0 <.text+0x2b5ad0>
14001bd83:     	movaps	%xmm7, %xmm1
14001bd86:     	xorps	0x2909b63(%rip), %xmm1  # 0x1429258f0
14001bd8d:     	mulss	%xmm7, %xmm7
14001bd91:     	movaps	%xmm6, %xmm0
14001bd94:     	mulss	%xmm6, %xmm0
14001bd98:     	addss	%xmm7, %xmm0
14001bd9c:     	sqrtss	%xmm0, %xmm0
14001bda0:     	movaps	%xmm8, %xmm2
14001bda4:     	divss	%xmm0, %xmm2
14001bda8:     	mulss	%xmm2, %xmm1
14001bdac:     	mulss	%xmm6, %xmm2
14001bdb0:     	leaq	0x290ac01(%rip), %rax   # 0x1429269b8
14001bdb7:     	movq	%rax, 0x20(%rsp)
14001bdbc:     	movl	$0x3, %r9d
14001bdc2:     	leaq	0x380(%rbp), %rcx
14001bdc9:     	callq	0x1402b6c60 <.text+0x2b5c60>
14001bdce:     	movq	0x6f0(%rbp), %rax
14001bdd5:     	movq	%rax, 0x5c0(%rbp)
14001bddc:     	movaps	0x6c0(%rbp), %xmm0
14001bde3:     	movaps	0x6d0(%rbp), %xmm1
14001bdea:     	movaps	0x6e0(%rbp), %xmm2
14001bdf1:     	movaps	%xmm2, 0x5b0(%rbp)
14001bdf8:     	movaps	%xmm1, 0x5a0(%rbp)
14001bdff:     	movaps	%xmm0, 0x590(%rbp)
14001be06:     	movq	0x390(%rbp), %rax
14001be0d:     	leaq	0x598(%rbp), %rcx
14001be14:     	movq	%rax, 0x40(%rcx)
14001be18:     	movups	0x380(%rbp), %xmm0
14001be1f:     	movups	%xmm0, 0x30(%rcx)
14001be23:     	movq	0x518(%rbp), %rax
14001be2a:     	movq	%rax, 0x58(%rcx)
14001be2e:     	movups	0x508(%rbp), %xmm0
14001be35:     	movups	%xmm0, 0x48(%rcx)
14001be39:     	movq	0x3f0(%rbp), %rax
14001be40:     	movq	%rax, 0x70(%rcx)
14001be44:     	movups	0x3e0(%rbp), %xmm0
14001be4b:     	movups	%xmm0, 0x60(%rcx)
14001be4f:     	movaps	0x590(%rbp), %xmm0
14001be56:     	movaps	0x5a0(%rbp), %xmm1
14001be5d:     	movdqa	0x5b0(%rbp), %xmm2
14001be65:     	movapd	0x5c0(%rbp), %xmm3
14001be6d:     	movaps	%xmm0, 0x6c0(%rbp)
14001be74:     	movaps	%xmm1, 0x6d0(%rbp)
14001be7b:     	movdqa	%xmm2, 0x6e0(%rbp)
14001be83:     	movapd	%xmm3, 0x6f0(%rbp)
14001be8b:     	movaps	0x5d0(%rbp), %xmm0
14001be92:     	movaps	%xmm0, 0x700(%rbp)
14001be99:     	movaps	0x5e0(%rbp), %xmm0
14001bea0:     	movaps	%xmm0, 0x710(%rbp)
14001bea7:     	movaps	0x5f0(%rbp), %xmm0
14001beae:     	movaps	%xmm0, 0x720(%rbp)
14001beb5:     	movdqa	0x600(%rbp), %xmm0
14001bebd:     	movdqa	%xmm0, 0x730(%rbp)
14001bec5:     	movq	0x438(%rbp), %rax
14001becc:     	movq	%rax, 0x740(%rbp)
14001bed3:     	movq	0x670(%rbp), %rax
14001beda:     	movq	(%rax), %rax
14001bedd:     	movq	%rax, 0x748(%rbp)
14001bee4:     	movq	0x788(%rbp), %rax
14001beeb:     	movl	%eax, 0x750(%rbp)
14001bef1:     	movl	$0x0, 0x754(%rbp)
14001befb:     	movb	%sil, 0x758(%rbp)
14001bf02:     	movb	%bl, 0x759(%rbp)
14001bf08:     	movw	$0x301, 0x75a(%rbp)     # imm = 0x301
14001bf11:     	movb	$0x2, 0x770(%rbp)
14001bf18:     	movb	$0x1, 0x816(%rbp)
14001bf1f:     	movb	$0x1, 0x815(%rbp)
14001bf26:     	leaq	0x440(%rbp), %rcx
14001bf2d:     	leaq	0x6c0(%rbp), %rdx
14001bf34:     	leaq	0x290aa95(%rip), %r8    # 0x1429269d0
14001bf3b:     	callq	0x140511a10 <.text+0x510a10>
14001bf40:     	movq	0x690(%rbp), %rax
14001bf47:     	movq	%rax, 0x6d0(%rbp)
14001bf4e:     	movaps	0x680(%rbp), %xmm0
14001bf55:     	movaps	%xmm0, 0x6c0(%rbp)
14001bf5c:     	movb	$0x0, 0x816(%rbp)
14001bf63:     	movb	$0x1, 0x815(%rbp)
14001bf6a:     	leaq	0x460(%rbp), %rcx
14001bf71:     	leaq	0x6c0(%rbp), %rdx
14001bf78:     	movl	$0x1, %r8d
14001bf7e:     	callq	0x140153640 <.text+0x152640>
14001bf83:     	xorl	%eax, %eax
14001bf85:     	cmpq	0x460(%rbp), %rax
14001bf8c:     	jo	0x14001b6de <.text+0x1a6de>
14001bf92:     	movq	0x490(%rbp), %rax
14001bf99:     	movq	%rax, 0x5c0(%rbp)
14001bfa0:     	movups	0x460(%rbp), %xmm0
14001bfa7:     	movups	0x470(%rbp), %xmm1
14001bfae:     	movups	0x480(%rbp), %xmm2
14001bfb5:     	movaps	%xmm2, 0x5b0(%rbp)
14001bfbc:     	movaps	%xmm1, 0x5a0(%rbp)
14001bfc3:     	movaps	%xmm0, 0x590(%rbp)
14001bfca:     	movq	0x578(%rbp), %rax
14001bfd1:     	leaq	0x598(%rbp), %rcx
14001bfd8:     	movq	%rax, 0x58(%rcx)
14001bfdc:     	movups	0x568(%rbp), %xmm0
14001bfe3:     	movups	%xmm0, 0x48(%rcx)
14001bfe7:     	movq	0x660(%rbp), %rax
14001bfee:     	movq	%rax, 0x40(%rcx)
14001bff2:     	movaps	0x650(%rbp), %xmm0
14001bff9:     	movups	%xmm0, 0x30(%rcx)
14001bffd:     	movaps	0x590(%rbp), %xmm0
14001c004:     	movaps	0x5a0(%rbp), %xmm1
14001c00b:     	movdqa	0x5b0(%rbp), %xmm2
14001c013:     	movq	0x5c0(%rbp), %rax
14001c01a:     	movq	0x5c8(%rbp), %rcx
14001c021:     	movaps	%xmm0, 0x6c0(%rbp)
14001c028:     	movaps	%xmm1, 0x6d0(%rbp)
14001c02f:     	movdqa	%xmm2, 0x6e0(%rbp)
14001c037:     	movq	%rax, 0x6f0(%rbp)
14001c03e:     	movq	%rcx, 0x6f8(%rbp)
14001c045:     	movaps	0x5d0(%rbp), %xmm0
14001c04c:     	movaps	%xmm0, 0x700(%rbp)
14001c053:     	movaps	0x5e0(%rbp), %xmm0
14001c05a:     	movaps	%xmm0, 0x710(%rbp)
14001c061:     	movq	0x5f0(%rbp), %rax
14001c068:     	movq	%rax, 0x720(%rbp)
14001c06f:     	movq	0x7d0(%rbp), %rax
14001c076:     	movq	%rax, 0x728(%rbp)
14001c07d:     	movq	0x7f0(%rbp), %rax
14001c084:     	movq	%rax, 0x730(%rbp)
14001c08b:     	movq	%r12, 0x738(%rbp)
14001c092:     	movq	0x438(%rbp), %rax
14001c099:     	movq	%rax, 0x740(%rbp)
14001c0a0:     	movq	0x670(%rbp), %rax
14001c0a7:     	movq	(%rax), %rax
14001c0aa:     	movq	%rax, 0x748(%rbp)
14001c0b1:     	movq	0x788(%rbp), %rax
14001c0b8:     	movl	%eax, 0x750(%rbp)
14001c0be:     	movl	$0x0, 0x754(%rbp)
14001c0c8:     	movzbl	0x678(%rbp), %eax
14001c0cf:     	movb	%al, 0x758(%rbp)
14001c0d5:     	movzbl	0x588(%rbp), %eax
14001c0dc:     	movb	%al, 0x759(%rbp)
14001c0e2:     	movw	$0x301, 0x75a(%rbp)     # imm = 0x301
14001c0eb:     	movb	$0x2, 0x770(%rbp)
14001c0f2:     	movb	$0x0, 0x816(%rbp)
14001c0f9:     	movb	$0x0, 0x815(%rbp)
14001c100:     	leaq	0x440(%rbp), %rcx
14001c107:     	leaq	0x6c0(%rbp), %rdx
14001c10e:     	leaq	0x290a8d3(%rip), %r8    # 0x1429269e8
14001c115:     	callq	0x140511a10 <.text+0x510a10>
14001c11a:     	leaq	0x590(%rbp), %r12
14001c121:     	movq	0x7e0(%rbp), %r14
14001c128:     	movq	0x698(%rbp), %rdi
14001c12f:     	movq	0x560(%rbp), %r8
14001c136:     	jmp	0x140018efc <.text+0x17efc>
14001c13b:     	cmpq	%rcx, %r14
14001c13e:     	je	0x14001c19f <.text+0x1b19f>
14001c140:     	movss	(%r15,%r8,4), %xmm0
14001c146:     	movaps	%xmm8, %xmm1
14001c14a:     	subss	%xmm0, %xmm1
14001c14e:     	movsd	0x550(%rbp), %xmm2
14001c156:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001c159:     	mulps	%xmm2, %xmm1
14001c15c:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c160:     	addss	%xmm1, %xmm0
14001c164:     	movss	%xmm0, (%rax,%r8,4)
14001c16a:     	movss	0x4(%r15,%r8,4), %xmm0
14001c171:     	movaps	%xmm8, %xmm1
14001c175:     	subss	%xmm0, %xmm1
14001c179:     	movsd	0x550(%rbp), %xmm2
14001c181:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001c184:     	mulps	%xmm2, %xmm1
14001c187:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c18b:     	addss	%xmm1, %xmm0
14001c18f:     	movss	%xmm0, 0x4(%rax,%r8,4)
14001c196:     	leaq	0x2(%r8), %rcx
14001c19a:     	movq	%rcx, %r8
14001c19d:     	jmp	0x14001c13b <.text+0x1b13b>
14001c19f:     	movq	%rax, 0x7f0(%rbp)
14001c1a6:     	movq	0x5a0(%rbp), %r15
14001c1ad:     	movq	0x5b8(%rbp), %rax
14001c1b4:     	cmpq	%r15, %rax
14001c1b7:     	cmovbq	%rax, %r15
14001c1bb:     	leaq	(,%r15,4), %rdi
14001c1c3:     	movq	%r15, %rax
14001c1c6:     	shrq	$0x3e, %rax
14001c1ca:     	jne	0x140020d6d <.text+0x1fd6d>
14001c1d0:     	movl	$0x0, %esi
14001c1d5:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14001c1df:     	cmpq	%rax, %rdi
14001c1e2:     	ja	0x140020d6f <.text+0x1fd6f>
14001c1e8:     	movq	0x5b0(%rbp), %r12
14001c1ef:     	testq	%rdi, %rdi
14001c1f2:     	je	0x14001c223 <.text+0x1b223>
14001c1f4:     	movzbl	0x3618a0e(%rip), %eax   # 0x143634c09
14001c1fb:     	movl	$0x4, %esi
14001c200:     	movl	$0x4, %edx
14001c205:     	movq	%rdi, %rcx
14001c208:     	callq	0x140613c10 <.text+0x612c10>
14001c20d:     	testq	%rax, %rax
14001c210:     	je	0x140020d6f <.text+0x1fd6f>
14001c216:     	movq	%r15, %rcx
14001c219:     	testq	%r15, %r15
14001c21c:     	jne	0x14001c233 <.text+0x1b233>
14001c21e:     	jmp	0x14001c350 <.text+0x1b350>
14001c223:     	movl	$0x4, %eax
14001c228:     	xorl	%ecx, %ecx
14001c22a:     	testq	%r15, %r15
14001c22d:     	je	0x14001c350 <.text+0x1b350>
14001c233:     	cmpq	$0x8, %r15
14001c237:     	jb	0x14001c2c9 <.text+0x1b2c9>
14001c23d:     	movq	%rax, %r8
14001c240:     	subq	%r12, %r8
14001c243:     	movl	$0x0, %edx
14001c248:     	cmpq	$0x20, %r8
14001c24c:     	jb	0x14001c2cb <.text+0x1b2cb>
14001c24e:     	movq	%r15, %rdx
14001c251:     	movabsq	$0x3ffffffffffffff8, %r8 # imm = 0x3FFFFFFFFFFFFFF8
14001c25b:     	andq	%r8, %rdx
14001c25e:     	movaps	%xmm10, %xmm0
14001c262:     	shufps	$0x0, %xmm10, %xmm0     # xmm0 = xmm0[0,0],xmm10[0,0]
14001c267:     	movaps	%xmm10, %xmm1
14001c26b:     	shufps	$0x55, %xmm10, %xmm1    # xmm1 = xmm1[1,1],xmm10[1,1]
14001c270:     	xorl	%r8d, %r8d
14001c273:     	nopw	%cs:(%rax,%rax)
14001c280:     	movups	(%r12,%r8,4), %xmm2
14001c285:     	movups	0x10(%r12,%r8,4), %xmm3
14001c28b:     	movaps	%xmm9, %xmm4
14001c28f:     	subps	%xmm2, %xmm4
14001c292:     	movaps	%xmm9, %xmm5
14001c296:     	subps	%xmm3, %xmm5
14001c299:     	mulps	%xmm0, %xmm4
14001c29c:     	mulps	%xmm0, %xmm5
14001c29f:     	mulps	%xmm1, %xmm2
14001c2a2:     	addps	%xmm4, %xmm2
14001c2a5:     	mulps	%xmm1, %xmm3
14001c2a8:     	addps	%xmm5, %xmm3
14001c2ab:     	movups	%xmm2, (%rax,%r8,4)
14001c2b0:     	movups	%xmm3, 0x10(%rax,%r8,4)
14001c2b6:     	addq	$0x8, %r8
14001c2ba:     	cmpq	%r8, %rdx
14001c2bd:     	jne	0x14001c280 <.text+0x1b280>
14001c2bf:     	cmpq	%rdx, %r15
14001c2c2:     	jne	0x14001c2cb <.text+0x1b2cb>
14001c2c4:     	jmp	0x14001c350 <.text+0x1b350>
14001c2c9:     	xorl	%edx, %edx
14001c2cb:     	movq	%rdx, %r8
14001c2ce:     	orq	$0x1, %r8
14001c2d2:     	testb	$0x1, %r15b
14001c2d6:     	je	0x14001c2fc <.text+0x1b2fc>
14001c2d8:     	movss	(%r12,%rdx,4), %xmm0
14001c2de:     	movaps	%xmm8, %xmm1
14001c2e2:     	subss	%xmm0, %xmm1
14001c2e6:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001c2e9:     	mulps	%xmm10, %xmm1
14001c2ed:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c2f1:     	addss	%xmm1, %xmm0
14001c2f5:     	movss	%xmm0, (%rax,%rdx,4)
14001c2fa:     	jmp	0x14001c34b <.text+0x1b34b>
14001c2fc:     	cmpq	%r8, %r15
14001c2ff:     	je	0x14001c350 <.text+0x1b350>
14001c301:     	movss	(%r12,%rdx,4), %xmm0
14001c307:     	movaps	%xmm8, %xmm1
14001c30b:     	subss	%xmm0, %xmm1
14001c30f:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001c312:     	mulps	%xmm10, %xmm1
14001c316:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c31a:     	addss	%xmm1, %xmm0
14001c31e:     	movss	%xmm0, (%rax,%rdx,4)
14001c323:     	movss	0x4(%r12,%rdx,4), %xmm0
14001c32a:     	movaps	%xmm8, %xmm1
14001c32e:     	subss	%xmm0, %xmm1
14001c332:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001c335:     	mulps	%xmm10, %xmm1
14001c339:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c33d:     	addss	%xmm1, %xmm0
14001c341:     	movss	%xmm0, 0x4(%rax,%rdx,4)
14001c347:     	leaq	0x2(%rdx), %r8
14001c34b:     	movq	%r8, %rdx
14001c34e:     	jmp	0x14001c2fc <.text+0x1b2fc>
14001c350:     	movq	0x258(%rbp), %rdx
14001c357:     	movl	0x424(%rbp), %r8d
14001c35e:     	movl	%r8d, (%rdx)
14001c361:     	movq	0x5c0(%rbp), %rdx
14001c368:     	movq	%rdx, 0x6f0(%rbp)
14001c36f:     	movaps	0x590(%rbp), %xmm0
14001c376:     	movaps	0x5a0(%rbp), %xmm1
14001c37d:     	movaps	0x5b0(%rbp), %xmm2
14001c384:     	movaps	%xmm2, 0x6e0(%rbp)
14001c38b:     	movaps	%xmm1, 0x6d0(%rbp)
14001c392:     	movaps	%xmm0, 0x6c0(%rbp)
14001c399:     	movq	0x808(%rbp), %rdx
14001c3a0:     	movq	%rdx, 0x6f8(%rbp)
14001c3a7:     	movq	0x7a0(%rbp), %rdx
14001c3ae:     	movq	%rdx, 0x700(%rbp)
14001c3b5:     	movq	%rbx, 0x708(%rbp)
14001c3bc:     	movq	0x6b0(%rbp), %rdx
14001c3c3:     	movq	%rdx, 0x728(%rbp)
14001c3ca:     	movq	0x7f0(%rbp), %rdx
14001c3d1:     	movq	%rdx, 0x730(%rbp)
14001c3d8:     	movq	%r14, 0x738(%rbp)
14001c3df:     	movq	%rcx, 0x710(%rbp)
14001c3e6:     	movq	%rax, 0x718(%rbp)
14001c3ed:     	movq	%r15, 0x720(%rbp)
14001c3f4:     	movq	0x788(%rbp), %rax
14001c3fb:     	movl	%eax, 0x760(%rbp)
14001c401:     	movq	0x670(%rbp), %rax
14001c408:     	movq	(%rax), %rax
14001c40b:     	movq	%rax, 0x758(%rbp)
14001c412:     	movw	$0x0, 0x764(%rbp)
14001c41b:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14001c425:     	movq	%rax, 0x740(%rbp)
14001c42c:     	movq	0x10(%r13), %rsi
14001c430:     	cmpq	(%r13), %rsi
14001c434:     	jne	0x14001c445 <.text+0x1b445>
14001c436:     	movq	%r13, %rcx
14001c439:     	leaq	0x290a3e8(%rip), %rdx   # 0x142926828
14001c440:     	callq	0x1405636c0 <.text+0x5626c0>
14001c445:     	imulq	$0xa8, %rsi, %rcx
14001c44c:     	addq	0x8(%r13), %rcx
14001c450:     	movl	$0xa8, %r8d
14001c456:     	leaq	0x6c0(%rbp), %rdx
14001c45d:     	callq	0x142923c10 <.text+0x2922c10>
14001c462:     	incq	%rsi
14001c465:     	movq	%rsi, 0x10(%r13)
14001c469:     	movq	0x200(%rbp), %rax
14001c470:     	movb	$0x1, 0x817(%rbp)
14001c477:     	movq	%rax, 0x48(%rsp)
14001c47c:     	movq	0x6a0(%rbp), %rax
14001c483:     	movq	%rax, 0x40(%rsp)
14001c488:     	leaq	0x278(%rbp), %rax
14001c48f:     	movq	%rax, 0x38(%rsp)
14001c494:     	leaq	0x550(%rbp), %rax
14001c49b:     	movq	%rax, 0x20(%rsp)
14001c4a0:     	movq	$0x1, 0x30(%rsp)
14001c4a9:     	movq	$0x2, 0x28(%rsp)
14001c4b2:     	movl	$0x2, %edx
14001c4b7:     	movl	$0x2, %r9d
14001c4bd:     	leaq	0x650(%rbp), %rcx
14001c4c4:     	leaq	0x300(%rbp), %r8
14001c4cb:     	callq	0x1407de8b0 <.text+0x7dd8b0>
14001c4d0:     	movb	$0x1, 0x817(%rbp)
14001c4d7:     	leaq	0x428(%rbp), %rax
14001c4de:     	movq	%rax, 0x38(%rsp)
14001c4e3:     	movl	0x640(%rbp), %eax
14001c4e9:     	movl	%eax, 0x30(%rsp)
14001c4ed:     	leaq	0x804(%rbp), %rax
14001c4f4:     	movq	%rax, 0x28(%rsp)
14001c4f9:     	movq	$0x2, 0x20(%rsp)
14001c502:     	movl	$0x2, %r8d
14001c508:     	leaq	0x198(%rbp), %rcx
14001c50f:     	leaq	0x650(%rbp), %rdx
14001c516:     	leaq	0x550(%rbp), %r9
14001c51d:     	callq	0x1407db3e0 <.text+0x7da3e0>
14001c522:     	movzbl	0x804(%rbp), %eax
14001c529:     	cmpl	$0x1, %eax
14001c52c:     	leaq	0x590(%rbp), %r12
14001c533:     	jbe	0x14001d34e <.text+0x1c34e>
14001c539:     	cmpl	$0x3, %eax
14001c53c:     	je	0x14001c63d <.text+0x1b63d>
14001c542:     	cmpl	$0x4, %eax
14001c545:     	movss	0x29093b7(%rip), %xmm3  # 0x142925904
14001c54d:     	jne	0x14001c674 <.text+0x1b674>
14001c553:     	movaps	%xmm14, %xmm0
14001c557:     	movlhps	%xmm13, %xmm0           # xmm0 = xmm0[0],xmm13[0]
14001c55b:     	shufps	$0xe2, %xmm13, %xmm0    # xmm0 = xmm0[2,0],xmm13[2,3]
14001c560:     	movaps	%xmm12, %xmm1
14001c564:     	movlhps	%xmm15, %xmm1           # xmm1 = xmm1[0],xmm15[0]
14001c568:     	shufps	$0xe2, %xmm15, %xmm1    # xmm1 = xmm1[2,0],xmm15[2,3]
14001c56d:     	subps	%xmm1, %xmm0
14001c570:     	movaps	%xmm0, %xmm1
14001c573:     	mulps	%xmm0, %xmm1
14001c576:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14001c57a:     	addss	%xmm1, %xmm2
14001c57e:     	xorps	%xmm1, %xmm1
14001c581:     	sqrtss	%xmm2, %xmm1
14001c585:     	movaps	%xmm3, %xmm2
14001c588:     	divss	%xmm1, %xmm2
14001c58c:     	movd	%xmm2, %eax
14001c590:     	testl	%eax, %eax
14001c592:     	setns	%cl
14001c595:     	movl	%eax, %edx
14001c597:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
14001c59d:     	addl	$0xff800000, %edx       # imm = 0xFF800000
14001c5a3:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
14001c5a9:     	setb	%dl
14001c5ac:     	andb	%cl, %dl
14001c5ae:     	decl	%eax
14001c5b0:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
14001c5b5:     	setb	%al
14001c5b8:     	orb	%dl, %al
14001c5ba:     	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
14001c5be:     	mulps	%xmm0, %xmm1
14001c5c1:     	movzbl	%al, %eax
14001c5c4:     	movd	%eax, %xmm0
14001c5c8:     	pshufd	$0x50, %xmm0, %xmm11    # xmm11 = xmm0[0,0,1,1]
14001c5ce:     	pslld	$0x1f, %xmm11
14001c5d4:     	psrad	$0x1f, %xmm11
14001c5da:     	pand	%xmm1, %xmm11
14001c5df:     	movq	%xmm11, 0x6c0(%rbp)
14001c5e8:     	movb	$0x1, 0x817(%rbp)
14001c5ef:     	leaq	0x68(%rbp), %rcx
14001c5f3:     	leaq	0x6c0(%rbp), %rdx
14001c5fa:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14001c5ff:     	movsd	0x68(%rbp), %xmm0
14001c604:     	mulps	0x2909335(%rip), %xmm0  # 0x142925940
14001c60b:     	movss	0x70(%rbp), %xmm1
14001c610:     	mulss	0x2909338(%rip), %xmm1  # 0x142925950
14001c618:     	subss	%xmm1, %xmm12
14001c61d:     	subps	%xmm0, %xmm15
14001c621:     	addps	%xmm0, %xmm13
14001c625:     	addss	%xmm1, %xmm14
14001c62a:     	movq	0x550(%rbp), %rbx
14001c631:     	leaq	0x460(%rbp), %rcx
14001c638:     	jmp	0x14001c8bd <.text+0x1b8bd>
14001c63d:     	movaps	%xmm15, %xmm0
14001c641:     	cmpltps	%xmm13, %xmm0
14001c646:     	pextrw	$0x2, %xmm0, %eax
14001c64b:     	testb	$0x1, %al
14001c64d:     	movss	0x29092af(%rip), %xmm3  # 0x142925904
14001c655:     	je	0x14001c686 <.text+0x1b686>
14001c657:     	movl	0x550(%rbp), %esi
14001c65d:     	movaps	%xmm6, %xmm1
14001c660:     	mulps	%xmm6, %xmm1
14001c663:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c667:     	addss	%xmm1, %xmm0
14001c66b:     	movb	$0x1, %dil
14001c66e:     	movaps	%xmm6, %xmm11
14001c672:     	jmp	0x14001c69e <.text+0x1b69e>
14001c674:     	movaps	%xmm13, %xmm0
14001c678:     	cmpltps	%xmm15, %xmm0
14001c67d:     	pextrw	$0x2, %xmm0, %eax
14001c682:     	testb	$0x1, %al
14001c684:     	jne	0x14001c657 <.text+0x1b657>
14001c686:     	movl	0x554(%rbp), %esi
14001c68c:     	movaps	%xmm11, %xmm1
14001c690:     	mulps	%xmm11, %xmm1
14001c694:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001c698:     	addss	%xmm1, %xmm0
14001c69c:     	xorl	%edi, %edi
14001c69e:     	sqrtss	%xmm0, %xmm0
14001c6a2:     	movaps	%xmm3, %xmm1
14001c6a5:     	divss	%xmm0, %xmm1
14001c6a9:     	movd	%xmm1, %eax
14001c6ad:     	testl	%eax, %eax
14001c6af:     	setns	%cl
14001c6b2:     	movl	%eax, %edx
14001c6b4:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
14001c6ba:     	addl	$0xff800000, %edx       # imm = 0xFF800000
14001c6c0:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
14001c6c6:     	setb	%dl
14001c6c9:     	andb	%cl, %dl
14001c6cb:     	decl	%eax
14001c6cd:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
14001c6d2:     	setb	%al
14001c6d5:     	orb	%dl, %al
14001c6d7:     	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
14001c6db:     	mulps	%xmm11, %xmm0
14001c6df:     	movzbl	%al, %eax
14001c6e2:     	movd	%eax, %xmm1
14001c6e6:     	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
14001c6eb:     	pslld	$0x1f, %xmm1
14001c6f0:     	psrad	$0x1f, %xmm1
14001c6f5:     	pand	%xmm0, %xmm1
14001c6f9:     	movq	%xmm1, 0x590(%rbp)
14001c701:     	movshdup	%xmm15, %xmm0           # xmm0 = xmm15[1,1,3,3]
14001c706:     	movshdup	%xmm13, %xmm1           # xmm1 = xmm13[1,1,3,3]
14001c70b:     	movaps	%xmm0, %xmm11
14001c70f:     	cmpunordss	%xmm0, %xmm11
14001c715:     	movaps	%xmm11, %xmm2
14001c719:     	andps	%xmm1, %xmm2
14001c71c:     	minss	%xmm0, %xmm1
14001c720:     	andnps	%xmm1, %xmm11
14001c724:     	orps	%xmm2, %xmm11
14001c728:     	movss	%xmm15, 0x6c0(%rbp)
14001c731:     	movss	%xmm12, 0x6c4(%rbp)
14001c73a:     	movb	$0x1, 0x817(%rbp)
14001c741:     	leaq	0x228(%rbp), %rcx
14001c748:     	leaq	0x6c0(%rbp), %rdx
14001c74f:     	movaps	%xmm11, %xmm2
14001c753:     	callq	0x140c9cde0 <.text+0xc9bde0>
14001c758:     	movss	%xmm13, 0x6c0(%rbp)
14001c761:     	movss	%xmm14, 0x6c4(%rbp)
14001c76a:     	movb	$0x1, 0x817(%rbp)
14001c771:     	leaq	0x218(%rbp), %rcx
14001c778:     	leaq	0x6c0(%rbp), %rdx
14001c77f:     	movaps	%xmm11, %xmm2
14001c783:     	callq	0x140c9cde0 <.text+0xc9bde0>
14001c788:     	testb	%dil, %dil
14001c78b:     	je	0x14001c7e9 <.text+0x1b7e9>
14001c78d:     	movb	$0x1, 0x817(%rbp)
14001c794:     	leaq	0x6c0(%rbp), %rcx
14001c79b:     	leaq	0x590(%rbp), %rdx
14001c7a2:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14001c7a7:     	movsd	0x6c0(%rbp), %xmm13
14001c7b0:     	movss	0x6c8(%rbp), %xmm14
14001c7b9:     	cmpb	$0x3, 0x804(%rbp)
14001c7c0:     	jne	0x14001c85b <.text+0x1b85b>
14001c7c6:     	mulps	0x2909112(%rip), %xmm13 # 0x1429258e0
14001c7ce:     	mulss	0x2909099(%rip), %xmm14 # 0x142925870
14001c7d7:     	movsd	0x228(%rbp), %xmm0
14001c7df:     	movss	0x230(%rbp), %xmm1
14001c7e7:     	jmp	0x14001c83f <.text+0x1b83f>
14001c7e9:     	movb	$0x1, 0x817(%rbp)
14001c7f0:     	leaq	0x6c0(%rbp), %rcx
14001c7f7:     	leaq	0x590(%rbp), %rdx
14001c7fe:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14001c803:     	movsd	0x6c0(%rbp), %xmm13
14001c80c:     	movss	0x6c8(%rbp), %xmm14
14001c815:     	cmpb	$0x3, 0x804(%rbp)
14001c81c:     	jne	0x14001c878 <.text+0x1b878>
14001c81e:     	mulps	0x29090ba(%rip), %xmm13 # 0x1429258e0
14001c826:     	mulss	0x2909041(%rip), %xmm14 # 0x142925870
14001c82f:     	movsd	0x218(%rbp), %xmm0
14001c837:     	movss	0x220(%rbp), %xmm1
14001c83f:     	movaps	%xmm0, %xmm15
14001c843:     	subps	%xmm13, %xmm15
14001c847:     	movaps	%xmm1, %xmm12
14001c84b:     	subss	%xmm14, %xmm12
14001c850:     	addps	%xmm0, %xmm13
14001c854:     	addss	%xmm1, %xmm14
14001c859:     	jmp	0x14001c8a1 <.text+0x1b8a1>
14001c85b:     	movsd	0x228(%rbp), %xmm15
14001c864:     	movss	0x230(%rbp), %xmm12
14001c86d:     	addps	%xmm15, %xmm13
14001c871:     	addss	%xmm12, %xmm14
14001c876:     	jmp	0x14001c8a1 <.text+0x1b8a1>
14001c878:     	movsd	0x218(%rbp), %xmm0
14001c880:     	movss	0x220(%rbp), %xmm1
14001c888:     	movaps	%xmm0, %xmm15
14001c88c:     	subps	%xmm13, %xmm15
14001c890:     	movaps	%xmm1, %xmm12
14001c894:     	subss	%xmm14, %xmm12
14001c899:     	movaps	%xmm1, %xmm14
14001c89d:     	movaps	%xmm0, %xmm13
14001c8a1:     	leaq	0x460(%rbp), %rcx
14001c8a8:     	movq	0x590(%rbp), %xmm11
14001c8b1:     	movl	%esi, %eax
14001c8b3:     	movq	%rax, %rbx
14001c8b6:     	shlq	$0x20, %rbx
14001c8ba:     	orq	%rax, %rbx
14001c8bd:     	movlps	%xmm15, 0x680(%rbp)
14001c8c5:     	movss	%xmm12, 0x688(%rbp)
14001c8ce:     	movlps	%xmm13, 0x68c(%rbp)
14001c8d6:     	movss	%xmm14, 0x694(%rbp)
14001c8df:     	movshdup	%xmm15, %xmm0           # xmm0 = xmm15[1,1,3,3]
14001c8e4:     	minss	0x2909068(%rip), %xmm0  # 0x142925954
14001c8ec:     	movshdup	%xmm13, %xmm1           # xmm1 = xmm13[1,1,3,3]
14001c8f1:     	minss	%xmm0, %xmm1
14001c8f5:     	movss	%xmm1, 0x568(%rbp)
14001c8fd:     	leaq	0x680(%rbp), %rax
14001c904:     	movq	%rax, 0x6c0(%rbp)
14001c90b:     	leaq	0x698(%rbp), %rax
14001c912:     	movq	%rax, 0x6c8(%rbp)
14001c919:     	leaq	0x568(%rbp), %rax
14001c920:     	movq	%rax, 0x6d0(%rbp)
14001c927:     	movb	$0x1, 0x817(%rbp)
14001c92e:     	leaq	0x6c0(%rbp), %rdx
14001c935:     	leaq	0x290a9f4(%rip), %r8    # 0x142927330
14001c93c:     	callq	0x1402befa0 <.text+0x2bdfa0>
14001c941:     	movb	$0x1, 0x817(%rbp)
14001c948:     	leaq	0x590(%rbp), %rcx
14001c94f:     	leaq	0x460(%rbp), %rdx
14001c956:     	xorl	%r8d, %r8d
14001c959:     	callq	0x140153640 <.text+0x152640>
14001c95e:     	xorl	%esi, %esi
14001c960:     	cmpq	0x590(%rbp), %rsi
14001c967:     	jo	0x14001d34e <.text+0x1c34e>
14001c96d:     	movq	0x5c0(%rbp), %rax
14001c974:     	movq	%rax, 0x6f0(%rbp)
14001c97b:     	movups	0x590(%rbp), %xmm0
14001c982:     	movups	0x5a0(%rbp), %xmm1
14001c989:     	movups	0x5b0(%rbp), %xmm2
14001c990:     	movaps	%xmm2, 0x6e0(%rbp)
14001c997:     	movaps	%xmm1, 0x6d0(%rbp)
14001c99e:     	movaps	%xmm0, 0x6c0(%rbp)
14001c9a5:     	leaq	0x380(%rbp), %rcx
14001c9ac:     	leaq	0x6c0(%rbp), %rdx
14001c9b3:     	movd	0x2908eb5(%rip), %xmm2  # 0x142925870
14001c9bb:     	movl	$0x1, %r9d
14001c9c1:     	callq	0x140152b00 <.text+0x151b00>
14001c9c6:     	movq	0x6c0(%rbp), %rax
14001c9cd:     	testq	%rax, %rax
14001c9d0:     	je	0x14001c9ec <.text+0x1b9ec>
14001c9d2:     	movq	0x6c8(%rbp), %rcx
14001c9d9:     	shlq	$0x2, %rax
14001c9dd:     	leaq	(%rax,%rax,2), %rdx
14001c9e1:     	movl	$0x4, %r8d
14001c9e7:     	callq	0x140613c20 <.text+0x612c20>
14001c9ec:     	movq	0x6d8(%rbp), %rdx
14001c9f3:     	testq	%rdx, %rdx
14001c9f6:     	je	0x14001ca0e <.text+0x1ba0e>
14001c9f8:     	movq	0x6e0(%rbp), %rcx
14001c9ff:     	shlq	$0x2, %rdx
14001ca03:     	movl	$0x4, %r8d
14001ca09:     	callq	0x140613c20 <.text+0x612c20>
14001ca0e:     	cmpq	0x380(%rbp), %rsi
14001ca15:     	jo	0x14001d34e <.text+0x1c34e>
14001ca1b:     	movq	0x3b0(%rbp), %rax
14001ca22:     	movq	%rax, 0x490(%rbp)
14001ca29:     	movupd	0x380(%rbp), %xmm0
14001ca31:     	movups	0x390(%rbp), %xmm1
14001ca38:     	movups	0x3a0(%rbp), %xmm2
14001ca3f:     	movaps	%xmm2, 0x480(%rbp)
14001ca46:     	movaps	%xmm1, 0x470(%rbp)
14001ca4d:     	movapd	%xmm0, 0x460(%rbp)
14001ca55:     	movq	0x470(%rbp), %rdi
14001ca5c:     	testq	%rdi, %rdi
14001ca5f:     	je	0x14001cb1b <.text+0x1bb1b>
14001ca65:     	leaq	(,%rdi,8), %rsi
14001ca6d:     	movzbl	0x3618195(%rip), %eax   # 0x143634c09
14001ca74:     	movl	$0x4, %edx
14001ca79:     	movq	%rsi, %rcx
14001ca7c:     	callq	0x140613c10 <.text+0x612c10>
14001ca81:     	testq	%rax, %rax
14001ca84:     	je	0x140020c7d <.text+0x1fc7d>
14001ca8a:     	movl	$0x1, %r8d
14001ca90:     	movq	%rax, %rcx
14001ca93:     	cmpq	$0x1, %rdi
14001ca97:     	movss	0x2908e65(%rip), %xmm7  # 0x142925904
14001ca9f:     	movaps	0x2908e89(%rip), %xmm8  # 0x142925930
14001caa7:     	je	0x14001cb14 <.text+0x1bb14>
14001caa9:     	movl	$0x1, %edx
14001caae:     	movq	%rax, %rcx
14001cab1:     	cmpq	$0x4, %rdi
14001cab5:     	jbe	0x14001cb00 <.text+0x1bb00>
14001cab7:     	leaq	-0x1(%rdi), %r9
14001cabb:     	movq	%r9, %r10
14001cabe:     	andq	$-0x4, %r10
14001cac2:     	leaq	(%rax,%r10,8), %rcx
14001cac6:     	leaq	0x1(%r10), %rdx
14001caca:     	movddup	%xmm11, %xmm0           # xmm0 = xmm11[0,0]
14001cacf:     	xorl	%r8d, %r8d
14001cad2:     	nopw	%cs:(%rax,%rax)
14001cae0:     	movupd	%xmm0, (%rax,%r8,8)
14001cae6:     	movupd	%xmm0, 0x10(%rax,%r8,8)
14001caed:     	addq	$0x4, %r8
14001caf1:     	cmpq	%r8, %r10
14001caf4:     	jne	0x14001cae0 <.text+0x1bae0>
14001caf6:     	movq	%rdi, %r8
14001caf9:     	cmpq	%r10, %r9
14001cafc:     	je	0x14001cb14 <.text+0x1bb14>
14001cafe:     	nop
14001cb00:     	incq	%rdx
14001cb03:     	movlpd	%xmm11, (%rcx)
14001cb08:     	addq	$0x8, %rcx
14001cb0c:     	cmpq	%rdx, %rdi
14001cb0f:     	jne	0x14001cb00 <.text+0x1bb00>
14001cb11:     	movq	%rdi, %r8
14001cb14:     	movlpd	%xmm11, (%rcx)
14001cb19:     	jmp	0x14001cb33 <.text+0x1bb33>
14001cb1b:     	movl	$0x4, %eax
14001cb20:     	xorl	%r8d, %r8d
14001cb23:     	movss	0x2908dd9(%rip), %xmm7  # 0x142925904
14001cb2b:     	movaps	0x2908dfd(%rip), %xmm8  # 0x142925930
14001cb33:     	movq	%r8, 0x508(%rbp)
14001cb3a:     	movq	%rax, 0x510(%rbp)
14001cb41:     	movq	0x470(%rbp), %r15
14001cb48:     	movq	0x488(%rbp), %rax
14001cb4f:     	cmpq	%r15, %rax
14001cb52:     	cmovbq	%rax, %r15
14001cb56:     	movq	%rdi, 0x518(%rbp)
14001cb5d:     	leaq	(,%r15,4), %rdi
14001cb65:     	movq	%r15, %rax
14001cb68:     	shrq	$0x3e, %rax
14001cb6c:     	jne	0x140020d8d <.text+0x1fd8d>
14001cb72:     	movl	$0x0, %esi
14001cb77:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14001cb81:     	cmpq	%rax, %rdi
14001cb84:     	ja	0x140020d8f <.text+0x1fd8f>
14001cb8a:     	movq	0x480(%rbp), %r14
14001cb91:     	testq	%rdi, %rdi
14001cb94:     	je	0x14001cbc5 <.text+0x1bbc5>
14001cb96:     	movzbl	0x361806c(%rip), %eax   # 0x143634c09
14001cb9d:     	movl	$0x4, %esi
14001cba2:     	movl	$0x4, %edx
14001cba7:     	movq	%rdi, %rcx
14001cbaa:     	callq	0x140613c10 <.text+0x612c10>
14001cbaf:     	testq	%rax, %rax
14001cbb2:     	je	0x140020d8f <.text+0x1fd8f>
14001cbb8:     	movq	%r15, %rcx
14001cbbb:     	testq	%r15, %r15
14001cbbe:     	jne	0x14001cbd5 <.text+0x1bbd5>
14001cbc0:     	jmp	0x14001ccfa <.text+0x1bcfa>
14001cbc5:     	movl	$0x4, %eax
14001cbca:     	xorl	%ecx, %ecx
14001cbcc:     	testq	%r15, %r15
14001cbcf:     	je	0x14001ccfa <.text+0x1bcfa>
14001cbd5:     	movq	%rbx, %rdx
14001cbd8:     	shrq	$0x20, %rdx
14001cbdc:     	movd	%ebx, %xmm1
14001cbe0:     	movd	%edx, %xmm0
14001cbe4:     	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
14001cbe8:     	cmpq	$0x8, %r15
14001cbec:     	jb	0x14001cc79 <.text+0x1bc79>
14001cbf2:     	movq	%rax, %r8
14001cbf5:     	subq	%r14, %r8
14001cbf8:     	movl	$0x0, %edx
14001cbfd:     	cmpq	$0x20, %r8
14001cc01:     	jb	0x14001cc7b <.text+0x1bc7b>
14001cc03:     	movq	%r15, %rdx
14001cc06:     	movabsq	$0x3ffffffffffffff8, %r8 # imm = 0x3FFFFFFFFFFFFFF8
14001cc10:     	andq	%r8, %rdx
14001cc13:     	movdqa	%xmm0, %xmm1
14001cc17:     	shufps	$0x55, %xmm0, %xmm1     # xmm1 = xmm1[1,1],xmm0[1,1]
14001cc1b:     	movaps	%xmm0, %xmm2
14001cc1e:     	shufps	$0x0, %xmm0, %xmm2      # xmm2 = xmm2[0,0],xmm0[0,0]
14001cc22:     	xorl	%r8d, %r8d
14001cc25:     	nopw	%cs:(%rax,%rax)
14001cc30:     	movups	(%r14,%r8,4), %xmm3
14001cc35:     	movups	0x10(%r14,%r8,4), %xmm4
14001cc3b:     	movaps	%xmm8, %xmm5
14001cc3f:     	subps	%xmm3, %xmm5
14001cc42:     	movaps	%xmm8, %xmm6
14001cc46:     	subps	%xmm4, %xmm6
14001cc49:     	mulps	%xmm1, %xmm5
14001cc4c:     	mulps	%xmm1, %xmm6
14001cc4f:     	mulps	%xmm2, %xmm3
14001cc52:     	addps	%xmm5, %xmm3
14001cc55:     	mulps	%xmm2, %xmm4
14001cc58:     	addps	%xmm6, %xmm4
14001cc5b:     	movups	%xmm3, (%rax,%r8,4)
14001cc60:     	movups	%xmm4, 0x10(%rax,%r8,4)
14001cc66:     	addq	$0x8, %r8
14001cc6a:     	cmpq	%r8, %rdx
14001cc6d:     	jne	0x14001cc30 <.text+0x1bc30>
14001cc6f:     	cmpq	%rdx, %r15
14001cc72:     	jne	0x14001cc7b <.text+0x1bc7b>
14001cc74:     	jmp	0x14001ccfa <.text+0x1bcfa>
14001cc79:     	xorl	%edx, %edx
14001cc7b:     	movq	%rdx, %r8
14001cc7e:     	orq	$0x1, %r8
14001cc82:     	testb	$0x1, %r15b
14001cc86:     	je	0x14001ccaa <.text+0x1bcaa>
14001cc88:     	movss	(%r14,%rdx,4), %xmm1
14001cc8e:     	movaps	%xmm7, %xmm2
14001cc91:     	subss	%xmm1, %xmm2
14001cc95:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14001cc98:     	mulps	%xmm0, %xmm1
14001cc9b:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14001cc9f:     	addss	%xmm1, %xmm2
14001cca3:     	movss	%xmm2, (%rax,%rdx,4)
14001cca8:     	jmp	0x14001ccf5 <.text+0x1bcf5>
14001ccaa:     	cmpq	%r8, %r15
14001ccad:     	je	0x14001ccfa <.text+0x1bcfa>
14001ccaf:     	movss	(%r14,%rdx,4), %xmm1
14001ccb5:     	movaps	%xmm7, %xmm2
14001ccb8:     	subss	%xmm1, %xmm2
14001ccbc:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14001ccbf:     	mulps	%xmm0, %xmm1
14001ccc2:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14001ccc6:     	addss	%xmm1, %xmm2
14001ccca:     	movss	%xmm2, (%rax,%rdx,4)
14001cccf:     	movss	0x4(%r14,%rdx,4), %xmm1
14001ccd6:     	movaps	%xmm7, %xmm2
14001ccd9:     	subss	%xmm1, %xmm2
14001ccdd:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14001cce0:     	mulps	%xmm0, %xmm1
14001cce3:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14001cce7:     	addss	%xmm1, %xmm2
14001cceb:     	movss	%xmm2, 0x4(%rax,%rdx,4)
14001ccf1:     	leaq	0x2(%rdx), %r8
14001ccf5:     	movq	%r8, %rdx
14001ccf8:     	jmp	0x14001ccaa <.text+0x1bcaa>
14001ccfa:     	movq	%rcx, 0x3e0(%rbp)
14001cd01:     	movq	%rax, 0x3e8(%rbp)
14001cd08:     	movq	0x470(%rbp), %r14
14001cd0f:     	movq	0x488(%rbp), %rax
14001cd16:     	cmpq	%r14, %rax
14001cd19:     	cmovbq	%rax, %r14
14001cd1d:     	movq	%r15, 0x3f0(%rbp)
14001cd24:     	leaq	(,%r14,4), %rdi
14001cd2c:     	movq	%r14, %rax
14001cd2f:     	shrq	$0x3e, %rax
14001cd33:     	jne	0x140020d06 <.text+0x1fd06>
14001cd39:     	movl	$0x0, %esi
14001cd3e:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14001cd48:     	cmpq	%rax, %rdi
14001cd4b:     	ja	0x140020d08 <.text+0x1fd08>
14001cd51:     	movq	0x480(%rbp), %rbx
14001cd58:     	testq	%rdi, %rdi
14001cd5b:     	je	0x14001cd90 <.text+0x1bd90>
14001cd5d:     	movzbl	0x3617ea5(%rip), %eax   # 0x143634c09
14001cd64:     	movl	$0x4, %esi
14001cd69:     	movl	$0x4, %edx
14001cd6e:     	movq	%rdi, %rcx
14001cd71:     	callq	0x140613c10 <.text+0x612c10>
14001cd76:     	testq	%rax, %rax
14001cd79:     	je	0x140020d08 <.text+0x1fd08>
14001cd7f:     	movq	%r14, 0x7a0(%rbp)
14001cd86:     	testq	%r14, %r14
14001cd89:     	jne	0x14001cda9 <.text+0x1bda9>
14001cd8b:     	jmp	0x14001ceb6 <.text+0x1beb6>
14001cd90:     	movl	$0x4, %eax
14001cd95:     	movq	$0x0, 0x7a0(%rbp)
14001cda0:     	testq	%r14, %r14
14001cda3:     	je	0x14001ceb6 <.text+0x1beb6>
14001cda9:     	cmpq	$0x8, %r14
14001cdad:     	jb	0x14001ce35 <.text+0x1be35>
14001cdb3:     	movq	%rax, %rcx
14001cdb6:     	subq	%rbx, %rcx
14001cdb9:     	movl	$0x0, %edx
14001cdbe:     	cmpq	$0x20, %rcx
14001cdc2:     	jb	0x14001ce37 <.text+0x1be37>
14001cdc4:     	movq	%r14, %rdx
14001cdc7:     	movabsq	$0x3ffffffffffffff8, %rcx # imm = 0x3FFFFFFFFFFFFFF8
14001cdd1:     	andq	%rcx, %rdx
14001cdd4:     	movaps	%xmm10, %xmm0
14001cdd8:     	shufps	$0x0, %xmm10, %xmm0     # xmm0 = xmm0[0,0],xmm10[0,0]
14001cddd:     	movaps	%xmm10, %xmm1
14001cde1:     	shufps	$0x55, %xmm10, %xmm1    # xmm1 = xmm1[1,1],xmm10[1,1]
14001cde6:     	xorl	%ecx, %ecx
14001cde8:     	nopl	(%rax,%rax)
14001cdf0:     	movups	(%rbx,%rcx,4), %xmm2
14001cdf4:     	movups	0x10(%rbx,%rcx,4), %xmm3
14001cdf9:     	movaps	%xmm8, %xmm4
14001cdfd:     	subps	%xmm2, %xmm4
14001ce00:     	movaps	%xmm8, %xmm5
14001ce04:     	subps	%xmm3, %xmm5
14001ce07:     	mulps	%xmm0, %xmm4
14001ce0a:     	mulps	%xmm0, %xmm5
14001ce0d:     	mulps	%xmm1, %xmm2
14001ce10:     	addps	%xmm4, %xmm2
14001ce13:     	mulps	%xmm1, %xmm3
14001ce16:     	addps	%xmm5, %xmm3
14001ce19:     	movups	%xmm2, (%rax,%rcx,4)
14001ce1d:     	movups	%xmm3, 0x10(%rax,%rcx,4)
14001ce22:     	addq	$0x8, %rcx
14001ce26:     	cmpq	%rcx, %rdx
14001ce29:     	jne	0x14001cdf0 <.text+0x1bdf0>
14001ce2b:     	cmpq	%rdx, %r14
14001ce2e:     	jne	0x14001ce37 <.text+0x1be37>
14001ce30:     	jmp	0x14001ceb6 <.text+0x1beb6>
14001ce35:     	xorl	%edx, %edx
14001ce37:     	movq	%rdx, %rcx
14001ce3a:     	orq	$0x1, %rcx
14001ce3e:     	testb	$0x1, %r14b
14001ce42:     	je	0x14001ce66 <.text+0x1be66>
14001ce44:     	movss	(%rbx,%rdx,4), %xmm0
14001ce49:     	movaps	%xmm7, %xmm1
14001ce4c:     	subss	%xmm0, %xmm1
14001ce50:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001ce53:     	mulps	%xmm10, %xmm1
14001ce57:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001ce5b:     	addss	%xmm1, %xmm0
14001ce5f:     	movss	%xmm0, (%rax,%rdx,4)
14001ce64:     	jmp	0x14001ceb1 <.text+0x1beb1>
14001ce66:     	cmpq	%rcx, %r14
14001ce69:     	je	0x14001ceb6 <.text+0x1beb6>
14001ce6b:     	movss	(%rbx,%rdx,4), %xmm0
14001ce70:     	movaps	%xmm7, %xmm1
14001ce73:     	subss	%xmm0, %xmm1
14001ce77:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001ce7a:     	mulps	%xmm10, %xmm1
14001ce7e:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001ce82:     	addss	%xmm1, %xmm0
14001ce86:     	movss	%xmm0, (%rax,%rdx,4)
14001ce8b:     	movss	0x4(%rbx,%rdx,4), %xmm0
14001ce91:     	movaps	%xmm7, %xmm1
14001ce94:     	subss	%xmm0, %xmm1
14001ce98:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14001ce9b:     	mulps	%xmm10, %xmm1
14001ce9f:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14001cea3:     	addss	%xmm1, %xmm0
14001cea7:     	movss	%xmm0, 0x4(%rax,%rdx,4)
14001cead:     	leaq	0x2(%rdx), %rcx
14001ceb1:     	movq	%rcx, %rdx
14001ceb4:     	jmp	0x14001ce66 <.text+0x1be66>
14001ceb6:     	movq	%rax, 0x808(%rbp)
14001cebd:     	movb	$0x1, 0x812(%rbp)
14001cec4:     	movb	$0x1, 0x811(%rbp)
14001cecb:     	movq	0x268(%rbp), %rcx
14001ced2:     	callq	0x140987ca0 <.text+0x986ca0>
14001ced7:     	movq	%rax, %rdi
14001ceda:     	movq	0x3e8(%rbp), %r12
14001cee1:     	movq	0x3f0(%rbp), %r8
14001cee8:     	movq	0x510(%rbp), %r13
14001ceef:     	movq	0x518(%rbp), %r11
14001cef6:     	movq	0x468(%rbp), %rsi
14001cefd:     	movq	0x470(%rbp), %rbx
14001cf04:     	testq	%r8, %r8
14001cf07:     	je	0x14001cf5f <.text+0x1bf5f>
14001cf09:     	movq	%rbx, 0x7f0(%rbp)
14001cf10:     	movq	%r11, %rbx
14001cf13:     	movq	%r8, %r15
14001cf16:     	leaq	(,%r8,4), %rcx
14001cf1e:     	movzbl	0x3617ce4(%rip), %eax   # 0x143634c09
14001cf25:     	movl	$0x4, %edx
14001cf2a:     	movq	%rcx, 0x7d0(%rbp)
14001cf31:     	callq	0x140613c40 <.text+0x612c40>
14001cf36:     	testq	%rax, %rax
14001cf39:     	je	0x140020c39 <.text+0x1fc39>
14001cf3f:     	movq	%rax, %r10
14001cf42:     	movl	0x540(%rbp), %ecx
14001cf48:     	movss	0x370(%rbp), %xmm1
14001cf50:     	movq	%r15, %r8
14001cf53:     	movq	%rbx, %r11
14001cf56:     	movq	0x7f0(%rbp), %rbx
14001cf5d:     	jmp	0x14001cf73 <.text+0x1bf73>
14001cf5f:     	movl	$0x4, %r10d
14001cf65:     	movl	0x540(%rbp), %ecx
14001cf6b:     	movss	0x370(%rbp), %xmm1
14001cf73:     	movq	%r8, 0x6b0(%rbp)
14001cf7a:     	movl	0x500(%rbp), %eax
14001cf80:     	movl	%eax, 0x6c0(%rbp)
14001cf86:     	movss	0x548(%rbp), %xmm0
14001cf8e:     	movss	%xmm0, 0x6c4(%rbp)
14001cf96:     	movl	%ecx, 0x6c8(%rbp)
14001cf9c:     	movss	%xmm1, 0x6cc(%rbp)
14001cfa4:     	movq	%r8, 0x60(%rsp)
14001cfa9:     	movq	%r10, 0x7f0(%rbp)
14001cfb0:     	movq	%r10, 0x58(%rsp)
14001cfb5:     	leaq	0x6c0(%rbp), %rax
14001cfbc:     	movq	%rax, 0x50(%rsp)
14001cfc1:     	movq	0x788(%rbp), %rax
14001cfc8:     	movl	%eax, 0x48(%rsp)
14001cfcc:     	movq	0x940(%rbp), %rax
14001cfd3:     	movq	%rax, 0x40(%rsp)
14001cfd8:     	leaq	0x140(%rbp), %rax
14001cfdf:     	movq	%rax, 0x38(%rsp)
14001cfe4:     	movq	%rbx, 0x30(%rsp)
14001cfe9:     	movq	%rsi, 0x28(%rsp)
14001cfee:     	movq	%r11, 0x20(%rsp)
14001cff3:     	movq	%rdi, %rcx
14001cff6:     	movq	%r12, %rdx
14001cff9:     	movq	%r13, %r9
14001cffc:     	callq	0x1407dc170 <.text+0x7db170>
14001d001:     	movq	0x6b0(%rbp), %rdx
14001d008:     	testq	%rdx, %rdx
14001d00b:     	leaq	0x590(%rbp), %r12
14001d012:     	je	0x14001d02a <.text+0x1c02a>
14001d014:     	shlq	$0x2, %rdx
14001d018:     	movl	$0x4, %r8d
14001d01e:     	movq	0x7f0(%rbp), %rcx
14001d025:     	callq	0x140613c20 <.text+0x612c20>
14001d02a:     	movq	0x938(%rbp), %rax
14001d031:     	movq	(%rax), %rsi
14001d034:     	movb	0x806(%rbp), %bl
14001d03a:     	movb	0x807(%rbp), %dil
14001d041:     	movb	$0x1, 0x812(%rbp)
14001d048:     	movb	$0x1, 0x811(%rbp)
14001d04f:     	leaq	0x290(%rbp), %rcx
14001d056:     	callq	0x1400284b0 <.text+0x274b0>
14001d05b:     	movl	$0x0, 0x6c0(%rbp)
14001d065:     	movl	$0x0, 0x6c8(%rbp)
14001d06f:     	movb	$0x1, 0x812(%rbp)
14001d076:     	movb	$0x1, 0x811(%rbp)
14001d07d:     	movq	%r14, 0x80(%rsp)
14001d085:     	movq	0x808(%rbp), %rcx
14001d08c:     	movq	%rcx, 0x78(%rsp)
14001d091:     	leaq	0x6c0(%rbp), %rcx
14001d098:     	movq	%rcx, 0x70(%rsp)
14001d09d:     	andb	$0x1, %dl
14001d0a0:     	movb	%dl, 0x68(%rsp)
14001d0a4:     	andb	$0x1, %al
14001d0a6:     	movb	%al, 0x60(%rsp)
14001d0aa:     	movb	%dil, 0x58(%rsp)
14001d0af:     	movb	%bl, 0x50(%rsp)
14001d0b3:     	movl	0x6a8(%rbp), %eax
14001d0b9:     	movb	%al, 0x48(%rsp)
14001d0bd:     	movq	%rsi, 0x38(%rsp)
14001d0c2:     	movl	0x640(%rbp), %eax
14001d0c8:     	movl	%eax, 0x30(%rsp)
14001d0cc:     	movq	0x788(%rbp), %rax
14001d0d3:     	movl	%eax, 0x28(%rsp)
14001d0d7:     	leaq	0x428(%rbp), %rax
14001d0de:     	movq	%rax, 0x20(%rsp)
14001d0e3:     	movw	$0x101, 0x40(%rsp)      # imm = 0x101
14001d0ea:     	leaq	0x140(%rbp), %rcx
14001d0f1:     	leaq	0x3e0(%rbp), %rdx
14001d0f8:     	leaq	0x508(%rbp), %r8
14001d0ff:     	leaq	0x460(%rbp), %r9
14001d106:     	callq	0x1407f98e0 <.text+0x7f88e0>
14001d10b:     	movq	0x490(%rbp), %rax
14001d112:     	movq	%rax, 0x6f0(%rbp)
14001d119:     	movaps	0x460(%rbp), %xmm0
14001d120:     	movaps	0x470(%rbp), %xmm1
14001d127:     	movaps	0x480(%rbp), %xmm2
14001d12e:     	movaps	%xmm2, 0x6e0(%rbp)
14001d135:     	movaps	%xmm1, 0x6d0(%rbp)
14001d13c:     	movaps	%xmm0, 0x6c0(%rbp)
14001d143:     	movq	0x3f0(%rbp), %rdi
14001d14a:     	testq	%rdi, %rdi
14001d14d:     	je	0x14001d175 <.text+0x1c175>
14001d14f:     	leaq	(,%rdi,4), %rsi
14001d157:     	movzbl	0x3617aab(%rip), %eax   # 0x143634c09
14001d15e:     	movl	$0x4, %edx
14001d163:     	movq	%rsi, %rcx
14001d166:     	callq	0x140613c40 <.text+0x612c40>
14001d16b:     	testq	%rax, %rax
14001d16e:     	jne	0x14001d17a <.text+0x1c17a>
14001d170:     	jmp	0x140020c64 <.text+0x1fc64>
14001d175:     	movl	$0x4, %eax
14001d17a:     	movq	0x3f0(%rbp), %rcx
14001d181:     	leaq	0x598(%rbp), %rdx
14001d188:     	movq	%rcx, 0x58(%rdx)
14001d18c:     	movups	0x3e0(%rbp), %xmm0
14001d193:     	movups	%xmm0, 0x48(%rdx)
14001d197:     	movq	0x518(%rbp), %rcx
14001d19e:     	movq	%rcx, 0x40(%rdx)
14001d1a2:     	movups	0x508(%rbp), %xmm0
14001d1a9:     	movups	%xmm0, 0x30(%rdx)
14001d1ad:     	movq	0x6f0(%rbp), %rcx
14001d1b4:     	movq	%rcx, 0x5c0(%rbp)
14001d1bb:     	movaps	0x6c0(%rbp), %xmm0
14001d1c2:     	movaps	0x6d0(%rbp), %xmm1
14001d1c9:     	movaps	0x6e0(%rbp), %xmm2
14001d1d0:     	movaps	%xmm0, 0x590(%rbp)
14001d1d7:     	movaps	%xmm1, 0x5a0(%rbp)
14001d1de:     	movaps	%xmm2, 0x5b0(%rbp)
14001d1e5:     	movaps	%xmm0, 0x6c0(%rbp)
14001d1ec:     	movaps	%xmm1, 0x6d0(%rbp)
14001d1f3:     	movaps	%xmm2, 0x6e0(%rbp)
14001d1fa:     	movaps	0x5d0(%rbp), %xmm0
14001d201:     	movaps	0x5e0(%rbp), %xmm1
14001d208:     	movaps	%xmm0, 0x700(%rbp)
14001d20f:     	movaps	%xmm1, 0x710(%rbp)
14001d216:     	movq	0x5f0(%rbp), %rcx
14001d21d:     	movq	%rcx, 0x720(%rbp)
14001d224:     	movq	0x5c0(%rbp), %rcx
14001d22b:     	movq	%rcx, 0x6f0(%rbp)
14001d232:     	movq	0x5c8(%rbp), %rcx
14001d239:     	movq	%rcx, 0x6f8(%rbp)
14001d240:     	movq	%rdi, 0x728(%rbp)
14001d247:     	movq	%rax, 0x730(%rbp)
14001d24e:     	movq	%rdi, 0x738(%rbp)
14001d255:     	movq	0x438(%rbp), %rax
14001d25c:     	movq	%rax, 0x740(%rbp)
14001d263:     	movq	0x670(%rbp), %rax
14001d26a:     	movq	(%rax), %rax
14001d26d:     	movq	%rax, 0x748(%rbp)
14001d274:     	movq	0x788(%rbp), %rax
14001d27b:     	movl	%eax, 0x750(%rbp)
14001d281:     	movl	$0x101, 0x754(%rbp)     # imm = 0x101
14001d28b:     	movzbl	0x678(%rbp), %eax
14001d292:     	movb	%al, 0x758(%rbp)
14001d298:     	movzbl	0x588(%rbp), %eax
14001d29f:     	movb	%al, 0x759(%rbp)
14001d2a5:     	movw	$0x300, 0x75a(%rbp)     # imm = 0x300
14001d2ae:     	movb	$0x2, 0x770(%rbp)
14001d2b5:     	movq	0x440(%rbp), %rax
14001d2bc:     	movq	0x458(%rbp), %rsi
14001d2c3:     	cmpq	%rax, %rsi
14001d2c6:     	jne	0x14001d2e9 <.text+0x1c2e9>
14001d2c8:     	leaq	0x440(%rbp), %rcx
14001d2cf:     	leaq	0x29095b2(%rip), %rdx   # 0x142926888
14001d2d6:     	callq	0x1405114a0 <.text+0x5104a0>
14001d2db:     	movq	0x440(%rbp), %rax
14001d2e2:     	movq	0x458(%rbp), %rsi
14001d2e9:     	movq	0x450(%rbp), %rcx
14001d2f0:     	addq	%rsi, %rcx
14001d2f3:     	cmpq	%rax, %rcx
14001d2f6:     	movl	$0x0, %edx
14001d2fb:     	cmovaeq	%rax, %rdx
14001d2ff:     	subq	%rdx, %rcx
14001d302:     	imulq	$0xb8, %rcx, %rcx
14001d309:     	addq	0x448(%rbp), %rcx
14001d310:     	movl	$0xb8, %r8d
14001d316:     	leaq	0x6c0(%rbp), %rdx
14001d31d:     	callq	0x142923c10 <.text+0x2922c10>
14001d322:     	incq	%rsi
14001d325:     	movq	%rsi, 0x458(%rbp)
14001d32c:     	movq	0x7a0(%rbp), %rdx
14001d333:     	testq	%rdx, %rdx
14001d336:     	je	0x14001d34e <.text+0x1c34e>
14001d338:     	shlq	$0x2, %rdx
14001d33c:     	movl	$0x4, %r8d
14001d342:     	movq	0x808(%rbp), %rcx
14001d349:     	callq	0x140613c20 <.text+0x612c20>
14001d34e:     	movq	0x7e0(%rbp), %r14
14001d355:     	movq	0x560(%rbp), %r8
14001d35c:     	movq	0x698(%rbp), %rdi
14001d363:     	jmp	0x140018efc <.text+0x17efc>
14001d368:     	movl	0x324(%rbp), %eax
14001d36e:     	movq	0x110(%rbp), %rcx
14001d375:     	movl	%eax, (%rcx)
14001d377:     	movq	0x368(%rbp), %rdx
14001d37e:     	movl	0x1c(%rdx), %eax
14001d381:     	movq	0x10(%rdx), %rcx
14001d385:     	movl	%eax, (%rcx)
14001d387:     	movq	(%rdx), %rdx
14001d38a:     	movl	0x128(%r14), %r9d
14001d391:     	movl	0x12c(%r14), %eax
14001d398:     	movb	$0x1, 0x817(%rbp)
14001d39f:     	movl	%eax, 0x20(%rsp)
14001d3a3:     	leaq	0x6c0(%rbp), %rcx
14001d3aa:     	movq	0xa8(%rbp), %r8
14001d3b1:     	callq	0x140252ad0 <.text+0x251ad0>
14001d3b6:     	cmpb	$0x11, 0x6e0(%rbp)
14001d3bd:     	je	0x14001d3df <.text+0x1c3df>
14001d3bf:     	movq	0x6f8(%rbp), %rax
14001d3c6:     	lock
14001d3c7:     	decq	(%rax)
14001d3ca:     	jne	0x14001d3df <.text+0x1c3df>
14001d3cc:     	movb	$0x1, 0x817(%rbp)
14001d3d3:     	leaq	0x6f8(%rbp), %rcx
14001d3da:     	callq	0x140b5c620 <.text+0xb5b620>
14001d3df:     	movq	0x368(%rbp), %rdx
14001d3e6:     	movl	0x1c(%rdx), %eax
14001d3e9:     	movq	0x10(%rdx), %rcx
14001d3ed:     	movl	%eax, (%rcx)
14001d3ef:     	movq	(%rdx), %rdx
14001d3f2:     	movq	0x7e0(%rbp), %rax
14001d3f9:     	movl	0x128(%rax), %r9d
14001d400:     	movl	0x12c(%rax), %eax
14001d406:     	movb	$0x1, 0x817(%rbp)
14001d40d:     	movl	%eax, 0x20(%rsp)
14001d411:     	leaq	0x6c0(%rbp), %rcx
14001d418:     	movq	0xb0(%rbp), %r8
14001d41f:     	callq	0x140252ad0 <.text+0x251ad0>
14001d424:     	cmpb	$0x11, 0x6e0(%rbp)
14001d42b:     	je	0x14001d44d <.text+0x1c44d>
14001d42d:     	movq	0x6f8(%rbp), %rax
14001d434:     	lock
14001d435:     	decq	(%rax)
14001d438:     	jne	0x14001d44d <.text+0x1c44d>
14001d43a:     	movb	$0x1, 0x817(%rbp)
14001d441:     	leaq	0x6f8(%rbp), %rcx
14001d448:     	callq	0x140b5c620 <.text+0xb5b620>
14001d44d:     	cmpq	$0x0, 0x288(%rbp)
14001d455:     	je	0x14001d55e <.text+0x1c55e>
14001d45b:     	movq	0x288(%rbp), %rax
14001d462:     	movq	%rax, 0x470(%rbp)
14001d469:     	movups	0x278(%rbp), %xmm0
14001d470:     	movaps	%xmm0, 0x460(%rbp)
14001d477:     	movaps	0x2909152(%rip), %xmm0  # 0x1429265d0
14001d47e:     	movaps	%xmm0, 0x5a0(%rbp)
14001d485:     	movaps	0x2909134(%rip), %xmm0  # 0x1429265c0
14001d48c:     	movaps	%xmm0, 0x590(%rbp)
14001d493:     	movb	$0x0, 0x817(%rbp)
14001d49a:     	leaq	0x6d0(%rbp), %rcx
14001d4a1:     	leaq	0x460(%rbp), %rdx
14001d4a8:     	leaq	0x590(%rbp), %r8
14001d4af:     	callq	0x1408d0230 <.text+0x8cf230>
14001d4b4:     	movq	$0x1, 0x6c0(%rbp)
14001d4bf:     	movq	$0x1, 0x6c8(%rbp)
14001d4ca:     	movzbl	0x3617738(%rip), %eax   # 0x143634c09
14001d4d1:     	movl	$0x90, %ecx
14001d4d6:     	movl	$0x10, %edx
14001d4db:     	callq	0x140613c10 <.text+0x612c10>
14001d4e0:     	testq	%rax, %rax
14001d4e3:     	je	0x140020b34 <.text+0x1fb34>
14001d4e9:     	movq	%rax, %rsi
14001d4ec:     	movl	$0x90, %r8d
14001d4f2:     	movq	%rax, %rcx
14001d4f5:     	leaq	0x6c0(%rbp), %rdi
14001d4fc:     	movq	%rdi, %rdx
14001d4ff:     	callq	0x142923c00 <.text+0x2922c00>
14001d504:     	movq	0x670(%rbp), %rax
14001d50b:     	movq	(%rax), %rax
14001d50e:     	movq	%rax, 0x6c4(%rbp)
14001d515:     	movb	$0xd, 0x6c0(%rbp)
14001d51c:     	movb	$0x0, 0x817(%rbp)
14001d523:     	leaq	0x508(%rbp), %rcx
14001d52a:     	movq	%rsi, %rdx
14001d52d:     	movq	%rdi, %r8
14001d530:     	callq	0x1408ceb10 <.text+0x8cdb10>
14001d535:     	xorl	%esi, %esi
14001d537:     	leaq	0x590(%rbp), %r12
14001d53e:     	movq	0x7e0(%rbp), %rbx
14001d545:     	cmpb	$0x11, 0x508(%rbp)
14001d54c:     	leaq	0x508(%rbp), %rdi
14001d553:     	cmoveq	%rsi, %rdi
14001d557:     	jne	0x14001d5b6 <.text+0x1c5b6>
14001d559:     	jmp	0x14001dff8 <.text+0x1cff8>
14001d55e:     	movq	0x278(%rbp), %rdx
14001d565:     	movq	0x280(%rbp), %rcx
14001d56c:     	movb	$0x11, 0x508(%rbp)
14001d573:     	testq	%rdx, %rdx
14001d576:     	movl	$0x0, %esi
14001d57b:     	leaq	0x590(%rbp), %r12
14001d582:     	movq	0x7e0(%rbp), %rbx
14001d589:     	je	0x14001dff8 <.text+0x1cff8>
14001d58f:     	shlq	$0x7, %rdx
14001d593:     	movl	$0x10, %r8d
14001d599:     	callq	0x140613c20 <.text+0x612c20>
14001d59e:     	cmpb	$0x11, 0x508(%rbp)
14001d5a5:     	leaq	0x508(%rbp), %rdi
14001d5ac:     	cmoveq	%rsi, %rdi
14001d5b0:     	je	0x14001dff8 <.text+0x1cff8>
14001d5b6:     	movb	$0x1, 0x805(%rbp)
14001d5bd:     	xorl	%ecx, %ecx
14001d5bf:     	callq	0x1400d9b20 <.text+0xd8b20>
14001d5c4:     	testq	%rax, %rax
14001d5c7:     	je	0x140020b48 <.text+0x1fb48>
14001d5cd:     	movups	(%rax), %xmm0
14001d5d0:     	incq	(%rax)
14001d5d3:     	movaps	%xmm0, 0x480(%rbp)
14001d5da:     	movups	0x2992a3f(%rip), %xmm0  # 0x1429b0020
14001d5e1:     	movaps	%xmm0, 0x460(%rbp)
14001d5e8:     	movups	0x2992a41(%rip), %xmm0  # 0x1429b0030
14001d5ef:     	movaps	%xmm0, 0x470(%rbp)
14001d5f6:     	movq	0x368(%rbp), %rax
14001d5fd:     	movq	(%rax), %rbx
14001d600:     	movsd	0x2908358(%rip), %xmm0  # 0x142925960
14001d608:     	movsd	%xmm0, 0x6c0(%rbp)
14001d610:     	movl	$0x0, 0x6c8(%rbp)
14001d61a:     	movb	$0x1, 0x7ef(%rbp)
14001d621:     	movq	%r12, %rcx
14001d624:     	leaq	0x6c0(%rbp), %rdx
14001d62b:     	callq	0x140c91b30 <.text+0xc90b30>
14001d630:     	movb	$0x1, 0x7ef(%rbp)
14001d637:     	leaq	0x508(%rbp), %rcx
14001d63e:     	callq	0x1408ceb30 <.text+0x8cdb30>
14001d643:     	movaps	0x590(%rbp), %xmm0
14001d64a:     	movaps	0x5a0(%rbp), %xmm1
14001d651:     	movaps	%xmm1, 0x6d0(%rbp)
14001d658:     	movaps	%xmm0, 0x6c0(%rbp)
14001d65f:     	movq	%rax, 0x6e0(%rbp)
14001d666:     	movb	$0x1, 0x7ef(%rbp)
14001d66d:     	leaq	0x460(%rbp), %rax
14001d674:     	movq	%rax, 0x20(%rsp)
14001d679:     	movq	%rbx, %rcx
14001d67c:     	leaq	0x6c0(%rbp), %rdx
14001d683:     	movl	$0x2, %r8d
14001d689:     	xorl	%r9d, %r9d
14001d68c:     	callq	0x140254aa0 <.text+0x253aa0>
14001d691:     	movq	0x460(%rbp), %r14
14001d698:     	movq	0x468(%rbp), %rcx
14001d69f:     	testq	%rcx, %rcx
14001d6a2:     	movq	%rcx, 0x6a0(%rbp)
14001d6a9:     	je	0x14001d6e0 <.text+0x1c6e0>
14001d6ab:     	leaq	0x17(,%rcx,8), %rax
14001d6b3:     	andq	$-0x10, %rax
14001d6b7:     	addq	%rax, %rcx
14001d6ba:     	addq	$0x11, %rcx
14001d6be:     	movq	%rcx, 0x780(%rbp)
14001d6c5:     	movq	%r14, %rcx
14001d6c8:     	subq	%rax, %rcx
14001d6cb:     	movq	%rcx, 0x538(%rbp)
14001d6d2:     	movl	$0x10, %eax
14001d6d7:     	movq	%rax, 0x648(%rbp)
14001d6de:     	jmp	0x14001d6eb <.text+0x1c6eb>
14001d6e0:     	movq	$0x0, 0x648(%rbp)
14001d6eb:     	movq	0x478(%rbp), %rax
14001d6f2:     	movq	%rax, 0x560(%rbp)
14001d6f9:     	movdqa	(%r14), %xmm0
14001d6fe:     	leaq	0x10(%r14), %r13
14001d702:     	pmovmskb	%xmm0, %eax
14001d706:     	notl	%eax
14001d708:     	jmp	0x14001d716 <.text+0x1c716>
14001d70a:     	nopw	(%rax,%rax)
14001d710:     	movl	0x698(%rbp), %eax
14001d716:     	cmpq	$0x0, 0x560(%rbp)
14001d71e:     	je	0x14001dfc1 <.text+0x1cfc1>
14001d724:     	testw	%ax, %ax
14001d727:     	je	0x14001d740 <.text+0x1c740>
14001d729:     	leal	-0x1(%rax), %ecx
14001d72c:     	andl	%eax, %ecx
14001d72e:     	movl	%ecx, 0x698(%rbp)
14001d734:     	jmp	0x14001d76c <.text+0x1c76c>
14001d736:     	nopw	%cs:(%rax,%rax)
14001d740:     	movdqa	(%r13), %xmm0
14001d746:     	pmovmskb	%xmm0, %eax
14001d74a:     	addq	$-0x80, %r14
14001d74e:     	addq	$0x10, %r13
14001d752:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
14001d757:     	je	0x14001d740 <.text+0x1c740>
14001d759:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14001d75e:     	subl	%eax, %ecx
14001d760:     	notl	%eax
14001d762:     	movl	%eax, %edx
14001d764:     	andl	%ecx, %edx
14001d766:     	movl	%edx, 0x698(%rbp)
14001d76c:     	tzcntl	%eax, %eax
14001d770:     	shll	$0x3, %eax
14001d773:     	movq	%r14, %rcx
14001d776:     	subq	%rax, %rcx
14001d779:     	movq	-0x8(%rcx), %rdx
14001d77d:     	movq	0xe8(%rbp), %rcx
14001d784:     	movq	%rdx, 0x790(%rbp)
14001d78b:     	leaq	0x2908dfe(%rip), %r8    # 0x142926590
14001d792:     	callq	0x140a91ef0 <.text+0xa90ef0>
14001d797:     	movq	%rax, %r15
14001d79a:     	decq	0x560(%rbp)
14001d7a1:     	movl	0x698(%rbp), %eax
14001d7a7:     	testq	%r15, %r15
14001d7aa:     	je	0x14001d716 <.text+0x1c716>
14001d7b0:     	movq	0xe8(%rbp), %rcx
14001d7b7:     	movq	0x790(%rbp), %rdx
14001d7be:     	callq	0x140a922b0 <.text+0xa912b0>
14001d7c3:     	movq	%rax, %rcx
14001d7c6:     	shrq	$0x20, %rcx
14001d7ca:     	je	0x14001d8c0 <.text+0x1c8c0>
14001d7d0:     	movl	%eax, %eax
14001d7d2:     	movq	0x250(%rbp), %rdx
14001d7d9:     	cmpq	0x10(%rdx), %rax
14001d7dd:     	jae	0x14001d8c0 <.text+0x1c8c0>
14001d7e3:     	movq	0x250(%rbp), %rdx
14001d7ea:     	movq	0x8(%rdx), %rdx
14001d7ee:     	leaq	(%rax,%rax,4), %rax
14001d7f2:     	cmpl	%ecx, (%rdx,%rax,4)
14001d7f5:     	jne	0x14001d8c0 <.text+0x1c8c0>
14001d7fb:     	leaq	(%rdx,%rax,4), %rax
14001d7ff:     	movl	0x4(%rax), %ecx
14001d802:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14001d807:     	cmpq	%rdx, %rcx
14001d80a:     	je	0x14001d8c0 <.text+0x1c8c0>
14001d810:     	movq	0x260(%rbp), %rdx
14001d817:     	cmpq	%rcx, 0x38(%rdx)
14001d81b:     	jbe	0x14001d8c0 <.text+0x1c8c0>
14001d821:     	movq	0x260(%rbp), %rdx
14001d828:     	movq	0x28(%rdx), %rdx
14001d82c:     	movl	%ecx, %r8d
14001d82f:     	shrl	$0x6, %r8d
14001d833:     	movq	(%rdx,%r8,8), %rdx
14001d837:     	btq	%rcx, %rdx
14001d83b:     	jae	0x14001d8c0 <.text+0x1c8c0>
14001d841:     	movl	0xc(%rax), %ecx
14001d844:     	movq	0x250(%rbp), %rdx
14001d84b:     	movq	0x1a8(%rdx), %rdx
14001d852:     	leaq	(%rcx,%rcx,8), %r8
14001d856:     	movq	0x260(%rbp), %rcx
14001d85d:     	movq	0x110(%rcx), %rcx
14001d864:     	cmpq	%rcx, 0x40(%rdx,%r8,8)
14001d869:     	jbe	0x14001d892 <.text+0x1c892>
14001d86b:     	leaq	(%rdx,%r8,8), %rdx
14001d86f:     	movq	0x38(%rdx), %r8
14001d873:     	movq	(%r8,%rcx,8), %rcx
14001d877:     	testq	%rcx, %rcx
14001d87a:     	je	0x14001d892 <.text+0x1c892>
14001d87c:     	movq	0x18(%rdx), %rdx
14001d880:     	notq	%rcx
14001d883:     	leaq	(%rcx,%rcx,2), %rcx
14001d887:     	shlq	$0x4, %rcx
14001d88b:     	movq	0x10(%rdx,%rcx), %rcx
14001d890:     	jmp	0x14001d894 <.text+0x1c894>
14001d892:     	xorl	%ecx, %ecx
14001d894:     	movl	0x10(%rax), %eax
14001d897:     	imulq	$0x58, %rax, %rax
14001d89b:     	movl	0x40(%rcx,%rax), %edx
14001d89f:     	xorl	%esi, %esi
14001d8a1:     	cmpb	$0x3, %dl
14001d8a4:     	cmovel	%esi, %edx
14001d8a7:     	cmpb	$0x2, %dl
14001d8aa:     	je	0x14001dfb5 <.text+0x1cfb5>
14001d8b0:     	movzbl	%dl, %eax
14001d8b3:     	cmpl	$0x1, %eax
14001d8b6:     	je	0x14001d8ce <.text+0x1c8ce>
14001d8b8:     	nopl	(%rax,%rax)
14001d8c0:     	leaq	0x299(%r15), %rcx
14001d8c7:     	callq	0x1408c5670 <.text+0x8c4670>
14001d8cc:     	movl	%eax, %esi
14001d8ce:     	movq	%r15, %rcx
14001d8d1:     	callq	0x140b2a600 <.text+0xb29600>
14001d8d6:     	movq	%rax, %r12
14001d8d9:     	movl	0x698(%rbp), %eax
14001d8df:     	testq	%r12, %r12
14001d8e2:     	je	0x14001d716 <.text+0x1c716>
14001d8e8:     	testb	$0x1, (%r12)
14001d8ed:     	je	0x14001d979 <.text+0x1c979>
14001d8f3:     	movq	%r15, %rcx
14001d8f6:     	callq	0x140b2a6e0 <.text+0xb296e0>
14001d8fb:     	leaq	0x4(%r12), %rdx
14001d900:     	addss	0x2907ff8(%rip), %xmm0  # 0x142925900
14001d908:     	leaq	0x6c0(%rbp), %rcx
14001d90f:     	movaps	%xmm0, %xmm2
14001d912:     	callq	0x140c98b90 <.text+0xc97b90>
14001d917:     	movzbl	0x36172eb(%rip), %eax   # 0x143634c09
14001d91e:     	movl	$0x30, %ecx
14001d923:     	movl	$0x4, %edx
14001d928:     	callq	0x140613c10 <.text+0x612c10>
14001d92d:     	testq	%rax, %rax
14001d930:     	je	0x1400209d3 <.text+0x1f9d3>
14001d936:     	movups	0x6c0(%rbp), %xmm0
14001d93d:     	movups	0x6d0(%rbp), %xmm1
14001d944:     	movups	0x6e0(%rbp), %xmm2
14001d94b:     	movups	%xmm2, 0x20(%rax)
14001d94f:     	movups	%xmm1, 0x10(%rax)
14001d953:     	movq	%rax, 0x7a0(%rbp)
14001d95a:     	movups	%xmm0, (%rax)
14001d95d:     	leaq	0xc(%r12), %rbx
14001d962:     	addq	$0x10, %r12
14001d966:     	movl	$0x4, %eax
14001d96b:     	movq	%rax, 0x808(%rbp)
14001d972:     	movl	$0x4, %ecx
14001d977:     	jmp	0x14001d9dd <.text+0x1c9dd>
14001d979:     	movq	%r15, %rcx
14001d97c:     	callq	0x140b2a6e0 <.text+0xb296e0>
14001d981:     	leaq	0x4(%r12), %rbx
14001d986:     	addss	0x2907f72(%rip), %xmm0  # 0x142925900
14001d98e:     	leaq	0x6c0(%rbp), %rcx
14001d995:     	movq	%rbx, %rdx
14001d998:     	movss	0x2907fd0(%rip), %xmm2  # 0x142925970
14001d9a0:     	movaps	%xmm0, %xmm3
14001d9a3:     	callq	0x140c97c50 <.text+0xc96c50>
14001d9a8:     	leaq	0x50(%rbp), %rcx
14001d9ac:     	leaq	0x6c0(%rbp), %rdx
14001d9b3:     	leaq	0x29c06ce(%rip), %r8    # 0x1429de088
14001d9ba:     	callq	0x1402bd6b0 <.text+0x2bc6b0>
14001d9bf:     	addq	$0x8, %r12
14001d9c3:     	movq	0x50(%rbp), %rax
14001d9c7:     	movq	%rax, 0x808(%rbp)
14001d9ce:     	movq	0x58(%rbp), %rax
14001d9d2:     	movq	%rax, 0x7a0(%rbp)
14001d9d9:     	movq	0x60(%rbp), %rcx
14001d9dd:     	movl	%esi, 0x6b0(%rbp)
14001d9e3:     	movss	(%r12), %xmm0
14001d9e9:     	movss	(%rbx), %xmm1
14001d9ed:     	movss	%xmm1, 0x680(%rbp)
14001d9f5:     	movss	%xmm0, 0x684(%rbp)
14001d9fd:     	leaq	(,%rcx,4), %rax
14001da05:     	leaq	(%rax,%rax,2), %rbx
14001da09:     	movq	%rcx, 0x7f0(%rbp)
14001da10:     	testq	%rcx, %rcx
14001da13:     	je	0x14001da37 <.text+0x1ca37>
14001da15:     	movzbl	0x36171ed(%rip), %eax   # 0x143634c09
14001da1c:     	movl	$0x4, %edx
14001da21:     	movq	%rbx, %rcx
14001da24:     	callq	0x140613c10 <.text+0x612c10>
14001da29:     	testq	%rax, %rax
14001da2c:     	je	0x1400209a1 <.text+0x1f9a1>
14001da32:     	movq	%rax, %rsi
14001da35:     	jmp	0x14001da3c <.text+0x1ca3c>
14001da37:     	movl	$0x4, %esi
14001da3c:     	movq	0x7a0(%rbp), %r12
14001da43:     	leaq	(%r12,%rbx), %rax
14001da47:     	movq	%rax, 0x7d0(%rbp)
14001da4e:     	movq	%rsi, %rcx
14001da51:     	movq	%r12, %rdx
14001da54:     	movq	%rbx, %r8
14001da57:     	callq	0x142923c00 <.text+0x2922c00>
14001da5c:     	addq	%rsi, %rbx
14001da5f:     	movq	0x7f0(%rbp), %rax
14001da66:     	movq	%rax, 0x6c0(%rbp)
14001da6d:     	movq	%rsi, 0x6c8(%rbp)
14001da74:     	movq	%rsi, 0x6d0(%rbp)
14001da7b:     	movq	%rax, 0x6d8(%rbp)
14001da82:     	movq	%rbx, 0x6e0(%rbp)
14001da89:     	movq	%r12, 0x6e8(%rbp)
14001da90:     	movq	%r12, 0x6f0(%rbp)
14001da97:     	movq	0x808(%rbp), %rax
14001da9e:     	movq	%rax, 0x6f8(%rbp)
14001daa5:     	movq	0x7d0(%rbp), %rax
14001daac:     	movq	%rax, 0x700(%rbp)
14001dab3:     	movl	$0x0, 0x708(%rbp)
14001dabd:     	movss	0x2907eaf(%rip), %xmm0  # 0x142925974
14001dac5:     	movss	%xmm0, 0x558(%rbp)
14001dacd:     	testb	$0x1, 0x6b0(%rbp)
14001dad4:     	jne	0x14001dae6 <.text+0x1cae6>
14001dad6:     	movss	0x2907e9a(%rip), %xmm0  # 0x142925978
14001dade:     	movss	%xmm0, 0x558(%rbp)
14001dae6:     	xorps	%xmm0, %xmm0
14001dae9:     	jmp	0x14001db44 <.text+0x1cb44>
14001daeb:     	nopl	(%rax,%rax)
14001daf0:     	movss	0x7d8(%rbp), %xmm0
14001daf8:     	subss	%xmm7, %xmm0
14001dafc:     	movq	0x8(%r12), %rax
14001db01:     	leaq	(%rsi,%rsi,2), %rcx
14001db05:     	movq	0x790(%rbp), %rdx
14001db0c:     	movq	%rdx, (%rax,%rcx,8)
14001db10:     	movss	%xmm12, 0x8(%rax,%rcx,8)
14001db17:     	movss	%xmm11, 0xc(%rax,%rcx,8)
14001db1e:     	movss	%xmm0, 0x10(%rax,%rcx,8)
14001db24:     	movl	$0x41200000, 0x14(%rax,%rcx,8) # imm = 0x41200000
14001db2c:     	incq	%rsi
14001db2f:     	movq	%rsi, 0x10(%r12)
14001db34:     	movss	0x7f0(%rbp), %xmm0
14001db3c:     	addss	0x7d0(%rbp), %xmm0
14001db44:     	movss	%xmm0, 0x7f0(%rbp)
14001db4c:     	leaq	0x380(%rbp), %rcx
14001db53:     	leaq	0x6c0(%rbp), %rdx
14001db5a:     	callq	0x1404c2650 <.text+0x4c1650>
14001db5f:     	testb	$0x1, 0x380(%rbp)
14001db66:     	je	0x14001df60 <.text+0x1cf60>
14001db6c:     	movss	0x38c(%rbp), %xmm0
14001db74:     	movss	%xmm0, 0x808(%rbp)
14001db7c:     	movsd	0x384(%rbp), %xmm0
14001db84:     	movaps	%xmm0, 0x7a0(%rbp)
14001db8b:     	movsd	0x390(%rbp), %xmm0
14001db93:     	movaps	%xmm0, 0x6b0(%rbp)
14001db9a:     	movss	0x398(%rbp), %xmm8
14001dba3:     	leaq	0x590(%rbp), %rcx
14001dbaa:     	leaq	0x680(%rbp), %rdx
14001dbb1:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14001dbb6:     	movss	0x808(%rbp), %xmm11
14001dbbf:     	addss	%xmm8, %xmm11
14001dbc4:     	mulss	0x2907ca3(%rip), %xmm11 # 0x142925870
14001dbcd:     	subss	0x598(%rbp), %xmm11
14001dbd6:     	movaps	%xmm11, %xmm1
14001dbda:     	mulss	%xmm11, %xmm1
14001dbdf:     	movaps	0x7a0(%rbp), %xmm0
14001dbe6:     	addps	0x6b0(%rbp), %xmm0
14001dbed:     	mulps	0x2907cec(%rip), %xmm0  # 0x1429258e0
14001dbf4:     	movsd	0x590(%rbp), %xmm2
14001dbfc:     	subps	%xmm2, %xmm0
14001dbff:     	movaps	%xmm0, %xmm2
14001dc02:     	mulps	%xmm0, %xmm2
14001dc05:     	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14001dc09:     	addss	%xmm2, %xmm3
14001dc0d:     	addss	%xmm1, %xmm3
14001dc11:     	xorps	%xmm2, %xmm2
14001dc14:     	sqrtss	%xmm3, %xmm2
14001dc18:     	movss	0x2907ce4(%rip), %xmm1  # 0x142925904
14001dc20:     	divss	%xmm2, %xmm1
14001dc24:     	movd	%xmm1, %eax
14001dc28:     	testl	%eax, %eax
14001dc2a:     	setns	%cl
14001dc2d:     	movl	%eax, %edx
14001dc2f:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
14001dc35:     	addl	$0xff800000, %edx       # imm = 0xFF800000
14001dc3b:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
14001dc41:     	setb	%dl
14001dc44:     	andb	%cl, %dl
14001dc46:     	decl	%eax
14001dc48:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
14001dc4d:     	setb	%al
14001dc50:     	orb	%dl, %al
14001dc52:     	jne	0x14001dc60 <.text+0x1cc60>
14001dc54:     	xorps	%xmm11, %xmm11
14001dc58:     	jmp	0x14001dc65 <.text+0x1cc65>
14001dc5a:     	nopw	(%rax,%rax)
14001dc60:     	mulss	%xmm1, %xmm11
14001dc65:     	leaq	0x590(%rbp), %rcx
14001dc6c:     	movaps	0x6b0(%rbp), %xmm4
14001dc73:     	movaps	%xmm4, %xmm9
14001dc77:     	movaps	0x7a0(%rbp), %xmm12
14001dc7f:     	subps	%xmm12, %xmm9
14001dc83:     	movaps	%xmm8, %xmm6
14001dc87:     	movss	0x808(%rbp), %xmm10
14001dc90:     	subss	%xmm10, %xmm6
14001dc95:     	movaps	%xmm9, %xmm2
14001dc99:     	mulps	%xmm9, %xmm2
14001dc9d:     	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14001dca1:     	addss	%xmm2, %xmm3
14001dca5:     	movaps	%xmm6, %xmm2
14001dca8:     	mulss	%xmm6, %xmm2
14001dcac:     	addss	%xmm3, %xmm2
14001dcb0:     	sqrtss	%xmm2, %xmm2
14001dcb4:     	movss	%xmm2, 0x7d0(%rbp)
14001dcbc:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14001dcc0:     	mulps	%xmm1, %xmm0
14001dcc3:     	movzbl	%al, %eax
14001dcc6:     	movd	%eax, %xmm1
14001dcca:     	pshufd	$0x50, %xmm1, %xmm14    # xmm14 = xmm1[0,0,1,1]
14001dcd0:     	pslld	$0x1f, %xmm14
14001dcd6:     	psrad	$0x1f, %xmm14
14001dcdc:     	pand	%xmm0, %xmm14
14001dce1:     	movq	$0x2, 0x298(%rbp)
14001dcec:     	movabsq	$0x3e99999a00000000, %rax # imm = 0x3E99999A00000000
14001dcf6:     	movq	%rax, 0x2a0(%rbp)
14001dcfd:     	subps	%xmm4, %xmm12
14001dd01:     	subss	%xmm8, %xmm10
14001dd06:     	xorl	%esi, %esi
14001dd08:     	movss	0x2907c70(%rip), %xmm7  # 0x142925980
14001dd10:     	movss	0x2907c64(%rip), %xmm0  # 0x14292597c
14001dd18:     	movss	%xmm0, 0x7d8(%rbp)
14001dd20:     	movq	%rsi, %rax
14001dd23:     	cmpq	$0x2, %rax
14001dd27:     	je	0x14001dec0 <.text+0x1cec0>
14001dd2d:     	nopl	(%rax)
14001dd30:     	leaq	0x1(%rax), %rsi
14001dd34:     	movq	%rsi, 0x290(%rbp)
14001dd3b:     	movss	0x2a0(%rbp,%rax,4), %xmm15
14001dd45:     	movsldup	%xmm15, %xmm13          # xmm13 = xmm15[0,0,2,2]
14001dd4a:     	mulps	%xmm14, %xmm13
14001dd4e:     	mulss	%xmm11, %xmm15
14001dd53:     	movss	0x808(%rbp), %xmm0
14001dd5b:     	addss	%xmm15, %xmm0
14001dd60:     	movaps	0x7a0(%rbp), %xmm1
14001dd67:     	addps	%xmm13, %xmm1
14001dd6b:     	movlps	%xmm1, 0x3e0(%rbp)
14001dd72:     	movss	%xmm0, 0x3e8(%rbp)
14001dd7a:     	movlps	%xmm9, 0x3ec(%rbp)
14001dd82:     	movss	%xmm6, 0x3f4(%rbp)
14001dd8a:     	movl	$0x3f800000, 0x20(%rsp) # imm = 0x3F800000
14001dd92:     	movq	%rdi, %rdx
14001dd95:     	leaq	0x2908824(%rip), %r8    # 0x1429265c0
14001dd9c:     	leaq	0x3e0(%rbp), %r9
14001dda3:     	callq	0x1408ceb40 <.text+0x8cdb40>
14001dda8:     	cmpl	$0x2, 0x5c4(%rbp)
14001ddaf:     	je	0x14001dde4 <.text+0x1cde4>
14001ddb1:     	movss	0x5c0(%rbp), %xmm0
14001ddb9:     	mulss	0x7d0(%rbp), %xmm0
14001ddc1:     	addss	0x7f0(%rbp), %xmm0
14001ddc9:     	movaps	%xmm7, %xmm1
14001ddcc:     	cmpunordss	%xmm7, %xmm1
14001ddd1:     	movaps	%xmm1, %xmm2
14001ddd4:     	andps	%xmm0, %xmm2
14001ddd7:     	minss	%xmm7, %xmm0
14001dddb:     	andnps	%xmm0, %xmm1
14001ddde:     	orps	%xmm2, %xmm1
14001dde1:     	movaps	%xmm1, %xmm7
14001dde4:     	leaq	0x590(%rbp), %rcx
14001ddeb:     	addss	%xmm8, %xmm15
14001ddf0:     	addps	0x6b0(%rbp), %xmm13
14001ddf8:     	movlps	%xmm13, 0x3e0(%rbp)
14001de00:     	movss	%xmm15, 0x3e8(%rbp)
14001de09:     	movlps	%xmm12, 0x3ec(%rbp)
14001de11:     	movss	%xmm10, 0x3f4(%rbp)
14001de1a:     	movl	$0x3f800000, 0x20(%rsp) # imm = 0x3F800000
14001de22:     	movq	%rdi, %rdx
14001de25:     	leaq	0x2908794(%rip), %r8    # 0x1429265c0
14001de2c:     	leaq	0x3e0(%rbp), %r9
14001de33:     	callq	0x1408ceb40 <.text+0x8cdb40>
14001de38:     	cmpl	$0x2, 0x5c4(%rbp)
14001de3f:     	jne	0x14001de57 <.text+0x1ce57>
14001de41:     	movq	%rsi, %rax
14001de44:     	leaq	0x590(%rbp), %rcx
14001de4b:     	cmpq	$0x2, %rax
14001de4f:     	jne	0x14001dd30 <.text+0x1cd30>
14001de55:     	jmp	0x14001dec0 <.text+0x1cec0>
14001de57:     	movss	0x2907aa5(%rip), %xmm0  # 0x142925904
14001de5f:     	subss	0x5c0(%rbp), %xmm0
14001de67:     	mulss	0x7d0(%rbp), %xmm0
14001de6f:     	addss	0x7f0(%rbp), %xmm0
14001de77:     	movss	0x7d8(%rbp), %xmm3
14001de7f:     	movaps	%xmm3, %xmm1
14001de82:     	cmpunordss	%xmm3, %xmm1
14001de87:     	movaps	%xmm1, %xmm2
14001de8a:     	andps	%xmm0, %xmm2
14001de8d:     	maxss	%xmm3, %xmm0
14001de91:     	andnps	%xmm0, %xmm1
14001de94:     	orps	%xmm2, %xmm1
14001de97:     	movss	%xmm1, 0x7d8(%rbp)
14001de9f:     	leaq	0x590(%rbp), %rcx
14001dea6:     	movq	%rsi, %rax
14001dea9:     	cmpq	$0x2, %rax
14001dead:     	jne	0x14001dd30 <.text+0x1cd30>
14001deb3:     	nopw	%cs:(%rax,%rax)
14001dec0:     	movaps	%xmm7, %xmm0
14001dec3:     	addss	0x2907ab9(%rip), %xmm0  # 0x142925984
14001decb:     	movss	0x7d8(%rbp), %xmm1
14001ded3:     	ucomiss	%xmm0, %xmm1
14001ded6:     	jbe	0x14001db34 <.text+0x1cb34>
14001dedc:     	movq	%r15, %rcx
14001dedf:     	callq	0x140b2a620 <.text+0xb29620>
14001dee4:     	movaps	%xmm0, %xmm11
14001dee8:     	movq	0x208(%rbp), %r12
14001deef:     	movq	%r15, %rcx
14001def2:     	callq	0x140b2a740 <.text+0xb29740>
14001def7:     	subss	0x558(%rbp), %xmm11
14001df00:     	addss	0x2907a7f(%rip), %xmm11 # 0x142925988
14001df09:     	movaps	%xmm7, %xmm1
14001df0c:     	addss	0x7d8(%rbp), %xmm1
14001df14:     	mulss	0x2907954(%rip), %xmm1  # 0x142925870
14001df1c:     	movl	%eax, 0x590(%rbp)
14001df22:     	movss	%xmm0, 0x594(%rbp)
14001df2a:     	leaq	0x590(%rbp), %rcx
14001df31:     	callq	0x140c91eb0 <.text+0xc90eb0>
14001df36:     	movaps	%xmm0, %xmm12
14001df3a:     	movq	0x10(%r12), %rsi
14001df3f:     	cmpq	(%r12), %rsi
14001df43:     	jne	0x14001daf0 <.text+0x1caf0>
14001df49:     	movq	%r12, %rcx
14001df4c:     	leaq	0x2908655(%rip), %rdx   # 0x1429265a8
14001df53:     	callq	0x140562db0 <.text+0x561db0>
14001df58:     	jmp	0x14001daf0 <.text+0x1caf0>
14001df5d:     	nopl	(%rax)
14001df60:     	movq	0x6d8(%rbp), %rax
14001df67:     	testq	%rax, %rax
14001df6a:     	je	0x14001df86 <.text+0x1cf86>
14001df6c:     	movq	0x6c8(%rbp), %rcx
14001df73:     	shlq	$0x2, %rax
14001df77:     	leaq	(%rax,%rax,2), %rdx
14001df7b:     	movl	$0x4, %r8d
14001df81:     	callq	0x140613c20 <.text+0x612c20>
14001df86:     	movq	0x6f8(%rbp), %rax
14001df8d:     	testq	%rax, %rax
14001df90:     	je	0x14001d710 <.text+0x1c710>
14001df96:     	movq	0x6e8(%rbp), %rcx
14001df9d:     	shlq	$0x2, %rax
14001dfa1:     	leaq	(%rax,%rax,2), %rdx
14001dfa5:     	movl	$0x4, %r8d
14001dfab:     	callq	0x140613c20 <.text+0x612c20>
14001dfb0:     	jmp	0x14001d710 <.text+0x1c710>
14001dfb5:     	addq	%rax, %rcx
14001dfb8:     	movzbl	0x41(%rcx), %esi
14001dfbc:     	jmp	0x14001d8ce <.text+0x1c8ce>
14001dfc1:     	cmpq	$0x0, 0x6a0(%rbp)
14001dfc9:     	leaq	0x590(%rbp), %r12
14001dfd0:     	movq	0x780(%rbp), %rdx
14001dfd7:     	je	0x14001dff1 <.text+0x1cff1>
14001dfd9:     	testq	%rdx, %rdx
14001dfdc:     	je	0x14001dff1 <.text+0x1cff1>
14001dfde:     	movq	0x538(%rbp), %rcx
14001dfe5:     	movq	0x648(%rbp), %r8
14001dfec:     	callq	0x140613c20 <.text+0x612c20>
14001dff1:     	movq	0x7e0(%rbp), %rbx
14001dff8:     	movq	0x368(%rbp), %rdx
14001dfff:     	movl	0x1c(%rdx), %eax
14001e002:     	movq	0x10(%rdx), %rcx
14001e006:     	movl	%eax, (%rcx)
14001e008:     	movq	(%rdx), %rcx
14001e00b:     	movl	0x324(%rbp), %eax
14001e011:     	movq	0x110(%rbp), %rdx
14001e018:     	movl	%eax, (%rdx)
14001e01a:     	movzbl	0x508(%rbp), %eax
14001e021:     	movq	0x509(%rbp), %rdx
14001e028:     	movl	0x511(%rbp), %r8d
14001e02f:     	movzwl	0x515(%rbp), %r9d
14001e037:     	movzbl	0x517(%rbp), %r10d
14001e03f:     	movups	0x518(%rbp), %xmm0
14001e046:     	movaps	%xmm0, 0x6d0(%rbp)
14001e04d:     	movb	%al, 0x6c0(%rbp)
14001e053:     	movq	%rdx, 0x6c1(%rbp)
14001e05a:     	movl	%r8d, 0x6c9(%rbp)
14001e061:     	movw	%r9w, 0x6cd(%rbp)
14001e069:     	movb	%r10b, 0x6cf(%rbp)
14001e070:     	movl	0x128(%rbx), %r9d
14001e077:     	movl	0x12c(%rbx), %eax
14001e07d:     	movb	$0x0, 0x805(%rbp)
14001e084:     	movl	%eax, 0x20(%rsp)
14001e088:     	movq	0xb8(%rbp), %rdx
14001e08f:     	leaq	0x6c0(%rbp), %r8
14001e096:     	callq	0x1407deb60 <.text+0x7ddb60>
14001e09b:     	movq	0x458(%rbp), %rax
14001e0a2:     	testq	%rax, %rax
14001e0a5:     	movq	0x940(%rbp), %r13
14001e0ac:     	je	0x14001e10a <.text+0x1d10a>
14001e0ae:     	movq	0x440(%rbp), %rcx
14001e0b5:     	movq	0x450(%rbp), %rdx
14001e0bc:     	leaq	0x1(%rdx), %r8
14001e0c0:     	cmpq	%rcx, %r8
14001e0c3:     	movl	$0x0, %r8d
14001e0c9:     	cmovaeq	%rcx, %r8
14001e0cd:     	negq	%r8
14001e0d0:     	leaq	(%rdx,%r8), %rcx
14001e0d4:     	incq	%rcx
14001e0d7:     	movq	%rcx, 0x450(%rbp)
14001e0de:     	decq	%rax
14001e0e1:     	movq	%rax, 0x458(%rbp)
14001e0e8:     	imulq	$0xb8, %rdx, %rdx
14001e0ef:     	addq	0x448(%rbp), %rdx
14001e0f6:     	movl	$0xb8, %r8d
14001e0fc:     	leaq	0x6c0(%rbp), %rcx
14001e103:     	callq	0x142923c00 <.text+0x2922c00>
14001e108:     	jmp	0x14001e140 <.text+0x1d140>
14001e10a:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14001e114:     	movq	%rax, 0x6c0(%rbp)
14001e11b:     	jmp	0x14001e140 <.text+0x1d140>
14001e11d:     	imulq	$0xa8, %rsi, %rcx
14001e124:     	addq	0x8(%rdi), %rcx
14001e128:     	movl	$0xa8, %r8d
14001e12e:     	movq	%r12, %rdx
14001e131:     	callq	0x142923c10 <.text+0x2922c10>
14001e136:     	incq	%rsi
14001e139:     	movq	%rsi, 0x10(%rdi)
14001e13d:     	nopl	(%rax)
14001e140:     	movq	0x6c0(%rbp), %rcx
14001e147:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14001e151:     	movq	%rax, 0x6c0(%rbp)
14001e158:     	movq	%rcx, 0x808(%rbp)
14001e15f:     	movq	%rcx, %rax
14001e162:     	negq	%rax
14001e165:     	jo	0x14001f9cb <.text+0x1e9cb>
14001e16b:     	movq	0x6c8(%rbp), %rax
14001e172:     	movq	%rax, 0x6b0(%rbp)
14001e179:     	movq	0x6d0(%rbp), %r14
14001e180:     	movq	0x6d8(%rbp), %r15
14001e187:     	movq	0x6e0(%rbp), %rax
14001e18e:     	movq	%rax, 0x430(%rbp)
14001e195:     	leaq	0x6f8(%rbp), %rcx
14001e19c:     	movdqu	-0x10(%rcx), %xmm0
14001e1a1:     	movdqa	%xmm0, 0x300(%rbp)
14001e1a9:     	movq	0x6f8(%rbp), %r8
14001e1b0:     	movq	0x700(%rbp), %rax
14001e1b7:     	movq	%rax, 0x7f0(%rbp)
14001e1be:     	movq	0x708(%rbp), %rbx
14001e1c5:     	movq	0x710(%rbp), %rax
14001e1cc:     	movq	%rax, 0x7a0(%rbp)
14001e1d3:     	movq	0x718(%rbp), %rax
14001e1da:     	movq	%rax, 0x7d0(%rbp)
14001e1e1:     	movq	0x720(%rbp), %rdi
14001e1e8:     	movq	0x728(%rbp), %rdx
14001e1ef:     	movq	0x730(%rbp), %rax
14001e1f6:     	movq	%rax, 0x7d8(%rbp)
14001e1fd:     	movzwl	0x66(%rcx), %eax
14001e201:     	movw	%ax, 0x56c(%rbp)
14001e208:     	movl	0x62(%rcx), %eax
14001e20b:     	movl	%eax, 0x568(%rbp)
14001e211:     	movq	0x738(%rbp), %rsi
14001e218:     	movss	0x754(%rbp), %xmm6
14001e220:     	movq	0x740(%rbp), %rax
14001e227:     	movq	%rax, 0x678(%rbp)
14001e22e:     	movl	0x748(%rbp), %eax
14001e234:     	movl	%eax, 0x588(%rbp)
14001e23a:     	movl	0x74c(%rbp), %r10d
14001e241:     	movl	0x750(%rbp), %r9d
14001e248:     	movzwl	0x758(%rbp), %r11d
14001e250:     	movq	0x458(%rbp), %rcx
14001e257:     	testq	%rcx, %rcx
14001e25a:     	movq	%r15, 0x558(%rbp)
14001e261:     	movq	%rdx, 0x698(%rbp)
14001e268:     	movq	%r8, 0x790(%rbp)
14001e26f:     	je	0x14001ea07 <.text+0x1da07>
14001e275:     	movl	%r10d, 0x640(%rbp)
14001e27c:     	movl	%r9d, 0x528(%rbp)
14001e283:     	movq	0x450(%rbp), %rdx
14001e28a:     	movq	0x440(%rbp), %r8
14001e291:     	movq	0x448(%rbp), %rax
14001e298:     	cmpq	%r8, %rdx
14001e29b:     	movl	$0x0, %r9d
14001e2a1:     	cmovaeq	%r8, %r9
14001e2a5:     	movq	%rdx, %r10
14001e2a8:     	subq	%r9, %r10
14001e2ab:     	imulq	$0xb8, %r10, %r9
14001e2b2:     	cmpb	$0x2, 0xb0(%rax,%r9)
14001e2bb:     	je	0x14001e96e <.text+0x1d96e>
14001e2c1:     	cmpb	$0x2, 0x770(%rbp)
14001e2c8:     	je	0x14001e96e <.text+0x1d96e>
14001e2ce:     	movq	0x768(%rbp), %r10
14001e2d5:     	addq	%rax, %r9
14001e2d8:     	cmpq	0xa0(%r9), %r10
14001e2df:     	jne	0x14001e96e <.text+0x1d96e>
14001e2e5:     	movq	0x760(%rbp), %r10
14001e2ec:     	movq	%r10, 0x500(%rbp)
14001e2f3:     	movq	0xa8(%r9), %r9
14001e2fa:     	movq	%r9, 0x540(%rbp)
14001e301:     	leaq	0x1(%rdx), %r9
14001e305:     	cmpq	%r8, %r9
14001e308:     	movl	$0x0, %r9d
14001e30e:     	cmovaeq	%r8, %r9
14001e312:     	negq	%r9
14001e315:     	leaq	(%rdx,%r9), %r8
14001e319:     	incq	%r8
14001e31c:     	movq	%r8, 0x450(%rbp)
14001e323:     	decq	%rcx
14001e326:     	movq	%rcx, 0x458(%rbp)
14001e32d:     	imulq	$0xb8, %rdx, %r9
14001e334:     	movq	(%rax,%r9), %rcx
14001e338:     	movq	%rcx, %rdx
14001e33b:     	negq	%rdx
14001e33e:     	jo	0x1400207e1 <.text+0x1f7e1>
14001e344:     	movw	%r11w, 0x548(%rbp)
14001e34c:     	movq	0x8(%rax,%r9), %rdx
14001e351:     	movq	0x10(%rax,%r9), %r8
14001e356:     	movq	0x18(%rax,%r9), %r10
14001e35b:     	movq	%r10, 0x648(%rbp)
14001e362:     	movq	0x20(%rax,%r9), %r10
14001e367:     	movq	%r10, 0x6a8(%rbp)
14001e36e:     	movq	0x38(%rax,%r9), %r10
14001e373:     	movq	%r10, 0x560(%rbp)
14001e37a:     	movq	0x40(%rax,%r9), %r10
14001e37f:     	movq	%r10, 0x6a0(%rbp)
14001e386:     	movq	0x48(%rax,%r9), %r13
14001e38b:     	movq	0x50(%rax,%r9), %r10
14001e390:     	movq	%r10, 0x7e0(%rbp)
14001e397:     	movq	0x58(%rax,%r9), %r10
14001e39c:     	movq	%r10, 0x780(%rbp)
14001e3a3:     	movq	0x60(%rax,%r9), %r12
14001e3a8:     	movq	0x68(%rax,%r9), %r10
14001e3ad:     	movq	%r10, 0x538(%rbp)
14001e3b4:     	movq	0x70(%rax,%r9), %r10
14001e3b9:     	movq	%r10, 0x580(%rbp)
14001e3c0:     	movq	0x78(%rax,%r9), %r15
14001e3c5:     	movq	0x808(%rbp), %rax
14001e3cc:     	movq	%rax, 0x650(%rbp)
14001e3d3:     	movq	0x6b0(%rbp), %rax
14001e3da:     	movq	%rax, 0x658(%rbp)
14001e3e1:     	movq	%r14, 0x660(%rbp)
14001e3e8:     	testq	%r14, %r14
14001e3eb:     	je	0x14001e3f7 <.text+0x1d3f7>
14001e3ed:     	decq	%r14
14001e3f0:     	movq	%r14, 0x660(%rbp)
14001e3f7:     	leaq	(%r8,%r8,2), %rax
14001e3fb:     	leaq	(%rdx,%rax,4), %rax
14001e3ff:     	movq	%rdx, 0x590(%rbp)
14001e406:     	movq	%rdx, 0x598(%rbp)
14001e40d:     	movq	%rcx, 0x5a0(%rbp)
14001e414:     	movq	%rax, 0x5a8(%rbp)
14001e41b:     	movq	$0x1, 0x5b0(%rbp)
14001e426:     	movb	$0x1, 0x7cd(%rbp)
14001e42d:     	movb	$0x1, 0x7cc(%rbp)
14001e434:     	movb	$0x1, 0x7cb(%rbp)
14001e43b:     	movb	$0x1, 0x7ca(%rbp)
14001e442:     	leaq	0x650(%rbp), %rcx
14001e449:     	leaq	0x590(%rbp), %rdx
14001e450:     	leaq	0x29081a1(%rip), %r8    # 0x1429265f8
14001e457:     	callq	0x1402b9e00 <.text+0x2b8e00>
14001e45c:     	movq	0x790(%rbp), %rax
14001e463:     	movq	%rax, 0x680(%rbp)
14001e46a:     	movq	0x7f0(%rbp), %rax
14001e471:     	movq	%rax, 0x688(%rbp)
14001e478:     	movq	%rbx, 0x690(%rbp)
14001e47f:     	testq	%rbx, %rbx
14001e482:     	movl	$0x0, %r14d
14001e488:     	je	0x14001e494 <.text+0x1d494>
14001e48a:     	decq	%rbx
14001e48d:     	movq	%rbx, 0x690(%rbp)
14001e494:     	movq	0x6a0(%rbp), %rcx
14001e49b:     	leaq	(%rcx,%r13,8), %rax
14001e49f:     	movq	%rcx, 0x590(%rbp)
14001e4a6:     	movq	%rcx, 0x598(%rbp)
14001e4ad:     	movq	0x560(%rbp), %rcx
14001e4b4:     	movq	%rcx, 0x5a0(%rbp)
14001e4bb:     	movq	%rax, 0x5a8(%rbp)
14001e4c2:     	movq	$0x1, 0x5b0(%rbp)
14001e4cd:     	movb	$0x1, 0x7c9(%rbp)
14001e4d4:     	movb	$0x1, 0x7c8(%rbp)
14001e4db:     	movb	$0x1, 0x7c7(%rbp)
14001e4e2:     	movb	$0x1, 0x7c6(%rbp)
14001e4e9:     	leaq	0x680(%rbp), %rcx
14001e4f0:     	leaq	0x590(%rbp), %rdx
14001e4f7:     	leaq	0x2908112(%rip), %r8    # 0x142926610
14001e4fe:     	callq	0x1402ba000 <.text+0x2b9000>
14001e503:     	movq	0x7a0(%rbp), %rax
14001e50a:     	movq	%rax, 0x290(%rbp)
14001e511:     	movq	0x7d0(%rbp), %rax
14001e518:     	movq	%rax, 0x298(%rbp)
14001e51f:     	movq	%rdi, 0x2a0(%rbp)
14001e526:     	testq	%rdi, %rdi
14001e529:     	movq	0x940(%rbp), %r13
14001e530:     	je	0x14001e53c <.text+0x1d53c>
14001e532:     	decq	%rdi
14001e535:     	movq	%rdi, 0x2a0(%rbp)
14001e53c:     	movq	0x780(%rbp), %rcx
14001e543:     	leaq	(%rcx,%r12,4), %rax
14001e547:     	movq	%rcx, 0x590(%rbp)
14001e54e:     	movq	%rcx, 0x598(%rbp)
14001e555:     	movq	0x7e0(%rbp), %rcx
14001e55c:     	movq	%rcx, 0x5a0(%rbp)
14001e563:     	movq	%rax, 0x5a8(%rbp)
14001e56a:     	movq	$0x1, 0x5b0(%rbp)
14001e575:     	movb	$0x1, 0x7c5(%rbp)
14001e57c:     	movb	$0x1, 0x7c4(%rbp)
14001e583:     	movb	$0x1, 0x7c3(%rbp)
14001e58a:     	leaq	0x290(%rbp), %rcx
14001e591:     	leaq	0x590(%rbp), %r12
14001e598:     	movq	%r12, %rdx
14001e59b:     	leaq	0x2908086(%rip), %r8    # 0x142926628
14001e5a2:     	callq	0x1402b9c50 <.text+0x2b8c50>
14001e5a7:     	movq	0x698(%rbp), %rax
14001e5ae:     	movq	%rax, 0x3e0(%rbp)
14001e5b5:     	movq	0x7d8(%rbp), %rax
14001e5bc:     	movq	%rax, 0x3e8(%rbp)
14001e5c3:     	movq	%rsi, 0x3f0(%rbp)
14001e5ca:     	testq	%rsi, %rsi
14001e5cd:     	je	0x14001e5d9 <.text+0x1d5d9>
14001e5cf:     	decq	%rsi
14001e5d2:     	movq	%rsi, 0x3f0(%rbp)
14001e5d9:     	movq	0x580(%rbp), %rcx
14001e5e0:     	leaq	(%rcx,%r15,4), %rax
14001e5e4:     	movq	%rcx, 0x590(%rbp)
14001e5eb:     	movq	%rcx, 0x598(%rbp)
14001e5f2:     	movq	0x538(%rbp), %rcx
14001e5f9:     	movq	%rcx, 0x5a0(%rbp)
14001e600:     	movq	%rax, 0x5a8(%rbp)
14001e607:     	movq	$0x1, 0x5b0(%rbp)
14001e612:     	movb	$0x1, 0x7c2(%rbp)
14001e619:     	movb	$0x1, 0x7c1(%rbp)
14001e620:     	leaq	0x3e0(%rbp), %rcx
14001e627:     	movq	%r12, %rdx
14001e62a:     	leaq	0x290800f(%rip), %r8    # 0x142926640
14001e631:     	callq	0x1402b9c50 <.text+0x2b8c50>
14001e636:     	movq	0x660(%rbp), %rax
14001e63d:     	movq	%rax, 0x5a0(%rbp)
14001e644:     	movdqu	0x650(%rbp), %xmm0
14001e64c:     	movdqa	%xmm0, 0x590(%rbp)
14001e654:     	movb	$0x1, 0x7c2(%rbp)
14001e65b:     	movb	$0x0, 0x7c1(%rbp)
14001e662:     	leaq	0x380(%rbp), %rcx
14001e669:     	movq	%r12, %rdx
14001e66c:     	movl	$0x1, %r8d
14001e672:     	callq	0x140153640 <.text+0x152640>
14001e677:     	cmpq	0x380(%rbp), %r14
14001e67e:     	jno	0x14001e710 <.text+0x1d710>
14001e684:     	movq	0x3e0(%rbp), %rdx
14001e68b:     	testq	%rdx, %rdx
14001e68e:     	je	0x14001e6a6 <.text+0x1d6a6>
14001e690:     	movq	0x3e8(%rbp), %rcx
14001e697:     	shlq	$0x2, %rdx
14001e69b:     	movl	$0x4, %r8d
14001e6a1:     	callq	0x140613c20 <.text+0x612c20>
14001e6a6:     	movq	0x290(%rbp), %rdx
14001e6ad:     	testq	%rdx, %rdx
14001e6b0:     	movq	0x558(%rbp), %rsi
14001e6b7:     	movq	0x648(%rbp), %rdi
14001e6be:     	je	0x14001e6d6 <.text+0x1d6d6>
14001e6c0:     	movq	0x298(%rbp), %rcx
14001e6c7:     	shlq	$0x2, %rdx
14001e6cb:     	movl	$0x4, %r8d
14001e6d1:     	callq	0x140613c20 <.text+0x612c20>
14001e6d6:     	movq	0x680(%rbp), %rdx
14001e6dd:     	testq	%rdx, %rdx
14001e6e0:     	je	0x14001e929 <.text+0x1d929>
14001e6e6:     	movq	0x688(%rbp), %rcx
14001e6ed:     	shlq	$0x3, %rdx
14001e6f1:     	movl	$0x4, %r8d
14001e6f7:     	callq	0x140613c20 <.text+0x612c20>
14001e6fc:     	jmp	0x14001e929 <.text+0x1d929>
14001e701:     	nopw	%cs:(%rax,%rax)
14001e710:     	movaps	%xmm6, 0x370(%rbp)
14001e717:     	movq	0x3b0(%rbp), %rax
14001e71e:     	movq	%rax, 0x5c0(%rbp)
14001e725:     	movups	0x380(%rbp), %xmm0
14001e72c:     	movups	0x390(%rbp), %xmm1
14001e733:     	movups	0x3a0(%rbp), %xmm2
14001e73a:     	movaps	%xmm2, 0x5b0(%rbp)
14001e741:     	movaps	%xmm1, 0x5a0(%rbp)
14001e748:     	movaps	%xmm0, 0x590(%rbp)
14001e74f:     	movq	0x690(%rbp), %rax
14001e756:     	leaq	0x598(%rbp), %rcx
14001e75d:     	movq	%rax, 0x40(%rcx)
14001e761:     	movups	0x680(%rbp), %xmm0
14001e768:     	movups	%xmm0, 0x30(%rcx)
14001e76c:     	movq	0x2a0(%rbp), %rax
14001e773:     	movq	%rax, 0x58(%rcx)
14001e777:     	movups	0x290(%rbp), %xmm0
14001e77e:     	movups	%xmm0, 0x48(%rcx)
14001e782:     	movq	0x3f0(%rbp), %rax
14001e789:     	movq	%rax, 0x70(%rcx)
14001e78d:     	movups	0x3e0(%rbp), %xmm0
14001e794:     	movups	%xmm0, 0x60(%rcx)
14001e798:     	movaps	0x590(%rbp), %xmm0
14001e79f:     	movaps	0x5a0(%rbp), %xmm1
14001e7a6:     	movaps	0x5b0(%rbp), %xmm2
14001e7ad:     	movaps	0x5c0(%rbp), %xmm3
14001e7b4:     	movaps	%xmm0, 0x460(%rbp)
14001e7bb:     	movaps	%xmm1, 0x470(%rbp)
14001e7c2:     	movaps	%xmm2, 0x480(%rbp)
14001e7c9:     	movaps	%xmm3, 0x490(%rbp)
14001e7d0:     	movaps	0x5d0(%rbp), %xmm0
14001e7d7:     	movaps	%xmm0, 0x4a0(%rbp)
14001e7de:     	movaps	0x5e0(%rbp), %xmm0
14001e7e5:     	movaps	%xmm0, 0x4b0(%rbp)
14001e7ec:     	movaps	0x5f0(%rbp), %xmm0
14001e7f3:     	movaps	%xmm0, 0x4c0(%rbp)
14001e7fa:     	movaps	0x600(%rbp), %xmm0
14001e801:     	movaps	%xmm0, 0x4d0(%rbp)
14001e808:     	cmpq	0x6c0(%rbp), %r14
14001e80f:     	jo	0x14001e81d <.text+0x1d81d>
14001e811:     	leaq	0x6c0(%rbp), %rcx
14001e818:     	callq	0x140609eb0 <.text+0x608eb0>
14001e81d:     	movaps	0x4d0(%rbp), %xmm0
14001e824:     	movaps	%xmm0, 0x730(%rbp)
14001e82b:     	movaps	0x4c0(%rbp), %xmm0
14001e832:     	movaps	%xmm0, 0x720(%rbp)
14001e839:     	movaps	0x4b0(%rbp), %xmm0
14001e840:     	movaps	%xmm0, 0x710(%rbp)
14001e847:     	movaps	0x4a0(%rbp), %xmm0
14001e84e:     	movaps	%xmm0, 0x700(%rbp)
14001e855:     	movaps	0x460(%rbp), %xmm0
14001e85c:     	movdqa	0x470(%rbp), %xmm1
14001e864:     	movdqa	0x480(%rbp), %xmm2
14001e86c:     	movapd	0x490(%rbp), %xmm3
14001e874:     	movapd	%xmm3, 0x6f0(%rbp)
14001e87c:     	movdqa	%xmm2, 0x6e0(%rbp)
14001e884:     	movdqa	%xmm1, 0x6d0(%rbp)
14001e88c:     	movaps	%xmm0, 0x6c0(%rbp)
14001e893:     	movq	0x678(%rbp), %rax
14001e89a:     	movq	%rax, 0x740(%rbp)
14001e8a1:     	movl	0x588(%rbp), %eax
14001e8a7:     	movl	%eax, 0x748(%rbp)
14001e8ad:     	movl	0x640(%rbp), %eax
14001e8b3:     	movl	%eax, 0x74c(%rbp)
14001e8b9:     	movl	0x528(%rbp), %eax
14001e8bf:     	movl	%eax, 0x750(%rbp)
14001e8c5:     	movdqa	0x370(%rbp), %xmm0
14001e8cd:     	movd	%xmm0, %eax
14001e8d1:     	movw	%ax, 0x754(%rbp)
14001e8d8:     	movw	$0x0, 0x756(%rbp)
14001e8e1:     	movzwl	0x548(%rbp), %eax
14001e8e8:     	movw	%ax, 0x758(%rbp)
14001e8ef:     	movw	$0x300, 0x75a(%rbp)     # imm = 0x300
14001e8f8:     	movq	0x500(%rbp), %rax
14001e8ff:     	movq	%rax, 0x760(%rbp)
14001e906:     	movq	0x540(%rbp), %rax
14001e90d:     	movq	%rax, 0x768(%rbp)
14001e914:     	movb	$0x0, 0x770(%rbp)
14001e91b:     	movq	0x558(%rbp), %rsi
14001e922:     	movq	0x648(%rbp), %rdi
14001e929:     	testq	%rdi, %rdi
14001e92c:     	je	0x14001e947 <.text+0x1d947>
14001e92e:     	shlq	$0x2, %rdi
14001e932:     	movl	$0x4, %r8d
14001e938:     	movq	0x6a8(%rbp), %rcx
14001e93f:     	movq	%rdi, %rdx
14001e942:     	callq	0x140613c20 <.text+0x612c20>
14001e947:     	testq	%rsi, %rsi
14001e94a:     	je	0x14001e140 <.text+0x1d140>
14001e950:     	shlq	$0x2, %rsi
14001e954:     	movl	$0x4, %r8d
14001e95a:     	movq	0x430(%rbp), %rcx
14001e961:     	movq	%rsi, %rdx
14001e964:     	callq	0x140613c20 <.text+0x612c20>
14001e969:     	jmp	0x14001e140 <.text+0x1d140>
14001e96e:     	leaq	0x1(%rdx), %r9
14001e972:     	cmpq	%r8, %r9
14001e975:     	movl	$0x0, %r9d
14001e97b:     	cmovaeq	%r8, %r9
14001e97f:     	negq	%r9
14001e982:     	leaq	(%rdx,%r9), %r8
14001e986:     	incq	%r8
14001e989:     	movq	%r8, 0x450(%rbp)
14001e990:     	decq	%rcx
14001e993:     	movq	%rcx, 0x458(%rbp)
14001e99a:     	imulq	$0xb8, %rdx, %rcx
14001e9a1:     	movq	(%rax,%rcx), %r15
14001e9a5:     	leaq	(%rax,%rcx), %rdx
14001e9a9:     	addq	$0x8, %rdx
14001e9ad:     	movl	$0xb0, %r8d
14001e9b3:     	movq	%r12, %rcx
14001e9b6:     	movl	%r11d, %r13d
14001e9b9:     	callq	0x142923c00 <.text+0x2922c00>
14001e9be:     	movq	%r15, 0x6c0(%rbp)
14001e9c5:     	movq	0x558(%rbp), %r15
14001e9cc:     	movl	$0xb0, %r8d
14001e9d2:     	leaq	0x6c8(%rbp), %rcx
14001e9d9:     	movq	%r12, %rdx
14001e9dc:     	callq	0x142923c00 <.text+0x2922c00>
14001e9e1:     	movl	%r13d, %r11d
14001e9e4:     	movq	0x940(%rbp), %r13
14001e9eb:     	movq	0x698(%rbp), %rdx
14001e9f2:     	movq	0x790(%rbp), %r8
14001e9f9:     	movl	0x528(%rbp), %r9d
14001ea00:     	movl	0x640(%rbp), %r10d
14001ea07:     	movq	0x808(%rbp), %rax
14001ea0e:     	movq	%rax, 0x460(%rbp)
14001ea15:     	movq	0x6b0(%rbp), %rax
14001ea1c:     	movq	%rax, 0x468(%rbp)
14001ea23:     	movq	%r14, 0x470(%rbp)
14001ea2a:     	movq	%r15, 0x478(%rbp)
14001ea31:     	movq	0x430(%rbp), %rax
14001ea38:     	movq	%rax, 0x480(%rbp)
14001ea3f:     	movaps	0x300(%rbp), %xmm0
14001ea46:     	leaq	0x468(%rbp), %rcx
14001ea4d:     	movups	%xmm0, 0x20(%rcx)
14001ea51:     	movq	%r8, 0x498(%rbp)
14001ea58:     	movq	0x7f0(%rbp), %rax
14001ea5f:     	movq	%rax, 0x4a0(%rbp)
14001ea66:     	movq	%rbx, 0x4a8(%rbp)
14001ea6d:     	movq	0x7a0(%rbp), %rax
14001ea74:     	movq	%rax, 0x4b0(%rbp)
14001ea7b:     	movq	0x7d0(%rbp), %r8
14001ea82:     	movq	%r8, 0x4b8(%rbp)
14001ea89:     	movq	%rdi, 0x4c0(%rbp)
14001ea90:     	movq	%rdx, 0x4c8(%rbp)
14001ea97:     	movq	0x7d8(%rbp), %rax
14001ea9e:     	movq	%rax, 0x4d0(%rbp)
14001eaa5:     	movq	%rsi, 0x4d8(%rbp)
14001eaac:     	movq	0x678(%rbp), %rax
14001eab3:     	movq	%rax, 0x4e0(%rbp)
14001eaba:     	movl	0x588(%rbp), %eax
14001eac0:     	movl	%eax, 0x4e8(%rbp)
14001eac6:     	movl	%r10d, 0x4ec(%rbp)
14001eacd:     	movl	%r9d, 0x4f0(%rbp)
14001ead4:     	movss	%xmm6, 0x4f4(%rbp)
14001eadc:     	movw	%r11w, 0x4f8(%rbp)
14001eae4:     	movzwl	0x56c(%rbp), %eax
14001eaeb:     	movw	%ax, 0x96(%rcx)
14001eaf2:     	movl	0x568(%rbp), %eax
14001eaf8:     	movl	%eax, 0x92(%rcx)
14001eafe:     	testq	%rdi, %rdi
14001eb01:     	je	0x140020958 <.text+0x1f958>
14001eb07:     	leaq	(%r8,%rdi,4), %rax
14001eb0b:     	addq	$-0x4, %rax
14001eb0f:     	je	0x140020970 <.text+0x1f970>
14001eb15:     	cmpb	$0x0, 0x530(%rbp)
14001eb1c:     	je	0x14001ec06 <.text+0x1dc06>
14001eb22:     	movq	0x258(%rbp), %rax
14001eb29:     	movl	0x424(%rbp), %ecx
14001eb2f:     	movl	%ecx, (%rax)
14001eb31:     	movq	0x490(%rbp), %rax
14001eb38:     	movq	%rax, 0x5c0(%rbp)
14001eb3f:     	movups	0x460(%rbp), %xmm0
14001eb46:     	movups	0x470(%rbp), %xmm1
14001eb4d:     	movups	0x480(%rbp), %xmm2
14001eb54:     	movaps	%xmm2, 0x5b0(%rbp)
14001eb5b:     	movaps	%xmm1, 0x5a0(%rbp)
14001eb62:     	movaps	%xmm0, 0x590(%rbp)
14001eb69:     	leaq	0x468(%rbp), %rdx
14001eb70:     	movq	0x40(%rdx), %rax
14001eb74:     	leaq	0x598(%rbp), %rcx
14001eb7b:     	movq	%rax, 0x40(%rcx)
14001eb7f:     	movups	0x30(%rdx), %xmm0
14001eb83:     	movups	%xmm0, 0x30(%rcx)
14001eb87:     	movq	0x58(%rdx), %rax
14001eb8b:     	movq	%rax, 0x70(%rcx)
14001eb8f:     	movups	0x48(%rdx), %xmm0
14001eb93:     	movups	%xmm0, 0x60(%rcx)
14001eb97:     	movq	0x70(%rdx), %rax
14001eb9b:     	movq	%rax, 0x58(%rcx)
14001eb9f:     	movups	0x60(%rdx), %xmm0
14001eba3:     	movups	%xmm0, 0x48(%rcx)
14001eba7:     	movq	0x788(%rbp), %rax
14001ebae:     	movl	%eax, 0x630(%rbp)
14001ebb4:     	movq	0x670(%rbp), %rax
14001ebbb:     	movq	(%rax), %rax
14001ebbe:     	movq	%rax, 0x628(%rbp)
14001ebc5:     	movw	%r11w, 0x634(%rbp)
14001ebcd:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14001ebd7:     	movq	%rax, 0x610(%rbp)
14001ebde:     	movq	0x338(%rbp), %rdi
14001ebe5:     	movq	0x10(%rdi), %rsi
14001ebe9:     	cmpq	(%rdi), %rsi
14001ebec:     	jne	0x14001e11d <.text+0x1d11d>
14001ebf2:     	movq	%rdi, %rcx
14001ebf5:     	leaq	0x2907b94(%rip), %rdx   # 0x142926790
14001ebfc:     	callq	0x1405636c0 <.text+0x5626c0>
14001ec01:     	jmp	0x14001e11d <.text+0x1d11d>
14001ec06:     	movss	(%r8), %xmm0
14001ec0b:     	movss	(%rax), %xmm1
14001ec0f:     	movaps	%xmm0, %xmm2
14001ec12:     	cmpunordss	%xmm0, %xmm2
14001ec17:     	movaps	%xmm2, %xmm3
14001ec1a:     	andps	%xmm1, %xmm3
14001ec1d:     	maxss	%xmm0, %xmm1
14001ec21:     	andnps	%xmm1, %xmm2
14001ec24:     	orps	%xmm3, %xmm2
14001ec27:     	movss	0x2906ca1(%rip), %xmm0  # 0x1429258d0
14001ec2f:     	ucomiss	%xmm2, %xmm0
14001ec32:     	jb	0x14001ec6d <.text+0x1dc6d>
14001ec34:     	movb	$0x1, 0x814(%rbp)
14001ec3b:     	movq	0x2d8(%rbp), %rcx
14001ec42:     	callq	0x140959ef0 <.text+0x958ef0>
14001ec47:     	movq	%rax, %rdi
14001ec4a:     	movb	$0x1, 0x814(%rbp)
14001ec51:     	movq	0x118(%rbp), %rcx
14001ec58:     	callq	0x140959ef0 <.text+0x958ef0>
14001ec5d:     	testq	%rdi, %rdi
14001ec60:     	sete	%cl
14001ec63:     	testq	%rax, %rax
14001ec66:     	sete	%dl
14001ec69:     	orb	%cl, %dl
14001ec6b:     	je	0x14001ecc7 <.text+0x1dcc7>
14001ec6d:     	movq	0x330(%rbp), %rax
14001ec74:     	movl	0x534(%rbp), %ecx
14001ec7a:     	movl	%ecx, (%rax)
14001ec7c:     	movl	$0xa0, %r8d
14001ec82:     	movq	%r12, %rcx
14001ec85:     	leaq	0x460(%rbp), %rdx
14001ec8c:     	callq	0x142923c00 <.text+0x2922c00>
14001ec91:     	movq	0x360(%rbp), %rdi
14001ec98:     	movq	0x10(%rdi), %rsi
14001ec9c:     	cmpq	(%rdi), %rsi
14001ec9f:     	jne	0x14001ecb0 <.text+0x1dcb0>
14001eca1:     	movq	%rdi, %rcx
14001eca4:     	leaq	0x2907acd(%rip), %rdx   # 0x142926778
14001ecab:     	callq	0x140563480 <.text+0x562480>
14001ecb0:     	leaq	(%rsi,%rsi,4), %rcx
14001ecb4:     	shlq	$0x5, %rcx
14001ecb8:     	addq	0x8(%rdi), %rcx
14001ecbc:     	movl	$0xa0, %r8d
14001ecc2:     	jmp	0x14001e12e <.text+0x1d12e>
14001ecc7:     	movq	(%rdi), %rcx
14001ecca:     	cmpq	(%rax), %rcx
14001eccd:     	jne	0x14001ecdc <.text+0x1dcdc>
14001eccf:     	movzwl	0x4f8(%rbp), %r11d
14001ecd7:     	jmp	0x14001eb22 <.text+0x1db22>
14001ecdc:     	movb	$0x1, 0x814(%rbp)
14001ece3:     	movq	%rdi, 0x20(%rsp)
14001ece8:     	movq	%r12, %rcx
14001eceb:     	movq	0x368(%rbp), %rdx
14001ecf2:     	movq	0x938(%rbp), %r8
14001ecf9:     	leaq	0x460(%rbp), %r9
14001ed00:     	callq	0x1407d94a0 <.text+0x7d84a0>
14001ed05:     	movq	0x590(%rbp), %rax
14001ed0c:     	movq	%rax, 0x648(%rbp)
14001ed13:     	movq	0x598(%rbp), %r13
14001ed1a:     	movq	0x5a0(%rbp), %rcx
14001ed21:     	movq	0x5a8(%rbp), %rax
14001ed28:     	movq	%rax, 0x640(%rbp)
14001ed2f:     	movq	0x5b0(%rbp), %rax
14001ed36:     	movq	%rax, 0x540(%rbp)
14001ed3d:     	movq	0x5b8(%rbp), %rax
14001ed44:     	movq	%rax, 0x538(%rbp)
14001ed4b:     	movq	0x5c0(%rbp), %rax
14001ed52:     	movq	%rax, 0x528(%rbp)
14001ed59:     	movq	0x5c8(%rbp), %rax
14001ed60:     	movq	%rax, 0x6a8(%rbp)
14001ed67:     	movq	0x5d0(%rbp), %rax
14001ed6e:     	movq	%rax, 0x500(%rbp)
14001ed75:     	shlq	$0x4, %rcx
14001ed79:     	addq	%r13, %rcx
14001ed7c:     	movq	%rcx, 0x7e0(%rbp)
14001ed83:     	movq	%r13, 0x580(%rbp)
14001ed8a:     	nopw	(%rax,%rax)
14001ed90:     	movq	%r13, %r12
14001ed93:     	cmpq	0x7e0(%rbp), %r12
14001ed9a:     	movq	%r12, 0x560(%rbp)
14001eda1:     	je	0x14001f263 <.text+0x1e263>
14001eda7:     	movq	0x8(%r12), %r14
14001edac:     	cmpq	$-0x1, %r14
14001edb0:     	je	0x1400204c1 <.text+0x1f4c1>
14001edb6:     	movq	(%r12), %r13
14001edba:     	leaq	0x1(%r14), %rax
14001edbe:     	movq	%rax, %rdi
14001edc1:     	subq	%r13, %rdi
14001edc4:     	movq	0x80(%rbp), %r15
14001edcb:     	jb	0x1400204d2 <.text+0x1f4d2>
14001edd1:     	movq	0x470(%rbp), %rdx
14001edd8:     	cmpq	%rdx, %r14
14001eddb:     	jae	0x1400204e9 <.text+0x1f4e9>
14001ede1:     	movq	%rax, 0x780(%rbp)
14001ede8:     	movq	%rdi, %rax
14001edeb:     	movl	$0xc, %ecx
14001edf0:     	mulq	%rcx
14001edf3:     	jo	0x140020988 <.text+0x1f988>
14001edf9:     	movl	$0x0, %esi
14001edfe:     	movabsq	$0x7ffffffffffffffc, %rcx # imm = 0x7FFFFFFFFFFFFFFC
14001ee08:     	cmpq	%rcx, %rax
14001ee0b:     	ja	0x14002098a <.text+0x1f98a>
14001ee11:     	movq	0x468(%rbp), %rbx
14001ee18:     	testq	%rax, %rax
14001ee1b:     	je	0x14001ee50 <.text+0x1de50>
14001ee1d:     	movzbl	0x3615de5(%rip), %ecx   # 0x143634c09
14001ee24:     	movl	$0x4, %esi
14001ee29:     	movl	$0x4, %edx
14001ee2e:     	movq	%rax, %rcx
14001ee31:     	movq	%rax, %r15
14001ee34:     	callq	0x140613c10 <.text+0x612c10>
14001ee39:     	testq	%rax, %rax
14001ee3c:     	je	0x14002098a <.text+0x1f98a>
14001ee42:     	movq	%rdi, %rsi
14001ee45:     	movq	%rax, %r15
14001ee48:     	jmp	0x14001ee58 <.text+0x1de58>
14001ee4a:     	nopw	(%rax,%rax)
14001ee50:     	movl	$0x4, %r15d
14001ee56:     	xorl	%esi, %esi
14001ee58:     	movq	%r13, 0x6a0(%rbp)
14001ee5f:     	leaq	(,%r13,2), %rax
14001ee67:     	addq	%r13, %rax
14001ee6a:     	leaq	(%rbx,%rax,4), %rdx
14001ee6e:     	leaq	(,%rdi,4), %r12
14001ee76:     	leaq	(%r12,%r12,2), %r8
14001ee7a:     	movq	%r15, %rcx
14001ee7d:     	callq	0x142923c00 <.text+0x2922c00>
14001ee82:     	movq	%rsi, 0x380(%rbp)
14001ee89:     	movq	%r15, 0x388(%rbp)
14001ee90:     	movq	%rdi, 0x390(%rbp)
14001ee97:     	leaq	0x590(%rbp), %rcx
14001ee9e:     	leaq	0x380(%rbp), %rdx
14001eea5:     	movl	$0x1, %r8d
14001eeab:     	callq	0x140153640 <.text+0x152640>
14001eeb0:     	movq	%r15, 0x80(%rbp)
14001eeb7:     	movq	0x560(%rbp), %rax
14001eebe:     	leaq	0x10(%rax), %r13
14001eec2:     	movq	0x590(%rbp), %rax
14001eec9:     	movq	%rax, %rcx
14001eecc:     	negq	%rcx
14001eecf:     	jo	0x14001ed90 <.text+0x1dd90>
14001eed5:     	movq	0x598(%rbp), %rcx
14001eedc:     	movq	0x5a8(%rbp), %r15
14001eee3:     	movq	0x5b0(%rbp), %rbx
14001eeea:     	movss	0x5c0(%rbp), %xmm0
14001eef2:     	ucomiss	0x2906a93(%rip), %xmm0  # 0x14292598c
14001eef9:     	ja	0x14001ef39 <.text+0x1df39>
14001eefb:     	testq	%rax, %rax
14001eefe:     	je	0x14001ef13 <.text+0x1df13>
14001ef00:     	shlq	$0x2, %rax
14001ef04:     	leaq	(%rax,%rax,2), %rdx
14001ef08:     	movl	$0x4, %r8d
14001ef0e:     	callq	0x140613c20 <.text+0x612c20>
14001ef13:     	movq	%r13, %r12
14001ef16:     	testq	%r15, %r15
14001ef19:     	je	0x14001ed93 <.text+0x1dd93>
14001ef1f:     	shlq	$0x2, %r15
14001ef23:     	movl	$0x4, %r8d
14001ef29:     	movq	%rbx, %rcx
14001ef2c:     	movq	%r15, %rdx
14001ef2f:     	callq	0x140613c20 <.text+0x612c20>
14001ef34:     	jmp	0x14001ed90 <.text+0x1dd90>
14001ef39:     	movq	0x5a0(%rbp), %rdx
14001ef40:     	movq	0x5b8(%rbp), %r8
14001ef47:     	movl	0x5c4(%rbp), %r9d
14001ef4e:     	movq	0x330(%rbp), %r10
14001ef55:     	movl	0x534(%rbp), %r11d
14001ef5c:     	movl	%r11d, (%r10)
14001ef5f:     	movq	%rax, 0x380(%rbp)
14001ef66:     	movq	%rcx, 0x388(%rbp)
14001ef6d:     	movq	%rdx, 0x390(%rbp)
14001ef74:     	movq	%r15, 0x398(%rbp)
14001ef7b:     	movq	%rbx, 0x3a0(%rbp)
14001ef82:     	movq	%r8, 0x3a8(%rbp)
14001ef89:     	movss	%xmm0, 0x3b0(%rbp)
14001ef91:     	movl	%r9d, 0x3b4(%rbp)
14001ef98:     	movq	0x4a8(%rbp), %rdx
14001ef9f:     	cmpq	%rdx, %r14
14001efa2:     	jae	0x1400206ec <.text+0x1f6ec>
14001efa8:     	movq	%rdi, %rax
14001efab:     	shrq	$0x3c, %rax
14001efaf:     	jne	0x140020c00 <.text+0x1fc00>
14001efb5:     	movq	0x6a0(%rbp), %rax
14001efbc:     	leaq	(,%rax,8), %r15
14001efc4:     	addq	0x4a0(%rbp), %r15
14001efcb:     	leaq	(,%rdi,8), %rbx
14001efd3:     	testq	%rdi, %rdi
14001efd6:     	je	0x14001f003 <.text+0x1e003>
14001efd8:     	movzbl	0x3615c2a(%rip), %eax   # 0x143634c09
14001efdf:     	movl	$0x4, %esi
14001efe4:     	movl	$0x4, %edx
14001efe9:     	movq	%rbx, %rcx
14001efec:     	callq	0x140613c10 <.text+0x612c10>
14001eff1:     	testq	%rax, %rax
14001eff4:     	je	0x140020c9d <.text+0x1fc9d>
14001effa:     	movq	%rdi, 0x588(%rbp)
14001f001:     	jmp	0x14001f013 <.text+0x1e013>
14001f003:     	movl	$0x4, %eax
14001f008:     	movq	$0x0, 0x588(%rbp)
14001f013:     	movq	%rax, 0x2e8(%rbp)
14001f01a:     	movq	%rax, %rcx
14001f01d:     	movq	%r15, %rdx
14001f020:     	movq	%rbx, %r8
14001f023:     	callq	0x142923c00 <.text+0x2922c00>
14001f028:     	movq	0x4c0(%rbp), %rdx
14001f02f:     	cmpq	%rdx, %r14
14001f032:     	jae	0x140020704 <.text+0x1f704>
14001f038:     	movq	0x6a0(%rbp), %rax
14001f03f:     	leaq	(,%rax,4), %rsi
14001f047:     	addq	0x4b8(%rbp), %rsi
14001f04e:     	testq	%rdi, %rdi
14001f051:     	je	0x14001f0cb <.text+0x1e0cb>
14001f053:     	movzbl	0x3615baf(%rip), %eax   # 0x143634c09
14001f05a:     	movl	$0x4, %edx
14001f05f:     	movq	%r12, %rcx
14001f062:     	callq	0x140613c10 <.text+0x612c10>
14001f067:     	testq	%rax, %rax
14001f06a:     	je	0x140020ce6 <.text+0x1fce6>
14001f070:     	movq	%rax, %rbx
14001f073:     	movq	%rax, %rcx
14001f076:     	movq	%rsi, %rdx
14001f079:     	movq	%r12, %r8
14001f07c:     	callq	0x142923c00 <.text+0x2922c00>
14001f081:     	movq	0x4d8(%rbp), %rdx
14001f088:     	cmpq	%rdx, %r14
14001f08b:     	jae	0x140020817 <.text+0x1f817>
14001f091:     	movq	%rbx, %rsi
14001f094:     	movq	0x4d0(%rbp), %rbx
14001f09b:     	movzbl	0x3615b67(%rip), %eax   # 0x143634c09
14001f0a2:     	movl	$0x4, %edx
14001f0a7:     	movq	%r12, %rcx
14001f0aa:     	callq	0x140613c10 <.text+0x612c10>
14001f0af:     	testq	%rax, %rax
14001f0b2:     	je	0x140020cbf <.text+0x1fcbf>
14001f0b8:     	movq	%rax, %r15
14001f0bb:     	movq	%rdi, 0x780(%rbp)
14001f0c2:     	movq	%rsi, 0x678(%rbp)
14001f0c9:     	jmp	0x14001f114 <.text+0x1e114>
14001f0cb:     	movl	$0x4, %ebx
14001f0d0:     	movl	$0x4, %ecx
14001f0d5:     	movq	%rsi, %rdx
14001f0d8:     	movq	%r12, %r8
14001f0db:     	callq	0x142923c00 <.text+0x2922c00>
14001f0e0:     	movq	0x4d8(%rbp), %rdx
14001f0e7:     	cmpq	%rdx, %r14
14001f0ea:     	jae	0x140020815 <.text+0x1f815>
14001f0f0:     	movl	$0x4, %eax
14001f0f5:     	movq	%rax, 0x678(%rbp)
14001f0fc:     	movq	0x4d0(%rbp), %rbx
14001f103:     	movq	$0x0, 0x780(%rbp)
14001f10e:     	movl	$0x4, %r15d
14001f114:     	movq	0x6a0(%rbp), %rsi
14001f11b:     	leaq	(%rbx,%rsi,4), %rdx
14001f11f:     	movq	%r15, %rcx
14001f122:     	movq	%r12, %r8
14001f125:     	callq	0x142923c00 <.text+0x2922c00>
14001f12a:     	movq	0x470(%rbp), %rax
14001f131:     	decq	%rax
14001f134:     	testq	%rsi, %rsi
14001f137:     	movups	0x380(%rbp), %xmm0
14001f13e:     	movups	0x390(%rbp), %xmm1
14001f145:     	movups	0x3a0(%rbp), %xmm2
14001f14c:     	movaps	%xmm0, 0x590(%rbp)
14001f153:     	movaps	%xmm1, 0x5a0(%rbp)
14001f15a:     	movaps	%xmm2, 0x5b0(%rbp)
14001f161:     	movq	0x3b0(%rbp), %rcx
14001f168:     	movq	%rcx, 0x5c0(%rbp)
14001f16f:     	movq	0x588(%rbp), %rcx
14001f176:     	movq	%rcx, 0x5c8(%rbp)
14001f17d:     	movq	0x2e8(%rbp), %rcx
14001f184:     	movq	%rcx, 0x5d0(%rbp)
14001f18b:     	movq	%rdi, 0x5d8(%rbp)
14001f192:     	movq	0x780(%rbp), %rcx
14001f199:     	movq	%rcx, 0x5e0(%rbp)
14001f1a0:     	movq	0x678(%rbp), %rdx
14001f1a7:     	movq	%rdx, 0x5e8(%rbp)
14001f1ae:     	movq	%rdi, 0x5f0(%rbp)
14001f1b5:     	movq	%rcx, 0x5f8(%rbp)
14001f1bc:     	movq	%r15, 0x600(%rbp)
14001f1c3:     	movq	%rdi, 0x608(%rbp)
14001f1ca:     	movq	0x438(%rbp), %rcx
14001f1d1:     	movq	%rcx, 0x610(%rbp)
14001f1d8:     	movw	$0x101, 0x624(%rbp)     # imm = 0x101
14001f1e1:     	movq	0x788(%rbp), %rcx
14001f1e8:     	movl	%ecx, 0x620(%rbp)
14001f1ee:     	movq	0x670(%rbp), %rcx
14001f1f5:     	movq	(%rcx), %rcx
14001f1f8:     	setne	0x626(%rbp)
14001f1ff:     	cmpq	%rax, %r14
14001f202:     	movq	%rcx, 0x618(%rbp)
14001f209:     	setne	0x627(%rbp)
14001f210:     	movl	$0x3000000, 0x628(%rbp) # imm = 0x3000000
14001f21a:     	movq	0x360(%rbp), %rdi
14001f221:     	movq	0x10(%rdi), %rsi
14001f225:     	cmpq	(%rdi), %rsi
14001f228:     	jne	0x14001f239 <.text+0x1e239>
14001f22a:     	movq	%rdi, %rcx
14001f22d:     	leaq	0x290752c(%rip), %rdx   # 0x142926760
14001f234:     	callq	0x140563480 <.text+0x562480>
14001f239:     	leaq	(%rsi,%rsi,4), %rcx
14001f23d:     	shlq	$0x5, %rcx
14001f241:     	addq	0x8(%rdi), %rcx
14001f245:     	movl	$0xa0, %r8d
14001f24b:     	leaq	0x590(%rbp), %rdx
14001f252:     	callq	0x142923c10 <.text+0x2922c10>
14001f257:     	incq	%rsi
14001f25a:     	movq	%rsi, 0x10(%rdi)
14001f25e:     	jmp	0x14001ed90 <.text+0x1dd90>
14001f263:     	movq	0x648(%rbp), %rdx
14001f26a:     	testq	%rdx, %rdx
14001f26d:     	movq	0x500(%rbp), %rsi
14001f274:     	je	0x14001f28c <.text+0x1e28c>
14001f276:     	shlq	$0x4, %rdx
14001f27a:     	movl	$0x8, %r8d
14001f280:     	movq	0x580(%rbp), %rcx
14001f287:     	callq	0x140613c20 <.text+0x612c20>
14001f28c:     	movq	0x538(%rbp), %rdx
14001f293:     	shlq	$0x4, %rdx
14001f297:     	movq	0x540(%rbp), %rcx
14001f29e:     	addq	%rcx, %rdx
14001f2a1:     	leaq	(%rsi,%rsi,2), %rax
14001f2a5:     	movq	0x6a8(%rbp), %r13
14001f2ac:     	leaq	(,%rax,8), %r8
14001f2b4:     	addq	%r13, %r8
14001f2b7:     	xorps	%xmm0, %xmm0
14001f2ba:     	leaq	0x3c0(%rbp), %rax
14001f2c1:     	movups	%xmm0, (%rax)
14001f2c4:     	movq	$0x0, 0x10(%rax)
14001f2cc:     	movq	%rcx, 0x380(%rbp)
14001f2d3:     	movq	%rcx, 0x388(%rbp)
14001f2da:     	movq	0x640(%rbp), %rax
14001f2e1:     	movq	%rax, 0x390(%rbp)
14001f2e8:     	movq	%rdx, 0x538(%rbp)
14001f2ef:     	movq	%rdx, 0x398(%rbp)
14001f2f6:     	movq	%r13, 0x3a0(%rbp)
14001f2fd:     	movq	%r13, 0x3a8(%rbp)
14001f304:     	movq	0x528(%rbp), %rax
14001f30b:     	movq	%rax, 0x3b0(%rbp)
14001f312:     	movq	%r8, 0x580(%rbp)
14001f319:     	movq	%r8, 0x3b8(%rbp)
14001f320:     	movq	%r13, %r12
14001f323:     	movq	%rcx, %r14
14001f326:     	nopw	%cs:(%rax,%rax)
14001f330:     	movq	%r14, %rax
14001f333:     	cmpq	0x538(%rbp), %rax
14001f33a:     	je	0x14001f842 <.text+0x1e842>
14001f340:     	leaq	0x10(%rax), %r13
14001f344:     	movq	%r13, 0x388(%rbp)
14001f34b:     	cmpq	0x580(%rbp), %r12
14001f352:     	je	0x14001f83b <.text+0x1e83b>
14001f358:     	movq	(%rax), %rbx
14001f35b:     	movq	0x8(%rax), %r15
14001f35f:     	leaq	0x18(%r12), %rdx
14001f364:     	movq	%rdx, 0x3a8(%rbp)
14001f36b:     	movq	(%r12), %rcx
14001f36f:     	movq	%rcx, %rax
14001f372:     	negq	%rax
14001f375:     	jo	0x14001f9ac <.text+0x1e9ac>
14001f37b:     	movq	0x8(%r12), %rax
14001f380:     	movq	%rax, 0x648(%rbp)
14001f387:     	cmpq	$-0x1, %r15
14001f38b:     	movq	%rcx, 0x6a0(%rbp)
14001f392:     	je	0x1400204fd <.text+0x1f4fd>
14001f398:     	movq	%rdx, 0x780(%rbp)
14001f39f:     	leaq	0x1(%r15), %rax
14001f3a3:     	movq	%rax, %rdi
14001f3a6:     	subq	%rbx, %rdi
14001f3a9:     	jb	0x140020515 <.text+0x1f515>
14001f3af:     	movq	0x470(%rbp), %rdx
14001f3b6:     	cmpq	%rdx, %r15
14001f3b9:     	jae	0x140020533 <.text+0x1f533>
14001f3bf:     	movq	%rax, 0x678(%rbp)
14001f3c6:     	movq	%rdi, %rax
14001f3c9:     	movl	$0xc, %ecx
14001f3ce:     	mulq	%rcx
14001f3d1:     	jo	0x140020a31 <.text+0x1fa31>
14001f3d7:     	movl	$0x0, %esi
14001f3dc:     	movabsq	$0x7ffffffffffffffc, %rcx # imm = 0x7FFFFFFFFFFFFFFC
14001f3e6:     	cmpq	%rcx, %rax
14001f3e9:     	ja	0x140020a33 <.text+0x1fa33>
14001f3ef:     	movq	0x10(%r12), %rcx
14001f3f4:     	movq	%rcx, 0x588(%rbp)
14001f3fb:     	movq	0x468(%rbp), %r12
14001f402:     	testq	%rax, %rax
14001f405:     	je	0x14001f440 <.text+0x1e440>
14001f407:     	movzbl	0x36157fb(%rip), %ecx   # 0x143634c09
14001f40e:     	movl	$0x4, %esi
14001f413:     	movl	$0x4, %edx
14001f418:     	movq	%rax, %rcx
14001f41b:     	movq	%rax, %r14
14001f41e:     	callq	0x140613c10 <.text+0x612c10>
14001f423:     	movq	%r14, 0xf8(%rbp)
14001f42a:     	testq	%rax, %rax
14001f42d:     	je	0x140020a33 <.text+0x1fa33>
14001f433:     	movq	%rdi, %rsi
14001f436:     	movq	%rax, %r14
14001f439:     	jmp	0x14001f448 <.text+0x1e448>
14001f43b:     	nopl	(%rax,%rax)
14001f440:     	movl	$0x4, %r14d
14001f446:     	xorl	%esi, %esi
14001f448:     	leaq	(%rbx,%rbx,2), %rax
14001f44c:     	leaq	(%r12,%rax,4), %rdx
14001f450:     	leaq	(,%rdi,4), %r12
14001f458:     	leaq	(%r12,%r12,2), %r8
14001f45c:     	movq	%r14, %rcx
14001f45f:     	callq	0x142923c00 <.text+0x2922c00>
14001f464:     	movq	%rsi, 0x3e0(%rbp)
14001f46b:     	movq	%r14, 0xf8(%rbp)
14001f472:     	movq	%r14, 0x3e8(%rbp)
14001f479:     	movq	%rdi, 0x3f0(%rbp)
14001f480:     	movb	$0x1, 0x813(%rbp)
14001f487:     	leaq	0x590(%rbp), %rcx
14001f48e:     	leaq	0x3e0(%rbp), %rdx
14001f495:     	movl	$0x1, %r8d
14001f49b:     	callq	0x140153640 <.text+0x152640>
14001f4a0:     	xorl	%eax, %eax
14001f4a2:     	cmpq	0x590(%rbp), %rax
14001f4a9:     	movq	%r13, %r14
14001f4ac:     	jno	0x14001f4e9 <.text+0x1e4e9>
14001f4ae:     	movq	0x780(%rbp), %rsi
14001f4b5:     	movq	%rsi, %r12
14001f4b8:     	movq	%r14, %rax
14001f4bb:     	movq	0x6a0(%rbp), %rdx
14001f4c2:     	testq	%rdx, %rdx
14001f4c5:     	je	0x14001f333 <.text+0x1e333>
14001f4cb:     	shlq	$0x2, %rdx
14001f4cf:     	movl	$0x4, %r8d
14001f4d5:     	movq	0x648(%rbp), %rcx
14001f4dc:     	callq	0x140613c20 <.text+0x612c20>
14001f4e1:     	movq	%rsi, %r12
14001f4e4:     	jmp	0x14001f330 <.text+0x1e330>
14001f4e9:     	movq	0x5c0(%rbp), %rax
14001f4f0:     	movq	%rax, 0x2c0(%rbp)
14001f4f7:     	movups	0x590(%rbp), %xmm0
14001f4fe:     	movups	0x5a0(%rbp), %xmm1
14001f505:     	movups	0x5b0(%rbp), %xmm2
14001f50c:     	movaps	%xmm2, 0x2b0(%rbp)
14001f513:     	movaps	%xmm1, 0x2a0(%rbp)
14001f51a:     	movaps	%xmm0, 0x290(%rbp)
14001f521:     	movq	0x258(%rbp), %rax
14001f528:     	movl	0x424(%rbp), %ecx
14001f52e:     	movl	%ecx, (%rax)
14001f530:     	movq	0x2c0(%rbp), %rax
14001f537:     	movq	%rax, 0x410(%rbp)
14001f53e:     	movaps	0x290(%rbp), %xmm0
14001f545:     	movaps	0x2a0(%rbp), %xmm1
14001f54c:     	movaps	0x2b0(%rbp), %xmm2
14001f553:     	movaps	%xmm2, 0x400(%rbp)
14001f55a:     	movaps	%xmm1, 0x3f0(%rbp)
14001f561:     	movaps	%xmm0, 0x3e0(%rbp)
14001f568:     	movq	0x4a8(%rbp), %rdx
14001f56f:     	cmpq	%rdx, %r15
14001f572:     	jae	0x140020766 <.text+0x1f766>
14001f578:     	movq	%rdi, %rax
14001f57b:     	shrq	$0x3c, %rax
14001f57f:     	jne	0x140020c32 <.text+0x1fc32>
14001f585:     	leaq	(,%rbx,8), %r13
14001f58d:     	addq	0x4a0(%rbp), %r13
14001f594:     	leaq	(,%rdi,8), %r8
14001f59c:     	testq	%rdi, %rdi
14001f59f:     	je	0x14001f5cd <.text+0x1e5cd>
14001f5a1:     	movzbl	0x3615661(%rip), %eax   # 0x143634c09
14001f5a8:     	movl	$0x4, %edx
14001f5ad:     	movq	%r8, %rsi
14001f5b0:     	movq	%r8, %rcx
14001f5b3:     	callq	0x140613c10 <.text+0x612c10>
14001f5b8:     	testq	%rax, %rax
14001f5bb:     	je	0x140020df6 <.text+0x1fdf6>
14001f5c1:     	movq	%rdi, 0x548(%rbp)
14001f5c8:     	movq	%rsi, %r8
14001f5cb:     	jmp	0x14001f5dd <.text+0x1e5dd>
14001f5cd:     	movl	$0x4, %eax
14001f5d2:     	movq	$0x0, 0x548(%rbp)
14001f5dd:     	movq	%rax, 0x2f0(%rbp)
14001f5e4:     	movq	%rax, %rcx
14001f5e7:     	movq	%r13, %rdx
14001f5ea:     	callq	0x142923c00 <.text+0x2922c00>
14001f5ef:     	movq	0x4c0(%rbp), %rdx
14001f5f6:     	cmpq	%rdx, %r15
14001f5f9:     	jae	0x14002077e <.text+0x1f77e>
14001f5ff:     	leaq	(,%rbx,4), %rsi
14001f607:     	addq	0x4b8(%rbp), %rsi
14001f60e:     	testq	%rdi, %rdi
14001f611:     	je	0x14001f689 <.text+0x1e689>
14001f613:     	movq	0x548(%rbp), %r13
14001f61a:     	movzbl	0x36155e8(%rip), %eax   # 0x143634c09
14001f621:     	movl	$0x4, %edx
14001f626:     	movq	%r12, %rcx
14001f629:     	callq	0x140613c10 <.text+0x612c10>
14001f62e:     	testq	%rax, %rax
14001f631:     	je	0x140020e48 <.text+0x1fe48>
14001f637:     	movq	%rax, 0x370(%rbp)
14001f63e:     	movq	%rax, %rcx
14001f641:     	movq	%rsi, %rdx
14001f644:     	movq	%r12, %r8
14001f647:     	callq	0x142923c00 <.text+0x2922c00>
14001f64c:     	movq	0x4d8(%rbp), %rdx
14001f653:     	cmpq	%rdx, %r15
14001f656:     	jae	0x140020867 <.text+0x1f867>
14001f65c:     	movq	0x4d0(%rbp), %rsi
14001f663:     	movzbl	0x361559f(%rip), %eax   # 0x143634c09
14001f66a:     	movl	$0x4, %edx
14001f66f:     	movq	%r12, %rcx
14001f672:     	callq	0x140613c10 <.text+0x612c10>
14001f677:     	testq	%rax, %rax
14001f67a:     	je	0x140020e1a <.text+0x1fe1a>
14001f680:     	movq	%rdi, 0x678(%rbp)
14001f687:     	jmp	0x14001f6ce <.text+0x1e6ce>
14001f689:     	movq	0x548(%rbp), %r13
14001f690:     	movl	$0x4, %ecx
14001f695:     	movq	%rsi, %rdx
14001f698:     	movq	%r12, %r8
14001f69b:     	callq	0x142923c00 <.text+0x2922c00>
14001f6a0:     	movq	0x4d8(%rbp), %rdx
14001f6a7:     	movl	$0x4, %eax
14001f6ac:     	cmpq	%rdx, %r15
14001f6af:     	jae	0x14002085e <.text+0x1f85e>
14001f6b5:     	movq	%rax, 0x370(%rbp)
14001f6bc:     	movq	0x4d0(%rbp), %rsi
14001f6c3:     	movq	$0x0, 0x678(%rbp)
14001f6ce:     	leaq	(%rsi,%rbx,4), %rdx
14001f6d2:     	movq	%rax, %rsi
14001f6d5:     	movq	%rax, %rcx
14001f6d8:     	movq	%r12, %r8
14001f6db:     	callq	0x142923c00 <.text+0x2922c00>
14001f6e0:     	xorl	%eax, %eax
14001f6e2:     	movl	$0x0, %ecx
14001f6e7:     	testq	%rbx, %rbx
14001f6ea:     	jne	0x14001f6f2 <.text+0x1e6f2>
14001f6ec:     	movl	0x4f8(%rbp), %ecx
14001f6f2:     	movq	0x470(%rbp), %rdx
14001f6f9:     	decq	%rdx
14001f6fc:     	cmpq	%rdx, %r15
14001f6ff:     	movzbl	0x4f9(%rbp), %edx
14001f706:     	cmovel	%edx, %eax
14001f709:     	movq	0x6a0(%rbp), %rdx
14001f710:     	movq	%rdx, 0x610(%rbp)
14001f717:     	movq	0x648(%rbp), %rdx
14001f71e:     	movq	%rdx, 0x618(%rbp)
14001f725:     	movq	0x588(%rbp), %rdx
14001f72c:     	movq	%rdx, 0x620(%rbp)
14001f733:     	movq	0x410(%rbp), %rdx
14001f73a:     	movq	%rdx, 0x5c0(%rbp)
14001f741:     	movaps	0x3e0(%rbp), %xmm0
14001f748:     	movaps	0x3f0(%rbp), %xmm1
14001f74f:     	movaps	0x400(%rbp), %xmm2
14001f756:     	movaps	%xmm0, 0x590(%rbp)
14001f75d:     	movaps	%xmm1, 0x5a0(%rbp)
14001f764:     	movaps	%xmm2, 0x5b0(%rbp)
14001f76b:     	movq	%r13, 0x5c8(%rbp)
14001f772:     	movq	0x2f0(%rbp), %rdx
14001f779:     	movq	%rdx, 0x5d0(%rbp)
14001f780:     	movq	%rdi, 0x5d8(%rbp)
14001f787:     	movq	0x678(%rbp), %rdx
14001f78e:     	movq	%rdx, 0x5e0(%rbp)
14001f795:     	movq	%rsi, 0x5e8(%rbp)
14001f79c:     	movq	%rdi, 0x5f0(%rbp)
14001f7a3:     	movq	%rdx, 0x5f8(%rbp)
14001f7aa:     	movq	0x370(%rbp), %rdx
14001f7b1:     	movq	%rdx, 0x600(%rbp)
14001f7b8:     	movq	%rdi, 0x608(%rbp)
14001f7bf:     	movq	0x788(%rbp), %rdx
14001f7c6:     	movl	%edx, 0x630(%rbp)
14001f7cc:     	movq	0x670(%rbp), %rdx
14001f7d3:     	movq	(%rdx), %rdx
14001f7d6:     	movq	%rdx, 0x628(%rbp)
14001f7dd:     	movb	%cl, 0x634(%rbp)
14001f7e3:     	movb	%al, 0x635(%rbp)
14001f7e9:     	movq	0x338(%rbp), %rdi
14001f7f0:     	movq	0x10(%rdi), %rsi
14001f7f4:     	cmpq	(%rdi), %rsi
14001f7f7:     	leaq	0x590(%rbp), %rbx
14001f7fe:     	jne	0x14001f80f <.text+0x1e80f>
14001f800:     	movq	%rdi, %rcx
14001f803:     	leaq	0x2906ede(%rip), %rdx   # 0x1429266e8
14001f80a:     	callq	0x1405636c0 <.text+0x5626c0>
14001f80f:     	imulq	$0xa8, %rsi, %rcx
14001f816:     	addq	0x8(%rdi), %rcx
14001f81a:     	movl	$0xa8, %r8d
14001f820:     	movq	%rbx, %rdx
14001f823:     	callq	0x142923c10 <.text+0x2922c10>
14001f828:     	incq	%rsi
14001f82b:     	movq	%rsi, 0x10(%rdi)
14001f82f:     	movq	0x780(%rbp), %r12
14001f836:     	jmp	0x14001f330 <.text+0x1e330>
14001f83b:     	movq	0x580(%rbp), %r12
14001f842:     	movq	0x640(%rbp), %rdx
14001f849:     	testq	%rdx, %rdx
14001f84c:     	movq	0x940(%rbp), %r13
14001f853:     	je	0x14001f86b <.text+0x1e86b>
14001f855:     	shlq	$0x4, %rdx
14001f859:     	movl	$0x8, %r8d
14001f85f:     	movq	0x540(%rbp), %rcx
14001f866:     	callq	0x140613c20 <.text+0x612c20>
14001f86b:     	movq	0x580(%rbp), %r8
14001f872:     	movq	%r8, %rax
14001f875:     	subq	%r12, %rax
14001f878:     	movabsq	$-0x5555555555555555, %rcx # imm = 0xAAAAAAAAAAAAAAAB
14001f882:     	mulq	%rcx
14001f885:     	cmpq	%r12, %r8
14001f888:     	je	0x14001f8c8 <.text+0x1e8c8>
14001f88a:     	movq	%rdx, %rdi
14001f88d:     	shrq	$0x4, %rdi
14001f891:     	addq	$0x8, %r12
14001f895:     	jmp	0x14001f8a9 <.text+0x1e8a9>
14001f897:     	nopw	(%rax,%rax)
14001f8a0:     	addq	$0x18, %r12
14001f8a4:     	decq	%rdi
14001f8a7:     	je	0x14001f8c8 <.text+0x1e8c8>
14001f8a9:     	movq	-0x8(%r12), %rdx
14001f8ae:     	testq	%rdx, %rdx
14001f8b1:     	je	0x14001f8a0 <.text+0x1e8a0>
14001f8b3:     	movq	(%r12), %rcx
14001f8b7:     	shlq	$0x2, %rdx
14001f8bb:     	movl	$0x4, %r8d
14001f8c1:     	callq	0x140613c20 <.text+0x612c20>
14001f8c6:     	jmp	0x14001f8a0 <.text+0x1e8a0>
14001f8c8:     	movq	0x528(%rbp), %rax
14001f8cf:     	testq	%rax, %rax
14001f8d2:     	je	0x14001f8ee <.text+0x1e8ee>
14001f8d4:     	shlq	$0x3, %rax
14001f8d8:     	leaq	(%rax,%rax,2), %rdx
14001f8dc:     	movl	$0x8, %r8d
14001f8e2:     	movq	0x6a8(%rbp), %rcx
14001f8e9:     	callq	0x140613c20 <.text+0x612c20>
14001f8ee:     	movq	0x460(%rbp), %rax
14001f8f5:     	testq	%rax, %rax
14001f8f8:     	leaq	0x590(%rbp), %r12
14001f8ff:     	je	0x14001f91b <.text+0x1e91b>
14001f901:     	movq	0x468(%rbp), %rcx
14001f908:     	shlq	$0x2, %rax
14001f90c:     	leaq	(%rax,%rax,2), %rdx
14001f910:     	movl	$0x4, %r8d
14001f916:     	callq	0x140613c20 <.text+0x612c20>
14001f91b:     	movq	0x478(%rbp), %rdx
14001f922:     	testq	%rdx, %rdx
14001f925:     	je	0x14001f93d <.text+0x1e93d>
14001f927:     	movq	0x480(%rbp), %rcx
14001f92e:     	shlq	$0x2, %rdx
14001f932:     	movl	$0x4, %r8d
14001f938:     	callq	0x140613c20 <.text+0x612c20>
14001f93d:     	movq	0x498(%rbp), %rdx
14001f944:     	testq	%rdx, %rdx
14001f947:     	je	0x14001f95f <.text+0x1e95f>
14001f949:     	movq	0x4a0(%rbp), %rcx
14001f950:     	shlq	$0x3, %rdx
14001f954:     	movl	$0x4, %r8d
14001f95a:     	callq	0x140613c20 <.text+0x612c20>
14001f95f:     	movq	0x4b0(%rbp), %rdx
14001f966:     	testq	%rdx, %rdx
14001f969:     	je	0x14001f981 <.text+0x1e981>
14001f96b:     	movq	0x4b8(%rbp), %rcx
14001f972:     	shlq	$0x2, %rdx
14001f976:     	movl	$0x4, %r8d
14001f97c:     	callq	0x140613c20 <.text+0x612c20>
14001f981:     	movq	0x4c8(%rbp), %rdx
14001f988:     	testq	%rdx, %rdx
14001f98b:     	je	0x14001e140 <.text+0x1d140>
14001f991:     	movq	0x4d0(%rbp), %rcx
14001f998:     	shlq	$0x2, %rdx
14001f99c:     	movl	$0x4, %r8d
14001f9a2:     	callq	0x140613c20 <.text+0x612c20>
14001f9a7:     	jmp	0x14001e140 <.text+0x1d140>
14001f9ac:     	movq	%rdx, %r12
14001f9af:     	movq	0x640(%rbp), %rdx
14001f9b6:     	testq	%rdx, %rdx
14001f9b9:     	movq	0x940(%rbp), %r13
14001f9c0:     	jne	0x14001f855 <.text+0x1e855>
14001f9c6:     	jmp	0x14001f86b <.text+0x1e86b>
14001f9cb:     	leaq	0x440(%rbp), %rcx
14001f9d2:     	callq	0x1405dec70 <.text+0x5ddc70>
14001f9d7:     	movq	0x88(%rbp), %rbx
14001f9de:     	xorl	%r14d, %r14d
14001f9e1:     	movq	0x160(%rbp), %rsi
14001f9e8:     	addq	$0x18, %rsi
14001f9ec:     	movq	%rsi, %rcx
14001f9ef:     	callq	0x140c81e10 <.text+0xc80e10>
14001f9f4:     	movb	$0x1, %al
14001f9f6:     	lock
14001f9f7:     	cmpxchgb	%r14b, (%rsi)
14001f9fb:     	jne	0x14001fa22 <.text+0x1ea22>
14001f9fd:     	movq	0x1b8(%rbp), %rsi
14001fa04:     	addq	$0x18, %rsi
14001fa08:     	movq	%rsi, %rcx
14001fa0b:     	callq	0x140c81e10 <.text+0xc80e10>
14001fa10:     	movb	$0x1, %al
14001fa12:     	lock
14001fa13:     	cmpxchgb	%r14b, (%rsi)
14001fa17:     	jne	0x14001fa2e <.text+0x1ea2e>
14001fa19:     	movb	$0x1, 0x790(%rbp)
14001fa20:     	jmp	0x14001fa40 <.text+0x1ea40>
14001fa22:     	movq	%rsi, %rcx
14001fa25:     	xorl	%edx, %edx
14001fa27:     	callq	0x1428cdb90 <.text+0x28ccb90>
14001fa2c:     	jmp	0x14001f9fd <.text+0x1e9fd>
14001fa2e:     	movq	%rsi, %rcx
14001fa31:     	xorl	%edx, %edx
14001fa33:     	callq	0x1428cdb90 <.text+0x28ccb90>
14001fa38:     	jmp	0x14001fa19 <.text+0x1ea19>
14001fa3a:     	nopw	(%rax,%rax)
14001fa40:     	cmpq	0xf0(%rbp), %rbx
14001fa47:     	je	0x14001fac5 <.text+0x1eac5>
14001fa49:     	movq	%rbx, %rcx
14001fa4c:     	addq	$0x150, %rbx            # imm = 0x150
14001fa53:     	cmpb	$0x1, 0x148(%rcx)
14001fa5a:     	jne	0x14001fa40 <.text+0x1ea40>
14001fa5c:     	cmpq	0x98(%rcx), %r14
14001fa63:     	jo	0x14001fa40 <.text+0x1ea40>
14001fa65:     	movq	0x328(%rbp), %rax
14001fa6c:     	cmpq	$0x0, 0x18(%rax)
14001fa71:     	je	0x14001fa40 <.text+0x1ea40>
14001fa73:     	movq	%rcx, 0x7e0(%rbp)
14001fa7a:     	leaq	0x128(%rcx), %rdx
14001fa81:     	movq	0x108(%rbp), %rcx
14001fa88:     	movq	%rdx, 0x670(%rbp)
14001fa8f:     	callq	0x1405b1be0 <.text+0x5b0be0>
14001fa94:     	movq	0x328(%rbp), %rcx
14001fa9b:     	movq	(%rcx), %r8
14001fa9e:     	movq	0x8(%rcx), %r9
14001faa2:     	movq	%r9, %r10
14001faa5:     	andq	%rax, %r10
14001faa8:     	shrq	$0x39, %rax
14001faac:     	movd	%eax, %xmm0
14001fab0:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14001fab4:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14001fab9:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14001fabe:     	xorl	%eax, %eax
14001fac0:     	jmp	0x140018640 <.text+0x17640>
14001fac5:     	movq	0x360(%rbp), %rax
14001facc:     	movq	0x8(%rax), %rsi
14001fad0:     	movq	0x10(%rax), %rax
14001fad4:     	leaq	(%rax,%rax,4), %rdi
14001fad8:     	shlq	$0x5, %rdi
14001fadc:     	addq	%rsi, %rdi
14001fadf:     	addq	$0x168, 0x7b0(%rbp)     # imm = 0x168
14001faea:     	nopw	(%rax,%rax)
14001faf0:     	cmpq	%rdi, %rsi
14001faf3:     	je	0x14001fb21 <.text+0x1eb21>
14001faf5:     	movl	0x88(%rsi), %ecx
14001fafb:     	movl	0x8c(%rsi), %edx
14001fb01:     	callq	0x140959e70 <.text+0x958e70>
14001fb06:     	addq	$0xa0, %rsi
14001fb0d:     	movq	0x7b0(%rbp), %rcx
14001fb14:     	movq	%rax, %rdx
14001fb17:     	movb	$0x1, %r8b
14001fb1a:     	callq	0x1402c8dc0 <.text+0x2c7dc0>
14001fb1f:     	jmp	0x14001faf0 <.text+0x1eaf0>
14001fb21:     	movq	0x338(%rbp), %rax
14001fb28:     	movq	0x8(%rax), %rsi
14001fb2c:     	imulq	$0xa8, 0x10(%rax), %rdi
14001fb34:     	addq	%rsi, %rdi
14001fb37:     	nopw	(%rax,%rax)
14001fb40:     	cmpq	%rdi, %rsi
14001fb43:     	je	0x14001fb71 <.text+0x1eb71>
14001fb45:     	movl	0x98(%rsi), %ecx
14001fb4b:     	movl	0x9c(%rsi), %edx
14001fb51:     	callq	0x140959e70 <.text+0x958e70>
14001fb56:     	addq	$0xa8, %rsi
14001fb5d:     	movq	0x7b0(%rbp), %rcx
14001fb64:     	movq	%rax, %rdx
14001fb67:     	xorl	%r8d, %r8d
14001fb6a:     	callq	0x1402c8dc0 <.text+0x2c7dc0>
14001fb6f:     	jmp	0x14001fb40 <.text+0x1eb40>
14001fb71:     	movq	0x338(%rbp), %rax
14001fb78:     	movq	0x8(%rax), %r12
14001fb7c:     	imulq	$0xa8, 0x10(%rax), %r14
14001fb84:     	addq	%r12, %r14
14001fb87:     	leaq	0x460(%rbp), %rsi
14001fb8e:     	leaq	0x6c0(%rbp), %rdi
14001fb95:     	leaq	0x380(%rbp), %rbx
14001fb9c:     	xorl	%r15d, %r15d
14001fb9f:     	cmpq	%r14, %r12
14001fba2:     	jne	0x14001fbc0 <.text+0x1ebc0>
14001fba4:     	jmp	0x1400200f5 <.text+0x1f0f5>
14001fba9:     	nopl	(%rax)
14001fbb0:     	addq	$0xa8, %r12
14001fbb7:     	cmpq	%r14, %r12
14001fbba:     	je	0x1400200f5 <.text+0x1f0f5>
14001fbc0:     	leaq	0x68(%r12), %rax
14001fbc5:     	leaq	0x38(%r12), %rcx
14001fbca:     	movq	0x950(%rbp), %rdx
14001fbd1:     	movq	%rdx, 0x460(%rbp)
14001fbd8:     	movq	0x7b0(%rbp), %rdx
14001fbdf:     	movq	%rdx, 0x468(%rbp)
14001fbe6:     	movq	0x248(%rbp), %rdx
14001fbed:     	movq	%rdx, 0x470(%rbp)
14001fbf4:     	movq	0x2d0(%rbp), %rdx
14001fbfb:     	movq	%rdx, 0x478(%rbp)
14001fc02:     	movq	%r12, 0x380(%rbp)
14001fc09:     	movq	%rax, 0x388(%rbp)
14001fc10:     	movq	%rcx, 0x390(%rbp)
14001fc17:     	cmpb	$0x0, 0xa4(%r12)
14001fc20:     	je	0x14001fe80 <.text+0x1ee80>
14001fc26:     	movl	0x98(%r12), %edx
14001fc2e:     	movl	0x9c(%r12), %r8d
14001fc36:     	movq	%rsi, %rcx
14001fc39:     	movb	$0x1, %r9b
14001fc3c:     	callq	0x1400286d0 <.text+0x276d0>
14001fc41:     	testb	%al, %al
14001fc43:     	je	0x14001fe80 <.text+0x1ee80>
14001fc49:     	movq	%rdi, %rcx
14001fc4c:     	movq	%rbx, %rdx
14001fc4f:     	movb	$0x1, %r8b
14001fc52:     	callq	0x140027840 <.text+0x26840>
14001fc57:     	cmpq	0x6c0(%rbp), %r15
14001fc5e:     	jo	0x14001fe80 <.text+0x1ee80>
14001fc64:     	movq	0x710(%rbp), %rax
14001fc6b:     	movq	%rax, 0x808(%rbp)
14001fc72:     	movq	0x718(%rbp), %rax
14001fc79:     	movq	%rax, 0x6b0(%rbp)
14001fc80:     	movq	0x720(%rbp), %rdx
14001fc87:     	movq	0x330(%rbp), %rax
14001fc8e:     	movl	0x534(%rbp), %ecx
14001fc94:     	movl	%ecx, (%rax)
14001fc96:     	movq	0x6f0(%rbp), %rax
14001fc9d:     	movq	%rax, 0x5c0(%rbp)
14001fca4:     	movups	0x6c0(%rbp), %xmm0
14001fcab:     	movdqu	0x6d0(%rbp), %xmm1
14001fcb3:     	movdqu	0x6e0(%rbp), %xmm2
14001fcbb:     	movdqa	%xmm2, 0x5b0(%rbp)
14001fcc3:     	movdqa	%xmm1, 0x5a0(%rbp)
14001fccb:     	movaps	%xmm0, 0x590(%rbp)
14001fcd2:     	movq	0x6f8(%rbp), %rax
14001fcd9:     	movq	%rax, 0x7a0(%rbp)
14001fce0:     	movq	0x700(%rbp), %rax
14001fce7:     	movq	%rax, 0x7d0(%rbp)
14001fcee:     	movq	0x708(%rbp), %r15
14001fcf5:     	testq	%rdx, %rdx
14001fcf8:     	movq	%rdx, 0x7f0(%rbp)
14001fcff:     	je	0x14001fd27 <.text+0x1ed27>
14001fd01:     	leaq	(,%rdx,4), %r13
14001fd09:     	movzbl	0x3614ef9(%rip), %eax   # 0x143634c09
14001fd10:     	movl	$0x4, %edx
14001fd15:     	movq	%r13, %rcx
14001fd18:     	callq	0x140613c40 <.text+0x612c40>
14001fd1d:     	testq	%rax, %rax
14001fd20:     	jne	0x14001fd2c <.text+0x1ed2c>
14001fd22:     	jmp	0x140020a88 <.text+0x1fa88>
14001fd27:     	movl	$0x4, %eax
14001fd2c:     	movq	%rax, 0x7d8(%rbp)
14001fd33:     	movl	0x98(%r12), %ecx
14001fd3b:     	movl	0x9c(%r12), %edx
14001fd43:     	callq	0x140959e70 <.text+0x958e70>
14001fd48:     	movl	0xa0(%r12), %ecx
14001fd50:     	movq	0x5c0(%rbp), %rdx
14001fd57:     	movq	%rdx, 0x170(%rbp)
14001fd5e:     	movaps	0x590(%rbp), %xmm0
14001fd65:     	movdqa	0x5a0(%rbp), %xmm1
14001fd6d:     	movdqa	0x5b0(%rbp), %xmm2
14001fd75:     	movdqa	%xmm2, 0x160(%rbp)
14001fd7d:     	movdqa	%xmm1, 0x150(%rbp)
14001fd85:     	movaps	%xmm0, 0x140(%rbp)
14001fd8c:     	movq	0x7a0(%rbp), %rdx
14001fd93:     	movq	%rdx, 0x178(%rbp)
14001fd9a:     	movq	0x7d0(%rbp), %rdx
14001fda1:     	movq	%rdx, 0x180(%rbp)
14001fda8:     	movq	%r15, 0x188(%rbp)
14001fdaf:     	movq	0x808(%rbp), %rdx
14001fdb6:     	movq	%rdx, 0x190(%rbp)
14001fdbd:     	movq	0x6b0(%rbp), %rdx
14001fdc4:     	movq	%rdx, 0x198(%rbp)
14001fdcb:     	movq	0x7f0(%rbp), %rdx
14001fdd2:     	movq	%rdx, 0x1a0(%rbp)
14001fdd9:     	movq	%rdx, 0x1a8(%rbp)
14001fde0:     	movq	0x7d8(%rbp), %r8
14001fde7:     	movq	%r8, 0x1b0(%rbp)
14001fdee:     	movq	%rdx, 0x1b8(%rbp)
14001fdf5:     	movq	%rax, 0x1c0(%rbp)
14001fdfc:     	movw	$0x0, 0x1d4(%rbp)
14001fe05:     	movl	%ecx, 0x1d0(%rbp)
14001fe0b:     	movq	0x98(%r12), %rax
14001fe13:     	movq	%rax, 0x1c8(%rbp)
14001fe1a:     	movl	$0x0, 0x1d6(%rbp)
14001fe24:     	movw	$0x100, 0x1da(%rbp)     # imm = 0x100
14001fe2d:     	movq	0x360(%rbp), %r13
14001fe34:     	movq	0x10(%r13), %r15
14001fe38:     	cmpq	(%r13), %r15
14001fe3c:     	jne	0x14001fe4d <.text+0x1ee4d>
14001fe3e:     	movq	%r13, %rcx
14001fe41:     	leaq	0x29066e8(%rip), %rdx   # 0x142926530
14001fe48:     	callq	0x140563480 <.text+0x562480>
14001fe4d:     	leaq	(%r15,%r15,4), %rcx
14001fe51:     	shlq	$0x5, %rcx
14001fe55:     	addq	0x8(%r13), %rcx
14001fe59:     	movl	$0xa0, %r8d
14001fe5f:     	leaq	0x140(%rbp), %rdx
14001fe66:     	callq	0x142923c10 <.text+0x2922c10>
14001fe6b:     	incq	%r15
14001fe6e:     	movq	%r15, 0x10(%r13)
14001fe72:     	movq	0x940(%rbp), %r13
14001fe79:     	xorl	%r15d, %r15d
14001fe7c:     	nopl	(%rax)
14001fe80:     	cmpb	$0x0, 0xa5(%r12)
14001fe89:     	je	0x14001fbb0 <.text+0x1ebb0>
14001fe8f:     	movl	0x98(%r12), %r8d
14001fe97:     	movl	0x9c(%r12), %edx
14001fe9f:     	movq	%rsi, %rcx
14001fea2:     	xorl	%r9d, %r9d
14001fea5:     	callq	0x1400286d0 <.text+0x276d0>
14001feaa:     	testb	%al, %al
14001feac:     	je	0x14001fbb0 <.text+0x1ebb0>
14001feb2:     	movq	%rdi, %rcx
14001feb5:     	movq	%rbx, %rdx
14001feb8:     	xorl	%r8d, %r8d
14001febb:     	callq	0x140027840 <.text+0x26840>
14001fec0:     	cmpq	0x6c0(%rbp), %r15
14001fec7:     	jo	0x14001fbb0 <.text+0x1ebb0>
14001fecd:     	movq	0x710(%rbp), %rax
14001fed4:     	movq	%rax, 0x808(%rbp)
14001fedb:     	movq	0x718(%rbp), %rax
14001fee2:     	movq	%rax, 0x6b0(%rbp)
14001fee9:     	movq	0x720(%rbp), %rdx
14001fef0:     	movq	0x330(%rbp), %rax
14001fef7:     	movl	0x534(%rbp), %ecx
14001fefd:     	movl	%ecx, (%rax)
14001feff:     	movq	0x6f0(%rbp), %rax
14001ff06:     	movq	%rax, 0x5c0(%rbp)
14001ff0d:     	movups	0x6c0(%rbp), %xmm0
14001ff14:     	movdqu	0x6d0(%rbp), %xmm1
14001ff1c:     	movdqu	0x6e0(%rbp), %xmm2
14001ff24:     	movdqa	%xmm2, 0x5b0(%rbp)
14001ff2c:     	movdqa	%xmm1, 0x5a0(%rbp)
14001ff34:     	movaps	%xmm0, 0x590(%rbp)
14001ff3b:     	movq	0x6f8(%rbp), %rax
14001ff42:     	movq	%rax, 0x7a0(%rbp)
14001ff49:     	movq	0x700(%rbp), %rax
14001ff50:     	movq	%rax, 0x7d0(%rbp)
14001ff57:     	movq	0x708(%rbp), %r15
14001ff5e:     	testq	%rdx, %rdx
14001ff61:     	movq	%rdx, 0x7f0(%rbp)
14001ff68:     	je	0x14001ff90 <.text+0x1ef90>
14001ff6a:     	leaq	(,%rdx,4), %r13
14001ff72:     	movzbl	0x3614c90(%rip), %eax   # 0x143634c09
14001ff79:     	movl	$0x4, %edx
14001ff7e:     	movq	%r13, %rcx
14001ff81:     	callq	0x140613c40 <.text+0x612c40>
14001ff86:     	testq	%rax, %rax
14001ff89:     	jne	0x14001ff95 <.text+0x1ef95>
14001ff8b:     	jmp	0x140020aa8 <.text+0x1faa8>
14001ff90:     	movl	$0x4, %eax
14001ff95:     	movq	%rax, 0x7d8(%rbp)
14001ff9c:     	movl	0x98(%r12), %ecx
14001ffa4:     	movl	0x9c(%r12), %edx
14001ffac:     	callq	0x140959e70 <.text+0x958e70>
14001ffb1:     	movl	0xa0(%r12), %ecx
14001ffb9:     	movq	0x5c0(%rbp), %rdx
14001ffc0:     	movq	%rdx, 0x170(%rbp)
14001ffc7:     	movaps	0x590(%rbp), %xmm0
14001ffce:     	movdqa	0x5a0(%rbp), %xmm1
14001ffd6:     	movdqa	0x5b0(%rbp), %xmm2
14001ffde:     	movdqa	%xmm2, 0x160(%rbp)
14001ffe6:     	movdqa	%xmm1, 0x150(%rbp)
14001ffee:     	movaps	%xmm0, 0x140(%rbp)
14001fff5:     	movq	0x7a0(%rbp), %rdx
14001fffc:     	movq	%rdx, 0x178(%rbp)
140020003:     	movq	0x7d0(%rbp), %rdx
14002000a:     	movq	%rdx, 0x180(%rbp)
140020011:     	movq	%r15, 0x188(%rbp)
140020018:     	movq	0x808(%rbp), %rdx
14002001f:     	movq	%rdx, 0x190(%rbp)
140020026:     	movq	0x6b0(%rbp), %rdx
14002002d:     	movq	%rdx, 0x198(%rbp)
140020034:     	movq	0x7f0(%rbp), %rdx
14002003b:     	movq	%rdx, 0x1a0(%rbp)
140020042:     	movq	%rdx, 0x1a8(%rbp)
140020049:     	movq	0x7d8(%rbp), %r8
140020050:     	movq	%r8, 0x1b0(%rbp)
140020057:     	movq	%rdx, 0x1b8(%rbp)
14002005e:     	movq	%rax, 0x1c0(%rbp)
140020065:     	movw	$0x0, 0x1d4(%rbp)
14002006e:     	movl	%ecx, 0x1d0(%rbp)
140020074:     	movq	0x98(%r12), %rax
14002007c:     	movq	%rax, 0x1c8(%rbp)
140020083:     	movl	$0x0, 0x1d6(%rbp)
14002008d:     	movw	$0x200, 0x1da(%rbp)     # imm = 0x200
140020096:     	movq	0x360(%rbp), %r13
14002009d:     	movq	0x10(%r13), %r15
1400200a1:     	cmpq	(%r13), %r15
1400200a5:     	jne	0x1400200b6 <.text+0x1f0b6>
1400200a7:     	movq	%r13, %rcx
1400200aa:     	leaq	0x29064af(%rip), %rdx   # 0x142926560
1400200b1:     	callq	0x140563480 <.text+0x562480>
1400200b6:     	leaq	(%r15,%r15,4), %rcx
1400200ba:     	shlq	$0x5, %rcx
1400200be:     	addq	0x8(%r13), %rcx
1400200c2:     	movl	$0xa0, %r8d
1400200c8:     	leaq	0x140(%rbp), %rdx
1400200cf:     	callq	0x142923c10 <.text+0x2922c10>
1400200d4:     	incq	%r15
1400200d7:     	movq	%r15, 0x10(%r13)
1400200db:     	movq	0x940(%rbp), %r13
1400200e2:     	xorl	%r15d, %r15d
1400200e5:     	addq	$0xa8, %r12
1400200ec:     	cmpq	%r14, %r12
1400200ef:     	jne	0x14001fbc0 <.text+0x1ebc0>
1400200f5:     	testb	$0x1, 0x790(%rbp)
1400200fc:     	je	0x14002026f <.text+0x1f26f>
140020102:     	movq	0x2e0(%rbp), %r8
140020109:     	movq	(%r8), %rax
14002010c:     	movq	0x8(%r8), %rcx
140020110:     	addq	%rax, %rcx
140020113:     	incq	%rcx
140020116:     	movdqa	(%rax), %xmm0
14002011a:     	pmovmskb	%xmm0, %edx
14002011e:     	notl	%edx
140020120:     	movq	0x18(%r8), %r8
140020124:     	movq	$0x0, 0x140(%rbp)
14002012f:     	movq	$0x0, 0x150(%rbp)
14002013a:     	movq	%rax, 0x160(%rbp)
140020141:     	addq	$0x10, %rax
140020145:     	movq	%rax, 0x168(%rbp)
14002014c:     	movq	%rcx, 0x170(%rbp)
140020153:     	movw	%dx, 0x178(%rbp)
14002015a:     	movq	%r8, 0x180(%rbp)
140020161:     	leaq	0x29071c8(%rip), %r8    # 0x142927330
140020168:     	leaq	0x590(%rbp), %rcx
14002016f:     	leaq	0x140(%rbp), %rdx
140020176:     	callq	0x1402bcd50 <.text+0x2bbd50>
14002017b:     	movq	0x598(%rbp), %rax
140020182:     	movq	%rax, 0x808(%rbp)
140020189:     	movq	0x5a0(%rbp), %r14
140020190:     	testq	%r14, %r14
140020193:     	je	0x1400202bb <.text+0x1f2bb>
140020199:     	movq	0x268(%rbp), %rcx
1400201a0:     	callq	0x140987ca0 <.text+0x986ca0>
1400201a5:     	movq	%rax, %rsi
1400201a8:     	movq	%r14, %rdi
1400201ab:     	shlq	$0x4, %rdi
1400201af:     	leaq	0x6c0(%rbp), %rbx
1400201b6:     	movq	%rbx, %rcx
1400201b9:     	movq	%rdi, %rdx
1400201bc:     	movl	$0x20, %r8d
1400201c2:     	callq	0x140bd4e20 <.text+0xbd3e20>
1400201c7:     	movq	%rdi, 0x30(%rsp)
1400201cc:     	movq	0x808(%rbp), %rax
1400201d3:     	movq	%rax, 0x28(%rsp)
1400201d8:     	movq	%rbx, 0x20(%rsp)
1400201dd:     	leaq	0x290630c(%rip), %r8    # 0x1429264f0
1400201e4:     	leaq	0x140(%rbp), %rcx
1400201eb:     	movl	$0xf, %r9d
1400201f1:     	movq	%rsi, %rdx
1400201f4:     	callq	0x1405b93b0 <.text+0x5b83b0>
1400201f9:     	cmpl	$0xb, 0x140(%rbp)
140020200:     	jne	0x1400202f5 <.text+0x1f2f5>
140020206:     	movq	0x168(%rbp), %rax
14002020d:     	movq	%rax, 0x6e0(%rbp)
140020214:     	movdqu	0x148(%rbp), %xmm0
14002021c:     	movdqu	0x158(%rbp), %xmm1
140020224:     	movdqa	%xmm1, 0x6d0(%rbp)
14002022c:     	movdqa	%xmm0, 0x6c0(%rbp)
140020234:     	movl	%r14d, 0x6e8(%rbp)
14002023b:     	leaq	0x29062be(%rip), %r8    # 0x142926500
140020242:     	leaq	0x6c0(%rbp), %rdx
140020249:     	movq	%r13, %rcx
14002024c:     	callq	0x14014dd30 <.text+0x14cd30>
140020251:     	cmpl	$0xb, 0x140(%rbp)
140020258:     	je	0x140020301 <.text+0x1f301>
14002025e:     	leaq	0x140(%rbp), %rcx
140020265:     	callq	0x1400297d0 <.text+0x287d0>
14002026a:     	jmp	0x140020301 <.text+0x1f301>
14002026f:     	movq	0x2e0(%rbp), %rax
140020276:     	cmpq	$0x0, 0x18(%rax)
14002027b:     	jne	0x140020323 <.text+0x1f323>
140020281:     	movq	0x8(%r13), %rsi
140020285:     	testq	%rsi, %rsi
140020288:     	jne	0x14002028e <.text+0x1f28e>
14002028a:     	movq	0x10(%r13), %rsi
14002028e:     	movq	(%rsi), %rax
140020291:     	movq	0x10(%rsi), %rdi
140020295:     	subq	%rdi, %rax
140020298:     	cmpq	$0x7, %rax
14002029c:     	jbe	0x14002071c <.text+0x1f71c>
1400202a2:     	movq	0x8(%rsi), %rax
1400202a6:     	leaq	0x12d313(%rip), %rcx    # 0x14014d5c0 <.text+0x14c5c0>
1400202ad:     	movq	%rcx, (%rax,%rdi)
1400202b1:     	addq	$0x8, %rdi
1400202b5:     	movq	%rdi, 0x10(%rsi)
1400202b9:     	jmp	0x140020323 <.text+0x1f323>
1400202bb:     	movq	0x8(%r13), %rsi
1400202bf:     	testq	%rsi, %rsi
1400202c2:     	jne	0x1400202c8 <.text+0x1f2c8>
1400202c4:     	movq	0x10(%r13), %rsi
1400202c8:     	movq	(%rsi), %rax
1400202cb:     	movq	0x10(%rsi), %rdi
1400202cf:     	subq	%rdi, %rax
1400202d2:     	cmpq	$0x7, %rax
1400202d6:     	jbe	0x140020741 <.text+0x1f741>
1400202dc:     	movq	0x8(%rsi), %rax
1400202e0:     	leaq	0x12d2d9(%rip), %rcx    # 0x14014d5c0 <.text+0x14c5c0>
1400202e7:     	movq	%rcx, (%rax,%rdi)
1400202eb:     	addq	$0x8, %rdi
1400202ef:     	movq	%rdi, 0x10(%rsi)
1400202f3:     	jmp	0x140020301 <.text+0x1f301>
1400202f5:     	leaq	0x140(%rbp), %rcx
1400202fc:     	callq	0x1400297d0 <.text+0x287d0>
140020301:     	movq	0x590(%rbp), %rdx
140020308:     	testq	%rdx, %rdx
14002030b:     	je	0x140020323 <.text+0x1f323>
14002030d:     	shlq	$0x4, %rdx
140020311:     	movl	$0x10, %r8d
140020317:     	movq	0x808(%rbp), %rcx
14002031e:     	callq	0x140613c20 <.text+0x612c20>
140020323:     	cmpb	$0x0, 0x79f(%rbp)
14002032a:     	je	0x140020377 <.text+0x1f377>
14002032c:     	callq	0x140c18130 <.text+0xc17130>
140020331:     	movq	%rax, %rcx
140020334:     	movq	(%rax), %rax
140020337:     	cmpq	$0x1, %rax
14002033b:     	jne	0x140020478 <.text+0x1f478>
140020341:     	addq	$0x8, %rcx
140020345:     	movq	0x270(%rbp), %rdx
14002034c:     	cmpq	$0x0, (%rcx)
140020350:     	jne	0x1400206e0 <.text+0x1f6e0>
140020356:     	movq	$-0x1, (%rcx)
14002035d:     	movq	%rcx, 0x808(%rbp)
140020364:     	addq	$0x8, %rcx
140020368:     	callq	0x140c170f0 <.text+0xc160f0>
14002036d:     	movq	0x808(%rbp), %rax
140020374:     	incq	(%rax)
140020377:     	movaps	0x820(%rbp), %xmm6
14002037e:     	movaps	0x830(%rbp), %xmm7
140020385:     	movaps	0x840(%rbp), %xmm8
14002038d:     	movaps	0x850(%rbp), %xmm9
140020395:     	movaps	0x860(%rbp), %xmm10
14002039d:     	movaps	0x870(%rbp), %xmm11
1400203a5:     	movaps	0x880(%rbp), %xmm12
1400203ad:     	movaps	0x890(%rbp), %xmm13
1400203b5:     	movaps	0x8a0(%rbp), %xmm14
1400203bd:     	movaps	0x8b0(%rbp), %xmm15
1400203c5:     	addq	$0x948, %rsp            # imm = 0x948
1400203cc:     	popq	%rbx
1400203cd:     	popq	%rdi
1400203ce:     	popq	%rsi
1400203cf:     	popq	%r12
1400203d1:     	popq	%r13
1400203d3:     	popq	%r14
1400203d5:     	popq	%r15
1400203d7:     	popq	%rbp
1400203d8:     	retq
1400203d9:     	movb	$0x1, 0x817(%rbp)
1400203e0:     	leaq	0x29063c1(%rip), %rcx   # 0x1429267a8
1400203e7:     	callq	0x1428d98a0 <.text+0x28d88a0>
1400203ec:     	jmp	0x140020fdc <.text+0x1ffdc>
1400203f1:     	movb	$0x1, 0x817(%rbp)
1400203f8:     	leaq	0x29063a9(%rip), %r8    # 0x1429267a8
1400203ff:     	movq	%rcx, %rdx
140020402:     	movq	%rdi, %rcx
140020405:     	callq	0x1428d9850 <.text+0x28d8850>
14002040a:     	jmp	0x140020fdc <.text+0x1ffdc>
14002040f:     	movb	$0x1, 0x817(%rbp)
140020416:     	leaq	0x290638b(%rip), %r8    # 0x1429267a8
14002041d:     	movq	0x780(%rbp), %rdx
140020424:     	callq	0x1428d9840 <.text+0x28d8840>
140020429:     	jmp	0x140020fdc <.text+0x1ffdc>
14002042e:     	movb	$0x1, 0x817(%rbp)
140020435:     	leaq	0x2906384(%rip), %r8    # 0x1429267c0
14002043c:     	callq	0x1428d9840 <.text+0x28d8840>
140020441:     	jmp	0x140020fdc <.text+0x1ffdc>
140020446:     	movb	$0x1, 0x817(%rbp)
14002044d:     	leaq	0x2906384(%rip), %r8    # 0x1429267d8
140020454:     	callq	0x1428d9840 <.text+0x28d8840>
140020459:     	jmp	0x140020fdc <.text+0x1ffdc>
14002045e:     	testq	%rax, %rax
140020461:     	jne	0x14002049d <.text+0x1f49d>
140020463:     	xorl	%edx, %edx
140020465:     	callq	0x1428a1640 <.text+0x28a0640>
14002046a:     	movq	%rax, %rcx
14002046d:     	testq	%rax, %rax
140020470:     	jne	0x140018368 <.text+0x17368>
140020476:     	jmp	0x14002049d <.text+0x1f49d>
140020478:     	testq	%rax, %rax
14002047b:     	movq	0x270(%rbp), %rax
140020482:     	jne	0x14002049d <.text+0x1f49d>
140020484:     	movq	%rax, %rsi
140020487:     	xorl	%edx, %edx
140020489:     	callq	0x1428a1640 <.text+0x28a0640>
14002048e:     	movq	%rsi, %rdx
140020491:     	movq	%rax, %rcx
140020494:     	testq	%rax, %rax
140020497:     	jne	0x14002034c <.text+0x1f34c>
14002049d:     	leaq	0x291256c(%rip), %rcx   # 0x142932a10
1400204a4:     	callq	0x1428d80d0 <.text+0x28d70d0>
1400204a9:     	movb	$0x1, 0x817(%rbp)
1400204b0:     	leaq	0x2906339(%rip), %r8    # 0x1429267f0
1400204b7:     	callq	0x1428d9840 <.text+0x28d8840>
1400204bc:     	jmp	0x140020fdc <.text+0x1ffdc>
1400204c1:     	leaq	0x2906238(%rip), %rcx   # 0x142926700
1400204c8:     	callq	0x1428d98a0 <.text+0x28d88a0>
1400204cd:     	jmp	0x140020fdc <.text+0x1ffdc>
1400204d2:     	leaq	0x2906227(%rip), %r8    # 0x142926700
1400204d9:     	movq	%r13, %rcx
1400204dc:     	movq	%rax, %rdx
1400204df:     	callq	0x1428d9850 <.text+0x28d8850>
1400204e4:     	jmp	0x140020fdc <.text+0x1ffdc>
1400204e9:     	leaq	0x2906210(%rip), %r8    # 0x142926700
1400204f0:     	movq	%rax, %rcx
1400204f3:     	callq	0x1428d9840 <.text+0x28d8840>
1400204f8:     	jmp	0x140020fdc <.text+0x1ffdc>
1400204fd:     	movb	$0x1, 0x813(%rbp)
140020504:     	leaq	0x290617d(%rip), %rcx   # 0x142926688
14002050b:     	callq	0x1428d98a0 <.text+0x28d88a0>
140020510:     	jmp	0x140020fdc <.text+0x1ffdc>
140020515:     	movb	$0x1, 0x813(%rbp)
14002051c:     	leaq	0x2906165(%rip), %r8    # 0x142926688
140020523:     	movq	%rbx, %rcx
140020526:     	movq	%rax, %rdx
140020529:     	callq	0x1428d9850 <.text+0x28d8850>
14002052e:     	jmp	0x140020fdc <.text+0x1ffdc>
140020533:     	movb	$0x1, 0x813(%rbp)
14002053a:     	leaq	0x2906147(%rip), %r8    # 0x142926688
140020541:     	movq	%rax, %rcx
140020544:     	callq	0x1428d9840 <.text+0x28d8840>
140020549:     	jmp	0x140020fdc <.text+0x1ffdc>
14002054e:     	movb	$0x1, 0x816(%rbp)
140020555:     	movb	$0x1, 0x815(%rbp)
14002055c:     	leaq	0x29064e5(%rip), %rcx   # 0x142926a48
140020563:     	callq	0x1428d98a0 <.text+0x28d88a0>
140020568:     	jmp	0x140020fdc <.text+0x1ffdc>
14002056d:     	movb	$0x1, 0x816(%rbp)
140020574:     	movb	$0x1, 0x815(%rbp)
14002057b:     	leaq	0x29064c6(%rip), %r8    # 0x142926a48
140020582:     	movq	0x808(%rbp), %rcx
140020589:     	movq	%r9, %rdx
14002058c:     	callq	0x1428d9850 <.text+0x28d8850>
140020591:     	jmp	0x140020fdc <.text+0x1ffdc>
140020596:     	movb	$0x1, 0x816(%rbp)
14002059d:     	movb	$0x1, 0x815(%rbp)
1400205a4:     	leaq	0x290649d(%rip), %r8    # 0x142926a48
1400205ab:     	movq	%r9, %rcx
1400205ae:     	callq	0x1428d9840 <.text+0x28d8840>
1400205b3:     	jmp	0x140020fdc <.text+0x1ffdc>
1400205b8:     	movq	$0x0, 0x6c0(%rbp)
1400205c3:     	movb	$0x1, 0x817(%rbp)
1400205ca:     	leaq	0x290623f(%rip), %rax   # 0x142926810
1400205d1:     	movq	%rax, 0x20(%rsp)
1400205d6:     	leaq	0x290622b(%rip), %r8    # 0x142926808
1400205dd:     	leaq	0x78(%rbp), %rdx
1400205e1:     	leaq	0x6c0(%rbp), %r9
1400205e8:     	xorl	%ecx, %ecx
1400205ea:     	callq	0x1428d9596 <.text+0x28d8596>
1400205ef:     	jmp	0x140020fdc <.text+0x1ffdc>
1400205f4:     	movq	$0x0, 0x140(%rbp)
1400205ff:     	leaq	0x298db92(%rip), %rax   # 0x1429ae198
140020606:     	movq	%rax, 0x20(%rsp)
14002060b:     	leaq	0x6c0(%rbp), %rdx
140020612:     	leaq	0x140(%rbp), %r9
140020619:     	xorl	%ecx, %ecx
14002061b:     	movq	%r14, %r8
14002061e:     	callq	0x1428cab41 <.text+0x28c9b41>
140020623:     	jmp	0x140020fdc <.text+0x1ffdc>
140020628:     	movq	$0x0, 0x140(%rbp)
140020633:     	leaq	0x298db5e(%rip), %rax   # 0x1429ae198
14002063a:     	movq	%rax, 0x20(%rsp)
14002063f:     	leaq	0x6c0(%rbp), %rdx
140020646:     	leaq	0x140(%rbp), %r9
14002064d:     	xorl	%ecx, %ecx
14002064f:     	movq	%r14, %r8
140020652:     	callq	0x1428cab41 <.text+0x28c9b41>
140020657:     	jmp	0x140020fdc <.text+0x1ffdc>
14002065c:     	movq	$0x0, 0x140(%rbp)
140020667:     	leaq	0x298db2a(%rip), %rax   # 0x1429ae198
14002066e:     	movq	%rax, 0x20(%rsp)
140020673:     	leaq	0x6c0(%rbp), %rdx
14002067a:     	leaq	0x140(%rbp), %r9
140020681:     	xorl	%ecx, %ecx
140020683:     	movq	%rbx, %r8
140020686:     	callq	0x1428cab41 <.text+0x28c9b41>
14002068b:     	jmp	0x140020fdc <.text+0x1ffdc>
140020690:     	leaq	0x2905d71(%rip), %r8    # 0x142926408
140020697:     	movq	%r12, %rcx
14002069a:     	callq	0x1428d9518 <.text+0x28d8518>
14002069f:     	jmp	0x140020fdc <.text+0x1ffdc>
1400206a4:     	leaq	0x2905ce5(%rip), %r8    # 0x142926390
1400206ab:     	movq	%rbx, %rcx
1400206ae:     	callq	0x1428d9518 <.text+0x28d8518>
1400206b3:     	jmp	0x140020fdc <.text+0x1ffdc>
1400206b8:     	leaq	0x2905d49(%rip), %r8    # 0x142926408
1400206bf:     	movq	%rbx, %rcx
1400206c2:     	callq	0x1428d9518 <.text+0x28d8518>
1400206c7:     	jmp	0x140020fdc <.text+0x1ffdc>
1400206cc:     	leaq	0x2905cbd(%rip), %r8    # 0x142926390
1400206d3:     	movq	%r12, %rcx
1400206d6:     	callq	0x1428d9518 <.text+0x28d8518>
1400206db:     	jmp	0x140020fdc <.text+0x1ffdc>
1400206e0:     	leaq	0x29123d1(%rip), %rcx   # 0x142932ab8
1400206e7:     	callq	0x1428d9250 <.text+0x28d8250>
1400206ec:     	leaq	0x2906025(%rip), %r8    # 0x142926718
1400206f3:     	movq	0x780(%rbp), %rcx
1400206fa:     	callq	0x1428d9840 <.text+0x28d8840>
1400206ff:     	jmp	0x140020fdc <.text+0x1ffdc>
140020704:     	leaq	0x2906025(%rip), %r8    # 0x142926730
14002070b:     	movq	0x780(%rbp), %rcx
140020712:     	callq	0x1428d9840 <.text+0x28d8840>
140020717:     	jmp	0x140020fdc <.text+0x1ffdc>
14002071c:     	movq	$0x1, 0x20(%rsp)
140020725:     	movl	$0x8, %r8d
14002072b:     	movl	$0x1, %r9d
140020731:     	movq	%rsi, %rcx
140020734:     	movq	%rdi, %rdx
140020737:     	callq	0x1428a29e0 <.text+0x28a19e0>
14002073c:     	jmp	0x1400202a2 <.text+0x1f2a2>
140020741:     	movq	$0x1, 0x20(%rsp)
14002074a:     	movl	$0x8, %r8d
140020750:     	movl	$0x1, %r9d
140020756:     	movq	%rsi, %rcx
140020759:     	movq	%rdi, %rdx
14002075c:     	callq	0x1428a29e0 <.text+0x28a19e0>
140020761:     	jmp	0x1400202dc <.text+0x1f2dc>
140020766:     	leaq	0x2905f33(%rip), %r8    # 0x1429266a0
14002076d:     	movq	0x678(%rbp), %rcx
140020774:     	callq	0x1428d9840 <.text+0x28d8840>
140020779:     	jmp	0x140020fdc <.text+0x1ffdc>
14002077e:     	leaq	0x2905f33(%rip), %r8    # 0x1429266b8
140020785:     	movq	0x678(%rbp), %rcx
14002078c:     	callq	0x1428d9840 <.text+0x28d8840>
140020791:     	jmp	0x140020fdc <.text+0x1ffdc>
140020796:     	leaq	0x29062c3(%rip), %r8    # 0x142926a60
14002079d:     	movq	0x6b0(%rbp), %rcx
1400207a4:     	callq	0x1428d9840 <.text+0x28d8840>
1400207a9:     	jmp	0x140020fdc <.text+0x1ffdc>
1400207ae:     	leaq	0x29062c3(%rip), %r8    # 0x142926a78
1400207b5:     	movq	0x6b0(%rbp), %rcx
1400207bc:     	callq	0x1428d9840 <.text+0x28d8840>
1400207c1:     	jmp	0x140020fdc <.text+0x1ffdc>
1400207c6:     	leaq	0x29062c3(%rip), %r8    # 0x142926a90
1400207cd:     	movq	0x6b0(%rbp), %rcx
1400207d4:     	movq	%r12, %rdx
1400207d7:     	callq	0x1428d9840 <.text+0x28d8840>
1400207dc:     	jmp	0x140020fdc <.text+0x1ffdc>
1400207e1:     	movb	$0x1, 0x803(%rbp)
1400207e8:     	movb	$0x1, 0x802(%rbp)
1400207ef:     	movb	$0x1, 0x801(%rbp)
1400207f6:     	movb	$0x0, 0x800(%rbp)
1400207fd:     	movb	$0x0, 0x7ff(%rbp)
140020804:     	leaq	0x2905dd5(%rip), %rcx   # 0x1429265e0
14002080b:     	callq	0x1428d9310 <.text+0x28d8310>
140020810:     	jmp	0x140020fdc <.text+0x1ffdc>
140020815:     	xorl	%edi, %edi
140020817:     	movq	%rbx, 0xd0(%rbp)
14002081e:     	movq	%rdi, 0xc8(%rbp)
140020825:     	leaq	0x2905f1c(%rip), %r8    # 0x142926748
14002082c:     	movq	0x780(%rbp), %rcx
140020833:     	callq	0x1428d9840 <.text+0x28d8840>
140020838:     	jmp	0x140020fdc <.text+0x1ffdc>
14002083d:     	xorl	%r12d, %r12d
140020840:     	movb	$0x1, 0x817(%rbp)
140020847:     	leaq	0x29936e2(%rip), %r8    # 0x1429b3f30
14002084e:     	movq	%r12, %rcx
140020851:     	movq	%rbx, %rdx
140020854:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020859:     	jmp	0x140020fdc <.text+0x1ffdc>
14002085e:     	movq	%rax, 0x370(%rbp)
140020865:     	xorl	%edi, %edi
140020867:     	movq	0x370(%rbp), %rax
14002086e:     	movq	%rax, 0xe0(%rbp)
140020875:     	movq	%rdi, 0xd8(%rbp)
14002087c:     	leaq	0x2905e4d(%rip), %r8    # 0x1429266d0
140020883:     	movq	0x678(%rbp), %rcx
14002088a:     	callq	0x1428d9840 <.text+0x28d8840>
14002088f:     	jmp	0x140020fdc <.text+0x1ffdc>
140020894:     	xorl	%esi, %esi
140020896:     	leaq	0x298db9b(%rip), %r8    # 0x1429ae438
14002089d:     	movq	%rsi, %rcx
1400208a0:     	movq	%r12, %rdx
1400208a3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400208a8:     	jmp	0x140020fdc <.text+0x1ffdc>
1400208ad:     	leaq	0x568(%rbp), %rax
1400208b4:     	movq	%rax, 0x590(%rbp)
1400208bb:     	leaq	0x967e(%rip), %rax      # 0x140029f40 <.text+0x28f40>
1400208c2:     	movq	%rax, 0x598(%rbp)
1400208c9:     	leaq	0x2905b70(%rip), %rax   # 0x142926440
1400208d0:     	movq	%rax, 0x6c0(%rbp)
1400208d7:     	movq	$0x1, 0x6c8(%rbp)
1400208e2:     	movq	$0x0, 0x6e0(%rbp)
1400208ed:     	leaq	0x590(%rbp), %rax
1400208f4:     	movq	%rax, 0x6d0(%rbp)
1400208fb:     	movq	$0x1, 0x6d8(%rbp)
140020906:     	leaq	0x2905b43(%rip), %rdx   # 0x142926450
14002090d:     	leaq	0x6c0(%rbp), %rcx
140020914:     	callq	0x1428d9390 <.text+0x28d8390>
140020919:     	jmp	0x140020fdc <.text+0x1ffdc>
14002091e:     	xorl	%r14d, %r14d
140020921:     	movq	%rdi, 0x808(%rbp)
140020928:     	leaq	0x298db09(%rip), %r8    # 0x1429ae438
14002092f:     	movq	%r14, %rcx
140020932:     	movq	%r12, %rdx
140020935:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14002093a:     	jmp	0x140020fdc <.text+0x1ffdc>
14002093f:     	leaq	0x29059ea(%rip), %r8    # 0x142926330
140020946:     	movl	$0x4, %ecx
14002094b:     	movq	%rsi, %rdx
14002094e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020953:     	jmp	0x140020fdc <.text+0x1ffdc>
140020958:     	movb	$0x1, 0x814(%rbp)
14002095f:     	leaq	0x2905cf2(%rip), %rcx   # 0x142926658
140020966:     	callq	0x1428d9310 <.text+0x28d8310>
14002096b:     	jmp	0x140020fdc <.text+0x1ffdc>
140020970:     	movb	$0x1, 0x814(%rbp)
140020977:     	leaq	0x2905cf2(%rip), %rcx   # 0x142926670
14002097e:     	callq	0x1428d9310 <.text+0x28d8310>
140020983:     	jmp	0x140020fdc <.text+0x1ffdc>
140020988:     	xorl	%esi, %esi
14002098a:     	leaq	0x2906a07(%rip), %r8    # 0x142927398
140020991:     	movq	%rsi, %rcx
140020994:     	movq	%r15, %rdx
140020997:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14002099c:     	jmp	0x140020fdc <.text+0x1ffdc>
1400209a1:     	leaq	0x29b4270(%rip), %r8    # 0x1429d4c18
1400209a8:     	movl	$0x4, %ecx
1400209ad:     	movq	%rbx, %rdx
1400209b0:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400209b5:     	jmp	0x140020fdc <.text+0x1ffdc>
1400209ba:     	leaq	0x2905987(%rip), %r8    # 0x142926348
1400209c1:     	movl	$0x4, %ecx
1400209c6:     	movq	%rsi, %rdx
1400209c9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400209ce:     	jmp	0x140020fdc <.text+0x1ffdc>
1400209d3:     	leaq	0x29069be(%rip), %r8    # 0x142927398
1400209da:     	movl	$0x4, %ecx
1400209df:     	movl	$0x30, %edx
1400209e4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400209e9:     	jmp	0x140020fdc <.text+0x1ffdc>
1400209ee:     	leaq	0x290596b(%rip), %r8    # 0x142926360
1400209f5:     	movl	$0x4, %ecx
1400209fa:     	movq	%rsi, %rdx
1400209fd:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020a02:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a07:     	leaq	0x290596a(%rip), %r8    # 0x142926378
140020a0e:     	movl	$0x4, %ecx
140020a13:     	movq	%rsi, %rdx
140020a16:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020a1b:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a20:     	leaq	0x2905b51(%rip), %rcx   # 0x142926578
140020a27:     	callq	0x1428d9310 <.text+0x28d8310>
140020a2c:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a31:     	xorl	%esi, %esi
140020a33:     	movb	$0x1, 0x813(%rbp)
140020a3a:     	leaq	0x2906957(%rip), %r8    # 0x142927398
140020a41:     	movq	%rsi, %rcx
140020a44:     	movq	0xf8(%rbp), %rdx
140020a4b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020a50:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a55:     	leaq	0x2905a4c(%rip), %rcx   # 0x1429264a8
140020a5c:     	callq	0x1428d9310 <.text+0x28d8310>
140020a61:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a66:     	leaq	0x2905a53(%rip), %rcx   # 0x1429264c0
140020a6d:     	callq	0x1428d9310 <.text+0x28d8310>
140020a72:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a77:     	leaq	0x2905a5a(%rip), %rcx   # 0x1429264d8
140020a7e:     	callq	0x1428d9310 <.text+0x28d8310>
140020a83:     	jmp	0x140020fdc <.text+0x1ffdc>
140020a88:     	movb	$0x1, 0x79e(%rbp)
140020a8f:     	leaq	0x2905a82(%rip), %r8    # 0x142926518
140020a96:     	movl	$0x4, %ecx
140020a9b:     	movq	%r13, %rdx
140020a9e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020aa3:     	jmp	0x140020fdc <.text+0x1ffdc>
140020aa8:     	movb	$0x1, 0x79d(%rbp)
140020aaf:     	leaq	0x2905a92(%rip), %r8    # 0x142926548
140020ab6:     	movl	$0x4, %ecx
140020abb:     	movq	%r13, %rdx
140020abe:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020ac3:     	jmp	0x140020fdc <.text+0x1ffdc>
140020ac8:     	xorl	%esi, %esi
140020aca:     	movq	0x100(%rbp), %rbx
140020ad1:     	movb	$0x1, 0x816(%rbp)
140020ad8:     	movb	$0x1, 0x815(%rbp)
140020adf:     	leaq	0x29068b2(%rip), %r8    # 0x142927398
140020ae6:     	movq	%rsi, %rcx
140020ae9:     	movq	%rbx, %rdx
140020aec:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020af1:     	jmp	0x140020fdc <.text+0x1ffdc>
140020af6:     	movb	$0x1, 0x817(%rbp)
140020afd:     	leaq	0x2905fbc(%rip), %r8    # 0x142926ac0
140020b04:     	xorl	%ecx, %ecx
140020b06:     	xorl	%edx, %edx
140020b08:     	callq	0x1428d9518 <.text+0x28d8518>
140020b0d:     	jmp	0x140020fdc <.text+0x1ffdc>
140020b12:     	movb	$0x1, 0x817(%rbp)
140020b19:     	leaq	0x2905fb8(%rip), %r8    # 0x142926ad8
140020b20:     	movl	$0x1, %ecx
140020b25:     	movl	$0x1, %edx
140020b2a:     	callq	0x1428d9518 <.text+0x28d8518>
140020b2f:     	jmp	0x140020fdc <.text+0x1ffdc>
140020b34:     	movl	$0x10, %ecx
140020b39:     	movl	$0x90, %edx
140020b3e:     	callq	0x1428d9000 <.text+0x28d8000>
140020b43:     	jmp	0x140020fdc <.text+0x1ffdc>
140020b48:     	movb	$0x1, 0x805(%rbp)
140020b4f:     	leaq	0x2911eba(%rip), %rcx   # 0x142932a10
140020b56:     	callq	0x1428d80d0 <.text+0x28d70d0>
140020b5b:     	jmp	0x140020fdc <.text+0x1ffdc>
140020b60:     	movb	$0x1, 0x817(%rbp)
140020b67:     	leaq	0x290682a(%rip), %r8    # 0x142927398
140020b6e:     	movl	$0x4, %ecx
140020b73:     	movl	$0x18, %edx
140020b78:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020b7d:     	jmp	0x140020fdc <.text+0x1ffdc>
140020b82:     	movb	$0x1, 0x817(%rbp)
140020b89:     	leaq	0x292d7f0(%rip), %r8    # 0x14294e380
140020b90:     	movq	%rdx, %rcx
140020b93:     	movq	%rdi, %rdx
140020b96:     	callq	0x1428d9518 <.text+0x28d8518>
140020b9b:     	jmp	0x140020fdc <.text+0x1ffdc>
140020ba0:     	movb	$0x1, 0x817(%rbp)
140020ba7:     	leaq	0x292d7d2(%rip), %r8    # 0x14294e380
140020bae:     	movq	%rdx, %rcx
140020bb1:     	movq	%rdi, %rdx
140020bb4:     	callq	0x1428d9518 <.text+0x28d8518>
140020bb9:     	jmp	0x140020fdc <.text+0x1ffdc>
140020bbe:     	movb	$0x1, 0x816(%rbp)
140020bc5:     	movb	$0x1, 0x815(%rbp)
140020bcc:     	leaq	0x2905e45(%rip), %r8    # 0x142926a18
140020bd3:     	xorl	%ecx, %ecx
140020bd5:     	xorl	%edx, %edx
140020bd7:     	callq	0x1428d9518 <.text+0x28d8518>
140020bdc:     	jmp	0x140020fdc <.text+0x1ffdc>
140020be1:     	movb	$0x1, 0x816(%rbp)
140020be8:     	movb	$0x1, 0x815(%rbp)
140020bef:     	leaq	0x2905e3a(%rip), %rcx   # 0x142926a30
140020bf6:     	callq	0x1428d9310 <.text+0x28d8310>
140020bfb:     	jmp	0x140020fdc <.text+0x1ffdc>
140020c00:     	xorl	%esi, %esi
140020c02:     	jmp	0x140020ca4 <.text+0x1fca4>
140020c07:     	movb	$0x1, 0x816(%rbp)
140020c0e:     	movb	$0x1, 0x815(%rbp)
140020c15:     	leaq	0x2905cb4(%rip), %rcx   # 0x1429268d0
140020c1c:     	leaq	0x2905cd5(%rip), %r8    # 0x1429268f8
140020c23:     	movl	$0x28, %edx
140020c28:     	callq	0x1428d9430 <.text+0x28d8430>
140020c2d:     	jmp	0x140020fdc <.text+0x1ffdc>
140020c32:     	xorl	%ecx, %ecx
140020c34:     	jmp	0x140020e02 <.text+0x1fe02>
140020c39:     	movb	$0x1, 0x812(%rbp)
140020c40:     	movb	$0x1, 0x811(%rbp)
140020c47:     	leaq	0x2905c0a(%rip), %r8    # 0x142926858
140020c4e:     	movl	$0x4, %ecx
140020c53:     	movq	0x7d0(%rbp), %rdx
140020c5a:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020c5f:     	jmp	0x140020fdc <.text+0x1ffdc>
140020c64:     	leaq	0x2905c05(%rip), %r8    # 0x142926870
140020c6b:     	movl	$0x4, %ecx
140020c70:     	movq	%rsi, %rdx
140020c73:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020c78:     	jmp	0x140020fdc <.text+0x1ffdc>
140020c7d:     	movb	$0x1, 0x7bd(%rbp)
140020c84:     	leaq	0x2905bb5(%rip), %r8    # 0x142926840
140020c8b:     	movl	$0x4, %ecx
140020c90:     	movq	%rsi, %rdx
140020c93:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020c98:     	jmp	0x140020fdc <.text+0x1ffdc>
140020c9d:     	movq	%rbx, 0x2e8(%rbp)
140020ca4:     	leaq	0x29066ed(%rip), %r8    # 0x142927398
140020cab:     	movq	%rsi, %rcx
140020cae:     	movq	0x2e8(%rbp), %rdx
140020cb5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020cba:     	jmp	0x140020fdc <.text+0x1ffdc>
140020cbf:     	movq	%rsi, 0xd0(%rbp)
140020cc6:     	movq	%rdi, 0xc8(%rbp)
140020ccd:     	leaq	0x29066c4(%rip), %r8    # 0x142927398
140020cd4:     	movl	$0x4, %ecx
140020cd9:     	movq	%r12, %rdx
140020cdc:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020ce1:     	jmp	0x140020fdc <.text+0x1ffdc>
140020ce6:     	leaq	0x29066ab(%rip), %r8    # 0x142927398
140020ced:     	movl	$0x4, %ecx
140020cf2:     	movq	%r12, %rdx
140020cf5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020cfa:     	jmp	0x140020fdc <.text+0x1ffdc>
140020cff:     	xorl	%ecx, %ecx
140020d01:     	jmp	0x140020e6d <.text+0x1fe6d>
140020d06:     	xorl	%esi, %esi
140020d08:     	movb	$0x1, 0x79c(%rbp)
140020d0f:     	movb	$0x1, 0x79b(%rbp)
140020d16:     	leaq	0x2906613(%rip), %r8    # 0x142927330
140020d1d:     	movq	%rsi, %rcx
140020d20:     	movq	%rdi, %rdx
140020d23:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020d28:     	jmp	0x140020fdc <.text+0x1ffdc>
140020d2d:     	xorl	%esi, %esi
140020d2f:     	movb	$0x1, 0x7c0(%rbp)
140020d36:     	leaq	0x29065f3(%rip), %r8    # 0x142927330
140020d3d:     	movq	%rsi, %rcx
140020d40:     	movq	%rdi, %rdx
140020d43:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020d48:     	jmp	0x140020fdc <.text+0x1ffdc>
140020d4d:     	xorl	%esi, %esi
140020d4f:     	movb	$0x1, 0x7bf(%rbp)
140020d56:     	leaq	0x29065d3(%rip), %r8    # 0x142927330
140020d5d:     	movq	%rsi, %rcx
140020d60:     	movq	%rdi, %rdx
140020d63:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020d68:     	jmp	0x140020fdc <.text+0x1ffdc>
140020d6d:     	xorl	%esi, %esi
140020d6f:     	movb	$0x1, 0x7be(%rbp)
140020d76:     	leaq	0x29065b3(%rip), %r8    # 0x142927330
140020d7d:     	movq	%rsi, %rcx
140020d80:     	movq	%rdi, %rdx
140020d83:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020d88:     	jmp	0x140020fdc <.text+0x1ffdc>
140020d8d:     	xorl	%esi, %esi
140020d8f:     	movb	$0x1, 0x7bc(%rbp)
140020d96:     	movb	$0x1, 0x7bb(%rbp)
140020d9d:     	leaq	0x290658c(%rip), %r8    # 0x142927330
140020da4:     	movq	%rsi, %rcx
140020da7:     	movq	%rdi, %rdx
140020daa:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020daf:     	jmp	0x140020fdc <.text+0x1ffdc>
140020db4:     	movb	$0x1, 0x816(%rbp)
140020dbb:     	movb	$0x1, 0x815(%rbp)
140020dc2:     	leaq	0x2905ad7(%rip), %r8    # 0x1429268a0
140020dc9:     	xorl	%ecx, %ecx
140020dcb:     	xorl	%edx, %edx
140020dcd:     	callq	0x1428d9518 <.text+0x28d8518>
140020dd2:     	jmp	0x140020fdc <.text+0x1ffdc>
140020dd7:     	movb	$0x1, 0x816(%rbp)
140020dde:     	movb	$0x1, 0x815(%rbp)
140020de5:     	leaq	0x2905acc(%rip), %rcx   # 0x1429268b8
140020dec:     	callq	0x1428d9310 <.text+0x28d8310>
140020df1:     	jmp	0x140020fdc <.text+0x1ffdc>
140020df6:     	movl	$0x4, %ecx
140020dfb:     	movq	%rsi, 0x2f0(%rbp)
140020e02:     	leaq	0x290658f(%rip), %r8    # 0x142927398
140020e09:     	movq	0x2f0(%rbp), %rdx
140020e10:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020e15:     	jmp	0x140020fdc <.text+0x1ffdc>
140020e1a:     	movq	0x370(%rbp), %rax
140020e21:     	movq	%rax, 0xe0(%rbp)
140020e28:     	movq	%rdi, 0xd8(%rbp)
140020e2f:     	leaq	0x2906562(%rip), %r8    # 0x142927398
140020e36:     	movl	$0x4, %ecx
140020e3b:     	movq	%r12, %rdx
140020e3e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020e43:     	jmp	0x140020fdc <.text+0x1ffdc>
140020e48:     	leaq	0x2906549(%rip), %r8    # 0x142927398
140020e4f:     	movl	$0x4, %ecx
140020e54:     	movq	%r12, %rdx
140020e57:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020e5c:     	jmp	0x140020fdc <.text+0x1ffdc>
140020e61:     	movl	$0x4, %ecx
140020e66:     	movq	%r14, 0x2f8(%rbp)
140020e6d:     	leaq	0x2906524(%rip), %r8    # 0x142927398
140020e74:     	movq	0x2f8(%rbp), %rdx
140020e7b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020e80:     	jmp	0x140020fdc <.text+0x1ffdc>
140020e85:     	leaq	0x290650c(%rip), %r8    # 0x142927398
140020e8c:     	movl	$0x4, %ecx
140020e91:     	movq	%r15, %rdx
140020e94:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020e99:     	jmp	0x140020fdc <.text+0x1ffdc>
140020e9e:     	leaq	0x29064f3(%rip), %r8    # 0x142927398
140020ea5:     	movl	$0x4, %ecx
140020eaa:     	movq	%r15, %rdx
140020ead:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140020eb2:     	jmp	0x140020fdc <.text+0x1ffdc>
140020eb7:     	movb	$0x1, 0x816(%rbp)
140020ebe:     	movb	$0x1, 0x815(%rbp)
140020ec5:     	leaq	0x292d4b4(%rip), %r8    # 0x14294e380
140020ecc:     	movq	%rdx, %rcx
140020ecf:     	movq	%rdi, %rdx
140020ed2:     	callq	0x1428d9518 <.text+0x28d8518>
140020ed7:     	jmp	0x140020fdc <.text+0x1ffdc>
140020edc:     	movb	$0x1, 0x816(%rbp)
140020ee3:     	movb	$0x1, 0x815(%rbp)
140020eea:     	movl	$0x4, %ecx
140020eef:     	movl	$0x24, %edx
140020ef4:     	callq	0x1428d9000 <.text+0x28d8000>
140020ef9:     	jmp	0x140020fdc <.text+0x1ffdc>
140020efe:     	movb	$0x1, 0x816(%rbp)
140020f05:     	movb	$0x1, 0x815(%rbp)
140020f0c:     	leaq	0x29059fd(%rip), %r8    # 0x142926910
140020f13:     	movl	$0x1, %ecx
140020f18:     	callq	0x1428d9518 <.text+0x28d8518>
140020f1d:     	jmp	0x140020fdc <.text+0x1ffdc>
140020f22:     	movb	$0x1, 0x816(%rbp)
140020f29:     	movb	$0x1, 0x815(%rbp)
140020f30:     	leaq	0x29059f1(%rip), %r8    # 0x142926928
140020f37:     	movl	$0x1, %ecx
140020f3c:     	callq	0x1428d9518 <.text+0x28d8518>
140020f41:     	jmp	0x140020fdc <.text+0x1ffdc>
140020f46:     	movb	$0x1, 0x816(%rbp)
140020f4d:     	movb	$0x1, 0x815(%rbp)
140020f54:     	leaq	0x29059e5(%rip), %r8    # 0x142926940
140020f5b:     	movl	$0x1, %ecx
140020f60:     	callq	0x1428d9518 <.text+0x28d8518>
140020f65:     	jmp	0x140020fdc <.text+0x1ffdc>
140020f67:     	movb	$0x1, 0x816(%rbp)
140020f6e:     	movb	$0x1, 0x815(%rbp)
140020f75:     	leaq	0x292d404(%rip), %r8    # 0x14294e380
140020f7c:     	movq	%rdx, %rcx
140020f7f:     	movq	%rdi, %rdx
140020f82:     	callq	0x1428d9518 <.text+0x28d8518>
140020f87:     	jmp	0x140020fdc <.text+0x1ffdc>
140020f89:     	movb	$0x1, 0x816(%rbp)
140020f90:     	movb	$0x1, 0x815(%rbp)
140020f97:     	leaq	0x29059ba(%rip), %r8    # 0x142926958
140020f9e:     	movl	$0x1, %ecx
140020fa3:     	callq	0x1428d9518 <.text+0x28d8518>
140020fa8:     	jmp	0x140020fdc <.text+0x1ffdc>
140020faa:     	movb	$0x1, 0x816(%rbp)
140020fb1:     	movb	$0x1, 0x815(%rbp)
140020fb8:     	leaq	0x292d3c1(%rip), %r8    # 0x14294e380
140020fbf:     	movq	%rdx, %rcx
140020fc2:     	movq	%rdi, %rdx
140020fc5:     	callq	0x1428d9518 <.text+0x28d8518>
140020fca:     	jmp	0x140020fdc <.text+0x1ffdc>
140020fcc:     	leaq	0x29059b5(%rip), %r8    # 0x142926988
140020fd3:     	xorl	%ecx, %ecx
140020fd5:     	xorl	%edx, %edx
140020fd7:     	callq	0x1428d9518 <.text+0x28d8518>
140020fdc:     	ud2
140020fde:     	nop
140020fe0:     	movq	%rdx, 0x10(%rsp)
140020fe5:     	pushq	%rbp
140020fe6:     	pushq	%r15
140020fe8:     	pushq	%r14
140020fea:     	pushq	%r13
140020fec:     	pushq	%r12
140020fee:     	pushq	%rsi
140020fef:     	pushq	%rdi
140020ff0:     	pushq	%rbx
140020ff1:     	subq	$0x138, %rsp            # imm = 0x138
140020ff8:     	leaq	0x80(%rdx), %rbp
140020fff:     	movaps	%xmm15, 0x90(%rsp)
140021008:     	movaps	%xmm14, 0xa0(%rsp)
140021011:     	movaps	%xmm13, 0xb0(%rsp)
14002101a:     	movaps	%xmm12, 0xc0(%rsp)
140021023:     	movaps	%xmm11, 0xd0(%rsp)
14002102c:     	movaps	%xmm10, 0xe0(%rsp)
140021035:     	movaps	%xmm9, 0xf0(%rsp)
14002103e:     	movaps	%xmm8, 0x100(%rsp)
140021047:     	movaps	%xmm7, 0x110(%rsp)
14002104f:     	movaps	%xmm6, 0x120(%rsp)
140021057:     	movq	0x3e0(%rbp), %rdx
14002105e:     	testq	%rdx, %rdx
140021061:     	je	0x140021079 <.text+0x20079>
140021063:     	movq	0x3e8(%rbp), %rcx
14002106a:     	shlq	$0x2, %rdx
14002106e:     	movl	$0x4, %r8d
140021074:     	callq	0x140613c20 <.text+0x612c20>
140021079:     	movaps	0x120(%rsp), %xmm6
140021081:     	movaps	0x110(%rsp), %xmm7
140021089:     	movaps	0x100(%rsp), %xmm8
140021092:     	movaps	0xf0(%rsp), %xmm9
14002109b:     	movaps	0xe0(%rsp), %xmm10
1400210a4:     	movaps	0xd0(%rsp), %xmm11
1400210ad:     	movaps	0xc0(%rsp), %xmm12
1400210b6:     	movaps	0xb0(%rsp), %xmm13
1400210bf:     	movaps	0xa0(%rsp), %xmm14
1400210c8:     	movaps	0x90(%rsp), %xmm15
1400210d1:     	addq	$0x138, %rsp            # imm = 0x138
1400210d8:     	popq	%rbx
1400210d9:     	popq	%rdi
1400210da:     	popq	%rsi
1400210db:     	popq	%r12
1400210dd:     	popq	%r13
1400210df:     	popq	%r14
1400210e1:     	popq	%r15
1400210e3:     	popq	%rbp
1400210e4:     	retq
1400210e5:     	nopw	%cs:(%rax,%rax)
1400210f0:     	movq	%rdx, 0x10(%rsp)
1400210f5:     	pushq	%rbp
1400210f6:     	pushq	%r15
1400210f8:     	pushq	%r14
1400210fa:     	pushq	%r13
1400210fc:     	pushq	%r12
1400210fe:     	pushq	%rsi
1400210ff:     	pushq	%rdi
140021100:     	pushq	%rbx
140021101:     	subq	$0x138, %rsp            # imm = 0x138
140021108:     	leaq	0x80(%rdx), %rbp
14002110f:     	movaps	%xmm15, 0x90(%rsp)
140021118:     	movaps	%xmm14, 0xa0(%rsp)
140021121:     	movaps	%xmm13, 0xb0(%rsp)
14002112a:     	movaps	%xmm12, 0xc0(%rsp)
140021133:     	movaps	%xmm11, 0xd0(%rsp)
14002113c:     	movaps	%xmm10, 0xe0(%rsp)
140021145:     	movaps	%xmm9, 0xf0(%rsp)
14002114e:     	movaps	%xmm8, 0x100(%rsp)
140021157:     	movaps	%xmm7, 0x110(%rsp)
14002115f:     	movaps	%xmm6, 0x120(%rsp)
140021167:     	movq	0x508(%rbp), %rdx
14002116e:     	testq	%rdx, %rdx
140021171:     	je	0x140021189 <.text+0x20189>
140021173:     	movq	0x510(%rbp), %rcx
14002117a:     	shlq	$0x2, %rdx
14002117e:     	movl	$0x4, %r8d
140021184:     	callq	0x140613c20 <.text+0x612c20>
140021189:     	movaps	0x120(%rsp), %xmm6
140021191:     	movaps	0x110(%rsp), %xmm7
140021199:     	movaps	0x100(%rsp), %xmm8
1400211a2:     	movaps	0xf0(%rsp), %xmm9
1400211ab:     	movaps	0xe0(%rsp), %xmm10
1400211b4:     	movaps	0xd0(%rsp), %xmm11
1400211bd:     	movaps	0xc0(%rsp), %xmm12
1400211c6:     	movaps	0xb0(%rsp), %xmm13
1400211cf:     	movaps	0xa0(%rsp), %xmm14
1400211d8:     	movaps	0x90(%rsp), %xmm15
1400211e1:     	addq	$0x138, %rsp            # imm = 0x138
1400211e8:     	popq	%rbx
1400211e9:     	popq	%rdi
1400211ea:     	popq	%rsi
1400211eb:     	popq	%r12
1400211ed:     	popq	%r13
1400211ef:     	popq	%r14
1400211f1:     	popq	%r15
1400211f3:     	popq	%rbp
1400211f4:     	retq
1400211f5:     	nopw	%cs:(%rax,%rax)
140021200:     	movq	%rdx, 0x10(%rsp)
140021205:     	pushq	%rbp
140021206:     	pushq	%r15
140021208:     	pushq	%r14
14002120a:     	pushq	%r13
14002120c:     	pushq	%r12
14002120e:     	pushq	%rsi
14002120f:     	pushq	%rdi
140021210:     	pushq	%rbx
140021211:     	subq	$0x138, %rsp            # imm = 0x138
140021218:     	leaq	0x80(%rdx), %rbp
14002121f:     	movaps	%xmm15, 0x90(%rsp)
140021228:     	movaps	%xmm14, 0xa0(%rsp)
140021231:     	movaps	%xmm13, 0xb0(%rsp)
14002123a:     	movaps	%xmm12, 0xc0(%rsp)
140021243:     	movaps	%xmm11, 0xd0(%rsp)
14002124c:     	movaps	%xmm10, 0xe0(%rsp)
140021255:     	movaps	%xmm9, 0xf0(%rsp)
14002125e:     	movaps	%xmm8, 0x100(%rsp)
140021267:     	movaps	%xmm7, 0x110(%rsp)
14002126f:     	movaps	%xmm6, 0x120(%rsp)
140021277:     	leaq	0x6c0(%rbp), %rcx
14002127e:     	callq	0x14014ba20 <.text+0x14aa20>
140021283:     	movb	$0x1, 0x816(%rbp)
14002128a:     	movb	$0x1, 0x815(%rbp)
140021291:     	movaps	0x120(%rsp), %xmm6
140021299:     	movaps	0x110(%rsp), %xmm7
1400212a1:     	movaps	0x100(%rsp), %xmm8
1400212aa:     	movaps	0xf0(%rsp), %xmm9
1400212b3:     	movaps	0xe0(%rsp), %xmm10
1400212bc:     	movaps	0xd0(%rsp), %xmm11
1400212c5:     	movdqa	0xc0(%rsp), %xmm12
1400212cf:     	movdqa	0xb0(%rsp), %xmm13
1400212d9:     	movaps	0xa0(%rsp), %xmm14
1400212e2:     	movdqa	0x90(%rsp), %xmm15
1400212ec:     	addq	$0x138, %rsp            # imm = 0x138
1400212f3:     	popq	%rbx
1400212f4:     	popq	%rdi
1400212f5:     	popq	%rsi
1400212f6:     	popq	%r12
1400212f8:     	popq	%r13
1400212fa:     	popq	%r14
1400212fc:     	popq	%r15
1400212fe:     	popq	%rbp
1400212ff:     	retq
140021300:     	movq	%rdx, 0x10(%rsp)
140021305:     	pushq	%rbp
140021306:     	pushq	%r15
140021308:     	pushq	%r14
14002130a:     	pushq	%r13
14002130c:     	pushq	%r12
14002130e:     	pushq	%rsi
14002130f:     	pushq	%rdi
140021310:     	pushq	%rbx
140021311:     	subq	$0x138, %rsp            # imm = 0x138
140021318:     	leaq	0x80(%rdx), %rbp
14002131f:     	movdqa	%xmm15, 0x90(%rsp)
140021329:     	movaps	%xmm14, 0xa0(%rsp)
140021332:     	movdqa	%xmm13, 0xb0(%rsp)
14002133c:     	movdqa	%xmm12, 0xc0(%rsp)
140021346:     	movaps	%xmm11, 0xd0(%rsp)
14002134f:     	movaps	%xmm10, 0xe0(%rsp)
140021358:     	movaps	%xmm9, 0xf0(%rsp)
140021361:     	movaps	%xmm8, 0x100(%rsp)
14002136a:     	movaps	%xmm7, 0x110(%rsp)
140021372:     	movaps	%xmm6, 0x120(%rsp)
14002137a:     	movb	$0x0, 0x816(%rbp)
140021381:     	movb	$0x0, 0x815(%rbp)
140021388:     	leaq	0x6c0(%rbp), %rcx
14002138f:     	callq	0x140609eb0 <.text+0x608eb0>
140021394:     	movb	$0x0, 0x816(%rbp)
14002139b:     	movb	$0x0, 0x815(%rbp)
1400213a2:     	movaps	0x120(%rsp), %xmm6
1400213aa:     	movaps	0x110(%rsp), %xmm7
1400213b2:     	movaps	0x100(%rsp), %xmm8
1400213bb:     	movaps	0xf0(%rsp), %xmm9
1400213c4:     	movaps	0xe0(%rsp), %xmm10
1400213cd:     	movaps	0xd0(%rsp), %xmm11
1400213d6:     	movdqa	0xc0(%rsp), %xmm12
1400213e0:     	movdqa	0xb0(%rsp), %xmm13
1400213ea:     	movaps	0xa0(%rsp), %xmm14
1400213f3:     	movdqa	0x90(%rsp), %xmm15
1400213fd:     	addq	$0x138, %rsp            # imm = 0x138
140021404:     	popq	%rbx
140021405:     	popq	%rdi
140021406:     	popq	%rsi
140021407:     	popq	%r12
140021409:     	popq	%r13
14002140b:     	popq	%r14
14002140d:     	popq	%r15
14002140f:     	popq	%rbp
140021410:     	retq
140021411:     	nopw	%cs:(%rax,%rax)
140021420:     	movq	%rdx, 0x10(%rsp)
140021425:     	pushq	%rbp
140021426:     	pushq	%r15
140021428:     	pushq	%r14
14002142a:     	pushq	%r13
14002142c:     	pushq	%r12
14002142e:     	pushq	%rsi
14002142f:     	pushq	%rdi
140021430:     	pushq	%rbx
140021431:     	subq	$0x138, %rsp            # imm = 0x138
140021438:     	leaq	0x80(%rdx), %rbp
14002143f:     	movdqa	%xmm15, 0x90(%rsp)
140021449:     	movaps	%xmm14, 0xa0(%rsp)
140021452:     	movdqa	%xmm13, 0xb0(%rsp)
14002145c:     	movdqa	%xmm12, 0xc0(%rsp)
140021466:     	movaps	%xmm11, 0xd0(%rsp)
14002146f:     	movaps	%xmm10, 0xe0(%rsp)
140021478:     	movaps	%xmm9, 0xf0(%rsp)
140021481:     	movaps	%xmm8, 0x100(%rsp)
14002148a:     	movaps	%xmm7, 0x110(%rsp)
140021492:     	movaps	%xmm6, 0x120(%rsp)
14002149a:     	movb	$0x1, 0x816(%rbp)
1400214a1:     	movb	$0x1, 0x815(%rbp)
1400214a8:     	leaq	0x6c0(%rbp), %rcx
1400214af:     	callq	0x140609eb0 <.text+0x608eb0>
1400214b4:     	movb	$0x1, 0x816(%rbp)
1400214bb:     	movb	$0x1, 0x815(%rbp)
1400214c2:     	movaps	0x120(%rsp), %xmm6
1400214ca:     	movaps	0x110(%rsp), %xmm7
1400214d2:     	movaps	0x100(%rsp), %xmm8
1400214db:     	movaps	0xf0(%rsp), %xmm9
1400214e4:     	movaps	0xe0(%rsp), %xmm10
1400214ed:     	movaps	0xd0(%rsp), %xmm11
1400214f6:     	movdqa	0xc0(%rsp), %xmm12
140021500:     	movdqa	0xb0(%rsp), %xmm13
14002150a:     	movaps	0xa0(%rsp), %xmm14
140021513:     	movdqa	0x90(%rsp), %xmm15
14002151d:     	addq	$0x138, %rsp            # imm = 0x138
140021524:     	popq	%rbx
140021525:     	popq	%rdi
140021526:     	popq	%rsi
140021527:     	popq	%r12
140021529:     	popq	%r13
14002152b:     	popq	%r14
14002152d:     	popq	%r15
14002152f:     	popq	%rbp
140021530:     	retq
140021531:     	nopw	%cs:(%rax,%rax)
140021540:     	movq	%rdx, 0x10(%rsp)
140021545:     	pushq	%rbp
140021546:     	pushq	%r15
140021548:     	pushq	%r14
14002154a:     	pushq	%r13
14002154c:     	pushq	%r12
14002154e:     	pushq	%rsi
14002154f:     	pushq	%rdi
140021550:     	pushq	%rbx
140021551:     	subq	$0x138, %rsp            # imm = 0x138
140021558:     	leaq	0x80(%rdx), %rbp
14002155f:     	movdqa	%xmm15, 0x90(%rsp)
140021569:     	movaps	%xmm14, 0xa0(%rsp)
140021572:     	movdqa	%xmm13, 0xb0(%rsp)
14002157c:     	movdqa	%xmm12, 0xc0(%rsp)
140021586:     	movaps	%xmm11, 0xd0(%rsp)
14002158f:     	movaps	%xmm10, 0xe0(%rsp)
140021598:     	movaps	%xmm9, 0xf0(%rsp)
1400215a1:     	movaps	%xmm8, 0x100(%rsp)
1400215aa:     	movaps	%xmm7, 0x110(%rsp)
1400215b2:     	movaps	%xmm6, 0x120(%rsp)
1400215ba:     	movb	$0x0, 0x813(%rbp)
1400215c1:     	leaq	0x590(%rbp), %rcx
1400215c8:     	callq	0x140607a30 <.text+0x606a30>
1400215cd:     	movb	$0x0, 0x813(%rbp)
1400215d4:     	movaps	0x120(%rsp), %xmm6
1400215dc:     	movaps	0x110(%rsp), %xmm7
1400215e4:     	movaps	0x100(%rsp), %xmm8
1400215ed:     	movaps	0xf0(%rsp), %xmm9
1400215f6:     	movaps	0xe0(%rsp), %xmm10
1400215ff:     	movaps	0xd0(%rsp), %xmm11
140021608:     	movdqa	0xc0(%rsp), %xmm12
140021612:     	movdqa	0xb0(%rsp), %xmm13
14002161c:     	movaps	0xa0(%rsp), %xmm14
140021625:     	movdqa	0x90(%rsp), %xmm15
14002162f:     	addq	$0x138, %rsp            # imm = 0x138
140021636:     	popq	%rbx
140021637:     	popq	%rdi
140021638:     	popq	%rsi
140021639:     	popq	%r12
14002163b:     	popq	%r13
14002163d:     	popq	%r14
14002163f:     	popq	%r15
140021641:     	popq	%rbp
140021642:     	retq
140021643:     	nopw	%cs:(%rax,%rax)
140021650:     	movq	%rdx, 0x10(%rsp)
140021655:     	pushq	%rbp
140021656:     	pushq	%r15
140021658:     	pushq	%r14
14002165a:     	pushq	%r13
14002165c:     	pushq	%r12
14002165e:     	pushq	%rsi
14002165f:     	pushq	%rdi
140021660:     	pushq	%rbx
140021661:     	subq	$0x138, %rsp            # imm = 0x138
140021668:     	leaq	0x80(%rdx), %rbp
14002166f:     	movdqa	%xmm15, 0x90(%rsp)
140021679:     	movaps	%xmm14, 0xa0(%rsp)
140021682:     	movdqa	%xmm13, 0xb0(%rsp)
14002168c:     	movdqa	%xmm12, 0xc0(%rsp)
140021696:     	movaps	%xmm11, 0xd0(%rsp)
14002169f:     	movaps	%xmm10, 0xe0(%rsp)
1400216a8:     	movaps	%xmm9, 0xf0(%rsp)
1400216b1:     	movaps	%xmm8, 0x100(%rsp)
1400216ba:     	movaps	%xmm7, 0x110(%rsp)
1400216c2:     	movaps	%xmm6, 0x120(%rsp)
1400216ca:     	cmpq	$0x0, 0x7d0(%rbp)
1400216d2:     	jne	0x140021796 <.text+0x20796>
1400216d8:     	cmpq	$0x0, 0x7a0(%rbp)
1400216e0:     	jne	0x1400217c5 <.text+0x207c5>
1400216e6:     	cmpq	$0x0, 0x808(%rbp)
1400216ee:     	je	0x14002170d <.text+0x2070d>
1400216f0:     	movq	0x808(%rbp), %rdx
1400216f7:     	shlq	$0x2, %rdx
1400216fb:     	movl	$0x4, %r8d
140021701:     	movq	0x6b0(%rbp), %rcx
140021708:     	callq	0x140613c20 <.text+0x612c20>
14002170d:     	leaq	0x590(%rbp), %rcx
140021714:     	callq	0x14014ba20 <.text+0x14aa20>
140021719:     	movb	$0x0, 0x816(%rbp)
140021720:     	movb	$0x0, 0x815(%rbp)
140021727:     	movaps	0x120(%rsp), %xmm6
14002172f:     	movaps	0x110(%rsp), %xmm7
140021737:     	movaps	0x100(%rsp), %xmm8
140021740:     	movaps	0xf0(%rsp), %xmm9
140021749:     	movaps	0xe0(%rsp), %xmm10
140021752:     	movaps	0xd0(%rsp), %xmm11
14002175b:     	movdqa	0xc0(%rsp), %xmm12
140021765:     	movdqa	0xb0(%rsp), %xmm13
14002176f:     	movaps	0xa0(%rsp), %xmm14
140021778:     	movdqa	0x90(%rsp), %xmm15
140021782:     	addq	$0x138, %rsp            # imm = 0x138
140021789:     	popq	%rbx
14002178a:     	popq	%rdi
14002178b:     	popq	%rsi
14002178c:     	popq	%r12
14002178e:     	popq	%r13
140021790:     	popq	%r14
140021792:     	popq	%r15
140021794:     	popq	%rbp
140021795:     	retq
140021796:     	movq	0x7d0(%rbp), %rax
14002179d:     	leaq	(,%rax,4), %rdx
1400217a5:     	movl	$0x4, %r8d
1400217ab:     	movq	0x7f0(%rbp), %rcx
1400217b2:     	callq	0x140613c20 <.text+0x612c20>
1400217b7:     	cmpq	$0x0, 0x7a0(%rbp)
1400217bf:     	je	0x1400216e6 <.text+0x206e6>
1400217c5:     	movq	0x7a0(%rbp), %rdx
1400217cc:     	shlq	$0x3, %rdx
1400217d0:     	movl	$0x4, %r8d
1400217d6:     	movq	0x7d8(%rbp), %rcx
1400217dd:     	callq	0x140613c20 <.text+0x612c20>
1400217e2:     	cmpq	$0x0, 0x808(%rbp)
1400217ea:     	jne	0x1400216f0 <.text+0x206f0>
1400217f0:     	jmp	0x14002170d <.text+0x2070d>
1400217f5:     	nopw	%cs:(%rax,%rax)
140021800:     	movq	%rdx, 0x10(%rsp)
140021805:     	pushq	%rbp
140021806:     	pushq	%r15
140021808:     	pushq	%r14
14002180a:     	pushq	%r13
14002180c:     	pushq	%r12
14002180e:     	pushq	%rsi
14002180f:     	pushq	%rdi
140021810:     	pushq	%rbx
140021811:     	subq	$0x138, %rsp            # imm = 0x138
140021818:     	leaq	0x80(%rdx), %rbp
14002181f:     	movaps	%xmm15, 0x90(%rsp)
140021828:     	movaps	%xmm14, 0xa0(%rsp)
140021831:     	movaps	%xmm13, 0xb0(%rsp)
14002183a:     	movaps	%xmm12, 0xc0(%rsp)
140021843:     	movaps	%xmm11, 0xd0(%rsp)
14002184c:     	movaps	%xmm10, 0xe0(%rsp)
140021855:     	movaps	%xmm9, 0xf0(%rsp)
14002185e:     	movaps	%xmm8, 0x100(%rsp)
140021867:     	movaps	%xmm7, 0x110(%rsp)
14002186f:     	movaps	%xmm6, 0x120(%rsp)
140021877:     	movb	$0x0, 0x7be(%rbp)
14002187e:     	leaq	0x6c0(%rbp), %rcx
140021885:     	callq	0x140607a30 <.text+0x606a30>
14002188a:     	movb	$0x0, 0x7be(%rbp)
140021891:     	movaps	0x120(%rsp), %xmm6
140021899:     	movaps	0x110(%rsp), %xmm7
1400218a1:     	movaps	0x100(%rsp), %xmm8
1400218aa:     	movaps	0xf0(%rsp), %xmm9
1400218b3:     	movaps	0xe0(%rsp), %xmm10
1400218bc:     	movaps	0xd0(%rsp), %xmm11
1400218c5:     	movaps	0xc0(%rsp), %xmm12
1400218ce:     	movaps	0xb0(%rsp), %xmm13
1400218d7:     	movaps	0xa0(%rsp), %xmm14
1400218e0:     	movaps	0x90(%rsp), %xmm15
1400218e9:     	addq	$0x138, %rsp            # imm = 0x138
1400218f0:     	popq	%rbx
1400218f1:     	popq	%rdi
1400218f2:     	popq	%rsi
1400218f3:     	popq	%r12
1400218f5:     	popq	%r13
1400218f7:     	popq	%r14
1400218f9:     	popq	%r15
1400218fb:     	popq	%rbp
1400218fc:     	retq
1400218fd:     	nopl	(%rax)
140021900:     	movq	%rdx, 0x10(%rsp)
140021905:     	pushq	%rbp
140021906:     	pushq	%r15
140021908:     	pushq	%r14
14002190a:     	pushq	%r13
14002190c:     	pushq	%r12
14002190e:     	pushq	%rsi
14002190f:     	pushq	%rdi
140021910:     	pushq	%rbx
140021911:     	subq	$0x138, %rsp            # imm = 0x138
140021918:     	leaq	0x80(%rdx), %rbp
14002191f:     	movdqa	%xmm15, 0x90(%rsp)
140021929:     	movaps	%xmm14, 0xa0(%rsp)
140021932:     	movdqa	%xmm13, 0xb0(%rsp)
14002193c:     	movdqa	%xmm12, 0xc0(%rsp)
140021946:     	movaps	%xmm11, 0xd0(%rsp)
14002194f:     	movaps	%xmm10, 0xe0(%rsp)
140021958:     	movaps	%xmm9, 0xf0(%rsp)
140021961:     	movaps	%xmm8, 0x100(%rsp)
14002196a:     	movaps	%xmm7, 0x110(%rsp)
140021972:     	movaps	%xmm6, 0x120(%rsp)
14002197a:     	leaq	0x590(%rbp), %rcx
140021981:     	callq	0x140609eb0 <.text+0x608eb0>
140021986:     	movaps	0x120(%rsp), %xmm6
14002198e:     	movaps	0x110(%rsp), %xmm7
140021996:     	movaps	0x100(%rsp), %xmm8
14002199f:     	movaps	0xf0(%rsp), %xmm9
1400219a8:     	movaps	0xe0(%rsp), %xmm10
1400219b1:     	movaps	0xd0(%rsp), %xmm11
1400219ba:     	movdqa	0xc0(%rsp), %xmm12
1400219c4:     	movdqa	0xb0(%rsp), %xmm13
1400219ce:     	movaps	0xa0(%rsp), %xmm14
1400219d7:     	movdqa	0x90(%rsp), %xmm15
1400219e1:     	addq	$0x138, %rsp            # imm = 0x138
1400219e8:     	popq	%rbx
1400219e9:     	popq	%rdi
1400219ea:     	popq	%rsi
1400219eb:     	popq	%r12
1400219ed:     	popq	%r13
1400219ef:     	popq	%r14
1400219f1:     	popq	%r15
1400219f3:     	popq	%rbp
1400219f4:     	retq
1400219f5:     	nopw	%cs:(%rax,%rax)
140021a00:     	movq	%rdx, 0x10(%rsp)
140021a05:     	pushq	%rbp
140021a06:     	pushq	%r15
140021a08:     	pushq	%r14
140021a0a:     	pushq	%r13
140021a0c:     	pushq	%r12
140021a0e:     	pushq	%rsi
140021a0f:     	pushq	%rdi
140021a10:     	pushq	%rbx
140021a11:     	subq	$0x138, %rsp            # imm = 0x138
140021a18:     	leaq	0x80(%rdx), %rbp
140021a1f:     	movaps	%xmm15, 0x90(%rsp)
140021a28:     	movaps	%xmm14, 0xa0(%rsp)
140021a31:     	movaps	%xmm13, 0xb0(%rsp)
140021a3a:     	movaps	%xmm12, 0xc0(%rsp)
140021a43:     	movaps	%xmm11, 0xd0(%rsp)
140021a4c:     	movaps	%xmm10, 0xe0(%rsp)
140021a55:     	movaps	%xmm9, 0xf0(%rsp)
140021a5e:     	movaps	%xmm8, 0x100(%rsp)
140021a67:     	movaps	%xmm7, 0x110(%rsp)
140021a6f:     	movaps	%xmm6, 0x120(%rsp)
140021a77:     	movb	$0x0, 0x812(%rbp)
140021a7e:     	movb	$0x0, 0x811(%rbp)
140021a85:     	leaq	0x6c0(%rbp), %rcx
140021a8c:     	callq	0x140609eb0 <.text+0x608eb0>
140021a91:     	movb	$0x0, 0x812(%rbp)
140021a98:     	movb	$0x0, 0x811(%rbp)
140021a9f:     	movaps	0x120(%rsp), %xmm6
140021aa7:     	movaps	0x110(%rsp), %xmm7
140021aaf:     	movaps	0x100(%rsp), %xmm8
140021ab8:     	movaps	0xf0(%rsp), %xmm9
140021ac1:     	movaps	0xe0(%rsp), %xmm10
140021aca:     	movapd	0xd0(%rsp), %xmm11
140021ad4:     	movaps	0xc0(%rsp), %xmm12
140021add:     	movaps	0xb0(%rsp), %xmm13
140021ae6:     	movaps	0xa0(%rsp), %xmm14
140021aef:     	movaps	0x90(%rsp), %xmm15
140021af8:     	addq	$0x138, %rsp            # imm = 0x138
140021aff:     	popq	%rbx
140021b00:     	popq	%rdi
140021b01:     	popq	%rsi
140021b02:     	popq	%r12
140021b04:     	popq	%r13
140021b06:     	popq	%r14
140021b08:     	popq	%r15
140021b0a:     	popq	%rbp
140021b0b:     	retq
140021b0c:     	nopl	(%rax)
140021b10:     	movq	%rdx, 0x10(%rsp)
140021b15:     	pushq	%rbp
140021b16:     	pushq	%r15
140021b18:     	pushq	%r14
140021b1a:     	pushq	%r13
140021b1c:     	pushq	%r12
140021b1e:     	pushq	%rsi
140021b1f:     	pushq	%rdi
140021b20:     	pushq	%rbx
140021b21:     	subq	$0x138, %rsp            # imm = 0x138
140021b28:     	leaq	0x80(%rdx), %rbp
140021b2f:     	movdqa	%xmm15, 0x90(%rsp)
140021b39:     	movaps	%xmm14, 0xa0(%rsp)
140021b42:     	movdqa	%xmm13, 0xb0(%rsp)
140021b4c:     	movdqa	%xmm12, 0xc0(%rsp)
140021b56:     	movaps	%xmm11, 0xd0(%rsp)
140021b5f:     	movaps	%xmm10, 0xe0(%rsp)
140021b68:     	movaps	%xmm9, 0xf0(%rsp)
140021b71:     	movaps	%xmm8, 0x100(%rsp)
140021b7a:     	movaps	%xmm7, 0x110(%rsp)
140021b82:     	movaps	%xmm6, 0x120(%rsp)
140021b8a:     	cmpl	$0xb, 0x140(%rbp)
140021b91:     	je	0x140021c0e <.text+0x20c0e>
140021b93:     	leaq	0x140(%rbp), %rcx
140021b9a:     	callq	0x1400297d0 <.text+0x287d0>
140021b9f:     	movaps	0x120(%rsp), %xmm6
140021ba7:     	movaps	0x110(%rsp), %xmm7
140021baf:     	movaps	0x100(%rsp), %xmm8
140021bb8:     	movaps	0xf0(%rsp), %xmm9
140021bc1:     	movaps	0xe0(%rsp), %xmm10
140021bca:     	movaps	0xd0(%rsp), %xmm11
140021bd3:     	movdqa	0xc0(%rsp), %xmm12
140021bdd:     	movdqa	0xb0(%rsp), %xmm13
140021be7:     	movaps	0xa0(%rsp), %xmm14
140021bf0:     	movdqa	0x90(%rsp), %xmm15
140021bfa:     	addq	$0x138, %rsp            # imm = 0x138
140021c01:     	popq	%rbx
140021c02:     	popq	%rdi
140021c03:     	popq	%rsi
140021c04:     	popq	%r12
140021c06:     	popq	%r13
140021c08:     	popq	%r14
140021c0a:     	popq	%r15
140021c0c:     	popq	%rbp
140021c0d:     	retq
140021c0e:     	movaps	0x120(%rsp), %xmm6
140021c16:     	movaps	0x110(%rsp), %xmm7
140021c1e:     	movaps	0x100(%rsp), %xmm8
140021c27:     	movaps	0xf0(%rsp), %xmm9
140021c30:     	movaps	0xe0(%rsp), %xmm10
140021c39:     	movaps	0xd0(%rsp), %xmm11
140021c42:     	movdqa	0xc0(%rsp), %xmm12
140021c4c:     	movdqa	0xb0(%rsp), %xmm13
140021c56:     	movaps	0xa0(%rsp), %xmm14
140021c5f:     	movdqa	0x90(%rsp), %xmm15
140021c69:     	addq	$0x138, %rsp            # imm = 0x138
140021c70:     	popq	%rbx
140021c71:     	popq	%rdi
140021c72:     	popq	%rsi
140021c73:     	popq	%r12
140021c75:     	popq	%r13
140021c77:     	popq	%r14
140021c79:     	popq	%r15
140021c7b:     	popq	%rbp
140021c7c:     	retq
140021c7d:     	nopl	(%rax)
140021c80:     	movq	%rdx, 0x10(%rsp)
140021c85:     	pushq	%rbp
140021c86:     	pushq	%r15
140021c88:     	pushq	%r14
140021c8a:     	pushq	%r13
140021c8c:     	pushq	%r12
140021c8e:     	pushq	%rsi
140021c8f:     	pushq	%rdi
140021c90:     	pushq	%rbx
140021c91:     	subq	$0x138, %rsp            # imm = 0x138
140021c98:     	leaq	0x80(%rdx), %rbp
140021c9f:     	movaps	%xmm15, 0x90(%rsp)
140021ca8:     	movaps	%xmm14, 0xa0(%rsp)
140021cb1:     	movaps	%xmm13, 0xb0(%rsp)
140021cba:     	movaps	%xmm12, 0xc0(%rsp)
140021cc3:     	movaps	%xmm11, 0xd0(%rsp)
140021ccc:     	movaps	%xmm10, 0xe0(%rsp)
140021cd5:     	movaps	%xmm9, 0xf0(%rsp)
140021cde:     	movaps	%xmm8, 0x100(%rsp)
140021ce7:     	movaps	%xmm7, 0x110(%rsp)
140021cef:     	movaps	%xmm6, 0x120(%rsp)
140021cf7:     	leaq	0x6c0(%rbp), %rcx
140021cfe:     	callq	0x14014ba20 <.text+0x14aa20>
140021d03:     	movb	$0x1, 0x812(%rbp)
140021d0a:     	movb	$0x0, 0x811(%rbp)
140021d11:     	movaps	0x120(%rsp), %xmm6
140021d19:     	movaps	0x110(%rsp), %xmm7
140021d21:     	movaps	0x100(%rsp), %xmm8
140021d2a:     	movaps	0xf0(%rsp), %xmm9
140021d33:     	movaps	0xe0(%rsp), %xmm10
140021d3c:     	movapd	0xd0(%rsp), %xmm11
140021d46:     	movaps	0xc0(%rsp), %xmm12
140021d4f:     	movaps	0xb0(%rsp), %xmm13
140021d58:     	movaps	0xa0(%rsp), %xmm14
140021d61:     	movaps	0x90(%rsp), %xmm15
140021d6a:     	addq	$0x138, %rsp            # imm = 0x138
140021d71:     	popq	%rbx
140021d72:     	popq	%rdi
140021d73:     	popq	%rsi
140021d74:     	popq	%r12
140021d76:     	popq	%r13
140021d78:     	popq	%r14
140021d7a:     	popq	%r15
140021d7c:     	popq	%rbp
140021d7d:     	retq
140021d7e:     	nop
140021d80:     	movq	%rdx, 0x10(%rsp)
140021d85:     	pushq	%rbp
140021d86:     	pushq	%r15
140021d88:     	pushq	%r14
140021d8a:     	pushq	%r13
140021d8c:     	pushq	%r12
140021d8e:     	pushq	%rsi
140021d8f:     	pushq	%rdi
140021d90:     	pushq	%rbx
140021d91:     	subq	$0x138, %rsp            # imm = 0x138
140021d98:     	leaq	0x80(%rdx), %rbp
140021d9f:     	movaps	%xmm15, 0x90(%rsp)
140021da8:     	movaps	%xmm14, 0xa0(%rsp)
140021db1:     	movaps	%xmm13, 0xb0(%rsp)
140021dba:     	movaps	%xmm12, 0xc0(%rsp)
140021dc3:     	movaps	%xmm11, 0xd0(%rsp)
140021dcc:     	movaps	%xmm10, 0xe0(%rsp)
140021dd5:     	movaps	%xmm9, 0xf0(%rsp)
140021dde:     	movaps	%xmm8, 0x100(%rsp)
140021de7:     	movaps	%xmm7, 0x110(%rsp)
140021def:     	movaps	%xmm6, 0x120(%rsp)
140021df7:     	movq	0x6b0(%rbp), %rax
140021dfe:     	testq	%rax, %rax
140021e01:     	je	0x140021e1c <.text+0x20e1c>
140021e03:     	movq	%rax, %rdx
140021e06:     	shlq	$0x2, %rdx
140021e0a:     	movl	$0x4, %r8d
140021e10:     	movq	0x7f0(%rbp), %rcx
140021e17:     	callq	0x140613c20 <.text+0x612c20>
140021e1c:     	movb	$0x1, 0x812(%rbp)
140021e23:     	movb	$0x1, 0x811(%rbp)
140021e2a:     	movaps	0x120(%rsp), %xmm6
140021e32:     	movaps	0x110(%rsp), %xmm7
140021e3a:     	movaps	0x100(%rsp), %xmm8
140021e43:     	movaps	0xf0(%rsp), %xmm9
140021e4c:     	movaps	0xe0(%rsp), %xmm10
140021e55:     	movapd	0xd0(%rsp), %xmm11
140021e5f:     	movaps	0xc0(%rsp), %xmm12
140021e68:     	movaps	0xb0(%rsp), %xmm13
140021e71:     	movaps	0xa0(%rsp), %xmm14
140021e7a:     	movaps	0x90(%rsp), %xmm15
140021e83:     	addq	$0x138, %rsp            # imm = 0x138
140021e8a:     	popq	%rbx
140021e8b:     	popq	%rdi
140021e8c:     	popq	%rsi
140021e8d:     	popq	%r12
140021e8f:     	popq	%r13
140021e91:     	popq	%r14
140021e93:     	popq	%r15
140021e95:     	popq	%rbp
140021e96:     	retq
140021e97:     	nopw	(%rax,%rax)
140021ea0:     	movq	%rdx, 0x10(%rsp)
140021ea5:     	pushq	%rbp
140021ea6:     	pushq	%r15
140021ea8:     	pushq	%r14
140021eaa:     	pushq	%r13
140021eac:     	pushq	%r12
140021eae:     	pushq	%rsi
140021eaf:     	pushq	%rdi
140021eb0:     	pushq	%rbx
140021eb1:     	subq	$0x138, %rsp            # imm = 0x138
140021eb8:     	leaq	0x80(%rdx), %rbp
140021ebf:     	movdqa	%xmm15, 0x90(%rsp)
140021ec9:     	movaps	%xmm14, 0xa0(%rsp)
140021ed2:     	movdqa	%xmm13, 0xb0(%rsp)
140021edc:     	movdqa	%xmm12, 0xc0(%rsp)
140021ee6:     	movaps	%xmm11, 0xd0(%rsp)
140021eef:     	movaps	%xmm10, 0xe0(%rsp)
140021ef8:     	movaps	%xmm9, 0xf0(%rsp)
140021f01:     	movaps	%xmm8, 0x100(%rsp)
140021f0a:     	movaps	%xmm7, 0x110(%rsp)
140021f12:     	movaps	%xmm6, 0x120(%rsp)
140021f1a:     	movq	0x808(%rbp), %rax
140021f21:     	incq	(%rax)
140021f24:     	movaps	0x120(%rsp), %xmm6
140021f2c:     	movaps	0x110(%rsp), %xmm7
140021f34:     	movaps	0x100(%rsp), %xmm8
140021f3d:     	movaps	0xf0(%rsp), %xmm9
140021f46:     	movaps	0xe0(%rsp), %xmm10
140021f4f:     	movaps	0xd0(%rsp), %xmm11
140021f58:     	movaps	0xc0(%rsp), %xmm12
140021f61:     	movaps	0xb0(%rsp), %xmm13
140021f6a:     	movaps	0xa0(%rsp), %xmm14
140021f73:     	movaps	0x90(%rsp), %xmm15
140021f7c:     	addq	$0x138, %rsp            # imm = 0x138
140021f83:     	popq	%rbx
140021f84:     	popq	%rdi
140021f85:     	popq	%rsi
140021f86:     	popq	%r12
140021f88:     	popq	%r13
140021f8a:     	popq	%r14
140021f8c:     	popq	%r15
140021f8e:     	popq	%rbp
140021f8f:     	retq
140021f90:     	movq	%rdx, 0x10(%rsp)
140021f95:     	pushq	%rbp
140021f96:     	pushq	%r15
140021f98:     	pushq	%r14
140021f9a:     	pushq	%r13
140021f9c:     	pushq	%r12
140021f9e:     	pushq	%rsi
140021f9f:     	pushq	%rdi
140021fa0:     	pushq	%rbx
140021fa1:     	subq	$0x138, %rsp            # imm = 0x138
140021fa8:     	leaq	0x80(%rdx), %rbp
140021faf:     	movaps	%xmm15, 0x90(%rsp)
140021fb8:     	movaps	%xmm14, 0xa0(%rsp)
140021fc1:     	movaps	%xmm13, 0xb0(%rsp)
140021fca:     	movaps	%xmm12, 0xc0(%rsp)
140021fd3:     	movdqa	%xmm11, 0xd0(%rsp)
140021fdd:     	movaps	%xmm10, 0xe0(%rsp)
140021fe6:     	movaps	%xmm9, 0xf0(%rsp)
140021fef:     	movaps	%xmm8, 0x100(%rsp)
140021ff8:     	movaps	%xmm7, 0x110(%rsp)
140022000:     	movaps	%xmm6, 0x120(%rsp)
140022008:     	movb	$0x1, 0x817(%rbp)
14002200f:     	leaq	0x6c0(%rbp), %rcx
140022016:     	callq	0x14014ba20 <.text+0x14aa20>
14002201b:     	movb	$0x1, 0x817(%rbp)
140022022:     	movaps	0x120(%rsp), %xmm6
14002202a:     	movaps	0x110(%rsp), %xmm7
140022032:     	movaps	0x100(%rsp), %xmm8
14002203b:     	movaps	0xf0(%rsp), %xmm9
140022044:     	movaps	0xe0(%rsp), %xmm10
14002204d:     	movaps	0xd0(%rsp), %xmm11
140022056:     	movdqa	0xc0(%rsp), %xmm12
140022060:     	movdqa	0xb0(%rsp), %xmm13
14002206a:     	movaps	0xa0(%rsp), %xmm14
140022073:     	movdqa	0x90(%rsp), %xmm15
14002207d:     	addq	$0x138, %rsp            # imm = 0x138
140022084:     	popq	%rbx
140022085:     	popq	%rdi
140022086:     	popq	%rsi
140022087:     	popq	%r12
140022089:     	popq	%r13
14002208b:     	popq	%r14
14002208d:     	popq	%r15
14002208f:     	popq	%rbp
140022090:     	retq
140022091:     	nopw	%cs:(%rax,%rax)
1400220a0:     	movq	%rdx, 0x10(%rsp)
1400220a5:     	pushq	%rbp
1400220a6:     	pushq	%r15
1400220a8:     	pushq	%r14
1400220aa:     	pushq	%r13
1400220ac:     	pushq	%r12
1400220ae:     	pushq	%rsi
1400220af:     	pushq	%rdi
1400220b0:     	pushq	%rbx
1400220b1:     	subq	$0x138, %rsp            # imm = 0x138
1400220b8:     	leaq	0x80(%rdx), %rbp
1400220bf:     	movaps	%xmm15, 0x90(%rsp)
1400220c8:     	movaps	%xmm14, 0xa0(%rsp)
1400220d1:     	movaps	%xmm13, 0xb0(%rsp)
1400220da:     	movaps	%xmm12, 0xc0(%rsp)
1400220e3:     	movaps	%xmm11, 0xd0(%rsp)
1400220ec:     	movaps	%xmm10, 0xe0(%rsp)
1400220f5:     	movaps	%xmm9, 0xf0(%rsp)
1400220fe:     	movaps	%xmm8, 0x100(%rsp)
140022107:     	movaps	%xmm7, 0x110(%rsp)
14002210f:     	movaps	%xmm6, 0x120(%rsp)
140022117:     	movb	$0x1, 0x817(%rbp)
14002211e:     	leaq	0x6c0(%rbp), %rcx
140022125:     	callq	0x14014ba20 <.text+0x14aa20>
14002212a:     	movb	$0x1, 0x817(%rbp)
140022131:     	movaps	0x120(%rsp), %xmm6
140022139:     	movaps	0x110(%rsp), %xmm7
140022141:     	movaps	0x100(%rsp), %xmm8
14002214a:     	movaps	0xf0(%rsp), %xmm9
140022153:     	movaps	0xe0(%rsp), %xmm10
14002215c:     	movaps	0xd0(%rsp), %xmm11
140022165:     	movdqa	0xc0(%rsp), %xmm12
14002216f:     	movdqa	0xb0(%rsp), %xmm13
140022179:     	movaps	0xa0(%rsp), %xmm14
140022182:     	movdqa	0x90(%rsp), %xmm15
14002218c:     	addq	$0x138, %rsp            # imm = 0x138
140022193:     	popq	%rbx
140022194:     	popq	%rdi
140022195:     	popq	%rsi
140022196:     	popq	%r12
140022198:     	popq	%r13
14002219a:     	popq	%r14
14002219c:     	popq	%r15
14002219e:     	popq	%rbp
14002219f:     	retq
1400221a0:     	movq	%rdx, 0x10(%rsp)
1400221a5:     	pushq	%rbp
1400221a6:     	pushq	%r15
1400221a8:     	pushq	%r14
1400221aa:     	pushq	%r13
1400221ac:     	pushq	%r12
1400221ae:     	pushq	%rsi
1400221af:     	pushq	%rdi
1400221b0:     	pushq	%rbx
1400221b1:     	subq	$0x138, %rsp            # imm = 0x138
1400221b8:     	leaq	0x80(%rdx), %rbp
1400221bf:     	movaps	%xmm15, 0x90(%rsp)
1400221c8:     	movaps	%xmm14, 0xa0(%rsp)
1400221d1:     	movaps	%xmm13, 0xb0(%rsp)
1400221da:     	movaps	%xmm12, 0xc0(%rsp)
1400221e3:     	movaps	%xmm11, 0xd0(%rsp)
1400221ec:     	movaps	%xmm10, 0xe0(%rsp)
1400221f5:     	movaps	%xmm9, 0xf0(%rsp)
1400221fe:     	movaps	%xmm8, 0x100(%rsp)
140022207:     	movaps	%xmm7, 0x110(%rsp)
14002220f:     	movaps	%xmm6, 0x120(%rsp)
140022217:     	movzbl	0x7be(%rbp), %ebx
14002221e:     	testb	$0x1, %bl
140022221:     	je	0x1400222c0 <.text+0x212c0>
140022227:     	cmpq	$0x0, 0x6b0(%rbp)
14002222f:     	je	0x14002224e <.text+0x2124e>
140022231:     	movq	0x6b0(%rbp), %rdx
140022238:     	shlq	$0x2, %rdx
14002223c:     	movl	$0x4, %r8d
140022242:     	movq	0x7f0(%rbp), %rcx
140022249:     	callq	0x140613c20 <.text+0x612c20>
14002224e:     	movb	%bl, 0x7bf(%rbp)
140022254:     	movaps	0x120(%rsp), %xmm6
14002225c:     	movaps	0x110(%rsp), %xmm7
140022264:     	movaps	0x100(%rsp), %xmm8
14002226d:     	movaps	0xf0(%rsp), %xmm9
140022276:     	movaps	0xe0(%rsp), %xmm10
14002227f:     	movaps	0xd0(%rsp), %xmm11
140022288:     	movaps	0xc0(%rsp), %xmm12
140022291:     	movaps	0xb0(%rsp), %xmm13
14002229a:     	movaps	0xa0(%rsp), %xmm14
1400222a3:     	movaps	0x90(%rsp), %xmm15
1400222ac:     	addq	$0x138, %rsp            # imm = 0x138
1400222b3:     	popq	%rbx
1400222b4:     	popq	%rdi
1400222b5:     	popq	%rsi
1400222b6:     	popq	%r12
1400222b8:     	popq	%r13
1400222ba:     	popq	%r14
1400222bc:     	popq	%r15
1400222be:     	popq	%rbp
1400222bf:     	retq
1400222c0:     	movb	%bl, 0x7bf(%rbp)
1400222c6:     	movaps	0x120(%rsp), %xmm6
1400222ce:     	movaps	0x110(%rsp), %xmm7
1400222d6:     	movaps	0x100(%rsp), %xmm8
1400222df:     	movaps	0xf0(%rsp), %xmm9
1400222e8:     	movaps	0xe0(%rsp), %xmm10
1400222f1:     	movaps	0xd0(%rsp), %xmm11
1400222fa:     	movaps	0xc0(%rsp), %xmm12
140022303:     	movaps	0xb0(%rsp), %xmm13
14002230c:     	movaps	0xa0(%rsp), %xmm14
140022315:     	movaps	0x90(%rsp), %xmm15
14002231e:     	addq	$0x138, %rsp            # imm = 0x138
140022325:     	popq	%rbx
140022326:     	popq	%rdi
140022327:     	popq	%rsi
140022328:     	popq	%r12
14002232a:     	popq	%r13
14002232c:     	popq	%r14
14002232e:     	popq	%r15
140022330:     	popq	%rbp
140022331:     	retq
140022332:     	nopw	%cs:(%rax,%rax)
140022340:     	movq	%rdx, 0x10(%rsp)
140022345:     	pushq	%rbp
140022346:     	pushq	%r15
140022348:     	pushq	%r14
14002234a:     	pushq	%r13
14002234c:     	pushq	%r12
14002234e:     	pushq	%rsi
14002234f:     	pushq	%rdi
140022350:     	pushq	%rbx
140022351:     	subq	$0x138, %rsp            # imm = 0x138
140022358:     	leaq	0x80(%rdx), %rbp
14002235f:     	movdqa	%xmm15, 0x90(%rsp)
140022369:     	movaps	%xmm14, 0xa0(%rsp)
140022372:     	movdqa	%xmm13, 0xb0(%rsp)
14002237c:     	movdqa	%xmm12, 0xc0(%rsp)
140022386:     	movaps	%xmm11, 0xd0(%rsp)
14002238f:     	movaps	%xmm10, 0xe0(%rsp)
140022398:     	movaps	%xmm9, 0xf0(%rsp)
1400223a1:     	movaps	%xmm8, 0x100(%rsp)
1400223aa:     	movaps	%xmm7, 0x110(%rsp)
1400223b2:     	movaps	%xmm6, 0x120(%rsp)
1400223ba:     	movq	0x7a0(%rbp), %rax
1400223c1:     	incq	(%rax)
1400223c4:     	movaps	0x120(%rsp), %xmm6
1400223cc:     	movaps	0x110(%rsp), %xmm7
1400223d4:     	movaps	0x100(%rsp), %xmm8
1400223dd:     	movaps	0xf0(%rsp), %xmm9
1400223e6:     	movaps	0xe0(%rsp), %xmm10
1400223ef:     	movaps	0xd0(%rsp), %xmm11
1400223f8:     	movaps	0xc0(%rsp), %xmm12
140022401:     	movaps	0xb0(%rsp), %xmm13
14002240a:     	movaps	0xa0(%rsp), %xmm14
140022413:     	movaps	0x90(%rsp), %xmm15
14002241c:     	addq	$0x138, %rsp            # imm = 0x138
140022423:     	popq	%rbx
140022424:     	popq	%rdi
140022425:     	popq	%rsi
140022426:     	popq	%r12
140022428:     	popq	%r13
14002242a:     	popq	%r14
14002242c:     	popq	%r15
14002242e:     	popq	%rbp
14002242f:     	retq
140022430:     	movq	%rdx, 0x10(%rsp)
140022435:     	pushq	%rbp
140022436:     	pushq	%r15
140022438:     	pushq	%r14
14002243a:     	pushq	%r13
14002243c:     	pushq	%r12
14002243e:     	pushq	%rsi
14002243f:     	pushq	%rdi
140022440:     	pushq	%rbx
140022441:     	subq	$0x138, %rsp            # imm = 0x138
140022448:     	leaq	0x80(%rdx), %rbp
14002244f:     	movaps	%xmm15, 0x90(%rsp)
140022458:     	movaps	%xmm14, 0xa0(%rsp)
140022461:     	movaps	%xmm13, 0xb0(%rsp)
14002246a:     	movaps	%xmm12, 0xc0(%rsp)
140022473:     	movaps	%xmm11, 0xd0(%rsp)
14002247c:     	movaps	%xmm10, 0xe0(%rsp)
140022485:     	movaps	%xmm9, 0xf0(%rsp)
14002248e:     	movaps	%xmm8, 0x100(%rsp)
140022497:     	movaps	%xmm7, 0x110(%rsp)
14002249f:     	movaps	%xmm6, 0x120(%rsp)
1400224a7:     	movzbl	0x7bf(%rbp), %ebx
1400224ae:     	testb	$0x1, %bl
1400224b1:     	je	0x140022550 <.text+0x21550>
1400224b7:     	cmpq	$0x0, 0x808(%rbp)
1400224bf:     	je	0x1400224de <.text+0x214de>
1400224c1:     	movq	0x808(%rbp), %rdx
1400224c8:     	shlq	$0x3, %rdx
1400224cc:     	movl	$0x4, %r8d
1400224d2:     	movq	0x7a0(%rbp), %rcx
1400224d9:     	callq	0x140613c20 <.text+0x612c20>
1400224de:     	movb	%bl, 0x7c0(%rbp)
1400224e4:     	movaps	0x120(%rsp), %xmm6
1400224ec:     	movaps	0x110(%rsp), %xmm7
1400224f4:     	movaps	0x100(%rsp), %xmm8
1400224fd:     	movaps	0xf0(%rsp), %xmm9
140022506:     	movaps	0xe0(%rsp), %xmm10
14002250f:     	movaps	0xd0(%rsp), %xmm11
140022518:     	movaps	0xc0(%rsp), %xmm12
140022521:     	movaps	0xb0(%rsp), %xmm13
14002252a:     	movaps	0xa0(%rsp), %xmm14
140022533:     	movaps	0x90(%rsp), %xmm15
14002253c:     	addq	$0x138, %rsp            # imm = 0x138
140022543:     	popq	%rbx
140022544:     	popq	%rdi
140022545:     	popq	%rsi
140022546:     	popq	%r12
140022548:     	popq	%r13
14002254a:     	popq	%r14
14002254c:     	popq	%r15
14002254e:     	popq	%rbp
14002254f:     	retq
140022550:     	movb	%bl, 0x7c0(%rbp)
140022556:     	movaps	0x120(%rsp), %xmm6
14002255e:     	movaps	0x110(%rsp), %xmm7
140022566:     	movaps	0x100(%rsp), %xmm8
14002256f:     	movaps	0xf0(%rsp), %xmm9
140022578:     	movaps	0xe0(%rsp), %xmm10
140022581:     	movaps	0xd0(%rsp), %xmm11
14002258a:     	movaps	0xc0(%rsp), %xmm12
140022593:     	movaps	0xb0(%rsp), %xmm13
14002259c:     	movaps	0xa0(%rsp), %xmm14
1400225a5:     	movaps	0x90(%rsp), %xmm15
1400225ae:     	addq	$0x138, %rsp            # imm = 0x138
1400225b5:     	popq	%rbx
1400225b6:     	popq	%rdi
1400225b7:     	popq	%rsi
1400225b8:     	popq	%r12
1400225ba:     	popq	%r13
1400225bc:     	popq	%r14
1400225be:     	popq	%r15
1400225c0:     	popq	%rbp
1400225c1:     	retq
1400225c2:     	nopw	%cs:(%rax,%rax)
1400225d0:     	movq	%rdx, 0x10(%rsp)
1400225d5:     	pushq	%rbp
1400225d6:     	pushq	%r15
1400225d8:     	pushq	%r14
1400225da:     	pushq	%r13
1400225dc:     	pushq	%r12
1400225de:     	pushq	%rsi
1400225df:     	pushq	%rdi
1400225e0:     	pushq	%rbx
1400225e1:     	subq	$0x138, %rsp            # imm = 0x138
1400225e8:     	leaq	0x80(%rdx), %rbp
1400225ef:     	movdqa	%xmm15, 0x90(%rsp)
1400225f9:     	movaps	%xmm14, 0xa0(%rsp)
140022602:     	movdqa	%xmm13, 0xb0(%rsp)
14002260c:     	movdqa	%xmm12, 0xc0(%rsp)
140022616:     	movaps	%xmm11, 0xd0(%rsp)
14002261f:     	movaps	%xmm10, 0xe0(%rsp)
140022628:     	movaps	%xmm9, 0xf0(%rsp)
140022631:     	movaps	%xmm8, 0x100(%rsp)
14002263a:     	movaps	%xmm7, 0x110(%rsp)
140022642:     	movaps	%xmm6, 0x120(%rsp)
14002264a:     	movq	0x590(%rbp), %rdx
140022651:     	testq	%rdx, %rdx
140022654:     	je	0x14002266c <.text+0x2166c>
140022656:     	shlq	$0x4, %rdx
14002265a:     	movl	$0x10, %r8d
140022660:     	movq	0x808(%rbp), %rcx
140022667:     	callq	0x140613c20 <.text+0x612c20>
14002266c:     	movaps	0x120(%rsp), %xmm6
140022674:     	movaps	0x110(%rsp), %xmm7
14002267c:     	movaps	0x100(%rsp), %xmm8
140022685:     	movaps	0xf0(%rsp), %xmm9
14002268e:     	movaps	0xe0(%rsp), %xmm10
140022697:     	movaps	0xd0(%rsp), %xmm11
1400226a0:     	movdqa	0xc0(%rsp), %xmm12
1400226aa:     	movdqa	0xb0(%rsp), %xmm13
1400226b4:     	movaps	0xa0(%rsp), %xmm14
1400226bd:     	movdqa	0x90(%rsp), %xmm15
1400226c7:     	addq	$0x138, %rsp            # imm = 0x138
1400226ce:     	popq	%rbx
1400226cf:     	popq	%rdi
1400226d0:     	popq	%rsi
1400226d1:     	popq	%r12
1400226d3:     	popq	%r13
1400226d5:     	popq	%r14
1400226d7:     	popq	%r15
1400226d9:     	popq	%rbp
1400226da:     	retq
1400226db:     	nopl	(%rax,%rax)
1400226e0:     	movq	%rdx, 0x10(%rsp)
1400226e5:     	pushq	%rbp
1400226e6:     	pushq	%r15
1400226e8:     	pushq	%r14
1400226ea:     	pushq	%r13
1400226ec:     	pushq	%r12
1400226ee:     	pushq	%rsi
1400226ef:     	pushq	%rdi
1400226f0:     	pushq	%rbx
1400226f1:     	subq	$0x138, %rsp            # imm = 0x138
1400226f8:     	leaq	0x80(%rdx), %rbp
1400226ff:     	movaps	%xmm15, 0x90(%rsp)
140022708:     	movaps	%xmm14, 0xa0(%rsp)
140022711:     	movaps	%xmm13, 0xb0(%rsp)
14002271a:     	movaps	%xmm12, 0xc0(%rsp)
140022723:     	movaps	%xmm11, 0xd0(%rsp)
14002272c:     	movaps	%xmm10, 0xe0(%rsp)
140022735:     	movaps	%xmm9, 0xf0(%rsp)
14002273e:     	movaps	%xmm8, 0x100(%rsp)
140022747:     	movaps	%xmm7, 0x110(%rsp)
14002274f:     	movaps	%xmm6, 0x120(%rsp)
140022757:     	testb	$0x1, 0x7c0(%rbp)
14002275e:     	je	0x1400227e6 <.text+0x217e6>
140022764:     	leaq	0x590(%rbp), %rcx
14002276b:     	callq	0x14014ba20 <.text+0x14aa20>
140022770:     	movb	$0x1, 0x817(%rbp)
140022777:     	movaps	0x120(%rsp), %xmm6
14002277f:     	movaps	0x110(%rsp), %xmm7
140022787:     	movaps	0x100(%rsp), %xmm8
140022790:     	movaps	0xf0(%rsp), %xmm9
140022799:     	movaps	0xe0(%rsp), %xmm10
1400227a2:     	movaps	0xd0(%rsp), %xmm11
1400227ab:     	movdqa	0xc0(%rsp), %xmm12
1400227b5:     	movdqa	0xb0(%rsp), %xmm13
1400227bf:     	movaps	0xa0(%rsp), %xmm14
1400227c8:     	movdqa	0x90(%rsp), %xmm15
1400227d2:     	addq	$0x138, %rsp            # imm = 0x138
1400227d9:     	popq	%rbx
1400227da:     	popq	%rdi
1400227db:     	popq	%rsi
1400227dc:     	popq	%r12
1400227de:     	popq	%r13
1400227e0:     	popq	%r14
1400227e2:     	popq	%r15
1400227e4:     	popq	%rbp
1400227e5:     	retq
1400227e6:     	movb	$0x1, 0x817(%rbp)
1400227ed:     	movaps	0x120(%rsp), %xmm6
1400227f5:     	movaps	0x110(%rsp), %xmm7
1400227fd:     	movaps	0x100(%rsp), %xmm8
140022806:     	movaps	0xf0(%rsp), %xmm9
14002280f:     	movaps	0xe0(%rsp), %xmm10
140022818:     	movaps	0xd0(%rsp), %xmm11
140022821:     	movdqa	0xc0(%rsp), %xmm12
14002282b:     	movdqa	0xb0(%rsp), %xmm13
140022835:     	movaps	0xa0(%rsp), %xmm14
14002283e:     	movdqa	0x90(%rsp), %xmm15
140022848:     	addq	$0x138, %rsp            # imm = 0x138
14002284f:     	popq	%rbx
140022850:     	popq	%rdi
140022851:     	popq	%rsi
140022852:     	popq	%r12
140022854:     	popq	%r13
140022856:     	popq	%r14
140022858:     	popq	%r15
14002285a:     	popq	%rbp
14002285b:     	retq
14002285c:     	nopl	(%rax)
140022860:     	movq	%rdx, 0x10(%rsp)
140022865:     	pushq	%rbp
140022866:     	pushq	%r15
140022868:     	pushq	%r14
14002286a:     	pushq	%r13
14002286c:     	pushq	%r12
14002286e:     	pushq	%rsi
14002286f:     	pushq	%rdi
140022870:     	pushq	%rbx
140022871:     	subq	$0x138, %rsp            # imm = 0x138
140022878:     	leaq	0x80(%rdx), %rbp
14002287f:     	movaps	%xmm15, 0x90(%rsp)
140022888:     	movaps	%xmm14, 0xa0(%rsp)
140022891:     	movaps	%xmm13, 0xb0(%rsp)
14002289a:     	movaps	%xmm12, 0xc0(%rsp)
1400228a3:     	movaps	%xmm11, 0xd0(%rsp)
1400228ac:     	movaps	%xmm10, 0xe0(%rsp)
1400228b5:     	movaps	%xmm9, 0xf0(%rsp)
1400228be:     	movaps	%xmm8, 0x100(%rsp)
1400228c7:     	movaps	%xmm7, 0x110(%rsp)
1400228cf:     	movaps	%xmm6, 0x120(%rsp)
1400228d7:     	movzbl	0x811(%rbp), %ebx
1400228de:     	movzbl	0x812(%rbp), %esi
1400228e5:     	cmpq	$0x0, 0x7a0(%rbp)
1400228ed:     	je	0x14002290c <.text+0x2190c>
1400228ef:     	movq	0x7a0(%rbp), %rdx
1400228f6:     	shlq	$0x2, %rdx
1400228fa:     	movl	$0x4, %r8d
140022900:     	movq	0x808(%rbp), %rcx
140022907:     	callq	0x140613c20 <.text+0x612c20>
14002290c:     	movb	%sil, 0x79c(%rbp)
140022913:     	andb	$0x1, %bl
140022916:     	movb	%bl, 0x79b(%rbp)
14002291c:     	movaps	0x120(%rsp), %xmm6
140022924:     	movaps	0x110(%rsp), %xmm7
14002292c:     	movaps	0x100(%rsp), %xmm8
140022935:     	movaps	0xf0(%rsp), %xmm9
14002293e:     	movaps	0xe0(%rsp), %xmm10
140022947:     	movapd	0xd0(%rsp), %xmm11
140022951:     	movaps	0xc0(%rsp), %xmm12
14002295a:     	movaps	0xb0(%rsp), %xmm13
140022963:     	movaps	0xa0(%rsp), %xmm14
14002296c:     	movaps	0x90(%rsp), %xmm15
140022975:     	addq	$0x138, %rsp            # imm = 0x138
14002297c:     	popq	%rbx
14002297d:     	popq	%rdi
14002297e:     	popq	%rsi
14002297f:     	popq	%r12
140022981:     	popq	%r13
140022983:     	popq	%r14
140022985:     	popq	%r15
140022987:     	popq	%rbp
140022988:     	retq
140022989:     	nopl	(%rax)
140022990:     	movq	%rdx, 0x10(%rsp)
140022995:     	pushq	%rbp
140022996:     	pushq	%r15
140022998:     	pushq	%r14
14002299a:     	pushq	%r13
14002299c:     	pushq	%r12
14002299e:     	pushq	%rsi
14002299f:     	pushq	%rdi
1400229a0:     	pushq	%rbx
1400229a1:     	subq	$0x138, %rsp            # imm = 0x138
1400229a8:     	leaq	0x80(%rdx), %rbp
1400229af:     	movdqa	%xmm15, 0x90(%rsp)
1400229b9:     	movaps	%xmm14, 0xa0(%rsp)
1400229c2:     	movdqa	%xmm13, 0xb0(%rsp)
1400229cc:     	movdqa	%xmm12, 0xc0(%rsp)
1400229d6:     	movaps	%xmm11, 0xd0(%rsp)
1400229df:     	movaps	%xmm10, 0xe0(%rsp)
1400229e8:     	movaps	%xmm9, 0xf0(%rsp)
1400229f1:     	movaps	%xmm8, 0x100(%rsp)
1400229fa:     	movaps	%xmm7, 0x110(%rsp)
140022a02:     	movaps	%xmm6, 0x120(%rsp)
140022a0a:     	movb	$0x0, 0x817(%rbp)
140022a11:     	leaq	0x6d0(%rbp), %rcx
140022a18:     	callq	0x140601be0 <.text+0x600be0>
140022a1d:     	movb	$0x0, 0x817(%rbp)
140022a24:     	movaps	0x120(%rsp), %xmm6
140022a2c:     	movaps	0x110(%rsp), %xmm7
140022a34:     	movaps	0x100(%rsp), %xmm8
140022a3d:     	movaps	0xf0(%rsp), %xmm9
140022a46:     	movaps	0xe0(%rsp), %xmm10
140022a4f:     	movaps	0xd0(%rsp), %xmm11
140022a58:     	movdqa	0xc0(%rsp), %xmm12
140022a62:     	movdqa	0xb0(%rsp), %xmm13
140022a6c:     	movaps	0xa0(%rsp), %xmm14
140022a75:     	movdqa	0x90(%rsp), %xmm15
140022a7f:     	addq	$0x138, %rsp            # imm = 0x138
140022a86:     	popq	%rbx
140022a87:     	popq	%rdi
140022a88:     	popq	%rsi
140022a89:     	popq	%r12
140022a8b:     	popq	%r13
140022a8d:     	popq	%r14
140022a8f:     	popq	%r15
140022a91:     	popq	%rbp
140022a92:     	retq
140022a93:     	nopw	%cs:(%rax,%rax)
140022aa0:     	movq	%rdx, 0x10(%rsp)
140022aa5:     	pushq	%rbp
140022aa6:     	pushq	%r15
140022aa8:     	pushq	%r14
140022aaa:     	pushq	%r13
140022aac:     	pushq	%r12
140022aae:     	pushq	%rsi
140022aaf:     	pushq	%rdi
140022ab0:     	pushq	%rbx
140022ab1:     	subq	$0x138, %rsp            # imm = 0x138
140022ab8:     	leaq	0x80(%rdx), %rbp
140022abf:     	movdqa	%xmm15, 0x90(%rsp)
140022ac9:     	movaps	%xmm14, 0xa0(%rsp)
140022ad2:     	movdqa	%xmm13, 0xb0(%rsp)
140022adc:     	movdqa	%xmm12, 0xc0(%rsp)
140022ae6:     	movaps	%xmm11, 0xd0(%rsp)
140022aef:     	movaps	%xmm10, 0xe0(%rsp)
140022af8:     	movaps	%xmm9, 0xf0(%rsp)
140022b01:     	movaps	%xmm8, 0x100(%rsp)
140022b0a:     	movaps	%xmm7, 0x110(%rsp)
140022b12:     	movaps	%xmm6, 0x120(%rsp)
140022b1a:     	movb	$0x0, 0x814(%rbp)
140022b21:     	leaq	0x590(%rbp), %rcx
140022b28:     	callq	0x140609eb0 <.text+0x608eb0>
140022b2d:     	movb	$0x0, 0x814(%rbp)
140022b34:     	movaps	0x120(%rsp), %xmm6
140022b3c:     	movaps	0x110(%rsp), %xmm7
140022b44:     	movaps	0x100(%rsp), %xmm8
140022b4d:     	movaps	0xf0(%rsp), %xmm9
140022b56:     	movaps	0xe0(%rsp), %xmm10
140022b5f:     	movaps	0xd0(%rsp), %xmm11
140022b68:     	movdqa	0xc0(%rsp), %xmm12
140022b72:     	movdqa	0xb0(%rsp), %xmm13
140022b7c:     	movaps	0xa0(%rsp), %xmm14
140022b85:     	movdqa	0x90(%rsp), %xmm15
140022b8f:     	addq	$0x138, %rsp            # imm = 0x138
140022b96:     	popq	%rbx
140022b97:     	popq	%rdi
140022b98:     	popq	%rsi
140022b99:     	popq	%r12
140022b9b:     	popq	%r13
140022b9d:     	popq	%r14
140022b9f:     	popq	%r15
140022ba1:     	popq	%rbp
140022ba2:     	retq
140022ba3:     	nopw	%cs:(%rax,%rax)
140022bb0:     	movq	%rdx, 0x10(%rsp)
140022bb5:     	pushq	%rbp
140022bb6:     	pushq	%r15
140022bb8:     	pushq	%r14
140022bba:     	pushq	%r13
140022bbc:     	pushq	%r12
140022bbe:     	pushq	%rsi
140022bbf:     	pushq	%rdi
140022bc0:     	pushq	%rbx
140022bc1:     	subq	$0x138, %rsp            # imm = 0x138
140022bc8:     	leaq	0x80(%rdx), %rbp
140022bcf:     	movdqa	%xmm15, 0x90(%rsp)
140022bd9:     	movaps	%xmm14, 0xa0(%rsp)
140022be2:     	movdqa	%xmm13, 0xb0(%rsp)
140022bec:     	movdqa	%xmm12, 0xc0(%rsp)
140022bf6:     	movaps	%xmm11, 0xd0(%rsp)
140022bff:     	movaps	%xmm10, 0xe0(%rsp)
140022c08:     	movaps	%xmm9, 0xf0(%rsp)
140022c11:     	movaps	%xmm8, 0x100(%rsp)
140022c1a:     	movaps	%xmm7, 0x110(%rsp)
140022c22:     	movaps	%xmm6, 0x120(%rsp)
140022c2a:     	movb	$0x0, 0x7ce(%rbp)
140022c31:     	leaq	0x140(%rbp), %rcx
140022c38:     	callq	0x140609eb0 <.text+0x608eb0>
140022c3d:     	movb	$0x0, 0x7ce(%rbp)
140022c44:     	movaps	0x120(%rsp), %xmm6
140022c4c:     	movaps	0x110(%rsp), %xmm7
140022c54:     	movaps	0x100(%rsp), %xmm8
140022c5d:     	movaps	0xf0(%rsp), %xmm9
140022c66:     	movaps	0xe0(%rsp), %xmm10
140022c6f:     	movaps	0xd0(%rsp), %xmm11
140022c78:     	movaps	0xc0(%rsp), %xmm12
140022c81:     	movaps	0xb0(%rsp), %xmm13
140022c8a:     	movaps	0xa0(%rsp), %xmm14
140022c93:     	movaps	0x90(%rsp), %xmm15
140022c9c:     	addq	$0x138, %rsp            # imm = 0x138
140022ca3:     	popq	%rbx
140022ca4:     	popq	%rdi
140022ca5:     	popq	%rsi
140022ca6:     	popq	%r12
140022ca8:     	popq	%r13
140022caa:     	popq	%r14
140022cac:     	popq	%r15
140022cae:     	popq	%rbp
140022caf:     	retq
140022cb0:     	movq	%rdx, 0x10(%rsp)
140022cb5:     	pushq	%rbp
140022cb6:     	pushq	%r15
140022cb8:     	pushq	%r14
140022cba:     	pushq	%r13
140022cbc:     	pushq	%r12
140022cbe:     	pushq	%rsi
140022cbf:     	pushq	%rdi
140022cc0:     	pushq	%rbx
140022cc1:     	subq	$0x138, %rsp            # imm = 0x138
140022cc8:     	leaq	0x80(%rdx), %rbp
140022ccf:     	movdqa	%xmm15, 0x90(%rsp)
140022cd9:     	movaps	%xmm14, 0xa0(%rsp)
140022ce2:     	movdqa	%xmm13, 0xb0(%rsp)
140022cec:     	movdqa	%xmm12, 0xc0(%rsp)
140022cf6:     	movaps	%xmm11, 0xd0(%rsp)
140022cff:     	movaps	%xmm10, 0xe0(%rsp)
140022d08:     	movaps	%xmm9, 0xf0(%rsp)
140022d11:     	movaps	%xmm8, 0x100(%rsp)
140022d1a:     	movaps	%xmm7, 0x110(%rsp)
140022d22:     	movaps	%xmm6, 0x120(%rsp)
140022d2a:     	movb	$0x0, 0x7cf(%rbp)
140022d31:     	leaq	0x140(%rbp), %rcx
140022d38:     	callq	0x140609eb0 <.text+0x608eb0>
140022d3d:     	movb	$0x0, 0x7cf(%rbp)
140022d44:     	movaps	0x120(%rsp), %xmm6
140022d4c:     	movaps	0x110(%rsp), %xmm7
140022d54:     	movaps	0x100(%rsp), %xmm8
140022d5d:     	movaps	0xf0(%rsp), %xmm9
140022d66:     	movaps	0xe0(%rsp), %xmm10
140022d6f:     	movaps	0xd0(%rsp), %xmm11
140022d78:     	movaps	0xc0(%rsp), %xmm12
140022d81:     	movaps	0xb0(%rsp), %xmm13
140022d8a:     	movaps	0xa0(%rsp), %xmm14
140022d93:     	movaps	0x90(%rsp), %xmm15
140022d9c:     	addq	$0x138, %rsp            # imm = 0x138
140022da3:     	popq	%rbx
140022da4:     	popq	%rdi
140022da5:     	popq	%rsi
140022da6:     	popq	%r12
140022da8:     	popq	%r13
140022daa:     	popq	%r14
140022dac:     	popq	%r15
140022dae:     	popq	%rbp
140022daf:     	retq
140022db0:     	movq	%rdx, 0x10(%rsp)
140022db5:     	pushq	%rbp
140022db6:     	pushq	%r15
140022db8:     	pushq	%r14
140022dba:     	pushq	%r13
140022dbc:     	pushq	%r12
140022dbe:     	pushq	%rsi
140022dbf:     	pushq	%rdi
140022dc0:     	pushq	%rbx
140022dc1:     	subq	$0x138, %rsp            # imm = 0x138
140022dc8:     	leaq	0x80(%rdx), %rbp
140022dcf:     	movaps	%xmm15, 0x90(%rsp)
140022dd8:     	movaps	%xmm14, 0xa0(%rsp)
140022de1:     	movaps	%xmm13, 0xb0(%rsp)
140022dea:     	movaps	%xmm12, 0xc0(%rsp)
140022df3:     	movapd	%xmm11, 0xd0(%rsp)
140022dfd:     	movaps	%xmm10, 0xe0(%rsp)
140022e06:     	movaps	%xmm9, 0xf0(%rsp)
140022e0f:     	movaps	%xmm8, 0x100(%rsp)
140022e18:     	movaps	%xmm7, 0x110(%rsp)
140022e20:     	movaps	%xmm6, 0x120(%rsp)
140022e28:     	movzbl	0x79b(%rbp), %ebx
140022e2f:     	movzbl	0x79c(%rbp), %esi
140022e36:     	testb	$0x1, %sil
140022e3a:     	je	0x140022edf <.text+0x21edf>
140022e40:     	movq	0x3e0(%rbp), %rdx
140022e47:     	testq	%rdx, %rdx
140022e4a:     	je	0x140022e62 <.text+0x21e62>
140022e4c:     	movq	0x3e8(%rbp), %rcx
140022e53:     	shlq	$0x2, %rdx
140022e57:     	movl	$0x4, %r8d
140022e5d:     	callq	0x140613c20 <.text+0x612c20>
140022e62:     	movb	%sil, 0x7bc(%rbp)
140022e69:     	andb	$0x1, %bl
140022e6c:     	movb	%bl, 0x7bb(%rbp)
140022e72:     	movaps	0x120(%rsp), %xmm6
140022e7a:     	movaps	0x110(%rsp), %xmm7
140022e82:     	movaps	0x100(%rsp), %xmm8
140022e8b:     	movaps	0xf0(%rsp), %xmm9
140022e94:     	movaps	0xe0(%rsp), %xmm10
140022e9d:     	movapd	0xd0(%rsp), %xmm11
140022ea7:     	movaps	0xc0(%rsp), %xmm12
140022eb0:     	movaps	0xb0(%rsp), %xmm13
140022eb9:     	movaps	0xa0(%rsp), %xmm14
140022ec2:     	movaps	0x90(%rsp), %xmm15
140022ecb:     	addq	$0x138, %rsp            # imm = 0x138
140022ed2:     	popq	%rbx
140022ed3:     	popq	%rdi
140022ed4:     	popq	%rsi
140022ed5:     	popq	%r12
140022ed7:     	popq	%r13
140022ed9:     	popq	%r14
140022edb:     	popq	%r15
140022edd:     	popq	%rbp
140022ede:     	retq
140022edf:     	movb	%sil, 0x7bc(%rbp)
140022ee6:     	andb	$0x1, %bl
140022ee9:     	movb	%bl, 0x7bb(%rbp)
140022eef:     	movaps	0x120(%rsp), %xmm6
140022ef7:     	movaps	0x110(%rsp), %xmm7
140022eff:     	movaps	0x100(%rsp), %xmm8
140022f08:     	movaps	0xf0(%rsp), %xmm9
140022f11:     	movaps	0xe0(%rsp), %xmm10
140022f1a:     	movapd	0xd0(%rsp), %xmm11
140022f24:     	movaps	0xc0(%rsp), %xmm12
140022f2d:     	movaps	0xb0(%rsp), %xmm13
140022f36:     	movaps	0xa0(%rsp), %xmm14
140022f3f:     	movaps	0x90(%rsp), %xmm15
140022f48:     	addq	$0x138, %rsp            # imm = 0x138
140022f4f:     	popq	%rbx
140022f50:     	popq	%rdi
140022f51:     	popq	%rsi
140022f52:     	popq	%r12
140022f54:     	popq	%r13
140022f56:     	popq	%r14
140022f58:     	popq	%r15
140022f5a:     	popq	%rbp
140022f5b:     	retq
140022f5c:     	nopl	(%rax)
140022f60:     	movq	%rdx, 0x10(%rsp)
140022f65:     	pushq	%rbp
140022f66:     	pushq	%r15
140022f68:     	pushq	%r14
140022f6a:     	pushq	%r13
140022f6c:     	pushq	%r12
140022f6e:     	pushq	%rsi
140022f6f:     	pushq	%rdi
140022f70:     	pushq	%rbx
140022f71:     	subq	$0x138, %rsp            # imm = 0x138
140022f78:     	leaq	0x80(%rdx), %rbp
140022f7f:     	movaps	%xmm15, 0x90(%rsp)
140022f88:     	movaps	%xmm14, 0xa0(%rsp)
140022f91:     	movaps	%xmm13, 0xb0(%rsp)
140022f9a:     	movaps	%xmm12, 0xc0(%rsp)
140022fa3:     	movapd	%xmm11, 0xd0(%rsp)
140022fad:     	movaps	%xmm10, 0xe0(%rsp)
140022fb6:     	movaps	%xmm9, 0xf0(%rsp)
140022fbf:     	movaps	%xmm8, 0x100(%rsp)
140022fc8:     	movaps	%xmm7, 0x110(%rsp)
140022fd0:     	movaps	%xmm6, 0x120(%rsp)
140022fd8:     	movzbl	0x7bb(%rbp), %ebx
140022fdf:     	testb	$0x1, 0x7bc(%rbp)
140022fe6:     	je	0x140023084 <.text+0x22084>
140022fec:     	movq	0x508(%rbp), %rdx
140022ff3:     	testq	%rdx, %rdx
140022ff6:     	je	0x14002300e <.text+0x2200e>
140022ff8:     	movq	0x510(%rbp), %rcx
140022fff:     	shlq	$0x3, %rdx
140023003:     	movl	$0x4, %r8d
140023009:     	callq	0x140613c20 <.text+0x612c20>
14002300e:     	andb	$0x1, %bl
140023011:     	movb	%bl, 0x7bd(%rbp)
140023017:     	movaps	0x120(%rsp), %xmm6
14002301f:     	movaps	0x110(%rsp), %xmm7
140023027:     	movaps	0x100(%rsp), %xmm8
140023030:     	movaps	0xf0(%rsp), %xmm9
140023039:     	movaps	0xe0(%rsp), %xmm10
140023042:     	movapd	0xd0(%rsp), %xmm11
14002304c:     	movaps	0xc0(%rsp), %xmm12
140023055:     	movaps	0xb0(%rsp), %xmm13
14002305e:     	movaps	0xa0(%rsp), %xmm14
140023067:     	movaps	0x90(%rsp), %xmm15
140023070:     	addq	$0x138, %rsp            # imm = 0x138
140023077:     	popq	%rbx
140023078:     	popq	%rdi
140023079:     	popq	%rsi
14002307a:     	popq	%r12
14002307c:     	popq	%r13
14002307e:     	popq	%r14
140023080:     	popq	%r15
140023082:     	popq	%rbp
140023083:     	retq
140023084:     	andb	$0x1, %bl
140023087:     	movb	%bl, 0x7bd(%rbp)
14002308d:     	movaps	0x120(%rsp), %xmm6
140023095:     	movaps	0x110(%rsp), %xmm7
14002309d:     	movaps	0x100(%rsp), %xmm8
1400230a6:     	movaps	0xf0(%rsp), %xmm9
1400230af:     	movaps	0xe0(%rsp), %xmm10
1400230b8:     	movapd	0xd0(%rsp), %xmm11
1400230c2:     	movaps	0xc0(%rsp), %xmm12
1400230cb:     	movaps	0xb0(%rsp), %xmm13
1400230d4:     	movaps	0xa0(%rsp), %xmm14
1400230dd:     	movaps	0x90(%rsp), %xmm15
1400230e6:     	addq	$0x138, %rsp            # imm = 0x138
1400230ed:     	popq	%rbx
1400230ee:     	popq	%rdi
1400230ef:     	popq	%rsi
1400230f0:     	popq	%r12
1400230f2:     	popq	%r13
1400230f4:     	popq	%r14
1400230f6:     	popq	%r15
1400230f8:     	popq	%rbp
1400230f9:     	retq
1400230fa:     	nopw	(%rax,%rax)
140023100:     	movq	%rdx, 0x10(%rsp)
140023105:     	pushq	%rbp
140023106:     	pushq	%r15
140023108:     	pushq	%r14
14002310a:     	pushq	%r13
14002310c:     	pushq	%r12
14002310e:     	pushq	%rsi
14002310f:     	pushq	%rdi
140023110:     	pushq	%rbx
140023111:     	subq	$0x138, %rsp            # imm = 0x138
140023118:     	leaq	0x80(%rdx), %rbp
14002311f:     	movdqa	%xmm15, 0x90(%rsp)
140023129:     	movaps	%xmm14, 0xa0(%rsp)
140023132:     	movdqa	%xmm13, 0xb0(%rsp)
14002313c:     	movdqa	%xmm12, 0xc0(%rsp)
140023146:     	movaps	%xmm11, 0xd0(%rsp)
14002314f:     	movaps	%xmm10, 0xe0(%rsp)
140023158:     	movaps	%xmm9, 0xf0(%rsp)
140023161:     	movaps	%xmm8, 0x100(%rsp)
14002316a:     	movaps	%xmm7, 0x110(%rsp)
140023172:     	movaps	%xmm6, 0x120(%rsp)
14002317a:     	movq	0x1b8(%rbp), %rcx
140023181:     	callq	0x1405df260 <.text+0x5de260>
140023186:     	movaps	0x120(%rsp), %xmm6
14002318e:     	movaps	0x110(%rsp), %xmm7
140023196:     	movaps	0x100(%rsp), %xmm8
14002319f:     	movaps	0xf0(%rsp), %xmm9
1400231a8:     	movaps	0xe0(%rsp), %xmm10
1400231b1:     	movaps	0xd0(%rsp), %xmm11
1400231ba:     	movaps	0xc0(%rsp), %xmm12
1400231c3:     	movaps	0xb0(%rsp), %xmm13
1400231cc:     	movaps	0xa0(%rsp), %xmm14
1400231d5:     	movaps	0x90(%rsp), %xmm15
1400231de:     	addq	$0x138, %rsp            # imm = 0x138
1400231e5:     	popq	%rbx
1400231e6:     	popq	%rdi
1400231e7:     	popq	%rsi
1400231e8:     	popq	%r12
1400231ea:     	popq	%r13
1400231ec:     	popq	%r14
1400231ee:     	popq	%r15
1400231f0:     	popq	%rbp
1400231f1:     	retq
1400231f2:     	nopw	%cs:(%rax,%rax)
140023200:     	movq	%rdx, 0x10(%rsp)
140023205:     	pushq	%rbp
140023206:     	pushq	%r15
140023208:     	pushq	%r14
14002320a:     	pushq	%r13
14002320c:     	pushq	%r12
14002320e:     	pushq	%rsi
14002320f:     	pushq	%rdi
140023210:     	pushq	%rbx
140023211:     	subq	$0x138, %rsp            # imm = 0x138
140023218:     	leaq	0x80(%rdx), %rbp
14002321f:     	movaps	%xmm15, 0x90(%rsp)
140023228:     	movaps	%xmm14, 0xa0(%rsp)
140023231:     	movaps	%xmm13, 0xb0(%rsp)
14002323a:     	movaps	%xmm12, 0xc0(%rsp)
140023243:     	movaps	%xmm11, 0xd0(%rsp)
14002324c:     	movaps	%xmm10, 0xe0(%rsp)
140023255:     	movaps	%xmm9, 0xf0(%rsp)
14002325e:     	movaps	%xmm8, 0x100(%rsp)
140023267:     	movaps	%xmm7, 0x110(%rsp)
14002326f:     	movaps	%xmm6, 0x120(%rsp)
140023277:     	cmpb	$0x0, 0x7bd(%rbp)
14002327e:     	je	0x140023306 <.text+0x22306>
140023284:     	leaq	0x460(%rbp), %rcx
14002328b:     	callq	0x14014ba20 <.text+0x14aa20>
140023290:     	movb	$0x1, 0x817(%rbp)
140023297:     	movaps	0x120(%rsp), %xmm6
14002329f:     	movaps	0x110(%rsp), %xmm7
1400232a7:     	movaps	0x100(%rsp), %xmm8
1400232b0:     	movaps	0xf0(%rsp), %xmm9
1400232b9:     	movaps	0xe0(%rsp), %xmm10
1400232c2:     	movaps	0xd0(%rsp), %xmm11
1400232cb:     	movdqa	0xc0(%rsp), %xmm12
1400232d5:     	movdqa	0xb0(%rsp), %xmm13
1400232df:     	movaps	0xa0(%rsp), %xmm14
1400232e8:     	movdqa	0x90(%rsp), %xmm15
1400232f2:     	addq	$0x138, %rsp            # imm = 0x138
1400232f9:     	popq	%rbx
1400232fa:     	popq	%rdi
1400232fb:     	popq	%rsi
1400232fc:     	popq	%r12
1400232fe:     	popq	%r13
140023300:     	popq	%r14
140023302:     	popq	%r15
140023304:     	popq	%rbp
140023305:     	retq
140023306:     	movb	$0x1, 0x817(%rbp)
14002330d:     	movaps	0x120(%rsp), %xmm6
140023315:     	movaps	0x110(%rsp), %xmm7
14002331d:     	movaps	0x100(%rsp), %xmm8
140023326:     	movaps	0xf0(%rsp), %xmm9
14002332f:     	movaps	0xe0(%rsp), %xmm10
140023338:     	movaps	0xd0(%rsp), %xmm11
140023341:     	movdqa	0xc0(%rsp), %xmm12
14002334b:     	movdqa	0xb0(%rsp), %xmm13
140023355:     	movaps	0xa0(%rsp), %xmm14
14002335e:     	movdqa	0x90(%rsp), %xmm15
140023368:     	addq	$0x138, %rsp            # imm = 0x138
14002336f:     	popq	%rbx
140023370:     	popq	%rdi
140023371:     	popq	%rsi
140023372:     	popq	%r12
140023374:     	popq	%r13
140023376:     	popq	%r14
140023378:     	popq	%r15
14002337a:     	popq	%rbp
14002337b:     	retq
14002337c:     	nopl	(%rax)
140023380:     	movq	%rdx, 0x10(%rsp)
140023385:     	pushq	%rbp
140023386:     	pushq	%r15
140023388:     	pushq	%r14
14002338a:     	pushq	%r13
14002338c:     	pushq	%r12
14002338e:     	pushq	%rsi
14002338f:     	pushq	%rdi
140023390:     	pushq	%rbx
140023391:     	subq	$0x138, %rsp            # imm = 0x138
140023398:     	leaq	0x80(%rdx), %rbp
14002339f:     	movdqa	%xmm15, 0x90(%rsp)
1400233a9:     	movaps	%xmm14, 0xa0(%rsp)
1400233b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400233bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400233c6:     	movaps	%xmm11, 0xd0(%rsp)
1400233cf:     	movaps	%xmm10, 0xe0(%rsp)
1400233d8:     	movaps	%xmm9, 0xf0(%rsp)
1400233e1:     	movaps	%xmm8, 0x100(%rsp)
1400233ea:     	movaps	%xmm7, 0x110(%rsp)
1400233f2:     	movaps	%xmm6, 0x120(%rsp)
1400233fa:     	movb	$0x0, 0x814(%rbp)
140023401:     	leaq	0x590(%rbp), %rcx
140023408:     	callq	0x140607a30 <.text+0x606a30>
14002340d:     	movb	$0x0, 0x814(%rbp)
140023414:     	movaps	0x120(%rsp), %xmm6
14002341c:     	movaps	0x110(%rsp), %xmm7
140023424:     	movaps	0x100(%rsp), %xmm8
14002342d:     	movaps	0xf0(%rsp), %xmm9
140023436:     	movaps	0xe0(%rsp), %xmm10
14002343f:     	movaps	0xd0(%rsp), %xmm11
140023448:     	movdqa	0xc0(%rsp), %xmm12
140023452:     	movdqa	0xb0(%rsp), %xmm13
14002345c:     	movaps	0xa0(%rsp), %xmm14
140023465:     	movdqa	0x90(%rsp), %xmm15
14002346f:     	addq	$0x138, %rsp            # imm = 0x138
140023476:     	popq	%rbx
140023477:     	popq	%rdi
140023478:     	popq	%rsi
140023479:     	popq	%r12
14002347b:     	popq	%r13
14002347d:     	popq	%r14
14002347f:     	popq	%r15
140023481:     	popq	%rbp
140023482:     	retq
140023483:     	nopw	%cs:(%rax,%rax)
140023490:     	movq	%rdx, 0x10(%rsp)
140023495:     	pushq	%rbp
140023496:     	pushq	%r15
140023498:     	pushq	%r14
14002349a:     	pushq	%r13
14002349c:     	pushq	%r12
14002349e:     	pushq	%rsi
14002349f:     	pushq	%rdi
1400234a0:     	pushq	%rbx
1400234a1:     	subq	$0x138, %rsp            # imm = 0x138
1400234a8:     	leaq	0x80(%rdx), %rbp
1400234af:     	movdqa	%xmm15, 0x90(%rsp)
1400234b9:     	movaps	%xmm14, 0xa0(%rsp)
1400234c2:     	movdqa	%xmm13, 0xb0(%rsp)
1400234cc:     	movdqa	%xmm12, 0xc0(%rsp)
1400234d6:     	movaps	%xmm11, 0xd0(%rsp)
1400234df:     	movaps	%xmm10, 0xe0(%rsp)
1400234e8:     	movaps	%xmm9, 0xf0(%rsp)
1400234f1:     	movaps	%xmm8, 0x100(%rsp)
1400234fa:     	movaps	%xmm7, 0x110(%rsp)
140023502:     	movaps	%xmm6, 0x120(%rsp)
14002350a:     	cmpq	$0x0, 0x808(%rbp)
140023512:     	je	0x140023535 <.text+0x22535>
140023514:     	movq	0x808(%rbp), %rax
14002351b:     	leaq	(,%rax,4), %rdx
140023523:     	movl	$0x4, %r8d
140023529:     	movq	0x6b0(%rbp), %rcx
140023530:     	callq	0x140613c20 <.text+0x612c20>
140023535:     	cmpq	$0x0, 0x7f0(%rbp)
14002353d:     	je	0x14002355c <.text+0x2255c>
14002353f:     	movq	0x7f0(%rbp), %rdx
140023546:     	shlq	$0x2, %rdx
14002354a:     	movl	$0x4, %r8d
140023550:     	movq	0x7d8(%rbp), %rcx
140023557:     	callq	0x140613c20 <.text+0x612c20>
14002355c:     	movb	$0x0, 0x79d(%rbp)
140023563:     	movaps	0x120(%rsp), %xmm6
14002356b:     	movaps	0x110(%rsp), %xmm7
140023573:     	movaps	0x100(%rsp), %xmm8
14002357c:     	movaps	0xf0(%rsp), %xmm9
140023585:     	movaps	0xe0(%rsp), %xmm10
14002358e:     	movaps	0xd0(%rsp), %xmm11
140023597:     	movdqa	0xc0(%rsp), %xmm12
1400235a1:     	movdqa	0xb0(%rsp), %xmm13
1400235ab:     	movaps	0xa0(%rsp), %xmm14
1400235b4:     	movdqa	0x90(%rsp), %xmm15
1400235be:     	addq	$0x138, %rsp            # imm = 0x138
1400235c5:     	popq	%rbx
1400235c6:     	popq	%rdi
1400235c7:     	popq	%rsi
1400235c8:     	popq	%r12
1400235ca:     	popq	%r13
1400235cc:     	popq	%r14
1400235ce:     	popq	%r15
1400235d0:     	popq	%rbp
1400235d1:     	retq
1400235d2:     	nopw	%cs:(%rax,%rax)
1400235e0:     	movq	%rdx, 0x10(%rsp)
1400235e5:     	pushq	%rbp
1400235e6:     	pushq	%r15
1400235e8:     	pushq	%r14
1400235ea:     	pushq	%r13
1400235ec:     	pushq	%r12
1400235ee:     	pushq	%rsi
1400235ef:     	pushq	%rdi
1400235f0:     	pushq	%rbx
1400235f1:     	subq	$0x138, %rsp            # imm = 0x138
1400235f8:     	leaq	0x80(%rdx), %rbp
1400235ff:     	movdqa	%xmm15, 0x90(%rsp)
140023609:     	movaps	%xmm14, 0xa0(%rsp)
140023612:     	movdqa	%xmm13, 0xb0(%rsp)
14002361c:     	movdqa	%xmm12, 0xc0(%rsp)
140023626:     	movaps	%xmm11, 0xd0(%rsp)
14002362f:     	movaps	%xmm10, 0xe0(%rsp)
140023638:     	movaps	%xmm9, 0xf0(%rsp)
140023641:     	movaps	%xmm8, 0x100(%rsp)
14002364a:     	movaps	%xmm7, 0x110(%rsp)
140023652:     	movaps	%xmm6, 0x120(%rsp)
14002365a:     	cmpq	$0x0, 0x808(%rbp)
140023662:     	je	0x140023685 <.text+0x22685>
140023664:     	movq	0x808(%rbp), %rax
14002366b:     	leaq	(,%rax,4), %rdx
140023673:     	movl	$0x4, %r8d
140023679:     	movq	0x6b0(%rbp), %rcx
140023680:     	callq	0x140613c20 <.text+0x612c20>
140023685:     	cmpq	$0x0, 0x7f0(%rbp)
14002368d:     	je	0x1400236ac <.text+0x226ac>
14002368f:     	movq	0x7f0(%rbp), %rdx
140023696:     	shlq	$0x2, %rdx
14002369a:     	movl	$0x4, %r8d
1400236a0:     	movq	0x7d8(%rbp), %rcx
1400236a7:     	callq	0x140613c20 <.text+0x612c20>
1400236ac:     	movb	$0x0, 0x79e(%rbp)
1400236b3:     	movaps	0x120(%rsp), %xmm6
1400236bb:     	movaps	0x110(%rsp), %xmm7
1400236c3:     	movaps	0x100(%rsp), %xmm8
1400236cc:     	movaps	0xf0(%rsp), %xmm9
1400236d5:     	movaps	0xe0(%rsp), %xmm10
1400236de:     	movaps	0xd0(%rsp), %xmm11
1400236e7:     	movdqa	0xc0(%rsp), %xmm12
1400236f1:     	movdqa	0xb0(%rsp), %xmm13
1400236fb:     	movaps	0xa0(%rsp), %xmm14
140023704:     	movdqa	0x90(%rsp), %xmm15
14002370e:     	addq	$0x138, %rsp            # imm = 0x138
140023715:     	popq	%rbx
140023716:     	popq	%rdi
140023717:     	popq	%rsi
140023718:     	popq	%r12
14002371a:     	popq	%r13
14002371c:     	popq	%r14
14002371e:     	popq	%r15
140023720:     	popq	%rbp
140023721:     	retq
140023722:     	nopw	%cs:(%rax,%rax)
140023730:     	movq	%rdx, 0x10(%rsp)
140023735:     	pushq	%rbp
140023736:     	pushq	%r15
140023738:     	pushq	%r14
14002373a:     	pushq	%r13
14002373c:     	pushq	%r12
14002373e:     	pushq	%rsi
14002373f:     	pushq	%rdi
140023740:     	pushq	%rbx
140023741:     	subq	$0x138, %rsp            # imm = 0x138
140023748:     	leaq	0x80(%rdx), %rbp
14002374f:     	movdqa	%xmm15, 0x90(%rsp)
140023759:     	movaps	%xmm14, 0xa0(%rsp)
140023762:     	movdqa	%xmm13, 0xb0(%rsp)
14002376c:     	movdqa	%xmm12, 0xc0(%rsp)
140023776:     	movaps	%xmm11, 0xd0(%rsp)
14002377f:     	movaps	%xmm10, 0xe0(%rsp)
140023788:     	movaps	%xmm9, 0xf0(%rsp)
140023791:     	movaps	%xmm8, 0x100(%rsp)
14002379a:     	movaps	%xmm7, 0x110(%rsp)
1400237a2:     	movaps	%xmm6, 0x120(%rsp)
1400237aa:     	movzbl	0x79d(%rbp), %ebx
1400237b1:     	cmpq	$0x0, 0x7a0(%rbp)
1400237b9:     	je	0x1400237d8 <.text+0x227d8>
1400237bb:     	movq	0x7a0(%rbp), %rdx
1400237c2:     	shlq	$0x3, %rdx
1400237c6:     	movl	$0x4, %r8d
1400237cc:     	movq	0x7d0(%rbp), %rcx
1400237d3:     	callq	0x140613c20 <.text+0x612c20>
1400237d8:     	leaq	0x590(%rbp), %rcx
1400237df:     	callq	0x14014ba20 <.text+0x14aa20>
1400237e4:     	andb	$0x1, %bl
1400237e7:     	movb	%bl, 0x7ce(%rbp)
1400237ed:     	movaps	0x120(%rsp), %xmm6
1400237f5:     	movaps	0x110(%rsp), %xmm7
1400237fd:     	movaps	0x100(%rsp), %xmm8
140023806:     	movaps	0xf0(%rsp), %xmm9
14002380f:     	movaps	0xe0(%rsp), %xmm10
140023818:     	movaps	0xd0(%rsp), %xmm11
140023821:     	movaps	0xc0(%rsp), %xmm12
14002382a:     	movaps	0xb0(%rsp), %xmm13
140023833:     	movaps	0xa0(%rsp), %xmm14
14002383c:     	movaps	0x90(%rsp), %xmm15
140023845:     	addq	$0x138, %rsp            # imm = 0x138
14002384c:     	popq	%rbx
14002384d:     	popq	%rdi
14002384e:     	popq	%rsi
14002384f:     	popq	%r12
140023851:     	popq	%r13
140023853:     	popq	%r14
140023855:     	popq	%r15
140023857:     	popq	%rbp
140023858:     	retq
140023859:     	nopl	(%rax)
140023860:     	movq	%rdx, 0x10(%rsp)
140023865:     	pushq	%rbp
140023866:     	pushq	%r15
140023868:     	pushq	%r14
14002386a:     	pushq	%r13
14002386c:     	pushq	%r12
14002386e:     	pushq	%rsi
14002386f:     	pushq	%rdi
140023870:     	pushq	%rbx
140023871:     	subq	$0x138, %rsp            # imm = 0x138
140023878:     	leaq	0x80(%rdx), %rbp
14002387f:     	movdqa	%xmm15, 0x90(%rsp)
140023889:     	movaps	%xmm14, 0xa0(%rsp)
140023892:     	movdqa	%xmm13, 0xb0(%rsp)
14002389c:     	movdqa	%xmm12, 0xc0(%rsp)
1400238a6:     	movaps	%xmm11, 0xd0(%rsp)
1400238af:     	movaps	%xmm10, 0xe0(%rsp)
1400238b8:     	movaps	%xmm9, 0xf0(%rsp)
1400238c1:     	movaps	%xmm8, 0x100(%rsp)
1400238ca:     	movaps	%xmm7, 0x110(%rsp)
1400238d2:     	movaps	%xmm6, 0x120(%rsp)
1400238da:     	movzbl	0x79e(%rbp), %ebx
1400238e1:     	cmpq	$0x0, 0x7a0(%rbp)
1400238e9:     	je	0x140023908 <.text+0x22908>
1400238eb:     	movq	0x7a0(%rbp), %rdx
1400238f2:     	shlq	$0x3, %rdx
1400238f6:     	movl	$0x4, %r8d
1400238fc:     	movq	0x7d0(%rbp), %rcx
140023903:     	callq	0x140613c20 <.text+0x612c20>
140023908:     	leaq	0x590(%rbp), %rcx
14002390f:     	callq	0x14014ba20 <.text+0x14aa20>
140023914:     	andb	$0x1, %bl
140023917:     	movb	%bl, 0x7cf(%rbp)
14002391d:     	movaps	0x120(%rsp), %xmm6
140023925:     	movaps	0x110(%rsp), %xmm7
14002392d:     	movaps	0x100(%rsp), %xmm8
140023936:     	movaps	0xf0(%rsp), %xmm9
14002393f:     	movaps	0xe0(%rsp), %xmm10
140023948:     	movaps	0xd0(%rsp), %xmm11
140023951:     	movaps	0xc0(%rsp), %xmm12
14002395a:     	movaps	0xb0(%rsp), %xmm13
140023963:     	movaps	0xa0(%rsp), %xmm14
14002396c:     	movaps	0x90(%rsp), %xmm15
140023975:     	addq	$0x138, %rsp            # imm = 0x138
14002397c:     	popq	%rbx
14002397d:     	popq	%rdi
14002397e:     	popq	%rsi
14002397f:     	popq	%r12
140023981:     	popq	%r13
140023983:     	popq	%r14
140023985:     	popq	%r15
140023987:     	popq	%rbp
140023988:     	retq
140023989:     	nopl	(%rax)
140023990:     	movq	%rdx, 0x10(%rsp)
140023995:     	pushq	%rbp
140023996:     	pushq	%r15
140023998:     	pushq	%r14
14002399a:     	pushq	%r13
14002399c:     	pushq	%r12
14002399e:     	pushq	%rsi
14002399f:     	pushq	%rdi
1400239a0:     	pushq	%rbx
1400239a1:     	subq	$0x138, %rsp            # imm = 0x138
1400239a8:     	leaq	0x80(%rdx), %rbp
1400239af:     	movaps	%xmm15, 0x90(%rsp)
1400239b8:     	movaps	%xmm14, 0xa0(%rsp)
1400239c1:     	movaps	%xmm13, 0xb0(%rsp)
1400239ca:     	movaps	%xmm12, 0xc0(%rsp)
1400239d3:     	movaps	%xmm11, 0xd0(%rsp)
1400239dc:     	movaps	%xmm10, 0xe0(%rsp)
1400239e5:     	movaps	%xmm9, 0xf0(%rsp)
1400239ee:     	movaps	%xmm8, 0x100(%rsp)
1400239f7:     	movaps	%xmm7, 0x110(%rsp)
1400239ff:     	movaps	%xmm6, 0x120(%rsp)
140023a07:     	cmpq	$0x0, 0x808(%rbp)
140023a0f:     	setne	%al
140023a12:     	testb	%al, 0x7ce(%rbp)
140023a18:     	je	0x140023a37 <.text+0x22a37>
140023a1a:     	movq	0x808(%rbp), %rdx
140023a21:     	shlq	$0x2, %rdx
140023a25:     	movl	$0x4, %r8d
140023a2b:     	movq	0x6b0(%rbp), %rcx
140023a32:     	callq	0x140613c20 <.text+0x612c20>
140023a37:     	movaps	0x120(%rsp), %xmm6
140023a3f:     	movaps	0x110(%rsp), %xmm7
140023a47:     	movaps	0x100(%rsp), %xmm8
140023a50:     	movaps	0xf0(%rsp), %xmm9
140023a59:     	movaps	0xe0(%rsp), %xmm10
140023a62:     	movaps	0xd0(%rsp), %xmm11
140023a6b:     	movaps	0xc0(%rsp), %xmm12
140023a74:     	movaps	0xb0(%rsp), %xmm13
140023a7d:     	movaps	0xa0(%rsp), %xmm14
140023a86:     	movaps	0x90(%rsp), %xmm15
140023a8f:     	addq	$0x138, %rsp            # imm = 0x138
140023a96:     	popq	%rbx
140023a97:     	popq	%rdi
140023a98:     	popq	%rsi
140023a99:     	popq	%r12
140023a9b:     	popq	%r13
140023a9d:     	popq	%r14
140023a9f:     	popq	%r15
140023aa1:     	popq	%rbp
140023aa2:     	retq
140023aa3:     	nopw	%cs:(%rax,%rax)
140023ab0:     	movq	%rdx, 0x10(%rsp)
140023ab5:     	pushq	%rbp
140023ab6:     	pushq	%r15
140023ab8:     	pushq	%r14
140023aba:     	pushq	%r13
140023abc:     	pushq	%r12
140023abe:     	pushq	%rsi
140023abf:     	pushq	%rdi
140023ac0:     	pushq	%rbx
140023ac1:     	subq	$0x138, %rsp            # imm = 0x138
140023ac8:     	leaq	0x80(%rdx), %rbp
140023acf:     	movaps	%xmm15, 0x90(%rsp)
140023ad8:     	movaps	%xmm14, 0xa0(%rsp)
140023ae1:     	movaps	%xmm13, 0xb0(%rsp)
140023aea:     	movaps	%xmm12, 0xc0(%rsp)
140023af3:     	movaps	%xmm11, 0xd0(%rsp)
140023afc:     	movaps	%xmm10, 0xe0(%rsp)
140023b05:     	movaps	%xmm9, 0xf0(%rsp)
140023b0e:     	movaps	%xmm8, 0x100(%rsp)
140023b17:     	movaps	%xmm7, 0x110(%rsp)
140023b1f:     	movaps	%xmm6, 0x120(%rsp)
140023b27:     	cmpq	$0x0, 0x808(%rbp)
140023b2f:     	setne	%al
140023b32:     	testb	%al, 0x7cf(%rbp)
140023b38:     	je	0x140023b57 <.text+0x22b57>
140023b3a:     	movq	0x808(%rbp), %rdx
140023b41:     	shlq	$0x2, %rdx
140023b45:     	movl	$0x4, %r8d
140023b4b:     	movq	0x6b0(%rbp), %rcx
140023b52:     	callq	0x140613c20 <.text+0x612c20>
140023b57:     	movaps	0x120(%rsp), %xmm6
140023b5f:     	movaps	0x110(%rsp), %xmm7
140023b67:     	movaps	0x100(%rsp), %xmm8
140023b70:     	movaps	0xf0(%rsp), %xmm9
140023b79:     	movaps	0xe0(%rsp), %xmm10
140023b82:     	movaps	0xd0(%rsp), %xmm11
140023b8b:     	movaps	0xc0(%rsp), %xmm12
140023b94:     	movaps	0xb0(%rsp), %xmm13
140023b9d:     	movaps	0xa0(%rsp), %xmm14
140023ba6:     	movaps	0x90(%rsp), %xmm15
140023baf:     	addq	$0x138, %rsp            # imm = 0x138
140023bb6:     	popq	%rbx
140023bb7:     	popq	%rdi
140023bb8:     	popq	%rsi
140023bb9:     	popq	%r12
140023bbb:     	popq	%r13
140023bbd:     	popq	%r14
140023bbf:     	popq	%r15
140023bc1:     	popq	%rbp
140023bc2:     	retq
140023bc3:     	nopw	%cs:(%rax,%rax)
140023bd0:     	movq	%rdx, 0x10(%rsp)
140023bd5:     	pushq	%rbp
140023bd6:     	pushq	%r15
140023bd8:     	pushq	%r14
140023bda:     	pushq	%r13
140023bdc:     	pushq	%r12
140023bde:     	pushq	%rsi
140023bdf:     	pushq	%rdi
140023be0:     	pushq	%rbx
140023be1:     	subq	$0x138, %rsp            # imm = 0x138
140023be8:     	leaq	0x80(%rdx), %rbp
140023bef:     	movdqa	%xmm15, 0x90(%rsp)
140023bf9:     	movaps	%xmm14, 0xa0(%rsp)
140023c02:     	movdqa	%xmm13, 0xb0(%rsp)
140023c0c:     	movdqa	%xmm12, 0xc0(%rsp)
140023c16:     	movaps	%xmm11, 0xd0(%rsp)
140023c1f:     	movaps	%xmm10, 0xe0(%rsp)
140023c28:     	movaps	%xmm9, 0xf0(%rsp)
140023c31:     	movaps	%xmm8, 0x100(%rsp)
140023c3a:     	movaps	%xmm7, 0x110(%rsp)
140023c42:     	movaps	%xmm6, 0x120(%rsp)
140023c4a:     	cmpq	$0x0, 0x808(%rbp)
140023c52:     	je	0x140023c75 <.text+0x22c75>
140023c54:     	movq	0x808(%rbp), %rax
140023c5b:     	shlq	$0x2, %rax
140023c5f:     	leaq	(%rax,%rax,2), %rdx
140023c63:     	movl	$0x4, %r8d
140023c69:     	movq	0x7a0(%rbp), %rcx
140023c70:     	callq	0x140613c20 <.text+0x612c20>
140023c75:     	movaps	0x120(%rsp), %xmm6
140023c7d:     	movaps	0x110(%rsp), %xmm7
140023c85:     	movaps	0x100(%rsp), %xmm8
140023c8e:     	movaps	0xf0(%rsp), %xmm9
140023c97:     	movaps	0xe0(%rsp), %xmm10
140023ca0:     	movaps	0xd0(%rsp), %xmm11
140023ca9:     	movdqa	0xc0(%rsp), %xmm12
140023cb3:     	movdqa	0xb0(%rsp), %xmm13
140023cbd:     	movaps	0xa0(%rsp), %xmm14
140023cc6:     	movdqa	0x90(%rsp), %xmm15
140023cd0:     	addq	$0x138, %rsp            # imm = 0x138
140023cd7:     	popq	%rbx
140023cd8:     	popq	%rdi
140023cd9:     	popq	%rsi
140023cda:     	popq	%r12
140023cdc:     	popq	%r13
140023cde:     	popq	%r14
140023ce0:     	popq	%r15
140023ce2:     	popq	%rbp
140023ce3:     	retq
140023ce4:     	nopw	%cs:(%rax,%rax)
140023cf0:     	movq	%rdx, 0x10(%rsp)
140023cf5:     	pushq	%rbp
140023cf6:     	pushq	%r15
140023cf8:     	pushq	%r14
140023cfa:     	pushq	%r13
140023cfc:     	pushq	%r12
140023cfe:     	pushq	%rsi
140023cff:     	pushq	%rdi
140023d00:     	pushq	%rbx
140023d01:     	subq	$0x138, %rsp            # imm = 0x138
140023d08:     	leaq	0x80(%rdx), %rbp
140023d0f:     	movdqa	%xmm15, 0x90(%rsp)
140023d19:     	movaps	%xmm14, 0xa0(%rsp)
140023d22:     	movdqa	%xmm13, 0xb0(%rsp)
140023d2c:     	movdqa	%xmm12, 0xc0(%rsp)
140023d36:     	movaps	%xmm11, 0xd0(%rsp)
140023d3f:     	movaps	%xmm10, 0xe0(%rsp)
140023d48:     	movaps	%xmm9, 0xf0(%rsp)
140023d51:     	movaps	%xmm8, 0x100(%rsp)
140023d5a:     	movaps	%xmm7, 0x110(%rsp)
140023d62:     	movaps	%xmm6, 0x120(%rsp)
140023d6a:     	movaps	0x4d0(%rbp), %xmm0
140023d71:     	movaps	%xmm0, 0x730(%rbp)
140023d78:     	movaps	0x4c0(%rbp), %xmm0
140023d7f:     	movaps	%xmm0, 0x720(%rbp)
140023d86:     	movaps	0x4b0(%rbp), %xmm0
140023d8d:     	movaps	%xmm0, 0x710(%rbp)
140023d94:     	movaps	0x4a0(%rbp), %xmm0
140023d9b:     	movaps	%xmm0, 0x700(%rbp)
140023da2:     	movaps	0x460(%rbp), %xmm0
140023da9:     	movdqa	0x470(%rbp), %xmm1
140023db1:     	movdqa	0x480(%rbp), %xmm2
140023db9:     	movapd	0x490(%rbp), %xmm3
140023dc1:     	movapd	%xmm3, 0x6f0(%rbp)
140023dc9:     	movdqa	%xmm2, 0x6e0(%rbp)
140023dd1:     	movdqa	%xmm1, 0x6d0(%rbp)
140023dd9:     	movaps	%xmm0, 0x6c0(%rbp)
140023de0:     	movq	0x678(%rbp), %rax
140023de7:     	movq	%rax, 0x740(%rbp)
140023dee:     	movl	0x588(%rbp), %eax
140023df4:     	movl	%eax, 0x748(%rbp)
140023dfa:     	movl	0x640(%rbp), %eax
140023e00:     	movl	%eax, 0x74c(%rbp)
140023e06:     	movl	0x528(%rbp), %eax
140023e0c:     	movl	%eax, 0x750(%rbp)
140023e12:     	movdqa	0x370(%rbp), %xmm0
140023e1a:     	movd	%xmm0, %eax
140023e1e:     	movw	%ax, 0x754(%rbp)
140023e25:     	movw	$0x0, 0x756(%rbp)
140023e2e:     	movzwl	0x548(%rbp), %eax
140023e35:     	movw	%ax, 0x758(%rbp)
140023e3c:     	movw	$0x300, 0x75a(%rbp)     # imm = 0x300
140023e45:     	movq	0x500(%rbp), %rax
140023e4c:     	movq	%rax, 0x760(%rbp)
140023e53:     	movq	0x540(%rbp), %rax
140023e5a:     	movq	%rax, 0x768(%rbp)
140023e61:     	movb	$0x0, 0x770(%rbp)
140023e68:     	movb	$0x0, 0x7c2(%rbp)
140023e6f:     	movb	$0x0, 0x7c1(%rbp)
140023e76:     	movaps	0x120(%rsp), %xmm6
140023e7e:     	movaps	0x110(%rsp), %xmm7
140023e86:     	movaps	0x100(%rsp), %xmm8
140023e8f:     	movaps	0xf0(%rsp), %xmm9
140023e98:     	movaps	0xe0(%rsp), %xmm10
140023ea1:     	movaps	0xd0(%rsp), %xmm11
140023eaa:     	movdqa	0xc0(%rsp), %xmm12
140023eb4:     	movdqa	0xb0(%rsp), %xmm13
140023ebe:     	movaps	0xa0(%rsp), %xmm14
140023ec7:     	movdqa	0x90(%rsp), %xmm15
140023ed1:     	addq	$0x138, %rsp            # imm = 0x138
140023ed8:     	popq	%rbx
140023ed9:     	popq	%rdi
140023eda:     	popq	%rsi
140023edb:     	popq	%r12
140023edd:     	popq	%r13
140023edf:     	popq	%r14
140023ee1:     	popq	%r15
140023ee3:     	popq	%rbp
140023ee4:     	retq
140023ee5:     	nopw	%cs:(%rax,%rax)
140023ef0:     	movq	%rdx, 0x10(%rsp)
140023ef5:     	pushq	%rbp
140023ef6:     	pushq	%r15
140023ef8:     	pushq	%r14
140023efa:     	pushq	%r13
140023efc:     	pushq	%r12
140023efe:     	pushq	%rsi
140023eff:     	pushq	%rdi
140023f00:     	pushq	%rbx
140023f01:     	subq	$0x138, %rsp            # imm = 0x138
140023f08:     	leaq	0x80(%rdx), %rbp
140023f0f:     	movdqa	%xmm15, 0x90(%rsp)
140023f19:     	movaps	%xmm14, 0xa0(%rsp)
140023f22:     	movdqa	%xmm13, 0xb0(%rsp)
140023f2c:     	movdqa	%xmm12, 0xc0(%rsp)
140023f36:     	movaps	%xmm11, 0xd0(%rsp)
140023f3f:     	movaps	%xmm10, 0xe0(%rsp)
140023f48:     	movaps	%xmm9, 0xf0(%rsp)
140023f51:     	movaps	%xmm8, 0x100(%rsp)
140023f5a:     	movaps	%xmm7, 0x110(%rsp)
140023f62:     	movaps	%xmm6, 0x120(%rsp)
140023f6a:     	leaq	0x6c0(%rbp), %rcx
140023f71:     	callq	0x14014ba20 <.text+0x14aa20>
140023f76:     	movaps	0x120(%rsp), %xmm6
140023f7e:     	movaps	0x110(%rsp), %xmm7
140023f86:     	movaps	0x100(%rsp), %xmm8
140023f8f:     	movaps	0xf0(%rsp), %xmm9
140023f98:     	movaps	0xe0(%rsp), %xmm10
140023fa1:     	movaps	0xd0(%rsp), %xmm11
140023faa:     	movdqa	0xc0(%rsp), %xmm12
140023fb4:     	movdqa	0xb0(%rsp), %xmm13
140023fbe:     	movaps	0xa0(%rsp), %xmm14
140023fc7:     	movdqa	0x90(%rsp), %xmm15
140023fd1:     	addq	$0x138, %rsp            # imm = 0x138
140023fd8:     	popq	%rbx
140023fd9:     	popq	%rdi
140023fda:     	popq	%rsi
140023fdb:     	popq	%r12
140023fdd:     	popq	%r13
140023fdf:     	popq	%r14
140023fe1:     	popq	%r15
140023fe3:     	popq	%rbp
140023fe4:     	retq
140023fe5:     	nopw	%cs:(%rax,%rax)
140023ff0:     	movq	%rdx, 0x10(%rsp)
140023ff5:     	pushq	%rbp
140023ff6:     	pushq	%r15
140023ff8:     	pushq	%r14
140023ffa:     	pushq	%r13
140023ffc:     	pushq	%r12
140023ffe:     	pushq	%rsi
140023fff:     	pushq	%rdi
140024000:     	pushq	%rbx
140024001:     	subq	$0x138, %rsp            # imm = 0x138
140024008:     	leaq	0x80(%rdx), %rbp
14002400f:     	movdqa	%xmm15, 0x90(%rsp)
140024019:     	movaps	%xmm14, 0xa0(%rsp)
140024022:     	movdqa	%xmm13, 0xb0(%rsp)
14002402c:     	movdqa	%xmm12, 0xc0(%rsp)
140024036:     	movaps	%xmm11, 0xd0(%rsp)
14002403f:     	movaps	%xmm10, 0xe0(%rsp)
140024048:     	movaps	%xmm9, 0xf0(%rsp)
140024051:     	movaps	%xmm8, 0x100(%rsp)
14002405a:     	movaps	%xmm7, 0x110(%rsp)
140024062:     	movaps	%xmm6, 0x120(%rsp)
14002406a:     	cmpq	$0x0, 0x790(%rbp)
140024072:     	je	0x140024095 <.text+0x23095>
140024074:     	movq	0x790(%rbp), %rax
14002407b:     	shlq	$0x2, %rax
14002407f:     	leaq	(%rax,%rax,2), %rdx
140024083:     	movl	$0x4, %r8d
140024089:     	movq	0x808(%rbp), %rcx
140024090:     	callq	0x140613c20 <.text+0x612c20>
140024095:     	movaps	0x120(%rsp), %xmm6
14002409d:     	movaps	0x110(%rsp), %xmm7
1400240a5:     	movaps	0x100(%rsp), %xmm8
1400240ae:     	movaps	0xf0(%rsp), %xmm9
1400240b7:     	movaps	0xe0(%rsp), %xmm10
1400240c0:     	movaps	0xd0(%rsp), %xmm11
1400240c9:     	movdqa	0xc0(%rsp), %xmm12
1400240d3:     	movdqa	0xb0(%rsp), %xmm13
1400240dd:     	movaps	0xa0(%rsp), %xmm14
1400240e6:     	movdqa	0x90(%rsp), %xmm15
1400240f0:     	addq	$0x138, %rsp            # imm = 0x138
1400240f7:     	popq	%rbx
1400240f8:     	popq	%rdi
1400240f9:     	popq	%rsi
1400240fa:     	popq	%r12
1400240fc:     	popq	%r13
1400240fe:     	popq	%r14
140024100:     	popq	%r15
140024102:     	popq	%rbp
140024103:     	retq
140024104:     	nopw	%cs:(%rax,%rax)
140024110:     	movq	%rdx, 0x10(%rsp)
140024115:     	pushq	%rbp
140024116:     	pushq	%r15
140024118:     	pushq	%r14
14002411a:     	pushq	%r13
14002411c:     	pushq	%r12
14002411e:     	pushq	%rsi
14002411f:     	pushq	%rdi
140024120:     	pushq	%rbx
140024121:     	subq	$0x138, %rsp            # imm = 0x138
140024128:     	leaq	0x80(%rdx), %rbp
14002412f:     	movdqa	%xmm15, 0x90(%rsp)
140024139:     	movaps	%xmm14, 0xa0(%rsp)
140024142:     	movdqa	%xmm13, 0xb0(%rsp)
14002414c:     	movdqa	%xmm12, 0xc0(%rsp)
140024156:     	movaps	%xmm11, 0xd0(%rsp)
14002415f:     	movaps	%xmm10, 0xe0(%rsp)
140024168:     	movaps	%xmm9, 0xf0(%rsp)
140024171:     	movaps	%xmm8, 0x100(%rsp)
14002417a:     	movaps	%xmm7, 0x110(%rsp)
140024182:     	movaps	%xmm6, 0x120(%rsp)
14002418a:     	cmpq	$0x0, 0x790(%rbp)
140024192:     	je	0x1400241b1 <.text+0x231b1>
140024194:     	movq	0x790(%rbp), %rdx
14002419b:     	shlq	$0x3, %rdx
14002419f:     	movl	$0x4, %r8d
1400241a5:     	movq	0x2f8(%rbp), %rcx
1400241ac:     	callq	0x140613c20 <.text+0x612c20>
1400241b1:     	movaps	0x120(%rsp), %xmm6
1400241b9:     	movaps	0x110(%rsp), %xmm7
1400241c1:     	movaps	0x100(%rsp), %xmm8
1400241ca:     	movaps	0xf0(%rsp), %xmm9
1400241d3:     	movaps	0xe0(%rsp), %xmm10
1400241dc:     	movaps	0xd0(%rsp), %xmm11
1400241e5:     	movdqa	0xc0(%rsp), %xmm12
1400241ef:     	movdqa	0xb0(%rsp), %xmm13
1400241f9:     	movaps	0xa0(%rsp), %xmm14
140024202:     	movdqa	0x90(%rsp), %xmm15
14002420c:     	addq	$0x138, %rsp            # imm = 0x138
140024213:     	popq	%rbx
140024214:     	popq	%rdi
140024215:     	popq	%rsi
140024216:     	popq	%r12
140024218:     	popq	%r13
14002421a:     	popq	%r14
14002421c:     	popq	%r15
14002421e:     	popq	%rbp
14002421f:     	retq
140024220:     	movq	%rdx, 0x10(%rsp)
140024225:     	pushq	%rbp
140024226:     	pushq	%r15
140024228:     	pushq	%r14
14002422a:     	pushq	%r13
14002422c:     	pushq	%r12
14002422e:     	pushq	%rsi
14002422f:     	pushq	%rdi
140024230:     	pushq	%rbx
140024231:     	subq	$0x138, %rsp            # imm = 0x138
140024238:     	leaq	0x80(%rdx), %rbp
14002423f:     	movdqa	%xmm15, 0x90(%rsp)
140024249:     	movaps	%xmm14, 0xa0(%rsp)
140024252:     	movdqa	%xmm13, 0xb0(%rsp)
14002425c:     	movdqa	%xmm12, 0xc0(%rsp)
140024266:     	movaps	%xmm11, 0xd0(%rsp)
14002426f:     	movaps	%xmm10, 0xe0(%rsp)
140024278:     	movaps	%xmm9, 0xf0(%rsp)
140024281:     	movaps	%xmm8, 0x100(%rsp)
14002428a:     	movaps	%xmm7, 0x110(%rsp)
140024292:     	movaps	%xmm6, 0x120(%rsp)
14002429a:     	movq	0xd8(%rbp), %rdx
1400242a1:     	testq	%rdx, %rdx
1400242a4:     	je	0x1400242bc <.text+0x232bc>
1400242a6:     	movq	0xe0(%rbp), %rcx
1400242ad:     	shlq	$0x2, %rdx
1400242b1:     	movl	$0x4, %r8d
1400242b7:     	callq	0x140613c20 <.text+0x612c20>
1400242bc:     	movaps	0x120(%rsp), %xmm6
1400242c4:     	movaps	0x110(%rsp), %xmm7
1400242cc:     	movaps	0x100(%rsp), %xmm8
1400242d5:     	movaps	0xf0(%rsp), %xmm9
1400242de:     	movaps	0xe0(%rsp), %xmm10
1400242e7:     	movaps	0xd0(%rsp), %xmm11
1400242f0:     	movdqa	0xc0(%rsp), %xmm12
1400242fa:     	movdqa	0xb0(%rsp), %xmm13
140024304:     	movaps	0xa0(%rsp), %xmm14
14002430d:     	movdqa	0x90(%rsp), %xmm15
140024317:     	addq	$0x138, %rsp            # imm = 0x138
14002431e:     	popq	%rbx
14002431f:     	popq	%rdi
140024320:     	popq	%rsi
140024321:     	popq	%r12
140024323:     	popq	%r13
140024325:     	popq	%r14
140024327:     	popq	%r15
140024329:     	popq	%rbp
14002432a:     	retq
14002432b:     	nopl	(%rax,%rax)
140024330:     	movq	%rdx, 0x10(%rsp)
140024335:     	pushq	%rbp
140024336:     	pushq	%r15
140024338:     	pushq	%r14
14002433a:     	pushq	%r13
14002433c:     	pushq	%r12
14002433e:     	pushq	%rsi
14002433f:     	pushq	%rdi
140024340:     	pushq	%rbx
140024341:     	subq	$0x138, %rsp            # imm = 0x138
140024348:     	leaq	0x80(%rdx), %rbp
14002434f:     	movdqa	%xmm15, 0x90(%rsp)
140024359:     	movaps	%xmm14, 0xa0(%rsp)
140024362:     	movdqa	%xmm13, 0xb0(%rsp)
14002436c:     	movdqa	%xmm12, 0xc0(%rsp)
140024376:     	movaps	%xmm11, 0xd0(%rsp)
14002437f:     	movaps	%xmm10, 0xe0(%rsp)
140024388:     	movaps	%xmm9, 0xf0(%rsp)
140024391:     	movaps	%xmm8, 0x100(%rsp)
14002439a:     	movaps	%xmm7, 0x110(%rsp)
1400243a2:     	movaps	%xmm6, 0x120(%rsp)
1400243aa:     	movq	0xc8(%rbp), %rdx
1400243b1:     	testq	%rdx, %rdx
1400243b4:     	je	0x1400243cc <.text+0x233cc>
1400243b6:     	movq	0xd0(%rbp), %rcx
1400243bd:     	shlq	$0x2, %rdx
1400243c1:     	movl	$0x4, %r8d
1400243c7:     	callq	0x140613c20 <.text+0x612c20>
1400243cc:     	movaps	0x120(%rsp), %xmm6
1400243d4:     	movaps	0x110(%rsp), %xmm7
1400243dc:     	movaps	0x100(%rsp), %xmm8
1400243e5:     	movaps	0xf0(%rsp), %xmm9
1400243ee:     	movaps	0xe0(%rsp), %xmm10
1400243f7:     	movaps	0xd0(%rsp), %xmm11
140024400:     	movdqa	0xc0(%rsp), %xmm12
14002440a:     	movdqa	0xb0(%rsp), %xmm13
140024414:     	movaps	0xa0(%rsp), %xmm14
14002441d:     	movdqa	0x90(%rsp), %xmm15
140024427:     	addq	$0x138, %rsp            # imm = 0x138
14002442e:     	popq	%rbx
14002442f:     	popq	%rdi
140024430:     	popq	%rsi
140024431:     	popq	%r12
140024433:     	popq	%r13
140024435:     	popq	%r14
140024437:     	popq	%r15
140024439:     	popq	%rbp
14002443a:     	retq
14002443b:     	nopl	(%rax,%rax)
140024440:     	movq	%rdx, 0x10(%rsp)
140024445:     	pushq	%rbp
140024446:     	pushq	%r15
140024448:     	pushq	%r14
14002444a:     	pushq	%r13
14002444c:     	pushq	%r12
14002444e:     	pushq	%rsi
14002444f:     	pushq	%rdi
140024450:     	pushq	%rbx
140024451:     	subq	$0x138, %rsp            # imm = 0x138
140024458:     	leaq	0x80(%rdx), %rbp
14002445f:     	movdqa	%xmm15, 0x90(%rsp)
140024469:     	movaps	%xmm14, 0xa0(%rsp)
140024472:     	movdqa	%xmm13, 0xb0(%rsp)
14002447c:     	movdqa	%xmm12, 0xc0(%rsp)
140024486:     	movaps	%xmm11, 0xd0(%rsp)
14002448f:     	movaps	%xmm10, 0xe0(%rsp)
140024498:     	movaps	%xmm9, 0xf0(%rsp)
1400244a1:     	movaps	%xmm8, 0x100(%rsp)
1400244aa:     	movaps	%xmm7, 0x110(%rsp)
1400244b2:     	movaps	%xmm6, 0x120(%rsp)
1400244ba:     	movq	0x7d8(%rbp), %rdx
1400244c1:     	testq	%rdx, %rdx
1400244c4:     	je	0x1400244dc <.text+0x234dc>
1400244c6:     	shlq	$0x2, %rdx
1400244ca:     	movl	$0x4, %r8d
1400244d0:     	movq	0x138(%rbp), %rcx
1400244d7:     	callq	0x140613c20 <.text+0x612c20>
1400244dc:     	movaps	0x120(%rsp), %xmm6
1400244e4:     	movaps	0x110(%rsp), %xmm7
1400244ec:     	movaps	0x100(%rsp), %xmm8
1400244f5:     	movaps	0xf0(%rsp), %xmm9
1400244fe:     	movaps	0xe0(%rsp), %xmm10
140024507:     	movaps	0xd0(%rsp), %xmm11
140024510:     	movdqa	0xc0(%rsp), %xmm12
14002451a:     	movdqa	0xb0(%rsp), %xmm13
140024524:     	movaps	0xa0(%rsp), %xmm14
14002452d:     	movdqa	0x90(%rsp), %xmm15
140024537:     	addq	$0x138, %rsp            # imm = 0x138
14002453e:     	popq	%rbx
14002453f:     	popq	%rdi
140024540:     	popq	%rsi
140024541:     	popq	%r12
140024543:     	popq	%r13
140024545:     	popq	%r14
140024547:     	popq	%r15
140024549:     	popq	%rbp
14002454a:     	retq
14002454b:     	nopl	(%rax,%rax)
140024550:     	movq	%rdx, 0x10(%rsp)
140024555:     	pushq	%rbp
140024556:     	pushq	%r15
140024558:     	pushq	%r14
14002455a:     	pushq	%r13
14002455c:     	pushq	%r12
14002455e:     	pushq	%rsi
14002455f:     	pushq	%rdi
140024560:     	pushq	%rbx
140024561:     	subq	$0x138, %rsp            # imm = 0x138
140024568:     	leaq	0x80(%rdx), %rbp
14002456f:     	movdqa	%xmm15, 0x90(%rsp)
140024579:     	movaps	%xmm14, 0xa0(%rsp)
140024582:     	movdqa	%xmm13, 0xb0(%rsp)
14002458c:     	movdqa	%xmm12, 0xc0(%rsp)
140024596:     	movaps	%xmm11, 0xd0(%rsp)
14002459f:     	movaps	%xmm10, 0xe0(%rsp)
1400245a8:     	movaps	%xmm9, 0xf0(%rsp)
1400245b1:     	movaps	%xmm8, 0x100(%rsp)
1400245ba:     	movaps	%xmm7, 0x110(%rsp)
1400245c2:     	movaps	%xmm6, 0x120(%rsp)
1400245ca:     	movzbl	0x7c1(%rbp), %ebx
1400245d1:     	movzbl	0x7c2(%rbp), %esi
1400245d8:     	testb	$0x1, %sil
1400245dc:     	je	0x14002468a <.text+0x2368a>
1400245e2:     	movq	0x3e0(%rbp), %rdx
1400245e9:     	testq	%rdx, %rdx
1400245ec:     	je	0x140024604 <.text+0x23604>
1400245ee:     	movq	0x3e8(%rbp), %rcx
1400245f5:     	shlq	$0x2, %rdx
1400245f9:     	movl	$0x4, %r8d
1400245ff:     	callq	0x140613c20 <.text+0x612c20>
140024604:     	movb	%sil, 0x7c5(%rbp)
14002460b:     	andb	$0x1, %bl
14002460e:     	movb	%bl, 0x7c4(%rbp)
140024614:     	movb	$0x0, 0x7c3(%rbp)
14002461b:     	movaps	0x120(%rsp), %xmm6
140024623:     	movaps	0x110(%rsp), %xmm7
14002462b:     	movaps	0x100(%rsp), %xmm8
140024634:     	movaps	0xf0(%rsp), %xmm9
14002463d:     	movaps	0xe0(%rsp), %xmm10
140024646:     	movaps	0xd0(%rsp), %xmm11
14002464f:     	movdqa	0xc0(%rsp), %xmm12
140024659:     	movdqa	0xb0(%rsp), %xmm13
140024663:     	movaps	0xa0(%rsp), %xmm14
14002466c:     	movdqa	0x90(%rsp), %xmm15
140024676:     	addq	$0x138, %rsp            # imm = 0x138
14002467d:     	popq	%rbx
14002467e:     	popq	%rdi
14002467f:     	popq	%rsi
140024680:     	popq	%r12
140024682:     	popq	%r13
140024684:     	popq	%r14
140024686:     	popq	%r15
140024688:     	popq	%rbp
140024689:     	retq
14002468a:     	movb	%sil, 0x7c5(%rbp)
140024691:     	andb	$0x1, %bl
140024694:     	movb	%bl, 0x7c4(%rbp)
14002469a:     	movb	$0x0, 0x7c3(%rbp)
1400246a1:     	movaps	0x120(%rsp), %xmm6
1400246a9:     	movaps	0x110(%rsp), %xmm7
1400246b1:     	movaps	0x100(%rsp), %xmm8
1400246ba:     	movaps	0xf0(%rsp), %xmm9
1400246c3:     	movaps	0xe0(%rsp), %xmm10
1400246cc:     	movaps	0xd0(%rsp), %xmm11
1400246d5:     	movdqa	0xc0(%rsp), %xmm12
1400246df:     	movdqa	0xb0(%rsp), %xmm13
1400246e9:     	movaps	0xa0(%rsp), %xmm14
1400246f2:     	movdqa	0x90(%rsp), %xmm15
1400246fc:     	addq	$0x138, %rsp            # imm = 0x138
140024703:     	popq	%rbx
140024704:     	popq	%rdi
140024705:     	popq	%rsi
140024706:     	popq	%r12
140024708:     	popq	%r13
14002470a:     	popq	%r14
14002470c:     	popq	%r15
14002470e:     	popq	%rbp
14002470f:     	retq
140024710:     	movq	%rdx, 0x10(%rsp)
140024715:     	pushq	%rbp
140024716:     	pushq	%r15
140024718:     	pushq	%r14
14002471a:     	pushq	%r13
14002471c:     	pushq	%r12
14002471e:     	pushq	%rsi
14002471f:     	pushq	%rdi
140024720:     	pushq	%rbx
140024721:     	subq	$0x138, %rsp            # imm = 0x138
140024728:     	leaq	0x80(%rdx), %rbp
14002472f:     	movdqa	%xmm15, 0x90(%rsp)
140024739:     	movaps	%xmm14, 0xa0(%rsp)
140024742:     	movdqa	%xmm13, 0xb0(%rsp)
14002474c:     	movdqa	%xmm12, 0xc0(%rsp)
140024756:     	movaps	%xmm11, 0xd0(%rsp)
14002475f:     	movaps	%xmm10, 0xe0(%rsp)
140024768:     	movaps	%xmm9, 0xf0(%rsp)
140024771:     	movaps	%xmm8, 0x100(%rsp)
14002477a:     	movaps	%xmm7, 0x110(%rsp)
140024782:     	movaps	%xmm6, 0x120(%rsp)
14002478a:     	movq	0x548(%rbp), %rdx
140024791:     	testq	%rdx, %rdx
140024794:     	je	0x1400247ac <.text+0x237ac>
140024796:     	shlq	$0x3, %rdx
14002479a:     	movl	$0x4, %r8d
1400247a0:     	movq	0x2f0(%rbp), %rcx
1400247a7:     	callq	0x140613c20 <.text+0x612c20>
1400247ac:     	movaps	0x120(%rsp), %xmm6
1400247b4:     	movaps	0x110(%rsp), %xmm7
1400247bc:     	movaps	0x100(%rsp), %xmm8
1400247c5:     	movaps	0xf0(%rsp), %xmm9
1400247ce:     	movaps	0xe0(%rsp), %xmm10
1400247d7:     	movaps	0xd0(%rsp), %xmm11
1400247e0:     	movdqa	0xc0(%rsp), %xmm12
1400247ea:     	movdqa	0xb0(%rsp), %xmm13
1400247f4:     	movaps	0xa0(%rsp), %xmm14
1400247fd:     	movdqa	0x90(%rsp), %xmm15
140024807:     	addq	$0x138, %rsp            # imm = 0x138
14002480e:     	popq	%rbx
14002480f:     	popq	%rdi
140024810:     	popq	%rsi
140024811:     	popq	%r12
140024813:     	popq	%r13
140024815:     	popq	%r14
140024817:     	popq	%r15
140024819:     	popq	%rbp
14002481a:     	retq
14002481b:     	nopl	(%rax,%rax)
140024820:     	movq	%rdx, 0x10(%rsp)
140024825:     	pushq	%rbp
140024826:     	pushq	%r15
140024828:     	pushq	%r14
14002482a:     	pushq	%r13
14002482c:     	pushq	%r12
14002482e:     	pushq	%rsi
14002482f:     	pushq	%rdi
140024830:     	pushq	%rbx
140024831:     	subq	$0x138, %rsp            # imm = 0x138
140024838:     	leaq	0x80(%rdx), %rbp
14002483f:     	movdqa	%xmm15, 0x90(%rsp)
140024849:     	movaps	%xmm14, 0xa0(%rsp)
140024852:     	movdqa	%xmm13, 0xb0(%rsp)
14002485c:     	movdqa	%xmm12, 0xc0(%rsp)
140024866:     	movaps	%xmm11, 0xd0(%rsp)
14002486f:     	movaps	%xmm10, 0xe0(%rsp)
140024878:     	movaps	%xmm9, 0xf0(%rsp)
140024881:     	movaps	%xmm8, 0x100(%rsp)
14002488a:     	movaps	%xmm7, 0x110(%rsp)
140024892:     	movaps	%xmm6, 0x120(%rsp)
14002489a:     	leaq	0x590(%rbp), %rcx
1400248a1:     	callq	0x14014ba20 <.text+0x14aa20>
1400248a6:     	movb	$0x1, 0x816(%rbp)
1400248ad:     	movb	$0x1, 0x815(%rbp)
1400248b4:     	movaps	0x120(%rsp), %xmm6
1400248bc:     	movaps	0x110(%rsp), %xmm7
1400248c4:     	movaps	0x100(%rsp), %xmm8
1400248cd:     	movaps	0xf0(%rsp), %xmm9
1400248d6:     	movaps	0xe0(%rsp), %xmm10
1400248df:     	movaps	0xd0(%rsp), %xmm11
1400248e8:     	movdqa	0xc0(%rsp), %xmm12
1400248f2:     	movdqa	0xb0(%rsp), %xmm13
1400248fc:     	movaps	0xa0(%rsp), %xmm14
140024905:     	movdqa	0x90(%rsp), %xmm15
14002490f:     	addq	$0x138, %rsp            # imm = 0x138
140024916:     	popq	%rbx
140024917:     	popq	%rdi
140024918:     	popq	%rsi
140024919:     	popq	%r12
14002491b:     	popq	%r13
14002491d:     	popq	%r14
14002491f:     	popq	%r15
140024921:     	popq	%rbp
140024922:     	retq
140024923:     	nopw	%cs:(%rax,%rax)
140024930:     	movq	%rdx, 0x10(%rsp)
140024935:     	pushq	%rbp
140024936:     	pushq	%r15
140024938:     	pushq	%r14
14002493a:     	pushq	%r13
14002493c:     	pushq	%r12
14002493e:     	pushq	%rsi
14002493f:     	pushq	%rdi
140024940:     	pushq	%rbx
140024941:     	subq	$0x138, %rsp            # imm = 0x138
140024948:     	leaq	0x80(%rdx), %rbp
14002494f:     	movdqa	%xmm15, 0x90(%rsp)
140024959:     	movaps	%xmm14, 0xa0(%rsp)
140024962:     	movdqa	%xmm13, 0xb0(%rsp)
14002496c:     	movdqa	%xmm12, 0xc0(%rsp)
140024976:     	movaps	%xmm11, 0xd0(%rsp)
14002497f:     	movaps	%xmm10, 0xe0(%rsp)
140024988:     	movaps	%xmm9, 0xf0(%rsp)
140024991:     	movaps	%xmm8, 0x100(%rsp)
14002499a:     	movaps	%xmm7, 0x110(%rsp)
1400249a2:     	movaps	%xmm6, 0x120(%rsp)
1400249aa:     	movzbl	0x7c3(%rbp), %ebx
1400249b1:     	movzbl	0x7c4(%rbp), %esi
1400249b8:     	movzbl	0x7c5(%rbp), %edi
1400249bf:     	testb	$0x1, %dil
1400249c3:     	je	0x140024a7c <.text+0x23a7c>
1400249c9:     	movq	0x290(%rbp), %rdx
1400249d0:     	testq	%rdx, %rdx
1400249d3:     	je	0x1400249eb <.text+0x239eb>
1400249d5:     	movq	0x298(%rbp), %rcx
1400249dc:     	shlq	$0x2, %rdx
1400249e0:     	movl	$0x4, %r8d
1400249e6:     	callq	0x140613c20 <.text+0x612c20>
1400249eb:     	movb	%dil, 0x7c9(%rbp)
1400249f2:     	andb	$0x1, %sil
1400249f6:     	movb	%sil, 0x7c8(%rbp)
1400249fd:     	andb	$0x1, %bl
140024a00:     	movb	%bl, 0x7c7(%rbp)
140024a06:     	movb	$0x0, 0x7c6(%rbp)
140024a0d:     	movaps	0x120(%rsp), %xmm6
140024a15:     	movaps	0x110(%rsp), %xmm7
140024a1d:     	movaps	0x100(%rsp), %xmm8
140024a26:     	movaps	0xf0(%rsp), %xmm9
140024a2f:     	movaps	0xe0(%rsp), %xmm10
140024a38:     	movaps	0xd0(%rsp), %xmm11
140024a41:     	movdqa	0xc0(%rsp), %xmm12
140024a4b:     	movdqa	0xb0(%rsp), %xmm13
140024a55:     	movaps	0xa0(%rsp), %xmm14
140024a5e:     	movdqa	0x90(%rsp), %xmm15
140024a68:     	addq	$0x138, %rsp            # imm = 0x138
140024a6f:     	popq	%rbx
140024a70:     	popq	%rdi
140024a71:     	popq	%rsi
140024a72:     	popq	%r12
140024a74:     	popq	%r13
140024a76:     	popq	%r14
140024a78:     	popq	%r15
140024a7a:     	popq	%rbp
140024a7b:     	retq
140024a7c:     	movb	%dil, 0x7c9(%rbp)
140024a83:     	andb	$0x1, %sil
140024a87:     	movb	%sil, 0x7c8(%rbp)
140024a8e:     	andb	$0x1, %bl
140024a91:     	movb	%bl, 0x7c7(%rbp)
140024a97:     	movb	$0x0, 0x7c6(%rbp)
140024a9e:     	movaps	0x120(%rsp), %xmm6
140024aa6:     	movaps	0x110(%rsp), %xmm7
140024aae:     	movaps	0x100(%rsp), %xmm8
140024ab7:     	movaps	0xf0(%rsp), %xmm9
140024ac0:     	movaps	0xe0(%rsp), %xmm10
140024ac9:     	movaps	0xd0(%rsp), %xmm11
140024ad2:     	movdqa	0xc0(%rsp), %xmm12
140024adc:     	movdqa	0xb0(%rsp), %xmm13
140024ae6:     	movaps	0xa0(%rsp), %xmm14
140024aef:     	movdqa	0x90(%rsp), %xmm15
140024af9:     	addq	$0x138, %rsp            # imm = 0x138
140024b00:     	popq	%rbx
140024b01:     	popq	%rdi
140024b02:     	popq	%rsi
140024b03:     	popq	%r12
140024b05:     	popq	%r13
140024b07:     	popq	%r14
140024b09:     	popq	%r15
140024b0b:     	popq	%rbp
140024b0c:     	retq
140024b0d:     	nopl	(%rax)
140024b10:     	movq	%rdx, 0x10(%rsp)
140024b15:     	pushq	%rbp
140024b16:     	pushq	%r15
140024b18:     	pushq	%r14
140024b1a:     	pushq	%r13
140024b1c:     	pushq	%r12
140024b1e:     	pushq	%rsi
140024b1f:     	pushq	%rdi
140024b20:     	pushq	%rbx
140024b21:     	subq	$0x138, %rsp            # imm = 0x138
140024b28:     	leaq	0x80(%rdx), %rbp
140024b2f:     	movdqa	%xmm15, 0x90(%rsp)
140024b39:     	movaps	%xmm14, 0xa0(%rsp)
140024b42:     	movdqa	%xmm13, 0xb0(%rsp)
140024b4c:     	movdqa	%xmm12, 0xc0(%rsp)
140024b56:     	movaps	%xmm11, 0xd0(%rsp)
140024b5f:     	movaps	%xmm10, 0xe0(%rsp)
140024b68:     	movaps	%xmm9, 0xf0(%rsp)
140024b71:     	movaps	%xmm8, 0x100(%rsp)
140024b7a:     	movaps	%xmm7, 0x110(%rsp)
140024b82:     	movaps	%xmm6, 0x120(%rsp)
140024b8a:     	leaq	0x3e0(%rbp), %rcx
140024b91:     	callq	0x14014ba20 <.text+0x14aa20>
140024b96:     	movb	$0x1, 0x813(%rbp)
140024b9d:     	movaps	0x120(%rsp), %xmm6
140024ba5:     	movaps	0x110(%rsp), %xmm7
140024bad:     	movaps	0x100(%rsp), %xmm8
140024bb6:     	movaps	0xf0(%rsp), %xmm9
140024bbf:     	movaps	0xe0(%rsp), %xmm10
140024bc8:     	movaps	0xd0(%rsp), %xmm11
140024bd1:     	movdqa	0xc0(%rsp), %xmm12
140024bdb:     	movdqa	0xb0(%rsp), %xmm13
140024be5:     	movaps	0xa0(%rsp), %xmm14
140024bee:     	movdqa	0x90(%rsp), %xmm15
140024bf8:     	addq	$0x138, %rsp            # imm = 0x138
140024bff:     	popq	%rbx
140024c00:     	popq	%rdi
140024c01:     	popq	%rsi
140024c02:     	popq	%r12
140024c04:     	popq	%r13
140024c06:     	popq	%r14
140024c08:     	popq	%r15
140024c0a:     	popq	%rbp
140024c0b:     	retq
140024c0c:     	nopl	(%rax)
140024c10:     	movq	%rdx, 0x10(%rsp)
140024c15:     	pushq	%rbp
140024c16:     	pushq	%r15
140024c18:     	pushq	%r14
140024c1a:     	pushq	%r13
140024c1c:     	pushq	%r12
140024c1e:     	pushq	%rsi
140024c1f:     	pushq	%rdi
140024c20:     	pushq	%rbx
140024c21:     	subq	$0x138, %rsp            # imm = 0x138
140024c28:     	leaq	0x80(%rdx), %rbp
140024c2f:     	movdqa	%xmm15, 0x90(%rsp)
140024c39:     	movaps	%xmm14, 0xa0(%rsp)
140024c42:     	movdqa	%xmm13, 0xb0(%rsp)
140024c4c:     	movdqa	%xmm12, 0xc0(%rsp)
140024c56:     	movaps	%xmm11, 0xd0(%rsp)
140024c5f:     	movaps	%xmm10, 0xe0(%rsp)
140024c68:     	movaps	%xmm9, 0xf0(%rsp)
140024c71:     	movaps	%xmm8, 0x100(%rsp)
140024c7a:     	movaps	%xmm7, 0x110(%rsp)
140024c82:     	movaps	%xmm6, 0x120(%rsp)
140024c8a:     	movq	0x588(%rbp), %rdx
140024c91:     	testq	%rdx, %rdx
140024c94:     	je	0x140024cac <.text+0x23cac>
140024c96:     	shlq	$0x3, %rdx
140024c9a:     	movl	$0x4, %r8d
140024ca0:     	movq	0x2e8(%rbp), %rcx
140024ca7:     	callq	0x140613c20 <.text+0x612c20>
140024cac:     	movaps	0x120(%rsp), %xmm6
140024cb4:     	movaps	0x110(%rsp), %xmm7
140024cbc:     	movaps	0x100(%rsp), %xmm8
140024cc5:     	movaps	0xf0(%rsp), %xmm9
140024cce:     	movaps	0xe0(%rsp), %xmm10
140024cd7:     	movaps	0xd0(%rsp), %xmm11
140024ce0:     	movdqa	0xc0(%rsp), %xmm12
140024cea:     	movdqa	0xb0(%rsp), %xmm13
140024cf4:     	movaps	0xa0(%rsp), %xmm14
140024cfd:     	movdqa	0x90(%rsp), %xmm15
140024d07:     	addq	$0x138, %rsp            # imm = 0x138
140024d0e:     	popq	%rbx
140024d0f:     	popq	%rdi
140024d10:     	popq	%rsi
140024d11:     	popq	%r12
140024d13:     	popq	%r13
140024d15:     	popq	%r14
140024d17:     	popq	%r15
140024d19:     	popq	%rbp
140024d1a:     	retq
140024d1b:     	nopl	(%rax,%rax)
140024d20:     	movq	%rdx, 0x10(%rsp)
140024d25:     	pushq	%rbp
140024d26:     	pushq	%r15
140024d28:     	pushq	%r14
140024d2a:     	pushq	%r13
140024d2c:     	pushq	%r12
140024d2e:     	pushq	%rsi
140024d2f:     	pushq	%rdi
140024d30:     	pushq	%rbx
140024d31:     	subq	$0x138, %rsp            # imm = 0x138
140024d38:     	leaq	0x80(%rdx), %rbp
140024d3f:     	movdqa	%xmm15, 0x90(%rsp)
140024d49:     	movaps	%xmm14, 0xa0(%rsp)
140024d52:     	movdqa	%xmm13, 0xb0(%rsp)
140024d5c:     	movdqa	%xmm12, 0xc0(%rsp)
140024d66:     	movaps	%xmm11, 0xd0(%rsp)
140024d6f:     	movaps	%xmm10, 0xe0(%rsp)
140024d78:     	movaps	%xmm9, 0xf0(%rsp)
140024d81:     	movaps	%xmm8, 0x100(%rsp)
140024d8a:     	movaps	%xmm7, 0x110(%rsp)
140024d92:     	movaps	%xmm6, 0x120(%rsp)
140024d9a:     	movzbl	0x7c6(%rbp), %ebx
140024da1:     	movzbl	0x7c7(%rbp), %esi
140024da8:     	movzbl	0x7c8(%rbp), %edi
140024daf:     	testb	$0x1, 0x7c9(%rbp)
140024db6:     	je	0x140024e73 <.text+0x23e73>
140024dbc:     	movq	0x680(%rbp), %rdx
140024dc3:     	testq	%rdx, %rdx
140024dc6:     	je	0x140024dde <.text+0x23dde>
140024dc8:     	movq	0x688(%rbp), %rcx
140024dcf:     	shlq	$0x3, %rdx
140024dd3:     	movl	$0x4, %r8d
140024dd9:     	callq	0x140613c20 <.text+0x612c20>
140024dde:     	andb	$0x1, %dil
140024de2:     	movb	%dil, 0x7cd(%rbp)
140024de9:     	andb	$0x1, %sil
140024ded:     	movb	%sil, 0x7cc(%rbp)
140024df4:     	andb	$0x1, %bl
140024df7:     	movb	%bl, 0x7cb(%rbp)
140024dfd:     	movb	$0x0, 0x7ca(%rbp)
140024e04:     	movaps	0x120(%rsp), %xmm6
140024e0c:     	movaps	0x110(%rsp), %xmm7
140024e14:     	movaps	0x100(%rsp), %xmm8
140024e1d:     	movaps	0xf0(%rsp), %xmm9
140024e26:     	movaps	0xe0(%rsp), %xmm10
140024e2f:     	movaps	0xd0(%rsp), %xmm11
140024e38:     	movdqa	0xc0(%rsp), %xmm12
140024e42:     	movdqa	0xb0(%rsp), %xmm13
140024e4c:     	movaps	0xa0(%rsp), %xmm14
140024e55:     	movdqa	0x90(%rsp), %xmm15
140024e5f:     	addq	$0x138, %rsp            # imm = 0x138
140024e66:     	popq	%rbx
140024e67:     	popq	%rdi
140024e68:     	popq	%rsi
140024e69:     	popq	%r12
140024e6b:     	popq	%r13
140024e6d:     	popq	%r14
140024e6f:     	popq	%r15
140024e71:     	popq	%rbp
140024e72:     	retq
140024e73:     	andb	$0x1, %dil
140024e77:     	movb	%dil, 0x7cd(%rbp)
140024e7e:     	andb	$0x1, %sil
140024e82:     	movb	%sil, 0x7cc(%rbp)
140024e89:     	andb	$0x1, %bl
140024e8c:     	movb	%bl, 0x7cb(%rbp)
140024e92:     	movb	$0x0, 0x7ca(%rbp)
140024e99:     	movaps	0x120(%rsp), %xmm6
140024ea1:     	movaps	0x110(%rsp), %xmm7
140024ea9:     	movaps	0x100(%rsp), %xmm8
140024eb2:     	movaps	0xf0(%rsp), %xmm9
140024ebb:     	movaps	0xe0(%rsp), %xmm10
140024ec4:     	movaps	0xd0(%rsp), %xmm11
140024ecd:     	movdqa	0xc0(%rsp), %xmm12
140024ed7:     	movdqa	0xb0(%rsp), %xmm13
140024ee1:     	movaps	0xa0(%rsp), %xmm14
140024eea:     	movdqa	0x90(%rsp), %xmm15
140024ef4:     	addq	$0x138, %rsp            # imm = 0x138
140024efb:     	popq	%rbx
140024efc:     	popq	%rdi
140024efd:     	popq	%rsi
140024efe:     	popq	%r12
140024f00:     	popq	%r13
140024f02:     	popq	%r14
140024f04:     	popq	%r15
140024f06:     	popq	%rbp
140024f07:     	retq
140024f08:     	nopl	(%rax,%rax)
140024f10:     	movq	%rdx, 0x10(%rsp)
140024f15:     	pushq	%rbp
140024f16:     	pushq	%r15
140024f18:     	pushq	%r14
140024f1a:     	pushq	%r13
140024f1c:     	pushq	%r12
140024f1e:     	pushq	%rsi
140024f1f:     	pushq	%rdi
140024f20:     	pushq	%rbx
140024f21:     	subq	$0x138, %rsp            # imm = 0x138
140024f28:     	leaq	0x80(%rdx), %rbp
140024f2f:     	movdqa	%xmm15, 0x90(%rsp)
140024f39:     	movaps	%xmm14, 0xa0(%rsp)
140024f42:     	movdqa	%xmm13, 0xb0(%rsp)
140024f4c:     	movdqa	%xmm12, 0xc0(%rsp)
140024f56:     	movaps	%xmm11, 0xd0(%rsp)
140024f5f:     	movaps	%xmm10, 0xe0(%rsp)
140024f68:     	movaps	%xmm9, 0xf0(%rsp)
140024f71:     	movaps	%xmm8, 0x100(%rsp)
140024f7a:     	movaps	%xmm7, 0x110(%rsp)
140024f82:     	movaps	%xmm6, 0x120(%rsp)
140024f8a:     	movzbl	0x7ca(%rbp), %ebx
140024f91:     	movzbl	0x7cb(%rbp), %esi
140024f98:     	movzbl	0x7cc(%rbp), %edi
140024f9f:     	cmpb	$0x0, 0x7cd(%rbp)
140024fa6:     	je	0x140024fce <.text+0x23fce>
140024fa8:     	movq	0x650(%rbp), %rax
140024faf:     	testq	%rax, %rax
140024fb2:     	je	0x140024fce <.text+0x23fce>
140024fb4:     	movq	0x658(%rbp), %rcx
140024fbb:     	shlq	$0x2, %rax
140024fbf:     	leaq	(%rax,%rax,2), %rdx
140024fc3:     	movl	$0x4, %r8d
140024fc9:     	callq	0x140613c20 <.text+0x612c20>
140024fce:     	movq	0x648(%rbp), %rdx
140024fd5:     	testq	%rdx, %rdx
140024fd8:     	je	0x140024ff0 <.text+0x23ff0>
140024fda:     	shlq	$0x2, %rdx
140024fde:     	movl	$0x4, %r8d
140024fe4:     	movq	0x6a8(%rbp), %rcx
140024feb:     	callq	0x140613c20 <.text+0x612c20>
140024ff0:     	movq	0x560(%rbp), %rdx
140024ff7:     	testq	%rdx, %rdx
140024ffa:     	setne	%al
140024ffd:     	testb	%al, %bl
140024fff:     	je	0x140025017 <.text+0x24017>
140025001:     	shlq	$0x3, %rdx
140025005:     	movl	$0x4, %r8d
14002500b:     	movq	0x6a0(%rbp), %rcx
140025012:     	callq	0x140613c20 <.text+0x612c20>
140025017:     	movq	0x7e0(%rbp), %rdx
14002501e:     	testq	%rdx, %rdx
140025021:     	setne	%al
140025024:     	testb	%al, %sil
140025027:     	je	0x14002503f <.text+0x2403f>
140025029:     	shlq	$0x2, %rdx
14002502d:     	movl	$0x4, %r8d
140025033:     	movq	0x780(%rbp), %rcx
14002503a:     	callq	0x140613c20 <.text+0x612c20>
14002503f:     	testb	$0x1, %dil
140025043:     	je	0x140025103 <.text+0x24103>
140025049:     	movq	0x538(%rbp), %rdx
140025050:     	testq	%rdx, %rdx
140025053:     	je	0x14002506b <.text+0x2406b>
140025055:     	shlq	$0x2, %rdx
140025059:     	movl	$0x4, %r8d
14002505f:     	movq	0x580(%rbp), %rcx
140025066:     	callq	0x140613c20 <.text+0x612c20>
14002506b:     	movb	$0x1, 0x803(%rbp)
140025072:     	andb	$0x1, %sil
140025076:     	movb	%sil, 0x802(%rbp)
14002507d:     	andb	$0x1, %bl
140025080:     	movb	%bl, 0x801(%rbp)
140025086:     	movb	$0x0, 0x800(%rbp)
14002508d:     	movb	$0x1, 0x7ff(%rbp)
140025094:     	movaps	0x120(%rsp), %xmm6
14002509c:     	movaps	0x110(%rsp), %xmm7
1400250a4:     	movaps	0x100(%rsp), %xmm8
1400250ad:     	movaps	0xf0(%rsp), %xmm9
1400250b6:     	movaps	0xe0(%rsp), %xmm10
1400250bf:     	movaps	0xd0(%rsp), %xmm11
1400250c8:     	movdqa	0xc0(%rsp), %xmm12
1400250d2:     	movdqa	0xb0(%rsp), %xmm13
1400250dc:     	movaps	0xa0(%rsp), %xmm14
1400250e5:     	movdqa	0x90(%rsp), %xmm15
1400250ef:     	addq	$0x138, %rsp            # imm = 0x138
1400250f6:     	popq	%rbx
1400250f7:     	popq	%rdi
1400250f8:     	popq	%rsi
1400250f9:     	popq	%r12
1400250fb:     	popq	%r13
1400250fd:     	popq	%r14
1400250ff:     	popq	%r15
140025101:     	popq	%rbp
140025102:     	retq
140025103:     	movb	$0x0, 0x803(%rbp)
14002510a:     	andb	$0x1, %sil
14002510e:     	movb	%sil, 0x802(%rbp)
140025115:     	andb	$0x1, %bl
140025118:     	movb	%bl, 0x801(%rbp)
14002511e:     	movb	$0x0, 0x800(%rbp)
140025125:     	movb	$0x1, 0x7ff(%rbp)
14002512c:     	movaps	0x120(%rsp), %xmm6
140025134:     	movaps	0x110(%rsp), %xmm7
14002513c:     	movaps	0x100(%rsp), %xmm8
140025145:     	movaps	0xf0(%rsp), %xmm9
14002514e:     	movaps	0xe0(%rsp), %xmm10
140025157:     	movaps	0xd0(%rsp), %xmm11
140025160:     	movdqa	0xc0(%rsp), %xmm12
14002516a:     	movdqa	0xb0(%rsp), %xmm13
140025174:     	movaps	0xa0(%rsp), %xmm14
14002517d:     	movdqa	0x90(%rsp), %xmm15
140025187:     	addq	$0x138, %rsp            # imm = 0x138
14002518e:     	popq	%rbx
14002518f:     	popq	%rdi
140025190:     	popq	%rsi
140025191:     	popq	%r12
140025193:     	popq	%r13
140025195:     	popq	%r14
140025197:     	popq	%r15
140025199:     	popq	%rbp
14002519a:     	retq
14002519b:     	nopl	(%rax,%rax)
1400251a0:     	movq	%rdx, 0x10(%rsp)
1400251a5:     	pushq	%rbp
1400251a6:     	pushq	%r15
1400251a8:     	pushq	%r14
1400251aa:     	pushq	%r13
1400251ac:     	pushq	%r12
1400251ae:     	pushq	%rsi
1400251af:     	pushq	%rdi
1400251b0:     	pushq	%rbx
1400251b1:     	subq	$0x138, %rsp            # imm = 0x138
1400251b8:     	leaq	0x80(%rdx), %rbp
1400251bf:     	movdqa	%xmm15, 0x90(%rsp)
1400251c9:     	movaps	%xmm14, 0xa0(%rsp)
1400251d2:     	movdqa	%xmm13, 0xb0(%rsp)
1400251dc:     	movdqa	%xmm12, 0xc0(%rsp)
1400251e6:     	movaps	%xmm11, 0xd0(%rsp)
1400251ef:     	movaps	%xmm10, 0xe0(%rsp)
1400251f8:     	movaps	%xmm9, 0xf0(%rsp)
140025201:     	movaps	%xmm8, 0x100(%rsp)
14002520a:     	movaps	%xmm7, 0x110(%rsp)
140025212:     	movaps	%xmm6, 0x120(%rsp)
14002521a:     	leaq	0x380(%rbp), %rcx
140025221:     	callq	0x14014ba20 <.text+0x14aa20>
140025226:     	movaps	0x120(%rsp), %xmm6
14002522e:     	movaps	0x110(%rsp), %xmm7
140025236:     	movaps	0x100(%rsp), %xmm8
14002523f:     	movaps	0xf0(%rsp), %xmm9
140025248:     	movaps	0xe0(%rsp), %xmm10
140025251:     	movaps	0xd0(%rsp), %xmm11
14002525a:     	movdqa	0xc0(%rsp), %xmm12
140025264:     	movdqa	0xb0(%rsp), %xmm13
14002526e:     	movaps	0xa0(%rsp), %xmm14
140025277:     	movdqa	0x90(%rsp), %xmm15
140025281:     	addq	$0x138, %rsp            # imm = 0x138
140025288:     	popq	%rbx
140025289:     	popq	%rdi
14002528a:     	popq	%rsi
14002528b:     	popq	%r12
14002528d:     	popq	%r13
14002528f:     	popq	%r14
140025291:     	popq	%r15
140025293:     	popq	%rbp
140025294:     	retq
140025295:     	nopw	%cs:(%rax,%rax)
1400252a0:     	movq	%rdx, 0x10(%rsp)
1400252a5:     	pushq	%rbp
1400252a6:     	pushq	%r15
1400252a8:     	pushq	%r14
1400252aa:     	pushq	%r13
1400252ac:     	pushq	%r12
1400252ae:     	pushq	%rsi
1400252af:     	pushq	%rdi
1400252b0:     	pushq	%rbx
1400252b1:     	subq	$0x138, %rsp            # imm = 0x138
1400252b8:     	leaq	0x80(%rdx), %rbp
1400252bf:     	movdqa	%xmm15, 0x90(%rsp)
1400252c9:     	movaps	%xmm14, 0xa0(%rsp)
1400252d2:     	movdqa	%xmm13, 0xb0(%rsp)
1400252dc:     	movdqa	%xmm12, 0xc0(%rsp)
1400252e6:     	movaps	%xmm11, 0xd0(%rsp)
1400252ef:     	movaps	%xmm10, 0xe0(%rsp)
1400252f8:     	movaps	%xmm9, 0xf0(%rsp)
140025301:     	movaps	%xmm8, 0x100(%rsp)
14002530a:     	movaps	%xmm7, 0x110(%rsp)
140025312:     	movaps	%xmm6, 0x120(%rsp)
14002531a:     	movq	0x590(%rbp), %rdx
140025321:     	testq	%rdx, %rdx
140025324:     	je	0x14002533c <.text+0x2433c>
140025326:     	movq	0x598(%rbp), %rcx
14002532d:     	shlq	$0x2, %rdx
140025331:     	movl	$0x4, %r8d
140025337:     	callq	0x140613c20 <.text+0x612c20>
14002533c:     	movaps	0x120(%rsp), %xmm6
140025344:     	movaps	0x110(%rsp), %xmm7
14002534c:     	movaps	0x100(%rsp), %xmm8
140025355:     	movaps	0xf0(%rsp), %xmm9
14002535e:     	movaps	0xe0(%rsp), %xmm10
140025367:     	movaps	0xd0(%rsp), %xmm11
140025370:     	movdqa	0xc0(%rsp), %xmm12
14002537a:     	movdqa	0xb0(%rsp), %xmm13
140025384:     	movaps	0xa0(%rsp), %xmm14
14002538d:     	movdqa	0x90(%rsp), %xmm15
140025397:     	addq	$0x138, %rsp            # imm = 0x138
14002539e:     	popq	%rbx
14002539f:     	popq	%rdi
1400253a0:     	popq	%rsi
1400253a1:     	popq	%r12
1400253a3:     	popq	%r13
1400253a5:     	popq	%r14
1400253a7:     	popq	%r15
1400253a9:     	popq	%rbp
1400253aa:     	retq
1400253ab:     	nopl	(%rax,%rax)
1400253b0:     	movq	%rdx, 0x10(%rsp)
1400253b5:     	pushq	%rbp
1400253b6:     	pushq	%r15
1400253b8:     	pushq	%r14
1400253ba:     	pushq	%r13
1400253bc:     	pushq	%r12
1400253be:     	pushq	%rsi
1400253bf:     	pushq	%rdi
1400253c0:     	pushq	%rbx
1400253c1:     	subq	$0x138, %rsp            # imm = 0x138
1400253c8:     	leaq	0x80(%rdx), %rbp
1400253cf:     	movdqa	%xmm15, 0x90(%rsp)
1400253d9:     	movaps	%xmm14, 0xa0(%rsp)
1400253e2:     	movdqa	%xmm13, 0xb0(%rsp)
1400253ec:     	movdqa	%xmm12, 0xc0(%rsp)
1400253f6:     	movaps	%xmm11, 0xd0(%rsp)
1400253ff:     	movaps	%xmm10, 0xe0(%rsp)
140025408:     	movaps	%xmm9, 0xf0(%rsp)
140025411:     	movaps	%xmm8, 0x100(%rsp)
14002541a:     	movaps	%xmm7, 0x110(%rsp)
140025422:     	movaps	%xmm6, 0x120(%rsp)
14002542a:     	movq	0x508(%rbp), %rax
140025431:     	testq	%rax, %rax
140025434:     	je	0x140025450 <.text+0x24450>
140025436:     	movq	0x510(%rbp), %rcx
14002543d:     	shlq	$0x2, %rax
140025441:     	leaq	(%rax,%rax,2), %rdx
140025445:     	movl	$0x4, %r8d
14002544b:     	callq	0x140613c20 <.text+0x612c20>
140025450:     	movaps	0x120(%rsp), %xmm6
140025458:     	movaps	0x110(%rsp), %xmm7
140025460:     	movaps	0x100(%rsp), %xmm8
140025469:     	movaps	0xf0(%rsp), %xmm9
140025472:     	movaps	0xe0(%rsp), %xmm10
14002547b:     	movaps	0xd0(%rsp), %xmm11
140025484:     	movdqa	0xc0(%rsp), %xmm12
14002548e:     	movdqa	0xb0(%rsp), %xmm13
140025498:     	movaps	0xa0(%rsp), %xmm14
1400254a1:     	movdqa	0x90(%rsp), %xmm15
1400254ab:     	addq	$0x138, %rsp            # imm = 0x138
1400254b2:     	popq	%rbx
1400254b3:     	popq	%rdi
1400254b4:     	popq	%rsi
1400254b5:     	popq	%r12
1400254b7:     	popq	%r13
1400254b9:     	popq	%r14
1400254bb:     	popq	%r15
1400254bd:     	popq	%rbp
1400254be:     	retq
1400254bf:     	nop
1400254c0:     	movq	%rdx, 0x10(%rsp)
1400254c5:     	pushq	%rbp
1400254c6:     	pushq	%r15
1400254c8:     	pushq	%r14
1400254ca:     	pushq	%r13
1400254cc:     	pushq	%r12
1400254ce:     	pushq	%rsi
1400254cf:     	pushq	%rdi
1400254d0:     	pushq	%rbx
1400254d1:     	subq	$0x138, %rsp            # imm = 0x138
1400254d8:     	leaq	0x80(%rdx), %rbp
1400254df:     	movdqa	%xmm15, 0x90(%rsp)
1400254e9:     	movaps	%xmm14, 0xa0(%rsp)
1400254f2:     	movdqa	%xmm13, 0xb0(%rsp)
1400254fc:     	movdqa	%xmm12, 0xc0(%rsp)
140025506:     	movaps	%xmm11, 0xd0(%rsp)
14002550f:     	movaps	%xmm10, 0xe0(%rsp)
140025518:     	movaps	%xmm9, 0xf0(%rsp)
140025521:     	movaps	%xmm8, 0x100(%rsp)
14002552a:     	movaps	%xmm7, 0x110(%rsp)
140025532:     	movaps	%xmm6, 0x120(%rsp)
14002553a:     	movq	0x680(%rbp), %rdx
140025541:     	testq	%rdx, %rdx
140025544:     	je	0x14002555c <.text+0x2455c>
140025546:     	movq	0x688(%rbp), %rcx
14002554d:     	shlq	$0x3, %rdx
140025551:     	movl	$0x4, %r8d
140025557:     	callq	0x140613c20 <.text+0x612c20>
14002555c:     	movaps	0x120(%rsp), %xmm6
140025564:     	movaps	0x110(%rsp), %xmm7
14002556c:     	movaps	0x100(%rsp), %xmm8
140025575:     	movaps	0xf0(%rsp), %xmm9
14002557e:     	movaps	0xe0(%rsp), %xmm10
140025587:     	movaps	0xd0(%rsp), %xmm11
140025590:     	movdqa	0xc0(%rsp), %xmm12
14002559a:     	movdqa	0xb0(%rsp), %xmm13
1400255a4:     	movaps	0xa0(%rsp), %xmm14
1400255ad:     	movdqa	0x90(%rsp), %xmm15
1400255b7:     	addq	$0x138, %rsp            # imm = 0x138
1400255be:     	popq	%rbx
1400255bf:     	popq	%rdi
1400255c0:     	popq	%rsi
1400255c1:     	popq	%r12
1400255c3:     	popq	%r13
1400255c5:     	popq	%r14
1400255c7:     	popq	%r15
1400255c9:     	popq	%rbp
1400255ca:     	retq
1400255cb:     	nopl	(%rax,%rax)
1400255d0:     	movq	%rdx, 0x10(%rsp)
1400255d5:     	pushq	%rbp
1400255d6:     	pushq	%r15
1400255d8:     	pushq	%r14
1400255da:     	pushq	%r13
1400255dc:     	pushq	%r12
1400255de:     	pushq	%rsi
1400255df:     	pushq	%rdi
1400255e0:     	pushq	%rbx
1400255e1:     	subq	$0x138, %rsp            # imm = 0x138
1400255e8:     	leaq	0x80(%rdx), %rbp
1400255ef:     	movdqa	%xmm15, 0x90(%rsp)
1400255f9:     	movaps	%xmm14, 0xa0(%rsp)
140025602:     	movdqa	%xmm13, 0xb0(%rsp)
14002560c:     	movdqa	%xmm12, 0xc0(%rsp)
140025616:     	movaps	%xmm11, 0xd0(%rsp)
14002561f:     	movaps	%xmm10, 0xe0(%rsp)
140025628:     	movaps	%xmm9, 0xf0(%rsp)
140025631:     	movaps	%xmm8, 0x100(%rsp)
14002563a:     	movaps	%xmm7, 0x110(%rsp)
140025642:     	movaps	%xmm6, 0x120(%rsp)
14002564a:     	movq	0x650(%rbp), %rdx
140025651:     	testq	%rdx, %rdx
140025654:     	je	0x14002566c <.text+0x2466c>
140025656:     	movq	0x658(%rbp), %rcx
14002565d:     	shlq	$0x2, %rdx
140025661:     	movl	$0x4, %r8d
140025667:     	callq	0x140613c20 <.text+0x612c20>
14002566c:     	movaps	0x120(%rsp), %xmm6
140025674:     	movaps	0x110(%rsp), %xmm7
14002567c:     	movaps	0x100(%rsp), %xmm8
140025685:     	movaps	0xf0(%rsp), %xmm9
14002568e:     	movaps	0xe0(%rsp), %xmm10
140025697:     	movaps	0xd0(%rsp), %xmm11
1400256a0:     	movdqa	0xc0(%rsp), %xmm12
1400256aa:     	movdqa	0xb0(%rsp), %xmm13
1400256b4:     	movaps	0xa0(%rsp), %xmm14
1400256bd:     	movdqa	0x90(%rsp), %xmm15
1400256c7:     	addq	$0x138, %rsp            # imm = 0x138
1400256ce:     	popq	%rbx
1400256cf:     	popq	%rdi
1400256d0:     	popq	%rsi
1400256d1:     	popq	%r12
1400256d3:     	popq	%r13
1400256d5:     	popq	%r14
1400256d7:     	popq	%r15
1400256d9:     	popq	%rbp
1400256da:     	retq
1400256db:     	nopl	(%rax,%rax)
1400256e0:     	movq	%rdx, 0x10(%rsp)
1400256e5:     	pushq	%rbp
1400256e6:     	pushq	%r15
1400256e8:     	pushq	%r14
1400256ea:     	pushq	%r13
1400256ec:     	pushq	%r12
1400256ee:     	pushq	%rsi
1400256ef:     	pushq	%rdi
1400256f0:     	pushq	%rbx
1400256f1:     	subq	$0x138, %rsp            # imm = 0x138
1400256f8:     	leaq	0x80(%rdx), %rbp
1400256ff:     	movdqa	%xmm15, 0x90(%rsp)
140025709:     	movaps	%xmm14, 0xa0(%rsp)
140025712:     	movdqa	%xmm13, 0xb0(%rsp)
14002571c:     	movdqa	%xmm12, 0xc0(%rsp)
140025726:     	movaps	%xmm11, 0xd0(%rsp)
14002572f:     	movaps	%xmm10, 0xe0(%rsp)
140025738:     	movaps	%xmm9, 0xf0(%rsp)
140025741:     	movaps	%xmm8, 0x100(%rsp)
14002574a:     	movaps	%xmm7, 0x110(%rsp)
140025752:     	movaps	%xmm6, 0x120(%rsp)
14002575a:     	leaq	0x460(%rbp), %rcx
140025761:     	callq	0x14014ba20 <.text+0x14aa20>
140025766:     	movaps	0x120(%rsp), %xmm6
14002576e:     	movaps	0x110(%rsp), %xmm7
140025776:     	movaps	0x100(%rsp), %xmm8
14002577f:     	movaps	0xf0(%rsp), %xmm9
140025788:     	movaps	0xe0(%rsp), %xmm10
140025791:     	movaps	0xd0(%rsp), %xmm11
14002579a:     	movdqa	0xc0(%rsp), %xmm12
1400257a4:     	movdqa	0xb0(%rsp), %xmm13
1400257ae:     	movaps	0xa0(%rsp), %xmm14
1400257b7:     	movdqa	0x90(%rsp), %xmm15
1400257c1:     	addq	$0x138, %rsp            # imm = 0x138
1400257c8:     	popq	%rbx
1400257c9:     	popq	%rdi
1400257ca:     	popq	%rsi
1400257cb:     	popq	%r12
1400257cd:     	popq	%r13
1400257cf:     	popq	%r14
1400257d1:     	popq	%r15
1400257d3:     	popq	%rbp
1400257d4:     	retq
1400257d5:     	nopw	%cs:(%rax,%rax)
1400257e0:     	movq	%rdx, 0x10(%rsp)
1400257e5:     	pushq	%rbp
1400257e6:     	pushq	%r15
1400257e8:     	pushq	%r14
1400257ea:     	pushq	%r13
1400257ec:     	pushq	%r12
1400257ee:     	pushq	%rsi
1400257ef:     	pushq	%rdi
1400257f0:     	pushq	%rbx
1400257f1:     	subq	$0x138, %rsp            # imm = 0x138
1400257f8:     	leaq	0x80(%rdx), %rbp
1400257ff:     	movdqa	%xmm15, 0x90(%rsp)
140025809:     	movaps	%xmm14, 0xa0(%rsp)
140025812:     	movdqa	%xmm13, 0xb0(%rsp)
14002581c:     	movdqa	%xmm12, 0xc0(%rsp)
140025826:     	movaps	%xmm11, 0xd0(%rsp)
14002582f:     	movaps	%xmm10, 0xe0(%rsp)
140025838:     	movaps	%xmm9, 0xf0(%rsp)
140025841:     	movaps	%xmm8, 0x100(%rsp)
14002584a:     	movaps	%xmm7, 0x110(%rsp)
140025852:     	movaps	%xmm6, 0x120(%rsp)
14002585a:     	movb	$0x1, 0x817(%rbp)
140025861:     	leaq	0x380(%rbp), %rcx
140025868:     	callq	0x14014ba20 <.text+0x14aa20>
14002586d:     	movb	$0x1, 0x817(%rbp)
140025874:     	movaps	0x120(%rsp), %xmm6
14002587c:     	movaps	0x110(%rsp), %xmm7
140025884:     	movaps	0x100(%rsp), %xmm8
14002588d:     	movaps	0xf0(%rsp), %xmm9
140025896:     	movaps	0xe0(%rsp), %xmm10
14002589f:     	movaps	0xd0(%rsp), %xmm11
1400258a8:     	movdqa	0xc0(%rsp), %xmm12
1400258b2:     	movdqa	0xb0(%rsp), %xmm13
1400258bc:     	movaps	0xa0(%rsp), %xmm14
1400258c5:     	movdqa	0x90(%rsp), %xmm15
1400258cf:     	addq	$0x138, %rsp            # imm = 0x138
1400258d6:     	popq	%rbx
1400258d7:     	popq	%rdi
1400258d8:     	popq	%rsi
1400258d9:     	popq	%r12
1400258db:     	popq	%r13
1400258dd:     	popq	%r14
1400258df:     	popq	%r15
1400258e1:     	popq	%rbp
1400258e2:     	retq
1400258e3:     	nopw	%cs:(%rax,%rax)
1400258f0:     	movq	%rdx, 0x10(%rsp)
1400258f5:     	pushq	%rbp
1400258f6:     	pushq	%r15
1400258f8:     	pushq	%r14
1400258fa:     	pushq	%r13
1400258fc:     	pushq	%r12
1400258fe:     	pushq	%rsi
1400258ff:     	pushq	%rdi
140025900:     	pushq	%rbx
140025901:     	subq	$0x138, %rsp            # imm = 0x138
140025908:     	leaq	0x80(%rdx), %rbp
14002590f:     	movdqa	%xmm15, 0x90(%rsp)
140025919:     	movaps	%xmm14, 0xa0(%rsp)
140025922:     	movdqa	%xmm13, 0xb0(%rsp)
14002592c:     	movdqa	%xmm12, 0xc0(%rsp)
140025936:     	movaps	%xmm11, 0xd0(%rsp)
14002593f:     	movaps	%xmm10, 0xe0(%rsp)
140025948:     	movaps	%xmm9, 0xf0(%rsp)
140025951:     	movaps	%xmm8, 0x100(%rsp)
14002595a:     	movaps	%xmm7, 0x110(%rsp)
140025962:     	movaps	%xmm6, 0x120(%rsp)
14002596a:     	movzbl	0x815(%rbp), %eax
140025971:     	movzbl	0x816(%rbp), %ecx
140025978:     	movb	%al, 0x808(%rbp)
14002597e:     	testb	%al, %al
140025980:     	movb	%cl, 0x7a0(%rbp)
140025986:     	je	0x140025a1a <.text+0x24a1a>
14002598c:     	movq	0x7d0(%rbp), %rdx
140025993:     	testq	%rdx, %rdx
140025996:     	je	0x1400259ae <.text+0x249ae>
140025998:     	shlq	$0x2, %rdx
14002599c:     	movl	$0x4, %r8d
1400259a2:     	movq	0x7f0(%rbp), %rcx
1400259a9:     	callq	0x140613c20 <.text+0x612c20>
1400259ae:     	movaps	0x120(%rsp), %xmm6
1400259b6:     	movaps	0x110(%rsp), %xmm7
1400259be:     	movaps	0x100(%rsp), %xmm8
1400259c7:     	movaps	0xf0(%rsp), %xmm9
1400259d0:     	movaps	0xe0(%rsp), %xmm10
1400259d9:     	movaps	0xd0(%rsp), %xmm11
1400259e2:     	movaps	0xc0(%rsp), %xmm12
1400259eb:     	movaps	0xb0(%rsp), %xmm13
1400259f4:     	movaps	0xa0(%rsp), %xmm14
1400259fd:     	movaps	0x90(%rsp), %xmm15
140025a06:     	addq	$0x138, %rsp            # imm = 0x138
140025a0d:     	popq	%rbx
140025a0e:     	popq	%rdi
140025a0f:     	popq	%rsi
140025a10:     	popq	%r12
140025a12:     	popq	%r13
140025a14:     	popq	%r14
140025a16:     	popq	%r15
140025a18:     	popq	%rbp
140025a19:     	retq
140025a1a:     	movaps	0x120(%rsp), %xmm6
140025a22:     	movaps	0x110(%rsp), %xmm7
140025a2a:     	movaps	0x100(%rsp), %xmm8
140025a33:     	movaps	0xf0(%rsp), %xmm9
140025a3c:     	movaps	0xe0(%rsp), %xmm10
140025a45:     	movaps	0xd0(%rsp), %xmm11
140025a4e:     	movaps	0xc0(%rsp), %xmm12
140025a57:     	movaps	0xb0(%rsp), %xmm13
140025a60:     	movaps	0xa0(%rsp), %xmm14
140025a69:     	movaps	0x90(%rsp), %xmm15
140025a72:     	addq	$0x138, %rsp            # imm = 0x138
140025a79:     	popq	%rbx
140025a7a:     	popq	%rdi
140025a7b:     	popq	%rsi
140025a7c:     	popq	%r12
140025a7e:     	popq	%r13
140025a80:     	popq	%r14
140025a82:     	popq	%r15
140025a84:     	popq	%rbp
140025a85:     	retq
140025a86:     	nopw	%cs:(%rax,%rax)
140025a90:     	movq	%rdx, 0x10(%rsp)
140025a95:     	pushq	%rbp
140025a96:     	pushq	%r15
140025a98:     	pushq	%r14
140025a9a:     	pushq	%r13
140025a9c:     	pushq	%r12
140025a9e:     	pushq	%rsi
140025a9f:     	pushq	%rdi
140025aa0:     	pushq	%rbx
140025aa1:     	subq	$0x138, %rsp            # imm = 0x138
140025aa8:     	leaq	0x80(%rdx), %rbp
140025aaf:     	movaps	%xmm15, 0x90(%rsp)
140025ab8:     	movaps	%xmm14, 0xa0(%rsp)
140025ac1:     	movaps	%xmm13, 0xb0(%rsp)
140025aca:     	movaps	%xmm12, 0xc0(%rsp)
140025ad3:     	movaps	%xmm11, 0xd0(%rsp)
140025adc:     	movaps	%xmm10, 0xe0(%rsp)
140025ae5:     	movaps	%xmm9, 0xf0(%rsp)
140025aee:     	movaps	%xmm8, 0x100(%rsp)
140025af7:     	movaps	%xmm7, 0x110(%rsp)
140025aff:     	movaps	%xmm6, 0x120(%rsp)
140025b07:     	testb	$0x1, 0x7a0(%rbp)
140025b0e:     	je	0x140025ba6 <.text+0x24ba6>
140025b14:     	movq	0x680(%rbp), %rax
140025b1b:     	testq	%rax, %rax
140025b1e:     	je	0x140025b3a <.text+0x24b3a>
140025b20:     	movq	0x688(%rbp), %rcx
140025b27:     	shlq	$0x2, %rax
140025b2b:     	leaq	(%rax,%rax,2), %rdx
140025b2f:     	movl	$0x4, %r8d
140025b35:     	callq	0x140613c20 <.text+0x612c20>
140025b3a:     	movaps	0x120(%rsp), %xmm6
140025b42:     	movaps	0x110(%rsp), %xmm7
140025b4a:     	movaps	0x100(%rsp), %xmm8
140025b53:     	movaps	0xf0(%rsp), %xmm9
140025b5c:     	movaps	0xe0(%rsp), %xmm10
140025b65:     	movaps	0xd0(%rsp), %xmm11
140025b6e:     	movaps	0xc0(%rsp), %xmm12
140025b77:     	movaps	0xb0(%rsp), %xmm13
140025b80:     	movaps	0xa0(%rsp), %xmm14
140025b89:     	movaps	0x90(%rsp), %xmm15
140025b92:     	addq	$0x138, %rsp            # imm = 0x138
140025b99:     	popq	%rbx
140025b9a:     	popq	%rdi
140025b9b:     	popq	%rsi
140025b9c:     	popq	%r12
140025b9e:     	popq	%r13
140025ba0:     	popq	%r14
140025ba2:     	popq	%r15
140025ba4:     	popq	%rbp
140025ba5:     	retq
140025ba6:     	movaps	0x120(%rsp), %xmm6
140025bae:     	movaps	0x110(%rsp), %xmm7
140025bb6:     	movaps	0x100(%rsp), %xmm8
140025bbf:     	movaps	0xf0(%rsp), %xmm9
140025bc8:     	movaps	0xe0(%rsp), %xmm10
140025bd1:     	movaps	0xd0(%rsp), %xmm11
140025bda:     	movaps	0xc0(%rsp), %xmm12
140025be3:     	movaps	0xb0(%rsp), %xmm13
140025bec:     	movaps	0xa0(%rsp), %xmm14
140025bf5:     	movaps	0x90(%rsp), %xmm15
140025bfe:     	addq	$0x138, %rsp            # imm = 0x138
140025c05:     	popq	%rbx
140025c06:     	popq	%rdi
140025c07:     	popq	%rsi
140025c08:     	popq	%r12
140025c0a:     	popq	%r13
140025c0c:     	popq	%r14
140025c0e:     	popq	%r15
140025c10:     	popq	%rbp
140025c11:     	retq
140025c12:     	nopw	%cs:(%rax,%rax)
140025c20:     	movq	%rdx, 0x10(%rsp)
140025c25:     	pushq	%rbp
140025c26:     	pushq	%r15
140025c28:     	pushq	%r14
140025c2a:     	pushq	%r13
140025c2c:     	pushq	%r12
140025c2e:     	pushq	%rsi
140025c2f:     	pushq	%rdi
140025c30:     	pushq	%rbx
140025c31:     	subq	$0x138, %rsp            # imm = 0x138
140025c38:     	leaq	0x80(%rdx), %rbp
140025c3f:     	movaps	%xmm15, 0x90(%rsp)
140025c48:     	movaps	%xmm14, 0xa0(%rsp)
140025c51:     	movaps	%xmm13, 0xb0(%rsp)
140025c5a:     	movaps	%xmm12, 0xc0(%rsp)
140025c63:     	movaps	%xmm11, 0xd0(%rsp)
140025c6c:     	movaps	%xmm10, 0xe0(%rsp)
140025c75:     	movaps	%xmm9, 0xf0(%rsp)
140025c7e:     	movaps	%xmm8, 0x100(%rsp)
140025c87:     	movaps	%xmm7, 0x110(%rsp)
140025c8f:     	movaps	%xmm6, 0x120(%rsp)
140025c97:     	cmpb	$0x0, 0x808(%rbp)
140025c9e:     	je	0x140025d32 <.text+0x24d32>
140025ca4:     	movq	0x650(%rbp), %rdx
140025cab:     	testq	%rdx, %rdx
140025cae:     	je	0x140025cc6 <.text+0x24cc6>
140025cb0:     	movq	0x658(%rbp), %rcx
140025cb7:     	shlq	$0x3, %rdx
140025cbb:     	movl	$0x4, %r8d
140025cc1:     	callq	0x140613c20 <.text+0x612c20>
140025cc6:     	movaps	0x120(%rsp), %xmm6
140025cce:     	movaps	0x110(%rsp), %xmm7
140025cd6:     	movaps	0x100(%rsp), %xmm8
140025cdf:     	movaps	0xf0(%rsp), %xmm9
140025ce8:     	movaps	0xe0(%rsp), %xmm10
140025cf1:     	movaps	0xd0(%rsp), %xmm11
140025cfa:     	movaps	0xc0(%rsp), %xmm12
140025d03:     	movaps	0xb0(%rsp), %xmm13
140025d0c:     	movaps	0xa0(%rsp), %xmm14
140025d15:     	movaps	0x90(%rsp), %xmm15
140025d1e:     	addq	$0x138, %rsp            # imm = 0x138
140025d25:     	popq	%rbx
140025d26:     	popq	%rdi
140025d27:     	popq	%rsi
140025d28:     	popq	%r12
140025d2a:     	popq	%r13
140025d2c:     	popq	%r14
140025d2e:     	popq	%r15
140025d30:     	popq	%rbp
140025d31:     	retq
140025d32:     	movaps	0x120(%rsp), %xmm6
140025d3a:     	movaps	0x110(%rsp), %xmm7
140025d42:     	movaps	0x100(%rsp), %xmm8
140025d4b:     	movaps	0xf0(%rsp), %xmm9
140025d54:     	movaps	0xe0(%rsp), %xmm10
140025d5d:     	movaps	0xd0(%rsp), %xmm11
140025d66:     	movaps	0xc0(%rsp), %xmm12
140025d6f:     	movaps	0xb0(%rsp), %xmm13
140025d78:     	movaps	0xa0(%rsp), %xmm14
140025d81:     	movaps	0x90(%rsp), %xmm15
140025d8a:     	addq	$0x138, %rsp            # imm = 0x138
140025d91:     	popq	%rbx
140025d92:     	popq	%rdi
140025d93:     	popq	%rsi
140025d94:     	popq	%r12
140025d96:     	popq	%r13
140025d98:     	popq	%r14
140025d9a:     	popq	%r15
140025d9c:     	popq	%rbp
140025d9d:     	retq
140025d9e:     	nop
140025da0:     	movq	%rdx, 0x10(%rsp)
140025da5:     	pushq	%rbp
140025da6:     	pushq	%r15
140025da8:     	pushq	%r14
140025daa:     	pushq	%r13
140025dac:     	pushq	%r12
140025dae:     	pushq	%rsi
140025daf:     	pushq	%rdi
140025db0:     	pushq	%rbx
140025db1:     	subq	$0x138, %rsp            # imm = 0x138
140025db8:     	leaq	0x80(%rdx), %rbp
140025dbf:     	movaps	%xmm15, 0x90(%rsp)
140025dc8:     	movaps	%xmm14, 0xa0(%rsp)
140025dd1:     	movaps	%xmm13, 0xb0(%rsp)
140025dda:     	movaps	%xmm12, 0xc0(%rsp)
140025de3:     	movaps	%xmm11, 0xd0(%rsp)
140025dec:     	movaps	%xmm10, 0xe0(%rsp)
140025df5:     	movaps	%xmm9, 0xf0(%rsp)
140025dfe:     	movaps	%xmm8, 0x100(%rsp)
140025e07:     	movaps	%xmm7, 0x110(%rsp)
140025e0f:     	movaps	%xmm6, 0x120(%rsp)
140025e17:     	cmpb	$0x0, 0x808(%rbp)
140025e1e:     	je	0x140025ebc <.text+0x24ebc>
140025e24:     	movq	0x568(%rbp), %rdx
140025e2b:     	testq	%rdx, %rdx
140025e2e:     	je	0x140025e46 <.text+0x24e46>
140025e30:     	movq	0x570(%rbp), %rcx
140025e37:     	shlq	$0x2, %rdx
140025e3b:     	movl	$0x4, %r8d
140025e41:     	callq	0x140613c20 <.text+0x612c20>
140025e46:     	movb	$0x1, 0x817(%rbp)
140025e4d:     	movaps	0x120(%rsp), %xmm6
140025e55:     	movaps	0x110(%rsp), %xmm7
140025e5d:     	movaps	0x100(%rsp), %xmm8
140025e66:     	movaps	0xf0(%rsp), %xmm9
140025e6f:     	movaps	0xe0(%rsp), %xmm10
140025e78:     	movaps	0xd0(%rsp), %xmm11
140025e81:     	movdqa	0xc0(%rsp), %xmm12
140025e8b:     	movdqa	0xb0(%rsp), %xmm13
140025e95:     	movaps	0xa0(%rsp), %xmm14
140025e9e:     	movdqa	0x90(%rsp), %xmm15
140025ea8:     	addq	$0x138, %rsp            # imm = 0x138
140025eaf:     	popq	%rbx
140025eb0:     	popq	%rdi
140025eb1:     	popq	%rsi
140025eb2:     	popq	%r12
140025eb4:     	popq	%r13
140025eb6:     	popq	%r14
140025eb8:     	popq	%r15
140025eba:     	popq	%rbp
140025ebb:     	retq
140025ebc:     	movb	$0x1, 0x817(%rbp)
140025ec3:     	movaps	0x120(%rsp), %xmm6
140025ecb:     	movaps	0x110(%rsp), %xmm7
140025ed3:     	movaps	0x100(%rsp), %xmm8
140025edc:     	movaps	0xf0(%rsp), %xmm9
140025ee5:     	movaps	0xe0(%rsp), %xmm10
140025eee:     	movaps	0xd0(%rsp), %xmm11
140025ef7:     	movdqa	0xc0(%rsp), %xmm12
140025f01:     	movdqa	0xb0(%rsp), %xmm13
140025f0b:     	movaps	0xa0(%rsp), %xmm14
140025f14:     	movdqa	0x90(%rsp), %xmm15
140025f1e:     	addq	$0x138, %rsp            # imm = 0x138
140025f25:     	popq	%rbx
140025f26:     	popq	%rdi
140025f27:     	popq	%rsi
140025f28:     	popq	%r12
140025f2a:     	popq	%r13
140025f2c:     	popq	%r14
140025f2e:     	popq	%r15
140025f30:     	popq	%rbp
140025f31:     	retq
140025f32:     	nopw	%cs:(%rax,%rax)
140025f40:     	movq	%rdx, 0x10(%rsp)
140025f45:     	pushq	%rbp
140025f46:     	pushq	%r15
140025f48:     	pushq	%r14
140025f4a:     	pushq	%r13
140025f4c:     	pushq	%r12
140025f4e:     	pushq	%rsi
140025f4f:     	pushq	%rdi
140025f50:     	pushq	%rbx
140025f51:     	subq	$0x138, %rsp            # imm = 0x138
140025f58:     	leaq	0x80(%rdx), %rbp
140025f5f:     	movdqa	%xmm15, 0x90(%rsp)
140025f69:     	movaps	%xmm14, 0xa0(%rsp)
140025f72:     	movdqa	%xmm13, 0xb0(%rsp)
140025f7c:     	movdqa	%xmm12, 0xc0(%rsp)
140025f86:     	movaps	%xmm11, 0xd0(%rsp)
140025f8f:     	movaps	%xmm10, 0xe0(%rsp)
140025f98:     	movaps	%xmm9, 0xf0(%rsp)
140025fa1:     	movaps	%xmm8, 0x100(%rsp)
140025faa:     	movaps	%xmm7, 0x110(%rsp)
140025fb2:     	movaps	%xmm6, 0x120(%rsp)
140025fba:     	cmpb	$0x0, 0x813(%rbp)
140025fc1:     	je	0x140026055 <.text+0x25055>
140025fc7:     	movq	0x6a0(%rbp), %rdx
140025fce:     	testq	%rdx, %rdx
140025fd1:     	je	0x140025fe9 <.text+0x24fe9>
140025fd3:     	shlq	$0x2, %rdx
140025fd7:     	movl	$0x4, %r8d
140025fdd:     	movq	0x648(%rbp), %rcx
140025fe4:     	callq	0x140613c20 <.text+0x612c20>
140025fe9:     	movaps	0x120(%rsp), %xmm6
140025ff1:     	movaps	0x110(%rsp), %xmm7
140025ff9:     	movaps	0x100(%rsp), %xmm8
140026002:     	movaps	0xf0(%rsp), %xmm9
14002600b:     	movaps	0xe0(%rsp), %xmm10
140026014:     	movaps	0xd0(%rsp), %xmm11
14002601d:     	movaps	0xc0(%rsp), %xmm12
140026026:     	movaps	0xb0(%rsp), %xmm13
14002602f:     	movaps	0xa0(%rsp), %xmm14
140026038:     	movaps	0x90(%rsp), %xmm15
140026041:     	addq	$0x138, %rsp            # imm = 0x138
140026048:     	popq	%rbx
140026049:     	popq	%rdi
14002604a:     	popq	%rsi
14002604b:     	popq	%r12
14002604d:     	popq	%r13
14002604f:     	popq	%r14
140026051:     	popq	%r15
140026053:     	popq	%rbp
140026054:     	retq
140026055:     	movaps	0x120(%rsp), %xmm6
14002605d:     	movaps	0x110(%rsp), %xmm7
140026065:     	movaps	0x100(%rsp), %xmm8
14002606e:     	movaps	0xf0(%rsp), %xmm9
140026077:     	movaps	0xe0(%rsp), %xmm10
140026080:     	movaps	0xd0(%rsp), %xmm11
140026089:     	movaps	0xc0(%rsp), %xmm12
140026092:     	movaps	0xb0(%rsp), %xmm13
14002609b:     	movaps	0xa0(%rsp), %xmm14
1400260a4:     	movaps	0x90(%rsp), %xmm15
1400260ad:     	addq	$0x138, %rsp            # imm = 0x138
1400260b4:     	popq	%rbx
1400260b5:     	popq	%rdi
1400260b6:     	popq	%rsi
1400260b7:     	popq	%r12
1400260b9:     	popq	%r13
1400260bb:     	popq	%r14
1400260bd:     	popq	%r15
1400260bf:     	popq	%rbp
1400260c0:     	retq
1400260c1:     	nopw	%cs:(%rax,%rax)
1400260d0:     	movq	%rdx, 0x10(%rsp)
1400260d5:     	pushq	%rbp
1400260d6:     	pushq	%r15
1400260d8:     	pushq	%r14
1400260da:     	pushq	%r13
1400260dc:     	pushq	%r12
1400260de:     	pushq	%rsi
1400260df:     	pushq	%rdi
1400260e0:     	pushq	%rbx
1400260e1:     	subq	$0x138, %rsp            # imm = 0x138
1400260e8:     	leaq	0x80(%rdx), %rbp
1400260ef:     	movaps	%xmm15, 0x90(%rsp)
1400260f8:     	movaps	%xmm14, 0xa0(%rsp)
140026101:     	movaps	%xmm13, 0xb0(%rsp)
14002610a:     	movaps	%xmm12, 0xc0(%rsp)
140026113:     	movaps	%xmm11, 0xd0(%rsp)
14002611c:     	movaps	%xmm10, 0xe0(%rsp)
140026125:     	movaps	%xmm9, 0xf0(%rsp)
14002612e:     	movaps	%xmm8, 0x100(%rsp)
140026137:     	movaps	%xmm7, 0x110(%rsp)
14002613f:     	movaps	%xmm6, 0x120(%rsp)
140026147:     	leaq	0x380(%rbp), %rcx
14002614e:     	callq	0x1405e16f0 <.text+0x5e06f0>
140026153:     	movaps	0x120(%rsp), %xmm6
14002615b:     	movaps	0x110(%rsp), %xmm7
140026163:     	movaps	0x100(%rsp), %xmm8
14002616c:     	movaps	0xf0(%rsp), %xmm9
140026175:     	movaps	0xe0(%rsp), %xmm10
14002617e:     	movaps	0xd0(%rsp), %xmm11
140026187:     	movdqa	0xc0(%rsp), %xmm12
140026191:     	movdqa	0xb0(%rsp), %xmm13
14002619b:     	movaps	0xa0(%rsp), %xmm14
1400261a4:     	movdqa	0x90(%rsp), %xmm15
1400261ae:     	addq	$0x138, %rsp            # imm = 0x138
1400261b5:     	popq	%rbx
1400261b6:     	popq	%rdi
1400261b7:     	popq	%rsi
1400261b8:     	popq	%r12
1400261ba:     	popq	%r13
1400261bc:     	popq	%r14
1400261be:     	popq	%r15
1400261c0:     	popq	%rbp
1400261c1:     	retq
1400261c2:     	nopw	%cs:(%rax,%rax)
1400261d0:     	movq	%rdx, 0x10(%rsp)
1400261d5:     	pushq	%rbp
1400261d6:     	pushq	%r15
1400261d8:     	pushq	%r14
1400261da:     	pushq	%r13
1400261dc:     	pushq	%r12
1400261de:     	pushq	%rsi
1400261df:     	pushq	%rdi
1400261e0:     	pushq	%rbx
1400261e1:     	subq	$0x138, %rsp            # imm = 0x138
1400261e8:     	leaq	0x80(%rdx), %rbp
1400261ef:     	movdqa	%xmm15, 0x90(%rsp)
1400261f9:     	movaps	%xmm14, 0xa0(%rsp)
140026202:     	movdqa	%xmm13, 0xb0(%rsp)
14002620c:     	movdqa	%xmm12, 0xc0(%rsp)
140026216:     	movaps	%xmm11, 0xd0(%rsp)
14002621f:     	movaps	%xmm10, 0xe0(%rsp)
140026228:     	movaps	%xmm9, 0xf0(%rsp)
140026231:     	movaps	%xmm8, 0x100(%rsp)
14002623a:     	movaps	%xmm7, 0x110(%rsp)
140026242:     	movaps	%xmm6, 0x120(%rsp)
14002624a:     	leaq	0x6c0(%rbp), %rcx
140026251:     	callq	0x1405e1b90 <.text+0x5e0b90>
140026256:     	movaps	0x120(%rsp), %xmm6
14002625e:     	movaps	0x110(%rsp), %xmm7
140026266:     	movaps	0x100(%rsp), %xmm8
14002626f:     	movaps	0xf0(%rsp), %xmm9
140026278:     	movaps	0xe0(%rsp), %xmm10
140026281:     	movaps	0xd0(%rsp), %xmm11
14002628a:     	movdqa	0xc0(%rsp), %xmm12
140026294:     	movdqa	0xb0(%rsp), %xmm13
14002629e:     	movaps	0xa0(%rsp), %xmm14
1400262a7:     	movdqa	0x90(%rsp), %xmm15
1400262b1:     	addq	$0x138, %rsp            # imm = 0x138
1400262b8:     	popq	%rbx
1400262b9:     	popq	%rdi
1400262ba:     	popq	%rsi
1400262bb:     	popq	%r12
1400262bd:     	popq	%r13
1400262bf:     	popq	%r14
1400262c1:     	popq	%r15
1400262c3:     	popq	%rbp
1400262c4:     	retq
1400262c5:     	nopw	%cs:(%rax,%rax)
1400262d0:     	movq	%rdx, 0x10(%rsp)
1400262d5:     	pushq	%rbp
1400262d6:     	pushq	%r15
1400262d8:     	pushq	%r14
1400262da:     	pushq	%r13
1400262dc:     	pushq	%r12
1400262de:     	pushq	%rsi
1400262df:     	pushq	%rdi
1400262e0:     	pushq	%rbx
1400262e1:     	subq	$0x138, %rsp            # imm = 0x138
1400262e8:     	leaq	0x80(%rdx), %rbp
1400262ef:     	movdqa	%xmm15, 0x90(%rsp)
1400262f9:     	movaps	%xmm14, 0xa0(%rsp)
140026302:     	movdqa	%xmm13, 0xb0(%rsp)
14002630c:     	movdqa	%xmm12, 0xc0(%rsp)
140026316:     	movaps	%xmm11, 0xd0(%rsp)
14002631f:     	movaps	%xmm10, 0xe0(%rsp)
140026328:     	movaps	%xmm9, 0xf0(%rsp)
140026331:     	movaps	%xmm8, 0x100(%rsp)
14002633a:     	movaps	%xmm7, 0x110(%rsp)
140026342:     	movaps	%xmm6, 0x120(%rsp)
14002634a:     	cmpq	$0x0, 0x6a0(%rbp)
140026352:     	movq	0x780(%rbp), %rdx
140026359:     	je	0x140026373 <.text+0x25373>
14002635b:     	testq	%rdx, %rdx
14002635e:     	je	0x140026373 <.text+0x25373>
140026360:     	movq	0x538(%rbp), %rcx
140026367:     	movq	0x648(%rbp), %r8
14002636e:     	callq	0x140613c20 <.text+0x612c20>
140026373:     	movb	$0x0, 0x7ef(%rbp)
14002637a:     	movaps	0x120(%rsp), %xmm6
140026382:     	movaps	0x110(%rsp), %xmm7
14002638a:     	movaps	0x100(%rsp), %xmm8
140026393:     	movaps	0xf0(%rsp), %xmm9
14002639c:     	movaps	0xe0(%rsp), %xmm10
1400263a5:     	movaps	0xd0(%rsp), %xmm11
1400263ae:     	movdqa	0xc0(%rsp), %xmm12
1400263b8:     	movdqa	0xb0(%rsp), %xmm13
1400263c2:     	movaps	0xa0(%rsp), %xmm14
1400263cb:     	movdqa	0x90(%rsp), %xmm15
1400263d5:     	addq	$0x138, %rsp            # imm = 0x138
1400263dc:     	popq	%rbx
1400263dd:     	popq	%rdi
1400263de:     	popq	%rsi
1400263df:     	popq	%r12
1400263e1:     	popq	%r13
1400263e3:     	popq	%r14
1400263e5:     	popq	%r15
1400263e7:     	popq	%rbp
1400263e8:     	retq
1400263e9:     	nopl	(%rax)
1400263f0:     	movq	%rdx, 0x10(%rsp)
1400263f5:     	pushq	%rbp
1400263f6:     	pushq	%r15
1400263f8:     	pushq	%r14
1400263fa:     	pushq	%r13
1400263fc:     	pushq	%r12
1400263fe:     	pushq	%rsi
1400263ff:     	pushq	%rdi
140026400:     	pushq	%rbx
140026401:     	subq	$0x138, %rsp            # imm = 0x138
140026408:     	leaq	0x80(%rdx), %rbp
14002640f:     	movdqa	%xmm15, 0x90(%rsp)
140026419:     	movaps	%xmm14, 0xa0(%rsp)
140026422:     	movdqa	%xmm13, 0xb0(%rsp)
14002642c:     	movdqa	%xmm12, 0xc0(%rsp)
140026436:     	movaps	%xmm11, 0xd0(%rsp)
14002643f:     	movaps	%xmm10, 0xe0(%rsp)
140026448:     	movaps	%xmm9, 0xf0(%rsp)
140026451:     	movaps	%xmm8, 0x100(%rsp)
14002645a:     	movaps	%xmm7, 0x110(%rsp)
140026462:     	movaps	%xmm6, 0x120(%rsp)
14002646a:     	cmpb	$0x0, 0x7ef(%rbp)
140026471:     	je	0x1400264f9 <.text+0x254f9>
140026477:     	leaq	0x460(%rbp), %rcx
14002647e:     	callq	0x140149f50 <.text+0x148f50>
140026483:     	movb	$0x1, 0x805(%rbp)
14002648a:     	movaps	0x120(%rsp), %xmm6
140026492:     	movaps	0x110(%rsp), %xmm7
14002649a:     	movaps	0x100(%rsp), %xmm8
1400264a3:     	movaps	0xf0(%rsp), %xmm9
1400264ac:     	movaps	0xe0(%rsp), %xmm10
1400264b5:     	movaps	0xd0(%rsp), %xmm11
1400264be:     	movdqa	0xc0(%rsp), %xmm12
1400264c8:     	movdqa	0xb0(%rsp), %xmm13
1400264d2:     	movaps	0xa0(%rsp), %xmm14
1400264db:     	movdqa	0x90(%rsp), %xmm15
1400264e5:     	addq	$0x138, %rsp            # imm = 0x138
1400264ec:     	popq	%rbx
1400264ed:     	popq	%rdi
1400264ee:     	popq	%rsi
1400264ef:     	popq	%r12
1400264f1:     	popq	%r13
1400264f3:     	popq	%r14
1400264f5:     	popq	%r15
1400264f7:     	popq	%rbp
1400264f8:     	retq
1400264f9:     	movb	$0x1, 0x805(%rbp)
140026500:     	movaps	0x120(%rsp), %xmm6
140026508:     	movaps	0x110(%rsp), %xmm7
140026510:     	movaps	0x100(%rsp), %xmm8
140026519:     	movaps	0xf0(%rsp), %xmm9
140026522:     	movaps	0xe0(%rsp), %xmm10
14002652b:     	movaps	0xd0(%rsp), %xmm11
140026534:     	movdqa	0xc0(%rsp), %xmm12
14002653e:     	movdqa	0xb0(%rsp), %xmm13
140026548:     	movaps	0xa0(%rsp), %xmm14
140026551:     	movdqa	0x90(%rsp), %xmm15
14002655b:     	addq	$0x138, %rsp            # imm = 0x138
140026562:     	popq	%rbx
140026563:     	popq	%rdi
140026564:     	popq	%rsi
140026565:     	popq	%r12
140026567:     	popq	%r13
140026569:     	popq	%r14
14002656b:     	popq	%r15
14002656d:     	popq	%rbp
14002656e:     	retq
14002656f:     	nop
140026570:     	movq	%rdx, 0x10(%rsp)
140026575:     	pushq	%rbp
140026576:     	pushq	%r15
140026578:     	pushq	%r14
14002657a:     	pushq	%r13
14002657c:     	pushq	%r12
14002657e:     	pushq	%rsi
14002657f:     	pushq	%rdi
140026580:     	pushq	%rbx
140026581:     	subq	$0x138, %rsp            # imm = 0x138
140026588:     	leaq	0x80(%rdx), %rbp
14002658f:     	movdqa	%xmm15, 0x90(%rsp)
140026599:     	movaps	%xmm14, 0xa0(%rsp)
1400265a2:     	movdqa	%xmm13, 0xb0(%rsp)
1400265ac:     	movdqa	%xmm12, 0xc0(%rsp)
1400265b6:     	movaps	%xmm11, 0xd0(%rsp)
1400265bf:     	movaps	%xmm10, 0xe0(%rsp)
1400265c8:     	movaps	%xmm9, 0xf0(%rsp)
1400265d1:     	movaps	%xmm8, 0x100(%rsp)
1400265da:     	movaps	%xmm7, 0x110(%rsp)
1400265e2:     	movaps	%xmm6, 0x120(%rsp)
1400265ea:     	movq	0x648(%rbp), %rdx
1400265f1:     	testq	%rdx, %rdx
1400265f4:     	je	0x14002660c <.text+0x2560c>
1400265f6:     	shlq	$0x4, %rdx
1400265fa:     	movl	$0x8, %r8d
140026600:     	movq	0x580(%rbp), %rcx
140026607:     	callq	0x140613c20 <.text+0x612c20>
14002660c:     	movaps	0x120(%rsp), %xmm6
140026614:     	movaps	0x110(%rsp), %xmm7
14002661c:     	movaps	0x100(%rsp), %xmm8
140026625:     	movaps	0xf0(%rsp), %xmm9
14002662e:     	movaps	0xe0(%rsp), %xmm10
140026637:     	movaps	0xd0(%rsp), %xmm11
140026640:     	movdqa	0xc0(%rsp), %xmm12
14002664a:     	movdqa	0xb0(%rsp), %xmm13
140026654:     	movaps	0xa0(%rsp), %xmm14
14002665d:     	movdqa	0x90(%rsp), %xmm15
140026667:     	addq	$0x138, %rsp            # imm = 0x138
14002666e:     	popq	%rbx
14002666f:     	popq	%rdi
140026670:     	popq	%rsi
140026671:     	popq	%r12
140026673:     	popq	%r13
140026675:     	popq	%r14
140026677:     	popq	%r15
140026679:     	popq	%rbp
14002667a:     	retq
14002667b:     	nopl	(%rax,%rax)
140026680:     	movq	%rdx, 0x10(%rsp)
140026685:     	pushq	%rbp
140026686:     	pushq	%r15
140026688:     	pushq	%r14
14002668a:     	pushq	%r13
14002668c:     	pushq	%r12
14002668e:     	pushq	%rsi
14002668f:     	pushq	%rdi
140026690:     	pushq	%rbx
140026691:     	subq	$0x138, %rsp            # imm = 0x138
140026698:     	leaq	0x80(%rdx), %rbp
14002669f:     	movdqa	%xmm15, 0x90(%rsp)
1400266a9:     	movaps	%xmm14, 0xa0(%rsp)
1400266b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400266bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400266c6:     	movaps	%xmm11, 0xd0(%rsp)
1400266cf:     	movaps	%xmm10, 0xe0(%rsp)
1400266d8:     	movaps	%xmm9, 0xf0(%rsp)
1400266e1:     	movaps	%xmm8, 0x100(%rsp)
1400266ea:     	movaps	%xmm7, 0x110(%rsp)
1400266f2:     	movaps	%xmm6, 0x120(%rsp)
1400266fa:     	movq	0x560(%rbp), %rax
140026701:     	cmpq	0x7e0(%rbp), %rax
140026708:     	jne	0x140026780 <.text+0x25780>
14002670a:     	movb	$0x1, 0x814(%rbp)
140026711:     	movaps	0x120(%rsp), %xmm6
140026719:     	movaps	0x110(%rsp), %xmm7
140026721:     	movaps	0x100(%rsp), %xmm8
14002672a:     	movaps	0xf0(%rsp), %xmm9
140026733:     	movaps	0xe0(%rsp), %xmm10
14002673c:     	movaps	0xd0(%rsp), %xmm11
140026745:     	movdqa	0xc0(%rsp), %xmm12
14002674f:     	movdqa	0xb0(%rsp), %xmm13
140026759:     	movaps	0xa0(%rsp), %xmm14
140026762:     	movdqa	0x90(%rsp), %xmm15
14002676c:     	addq	$0x138, %rsp            # imm = 0x138
140026773:     	popq	%rbx
140026774:     	popq	%rdi
140026775:     	popq	%rsi
140026776:     	popq	%r12
140026778:     	popq	%r13
14002677a:     	popq	%r14
14002677c:     	popq	%r15
14002677e:     	popq	%rbp
14002677f:     	retq
140026780:     	movq	0x500(%rbp), %r14
140026787:     	testq	%r14, %r14
14002678a:     	movq	0x528(%rbp), %rdi
140026791:     	movq	0x640(%rbp), %rbx
140026798:     	jne	0x140026851 <.text+0x25851>
14002679e:     	testq	%rdi, %rdi
1400267a1:     	je	0x1400267bd <.text+0x257bd>
1400267a3:     	shlq	$0x3, %rdi
1400267a7:     	leaq	(%rdi,%rdi,2), %rdx
1400267ab:     	movl	$0x8, %r8d
1400267b1:     	movq	0x6a8(%rbp), %rcx
1400267b8:     	callq	0x140613c20 <.text+0x612c20>
1400267bd:     	testq	%rbx, %rbx
1400267c0:     	je	0x1400267db <.text+0x257db>
1400267c2:     	shlq	$0x4, %rbx
1400267c6:     	movl	$0x8, %r8d
1400267cc:     	movq	0x540(%rbp), %rcx
1400267d3:     	movq	%rbx, %rdx
1400267d6:     	callq	0x140613c20 <.text+0x612c20>
1400267db:     	movb	$0x1, 0x814(%rbp)
1400267e2:     	movaps	0x120(%rsp), %xmm6
1400267ea:     	movaps	0x110(%rsp), %xmm7
1400267f2:     	movaps	0x100(%rsp), %xmm8
1400267fb:     	movaps	0xf0(%rsp), %xmm9
140026804:     	movaps	0xe0(%rsp), %xmm10
14002680d:     	movaps	0xd0(%rsp), %xmm11
140026816:     	movdqa	0xc0(%rsp), %xmm12
140026820:     	movdqa	0xb0(%rsp), %xmm13
14002682a:     	movaps	0xa0(%rsp), %xmm14
140026833:     	movdqa	0x90(%rsp), %xmm15
14002683d:     	addq	$0x138, %rsp            # imm = 0x138
140026844:     	popq	%rbx
140026845:     	popq	%rdi
140026846:     	popq	%rsi
140026847:     	popq	%r12
140026849:     	popq	%r13
14002684b:     	popq	%r14
14002684d:     	popq	%r15
14002684f:     	popq	%rbp
140026850:     	retq
140026851:     	movq	0x6a8(%rbp), %rax
140026858:     	leaq	0x8(%rax), %rsi
14002685c:     	jmp	0x14002686d <.text+0x2586d>
14002685e:     	nop
140026860:     	addq	$0x18, %rsi
140026864:     	decq	%r14
140026867:     	je	0x14002679e <.text+0x2579e>
14002686d:     	movq	-0x8(%rsi), %rdx
140026871:     	testq	%rdx, %rdx
140026874:     	je	0x140026860 <.text+0x25860>
140026876:     	movq	(%rsi), %rcx
140026879:     	shlq	$0x2, %rdx
14002687d:     	movl	$0x4, %r8d
140026883:     	callq	0x140613c20 <.text+0x612c20>
140026888:     	jmp	0x140026860 <.text+0x25860>
14002688a:     	nopw	(%rax,%rax)
140026890:     	movq	%rdx, 0x10(%rsp)
140026895:     	pushq	%rbp
140026896:     	pushq	%r15
140026898:     	pushq	%r14
14002689a:     	pushq	%r13
14002689c:     	pushq	%r12
14002689e:     	pushq	%rsi
14002689f:     	pushq	%rdi
1400268a0:     	pushq	%rbx
1400268a1:     	subq	$0x138, %rsp            # imm = 0x138
1400268a8:     	leaq	0x80(%rdx), %rbp
1400268af:     	movdqa	%xmm15, 0x90(%rsp)
1400268b9:     	movaps	%xmm14, 0xa0(%rsp)
1400268c2:     	movdqa	%xmm13, 0xb0(%rsp)
1400268cc:     	movdqa	%xmm12, 0xc0(%rsp)
1400268d6:     	movaps	%xmm11, 0xd0(%rsp)
1400268df:     	movaps	%xmm10, 0xe0(%rsp)
1400268e8:     	movaps	%xmm9, 0xf0(%rsp)
1400268f1:     	movaps	%xmm8, 0x100(%rsp)
1400268fa:     	movaps	%xmm7, 0x110(%rsp)
140026902:     	movaps	%xmm6, 0x120(%rsp)
14002690a:     	cmpb	$0x0, 0x814(%rbp)
140026911:     	je	0x140026a0b <.text+0x25a0b>
140026917:     	movq	0x460(%rbp), %rax
14002691e:     	testq	%rax, %rax
140026921:     	jne	0x140026a9d <.text+0x25a9d>
140026927:     	movq	0x478(%rbp), %rdx
14002692e:     	testq	%rdx, %rdx
140026931:     	jne	0x140026ac7 <.text+0x25ac7>
140026937:     	movq	0x498(%rbp), %rdx
14002693e:     	testq	%rdx, %rdx
140026941:     	jne	0x140026aed <.text+0x25aed>
140026947:     	movq	0x4b0(%rbp), %rdx
14002694e:     	testq	%rdx, %rdx
140026951:     	jne	0x140026b13 <.text+0x25b13>
140026957:     	movq	0x4c8(%rbp), %rdx
14002695e:     	testq	%rdx, %rdx
140026961:     	je	0x140026979 <.text+0x25979>
140026963:     	movq	0x4d0(%rbp), %rcx
14002696a:     	shlq	$0x2, %rdx
14002696e:     	movl	$0x4, %r8d
140026974:     	callq	0x140613c20 <.text+0x612c20>
140026979:     	movb	$0x0, 0x803(%rbp)
140026980:     	movb	$0x0, 0x802(%rbp)
140026987:     	movb	$0x0, 0x801(%rbp)
14002698e:     	movb	$0x1, 0x800(%rbp)
140026995:     	movb	$0x1, 0x7ff(%rbp)
14002699c:     	movaps	0x120(%rsp), %xmm6
1400269a4:     	movaps	0x110(%rsp), %xmm7
1400269ac:     	movaps	0x100(%rsp), %xmm8
1400269b5:     	movaps	0xf0(%rsp), %xmm9
1400269be:     	movaps	0xe0(%rsp), %xmm10
1400269c7:     	movaps	0xd0(%rsp), %xmm11
1400269d0:     	movdqa	0xc0(%rsp), %xmm12
1400269da:     	movdqa	0xb0(%rsp), %xmm13
1400269e4:     	movaps	0xa0(%rsp), %xmm14
1400269ed:     	movdqa	0x90(%rsp), %xmm15
1400269f7:     	addq	$0x138, %rsp            # imm = 0x138
1400269fe:     	popq	%rbx
1400269ff:     	popq	%rdi
140026a00:     	popq	%rsi
140026a01:     	popq	%r12
140026a03:     	popq	%r13
140026a05:     	popq	%r14
140026a07:     	popq	%r15
140026a09:     	popq	%rbp
140026a0a:     	retq
140026a0b:     	movb	$0x0, 0x803(%rbp)
140026a12:     	movb	$0x0, 0x802(%rbp)
140026a19:     	movb	$0x0, 0x801(%rbp)
140026a20:     	movb	$0x1, 0x800(%rbp)
140026a27:     	movb	$0x1, 0x7ff(%rbp)
140026a2e:     	movaps	0x120(%rsp), %xmm6
140026a36:     	movaps	0x110(%rsp), %xmm7
140026a3e:     	movaps	0x100(%rsp), %xmm8
140026a47:     	movaps	0xf0(%rsp), %xmm9
140026a50:     	movaps	0xe0(%rsp), %xmm10
140026a59:     	movaps	0xd0(%rsp), %xmm11
140026a62:     	movdqa	0xc0(%rsp), %xmm12
140026a6c:     	movdqa	0xb0(%rsp), %xmm13
140026a76:     	movaps	0xa0(%rsp), %xmm14
140026a7f:     	movdqa	0x90(%rsp), %xmm15
140026a89:     	addq	$0x138, %rsp            # imm = 0x138
140026a90:     	popq	%rbx
140026a91:     	popq	%rdi
140026a92:     	popq	%rsi
140026a93:     	popq	%r12
140026a95:     	popq	%r13
140026a97:     	popq	%r14
140026a99:     	popq	%r15
140026a9b:     	popq	%rbp
140026a9c:     	retq
140026a9d:     	movq	0x468(%rbp), %rcx
140026aa4:     	shlq	$0x2, %rax
140026aa8:     	leaq	(%rax,%rax,2), %rdx
140026aac:     	movl	$0x4, %r8d
140026ab2:     	callq	0x140613c20 <.text+0x612c20>
140026ab7:     	movq	0x478(%rbp), %rdx
140026abe:     	testq	%rdx, %rdx
140026ac1:     	je	0x140026937 <.text+0x25937>
140026ac7:     	movq	0x480(%rbp), %rcx
140026ace:     	shlq	$0x2, %rdx
140026ad2:     	movl	$0x4, %r8d
140026ad8:     	callq	0x140613c20 <.text+0x612c20>
140026add:     	movq	0x498(%rbp), %rdx
140026ae4:     	testq	%rdx, %rdx
140026ae7:     	je	0x140026947 <.text+0x25947>
140026aed:     	movq	0x4a0(%rbp), %rcx
140026af4:     	shlq	$0x3, %rdx
140026af8:     	movl	$0x4, %r8d
140026afe:     	callq	0x140613c20 <.text+0x612c20>
140026b03:     	movq	0x4b0(%rbp), %rdx
140026b0a:     	testq	%rdx, %rdx
140026b0d:     	je	0x140026957 <.text+0x25957>
140026b13:     	movq	0x4b8(%rbp), %rcx
140026b1a:     	shlq	$0x2, %rdx
140026b1e:     	movl	$0x4, %r8d
140026b24:     	callq	0x140613c20 <.text+0x612c20>
140026b29:     	movq	0x4c8(%rbp), %rdx
140026b30:     	testq	%rdx, %rdx
140026b33:     	jne	0x140026963 <.text+0x25963>
140026b39:     	jmp	0x140026979 <.text+0x25979>
140026b3e:     	nop
140026b40:     	movq	%rdx, 0x10(%rsp)
140026b45:     	pushq	%rbp
140026b46:     	pushq	%r15
140026b48:     	pushq	%r14
140026b4a:     	pushq	%r13
140026b4c:     	pushq	%r12
140026b4e:     	pushq	%rsi
140026b4f:     	pushq	%rdi
140026b50:     	pushq	%rbx
140026b51:     	subq	$0x138, %rsp            # imm = 0x138
140026b58:     	leaq	0x80(%rdx), %rbp
140026b5f:     	movdqa	%xmm15, 0x90(%rsp)
140026b69:     	movaps	%xmm14, 0xa0(%rsp)
140026b72:     	movdqa	%xmm13, 0xb0(%rsp)
140026b7c:     	movdqa	%xmm12, 0xc0(%rsp)
140026b86:     	movaps	%xmm11, 0xd0(%rsp)
140026b8f:     	movaps	%xmm10, 0xe0(%rsp)
140026b98:     	movaps	%xmm9, 0xf0(%rsp)
140026ba1:     	movaps	%xmm8, 0x100(%rsp)
140026baa:     	movaps	%xmm7, 0x110(%rsp)
140026bb2:     	movaps	%xmm6, 0x120(%rsp)
140026bba:     	movzbl	0x800(%rbp), %r14d
140026bc2:     	movzbl	0x801(%rbp), %edi
140026bc9:     	movzbl	0x802(%rbp), %esi
140026bd0:     	movzbl	0x803(%rbp), %ebx
140026bd7:     	cmpq	$0x0, 0x808(%rbp)
140026bdf:     	je	0x140026c0b <.text+0x25c0b>
140026be1:     	testb	$0x1, 0x7ff(%rbp)
140026be8:     	jne	0x140026c0b <.text+0x25c0b>
140026bea:     	movq	0x808(%rbp), %rax
140026bf1:     	shlq	$0x2, %rax
140026bf5:     	leaq	(%rax,%rax,2), %rdx
140026bf9:     	movl	$0x4, %r8d
140026bff:     	movq	0x6b0(%rbp), %rcx
140026c06:     	callq	0x140613c20 <.text+0x612c20>
140026c0b:     	movq	0x558(%rbp), %rdx
140026c12:     	testq	%rdx, %rdx
140026c15:     	sete	%al
140026c18:     	orb	%al, %r14b
140026c1b:     	testb	$0x1, %r14b
140026c1f:     	jne	0x140026c37 <.text+0x25c37>
140026c21:     	shlq	$0x2, %rdx
140026c25:     	movl	$0x4, %r8d
140026c2b:     	movq	0x430(%rbp), %rcx
140026c32:     	callq	0x140613c20 <.text+0x612c20>
140026c37:     	movq	0x790(%rbp), %rdx
140026c3e:     	testq	%rdx, %rdx
140026c41:     	setne	%al
140026c44:     	testb	%al, %dil
140026c47:     	je	0x140026c5f <.text+0x25c5f>
140026c49:     	shlq	$0x3, %rdx
140026c4d:     	movl	$0x4, %r8d
140026c53:     	movq	0x7f0(%rbp), %rcx
140026c5a:     	callq	0x140613c20 <.text+0x612c20>
140026c5f:     	movq	0x7a0(%rbp), %rdx
140026c66:     	testq	%rdx, %rdx
140026c69:     	setne	%al
140026c6c:     	testb	%al, %sil
140026c6f:     	je	0x140026c87 <.text+0x25c87>
140026c71:     	shlq	$0x2, %rdx
140026c75:     	movl	$0x4, %r8d
140026c7b:     	movq	0x7d0(%rbp), %rcx
140026c82:     	callq	0x140613c20 <.text+0x612c20>
140026c87:     	testb	$0x1, %bl
140026c8a:     	je	0x140026d1e <.text+0x25d1e>
140026c90:     	movq	0x698(%rbp), %rdx
140026c97:     	testq	%rdx, %rdx
140026c9a:     	je	0x140026cb2 <.text+0x25cb2>
140026c9c:     	shlq	$0x2, %rdx
140026ca0:     	movl	$0x4, %r8d
140026ca6:     	movq	0x7d8(%rbp), %rcx
140026cad:     	callq	0x140613c20 <.text+0x612c20>
140026cb2:     	movaps	0x120(%rsp), %xmm6
140026cba:     	movaps	0x110(%rsp), %xmm7
140026cc2:     	movaps	0x100(%rsp), %xmm8
140026ccb:     	movaps	0xf0(%rsp), %xmm9
140026cd4:     	movaps	0xe0(%rsp), %xmm10
140026cdd:     	movaps	0xd0(%rsp), %xmm11
140026ce6:     	movaps	0xc0(%rsp), %xmm12
140026cef:     	movaps	0xb0(%rsp), %xmm13
140026cf8:     	movaps	0xa0(%rsp), %xmm14
140026d01:     	movaps	0x90(%rsp), %xmm15
140026d0a:     	addq	$0x138, %rsp            # imm = 0x138
140026d11:     	popq	%rbx
140026d12:     	popq	%rdi
140026d13:     	popq	%rsi
140026d14:     	popq	%r12
140026d16:     	popq	%r13
140026d18:     	popq	%r14
140026d1a:     	popq	%r15
140026d1c:     	popq	%rbp
140026d1d:     	retq
140026d1e:     	movaps	0x120(%rsp), %xmm6
140026d26:     	movaps	0x110(%rsp), %xmm7
140026d2e:     	movaps	0x100(%rsp), %xmm8
140026d37:     	movaps	0xf0(%rsp), %xmm9
140026d40:     	movaps	0xe0(%rsp), %xmm10
140026d49:     	movaps	0xd0(%rsp), %xmm11
140026d52:     	movaps	0xc0(%rsp), %xmm12
140026d5b:     	movaps	0xb0(%rsp), %xmm13
140026d64:     	movaps	0xa0(%rsp), %xmm14
140026d6d:     	movaps	0x90(%rsp), %xmm15
140026d76:     	addq	$0x138, %rsp            # imm = 0x138
140026d7d:     	popq	%rbx
140026d7e:     	popq	%rdi
140026d7f:     	popq	%rsi
140026d80:     	popq	%r12
140026d82:     	popq	%r13
140026d84:     	popq	%r14
140026d86:     	popq	%r15
140026d88:     	popq	%rbp
140026d89:     	retq
140026d8a:     	nopw	(%rax,%rax)
140026d90:     	movq	%rdx, 0x10(%rsp)
140026d95:     	pushq	%rbp
140026d96:     	pushq	%r15
140026d98:     	pushq	%r14
140026d9a:     	pushq	%r13
140026d9c:     	pushq	%r12
140026d9e:     	pushq	%rsi
140026d9f:     	pushq	%rdi
140026da0:     	pushq	%rbx
140026da1:     	subq	$0x138, %rsp            # imm = 0x138
140026da8:     	leaq	0x80(%rdx), %rbp
140026daf:     	movaps	%xmm15, 0x90(%rsp)
140026db8:     	movaps	%xmm14, 0xa0(%rsp)
140026dc1:     	movaps	%xmm13, 0xb0(%rsp)
140026dca:     	movaps	%xmm12, 0xc0(%rsp)
140026dd3:     	movaps	%xmm11, 0xd0(%rsp)
140026ddc:     	movaps	%xmm10, 0xe0(%rsp)
140026de5:     	movaps	%xmm9, 0xf0(%rsp)
140026dee:     	movaps	%xmm8, 0x100(%rsp)
140026df7:     	movaps	%xmm7, 0x110(%rsp)
140026dff:     	movaps	%xmm6, 0x120(%rsp)
140026e07:     	xorl	%eax, %eax
140026e09:     	cmpq	0x6c0(%rbp), %rax
140026e10:     	jo	0x140026e1e <.text+0x25e1e>
140026e12:     	leaq	0x6c0(%rbp), %rcx
140026e19:     	callq	0x140609eb0 <.text+0x608eb0>
140026e1e:     	movb	$0x0, 0x805(%rbp)
140026e25:     	movaps	0x120(%rsp), %xmm6
140026e2d:     	movaps	0x110(%rsp), %xmm7
140026e35:     	movaps	0x100(%rsp), %xmm8
140026e3e:     	movaps	0xf0(%rsp), %xmm9
140026e47:     	movaps	0xe0(%rsp), %xmm10
140026e50:     	movaps	0xd0(%rsp), %xmm11
140026e59:     	movdqa	0xc0(%rsp), %xmm12
140026e63:     	movdqa	0xb0(%rsp), %xmm13
140026e6d:     	movaps	0xa0(%rsp), %xmm14
140026e76:     	movdqa	0x90(%rsp), %xmm15
140026e80:     	addq	$0x138, %rsp            # imm = 0x138
140026e87:     	popq	%rbx
140026e88:     	popq	%rdi
140026e89:     	popq	%rsi
140026e8a:     	popq	%r12
140026e8c:     	popq	%r13
140026e8e:     	popq	%r14
140026e90:     	popq	%r15
140026e92:     	popq	%rbp
140026e93:     	retq
140026e94:     	nopw	%cs:(%rax,%rax)
140026ea0:     	movq	%rdx, 0x10(%rsp)
140026ea5:     	pushq	%rbp
140026ea6:     	pushq	%r15
140026ea8:     	pushq	%r14
140026eaa:     	pushq	%r13
140026eac:     	pushq	%r12
140026eae:     	pushq	%rsi
140026eaf:     	pushq	%rdi
140026eb0:     	pushq	%rbx
140026eb1:     	subq	$0x138, %rsp            # imm = 0x138
140026eb8:     	leaq	0x80(%rdx), %rbp
140026ebf:     	movdqa	%xmm15, 0x90(%rsp)
140026ec9:     	movaps	%xmm14, 0xa0(%rsp)
140026ed2:     	movdqa	%xmm13, 0xb0(%rsp)
140026edc:     	movdqa	%xmm12, 0xc0(%rsp)
140026ee6:     	movaps	%xmm11, 0xd0(%rsp)
140026eef:     	movaps	%xmm10, 0xe0(%rsp)
140026ef8:     	movaps	%xmm9, 0xf0(%rsp)
140026f01:     	movaps	%xmm8, 0x100(%rsp)
140026f0a:     	movaps	%xmm7, 0x110(%rsp)
140026f12:     	movaps	%xmm6, 0x120(%rsp)
140026f1a:     	cmpb	$0x0, 0x805(%rbp)
140026f21:     	je	0x140026fbf <.text+0x25fbf>
140026f27:     	cmpb	$0x11, 0x508(%rbp)
140026f2e:     	je	0x140026f49 <.text+0x25f49>
140026f30:     	movq	0x520(%rbp), %rax
140026f37:     	lock
140026f38:     	decq	(%rax)
140026f3b:     	jne	0x140026f49 <.text+0x25f49>
140026f3d:     	leaq	0x520(%rbp), %rcx
140026f44:     	callq	0x140b5c620 <.text+0xb5b620>
140026f49:     	movb	$0x0, 0x817(%rbp)
140026f50:     	movaps	0x120(%rsp), %xmm6
140026f58:     	movaps	0x110(%rsp), %xmm7
140026f60:     	movaps	0x100(%rsp), %xmm8
140026f69:     	movaps	0xf0(%rsp), %xmm9
140026f72:     	movaps	0xe0(%rsp), %xmm10
140026f7b:     	movaps	0xd0(%rsp), %xmm11
140026f84:     	movdqa	0xc0(%rsp), %xmm12
140026f8e:     	movdqa	0xb0(%rsp), %xmm13
140026f98:     	movaps	0xa0(%rsp), %xmm14
140026fa1:     	movdqa	0x90(%rsp), %xmm15
140026fab:     	addq	$0x138, %rsp            # imm = 0x138
140026fb2:     	popq	%rbx
140026fb3:     	popq	%rdi
140026fb4:     	popq	%rsi
140026fb5:     	popq	%r12
140026fb7:     	popq	%r13
140026fb9:     	popq	%r14
140026fbb:     	popq	%r15
140026fbd:     	popq	%rbp
140026fbe:     	retq
140026fbf:     	movb	$0x0, 0x817(%rbp)
140026fc6:     	movaps	0x120(%rsp), %xmm6
140026fce:     	movaps	0x110(%rsp), %xmm7
140026fd6:     	movaps	0x100(%rsp), %xmm8
140026fdf:     	movaps	0xf0(%rsp), %xmm9
140026fe8:     	movaps	0xe0(%rsp), %xmm10
140026ff1:     	movaps	0xd0(%rsp), %xmm11
140026ffa:     	movdqa	0xc0(%rsp), %xmm12
140027004:     	movdqa	0xb0(%rsp), %xmm13
14002700e:     	movaps	0xa0(%rsp), %xmm14
140027017:     	movdqa	0x90(%rsp), %xmm15
140027021:     	addq	$0x138, %rsp            # imm = 0x138
140027028:     	popq	%rbx
140027029:     	popq	%rdi
14002702a:     	popq	%rsi
14002702b:     	popq	%r12
14002702d:     	popq	%r13
14002702f:     	popq	%r14
140027031:     	popq	%r15
140027033:     	popq	%rbp
140027034:     	retq
140027035:     	nopw	%cs:(%rax,%rax)
140027040:     	movq	%rdx, 0x10(%rsp)
140027045:     	pushq	%rbp
140027046:     	pushq	%r15
140027048:     	pushq	%r14
14002704a:     	pushq	%r13
14002704c:     	pushq	%r12
14002704e:     	pushq	%rsi
14002704f:     	pushq	%rdi
140027050:     	pushq	%rbx
140027051:     	subq	$0x138, %rsp            # imm = 0x138
140027058:     	leaq	0x80(%rdx), %rbp
14002705f:     	movdqa	%xmm15, 0x90(%rsp)
140027069:     	movaps	%xmm14, 0xa0(%rsp)
140027072:     	movdqa	%xmm13, 0xb0(%rsp)
14002707c:     	movdqa	%xmm12, 0xc0(%rsp)
140027086:     	movaps	%xmm11, 0xd0(%rsp)
14002708f:     	movaps	%xmm10, 0xe0(%rsp)
140027098:     	movaps	%xmm9, 0xf0(%rsp)
1400270a1:     	movaps	%xmm8, 0x100(%rsp)
1400270aa:     	movaps	%xmm7, 0x110(%rsp)
1400270b2:     	movaps	%xmm6, 0x120(%rsp)
1400270ba:     	cmpb	$0x0, 0x817(%rbp)
1400270c1:     	je	0x140027181 <.text+0x26181>
1400270c7:     	movq	0x280(%rbp), %rsi
1400270ce:     	movq	0x288(%rbp), %rdi
1400270d5:     	xorl	%ebx, %ebx
1400270d7:     	movq	%rsi, %rcx
1400270da:     	cmpq	%rbx, %rdi
1400270dd:     	je	0x1400270f7 <.text+0x260f7>
1400270df:     	nop
1400270e0:     	incq	%rbx
1400270e3:     	leaq	0x80(%rcx), %r14
1400270ea:     	callq	0x140601be0 <.text+0x600be0>
1400270ef:     	movq	%r14, %rcx
1400270f2:     	cmpq	%rbx, %rdi
1400270f5:     	jne	0x1400270e0 <.text+0x260e0>
1400270f7:     	movq	0x278(%rbp), %rdx
1400270fe:     	testq	%rdx, %rdx
140027101:     	je	0x140027115 <.text+0x26115>
140027103:     	shlq	$0x7, %rdx
140027107:     	movl	$0x10, %r8d
14002710d:     	movq	%rsi, %rcx
140027110:     	callq	0x140613c20 <.text+0x612c20>
140027115:     	movaps	0x120(%rsp), %xmm6
14002711d:     	movaps	0x110(%rsp), %xmm7
140027125:     	movaps	0x100(%rsp), %xmm8
14002712e:     	movaps	0xf0(%rsp), %xmm9
140027137:     	movaps	0xe0(%rsp), %xmm10
140027140:     	movaps	0xd0(%rsp), %xmm11
140027149:     	movaps	0xc0(%rsp), %xmm12
140027152:     	movaps	0xb0(%rsp), %xmm13
14002715b:     	movaps	0xa0(%rsp), %xmm14
140027164:     	movaps	0x90(%rsp), %xmm15
14002716d:     	addq	$0x138, %rsp            # imm = 0x138
140027174:     	popq	%rbx
140027175:     	popq	%rdi
140027176:     	popq	%rsi
140027177:     	popq	%r12
140027179:     	popq	%r13
14002717b:     	popq	%r14
14002717d:     	popq	%r15
14002717f:     	popq	%rbp
140027180:     	retq
140027181:     	movaps	0x120(%rsp), %xmm6
140027189:     	movaps	0x110(%rsp), %xmm7
140027191:     	movaps	0x100(%rsp), %xmm8
14002719a:     	movaps	0xf0(%rsp), %xmm9
1400271a3:     	movaps	0xe0(%rsp), %xmm10
1400271ac:     	movaps	0xd0(%rsp), %xmm11
1400271b5:     	movaps	0xc0(%rsp), %xmm12
1400271be:     	movaps	0xb0(%rsp), %xmm13
1400271c7:     	movaps	0xa0(%rsp), %xmm14
1400271d0:     	movaps	0x90(%rsp), %xmm15
1400271d9:     	addq	$0x138, %rsp            # imm = 0x138
1400271e0:     	popq	%rbx
1400271e1:     	popq	%rdi
1400271e2:     	popq	%rsi
1400271e3:     	popq	%r12
1400271e5:     	popq	%r13
1400271e7:     	popq	%r14
1400271e9:     	popq	%r15
1400271eb:     	popq	%rbp
1400271ec:     	retq
1400271ed:     	nopl	(%rax)
1400271f0:     	movq	%rdx, 0x10(%rsp)
1400271f5:     	pushq	%rbp
1400271f6:     	pushq	%r15
1400271f8:     	pushq	%r14
1400271fa:     	pushq	%r13
1400271fc:     	pushq	%r12
1400271fe:     	pushq	%rsi
1400271ff:     	pushq	%rdi
140027200:     	pushq	%rbx
140027201:     	subq	$0x138, %rsp            # imm = 0x138
140027208:     	leaq	0x80(%rdx), %rbp
14002720f:     	movaps	%xmm15, 0x90(%rsp)
140027218:     	movaps	%xmm14, 0xa0(%rsp)
140027221:     	movaps	%xmm13, 0xb0(%rsp)
14002722a:     	movaps	%xmm12, 0xc0(%rsp)
140027233:     	movaps	%xmm11, 0xd0(%rsp)
14002723c:     	movaps	%xmm10, 0xe0(%rsp)
140027245:     	movaps	%xmm9, 0xf0(%rsp)
14002724e:     	movaps	%xmm8, 0x100(%rsp)
140027257:     	movaps	%xmm7, 0x110(%rsp)
14002725f:     	movaps	%xmm6, 0x120(%rsp)
140027267:     	leaq	0x440(%rbp), %rcx
14002726e:     	callq	0x1405dec70 <.text+0x5ddc70>
140027273:     	movaps	0x120(%rsp), %xmm6
14002727b:     	movaps	0x110(%rsp), %xmm7
140027283:     	movaps	0x100(%rsp), %xmm8
14002728c:     	movaps	0xf0(%rsp), %xmm9
140027295:     	movaps	0xe0(%rsp), %xmm10
14002729e:     	movaps	0xd0(%rsp), %xmm11
1400272a7:     	movdqa	0xc0(%rsp), %xmm12
1400272b1:     	movdqa	0xb0(%rsp), %xmm13
1400272bb:     	movaps	0xa0(%rsp), %xmm14
1400272c4:     	movdqa	0x90(%rsp), %xmm15
1400272ce:     	addq	$0x138, %rsp            # imm = 0x138
1400272d5:     	popq	%rbx
1400272d6:     	popq	%rdi
1400272d7:     	popq	%rsi
1400272d8:     	popq	%r12
1400272da:     	popq	%r13
1400272dc:     	popq	%r14
1400272de:     	popq	%r15
1400272e0:     	popq	%rbp
1400272e1:     	retq
1400272e2:     	nopw	%cs:(%rax,%rax)
1400272f0:     	movq	%rdx, 0x10(%rsp)
1400272f5:     	pushq	%rbp
1400272f6:     	pushq	%r15
1400272f8:     	pushq	%r14
1400272fa:     	pushq	%r13
1400272fc:     	pushq	%r12
1400272fe:     	pushq	%rsi
1400272ff:     	pushq	%rdi
140027300:     	pushq	%rbx
140027301:     	subq	$0x138, %rsp            # imm = 0x138
140027308:     	leaq	0x80(%rdx), %rbp
14002730f:     	movdqa	%xmm15, 0x90(%rsp)
140027319:     	movaps	%xmm14, 0xa0(%rsp)
140027322:     	movdqa	%xmm13, 0xb0(%rsp)
14002732c:     	movdqa	%xmm12, 0xc0(%rsp)
140027336:     	movaps	%xmm11, 0xd0(%rsp)
14002733f:     	movaps	%xmm10, 0xe0(%rsp)
140027348:     	movaps	%xmm9, 0xf0(%rsp)
140027351:     	movaps	%xmm8, 0x100(%rsp)
14002735a:     	movaps	%xmm7, 0x110(%rsp)
140027362:     	movaps	%xmm6, 0x120(%rsp)
14002736a:     	movq	0x160(%rbp), %rdi
140027371:     	leaq	0x18(%rdi), %rsi
140027375:     	movq	%rsi, %rcx
140027378:     	callq	0x140c81e10 <.text+0xc80e10>
14002737d:     	xorl	%ecx, %ecx
14002737f:     	movb	$0x1, %al
140027381:     	lock
140027382:     	cmpxchgb	%cl, 0x18(%rdi)
140027386:     	jne	0x140027416 <.text+0x26416>
14002738c:     	movq	0x1b8(%rbp), %rdi
140027393:     	leaq	0x18(%rdi), %rsi
140027397:     	movq	%rsi, %rcx
14002739a:     	callq	0x140c81e10 <.text+0xc80e10>
14002739f:     	xorl	%ecx, %ecx
1400273a1:     	movb	$0x1, %al
1400273a3:     	lock
1400273a4:     	cmpxchgb	%cl, 0x18(%rdi)
1400273a8:     	jne	0x140027425 <.text+0x26425>
1400273aa:     	movaps	0x120(%rsp), %xmm6
1400273b2:     	movaps	0x110(%rsp), %xmm7
1400273ba:     	movaps	0x100(%rsp), %xmm8
1400273c3:     	movaps	0xf0(%rsp), %xmm9
1400273cc:     	movaps	0xe0(%rsp), %xmm10
1400273d5:     	movaps	0xd0(%rsp), %xmm11
1400273de:     	movaps	0xc0(%rsp), %xmm12
1400273e7:     	movaps	0xb0(%rsp), %xmm13
1400273f0:     	movaps	0xa0(%rsp), %xmm14
1400273f9:     	movaps	0x90(%rsp), %xmm15
140027402:     	addq	$0x138, %rsp            # imm = 0x138
140027409:     	popq	%rbx
14002740a:     	popq	%rdi
14002740b:     	popq	%rsi
14002740c:     	popq	%r12
14002740e:     	popq	%r13
140027410:     	popq	%r14
140027412:     	popq	%r15
140027414:     	popq	%rbp
140027415:     	retq
140027416:     	movq	%rsi, %rcx
140027419:     	xorl	%edx, %edx
14002741b:     	callq	0x1428cdb90 <.text+0x28ccb90>
140027420:     	jmp	0x14002738c <.text+0x2638c>
140027425:     	movq	%rsi, %rcx
140027428:     	xorl	%edx, %edx
14002742a:     	callq	0x1428cdb90 <.text+0x28ccb90>
14002742f:     	jmp	0x1400273aa <.text+0x263aa>
140027434:     	nopw	%cs:(%rax,%rax)
140027440:     	movq	%rdx, 0x10(%rsp)
140027445:     	pushq	%rbp
140027446:     	pushq	%r15
140027448:     	pushq	%r14
14002744a:     	pushq	%r13
14002744c:     	pushq	%r12
14002744e:     	pushq	%rsi
14002744f:     	pushq	%rdi
140027450:     	pushq	%rbx
140027451:     	subq	$0x138, %rsp            # imm = 0x138
140027458:     	leaq	0x80(%rdx), %rbp
14002745f:     	movdqa	%xmm15, 0x90(%rsp)
140027469:     	movaps	%xmm14, 0xa0(%rsp)
140027472:     	movdqa	%xmm13, 0xb0(%rsp)
14002747c:     	movdqa	%xmm12, 0xc0(%rsp)
140027486:     	movaps	%xmm11, 0xd0(%rsp)
14002748f:     	movaps	%xmm10, 0xe0(%rsp)
140027498:     	movaps	%xmm9, 0xf0(%rsp)
1400274a1:     	movaps	%xmm8, 0x100(%rsp)
1400274aa:     	movaps	%xmm7, 0x110(%rsp)
1400274b2:     	movaps	%xmm6, 0x120(%rsp)
1400274ba:     	cmpb	$0x0, 0x79f(%rbp)
1400274c1:     	je	0x140027500 <.text+0x26500>
1400274c3:     	callq	0x140c18130 <.text+0xc17130>
1400274c8:     	movq	%rax, %rsi
1400274cb:     	movq	(%rax), %rax
1400274ce:     	cmpq	$0x1, %rax
1400274d2:     	movq	0x270(%rbp), %rdx
1400274d9:     	jne	0x14002756c <.text+0x2656c>
1400274df:     	addq	$0x8, %rsi
1400274e3:     	cmpq	$0x0, (%rsi)
1400274e7:     	jne	0x140027599 <.text+0x26599>
1400274ed:     	movq	$-0x1, (%rsi)
1400274f4:     	leaq	0x8(%rsi), %rcx
1400274f8:     	callq	0x140c170f0 <.text+0xc160f0>
1400274fd:     	incq	(%rsi)
140027500:     	movaps	0x120(%rsp), %xmm6
140027508:     	movaps	0x110(%rsp), %xmm7
140027510:     	movaps	0x100(%rsp), %xmm8
140027519:     	movaps	0xf0(%rsp), %xmm9
140027522:     	movaps	0xe0(%rsp), %xmm10
14002752b:     	movaps	0xd0(%rsp), %xmm11
140027534:     	movaps	0xc0(%rsp), %xmm12
14002753d:     	movaps	0xb0(%rsp), %xmm13
140027546:     	movaps	0xa0(%rsp), %xmm14
14002754f:     	movaps	0x90(%rsp), %xmm15
140027558:     	addq	$0x138, %rsp            # imm = 0x138
14002755f:     	popq	%rbx
140027560:     	popq	%rdi
140027561:     	popq	%rsi
140027562:     	popq	%r12
140027564:     	popq	%r13
140027566:     	popq	%r14
140027568:     	popq	%r15
14002756a:     	popq	%rbp
14002756b:     	retq
14002756c:     	testq	%rax, %rax
14002756f:     	jne	0x14002758d <.text+0x2658d>
140027571:     	movq	%rsi, %rcx
140027574:     	movq	%rdx, %rsi
140027577:     	xorl	%edx, %edx
140027579:     	callq	0x1428a1640 <.text+0x28a0640>
14002757e:     	movq	%rsi, %rdx
140027581:     	movq	%rax, %rsi
140027584:     	testq	%rax, %rax
140027587:     	jne	0x1400274e3 <.text+0x264e3>
14002758d:     	leaq	0x290b47c(%rip), %rcx   # 0x142932a10
140027594:     	callq	0x1428d80d0 <.text+0x28d70d0>
140027599:     	leaq	0x290b518(%rip), %rcx   # 0x142932ab8
1400275a0:     	callq	0x1428d9250 <.text+0x28d8250>
1400275a5:     	int3
