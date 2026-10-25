
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140500370:     	pushq	%r15
140500372:     	pushq	%r14
140500374:     	pushq	%r13
140500376:     	pushq	%r12
140500378:     	pushq	%rsi
140500379:     	pushq	%rdi
14050037a:     	pushq	%rbx
14050037b:     	subq	$0xf0, %rsp
140500382:     	movq	%rcx, %rdi
140500385:     	movq	%rdx, 0x60(%rsp)
14050038a:     	movq	%r8, 0x68(%rsp)
14050038f:     	leaq	0x10(%rcx), %rsi
140500393:     	movq	%rsi, %rcx
140500396:     	callq	0x1406868b0 <.text+0x6858b0>
14050039b:     	movq	%rax, %r15
14050039e:     	leaq	0x60(%rsp), %r14
1405003a3:     	movq	%r14, %rcx
1405003a6:     	callq	0x140683c10 <.text+0x682c10>
1405003ab:     	movq	%rax, 0x40(%rsp)
1405003b0:     	leaq	0x40(%rsp), %rbx
1405003b5:     	movq	%rbx, %rcx
1405003b8:     	callq	0x1400885d0 <.text+0x875d0>
1405003bd:     	movq	%rax, 0x70(%rsp)
1405003c2:     	movabsq	$0x41223169ff28813b, %r12 # imm = 0x41223169FF28813B
1405003cc:     	movq	%r12, 0x78(%rsp)
1405003d1:     	movabsq	$-0x58648d975d569727, %r13 # imm = 0xA79B7268A2A968D9
1405003db:     	movq	%r13, 0x80(%rsp)
1405003e3:     	leaq	0x70(%rsp), %rcx
1405003e8:     	callq	0x1405bdf20 <.text+0x5bcf20>
1405003ed:     	movq	%r15, 0x70(%rsp)
1405003f2:     	movq	%rax, 0x78(%rsp)
1405003f7:     	leaq	0x70(%rsp), %rcx
1405003fc:     	callq	0x1405bd910 <.text+0x5bc910>
140500401:     	movq	%rax, %r15
140500404:     	movq	%rsi, %rcx
140500407:     	movq	%rax, %rdx
14050040a:     	callq	0x140686450 <.text+0x685450>
14050040f:     	testb	%al, %al
140500411:     	je	0x14050042b <.text+0x4ff42b>
140500413:     	leaq	0x40(%rsp), %rdi
140500418:     	movq	%rdi, %rcx
14050041b:     	movq	%rsi, %rdx
14050041e:     	movq	%r15, %r8
140500421:     	callq	0x140686530 <.text+0x685530>
140500426:     	jmp	0x140500577 <.text+0x4ff577>
14050042b:     	movq	0x312bf76(%rip), %rax   # 0x14362c3a8
140500432:     	movq	(%rax), %rax
140500435:     	testq	%rax, %rax
140500438:     	je	0x14050051c <.text+0x4ff51c>
14050043e:     	movq	%rdi, 0x40(%rsp)
140500443:     	leaq	0x33456(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
14050044a:     	movq	%rax, 0x48(%rsp)
14050044f:     	movq	%r14, 0x50(%rsp)
140500454:     	movq	%rax, 0x58(%rsp)
140500459:     	leaq	0x24ed8f8(%rip), %rcx   # 0x1429edd58
140500460:     	callq	0x14067dd80 <.text+0x67cd80>
140500465:     	movups	(%rax), %xmm0
140500468:     	movl	0x10(%rax), %eax
14050046b:     	movq	$0x1, 0xa0(%rsp)
140500477:     	leaq	0x24ed8aa(%rip), %rcx   # 0x1429edd28
14050047e:     	movq	%rcx, 0xa8(%rsp)
140500486:     	movq	$0x14, 0xb0(%rsp)
140500492:     	leaq	0x24ed7ff(%rip), %rdx   # 0x1429edc98
140500499:     	movq	%rdx, 0xc0(%rsp)
1405004a1:     	movq	$0x2, 0xc8(%rsp)
1405004ad:     	movq	%rbx, 0xd0(%rsp)
1405004b5:     	movq	$0x2, 0xd8(%rsp)
1405004c1:     	movq	$0x0, 0xe0(%rsp)
1405004cd:     	movq	$0x0, 0x70(%rsp)
1405004d6:     	movq	%rcx, 0x78(%rsp)
1405004db:     	movq	$0x14, 0x80(%rsp)
1405004e7:     	movq	$0x0, 0x88(%rsp)
1405004f3:     	movups	%xmm0, 0x90(%rsp)
1405004fb:     	movl	$0x1, 0xb8(%rsp)
140500506:     	movl	%eax, 0xbc(%rsp)
14050050d:     	leaq	0x3f(%rsp), %rcx
140500512:     	leaq	0x70(%rsp), %rdx
140500517:     	callq	0x140d4a330 <.text+0xd49330>
14050051c:     	movups	0x60(%rsp), %xmm0
140500521:     	movq	0x18(%rdi), %rdi
140500525:     	movups	%xmm0, 0x40(%rsp)
14050052a:     	leaq	0x40(%rsp), %rcx
14050052f:     	callq	0x140683c10 <.text+0x682c10>
140500534:     	movq	%rax, 0x70(%rsp)
140500539:     	movq	%r12, 0x78(%rsp)
14050053e:     	movq	%r13, 0x80(%rsp)
140500546:     	leaq	0x70(%rsp), %rcx
14050054b:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500550:     	movq	%rdi, 0x70(%rsp)
140500555:     	movq	%rax, 0x78(%rsp)
14050055a:     	leaq	0x70(%rsp), %rcx
14050055f:     	callq	0x1405bd910 <.text+0x5bc910>
140500564:     	leaq	0x40(%rsp), %rdi
140500569:     	movq	%rdi, %rcx
14050056c:     	movq	%rsi, %rdx
14050056f:     	movq	%rax, %r8
140500572:     	callq	0x1406861b0 <.text+0x6851b0>
140500577:     	movq	0x312bb1a(%rip), %rax   # 0x14362c098
14050057e:     	movzbl	(%rax), %eax
140500581:     	leaq	(%rax,%rax,4), %rcx
140500585:     	addq	$0x14, %rcx
140500589:     	leaq	0x24ed923(%rip), %rdx   # 0x1429edeb3
140500590:     	leaq	0x24ed908(%rip), %r8    # 0x1429ede9f
140500597:     	testq	%rax, %rax
14050059a:     	cmovneq	%rdx, %r8
14050059e:     	movq	%rcx, 0x28(%rsp)
1405005a3:     	movq	%r8, 0x20(%rsp)
1405005a8:     	movl	$0x43960000, 0x30(%rsp) # imm = 0x43960000
1405005b0:     	leaq	0x24ed915(%rip), %r8    # 0x1429edecc
1405005b7:     	leaq	0x70(%rsp), %rcx
1405005bc:     	movl	$0x8, %r9d
1405005c2:     	movq	%rdi, %rdx
1405005c5:     	callq	0x14032e0b0 <.text+0x32d0b0>
1405005ca:     	nop
1405005cb:     	addq	$0xf0, %rsp
1405005d2:     	popq	%rbx
1405005d3:     	popq	%rdi
1405005d4:     	popq	%rsi
1405005d5:     	popq	%r12
1405005d7:     	popq	%r13
1405005d9:     	popq	%r14
1405005db:     	popq	%r15
1405005dd:     	retq
