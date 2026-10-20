
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140683dd0:     	pushq	%rsi
140683dd1:     	pushq	%rdi
140683dd2:     	subq	$0x28, %rsp
140683dd6:     	movq	(%rcx), %rdi
140683dd9:     	cmpq	$0x0, 0x1b8(%rdi)
140683de1:     	je	0x140683eaa <.text+0x682eaa>
140683de7:     	movq	%rcx, %rsi
140683dea:     	leaq	0x8(%rcx), %rdx
140683dee:     	leaq	0x1c0(%rdi), %rcx
140683df5:     	callq	0x1405b0d70 <.text+0x5afd70>
140683dfa:     	movq	0x1a0(%rdi), %rcx
140683e01:     	movq	0x1a8(%rdi), %rdx
140683e08:     	movq	%rdx, %r8
140683e0b:     	andq	%rax, %r8
140683e0e:     	shrq	$0x39, %rax
140683e12:     	movd	%eax, %xmm0
140683e16:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140683e1a:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140683e1f:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140683e24:     	leaq	-0xc0(%rcx), %rax
140683e2b:     	movq	0x8(%rsi), %r9
140683e2f:     	xorl	%r10d, %r10d
140683e32:     	pcmpeqd	%xmm1, %xmm1
140683e36:     	movdqu	(%rcx,%r8), %xmm2
140683e3c:     	movdqa	%xmm2, %xmm3
140683e40:     	pcmpeqb	%xmm0, %xmm3
140683e44:     	pmovmskb	%xmm3, %r11d
140683e49:     	testl	%r11d, %r11d
140683e4c:     	je	0x140683e80 <.text+0x682e80>
140683e4e:     	tzcntl	%r11d, %esi
140683e53:     	addq	%r8, %rsi
140683e56:     	andq	%rdx, %rsi
140683e59:     	negq	%rsi
140683e5c:     	leaq	(%rsi,%rsi,2), %rsi
140683e60:     	shlq	$0x6, %rsi
140683e64:     	cmpq	(%rax,%rsi), %r9
140683e68:     	je	0x140683e9e <.text+0x682e9e>
140683e6a:     	leal	-0x1(%r11), %esi
140683e6e:     	andw	%r11w, %si
140683e72:     	movl	%esi, %r11d
140683e75:     	jne	0x140683e4e <.text+0x682e4e>
140683e77:     	nopw	(%rax,%rax)
140683e80:     	pcmpeqb	%xmm1, %xmm2
140683e84:     	pmovmskb	%xmm2, %r11d
140683e89:     	testl	%r11d, %r11d
140683e8c:     	jne	0x140683eaa <.text+0x682eaa>
140683e8e:     	addq	%r10, %r8
140683e91:     	addq	$0x10, %r8
140683e95:     	addq	$0x10, %r10
140683e99:     	andq	%rdx, %r8
140683e9c:     	jmp	0x140683e36 <.text+0x682e36>
140683e9e:     	movzbl	-0x2(%rcx,%rsi), %eax
140683ea3:     	addq	$0x28, %rsp
140683ea7:     	popq	%rdi
140683ea8:     	popq	%rsi
140683ea9:     	retq
140683eaa:     	leaq	0x2393d87(%rip), %rcx   # 0x142a17c38
140683eb1:     	callq	0x1428d9310 <.text+0x28d8310>
140683eb6:     	int3
