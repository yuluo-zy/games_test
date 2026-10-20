
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142347f90:     	pushq	%r15
142347f92:     	pushq	%r14
142347f94:     	pushq	%r13
142347f96:     	pushq	%r12
142347f98:     	pushq	%rsi
142347f99:     	pushq	%rdi
142347f9a:     	pushq	%rbp
142347f9b:     	pushq	%rbx
142347f9c:     	subq	$0x118, %rsp            # imm = 0x118
142347fa3:     	movaps	%xmm12, 0x100(%rsp)
142347fac:     	movaps	%xmm11, 0xf0(%rsp)
142347fb5:     	movaps	%xmm10, 0xe0(%rsp)
142347fbe:     	movaps	%xmm9, 0xd0(%rsp)
142347fc7:     	movaps	%xmm8, 0xc0(%rsp)
142347fd0:     	movaps	%xmm7, 0xb0(%rsp)
142347fd8:     	movaps	%xmm6, 0xa0(%rsp)
142347fe0:     	movq	0x180(%rsp), %rax
142347fe8:     	movq	(%rcx), %r13
142347feb:     	movq	0x8(%rcx), %r10
142347fef:     	movq	(%r13), %rsi
142347ff3:     	xorl	%r11d, %r11d
142347ff6:     	movq	%rsi, %rbx
142347ff9:     	subq	0x18(%r10), %rbx
142347ffd:     	cmovbq	%r11, %rbx
142348001:     	subq	0x38(%r10), %rsi
142348005:     	movq	0x188(%rsp), %rcx
14234800d:     	cmovbq	%r11, %rsi
142348011:     	movq	%rbx, %rdi
142348014:     	shlq	$0x4, %rdi
142348018:     	addq	0x8(%r10), %rdi
14234801c:     	movq	0x10(%r10), %r15
142348020:     	movq	0x30(%r10), %rbp
142348024:     	subq	%rbx, %r15
142348027:     	cmovbq	%r11, %r15
14234802b:     	movl	$0x8, %r14d
142348031:     	cmovbq	%r14, %rdi
142348035:     	movq	%rsi, %rbx
142348038:     	shlq	$0x4, %rbx
14234803c:     	addq	0x28(%r10), %rbx
142348040:     	subq	%rsi, %rbp
142348043:     	cmovbq	%r11, %rbp
142348047:     	cmovbq	%r14, %rbx
14234804b:     	movq	0x40(%r10), %r10
14234804f:     	subq	%rbp, %r10
142348052:     	subq	%r15, %r10
142348055:     	movq	%r10, (%r13)
142348059:     	shlq	$0x4, %r15
14234805d:     	shlq	$0x4, %rbp
142348061:     	movq	0x8(%r8), %r10
142348065:     	movq	%r10, 0x68(%rsp)
14234806a:     	testq	%r10, %r10
14234806d:     	jne	0x142348078 <.text+0x2347078>
14234806f:     	movq	0x10(%r8), %r8
142348073:     	movq	%r8, 0x68(%rsp)
142348078:     	addq	%rdi, %r15
14234807b:     	addq	%rbx, %rbp
14234807e:     	movq	(%rdx), %r10
142348081:     	movq	0x8(%rdx), %rsi
142348085:     	movl	0x1c(%rcx), %edx
142348088:     	movl	%edx, 0x54(%rsp)
14234808c:     	movq	(%rcx), %rdx
14234808f:     	movq	%rdx, 0x88(%rsp)
142348097:     	movq	0x10(%rcx), %rcx
14234809b:     	movq	%rcx, 0x80(%rsp)
1423480a3:     	movl	0x1c(%rax), %ecx
1423480a6:     	movl	%ecx, 0x50(%rsp)
1423480aa:     	movq	(%rax), %rcx
1423480ad:     	movq	%rcx, 0x78(%rsp)
1423480b2:     	movq	0x10(%rax), %rax
1423480b6:     	movq	%rax, 0x70(%rsp)
1423480bb:     	movl	0x1c(%r9), %eax
1423480bf:     	movl	%eax, 0x34(%rsp)
1423480c3:     	movq	(%r9), %rax
1423480c6:     	movq	%rax, 0x60(%rsp)
1423480cb:     	movq	0x10(%r9), %rax
1423480cf:     	movq	%rax, 0x58(%rsp)
1423480d4:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
1423480da:     	movsd	0xd05fcd(%rip), %xmm8   # 0x14304e0b0
1423480e3:     	movss	0x5dd818(%rip), %xmm9   # 0x142925904
1423480ec:     	movss	0x5ec5a7(%rip), %xmm10  # 0x14293469c
1423480f5:     	movss	0x5e65f2(%rip), %xmm11  # 0x14292e6f0
1423480fe:     	movss	0x78d7e1(%rip), %xmm12  # 0x142ad58e8
142348107:     	movq	%r15, 0x98(%rsp)
14234810f:     	movq	%rbp, 0x90(%rsp)
142348117:     	movq	%r10, 0x28(%rsp)
14234811c:     	jmp	0x142348130 <.text+0x2347130>
14234811e:     	movq	0x28(%rsp), %r10
142348123:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
142348129:     	nopl	(%rax)
142348130:     	testq	%rdi, %rdi
142348133:     	je	0x142348140 <.text+0x2347140>
142348135:     	cmpq	%r15, %rdi
142348138:     	je	0x142348140 <.text+0x2347140>
14234813a:     	leaq	0x10(%rdi), %rax
14234813e:     	jmp	0x14234815b <.text+0x234715b>
142348140:     	testq	%rbx, %rbx
142348143:     	je	0x14234867f <.text+0x234767f>
142348149:     	cmpq	%rbp, %rbx
14234814c:     	je	0x14234867f <.text+0x234767f>
142348152:     	movq	%rbx, %rdi
142348155:     	addq	$0x10, %rbx
142348159:     	xorl	%eax, %eax
14234815b:     	incq	(%r13)
14234815f:     	movq	0x8(%rdi), %r14
142348163:     	movq	%rax, %rdi
142348166:     	movl	%r14d, %eax
142348169:     	cmpq	0x10(%rsi), %rax
14234816d:     	jae	0x142348130 <.text+0x2347130>
14234816f:     	movq	0x8(%rsi), %rcx
142348173:     	movq	%r14, %rdx
142348176:     	shrq	$0x20, %rdx
14234817a:     	leaq	(%rax,%rax,4), %rax
14234817e:     	cmpl	%edx, (%rcx,%rax,4)
142348181:     	jne	0x142348130 <.text+0x2347130>
142348183:     	leaq	(%rcx,%rax,4), %rax
142348187:     	movl	0x4(%rax), %ecx
14234818a:     	cmpq	%r9, %rcx
14234818d:     	je	0x142348130 <.text+0x2347130>
14234818f:     	cmpq	%rcx, 0x38(%r10)
142348193:     	jbe	0x142348130 <.text+0x2347130>
142348195:     	movq	0x28(%r10), %rdx
142348199:     	movl	%ecx, %r8d
14234819c:     	shrl	$0x6, %r8d
1423481a0:     	movq	(%rdx,%r8,8), %rdx
1423481a4:     	btq	%rcx, %rdx
1423481a8:     	jae	0x142348130 <.text+0x2347130>
1423481aa:     	movl	0xc(%rax), %ecx
1423481ad:     	movq	0x1a8(%rsi), %rdx
1423481b4:     	leaq	(%rcx,%rcx,8), %r8
1423481b8:     	movq	0x110(%r10), %rcx
1423481bf:     	cmpq	%rcx, 0x40(%rdx,%r8,8)
1423481c4:     	jbe	0x1423481ed <.text+0x23471ed>
1423481c6:     	leaq	(%rdx,%r8,8), %rdx
1423481ca:     	movq	0x38(%rdx), %r8
1423481ce:     	movq	(%r8,%rcx,8), %rcx
1423481d2:     	testq	%rcx, %rcx
1423481d5:     	je	0x1423481ed <.text+0x23471ed>
1423481d7:     	movq	0x18(%rdx), %rdx
1423481db:     	notq	%rcx
1423481de:     	leaq	(%rcx,%rcx,2), %rcx
1423481e2:     	shlq	$0x4, %rcx
1423481e6:     	movq	0x10(%rdx,%rcx), %r15
1423481eb:     	jmp	0x1423481f0 <.text+0x23471f0>
1423481ed:     	xorl	%r15d, %r15d
1423481f0:     	movl	0x10(%rax), %ebp
1423481f3:     	movq	0x68(%rsp), %rcx
1423481f8:     	movq	(%rcx), %rax
1423481fb:     	movq	0x10(%rcx), %r12
1423481ff:     	subq	%r12, %rax
142348202:     	cmpq	$0x27, %rax
142348206:     	jbe	0x142348653 <.text+0x2347653>
14234820c:     	movq	0x8(%rcx), %rax
142348210:     	leaq	-0x1369017(%rip), %rdx  # 0x140fdf200 <.text+0xfde200>
142348217:     	movq	%rdx, (%rax,%r12)
14234821b:     	leaq	-0x4f89c2(%rip), %rdx   # 0x141e4f860 <.text+0x1e4e860>
142348222:     	movq	%rdx, 0x8(%rax,%r12)
142348227:     	movq	%r14, 0x10(%rax,%r12)
14234822c:     	movsd	%xmm8, 0x18(%rax,%r12)
142348233:     	movl	$0x0, 0x20(%rax,%r12)
14234823c:     	movb	$0x0, 0x24(%rax,%r12)
142348242:     	addq	$0x28, %r12
142348246:     	movq	%r12, 0x10(%rcx)
14234824a:     	movzbl	(%r15,%rbp), %eax
14234824f:     	shll	$0x2, %eax
142348252:     	leaq	0xd06b67(%rip), %rcx    # 0x14304edc0
142348259:     	movss	(%rax,%rcx), %xmm7
14234825e:     	leaq	0xd06b27(%rip), %rcx    # 0x14304ed8c
142348265:     	movss	(%rax,%rcx), %xmm6
14234826a:     	leaq	0xd06ae7(%rip), %rcx    # 0x14304ed58
142348271:     	movss	(%rax,%rcx), %xmm2
142348276:     	movaps	%xmm7, %xmm0
142348279:     	movaps	%xmm6, %xmm1
14234827c:     	callq	0x140c904a0 <.text+0xc8f4a0>
142348281:     	movaps	%xmm9, %xmm1
142348285:     	subss	%xmm0, %xmm1
142348289:     	mulss	%xmm7, %xmm1
14234828d:     	mulss	%xmm0, %xmm6
142348291:     	addss	%xmm1, %xmm6
142348295:     	movzbl	(%r15,%rbp), %eax
14234829a:     	leaq	0xd06b53(%rip), %rcx    # 0x14304edf4
1423482a1:     	movslq	(%rcx,%rax,4), %rax
1423482a5:     	addq	%rcx, %rax
1423482a8:     	movq	0x98(%rsp), %r15
1423482b0:     	movq	0x90(%rsp), %rbp
1423482b8:     	movq	0x28(%rsp), %r10
1423482bd:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
1423482c3:     	jmpq	*%rax
1423482c5:     	movq	0x60(%rsp), %r14
1423482ca:     	movq	%r14, %rcx
1423482cd:     	callq	0x14067dd80 <.text+0x67cd80>
1423482d2:     	movss	0x1c(%rax), %xmm6
1423482d7:     	movl	0x34(%rsp), %eax
1423482db:     	movq	0x58(%rsp), %rcx
1423482e0:     	movl	%eax, (%rcx)
1423482e2:     	leaq	0x38(%rsp), %rcx
1423482e7:     	movq	%r14, %rdx
1423482ea:     	callq	0x1408aea60 <.text+0x8ada60>
1423482ef:     	movq	0x38(%rsp), %rax
1423482f4:     	movq	0x40(%rsp), %rcx
1423482f9:     	movq	0x48(%rsp), %rdx
1423482fe:     	jmp	0x142348307 <.text+0x2347307>
142348300:     	addq	$0x40, %rax
142348304:     	incq	%rdx
142348307:     	cmpq	%rcx, %rax
14234830a:     	je	0x14234811e <.text+0x234711e>
142348310:     	testq	%rdx, %rdx
142348313:     	setne	%r8b
142348317:     	testq	%rax, %rax
14234831a:     	setne	%r9b
14234831e:     	testb	%r9b, %r8b
142348321:     	je	0x142348300 <.text+0x2347300>
142348323:     	movss	%xmm6, 0x1c(%rax)
142348328:     	jmp	0x142348300 <.text+0x2347300>
14234832a:     	movl	0x54(%rsp), %eax
14234832e:     	movq	0x80(%rsp), %rcx
142348336:     	movl	%eax, (%rcx)
142348338:     	movq	0x88(%rsp), %rcx
142348340:     	callq	0x1409f18c0 <.text+0x9f08c0>
142348345:     	jmp	0x142348614 <.text+0x2347614>
14234834a:     	movl	0x34(%rsp), %eax
14234834e:     	movq	0x58(%rsp), %rcx
142348353:     	movl	%eax, (%rcx)
142348355:     	leaq	0x38(%rsp), %rcx
14234835a:     	movq	0x60(%rsp), %rdx
14234835f:     	callq	0x1408aea60 <.text+0x8ada60>
142348364:     	movq	0x38(%rsp), %rax
142348369:     	movq	0x40(%rsp), %rcx
14234836e:     	movq	0x48(%rsp), %rdx
142348373:     	jmp	0x142348387 <.text+0x2347387>
142348375:     	nopw	%cs:(%rax,%rax)
142348380:     	addq	$0x40, %rax
142348384:     	incq	%rdx
142348387:     	cmpq	%rcx, %rax
14234838a:     	je	0x14234811e <.text+0x234711e>
142348390:     	testq	%rdx, %rdx
142348393:     	setne	%r8b
142348397:     	testq	%rax, %rax
14234839a:     	setne	%r9b
14234839e:     	testb	%r9b, %r8b
1423483a1:     	je	0x142348380 <.text+0x2347380>
1423483a3:     	movss	%xmm6, 0x38(%rax)
1423483a8:     	jmp	0x142348380 <.text+0x2347380>
1423483aa:     	movl	0x54(%rsp), %eax
1423483ae:     	movq	0x80(%rsp), %rcx
1423483b6:     	movl	%eax, (%rcx)
1423483b8:     	movq	0x88(%rsp), %rcx
1423483c0:     	callq	0x1409f1860 <.text+0x9f0860>
1423483c5:     	jmp	0x142348614 <.text+0x2347614>
1423483ca:     	movq	0x60(%rsp), %r14
1423483cf:     	movq	%r14, %rcx
1423483d2:     	callq	0x14067dd80 <.text+0x67cd80>
1423483d7:     	movss	0x24(%rax), %xmm6
1423483dc:     	movl	0x34(%rsp), %eax
1423483e0:     	movq	0x58(%rsp), %rcx
1423483e5:     	movl	%eax, (%rcx)
1423483e7:     	leaq	0x38(%rsp), %rcx
1423483ec:     	movq	%r14, %rdx
1423483ef:     	callq	0x1408aea60 <.text+0x8ada60>
1423483f4:     	movq	0x38(%rsp), %rax
1423483f9:     	movq	0x40(%rsp), %rcx
1423483fe:     	movq	0x48(%rsp), %rdx
142348403:     	jmp	0x142348417 <.text+0x2347417>
142348405:     	nopw	%cs:(%rax,%rax)
142348410:     	addq	$0x40, %rax
142348414:     	incq	%rdx
142348417:     	cmpq	%rcx, %rax
14234841a:     	je	0x14234811e <.text+0x234711e>
142348420:     	testq	%rdx, %rdx
142348423:     	setne	%r8b
142348427:     	testq	%rax, %rax
14234842a:     	setne	%r9b
14234842e:     	testb	%r9b, %r8b
142348431:     	je	0x142348410 <.text+0x2347410>
142348433:     	movss	%xmm6, 0x24(%rax)
142348438:     	jmp	0x142348410 <.text+0x2347410>
14234843a:     	movl	0x34(%rsp), %eax
14234843e:     	movq	0x58(%rsp), %rcx
142348443:     	movl	%eax, (%rcx)
142348445:     	leaq	0x38(%rsp), %rcx
14234844a:     	movq	0x60(%rsp), %rdx
14234844f:     	callq	0x1408aea60 <.text+0x8ada60>
142348454:     	movq	0x38(%rsp), %rax
142348459:     	movq	0x40(%rsp), %rcx
14234845e:     	movq	0x48(%rsp), %rdx
142348463:     	jmp	0x142348477 <.text+0x2347477>
142348465:     	nopw	%cs:(%rax,%rax)
142348470:     	addq	$0x40, %rax
142348474:     	incq	%rdx
142348477:     	cmpq	%rcx, %rax
14234847a:     	je	0x14234811e <.text+0x234711e>
142348480:     	testq	%rdx, %rdx
142348483:     	setne	%r8b
142348487:     	testq	%rax, %rax
14234848a:     	setne	%r9b
14234848e:     	testb	%r9b, %r8b
142348491:     	je	0x142348470 <.text+0x2347470>
142348493:     	movss	%xmm6, 0x30(%rax)
142348498:     	jmp	0x142348470 <.text+0x2347470>
14234849a:     	movl	0x50(%rsp), %eax
14234849e:     	movq	0x70(%rsp), %rcx
1423484a3:     	movl	%eax, (%rcx)
1423484a5:     	movq	0x78(%rsp), %rcx
1423484aa:     	callq	0x140911240 <.text+0x910240>
1423484af:     	movq	%rax, %r12
1423484b2:     	movaps	%xmm10, %xmm1
1423484b6:     	maxss	%xmm6, %xmm1
1423484ba:     	movaps	%xmm11, %xmm0
1423484be:     	minss	%xmm1, %xmm0
1423484c2:     	addss	%xmm12, %xmm0
1423484c7:     	callq	0x142923eb0 <.text+0x2922eb0>
1423484cc:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
1423484d2:     	movq	0x28(%rsp), %r10
1423484d7:     	movss	%xmm0, 0x18(%r12)
1423484de:     	cmpl	$0x2, (%r12)
1423484e3:     	jae	0x142348130 <.text+0x2347130>
1423484e9:     	movl	$0x3, (%r12)
1423484f1:     	movsd	0x10(%r12), %xmm0
1423484f8:     	movsd	%xmm0, 0x4(%r12)
1423484ff:     	jmp	0x142348130 <.text+0x2347130>
142348504:     	movq	0x60(%rsp), %r14
142348509:     	movq	%r14, %rcx
14234850c:     	callq	0x14067dd80 <.text+0x67cd80>
142348511:     	movss	0x20(%rax), %xmm6
142348516:     	movl	0x34(%rsp), %eax
14234851a:     	movq	0x58(%rsp), %rcx
14234851f:     	movl	%eax, (%rcx)
142348521:     	leaq	0x38(%rsp), %rcx
142348526:     	movq	%r14, %rdx
142348529:     	callq	0x1408aea60 <.text+0x8ada60>
14234852e:     	movq	0x38(%rsp), %rax
142348533:     	movq	0x40(%rsp), %rcx
142348538:     	movq	0x48(%rsp), %rdx
14234853d:     	jmp	0x142348547 <.text+0x2347547>
14234853f:     	nop
142348540:     	addq	$0x40, %rax
142348544:     	incq	%rdx
142348547:     	cmpq	%rcx, %rax
14234854a:     	je	0x14234811e <.text+0x234711e>
142348550:     	testq	%rdx, %rdx
142348553:     	setne	%r8b
142348557:     	testq	%rax, %rax
14234855a:     	setne	%r9b
14234855e:     	testb	%r9b, %r8b
142348561:     	je	0x142348540 <.text+0x2347540>
142348563:     	movss	%xmm6, 0x20(%rax)
142348568:     	jmp	0x142348540 <.text+0x2347540>
14234856a:     	movl	0x34(%rsp), %eax
14234856e:     	movq	0x58(%rsp), %rcx
142348573:     	movl	%eax, (%rcx)
142348575:     	leaq	0x38(%rsp), %rcx
14234857a:     	movq	0x60(%rsp), %rdx
14234857f:     	callq	0x1408aea60 <.text+0x8ada60>
142348584:     	movq	0x38(%rsp), %rax
142348589:     	movq	0x40(%rsp), %rcx
14234858e:     	movq	0x48(%rsp), %rdx
142348593:     	jmp	0x1423485a7 <.text+0x23475a7>
142348595:     	nopw	%cs:(%rax,%rax)
1423485a0:     	addq	$0x40, %rax
1423485a4:     	incq	%rdx
1423485a7:     	cmpq	%rcx, %rax
1423485aa:     	je	0x14234811e <.text+0x234711e>
1423485b0:     	testq	%rdx, %rdx
1423485b3:     	setne	%r8b
1423485b7:     	testq	%rax, %rax
1423485ba:     	setne	%r9b
1423485be:     	testb	%r9b, %r8b
1423485c1:     	je	0x1423485a0 <.text+0x23475a0>
1423485c3:     	movss	%xmm6, 0x34(%rax)
1423485c8:     	jmp	0x1423485a0 <.text+0x23475a0>
1423485ca:     	movl	0x50(%rsp), %eax
1423485ce:     	movq	0x70(%rsp), %rcx
1423485d3:     	movl	%eax, (%rcx)
1423485d5:     	movq	0x78(%rsp), %rcx
1423485da:     	callq	0x140911260 <.text+0x910260>
1423485df:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
1423485e5:     	movq	0x28(%rsp), %r10
1423485ea:     	movss	0x14(%rax), %xmm0
1423485ef:     	movss	%xmm0, 0x8(%rax)
1423485f4:     	jmp	0x142348130 <.text+0x2347130>
1423485f9:     	movl	0x54(%rsp), %eax
1423485fd:     	movq	0x80(%rsp), %rcx
142348605:     	movl	%eax, (%rcx)
142348607:     	movq	0x88(%rsp), %rcx
14234860f:     	callq	0x1409f1890 <.text+0x9f0890>
142348614:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14234861a:     	movq	0x28(%rsp), %r10
14234861f:     	jmp	0x142348130 <.text+0x2347130>
142348624:     	movl	0x50(%rsp), %eax
142348628:     	movq	0x70(%rsp), %rcx
14234862d:     	movl	%eax, (%rcx)
14234862f:     	movq	0x78(%rsp), %rcx
142348634:     	callq	0x140911260 <.text+0x910260>
142348639:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14234863f:     	movq	0x28(%rsp), %r10
142348644:     	movss	0x24(%rax), %xmm0
142348649:     	movss	%xmm0, 0x18(%rax)
14234864e:     	jmp	0x142348130 <.text+0x2347130>
142348653:     	movq	$0x1, 0x20(%rsp)
14234865c:     	movl	$0x28, %r8d
142348662:     	movl	$0x1, %r9d
142348668:     	movq	0x68(%rsp), %rcx
14234866d:     	movq	%r12, %rdx
142348670:     	callq	0x1428a58d0 <.text+0x28a48d0>
142348675:     	movq	0x68(%rsp), %rcx
14234867a:     	jmp	0x14234820c <.text+0x234720c>
14234867f:     	movaps	0xa0(%rsp), %xmm6
142348687:     	movaps	0xb0(%rsp), %xmm7
14234868f:     	movaps	0xc0(%rsp), %xmm8
142348698:     	movaps	0xd0(%rsp), %xmm9
1423486a1:     	movaps	0xe0(%rsp), %xmm10
1423486aa:     	movaps	0xf0(%rsp), %xmm11
1423486b3:     	movaps	0x100(%rsp), %xmm12
1423486bc:     	addq	$0x118, %rsp            # imm = 0x118
1423486c3:     	popq	%rbx
1423486c4:     	popq	%rbp
1423486c5:     	popq	%rdi
1423486c6:     	popq	%rsi
1423486c7:     	popq	%r12
1423486c9:     	popq	%r13
1423486cb:     	popq	%r14
1423486cd:     	popq	%r15
1423486cf:     	retq
