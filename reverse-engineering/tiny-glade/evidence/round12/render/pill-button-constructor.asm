
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fa0130:     	pushq	%rsi
140fa0131:     	pushq	%rdi
140fa0132:     	pushq	%rbx
140fa0133:     	subq	$0x80, %rsp
140fa013a:     	movaps	%xmm6, 0x70(%rsp)
140fa013f:     	movq	%rdx, %rdi
140fa0142:     	movq	%rcx, %rsi
140fa0145:     	movaps	0xc0(%rsp), %xmm6
140fa014d:     	movq	%r8, 0x50(%rsp)
140fa0152:     	movq	%r9, 0x58(%rsp)
140fa0157:     	leaq	0x50(%rsp), %rcx
140fa015c:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fa0161:     	movq	%rax, 0x20(%rsp)
140fa0166:     	movabsq	$-0x7931e0e99eebebe0, %rax # imm = 0x86CE1F1661141420
140fa0170:     	movq	%rax, 0x28(%rsp)
140fa0175:     	movabsq	$-0x5922082d28be21b3, %rax # imm = 0xA6DDF7D2D741DE4D
140fa017f:     	movq	%rax, 0x30(%rsp)
140fa0184:     	leaq	0x20(%rsp), %rcx
140fa0189:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fa018e:     	movq	0x8(%rdi), %rcx
140fa0192:     	movq	%rcx, 0x20(%rsp)
140fa0197:     	movq	%rax, 0x28(%rsp)
140fa019c:     	leaq	0x20(%rsp), %rcx
140fa01a1:     	callq	0x1405bd910 <.text+0x5bc910>
140fa01a6:     	leaq	0x38(%rsp), %rbx
140fa01ab:     	movq	%rbx, %rcx
140fa01ae:     	movq	%rdi, %rdx
140fa01b1:     	movq	%rax, %r8
140fa01b4:     	callq	0x1406861b0 <.text+0x6851b0>
140fa01b9:     	movq	%rbx, %rcx
140fa01bc:     	xorl	%edx, %edx
140fa01be:     	callq	0x140f9de80 <.text+0xf9ce80>
140fa01c3:     	movups	%xmm6, 0x30(%rax)
140fa01c7:     	movq	0x38(%rsp), %rdi
140fa01cc:     	cmpq	$0x0, 0x1b8(%rdi)
140fa01d4:     	je	0x140fa0496 <.text+0xf9f496>
140fa01da:     	leaq	0x40(%rsp), %rdx
140fa01df:     	leaq	0x1c0(%rdi), %rcx
140fa01e6:     	callq	0x1405b0d70 <.text+0x5afd70>
140fa01eb:     	movq	0x1a0(%rdi), %rcx
140fa01f2:     	movq	0x1a8(%rdi), %rdx
140fa01f9:     	movq	%rdx, %r8
140fa01fc:     	andq	%rax, %r8
140fa01ff:     	shrq	$0x39, %rax
140fa0203:     	movd	%eax, %xmm0
140fa0207:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fa020b:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fa0210:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fa0215:     	leaq	-0xc0(%rcx), %rax
140fa021c:     	xorl	%r9d, %r9d
140fa021f:     	movq	0x40(%rsp), %r10
140fa0224:     	pcmpeqd	%xmm1, %xmm1
140fa0228:     	movdqu	(%rcx,%r8), %xmm2
140fa022e:     	movdqa	%xmm2, %xmm3
140fa0232:     	pcmpeqb	%xmm0, %xmm3
140fa0236:     	pmovmskb	%xmm3, %r11d
140fa023b:     	testl	%r11d, %r11d
140fa023e:     	je	0x140fa0270 <.text+0xf9f270>
140fa0240:     	tzcntl	%r11d, %edi
140fa0245:     	addq	%r8, %rdi
140fa0248:     	andq	%rdx, %rdi
140fa024b:     	negq	%rdi
140fa024e:     	leaq	(%rdi,%rdi,2), %rdi
140fa0252:     	shlq	$0x6, %rdi
140fa0256:     	cmpq	(%rax,%rdi), %r10
140fa025a:     	je	0x140fa0292 <.text+0xf9f292>
140fa025c:     	leal	-0x1(%r11), %edi
140fa0260:     	andw	%r11w, %di
140fa0264:     	movl	%edi, %r11d
140fa0267:     	jne	0x140fa0240 <.text+0xf9f240>
140fa0269:     	nopl	(%rax)
140fa0270:     	pcmpeqb	%xmm1, %xmm2
140fa0274:     	pmovmskb	%xmm2, %r11d
140fa0279:     	testl	%r11d, %r11d
140fa027c:     	jne	0x140fa0496 <.text+0xf9f496>
140fa0282:     	addq	%r9, %r8
140fa0285:     	addq	$0x10, %r8
140fa0289:     	addq	$0x10, %r9
140fa028d:     	andq	%rdx, %r8
140fa0290:     	jmp	0x140fa0228 <.text+0xf9f228>
140fa0292:     	orb	$0x20, -0x1(%rcx,%rdi)
140fa0297:     	movdqu	0x38(%rsp), %xmm0
140fa029d:     	movdqa	%xmm0, 0x20(%rsp)
140fa02a3:     	movq	0x48(%rsp), %rax
140fa02a8:     	movq	%rax, 0x30(%rsp)
140fa02ad:     	movq	0x20(%rsp), %rdi
140fa02b2:     	cmpq	$0x0, 0x1b8(%rdi)
140fa02ba:     	je	0x140fa04cd <.text+0xf9f4cd>
140fa02c0:     	leaq	0x28(%rsp), %rdx
140fa02c5:     	leaq	0x1c0(%rdi), %rcx
140fa02cc:     	callq	0x1405b0d70 <.text+0x5afd70>
140fa02d1:     	movq	0x1a0(%rdi), %rcx
140fa02d8:     	movq	0x1a8(%rdi), %rdx
140fa02df:     	movq	%rdx, %r8
140fa02e2:     	andq	%rax, %r8
140fa02e5:     	shrq	$0x39, %rax
140fa02e9:     	movd	%eax, %xmm0
140fa02ed:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fa02f1:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fa02f6:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fa02fb:     	leaq	-0xc0(%rcx), %rax
140fa0302:     	xorl	%r9d, %r9d
140fa0305:     	movq	0x28(%rsp), %r10
140fa030a:     	pcmpeqd	%xmm1, %xmm1
140fa030e:     	movdqu	(%rcx,%r8), %xmm2
140fa0314:     	movdqa	%xmm2, %xmm3
140fa0318:     	pcmpeqb	%xmm0, %xmm3
140fa031c:     	pmovmskb	%xmm3, %r11d
140fa0321:     	testl	%r11d, %r11d
140fa0324:     	je	0x140fa0350 <.text+0xf9f350>
140fa0326:     	tzcntl	%r11d, %edi
140fa032b:     	addq	%r8, %rdi
140fa032e:     	andq	%rdx, %rdi
140fa0331:     	negq	%rdi
140fa0334:     	leaq	(%rdi,%rdi,2), %rdi
140fa0338:     	shlq	$0x6, %rdi
140fa033c:     	cmpq	(%rax,%rdi), %r10
140fa0340:     	je	0x140fa0372 <.text+0xf9f372>
140fa0342:     	leal	-0x1(%r11), %edi
140fa0346:     	andw	%r11w, %di
140fa034a:     	movl	%edi, %r11d
140fa034d:     	jne	0x140fa0326 <.text+0xf9f326>
140fa034f:     	nop
140fa0350:     	pcmpeqb	%xmm1, %xmm2
140fa0354:     	pmovmskb	%xmm2, %r11d
140fa0359:     	testl	%r11d, %r11d
140fa035c:     	jne	0x140fa04cd <.text+0xf9f4cd>
140fa0362:     	addq	%r9, %r8
140fa0365:     	addq	$0x10, %r8
140fa0369:     	addq	$0x10, %r9
140fa036d:     	andq	%rdx, %r8
140fa0370:     	jmp	0x140fa030e <.text+0xf9f30e>
140fa0372:     	pxor	%xmm0, %xmm0
140fa0376:     	movdqu	%xmm0, -0x84(%rcx,%rdi)
140fa037f:     	movdqu	%xmm0, -0x90(%rcx,%rdi)
140fa0388:     	movdqu	%xmm0, -0xa0(%rcx,%rdi)
140fa0391:     	movq	$0x42a00000, -0x74(%rcx,%rdi) # imm = 0x42A00000
140fa039a:     	movl	$0x42700000, -0x6c(%rcx,%rdi) # imm = 0x42700000
140fa03a2:     	movq	$0x0, -0x68(%rcx,%rdi)
140fa03ab:     	movq	$0x0, -0x63(%rcx,%rdi)
140fa03b4:     	movb	$0x2, -0x5b(%rcx,%rdi)
140fa03b9:     	movw	$0x1, -0x5a(%rcx,%rdi)
140fa03c0:     	movq	0x48(%rsp), %rax
140fa03c5:     	movq	%rax, 0x60(%rsp)
140fa03ca:     	movdqu	0x38(%rsp), %xmm0
140fa03d0:     	movdqa	%xmm0, 0x50(%rsp)
140fa03d6:     	movq	0x50(%rsp), %rdi
140fa03db:     	cmpq	$0x0, 0x1b8(%rdi)
140fa03e3:     	je	0x140fa04d9 <.text+0xf9f4d9>
140fa03e9:     	leaq	0x58(%rsp), %rdx
140fa03ee:     	leaq	0x1c0(%rdi), %rcx
140fa03f5:     	callq	0x1405b0d70 <.text+0x5afd70>
140fa03fa:     	movq	0x1a0(%rdi), %rcx
140fa0401:     	movq	0x1a8(%rdi), %rdx
140fa0408:     	movq	%rdx, %r8
140fa040b:     	andq	%rax, %r8
140fa040e:     	shrq	$0x39, %rax
140fa0412:     	movd	%eax, %xmm0
140fa0416:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fa041a:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fa041f:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fa0424:     	leaq	-0xc0(%rcx), %rax
140fa042b:     	xorl	%r9d, %r9d
140fa042e:     	movq	0x58(%rsp), %r10
140fa0433:     	pcmpeqd	%xmm1, %xmm1
140fa0437:     	movdqu	(%rcx,%r8), %xmm2
140fa043d:     	movdqa	%xmm2, %xmm3
140fa0441:     	pcmpeqb	%xmm0, %xmm3
140fa0445:     	pmovmskb	%xmm3, %r11d
140fa044a:     	testl	%r11d, %r11d
140fa044d:     	je	0x140fa0478 <.text+0xf9f478>
140fa044f:     	tzcntl	%r11d, %edi
140fa0454:     	addq	%r8, %rdi
140fa0457:     	andq	%rdx, %rdi
140fa045a:     	negq	%rdi
140fa045d:     	leaq	(%rdi,%rdi,2), %rdi
140fa0461:     	shlq	$0x6, %rdi
140fa0465:     	cmpq	(%rax,%rdi), %r10
140fa0469:     	je	0x140fa04a2 <.text+0xf9f4a2>
140fa046b:     	leal	-0x1(%r11), %edi
140fa046f:     	andw	%r11w, %di
140fa0473:     	movl	%edi, %r11d
140fa0476:     	jne	0x140fa044f <.text+0xf9f44f>
140fa0478:     	pcmpeqb	%xmm1, %xmm2
140fa047c:     	pmovmskb	%xmm2, %r11d
140fa0481:     	testl	%r11d, %r11d
140fa0484:     	jne	0x140fa04d9 <.text+0xf9f4d9>
140fa0486:     	addq	%r9, %r8
140fa0489:     	addq	$0x10, %r8
140fa048d:     	addq	$0x10, %r9
140fa0491:     	andq	%rdx, %r8
140fa0494:     	jmp	0x140fa0437 <.text+0xf9f437>
140fa0496:     	leaq	0x1a77b23(%rip), %rcx   # 0x142a17fc0
140fa049d:     	callq	0x1428d9310 <.text+0x28d8310>
140fa04a2:     	movw	$0x200, -0x4(%rcx,%rdi) # imm = 0x200
140fa04a9:     	movq	0x48(%rsp), %rax
140fa04ae:     	movq	%rax, 0x10(%rsi)
140fa04b2:     	movups	0x38(%rsp), %xmm0
140fa04b7:     	movups	%xmm0, (%rsi)
140fa04ba:     	movq	%rsi, %rax
140fa04bd:     	movaps	0x70(%rsp), %xmm6
140fa04c2:     	addq	$0x80, %rsp
140fa04c9:     	popq	%rbx
140fa04ca:     	popq	%rdi
140fa04cb:     	popq	%rsi
140fa04cc:     	retq
140fa04cd:     	leaq	0x1a77a8c(%rip), %rcx   # 0x142a17f60
140fa04d4:     	callq	0x1428d9310 <.text+0x28d8310>
140fa04d9:     	leaq	0x1a77b10(%rip), %rcx   # 0x142a17ff0
140fa04e0:     	callq	0x1428d9310 <.text+0x28d8310>
140fa04e5:     	int3
