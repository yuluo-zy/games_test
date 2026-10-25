
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140683ce0:     	pushq	%rsi
140683ce1:     	pushq	%rdi
140683ce2:     	subq	$0x28, %rsp
140683ce6:     	movq	(%rcx), %rdi
140683ce9:     	cmpq	$0x0, 0x1b8(%rdi)
140683cf1:     	je	0x140683dbc <.text+0x682dbc>
140683cf7:     	movq	%rcx, %rsi
140683cfa:     	leaq	0x8(%rcx), %rdx
140683cfe:     	leaq	0x1c0(%rdi), %rcx
140683d05:     	callq	0x1405b0d70 <.text+0x5afd70>
140683d0a:     	movq	0x1a0(%rdi), %rcx
140683d11:     	movq	0x1a8(%rdi), %rdx
140683d18:     	movq	%rdx, %r8
140683d1b:     	andq	%rax, %r8
140683d1e:     	shrq	$0x39, %rax
140683d22:     	movd	%eax, %xmm0
140683d26:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140683d2a:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140683d2f:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140683d34:     	leaq	-0xc0(%rcx), %rax
140683d3b:     	movq	0x8(%rsi), %r9
140683d3f:     	xorl	%r10d, %r10d
140683d42:     	pcmpeqd	%xmm1, %xmm1
140683d46:     	movdqu	(%rcx,%r8), %xmm2
140683d4c:     	movdqa	%xmm2, %xmm3
140683d50:     	pcmpeqb	%xmm0, %xmm3
140683d54:     	pmovmskb	%xmm3, %r11d
140683d59:     	testl	%r11d, %r11d
140683d5c:     	je	0x140683d90 <.text+0x682d90>
140683d5e:     	tzcntl	%r11d, %esi
140683d63:     	addq	%r8, %rsi
140683d66:     	andq	%rdx, %rsi
140683d69:     	negq	%rsi
140683d6c:     	leaq	(%rsi,%rsi,2), %rsi
140683d70:     	shlq	$0x6, %rsi
140683d74:     	cmpq	(%rax,%rsi), %r9
140683d78:     	je	0x140683dae <.text+0x682dae>
140683d7a:     	leal	-0x1(%r11), %esi
140683d7e:     	andw	%r11w, %si
140683d82:     	movl	%esi, %r11d
140683d85:     	jne	0x140683d5e <.text+0x682d5e>
140683d87:     	nopw	(%rax,%rax)
140683d90:     	pcmpeqb	%xmm1, %xmm2
140683d94:     	pmovmskb	%xmm2, %r11d
140683d99:     	testl	%r11d, %r11d
140683d9c:     	jne	0x140683dbc <.text+0x682dbc>
140683d9e:     	addq	%r10, %r8
140683da1:     	addq	$0x10, %r8
140683da5:     	addq	$0x10, %r10
140683da9:     	andq	%rdx, %r8
140683dac:     	jmp	0x140683d46 <.text+0x682d46>
140683dae:     	movzbl	-0x2(%rcx,%rsi), %eax
140683db3:     	andb	$0x1, %al
140683db5:     	addq	$0x28, %rsp
140683db9:     	popq	%rdi
140683dba:     	popq	%rsi
140683dbb:     	retq
140683dbc:     	leaq	0x2393e5d(%rip), %rcx   # 0x142a17c20
140683dc3:     	callq	0x1428d9310 <.text+0x28d8310>
140683dc8:     	int3
