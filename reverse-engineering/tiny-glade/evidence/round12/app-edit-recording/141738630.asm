
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141738630:     	pushq	%rbp
141738631:     	pushq	%rsi
141738632:     	pushq	%rdi
141738633:     	pushq	%rbx
141738634:     	subq	$0x208, %rsp            # imm = 0x208
14173863b:     	leaq	0x80(%rsp), %rbp
141738643:     	movq	$-0x2, 0x180(%rbp)
14173864e:     	movl	%edx, %ebx
141738650:     	movq	%rcx, %rsi
141738653:     	xorps	%xmm0, %xmm0
141738656:     	movups	%xmm0, 0x60(%rbp)
14173865a:     	movups	%xmm0, 0x78(%rbp)
14173865e:     	movups	%xmm0, 0x90(%rbp)
141738665:     	movups	%xmm0, 0xa8(%rbp)
14173866c:     	movups	%xmm0, 0xc0(%rbp)
141738673:     	movl	$0x0, 0xd0(%rbp)
14173867d:     	movups	%xmm0, 0xf8(%rbp)
141738684:     	movups	%xmm0, 0x110(%rbp)
14173868b:     	movups	%xmm0, 0x128(%rbp)
141738692:     	movups	%xmm0, 0x140(%rbp)
141738699:     	movups	%xmm0, 0x158(%rbp)
1417386a0:     	movl	$0x0, 0x168(%rbp)
1417386aa:     	movq	$0x0, 0x18(%rbp)
1417386b2:     	movq	$0x0, 0x40(%rbp)
1417386ba:     	movq	$0x8, 0x48(%rbp)
1417386c2:     	movq	$0x0, 0x50(%rbp)
1417386ca:     	movq	$0x10, 0x58(%rbp)
1417386d2:     	movq	$0x10, 0x70(%rbp)
1417386da:     	movq	$0x10, 0x88(%rbp)
1417386e5:     	movq	$0x10, 0xa0(%rbp)
1417386f0:     	movq	$0x10, 0xb8(%rbp)
1417386fb:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141738705:     	movq	%rax, 0xd8(%rbp)
14173870c:     	leaq	0x1594dfe(%rip), %rax   # 0x142ccd511
141738713:     	movq	%rax, 0xe0(%rbp)
14173871a:     	movq	$0x4a, 0xe8(%rbp)
141738725:     	movq	$0x10, 0xf0(%rbp)
141738730:     	movq	$0x10, 0x108(%rbp)
14173873b:     	movq	$0x10, 0x120(%rbp)
141738746:     	movq	$0x10, 0x138(%rbp)
141738751:     	movq	$0x10, 0x150(%rbp)
14173875c:     	movl	$0x0, 0x170(%rbp)
141738766:     	movw	$0x1, 0x174(%rbp)
14173876f:     	movl	$0x0, 0x178(%rbp)
141738779:     	movzbl	0x1efc489(%rip), %eax   # 0x143634c09
141738780:     	movl	$0x168, %ecx            # imm = 0x168
141738785:     	movl	$0x8, %edx
14173878a:     	callq	0x140613c10 <.text+0x612c10>
14173878f:     	testq	%rax, %rax
141738792:     	je	0x14173882f <.text+0x173782f>
141738798:     	movq	%rax, %rdi
14173879b:     	leaq	0x18(%rbp), %rdx
14173879f:     	movl	$0x168, %r8d            # imm = 0x168
1417387a5:     	movq	%rax, %rcx
1417387a8:     	callq	0x142923c00 <.text+0x2922c00>
1417387ad:     	leaq	0x1594b8c(%rip), %r8    # 0x142ccd340
1417387b4:     	leaq	-0x58(%rbp), %rcx
1417387b8:     	movq	%rdi, %rdx
1417387bb:     	callq	0x140ce4b10 <.text+0xce3b10>
1417387c0:     	movb	%bl, 0x18(%rbp)
1417387c3:     	movq	0x1ef3ba6(%rip), %rcx   # 0x14362c370
1417387ca:     	leaq	0x152758f(%rip), %r8    # 0x142c5fd60
1417387d1:     	leaq	0x18(%rbp), %rdx
1417387d5:     	callq	0x140b218a0 <.text+0xb208a0>
1417387da:     	movq	%rdx, %r8
1417387dd:     	leaq	-0x58(%rbp), %rcx
1417387e1:     	movq	%rax, %rdx
1417387e4:     	callq	0x141453000 <.text+0x1452000>
1417387e9:     	movups	0x8(%rbp), %xmm0
1417387ed:     	movups	%xmm0, 0x60(%rsi)
1417387f1:     	movups	-0x8(%rbp), %xmm0
1417387f5:     	movups	%xmm0, 0x50(%rsi)
1417387f9:     	movups	-0x18(%rbp), %xmm0
1417387fd:     	movups	%xmm0, 0x40(%rsi)
141738801:     	movups	-0x58(%rbp), %xmm0
141738805:     	movups	-0x48(%rbp), %xmm1
141738809:     	movups	-0x38(%rbp), %xmm2
14173880d:     	movups	-0x28(%rbp), %xmm3
141738811:     	movups	%xmm3, 0x30(%rsi)
141738815:     	movups	%xmm2, 0x20(%rsi)
141738819:     	movups	%xmm1, 0x10(%rsi)
14173881d:     	movups	%xmm0, (%rsi)
141738820:     	movq	%rsi, %rax
141738823:     	addq	$0x208, %rsp            # imm = 0x208
14173882a:     	popq	%rbx
14173882b:     	popq	%rdi
14173882c:     	popq	%rsi
14173882d:     	popq	%rbp
14173882e:     	retq
14173882f:     	movl	$0x8, %ecx
141738834:     	movl	$0x168, %edx            # imm = 0x168
141738839:     	callq	0x1428d9000 <.text+0x28d8000>
14173883e:     	ud2
