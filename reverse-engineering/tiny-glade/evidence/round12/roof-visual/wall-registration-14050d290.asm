
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14050d290:     	pushq	%rbp
14050d291:     	pushq	%rsi
14050d292:     	pushq	%rdi
14050d293:     	pushq	%rbx
14050d294:     	subq	$0x9f8, %rsp            # imm = 0x9F8
14050d29b:     	leaq	0x80(%rsp), %rbp
14050d2a3:     	movq	$-0x2, 0x970(%rbp)
14050d2ae:     	movq	%rcx, %rsi
14050d2b1:     	movups	0x60(%rdx), %xmm0
14050d2b5:     	movaps	%xmm0, 0x950(%rbp)
14050d2bc:     	movups	0x50(%rdx), %xmm0
14050d2c0:     	movaps	%xmm0, 0x940(%rbp)
14050d2c7:     	movups	0x40(%rdx), %xmm0
14050d2cb:     	movaps	%xmm0, 0x930(%rbp)
14050d2d2:     	movups	(%rdx), %xmm0
14050d2d5:     	movups	0x10(%rdx), %xmm1
14050d2d9:     	movups	0x20(%rdx), %xmm2
14050d2dd:     	movups	0x30(%rdx), %xmm3
14050d2e1:     	movaps	%xmm3, 0x920(%rbp)
14050d2e8:     	movaps	%xmm2, 0x910(%rbp)
14050d2ef:     	movaps	%xmm1, 0x900(%rbp)
14050d2f6:     	movaps	%xmm0, 0x8f0(%rbp)
14050d2fd:     	movzbl	0x3127905(%rip), %eax   # 0x143634c09
14050d304:     	movl	$0x7e0, %ecx            # imm = 0x7E0
14050d309:     	movl	$0x8, %edx
14050d30e:     	callq	0x140613c10 <.text+0x612c10>
14050d313:     	testq	%rax, %rax
14050d316:     	je	0x14050dd40 <.text+0x50cd40>
14050d31c:     	movq	%rax, 0x960(%rbp)
14050d323:     	xorps	%xmm0, %xmm0
14050d326:     	movups	%xmm0, 0x760(%rbp)
14050d32d:     	movups	%xmm0, 0x778(%rbp)
14050d334:     	movups	%xmm0, 0x790(%rbp)
14050d33b:     	movups	%xmm0, 0x7a8(%rbp)
14050d342:     	movups	%xmm0, 0x7c0(%rbp)
14050d349:     	movl	$0x0, 0x7d0(%rbp)
14050d353:     	movups	%xmm0, 0x7f8(%rbp)
14050d35a:     	movups	%xmm0, 0x810(%rbp)
14050d361:     	movups	%xmm0, 0x828(%rbp)
14050d368:     	movups	%xmm0, 0x840(%rbp)
14050d36f:     	movups	%xmm0, 0x858(%rbp)
14050d376:     	movl	$0x0, 0x868(%rbp)
14050d380:     	movq	$0x0, 0x6a0(%rbp)
14050d38b:     	movq	$0x0, 0x740(%rbp)
14050d396:     	movq	$0x8, 0x748(%rbp)
14050d3a1:     	movq	$0x0, 0x750(%rbp)
14050d3ac:     	movq	$0x10, 0x758(%rbp)
14050d3b7:     	movq	$0x10, 0x770(%rbp)
14050d3c2:     	movq	$0x10, 0x788(%rbp)
14050d3cd:     	movq	$0x10, 0x7a0(%rbp)
14050d3d8:     	movq	$0x10, 0x7b8(%rbp)
14050d3e3:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
14050d3ed:     	movq	%rbx, 0x7d8(%rbp)
14050d3f4:     	leaq	0x24e27d2(%rip), %rax   # 0x1429efbcd
14050d3fb:     	movq	%rax, 0x7e0(%rbp)
14050d402:     	movq	$0x39, 0x7e8(%rbp)
14050d40d:     	movq	$0x10, 0x7f0(%rbp)
14050d418:     	movq	$0x10, 0x808(%rbp)
14050d423:     	movq	$0x10, 0x820(%rbp)
14050d42e:     	movq	$0x10, 0x838(%rbp)
14050d439:     	movq	$0x10, 0x850(%rbp)
14050d444:     	movl	$0x0, 0x870(%rbp)
14050d44e:     	movw	$0x1, 0x874(%rbp)
14050d457:     	movl	$0x0, 0x878(%rbp)
14050d461:     	movzbl	0x31277a1(%rip), %eax   # 0x143634c09
14050d468:     	movl	$0x1e0, %ecx            # imm = 0x1E0
14050d46d:     	movl	$0x8, %edx
14050d472:     	callq	0x140613c10 <.text+0x612c10>
14050d477:     	testq	%rax, %rax
14050d47a:     	je	0x14050dd58 <.text+0x50cd58>
14050d480:     	movq	%rax, %rdi
14050d483:     	leaq	0x6a0(%rbp), %rdx
14050d48a:     	movl	$0x1e0, %r8d            # imm = 0x1E0
14050d490:     	movq	%rax, %rcx
14050d493:     	callq	0x142923c00 <.text+0x2922c00>
14050d498:     	movb	$0x1, 0x96f(%rbp)
14050d49f:     	leaq	0x24e2652(%rip), %r8    # 0x1429efaf8
14050d4a6:     	leaq	0x5c0(%rbp), %rcx
14050d4ad:     	movq	%rdi, %rdx
14050d4b0:     	callq	0x140ce4b10 <.text+0xce3b10>
14050d4b5:     	movaps	0x950(%rbp), %xmm0
14050d4bc:     	movaps	%xmm0, 0x8e0(%rbp)
14050d4c3:     	movaps	0x940(%rbp), %xmm0
14050d4ca:     	movaps	%xmm0, 0x8d0(%rbp)
14050d4d1:     	movaps	0x930(%rbp), %xmm0
14050d4d8:     	movaps	%xmm0, 0x8c0(%rbp)
14050d4df:     	movaps	0x8f0(%rbp), %xmm0
14050d4e6:     	movaps	0x900(%rbp), %xmm1
14050d4ed:     	movaps	0x910(%rbp), %xmm2
14050d4f4:     	movaps	0x920(%rbp), %xmm3
14050d4fb:     	movaps	%xmm3, 0x8b0(%rbp)
14050d502:     	movaps	%xmm2, 0x8a0(%rbp)
14050d509:     	movaps	%xmm1, 0x890(%rbp)
14050d510:     	movaps	%xmm0, 0x880(%rbp)
14050d517:     	leaq	0x550(%rbp), %rcx
14050d51e:     	callq	0x140536540 <.text+0x535540>
14050d523:     	leaq	0x4e0(%rbp), %rcx
14050d52a:     	callq	0x140318670 <.text+0x317670>
14050d52f:     	leaq	0x470(%rbp), %rcx
14050d536:     	callq	0x140307300 <.text+0x306300>
14050d53b:     	leaq	0x400(%rbp), %rcx
14050d542:     	callq	0x1405b4430 <.text+0x5b3430>
14050d547:     	leaq	0x390(%rbp), %rcx
14050d54e:     	callq	0x1405b45f0 <.text+0x5b35f0>
14050d553:     	leaq	0x320(%rbp), %rcx
14050d55a:     	callq	0x1402cf8a0 <.text+0x2ce8a0>
14050d55f:     	leaq	0x2b0(%rbp), %rcx
14050d566:     	callq	0x140325220 <.text+0x324220>
14050d56b:     	leaq	0x240(%rbp), %rcx
14050d572:     	callq	0x140325050 <.text+0x324050>
14050d577:     	leaq	0x1d0(%rbp), %rcx
14050d57e:     	callq	0x1400ecfc0 <.text+0xebfc0>
14050d583:     	leaq	0x160(%rbp), %rcx
14050d58a:     	callq	0x1405bfb20 <.text+0x5beb20>
14050d58f:     	leaq	0xf0(%rbp), %rcx
14050d596:     	callq	0x1400ecc50 <.text+0xebc50>
14050d59b:     	leaq	0x80(%rbp), %rcx
14050d5a2:     	callq	0x1400ece00 <.text+0xebe00>
14050d5a7:     	leaq	0x10(%rbp), %rcx
14050d5ab:     	callq	0x1403146f0 <.text+0x3136f0>
14050d5b0:     	leaq	-0x60(%rbp), %rcx
14050d5b4:     	callq	0x1404e4a70 <.text+0x4e3a70>
14050d5b9:     	leaq	0x630(%rbp), %rcx
14050d5c0:     	callq	0x1403cbe60 <.text+0x3cae60>
14050d5c5:     	leaq	0x6a0(%rbp), %rcx
14050d5cc:     	callq	0x140556f20 <.text+0x555f20>
14050d5d1:     	movups	0x620(%rbp), %xmm0
14050d5d8:     	movq	0x960(%rbp), %rax
14050d5df:     	movups	%xmm0, 0x60(%rax)
14050d5e3:     	movups	0x610(%rbp), %xmm0
14050d5ea:     	movups	%xmm0, 0x50(%rax)
14050d5ee:     	movups	0x600(%rbp), %xmm0
14050d5f5:     	movups	%xmm0, 0x40(%rax)
14050d5f9:     	movups	0x5c0(%rbp), %xmm0
14050d600:     	movups	0x5d0(%rbp), %xmm1
14050d607:     	movups	0x5e0(%rbp), %xmm2
14050d60e:     	movups	0x5f0(%rbp), %xmm3
14050d615:     	movups	%xmm3, 0x30(%rax)
14050d619:     	movups	%xmm2, 0x20(%rax)
14050d61d:     	movups	%xmm1, 0x10(%rax)
14050d621:     	movups	%xmm0, (%rax)
14050d624:     	movaps	0x880(%rbp), %xmm0
14050d62b:     	movaps	0x890(%rbp), %xmm1
14050d632:     	movaps	0x8a0(%rbp), %xmm2
14050d639:     	movaps	0x8b0(%rbp), %xmm3
14050d640:     	movups	%xmm0, 0x70(%rax)
14050d644:     	movups	%xmm1, 0x80(%rax)
14050d64b:     	movups	%xmm2, 0x90(%rax)
14050d652:     	movups	%xmm3, 0xa0(%rax)
14050d659:     	movaps	0x8c0(%rbp), %xmm0
14050d660:     	movups	%xmm0, 0xb0(%rax)
14050d667:     	movaps	0x8d0(%rbp), %xmm0
14050d66e:     	movups	%xmm0, 0xc0(%rax)
14050d675:     	movaps	0x8e0(%rbp), %xmm0
14050d67c:     	movups	%xmm0, 0xd0(%rax)
14050d683:     	movups	0x5b0(%rbp), %xmm0
14050d68a:     	movups	%xmm0, 0x140(%rax)
14050d691:     	movups	0x5a0(%rbp), %xmm0
14050d698:     	movups	%xmm0, 0x130(%rax)
14050d69f:     	movups	0x590(%rbp), %xmm0
14050d6a6:     	movups	%xmm0, 0x120(%rax)
14050d6ad:     	movups	0x550(%rbp), %xmm0
14050d6b4:     	movups	0x560(%rbp), %xmm1
14050d6bb:     	movups	0x570(%rbp), %xmm2
14050d6c2:     	movups	0x580(%rbp), %xmm3
14050d6c9:     	movups	%xmm3, 0x110(%rax)
14050d6d0:     	movups	%xmm2, 0x100(%rax)
14050d6d7:     	movups	%xmm1, 0xf0(%rax)
14050d6de:     	movups	%xmm0, 0xe0(%rax)
14050d6e5:     	movups	0x4e0(%rbp), %xmm0
14050d6ec:     	movups	0x4f0(%rbp), %xmm1
14050d6f3:     	movups	0x500(%rbp), %xmm2
14050d6fa:     	movups	0x510(%rbp), %xmm3
14050d701:     	movups	%xmm0, 0x150(%rax)
14050d708:     	movups	%xmm1, 0x160(%rax)
14050d70f:     	movups	%xmm2, 0x170(%rax)
14050d716:     	movups	%xmm3, 0x180(%rax)
14050d71d:     	movups	0x520(%rbp), %xmm0
14050d724:     	movups	%xmm0, 0x190(%rax)
14050d72b:     	movups	0x530(%rbp), %xmm0
14050d732:     	movups	%xmm0, 0x1a0(%rax)
14050d739:     	movups	0x540(%rbp), %xmm0
14050d740:     	movups	%xmm0, 0x1b0(%rax)
14050d747:     	movups	0x470(%rbp), %xmm0
14050d74e:     	movups	0x480(%rbp), %xmm1
14050d755:     	movups	0x490(%rbp), %xmm2
14050d75c:     	movups	0x4a0(%rbp), %xmm3
14050d763:     	movups	%xmm0, 0x1c0(%rax)
14050d76a:     	movups	%xmm1, 0x1d0(%rax)
14050d771:     	movups	%xmm2, 0x1e0(%rax)
14050d778:     	movups	%xmm3, 0x1f0(%rax)
14050d77f:     	movups	0x4b0(%rbp), %xmm0
14050d786:     	movups	%xmm0, 0x200(%rax)
14050d78d:     	movups	0x4d0(%rbp), %xmm0
14050d794:     	movups	%xmm0, 0x220(%rax)
14050d79b:     	movups	0x4c0(%rbp), %xmm0
14050d7a2:     	movups	%xmm0, 0x210(%rax)
14050d7a9:     	movups	0x460(%rbp), %xmm0
14050d7b0:     	movups	%xmm0, 0x290(%rax)
14050d7b7:     	movups	0x450(%rbp), %xmm0
14050d7be:     	movups	%xmm0, 0x280(%rax)
14050d7c5:     	movups	0x440(%rbp), %xmm0
14050d7cc:     	movups	%xmm0, 0x270(%rax)
14050d7d3:     	movups	0x400(%rbp), %xmm0
14050d7da:     	movups	0x410(%rbp), %xmm1
14050d7e1:     	movups	0x420(%rbp), %xmm2
14050d7e8:     	movups	0x430(%rbp), %xmm3
14050d7ef:     	movups	%xmm3, 0x260(%rax)
14050d7f6:     	movups	%xmm2, 0x250(%rax)
14050d7fd:     	movups	%xmm1, 0x240(%rax)
14050d804:     	movups	%xmm0, 0x230(%rax)
14050d80b:     	movups	0x390(%rbp), %xmm0
14050d812:     	movups	0x3a0(%rbp), %xmm1
14050d819:     	movups	0x3b0(%rbp), %xmm2
14050d820:     	movups	0x3c0(%rbp), %xmm3
14050d827:     	movups	%xmm0, 0x2a0(%rax)
14050d82e:     	movups	%xmm1, 0x2b0(%rax)
14050d835:     	movups	%xmm2, 0x2c0(%rax)
14050d83c:     	movups	%xmm3, 0x2d0(%rax)
14050d843:     	movups	0x3d0(%rbp), %xmm0
14050d84a:     	movups	%xmm0, 0x2e0(%rax)
14050d851:     	movups	0x3e0(%rbp), %xmm0
14050d858:     	movups	%xmm0, 0x2f0(%rax)
14050d85f:     	movups	0x3f0(%rbp), %xmm0
14050d866:     	movups	%xmm0, 0x300(%rax)
14050d86d:     	movups	0x320(%rbp), %xmm0
14050d874:     	movups	0x330(%rbp), %xmm1
14050d87b:     	movups	0x340(%rbp), %xmm2
14050d882:     	movups	0x350(%rbp), %xmm3
14050d889:     	movups	%xmm0, 0x310(%rax)
14050d890:     	movups	%xmm1, 0x320(%rax)
14050d897:     	movups	%xmm2, 0x330(%rax)
14050d89e:     	movups	%xmm3, 0x340(%rax)
14050d8a5:     	movups	0x360(%rbp), %xmm0
14050d8ac:     	movups	%xmm0, 0x350(%rax)
14050d8b3:     	movups	0x370(%rbp), %xmm0
14050d8ba:     	movups	%xmm0, 0x360(%rax)
14050d8c1:     	movups	0x380(%rbp), %xmm0
14050d8c8:     	movups	%xmm0, 0x370(%rax)
14050d8cf:     	movups	0x2b0(%rbp), %xmm0
14050d8d6:     	movups	0x2c0(%rbp), %xmm1
14050d8dd:     	movups	0x2d0(%rbp), %xmm2
14050d8e4:     	movups	0x2e0(%rbp), %xmm3
14050d8eb:     	movups	%xmm0, 0x380(%rax)
14050d8f2:     	movups	%xmm1, 0x390(%rax)
14050d8f9:     	movups	%xmm2, 0x3a0(%rax)
14050d900:     	movups	%xmm3, 0x3b0(%rax)
14050d907:     	movups	0x2f0(%rbp), %xmm0
14050d90e:     	movups	%xmm0, 0x3c0(%rax)
14050d915:     	movups	0x300(%rbp), %xmm0
14050d91c:     	movups	%xmm0, 0x3d0(%rax)
14050d923:     	movups	0x310(%rbp), %xmm0
14050d92a:     	movups	%xmm0, 0x3e0(%rax)
14050d931:     	movups	0x240(%rbp), %xmm0
14050d938:     	movups	0x250(%rbp), %xmm1
14050d93f:     	movups	0x260(%rbp), %xmm2
14050d946:     	movups	0x270(%rbp), %xmm3
14050d94d:     	movups	%xmm0, 0x3f0(%rax)
14050d954:     	movups	%xmm1, 0x400(%rax)
14050d95b:     	movups	%xmm2, 0x410(%rax)
14050d962:     	movups	%xmm3, 0x420(%rax)
14050d969:     	movups	0x280(%rbp), %xmm0
14050d970:     	movups	%xmm0, 0x430(%rax)
14050d977:     	movups	0x290(%rbp), %xmm0
14050d97e:     	movups	%xmm0, 0x440(%rax)
14050d985:     	movups	0x2a0(%rbp), %xmm0
14050d98c:     	movups	%xmm0, 0x450(%rax)
14050d993:     	movups	0x1d0(%rbp), %xmm0
14050d99a:     	movups	0x1e0(%rbp), %xmm1
14050d9a1:     	movups	0x1f0(%rbp), %xmm2
14050d9a8:     	movups	0x200(%rbp), %xmm3
14050d9af:     	movups	%xmm0, 0x460(%rax)
14050d9b6:     	movups	%xmm1, 0x470(%rax)
14050d9bd:     	movups	%xmm2, 0x480(%rax)
14050d9c4:     	movups	%xmm3, 0x490(%rax)
14050d9cb:     	movups	0x210(%rbp), %xmm0
14050d9d2:     	movups	%xmm0, 0x4a0(%rax)
14050d9d9:     	movups	0x220(%rbp), %xmm0
14050d9e0:     	movups	%xmm0, 0x4b0(%rax)
14050d9e7:     	movups	0x230(%rbp), %xmm0
14050d9ee:     	movups	%xmm0, 0x4c0(%rax)
14050d9f5:     	movups	0x160(%rbp), %xmm0
14050d9fc:     	movups	0x170(%rbp), %xmm1
14050da03:     	movups	0x180(%rbp), %xmm2
14050da0a:     	movups	0x190(%rbp), %xmm3
14050da11:     	movups	%xmm0, 0x4d0(%rax)
14050da18:     	movups	%xmm1, 0x4e0(%rax)
14050da1f:     	movups	%xmm2, 0x4f0(%rax)
14050da26:     	movups	%xmm3, 0x500(%rax)
14050da2d:     	movups	0x1a0(%rbp), %xmm0
14050da34:     	movups	%xmm0, 0x510(%rax)
14050da3b:     	movups	0x1b0(%rbp), %xmm0
14050da42:     	movups	%xmm0, 0x520(%rax)
14050da49:     	movups	0x1c0(%rbp), %xmm0
14050da50:     	movups	%xmm0, 0x530(%rax)
14050da57:     	movups	0xf0(%rbp), %xmm0
14050da5e:     	movups	0x100(%rbp), %xmm1
14050da65:     	movups	0x110(%rbp), %xmm2
14050da6c:     	movups	0x120(%rbp), %xmm3
14050da73:     	movups	%xmm0, 0x540(%rax)
14050da7a:     	movups	%xmm1, 0x550(%rax)
14050da81:     	movups	%xmm2, 0x560(%rax)
14050da88:     	movups	%xmm3, 0x570(%rax)
14050da8f:     	movups	0x130(%rbp), %xmm0
14050da96:     	movups	%xmm0, 0x580(%rax)
14050da9d:     	movups	0x140(%rbp), %xmm0
14050daa4:     	movups	%xmm0, 0x590(%rax)
14050daab:     	movups	0x150(%rbp), %xmm0
14050dab2:     	movups	%xmm0, 0x5a0(%rax)
14050dab9:     	movups	0x80(%rbp), %xmm0
14050dac0:     	movups	0x90(%rbp), %xmm1
14050dac7:     	movups	0xa0(%rbp), %xmm2
14050dace:     	movups	0xb0(%rbp), %xmm3
14050dad5:     	movups	%xmm0, 0x5b0(%rax)
14050dadc:     	movups	%xmm1, 0x5c0(%rax)
14050dae3:     	movups	%xmm2, 0x5d0(%rax)
14050daea:     	movups	%xmm3, 0x5e0(%rax)
14050daf1:     	movups	0xc0(%rbp), %xmm0
14050daf8:     	movups	%xmm0, 0x5f0(%rax)
14050daff:     	movups	0xd0(%rbp), %xmm0
14050db06:     	movups	%xmm0, 0x600(%rax)
14050db0d:     	movups	0xe0(%rbp), %xmm0
14050db14:     	movups	%xmm0, 0x610(%rax)
14050db1b:     	movups	0x10(%rbp), %xmm0
14050db1f:     	movups	0x20(%rbp), %xmm1
14050db23:     	movups	0x30(%rbp), %xmm2
14050db27:     	movups	0x40(%rbp), %xmm3
14050db2b:     	movups	%xmm0, 0x620(%rax)
14050db32:     	movups	%xmm1, 0x630(%rax)
14050db39:     	movups	%xmm2, 0x640(%rax)
14050db40:     	movups	%xmm3, 0x650(%rax)
14050db47:     	movups	0x50(%rbp), %xmm0
14050db4b:     	movups	%xmm0, 0x660(%rax)
14050db52:     	movups	0x60(%rbp), %xmm0
14050db56:     	movups	%xmm0, 0x670(%rax)
14050db5d:     	movups	0x70(%rbp), %xmm0
14050db61:     	movups	%xmm0, 0x680(%rax)
14050db68:     	movups	-0x60(%rbp), %xmm0
14050db6c:     	movups	-0x50(%rbp), %xmm1
14050db70:     	movups	-0x40(%rbp), %xmm2
14050db74:     	movups	-0x30(%rbp), %xmm3
14050db78:     	movups	%xmm0, 0x690(%rax)
14050db7f:     	movups	%xmm1, 0x6a0(%rax)
14050db86:     	movups	%xmm2, 0x6b0(%rax)
14050db8d:     	movups	%xmm3, 0x6c0(%rax)
14050db94:     	movups	-0x20(%rbp), %xmm0
14050db98:     	movups	%xmm0, 0x6d0(%rax)
14050db9f:     	movups	-0x10(%rbp), %xmm0
14050dba3:     	movups	%xmm0, 0x6e0(%rax)
14050dbaa:     	movups	(%rbp), %xmm0
14050dbae:     	movups	%xmm0, 0x6f0(%rax)
14050dbb5:     	movups	0x630(%rbp), %xmm0
14050dbbc:     	movups	0x640(%rbp), %xmm1
14050dbc3:     	movups	0x650(%rbp), %xmm2
14050dbca:     	movups	0x660(%rbp), %xmm3
14050dbd1:     	movups	%xmm0, 0x700(%rax)
14050dbd8:     	movups	%xmm1, 0x710(%rax)
14050dbdf:     	movups	%xmm2, 0x720(%rax)
14050dbe6:     	movups	%xmm3, 0x730(%rax)
14050dbed:     	movups	0x670(%rbp), %xmm0
14050dbf4:     	movups	%xmm0, 0x740(%rax)
14050dbfb:     	movups	0x680(%rbp), %xmm0
14050dc02:     	movups	%xmm0, 0x750(%rax)
14050dc09:     	movups	0x690(%rbp), %xmm0
14050dc10:     	movups	%xmm0, 0x760(%rax)
14050dc17:     	movups	0x6a0(%rbp), %xmm0
14050dc1e:     	movups	0x6b0(%rbp), %xmm1
14050dc25:     	movups	0x6c0(%rbp), %xmm2
14050dc2c:     	movups	0x6d0(%rbp), %xmm3
14050dc33:     	movups	%xmm0, 0x770(%rax)
14050dc3a:     	movups	%xmm1, 0x780(%rax)
14050dc41:     	movups	%xmm2, 0x790(%rax)
14050dc48:     	movups	%xmm3, 0x7a0(%rax)
14050dc4f:     	movups	0x6e0(%rbp), %xmm0
14050dc56:     	movups	%xmm0, 0x7b0(%rax)
14050dc5d:     	movups	0x700(%rbp), %xmm0
14050dc64:     	movups	%xmm0, 0x7d0(%rax)
14050dc6b:     	movups	0x6f0(%rbp), %xmm0
14050dc72:     	movups	%xmm0, 0x7c0(%rax)
14050dc79:     	movq	$0x12, 0x6a8(%rbp)
14050dc84:     	movq	%rax, 0x6b0(%rbp)
14050dc8b:     	movq	$0x12, 0x6b8(%rbp)
14050dc96:     	movq	$0x0, 0x6c0(%rbp)
14050dca1:     	movq	$0x8, 0x6c8(%rbp)
14050dcac:     	xorps	%xmm0, %xmm0
14050dcaf:     	movups	%xmm0, 0x6d0(%rbp)
14050dcb6:     	movq	0x640(%rbp), %rax
14050dcbd:     	movq	%rax, 0x6f0(%rbp)
14050dcc4:     	movups	0x630(%rbp), %xmm0
14050dccb:     	movups	%xmm0, 0x6e0(%rbp)
14050dcd2:     	movq	%rbx, 0x6a0(%rbp)
14050dcd9:     	leaq	0x6d8(%rbp), %rcx
14050dce0:     	callq	0x1401705d0 <.text+0x16f5d0>
14050dce5:     	movups	0x700(%rbp), %xmm0
14050dcec:     	movups	%xmm0, 0x60(%rsi)
14050dcf0:     	movups	0x6f0(%rbp), %xmm0
14050dcf7:     	movups	%xmm0, 0x50(%rsi)
14050dcfb:     	movups	0x6e0(%rbp), %xmm0
14050dd02:     	movups	%xmm0, 0x40(%rsi)
14050dd06:     	movups	0x6a0(%rbp), %xmm0
14050dd0d:     	movups	0x6b0(%rbp), %xmm1
14050dd14:     	movups	0x6c0(%rbp), %xmm2
14050dd1b:     	movups	0x6d0(%rbp), %xmm3
14050dd22:     	movups	%xmm3, 0x30(%rsi)
14050dd26:     	movups	%xmm2, 0x20(%rsi)
14050dd2a:     	movups	%xmm1, 0x10(%rsi)
14050dd2e:     	movups	%xmm0, (%rsi)
14050dd31:     	movq	%rsi, %rax
14050dd34:     	addq	$0x9f8, %rsp            # imm = 0x9F8
14050dd3b:     	popq	%rbx
14050dd3c:     	popq	%rdi
14050dd3d:     	popq	%rsi
14050dd3e:     	popq	%rbp
14050dd3f:     	retq
14050dd40:     	movb	$0x1, 0x96e(%rbp)
14050dd47:     	movl	$0x8, %ecx
14050dd4c:     	movl	$0x7e0, %edx            # imm = 0x7E0
14050dd51:     	callq	0x1428d9000 <.text+0x28d8000>
14050dd56:     	jmp	0x14050dd67 <.text+0x50cd67>
14050dd58:     	movl	$0x8, %ecx
14050dd5d:     	movl	$0x1e0, %edx            # imm = 0x1E0
14050dd62:     	callq	0x1428d9000 <.text+0x28d8000>
14050dd67:     	ud2
14050dd69:     	nopl	(%rax)
14050dd70:     	movq	%rdx, 0x10(%rsp)
14050dd75:     	pushq	%rbp
14050dd76:     	pushq	%rsi
14050dd77:     	pushq	%rdi
14050dd78:     	pushq	%rbx
14050dd79:     	subq	$0x28, %rsp
14050dd7d:     	leaq	0x80(%rdx), %rbp
14050dd84:     	movb	$0x1, 0x96f(%rbp)
14050dd8b:     	leaq	0x6a0(%rbp), %rcx
14050dd92:     	callq	0x1405dcec0 <.text+0x5dbec0>
14050dd97:     	movb	$0x1, 0x96f(%rbp)
14050dd9e:     	addq	$0x28, %rsp
14050dda2:     	popq	%rbx
14050dda3:     	popq	%rdi
14050dda4:     	popq	%rsi
14050dda5:     	popq	%rbp
14050dda6:     	retq
14050dda7:     	nopw	(%rax,%rax)
14050ddb0:     	movq	%rdx, 0x10(%rsp)
14050ddb5:     	pushq	%rbp
14050ddb6:     	pushq	%rsi
14050ddb7:     	pushq	%rdi
14050ddb8:     	pushq	%rbx
14050ddb9:     	subq	$0x28, %rsp
14050ddbd:     	leaq	0x80(%rdx), %rbp
14050ddc4:     	movzbl	0x96f(%rbp), %ebx
14050ddcb:     	movl	$0x7e0, %edx            # imm = 0x7E0
14050ddd0:     	movl	$0x8, %r8d
14050ddd6:     	movq	0x960(%rbp), %rcx
14050dddd:     	callq	0x140613c20 <.text+0x612c20>
14050dde2:     	movb	%bl, 0x96e(%rbp)
14050dde8:     	addq	$0x28, %rsp
14050ddec:     	popq	%rbx
14050dded:     	popq	%rdi
14050ddee:     	popq	%rsi
14050ddef:     	popq	%rbp
14050ddf0:     	retq
14050ddf1:     	nopw	%cs:(%rax,%rax)
14050de00:     	movq	%rdx, 0x10(%rsp)
14050de05:     	pushq	%rbp
14050de06:     	pushq	%rsi
14050de07:     	pushq	%rdi
14050de08:     	pushq	%rbx
14050de09:     	subq	$0x28, %rsp
14050de0d:     	leaq	0x80(%rdx), %rbp
14050de14:     	leaq	0x630(%rbp), %rcx
14050de1b:     	callq	0x140029880 <.text+0x28880>
14050de20:     	nop
14050de21:     	addq	$0x28, %rsp
14050de25:     	popq	%rbx
14050de26:     	popq	%rdi
14050de27:     	popq	%rsi
14050de28:     	popq	%rbp
14050de29:     	retq
14050de2a:     	nopw	(%rax,%rax)
14050de30:     	movq	%rdx, 0x10(%rsp)
14050de35:     	pushq	%rbp
14050de36:     	pushq	%rsi
14050de37:     	pushq	%rdi
14050de38:     	pushq	%rbx
14050de39:     	subq	$0x28, %rsp
14050de3d:     	leaq	0x80(%rdx), %rbp
14050de44:     	cmpb	$0x0, 0x96e(%rbp)
14050de4b:     	jne	0x14050de56 <.text+0x50ce56>
14050de4d:     	addq	$0x28, %rsp
14050de51:     	popq	%rbx
14050de52:     	popq	%rdi
14050de53:     	popq	%rsi
14050de54:     	popq	%rbp
14050de55:     	retq
14050de56:     	leaq	0x8f0(%rbp), %rcx
14050de5d:     	callq	0x140029880 <.text+0x28880>
14050de62:     	jmp	0x14050de4d <.text+0x50ce4d>
14050de64:     	nopw	%cs:(%rax,%rax)
14050de70:     	movq	%rdx, 0x10(%rsp)
14050de75:     	pushq	%rbp
14050de76:     	pushq	%rsi
14050de77:     	pushq	%rdi
14050de78:     	pushq	%rbx
14050de79:     	subq	$0x28, %rsp
14050de7d:     	leaq	0x80(%rdx), %rbp
14050de84:     	leaq	0x6a0(%rbp), %rcx
14050de8b:     	callq	0x140029880 <.text+0x28880>
14050de90:     	nop
14050de91:     	addq	$0x28, %rsp
14050de95:     	popq	%rbx
14050de96:     	popq	%rdi
14050de97:     	popq	%rsi
14050de98:     	popq	%rbp
14050de99:     	retq
14050de9a:     	nopw	(%rax,%rax)
14050dea0:     	movq	%rdx, 0x10(%rsp)
14050dea5:     	pushq	%rbp
14050dea6:     	pushq	%rsi
14050dea7:     	pushq	%rdi
14050dea8:     	pushq	%rbx
14050dea9:     	subq	$0x28, %rsp
14050dead:     	leaq	0x80(%rdx), %rbp
14050deb4:     	leaq	0x880(%rbp), %rcx
14050debb:     	callq	0x140029880 <.text+0x28880>
14050dec0:     	leaq	0x5c0(%rbp), %rcx
14050dec7:     	callq	0x140029880 <.text+0x28880>
14050decc:     	movb	$0x0, 0x96f(%rbp)
14050ded3:     	addq	$0x28, %rsp
14050ded7:     	popq	%rbx
14050ded8:     	popq	%rdi
14050ded9:     	popq	%rsi
14050deda:     	popq	%rbp
14050dedb:     	retq
14050dedc:     	nopl	(%rax)
14050dee0:     	movq	%rdx, 0x10(%rsp)
14050dee5:     	pushq	%rbp
14050dee6:     	pushq	%rsi
14050dee7:     	pushq	%rdi
14050dee8:     	pushq	%rbx
14050dee9:     	subq	$0x28, %rsp
14050deed:     	leaq	0x80(%rdx), %rbp
14050def4:     	leaq	0x550(%rbp), %rcx
14050defb:     	callq	0x140029880 <.text+0x28880>
14050df00:     	nop
14050df01:     	addq	$0x28, %rsp
14050df05:     	popq	%rbx
14050df06:     	popq	%rdi
14050df07:     	popq	%rsi
14050df08:     	popq	%rbp
14050df09:     	retq
14050df0a:     	nopw	(%rax,%rax)
14050df10:     	movq	%rdx, 0x10(%rsp)
14050df15:     	pushq	%rbp
14050df16:     	pushq	%rsi
14050df17:     	pushq	%rdi
14050df18:     	pushq	%rbx
14050df19:     	subq	$0x28, %rsp
14050df1d:     	leaq	0x80(%rdx), %rbp
14050df24:     	leaq	0x4e0(%rbp), %rcx
14050df2b:     	callq	0x140029880 <.text+0x28880>
14050df30:     	nop
14050df31:     	addq	$0x28, %rsp
14050df35:     	popq	%rbx
14050df36:     	popq	%rdi
14050df37:     	popq	%rsi
14050df38:     	popq	%rbp
14050df39:     	retq
14050df3a:     	nopw	(%rax,%rax)
14050df40:     	movq	%rdx, 0x10(%rsp)
14050df45:     	pushq	%rbp
14050df46:     	pushq	%rsi
14050df47:     	pushq	%rdi
14050df48:     	pushq	%rbx
14050df49:     	subq	$0x28, %rsp
14050df4d:     	leaq	0x80(%rdx), %rbp
14050df54:     	leaq	0x470(%rbp), %rcx
14050df5b:     	callq	0x140029880 <.text+0x28880>
14050df60:     	nop
14050df61:     	addq	$0x28, %rsp
14050df65:     	popq	%rbx
14050df66:     	popq	%rdi
14050df67:     	popq	%rsi
14050df68:     	popq	%rbp
14050df69:     	retq
14050df6a:     	nopw	(%rax,%rax)
14050df70:     	movq	%rdx, 0x10(%rsp)
14050df75:     	pushq	%rbp
14050df76:     	pushq	%rsi
14050df77:     	pushq	%rdi
14050df78:     	pushq	%rbx
14050df79:     	subq	$0x28, %rsp
14050df7d:     	leaq	0x80(%rdx), %rbp
14050df84:     	leaq	0x400(%rbp), %rcx
14050df8b:     	callq	0x140029880 <.text+0x28880>
14050df90:     	nop
14050df91:     	addq	$0x28, %rsp
14050df95:     	popq	%rbx
14050df96:     	popq	%rdi
14050df97:     	popq	%rsi
14050df98:     	popq	%rbp
14050df99:     	retq
14050df9a:     	nopw	(%rax,%rax)
14050dfa0:     	movq	%rdx, 0x10(%rsp)
14050dfa5:     	pushq	%rbp
14050dfa6:     	pushq	%rsi
14050dfa7:     	pushq	%rdi
14050dfa8:     	pushq	%rbx
14050dfa9:     	subq	$0x28, %rsp
14050dfad:     	leaq	0x80(%rdx), %rbp
14050dfb4:     	leaq	0x390(%rbp), %rcx
14050dfbb:     	callq	0x140029880 <.text+0x28880>
14050dfc0:     	nop
14050dfc1:     	addq	$0x28, %rsp
14050dfc5:     	popq	%rbx
14050dfc6:     	popq	%rdi
14050dfc7:     	popq	%rsi
14050dfc8:     	popq	%rbp
14050dfc9:     	retq
14050dfca:     	nopw	(%rax,%rax)
14050dfd0:     	movq	%rdx, 0x10(%rsp)
14050dfd5:     	pushq	%rbp
14050dfd6:     	pushq	%rsi
14050dfd7:     	pushq	%rdi
14050dfd8:     	pushq	%rbx
14050dfd9:     	subq	$0x28, %rsp
14050dfdd:     	leaq	0x80(%rdx), %rbp
14050dfe4:     	leaq	0x320(%rbp), %rcx
14050dfeb:     	callq	0x140029880 <.text+0x28880>
14050dff0:     	nop
14050dff1:     	addq	$0x28, %rsp
14050dff5:     	popq	%rbx
14050dff6:     	popq	%rdi
14050dff7:     	popq	%rsi
14050dff8:     	popq	%rbp
14050dff9:     	retq
14050dffa:     	nopw	(%rax,%rax)
14050e000:     	movq	%rdx, 0x10(%rsp)
14050e005:     	pushq	%rbp
14050e006:     	pushq	%rsi
14050e007:     	pushq	%rdi
14050e008:     	pushq	%rbx
14050e009:     	subq	$0x28, %rsp
14050e00d:     	leaq	0x80(%rdx), %rbp
14050e014:     	leaq	0x2b0(%rbp), %rcx
14050e01b:     	callq	0x140029880 <.text+0x28880>
14050e020:     	nop
14050e021:     	addq	$0x28, %rsp
14050e025:     	popq	%rbx
14050e026:     	popq	%rdi
14050e027:     	popq	%rsi
14050e028:     	popq	%rbp
14050e029:     	retq
14050e02a:     	nopw	(%rax,%rax)
14050e030:     	movq	%rdx, 0x10(%rsp)
14050e035:     	pushq	%rbp
14050e036:     	pushq	%rsi
14050e037:     	pushq	%rdi
14050e038:     	pushq	%rbx
14050e039:     	subq	$0x28, %rsp
14050e03d:     	leaq	0x80(%rdx), %rbp
14050e044:     	leaq	0x240(%rbp), %rcx
14050e04b:     	callq	0x140029880 <.text+0x28880>
14050e050:     	nop
14050e051:     	addq	$0x28, %rsp
14050e055:     	popq	%rbx
14050e056:     	popq	%rdi
14050e057:     	popq	%rsi
14050e058:     	popq	%rbp
14050e059:     	retq
14050e05a:     	nopw	(%rax,%rax)
14050e060:     	movq	%rdx, 0x10(%rsp)
14050e065:     	pushq	%rbp
14050e066:     	pushq	%rsi
14050e067:     	pushq	%rdi
14050e068:     	pushq	%rbx
14050e069:     	subq	$0x28, %rsp
14050e06d:     	leaq	0x80(%rdx), %rbp
14050e074:     	leaq	0x1d0(%rbp), %rcx
14050e07b:     	callq	0x140029880 <.text+0x28880>
14050e080:     	nop
14050e081:     	addq	$0x28, %rsp
14050e085:     	popq	%rbx
14050e086:     	popq	%rdi
14050e087:     	popq	%rsi
14050e088:     	popq	%rbp
14050e089:     	retq
14050e08a:     	nopw	(%rax,%rax)
14050e090:     	movq	%rdx, 0x10(%rsp)
14050e095:     	pushq	%rbp
14050e096:     	pushq	%rsi
14050e097:     	pushq	%rdi
14050e098:     	pushq	%rbx
14050e099:     	subq	$0x28, %rsp
14050e09d:     	leaq	0x80(%rdx), %rbp
14050e0a4:     	leaq	0x160(%rbp), %rcx
14050e0ab:     	callq	0x140029880 <.text+0x28880>
14050e0b0:     	nop
14050e0b1:     	addq	$0x28, %rsp
14050e0b5:     	popq	%rbx
14050e0b6:     	popq	%rdi
14050e0b7:     	popq	%rsi
14050e0b8:     	popq	%rbp
14050e0b9:     	retq
14050e0ba:     	nopw	(%rax,%rax)
14050e0c0:     	movq	%rdx, 0x10(%rsp)
14050e0c5:     	pushq	%rbp
14050e0c6:     	pushq	%rsi
14050e0c7:     	pushq	%rdi
14050e0c8:     	pushq	%rbx
14050e0c9:     	subq	$0x28, %rsp
14050e0cd:     	leaq	0x80(%rdx), %rbp
14050e0d4:     	leaq	0xf0(%rbp), %rcx
14050e0db:     	callq	0x140029880 <.text+0x28880>
14050e0e0:     	nop
14050e0e1:     	addq	$0x28, %rsp
14050e0e5:     	popq	%rbx
14050e0e6:     	popq	%rdi
14050e0e7:     	popq	%rsi
14050e0e8:     	popq	%rbp
14050e0e9:     	retq
14050e0ea:     	nopw	(%rax,%rax)
14050e0f0:     	movq	%rdx, 0x10(%rsp)
14050e0f5:     	pushq	%rbp
14050e0f6:     	pushq	%rsi
14050e0f7:     	pushq	%rdi
14050e0f8:     	pushq	%rbx
14050e0f9:     	subq	$0x28, %rsp
14050e0fd:     	leaq	0x80(%rdx), %rbp
14050e104:     	leaq	0x80(%rbp), %rcx
14050e10b:     	callq	0x140029880 <.text+0x28880>
14050e110:     	nop
14050e111:     	addq	$0x28, %rsp
14050e115:     	popq	%rbx
14050e116:     	popq	%rdi
14050e117:     	popq	%rsi
14050e118:     	popq	%rbp
14050e119:     	retq
14050e11a:     	nopw	(%rax,%rax)
14050e120:     	movq	%rdx, 0x10(%rsp)
14050e125:     	pushq	%rbp
14050e126:     	pushq	%rsi
14050e127:     	pushq	%rdi
14050e128:     	pushq	%rbx
14050e129:     	subq	$0x28, %rsp
14050e12d:     	leaq	0x80(%rdx), %rbp
14050e134:     	leaq	0x10(%rbp), %rcx
14050e138:     	callq	0x140029880 <.text+0x28880>
14050e13d:     	nop
14050e13e:     	addq	$0x28, %rsp
14050e142:     	popq	%rbx
14050e143:     	popq	%rdi
14050e144:     	popq	%rsi
14050e145:     	popq	%rbp
14050e146:     	retq
14050e147:     	nopw	(%rax,%rax)
14050e150:     	movq	%rdx, 0x10(%rsp)
14050e155:     	pushq	%rbp
14050e156:     	pushq	%rsi
14050e157:     	pushq	%rdi
14050e158:     	pushq	%rbx
14050e159:     	subq	$0x28, %rsp
14050e15d:     	leaq	0x80(%rdx), %rbp
14050e164:     	leaq	-0x60(%rbp), %rcx
14050e168:     	callq	0x140029880 <.text+0x28880>
14050e16d:     	nop
14050e16e:     	addq	$0x28, %rsp
14050e172:     	popq	%rbx
14050e173:     	popq	%rdi
14050e174:     	popq	%rsi
14050e175:     	popq	%rbp
14050e176:     	retq
