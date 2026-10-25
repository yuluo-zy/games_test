
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14031a8a0:     	pushq	%rbp
14031a8a1:     	pushq	%rsi
14031a8a2:     	pushq	%rdi
14031a8a3:     	subq	$0x200, %rsp            # imm = 0x200
14031a8aa:     	leaq	0x80(%rsp), %rbp
14031a8b2:     	movq	$-0x2, 0x178(%rbp)
14031a8bd:     	movq	%rcx, %rsi
14031a8c0:     	xorps	%xmm0, %xmm0
14031a8c3:     	movups	%xmm0, -0x40(%rbp)
14031a8c7:     	movups	%xmm0, -0x28(%rbp)
14031a8cb:     	movups	%xmm0, -0x10(%rbp)
14031a8cf:     	movups	%xmm0, 0x8(%rbp)
14031a8d3:     	movups	%xmm0, 0x20(%rbp)
14031a8d7:     	movl	$0x0, 0x30(%rbp)
14031a8de:     	movups	%xmm0, 0x58(%rbp)
14031a8e2:     	movups	%xmm0, 0x70(%rbp)
14031a8e6:     	movups	%xmm0, 0x88(%rbp)
14031a8ed:     	movups	%xmm0, 0xa0(%rbp)
14031a8f4:     	movups	%xmm0, 0xb8(%rbp)
14031a8fb:     	movl	$0x0, 0xc8(%rbp)
14031a905:     	movq	$0x0, -0x60(%rbp)
14031a90d:     	movq	$0x8, -0x58(%rbp)
14031a915:     	movq	$0x0, -0x50(%rbp)
14031a91d:     	movq	$0x10, -0x48(%rbp)
14031a925:     	movq	$0x10, -0x30(%rbp)
14031a92d:     	movq	$0x10, -0x18(%rbp)
14031a935:     	movq	$0x10, (%rbp)
14031a93d:     	movq	$0x10, 0x18(%rbp)
14031a945:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14031a94f:     	movq	%rax, 0x38(%rbp)
14031a953:     	leaq	0x26a1a23(%rip), %rcx   # 0x1429bc37d
14031a95a:     	movq	%rcx, 0x40(%rbp)
14031a95e:     	movq	$0x3b, 0x48(%rbp)
14031a966:     	movq	$0x10, 0x50(%rbp)
14031a96e:     	movq	$0x10, 0x68(%rbp)
14031a976:     	movq	$0x10, 0x80(%rbp)
14031a981:     	movq	$0x10, 0x98(%rbp)
14031a98c:     	movq	$0x10, 0xb0(%rbp)
14031a997:     	movl	$0x0, 0xd0(%rbp)
14031a9a1:     	movw	$0x1, 0xd4(%rbp)
14031a9aa:     	movq	%rax, 0x138(%rbp)
14031a9b1:     	movl	$0x0, 0x170(%rbp)
14031a9bb:     	movzbl	0x331a247(%rip), %eax   # 0x143634c09
14031a9c2:     	movl	$0x1d8, %ecx            # imm = 0x1D8
14031a9c7:     	movl	$0x8, %edx
14031a9cc:     	callq	0x140613c10 <.text+0x612c10>
14031a9d1:     	testq	%rax, %rax
14031a9d4:     	je	0x14031aa0b <.text+0x319a0b>
14031a9d6:     	movq	%rax, %rdi
14031a9d9:     	leaq	-0x60(%rbp), %rdx
14031a9dd:     	movl	$0x1d8, %r8d            # imm = 0x1D8
14031a9e3:     	movq	%rax, %rcx
14031a9e6:     	callq	0x142923c00 <.text+0x2922c00>
14031a9eb:     	leaq	0x26a18b6(%rip), %r8    # 0x1429bc2a8
14031a9f2:     	movq	%rsi, %rcx
14031a9f5:     	movq	%rdi, %rdx
14031a9f8:     	callq	0x140ce4b10 <.text+0xce3b10>
14031a9fd:     	movq	%rsi, %rax
14031aa00:     	addq	$0x200, %rsp            # imm = 0x200
14031aa07:     	popq	%rdi
14031aa08:     	popq	%rsi
14031aa09:     	popq	%rbp
14031aa0a:     	retq
14031aa0b:     	movl	$0x8, %ecx
14031aa10:     	movl	$0x1d8, %edx            # imm = 0x1D8
14031aa15:     	callq	0x1428d9000 <.text+0x28d8000>
14031aa1a:     	ud2
