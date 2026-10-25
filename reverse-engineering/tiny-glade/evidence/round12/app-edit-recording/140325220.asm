
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140325220:     	pushq	%rbp
140325221:     	pushq	%rsi
140325222:     	pushq	%rdi
140325223:     	subq	$0x1a0, %rsp            # imm = 0x1A0
14032522a:     	leaq	0x80(%rsp), %rbp
140325232:     	movq	$-0x2, 0x118(%rbp)
14032523d:     	movq	%rcx, %rsi
140325240:     	xorps	%xmm0, %xmm0
140325243:     	movups	%xmm0, -0x8(%rbp)
140325247:     	movups	%xmm0, 0x10(%rbp)
14032524b:     	movups	%xmm0, 0x28(%rbp)
14032524f:     	movups	%xmm0, 0x40(%rbp)
140325253:     	movups	%xmm0, 0x58(%rbp)
140325257:     	movl	$0x0, 0x68(%rbp)
14032525e:     	movups	%xmm0, 0x90(%rbp)
140325265:     	movups	%xmm0, 0xa8(%rbp)
14032526c:     	movups	%xmm0, 0xc0(%rbp)
140325273:     	movups	%xmm0, 0xd8(%rbp)
14032527a:     	movups	%xmm0, 0xf0(%rbp)
140325281:     	movl	$0x0, 0x100(%rbp)
14032528b:     	movq	$0x0, -0x58(%rbp)
140325293:     	movq	$0x0, -0x28(%rbp)
14032529b:     	movq	$0x8, -0x20(%rbp)
1403252a3:     	movq	$0x0, -0x18(%rbp)
1403252ab:     	movq	$0x10, -0x10(%rbp)
1403252b3:     	movq	$0x10, 0x8(%rbp)
1403252bb:     	movq	$0x10, 0x20(%rbp)
1403252c3:     	movq	$0x10, 0x38(%rbp)
1403252cb:     	movq	$0x10, 0x50(%rbp)
1403252d3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403252dd:     	movq	%rax, 0x70(%rbp)
1403252e1:     	leaq	0x2698cc7(%rip), %rax   # 0x1429bdfaf
1403252e8:     	movq	%rax, 0x78(%rbp)
1403252ec:     	movq	$0x49, 0x80(%rbp)
1403252f7:     	movq	$0x10, 0x88(%rbp)
140325302:     	movq	$0x10, 0xa0(%rbp)
14032530d:     	movq	$0x10, 0xb8(%rbp)
140325318:     	movq	$0x10, 0xd0(%rbp)
140325323:     	movq	$0x10, 0xe8(%rbp)
14032532e:     	movl	$0x0, 0x108(%rbp)
140325338:     	movw	$0x1, 0x10c(%rbp)
140325341:     	movl	$0x0, 0x110(%rbp)
14032534b:     	movzbl	0x330f8b7(%rip), %eax   # 0x143634c09
140325352:     	movl	$0x170, %ecx            # imm = 0x170
140325357:     	movl	$0x8, %edx
14032535c:     	callq	0x140613c10 <.text+0x612c10>
140325361:     	testq	%rax, %rax
140325364:     	je	0x14032539b <.text+0x32439b>
140325366:     	movq	%rax, %rdi
140325369:     	leaq	-0x58(%rbp), %rdx
14032536d:     	movl	$0x170, %r8d            # imm = 0x170
140325373:     	movq	%rax, %rcx
140325376:     	callq	0x142923c00 <.text+0x2922c00>
14032537b:     	leaq	0x2698a16(%rip), %r8    # 0x1429bdd98
140325382:     	movq	%rsi, %rcx
140325385:     	movq	%rdi, %rdx
140325388:     	callq	0x140ce4b10 <.text+0xce3b10>
14032538d:     	movq	%rsi, %rax
140325390:     	addq	$0x1a0, %rsp            # imm = 0x1A0
140325397:     	popq	%rdi
140325398:     	popq	%rsi
140325399:     	popq	%rbp
14032539a:     	retq
14032539b:     	movl	$0x8, %ecx
1403253a0:     	movl	$0x170, %edx            # imm = 0x170
1403253a5:     	callq	0x1428d9000 <.text+0x28d8000>
1403253aa:     	ud2
