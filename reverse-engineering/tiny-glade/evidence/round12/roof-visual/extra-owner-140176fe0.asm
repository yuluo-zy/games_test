
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140176fe0:     	pushq	%rbp
140176fe1:     	pushq	%rsi
140176fe2:     	pushq	%rdi
140176fe3:     	subq	$0x2e0, %rsp            # imm = 0x2E0
140176fea:     	leaq	0x80(%rsp), %rbp
140176ff2:     	movq	$-0x2, 0x258(%rbp)
140176ffd:     	movq	%rcx, %rsi
140177000:     	xorps	%xmm0, %xmm0
140177003:     	movups	%xmm0, -0x40(%rbp)
140177007:     	movups	%xmm0, -0x28(%rbp)
14017700b:     	movups	%xmm0, -0x10(%rbp)
14017700f:     	movups	%xmm0, 0x8(%rbp)
140177013:     	movups	%xmm0, 0x20(%rbp)
140177017:     	movl	$0x0, 0x30(%rbp)
14017701e:     	movups	%xmm0, 0x58(%rbp)
140177022:     	movups	%xmm0, 0x70(%rbp)
140177026:     	movups	%xmm0, 0x88(%rbp)
14017702d:     	movups	%xmm0, 0xa0(%rbp)
140177034:     	movups	%xmm0, 0xb8(%rbp)
14017703b:     	movl	$0x0, 0xc8(%rbp)
140177045:     	movq	$0x0, -0x60(%rbp)
14017704d:     	movq	$0x8, -0x58(%rbp)
140177055:     	movq	$0x0, -0x50(%rbp)
14017705d:     	movq	$0x10, -0x48(%rbp)
140177065:     	movq	$0x10, -0x30(%rbp)
14017706d:     	movq	$0x10, -0x18(%rbp)
140177075:     	movq	$0x10, (%rbp)
14017707d:     	movq	$0x10, 0x18(%rbp)
140177085:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14017708f:     	movq	%rax, 0x38(%rbp)
140177093:     	leaq	0x27e9558(%rip), %rcx   # 0x1429605f2
14017709a:     	movq	%rcx, 0x40(%rbp)
14017709e:     	movq	$0x81, 0x48(%rbp)
1401770a6:     	movq	$0x10, 0x50(%rbp)
1401770ae:     	movq	$0x10, 0x68(%rbp)
1401770b6:     	movq	$0x10, 0x80(%rbp)
1401770c1:     	movq	$0x10, 0x98(%rbp)
1401770cc:     	movq	$0x10, 0xb0(%rbp)
1401770d7:     	movl	$0x0, 0xd0(%rbp)
1401770e1:     	movw	$0x1, 0xd4(%rbp)
1401770ea:     	movq	%rax, 0x1d0(%rbp)
1401770f1:     	movl	$0x0, 0x250(%rbp)
1401770fb:     	movzbl	0x34bdb07(%rip), %eax   # 0x143634c09
140177102:     	movl	$0x2b8, %ecx            # imm = 0x2B8
140177107:     	movl	$0x8, %edx
14017710c:     	callq	0x140613c10 <.text+0x612c10>
140177111:     	testq	%rax, %rax
140177114:     	je	0x14017714b <.text+0x17614b>
140177116:     	movq	%rax, %rdi
140177119:     	leaq	-0x60(%rbp), %rdx
14017711d:     	movl	$0x2b8, %r8d            # imm = 0x2B8
140177123:     	movq	%rax, %rcx
140177126:     	callq	0x142923c00 <.text+0x2922c00>
14017712b:     	leaq	0x27e92f6(%rip), %r8    # 0x142960428
140177132:     	movq	%rsi, %rcx
140177135:     	movq	%rdi, %rdx
140177138:     	callq	0x140ce4b10 <.text+0xce3b10>
14017713d:     	movq	%rsi, %rax
140177140:     	addq	$0x2e0, %rsp            # imm = 0x2E0
140177147:     	popq	%rdi
140177148:     	popq	%rsi
140177149:     	popq	%rbp
14017714a:     	retq
14017714b:     	movl	$0x8, %ecx
140177150:     	movl	$0x2b8, %edx            # imm = 0x2B8
140177155:     	callq	0x1428d9000 <.text+0x28d8000>
14017715a:     	ud2
14017715c:     	nopl	(%rax)
140177160:     	movq	%rdx, 0x10(%rsp)
140177165:     	pushq	%rbp
140177166:     	pushq	%rsi
140177167:     	pushq	%rdi
140177168:     	subq	$0x20, %rsp
14017716c:     	leaq	0x80(%rdx), %rbp
140177173:     	leaq	-0x60(%rbp), %rcx
140177177:     	callq	0x1405cf3b0 <.text+0x5ce3b0>
14017717c:     	nop
14017717d:     	addq	$0x20, %rsp
140177181:     	popq	%rdi
140177182:     	popq	%rsi
140177183:     	popq	%rbp
140177184:     	retq
