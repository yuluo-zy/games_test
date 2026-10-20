
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146f6d8:     	movq	0x218(%rbp), %rcx
14146f6df:     	testq	%rcx, %rcx
14146f6e2:     	je	0x14146f79e <.text+0x146e79e>
14146f6e8:     	movq	0x210(%rbp), %rax
14146f6ef:     	leaq	(%rcx,%rcx,4), %rcx
14146f6f3:     	leaq	(%rax,%rcx,8), %rcx
14146f6f7:     	leaq	0x17f55ce(%rip), %rdx   # 0x142c64ccc
14146f6fe:     	jmp	0x14146f70d <.text+0x146e70d>
14146f700:     	addq	$0x28, %rax
14146f704:     	cmpq	%rcx, %rax
14146f707:     	je	0x14146f79e <.text+0x146e79e>
14146f70d:     	movq	0x20(%rax), %r9
14146f711:     	testq	%r9, %r9
14146f714:     	je	0x14146f700 <.text+0x146e700>
14146f716:     	movq	0x18(%rax), %r8
14146f71a:     	imulq	$0x58, %r9, %r9
14146f71e:     	xorl	%r10d, %r10d
14146f721:     	jmp	0x14146f741 <.text+0x146e741>
14146f723:     	nopw	%cs:(%rax,%rax)
14146f730:     	movl	%esi, %r11d
14146f733:     	movl	%r11d, 0x30(%r8,%r10)
14146f738:     	addq	$0x58, %r10
14146f73c:     	cmpq	%r10, %r9
14146f73f:     	je	0x14146f700 <.text+0x146e700>
14146f741:     	movzbl	0x52(%r8,%r10), %r11d
14146f747:     	addb	$-0x38, %r11b
14146f74b:     	cmpb	$0x2, %r11b
14146f74f:     	ja	0x14146f738 <.text+0x146e738>
14146f751:     	movl	0x30(%r8,%r10), %esi
14146f756:     	cmpq	$0x8, %rsi
14146f75a:     	ja	0x14146f76e <.text+0x146e76e>
14146f75c:     	movslq	(%rdx,%rsi,4), %r11
14146f760:     	addq	%rdx, %r11
14146f763:     	jmpq	*%r11
14146f766:     	movl	$0x2, %r11d
14146f76c:     	jmp	0x14146f733 <.text+0x146e733>
14146f76e:     	movl	$0x1, %r11d
14146f774:     	cmpl	$0x3e7, %esi            # imm = 0x3E7
14146f77a:     	je	0x14146f730 <.text+0x146e730>
14146f77c:     	jmp	0x14146f733 <.text+0x146e733>
14146f77e:     	movl	$0x3, %r11d
14146f784:     	jmp	0x14146f733 <.text+0x146e733>
14146f786:     	movl	$0x5, %r11d
14146f78c:     	jmp	0x14146f733 <.text+0x146e733>
14146f78e:     	movl	$0x8, %r11d
14146f794:     	jmp	0x14146f733 <.text+0x146e733>
14146f796:     	movl	$0x4, %r11d
14146f79c:     	jmp	0x14146f733 <.text+0x146e733>
