
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141ec04d0:     	movq	%rsi, %r9
141ec04d3:     	callq	0x140c16d40 <.text+0xc15d40>
141ec04d8:     	movq	%rax, %rsi
141ec04db:     	movq	0x8(%rbp), %rax
141ec04df:     	incq	(%rax)
141ec04e2:     	movq	%rsi, (%rbp)
141ec04e6:     	movl	$0x1, %eax
141ec04eb:     	movq	%r14, %r13
141ec04ee:     	movq	0x98(%rbp), %r12
141ec04f5:     	movq	0x78(%rbp), %r14
141ec04f9:     	movq	%rax, -0x8(%rbp)
141ec04fd:     	movq	0x176bb94(%rip), %rax   # 0x14362c098
141ec0504:     	cmpb	$0x0, (%rax)
141ec0507:     	jne	0x141ec032e <.text+0x1ebf32e>
141ec050d:     	movq	(%r15), %rax
141ec0510:     	cmpl	$0xa, (%rax)
141ec0513:     	jne	0x141ec08b8 <.text+0x1ebf8b8>
141ec0519:     	cmpb	$0x1, 0x8(%rax)
141ec051d:     	jne	0x141ec08b8 <.text+0x1ebf8b8>
141ec0523:     	movq	0x120(%rbp), %rax
141ec052a:     	movl	0x1c(%rax), %ecx
141ec052d:     	movq	0x10(%rax), %rdx
141ec0531:     	movl	%ecx, 0x98(%rbp)
141ec0537:     	movq	%rdx, 0x78(%rbp)
141ec053b:     	movl	%ecx, (%rdx)
141ec053d:     	movq	(%rax), %rdi
141ec0540:     	leaq	0x10dafb4(%rip), %rax   # 0x142f9b4fb
141ec0547:     	movq	%rax, 0x10(%rbp)
141ec054b:     	movq	$0xb, 0x18(%rbp)
141ec0553:     	leaq	0x10(%rbp), %rcx
141ec0557:     	callq	0x1405bdda0 <.text+0x5bcda0>
141ec055c:     	movq	%rax, %rsi
141ec055f:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141ec0569:     	xorq	%rax, %rsi
141ec056c:     	movq	%rdi, %rcx
141ec056f:     	movq	%rsi, %rdx
141ec0572:     	callq	0x140683ec0 <.text+0x682ec0>
141ec0577:     	movb	$0x0, 0x5d(%rax)
141ec057b:     	movb	$0x1, 0xb7(%rax)
141ec0582:     	testb	$0x1, %dl
141ec0585:     	jne	0x141ec05ba <.text+0x1ebf5ba>
141ec0587:     	movq	0x10(%rdi), %rbx
141ec058b:     	cmpq	(%rdi), %rbx
141ec058e:     	jne	0x141ec059f <.text+0x1ebf59f>
141ec0590:     	leaq	0xceea31(%rip), %rdx    # 0x142baefc8
141ec0597:     	movq	%rdi, %rcx
141ec059a:     	callq	0x1406319d0 <.text+0x6309d0>
141ec059f:     	movq	0x8(%rdi), %rax
141ec05a3:     	imulq	$0x38, %rbx, %rcx
141ec05a7:     	movl	$0x0, (%rax,%rcx)
141ec05ae:     	movq	%rsi, 0x30(%rax,%rcx)
141ec05b3:     	incq	%rbx
141ec05b6:     	movq	%rbx, 0x10(%rdi)
141ec05ba:     	movq	%rdi, -0x48(%rbp)
141ec05be:     	movq	%rsi, -0x40(%rbp)
141ec05c2:     	movq	$0x0, -0x38(%rbp)
141ec05ca:     	movq	(%r14), %r14
141ec05cd:     	movq	0x10(%r14), %r8
141ec05d1:     	movq	(%r12), %rax
141ec05d5:     	movq	(%rax), %r9
141ec05d8:     	lock
141ec05d9:     	incq	(%r9)
141ec05dc:     	jle	0x141ec0b66 <.text+0x1ebfb66>
141ec05e2:     	incq	%r8
141ec05e5:     	leaq	0x10daf1a(%rip), %rax   # 0x142f9b506
141ec05ec:     	movq	%rax, 0x20(%rsp)
141ec05f1:     	movq	$0x4, 0x28(%rsp)
141ec05fa:     	leaq	0x10(%rbp), %rcx
141ec05fe:     	leaq	-0x48(%rbp), %rdx
141ec0602:     	callq	0x140fa06a0 <.text+0xf9f6a0>
141ec0607:     	leaq	0x10(%rbp), %rcx
141ec060b:     	movb	$0x1, %dl
141ec060d:     	callq	0x140f9d590 <.text+0xf9c590>
141ec0612:     	cmpb	$0x0, 0x75(%rax)
141ec0616:     	jne	0x141ec0636 <.text+0x1ebf636>
141ec0618:     	movss	0x54(%rax), %xmm0
141ec061d:     	divss	0x50(%rax), %xmm0
141ec0622:     	addss	0xa652da(%rip), %xmm0   # 0x142925904
141ec062a:     	xorps	%xmm1, %xmm1
141ec062d:     	mulss	%xmm0, %xmm1
141ec0631:     	movss	%xmm1, 0x5c(%rax)
141ec0636:     	movq	0x20(%rbp), %rax
141ec063a:     	movq	%rax, 0x90(%rbp)
141ec0641:     	movups	0x10(%rbp), %xmm0
141ec0645:     	movaps	%xmm0, 0x80(%rbp)
141ec064c:     	leaq	0x80(%rbp), %rcx
141ec0653:     	movb	$0x1, %dl
141ec0655:     	callq	0x140f9d590 <.text+0xf9c590>
141ec065a:     	movl	$0x3ed0e560, 0x50(%rax) # imm = 0x3ED0E560
141ec0661:     	movq	0x90(%rbp), %rax
141ec0668:     	movq	%rax, 0x20(%rbp)
141ec066c:     	movaps	0x80(%rbp), %xmm0
141ec0673:     	movaps	%xmm0, 0x10(%rbp)
141ec0677:     	leaq	0x10(%rbp), %rcx
141ec067b:     	movb	$0x1, %dl
141ec067d:     	callq	0x140f9d590 <.text+0xf9c590>
141ec0682:     	movl	$0x3c9374bc, 0x54(%rax) # imm = 0x3C9374BC
141ec0689:     	movq	0x20(%rbp), %rax
141ec068d:     	movq	%rax, 0x90(%rbp)
141ec0694:     	movaps	0x10(%rbp), %xmm0
141ec0698:     	movaps	%xmm0, 0x80(%rbp)
141ec069f:     	leaq	0x80(%rbp), %rcx
141ec06a6:     	movb	$0x1, %dl
141ec06a8:     	callq	0x140f9d590 <.text+0xf9c590>
141ec06ad:     	movl	$0x40000000, 0x70(%rax) # imm = 0x40000000
141ec06b4:     	movq	0x90(%rbp), %rax
141ec06bb:     	movq	%rax, 0x20(%rbp)
141ec06bf:     	movaps	0x80(%rbp), %xmm0
141ec06c6:     	movaps	%xmm0, 0x10(%rbp)
141ec06ca:     	leaq	0x10(%rbp), %rcx
141ec06ce:     	movb	$0x1, %dl
141ec06d0:     	callq	0x140f9d590 <.text+0xf9c590>
141ec06d5:     	movb	$0x1, 0x74(%rax)
141ec06d9:     	movq	0x20(%rbp), %rax
141ec06dd:     	movq	%rax, 0x90(%rbp)
141ec06e4:     	movaps	0x10(%rbp), %xmm0
141ec06e8:     	movaps	%xmm0, 0x80(%rbp)
141ec06ef:     	leaq	0x80(%rbp), %rcx
141ec06f6:     	movb	$0x1, %dl
141ec06f8:     	callq	0x140f9d590 <.text+0xf9c590>
141ec06fd:     	movl	$0x2, 0x58(%rax)
141ec0704:     	movq	0x90(%rbp), %rax
141ec070b:     	movq	%rax, -0x20(%rbp)
141ec070f:     	movaps	0x80(%rbp), %xmm0
141ec0716:     	movaps	%xmm0, -0x30(%rbp)
141ec071a:     	leaq	-0x30(%rbp), %rcx
141ec071e:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141ec0723:     	movq	0x128(%rbp), %rbx
141ec072a:     	cmpq	$0x0, (%rax)
141ec072e:     	je	0x141ec078a <.text+0x1ebf78a>
141ec0730:     	testb	$0x1, (%r13)
141ec0735:     	je	0x141ec0741 <.text+0x1ebf741>
141ec0737:     	movq	0x8(%r13), %rcx
141ec073b:     	cmpq	%rcx, 0x8(%rax)
141ec073f:     	je	0x141ec078a <.text+0x1ebf78a>
141ec0741:     	movl	0x1c(%rbx), %eax
141ec0744:     	movq	0x10(%rbx), %rcx
141ec0748:     	movl	%eax, (%rcx)
141ec074a:     	movq	(%rbx), %rsi
141ec074d:     	movq	0x30(%rsi), %r15
141ec0751:     	movq	0x40(%rsi), %r12
141ec0755:     	cmpq	0x20(%rsi), %r15
141ec0759:     	jne	0x141ec076b <.text+0x1ebf76b>
141ec075b:     	leaq	0x20(%rsi), %rcx
141ec075f:     	leaq	0xcf4b0a(%rip), %rdx    # 0x142bb5270
141ec0766:     	callq	0x140562630 <.text+0x561630>
141ec076b:     	movq	0x28(%rsi), %rax
141ec076f:     	movq	%r15, %rcx
141ec0772:     	shlq	$0x4, %rcx
141ec0776:     	movq	%r12, (%rax,%rcx)
141ec077a:     	movb	$0x0, 0x8(%rax,%rcx)
141ec077f:     	incq	%r15
141ec0782:     	movq	%r15, 0x30(%rsi)
141ec0786:     	incq	0x40(%rsi)
141ec078a:     	leaq	-0x30(%rbp), %rcx
141ec078e:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141ec0793:     	movups	(%rax), %xmm0
141ec0796:     	movups	%xmm0, (%r13)
141ec079b:     	leaq	-0x30(%rbp), %rcx
141ec079f:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141ec07a4:     	cmpq	$0x0, 0x10(%rax)
141ec07a9:     	je	0x141ec09ff <.text+0x1ebf9ff>
141ec07af:     	movq	0x18(%rax), %r15
141ec07b3:     	movq	0x110(%rbp), %rdx
141ec07ba:     	movl	0x1c(%rdx), %eax
141ec07bd:     	movq	0x10(%rdx), %rcx
141ec07c1:     	movl	%eax, (%rcx)
141ec07c3:     	movq	(%rdx), %rcx
141ec07c6:     	movq	$0x2, 0x10(%rbp)
141ec07ce:     	leaq	0x10(%rbp), %rdx
141ec07d2:     	callq	0x14093fad0 <.text+0x93ead0>
141ec07d7:     	movl	0x1c(%rbx), %eax
141ec07da:     	movq	0x10(%rbx), %rcx
141ec07de:     	movl	%eax, (%rcx)
141ec07e0:     	movq	(%rbx), %rbx
141ec07e3:     	movq	0x30(%rbx), %r12
141ec07e7:     	movq	0x40(%rbx), %rsi
141ec07eb:     	cmpq	0x20(%rbx), %r12
141ec07ef:     	jne	0x141ec0801 <.text+0x1ebf801>
141ec07f1:     	leaq	0x20(%rbx), %rcx
141ec07f5:     	leaq	0xcf4a74(%rip), %rdx    # 0x142bb5270
141ec07fc:     	callq	0x140562630 <.text+0x561630>
141ec0801:     	movq	0x118(%rbp), %r13
141ec0808:     	movq	0x28(%rbx), %rax
141ec080c:     	movq	%r12, %rcx
141ec080f:     	shlq	$0x4, %rcx
141ec0813:     	movq	%rsi, (%rax,%rcx)
141ec0817:     	movb	$0x2, 0x8(%rax,%rcx)
141ec081c:     	incq	%r12
141ec081f:     	movq	%r12, 0x30(%rbx)
141ec0823:     	incq	0x40(%rbx)
141ec0827:     	testq	%r15, %r15
141ec082a:     	je	0x141ec0941 <.text+0x1ebf941>
141ec0830:     	decq	%r15
141ec0833:     	leaq	(%r15,%r15,4), %rax
141ec0837:     	shlq	$0x3, %rax
141ec083b:     	addq	0x8(%r14), %rax
141ec083f:     	xorl	%ebx, %ebx
141ec0841:     	cmpq	0x10(%r14), %r15
141ec0845:     	cmovbq	%rax, %rbx
141ec0849:     	jae	0x141ec09ff <.text+0x1ebf9ff>
141ec084f:     	movl	0x1c(%r13), %eax
141ec0853:     	movq	0x10(%r13), %rcx
141ec0857:     	movl	%eax, (%rcx)
141ec0859:     	movq	(%r13), %rdx
141ec085d:     	leaq	0x10(%rbp), %rcx
141ec0861:     	callq	0x1408aea60 <.text+0x8ada60>
141ec0866:     	movq	0x10(%rbp), %r12
141ec086a:     	movq	0x18(%rbp), %r15
141ec086e:     	movq	0x20(%rbp), %r13
141ec0872:     	nopw	%cs:(%rax,%rax)
141ec0880:     	cmpq	%r15, %r12
141ec0883:     	je	0x141ec09ff <.text+0x1ebf9ff>
141ec0889:     	movq	%r12, %r14
141ec088c:     	addq	$0x40, %r12
141ec0890:     	testq	%r13, %r13
141ec0893:     	sete	%al
141ec0896:     	incq	%r13
141ec0899:     	testq	%r14, %r14
141ec089c:     	sete	%cl
141ec089f:     	orb	%al, %cl
141ec08a1:     	jne	0x141ec0880 <.text+0x1ebf880>
141ec08a3:     	movq	%r14, %rcx
141ec08a6:     	movq	%rbx, %rdx
141ec08a9:     	callq	0x140d7be20 <.text+0xd7ae20>
141ec08ae:     	movl	$0x3f800000, 0x18(%r14) # imm = 0x3F800000
141ec08b6:     	jmp	0x141ec0880 <.text+0x1ebf880>
141ec08b8:     	movq	$0x0, (%r13)
141ec08c0:     	testb	%dil, %dil
141ec08c3:     	je	0x141ec090c <.text+0x1ebf90c>
141ec08c5:     	callq	0x140c18130 <.text+0xc17130>
141ec08ca:     	movq	%rax, %rcx
141ec08cd:     	movq	(%rax), %rax
141ec08d0:     	cmpq	$0x1, %rax
141ec08d4:     	jne	0x141ec0b20 <.text+0x1ebfb20>
141ec08da:     	addq	$0x8, %rcx
141ec08de:     	cmpq	$0x0, (%rcx)
141ec08e2:     	jne	0x141ec0b5a <.text+0x1ebfb5a>
141ec08e8:     	movq	$-0x1, (%rcx)
141ec08ef:     	movq	%rcx, 0x98(%rbp)
141ec08f6:     	addq	$0x8, %rcx
141ec08fa:     	movq	%rsi, %rdx
141ec08fd:     	callq	0x140c170f0 <.text+0xc160f0>
141ec0902:     	movq	0x98(%rbp), %rax
141ec0909:     	incq	(%rax)
141ec090c:     	addq	$0x128, %rsp            # imm = 0x128
141ec0913:     	popq	%rbx
141ec0914:     	popq	%rdi
141ec0915:     	popq	%rsi
141ec0916:     	popq	%r12
141ec0918:     	popq	%r13
141ec091a:     	popq	%r14
141ec091c:     	popq	%r15
141ec091e:     	popq	%rbp
141ec091f:     	retq
141ec0920:     	testq	%rax, %rax
141ec0923:     	jne	0x141ec0b4e <.text+0x1ebfb4e>
141ec0929:     	xorl	%edx, %edx
141ec092b:     	callq	0x1428e6250 <.text+0x28e5250>
141ec0930:     	movq	%rax, %rcx
141ec0933:     	testq	%rax, %rax
141ec0936:     	jne	0x141ec049a <.text+0x1ebf49a>
141ec093c:     	jmp	0x141ec0b4e <.text+0x1ebfb4e>
141ec0941:     	movq	(%r13), %rbx
141ec0945:     	movq	%rbx, %rcx
141ec0948:     	xorl	%edx, %edx
141ec094a:     	callq	0x1408aea80 <.text+0x8ada80>
141ec094f:     	movq	%rax, %r14
141ec0952:     	movb	0x28(%rax), %sil
141ec0956:     	leaq	0x10(%rbp), %rcx
141ec095a:     	movq	%rax, %rdx
141ec095d:     	callq	0x140d7bd90 <.text+0xd7ad90>
141ec0962:     	movb	%sil, 0x38(%rbp)
141ec0966:     	movups	0x18(%r14), %xmm0
141ec096b:     	movups	%xmm0, 0x28(%rbp)
141ec096f:     	movl	0x1c(%r13), %eax
141ec0973:     	movq	0x10(%r13), %rcx
141ec0977:     	movl	%eax, (%rcx)
141ec0979:     	leaq	0x80(%rbp), %rcx
141ec0980:     	movq	%rbx, %rdx
141ec0983:     	callq	0x1408aea60 <.text+0x8ada60>
141ec0988:     	movq	0x80(%rbp), %r12
141ec098f:     	movq	0x88(%rbp), %r15
141ec0996:     	movq	0x90(%rbp), %r13
141ec099d:     	leaq	0x10(%rbp), %rbx
141ec09a1:     	nopw	%cs:(%rax,%rax)
141ec09b0:     	cmpq	%r15, %r12
141ec09b3:     	je	0x141ec09e7 <.text+0x1ebf9e7>
141ec09b5:     	movq	%r12, %r14
141ec09b8:     	addq	$0x40, %r12
141ec09bc:     	testq	%r13, %r13
141ec09bf:     	sete	%al
141ec09c2:     	incq	%r13
141ec09c5:     	testq	%r14, %r14
141ec09c8:     	sete	%cl
141ec09cb:     	orb	%al, %cl
141ec09cd:     	jne	0x141ec09b0 <.text+0x1ebf9b0>
141ec09cf:     	movq	%r14, %rcx
141ec09d2:     	movq	%rbx, %rdx
141ec09d5:     	callq	0x140d7be20 <.text+0xd7ae20>
141ec09da:     	movss	0x28(%rbp), %xmm0
141ec09df:     	movss	%xmm0, 0x18(%r14)
141ec09e5:     	jmp	0x141ec09b0 <.text+0x1ebf9b0>
141ec09e7:     	movq	0x10(%rbp), %rdx
141ec09eb:     	testq	%rdx, %rdx
141ec09ee:     	je	0x141ec09ff <.text+0x1ebf9ff>
141ec09f0:     	movq	0x18(%rbp), %rcx
141ec09f4:     	movl	$0x1, %r8d
141ec09fa:     	callq	0x140613c20 <.text+0x612c20>
141ec09ff:     	movl	0x98(%rbp), %eax
141ec0a05:     	movq	0x78(%rbp), %rcx
141ec0a09:     	movl	%eax, (%rcx)
141ec0a0b:     	leaq	0x10daaf8(%rip), %rax   # 0x142f9b50a
141ec0a12:     	movq	%rax, 0x10(%rbp)
141ec0a16:     	movq	$0x4, 0x18(%rbp)
141ec0a1e:     	leaq	0x10(%rbp), %rcx
141ec0a22:     	callq	0x1405bdda0 <.text+0x5bcda0>
141ec0a27:     	movq	%rax, %rbx
141ec0a2a:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141ec0a34:     	xorq	%rax, %rbx
141ec0a37:     	movq	%rdi, %rcx
141ec0a3a:     	movq	%rbx, %rdx
141ec0a3d:     	callq	0x140683ec0 <.text+0x682ec0>
141ec0a42:     	movb	$0x0, 0x5d(%rax)
141ec0a46:     	testb	$0x1, %dl
141ec0a49:     	jne	0x141ec0a7e <.text+0x1ebfa7e>
141ec0a4b:     	movq	0x10(%rdi), %r14
141ec0a4f:     	cmpq	(%rdi), %r14
141ec0a52:     	jne	0x141ec0a63 <.text+0x1ebfa63>
141ec0a54:     	leaq	0xcee56d(%rip), %rdx    # 0x142baefc8
141ec0a5b:     	movq	%rdi, %rcx
141ec0a5e:     	callq	0x1406319d0 <.text+0x6309d0>
141ec0a63:     	movq	0x8(%rdi), %rax
141ec0a67:     	imulq	$0x38, %r14, %rcx
141ec0a6b:     	movl	$0x2, (%rax,%rcx)
141ec0a72:     	movq	%rbx, 0x30(%rax,%rcx)
141ec0a77:     	incq	%r14
141ec0a7a:     	movq	%r14, 0x10(%rdi)
141ec0a7e:     	movq	%rdi, 0x80(%rbp)
141ec0a85:     	movq	%rbx, 0x88(%rbp)
141ec0a8c:     	movq	$0x0, 0x90(%rbp)
141ec0a97:     	movq	$0x0, 0x20(%rsp)
141ec0aa0:     	leaq	0x10daa67(%rip), %r8    # 0x142f9b50e
141ec0aa7:     	leaq	0x10(%rbp), %rcx
141ec0aab:     	leaq	0x80(%rbp), %rdx
141ec0ab2:     	movl	$0x9, %r9d
141ec0ab8:     	callq	0x140f9e970 <.text+0xf9d970>
141ec0abd:     	leaq	0x10daa53(%rip), %rdx   # 0x142f9b517
141ec0ac4:     	leaq	0x10(%rbp), %rcx
141ec0ac8:     	movl	$0x6, %r8d
141ec0ace:     	movq	0x110(%rbp), %r9
141ec0ad5:     	callq	0x140fba410 <.text+0xfb9410>
141ec0ada:     	cmpq	$0x0, -0x8(%rbp)
141ec0adf:     	je	0x141ec090c <.text+0x1ebf90c>
141ec0ae5:     	callq	0x140c18130 <.text+0xc17130>
141ec0aea:     	movq	%rax, %rcx
141ec0aed:     	movq	(%rax), %rax
141ec0af0:     	cmpq	$0x1, %rax
141ec0af4:     	jne	0x141ec0b3a <.text+0x1ebfb3a>
141ec0af6:     	addq	$0x8, %rcx
141ec0afa:     	cmpq	$0x0, (%rcx)
141ec0afe:     	jne	0x141ec0b5a <.text+0x1ebfb5a>
141ec0b00:     	movq	$-0x1, (%rcx)
141ec0b07:     	movq	%rcx, 0x98(%rbp)
141ec0b0e:     	addq	$0x8, %rcx
141ec0b12:     	movq	(%rbp), %rdx
141ec0b16:     	callq	0x140c170f0 <.text+0xc160f0>
141ec0b1b:     	jmp	0x141ec0902 <.text+0x1ebf902>
141ec0b20:     	testq	%rax, %rax
141ec0b23:     	jne	0x141ec0b4e <.text+0x1ebfb4e>
141ec0b25:     	xorl	%edx, %edx
141ec0b27:     	callq	0x1428e6250 <.text+0x28e5250>
141ec0b2c:     	movq	%rax, %rcx
141ec0b2f:     	testq	%rax, %rax
141ec0b32:     	jne	0x141ec08de <.text+0x1ebf8de>
141ec0b38:     	jmp	0x141ec0b4e <.text+0x1ebfb4e>
141ec0b3a:     	testq	%rax, %rax
141ec0b3d:     	jne	0x141ec0b4e <.text+0x1ebfb4e>
141ec0b3f:     	xorl	%edx, %edx
141ec0b41:     	callq	0x1428e6250 <.text+0x28e5250>
141ec0b46:     	movq	%rax, %rcx
141ec0b49:     	testq	%rax, %rax
141ec0b4c:     	jne	0x141ec0afa <.text+0x1ebfafa>
141ec0b4e:     	leaq	0xcf4783(%rip), %rcx    # 0x142bb52d8
141ec0b55:     	callq	0x1428d80d0 <.text+0x28d70d0>
141ec0b5a:     	leaq	0xcf47e7(%rip), %rcx    # 0x142bb5348
141ec0b61:     	callq	0x1428d9250 <.text+0x28d8250>
141ec0b66:     	ud2
141ec0b68:     	nopl	(%rax,%rax)
141ec0b70:     	movq	%rdx, 0x10(%rsp)
141ec0b75:     	pushq	%rbp
141ec0b76:     	pushq	%r15
141ec0b78:     	pushq	%r14
141ec0b7a:     	pushq	%r13
141ec0b7c:     	pushq	%r12
141ec0b7e:     	pushq	%rsi
141ec0b7f:     	pushq	%rdi
141ec0b80:     	pushq	%rbx
141ec0b81:     	subq	$0x38, %rsp
141ec0b85:     	leaq	0x80(%rdx), %rbp
141ec0b8c:     	movq	0x8(%rbp), %rax
141ec0b90:     	incq	(%rax)
141ec0b93:     	addq	$0x38, %rsp
141ec0b97:     	popq	%rbx
141ec0b98:     	popq	%rdi
141ec0b99:     	popq	%rsi
141ec0b9a:     	popq	%r12
141ec0b9c:     	popq	%r13
141ec0b9e:     	popq	%r14
141ec0ba0:     	popq	%r15
141ec0ba2:     	popq	%rbp
141ec0ba3:     	retq
141ec0ba4:     	nopw	%cs:(%rax,%rax)
141ec0bb0:     	movq	%rdx, 0x10(%rsp)
141ec0bb5:     	pushq	%rbp
141ec0bb6:     	pushq	%r15
141ec0bb8:     	pushq	%r14
141ec0bba:     	pushq	%r13
141ec0bbc:     	pushq	%r12
141ec0bbe:     	pushq	%rsi
141ec0bbf:     	pushq	%rdi
141ec0bc0:     	pushq	%rbx
141ec0bc1:     	subq	$0x38, %rsp
141ec0bc5:     	leaq	0x80(%rdx), %rbp
141ec0bcc:     	cmpq	$0x0, -0x8(%rbp)
141ec0bd1:     	je	0x141ec0bf3 <.text+0x1ebfbf3>
141ec0bd3:     	leaq	(%rbp), %rax
