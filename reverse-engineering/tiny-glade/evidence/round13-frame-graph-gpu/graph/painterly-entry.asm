
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408a6410:     	pushq	%rbp
1408a6411:     	pushq	%r15
1408a6413:     	pushq	%r14
1408a6415:     	pushq	%r13
1408a6417:     	pushq	%r12
1408a6419:     	pushq	%rsi
1408a641a:     	pushq	%rdi
1408a641b:     	pushq	%rbx
1408a641c:     	subq	$0xaf8, %rsp            # imm = 0xAF8
1408a6423:     	leaq	0x80(%rsp), %rbp
1408a642b:     	movaps	%xmm7, 0xa60(%rbp)
1408a6432:     	movapd	%xmm6, 0xa50(%rbp)
1408a643a:     	movq	$-0x2, 0xa48(%rbp)
1408a6445:     	movq	%r9, %r15
1408a6448:     	movq	%r8, %r13
1408a644b:     	movq	%rdx, %rsi
1408a644e:     	movq	%rcx, %r12
1408a6451:     	leaq	0x38(%rdx), %r14
1408a6455:     	movabsq	$-0x2c72519e80486386, %rdx # imm = 0xD38DAE617FB79C7A
1408a645f:     	movabsq	$0x22aadf5c5dad1eae, %r8 # imm = 0x22AADF5C5DAD1EAE
1408a6469:     	movq	%r14, %rcx
1408a646c:     	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a6471:     	testq	%rax, %rax
1408a6474:     	je	0x1408a77b9 <.text+0x8a67b9>
1408a647a:     	cmpq	%rdx, 0x218(%rsi)
1408a6481:     	jbe	0x1408a77b9 <.text+0x8a67b9>
1408a6487:     	movq	0x210(%rsi), %rax
1408a648e:     	movq	(%rax,%rdx,8), %rcx
1408a6492:     	testq	%rcx, %rcx
1408a6495:     	je	0x1408a77b9 <.text+0x8a67b9>
1408a649b:     	movq	0x1e0(%rsi), %rax
1408a64a2:     	notq	%rcx
1408a64a5:     	leaq	(%rcx,%rcx,2), %rcx
1408a64a9:     	shlq	$0x5, %rcx
1408a64ad:     	cmpq	$0x0, 0x20(%rax,%rcx)
1408a64b3:     	je	0x1408a77b9 <.text+0x8a67b9>
1408a64b9:     	addq	%rcx, %rax
1408a64bc:     	movq	0x10(%rax), %rax
1408a64c0:     	cmpl	$0x7, (%rax)
1408a64c3:     	jne	0x1408a64cf <.text+0x8a54cf>
1408a64c5:     	cmpb	$0x1, 0x8(%rax)
1408a64c9:     	je	0x1408a7797 <.text+0x8a6797>
1408a64cf:     	movabsq	$0x1f18d9c9bdba6c60, %rdx # imm = 0x1F18D9C9BDBA6C60
1408a64d9:     	movabsq	$-0x7ab86799b716f499, %r8 # imm = 0x8547986648E90B67
1408a64e3:     	movq	%r14, %rcx
1408a64e6:     	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a64eb:     	testq	%rax, %rax
1408a64ee:     	je	0x1408a77c5 <.text+0x8a67c5>
1408a64f4:     	cmpq	%rdx, 0x218(%rsi)
1408a64fb:     	jbe	0x1408a77c5 <.text+0x8a67c5>
1408a6501:     	movq	0x210(%rsi), %rax
1408a6508:     	movq	(%rax,%rdx,8), %rcx
1408a650c:     	testq	%rcx, %rcx
1408a650f:     	je	0x1408a77c5 <.text+0x8a67c5>
1408a6515:     	movq	0x1e0(%rsi), %rax
1408a651c:     	notq	%rcx
1408a651f:     	leaq	(%rcx,%rcx,2), %rcx
1408a6523:     	shlq	$0x5, %rcx
1408a6527:     	cmpq	$0x0, 0x20(%rax,%rcx)
1408a652d:     	je	0x1408a77c5 <.text+0x8a67c5>
1408a6533:     	addq	%rcx, %rax
1408a6536:     	movq	0x10(%rax), %rdi
1408a653a:     	movl	0x5b0(%rsi), %ecx
1408a6540:     	movl	%ecx, 0x5c(%rax)
1408a6543:     	movabsq	$0x48b321ef779dc301, %rdx # imm = 0x48B321EF779DC301
1408a654d:     	movabsq	$-0x429a898e6b262238, %r8 # imm = 0xBD65767194D9DDC8
1408a6557:     	movq	%r14, %rcx
1408a655a:     	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a655f:     	testq	%rax, %rax
1408a6562:     	je	0x1408a77d1 <.text+0x8a67d1>
1408a6568:     	cmpq	%rdx, 0x218(%rsi)
1408a656f:     	jbe	0x1408a77d1 <.text+0x8a67d1>
1408a6575:     	movq	0x210(%rsi), %rax
1408a657c:     	movq	(%rax,%rdx,8), %rcx
1408a6580:     	testq	%rcx, %rcx
1408a6583:     	je	0x1408a77d1 <.text+0x8a67d1>
1408a6589:     	movq	0x1e0(%rsi), %rax
1408a6590:     	notq	%rcx
1408a6593:     	leaq	(%rcx,%rcx,2), %rcx
1408a6597:     	shlq	$0x5, %rcx
1408a659b:     	cmpq	$0x0, 0x20(%rax,%rcx)
1408a65a1:     	je	0x1408a77d1 <.text+0x8a67d1>
1408a65a7:     	movzbl	0xae8(%rbp), %ebx
1408a65ae:     	addq	%rcx, %rax
1408a65b1:     	movq	0x10(%rax), %rax
1408a65b5:     	cmpb	$0x0, 0xff(%rax)
1408a65bc:     	je	0x1408a662b <.text+0x8a562b>
1408a65be:     	movb	$0x1, %cl
1408a65c0:     	testb	%bl, %bl
1408a65c2:     	je	0x1408a66a8 <.text+0x8a56a8>
1408a65c8:     	movl	0x8(%rdi), %eax
1408a65cb:     	cmpl	$0x2, %eax
1408a65ce:     	jne	0x1408a65e7 <.text+0x8a55e7>
1408a65d0:     	movq	$0x0, (%rdi)
1408a65d7:     	movl	$0x0, 0x8(%rdi)
1408a65de:     	movl	$0x0, 0x14(%rdi)
1408a65e5:     	xorl	%eax, %eax
1408a65e7:     	testb	%cl, %cl
1408a65e9:     	movq	%rdi, 0xa30(%rbp)
1408a65f0:     	movq	%r12, 0xa38(%rbp)
1408a65f7:     	movq	%r13, 0xa20(%rbp)
1408a65fe:     	movb	%cl, 0xa46(%rbp)
1408a6604:     	movq	%r14, 0x918(%rbp)
1408a660b:     	je	0x1408a66b4 <.text+0x8a56b4>
1408a6611:     	movl	$0x0, 0x8(%rdi)
1408a6618:     	movq	$0x0, (%rdi)
1408a661f:     	movl	$0x0, 0x14(%rdi)
1408a6626:     	jmp	0x1408a66c1 <.text+0x8a56c1>
1408a662b:     	movabsq	$-0x702623e63868c436, %rdx # imm = 0x8FD9DC19C7973BCA
1408a6635:     	movabsq	$0x361ff4c5eb9b45fd, %r8 # imm = 0x361FF4C5EB9B45FD
1408a663f:     	movq	%r14, %rcx
1408a6642:     	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a6647:     	testq	%rax, %rax
1408a664a:     	je	0x1408a77dd <.text+0x8a67dd>
1408a6650:     	cmpq	%rdx, 0x218(%rsi)
1408a6657:     	jbe	0x1408a77dd <.text+0x8a67dd>
1408a665d:     	movq	0x210(%rsi), %rax
1408a6664:     	movq	(%rax,%rdx,8), %rcx
1408a6668:     	testq	%rcx, %rcx
1408a666b:     	je	0x1408a77dd <.text+0x8a67dd>
1408a6671:     	movq	0x1e0(%rsi), %rax
1408a6678:     	notq	%rcx
1408a667b:     	leaq	(%rcx,%rcx,2), %rcx
1408a667f:     	shlq	$0x5, %rcx
1408a6683:     	cmpq	$0x0, 0x20(%rax,%rcx)
1408a6689:     	je	0x1408a77dd <.text+0x8a67dd>
1408a668f:     	addq	%rcx, %rax
1408a6692:     	movq	0x10(%rax), %rax
1408a6696:     	cmpl	$0x0, 0x80(%rax)
1408a669d:     	sete	%cl
1408a66a0:     	testb	%bl, %bl
1408a66a2:     	jne	0x1408a65c8 <.text+0x8a55c8>
1408a66a8:     	movl	$0x2, 0x8(%rdi)
1408a66af:     	jmp	0x1408a7797 <.text+0x8a6797>
1408a66b4:     	testb	$0x1, %al
1408a66b6:     	je	0x1408a66c1 <.text+0x8a56c1>
1408a66b8:     	movq	%r15, %r14
1408a66bb:     	movq	0xc(%rdi), %r13
1408a66bf:     	jmp	0x1408a66ee <.text+0x8a56ee>
1408a66c1:     	leaq	0x4(%r13), %rcx
1408a66c5:     	callq	0x140bd5790 <.text+0xbd4790>
1408a66ca:     	movq	%rax, %rcx
1408a66cd:     	shrq	$0x20, %rcx
1408a66d1:     	je	0x1408a78d6 <.text+0x8a68d6>
1408a66d7:     	movq	%r15, %r14
1408a66da:     	shll	$0x8, %eax
1408a66dd:     	xorl	%edx, %edx
1408a66df:     	divl	%ecx
1408a66e1:     	movabsq	$0x10000000000, %r13    # imm = 0x10000000000
1408a66eb:     	orq	%rax, %r13
1408a66ee:     	leaq	0x4e0(%rbp), %r12
1408a66f5:     	movq	%r12, %rcx
1408a66f8:     	movq	%r13, %rdx
1408a66fb:     	movl	$0x25, %r8d
1408a6701:     	callq	0x140bd52a0 <.text+0xbd42a0>
1408a6706:     	leaq	0xc0(%rbp), %rbx
1408a670d:     	movq	%rbx, %rcx
1408a6710:     	movq	%r12, %rdx
1408a6713:     	callq	0x140bd5390 <.text+0xbd4390>
1408a6718:     	leaq	0x920(%rbp), %r15
1408a671f:     	movq	%r15, %rcx
1408a6722:     	movq	%rbx, %rdx
1408a6725:     	callq	0x140bd5370 <.text+0xbd4370>
1408a672a:     	movq	%r15, 0x20(%rsp)
1408a672f:     	leaq	0x21f4fa2(%rip), %r8    # 0x142a9b6d8
