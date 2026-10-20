
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1406811c0:     	pushq	%rsi
1406811c1:     	pushq	%rdi
1406811c2:     	subq	$0x68, %rsp
1406811c6:     	movq	%rcx, %rsi
1406811c9:     	movq	%r9, 0x28(%rsp)
1406811ce:     	movq	%r8, 0x20(%rsp)
1406811d3:     	leaq	0x30(%rsp), %rcx
1406811d8:     	callq	0x140fa0130 <.text+0xf9f130>
1406811dd:     	movq	0x30(%rsp), %rdi
1406811e2:     	cmpq	$0x0, 0x1b8(%rdi)
1406811ea:     	je	0x1406814b4 <.text+0x6804b4>
1406811f0:     	leaq	0x38(%rsp), %rdx
1406811f5:     	leaq	0x1c0(%rdi), %rcx
1406811fc:     	callq	0x1405b0d70 <.text+0x5afd70>
140681201:     	movq	0x1a0(%rdi), %rcx
140681208:     	movq	0x1a8(%rdi), %rdx
14068120f:     	movq	%rdx, %r8
140681212:     	andq	%rax, %r8
140681215:     	shrq	$0x39, %rax
140681219:     	movd	%eax, %xmm0
14068121d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140681221:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140681226:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068122b:     	leaq	-0xc0(%rcx), %rax
140681232:     	xorl	%r9d, %r9d
140681235:     	movq	0x38(%rsp), %r10
14068123a:     	pcmpeqd	%xmm1, %xmm1
14068123e:     	movdqu	(%rcx,%r8), %xmm2
140681244:     	movdqa	%xmm2, %xmm3
140681248:     	pcmpeqb	%xmm0, %xmm3
14068124c:     	pmovmskb	%xmm3, %r11d
140681251:     	testl	%r11d, %r11d
140681254:     	je	0x140681280 <.text+0x680280>
140681256:     	tzcntl	%r11d, %edi
14068125b:     	addq	%r8, %rdi
14068125e:     	andq	%rdx, %rdi
140681261:     	negq	%rdi
140681264:     	leaq	(%rdi,%rdi,2), %rdi
140681268:     	shlq	$0x6, %rdi
14068126c:     	cmpq	(%rax,%rdi), %r10
140681270:     	je	0x1406812a2 <.text+0x6802a2>
140681272:     	leal	-0x1(%r11), %edi
140681276:     	andw	%r11w, %di
14068127a:     	movl	%edi, %r11d
14068127d:     	jne	0x140681256 <.text+0x680256>
14068127f:     	nop
140681280:     	pcmpeqb	%xmm1, %xmm2
140681284:     	pmovmskb	%xmm2, %r11d
140681289:     	testl	%r11d, %r11d
14068128c:     	jne	0x1406814b4 <.text+0x6804b4>
140681292:     	addq	%r9, %r8
140681295:     	addq	$0x10, %r8
140681299:     	addq	$0x10, %r9
14068129d:     	andq	%rdx, %r8
1406812a0:     	jmp	0x14068123e <.text+0x68023e>
1406812a2:     	movw	$0xd00, -0x4(%rcx,%rdi) # imm = 0xD00
1406812a9:     	movq	0x40(%rsp), %rax
1406812ae:     	movq	%rax, 0x60(%rsp)
1406812b3:     	movdqu	0x30(%rsp), %xmm0
1406812b9:     	movdqa	%xmm0, 0x50(%rsp)
1406812bf:     	movq	0x50(%rsp), %rdi
1406812c4:     	cmpq	$0x0, 0x1b8(%rdi)
1406812cc:     	je	0x1406814e0 <.text+0x6804e0>
1406812d2:     	leaq	0x58(%rsp), %rdx
1406812d7:     	leaq	0x1c0(%rdi), %rcx
1406812de:     	callq	0x1405b0d70 <.text+0x5afd70>
1406812e3:     	movq	0x1a0(%rdi), %rcx
1406812ea:     	movq	0x1a8(%rdi), %rdx
1406812f1:     	movq	%rdx, %r8
1406812f4:     	andq	%rax, %r8
1406812f7:     	shrq	$0x39, %rax
1406812fb:     	movd	%eax, %xmm0
1406812ff:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140681303:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140681308:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068130d:     	leaq	-0xc0(%rcx), %rax
140681314:     	xorl	%r9d, %r9d
140681317:     	movq	0x58(%rsp), %r10
14068131c:     	pcmpeqd	%xmm1, %xmm1
140681320:     	movdqu	(%rcx,%r8), %xmm2
140681326:     	movdqa	%xmm2, %xmm3
14068132a:     	pcmpeqb	%xmm0, %xmm3
14068132e:     	pmovmskb	%xmm3, %r11d
140681333:     	testl	%r11d, %r11d
140681336:     	je	0x140681370 <.text+0x680370>
140681338:     	tzcntl	%r11d, %edi
14068133d:     	addq	%r8, %rdi
140681340:     	andq	%rdx, %rdi
140681343:     	negq	%rdi
140681346:     	leaq	(%rdi,%rdi,2), %rdi
14068134a:     	shlq	$0x6, %rdi
14068134e:     	cmpq	(%rax,%rdi), %r10
140681352:     	je	0x140681392 <.text+0x680392>
140681354:     	leal	-0x1(%r11), %edi
140681358:     	andw	%r11w, %di
14068135c:     	movl	%edi, %r11d
14068135f:     	jne	0x140681338 <.text+0x680338>
140681361:     	nopw	%cs:(%rax,%rax)
140681370:     	pcmpeqb	%xmm1, %xmm2
140681374:     	pmovmskb	%xmm2, %r11d
140681379:     	testl	%r11d, %r11d
14068137c:     	jne	0x1406814e0 <.text+0x6804e0>
140681382:     	addq	%r9, %r8
140681385:     	addq	$0x10, %r8
140681389:     	addq	$0x10, %r9
14068138d:     	andq	%rdx, %r8
140681390:     	jmp	0x140681320 <.text+0x680320>
140681392:     	pxor	%xmm0, %xmm0
140681396:     	movdqu	%xmm0, -0x84(%rcx,%rdi)
14068139f:     	movdqu	%xmm0, -0x90(%rcx,%rdi)
1406813a8:     	movdqu	%xmm0, -0xa0(%rcx,%rdi)
1406813b1:     	movq	$0x42480000, -0x74(%rcx,%rdi) # imm = 0x42480000
1406813ba:     	movl	$0x42480000, -0x6c(%rcx,%rdi) # imm = 0x42480000
1406813c2:     	movq	$0x0, -0x68(%rcx,%rdi)
1406813cb:     	movq	$0x0, -0x63(%rcx,%rdi)
1406813d4:     	movb	$0x2, -0x5b(%rcx,%rdi)
1406813d9:     	movw	$0x1, -0x5a(%rcx,%rdi)
1406813e0:     	movq	0x60(%rsp), %rax
1406813e5:     	movq	%rax, 0x40(%rsp)
1406813ea:     	movaps	0x50(%rsp), %xmm0
1406813ef:     	movaps	%xmm0, 0x30(%rsp)
1406813f4:     	movq	0x30(%rsp), %rdi
1406813f9:     	cmpq	$0x0, 0x1b8(%rdi)
140681401:     	je	0x1406814b4 <.text+0x6804b4>
140681407:     	leaq	0x38(%rsp), %rdx
14068140c:     	leaq	0x1c0(%rdi), %rcx
140681413:     	callq	0x1405b0d70 <.text+0x5afd70>
140681418:     	movq	0x1a0(%rdi), %rcx
14068141f:     	movq	0x1a8(%rdi), %rdx
140681426:     	movq	%rdx, %r8
140681429:     	andq	%rax, %r8
14068142c:     	shrq	$0x39, %rax
140681430:     	movd	%eax, %xmm0
140681434:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140681438:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14068143d:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140681442:     	leaq	-0xc0(%rcx), %rax
140681449:     	xorl	%r9d, %r9d
14068144c:     	movq	0x38(%rsp), %r10
140681451:     	pcmpeqd	%xmm1, %xmm1
140681455:     	movdqu	(%rcx,%r8), %xmm2
14068145b:     	movdqa	%xmm2, %xmm3
14068145f:     	pcmpeqb	%xmm0, %xmm3
140681463:     	pmovmskb	%xmm3, %r11d
140681468:     	testl	%r11d, %r11d
14068146b:     	je	0x140681496 <.text+0x680496>
14068146d:     	tzcntl	%r11d, %edi
140681472:     	addq	%r8, %rdi
140681475:     	andq	%rdx, %rdi
140681478:     	negq	%rdi
14068147b:     	leaq	(%rdi,%rdi,2), %rdi
14068147f:     	shlq	$0x6, %rdi
140681483:     	cmpq	(%rax,%rdi), %r10
140681487:     	je	0x1406814c0 <.text+0x6804c0>
140681489:     	leal	-0x1(%r11), %edi
14068148d:     	andw	%r11w, %di
140681491:     	movl	%edi, %r11d
140681494:     	jne	0x14068146d <.text+0x68046d>
140681496:     	pcmpeqb	%xmm1, %xmm2
14068149a:     	pmovmskb	%xmm2, %r11d
14068149f:     	testl	%r11d, %r11d
1406814a2:     	jne	0x1406814b4 <.text+0x6804b4>
1406814a4:     	addq	%r9, %r8
1406814a7:     	addq	$0x10, %r8
1406814ab:     	addq	$0x10, %r9
1406814af:     	andq	%rdx, %r8
1406814b2:     	jmp	0x140681455 <.text+0x680455>
1406814b4:     	leaq	0x2396b35(%rip), %rcx   # 0x142a17ff0
1406814bb:     	callq	0x1428d9310 <.text+0x28d8310>
1406814c0:     	andb	$-0x21, -0x1(%rcx,%rdi)
1406814c5:     	movq	0x40(%rsp), %rax
1406814ca:     	movq	%rax, 0x10(%rsi)
1406814ce:     	movaps	0x30(%rsp), %xmm0
1406814d3:     	movups	%xmm0, (%rsi)
1406814d6:     	movq	%rsi, %rax
1406814d9:     	addq	$0x68, %rsp
1406814dd:     	popq	%rdi
1406814de:     	popq	%rsi
1406814df:     	retq
1406814e0:     	leaq	0x2396a79(%rip), %rcx   # 0x142a17f60
1406814e7:     	callq	0x1428d9310 <.text+0x28d8310>
1406814ec:     	int3
