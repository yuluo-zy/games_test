
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140369050:     	pushq	%rbp
140369051:     	pushq	%rsi
140369052:     	pushq	%rdi
140369053:     	subq	$0x190, %rsp            # imm = 0x190
14036905a:     	leaq	0x80(%rsp), %rbp
140369062:     	movq	$-0x2, 0x108(%rbp)
14036906d:     	movq	%rcx, %rsi
140369070:     	xorps	%xmm0, %xmm0
140369073:     	movups	%xmm0, -0x18(%rbp)
140369077:     	movups	%xmm0, (%rbp)
14036907b:     	movups	%xmm0, 0x18(%rbp)
14036907f:     	movups	%xmm0, 0x30(%rbp)
140369083:     	movups	%xmm0, 0x48(%rbp)
140369087:     	movl	$0x0, 0x58(%rbp)
14036908e:     	movups	%xmm0, 0x80(%rbp)
140369095:     	movups	%xmm0, 0x98(%rbp)
14036909c:     	movups	%xmm0, 0xb0(%rbp)
1403690a3:     	movups	%xmm0, 0xc8(%rbp)
1403690aa:     	movups	%xmm0, 0xe0(%rbp)
1403690b1:     	movl	$0x0, 0xf0(%rbp)
1403690bb:     	movq	$0x0, -0x58(%rbp)
1403690c3:     	movq	$0x0, -0x38(%rbp)
1403690cb:     	movq	$0x8, -0x30(%rbp)
1403690d3:     	movq	$0x0, -0x28(%rbp)
1403690db:     	movq	$0x10, -0x20(%rbp)
1403690e3:     	movq	$0x10, -0x8(%rbp)
1403690eb:     	movq	$0x10, 0x10(%rbp)
1403690f3:     	movq	$0x10, 0x28(%rbp)
1403690fb:     	movq	$0x10, 0x40(%rbp)
140369103:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14036910d:     	movq	%rax, 0x60(%rbp)
140369111:     	leaq	0x265d7da(%rip), %rax   # 0x1429c68f2
140369118:     	movq	%rax, 0x68(%rbp)
14036911c:     	movq	$0x32, 0x70(%rbp)
140369124:     	movq	$0x10, 0x78(%rbp)
14036912c:     	movq	$0x10, 0x90(%rbp)
140369137:     	movq	$0x10, 0xa8(%rbp)
140369142:     	movq	$0x10, 0xc0(%rbp)
14036914d:     	movq	$0x10, 0xd8(%rbp)
140369158:     	movl	$0x0, 0xf8(%rbp)
140369162:     	movw	$0x1, 0xfc(%rbp)
14036916b:     	movl	$0x0, 0x100(%rbp)
140369175:     	movzbl	0x32cba8d(%rip), %eax   # 0x143634c09
14036917c:     	movl	$0x160, %ecx            # imm = 0x160
140369181:     	movl	$0x8, %edx
140369186:     	callq	0x140613c10 <.text+0x612c10>
14036918b:     	testq	%rax, %rax
14036918e:     	je	0x1403691c5 <.text+0x3681c5>
140369190:     	movq	%rax, %rdi
140369193:     	leaq	-0x58(%rbp), %rdx
140369197:     	movl	$0x160, %r8d            # imm = 0x160
14036919d:     	movq	%rax, %rcx
1403691a0:     	callq	0x142923c00 <.text+0x2922c00>
1403691a5:     	leaq	0x265d52c(%rip), %r8    # 0x1429c66d8
1403691ac:     	movq	%rsi, %rcx
1403691af:     	movq	%rdi, %rdx
1403691b2:     	callq	0x140ce4b10 <.text+0xce3b10>
1403691b7:     	movq	%rsi, %rax
1403691ba:     	addq	$0x190, %rsp            # imm = 0x190
1403691c1:     	popq	%rdi
1403691c2:     	popq	%rsi
1403691c3:     	popq	%rbp
1403691c4:     	retq
1403691c5:     	movl	$0x8, %ecx
1403691ca:     	movl	$0x160, %edx            # imm = 0x160
1403691cf:     	callq	0x1428d9000 <.text+0x28d8000>
1403691d4:     	ud2
