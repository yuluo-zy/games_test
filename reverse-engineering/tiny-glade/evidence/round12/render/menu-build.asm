
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068de50:     	pushq	%rbp
14068de51:     	pushq	%rsi
14068de52:     	pushq	%rdi
14068de53:     	subq	$0x810, %rsp            # imm = 0x810
14068de5a:     	leaq	0x80(%rsp), %rbp
14068de62:     	movq	$-0x2, 0x788(%rbp)
14068de6d:     	movq	%rdx, %rcx
14068de70:     	movq	$0x2, -0x60(%rbp)
14068de78:     	leaq	-0x60(%rbp), %rdx
14068de7c:     	callq	0x140fd1390 <.text+0xfd0390>
14068de81:     	movq	$0xb, -0x60(%rbp)
14068de89:     	leaq	-0x60(%rbp), %rdx
14068de8d:     	movq	%rax, %rcx
14068de90:     	callq	0x140fd1fa0 <.text+0xfd0fa0>
14068de95:     	movq	$0xb, -0x60(%rbp)
14068de9d:     	leaq	-0x60(%rbp), %rdx
14068dea1:     	movq	%rax, %rcx
14068dea4:     	callq	0x140fcfcb0 <.text+0xfcecb0>
14068dea9:     	movq	%rax, %rcx
14068deac:     	xorl	%edx, %edx
14068deae:     	callq	0x140fd0890 <.text+0xfcf890>
14068deb3:     	movq	%rax, %rcx
14068deb6:     	movl	$0x1, %edx
14068debb:     	callq	0x140fd0e10 <.text+0xfcfe10>
14068dec0:     	movq	%rax, %rsi
14068dec3:     	leaq	0x630(%rbp), %rcx
14068deca:     	movb	$0x1, %dl
14068decc:     	callq	0x140ff2b10 <.text+0xff1b10>
14068ded1:     	leaq	0x5c0(%rbp), %rcx
14068ded8:     	movb	$0x1, %dl
14068deda:     	callq	0x140ff3700 <.text+0xff2700>
14068dedf:     	leaq	0x550(%rbp), %rcx
14068dee6:     	movb	$0x1, %dl
14068dee8:     	callq	0x140ff4d40 <.text+0xff3d40>
14068deed:     	leaq	0x4e0(%rbp), %rcx
14068def4:     	movb	$0x1, %dl
14068def6:     	callq	0x140ff69a0 <.text+0xff59a0>
14068defb:     	leaq	0x470(%rbp), %rcx
14068df02:     	movb	$0x1, %dl
14068df04:     	callq	0x140ff7a30 <.text+0xff6a30>
14068df09:     	leaq	0x400(%rbp), %rcx
14068df10:     	movb	$0x1, %dl
14068df12:     	callq	0x140ff8820 <.text+0xff7820>
14068df17:     	leaq	0x390(%rbp), %rcx
14068df1e:     	movb	$0x1, %dl
14068df20:     	callq	0x140ff9040 <.text+0xff8040>
14068df25:     	leaq	0x6a0(%rbp), %rcx
14068df2c:     	movb	$0x1, %dl
14068df2e:     	callq	0x140ff9df0 <.text+0xff8df0>
14068df33:     	leaq	0x710(%rbp), %rcx
14068df3a:     	movb	$0x1, %dl
14068df3c:     	callq	0x140ffa810 <.text+0xff9810>
14068df41:     	movups	0x690(%rbp), %xmm0
14068df48:     	movaps	%xmm0, (%rbp)
14068df4c:     	movups	0x680(%rbp), %xmm0
14068df53:     	movaps	%xmm0, -0x10(%rbp)
14068df57:     	movups	0x670(%rbp), %xmm0
14068df5e:     	movaps	%xmm0, -0x20(%rbp)
14068df62:     	movups	0x630(%rbp), %xmm0
14068df69:     	movups	0x640(%rbp), %xmm1
14068df70:     	movups	0x650(%rbp), %xmm2
14068df77:     	movups	0x660(%rbp), %xmm3
14068df7e:     	movaps	%xmm3, -0x30(%rbp)
14068df82:     	movaps	%xmm2, -0x40(%rbp)
14068df86:     	movaps	%xmm1, -0x50(%rbp)
14068df8a:     	movaps	%xmm0, -0x60(%rbp)
14068df8e:     	movups	0x5c0(%rbp), %xmm0
14068df95:     	movups	0x5d0(%rbp), %xmm1
14068df9c:     	movups	0x5e0(%rbp), %xmm2
14068dfa3:     	movups	0x5f0(%rbp), %xmm3
14068dfaa:     	movaps	%xmm0, 0x10(%rbp)
14068dfae:     	movaps	%xmm1, 0x20(%rbp)
14068dfb2:     	movaps	%xmm2, 0x30(%rbp)
14068dfb6:     	movaps	%xmm3, 0x40(%rbp)
14068dfba:     	movups	0x600(%rbp), %xmm0
14068dfc1:     	movaps	%xmm0, 0x50(%rbp)
14068dfc5:     	movups	0x610(%rbp), %xmm0
14068dfcc:     	movaps	%xmm0, 0x60(%rbp)
14068dfd0:     	movups	0x620(%rbp), %xmm0
14068dfd7:     	movaps	%xmm0, 0x70(%rbp)
14068dfdb:     	movups	0x5b0(%rbp), %xmm0
14068dfe2:     	movaps	%xmm0, 0xe0(%rbp)
14068dfe9:     	movups	0x5a0(%rbp), %xmm0
14068dff0:     	movaps	%xmm0, 0xd0(%rbp)
14068dff7:     	movups	0x590(%rbp), %xmm0
14068dffe:     	movaps	%xmm0, 0xc0(%rbp)
14068e005:     	movups	0x550(%rbp), %xmm0
14068e00c:     	movups	0x560(%rbp), %xmm1
14068e013:     	movups	0x570(%rbp), %xmm2
14068e01a:     	movups	0x580(%rbp), %xmm3
14068e021:     	movaps	%xmm3, 0xb0(%rbp)
14068e028:     	movaps	%xmm2, 0xa0(%rbp)
14068e02f:     	movaps	%xmm1, 0x90(%rbp)
14068e036:     	movaps	%xmm0, 0x80(%rbp)
14068e03d:     	movups	0x540(%rbp), %xmm0
14068e044:     	movaps	%xmm0, 0x150(%rbp)
14068e04b:     	movups	0x530(%rbp), %xmm0
14068e052:     	movaps	%xmm0, 0x140(%rbp)
14068e059:     	movups	0x520(%rbp), %xmm0
14068e060:     	movaps	%xmm0, 0x130(%rbp)
14068e067:     	movups	0x4e0(%rbp), %xmm0
14068e06e:     	movups	0x4f0(%rbp), %xmm1
14068e075:     	movups	0x500(%rbp), %xmm2
14068e07c:     	movups	0x510(%rbp), %xmm3
14068e083:     	movaps	%xmm3, 0x120(%rbp)
14068e08a:     	movaps	%xmm2, 0x110(%rbp)
14068e091:     	movaps	%xmm1, 0x100(%rbp)
14068e098:     	movaps	%xmm0, 0xf0(%rbp)
14068e09f:     	movups	0x4d0(%rbp), %xmm0
14068e0a6:     	movaps	%xmm0, 0x1c0(%rbp)
14068e0ad:     	movups	0x4c0(%rbp), %xmm0
14068e0b4:     	movaps	%xmm0, 0x1b0(%rbp)
14068e0bb:     	movups	0x4b0(%rbp), %xmm0
14068e0c2:     	movaps	%xmm0, 0x1a0(%rbp)
14068e0c9:     	movups	0x470(%rbp), %xmm0
14068e0d0:     	movups	0x480(%rbp), %xmm1
14068e0d7:     	movups	0x490(%rbp), %xmm2
14068e0de:     	movups	0x4a0(%rbp), %xmm3
14068e0e5:     	movaps	%xmm3, 0x190(%rbp)
14068e0ec:     	movaps	%xmm2, 0x180(%rbp)
14068e0f3:     	movaps	%xmm1, 0x170(%rbp)
14068e0fa:     	movaps	%xmm0, 0x160(%rbp)
14068e101:     	movups	0x460(%rbp), %xmm0
14068e108:     	movaps	%xmm0, 0x230(%rbp)
14068e10f:     	movups	0x450(%rbp), %xmm0
14068e116:     	movaps	%xmm0, 0x220(%rbp)
14068e11d:     	movups	0x440(%rbp), %xmm0
14068e124:     	movaps	%xmm0, 0x210(%rbp)
14068e12b:     	movups	0x400(%rbp), %xmm0
14068e132:     	movups	0x410(%rbp), %xmm1
14068e139:     	movups	0x420(%rbp), %xmm2
14068e140:     	movups	0x430(%rbp), %xmm3
14068e147:     	movaps	%xmm3, 0x200(%rbp)
14068e14e:     	movaps	%xmm2, 0x1f0(%rbp)
14068e155:     	movaps	%xmm1, 0x1e0(%rbp)
14068e15c:     	movaps	%xmm0, 0x1d0(%rbp)
14068e163:     	movups	0x3f0(%rbp), %xmm0
14068e16a:     	movaps	%xmm0, 0x2a0(%rbp)
14068e171:     	movups	0x3e0(%rbp), %xmm0
14068e178:     	movaps	%xmm0, 0x290(%rbp)
14068e17f:     	movups	0x3d0(%rbp), %xmm0
14068e186:     	movaps	%xmm0, 0x280(%rbp)
14068e18d:     	movups	0x390(%rbp), %xmm0
14068e194:     	movups	0x3a0(%rbp), %xmm1
14068e19b:     	movups	0x3b0(%rbp), %xmm2
14068e1a2:     	movups	0x3c0(%rbp), %xmm3
14068e1a9:     	movaps	%xmm3, 0x270(%rbp)
14068e1b0:     	movaps	%xmm2, 0x260(%rbp)
14068e1b7:     	movaps	%xmm1, 0x250(%rbp)
14068e1be:     	movaps	%xmm0, 0x240(%rbp)
14068e1c5:     	movups	0x700(%rbp), %xmm0
14068e1cc:     	movaps	%xmm0, 0x310(%rbp)
14068e1d3:     	movups	0x6f0(%rbp), %xmm0
14068e1da:     	movaps	%xmm0, 0x300(%rbp)
14068e1e1:     	movups	0x6e0(%rbp), %xmm0
14068e1e8:     	movaps	%xmm0, 0x2f0(%rbp)
14068e1ef:     	movups	0x6a0(%rbp), %xmm0
14068e1f6:     	movups	0x6b0(%rbp), %xmm1
14068e1fd:     	movups	0x6c0(%rbp), %xmm2
14068e204:     	movups	0x6d0(%rbp), %xmm3
14068e20b:     	movaps	%xmm3, 0x2e0(%rbp)
14068e212:     	movaps	%xmm2, 0x2d0(%rbp)
14068e219:     	movaps	%xmm1, 0x2c0(%rbp)
14068e220:     	movaps	%xmm0, 0x2b0(%rbp)
14068e227:     	movups	0x770(%rbp), %xmm0
14068e22e:     	movaps	%xmm0, 0x380(%rbp)
14068e235:     	movups	0x760(%rbp), %xmm0
14068e23c:     	movaps	%xmm0, 0x370(%rbp)
14068e243:     	movups	0x750(%rbp), %xmm0
14068e24a:     	movaps	%xmm0, 0x360(%rbp)
14068e251:     	movups	0x710(%rbp), %xmm0
14068e258:     	movups	0x720(%rbp), %xmm1
14068e25f:     	movups	0x730(%rbp), %xmm2
14068e266:     	movups	0x740(%rbp), %xmm3
14068e26d:     	movaps	%xmm3, 0x350(%rbp)
14068e274:     	movaps	%xmm2, 0x340(%rbp)
14068e27b:     	movaps	%xmm1, 0x330(%rbp)
14068e282:     	movaps	%xmm0, 0x320(%rbp)
14068e289:     	leaq	0x238a0f0(%rip), %r9    # 0x142a18380
14068e290:     	leaq	0x710(%rbp), %rdi
14068e297:     	leaq	-0x60(%rbp), %rdx
14068e29b:     	movq	%rdi, %rcx
14068e29e:     	movb	$0x24, %r8b
14068e2a1:     	callq	0x140fdd640 <.text+0xfdc640>
14068e2a6:     	movq	%rsi, %rcx
14068e2a9:     	movq	%rdi, %rdx
14068e2ac:     	callq	0x140fd2dd0 <.text+0xfd1dd0>
14068e2b1:     	leaq	-0x60(%rbp), %rcx
14068e2b5:     	callq	0x140ffb4f0 <.text+0xffa4f0>
14068e2ba:     	movb	$0xa, 0x6a0(%rbp)
14068e2c1:     	movq	0x2f9e0a8(%rip), %rcx   # 0x14362c370
14068e2c8:     	leaq	0x25338a9(%rip), %r8    # 0x142bc1b78
14068e2cf:     	leaq	0x6a0(%rbp), %rdx
14068e2d6:     	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068e2db:     	movq	%rdx, %r8
14068e2de:     	leaq	-0x60(%rbp), %rcx
14068e2e2:     	movq	%rax, %rdx
14068e2e5:     	callq	0x140fddf80 <.text+0xfdcf80>
14068e2ea:     	movups	(%rbp), %xmm0
14068e2ee:     	movaps	%xmm0, 0x770(%rbp)
14068e2f5:     	movups	-0x10(%rbp), %xmm0
14068e2f9:     	movaps	%xmm0, 0x760(%rbp)
14068e300:     	movups	-0x20(%rbp), %xmm0
14068e304:     	movaps	%xmm0, 0x750(%rbp)
14068e30b:     	movups	-0x60(%rbp), %xmm0
14068e30f:     	movups	-0x50(%rbp), %xmm1
14068e313:     	movups	-0x40(%rbp), %xmm2
14068e317:     	movups	-0x30(%rbp), %xmm3
14068e31b:     	movaps	%xmm3, 0x740(%rbp)
14068e322:     	movaps	%xmm2, 0x730(%rbp)
14068e329:     	movaps	%xmm1, 0x720(%rbp)
14068e330:     	movaps	%xmm0, 0x710(%rbp)
14068e337:     	leaq	0x710(%rbp), %rdx
14068e33e:     	movq	%rsi, %rcx
14068e341:     	callq	0x140fd2dd0 <.text+0xfd1dd0>
14068e346:     	movq	%rsi, %rcx
14068e349:     	addq	$0x38, %rcx
14068e34d:     	movabsq	$0x633d43f1bfc13234, %rdx # imm = 0x633D43F1BFC13234
14068e357:     	movabsq	$0x643ff3691e81d561, %r8 # imm = 0x643FF3691E81D561
14068e361:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068e366:     	testq	%rax, %rax
14068e369:     	je	0x14068e3a9 <.text+0x68d3a9>
14068e36b:     	cmpq	%rdx, 0x218(%rsi)
14068e372:     	jbe	0x14068e3a9 <.text+0x68d3a9>
14068e374:     	movq	0x210(%rsi), %rax
14068e37b:     	movq	(%rax,%rdx,8), %rax
14068e37f:     	testq	%rax, %rax
14068e382:     	je	0x14068e3a9 <.text+0x68d3a9>
14068e384:     	movq	0x1e0(%rsi), %rcx
14068e38b:     	notq	%rax
14068e38e:     	leaq	(%rax,%rax,2), %rax
14068e392:     	shlq	$0x5, %rax
14068e396:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068e39c:     	je	0x14068e3a9 <.text+0x68d3a9>
14068e39e:     	addq	$0x810, %rsp            # imm = 0x810
14068e3a5:     	popq	%rdi
14068e3a6:     	popq	%rsi
14068e3a7:     	popq	%rbp
14068e3a8:     	retq
14068e3a9:     	movq	%rsi, %rcx
14068e3ac:     	addq	$0x810, %rsp            # imm = 0x810
14068e3b3:     	popq	%rdi
14068e3b4:     	popq	%rsi
14068e3b5:     	popq	%rbp
14068e3b6:     	jmp	0x140fe9d40 <.text+0xfe8d40>
14068e3bb:     	nopl	(%rax,%rax)
14068e3c0:     	movq	%rdx, 0x10(%rsp)
14068e3c5:     	pushq	%rbp
14068e3c6:     	pushq	%rsi
14068e3c7:     	pushq	%rdi
14068e3c8:     	subq	$0x20, %rsp
14068e3cc:     	leaq	0x80(%rdx), %rbp
14068e3d3:     	leaq	0x630(%rbp), %rcx
14068e3da:     	callq	0x140029880 <.text+0x28880>
14068e3df:     	nop
14068e3e0:     	addq	$0x20, %rsp
14068e3e4:     	popq	%rdi
14068e3e5:     	popq	%rsi
14068e3e6:     	popq	%rbp
14068e3e7:     	retq
14068e3e8:     	nopl	(%rax,%rax)
14068e3f0:     	movq	%rdx, 0x10(%rsp)
14068e3f5:     	pushq	%rbp
14068e3f6:     	pushq	%rsi
14068e3f7:     	pushq	%rdi
14068e3f8:     	subq	$0x20, %rsp
14068e3fc:     	leaq	0x80(%rdx), %rbp
14068e403:     	leaq	0x5c0(%rbp), %rcx
14068e40a:     	callq	0x140029880 <.text+0x28880>
14068e40f:     	nop
14068e410:     	addq	$0x20, %rsp
14068e414:     	popq	%rdi
14068e415:     	popq	%rsi
14068e416:     	popq	%rbp
14068e417:     	retq
14068e418:     	nopl	(%rax,%rax)
14068e420:     	movq	%rdx, 0x10(%rsp)
14068e425:     	pushq	%rbp
14068e426:     	pushq	%rsi
14068e427:     	pushq	%rdi
14068e428:     	subq	$0x20, %rsp
14068e42c:     	leaq	0x80(%rdx), %rbp
14068e433:     	leaq	0x550(%rbp), %rcx
14068e43a:     	callq	0x140029880 <.text+0x28880>
14068e43f:     	nop
14068e440:     	addq	$0x20, %rsp
14068e444:     	popq	%rdi
14068e445:     	popq	%rsi
14068e446:     	popq	%rbp
14068e447:     	retq
14068e448:     	nopl	(%rax,%rax)
14068e450:     	movq	%rdx, 0x10(%rsp)
14068e455:     	pushq	%rbp
14068e456:     	pushq	%rsi
14068e457:     	pushq	%rdi
14068e458:     	subq	$0x20, %rsp
14068e45c:     	leaq	0x80(%rdx), %rbp
14068e463:     	leaq	0x4e0(%rbp), %rcx
14068e46a:     	callq	0x140029880 <.text+0x28880>
14068e46f:     	nop
14068e470:     	addq	$0x20, %rsp
14068e474:     	popq	%rdi
14068e475:     	popq	%rsi
14068e476:     	popq	%rbp
14068e477:     	retq
14068e478:     	nopl	(%rax,%rax)
14068e480:     	movq	%rdx, 0x10(%rsp)
14068e485:     	pushq	%rbp
14068e486:     	pushq	%rsi
14068e487:     	pushq	%rdi
14068e488:     	subq	$0x20, %rsp
14068e48c:     	leaq	0x80(%rdx), %rbp
14068e493:     	leaq	0x470(%rbp), %rcx
14068e49a:     	callq	0x140029880 <.text+0x28880>
14068e49f:     	nop
14068e4a0:     	addq	$0x20, %rsp
14068e4a4:     	popq	%rdi
14068e4a5:     	popq	%rsi
14068e4a6:     	popq	%rbp
14068e4a7:     	retq
14068e4a8:     	nopl	(%rax,%rax)
14068e4b0:     	movq	%rdx, 0x10(%rsp)
14068e4b5:     	pushq	%rbp
14068e4b6:     	pushq	%rsi
14068e4b7:     	pushq	%rdi
14068e4b8:     	subq	$0x20, %rsp
14068e4bc:     	leaq	0x80(%rdx), %rbp
14068e4c3:     	leaq	0x400(%rbp), %rcx
14068e4ca:     	callq	0x140029880 <.text+0x28880>
14068e4cf:     	nop
14068e4d0:     	addq	$0x20, %rsp
14068e4d4:     	popq	%rdi
14068e4d5:     	popq	%rsi
14068e4d6:     	popq	%rbp
14068e4d7:     	retq
14068e4d8:     	nopl	(%rax,%rax)
14068e4e0:     	movq	%rdx, 0x10(%rsp)
14068e4e5:     	pushq	%rbp
14068e4e6:     	pushq	%rsi
14068e4e7:     	pushq	%rdi
14068e4e8:     	subq	$0x20, %rsp
14068e4ec:     	leaq	0x80(%rdx), %rbp
14068e4f3:     	leaq	0x390(%rbp), %rcx
14068e4fa:     	callq	0x140029880 <.text+0x28880>
14068e4ff:     	nop
14068e500:     	addq	$0x20, %rsp
14068e504:     	popq	%rdi
14068e505:     	popq	%rsi
14068e506:     	popq	%rbp
14068e507:     	retq
14068e508:     	nopl	(%rax,%rax)
14068e510:     	movq	%rdx, 0x10(%rsp)
14068e515:     	pushq	%rbp
14068e516:     	pushq	%rsi
14068e517:     	pushq	%rdi
14068e518:     	subq	$0x20, %rsp
14068e51c:     	leaq	0x80(%rdx), %rbp
14068e523:     	leaq	0x6a0(%rbp), %rcx
14068e52a:     	callq	0x140029880 <.text+0x28880>
14068e52f:     	nop
14068e530:     	addq	$0x20, %rsp
14068e534:     	popq	%rdi
14068e535:     	popq	%rsi
14068e536:     	popq	%rbp
14068e537:     	retq
14068e538:     	nopl	(%rax,%rax)
14068e540:     	movq	%rdx, 0x10(%rsp)
14068e545:     	pushq	%rbp
14068e546:     	pushq	%rsi
14068e547:     	pushq	%rdi
14068e548:     	subq	$0x20, %rsp
14068e54c:     	leaq	0x80(%rdx), %rbp
14068e553:     	leaq	-0x60(%rbp), %rcx
14068e557:     	callq	0x140029880 <.text+0x28880>
14068e55c:     	nop
14068e55d:     	addq	$0x20, %rsp
14068e561:     	popq	%rdi
14068e562:     	popq	%rsi
14068e563:     	popq	%rbp
14068e564:     	retq
