
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146ed83:     	movq	0x248(%rbp), %rax
14146ed8a:     	testq	%rax, %rax
14146ed8d:     	je	0x14146f6d8 <.text+0x146e6d8>
14146ed93:     	movq	0x240(%rbp), %rcx
14146ed9a:     	shlq	$0x4, %rax
14146ed9e:     	xorl	%edx, %edx
14146eda0:     	leaq	0x17f5f01(%rip), %r8    # 0x142c64ca8
14146eda7:     	jmp	0x14146edbe <.text+0x146ddbe>
14146eda9:     	movl	%r10d, %r9d
14146edac:     	movl	%r9d, 0x8(%rcx,%rdx)
14146edb1:     	addq	$0x10, %rdx
14146edb5:     	cmpq	%rdx, %rax
14146edb8:     	je	0x14146f6d8 <.text+0x146e6d8>
14146edbe:     	movl	0x8(%rcx,%rdx), %r10d
14146edc3:     	cmpq	$0x8, %r10
14146edc7:     	ja	0x14146eddb <.text+0x146dddb>
14146edc9:     	movslq	(%r8,%r10,4), %r9
14146edcd:     	addq	%r8, %r9
14146edd0:     	jmpq	*%r9
14146edd3:     	movl	$0x2, %r9d
14146edd9:     	jmp	0x14146edac <.text+0x146ddac>
14146eddb:     	movl	$0x1, %r9d
14146ede1:     	cmpl	$0x3e7, %r10d           # imm = 0x3E7
14146ede8:     	je	0x14146eda9 <.text+0x146dda9>
14146edea:     	jmp	0x14146edac <.text+0x146ddac>
14146edec:     	movl	$0x3, %r9d
14146edf2:     	jmp	0x14146edac <.text+0x146ddac>
14146edf4:     	movl	$0x5, %r9d
14146edfa:     	jmp	0x14146edac <.text+0x146ddac>
14146edfc:     	movl	$0x8, %r9d
14146ee02:     	jmp	0x14146edac <.text+0x146ddac>
14146ee04:     	movl	$0x4, %r9d
14146ee0a:     	jmp	0x14146edac <.text+0x146ddac>
