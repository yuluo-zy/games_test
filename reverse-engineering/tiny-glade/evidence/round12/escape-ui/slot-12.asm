
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140503950:     	pushq	%r15
140503952:     	pushq	%r14
140503954:     	pushq	%r13
140503956:     	pushq	%r12
140503958:     	pushq	%rsi
140503959:     	pushq	%rdi
14050395a:     	pushq	%rbp
14050395b:     	pushq	%rbx
14050395c:     	subq	$0xf8, %rsp
140503963:     	movq	%r9, %rsi
140503966:     	movq	%rcx, %rbx
140503969:     	movq	%rdx, 0x68(%rsp)
14050396e:     	movq	%r8, 0x70(%rsp)
140503973:     	leaq	0x10(%rcx), %rdi
140503977:     	movq	%rdi, %rcx
14050397a:     	callq	0x1406868b0 <.text+0x6858b0>
14050397f:     	movq	%rax, %r12
140503982:     	leaq	0x68(%rsp), %r15
140503987:     	movq	%r15, %rcx
14050398a:     	callq	0x140683c10 <.text+0x682c10>
14050398f:     	movq	%rax, 0x48(%rsp)
140503994:     	leaq	0x48(%rsp), %r14
140503999:     	movq	%r14, %rcx
14050399c:     	callq	0x1400885d0 <.text+0x875d0>
1405039a1:     	movq	%rax, 0x78(%rsp)
1405039a6:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
1405039b0:     	movq	%r13, 0x80(%rsp)
1405039b8:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
1405039c2:     	movq	%rbp, 0x88(%rsp)
1405039ca:     	leaq	0x78(%rsp), %rcx
1405039cf:     	callq	0x1405bdf20 <.text+0x5bcf20>
1405039d4:     	movq	%r12, 0x78(%rsp)
1405039d9:     	movq	%rax, 0x80(%rsp)
1405039e1:     	leaq	0x78(%rsp), %rcx
1405039e6:     	callq	0x1405bd910 <.text+0x5bc910>
1405039eb:     	movq	%rax, %r12
1405039ee:     	movq	%rdi, %rcx
1405039f1:     	movq	%rax, %rdx
1405039f4:     	callq	0x140686450 <.text+0x685450>
1405039f9:     	testb	%al, %al
1405039fb:     	je	0x140503a15 <.text+0x502a15>
1405039fd:     	leaq	0x48(%rsp), %rbx
140503a02:     	movq	%rbx, %rcx
140503a05:     	movq	%rdi, %rdx
140503a08:     	movq	%r12, %r8
140503a0b:     	callq	0x140686530 <.text+0x685530>
140503a10:     	jmp	0x140503b6a <.text+0x502b6a>
140503a15:     	movq	0x312898c(%rip), %rax   # 0x14362c3a8
140503a1c:     	movq	(%rax), %rax
140503a1f:     	testq	%rax, %rax
140503a22:     	je	0x140503b09 <.text+0x502b09>
140503a28:     	movq	%rbx, 0x48(%rsp)
140503a2d:     	leaq	0x2fe6c(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140503a34:     	movq	%rax, 0x50(%rsp)
140503a39:     	movq	%r15, 0x58(%rsp)
140503a3e:     	movq	%rax, 0x60(%rsp)
140503a43:     	leaq	0x24ea30e(%rip), %rcx   # 0x1429edd58
140503a4a:     	callq	0x14067dd80 <.text+0x67cd80>
140503a4f:     	movups	(%rax), %xmm0
140503a52:     	movl	0x10(%rax), %eax
140503a55:     	movq	$0x1, 0xa8(%rsp)
140503a61:     	leaq	0x24ea2c0(%rip), %rcx   # 0x1429edd28
140503a68:     	movq	%rcx, 0xb0(%rsp)
140503a70:     	movq	$0x14, 0xb8(%rsp)
140503a7c:     	leaq	0x24ea215(%rip), %rdx   # 0x1429edc98
140503a83:     	movq	%rdx, 0xc8(%rsp)
140503a8b:     	movq	$0x2, 0xd0(%rsp)
140503a97:     	movq	%r14, 0xd8(%rsp)
140503a9f:     	movq	$0x2, 0xe0(%rsp)
140503aab:     	movq	$0x0, 0xe8(%rsp)
140503ab7:     	movq	$0x0, 0x78(%rsp)
140503ac0:     	movq	%rcx, 0x80(%rsp)
140503ac8:     	movq	$0x14, 0x88(%rsp)
140503ad4:     	movq	$0x0, 0x90(%rsp)
140503ae0:     	movups	%xmm0, 0x98(%rsp)
140503ae8:     	movl	$0x1, 0xc0(%rsp)
140503af3:     	movl	%eax, 0xc4(%rsp)
140503afa:     	leaq	0x47(%rsp), %rcx
140503aff:     	leaq	0x78(%rsp), %rdx
140503b04:     	callq	0x140d4a330 <.text+0xd49330>
140503b09:     	movups	0x68(%rsp), %xmm0
140503b0e:     	movq	0x18(%rbx), %rbx
140503b12:     	movups	%xmm0, 0x48(%rsp)
140503b17:     	leaq	0x48(%rsp), %rcx
140503b1c:     	callq	0x140683c10 <.text+0x682c10>
140503b21:     	movq	%rax, 0x78(%rsp)
140503b26:     	movq	%r13, 0x80(%rsp)
140503b2e:     	movq	%rbp, 0x88(%rsp)
140503b36:     	leaq	0x78(%rsp), %rcx
140503b3b:     	callq	0x1405bdf20 <.text+0x5bcf20>
140503b40:     	movq	%rbx, 0x78(%rsp)
140503b45:     	movq	%rax, 0x80(%rsp)
140503b4d:     	leaq	0x78(%rsp), %rcx
140503b52:     	callq	0x1405bd910 <.text+0x5bc910>
140503b57:     	leaq	0x48(%rsp), %rbx
140503b5c:     	movq	%rbx, %rcx
140503b5f:     	movq	%rdi, %rdx
140503b62:     	movq	%rax, %r8
140503b65:     	callq	0x1406861b0 <.text+0x6851b0>
140503b6a:     	movq	%rbx, %rcx
140503b6d:     	callq	0x140683ce0 <.text+0x682ce0>
140503b72:     	movl	%eax, %ecx
140503b74:     	xorb	$0x1, %cl
140503b77:     	movzbl	%cl, %ecx
140503b7a:     	leaq	0x11(,%rcx,2), %rcx
140503b82:     	leaq	0x24ea2cc(%rip), %rdx   # 0x1429ede55
140503b89:     	leaq	0x24ea2b2(%rip), %r8    # 0x1429ede42
140503b90:     	testb	%al, %al
140503b92:     	cmovneq	%rdx, %r8
140503b96:     	movq	%r8, 0x20(%rsp)
140503b9b:     	movq	%rcx, 0x28(%rsp)
140503ba0:     	movl	$0x42480000, 0x30(%rsp) # imm = 0x42480000
140503ba8:     	leaq	0x24ea2b7(%rip), %r8    # 0x1429ede66
140503baf:     	leaq	0x78(%rsp), %rdi
140503bb4:     	movl	$0xb, %r9d
140503bba:     	movq	%rdi, %rcx
140503bbd:     	movq	%rbx, %rdx
140503bc0:     	callq	0x14032e0b0 <.text+0x32d0b0>
140503bc5:     	movq	%rdi, %rcx
140503bc8:     	callq	0x140681d50 <.text+0x680d50>
140503bcd:     	movq	%rax, %rcx
140503bd0:     	callq	0x140683c80 <.text+0x682c80>
140503bd5:     	testb	%al, %al
140503bd7:     	je	0x140503be1 <.text+0x502be1>
140503bd9:     	movq	%rsi, %rcx
140503bdc:     	callq	0x1403d09d0 <.text+0x3cf9d0>
140503be1:     	nop
140503be2:     	addq	$0xf8, %rsp
140503be9:     	popq	%rbx
140503bea:     	popq	%rbp
140503beb:     	popq	%rdi
140503bec:     	popq	%rsi
140503bed:     	popq	%r12
140503bef:     	popq	%r13
140503bf1:     	popq	%r14
140503bf3:     	popq	%r15
140503bf5:     	retq
