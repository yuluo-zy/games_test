
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407d1f80:     	pushq	%r15
1407d1f82:     	pushq	%r14
1407d1f84:     	pushq	%r13
1407d1f86:     	pushq	%r12
1407d1f88:     	pushq	%rsi
1407d1f89:     	pushq	%rdi
1407d1f8a:     	pushq	%rbp
1407d1f8b:     	pushq	%rbx
1407d1f8c:     	subq	$0x48, %rsp
1407d1f90:     	movq	(%rcx), %rax
1407d1f93:     	movq	0x18(%rax), %r10
1407d1f97:     	testq	%r10, %r10
1407d1f9a:     	je	0x1407d20a0 <.text+0x7d10a0>
1407d1fa0:     	movl	0x1c(%rdx), %r14d
1407d1fa4:     	movq	(%rdx), %r13
1407d1fa7:     	movq	0x10(%rdx), %rsi
1407d1fab:     	movq	(%rax), %r15
1407d1fae:     	leaq	0x10(%r15), %r12
1407d1fb2:     	movdqa	(%r15), %xmm0
1407d1fb7:     	pmovmskb	%xmm0, %r11d
1407d1fbc:     	notl	%r11d
1407d1fbf:     	leaq	0x30(%rsp), %rbp
1407d1fc4:     	jmp	0x1407d1fe3 <.text+0x7d0fe3>
1407d1fc6:     	nopw	%cs:(%rax,%rax)
1407d1fd0:     	movq	0x28(%rsp), %r10
1407d1fd5:     	decq	%r10
1407d1fd8:     	movq	0x20(%rsp), %r11
1407d1fdd:     	je	0x1407d20a0 <.text+0x7d10a0>
1407d1fe3:     	testw	%r11w, %r11w
1407d1fe7:     	movq	%r10, 0x28(%rsp)
1407d1fec:     	je	0x1407d2000 <.text+0x7d1000>
1407d1fee:     	leal	-0x1(%r11), %eax
1407d1ff2:     	andl	%r11d, %eax
1407d1ff5:     	jmp	0x1407d202f <.text+0x7d102f>
1407d1ff7:     	nopw	(%rax,%rax)
1407d2000:     	movdqa	(%r12), %xmm0
1407d2006:     	pmovmskb	%xmm0, %r11d
1407d200b:     	addq	$-0x1100, %r15          # imm = 0xEF00
1407d2012:     	addq	$0x10, %r12
1407d2016:     	cmpl	$0xffff, %r11d          # imm = 0xFFFF
1407d201d:     	je	0x1407d2000 <.text+0x7d1000>
1407d201f:     	movl	$0xfffffffe, %edx       # imm = 0xFFFFFFFE
1407d2024:     	subl	%r11d, %edx
1407d2027:     	notl	%r11d
1407d202a:     	movl	%r11d, %eax
1407d202d:     	andl	%edx, %eax
1407d202f:     	movl	%r11d, %edx
1407d2032:     	tzcntl	%edx, %edx
1407d2036:     	movq	%rax, 0x20(%rsp)
1407d203b:     	negq	%rdx
1407d203e:     	imulq	$0x110, %rdx, %rax      # imm = 0x110
1407d2045:     	movq	-0xc8(%r15,%rax), %rdx
1407d204d:     	testq	%rdx, %rdx
1407d2050:     	je	0x1407d1fd0 <.text+0x7d0fd0>
1407d2056:     	movq	-0xd0(%r15,%rax), %rax
1407d205e:     	leaq	(%rdx,%rdx,2), %rdx
1407d2062:     	leaq	(%rax,%rdx,8), %rbx
1407d2066:     	nopw	%cs:(%rax,%rax)
1407d2070:     	leaq	0x18(%rax), %rdi
1407d2074:     	movq	(%rax), %rdx
1407d2077:     	movdqu	0x8(%rax), %xmm0
1407d207c:     	movdqa	%xmm0, 0x30(%rsp)
1407d2082:     	movl	%r14d, (%rsi)
1407d2085:     	movq	%r13, %rcx
1407d2088:     	movq	%rbp, %r8
1407d208b:     	movb	$0x4, %r9b
1407d208e:     	callq	0x140f86500 <.text+0xf85500>
1407d2093:     	movq	%rdi, %rax
1407d2096:     	cmpq	%rbx, %rdi
1407d2099:     	jne	0x1407d2070 <.text+0x7d1070>
1407d209b:     	jmp	0x1407d1fd0 <.text+0x7d0fd0>
1407d20a0:     	addq	$0x48, %rsp
1407d20a4:     	popq	%rbx
1407d20a5:     	popq	%rbp
1407d20a6:     	popq	%rdi
1407d20a7:     	popq	%rsi
1407d20a8:     	popq	%r12
1407d20aa:     	popq	%r13
1407d20ac:     	popq	%r14
1407d20ae:     	popq	%r15
1407d20b0:     	retq
1407d20b1:     	int3
1407d20b2:     	int3
1407d20b3:     	int3
1407d20b4:     	int3
1407d20b5:     	int3
1407d20b6:     	int3
1407d20b7:     	int3
1407d20b8:     	int3
1407d20b9:     	int3
1407d20ba:     	int3
1407d20bb:     	int3
