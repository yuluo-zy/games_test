
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146ec35:     	movq	0x218(%rbp), %rcx
14146ec3c:     	testq	%rcx, %rcx
14146ec3f:     	je	0x14146ec9c <.text+0x146dc9c>
14146ec41:     	movq	0x210(%rbp), %rax
14146ec48:     	leaq	(%rcx,%rcx,4), %rcx
14146ec4c:     	leaq	(%rax,%rcx,8), %rcx
14146ec50:     	jmp	0x14146ec5b <.text+0x146dc5b>
14146ec52:     	addq	$0x28, %rax
14146ec56:     	cmpq	%rcx, %rax
14146ec59:     	je	0x14146ec9c <.text+0x146dc9c>
14146ec5b:     	movq	0x20(%rax), %r8
14146ec5f:     	testq	%r8, %r8
14146ec62:     	je	0x14146ec52 <.text+0x146dc52>
14146ec64:     	movq	0x18(%rax), %rdx
14146ec68:     	imulq	$0x58, %r8, %r8
14146ec6c:     	xorl	%r9d, %r9d
14146ec6f:     	jmp	0x14146ec89 <.text+0x146dc89>
14146ec71:     	nopw	%cs:(%rax,%rax)
14146ec80:     	addq	$0x58, %r9
14146ec84:     	cmpq	%r9, %r8
14146ec87:     	je	0x14146ec52 <.text+0x146dc52>
14146ec89:     	cmpb	$0x3a, 0x52(%rdx,%r9)
14146ec8f:     	jne	0x14146ec80 <.text+0x146dc80>
14146ec91:     	movl	$0x3e7, 0x30(%rdx,%r9)  # imm = 0x3E7
14146ec9a:     	jmp	0x14146ec80 <.text+0x146dc80>
