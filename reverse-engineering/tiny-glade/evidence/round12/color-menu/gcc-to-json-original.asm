
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14085ee90:     	pushq	%rbp
14085ee91:     	pushq	%rsi
14085ee92:     	pushq	%rdi
14085ee93:     	pushq	%rbx
14085ee94:     	subq	$0x58, %rsp
14085ee98:     	leaq	0x50(%rsp), %rbp
14085ee9d:     	movq	$-0x2, (%rbp)
14085eea5:     	movq	%rdx, %rdi
14085eea8:     	movq	%rcx, %rsi
14085eeab:     	movzbl	0x2dd5d57(%rip), %eax   # 0x143634c09
14085eeb2:     	movl	$0x80, %ecx
14085eeb7:     	movl	$0x1, %edx
14085eebc:     	callq	0x140613c10 <.text+0x612c10>
14085eec1:     	testq	%rax, %rax
14085eec4:     	je	0x14085f156 <.text+0x85e156>
14085eeca:     	movq	$0x80, -0x28(%rbp)
14085eed2:     	movq	%rax, -0x20(%rbp)
14085eed6:     	leaq	-0x28(%rbp), %rcx
14085eeda:     	movq	%rcx, -0x30(%rbp)
14085eede:     	movb	$0x7b, (%rax)
14085eee1:     	movq	$0x1, -0x18(%rbp)
14085eee9:     	leaq	-0x30(%rbp), %rax
14085eeed:     	movq	%rax, -0x10(%rbp)
14085eef1:     	movb	$0x1, -0x8(%rbp)
14085eef5:     	leaq	0x18(%rdi), %r9
14085eef9:     	leaq	0x223680f(%rip), %rdx   # 0x142a9570f
14085ef00:     	leaq	-0x10(%rbp), %rcx
14085ef04:     	movl	$0xd, %r8d
14085ef0a:     	callq	0x141583a40 <.text+0x1582a40>
14085ef0f:     	movq	%rax, %rbx
14085ef12:     	testq	%rax, %rax
14085ef15:     	jne	0x14085f121 <.text+0x85e121>
14085ef1b:     	leaq	0x1c(%rdi), %r9
14085ef1f:     	leaq	0x22367f6(%rip), %rdx   # 0x142a9571c
14085ef26:     	leaq	-0x10(%rbp), %rcx
14085ef2a:     	movl	$0x8, %r8d
14085ef30:     	callq	0x141583a40 <.text+0x1582a40>
14085ef35:     	movq	%rax, %rbx
14085ef38:     	testq	%rax, %rax
14085ef3b:     	jne	0x14085f121 <.text+0x85e121>
14085ef41:     	leaq	0x20(%rdi), %r9
14085ef45:     	leaq	0x22367d8(%rip), %rdx   # 0x142a95724
14085ef4c:     	leaq	-0x10(%rbp), %rcx
14085ef50:     	movl	$0x4, %r8d
14085ef56:     	callq	0x141583a40 <.text+0x1582a40>
14085ef5b:     	movq	%rax, %rbx
14085ef5e:     	testq	%rax, %rax
14085ef61:     	jne	0x14085f121 <.text+0x85e121>
14085ef67:     	leaq	0x42(%rdi), %r9
14085ef6b:     	leaq	0x22367b6(%rip), %rdx   # 0x142a95728
14085ef72:     	leaq	-0x10(%rbp), %rcx
14085ef76:     	movl	$0x13, %r8d
14085ef7c:     	callq	0x141583920 <.text+0x1582920>
14085ef81:     	movq	%rax, %rbx
14085ef84:     	testq	%rax, %rax
14085ef87:     	jne	0x14085f121 <.text+0x85e121>
14085ef8d:     	leaq	0x40(%rdi), %r9
14085ef91:     	leaq	0x22367a3(%rip), %rdx   # 0x142a9573b
14085ef98:     	leaq	-0x10(%rbp), %rcx
14085ef9c:     	movl	$0xb, %r8d
14085efa2:     	callq	0x141584b90 <.text+0x1583b90>
14085efa7:     	movq	%rax, %rbx
14085efaa:     	testq	%rax, %rax
14085efad:     	jne	0x14085f121 <.text+0x85e121>
14085efb3:     	leaq	0x24(%rdi), %r9
14085efb7:     	leaq	0x2236788(%rip), %rdx   # 0x142a95746
14085efbe:     	leaq	-0x10(%rbp), %rcx
14085efc2:     	movl	$0xa, %r8d
14085efc8:     	callq	0x141583a40 <.text+0x1582a40>
14085efcd:     	movq	%rax, %rbx
14085efd0:     	testq	%rax, %rax
14085efd3:     	jne	0x14085f121 <.text+0x85e121>
14085efd9:     	leaq	0x28(%rdi), %r9
14085efdd:     	leaq	0x223676c(%rip), %rdx   # 0x142a95750
14085efe4:     	leaq	-0x10(%rbp), %rcx
14085efe8:     	movl	$0xd, %r8d
14085efee:     	callq	0x141583a40 <.text+0x1582a40>
14085eff3:     	movq	%rax, %rbx
14085eff6:     	testq	%rax, %rax
14085eff9:     	jne	0x14085f121 <.text+0x85e121>
14085efff:     	leaq	0x2c(%rdi), %r9
14085f003:     	leaq	0x2236753(%rip), %rdx   # 0x142a9575d
14085f00a:     	leaq	-0x10(%rbp), %rcx
14085f00e:     	movl	$0x6, %r8d
14085f014:     	callq	0x141583a40 <.text+0x1582a40>
14085f019:     	movq	%rax, %rbx
14085f01c:     	testq	%rax, %rax
14085f01f:     	jne	0x14085f121 <.text+0x85e121>
14085f025:     	leaq	0x30(%rdi), %r9
14085f029:     	leaq	0x2236733(%rip), %rdx   # 0x142a95763
14085f030:     	leaq	-0x10(%rbp), %rcx
14085f034:     	movl	$0xc, %r8d
14085f03a:     	callq	0x141583a40 <.text+0x1582a40>
14085f03f:     	movq	%rax, %rbx
14085f042:     	testq	%rax, %rax
14085f045:     	jne	0x14085f121 <.text+0x85e121>
14085f04b:     	leaq	0x34(%rdi), %r9
14085f04f:     	leaq	0x2236719(%rip), %rdx   # 0x142a9576f
14085f056:     	leaq	-0x10(%rbp), %rcx
14085f05a:     	movl	$0x4, %r8d
14085f060:     	callq	0x141583a40 <.text+0x1582a40>
14085f065:     	movq	%rax, %rbx
14085f068:     	testq	%rax, %rax
14085f06b:     	jne	0x14085f121 <.text+0x85e121>
14085f071:     	leaq	0x22366fb(%rip), %rdx   # 0x142a95773
14085f078:     	leaq	-0x10(%rbp), %rcx
14085f07c:     	movl	$0x7, %r8d
14085f082:     	movq	%rdi, %r9
14085f085:     	callq	0x14158b780 <.text+0x158a780>
14085f08a:     	movq	%rax, %rbx
14085f08d:     	testq	%rax, %rax
14085f090:     	jne	0x14085f121 <.text+0x85e121>
14085f096:     	leaq	0x38(%rdi), %r9
14085f09a:     	leaq	0x22366d9(%rip), %rdx   # 0x142a9577a
14085f0a1:     	leaq	-0x10(%rbp), %rcx
14085f0a5:     	movl	$0x5, %r8d
14085f0ab:     	callq	0x141583a40 <.text+0x1582a40>
14085f0b0:     	movq	%rax, %rbx
14085f0b3:     	testq	%rax, %rax
14085f0b6:     	jne	0x14085f121 <.text+0x85e121>
14085f0b8:     	leaq	0x41(%rdi), %r9
14085f0bc:     	leaq	0x22366bc(%rip), %rdx   # 0x142a9577f
14085f0c3:     	leaq	-0x10(%rbp), %rcx
14085f0c7:     	movl	$0xe, %r8d
14085f0cd:     	callq	0x141584610 <.text+0x1583610>
14085f0d2:     	movq	%rax, %rbx
14085f0d5:     	testq	%rax, %rax
14085f0d8:     	jne	0x14085f121 <.text+0x85e121>
14085f0da:     	leaq	0x3c(%rdi), %r9
14085f0de:     	leaq	0x22366a8(%rip), %rdx   # 0x142a9578d
14085f0e5:     	leaq	-0x10(%rbp), %rcx
14085f0e9:     	movl	$0x12, %r8d
14085f0ef:     	callq	0x141586090 <.text+0x1585090>
14085f0f4:     	movq	%rax, %rbx
14085f0f7:     	testq	%rax, %rax
14085f0fa:     	jne	0x14085f121 <.text+0x85e121>
14085f0fc:     	addq	$0x8, %rdi
14085f100:     	leaq	0x2236698(%rip), %rdx   # 0x142a9579f
14085f107:     	leaq	-0x10(%rbp), %rcx
14085f10b:     	movl	$0x6, %r8d
14085f111:     	movq	%rdi, %r9
14085f114:     	callq	0x14158c2c0 <.text+0x158b2c0>
14085f119:     	movq	%rax, %rbx
14085f11c:     	testq	%rax, %rax
14085f11f:     	je	0x14085f16c <.text+0x85e16c>
14085f121:     	movq	-0x28(%rbp), %rdx
14085f125:     	testq	%rdx, %rdx
14085f128:     	je	0x14085f139 <.text+0x85e139>
14085f12a:     	movq	-0x20(%rbp), %rcx
14085f12e:     	movl	$0x1, %r8d
14085f134:     	callq	0x140613c20 <.text+0x612c20>
14085f139:     	movq	%rbx, 0x8(%rsi)
14085f13d:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14085f147:     	movq	%rax, (%rsi)
14085f14a:     	movq	%rsi, %rax
14085f14d:     	addq	$0x58, %rsp
14085f151:     	popq	%rbx
14085f152:     	popq	%rdi
14085f153:     	popq	%rsi
14085f154:     	popq	%rbp
14085f155:     	retq
14085f156:     	leaq	0x2236383(%rip), %r8    # 0x142a954e0
14085f15d:     	movl	$0x1, %ecx
14085f162:     	movl	$0x80, %edx
14085f167:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14085f16c:     	cmpb	$0x0, -0x8(%rbp)
14085f170:     	je	0x14085f192 <.text+0x85e192>
14085f172:     	movq	-0x10(%rbp), %rax
14085f176:     	movq	(%rax), %rcx
14085f179:     	leaq	0x2236378(%rip), %rdx   # 0x142a954f8
14085f180:     	leaq	0x2236429(%rip), %r9    # 0x142a955b0
14085f187:     	movl	$0x1, %r8d
14085f18d:     	callq	0x1412f4520 <.text+0x12f3520>
14085f192:     	movq	-0x28(%rbp), %rax
14085f196:     	movq	-0x20(%rbp), %rbx
14085f19a:     	movq	%rax, %rcx
14085f19d:     	negq	%rcx
14085f1a0:     	jo	0x14085f139 <.text+0x85e139>
14085f1a2:     	movq	-0x18(%rbp), %rcx
14085f1a6:     	movq	%rax, (%rsi)
14085f1a9:     	movq	%rbx, 0x8(%rsi)
14085f1ad:     	movq	%rcx, 0x10(%rsi)
14085f1b1:     	jmp	0x14085f14a <.text+0x85e14a>
14085f1b3:     	nopw	%cs:(%rax,%rax)
14085f1c0:     	movq	%rdx, 0x10(%rsp)
14085f1c5:     	pushq	%rbp
14085f1c6:     	pushq	%rsi
14085f1c7:     	pushq	%rdi
14085f1c8:     	pushq	%rbx
14085f1c9:     	subq	$0x28, %rsp
14085f1cd:     	leaq	0x50(%rdx), %rbp
14085f1d1:     	movq	-0x28(%rbp), %rdx
14085f1d5:     	testq	%rdx, %rdx
14085f1d8:     	je	0x14085f1e9 <.text+0x85e1e9>
14085f1da:     	movq	-0x20(%rbp), %rcx
14085f1de:     	movl	$0x1, %r8d
14085f1e4:     	callq	0x140613c20 <.text+0x612c20>
14085f1e9:     	nop
14085f1ea:     	addq	$0x28, %rsp
14085f1ee:     	popq	%rbx
14085f1ef:     	popq	%rdi
14085f1f0:     	popq	%rsi
14085f1f1:     	popq	%rbp
14085f1f2:     	retq
14085f1f3:     	int3
14085f1f4:     	int3
14085f1f5:     	int3
14085f1f6:     	int3
14085f1f7:     	int3
14085f1f8:     	int3
14085f1f9:     	int3
14085f1fa:     	int3
14085f1fb:     	int3
14085f1fc:     	int3
14085f1fd:     	int3
14085f1fe:     	int3
14085f1ff:     	int3
