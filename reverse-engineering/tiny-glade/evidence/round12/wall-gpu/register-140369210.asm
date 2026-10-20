
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140369210:     	pushq	%rbp
140369211:     	pushq	%rsi
140369212:     	pushq	%rdi
140369213:     	subq	$0x190, %rsp            # imm = 0x190
14036921a:     	leaq	0x80(%rsp), %rbp
140369222:     	movq	$-0x2, 0x108(%rbp)
14036922d:     	movq	%rcx, %rsi
140369230:     	xorps	%xmm0, %xmm0
140369233:     	movups	%xmm0, -0x18(%rbp)
140369237:     	movups	%xmm0, (%rbp)
14036923b:     	movups	%xmm0, 0x18(%rbp)
14036923f:     	movups	%xmm0, 0x30(%rbp)
140369243:     	movups	%xmm0, 0x48(%rbp)
140369247:     	movl	$0x0, 0x58(%rbp)
14036924e:     	movups	%xmm0, 0x80(%rbp)
140369255:     	movups	%xmm0, 0x98(%rbp)
14036925c:     	movups	%xmm0, 0xb0(%rbp)
140369263:     	movups	%xmm0, 0xc8(%rbp)
14036926a:     	movups	%xmm0, 0xe0(%rbp)
140369271:     	movl	$0x0, 0xf0(%rbp)
14036927b:     	movq	$0x0, -0x58(%rbp)
140369283:     	movq	$0x0, -0x38(%rbp)
14036928b:     	movq	$0x8, -0x30(%rbp)
140369293:     	movq	$0x0, -0x28(%rbp)
14036929b:     	movq	$0x10, -0x20(%rbp)
1403692a3:     	movq	$0x10, -0x8(%rbp)
1403692ab:     	movq	$0x10, 0x10(%rbp)
1403692b3:     	movq	$0x10, 0x28(%rbp)
1403692bb:     	movq	$0x10, 0x40(%rbp)
1403692c3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403692cd:     	movq	%rax, 0x60(%rbp)
1403692d1:     	leaq	0x265d5ea(%rip), %rax   # 0x1429c68c2
1403692d8:     	movq	%rax, 0x68(%rbp)
1403692dc:     	movq	$0x30, 0x70(%rbp)
1403692e4:     	movq	$0x10, 0x78(%rbp)
1403692ec:     	movq	$0x10, 0x90(%rbp)
1403692f7:     	movq	$0x10, 0xa8(%rbp)
140369302:     	movq	$0x10, 0xc0(%rbp)
14036930d:     	movq	$0x10, 0xd8(%rbp)
140369318:     	movl	$0x0, 0xf8(%rbp)
140369322:     	movw	$0x1, 0xfc(%rbp)
14036932b:     	movl	$0x0, 0x100(%rbp)
140369335:     	movzbl	0x32cb8cd(%rip), %eax   # 0x143634c09
14036933c:     	movl	$0x160, %ecx            # imm = 0x160
140369341:     	movl	$0x8, %edx
140369346:     	callq	0x140613c10 <.text+0x612c10>
14036934b:     	testq	%rax, %rax
14036934e:     	je	0x140369385 <.text+0x368385>
140369350:     	movq	%rax, %rdi
140369353:     	leaq	-0x58(%rbp), %rdx
140369357:     	movl	$0x160, %r8d            # imm = 0x160
14036935d:     	movq	%rax, %rcx
140369360:     	callq	0x142923c00 <.text+0x2922c00>
140369365:     	leaq	0x265d424(%rip), %r8    # 0x1429c6790
14036936c:     	movq	%rsi, %rcx
14036936f:     	movq	%rdi, %rdx
140369372:     	callq	0x140ce4b10 <.text+0xce3b10>
140369377:     	movq	%rsi, %rax
14036937a:     	addq	$0x190, %rsp            # imm = 0x190
140369381:     	popq	%rdi
140369382:     	popq	%rsi
140369383:     	popq	%rbp
140369384:     	retq
140369385:     	movl	$0x8, %ecx
14036938a:     	movl	$0x160, %edx            # imm = 0x160
14036938f:     	callq	0x1428d9000 <.text+0x28d8000>
140369394:     	ud2
