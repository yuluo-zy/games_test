
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409125e0:     	pushq	%rbp
1409125e1:     	pushq	%rsi
1409125e2:     	pushq	%rdi
1409125e3:     	subq	$0x3e0, %rsp            # imm = 0x3E0
1409125ea:     	leaq	0x80(%rsp), %rbp
1409125f2:     	movq	$-0x2, 0x358(%rbp)
1409125fd:     	movq	%rdx, %rsi
140912600:     	leaq	0x21994c1(%rip), %r8    # 0x142aabac8
140912607:     	leaq	0x80(%rbp), %rdi
14091260e:     	movq	%rdi, %rcx
140912611:     	movb	$0x9, %dl
140912613:     	callq	0x1416fe600 <.text+0x16fd600>
140912618:     	leaq	0x270(%rbp), %rcx
14091261f:     	movq	%rdi, %rdx
140912622:     	callq	0x14144dc80 <.text+0x144cc80>
140912627:     	movq	0x2d19d42(%rip), %rcx   # 0x14362c370
14091262e:     	leaq	0x238493b(%rip), %r8    # 0x142c96f70
140912635:     	leaq	0x200(%rbp), %rdx
14091263c:     	callq	0x140b218a0 <.text+0xb208a0>
140912641:     	movq	%rdx, %r8
140912644:     	leaq	0x270(%rbp), %rcx
14091264b:     	movq	%rax, %rdx
14091264e:     	callq	0x141452ef0 <.text+0x1451ef0>
140912653:     	movups	0x2d0(%rbp), %xmm0
14091265a:     	movaps	%xmm0, 0xe0(%rbp)
140912661:     	movups	0x2c0(%rbp), %xmm0
140912668:     	movaps	%xmm0, 0xd0(%rbp)
14091266f:     	movups	0x2b0(%rbp), %xmm0
140912676:     	movaps	%xmm0, 0xc0(%rbp)
14091267d:     	movups	0x270(%rbp), %xmm0
140912684:     	movups	0x280(%rbp), %xmm1
14091268b:     	movups	0x290(%rbp), %xmm2
140912692:     	movups	0x2a0(%rbp), %xmm3
140912699:     	movaps	%xmm3, 0xb0(%rbp)
1409126a0:     	movaps	%xmm2, 0xa0(%rbp)
1409126a7:     	movaps	%xmm1, 0x90(%rbp)
1409126ae:     	movaps	%xmm0, 0x80(%rbp)
1409126b5:     	leaq	0x80(%rbp), %rdx
1409126bc:     	movq	%rsi, %rcx
1409126bf:     	callq	0x140b67280 <.text+0xb66280>
1409126c4:     	leaq	0x10(%rbp), %rcx
1409126c8:     	callq	0x1416fe870 <.text+0x16fd870>
1409126cd:     	leaq	-0x60(%rbp), %rcx
1409126d1:     	callq	0x1416fea60 <.text+0x16fda60>
1409126d6:     	leaq	0x80(%rbp), %rcx
1409126dd:     	callq	0x141564e80 <.text+0x1563e80>
1409126e2:     	leaq	0x200(%rbp), %rcx
1409126e9:     	leaq	0x80(%rbp), %rdx
1409126f0:     	callq	0x1416fec40 <.text+0x16fdc40>
1409126f5:     	movups	-0x60(%rbp), %xmm0
1409126f9:     	movups	-0x50(%rbp), %xmm1
1409126fd:     	movups	-0x40(%rbp), %xmm2
140912701:     	movups	-0x30(%rbp), %xmm3
140912705:     	movaps	%xmm0, 0x270(%rbp)
14091270c:     	movaps	%xmm1, 0x280(%rbp)
140912713:     	movaps	%xmm2, 0x290(%rbp)
14091271a:     	movaps	%xmm3, 0x2a0(%rbp)
140912721:     	movups	-0x20(%rbp), %xmm0
140912725:     	movaps	%xmm0, 0x2b0(%rbp)
14091272c:     	movups	-0x10(%rbp), %xmm0
140912730:     	movaps	%xmm0, 0x2c0(%rbp)
140912737:     	movups	(%rbp), %xmm0
14091273b:     	movaps	%xmm0, 0x2d0(%rbp)
140912742:     	movups	0x200(%rbp), %xmm0
140912749:     	movups	0x210(%rbp), %xmm1
140912750:     	movups	0x220(%rbp), %xmm2
140912757:     	movups	0x230(%rbp), %xmm3
14091275e:     	movaps	%xmm0, 0x2e0(%rbp)
140912765:     	movaps	%xmm1, 0x2f0(%rbp)
14091276c:     	movaps	%xmm2, 0x300(%rbp)
140912773:     	movaps	%xmm3, 0x310(%rbp)
14091277a:     	movups	0x240(%rbp), %xmm0
140912781:     	movaps	%xmm0, 0x320(%rbp)
140912788:     	movups	0x250(%rbp), %xmm0
14091278f:     	movaps	%xmm0, 0x330(%rbp)
140912796:     	movups	0x260(%rbp), %xmm0
14091279d:     	movaps	%xmm0, 0x340(%rbp)
1409127a4:     	movups	0x70(%rbp), %xmm0
1409127a8:     	movaps	%xmm0, 0xe0(%rbp)
1409127af:     	movups	0x60(%rbp), %xmm0
1409127b3:     	movaps	%xmm0, 0xd0(%rbp)
1409127ba:     	movups	0x50(%rbp), %xmm0
1409127be:     	movaps	%xmm0, 0xc0(%rbp)
1409127c5:     	movups	0x10(%rbp), %xmm0
1409127c9:     	movups	0x20(%rbp), %xmm1
1409127cd:     	movups	0x30(%rbp), %xmm2
1409127d1:     	movups	0x40(%rbp), %xmm3
1409127d5:     	movaps	%xmm3, 0xb0(%rbp)
1409127dc:     	movaps	%xmm2, 0xa0(%rbp)
1409127e3:     	movaps	%xmm1, 0x90(%rbp)
1409127ea:     	movaps	%xmm0, 0x80(%rbp)
1409127f1:     	leaq	0xf0(%rbp), %rcx
1409127f8:     	leaq	0x270(%rbp), %rdx
1409127ff:     	movl	$0xe0, %r8d
140912805:     	callq	0x142923c00 <.text+0x2922c00>
14091280a:     	leaq	0x270(%rbp), %rcx
140912811:     	leaq	0x80(%rbp), %rdx
140912818:     	callq	0x141450ae0 <.text+0x144fae0>
14091281d:     	movb	$0x9, 0x200(%rbp)
140912824:     	movq	0x2d19b45(%rip), %rcx   # 0x14362c370
14091282b:     	leaq	0x234d52e(%rip), %r8    # 0x142c5fd60
140912832:     	leaq	0x200(%rbp), %rdx
140912839:     	callq	0x140b218a0 <.text+0xb208a0>
14091283e:     	movq	%rdx, %r8
140912841:     	leaq	0x270(%rbp), %rcx
140912848:     	movq	%rax, %rdx
14091284b:     	callq	0x141453000 <.text+0x1452000>
140912850:     	movups	0x2d0(%rbp), %xmm0
140912857:     	movaps	%xmm0, 0xe0(%rbp)
14091285e:     	movups	0x2c0(%rbp), %xmm0
140912865:     	movaps	%xmm0, 0xd0(%rbp)
14091286c:     	movups	0x2b0(%rbp), %xmm0
140912873:     	movaps	%xmm0, 0xc0(%rbp)
14091287a:     	movups	0x270(%rbp), %xmm0
140912881:     	movups	0x280(%rbp), %xmm1
140912888:     	movups	0x290(%rbp), %xmm2
14091288f:     	movups	0x2a0(%rbp), %xmm3
140912896:     	movaps	%xmm3, 0xb0(%rbp)
14091289d:     	movaps	%xmm2, 0xa0(%rbp)
1409128a4:     	movaps	%xmm1, 0x90(%rbp)
1409128ab:     	movaps	%xmm0, 0x80(%rbp)
1409128b2:     	leaq	0x80(%rbp), %rdx
1409128b9:     	movq	%rsi, %rcx
1409128bc:     	callq	0x140b67280 <.text+0xb66280>
1409128c1:     	movq	$0x0, 0x80(%rbp)
1409128cc:     	movq	$0x4, 0x88(%rbp)
1409128d7:     	movq	$0x0, 0x90(%rbp)
1409128e2:     	movq	$0x0, 0xa0(%rbp)
1409128ed:     	movl	$0x0, 0x98(%rbp)
1409128f7:     	leaq	0x80(%rbp), %rdx
1409128fe:     	movq	%rsi, %rcx
140912901:     	callq	0x141545d70 <.text+0x1544d70>
140912906:     	movq	%rax, %rsi
140912909:     	leaq	0x38(%rax), %rcx
14091290d:     	movabsq	$0x80c70da6101ec1a, %rdx # imm = 0x80C70DA6101EC1A
140912917:     	movabsq	$0x668ae4faf3605349, %r8 # imm = 0x668AE4FAF3605349
140912921:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140912926:     	testq	%rax, %rax
140912929:     	je	0x140912969 <.text+0x911969>
14091292b:     	cmpq	%rdx, 0x218(%rsi)
140912932:     	jbe	0x140912969 <.text+0x911969>
140912934:     	movq	0x210(%rsi), %rax
14091293b:     	movq	(%rax,%rdx,8), %rax
14091293f:     	testq	%rax, %rax
140912942:     	je	0x140912969 <.text+0x911969>
140912944:     	movq	0x1e0(%rsi), %rcx
14091294b:     	notq	%rax
14091294e:     	leaq	(%rax,%rax,2), %rax
140912952:     	shlq	$0x5, %rax
140912956:     	cmpq	$0x0, 0x20(%rcx,%rax)
14091295c:     	je	0x140912969 <.text+0x911969>
14091295e:     	addq	$0x3e0, %rsp            # imm = 0x3E0
140912965:     	popq	%rdi
140912966:     	popq	%rsi
140912967:     	popq	%rbp
140912968:     	retq
140912969:     	movq	%rsi, %rcx
14091296c:     	addq	$0x3e0, %rsp            # imm = 0x3E0
140912973:     	popq	%rdi
140912974:     	popq	%rsi
140912975:     	popq	%rbp
140912976:     	jmp	0x14151f980 <.text+0x151e980>
14091297b:     	nopl	(%rax,%rax)
140912980:     	movq	%rdx, 0x10(%rsp)
140912985:     	pushq	%rbp
140912986:     	pushq	%rsi
140912987:     	pushq	%rdi
140912988:     	subq	$0x20, %rsp
14091298c:     	leaq	0x80(%rdx), %rbp
140912993:     	leaq	0x270(%rbp), %rcx
14091299a:     	callq	0x140029880 <.text+0x28880>
14091299f:     	nop
1409129a0:     	addq	$0x20, %rsp
1409129a4:     	popq	%rdi
1409129a5:     	popq	%rsi
1409129a6:     	popq	%rbp
1409129a7:     	retq
1409129a8:     	nopl	(%rax,%rax)
1409129b0:     	movq	%rdx, 0x10(%rsp)
1409129b5:     	pushq	%rbp
1409129b6:     	pushq	%rsi
1409129b7:     	pushq	%rdi
1409129b8:     	subq	$0x20, %rsp
1409129bc:     	leaq	0x80(%rdx), %rbp
1409129c3:     	leaq	0x10(%rbp), %rcx
1409129c7:     	callq	0x140029880 <.text+0x28880>
1409129cc:     	nop
1409129cd:     	addq	$0x20, %rsp
1409129d1:     	popq	%rdi
1409129d2:     	popq	%rsi
1409129d3:     	popq	%rbp
1409129d4:     	retq
1409129d5:     	nopw	%cs:(%rax,%rax)
1409129e0:     	movq	%rdx, 0x10(%rsp)
1409129e5:     	pushq	%rbp
1409129e6:     	pushq	%rsi
1409129e7:     	pushq	%rdi
1409129e8:     	subq	$0x20, %rsp
1409129ec:     	leaq	0x80(%rdx), %rbp
1409129f3:     	leaq	-0x60(%rbp), %rcx
1409129f7:     	callq	0x140029880 <.text+0x28880>
1409129fc:     	nop
1409129fd:     	addq	$0x20, %rsp
140912a01:     	popq	%rdi
140912a02:     	popq	%rsi
140912a03:     	popq	%rbp
140912a04:     	retq
140912a05:     	nopw	%cs:(%rax,%rax)
140912a10:     	movq	%rdx, 0x10(%rsp)
140912a15:     	pushq	%rbp
140912a16:     	pushq	%rsi
140912a17:     	pushq	%rdi
140912a18:     	subq	$0x20, %rsp
140912a1c:     	leaq	0x80(%rdx), %rbp
140912a23:     	leaq	0x270(%rbp), %rcx
140912a2a:     	callq	0x140029880 <.text+0x28880>
140912a2f:     	nop
140912a30:     	addq	$0x20, %rsp
140912a34:     	popq	%rdi
140912a35:     	popq	%rsi
140912a36:     	popq	%rbp
140912a37:     	retq
140912a38:     	int3
140912a39:     	int3
140912a3a:     	int3
140912a3b:     	int3
140912a3c:     	int3
140912a3d:     	int3
140912a3e:     	int3
140912a3f:     	int3
