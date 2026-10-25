
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140307300:     	pushq	%rbp
140307301:     	pushq	%rsi
140307302:     	pushq	%rdi
140307303:     	subq	$0x220, %rsp            # imm = 0x220
14030730a:     	leaq	0x80(%rsp), %rbp
140307312:     	movq	$-0x2, 0x198(%rbp)
14030731d:     	movq	%rcx, %rsi
140307320:     	xorps	%xmm0, %xmm0
140307323:     	movups	%xmm0, 0x78(%rbp)
140307327:     	movups	%xmm0, 0x90(%rbp)
14030732e:     	movups	%xmm0, 0xa8(%rbp)
140307335:     	movups	%xmm0, 0xc0(%rbp)
14030733c:     	movups	%xmm0, 0xd8(%rbp)
140307343:     	movl	$0x0, 0xe8(%rbp)
14030734d:     	movups	%xmm0, 0x110(%rbp)
140307354:     	movups	%xmm0, 0x128(%rbp)
14030735b:     	movups	%xmm0, 0x140(%rbp)
140307362:     	movups	%xmm0, 0x158(%rbp)
140307369:     	movups	%xmm0, 0x170(%rbp)
140307370:     	movl	$0x0, 0x180(%rbp)
14030737a:     	movq	$0x0, -0x60(%rbp)
140307382:     	movq	$0x0, 0x58(%rbp)
14030738a:     	movq	$0x8, 0x60(%rbp)
140307392:     	movq	$0x0, 0x68(%rbp)
14030739a:     	movq	$0x10, 0x70(%rbp)
1403073a2:     	movq	$0x10, 0x88(%rbp)
1403073ad:     	movq	$0x10, 0xa0(%rbp)
1403073b8:     	movq	$0x10, 0xb8(%rbp)
1403073c3:     	movq	$0x10, 0xd0(%rbp)
1403073ce:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403073d8:     	movq	%rax, 0xf0(%rbp)
1403073df:     	leaq	0x26b084f(%rip), %rax   # 0x1429b7c35
1403073e6:     	movq	%rax, 0xf8(%rbp)
1403073ed:     	movq	$0x37, 0x100(%rbp)
1403073f8:     	movq	$0x10, 0x108(%rbp)
140307403:     	movq	$0x10, 0x120(%rbp)
14030740e:     	movq	$0x10, 0x138(%rbp)
140307419:     	movq	$0x10, 0x150(%rbp)
140307424:     	movq	$0x10, 0x168(%rbp)
14030742f:     	movl	$0x0, 0x188(%rbp)
140307439:     	movw	$0x1, 0x18c(%rbp)
140307442:     	movl	$0x0, 0x190(%rbp)
14030744c:     	movzbl	0x332d7b6(%rip), %eax   # 0x143634c09
140307453:     	movl	$0x1f8, %ecx            # imm = 0x1F8
140307458:     	movl	$0x8, %edx
14030745d:     	callq	0x140613c10 <.text+0x612c10>
140307462:     	testq	%rax, %rax
140307465:     	je	0x14030749c <.text+0x30649c>
140307467:     	movq	%rax, %rdi
14030746a:     	leaq	-0x60(%rbp), %rdx
14030746e:     	movl	$0x1f8, %r8d            # imm = 0x1F8
140307474:     	movq	%rax, %rcx
140307477:     	callq	0x142923c00 <.text+0x2922c00>
14030747c:     	leaq	0x26b06dd(%rip), %r8    # 0x1429b7b60
140307483:     	movq	%rsi, %rcx
140307486:     	movq	%rdi, %rdx
140307489:     	callq	0x140ce4b10 <.text+0xce3b10>
14030748e:     	movq	%rsi, %rax
140307491:     	addq	$0x220, %rsp            # imm = 0x220
140307498:     	popq	%rdi
140307499:     	popq	%rsi
14030749a:     	popq	%rbp
14030749b:     	retq
14030749c:     	movl	$0x8, %ecx
1403074a1:     	movl	$0x1f8, %edx            # imm = 0x1F8
1403074a6:     	callq	0x1428d9000 <.text+0x28d8000>
1403074ab:     	ud2
