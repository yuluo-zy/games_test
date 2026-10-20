
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140503460:     	pushq	%r15
140503462:     	pushq	%r14
140503464:     	pushq	%r13
140503466:     	pushq	%r12
140503468:     	pushq	%rsi
140503469:     	pushq	%rdi
14050346a:     	pushq	%rbp
14050346b:     	pushq	%rbx
14050346c:     	subq	$0xf8, %rsp
140503473:     	movq	%r9, %rsi
140503476:     	movq	%rcx, %rbx
140503479:     	movq	%rdx, 0x68(%rsp)
14050347e:     	movq	%r8, 0x70(%rsp)
140503483:     	leaq	0x10(%rcx), %rdi
140503487:     	movq	%rdi, %rcx
14050348a:     	callq	0x1406868b0 <.text+0x6858b0>
14050348f:     	movq	%rax, %r12
140503492:     	leaq	0x68(%rsp), %r15
140503497:     	movq	%r15, %rcx
14050349a:     	callq	0x140683c10 <.text+0x682c10>
14050349f:     	movq	%rax, 0x48(%rsp)
1405034a4:     	leaq	0x48(%rsp), %r14
1405034a9:     	movq	%r14, %rcx
1405034ac:     	callq	0x1400885d0 <.text+0x875d0>
1405034b1:     	movq	%rax, 0x78(%rsp)
1405034b6:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
1405034c0:     	movq	%r13, 0x80(%rsp)
1405034c8:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
1405034d2:     	movq	%rbp, 0x88(%rsp)
1405034da:     	leaq	0x78(%rsp), %rcx
1405034df:     	callq	0x1405bdf20 <.text+0x5bcf20>
1405034e4:     	movq	%r12, 0x78(%rsp)
1405034e9:     	movq	%rax, 0x80(%rsp)
1405034f1:     	leaq	0x78(%rsp), %rcx
1405034f6:     	callq	0x1405bd910 <.text+0x5bc910>
1405034fb:     	movq	%rax, %r12
1405034fe:     	movq	%rdi, %rcx
140503501:     	movq	%rax, %rdx
140503504:     	callq	0x140686450 <.text+0x685450>
140503509:     	testb	%al, %al
14050350b:     	je	0x140503525 <.text+0x502525>
14050350d:     	leaq	0x48(%rsp), %rbx
140503512:     	movq	%rbx, %rcx
140503515:     	movq	%rdi, %rdx
140503518:     	movq	%r12, %r8
14050351b:     	callq	0x140686530 <.text+0x685530>
140503520:     	jmp	0x14050367a <.text+0x50267a>
140503525:     	movq	0x3128e7c(%rip), %rax   # 0x14362c3a8
14050352c:     	movq	(%rax), %rax
14050352f:     	testq	%rax, %rax
140503532:     	je	0x140503619 <.text+0x502619>
140503538:     	movq	%rbx, 0x48(%rsp)
14050353d:     	leaq	0x3035c(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140503544:     	movq	%rax, 0x50(%rsp)
140503549:     	movq	%r15, 0x58(%rsp)
14050354e:     	movq	%rax, 0x60(%rsp)
140503553:     	leaq	0x24ea7fe(%rip), %rcx   # 0x1429edd58
14050355a:     	callq	0x14067dd80 <.text+0x67cd80>
14050355f:     	movups	(%rax), %xmm0
140503562:     	movl	0x10(%rax), %eax
140503565:     	movq	$0x1, 0xa8(%rsp)
140503571:     	leaq	0x24ea7b0(%rip), %rcx   # 0x1429edd28
140503578:     	movq	%rcx, 0xb0(%rsp)
140503580:     	movq	$0x14, 0xb8(%rsp)
14050358c:     	leaq	0x24ea705(%rip), %rdx   # 0x1429edc98
140503593:     	movq	%rdx, 0xc8(%rsp)
14050359b:     	movq	$0x2, 0xd0(%rsp)
1405035a7:     	movq	%r14, 0xd8(%rsp)
1405035af:     	movq	$0x2, 0xe0(%rsp)
1405035bb:     	movq	$0x0, 0xe8(%rsp)
1405035c7:     	movq	$0x0, 0x78(%rsp)
1405035d0:     	movq	%rcx, 0x80(%rsp)
1405035d8:     	movq	$0x14, 0x88(%rsp)
1405035e4:     	movq	$0x0, 0x90(%rsp)
1405035f0:     	movups	%xmm0, 0x98(%rsp)
1405035f8:     	movl	$0x1, 0xc0(%rsp)
140503603:     	movl	%eax, 0xc4(%rsp)
14050360a:     	leaq	0x47(%rsp), %rcx
14050360f:     	leaq	0x78(%rsp), %rdx
140503614:     	callq	0x140d4a330 <.text+0xd49330>
140503619:     	movups	0x68(%rsp), %xmm0
14050361e:     	movq	0x18(%rbx), %rbx
140503622:     	movups	%xmm0, 0x48(%rsp)
140503627:     	leaq	0x48(%rsp), %rcx
14050362c:     	callq	0x140683c10 <.text+0x682c10>
140503631:     	movq	%rax, 0x78(%rsp)
140503636:     	movq	%r13, 0x80(%rsp)
14050363e:     	movq	%rbp, 0x88(%rsp)
140503646:     	leaq	0x78(%rsp), %rcx
14050364b:     	callq	0x1405bdf20 <.text+0x5bcf20>
140503650:     	movq	%rbx, 0x78(%rsp)
140503655:     	movq	%rax, 0x80(%rsp)
14050365d:     	leaq	0x78(%rsp), %rcx
140503662:     	callq	0x1405bd910 <.text+0x5bc910>
140503667:     	leaq	0x48(%rsp), %rbx
14050366c:     	movq	%rbx, %rcx
14050366f:     	movq	%rdi, %rdx
140503672:     	movq	%rax, %r8
140503675:     	callq	0x1406861b0 <.text+0x6851b0>
14050367a:     	movq	%rbx, %rcx
14050367d:     	callq	0x140683ce0 <.text+0x682ce0>
140503682:     	movl	%eax, %ecx
140503684:     	xorb	$0x1, %cl
140503687:     	movzbl	%cl, %ecx
14050368a:     	leaq	0x11(,%rcx,2), %rcx
140503692:     	leaq	0x24ea7bc(%rip), %rdx   # 0x1429ede55
140503699:     	leaq	0x24ea7a2(%rip), %r8    # 0x1429ede42
1405036a0:     	testb	%al, %al
1405036a2:     	cmovneq	%rdx, %r8
1405036a6:     	movq	%r8, 0x20(%rsp)
1405036ab:     	movq	%rcx, 0x28(%rsp)
1405036b0:     	movl	$0x428c0000, 0x30(%rsp) # imm = 0x428C0000
1405036b8:     	leaq	0x24ea7a7(%rip), %r8    # 0x1429ede66
1405036bf:     	leaq	0x78(%rsp), %rdi
1405036c4:     	movl	$0xb, %r9d
1405036ca:     	movq	%rdi, %rcx
1405036cd:     	movq	%rbx, %rdx
1405036d0:     	callq	0x14032e0b0 <.text+0x32d0b0>
1405036d5:     	movq	%rdi, %rcx
1405036d8:     	callq	0x140681d50 <.text+0x680d50>
1405036dd:     	movq	%rax, %rcx
1405036e0:     	callq	0x140683c80 <.text+0x682c80>
1405036e5:     	testb	%al, %al
1405036e7:     	je	0x1405036f1 <.text+0x5026f1>
1405036e9:     	movq	%rsi, %rcx
1405036ec:     	callq	0x1403d09d0 <.text+0x3cf9d0>
1405036f1:     	nop
1405036f2:     	addq	$0xf8, %rsp
1405036f9:     	popq	%rbx
1405036fa:     	popq	%rbp
1405036fb:     	popq	%rdi
1405036fc:     	popq	%rsi
1405036fd:     	popq	%r12
1405036ff:     	popq	%r13
140503701:     	popq	%r14
140503703:     	popq	%r15
140503705:     	retq
