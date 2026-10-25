
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e1f480:     	cmpl	$0xc3, 0x58(%rdx)
141e1f487:     	jne	0x141e1f4a6 <.text+0x1e1e4a6>
141e1f489:     	cmpb	$0x1, 0x76(%rcx)
141e1f48d:     	je	0x141e1f4a6 <.text+0x1e1e4a6>
141e1f48f:     	movss	0x4(%rdx), %xmm0
141e1f494:     	mulss	0xdff0fc(%rip), %xmm0   # 0x142c1e598
141e1f49c:     	addss	0x5c(%rcx), %xmm0
141e1f4a1:     	movss	%xmm0, 0x5c(%rcx)
141e1f4a6:     	retq
141e1f4a7:     	int3
141e1f4a8:     	int3
141e1f4a9:     	int3
141e1f4aa:     	int3
141e1f4ab:     	int3
141e1f4ac:     	int3
141e1f4ad:     	int3
141e1f4ae:     	int3
141e1f4af:     	int3
141e1f4b0:     	movq	%rcx, %rax
141e1f4b3:     	movb	$0x0, 0x77(%rdx)
141e1f4b7:     	xorps	%xmm0, %xmm0
141e1f4ba:     	movups	%xmm0, (%rcx)
141e1f4bd:     	retq
141e1f4be:     	int3
141e1f4bf:     	int3
141e1f4c0:     	pushq	%rsi
141e1f4c1:     	pushq	%rdi
141e1f4c2:     	pushq	%rbx
141e1f4c3:     	subq	$0xf0, %rsp
141e1f4ca:     	movaps	%xmm10, 0xe0(%rsp)
141e1f4d3:     	movaps	%xmm9, 0xd0(%rsp)
141e1f4dc:     	movaps	%xmm8, 0xc0(%rsp)
141e1f4e5:     	movaps	%xmm7, 0xb0(%rsp)
141e1f4ed:     	movaps	%xmm6, 0xa0(%rsp)
141e1f4f5:     	movq	%rdx, %rdi
141e1f4f8:     	movq	%rcx, %rsi
141e1f4fb:     	movss	0x14(%rdx), %xmm10
141e1f501:     	movss	0x50(%rcx), %xmm8
141e1f507:     	mulss	%xmm10, %xmm8
141e1f50c:     	movss	0x54(%rcx), %xmm9
141e1f512:     	mulss	%xmm10, %xmm9
141e1f517:     	cmpb	$0x0, 0x76(%rcx)
141e1f51b:     	jne	0x141e1f853 <.text+0x1e1e853>
141e1f521:     	movzbl	0x74(%rsi), %ecx
141e1f525:     	movl	%ecx, %eax
141e1f527:     	xorl	$0x1, %eax
141e1f52a:     	testl	%ecx, %ecx
141e1f52c:     	je	0x141e1f534 <.text+0x1e1e534>
141e1f52e:     	movss	0x10(%rdi), %xmm10
141e1f534:     	movss	0x20(%rdi,%rax,4), %xmm0
141e1f53a:     	movzbl	0x2c(%rdi), %ebx
141e1f53e:     	testb	$0x2, %bl
141e1f541:     	je	0x141e1f54f <.text+0x1e1e54f>
141e1f543:     	movss	%xmm0, 0x64(%rsi)
141e1f548:     	movl	$0x0, 0x68(%rsi)
141e1f54f:     	movss	0x28(%rdi), %xmm6
141e1f554:     	movq	$0x0, (%rsi)
141e1f55b:     	movq	$0x0, 0x10(%rsi)
141e1f563:     	testb	$0x4, %bl
141e1f566:     	jne	0x141e1f594 <.text+0x1e1e594>
141e1f568:     	leaq	0x60(%rsi), %rcx
141e1f56c:     	movss	0x60(%rsi), %xmm0
141e1f571:     	mulss	%xmm6, %xmm0
141e1f575:     	addss	0x5c(%rsi), %xmm0
141e1f57a:     	movss	%xmm0, 0x5c(%rsi)
141e1f57f:     	movss	0xb0f169(%rip), %xmm3   # 0x14292e6f0
141e1f587:     	xorps	%xmm1, %xmm1
141e1f58a:     	movaps	%xmm6, %xmm2
141e1f58d:     	callq	0x140c90670 <.text+0xc8f670>
141e1f592:     	jmp	0x141e1f5e2 <.text+0x1e1e5e2>
141e1f594:     	movss	0x64(%rsi), %xmm1
141e1f599:     	subss	%xmm0, %xmm1
141e1f59d:     	movaps	0xb0f3dc(%rip), %xmm2   # 0x14292e980
141e1f5a4:     	andps	%xmm1, %xmm2
141e1f5a7:     	mulss	0x1c(%rdi), %xmm2
141e1f5ac:     	addss	0x68(%rsi), %xmm2
141e1f5b1:     	movss	%xmm2, 0x68(%rsi)
141e1f5b6:     	mulss	0x6c(%rsi), %xmm1
141e1f5bb:     	movss	0x5c(%rsi), %xmm2
141e1f5c0:     	addss	%xmm1, %xmm2
141e1f5c4:     	movaps	%xmm6, %xmm3
141e1f5c7:     	maxss	0xb317a1(%rip), %xmm3   # 0x142950d70
141e1f5cf:     	movss	%xmm2, 0x5c(%rsi)
141e1f5d4:     	divss	%xmm3, %xmm1
141e1f5d8:     	movss	%xmm1, 0x60(%rsi)
141e1f5dd:     	movss	%xmm0, 0x64(%rsi)
141e1f5e2:     	movl	0x58(%rsi), %ecx
141e1f5e5:     	testl	%ecx, %ecx
141e1f5e7:     	je	0x141e1fcd2 <.text+0x1e1ecd2>
141e1f5ed:     	movq	0x40(%rsi), %rax
141e1f5f1:     	cmpl	$0x1, 0x30(%rsi)
141e1f5f5:     	jne	0x141e1f617 <.text+0x1e1e617>
141e1f5f7:     	xorl	%ecx, %ecx
141e1f5f9:     	subq	$0x1, %rax
141e1f5fd:     	cmovaeq	%rax, %rcx
141e1f601:     	testq	%rcx, %rcx
141e1f604:     	js	0x141e1f6c1 <.text+0x1e1e6c1>
141e1f60a:     	xorps	%xmm0, %xmm0
141e1f60d:     	cvtsi2ss	%rcx, %xmm0
141e1f612:     	jmp	0x141e1f6d9 <.text+0x1e1e6d9>
141e1f617:     	xorl	%edx, %edx
141e1f619:     	divl	%ecx
141e1f61b:     	xorl	%ecx, %ecx
141e1f61d:     	testl	%edx, %edx
141e1f61f:     	setne	%cl
141e1f622:     	leal	(%rax,%rcx), %edx
141e1f625:     	cmpl	$0x1, %edx
141e1f628:     	adcl	%ecx, %eax
141e1f62a:     	movaps	%xmm8, %xmm0
141e1f62e:     	cvtsi2ss	%rax, %xmm7
141e1f633:     	addss	%xmm9, %xmm0
141e1f638:     	mulss	%xmm0, %xmm7
141e1f63c:     	subss	%xmm9, %xmm7
141e1f641:     	movaps	%xmm10, %xmm0
141e1f645:     	minss	%xmm7, %xmm0
141e1f649:     	movaps	%xmm7, %xmm1
141e1f64c:     	cmpunordss	%xmm7, %xmm1
141e1f651:     	movaps	%xmm1, %xmm2
141e1f654:     	andnps	%xmm0, %xmm2
141e1f657:     	andps	%xmm10, %xmm1
141e1f65b:     	orps	%xmm2, %xmm1
141e1f65e:     	movaps	%xmm10, %xmm0
141e1f662:     	subss	%xmm1, %xmm0
141e1f666:     	mulss	0xb3aa7a(%rip), %xmm0   # 0x14295a0e8
141e1f66e:     	movaps	%xmm9, %xmm2
141e1f672:     	addss	%xmm9, %xmm2
141e1f677:     	mulss	0xb061f0(%rip), %xmm10  # 0x142925870
141e1f680:     	subss	%xmm9, %xmm0
141e1f685:     	addss	%xmm0, %xmm7
141e1f689:     	subss	%xmm1, %xmm7
141e1f68d:     	addss	%xmm2, %xmm7
141e1f691:     	movsldup	%xmm10, %xmm1           # xmm1 = xmm10[0,0,2,2]
141e1f696:     	unpcklps	%xmm0, %xmm7            # xmm7 = xmm7[0],xmm0[0],xmm7[1],xmm0[1]
141e1f699:     	addps	%xmm1, %xmm7
141e1f69c:     	movsldup	%xmm8, %xmm0            # xmm0 = xmm8[0,0,2,2]
141e1f6a1:     	divps	%xmm0, %xmm7
141e1f6a4:     	addps	0xbf7b45(%rip), %xmm7   # 0x142a171f0
141e1f6ab:     	cmpb	$0x1, 0x75(%rsi)
141e1f6af:     	je	0x141e1f6f7 <.text+0x1e1e6f7>
141e1f6b1:     	movshdup	%xmm7, %xmm0            # xmm0 = xmm7[1,1,3,3]
141e1f6b5:     	addss	0x5c(%rsi), %xmm0
141e1f6ba:     	movss	%xmm0, 0x5c(%rsi)
141e1f6bf:     	jmp	0x141e1f6f7 <.text+0x1e1e6f7>
141e1f6c1:     	movq	%rcx, %rax
141e1f6c4:     	shrq	%rax
141e1f6c7:     	andl	$0x1, %ecx
141e1f6ca:     	orq	%rax, %rcx
141e1f6cd:     	xorps	%xmm0, %xmm0
141e1f6d0:     	cvtsi2ss	%rcx, %xmm0
141e1f6d5:     	addss	%xmm0, %xmm0
141e1f6d9:     	movss	0x54(%rsi), %xmm7
141e1f6de:     	divss	0x50(%rsi), %xmm7
141e1f6e3:     	addss	0xb06219(%rip), %xmm7   # 0x142925904
141e1f6eb:     	mulss	%xmm0, %xmm7
141e1f6ef:     	movss	0x34(%rsi), %xmm0
141e1f6f4:     	unpcklps	%xmm0, %xmm7            # xmm7 = xmm7[0],xmm0[0],xmm7[1],xmm0[1]
141e1f6f7:     	testb	$0x4, %bl
141e1f6fa:     	jne	0x141e1f78d <.text+0x1e1e78d>
141e1f700:     	leaq	0x5c(%rsi), %rbx
141e1f704:     	movss	0x5c(%rsi), %xmm0
141e1f709:     	movshdup	%xmm7, %xmm1            # xmm1 = xmm7[1,1,3,3]
141e1f70d:     	ucomiss	%xmm0, %xmm1
141e1f710:     	jbe	0x141e1f74f <.text+0x1e1e74f>
141e1f712:     	movaps	%xmm1, %xmm2
141e1f715:     	subss	%xmm0, %xmm2
141e1f719:     	movss	0xb40f53(%rip), %xmm0   # 0x142960674
141e1f721:     	maxss	%xmm2, %xmm0
141e1f725:     	movss	0xb061d7(%rip), %xmm3   # 0x142925904
141e1f72d:     	movaps	%xmm3, %xmm2
141e1f730:     	minss	%xmm0, %xmm2
141e1f734:     	divss	%xmm2, %xmm3
141e1f738:     	mulss	0xbad92c(%rip), %xmm3   # 0x1429cd06c
141e1f740:     	movq	%rbx, %rcx
141e1f743:     	movaps	%xmm6, %xmm2
141e1f746:     	callq	0x140c90670 <.text+0xc8f670>
141e1f74b:     	movss	(%rbx), %xmm0
141e1f74f:     	ucomiss	%xmm7, %xmm0
141e1f752:     	jbe	0x141e1f78d <.text+0x1e1e78d>
141e1f754:     	subss	%xmm7, %xmm0
141e1f758:     	movss	0xb40f14(%rip), %xmm1   # 0x142960674
141e1f760:     	maxss	%xmm0, %xmm1
141e1f764:     	movss	0xb06198(%rip), %xmm3   # 0x142925904
141e1f76c:     	movaps	%xmm3, %xmm0
141e1f76f:     	minss	%xmm1, %xmm0
141e1f773:     	divss	%xmm0, %xmm3
141e1f777:     	mulss	0xbad8ed(%rip), %xmm3   # 0x1429cd06c
141e1f77f:     	movq	%rbx, %rcx
141e1f782:     	movaps	%xmm7, %xmm1
141e1f785:     	movaps	%xmm6, %xmm2
141e1f788:     	callq	0x140c90670 <.text+0xc8f670>
141e1f78d:     	testb	$0x1, 0x20(%rsi)
141e1f791:     	je	0x141e1f853 <.text+0x1e1e853>
141e1f797:     	movq	0x28(%rsi), %rax
141e1f79b:     	movq	$0x1, (%rsi)
141e1f7a2:     	movq	%rax, 0x8(%rsi)
141e1f7a6:     	movl	0x58(%rsi), %ecx
141e1f7a9:     	testq	%rcx, %rcx
141e1f7ac:     	je	0x141e1fcde <.text+0x1e1ecde>
141e1f7b2:     	movss	0x54(%rsi), %xmm1
141e1f7b7:     	divss	0x50(%rsi), %xmm1
141e1f7bc:     	movq	%rax, %rdx
141e1f7bf:     	shrq	$0x20, %rdx
141e1f7c3:     	je	0x141e1f7e1 <.text+0x1e1e7e1>
141e1f7c5:     	xorl	%edx, %edx
141e1f7c7:     	divq	%rcx
141e1f7ca:     	addss	0xb06132(%rip), %xmm1   # 0x142925904
141e1f7d2:     	testq	%rax, %rax
141e1f7d5:     	js	0x141e1f7f2 <.text+0x1e1e7f2>
141e1f7d7:     	xorps	%xmm0, %xmm0
141e1f7da:     	cvtsi2ss	%rax, %xmm0
141e1f7df:     	jmp	0x141e1f80a <.text+0x1e1e80a>
141e1f7e1:     	xorl	%edx, %edx
141e1f7e3:     	divl	%ecx
141e1f7e5:     	addss	0xb06117(%rip), %xmm1   # 0x142925904
141e1f7ed:     	testq	%rax, %rax
141e1f7f0:     	jns	0x141e1f7d7 <.text+0x1e1e7d7>
141e1f7f2:     	movq	%rax, %rcx
141e1f7f5:     	shrq	%rcx
141e1f7f8:     	andl	$0x1, %eax
141e1f7fb:     	orq	%rcx, %rax
141e1f7fe:     	xorps	%xmm0, %xmm0
141e1f801:     	cvtsi2ss	%rax, %xmm0
141e1f806:     	addss	%xmm0, %xmm0
141e1f80a:     	mulss	%xmm0, %xmm1
141e1f80e:     	movshdup	%xmm7, %xmm0            # xmm0 = xmm7[1,1,3,3]
141e1f812:     	movaps	%xmm0, %xmm2
141e1f815:     	maxss	%xmm1, %xmm2
141e1f819:     	cmpunordss	%xmm1, %xmm1
141e1f81e:     	movaps	%xmm1, %xmm3
141e1f821:     	andnps	%xmm2, %xmm3
141e1f824:     	andps	%xmm0, %xmm1
141e1f827:     	orps	%xmm3, %xmm1
141e1f82a:     	movaps	%xmm7, %xmm0
141e1f82d:     	minss	%xmm1, %xmm0
141e1f831:     	cmpunordss	%xmm1, %xmm1
141e1f836:     	movaps	%xmm1, %xmm2
141e1f839:     	andnps	%xmm0, %xmm2
141e1f83c:     	andps	%xmm7, %xmm1
141e1f83f:     	orps	%xmm2, %xmm1
141e1f842:     	leaq	0x5c(%rsi), %rcx
141e1f846:     	movss	0x70(%rsi), %xmm3
141e1f84b:     	movaps	%xmm6, %xmm2
141e1f84e:     	callq	0x140c90670 <.text+0xc8f670>
141e1f853:     	movaps	%xmm8, %xmm0
141e1f857:     	maxss	0xb31511(%rip), %xmm0   # 0x142950d70
141e1f85f:     	movss	0xb0609d(%rip), %xmm2   # 0x142925904
141e1f867:     	movaps	%xmm2, %xmm1
141e1f86a:     	divss	%xmm0, %xmm1
141e1f86e:     	movss	%xmm1, 0x6c(%rsi)
141e1f873:     	movzbl	0x74(%rsi), %eax
141e1f877:     	addss	%xmm8, %xmm9
141e1f87c:     	movaps	%xmm8, %xmm6
141e1f880:     	unpcklps	%xmm9, %xmm6            # xmm6 = xmm6[0],xmm9[0],xmm6[1],xmm9[1]
141e1f884:     	divps	0x1168655(%rip), %xmm6  # 0x142f87ee0
141e1f88b:     	movsd	0x8(%rdi), %xmm1
141e1f890:     	movaps	%xmm6, %xmm0
141e1f893:     	unpcklps	%xmm6, %xmm0            # xmm0 = xmm0[0],xmm6[0],xmm0[1],xmm6[1]
141e1f896:     	movss	0x54(%rsi), %xmm4
141e1f89b:     	divss	0x50(%rsi), %xmm4
141e1f8a0:     	addss	%xmm2, %xmm4
141e1f8a4:     	testb	%al, %al
141e1f8a6:     	je	0x141e1f8d2 <.text+0x1e1e8d2>
141e1f8a8:     	movl	0x58(%rsi), %ecx
141e1f8ab:     	leal	-0x1(%rcx), %edx
141e1f8ae:     	xorps	%xmm3, %xmm3
141e1f8b1:     	cvtsi2ss	%rdx, %xmm3
141e1f8b6:     	movss	0x5c(%rsi), %xmm2
141e1f8bb:     	mulss	0xb05fad(%rip), %xmm3   # 0x142925870
141e1f8c3:     	movaps	%xmm2, %xmm5
141e1f8c6:     	divss	%xmm4, %xmm5
141e1f8ca:     	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
141e1f8cd:     	xorps	%xmm3, %xmm3
141e1f8d0:     	jmp	0x141e1f8f7 <.text+0x1e1e8f7>
141e1f8d2:     	movl	0x58(%rsi), %ecx
141e1f8d5:     	leal	-0x1(%rcx), %edx
141e1f8d8:     	xorps	%xmm2, %xmm2
141e1f8db:     	cvtsi2ss	%rdx, %xmm2
141e1f8e0:     	mulss	0xb05f88(%rip), %xmm2   # 0x142925870
141e1f8e8:     	movss	0x5c(%rsi), %xmm3
141e1f8ed:     	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
141e1f8f0:     	divss	%xmm4, %xmm3
141e1f8f4:     	xorps	%xmm5, %xmm5
141e1f8f7:     	movsldup	%xmm8, %xmm4            # xmm4 = xmm8[0,0,2,2]
141e1f8fc:     	mulps	%xmm2, %xmm4
141e1f8ff:     	subps	%xmm4, %xmm1
141e1f902:     	movl	0x40(%rsi), %edx
141e1f905:     	divps	0x11685d4(%rip), %xmm1  # 0x142f87ee0
141e1f90c:     	movlps	%xmm1, 0x34(%rsp)
141e1f911:     	movups	%xmm0, 0x3c(%rsp)
141e1f916:     	movss	%xmm5, 0x4c(%rsp)
141e1f91c:     	movss	%xmm3, 0x50(%rsp)
141e1f922:     	movb	%al, 0x5c(%rsp)
141e1f926:     	movl	%edx, 0x54(%rsp)
141e1f92a:     	movl	$0x0, 0x2c(%rsp)
141e1f932:     	movl	%ecx, 0x58(%rsp)
141e1f936:     	leaq	0x60(%rsp), %rcx
141e1f93b:     	leaq	0x2c(%rsp), %rdx
141e1f940:     	callq	0x14093fc70 <.text+0x93ec70>
141e1f945:     	movq	0x68(%rsp), %rax
141e1f94a:     	movq	0x78(%rsp), %rcx
141e1f94f:     	cmpq	%rcx, %rax
141e1f952:     	je	0x141e1fbbc <.text+0x1e1ebbc>
141e1f958:     	mulss	0xb3bcd0(%rip), %xmm6   # 0x14295b630
141e1f960:     	mulss	0xb05f08(%rip), %xmm6   # 0x142925870
141e1f968:     	movsd	0x20(%rdi), %xmm0
141e1f96d:     	movq	0x20(%rsi), %rdx
141e1f971:     	movq	0x28(%rsi), %r8
141e1f975:     	testb	$0x8, 0x2c(%rdi)
141e1f979:     	jne	0x141e1fa01 <.text+0x1e1ea01>
141e1f97f:     	movsldup	%xmm6, %xmm1            # xmm1 = xmm6[0,0,2,2]
141e1f983:     	xorl	%r9d, %r9d
141e1f986:     	movaps	0x1168553(%rip), %xmm2  # 0x142f87ee0
141e1f98d:     	jmp	0x141e1f9a0 <.text+0x1e1e9a0>
141e1f98f:     	nop
141e1f990:     	addq	$0x8, %rax
141e1f994:     	incq	%r9
141e1f997:     	cmpq	%rcx, %rax
141e1f99a:     	je	0x141e1fbbc <.text+0x1e1ebbc>
141e1f9a0:     	testq	%rdx, %rdx
141e1f9a3:     	sete	%r10b
141e1f9a7:     	movsd	(%rax), %xmm3
141e1f9ab:     	mulps	%xmm2, %xmm3
141e1f9ae:     	movaps	%xmm3, %xmm4
141e1f9b1:     	subps	%xmm1, %xmm4
141e1f9b4:     	movaps	%xmm1, %xmm5
141e1f9b7:     	addps	%xmm3, %xmm5
141e1f9ba:     	cmpleps	%xmm0, %xmm4
141e1f9be:     	movaps	%xmm0, %xmm6
141e1f9c1:     	cmpleps	%xmm5, %xmm6
141e1f9c5:     	andps	%xmm4, %xmm6
141e1f9c8:     	pshufd	$0x50, %xmm6, %xmm4     # xmm4 = xmm6[0,0,1,1]
141e1f9cd:     	movmskpd	%xmm4, %r11d
141e1f9d2:     	cmpl	$0x3, %r11d
141e1f9d6:     	sete	%r11b
141e1f9da:     	andb	%r10b, %r11b
141e1f9dd:     	cmpq	%r9, %r8
141e1f9e0:     	sete	%r10b
141e1f9e4:     	andb	%dl, %r10b
141e1f9e7:     	orb	%r11b, %r10b
141e1f9ea:     	cmpb	$0x1, %r10b
141e1f9ee:     	jne	0x141e1f990 <.text+0x1e1e990>
141e1f9f0:     	movq	$0x1, (%rsi)
141e1f9f7:     	movq	%r9, 0x8(%rsi)
141e1f9fb:     	movlps	%xmm3, 0x48(%rsi)
141e1f9ff:     	jmp	0x141e1f990 <.text+0x1e1e990>
141e1fa01:     	movss	0xb0ece7(%rip), %xmm1   # 0x14292e6f0
141e1fa09:     	ucomiss	0x68(%rsi), %xmm1
141e1fa0d:     	jbe	0x141e1fac7 <.text+0x1e1eac7>
141e1fa13:     	testq	%rdx, %rdx
141e1fa16:     	je	0x141e1fb4d <.text+0x1e1eb4d>
141e1fa1c:     	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141e1fa20:     	xorl	%r9d, %r9d
141e1fa23:     	movaps	0x11684b6(%rip), %xmm2  # 0x142f87ee0
141e1fa2a:     	jmp	0x141e1fa40 <.text+0x1e1ea40>
141e1fa2c:     	nopl	(%rax)
141e1fa30:     	addq	$0x8, %rax
141e1fa34:     	incq	%r9
141e1fa37:     	cmpq	%rcx, %rax
141e1fa3a:     	je	0x141e1fbbc <.text+0x1e1ebbc>
141e1fa40:     	movsd	(%rax), %xmm3
141e1fa44:     	mulps	%xmm2, %xmm3
141e1fa47:     	cmpq	%r9, %r8
141e1fa4a:     	sete	%r10b
141e1fa4e:     	andb	%dl, %r10b
141e1fa51:     	cmpb	$0x1, %r10b
141e1fa55:     	jne	0x141e1fa66 <.text+0x1e1ea66>
141e1fa57:     	movq	$0x1, (%rsi)
141e1fa5e:     	movq	%r8, 0x8(%rsi)
141e1fa62:     	movlps	%xmm3, 0x48(%rsi)
141e1fa66:     	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
141e1fa6a:     	movaps	%xmm6, %xmm5
141e1fa6d:     	addss	%xmm3, %xmm5
141e1fa71:     	subss	%xmm6, %xmm3
141e1fa75:     	movaps	%xmm6, %xmm7
141e1fa78:     	addss	%xmm4, %xmm7
141e1fa7c:     	subss	%xmm6, %xmm4
141e1fa80:     	movaps	%xmm0, %xmm8
141e1fa84:     	cmpleps	%xmm5, %xmm8
141e1fa89:     	cmpleps	%xmm0, %xmm3
141e1fa8d:     	andps	%xmm8, %xmm3
141e1fa91:     	movd	%xmm3, %r10d
141e1fa96:     	movaps	%xmm1, %xmm3
141e1fa99:     	cmpleps	%xmm7, %xmm3
141e1fa9d:     	cmpleps	%xmm1, %xmm4
141e1faa1:     	andps	%xmm3, %xmm4
141e1faa4:     	movd	%xmm4, %r11d
141e1faa9:     	andl	%r10d, %r11d
141e1faac:     	testb	$0x1, %r11b
141e1fab0:     	je	0x141e1fa30 <.text+0x1e1ea30>
141e1fab6:     	movq	$0x1, 0x10(%rsi)
141e1fabe:     	movq	%r9, 0x18(%rsi)
141e1fac2:     	jmp	0x141e1fa30 <.text+0x1e1ea30>
141e1fac7:     	movddup	%xmm0, %xmm0            # xmm0 = xmm0[0,0]
141e1facb:     	shufps	$0x0, %xmm6, %xmm6      # xmm6 = xmm6[0,0,0,0]
141e1facf:     	xorl	%r9d, %r9d
141e1fad2:     	movaps	0x1168407(%rip), %xmm1  # 0x142f87ee0
141e1fad9:     	jmp	0x141e1fb00 <.text+0x1e1eb00>
141e1fadb:     	nopl	(%rax,%rax)
141e1fae0:     	movq	$0x1, (%rsi)
141e1fae7:     	movq	%r9, 0x8(%rsi)
141e1faeb:     	movlpd	%xmm2, 0x48(%rsi)
141e1faf0:     	addq	$0x8, %rax
141e1faf4:     	incq	%r9
141e1faf7:     	cmpq	%rcx, %rax
141e1fafa:     	je	0x141e1fbbc <.text+0x1e1ebbc>
141e1fb00:     	testq	%rdx, %rdx
141e1fb03:     	sete	%r11b
141e1fb07:     	movsd	(%rax), %xmm2
141e1fb0b:     	mulps	%xmm1, %xmm2
141e1fb0e:     	movddup	%xmm2, %xmm3            # xmm3 = xmm2[0,0]
141e1fb12:     	movaps	%xmm2, %xmm4
141e1fb15:     	addps	%xmm6, %xmm4
141e1fb18:     	subps	%xmm6, %xmm3
141e1fb1b:     	movsd	%xmm4, %xmm3            # xmm3 = xmm4[0],xmm3[1]
141e1fb1f:     	movapd	%xmm0, %xmm4
141e1fb23:     	cmpleps	%xmm3, %xmm4
141e1fb27:     	cmpleps	%xmm0, %xmm3
141e1fb2b:     	movsd	%xmm4, %xmm3            # xmm3 = xmm4[0],xmm3[1]
141e1fb2f:     	movmskps	%xmm3, %r10d
141e1fb33:     	cmpl	$0xf, %r10d
141e1fb37:     	sete	%bl
141e1fb3a:     	cmpq	%r9, %r8
141e1fb3d:     	sete	%r10b
141e1fb41:     	testb	%r11b, %bl
141e1fb44:     	jne	0x141e1fae0 <.text+0x1e1eae0>
141e1fb46:     	andb	%dl, %r10b
141e1fb49:     	je	0x141e1faf0 <.text+0x1e1eaf0>
141e1fb4b:     	jmp	0x141e1fae0 <.text+0x1e1eae0>
141e1fb4d:     	movddup	%xmm0, %xmm0            # xmm0 = xmm0[0,0]
141e1fb51:     	shufps	$0x0, %xmm6, %xmm6      # xmm6 = xmm6[0,0,0,0]
141e1fb55:     	xorl	%edx, %edx
141e1fb57:     	movaps	0x1168382(%rip), %xmm1  # 0x142f87ee0
141e1fb5e:     	jmp	0x141e1fb6c <.text+0x1e1eb6c>
141e1fb60:     	addq	$0x8, %rax
141e1fb64:     	incq	%rdx
141e1fb67:     	cmpq	%rcx, %rax
141e1fb6a:     	je	0x141e1fbbc <.text+0x1e1ebbc>
141e1fb6c:     	movsd	(%rax), %xmm2
141e1fb70:     	mulps	%xmm1, %xmm2
141e1fb73:     	movddup	%xmm2, %xmm3            # xmm3 = xmm2[0,0]
141e1fb77:     	movaps	%xmm2, %xmm4
141e1fb7a:     	addps	%xmm6, %xmm4
141e1fb7d:     	subps	%xmm6, %xmm3
141e1fb80:     	movsd	%xmm4, %xmm3            # xmm3 = xmm4[0],xmm3[1]
141e1fb84:     	movapd	%xmm0, %xmm4
141e1fb88:     	cmpleps	%xmm3, %xmm4
141e1fb8c:     	cmpleps	%xmm0, %xmm3
141e1fb90:     	movsd	%xmm4, %xmm3            # xmm3 = xmm4[0],xmm3[1]
141e1fb94:     	movmskps	%xmm3, %r8d
141e1fb98:     	cmpl	$0xf, %r8d
141e1fb9c:     	jne	0x141e1fb60 <.text+0x1e1eb60>
141e1fb9e:     	movq	$0x1, (%rsi)
141e1fba5:     	movq	%rdx, 0x8(%rsi)
141e1fba9:     	movlpd	%xmm2, 0x48(%rsi)
141e1fbae:     	movq	$0x1, 0x10(%rsi)
141e1fbb6:     	movq	%rdx, 0x18(%rsi)
141e1fbba:     	jmp	0x141e1fb60 <.text+0x1e1eb60>
141e1fbbc:     	movq	0x70(%rsp), %rdx
141e1fbc1:     	testq	%rdx, %rdx
141e1fbc4:     	je	0x141e1fbda <.text+0x1e1ebda>
141e1fbc6:     	movq	0x60(%rsp), %rcx
141e1fbcb:     	shlq	$0x3, %rdx
141e1fbcf:     	movl	$0x4, %r8d
141e1fbd5:     	callq	0x140613c20 <.text+0x612c20>
141e1fbda:     	movl	(%rsi), %eax
141e1fbdc:     	movl	0x8(%rsi), %ecx
141e1fbdf:     	movl	%eax, 0x2c(%rsp)
141e1fbe3:     	movl	%ecx, 0x30(%rsp)
141e1fbe7:     	movl	0x5c(%rsp), %eax
141e1fbeb:     	movl	%eax, 0x90(%rsp)
141e1fbf2:     	movups	0x3c(%rsp), %xmm0
141e1fbf7:     	movups	0x4c(%rsp), %xmm1
141e1fbfc:     	movaps	%xmm1, 0x80(%rsp)
141e1fc04:     	movaps	%xmm0, 0x70(%rsp)
141e1fc09:     	movl	0x2c(%rsp), %eax
141e1fc0d:     	movl	%eax, 0x60(%rsp)
141e1fc11:     	movl	0x30(%rsp), %eax
141e1fc15:     	movl	%eax, 0x64(%rsp)
141e1fc19:     	movq	0x34(%rsp), %rax
141e1fc1e:     	movq	%rax, 0x68(%rsp)
141e1fc23:     	movq	0x38(%rsi), %rbx
141e1fc27:     	leaq	0x10(%rbx), %rdi
141e1fc2b:     	movb	$0x1, %cl
141e1fc2d:     	xorl	%eax, %eax
141e1fc2f:     	lock
141e1fc30:     	cmpxchgb	%cl, 0x10(%rbx)
141e1fc34:     	jne	0x141e1fcb3 <.text+0x1e1ecb3>
141e1fc36:     	movq	%rdi, %rcx
141e1fc39:     	callq	0x140c81c70 <.text+0xc80c70>
141e1fc3e:     	movl	0x90(%rsp), %eax
141e1fc45:     	movl	%eax, 0x44(%rbx)
141e1fc48:     	movaps	0x60(%rsp), %xmm0
141e1fc4d:     	movaps	0x70(%rsp), %xmm1
141e1fc52:     	movaps	0x80(%rsp), %xmm2
141e1fc5a:     	movups	%xmm2, 0x34(%rbx)
141e1fc5e:     	movups	%xmm1, 0x24(%rbx)
141e1fc62:     	movups	%xmm0, 0x14(%rbx)
141e1fc66:     	movq	%rdi, %rcx
141e1fc69:     	callq	0x140c81e10 <.text+0xc80e10>
141e1fc6e:     	xorl	%ecx, %ecx
141e1fc70:     	movb	$0x1, %al
141e1fc72:     	lock
141e1fc73:     	cmpxchgb	%cl, 0x10(%rbx)
141e1fc77:     	jne	0x141e1fcc6 <.text+0x1e1ecc6>
141e1fc79:     	movb	$0x1, 0x75(%rsi)
141e1fc7d:     	movaps	0xa0(%rsp), %xmm6
141e1fc85:     	movaps	0xb0(%rsp), %xmm7
141e1fc8d:     	movaps	0xc0(%rsp), %xmm8
141e1fc96:     	movaps	0xd0(%rsp), %xmm9
141e1fc9f:     	movaps	0xe0(%rsp), %xmm10
141e1fca8:     	addq	$0xf0, %rsp
141e1fcaf:     	popq	%rbx
141e1fcb0:     	popq	%rdi
141e1fcb1:     	popq	%rsi
141e1fcb2:     	retq
141e1fcb3:     	movq	%rdi, %rcx
141e1fcb6:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
141e1fcbc:     	callq	0x1428cd210 <.text+0x28cc210>
141e1fcc1:     	jmp	0x141e1fc36 <.text+0x1e1ec36>
141e1fcc6:     	movq	%rdi, %rcx
141e1fcc9:     	xorl	%edx, %edx
141e1fccb:     	callq	0x1428cdb90 <.text+0x28ccb90>
141e1fcd0:     	jmp	0x141e1fc79 <.text+0x1e1ec79>
141e1fcd2:     	leaq	0x11684cf(%rip), %rcx   # 0x142f881a8
141e1fcd9:     	callq	0x1428d99e0 <.text+0x28d89e0>
141e1fcde:     	leaq	0x11684db(%rip), %rcx   # 0x142f881c0
141e1fce5:     	callq	0x1428d99e0 <.text+0x28d89e0>
141e1fcea:     	int3
141e1fceb:     	int3
141e1fcec:     	int3
141e1fced:     	int3
141e1fcee:     	int3
141e1fcef:     	int3
141e1fcf0:     	pushq	%r14
141e1fcf2:     	pushq	%rsi
141e1fcf3:     	pushq	%rdi
141e1fcf4:     	pushq	%rbp
141e1fcf5:     	pushq	%rbx
141e1fcf6:     	subq	$0x30, %rsp
141e1fcfa:     	cmpb	$0x1, (%r8)
141e1fcfe:     	jne	0x141e1fe54 <.text+0x1e1ee54>
141e1fd04:     	movq	%rdx, %rdi
141e1fd07:     	movq	%rcx, %rsi
141e1fd0a:     	xorl	%ebx, %ebx
141e1fd0c:     	leaq	0x24(%rsp), %rcx
141e1fd11:     	xorl	%r8d, %r8d
141e1fd14:     	callq	0x140948800 <.text+0x947800>
141e1fd19:     	movq	0x28(%rsp), %xmm0
141e1fd1f:     	movd	0x24(%rsp), %xmm1
141e1fd25:     	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
141e1fd2a:     	pslld	$0x1f, %xmm1
141e1fd2f:     	psrad	$0x1f, %xmm1
141e1fd34:     	pand	%xmm0, %xmm1
141e1fd38:     	mulps	%xmm1, %xmm1
141e1fd3b:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
141e1fd3f:     	addss	%xmm1, %xmm0
141e1fd43:     	sqrtss	%xmm0, %xmm0
141e1fd47:     	movb	$0x1, %bpl
141e1fd4a:     	movl	$0x20, %r14d
141e1fd50:     	ucomiss	0xb05bad(%rip), %xmm0   # 0x142925904
141e1fd57:     	ja	0x141e1fd69 <.text+0x1e1ed69>
141e1fd59:     	xorl	%ebx, %ebx
141e1fd5b:     	movq	%rdi, %rcx
141e1fd5e:     	xorl	%edx, %edx
141e1fd60:     	callq	0x1409487b0 <.text+0x9477b0>
141e1fd65:     	testb	%al, %al
141e1fd67:     	je	0x141e1fda5 <.text+0x1e1eda5>
141e1fd69:     	xorl	%eax, %eax
141e1fd6b:     	movq	%rax, (%rsi,%r14)
141e1fd6f:     	movq	%rdi, %rcx
141e1fd72:     	movb	$0x14, %dl
141e1fd74:     	callq	0x1409487b0 <.text+0x9477b0>
141e1fd79:     	testb	%al, %al
141e1fd7b:     	je	0x141e1fd8b <.text+0x1e1ed8b>
141e1fd7d:     	xorl	%eax, %eax
141e1fd7f:     	subl	0x58(%rsi), %eax
141e1fd82:     	testl	%eax, %eax
141e1fd84:     	jne	0x141e1fdf3 <.text+0x1e1edf3>
141e1fd86:     	jmp	0x141e1fe54 <.text+0x1e1ee54>
141e1fd8b:     	movq	%rdi, %rcx
141e1fd8e:     	movb	$0x15, %dl
141e1fd90:     	callq	0x1409487b0 <.text+0x9477b0>
141e1fd95:     	testb	%al, %al
141e1fd97:     	je	0x141e1fdda <.text+0x1e1edda>
141e1fd99:     	movl	0x58(%rsi), %eax
141e1fd9c:     	testl	%eax, %eax
141e1fd9e:     	jne	0x141e1fdf3 <.text+0x1e1edf3>
141e1fda0:     	jmp	0x141e1fe54 <.text+0x1e1ee54>
141e1fda5:     	cmpq	$0x0, 0x20(%rsi)
141e1fdaa:     	je	0x141e1fe61 <.text+0x1e1ee61>
141e1fdb0:     	movq	%rdi, %rcx
141e1fdb3:     	movb	$0x13, %dl
141e1fdb5:     	callq	0x1409487b0 <.text+0x9477b0>
141e1fdba:     	testb	%al, %al
141e1fdbc:     	je	0x141e1fe6b <.text+0x1e1ee6b>
141e1fdc2:     	movq	0x28(%rsi), %rax
141e1fdc6:     	movq	$0x1, 0x10(%rsi)
141e1fdce:     	movb	$0x1, %bl
141e1fdd0:     	movl	$0x18, %r14d
141e1fdd6:     	xorl	%ebp, %ebp
141e1fdd8:     	jmp	0x141e1fd6b <.text+0x1e1ed6b>
141e1fdda:     	movq	%rdi, %rcx
141e1fddd:     	movb	$0x17, %dl
141e1fddf:     	callq	0x1409487b0 <.text+0x9477b0>
141e1fde4:     	testb	%al, %al
141e1fde6:     	je	0x141e1fe37 <.text+0x1e1ee37>
141e1fde8:     	movl	$0xffffffff, %eax       # imm = 0xFFFFFFFF
141e1fded:     	cmpl	$0x1, 0x58(%rsi)
141e1fdf1:     	jbe	0x141e1fe50 <.text+0x1e1ee50>
141e1fdf3:     	testb	%bpl, %bpl
141e1fdf6:     	je	0x141e1fe03 <.text+0x1e1ee03>
141e1fdf8:     	cmpb	$0x0, (%rsi)
141e1fdfb:     	je	0x141e1fe0e <.text+0x1e1ee0e>
141e1fdfd:     	movq	0x8(%rsi), %rcx
141e1fe01:     	jmp	0x141e1fe10 <.text+0x1e1ee10>
141e1fe03:     	xorl	%ecx, %ecx
141e1fe05:     	testb	%bl, %bl
141e1fe07:     	je	0x141e1fe1c <.text+0x1e1ee1c>
141e1fe09:     	movl	0x28(%rsi), %ecx
141e1fe0c:     	jmp	0x141e1fe1c <.text+0x1e1ee1c>
141e1fe0e:     	xorl	%ecx, %ecx
141e1fe10:     	movq	$0x1, 0x20(%rsi)
141e1fe18:     	movq	%rcx, 0x28(%rsi)
141e1fe1c:     	addl	%eax, %ecx
141e1fe1e:     	movb	$0x1, %al
141e1fe20:     	js	0x141e1fe56 <.text+0x1e1ee56>
141e1fe22:     	cmpl	0x40(%rsi), %ecx
141e1fe25:     	jge	0x141e1fe56 <.text+0x1e1ee56>
141e1fe27:     	movl	%ecx, %ecx
141e1fe29:     	movq	$0x1, 0x20(%rsi)
141e1fe31:     	movq	%rcx, 0x28(%rsi)
141e1fe35:     	jmp	0x141e1fe56 <.text+0x1e1ee56>
141e1fe37:     	movq	%rdi, %rcx
141e1fe3a:     	movb	$0x16, %dl
141e1fe3c:     	callq	0x1409487b0 <.text+0x9477b0>
141e1fe41:     	testb	%al, %al
141e1fe43:     	je	0x141e1fe54 <.text+0x1e1ee54>
141e1fe45:     	movl	$0x1, %eax
141e1fe4a:     	cmpl	$0x1, 0x58(%rsi)
141e1fe4e:     	ja	0x141e1fdf3 <.text+0x1e1edf3>
141e1fe50:     	movb	$0x1, 0x77(%rsi)
141e1fe54:     	xorl	%eax, %eax
141e1fe56:     	addq	$0x30, %rsp
141e1fe5a:     	popq	%rbx
141e1fe5b:     	popq	%rbp
141e1fe5c:     	popq	%rdi
141e1fe5d:     	popq	%rsi
141e1fe5e:     	popq	%r14
141e1fe60:     	retq
141e1fe61:     	movb	$0x1, %bpl
141e1fe64:     	xorl	%ebx, %ebx
141e1fe66:     	jmp	0x141e1fd6f <.text+0x1e1ed6f>
141e1fe6b:     	movb	$0x1, %bl
141e1fe6d:     	xorl	%ebp, %ebp
141e1fe6f:     	jmp	0x141e1fd6f <.text+0x1e1ed6f>
