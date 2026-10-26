
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a65fe0:     	pushq	%rbp
140a65fe1:     	pushq	%rsi
140a65fe2:     	pushq	%rdi
140a65fe3:     	subq	$0xd0, %rsp
140a65fea:     	leaq	0x80(%rsp), %rbp
140a65ff2:     	movq	$-0x2, 0x48(%rbp)
140a65ffa:     	movq	%r8, %rax
140a65ffd:     	movq	%rdx, %rdi
140a66000:     	movq	%rcx, %rsi
140a66003:     	movq	$0x13, 0x20(%rsp)
140a6600c:     	leaq	0x206e689(%rip), %r8    # 0x142ad469c
140a66013:     	leaq	-0x10(%rbp), %rcx
140a66017:     	movl	$0x17, %r9d
140a6601d:     	movq	%rax, %rdx
140a66020:     	callq	0x1413132c0 <.text+0x13122c0>
140a66025:     	movzbl	-0x10(%rbp), %eax
140a66029:     	cmpb	$0x32, %al
140a6602b:     	jne	0x140a66427 <.text+0xa65427>
140a66031:     	movq	0x8(%rbp), %rax
140a66035:     	movups	-0x8(%rbp), %xmm0
140a66039:     	movaps	%xmm0, -0x30(%rbp)
140a6603d:     	movq	%rax, -0x20(%rbp)
140a66041:     	movb	$0x1, 0x47(%rbp)
140a66045:     	movq	%rdi, 0x20(%rsp)
140a6604a:     	leaq	0x206e662(%rip), %r8    # 0x142ad46b3
140a66051:     	leaq	-0x10(%rbp), %rcx
140a66055:     	leaq	-0x30(%rbp), %rdx
140a66059:     	movl	$0x11, %r9d
140a6605f:     	callq	0x141311e70 <.text+0x1310e70>
140a66064:     	movzbl	-0x10(%rbp), %eax
140a66068:     	cmpb	$0x32, %al
140a6606a:     	jne	0x140a66463 <.text+0xa65463>
140a66070:     	leaq	0x1(%rdi), %rax
140a66074:     	movb	$0x1, 0x47(%rbp)
140a66078:     	movq	%rax, 0x20(%rsp)
140a6607d:     	leaq	0x206e640(%rip), %r8    # 0x142ad46c4
140a66084:     	leaq	-0x10(%rbp), %rcx
140a66088:     	leaq	-0x30(%rbp), %rdx
140a6608c:     	movl	$0x15, %r9d
140a66092:     	callq	0x141311e70 <.text+0x1310e70>
140a66097:     	movzbl	-0x10(%rbp), %eax
140a6609b:     	cmpb	$0x32, %al
140a6609d:     	jne	0x140a66463 <.text+0xa65463>
140a660a3:     	leaq	0x2(%rdi), %rax
140a660a7:     	movb	$0x1, 0x47(%rbp)
140a660ab:     	movq	%rax, 0x20(%rsp)
140a660b0:     	leaq	0x206e622(%rip), %r8    # 0x142ad46d9
140a660b7:     	leaq	-0x10(%rbp), %rcx
140a660bb:     	leaq	-0x30(%rbp), %rdx
140a660bf:     	movl	$0x13, %r9d
140a660c5:     	callq	0x141311e70 <.text+0x1310e70>
140a660ca:     	movzbl	-0x10(%rbp), %eax
140a660ce:     	cmpb	$0x32, %al
140a660d0:     	jne	0x140a66463 <.text+0xa65463>
140a660d6:     	leaq	0x3(%rdi), %rax
140a660da:     	movb	$0x1, 0x47(%rbp)
140a660de:     	movq	%rax, 0x20(%rsp)
140a660e3:     	leaq	0x206e602(%rip), %r8    # 0x142ad46ec
140a660ea:     	leaq	-0x10(%rbp), %rcx
140a660ee:     	leaq	-0x30(%rbp), %rdx
140a660f2:     	movl	$0x16, %r9d
140a660f8:     	callq	0x141311e70 <.text+0x1310e70>
140a660fd:     	movzbl	-0x10(%rbp), %eax
140a66101:     	cmpb	$0x32, %al
140a66103:     	jne	0x140a66463 <.text+0xa65463>
140a66109:     	leaq	0x4(%rdi), %rax
140a6610d:     	movb	$0x1, 0x47(%rbp)
140a66111:     	movq	%rax, 0x20(%rsp)
140a66116:     	leaq	0x206e5e5(%rip), %r8    # 0x142ad4702
140a6611d:     	leaq	-0x10(%rbp), %rcx
140a66121:     	leaq	-0x30(%rbp), %rdx
140a66125:     	movl	$0x14, %r9d
140a6612b:     	callq	0x141311e70 <.text+0x1310e70>
140a66130:     	movzbl	-0x10(%rbp), %eax
140a66134:     	cmpb	$0x32, %al
140a66136:     	jne	0x140a66463 <.text+0xa65463>
140a6613c:     	leaq	0x5(%rdi), %rax
140a66140:     	movb	$0x1, 0x47(%rbp)
140a66144:     	movq	%rax, 0x20(%rsp)
140a66149:     	leaq	0x206e5c6(%rip), %r8    # 0x142ad4716
140a66150:     	leaq	-0x10(%rbp), %rcx
140a66154:     	leaq	-0x30(%rbp), %rdx
140a66158:     	movl	$0x10, %r9d
140a6615e:     	callq	0x141311e70 <.text+0x1310e70>
140a66163:     	movzbl	-0x10(%rbp), %eax
140a66167:     	cmpb	$0x32, %al
140a66169:     	jne	0x140a66463 <.text+0xa65463>
140a6616f:     	leaq	0x6(%rdi), %rax
140a66173:     	movb	$0x1, 0x47(%rbp)
140a66177:     	movq	%rax, 0x20(%rsp)
140a6617c:     	leaq	0x206e5a3(%rip), %r8    # 0x142ad4726
140a66183:     	leaq	-0x10(%rbp), %rcx
140a66187:     	leaq	-0x30(%rbp), %rdx
140a6618b:     	movl	$0x1a, %r9d
140a66191:     	callq	0x141311e70 <.text+0x1310e70>
140a66196:     	movzbl	-0x10(%rbp), %eax
140a6619a:     	cmpb	$0x32, %al
140a6619c:     	jne	0x140a66463 <.text+0xa65463>
140a661a2:     	leaq	0x7(%rdi), %rax
140a661a6:     	movb	$0x1, 0x47(%rbp)
140a661aa:     	movq	%rax, 0x20(%rsp)
140a661af:     	leaq	0x206e58a(%rip), %r8    # 0x142ad4740
140a661b6:     	leaq	-0x10(%rbp), %rcx
140a661ba:     	leaq	-0x30(%rbp), %rdx
140a661be:     	movl	$0x12, %r9d
140a661c4:     	callq	0x141311e70 <.text+0x1310e70>
140a661c9:     	movzbl	-0x10(%rbp), %eax
140a661cd:     	cmpb	$0x32, %al
140a661cf:     	jne	0x140a66463 <.text+0xa65463>
140a661d5:     	leaq	0x8(%rdi), %rax
140a661d9:     	movb	$0x1, 0x47(%rbp)
140a661dd:     	movq	%rax, 0x20(%rsp)
140a661e2:     	leaq	0x206e569(%rip), %r8    # 0x142ad4752
140a661e9:     	leaq	-0x10(%rbp), %rcx
140a661ed:     	leaq	-0x30(%rbp), %rdx
140a661f1:     	movl	$0x11, %r9d
140a661f7:     	callq	0x141311e70 <.text+0x1310e70>
140a661fc:     	movzbl	-0x10(%rbp), %eax
140a66200:     	cmpb	$0x32, %al
140a66202:     	jne	0x140a66463 <.text+0xa65463>
140a66208:     	leaq	0x9(%rdi), %rax
140a6620c:     	movb	$0x1, 0x47(%rbp)
140a66210:     	movq	%rax, 0x20(%rsp)
140a66215:     	leaq	0x206e547(%rip), %r8    # 0x142ad4763
140a6621c:     	leaq	-0x10(%rbp), %rcx
140a66220:     	leaq	-0x30(%rbp), %rdx
140a66224:     	movl	$0xe, %r9d
140a6622a:     	callq	0x141311e70 <.text+0x1310e70>
140a6622f:     	movzbl	-0x10(%rbp), %eax
140a66233:     	cmpb	$0x32, %al
140a66235:     	jne	0x140a66463 <.text+0xa65463>
140a6623b:     	leaq	0xa(%rdi), %rax
140a6623f:     	movb	$0x1, 0x47(%rbp)
140a66243:     	movq	%rax, 0x20(%rsp)
140a66248:     	leaq	0x206e522(%rip), %r8    # 0x142ad4771
140a6624f:     	leaq	-0x10(%rbp), %rcx
140a66253:     	leaq	-0x30(%rbp), %rdx
140a66257:     	movl	$0xd, %r9d
140a6625d:     	callq	0x141311e70 <.text+0x1310e70>
140a66262:     	movzbl	-0x10(%rbp), %eax
140a66266:     	cmpb	$0x32, %al
140a66268:     	jne	0x140a66463 <.text+0xa65463>
140a6626e:     	leaq	0xb(%rdi), %rax
140a66272:     	movb	$0x1, 0x47(%rbp)
140a66276:     	movq	%rax, 0x20(%rsp)
140a6627b:     	leaq	0x206e4fc(%rip), %r8    # 0x142ad477e
140a66282:     	leaq	-0x10(%rbp), %rcx
140a66286:     	leaq	-0x30(%rbp), %rdx
140a6628a:     	movl	$0x13, %r9d
140a66290:     	callq	0x141311e70 <.text+0x1310e70>
140a66295:     	movzbl	-0x10(%rbp), %eax
140a66299:     	cmpb	$0x32, %al
140a6629b:     	jne	0x140a66463 <.text+0xa65463>
140a662a1:     	leaq	0xc(%rdi), %rax
140a662a5:     	movb	$0x1, 0x47(%rbp)
140a662a9:     	movq	%rax, 0x20(%rsp)
140a662ae:     	leaq	0x206e4dc(%rip), %r8    # 0x142ad4791
140a662b5:     	leaq	-0x10(%rbp), %rcx
140a662b9:     	leaq	-0x30(%rbp), %rdx
140a662bd:     	movl	$0x16, %r9d
140a662c3:     	callq	0x141311e70 <.text+0x1310e70>
140a662c8:     	movzbl	-0x10(%rbp), %eax
140a662cc:     	cmpb	$0x32, %al
140a662ce:     	jne	0x140a66463 <.text+0xa65463>
140a662d4:     	leaq	0xd(%rdi), %rax
140a662d8:     	movb	$0x1, 0x47(%rbp)
140a662dc:     	movq	%rax, 0x20(%rsp)
140a662e1:     	leaq	0x206e4bf(%rip), %r8    # 0x142ad47a7
140a662e8:     	leaq	-0x10(%rbp), %rcx
140a662ec:     	leaq	-0x30(%rbp), %rdx
140a662f0:     	movl	$0x1b, %r9d
140a662f6:     	callq	0x141311e70 <.text+0x1310e70>
140a662fb:     	movzbl	-0x10(%rbp), %eax
140a662ff:     	cmpb	$0x32, %al
140a66301:     	jne	0x140a66463 <.text+0xa65463>
140a66307:     	leaq	0xe(%rdi), %rax
140a6630b:     	movb	$0x1, 0x47(%rbp)
140a6630f:     	movq	%rax, 0x20(%rsp)
140a66314:     	leaq	0x206e4a7(%rip), %r8    # 0x142ad47c2
140a6631b:     	leaq	-0x10(%rbp), %rcx
140a6631f:     	leaq	-0x30(%rbp), %rdx
140a66323:     	movl	$0x1c, %r9d
140a66329:     	callq	0x141311e70 <.text+0x1310e70>
140a6632e:     	movzbl	-0x10(%rbp), %eax
140a66332:     	cmpb	$0x32, %al
140a66334:     	jne	0x140a66463 <.text+0xa65463>
140a6633a:     	leaq	0xf(%rdi), %rax
140a6633e:     	movb	$0x1, 0x47(%rbp)
140a66342:     	movq	%rax, 0x20(%rsp)
140a66347:     	leaq	0x206e490(%rip), %r8    # 0x142ad47de
140a6634e:     	leaq	-0x10(%rbp), %rcx
140a66352:     	leaq	-0x30(%rbp), %rdx
140a66356:     	movl	$0x14, %r9d
140a6635c:     	callq	0x141311e70 <.text+0x1310e70>
140a66361:     	movzbl	-0x10(%rbp), %eax
140a66365:     	cmpb	$0x32, %al
140a66367:     	jne	0x140a66463 <.text+0xa65463>
140a6636d:     	leaq	0x10(%rdi), %rax
140a66371:     	movb	$0x1, 0x47(%rbp)
140a66375:     	movq	%rax, 0x20(%rsp)
140a6637a:     	leaq	0x206e471(%rip), %r8    # 0x142ad47f2
140a66381:     	leaq	-0x10(%rbp), %rcx
140a66385:     	leaq	-0x30(%rbp), %rdx
140a66389:     	movl	$0x17, %r9d
140a6638f:     	callq	0x141311e70 <.text+0x1310e70>
140a66394:     	movzbl	-0x10(%rbp), %eax
140a66398:     	cmpb	$0x32, %al
140a6639a:     	jne	0x140a66463 <.text+0xa65463>
140a663a0:     	leaq	0x11(%rdi), %rax
140a663a4:     	movb	$0x1, 0x47(%rbp)
140a663a8:     	movq	%rax, 0x20(%rsp)
140a663ad:     	leaq	0x206e455(%rip), %r8    # 0x142ad4809
140a663b4:     	leaq	-0x10(%rbp), %rcx
140a663b8:     	leaq	-0x30(%rbp), %rdx
140a663bc:     	movl	$0x15, %r9d
140a663c2:     	callq	0x141311e70 <.text+0x1310e70>
140a663c7:     	movzbl	-0x10(%rbp), %eax
140a663cb:     	cmpb	$0x32, %al
140a663cd:     	jne	0x140a66463 <.text+0xa65463>
140a663d3:     	addq	$0x12, %rdi
140a663d7:     	movb	$0x1, 0x47(%rbp)
140a663db:     	movq	%rdi, 0x20(%rsp)
140a663e0:     	leaq	0x206e437(%rip), %r8    # 0x142ad481e
140a663e7:     	leaq	-0x10(%rbp), %rcx
140a663eb:     	leaq	-0x30(%rbp), %rdx
140a663ef:     	movl	$0x14, %r9d
140a663f5:     	callq	0x141311e70 <.text+0x1310e70>
140a663fa:     	movzbl	-0x10(%rbp), %eax
140a663fe:     	cmpb	$0x32, %al
140a66400:     	jne	0x140a66463 <.text+0xa65463>
140a66402:     	movq	-0x20(%rbp), %rax
140a66406:     	movq	%rax, (%rbp)
140a6640a:     	movaps	-0x30(%rbp), %xmm0
140a6640e:     	movaps	%xmm0, -0x10(%rbp)
140a66412:     	movb	$0x0, 0x47(%rbp)
140a66416:     	leaq	-0x10(%rbp), %rdx
140a6641a:     	movq	%rsi, %rcx
140a6641d:     	callq	0x141312a40 <.text+0x1311a40>
140a66422:     	jmp	0x140a664ac <.text+0xa654ac>
140a66427:     	movups	-0xf(%rbp), %xmm0
140a6642b:     	movups	(%rbp), %xmm1
140a6642f:     	movups	0x10(%rbp), %xmm2
140a66433:     	movups	0x20(%rbp), %xmm3
140a66437:     	movups	%xmm1, -0x41(%rbp)
140a6643b:     	movaps	%xmm0, -0x50(%rbp)
140a6643f:     	movq	0x30(%rbp), %rcx
140a66443:     	movq	%rcx, 0x40(%rsi)
140a66447:     	movups	%xmm3, 0x30(%rsi)
140a6644b:     	movups	%xmm2, 0x20(%rsi)
140a6644f:     	movups	-0x41(%rbp), %xmm0
140a66453:     	movups	%xmm0, 0x10(%rsi)
140a66457:     	movaps	-0x50(%rbp), %xmm0
140a6645b:     	movups	%xmm0, 0x1(%rsi)
140a6645f:     	movb	%al, (%rsi)
140a66461:     	jmp	0x140a664ac <.text+0xa654ac>
140a66463:     	movq	0x30(%rbp), %rcx
140a66467:     	movq	%rcx, 0x40(%rsi)
140a6646b:     	movups	-0xf(%rbp), %xmm0
140a6646f:     	movups	0x1(%rbp), %xmm1
140a66473:     	movups	0x11(%rbp), %xmm2
140a66477:     	movups	0x21(%rbp), %xmm3
140a6647b:     	movups	%xmm3, 0x31(%rsi)
140a6647f:     	movups	%xmm2, 0x21(%rsi)
140a66483:     	movups	%xmm1, 0x11(%rsi)
140a66487:     	movups	%xmm0, 0x1(%rsi)
140a6648b:     	movb	%al, (%rsi)
140a6648d:     	movq	-0x28(%rbp), %rax
140a66491:     	cmpl	$0x1, (%rax)
140a66494:     	jne	0x140a664ac <.text+0xa654ac>
140a66496:     	movq	0x8(%rax), %rcx
140a6649a:     	incq	%rcx
140a6649d:     	movq	$-0x1, %rdx
140a664a4:     	cmovneq	%rcx, %rdx
140a664a8:     	movq	%rdx, 0x8(%rax)
140a664ac:     	movq	%rsi, %rax
140a664af:     	addq	$0xd0, %rsp
140a664b6:     	popq	%rdi
140a664b7:     	popq	%rsi
140a664b8:     	popq	%rbp
140a664b9:     	retq
140a664ba:     	nopw	(%rax,%rax)
140a664c0:     	movq	%rdx, 0x10(%rsp)
140a664c5:     	pushq	%rbp
140a664c6:     	pushq	%rsi
140a664c7:     	pushq	%rdi
140a664c8:     	subq	$0x30, %rsp
140a664cc:     	leaq	0x80(%rdx), %rbp
140a664d3:     	cmpb	$0x0, 0x47(%rbp)
140a664d7:     	je	0x140a664f8 <.text+0xa654f8>
140a664d9:     	movq	-0x28(%rbp), %rax
140a664dd:     	cmpl	$0x1, (%rax)
140a664e0:     	jne	0x140a664f8 <.text+0xa654f8>
140a664e2:     	movq	0x8(%rax), %rcx
140a664e6:     	incq	%rcx
140a664e9:     	movq	$-0x1, %rdx
140a664f0:     	cmovneq	%rcx, %rdx
140a664f4:     	movq	%rdx, 0x8(%rax)
140a664f8:     	addq	$0x30, %rsp
140a664fc:     	popq	%rdi
140a664fd:     	popq	%rsi
140a664fe:     	popq	%rbp
140a664ff:     	retq
140a66500:     	pushq	%r15
140a66502:     	pushq	%r14
140a66504:     	pushq	%rsi
140a66505:     	pushq	%rdi
140a66506:     	pushq	%rbp
140a66507:     	pushq	%rbx
140a66508:     	subq	$0x28, %rsp
140a6650c:     	movq	%r8, %rdi
140a6650f:     	movq	%rdx, %rbx
140a66512:     	movq	%rcx, %rsi
140a66515:     	cmpq	$0x11, %r8
140a66519:     	jne	0x140a6654e <.text+0xa6554e>
140a6651b:     	movdqu	(%rbx), %xmm0
140a6651f:     	movzbl	0x10(%rbx), %eax
140a66523:     	movd	%eax, %xmm1
140a66527:     	pcmpeqb	0x1f822d1(%rip), %xmm0  # 0x1429e8800
140a6652f:     	pcmpeqb	0x1ed19a9(%rip), %xmm1  # 0x142937ee0
140a66537:     	pand	%xmm0, %xmm1
140a6653b:     	pmovmskb	%xmm1, %eax
140a6653f:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a66544:     	je	0x140a6660e <.text+0xa6560e>
140a6654a:     	xorl	%ebp, %ebp
140a6654c:     	jmp	0x140a66583 <.text+0xa65583>
140a6654e:     	cmpq	$0x15, %rdi
140a66552:     	jne	0x140a665c9 <.text+0xa655c9>
140a66554:     	movdqu	(%rbx), %xmm0
140a66558:     	movdqu	0x5(%rbx), %xmm1
140a6655d:     	pcmpeqb	0x1f8227b(%rip), %xmm1  # 0x1429e87e0
140a66565:     	pcmpeqb	0x1f82283(%rip), %xmm0  # 0x1429e87f0
140a6656d:     	pand	%xmm1, %xmm0
140a66571:     	pmovmskb	%xmm0, %eax
140a66575:     	movb	$0x1, %bpl
140a66578:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a6657d:     	je	0x140a668b8 <.text+0xa658b8>
140a66583:     	leaq	-0x10(%rdi), %rax
140a66587:     	rorq	%rax
140a6658a:     	xorl	%r14d, %r14d
140a6658d:     	cmpq	$0x5, %rax
140a66591:     	ja	0x140a666a3 <.text+0xa656a3>
140a66597:     	leaq	0x206e38a(%rip), %rcx   # 0x142ad4928
140a6659e:     	movslq	(%rcx,%rax,4), %rax
140a665a2:     	addq	%rcx, %rax
140a665a5:     	jmpq	*%rax
140a665a7:     	leaq	0x206e168(%rip), %rdx   # 0x142ad4716
140a665ae:     	movq	%rbx, %rcx
140a665b1:     	movq	%rdi, %r8
140a665b4:     	callq	0x142923bf0 <.text+0x2922bf0>
140a665b9:     	testl	%eax, %eax
140a665bb:     	je	0x140a668df <.text+0xa658df>
140a665c1:     	xorl	%r14d, %r14d
140a665c4:     	jmp	0x140a666a3 <.text+0xa656a3>
140a665c9:     	cmpq	$0x13, %rdi
140a665cd:     	setne	%r15b
