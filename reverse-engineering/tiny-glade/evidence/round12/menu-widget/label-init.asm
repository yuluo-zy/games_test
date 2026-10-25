
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f9d930:     	pushq	%rbp
140f9d931:     	pushq	%r14
140f9d933:     	pushq	%rsi
140f9d934:     	pushq	%rdi
140f9d935:     	pushq	%rbx
140f9d936:     	subq	$0x290, %rsp            # imm = 0x290
140f9d93d:     	leaq	0x80(%rsp), %rbp
140f9d945:     	movq	$-0x2, 0x208(%rbp)
140f9d950:     	movq	(%rcx), %rdi
140f9d953:     	cmpq	$0x0, 0x1b8(%rdi)
140f9d95b:     	je	0x140f9da35 <.text+0xf9ca35>
140f9d961:     	movl	%edx, %ebx
140f9d963:     	movq	%rcx, %rsi
140f9d966:     	leaq	0x8(%rcx), %rdx
140f9d96a:     	leaq	0x1c0(%rdi), %rcx
140f9d971:     	callq	0x1405b0d70 <.text+0x5afd70>
140f9d976:     	movq	0x1a0(%rdi), %rcx
140f9d97d:     	movq	0x1a8(%rdi), %rdx
140f9d984:     	movq	%rdx, %r8
140f9d987:     	andq	%rax, %r8
140f9d98a:     	shrq	$0x39, %rax
140f9d98e:     	movd	%eax, %xmm0
140f9d992:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140f9d996:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140f9d99b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140f9d9a0:     	leaq	-0xc0(%rcx), %rax
140f9d9a7:     	movq	0x8(%rsi), %r9
140f9d9ab:     	xorl	%r10d, %r10d
140f9d9ae:     	pcmpeqd	%xmm1, %xmm1
140f9d9b2:     	movdqu	(%rcx,%r8), %xmm2
140f9d9b8:     	movdqa	%xmm2, %xmm3
140f9d9bc:     	pcmpeqb	%xmm0, %xmm3
140f9d9c0:     	pmovmskb	%xmm3, %r11d
140f9d9c5:     	testl	%r11d, %r11d
140f9d9c8:     	je	0x140f9da00 <.text+0xf9ca00>
140f9d9ca:     	tzcntl	%r11d, %esi
140f9d9cf:     	addq	%r8, %rsi
140f9d9d2:     	andq	%rdx, %rsi
140f9d9d5:     	negq	%rsi
140f9d9d8:     	leaq	(%rsi,%rsi,2), %rsi
140f9d9dc:     	shlq	$0x6, %rsi
140f9d9e0:     	cmpq	(%rax,%rsi), %r9
140f9d9e4:     	je	0x140f9da1e <.text+0xf9ca1e>
140f9d9e6:     	leal	-0x1(%r11), %esi
140f9d9ea:     	andw	%r11w, %si
140f9d9ee:     	movl	%esi, %r11d
140f9d9f1:     	jne	0x140f9d9ca <.text+0xf9c9ca>
140f9d9f3:     	nopw	%cs:(%rax,%rax)
140f9da00:     	pcmpeqb	%xmm1, %xmm2
140f9da04:     	pmovmskb	%xmm2, %r11d
140f9da09:     	testl	%r11d, %r11d
140f9da0c:     	jne	0x140f9da35 <.text+0xf9ca35>
140f9da0e:     	addq	%r10, %r8
140f9da11:     	addq	$0x10, %r8
140f9da15:     	addq	$0x10, %r10
140f9da19:     	andq	%rdx, %r8
140f9da1c:     	jmp	0x140f9d9b2 <.text+0xf9c9b2>
140f9da1e:     	leaq	(%rcx,%rsi), %r14
140f9da22:     	movq	-0x38(%rcx,%rsi), %rcx
140f9da27:     	testq	%rcx, %rcx
140f9da2a:     	je	0x140f9da4d <.text+0xf9ca4d>
140f9da2c:     	movq	-0x30(%r14), %rax
140f9da30:     	jmp	0x140f9daca <.text+0xf9caca>
140f9da35:     	leaq	0x1c11604(%rip), %rcx   # 0x142baf040
140f9da3c:     	leaq	0x1c1160d(%rip), %r8    # 0x142baf050
140f9da43:     	movl	$0xb, %edx
140f9da48:     	callq	0x1428d9330 <.text+0x28d8330>
140f9da4d:     	testb	%bl, %bl
140f9da4f:     	jne	0x140f9db0a <.text+0xf9cb0a>
140f9da55:     	movq	0x260(%rdi), %r9
140f9da5c:     	lock
140f9da5d:     	incq	(%r9)
140f9da60:     	jle	0x140f9db65 <.text+0xf9cb65>
140f9da66:     	leaq	-0x58(%rbp), %rcx
140f9da6a:     	movd	0x1a793ce(%rip), %xmm1  # 0x142a16e40
140f9da72:     	xorl	%r8d, %r8d
140f9da75:     	callq	0x140fac1f0 <.text+0xfab1f0>
140f9da7a:     	movl	$0xffffffff, 0x200(%rbp) # imm = 0xFFFFFFFF
140f9da84:     	movzbl	0x269717e(%rip), %eax   # 0x143634c09
140f9da8b:     	movl	$0x260, %ecx            # imm = 0x260
140f9da90:     	movl	$0x8, %edx
140f9da95:     	callq	0x140613c10 <.text+0x612c10>
140f9da9a:     	testq	%rax, %rax
140f9da9d:     	je	0x140f9db56 <.text+0xf9cb56>
140f9daa3:     	leaq	-0x58(%rbp), %rdx
140f9daa7:     	movl	$0x260, %r8d            # imm = 0x260
140f9daad:     	movq	%rax, %rsi
140f9dab0:     	movq	%rax, %rcx
140f9dab3:     	callq	0x142923c00 <.text+0x2922c00>
140f9dab8:     	movq	%rsi, %rcx
140f9dabb:     	movq	%rsi, -0x38(%r14)
140f9dabf:     	leaq	0x1c115f2(%rip), %rax   # 0x142baf0b8
140f9dac6:     	movq	%rax, -0x30(%r14)
140f9daca:     	callq	*0x28(%rax)
140f9dacd:     	movq	%rax, %rsi
140f9dad0:     	movq	%rax, %rcx
140f9dad3:     	callq	*0x18(%rdx)
140f9dad6:     	testq	%rsi, %rsi
140f9dad9:     	je	0x140f9db3e <.text+0xf9cb3e>
140f9dadb:     	movabsq	$-0x239fc26ae46c7f4e, %r8 # imm = 0xDC603D951B9380B2
140f9dae5:     	cmpq	%r8, %rax
140f9dae8:     	jne	0x140f9db3e <.text+0xf9cb3e>
140f9daea:     	movabsq	$-0x3196240f511ec25e, %rcx # imm = 0xCE69DBF0AEE13DA2
140f9daf4:     	cmpq	%rcx, %rdx
140f9daf7:     	jne	0x140f9db3e <.text+0xf9cb3e>
140f9daf9:     	movq	%rsi, %rax
140f9dafc:     	addq	$0x290, %rsp            # imm = 0x290
140f9db03:     	popq	%rbx
140f9db04:     	popq	%rdi
140f9db05:     	popq	%rsi
140f9db06:     	popq	%r14
140f9db08:     	popq	%rbp
140f9db09:     	retq
140f9db0a:     	leaq	0x1c11667(%rip), %rax   # 0x142baf178
140f9db11:     	movq	%rax, -0x58(%rbp)
140f9db15:     	movq	$0x1, -0x50(%rbp)
140f9db1d:     	movq	$0x8, -0x48(%rbp)
140f9db25:     	pxor	%xmm0, %xmm0
140f9db29:     	movdqu	%xmm0, -0x40(%rbp)
140f9db2e:     	leaq	0x1c11653(%rip), %rdx   # 0x142baf188
140f9db35:     	leaq	-0x58(%rbp), %rcx
140f9db39:     	callq	0x1428d9390 <.text+0x28d8390>
140f9db3e:     	leaq	0x1c11523(%rip), %rcx   # 0x142baf068
140f9db45:     	leaq	0x1c11524(%rip), %r8    # 0x142baf070
140f9db4c:     	movl	$0x8, %edx
140f9db51:     	callq	0x1428d9330 <.text+0x28d8330>
140f9db56:     	movl	$0x8, %ecx
140f9db5b:     	movl	$0x260, %edx            # imm = 0x260
140f9db60:     	callq	0x1428d9000 <.text+0x28d8000>
140f9db65:     	ud2
140f9db67:     	nopw	(%rax,%rax)
140f9db70:     	movq	%rdx, 0x10(%rsp)
140f9db75:     	pushq	%rbp
140f9db76:     	pushq	%r14
140f9db78:     	pushq	%rsi
140f9db79:     	pushq	%rdi
140f9db7a:     	pushq	%rbx
140f9db7b:     	subq	$0x20, %rsp
140f9db7f:     	leaq	0x80(%rdx), %rbp
140f9db86:     	leaq	-0x58(%rbp), %rcx
140f9db8a:     	callq	0x140fa47e0 <.text+0xfa37e0>
140f9db8f:     	nop
140f9db90:     	addq	$0x20, %rsp
140f9db94:     	popq	%rbx
140f9db95:     	popq	%rdi
140f9db96:     	popq	%rsi
140f9db97:     	popq	%r14
140f9db99:     	popq	%rbp
140f9db9a:     	retq
