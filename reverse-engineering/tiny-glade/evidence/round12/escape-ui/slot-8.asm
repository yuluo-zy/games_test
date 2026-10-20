
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404fe790:     	pushq	%r15
1404fe792:     	pushq	%r14
1404fe794:     	pushq	%r13
1404fe796:     	pushq	%r12
1404fe798:     	pushq	%rsi
1404fe799:     	pushq	%rdi
1404fe79a:     	pushq	%rbp
1404fe79b:     	pushq	%rbx
1404fe79c:     	subq	$0xd8, %rsp
1404fe7a3:     	movq	%r9, %rsi
1404fe7a6:     	movq	%rcx, %r14
1404fe7a9:     	movq	0x140(%rsp), %rbp
1404fe7b1:     	movq	%rdx, 0x28(%rsp)
1404fe7b6:     	movq	%r8, 0x30(%rsp)
1404fe7bb:     	leaq	0x10(%rcx), %rbx
1404fe7bf:     	movq	%rbx, %rcx
1404fe7c2:     	callq	0x1406868b0 <.text+0x6858b0>
1404fe7c7:     	movq	%rax, %r13
1404fe7ca:     	leaq	0x28(%rsp), %r12
1404fe7cf:     	movq	%r12, %rcx
1404fe7d2:     	callq	0x140683c10 <.text+0x682c10>
1404fe7d7:     	movq	%rax, 0x38(%rsp)
1404fe7dc:     	leaq	0x38(%rsp), %r15
1404fe7e1:     	movq	%r15, %rcx
1404fe7e4:     	callq	0x1400885d0 <.text+0x875d0>
1404fe7e9:     	movq	%rax, 0x58(%rsp)
1404fe7ee:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
1404fe7f8:     	movq	%rax, 0x60(%rsp)
1404fe7fd:     	movabsq	$-0x58648d975d569727, %rdi # imm = 0xA79B7268A2A968D9
1404fe807:     	movq	%rdi, 0x68(%rsp)
1404fe80c:     	leaq	0x58(%rsp), %rcx
1404fe811:     	callq	0x1405bdf20 <.text+0x5bcf20>
1404fe816:     	movq	%r13, 0x58(%rsp)
1404fe81b:     	movq	%rax, 0x60(%rsp)
1404fe820:     	leaq	0x58(%rsp), %rcx
1404fe825:     	callq	0x1405bd910 <.text+0x5bc910>
1404fe82a:     	movq	%rax, %r13
1404fe82d:     	movq	%rbx, %rcx
1404fe830:     	movq	%rax, %rdx
1404fe833:     	callq	0x140686450 <.text+0x685450>
1404fe838:     	testb	%al, %al
1404fe83a:     	je	0x1404fe854 <.text+0x4fd854>
1404fe83c:     	leaq	0x58(%rsp), %r14
1404fe841:     	movq	%r14, %rcx
1404fe844:     	movq	%rbx, %rdx
1404fe847:     	movq	%r13, %r8
1404fe84a:     	callq	0x140686530 <.text+0x685530>
1404fe84f:     	jmp	0x1404fe99e <.text+0x4fd99e>
1404fe854:     	movq	0x312db4d(%rip), %rax   # 0x14362c3a8
1404fe85b:     	movq	(%rax), %rax
1404fe85e:     	testq	%rax, %rax
1404fe861:     	je	0x1404fe93c <.text+0x4fd93c>
1404fe867:     	movq	%r14, 0x38(%rsp)
1404fe86c:     	leaq	0x3502d(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
1404fe873:     	movq	%rax, 0x40(%rsp)
1404fe878:     	movq	%r12, 0x48(%rsp)
1404fe87d:     	movq	%rax, 0x50(%rsp)
1404fe882:     	leaq	0x24ef4cf(%rip), %rcx   # 0x1429edd58
1404fe889:     	callq	0x14067dd80 <.text+0x67cd80>
1404fe88e:     	movups	(%rax), %xmm0
1404fe891:     	movl	0x10(%rax), %eax
1404fe894:     	movq	$0x1, 0x88(%rsp)
1404fe8a0:     	leaq	0x24ef481(%rip), %rcx   # 0x1429edd28
1404fe8a7:     	movq	%rcx, 0x90(%rsp)
1404fe8af:     	movq	$0x14, 0x98(%rsp)
1404fe8bb:     	leaq	0x24ef3d6(%rip), %rdx   # 0x1429edc98
1404fe8c2:     	movq	%rdx, 0xa8(%rsp)
1404fe8ca:     	movq	$0x2, 0xb0(%rsp)
1404fe8d6:     	movq	%r15, 0xb8(%rsp)
1404fe8de:     	movq	$0x2, 0xc0(%rsp)
1404fe8ea:     	movq	$0x0, 0xc8(%rsp)
1404fe8f6:     	movq	$0x0, 0x58(%rsp)
1404fe8ff:     	movq	%rcx, 0x60(%rsp)
1404fe904:     	movq	$0x14, 0x68(%rsp)
1404fe90d:     	movq	$0x0, 0x70(%rsp)
1404fe916:     	movups	%xmm0, 0x78(%rsp)
1404fe91b:     	movl	$0x1, 0xa0(%rsp)
1404fe926:     	movl	%eax, 0xa4(%rsp)
1404fe92d:     	leaq	0x27(%rsp), %rcx
1404fe932:     	leaq	0x58(%rsp), %rdx
1404fe937:     	callq	0x140d4a330 <.text+0xd49330>
1404fe93c:     	movups	0x28(%rsp), %xmm0
1404fe941:     	movq	0x18(%r14), %r14
1404fe945:     	movups	%xmm0, 0x38(%rsp)
1404fe94a:     	leaq	0x38(%rsp), %rcx
1404fe94f:     	callq	0x140683c10 <.text+0x682c10>
1404fe954:     	movq	%rax, 0x58(%rsp)
1404fe959:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
1404fe963:     	movq	%rax, 0x60(%rsp)
1404fe968:     	movq	%rdi, 0x68(%rsp)
1404fe96d:     	leaq	0x58(%rsp), %rcx
1404fe972:     	callq	0x1405bdf20 <.text+0x5bcf20>
1404fe977:     	movq	%r14, 0x58(%rsp)
1404fe97c:     	movq	%rax, 0x60(%rsp)
1404fe981:     	leaq	0x58(%rsp), %rcx
1404fe986:     	callq	0x1405bd910 <.text+0x5bc910>
1404fe98b:     	leaq	0x58(%rsp), %r14
1404fe990:     	movq	%r14, %rcx
1404fe993:     	movq	%rbx, %rdx
1404fe996:     	movq	%rax, %r8
1404fe999:     	callq	0x1406861b0 <.text+0x6851b0>
1404fe99e:     	movq	%rsi, %rcx
1404fe9a1:     	movq	%rbp, %rdx
1404fe9a4:     	movq	%r14, %r8
1404fe9a7:     	callq	0x140504b40 <.text+0x503b40>
1404fe9ac:     	nop
1404fe9ad:     	addq	$0xd8, %rsp
1404fe9b4:     	popq	%rbx
1404fe9b5:     	popq	%rbp
1404fe9b6:     	popq	%rdi
1404fe9b7:     	popq	%rsi
1404fe9b8:     	popq	%r12
1404fe9ba:     	popq	%r13
1404fe9bc:     	popq	%r14
1404fe9be:     	popq	%r15
1404fe9c0:     	retq
