
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d07f0:     	pushq	%rbp
1403d07f1:     	pushq	%r14
1403d07f3:     	pushq	%rsi
1403d07f4:     	pushq	%rdi
1403d07f5:     	pushq	%rbx
1403d07f6:     	subq	$0xb0, %rsp
1403d07fd:     	leaq	0x80(%rsp), %rbp
1403d0805:     	movaps	%xmm7, 0x20(%rbp)
1403d0809:     	movaps	%xmm6, 0x10(%rbp)
1403d080d:     	movq	$-0x2, 0x8(%rbp)
1403d0815:     	movq	%r8, %rsi
1403d0818:     	movaps	%xmm1, %xmm6
1403d081b:     	movq	%rcx, %rbx
1403d081e:     	movss	0x26010a6(%rip), %xmm0  # 0x1429d18cc
1403d0826:     	mulss	%xmm1, %xmm0
1403d082a:     	movss	%xmm0, 0x30(%rsp)
1403d0830:     	leaq	0x26013e3(%rip), %rax   # 0x1429d1c1a
1403d0837:     	movq	%rax, 0x20(%rsp)
1403d083c:     	movq	$0x11, 0x28(%rsp)
1403d0845:     	leaq	0x26013c0(%rip), %r8    # 0x1429d1c0c
1403d084c:     	leaq	-0x10(%rbp), %rcx
1403d0850:     	movl	$0xe, %r9d
1403d0856:     	movq	%rbx, %rdx
1403d0859:     	callq	0x14032e0b0 <.text+0x32d0b0>
1403d085e:     	leaq	0x26013c6(%rip), %rcx   # 0x1429d1c2b
1403d0865:     	movl	$0xd, %edx
1403d086a:     	callq	0x140b9d480 <.text+0xb9c480>
1403d086f:     	movq	%rdx, %r8
1403d0872:     	leaq	-0x28(%rbp), %rcx
1403d0876:     	movq	%rax, %rdx
1403d0879:     	callq	0x140d7ad40 <.text+0xd79d40>
1403d087e:     	movq	-0x18(%rbp), %r14
1403d0882:     	cvtsi2ss	%r14, %xmm2
1403d0887:     	movss	0x2563d19(%rip), %xmm0  # 0x1429345a8
1403d088f:     	movss	0x25fc79d(%rip), %xmm1  # 0x1429cd034
1403d0897:     	callq	0x140c904b0 <.text+0xc8f4b0>
1403d089c:     	movaps	%xmm0, %xmm2
1403d089f:     	movss	0x25cfee1(%rip), %xmm0  # 0x1429a0788
1403d08a7:     	movss	0x25fc785(%rip), %xmm1  # 0x1429cd034
1403d08af:     	callq	0x140c904d0 <.text+0xc8f4d0>
1403d08b4:     	movaps	%xmm0, %xmm7
1403d08b7:     	movq	-0x20(%rbp), %rdi
1403d08bb:     	leaq	-0x10(%rbp), %rcx
1403d08bf:     	movq	%rbx, %rdx
1403d08c2:     	movq	%rdi, %r8
1403d08c5:     	movq	%r14, %r9
1403d08c8:     	callq	0x14068bfc0 <.text+0x68afc0>
1403d08cd:     	leaq	-0x10(%rbp), %rcx
1403d08d1:     	callq	0x140681d40 <.text+0x680d40>
1403d08d6:     	mulss	%xmm6, %xmm7
1403d08da:     	movq	%rax, %rcx
1403d08dd:     	movaps	%xmm7, %xmm1
1403d08e0:     	callq	0x140106350 <.text+0x105350>
1403d08e5:     	movq	(%rbp), %rax
1403d08e9:     	movq	%rax, -0x30(%rbp)
1403d08ed:     	movups	-0x10(%rbp), %xmm0
1403d08f1:     	movaps	%xmm0, -0x40(%rbp)
1403d08f5:     	mulss	0x2600fd3(%rip), %xmm6  # 0x1429d18d0
1403d08fd:     	leaq	-0x10(%rbp), %rcx
1403d0901:     	leaq	-0x40(%rbp), %rdx
1403d0905:     	movaps	%xmm6, %xmm2
1403d0908:     	callq	0x14067e910 <.text+0x67d910>
1403d090d:     	leaq	-0x40(%rbp), %rcx
1403d0911:     	leaq	-0x10(%rbp), %rdx
1403d0915:     	movb	$0x2, %r8b
1403d0918:     	callq	0x14067e960 <.text+0x67d960>
1403d091d:     	leaq	-0x40(%rbp), %rcx
1403d0921:     	callq	0x140681d50 <.text+0x680d50>
1403d0926:     	movq	%rax, %rcx
1403d0929:     	callq	0x140683c80 <.text+0x682c80>
1403d092e:     	testb	%al, %al
1403d0930:     	je	0x1403d0947 <.text+0x3cf947>
1403d0932:     	leaq	0x26012ff(%rip), %rdx   # 0x1429d1c38
1403d0939:     	movl	$0x36, %r8d
1403d093f:     	movq	%rsi, %rcx
1403d0942:     	callq	0x14026d3c0 <.text+0x26c3c0>
1403d0947:     	movq	-0x28(%rbp), %rdx
1403d094b:     	testq	%rdx, %rdx
1403d094e:     	je	0x1403d095e <.text+0x3cf95e>
1403d0950:     	movl	$0x1, %r8d
1403d0956:     	movq	%rdi, %rcx
1403d0959:     	callq	0x140613c20 <.text+0x612c20>
1403d095e:     	movaps	0x10(%rbp), %xmm6
1403d0962:     	movaps	0x20(%rbp), %xmm7
1403d0966:     	addq	$0xb0, %rsp
1403d096d:     	popq	%rbx
1403d096e:     	popq	%rdi
1403d096f:     	popq	%rsi
1403d0970:     	popq	%r14
1403d0972:     	popq	%rbp
1403d0973:     	retq
1403d0974:     	nopw	%cs:(%rax,%rax)
1403d0980:     	movq	%rdx, 0x10(%rsp)
1403d0985:     	pushq	%rbp
1403d0986:     	pushq	%r14
1403d0988:     	pushq	%rsi
1403d0989:     	pushq	%rdi
1403d098a:     	pushq	%rbx
1403d098b:     	subq	$0x60, %rsp
1403d098f:     	leaq	0x80(%rdx), %rbp
1403d0996:     	movaps	%xmm7, 0x30(%rsp)
1403d099b:     	movaps	%xmm6, 0x40(%rsp)
1403d09a0:     	movq	-0x28(%rbp), %rdx
1403d09a4:     	testq	%rdx, %rdx
1403d09a7:     	je	0x1403d09b8 <.text+0x3cf9b8>
1403d09a9:     	movq	-0x20(%rbp), %rcx
1403d09ad:     	movl	$0x1, %r8d
1403d09b3:     	callq	0x140613c20 <.text+0x612c20>
1403d09b8:     	movaps	0x40(%rsp), %xmm6
1403d09bd:     	movaps	0x30(%rsp), %xmm7
1403d09c2:     	addq	$0x60, %rsp
1403d09c6:     	popq	%rbx
1403d09c7:     	popq	%rdi
1403d09c8:     	popq	%rsi
1403d09c9:     	popq	%r14
1403d09cb:     	popq	%rbp
1403d09cc:     	retq
