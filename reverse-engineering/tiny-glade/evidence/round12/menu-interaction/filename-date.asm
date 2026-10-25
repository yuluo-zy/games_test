
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c96690:     	pushq	%rbp
140c96691:     	pushq	%rsi
140c96692:     	subq	$0xc8, %rsp
140c96699:     	leaq	0x80(%rsp), %rbp
140c966a1:     	movq	$-0x2, 0x40(%rbp)
140c966a9:     	movq	%rcx, %rsi
140c966ac:     	movq	%rbp, %rcx
140c966af:     	callq	0x140cafc00 <.text+0xcaec00>
140c966b4:     	movl	0x29a28ed(%rip), %r8d   # 0x143638fa8
140c966bb:     	testb	$0x1, %r8b
140c966bf:     	je	0x140c966dc <.text+0xc956dc>
140c966c1:     	shrl	$0x8, %r8d
140c966c5:     	movups	(%rbp), %xmm0
140c966c9:     	movaps	%xmm0, -0x30(%rbp)
140c966cd:     	leaq	0x30(%rbp), %rcx
140c966d1:     	leaq	-0x30(%rbp), %rdx
140c966d5:     	callq	0x140cafc70 <.text+0xcaec70>
140c966da:     	jmp	0x140c966e4 <.text+0xc956e4>
140c966dc:     	movups	(%rbp), %xmm0
140c966e0:     	movaps	%xmm0, 0x30(%rbp)
140c966e4:     	leaq	0x1e97b3a(%rip), %rdx   # 0x142b2e225
140c966eb:     	movq	%rbp, %rcx
140c966ee:     	movl	$0x2d, %r8d
140c966f4:     	callq	0x140caf8e0 <.text+0xcae8e0>
140c966f9:     	cmpl	$0x7, (%rbp)
140c966fd:     	jne	0x140c9677c <.text+0xc9577c>
140c966ff:     	movq	0x18(%rbp), %rax
140c96703:     	movq	%rax, -0x40(%rbp)
140c96707:     	movups	0x8(%rbp), %xmm0
140c9670b:     	movaps	%xmm0, -0x50(%rbp)
140c9670f:     	movl	0x30(%rbp), %r8d
140c96713:     	movq	0x34(%rbp), %r9
140c96717:     	movzwl	0x3c(%rbp), %eax
140c9671b:     	movzbl	0x3e(%rbp), %ecx
140c9671f:     	shll	$0x18, %ecx
140c96722:     	shll	$0x8, %eax
140c96725:     	addl	%ecx, %eax
140c96727:     	incl	%eax
140c96729:     	movl	%eax, 0x20(%rsp)
140c9672d:     	leaq	-0x30(%rbp), %rcx
140c96731:     	leaq	-0x50(%rbp), %rdx
140c96735:     	callq	0x141941820 <.text+0x1940820>
140c9673a:     	cmpl	$0x1, -0x30(%rbp)
140c9673e:     	je	0x140c967be <.text+0xc957be>
140c96740:     	movq	-0x18(%rbp), %rax
140c96744:     	movq	%rax, 0x10(%rsi)
140c96748:     	movups	-0x28(%rbp), %xmm0
140c9674c:     	movups	%xmm0, (%rsi)
140c9674f:     	movq	-0x50(%rbp), %rax
140c96753:     	testq	%rax, %rax
140c96756:     	je	0x140c9676f <.text+0xc9576f>
140c96758:     	movq	-0x48(%rbp), %rcx
140c9675c:     	shlq	$0x3, %rax
140c96760:     	leaq	(%rax,%rax,2), %rdx
140c96764:     	movl	$0x8, %r8d
140c9676a:     	callq	0x140613c20 <.text+0x612c20>
140c9676f:     	movq	%rsi, %rax
140c96772:     	addq	$0xc8, %rsp
140c96779:     	popq	%rsi
140c9677a:     	popq	%rbp
140c9677b:     	retq
140c9677c:     	movups	(%rbp), %xmm0
140c96780:     	movups	0x10(%rbp), %xmm1
140c96784:     	movups	0x20(%rbp), %xmm2
140c96788:     	movaps	%xmm2, -0x10(%rbp)
140c9678c:     	movaps	%xmm1, -0x20(%rbp)
140c96790:     	movaps	%xmm0, -0x30(%rbp)
140c96794:     	leaq	0x1e97ad5(%rip), %rax   # 0x142b2e270
140c9679b:     	movq	%rax, 0x20(%rsp)
140c967a0:     	leaq	0x1e979b9(%rip), %rcx   # 0x142b2e160
140c967a7:     	leaq	0x1e97992(%rip), %r9    # 0x142b2e140
140c967ae:     	leaq	-0x30(%rbp), %r8
140c967b2:     	movl	$0x2b, %edx
140c967b7:     	callq	0x1428d9760 <.text+0x28d8760>
140c967bc:     	jmp	0x140c967f5 <.text+0xc957f5>
140c967be:     	movq	-0x18(%rbp), %rax
140c967c2:     	movq	%rax, 0x10(%rbp)
140c967c6:     	movups	-0x28(%rbp), %xmm0
140c967ca:     	movaps	%xmm0, (%rbp)
140c967ce:     	leaq	0x1e97ab3(%rip), %rax   # 0x142b2e288
140c967d5:     	movq	%rax, 0x20(%rsp)
140c967da:     	leaq	0x1e9797f(%rip), %rcx   # 0x142b2e160
140c967e1:     	leaq	0x1e979a8(%rip), %r9    # 0x142b2e190
140c967e8:     	movq	%rbp, %r8
140c967eb:     	movl	$0x2b, %edx
140c967f0:     	callq	0x1428d9760 <.text+0x28d8760>
140c967f5:     	ud2
140c967f7:     	nopw	(%rax,%rax)
140c96800:     	movq	%rdx, 0x10(%rsp)
140c96805:     	pushq	%rbp
140c96806:     	pushq	%rsi
140c96807:     	subq	$0x28, %rsp
140c9680b:     	leaq	0x80(%rdx), %rbp
140c96812:     	movq	-0x30(%rbp), %rax
140c96816:     	decq	%rax
140c96819:     	cmpq	$0x1, %rax
140c9681d:     	ja	0x140c96837 <.text+0xc95837>
140c9681f:     	movq	-0x20(%rbp), %rdx
140c96823:     	testq	%rdx, %rdx
140c96826:     	je	0x140c96837 <.text+0xc95837>
140c96828:     	movq	-0x18(%rbp), %rcx
140c9682c:     	movl	$0x1, %r8d
140c96832:     	callq	0x140613c20 <.text+0x612c20>
140c96837:     	nop
140c96838:     	addq	$0x28, %rsp
140c9683c:     	popq	%rsi
140c9683d:     	popq	%rbp
140c9683e:     	retq
140c9683f:     	nop
140c96840:     	movq	%rdx, 0x10(%rsp)
140c96845:     	pushq	%rbp
140c96846:     	pushq	%rsi
140c96847:     	subq	$0x28, %rsp
140c9684b:     	leaq	0x80(%rdx), %rbp
140c96852:     	movq	-0x50(%rbp), %rax
140c96856:     	testq	%rax, %rax
140c96859:     	je	0x140c96872 <.text+0xc95872>
140c9685b:     	movq	-0x48(%rbp), %rcx
140c9685f:     	shlq	$0x3, %rax
140c96863:     	leaq	(%rax,%rax,2), %rdx
140c96867:     	movl	$0x8, %r8d
140c9686d:     	callq	0x140613c20 <.text+0x612c20>
140c96872:     	nop
140c96873:     	addq	$0x28, %rsp
140c96877:     	popq	%rsi
140c96878:     	popq	%rbp
140c96879:     	retq
140c9687a:     	nopw	(%rax,%rax)
140c96880:     	movq	%rdx, 0x10(%rsp)
140c96885:     	pushq	%rbp
140c96886:     	pushq	%rsi
140c96887:     	subq	$0x28, %rsp
140c9688b:     	leaq	0x80(%rdx), %rbp
140c96892:     	cmpl	$0x2, (%rbp)
140c96896:     	jb	0x140c968a1 <.text+0xc958a1>
140c96898:     	leaq	0x8(%rbp), %rcx
140c9689c:     	callq	0x1405e7a00 <.text+0x5e6a00>
140c968a1:     	nop
140c968a2:     	addq	$0x28, %rsp
140c968a6:     	popq	%rsi
140c968a7:     	popq	%rbp
140c968a8:     	retq
