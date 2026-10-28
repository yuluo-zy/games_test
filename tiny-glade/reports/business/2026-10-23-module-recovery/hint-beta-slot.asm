
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14228b8f0:     	pushq	%r15
14228b8f2:     	pushq	%r14
14228b8f4:     	pushq	%r13
14228b8f6:     	pushq	%r12
14228b8f8:     	pushq	%rsi
14228b8f9:     	pushq	%rdi
14228b8fa:     	pushq	%rbx
14228b8fb:     	subq	$0xf0, %rsp
14228b902:     	movq	%rcx, %rdi
14228b905:     	movq	%rdx, 0xc8(%rsp)
14228b90d:     	movq	%r8, 0xd0(%rsp)
14228b915:     	leaq	0x10(%rcx), %rsi
14228b919:     	movq	%rsi, %rcx
14228b91c:     	callq	0x1406868b0 <.text+0x6858b0>
14228b921:     	movq	%rax, %r15
14228b924:     	leaq	0xc8(%rsp), %r14
14228b92c:     	movq	%r14, %rcx
14228b92f:     	callq	0x140683c10 <.text+0x682c10>
14228b934:     	movq	%rax, 0x20(%rsp)
14228b939:     	leaq	0x20(%rsp), %rbx
14228b93e:     	movq	%rbx, %rcx
14228b941:     	callq	0x1400885d0 <.text+0x875d0>
14228b946:     	movq	%rax, 0x40(%rsp)
14228b94b:     	movabsq	$0x41223169ff28813b, %r12 # imm = 0x41223169FF28813B
14228b955:     	movq	%r12, 0x48(%rsp)
14228b95a:     	movabsq	$-0x58648d975d569727, %r13 # imm = 0xA79B7268A2A968D9
14228b964:     	movq	%r13, 0x50(%rsp)
14228b969:     	leaq	0x40(%rsp), %rcx
14228b96e:     	callq	0x14228d080 <.text+0x228c080>
14228b973:     	movq	%r15, 0x40(%rsp)
14228b978:     	movq	%rax, 0x48(%rsp)
14228b97d:     	leaq	0x40(%rsp), %rcx
14228b982:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b987:     	movq	%rax, %r15
14228b98a:     	movq	%rsi, %rcx
14228b98d:     	movq	%rax, %rdx
14228b990:     	callq	0x140686450 <.text+0x685450>
14228b995:     	testb	%al, %al
14228b997:     	je	0x14228b9b4 <.text+0x228a9b4>
14228b999:     	leaq	0xd8(%rsp), %rdi
14228b9a1:     	movq	%rdi, %rcx
14228b9a4:     	movq	%rsi, %rdx
14228b9a7:     	movq	%r15, %r8
14228b9aa:     	callq	0x140686530 <.text+0x685530>
14228b9af:     	jmp	0x14228baf7 <.text+0x228aaf7>
14228b9b4:     	movq	0x13a09ed(%rip), %rax   # 0x14362c3a8
14228b9bb:     	movq	(%rax), %rax
14228b9be:     	testq	%rax, %rax
14228b9c1:     	je	0x14228ba99 <.text+0x228aa99>
14228b9c7:     	movq	%rdi, 0x20(%rsp)
14228b9cc:     	leaq	-0x1d58133(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
14228b9d3:     	movq	%rax, 0x28(%rsp)
14228b9d8:     	movq	%r14, 0x30(%rsp)
14228b9dd:     	movq	%rax, 0x38(%rsp)
14228b9e2:     	leaq	0xdaec37(%rip), %rcx    # 0x14303a620
14228b9e9:     	callq	0x14067dd80 <.text+0x67cd80>
14228b9ee:     	movups	(%rax), %xmm0
14228b9f1:     	movl	0x10(%rax), %eax
14228b9f4:     	movq	$0x1, 0x70(%rsp)
14228b9fd:     	leaq	0xdaec34(%rip), %rcx    # 0x14303a638
14228ba04:     	movq	%rcx, 0x78(%rsp)
14228ba09:     	movq	$0x14, 0x80(%rsp)
14228ba15:     	leaq	0xdaeb8c(%rip), %rdx    # 0x14303a5a8
14228ba1c:     	movq	%rdx, 0x90(%rsp)
14228ba24:     	movq	$0x2, 0x98(%rsp)
14228ba30:     	movq	%rbx, 0xa0(%rsp)
14228ba38:     	movq	$0x2, 0xa8(%rsp)
14228ba44:     	movq	$0x0, 0xb0(%rsp)
14228ba50:     	movq	$0x0, 0x40(%rsp)
14228ba59:     	movq	%rcx, 0x48(%rsp)
14228ba5e:     	movq	$0x14, 0x50(%rsp)
14228ba67:     	movq	$0x0, 0x58(%rsp)
14228ba70:     	movups	%xmm0, 0x60(%rsp)
14228ba75:     	movl	$0x1, 0x88(%rsp)
14228ba80:     	movl	%eax, 0x8c(%rsp)
14228ba87:     	leaq	0xd8(%rsp), %rcx
14228ba8f:     	leaq	0x40(%rsp), %rdx
14228ba94:     	callq	0x140d4a330 <.text+0xd49330>
14228ba99:     	movups	0xc8(%rsp), %xmm0
14228baa1:     	movq	0x18(%rdi), %rdi
14228baa5:     	movups	%xmm0, 0x20(%rsp)
14228baaa:     	leaq	0x20(%rsp), %rcx
14228baaf:     	callq	0x140683c10 <.text+0x682c10>
14228bab4:     	movq	%rax, 0x40(%rsp)
14228bab9:     	movq	%r12, 0x48(%rsp)
14228babe:     	movq	%r13, 0x50(%rsp)
14228bac3:     	leaq	0x40(%rsp), %rcx
14228bac8:     	callq	0x14228d080 <.text+0x228c080>
14228bacd:     	movq	%rdi, 0x40(%rsp)
14228bad2:     	movq	%rax, 0x48(%rsp)
14228bad7:     	leaq	0x40(%rsp), %rcx
14228badc:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228bae1:     	leaq	0xd8(%rsp), %rdi
14228bae9:     	movq	%rdi, %rcx
14228baec:     	movq	%rsi, %rdx
14228baef:     	movq	%rax, %r8
14228baf2:     	callq	0x1406861b0 <.text+0x6851b0>
14228baf7:     	leaq	0xdaeb4e(%rip), %r8     # 0x14303a64c
14228bafe:     	leaq	0x40(%rsp), %rsi
14228bb03:     	movl	$0x18, %r9d
14228bb09:     	movq	%rsi, %rcx
14228bb0c:     	movq	%rdi, %rdx
14228bb0f:     	callq	0x14068bf00 <.text+0x68af00>
14228bb14:     	leaq	0x20(%rsp), %rdi
14228bb19:     	movq	%rdi, %rcx
14228bb1c:     	movq	%rsi, %rdx
14228bb1f:     	movw	$0x2bc, %r8w            # imm = 0x2BC
14228bb24:     	callq	0x14067e460 <.text+0x67d460>
14228bb29:     	movq	%rdi, %rcx
14228bb2c:     	callq	0x140681d30 <.text+0x680d30>
14228bb31:     	movss	0x6a8ac7(%rip), %xmm1   # 0x142934600
14228bb39:     	movq	%rax, %rcx
14228bb3c:     	callq	0x1425f0ac0 <.text+0x25efac0>
14228bb41:     	movq	0x30(%rsp), %rax
14228bb46:     	movq	%rax, 0x50(%rsp)
14228bb4b:     	movups	0x20(%rsp), %xmm0
14228bb50:     	movaps	%xmm0, 0x40(%rsp)
14228bb55:     	leaq	0x20(%rsp), %rsi
14228bb5a:     	leaq	0x40(%rsp), %rdx
14228bb5f:     	movq	%rsi, %rcx
14228bb62:     	movb	$0x2, %r8b
14228bb65:     	callq	0x14067e3e0 <.text+0x67d3e0>
14228bb6a:     	leaq	0x40(%rsp), %rcx
14228bb6f:     	movq	%rsi, %rdx
14228bb72:     	movb	$0x1, %r8b
14228bb75:     	callq	0x14067e420 <.text+0x67d420>
14228bb7a:     	nop
14228bb7b:     	addq	$0xf0, %rsp
14228bb82:     	popq	%rbx
14228bb83:     	popq	%rdi
14228bb84:     	popq	%rsi
14228bb85:     	popq	%r12
14228bb87:     	popq	%r13
14228bb89:     	popq	%r14
14228bb8b:     	popq	%r15
14228bb8d:     	retq
14228bb8e:     	int3
14228bb8f:     	int3
