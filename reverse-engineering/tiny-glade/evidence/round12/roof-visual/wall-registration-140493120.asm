
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140493120:     	pushq	%rbp
140493121:     	pushq	%rsi
140493122:     	pushq	%rdi
140493123:     	subq	$0x1f0, %rsp            # imm = 0x1F0
14049312a:     	leaq	0x80(%rsp), %rbp
140493132:     	movq	$-0x2, 0x168(%rbp)
14049313d:     	movq	%rcx, %rsi
140493140:     	leaq	0x254bf41(%rip), %r8    # 0x1429df088
140493147:     	leaq	0x10(%rbp), %rdi
14049314b:     	movq	%rdi, %rcx
14049314e:     	movb	$0x11, %dl
140493150:     	callq	0x140112d30 <.text+0x111d30>
140493155:     	movq	%rsi, %rcx
140493158:     	movq	%rdi, %rdx
14049315b:     	callq	0x1403276f0 <.text+0x3266f0>
140493160:     	leaq	0x254bf39(%rip), %r8    # 0x1429df0a0
140493167:     	movq	%rdi, %rcx
14049316a:     	movb	$0x2a, %dl
14049316c:     	callq	0x140322d40 <.text+0x321d40>
140493171:     	movq	%rsi, %rcx
140493174:     	movq	%rdi, %rdx
140493177:     	callq	0x1403276f0 <.text+0x3266f0>
14049317c:     	movq	%rdi, %rcx
14049317f:     	callq	0x14054e960 <.text+0x54d960>
140493184:     	leaq	-0x60(%rbp), %rcx
140493188:     	movq	%rdi, %rdx
14049318b:     	callq	0x14050d290 <.text+0x50c290>
140493190:     	leaq	0xf0(%rbp), %rcx
140493197:     	callq	0x140335b30 <.text+0x334b30>
14049319c:     	movups	(%rbp), %xmm0
1404931a0:     	movaps	%xmm0, 0x70(%rbp)
1404931a4:     	movups	-0x10(%rbp), %xmm0
1404931a8:     	movaps	%xmm0, 0x60(%rbp)
1404931ac:     	movups	-0x20(%rbp), %xmm0
1404931b0:     	movaps	%xmm0, 0x50(%rbp)
1404931b4:     	movups	-0x60(%rbp), %xmm0
1404931b8:     	movups	-0x50(%rbp), %xmm1
1404931bc:     	movups	-0x40(%rbp), %xmm2
1404931c0:     	movups	-0x30(%rbp), %xmm3
1404931c4:     	movaps	%xmm3, 0x40(%rbp)
1404931c8:     	movaps	%xmm2, 0x30(%rbp)
1404931cc:     	movaps	%xmm1, 0x20(%rbp)
1404931d0:     	movaps	%xmm0, 0x10(%rbp)
1404931d4:     	movups	0xf0(%rbp), %xmm0
1404931db:     	movups	0x100(%rbp), %xmm1
1404931e2:     	movups	0x110(%rbp), %xmm2
1404931e9:     	movups	0x120(%rbp), %xmm3
1404931f0:     	movaps	%xmm0, 0x80(%rbp)
1404931f7:     	movaps	%xmm1, 0x90(%rbp)
1404931fe:     	movaps	%xmm2, 0xa0(%rbp)
140493205:     	movaps	%xmm3, 0xb0(%rbp)
14049320c:     	movups	0x130(%rbp), %xmm0
140493213:     	movaps	%xmm0, 0xc0(%rbp)
14049321a:     	movups	0x140(%rbp), %xmm0
140493221:     	movaps	%xmm0, 0xd0(%rbp)
140493228:     	movups	0x150(%rbp), %xmm0
14049322f:     	movaps	%xmm0, 0xe0(%rbp)
140493236:     	leaq	0xf0(%rbp), %rcx
14049323d:     	leaq	0x10(%rbp), %rdx
140493241:     	callq	0x1400d5080 <.text+0xd4080>
140493246:     	movb	$0x13, -0x60(%rbp)
14049324a:     	movq	0x319911f(%rip), %rcx   # 0x14362c370
140493251:     	leaq	0x2497d40(%rip), %r8    # 0x14292af98
140493258:     	leaq	-0x60(%rbp), %rdx
14049325c:     	callq	0x140364d20 <.text+0x363d20>
140493261:     	movq	%rdx, %r8
140493264:     	leaq	0xf0(%rbp), %rcx
14049326b:     	movq	%rax, %rdx
14049326e:     	callq	0x1400d68c0 <.text+0xd58c0>
140493273:     	movups	0x150(%rbp), %xmm0
14049327a:     	movaps	%xmm0, 0x70(%rbp)
14049327e:     	movups	0x140(%rbp), %xmm0
140493285:     	movaps	%xmm0, 0x60(%rbp)
140493289:     	movups	0x130(%rbp), %xmm0
140493290:     	movaps	%xmm0, 0x50(%rbp)
140493294:     	movups	0xf0(%rbp), %xmm0
14049329b:     	movups	0x100(%rbp), %xmm1
1404932a2:     	movups	0x110(%rbp), %xmm2
1404932a9:     	movups	0x120(%rbp), %xmm3
1404932b0:     	movaps	%xmm3, 0x40(%rbp)
1404932b4:     	movaps	%xmm2, 0x30(%rbp)
1404932b8:     	movaps	%xmm1, 0x20(%rbp)
1404932bc:     	movaps	%xmm0, 0x10(%rbp)
1404932c0:     	leaq	0x10(%rbp), %rdx
1404932c4:     	movq	%rsi, %rcx
1404932c7:     	callq	0x140326b40 <.text+0x325b40>
1404932cc:     	movq	$0x0, 0x10(%rbp)
1404932d4:     	movq	$0x8, 0x18(%rbp)
1404932dc:     	movq	$0x0, 0x20(%rbp)
1404932e4:     	leaq	0x10(%rbp), %rdx
1404932e8:     	movq	%rsi, %rcx
1404932eb:     	callq	0x14011bfe0 <.text+0x11afe0>
1404932f0:     	movq	%rax, %rsi
1404932f3:     	movq	%rax, %rcx
1404932f6:     	callq	0x1403270d0 <.text+0x3260d0>
1404932fb:     	leaq	0x10(%rbp), %rcx
1404932ff:     	callq	0x14030fb40 <.text+0x30eb40>
140493304:     	movb	$0x18, -0x60(%rbp)
140493308:     	movq	0x3199061(%rip), %rcx   # 0x14362c370
14049330f:     	leaq	0x2497c82(%rip), %r8    # 0x14292af98
140493316:     	leaq	-0x60(%rbp), %rdx
14049331a:     	callq	0x140364d20 <.text+0x363d20>
14049331f:     	movq	%rdx, %r8
140493322:     	leaq	0x10(%rbp), %rcx
140493326:     	movq	%rax, %rdx
140493329:     	callq	0x1400d68c0 <.text+0xd58c0>
14049332e:     	movups	0x70(%rbp), %xmm0
140493332:     	movaps	%xmm0, 0x150(%rbp)
140493339:     	movups	0x60(%rbp), %xmm0
14049333d:     	movaps	%xmm0, 0x140(%rbp)
140493344:     	movups	0x50(%rbp), %xmm0
140493348:     	movaps	%xmm0, 0x130(%rbp)
14049334f:     	movups	0x10(%rbp), %xmm0
140493353:     	movups	0x20(%rbp), %xmm1
140493357:     	movups	0x30(%rbp), %xmm2
14049335b:     	movups	0x40(%rbp), %xmm3
14049335f:     	movaps	%xmm3, 0x120(%rbp)
140493366:     	movaps	%xmm2, 0x110(%rbp)
14049336d:     	movaps	%xmm1, 0x100(%rbp)
140493374:     	movaps	%xmm0, 0xf0(%rbp)
14049337b:     	leaq	0xf0(%rbp), %rdx
140493382:     	movq	%rsi, %rcx
140493385:     	callq	0x1403276f0 <.text+0x3266f0>
14049338a:     	leaq	0x10(%rbp), %rcx
14049338e:     	callq	0x14033def0 <.text+0x33cef0>
140493393:     	movb	$0x1c, -0x60(%rbp)
140493397:     	movq	0x3198fd2(%rip), %rcx   # 0x14362c370
14049339e:     	leaq	0x2497bf3(%rip), %r8    # 0x14292af98
1404933a5:     	leaq	-0x60(%rbp), %rdx
1404933a9:     	callq	0x140364d20 <.text+0x363d20>
1404933ae:     	movq	%rdx, %r8
1404933b1:     	leaq	0x10(%rbp), %rcx
1404933b5:     	movq	%rax, %rdx
1404933b8:     	callq	0x1400d68c0 <.text+0xd58c0>
1404933bd:     	movups	0x70(%rbp), %xmm0
1404933c1:     	movaps	%xmm0, 0x150(%rbp)
1404933c8:     	movups	0x60(%rbp), %xmm0
1404933cc:     	movaps	%xmm0, 0x140(%rbp)
1404933d3:     	movups	0x50(%rbp), %xmm0
1404933d7:     	movaps	%xmm0, 0x130(%rbp)
1404933de:     	movups	0x10(%rbp), %xmm0
1404933e2:     	movups	0x20(%rbp), %xmm1
1404933e6:     	movups	0x30(%rbp), %xmm2
1404933ea:     	movups	0x40(%rbp), %xmm3
1404933ee:     	movaps	%xmm3, 0x120(%rbp)
1404933f5:     	movaps	%xmm2, 0x110(%rbp)
1404933fc:     	movaps	%xmm1, 0x100(%rbp)
140493403:     	movaps	%xmm0, 0xf0(%rbp)
14049340a:     	leaq	0xf0(%rbp), %rdx
140493411:     	movq	%rsi, %rcx
140493414:     	callq	0x1403276f0 <.text+0x3266f0>
140493419:     	leaq	0x254bc98(%rip), %r8    # 0x1429df0b8
140493420:     	movq	%rdi, %rcx
140493423:     	movb	$0x24, %dl
140493425:     	callq	0x1403104a0 <.text+0x30f4a0>
14049342a:     	movq	%rsi, %rcx
14049342d:     	movq	%rdi, %rdx
140493430:     	callq	0x1403276f0 <.text+0x3266f0>
140493435:     	leaq	0x10(%rbp), %rcx
140493439:     	callq	0x1400cbd40 <.text+0xcad40>
14049343e:     	movb	$0x22, -0x60(%rbp)
140493442:     	movq	0x3198f27(%rip), %rcx   # 0x14362c370
140493449:     	leaq	0x2497b48(%rip), %r8    # 0x14292af98
140493450:     	leaq	-0x60(%rbp), %rdx
140493454:     	callq	0x140364d20 <.text+0x363d20>
140493459:     	movq	%rdx, %r8
14049345c:     	leaq	0x10(%rbp), %rcx
140493460:     	movq	%rax, %rdx
140493463:     	callq	0x1400d68c0 <.text+0xd58c0>
140493468:     	movups	0x70(%rbp), %xmm0
14049346c:     	movaps	%xmm0, 0x150(%rbp)
140493473:     	movups	0x60(%rbp), %xmm0
140493477:     	movaps	%xmm0, 0x140(%rbp)
14049347e:     	movups	0x50(%rbp), %xmm0
140493482:     	movaps	%xmm0, 0x130(%rbp)
140493489:     	movups	0x10(%rbp), %xmm0
14049348d:     	movups	0x20(%rbp), %xmm1
140493491:     	movups	0x30(%rbp), %xmm2
140493495:     	movups	0x40(%rbp), %xmm3
140493499:     	movaps	%xmm3, 0x120(%rbp)
1404934a0:     	movaps	%xmm2, 0x110(%rbp)
1404934a7:     	movaps	%xmm1, 0x100(%rbp)
1404934ae:     	movaps	%xmm0, 0xf0(%rbp)
1404934b5:     	leaq	0xf0(%rbp), %rdx
1404934bc:     	movq	%rsi, %rcx
1404934bf:     	callq	0x1403276f0 <.text+0x3266f0>
1404934c4:     	leaq	0x254bc05(%rip), %r8    # 0x1429df0d0
1404934cb:     	movq	%rdi, %rcx
1404934ce:     	movb	$0x2c, %dl
1404934d0:     	callq	0x1400f50e0 <.text+0xf40e0>
1404934d5:     	movq	%rsi, %rcx
1404934d8:     	movq	%rdi, %rdx
1404934db:     	callq	0x1403276f0 <.text+0x3266f0>
1404934e0:     	leaq	0x10(%rbp), %rcx
1404934e4:     	callq	0x140272c20 <.text+0x271c20>
1404934e9:     	movb	$0x20, -0x60(%rbp)
1404934ed:     	movq	0x3198e7c(%rip), %rcx   # 0x14362c370
1404934f4:     	leaq	0x2497a9d(%rip), %r8    # 0x14292af98
1404934fb:     	leaq	-0x60(%rbp), %rdx
1404934ff:     	callq	0x140364d20 <.text+0x363d20>
140493504:     	movq	%rdx, %r8
140493507:     	leaq	0x10(%rbp), %rcx
14049350b:     	movq	%rax, %rdx
14049350e:     	callq	0x1400d68c0 <.text+0xd58c0>
140493513:     	movups	0x70(%rbp), %xmm0
140493517:     	movaps	%xmm0, 0x150(%rbp)
14049351e:     	movups	0x60(%rbp), %xmm0
140493522:     	movaps	%xmm0, 0x140(%rbp)
140493529:     	movups	0x50(%rbp), %xmm0
14049352d:     	movaps	%xmm0, 0x130(%rbp)
140493534:     	movups	0x10(%rbp), %xmm0
140493538:     	movups	0x20(%rbp), %xmm1
14049353c:     	movups	0x30(%rbp), %xmm2
140493540:     	movups	0x40(%rbp), %xmm3
140493544:     	movaps	%xmm3, 0x120(%rbp)
14049354b:     	movaps	%xmm2, 0x110(%rbp)
140493552:     	movaps	%xmm1, 0x100(%rbp)
140493559:     	movaps	%xmm0, 0xf0(%rbp)
140493560:     	leaq	0xf0(%rbp), %rdx
140493567:     	movq	%rsi, %rcx
14049356a:     	callq	0x1403276f0 <.text+0x3266f0>
14049356f:     	movq	%rsi, %rcx
140493572:     	movb	$0x1, %dl
140493574:     	callq	0x140327c20 <.text+0x326c20>
140493579:     	movq	%rsi, %rcx
14049357c:     	addq	$0x1f0, %rsp            # imm = 0x1F0
140493583:     	popq	%rdi
140493584:     	popq	%rsi
140493585:     	popq	%rbp
140493586:     	jmp	0x140327ae0 <.text+0x326ae0>
14049358b:     	nopl	(%rax,%rax)
140493590:     	movq	%rdx, 0x10(%rsp)
140493595:     	pushq	%rbp
140493596:     	pushq	%rsi
140493597:     	pushq	%rdi
140493598:     	subq	$0x20, %rsp
14049359c:     	leaq	0x80(%rdx), %rbp
1404935a3:     	leaq	-0x60(%rbp), %rcx
1404935a7:     	callq	0x140029880 <.text+0x28880>
1404935ac:     	nop
1404935ad:     	addq	$0x20, %rsp
1404935b1:     	popq	%rdi
1404935b2:     	popq	%rsi
1404935b3:     	popq	%rbp
1404935b4:     	retq
1404935b5:     	nopw	%cs:(%rax,%rax)
1404935c0:     	movq	%rdx, 0x10(%rsp)
1404935c5:     	pushq	%rbp
1404935c6:     	pushq	%rsi
1404935c7:     	pushq	%rdi
1404935c8:     	subq	$0x20, %rsp
1404935cc:     	leaq	0x80(%rdx), %rbp
1404935d3:     	leaq	0xf0(%rbp), %rcx
1404935da:     	callq	0x140029880 <.text+0x28880>
1404935df:     	nop
1404935e0:     	addq	$0x20, %rsp
1404935e4:     	popq	%rdi
1404935e5:     	popq	%rsi
1404935e6:     	popq	%rbp
1404935e7:     	retq
1404935e8:     	nopl	(%rax,%rax)
1404935f0:     	movq	%rdx, 0x10(%rsp)
1404935f5:     	pushq	%rbp
1404935f6:     	pushq	%rsi
1404935f7:     	pushq	%rdi
1404935f8:     	subq	$0x20, %rsp
1404935fc:     	leaq	0x80(%rdx), %rbp
140493603:     	leaq	0x10(%rbp), %rcx
140493607:     	callq	0x140029880 <.text+0x28880>
14049360c:     	nop
14049360d:     	addq	$0x20, %rsp
140493611:     	popq	%rdi
140493612:     	popq	%rsi
140493613:     	popq	%rbp
140493614:     	retq
140493615:     	nopw	%cs:(%rax,%rax)
140493620:     	movq	%rdx, 0x10(%rsp)
140493625:     	pushq	%rbp
140493626:     	pushq	%rsi
140493627:     	pushq	%rdi
140493628:     	subq	$0x20, %rsp
14049362c:     	leaq	0x80(%rdx), %rbp
140493633:     	leaq	0x10(%rbp), %rcx
140493637:     	callq	0x140029880 <.text+0x28880>
14049363c:     	nop
14049363d:     	addq	$0x20, %rsp
140493641:     	popq	%rdi
140493642:     	popq	%rsi
140493643:     	popq	%rbp
140493644:     	retq
140493645:     	nopw	%cs:(%rax,%rax)
140493650:     	movq	%rdx, 0x10(%rsp)
140493655:     	pushq	%rbp
140493656:     	pushq	%rsi
140493657:     	pushq	%rdi
140493658:     	subq	$0x20, %rsp
14049365c:     	leaq	0x80(%rdx), %rbp
140493663:     	leaq	0x10(%rbp), %rcx
140493667:     	callq	0x140029880 <.text+0x28880>
14049366c:     	nop
14049366d:     	addq	$0x20, %rsp
140493671:     	popq	%rdi
140493672:     	popq	%rsi
140493673:     	popq	%rbp
140493674:     	retq
140493675:     	nopw	%cs:(%rax,%rax)
140493680:     	movq	%rdx, 0x10(%rsp)
140493685:     	pushq	%rbp
140493686:     	pushq	%rsi
140493687:     	pushq	%rdi
140493688:     	subq	$0x20, %rsp
14049368c:     	leaq	0x80(%rdx), %rbp
140493693:     	leaq	0x10(%rbp), %rcx
140493697:     	callq	0x140029880 <.text+0x28880>
14049369c:     	nop
14049369d:     	addq	$0x20, %rsp
1404936a1:     	popq	%rdi
1404936a2:     	popq	%rsi
1404936a3:     	popq	%rbp
1404936a4:     	retq
