
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140503240:     	pushq	%r15
140503242:     	pushq	%r14
140503244:     	pushq	%r13
140503246:     	pushq	%r12
140503248:     	pushq	%rsi
140503249:     	pushq	%rdi
14050324a:     	pushq	%rbp
14050324b:     	pushq	%rbx
14050324c:     	subq	$0xd8, %rsp
140503253:     	movq	%r9, %rsi
140503256:     	movq	%rcx, %rbx
140503259:     	movq	%rdx, 0x28(%rsp)
14050325e:     	movq	%r8, 0x30(%rsp)
140503263:     	leaq	0x10(%rcx), %rdi
140503267:     	movq	%rdi, %rcx
14050326a:     	callq	0x1406868b0 <.text+0x6858b0>
14050326f:     	movq	%rax, %r12
140503272:     	leaq	0x28(%rsp), %r15
140503277:     	movq	%r15, %rcx
14050327a:     	callq	0x140683c10 <.text+0x682c10>
14050327f:     	movq	%rax, 0x38(%rsp)
140503284:     	leaq	0x38(%rsp), %r14
140503289:     	movq	%r14, %rcx
14050328c:     	callq	0x1400885d0 <.text+0x875d0>
140503291:     	movq	%rax, 0x58(%rsp)
140503296:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
1405032a0:     	movq	%r13, 0x60(%rsp)
1405032a5:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
1405032af:     	movq	%rbp, 0x68(%rsp)
1405032b4:     	leaq	0x58(%rsp), %rcx
1405032b9:     	callq	0x1405bdf20 <.text+0x5bcf20>
1405032be:     	movq	%r12, 0x58(%rsp)
1405032c3:     	movq	%rax, 0x60(%rsp)
1405032c8:     	leaq	0x58(%rsp), %rcx
1405032cd:     	callq	0x1405bd910 <.text+0x5bc910>
1405032d2:     	movq	%rax, %r12
1405032d5:     	movq	%rdi, %rcx
1405032d8:     	movq	%rax, %rdx
1405032db:     	callq	0x140686450 <.text+0x685450>
1405032e0:     	testb	%al, %al
1405032e2:     	je	0x1405032fc <.text+0x5022fc>
1405032e4:     	leaq	0x58(%rsp), %rbx
1405032e9:     	movq	%rbx, %rcx
1405032ec:     	movq	%rdi, %rdx
1405032ef:     	movq	%r12, %r8
1405032f2:     	callq	0x140686530 <.text+0x685530>
1405032f7:     	jmp	0x14050343c <.text+0x50243c>
1405032fc:     	movq	0x31290a5(%rip), %rax   # 0x14362c3a8
140503303:     	movq	(%rax), %rax
140503306:     	testq	%rax, %rax
140503309:     	je	0x1405033e4 <.text+0x5023e4>
14050330f:     	movq	%rbx, 0x38(%rsp)
140503314:     	leaq	0x30585(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
14050331b:     	movq	%rax, 0x40(%rsp)
140503320:     	movq	%r15, 0x48(%rsp)
140503325:     	movq	%rax, 0x50(%rsp)
14050332a:     	leaq	0x24eaa27(%rip), %rcx   # 0x1429edd58
140503331:     	callq	0x14067dd80 <.text+0x67cd80>
140503336:     	movups	(%rax), %xmm0
140503339:     	movl	0x10(%rax), %eax
14050333c:     	movq	$0x1, 0x88(%rsp)
140503348:     	leaq	0x24ea9d9(%rip), %rcx   # 0x1429edd28
14050334f:     	movq	%rcx, 0x90(%rsp)
140503357:     	movq	$0x14, 0x98(%rsp)
140503363:     	leaq	0x24ea92e(%rip), %rdx   # 0x1429edc98
14050336a:     	movq	%rdx, 0xa8(%rsp)
140503372:     	movq	$0x2, 0xb0(%rsp)
14050337e:     	movq	%r14, 0xb8(%rsp)
140503386:     	movq	$0x2, 0xc0(%rsp)
140503392:     	movq	$0x0, 0xc8(%rsp)
14050339e:     	movq	$0x0, 0x58(%rsp)
1405033a7:     	movq	%rcx, 0x60(%rsp)
1405033ac:     	movq	$0x14, 0x68(%rsp)
1405033b5:     	movq	$0x0, 0x70(%rsp)
1405033be:     	movups	%xmm0, 0x78(%rsp)
1405033c3:     	movl	$0x1, 0xa0(%rsp)
1405033ce:     	movl	%eax, 0xa4(%rsp)
1405033d5:     	leaq	0x27(%rsp), %rcx
1405033da:     	leaq	0x58(%rsp), %rdx
1405033df:     	callq	0x140d4a330 <.text+0xd49330>
1405033e4:     	movups	0x28(%rsp), %xmm0
1405033e9:     	movq	0x18(%rbx), %rbx
1405033ed:     	movups	%xmm0, 0x38(%rsp)
1405033f2:     	leaq	0x38(%rsp), %rcx
1405033f7:     	callq	0x140683c10 <.text+0x682c10>
1405033fc:     	movq	%rax, 0x58(%rsp)
140503401:     	movq	%r13, 0x60(%rsp)
140503406:     	movq	%rbp, 0x68(%rsp)
14050340b:     	leaq	0x58(%rsp), %rcx
140503410:     	callq	0x1405bdf20 <.text+0x5bcf20>
140503415:     	movq	%rbx, 0x58(%rsp)
14050341a:     	movq	%rax, 0x60(%rsp)
14050341f:     	leaq	0x58(%rsp), %rcx
140503424:     	callq	0x1405bd910 <.text+0x5bc910>
140503429:     	leaq	0x58(%rsp), %rbx
14050342e:     	movq	%rbx, %rcx
140503431:     	movq	%rdi, %rdx
140503434:     	movq	%rax, %r8
140503437:     	callq	0x1406861b0 <.text+0x6851b0>
14050343c:     	movq	%rsi, %rcx
14050343f:     	movq	%rbx, %rdx
140503442:     	callq	0x1405047f0 <.text+0x5037f0>
140503447:     	nop
140503448:     	addq	$0xd8, %rsp
14050344f:     	popq	%rbx
140503450:     	popq	%rbp
140503451:     	popq	%rdi
140503452:     	popq	%rsi
140503453:     	popq	%r12
140503455:     	popq	%r13
140503457:     	popq	%r14
140503459:     	popq	%r15
14050345b:     	retq
