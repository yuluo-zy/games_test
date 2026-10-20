
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140929160:     	pushq	%rbp
140929161:     	pushq	%r15
140929163:     	pushq	%r14
140929165:     	pushq	%r13
140929167:     	pushq	%r12
140929169:     	pushq	%rsi
14092916a:     	pushq	%rdi
14092916b:     	pushq	%rbx
14092916c:     	subq	$0x118, %rsp            # imm = 0x118
140929173:     	leaq	0x80(%rsp), %rbp
14092917b:     	movdqa	%xmm6, 0x80(%rbp)
140929183:     	movq	$-0x2, 0x78(%rbp)
14092918b:     	movq	%r8, %rbx
14092918e:     	movq	%rdx, %r14
140929191:     	movq	%rcx, %rdi
140929194:     	callq	0x140c16a00 <.text+0xc15a00>
140929199:     	testb	%al, %al
14092919b:     	movb	%al, 0x77(%rbp)
14092919e:     	je	0x1409291ff <.text+0x9281ff>
1409291a0:     	leaq	0x2185461(%rip), %r15   # 0x142aae608
1409291a7:     	movl	$0x5a, %esi
1409291ac:     	movl	$0x5a, %edx
1409291b1:     	movq	%r15, %rcx
1409291b4:     	callq	0x140923920 <.text+0x922920>
1409291b9:     	cmpq	$0x1, %rax
1409291bd:     	jne	0x140929274 <.text+0x928274>
1409291c3:     	movq	%rdx, %r9
1409291c6:     	testq	%rdx, %rdx
1409291c9:     	je	0x140929208 <.text+0x928208>
1409291cb:     	cmpq	$0x5a, %r9
1409291cf:     	jae	0x140929206 <.text+0x928206>
1409291d1:     	leaq	0x2185430(%rip), %rax   # 0x142aae608
1409291d8:     	cmpb	$-0x41, (%r9,%rax)
1409291dd:     	jg	0x140929208 <.text+0x928208>
1409291df:     	leaq	0x2185552(%rip), %rax   # 0x142aae738
1409291e6:     	movq	%rax, 0x20(%rsp)
1409291eb:     	leaq	0x2185416(%rip), %rcx   # 0x142aae608
1409291f2:     	movl	$0x5a, %edx
1409291f7:     	xorl	%r8d, %r8d
1409291fa:     	callq	0x1428d9940 <.text+0x28d8940>
1409291ff:     	xorl	%eax, %eax
140929201:     	jmp	0x1409293da <.text+0x9283da>
140929206:     	jne	0x1409291df <.text+0x9281df>
140929208:     	leaq	0x21853f9(%rip), %r15   # 0x142aae608
14092920f:     	movq	%r15, %rcx
140929212:     	movq	%r9, %rdx
140929215:     	callq	0x140923920 <.text+0x922920>
14092921a:     	cmpq	$0x1, %rax
14092921e:     	jne	0x140929274 <.text+0x928274>
140929220:     	movq	%rdx, %r8
140929223:     	addq	$0x2, %r8
140929227:     	je	0x140929262 <.text+0x928262>
140929229:     	cmpq	$0x5a, %r8
14092922d:     	jae	0x140929260 <.text+0x928260>
14092922f:     	leaq	0x21853d2(%rip), %rax   # 0x142aae608
140929236:     	cmpb	$-0x41, (%r8,%rax)
14092923b:     	jg	0x140929262 <.text+0x928262>
14092923d:     	leaq	0x218550c(%rip), %rax   # 0x142aae750
140929244:     	movq	%rax, 0x20(%rsp)
140929249:     	leaq	0x21853b8(%rip), %rcx   # 0x142aae608
140929250:     	movl	$0x5a, %edx
140929255:     	movl	$0x5a, %r9d
14092925b:     	callq	0x1428d9940 <.text+0x28d8940>
140929260:     	jne	0x14092923d <.text+0x92823d>
140929262:     	movl	$0x58, %esi
140929267:     	subq	%rdx, %rsi
14092926a:     	leaq	0x2185397(%rip), %r15   # 0x142aae608
140929271:     	addq	%r8, %r15
140929274:     	leaq	0x218630b(%rip), %rax   # 0x142aaf586
14092927b:     	leaq	0x21862ba(%rip), %r12   # 0x142aaf53c
140929282:     	nopw	%cs:(%rax,%rax)
140929290:     	movsbl	-0x1(%rax), %ecx
140929294:     	testl	%ecx, %ecx
140929296:     	js	0x1409292b0 <.text+0x9282b0>
140929298:     	decq	%rax
14092929b:     	cmpl	$0x5c, %ecx
14092929e:     	jne	0x140929309 <.text+0x928309>
1409292a0:     	jmp	0x14092931f <.text+0x92831f>
1409292a2:     	nopw	%cs:(%rax,%rax)
1409292b0:     	movzbl	-0x2(%rax), %edx
1409292b4:     	cmpb	$-0x40, %dl
1409292b7:     	jge	0x1409292de <.text+0x9282de>
1409292b9:     	movzbl	-0x3(%rax), %r8d
1409292be:     	cmpb	$-0x40, %r8b
1409292c2:     	jge	0x1409292e7 <.text+0x9282e7>
1409292c4:     	movzbl	-0x4(%rax), %r9d
1409292c9:     	addq	$-0x4, %rax
1409292cd:     	andl	$0x7, %r9d
1409292d1:     	shll	$0x6, %r9d
1409292d5:     	andl	$0x3f, %r8d
1409292d9:     	orl	%r9d, %r8d
1409292dc:     	jmp	0x1409292ef <.text+0x9282ef>
1409292de:     	addq	$-0x2, %rax
1409292e2:     	andl	$0x1f, %edx
1409292e5:     	jmp	0x1409292f9 <.text+0x9282f9>
1409292e7:     	addq	$-0x3, %rax
1409292eb:     	andl	$0xf, %r8d
1409292ef:     	shll	$0x6, %r8d
1409292f3:     	andl	$0x3f, %edx
1409292f6:     	orl	%r8d, %edx
1409292f9:     	shll	$0x6, %edx
1409292fc:     	andb	$0x3f, %cl
1409292ff:     	movzbl	%cl, %ecx
140929302:     	orl	%edx, %ecx
140929304:     	cmpl	$0x5c, %ecx
140929307:     	je	0x14092931f <.text+0x92831f>
140929309:     	cmpl	$0x2f, %ecx
14092930c:     	je	0x14092931f <.text+0x92831f>
14092930e:     	cmpq	%r12, %rax
140929311:     	jne	0x140929290 <.text+0x928290>
140929317:     	movl	$0x4a, %r13d
14092931d:     	jmp	0x14092936f <.text+0x92836f>
14092931f:     	leaq	0x2186216(%rip), %r12   # 0x142aaf53c
140929326:     	subq	%r12, %rax
140929329:     	movq	%rax, %r8
14092932c:     	incq	%r8
14092932f:     	je	0x140929363 <.text+0x928363>
140929331:     	cmpq	$0x4a, %r8
140929335:     	jae	0x140929361 <.text+0x928361>
140929337:     	cmpb	$-0x41, (%r8,%r12)
14092933c:     	jg	0x140929363 <.text+0x928363>
14092933e:     	leaq	0x21853d3(%rip), %rax   # 0x142aae718
140929345:     	movq	%rax, 0x20(%rsp)
14092934a:     	leaq	0x21861eb(%rip), %rcx   # 0x142aaf53c
140929351:     	movl	$0x4a, %edx
140929356:     	movl	$0x4a, %r9d
14092935c:     	callq	0x1428d9940 <.text+0x28d8940>
140929361:     	jne	0x14092933e <.text+0x92833e>
140929363:     	movl	$0x49, %r13d
140929369:     	subq	%rax, %r13
14092936c:     	addq	%r8, %r12
14092936f:     	callq	0x140c18130 <.text+0xc17130>
140929374:     	movq	%rax, %rcx
140929377:     	movq	(%rax), %rax
14092937a:     	cmpq	$0x1, %rax
14092937e:     	jne	0x14092990b <.text+0x92890b>
140929384:     	addq	$0x8, %rcx
140929388:     	cmpq	$0x0, (%rcx)
14092938c:     	jne	0x140929945 <.text+0x928945>
140929392:     	movq	$-0x1, (%rcx)
140929399:     	movq	%rcx, 0x60(%rbp)
14092939d:     	addq	$0x8, %rcx
1409293a1:     	movq	%r13, 0x20(%rsp)
1409293a6:     	movq	$0x0, 0x30(%rsp)
1409293af:     	movq	$0x1, 0x28(%rsp)
1409293b8:     	movq	%r15, %rdx
1409293bb:     	movq	%rsi, %r8
1409293be:     	movq	%r12, %r9
1409293c1:     	callq	0x140c16d40 <.text+0xc15d40>
1409293c6:     	movq	0x60(%rbp), %rcx
1409293ca:     	incq	(%rcx)
1409293cd:     	movq	%rax, 0x30(%rbp)
1409293d1:     	movq	%rax, -0x28(%rbp)
1409293d5:     	movl	$0x1, %eax
1409293da:     	movq	%rax, -0x30(%rbp)
1409293de:     	xorl	%ecx, %ecx
1409293e0:     	callq	0x1400d9b20 <.text+0xd8b20>
1409293e5:     	testq	%rax, %rax
1409293e8:     	je	0x140929951 <.text+0x928951>
1409293ee:     	movups	(%rax), %xmm0
1409293f1:     	incq	(%rax)
1409293f4:     	movaps	%xmm0, (%rbp)
1409293f8:     	movups	0x2342b11(%rip), %xmm0  # 0x142c6bf10
1409293ff:     	movaps	%xmm0, -0x10(%rbp)
140929403:     	movdqu	0x2342af5(%rip), %xmm0  # 0x142c6bf00
14092940b:     	movdqa	%xmm0, -0x20(%rbp)
140929410:     	movq	(%rdi), %rsi
140929413:     	cmpb	$0x0, 0x18(%rsi)
140929417:     	je	0x140929472 <.text+0x928472>
140929419:     	movq	0x20(%rsi), %rax
14092941d:     	movq	0x18(%rax), %r12
140929421:     	movq	0x20(%rax), %rax
140929425:     	leaq	(%rax,%rax,4), %rax
140929429:     	leaq	(%r12,%rax,8), %r13
14092942d:     	leaq	-0x48(%rbp), %rdi
140929431:     	leaq	-0x20(%rbp), %r15
140929435:     	nopw	%cs:(%rax,%rax)
140929440:     	cmpq	%r13, %r12
140929443:     	je	0x140929472 <.text+0x928472>
140929445:     	movq	0x38(%rsi), %rax
140929449:     	movdqu	0x8(%r12), %xmm0
140929450:     	movq	%rax, -0x48(%rbp)
140929454:     	movdqu	%xmm0, -0x40(%rbp)
140929459:     	movq	%rdi, %rcx
14092945c:     	callq	0x14145a710 <.text+0x1459710>
140929461:     	addq	$0x28, %r12
140929465:     	movq	%r15, %rcx
140929468:     	movq	%rax, %rdx
14092946b:     	callq	0x1402ca2f0 <.text+0x2c92f0>
140929470:     	jmp	0x140929440 <.text+0x928440>
140929472:     	movq	(%r14), %rcx
140929475:     	movq	0xf8(%rcx), %r12
14092947c:     	movq	0x100(%rcx), %rax
140929483:     	movq	0x8(%rbx), %rdx
140929487:     	testq	%rdx, %rdx
14092948a:     	jne	0x140929490 <.text+0x928490>
14092948c:     	movq	0x10(%rbx), %rdx
140929490:     	movq	%rdx, 0x28(%rbp)
140929494:     	movq	0x8(%r14), %r13
140929498:     	leaq	(%r12,%rax,4), %rsi
14092949c:     	movq	%rcx, 0x50(%rbp)
1409294a0:     	movzbl	0x108(%rcx), %eax
1409294a7:     	movb	%al, 0x60(%rbp)
1409294aa:     	movl	$0x8, %eax
1409294af:     	movq	%rax, 0x40(%rbp)
1409294b3:     	pcmpeqd	%xmm6, %xmm6
1409294b7:     	movq	%rax, 0x48(%rbp)
1409294bb:     	xorl	%ebx, %ebx
1409294bd:     	xorl	%r15d, %r15d
1409294c0:     	movq	$0x0, 0x68(%rbp)
1409294c8:     	movq	$0x0, 0x58(%rbp)
1409294d0:     	movl	$0x0, 0x70(%rbp)
1409294d7:     	jmp	0x140929560 <.text+0x928560>
1409294dc:     	nopl	(%rax)
1409294e0:     	movdqu	(%rdx,%r8), %xmm1
1409294e6:     	movdqa	%xmm1, %xmm2
1409294ea:     	pcmpeqb	%xmm0, %xmm2
1409294ee:     	pmovmskb	%xmm2, %r9d
1409294f3:     	testl	%r9d, %r9d
1409294f6:     	je	0x140929530 <.text+0x928530>
1409294f8:     	movq	(%r14), %r10
1409294fb:     	movq	%rcx, %rax
1409294fe:     	tzcntl	%r9d, %r11d
140929503:     	addq	%r8, %r11
140929506:     	andq	0x38(%rbp), %r11
14092950a:     	shlq	$0x3, %r11
14092950e:     	movq	%rdx, %rcx
140929511:     	subq	%r11, %rcx
140929514:     	cmpq	-0x8(%rcx), %r10
140929518:     	je	0x140929560 <.text+0x928560>
14092951a:     	leal	-0x1(%r9), %ecx
14092951e:     	andw	%r9w, %cx
140929522:     	movl	%ecx, %r9d
140929525:     	movq	%rax, %rcx
140929528:     	jne	0x1409294fb <.text+0x9284fb>
14092952a:     	nopw	(%rax,%rax)
140929530:     	pcmpeqb	%xmm6, %xmm1
140929534:     	pmovmskb	%xmm1, %eax
140929538:     	testl	%eax, %eax
14092953a:     	jne	0x140929800 <.text+0x928800>
140929540:     	addq	%rcx, %r8
140929543:     	addq	$0x10, %r8
140929547:     	addq	$0x10, %rcx
14092954b:     	andq	0x38(%rbp), %r8
14092954f:     	jmp	0x1409294e0 <.text+0x9284e0>
140929551:     	nopw	%cs:(%rax,%rax)
140929560:     	cmpb	$0x0, 0x60(%rbp)
140929564:     	je	0x140929600 <.text+0x928600>
14092956a:     	cmpq	%rbx, %r15
14092956d:     	jne	0x140929750 <.text+0x928750>
140929573:     	nopw	%cs:(%rax,%rax)
140929580:     	cmpq	%rsi, %r12
140929583:     	je	0x140929880 <.text+0x928880>
140929589:     	movl	(%r12), %ecx
14092958d:     	addq	$0x4, %r12
140929591:     	movq	0x1a8(%r13), %rax
140929598:     	leaq	(%rcx,%rcx,8), %rcx
14092959c:     	movq	0x10(%rax,%rcx,8), %rbx
1409295a1:     	testq	%rbx, %rbx
1409295a4:     	je	0x140929580 <.text+0x928580>
1409295a6:     	leaq	(%rax,%rcx,8), %rax
1409295aa:     	movq	0x50(%rbp), %r9
1409295ae:     	movq	0x110(%r9), %rdx
1409295b5:     	movq	0x40(%rax), %rcx
1409295b9:     	cmpq	%rdx, %rcx
1409295bc:     	jbe	0x1409296a1 <.text+0x9286a1>
1409295c2:     	movq	0x38(%rax), %r8
1409295c6:     	movq	(%r8,%rdx,8), %rdx
1409295ca:     	testq	%rdx, %rdx
1409295cd:     	je	0x1409296a1 <.text+0x9286a1>
1409295d3:     	movq	0x18(%rax), %r8
1409295d7:     	notq	%rdx
1409295da:     	leaq	(%rdx,%rdx,2), %rdx
1409295de:     	shlq	$0x4, %rdx
1409295e2:     	movq	0x10(%r8,%rdx), %rdx
1409295e7:     	movq	%rdx, 0x68(%rbp)
1409295eb:     	movq	0x118(%r9), %rdx
1409295f2:     	cmpq	%rdx, %rcx
1409295f5:     	ja	0x1409296b9 <.text+0x9286b9>
1409295fb:     	jmp	0x14092973e <.text+0x92873e>
140929600:     	cmpq	%rbx, %r15
140929603:     	jne	0x140929779 <.text+0x928779>
140929609:     	nopl	(%rax)
140929610:     	cmpq	%rsi, %r12
140929613:     	je	0x140929880 <.text+0x928880>
140929619:     	movl	(%r12), %ecx
14092961d:     	addq	$0x4, %r12
140929621:     	movq	0x100(%r13), %rax
140929628:     	leaq	(%rcx,%rcx,4), %rcx
14092962c:     	shlq	$0x5, %rcx
140929630:     	movq	0x58(%rax,%rcx), %rbx
140929635:     	testq	%rbx, %rbx
140929638:     	je	0x140929610 <.text+0x928610>
14092963a:     	addq	%rcx, %rax
14092963d:     	movl	0x94(%rax), %ecx
140929643:     	movq	0x1a8(%r13), %rdx
14092964a:     	leaq	(%rcx,%rcx,8), %r9
14092964e:     	leaq	(%rdx,%r9,8), %rcx
140929652:     	movq	0x50(%rbp), %r10
140929656:     	movq	0x110(%r10), %r8
14092965d:     	movq	0x40(%rdx,%r9,8), %rdx
140929662:     	cmpq	%r8, %rdx
140929665:     	jbe	0x1409296e5 <.text+0x9286e5>
140929667:     	movq	0x38(%rcx), %r9
14092966b:     	movq	(%r9,%r8,8), %r8
14092966f:     	testq	%r8, %r8
140929672:     	je	0x1409296e5 <.text+0x9286e5>
140929674:     	movq	0x18(%rcx), %r9
140929678:     	notq	%r8
14092967b:     	leaq	(%r8,%r8,2), %r8
14092967f:     	shlq	$0x4, %r8
140929683:     	movq	0x10(%r9,%r8), %r8
140929688:     	movq	%r8, 0x68(%rbp)
14092968c:     	movq	0x118(%r10), %r8
140929693:     	cmpq	%r8, 0x88(%rax)
14092969a:     	ja	0x1409296fd <.text+0x9286fd>
14092969c:     	jmp	0x14092975d <.text+0x92875d>
1409296a1:     	movq	$0x0, 0x68(%rbp)
1409296a9:     	movq	0x118(%r9), %rdx
1409296b0:     	cmpq	%rdx, %rcx
1409296b3:     	jbe	0x14092973e <.text+0x92873e>
1409296b9:     	movq	0x38(%rax), %rcx
1409296bd:     	movq	(%rcx,%rdx,8), %rcx
1409296c1:     	testq	%rcx, %rcx
1409296c4:     	je	0x14092973e <.text+0x92873e>
1409296c6:     	movq	0x18(%rax), %rdx
1409296ca:     	notq	%rcx
1409296cd:     	leaq	(%rcx,%rcx,2), %rcx
1409296d1:     	shlq	$0x4, %rcx
1409296d5:     	movq	0x10(%rdx,%rcx), %rcx
1409296da:     	movq	%rcx, 0x58(%rbp)
1409296de:     	movb	$0x1, %cl
1409296e0:     	movl	%ecx, 0x70(%rbp)
1409296e3:     	jmp	0x140929745 <.text+0x928745>
1409296e5:     	movq	$0x0, 0x68(%rbp)
1409296ed:     	movq	0x118(%r10), %r8
1409296f4:     	cmpq	%r8, 0x88(%rax)
1409296fb:     	jbe	0x14092975d <.text+0x92875d>
1409296fd:     	movq	0x80(%rax), %r9
140929704:     	cmpq	$0x0, (%r9,%r8,8)
140929709:     	je	0x14092975d <.text+0x92875d>
14092970b:     	movb	$0x1, %r9b
14092970e:     	movl	%r9d, 0x70(%rbp)
140929712:     	cmpq	%r8, %rdx
140929715:     	jbe	0x140929766 <.text+0x928766>
140929717:     	movq	0x38(%rcx), %rdx
14092971b:     	movq	(%rdx,%r8,8), %rdx
14092971f:     	testq	%rdx, %rdx
140929722:     	je	0x140929766 <.text+0x928766>
140929724:     	movq	0x18(%rcx), %rcx
140929728:     	notq	%rdx
14092972b:     	leaq	(%rdx,%rdx,2), %rdx
14092972f:     	shlq	$0x4, %rdx
140929733:     	movq	0x10(%rcx,%rdx), %rcx
140929738:     	movq	%rcx, 0x58(%rbp)
14092973c:     	jmp	0x14092976e <.text+0x92876e>
14092973e:     	movl	$0x0, 0x70(%rbp)
140929745:     	movq	0x8(%rax), %rax
140929749:     	movq	%rax, 0x40(%rbp)
14092974d:     	xorl	%r15d, %r15d
140929750:     	movq	0x40(%rbp), %rax
140929754:     	leaq	(%rax,%r15,8), %rax
140929758:     	movl	%r15d, %r14d
14092975b:     	jmp	0x14092978d <.text+0x92878d>
14092975d:     	movl	$0x0, 0x70(%rbp)
140929764:     	jmp	0x14092976e <.text+0x92876e>
140929766:     	movq	$0x0, 0x58(%rbp)
14092976e:     	movq	0x50(%rax), %rax
140929772:     	movq	%rax, 0x48(%rbp)
140929776:     	xorl	%r15d, %r15d
140929779:     	movq	%r15, %rcx
14092977c:     	shlq	$0x4, %rcx
140929780:     	movq	0x48(%rbp), %rdx
140929784:     	leaq	(%rdx,%rcx), %rax
140929788:     	movl	0x8(%rdx,%rcx), %r14d
14092978d:     	movq	0x58(%rbp), %rcx
140929791:     	leaq	(%rcx,%r14,8), %rdi
140929795:     	shlq	$0x4, %r14
140929799:     	addq	0x68(%rbp), %r14
14092979d:     	testb	$0x1, 0x70(%rbp)
1409297a1:     	movl	$0x0, %ecx
1409297a6:     	cmoveq	%rcx, %rdi
1409297aa:     	movq	(%rax), %rax
1409297ad:     	movq	%rax, 0x20(%rbp)
1409297b1:     	incq	%r15
1409297b4:     	cmpq	$0x0, -0x8(%rbp)
1409297b9:     	je	0x140929800 <.text+0x928800>
1409297bb:     	leaq	(%rbp), %rcx
1409297bf:     	movq	%r14, %rdx
1409297c2:     	callq	0x1405b0d70 <.text+0x5afd70>
1409297c7:     	movq	-0x20(%rbp), %rdx
1409297cb:     	movq	-0x18(%rbp), %r8
1409297cf:     	movq	%r8, 0x38(%rbp)
1409297d3:     	andq	%rax, %r8
1409297d6:     	shrq	$0x39, %rax
1409297da:     	movd	%eax, %xmm0
1409297de:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409297e2:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409297e7:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409297ec:     	xorl	%ecx, %ecx
1409297ee:     	jmp	0x1409294e0 <.text+0x9284e0>
1409297f3:     	nopw	%cs:(%rax,%rax)
140929800:     	testq	%rdi, %rdi
140929803:     	je	0x140929813 <.text+0x928813>
140929805:     	movss	0x4(%rdi), %xmm0
14092980a:     	ucomiss	(%rdi), %xmm0
14092980d:     	ja	0x140929560 <.text+0x928560>
140929813:     	movq	0x28(%rbp), %rdi
140929817:     	movq	(%rdi), %rax
14092981a:     	movq	0x10(%rdi), %r14
14092981e:     	subq	%r14, %rax
140929821:     	cmpq	$0x1f, %rax
140929825:     	jbe	0x14092985e <.text+0x92885e>
140929827:     	movq	0x8(%rdi), %rax
14092982b:     	leaq	0x9b34fe(%rip), %rcx    # 0x1412dcd30 <.text+0x12dbd30>
140929832:     	movq	%rcx, (%rax,%r14)
140929836:     	leaq	0xc3b4f3(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14092983d:     	movq	%rcx, 0x8(%rax,%r14)
140929842:     	movq	0x20(%rbp), %rcx
140929846:     	movq	%rcx, 0x10(%rax,%r14)
14092984b:     	movb	$0x0, 0x18(%rax,%r14)
140929851:     	addq	$0x20, %r14
140929855:     	movq	%r14, 0x10(%rdi)
140929859:     	jmp	0x140929560 <.text+0x928560>
14092985e:     	movq	$0x1, 0x20(%rsp)
140929867:     	movl	$0x20, %r8d
14092986d:     	movl	$0x1, %r9d
140929873:     	movq	%rdi, %rcx
140929876:     	movq	%r14, %rdx
140929879:     	callq	0x1428a9a60 <.text+0x28a8a60>
14092987e:     	jmp	0x140929827 <.text+0x928827>
140929880:     	movq	-0x18(%rbp), %rdx
140929884:     	testq	%rdx, %rdx
140929887:     	je	0x1409298b0 <.text+0x9288b0>
140929889:     	leaq	0x17(,%rdx,8), %rax
140929891:     	andq	$-0x10, %rax
140929895:     	addq	%rax, %rdx
140929898:     	addq	$0x11, %rdx
14092989c:     	je	0x1409298b0 <.text+0x9288b0>
14092989e:     	movq	-0x20(%rbp), %rcx
1409298a2:     	subq	%rax, %rcx
1409298a5:     	movl	$0x10, %r8d
1409298ab:     	callq	0x140613c20 <.text+0x612c20>
1409298b0:     	cmpb	$0x0, 0x77(%rbp)
1409298b4:     	je	0x1409298f0 <.text+0x9288f0>
1409298b6:     	callq	0x140c18130 <.text+0xc17130>
1409298bb:     	movq	%rax, %rcx
1409298be:     	movq	(%rax), %rax
1409298c1:     	cmpq	$0x1, %rax
1409298c5:     	jne	0x140929925 <.text+0x928925>
1409298c7:     	addq	$0x8, %rcx
1409298cb:     	cmpq	$0x0, (%rcx)
1409298cf:     	jne	0x140929945 <.text+0x928945>
1409298d1:     	movq	$-0x1, (%rcx)
1409298d8:     	movq	%rcx, 0x60(%rbp)
1409298dc:     	addq	$0x8, %rcx
1409298e0:     	movq	0x30(%rbp), %rdx
1409298e4:     	callq	0x140c170f0 <.text+0xc160f0>
1409298e9:     	movq	0x60(%rbp), %rax
1409298ed:     	incq	(%rax)
1409298f0:     	movaps	0x80(%rbp), %xmm6
1409298f7:     	addq	$0x118, %rsp            # imm = 0x118
1409298fe:     	popq	%rbx
1409298ff:     	popq	%rdi
140929900:     	popq	%rsi
140929901:     	popq	%r12
140929903:     	popq	%r13
140929905:     	popq	%r14
140929907:     	popq	%r15
140929909:     	popq	%rbp
14092990a:     	retq
14092990b:     	testq	%rax, %rax
14092990e:     	jne	0x140929939 <.text+0x928939>
140929910:     	xorl	%edx, %edx
140929912:     	callq	0x1428f87d0 <.text+0x28f77d0>
140929917:     	movq	%rax, %rcx
14092991a:     	testq	%rax, %rax
14092991d:     	jne	0x140929388 <.text+0x928388>
140929923:     	jmp	0x140929939 <.text+0x928939>
140929925:     	testq	%rax, %rax
140929928:     	jne	0x140929939 <.text+0x928939>
14092992a:     	xorl	%edx, %edx
14092992c:     	callq	0x1428f87d0 <.text+0x28f77d0>
140929931:     	movq	%rax, %rcx
140929934:     	testq	%rax, %rax
140929937:     	jne	0x1409298cb <.text+0x9288cb>
140929939:     	leaq	0x2311160(%rip), %rcx   # 0x142c3aaa0
140929940:     	callq	0x1428d80d0 <.text+0x28d70d0>
140929945:     	leaq	0x2311224(%rip), %rcx   # 0x142c3ab70
14092994c:     	callq	0x1428d9250 <.text+0x28d8250>
140929951:     	leaq	0x2311148(%rip), %rcx   # 0x142c3aaa0
140929958:     	callq	0x1428d80d0 <.text+0x28d70d0>
14092995d:     	ud2
