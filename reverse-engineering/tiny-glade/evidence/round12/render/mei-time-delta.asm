
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1406842e0:     	pushq	%rbp
1406842e1:     	pushq	%r15
1406842e3:     	pushq	%r14
1406842e5:     	pushq	%r13
1406842e7:     	pushq	%r12
1406842e9:     	pushq	%rsi
1406842ea:     	pushq	%rdi
1406842eb:     	pushq	%rbx
1406842ec:     	subq	$0x4e8, %rsp            # imm = 0x4E8
1406842f3:     	leaq	0x80(%rsp), %rbp
1406842fb:     	movaps	%xmm11, 0x450(%rbp)
140684303:     	movaps	%xmm10, 0x440(%rbp)
14068430b:     	movaps	%xmm9, 0x430(%rbp)
140684313:     	movaps	%xmm8, 0x420(%rbp)
14068431b:     	movaps	%xmm7, 0x410(%rbp)
140684322:     	movaps	%xmm6, 0x400(%rbp)
140684329:     	movq	$-0x2, 0x3f8(%rbp)
140684334:     	movq	%r9, %r14
140684337:     	movq	%r8, %rdi
14068433a:     	movq	%rdx, %rbx
14068433d:     	movq	%rcx, %rsi
140684340:     	callq	0x140c16a00 <.text+0xc15a00>
140684345:     	testb	%al, %al
140684347:     	movq	%rdi, 0x3b8(%rbp)
14068434e:     	je	0x1406843b0 <.text+0x6833b0>
140684350:     	leaq	0x239361f(%rip), %r12   # 0x142a17976
140684357:     	movl	$0x22, %r15d
14068435d:     	movl	$0x22, %edx
140684362:     	movq	%r12, %rcx
140684365:     	callq	0x140682140 <.text+0x681140>
14068436a:     	cmpq	$0x1, %rax
14068436e:     	jne	0x140684427 <.text+0x683427>
140684374:     	movq	%rdx, %r9
140684377:     	testq	%rdx, %rdx
14068437a:     	je	0x1406843ba <.text+0x6833ba>
14068437c:     	cmpq	$0x22, %r9
140684380:     	jae	0x1406843b8 <.text+0x6833b8>
140684382:     	leaq	0x23935ed(%rip), %rax   # 0x142a17976
140684389:     	cmpb	$-0x41, (%r9,%rax)
14068438e:     	jg	0x1406843ba <.text+0x6833ba>
140684390:     	leaq	0x2393689(%rip), %rax   # 0x142a17a20
140684397:     	movq	%rax, 0x20(%rsp)
14068439c:     	leaq	0x23935d3(%rip), %rcx   # 0x142a17976
1406843a3:     	movl	$0x22, %edx
1406843a8:     	xorl	%r8d, %r8d
1406843ab:     	callq	0x1428d9940 <.text+0x28d8940>
1406843b0:     	xorl	%r13d, %r13d
1406843b3:     	jmp	0x140684582 <.text+0x683582>
1406843b8:     	jne	0x140684390 <.text+0x683390>
1406843ba:     	leaq	0x23935b5(%rip), %r12   # 0x142a17976
1406843c1:     	movq	%r12, %rcx
1406843c4:     	movq	%r9, %rdx
1406843c7:     	callq	0x140682140 <.text+0x681140>
1406843cc:     	cmpq	$0x1, %rax
1406843d0:     	jne	0x140684427 <.text+0x683427>
1406843d2:     	movq	%rdx, %r8
1406843d5:     	addq	$0x2, %r8
1406843d9:     	je	0x140684414 <.text+0x683414>
1406843db:     	cmpq	$0x22, %r8
1406843df:     	jae	0x140684412 <.text+0x683412>
1406843e1:     	leaq	0x239358e(%rip), %rax   # 0x142a17976
1406843e8:     	cmpb	$-0x41, (%r8,%rax)
1406843ed:     	jg	0x140684414 <.text+0x683414>
1406843ef:     	leaq	0x2393642(%rip), %rax   # 0x142a17a38
1406843f6:     	movq	%rax, 0x20(%rsp)
1406843fb:     	leaq	0x2393574(%rip), %rcx   # 0x142a17976
140684402:     	movl	$0x22, %edx
140684407:     	movl	$0x22, %r9d
14068440d:     	callq	0x1428d9940 <.text+0x28d8940>
140684412:     	jne	0x1406843ef <.text+0x6833ef>
140684414:     	movl	$0x20, %r15d
14068441a:     	subq	%rdx, %r15
14068441d:     	leaq	0x2393552(%rip), %r12   # 0x142a17976
140684424:     	addq	%r8, %r12
140684427:     	leaq	0x23937f0(%rip), %r8    # 0x142a17c1e
14068442e:     	leaq	0x23937c3(%rip), %r13   # 0x142a17bf8
140684435:     	nopw	%cs:(%rax,%rax)
140684440:     	movsbl	-0x1(%r8), %eax
140684445:     	testl	%eax, %eax
140684447:     	js	0x140684460 <.text+0x683460>
140684449:     	decq	%r8
14068444c:     	cmpl	$0x5c, %eax
14068444f:     	jne	0x1406844b4 <.text+0x6834b4>
140684451:     	jmp	0x1406844c5 <.text+0x6834c5>
140684453:     	nopw	%cs:(%rax,%rax)
140684460:     	movzbl	-0x2(%r8), %ecx
140684465:     	cmpb	$-0x40, %cl
140684468:     	jge	0x14068448d <.text+0x68348d>
14068446a:     	movzbl	-0x3(%r8), %edx
14068446f:     	cmpb	$-0x40, %dl
140684472:     	jge	0x140684496 <.text+0x683496>
140684474:     	movzbl	-0x4(%r8), %r9d
140684479:     	addq	$-0x4, %r8
14068447d:     	andl	$0x7, %r9d
140684481:     	shll	$0x6, %r9d
140684485:     	andl	$0x3f, %edx
140684488:     	orl	%r9d, %edx
14068448b:     	jmp	0x14068449d <.text+0x68349d>
14068448d:     	addq	$-0x2, %r8
140684491:     	andl	$0x1f, %ecx
140684494:     	jmp	0x1406844a5 <.text+0x6834a5>
140684496:     	addq	$-0x3, %r8
14068449a:     	andl	$0xf, %edx
14068449d:     	shll	$0x6, %edx
1406844a0:     	andl	$0x3f, %ecx
1406844a3:     	orl	%edx, %ecx
1406844a5:     	shll	$0x6, %ecx
1406844a8:     	andb	$0x3f, %al
1406844aa:     	movzbl	%al, %eax
1406844ad:     	orl	%ecx, %eax
1406844af:     	cmpl	$0x5c, %eax
1406844b2:     	je	0x1406844c5 <.text+0x6834c5>
1406844b4:     	cmpl	$0x2f, %eax
1406844b7:     	je	0x1406844c5 <.text+0x6834c5>
1406844b9:     	cmpq	%r13, %r8
1406844bc:     	jne	0x140684440 <.text+0x683440>
1406844be:     	movl	$0x26, %edi
1406844c3:     	jmp	0x140684511 <.text+0x683511>
1406844c5:     	leaq	0x239372c(%rip), %r13   # 0x142a17bf8
1406844cc:     	subq	%r13, %r8
1406844cf:     	incq	%r8
1406844d2:     	je	0x140684506 <.text+0x683506>
1406844d4:     	cmpq	$0x26, %r8
1406844d8:     	jae	0x140684504 <.text+0x683504>
1406844da:     	cmpb	$-0x41, (%r8,%r13)
1406844df:     	jg	0x140684506 <.text+0x683506>
1406844e1:     	leaq	0x2393518(%rip), %rax   # 0x142a17a00
1406844e8:     	movq	%rax, 0x20(%rsp)
1406844ed:     	leaq	0x2393704(%rip), %rcx   # 0x142a17bf8
1406844f4:     	movl	$0x26, %edx
1406844f9:     	movl	$0x26, %r9d
1406844ff:     	callq	0x1428d9940 <.text+0x28d8940>
140684504:     	jne	0x1406844e1 <.text+0x6834e1>
140684506:     	movl	$0x26, %edi
14068450b:     	subq	%r8, %rdi
14068450e:     	addq	%r8, %r13
140684511:     	callq	0x140c18130 <.text+0xc17130>
140684516:     	movq	%rax, %rcx
140684519:     	movq	(%rax), %rax
14068451c:     	cmpq	$0x1, %rax
140684520:     	jne	0x1406855d0 <.text+0x6845d0>
140684526:     	addq	$0x8, %rcx
14068452a:     	cmpq	$0x0, (%rcx)
14068452e:     	jne	0x1406855f4 <.text+0x6845f4>
140684534:     	movq	$-0x1, (%rcx)
14068453b:     	movq	%rcx, 0x3e8(%rbp)
140684542:     	addq	$0x8, %rcx
140684546:     	movq	%rdi, 0x20(%rsp)
14068454b:     	movq	$0x0, 0x30(%rsp)
140684554:     	movq	$0x1, 0x28(%rsp)
14068455d:     	movq	%r12, %rdx
140684560:     	movq	%r15, %r8
140684563:     	movq	%r13, %r9
140684566:     	callq	0x140c16d40 <.text+0xc15d40>
14068456b:     	movq	0x3e8(%rbp), %rcx
140684572:     	incq	(%rcx)
140684575:     	movq	%rax, 0x348(%rbp)
14068457c:     	movl	$0x1, %r13d
140684582:     	movq	0x4d0(%rbp), %rax
140684589:     	movq	%r13, 0x340(%rbp)
140684590:     	movl	0x1c(%rsi), %ecx
140684593:     	movq	0x10(%rsi), %rdx
140684597:     	movl	%ecx, (%rdx)
140684599:     	movq	(%rsi), %rdx
14068459c:     	movq	(%rax), %rax
14068459f:     	movss	0x58(%rax), %xmm0
1406845a4:     	movss	%xmm0, 0x270(%rdx)
1406845ac:     	leaq	0x1d0(%rdx), %r15
1406845b3:     	cmpq	$0x0, 0x1e8(%rdx)
1406845bb:     	movq	%rdx, 0x3c8(%rbp)
1406845c2:     	je	0x140684611 <.text+0x683611>
1406845c4:     	movq	0x1d8(%rdx), %rsi
1406845cb:     	testq	%rsi, %rsi
1406845ce:     	je	0x1406845de <.text+0x6835de>
1406845d0:     	movq	(%r15), %rcx
1406845d3:     	leaq	0x11(%rsi), %r8
1406845d7:     	movb	$-0x1, %dl
1406845d9:     	callq	0x142923c20 <.text+0x2922c20>
1406845de:     	movq	0x3c8(%rbp), %rdx
1406845e5:     	movq	$0x0, 0x1e8(%rdx)
1406845f0:     	leaq	0x1(%rsi), %rax
1406845f4:     	movq	%rax, %rcx
1406845f7:     	shrq	$0x3, %rcx
1406845fb:     	andq	$-0x8, %rax
1406845ff:     	subq	%rcx, %rax
140684602:     	cmpq	$0x8, %rsi
140684606:     	cmovbq	%rsi, %rax
14068460a:     	movq	%rax, 0x1e0(%rdx)
140684611:     	movq	%r14, 0x3c0(%rbp)
140684618:     	movq	%rbx, 0x3e0(%rbp)
14068461f:     	leaq	0x268(%rdx), %rax
140684626:     	movq	%rax, 0x3a8(%rbp)
14068462d:     	leaq	0x1a0(%rdx), %rax
140684634:     	movq	%rax, 0x218(%rbp)
14068463b:     	movq	0x1a0(%rdx), %rbx
