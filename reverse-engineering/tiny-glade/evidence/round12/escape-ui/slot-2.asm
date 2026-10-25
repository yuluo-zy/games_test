
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140503eb0:     	pushq	%r15
140503eb2:     	pushq	%r14
140503eb4:     	pushq	%r13
140503eb6:     	pushq	%r12
140503eb8:     	pushq	%rsi
140503eb9:     	pushq	%rdi
140503eba:     	pushq	%rbp
140503ebb:     	pushq	%rbx
140503ebc:     	subq	$0xe8, %rsp
140503ec3:     	movq	%r9, %rsi
140503ec6:     	movq	%rcx, %rbx
140503ec9:     	movq	%rdx, 0x40(%rsp)
140503ece:     	movq	%r8, 0x48(%rsp)
140503ed3:     	leaq	0x10(%rcx), %rdi
140503ed7:     	movq	%rdi, %rcx
140503eda:     	callq	0x1406868b0 <.text+0x6858b0>
140503edf:     	movq	%rax, %r12
140503ee2:     	leaq	0x40(%rsp), %r15
140503ee7:     	movq	%r15, %rcx
140503eea:     	callq	0x140683c10 <.text+0x682c10>
140503eef:     	movq	%rax, 0x20(%rsp)
140503ef4:     	leaq	0x20(%rsp), %r14
140503ef9:     	movq	%r14, %rcx
140503efc:     	callq	0x1400885d0 <.text+0x875d0>
140503f01:     	movq	%rax, 0x50(%rsp)
140503f06:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140503f10:     	movq	%r13, 0x58(%rsp)
140503f15:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140503f1f:     	movq	%rbp, 0x60(%rsp)
140503f24:     	leaq	0x50(%rsp), %rcx
140503f29:     	callq	0x1405bdf20 <.text+0x5bcf20>
140503f2e:     	movq	%r12, 0x50(%rsp)
140503f33:     	movq	%rax, 0x58(%rsp)
140503f38:     	leaq	0x50(%rsp), %rcx
140503f3d:     	callq	0x1405bd910 <.text+0x5bc910>
140503f42:     	movq	%rax, %r12
140503f45:     	movq	%rdi, %rcx
140503f48:     	movq	%rax, %rdx
140503f4b:     	callq	0x140686450 <.text+0x685450>
140503f50:     	testb	%al, %al
140503f52:     	je	0x140503f6f <.text+0x502f6f>
140503f54:     	leaq	0xd0(%rsp), %rbx
140503f5c:     	movq	%rbx, %rcx
140503f5f:     	movq	%rdi, %rdx
140503f62:     	movq	%r12, %r8
140503f65:     	callq	0x140686530 <.text+0x685530>
140503f6a:     	jmp	0x1405040b5 <.text+0x5030b5>
140503f6f:     	movq	0x3128432(%rip), %rax   # 0x14362c3a8
140503f76:     	movq	(%rax), %rax
140503f79:     	testq	%rax, %rax
140503f7c:     	je	0x14050405a <.text+0x50305a>
140503f82:     	movq	%rbx, 0x20(%rsp)
140503f87:     	leaq	0x2f912(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140503f8e:     	movq	%rax, 0x28(%rsp)
140503f93:     	movq	%r15, 0x30(%rsp)
140503f98:     	movq	%rax, 0x38(%rsp)
140503f9d:     	leaq	0x24e9db4(%rip), %rcx   # 0x1429edd58
140503fa4:     	callq	0x14067dd80 <.text+0x67cd80>
140503fa9:     	movups	(%rax), %xmm0
140503fac:     	movl	0x10(%rax), %eax
140503faf:     	movq	$0x1, 0x80(%rsp)
140503fbb:     	leaq	0x24e9d66(%rip), %rcx   # 0x1429edd28
140503fc2:     	movq	%rcx, 0x88(%rsp)
140503fca:     	movq	$0x14, 0x90(%rsp)
140503fd6:     	leaq	0x24e9cbb(%rip), %rdx   # 0x1429edc98
140503fdd:     	movq	%rdx, 0xa0(%rsp)
140503fe5:     	movq	$0x2, 0xa8(%rsp)
140503ff1:     	movq	%r14, 0xb0(%rsp)
140503ff9:     	movq	$0x2, 0xb8(%rsp)
140504005:     	movq	$0x0, 0xc0(%rsp)
140504011:     	movq	$0x0, 0x50(%rsp)
14050401a:     	movq	%rcx, 0x58(%rsp)
14050401f:     	movq	$0x14, 0x60(%rsp)
140504028:     	movq	$0x0, 0x68(%rsp)
140504031:     	movups	%xmm0, 0x70(%rsp)
140504036:     	movl	$0x1, 0x98(%rsp)
140504041:     	movl	%eax, 0x9c(%rsp)
140504048:     	leaq	0xd0(%rsp), %rcx
140504050:     	leaq	0x50(%rsp), %rdx
140504055:     	callq	0x140d4a330 <.text+0xd49330>
14050405a:     	movups	0x40(%rsp), %xmm0
14050405f:     	movq	0x18(%rbx), %rbx
140504063:     	movups	%xmm0, 0x20(%rsp)
140504068:     	leaq	0x20(%rsp), %rcx
14050406d:     	callq	0x140683c10 <.text+0x682c10>
140504072:     	movq	%rax, 0x50(%rsp)
140504077:     	movq	%r13, 0x58(%rsp)
14050407c:     	movq	%rbp, 0x60(%rsp)
140504081:     	leaq	0x50(%rsp), %rcx
140504086:     	callq	0x1405bdf20 <.text+0x5bcf20>
14050408b:     	movq	%rbx, 0x50(%rsp)
140504090:     	movq	%rax, 0x58(%rsp)
140504095:     	leaq	0x50(%rsp), %rcx
14050409a:     	callq	0x1405bd910 <.text+0x5bc910>
14050409f:     	leaq	0xd0(%rsp), %rbx
1405040a7:     	movq	%rbx, %rcx
1405040aa:     	movq	%rdi, %rdx
1405040ad:     	movq	%rax, %r8
1405040b0:     	callq	0x1406861b0 <.text+0x6851b0>
1405040b5:     	leaq	0x24e9d7d(%rip), %r8    # 0x1429ede39
1405040bc:     	leaq	0x50(%rsp), %rdi
1405040c1:     	movl	$0x9, %r9d
1405040c7:     	movq	%rdi, %rcx
1405040ca:     	movq	%rbx, %rdx
1405040cd:     	callq	0x1406811c0 <.text+0x6801c0>
1405040d2:     	leaq	0x20(%rsp), %rbx
1405040d7:     	movq	%rbx, %rcx
1405040da:     	movq	%rdi, %rdx
1405040dd:     	movb	$0x12, %r8b
1405040e0:     	callq	0x1406816c0 <.text+0x6806c0>
1405040e5:     	movq	%rbx, %rcx
1405040e8:     	callq	0x140681d50 <.text+0x680d50>
1405040ed:     	movq	%rax, %rcx
1405040f0:     	callq	0x140683c80 <.text+0x682c80>
1405040f5:     	testb	%al, %al
1405040f7:     	je	0x1405040fc <.text+0x5030fc>
1405040f9:     	movb	$0x1, (%rsi)
1405040fc:     	addq	$0xe8, %rsp
140504103:     	popq	%rbx
140504104:     	popq	%rbp
140504105:     	popq	%rdi
140504106:     	popq	%rsi
140504107:     	popq	%r12
140504109:     	popq	%r13
14050410b:     	popq	%r14
14050410d:     	popq	%r15
14050410f:     	retq
