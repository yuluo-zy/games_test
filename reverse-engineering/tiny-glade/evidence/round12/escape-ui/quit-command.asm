
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140526fb0:     	pushq	%rbp
140526fb1:     	pushq	%r15
140526fb3:     	pushq	%r14
140526fb5:     	pushq	%rsi
140526fb6:     	pushq	%rdi
140526fb7:     	pushq	%rbx
140526fb8:     	subq	$0x248, %rsp            # imm = 0x248
140526fbf:     	leaq	0x80(%rsp), %rbp
140526fc7:     	movq	$-0x2, 0x1c0(%rbp)
140526fd2:     	movq	%rcx, %rsi
140526fd5:     	leaq	0x38(%rcx), %rdi
140526fd9:     	leaq	0x5a8(%rcx), %rax
140526fe0:     	movq	%rdi, 0x1a0(%rbp)
140526fe7:     	movq	%rax, 0x1a8(%rbp)
140526fee:     	cmpq	$0x0, 0x88(%rcx)
140526ff6:     	movabsq	$-0x1b61c2df6f2309c7, %rax # imm = 0xE49E3D2090DCF639
140527000:     	movabsq	$-0x4c7973b4084c4af, %rcx # imm = 0xFB3868C4BF7B3B51
14052700a:     	je	0x1405270aa <.text+0x5260aa>
140527010:     	movq	0x70(%rsi), %rdx
140527014:     	movq	0x78(%rsi), %r8
140527018:     	movq	%r8, %r9
14052701b:     	andq	%rax, %r9
14052701e:     	xorl	%r10d, %r10d
140527021:     	movdqa	0x243a577(%rip), %xmm0  # 0x1429615a0
140527029:     	pcmpeqd	%xmm1, %xmm1
14052702d:     	movdqu	(%rdx,%r9), %xmm2
140527033:     	movdqa	%xmm2, %xmm3
140527037:     	pcmpeqb	%xmm0, %xmm3
14052703b:     	pmovmskb	%xmm3, %r11d
140527040:     	testl	%r11d, %r11d
140527043:     	jne	0x14052707d <.text+0x52607d>
140527045:     	pcmpeqb	%xmm1, %xmm2
140527049:     	pmovmskb	%xmm2, %r11d
14052704e:     	testl	%r11d, %r11d
140527051:     	jne	0x1405270aa <.text+0x5260aa>
140527053:     	addq	%r10, %r9
140527056:     	addq	$0x10, %r9
14052705a:     	addq	$0x10, %r10
14052705e:     	andq	%r8, %r9
140527061:     	jmp	0x14052702d <.text+0x52602d>
140527063:     	nopw	%cs:(%rax,%rax)
140527070:     	leal	-0x1(%r11), %ebx
140527074:     	andw	%r11w, %bx
140527078:     	movl	%ebx, %r11d
14052707b:     	je	0x140527045 <.text+0x526045>
14052707d:     	tzcntl	%r11d, %ebx
140527082:     	addq	%r9, %rbx
140527085:     	andq	%r8, %rbx
140527088:     	negq	%rbx
14052708b:     	leaq	(%rbx,%rbx,2), %rbx
14052708f:     	cmpq	%rcx, -0x18(%rdx,%rbx,8)
140527094:     	jne	0x140527070 <.text+0x526070>
140527096:     	cmpq	%rax, -0x10(%rdx,%rbx,8)
14052709b:     	jne	0x140527070 <.text+0x526070>
14052709d:     	leaq	(%rdx,%rbx,8), %rax
1405270a1:     	movq	-0x8(%rax), %rbx
1405270a5:     	jmp	0x14052738e <.text+0x52638e>
1405270aa:     	movzbl	0x98(%rsi), %edx
1405270b1:     	movq	0xd8(%rsi), %rdx
1405270b8:     	movq	0xe0(%rsi), %r9
1405270bf:     	movq	%r9, %r10
1405270c2:     	andq	%rax, %r10
1405270c5:     	xorl	%r11d, %r11d
1405270c8:     	movdqa	0x243a4d0(%rip), %xmm0  # 0x1429615a0
1405270d0:     	pcmpeqd	%xmm1, %xmm1
1405270d4:     	movdqu	(%rdx,%r10), %xmm2
1405270da:     	movdqa	%xmm2, %xmm3
1405270de:     	pcmpeqb	%xmm0, %xmm3
1405270e2:     	pmovmskb	%xmm3, %ebx
1405270e6:     	testl	%ebx, %ebx
1405270e8:     	jne	0x14052711d <.text+0x52611d>
1405270ea:     	pcmpeqb	%xmm1, %xmm2
1405270ee:     	pmovmskb	%xmm2, %r8d
1405270f3:     	testl	%r8d, %r8d
1405270f6:     	jne	0x1405271e7 <.text+0x5261e7>
1405270fc:     	addq	%r11, %r10
1405270ff:     	addq	$0x10, %r10
140527103:     	addq	$0x10, %r11
140527107:     	andq	%r9, %r10
14052710a:     	jmp	0x1405270d4 <.text+0x5260d4>
14052710c:     	nopl	(%rax)
140527110:     	leal	-0x1(%rbx), %r8d
140527114:     	andw	%bx, %r8w
140527118:     	movl	%r8d, %ebx
14052711b:     	je	0x1405270ea <.text+0x5260ea>
14052711d:     	tzcntl	%ebx, %r14d
140527122:     	addq	%r10, %r14
140527125:     	andq	%r9, %r14
140527128:     	movq	%r14, %r8
14052712b:     	negq	%r8
14052712e:     	leaq	(,%r14,8), %r15
140527136:     	shlq	$0x7, %r8
14052713a:     	subq	%r15, %r8
14052713d:     	cmpq	%rcx, -0x88(%rdx,%r8)
140527145:     	jne	0x140527110 <.text+0x526110>
140527147:     	cmpq	%rax, -0x80(%rdx,%r8)
14052714c:     	jne	0x140527110 <.text+0x526110>
14052714e:     	movq	%r14, %r10
140527151:     	shlq	$0x7, %r10
140527155:     	leaq	(%r10,%r14,8), %r11
140527159:     	sarq	$0x3, %r11
14052715d:     	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
140527167:     	imulq	%r11, %r10
14052716b:     	leaq	-0x10(%r10), %r11
14052716f:     	andq	%r9, %r11
140527172:     	movdqu	(%rdx,%r11), %xmm0
140527178:     	pcmpeqd	%xmm1, %xmm1
14052717c:     	pcmpeqb	%xmm1, %xmm0
140527180:     	pmovmskb	%xmm0, %ebx
140527184:     	movdqu	(%rdx,%r10), %xmm0
14052718a:     	pcmpeqb	%xmm1, %xmm0
14052718e:     	pmovmskb	%xmm0, %r9d
140527193:     	testl	%ebx, %ebx
140527195:     	je	0x1405271a0 <.text+0x5261a0>
140527197:     	bsrw	%bx, %bx
14052719b:     	xorl	$0xf, %ebx
14052719e:     	jmp	0x1405271a4 <.text+0x5261a4>
1405271a0:     	movw	$0x10, %bx
1405271a4:     	addq	%rdx, %r8
1405271a7:     	orl	$0x10000, %r9d          # imm = 0x10000
1405271ae:     	tzcntl	%r9d, %r9d
1405271b3:     	addl	%r9d, %ebx
1405271b6:     	movb	$-0x80, %r9b
1405271b9:     	cmpw	$0xf, %bx
1405271bd:     	ja	0x1405271c9 <.text+0x5261c9>
1405271bf:     	incq	0xe8(%rsi)
1405271c6:     	movb	$-0x1, %r9b
1405271c9:     	movb	%r9b, (%rdx,%r10)
1405271cd:     	movb	%r9b, 0x10(%rdx,%r11)
1405271d2:     	decq	0xf0(%rsi)
1405271d9:     	movq	-0x78(%r8), %rdx
1405271dd:     	cmpq	$0x2, %rdx
1405271e1:     	jne	0x14052731e <.text+0x52631e>
1405271e7:     	movq	0x5a8(%rsi), %rbx
1405271ee:     	leaq	0x1(%rbx), %r14
1405271f2:     	movq	%r14, 0x5a8(%rsi)
1405271f9:     	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
140527203:     	movq	%rdx, 0x178(%rbp)
14052720a:     	leaq	0x242b339(%rip), %rdx   # 0x14295254a
140527211:     	movq	%rdx, 0x180(%rbp)
140527218:     	movq	$0x2f, 0x188(%rbp)
140527223:     	movw	$0x100, 0x198(%rbp)     # imm = 0x100
14052722c:     	movq	$0x1, 0x140(%rbp)
140527237:     	movq	%rcx, 0x148(%rbp)
14052723e:     	movq	%rax, 0x150(%rbp)
140527245:     	movq	$0x1, 0x168(%rbp)
140527250:     	movq	$0x0, 0x170(%rbp)
14052725b:     	movq	$0x0, 0x190(%rbp)
140527266:     	movb	$0x1, 0x19a(%rbp)
14052726d:     	movq	$0x0, 0x158(%rbp)
140527278:     	leaq	-0x60(%rbp), %rcx
14052727c:     	leaq	0x140(%rbp), %r8
140527283:     	movq	%rbx, %rdx
140527286:     	callq	0x140d3f870 <.text+0xd3e870>
14052728b:     	cmpq	%r14, 0x48(%rsi)
14052728f:     	jae	0x1405272aa <.text+0x5262aa>
140527291:     	movb	$0x1, 0x1bf(%rbp)
140527298:     	leaq	0x24cad19(%rip), %r8    # 0x1429f1fb8
14052729f:     	movq	%rdi, %rcx
1405272a2:     	movq	%r14, %rdx
1405272a5:     	callq	0x1402b5690 <.text+0x2b4690>
1405272aa:     	movq	0x40(%rsi), %r14
1405272ae:     	imulq	$0xd0, %rbx, %r15
1405272b5:     	leaq	(%r14,%r15), %rax
1405272b9:     	movq	%rax, 0x1b0(%rbp)
1405272c0:     	leaq	0x70(%rbp), %rcx
1405272c4:     	leaq	-0x60(%rbp), %rdx
1405272c8:     	movl	$0xd0, %r8d
1405272ce:     	callq	0x142923c00 <.text+0x2922c00>
1405272d3:     	cmpl	$0x2, (%r14,%r15)
1405272d8:     	je	0x1405272e6 <.text+0x5262e6>
1405272da:     	movq	0x1b0(%rbp), %rcx
1405272e1:     	callq	0x1405f0b50 <.text+0x5efb50>
1405272e6:     	leaq	0x70(%rbp), %rdx
1405272ea:     	movl	$0xd0, %r8d
1405272f0:     	movq	0x1b0(%rbp), %rcx
1405272f7:     	callq	0x142923c00 <.text+0x2922c00>
1405272fc:     	leaq	0x70(%rsi), %rcx
140527300:     	movabsq	$-0x4c7973b4084c4af, %rdx # imm = 0xFB3868C4BF7B3B51
14052730a:     	movabsq	$-0x1b61c2df6f2309c7, %r8 # imm = 0xE49E3D2090DCF639
140527314:     	movq	%rbx, %r9
140527317:     	callq	0x140082c20 <.text+0x81c20>
14052731c:     	jmp	0x14052738e <.text+0x52638e>
14052731e:     	movups	-0x10(%r8), %xmm0
140527323:     	movups	%xmm0, 0xd8(%rbp)
14052732a:     	movups	-0x20(%r8), %xmm0
14052732f:     	movups	%xmm0, 0xc8(%rbp)
140527336:     	movups	-0x30(%r8), %xmm0
14052733b:     	movups	%xmm0, 0xb8(%rbp)
140527342:     	movdqu	-0x70(%r8), %xmm0
140527348:     	movdqu	-0x60(%r8), %xmm1
14052734e:     	movdqu	-0x50(%r8), %xmm2
140527354:     	movdqu	-0x40(%r8), %xmm3
14052735a:     	movdqu	%xmm3, 0xa8(%rbp)
140527362:     	movdqu	%xmm2, 0x98(%rbp)
14052736a:     	movdqu	%xmm1, 0x88(%rbp)
140527372:     	movdqu	%xmm0, 0x78(%rbp)
140527377:     	movq	%rdx, 0x70(%rbp)
14052737b:     	leaq	0x70(%rbp), %rcx
14052737f:     	leaq	0x1a0(%rbp), %rdx
140527386:     	callq	0x140d3fa40 <.text+0xd3ea40>
14052738b:     	movq	%rax, %rbx
14052738e:     	movl	0x5b0(%rsi), %r14d
140527395:     	movq	%rsi, %rcx
140527398:     	callq	0x140cf8250 <.text+0xcf7250>
14052739d:     	leaq	0xf8(%rsi), %rax
1405273a4:     	movq	%rbx, -0x60(%rbp)
1405273a8:     	addq	$0x1d8, %rsi            # imm = 0x1D8
1405273af:     	movq	%rdi, 0x70(%rbp)
1405273b3:     	leaq	-0x60(%rbp), %rcx
1405273b7:     	movq	%rcx, 0x78(%rbp)
1405273bb:     	movq	%rax, 0x80(%rbp)
1405273c2:     	leaq	0x70(%rbp), %r8
1405273c6:     	movq	%rsi, %rcx
1405273c9:     	movq	%rbx, %rdx
1405273cc:     	callq	0x140509180 <.text+0x508180>
1405273d1:     	movq	%rax, %rsi
1405273d4:     	cmpq	$0x0, 0x20(%rax)
1405273d9:     	je	0x1405273eb <.text+0x5263eb>
1405273db:     	leaq	0x70(%rbp), %r8
1405273df:     	movq	%rsi, %rcx
1405273e2:     	xorl	%edx, %edx
1405273e4:     	callq	0x140cff670 <.text+0xcfe670>
1405273e9:     	jmp	0x140527415 <.text+0x526415>
1405273eb:     	cmpq	$0x0, 0x18(%rsi)
1405273f0:     	je	0x140527429 <.text+0x526429>
1405273f2:     	xorl	%ecx, %ecx
1405273f4:     	leaq	0x1(%rcx), %rax
1405273f8:     	movq	%rax, 0x20(%rsi)
1405273fc:     	movq	0x8(%rsi), %r8
140527400:     	imulq	%r8, %rcx
140527404:     	addq	0x10(%rsi), %rcx
140527408:     	leaq	0x70(%rbp), %rdx
14052740c:     	callq	0x142923c00 <.text+0x2922c00>
140527411:     	movl	%r14d, 0x58(%rsi)
140527415:     	movl	%r14d, 0x5c(%rsi)
140527419:     	addq	$0x248, %rsp            # imm = 0x248
140527420:     	popq	%rbx
140527421:     	popq	%rdi
140527422:     	popq	%rsi
140527423:     	popq	%r14
140527425:     	popq	%r15
140527427:     	popq	%rbp
140527428:     	retq
140527429:     	movl	$0x1, %edx
14052742e:     	movq	%rsi, %rcx
140527431:     	callq	0x1428cec40 <.text+0x28cdc40>
140527436:     	movq	0x20(%rsi), %rcx
14052743a:     	jmp	0x1405273f4 <.text+0x5263f4>
14052743c:     	nopl	(%rax)
140527440:     	movq	%rdx, 0x10(%rsp)
140527445:     	pushq	%rbp
140527446:     	pushq	%r15
140527448:     	pushq	%r14
14052744a:     	pushq	%rsi
14052744b:     	pushq	%rdi
14052744c:     	pushq	%rbx
14052744d:     	subq	$0x28, %rsp
140527451:     	leaq	0x80(%rdx), %rbp
140527458:     	leaq	0x70(%rbp), %rdx
14052745c:     	movl	$0xd0, %r8d
140527462:     	movq	0x1b0(%rbp), %rcx
140527469:     	callq	0x142923c00 <.text+0x2922c00>
14052746e:     	movb	$0x0, 0x1bf(%rbp)
140527475:     	addq	$0x28, %rsp
140527479:     	popq	%rbx
14052747a:     	popq	%rdi
14052747b:     	popq	%rsi
14052747c:     	popq	%r14
14052747e:     	popq	%r15
140527480:     	popq	%rbp
140527481:     	retq
140527482:     	nopw	%cs:(%rax,%rax)
140527490:     	movq	%rdx, 0x10(%rsp)
140527495:     	pushq	%rbp
140527496:     	pushq	%r15
140527498:     	pushq	%r14
14052749a:     	pushq	%rsi
14052749b:     	pushq	%rdi
14052749c:     	pushq	%rbx
14052749d:     	subq	$0x28, %rsp
1405274a1:     	leaq	0x80(%rdx), %rbp
1405274a8:     	cmpb	$0x0, 0x1bf(%rbp)
1405274af:     	jne	0x1405274be <.text+0x5264be>
1405274b1:     	addq	$0x28, %rsp
1405274b5:     	popq	%rbx
1405274b6:     	popq	%rdi
1405274b7:     	popq	%rsi
1405274b8:     	popq	%r14
1405274ba:     	popq	%r15
1405274bc:     	popq	%rbp
1405274bd:     	retq
1405274be:     	leaq	-0x60(%rbp), %rcx
1405274c2:     	callq	0x1405f0b50 <.text+0x5efb50>
1405274c7:     	jmp	0x1405274b1 <.text+0x5264b1>
